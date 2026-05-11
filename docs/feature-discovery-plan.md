# Feature Discovery Plan

## Objective

Produce a source-code-derived PRISME feature inventory for the Offline CPE Compatibility Analyzer.

The deliverable for this step is a Markdown feature description file. It must identify active PRISME features, describe their user-facing purpose, assign an end-user importance score, and cite the code locations where the feature or its critical calculation/decision logic is implemented.

This step must not produce the final feature-to-data-model-parameter mapping. Parameter mapping is a later step after the feature list and classification are confirmed.

## Review Scope

Repositories to review:

- `/home/mtorbus/hdd/gitrep/prisme/prisme-backend`
- `/home/mtorbus/hdd/gitrep/prisme/prisme-ui`
- `/home/mtorbus/hdd/gitrep/prisme/tss`

Primary evidence should come from active backend and TSS implementation paths. UI evidence may be used for names, labels, DTO exposure, and user-visible feature semantics, but UI labels alone are not enough to classify a feature as active.

## Output File

Create the feature inventory here:

- `docs/offline-analyzer/feature-inventory.md`

The file must include:

- feature name
- feature ID candidate
- category
- short user-facing description
- end-user importance score from 1 to 5
- importance rationale
- active implementation evidence with repository-relative file paths and line numbers where practical
- notes on whether the feature consumes CPE data model values in calculation or decision logic
- exclusion notes for obsolete or unused candidates discovered during review

## Importance Scoring

Use this scale:

- `5`: critical to core customer-visible PRISME value, service health, connectivity, or operational decisions
- `4`: high-value feature used for diagnostics, monitoring, scoring, or support workflows
- `3`: useful feature that improves visibility or workflow quality but is not central
- `2`: narrow or supporting feature with limited direct end-user impact
- `1`: low-impact or mostly internal support capability

Scores must be based on source-code-informed judgment. Do not use product assumptions or external PRISME documentation.

## Active Code Criteria

Include a feature in the main inventory only when there is evidence that it is active or intended to be active, such as:

- referenced by live services, controllers, jobs, consumers, calculators, or APIs
- wired through dependency injection, routing, configuration, scheduling, or message handling
- used by current UI/API flows
- represented in active DTOs, schemas, database models, tests, or fixtures
- connected to score, insight, discovery, diagnostics, self-healing, topology, monitoring, or NOC workflows

## Obsolete or Unused Code Handling

Do not include obsolete or unused code in the main feature list.

Move a candidate to an `Excluded or obsolete candidates` section when evidence suggests it is inactive, such as:

- legacy/deprecated package or filename
- no active call path or DI/routing/scheduler wiring found
- only historical tests or fixtures reference it
- replaced by a newer implementation
- dead code discovered by references/search inspection

For excluded candidates, record the candidate name, evidence path, and reason for exclusion.

## Search Areas

Use bounded source investigation across the three repositories. Search at minimum for:

- discovery, rediscovery, capability, capabilities, compliance
- score, scores, scoring, health, insight, KPI, QoE
- selfHealing, self-healing, healing, optimization, optimizer
- diagnostic, diagnostics, speedtest, speed test, ping, TR143
- WiFi, channel, bandwidth, interference, noise, retry, RSSI, signal, utilization
- WAN, LAN, device, CPE, host, topology, mesh
- BulkData, CWMP, USP, ACS
- supported, unsupported, partial, feature flag, featureFlags
- TR181, TR-181, Device., TR098, TR-098, InternetGatewayDevice.

Also inspect:

- controllers/routes/API handlers
- scheduled jobs and consumers
- service classes/packages
- score calculators and query templates
- self-healing decision/action code
- OpenAPI/Swagger/protobuf/GraphQL schemas if present
- DTOs and database entities
- UI feature labels and API clients
- tests, fixtures, and sample payloads
- configuration and dependency injection wiring

## Milestones

### Milestone 1: Repository and Entry Point Inventory

Scope:

- Confirm the three repositories exist.
- Identify language stacks and primary module structure.
- Locate likely entry points for APIs, services, jobs, calculators, UI feature exposure, and tests.

Files:

- `docs/offline-analyzer/feature-inventory.md`

Commands:

- `find /home/mtorbus/hdd/gitrep/prisme -maxdepth 1 -mindepth 1 -type d -printf '%f\n' | sort`
- `rg --files /home/mtorbus/hdd/gitrep/prisme/prisme-backend /home/mtorbus/hdd/gitrep/prisme/prisme-ui /home/mtorbus/hdd/gitrep/prisme/tss`

Acceptance criteria:

- Feature inventory contains a repository overview section.
- Review scope and active-code criteria are recorded.

### Milestone 2: Candidate Feature Discovery

Scope:

- Search all three repositories for feature candidates.
- Group candidates into categories such as discovery, scores, diagnostics, self-healing, topology, monitoring, NOC, UI-exposed capabilities, and operational workflows.
- Record enough evidence to separate active candidates from weak references.

Files:

- `docs/offline-analyzer/feature-inventory.md`

Commands:

- `rg -n "discovery|rediscovery|capability|capabilities|compliance" <repos>`
- `rg -n "score|scores|scoring|health|insight|kpi|QoE" <repos>`
- `rg -n "selfHealing|self-healing|healing|optimization|optimizer" <repos>`
- `rg -n "diagnostic|diagnostics|speedtest|speed test|ping|TR143" <repos>`
- `rg -n "WiFi|channel|bandwidth|interference|noise|retry|RSSI|signal|utilization" <repos>`
- `rg -n "WAN|LAN|device|CPE|host|topology|mesh" <repos>`

Acceptance criteria:

- Candidate feature list exists with evidence pointers.
- Weak or label-only references are clearly marked for further verification.

### Milestone 3: Active Implementation Verification

Scope:

- For each candidate, verify whether it is active by tracing call paths, wiring, routes, jobs, consumers, configuration, or tests.
- Identify the code location where CPE-derived values are used in calculation or decision logic when such a location exists.
- Exclude obsolete or unused candidates from the main list.

Files:

- `docs/offline-analyzer/feature-inventory.md`

Commands:

- targeted `rg -n` searches per candidate feature
- targeted `sed -n` or `nl -ba` reads around active implementation paths
- `rg -n` reference searches for critical classes/functions/constants

Acceptance criteria:

- Each included feature has active implementation evidence.
- Each excluded candidate has an exclusion reason.
- No feature is included solely from a UI label or orphaned enum.

### Milestone 4: Importance Classification and Final Review

Scope:

- Assign importance scores from 1 to 5.
- Add concise rationales grounded in user impact and source-code context.
- Ensure the file explicitly states that parameter mapping is intentionally deferred.

Files:

- `docs/offline-analyzer/feature-inventory.md`

Commands:

- `rg -n "TODO|unknown|implementation_not_found|Excluded|importance" docs/offline-analyzer/feature-inventory.md`
- `git diff -- docs/offline-analyzer/feature-inventory.md`

Acceptance criteria:

- Feature inventory is reviewable as a standalone Markdown document.
- The report separates included active features from excluded/obsolete candidates.
- The report does not contain final feature-to-data-model-parameter mappings.

## Gate Checklist

Before considering this discovery step complete:

- Verify the feature inventory file exists.
- Verify it contains source-code evidence for every included feature.
- Verify obsolete or unused candidates are separated from active features.
- Verify no final parameter mapping table is included.
- Run `git diff -- docs/offline-analyzer/feature-inventory.md` and review for accidental unrelated edits.

No test suite is required for this documentation-only discovery step. If `make test` is unavailable in this repository, record that as expected until implementation scaffolding exists.
