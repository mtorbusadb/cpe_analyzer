# Capability Mapping Plan

## Objective

Define the next source-code-derived layer for the Offline PRISME CPE Compatibility Analyzer: canonical capability concepts and a reviewable feature-to-concept requirement model.

This step comes after `docs/offline-analyzer/feature-inventory.md` and before analyzer implementation.

The deliverables for this step are documentation and evidence only. Do not implement analyzer code yet.

## Scope

Input:

- Confirmed feature candidates from `docs/offline-analyzer/feature-inventory.md`.
- Source code under:
  - `/home/mtorbus/hdd/gitrep/prisme/prisme-backend`
  - `/home/mtorbus/hdd/gitrep/prisme/prisme-ui`
  - `/home/mtorbus/hdd/gitrep/prisme/tss`

Output:

- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

This phase must still avoid producing the final raw TR-181/TR-098/vendor path mapping table unless a source file directly defines such a mapping and it must be cited as evidence.

## Non-Goals

Do not:

- implement parser or analyzer code,
- define final JSON/text report schema,
- create final feature support rules,
- claim support/unsupported status for a device snapshot,
- invent requirements from broadband knowledge,
- map every raw data model path exhaustively.

## Deliverable 1: Capability Concepts

File:

- `docs/offline-analyzer/capability-concepts.md`

The document must define canonical concept candidates discovered from source code.

Each concept entry must include:

- `conceptId`
- name
- category
- description
- source-code evidence
- whether the concept is telemetry, static metadata, control/write capability, diagnostic/action, history/window, or derived metric
- confidence: `high`, `medium`, or `low`
- notes on whether the source code directly reads a raw data model parameter, reads a derived metric, or uses an API/DTO field

Examples of concept groups to investigate:

- CPE identity and firmware metadata
- Wi-Fi radio state
- Wi-Fi AP/SSID/security settings
- Wi-Fi channel control
- Wi-Fi scan/ACS/pre-CAC results
- Wi-Fi client association and host identity
- Wi-Fi RSSI/PHY/error/traffic metrics
- Wi-Fi noise and utilization metrics
- WAN access type and link properties
- WAN speedtest/download/upload/latency metrics
- DSL/Ethernet/GPON-derived WAN metrics where actually implemented
- QoS classification and WMM controls
- Airtime fairness controls
- RTS/CTS/retry controls
- topology/host map concepts
- Discovery Program capability/property checks

## Deliverable 2: Feature Requirements Draft

File:

- `docs/offline-analyzer/feature-requirements-draft.md`

The document must map each confirmed feature from the inventory to canonical concept requirements.

Each feature requirement entry must include:

- `featureId`
- feature name
- category
- importance score copied from inventory
- mandatory concept candidates
- optional/degrading concept candidates
- control/write concepts if applicable
- diagnostic/action concepts if applicable
- required granularity if known: device, radio, SSID, AP, host/client, WAN interface, mesh node
- history/window/frequency needs if visible in source
- fallback behavior if visible in source
- evidence references proving the requirement
- open questions

This is a draft. It may say `unknown` where the source code does not provide enough evidence.

## Evidence Rules

Every concept and feature requirement must cite source evidence.

Evidence format:

- repository
- relative file path
- line number or line range where practical
- symbol/function/class/config key if available
- short explanation of what the evidence proves

Use UI evidence only for naming and user-visible exposure. Requirement evidence should come from backend/TSS logic where possible.

## Milestones

Status:

- Milestone 1 complete: Discovery Program concepts and draft requirements are documented.
- Milestone 2 complete: QoE score concepts and draft requirements are documented.
- Milestone 3 complete: Self-healing telemetry, control/action, and runtime-only concepts are documented.
- Milestone 4 complete: Customer-care Wi-Fi settings, topology map, speedtest, and score drill-down concepts are documented.
- Next unlocked milestone: Milestone 5, Review and Gate.

### Milestone 1: Concept Extraction From Discovery Program

Scope:

- Analyze Discovery Program capability and feature rule definitions.
- Extract concepts around data model availability, parameter value retrieval, mandatory/recommended/optional requirements, and supported/partial/unsupported status.
- Identify whether existing TR-181/TR-098 rule files already encode reusable concepts or raw-path checks.

Files:

- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

Commands:

- `rg -n "Capability|Requirement|MANDATORY|RECOMMENDED|OPTIONAL|DataModelHas|get_parameter_value|is_parameter_available" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/discovery-program`
- `rg -n "TR181|TR-181|TR098|TR-098|Device\\.|InternetGatewayDevice\\." /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/discovery-program`

Acceptance criteria:

- Discovery concepts are documented.
- Discovery feature requirements are drafted without final raw path mapping.
- Evidence is cited for every Discovery concept and requirement.

### Milestone 2: Concept Extraction From Score Calculations

Scope:

- Analyze QoE score calculation code and related KPI/aggregation scripts.
- Extract concepts for CPE score, Wi-Fi score, host Wi-Fi score, Internet score, WAN access score, and their derived inputs.
- Record history/window/fallback behavior visible in source.

Files:

- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

Commands:

- `rg -n "qoe_.*_score|calculate.*Score|DEFAULT_METRIC_HISTORY_WINDOW|getMemoKeyValue|median|history|last\\.value" /home/mtorbus/hdd/gitrep/prisme/tss/services/report-parser/src/main/resources/poc/javascript`
- `rg -n "noise|utilization|rssi|phy|error|download|upload|latency|wan|dsl|gpon|ethernet" /home/mtorbus/hdd/gitrep/prisme/tss/services/report-parser/src/main/resources/poc/javascript`

Acceptance criteria:

- Score concepts are documented as derived metric concepts, not assumed raw parameters.
- Feature requirement draft identifies mandatory versus optional only where source code supports that distinction.
- TBD/stub score branches remain explicitly marked as incomplete.

### Milestone 3: Concept Extraction From Self-Healing

Scope:

- Analyze Remote Channel Management and AQoS workflows.
- Extract telemetry concepts, control/write concepts, diagnostic/action concepts, and runtime-only limitations.
- Separate observe/recommend/write behavior where code supports the distinction.

Files:

- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

Commands:

- `rg -n "GetRadios|GetCurrentChannel|SetChannel|ScanWifi|ScanWifiAcs|PreCac|Allowed|GetBestChannel|Normalize|SetAtf|PrioritizeTraffic|CollisionTuning|RTSThreshold|WMMEnable|QoS\\.Classification" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/self-healing`
- `rg -n "host traffic|coverage|score|associated|MAC|SetString|client\\.Get|client\\.Set|client\\.Add" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/self-healing`

Acceptance criteria:

- Self-healing feature requirements distinguish read telemetry from write/control capability.
- Runtime-only validation limits are recorded.
- Evidence references point to workflow decision logic and activity action logic.

### Milestone 4: Concept Extraction From Customer-Care and Diagnostics

Scope:

- Analyze customer-care Wi-Fi settings, topology map, speedtest, and score drill-down.
- Extract concepts for management actions, topology visibility, diagnostics, and score consumption.
- Distinguish direct data model dependency from derived OpenSearch/API dependency.

Files:

- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

Commands:

- `rg -n "GetWifiSettings|SetWifiSettings|Device\\.WiFi|Speedtest|Diagnostic|GetCpeMap|mapBulk|GetHostScore|GetCpeScore" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/customer-care-agent`
- `rg -n "wifi-suite|port-mapping|mapFlat|hosts/scores|command/speedtest" /home/mtorbus/hdd/gitrep/prisme/prisme-ui/apps/customer-care-dashboard /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/customer-care-agent`

Acceptance criteria:

- Customer-care concepts are documented with direct/indirect dependency classification.
- Diagnostics concepts are separated from score concepts.
- Topology/map concepts are not overclaimed as raw data model requirements unless source confirms the path.

### Milestone 5: Review and Gate

Scope:

- Review concept and requirement documents for consistency.
- Ensure all inventory features are represented or explicitly deferred.
- Ensure no final raw-path mapping table has been introduced prematurely.

Files:

- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

Commands:

- `rg -n "TODO|TBD|unknown|deferred|open question|unsupported|implementation_not_found" docs/offline-analyzer/capability-concepts.md docs/offline-analyzer/feature-requirements-draft.md`
- `rg -n "Device\\.|InternetGatewayDevice\\." docs/offline-analyzer/capability-concepts.md docs/offline-analyzer/feature-requirements-draft.md`
- `git diff -- docs/offline-analyzer/capability-concepts.md docs/offline-analyzer/feature-requirements-draft.md`

Acceptance criteria:

- Every active feature from `feature-inventory.md` is covered or explicitly deferred with a reason.
- Every concept has evidence.
- Requirements distinguish direct data model, derived metric, control/action, and runtime-only needs.
- The docs do not claim final snapshot support status.
- The docs do not contain broad unsupported claims without source evidence.

## Gate Checklist

Before committing this phase:

- Verify both deliverable files exist.
- Verify every concept has at least one evidence reference.
- Verify every feature requirement has at least one evidence reference.
- Verify no final raw-path mapping table is introduced.
- Run `make test`; if unavailable, document that this repository still has no implementation test target.
- Run the milestone-specific `rg` and `git diff` checks.

## Next Command

Run `planlock` to lock Milestone 5, or run `autostep` if Milestone 5 is already approved.
