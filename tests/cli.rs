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
