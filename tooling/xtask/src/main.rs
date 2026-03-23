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
    /// Checks that packages conform to a set of standards.
    PackageConformity(tasks::package_conformity::PackageConformityArgs),
    /// Publishes GPUI and its dependencies to crates.io.
    PublishGpui(tasks::publish_gpui::PublishGpuiArgs),
    Workflows(tasks::workflows::GenerateWorkflowArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        CliCommand::Clippy(args) => tasks::clippy::run_clippy(args),
        CliCommand::Licenses(args) => tasks::licenses::run_licenses(args),
        CliCommand::PackageConformity(args) => {
            tasks::package_conformity::run_package_conformity(args)
        }
        CliCommand::PublishGpui(args) => tasks::publish_gpui::run_publish_gpui(args),
        CliCommand::Workflows(args) => tasks::workflows::run_workflows(args),
    }
}
