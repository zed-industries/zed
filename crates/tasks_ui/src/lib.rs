use ::settings::Settings;
use editor::{tasks::task_context, Editor};
use gpui::{AppContext, Task as AsyncTask, ViewContext, WindowContext};
use modal::TasksModal;
use project::{Location, WorktreeId};
use workspace::tasks::schedule_task;
use workspace::{tasks::schedule_resolved_task, Workspace};

mod modal;
mod settings;

pub use modal::{Rerun, Spawn};

pub fn init(cx: &mut AppContext) {
    settings::TaskSettings::register(cx);
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace
                .register_action(spawn_task_or_modal)
                .register_action(move |workspace, action: &modal::Rerun, cx| {
                    if let Some((task_source_kind, mut last_scheduled_task)) =
                        workspace.project().update(cx, |project, cx| {
                            project
                                .task_inventory()
                                .read(cx)
                                .last_scheduled_task(action.task_id.as_ref())
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
                            let context_task = task_context(workspace, cx);
                            cx.spawn(|workspace, mut cx| async move {
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
                        toggle_modal(workspace, cx).detach();
                    };
                });
        },
    )
    .detach();
}

fn spawn_task_or_modal(workspace: &mut Workspace, action: &Spawn, cx: &mut ViewContext<Workspace>) {
    match &action.task_name {
        Some(name) => spawn_task_with_name(name.clone(), cx).detach_and_log_err(cx),
        None => toggle_modal(workspace, cx).detach(),
    }
}

fn toggle_modal(workspace: &mut Workspace, cx: &mut ViewContext<'_, Workspace>) -> AsyncTask<()> {
    let project = workspace.project().clone();
    let workspace_handle = workspace.weak_handle();
    let context_task = task_context(workspace, cx);
    cx.spawn(|workspace, mut cx| async move {
        let task_context = context_task.await;
        workspace
            .update(&mut cx, |workspace, cx| {
                if workspace.project().update(cx, |project, cx| {
                    project.is_local() || project.ssh_connection_string(cx).is_some()
                }) {
                    workspace.toggle_modal(cx, |cx| {
                        TasksModal::new(project, task_context, workspace_handle, cx)
                    })
                }
            })
            .ok();
    })
}

fn spawn_task_with_name(
    name: String,
    cx: &mut ViewContext<Workspace>,
) -> AsyncTask<anyhow::Result<()>> {
    cx.spawn(|workspace, mut cx| async move {
        let context_task =
            workspace.update(&mut cx, |workspace, cx| task_context(workspace, cx))?;
        let task_context = context_task.await;
        let tasks = workspace
            .update(&mut cx, |workspace, cx| {
                let (worktree, location) = active_item_selection_properties(workspace, cx);
                workspace.project().update(cx, |project, cx| {
                    project.task_templates(worktree, location, cx)
                })
            })?
            .await?;

        let did_spawn = workspace
            .update(&mut cx, |workspace, cx| {
                let (task_source_kind, target_task) =
                    tasks.into_iter().find(|(_, task)| task.label == name)?;
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
                .update(&mut cx, |workspace, cx| {
                    spawn_task_or_modal(workspace, &Spawn::default(), cx);
                })
                .ok();
        }

        Ok(())
    })
}

fn active_item_selection_properties(
    workspace: &Workspace,
    cx: &mut WindowContext,
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
    use std::sync::Arc;

    use editor::Editor;
    use gpui::{Entity, TestAppContext};
    use language::{Language, LanguageConfig};
    use project::{BasicContextProvider, FakeFs, Project};
    use serde_json::json;
    use task::{TaskContext, TaskVariables, VariableName};
    use ui::VisualContext;
    use workspace::{AppState, Workspace};

    use crate::task_context;

    #[gpui::test]
    async fn test_default_language_context(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/dir",
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
        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        let rust_language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_rust::language()),
            )
            .with_outline_query(
                r#"(function_item
            "fn" @context
            name: (_) @name) @item"#,
            )
            .unwrap()
            .with_context_provider(Some(Arc::new(BasicContextProvider::new(project.clone())))),
        );

        let typescript_language = Arc::new(
            Language::new(
                LanguageConfig::default(),
                Some(tree_sitter_typescript::language_typescript()),
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
            .with_context_provider(Some(Arc::new(BasicContextProvider::new(project.clone())))),
        );

        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees().next().unwrap().read(cx).id()
        });
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

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
        let editor1 = cx.new_view(|cx| Editor::for_buffer(buffer1, Some(project.clone()), cx));
        let buffer2 = workspace
            .update(cx, |this, cx| {
                this.project().update(cx, |this, cx| {
                    this.open_buffer((worktree_id, "rust/b.rs"), cx)
                })
            })
            .await
            .unwrap();
        buffer2.update(cx, |this, cx| this.set_language(Some(rust_language), cx));
        let editor2 = cx.new_view(|cx| Editor::for_buffer(buffer2, Some(project), cx));

        let first_context = workspace
            .update(cx, |workspace, cx| {
                workspace.add_item_to_center(Box::new(editor1.clone()), cx);
                workspace.add_item_to_center(Box::new(editor2.clone()), cx);
                assert_eq!(
                    workspace.active_item(cx).unwrap().item_id(),
                    editor2.entity_id()
                );
                task_context(workspace, cx)
            })
            .await;
        assert_eq!(
            first_context,
            TaskContext {
                cwd: Some("/dir".into()),
                task_variables: TaskVariables::from_iter([
                    (VariableName::File, "/dir/rust/b.rs".into()),
                    (VariableName::Filename, "b.rs".into()),
                    (VariableName::RelativeFile, "rust/b.rs".into()),
                    (VariableName::Dirname, "/dir/rust".into()),
                    (VariableName::Stem, "b".into()),
                    (VariableName::WorktreeRoot, "/dir".into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "1".into()),
                ])
            }
        );

        // And now, let's select an identifier.
        editor2.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |selections| selections.select_ranges([14..18]))
        });

        assert_eq!(
            workspace
                .update(cx, |workspace, cx| { task_context(workspace, cx) })
                .await,
            TaskContext {
                cwd: Some("/dir".into()),
                task_variables: TaskVariables::from_iter([
                    (VariableName::File, "/dir/rust/b.rs".into()),
                    (VariableName::Filename, "b.rs".into()),
                    (VariableName::RelativeFile, "rust/b.rs".into()),
                    (VariableName::Dirname, "/dir/rust".into()),
                    (VariableName::Stem, "b".into()),
                    (VariableName::WorktreeRoot, "/dir".into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "15".into()),
                    (VariableName::SelectedText, "is_i".into()),
                    (VariableName::Symbol, "this_is_a_rust_file".into()),
                ])
            }
        );

        assert_eq!(
            workspace
                .update(cx, |workspace, cx| {
                    // Now, let's switch the active item to .ts file.
                    workspace.activate_item(&editor1, cx);
                    task_context(workspace, cx)
                })
                .await,
            TaskContext {
                cwd: Some("/dir".into()),
                task_variables: TaskVariables::from_iter([
                    (VariableName::File, "/dir/a.ts".into()),
                    (VariableName::Filename, "a.ts".into()),
                    (VariableName::RelativeFile, "a.ts".into()),
                    (VariableName::Dirname, "/dir".into()),
                    (VariableName::Stem, "a".into()),
                    (VariableName::WorktreeRoot, "/dir".into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "1".into()),
                    (VariableName::Symbol, "this_is_a_test".into()),
                ])
            }
        );
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            file_icons::init((), cx);
            language::init(cx);
            crate::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            state
        })
    }
}
