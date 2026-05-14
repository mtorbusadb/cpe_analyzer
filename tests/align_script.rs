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
            "--generated-at",
            "2026-05-14 00:00:00 UTC",
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

#[test]
fn align_script_can_be_made_deterministic_with_generated_at_override() {
    let a_rel = "out/test-align-drift-a.md";
    let b_rel = "out/test-align-drift-b.md";
    let a_path = Path::new(a_rel);
    let b_path = Path::new(b_rel);
    if a_path.exists() {
        fs::remove_file(a_path).unwrap();
    }
    if b_path.exists() {
        fs::remove_file(b_path).unwrap();
    }

    for out in [a_rel, b_rel] {
        let status = Command::new("python3")
            .args([
                "scripts/align.py",
                "--prisme-root",
                "/tmp/prisme-root-does-not-exist",
                "--output",
                out,
                "--generated-at",
                "2026-05-14 00:00:00 UTC",
            ])
            .status()
            .unwrap();
        assert!(status.success());
    }

    let a = fs::read_to_string(a_path).unwrap();
    let b = fs::read_to_string(b_path).unwrap();
    assert_eq!(a, b);
}
