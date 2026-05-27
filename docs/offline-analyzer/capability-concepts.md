# Capability Concepts

## Scope

This document defines canonical capability concept candidates for the Offline PRISME CPE Compatibility Analyzer.

Current coverage:

- Milestone 1: Discovery Program concepts only.
- Milestone 2: QoE score derived metric concepts from active score calculation code.
- Milestone 3: Self-healing telemetry, control/action, and runtime-only concepts.

Later milestones will extend this file with customer-care, topology, and diagnostics concepts.

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

Implementation note: current analyzer code now includes the first canonical
concept graph and source-inventory binder. This is intentionally a small seed
set for platform-agnostic concepts; it is not a final path mapping table.

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

## QoE Score Concepts

These concepts describe source-code-confirmed score calculation inputs and outputs as derived metrics. They are not final raw data-model parameter mappings.

### `score.qoe.metricHistory`

- Name: QoE metric history
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Memo-backed metric history used by QoE score aggregation to calculate medians, last values, and weighted means.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:675` reads WAN download history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:698` reads WAN upload history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:739` reads DSL downstream current-rate last value.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/download/qoe_download_total_kbps.js:99` stores WAN download history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/upload/qoe_upload_total_kbps.js:127` stores WAN upload history.
- Notes: Offline static snapshots can only infer whether source inputs for these histories may exist. They cannot prove enough runtime history exists.

### `score.qoe.historyWindowFreshness`

- Name: QoE history window freshness
- Category: Score
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: Median-based scores require median timestamps within the default metric history window.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:602` defines median freshness validation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:606` rejects medians older than `DEFAULT_METRIC_HISTORY_WINDOW`.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:617` returns null when a band-specific median is missing or stale.
- Notes: This is runtime/history evidence. A single input JSON snapshot cannot fully validate this concept.

### `score.qoe.zeroMeansNotAvailable`

- Name: QoE zero-as-not-available fallback
- Category: Score
- Concept type: `ruleEvaluation`
- Confidence: `high`
- Description: Score value `0` is treated as missing/not available by score composition helpers and is not emitted as an output score.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1081` defines score output helper.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1082` emits only scores greater than zero.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1161` documents min-score composition while ignoring `0`.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1165` implements `score_min_ignore_na`.
- Notes: Analyzer output should distinguish missing support from a calculated zero/no-output score.

### `score.cpe.overall`

- Name: CPE overall QoE score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Overall CPE score derived from CPE Wi-Fi score and CPE Internet score using zero-as-not-available minimum composition.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:825` composes Internet score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:828` composes CPE score from Wi-Fi and Internet scores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:834` emits `qoe_cpe_score`.
- Notes: Full runtime support depends on the child score concepts and their history inputs.

### `score.cpe.wifi`

- Name: CPE Wi-Fi QoE score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: CPE Wi-Fi score derived from Wi-Fi traffic, coverage, and network interference subscores.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:167` calculates CPE Wi-Fi network interference.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:168` calculates CPE Wi-Fi score with the truth table.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:439` derives weighted CPE Wi-Fi coverage.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:444` derives weighted CPE Wi-Fi traffic.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1196` implements CPE Wi-Fi truth-table composition.
- Notes: If more than one of traffic, coverage, and interference is missing, the current implementation returns no CPE Wi-Fi score.

### `score.host.wifi`

- Name: Host Wi-Fi QoE score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Per-host Wi-Fi score derived from host Wi-Fi coverage and traffic subscores.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:382` derives host Wi-Fi coverage.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:388` derives host Wi-Fi traffic.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:394` calculates host Wi-Fi score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:397` maps host score to host Wi-Fi score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1232` implements host Wi-Fi truth-table composition.
- Notes: Host score currently follows host Wi-Fi score in this aggregation path.

### `score.wifi.coverage`

- Name: Wi-Fi coverage score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Coverage subscore derived from RSSI/signal-strength score and PHY/downlink-rate score.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:382` composes host coverage from RSSI and PHY-rate scores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:439` composes CPE coverage from weighted RSSI and PHY-rate scores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_signal_strength.js:34` stores per-host signal-strength history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_host_last_data_downlink_rate_kbps.js:75` stores per-host downlink-rate history.
- Notes: The analyzer should treat this as a derived score concept, not as one raw parameter.

### `score.wifi.traffic`

- Name: Wi-Fi traffic score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Wi-Fi traffic subscore currently follows error-rate scoring.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:388` maps host traffic to host error-rates score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:444` maps CPE traffic to weighted CPE error-rates score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_stats_error_rate.js:134` stores host sent-error-rate history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_ap_X_instance_X_stats_error_rate.js:153` stores host received-error-rate history.
- Notes: Latency appears in comments/history support but is not used by the current traffic-score calculation path documented here.

### `score.wifi.interference`

- Name: Wi-Fi network interference score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: CPE Wi-Fi network interference score derived from noise score and channel-utilization score.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:165` calculates CPE Wi-Fi noise score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:166` calculates CPE Wi-Fi channel-utilization score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:167` composes network interference score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_wifi_radio_X_instance_X_stats_noise.js:23` stores band-specific noise history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/sampling/qoe_data_elements_radio.js:68` stores band-specific utilization history.
- Notes: A static snapshot cannot prove median freshness or adequate per-band history.

### `score.cpe.internet`

- Name: CPE Internet QoE score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: CPE Internet score derived from Internet performance, Internet latency, and WAN access scores.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:162` calculates Internet latency score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:178` calculates Internet performance score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:825` composes Internet score from performance and latency.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:826` composes WAN access into Internet score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:844` emits `qoe_cpe_internet_score`.
- Notes: The source also emits Internet quality/resolution components, but the main Internet score path uses performance, latency, and WAN access.

### `score.internet.performance`

- Name: Internet performance score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Internet performance score derived from download and upload speed scores.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:175` calculates CPE Internet download score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:176` calculates CPE Internet upload score.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:178` composes performance from download and upload scores.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:672` defines download score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:695` defines upload score calculation.
- Notes: Contract download/upload enrichment values are used when present; otherwise the code falls back to `250`.

### `score.internet.latency`

- Name: Internet latency score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Internet latency score derived from WAN ping average response time and packet-loss histories.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:642` defines Internet latency score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:647` reads ping average-response-time history and last value.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:649` reads packet-loss history and last value.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/ipping/qoe_wan_ping_averageresponsetime_ms.js:140` stores ping average-response-time history.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/kpi/ipping/qoe_wan_ping_packetLoss.js:154` stores packet-loss history.
- Notes: This requires runtime diagnostic/history evidence, not only static path presence.

### `score.wan.access`

- Name: WAN access score
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `medium`
- Description: WAN access score composed from Ethernet, DSL, GPON, or L2TP branch scores selected by WAN access type.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:769` reads WAN access type.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:770` selects WAN access scoring branch.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:779` implements Ethernet scoring.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:804` leaves xDSL standard scoring as TBD.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:808` reads GPON inputs but leaves GPON scoring as TBD.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:814` leaves L2TP scoring as TBD.
- Notes: Confidence is medium because only parts of WAN access scoring are implemented. TBD branches must not be treated as full support.

### `score.daily.rollup`

- Name: Daily score rollup
- Category: Score
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Daily score rollups calculated from stored score history medians.
- Source-code evidence:
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:911` starts daily device-score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:938` reads daily CPE Wi-Fi history medians.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1022` starts daily host-score calculation.
- `tss/services/report-parser/src/main/resources/poc/javascript/default/aggr/sampling/qoe_scores_calculation.js:1038` reads daily host Wi-Fi history medians.
- Notes: Daily rollups are history-dependent and cannot be proven from a single static input snapshot.

## Self-Healing Concepts

These concepts describe self-healing runtime dependencies discovered from workflow and activity code. They distinguish direct CPE data model reads/writes from derived OpenSearch score/traffic reads and diagnostic execution.

### `selfHealing.runtime.cpeOnline`

- Name: Runtime CPE online gate
- Category: Self-healing
- Concept type: `runtimeOnly`
- Confidence: `high`
- Description: Self-healing workflows wait for online CPE state before executing device activities.
- Source-code evidence:
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:492` wraps radio discovery in `waitForOnlineAndExecuteCpeActivity`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:526` wraps current-channel read in `waitForOnlineAndExecuteCpeActivity`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:625` wraps channel write in `waitForOnlineAndExecuteCpeActivity`.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:196` waits for online state before prioritization.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:111` waits for online state before ATF logic.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:124` waits for online state before collision tuning.
- Notes: Offline analyzer can only report this as runtime validation required.

### `selfHealing.rcm.radioInventory`

- Name: RCM radio inventory
- Category: Self-healing
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Remote Channel Management reads Wi-Fi radio objects, frequency band, and possible channels before channel selection.
- Source-code evidence:
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:491` initializes with `GetRadios`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:498` stores radio bands.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:499` stores possible radio channels.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/radio.go:126` defines `GetRadios`.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/radio.go:142` reads radio objects.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/radio.go:154` reads radio operating band.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/radio.go:160` reads possible channels.
- Notes: The final raw path mapping is deferred; this concept captures the required radio object metadata.

### `selfHealing.rcm.allowedChannels`

- Name: RCM allowed channel policy
- Category: Self-healing
- Concept type: `staticMetadata`
- Confidence: `high`
- Description: Remote Channel Management combines radio possible channels with allowed-channel policy loaded from system/platform/device metadata.
- Source-code evidence:
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:395` defines `UpdateAllowedChannelList`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:399` executes `GetUserAllowedChannels`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:405` filters radio channels by allowed-channel policy.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channels_allowed.go:38` defines `GetUserAllowedChannels`.
- `prisme-backend/services/self-healing/remote-channel-management/src/types/system_configuration.go:11` defines the 2.4 GHz allowed-channel property.
- `prisme-backend/services/self-healing/remote-channel-management/src/types/system_configuration.go:12` defines the 5 GHz allowed-channel property.
- `prisme-backend/services/self-healing/remote-channel-management/src/types/system_configuration.go:13` defines the 6 GHz allowed-channel property.
- Notes: This is not solely a CPE data model requirement; it also depends on PRISME metadata configuration.

### `selfHealing.rcm.currentChannel`

- Name: RCM current channel read
- Category: Self-healing
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Remote Channel Management reads the current radio channel before deciding whether to switch.
- Source-code evidence:
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:526` executes `GetCurrentChannel`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:534` aborts switching when current channel is invalid.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:14` defines `GetCurrentChannel`.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:31` builds the current-channel key from the radio object.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:33` reads current channel.
- Notes: Offline support requires parameter presence/value evidence, but runtime value correctness remains unproven.

### `selfHealing.rcm.channelWrite`

- Name: RCM channel write
- Category: Self-healing
- Concept type: `controlAction`
- Confidence: `high`
- Description: Remote Channel Management disables auto-channel mode and writes the selected channel to the target radio.
- Source-code evidence:
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:620` prepares channel switch update.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:626` executes `SetChannel`.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:43` defines `SetChannel`.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:61` builds auto-channel control key.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:62` writes auto-channel disabled.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:67` builds channel key.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/channel.go:68` writes selected channel.
- Notes: This concept requires write/control capability. Missing write support means observe/recommend may still be possible, but automatic channel switch is not.

### `selfHealing.rcm.scanDiagnostics`

- Name: RCM scan diagnostics
- Category: Self-healing
- Concept type: `diagnosticAction`
- Confidence: `high`
- Description: Remote Channel Management executes neighboring Wi-Fi scan diagnostics, ACS diagnostics/result reads, and pre-CAC result reads.
- Source-code evidence:
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:280` executes repeated `ScanWifi` in run-once mode.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:291` executes `ScanWifi`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:299` executes `ScanWifiAcs`.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:305` executes `PreCacResult`.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/scan_wifi.go:202` defines `ScanWifi`.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/scan_wifi.go:229` requests the neighboring Wi-Fi diagnostic.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/scan_wifi.go:243` polls diagnostic state.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/scan_wifi_acs.go:18` defines ACS scan-result activity.
- `prisme-backend/services/self-healing/remote-channel-management/src/activities/pre_cac.go:33` reads pre-CAC diagnostic results.
- Notes: Static snapshots cannot prove that diagnostics execute, complete, or return useful results.

### `selfHealing.rcm.channelScoring`

- Name: RCM channel scoring
- Category: Self-healing
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Remote Channel Management scores candidate channels using normalized RSSI/proximity, ACS noise, transmit power, blocked-by-noise, and utilization inputs.
- Source-code evidence:
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:555` normalizes RSSI scan results.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:556` normalizes ACS noise results.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:557` normalizes transmit power.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:558` calculates blocked-by-noise percentage.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:559` normalizes utilization.
- `prisme-backend/services/self-healing/remote-channel-management/src/workflow.go:561` selects best channel.
- `prisme-backend/services/self-healing/remote-channel-management/src/utils/algorithm.go:50` defines `GetBestChannel`.
- `prisme-backend/services/self-healing/remote-channel-management/src/utils/algorithm.go:64` sums normalized sub-scores.
- `prisme-backend/services/self-healing/remote-channel-management/src/utils/algorithm.go:87` avoids switching when the current channel score is effectively equal.
- Notes: This is a derived algorithm concept, not a raw path mapping.

### `selfHealing.qos.associatedDevices`

- Name: AQoS associated devices
- Category: Self-healing
- Concept type: `parameterValue`
- Confidence: `high`
- Description: AQoS workflows read associated devices, MAC addresses, SSID/AP/radio relationships, and radio band grouping.
- Source-code evidence:
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:21` defines `GetAssociatedDevices`.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:32` reads associated devices.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:39` defines `GetAssociatedDevicesByRadio`.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:50` reads radio objects.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:79` reads SSID objects.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:94` reads access point objects.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:114` reads associated device objects.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/devices.go:135` reads associated-device MAC addresses.
- Notes: This concept covers topology/association information needed by AQoS, not the customer-care topology feature.

### `selfHealing.qos.scoreTelemetry`

- Name: AQoS score telemetry
- Category: Self-healing
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: AQoS workflows read rolling device/host score series from OpenSearch, including host Wi-Fi traffic, coverage, RSSI, and CPE Wi-Fi network interference.
- Source-code evidence:
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:83` reads rolling host Wi-Fi traffic score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:111` reads rolling host Wi-Fi coverage score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:365` reads rolling CPE Wi-Fi network interference score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:576` reads rolling host Wi-Fi RSSI score.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/scores.go:87` defines rolling device score retrieval.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/scores.go:114` queries device score series.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/scores.go:167` defines rolling host score retrieval.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/scores.go:199` queries host score series.
- Notes: This is not directly validated from a single static data model snapshot unless score history is present in the input JSON.

### `selfHealing.qos.hostTrafficTelemetry`

- Name: AQoS host traffic and packet telemetry
- Category: Self-healing
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: AQoS workflows read host Wi-Fi traffic and packet/error activity from OpenSearch over recent time windows.
- Source-code evidence:
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:151` queries host traffic for importance ordering.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:523` queries host traffic for ATF ordering.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:512` queries host traffic during collision metrics collection.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:524` queries host packets during collision metrics collection.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/host_traffic.go:261` defines host traffic retrieval.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/host_traffic.go:274` calls `GetHostWifiTrafficActivity`.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/host_traffic.go:277` defines host packet retrieval.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/host_traffic.go:294` calls `GetHostWifiPacketsActivity`.
- Notes: Offline static input cannot prove time-window completeness unless historical telemetry is included.

### `selfHealing.qos.priorityControl`

- Name: AQoS DSCP/WMM prioritization control
- Category: Self-healing
- Concept type: `controlAction`
- Confidence: `high`
- Description: Dynamic prioritization enables WMM on affected APs and creates QoS classification rules for selected client IPs.
- Source-code evidence:
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/device_prioritization.go:262` executes `PrioritizeTraffic`.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:28` defines `PrioritizeTraffic`.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:46` deletes existing PRISME QoS classification rules.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:67` reads host by MAC address.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:80` enables WMM on the touched AP.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:87` builds QoS classification objects.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/prioritize.go:98` adds QoS classification objects.
- Notes: Automatic support requires add/delete/write capability for QoS classification and write capability for WMM.

### `selfHealing.qos.atfControl`

- Name: AQoS airtime fairness control
- Category: Self-healing
- Concept type: `controlAction`
- Confidence: `high`
- Description: Airtime fairness tuning enables ATF, sets global ATF defaults, and writes per-station ATF values.
- Source-code evidence:
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:288` applies legacy-client ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:402` applies greedy-client ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/airtime_fairness.go:426` resets greedy-client ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:24` defines `SetAtf`.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:37` enables ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:41` sets global ATF.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:44` builds the per-station ATF key.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/atf.go:45` writes per-station ATF.
- Notes: This is a control/write concept; missing write capability prevents automatic ATF tuning.

### `selfHealing.qos.collisionMetrics`

- Name: AQoS collision tuning metrics
- Category: Self-healing
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Collision tuning derives per-radio collision metrics from associated clients, host RSSI score, recent traffic, packet counts, and received error rate.
- Source-code evidence:
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:482` starts radio metrics collection.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:502` checks host RSSI score.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:512` reads host traffic.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:524` reads host packet activity.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:533` calculates received error rate.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:556` stores suspicious station count.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:558` stores worst error rate.
- Notes: This is derived from score and telemetry histories and cannot be fully proven by path presence alone.

### `selfHealing.qos.rtsCtsControl`

- Name: AQoS RTS/CTS collision control
- Category: Self-healing
- Concept type: `controlAction`
- Confidence: `high`
- Description: Collision tuning writes RTS threshold and retry limits to radio objects after workflow state transitions.
- Source-code evidence:
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:209` defines RTS/CTS threshold update wrapper.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:219` executes `CollisionTuning`.
- `prisme-backend/services/self-healing/quality-of-service/src/workflows/collision_tuning.go:284` processes radio state.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:12` defines collision tuning activity.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:19` builds the radio update object.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:23` writes RTS threshold.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:24` writes retry limit.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:25` writes long retry limit.
- `prisme-backend/services/self-healing/quality-of-service/src/activities/collision.go:30` sends the write request.
- Notes: Missing radio write support prevents automatic collision tuning.

## Customer-Care and Diagnostics Concepts

### `customerCare.wifi.fullTreeRead`

- Name: Customer-care Wi-Fi full tree read
- Category: Customer care
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Customer-care Wi-Fi suite settings initialize an internal Wi-Fi context from the endpoint data model and split the result into radio, access point, SSID, security, and Data Elements SSID object groups.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:33` registers legacy Wi-Fi settings routes.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:35` registers Wi-Fi suite settings read route.
- `prisme-backend/services/customer-care-agent/src/services/wifi.go:11` delegates legacy Wi-Fi settings read to management.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:382` starts Wi-Fi suite access point and radio discovery.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:74` initializes the Wi-Fi suite API context.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:57` defines the Wi-Fi suite API context initialization.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:84` reads the full Wi-Fi object tree.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:98` partitions the tree into typed object maps.
- Notes: This is a direct data model dependency. The concept intentionally stops at source-confirmed object groups and does not define the final raw path mapping table.

### `customerCare.wifi.radioCapabilities`

- Name: Customer-care Wi-Fi radio capability view
- Category: Customer care
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Wi-Fi suite settings expose radio status, standards, possible channels, supported bandwidths, and fallback defaults used when capability values are absent.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:255` reads radio status and operating standards.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:280` resolves possible channels.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:287` falls back to default channels when possible channels are empty.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:305` resolves supported bandwidths.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:312` falls back to default bandwidths when supported bandwidths are empty.
- Notes: The fallback defaults mean missing channel/bandwidth capability values may degrade precision rather than always blocking display, but write safety still depends on runtime acceptance.

### `customerCare.wifi.apSsidSecuritySettings`

- Name: Customer-care Wi-Fi AP, SSID, and security settings
- Category: Customer care
- Concept type: `parameterValue`
- Confidence: `high`
- Description: Wi-Fi suite settings derive customer-visible access point, SSID, advertisement, isolation, status, security mode, and MLO fields from the Wi-Fi object groups.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:96` builds Wi-Fi suite settings output from access point groups and radios.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:350` reads SSID values and references.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:364` reads access point enable and advertisement state.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:382` reads isolation and access point status.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:396` maps access point security mode.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:815` generates SSID enable, name, and MLO commands for changed settings.
- Notes: This concept is both a read/display dependency and, when paired with write support, part of customer-care configuration management.

### `customerCare.wifi.configurationWrite`

- Name: Customer-care Wi-Fi configuration write
- Category: Customer care
- Concept type: `controlAction`
- Confidence: `high`
- Description: Wi-Fi suite updates compare requested settings with current device state, build only changed AP/radio/security/SSID commands, and commit them through the existing management path.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:36` registers Wi-Fi suite settings update route.
- `prisme-backend/services/customer-care-agent/src/services/wifi.go:38` delegates Wi-Fi suite updates to management.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:441` starts command construction for changed Wi-Fi settings.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:451` builds access point update commands.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:457` builds radio update commands.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite/wifi.go:465` commits generated Wi-Fi configuration.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:147` documents endpoint-specific commit behavior.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:153` executes configuration through the campaign executor for one endpoint type.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:175` connects the data model client for the other endpoint type.
- `prisme-backend/services/customer-care-agent/src/management/wifi_suite_api/api.go:180` writes generated data model objects.
- Notes: Offline support must distinguish read-only Wi-Fi settings from automatic write capability.

### `customerCare.topology.deviceServiceMap`

- Name: Customer-care topology map from device service
- Category: Customer care
- Concept type: `runtimeOnly`
- Confidence: `high`
- Description: Customer-care topology map endpoints resolve the device endpoint and proxy the current map or flat-map response from device-service status.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:52` registers structured map route.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:54` registers flat map route.
- `prisme-backend/services/customer-care-agent/src/rest/map.go:11` defines the map response handler.
- `prisme-backend/services/customer-care-agent/src/rest/map.go:28` calls the customer-care map service.
- `prisme-backend/services/customer-care-agent/src/services/map.go:12` defines `GetCpeMap`.
- `prisme-backend/services/customer-care-agent/src/services/map.go:21` resolves endpoint identity.
- `prisme-backend/services/customer-care-agent/src/services/map.go:27` calls device-service status response.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/mapApi.ts:7` exposes the customer-care map query in the UI.
- Notes: This is an indirect API/runtime dependency, not a direct raw snapshot requirement unless the same topology payload is included in analyzer input.

### `diagnostics.speedtest.dispatch`

- Name: Customer-care speedtest dispatch
- Category: Diagnostics
- Concept type: `diagnosticAction`
- Confidence: `high`
- Description: Customer-care speedtest resolves endpoint metadata and dispatches either through the QoE-agent path or through the generic diagnostics request path.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:32` registers speedtest command route.
- `prisme-backend/services/customer-care-agent/src/rest/speedtest.go:9` defines the speedtest REST handler.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:11` defines the service speedtest command.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:20` resolves endpoint identity.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:26` branches on QoE-agent availability metadata.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:32` invokes the QoE-agent speedtest path.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:38` invokes the diagnostics request path.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/managementApi.ts:127` exposes the speedtest mutation in the UI.
- Notes: This is an action concept. A static snapshot can show related diagnostics capabilities only if the input contains enough metadata; it cannot prove that runtime execution will succeed.

### `diagnostics.speedtest.qoeAgentAction`

- Name: QoE-agent speedtest action
- Category: Diagnostics
- Concept type: `diagnosticAction`
- Confidence: `high`
- Description: When endpoint metadata indicates QoE-agent availability, customer-care starts an immediate performance test through the test API proxy using endpoint and VPN information.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:52` obtains VPN IP address for the QoE-agent path.
- `prisme-backend/services/customer-care-agent/src/services/speedtest.go:32` calls the QoE-agent speedtest service.
- `prisme-backend/services/customer-care-agent/src/speedtest/speedtest.go:23` defines the QoE-agent speedtest request.
- `prisme-backend/services/customer-care-agent/src/speedtest/speedtest.go:29` prepares the test API proxy request.
- `prisme-backend/services/customer-care-agent/src/speedtest/speedtest.go:42` adds the destination IP header.
- `prisme-backend/services/customer-care-agent/src/speedtest/speedtest.go:46` sends the request.
- Notes: This path depends on runtime metadata, VPN reachability, and test-api availability, not just CPE data model paths.

### `diagnostics.speedtest.diagnosticsRequest`

- Name: Generic diagnostics speedtest request
- Category: Diagnostics
- Concept type: `diagnosticAction`
- Confidence: `high`
- Description: When the QoE-agent path is unavailable, customer-care builds a speedtest diagnostic request and publishes it to the diagnostics request subject.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go:89` starts construction of the diagnostic request.
- `prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go:97` assigns device identity.
- `prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go:104` attaches QoE speedtest configuration.
- `prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go:107` sends the diagnostic request.
- `prisme-backend/services/customer-care-agent/src/diagnostics/diagnostics.go:180` configures the diagnostics request subject.
- `prisme-backend/services/customer-care-agent/src/diagnostics/diagnostics.go:216` marshals the diagnostic request.
- `prisme-backend/services/customer-care-agent/src/speedtest/diagnostics.go:109` publishes the diagnostic request.
- Notes: This path is runtime action dispatch. Offline analysis can report required action support and configuration evidence but cannot execute it.

### `customerCare.scoreDrilldown.scoreConsumption`

- Name: Customer-care score drill-down consumption
- Category: Customer care
- Concept type: `derivedMetric`
- Confidence: `high`
- Description: Customer-care score drill-down endpoints consume already-calculated device, host, host-list, and time-series scores from the score service and expose them for troubleshooting.
- Source-code evidence:
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:64` registers CPE score route.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:67` registers host score route.
- `prisme-backend/services/customer-care-agent/src/rest/handler.go:69` registers host-list score route.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:12` defines device score handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:30` defines device score series handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:62` defines host score handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:86` defines host score series handler.
- `prisme-backend/services/customer-care-agent/src/rest/scores.go:124` defines host-list score handler.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:16` retrieves device score.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:53` retrieves device score series.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:92` retrieves host score.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:115` retrieves host score series.
- `prisme-backend/services/customer-care-agent/src/services/scores.go:141` retrieves host-list scores.
- `prisme-ui/apps/customer-care-dashboard/src/api/endpoints/hostApi.ts:38` exposes host-list scores in the UI.
- Notes: This is not a direct data model requirement for the analyzer. It depends on prior score calculation and score storage availability.

## Deferred Concept Areas

The following concept group is not covered yet and will be added in a later milestone:

- Raw input JSON parsing concepts.
