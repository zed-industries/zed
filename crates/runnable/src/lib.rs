//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
// TODO kb revisit the visibility
pub mod static_runnable_file;
pub mod static_runner;
pub mod static_source;

use anyhow::Result;
use core::future::Future;
use futures::future::{BoxFuture, Shared};
pub use futures::stream::Aborted as TaskTerminated;
use futures::stream::{AbortHandle, Abortable};
use futures::FutureExt;
use gpui::{AppContext, AsyncAppContext, EntityId, Model, Task, WeakModel};
pub use static_runner::StaticRunner;
use std::any::Any;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use util::ResultExt as _;

/// Represents a task that's already underway. That task can be cancelled at any time.
#[derive(Clone)]
pub struct TaskHandle {
    fut: Shared<Task<Result<ExecutionResult, TaskTerminated>>>,
    cancel_token: AbortHandle,
}

impl TaskHandle {
    pub fn new(fut: BoxFuture<'static, ExecutionResult>, cx: AsyncAppContext) -> Result<Self> {
        let (cancel_token, abort_registration) = AbortHandle::new_pair();
        let fut = cx
            .spawn(move |_| Abortable::new(fut, abort_registration))
            .shared();
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

#[derive(Clone, Debug)]
/// Represents the result of a task.
pub struct ExecutionResult {
    /// Status of the task. Should be Ok(()) if the task succeeded, Err(()) otherwise. Note that
    /// the task might not even start up (e.g. due to a process spawning failure) and the status
    /// will still be Err().
    pub status: Result<(), Arc<dyn Error>>,
    /// Contains user-facing details for inspection. It could be e.g. stdout/stderr of a task.
    pub details: String,
}

/// Represents a short lived recipe of a runnable, whose main purpose
/// is to get spawned.
pub trait Runnable {
    fn name(&self) -> String;
    fn exec(&self, cx: gpui::AsyncAppContext) -> Result<TaskHandle>;
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}

pub trait Source: Any {
    fn as_any(&mut self) -> &mut dyn Any;
    fn runnables_for_path<'a>(
        &'a self,
        path: &Path,
        cx: &'a AppContext,
    ) -> anyhow::Result<Box<dyn Iterator<Item = RunnableToken> + 'a>>;
}

pub struct RunnableMetadata {
    source: WeakModel<Box<dyn Source>>,
    display_name: String,
}

impl RunnableMetadata {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

/// Represents a runnable that might or might not be already running.
#[derive(Clone)]
pub struct RunnableToken {
    metadata: Arc<RunnableMetadata>,
    state: Model<RunState>,
}

#[derive(Clone)]
pub(crate) enum RunState {
    NotScheduled(Arc<dyn Runnable>),
    Scheduled(TaskHandle),
}

impl RunnableToken {
    /// Schedules a task or returns a handle to it if it's already running.
    pub fn schedule(&self, cx: &mut AppContext) -> Result<TaskHandle> {
        let mut spawned_first_time = false;
        let ret = self.state.update(cx, |this, cx| match this {
            RunState::NotScheduled(runnable) => {
                dbg!("##############");
                let handle = runnable.exec(cx.to_async())?;
                spawned_first_time = true;
                *this = RunState::Scheduled(handle.clone());

                Ok(handle)
            }
            RunState::Scheduled(handle) => Ok(handle.clone()),
        });
        if dbg!(spawned_first_time) {
            // todo: this should be a noop when ran multiple times, but we should still strive to do it just once.
            cx.spawn(|_| async_process::driver()).detach();
            self.state.update(cx, |_, cx| {
                cx.spawn(|state, mut cx| async move {
                    let Some(this) = state.upgrade() else {
                        return;
                    };
                    let Some(handle) = this
                        .update(&mut cx, |this, _| {
                            if let RunState::Scheduled(this) = this {
                                Some(this.clone())
                            } else {
                                None
                            }
                        })
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let _ = handle.await.log_err();
                })
                .detach()
            })
        }
        ret
    }

    pub fn result<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<&'a Result<ExecutionResult, TaskTerminated>> {
        if let RunState::Scheduled(state) = self.state.read(cx) {
            state.fut.peek()
        } else {
            None
        }
    }
    pub fn cancel_handle(&self, cx: &AppContext) -> Option<AbortHandle> {
        if let RunState::Scheduled(state) = self.state.read(cx) {
            Some(state.termination_handle())
        } else {
            None
        }
    }
    pub fn metadata(&self) -> &RunnableMetadata {
        &self.metadata
    }
    pub fn id(&self) -> EntityId {
        self.state.entity_id()
    }
}
