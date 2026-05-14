use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn align_script_generates_report_with_missing_prisme_root() {
    let output_rel = "out/test-align-drift-report.md";
    let output_path = Path::new(output_rel);
    if output_path.exists() {
        fs::remove_file(output_path).unwrap();
    }

    let status = Command::new("python3")
        .args([
            "scripts/align.py",
            "--prisme-root",
            "/tmp/prisme-root-does-not-exist",
            "--output",
            output_rel,
        ])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(output_path.exists());

    let text = fs::read_to_string(output_path).unwrap();
    assert!(text.contains("# Align Drift Report"));
    assert!(text.contains("Mode: read-only PRISME repo scan"));
    assert!(text.contains("## Baseline vs current repository versions"));
    assert!(text.contains("## Evidence-path integrity scan"));
    assert!(text.contains("missing referenced files"));
}
