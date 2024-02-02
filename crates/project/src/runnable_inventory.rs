use std::path::Path;

use anyhow::{bail, Result};
use gpui::{AppContext, Model};
use runnable::{ExecutionResult, Runnable, RunnableId, RunnablePebble, Source, TaskHandle};

/// Inventory tracks available runnables for a given project.
#[derive(Default)]
pub struct Inventory {
    sources: Vec<Box<dyn Source>>,
}

impl Inventory {
    pub fn add_source(&mut self, source: impl Source + 'static) {
        self.sources.push(Box::new(source));
    }

    pub fn list_runnables<'a>(
        &'a self,
        path: &'a Path,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = RunnablePebble> + 'a {
        self.sources
            .iter()
            .flat_map(|source| source.runnables_for_path(path, cx).unwrap())
    }
}
