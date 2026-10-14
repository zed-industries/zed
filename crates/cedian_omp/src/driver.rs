//! One driver per session (ADR-0040 decision 5). OMP lets a second process
//! resume a session another live process holds, and both then append to one
//! file (measured on OMP 18.6.1). cedian refuses instead: before it drives a
//! session it lists the processes holding the session's files, and any
//! process outside its own OMP's process group is another driver (OMP is
//! spawned as a group leader, so a proxy's OMP, such as a recording's,
//! stays one of its own).
//!
//! The files: the session `.jsonl`, which a process keeps open for writing
//! once it has written to the session, and the owner lease OMP's creating
//! process holds an exclusive flock on, `<state root>/run/session-owners/
//! <sessionId>.lock` (the state root is `~/.omp`; `PI_CODING_AGENT_DIR` does
//! not move it). A process that resumed but has not written yet holds
//! neither, so it goes unseen.
//!
//! OMP is a Bun program, and Bun opens files close-on-exec and spawns
//! children with every other descriptor closed, so OMP's own children (its
//! bash tool) never hold these files: only another process does.
//!
//! One deliberate fail-open: `lsof` exiting 1 with nothing on stdout and only
//! warnings on stderr (an unreachable mount, say) counts as nobody holding
//! the files, since that is how it reports "no holder" next to such a mount.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// How long `lsof` may take before the check counts as failed.
const LSOF_DEADLINE: Duration = Duration::from_secs(5);

/// The session's files another driver would hold: the owner lease (when
/// OMP's state root is known) and the session file (when OMP named it).
pub fn session_files(
    state_root: Option<&Path>,
    session_id: &str,
    session_file: Option<&Path>,
) -> Vec<PathBuf> {
    state_root
        .map(|root| {
            root.join("run/session-owners")
                .join(format!("{session_id}.lock"))
        })
        .into_iter()
        .chain(session_file.map(Path::to_path_buf))
        .collect()
}

/// OMP's state root: `~/.omp`.
pub fn state_root() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".omp"))
}

/// The holders outside `own`'s process group (`own` is the app's OMP, a
/// group leader; `group_of` answers a pid's group): other drivers.
pub fn other_drivers(holders: &[u32], own: u32, group_of: impl Fn(u32) -> Option<u32>) -> Vec<u32> {
    let mut others: Vec<u32> = holders
        .iter()
        .copied()
        .filter(|pid| *pid != own && group_of(*pid) != Some(own))
        .collect();
    others.sort_unstable();
    others.dedup();
    others
}

fn lsof() -> &'static Path {
    Path::new(
        ["/usr/sbin/lsof", "/usr/bin/lsof"]
            .into_iter()
            .find(|p| Path::new(p).exists())
            .unwrap_or("lsof"),
    )
}

/// Process ids holding any of `files` open, by `lsof`. Files that do not
/// exist are held by nobody.
pub fn holders(files: &[PathBuf]) -> Result<Vec<u32>, String> {
    holders_with(lsof(), files, LSOF_DEADLINE)
}

/// [`holders`] with the `lsof` to run and how long it may take.
pub fn holders_with(
    lsof: &Path,
    files: &[PathBuf],
    deadline: Duration,
) -> Result<Vec<u32>, String> {
    let files: Vec<&PathBuf> = files.iter().filter(|f| f.exists()).collect();
    if files.is_empty() {
        return Ok(Vec::new());
    }
    let mut child = std::process::Command::new(lsof)
        .arg("-t")
        .args(&files)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot run lsof: {e}"))?;
    let drain = |pipe: Option<Box<dyn std::io::Read + Send>>| {
        std::thread::spawn(move || {
            let mut out = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut out);
            }
            out
        })
    };
    let stdout = drain(child.stdout.take().map(|p| Box::new(p) as _));
    let stderr = drain(child.stderr.take().map(|p| Box::new(p) as _));
    let until = Instant::now() + deadline;
    while child
        .try_wait()
        .map_err(|e| format!("lsof: {e}"))?
        .is_none()
    {
        if Instant::now() >= until {
            child.kill().ok();
            child.wait().ok();
            return Err(format!(
                "lsof did not finish in {} s",
                deadline.as_secs_f32()
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let status = child.wait().map_err(|e| format!("lsof: {e}"))?;
    let stdout = String::from_utf8_lossy(&stdout.join().unwrap_or_default()).into_owned();
    let stderr = String::from_utf8_lossy(&stderr.join().unwrap_or_default()).into_owned();
    let pids: Vec<u32> = stdout
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .collect();
    if !pids.is_empty() || (status.success() && stdout.trim().is_empty()) {
        return Ok(pids);
    }
    // lsof exits 1 when no process has any of the files open, silent or
    // with only warnings (an unreachable mount, say).
    let only_warnings = stderr
        .lines()
        .all(|l| l.trim().is_empty() || l.contains("WARNING"));
    if status.code() == Some(1) && stdout.trim().is_empty() && only_warnings {
        return Ok(Vec::new());
    }
    Err(format!(
        "lsof failed ({status}): {}{}",
        stderr.trim(),
        stdout.trim()
    ))
}

fn process_group(pid: u32) -> Option<u32> {
    let pid = libc::pid_t::try_from(pid).ok()?;
    // SAFETY: getpgid reads another process's group id and touches no memory.
    let group = unsafe { libc::getpgid(pid) };
    u32::try_from(group).ok().filter(|g| *g > 0)
}

/// SIGKILL the process group led by `pid`, a driver of the app's own that
/// did not exit (OMP is spawned as a group leader, so this reaches every
/// process it started).
pub fn kill_group(pid: u32) {
    let _ = std::process::Command::new("/bin/kill")
        .args(["-KILL", "--", &format!("-{pid}")])
        .stdin(std::process::Stdio::null())
        .status();
}

/// `Ok(pids)` of the other drivers of the session (empty when cedian may
/// drive it), or why it could not be checked: `own` (the app's OMP) is
/// unknown, `lsof` failed, or `must_see` is set and none of `files` exists
/// (a resumed session has at least one).
pub fn other_drivers_of(
    files: &[PathBuf],
    own: Option<u32>,
    must_see: bool,
) -> Result<Vec<u32>, String> {
    let own = own.ok_or("OMP's process id is unknown")?;
    if must_see && !files.iter().any(|f| f.exists()) {
        return Err("none of the session's files could be found".to_string());
    }
    holders(files).map(|pids| other_drivers(&pids, own, process_group))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_apps_own_omp_is_not_another_driver() {
        let alone = |pid: u32| Some(pid);
        assert_eq!(other_drivers(&[7, 9, 7, 3], 7, alone), [3, 9]);
        assert!(other_drivers(&[7], 7, alone).is_empty());
    }

    /// A recording's proxy is the app's OMP and the real OMP its child, in
    /// the group the proxy leads: that child holds the session, and is ours.
    #[test]
    fn a_process_in_the_apps_omp_group_is_not_another_driver() {
        let group_of = |pid: u32| match pid {
            8 => Some(7),
            9 => Some(9),
            _ => None,
        };
        assert_eq!(other_drivers(&[8, 9, 3], 7, group_of), [3, 9]);
    }

    #[test]
    fn the_real_process_group_of_this_test_is_read() {
        let me = std::process::id();
        let group = process_group(me).expect("this process has a group");
        assert!(other_drivers(&[me], group, process_group).is_empty());
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cedian-lsof-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("s.jsonl"), "").unwrap();
        dir
    }

    /// A stand-in `lsof` running `script`.
    fn fake_lsof(dir: &Path, script: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt as _;
        let path = dir.join("lsof");
        std::fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[test]
    fn lsof_reports_nobody_only_by_a_quiet_exit_1() {
        let dir = scratch("exits");
        let files = [dir.join("s.jsonl")];
        let run =
            |script: &str| holders_with(&fake_lsof(&dir, script), &files, Duration::from_secs(5));
        assert_eq!(run("exit 1"), Ok(vec![]));
        assert_eq!(
            run("echo 'lsof: WARNING: cannot stat() nfs file system /net' >&2; exit 1"),
            Ok(vec![])
        );
        assert_eq!(run("echo 42; exit 0"), Ok(vec![42]));
        assert_eq!(run("echo 42; exit 1"), Ok(vec![42]));
        for script in [
            "exit 2",
            "echo garbage; exit 0",
            "echo 'lsof: status error on x' >&2; exit 1",
        ] {
            assert!(run(script).is_err(), "{script}");
        }
        let many = run("seq 1 100000; exit 0").map(|pids| pids.len());
        assert_eq!(many, Ok(100000), "a long list is read while lsof runs");
        let slow = holders_with(
            &fake_lsof(&dir, "exec sleep 10"),
            &files,
            Duration::from_millis(200),
        );
        assert!(slow.is_err(), "{slow:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_check_that_cannot_see_fails_closed() {
        let dir = scratch("blind");
        let missing = [dir.join("gone.jsonl")];
        assert!(
            other_drivers_of(&missing, Some(1), true).is_err(),
            "resumed, nothing to check"
        );
        assert_eq!(
            other_drivers_of(&missing, Some(1), false),
            Ok(vec![]),
            "new, not written yet"
        );
        let present = [dir.join("s.jsonl")];
        assert!(
            other_drivers_of(&present, None, true).is_err(),
            "own pid unknown"
        );
        assert!(
            session_files(None, "s", None).is_empty(),
            "no HOME, no file named"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// OMP's children (its bash tool) do not inherit the session files: run
    /// OMP's own Bun runtime (`BUN_BE_BUN=1`, no model call), open a file the
    /// way OMP does, spawn a child both ways OMP does, and list the holders.
    /// `cargo test -p cedian_omp -- --ignored live_omp_children`
    #[test]
    #[ignore]
    fn live_omp_children_do_not_hold_the_session_files() {
        let dir = scratch("children");
        let file = dir.join("s.jsonl");
        let script = dir.join("probe.js");
        std::fs::write(
            &script,
            r#"const fs = require("fs");
fs.openSync(process.argv[2], "a");
const a = require("child_process").spawn("/bin/sleep", ["5"]);
const b = Bun.spawn(["/bin/sh", "-c", "sleep 5"]);
setTimeout(() => { console.log(`${process.pid} ${a.pid} ${b.pid}`); }, 300);
setTimeout(() => { a.kill(); b.kill(); process.exit(0); }, 3000);"#,
        )
        .unwrap();
        let omp = crate::resolve_on_path("omp", std::env::var("PATH").ok().as_deref()).unwrap();
        let mut probe = std::process::Command::new(omp)
            .env("BUN_BE_BUN", "1")
            .arg("run")
            .arg(&script)
            .arg(&file)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut line = String::new();
        std::io::BufRead::read_line(
            &mut std::io::BufReader::new(probe.stdout.take().unwrap()),
            &mut line,
        )
        .unwrap();
        let pids: Vec<u32> = line
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();
        let held = holders(&[file]).unwrap();
        probe.wait().ok();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(pids.len(), 3, "{line}");
        assert_eq!(
            held,
            [pids[0]],
            "only OMP itself, not its children {:?}",
            &pids[1..]
        );
    }

    #[test]
    fn a_process_holding_the_session_file_is_another_driver() {
        let dir = std::env::temp_dir().join(format!("cedian-driver-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("s.jsonl");
        std::fs::write(&file, "").unwrap();
        let files = session_files(Some(&dir), "s", Some(&file));
        assert!(
            other_drivers_of(&files, Some(1), true).unwrap().is_empty(),
            "nobody yet"
        );
        let mut holder = std::process::Command::new("/bin/sh")
            .arg("-c")
            .arg("exec 3>>\"$0\"; exec sleep 30")
            .arg(&file)
            .spawn()
            .unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        let found = loop {
            let found = other_drivers_of(&files, Some(1), true).unwrap();
            if !found.is_empty() || std::time::Instant::now() > deadline {
                break found;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        };
        assert_eq!(
            other_drivers_of(&files, Some(holder.id()), true).unwrap(),
            Vec::<u32>::new(),
            "its own OMP"
        );
        holder.kill().ok();
        holder.wait().ok();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(found, [holder.id()]);
    }
}
