use crate::model::{
    EvidenceConfidence, FeatureDefinition, MatchMode, Requirement, RequirementGroup,
    SourceCodeReference,
};

pub fn phase1_features() -> Vec<FeatureDefinition> {
    vec![
        feature(
            "discovery.capabilityScan",
            "Discovery capability scan",
            "Discovery",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("discovery.capabilityScan.dataModel", "discovery.dataModel.family", "Detected TR data model family", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice.", "device.declaredDataModel"], "The analyzer cannot select the active TR-181/TR-098 Discovery registry."),
                req("discovery.capabilityScan.parameterPresence", "discovery.capability.propertyPresence", "Capability parameter or object presence", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice."], "No CPE data model paths are available for capability checks."),
                req("discovery.capabilityScan.propertySet", "discovery.capability.propertySet", "Grouped property-set evidence", RequirementGroup::Optional, MatchMode::Present, vec!["Device.", "InternetGatewayDevice."], "Capability property-set semantics may degrade to coarse path presence."),
            ],
            discovery_capability_refs(),
        ),
        feature(
            "discovery.platformFeatureScan",
            "Platform feature scan",
            "Discovery",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("discovery.platformFeatureScan.dataModel", "discovery.dataModel.family", "Detected TR data model family", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice.", "device.declaredDataModel"], "Feature scan cannot select the active rule set."),
                req("discovery.platformFeatureScan.pathRule", "discovery.feature.dataModelPresenceRule", "Data model presence rule evidence", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice."], "Feature rules based on data model presence cannot be evaluated."),
                req("discovery.platformFeatureScan.valueRule", "discovery.feature.parameterValueRule", "Parameter value rule evidence", RequirementGroup::Optional, MatchMode::Value, vec!["Device.", "InternetGatewayDevice."], "GET and indexed value rules may fall back to rule defaults when values are missing."),
            ],
            discovery_feature_refs(),
        ),
        feature(
            "score.cpe.wifi",
            "CPE Wi-Fi score",
            "Score",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("score.cpe.wifi.noise.history", "score.cpe.wifi.noiseHistory", "Radio noise history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.noise", "Device.WiFi.Radio.{i}.Stats.Noise"], "CPE Wi-Fi score cannot calculate network interference noise input."),
                req("score.cpe.wifi.utilization.history", "score.cpe.wifi.utilizationHistory", "Radio utilization history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.utilization", "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.Utilization", "Device.WiFi.Radio.{i}.Stats.ChannelUtilization"], "CPE Wi-Fi score cannot calculate network interference utilization input."),
                req("score.cpe.wifi.clientSignal.history", "score.host.wifi.rssiHistory", "Client signal history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.signalStrength", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.SignalStrength"], "CPE Wi-Fi score cannot calculate coverage from weighted client signal evidence."),
                req("score.cpe.wifi.clientPhy.history", "score.host.wifi.phyRateHistory", "Client PHY-rate history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.downlinkRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.LastDataDownlinkRate"], "CPE Wi-Fi score cannot calculate coverage from weighted PHY-rate evidence."),
                req("score.cpe.wifi.clientError.history", "score.host.wifi.errorRateHistory", "Client error-rate history", RequirementGroup::Optional, MatchMode::History, vec!["scores.metrics.history.host.errorsSentRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.Stats.ErrorsSent"], "Score may degrade because traffic/error-rate contribution is missing."),
            ],
            cpe_wifi_score_refs(),
        ),
        feature(
            "score.host.wifi",
            "Host Wi-Fi score",
            "Score",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("score.host.wifi.signal.history", "score.host.wifi.rssiHistory", "Client signal strength history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.signalStrength", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.SignalStrength"], "Host Wi-Fi score cannot calculate signal contribution."),
                req("score.host.wifi.phyRate.history", "score.host.wifi.phyRateHistory", "Client PHY-rate history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.downlinkRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.LastDataDownlinkRate"], "Host Wi-Fi score cannot calculate PHY-rate contribution."),
                req("score.host.wifi.errorRate.history", "score.host.wifi.errorRateHistory", "Client retry/error history", RequirementGroup::Optional, MatchMode::History, vec!["scores.metrics.history.host.errorsSentRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.Stats.ErrorsSent"], "Host Wi-Fi score can fall back through zero-as-not-available semantics, but traffic confidence is lower."),
            ],
            host_wifi_score_refs(),
        ),
        feature(
            "score.cpe.internet",
            "CPE internet score",
            "Score",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("score.cpe.internet.latency.history", "score.cpe.internet.latencyHistory", "Latency and packet-loss history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.ping", "Device.IP.Diagnostics.IPPing.DiagnosticsState"], "Internet score cannot calculate latency contribution."),
                req("score.cpe.internet.download.history", "score.cpe.internet.downloadHistory", "Download throughput history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.download", "Device.IP.Diagnostics.DownloadDiagnostics.DiagnosticsState"], "Internet score cannot calculate download throughput contribution."),
                req("score.cpe.internet.upload.history", "score.cpe.internet.uploadHistory", "Upload throughput history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.upload", "Device.IP.Diagnostics.UploadDiagnostics.DiagnosticsState"], "Internet score cannot calculate upload throughput contribution."),
                req("score.cpe.internet.wanAccess.history", "score.wan.access.ethernetHistory", "WAN access metric history", RequirementGroup::Optional, MatchMode::History, vec!["scores.metrics.history.device.wan.access.ethernet", "scores.metrics.history.device.wan.dsl"], "WAN access branch support is partial without mapped Ethernet or DSL-speed history."),
            ],
            internet_score_refs(),
        ),
        feature(
            "selfHealing.remoteChannelManagement",
            "Remote channel management",
            "SelfHealing",
            EvidenceConfidence::High,
            true,
            false,
            vec![
                req("selfHealing.remoteChannelManagement.radio.band", "selfHealing.rcm.radioInventory", "Radio operating band", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.OperatingFrequencyBand"], "Channel management cannot identify applicable radio band."),
                req("selfHealing.remoteChannelManagement.channel.current", "selfHealing.rcm.currentChannel", "Current radio channel", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.Channel"], "Channel management cannot read current channel."),
                req("selfHealing.remoteChannelManagement.channel.allowed", "selfHealing.rcm.radioInventory", "Allowed channel list", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.PossibleChannels", "Device.WiFi.Radio.{i}.ChannelsInUse"], "Channel management cannot identify candidate channels."),
                req("selfHealing.remoteChannelManagement.channel.scoring", "selfHealing.rcm.channelScoring", "Scan-derived channel scoring evidence", RequirementGroup::Optional, MatchMode::Value, vec!["selfHealing.rcm.scanResult", "Device.WiFi.NeighboringWiFiDiagnostic.Result.", "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.ScanResult"], "Automatic channel choice confidence is degraded without scan-derived scoring evidence."),
                req("selfHealing.remoteChannelManagement.channel.write", "selfHealing.rcm.channelWrite", "Writable channel control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.Radio.{i}.Channel", "Device.WiFi.Radio.{i}.AutoChannelEnable"], "PRISME may recommend a channel but cannot apply channel changes automatically."),
                req("selfHealing.remoteChannelManagement.scan.diagnostic", "selfHealing.rcm.scanDiagnostics", "Neighbor scan diagnostic/action", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["Device.WiFi.NeighboringWiFiDiagnostic.DiagnosticsState", "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.ScanResult"], "Channel decision confidence is lower without scan diagnostics."),
            ],
            rcm_refs(),
        ),
        feature(
            "customerCare.wifiSettings",
            "Customer-care Wi-Fi settings",
            "CustomerCare",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("customerCare.wifiSettings.ssid.read", "customerCare.wifi.apSsidSecuritySettings", "SSID name readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.SSID.{i}.SSID"], "Customer care cannot display SSID names."),
                req("customerCare.wifiSettings.ap.read", "customerCare.wifi.fullTreeRead", "Access point configuration readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.AccessPoint.{i}.Enable", "Device.WiFi.AccessPoint.{i}.SSIDReference"], "Customer care cannot display access point configuration."),
                req("customerCare.wifiSettings.security.read", "customerCare.wifi.apSsidSecuritySettings", "Security mode readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.AccessPoint.{i}.Security.ModeEnabled"], "Customer care cannot display Wi-Fi security mode."),
                req("customerCare.wifiSettings.ssid.write", "customerCare.wifi.configurationWrite", "SSID/security write control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.SSID.{i}.SSID", "Device.WiFi.AccessPoint.{i}.Security.KeyPassphrase"], "Customer care can display settings but cannot update them."),
            ],
            customer_wifi_refs(),
        ),
        feature(
            "diagnostics.speedtest",
            "Speed-test diagnostics",
            "Diagnostics",
            EvidenceConfidence::High,
            false,
            true,
            vec![
                req("diagnostics.speedtest.dispatch.action", "diagnostics.speedtest.dispatch", "Speed-test dispatch action", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["diagnostics.speedtest.dispatch"], "PRISME cannot trigger the speed-test command path."),
                req("diagnostics.speedtest.download.action", "diagnostics.speedtest.diagnosticsRequest", "Download diagnostic request path", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["Device.IP.Diagnostics.DownloadDiagnostics.DiagnosticsState"], "PRISME cannot trigger or observe download diagnostics."),
                req("diagnostics.speedtest.upload.action", "diagnostics.speedtest.diagnosticsRequest", "Upload diagnostic request path", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["Device.IP.Diagnostics.UploadDiagnostics.DiagnosticsState"], "PRISME cannot trigger or observe upload diagnostics."),
                req("diagnostics.speedtest.results", "speedtest.result.values", "Speed-test result values", RequirementGroup::Optional, MatchMode::Value, vec!["Device.IP.Diagnostics.DownloadDiagnostics.DownloadTransports", "Device.IP.Diagnostics.UploadDiagnostics.UploadTransports", "scores.metrics.history.device.wan.download", "scores.metrics.history.device.wan.upload"], "Diagnostics can be discovered but result support cannot be validated from output parameters."),
            ],
            speedtest_refs(),
        ),
    ]
}

fn feature(
    id: &'static str,
    name: &'static str,
    category: &'static str,
    confidence: EvidenceConfidence,
    control_required: bool,
    diagnostic_required: bool,
    requirements: Vec<Requirement>,
    source_refs: Vec<SourceCodeReference>,
) -> FeatureDefinition {
    FeatureDefinition {
        feature_id: id,
        feature_name: name,
        category,
        confidence,
        runtime_validation_required: true,
        control_required_for_support: control_required,
        diagnostic_required_for_support: diagnostic_required,
        implementation_found: true,
        requirements,
        limitations: vec![
            "Offline analysis cannot prove runtime value freshness, firmware behavior, or command execution success.",
        ],
        source_refs,
    }
}

fn req(
    rule_id: &'static str,
    concept_id: &'static str,
    label: &'static str,
    group: RequirementGroup,
    match_mode: MatchMode,
    candidates: Vec<&'static str>,
    impact: &'static str,
) -> Requirement {
    Requirement {
        rule_id,
        concept_id,
        label,
        group,
        match_mode,
        candidates,
        impact,
    }
}

fn src(
    file: &'static str,
    line: u32,
    symbol: &'static str,
    proves: &'static str,
) -> SourceCodeReference {
    SourceCodeReference {
        file,
        line_start: Some(line),
        line_end: Some(line),
        symbol,
        proves,
    }
}

fn discovery_capability_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-backend/services/discovery-program/src/device_processor.py",
            111,
            "device_processor",
            "Selects TR-181 capability suites.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/device_processor.py",
            114,
            "device_processor",
            "Selects TR-098 capability suites.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/disc_prog_capability_types.py",
            8,
            "RequirementType",
            "Defines capability requirement types.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py",
            259,
            "analyze_property_set",
            "Evaluates nested property sets.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py",
            327,
            "analyze_module",
            "Computes module support from requirements.",
        ),
    ]
}

fn discovery_feature_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-backend/services/discovery-program/src/device_processor.py",
            139,
            "device_processor",
            "Selects TR-181 platform feature rule sets.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/device_processor.py",
            142,
            "device_processor",
            "Selects TR-098 platform feature rule sets.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/disc_prog_features_types.py",
            51,
            "RuleSet",
            "Documents ordered rule processing.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py",
            111,
            "GET",
            "Reads direct parameter values.",
        ),
        src(
            "prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py",
            244,
            "DataModelHas",
            "Checks data model path presence.",
        ),
    ]
}

fn cpe_wifi_score_refs() -> Vec<SourceCodeReference> {
    vec![
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 167, "network_interference", "Calculates CPE Wi-Fi network interference."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 168, "cpe_wifi_score", "Calculates CPE Wi-Fi score with truth-table semantics."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 439, "weighted_coverage", "Uses weighted RSSI/PHY coverage."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 444, "weighted_traffic", "Uses weighted traffic/error-rate contribution."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 1196, "truth_table", "Implements CPE Wi-Fi score composition."),
    ]
}

fn host_wifi_score_refs() -> Vec<SourceCodeReference> {
    vec![
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 382, "host_coverage", "Derives host coverage from RSSI and PHY-rate."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 388, "host_traffic", "Derives host traffic/error score."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 394, "host_wifi_score", "Calculates host Wi-Fi score."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 1232, "truth_table", "Implements host Wi-Fi score composition."),
    ]
}

fn internet_score_refs() -> Vec<SourceCodeReference> {
    vec![
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 642, "latency_score", "Defines Internet latency score calculation."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 672, "download_score", "Defines download score calculation."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 695, "upload_score", "Defines upload score calculation."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 825, "internet_score", "Composes Internet score from latency and performance."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 779, "wan_access", "Implements Ethernet WAN access branch."),
    ]
}

fn rcm_refs() -> Vec<SourceCodeReference> {
    vec![
        src("prisme-backend/services/self-healing/remote-channel-management/src/activities/radio.go", 65, "getRadios", "Reads Wi-Fi radio inventory."),
        src("prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go", 31, "getChannel", "Reads current channel."),
        src("prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go", 62, "setChannel", "Writes AutoChannelEnable and Channel."),
        src("prisme-backend/services/self-healing/remote-channel-management/src/activities/scan_wifi.go", 54, "scanWiFi", "Uses scan diagnostic evidence."),
        src("prisme-backend/services/self-healing/remote-channel-management/src/workflow.go", 561, "channelScoring", "Uses scan-derived channel scoring."),
        src("prisme-backend/services/self-healing/remote-channel-management/src/workflow.go", 626, "applyChannel", "Applies chosen channel."),
    ]
}

fn customer_wifi_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-backend/services/customer-care-agent/src/rest/handler.go",
            35,
            "management routes",
            "Registers Wi-Fi settings management endpoints.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go",
            83,
            "wifiSuiteApi",
            "Exposes Wi-Fi suite read API.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go",
            382,
            "wifi settings",
            "Reads Wi-Fi AP/SSID/security settings.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go",
            465,
            "wifi settings",
            "Handles Wi-Fi setting updates.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go",
            147,
            "wifiSuiteApi",
            "Exposes Wi-Fi write API.",
        ),
    ]
}

fn speedtest_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-backend/services/customer-care-agent/src/rest/handler.go",
            32,
            "speedtest route",
            "Registers speedtest command route.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/rest/speedtest.go",
            18,
            "speedtest handler",
            "Handles speedtest REST command.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/services/speedtest.go",
            26,
            "speedtest service",
            "Branches on QoE-agent availability.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/services/speedtest.go",
            38,
            "speedtest service",
            "Falls back to diagnostics request path.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go",
            89,
            "diagnostic request",
            "Constructs speedtest diagnostic request.",
        ),
    ]
}
