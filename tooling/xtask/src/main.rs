mod tasks;
mod workspace;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo xtask")]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Runs `cargo clippy`.
    Clippy(tasks::clippy::ClippyArgs),
    Licenses(tasks::licenses::LicensesArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        CliCommand::Clippy(args) => tasks::clippy::run_clippy(args),
        CliCommand::Licenses(args) => tasks::licenses::run_licenses(args),
    }
}
