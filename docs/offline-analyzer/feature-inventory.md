# PRISME Feature Inventory for Offline CPE Compatibility Analyzer

## Scope

This document is the first source-code-derived feature inventory for the Offline PRISME CPE Compatibility Analyzer.

It intentionally does not define the final mapping from PRISME features to CPE data model parameters. That mapping is the next step after the feature list and classification are reviewed.

Reviewed repositories:

- `prisme-backend`: Go and Python backend services, REST APIs, self-healing workflows, discovery program, NOC/customer-care services.
- `prisme-ui`: TypeScript/React dashboards and API clients used to confirm user-visible feature exposure.
- `tss`: Java/Kotlin services and JavaScript report-parser aggregation logic used for QoE/KPI score calculation.

Inclusion rule:

- A feature is listed as active only when there is executable backend/TSS logic, an active route/job/processor, or a current UI/API path connected to it.
- UI labels alone are not enough.
- Obsolete, incomplete, stub-only, or weakly connected candidates are listed separately at the end.

Importance scale:

- `5`: critical to core customer-visible PRISME value, service health, connectivity, or operational decisions.
- `4`: high-value feature used for diagnostics, monitoring, scoring, or support workflows.
- `3`: useful feature that improves visibility or workflow quality but is not central.
- `2`: narrow or supporting feature with limited direct end-user impact.
- `1`: low-impact or mostly internal support capability.

## Active Feature Inventory

### 1. Discovery Program capability scan

- Feature ID candidate: `discovery.capabilityScan`
- Category: Discovery
- Importance: `5`
- Description: Evaluates firmware/device capability suites and modules, producing supported, partial, and unsupported capability status used by PRISME to understand platform readiness.
- Importance rationale: Capability discovery is foundational for determining what PRISME can safely expose or execute for a CPE/platform.
- CPE-derived value usage: Yes. The scanner checks required/recommended/optional capability requirements through a communication interface and stores capability status/statistics. Final parameter mapping is deferred.
- Active implementation evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:18` defines `DiscProgCapabilityScanner`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:76` computes module status from mandatory/recommended/optional results.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:137` computes suite status.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:248` computes an overall discovery score from supported/partial totals.
- `prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts:29` exposes `getDeviceCapabilities`.

### 2. Discovery Program platform feature scan

- Feature ID candidate: `discovery.platformFeatureScan`
- Category: Discovery
- Importance: `5`
- Description: Processes rule sets to derive platform/firmware feature values from device data and stores evaluated/default feature results.
- Importance rationale: This is the closest existing source-code mechanism for deriving PRISME feature availability from CPE data.
- CPE-derived value usage: Yes. Rule processors retrieve parameter values, index into instance paths, and check data model availability. Final parameter mapping is deferred.
- Active implementation evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:15` defines the rule-set processor.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:56` processes rules and evaluates feature status.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:192` stores feature results.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:111` implements parameter value retrieval.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:244` implements data-model availability checks.

### 3. Firmware rediscovery workflow

- Feature ID candidate: `discovery.rediscovery`
- Category: Discovery
- Importance: `4`
- Description: Allows rediscovery of one firmware or all devices, then monitors and refreshes capability/state information.
- Importance rationale: Important operational workflow for refreshing CPE capability knowledge after firmware/device changes.
- CPE-derived value usage: Indirect. The workflow triggers discovery and refreshes status/capability state; detailed value use belongs to capability and feature scan processors.
- Active implementation evidence:
- `prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts:54` defines all-device rediscovery mutation.
- `prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts:65` defines per-firmware rediscovery mutation.
- `prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts:34` triggers rediscovery and starts monitoring.
- `prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts:50` refetches capabilities and states during monitoring.

### 4. Device QoE overall score

- Feature ID candidate: `score.cpe.overall`
- Category: Scores
- Importance: `5`
- Description: Calculates the top-level CPE QoE score from Wi-Fi and Internet components.
- Importance rationale: This is a core PRISME health indicator surfaced across dashboards and customer-care flows.
- CPE-derived value usage: Yes. It consumes derived Wi-Fi and WAN/Internet metrics. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:160` starts main device score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:176` derives Wi-Fi network interference and Wi-Fi score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:825` combines Internet score and WAN access score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:828` combines Wi-Fi and Internet into CPE score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:834` outputs `qoe_cpe_score`.

### 5. CPE Wi-Fi health scoring

- Feature ID candidate: `score.cpe.wifi`
- Category: Scores
- Importance: `5`
- Description: Calculates CPE Wi-Fi score and subscores for coverage, RSSI, PHY rate, error rates, traffic, noise, utilization, and network interference.
- Importance rationale: Wi-Fi quality is a high-impact end-user connectivity feature and is also used by self-healing decisions.
- CPE-derived value usage: Yes. It consumes host/client radio metrics and device-level interference metrics. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:171` calculates noise and channel utilization scores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:382` calculates host Wi-Fi coverage and traffic scores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:439` calculates CPE Wi-Fi compound scores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:544` calculates noise score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:573` calculates channel utilization score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:835` outputs CPE Wi-Fi score and Wi-Fi subscores.

### 6. Host/client Wi-Fi health scoring

- Feature ID candidate: `score.host.wifi`
- Category: Scores
- Importance: `4`
- Description: Calculates per-host score and Wi-Fi subscores for client coverage, RSSI, PHY rate, error rates, and traffic.
- Importance rationale: Helps customer-care and operations identify problematic clients, not only problematic gateways.
- CPE-derived value usage: Yes. It consumes host/client metrics and weights CPE-level aggregation. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:193` starts host score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:382` derives host Wi-Fi coverage and traffic.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:398` outputs host score and host Wi-Fi subscores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:408` also pushes top-level host score to metrics documents.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:67` exposes host score APIs.

### 7. CPE Internet and WAN access scoring

- Feature ID candidate: `score.cpe.internet`
- Category: Scores
- Importance: `5`
- Description: Calculates Internet score and WAN access subscores for latency, download, upload, performance, quality, WAN access type, Ethernet link speed/duplex, DSL speed, and placeholders for GPON/L2TP branches.
- Importance rationale: Internet service quality is central to support and NOC workflows.
- CPE-derived value usage: Yes. It consumes WAN ping, speed, access type, DSL, Ethernet, and optical-related derived metrics. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:642` calculates latency score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:672` calculates download score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:695` calculates upload score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:719` starts WAN access scoring.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:769` selects WAN access type branch.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:844` outputs Internet and WAN access score fields.

### 8. NOC population score analytics

- Feature ID candidate: `noc.populationScores`
- Category: Monitoring
- Importance: `4`
- Description: Exposes population-level score series, distributions, and summaries for NOC dashboards.
- Importance rationale: High-value operational view for fleet health and trend monitoring.
- CPE-derived value usage: Indirect. It queries score documents produced by score calculation pipelines rather than reading raw CPE data directly.
- Active implementation evidence:
- `prisme-backend/services/network-operation-center/src/rest/handler.go:23` registers score series/distribution/summary endpoints.
- `prisme-backend/services/network-operation-center/src/score/service.go:23` converts requested score IDs into `qoe_*_score` fields.
- `prisme-backend/services/network-operation-center/src/score/service.go:46` filters metrics type to `score`.
- `prisme-backend/services/network-operation-center/src/score/service.go:79` implements score distribution query.
- `prisme-backend/services/network-operation-center/src/score/service.go:91` implements score series query.
- `prisme-backend/services/network-operation-center/src/score/service.go:104` implements score summary query.

### 9. Remote Channel Management self-healing

- Feature ID candidate: `selfHealing.remoteChannelManagement`
- Category: Self-healing
- Importance: `5`
- Description: Scans Wi-Fi environment, scores candidate channels, and switches radio channels when a better channel is selected.
- Importance rationale: Directly affects Wi-Fi performance and can automatically remediate interference/channel issues.
- CPE-derived value usage: Yes. It reads radios, allowed channels, current channel, associated clients, scan/ACS/pre-CAC results, and writes channel controls. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `prisme-ui/apps/customer-care-dashboard/src/modules/RemoteManagementModule/features/selfHealing/SelfHealingModal/SelfHealingModalBody/SelfHealingModalBody.tsx:69` exposes support flag for Remote Channel Management.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/selfHealingApi.ts:14` exposes start API call for self-healing algorithms.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:491` retrieves radios at workflow initialization.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:515` selects best channel for a radio.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:554` derives channel selection score inputs.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:561` chooses best channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:14` reads current channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:43` writes selected channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/radio.go:50` reads radio objects and possible channels.

### 10. AQoS dynamic device-aware prioritization

- Feature ID candidate: `selfHealing.aqosDynamicPrioritization`
- Category: Self-healing
- Importance: `4`
- Description: Selects clients for prioritization based on host traffic/coverage conditions and writes QoS classification rules.
- Importance rationale: Can improve perceived service quality for important or affected devices, but is narrower than overall scoring/RCM.
- CPE-derived value usage: Yes. It reads host association data and writes QoS/WMM/classification controls. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `prisme-ui/apps/customer-care-dashboard/src/modules/RemoteManagementModule/features/selfHealing/SelfHealingModal/SelfHealingModalBody/SelfHealingModalBody.tsx:78` exposes support flag for AQoS dynamic prioritization.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:53` defines the dynamic prioritization workflow.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:67` uses host traffic score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:95` checks host coverage score.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:28` implements traffic prioritization.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:67` reads host by MAC address.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:80` enables WMM on touched APs.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:87` adds QoS classification objects.

### 11. AQoS airtime fairness tuning

- Feature ID candidate: `selfHealing.aqosAirtimeFairnessTuning`
- Category: Self-healing
- Importance: `4`
- Description: Detects greedy clients and applies station airtime fairness values to balance Wi-Fi airtime use.
- Importance rationale: Important Wi-Fi optimization feature for shared-medium fairness and poor-client mitigation.
- CPE-derived value usage: Yes. It uses associated devices, prioritized hosts, and host traffic; it writes ATF controls. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `prisme-ui/apps/customer-care-dashboard/src/modules/RemoteManagementModule/features/selfHealing/SelfHealingModal/SelfHealingModalBody/SelfHealingModalBody.tsx:87` exposes support flag for AQoS airtime fairness tuning.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:54` defines the Airtime Fairness workflow.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:191` gets associated devices and prioritized hosts.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:237` uses per-MAC host traffic.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:301` starts greedy-client detection and ATF application logic.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:24` writes station ATF controls.

### 12. AQoS RTS/CTS threshold tuning

- Feature ID candidate: `selfHealing.aqosRtsCtsThresholdTuning`
- Category: Self-healing
- Importance: `3`
- Description: Adjusts radio collision-related thresholds, including RTS threshold and retry limits.
- Importance rationale: Useful optimization for collision scenarios, but more specialized than RCM and dynamic prioritization.
- CPE-derived value usage: Yes. It writes radio collision-tuning controls. The decision input path requires further detailed tracing during parameter mapping.
- Active implementation evidence:
- `prisme-ui/apps/customer-care-dashboard/src/modules/RemoteManagementModule/features/selfHealing/SelfHealingModal/SelfHealingModalBody/SelfHealingModalBody.tsx:96` exposes support flag for AQoS RTS/CTS threshold tuning.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go` contains the collision tuning workflow.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:12` implements collision tuning activity.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:19` builds the radio data model object for tuning.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:30` writes the radio tuning object.

### 13. Customer-care Wi-Fi settings management

- Feature ID candidate: `customerCare.wifiSettings`
- Category: Customer care
- Importance: `4`
- Description: Reads and updates Wi-Fi SSID/AP/radio/security settings through customer-care APIs.
- Importance rationale: Direct support workflow for fixing or changing end-user Wi-Fi configuration.
- CPE-derived value usage: Yes. It reads Wi-Fi objects and writes SSID, AP, radio, channel, security, and advertisement settings. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:33` registers Wi-Fi settings GET/POST routes.
- `prisme-backend/services/customer-care-agent/src/rest/wifi.go:11` handles Wi-Fi settings read API.
- `prisme-backend/services/customer-care-agent/src/rest/wifi.go:29` handles Wi-Fi settings write API.
- `prisme-backend/services/customer-care-agent/src/services/wifi.go:11` calls management read service.
- `prisme-backend/services/customer-care-agent/src/services/wifi.go:38` calls management write service.
- `prisme-backend/services/customer-care-agent/src/management/wifi.go:152` reads Wi-Fi settings.
- `prisme-backend/services/customer-care-agent/src/management/wifi.go:223` reads `Device.WiFi.` data model objects.
- `prisme-backend/services/customer-care-agent/src/management/wifi.go:411` builds Wi-Fi update commands.
- `prisme-backend/services/customer-care-agent/src/management/wifi.go:496` applies Wi-Fi settings.

### 14. Customer-care CPE/host topology map

- Feature ID candidate: `customerCare.topologyMap`
- Category: Customer care
- Importance: `4`
- Description: Provides structured and flat CPE network maps, including gateway, interfaces, hosts, and mesh/client entities.
- Importance rationale: High-value support visualization for understanding connected devices and local topology.
- CPE-derived value usage: Yes, but partly indirect. It calls device-service topology/status output and can merge host snapshot data when map output lacks hosts. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:52` registers CPE map endpoints.
- `prisme-backend/services/customer-care-agent/src/services/map.go:12` retrieves CPE map from device-service.
- `prisme-backend/services/customer-care-agent/src/services/map.go:27` calls `DeviceService.GetStatusResponse`.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:85` detects missing hosts and starts fallback.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:176` maps latest host snapshots to map hosts.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:254` merges hosts into flat map payloads.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:297` merges hosts into structured map payloads.
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go:474` infers Wi-Fi band and uplink type from host source data.

### 15. Speedtest and diagnostics execution

- Feature ID candidate: `diagnostics.speedtest`
- Category: Diagnostics
- Importance: `4`
- Description: Starts speedtest diagnostics from customer care, using QoE agent path when available or diagnostic path otherwise.
- Importance rationale: Speed testing is a common high-value support and troubleshooting action.
- CPE-derived value usage: Indirect and action-oriented. It depends on endpoint metadata, QoE-agent availability, VPN IP, and diagnostic execution path. Final raw parameter mapping is deferred.
- Active implementation evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:32` registers speedtest command route.
- `prisme-backend/services/customer-care-agent/src/rest/speedtest.go:9` handles speedtest API request.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:11` implements speedtest dispatch.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:26` chooses QoE-agent speedtest path when available.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:38` falls back to diagnostic speedtest path.

### 16. Customer-care device and host score drill-down

- Feature ID candidate: `customerCare.scoreDrilldown`
- Category: Customer care
- Importance: `4`
- Description: Exposes CPE, host, and host-list score endpoints and score series for customer-care troubleshooting.
- Importance rationale: Turns calculated QoE scores into actionable support workflows for individual subscribers/devices.
- CPE-derived value usage: Indirect. It consumes score documents generated from CPE-derived metrics rather than computing directly.
- Active implementation evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:64` registers CPE score APIs.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:67` registers host score APIs.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:69` registers host-list score API.
- `prisme-backend/services/customer-care-agent/src/services/scores.go` implements score retrieval and series access through score services/OpenSearch.

## Excluded or Deferred Candidates

### NOC insight top-series stub path

- Candidate: `noc.insights.topSyntheticSeries`
- Evidence: `prisme-backend/services/network-operation-center/src/insight/service.go:21` uses a stub distribution and randomized series for `getSeries`; `prisme-backend/services/network-operation-center/src/insight/service.go:39` builds random top insight IDs.
- Reason: This path appears synthetic/stubbed for the top-series part. It should not be used as CPE data-model compatibility evidence. Category top using OpenSearch may be revisited separately if a concrete CPE-derived issue taxonomy is traced.

### WAN sub-branches with TBD logic

- Candidate: standalone GPON, xDSL standard, and L2TP score features.
- Evidence: `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:804` has `calcXDSLScore()` marked `TBD`; `:808` has `calcGPONScore()` with partial input reads and `TBD`; `:814` has `calcL2TPScore()` marked `TBD`.
- Reason: These are not excluded from Internet/WAN scoring as a whole, but they should not be treated as complete standalone implemented features until the active calculation is confirmed.

### UI-only labels and flags

- Candidate: any feature found only in UI labels, locale strings, enums, or support flags.
- Evidence: Self-healing UI flags in `prisme-ui/apps/customer-care-dashboard/src/modules/RemoteManagementModule/features/selfHealing/SelfHealingModal/SelfHealingModalBody/SelfHealingModalBody.tsx` are used only as feature exposure evidence, not as implementation proof.
- Reason: UI-only references are insufficient by themselves. They require executable backend/TSS evidence before inclusion.

### Generic reboot, factory reset, and firmware upgrade commands

- Candidate: `customerCare.reboot`, `customerCare.factoryReset`, `customerCare.firmwareUpgrade`.
- Evidence: Routes are present in `prisme-backend/services/customer-care-agent/src/rest/handler.go:28`.
- Reason: They are active operational commands, but this inventory focuses on PRISME features whose support depends materially on CPE data model capabilities and calculations. They can be revisited later as control/action capability checks if the analyzer scope expands.

## Next Step

After this feature list is reviewed, the next step is to build the feature-to-capability concept model and then derive source-code-backed mappings from input JSON report data to those concepts.
