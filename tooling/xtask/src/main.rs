use std::process::Command;

use anyhow::{bail, Context, Result};
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
    Clippy(ClippyArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        CliCommand::Clippy(args) => run_clippy(args),
    }
}

#[derive(Parser)]
struct ClippyArgs {
    /// Whether to deny warnings (`clippy --deny warnings`).
    #[arg(long)]
    deny_warnings: bool,
}

fn run_clippy(args: ClippyArgs) -> Result<()> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let mut clippy_command = Command::new(&cargo);
    clippy_command
        .arg("clippy")
        .arg("--workspace")
        .arg("--release")
        .arg("--all-targets")
        .arg("--all-features");

    clippy_command.arg("--");

    if args.deny_warnings {
        clippy_command.args(["--deny", "warnings"]);
    }

    // Allow all Clippy lints by default, as we have a lot of violations at the moment.
    // We can tighten things up once we have a better handle on them.
    clippy_command.args(["--allow", "clippy::all"]);

    // Deny `dbg!` and `todo!`s.
    clippy_command
        .args(["--deny", "clippy::dbg_macro"])
        .args(["--deny", "clippy::todo"]);

    eprintln!(
        "running: {cargo} {}",
        clippy_command
            .get_args()
            .map(|arg| format!("{}", arg.to_str().unwrap()))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let exit_status = clippy_command
        .spawn()
        .context("failed to spawn child process")?
        .wait()
        .context("failed to wait for child process")?;

    if !exit_status.success() {
        bail!("clippy failed: {}", exit_status);
    }

    Ok(())
}
