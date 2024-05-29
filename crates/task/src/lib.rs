//! Baseline interface of Tasks in Zed: all tasks in Zed are intended to use those for implementing their own logic.
#![deny(missing_docs)]

pub mod static_source;
mod task_template;
mod vscode_format;

use collections::{hash_map, HashMap, HashSet};
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use std::{borrow::Cow, path::Path};

pub use task_template::{RevealStrategy, TaskTemplate, TaskTemplates};
pub use vscode_format::VsCodeTaskFile;

/// Task identifier, unique within the application.
/// Based on it, task reruns and terminal tabs are managed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
pub struct TaskId(pub String);

/// TerminalWorkDir describes where a task should be run
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalWorkDir {
    /// Local is on this machine
    Local(PathBuf),
    /// SSH runs the terminal over ssh
    Ssh {
        /// The command to run to connect
        ssh_command: String,
        /// The path on the remote server
        path: Option<String>,
    },
}

impl TerminalWorkDir {
    /// Returns whether the terminal task is supposed to be spawned on a local machine or not.
    pub fn is_local(&self) -> bool {
        match self {
            Self::Local(_) => true,
            Self::Ssh { .. } => false,
        }
    }

    /// Returns a local CWD if the terminal is local, None otherwise.
    pub fn local_path(&self) -> Option<&Path> {
        match self {
            Self::Local(path) => Some(path),
            Self::Ssh { .. } => None,
        }
    }
}

/// Contains all information needed by Zed to spawn a new terminal tab for the given task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnInTerminal {
    /// Id of the task to use when determining task tab affinity.
    pub id: TaskId,
    /// Full unshortened form of `label` field.
    pub full_label: String,
    /// Human readable name of the terminal tab.
    pub label: String,
    /// Executable command to spawn.
    pub command: String,
    /// Arguments to the command, potentially unsubstituted,
    /// to let the shell that spawns the command to do the substitution, if needed.
    pub args: Vec<String>,
    /// A human-readable label, containing command and all of its arguments, joined and substituted.
    pub command_label: String,
    /// Current working directory to spawn the command into.
    pub cwd: Option<TerminalWorkDir>,
    /// Env overrides for the command, will be appended to the terminal's environment from the settings.
    pub env: HashMap<String, String>,
    /// Whether to use a new terminal tab or reuse the existing one to spawn the process.
    pub use_new_terminal: bool,
    /// Whether to allow multiple instances of the same task to be run, or rather wait for the existing ones to finish.
    pub allow_concurrent_runs: bool,
    /// What to do with the terminal pane and tab, after the command was started.
    pub reveal: RevealStrategy,
}

/// A final form of the [`TaskTemplate`], that got resolved with a particualar [`TaskContext`] and now is ready to spawn the actual task.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedTask {
    /// A way to distinguish tasks produced by the same template, but different contexts.
    /// NOTE: Resolved tasks may have the same labels, commands and do the same things,
    /// but still may have different ids if the context was different during the resolution.
    /// Since the template has `env` field, for a generic task that may be a bash command,
    /// so it's impossible to determine the id equality without more context in a generic case.
    pub id: TaskId,
    /// A template the task got resolved from.
    original_task: TaskTemplate,
    /// Full, unshortened label of the task after all resolutions are made.
    pub resolved_label: String,
    /// Variables that were substituted during the task template resolution.
    substituted_variables: HashSet<VariableName>,
    /// Further actions that need to take place after the resolved task is spawned,
    /// with all task variables resolved.
    pub resolved: Option<SpawnInTerminal>,
}

impl ResolvedTask {
    /// A task template before the resolution.
    pub fn original_task(&self) -> &TaskTemplate {
        &self.original_task
    }

    /// Variables that were substituted during the task template resolution.
    pub fn substituted_variables(&self) -> &HashSet<VariableName> {
        &self.substituted_variables
    }

    /// A human-readable label to display in the UI.
    pub fn display_label(&self) -> &str {
        self.resolved
            .as_ref()
            .map(|resolved| resolved.label.as_str())
            .unwrap_or_else(|| self.resolved_label.as_str())
    }
}

/// Variables, available for use in [`TaskContext`] when a Zed's [`TaskTemplate`] gets resolved into a [`ResolvedTask`].
/// Name of the variable must be a valid shell variable identifier, which generally means that it is
/// a word  consisting only  of alphanumeric characters and underscores,
/// and beginning with an alphabetic character or an  underscore.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum VariableName {
    /// An absolute path of the currently opened file.
    File,
    /// A path of the currently opened file (relative to worktree root).
    RelativeFile,
    /// The currently opened filename.
    Filename,
    /// The path to a parent directory of a currently opened file.
    Dirname,
    /// Stem (filename without extension) of the currently opened file.
    Stem,
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
    /// The symbol selected by the symbol tagging system, specifically the @run capture in a runnables.scm
    RunnableSymbol,
    /// Custom variable, provided by the plugin or other external source.
    /// Will be printed with `CUSTOM_` prefix to avoid potential conflicts with other variables.
    Custom(Cow<'static, str>),
}

impl VariableName {
    /// Generates a `$VARIABLE`-like string value to be used in templates.
    pub fn template_value(&self) -> String {
        format!("${self}")
    }
}

impl FromStr for VariableName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let without_prefix = s.strip_prefix(ZED_VARIABLE_NAME_PREFIX).ok_or(())?;
        let value = match without_prefix {
            "FILE" => Self::File,
            "WORKTREE_ROOT" => Self::WorktreeRoot,
            "SYMBOL" => Self::Symbol,
            "SELECTED_TEXT" => Self::SelectedText,
            "ROW" => Self::Row,
            "COLUMN" => Self::Column,
            _ => {
                if let Some(custom_name) =
                    without_prefix.strip_prefix(ZED_CUSTOM_VARIABLE_NAME_PREFIX)
                {
                    Self::Custom(Cow::Owned(custom_name.to_owned()))
                } else {
                    return Err(());
                }
            }
        };
        Ok(value)
    }
}

/// A prefix that all [`VariableName`] variants are prefixed with when used in environment variables and similar template contexts.
pub const ZED_VARIABLE_NAME_PREFIX: &str = "ZED_";
const ZED_CUSTOM_VARIABLE_NAME_PREFIX: &str = "CUSTOM_";

impl std::fmt::Display for VariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "{ZED_VARIABLE_NAME_PREFIX}FILE"),
            Self::Filename => write!(f, "{ZED_VARIABLE_NAME_PREFIX}FILENAME"),
            Self::RelativeFile => write!(f, "{ZED_VARIABLE_NAME_PREFIX}RELATIVE_FILE"),
            Self::Dirname => write!(f, "{ZED_VARIABLE_NAME_PREFIX}DIRNAME"),
            Self::Stem => write!(f, "{ZED_VARIABLE_NAME_PREFIX}STEM"),
            Self::WorktreeRoot => write!(f, "{ZED_VARIABLE_NAME_PREFIX}WORKTREE_ROOT"),
            Self::Symbol => write!(f, "{ZED_VARIABLE_NAME_PREFIX}SYMBOL"),
            Self::Row => write!(f, "{ZED_VARIABLE_NAME_PREFIX}ROW"),
            Self::Column => write!(f, "{ZED_VARIABLE_NAME_PREFIX}COLUMN"),
            Self::SelectedText => write!(f, "{ZED_VARIABLE_NAME_PREFIX}SELECTED_TEXT"),
            Self::RunnableSymbol => write!(f, "{ZED_VARIABLE_NAME_PREFIX}RUNNABLE_SYMBOL"),
            Self::Custom(s) => write!(
                f,
                "{ZED_VARIABLE_NAME_PREFIX}{ZED_CUSTOM_VARIABLE_NAME_PREFIX}{s}"
            ),
        }
    }
}

/// Container for predefined environment variables that describe state of Zed at the time the task was spawned.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct TaskVariables(HashMap<VariableName, String>);

impl TaskVariables {
    /// Inserts another variable into the container, overwriting the existing one if it already exists — in this case, the old value is returned.
    pub fn insert(&mut self, variable: VariableName, value: String) -> Option<String> {
        self.0.insert(variable, value)
    }

    /// Extends the container with another one, overwriting the existing variables on collision.
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
    /// Get the value associated with given variable name, if there is one.
    pub fn get(&self, key: &VariableName) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    /// Clear out variables obtained from tree-sitter queries, which are prefixed with '_' character
    pub fn sweep(&mut self) {
        self.0.retain(|name, _| {
            if let VariableName::Custom(name) = name {
                !name.starts_with('_')
            } else {
                true
            }
        })
    }
}

impl FromIterator<(VariableName, String)> for TaskVariables {
    fn from_iter<T: IntoIterator<Item = (VariableName, String)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl IntoIterator for TaskVariables {
    type Item = (VariableName, String);

    type IntoIter = hash_map::IntoIter<VariableName, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Keeps track of the file associated with a task and context of tasks execution (i.e. current file or current function).
/// Keeps all Zed-related state inside, used to produce a resolved task out of its template.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct TaskContext {
    /// A path to a directory in which the task should be executed.
    pub cwd: Option<PathBuf>,
    /// Additional environment variables associated with a given task.
    pub task_variables: TaskVariables,
}

/// This is a new type representing a 'tag' on a 'runnable symbol', typically a test of main() function, found via treesitter.
#[derive(Clone, Debug)]
pub struct RunnableTag(pub SharedString);
