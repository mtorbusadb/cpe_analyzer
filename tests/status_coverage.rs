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
        assert_sorted_missing(&feature.missing_mandatory_requirements, &feature.feature_id);
        assert_sorted_missing(&feature.missing_optional_requirements, &feature.feature_id);
        assert_sorted_missing(&feature.missing_control_requirements, &feature.feature_id);
        assert_sorted_missing(
            &feature.missing_diagnostic_requirements,
            &feature.feature_id,
        );
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

#[test]
fn rediscovery_is_partial_when_underlying_static_discovery_evidence_exists() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/basic-supported.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    let feature = report
        .features
        .iter()
        .find(|feature| feature.feature_id == "discovery.rediscovery")
        .unwrap();
    assert_eq!(feature.support, SupportStatus::Partial);
    assert!(feature.runtime_validation_required);
    assert!(!feature.missing_optional_requirements.is_empty());
}

#[test]
fn rediscovery_is_unsupported_without_underlying_static_discovery_evidence() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/missing-mandatory.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(
        feature_status(&report, "discovery.rediscovery"),
        SupportStatus::Unsupported
    );
}

#[test]
fn mapped_score_consumption_features_are_assessed_from_score_evidence() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/score-consumption.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(
        feature_status(&report, "score.cpe.overall"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "customerCare.scoreDrilldown"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "noc.populationScores"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "customerCare.topologyMap"),
        SupportStatus::Partial
    );
}

#[test]
fn expanded_mapped_features_are_present_in_default_report() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/missing-mandatory.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "score.cpe.overall",
        "customerCare.scoreDrilldown",
        "customerCare.topologyMap",
        "noc.populationScores",
        "discovery.rediscovery",
    ] {
        assert!(
            report
                .features
                .iter()
                .any(|feature| feature.feature_id == feature_id),
            "missing {feature_id}"
        );
    }
}

#[test]
fn aqos_features_are_present_in_default_report() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/missing-mandatory.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "selfHealing.aqosDynamicPrioritization",
        "selfHealing.aqosAirtimeFairnessTuning",
        "selfHealing.aqosRtsCtsThresholdTuning",
    ] {
        assert!(
            report
                .features
                .iter()
                .any(|feature| feature.feature_id == feature_id),
            "missing {feature_id}"
        );
    }
}

#[test]
fn aqos_control_supported_snapshot_is_partial_due_to_runtime_validation() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/aqos-control-supported.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "selfHealing.aqosDynamicPrioritization",
        "selfHealing.aqosAirtimeFairnessTuning",
        "selfHealing.aqosRtsCtsThresholdTuning",
    ] {
        assert_eq!(
            feature_status(&report, feature_id),
            SupportStatus::Partial,
            "{feature_id}"
        );
    }
}

#[test]
fn aqos_read_only_control_snapshot_is_unsupported_for_automatic_workflows() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/aqos-read-only-control.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "selfHealing.aqosDynamicPrioritization",
        "selfHealing.aqosAirtimeFairnessTuning",
        "selfHealing.aqosRtsCtsThresholdTuning",
    ] {
        let feature = report
            .features
            .iter()
            .find(|feature| feature.feature_id == feature_id)
            .unwrap();
        assert_eq!(feature.support, SupportStatus::Unsupported, "{feature_id}");
        assert!(
            !feature.missing_control_requirements.is_empty(),
            "{feature_id}"
        );
    }
}

#[test]
fn aqos_features_have_source_references_with_line_numbers() {
    for feature in phase1_features()
        .into_iter()
        .filter(|feature| feature.feature_id.starts_with("selfHealing.aqos"))
    {
        assert!(!feature.source_refs.is_empty(), "{}", feature.feature_id);
        assert!(
            feature
                .source_refs
                .iter()
                .all(|source| source.line_start.is_some()),
            "{}",
            feature.feature_id
        );
        assert!(
            feature
                .source_refs
                .iter()
                .all(|source| source.file.contains("quality-of-service")),
            "{}",
            feature.feature_id
        );
    }
}

#[test]
fn p0_feature_statuses_on_basic_supported_fixture_are_stable() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/basic-supported.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(
        feature_status(&report, "discovery.capabilityScan"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "discovery.platformFeatureScan"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "score.cpe.overall"),
        SupportStatus::Unsupported
    );
    assert_eq!(
        feature_status(&report, "score.cpe.wifi"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "score.host.wifi"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "score.cpe.internet"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "selfHealing.remoteChannelManagement"),
        SupportStatus::Partial
    );
}

#[test]
fn p0_feature_statuses_on_minimal_fixture_are_unsupported() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/missing-mandatory.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "discovery.capabilityScan",
        "discovery.platformFeatureScan",
        "score.cpe.overall",
        "score.cpe.wifi",
        "score.host.wifi",
        "score.cpe.internet",
        "selfHealing.remoteChannelManagement",
    ] {
        assert_eq!(
            feature_status(&report, feature_id),
            SupportStatus::Unsupported,
            "{feature_id}"
        );
    }
}

#[test]
fn p1_feature_statuses_on_basic_supported_fixture_are_stable() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/basic-supported.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(
        feature_status(&report, "discovery.rediscovery"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "selfHealing.aqosDynamicPrioritization"),
        SupportStatus::Unsupported
    );
    assert_eq!(
        feature_status(&report, "selfHealing.aqosAirtimeFairnessTuning"),
        SupportStatus::Unsupported
    );
    assert_eq!(
        feature_status(&report, "selfHealing.aqosRtsCtsThresholdTuning"),
        SupportStatus::Unsupported
    );
    assert_eq!(
        feature_status(&report, "customerCare.wifiSettings"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "diagnostics.speedtest"),
        SupportStatus::Unsupported
    );
}

#[test]
fn p1_feature_statuses_on_minimal_fixture_are_unsupported() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/missing-mandatory.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "discovery.rediscovery",
        "selfHealing.aqosDynamicPrioritization",
        "selfHealing.aqosAirtimeFairnessTuning",
        "selfHealing.aqosRtsCtsThresholdTuning",
        "customerCare.wifiSettings",
        "diagnostics.speedtest",
    ] {
        assert_eq!(
            feature_status(&report, feature_id),
            SupportStatus::Unsupported,
            "{feature_id}"
        );
    }
}

#[test]
fn tr098_wifi_and_diagnostics_fixture_exercises_new_candidate_paths() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/tr098-wifi-speedtest.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(snapshot.detected_data_model, "TR-098");
    assert_eq!(
        feature_status(&report, "selfHealing.remoteChannelManagement"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "customerCare.wifiSettings"),
        SupportStatus::Partial
    );
    assert_eq!(
        feature_status(&report, "diagnostics.speedtest"),
        SupportStatus::Partial
    );
}

#[test]
fn tr098_speedtest_non_commandable_diagnostics_are_unsupported() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/tr098-speedtest-readonly-diagnostic.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(snapshot.detected_data_model, "TR-098");
    assert_eq!(
        feature_status(&report, "diagnostics.speedtest"),
        SupportStatus::Unsupported
    );
}

#[test]
fn p2_features_can_be_degraded_partial_with_runtime_heavy_evidence() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/p2-degraded-partial.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "customerCare.topologyMap",
        "customerCare.scoreDrilldown",
        "noc.populationScores",
    ] {
        assert_eq!(
            feature_status(&report, feature_id),
            SupportStatus::Partial,
            "{feature_id}"
        );
        assert_eq!(
            report
                .features
                .iter()
                .find(|f| f.feature_id == feature_id)
                .unwrap()
                .evidence_confidence
                .as_str(),
            EvidenceConfidence::Medium.as_str(),
            "{feature_id}"
        );
    }
}

#[test]
fn tr098_rich_snapshot_drives_all_p0_features_to_partial() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/tr098-p0-rich-partial.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    assert_eq!(snapshot.detected_data_model, "TR-098");
    for feature_id in [
        "discovery.capabilityScan",
        "discovery.platformFeatureScan",
        "score.cpe.overall",
        "score.cpe.wifi",
        "score.host.wifi",
        "score.cpe.internet",
        "selfHealing.remoteChannelManagement",
    ] {
        assert_eq!(
            feature_status(&report, feature_id),
            SupportStatus::Partial,
            "{feature_id}"
        );
    }
}

#[test]
fn vendor_extension_fixture_drives_aqos_features_to_partial() {
    let snapshot = load_snapshot(Path::new(
        "tests/fixtures/offline-analyzer/vendor-aqos-partial.json",
    ))
    .unwrap();
    let report = evaluate(&snapshot);
    for feature_id in [
        "selfHealing.aqosDynamicPrioritization",
        "selfHealing.aqosAirtimeFairnessTuning",
        "selfHealing.aqosRtsCtsThresholdTuning",
    ] {
        assert_eq!(
            feature_status(&report, feature_id),
            SupportStatus::Partial,
            "{feature_id}"
        );
    }
}

fn assert_sorted_missing(missing: &[cpe_analyzer::model::MissingRequirement], feature_id: &str) {
    let ids: Vec<_> = missing.iter().map(|item| item.rule_id.as_str()).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted, "{feature_id}");
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
