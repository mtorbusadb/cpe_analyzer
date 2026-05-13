use std::fs;
use std::process::Command;

#[test]
fn cli_writes_plain_text_report() {
    let exe = env!("CARGO_BIN_EXE_cpe-analyzer");
    let output_path = "out/test-cli-report.txt";
    let status = Command::new(exe)
        .args([
            "--input",
            "tests/fixtures/offline-analyzer/read-only-control.json",
            "--output",
            output_path,
            "--color",
            "never",
        ])
        .status()
        .expect("runs cpe-analyzer");
    assert!(status.success());
    let text = fs::read_to_string(output_path).expect("report is written");
    assert!(text.contains("PRISME Offline CPE Compatibility Report"));
    assert!(text.contains("Missing for support"));
    assert!(!text.contains('\u{1b}'));
}

#[test]
fn cli_writes_colored_statuses_when_requested() {
    let exe = env!("CARGO_BIN_EXE_cpe-analyzer");
    let output_path = "out/test-cli-report-color.txt";
    let status = Command::new(exe)
        .args([
            "--input",
            "tests/fixtures/offline-analyzer/read-only-control.json",
            "--output",
            output_path,
            "--color",
            "always",
        ])
        .status()
        .expect("runs cpe-analyzer");
    assert!(status.success());
    let text = fs::read_to_string(output_path).expect("report is written");
    assert!(text.contains("\u{1b}[31munsupported\u{1b}[0m"));
}

#[test]
fn cli_fail_on_writes_report_then_exits_non_zero_when_status_matches() {
    let exe = env!("CARGO_BIN_EXE_cpe-analyzer");
    let output_path = "out/test-cli-fail-on-report.txt";
    let output = Command::new(exe)
        .args([
            "--input",
            "tests/fixtures/offline-analyzer/read-only-control.json",
            "--output",
            output_path,
            "--color",
            "never",
            "--fail-on",
            "unsupported",
        ])
        .output()
        .expect("runs cpe-analyzer");
    assert_eq!(output.status.code(), Some(2));
    let text = fs::read_to_string(output_path).expect("report is written before failing");
    assert!(text.contains("selfHealing.remoteChannelManagement"));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("fail-on matched status: unsupported"));
}

#[test]
fn cli_fail_on_can_be_repeated_for_multiple_exact_statuses() {
    let exe = env!("CARGO_BIN_EXE_cpe-analyzer");
    let output_path = "out/test-cli-fail-on-multiple-report.txt";
    let output = Command::new(exe)
        .args([
            "--input",
            "tests/fixtures/offline-analyzer/basic-supported.json",
            "--output",
            output_path,
            "--color",
            "never",
            "--fail-on",
            "unknown",
            "--fail-on",
            "implementation_not_found",
        ])
        .output()
        .expect("runs cpe-analyzer");
    assert!(output.status.success());
    assert!(fs::read_to_string(output_path)
        .unwrap()
        .contains("reportFormatVersion: 1"));
}

#[test]
fn cli_rejects_invalid_fail_on_status() {
    let exe = env!("CARGO_BIN_EXE_cpe-analyzer");
    let output = Command::new(exe)
        .args([
            "--input",
            "tests/fixtures/offline-analyzer/basic-supported.json",
            "--output",
            "out/test-cli-invalid-fail-on-report.txt",
            "--fail-on",
            "implementation-not-found",
        ])
        .output()
        .expect("runs cpe-analyzer");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid --fail-on status 'implementation-not-found'"));
}
