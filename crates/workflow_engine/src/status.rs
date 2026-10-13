//! §6 of `DOCS/workflow-schema/execution-engine-design.md` — per-action
//! status transitions, threaded through `execute_action` and every
//! container executor so a UI layer can render progress live. Deliberately
//! just a callback (`StatusSink`), not a broadcast channel keyed by
//! `(run_id, action_id)` — this module stays gpui-free (see the parent
//! module's own doc: "pure data/logic, no gpui dependency"); the caller
//! (`ForgeShell`/`DesignerPanel`) decides how to fan updates out to the
//! canvas, and for a single Designer tab running its own flow, a direct
//! callback into an mpsc channel is simpler than AppState-level broadcast
//! infrastructure with no second subscriber to justify it yet.

use std::sync::Arc;

use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Running,
    Succeeded,
    Failed,
    Skipped,
}

/// One `Foreach` iteration a status report was made inside.
///
/// A loop body's actions run once per item under the same action ids, so the
/// id alone can't say which run of the body a report belongs to. A report
/// carries one of these per enclosing loop, outermost first.
#[derive(Debug, Clone, PartialEq)]
pub struct IterationScope {
    /// Id of the `Foreach` action.
    pub container_id: String,
    /// Zero-based position of this iteration's item in the list.
    pub index: usize,
    /// How many items the list has.
    pub total: usize,
    /// The item this iteration is bound to (`$item` in its expressions).
    pub item: Value,
}

type Callback = dyn Fn(&[IterationScope], &str, RunStatus, Option<String>) + Send + Sync;

/// Cheap to `Clone` (an `Arc` underneath) so every recursive
/// `execute_action`/container-executor call can carry its own copy without
/// threading a reference through every lifetime in `container.rs`.
#[derive(Clone)]
pub struct StatusSink {
    callback: Option<Arc<Callback>>,
    /// The loop iterations this sink reports from inside, outermost first.
    scope: Arc<[IterationScope]>,
}

impl StatusSink {
    /// For callers that don't care about status — every existing engine
    /// test uses this, so adding live status didn't require touching any
    /// of them beyond passing this in.
    pub fn none() -> Self {
        Self {
            callback: None,
            scope: Arc::from([]),
        }
    }

    /// A sink that is told the action and its status, and not which loop
    /// iteration it ran in. Reports from every iteration of a loop body
    /// arrive under the same action id.
    pub fn new(f: impl Fn(&str, RunStatus, Option<String>) + Send + Sync + 'static) -> Self {
        Self::with_scope(move |_scope, action_id, status, detail| f(action_id, status, detail))
    }

    /// A sink that is also told the enclosing `Foreach` iterations, so it
    /// can keep each iteration's results apart. The slice is empty for an
    /// action outside any loop.
    pub fn with_scope(
        f: impl Fn(&[IterationScope], &str, RunStatus, Option<String>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            callback: Some(Arc::new(f)),
            scope: Arc::from([]),
        }
    }

    /// The sink a loop hands to one iteration of its body: the same
    /// callback, with `iteration` added as the innermost scope.
    pub fn in_iteration(&self, iteration: IterationScope) -> Self {
        let mut scope = self.scope.to_vec();
        scope.push(iteration);
        Self {
            callback: self.callback.clone(),
            scope: scope.into(),
        }
    }

    /// `detail` is the human-readable "what actually happened" a UI log
    /// wants — the serialized `outputs` map for a `Succeeded` leaf, the
    /// error message for `Failed`, `None` for `Running`/`Skipped` (nothing
    /// to show yet).
    pub fn report(&self, action_id: &str, status: RunStatus, detail: Option<String>) {
        if let Some(f) = &self.callback {
            f(&self.scope, action_id, status, detail);
        }
    }
}
