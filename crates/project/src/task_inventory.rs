//! Project-wide storage of the tasks available, capable of updating itself from the sources set.

use std::{any::TypeId, path::Path, sync::Arc};

use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use task::{Source, Task, TaskId};

/// Inventory tracks available tasks for a given project.
pub struct Inventory {
    sources: Vec<SourceInInventory>,
    pub last_scheduled_task: Option<TaskId>,
}

struct SourceInInventory {
    source: Model<Box<dyn Source>>,
    _subscription: Subscription,
    type_id: TypeId,
}

impl Inventory {
    pub(crate) fn new(cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_| Self {
            sources: Vec::new(),
            last_scheduled_task: None,
        })
    }

    /// Registers a new tasks source, that would be fetched for available tasks.
    pub fn add_source(&mut self, source: Model<Box<dyn Source>>, cx: &mut ModelContext<Self>) {
        let _subscription = cx.observe(&source, |_, _, cx| {
            cx.notify();
        });
        let type_id = source.read(cx).type_id();
        let source = SourceInInventory {
            source,
            _subscription,
            type_id,
        };
        self.sources.push(source);
        cx.notify();
    }
    pub fn source<T: Source>(&self) -> Option<Model<Box<dyn Source>>> {
        let target_type_id = std::any::TypeId::of::<T>();
        self.sources.iter().find_map(
            |SourceInInventory {
                 type_id, source, ..
             }| {
                if &target_type_id == type_id {
                    Some(source.clone())
                } else {
                    None
                }
            },
        )
    }

    /// Pulls its sources to list runanbles for the path given (up to the source to decide what to return for no path).
    pub fn list_tasks(&self, path: Option<&Path>, cx: &mut AppContext) -> Vec<Arc<dyn Task>> {
        let mut tasks = Vec::new();
        for source in &self.sources {
            tasks.extend(
                source
                    .source
                    .update(cx, |source, cx| source.tasks_for_path(path, cx)),
            );
        }
        tasks
    }

    /// Returns the last scheduled task, if any of the sources contains one with the matching id.
    pub fn last_scheduled_task(&self, cx: &mut AppContext) -> Option<Arc<dyn Task>> {
        self.last_scheduled_task.as_ref().and_then(|id| {
            // TODO straighten the `Path` story to understand what has to be passed here: or it will break in the future.
            self.list_tasks(None, cx)
                .into_iter()
                .find(|task| task.id() == id)
        })
    }
}
