use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A template definition of a Zed task to run.
/// May use the [`VariableName`] to get the corresponding substitutions into its fields.
///
/// Template itself is not ready to spawn a task, it needs to be resolved with a [`TaskContext`] first, that
/// contains all relevant Zed state in task variables.
/// A single template may produce different tasks (or none) for different contexts.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DebuggerConfigTemplate {
    pub _type: String,
    pub request: String,
    #[serde(default)]
    pub args: Vec<String>,
}
