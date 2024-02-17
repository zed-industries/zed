use std::{path::Path, sync::Arc};

use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use runnable::{Runnable, Source};

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

    pub fn add_source(&mut self, source: Model<Box<dyn Source>>, cx: &mut ModelContext<Self>) {
        let _subscription = cx.observe(&source, |_, _, cx| {
            cx.notify();
        });
        let source = SourceInInventory {
            source,
            _subscription,
        };
        self.sources.push(source);
        cx.notify();
    }

    pub fn list_runnables(&self, path: &Path, cx: &mut AppContext) -> Vec<Arc<dyn Runnable>> {
        let mut runnables = Vec::with_capacity(self.sources.len());
        for source in &self.sources {
            runnables.extend(
                source
                    .source
                    .update(cx, |this, cx| this.runnables_for_path(path, cx)),
            );
        }
        runnables
    }
}
