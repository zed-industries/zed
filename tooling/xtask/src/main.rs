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
    // We don't do this yet on Windows, as it still has some warnings present.
    #[cfg(not(target_os = "windows"))]
    clippy_command.args(["--deny", "warnings"]);

    /// These are all of the rules that currently have violations in the Zed
    /// codebase.
    ///
    /// We'll want to drive this list down by either:
    /// 1. fixing violations of the rule and begin enforcing it
    /// 2. deciding we want to allow the rule permanently, at which point
    ///    we should codify that separately in this task.
    ///
    /// This list shouldn't be added to; it should only get shorter.
    const MIGRATORY_RULES_TO_ALLOW: &[&str] = &[
        // There are a bunch of rules currently failing in the `style` group, so
        // allow all of those, for now.
        "clippy::style",
        // Individual rules that have violations in the codebase:
        "clippy::almost_complete_range",
        "clippy::arc_with_non_send_sync",
        "clippy::await_holding_lock",
        "clippy::borrow_deref_ref",
        "clippy::borrowed_box",
        "clippy::cast_abs_to_unsigned",
        "clippy::cmp_owned",
        "clippy::derive_ord_xor_partial_ord",
        "clippy::eq_op",
        "clippy::implied_bounds_in_impls",
        "clippy::let_underscore_future",
        "clippy::map_entry",
        "clippy::never_loop",
        "clippy::non_canonical_clone_impl",
        "clippy::non_canonical_partial_ord_impl",
        "clippy::reversed_empty_ranges",
        "clippy::single_range_in_vec_init",
        "clippy::suspicious_to_owned",
        "clippy::type_complexity",
        "clippy::unnecessary_to_owned",
    ];

    // When fixing violations automatically for a single package we don't care
    // about the rules we're already violating, since it may be possible to
    // have them fixed automatically.
    let ignore_suppressed_rules = args.fix && args.package.is_some();
    if !ignore_suppressed_rules {
        for rule in MIGRATORY_RULES_TO_ALLOW {
            clippy_command.args(["--allow", rule]);
        }
    }

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
