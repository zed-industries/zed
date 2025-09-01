use anyhow::{Context as _, bail};
use collections::{HashMap, HashSet};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use util::schemars::DefaultDenyUnknownFields;
use util::serde::default_true;
use util::{ResultExt, truncate_and_remove_front};

use crate::{
    AttachRequest, ResolvedTask, RevealTarget, Shell, SpawnInTerminal, TaskContext, TaskId,
    VariableName, ZED_VARIABLE_NAME_PREFIX, serde_helpers::non_empty_string_vec,
};

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
    /// * `always` — always show the task's pane, and focus the corresponding tab in it (default)
    // * `no_focus` — always show the task's pane, add the task's tab in it, but don't focus it
    // * `never` — do not alter focus, but still add/reuse the task's tab in its pane
    #[serde(default)]
    pub reveal: RevealStrategy,
    /// Where to place the task's terminal item after starting the task.
    /// * `dock` — in the terminal dock, "regular" terminal items' place (default).
    /// * `center` — in the central pane group, "main" editor area.
    #[serde(default)]
    pub reveal_target: RevealTarget,
    /// What to do with the terminal pane and tab, after the command had finished:
    /// * `never` — do nothing when the command finishes (default)
    /// * `always` — always hide the terminal tab, hide the pane also if it was the last tab in it
    /// * `on_success` — hide the terminal tab on task success only, otherwise behaves similar to `always`.
    #[serde(default)]
    pub hide: HideStrategy,
    /// Represents the tags which this template attaches to.
    /// Adding this removes this task from other UI and gives you ability to run it by tag.
    #[serde(default, deserialize_with = "non_empty_string_vec")]
    #[schemars(length(min = 1))]
    pub tags: Vec<String>,
    /// Which shell to use when spawning the task.
    #[serde(default)]
    pub shell: Shell,
    /// Whether to show the task line in the task output.
    #[serde(default = "default_true")]
    pub show_summary: bool,
    /// Whether to show the command line in the task output.
    #[serde(default = "default_true")]
    pub show_command: bool,
}

#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
/// Use to represent debug request type
pub enum DebugArgsRequest {
    /// launch (program, cwd) are stored in TaskTemplate as (command, cwd)
    Launch,
    /// Attach
    Attach(AttachRequest),
}

/// What to do with the terminal pane and tab, after the command was started.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RevealStrategy {
    /// Always show the task's pane, and focus the corresponding tab in it.
    #[default]
    Always,
    /// Always show the task's pane, add the task's tab in it, but don't focus it.
    NoFocus,
    /// Do not alter focus, but still add/reuse the task's tab in its pane.
    Never,
}

/// What to do with the terminal pane and tab, after the command has finished.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HideStrategy {
    /// Do nothing when the command finishes.
    #[default]
    Never,
    /// Always hide the terminal tab, hide the pane also if it was the last tab in it.
    Always,
    /// Hide the terminal tab on task success only, otherwise behaves similar to `Always`.
    OnSuccess,
}

/// A group of Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TaskTemplates(pub Vec<TaskTemplate>);

impl TaskTemplates {
    /// Generates JSON schema of Tasks JSON template format.
    pub fn generate_json_schema() -> serde_json_lenient::Value {
        let schema = schemars::generate::SchemaSettings::draft2019_09()
            .with_transform(DefaultDenyUnknownFields)
            .into_generator()
            .root_schema_for::<Self>();

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
    pub fn resolve_task(&self, id_base: &str, cx: &TaskContext) -> Option<ResolvedTask> {
        if self.label.trim().is_empty() || self.command.trim().is_empty() {
            return None;
        }

        let mut variable_names = HashMap::default();
        let mut substituted_variables = HashSet::default();
        let task_variables = cx
            .task_variables
            .0
            .iter()
            .map(|(key, value)| {
                let key_string = key.to_string();
                if !variable_names.contains_key(&key_string) {
                    variable_names.insert(key_string.clone(), key.clone());
                }
                (key_string, value.as_str())
            })
            .collect::<HashMap<_, _>>();
        let truncated_variables = truncate_variables(&task_variables);
        let cwd = match self.cwd.as_deref() {
            Some(cwd) => {
                let substituted_cwd = substitute_all_template_variables_in_str(
                    cwd,
                    &task_variables,
                    &variable_names,
                    &mut substituted_variables,
                )?;
                Some(PathBuf::from(substituted_cwd))
            }
            None => None,
        }
        .or(cx.cwd.clone());
        let full_label = substitute_all_template_variables_in_str(
            &self.label,
            &task_variables,
            &variable_names,
            &mut substituted_variables,
        )?;

        // Arbitrarily picked threshold below which we don't truncate any variables.
        const TRUNCATION_THRESHOLD: usize = 64;

        let human_readable_label = if full_label.len() > TRUNCATION_THRESHOLD {
            substitute_all_template_variables_in_str(
                &self.label,
                &truncated_variables,
                &variable_names,
                &mut substituted_variables,
            )?
        } else {
            #[allow(
                clippy::redundant_clone,
                reason = "We want to clone the full_label to avoid borrowing it in the fold closure"
            )]
            full_label.clone()
        }
        .lines()
        .fold(String::new(), |mut string, line| {
            if string.is_empty() {
                string.push_str(line);
            } else {
                string.push_str("\\n");
                string.push_str(line);
            }
            string
        });

        let command = substitute_all_template_variables_in_str(
            &self.command,
            &task_variables,
            &variable_names,
            &mut substituted_variables,
        )?;
        let args_with_substitutions = substitute_all_template_variables_in_vec(
            &self.args,
            &task_variables,
            &variable_names,
            &mut substituted_variables,
        )?;

        let task_hash = to_hex_hash(self)
            .context("hashing task template")
            .log_err()?;
        let variables_hash = to_hex_hash(&task_variables)
            .context("hashing task variables")
            .log_err()?;
        let id = TaskId(format!("{id_base}_{task_hash}_{variables_hash}"));

        let env = {
            // Start with the project environment as the base.
            let mut env = cx.project_env.clone();

            // Extend that environment with what's defined in the TaskTemplate
            env.extend(self.env.clone());

            // Then we replace all task variables that could be set in environment variables
            let mut env = substitute_all_template_variables_in_map(
                &env,
                &task_variables,
                &variable_names,
                &mut substituted_variables,
            )?;

            // Last step: set the task variables as environment variables too
            env.extend(task_variables.into_iter().map(|(k, v)| (k, v.to_owned())));
            env
        };

        Some(ResolvedTask {
            id: id.clone(),
            substituted_variables,
            original_task: self.clone(),
            resolved_label: full_label.clone(),
            resolved: SpawnInTerminal {
                id,
                cwd,
                full_label,
                label: human_readable_label,
                command_label: args_with_substitutions.iter().fold(
                    command.clone(),
                    |mut command_label, arg| {
                        command_label.push(' ');
                        command_label.push_str(arg);
                        command_label
                    },
                ),
                command: Some(command),
                args: args_with_substitutions,
                env,
                use_new_terminal: self.use_new_terminal,
                allow_concurrent_runs: self.allow_concurrent_runs,
                reveal: self.reveal,
                reveal_target: self.reveal_target,
                hide: self.hide,
                shell: self.shell.clone(),
                show_summary: self.show_summary,
                show_command: self.show_command,
                show_rerun: true,
            },
        })
    }
}

const MAX_DISPLAY_VARIABLE_LENGTH: usize = 15;

fn truncate_variables(task_variables: &HashMap<String, &str>) -> HashMap<String, String> {
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

pub fn substitute_variables_in_str(template_str: &str, context: &TaskContext) -> Option<String> {
    let mut variable_names = HashMap::default();
    let mut substituted_variables = HashSet::default();
    let task_variables = context
        .task_variables
        .0
        .iter()
        .map(|(key, value)| {
            let key_string = key.to_string();
            if !variable_names.contains_key(&key_string) {
                variable_names.insert(key_string.clone(), key.clone());
            }
            (key_string, value.as_str())
        })
        .collect::<HashMap<_, _>>();
    substitute_all_template_variables_in_str(
        template_str,
        &task_variables,
        &variable_names,
        &mut substituted_variables,
    )
}
fn substitute_all_template_variables_in_str<A: AsRef<str>>(
    template_str: &str,
    task_variables: &HashMap<String, A>,
    variable_names: &HashMap<String, VariableName>,
    substituted_variables: &mut HashSet<VariableName>,
) -> Option<String> {
    let substituted_string = shellexpand::env_with_context(template_str, |var| {
        // Colons denote a default value in case the variable is not set. We want to preserve that default, as otherwise shellexpand will substitute it for us.
        let colon_position = var.find(':').unwrap_or(var.len());
        let (variable_name, default) = var.split_at(colon_position);
        if let Some(name) = task_variables.get(variable_name) {
            if let Some(substituted_variable) = variable_names.get(variable_name) {
                substituted_variables.insert(substituted_variable.clone());
            }

            let mut name = name.as_ref().to_owned();
            // Got a task variable hit
            if !default.is_empty() {
                name.push_str(default);
            }
            return Ok(Some(name));
        } else if variable_name.starts_with(ZED_VARIABLE_NAME_PREFIX) {
            bail!("Unknown variable name: {variable_name}");
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
    template_strs: &[String],
    task_variables: &HashMap<String, &str>,
    variable_names: &HashMap<String, VariableName>,
    substituted_variables: &mut HashSet<VariableName>,
) -> Option<Vec<String>> {
    let mut expanded = Vec::with_capacity(template_strs.len());
    for variable in template_strs {
        let new_value = substitute_all_template_variables_in_str(
            variable,
            task_variables,
            variable_names,
            substituted_variables,
        )?;
        expanded.push(new_value);
    }
    Some(expanded)
}

pub fn substitute_variables_in_map(
    keys_and_values: &HashMap<String, String>,
    context: &TaskContext,
) -> Option<HashMap<String, String>> {
    let mut variable_names = HashMap::default();
    let mut substituted_variables = HashSet::default();
    let task_variables = context
        .task_variables
        .0
        .iter()
        .map(|(key, value)| {
            let key_string = key.to_string();
            if !variable_names.contains_key(&key_string) {
                variable_names.insert(key_string.clone(), key.clone());
            }
            (key_string, value.as_str())
        })
        .collect::<HashMap<_, _>>();
    substitute_all_template_variables_in_map(
        keys_and_values,
        &task_variables,
        &variable_names,
        &mut substituted_variables,
    )
}
fn substitute_all_template_variables_in_map(
    keys_and_values: &HashMap<String, String>,
    task_variables: &HashMap<String, &str>,
    variable_names: &HashMap<String, VariableName>,
    substituted_variables: &mut HashSet<VariableName>,
) -> Option<HashMap<String, String>> {
    let mut new_map: HashMap<String, String> = Default::default();
    for (key, value) in keys_and_values {
        let new_value = substitute_all_template_variables_in_str(
            value,
            task_variables,
            variable_names,
            substituted_variables,
        )?;
        let new_key = substitute_all_template_variables_in_str(
            key,
            task_variables,
            variable_names,
            substituted_variables,
        )?;
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
                ..task_with_all_properties
            },
        ] {
            assert_eq!(
                task_with_blank_property.resolve_task(TEST_ID_BASE, &TaskContext::default()),
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
            assert_substituted_variables(&resolved_task, Vec::new());
            resolved_task.resolved
        };

        let cx = TaskContext {
            cwd: None,
            task_variables: TaskVariables::default(),
            project_env: HashMap::default(),
        };
        assert_eq!(
            resolved_task(&task_without_cwd, &cx).cwd,
            None,
            "When neither task nor task context have cwd, it should be None"
        );

        let context_cwd = Path::new("a").join("b").join("c");
        let cx = TaskContext {
            cwd: Some(context_cwd.clone()),
            task_variables: TaskVariables::default(),
            project_env: HashMap::default(),
        };
        assert_eq!(
            resolved_task(&task_without_cwd, &cx).cwd,
            Some(context_cwd.clone()),
            "TaskContext's cwd should be taken on resolve if task's cwd is None"
        );

        let task_cwd = Path::new("d").join("e").join("f");
        let mut task_with_cwd = task_without_cwd.clone();
        task_with_cwd.cwd = Some(task_cwd.display().to_string());
        let task_with_cwd = task_with_cwd;

        let cx = TaskContext {
            cwd: None,
            task_variables: TaskVariables::default(),
            project_env: HashMap::default(),
        };
        assert_eq!(
            resolved_task(&task_with_cwd, &cx).cwd,
            Some(task_cwd.clone()),
            "TaskTemplate's cwd should be taken on resolve if TaskContext's cwd is None"
        );

        let cx = TaskContext {
            cwd: Some(context_cwd),
            task_variables: TaskVariables::default(),
            project_env: HashMap::default(),
        };
        assert_eq!(
            resolved_task(&task_with_cwd, &cx).cwd,
            Some(task_cwd),
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
                        "env_var_2 {} {}",
                        custom_variable_1.template_value(),
                        custom_variable_2.template_value()
                    ),
                ),
                (
                    "env_key_3".to_string(),
                    format!("env_var_3 {}", VariableName::Symbol.template_value()),
                ),
            ]),
            ..TaskTemplate::default()
        };

        let mut first_resolved_id = None;
        for i in 0..15 {
            let resolved_task = task_with_all_variables.resolve_task(
                TEST_ID_BASE,
                &TaskContext {
                    cwd: None,
                    task_variables: TaskVariables::from_iter(all_variables.clone()),
                    project_env: HashMap::default(),
                },
            ).unwrap_or_else(|| panic!("Should successfully resolve task {task_with_all_variables:?} with variables {all_variables:?}"));

            match &first_resolved_id {
                None => first_resolved_id = Some(resolved_task.id.clone()),
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
            assert_substituted_variables(
                &resolved_task,
                all_variables.iter().map(|(name, _)| name.clone()).collect(),
            );

            let spawn_in_terminal = &resolved_task.resolved;
            assert_eq!(
                spawn_in_terminal.label,
                format!(
                    "test label for 1234 and …{}",
                    &long_value[long_value.len() - MAX_DISPLAY_VARIABLE_LENGTH..]
                ),
                "Human-readable label should have long substitutions trimmed"
            );
            assert_eq!(
                spawn_in_terminal.command.clone().unwrap(),
                format!("echo test_file {long_value}"),
                "Command should be substituted with variables and those should not be shortened"
            );
            assert_eq!(
                spawn_in_terminal.args,
                &[
                    "arg1 test_selected_text",
                    "arg2 5678",
                    "arg3 010101010101010101010101010101010101010101010101010101010101",
                ],
                "Args should be substituted with variables"
            );
            assert_eq!(
                spawn_in_terminal.command_label,
                format!(
                    "{} arg1 test_selected_text arg2 5678 arg3 {long_value}",
                    spawn_in_terminal.command.clone().unwrap()
                ),
                "Command label args should be substituted with variables and those should not be shortened"
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
                Some("env_var_2 test_custom_variable_1 test_custom_variable_2")
            );
            assert_eq!(
                spawn_in_terminal.env.get("env_key_3"),
                Some(&format!("env_var_3 {long_value}")),
                "Env vars should be substituted with variables and those should not be shortened"
            );
        }

        for i in 0..all_variables.len() {
            let mut not_all_variables = all_variables.to_vec();
            let removed_variable = not_all_variables.remove(i);
            let resolved_task_attempt = task_with_all_variables.resolve_task(
                TEST_ID_BASE,
                &TaskContext {
                    cwd: None,
                    task_variables: TaskVariables::from_iter(not_all_variables),
                    project_env: HashMap::default(),
                },
            );
            assert_eq!(
                resolved_task_attempt, None,
                "If any of the Zed task variables is not substituted, the task should not be resolved, but got some resolution without the variable {removed_variable:?} (index {i})"
            );
        }
    }

    #[test]
    fn test_can_resolve_free_variables() {
        let task = TaskTemplate {
            label: "My task".into(),
            command: "echo".into(),
            args: vec!["$PATH".into()],
            ..TaskTemplate::default()
        };
        let resolved_task = task
            .resolve_task(TEST_ID_BASE, &TaskContext::default())
            .unwrap();
        assert_substituted_variables(&resolved_task, Vec::new());
        let resolved = resolved_task.resolved;
        assert_eq!(resolved.label, task.label);
        assert_eq!(resolved.command, Some(task.command));
        assert_eq!(resolved.args, task.args);
    }

    #[test]
    fn test_errors_on_missing_zed_variable() {
        let task = TaskTemplate {
            label: "My task".into(),
            command: "echo".into(),
            args: vec!["$ZED_VARIABLE".into()],
            ..TaskTemplate::default()
        };
        assert!(
            task.resolve_task(TEST_ID_BASE, &TaskContext::default())
                .is_none()
        );
    }

    #[test]
    fn test_symbol_dependent_tasks() {
        let task_with_all_properties = TaskTemplate {
            label: "test_label".to_string(),
            command: "test_command".to_string(),
            args: vec!["test_arg".to_string()],
            env: HashMap::from_iter([("test_env_key".to_string(), "test_env_var".to_string())]),
            ..TaskTemplate::default()
        };
        let cx = TaskContext {
            cwd: None,
            task_variables: TaskVariables::from_iter(Some((
                VariableName::Symbol,
                "test_symbol".to_string(),
            ))),
            project_env: HashMap::default(),
        };

        for (i, symbol_dependent_task) in [
            TaskTemplate {
                label: format!("test_label_{}", VariableName::Symbol.template_value()),
                ..task_with_all_properties.clone()
            },
            TaskTemplate {
                command: format!("test_command_{}", VariableName::Symbol.template_value()),
                ..task_with_all_properties.clone()
            },
            TaskTemplate {
                args: vec![format!(
                    "test_arg_{}",
                    VariableName::Symbol.template_value()
                )],
                ..task_with_all_properties.clone()
            },
            TaskTemplate {
                env: HashMap::from_iter([(
                    "test_env_key".to_string(),
                    format!("test_env_var_{}", VariableName::Symbol.template_value()),
                )]),
                ..task_with_all_properties
            },
        ]
        .into_iter()
        .enumerate()
        {
            let resolved = symbol_dependent_task
                .resolve_task(TEST_ID_BASE, &cx)
                .unwrap_or_else(|| panic!("Failed to resolve task {symbol_dependent_task:?}"));
            assert_eq!(
                resolved.substituted_variables,
                HashSet::from_iter(Some(VariableName::Symbol)),
                "(index {i}) Expected the task to depend on symbol task variable: {resolved:?}"
            )
        }
    }

    #[track_caller]
    fn assert_substituted_variables(resolved_task: &ResolvedTask, mut expected: Vec<VariableName>) {
        let mut resolved_variables = resolved_task
            .substituted_variables
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        resolved_variables.sort_by_key(|var| var.to_string());
        expected.sort_by_key(|var| var.to_string());
        assert_eq!(resolved_variables, expected)
    }

    #[test]
    fn substitute_funky_labels() {
        let faulty_go_test = TaskTemplate {
            label: format!(
                "go test {}/{}",
                VariableName::Symbol.template_value(),
                VariableName::Symbol.template_value(),
            ),
            command: "go".into(),
            args: vec![format!(
                "^{}$/^{}$",
                VariableName::Symbol.template_value(),
                VariableName::Symbol.template_value()
            )],
            ..TaskTemplate::default()
        };
        let mut context = TaskContext::default();
        context
            .task_variables
            .insert(VariableName::Symbol, "my-symbol".to_string());
        assert!(faulty_go_test.resolve_task("base", &context).is_some());
    }

    #[test]
    fn test_project_env() {
        let all_variables = [
            (VariableName::Row, "1234".to_string()),
            (VariableName::Column, "5678".to_string()),
            (VariableName::File, "test_file".to_string()),
            (VariableName::Symbol, "my symbol".to_string()),
        ];

        let template = TaskTemplate {
            label: "my task".to_string(),
            command: format!(
                "echo {} {}",
                VariableName::File.template_value(),
                VariableName::Symbol.template_value(),
            ),
            args: vec![],
            env: HashMap::from_iter([
                (
                    "TASK_ENV_VAR1".to_string(),
                    "TASK_ENV_VAR1_VALUE".to_string(),
                ),
                (
                    "TASK_ENV_VAR2".to_string(),
                    format!(
                        "env_var_2 {} {}",
                        VariableName::Row.template_value(),
                        VariableName::Column.template_value()
                    ),
                ),
                (
                    "PROJECT_ENV_WILL_BE_OVERWRITTEN".to_string(),
                    "overwritten".to_string(),
                ),
            ]),
            ..TaskTemplate::default()
        };

        let project_env = HashMap::from_iter([
            (
                "PROJECT_ENV_VAR1".to_string(),
                "PROJECT_ENV_VAR1_VALUE".to_string(),
            ),
            (
                "PROJECT_ENV_WILL_BE_OVERWRITTEN".to_string(),
                "PROJECT_ENV_WILL_BE_OVERWRITTEN_VALUE".to_string(),
            ),
        ]);

        let context = TaskContext {
            cwd: None,
            task_variables: TaskVariables::from_iter(all_variables),
            project_env,
        };

        let resolved = template
            .resolve_task(TEST_ID_BASE, &context)
            .unwrap()
            .resolved;

        assert_eq!(resolved.env["TASK_ENV_VAR1"], "TASK_ENV_VAR1_VALUE");
        assert_eq!(resolved.env["TASK_ENV_VAR2"], "env_var_2 1234 5678");
        assert_eq!(resolved.env["PROJECT_ENV_VAR1"], "PROJECT_ENV_VAR1_VALUE");
        assert_eq!(
            resolved.env["PROJECT_ENV_WILL_BE_OVERWRITTEN"],
            "overwritten"
        );
    }
}
