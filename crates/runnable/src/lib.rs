//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
mod static_runnable;
mod static_runner;

use anyhow::Result;
use core::future::Future;
use futures::future::BoxFuture;
pub use futures::stream::Aborted as TaskTerminated;
use futures::stream::{AbortHandle, Abortable};
use futures::FutureExt;
use gpui::{AsyncWindowContext, Task};
pub use static_runner::StaticRunner;

pub struct TaskHandle {
    fut: Task<Result<ExecutionResult, TaskTerminated>>,
    cancel_token: AbortHandle,
}

impl TaskHandle {
    pub fn new(fut: BoxFuture<'static, ExecutionResult>, cx: AsyncWindowContext) -> Result<Self> {
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
    fn exec(self, cx: gpui::AsyncWindowContext) -> Result<TaskHandle>;
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}
