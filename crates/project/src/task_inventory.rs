//! Project-wide storage of the tasks available, capable of updating itself from the sources set.

use std::{any::TypeId, path::Path, sync::Arc};

use collections::{HashMap, VecDeque};
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use itertools::Itertools;
use task::{Task, TaskId, TaskSource};
use util::{post_inc, NumericPrefixWithSuffix};

/// Inventory tracks available tasks for a given project.
pub struct Inventory {
    sources: Vec<SourceInInventory>,
    last_scheduled_tasks: VecDeque<TaskId>,
}

struct SourceInInventory {
    source: Model<Box<dyn TaskSource>>,
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
    pub fn add_source(&mut self, source: Model<Box<dyn TaskSource>>, cx: &mut ModelContext<Self>) {
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

    pub fn source<T: TaskSource>(&self) -> Option<Model<Box<dyn TaskSource>>> {
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
        let not_used_score = post_inc(&mut lru_score);

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
                        .unwrap_or(not_used_score)
                } else {
                    not_used_score
                };
                (task, usages)
            })
            .sorted_unstable_by(|(task_a, usages_a), (task_b, usages_b)| {
                usages_a.cmp(usages_b).then({
                    NumericPrefixWithSuffix::from_numeric_prefixed_str(task_a.name())
                        .cmp(&NumericPrefixWithSuffix::from_numeric_prefixed_str(
                            task_b.name(),
                        ))
                        .then(task_a.name().cmp(task_b.name()))
                })
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

    /// Registers task "usage" as being scheduled – to be used for LRU sorting when listing all tasks.
    pub fn task_scheduled(&mut self, id: TaskId) {
        self.last_scheduled_tasks.push_back(id);
        if self.last_scheduled_tasks.len() > 5_000 {
            self.last_scheduled_tasks.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use gpui::TestAppContext;

    use super::*;

    #[gpui::test]
    fn test_task_list_sorting(cx: &mut TestAppContext) {
        let inventory = cx.update(Inventory::new);
        let initial_tasks = list_task_names(&inventory, None, true, cx);
        assert!(
            initial_tasks.is_empty(),
            "No tasks expected for empty inventory, but got {initial_tasks:?}"
        );
        let initial_tasks = list_task_names(&inventory, None, false, cx);
        assert!(
            initial_tasks.is_empty(),
            "No tasks expected for empty inventory, but got {initial_tasks:?}"
        );

        inventory.update(cx, |inventory, cx| {
            inventory.add_source(TestSource::new(vec!["3_task".to_string()], cx), cx);
        });
        inventory.update(cx, |inventory, cx| {
            inventory.add_source(
                TestSource::new(
                    vec![
                        "1_task".to_string(),
                        "2_task".to_string(),
                        "1_a_task".to_string(),
                    ],
                    cx,
                ),
                cx,
            );
        });

        let expected_initial_state = [
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
        ];
        assert_eq!(
            list_task_names(&inventory, None, false, cx),
            &expected_initial_state,
            "Task list without lru sorting, should be sorted alphanumerically"
        );
        assert_eq!(
            list_task_names(&inventory, None, true, cx),
            &expected_initial_state,
            "Tasks with equal amount of usages should be sorted alphanumerically"
        );

        register_task_used(&inventory, "2_task", cx);
        assert_eq!(
            list_task_names(&inventory, None, false, cx),
            &expected_initial_state,
            "Task list without lru sorting, should be sorted alphanumerically"
        );
        assert_eq!(
            list_task_names(&inventory, None, true, cx),
            vec![
                "2_task".to_string(),
                "1_a_task".to_string(),
                "1_task".to_string(),
                "3_task".to_string()
            ],
        );

        register_task_used(&inventory, "1_task", cx);
        register_task_used(&inventory, "1_task", cx);
        register_task_used(&inventory, "1_task", cx);
        register_task_used(&inventory, "3_task", cx);
        assert_eq!(
            list_task_names(&inventory, None, false, cx),
            &expected_initial_state,
            "Task list without lru sorting, should be sorted alphanumerically"
        );
        assert_eq!(
            list_task_names(&inventory, None, true, cx),
            vec![
                "3_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "1_a_task".to_string(),
            ],
        );

        inventory.update(cx, |inventory, cx| {
            inventory.add_source(
                TestSource::new(vec!["10_hello".to_string(), "11_hello".to_string()], cx),
                cx,
            );
        });
        let expected_updated_state = [
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
            "10_hello".to_string(),
            "11_hello".to_string(),
        ];
        assert_eq!(
            list_task_names(&inventory, None, false, cx),
            &expected_updated_state,
            "Task list without lru sorting, should be sorted alphanumerically"
        );
        assert_eq!(
            list_task_names(&inventory, None, true, cx),
            vec![
                "3_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "1_a_task".to_string(),
                "10_hello".to_string(),
                "11_hello".to_string(),
            ],
        );

        register_task_used(&inventory, "11_hello", cx);
        assert_eq!(
            list_task_names(&inventory, None, false, cx),
            &expected_updated_state,
            "Task list without lru sorting, should be sorted alphanumerically"
        );
        assert_eq!(
            list_task_names(&inventory, None, true, cx),
            vec![
                "11_hello".to_string(),
                "3_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "1_a_task".to_string(),
                "10_hello".to_string(),
            ],
        );
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestTask {
        id: TaskId,
        name: String,
    }

    impl Task for TestTask {
        fn id(&self) -> &TaskId {
            &self.id
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn cwd(&self) -> Option<&Path> {
            None
        }

        fn exec(&self, _cwd: Option<PathBuf>) -> Option<task::SpawnInTerminal> {
            None
        }
    }

    struct TestSource {
        tasks: Vec<TestTask>,
    }

    impl TestSource {
        fn new(
            task_names: impl IntoIterator<Item = String>,
            cx: &mut AppContext,
        ) -> Model<Box<dyn TaskSource>> {
            cx.new_model(|_| {
                Box::new(Self {
                    tasks: task_names
                        .into_iter()
                        .enumerate()
                        .map(|(i, name)| TestTask {
                            id: TaskId(format!("task_{i}_{name}")),
                            name,
                        })
                        .collect(),
                }) as Box<dyn TaskSource>
            })
        }
    }

    impl TaskSource for TestSource {
        fn tasks_for_path(
            &mut self,
            _path: Option<&Path>,
            _cx: &mut ModelContext<Box<dyn TaskSource>>,
        ) -> Vec<Arc<dyn Task>> {
            self.tasks
                .clone()
                .into_iter()
                .map(|task| Arc::new(task) as Arc<dyn Task>)
                .collect()
        }

        fn as_any(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    fn list_task_names(
        inventory: &Model<Inventory>,
        path: Option<&Path>,
        lru: bool,
        cx: &mut TestAppContext,
    ) -> Vec<String> {
        inventory.update(cx, |inventory, cx| {
            inventory
                .list_tasks(path, lru, cx)
                .into_iter()
                .map(|task| task.name().to_string())
                .collect()
        })
    }

    fn register_task_used(inventory: &Model<Inventory>, task_name: &str, cx: &mut TestAppContext) {
        inventory.update(cx, |inventory, cx| {
            let task = inventory
                .list_tasks(None, false, cx)
                .into_iter()
                .find(|task| task.name() == task_name)
                .unwrap_or_else(|| panic!("Failed to find task with name {task_name}"));
            inventory.task_scheduled(task.id().clone());
        });
    }
}
