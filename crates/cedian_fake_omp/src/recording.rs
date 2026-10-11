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

/// The env var naming the model a recording must have run on: the
/// benchmark's (`script/cedian-bench/run.sh record` sets it).
pub const RECORD_MODEL_ENV: &str = "CEDIAN_RECORD_MODEL";

/// Who recorded: what a fixture must never carry about them.
pub struct Owner {
    pub home: Option<String>,
    pub user: Option<String>,
    pub email: Option<String>,
}

impl Owner {
    /// The person running this process: HOME, USER and `git config
    /// user.email`.
    pub fn current() -> Self {
        let email = std::process::Command::new("git")
            .args(["config", "user.email"])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
        Owner {
            home: std::env::var("HOME").ok(),
            user: std::env::var("USER").ok(),
            email,
        }
    }
}

/// What in `text` a committed fixture must not carry, named by kind, never
/// by value. Runs on the whole text, so a line that is not JSON, a raw
/// `rpc_chunk` or a tool's output is checked like any frame.
pub fn leaks(text: &str, owner: &Owner) -> Vec<String> {
    let shapes = [
        (
            r"(?-u)[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)+",
            "an email address",
        ),
        (r"(?i-u)\bbearer\s+[A-Za-z0-9._~+/=-]{8,}", "a Bearer token"),
        (r"(?-u)\bsk-[A-Za-z0-9_-]{16,}", "an sk- key"),
        (
            r#"(?i-u)api[_-]?key["']?\s*[:=]\s*["']?[A-Za-z0-9_-]{12,}|\bgh[pousr]_[A-Za-z0-9]{20,}|\bAKIA[0-9A-Z]{16}\b"#,
            "an API key",
        ),
        (
            r"(?-u)\beyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}",
            "a JWT",
        ),
        (
            r"(?i-u)(/|\\/|%2f)users(/|\\/|%2f)[a-z0-9._-]",
            "a /Users/<name> path",
        ),
    ];
    let mut hits: Vec<String> = shapes
        .iter()
        .filter(|(re, _)| regex::Regex::new(re).expect("a leak shape").is_match(text))
        .map(|(_, kind)| kind.to_string())
        .collect();
    let lower = text.to_lowercase();
    let mut has = |value: &Option<String>, kind: &str, word: bool| {
        let Some(v) = value.as_deref().map(str::trim).filter(|v| v.len() > 1) else {
            return;
        };
        let v = v.trim_end_matches('/').to_lowercase();
        let found = if word {
            regex::Regex::new(&format!(r"(?-u)\b{}\b", regex::escape(&v)))
                .is_ok_and(|re| re.is_match(&lower))
        } else {
            lower.contains(&v)
        };
        if found {
            hits.push(kind.to_string());
        }
    };
    has(&owner.email, "the owner's git email", false);
    has(&owner.home, "the HOME value", false);
    has(&owner.user, "the USER value", true);
    hits
}

/// The models OMP reported in a recording, as `provider/id`, in order:
/// `get_state`'s model and every message that names its model.
pub fn observed_models(recorded: &str) -> Vec<String> {
    fn walk(v: &Value, out: &mut Vec<String>) {
        match v {
            Value::Object(map) => {
                let named = match (map.get("provider"), map.get("model"), map.get("id")) {
                    (Some(Value::String(p)), Some(Value::String(m)), _) => Some(format!("{p}/{m}")),
                    _ => None,
                };
                if let Some(Value::Object(model)) = map.get("model") {
                    if let (Some(Value::String(p)), Some(Value::String(id))) =
                        (model.get("provider"), model.get("id"))
                    {
                        let m = format!("{p}/{id}");
                        if !out.contains(&m) {
                            out.push(m);
                        }
                    }
                }
                if let Some(m) = named {
                    if !out.contains(&m) {
                        out.push(m);
                    }
                }
                map.values().for_each(|v| walk(v, out));
            }
            Value::Array(items) => items.iter().for_each(|v| walk(v, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for line in recorded.lines() {
        if let Ok(record) = serde_json::from_str::<Value>(line) {
            walk(&record, &mut out);
        }
    }
    out
}

/// Scrub `recorded` into `fixture` and return its model turns, or refuse
/// without writing: a recording on another model than `model`, or one whose
/// scrubbed text still carries a [`leaks`] hit.
pub fn try_write_fixture(
    recorded: &Path,
    fixture: &Path,
    owner: &Owner,
    model: &str,
) -> Result<usize, String> {
    let text =
        std::fs::read_to_string(recorded).map_err(|e| format!("{}: {e}", recorded.display()))?;
    let observed = observed_models(&text);
    if observed.is_empty() || observed.iter().any(|m| m != model) {
        return Err(format!(
            "{}: OMP ran {observed:?}, the benchmark's model is {model}; not written",
            fixture.display()
        ));
    }
    let scrubbed = scrub(&text, owner.home.as_deref());
    let hits = leaks(&scrubbed, owner);
    if !hits.is_empty() {
        return Err(format!(
            "{}: the scrubbed recording still carries {}; not written",
            fixture.display(),
            hits.join(", ")
        ));
    }
    std::fs::write(fixture, scrubbed).map_err(|e| format!("{}: {e}", fixture.display()))?;
    Ok(model_turns(&text))
}

/// Model turns in a recording: the runs OMP started (`agent_start`).
pub fn model_turns(recorded: &str) -> usize {
    recorded
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|r| r["dir"] == "out" && r["frame"]["type"] == "agent_start")
        .count()
}

/// Scrub the recording at `recorded` into `fixture` ([`try_write_fixture`]
/// for the person running this, on [`RECORD_MODEL_ENV`]'s model), and
/// print the model turns it used, the line `script/cedian-bench/run.sh
/// record` counts. A refused recording panics and writes nothing.
pub fn write_fixture(recorded: &Path, fixture: &Path) {
    let model = std::env::var(RECORD_MODEL_ENV)
        .unwrap_or_else(|_| panic!("{RECORD_MODEL_ENV}: the model this recording must run on"));
    let text = std::fs::read_to_string(recorded).unwrap_or_default();
    match try_write_fixture(recorded, fixture, &Owner::current(), &model) {
        Ok(turns) => println!(
            "fixture written: {} (model turns: {turns})",
            fixture.display()
        ),
        Err(refused) => {
            println!("fixture refused (model turns: {})", model_turns(&text));
            panic!("{refused}")
        }
    }
}

/// A wrapper that runs `omp` with its sessions in `sessions`: the app opens
/// OMP's own store (`Sessions::OmpDefault`), which a recording must keep
/// out of the person's.
pub fn session_wrapper(omp: &Path, sessions: &Path, at: &Path) -> std::io::Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::write(
        at,
        format!(
            "#!/bin/sh\nexec {} \"$@\" --session-dir {}\n",
            sh_quote(omp),
            sh_quote(sessions)
        ),
    )?;
    std::fs::set_permissions(at, std::fs::Permissions::from_mode(0o755))?;
    Ok(at.to_path_buf())
}

fn sh_quote(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', r"'\''"))
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

    fn owner() -> Owner {
        Owner {
            home: Some("/Users/pond".into()),
            user: Some("pond".into()),
            email: Some("owner@example.org".into()),
        }
    }

    #[test]
    fn every_secret_shape_is_named_not_shown() {
        let cases = [
            ("mail me at a.b+c@host.io", "an email address"),
            ("author owner@example.org", "the owner's git email"),
            (
                "Authorization: Bearer abcDEF123456789.xyz",
                "a Bearer token",
            ),
            ("key sk-proj1234567890abcdefXYZ", "an sk- key"),
            (r#""api_key": "A1b2C3d4E5f6G7h8""#, "an API key"),
            ("ghp_abcdefghijklmnopqrstuvwxyz0123", "an API key"),
            (
                "eyJhbGciOiJIUzI1.eyJzdWIiOiIxMjM0.SflKxwRJSMeKKF2Q",
                "a JWT",
            ),
            ("cat /Users/somebody/x", "a /Users/<name> path"),
            (r#"cat \/Users\/somebody"#, "a /Users/<name> path"),
            ("cat %2FUsers%2Fsomebody", "a /Users/<name> path"),
            ("cat /USERS/POND", "a /Users/<name> path"),
            ("in /home-less/Users/pond", "a /Users/<name> path"),
            ("logged in as pond today", "the USER value"),
        ];
        for (text, kind) in cases {
            let hits = leaks(text, &owner());
            assert!(hits.iter().any(|h| h == kind), "{text}: {hits:?}");
            assert!(
                hits.iter().all(|h| !h.contains("pond") && !h.contains('@')),
                "{hits:?}"
            );
        }
        let home_only = Owner {
            home: Some("/var/home/me".into()),
            user: None,
            email: None,
        };
        assert_eq!(
            leaks("at /var/home/me/x", &home_only),
            vec!["the HOME value"]
        );
        assert!(leaks("respond with ${HOME}/notes and t@t", &owner()).is_empty());
    }

    #[test]
    fn a_leaky_recording_is_never_written() {
        let dir = std::env::temp_dir().join(format!("cedian-record-leak-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let recorded = dir.join("recorded.jsonl");
        let fixture = dir.join("fixture.jsonl");
        let model = r#"{"dir":"out","frame":{"type":"response","command":"get_state","data":{"model":{"id":"m","provider":"p"}}}}"#;
        std::fs::write(
            &recorded,
            format!("{model}\nrpc_chunk raw: Bearer abcdefghijklmnop\n"),
        )
        .unwrap();
        let refused = try_write_fixture(&recorded, &fixture, &owner(), "p/m").unwrap_err();
        assert!(refused.contains("a Bearer token"), "{refused}");
        assert!(!refused.contains("abcdefghijklmnop"), "{refused}");
        assert!(!fixture.exists());

        std::fs::write(&recorded, format!("{model}\n")).unwrap();
        let wrong = try_write_fixture(&recorded, &fixture, &owner(), "p/other").unwrap_err();
        assert!(
            wrong.contains("p/m") && wrong.contains("p/other"),
            "{wrong}"
        );
        assert!(!fixture.exists());
        assert_eq!(
            try_write_fixture(&recorded, &fixture, &owner(), "p/m"),
            Ok(0)
        );
        assert!(fixture.exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn observed_models_come_from_state_and_messages() {
        let recorded = concat!(
            r#"{"dir":"out","frame":{"type":"response","command":"get_state","data":{"model":{"id":"m","provider":"p"}}}}"#,
            "\n",
            r#"{"dir":"out","frame":{"type":"message_end","message":{"role":"assistant","provider":"q","model":"n"}}}"#,
            "\n"
        );
        assert_eq!(observed_models(recorded), vec!["p/m", "q/n"]);
        assert!(observed_models("{}").is_empty());
    }

    #[test]
    fn a_wrapper_path_may_hold_a_quote() {
        let dir = std::env::temp_dir().join(format!("cedian-wrap-it's-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let omp = dir.join("o'mp");
        std::fs::write(&omp, "#!/bin/sh\nprintf '%s\\n' \"$@\"\n").unwrap();
        std::fs::set_permissions(&omp, std::os::unix::fs::PermissionsExt::from_mode(0o755))
            .unwrap();
        let w = session_wrapper(&omp, &dir.join("s'ess"), &dir.join("w")).unwrap();
        let out = std::process::Command::new(&w)
            .arg("--mode")
            .output()
            .unwrap();
        let printed = String::from_utf8_lossy(&out.stdout);
        assert!(
            printed.contains("--mode") && printed.contains("s'ess"),
            "{printed}"
        );
        std::fs::remove_dir_all(&dir).unwrap();
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
        // Not a version at all, only "not the pin": a pin bump cannot make
        // this pass by accident.
        let off_pin = fake("other", &format!("{}-next", pinned_version()));
        assert_eq!(check_pinned(&pinned), Ok(()));
        let refused = check_pinned(&off_pin).unwrap_err();
        assert!(
            refused.contains(&pinned_version()) && refused.contains("-next"),
            "{refused}"
        );
        assert!(check_pinned(&dir.join("missing")).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
