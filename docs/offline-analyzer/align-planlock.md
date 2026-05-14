# Align Planlock (Gap-Closure Milestones)

Date: 2026-05-14
Derived from: `docs/offline-analyzer/align-drift-report.md`

## Milestone A — Baseline lock finalization

Scope:
- Replace `unknown` entries in `docs/offline-analyzer/prisme-baseline.lock` with concrete SHAs captured in align report.
- Ensure report header always emits baseline SHAs.

Files:
- `docs/offline-analyzer/prisme-baseline.lock`
- `src/baseline.rs` (only if parser/validation needs updates)
- `src/report.rs` (only if formatting updates needed)

Tests:
- existing `make test`
- add/extend test to assert baseline SHAs are surfaced in rendered text report.

Commands:
- `make test`
- sample run to `out/compatibility-report.txt`

Acceptance criteria:
- baseline lock contains concrete SHAs for all 3 repos.
- report includes these exact SHAs.

## Milestone B — Topology evidence drift remediation

Scope:
- Review `customerCare.topologyMap` source references and rules against current PRISME checkout.
- Remove/replace stale reference to `map_bulk_fallback.go`.
- keep status semantics aligned with AGENTS decision table.

Files:
- `src/registry.rs`
- `docs/offline-analyzer/investigation-report.md` (if evidence narrative needs correction)

Tests:
- existing `tests/status_coverage.rs`
- add/adjust topology-related checks if needed.

Commands:
- `make test`

Acceptance criteria:
- no missing PRISME evidence file references in registry scan.
- topology feature evidence confidence remains defensible.

## Milestone C — Align automation hardening

Scope:
- Implement deterministic align output artifact generation (optional CLI/utility path), including:
  - baseline vs current table
  - missing-reference list
  - impacted features list
- Keep PRISME repos strictly read-only.

Files:
- `scripts/` helper and/or `docs/offline-analyzer/` templates
- `README.md` usage note for `align` workflow

Tests:
- smoke test for align artifact generation (if script added).

Commands:
- `make test`

Acceptance criteria:
- repeatable align artifact generation from same inputs.
- no writes to PRISME repositories.

## Milestone D — Gate and publish

Scope:
- run gate
- commit only `cpe_analyzer` changes
- push

Commands:
- `make test`
- `commit&push` when requested

Acceptance criteria:
- all milestone criteria above met
- clean explanation of baseline compatibility state in docs/report
