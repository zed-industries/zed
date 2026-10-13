//! S5 exit: the worker worktrees the panel shows, and the four things a
//! person does with them (preview, merge back, remove, clean up).
//!
//! The mechanism is [`cedian_worker`]'s; this is the panel's view of it plus
//! the registry bookkeeping each action needs, so the app never leaves a
//! `workers.json` row naming a tree that is gone, nor one that says `done`
//! before the merge landed.

use cedian_worker::{MergePlan, Registry, TreeState, WorkerError, WorkerHead, WorkerStatus};
use std::path::Path;

/// One live cedian worktree as the panel shows it: git's state, the registry
/// row when there is one, and whether that row's worker has gone quiet.
#[derive(Debug, Clone)]
pub struct WorkerTreeRow {
    pub state: TreeState,
    pub head: Option<WorkerHead>,
    pub stuck: bool,
}

/// Every cedian worktree in `repo`, joined with its registry row by id.
///
/// Git is the source of truth (§43 R1): a tree with no row still appears, and
/// a row whose tree is gone is left to `cleanup` — the panel shows what
/// exists, never what the registry remembers.
pub fn read(repo: &Path, state: &Path, base: &str) -> Result<Vec<WorkerTreeRow>, String> {
    let trees = cedian_worker::trees(repo, base).map_err(|e| e.to_string())?;
    let (registry, _) = Registry::open(state).map_err(|e| e.to_string())?;
    let now = cedian_worker::now_ms();
    Ok(trees
        .into_iter()
        .map(|state| {
            let head = registry.get(&state.id).cloned();
            let stuck = head.as_ref().is_some_and(|head| head.is_stuck(now, repo));
            WorkerTreeRow { state, head, stuck }
        })
        .collect())
}

/// Mark `id` complete after its brief's verify items have fresh attributed
/// `pass` evidence (ADR-0033 decision 4). Sets `Done` and notes progress.
pub fn complete(
    state: &Path,
    id: &str,
    evidence: &[cedian_workflow::Evidence],
    current: &cedian_workflow::CurrentState,
) -> Result<(), String> {
    let brief = cedian_worker::Brief::latest(state, id).map_err(|e| e.to_string())?;
    let missing = crate::worker_complete::missing_verify(&brief, evidence, current);
    if !missing.is_empty() {
        return Err(format!(
            "worker {id} is not complete:\n  - {}",
            missing.join("\n  - ")
        ));
    }
    let _lock = Registry::lock(state).map_err(|e| e.to_string())?;
    let (mut registry, _) = Registry::open(state).map_err(|e| e.to_string())?;
    let head = row(&registry, id)?;
    if head.status == WorkerStatus::Done {
        return Ok(());
    }
    registry
        .set_status(id, WorkerStatus::Done, String::new())
        .map_err(|e| e.to_string())?;
    registry.note_progress(id).map_err(|e| e.to_string())?;
    registry.save(state).map_err(|e| e.to_string())
}

/// Merge `id`'s branch into `base`. Refused unless the worker is already
/// `Done` (ADR-0033 decision 4); a refusal (STALE) leaves the row as it was.
pub fn merge_back(repo: &Path, state: &Path, id: &str, base: &str) -> Result<(), String> {
    let _lock = Registry::lock(state).map_err(|e| e.to_string())?;
    let (registry, _) = Registry::open(state).map_err(|e| e.to_string())?;
    let head = row(&registry, id)?;
    if head.status != WorkerStatus::Done {
        return Err(WorkerError::NotComplete(id.to_string()).to_string());
    }
    cedian_worker::merge_back(repo, &head, base).map_err(|e| e.to_string())
}

/// Remove `id`'s worktree and branch, then drop its row. Git refuses a tree
/// with uncommitted or unmerged work, and the row stays so the refusal is what
/// the person sees — never a tree quietly thrown away.
pub fn remove(repo: &Path, state: &Path, id: &str) -> Result<(), String> {
    let _lock = Registry::lock(state).map_err(|e| e.to_string())?;
    let (mut registry, _) = Registry::open(state).map_err(|e| e.to_string())?;
    let head = row(&registry, id)?;
    cedian_worker::remove(repo, &head).map_err(|e| e.to_string())?;
    registry.remove(id);
    registry.save(state).map_err(|e| e.to_string())
}

/// Clean up `id`, this call being its explicit per-tree decision (§43: a `Wip`
/// tree is never removed without one). Only that tree goes: a person who
/// clicked one row's Cleanup did not decide anything about the others. The row
/// goes with the tree.
pub fn cleanup(repo: &Path, state: &Path, id: &str, base: &str) -> Result<String, String> {
    let outcome = cedian_worker::cleanup_one(repo, base, id, true).map_err(|e| e.to_string())?;
    let said = if outcome.removed {
        let _lock = Registry::lock(state).map_err(|e| e.to_string())?;
        let (mut registry, _) = Registry::open(state).map_err(|e| e.to_string())?;
        if registry.remove(id).is_some() {
            registry.save(state).map_err(|e| e.to_string())?;
        }
        format!("removed {id}")
    } else {
        format!("{id}: {}", outcome.reason)
    };
    Ok(said)
}

/// What merging `id` into `base` would come to, without touching the repo: a
/// throwaway clone does the trial merge, squeezed into one line for the notice.
/// The row is needed for its branch, so a tree without one is refused exactly
/// as the other actions refuse it.
pub fn preview(repo: &Path, state: &Path, id: &str, base: &str) -> Result<String, String> {
    let (registry, _) = Registry::open(state).map_err(|e| e.to_string())?;
    let head = row(&registry, id)?;
    let plan = cedian_worker::merge_preview(repo, &head, base).map_err(|e| e.to_string())?;
    Ok(said(id, &plan))
}

/// The preview line: how many files the merge would touch, the clean ones by
/// name, then the conflicts, in the words the trial merge reports.
fn said(id: &str, plan: &MergePlan) -> String {
    let touched = plan.clean.len() + plan.conflicted.len();
    let mut said = format!("{id}: a merge would touch {touched} file(s)");
    if !plan.clean.is_empty() {
        said.push_str(&format!("; clean: {}", plan.clean.join(", ")));
    }
    if !plan.conflicted.is_empty() {
        said.push_str(&format!(
            "; {} conflicted(STALE): {}",
            plan.conflicted.len(),
            plan.conflicted.join(", ")
        ));
    }
    said
}

/// The row an action needs; a tree without one is refused naming the id.
fn row(registry: &Registry, id: &str) -> Result<WorkerHead, String> {
    registry
        .get(id)
        .cloned()
        .ok_or_else(|| WorkerError::NoSuch(id.to_string()).to_string())
}
