# Feature Requirements Draft

## Scope

This document drafts feature-to-concept requirements for the Offline PRISME CPE Compatibility Analyzer.

Current coverage:

- Milestone 1: Discovery Program features only.
- Milestone 2: QoE score calculation features from active aggregation code.
- Milestone 3: Self-healing workflows and control actions.

Later milestones will extend this draft with customer-care, topology, and diagnostics features from `docs/offline-analyzer/feature-inventory.md`.

Mapping pass note:

- Input JSON mapping Milestone 2 confirms Discovery capability and platform-feature dependencies in `docs/offline-analyzer/concept-parameter-mapping.md` and `docs/offline-analyzer/feature-parameter-dependencies.md`.
- Discovery raw path evidence must be generated from the active TR-181/TR-098 registries selected by `device_processor.py`, while rule-chain semantics come from the Discovery processors.
- Commented-out or TODO-only Discovery rule branches remain inactive and must not be used as active analyzer requirements.
- Input JSON mapping Milestone 3 confirms active QoE score dependencies as derived metric histories and score output fields; score consumption remains separate from score calculation.
- GPON, L2TP, and xDSL-standard WAN access score branches remain incomplete/TBD where the source code says so.

This is not the final support-rule registry. It does not claim whether a specific device snapshot is supported, partial, unsupported, or unknown.

## Requirement Format

Each feature entry includes:

- `featureId`
- feature name
- category
- importance
- mandatory concept candidates
- optional/degrading concept candidates
- control/write concepts
- diagnostic/action concepts
- required granularity
- history/window/frequency needs
- fallback behavior
- evidence references
- open questions

Concept IDs refer to `docs/offline-analyzer/capability-concepts.md`.

## Draft Requirements

### `discovery.capabilityScan`

- Feature name: Discovery Program capability scan
- Category: Discovery
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Discovery Program capability scan`

Mandatory concept candidates:

- `discovery.dataModel.family`
- `discovery.capability.suite`
- `discovery.capability.module`
- `discovery.capability.requirement`
- `discovery.capability.propertyPresence`
- `discovery.capability.status`

Optional/degrading concept candidates:

- `discovery.capability.propertySet`
- `discovery.capability.score`
- `discovery.comm.qoeAgent.diagnosticCapability`

Control/write concepts:

- None found for the capability scanner itself in Milestone 1.

Diagnostic/action concepts:

- `discovery.comm.qoeAgent.diagnosticCapability` when QoE Agent capability suite is included.

Required granularity:

- Firmware/device.
- Capability suite.
- Capability module.
- Individual requirement/property.

History/window/frequency needs:

- No history or collection frequency requirement found in the Discovery Program capability scanner.

Fallback behavior:

- Data model family selects TR-181 capability suites when firmware data model is TR-181; otherwise TR-098 suites are used.
- Empty CPE IP address skips QoE Agent scan.
- Missing mandatory property makes a module unsupported.
- Missing recommended property makes a module partial if mandatory requirements pass.
- Optional-only modules are considered supported by current scanner logic.

Evidence references:

- `prisme-backend/services/discovery-program/src/device_processor.py:111` selects TR-181 capability suites.
- `prisme-backend/services/discovery-program/src/device_processor.py:114` selects TR-098 capability suites.
- `prisme-backend/services/discovery-program/src/device_processor.py:121` creates the capability scanner.
- `prisme-backend/services/discovery-program/src/device_processor.py:123` adds the selected capability suites to the scanner.
- `prisme-backend/services/discovery-program/src/device_processor.py:124` conditionally adds QoE Agent capability suite.
- `prisme-backend/services/discovery-program/src/device_processor.py:127` logs that QoE Agent scan is skipped when IP address is empty.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:8` defines requirement types.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:27` defines capability requirements.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:76` computes module support status.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:259` evaluates property sets.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:327` analyzes modules.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:343` analyzes suites.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:248` computes overall Discovery capability score.

Open questions:

- The offline analyzer should decide whether to preserve Discovery's optional-only-is-supported behavior exactly or annotate it with static-analysis limitations.
- The final implementation needs to determine how input JSON report data represents absent paths, present paths with no value, and object-only entries.

### `discovery.platformFeatureScan`

- Feature name: Discovery Program platform feature scan
- Category: Discovery
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Discovery Program platform feature scan`

Mandatory concept candidates:

- `discovery.dataModel.family`
- `discovery.feature.ruleSet`
- `discovery.feature.dataModelPresenceRule`
- `discovery.feature.parameterValueRule`
- `discovery.feature.valueComparisonRule`

Optional/degrading concept candidates:

- `discovery.feature.indexedValueRule`
- `discovery.feature.countMatchingInstancesRule`
- `discovery.feature.valueSwitchRule`
- `discovery.feature.constantRule`

Control/write concepts:

- None found in the platform feature rule processor. Rules read data model availability/values and produce feature outputs.

Diagnostic/action concepts:

- None found directly in platform feature scan rules in Milestone 1.

Required granularity:

- Firmware/device.
- Rule context.
- Rule set.
- Individual rule.
- Object instance when rules use `.{i}.` instance placeholders.

History/window/frequency needs:

- No history or collection frequency requirement found in the platform feature processor.

Fallback behavior:

- Rules execute in defined order.
- If a rule fails, no further rules in that rule set are processed and the rule set default is returned.
- If an intermediate rule returns boolean `false`, processing stops and the default is returned.
- If a boolean is the final evaluated value, it is converted to `"true"` or `"false"`.
- If no evaluated value is produced, the rule set default is used.

Evidence references:

- `prisme-backend/services/discovery-program/src/device_processor.py:139` selects TR-181 feature rules.
- `prisme-backend/services/discovery-program/src/device_processor.py:142` selects TR-098 feature rules.
- `prisme-backend/services/discovery-program/src/device_processor.py:144` creates the platform features processor.
- `prisme-backend/services/discovery-program/src/device_processor.py:147` adds each feature context.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:51` documents ordered rule processing.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:52` documents failure/default behavior.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:56` processes rule sets.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:60` iterates rules.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:70` initializes feature status.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:78` applies the rule set default.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:192` stores feature outputs.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:244` implements data model presence checks.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:111` implements direct parameter value retrieval.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:182` implements indexed instance lookup.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:308` implements count-if instance logic.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:281` implements value comparison.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:400` implements value switching.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:413` implements constant set rules.

Open questions:

- Some TR-098 feature rule sets contain commented TODO logic. Those should remain `unknown` or deferred until active executable rules are confirmed.
- The offline analyzer needs deterministic behavior for rule defaults when a required snapshot value is missing.

### `discovery.rediscovery`

- Feature name: Firmware rediscovery workflow
- Category: Discovery
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Firmware rediscovery workflow`

Mandatory concept candidates:

- `discovery.capabilityScan`
- `discovery.platformFeatureScan`

Optional/degrading concept candidates:

- `discovery.comm.qoeAgent.diagnosticCapability`

Control/write concepts:

- None for offline analyzer scope. Runtime rediscovery triggers live scanning and is explicitly outside offline execution.

Diagnostic/action concepts:

- None for offline analyzer execution. Runtime Discovery may include QoE Agent capability checks when IP address is available.

Required granularity:

- Firmware.
- Device endpoint.

History/window/frequency needs:

- No history or collection frequency requirement found for the offline analyzer.

Fallback behavior:

- Offline analyzer should not execute rediscovery.
- Rediscovery should be represented as a runtime workflow whose offline equivalent is running the analyzer over a static JSON input.

Evidence references:

- `prisme-backend/services/discovery-program/src/device_processor.py:80` starts a Discovery thread for TR-369.
- `prisme-backend/services/discovery-program/src/device_processor.py:85` starts a Discovery thread for TR-069.
- `prisme-backend/services/discovery-program/src/device_processor.py:90` starts a Discovery thread for direct/QoE path.
- `prisme-backend/services/discovery-program/src/device_processor.py:104` prepares capability scanning.
- `prisme-backend/services/discovery-program/src/device_processor.py:133` prepares feature scanning.
- `prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts:54` exposes all-device rediscovery request.
- `prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts:65` exposes per-firmware rediscovery request.
- `prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts:34` triggers rediscovery and monitoring.
- `prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts:50` refetches capabilities and states.

Open questions:

- Rediscovery is not itself an offline feature to execute. It should likely appear in reports as `runtimeValidationRequired` or as context explaining that offline analysis replaces live rediscovery for static snapshots.

### `score.cpe.overall`

- Feature name: Overall CPE QoE score
- Category: Score
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `QoE CPE score calculation`

Mandatory concept candidates:

- `score.cpe.overall`
- `score.cpe.wifi`
- `score.cpe.internet`
- `score.qoe.zeroMeansNotAvailable`

Optional/degrading concept candidates:

- `score.daily.rollup`

Control/write concepts:

- None found. The score calculator is read/derive/output only.

Diagnostic/action concepts:

- None directly in the aggregate score calculator. Some child score histories may originate from runtime diagnostics such as WAN ping or speed tests.

Required granularity:

- Device-level CPE score.
- Device-level Wi-Fi and Internet child scores.

History/window/frequency needs:

- CPE score itself is an aggregate over child scores.
- Child scores may require metric histories and fresh medians.
- Daily score rollups require stored score history.

Fallback behavior:

- Overall CPE score uses `score_min_ignore_na`, so a missing/zero Wi-Fi or Internet score can be ignored if the other child score is present.
- If both child scores are zero/not available, no positive CPE score is emitted.

Evidence references:

- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:825` composes Internet score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:828` composes CPE score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:834` emits `qoe_cpe_score`.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1165` implements zero-as-not-available minimum composition.

Open questions:

- Static JSON input can show raw values for one report, but cannot prove enough score history exists unless the input includes the required histories.

### `score.cpe.wifi`

- Feature name: CPE Wi-Fi QoE score
- Category: Score
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `QoE CPE Wi-Fi score calculation`

Mandatory concept candidates:

- `score.cpe.wifi`
- At least two of:
- `score.wifi.coverage`
- `score.wifi.traffic`
- `score.wifi.interference`
- `score.qoe.zeroMeansNotAvailable`

Optional/degrading concept candidates:

- All three child concepts are needed for full-quality truth-table scoring.
- `score.qoe.historyWindowFreshness`

Control/write concepts:

- None found.

Diagnostic/action concepts:

- None found in this score calculation path.

Required granularity:

- Device-level CPE Wi-Fi score.
- Host-level input contribution where weighted host subscores feed CPE Wi-Fi subscores.
- Radio-band granularity for noise and utilization histories.

History/window/frequency needs:

- Host Wi-Fi histories feed weighted CPE Wi-Fi coverage and traffic.
- Noise and utilization use band-specific medians that must be fresh within the default metric history window.

Fallback behavior:

- If more than one of traffic, coverage, and interference is missing/zero, the CPE Wi-Fi score returns zero/not available.
- If exactly one child subscore is missing/zero, the CPE Wi-Fi score falls back to the minimum of the remaining non-zero scores.
- If all three child subscores exist, the CPE Wi-Fi truth table is used.

Evidence references:

- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:171` calculates CPE Wi-Fi noise score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:172` calculates CPE Wi-Fi channel-utilization score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:176` composes network interference score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:177` calculates CPE Wi-Fi score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:421` weights host score contributions.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:439` derives CPE Wi-Fi coverage score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:462` derives CPE Wi-Fi traffic score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1196` implements CPE Wi-Fi truth-table fallback behavior.

Open questions:

- The analyzer should not mark full CPE Wi-Fi score support from raw path presence alone; it needs evidence for the derived child score inputs and history constraints.

### `score.host.wifi`

- Feature name: Host Wi-Fi QoE score
- Category: Score
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `QoE host Wi-Fi score calculation`

Mandatory concept candidates:

- `score.host.wifi`
- At least one of:
- `score.wifi.coverage`
- `score.wifi.traffic`
- `score.qoe.zeroMeansNotAvailable`

Optional/degrading concept candidates:

- Both `score.wifi.coverage` and `score.wifi.traffic` are needed for full truth-table scoring.
- `score.daily.rollup`

Control/write concepts:

- None found.

Diagnostic/action concepts:

- None found in this score calculation path.

Required granularity:

- Per-host.
- Per-radio-band histories for host signal strength, downlink rate, and error-rate inputs.

History/window/frequency needs:

- Host score inputs are based on metric histories and medians.
- Daily host score rollup requires stored host score history.

Fallback behavior:

- Host Wi-Fi score uses coverage and traffic when both exist.
- If one of coverage or traffic is missing/zero, host Wi-Fi falls back to the other score through `score_min_ignore_na`.
- Host score is assigned from host Wi-Fi score in this aggregation path.

Evidence references:

- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:382` derives host Wi-Fi coverage.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:384` derives host Wi-Fi traffic.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:385` calculates host Wi-Fi score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:387` assigns host score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1232` implements host Wi-Fi truth-table fallback behavior.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_signal_strength.js:34` stores host signal-strength history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_host_last_data_downlink_rate_kbps.js:33` stores host downlink-rate history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_stats_error_rate.js:45` stores host sent-error-rate history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_stats_error_rate.js:64` stores host received-error-rate history.

Open questions:

- Host identification and association state need a later mapping pass. This milestone records score concepts only.

### `score.cpe.internet`

- Feature name: CPE Internet QoE score
- Category: Score
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `QoE CPE Internet score calculation`

Mandatory concept candidates:

- `score.cpe.internet`
- At least one implemented child path among:
- `score.internet.performance`
- `score.internet.latency`
- `score.wan.access`
- `score.qoe.zeroMeansNotAvailable`

Optional/degrading concept candidates:

- `score.internet.performance`
- `score.internet.latency`
- `score.wan.access`
- `score.qoe.metricHistory`
- `score.qoe.historyWindowFreshness`

Control/write concepts:

- None found.

Diagnostic/action concepts:

- Runtime WAN ping and speed-test style measurements feed some Internet child histories, but the aggregate score calculator does not execute diagnostics.

Required granularity:

- Device-level Internet score.
- WAN access branch selected by WAN access type.

History/window/frequency needs:

- Download and upload scores require history plus last value.
- Latency score requires WAN ping average-response-time history/last value and packet-loss history/last value.
- WAN access score may require medians or last values depending on branch.

Fallback behavior:

- Internet performance is the zero-as-not-available minimum of download and upload scores.
- Internet score is the zero-as-not-available minimum of performance, latency, and WAN access score.
- Download/upload contract values fall back to `250` if enrichment values are missing.
- WAN access has incomplete branches: xDSL standard, GPON scoring, and L2TP scoring are marked TBD in code.

Evidence references:

- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:642` defines latency score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:672` defines download score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:695` defines upload score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:769` selects WAN access score branch.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:779` implements Ethernet access scoring.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:804` leaves xDSL standard scoring as TBD.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:808` reads GPON inputs but leaves GPON scoring as TBD.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:814` leaves L2TP scoring as TBD.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:825` composes Internet score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/ipping/qoe_wan_ping_averageresponsetime_ms.js:6` stores ping average-response-time history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/ipping/qoe_wan_ping_packetLoss.js:9` stores packet-loss history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/download/qoe_download_total_kbps.js:21` stores download history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/upload/qoe_upload_total_kbps.js:21` stores upload history.

Open questions:

- Static analysis must report incomplete WAN access branches as partial or unknown depending on the selected access type and available source evidence.
- The next mapping step must decide how much runtime diagnostic history can be inferred from the supported JSON input format.

### `selfHealing.remoteChannelManagement`

- Feature name: Remote Channel Management self-healing
- Category: Self-healing
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Remote Channel Management self-healing`

Mandatory concept candidates:

- `selfHealing.runtime.cpeOnline`
- `selfHealing.rcm.radioInventory`
- `selfHealing.rcm.currentChannel`
- `selfHealing.rcm.scanDiagnostics`
- `selfHealing.rcm.channelScoring`
- `selfHealing.rcm.channelWrite`

Optional/degrading concept candidates:

- `selfHealing.rcm.allowedChannels`
- `score.cpe.wifi`
- `score.wifi.interference`
- `selfHealing.qos.scoreTelemetry`

Control/write concepts:

- `selfHealing.rcm.channelWrite`

Diagnostic/action concepts:

- `selfHealing.rcm.scanDiagnostics`

Required granularity:

- Device endpoint.
- Radio.
- Radio band.
- Channel.
- Associated client MAC list for filtering own/mesh scan results.

History/window/frequency needs:

- RCM can gate execution based on recent Wi-Fi score history after the latest channel change.
- Scan sampling can run multiple iterations and waits for configured time windows.
- Offline snapshots cannot prove CPE online state, diagnostic completion, scan freshness, or channel-switch execution success.

Fallback behavior:

- If current channel is invalid, channel switching aborts.
- If scan/proximity data is empty, best-channel selection returns no channel instead of forcing a switch.
- If all 5 GHz scans are during active pre-CAC, channel switch is skipped.
- If selected channel equals current channel, no write is performed.
- Allowed-channel policy is applied when present; otherwise possible radio channels are used.

Evidence references:

- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:491` initializes radios with `GetRadios`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:508` updates allowed channel list.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:526` reads current channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:555` normalizes RSSI scan results.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:556` normalizes noise results.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:559` normalizes utilization.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:561` selects best channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:626` writes selected channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:62` disables auto-channel mode.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:68` writes channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/scan_wifi.go:229` requests neighboring Wi-Fi diagnostic.
- `prisme-backend/services/self-healing/remote-channel-management/src/utils/algorithm.go:50` defines best-channel scoring.

Open questions:

- The mapping step must distinguish static evidence for radio/channel support from runtime-only evidence for diagnostics, online state, and scan quality.

### `selfHealing.aqosDynamicPrioritization`

- Feature name: AQoS dynamic device-aware prioritization
- Category: Self-healing
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `AQoS dynamic device-aware prioritization`

Mandatory concept candidates:

- `selfHealing.runtime.cpeOnline`
- `selfHealing.qos.associatedDevices`
- `selfHealing.qos.scoreTelemetry`
- `selfHealing.qos.priorityControl`

Optional/degrading concept candidates:

- `selfHealing.qos.hostTrafficTelemetry`

Control/write concepts:

- `selfHealing.qos.priorityControl`

Diagnostic/action concepts:

- None found. The workflow uses score/traffic telemetry and CPE writes, not explicit diagnostics.

Required granularity:

- Device endpoint.
- Associated client MAC.
- Host IP address.
- Access point for WMM enablement.
- QoS classification entries.

History/window/frequency needs:

- Host Wi-Fi traffic and coverage scores are queried over the last hour.
- CPE Wi-Fi network interference score is queried over the last hour.
- Host traffic ordering uses last-hour host traffic.
- Main workflow refreshes prioritization every hour and forces resync on signal/run-once.

Fallback behavior:

- If configured score thresholds are disabled with `NO_SCORE`, the corresponding check passes.
- If a required score returns `NO_SCORE`, the associated check fails.
- If no desired clients exist, existing PRISME QoS rules can be removed and no new priority rules are added.
- Synchronization is skipped when desired MACs match current state and the 24-hour resync interval has not elapsed.

Evidence references:

- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:83` reads rolling host Wi-Fi traffic score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:111` reads rolling host Wi-Fi coverage score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:151` reads host traffic for ordering.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:202` reads associated devices.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:210` checks interference score gate.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:263` executes traffic prioritization.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:365` reads rolling CPE Wi-Fi network interference score.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:46` deletes old QoS classification rules.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:67` reads host by MAC.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:80` enables WMM.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:98` adds QoS classification objects.

Open questions:

- Static input can expose data model write capability, but cannot prove OpenSearch score/traffic history availability unless historical report data is included.

### `selfHealing.aqosAirtimeFairnessTuning`

- Feature name: AQoS airtime fairness tuning
- Category: Self-healing
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `AQoS airtime fairness tuning`

Mandatory concept candidates:

- `selfHealing.runtime.cpeOnline`
- `selfHealing.qos.associatedDevices`
- `selfHealing.qos.hostTrafficTelemetry`
- `selfHealing.qos.atfControl`

Optional/degrading concept candidates:

- `selfHealing.qos.priorityControl`

Control/write concepts:

- `selfHealing.qos.atfControl`

Diagnostic/action concepts:

- None found. The workflow uses associated-device data, prioritization state, host traffic, and ATF writes.

Required granularity:

- Device endpoint.
- Associated client MAC.
- Associated-device/client type.
- Per-station ATF control.
- SSID-wide ATF enable/default controls.

History/window/frequency needs:

- Workflow interval is 10 minutes.
- Host traffic window is 10 minutes.
- Greedy-client state includes cooldown and enter/exit counters.

Fallback behavior:

- Devices without MAC address are filtered out.
- Prioritized hosts are excluded from greedy-client candidates.
- Legacy clients receive a fixed ATF value when not already applied.
- Greedy clients receive a stronger ATF value; clients are reset when airtime falls below exit threshold.
- If no candidates exist, greedy ATF detection returns no changes.

Evidence references:

- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:17` defines the workflow interval.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:18` defines the traffic window.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:190` reads associated devices.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:199` reads prioritized host list.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:231` orders clients by traffic.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:288` applies legacy-client ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:402` applies greedy-client ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:426` resets greedy-client ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:523` reads host traffic.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:37` enables ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:41` sets global ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:45` writes per-station ATF.

Open questions:

- The feature is automatic-write capable only if station ATF and SSID ATF controls are writable and accepted by firmware at runtime.

### `selfHealing.aqosRtsCtsThresholdTuning`

- Feature name: AQoS RTS/CTS threshold tuning
- Category: Self-healing
- Importance: `3`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `AQoS RTS/CTS threshold tuning`

Mandatory concept candidates:

- `selfHealing.runtime.cpeOnline`
- `selfHealing.qos.associatedDevices`
- `selfHealing.qos.scoreTelemetry`
- `selfHealing.qos.hostTrafficTelemetry`
- `selfHealing.qos.collisionMetrics`
- `selfHealing.qos.rtsCtsControl`

Optional/degrading concept candidates:

- None confirmed beyond configurable thresholds/timers.

Control/write concepts:

- `selfHealing.qos.rtsCtsControl`

Diagnostic/action concepts:

- None found. The workflow uses score/traffic/packet telemetry and radio writes.

Required granularity:

- Device endpoint.
- Radio.
- Associated client grouped by radio.
- Client MAC.
- Host score and packet/error activity.
- Radio collision control parameters.

History/window/frequency needs:

- Workflow interval is 10 minutes.
- Host RSSI score is queried over the last 10 minutes.
- Host traffic and packet activity are queried over the last 10 minutes.
- State machine uses validation and cooldown timers.

Fallback behavior:

- Clients with RSSI score at or below minimum are ignored.
- Clients with less than minimum traffic are ignored.
- Received packet error rate is calculated only when received packets are present.
- Threshold is not rewritten when the desired threshold already matches current state.
- State transitions choose disabled, normal, aggressive, stable, or ineffective modes based on metrics and timers.

Evidence references:

- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:16` defines workflow interval.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:17` defines default RTS threshold.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:18` defines normal RTS threshold.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:19` defines aggressive RTS threshold.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:209` wraps RTS/CTS threshold setting.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:284` processes radio state.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:482` collects radio metrics.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:502` checks host RSSI score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:512` reads host traffic.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:524` reads host packet activity.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:533` calculates received error rate.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:23` writes RTS threshold.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:24` writes retry limit.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:25` writes long retry limit.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:30` sends radio write request.

Open questions:

- The offline analyzer must distinguish control support from runtime effectiveness, because this workflow validates improvement over time after applying thresholds.

### `customerCare.wifiSettings`

- Feature name: Customer-care Wi-Fi settings
- Category: Customer care
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Customer-care Wi-Fi settings`

Mandatory concept candidates:

- `customerCare.wifi.fullTreeRead`
- `customerCare.wifi.apSsidSecuritySettings`

Optional/degrading concept candidates:

- `customerCare.wifi.radioCapabilities`

Control/write concepts:

- `customerCare.wifi.configurationWrite` when update support is assessed.

Diagnostic/action concepts:

- None found. This feature reads and writes configuration; it does not execute a diagnostic.

Required granularity:

- Device endpoint.
- Radio.
- Access point.
- SSID.
- Security object.
- Data Elements SSID object where present.

History/window/frequency needs:

- No score history or collection frequency requirement found for this customer-care setting view.

Fallback behavior:

- Radio possible-channel and bandwidth values have default fallbacks in the Wi-Fi suite API layer.
- Update command generation writes only changed AP/radio/security/SSID values.
- No-op updates return without committing a configuration change.
- Runtime write success depends on endpoint type, campaign execution or data model client write behavior, and firmware acceptance.

Evidence references:

- `prisme-backend/services/customer-care-agent/src/rest/handler.go:33` registers legacy Wi-Fi settings routes.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:35` registers Wi-Fi suite settings read route.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:36` registers Wi-Fi suite settings update route.
- `prisme-backend/services/customer-care-agent/src/services/wifi.go:11` delegates legacy Wi-Fi read to management.
- `prisme-backend/services/customer-care-agent/src/services/wifi.go:38` delegates Wi-Fi suite update to management.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:74` initializes the Wi-Fi suite API context.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:96` builds Wi-Fi suite settings output.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:441` starts update command construction.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:465` commits generated Wi-Fi configuration.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:84` reads the full Wi-Fi object tree.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:98` partitions Wi-Fi objects into typed maps.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:147` documents endpoint-specific commit behavior.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:153` executes configuration through the campaign executor for one endpoint type.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:180` writes generated data model objects for the other endpoint type.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/managementApi.ts:17` exposes Wi-Fi suite settings read in the UI.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/managementApi.ts:66` exposes Wi-Fi suite settings update in the UI.

Open questions:

- The offline analyzer should assess read/display support separately from write/control support, because the same customer-care feature has both modes.
- Legacy and Wi-Fi suite settings routes may not have identical data model coverage. Final registry work should prefer active Wi-Fi suite paths unless source evidence shows the legacy route is still required.

### `customerCare.topologyMap`

- Feature name: Customer-care topology map
- Category: Customer care
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Customer-care topology map`

Mandatory concept candidates:

- `customerCare.topology.deviceServiceMap`

Optional/degrading concept candidates:

- None confirmed as active in Milestone 4.

Control/write concepts:

- None found.

Diagnostic/action concepts:

- None found.

Required granularity:

- Device endpoint.
- Structured or flat topology response.
- Gateway, interface, host/client, and mesh-node visibility when provided by the map response.

History/window/frequency needs:

- No explicit history/window requirement found in the active map endpoint path.

Fallback behavior:

- The active service path resolves endpoint identity and returns the device-service status response.
- Bulk host fallback code exists in the repository, but Milestone 4 search found only definitions and no active call site. It is therefore not treated as a current feature requirement.

Evidence references:

- `prisme-backend/services/customer-care-agent/src/rest/handler.go:52` registers structured map route.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:54` registers flat map route.
- `prisme-backend/services/customer-care-agent/src/rest/map.go:11` defines the map response handler.
- `prisme-backend/services/customer-care-agent/src/rest/map.go:28` calls the customer-care map service.
- `prisme-backend/services/customer-care-agent/src/services/map.go:12` defines `GetCpeMap`.
- `prisme-backend/services/customer-care-agent/src/services/map.go:21` resolves endpoint identity.
- `prisme-backend/services/customer-care-agent/src/services/map.go:27` calls device-service status response.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:39` defines a bulk-host fallback function.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:78` defines host-snapshot payload merging.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:176` maps host snapshots to map hosts.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/mapApi.ts:7` exposes the structured topology map query in the UI.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/mapApi.ts:16` exposes the flat topology map query in the UI.

Open questions:

- If the bulk-host fallback is intended to be active, a future source investigation must find or add the call path before the analyzer treats it as a requirement.
- A static input can only validate this feature directly if the input includes topology/map payloads or enough source-derived raw concepts to reconstruct them.

### `diagnostics.speedtest`

- Feature name: Speedtest diagnostics
- Category: Diagnostics
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Speedtest diagnostics`

Mandatory concept candidates:

- `diagnostics.speedtest.dispatch`

Optional/degrading concept candidates:

- `diagnostics.speedtest.qoeAgentAction`
- `diagnostics.speedtest.diagnosticsRequest`

Control/write concepts:

- None found as data model writes.

Diagnostic/action concepts:

- `diagnostics.speedtest.dispatch`
- `diagnostics.speedtest.qoeAgentAction`
- `diagnostics.speedtest.diagnosticsRequest`

Required granularity:

- Device endpoint.
- Device triplet.
- QoE-agent availability metadata.
- VPN IP metadata for the QoE-agent path.
- Diagnostic request configuration for the generic diagnostics path.

History/window/frequency needs:

- No history requirement found for starting the speedtest action.
- Result availability and score impact are runtime concerns outside static start-command support.

Fallback behavior:

- If endpoint metadata indicates QoE-agent availability, customer-care uses the QoE-agent path.
- Otherwise, customer-care publishes a generic diagnostics request.
- Both paths require live runtime services and cannot be proven by a static data model snapshot alone.

Evidence references:

- `prisme-backend/services/customer-care-agent/src/rest/handler.go:32` registers speedtest command route.
- `prisme-backend/services/customer-care-agent/src/rest/speedtest.go:9` defines the speedtest handler.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:11` defines speedtest dispatch.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:20` resolves endpoint identity.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:26` branches on QoE-agent availability metadata.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:32` invokes QoE-agent speedtest.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:38` invokes generic diagnostics speedtest.
- `prisme-backend/services/customer-care-agent/src/speedtest/speedtest.go:23` defines the QoE-agent speedtest request.
- `prisme-backend/services/customer-care-agent/src/speedtest/speedtest.go:42` adds destination IP routing information.
- `prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go:89` builds a diagnostic request.
- `prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go:109` publishes the diagnostic request.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/managementApi.ts:127` exposes the speedtest command in the UI.

Open questions:

- The analyzer should report static action capability and runtime limitations separately. It must not claim a speedtest will complete successfully from path or metadata presence alone.

### `customerCare.scoreDrilldown`

- Feature name: Customer-care score drill-down
- Category: Customer care
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Customer-care score drill-down`

Mandatory concept candidates:

- `customerCare.scoreDrilldown.scoreConsumption`

Optional/degrading concept candidates:

- `score.cpe.overall`
- `score.cpe.wifi`
- `score.host.wifi`
- `score.cpe.internet`
- Time-series score history concepts from Milestone 2 when score-series endpoints are assessed.

Control/write concepts:

- None found.

Diagnostic/action concepts:

- None found. This feature reads derived score documents.

Required granularity:

- Device score.
- Device score series.
- Host score.
- Host score series.
- Host-list score map.
- Time range, bucket, and aggregation fields for series endpoints.

History/window/frequency needs:

- Current score endpoints need current score documents.
- Series endpoints require score history for the requested time range and bucket.
- Score calculation history requirements are documented under Milestone 2 score concepts.

Fallback behavior:

- Device score and series paths branch to STB score APIs when metadata identifies the device as an STB.
- OpenSearch errors are converted before being returned by the customer-care service.
- If score calculation output is unavailable, this feature can expose missing score data but cannot calculate replacement values by itself.

Evidence references:

- `prisme-backend/services/customer-care-agent/src/rest/handler.go:64` registers CPE score route.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:67` registers host score route.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:69` registers host-list score route.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:12` defines device score handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:30` defines device score series handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:62` defines host score handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:86` defines host score series handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:124` defines host-list score handler.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:16` retrieves device score.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:28` reads STB metadata before device score selection.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:53` retrieves device score series.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:92` retrieves host score.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:115` retrieves host score series.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:141` retrieves host-list scores.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/hostApi.ts:38` exposes host-list scores in the UI.

Open questions:

- Final analyzer output should distinguish score calculation support from customer-care score consumption support. The latter can be present while the underlying calculated score is unavailable.

## Inventory Coverage Status

Covered in Milestone 1:

- `discovery.capabilityScan`
- `discovery.platformFeatureScan`
- `discovery.rediscovery`

Covered in Milestone 2:

- `score.cpe.overall`
- `score.cpe.wifi`
- `score.host.wifi`
- `score.cpe.internet`

Covered in Milestone 3:

- `selfHealing.remoteChannelManagement`
- `selfHealing.aqosDynamicPrioritization`
- `selfHealing.aqosAirtimeFairnessTuning`
- `selfHealing.aqosRtsCtsThresholdTuning`

Covered in Milestone 4:

- `customerCare.wifiSettings`
- `customerCare.topologyMap`
- `diagnostics.speedtest`
- `customerCare.scoreDrilldown`

Deferred to later milestones:

- `noc.populationScores`

Deferred reason:

- NOC population score reporting is still deferred because the locked plan covered Discovery, QoE score calculation, self-healing, and customer-care/diagnostics first.
