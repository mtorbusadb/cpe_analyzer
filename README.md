# Offline PRISME CPE Compatibility Analyzer

This project analyzes a static CPE data model snapshot against PRISME feature requirements discovered from source code.

The analyzer runs offline. It does not connect to a CPE, ACS, USP controller, BulkData ingestion path, PRISME deployment, production database, or message broker.

## Current Scope

The analyzer currently supports JSON input and produces a structured plain-text report.

The report answers:

- which discovered PRISME features are supported by the supplied static evidence,
- which features are partial, unsupported, unknown, or referenced without implementation evidence,
- what evidence is missing for unsupported or degraded features,
- which runtime limitations remain even when static evidence is present.

Static JSON evidence is not proof of runtime behavior. A writable flag, diagnostic object, score document, or parameter value can make a feature statically possible, but the report still marks runtime validation where live behavior cannot be proven offline.

## Input

Input must be JSON with a top-level `Report` array.

Minimal example:

```json
{
  "Report": [
    {
      "path": "Device.WiFi.Radio.1.Channel",
      "value": "6",
      "readable": true,
      "writable": true
    }
  ]
}
```

Each `Report` entry can represent a parameter, object, diagnostic/action surface, derived metric, score document, history sample, or metadata entry.

See the full supported shape in [input-json-format.md](docs/offline-analyzer/input-json-format.md).

## Output

The primary output is a structured plain-text report.

Example command:

```bash
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --color=never
```

The report includes:

- `reportFormatVersion`,
- device identification and detected data model,
- feature support summary,
- feature matrix,
- matched requirements,
- missing mandatory requirements,
- missing optional or degraded evidence,
- runtime limitations,
- source-code evidence references.

Generated files under `out/` are local artifacts and must not be committed.

## Status Values

The analyzer uses these support statuses:

| Status | Meaning |
| --- | --- |
| `supported` | Static input satisfies all mandatory source-code-derived requirements for the feature. Runtime-only behavior is still reported separately when relevant. |
| `partial` | The feature appears possible but degraded, incomplete, lower confidence, missing optional evidence, missing granularity, or still requires runtime validation. |
| `unsupported` | Source code confirms the requirement, but the input lacks mandatory data, control, diagnostic, or action evidence. |
| `unknown` | The analyzer cannot determine support from available source-code and input evidence. |
| `implementation_not_found` | A feature name, enum, flag, UI label, config entry, or API field exists, but executable implementation or dependency logic was not found. |

For unsupported and partial features, inspect `Missing for support`, `Missing optional / degraded evidence`, and `Runtime limitations` in the report.

## Color

Color is optional ANSI text coloring.

```bash
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --color=never
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --color=always
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --no-color
```

Use `--color=never` or `--no-color` for files, CI logs, and deterministic text comparisons.

## Failing Automation On Status

Use repeatable `--fail-on <status>` when scripts should generate the report and fail if selected statuses appear.

Accepted exact values:

- `supported`
- `partial`
- `unsupported`
- `unknown`
- `implementation_not_found`

Example:

```bash
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --color=never --fail-on unsupported
```

If a selected status is present, the report is written first and the process exits with code `2`.

Invalid `--fail-on` values exit with code `1`.

## Development Commands

Run unit and integration tests:

```bash
cargo test
```

Run the project test target:

```bash
make test
```

Run the standard sample report:

```bash
cargo run -- --input docs/offline-analyzer/examples/input-basic.json --output out/compatibility-report.txt --color=never
```

Before committing changes, also run:

```bash
cargo fmt --check
git diff --check
```
