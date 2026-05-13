# Offline Analyzer Implementation Plan

## Objective

Implement the first working Offline PRISME CPE Compatibility Analyzer after the feature inventory, capability concepts, requirement draft, and input-to-concept mapping documents have been reviewed.

This plan turns the documentation artifacts into an executable analyzer while keeping the first implementation small, deterministic, and reviewable.

## Source Documents

Use these documents as the implementation source of truth:

- `AGENTS.md`
- `docs/offline-analyzer/input-json-format.md`
- `docs/offline-analyzer/capability-concepts.md`
- `docs/offline-analyzer/concept-parameter-mapping.md`
- `docs/offline-analyzer/feature-requirements-draft.md`
- `docs/offline-analyzer/feature-parameter-dependencies.md`
- `docs/offline-analyzer/feature-inventory.md`

The implementation must not redo broad source discovery unless a documented mapping is ambiguous or inconsistent.

## Implementation Principles

- Implement in this repository unless a later human decision explicitly moves the engine into a PRISME host repository.
- Keep the first implementation independent from live PRISME services.
- Do not call PRISME runtime APIs, databases, brokers, ACS, USP controllers, CPEs, or diagnostics services.
- Support only the documented JSON input format with top-level `Report` evidence.
- Produce structured plain text as the primary output.
- ANSI colors are allowed only for terminal output and must be controlled by `--color=always|auto|never`.
- Keep machine-readable internal structures deterministic even if the primary output is text.
- Sort feature assessments, requirements, matched paths, missing requirements, and evidence by stable IDs or paths.
- Do not claim full runtime support from static evidence.

## Proposed Technology Choice

Use a small Rust CLI in this repository unless an implementation blocker appears.

Rationale:

- The sibling project workflow already uses Rust/Cargo successfully.
- Rust gives a single portable CLI binary, deterministic data structures, and straightforward tests.
- The project currently has no runtime stack, so adding a minimal Cargo project is acceptable if kept dependency-light.

Initial dependencies should be limited to:

- `serde` and `serde_json` for JSON parsing.
- `clap` for CLI parsing, only if approved by the implementation step or already acceptable for this repository.
- Avoid templating, database, HTTP, async runtime, or framework dependencies in Phase 1.

If avoiding `clap`, implement a small manual argument parser for `--input`, `--output`, and `--color`.

## Phase 1 Scope Cap

Implement no more than 8 feature assessments in the first engine PR.

Initial feature set:

1. `discovery.capabilityScan`
2. `discovery.platformFeatureScan`
3. `score.cpe.wifi`
4. `score.host.wifi`
5. `score.cpe.internet`
6. `selfHealing.remoteChannelManagement`
7. `customerCare.wifiSettings`
8. `diagnostics.speedtest`

Reasoning:

- Covers Discovery, scores, self-healing, customer-care configuration, and diagnostics/runtime-only behavior.
- Exercises read, write, diagnostic, derived, history, partial, unsupported, and runtime-only classifications.
- Avoids implementing every mapped feature before the engine/report contract is proven.

## Phase 1 Deliverables

Files and directories:

- `Cargo.toml`
- `src/main.rs`
- `src/input.rs`
- `src/model.rs`
- `src/registry.rs`
- `src/evaluator.rs`
- `src/report.rs`
- `tests/fixtures/offline-analyzer/`
- `docs/offline-analyzer/examples/input-basic.json`
- `docs/offline-analyzer/examples/output-compatibility-report.txt`
- `.gitignore` entry for `/out/`

CLI behavior:

```bash
cpe-analyzer --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt
```

Required CLI options:

- `--input <path>`
- `--output <path>`
- `--color=auto|always|never`, default `auto`

Optional CLI options after the first working version:

- `--no-color` as alias for `--color=never`
- `--fail-on unsupported|unknown|implementation_not_found`

## Input Model

Parse the supported JSON input shape from `docs/offline-analyzer/input-json-format.md`.

Minimum accepted evidence fields:

- top-level `Report` array
- parameter path
- value when present
- timestamp when present
- type metadata when present
- readable metadata when present
- writable metadata when present
- diagnostic/action metadata when present

The parser should normalize entries into an internal `Snapshot` model:

- `device` metadata if available
- `parametersByPath`
- `objectsByPath`
- `historiesByMetric`
- `actionsByIdOrPath`
- `repositoryMetadata` only if supplied or discoverable locally without network

Missing optional input sections must not fail parsing.

Malformed JSON or unsupported input shape must produce a clear error.

## Registry Model

Represent concepts, requirements, and features in code as static registry data.

Core types:

- `ConceptId`
- `FeatureId`
- `RuleId`
- `AccessKind`: `read`, `write`, `diagnostic`, `command`, `derived`, `runtime_only`
- `ValueRequirement`: `path_only`, `value_required`, `metadata_required`, `history_required`, `runtime_only`
- `SupportStatus`: `supported`, `partial`, `unsupported`, `unknown`, `implementation_not_found`
- `EvidenceConfidence`: `high`, `medium`, `low`

Registry entries must include source-code evidence copied from the mapped docs.

Phase 1 registry should be coded directly in Rust constants or simple constructors. Do not introduce a custom DSL or external config format yet.

## Evaluation Rules

Evaluate each feature by matching mandatory, optional, control, diagnostic, derived, and runtime-only requirements against the normalized snapshot.

Rules:

- A read/path-only requirement passes when at least one candidate path exists.
- A value-required requirement passes only when a candidate path exists with a non-empty value.
- A metadata-required write requirement passes only when the candidate path exists and writable metadata is true, or the object/action metadata explicitly supports add/write where required.
- A history-required requirement passes only when the input includes history evidence for the metric or concept.
- Runtime-only requirements never make a feature fully runtime-supported; they must add a limitation and usually force `partial` unless the feature itself is classified as runtime-only.
- Diagnostic/action requirements pass only when explicit action/diagnostic capability evidence exists; otherwise report missing action capability or runtime-only limitation according to the source mapping.
- If all mandatory static requirements pass and unresolved runtime-only requirements remain, prefer `partial` over `supported` unless the feature is explicitly only a static read/display feature.
- If a known mandatory static requirement is missing, return `unsupported` and include a `Missing for support` block.
- If source mapping says `implementation_not_found`, do not convert it to unsupported.

## Text Report Contract

Primary output is plain text with stable sections.

Required header:

```text
PRISME Offline CPE Compatibility Report
reportFormatVersion: 1
assessmentBasis: staticDataModelSnapshot
```

Required sections:

- Device
- Summary
- Feature Matrix
- Feature Details
- Runtime Validation Limitations
- Source Evidence

Each feature detail must include:

- feature ID
- feature name
- category
- support status
- evidence confidence
- matched requirements
- missing mandatory requirements
- missing optional requirements
- missing write/control capability
- diagnostics/actions required
- runtime validation limitations
- source-code references

Each unsupported feature must include:

```text
Missing for support:
- <ruleId>: <conceptId> requires <access/value/history>; missing <specific path/metadata/history>. Impact: <why this blocks support>.
```

Color policy:

- Color status labels only.
- Never color file paths or evidence IDs.
- `--color=never` must emit no ANSI escape codes.
- Output file should default to no color unless `--color=always` is explicitly requested.

## Tests

Add unit and integration tests for:

- JSON parsing of top-level `Report` entries.
- deterministic ordering of features and requirements.
- supported read requirement.
- unsupported missing mandatory read requirement.
- partial due to missing optional/history/runtime-only evidence.
- unsupported due to missing writability/control metadata.
- runtime-only diagnostic feature behavior.
- ANSI color disabled output has no escape sequences.
- example command generates a report containing `reportFormatVersion` and `Missing for support` for unsupported features.

Fixtures:

- `tests/fixtures/offline-analyzer/basic-supported.json`
- `tests/fixtures/offline-analyzer/missing-mandatory.json`
- `tests/fixtures/offline-analyzer/read-only-control.json`
- `tests/fixtures/offline-analyzer/runtime-only.json`
- `tests/fixtures/offline-analyzer/malformed.json`

## Phase 1 Gate

Before committing Phase 1 implementation:

- `cargo fmt --check`
- `cargo test`
- `make test`
- sample analyzer command writes `out/compatibility-report.txt`
- `rg -n "reportFormatVersion|Missing for support|supported|partial|unsupported|unknown|implementation_not_found" out/compatibility-report.txt`
- `git diff --check`

If any command is unavailable, document the skipped command and reason in the final response.

## Phase 2 Scope

After Phase 1 is reviewed:

- Add remaining mapped features from `feature-parameter-dependencies.md`.
- Add richer score-history matching.
- Add TR-098/lossy mapping only where source-backed.
- Add optional JSON secondary output only if useful.
- Improve example reports and documentation.

## Phase 1 Implementation Decisions

- Rust CLI is approved for this repository.
- `clap` is approved for command-line argument parsing.
- Output files should default to no color unless `--color=always` is explicitly requested.

These decisions unblock Phase 1 implementation.

## Phase 2 Locked Plan - Registry Accuracy and Status Coverage

### Scope

Replace the provisional Phase 1 registry with a tighter source-backed registry derived from the existing dependency documents, without broad new PRISME source discovery.

This phase must not expand to every PRISME feature. It focuses on making the current eight Phase 1 features more defensible and testable:

1. `discovery.capabilityScan`
2. `discovery.platformFeatureScan`
3. `score.cpe.wifi`
4. `score.host.wifi`
5. `score.cpe.internet`
6. `selfHealing.remoteChannelManagement`
7. `customerCare.wifiSettings`
8. `diagnostics.speedtest`

The phase should improve these areas:

- replace placeholder evidence references with specific references from the prepared docs,
- normalize registry data so source evidence is attached per feature and per requirement where practical,
- add explicit tests for `supported`, `partial`, `unsupported`, `unknown`, and `implementation_not_found`,
- add deterministic ordering tests for features, requirements, missing requirements, and matched paths,
- add TR-181 and TR-098 fixture coverage where mappings are already documented,
- keep output text as the primary report format.

### Files

Expected files to modify:

- `src/model.rs`
- `src/registry.rs`
- `src/evaluator.rs`
- `src/report.rs`
- `tests/cli.rs`
- `tests/fixtures/offline-analyzer/*.json`
- `docs/offline-analyzer/examples/input-basic.json`
- `docs/offline-analyzer/examples/output-compatibility-report.txt`
- `docs/implementation-plan.md`

Optional files if the implementation benefits from separation:

- `src/matcher.rs`
- `src/status.rs`
- `tests/registry.rs`

Do not modify unrelated docs or PRISME source repositories in this phase.

### Tests

Add or update tests for:

- feature order is stable by `featureId`,
- matched paths are stable and sorted,
- missing requirements are stable and sorted by `ruleId`,
- `supported` status for a static read/display feature where all mandatory evidence is present and no runtime-only blocker exists,
- `partial` status caused by runtime validation limitations or missing optional evidence,
- `unsupported` status caused by missing mandatory evidence,
- `unsupported` status caused by missing required write/control capability,
- `unknown` status for a deliberately registered feature with insufficient source/snapshot evidence,
- `implementation_not_found` status for a deliberately registered feature reference with no implementation dependency evidence,
- TR-181 fixture behavior,
- TR-098 fixture behavior only where current mapping docs justify it,
- no ANSI escape sequences when `--color=never`,
- ANSI status colors only when `--color=always`.

If `unknown` and `implementation_not_found` require test-only registry entries, keep them isolated from normal Phase 2 output or clearly mark them as test fixtures.

### Commands

Run the full gate before commit:

```bash
cargo fmt --check
cargo test
make test
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --color=never
rg -n "reportFormatVersion|Missing for support|supported|partial|unsupported|unknown|implementation_not_found" out/compatibility-report.txt
git diff --check
```

If a command fails because of local toolchain or dependency constraints, fix the implementation if possible. If it cannot be fixed locally, record the exact failing command and failure reason.

### Acceptance Criteria

Phase 2 is complete when:

- the current eight features still appear in the text report,
- each feature has more specific evidence than the Phase 1 common placeholder evidence,
- status evaluation covers all allowed support enum values in tests,
- unsupported features include `Missing for support`,
- partial features explain degradation or runtime validation limitation,
- output remains deterministic across repeated runs for the same input,
- `out/` remains uncommitted,
- all gate commands pass or any blocked command is explicitly documented.

### Next Command

Run `autostep` to execute Phase 2.

## Phase 3 Autostep - Additional Mapped Feature Rows

### Scope

This autostep broadens the default registry with a small set of source-mapped features from `docs/offline-analyzer/feature-parameter-dependencies.md` without adding AQoS workflows yet.

Added features:

- `score.cpe.overall`
- `customerCare.scoreDrilldown`
- `customerCare.topologyMap`
- `noc.populationScores`

### Acceptance Criteria

- The new features appear in the default report.
- Each new feature has source-code evidence references.
- Score-consumption fixtures exercise these features without requiring live services.
- Existing Phase 2 status and determinism tests continue to pass.
- The full gate from Phase 2 continues to pass.

### Next Command

Run `planlock` before adding AQoS/self-healing workflow expansion, because those features introduce more control/write semantics and should be scoped separately.

## Phase 4 Locked Plan - AQoS Self-Healing Workflow Expansion

### Scope

Add the remaining source-mapped AQoS self-healing workflow features from `docs/offline-analyzer/feature-parameter-dependencies.md` to the default analyzer registry.

Implement exactly these feature IDs in this phase:

1. `selfHealing.aqosDynamicPrioritization`
2. `selfHealing.aqosAirtimeFairnessTuning`
3. `selfHealing.aqosRtsCtsThresholdTuning`

Do not add new broad feature families in this phase.
Do not inspect or modify PRISME source repositories.
Use the existing prepared docs as source of truth.

### Feature Semantics To Preserve

- All three AQoS features are automatic self-healing workflows and must keep `runtimeValidationRequired: true`.
- Static JSON may prove data model/control capability, score/history evidence, and associated-device evidence.
- Static JSON must not claim that runtime workflow execution, cooldown behavior, score freshness, OpenSearch query success, or firmware write acceptance is proven.
- Missing required control/write capability must make the automatic workflow `unsupported`.
- Missing history/telemetry should produce `unsupported` when mandatory, or `partial` when optional/degrading.

### Files

Expected files to modify:

- `src/registry.rs`
- `tests/status_coverage.rs`
- `tests/fixtures/offline-analyzer/*.json`
- `docs/offline-analyzer/examples/output-compatibility-report.txt`
- `docs/implementation-plan.md`

Optional only if needed:

- `src/evaluator.rs`
- `src/model.rs`

Do not change CLI behavior or output format in this phase.

### Registry Requirements

`selfHealing.aqosDynamicPrioritization` must include:

- mandatory associated device evidence: `selfHealing.qos.associatedDevices`,
- mandatory score telemetry evidence: `selfHealing.qos.scoreTelemetry`,
- optional host traffic evidence: `selfHealing.qos.hostTrafficTelemetry`,
- required control evidence: `selfHealing.qos.priorityControl`,
- source references from `device_prioritization.go` and `activities/prioritize.go`.

`selfHealing.aqosAirtimeFairnessTuning` must include:

- mandatory associated device evidence: `selfHealing.qos.associatedDevices`,
- mandatory host traffic evidence: `selfHealing.qos.hostTrafficTelemetry`,
- optional prioritization state/control evidence: `selfHealing.qos.priorityControl`,
- required control evidence: `selfHealing.qos.atfControl`,
- source references from `airtime_fairness.go` and `activities/atf.go`.

`selfHealing.aqosRtsCtsThresholdTuning` must include:

- mandatory associated device evidence: `selfHealing.qos.associatedDevices`,
- mandatory score telemetry evidence: `selfHealing.qos.scoreTelemetry`,
- mandatory host traffic/packet evidence: `selfHealing.qos.hostTrafficTelemetry`,
- mandatory collision metric evidence: `selfHealing.qos.collisionMetrics`,
- required control evidence: `selfHealing.qos.rtsCtsControl`,
- source references from `collision_tuning.go` and `activities/collision.go`.

### Tests

Add or update tests for:

- all three AQoS features appear in the default report,
- a fixture with associated devices, score histories, host traffic, and write/control metadata makes AQoS features `partial` because runtime validation remains required,
- a fixture with telemetry but read-only/missing control metadata makes automatic AQoS workflows `unsupported`,
- missing mandatory telemetry remains `unsupported`,
- source references for AQoS features are source-code paths with line numbers,
- deterministic feature ordering still holds.

Recommended new fixtures:

- `tests/fixtures/offline-analyzer/aqos-control-supported.json`
- `tests/fixtures/offline-analyzer/aqos-read-only-control.json`

### Commands

Run the full gate before commit:

```bash
cargo fmt --check
cargo test
make test
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --color=never
rg -n "reportFormatVersion|Missing for support|supported|partial|unsupported|unknown|implementation_not_found" out/compatibility-report.txt
git diff --check
```

### Acceptance Criteria

Phase 4 is complete when:

- the three AQoS feature IDs appear in the default text report,
- each AQoS feature includes source-code evidence paths and line numbers,
- each AQoS feature distinguishes telemetry/read support from required control/write support,
- automatic AQoS features are not reported as fully runtime-supported from static JSON alone,
- tests cover partial runtime-limited AQoS support and unsupported missing-control AQoS support,
- all gate commands pass.

### Next Command

Run `autostep` to execute Phase 4.

## Phase 5 Autostep - Runtime Rediscovery Coverage

### Scope

Add `discovery.rediscovery` to the default registry as a runtime-only workflow that depends on the underlying static Discovery capability and platform-feature scan evidence.

### Acceptance Criteria

- `discovery.rediscovery` appears in the default text report.
- Static discovery evidence can make it `partial`, never fully runtime-supported.
- Missing underlying discovery evidence makes it `unsupported`.
- Source evidence points to the rediscovery API/hook call paths with line numbers.
- Full gate continues to pass.

### Next Command

Run `planlock` before the next expansion or hardening step.
