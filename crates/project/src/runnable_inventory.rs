use std::path::Path;

use anyhow::{bail, Result};
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use runnable::{RunnablePebble, Source};

struct SourceInInventory {
    source: Model<Box<dyn Source>>,
    _subscription: Subscription,
}

/// Inventory tracks available runnables for a given project.
pub struct Inventory {
    sources: Vec<SourceInInventory>,
}

impl Inventory {
    pub(crate) fn new(cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_| Self { sources: vec![] })
    }
    pub fn add_source(&mut self, source: impl Source + 'static, cx: &mut ModelContext<Self>) {
        let source: Model<Box<dyn Source>> = cx.new_model(|_| Box::new(source) as Box<dyn Source>);
        let _subscription = cx.observe(&source, |_, _, cx| {
            cx.notify();
        });
        let source = SourceInInventory {
            source,
            _subscription,
        };
        self.sources.push(source);
    }

    pub fn list_runnables<'a>(
        &'a self,
        path: &'a Path,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = RunnablePebble> + 'a {
        self.sources
            .iter()
            .flat_map(|source| source.source.read(cx).runnables_for_path(path, cx).unwrap())
    }
}
