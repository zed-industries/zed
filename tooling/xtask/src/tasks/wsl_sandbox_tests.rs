#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]

use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

/// Runs the Windows WSL Bubblewrap sandbox behavior tests — the Windows analog
/// of `cargo xtask sandbox-tests`.
///
/// Where the Linux tests boot a NixOS VM, here we build and run
/// `wsl_sandbox_test_helper` (sandbox crate, `wsl-test` feature), which drives
/// the real `windows_wsl::wrap_invocation`, spawns the resulting `wsl.exe`
/// command, and asserts the sandbox's grants and restrictions hold end-to-end.
///
/// This must run on Windows with WSL, against a default distro that has
/// `bubblewrap` installed and unprivileged user namespaces enabled.
/// `script/test-wsl-sandbox.ps1` can provision that environment and invoke this.
#[derive(Parser)]
pub struct WslSandboxTestsArgs {
    /// Fail (rather than skip the enforcement checks) when the sandbox can't
    /// actually be enforced. Use this once the WSL environment is provisioned,
    /// so a broken sandbox is caught instead of silently skipped.
    #[arg(long)]
    require_enforced: bool,

    /// Build and run the helper in release mode.
    #[arg(long)]
    release: bool,
}

pub fn run_wsl_sandbox_tests(args: WslSandboxTestsArgs) -> Result<()> {
    if !cfg!(target_os = "windows") {
        bail!(
            "wsl-sandbox-tests drives the Windows WSL sandbox and must run on Windows with WSL \
             installed. See script/test-wsl-sandbox.ps1."
        );
    }

    let mut command = Command::new("cargo");
    command.args([
        "run",
        "-p",
        "sandbox",
        "--features",
        "wsl-test",
        "--bin",
        "wsl_sandbox_test_helper",
    ]);
    if args.release {
        command.arg("--release");
    }
    if args.require_enforced {
        command.env("ZED_TEST_SANDBOX_REQUIRE_ENFORCED", "1");
    }

    eprintln!("Running WSL sandbox behavior tests");
    let status = command
        .status()
        .context("failed to run `cargo run` for the WSL sandbox helper")?;
    if !status.success() {
        bail!("WSL sandbox tests failed");
    }
    Ok(())
}
