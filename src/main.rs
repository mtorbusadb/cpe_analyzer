use clap::{ArgEnum, Parser};
use cpe_analyzer::evaluator::evaluate;
use cpe_analyzer::input::load_snapshot;
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
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let cli = Cli::parse();
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
    Ok(())
}
