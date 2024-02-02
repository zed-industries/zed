use std::path::Path;

use anyhow::{bail, Result};
use gpui::{AppContext, Model};
use runnable::{ExecutionResult, Runnable, RunnableId, RunnablePebble, Source, TaskHandle};

/// Inventory tracks available runnables for a given project.
#[derive(Default)]
pub(crate) struct Inventory {
    sources: Vec<Model<Box<dyn Source>>>,
}

impl Inventory {
    pub(crate) fn add_source(&mut self, source: Model<Box<dyn Source>>) {
        self.sources.push(source);
    }
    pub(crate) fn list_runnables<'a>(
        &'a self,
        path: &'a Path,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = RunnablePebble> + 'a {
        self.sources
            .iter()
            .flat_map(|source| source.read(cx).runnables_for_path(path))
    }
}
