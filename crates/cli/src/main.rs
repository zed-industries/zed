//! A forwarding shim: all CLI logic lives in the `zed` binary itself
//! (see `crates/zed/src/cli_client.rs`), this binary only locates it and
//! forwards the invocation verbatim.
//!
//! It continues to exist because:
//! - it is the stable target for `zed` symlinks/desktop entries created by
//!   older installs and packaging scripts;
//! - on Windows, `zed.exe` is a windows-subsystem binary: interactive shells
//!   do not wait for it and do not attach its stdio to the console, and
//!   `SSH_ASKPASS` requires a directly executable console binary.

#![allow(
    clippy::disallowed_methods,
    reason = "We are not in an async environment, so std::process::Command is fine"
)]

use std::path::PathBuf;
use std::process::Command;

fn locate_zed_binary() -> anyhow::Result<PathBuf> {
    use anyhow::Context as _;

    let cli = std::env::current_exe()?.canonicalize()?;
    let dir = cli.parent().context("no parent path for cli")?;

    let possible_locations: &[&str] = if cfg!(target_os = "macos") {
        // Both the installed app bundle and development target directories
        // keep the two binaries side by side.
        &["./zed"]
    } else if cfg!(target_os = "windows") {
        // ../Zed.exe is the standard, lib/zed is for MSYS2, ./zed.exe is for
        // the target directory in development builds.
        &["../Zed.exe", "../lib/zed/zed-editor.exe", "./zed.exe"]
    } else {
        // libexec is the standard, lib/zed is for Arch (and other non-libexec
        // distros), ./zed is for the target directory in development builds.
        &["../libexec/zed-editor", "../lib/zed/zed-editor", "./zed"]
    };

    possible_locations
        .iter()
        .find_map(|location| {
            dir.join(location)
                .canonicalize()
                .ok()
                .filter(|path| path != &cli)
        })
        .with_context(|| {
            format!(
                "could not find the zed binary at any of: {}",
                possible_locations.join(", ")
            )
        })
}

fn main() {
    let zed = match locate_zed_binary() {
        Ok(zed) => zed,
        Err(error) => {
            eprintln!("error: {error:#}");
            std::process::exit(1);
        }
    };

    let args = std::env::args_os().skip(1);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;

        let argv0 = std::env::args_os()
            .next()
            .unwrap_or_else(|| zed.clone().into_os_string());
        let error = Command::new(&zed).arg0(argv0).args(args).exec();
        eprintln!("error: failed to exec {zed:?}: {error}");
        std::process::exit(1);
    }

    #[cfg(not(unix))]
    {
        match Command::new(&zed).args(args).status() {
            Ok(status) => std::process::exit(status.code().unwrap_or(1)),
            Err(error) => {
                eprintln!("error: failed to run {zed:?}: {error}");
                std::process::exit(1);
            }
        }
    }
}
