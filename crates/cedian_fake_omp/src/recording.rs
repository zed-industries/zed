//! Record lanes (ADR-0058 decision 5): which OMP a lane may record against,
//! what a recording may not carry into a committed fixture, and how many
//! model turns it took.

use serde_json::Value;
use std::path::{Path, PathBuf};

/// The env var naming the OMP a record lane proxies to.
pub const RECORD_OMP_ENV: &str = "CEDIAN_RECORD_OMP";

/// The env var naming the app's record lane to arm (one per run).
pub const APP_RECORD_ENV: &str = "CEDIAN_U12_RECORD";

/// Whether the app test `lane` records this run.
pub fn armed(lane: &str) -> bool {
    std::env::var(APP_RECORD_ENV).as_deref() == Ok(lane)
}

/// How long a test waits for OMP: `replay`, or minutes while a real model
/// answers.
pub fn wait_limit(replay: std::time::Duration) -> std::time::Duration {
    if std::env::var_os(APP_RECORD_ENV).is_some() || std::env::var_os("CEDIAN_U4_RECORD").is_some()
    {
        std::time::Duration::from_secs(300)
    } else {
        replay
    }
}

/// Arm an app recording in `sessions`: the pinned OMP behind a
/// [`session_wrapper`] written in `scratch`.
pub fn arm_app(sessions: &Path, scratch: &Path) {
    let omp = pinned_omp().unwrap_or_else(|e| panic!("{e}"));
    let wrapper = session_wrapper(&omp, sessions, &scratch.join("omp-in-temp-store"))
        .unwrap_or_else(|e| panic!("the session wrapper: {e}"));
    crate::arm_record(sessions, &wrapper).unwrap_or_else(|e| panic!("arm record: {e}"));
}

/// The pinned OMP version (`vendor/omp-revision.json`).
pub fn pinned_version() -> String {
    let pin: Value = serde_json::from_str(include_str!("../../../vendor/omp-revision.json"))
        .expect("vendor/omp-revision.json is JSON");
    pin["spikeVerified"]["ompVersion"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

/// The OMP named by [`RECORD_OMP_ENV`], once `--version` shows it is the
/// pin. A recording made on another version would replay as the pin's.
pub fn pinned_omp() -> Result<PathBuf, String> {
    let omp = std::env::var_os(RECORD_OMP_ENV)
        .map(PathBuf::from)
        .ok_or_else(|| format!("{RECORD_OMP_ENV}: the pinned omp to record against"))?;
    check_pinned(&omp)?;
    Ok(omp)
}

fn check_pinned(omp: &Path) -> Result<(), String> {
    let out = std::process::Command::new(omp)
        .arg("--version")
        .output()
        .map_err(|e| format!("{}: {e}", omp.display()))?;
    let printed = String::from_utf8_lossy(&out.stdout);
    let found = printed.split_whitespace().next().unwrap_or_default();
    let want = format!("omp/{}", pinned_version());
    if found == want {
        Ok(())
    } else {
        Err(format!(
            "{} is {found:?}, not the pinned {want}: record only against the pin",
            omp.display()
        ))
    }
}

/// What the recorder's path redaction leaves but a fixture must not carry:
/// `get_state`'s system prompt (the person's mandate, prefs and memory) and
/// tool dump, the installed skills in OMP's command lists, and any spelling
/// of the person's home the redaction missed.
pub fn scrub(recorded: &str, home: Option<&str>) -> String {
    let scrubbed: String = recorded
        .lines()
        .map(|line| {
            let Ok(mut record) = serde_json::from_str::<Value>(line) else {
                return format!("{line}\n");
            };
            let frame = &mut record["frame"];
            if frame["command"] == "get_state" {
                if let Some(data) = frame.get_mut("data").and_then(Value::as_object_mut) {
                    data.insert("systemPrompt".into(), "${SCRUBBED}".into());
                    data.insert("dumpTools".into(), Value::Array(Vec::new()));
                }
            }
            if frame["type"] == "available_commands_update" {
                frame["commands"] = Value::Array(Vec::new());
            }
            format!("{record}\n")
        })
        .collect();
    match home {
        Some(home) if home.len() > 1 => scrubbed.replace(home.trim_end_matches('/'), "${HOME}"),
        _ => scrubbed,
    }
}

/// Model turns in a recording: the runs OMP started (`agent_start`).
pub fn model_turns(recorded: &str) -> usize {
    recorded
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|r| r["dir"] == "out" && r["frame"]["type"] == "agent_start")
        .count()
}

/// Scrub the recording at `recorded` into `fixture`, and print the model
/// turns it used, the line `script/cedian-bench/run.sh record` counts.
pub fn write_fixture(recorded: &Path, fixture: &Path) {
    let text =
        std::fs::read_to_string(recorded).unwrap_or_else(|e| panic!("{}: {e}", recorded.display()));
    let home = std::env::var("HOME").ok();
    std::fs::write(fixture, scrub(&text, home.as_deref()))
        .unwrap_or_else(|e| panic!("{}: {e}", fixture.display()));
    println!(
        "fixture written: {} (model turns: {})",
        fixture.display(),
        model_turns(&text)
    );
}

/// A wrapper that runs `omp` with its sessions in `sessions`: the app opens
/// OMP's own store (`Sessions::OmpDefault`), which a recording must keep
/// out of the person's.
pub fn session_wrapper(omp: &Path, sessions: &Path, at: &Path) -> std::io::Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::write(
        at,
        format!(
            "#!/bin/sh\nexec '{}' \"$@\" --session-dir '{}'\n",
            omp.display(),
            sessions.display()
        ),
    )?;
    std::fs::set_permissions(at, std::fs::Permissions::from_mode(0o755))?;
    Ok(at.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrub_removes_the_persons_setup() {
        let recorded = concat!(
            r#"{"dir":"out","frame":{"type":"response","command":"get_state","data":{"systemPrompt":"my memory","dumpTools":[{"name":"x"}],"sessionId":"s"}}}"#,
            "\n",
            r#"{"dir":"out","frame":{"type":"available_commands_update","commands":[{"name":"my-skill"}]}}"#,
            "\n",
            r#"{"dir":"out","frame":{"type":"tool_execution_end","result":"/Users/me/notes and /Users/me/"}}"#,
            "\n"
        );
        let scrubbed = scrub(recorded, Some("/Users/me/"));
        for secret in ["my memory", "\"x\"", "my-skill", "/Users/me"] {
            assert!(!scrubbed.contains(secret), "{secret} in {scrubbed}");
        }
        assert!(scrubbed.contains("\"sessionId\":\"s\""));
        assert!(scrubbed.contains("${HOME}/notes"), "{scrubbed}");
    }

    #[test]
    fn model_turns_are_omps_runs() {
        let recorded = concat!(
            r#"{"dir":"in","frame":{"type":"prompt","message":"agent_start"}}"#,
            "\n",
            r#"{"dir":"out","frame":{"type":"agent_start"}}"#,
            "\n",
            r#"{"dir":"out","frame":{"type":"turn_start"}}"#,
            "\n",
            r#"{"dir":"out","frame":{"type":"agent_start"}}"#,
            "\n"
        );
        assert_eq!(model_turns(recorded), 2);
    }

    #[test]
    fn only_the_pin_records() {
        let dir = std::env::temp_dir().join(format!("cedian-record-pin-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let fake = |name: &str, version: &str| {
            let path = dir.join(name);
            std::fs::write(&path, format!("#!/bin/sh\necho omp/{version}\n")).unwrap();
            std::fs::set_permissions(&path, std::os::unix::fs::PermissionsExt::from_mode(0o755))
                .unwrap();
            path
        };
        let pinned = fake("pinned", &pinned_version());
        let newer = fake("newer", "18.8.6");
        assert_eq!(check_pinned(&pinned), Ok(()));
        let refused = check_pinned(&newer).unwrap_err();
        assert!(refused.contains("18.8.6"), "{refused}");
        assert!(check_pinned(&dir.join("missing")).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
