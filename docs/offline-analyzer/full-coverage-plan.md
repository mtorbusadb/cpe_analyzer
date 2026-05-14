# Full Coverage Plan

Date: 2026-05-14
Scope: `cpe_analyzer` only (PRISME repositories remain read-only)

## Full coverage definition

For this project, "full coverage" means:

1. Every active source-code-discovered feature in `docs/offline-analyzer/feature-inventory.md` has a stable `featureId` implementation in `src/registry.rs`.
2. Each implemented feature includes source evidence references and deterministic rule IDs.
3. For each feature, the evaluator can emit all required statuses (`supported`, `partial`, `unsupported`, `unknown`, `implementation_not_found`) when corresponding fixture/input conditions exist.
4. TR-181 and TR-098 candidate-path coverage exists for all applicable features.
5. Control and diagnostic requirements are modeled where source code indicates write/action dependencies.

## Priority tiers

### P0 (foundational/high-impact)

- `discovery.capabilityScan`
- `discovery.platformFeatureScan`
- `score.cpe.overall`
- `score.cpe.wifi`
- `score.host.wifi`
- `score.cpe.internet`
- `selfHealing.remoteChannelManagement`

### P1 (major breadth)

- `discovery.rediscovery`
- `selfHealing.aqosDynamicPrioritization`
- `selfHealing.aqosAirtimeFairnessTuning`
- `selfHealing.aqosRtsCtsThresholdTuning`
- `customerCare.wifiSettings`
- `diagnostics.speedtest`

### P2 (extended operational breadth)

- `customerCare.topologyMap`
- `customerCare.scoreDrilldown`
- `noc.populationScores`

## Target feature list

Total target features: 16

1. discovery.capabilityScan
2. discovery.platformFeatureScan
3. discovery.rediscovery
4. score.cpe.overall
5. score.cpe.wifi
6. score.host.wifi
7. score.cpe.internet
8. selfHealing.remoteChannelManagement
9. selfHealing.aqosDynamicPrioritization
10. selfHealing.aqosAirtimeFairnessTuning
11. selfHealing.aqosRtsCtsThresholdTuning
12. customerCare.wifiSettings
13. customerCare.topologyMap
14. customerCare.scoreDrilldown
15. noc.populationScores
16. diagnostics.speedtest

## Explicit non-goals

- Live command execution validation against real CPE/ACS/USP.
- Runtime behavior guarantees beyond static snapshot evidence.
- Modification of `prisme-backend`, `prisme-ui`, or `tss`.
