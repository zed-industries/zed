//! Defines baseline interface of Runnables in Zed.
// #![deny(missing_docs)] // TODO kb rustdocs everywhere
mod static_runnable;
pub mod static_runnable_file;
mod static_source;

pub use static_runnable::StaticRunnable;
pub use static_source::{StaticSource, TrackedFile};

use anyhow::Result;
use gpui::{impl_actions, ModelContext};
use serde::Deserialize;
use std::any::Any;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// TODO kb fugly: can we actually use an event instead?
impl_actions!(runnable, [SpawnInTerminal]);

#[derive(Debug, Clone)]
pub struct SpawnInTerminal {
    pub id: RunnableId,
    pub use_new_terminal: bool,
    pub label: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
}

impl PartialEq for SpawnInTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
            && self.use_new_terminal.eq(&other.use_new_terminal)
            && self.label.eq(&other.label)
            && self.command.eq(&other.command)
            && self.args.eq(&other.args)
            && self.cwd.eq(&other.cwd)
    }
}

impl<'de> Deserialize<'de> for SpawnInTerminal {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            id: RunnableId(String::new()),
            use_new_terminal: false,
            label: String::new(),
            command: String::new(),
            args: Vec::new(),
            cwd: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnableId(String);

/// Represents a short lived recipe of a runnable, whose main purpose
/// is to get spawned.
pub trait Runnable {
    fn id(&self) -> &RunnableId;
    fn name(&self) -> &str;
    fn exec(&self, cwd: Option<PathBuf>) -> Option<SpawnInTerminal>;
    fn boxed_clone(&self) -> Box<dyn Runnable>;
}

/// [`Source`] produces runnables that can be scheduled.
///
/// Implementations of this trait could be e.g. [`StaticSource`] that parses runnables from a .json files and provides process templates to be spawned;
/// another one could be a language server providing lenses with tests or build server listing all targets for a given project.
pub trait Source: Any {
    fn as_any(&mut self) -> &mut dyn Any;
    fn runnables_for_path(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Box<dyn Source>>,
    ) -> Vec<Arc<dyn Runnable>>;
}
