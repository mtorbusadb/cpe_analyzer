# Align Drift Report

Date: 2026-05-14
Mode: read-only PRISME repo scan

## Baseline vs current repository versions

Baseline source:
- `docs/offline-analyzer/prisme-baseline.lock`

Baseline values:
- `prisme-backend`: `unknown`
- `prisme-ui`: `unknown`
- `tss`: `unknown`

Current values:
- `prisme-backend`
  - branch: `master`
  - commit: `d4821d7d2c5e24cd13c55fc109831a9fc3486529`
  - date: `2026-05-14 08:44:22 +0000`
- `prisme-ui`
  - branch: `master`
  - commit: `32d3cc32bf7f6cdbc1a9355ee296d3323d9ccd07`
  - date: `2026-05-13 12:13:08 +0000`
- `tss`
  - branch: `master`
  - commit: `6abab384e6f4b6bbffbecb8518e5a92beb236a25`
  - date: `2026-05-14 08:31:33 +0000`

Drift status:
- Bootstrap drift (baseline lock currently `unknown` for all repos).
- Exact historical compatibility cannot be computed from baseline lock alone.
- Action required: set baseline lock to concrete SHAs and re-run `align`.

## Evidence-path integrity scan

Scope:
- Source references declared in `src/registry.rs`.

Results:
- total referenced PRISME files: `30`
- missing referenced files: `1`

Missing reference:
- `prisme-backend/services/customer-care-agent/src/services/map_bulk_fallback.go`

Interpretation:
- The referenced fallback path appears absent in the current PRISME checkout.
- Any feature evidence relying on this file should be reviewed and either:
  - replaced with active executable references, or
  - downgraded in confidence/status semantics where appropriate.

## Impacted cpe_analyzer areas

Potentially impacted features/concepts/rules:
- `customerCare.topologyMap`
  - references include bulk fallback evidence and map assembly paths.

Potentially impacted tests/docs:
- docs:
  - `docs/offline-analyzer/investigation-report.md` (contains fallback mention)
- tests:
  - any tests assuming fallback evidence path existence should be revalidated.

## Risk notes

- High: baseline lock with `unknown` SHAs gives no deterministic compatibility anchor.
- Medium: one missing evidence file may degrade confidence correctness for topology-related assessments.
- Low: other scanned evidence file paths still resolve in current PRISME checkout.

## Unknowns

- Without a prior concrete baseline SHA set, commit-level delta analysis is unavailable.
- Behavioral drift inside existing files (logic changes without path moves) is not measured in this pass.
