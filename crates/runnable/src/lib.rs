//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
mod static_runnable;

use anyhow::Result;
use core::future::Future;
use futures::future::BoxFuture;
pub use futures::stream::Aborted as TaskTerminated;
use futures::stream::{AbortHandle, AbortRegistration, Abortable};
use futures::task::{Context, Poll};
use gpui::AppContext;

struct TaskHandle {
    fut: Abortable<BoxFuture<'static, ExecutionResult>>,
    cancel_token: AbortHandle,
}

impl TaskHandle {
    pub fn new(fut: BoxFuture<'static, ExecutionResult>) -> Self {
        let (cancel_token, abort_registration) = AbortHandle::new_pair();
        let fut = Abortable::new(fut, abort_registration);
        Self { fut, cancel_token }
    }

    /// Returns a handle that can be used to cancel this task.
    pub fn termination_handle(&self) -> AbortHandle {
        self.cancel_token.clone()
    }
}

impl Future for TaskHandle {
    type Output = Result<ExecutionResult, TaskTerminated>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Future::poll(self, cx)
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

/// Represents a short lived handle to a runnable, whose main purpose
/// is to get spawned
pub trait Runnable {
    fn name(&self) -> String;
    fn exec(&mut self, cx: &mut AppContext) -> TaskHandle;
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}
