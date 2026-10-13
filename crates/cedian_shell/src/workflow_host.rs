//! The workflow channel as a host runs it (ADR-0022, ADR-0055): its store
//! in the workspace's state dir, evidence bound to OMP's finished calls in
//! the router log (ADR-0031), and the workspace hashed now as the code
//! state (ADR-0024; row H until evidence binds to buffer versions).

use crate::workflow_store::{self, DiskWorkflowStore};
use crate::{corrections, verify_store, workspace_files};
use cedian_omp::FinishedToolCall;
use cedian_workflow::{BoundCall, CurrentState, WorkflowChannel, WorkflowStatus};
use std::path::Path;
use std::sync::Arc;

/// The channel for `task` over the workspace at `workdir`, with the
/// user's floor and the project's verification profiles. `resolve` binds
/// reported evidence to a finished tool call. `adjust` adds what the
/// disk does not hold: the app's shared browser frame (evidence from an
/// earlier frame reads `stale-frame`) and its unsaved buffers.
pub fn channel(
    task: &str,
    workdir: &Path,
    state_dir: &Path,
    settings: &crate::Settings,
    resolve: impl Fn(&str, &str) -> Option<BoundCall> + Send + Sync + 'static,
    adjust: impl Fn(&mut CurrentState) + Send + Sync + 'static,
) -> Arc<WorkflowChannel> {
    let root = workdir.to_path_buf();
    WorkflowChannel::with_policy(
        task,
        Box::new(DiskWorkflowStore(state_dir.to_path_buf())),
        resolve,
        move || {
            let mut state = current_state(&root);
            adjust(&mut state);
            state
        },
        settings.floor.clone(),
        Box::new(verify_store::DiskProfileStore {
            state_dir: state_dir.to_path_buf(),
            workdir: workdir.to_path_buf(),
        }),
    )
}

/// The most recent call of `tool` (args containing `needle`) that finished
/// without error and is not itself a channel report, with the first later
/// successful call that may have changed files.
pub fn bound_call(calls: &[FinishedToolCall], tool: &str, needle: &str) -> Option<BoundCall> {
    let at = calls.iter().rposition(|call| {
        !call.is_error
            && call.tool_name == tool
            && call.args_preview.contains(needle)
            && !cedian_workflow::is_channel_call(&call.tool_name, &call.args_preview)
    })?;
    // ADR-0024: a later call that may have changed files means the
    // workspace hashed now is not what this call saw. A denied or failed
    // call changed nothing (S2 finding: headless Deny of bash/eval).
    let mutated_after = calls[at + 1..]
        .iter()
        .find(|c| !c.is_error && cedian_workflow::may_mutate(&c.tool_name, &c.args_preview))
        .map(|c| format!("{} {}", c.tool_name, c.args_preview));
    let call = calls[at].clone();
    Some(BoundCall {
        tool_call_id: call.tool_call_id,
        tool_name: call.tool_name,
        args_preview: call.args_preview,
        mutated_after,
    })
}

/// The workspace hashed now. Files over the buffer cap are hashed by size
/// and mtime, not read.
pub fn current_state(workdir: &Path) -> CurrentState {
    current_state_unless(workdir, &std::sync::atomic::AtomicBool::new(false))
        .expect("never stopped")
}

/// [`current_state`], given up (`None`) as soon as `stop` is set.
pub fn current_state_unless(
    workdir: &Path,
    stop: &std::sync::atomic::AtomicBool,
) -> Option<CurrentState> {
    let stopped = || stop.load(std::sync::atomic::Ordering::Relaxed);
    let files: Vec<(String, Vec<u8>)> = workspace_files::scan_code_state_files(workdir)
        .into_iter()
        .take_while(|_| !stopped())
        .filter_map(|path| {
            let rel = path
                .strip_prefix(workdir)
                .ok()?
                .to_string_lossy()
                .into_owned();
            let meta = std::fs::metadata(&path).ok()?;
            let bytes = if meta.len() > workspace_files::MAX_FILE_BYTES {
                let mtime = meta
                    .modified()
                    .ok()?
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()?
                    .as_nanos();
                format!("{}:{mtime}", meta.len()).into_bytes()
            } else {
                std::fs::read(&path).ok()?
            };
            Some((rel, bytes))
        })
        .collect();
    if stopped() {
        return None;
    }
    Some(CurrentState::from_files(
        files
            .iter()
            .map(|(rel, bytes)| (rel.clone(), bytes.as_slice())),
    ))
}

/// §54 escalation: `why` went unanswered. A running workflow blocks at
/// its current phase and the escalation is a `continue_escalated` row;
/// outside one (the fast lane, ADR-0026) nothing changes. Returns whether
/// it blocked.
pub fn escalate(
    state_dir: &Path,
    task: &str,
    turn: Option<u32>,
    why: &str,
) -> Result<bool, String> {
    let _guard = cedian_workflow::store_lock();
    if !workflow_store::exists(state_dir) {
        return Ok(false);
    }
    let mut state = workflow_store::load(state_dir)?;
    if !state.escalate() {
        return Ok(false);
    }
    workflow_store::save(state_dir, &state)?;
    corrections::record(
        state_dir,
        task,
        corrections::CorrectionKind::ContinueEscalated,
        corrections::Event {
            turn,
            excerpt: Some(why.to_string()),
            ..corrections::Event::default()
        },
    )?;
    Ok(true)
}

/// The person's Resume of a blocked workflow (§54).
pub fn resume(state_dir: &Path) -> Result<(), String> {
    let _guard = cedian_workflow::store_lock();
    let mut state = workflow_store::load(state_dir)?;
    state.resume()?;
    workflow_store::save(state_dir, &state)
}

/// Turn boundary (ADR-0036): a refused `cedian_complete` in the turn that
/// just ended blocks the workflow (or leaves it failed when the agent
/// failed a phase) and is a `completion_refused` row for `task`'s `turn`.
/// Returns the status and the missing gates; `None` when there is nothing
/// to say (no workflow, or no refused claim).
pub fn end_turn(
    state_dir: &Path,
    task: &str,
    turn: Option<u32>,
) -> Result<Option<(WorkflowStatus, Vec<String>)>, String> {
    let _guard = cedian_workflow::store_lock();
    if !workflow_store::exists(state_dir) {
        return Ok(None);
    }
    let mut state = workflow_store::load(state_dir)?;
    let Some((status, missing)) = state.end_turn() else {
        if state.last_completion.is_some() {
            workflow_store::save(state_dir, &state)?;
        }
        return Ok(None);
    };
    corrections::record(
        state_dir,
        task,
        corrections::CorrectionKind::CompletionRefused,
        corrections::Event {
            turn,
            excerpt: Some(missing.join("\n")),
            ..corrections::Event::default()
        },
    )?;
    workflow_store::save(state_dir, &state)?;
    Ok(Some((status, missing)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cedian_omp::FinishedToolCall;
    use cedian_workflow::{TaskKind, TaskProfile, WorkflowState};

    fn finished(id: &str, tool: &str, args: &str, is_error: bool) -> FinishedToolCall {
        FinishedToolCall {
            tool_call_id: id.into(),
            tool_name: tool.into(),
            args_preview: args.into(),
            is_error,
        }
    }

    /// S2 finding / ROADMAP: a headless-denied bash after a read must not
    /// make the read's evidence born stale (it changed nothing).
    #[test]
    fn a_denied_later_call_does_not_make_evidence_born_stale() {
        let calls = [
            finished("1", "read", "notes.txt", false),
            finished("2", "bash", "sh check.sh", true),
            finished("3", "eval", "1+1", true),
        ];
        let bound = bound_call(&calls, "read", "notes").expect("the read binds");
        assert_eq!(
            bound.mutated_after, None,
            "denied bash/eval must not stale the read: {:?}",
            bound.mutated_after
        );
    }

    #[test]
    fn a_successful_later_mutator_still_stales() {
        let calls = [
            finished("1", "read", "notes.txt", false),
            finished("2", "bash", "sh check.sh", true),
            finished("3", "edit", "notes.txt", false),
        ];
        let bound = bound_call(&calls, "read", "notes").expect("the read binds");
        assert_eq!(
            bound.mutated_after.as_deref(),
            Some("edit notes.txt"),
            "a successful edit after the denied bash still stales"
        );
    }

    #[test]
    fn a_stopped_scan_gives_up() {
        let dir = crate::test_dir::TestDir::new("scan-stop");
        std::fs::write(dir.join("a.txt"), "a").unwrap();
        let stop = std::sync::atomic::AtomicBool::new(true);
        assert!(current_state_unless(&dir, &stop).is_none());
        stop.store(false, std::sync::atomic::Ordering::Relaxed);
        let state = current_state_unless(&dir, &stop).unwrap();
        assert!(state.files.contains_key("a.txt"), "{state:?}");
    }

    #[test]
    fn an_unanswered_dialog_blocks_a_running_workflow_only() {
        let dir = crate::test_dir::TestDir::new("escalate");
        assert!(
            !escalate(&dir, "panel", Some(1), "no answer").unwrap(),
            "fast lane"
        );
        assert!(!workflow_store::exists(&dir));
        let state = WorkflowState::start(TaskProfile::new("t", TaskKind::BugFix)).unwrap();
        workflow_store::save(&dir, &state).unwrap();
        assert!(escalate(&dir, "panel", Some(1), "no answer").unwrap());
        assert_eq!(
            workflow_store::load(&dir).unwrap().status,
            WorkflowStatus::Blocked
        );
        assert!(
            !escalate(&dir, "panel", Some(1), "again").unwrap(),
            "already blocked"
        );
        let rows = corrections::load(&dir).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].kind, corrections::CorrectionKind::ContinueEscalated);
    }

    /// An escalation waits for a channel update in flight instead of
    /// overwriting it.
    #[test]
    fn an_escalation_waits_for_the_workflow_lock() {
        let dir = crate::test_dir::TestDir::new("escalate-lock");
        let state = WorkflowState::start(TaskProfile::new("t", TaskKind::BugFix)).unwrap();
        workflow_store::save(&dir, &state).unwrap();
        let path = dir.to_path_buf();
        let held = cedian_workflow::store_lock();
        let (tx, rx) = std::sync::mpsc::channel();
        let escalating = std::thread::spawn(move || {
            tx.send(escalate(&path, "panel", Some(1), "no answer")).ok();
        });
        assert!(
            rx.recv_timeout(std::time::Duration::from_millis(200))
                .is_err(),
            "escalate ran while the lock was held"
        );
        drop(held);
        assert!(rx.recv().unwrap().unwrap());
        escalating.join().unwrap();
    }
}
