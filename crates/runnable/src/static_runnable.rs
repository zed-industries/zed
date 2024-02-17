//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.

use std::path::PathBuf;

use crate::{static_runnable_file::Definition, Runnable, SpawnTaskInTerminal};

/// [`StaticRunnable`] is a [`Runnable`] defined in .json file.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticRunnable {
    id: String,
    definition: Definition,
}

impl StaticRunnable {
    pub fn new(id: usize, runnable: Definition) -> Self {
        Self {
            id: format!("static_{id}"),
            definition: runnable,
        }
    }
}

impl Runnable for StaticRunnable {
    fn boxed_clone(&self) -> Box<dyn Runnable> {
        Box::new(self.clone())
    }

    fn exec(&self, cwd: Option<PathBuf>) -> Option<SpawnTaskInTerminal> {
        Some(SpawnTaskInTerminal {
            task_id: self.id.clone(),
            use_new_terminal: self.definition.spawn_in_new_terminal,
            label: self.definition.label.clone(),
            command: self.definition.command.clone(),
            args: self.definition.args.clone(),
            cwd,
        })
    }

    fn name(&self) -> &str {
        &self.definition.label
    }

    fn id(&self) -> &str {
        &self.id
    }
}
