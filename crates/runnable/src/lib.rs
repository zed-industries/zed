//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
pub mod static_runnable_file;
mod static_runner;
mod static_source;

use anyhow::Result;
use gpui::{impl_actions, AppContext, EntityId, Model, ModelContext, WeakModel};
use serde::Deserialize;
use smol::channel::{Receiver, Sender};
use static_runnable_file::Definition;
pub use static_runner::StaticRunner;
pub use static_source::{StaticSource, TrackedFile};
use std::any::Any;
use std::path::{Path, PathBuf};
use std::sync::Arc;

impl_actions!(runnable, [SpawnTaskInTerminal]);

#[derive(Debug, Default, Clone)]
pub struct SpawnTaskInTerminal {
    pub label: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub cancellation_rx: Option<Receiver<()>>,
    pub completion_tx: Option<Sender<bool>>,
}

impl PartialEq for SpawnTaskInTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
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
    fn exec(&self, cwd: Option<PathBuf>) -> Handle;
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}

/// Represents a runnable that's already underway. That runnable can be cancelled at any time.
#[derive(Clone)]
pub struct Handle {
    completion_rx: Receiver<bool>,
    cancelation_tx: Sender<()>,
    spawn_action: Option<SpawnTaskInTerminal>,
}

impl Handle {
    pub fn new(definition: &Definition, cwd: Option<PathBuf>) -> Self {
        let (completion_tx, completion_rx) = smol::channel::bounded(2);
        let (cancelation_tx, cancellation_rx) = smol::channel::bounded(2);
        Self {
            completion_rx,
            cancelation_tx,
            spawn_action: Some(SpawnTaskInTerminal {
                label: definition.label.clone(),
                command: definition.command.clone(),
                args: definition.args.clone(),
                cwd,
                cancellation_rx: Some(cancellation_rx),
                completion_tx: Some(completion_tx),
            }),
        }
    }

    pub fn has_succeeded(&self) -> Option<bool> {
        self.completion_rx.try_recv().ok()
    }

    pub fn cancel(&self) {
        self.cancelation_tx.try_send(()).ok();
    }

    pub fn completion_rx(&self) -> &Receiver<bool> {
        &self.completion_rx
    }

    pub fn take_spawn_action(&mut self) -> Option<SpawnTaskInTerminal> {
        self.spawn_action.take()
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
    pub fn schedule(&self, cwd: Option<PathBuf>, cx: &mut AppContext) -> Handle {
        let mut spawned_first_time = false;
        let handle = self.state.update(cx, |run_state, _| match run_state {
            RunState::NotScheduled(runnable) => {
                let handle = runnable.exec(cwd);
                spawned_first_time = true;
                *run_state = RunState::Scheduled(handle.clone());
                handle
            }
            RunState::Scheduled(handle) => handle.clone(),
        });
        if spawned_first_time {
            self.state.update(cx, |_, cx| {
                cx.spawn(|state, mut cx| async move {
                    let Some(this) = state.upgrade() else {
                        return;
                    };
                    let Some(handle) = this
                        .update(&mut cx, |state, _| {
                            if let RunState::Scheduled(handle) = state {
                                Some(handle.clone())
                            } else {
                                None
                            }
                        })
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let _ = handle.completion_rx.recv().await.ok();
                })
                .detach()
            })
        }
        handle
    }

    pub fn handle(&self, cx: &AppContext) -> Option<Handle> {
        let state = self.state.read(cx);
        if let RunState::Scheduled(handle) = state {
            Some(handle.clone())
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
