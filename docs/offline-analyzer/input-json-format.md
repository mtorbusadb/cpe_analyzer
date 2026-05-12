# Input JSON Format

## Purpose

This document defines the JSON input shape supported by the Offline PRISME CPE Compatibility Analyzer planning phase.

Only JSON input is in scope. Other text formats are intentionally not supported in the current plan.

The supported JSON format is a top-level `Report` array. Evidence for parameters, objects, diagnostics, derived metrics, score documents, and history must be represented as entries in that array.

## Required Top-Level Shape

```json
{
  "Report": [
    {
      "path": "Device.WiFi.Radio.1.Channel",
      "value": "6"
    }
  ]
}
```

The `Report` array is required. If it is missing, the input is malformed.

Optional top-level metadata may be accepted later only if implementation needs it, but the supported input evidence remains the `Report` array.

## Report Entry Shape

A `Report` entry may represent a raw parameter, object, diagnostic/action surface, derived metric, score document, or history sample.

```json
{
  "Report": [
    {
      "kind": "parameter",
      "path": "Device.WiFi.Radio.1.Channel",
      "value": "6",
      "type": "unsignedInt",
      "readable": true,
      "writable": true,
      "timestamp": "2026-05-12T10:00:00Z"
    }
  ]
}
```

Supported fields:

| Field | Required | Meaning |
| --- | --- | --- |
| `path` | yes for raw parameter/object evidence | Raw data model path, object path, source metric name, or source evidence key. |
| `value` | no | Runtime value if available. Missing value is different from present empty value. |
| `kind` | no | Evidence kind: `parameter`, `object`, `diagnostic`, `derivedMetric`, `score`, `history`, or `metadata`. Missing defaults to `parameter`. |
| `type` | no | Source-reported type if available. |
| `readable` | no | Whether read access is known. Missing means unknown. |
| `writable` | no | Whether write access is known. Missing means unknown. |
| `available` | no | Availability evidence for object, diagnostic, or action surfaces. |
| `commandable` | no | Whether command/start capability is known for diagnostics or actions. |
| `scope` | no | Device, radio, AP, SSID, host, WAN interface, mesh node, score document, or diagnostic request. |
| `entityId` | no | Host MAC, radio ref, AP ref, interface id, or similar entity key. |
| `timestamp` | no | Timestamp for sampled value, diagnostic state, derived metric, score, or history sample. |
| `window` | no | History window label when entry represents aggregate/history evidence. |
| `source` | no | Optional provenance such as report, fixture, export, or manual input. |

## Evidence Kinds

### `parameter`

Raw leaf parameter evidence. This is the default when `kind` is missing.

```json
{
  "path": "Device.WiFi.Radio.1.Channel",
  "value": "6",
  "kind": "parameter"
}
```

### `object`

Object or instance existence evidence. This is separate from leaf value availability.

```json
{
  "path": "Device.WiFi.Radio.1.",
  "kind": "object",
  "available": true,
  "readable": true,
  "writable": false
}
```

### `diagnostic`

Static evidence that a diagnostic or action surface exists. It does not prove runtime execution success.

```json
{
  "path": "Device.WiFi.X_ADB_ChannelMeasurementsDiagnostics.1.",
  "kind": "diagnostic",
  "available": true,
  "commandable": true,
  "value": "Complete"
}
```

### `derivedMetric`

KPI or metric evidence produced from previous processing, not a raw data model path.

```json
{
  "path": "qoe_host_wifi_score",
  "kind": "derivedMetric",
  "value": 4.2,
  "scope": "host",
  "entityId": "aa:bb:cc:dd:ee:ff",
  "timestamp": "2026-05-12T10:00:00Z"
}
```

### `score`

Score document evidence.

```json
{
  "path": "qoe_cpe_score",
  "kind": "score",
  "value": 4.5,
  "scope": "device",
  "timestamp": "2026-05-12T10:00:00Z"
}
```

### `history`

Timestamped metric history evidence. History completeness must be evaluated separately from path presence.

```json
{
  "path": "qoe_ap_signal_strength",
  "kind": "history",
  "scope": "host",
  "entityId": "aa:bb:cc:dd:ee:ff",
  "window": "10m",
  "value": -61,
  "timestamp": "2026-05-12T09:55:00Z"
}
```

### `metadata`

Static metadata supplied in the same report container. Metadata must not be treated as raw data model evidence unless a later mapping row explicitly says so.

```json
{
  "path": "declaredDataModel",
  "kind": "metadata",
  "value": "TR-181"
}
```

## Evidence Semantics

The analyzer must distinguish these states:

| State | Meaning |
| --- | --- |
| absent | No evidence for this path, object, metric, or action exists in JSON. |
| present_without_value | Path or object exists, but no value is provided. |
| present_with_value | Path or metric exists and includes value evidence. |
| readable_true | Read capability is explicitly known true. |
| readable_false | Read capability is explicitly known false. |
| readable_unknown | Read capability is missing from input. |
| writable_true | Write capability is explicitly known true. |
| writable_false | Write capability is explicitly known false. |
| writable_unknown | Write capability is missing from input. |
| history_available | Timestamped history evidence exists. |
| runtime_only | Static JSON cannot prove runtime behavior. |

## Source Evidence

This input model is based on current project requirements and source-code search evidence:

- `AGENTS.md:353` documents JSON input with a top-level `Report` array containing path/value pairs.
- `AGENTS.md:359` requires feature assessment to distinguish parameter presence, value availability, readability, writability, diagnostics, and history.
- `docs/input-json-concept-mapping-plan.md:117` requires JSON representation for paths, values, types, readability, writability, diagnostics/actions, history, and derived score documents.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:170` uses data model object path and parameter maps for writes.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:157` uses metric history roots for score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:4` reads input timestamps for score/history logic.

## Current Limitations

- This document defines supported input evidence shape, not parser implementation.
- It does not yet define complete feature-to-parameter mappings.
- It does not guarantee runtime diagnostic success, write success, score freshness, or history completeness.
- Non-JSON text formats are out of scope for the current phase.
