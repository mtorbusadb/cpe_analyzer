# Offline Analyzer Investigation Report

## Phase

Phase 0 (investigation only)

## 1. Repository inventory

### Repository root discovered

- Local PRISME parent folder: `/home/mtorbus/hdd/gitrep/prisme`
- Repositories found:
  - `prisme-backend`
  - `prisme-ui`
  - `tss`

### Technology stacks (observed from repository files)

- `prisme-backend`
  - Primary: Go (service layout under `services/`, multiple `*.go` packages)
  - Secondary: Python (notably `services/discovery-program/src/*.py`)
- `prisme-ui`
  - Primary: TypeScript/React (monorepo apps, `package.json` in repo root)
- `tss`
  - Primary: Java/Maven modules (`libs/pom.xml`, service modules)
  - Secondary: JavaScript scoring scripts in report parser resources

### Host repository decision

- Selected host repository: `prisme-backend`
- Rationale:
  - This is the default required by `AGENTS.md`.
  - Discovery program and self-healing executable paths are in `prisme-backend/services`.
  - Offline analyzer should integrate with existing discovery/data-model capability logic rather than creating a parallel implementation in UI/TSS.

### Relevant modules/packages/APIs/tests/fixtures discovered

- Discovery/data model/capabilities:
  - `prisme-backend/services/discovery-program/src/device_processor.py`
  - `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py`
  - `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py`
  - `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py`
  - `prisme-backend/services/discovery-program/src/rules/tr181_caps.py`
  - `prisme-backend/services/discovery-program/src/rules/tr098_caps.py`
  - `prisme-backend/services/discovery-program/src/rules/tr181_features.py`
  - `prisme-backend/services/discovery-program/src/rules/tr098_features.py`
- Scores:
  - `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js`
- Self-healing:
  - `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go`
  - `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go`
  - `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go`
  - `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go`
  - `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go`
- API/DTO exposure:
  - `prisme-backend/services/customer-care-agent/src/rest/handler.go`
  - `prisme-ui/apps/discovery-dashboard/src/api/types/common.types.ts`
  - `prisme-ui/apps/discovery-dashboard/src/api/types/firmware.types.ts`
- Tests/fixtures candidates:
  - `prisme-backend/services/self-healing/remote-channel-management/test/`
  - `prisme-backend/services/self-healing/quality-of-service/test/`
  - `prisme-backend/services/self-healing/test/`
  - (No direct `discovery-program` test directory found in top-level scan output; requires follow-up in host repo during Phase 1)

## 2. Existing Discovery Program analysis

### Location and executable entry path

- `DeviceProcessor` orchestrates discovery flow and selects TR-181 vs TR-098 rulesets:
  - `prisme-backend/services/discovery-program/src/device_processor.py:104-150`
- For capabilities:
  - chooses `rules.CAPS_TR181_ALL` vs `rules.CAPS_TR098_ALL` based on firmware data model (`TR_181` vs other):
  - `prisme-backend/services/discovery-program/src/device_processor.py:111-115`
- For platform features:
  - chooses `rules.FEATURES_TR181_ALL` vs `rules.FEATURES_TR098_ALL`:
  - `prisme-backend/services/discovery-program/src/device_processor.py:139-143`

### Inputs

- Firmware metadata (includes data model classification) via storage.
- Device endpoint ID and communication interface (TR-069/TR-369/direct).
- Rule suites and rule contexts from discovery rules modules.

### Outputs

- Capability suite/module/property status with statistics and score persisted through storage.
- Feature rule evaluation outputs persisted through storage as evaluated/default status.

### What it already checks

- Mandatory/recommended/optional support semantics for capabilities:
  - `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:76-108`
- Capability aggregate score calculation:
  - `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:248-257`
- Feature-rule primitives include value retrieval, indexed object lookup, data-model availability check, and value comparisons:
  - `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:111-180`
  - `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:244-279`

### Gap vs offline compatibility analyzer goal

- Discovery program is online-oriented (communicates with CPE endpoints through comm interfaces).
- It does not directly evaluate a static snapshot JSON with top-level `Report` entries.
- It does not emit the required offline report schema with:
  - `featureId`, `conceptId`, `ruleId`
  - explicit `supported|partial|unsupported|unknown|implementation_not_found`
  - per-feature `Missing for support` blocks
  - runtime validation limitation blocks

## 3. Data model handling analysis

### TR-181 handling

- TR-181 capability and feature rules are explicitly defined:
  - `prisme-backend/services/discovery-program/src/rules/tr181_caps.py`
  - `prisme-backend/services/discovery-program/src/rules/tr181_features.py`

### TR-098 handling

- TR-098 capability and feature rules are explicitly defined:
  - `prisme-backend/services/discovery-program/src/rules/tr098_caps.py`
  - `prisme-backend/services/discovery-program/src/rules/tr098_features.py`
- Example of explicit TR-098 path expectations (`InternetGatewayDevice.*`):
  - `prisme-backend/services/discovery-program/src/rules/tr098_caps.py:20-230` (sample sections)

### Vendor-specific parameter handling

- ADB/vendor extension parameters are present in feature rules (example: `X_ADB_*` fields) and score/processing code.
- Example references:
  - `prisme-backend/services/discovery-program/src/rules/tr181_features.py` (e.g., `X_ADB_RateSupported`)
  - `tss/services/device-service/doc/readme-tr-181-parameters.md` (vendor extension examples)

### Normalization/mapping layers

- Rule processor provides logical normalization primitives over raw DM paths:
  - `GET`, `GET_BY_IDX`, `GET_IDX`, `DATAMODEL_HAS`, `CHECK_VAL`, `COUNT_IF`.
  - `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:111-280`
- Current normalization is rule-oriented, not a canonical concept registry.

### Internal representation of parameter metadata

- Discovery feature engine mostly works with:
  - parameter availability (`is_parameter_available`),
  - parameter value (`get_parameter_value`),
  - and index/object traversal patterns.
- Explicit per-parameter readability/writability/type metadata is not represented as first-class offline model fields in the discovery code paths inspected; this is a required extension for the offline analyzer model.

## 4. PRISME feature inventory from source code

This report reuses existing inventory evidence and confirms active feature groups for Phase 1 cap:

- Discovery:
  - `discovery.capabilityScan`
  - `discovery.platformFeatureScan`
- Scores:
  - `score.cpe.wifi`
  - `score.host.wifi`
  - `score.cpe.internet`
- Self-healing:
  - `selfHealing.remoteChannelManagement`
  - `selfHealing.aqosDynamicPrioritization`
  - `selfHealing.aqosAirtimeFairnessTuning`
  - `selfHealing.aqosRtsCtsThresholdTuning`
- Customer-care control/diagnostics exposure:
  - Wi-Fi settings, speedtest endpoints, self-healing start/stop/config/status:
  - `prisme-backend/services/customer-care-agent/src/rest/handler.go:32-69`

Feature availability/status exposure in UI/API models:

- Capability support states in UI types:
  - `prisme-ui/apps/discovery-dashboard/src/api/types/common.types.ts:10`
- Firmware capability scan summary and self-healing supported flags:
  - `prisme-ui/apps/discovery-dashboard/src/api/types/firmware.types.ts:24-53`

## 5. Score calculation dependency analysis

### Score calculation location

- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js`

### Score types observed in executable code

- Device/cpe-level:
  - `qoe_cpe_score`
  - `qoe_cpe_wifi_score` and Wi-Fi subscores
  - `qoe_cpe_internet_score` and internet/wan-access subscores
- Host-level:
  - `qoe_host_score`
  - `qoe_host_wifi_score` and host Wi-Fi subscores
- Evidence:
  - output pushes at `.../qoe_scores_calculation.js:834-860`

### Inputs, derived metrics, windows, thresholds, fallbacks

- History roots and weighted derivations:
  - `metricsHistoryRoot` / `scoresHistoryRoot`: `:157-158`
- Wi-Fi derived path:
  - host rollups -> cpe Wi-Fi -> cpe score: `:162-188`, `:825-829`
- Noise/utilization history-based scoring:
  - `:544-600`
- Internet diagnostics scoring (latency/download/upload) uses history + last-value weighted means:
  - `:642-716`
- WAN access scoring branches by access type with partial/TBD branches (ETH/DSL/GPON/L2TP):
  - `:718-820`
- Median freshness window checks:
  - `isMedianValid`: `:608-614`

### Mandatory vs optional inputs (source-observed behavior)

- Many score branches short-circuit to `0`/N-A behavior when expected history/last values are missing (not hard fail), indicating partial/fallback tolerance.
- Some branches are explicitly TBD (`calcXDSLScore`, `calcGPONScore`, `calcL2TPScore`) and should not be assumed fully implemented.

## 6. Self-healing dependency analysis

### Remote Channel Management (RCM)

- Workflow reads radio inventory and sets channel decisions:
  - read radios: `.../remote-channel-management/src/workflow.go:491-506`
  - choose best channel from scan-derived normalized metrics: `:554-562`
- Required read/control params/actions (from activities):
  - read current channel: `.../activities/channel.go:30-37`
  - write `AutoChannelEnable=false` and `Channel=<n>`: `.../activities/channel.go:59-76`
- Diagnostics/actions involved:
  - scans, ACS/pre-CAC inputs and channel-switch action.
- Execution mode:
  - algorithm can execute automatic control actions (`SetChannel`), not only recommend.

### AQoS dynamic prioritization

- Workflow reads host rolling scores and traffic, then writes prioritization:
  - host traffic score check: `.../device_prioritization.go:67-93`
  - host coverage score check: `.../device_prioritization.go:95-121`
  - candidate selection + prioritization execution: `.../device_prioritization.go:194-275`
- Execution mode:
  - algorithm executes actions (prioritization), not observe-only.

### Other QoS self-healing algorithms discovered

- Airtime fairness tuning workflow exists:
  - `.../quality-of-service/src/workflows/airtime_fairness.go`
- Collision/RTS-CTS tuning workflow exists:
  - `.../quality-of-service/src/workflows/collision_tuning.go`
- Phase 1 should include at most one self-healing algorithm path as required by scope cap.

## 7. Proposed analyzer design

### Input format

- Supported input: JSON with top-level `Report` array (TR-181-style path/value entries).
- No alternate input formats in Phase 1.

### Output format

- Primary: structured plain text report.
- Required header fields:
  - `reportFormatVersion`
  - device identification
  - detected data model
  - repository commit/version metadata (where practical)
- Include disjoint counts for `unknown` vs `implementation_not_found`.

### Canonical capability concept model

- Introduce concept IDs (deterministic) such as:
  - `cpe.identity.*`
  - `wifi.radio.*`
  - `wifi.client.*`
  - `wan.interface.*`
  - `speedtest.*`
- Map raw TR-181/TR-098/vendor paths to canonical concepts.

### Raw parameter -> concept mapping approach

- Reuse discovery rule path knowledge as seed mapping.
- Add a mapping layer that accepts snapshot path metadata and resolves to concept matches with access semantics:
  - read/value
  - write/control
  - diagnostic/action
  - history/window indicators

### Feature requirement registry

- Registry entries per feature with deterministic IDs:
  - `featureId`, `ruleId`, `conceptId`
- Each feature declares:
  - mandatory concepts
  - optional quality concepts
  - required access kind
  - runtime-only limitations
  - source evidence references

### Support evaluation rules

- Apply required enums:
  - `supported`, `partial`, `unsupported`, `unknown`, `implementation_not_found`
- Mandatory missing capability => `unsupported`.
- Runtime-only unvalidated behavior => retain limitation block and avoid over-claiming full support.

### Artifact locations

- Investigation report:
  - `docs/offline-analyzer/investigation-report.md`
- Schema doc:
  - `docs/offline-analyzer/schema.md`
- Examples:
  - `docs/offline-analyzer/examples/`
- Local generated output:
  - `out/compatibility-report.txt` (gitignored)

### CLI/API entrypoint

- Prefer host repo command pattern in `prisme-backend` once implementation starts.
- Minimal CLI should accept:
  - `--input <snapshot.json>`
  - `--output out/compatibility-report.txt`
  - `--color=always|auto|never`

### Integration point with existing Discovery Program

- Build offline evaluator as a layer reusing discovery rule semantics and rule corpora (`tr181_*`, `tr098_*`) where possible.
- Do not call live comm interfaces.
- Reuse rule-derived requirements and status semantics; adapt input source from live CPE to static snapshot.

## Bounded search workflow evidence log

Required search terms were executed (bounded, non-shallow) across major areas, including:

- `TR181`, `TR-181`, `Device.`
- `TR098`, `TR-098`, `InternetGatewayDevice.`
- `discovery`, `capability`, `capabilities`, `compliance`
- `supported`, `unsupported`, `partial`
- `score`, `scores`, `scoring`, `health`, `kpi`
- `selfHealing`, `self-healing`, `healing`, `optimization`
- `speedtest`, `diagnostic`, `download`, `upload`, `ping`
- `bulkdata`, `cwmp`, `usp`, `acs`
- `writable`, `readable`, `parameter`, `object`, `instance`

Representative command outputs inspected:

- Discovery/data model handling:
  - `device_processor.py`, `disc_prog_capability_scanner.py`, `disc_prog_features_processor.py`, `disc_prog_features_rule_processor.py`, `rules/tr181_caps.py`, `rules/tr098_caps.py`, `rules/tr181_features.py`, `rules/tr098_features.py`
- Feature inventory & DTO/API exposure:
  - `customer-care-agent/src/rest/handler.go`
  - `prisme-ui/.../api/types/common.types.ts`
  - `prisme-ui/.../api/types/firmware.types.ts`
- Score calculation:
  - `tss/.../qoe_scores_calculation.js`
- Self-healing:
  - `remote-channel-management/src/workflow.go`
  - `remote-channel-management/src/activities/channel.go`
  - `quality-of-service/src/workflows/device_prioritization.go`
  - plus additional self-healing workflow/activity directories
- Tests/fixtures:
  - self-healing test directories (`services/self-healing/**/test`)

Major-area minimum file expectation was met for discovery, scores, self-healing, DTO/API exposure, and TR-098/TR-181 mapping. Discovery-program-specific tests were not clearly surfaced in this bounded pass and are flagged for deeper targeted host-repo test discovery during Phase 1.

## Initial Phase 1 scope proposal (capped)

Proposed first implementation set (8 features, within 5-10 cap):

1. `discovery.capabilityScan`
2. `discovery.platformFeatureScan`
3. `score.cpe.wifi`
4. `score.host.wifi`
5. `score.cpe.internet`
6. `selfHealing.remoteChannelManagement`
7. `customerCare.wifiSettings`
8. `diagnostics.speedtest`

Notes:

- Include TR-181 mapping first.
- Include TR-098 mappings only where already evidenced in discovery rules.
- Mark unproven runtime behaviors explicitly with `runtimeValidationRequired`.
- Do not broaden beyond capped set in first implementation PR.
