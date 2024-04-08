use std::sync::Arc;

use crate::{Definition, ResolvedTask, SpawnInTerminal, Task, TaskContext, TaskId};

/// TODO kb docs
#[derive(Clone, Debug, PartialEq)]
pub(super) struct TaskTemplate {
    /// TODO kb docs
    pub id: TaskId,
    /// TODO kb docs
    pub definition: Definition,
}

impl Task for TaskTemplate {
    fn resolve_task(&self, cx: TaskContext) -> Option<ResolvedTask> {
        let TaskContext {
            cwd,
            task_variables,
        } = cx;
        // TODO kb ensure all substitutions are possible to do: no `cwd` has the task prefix, no `env`, `args`, `label`, or `command` have vars with task prefix that are not in `task_variables`. Omit such tasks. + test this
        let task_variables = task_variables.into_env_variables();
        let cwd = self
            .definition
            .cwd
            .clone()
            .and_then(|path| {
                subst::substitute(&path, &task_variables)
                    .map(Into::into)
                    .ok()
            })
            .or(cwd);
        let mut definition_env = self.definition.env.clone();
        definition_env.extend(task_variables);
        Some(ResolvedTask::SpawnInTerminal(
            SpawnInTerminal {
                id: self.id.clone(),
                cwd,
                use_new_terminal: self.definition.use_new_terminal,
                allow_concurrent_runs: self.definition.allow_concurrent_runs,
                // TODO kb use expanded label here
                label: self.definition.label.clone(),
                command: self.definition.command.clone(),
                args: self.definition.args.clone(),
                reveal: self.definition.reveal,
                env: definition_env,
            },
            Arc::new(self.clone()),
        ))
    }

    fn name(&self) -> &str {
        &self.definition.label
    }

    fn id(&self) -> &TaskId {
        &self.id
    }

    fn cwd(&self) -> Option<&str> {
        self.definition.cwd.as_deref()
    }
}
