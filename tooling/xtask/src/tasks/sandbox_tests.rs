#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]

use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

/// Runs the Linux Bubblewrap sandboxing NixOS VM tests (see `nix/tests/sandboxing`).
///
/// Each test boots a real kernel under QEMU to exercise bwrap behavior across
/// host configurations: a working host (the sandbox must be enforced and the
/// full fs x network policy matrix must hold), and degraded hosts (no bwrap,
/// setuid-only bwrap, user namespaces disabled) where `Sandbox::can_create`
/// must report the specific failure and the consumer must fail closed. They
/// therefore only run on Linux with `/dev/kvm` available, and require `nix`
/// with flakes enabled.
#[derive(Parser)]
pub struct SandboxTestsArgs {
    /// Names of specific tests to run (e.g. `sandbox-working`). Defaults
    /// to all.
    tests: Vec<String>,

    /// Nix system to build for. Defaults to the host system.
    #[arg(long)]
    system: Option<String>,

    /// Force already-built tests to re-run instead of returning the cached
    /// result (passes `--rebuild` to `nix build`).
    ///
    /// This only works once a test has been built at least once; after changing
    /// the test code the derivation changes, so run without `--rebuild` first.
    #[arg(long)]
    rebuild: bool,

    /// List the available tests and exit without running anything.
    #[arg(long)]
    list: bool,
}

/// Tests in `nix/tests/sandboxing` are exposed as flake checks with this prefix.
const TEST_PREFIX: &str = "sandbox-";

pub fn run_sandbox_tests(args: SandboxTestsArgs) -> Result<()> {
    let system = args.system.unwrap_or_else(host_system);
    let available = available_tests(&system)?;

    if args.list {
        for name in &available {
            println!("{name}");
        }
        return Ok(());
    }

    let selected = if args.tests.is_empty() {
        available
    } else {
        for name in &args.tests {
            if !available.contains(name) {
                bail!(
                    "unknown sandbox test {name:?}; available: {}",
                    available.join(", ")
                );
            }
        }
        args.tests
    };

    if selected.is_empty() {
        bail!("no sandbox tests found for system `{system}`");
    }

    let mut command = Command::new("nix");
    command.args(["build", "-L", "--keep-going"]);
    if args.rebuild {
        command.arg("--rebuild");
    }
    for name in &selected {
        command.arg(format!(".#checks.{system}.{name}"));
    }

    eprintln!("Running sandbox tests: {}", selected.join(", "));
    let status = command.status().context("failed to run `nix build`")?;
    if !status.success() {
        bail!("sandbox tests failed");
    }
    Ok(())
}

/// The Nix system double for the host. These tests are Linux-only, so we always
/// target a `*-linux` system (overridable via `--system`).
fn host_system() -> String {
    format!("{}-linux", std::env::consts::ARCH)
}

/// Query the flake for the names of the sandbox-test checks, so the task stays
/// in sync with `nix/tests/sandboxing` without a hardcoded list.
fn available_tests(system: &str) -> Result<Vec<String>> {
    let output = Command::new("nix")
        .args([
            "eval",
            &format!(".#checks.{system}"),
            "--apply",
            "checks: builtins.concatStringsSep \"\\n\" (builtins.attrNames checks)",
            "--raw",
        ])
        .output()
        .context("failed to run `nix eval` to list checks (is `nix` installed?)")?;
    if !output.status.success() {
        bail!(
            "`nix eval` failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let names = String::from_utf8(output.stdout).context("`nix eval` output was not UTF-8")?;
    let mut tests: Vec<String> = names
        .lines()
        .filter(|name| name.starts_with(TEST_PREFIX))
        .map(str::to_string)
        .collect();
    tests.sort();
    Ok(tests)
}
