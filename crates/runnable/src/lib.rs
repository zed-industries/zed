//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)]
pub mod static_runnable_file;
mod static_runner;
mod static_source;

use anyhow::Result;
use gpui::{impl_actions, AppContext, Model, ModelContext, WeakModel};
use serde::Deserialize;
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
        })
    }
}

/// Represents a short lived recipe of a runnable, whose main purpose
/// is to get spawned.
pub trait Runnable {
    fn name(&self) -> String;
    fn exec(&self, id: usize, cwd: Option<PathBuf>) -> Option<SpawnTaskInTerminal>;
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
        spawn_in_terminal: Option<SpawnTaskInTerminal>,
    },
}

impl Token {
    pub fn schedule(
        &self,
        cwd: Option<PathBuf>,
        cx: &mut AppContext,
    ) -> Option<SpawnTaskInTerminal> {
        self.state.update(cx, |run_state, _| match run_state {
            RunState::NotScheduled(runnable) => {
                let spawn_in_terminal = runnable.exec(self.id(), cwd);
                let spawn_in_terminal_to_return = spawn_in_terminal.clone();
                *run_state = RunState::Scheduled { spawn_in_terminal };
                spawn_in_terminal_to_return
            }
            RunState::Scheduled {
                spawn_in_terminal, ..
            } => spawn_in_terminal.clone(),
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
