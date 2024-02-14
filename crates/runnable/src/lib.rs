//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
mod handle;
pub mod static_runnable_file;
mod static_runner;
mod static_source;

use anyhow::Result;
use async_process::ExitStatus;
use futures::stream::AbortHandle;
pub use futures::stream::Aborted as RunnableTerminated;
use gpui::{AppContext, EntityId, Model, ModelContext, WeakModel};
pub use handle::{Handle, NewLineAvailable, PendingOutput};
pub use static_runner::StaticRunner;
pub use static_source::{StaticSource, TrackedFile};
use std::any::Any;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use util::ResultExt;

#[derive(Clone, Debug)]
/// Represents the result of a runnable.
pub struct ExecutionResult {
    /// Status of the runnable. Should be `Ok` if the runnable launch succeeded, `Err` otherwise.
    pub status: Result<ExitStatus, Arc<anyhow::Error>>,
    pub output: Option<Model<PendingOutput>>,
}

/// Represents a short lived recipe of a runnable, whose main purpose
/// is to get spawned.
pub trait Runnable {
    fn name(&self) -> String;
    fn exec(&self, cwd: Option<PathBuf>, cx: &mut AppContext) -> Result<Handle>;
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}

/// [`Source`] produces runnables that can be scheduled.
///
/// Implementations of this trait could be e.g. [`StaticSource`] that parses tasks from a .json files and provides process templates to be spawned;
/// another one could be a language server providing lenses with tests or build server listing all targets for a given project.
pub trait Source: Any {
    fn as_any(&mut self) -> &mut dyn Any;
    fn runnables_for_path(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Box<dyn Source>>,
    ) -> anyhow::Result<Vec<Token>>;
}

#[derive(PartialEq)]
pub struct Metadata {
    source: WeakModel<Box<dyn Source>>,
    display_name: String,
}

impl Metadata {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

/// Represents a runnable that might or might not be already running.
#[derive(Clone)]
pub struct Token {
    metadata: Arc<Metadata>,
    state: Model<RunState>,
}

#[derive(Clone)]
pub(crate) enum RunState {
    NotScheduled(Arc<dyn Runnable>),
    Scheduled(Handle),
}

impl Token {
    /// Schedules a runnable or returns a handle to it if it's already running.
    pub fn schedule(&self, cwd: Option<PathBuf>, cx: &mut AppContext) -> Result<Handle> {
        let mut spawned_first_time = false;
        let ret = self.state.update(cx, |this, cx| match this {
            RunState::NotScheduled(runnable) => {
                let handle = runnable.exec(cwd, cx)?;
                spawned_first_time = true;
                *this = RunState::Scheduled(handle.clone());

                Ok(handle)
            }
            RunState::Scheduled(handle) => Ok(handle.clone()),
        });
        if spawned_first_time {
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
                    let _ = handle.fut.await.log_err();
                })
                .detach()
            })
        }
        ret
    }

    pub fn handle(&self, cx: &AppContext) -> Option<Handle> {
        let state = self.state.read(cx);
        if let RunState::Scheduled(state) = state {
            Some(state.clone())
        } else {
            None
        }
    }

    pub fn result<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<Result<ExecutionResult, RunnableTerminated>> {
        if let RunState::Scheduled(state) = self.state.read(cx) {
            state.fut.peek().cloned().map(|res| {
                res.map(|runnable_result| ExecutionResult {
                    status: runnable_result,
                    output: state.output.clone(),
                })
            })
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

    pub fn was_scheduled(&self, cx: &AppContext) -> bool {
        self.handle(cx).is_some()
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn id(&self) -> EntityId {
        self.state.entity_id()
    }
}
