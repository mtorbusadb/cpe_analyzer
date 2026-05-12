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
| `score.cpe.overall` | Overall CPE QoE score | `deferred` | `score.cpe.wifi`, `score.cpe.internet` | `score.daily.rollup` | none | none | `Report[]` | Score history completeness cannot be inferred from one current value. | Milestone 3 source evidence pending. | Milestone 3 |
| `score.cpe.wifi` | CPE Wi-Fi QoE score | `deferred` | Wi-Fi derived score concepts | metric history concepts | none | none | `Report[]` | Requires source metric and history mapping. | Milestone 3 source evidence pending. | Milestone 3 |
| `score.host.wifi` | Host/client Wi-Fi QoE score | `deferred` | Host Wi-Fi derived score concepts | daily rollup | none | none | `Report[]` | Host identity and history completeness need mapping. | Milestone 3 source evidence pending. | Milestone 3 |
| `score.cpe.internet` | CPE Internet and WAN access score | `deferred` | Internet/WAN score concepts | WAN branch-specific metrics | none | speedtest-derived metrics if present | `Report[]` | Some WAN branches are incomplete in source. | Milestone 3 source evidence pending. | Milestone 3 |
| `selfHealing.remoteChannelManagement` | Remote Channel Management | `deferred` | radio inventory/current channel/channel scoring | allowed channels | `selfHealing.rcm.channelWrite` | `selfHealing.rcm.scanDiagnostics` | `Report[]` | Runtime scan quality and write success cannot be proven offline. | Milestone 4 source evidence pending. | Milestone 4 |
| `selfHealing.aqosDynamicPrioritization` | AQoS dynamic device-aware prioritization | `deferred` | associated devices, score telemetry, host traffic | none confirmed | `selfHealing.qos.priorityControl` | none | `Report[]` | Runtime write acceptance and traffic windows need validation. | Milestone 4 source evidence pending. | Milestone 4 |
| `selfHealing.aqosAirtimeFairnessTuning` | AQoS airtime fairness tuning | `deferred` | associated devices and host traffic | prioritization state | `selfHealing.qos.atfControl` | none | `Report[]` | Runtime ATF write behavior cannot be proven offline. | Milestone 4 source evidence pending. | Milestone 4 |
| `selfHealing.aqosRtsCtsThresholdTuning` | AQoS RTS/CTS threshold tuning | `deferred` | collision metrics and associated devices | none confirmed | `selfHealing.qos.rtsCtsControl` | none | `Report[]` | Effectiveness requires post-action runtime validation. | Milestone 4 source evidence pending. | Milestone 4 |
| `customerCare.wifiSettings` | Customer-care Wi-Fi settings | `deferred` | Wi-Fi full tree and AP/SSID/security settings | radio capabilities | `customerCare.wifi.configurationWrite` | none | `Report[]` | Write success depends on endpoint and firmware behavior. | Milestone 4 source evidence pending. | Milestone 4 |
| `customerCare.topologyMap` | Customer-care topology map | `runtime_only` | `customerCare.topology.deviceServiceMap` | none confirmed | none | none | `Report[]` | Current active path consumes device-service runtime output. | Milestone 4 source evidence pending. | Milestone 4 |
| `diagnostics.speedtest` | Speedtest diagnostics | `runtime_only` | `diagnostics.speedtest.dispatch` | QoE-agent or diagnostics request path | none | `diagnostics.speedtest.qoeAgentAction`, `diagnostics.speedtest.diagnosticsRequest` | `Report[]` | Static JSON cannot prove command execution or result. | Milestone 4 source evidence pending. | Milestone 4 |
| `customerCare.scoreDrilldown` | Customer-care score drill-down | `derived_only` | `customerCare.scoreDrilldown.scoreConsumption` | score histories | none | none | `Report[]` | Consumption support is separate from score calculation support. | Milestone 3 source evidence pending. | Milestone 3 |
| `noc.populationScores` | NOC population score analytics | `deferred` | score document consumption | distribution/series fields | none | none | `Report[]` | Still deferred from concept extraction and needs separate NOC mapping. | Milestone 5 source evidence pending. | Milestone 5 |

## Discovery Mapping Notes

- `discovery.capabilityScan` depends on active capability registries selected by data model family: TR-181 uses `CAPS_TR181_ALL`; TR-098 uses `CAPS_TR098_ALL`.
- `discovery.platformFeatureScan` depends on active feature registries selected by data model family: TR-181 uses `FEATURES_TR181_ALL`; TR-098 uses `FEATURES_TR098_ALL`.
- Capability scan path checks are presence checks; feature scan `GET`, `GET_BY_IDX`, `GET_IDX`, and `COUNT_IF` rules require values.
- `GET_IDX` and `COUNT_IF` also require a `NumberOfEntries` parameter derived from the prefix before `.{i}.`.
- `CHECK_VAL`, `SWITCH`, and `SET` do not introduce new raw input paths, but they are mandatory parts of ordered feature rules when present.
- Commented-out rule blocks are inactive and must not be treated as requirements.

## Current Limitations

- Only Discovery Program dependencies are mapped in this milestone.
- Deferred rows point to planned detailed mapping milestones.
- Runtime-only rows must remain runtime-only unless later JSON input explicitly includes equivalent runtime evidence.
