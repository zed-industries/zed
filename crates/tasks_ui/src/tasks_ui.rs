use ::settings::Settings;
use editor::{tasks::task_context, Editor};
use gpui::{App, Context, Task as AsyncTask, Window};
use modal::{TaskOverrides, TasksModal};
use project::{Location, WorktreeId};
use task::{RevealTarget, TaskId};
use workspace::tasks::schedule_task;
use workspace::{tasks::schedule_resolved_task, Workspace};

mod modal;
mod settings;

pub use modal::{Rerun, Spawn};

pub fn init(cx: &mut App) {
    settings::TaskSettings::register(cx);
    cx.observe_new(
        |workspace: &mut Workspace, _window: Option<&mut Window>, _: &mut Context<Workspace>| {
            workspace
                .register_action(spawn_task_or_modal)
                .register_action(move |workspace, action: &modal::Rerun, window, cx| {
                    if let Some((task_source_kind, mut last_scheduled_task)) = workspace
                        .project()
                        .read(cx)
                        .task_store()
                        .read(cx)
                        .task_inventory()
                        .and_then(|inventory| {
                            inventory.read(cx).last_scheduled_task(
                                action
                                    .task_id
                                    .as_ref()
                                    .map(|id| TaskId(id.clone()))
                                    .as_ref(),
                            )
                        })
                    {
                        if action.reevaluate_context {
                            let mut original_task = last_scheduled_task.original_task().clone();
                            if let Some(allow_concurrent_runs) = action.allow_concurrent_runs {
                                original_task.allow_concurrent_runs = allow_concurrent_runs;
                            }
                            if let Some(use_new_terminal) = action.use_new_terminal {
                                original_task.use_new_terminal = use_new_terminal;
                            }
                            let context_task = task_context(workspace, window, cx);
                            cx.spawn_in(window, |workspace, mut cx| async move {
                                let task_context = context_task.await;
                                workspace
                                    .update(&mut cx, |workspace, cx| {
                                        schedule_task(
                                            workspace,
                                            task_source_kind,
                                            &original_task,
                                            &task_context,
                                            false,
                                            cx,
                                        )
                                    })
                                    .ok()
                            })
                            .detach()
                        } else {
                            if let Some(resolved) = last_scheduled_task.resolved.as_mut() {
                                if let Some(allow_concurrent_runs) = action.allow_concurrent_runs {
                                    resolved.allow_concurrent_runs = allow_concurrent_runs;
                                }
                                if let Some(use_new_terminal) = action.use_new_terminal {
                                    resolved.use_new_terminal = use_new_terminal;
                                }
                            }

                            schedule_resolved_task(
                                workspace,
                                task_source_kind,
                                last_scheduled_task,
                                false,
                                cx,
                            );
                        }
                    } else {
                        toggle_modal(workspace, None, window, cx).detach();
                    };
                });
        },
    )
    .detach();
}

fn spawn_task_or_modal(
    workspace: &mut Workspace,
    action: &Spawn,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    match action {
        Spawn::ByName {
            task_name,
            reveal_target,
        } => {
            let overrides = reveal_target.map(|reveal_target| TaskOverrides {
                reveal_target: Some(reveal_target),
            });
            spawn_task_with_name(task_name.clone(), overrides, window, cx).detach_and_log_err(cx)
        }
        Spawn::ViaModal { reveal_target } => {
            toggle_modal(workspace, *reveal_target, window, cx).detach()
        }
    }
}

fn toggle_modal(
    workspace: &mut Workspace,
    reveal_target: Option<RevealTarget>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> AsyncTask<()> {
    let task_store = workspace.project().read(cx).task_store().clone();
    let workspace_handle = workspace.weak_handle();
    let can_open_modal = workspace.project().update(cx, |project, cx| {
        project.is_local() || project.ssh_connection_string(cx).is_some() || project.is_via_ssh()
    });
    if can_open_modal {
        let context_task = task_context(workspace, window, cx);
        cx.spawn_in(window, |workspace, mut cx| async move {
            let task_context = context_task.await;
            workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        TasksModal::new(
                            task_store.clone(),
                            task_context,
                            reveal_target.map(|target| TaskOverrides {
                                reveal_target: Some(target),
                            }),
                            workspace_handle,
                            window,
                            cx,
                        )
                    })
                })
                .ok();
        })
    } else {
        AsyncTask::ready(())
    }
}

fn spawn_task_with_name(
    name: String,
    overrides: Option<TaskOverrides>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> AsyncTask<anyhow::Result<()>> {
    cx.spawn_in(window, |workspace, mut cx| async move {
        let context_task = workspace.update_in(&mut cx, |workspace, window, cx| {
            task_context(workspace, window, cx)
        })?;
        let task_context = context_task.await;
        let tasks = workspace.update(&mut cx, |workspace, cx| {
            let Some(task_inventory) = workspace
                .project()
                .read(cx)
                .task_store()
                .read(cx)
                .task_inventory()
                .cloned()
            else {
                return Vec::new();
            };
            let (worktree, location) = active_item_selection_properties(workspace, cx);
            let (file, language) = location
                .map(|location| {
                    let buffer = location.buffer.read(cx);
                    (
                        buffer.file().cloned(),
                        buffer.language_at(location.range.start),
                    )
                })
                .unwrap_or_default();
            task_inventory
                .read(cx)
                .list_tasks(file, language, worktree, cx)
        })?;

        let did_spawn = workspace
            .update(&mut cx, |workspace, cx| {
                let (task_source_kind, mut target_task) =
                    tasks.into_iter().find(|(_, task)| task.label == name)?;
                if let Some(overrides) = &overrides {
                    if let Some(target_override) = overrides.reveal_target {
                        target_task.reveal_target = target_override;
                    }
                }
                schedule_task(
                    workspace,
                    task_source_kind,
                    &target_task,
                    &task_context,
                    false,
                    cx,
                );
                Some(())
            })?
            .is_some();
        if !did_spawn {
            workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    spawn_task_or_modal(
                        workspace,
                        &Spawn::ViaModal {
                            reveal_target: overrides.and_then(|overrides| overrides.reveal_target),
                        },
                        window,
                        cx,
                    );
                })
                .ok();
        }

        Ok(())
    })
}

fn active_item_selection_properties(
    workspace: &Workspace,
    cx: &mut App,
) -> (Option<WorktreeId>, Option<Location>) {
    let active_item = workspace.active_item(cx);
    let worktree_id = active_item
        .as_ref()
        .and_then(|item| item.project_path(cx))
        .map(|path| path.worktree_id);
    let location = active_item
        .and_then(|active_item| active_item.act_as::<Editor>(cx))
        .and_then(|editor| {
            editor.update(cx, |editor, cx| {
                let selection = editor.selections.newest_anchor();
                let multi_buffer = editor.buffer().clone();
                let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
                let (buffer_snapshot, buffer_offset) =
                    multi_buffer_snapshot.point_to_buffer_offset(selection.head())?;
                let buffer_anchor = buffer_snapshot.anchor_before(buffer_offset);
                let buffer = multi_buffer.read(cx).buffer(buffer_snapshot.remote_id())?;
                Some(Location {
                    buffer,
                    range: buffer_anchor..buffer_anchor,
                })
            })
        });
    (worktree_id, location)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use editor::Editor;
    use gpui::TestAppContext;
    use language::{Language, LanguageConfig};
    use project::{task_store::TaskStore, BasicContextProvider, FakeFs, Project};
    use serde_json::json;
    use task::{TaskContext, TaskVariables, VariableName};
    use ui::VisualContext;
    use util::{path, separator};
    use workspace::{AppState, Workspace};

    use crate::task_context;

    #[gpui::test]
    async fn test_default_language_context(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                ".zed": {
                    "tasks.json": r#"[
                            {
                                "label": "example task",
                                "command": "echo",
                                "args": ["4"]
                            },
                            {
                                "label": "another one",
                                "command": "echo",
                                "args": ["55"]
                            },
                        ]"#,
                },
                "a.ts": "function this_is_a_test() { }",
                "rust": {
                                    "b.rs": "use std; fn this_is_a_rust_file() { }",
                }

            }),
        )
        .await;
        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let worktree_store = project.update(cx, |project, _| project.worktree_store().clone());
        let rust_language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_outline_query(
                r#"(function_item
            "fn" @context
            name: (_) @name) @item"#,
            )
            .unwrap()
            .with_context_provider(Some(Arc::new(BasicContextProvider::new(
                worktree_store.clone(),
            )))),
        );

        let typescript_language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            )
            .with_outline_query(
                r#"(function_declaration
                    "async"? @context
                    "function" @context
                    name: (_) @name
                    parameters: (formal_parameters
                      "(" @context
                      ")" @context)) @item"#,
            )
            .unwrap()
            .with_context_provider(Some(Arc::new(BasicContextProvider::new(
                worktree_store.clone(),
            )))),
        );

        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let buffer1 = workspace
            .update(cx, |this, cx| {
                this.project()
                    .update(cx, |this, cx| this.open_buffer((worktree_id, "a.ts"), cx))
            })
            .await
            .unwrap();
        buffer1.update(cx, |this, cx| {
            this.set_language(Some(typescript_language), cx)
        });
        let editor1 = cx.new_window_entity(|window, cx| {
            Editor::for_buffer(buffer1, Some(project.clone()), window, cx)
        });
        let buffer2 = workspace
            .update(cx, |this, cx| {
                this.project().update(cx, |this, cx| {
                    this.open_buffer((worktree_id, "rust/b.rs"), cx)
                })
            })
            .await
            .unwrap();
        buffer2.update(cx, |this, cx| this.set_language(Some(rust_language), cx));
        let editor2 = cx
            .new_window_entity(|window, cx| Editor::for_buffer(buffer2, Some(project), window, cx));

        let first_context = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.add_item_to_center(Box::new(editor1.clone()), window, cx);
                workspace.add_item_to_center(Box::new(editor2.clone()), window, cx);
                assert_eq!(
                    workspace.active_item(cx).unwrap().item_id(),
                    editor2.entity_id()
                );
                task_context(workspace, window, cx)
            })
            .await;

        assert_eq!(
            first_context,
            TaskContext {
                cwd: Some(path!("/dir").into()),
                task_variables: TaskVariables::from_iter([
                    (VariableName::File, path!("/dir/rust/b.rs").into()),
                    (VariableName::Filename, "b.rs".into()),
                    (VariableName::RelativeFile, separator!("rust/b.rs").into()),
                    (VariableName::Dirname, path!("/dir/rust").into()),
                    (VariableName::Stem, "b".into()),
                    (VariableName::WorktreeRoot, path!("/dir").into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "1".into()),
                ]),
                project_env: HashMap::default(),
            }
        );

        // And now, let's select an identifier.
        editor2.update_in(cx, |editor, window, cx| {
            editor.change_selections(None, window, cx, |selections| {
                selections.select_ranges([14..18])
            })
        });

        assert_eq!(
            workspace
                .update_in(cx, |workspace, window, cx| {
                    task_context(workspace, window, cx)
                })
                .await,
            TaskContext {
                cwd: Some(path!("/dir").into()),
                task_variables: TaskVariables::from_iter([
                    (VariableName::File, path!("/dir/rust/b.rs").into()),
                    (VariableName::Filename, "b.rs".into()),
                    (VariableName::RelativeFile, separator!("rust/b.rs").into()),
                    (VariableName::Dirname, path!("/dir/rust").into()),
                    (VariableName::Stem, "b".into()),
                    (VariableName::WorktreeRoot, path!("/dir").into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "15".into()),
                    (VariableName::SelectedText, "is_i".into()),
                    (VariableName::Symbol, "this_is_a_rust_file".into()),
                ]),
                project_env: HashMap::default(),
            }
        );

        assert_eq!(
            workspace
                .update_in(cx, |workspace, window, cx| {
                    // Now, let's switch the active item to .ts file.
                    workspace.activate_item(&editor1, true, true, window, cx);
                    task_context(workspace, window, cx)
                })
                .await,
            TaskContext {
                cwd: Some(path!("/dir").into()),
                task_variables: TaskVariables::from_iter([
                    (VariableName::File, path!("/dir/a.ts").into()),
                    (VariableName::Filename, "a.ts".into()),
                    (VariableName::RelativeFile, "a.ts".into()),
                    (VariableName::Dirname, path!("/dir").into()),
                    (VariableName::Stem, "a".into()),
                    (VariableName::WorktreeRoot, path!("/dir").into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "1".into()),
                    (VariableName::Symbol, "this_is_a_test".into()),
                ]),
                project_env: HashMap::default(),
            }
        );
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            language::init(cx);
            crate::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            TaskStore::init(None);
            state
        })
    }
}
