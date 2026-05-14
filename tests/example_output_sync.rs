use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn example_output_report_is_in_sync_with_cli() {
    let output_path = Path::new("out/test-example-output-sync.txt");
    if output_path.exists() {
        fs::remove_file(output_path).unwrap();
    }

    let bin = std::env::var("CARGO_BIN_EXE_cpe_analyzer")
        .unwrap_or_else(|_| "target/debug/cpe-analyzer".to_string());
    let status = Command::new(&bin)
        .args([
            "--input",
            "docs/offline-analyzer/examples/input-basic.json",
            "--output",
            "out/test-example-output-sync.txt",
            "--color=never",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let actual = fs::read_to_string(output_path).unwrap();
    let expected =
        fs::read_to_string("docs/offline-analyzer/examples/output-compatibility-report.txt")
            .unwrap();
    assert_eq!(actual, expected);
}
