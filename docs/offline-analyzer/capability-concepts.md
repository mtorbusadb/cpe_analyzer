# Capability Concepts

## Scope

This document defines canonical capability concept candidates for the Offline PRISME CPE Compatibility Analyzer.

Current coverage:

- Milestone 1: Discovery Program concepts only.
- Milestone 2: QoE score derived metric concepts from active score calculation code.

Later milestones will extend this file with self-healing, customer-care, topology, and diagnostics concepts.

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

## Deferred Concept Areas

The following concept groups are not covered yet and will be added in later milestones:

- Self-healing telemetry and control concepts.
- Customer-care Wi-Fi management concepts.
- Topology/map visibility concepts.
- Speedtest and diagnostics execution concepts outside the Discovery Program.
- Raw input JSON parsing concepts.
