//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
pub mod static_runnable_file;
mod static_runner;
mod static_source;

use anyhow::Result;
use futures::future::Shared;
use futures::FutureExt;
use gpui::{impl_actions, AppContext, Model, ModelContext, Task, WeakModel};
use serde::Deserialize;
use smol::channel::{Receiver, Sender};
pub use static_runner::StaticRunner;
pub use static_source::{StaticSource, TrackedFile};
use std::any::Any;
use std::path::{Path, PathBuf};
use std::sync::Arc;

impl_actions!(runnable, [SpawnTaskInTerminal]);

#[derive(Debug, Default, Clone)]
pub struct SpawnTaskInTerminal {
    pub task_id: usize,
    pub use_new_terminal: bool,
    pub label: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub cancellation_rx: Option<Receiver<()>>,
    pub completion_tx: Option<Sender<bool>>,
}

impl PartialEq for SpawnTaskInTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.task_id.eq(&other.task_id)
            && self.use_new_terminal.eq(&other.use_new_terminal)
            && self.label.eq(&other.label)
            && self.command.eq(&other.command)
            && self.args.eq(&other.args)
            && self.cwd.eq(&other.cwd)
    }
}

impl<'de> Deserialize<'de> for SpawnTaskInTerminal {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            task_id: 0,
            use_new_terminal: false,
            label: String::new(),
            command: String::new(),
            args: Vec::new(),
            cwd: None,
            cancellation_rx: None,
            completion_tx: None,
        })
    }
}

/// Represents a short lived recipe of a runnable, whose main purpose
/// is to get spawned.
pub trait Runnable {
    fn name(&self) -> String;
    fn exec(&self, id: usize, cwd: Option<PathBuf>) -> (Handle, Option<SpawnTaskInTerminal>);
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}

/// Represents a runnable that's already underway. That runnable can be cancelled at any time.
#[derive(Clone)]
pub struct Handle {
    pub completion_rx: Receiver<bool>,
    pub cancelation_tx: Sender<()>,
}

impl Handle {
    pub fn has_succeeded(&self) -> Option<bool> {
        self.completion_rx.try_recv().ok()
    }

    pub fn cancel(&self) {
        self.cancelation_tx.try_send(()).ok();
    }
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
    ) -> Vec<Token>;
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
    id: usize,
    metadata: Arc<Metadata>,
    state: Model<RunState>,
}

#[derive(Clone)]
pub(crate) enum RunState {
    NotScheduled(Arc<dyn Runnable>),
    Scheduled {
        handle: Handle,
        spawn_in_terminal: Option<SpawnTaskInTerminal>,
        _completion_task: Shared<Task<()>>,
    },
}

impl Token {
    /// Schedules a runnable or returns a handle to it if it's already running.
    pub fn schedule(
        &self,
        cwd: Option<PathBuf>,
        cx: &mut AppContext,
    ) -> (Handle, Option<SpawnTaskInTerminal>) {
        self.state.update(cx, |run_state, cx| match run_state {
            RunState::NotScheduled(runnable) => {
                let (handle, spawn_in_terminal) = runnable.exec(self.id(), cwd);
                let runnable = Arc::clone(runnable);
                let spawn_in_terminal_to_return = spawn_in_terminal.clone();
                let task_handle = handle.clone();
                let completion_task = cx
                    .spawn(move |state, mut cx| async move {
                        let _ = task_handle.completion_rx.recv().await.ok();
                        state
                            .update(&mut cx, |state, _| {
                                *state = RunState::NotScheduled(runnable);
                            })
                            .ok();
                    })
                    .shared();
                *run_state = RunState::Scheduled {
                    handle: handle.clone(),
                    spawn_in_terminal,
                    _completion_task: completion_task,
                };
                (handle, spawn_in_terminal_to_return)
            }
            RunState::Scheduled {
                handle,
                spawn_in_terminal,
                ..
            } => (handle.clone(), spawn_in_terminal.clone()),
        })
    }

    pub fn handle(&self, cx: &AppContext) -> Option<Handle> {
        let state = self.state.read(cx);
        if let RunState::Scheduled { handle, .. } = state {
            Some(handle.clone())
        } else {
            None
        }
    }

    pub fn was_scheduled(&self, cx: &AppContext) -> bool {
        matches!(self.state.read(cx), RunState::Scheduled { .. })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
