use std::{path::Path, sync::Arc};

use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use runnable::{Runnable, RunnableId, Source};

struct SourceInInventory {
    source: Model<Box<dyn Source>>,
    _subscription: Subscription,
}

/// Inventory tracks available runnables for a given project.
pub struct Inventory {
    sources: Vec<SourceInInventory>,
    pub last_scheduled_runnable: Option<RunnableId>,
}

impl Inventory {
    pub(crate) fn new(cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_| Self {
            sources: Vec::new(),
            last_scheduled_runnable: None,
        })
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

    pub fn list_runnables(&self, cx: &mut AppContext) -> Vec<Arc<dyn Runnable>> {
        let mut runnables = Vec::new();
        for source in &self.sources {
            runnables.extend(
                source
                    .source
                    .update(cx, |this, cx| this.runnables_for_path(Path::new(""), cx)),
            );
        }
        runnables
    }

    pub fn last_schedule_runnable(&self, cx: &mut AppContext) -> Option<Arc<dyn Runnable>> {
        self.last_scheduled_runnable.as_ref().and_then(|id| {
            self.list_runnables(cx)
                .into_iter()
                .find(|runnable| runnable.id() == id)
        })
    }
}
