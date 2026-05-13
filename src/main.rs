use clap::{ArgEnum, Parser};
use cpe_analyzer::evaluator::evaluate;
use cpe_analyzer::input::load_snapshot;
use cpe_analyzer::model::SupportStatus;
use cpe_analyzer::report::{render_text_report, ColorMode};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "cpe-analyzer")]
struct Cli {
    #[clap(long)]
    input: PathBuf,
    #[clap(long)]
    output: PathBuf,
    #[clap(long, arg_enum, default_value = "auto")]
    color: ColorArg,
    #[clap(long)]
    no_color: bool,
    #[clap(long)]
    fail_on: Vec<String>,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, ArgEnum)]
enum ColorArg {
    Auto,
    Always,
    Never,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        if err.starts_with("fail-on matched status:") {
            std::process::exit(2);
        }
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let fail_on_statuses = parse_fail_on_statuses(&cli.fail_on)?;
    let snapshot = load_snapshot(&cli.input)?;
    let report = evaluate(&snapshot);
    let color_mode = if cli.no_color || cli.color != ColorArg::Always {
        ColorMode::Never
    } else {
        ColorMode::Always
    };
    let rendered = render_text_report(&report, color_mode);
    if let Some(parent) = cli.output.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "failed to create output directory {}: {err}",
                parent.display()
            )
        })?;
    }
    fs::write(&cli.output, rendered)
        .map_err(|err| format!("failed to write output {}: {err}", cli.output.display()))?;
    println!("wrote {}", cli.output.display());
    let failing_statuses: Vec<_> = fail_on_statuses
        .into_iter()
        .filter(|status| {
            report
                .features
                .iter()
                .any(|feature| feature.support == *status)
        })
        .collect();
    if !failing_statuses.is_empty() {
        let statuses = failing_statuses
            .iter()
            .map(|status| status.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!("fail-on matched status: {statuses}"));
    }
    Ok(())
}

fn parse_fail_on_statuses(values: &[String]) -> Result<Vec<SupportStatus>, String> {
    let mut statuses = Vec::new();
    for value in values {
        let status = match value.as_str() {
            "supported" => SupportStatus::Supported,
            "partial" => SupportStatus::Partial,
            "unsupported" => SupportStatus::Unsupported,
            "unknown" => SupportStatus::Unknown,
            "implementation_not_found" => SupportStatus::ImplementationNotFound,
            _ => {
                return Err(format!(
                    "invalid --fail-on status '{value}'; expected supported, partial, unsupported, unknown, or implementation_not_found"
                ));
            }
        };
        if !statuses.contains(&status) {
            statuses.push(status);
        }
    }
    Ok(statuses)
}
