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

1. Unknown/implementation-not-found realism
- `unknown` and `implementation_not_found` are currently covered through custom test definitions.
- Add source-aligned fixture paths that exercise these statuses without synthetic-only feature definitions.

2. Mapping precision for vendor/extensions
- TR-181/TR-098 coverage is stronger, but vendor-specific extension patterns are still lightly represented in fixtures.

3. Ongoing evidence drift monitoring
- `align` smoke coverage now exists, and registry reference integrity is clean.
- Continue periodic read-only `align` runs to detect commit drift against the locked baseline.

## Current drift/risk snapshot

From latest align artifact:
- baseline SHAs are locked in `prisme-baseline.lock`.
- missing registry evidence paths: 0

## Next implementation focus

1. Source-aligned `unknown` and `implementation_not_found` scenarios
- Introduce fixture/rule cases that avoid synthetic-only status coverage and are traceable to documented feature inventory gaps.

2. Vendor-extension mapping fixtures
- Add one or more fixtures that explicitly exercise vendor parameter variants for mapped concepts and verify deterministic status transitions.

3. Documentation maintenance cadence
- Keep example report and gap matrix synchronized after each status/confidence rule change.
