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
    /// Automatically apply lint suggestions (`clippy --fix`).
    #[arg(long)]
    fix: bool,

    /// The package to run Clippy against (`cargo -p <PACKAGE> clippy`).
    #[arg(long, short)]
    package: Option<String>,
}

fn run_clippy(args: ClippyArgs) -> Result<()> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let mut clippy_command = Command::new(&cargo);
    clippy_command.arg("clippy");

    if let Some(package) = args.package.as_ref() {
        clippy_command.args(["--package", package]);
    } else {
        clippy_command.arg("--workspace");
    }

    clippy_command
        .arg("--release")
        .arg("--all-targets")
        .arg("--all-features");

    if args.fix {
        clippy_command.arg("--fix");
    }

    clippy_command.arg("--");

    // Deny all warnings.
    clippy_command.args(["--deny", "warnings"]);

    eprintln!(
        "running: {cargo} {}",
        clippy_command
            .get_args()
            .map(|arg| arg.to_str().unwrap())
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
