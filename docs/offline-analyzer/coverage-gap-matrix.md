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

Although all target features are present in registry, remaining gap categories are:

1. Status-case depth per feature
- Some features have broad candidate coverage but still need more focused fixtures for all edge outcomes (`unknown`, `implementation_not_found`, degraded `partial` branches).

2. Mapping precision
- TR-181/TR-098 and vendor path variants are present, but precision and confidence can still improve per feature with richer fixtures.

3. Evidence durability
- Source references currently resolve for registry references scanned by `align`, but commit-level behavior drift still requires periodic `align` and targeted evidence reviews.

## Current drift/risk snapshot

From latest align artifact:
- baseline SHAs are locked in `prisme-baseline.lock`.
- missing registry evidence paths: 0

## Next implementation focus

1. P0 quality hardening
- Add fixture/test cases that force each P0 feature through all relevant status boundaries.

2. P1 control/diagnostic rigor
- Strengthen tests around writable/diagnostic gates for AQoS and speedtest pathways.

3. P2 confidence tuning
- Refine topology and NOC evidence confidence semantics and add targeted degradation fixtures.
