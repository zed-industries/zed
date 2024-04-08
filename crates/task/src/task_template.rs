use std::sync::Arc;

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
    fn resolve_task(&self, cx: TaskContext) -> Option<ResolvedTask> {
        let TaskContext {
            cwd,
            task_variables,
        } = cx;
        // TODO kb ensure all substitutions are possible to do: no `cwd` has the task prefix, no `env`, `args`, `label`, or `command` have vars with task prefix that are not in `task_variables`. Omit such tasks. + test this
        let task_variables = task_variables.into_env_variables();
        let cwd = self
            .template
            .cwd
            .clone()
            .and_then(|path| {
                subst::substitute(&path, &task_variables)
                    .map(Into::into)
                    .ok()
            })
            .or(cwd);
        let mut template_env = self.template.env.clone();
        template_env.extend(task_variables);
        Some(ResolvedTask::SpawnInTerminal(
            SpawnInTerminal {
                id: self.id.clone(),
                cwd,
                use_new_terminal: self.template.use_new_terminal,
                allow_concurrent_runs: self.template.allow_concurrent_runs,
                // TODO kb use expanded label here
                label: self.template.label.clone(),
                command: self.template.command.clone(),
                args: self.template.args.clone(),
                reveal: self.template.reveal,
                env: template_env,
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
