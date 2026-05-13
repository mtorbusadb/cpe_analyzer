use cpe_analyzer::evaluator::{evaluate, evaluate_definitions};
use cpe_analyzer::input::load_snapshot;
use cpe_analyzer::model::{
    EvidenceConfidence, FeatureDefinition, MatchMode, Requirement, RequirementGroup,
    SourceCodeReference, SupportStatus,
};
use cpe_analyzer::registry::phase1_features;
use std::path::Path;

#[test]
fn phase1_feature_order_is_deterministic() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/basic-supported.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    let ids: Vec<_> = report
        .features
        .iter()
        .map(|feature| feature.feature_id.as_str())
        .collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted);
}

#[test]
fn matched_paths_and_missing_requirements_are_sorted() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/basic-supported.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature in &report.features {
        for matched in &feature.matched_requirements {
            let mut sorted = matched.matched_paths.clone();
            sorted.sort();
            assert_eq!(matched.matched_paths, sorted, "{}", matched.rule_id);
        }
        let missing_ids: Vec<_> = feature
            .missing_mandatory_requirements
            .iter()
            .chain(feature.missing_optional_requirements.iter())
            .chain(feature.missing_control_requirements.iter())
            .chain(feature.missing_diagnostic_requirements.iter())
            .map(|missing| missing.rule_id.as_str())
            .collect();
        let mut sorted = missing_ids.clone();
        sorted.sort();
        assert_eq!(missing_ids, sorted, "{}", feature.feature_id);
    }
}

#[test]
fn registry_source_references_are_feature_specific() {
    for feature in phase1_features() {
        assert!(!feature.source_refs.is_empty(), "{}", feature.feature_id);
        assert!(
            feature
                .source_refs
                .iter()
                .all(|source| source.line_start.is_some()),
            "{} still uses placeholder evidence",
            feature.feature_id
        );
        assert!(
            feature
                .source_refs
                .iter()
                .all(|source| !source.file.starts_with("docs/offline-analyzer/")),
            "{} should reference source-code evidence, not only docs",
            feature.feature_id
        );
    }
}

#[test]
fn tr098_snapshot_supports_discovery_but_not_tr181_only_features() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/tr098-discovery.json",
    ))
    .unwrap();
    assert_eq!(snapshot.detected_data_model, "TR-098");
    let report = evaluate(&snapshot);
    let discovery = feature_status(&report, "discovery.capabilityScan");
    let rcm = feature_status(&report, "selfHealing.remoteChannelManagement");
    assert_eq!(discovery, SupportStatus::Partial);
    assert_eq!(rcm, SupportStatus::Unsupported);
}

#[test]
fn speedtest_runtime_action_evidence_is_partial_not_supported() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/runtime-only.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(
        feature_status(&report, "diagnostics.speedtest"),
        SupportStatus::Partial
    );
}

#[test]
fn custom_registry_covers_supported_unknown_and_implementation_not_found() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/basic-supported.json",
    ))
    .unwrap();
    let report = evaluate_definitions(
        &snapshot,
        &[
            custom_feature(
                "test.staticSupported",
                true,
                false,
                vec![Requirement {
                    rule_id: "test.staticSupported.devicePath",
                    concept_id: "test.device.path",
                    label: "Device path present",
                    group: RequirementGroup::Mandatory,
                    match_mode: MatchMode::Present,
                    candidates: vec!["Device."],
                    impact: "Static device path is required.",
                }],
            ),
            custom_feature("test.unknown", true, true, vec![]),
            custom_feature("test.implementationNotFound", false, true, vec![]),
        ],
    );
    assert_eq!(
        feature_status(&report, "test.staticSupported"),
        SupportStatus::Supported
    );
    assert_eq!(
        feature_status(&report, "test.unknown"),
        SupportStatus::Unknown
    );
    assert_eq!(
        feature_status(&report, "test.implementationNotFound"),
        SupportStatus::ImplementationNotFound
    );
    assert_eq!(report.summary.get("supportedFeatures"), Some(&1));
    assert_eq!(report.summary.get("unknownFeatures"), Some(&1));
    assert_eq!(
        report.summary.get("implementationNotFoundFeatures"),
        Some(&1)
    );
}

fn feature_status(report: &cpe_analyzer::model::Report, feature_id: &str) -> SupportStatus {
    report
        .features
        .iter()
        .find(|feature| feature.feature_id == feature_id)
        .unwrap()
        .support
}

fn custom_feature(
    feature_id: &'static str,
    implementation_found: bool,
    runtime_validation_required: bool,
    requirements: Vec<Requirement>,
) -> FeatureDefinition {
    FeatureDefinition {
        feature_id,
        feature_name: feature_id,
        category: "Test",
        confidence: EvidenceConfidence::Low,
        runtime_validation_required,
        control_required_for_support: false,
        diagnostic_required_for_support: false,
        implementation_found,
        requirements,
        limitations: vec!["test limitation"],
        source_refs: vec![SourceCodeReference {
            file: "test/source.rs",
            line_start: Some(1),
            line_end: Some(1),
            symbol: "test",
            proves: "test evidence",
        }],
    }
}
