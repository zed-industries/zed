//! Baseline interface of Tasks in Zed: all tasks in Zed are intended to use those for implementing their own logic.
#![deny(missing_docs)]

pub mod oneshot_source;
pub mod static_source;
mod vscode_format;

use collections::HashMap;
use gpui::ModelContext;
use static_source::RevealStrategy;
use std::any::Any;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;
pub use vscode_format::VsCodeTaskFile;

/// Task identifier, unique within the application.
/// Based on it, task reruns and terminal tabs are managed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskId(pub String);

/// Contains all information needed by Zed to spawn a new terminal tab for the given task.
#[derive(Debug, Clone)]
pub struct SpawnInTerminal {
    /// Id of the task to use when determining task tab affinity.
    pub id: TaskId,
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
    /// Whether to allow multiple instances of the same task to be run, or rather wait for the existing ones to finish.
    pub allow_concurrent_runs: bool,
    /// What to do with the terminal pane and tab, after the command was started.
    pub reveal: RevealStrategy,
}

/// Variables, available for use in [`TaskContext`] when a Zed's task gets turned into real command.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VariableName {
    /// An absolute path of the currently opened file.
    File,
    /// An absolute path of the currently opened worktree, that contains the file.
    WorktreeRoot,
    /// A symbol text, that contains latest cursor/selection position.
    Symbol,
    /// A row with the latest cursor/selection position.
    Row,
    /// A column with the latest cursor/selection position.
    Column,
    /// Text from the latest selection.
    SelectedText,
    /// Custom variable, provided by the plugin or other external source.
    /// Will be printed with `ZED_` prefix to avoid potential conflicts with other variables.
    Custom(Cow<'static, str>),
}

impl VariableName {
    /// Generates a `$VARIABLE`-like string value to be used in templates.
    /// Custom variables are wrapped in `${}` to avoid substitution issues with whitespaces.
    pub fn template_value(&self) -> String {
        if matches!(self, Self::Custom(_)) {
            format!("${{{self}}}")
        } else {
            format!("${self}")
        }
    }
}

impl std::fmt::Display for VariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "ZED_FILE"),
            Self::WorktreeRoot => write!(f, "ZED_WORKTREE_ROOT"),
            Self::Symbol => write!(f, "ZED_SYMBOL"),
            Self::Row => write!(f, "ZED_ROW"),
            Self::Column => write!(f, "ZED_COLUMN"),
            Self::SelectedText => write!(f, "ZED_SELECTED_TEXT"),
            Self::Custom(s) => write!(f, "ZED_{s}"),
        }
    }
}

/// Container for predefined environment variables that describe state of Zed at the time the task was spawned.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TaskVariables(HashMap<VariableName, String>);

impl TaskVariables {
    /// Converts the container into a map of environment variables and their values.
    fn into_env_variables(self) -> HashMap<String, String> {
        self.0
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect()
    }

    /// Inserts another variable into the container, overwriting the existing one if it already exists — in this case, the old value is returned.
    pub fn insert(&mut self, variable: VariableName, value: String) -> Option<String> {
        self.0.insert(variable, value)
    }

    /// Extends the container with another one, overwriting the existing variables on collision.
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl FromIterator<(VariableName, String)> for TaskVariables {
    fn from_iter<T: IntoIterator<Item = (VariableName, String)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

/// Keeps track of the file associated with a task and context of tasks execution (i.e. current file or current function)
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TaskContext {
    /// A path to a directory in which the task should be executed.
    pub cwd: Option<PathBuf>,
    /// Additional environment variables associated with a given task.
    pub task_variables: TaskVariables,
}

/// Represents a short lived recipe of a task, whose main purpose
/// is to get spawned.
pub trait Task {
    /// Unique identifier of the task to spawn.
    fn id(&self) -> &TaskId;
    /// Human readable name of the task to display in the UI.
    fn name(&self) -> &str;
    /// Task's current working directory. If `None`, current project's root will be used.
    fn cwd(&self) -> Option<&str>;
    /// Sets up everything needed to spawn the task in the given directory (`cwd`).
    /// If a task is intended to be spawned in the terminal, it should return the corresponding struct filled with the data necessary.
    fn prepare_exec(&self, cx: TaskContext) -> Option<SpawnInTerminal>;
}

/// [`Source`] produces tasks that can be scheduled.
///
/// Implementations of this trait could be e.g. [`StaticSource`] that parses tasks from a .json files and provides process templates to be spawned;
/// another one could be a language server providing lenses with tests or build server listing all targets for a given project.
pub trait TaskSource: Any {
    /// A way to erase the type of the source, processing and storing them generically.
    fn as_any(&mut self) -> &mut dyn Any;
    /// Collects all tasks available for scheduling.
    fn tasks_to_schedule(
        &mut self,
        cx: &mut ModelContext<Box<dyn TaskSource>>,
    ) -> Vec<Arc<dyn Task>>;
}
