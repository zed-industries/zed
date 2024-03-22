use std::path::PathBuf;

use editor::Editor;
use gpui::{AppContext, ViewContext, WindowContext};
use language::Point;
use modal::{Spawn, TasksModal};
use project::{Location, WorktreeId};
use task::{Task, TaskContext, TaskVariables};
use util::ResultExt;
use workspace::Workspace;

mod modal;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace
                .register_action(spawn_task_or_modal)
                .register_action(move |workspace, action: &modal::Rerun, cx| {
                    if let Some((task, old_context)) =
                        workspace.project().update(cx, |project, cx| {
                            project
                                .task_inventory()
                                .update(cx, |inventory, cx| inventory.last_scheduled_task(cx))
                        })
                    {
                        let task_context = if action.reevaluate_context {
                            let cwd = task_cwd(workspace, cx).log_err().flatten();
                            task_context(workspace, cwd, cx)
                        } else {
                            old_context
                        };

                        schedule_task(workspace, task.as_ref(), task_context, cx)
                    };
                });
        },
    )
    .detach();
}

fn spawn_task_or_modal(workspace: &mut Workspace, action: &Spawn, cx: &mut ViewContext<Workspace>) {
    let inventory = workspace.project().read(cx).task_inventory().clone();
    let workspace_handle = workspace.weak_handle();
    let cwd = task_cwd(workspace, cx).log_err().flatten();
    let task_context = task_context(workspace, cwd, cx);
    if let Some(name) = action.task_name.clone() {
        // Do not actually show the modal.
        spawn_task_with_name(name.clone(), cx);
    } else {
        workspace.toggle_modal(cx, |cx| {
            TasksModal::new(inventory, task_context, workspace_handle, cx)
        })
    }
}

fn spawn_task_with_name(name: String, cx: &mut ViewContext<Workspace>) {
    cx.spawn(|workspace, mut cx| async move {
        let did_spawn = workspace
            .update(&mut cx, |this, cx| {
                let active_item = this
                    .active_item(cx)
                    .and_then(|item| item.project_path(cx))
                    .map(|path| path.worktree_id);
                let tasks = this.project().update(cx, |project, cx| {
                    project.task_inventory().update(cx, |inventory, cx| {
                        inventory.list_tasks(None, active_item, false, cx)
                    })
                });
                let (_, target_task) = tasks.into_iter().find(|(_, task)| task.name() == name)?;
                let cwd = task_cwd(this, cx).log_err().flatten();
                let task_context = task_context(this, cwd, cx);
                schedule_task(this, target_task.as_ref(), task_context, cx);
                Some(())
            })
            .ok()
            .flatten()
            .is_some();
        if !did_spawn {
            workspace
                .update(&mut cx, |workspace, cx| {
                    spawn_task_or_modal(workspace, &Spawn::default(), cx);
                })
                .ok();
        }
    })
    .detach();
}

fn task_context(
    workspace: &Workspace,
    cwd: Option<PathBuf>,
    cx: &mut WindowContext<'_>,
) -> TaskContext {
    let current_editor = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
        .clone();
    if let Some(current_editor) = current_editor {
        (|| {
            let editor = current_editor.read(cx);
            let selection = editor.selections.newest::<usize>(cx);
            let (buffer, _, _) = editor
                .buffer()
                .read(cx)
                .point_to_buffer_offset(selection.start, cx)?;

            current_editor.update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx);
                let selection_range = selection.range();
                let start = snapshot
                    .display_snapshot
                    .buffer_snapshot
                    .anchor_after(selection_range.start)
                    .text_anchor;
                let end = snapshot
                    .display_snapshot
                    .buffer_snapshot
                    .anchor_after(selection_range.end)
                    .text_anchor;
                let Point { row, column } = snapshot
                    .display_snapshot
                    .buffer_snapshot
                    .offset_to_point(selection_range.start);
                let row = row + 1;
                let column = column + 1;
                let location = Location {
                    buffer: buffer.clone(),
                    range: start..end,
                };

                let current_file = location
                    .buffer
                    .read(cx)
                    .file()
                    .and_then(|file| file.as_local())
                    .map(|file| file.abs_path(cx).to_string_lossy().to_string());
                let worktree_id = location
                    .buffer
                    .read(cx)
                    .file()
                    .map(|file| WorktreeId::from_usize(file.worktree_id()));
                let context = buffer
                    .read(cx)
                    .language()
                    .and_then(|language| language.context_provider())
                    .and_then(|provider| provider.build_context(location, cx).ok());

                let worktree_path = worktree_id.and_then(|worktree_id| {
                    workspace
                        .project()
                        .read(cx)
                        .worktree_for_id(worktree_id, cx)
                        .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().to_string())
                });

                let selected_text = buffer.read(cx).chars_for_range(selection_range).collect();

                let mut task_variables = TaskVariables::from_iter([
                    ("ZED_ROW".into(), row.to_string()),
                    ("ZED_COLUMN".into(), column.to_string()),
                    ("ZED_SELECTED_TEXT".into(), selected_text),
                ]);
                if let Some(path) = current_file {
                    task_variables.0.insert("ZED_FILE".into(), path);
                }
                if let Some(worktree_path) = worktree_path {
                    task_variables
                        .0
                        .insert("ZED_WORKTREE_ROOT".into(), worktree_path);
                }
                if let Some(language_context) = context {
                    task_variables.0.extend(language_context.0);
                }

                Some(TaskContext {
                    cwd: cwd.clone(),
                    task_variables,
                })
            })
        })()
        .unwrap_or_else(|| TaskContext {
            cwd,
            task_variables: Default::default(),
        })
    } else {
        TaskContext {
            cwd,
            task_variables: Default::default(),
        }
    }
}

fn schedule_task(
    workspace: &Workspace,
    task: &dyn Task,
    task_cx: TaskContext,
    cx: &mut ViewContext<'_, Workspace>,
) {
    let spawn_in_terminal = task.exec(task_cx.clone());
    if let Some(spawn_in_terminal) = spawn_in_terminal {
        workspace.project().update(cx, |project, cx| {
            project.task_inventory().update(cx, |inventory, _| {
                inventory.task_scheduled(task.id().clone(), task_cx);
            })
        });
        cx.emit(workspace::Event::SpawnTask(spawn_in_terminal));
    }
}

fn task_cwd(workspace: &Workspace, cx: &mut WindowContext) -> anyhow::Result<Option<PathBuf>> {
    let project = workspace.project().read(cx);
    let available_worktrees = project
        .worktrees()
        .filter(|worktree| {
            let worktree = worktree.read(cx);
            worktree.is_visible()
                && worktree.is_local()
                && worktree.root_entry().map_or(false, |e| e.is_dir())
        })
        .collect::<Vec<_>>();
    let cwd = match available_worktrees.len() {
        0 => None,
        1 => Some(available_worktrees[0].read(cx).abs_path()),
        _ => {
            let cwd_for_active_entry = project.active_entry().and_then(|entry_id| {
                available_worktrees.into_iter().find_map(|worktree| {
                    let worktree = worktree.read(cx);
                    if worktree.contains_entry(entry_id) {
                        Some(worktree.abs_path())
                    } else {
                        None
                    }
                })
            });
            anyhow::ensure!(
                cwd_for_active_entry.is_some(),
                "Cannot determine task cwd for multiple worktrees"
            );
            cwd_for_active_entry
        }
    };
    Ok(cwd.map(|path| path.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use editor::Editor;
    use gpui::{Entity, TestAppContext};
    use language::{Language, LanguageConfig, SymbolContextProvider};
    use project::{FakeFs, Project, TaskSourceKind};
    use serde_json::json;
    use task::{oneshot_source::OneshotSource, TaskContext, TaskVariables};
    use ui::VisualContext;
    use workspace::{AppState, Workspace};

    use crate::{task_context, task_cwd};

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
            .with_context_provider(Some(Arc::new(SymbolContextProvider))),
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
            .with_context_provider(Some(Arc::new(SymbolContextProvider))),
        );
        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.update(cx, |project, cx| {
            project.task_inventory().update(cx, |inventory, cx| {
                inventory.add_source(TaskSourceKind::UserInput, |cx| OneshotSource::new(cx), cx)
            })
        });
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
        workspace.update(cx, |this, cx| {
            this.add_item_to_center(Box::new(editor1.clone()), cx);
            this.add_item_to_center(Box::new(editor2.clone()), cx);
            assert_eq!(this.active_item(cx).unwrap().item_id(), editor2.entity_id());
            assert_eq!(
                task_context(this, task_cwd(this, cx).unwrap(), cx),
                TaskContext {
                    cwd: Some("/dir".into()),
                    task_variables: TaskVariables::from_iter([
                        ("ZED_FILE".into(), "/dir/rust/b.rs".into()),
                        ("ZED_WORKTREE_ROOT".into(), "/dir".into()),
                        ("ZED_ROW".into(), "1".into()),
                        ("ZED_COLUMN".into(), "1".into()),
                        ("ZED_SELECTED_TEXT".into(), "".into())
                    ])
                }
            );
            // And now, let's select an identifier.
            editor2.update(cx, |this, cx| {
                this.change_selections(None, cx, |selections| selections.select_ranges([14..18]))
            });
            assert_eq!(
                task_context(this, task_cwd(this, cx).unwrap(), cx),
                TaskContext {
                    cwd: Some("/dir".into()),
                    task_variables: TaskVariables::from_iter([
                        ("ZED_FILE".into(), "/dir/rust/b.rs".into()),
                        ("ZED_WORKTREE_ROOT".into(), "/dir".into()),
                        ("ZED_SYMBOL".into(), "this_is_a_rust_file".into()),
                        ("ZED_ROW".into(), "1".into()),
                        ("ZED_COLUMN".into(), "15".into()),
                        ("ZED_SELECTED_TEXT".into(), "is_i".into()),
                    ])
                }
            );

            // Now, let's switch the active item to .ts file.
            this.activate_item(&editor1, cx);
            assert_eq!(
                task_context(this, task_cwd(this, cx).unwrap(), cx),
                TaskContext {
                    cwd: Some("/dir".into()),
                    task_variables: TaskVariables::from_iter([
                        ("ZED_FILE".into(), "/dir/a.ts".into()),
                        ("ZED_WORKTREE_ROOT".into(), "/dir".into()),
                        ("ZED_SYMBOL".into(), "this_is_a_test".into()),
                        ("ZED_ROW".into(), "1".into()),
                        ("ZED_COLUMN".into(), "1".into()),
                        ("ZED_SELECTED_TEXT".into(), "".into()),
                    ])
                }
            );
        });
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            language::init(cx);
            crate::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            state
        })
    }
}
