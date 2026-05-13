use crate::model::{
    FeatureAssessment, FeatureDefinition, MatchMode, MissingRequirement, Report, Requirement,
    RequirementGroup, RequirementMatch, Snapshot, SupportStatus,
};
use crate::registry::phase1_features;
use std::collections::BTreeMap;

pub fn evaluate(snapshot: &Snapshot) -> Report {
    let mut features: Vec<_> = phase1_features()
        .iter()
        .map(|f| assess_feature(snapshot, f))
        .collect();
    features.sort_by(|a, b| a.feature_id.cmp(&b.feature_id));
    let mut summary = BTreeMap::new();
    summary.insert(
        "supportedFeatures".to_string(),
        count(&features, SupportStatus::Supported),
    );
    summary.insert(
        "partialFeatures".to_string(),
        count(&features, SupportStatus::Partial),
    );
    summary.insert(
        "unsupportedFeatures".to_string(),
        count(&features, SupportStatus::Unsupported),
    );
    summary.insert(
        "unknownFeatures".to_string(),
        count(&features, SupportStatus::Unknown),
    );
    summary.insert(
        "implementationNotFoundFeatures".to_string(),
        count(&features, SupportStatus::ImplementationNotFound),
    );
    Report {
        report_format_version: 1,
        assessment_basis: "staticDataModelSnapshot".to_string(),
        device: snapshot.device.clone(),
        detected_data_model: snapshot.detected_data_model.clone(),
        features,
        summary,
    }
}
fn count(features: &[FeatureAssessment], status: SupportStatus) -> usize {
    features.iter().filter(|f| f.support == status).count()
}

fn assess_feature(snapshot: &Snapshot, feature: &FeatureDefinition) -> FeatureAssessment {
    let mut matched = Vec::new();
    let mut missing_mandatory = Vec::new();
    let mut missing_optional = Vec::new();
    let mut missing_control = Vec::new();
    let mut missing_diagnostic = Vec::new();
    for requirement in &feature.requirements {
        let paths = match_requirement(snapshot, requirement);
        if paths.is_empty() {
            let missing = MissingRequirement {
                rule_id: requirement.rule_id.to_string(),
                concept_id: requirement.concept_id.to_string(),
                label: requirement.label.to_string(),
                candidate_paths: requirement
                    .candidates
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect(),
                impact: requirement.impact.to_string(),
                group: requirement.group,
            };
            match requirement.group {
                RequirementGroup::Mandatory => missing_mandatory.push(missing),
                RequirementGroup::Optional => missing_optional.push(missing),
                RequirementGroup::Control => missing_control.push(missing),
                RequirementGroup::Diagnostic => missing_diagnostic.push(missing),
            }
        } else {
            matched.push(RequirementMatch {
                rule_id: requirement.rule_id.to_string(),
                concept_id: requirement.concept_id.to_string(),
                label: requirement.label.to_string(),
                matched_paths: paths,
                group: requirement.group,
            });
        }
    }
    matched.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    missing_mandatory.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    missing_optional.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    missing_control.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    missing_diagnostic.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    let support = if !feature.implementation_found {
        SupportStatus::ImplementationNotFound
    } else if !missing_mandatory.is_empty() {
        SupportStatus::Unsupported
    } else if feature.control_required_for_support && !missing_control.is_empty() {
        SupportStatus::Unsupported
    } else if feature.diagnostic_required_for_support && !missing_diagnostic.is_empty() {
        SupportStatus::Unsupported
    } else if feature.runtime_validation_required
        || !missing_optional.is_empty()
        || !missing_control.is_empty()
        || !missing_diagnostic.is_empty()
    {
        SupportStatus::Partial
    } else {
        SupportStatus::Supported
    };
    FeatureAssessment {
        feature_id: feature.feature_id.to_string(),
        feature_name: feature.feature_name.to_string(),
        category: feature.category.to_string(),
        support,
        evidence_confidence: feature.confidence,
        runtime_validation_required: feature.runtime_validation_required,
        matched_requirements: matched,
        missing_mandatory_requirements: missing_mandatory,
        missing_optional_requirements: missing_optional,
        missing_control_requirements: missing_control,
        missing_diagnostic_requirements: missing_diagnostic,
        limitations: feature
            .limitations
            .iter()
            .map(|s| (*s).to_string())
            .collect(),
        source_refs: feature.source_refs.clone(),
    }
}

fn match_requirement(snapshot: &Snapshot, requirement: &Requirement) -> Vec<String> {
    let mut matches = Vec::new();
    for entry in &snapshot.entries {
        let Some(path) = entry.path.as_deref() else { continue; };
        if !requirement.candidates.iter().any(|c| path_matches(c, path)) {
            continue;
        }
        let ok = match requirement.match_mode {
            MatchMode::Present => true,
            MatchMode::Value => entry.has_value(),
            MatchMode::Readable => entry.readable.unwrap_or(false),
            MatchMode::Writable => entry.writable.unwrap_or(false),
            MatchMode::Diagnostic => {
                matches!(entry.kind_normalized(), "diagnostic" | "command" | "action")
                    && entry.available.unwrap_or(true)
                    && entry.commandable.unwrap_or(true)
            }
            MatchMode::History => {
                matches!(
                    entry.kind_normalized(),
                    "history" | "score" | "derivedMetric"
                ) && entry.has_value()
            }
        };
        if ok {
            matches.push(path.to_string());
        }
    }
    matches.sort();
    matches.dedup();
    matches
}
fn path_matches(pattern: &str, path: &str) -> bool {
    if pattern.ends_with('.') {
        return path.starts_with(pattern);
    }
    if pattern.contains("{i}") {
        return pattern_segments_match(pattern, path);
    }
    path == pattern || path.starts_with(pattern)
}
fn pattern_segments_match(pattern: &str, path: &str) -> bool {
    let pp: Vec<_> = pattern.trim_end_matches('.').split('.').collect();
    let sp: Vec<_> = path.trim_end_matches('.').split('.').collect();
    pp.len() <= sp.len() && pp.iter().zip(sp.iter()).all(|(a, b)| *a == "{i}" || a == b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::load_snapshot;
    use std::path::Path;
    #[test]
    fn read_only_channel_makes_remote_channel_management_unsupported() {
        let snapshot = load_snapshot(Path::new(
            "tests/fixtures/offline-analyzer/read-only-control.json",
        ))
        .unwrap();
        let report = evaluate(&snapshot);
        let feature = report
            .features
            .iter()
            .find(|f| f.feature_id == "selfHealing.remoteChannelManagement")
            .unwrap();
        assert_eq!(feature.support, SupportStatus::Unsupported);
        assert!(!feature.missing_control_requirements.is_empty());
    }
    #[test]
    fn empty_snapshot_leaves_capability_scan_unsupported() {
        let snapshot = load_snapshot(Path::new(
            "tests/fixtures/offline-analyzer/missing-mandatory.json",
        ))
        .unwrap();
        let report = evaluate(&snapshot);
        let feature = report
            .features
            .iter()
            .find(|f| f.feature_id == "discovery.capabilityScan")
            .unwrap();
        assert_eq!(feature.support, SupportStatus::Unsupported);
    }
}
