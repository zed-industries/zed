use std::path::PathBuf;

use collections::HashMap;
use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::{ResolvedTask, SpawnInTerminal, TaskContext, TaskId, ZED_VARIABLE_NAME_PREFIX};

/// Static task template from the tasks config file.
/// May use the [`VariableName`] to get the corresponding substitutions into its fields.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TaskTemplate {
    /// Human readable name of the task to display in the UI.
    pub label: String,
    /// Executable command to spawn.
    pub command: String,
    /// Arguments to the command.
    #[serde(default)]
    pub args: Vec<String>,
    /// Env overrides for the command, will be appended to the terminal's environment from the settings.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Current working directory to spawn the command into, defaults to current project root.
    #[serde(default)]
    pub cwd: Option<String>,
    /// Whether to use a new terminal tab or reuse the existing one to spawn the process.
    #[serde(default)]
    pub use_new_terminal: bool,
    /// Whether to allow multiple instances of the same task to be run, or rather wait for the existing ones to finish.
    #[serde(default)]
    pub allow_concurrent_runs: bool,
    /// What to do with the terminal pane and tab, after the command was started:
    /// * `always` — always show the terminal pane, add and focus the corresponding task's tab in it (default)
    /// * `never` — avoid changing current terminal pane focus, but still add/reuse the task's tab there
    #[serde(default)]
    pub reveal: RevealStrategy,
}

/// What to do with the terminal pane and tab, after the command was started.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RevealStrategy {
    /// Always show the terminal pane, add and focus the corresponding task's tab in it.
    #[default]
    Always,
    /// Do not change terminal pane focus, but still add/reuse the task's tab there.
    Never,
}

/// A group of Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TaskTemplates(pub Vec<TaskTemplate>);

impl TaskTemplates {
    /// Generates JSON schema of Tasks JSON template format.
    pub fn generate_json_schema() -> serde_json_lenient::Value {
        let schema = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator()
            .into_root_schema_for::<Self>();

        serde_json_lenient::to_value(schema).unwrap()
    }
}

impl TaskTemplate {
    /// TODO kb tests + docs
    pub fn resolve_task(&self, id_base: String, cx: TaskContext) -> Option<ResolvedTask> {
        let TaskContext {
            cwd,
            task_variables,
        } = cx;
        let task_variables = task_variables.into_env_variables();
        let cwd = match self.cwd.as_deref() {
            Some(cwd) => Some(substitute_all_template_variables_in_str(
                cwd,
                &task_variables,
            )?),
            None => None,
        }
        .map(PathBuf::from)
        .or(cwd);
        let label = substitute_all_template_variables_in_str(&self.label, &task_variables)?;
        let command = substitute_all_template_variables_in_str(&self.command, &task_variables)?;
        let args = substitute_all_template_variables_in_vec(self.args.clone(), &task_variables)?;
        let mut env = substitute_all_template_variables_in_map(self.env.clone(), &task_variables)?;
        env.extend(task_variables);
        let id = TaskId(format!(
            "{id_base}_TODO kb calculate hash of the TaskContext"
        ));
        Some(ResolvedTask {
            id: id.clone(),
            original_task: self.clone(),
            resolved_label: label.clone(),
            resolved: Some(SpawnInTerminal {
                id,
                cwd,
                label,
                command,
                args,
                env,
                use_new_terminal: self.use_new_terminal,
                allow_concurrent_runs: self.allow_concurrent_runs,
                reveal: self.reveal,
            }),
        })
    }
}

fn substitute_all_template_variables_in_str(
    template_str: &str,
    task_variables: &HashMap<String, String>,
) -> Option<String> {
    let substituted_string = subst::substitute(&template_str, task_variables).ok()?;
    if substituted_string.contains(ZED_VARIABLE_NAME_PREFIX) {
        return None;
    }
    Some(substituted_string)
}

fn substitute_all_template_variables_in_vec(
    mut keys: Vec<String>,
    task_variables: &HashMap<String, String>,
) -> Option<Vec<String>> {
    for key in &mut keys {
        match task_variables.get(key) {
            Some(variable_expansion) => *key = variable_expansion.clone(),
            None => {
                if key.starts_with(ZED_VARIABLE_NAME_PREFIX) {
                    return None;
                }
            }
        }
    }
    Some(keys)
}

fn substitute_all_template_variables_in_map(
    keys_and_values: HashMap<String, String>,
    task_variables: &HashMap<String, String>,
) -> Option<HashMap<String, String>> {
    keys_and_values
        .into_iter()
        .try_fold(HashMap::default(), |mut expanded_keys, (mut key, value)| {
            match task_variables.get(&key) {
                Some(variable_expansion) => key = variable_expansion.clone(),
                None => {
                    if key.starts_with(ZED_VARIABLE_NAME_PREFIX) {
                        return Err(());
                    }
                }
            }
            expanded_keys.insert(
                key,
                subst::substitute(&value, task_variables)
                    .map_err(|_| ())?
                    .to_string(),
            );
            Ok(expanded_keys)
        })
        .ok()
}
