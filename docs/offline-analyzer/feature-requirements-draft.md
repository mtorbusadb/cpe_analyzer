# Feature Requirements Draft

## Scope

This document drafts feature-to-concept requirements for the Offline PRISME CPE Compatibility Analyzer.

Current coverage:

- Milestone 1: Discovery Program features only.

Later milestones will extend this draft with score, self-healing, customer-care, topology, and diagnostics features from `docs/offline-analyzer/feature-inventory.md`.

This is not the final support-rule registry. It does not claim whether a specific device snapshot is supported, partial, unsupported, or unknown.

## Requirement Format

Each feature entry includes:

- `featureId`
- feature name
- category
- importance
- mandatory concept candidates
- optional/degrading concept candidates
- control/write concepts
- diagnostic/action concepts
- required granularity
- history/window/frequency needs
- fallback behavior
- evidence references
- open questions

Concept IDs refer to `docs/offline-analyzer/capability-concepts.md`.

## Draft Requirements

### `discovery.capabilityScan`

- Feature name: Discovery Program capability scan
- Category: Discovery
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Discovery Program capability scan`

Mandatory concept candidates:

- `discovery.dataModel.family`
- `discovery.capability.suite`
- `discovery.capability.module`
- `discovery.capability.requirement`
- `discovery.capability.propertyPresence`
- `discovery.capability.status`

Optional/degrading concept candidates:

- `discovery.capability.propertySet`
- `discovery.capability.score`
- `discovery.comm.qoeAgent.diagnosticCapability`

Control/write concepts:

- None found for the capability scanner itself in Milestone 1.

Diagnostic/action concepts:

- `discovery.comm.qoeAgent.diagnosticCapability` when QoE Agent capability suite is included.

Required granularity:

- Firmware/device.
- Capability suite.
- Capability module.
- Individual requirement/property.

History/window/frequency needs:

- No history or collection frequency requirement found in the Discovery Program capability scanner.

Fallback behavior:

- Data model family selects TR-181 capability suites when firmware data model is TR-181; otherwise TR-098 suites are used.
- Empty CPE IP address skips QoE Agent scan.
- Missing mandatory property makes a module unsupported.
- Missing recommended property makes a module partial if mandatory requirements pass.
- Optional-only modules are considered supported by current scanner logic.

Evidence references:

- `prisme-backend/services/discovery-program/src/device_processor.py:111` selects TR-181 capability suites.
- `prisme-backend/services/discovery-program/src/device_processor.py:114` selects TR-098 capability suites.
- `prisme-backend/services/discovery-program/src/device_processor.py:121` creates the capability scanner.
- `prisme-backend/services/discovery-program/src/device_processor.py:123` adds the selected capability suites to the scanner.
- `prisme-backend/services/discovery-program/src/device_processor.py:124` conditionally adds QoE Agent capability suite.
- `prisme-backend/services/discovery-program/src/device_processor.py:127` logs that QoE Agent scan is skipped when IP address is empty.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:8` defines requirement types.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:27` defines capability requirements.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:76` computes module support status.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:259` evaluates property sets.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:327` analyzes modules.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:343` analyzes suites.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:248` computes overall Discovery capability score.

Open questions:

- The offline analyzer should decide whether to preserve Discovery's optional-only-is-supported behavior exactly or annotate it with static-analysis limitations.
- The final implementation needs to determine how input JSON report data represents absent paths, present paths with no value, and object-only entries.

### `discovery.platformFeatureScan`

- Feature name: Discovery Program platform feature scan
- Category: Discovery
- Importance: `5`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Discovery Program platform feature scan`

Mandatory concept candidates:

- `discovery.dataModel.family`
- `discovery.feature.ruleSet`
- `discovery.feature.dataModelPresenceRule`
- `discovery.feature.parameterValueRule`
- `discovery.feature.valueComparisonRule`

Optional/degrading concept candidates:

- `discovery.feature.indexedValueRule`
- `discovery.feature.countMatchingInstancesRule`
- `discovery.feature.valueSwitchRule`
- `discovery.feature.constantRule`

Control/write concepts:

- None found in the platform feature rule processor. Rules read data model availability/values and produce feature outputs.

Diagnostic/action concepts:

- None found directly in platform feature scan rules in Milestone 1.

Required granularity:

- Firmware/device.
- Rule context.
- Rule set.
- Individual rule.
- Object instance when rules use `.{i}.` instance placeholders.

History/window/frequency needs:

- No history or collection frequency requirement found in the platform feature processor.

Fallback behavior:

- Rules execute in defined order.
- If a rule fails, no further rules in that rule set are processed and the rule set default is returned.
- If an intermediate rule returns boolean `false`, processing stops and the default is returned.
- If a boolean is the final evaluated value, it is converted to `"true"` or `"false"`.
- If no evaluated value is produced, the rule set default is used.

Evidence references:

- `prisme-backend/services/discovery-program/src/device_processor.py:139` selects TR-181 feature rules.
- `prisme-backend/services/discovery-program/src/device_processor.py:142` selects TR-098 feature rules.
- `prisme-backend/services/discovery-program/src/device_processor.py:144` creates the platform features processor.
- `prisme-backend/services/discovery-program/src/device_processor.py:147` adds each feature context.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:51` documents ordered rule processing.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:52` documents failure/default behavior.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:56` processes rule sets.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:60` iterates rules.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:70` initializes feature status.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:78` applies the rule set default.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:192` stores feature outputs.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:244` implements data model presence checks.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:111` implements direct parameter value retrieval.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:182` implements indexed instance lookup.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:308` implements count-if instance logic.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:281` implements value comparison.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:400` implements value switching.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:413` implements constant set rules.

Open questions:

- Some TR-098 feature rule sets contain commented TODO logic. Those should remain `unknown` or deferred until active executable rules are confirmed.
- The offline analyzer needs deterministic behavior for rule defaults when a required snapshot value is missing.

### `discovery.rediscovery`

- Feature name: Firmware rediscovery workflow
- Category: Discovery
- Importance: `4`
- Source inventory entry: `docs/offline-analyzer/feature-inventory.md`, section `Firmware rediscovery workflow`

Mandatory concept candidates:

- `discovery.capabilityScan`
- `discovery.platformFeatureScan`

Optional/degrading concept candidates:

- `discovery.comm.qoeAgent.diagnosticCapability`

Control/write concepts:

- None for offline analyzer scope. Runtime rediscovery triggers live scanning and is explicitly outside offline execution.

Diagnostic/action concepts:

- None for offline analyzer execution. Runtime Discovery may include QoE Agent capability checks when IP address is available.

Required granularity:

- Firmware.
- Device endpoint.

History/window/frequency needs:

- No history or collection frequency requirement found for the offline analyzer.

Fallback behavior:

- Offline analyzer should not execute rediscovery.
- Rediscovery should be represented as a runtime workflow whose offline equivalent is running the analyzer over a static JSON input.

Evidence references:

- `prisme-backend/services/discovery-program/src/device_processor.py:80` starts a Discovery thread for TR-369.
- `prisme-backend/services/discovery-program/src/device_processor.py:85` starts a Discovery thread for TR-069.
- `prisme-backend/services/discovery-program/src/device_processor.py:90` starts a Discovery thread for direct/QoE path.
- `prisme-backend/services/discovery-program/src/device_processor.py:104` prepares capability scanning.
- `prisme-backend/services/discovery-program/src/device_processor.py:133` prepares feature scanning.
- `prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts:54` exposes all-device rediscovery request.
- `prisme-ui/apps/discovery-dashboard/src/api/rtk/deviceApi.ts:65` exposes per-firmware rediscovery request.
- `prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts:34` triggers rediscovery and monitoring.
- `prisme-ui/apps/discovery-dashboard/src/hooks/useDeviceRediscoveryAndMonitor.ts:50` refetches capabilities and states.

Open questions:

- Rediscovery is not itself an offline feature to execute. It should likely appear in reports as `runtimeValidationRequired` or as context explaining that offline analysis replaces live rediscovery for static snapshots.

## Inventory Coverage Status

Covered in Milestone 1:

- `discovery.capabilityScan`
- `discovery.platformFeatureScan`
- `discovery.rediscovery`

Deferred to later milestones:

- `score.cpe.overall`
- `score.cpe.wifi`
- `score.host.wifi`
- `score.cpe.internet`
- `noc.populationScores`
- `selfHealing.remoteChannelManagement`
- `selfHealing.aqosDynamicPrioritization`
- `selfHealing.aqosAirtimeFairnessTuning`
- `selfHealing.aqosRtsCtsThresholdTuning`
- `customerCare.wifiSettings`
- `customerCare.topologyMap`
- `diagnostics.speedtest`
- `customerCare.scoreDrilldown`

Deferred reason:

- The locked plan separates Discovery, score calculation, self-healing, and customer-care concept extraction into different milestones to keep review scope bounded.
