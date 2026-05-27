#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum PathKind {
    Exact,
    Pattern,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum StandardClass {
    Tr181Standard,
    Tr098Standard,
    VendorExtension,
    NonStandard,
    Unknown,
}

pub fn normalize_path(path: &str) -> String {
    let mut normalized = String::new();
    let mut segment = String::new();
    for ch in path.trim().chars() {
        match ch {
            '.' => {
                push_normalized_segment(&mut normalized, &segment);
                normalized.push('.');
                segment.clear();
            }
            _ => segment.push(ch),
        }
    }
    push_normalized_segment(&mut normalized, &segment);
    normalized
}

pub fn classify_path_kind(normalized_path: &str) -> PathKind {
    if normalized_path.contains("{i}")
        || normalized_path.contains('*')
        || normalized_path.contains('[')
    {
        PathKind::Pattern
    } else {
        PathKind::Exact
    }
}

pub fn classify_standard(normalized_path: &str) -> StandardClass {
    if normalized_path
        .split('.')
        .any(|segment| segment.starts_with("X_"))
    {
        return StandardClass::VendorExtension;
    }
    if normalized_path.starts_with("Device.") {
        return StandardClass::Tr181Standard;
    }
    if normalized_path.starts_with("InternetGatewayDevice.") {
        return StandardClass::Tr098Standard;
    }
    if normalized_path.trim().is_empty() {
        StandardClass::Unknown
    } else {
        StandardClass::NonStandard
    }
}

fn push_normalized_segment(out: &mut String, segment: &str) {
    if segment.is_empty() {
        return;
    }
    if segment == "*" || segment.chars().all(|ch| ch.is_ascii_digit()) {
        out.push_str("{i}");
    } else {
        out.push_str(segment);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_numeric_instances_to_wildcards() {
        assert_eq!(
            normalize_path("Device.WiFi.Radio.12.Channel"),
            "Device.WiFi.Radio.{i}.Channel"
        );
    }

    #[test]
    fn classifies_exact_pattern_and_vendor_extension() {
        let standard = normalize_path("Device.WiFi.Radio.1.Channel");
        assert_eq!(classify_path_kind(&standard), PathKind::Pattern);
        assert_eq!(classify_standard(&standard), StandardClass::Tr181Standard);

        let vendor = normalize_path("Device.WiFi.Radio.1.X_ADB_ChannelSwitchReason");
        assert_eq!(classify_standard(&vendor), StandardClass::VendorExtension);
    }
}
