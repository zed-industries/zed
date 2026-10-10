//! `cedian_fake_omp`: hermetic stand-in for `omp --mode rpc-ui` (§86, P2).
//!
//! One binary, two modes, chosen by marker files inside `--session-dir`, or
//! beside the `--config` overlay when the session dir is recreated per run
//! (the reviewer, ADR-0043). The spawn profile scrubs the environment, so env
//! vars cannot carry the mode:
//!
//! - `fake-omp.record` exists → **record**: proxy to the real OMP binary named
//!   in that file, tee every stdout/stdin line into `fake-omp.recorded.jsonl`
//!   in the session dir (the one directory a sandboxed reviewer can write).
//! - `fake-omp.replay.jsonl` exists → **replay**: emit the recorded server
//!   frames, consume the host's frames in order, fail loudly on divergence.
//!
//! cedian spawns it through the real `OmpRuntime` + `SpawnProfile` path, so a
//! replay exercises the full boundary (handshake, v2 negotiation, host tools,
//! router) with no model call.
#![allow(
    clippy::disallowed_methods,
    reason = "headless, synchronous process control (OMP, LSP, DAP, Chrome, git, sandbox-exec): Zed's async spawn helpers do not apply"
)]

mod config;
mod fixture;
mod fs_effects;
mod record;
pub mod recording;
mod replay;

pub use fixture::{Dir, Placeholders, Record};

use std::path::{Path, PathBuf};

/// Marker: its content is the absolute path of the real `omp` to proxy.
pub const RECORD_MARKER: &str = "fake-omp.record";
/// Output of a record run (redacted fixture).
pub const RECORDED_FILE: &str = "fake-omp.recorded.jsonl";
/// Input of a replay run.
pub const REPLAY_FILE: &str = "fake-omp.replay.jsonl";

/// Exit code when the host diverges from the fixture.
pub const EXIT_DIVERGED: i32 = 3;

/// `omp --version`: fake-omp answers as the pinned OMP, so a hermetic lane
/// that points `CEDIAN_OMP_BINARY` at it runs with no off-pin warning
/// (ADR-0057 decision 4).
pub fn version() -> i32 {
    println!("omp/{}", recording::pinned_version());
    0
}

/// Entry point: `args` excludes the program name. Returns the exit code.
pub fn run(args: &[String]) -> i32 {
    // Without `--session-dir` (`Sessions::OmpDefault`) OMP picks its own
    // store; fake-omp stands in with the overlay's directory.
    let config_dir = || flag(args, "--config").and_then(|c| c.parent().map(Path::to_path_buf));
    let (Some(session_dir), Some(cwd)) = (
        flag(args, "--session-dir").or_else(config_dir),
        flag(args, "--cwd"),
    ) else {
        eprintln!("fake-omp: --cwd and --session-dir or --config are required");
        return 2;
    };
    let placeholders = Placeholders::new(&session_dir, &cwd, std::env::var("HOME").ok());
    let overlay_dir = flag(args, "--config").and_then(|c| c.parent().map(Path::to_path_buf));
    let marker_dir = [Some(session_dir.clone()), overlay_dir]
        .into_iter()
        .flatten()
        .find(|d| d.join(RECORD_MARKER).is_file() || d.join(REPLAY_FILE).is_file())
        .unwrap_or_else(|| session_dir.clone());
    let marker = marker_dir.join(RECORD_MARKER);
    if let Ok(real) = std::fs::read_to_string(&marker) {
        let real = PathBuf::from(real.trim());
        return record::run(
            &real,
            args,
            &session_dir.join(RECORDED_FILE),
            &cwd,
            &placeholders,
        );
    }
    let fixture = marker_dir.join(REPLAY_FILE);
    if fixture.is_file() {
        return replay::run(&fixture, &cwd, &placeholders);
    }
    eprintln!(
        "fake-omp: neither {} nor {} in {}",
        RECORD_MARKER,
        REPLAY_FILE,
        session_dir.display()
    );
    2
}

/// `omp config get <key> --json` stand-in: the badge's query in a replay.
/// `list`, `set`, `reset` and `path` go to [`config`] (the settings page).
/// Reads `tools.approvalMode` and `computer.enabled` from `.omp/config.yml`
/// in the current directory, with fixed defaults (`write`, `false`), so a
/// test workspace's project config shows up and an empty dir gives defaults.
pub fn config_get(args: &[String]) -> i32 {
    if let Some(code) = config::run(args) {
        return code;
    }
    let Some(key) = args.get(2) else {
        return 2;
    };
    let config = std::fs::read_to_string(".omp/config.yml").unwrap_or_default();
    let field = |section: &str, name: &str| {
        let mut inside = false;
        for line in config.lines() {
            if !line.starts_with(' ') {
                inside = line.trim_end() == format!("{section}:");
            } else if let Some(v) = line.trim().strip_prefix(&format!("{name}:")) {
                if inside {
                    return Some(v.trim().to_string());
                }
            }
        }
        None
    };
    let value = match key.as_str() {
        "tools.approvalMode" => {
            serde_json::json!(field("tools", "approvalMode").unwrap_or_else(|| "write".into()))
        }
        "computer.enabled" => {
            serde_json::json!(field("computer", "enabled").as_deref() == Some("true"))
        }
        "modelRoles" => {
            let mut record = serde_json::Map::new();
            let mut inside = false;
            for line in config.lines() {
                if !line.starts_with(' ') {
                    inside = line.trim_end() == "modelRoles:";
                } else if inside {
                    if let Some((k, v)) = line.trim().split_once(':') {
                        record.insert(
                            k.trim().into(),
                            serde_json::json!(v.trim().trim_matches('"')),
                        );
                    }
                }
            }
            serde_json::Value::Object(record)
        }
        "tools.approval" => {
            let mut record = serde_json::Map::new();
            let (mut in_tools, mut in_approval) = (false, false);
            for line in config.lines() {
                let indent = line.len() - line.trim_start().len();
                match indent {
                    0 => in_tools = line.trim_end() == "tools:",
                    2 => in_approval = in_tools && line.trim_end() == "  approval:",
                    _ if in_approval => {
                        if let Some((k, v)) = line.trim().split_once(':') {
                            record.insert(k.trim().into(), serde_json::json!(v.trim()));
                        }
                    }
                    _ => {}
                }
            }
            serde_json::Value::Object(record)
        }
        _ => return 1,
    };
    use std::io::Write as _;
    let line = serde_json::json!({"key": key, "value": value});
    i32::from(writeln!(std::io::stdout(), "{line}").is_err())
}

fn flag(args: &[String], name: &str) -> Option<PathBuf> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
}

/// Install `fixture` (a committed, redacted JSONL file) as the replay input
/// for a runtime that will use `session_dir`.
pub fn install_replay(session_dir: &Path, fixture: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(session_dir)?;
    std::fs::copy(fixture, session_dir.join(REPLAY_FILE)).map(|_| ())
}

/// Arm record mode: the next spawn in `session_dir` proxies to `real_omp`.
pub fn arm_record(session_dir: &Path, real_omp: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(session_dir)?;
    std::fs::write(
        session_dir.join(RECORD_MARKER),
        real_omp.to_string_lossy().as_bytes(),
    )
}
