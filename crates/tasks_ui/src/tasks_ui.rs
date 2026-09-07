use std::{
    path::Path,
    sync::{Arc, LazyLock},
};

use anyhow::Context as _;
use collections::HashMap;
use editor::{Editor, MultiBufferOffset, ToPoint as _};
use gpui::{
    App, AppContext as _, AsyncWindowContext, Context, Entity, Task, TaskExt, WeakEntity, Window,
};
use project::{Location, TaskContexts, TaskSourceKind, Worktree, WorktreeId};
use task::{RevealTarget, TaskContext, TaskId, TaskTemplate, TaskVariables, VariableName};
use tree_sitter::{Query, StreamingIterator as _};
use util::rel_path::RelPath;
use workspace::Workspace;

mod modal;

pub use modal::{Rerun, ShowAttachModal, Spawn, TaskOverrides, TasksModal};

/// Inserts `new_task` (pretty-printed JSON object text) at the end of the top-level JSON
/// array in the editor's buffer, creating the array if the buffer has none, and moves the
/// cursor to the inserted task. The edit is left unsaved so callers decide whether to persist it.
pub fn insert_task_json_into_editor(
    editor: &mut Editor,
    new_task: String,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> anyhow::Result<()> {
    static LAST_ITEM_QUERY: LazyLock<Query> = LazyLock::new(|| {
        Query::new(
            &tree_sitter_json::LANGUAGE.into(),
            "(document (array (object) @object))", // TODO: use "." anchor to only match last object
        )
        .expect("Failed to create LAST_ITEM_QUERY")
    });
    static EMPTY_ARRAY_QUERY: LazyLock<Query> = LazyLock::new(|| {
        Query::new(
            &tree_sitter_json::LANGUAGE.into(),
            "(document (array) @array)",
        )
        .expect("Failed to create EMPTY_ARRAY_QUERY")
    });

    let content = editor.text(cx);
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into())?;
    let mut cursor = tree_sitter::QueryCursor::new();
    let syntax_tree = parser
        .parse(&content, None)
        .context("could not parse tasks file")?;
    let mut matches = cursor.matches(
        &LAST_ITEM_QUERY,
        syntax_tree.root_node(),
        content.as_bytes(),
    );

    let mut last_offset = None;
    while let Some(mat) = matches.next() {
        if let Some(pos) = mat.captures.first().map(|m| m.node.byte_range().end) {
            last_offset = Some(MultiBufferOffset(pos))
        }
    }
    let mut edits = Vec::new();
    let mut cursor_position = MultiBufferOffset(0);

    if let Some(pos) = last_offset {
        edits.push((pos..pos, format!(",\n{new_task}")));
        cursor_position = pos + ",\n  ".len();
    } else {
        let mut matches = cursor.matches(
            &EMPTY_ARRAY_QUERY,
            syntax_tree.root_node(),
            content.as_bytes(),
        );

        if let Some(mat) = matches.next() {
            if let Some(pos) = mat.captures.first().map(|m| m.node.byte_range().end - 1) {
                edits.push((
                    MultiBufferOffset(pos)..MultiBufferOffset(pos),
                    format!("\n{new_task}\n"),
                ));
                cursor_position = MultiBufferOffset(pos) + "\n  ".len();
            }
        } else {
            edits.push((
                MultiBufferOffset(0)..MultiBufferOffset(0),
                format!("[\n{}\n]", new_task),
            ));
            cursor_position = MultiBufferOffset("[\n  ".len());
        }
    }
    editor.transact(window, cx, |editor, window, cx| {
        editor.edit(edits, cx);
        let snapshot = editor.buffer().read(cx).read(cx);
        let point = cursor_position.to_point(&snapshot);
        drop(snapshot);
        editor.go_to_singleton_buffer_point(point, window, cx);
    });
    Ok(())
}

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
                        spawn_task_or_modal(
                            workspace,
                            &Spawn::ViaModal {
                                reveal_target: None,
                            },
                            window,
                            cx,
                        );
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
    if let Some(provider) = workspace.debugger_provider() {
        provider.spawn_task_or_modal(workspace, action, window, cx);
        return;
    }

    match action {
        Spawn::ByName {
            task_name,
            reveal_target,
        } => {
            spawn_task_by_name(task_name.clone(), *reveal_target, window, cx)
                .detach_and_log_err(cx);
        }
        Spawn::ByTag {
            task_tag,
            reveal_target,
        } => {
            let tag = task_tag.clone();
            spawn_tasks_filtered(
                move |(_, task)| task.tags.contains(&tag),
                *reveal_target,
                window,
                cx,
            )
            .detach_and_log_err(cx)
        }
        Spawn::ViaModal { reveal_target } => {
            toggle_modal(workspace, *reveal_target, window, cx).detach()
        }
    }
}

/// Spawns the nearest applicable task with the given `name`, opening the task
/// picker if no matching task is available.
pub fn spawn_task_by_name(
    name: String,
    reveal_target: Option<RevealTarget>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Task<anyhow::Result<()>> {
    cx.spawn_in(window, async move |workspace, cx| {
        let (task_contexts, tasks) = load_task_candidates(&workspace, cx).await?;
        let active_path =
            cx.update(|_window, cx| task_contexts.file(cx).map(|file| file.path().clone()))?;
        let tasks = select_task_by_name(
            &name,
            task_contexts.worktree(),
            active_path.as_deref(),
            tasks,
        )
        .into_iter()
        .collect();

        if !schedule_tasks(&workspace, tasks, &task_contexts, reveal_target, cx)? {
            workspace
                .update_in(cx, |workspace, window, cx| {
                    spawn_task_or_modal(workspace, &Spawn::ViaModal { reveal_target }, window, cx);
                })
                .ok();
        }

        Ok(())
    })
}

/// Selects the nearest applicable task with the given name.
///
/// Worktree tasks must belong to the active worktree and have a configuration
/// scope containing the active file. When multiple worktree tasks apply, the
/// task from the deepest scope is selected. Without an active file, only a
/// root-scoped worktree task applies.
///
/// If no worktree task applies, the first matching non-worktree candidate is
/// returned, preserving the order provided by `Inventory::list_tasks`.
fn select_task_by_name(
    name: &str,
    worktree_id: Option<WorktreeId>,
    active_path: Option<&RelPath>,
    tasks: Vec<(TaskSourceKind, TaskTemplate)>,
) -> Option<(TaskSourceKind, TaskTemplate)> {
    let mut nearest_worktree_task = None;
    let mut fallback_task = None;

    for (source, template) in tasks {
        if template.label != name {
            continue;
        }

        let TaskSourceKind::Worktree {
            id,
            directory_in_worktree,
            ..
        } = &source
        else {
            if fallback_task.is_none() {
                fallback_task = Some((source, template));
            }
            continue;
        };

        if Some(*id) != worktree_id {
            continue;
        }

        let Some(scope) = directory_in_worktree.parent() else {
            continue;
        };
        let is_applicable = active_path
            .map(|active_path| active_path.starts_with(scope))
            .unwrap_or_else(|| scope.is_empty());
        if !is_applicable {
            continue;
        }

        let scope_depth = scope.len();
        if nearest_worktree_task
            .as_ref()
            .is_none_or(|(nearest_depth, _, _)| scope_depth > *nearest_depth)
        {
            nearest_worktree_task = Some((scope_depth, source, template));
        }
    }

    nearest_worktree_task
        .map(|(_, source, template)| (source, template))
        .or(fallback_task)
}

async fn load_task_candidates(
    workspace: &WeakEntity<Workspace>,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<(TaskContexts, Vec<(TaskSourceKind, TaskTemplate)>)> {
    let task_contexts = workspace
        .update_in(cx, |workspace, window, cx| {
            task_contexts(workspace, window, cx)
        })?
        .await;

    let tasks = workspace
        .update(cx, |workspace, cx| {
            let Some(task_inventory) = workspace
                .project()
                .read(cx)
                .task_store()
                .read(cx)
                .task_inventory()
            else {
                return Task::ready(vec![]);
            };

            let (language, buffer) = task_contexts
                .location()
                .cloned()
                .map(|location| {
                    (
                        location.buffer.read(cx).language_at(location.range.start),
                        Some(location.buffer),
                    )
                })
                .unwrap_or_default();

            task_inventory
                .read(cx)
                .list_tasks(buffer, language, task_contexts.worktree(), cx)
        })?
        .await;

    Ok((task_contexts, tasks))
}

pub fn toggle_modal(
    workspace: &mut Workspace,
    reveal_target: Option<RevealTarget>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Task<()> {
    let task_store = workspace.project().read(cx).task_store().clone();
    let workspace_handle = workspace.weak_handle();
    let can_open_modal = workspace
        .project()
        .read_with(cx, |project, _| !project.is_via_collab());
    if can_open_modal {
        let task_contexts = task_contexts(workspace, window, cx);
        cx.spawn_in(window, async move |workspace, cx| {
            let task_contexts = Arc::new(task_contexts.await);
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        TasksModal::new(
                            task_store.clone(),
                            task_contexts,
                            reveal_target.map(|target| TaskOverrides {
                                reveal_target: Some(target),
                            }),
                            true,
                            workspace_handle,
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

/// Spawns every applicable task matching the `predicate`, opening the task
/// picker if no tasks match.
pub fn spawn_tasks_filtered<F>(
    mut predicate: F,
    reveal_target: Option<RevealTarget>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Task<anyhow::Result<()>>
where
    F: FnMut((&TaskSourceKind, &TaskTemplate)) -> bool + 'static,
{
    cx.spawn_in(window, async move |workspace, cx| {
        let (task_contexts, mut tasks) = load_task_candidates(&workspace, cx).await?;
        tasks.retain(|(task_source_kind, target_task)| predicate((task_source_kind, target_task)));

        if !schedule_tasks(&workspace, tasks, &task_contexts, reveal_target, cx)? {
            workspace
                .update_in(cx, |workspace, window, cx| {
                    spawn_task_or_modal(workspace, &Spawn::ViaModal { reveal_target }, window, cx);
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
                .is_some_and(|worktree| is_visible_directory(&worktree, cx))
        })
        .or_else(|| {
            workspace
                .visible_worktrees(cx)
                .next()
                .map(|tree| tree.read(cx).id())
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
        .map(|active_editor| {
            active_editor.update(cx, |editor, cx| editor.lsp_task_sources(false, false, cx))
        })
        .unwrap_or_default();

    let latest_selection = active_editor.as_ref().and_then(|active_editor| {
        let snapshot = active_editor.read(cx).buffer().read(cx).snapshot(cx);
        snapshot
            .anchor_to_buffer_anchor(active_editor.read(cx).selections.newest_anchor().head())
            .map(|(anchor, _)| anchor)
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

        if let Some(editor_context_task) = editor_context_task
            && let Some(editor_context) = editor_context_task.await
        {
            task_contexts.active_item_context = Some((active_worktree, location, editor_context));
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

/// Schedules the provided tasks and returns whether any task was scheduled.
fn schedule_tasks(
    workspace: &WeakEntity<Workspace>,
    tasks: Vec<(TaskSourceKind, TaskTemplate)>,
    task_contexts: &TaskContexts,
    reveal_target: Option<RevealTarget>,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<bool> {
    if tasks.is_empty() {
        return Ok(false);
    }

    let default_context = TaskContext::default();
    let active_context = task_contexts.active_context().unwrap_or(&default_context);

    workspace.update_in(cx, move |workspace, window, cx| {
        for (task_source_kind, mut task) in tasks {
            if let Some(reveal_target) = reveal_target {
                task.reveal_target = reveal_target;
            }

            workspace.schedule_task(task_source_kind, &task, active_context, false, window, cx)
        }
    })?;

    Ok(true)
}

fn is_visible_directory(worktree: &Entity<Worktree>, cx: &App) -> bool {
    let worktree = worktree.read(cx);
    worktree.is_visible() && worktree.root_entry().is_some_and(|entry| entry.is_dir())
}

fn worktree_context(worktree_abs_path: &Path) -> TaskContext {
    let mut task_variables = TaskVariables::default();
    task_variables.insert(
        VariableName::WorktreeRoot,
        worktree_abs_path.to_string_lossy().into_owned(),
    );
    TaskContext {
        cwd: Some(worktree_abs_path.to_path_buf()),
        task_variables,
        project_env: HashMap::default(),
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::HashMap, sync::Arc};

    use editor::{Editor, MultiBufferOffset, SelectionEffects};
    use gpui::{Entity, TestAppContext, VisualTestContext};
    use language::{Language, LanguageConfig};
    use paths::tasks_file;
    use project::{
        BasicContextProvider, FakeFs, Project, WorktreeId,
        task_inventory::Inventory,
        task_store::{TaskSettingsLocation, TaskStore},
    };
    use serde_json::json;
    use task::{ResolvedTask, TaskContext, TaskTemplate, TaskVariables, VariableName};
    use ui::VisualContext;
    use util::{
        path,
        rel_path::{RelPath, rel_path},
    };
    use workspace::{AppState, MultiWorkspace};

    use crate::{Spawn, TaskSourceKind, select_task_by_name, task_contexts};

    fn take_scheduled_tasks(
        task_inventory: Entity<Inventory>,
        cx: &mut VisualTestContext,
    ) -> Vec<(TaskSourceKind, ResolvedTask)> {
        task_inventory.update(cx, |task_inventory, _cx| {
            let mut scheduled_tasks = Vec::new();

            while let Some((source, task)) = task_inventory.last_scheduled_task(None) {
                task_inventory.delete_previously_used(&task.id);
                scheduled_tasks.push((source, task));
            }

            scheduled_tasks
        })
    }

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
        let (worktree_store, git_store) = project.read_with(cx, |project, _| {
            (project.worktree_store(), project.git_store().clone())
        });
        let rust_language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    ..Default::default()
                },
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
                git_store.clone(),
            )))),
        );

        let typescript_language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "TypeScript".into(),
                    ..Default::default()
                },
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
                git_store.clone(),
            )))),
        );

        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let buffer1 = workspace
            .update(cx, |this, cx| {
                this.project().update(cx, |this, cx| {
                    this.open_buffer((worktree_id, rel_path("a.ts")), cx)
                })
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
                    this.open_buffer((worktree_id, rel_path("rust/b.rs")), cx)
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
                    (VariableName::RelativeFile, path!("rust/b.rs").into()),
                    (VariableName::RelativeDir, "rust".into()),
                    (VariableName::Dirname, path!("/dir/rust").into()),
                    (VariableName::Stem, "b".into()),
                    (VariableName::WorktreeRoot, path!("/dir").into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "1".into()),
                    (VariableName::Language, "Rust".into()),
                ]),
                project_env: HashMap::default(),
            }
        );

        // And now, let's select an identifier.
        editor2.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([MultiBufferOffset(14)..MultiBufferOffset(18)])
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
                    (VariableName::RelativeFile, path!("rust/b.rs").into()),
                    (VariableName::RelativeDir, "rust".into()),
                    (VariableName::Dirname, path!("/dir/rust").into()),
                    (VariableName::Stem, "b".into()),
                    (VariableName::WorktreeRoot, path!("/dir").into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "15".into()),
                    (VariableName::SelectedText, "is_i".into()),
                    (VariableName::Symbol, "this_is_a_rust_file".into()),
                    (VariableName::Language, "Rust".into()),
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
                    (VariableName::RelativeDir, ".".into()),
                    (VariableName::Dirname, path!("/dir").into()),
                    (VariableName::Stem, "a".into()),
                    (VariableName::WorktreeRoot, path!("/dir").into()),
                    (VariableName::Row, "1".into()),
                    (VariableName::Column, "1".into()),
                    (VariableName::Symbol, "this_is_a_test".into()),
                    (VariableName::Language, "TypeScript".into()),
                ]),
                project_env: HashMap::default(),
            }
        );
    }

    #[gpui::test]
    async fn test_spawn_by_name_prefers_worktree_task_over_global_task(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/repo"),
            json!({
                ".zed": {
                    "tasks.json": r#"[
                        {
                            "label": "run_file",
                            "command": "echo",
                            "args": ["worktree"]
                        }
                    ]"#
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/repo").as_ref()], cx).await;
        let task_inventory = project.read_with(cx, |project, cx| {
            project
                .task_store()
                .read(cx)
                .task_inventory()
                .cloned()
                .expect("task inventory should be initialized")
        });

        task_inventory.update(cx, |inventory, _| {
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Global(tasks_file()),
                    Some(
                        &json!([{
                            "label": "run_file",
                            "command": "echo",
                            "args": ["global"]
                        }])
                        .to_string(),
                    ),
                )
                .expect("global task should be valid");
        });

        let (_multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        cx.dispatch_action(Spawn::ByName {
            task_name: "run_file".to_string(),
            reveal_target: None,
        });
        cx.run_until_parked();

        let scheduled_tasks = take_scheduled_tasks(task_inventory, cx);
        assert_eq!(scheduled_tasks.len(), 1);
        assert_eq!(scheduled_tasks[0].1.resolved.args, ["worktree"]);
        assert!(matches!(
            scheduled_tasks[0].0,
            TaskSourceKind::Worktree { .. }
        ));
    }

    #[gpui::test]
    async fn test_spawn_by_name_prefers_task_nearest_to_active_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/repo"),
            json!({
                ".zed": {
                    "tasks.json": r#"[{ "label": "run_file", "command": "echo", "args": ["root"] }]"#
                },
                "project-a": {
                    ".zed": {
                        "tasks.json": r#"[{ "label": "run_file", "command": "echo", "args": ["project-a"] }]"#
                    },
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                },
                "project-b": {
                    ".zed": {
                        "tasks.json": r#"[{ "label": "run_file", "command": "echo", "args": ["project-b"] }]"#
                    }
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/repo").as_ref()], cx).await;
        let task_inventory = project.read_with(cx, |project, cx| {
            project
                .task_store()
                .read(cx)
                .task_inventory()
                .cloned()
                .expect("task inventory should be initialized")
        });
        let worktree_id = project.update(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .expect("project should have a worktree")
                .read(cx)
                .id()
        });
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree_id, rel_path("project-a/src/main.rs")), cx)
            })
            .await
            .expect("active file should open");
        let editor = cx
            .new_window_entity(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_center(Box::new(editor), window, cx);
        });

        cx.dispatch_action(Spawn::ByName {
            task_name: "run_file".to_string(),
            reveal_target: None,
        });
        cx.run_until_parked();

        let scheduled_tasks = take_scheduled_tasks(task_inventory, cx);
        let scheduled_task = scheduled_tasks
            .get(0)
            .expect("should have at least one task");

        assert_eq!(scheduled_tasks.len(), 1);
        assert_eq!(scheduled_task.1.resolved.args, ["project-a"]);
        assert!(matches!(
            &scheduled_task.0,
            TaskSourceKind::Worktree {
                directory_in_worktree,
                ..
            } if directory_in_worktree.as_ref() == rel_path("project-a/.zed")
        ));
    }

    #[test]
    fn test_select_task_by_name_prefers_task_nearest_to_active_file() {
        let name = String::from("echo");
        let worktree_id = WorktreeId::from_usize(1);
        let active_path = RelPath::new_test("project-a/src/main.rs");
        let mut template = TaskTemplate::default();
        template.label = name.clone();

        let tasks = vec![
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    directory_in_worktree: RelPath::new_test(".zed").into_arc(),
                    id_base: Cow::Owned(String::from("id_base")),
                },
                template.clone(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    directory_in_worktree: RelPath::new_test("project-b/.zed").into_arc(),
                    id_base: Cow::Owned(String::from("id_base")),
                },
                template.clone(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    directory_in_worktree: RelPath::new_test("project-a/.zed").into_arc(),
                    id_base: Cow::Owned(String::from("id_base")),
                },
                template,
            ),
        ];

        let task = select_task_by_name(&name, Some(worktree_id), Some(&active_path), tasks)
            .expect("should return a task");
        match task.0 {
            TaskSourceKind::Worktree {
                directory_in_worktree,
                ..
            } => assert_eq!(
                directory_in_worktree,
                RelPath::new_test("project-a/.zed").into_arc()
            ),
            _ => panic!("expected worktree task"),
        }
    }

    #[gpui::test]
    async fn test_spawn_by_tag_schedules_all_matching_tasks(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/repo"),
            json!({
                ".zed": {
                    "tasks.json": r#"[
                        {
                            "label": "echo_worktree_all",
                            "command": "echo",
                            "args": ["worktree"],
                            "tags": ["all"]
                        },
                        {
                            "label": "echo_worktree",
                            "command": "echo",
                            "args": ["worktree"],
                        }
                    ]"#
                }
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/repo").as_ref()], cx).await;
        let task_inventory = project.read_with(cx, |project, cx| {
            project
                .task_store()
                .read(cx)
                .task_inventory()
                .cloned()
                .expect("task inventory should be initialized")
        });

        task_inventory.update(cx, |inventory, _| {
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Global(tasks_file()),
                    Some(
                        &json!([{
                            "label": "echo_global_all",
                            "command": "echo",
                            "args": ["global"],
                            "tags": ["all"]
                        },
                        {
                            "label": "echo_global",
                            "command": "echo",
                            "args": ["global"],
                            "tags": ["not_all"]
                        }
                        ])
                        .to_string(),
                    ),
                )
                .expect("global task should be valid");
        });

        let (_multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        cx.dispatch_action(Spawn::ByTag {
            task_tag: String::from("all"),
            reveal_target: None,
        });
        cx.run_until_parked();

        let scheduled_tasks = take_scheduled_tasks(task_inventory, cx);
        let mut schedule_tasks_labels = scheduled_tasks
            .iter()
            .map(|(_, task)| task.resolved_label.as_str())
            .collect::<Vec<_>>();
        schedule_tasks_labels.sort_unstable();

        assert_eq!(scheduled_tasks.len(), 2);
        assert_eq!(
            schedule_tasks_labels,
            ["echo_global_all", "echo_worktree_all"]
        );
        assert!(
            scheduled_tasks
                .iter()
                .any(|(source, _)| matches!(source, TaskSourceKind::Worktree { .. }))
        );
        assert!(
            scheduled_tasks
                .iter()
                .any(|(source, _)| matches!(source, TaskSourceKind::AbsPath { .. }))
        );
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            TaskStore::init(None);
            state
        })
    }
}
