use crate::model::{
    FeatureAssessment, MissingRequirement, Report, RequirementGroup, SourceCodeReference,
    SupportStatus,
};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorMode {
    Always,
    Never,
}

pub fn render_text_report(report: &Report, color_mode: ColorMode) -> String {
    let mut out = String::new();
    writeln!(out, "PRISME Offline CPE Compatibility Report").unwrap();
    writeln!(out, "reportFormatVersion: {}", report.report_format_version).unwrap();
    writeln!(out, "assessmentBasis: {}", report.assessment_basis).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Device").unwrap();
    writeln!(out, "  vendor: {}", v(report.device.vendor.as_deref())).unwrap();
    writeln!(out, "  model: {}", v(report.device.model.as_deref())).unwrap();
    writeln!(
        out,
        "  firmwareVersion: {}",
        v(report.device.firmware_version.as_deref())
    )
    .unwrap();
    writeln!(
        out,
        "  declaredDataModel: {}",
        v(report.device.declared_data_model.as_deref())
    )
    .unwrap();
    writeln!(out, "  detectedDataModel: {}", report.detected_data_model).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "PRISME Baseline Compatibility").unwrap();
    writeln!(
        out,
        "  prisme-backend: {}",
        report.prisme_baseline.prisme_backend.commit_sha
    )
    .unwrap();
    writeln!(
        out,
        "  prisme-ui: {}",
        report.prisme_baseline.prisme_ui.commit_sha
    )
    .unwrap();
    writeln!(out, "  tss: {}", report.prisme_baseline.tss.commit_sha).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Summary").unwrap();
    for key in [
        "supportedFeatures",
        "partialFeatures",
        "unsupportedFeatures",
        "unknownFeatures",
        "implementationNotFoundFeatures",
    ] {
        writeln!(
            out,
            "  {key}: {}",
            report.summary.get(key).copied().unwrap_or(0)
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "Feature Matrix").unwrap();
    for f in &report.features {
        writeln!(
            out,
            "  - {:<24} {:<42} {}",
            color_status(f.support, color_mode),
            f.feature_id,
            f.feature_name
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "Feature Details").unwrap();
    for f in &report.features {
        render_feature(&mut out, f, color_mode);
    }
    writeln!(out, "Runtime Validation Limitations").unwrap();
    for f in &report.features {
        for l in &f.limitations {
            writeln!(out, "  - {}: {}", f.feature_id, l).unwrap();
        }
    }
    writeln!(out).unwrap();
    writeln!(out, "Source Evidence").unwrap();
    for f in &report.features {
        writeln!(out, "  {}", f.feature_id).unwrap();
        for s in &f.source_refs {
            render_source(&mut out, s);
        }
    }
    out
}
fn render_feature(out: &mut String, f: &FeatureAssessment, color_mode: ColorMode) {
    writeln!(out, "  {}", f.feature_id).unwrap();
    writeln!(out, "    name: {}", f.feature_name).unwrap();
    writeln!(out, "    category: {}", f.category).unwrap();
    writeln!(out, "    support: {}", color_status(f.support, color_mode)).unwrap();
    writeln!(
        out,
        "    evidenceConfidence: {}",
        f.evidence_confidence.as_str()
    )
    .unwrap();
    writeln!(
        out,
        "    runtimeValidationRequired: {}",
        f.runtime_validation_required
    )
    .unwrap();
    if !f.matched_requirements.is_empty() {
        writeln!(out, "    Matched requirements:").unwrap();
        for m in &f.matched_requirements {
            writeln!(
                out,
                "      - {} ({}) [{}]: {}",
                m.rule_id,
                m.concept_id,
                group(m.group),
                m.matched_paths.join(", ")
            )
            .unwrap();
        }
    }
    if f.support == SupportStatus::Unsupported {
        writeln!(out, "    Missing for support:").unwrap();
        for m in f
            .missing_mandatory_requirements
            .iter()
            .chain(f.missing_control_requirements.iter())
            .chain(f.missing_diagnostic_requirements.iter())
        {
            render_missing(out, m);
        }
    }
    if !f.missing_optional_requirements.is_empty() {
        writeln!(out, "    Missing optional / degraded evidence:").unwrap();
        for m in &f.missing_optional_requirements {
            render_missing(out, m);
        }
    }
    if f.support != SupportStatus::Unsupported && !f.missing_control_requirements.is_empty() {
        writeln!(out, "    Missing control capability:").unwrap();
        for m in &f.missing_control_requirements {
            render_missing(out, m);
        }
    }
    if f.support != SupportStatus::Unsupported && !f.missing_diagnostic_requirements.is_empty() {
        writeln!(out, "    Missing diagnostics/actions:").unwrap();
        for m in &f.missing_diagnostic_requirements {
            render_missing(out, m);
        }
    }
    writeln!(out).unwrap();
}
fn render_missing(out: &mut String, m: &MissingRequirement) {
    writeln!(
        out,
        "      - {} ({}) [{}]",
        m.rule_id,
        m.concept_id,
        group(m.group)
    )
    .unwrap();
    writeln!(out, "        expected: {}", m.label).unwrap();
    writeln!(out, "        candidates: {}", m.candidate_paths.join(", ")).unwrap();
    writeln!(out, "        impact: {}", m.impact).unwrap();
}
fn render_source(out: &mut String, s: &SourceCodeReference) {
    match (s.line_start, s.line_end) {
        (Some(a), Some(b)) => writeln!(
            out,
            "    - {}:{}-{} {}: {}",
            s.file, a, b, s.symbol, s.proves
        ),
        _ => writeln!(out, "    - {} {}: {}", s.file, s.symbol, s.proves),
    }
    .unwrap();
}
fn v(value: Option<&str>) -> &str {
    value.unwrap_or("unknown")
}
fn group(g: RequirementGroup) -> &'static str {
    match g {
        RequirementGroup::Mandatory => "mandatory",
        RequirementGroup::Optional => "optional",
        RequirementGroup::Control => "control",
        RequirementGroup::Diagnostic => "diagnostic",
    }
}
fn color_status(status: SupportStatus, color_mode: ColorMode) -> String {
    let plain = status.as_str();
    if color_mode == ColorMode::Never {
        return plain.to_string();
    }
    let code = match status {
        SupportStatus::Supported => "32",
        SupportStatus::Partial => "33",
        SupportStatus::Unsupported => "31",
        SupportStatus::Unknown => "36",
        SupportStatus::ImplementationNotFound => "35",
    };
    format!("\u{1b}[{code}m{plain}\u{1b}[0m")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::evaluate;
    use crate::input::load_snapshot;
    use std::path::Path;
    #[test]
    fn text_report_contains_required_contract_lines() {
        let snapshot = load_snapshot(Path::new(
            "tests/fixtures/offline-analyzer/missing-mandatory.json",
        ))
        .unwrap();
        let report = evaluate(&snapshot);
        let text = render_text_report(&report, ColorMode::Never);
        assert!(text.contains("PRISME Offline CPE Compatibility Report"));
        assert!(text.contains("reportFormatVersion: 1"));
        assert!(text.contains("PRISME Baseline Compatibility"));
        assert!(text.contains("prisme-backend:"));
        assert!(!text.contains("prisme-backend: unknown"));
        assert!(!text.contains("prisme-ui: unknown"));
        assert!(!text.contains("tss: unknown"));
        assert!(text.contains("Missing for support"));
        assert!(!text.contains("\u{1b}["));
    }

    #[test]
    fn status_colors_follow_contract_palette() {
        assert_eq!(
            color_status(SupportStatus::Supported, ColorMode::Always),
            "\u{1b}[32msupported\u{1b}[0m"
        );
        assert_eq!(
            color_status(SupportStatus::Partial, ColorMode::Always),
            "\u{1b}[33mpartial\u{1b}[0m"
        );
        assert_eq!(
            color_status(SupportStatus::Unsupported, ColorMode::Always),
            "\u{1b}[31munsupported\u{1b}[0m"
        );
        assert_eq!(
            color_status(SupportStatus::Unknown, ColorMode::Always),
            "\u{1b}[36munknown\u{1b}[0m"
        );
        assert_eq!(
            color_status(SupportStatus::ImplementationNotFound, ColorMode::Always),
            "\u{1b}[35mimplementation_not_found\u{1b}[0m"
        );
    }
}
