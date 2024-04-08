//! Project-wide storage of the tasks available, capable of updating itself from the sources set.

use std::{
    any::TypeId,
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{HashMap, VecDeque};
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use itertools::Itertools;
use language::Language;
use task::{from_template_definition, ResolvedTask, Task, TaskId, TaskSource};
use util::{post_inc, NumericPrefixWithSuffix};
use worktree::WorktreeId;

/// Inventory tracks available tasks for a given project.
pub struct Inventory {
    sources: Vec<SourceInInventory>,
    last_scheduled_tasks: VecDeque<ResolvedTask>,
}

struct SourceInInventory {
    source: Model<Box<dyn TaskSource>>,
    _subscription: Subscription,
    type_id: TypeId,
    kind: TaskSourceKind,
}

/// Kind of a source the tasks are fetched from, used to display more source information in the UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskSourceKind {
    /// bash-like commands spawned by users, not associated with any path
    UserInput,
    /// ~/.config/zed/task.json - like global files with task definitions, applicable to any path
    AbsPath(PathBuf),
    /// Tasks from the worktree's .zed/task.json
    Worktree { id: WorktreeId, abs_path: PathBuf },
    /// Languages-specific tasks coming from extensions.
    Language { name: Arc<str> },
}

impl TaskSourceKind {
    fn abs_path(&self) -> Option<&Path> {
        match self {
            Self::AbsPath(abs_path) | Self::Worktree { abs_path, .. } => Some(abs_path),
            Self::UserInput | Self::Language { .. } => None,
        }
    }

    fn worktree(&self) -> Option<WorktreeId> {
        match self {
            Self::Worktree { id, .. } => Some(*id),
            _ => None,
        }
    }
}

impl Inventory {
    pub fn new(cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_| Self {
            sources: Vec::new(),
            last_scheduled_tasks: VecDeque::new(),
        })
    }

    /// If the task with the same path was not added yet,
    /// registers a new tasks source to fetch for available tasks later.
    /// Unless a source is removed, ignores future additions for the same path.
    pub fn add_source(
        &mut self,
        kind: TaskSourceKind,
        create_source: impl FnOnce(&mut ModelContext<Self>) -> Model<Box<dyn TaskSource>>,
        cx: &mut ModelContext<Self>,
    ) {
        let abs_path = kind.abs_path();
        if abs_path.is_some() {
            if let Some(a) = self.sources.iter().find(|s| s.kind.abs_path() == abs_path) {
                log::debug!("Source for path {abs_path:?} already exists, not adding. Old kind: {OLD_KIND:?}, new kind: {kind:?}", OLD_KIND = a.kind);
                return;
            }
        }

        let source = create_source(cx);
        let type_id = source.read(cx).type_id();
        let source = SourceInInventory {
            _subscription: cx.observe(&source, |_, _, cx| {
                cx.notify();
            }),
            source,
            type_id,
            kind,
        };
        self.sources.push(source);
        cx.notify();
    }

    /// If present, removes the local static source entry that has the given path,
    /// making corresponding task definitions unavailable in the fetch results.
    ///
    /// Now, entry for this path can be re-added again.
    pub fn remove_local_static_source(&mut self, abs_path: &Path) {
        self.sources.retain(|s| s.kind.abs_path() != Some(abs_path));
    }

    /// If present, removes the worktree source entry that has the given worktree id,
    /// making corresponding task definitions unavailable in the fetch results.
    ///
    /// Now, entry for this path can be re-added again.
    pub fn remove_worktree_sources(&mut self, worktree: WorktreeId) {
        self.sources.retain(|s| s.kind.worktree() != Some(worktree));
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

    /// Pulls its sources to list runnables for the editor given, or all runnables for no editor.
    pub fn list_tasks(
        &self,
        language: Option<Arc<Language>>,
        worktree: Option<WorktreeId>,
        lru: bool,
        cx: &mut AppContext,
    ) -> Vec<(TaskSourceKind, Arc<dyn Task>)> {
        let task_source_kind = language.as_ref().map(|language| TaskSourceKind::Language {
            name: language.name(),
        });
        let language_tasks = language
            .and_then(|language| {
                let tasks = language.context_provider()?.associated_tasks()?;
                Some((tasks, language))
            })
            .into_iter()
            .flat_map(|(tasks, language)| {
                tasks.0.into_iter().map(move |definition| {
                    from_template_definition(
                        TaskId(format!("language_{}", language.name())),
                        definition,
                    )
                })
            })
            .flat_map(|task| Some((task_source_kind.as_ref()?, task as Arc<dyn Task>)));

        let mut lru_score = 0_u32;
        let tasks_by_usage = if lru {
            self.last_scheduled_tasks.iter().rev().fold(
                HashMap::default(),
                |mut tasks, resolved_task| {
                    tasks
                        .entry(resolved_task.id().clone())
                        .or_insert_with(|| post_inc(&mut lru_score));
                    tasks
                },
            )
        } else {
            HashMap::default()
        };
        let not_used_score = post_inc(&mut lru_score);
        self.sources
            .iter()
            .filter(|source| {
                let source_worktree = source.kind.worktree();
                worktree.is_none() || source_worktree.is_none() || source_worktree == worktree
            })
            .flat_map(|source| {
                source
                    .source
                    .update(cx, |source, cx| source.tasks_to_schedule(cx))
                    .into_iter()
                    .map(|task| (&source.kind, task))
            })
            .chain(language_tasks)
            .map(|task| {
                let usages = if lru {
                    tasks_by_usage
                        .get(&task.1.id())
                        .copied()
                        .unwrap_or(not_used_score)
                } else {
                    not_used_score
                };
                (task, usages)
            })
            .sorted_unstable_by(
                |((kind_a, task_a), usages_a), ((kind_b, task_b), usages_b)| {
                    usages_a
                        .cmp(&usages_b)
                        .then(
                            kind_a
                                .worktree()
                                .is_none()
                                .cmp(&kind_b.worktree().is_none()),
                        )
                        .then(kind_a.worktree().cmp(&kind_b.worktree()))
                        .then(
                            kind_a
                                .abs_path()
                                .is_none()
                                .cmp(&kind_b.abs_path().is_none()),
                        )
                        .then(kind_a.abs_path().cmp(&kind_b.abs_path()))
                        .then({
                            NumericPrefixWithSuffix::from_numeric_prefixed_str(task_a.name())
                                .cmp(&NumericPrefixWithSuffix::from_numeric_prefixed_str(
                                    task_b.name(),
                                ))
                                .then(task_a.name().cmp(task_b.name()))
                        })
                },
            )
            .map(|((kind, task), _)| (kind.clone(), task))
            .collect()
    }

    /// Returns the last scheduled task, if any of the sources contains one with the matching id.
    pub fn last_scheduled_task(&self) -> Option<ResolvedTask> {
        self.last_scheduled_tasks.back().cloned()
    }

    /// Registers task "usage" as being scheduled – to be used for LRU sorting when listing all tasks.
    pub fn task_scheduled(&mut self, resolved_task: ResolvedTask) {
        self.last_scheduled_tasks.push_back(resolved_task);
        if self.last_scheduled_tasks.len() > 5_000 {
            self.last_scheduled_tasks.pop_front();
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test_inventory {
    use std::sync::Arc;

    use gpui::{AppContext, Context as _, Model, ModelContext, TestAppContext};
    use task::{ResolvedTask, Task, TaskContext, TaskId, TaskSource};
    use worktree::WorktreeId;

    use crate::Inventory;

    use super::TaskSourceKind;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TestTask {
        id: task::TaskId,
        name: String,
    }

    impl Task for TestTask {
        fn id(&self) -> &TaskId {
            &self.id
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn cwd(&self) -> Option<&str> {
            None
        }

        fn resolve_task(&self, _: TaskContext) -> Option<ResolvedTask> {
            Some(ResolvedTask::Noop(Arc::new(self.clone())))
        }
    }

    pub struct StaticTestSource {
        pub tasks: Vec<TestTask>,
    }

    impl StaticTestSource {
        pub fn new(
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

    impl TaskSource for StaticTestSource {
        fn tasks_to_schedule(
            &mut self,
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

    pub fn list_task_names(
        inventory: &Model<Inventory>,
        worktree: Option<WorktreeId>,
        lru: bool,
        cx: &mut TestAppContext,
    ) -> Vec<String> {
        inventory.update(cx, |inventory, cx| {
            inventory
                .list_tasks(None, worktree, lru, cx)
                .into_iter()
                .map(|(_, task)| task.name().to_string())
                .collect()
        })
    }

    pub fn register_task_used(
        inventory: &Model<Inventory>,
        task_name: &str,
        cx: &mut TestAppContext,
    ) {
        inventory.update(cx, |inventory, cx| {
            let (_, task) = inventory
                .list_tasks(None, None, false, cx)
                .into_iter()
                .find(|(_, task)| task.name() == task_name)
                .unwrap_or_else(|| panic!("Failed to find task with name {task_name}"));
            inventory.task_scheduled(
                task.resolve_task(TaskContext::default())
                    .unwrap_or_else(|| panic!("Failed to resolve task with name {task_name}")),
            );
        });
    }

    pub fn list_tasks(
        inventory: &Model<Inventory>,
        worktree: Option<WorktreeId>,
        lru: bool,
        cx: &mut TestAppContext,
    ) -> Vec<(TaskSourceKind, String)> {
        inventory.update(cx, |inventory, cx| {
            inventory
                .list_tasks(None, worktree, lru, cx)
                .into_iter()
                .map(|(source_kind, task)| (source_kind, task.name().to_string()))
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;

    use super::test_inventory::*;
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
            inventory.add_source(
                TaskSourceKind::UserInput,
                |cx| StaticTestSource::new(vec!["3_task".to_string()], cx),
                cx,
            );
        });
        inventory.update(cx, |inventory, cx| {
            inventory.add_source(
                TaskSourceKind::UserInput,
                |cx| {
                    StaticTestSource::new(
                        vec![
                            "1_task".to_string(),
                            "2_task".to_string(),
                            "1_a_task".to_string(),
                        ],
                        cx,
                    )
                },
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
                TaskSourceKind::UserInput,
                |cx| {
                    StaticTestSource::new(vec!["10_hello".to_string(), "11_hello".to_string()], cx)
                },
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

    #[gpui::test]
    fn test_inventory_static_task_filters(cx: &mut TestAppContext) {
        let inventory_with_statics = cx.update(Inventory::new);
        let common_name = "common_task_name";
        let path_1 = Path::new("path_1");
        let path_2 = Path::new("path_2");
        let worktree_1 = WorktreeId::from_usize(1);
        let worktree_path_1 = Path::new("worktree_path_1");
        let worktree_2 = WorktreeId::from_usize(2);
        let worktree_path_2 = Path::new("worktree_path_2");
        inventory_with_statics.update(cx, |inventory, cx| {
            inventory.add_source(
                TaskSourceKind::UserInput,
                |cx| {
                    StaticTestSource::new(
                        vec!["user_input".to_string(), common_name.to_string()],
                        cx,
                    )
                },
                cx,
            );
            inventory.add_source(
                TaskSourceKind::AbsPath(path_1.to_path_buf()),
                |cx| {
                    StaticTestSource::new(
                        vec!["static_source_1".to_string(), common_name.to_string()],
                        cx,
                    )
                },
                cx,
            );
            inventory.add_source(
                TaskSourceKind::AbsPath(path_2.to_path_buf()),
                |cx| {
                    StaticTestSource::new(
                        vec!["static_source_2".to_string(), common_name.to_string()],
                        cx,
                    )
                },
                cx,
            );
            inventory.add_source(
                TaskSourceKind::Worktree {
                    id: worktree_1,
                    abs_path: worktree_path_1.to_path_buf(),
                },
                |cx| {
                    StaticTestSource::new(
                        vec!["worktree_1".to_string(), common_name.to_string()],
                        cx,
                    )
                },
                cx,
            );
            inventory.add_source(
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    abs_path: worktree_path_2.to_path_buf(),
                },
                |cx| {
                    StaticTestSource::new(
                        vec!["worktree_2".to_string(), common_name.to_string()],
                        cx,
                    )
                },
                cx,
            );
        });

        let worktree_independent_tasks = vec![
            (
                TaskSourceKind::AbsPath(path_1.to_path_buf()),
                common_name.to_string(),
            ),
            (
                TaskSourceKind::AbsPath(path_1.to_path_buf()),
                "static_source_1".to_string(),
            ),
            (
                TaskSourceKind::AbsPath(path_2.to_path_buf()),
                common_name.to_string(),
            ),
            (
                TaskSourceKind::AbsPath(path_2.to_path_buf()),
                "static_source_2".to_string(),
            ),
            (TaskSourceKind::UserInput, common_name.to_string()),
            (TaskSourceKind::UserInput, "user_input".to_string()),
        ];
        let worktree_1_tasks = [
            (
                TaskSourceKind::Worktree {
                    id: worktree_1,
                    abs_path: worktree_path_1.to_path_buf(),
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_1,
                    abs_path: worktree_path_1.to_path_buf(),
                },
                "worktree_1".to_string(),
            ),
        ];
        let worktree_2_tasks = [
            (
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    abs_path: worktree_path_2.to_path_buf(),
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    abs_path: worktree_path_2.to_path_buf(),
                },
                "worktree_2".to_string(),
            ),
        ];

        let all_tasks = worktree_1_tasks
            .iter()
            .chain(worktree_2_tasks.iter())
            // worktree-less tasks come later in the list
            .chain(worktree_independent_tasks.iter())
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            list_tasks(&inventory_with_statics, None, false, cx),
            all_tasks,
        );
        assert_eq!(
            list_tasks(&inventory_with_statics, Some(worktree_1), false, cx),
            worktree_1_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            list_tasks(&inventory_with_statics, Some(worktree_2), false, cx),
            worktree_2_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
}
