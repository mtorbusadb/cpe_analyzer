use crate::model::{DeviceInfo, InputDocument, ReportEntry, Snapshot};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn load_snapshot(path: &Path) -> Result<Snapshot, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read input {}: {err}", path.display()))?;
    let document: InputDocument = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse JSON input {}: {err}", path.display()))?;
    Ok(build_snapshot(document.report))
}

fn build_snapshot(entries: Vec<ReportEntry>) -> Snapshot {
    let detected_data_model = detect_data_model(&entries);
    let device = extract_device_info(&entries);
    Snapshot {
        entries,
        detected_data_model,
        device,
    }
}

fn detect_data_model(entries: &[ReportEntry]) -> String {
    if entries
        .iter()
        .any(|e| e.path.as_deref().is_some_and(|p| p.starts_with("Device.")))
    {
        "TR-181".to_string()
    } else if entries.iter().any(|e| {
        e.path
            .as_deref()
            .is_some_and(|p| p.starts_with("InternetGatewayDevice."))
    }) {
        "TR-098".to_string()
    } else {
        "unknown".to_string()
    }
}

fn extract_device_info(entries: &[ReportEntry]) -> DeviceInfo {
    DeviceInfo {
        vendor: metadata_value(entries, "device.vendor"),
        model: metadata_value(entries, "device.model"),
        firmware_version: metadata_value(entries, "device.firmwareVersion"),
        declared_data_model: metadata_value(entries, "device.declaredDataModel"),
    }
}

fn metadata_value(entries: &[ReportEntry], key: &str) -> Option<String> {
    entries
        .iter()
        .find(|e| e.path.as_deref() == Some(key))
        .and_then(|e| value_to_string(e.value.as_ref()))
}
fn value_to_string(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}
