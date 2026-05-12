# Feature Parameter Dependencies

## Purpose

This document is the feature-by-feature dependency matrix for analyzer implementation.

Milestone 2 maps Discovery Program capability and platform-feature dependencies. Later milestones will add source-backed score, self-healing, diagnostics, customer-care, and NOC mappings.

## Dependency Status Values

| Status | Meaning |
| --- | --- |
| `mapped` | Feature dependency is backed by source-code-confirmed input evidence. |
| `partially_mapped` | Dependency is partly known, but important evidence is still missing. |
| `runtime_only` | Dependency cannot be proven from static JSON alone. |
| `derived_only` | Dependency is based on score/KPI/derived metric evidence. |
| `unknown` | Source or input evidence is insufficient. |
| `implementation_not_found` | Feature reference exists but executable dependency logic was not found. |
| `deferred` | Mapping is intentionally scheduled for a later milestone. |

## Matrix Columns

| Column | Meaning |
| --- | --- |
| `featureId` | Feature ID from `feature-inventory.md` or `feature-requirements-draft.md`. |
| `featureName` | Human-readable feature name. |
| `dependencyStatus` | Current mapping status. |
| `mandatoryConcepts` | Concepts required for core operation. |
| `optionalConcepts` | Concepts that improve quality or coverage. |
| `controlConcepts` | Write/control concepts required for automatic action. |
| `diagnosticConcepts` | Diagnostic/action concepts. |
| `inputEvidenceSections` | JSON sections that can provide evidence. |
| `runtimeLimitations` | What static JSON cannot prove. |
| `sourceEvidence` | Source-code references proving the dependency mapping. |
| `mappingMilestone` | Planned milestone for detailed mapping. |

## Dependency Matrix

| featureId | featureName | dependencyStatus | mandatoryConcepts | optionalConcepts | controlConcepts | diagnosticConcepts | inputEvidenceSections | runtimeLimitations | sourceEvidence | mappingMilestone |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `discovery.capabilityScan` | Discovery Program capability scan | `mapped` | `discovery.dataModel.family`, `discovery.capability.requirementType`, `discovery.capability.propertyPresence` | `discovery.capability.propertySet`, `discovery.capability.moduleStatus`, `discovery.capability.suiteScore` | none | `discovery.comm.qoeAgent.diagnosticCapability` only where QoE-agent suite is selected | `Report[]` parameter/object entries plus metadata declaring TR-181 or TR-098 | Static JSON cannot execute live probes or prove parameter runtime quality. | `prisme-backend/services/discovery-program/src/device_processor.py:111`, `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:8`, `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:259`, `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:327` | Milestone 2 |
| `discovery.platformFeatureScan` | Discovery Program platform feature scan | `mapped` | `discovery.dataModel.family`, `discovery.feature.ruleSet`, `discovery.feature.dataModelPresenceRule`, `discovery.feature.parameterValueRule` | `discovery.feature.indexedParameterValueRule`, `discovery.feature.indexSelectionRule`, `discovery.feature.countMatchingInstancesRule`, `discovery.feature.valueComparisonRule`, `discovery.feature.valueSwitchRule`, `discovery.feature.constantRule` | none | none | `Report[]` parameter/object entries with values where rule type requires values | Missing values, missing `NumberOfEntries`, or incomplete indexed instances may force default results; ordered rule chains must be preserved. | `prisme-backend/services/discovery-program/src/device_processor.py:139`, `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:13`, `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:51`, `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:111`, `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:244`, `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:308` | Milestone 2 |
| `score.cpe.overall` | Overall CPE QoE score | `mapped` | `score.cpe.wifi`, `score.cpe.internet`, `score.qoe.zeroMeansNotAvailable` | `score.qoe.dailyScoreHistory` | none | none | `Report[]` score and history entries | Overall score is derivable only when at least one child score is available; static JSON must include child score evidence or enough histories to derive it. | `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:825`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:828`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:834`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1165` | Milestone 3 |
| `score.cpe.wifi` | CPE Wi-Fi QoE score | `mapped` | `score.host.wifi.rssiHistory`, `score.host.wifi.phyRateHistory`, `score.host.wifi.errorRateHistory`, `score.cpe.wifi.noiseHistory`, `score.cpe.wifi.utilizationHistory`, `score.qoe.zeroMeansNotAvailable` | `score.qoe.dailyScoreHistory` | none | none | `Report[]` host/device metric histories and score entries | Full quality needs traffic, coverage, and interference; more than one missing child returns not-available. | `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:171`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:176`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:177`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1196` | Milestone 3 |
| `score.host.wifi` | Host/client Wi-Fi QoE score | `mapped` | `score.host.wifi.rssiHistory`, `score.host.wifi.phyRateHistory`, `score.host.wifi.errorRateHistory`, host MAC identity, host radio-band association, `score.qoe.zeroMeansNotAvailable` | `score.qoe.dailyScoreHistory` | none | none | `Report[]` host metric histories and score entries | Per-host support needs stable MAC identity, associated AP/DataElements evidence, metric histories, and fresh medians. | `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:225`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:287`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:382`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:398`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1232` | Milestone 3 |
| `score.cpe.internet` | CPE Internet and WAN access score | `partially_mapped` | `score.cpe.internet.latencyHistory`, `score.cpe.internet.downloadHistory`, `score.cpe.internet.uploadHistory`, `score.wan.access.ethernetHistory`, `score.wan.access.dslSpeedHistory`, `score.qoe.zeroMeansNotAvailable` | `score.wan.access.gponHistory`, contract download/upload enrichment, `score.qoe.dailyScoreHistory` | none | WAN ping/download/upload diagnostic result histories if supplied | `Report[]` metric histories and score entries | Ethernet and DSL-speed branches are mapped; GPON/L2TP/xDSL-standard branches are incomplete/TBD in source. | `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:642`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:672`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:695`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:769`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:804`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:808`, `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:814` | Milestone 3 |
| `selfHealing.remoteChannelManagement` | Remote Channel Management | `deferred` | radio inventory/current channel/channel scoring | allowed channels | `selfHealing.rcm.channelWrite` | `selfHealing.rcm.scanDiagnostics` | `Report[]` | Runtime scan quality and write success cannot be proven offline. | Milestone 4 source evidence pending. | Milestone 4 |
| `selfHealing.aqosDynamicPrioritization` | AQoS dynamic device-aware prioritization | `deferred` | associated devices, score telemetry, host traffic | none confirmed | `selfHealing.qos.priorityControl` | none | `Report[]` | Runtime write acceptance and traffic windows need validation. | Milestone 4 source evidence pending. | Milestone 4 |
| `selfHealing.aqosAirtimeFairnessTuning` | AQoS airtime fairness tuning | `deferred` | associated devices and host traffic | prioritization state | `selfHealing.qos.atfControl` | none | `Report[]` | Runtime ATF write behavior cannot be proven offline. | Milestone 4 source evidence pending. | Milestone 4 |
| `selfHealing.aqosRtsCtsThresholdTuning` | AQoS RTS/CTS threshold tuning | `deferred` | collision metrics and associated devices | none confirmed | `selfHealing.qos.rtsCtsControl` | none | `Report[]` | Effectiveness requires post-action runtime validation. | Milestone 4 source evidence pending. | Milestone 4 |
| `customerCare.wifiSettings` | Customer-care Wi-Fi settings | `deferred` | Wi-Fi full tree and AP/SSID/security settings | radio capabilities | `customerCare.wifi.configurationWrite` | none | `Report[]` | Write success depends on endpoint and firmware behavior. | Milestone 4 source evidence pending. | Milestone 4 |
| `customerCare.topologyMap` | Customer-care topology map | `runtime_only` | `customerCare.topology.deviceServiceMap` | none confirmed | none | none | `Report[]` | Current active path consumes device-service runtime output. | Milestone 4 source evidence pending. | Milestone 4 |
| `diagnostics.speedtest` | Speedtest diagnostics | `runtime_only` | `diagnostics.speedtest.dispatch` | QoE-agent or diagnostics request path | none | `diagnostics.speedtest.qoeAgentAction`, `diagnostics.speedtest.diagnosticsRequest` | `Report[]` | Static JSON cannot prove command execution or result. | Milestone 4 source evidence pending. | Milestone 4 |
| `customerCare.scoreDrilldown` | Customer-care score drill-down | `mapped` | `customerCare.scoreDrilldown.scoreConsumption` | score histories and score series | none | none | `Report[]` score and series entries | This maps score consumption only; it does not prove raw score calculation support. | `prisme-backend/libs/goopensearch/scores.go:62`, `prisme-backend/libs/goopensearch/scores.go:92`, `prisme-backend/libs/goopensearch/scores.go:328`, `prisme-backend/libs/goopensearch/scores.go:625`, `prisme-backend/libs/goopensearch/scores.go:642`, `prisme-backend/libs/goopensearch/scores.go:838`, `prisme-backend/services/customer-care-agent/src/rest/scores.go:21`, `prisme-backend/services/customer-care-agent/src/rest/scores.go:77` | Milestone 3 |
| `noc.populationScores` | NOC population score analytics | `deferred` | score document consumption | distribution/series fields | none | none | `Report[]` | Still deferred from concept extraction and needs separate NOC mapping. | Milestone 5 source evidence pending. | Milestone 5 |

## Discovery Mapping Notes

- `discovery.capabilityScan` depends on active capability registries selected by data model family: TR-181 uses `CAPS_TR181_ALL`; TR-098 uses `CAPS_TR098_ALL`.
- `discovery.platformFeatureScan` depends on active feature registries selected by data model family: TR-181 uses `FEATURES_TR181_ALL`; TR-098 uses `FEATURES_TR098_ALL`.
- Capability scan path checks are presence checks; feature scan `GET`, `GET_BY_IDX`, `GET_IDX`, and `COUNT_IF` rules require values.
- `GET_IDX` and `COUNT_IF` also require a `NumberOfEntries` parameter derived from the prefix before `.{i}.`.
- `CHECK_VAL`, `SWITCH`, and `SET` do not introduce new raw input paths, but they are mandatory parts of ordered feature rules when present.
- Commented-out rule blocks are inactive and must not be treated as requirements.

## Score Mapping Notes

- Score calculation support is mapped from active TSS aggregation code, primarily `qoe_scores_calculation.js`.
- Score input support should be evaluated from derived metric histories such as `scores.metrics.history.*`, not from a single raw parameter snapshot unless the input includes equivalent history evidence.
- `0` means not available in these score paths; it must not be reported as a valid poor score.
- CPE Wi-Fi uses a truth table when traffic, coverage, and interference are all available; with exactly one missing child it falls back to the minimum of the available children; with more than one missing child it returns not-available.
- Host Wi-Fi uses coverage and traffic; if one is missing it falls back to the other through zero-as-not-available minimum semantics.
- Internet score maps latency, performance, and WAN access paths, but GPON/L2TP/xDSL-standard branches remain incomplete/TBD in the reviewed source.
- Customer-care score drill-down is score consumption through OpenSearch/backend APIs, not score calculation.

## Current Limitations

- Discovery Program and Score/KPI dependencies are mapped through Milestone 3.
- Deferred rows point to planned detailed mapping milestones.
- Runtime-only rows must remain runtime-only unless later JSON input explicitly includes equivalent runtime evidence.
