//! Zed Sim — an internal launcher that runs the real Zed binary in disposable,
//! state-controlled profiles for staff testing. See `SPEC.md` and `PLAN.md`.

mod config;
mod profile;
mod server;
mod states;

use std::path::PathBuf;

use anyhow::{Result, bail, ensure};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "zed-sim",
    about = "Launch disposable, state-controlled Zed sessions for staff testing."
)]
struct Args {
    /// Path to the Zed executable to launch. Overrides auto-discovery and the
    /// `ZED_SIM_BINARY` environment variable.
    #[arg(long, value_name = "PATH")]
    zed: Option<PathBuf>,

    /// Port for the local control panel. Defaults to an ephemeral port.
    #[arg(long, default_value_t = 0)]
    port: u16,

    /// Don't open the control panel in a browser automatically.
    #[arg(long)]
    no_open: bool,

    /// Path to the impersonation config file (JSON). Defaults to
    /// `tooling/zed-sim/zed-sim.config.json`. Missing is fine — impersonation
    /// simply stays unavailable.
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let zed_binary = resolve_zed_binary(args.zed)?;
    let config_path = args.config.unwrap_or_else(config::default_path);
    let app_config = config::load(&config_path)?;
    server::serve(&zed_binary, &app_config, args.port, !args.no_open)
}

/// Resolves which Zed executable to launch, in priority order: explicit `--zed`
/// flag, then `ZED_SIM_BINARY`, then the standard macOS app-bundle locations.
fn resolve_zed_binary(explicit: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        ensure!(path.exists(), "Zed binary not found at {}", path.display());
        return Ok(path);
    }

    if let Some(env_path) = std::env::var_os("ZED_SIM_BINARY") {
        let path = PathBuf::from(env_path);
        ensure!(
            path.exists(),
            "ZED_SIM_BINARY points to a missing file: {}",
            path.display()
        );
        return Ok(path);
    }

    for candidate in default_binary_candidates() {
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    bail!(
        "Could not find a Zed binary.\n\
         Pass --zed <path> or set ZED_SIM_BINARY=<path>.\n\
         Looked in: /Applications/Zed.app, Zed Preview.app, Zed Nightly.app."
    )
}

fn default_binary_candidates() -> Vec<PathBuf> {
    ["Zed.app", "Zed Preview.app", "Zed Nightly.app"]
        .iter()
        .map(|app| {
            PathBuf::from("/Applications")
                .join(app)
                .join("Contents/MacOS/zed")
        })
        .collect()
}
