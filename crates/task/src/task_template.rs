use std::{path::PathBuf, sync::Arc};

use collections::HashMap;

use crate::{
    ResolvedTask, SpawnInTerminal, Task, TaskContext, TaskId, TaskTemplate,
    ZED_VARIABLE_NAME_PREFIX,
};

/// TODO kb docs
#[derive(Clone, Debug, PartialEq)]
pub(super) struct TaskForTemplate {
    /// TODO kb docs
    pub id: TaskId,
    /// TODO kb docs
    pub template: TaskTemplate,
}

impl Task for TaskForTemplate {
    // TODO kb tests
    fn resolve_task(&self, cx: TaskContext) -> Option<ResolvedTask> {
        let TaskContext {
            cwd,
            task_variables,
        } = cx;
        let task_variables = task_variables.into_env_variables();
        let cwd = match self.template.cwd.as_deref() {
            Some(cwd) => Some(substitute_all_template_variables_in_str(
                cwd,
                &task_variables,
            )?),
            None => None,
        }
        .map(PathBuf::from)
        .or(cwd);
        let label =
            substitute_all_template_variables_in_str(&self.template.label, &task_variables)?;
        let command =
            substitute_all_template_variables_in_str(&self.template.command, &task_variables)?;
        let args =
            substitute_all_template_variables_in_vec(self.template.args.clone(), &task_variables)?;
        let mut env =
            substitute_all_template_variables_in_map(self.template.env.clone(), &task_variables)?;
        env.extend(task_variables);
        Some(ResolvedTask::SpawnInTerminal(
            SpawnInTerminal {
                id: self.id.clone(),
                cwd,
                label,
                command,
                args,
                env,
                use_new_terminal: self.template.use_new_terminal,
                allow_concurrent_runs: self.template.allow_concurrent_runs,
                reveal: self.template.reveal,
            },
            Arc::new(self.clone()),
        ))
    }

    fn name(&self) -> &str {
        &self.template.label
    }

    fn id(&self) -> &TaskId {
        &self.id
    }

    fn cwd(&self) -> Option<&str> {
        self.template.cwd.as_deref()
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
