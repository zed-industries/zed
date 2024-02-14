//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.
use std::path::PathBuf;

use crate::{static_runnable_file::Definition, Handle, Runnable};
use gpui::AppContext;

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

    fn exec(&self, cwd: Option<PathBuf>, cx: &mut AppContext) -> anyhow::Result<Handle> {
        Ok(Handle::new(&self.runnable, cwd, cx))
    }

    fn name(&self) -> String {
        self.runnable.label.clone()
    }
}
