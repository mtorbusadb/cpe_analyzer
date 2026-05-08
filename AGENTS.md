# AGENTS.md — Offline PRISME CPE Compatibility Analyzer

## Project placement decision

The Offline PRISME CPE Compatibility Analyzer must be implemented in **one chosen host repository**. Do not spread implementation across multiple repositories unless explicitly approved by a human reviewer.

Default host repository decision:

- Host repository: `prisme-backend`
- Read-only cross-repository scope:
  - `prisme-backend`
  - `prisme-ui`
  - `tss`
- Parent workspace:
  - repositories are expected to be located under the local `prisme/` folder.

Codex may inspect all repositories under `prisme/` to discover feature requirements, APIs, DTOs, schemas, UI labels, fixtures, tests, and implementation logic.

Codex must not modify `prisme-ui` or `tss` during Phase 0 or Phase 1 unless explicitly instructed. Cross-repository edits require a clear justification and human approval.

If repository names differ locally, Codex must first list the repositories under `prisme/`, identify the likely backend/service repository, and document the final host-repository choice in the Phase 0 investigation report before implementing anything.

## Primary objective

Build an Offline PRISME CPE Compatibility Analyzer.

The analyzer must take as input a static snapshot of a CPE data model, based on TR-098, TR-181, or vendor-specific extensions, and determine how well that CPE aligns with the current PRISME implementation and feature set.

The goal is to assess PRISME feature readiness for a specific CPE without connecting the CPE to a live PRISME environment.

The analyzer must answer:

> Given only this CPE data model snapshot and the current PRISME source code, which PRISME features can this CPE support, which features are degraded, which are unsupported, and what exact missing data model capabilities prevent full support?

## Source of truth

The only PRISME-specific source of truth is the current source code in the repositories under the local `prisme/` folder.

Do not assume access to internal PRISME documentation, Confluence pages, architecture documents, product descriptions, historical notes, or other private non-source-code material.

Do not invent PRISME features or parameter requirements from general broadband knowledge.

Use general TR-098/TR-181 knowledge only to understand or map concepts that are already present in source code.

If a feature, score, algorithm, or requirement cannot be confirmed from source code, mark it according to the status semantics in this document.

## Non-goals

The analyzer is not intended to perform runtime validation.

Phase 1 and Phase 2 must not include:

- live CPE probing,
- ACS integration,
- USP controller integration,
- BulkData ingestion,
- production database access,
- production message broker access,
- emulator or simulator execution,
- automatic remediation execution,
- applying self-healing actions to real devices,
- changing runtime PRISME product behavior.

The analyzer may report that runtime validation is required, but it must not attempt to perform that validation.

## Required initial workflow

Before implementing the analyzer, produce a source-code investigation report.

Do not start broad implementation before the investigation report exists.

The report must include:

1. Repository inventory
   - list all repositories under the local `prisme/` folder,
   - identify the technology stack of each repository,
   - identify the chosen host repository,
   - identify relevant modules, packages, services, APIs, tests, fixtures, and configuration files.
2. Existing Discovery Program analysis
   - locate current discovery logic,
   - describe its inputs and outputs,
   - identify what it already checks,
   - identify gaps versus the offline compatibility analyzer goal.
3. Data model handling analysis
   - locate TR-098 handling,
   - locate TR-181 handling,
   - locate vendor-specific parameter handling,
   - identify normalization/mapping layers,
   - identify the internal representation of parameter metadata such as path, object, instance, type, value, readability, writability, and diagnostics.
4. PRISME feature inventory from source code
   - identify features/modules that depend on CPE data model capabilities,
   - identify feature flags, enums, constants, schemas, DTOs, API response models, database models, UI models, tests, and fixtures that represent feature availability or feature support.
5. Score calculation dependency analysis
   - locate all score calculation code,
   - list score types found in source code,
   - list input metrics, raw parameters, derived metrics, history windows, thresholds, and fallback behavior,
   - distinguish mandatory inputs from optional or quality-improving inputs where the code allows this distinction.
6. Self-healing dependency analysis
   - locate all self-healing algorithms,
   - list required read parameters,
   - list required write/control parameters,
   - list diagnostics/actions used,
   - identify whether each algorithm can execute actions automatically, only recommend actions, or only observe/report.
7. Proposed analyzer design
   - proposed input format,
   - proposed output format,
   - canonical capability concept model,
   - raw parameter to concept mapping approach,
   - feature requirement registry,
   - support evaluation rules,
   - artifact locations,
   - CLI/API entrypoint,
   - integration point with the existing Discovery Program.

## Bounded search workflow

Codex must perform a bounded but non-shallow source investigation before classifying a feature as `unknown` or `implementation_not_found`.

For each major area being assessed, Codex must run the required search terms listed below and inspect at least the top relevant results.

Major areas:

- Discovery/data model handling
- Feature inventory
- Score calculation
- Self-healing
- TR-098/TR-181 mapping
- Diagnostics/actions
- DTO/schema/API exposure
- Tests/fixtures

Required search terms include:

- `TR181`, `TR-181`, `Device.`
- `TR098`, `TR-098`, `InternetGatewayDevice.`
- `discovery`, `capability`, `capabilities`, `compliance`
- `supported`, `unsupported`, `partial`
- `score`, `scores`, `scoring`, `health`, `insight`, `kpi`
- `selfHealing`, `self-healing`, `healing`, `optimization`, `optimizer`
- `channel`, `bandwidth`, `interference`, `noise`, `retry`, `rssi`, `signal`, `utilization`
- `speedtest`, `speed test`, `diagnostic`, `TR143`, `download`, `upload`, `ping`
- `bulkdata`, `cwmp`, `usp`, `acs`
- `writable`, `readable`, `parameter`, `object`, `instance`

Minimum bounded search expectation before returning `unknown` for a major area:

- inspect at least 5 relevant files or directories if that many exist,
- inspect tests and fixtures if present,
- inspect configuration/schema/API definitions if present,
- record the searched terms and candidate files in the investigation report or in the feature assessment evidence.

If fewer than 5 relevant files exist, record that explicitly.

## Risk and unknown handling

Do not guess.

When implementation is unclear:

1. Run the bounded search workflow.
2. Record searched terms and candidate files.
3. Classify the result using the status semantics below.
4. Include evidence and confidence.
5. Stop there.

Use `unknown` when there is insufficient evidence to decide support from source code and snapshot data.

Use `implementation_not_found` only when a symbol, feature name, enum, flag, UI label, configuration entry, or API field exists, but no executable implementation path or dependency logic is found after bounded search.

Do not convert `implementation_not_found` into `unsupported`. It means the analyzer found a reference but could not confirm executable feature logic.

## Status semantics

Use the following decision table consistently.

Allowed support enum values: `supported`, `partial`, `unsupported`, `unknown`, `implementation_not_found`.

| Status | Meaning | Required condition |
| --- | --- | --- |
| `supported` | Static snapshot satisfies all mandatory source-code-derived requirements for this feature. | All mandatory concepts are mapped to present snapshot parameters with required access/metadata. Runtime-only behavior, if any, is explicitly marked as not validated offline. |
| `partial` | Core feature appears possible, but support is degraded or incomplete. | At least one mandatory execution path appears possible, but optional inputs, quality inputs, required granularity, history, frequency, writability, diagnostics, or fallback precision are incomplete. |
| `unsupported` | Feature requirement is known, but the snapshot lacks mandatory data/control capability. | Source code confirms the requirement, and mandatory concepts/paths/actions are missing from the snapshot. |
| `unknown` | Analyzer cannot determine support from available source-code and snapshot evidence. | Bounded search completed, but source-code dependency or snapshot interpretation remains insufficient. |
| `implementation_not_found` | Feature reference exists, but executable logic was not found. | Name/flag/enum/UI/API/config reference exists, bounded search completed, but no executable logic path or dependency logic was found. |

A feature must not be marked `supported` only because a similarly named parameter exists. It must match source-code-derived requirements.

A feature must not be marked `unsupported` when the implementation itself cannot be confirmed. Use `unknown` or `implementation_not_found`.

## Evidence requirements

Every assessed feature must include at least one source-code reference.

Preferred evidence format:

- file path,
- `lineStart` and `lineEnd` when available,
- symbol/function/class name if available,
- short explanation of what the evidence proves.

When line numbers are available, use `lineStart` and `lineEnd` consistently.

When line numbers are not available, omit `lineStart` and `lineEnd`; the fallback evidence must include:

- file path,
- symbol/function/class name if available,
- search term used,
- short explanation of why this file is relevant.

Each feature assessment must include:

- `evidenceConfidence`: `high`, `medium`, or `low`.

Confidence guidance:

- `high`: direct implementation path confirms required parameters/actions.
- `medium`: requirement is inferred from DTOs, schema, tests, configuration, or indirect call chain.
- `low`: only weak references exist, or mapping is incomplete after bounded search.

## Determinism requirement

For the same input snapshot and the same repository commit, the analyzer must produce deterministic output.

This means:

- stable `featureId`, `conceptId`, and `ruleId`,
- stable support status,
- stable report section labels and field labels,
- stable sorting of arrays,
- stable ordering of features, concepts, matched parameters, missing requirements, and evidence entries.

Sort repeated report entries (features, missing requirements, and evidence) by stable IDs or paths unless a stronger domain-specific ordering already exists.

If numeric scores, ratios, percentages, or confidence values are emitted, use a fixed documented precision and rounding mode so output remains stable.

## Output report format versioning

The top-level text report must include a single `reportFormatVersion` value in the report header.

Stable IDs are required:

- `featureId`
- `conceptId`
- `ruleId`

IDs must be deterministic and should not depend on display labels.

If the report format changes incompatibly, update `reportFormatVersion` and document the migration impact in the PR.

## Output requirements

The primary output must be structured plain text.

Optional machine-readable JSON may be produced as a secondary artifact, but plain text is required.

The report must include:

- `reportFormatVersion`
- device identification,
- detected data model,
- repository commit/version metadata where practical,
- overall compatibility summary,
- feature-by-feature support matrix,
- score-by-score support matrix where scores are discovered in source code,
- self-healing-algorithm-by-algorithm support matrix where algorithms are discovered in source code,
- missing mandatory requirements,
- missing optional requirements,
- matched parameters,
- missing write/control capability,
- diagnostics/actions required,
- runtime validation limitations,
- source-code references used to derive requirements,
- evidence confidence.

For each feature with status `unsupported`, the text report must include a dedicated `Missing for support` block that explicitly lists what is missing and why that blocks support.

Summary count fields must be disjoint. `implementationNotFoundFeatures` must not be included in `unknownFeatures`.

Terminal output may use ANSI colors while remaining plain text:

- green: `supported`
- yellow: `partial`
- red: `unsupported`
- cyan: `unknown`
- magenta: `implementation_not_found`

Color behavior should support `--color=always|auto|never` and default to `auto`.

## Artifact locations

Use the following default locations inside the chosen host repository unless existing repository conventions strongly suggest better locations.

Documentation:

- `docs/offline-analyzer/investigation-report.md`
- `docs/offline-analyzer/schema.md`
- `docs/offline-analyzer/examples/`

Committed example inputs/outputs:

- `docs/offline-analyzer/examples/input-tr181-basic.json`
- `docs/offline-analyzer/examples/output-compatibility-report.txt`

Local generated output:

- `out/compatibility-report.txt`

The `out/` directory is a local/dev artifact location and must be gitignored. Do not commit generated `out/` reports.

Fixtures:

- Prefer the repository's existing test fixture location.
- If no convention exists, use `tests/fixtures/offline-analyzer/` or equivalent for the technology stack.

Mappings and requirement registry:

- Prefer existing configuration/schema style in the chosen host repository.
- If no convention exists, place implementation-owned mapping/registry files near the analyzer module, not in `docs/`.

CLI/API entrypoint:

- Prefer an existing CLI/task/command pattern in the chosen host repository.
- If no CLI pattern exists, add the smallest local command entrypoint needed to run the analyzer against a snapshot.

## Functional requirements

The analyzer must run offline.

It must not require:

- live CPE access,
- ACS connection,
- USP controller connection,
- BulkData ingestion,
- live PRISME deployment,
- production database,
- production message broker.

The analyzer must use only:

- the provided data model snapshot,
- source-code-derived PRISME feature requirements,
- local static configuration from the repositories,
- local tests/fixtures where useful.

## Input requirements

The analyzer must accept a static CPE data model snapshot in one supported input format.

Supported input format:

- JSON
- top-level `Report` array with TR-181-style path/value pairs (for example `Device.WiFi.Radio.1.Channel`)

No alternative input formats are supported at this time.

## Normative feature assessment rules

Apply this section to all feature groups, including Discovery-related features, scores, diagnostics, and self-healing.

The analyzer must clearly distinguish:

- parameter presence,
- value availability,
- readability,
- writability,
- diagnostic command availability,
- telemetry granularity,
- telemetry history/window requirements,
- collection frequency requirements,
- requirements that cannot be validated from a static snapshot.

Do not claim full runtime support when only static data model evidence is available. If runtime behavior cannot be proven offline, mark this explicitly using `runtimeValidationRequired`.

Scores must be evaluated individually based on actual score calculation code.

Self-healing algorithms must be evaluated individually based on actual algorithm code.

Do not assume a score or algorithm exists unless found in source code.

For each discovered score or algorithm, determine:

- required telemetry inputs,
- required control/write parameters if any,
- required diagnostics/actions if any,
- derived metrics,
- thresholds,
- aggregation or decision logic,
- history/window requirements,
- fallback behavior,
- behavior when data is missing,
- whether the static snapshot is sufficient to determine support,
- whether runtime validation remains required.

## Capability model

Prefer a canonical capability concept model over direct feature-to-raw-path checks.

Examples of canonical concepts:

- `cpe.identity.vendor`
- `cpe.identity.model`
- `cpe.identity.firmwareVersion`
- `wifi.radio.channel.current`
- `wifi.radio.channel.allowedList`
- `wifi.radio.bandwidth.current`
- `wifi.radio.noise`
- `wifi.radio.utilization`
- `wifi.client.rssi`
- `wifi.client.retryCount`
- `wifi.client.phyRate`
- `wan.interface.status`
- `wan.interface.rxBytes`
- `wan.interface.txBytes`
- `speedtest.download.throughput`
- `speedtest.upload.throughput`
- `speedtest.latency`
- `cpe.cpu.usage`
- `cpe.memory.usage`
- `cpe.temperature`
- `cpe.uptime`

The exact final concept list must be derived from current source code.

Raw TR-181, TR-098, and vendor-specific parameters should map to these canonical concepts.

TR-098 to TR-181/internal mappings may be exact, partial, lossy, or unsupported. The report must expose this.

## Feature requirement registry

Represent each discovered PRISME feature as a formal requirement definition where practical.

Each requirement should identify:

- feature ID,
- feature name,
- category,
- mandatory concepts,
- optional concepts,
- required access: read, write, diagnostic, command/action,
- required granularity: device, interface, radio, SSID, AP, client, mesh node,
- required history/window,
- required collection frequency if known,
- fallback behavior if some data is missing,
- source-code references proving the requirement.

Avoid scattered hardcoded if/else logic where a registry/configuration-driven model is feasible.

## Existing Discovery Program

Find and reuse the existing Discovery Program implementation where practical.

The new analyzer should not become a parallel inconsistent system if Discovery already has reusable parsing, normalization, data model traversal, parameter metadata, or capability checks.

If existing Discovery Program logic is insufficient, extend it or build a clearly separated layer on top of it.

Document the chosen integration approach.

## Initial feature scope cap

The first implementation PR must be intentionally small.

For the first PR after the investigation report, implement no more than 5 to 10 source-code-discovered features/algorithms.

The initial feature set should prioritize:

1. Existing Discovery Program features that are already represented in code.
2. One or more score calculations discovered in code.
3. One self-healing algorithm or decision path discovered in code.
4. Basic TR-181 mapping.
5. TR-098 mapping only if existing code or clear mappings are found.

Do not attempt full PRISME feature coverage in the first implementation PR.

Broaden coverage in later PRs after schema, engine, and tests are stable.

## Implementation expectations

After the investigation report, implement the first working version focused on:

1. Reusing/parsing existing Discovery Program snapshot/data model logic where possible.
2. Building the canonical capability concept registry.
3. Building the raw parameter mapping layer.
4. Building the feature requirement registry.
5. Evaluating support status for the capped initial feature set.
6. Producing a plain-text report.
7. Adding tests.

Prefer small, reviewable commits or implementation steps.

Do not make broad unrelated refactors.

Do not change runtime product behavior unless required for the analyzer and clearly justified.

## Repository-specific constraints

Prefer the existing DTO, schema, validation, configuration, dependency injection, CLI, logging, and test style of the chosen host repository.

Do not introduce a new framework or major dependency without explicit approval.

Do not introduce a new package manager, build system, test runner, or formatting tool.

Do not perform broad formatting changes outside files directly touched by this task.

Keep public API changes minimal and documented.

## Execution contract

Before finalizing any implementation PR, Codex must run the relevant repository commands.

Required command categories:

- formatting check or formatter,
- lint/static analysis if available,
- unit tests for the analyzer,
- targeted existing tests around Discovery Program, scoring, self-healing, or data model handling,
- sample analyzer command that writes `out/compatibility-report.txt`.

Codex must discover the exact commands from the chosen host repository.

If full tests are too heavy, unavailable, or blocked:

- run the most relevant targeted test suites,
- document skipped tests,
- document why they were skipped,
- document the exact commands that were run,
- document command results.

Phase 1 minimum expected local command behavior:

- run analyzer against a committed example snapshot,
- produce `out/compatibility-report.txt`,
- validate that the text report header contains `reportFormatVersion`,
- validate deterministic stable IDs for assessed features.

## Performance guardrail

Phase 1 may prioritize correctness over performance.

From Phase 2 onward, the analyzer should handle a snapshot with at least 50,000 parameters in under 30 seconds on a typical developer machine, unless repository constraints make this unrealistic.

If the expectation is not met, document the measured performance, bottleneck, and proposed optimization.

The analyzer should avoid unnecessary network, database, or service calls.

## Testing requirements

Add or update tests for:

- TR-181 snapshot with strong support,
- TR-181 snapshot with missing optional telemetry,
- TR-181 snapshot with missing mandatory telemetry,
- TR-181 snapshot with read-only parameters where write support is required,
- TR-098 snapshot with partial/lossy mapping where mappings exist,
- vendor-specific extension path if supported by existing code,
- empty/minimal snapshot,
- deterministic output ordering,
- report format version presence,
- existing Discovery Program fixtures if available.

Run the relevant test suite before finishing.

If full test execution is too expensive or blocked, run the most relevant targeted tests and document what was not run and why.

## Definition of done by phase

### Phase 0 — investigation only

Deliverable:

- `docs/offline-analyzer/investigation-report.md`

The report must include all required investigation sections from this document.

No broad implementation is expected in Phase 0.

Phase 0 is done when:

- repositories have been inventoried,
- host repository has been selected,
- existing Discovery Program has been analyzed,
- TR-098/TR-181 handling has been analyzed,
- score dependency paths have been investigated,
- self-healing dependency paths have been investigated,
- initial feature scope for Phase 1 has been proposed,
- artifact locations and command strategy have been proposed.

### Phase 1 — minimal engine and first capped feature set

Minimum deliverables:

- analyzer module in the chosen host repository,
- input snapshot loader,
- data model detection,
- top-level text report header with `reportFormatVersion`,
- stable IDs: `featureId`, `conceptId`, `ruleId`,
- initial capped feature requirement registry,
- plain-text report generation,
- tests for the initial feature set,
- example input under `docs/offline-analyzer/examples/`,
- committed example output under `docs/offline-analyzer/examples/`,
- local generated output path `out/compatibility-report.txt`,
- documentation for how to run the analyzer.

Phase 1 is done when:

- the analyzer runs offline against an example snapshot,
- `out/compatibility-report.txt` is produced locally,
- output ordering is deterministic,
- every assessed feature has evidence,
- targeted tests pass or skipped tests are documented,
- no runtime PRISME behavior is changed unexpectedly.

### Phase 2 — broader feature coverage

Minimum deliverables:

- broadened feature registry,
- additional score coverage,
- additional self-healing coverage,
- expanded TR-098/vendor mapping where source-code evidence exists,
- performance measurement against a large snapshot,
- updated schema documentation if needed,
- updated examples and tests.

Phase 2 is done when:

- the analyzer covers a broader source-code-discovered feature set,
- unsupported/partial/unknown classifications follow the decision table,
- performance guardrail is measured and documented,
- schema changes are versioned and documented.

## PR structure guidance

Use small PRs.

Recommended sequence:

1. Investigation report only.
2. Minimal engine + schema + CLI skeleton.
3. First capped feature set + tests.
4. Broaden score/self-healing coverage.
5. Broaden mappings and examples.
6. Performance and reporting improvements.

Avoid combining investigation, large refactoring, schema design, and broad feature coverage into a single PR.

## PR checklist

Each PR must include or explicitly address:

- [ ] Scope matches the planned phase.
- [ ] Docs updated.
- [ ] Tests added or updated.
- [ ] Relevant formatter/lint/test commands run.
- [ ] Sample analyzer command run if implementation is present.
- [ ] `out/compatibility-report.json` generated locally when applicable.
- [ ] Generated `out/` artifacts not committed.
- [ ] Output schema unchanged, or schema change documented with migration note.
- [ ] Stable IDs and deterministic ordering preserved.
- [ ] Every assessed feature has source evidence.
- [ ] Skipped tests documented with reason.
- [ ] No new framework or major dependency added without approval.

## Acceptance criteria

The overall task is complete when:

1. The analyzer runs offline using only a provided CPE data model snapshot.
2. The analyzer does not require a live CPE, ACS, USP controller, BulkData ingestion, or live PRISME deployment.
3. PRISME-specific feature requirements are derived from current source code only.
4. Existing Discovery Program logic is reused or extended where practical.
5. The analyzer produces a feature support matrix for source-code-discovered PRISME features that depend on CPE data model capabilities.
6. Each feature reports:
   - support status,
   - evidence confidence,
   - required concepts,
   - matched snapshot parameters,
   - missing mandatory requirements,
   - missing optional requirements,
   - required write/control capability,
   - diagnostics/actions required,
   - runtime validation limitations,
   - source-code references used to derive the requirement.
7. Score calculations are evaluated individually based on actual calculator dependencies found in code.
8. Self-healing algorithms are evaluated individually based on actual algorithm dependencies found in code.
9. The report distinguishes static data model evidence from runtime evidence.
10. The analyzer never claims full runtime support when this cannot be proven from the static snapshot.
11. Tests cover supported, partial, unsupported, unknown, implementation_not_found, TR-181, TR-098, missing writability/control capability, and deterministic ordering cases.
12. Documentation explains how to add a new feature requirement and how to add a new parameter mapping.

## 9) Collaboration Shortcuts
These shortcuts define agreed interaction commands between user and agent.

- Command-gated implementation rule (BINDING)
  - The agent may start implementation work (any code/file changes, commits, pushes, delivery execution) only after the user explicitly invokes one of the defined shortcut commands in this section.
  - If no defined shortcut command is invoked, the agent must stay in conversation mode only (analysis, clarification, review, planning, recommendations) and must not modify repository files.
  - If a user message requests implementation but does not invoke a defined shortcut command, the agent must ask for an explicit shortcut command before making changes.

- `commit&push`
  - Stage all modified/added files relevant to completed work.
  - Ensure the working tree contains only intended files for the step (no unrelated changes).
  - Create one clear commit message matching the implemented change.
  - Push to `origin main` (`git push -u origin main`).
  - Use only after the gate checklist passes for that step:
    - `make test`
    - milestone-specific required checks

- `commit&push!`
  - Same behavior as `commit&push`.
  - Additionally return a short post-push summary:
    - what changed (1-3 bullets)
    - tests/checks executed
    - pass/fail outcome

- `autostep`
  - Implement the next milestone in the agreed plan.
  - Apply the same gate checklist before each commit:
    - `make test`
    - milestone-specific required checks
  - If all checks pass: update required documentation, then run `commit&push`.
  - Continue to the next milestone automatically.
  - Stop only on blocker/failing checks and report exact failure details.

- `planlock`
  - Build or refresh a concrete milestone plan before implementation.
  - Each milestone must include: scope, files, tests, commands, acceptance criteria.
  - Keep milestones PR-sized and unambiguous.

- `plandiff`
  - Compare current implementation state against the agreed plan.
  - Report clearly: completed, in-progress, missing, blocked.

- `testgate`
  - Run only the agreed verification gate checklist without making code changes.
  - Return pass/fail with failing command output summary.

- `docsync`
  - Run documentation consistency pass across key docs:
    - `README.md`

- `releasecheck`
  - Execute pre-release verification:
    - full tests
    - bench gate
    - CLI/options/doc consistency check
    - key config schema sanity check

- `hotfix`
  - Implement one narrowly scoped bugfix.
  - Add minimal targeted tests.
  - Run gate checklist and perform `commit&push`.

- `audit`
  - Perform read-only technical assessment (no code changes).
  - Return findings, risks, and recommended actions.

- `deliveryflow`
  - End-to-end delivery sequence:
    - `planlock`
    - plan review/approval
    - `autostep` (repeat until milestones are complete)
    - `plandiff`
    - `testgate`
    - `docsync`
    - `releasecheck`
    - final `commit&push!`

- `devloop`
  - Day-to-day incremental loop:
    - run `autostep`
    - if blocked, resolve direction, then run `autostep` again
    - after milestone set is done: `releasecheck`
    - finalize with `commit&push!`

- `release-musl`
  - Build and publish Linux static CLI release artifact flow.
  - Execution sequence:
    - run release gate checklist:
      - `make test`
    - create version tag in format `vX.Y.Z`.
    - push tag to `origin`.
    - verify GitHub release workflow success.
    - report resulting release asset names and checksum entries.
  - Use when producing downloadable prebuilt static binary artifacts for team/users.
