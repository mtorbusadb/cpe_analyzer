# Feature Parameter Dependencies

## Purpose

This document will become the feature-by-feature dependency matrix for analyzer implementation.

Milestone 1 defines the matrix structure and status vocabulary only. It does not yet claim complete feature-to-parameter mappings.

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
| `mappingMilestone` | Planned milestone for detailed mapping. |

## Starter Dependency Matrix

| featureId | featureName | dependencyStatus | mandatoryConcepts | optionalConcepts | controlConcepts | diagnosticConcepts | inputEvidenceSections | runtimeLimitations | mappingMilestone |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `discovery.capabilityScan` | Discovery Program capability scan | `deferred` | `discovery.capability.propertyPresence` | `discovery.capability.propertySet` | none | `discovery.comm.qoeAgent.diagnosticCapability` | `Report[]` | Static JSON cannot execute live probes. | Milestone 2 |
| `discovery.platformFeatureScan` | Discovery Program platform feature scan | `deferred` | `discovery.feature.ruleSet` | indexed/count/value rules | none | none | `Report[]` | Missing values and instances may force unknown/default behavior. | Milestone 2 |
| `score.cpe.overall` | Overall CPE QoE score | `deferred` | `score.cpe.wifi`, `score.cpe.internet` | `score.daily.rollup` | none | none | `Report[]` | Score history completeness cannot be inferred from one current value. | Milestone 3 |
| `score.cpe.wifi` | CPE Wi-Fi QoE score | `deferred` | Wi-Fi derived score concepts | metric history concepts | none | none | `Report[]` | Requires source metric and history mapping. | Milestone 3 |
| `score.host.wifi` | Host/client Wi-Fi QoE score | `deferred` | Host Wi-Fi derived score concepts | daily rollup | none | none | `Report[]` | Host identity and history completeness need mapping. | Milestone 3 |
| `score.cpe.internet` | CPE Internet and WAN access score | `deferred` | Internet/WAN score concepts | WAN branch-specific metrics | none | speedtest-derived metrics if present | `Report[]` | Some WAN branches are incomplete in source. | Milestone 3 |
| `selfHealing.remoteChannelManagement` | Remote Channel Management | `deferred` | radio inventory/current channel/channel scoring | allowed channels | `selfHealing.rcm.channelWrite` | `selfHealing.rcm.scanDiagnostics` | `Report[]` | Runtime scan quality and write success cannot be proven offline. | Milestone 4 |
| `selfHealing.aqosDynamicPrioritization` | AQoS dynamic device-aware prioritization | `deferred` | associated devices, score telemetry, host traffic | none confirmed | `selfHealing.qos.priorityControl` | none | `Report[]` | Runtime write acceptance and traffic windows need validation. | Milestone 4 |
| `selfHealing.aqosAirtimeFairnessTuning` | AQoS airtime fairness tuning | `deferred` | associated devices and host traffic | prioritization state | `selfHealing.qos.atfControl` | none | `Report[]` | Runtime ATF write behavior cannot be proven offline. | Milestone 4 |
| `selfHealing.aqosRtsCtsThresholdTuning` | AQoS RTS/CTS threshold tuning | `deferred` | collision metrics and associated devices | none confirmed | `selfHealing.qos.rtsCtsControl` | none | `Report[]` | Effectiveness requires post-action runtime validation. | Milestone 4 |
| `customerCare.wifiSettings` | Customer-care Wi-Fi settings | `deferred` | Wi-Fi full tree and AP/SSID/security settings | radio capabilities | `customerCare.wifi.configurationWrite` | none | `Report[]` | Write success depends on endpoint and firmware behavior. | Milestone 4 |
| `customerCare.topologyMap` | Customer-care topology map | `runtime_only` | `customerCare.topology.deviceServiceMap` | none confirmed | none | none | `Report[]` | Current active path consumes device-service runtime output. | Milestone 4 |
| `diagnostics.speedtest` | Speedtest diagnostics | `runtime_only` | `diagnostics.speedtest.dispatch` | QoE-agent or diagnostics request path | none | `diagnostics.speedtest.qoeAgentAction`, `diagnostics.speedtest.diagnosticsRequest` | `Report[]` | Static JSON cannot prove command execution or result. | Milestone 4 |
| `customerCare.scoreDrilldown` | Customer-care score drill-down | `derived_only` | `customerCare.scoreDrilldown.scoreConsumption` | score histories | none | none | `Report[]` | Consumption support is separate from score calculation support. | Milestone 3 |
| `noc.populationScores` | NOC population score analytics | `deferred` | score document consumption | distribution/series fields | none | none | `Report[]` | Still deferred from concept extraction and needs separate NOC mapping. | Milestone 5 |

## Current Limitations

- This matrix is not yet analyzer-ready.
- `deferred` rows point to planned detailed mapping milestones.
- Raw data model path patterns and source metric names will be added only with source-code evidence.
- Runtime-only rows must remain runtime-only unless later JSON input explicitly includes equivalent runtime evidence.
