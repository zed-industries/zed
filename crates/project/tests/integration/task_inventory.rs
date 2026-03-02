use gpui::{AppContext, Entity, Task, TestAppContext};
use itertools::Itertools;
use paths::tasks_file;
use pretty_assertions::assert_eq;
use serde_json::json;
use settings::SettingsLocation;
use std::path::Path;
use std::sync::Arc;
use util::rel_path::rel_path;

use project::task_store::{TaskSettingsLocation, TaskStore};

use project::{WorktreeId, task_inventory::*};
use test_inventory::*;

mod test_inventory {
    use gpui::{AppContext as _, Entity, Task, TestAppContext};
    use itertools::Itertools;
    use task::TaskContext;
    use worktree::WorktreeId;

    use crate::Inventory;

    use super::TaskSourceKind;

    pub(super) fn task_template_names(
        inventory: &Entity<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Task<Vec<String>> {
        let new_tasks = inventory.update(cx, |inventory, cx| {
            inventory.list_tasks(None, None, worktree, cx)
        });
        cx.background_spawn(async move {
            new_tasks
                .await
                .into_iter()
                .map(|(_, task)| task.label)
                .sorted()
                .collect()
        })
    }

    pub(super) fn register_task_used(
        inventory: &Entity<Inventory>,
        task_name: &str,
        cx: &mut TestAppContext,
    ) -> Task<()> {
        let tasks = inventory.update(cx, |inventory, cx| {
            inventory.list_tasks(None, None, None, cx)
        });

        let task_name = task_name.to_owned();
        let inventory = inventory.clone();
        cx.spawn(|mut cx| async move {
            let (task_source_kind, task) = tasks
                .await
                .into_iter()
                .find(|(_, task)| task.label == task_name)
                .unwrap_or_else(|| panic!("Failed to find task with name {task_name}"));

            let id_base = task_source_kind.to_id_base();
            inventory.update(&mut cx, |inventory, _| {
                inventory.task_scheduled(
                    task_source_kind.clone(),
                    task.resolve_task(&id_base, &TaskContext::default())
                        .unwrap_or_else(|| panic!("Failed to resolve task with name {task_name}")),
                )
            });
        })
    }

    pub(super) fn register_worktree_task_used(
        inventory: &Entity<Inventory>,
        worktree_id: WorktreeId,
        task_name: &str,
        cx: &mut TestAppContext,
    ) -> Task<()> {
        let tasks = inventory.update(cx, |inventory, cx| {
            inventory.list_tasks(None, None, Some(worktree_id), cx)
        });

        let inventory = inventory.clone();
        let task_name = task_name.to_owned();
        cx.spawn(|mut cx| async move {
            let (task_source_kind, task) = tasks
                .await
                .into_iter()
                .find(|(_, task)| task.label == task_name)
                .unwrap_or_else(|| panic!("Failed to find task with name {task_name}"));
            let id_base = task_source_kind.to_id_base();
            inventory.update(&mut cx, |inventory, _| {
                inventory.task_scheduled(
                    task_source_kind.clone(),
                    task.resolve_task(&id_base, &TaskContext::default())
                        .unwrap_or_else(|| panic!("Failed to resolve task with name {task_name}")),
                );
            });
        })
    }

    pub(super) async fn list_tasks(
        inventory: &Entity<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Vec<(TaskSourceKind, String)> {
        let task_context = &TaskContext::default();
        inventory
            .update(cx, |inventory, cx| {
                inventory.list_tasks(None, None, worktree, cx)
            })
            .await
            .into_iter()
            .filter_map(|(source_kind, task)| {
                let id_base = source_kind.to_id_base();
                Some((source_kind, task.resolve_task(&id_base, task_context)?))
            })
            .map(|(source_kind, resolved_task)| (source_kind, resolved_task.resolved_label))
            .collect()
    }
}

#[gpui::test]
async fn test_task_list_sorting(cx: &mut TestAppContext) {
    init_test(cx);
    let inventory = cx.update(|cx| Inventory::new(cx));
    let initial_tasks = resolved_task_names(&inventory, None, cx).await;
    assert!(
        initial_tasks.is_empty(),
        "No tasks expected for empty inventory, but got {initial_tasks:?}"
    );
    let initial_tasks = task_template_names(&inventory, None, cx).await;
    assert!(
        initial_tasks.is_empty(),
        "No tasks expected for empty inventory, but got {initial_tasks:?}"
    );
    cx.run_until_parked();
    let expected_initial_state = [
        "1_a_task".to_string(),
        "1_task".to_string(),
        "2_task".to_string(),
        "3_task".to_string(),
    ];

    inventory.update(cx, |inventory, _| {
        inventory
            .update_file_based_tasks(
                TaskSettingsLocation::Global(tasks_file()),
                Some(&mock_tasks_from_names(
                    expected_initial_state.iter().map(|name| name.as_str()),
                )),
            )
            .unwrap();
    });
    assert_eq!(
        task_template_names(&inventory, None, cx).await,
        &expected_initial_state,
    );
    assert_eq!(
        resolved_task_names(&inventory, None, cx).await,
        &expected_initial_state,
        "Tasks with equal amount of usages should be sorted alphanumerically"
    );

    register_task_used(&inventory, "2_task", cx).await;
    assert_eq!(
        task_template_names(&inventory, None, cx).await,
        &expected_initial_state,
    );
    assert_eq!(
        resolved_task_names(&inventory, None, cx).await,
        vec![
            "2_task".to_string(),
            "1_a_task".to_string(),
            "1_task".to_string(),
            "3_task".to_string()
        ],
    );

    register_task_used(&inventory, "1_task", cx).await;
    register_task_used(&inventory, "1_task", cx).await;
    register_task_used(&inventory, "1_task", cx).await;
    register_task_used(&inventory, "3_task", cx).await;
    assert_eq!(
        task_template_names(&inventory, None, cx).await,
        &expected_initial_state,
    );
    assert_eq!(
        resolved_task_names(&inventory, None, cx).await,
        vec![
            "3_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "1_a_task".to_string(),
        ],
        "Most recently used task should be at the top"
    );

    let worktree_id = WorktreeId::from_usize(0);
    let local_worktree_location = SettingsLocation {
        worktree_id,
        path: rel_path("foo"),
    };
    inventory.update(cx, |inventory, _| {
        inventory
            .update_file_based_tasks(
                TaskSettingsLocation::Worktree(local_worktree_location),
                Some(&mock_tasks_from_names(["worktree_task_1"])),
            )
            .unwrap();
    });
    assert_eq!(
        resolved_task_names(&inventory, None, cx).await,
        vec![
            "3_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "1_a_task".to_string(),
        ],
        "Most recently used task should be at the top"
    );
    assert_eq!(
        resolved_task_names(&inventory, Some(worktree_id), cx).await,
        vec![
            "3_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "worktree_task_1".to_string(),
            "1_a_task".to_string(),
        ],
    );
    register_worktree_task_used(&inventory, worktree_id, "worktree_task_1", cx).await;
    assert_eq!(
        resolved_task_names(&inventory, Some(worktree_id), cx).await,
        vec![
            "worktree_task_1".to_string(),
            "3_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "1_a_task".to_string(),
        ],
        "Most recently used worktree task should be at the top"
    );

    inventory.update(cx, |inventory, _| {
        inventory
            .update_file_based_tasks(
                TaskSettingsLocation::Global(tasks_file()),
                Some(&mock_tasks_from_names(
                    ["10_hello", "11_hello"]
                        .into_iter()
                        .chain(expected_initial_state.iter().map(|name| name.as_str())),
                )),
            )
            .unwrap();
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
        task_template_names(&inventory, None, cx).await,
        &expected_updated_state,
    );
    assert_eq!(
        resolved_task_names(&inventory, None, cx).await,
        vec![
            "worktree_task_1".to_string(),
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
            "10_hello".to_string(),
            "11_hello".to_string(),
        ],
        "After global tasks update, worktree task usage is not erased and it's the first still; global task is back to regular order as its file was updated"
    );

    register_task_used(&inventory, "11_hello", cx).await;
    assert_eq!(
        task_template_names(&inventory, None, cx).await,
        &expected_updated_state,
    );
    assert_eq!(
        resolved_task_names(&inventory, None, cx).await,
        vec![
            "11_hello".to_string(),
            "worktree_task_1".to_string(),
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
            "10_hello".to_string(),
        ],
    );
}

#[gpui::test]
async fn test_reloading_debug_scenarios(cx: &mut TestAppContext) {
    init_test(cx);
    let inventory = cx.update(|cx| Inventory::new(cx));
    inventory.update(cx, |inventory, _| {
        inventory
            .update_file_based_scenarios(
                TaskSettingsLocation::Global(Path::new("")),
                Some(
                    r#"
                        [{
                            "label": "test scenario",
                            "adapter": "CodeLLDB",
                            "request": "launch",
                            "program": "wowzer",
                        }]
                        "#,
                ),
            )
            .unwrap();
    });

    let (_, scenario) = inventory
        .update(cx, |this, cx| {
            this.list_debug_scenarios(&TaskContexts::default(), vec![], vec![], false, cx)
        })
        .await
        .1
        .first()
        .unwrap()
        .clone();

    inventory.update(cx, |this, _| {
        this.scenario_scheduled(scenario.clone(), Default::default(), None, None);
    });

    assert_eq!(
        inventory
            .update(cx, |this, cx| {
                this.list_debug_scenarios(&Default::default(), vec![], vec![], false, cx)
            })
            .await
            .0
            .first()
            .unwrap()
            .clone()
            .0,
        scenario
    );

    inventory.update(cx, |this, _| {
        this.update_file_based_scenarios(
            TaskSettingsLocation::Global(Path::new("")),
            Some(
                r#"
                        [{
                            "label": "test scenario",
                            "adapter": "Delve",
                            "request": "launch",
                            "program": "wowzer",
                        }]
                        "#,
            ),
        )
        .unwrap();
    });

    assert_eq!(
        inventory
            .update(cx, |this, cx| {
                this.list_debug_scenarios(&Default::default(), vec![], vec![], false, cx)
            })
            .await
            .0
            .first()
            .unwrap()
            .0
            .adapter,
        "Delve",
    );

    inventory.update(cx, |this, _| {
        this.update_file_based_scenarios(
            TaskSettingsLocation::Global(Path::new("")),
            Some(
                r#"
                        [{
                            "label": "testing scenario",
                            "adapter": "Delve",
                            "request": "launch",
                            "program": "wowzer",
                        }]
                        "#,
            ),
        )
        .unwrap();
    });

    assert!(
        inventory
            .update(cx, |this, cx| {
                this.list_debug_scenarios(&TaskContexts::default(), vec![], vec![], false, cx)
            })
            .await
            .0
            .is_empty(),
    );
}

#[gpui::test]
async fn test_inventory_static_task_filters(cx: &mut TestAppContext) {
    init_test(cx);
    let inventory = cx.update(|cx| Inventory::new(cx));
    let common_name = "common_task_name";
    let worktree_1 = WorktreeId::from_usize(1);
    let worktree_2 = WorktreeId::from_usize(2);

    cx.run_until_parked();
    let worktree_independent_tasks = vec![
        (
            TaskSourceKind::AbsPath {
                id_base: "global tasks.json".into(),
                abs_path: paths::tasks_file().clone(),
            },
            common_name.to_string(),
        ),
        (
            TaskSourceKind::AbsPath {
                id_base: "global tasks.json".into(),
                abs_path: paths::tasks_file().clone(),
            },
            "static_source_1".to_string(),
        ),
        (
            TaskSourceKind::AbsPath {
                id_base: "global tasks.json".into(),
                abs_path: paths::tasks_file().clone(),
            },
            "static_source_2".to_string(),
        ),
    ];
    let worktree_1_tasks = [
        (
            TaskSourceKind::Worktree {
                id: worktree_1,
                directory_in_worktree: rel_path(".zed").into(),
                id_base: "local worktree tasks from directory \".zed\"".into(),
            },
            common_name.to_string(),
        ),
        (
            TaskSourceKind::Worktree {
                id: worktree_1,
                directory_in_worktree: rel_path(".zed").into(),
                id_base: "local worktree tasks from directory \".zed\"".into(),
            },
            "worktree_1".to_string(),
        ),
    ];
    let worktree_2_tasks = [
        (
            TaskSourceKind::Worktree {
                id: worktree_2,
                directory_in_worktree: rel_path(".zed").into(),
                id_base: "local worktree tasks from directory \".zed\"".into(),
            },
            common_name.to_string(),
        ),
        (
            TaskSourceKind::Worktree {
                id: worktree_2,
                directory_in_worktree: rel_path(".zed").into(),
                id_base: "local worktree tasks from directory \".zed\"".into(),
            },
            "worktree_2".to_string(),
        ),
    ];

    inventory.update(cx, |inventory, _| {
        inventory
            .update_file_based_tasks(
                TaskSettingsLocation::Global(tasks_file()),
                Some(&mock_tasks_from_names(
                    worktree_independent_tasks
                        .iter()
                        .map(|(_, name)| name.as_str()),
                )),
            )
            .unwrap();
        inventory
            .update_file_based_tasks(
                TaskSettingsLocation::Worktree(SettingsLocation {
                    worktree_id: worktree_1,
                    path: rel_path(".zed"),
                }),
                Some(&mock_tasks_from_names(
                    worktree_1_tasks.iter().map(|(_, name)| name.as_str()),
                )),
            )
            .unwrap();
        inventory
            .update_file_based_tasks(
                TaskSettingsLocation::Worktree(SettingsLocation {
                    worktree_id: worktree_2,
                    path: rel_path(".zed"),
                }),
                Some(&mock_tasks_from_names(
                    worktree_2_tasks.iter().map(|(_, name)| name.as_str()),
                )),
            )
            .unwrap();
    });

    assert_eq!(
        list_tasks_sorted_by_last_used(&inventory, None, cx).await,
        worktree_independent_tasks,
        "Without a worktree, only worktree-independent tasks should be listed"
    );
    assert_eq!(
        list_tasks_sorted_by_last_used(&inventory, Some(worktree_1), cx).await,
        worktree_1_tasks
            .iter()
            .chain(worktree_independent_tasks.iter())
            .cloned()
            .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        list_tasks_sorted_by_last_used(&inventory, Some(worktree_2), cx).await,
        worktree_2_tasks
            .iter()
            .chain(worktree_independent_tasks.iter())
            .cloned()
            .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        list_tasks(&inventory, None, cx).await,
        worktree_independent_tasks,
        "Without a worktree, only worktree-independent tasks should be listed"
    );
    assert_eq!(
        list_tasks(&inventory, Some(worktree_1), cx).await,
        worktree_1_tasks
            .iter()
            .chain(worktree_independent_tasks.iter())
            .cloned()
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        list_tasks(&inventory, Some(worktree_2), cx).await,
        worktree_2_tasks
            .iter()
            .chain(worktree_independent_tasks.iter())
            .cloned()
            .collect::<Vec<_>>(),
    );
}

fn init_test(_cx: &mut TestAppContext) {
    zlog::init_test();
    TaskStore::init(None);
}

fn resolved_task_names(
    inventory: &Entity<Inventory>,
    worktree: Option<WorktreeId>,
    cx: &mut TestAppContext,
) -> Task<Vec<String>> {
    let tasks = inventory.update(cx, |inventory, cx| {
        let mut task_contexts = TaskContexts::default();
        task_contexts.active_worktree_context =
            worktree.map(|worktree| (worktree, Default::default()));

        inventory.used_and_current_resolved_tasks(Arc::new(task_contexts), cx)
    });

    cx.background_spawn(async move {
        let (used, current) = tasks.await;
        used.into_iter()
            .chain(current)
            .map(|(_, task)| task.original_task().label.clone())
            .collect()
    })
}

fn mock_tasks_from_names<'a>(task_names: impl IntoIterator<Item = &'a str> + 'a) -> String {
    serde_json::to_string(&serde_json::Value::Array(
        task_names
            .into_iter()
            .map(|task_name| {
                json!({
                    "label": task_name,
                    "command": "echo",
                    "args": vec![task_name],
                })
            })
            .collect::<Vec<_>>(),
    ))
    .unwrap()
}

async fn list_tasks_sorted_by_last_used(
    inventory: &Entity<Inventory>,
    worktree: Option<WorktreeId>,
    cx: &mut TestAppContext,
) -> Vec<(TaskSourceKind, String)> {
    let (used, current) = inventory
        .update(cx, |inventory, cx| {
            let mut task_contexts = TaskContexts::default();
            task_contexts.active_worktree_context =
                worktree.map(|worktree| (worktree, Default::default()));

            inventory.used_and_current_resolved_tasks(Arc::new(task_contexts), cx)
        })
        .await;
    let mut all = used;
    all.extend(current);
    all.into_iter()
        .map(|(source_kind, task)| (source_kind, task.resolved_label))
        .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
        .collect()
}
