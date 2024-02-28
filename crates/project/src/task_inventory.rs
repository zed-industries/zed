//! Project-wide storage of the tasks available, capable of updating itself from the sources set.

use std::{any::TypeId, path::Path, sync::Arc};

use collections::{HashMap, VecDeque};
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use itertools::Itertools;
use task::{Source, Task, TaskId};
use util::post_inc;

/// Inventory tracks available tasks for a given project.
pub struct Inventory {
    sources: Vec<SourceInInventory>,
    last_scheduled_tasks: VecDeque<TaskId>,
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
            last_scheduled_tasks: VecDeque::new(),
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
    pub fn list_tasks(
        &self,
        path: Option<&Path>,
        lru: bool,
        cx: &mut AppContext,
    ) -> Vec<Arc<dyn Task>> {
        let mut lru_score = 0_u32;
        let tasks_by_usage = if lru {
            self.last_scheduled_tasks
                .iter()
                .rev()
                .fold(HashMap::default(), |mut tasks, id| {
                    tasks.entry(id).or_insert_with(|| post_inc(&mut lru_score));
                    tasks
                })
        } else {
            HashMap::default()
        };
        self.sources
            .iter()
            .flat_map(|source| {
                source
                    .source
                    .update(cx, |source, cx| source.tasks_for_path(path, cx))
            })
            .map(|task| {
                let usages = if lru {
                    tasks_by_usage
                        .get(&task.id())
                        .copied()
                        .unwrap_or_else(|| post_inc(&mut lru_score))
                } else {
                    post_inc(&mut lru_score)
                };
                (task, usages)
            })
            .sorted_unstable_by(|(task_a, usages_a), (task_b, usages_b)| {
                usages_a
                    .cmp(usages_b)
                    .then(task_a.name().cmp(task_b.name()))
            })
            .map(|(task, _)| task)
            .collect()
    }

    /// Returns the last scheduled task, if any of the sources contains one with the matching id.
    pub fn last_scheduled_task(&self, cx: &mut AppContext) -> Option<Arc<dyn Task>> {
        self.last_scheduled_tasks.back().and_then(|id| {
            // TODO straighten the `Path` story to understand what has to be passed here: or it will break in the future.
            self.list_tasks(None, false, cx)
                .into_iter()
                .find(|task| task.id() == id)
        })
    }

    pub fn task_scheduled(&mut self, id: TaskId) {
        self.last_scheduled_tasks.push_back(id);
        if self.last_scheduled_tasks.len() > 5_000 {
            self.last_scheduled_tasks.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo_kb() {
        todo!("TODO kb LRU tests")
    }
}
