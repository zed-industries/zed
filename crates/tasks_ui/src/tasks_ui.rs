use std::path::Path;

use collections::HashMap;
use editor::Editor;
use gpui::{App, AppContext as _, Context, Entity, Task, Window};
use modal::TaskOverrides;
use project::{Location, TaskContexts, TaskSourceKind, Worktree};
use task::{
    RevealTarget, TaskContext, TaskId, TaskModal, TaskTemplate, TaskVariables, VariableName,
};
use workspace::Workspace;

mod modal;

pub use modal::{Rerun, ShowAttachModal, Spawn, TasksModal};

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _: Option<&mut Window>, _: &mut Context<Workspace>| {
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
                            let task_contexts = task_contexts(workspace, window, cx);
                            cx.spawn_in(window, async move |workspace, cx| {
                                let task_contexts = task_contexts.await;
                                let default_context = TaskContext::default();
                                workspace
                                    .update_in(cx, |workspace, window, cx| {
                                        workspace.schedule_task(
                                            task_source_kind,
                                            &original_task,
                                            task_contexts
                                                .active_context()
                                                .unwrap_or(&default_context),
                                            false,
                                            window,
                                            cx,
                                        )
                                    })
                                    .ok()
                            })
                            .detach()
                        } else {
                            let resolved = &mut last_scheduled_task.resolved;

                            if let Some(allow_concurrent_runs) = action.allow_concurrent_runs {
                                resolved.allow_concurrent_runs = allow_concurrent_runs;
                            }
                            if let Some(use_new_terminal) = action.use_new_terminal {
                                resolved.use_new_terminal = use_new_terminal;
                            }

                            workspace.schedule_resolved_task(
                                task_source_kind,
                                last_scheduled_task,
                                false,
                                window,
                                cx,
                            );
                        }
                    } else {
                        toggle_modal(workspace, None, TaskModal::ScriptModal, window, cx).detach();
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
            let name = task_name.clone();
            spawn_tasks_filtered(move |(_, task)| task.label.eq(&name), overrides, window, cx)
                .detach_and_log_err(cx)
        }
        Spawn::ByTag {
            task_tag,
            reveal_target,
        } => {
            let overrides = reveal_target.map(|reveal_target| TaskOverrides {
                reveal_target: Some(reveal_target),
            });
            let tag = task_tag.clone();
            spawn_tasks_filtered(
                move |(_, task)| task.tags.contains(&tag),
                overrides,
                window,
                cx,
            )
            .detach_and_log_err(cx)
        }
        Spawn::ViaModal { reveal_target } => toggle_modal(
            workspace,
            *reveal_target,
            TaskModal::ScriptModal,
            window,
            cx,
        )
        .detach(),
    }
}

pub fn toggle_modal(
    workspace: &mut Workspace,
    reveal_target: Option<RevealTarget>,
    task_type: TaskModal,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Task<()> {
    let task_store = workspace.project().read(cx).task_store().clone();
    let workspace_handle = workspace.weak_handle();
    let can_open_modal = workspace.project().update(cx, |project, cx| {
        project.is_local() || project.ssh_connection_string(cx).is_some() || project.is_via_ssh()
    });
    if can_open_modal {
        let task_contexts = task_contexts(workspace, window, cx);
        cx.spawn_in(window, async move |workspace, cx| {
            let task_contexts = task_contexts.await;
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        TasksModal::new(
                            task_store.clone(),
                            task_contexts,
                            reveal_target.map(|target| TaskOverrides {
                                reveal_target: Some(target),
                            }),
                            workspace_handle,
                            task_type,
                            window,
                            cx,
                        )
                    })
                })
                .ok();
        })
    } else {
        Task::ready(())
    }
}

fn spawn_tasks_filtered<F>(
    mut predicate: F,
    overrides: Option<TaskOverrides>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Task<anyhow::Result<()>>
where
    F: FnMut((&TaskSourceKind, &TaskTemplate)) -> bool + 'static,
{
    cx.spawn_in(window, async move |workspace, cx| {
        let task_contexts = workspace.update_in(cx, |workspace, window, cx| {
            task_contexts(workspace, window, cx)
        })?;
        let task_contexts = task_contexts.await;
        let mut tasks = workspace.update(cx, |workspace, cx| {
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
            let (file, language) = task_contexts
                .location()
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
                .list_tasks(file, language, task_contexts.worktree(), cx)
        })?;

        let did_spawn = workspace
            .update_in(cx, |workspace, window, cx| {
                let default_context = TaskContext::default();
                let active_context = task_contexts.active_context().unwrap_or(&default_context);

                tasks.retain_mut(|(task_source_kind, target_task)| {
                    if predicate((task_source_kind, target_task)) {
                        if let Some(overrides) = &overrides {
                            if let Some(target_override) = overrides.reveal_target {
                                target_task.reveal_target = target_override;
                            }
                        }
                        workspace.schedule_task(
                            task_source_kind.clone(),
                            target_task,
                            active_context,
                            false,
                            window,
                            cx,
                        );
                        true
                    } else {
                        false
                    }
                });

                if tasks.is_empty() { None } else { Some(()) }
            })?
            .is_some();
        if !did_spawn {
            workspace
                .update_in(cx, |workspace, window, cx| {
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

pub fn task_contexts(
    workspace: &Workspace,
    window: &mut Window,
    cx: &mut App,
) -> Task<TaskContexts> {
    let active_item = workspace.active_item(cx);
    let active_worktree = active_item
        .as_ref()
        .and_then(|item| item.project_path(cx))
        .map(|project_path| project_path.worktree_id)
        .filter(|worktree_id| {
            workspace
                .project()
                .read(cx)
                .worktree_for_id(*worktree_id, cx)
                .map_or(false, |worktree| is_visible_directory(&worktree, cx))
        });

    let active_editor = active_item.and_then(|item| item.act_as::<Editor>(cx));

    let editor_context_task = active_editor.as_ref().map(|active_editor| {
        active_editor.update(cx, |editor, cx| editor.task_context(window, cx))
    });

    let location = active_editor.as_ref().and_then(|editor| {
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

    let lsp_task_sources = active_editor
        .as_ref()
        .map(|active_editor| active_editor.update(cx, |editor, cx| editor.lsp_task_sources(cx)))
        .unwrap_or_default();

    let latest_selection = active_editor.as_ref().map(|active_editor| {
        active_editor.update(cx, |editor, _| {
            editor.selections.newest_anchor().head().text_anchor
        })
    });

    let mut worktree_abs_paths = workspace
        .worktrees(cx)
        .filter(|worktree| is_visible_directory(worktree, cx))
        .map(|worktree| {
            let worktree = worktree.read(cx);
            (worktree.id(), worktree.abs_path())
        })
        .collect::<HashMap<_, _>>();

    cx.background_spawn(async move {
        let mut task_contexts = TaskContexts::default();

        task_contexts.lsp_task_sources = lsp_task_sources;
        task_contexts.latest_selection = latest_selection;

        if let Some(editor_context_task) = editor_context_task {
            if let Some(editor_context) = editor_context_task.await {
                task_contexts.active_item_context =
                    Some((active_worktree, location, editor_context));
            }
        }

        if let Some(active_worktree) = active_worktree {
            if let Some(active_worktree_abs_path) = worktree_abs_paths.remove(&active_worktree) {
                task_contexts.active_worktree_context =
                    Some((active_worktree, worktree_context(&active_worktree_abs_path)));
            }
        } else if worktree_abs_paths.len() == 1 {
            task_contexts.active_worktree_context = worktree_abs_paths
                .drain()
                .next()
                .map(|(id, abs_path)| (id, worktree_context(&abs_path)));
        }

        task_contexts.other_worktree_contexts.extend(
            worktree_abs_paths
                .into_iter()
                .map(|(id, abs_path)| (id, worktree_context(&abs_path))),
        );
        task_contexts
    })
}

fn is_visible_directory(worktree: &Entity<Worktree>, cx: &App) -> bool {
    let worktree = worktree.read(cx);
    worktree.is_visible() && worktree.root_entry().map_or(false, |entry| entry.is_dir())
}

fn worktree_context(worktree_abs_path: &Path) -> TaskContext {
    let mut task_variables = TaskVariables::default();
    task_variables.insert(
        VariableName::WorktreeRoot,
        worktree_abs_path.to_string_lossy().to_string(),
    );
    TaskContext {
        cwd: Some(worktree_abs_path.to_path_buf()),
        task_variables,
        project_env: HashMap::default(),
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use editor::Editor;
    use gpui::TestAppContext;
    use language::{Language, LanguageConfig};
    use project::{BasicContextProvider, FakeFs, Project, task_store::TaskStore};
    use serde_json::json;
    use task::{TaskContext, TaskVariables, VariableName};
    use ui::VisualContext;
    use util::{path, separator};
    use workspace::{AppState, Workspace};

    use crate::task_contexts;

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
                task_contexts(workspace, window, cx)
            })
            .await;

        assert_eq!(
            first_context
                .active_context()
                .expect("Should have an active context"),
            &TaskContext {
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
                    task_contexts(workspace, window, cx)
                })
                .await
                .active_context()
                .expect("Should have an active context"),
            &TaskContext {
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
                    task_contexts(workspace, window, cx)
                })
                .await
                .active_context()
                .expect("Should have an active context"),
            &TaskContext {
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
