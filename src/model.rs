use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct InputDocument {
    #[serde(rename = "Report")]
    pub report: Vec<ReportEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReportEntry {
    pub path: Option<String>,
    pub value: Option<Value>,
    pub kind: Option<String>,
    #[serde(rename = "type")]
    pub value_type: Option<String>,
    pub readable: Option<bool>,
    pub writable: Option<bool>,
    pub available: Option<bool>,
    pub commandable: Option<bool>,
    pub scope: Option<String>,
    #[serde(rename = "entityId")]
    pub entity_id: Option<String>,
    pub timestamp: Option<String>,
    pub window: Option<String>,
    pub source: Option<String>,
}

impl ReportEntry {
    pub fn kind_normalized(&self) -> &str {
        self.kind.as_deref().unwrap_or("parameter")
    }
    pub fn has_value(&self) -> bool {
        match &self.value {
            None | Some(Value::Null) => false,
            Some(Value::String(s)) => !s.trim().is_empty(),
            Some(_) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub entries: Vec<ReportEntry>,
    pub detected_data_model: String,
    pub device: DeviceInfo,
}

#[derive(Debug, Clone, Default)]
pub struct DeviceInfo {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
    pub declared_data_model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BaselineRepoVersion {
    pub commit_sha: String,
}

#[derive(Debug, Clone)]
pub struct PrismeBaseline {
    pub prisme_backend: BaselineRepoVersion,
    pub prisme_ui: BaselineRepoVersion,
    pub tss: BaselineRepoVersion,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum SupportStatus {
    Supported,
    Partial,
    Unsupported,
    Unknown,
    ImplementationNotFound,
}
impl SupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
            Self::ImplementationNotFound => "implementation_not_found",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EvidenceConfidence {
    High,
    Medium,
    Low,
}
impl EvidenceConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequirementGroup {
    Mandatory,
    Optional,
    Control,
    Diagnostic,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MatchMode {
    Present,
    Value,
    Readable,
    Writable,
    Diagnostic,
    History,
}

#[derive(Debug, Clone)]
pub struct SourceCodeReference {
    pub file: &'static str,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub symbol: &'static str,
    pub proves: &'static str,
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub rule_id: &'static str,
    pub concept_id: &'static str,
    pub label: &'static str,
    pub group: RequirementGroup,
    pub match_mode: MatchMode,
    pub candidates: Vec<&'static str>,
    pub impact: &'static str,
}

#[derive(Debug, Clone)]
pub struct FeatureDefinition {
    pub feature_id: &'static str,
    pub feature_name: &'static str,
    pub category: &'static str,
    pub confidence: EvidenceConfidence,
    pub runtime_validation_required: bool,
    pub control_required_for_support: bool,
    pub diagnostic_required_for_support: bool,
    pub implementation_found: bool,
    pub requirements: Vec<Requirement>,
    pub limitations: Vec<&'static str>,
    pub source_refs: Vec<SourceCodeReference>,
}

#[derive(Debug, Clone)]
pub struct RequirementMatch {
    pub rule_id: String,
    pub concept_id: String,
    pub label: String,
    pub matched_paths: Vec<String>,
    pub group: RequirementGroup,
}

#[derive(Debug, Clone)]
pub struct MissingRequirement {
    pub rule_id: String,
    pub concept_id: String,
    pub label: String,
    pub candidate_paths: Vec<String>,
    pub impact: String,
    pub group: RequirementGroup,
}

#[derive(Debug, Clone)]
pub struct FeatureAssessment {
    pub feature_id: String,
    pub feature_name: String,
    pub category: String,
    pub support: SupportStatus,
    pub evidence_confidence: EvidenceConfidence,
    pub runtime_validation_required: bool,
    pub matched_requirements: Vec<RequirementMatch>,
    pub missing_mandatory_requirements: Vec<MissingRequirement>,
    pub missing_optional_requirements: Vec<MissingRequirement>,
    pub missing_control_requirements: Vec<MissingRequirement>,
    pub missing_diagnostic_requirements: Vec<MissingRequirement>,
    pub limitations: Vec<String>,
    pub source_refs: Vec<SourceCodeReference>,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub report_format_version: u32,
    pub assessment_basis: String,
    pub device: DeviceInfo,
    pub detected_data_model: String,
    pub prisme_baseline: PrismeBaseline,
    pub features: Vec<FeatureAssessment>,
    pub summary: BTreeMap<String, usize>,
}
