//! A dedicated registry for processes started by `StartProcess` actions —
//! deliberately separate from `script_runner_panel.rs`'s `ScriptRunner`
//! (a different lifecycle model built around commands expected to exit;
//! reusing it would mix "a workflow started this and it's meant to keep
//! running" with "a user ran this and it's meant to finish"). A process
//! registered here survives past the `StartProcess` action's own
//! `execute_leaf` call returning — that's the whole point, a dev server
//! doesn't exit — until something calls `stop`.
//!
//! A single process-wide singleton (`registry()`), not threaded through
//! every executor call — `StartProcess` is the only action type that needs
//! it, and adding a parameter to `execute_leaf`/`execute_action`'s entire
//! call chain for one action type would ripple through code that has
//! nothing to do with process tracking. The Designer's "Processes" panel
//! reads the same singleton directly.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, LazyLock, Mutex};

use tokio::process::Child;
use tokio::sync::broadcast;

/// Bounds each process's own retained log buffer (`Entry::log`) — a dev
/// server can run (and print) indefinitely, so this can't be unbounded.
/// Oldest lines drop first once full; the live `log_tx` broadcast (see
/// `subscribe_log`) is unaffected by this cap, only the snapshot
/// (`log_snapshot`, read when a log viewer first opens) is.
const LOG_CAPACITY: usize = 1000;

/// A live handle plus display metadata for one started process.
struct Entry {
    child: Child,
    info: ProcessInfo,
    log: VecDeque<String>,
    /// Broadcasts each new line as it arrives — a log viewer already open
    /// subscribes to this instead of re-polling `log_snapshot`. Lagged
    /// receivers (viewer fell behind) just miss the oldest few lines, same
    /// trade-off every other broadcast channel in this codebase accepts.
    log_tx: broadcast::Sender<String>,
}

/// Snapshot for display — no process handle, safe to clone into a UI list.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub id: String,
    pub action_id: String,
    pub command: String,
    pub args: Vec<String>,
    pub pid: Option<u32>,
    /// From `StartProcess`'s optional `url` input — purely informational
    /// (see the registry's doc comment on that field), shown in the
    /// Processes panel with a copy-to-clipboard button when present.
    pub url: Option<String>,
    /// RFC 3339.
    pub started_at: String,
    pub status: ProcessStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Running,
    /// Exited on its own (crashed, or a command that wasn't actually
    /// long-running) — distinct from `Stopped` (killed via `stop`) so the
    /// UI can tell "it died" from "you killed it."
    Exited(i32),
    Stopped,
}

#[derive(Default)]
pub struct ProcessRegistry {
    entries: Mutex<HashMap<String, Entry>>,
}

static REGISTRY: LazyLock<Arc<ProcessRegistry>> =
    LazyLock::new(|| Arc::new(ProcessRegistry::default()));

pub fn registry() -> Arc<ProcessRegistry> {
    REGISTRY.clone()
}

impl ProcessRegistry {
    /// Takes ownership of `child` — it lives here until `stop` or the
    /// process exits on its own. Returns the freshly-minted tracking id
    /// (not the OS pid, which some platforms/processes may not expose).
    pub fn register(
        &self,
        action_id: String,
        command: String,
        args: Vec<String>,
        url: Option<String>,
        child: Child,
    ) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let info = ProcessInfo {
            id: id.clone(),
            action_id,
            pid: child.id(),
            command,
            args,
            url,
            started_at: chrono::Utc::now().to_rfc3339(),
            status: ProcessStatus::Running,
        };
        let (log_tx, _) = broadcast::channel(256);
        self.entries.lock().unwrap().insert(
            id.clone(),
            Entry {
                child,
                info,
                log: VecDeque::new(),
                log_tx,
            },
        );
        id
    }

    /// Appends one line to `id`'s retained log (dropping the oldest once
    /// [`LOG_CAPACITY`] is hit) and broadcasts it to any live subscriber.
    /// Silently a no-op for an unknown/since-removed id — a reader task
    /// racing the process's own removal isn't an error condition, just a
    /// line with nowhere left to go.
    pub fn append_log(&self, id: &str, line: String) {
        let entries = self.entries.lock().unwrap();
        if let Some(entry) = entries.get(id) {
            // `log_tx.send` before touching `log` so the receiver side (a
            // `broadcast::Receiver` per subscriber) never observes the
            // snapshot including a line it's also about to receive live —
            // matters only if a subscriber calls `log_snapshot` again after
            // subscribing, which the log viewer's own open sequence
            // deliberately does the other order around (snapshot first,
            // subscribe second) specifically to avoid that overlap.
            let _ = entry.log_tx.send(line.clone());
            drop(entries);
            let mut entries = self.entries.lock().unwrap();
            if let Some(entry) = entries.get_mut(id) {
                if entry.log.len() >= LOG_CAPACITY {
                    entry.log.pop_front();
                }
                entry.log.push_back(line);
            }
        }
    }

    /// The lines retained for `id` so far, oldest-first — the initial fill
    /// for a log viewer, read once when it opens; call [`subscribe_log`]
    /// right after to keep receiving new lines live.
    pub fn log_snapshot(&self, id: &str) -> Vec<String> {
        self.entries
            .lock()
            .unwrap()
            .get(id)
            .map(|e| e.log.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Subscribes to `id`'s new log lines as they arrive. `None` for an
    /// unknown id (process was never tracked, or the registry was cleared —
    /// doesn't happen today, but this stays `Option` rather than panicking
    /// either way).
    pub fn subscribe_log(&self, id: &str) -> Option<broadcast::Receiver<String>> {
        self.entries
            .lock()
            .unwrap()
            .get(id)
            .map(|e| e.log_tx.subscribe())
    }

    /// A live snapshot of every tracked process, oldest-first — refreshes
    /// each entry's `status` via a non-blocking `try_wait()` first, so a
    /// process that quietly exited on its own shows as `Exited`, not
    /// stale `Running`, the next time this is called (no background
    /// poller needed — the Designer's Processes panel calls this whenever
    /// it renders).
    pub fn list(&self) -> Vec<ProcessInfo> {
        let mut entries = self.entries.lock().unwrap();
        for entry in entries.values_mut() {
            if entry.info.status == ProcessStatus::Running
                && let Ok(Some(exit_status)) = entry.child.try_wait()
            {
                entry.info.status = ProcessStatus::Exited(exit_status.code().unwrap_or(-1));
            }
        }
        let mut infos: Vec<ProcessInfo> = entries.values().map(|e| e.info.clone()).collect();
        infos.sort_by(|a, b| a.started_at.cmp(&b.started_at));
        infos
    }

    /// Kills the process and marks it `Stopped`. `Ok(())` even if it had
    /// already exited on its own (nothing left to kill isn't an error —
    /// the caller just wanted it gone, and it already is); `Err` only for
    /// an unknown id or a real OS-level kill failure.
    ///
    /// On Windows, the *tracked* child is `cmd.exe`, not the real process
    /// (`StartProcess` routes through it so `.cmd`/`.bat` tools like `npm`
    /// can launch at all — see `execute_start_process`'s doc comment), so
    /// `entry.child.start_kill()` alone would kill an empty shell and leave
    /// its actual descendant (node.exe, python.exe, ...) running orphaned.
    /// `taskkill /T` kills the whole tree rooted at the tracked pid first —
    /// same technique `script_runner_panel::kill_process_tree` already uses
    /// for its own shelled-out commands.
    ///
    /// Elsewhere the tracked process is the real one, but it can have started
    /// others (`npm run dev` starts the server as its child), which a plain
    /// kill would leave running. It is started as the leader of its own
    /// process group, and the whole group is killed.
    pub fn stop(&self, id: &str) -> Result<(), String> {
        let mut entries = self.entries.lock().unwrap();
        let entry = entries
            .get_mut(id)
            .ok_or_else(|| format!("no tracked process with id \"{id}\""))?;

        if let Some(pid) = entry.info.pid {
            kill_process_tree(pid);
        }

        match entry.child.start_kill() {
            Ok(()) => {
                entry.info.status = ProcessStatus::Stopped;
                Ok(())
            }
            // `start_kill` errors (only) when the process has already
            // exited — not a real failure from this call's point of view.
            // Also the expected outcome on Windows once `kill_process_tree`
            // above already reaped it.
            Err(_) => {
                entry.info.status = ProcessStatus::Exited(-1);
                Ok(())
            }
        }
    }
}

#[cfg(target_os = "windows")]
#[allow(
    clippy::disallowed_methods,
    reason = "waits for taskkill, which returns at once; `stop` is not called on the UI thread"
)]
fn kill_process_tree(pid: u32) {
    use std::os::windows::process::CommandExt;
    let mut c = std::process::Command::new("taskkill.exe");
    c.args(["/F", "/T", "/PID", &pid.to_string()]);
    c.creation_flags(crate::async_rt::CREATE_NO_WINDOW);
    let _ = c.output();
}

/// `pid` leads its own process group (`execute_start_process` starts it that
/// way), so a negative id signals the whole group. A process that already
/// exited leaves no group, and `start_kill` in `stop` covers what is left.
#[cfg(not(target_os = "windows"))]
#[allow(
    clippy::disallowed_methods,
    reason = "waits for kill, which returns at once; `stop` is not called on the UI thread"
)]
fn kill_process_tree(pid: u32) {
    let _ = std::process::Command::new("kill")
        .args(["-9", "--", &format!("-{pid}")])
        .output();
}

/// Spawns a background task that reads `stream` line-by-line and appends
/// each line to `id`'s log (`ProcessRegistry::append_log`), prefixing
/// stderr lines with `"! "` so a log viewer can tell the two apart without
/// needing separate colored panes. Detached — same "outlives the caller"
/// lifetime as the tracked child itself (`register`'s own doc comment);
/// the loop ends naturally once the pipe closes, which happens when the
/// process exits (or is killed — `ProcessRegistry::stop`'s tree-kill closes
/// the pipe same as a normal exit does). Called by `execute_start_process`
/// once per stream (stdout and stderr both), right after `register` hands
/// back the tracking id.
pub fn spawn_log_reader<R>(id: String, stream: R, is_stderr: bool)
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    use tokio::io::{AsyncBufReadExt, BufReader};
    tokio::spawn(async move {
        let mut lines = BufReader::new(stream).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let line = if is_stderr { format!("! {line}") } else { line };
            registry().append_log(&id, line);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Stdio;

    fn spawn_sleep(seconds: u64) -> Child {
        #[cfg(target_os = "windows")]
        let mut cmd = tokio::process::Command::new("cmd");
        #[cfg(target_os = "windows")]
        cmd.args(["/C", &format!("timeout /T {seconds}")]);
        #[cfg(not(target_os = "windows"))]
        let mut cmd = tokio::process::Command::new("sleep");
        #[cfg(not(target_os = "windows"))]
        cmd.arg(seconds.to_string());
        cmd.stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        cmd.spawn()
            .expect("spawn a short-lived process for testing")
    }

    #[tokio::test]
    async fn register_then_list_shows_it_running() {
        let registry = ProcessRegistry::default();
        let child = spawn_sleep(5);
        let id = registry.register(
            "start1".to_string(),
            "sleep".to_string(),
            vec!["5".to_string()],
            None,
            child,
        );

        let list = registry.list();
        let entry = list.iter().find(|p| p.id == id).unwrap();
        assert_eq!(entry.status, ProcessStatus::Running);
        assert_eq!(entry.action_id, "start1");
    }

    #[tokio::test]
    async fn stop_kills_it_and_marks_stopped() {
        let registry = ProcessRegistry::default();
        let child = spawn_sleep(30);
        let id = registry.register(
            "start1".to_string(),
            "sleep".to_string(),
            vec!["30".to_string()],
            None,
            child,
        );

        registry.stop(&id).unwrap();
        let list = registry.list();
        let entry = list.iter().find(|p| p.id == id).unwrap();
        assert_eq!(entry.status, ProcessStatus::Stopped);
    }

    #[tokio::test]
    async fn stopping_an_unknown_id_is_an_error() {
        let registry = ProcessRegistry::default();
        assert!(registry.stop("never-existed").is_err());
    }

    #[tokio::test]
    async fn list_notices_a_process_that_exited_on_its_own() {
        let registry = ProcessRegistry::default();
        // A process that finishes almost immediately.
        #[cfg(target_os = "windows")]
        let mut cmd = tokio::process::Command::new("cmd");
        #[cfg(target_os = "windows")]
        cmd.args(["/C", "exit 0"]);
        #[cfg(not(target_os = "windows"))]
        let mut cmd = tokio::process::Command::new("true");
        cmd.stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        let child = cmd.spawn().unwrap();

        let id = registry.register(
            "start1".to_string(),
            "true".to_string(),
            vec![],
            None,
            child,
        );
        // Give it a moment to actually exit before we check.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let list = registry.list();
        let entry = list.iter().find(|p| p.id == id).unwrap();
        assert!(
            matches!(entry.status, ProcessStatus::Exited(_)),
            "expected Exited, got {:?}",
            entry.status
        );
    }

    #[tokio::test]
    async fn append_log_lands_in_the_snapshot_in_order() {
        let registry = ProcessRegistry::default();
        let id = registry.register(
            "start1".to_string(),
            "sleep".to_string(),
            vec![],
            None,
            spawn_sleep(5),
        );

        registry.append_log(&id, "line one".to_string());
        registry.append_log(&id, "line two".to_string());

        assert_eq!(
            registry.log_snapshot(&id),
            vec!["line one".to_string(), "line two".to_string()]
        );
    }

    #[tokio::test]
    async fn append_log_for_an_unknown_id_is_a_silent_no_op() {
        let registry = ProcessRegistry::default();
        // Just must not panic — nothing to assert beyond that.
        registry.append_log("never-registered", "line".to_string());
    }

    #[tokio::test]
    async fn log_snapshot_for_an_unknown_id_is_empty() {
        let registry = ProcessRegistry::default();
        assert!(registry.log_snapshot("never-registered").is_empty());
    }

    #[tokio::test]
    async fn subscribe_log_receives_lines_appended_after_subscribing() {
        let registry = ProcessRegistry::default();
        let id = registry.register(
            "start1".to_string(),
            "sleep".to_string(),
            vec![],
            None,
            spawn_sleep(5),
        );

        let mut rx = registry.subscribe_log(&id).unwrap();
        registry.append_log(&id, "live line".to_string());

        let received = rx.recv().await.unwrap();
        assert_eq!(received, "live line");
    }

    #[tokio::test]
    async fn subscribe_log_for_an_unknown_id_is_none() {
        let registry = ProcessRegistry::default();
        assert!(registry.subscribe_log("never-registered").is_none());
    }

    #[tokio::test]
    async fn appending_past_capacity_drops_the_oldest_line_first() {
        let registry = ProcessRegistry::default();
        let id = registry.register(
            "start1".to_string(),
            "sleep".to_string(),
            vec![],
            None,
            spawn_sleep(5),
        );

        for i in 0..(LOG_CAPACITY + 5) {
            registry.append_log(&id, format!("line {i}"));
        }

        let snapshot = registry.log_snapshot(&id);
        assert_eq!(snapshot.len(), LOG_CAPACITY);
        assert_eq!(snapshot.first().unwrap(), "line 5");
        assert_eq!(
            snapshot.last().unwrap(),
            &format!("line {}", LOG_CAPACITY + 4)
        );
    }
}
