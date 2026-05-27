#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConceptEvidenceRole {
    Native,
    Alternate,
    Derived,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConceptDefinition {
    pub concept_id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub bindings: Vec<ConceptPathBinding>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConceptPathBinding {
    pub pattern: &'static str,
    pub role: ConceptEvidenceRole,
}

pub fn canonical_concepts() -> Vec<ConceptDefinition> {
    vec![
        concept(
            "ap.security.macFilterControl",
            "AP MAC filter control",
            "WiFi",
            vec![
                binding(
                    "Device.WiFi.AccessPoint.{i}.MACAddressControlEnabled",
                    ConceptEvidenceRole::Native,
                ),
                binding(
                    "Device.WiFi.AccessPoint.{i}.AllowedMACAddress",
                    ConceptEvidenceRole::Native,
                ),
            ],
        ),
        concept(
            "host.identity.mac",
            "Host MAC identity",
            "Host",
            vec![
                binding(
                    "Device.WiFi.AccessPoint.{i}.AssociatedDevice.{i}.MACAddress",
                    ConceptEvidenceRole::Native,
                ),
                binding(
                    "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.AssociatedDevice.{i}.AssociatedDeviceMACAddress",
                    ConceptEvidenceRole::Native,
                ),
                binding(
                    "Device.Hosts.Host.{i}.PhysAddress",
                    ConceptEvidenceRole::Alternate,
                ),
            ],
        ),
        concept(
            "wifi.radio.channelBandwidth",
            "Wi-Fi radio channel bandwidth",
            "WiFi",
            vec![
                binding(
                    "Device.WiFi.Radio.{i}.CurrentOperatingChannelBandwidth",
                    ConceptEvidenceRole::Native,
                ),
                binding(
                    "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.CurrentOperatingClassProfile.{i}.Channel",
                    ConceptEvidenceRole::Alternate,
                ),
            ],
        ),
        concept(
            "wifi.radio.currentChannel",
            "Wi-Fi radio current channel",
            "WiFi",
            vec![
                binding("Device.WiFi.Radio.{i}.Channel", ConceptEvidenceRole::Native),
                binding(
                    "InternetGatewayDevice.LANDevice.{i}.WLANConfiguration.{i}.Channel",
                    ConceptEvidenceRole::Native,
                ),
            ],
        ),
        concept(
            "wifi.scan.neighborResults",
            "Wi-Fi neighbor scan results",
            "WiFi",
            vec![
                binding(
                    "Device.WiFi.NeighboringWiFiDiagnostic.Result.{i}.",
                    ConceptEvidenceRole::Native,
                ),
                binding(
                    "Device.WiFi.DataElements.Network.Device.{i}.Radio.{i}.ScanResult",
                    ConceptEvidenceRole::Alternate,
                ),
            ],
        ),
    ]
}

fn concept(
    concept_id: &'static str,
    name: &'static str,
    category: &'static str,
    bindings: Vec<ConceptPathBinding>,
) -> ConceptDefinition {
    ConceptDefinition {
        concept_id,
        name,
        category,
        bindings,
    }
}

fn binding(pattern: &'static str, role: ConceptEvidenceRole) -> ConceptPathBinding {
    ConceptPathBinding { pattern, role }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concept_ids_are_deterministic_and_sorted() {
        let ids: Vec<_> = canonical_concepts()
            .into_iter()
            .map(|concept| concept.concept_id)
            .collect();
        let mut sorted = ids.clone();
        sorted.sort();
        assert_eq!(ids, sorted);
    }
}
