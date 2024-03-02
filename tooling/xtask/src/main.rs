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

    /// Whether to deny warnings (`clippy --deny warnings`).
    #[arg(long)]
    deny_warnings: bool,
}

fn run_clippy(args: ClippyArgs) -> Result<()> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let mut clippy_command = Command::new(&cargo);
    clippy_command.arg("clippy");

    if let Some(package) = args.package {
        clippy_command.args(["--package", &package]);
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

    if args.deny_warnings {
        clippy_command.args(["--deny", "warnings"]);
    }

    const MIGRATORY_LINTS_TO_ALLOW: &[&str] = &[
        // There's a bunch of rules currently failing in the `style` group, so
        // allow all of those, for now.
        "clippy::style",
        // Individual rules that have violations in the codebase:
        "clippy::almost_complete_range",
        "clippy::arc_with_non_send_sync",
        "clippy::bool_comparison",
        "clippy::borrowed_box",
        "clippy::cast_abs_to_unsigned",
        "clippy::clone_on_copy",
        "clippy::eq_op",
        "clippy::explicit_auto_deref",
        "clippy::extra_unused_lifetimes",
        "clippy::iter_overeager_cloned",
        "clippy::map_entry",
        "clippy::map_identity",
        "clippy::needless_lifetimes",
        "clippy::non_canonical_clone_impl",
        "clippy::option_map_unit_fn",
        "clippy::redundant_locals",
        "clippy::reversed_empty_ranges",
        "clippy::search_is_some",
        "clippy::single_char_pattern",
        "clippy::single_range_in_vec_init",
        "clippy::too_many_arguments",
        "clippy::type_complexity",
        "clippy::unit_arg",
        "clippy::unnecessary_cast",
        "clippy::unnecessary_unwrap",
        "clippy::useless_conversion",
    ];

    if !args.fix {
        for rule in MIGRATORY_LINTS_TO_ALLOW {
            clippy_command.args(["--allow", rule]);
        }
    }

    // Allow all Clippy lints by default, as we have a lot of violations at the moment.
    // We can tighten things up once we have a better handle on them.
    // clippy_command.args(["--allow", "clippy::all"]);

    // Deny `dbg!` and `todo!`s.
    clippy_command
        .args(["--deny", "clippy::dbg_macro"])
        .args(["--deny", "clippy::todo"]);

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
