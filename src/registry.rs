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
            "discovery.rediscovery",
            "Firmware rediscovery workflow",
            "Discovery",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("discovery.rediscovery.capabilityScan", "discovery.capabilityScan", "Underlying capability scan evidence", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice.", "device.declaredDataModel"], "Rediscovery cannot be assessed offline unless the underlying capability scan can be assessed from the snapshot."),
                req("discovery.rediscovery.platformFeatureScan", "discovery.platformFeatureScan", "Underlying platform feature scan evidence", RequirementGroup::Mandatory, MatchMode::Present, vec!["Device.", "InternetGatewayDevice.", "device.declaredDataModel"], "Rediscovery cannot be assessed offline unless the underlying feature scan can be assessed from the snapshot."),
                req("discovery.rediscovery.executionTrigger", "discovery.rediscovery.trigger", "Runtime rediscovery trigger", RequirementGroup::Optional, MatchMode::Diagnostic, vec!["discovery.rediscovery.trigger"], "Offline analyzer can report static readiness but cannot execute rediscovery or monitor live completion."),
            ],
            rediscovery_refs(),
        ),
        feature(
            "score.cpe.overall",
            "Overall CPE QoE score",
            "Score",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("score.cpe.overall.child.wifi", "score.cpe.wifi", "CPE Wi-Fi child score or derivable histories", RequirementGroup::Mandatory, MatchMode::Value, vec!["qoe_cpe_wifi_score", "scores.scores.current.qoe_cpe_wifi_score", "score.cpe.wifi"], "Overall CPE score needs at least one usable child score path; Wi-Fi score evidence is absent."),
                req("score.cpe.overall.child.internet", "score.cpe.internet", "CPE Internet child score or derivable histories", RequirementGroup::Mandatory, MatchMode::Value, vec!["qoe_cpe_internet_score", "scores.scores.current.qoe_cpe_internet_score", "score.cpe.internet"], "Overall CPE score needs at least one usable child score path; Internet score evidence is absent."),
                req("score.cpe.overall.zeroSemantics", "score.qoe.zeroMeansNotAvailable", "Zero-as-not-available score semantics", RequirementGroup::Optional, MatchMode::Present, vec!["qoe_cpe_score", "scores.scores.current.qoe_cpe_score"], "Score output confidence is lower without current score evidence using the source zero-as-not-available semantics."),
                req("score.cpe.overall.dailyHistory", "score.qoe.dailyScoreHistory", "Daily score history", RequirementGroup::Optional, MatchMode::History, vec!["scores.scores.history.qoe_cpe_score", "scores.scores.history.device"], "Trend and drill-down support is degraded without daily score history."),
            ],
            overall_score_refs(),
        ),
        feature(
            "score.cpe.wifi",
            "CPE Wi-Fi score",
            "Score",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("score.cpe.wifi.noise.history", "score.cpe.wifi.noiseHistory", "Radio noise history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.noise", "Device.WiFi.Radio.{i}.Stats.Noise", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.Stats.Noise"], "CPE Wi-Fi score cannot calculate network interference noise input."),
                req("score.cpe.wifi.utilization.history", "score.cpe.wifi.utilizationHistory", "Radio utilization history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.utilization", "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.Utilization", "Device.WiFi.Radio.{i}.Stats.ChannelUtilization", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.ChannelUtilization"], "CPE Wi-Fi score cannot calculate network interference utilization input."),
                req("score.cpe.wifi.clientSignal.history", "score.host.wifi.rssiHistory", "Client signal history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.signalStrength", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.SignalStrength", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.AssociatedDevice.{i}.SignalStrength"], "CPE Wi-Fi score cannot calculate coverage from weighted client signal evidence."),
                req("score.cpe.wifi.clientPhy.history", "score.host.wifi.phyRateHistory", "Client PHY-rate history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.downlinkRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.LastDataDownlinkRate", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.AssociatedDevice.{i}.LastDataTransmitRate"], "CPE Wi-Fi score cannot calculate coverage from weighted PHY-rate evidence."),
                req("score.cpe.wifi.clientError.history", "score.host.wifi.errorRateHistory", "Client error-rate history", RequirementGroup::Optional, MatchMode::History, vec!["scores.metrics.history.host.errorsSentRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.Stats.ErrorsSent", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.Stats.ErrorsSent"], "Score may degrade because traffic/error-rate contribution is missing."),
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
                req("score.host.wifi.signal.history", "score.host.wifi.rssiHistory", "Client signal strength history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.signalStrength", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.SignalStrength", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.AssociatedDevice.{i}.SignalStrength"], "Host Wi-Fi score cannot calculate signal contribution."),
                req("score.host.wifi.phyRate.history", "score.host.wifi.phyRateHistory", "Client PHY-rate history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.host.downlinkRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.LastDataDownlinkRate", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.AssociatedDevice.{i}.LastDataTransmitRate"], "Host Wi-Fi score cannot calculate PHY-rate contribution."),
                req("score.host.wifi.errorRate.history", "score.host.wifi.errorRateHistory", "Client retry/error history", RequirementGroup::Optional, MatchMode::History, vec!["scores.metrics.history.host.errorsSentRate", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.Stats.ErrorsSent", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.Stats.ErrorsSent"], "Host Wi-Fi score can fall back through zero-as-not-available semantics, but traffic confidence is lower."),
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
                req("score.cpe.internet.latency.history", "score.cpe.internet.latencyHistory", "Latency and packet-loss history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.ping", "Device.IP.Diagnostics.IPPing.DiagnosticsState", "InternetGatewayDevice.IPPingDiagnostics.DiagnosticsState"], "Internet score cannot calculate latency contribution."),
                req("score.cpe.internet.download.history", "score.cpe.internet.downloadHistory", "Download throughput history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.download", "Device.IP.Diagnostics.DownloadDiagnostics.DiagnosticsState", "InternetGatewayDevice.DownloadDiagnostics.DiagnosticsState"], "Internet score cannot calculate download throughput contribution."),
                req("score.cpe.internet.upload.history", "score.cpe.internet.uploadHistory", "Upload throughput history", RequirementGroup::Mandatory, MatchMode::History, vec!["scores.metrics.history.device.wan.upload", "Device.IP.Diagnostics.UploadDiagnostics.DiagnosticsState", "InternetGatewayDevice.UploadDiagnostics.DiagnosticsState"], "Internet score cannot calculate upload throughput contribution."),
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
                req("selfHealing.remoteChannelManagement.radio.band", "selfHealing.rcm.radioInventory", "Radio operating band", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.OperatingFrequencyBand", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.OperatingFrequencyBand"], "Channel management cannot identify applicable radio band."),
                req("selfHealing.remoteChannelManagement.channel.current", "selfHealing.rcm.currentChannel", "Current radio channel", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.Channel", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.Channel"], "Channel management cannot read current channel."),
                req("selfHealing.remoteChannelManagement.channel.allowed", "selfHealing.rcm.radioInventory", "Allowed channel list", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.Radio.{i}.PossibleChannels", "Device.WiFi.Radio.{i}.ChannelsInUse", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.PossibleChannels", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.ChannelsInUse"], "Channel management cannot identify candidate channels."),
                req("selfHealing.remoteChannelManagement.channel.scoring", "selfHealing.rcm.channelScoring", "Scan-derived channel scoring evidence", RequirementGroup::Optional, MatchMode::Value, vec!["selfHealing.rcm.scanResult", "Device.WiFi.NeighboringWiFiDiagnostic.Result.", "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.ScanResult"], "Automatic channel choice confidence is degraded without scan-derived scoring evidence."),
                req("selfHealing.remoteChannelManagement.channel.write", "selfHealing.rcm.channelWrite", "Writable channel control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.Radio.{i}.Channel", "Device.WiFi.Radio.{i}.AutoChannelEnable", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.Channel", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.AutoChannelEnable"], "PRISME may recommend a channel but cannot apply channel changes automatically."),
                req("selfHealing.remoteChannelManagement.scan.diagnostic", "selfHealing.rcm.scanDiagnostics", "Neighbor scan diagnostic/action", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["Device.WiFi.NeighboringWiFiDiagnostic.DiagnosticsState", "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.ScanResult"], "Channel decision confidence is lower without scan diagnostics."),
            ],
            rcm_refs(),
        ),

        feature(
            "selfHealing.aqosDynamicPrioritization",
            "AQoS dynamic device-aware prioritization",
            "SelfHealing",
            EvidenceConfidence::High,
            true,
            false,
            vec![
                req("selfHealing.aqosDynamicPrioritization.associatedDevices", "selfHealing.qos.associatedDevices", "Associated device and AP/radio relationship evidence", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.MACAddress", "selfHealing.qos.associatedDevices"], "AQoS cannot select clients without associated-device and MAC/AP evidence."),
                req("selfHealing.aqosDynamicPrioritization.scoreTelemetry", "selfHealing.qos.scoreTelemetry", "Rolling host/CPE Wi-Fi score telemetry", RequirementGroup::Mandatory, MatchMode::History, vec!["selfHealing.qos.scoreTelemetry", "scores.scores.history.host.wifi", "scores.scores.history.device.wifi", "scores.metrics.history.host.wifiTrafficScore"], "AQoS prioritization cannot evaluate score gates without rolling score telemetry."),
                req("selfHealing.aqosDynamicPrioritization.hostTraffic", "selfHealing.qos.hostTrafficTelemetry", "Host traffic ordering telemetry", RequirementGroup::Optional, MatchMode::History, vec!["selfHealing.qos.hostTrafficTelemetry", "scores.metrics.history.host.traffic"], "Client ordering and prioritization precision are degraded without host traffic history."),
                req("selfHealing.aqosDynamicPrioritization.priorityControl", "selfHealing.qos.priorityControl", "QoS classification and WMM write control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.QoS.Classification.", "Device.WiFi.AccessPoint.{i}.WMMEnable", "selfHealing.qos.priorityControl"], "AQoS may identify clients but cannot automatically apply WMM/QoS prioritization."),
            ],
            aqos_prioritization_refs(),
        ),
        feature(
            "selfHealing.aqosAirtimeFairnessTuning",
            "AQoS airtime fairness tuning",
            "SelfHealing",
            EvidenceConfidence::High,
            true,
            false,
            vec![
                req("selfHealing.aqosAirtimeFairnessTuning.associatedDevices", "selfHealing.qos.associatedDevices", "Associated device and client type evidence", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.MACAddress", "selfHealing.qos.associatedDevices"], "Airtime fairness tuning cannot identify station targets without associated-device evidence."),
                req("selfHealing.aqosAirtimeFairnessTuning.hostTraffic", "selfHealing.qos.hostTrafficTelemetry", "Host traffic telemetry", RequirementGroup::Mandatory, MatchMode::History, vec!["selfHealing.qos.hostTrafficTelemetry", "scores.metrics.history.host.traffic"], "Airtime fairness tuning cannot identify greedy or legacy traffic candidates without traffic history."),
                req("selfHealing.aqosAirtimeFairnessTuning.priorityState", "selfHealing.qos.priorityControl", "Prioritized-host state/control evidence", RequirementGroup::Optional, MatchMode::Writable, vec!["Device.QoS.Classification.", "selfHealing.qos.priorityControl"], "Greedy-client candidate filtering is degraded without prioritization-state/control evidence."),
                req("selfHealing.aqosAirtimeFairnessTuning.atfControl", "selfHealing.qos.atfControl", "Airtime fairness write control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.SSID.{i}.X_ADB_AirTimeFairnessEnable", "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.X_ADB_AirTimeFairness", "selfHealing.qos.atfControl"], "AQoS may identify ATF candidates but cannot automatically apply station or SSID airtime fairness settings."),
            ],
            aqos_atf_refs(),
        ),
        feature(
            "selfHealing.aqosRtsCtsThresholdTuning",
            "AQoS RTS/CTS threshold tuning",
            "SelfHealing",
            EvidenceConfidence::High,
            true,
            false,
            vec![
                req("selfHealing.aqosRtsCtsThresholdTuning.associatedDevices", "selfHealing.qos.associatedDevices", "Associated device grouped-by-radio evidence", RequirementGroup::Mandatory, MatchMode::Value, vec!["Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.MACAddress", "selfHealing.qos.associatedDevices"], "RTS/CTS tuning cannot group clients by radio without associated-device evidence."),
                req("selfHealing.aqosRtsCtsThresholdTuning.scoreTelemetry", "selfHealing.qos.scoreTelemetry", "Host RSSI/score telemetry", RequirementGroup::Mandatory, MatchMode::History, vec!["selfHealing.qos.scoreTelemetry", "scores.scores.history.host.wifi", "scores.metrics.history.host.rssiScore"], "RTS/CTS tuning cannot filter candidate clients without score telemetry."),
                req("selfHealing.aqosRtsCtsThresholdTuning.hostTraffic", "selfHealing.qos.hostTrafficTelemetry", "Host traffic and packet telemetry", RequirementGroup::Mandatory, MatchMode::History, vec!["selfHealing.qos.hostTrafficTelemetry", "scores.metrics.history.host.traffic", "scores.metrics.history.host.packets"], "RTS/CTS tuning cannot calculate traffic and packet activity without telemetry history."),
                req("selfHealing.aqosRtsCtsThresholdTuning.collisionMetrics", "selfHealing.qos.collisionMetrics", "Collision/error-rate metrics", RequirementGroup::Mandatory, MatchMode::History, vec!["selfHealing.qos.collisionMetrics", "scores.metrics.history.radio.collision", "scores.metrics.history.host.packetErrorRate"], "RTS/CTS tuning cannot decide threshold mode without collision/error metrics."),
                req("selfHealing.aqosRtsCtsThresholdTuning.rtsCtsControl", "selfHealing.qos.rtsCtsControl", "RTS/CTS threshold write control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.Radio.{i}.X_ADB_RTSCTS_Threshold", "Device.WiFi.Radio.{i}.X_ADB_RetryLimit", "selfHealing.qos.rtsCtsControl"], "AQoS may detect collision conditions but cannot automatically apply RTS/CTS threshold tuning."),
            ],
            aqos_rts_refs(),
        ),
        feature(
            "customerCare.wifiSettings",
            "Customer-care Wi-Fi settings",
            "CustomerCare",
            EvidenceConfidence::High,
            false,
            false,
            vec![
                req("customerCare.wifiSettings.ssid.read", "customerCare.wifi.apSsidSecuritySettings", "SSID name readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.SSID.{i}.SSID", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.SSID"], "Customer care cannot display SSID names."),
                req("customerCare.wifiSettings.ap.read", "customerCare.wifi.fullTreeRead", "Access point configuration readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.AccessPoint.{i}.Enable", "Device.WiFi.AccessPoint.{i}.SSIDReference", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.Enable", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.BSSID"], "Customer care cannot display access point configuration."),
                req("customerCare.wifiSettings.security.read", "customerCare.wifi.apSsidSecuritySettings", "Security mode readable", RequirementGroup::Mandatory, MatchMode::Readable, vec!["Device.WiFi.AccessPoint.{i}.Security.ModeEnabled", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.BeaconType"], "Customer care cannot display Wi-Fi security mode."),
                req("customerCare.wifiSettings.ssid.write", "customerCare.wifi.configurationWrite", "SSID/security write control", RequirementGroup::Control, MatchMode::Writable, vec!["Device.WiFi.SSID.{i}.SSID", "Device.WiFi.AccessPoint.{i}.Security.KeyPassphrase", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.SSID", "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.PreSharedKey.1.KeyPassphrase"], "Customer care can display settings but cannot update them."),
            ],
            customer_wifi_refs(),
        ),

        feature(
            "customerCare.topologyMap",
            "Customer-care topology map",
            "CustomerCare",
            EvidenceConfidence::Medium,
            false,
            false,
            vec![
                req("customerCare.topologyMap.deviceServiceMap", "customerCare.topology.deviceServiceMap", "Topology payload from device service", RequirementGroup::Mandatory, MatchMode::Value, vec!["customerCare.topology.deviceServiceMap", "topology.map", "topology.flatMap"], "Static CPE parameter JSON does not include the runtime topology map payload consumed by customer care."),
                req("customerCare.topologyMap.bulkFallback", "customerCare.topology.bulkFallback", "Inactive bulk fallback evidence", RequirementGroup::Optional, MatchMode::Value, vec!["customerCare.topology.bulkFallback"], "Bulk fallback code exists but no active call path was confirmed; do not treat it as required support."),
            ],
            topology_refs(),
        ),
        feature(
            "customerCare.scoreDrilldown",
            "Customer-care score drill-down",
            "CustomerCare",
            EvidenceConfidence::Medium,
            false,
            false,
            vec![
                req("customerCare.scoreDrilldown.deviceScore", "customerCare.scoreDrilldown.scoreConsumption", "Device score document", RequirementGroup::Mandatory, MatchMode::Value, vec!["qoe_cpe_score", "scores.scores.current.qoe_cpe_score", "customerCare.score.device"], "Customer-care score drill-down cannot display device score without score document evidence."),
                req("customerCare.scoreDrilldown.hostScore", "customerCare.scoreDrilldown.scoreConsumption", "Host score document", RequirementGroup::Optional, MatchMode::Value, vec!["qoe_host_wifi_score", "scores.scores.current.qoe_host_wifi_score", "customerCare.score.host"], "Host drill-down is degraded without host score evidence."),
                req("customerCare.scoreDrilldown.scoreSeries", "score.qoe.dailyScoreHistory", "Score time series", RequirementGroup::Optional, MatchMode::History, vec!["scores.scores.history", "customerCare.score.series"], "Time-series drill-down is unavailable without score history evidence."),
            ],
            score_drilldown_refs(),
        ),
        feature(
            "noc.populationScores",
            "NOC population score analytics",
            "NOC",
            EvidenceConfidence::Medium,
            false,
            false,
            vec![
                req("noc.populationScores.scoreSeries", "noc.populationScores.scoreConsumption", "Population score series", RequirementGroup::Mandatory, MatchMode::History, vec!["noc.populationScores.scoreSeries", "scores.population.history", "scores.scores.history"], "NOC population analytics cannot build series/distribution output without score history evidence."),
                req("noc.populationScores.currentScores", "noc.populationScores.scoreConsumption", "Current score documents", RequirementGroup::Optional, MatchMode::Value, vec!["noc.populationScores.current", "qoe_cpe_score", "qoe_host_wifi_score"], "Population summary is degraded without current score document evidence."),
            ],
            noc_population_refs(),
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
                req("diagnostics.speedtest.download.action", "diagnostics.speedtest.diagnosticsRequest", "Download diagnostic request path", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["Device.IP.Diagnostics.DownloadDiagnostics.DiagnosticsState", "InternetGatewayDevice.DownloadDiagnostics.DiagnosticsState"], "PRISME cannot trigger or observe download diagnostics."),
                req("diagnostics.speedtest.upload.action", "diagnostics.speedtest.diagnosticsRequest", "Upload diagnostic request path", RequirementGroup::Diagnostic, MatchMode::Diagnostic, vec!["Device.IP.Diagnostics.UploadDiagnostics.DiagnosticsState", "InternetGatewayDevice.UploadDiagnostics.DiagnosticsState"], "PRISME cannot trigger or observe upload diagnostics."),
                req("diagnostics.speedtest.results", "speedtest.result.values", "Speed-test result values", RequirementGroup::Optional, MatchMode::Value, vec!["Device.IP.Diagnostics.DownloadDiagnostics.DownloadTransports", "Device.IP.Diagnostics.UploadDiagnostics.UploadTransports", "InternetGatewayDevice.DownloadDiagnostics.TotalBytesReceived", "InternetGatewayDevice.UploadDiagnostics.TotalBytesSent", "scores.metrics.history.device.wan.download", "scores.metrics.history.device.wan.upload"], "Diagnostics can be discovered but result support cannot be validated from output parameters."),
            ],
            speedtest_refs(),
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
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

fn rediscovery_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts",
            54,
            "rediscoverAllDevices",
            "Exposes all-device rediscovery mutation.",
        ),
        src(
            "prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts",
            65,
            "rediscoverFirmware",
            "Exposes per-firmware rediscovery mutation.",
        ),
        src(
            "prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts",
            34,
            "useDeviceRediscoveryAndMonitor",
            "Triggers rediscovery and starts monitoring.",
        ),
        src(
            "prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts",
            41,
            "monitoring",
            "Polls rediscovery progress.",
        ),
        src(
            "prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts",
            50,
            "refetch",
            "Refreshes capabilities and states during monitoring.",
        ),
    ]
}

fn overall_score_refs() -> Vec<SourceCodeReference> {
    vec![
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 825, "internet_score", "Composes Internet score before overall CPE score."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 828, "cpe_score", "Composes CPE score from Wi-Fi and Internet scores."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 834, "qoe_cpe_score", "Emits overall CPE score."),
        src("tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js", 1165, "score_min_ignore_na", "Implements zero-as-not-available min semantics."),
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

fn aqos_prioritization_refs() -> Vec<SourceCodeReference> {
    vec![
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go", 83, "host traffic score", "Reads rolling host Wi-Fi traffic score."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go", 111, "host coverage score", "Reads rolling host Wi-Fi coverage score."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go", 151, "host traffic", "Reads host traffic for importance ordering."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go", 263, "PrioritizeTraffic", "Executes traffic prioritization."),
        src("prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go", 80, "WMM", "Enables WMM on the touched access point."),
        src("prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go", 98, "QoS classification", "Adds QoS classification objects."),
    ]
}

fn aqos_atf_refs() -> Vec<SourceCodeReference> {
    vec![
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go", 190, "associated devices", "Reads associated devices for ATF candidates."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go", 199, "prioritized hosts", "Reads prioritized host list."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go", 288, "legacy ATF", "Applies legacy-client ATF."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go", 523, "host traffic", "Reads host traffic for ATF ordering."),
        src("prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go", 37, "SetAtf", "Enables airtime fairness."),
        src("prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go", 44, "station ATF", "Builds per-station ATF key."),
    ]
}

fn aqos_rts_refs() -> Vec<SourceCodeReference> {
    vec![
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go", 209, "threshold setting", "Wraps RTS/CTS threshold setting."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go", 482, "radio metrics", "Collects radio metrics."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go", 502, "host RSSI", "Checks host RSSI score."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go", 512, "host traffic", "Reads host traffic."),
        src("prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go", 524, "host packets", "Reads host packet activity."),
        src("prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go", 23, "RTS threshold", "Writes RTS threshold."),
        src("prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go", 30, "radio write", "Sends radio write request."),
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

fn topology_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-backend/services/customer-care-agent/src/rest/handler.go",
            53,
            "map routes",
            "Registers customer-care topology map routes.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/services/map.go",
            12,
            "GetCpeMap",
            "Defines topology map service entrypoint.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/services/map.go",
            27,
            "device-service status",
            "Consumes runtime device-service map output.",
        ),
    ]
}

fn score_drilldown_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-backend/services/customer-care-agent/src/rest/handler.go",
            64,
            "score routes",
            "Registers CPE score route.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/rest/handler.go",
            67,
            "score routes",
            "Registers host score route.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/rest/scores.go",
            12,
            "device score handler",
            "Defines device score handler.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/services/scores.go",
            16,
            "device score service",
            "Retrieves device score data.",
        ),
        src(
            "prisme-backend/services/customer-care-agent/src/services/scores.go",
            92,
            "host score service",
            "Retrieves host score data.",
        ),
    ]
}

fn noc_population_refs() -> Vec<SourceCodeReference> {
    vec![
        src(
            "prisme-backend/services/network-operation-center/src/rest/handler.go",
            23,
            "score routes",
            "Registers NOC score routes.",
        ),
        src(
            "prisme-backend/services/network-operation-center/src/score/service.go",
            23,
            "score service",
            "Defines NOC score service.",
        ),
        src(
            "prisme-backend/services/network-operation-center/src/score/service.go",
            46,
            "score series",
            "Reads population score series.",
        ),
        src(
            "prisme-backend/services/network-operation-center/src/score/service.go",
            79,
            "score distribution",
            "Reads population score distribution.",
        ),
        src(
            "prisme-backend/services/network-operation-center/src/score/service.go",
            104,
            "score summary",
            "Reads population score summary.",
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
