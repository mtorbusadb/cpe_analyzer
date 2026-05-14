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

3. Coverage depth for mixed-edge status transitions
- Core status coverage is broad, including source-aligned `unknown` and `implementation_not_found`.
- Remaining opportunity is richer mixed-edge fixtures that exercise multiple degraded branches in one snapshot while preserving deterministic ordering.

## Current drift/risk snapshot

From latest align artifact:
- baseline SHAs are locked in `prisme-baseline.lock`.
- missing registry evidence paths: 0

## Next implementation focus

1. Expand deterministic/reporting guardrails
- Add lightweight checks for additional committed docs that depend on generated/reported content.

2. Mixed-edge fixture hardening
- Add one composite fixture that simultaneously drives partial/unsupported splits across score, diagnostics, and customer-care features.

3. Prepare release-level verification pass
- Run `releasecheck` once no further milestone changes are pending.
