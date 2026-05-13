use crate::model::{
    EvidenceConfidence, FeatureDefinition, MatchMode, Requirement, RequirementGroup,
    SourceCodeReference,
};

pub fn phase1_features() -> Vec<FeatureDefinition> {
    let refs = common_source_refs();
    vec![
        feature("discovery.capabilityScan", "Discovery capability scan", "Discovery", EvidenceConfidence::Medium, false, false, vec![
            req("discovery.capabilityScan.dataModel", "discovery.dataModel.family", "Detected TR data model family", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice.", "device.declaredDataModel"], "The analyzer cannot align snapshot paths with PRISME data model expectations."),
            req("discovery.capabilityScan.parameterPresence", "discovery.parameter.presence", "At least one data model path", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice."], "No CPE data model paths are available for capability discovery."),
            req("discovery.capabilityScan.parameterValues", "discovery.parameter.value", "Representative parameter values", RequirementGroup::Optional, MatchMode::Value, vec!["Device.", "InternetGatewayDevice."], "Capability discovery can identify presence but cannot validate populated runtime values."),
        ], refs.clone()),
        feature("discovery.platformFeatureScan", "Platform feature scan", "Discovery", EvidenceConfidence::Medium, false, false, vec![
            req("discovery.platformFeatureScan.dataModel", "discovery.dataModel.family", "Detected TR data model family", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice.", "device.declaredDataModel"], "Feature scan cannot choose the applicable CPE model family."),
            req("discovery.platformFeatureScan.readableEvidence", "discovery.parameter.readable", "Readable parameter evidence", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.", "InternetGatewayDevice."], "Feature scan cannot confirm that PRISME can read data model evidence."),
        ], refs.clone()),
        feature("score.cpe.wifi", "CPE Wi-Fi score", "Score", EvidenceConfidence::High, false, false, vec![
            req("score.cpe.wifi.noise.history", "wifi.radio.noise.history", "Radio noise history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.noise", "Device.WiFi.Radio.{i}.Stats.Noise"], "CPE Wi-Fi score cannot calculate radio noise contribution."),
            req("score.cpe.wifi.utilization.history", "wifi.radio.utilization.history", "Radio utilization history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.utilization", "Device.WiFi.Radio.{i}.Stats.ChannelUtilization"], "CPE Wi-Fi score cannot calculate utilization contribution."),
            req("score.cpe.wifi.clientSignal.history", "wifi.client.rssi.history", "Client signal history", RequirementGroup::Optional, MatchMode::History, vec!["scores.metrics.history.host.signalStrength", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.SignalStrength"], "Score confidence is lower without client signal distribution."),
        ], refs.clone()),
        feature("score.host.wifi", "Host Wi-Fi score", "Score", EvidenceConfidence::High, false, false, vec![
            req("score.host.wifi.signal.history", "wifi.client.rssi.history", "Client signal strength history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.signalStrength", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.SignalStrength"], "Host Wi-Fi score cannot calculate signal contribution."),
            req("score.host.wifi.phyRate.history", "wifi.client.phyRate.history", "Client PHY rate history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.downlinkRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.LastDataDownlinkRate"], "Host Wi-Fi score cannot calculate PHY-rate contribution."),
            req("score.host.wifi.errorRate.history", "wifi.client.retryOrError.history", "Client retry/error history", RequirementGroup::Optional, MatchMode::History, vec!["scores.metrics.history.host.errorsSentRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.Retransmissions"], "Score confidence is lower without retry/error evidence."),
        ], refs.clone()),
        feature("score.cpe.internet", "CPE internet score", "Score", EvidenceConfidence::High, false, false, vec![
            req("score.cpe.internet.latency.history", "speedtest.latency.history", "Latency history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.ping", "Device.IP.Diagnostics.IPPing.DiagnosticsState"], "Internet score cannot calculate latency contribution."),
            req("score.cpe.internet.download.history", "speedtest.download.throughput.history", "Download throughput history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.download", "Device.IP.Diagnostics.DownloadDiagnostics.DiagnosticsState"], "Internet score cannot calculate download throughput contribution."),
            req("score.cpe.internet.upload.history", "speedtest.upload.throughput.history", "Upload throughput history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.upload", "Device.IP.Diagnostics.UploadDiagnostics.DiagnosticsState"], "Internet score cannot calculate upload throughput contribution."),
            req("score.cpe.internet.wan.link", "wan.interface.status", "WAN link status", RequirementGroup::Optional, MatchMode::Value, vec!["Device.Ethernet.Interface.{i}.Status", "Device.IP.Interface.{i}.Status"], "Score context is weaker without WAN interface status."),
        ], refs.clone()),
        feature("selfHealing.remoteChannelManagement", "Remote channel management", "SelfHealing", EvidenceConfidence::High, true, false, vec![
            req("selfHealing.remoteChannelManagement.radio.band", "wifi.radio.band", "Radio operating band", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.OperatingFrequencyBand"], "Channel management cannot identify applicable radio band."),
            req("selfHealing.remoteChannelManagement.channel.current", "wifi.radio.channel.current", "Current radio channel", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.Channel"], "Channel management cannot read current channel."),
            req("selfHealing.remoteChannelManagement.channel.allowed", "wifi.radio.channel.allowedList", "Allowed channel list", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.PossibleChannels", "Device.WiFi.Radio.{i}.ChannelsInUse"], "Channel management cannot identify candidate channels."),
            req("selfHealing.remoteChannelManagement.channel.write", "wifi.radio.channel.current", "Writable channel control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.Radio.{i}.Channel", "Device.WiFi.Radio.{i}.AutoChannelEnable"], "PRISME may recommend a channel but cannot apply channel changes automatically."),
            req("selfHealing.remoteChannelManagement.scan.diagnostic", "wifi.radio.neighborScan", "Neighbor scan diagnostic/action", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["Device.WiFi.NeighboringWiFiDiagnostic.DiagnosticsState", "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.ScanResult"], "Channel decision confidence is lower without scan diagnostics."),
        ], refs.clone()),
        feature("customerCare.wifiSettings", "Customer-care Wi-Fi settings", "CustomerCare", EvidenceConfidence::Medium, false, false, vec![
            req("customerCare.wifiSettings.ssid.read", "wifi.ssid.name", "SSID name readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.SSID.{i}.SSID"], "Customer care cannot display SSID names."),
            req("customerCare.wifiSettings.ap.read", "wifi.accessPoint.config", "Access point configuration readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.AccessPoint.{i}.Enable", "Device.WiFi.AccessPoint.{i}.SSIDReference"], "Customer care cannot display access point configuration."),
            req("customerCare.wifiSettings.security.read", "wifi.security.mode", "Security mode readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.AccessPoint.{i}.Security.ModeEnabled"], "Customer care cannot display Wi-Fi security mode."),
            req("customerCare.wifiSettings.ssid.write", "wifi.ssid.name", "SSID write control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.SSID.{i}.SSID", "Device.WiFi.AccessPoint.{i}.Security.KeyPassphrase"], "Customer care can display settings but cannot update them."),
        ], refs.clone()),
        feature("diagnostics.speedtest", "Speed-test diagnostics", "Diagnostics", EvidenceConfidence::Medium, false, true, vec![
            req("diagnostics.speedtest.download.action", "speedtest.download.command", "Download diagnostic action", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["diagnostics.speedtest.dispatch", "Device.IP.Diagnostics.DownloadDiagnostics.DiagnosticsState"], "PRISME cannot trigger or observe download diagnostics."),
            req("diagnostics.speedtest.upload.action", "speedtest.upload.command", "Upload diagnostic action", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["diagnostics.speedtest.dispatch", "Device.IP.Diagnostics.UploadDiagnostics.DiagnosticsState"], "PRISME cannot trigger or observe upload diagnostics."),
            req("diagnostics.speedtest.results", "speedtest.result.values", "Speed-test result values", RequirementGroup::Optional, MatchMode::Value, vec!["Device.IP.Diagnostics.DownloadDiagnostics.DownloadTransports", "Device.IP.Diagnostics.UploadDiagnostics.UploadTransports", "scores.metrics.history.device.wan.download", "scores.metrics.history.device.wan.upload"], "Diagnostics can be discovered but result support cannot be validated from output parameters."),
        ], refs),
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
    FeatureDefinition { feature_id: id, feature_name: name, category, confidence, runtime_validation_required: true, control_required_for_support: control_required, diagnostic_required_for_support: diagnostic_required, implementation_found: true, requirements, limitations: vec!["Offline analysis cannot prove runtime value freshness, firmware behavior, or command execution success."], source_refs }
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
fn common_source_refs() -> Vec<SourceCodeReference> {
    vec![
        SourceCodeReference {
            file: "docs/offline-analyzer/feature-inventory.md",
            line_start: None,
            line_end: None,
            symbol: "Phase 1 feature inventory",
            proves:
                "Documents source-code-discovered features selected for initial analyzer support.",
        },
        SourceCodeReference {
            file: "docs/offline-analyzer/feature-parameter-dependencies.md",
            line_start: None,
            line_end: None,
            symbol: "Feature dependency draft",
            proves: "Captures reviewed dependency evidence before formal parameter mapping.",
        },
    ]
}
