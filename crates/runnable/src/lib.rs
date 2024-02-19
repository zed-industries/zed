//! Baseline interface of Runnables in Zed: all runnables in Zed are intended to use those for implementing their own logic.
#![deny(missing_docs)]

mod static_runnable;
pub mod static_source;

pub use static_runnable::StaticRunnable;

use collections::HashMap;
use gpui::ModelContext;
use std::any::Any;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Runnable identifier, unique within the application.
/// Based on it, runnable reruns and terminal tabs are managed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunnableId(String);

/// Contains all information needed by Zed to spawn a new terminal tab for the given runnable.
#[derive(Debug, Clone)]
pub struct SpawnInTerminal {
    /// Id of the runnable to use when determining task tab affinity.
    pub id: RunnableId,
    /// Human readable name of the terminal tab.
    pub label: String,
    /// Executable command to spawn.
    pub command: String,
    /// Arguments to the command.
    pub args: Vec<String>,
    /// Current working directory to spawn the command into.
    pub cwd: Option<PathBuf>,
    /// Env overrides for the command, will be appended to the terminal's environment from the settings.
    pub env: HashMap<String, String>,
    /// Whether to use a new terminal tab or reuse the existing one to spawn the process.
    pub use_new_terminal: bool,
    /// Whether to allow multiple instances of the same runnable to be run, or rather wait for the existing ones to finish.
    pub allow_concurrent_runs: bool,
}

/// Represents a short lived recipe of a runnable, whose main purpose
/// is to get spawned.
pub trait Runnable {
    /// Unique identifier of the runnable to spawn.
    fn id(&self) -> &RunnableId;
    /// Human readable name of the runnable to display in the UI.
    fn name(&self) -> &str;
    /// Task's current working directory. If `None`, current project's root will be used.
    fn cwd(&self) -> Option<&Path>;
    /// Sets up everything needed to spawn the runnable in the given directory (`cwd`).
    /// If a runnable is intended to be spawned in the terminal, it should return the corresponding struct filled with the data necessary.
    fn exec(&self, cwd: Option<PathBuf>) -> Option<SpawnInTerminal>;
}

/// [`Source`] produces runnables that can be scheduled.
///
/// Implementations of this trait could be e.g. [`StaticSource`] that parses runnables from a .json files and provides process templates to be spawned;
/// another one could be a language server providing lenses with tests or build server listing all targets for a given project.
pub trait Source: Any {
    /// A way to erase the type of the source, processing and storing them generically.
    fn as_any(&mut self) -> &mut dyn Any;
    /// Collects all runnables available for scheduling, for the path given.
    fn runnables_for_path(
        &mut self,
        path: Option<&Path>,
        cx: &mut ModelContext<Box<dyn Source>>,
    ) -> Vec<Arc<dyn Runnable>>;
}
