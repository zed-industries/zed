//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.

use std::path::PathBuf;

use crate::{static_runnable_file::Definition, Handle, Runnable, SpawnTaskInTerminal};

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

    fn exec(&self, id: usize, cwd: Option<PathBuf>) -> (Handle, Option<SpawnTaskInTerminal>) {
        let (completion_tx, completion_rx) = smol::channel::bounded(2);
        let (cancelation_tx, cancellation_rx) = smol::channel::bounded(2);
        let handle = Handle {
            completion_rx,
            cancelation_tx,
        };
        let spawn_task = SpawnTaskInTerminal {
            task_id: id,
            use_new_terminal: self.runnable.spawn_in_new_terminal,
            label: self.runnable.label.clone(),
            command: self.runnable.command.clone(),
            args: self.runnable.args.clone(),
            cwd,
            cancellation_rx: Some(cancellation_rx),
            completion_tx: Some(completion_tx),
        };
        (handle, Some(spawn_task))
    }

    fn name(&self) -> String {
        self.runnable.label.clone()
    }
}
