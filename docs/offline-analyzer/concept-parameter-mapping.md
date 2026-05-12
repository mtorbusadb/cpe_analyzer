# Concept Parameter Mapping

## Purpose

This document will map canonical capability concepts to JSON input evidence, raw data model path patterns, source metric names, diagnostics/actions, and runtime-only limitations.

Milestone 1 defines the table structure and mapping semantics only. Full feature mappings are intentionally deferred to later milestones.

## Mapping Status Values

| Status | Meaning |
| --- | --- |
| `mapped` | Source code provides direct evidence for the input evidence needed to satisfy the concept. |
| `partially_mapped` | Some evidence is known, but granularity, value, access, history, or runtime behavior is incomplete. |
| `runtime_only` | The concept depends on live runtime behavior that static JSON cannot prove. |
| `derived_only` | The concept is satisfied only by derived metric or score evidence, not raw data model paths. |
| `unknown` | Bounded search did not provide enough source evidence for a defensible mapping. |
| `implementation_not_found` | A reference exists, but no executable dependency path was found after bounded search. |
| `deferred` | Mapping is intentionally postponed to a later milestone. |

## Evidence Dimensions

Each mapping row must distinguish these dimensions:

- `pathPresence`: raw path or object exists in input JSON.
- `valueAvailability`: a value is present and usable.
- `typeMetadata`: input includes type information.
- `readability`: read access is explicitly known.
- `writability`: write access is explicitly known.
- `diagnosticAction`: command or diagnostic availability is known.
- `history`: timestamped samples are present for required windows.
- `derivedMetric`: score or KPI-derived value is present.
- `runtimeValidation`: live behavior remains unproven.

## Mapping Table Columns

| Column | Meaning |
| --- | --- |
| `conceptId` | Canonical concept ID from `capability-concepts.md`. |
| `status` | Mapping status. |
| `jsonEvidence` | JSON section or field that can carry evidence. |
| `rawPathPattern` | Raw path pattern when source-confirmed. |
| `sourceMetric` | Source metric or score name when source-confirmed. |
| `requiredAccess` | `read`, `write`, `diagnostic`, `command`, `derived`, `runtime_only`, or `unknown`. |
| `valueRequirement` | `path_only`, `value_required`, `metadata_required`, `history_required`, `runtime_only`, or `unknown`. |
| `granularity` | Device, radio, AP, SSID, host, WAN interface, mesh node, score document, diagnostic request, or unknown. |
| `evidence` | Source-code references proving the mapping. |
| `limitations` | Static-analysis or runtime limitations. |

## Starter Mapping Sections

### Raw Data Model Concepts

| conceptId | status | jsonEvidence | rawPathPattern | sourceMetric | requiredAccess | valueRequirement | granularity | evidence | limitations |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `discovery.capability.propertyPresence` | `deferred` | `Report[]` entries with `kind=parameter` or `kind=object` | Deferred to Milestone 2 | n/a | `read` | `path_only` | device/object/parameter | `docs/input-json-concept-mapping-plan.md:142` | Full Discovery path extraction is Milestone 2 scope. |
| `discovery.feature.parameterValueRule` | `deferred` | `Report[]` entries with `kind=parameter` | Deferred to Milestone 2 | n/a | `read` | `value_required` | device/object/parameter | `docs/input-json-concept-mapping-plan.md:142` | Full Discovery rule extraction is Milestone 2 scope. |

### Derived Metric and Score Concepts

| conceptId | status | jsonEvidence | rawPathPattern | sourceMetric | requiredAccess | valueRequirement | granularity | evidence | limitations |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `score.qoe.metricHistory` | `deferred` | `Report[]` entries with `kind=history` | n/a | Deferred to Milestone 3 | `derived` | `history_required` | metric history | `docs/input-json-concept-mapping-plan.md:169` | Full score/KPI metric mapping is Milestone 3 scope. |
| `customerCare.scoreDrilldown.scoreConsumption` | `deferred` | `Report[]` entries with `kind=score` or `kind=derivedMetric` | n/a | Deferred to Milestone 3 | `derived` | `value_required` | score document | `docs/input-json-concept-mapping-plan.md:169` | Score consumption must remain separate from score calculation support. |

### Diagnostic and Runtime Concepts

| conceptId | status | jsonEvidence | rawPathPattern | sourceMetric | requiredAccess | valueRequirement | granularity | evidence | limitations |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `diagnostics.speedtest.dispatch` | `deferred` | `Report[]` | Deferred to Milestone 4 | n/a | `diagnostic` | `runtime_only` | diagnostic request | `docs/input-json-concept-mapping-plan.md:196` | Static JSON can identify action capability, not execution success. |
| `selfHealing.runtime.cpeOnline` | `runtime_only` | `Report[]` | n/a | n/a | `runtime_only` | `runtime_only` | device endpoint | `docs/offline-analyzer/capability-concepts.md:555` | Offline JSON cannot prove current online state unless external runtime metadata is supplied. |

## Notes

- Raw path patterns are allowed in this file only when source-code-backed.
- Rows marked `deferred` are placeholders for later milestones and are not analyzer-ready mappings.
- A later implementation registry should be generated from rows with sufficient evidence, not from placeholders.
