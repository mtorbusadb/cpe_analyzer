use crate::concepts::{ConceptDefinition, ConceptEvidenceRole};
use crate::source_inventory::{SourceInventoryEntry, SourceReference};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConceptEvidence {
    pub concept_id: String,
    pub role: ConceptEvidenceRole,
    pub matched_path: String,
    pub references: Vec<SourceReference>,
}

pub fn bind_inventory_to_concepts(
    inventory: &[SourceInventoryEntry],
    concepts: &[ConceptDefinition],
) -> Vec<ConceptEvidence> {
    let mut evidence = Vec::new();
    for concept in concepts {
        for binding in &concept.bindings {
            for entry in inventory {
                if path_matches(binding.pattern, &entry.normalized_path) {
                    evidence.push(ConceptEvidence {
                        concept_id: concept.concept_id.to_string(),
                        role: binding.role,
                        matched_path: entry.normalized_path.clone(),
                        references: entry.references.clone(),
                    });
                }
            }
        }
    }
    evidence.sort_by(|a, b| {
        a.concept_id
            .cmp(&b.concept_id)
            .then(a.role.cmp(&b.role))
            .then(a.matched_path.cmp(&b.matched_path))
    });
    evidence.dedup_by(|a, b| {
        a.concept_id == b.concept_id && a.role == b.role && a.matched_path == b.matched_path
    });
    evidence
}

fn path_matches(pattern: &str, path: &str) -> bool {
    if pattern.ends_with('.') {
        return path.starts_with(pattern);
    }
    if pattern.contains("{i}") {
        return pattern_segments_match(pattern, path);
    }
    path == pattern || path.starts_with(pattern)
}

fn pattern_segments_match(pattern: &str, path: &str) -> bool {
    let pp: Vec<_> = pattern.trim_end_matches('.').split('.').collect();
    let sp: Vec<_> = path.trim_end_matches('.').split('.').collect();
    pp.len() <= sp.len() && pp.iter().zip(sp.iter()).all(|(a, b)| *a == "{i}" || a == b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::canonical_concepts;
    use crate::path_classification::{PathKind, StandardClass};

    #[test]
    fn binds_multiple_paths_to_one_concept_with_roles() {
        let inventory = vec![
            entry("Device.WiFi.Radio.{i}.CurrentOperatingChannelBandwidth"),
            entry("Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.CurrentOperatingClassProfile.{i}.Channel"),
        ];
        let evidence = bind_inventory_to_concepts(&inventory, &canonical_concepts());
        let bandwidth: Vec<_> = evidence
            .iter()
            .filter(|item| item.concept_id == "wifi.radio.channelBandwidth")
            .collect();
        assert_eq!(bandwidth.len(), 2);
        assert!(bandwidth
            .iter()
            .any(|item| item.role == ConceptEvidenceRole::Native));
        assert!(bandwidth
            .iter()
            .any(|item| item.role == ConceptEvidenceRole::Alternate));
    }

    fn entry(path: &str) -> SourceInventoryEntry {
        SourceInventoryEntry {
            normalized_path: path.to_string(),
            kind: PathKind::Pattern,
            standard_class: StandardClass::Tr181Standard,
            references: vec![SourceReference {
                repo: "tss".to_string(),
                file: "metric.js".to_string(),
                line: 1,
            }],
        }
    }
}
