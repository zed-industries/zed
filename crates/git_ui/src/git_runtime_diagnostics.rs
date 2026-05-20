//! Best-effort runtime diagnostics gathered to accompany the git job queue
//! debug dump. Every individual step is fallible and any failure is logged at
//! `warn` level and silently omitted from the output. The dump itself must
//! never fail just because diagnostics couldn't be collected.
//!
//! What we gather:
//! - All transitive descendant processes of the current Zed process
//!   (cross-platform via `sysinfo`).
//! - On Linux: each descendant's `/proc/<pid>/wchan` (kernel function the
//!   thread is currently sleeping in) and `State:` from `/proc/<pid>/status`.
//! - On macOS: for any descendant whose executable name contains `git`, a
//!   short `sample`-based user-space stack and `lsof` output. Both require
//!   the corresponding system binaries; if they aren't present or the
//!   invocation fails we skip them.
//! - On Windows: just the process tree (no portable way to grab another
//!   process's stack).
//!
//! The output is a `serde_json::Value`. Callers merge it into whatever larger
//! JSON payload they're producing.
//!
//! This is invoked from a developer-only "show git job queue" action, so it
//! is acceptable for the macOS path to spend a few seconds sampling.

use serde_json::{Map, Value};

pub async fn gather() -> Value {
    let mut diag = Map::new();

    match collect_process_tree() {
        Ok(tree) => {
            diag.insert("processes".into(), tree);
        }
        Err(err) => {
            log::warn!("git runtime diagnostics: failed to collect process tree: {err:#}");
        }
    }

    #[cfg(target_os = "linux")]
    {
        match collect_linux_proc_info() {
            Ok(info) => {
                diag.insert("linux_proc".into(), info);
            }
            Err(err) => {
                log::warn!("git runtime diagnostics: failed to read /proc info: {err:#}");
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        match collect_macos_git_child_diagnostics().await {
            Ok(info) => {
                if !info.is_null() {
                    diag.insert("macos_git_children".into(), info);
                }
            }
            Err(err) => {
                log::warn!(
                    "git runtime diagnostics: failed to collect macOS git-child info: {err:#}"
                );
            }
        }
    }

    Value::Object(diag)
}

/// Walk the descendant tree of the current process and return a JSON array
/// describing each descendant. Cross-platform; uses `sysinfo`.
fn collect_process_tree() -> anyhow::Result<Value> {
    use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};

    let current_pid = sysinfo::get_current_pid()
        .map_err(|e| anyhow::anyhow!("sysinfo::get_current_pid failed: {e}"))?;

    let refresh = ProcessRefreshKind::nothing()
        .with_cmd(UpdateKind::Always)
        .with_exe(UpdateKind::Always);
    let mut system = System::new_with_specifics(RefreshKind::nothing().with_processes(refresh));
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh);

    let descendants = descendants_of(&system, current_pid);

    let entries: Vec<Value> = descendants
        .iter()
        .filter_map(|pid| {
            let process = system.process(*pid)?;
            let cmd: Vec<String> = process
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().into_owned())
                .collect();
            Some(serde_json::json!({
                "pid": pid.as_u32(),
                "ppid": process.parent().map(|p| p.as_u32()),
                "name": process.name().to_string_lossy(),
                "exe": process.exe().map(|p| p.display().to_string()),
                "cmd": cmd,
                "status": format!("{:?}", process.status()),
                "run_time_secs": process.run_time(),
            }))
        })
        .collect();

    Ok(serde_json::json!({
        "zed_pid": current_pid.as_u32(),
        "descendant_count": entries.len(),
        "descendants": entries,
    }))
}

fn descendants_of(system: &sysinfo::System, root: sysinfo::Pid) -> Vec<sysinfo::Pid> {
    let mut parent_map: std::collections::HashMap<sysinfo::Pid, Vec<sysinfo::Pid>> =
        std::collections::HashMap::new();
    for (pid, process) in system.processes() {
        if let Some(parent) = process.parent() {
            parent_map.entry(parent).or_default().push(*pid);
        }
    }
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(p) = stack.pop() {
        if let Some(children) = parent_map.get(&p) {
            for c in children {
                out.push(*c);
                stack.push(*c);
            }
        }
    }
    out
}

#[cfg(target_os = "linux")]
fn collect_linux_proc_info() -> anyhow::Result<Value> {
    use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

    let current_pid = sysinfo::get_current_pid()
        .map_err(|e| anyhow::anyhow!("sysinfo::get_current_pid failed: {e}"))?;
    let refresh = ProcessRefreshKind::nothing();
    let mut system = System::new_with_specifics(RefreshKind::nothing().with_processes(refresh));
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh);

    let descendants = descendants_of(&system, current_pid);

    let mut entries = Vec::new();
    for pid in descendants {
        let pid_u32 = pid.as_u32();

        let wchan = match std::fs::read_to_string(format!("/proc/{pid_u32}/wchan")) {
            Ok(s) => Value::String(s.trim().to_string()),
            Err(err) => {
                log::warn!("git runtime diagnostics: failed to read /proc/{pid_u32}/wchan: {err}");
                Value::Null
            }
        };

        let state = match std::fs::read_to_string(format!("/proc/{pid_u32}/status")) {
            Ok(contents) => contents
                .lines()
                .find(|l| l.starts_with("State:"))
                .map(|l| Value::String(l.trim_start_matches("State:").trim().to_string()))
                .unwrap_or(Value::Null),
            Err(err) => {
                log::warn!("git runtime diagnostics: failed to read /proc/{pid_u32}/status: {err}");
                Value::Null
            }
        };

        entries.push(serde_json::json!({
            "pid": pid_u32,
            "wchan": wchan,
            "state": state,
        }));
    }

    Ok(serde_json::json!({ "descendants": entries }))
}

#[cfg(target_os = "macos")]
async fn collect_macos_git_child_diagnostics() -> anyhow::Result<Value> {
    use std::path::Path;
    use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};

    let sample_available = Path::new("/usr/bin/sample").exists();
    let lsof_available =
        Path::new("/usr/sbin/lsof").exists() || Path::new("/usr/bin/lsof").exists();
    let lsof_path = if Path::new("/usr/sbin/lsof").exists() {
        "/usr/sbin/lsof"
    } else {
        "/usr/bin/lsof"
    };

    if !sample_available && !lsof_available {
        return Ok(Value::Null);
    }

    let current_pid = sysinfo::get_current_pid()
        .map_err(|e| anyhow::anyhow!("sysinfo::get_current_pid failed: {e}"))?;

    let refresh = ProcessRefreshKind::nothing().with_exe(UpdateKind::Always);
    let mut system = System::new_with_specifics(RefreshKind::nothing().with_processes(refresh));
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh);

    let descendants = descendants_of(&system, current_pid);

    // Limit ourselves to git-flavored descendants. We don't want to spend
    // several seconds sampling unrelated children (terminals, language
    // servers, etc.). Match by name containing "git" — covers `git`,
    // `git-remote-https`, `git-credential-osxkeychain`, hook subprocesses
    // named with `git` in them, etc.
    let git_descendants: Vec<u32> = descendants
        .iter()
        .filter_map(|pid| {
            let process = system.process(*pid)?;
            let name = process.name().to_string_lossy();
            if name.contains("git") {
                Some(pid.as_u32())
            } else {
                None
            }
        })
        .collect();

    if git_descendants.is_empty() {
        return Ok(Value::Null);
    }

    let mut entries = Vec::new();
    for pid in git_descendants {
        let mut entry = Map::new();
        entry.insert("pid".into(), Value::from(pid));

        if sample_available {
            match run_capturing("/usr/bin/sample", &[&pid.to_string(), "2", "-mayDie"]).await {
                Ok(output) => {
                    entry.insert("sample".into(), Value::String(truncate(output, 64 * 1024)));
                }
                Err(err) => {
                    log::warn!("git runtime diagnostics: `sample {pid}` failed: {err}");
                }
            }
        }

        if lsof_available {
            match run_capturing(lsof_path, &["-p", &pid.to_string()]).await {
                Ok(output) => {
                    entry.insert("lsof".into(), Value::String(truncate(output, 64 * 1024)));
                }
                Err(err) => {
                    log::warn!("git runtime diagnostics: `lsof -p {pid}` failed: {err}");
                }
            }
        }

        entries.push(Value::Object(entry));
    }

    Ok(Value::Array(entries))
}

#[cfg(target_os = "macos")]
async fn run_capturing(program: &str, args: &[&str]) -> anyhow::Result<String> {
    let output = util::command::new_command(program)
        .args(args)
        .output()
        .await?;
    if !output.status.success() {
        anyhow::bail!(
            "{program} exited with status {:?}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(target_os = "macos")]
fn truncate(mut s: String, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s;
    }
    // Find a UTF-8 char boundary at or before `max_bytes`.
    let mut cut = max_bytes;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }
    s.truncate(cut);
    s.push_str("\n…(truncated)");
    s
}
