use crate::path_classification::{
    classify_path_kind, classify_standard, normalize_path, PathKind, StandardClass,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceInventoryEntry {
    pub normalized_path: String,
    pub kind: PathKind,
    pub standard_class: StandardClass,
    pub references: Vec<SourceReference>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourceReference {
    pub repo: String,
    pub file: String,
    pub line: usize,
}

pub fn build_source_inventory(prisme_root: &Path) -> Result<Vec<SourceInventoryEntry>, String> {
    let mut grouped: BTreeMap<String, Vec<SourceReference>> = BTreeMap::new();
    for repo in ["prisme-backend", "prisme-ui", "tss"] {
        let repo_root = prisme_root.join(repo);
        if !repo_root.is_dir() {
            continue;
        }
        let files = collect_files(&repo_root)?;
        for file in files {
            scan_file(repo, &repo_root, &file, &mut grouped)?;
        }
    }

    Ok(grouped
        .into_iter()
        .map(|(normalized_path, mut references)| {
            references.sort();
            references.dedup();
            SourceInventoryEntry {
                kind: classify_path_kind(&normalized_path),
                standard_class: classify_standard(&normalized_path),
                normalized_path,
                references,
            }
        })
        .collect())
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(path) = pending.pop() {
        let entries = fs::read_dir(&path)
            .map_err(|err| format!("failed to read directory {}: {err}", path.display()))?;
        for entry in entries {
            let entry = entry.map_err(|err| format!("failed to read directory entry: {err}"))?;
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name == ".git" || name == "node_modules" || name == "target" {
                continue;
            }
            if path.is_dir() {
                pending.push(path);
            } else if is_supported_source_file(&path) {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn is_supported_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("go" | "py" | "js" | "ts" | "tsx" | "json" | "yaml" | "yml" | "xml" | "txt" | "md")
    )
}

fn scan_file(
    repo: &str,
    repo_root: &Path,
    file: &Path,
    grouped: &mut BTreeMap<String, Vec<SourceReference>>,
) -> Result<(), String> {
    let text = match fs::read_to_string(file) {
        Ok(text) => text,
        Err(_) => return Ok(()),
    };
    let rel = file
        .strip_prefix(repo_root)
        .map_err(|err| format!("failed to relativize {}: {err}", file.display()))?
        .to_string_lossy()
        .replace('\\', "/");
    for (line_idx, line) in text.lines().enumerate() {
        for raw in extract_path_candidates(line) {
            let normalized = normalize_path(&raw);
            grouped
                .entry(normalized)
                .or_default()
                .push(SourceReference {
                    repo: repo.to_string(),
                    file: rel.clone(),
                    line: line_idx + 1,
                });
        }
    }
    Ok(())
}

fn extract_path_candidates(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    for prefix in ["Device.", "InternetGatewayDevice."] {
        let mut search_from = 0usize;
        while let Some(offset) = line[search_from..].find(prefix) {
            let start = search_from + offset;
            let candidate = line[start..]
                .chars()
                .take_while(|ch| is_path_char(*ch))
                .collect::<String>()
                .trim_end_matches('.')
                .to_string();
            if candidate.len() > prefix.len() {
                out.push(candidate);
            }
            search_from = start + prefix.len();
        }
    }
    out
}

fn is_path_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '.' | '_' | '-' | '{' | '}' | '[' | ']' | '=' | '!' | '*' | ':'
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn builds_deterministic_inventory_from_repo_tree() {
        let root =
            std::env::temp_dir().join(format!("cpe-analyzer-inventory-{}", std::process::id()));
        let repo = root.join("tss");
        let nested = repo.join("src");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&nested).unwrap();
        fs::write(
            nested.join("metric.js"),
            "const p = 'Device.WiFi.Radio.1.X_ADB_ChannelSwitchReason';\n",
        )
        .unwrap();

        let inventory = build_source_inventory(&root).unwrap();
        let entry = inventory
            .iter()
            .find(|entry| {
                entry.normalized_path == "Device.WiFi.Radio.{i}.X_ADB_ChannelSwitchReason"
            })
            .unwrap();
        assert_eq!(entry.kind, PathKind::Pattern);
        assert_eq!(entry.standard_class, StandardClass::VendorExtension);
        assert_eq!(entry.references[0].repo, "tss");
        assert_eq!(entry.references[0].file, "src/metric.js");

        let _ = fs::remove_dir_all(&root);
    }
}
