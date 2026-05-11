# Feature Requirements Draft

## Scope

This document drafts feature-to-concept requirements for the Offline PRISME CPE Compatibility Analyzer.

Current coverage:

- Milestone 1: Discovery Program features only.
- Milestone 2: QoE score calculation features from active aggregation code.

Later milestones will extend this draft with self-healing, customer-care, topology, and diagnostics features from `docs/offline-analyzer/feature-inventory.md`.

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

- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:165` calculates CPE Wi-Fi noise score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:166` calculates CPE Wi-Fi channel-utilization score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:167` composes network interference score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:168` calculates CPE Wi-Fi score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:421` weights host score contributions.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:439` derives CPE Wi-Fi coverage score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:444` derives CPE Wi-Fi traffic score.
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
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:388` derives host Wi-Fi traffic.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:394` calculates host Wi-Fi score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:397` assigns host score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1232` implements host Wi-Fi truth-table fallback behavior.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_signal_strength.js:34` stores host signal-strength history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_host_last_data_downlink_rate_kbps.js:75` stores host downlink-rate history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_stats_error_rate.js:134` stores host sent-error-rate history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_stats_error_rate.js:153` stores host received-error-rate history.

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
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/ipping/qoe_wan_ping_averageresponsetime_ms.js:140` stores ping average-response-time history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/ipping/qoe_wan_ping_packetLoss.js:154` stores packet-loss history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/download/qoe_download_total_kbps.js:99` stores download history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/upload/qoe_upload_total_kbps.js:127` stores upload history.

Open questions:

- Static analysis must report incomplete WAN access branches as partial or unknown depending on the selected access type and available source evidence.
- The next mapping step must decide how much runtime diagnostic history can be inferred from the supported JSON input format.

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

Deferred to later milestones:

- `noc.populationScores`
- `selfHealing.remoteChannelManagement`
- `selfHealing.aqosDynamicPrioritization`
- `selfHealing.aqosAirtimeFairnessTuning`
- `selfHealing.aqosRtsCtsThresholdTuning`
- `customerCare.wifiSettings`
- `customerCare.topologyMap`
- `diagnostics.speedtest`
- `customerCare.scoreDrilldown`

Deferred reason:

- The locked plan separates Discovery, score calculation, self-healing, and customer-care concept extraction into different milestones to keep review scope bounded.
