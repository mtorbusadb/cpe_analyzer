# Coverage Gap Matrix

Date: 2026-05-14
Source of truth:
- Target list: `docs/offline-analyzer/full-coverage-plan.md`
- Current implementation: `src/registry.rs`

## Quantified baseline

- target features: 16
- implemented features in registry: 16
- coverage ratio: 16/16 (100%)

Status of target features by implementation presence:

- implemented: 16
- missing: 0

## Tier coverage

### P0

- target: 7
- implemented: 7
- missing: 0

### P1

- target: 6
- implemented: 6
- missing: 0

### P2

- target: 3
- implemented: 3
- missing: 0

## Residual gaps (quality, not existence)

Although all target features are present in registry and recent milestones improved quality, remaining gap categories are:

1. Ongoing evidence drift monitoring
- `align` smoke and deterministic-output coverage now exist, and registry reference integrity is clean.
- Continue periodic read-only `align` runs to detect commit drift against the locked baseline.

2. Documentation drift risk as rules evolve
- Example output sync is now enforced by test.
- Plan/gap docs still need periodic refresh whenever feature registry semantics change.

3. Release-readiness gate execution
- Core status coverage is broad, including mixed-edge fixtures and source-aligned `unknown`/`implementation_not_found`.
- The remaining step is full pre-release verification via the agreed `releasecheck` flow.

## Current drift/risk snapshot

From latest align artifact:
- baseline SHAs are locked in `prisme-baseline.lock`.
- missing registry evidence paths: 0

## Next implementation focus

1. Execute release-level verification pass
- Run `releasecheck` (full tests, bench gate, CLI/options/doc consistency check, config/schema sanity).

2. Resolve any releasecheck failures
- Apply narrowly scoped fixes, rerun gate, and keep deterministic output guarantees intact.

3. Finalize delivery sequence
- After green releasecheck, run `commit&push!` summary and proceed to release/tag flow when requested.
