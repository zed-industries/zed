//! Shared, thread-safe run state for a designer flow run.
//!
//! `workflow_engine::StatusSink` is a bare `Arc<dyn Fn + Send + Sync>`
//! callback invoked from whatever tokio/gpui thread drives the run, so the
//! per-action status events can't touch the UI thread directly. Both the
//! Designer tab and the "Run Results" tab instead share one
//! [`Arc<Mutex<RunState>>`] — the sink fills it, and a UI-side poll task
//! (see `DesignerPanel::start_run_poll` / `RunResults::start_poll`)
//! snapshots it onto the panel a few times a second. That keeps the engine
//! gpui-free (no change to `workflow_engine`) and the UI reactive enough
//! for a live "which action is running" indicator.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use workflow_engine::{IterationScope, RunOutcome};

/// One action's live phase, mirroring `workflow_engine::RunStatus` but
/// owned by this crate so the UI can style it without pulling the engine's
/// enum through `serde`/rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunPhase {
    Running,
    Succeeded,
    Failed,
    Skipped,
}

impl RunPhase {
    pub fn label(self) -> &'static str {
        match self {
            RunPhase::Running => "Running",
            RunPhase::Succeeded => "Succeeded",
            RunPhase::Failed => "Failed",
            RunPhase::Skipped => "Skipped",
        }
    }
}

/// Everything the UI knows about a single action's status during/after a
/// run. Keyed by the action's id — note a `Foreach` body action can be
/// reported `Running`/`Succeeded` multiple times; the map keeps only the
/// latest entry while [`RunLogLine`] keeps every transition's history.
#[derive(Debug, Clone)]
pub struct ActionRun {
    pub name: String,
    pub type_id: String,
    pub phase: RunPhase,
    /// For `Succeeded`: the serialized `outputs` map (empty when the
    /// executor returned no outputs). For `Failed`: the error message.
    /// `None` for `Running`/`Skipped`.
    pub detail: Option<String>,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
}

impl ActionRun {
    pub fn elapsed(&self) -> Option<Duration> {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(finish)) => Some(finish.saturating_duration_since(start)),
            (Some(start), None) => Some(start.elapsed()),
            _ => None,
        }
    }
}

/// One entry in the chronological run transcript.
#[derive(Debug, Clone)]
pub struct RunLogLine {
    /// Milliseconds since the run started.
    pub time_ms: u128,
    pub action_id: String,
    pub name: String,
    pub type_id: String,
    pub phase: RunPhase,
    pub detail: Option<String>,
}

/// What a run recorded about one `Foreach`: how many items it had, and the
/// item each iteration was bound to.
#[derive(Debug, Clone, Default)]
pub struct LoopRun {
    pub total: usize,
    /// Keyed by iteration index. Filled as iterations start, so an entry can
    /// be missing while the loop is still running.
    pub items: HashMap<usize, serde_json::Value>,
}

/// Key for the results of one pass through a loop body: each enclosing
/// `Foreach` and the iteration index inside it, outermost first, as
/// `outer:2/inner:0`. Empty for an action outside any loop.
pub fn scope_key(scope: &[(String, usize)]) -> String {
    scope
        .iter()
        .map(|(container_id, index)| format!("{container_id}:{index}"))
        .collect::<Vec<_>>()
        .join("/")
}

/// The full, lock-held state of one run. Written by the `StatusSink`
/// closure (possibly off the UI thread), read by UI poll tasks.
#[derive(Debug, Clone, Default)]
pub struct RunState {
    /// The latest result per action. For a loop body that is whichever
    /// iteration reported last; `iterations` has each one.
    pub nodes: HashMap<String, ActionRun>,
    /// Loop-body results kept apart per iteration: [`scope_key`] → action id
    /// → result.
    pub iterations: HashMap<String, HashMap<String, ActionRun>>,
    /// Each `Foreach`'s item list, keyed by the [`scope_key`] of the
    /// iterations enclosing it and then its id (`"/each"` at top level,
    /// `"outer:2/inner"` nested), since an inner loop runs once per outer
    /// iteration.
    pub loops: HashMap<String, LoopRun>,
    pub log: Vec<RunLogLine>,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub outcome: Option<RunOutcome>,
    pub error: Option<String>,
    /// Bumped by every mutation so UI poll tasks can cheaply detect "new
    /// events since my last snapshot" without `PartialEq` on `Instant`s.
    pub revision: u64,
}

impl RunState {
    /// `scope` is the `Foreach` iterations the action ran inside, outermost
    /// first, and empty outside any loop.
    pub fn record(
        &mut self,
        run_start: Instant,
        scope: &[IterationScope],
        action_id: &str,
        phase: RunPhase,
        detail: Option<String>,
        name: String,
        type_id: String,
    ) {
        let now = Instant::now();
        let update = |entry: &mut ActionRun| {
            entry.name = name.clone();
            entry.type_id = type_id.clone();
            entry.phase = phase;
            entry.detail = detail.clone();
            match phase {
                RunPhase::Running => entry.started_at = Some(now),
                RunPhase::Succeeded | RunPhase::Failed | RunPhase::Skipped => {
                    entry.finished_at = Some(now);
                }
            }
        };
        let blank = || ActionRun {
            name: name.clone(),
            type_id: type_id.clone(),
            phase,
            detail: None,
            started_at: None,
            finished_at: None,
        };
        update(
            self.nodes
                .entry(action_id.to_string())
                .or_insert_with(blank),
        );

        let path: Vec<(String, usize)> = scope
            .iter()
            .map(|iteration| (iteration.container_id.clone(), iteration.index))
            .collect();
        if !path.is_empty() {
            update(
                self.iterations
                    .entry(scope_key(&path))
                    .or_default()
                    .entry(action_id.to_string())
                    .or_insert_with(blank),
            );
        }
        for (depth, iteration) in scope.iter().enumerate() {
            let key = format!("{}/{}", scope_key(&path[..depth]), iteration.container_id);
            let entry = self.loops.entry(key).or_default();
            entry.total = iteration.total;
            entry
                .items
                .entry(iteration.index)
                .or_insert_with(|| iteration.item.clone());
        }

        // The transcript names the iteration, so a body's lines can be told
        // apart: `Charge [2/5]`.
        let log_name = match scope.last() {
            Some(iteration) => {
                format!("{name} [{}/{}]", iteration.index + 1, iteration.total)
            }
            None => name.clone(),
        };
        self.log.push(RunLogLine {
            time_ms: now.duration_since(run_start).as_millis(),
            action_id: action_id.to_string(),
            name: log_name,
            type_id: type_id.clone(),
            phase,
            detail: detail.clone(),
        });
        self.revision += 1;
    }

    /// `(succeeded, failed, skipped)` counts for the summary header.
    pub fn totals(&self) -> (u32, u32, u32) {
        let mut succeeded = 0;
        let mut failed = 0;
        let mut skipped = 0;
        for run in self.nodes.values() {
            match run.phase {
                RunPhase::Succeeded => succeeded += 1,
                RunPhase::Failed => failed += 1,
                RunPhase::Skipped => skipped += 1,
                RunPhase::Running => {}
            }
        }
        (succeeded, failed, skipped)
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(finish)) => Some(finish.saturating_duration_since(start)),
            (Some(start), None) => Some(start.elapsed()),
            _ => None,
        }
    }
}

pub type SharedRunState = Arc<Mutex<RunState>>;

/// Path-keyed registry of the *live* run state, so a run-results tab can
/// find the run it belongs to without the designer handing it a handle. The
/// Designer registers its `SharedRunState` on open and drops it on close;
/// `run_state_for` falls back to a fresh empty state (a "no run yet" tab).
static RUN_STATES: OnceLock<Arc<Mutex<HashMap<PathBuf, SharedRunState>>>> = OnceLock::new();

pub fn register_run_state(path: PathBuf, state: SharedRunState) {
    RUN_STATES
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
        .lock()
        .unwrap()
        .insert(path, state);
}

pub fn unregister_run_state(path: &Path) {
    if let Some(states) = RUN_STATES.get() {
        states.lock().unwrap().remove(path);
    }
}

pub fn run_state_for(path: &Path) -> SharedRunState {
    RUN_STATES
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
        .lock()
        .unwrap()
        .get(path)
        .cloned()
        .unwrap_or_else(|| Arc::new(Mutex::new(RunState::default())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iteration(container_id: &str, index: usize, total: usize) -> IterationScope {
        IterationScope {
            container_id: container_id.to_string(),
            index,
            total,
            item: serde_json::json!({ "n": index }),
        }
    }

    fn record(state: &mut RunState, scope: &[IterationScope], action_id: &str, detail: &str) {
        state.record(
            Instant::now(),
            scope,
            action_id,
            RunPhase::Succeeded,
            Some(detail.to_string()),
            action_id.to_string(),
            "transform".to_string(),
        );
    }

    /// A loop body reports under one id every iteration; each iteration's
    /// result has to survive the next one, alongside the item it ran for.
    #[test]
    fn loop_iterations_are_kept_apart() {
        let mut state = RunState::default();
        record(&mut state, &[], "fetch", "top");
        for index in 0..3 {
            record(
                &mut state,
                &[iteration("each", index, 3)],
                "shape",
                &format!("pass {index}"),
            );
        }

        // Outside a loop: one result, nothing per iteration.
        assert_eq!(state.nodes["fetch"].detail.as_deref(), Some("top"));
        assert!(!state.iterations.values().any(|run| run.contains_key("fetch")));

        // The latest still wins the per-action slot the canvas colours from.
        assert_eq!(state.nodes["shape"].detail.as_deref(), Some("pass 2"));
        for index in 0..3 {
            let key = scope_key(&[("each".to_string(), index)]);
            assert_eq!(
                state.iterations[&key]["shape"].detail.as_deref(),
                Some(format!("pass {index}").as_str())
            );
        }
        let each = &state.loops["/each"];
        assert_eq!(each.total, 3);
        assert_eq!(each.items[&1], serde_json::json!({ "n": 1 }));

        // The transcript says which pass a line belongs to.
        assert_eq!(state.log.last().unwrap().name, "shape [3/3]");
    }

    /// An inner loop runs once per outer iteration, so its items are kept
    /// per outer pass.
    #[test]
    fn nested_loops_are_keyed_by_the_outer_iteration() {
        let mut state = RunState::default();
        record(
            &mut state,
            &[iteration("outer", 1, 2), iteration("inner", 0, 5)],
            "leaf",
            "x",
        );
        assert_eq!(scope_key(&[("outer".into(), 1), ("inner".into(), 0)]), "outer:1/inner:0");
        assert!(state.iterations["outer:1/inner:0"].contains_key("leaf"));
        assert_eq!(state.loops["/outer"].total, 2);
        assert_eq!(state.loops["outer:1/inner"].total, 5);
    }
}
