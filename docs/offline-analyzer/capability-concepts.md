# Capability Concepts

## Scope

This document defines canonical capability concept candidates for the Offline PRISME CPE Compatibility Analyzer.

Current coverage:

- Milestone 1: Discovery Program concepts only.

Later milestones will extend this file with score, self-healing, customer-care, topology, and diagnostics concepts.

This document does not define the final raw TR-181/TR-098/vendor path mapping table. Source-code paths may appear only as evidence examples for existing PRISME behavior.

## Concept Format

Each concept entry includes:

- `conceptId`
- name
- category
- concept type
- confidence
- description
- source-code evidence
- notes

Concept types:

- `staticMetadata`
- `dataModelPresence`
- `parameterValue`
- `ruleEvaluation`
- `supportStatus`
- `derivedMetric`
- `diagnosticAction`
- `controlAction`
- `runtimeOnly`

Confidence:

- `high`: directly implemented in source code.
- `medium`: inferred from source-code structure or DTO/API/storage flow.
- `low`: weak or indirect evidence only.

## Discovery Program Concepts

### `discovery.dataModel.family`

- Name: Discovery data model family
- Category: Discovery
- Concept type: `staticMetadata`
- Confidence: `high`
- Description: Selects which Discovery rule set is used for a firmware: TR-181 or TR-098.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/device_processor.py:111` selects `rules.CAPS_TR181_ALL` for TR-181 firmware.
- `prisme-backend/services/discovery-program/src/device_processor.py:114` selects `rules.CAPS_TR098_ALL` otherwise.
- `prisme-backend/services/discovery-program/src/device_processor.py:139` selects `rules.FEATURES_TR181_ALL` for TR-181 firmware.
- `prisme-backend/services/discovery-program/src/device_processor.py:142` selects `rules.FEATURES_TR098_ALL` otherwise.
- Notes: The offline analyzer should detect or accept the input data model family before applying Discovery-derived requirement logic.

### `discovery.capability.suite`

- Name: Capability suite
- Category: Discovery
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: A grouped Discovery capability area such as WAN, LAN, Wi-Fi, speed-test diagnostics, mesh, self-healing, or CPE performance.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:39` defines `Capability.Suite`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:42` stores suite label.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:43` stores suite description.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:44` stores suite modules.
- `prisme-backend/services/discovery-program/src/rules/tr181_caps.py:293` defines `WAN_SUITE`.
- `prisme-backend/services/discovery-program/src/rules/tr181_caps.py:764` defines `WIFI_SUITE`.
- `prisme-backend/services/discovery-program/src/rules/tr181_caps.py:1347` defines `WIFI_SELF_HEALING_SUITE`.
- `prisme-backend/services/discovery-program/src/rules/tr181_caps.py:1772` defines `SPEED_TEST_SUITE`.
- `prisme-backend/services/discovery-program/src/rules/tr098_caps.py:134` defines `WAN_SUITE` for TR-098.
- `prisme-backend/services/discovery-program/src/rules/tr098_caps.py:459` defines `WIFI_SUITE` for TR-098.
- Notes: This is a grouping concept, not a direct CPE parameter concept.

### `discovery.capability.module`

- Name: Capability module
- Category: Discovery
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: A capability check group inside a suite. Module status is derived from the requirements it contains.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:33` defines `Capability.Module`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:36` stores module label.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:37` stores module requirements.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:327` analyzes a module.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:332` iterates module requirements.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:187` converts module results to storage output.
- Notes: Modules are the natural source-code unit for deriving feature requirement groups in the offline analyzer.

### `discovery.capability.requirement`

- Name: Capability requirement
- Category: Discovery
- Concept type: `dataModelPresence`
- Confidence: `high`
- Description: A required, recommended, or optional property/predicate checked by the Discovery capability scanner.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:8` defines requirement types.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:11` defines `MANDATORY`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:12` defines `RECOMMENDED`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:13` defines `OPTIONAL`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:27` defines `Capability.Requirement`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:30` stores the property or property set.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:31` stores the requirement type.
- Notes: This concept is high priority for the offline analyzer because it already encodes mandatory versus degrading requirements.

### `discovery.capability.propertyPresence`

- Name: Capability property presence
- Category: Discovery
- Concept type: `dataModelPresence`
- Confidence: `high`
- Description: Presence check for a single data model property used by capability scanning.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:15` defines property names as strings.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:296` analyzes a single property.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:302` delegates single-property support to `_check_supported`.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_069_discovery_endpoint/tr_069.py:21` checks TR-069 parameter availability by attempting value retrieval.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_369/tr_369.py:23` checks TR-369 parameter availability through supported object metadata.
- Notes: For offline analysis, this concept maps to snapshot path presence. Access flags may refine it later if snapshot metadata contains readability/writability.

### `discovery.capability.propertySet`

- Name: Capability property set
- Category: Discovery
- Concept type: `dataModelPresence`
- Confidence: `high`
- Description: Boolean AND/OR composition of data model property presence checks.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:17` defines property boolean operations.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:21` defines `Capability.PropertySet`.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:24` stores the property set operation.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_types.py:25` stores nested properties.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:259` recursively evaluates property sets.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:287` applies AND semantics.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:289` applies OR semantics.
- Notes: The offline analyzer should preserve AND/OR semantics instead of flattening property sets into unordered paths.

### `discovery.capability.status`

- Name: Capability support status
- Category: Discovery
- Concept type: `supportStatus`
- Confidence: `high`
- Description: Module and suite support result derived from mandatory, recommended, and optional requirement outcomes.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:76` computes module status.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:78` marks module unsupported when mandatory requirements are missing.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:83` marks supported when mandatory and recommended requirements are satisfied.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:87` marks partial when mandatory requirements are satisfied but not all recommended requirements are satisfied.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:137` computes suite status from module statuses.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:140` marks suite partial when at least one module is partial or supported but not all are supported.
- Notes: This is directly reusable for analyzer support-status semantics, but runtime-only limitations still need separate reporting.

### `discovery.capability.score`

- Name: Discovery capability score
- Category: Discovery
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Numeric Discovery score derived from supported and partial suite/module counts.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:248` defines overall score calculation.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:253` gives partial support half weight.
- `prisme-backend/services/discovery-program/src/disc_prog_capability_scanner.py:255` scales the result to 5.
- Notes: This is a Discovery-specific score, not a QoE score.

### `discovery.feature.ruleSet`

- Name: Platform feature rule set
- Category: Discovery
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: Ordered rule chain used to derive one platform feature value from data model presence, values, indexes, counts, switches, or constants.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:51` states rules are processed in the order defined.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:52` states failed rules stop further processing and return default value.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:237` defines `RuleSet`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:56` processes a rule set.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:60` iterates rules.
- `prisme-backend/services/discovery-program/src/disc_prog_features_processor.py:78` uses default value when evaluation does not produce an evaluated result.
- Notes: This concept is needed for feature value outputs such as Wi-Fi standard, radio count, LAN port count, and support booleans.

### `discovery.feature.dataModelPresenceRule`

- Name: Platform feature data model presence rule
- Category: Discovery
- Concept type: `dataModelPresence`
- Confidence: `high`
- Description: Platform feature rule that checks whether one or more data model paths exist using AND/OR logic.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:13` describes `DATAMODEL_HAS`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:149` defines `DataModelHasArgs`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:244` defines `DataModelHasProcessor`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:258` applies AND logic.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:264` applies OR logic.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:272` executes the data model presence check.
- Notes: This is equivalent to a feature-level presence rule, not a capability-suite requirement.

### `discovery.feature.parameterValueRule`

- Name: Platform feature parameter value rule
- Category: Discovery
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Platform feature rule that reads a parameter value from a device and uses it in later rule evaluation.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:22` describes `GET`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:111` defines `GetProcessor`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:125` executes value retrieval.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:127` calls `interface.get_parameter_value`.
- Notes: For offline analysis, this requires the snapshot to provide a value, not merely path presence, when the rule depends on `GET`.

### `discovery.feature.indexedValueRule`

- Name: Platform feature indexed value rule
- Category: Discovery
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Platform feature rule that resolves an instance index and then reads another value using that index.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:26` describes `GET_BY_IDX`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:31` describes `GET_IDX`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:140` defines `GetByIdxProcessor`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:167` replaces `.{i}.` with the previous value.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:182` defines `GetIdxProcessor`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:210` reads `NumberOfEntries` to iterate instances.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:229` checks each concrete instance path.
- Notes: Offline support requires instance-aware snapshot handling, including `NumberOfEntries` or equivalent object instance enumeration.

### `discovery.feature.countMatchingInstancesRule`

- Name: Platform feature count matching instances rule
- Category: Discovery
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Platform feature rule that counts instances matching a comparison condition.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:36` describes `COUNT_IF`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:308` defines `CountIfProcessor`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:338` reads `NumberOfEntries`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:352` iterates concrete instance paths.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:355` counts values matching the comparison.
- Notes: This drives feature values such as LAN port count and FXS port count.

### `discovery.feature.valueComparisonRule`

- Name: Platform feature value comparison rule
- Category: Discovery
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: Rule that compares the previous rule value against one or more expected values.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:18` describes `CHECK_VAL`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:73` defines comparison operators.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:83` defines `EQUAL`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:84` defines `NOT_EQUAL`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:85` defines `LIST_CONTAINS`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:281` defines `CheckValueProcessor`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:299` executes comparison against previous value.
- Notes: This is a rule-stage concept, not a raw CPE capability by itself.

### `discovery.feature.valueSwitchRule`

- Name: Platform feature value switch rule
- Category: Discovery
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: Rule that maps a previous value to a normalized feature output string.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:42` describes `SWITCH`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:203` defines `SwitchArgs`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:400` iterates switch matches.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:406` returns the mapped value.
- Notes: This appears in Wi-Fi standard and port speed normalization.

### `discovery.feature.constantRule`

- Name: Platform feature constant rule
- Category: Discovery
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: Rule that sets a platform feature to a fixed output value.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/disc_prog_features_types.py:47` describes `SET`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:413` defines `SetProcessor`.
- `prisme-backend/services/discovery-program/src/disc_prog_features_rule_processor.py:427` returns the configured value.
- Notes: This concept is useful for features whose value is derived from rule logic rather than direct CPE values.

### `discovery.comm.tr069.parameterProbe`

- Name: TR-069 parameter probe
- Category: Discovery
- Concept type: `runtimeOnly`
- Confidence: `high`
- Description: Runtime Discovery probe that checks parameter availability by attempting to read a value through the CWMP discovery endpoint.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/comm_interface/tr_069_discovery_endpoint/tr_069.py:10` defines the TR-069 interface.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_069_discovery_endpoint/tr_069.py:21` checks parameter availability.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_069_discovery_endpoint/tr_069.py:24` attempts value retrieval.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_069_discovery_endpoint/tr_069.py:37` implements parameter value retrieval.
- Notes: Offline analysis cannot execute this probe; it must approximate it using snapshot content.

### `discovery.comm.tr369.supportedObjectProbe`

- Name: TR-369 supported object probe
- Category: Discovery
- Concept type: `runtimeOnly`
- Confidence: `high`
- Description: Runtime Discovery probe that checks supported objects, parameters, commands, and events through USP metadata.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/comm_interface/tr_369/tr_369.py:10` defines the TR-369 interface.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_369/tr_369.py:23` checks parameter availability.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_369/tr_369.py:47` checks command availability.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_369/tr_369.py:68` checks event availability.
- `prisme-backend/services/discovery-program/src/comm_interface/tr_369/tr_369.py:89` retrieves a parameter value.
- Notes: Offline snapshot metadata would need explicit command/event information to match this runtime behavior.

### `discovery.comm.qoeAgent.diagnosticCapability`

- Name: QoE Agent diagnostic capability
- Category: Discovery
- Concept type: `diagnosticAction`
- Confidence: `high`
- Description: Runtime Discovery probe for QoE Agent supported diagnostics and callable APIs.
- Source-code evidence:
- `prisme-backend/services/discovery-program/src/comm_interface/qoe_agent/qoe_agent.py:67` checks whether a QoE Agent diagnostic is supported.
- `prisme-backend/services/discovery-program/src/comm_interface/qoe_agent/qoe_agent.py:71` lists expected diagnostic names in the docstring.
- `prisme-backend/services/discovery-program/src/comm_interface/qoe_agent/qoe_agent.py:89` parses `SupportedDiagnostics`.
- `prisme-backend/services/discovery-program/src/comm_interface/qoe_agent/qoe_agent.py:94` checks QoE Agent command availability.
- `prisme-backend/services/discovery-program/src/comm_interface/qoe_agent/qoe_agent.py:122` retrieves QoE Agent API output.
- Notes: This is not a TR-181/TR-098 data model path concept. It is a runtime agent/API capability.

## Deferred Concept Areas

The following concept groups are not covered in Milestone 1 and will be added in later milestones:

- QoE score derived metric concepts.
- Self-healing telemetry and control concepts.
- Customer-care Wi-Fi management concepts.
- Topology/map visibility concepts.
- Speedtest and diagnostics execution concepts outside the Discovery Program.
- Raw input JSON parsing concepts.
