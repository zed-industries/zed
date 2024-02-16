//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.

use std::path::PathBuf;

use crate::{static_runnable_file::Definition, Runnable, SpawnTaskInTerminal};

/// [`StaticRunner`] is a [`Runnable`] defined in .json file.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticRunner {
    runnable: Definition,
}

impl StaticRunner {
    pub fn new(runnable: Definition) -> Self {
        Self { runnable }
    }
}

impl Runnable for StaticRunner {
    fn boxed_clone(&self) -> Box<dyn Runnable> {
        Box::new(self.clone())
    }

    fn exec(&self, id: usize, cwd: Option<PathBuf>) -> Option<SpawnTaskInTerminal> {
        Some(SpawnTaskInTerminal {
            task_id: id,
            use_new_terminal: self.runnable.spawn_in_new_terminal,
            label: self.runnable.label.clone(),
            command: self.runnable.command.clone(),
            args: self.runnable.args.clone(),
            cwd,
        })
    }

    fn name(&self) -> String {
        self.runnable.label.clone()
    }
}
