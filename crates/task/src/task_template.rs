use std::path::PathBuf;

use anyhow::{bail, Context};
use collections::HashMap;
use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use util::{truncate_and_remove_front, ResultExt};

use crate::{ResolvedTask, SpawnInTerminal, TaskContext, TaskId, ZED_VARIABLE_NAME_PREFIX};

/// A template definition of a Zed task to run.
/// May use the [`VariableName`] to get the corresponding substitutions into its fields.
///
/// Template itself is not ready to spawn a task, it needs to be resolved with a [`TaskContext`] first, that
/// contains all relevant Zed state in task variables.
/// A single template may produce different tasks (or none) for different contexts.
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
    /// Replaces all `VariableName` task variables in the task template string fields.
    /// If any replacement fails or the new string substitutions still have [`ZED_VARIABLE_NAME_PREFIX`],
    /// `None` is returned.
    ///
    /// Every [`ResolvedTask`] gets a [`TaskId`], based on the `id_base` (to avoid collision with various task sources),
    /// and hashes of its template and [`TaskContext`], see [`ResolvedTask`] fields' documentation for more details.
    pub fn resolve_task(&self, id_base: &str, cx: TaskContext) -> Option<ResolvedTask> {
        if self.label.trim().is_empty() || self.command.trim().is_empty() {
            return None;
        }
        let TaskContext {
            cwd,
            task_variables,
        } = cx;
        let task_variables = task_variables.into_env_variables();
        let truncated_variables = truncate_variables(&task_variables);
        let cwd = match self.cwd.as_deref() {
            Some(cwd) => Some(substitute_all_template_variables_in_str(
                cwd,
                &task_variables,
            )?),
            None => None,
        }
        .map(PathBuf::from)
        .or(cwd);
        let shortened_label =
            substitute_all_template_variables_in_str(&self.label, &truncated_variables)?;
        let full_label = substitute_all_template_variables_in_str(&self.label, &task_variables)?;
        let command = substitute_all_template_variables_in_str(&self.command, &task_variables)?;
        let args = substitute_all_template_variables_in_vec(self.args.clone(), &task_variables)?;

        let task_hash = to_hex_hash(&self)
            .context("hashing task template")
            .log_err()?;
        let variables_hash = to_hex_hash(&task_variables)
            .context("hashing task variables")
            .log_err()?;
        let id = TaskId(format!("{id_base}_{task_hash}_{variables_hash}"));
        let mut env = substitute_all_template_variables_in_map(self.env.clone(), &task_variables)?;
        env.extend(task_variables);
        Some(ResolvedTask {
            id: id.clone(),
            original_task: self.clone(),
            resolved_label: full_label,
            resolved: Some(SpawnInTerminal {
                id,
                cwd,
                label: shortened_label,
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

const MAX_DISPLAY_VARIABLE_LENGTH: usize = 15;

fn truncate_variables(task_variables: &HashMap<String, String>) -> HashMap<String, String> {
    task_variables
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                truncate_and_remove_front(value, MAX_DISPLAY_VARIABLE_LENGTH),
            )
        })
        .collect()
}

fn to_hex_hash(object: impl Serialize) -> anyhow::Result<String> {
    let json = serde_json_lenient::to_string(&object).context("serializing the object")?;
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}

fn substitute_all_template_variables_in_str(
    template_str: &str,
    task_variables: &HashMap<String, String>,
) -> Option<String> {
    let substituted_string = shellexpand::env_with_context(&template_str, |var| {
        // Colons denote a default value in case the variable is not set. We want to preserve that default, as otherwise shellexpand will substitute it for us.
        let colon_position = var.find(':').unwrap_or(var.len());
        let (variable_name, default) = var.split_at(colon_position);
        let append_previous_default = |ret: &mut String| {
            if !default.is_empty() {
                ret.push_str(default);
            }
        };
        if let Some(mut name) = task_variables.get(variable_name).cloned() {
            // Got a task variable hit
            append_previous_default(&mut name);
            return Ok(Some(name));
        } else if variable_name.starts_with(ZED_VARIABLE_NAME_PREFIX) {
            bail!("Unknown variable name: {}", variable_name);
        }
        // This is an unknown variable.
        // We should not error out, as they may come from user environment (e.g. $PATH). That means that the variable substitution might not be perfect.
        // If there's a default, we need to return the string verbatim as otherwise shellexpand will apply that default for us.
        if !default.is_empty() {
            return Ok(Some(format!("${{{var}}}")));
        }
        // Else we can just return None and that variable will be left as is.
        Ok(None)
    })
    .ok()?;
    Some(substituted_string.into_owned())
}

fn substitute_all_template_variables_in_vec(
    mut template_strs: Vec<String>,
    task_variables: &HashMap<String, String>,
) -> Option<Vec<String>> {
    for variable in template_strs.iter_mut() {
        let new_value = substitute_all_template_variables_in_str(&variable, task_variables)?;
        *variable = new_value;
    }
    Some(template_strs)
}

fn substitute_all_template_variables_in_map(
    keys_and_values: HashMap<String, String>,
    task_variables: &HashMap<String, String>,
) -> Option<HashMap<String, String>> {
    let mut new_map: HashMap<String, String> = Default::default();
    for (key, value) in keys_and_values {
        let new_value = substitute_all_template_variables_in_str(&value, task_variables)?;
        let new_key = substitute_all_template_variables_in_str(&key, task_variables)?;
        new_map.insert(new_key, new_value);
    }
    Some(new_map)
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, path::Path};

    use crate::{TaskVariables, VariableName};

    use super::*;

    const TEST_ID_BASE: &str = "test_base";

    #[test]
    fn test_resolving_templates_with_blank_command_and_label() {
        let task_with_all_properties = TaskTemplate {
            label: "test_label".to_string(),
            command: "test_command".to_string(),
            args: vec!["test_arg".to_string()],
            env: HashMap::from_iter([("test_env_key".to_string(), "test_env_var".to_string())]),
            ..TaskTemplate::default()
        };

        for task_with_blank_property in &[
            TaskTemplate {
                label: "".to_string(),
                ..task_with_all_properties.clone()
            },
            TaskTemplate {
                command: "".to_string(),
                ..task_with_all_properties.clone()
            },
            TaskTemplate {
                label: "".to_string(),
                command: "".to_string(),
                ..task_with_all_properties.clone()
            },
        ] {
            assert_eq!(
                task_with_blank_property.resolve_task(TEST_ID_BASE, TaskContext::default()),
                None,
                "should not resolve task with blank label and/or command: {task_with_blank_property:?}"
            );
        }
    }

    #[test]
    fn test_template_cwd_resolution() {
        let task_without_cwd = TaskTemplate {
            cwd: None,
            label: "test task".to_string(),
            command: "echo 4".to_string(),
            ..TaskTemplate::default()
        };

        let resolved_task = |task_template: &TaskTemplate, task_cx| {
            let resolved_task = task_template
                .resolve_task(TEST_ID_BASE, task_cx)
                .unwrap_or_else(|| panic!("failed to resolve task {task_without_cwd:?}"));
            resolved_task
                .resolved
                .clone()
                .unwrap_or_else(|| {
                    panic!("failed to get resolve data for resolved task. Template: {task_without_cwd:?} Resolved: {resolved_task:?}")
                })
        };

        assert_eq!(
            resolved_task(
                &task_without_cwd,
                TaskContext {
                    cwd: None,
                    task_variables: TaskVariables::default(),
                }
            )
            .cwd,
            None,
            "When neither task nor task context have cwd, it should be None"
        );

        let context_cwd = Path::new("a").join("b").join("c");
        assert_eq!(
            resolved_task(
                &task_without_cwd,
                TaskContext {
                    cwd: Some(context_cwd.clone()),
                    task_variables: TaskVariables::default(),
                }
            )
            .cwd
            .as_deref(),
            Some(context_cwd.as_path()),
            "TaskContext's cwd should be taken on resolve if task's cwd is None"
        );

        let task_cwd = Path::new("d").join("e").join("f");
        let mut task_with_cwd = task_without_cwd.clone();
        task_with_cwd.cwd = Some(task_cwd.display().to_string());
        let task_with_cwd = task_with_cwd;

        assert_eq!(
            resolved_task(
                &task_with_cwd,
                TaskContext {
                    cwd: None,
                    task_variables: TaskVariables::default(),
                }
            )
            .cwd
            .as_deref(),
            Some(task_cwd.as_path()),
            "TaskTemplate's cwd should be taken on resolve if TaskContext's cwd is None"
        );

        assert_eq!(
            resolved_task(
                &task_with_cwd,
                TaskContext {
                    cwd: Some(context_cwd.clone()),
                    task_variables: TaskVariables::default(),
                }
            )
            .cwd
            .as_deref(),
            Some(task_cwd.as_path()),
            "TaskTemplate's cwd should be taken on resolve if TaskContext's cwd is not None"
        );
    }

    #[test]
    fn test_template_variables_resolution() {
        let custom_variable_1 = VariableName::Custom(Cow::Borrowed("custom_variable_1"));
        let custom_variable_2 = VariableName::Custom(Cow::Borrowed("custom_variable_2"));
        let long_value = "01".repeat(MAX_DISPLAY_VARIABLE_LENGTH * 2);
        let all_variables = [
            (VariableName::Row, "1234".to_string()),
            (VariableName::Column, "5678".to_string()),
            (VariableName::File, "test_file".to_string()),
            (VariableName::SelectedText, "test_selected_text".to_string()),
            (VariableName::Symbol, long_value.clone()),
            (VariableName::WorktreeRoot, "/test_root/".to_string()),
            (
                custom_variable_1.clone(),
                "test_custom_variable_1".to_string(),
            ),
            (
                custom_variable_2.clone(),
                "test_custom_variable_2".to_string(),
            ),
        ];

        let task_with_all_variables = TaskTemplate {
            label: format!(
                "test label for {} and {}",
                VariableName::Row.template_value(),
                VariableName::Symbol.template_value(),
            ),
            command: format!(
                "echo {} {}",
                VariableName::File.template_value(),
                VariableName::Symbol.template_value(),
            ),
            args: vec![
                format!("arg1 {}", VariableName::SelectedText.template_value()),
                format!("arg2 {}", VariableName::Column.template_value()),
                format!("arg3 {}", VariableName::Symbol.template_value()),
            ],
            env: HashMap::from_iter([
                ("test_env_key".to_string(), "test_env_var".to_string()),
                (
                    "env_key_1".to_string(),
                    VariableName::WorktreeRoot.template_value(),
                ),
                (
                    "env_key_2".to_string(),
                    format!(
                        "env_var_2_{}_{}",
                        custom_variable_1.template_value(),
                        custom_variable_2.template_value()
                    ),
                ),
                (
                    "env_key_3".to_string(),
                    format!("env_var_3_{}", VariableName::Symbol.template_value()),
                ),
            ]),
            ..TaskTemplate::default()
        };

        let mut first_resolved_id = None;
        for i in 0..15 {
            let resolved_task = task_with_all_variables.resolve_task(
                TEST_ID_BASE,
                TaskContext {
                    cwd: None,
                    task_variables: TaskVariables::from_iter(all_variables.clone()),
                },
            ).unwrap_or_else(|| panic!("Should successfully resolve task {task_with_all_variables:?} with variables {all_variables:?}"));

            match &first_resolved_id {
                None => first_resolved_id = Some(resolved_task.id),
                Some(first_id) => assert_eq!(
                    &resolved_task.id, first_id,
                    "Step {i}, for the same task template and context, there should be the same resolved task id"
                ),
            }

            assert_eq!(
                resolved_task.original_task, task_with_all_variables,
                "Resolved task should store its template without changes"
            );
            assert_eq!(
                resolved_task.resolved_label,
                format!("test label for 1234 and {long_value}"),
                "Resolved task label should be substituted with variables and those should not be shortened"
            );

            let spawn_in_terminal = resolved_task
                .resolved
                .as_ref()
                .expect("should have resolved a spawn in terminal task");
            assert_eq!(
                spawn_in_terminal.label,
                format!(
                    "test label for 1234 and …{}",
                    &long_value[..=MAX_DISPLAY_VARIABLE_LENGTH]
                ),
                "Human-readable label should have long substitutions trimmed"
            );
            assert_eq!(
                spawn_in_terminal.command,
                format!("echo test_file {long_value}"),
                "Command should be substituted with variables and those should not be shortened"
            );
            assert_eq!(
                spawn_in_terminal.args,
                &[
                    "arg1 test_selected_text",
                    "arg2 5678",
                    &format!("arg3 {long_value}")
                ],
                "Args should be substituted with variables and those should not be shortened"
            );

            assert_eq!(
                spawn_in_terminal
                    .env
                    .get("test_env_key")
                    .map(|s| s.as_str()),
                Some("test_env_var")
            );
            assert_eq!(
                spawn_in_terminal.env.get("env_key_1").map(|s| s.as_str()),
                Some("/test_root/")
            );
            assert_eq!(
                spawn_in_terminal.env.get("env_key_2").map(|s| s.as_str()),
                Some("env_var_2_test_custom_variable_1_test_custom_variable_2")
            );
            assert_eq!(
                spawn_in_terminal.env.get("env_key_3"),
                Some(&format!("env_var_3_{long_value}")),
                "Env vars should be substituted with variables and those should not be shortened"
            );
        }

        for i in 0..all_variables.len() {
            let mut not_all_variables = all_variables.to_vec();
            let removed_variable = not_all_variables.remove(i);
            let resolved_task_attempt = task_with_all_variables.resolve_task(
                TEST_ID_BASE,
                TaskContext {
                    cwd: None,
                    task_variables: TaskVariables::from_iter(not_all_variables),
                },
            );
            assert_eq!(resolved_task_attempt, None, "If any of the Zed task variables is not substituted, the task should not be resolved, but got some resolution without the variable {removed_variable:?} (index {i})");
        }
    }

    #[test]
    fn test_can_resolve_free_variables() {
        let task = TaskTemplate {
            label: "My task".into(),
            command: "echo".into(),
            args: vec!["$PATH".into()],
            ..Default::default()
        };
        let resolved = task
            .resolve_task(TEST_ID_BASE, TaskContext::default())
            .unwrap()
            .resolved
            .unwrap();
        assert_eq!(resolved.label, task.label);
        assert_eq!(resolved.command, task.command);
        assert_eq!(resolved.args, task.args);
    }

    #[test]
    fn test_errors_on_missing_zed_variable() {
        let task = TaskTemplate {
            label: "My task".into(),
            command: "echo".into(),
            args: vec!["$ZED_VARIABLE".into()],
            ..Default::default()
        };
        assert!(task
            .resolve_task(TEST_ID_BASE, TaskContext::default())
            .is_none());
    }
}
