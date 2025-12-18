mod configuration_template;
mod recipe;

use collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use task::{TaskContext, TaskVariables};

pub use configuration_template::{ConfigurationTemplate, ConfigurationTemplates, ConfigurationType};
pub use recipe::{Recipe, Recipes};

/// Configuration identifier, unique within the application.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ConfigurationId(pub String);

/// A resolved configuration ready to be executed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedConfiguration {
    /// Unique identifier for this resolved configuration
    pub id: ConfigurationId,
    /// The template this configuration was resolved from
    original_template: ConfigurationTemplate,
    /// The resolved label after variable substitution
    pub resolved_label: String,
    /// Variables that were substituted during resolution
    substituted_variables: HashSet<task::VariableName>,
    /// The resolved execution parameters
    pub resolved: ExecutionConfig,
}

impl ResolvedConfiguration {
    pub fn original_template(&self) -> &ConfigurationTemplate {
        &self.original_template
    }

    pub fn substituted_variables(&self) -> &HashSet<task::VariableName> {
        &self.substituted_variables
    }

    pub fn display_label(&self) -> &str {
        self.resolved.label.as_str()
    }
}

/// The final execution parameters for a configuration
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExecutionConfig {
    /// The configuration ID
    pub id: ConfigurationId,
    /// Full unshortened label
    pub full_label: String,
    /// Human-readable label (may be truncated)
    pub label: String,
    /// Command to execute (for non-attach configurations)
    pub command: Option<String>,
    /// Arguments to the command
    pub args: Vec<String>,
    /// Working directory
    pub cwd: Option<PathBuf>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Configuration type (run/debug)
    pub config_type: ConfigurationType,
}

/// Context for resolving configuration templates
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfigurationContext {
    /// Working directory for execution
    pub cwd: Option<PathBuf>,
    /// Task variables for substitution
    pub task_variables: TaskVariables,
    /// Project environment variables
    pub project_env: HashMap<String, String>,
}

impl From<TaskContext> for ConfigurationContext {
    fn from(task_ctx: TaskContext) -> Self {
        Self {
            cwd: task_ctx.cwd,
            task_variables: task_ctx.task_variables,
            project_env: task_ctx.project_env,
        }
    }
}

impl From<ConfigurationContext> for TaskContext {
    fn from(config_ctx: ConfigurationContext) -> Self {
        Self {
            cwd: config_ctx.cwd,
            task_variables: config_ctx.task_variables,
            project_env: config_ctx.project_env,
        }
    }
}
