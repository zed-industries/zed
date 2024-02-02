//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
mod static_runnable_file;
mod static_runner;
mod static_source;

use anyhow::{bail, Result};
use core::future::Future;
use futures::future::BoxFuture;
pub use futures::stream::Aborted as TaskTerminated;
use futures::stream::{AbortHandle, Abortable};
use futures::FutureExt;
use gpui::{AppContext, AsyncAppContext, Model, Task};
pub use static_runner::StaticRunner;
use std::num::NonZeroU64;
use std::path::Path;
use std::sync::atomic::{self, AtomicU64, AtomicUsize};

pub struct TaskHandle {
    fut: Task<Result<ExecutionResult, TaskTerminated>>,
    cancel_token: AbortHandle,
}

impl TaskHandle {
    pub fn new(fut: BoxFuture<'static, ExecutionResult>, cx: AsyncAppContext) -> Result<Self> {
        let (cancel_token, abort_registration) = AbortHandle::new_pair();
        let fut = cx.spawn(move |_| Abortable::new(fut, abort_registration));
        Ok(Self { fut, cancel_token })
    }

    /// Returns a handle that can be used to cancel this task.
    pub fn termination_handle(&self) -> AbortHandle {
        self.cancel_token.clone()
    }
}

impl Future for TaskHandle {
    type Output = Result<ExecutionResult, TaskTerminated>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let ret = self.fut.poll_unpin(cx);
        ret
    }
}

/// Represents the result of a task.
pub struct ExecutionResult {
    /// Status of the task. Should be Ok(()) if the task succeeded, Err(()) otherwise. Note that
    /// the task might not even start up (e.g. due to a process spawning failure) and the status
    /// will still be Err().
    pub status: Result<()>,
    /// Contains user-facing details for inspection. It could be e.g. stdout/stderr of a task.
    pub details: String,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RunnableId(u64);

impl std::fmt::Display for RunnableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RunnableId> for u64 {
    fn from(value: RunnableId) -> Self {
        value.0
    }
}

/// Represents a short lived handle to a runnable, whose main purpose
/// is to get spawned
pub trait Runnable {
    fn id(&self) -> RunnableId;
    fn name(&self) -> String;
    fn exec(&mut self, cx: gpui::AsyncAppContext) -> Result<TaskHandle>;
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceId(u64);

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// runnables_for_path(..) -> [("a"), ("b")]
// schedule("a")
// runnables_for_path(..) -> [("a"), ("b")]
//
// trait Source: EventEmitter<SourceEvent> {
static SOURCE_ID: AtomicU64 = AtomicU64::new(0);

pub fn next_source_id() -> SourceId {
    SourceId(SOURCE_ID.fetch_add(1, atomic::Ordering::Relaxed))
}

pub trait Source {
    fn id(&self) -> SourceId;
    fn runnables_for_path<'a>(
        &'a self,
        path: &'a Path,
        cx: &'a mut AppContext,
    ) -> anyhow::Result<Box<dyn Iterator<Item = RunnablePebble>> + 'a>;
}

/// Uniquely represents a runnable in an inventory.
/// Two different instances of a runnable (e.g. two different runs of the same static task)
/// must have a different RunnableLens
pub struct RunnableLens {
    source_id: SourceId,
    runnable_id: RunnableId,
    display_name: String,
}

pub struct RunnablePebble {
    metadata: RunnableLens,
    state: Model<RunState>,
}

pub enum RunState {
    NotScheduled(Box<dyn Runnable>),
    AlreadyUnderway(TaskHandle),
    Done(ExecutionResult),
}
impl RunnablePebble {
    fn schedule(&self, cx: &mut AppContext) -> Result<()> {
        self.state.update(cx, |this, cx| match this {
            RunState::NotScheduled(runnable) => {
                *this = RunState::AlreadyUnderway(runnable.exec(cx.to_async())?);

                Ok(())
            }
            RunState::AlreadyUnderway(_) | RunState::Done(_) => {
                bail!(
                    "A runnable {} from source {} cannot be scheduled.",
                    self.metadata.runnable_id,
                    self.metadata.source_id.0
                );
            }
        })
    }
}
