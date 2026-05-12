# Input JSON to Capability Concept Mapping Plan

## Objective

Derive source-code-backed mappings from the supported JSON input format to canonical capability concepts and then to feature parameter dependencies.

This phase comes after:

- `docs/offline-analyzer/feature-inventory.md`
- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

The goal is to answer, with evidence:

- which input JSON fields or raw data model parameters can satisfy each canonical concept,
- which concepts are mandatory, optional, control/write, diagnostic/action, derived, or runtime-only,
- which source-code-confirmed features can be evaluated from the supported JSON input,
- which feature dependencies remain unknown, runtime-only, or implementation-not-found.

## Scope

Input:

- The supported analyzer input is JSON.
- Current feature inventory and concept/requirement documents.
- Source code under:
  - `/home/mtorbus/hdd/gitrep/prisme/prisme-backend`
  - `/home/mtorbus/hdd/gitrep/prisme/prisme-ui`
  - `/home/mtorbus/hdd/gitrep/prisme/tss`

Output:

- `docs/offline-analyzer/input-json-format.md`
- `docs/offline-analyzer/concept-parameter-mapping.md`
- `docs/offline-analyzer/feature-parameter-dependencies.md`
- updates to `docs/offline-analyzer/feature-requirements-draft.md` where mapping evidence changes requirement confidence

## Non-Goals

Do not:

- implement analyzer code,
- create final parser code,
- create final support evaluation code,
- claim support status for a real device snapshot,
- invent mappings from general TR-098/TR-181 knowledge,
- treat obsolete, unused, or unwired code as active feature support,
- add broad mappings without source-code evidence,
- expand supported input beyond JSON in this phase.

## Mapping Rules

Every mapping must include:

- `conceptId`
- input JSON location or source metric name
- raw data model path pattern where source code confirms it
- required access: read, write, diagnostic/action, command, derived, runtime-only, or unknown
- required value presence: path-only, value-required, metadata-required, history-required, or runtime-only
- granularity: device, radio, AP, SSID, host/client, WAN interface, mesh node, score document, or diagnostic request
- source-code evidence
- confidence: high, medium, or low
- limitation if static JSON cannot prove runtime behavior

Allowed mapping statuses:

- `mapped`
- `partially_mapped`
- `runtime_only`
- `derived_only`
- `unknown`
- `implementation_not_found`
- `deferred`

Use `unknown` when code is insufficient to identify a defensible mapping.

Use `implementation_not_found` only when a feature or field is referenced but no executable dependency path is found after bounded search.

## Evidence Rules

Each mapping must cite source evidence:

- repository,
- relative file path,
- line number or line range where practical,
- symbol/function/class/config key when practical,
- short explanation of what the evidence proves.

Raw data model paths may appear in `concept-parameter-mapping.md` and `feature-parameter-dependencies.md` only when they are directly present in source code or are a clearly stated source-code-derived path pattern.

Do not add raw path mappings to `feature-inventory.md`.

Do not use UI-only labels as parameter evidence. UI evidence may only prove feature exposure or naming.

## Obsolete and Unwired Code Handling

Before treating any code path as active:

- find a reachable entry point, route, worker, scheduler, command, workflow registration, or test proving usage,
- check whether the code is referenced by active services or only defined,
- document unwired candidates separately,
- do not include unwired candidates as mandatory feature requirements.

If a fallback or helper exists but has no call site, record it as `deferred` or `implementation_not_found` depending on whether a feature reference exists.

## Milestones

Status:

- Next locked milestone: Milestone 1, Supported JSON Input Shape and Evidence Model.

### Milestone 1: Supported JSON Input Shape and Evidence Model

Scope:

- Define the supported JSON input shape at documentation level.
- Identify how JSON can represent raw data model paths, values, types, readability, writability, objects, instances, diagnostic/action availability, history, and derived score documents.
- Define mapping table columns and evidence format.
- Do not map all feature parameters yet.

Files:

- `docs/offline-analyzer/input-json-format.md`
- `docs/offline-analyzer/concept-parameter-mapping.md`
- `docs/offline-analyzer/feature-parameter-dependencies.md`
- `docs/input-json-concept-mapping-plan.md`

Commands:

- `rg -n "parameters|path|value|type|readable|writable|diagnostic|history|score|timestamp|metadata" docs AGENTS.md`
- `rg -n "DataModelObject|Parameter|readable|writable|GetParameter|SetParameter|GetSupported|diagnostic|Diagnostics" /home/mtorbus/hdd/gitrep/prisme/prisme-backend /home/mtorbus/hdd/gitrep/prisme/tss`
- `rg -n "qoe_.*score|qoe_.*Score|history|timestamp|last|median|bucket" /home/mtorbus/hdd/gitrep/prisme/tss/services/report-parser/src/main/resources/poc/javascript`

Acceptance criteria:

- `input-json-format.md` defines the supported JSON input shape without adding non-JSON formats.
- `concept-parameter-mapping.md` exists with mapping status definitions and empty or starter sections.
- `feature-parameter-dependencies.md` exists with dependency table structure and empty or starter sections.
- The docs distinguish raw parameters, derived metrics, diagnostics/actions, runtime-only requirements, and score history.
- The docs state that path presence, value availability, readability, writability, and history are separate evidence dimensions.

### Milestone 2: Discovery Program Raw Parameter Mapping

Scope:

- Extract raw parameter requirements from active Discovery Program capability and feature rules.
- Preserve mandatory, recommended, optional, AND/OR, indexed instance, count, value comparison, switch, and default semantics.
- Map Discovery concepts to source-code-defined TR-181/TR-098 path patterns where available.

Files:

- `docs/offline-analyzer/concept-parameter-mapping.md`
- `docs/offline-analyzer/feature-parameter-dependencies.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

Commands:

- `rg -n "Capability\\.Requirement|MANDATORY|RECOMMENDED|OPTIONAL|PropertySet|AND|OR" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/discovery-program/src`
- `rg -n "DATAMODEL_HAS|GET_BY_IDX|GET_IDX|COUNT_IF|GET|COMPARE|SWITCH|RuleSet" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/discovery-program/src`
- `rg -n "TR181|TR098|CAPS_TR181|CAPS_TR098|FEATURES_TR181|FEATURES_TR098" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/discovery-program/src`

Acceptance criteria:

- Discovery capability and platform feature dependencies have source-code-backed raw path or rule mappings.
- Requirement type and boolean composition are preserved.
- Indexed and count-based rules document instance requirements.
- Commented/TODO/inactive rule branches are not treated as active requirements.

### Milestone 3: Score and KPI Input Mapping

Scope:

- Map QoE score concepts to source-code-confirmed metric names, KPI scripts, aggregation inputs, history windows, fallback behavior, and output fields.
- Distinguish score documents and derived metrics from raw CPE data model parameters.
- Record where static JSON can provide enough history and where runtime telemetry history is required.

Files:

- `docs/offline-analyzer/concept-parameter-mapping.md`
- `docs/offline-analyzer/feature-parameter-dependencies.md`
- `docs/offline-analyzer/feature-requirements-draft.md`

Commands:

- `rg -n "qoe_.*_score|calculate.*Score|score_|score\\(|score_min_ignore_na|DEFAULT_METRIC_HISTORY_WINDOW|getMemoKeyValue|median|last\\.value" /home/mtorbus/hdd/gitrep/prisme/tss/services/report-parser/src/main/resources/poc/javascript`
- `rg -n "qoe_ap_|qoe_wan_|download|upload|ipping|noise|utilization|rssi|phy|error|traffic|packets" /home/mtorbus/hdd/gitrep/prisme/tss/services/report-parser/src/main/resources/poc/javascript`
- `rg -n "qoe_.*score|hosts/scores|GetHostScore|GetDeviceScore|GetHostsScores" /home/mtorbus/hdd/gitrep/prisme/prisme-backend /home/mtorbus/hdd/gitrep/prisme/prisme-ui`

Acceptance criteria:

- CPE, Wi-Fi, host Wi-Fi, Internet, and WAN access score dependencies are mapped to source metric names and history requirements.
- TBD score branches remain explicitly marked incomplete.
- Customer-care score drill-down is mapped as score consumption, not raw score calculation.
- Static JSON limitations for history/window completeness are documented.

### Milestone 4: Self-Healing and Customer-Care Control Mapping

Scope:

- Map Remote Channel Management, AQoS, customer-care Wi-Fi settings, topology, and speedtest concepts to required read/write/action concepts and source-code-confirmed parameter patterns where available.
- Separate control/write dependencies from read-only telemetry dependencies.
- Separate runtime-only diagnostics/actions from static JSON evidence.
- Re-check active call paths and exclude unwired helper code.

Files:

- `docs/offline-analyzer/concept-parameter-mapping.md`
- `docs/offline-analyzer/feature-parameter-dependencies.md`
- `docs/offline-analyzer/feature-requirements-draft.md`
- `docs/offline-analyzer/feature-inventory.md` only if active/unwired classification changes.

Commands:

- `rg -n "GetRadios|GetCurrentChannel|SetChannel|ScanWifi|ScanWifiAcs|PreCac|Allowed|GetBestChannel|SetAtf|PrioritizeTraffic|CollisionTuning|RTSThreshold|WMMEnable|QoS\\.Classification" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/self-healing`
- `rg -n "GetWifiSettings|SetWifiSettings|wifi-suite|CommitConfiguration|GetCpeMap|mapBulk|Speedtest|Diagnostic|GetHostScore|GetDeviceScore" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/customer-care-agent`
- `rg -n "maybeApplyBulkFallback|MapBulkFallback|bulk fallback|StartWorkflow|Register|handler|route" /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/customer-care-agent /home/mtorbus/hdd/gitrep/prisme/prisme-backend/services/self-healing`

Acceptance criteria:

- Read telemetry, write/control, diagnostic/action, derived metric, and runtime-only dependencies are separated.
- Automatic-action features identify required write/control capability.
- Speedtest is documented as action/runtime capability, not proof of completed runtime diagnostics.
- Topology map active requirements do not include unwired fallback code unless a call path is found.

### Milestone 5: Feature Dependency Matrix and Gate

Scope:

- Produce a feature-by-feature dependency matrix suitable for later analyzer implementation.
- Verify every active feature is mapped, partially mapped, runtime-only, derived-only, unknown, implementation-not-found, or deferred with a reason.
- Verify no broad unsupported claims are made without source evidence.

Files:

- `docs/offline-analyzer/feature-parameter-dependencies.md`
- `docs/offline-analyzer/concept-parameter-mapping.md`
- `docs/offline-analyzer/feature-requirements-draft.md`
- `docs/input-json-concept-mapping-plan.md`

Commands:

- `rg -n "mapped|partially_mapped|runtime_only|derived_only|unknown|implementation_not_found|deferred" docs/offline-analyzer/concept-parameter-mapping.md docs/offline-analyzer/feature-parameter-dependencies.md`
- `rg -n "TODO|TBD|open question|unwired|obsolete|inactive|call site" docs/offline-analyzer/concept-parameter-mapping.md docs/offline-analyzer/feature-parameter-dependencies.md docs/offline-analyzer/feature-requirements-draft.md`
- `git diff -- docs/offline-analyzer/concept-parameter-mapping.md docs/offline-analyzer/feature-parameter-dependencies.md docs/offline-analyzer/feature-requirements-draft.md docs/input-json-concept-mapping-plan.md`

Acceptance criteria:

- Every active inventory feature appears in `feature-parameter-dependencies.md`.
- Every mapped concept has source-code evidence.
- Every raw path or source metric has evidence.
- Runtime-only and derived-only dependencies are explicit.
- Obsolete/unwired code is not included as active feature dependency.
- The next implementation phase can use the matrix to build a registry without redoing broad source discovery.

## Gate Checklist

Before committing each milestone:

- Verify all milestone files exist.
- Verify every new mapping row has evidence.
- Verify mapping statuses are used consistently.
- Run `make test`; if unavailable, document that this repository has no implementation test target.
- Run the milestone-specific `rg` commands.
- Run `git diff --check`.

## Next Command

Run `autostep` to execute Milestone 1.
