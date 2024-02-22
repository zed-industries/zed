//! Definitions of tasks with a static file config definition, not dependent on the application state.

use std::path::{Path, PathBuf};

use crate::{static_source::Definition, SpawnInTerminal, Task, TaskId};

/// A single config file entry with the deserialized task definition.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticTask {
    id: TaskId,
    definition: Definition,
}

impl StaticTask {
    pub(super) fn new(id: usize, task_definition: Definition) -> Self {
        Self {
            id: TaskId(format!("static_{}_{}", task_definition.label, id)),
            definition: task_definition,
        }
    }
}

impl Task for StaticTask {
    fn exec(&self, cwd: Option<PathBuf>) -> Option<SpawnInTerminal> {
        Some(SpawnInTerminal {
            id: self.id.clone(),
            cwd,
            use_new_terminal: self.definition.use_new_terminal,
            allow_concurrent_runs: self.definition.allow_concurrent_runs,
            label: self.definition.label.clone(),
            command: self.definition.command.clone(),
            args: self.definition.args.clone(),
            env: self.definition.env.clone(),
            separate_shell: false,
        })
    }

    fn name(&self) -> &str {
        &self.definition.label
    }

    fn id(&self) -> &TaskId {
        &self.id
    }

    fn cwd(&self) -> Option<&Path> {
        self.definition.cwd.as_deref()
    }
}
