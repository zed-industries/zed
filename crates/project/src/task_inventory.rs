//! Project-wide storage of the tasks available, capable of updating itself from the sources set.

use std::{
    cmp::{self, Reverse},
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{btree_map, BTreeMap, VecDeque};
use gpui::{AppContext, Context, Model, ModelContext};
use itertools::{Either, Itertools};
use language::Language;
use task::{
    static_source::StaticSource, ResolvedTask, TaskContext, TaskId, TaskTemplate, VariableName,
};
use util::{post_inc, NumericPrefixWithSuffix};
use worktree::WorktreeId;

/// Inventory tracks available tasks for a given project.
pub struct Inventory {
    sources: Vec<SourceInInventory>,
    last_scheduled_tasks: VecDeque<(TaskSourceKind, ResolvedTask)>,
}

struct SourceInInventory {
    source: StaticSource,
    kind: TaskSourceKind,
}

/// Kind of a source the tasks are fetched from, used to display more source information in the UI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TaskSourceKind {
    /// bash-like commands spawned by users, not associated with any path
    UserInput,
    /// Tasks from the worktree's .zed/task.json
    Worktree {
        id: WorktreeId,
        abs_path: PathBuf,
        id_base: &'static str,
    },
    /// ~/.config/zed/task.json - like global files with task definitions, applicable to any path
    AbsPath {
        id_base: &'static str,
        abs_path: PathBuf,
    },
    /// Languages-specific tasks coming from extensions.
    Language { name: Arc<str> },
}

impl TaskSourceKind {
    pub fn abs_path(&self) -> Option<&Path> {
        match self {
            Self::AbsPath { abs_path, .. } | Self::Worktree { abs_path, .. } => Some(abs_path),
            Self::UserInput | Self::Language { .. } => None,
        }
    }

    pub fn worktree(&self) -> Option<WorktreeId> {
        match self {
            Self::Worktree { id, .. } => Some(*id),
            _ => None,
        }
    }

    pub fn to_id_base(&self) -> String {
        match self {
            TaskSourceKind::UserInput => "oneshot".to_string(),
            TaskSourceKind::AbsPath { id_base, abs_path } => {
                format!("{id_base}_{}", abs_path.display())
            }
            TaskSourceKind::Worktree {
                id,
                id_base,
                abs_path,
            } => {
                format!("{id_base}_{id}_{}", abs_path.display())
            }
            TaskSourceKind::Language { name } => format!("language_{name}"),
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
        source: StaticSource,
        cx: &mut ModelContext<Self>,
    ) {
        let abs_path = kind.abs_path();
        if abs_path.is_some() {
            if let Some(a) = self.sources.iter().find(|s| s.kind.abs_path() == abs_path) {
                log::debug!("Source for path {abs_path:?} already exists, not adding. Old kind: {OLD_KIND:?}, new kind: {kind:?}", OLD_KIND = a.kind);
                return;
            }
        }

        let source = SourceInInventory { source, kind };
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

    /// Pulls its task sources relevant to the worktree and the language given,
    /// returns all task templates with their source kinds, in no specific order.
    pub fn list_tasks(
        &self,
        language: Option<Arc<Language>>,
        worktree: Option<WorktreeId>,
    ) -> Vec<(TaskSourceKind, TaskTemplate)> {
        let task_source_kind = language.as_ref().map(|language| TaskSourceKind::Language {
            name: language.name(),
        });
        let language_tasks = language
            .and_then(|language| language.context_provider()?.associated_tasks())
            .into_iter()
            .flat_map(|tasks| tasks.0.into_iter())
            .flat_map(|task| Some((task_source_kind.as_ref()?, task)));

        self.sources
            .iter()
            .filter(|source| {
                let source_worktree = source.kind.worktree();
                worktree.is_none() || source_worktree.is_none() || source_worktree == worktree
            })
            .flat_map(|source| {
                source
                    .source
                    .tasks_to_schedule()
                    .0
                    .into_iter()
                    .map(|task| (&source.kind, task))
            })
            .chain(language_tasks)
            .map(|(task_source_kind, task)| (task_source_kind.clone(), task))
            .collect()
    }

    /// Pulls its task sources relevant to the worktree and the language given and resolves them with the [`TaskContext`] given.
    /// Joins the new resolutions with the resolved tasks that were used (spawned) before,
    /// orders them so that the most recently used come first, all equally used ones are ordered so that the most specific tasks come first.
    /// Deduplicates the tasks by their labels and splits the ordered list into two: used tasks and the rest, newly resolved tasks.
    pub fn used_and_current_resolved_tasks(
        &self,
        language: Option<Arc<Language>>,
        worktree: Option<WorktreeId>,
        task_context: &TaskContext,
    ) -> (
        Vec<(TaskSourceKind, ResolvedTask)>,
        Vec<(TaskSourceKind, ResolvedTask)>,
    ) {
        let task_source_kind = language.as_ref().map(|language| TaskSourceKind::Language {
            name: language.name(),
        });
        let language_tasks = language
            .and_then(|language| language.context_provider()?.associated_tasks())
            .into_iter()
            .flat_map(|tasks| tasks.0.into_iter())
            .flat_map(|task| Some((task_source_kind.as_ref()?, task)));

        let mut lru_score = 0_u32;
        let mut task_usage = self
            .last_scheduled_tasks
            .iter()
            .rev()
            .filter(|(_, task)| !task.original_task().ignore_previously_resolved)
            .filter(|(task_kind, _)| {
                if matches!(task_kind, TaskSourceKind::Language { .. }) {
                    Some(task_kind) == task_source_kind.as_ref()
                } else {
                    true
                }
            })
            .fold(
                BTreeMap::default(),
                |mut tasks, (task_source_kind, resolved_task)| {
                    tasks.entry(&resolved_task.id).or_insert_with(|| {
                        (task_source_kind, resolved_task, post_inc(&mut lru_score))
                    });
                    tasks
                },
            );
        let not_used_score = post_inc(&mut lru_score);
        let currently_resolved_tasks = self
            .sources
            .iter()
            .filter(|source| {
                let source_worktree = source.kind.worktree();
                worktree.is_none() || source_worktree.is_none() || source_worktree == worktree
            })
            .flat_map(|source| {
                source
                    .source
                    .tasks_to_schedule()
                    .0
                    .into_iter()
                    .map(|task| (&source.kind, task))
            })
            .chain(language_tasks)
            .filter_map(|(kind, task)| {
                let id_base = kind.to_id_base();
                Some((kind, task.resolve_task(&id_base, task_context)?))
            })
            .map(|(kind, task)| {
                let lru_score = task_usage
                    .remove(&task.id)
                    .map(|(_, _, lru_score)| lru_score)
                    .unwrap_or(not_used_score);
                (kind.clone(), task, lru_score)
            })
            .collect::<Vec<_>>();
        let previously_spawned_tasks = task_usage
            .into_iter()
            .map(|(_, (kind, task, lru_score))| (kind.clone(), task.clone(), lru_score));

        let mut tasks_by_label = BTreeMap::default();
        tasks_by_label = previously_spawned_tasks.into_iter().fold(
            tasks_by_label,
            |mut tasks_by_label, (source, task, lru_score)| {
                match tasks_by_label.entry((source, task.resolved_label.clone())) {
                    btree_map::Entry::Occupied(mut o) => {
                        let (_, previous_lru_score) = o.get();
                        if previous_lru_score >= &lru_score {
                            o.insert((task, lru_score));
                        }
                    }
                    btree_map::Entry::Vacant(v) => {
                        v.insert((task, lru_score));
                    }
                }
                tasks_by_label
            },
        );
        tasks_by_label = currently_resolved_tasks.into_iter().fold(
            tasks_by_label,
            |mut tasks_by_label, (source, task, lru_score)| {
                match tasks_by_label.entry((source, task.resolved_label.clone())) {
                    btree_map::Entry::Occupied(mut o) => {
                        let (previous_task, _) = o.get();
                        let new_template = task.original_task();
                        if new_template.ignore_previously_resolved
                            || new_template != previous_task.original_task()
                        {
                            o.insert((task, lru_score));
                        }
                    }
                    btree_map::Entry::Vacant(v) => {
                        v.insert((task, lru_score));
                    }
                }
                tasks_by_label
            },
        );

        tasks_by_label
            .into_iter()
            .map(|((kind, _), (task, lru_score))| (kind, task, lru_score))
            .sorted_unstable_by(task_lru_comparator)
            .partition_map(|(kind, task, lru_score)| {
                if lru_score < not_used_score {
                    Either::Left((kind, task))
                } else {
                    Either::Right((kind, task))
                }
            })
    }

    /// Returns the last scheduled task, if any of the sources contains one with the matching id.
    pub fn last_scheduled_task(&self) -> Option<(TaskSourceKind, ResolvedTask)> {
        self.last_scheduled_tasks.back().cloned()
    }

    /// Registers task "usage" as being scheduled – to be used for LRU sorting when listing all tasks.
    pub fn task_scheduled(
        &mut self,
        task_source_kind: TaskSourceKind,
        resolved_task: ResolvedTask,
    ) {
        self.last_scheduled_tasks
            .push_back((task_source_kind, resolved_task));
        if self.last_scheduled_tasks.len() > 5_000 {
            self.last_scheduled_tasks.pop_front();
        }
    }

    /// Deletes a resolved task from history, using its id.
    /// A similar may still resurface in `used_and_current_resolved_tasks` when its [`TaskTemplate`] is resolved again.
    pub fn delete_previously_used(&mut self, id: &TaskId) {
        self.last_scheduled_tasks.retain(|(_, task)| &task.id != id);
    }
}

fn task_lru_comparator(
    (kind_a, task_a, lru_score_a): &(TaskSourceKind, ResolvedTask, u32),
    (kind_b, task_b, lru_score_b): &(TaskSourceKind, ResolvedTask, u32),
) -> cmp::Ordering {
    lru_score_a
        // First, display recently used templates above all.
        .cmp(&lru_score_b)
        // Then, ensure more specific sources are displayed first.
        .then(task_source_kind_preference(kind_a).cmp(&task_source_kind_preference(kind_b)))
        // After that, display first more specific tasks, using more template variables.
        // Bonus points for tasks with symbol variables.
        .then(task_variables_preference(task_a).cmp(&task_variables_preference(task_b)))
        // Finally, sort by the resolved label, but a bit more specifically, to avoid mixing letters and digits.
        .then({
            NumericPrefixWithSuffix::from_numeric_prefixed_str(&task_a.resolved_label)
                .cmp(&NumericPrefixWithSuffix::from_numeric_prefixed_str(
                    &task_b.resolved_label,
                ))
                .then(task_a.resolved_label.cmp(&task_b.resolved_label))
        })
}

fn task_source_kind_preference(kind: &TaskSourceKind) -> u32 {
    match kind {
        TaskSourceKind::Language { .. } => 1,
        TaskSourceKind::UserInput => 2,
        TaskSourceKind::Worktree { .. } => 3,
        TaskSourceKind::AbsPath { .. } => 4,
    }
}

fn task_variables_preference(task: &ResolvedTask) -> Reverse<usize> {
    let task_variables = task.substituted_variables();
    Reverse(if task_variables.contains(&VariableName::Symbol) {
        task_variables.len() + 1
    } else {
        task_variables.len()
    })
}

#[cfg(test)]
mod test_inventory {
    use gpui::{AppContext, Model, TestAppContext};
    use itertools::Itertools;
    use task::{
        static_source::{StaticSource, TrackedFile},
        TaskContext, TaskTemplate, TaskTemplates,
    };
    use worktree::WorktreeId;

    use crate::Inventory;

    use super::{task_source_kind_preference, TaskSourceKind};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TestTask {
        name: String,
    }

    pub(super) fn static_test_source(
        task_names: impl IntoIterator<Item = String>,
        cx: &mut AppContext,
    ) -> StaticSource {
        let tasks = TaskTemplates(
            task_names
                .into_iter()
                .map(|name| TaskTemplate {
                    label: name,
                    command: "test command".to_owned(),
                    ..TaskTemplate::default()
                })
                .collect(),
        );
        let (tx, rx) = futures::channel::mpsc::unbounded();
        let file = TrackedFile::new(rx, cx);
        tx.unbounded_send(serde_json::to_string(&tasks).unwrap())
            .unwrap();
        StaticSource::new(file)
    }

    pub(super) fn task_template_names(
        inventory: &Model<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Vec<String> {
        inventory.update(cx, |inventory, _| {
            inventory
                .list_tasks(None, worktree)
                .into_iter()
                .map(|(_, task)| task.label)
                .sorted()
                .collect()
        })
    }

    pub(super) fn resolved_task_names(
        inventory: &Model<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Vec<String> {
        inventory.update(cx, |inventory, _| {
            let (used, current) =
                inventory.used_and_current_resolved_tasks(None, worktree, &TaskContext::default());
            used.into_iter()
                .chain(current)
                .map(|(_, task)| task.original_task().label.clone())
                .collect()
        })
    }

    pub(super) fn register_task_used(
        inventory: &Model<Inventory>,
        task_name: &str,
        cx: &mut TestAppContext,
    ) {
        inventory.update(cx, |inventory, _| {
            let (task_source_kind, task) = inventory
                .list_tasks(None, None)
                .into_iter()
                .find(|(_, task)| task.label == task_name)
                .unwrap_or_else(|| panic!("Failed to find task with name {task_name}"));
            let id_base = task_source_kind.to_id_base();
            inventory.task_scheduled(
                task_source_kind.clone(),
                task.resolve_task(&id_base, &TaskContext::default())
                    .unwrap_or_else(|| panic!("Failed to resolve task with name {task_name}")),
            );
        });
    }

    pub(super) fn list_tasks(
        inventory: &Model<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Vec<(TaskSourceKind, String)> {
        inventory.update(cx, |inventory, _| {
            let (used, current) =
                inventory.used_and_current_resolved_tasks(None, worktree, &TaskContext::default());
            let mut all = used;
            all.extend(current);
            all.into_iter()
                .map(|(source_kind, task)| (source_kind, task.resolved_label))
                .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
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
        let initial_tasks = resolved_task_names(&inventory, None, cx);
        assert!(
            initial_tasks.is_empty(),
            "No tasks expected for empty inventory, but got {initial_tasks:?}"
        );
        let initial_tasks = task_template_names(&inventory, None, cx);
        assert!(
            initial_tasks.is_empty(),
            "No tasks expected for empty inventory, but got {initial_tasks:?}"
        );

        inventory.update(cx, |inventory, cx| {
            inventory.add_source(
                TaskSourceKind::UserInput,
                static_test_source(vec!["3_task".to_string()], cx),
                cx,
            );
        });
        inventory.update(cx, |inventory, cx| {
            inventory.add_source(
                TaskSourceKind::UserInput,
                static_test_source(
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
        cx.run_until_parked();
        let expected_initial_state = [
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
        ];
        assert_eq!(
            task_template_names(&inventory, None, cx),
            &expected_initial_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx),
            &expected_initial_state,
            "Tasks with equal amount of usages should be sorted alphanumerically"
        );

        register_task_used(&inventory, "2_task", cx);
        assert_eq!(
            task_template_names(&inventory, None, cx),
            &expected_initial_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx),
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
            task_template_names(&inventory, None, cx),
            &expected_initial_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx),
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
                static_test_source(vec!["10_hello".to_string(), "11_hello".to_string()], cx),
                cx,
            );
        });
        cx.run_until_parked();
        let expected_updated_state = [
            "10_hello".to_string(),
            "11_hello".to_string(),
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
        ];
        assert_eq!(
            task_template_names(&inventory, None, cx),
            &expected_updated_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx),
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
            task_template_names(&inventory, None, cx),
            &expected_updated_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx),
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
                static_test_source(vec!["user_input".to_string(), common_name.to_string()], cx),
                cx,
            );
            inventory.add_source(
                TaskSourceKind::AbsPath {
                    id_base: "test source",
                    abs_path: path_1.to_path_buf(),
                },
                static_test_source(
                    vec!["static_source_1".to_string(), common_name.to_string()],
                    cx,
                ),
                cx,
            );
            inventory.add_source(
                TaskSourceKind::AbsPath {
                    id_base: "test source",
                    abs_path: path_2.to_path_buf(),
                },
                static_test_source(
                    vec!["static_source_2".to_string(), common_name.to_string()],
                    cx,
                ),
                cx,
            );
            inventory.add_source(
                TaskSourceKind::Worktree {
                    id: worktree_1,
                    abs_path: worktree_path_1.to_path_buf(),
                    id_base: "test_source",
                },
                static_test_source(vec!["worktree_1".to_string(), common_name.to_string()], cx),
                cx,
            );
            inventory.add_source(
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    abs_path: worktree_path_2.to_path_buf(),
                    id_base: "test_source",
                },
                static_test_source(vec!["worktree_2".to_string(), common_name.to_string()], cx),
                cx,
            );
        });
        cx.run_until_parked();
        let worktree_independent_tasks = vec![
            (
                TaskSourceKind::AbsPath {
                    id_base: "test source",
                    abs_path: path_1.to_path_buf(),
                },
                "static_source_1".to_string(),
            ),
            (
                TaskSourceKind::AbsPath {
                    id_base: "test source",
                    abs_path: path_1.to_path_buf(),
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::AbsPath {
                    id_base: "test source",
                    abs_path: path_2.to_path_buf(),
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::AbsPath {
                    id_base: "test source",
                    abs_path: path_2.to_path_buf(),
                },
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
                    id_base: "test_source",
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_1,
                    abs_path: worktree_path_1.to_path_buf(),
                    id_base: "test_source",
                },
                "worktree_1".to_string(),
            ),
        ];
        let worktree_2_tasks = [
            (
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    abs_path: worktree_path_2.to_path_buf(),
                    id_base: "test_source",
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    abs_path: worktree_path_2.to_path_buf(),
                    id_base: "test_source",
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
            .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
            .collect::<Vec<_>>();

        assert_eq!(list_tasks(&inventory_with_statics, None, cx), all_tasks);
        assert_eq!(
            list_tasks(&inventory_with_statics, Some(worktree_1), cx),
            worktree_1_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            list_tasks(&inventory_with_statics, Some(worktree_2), cx),
            worktree_2_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
                .collect::<Vec<_>>(),
        );
    }
}
