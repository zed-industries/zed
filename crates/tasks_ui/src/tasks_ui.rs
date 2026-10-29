use std::{
    path::Path,
    sync::{Arc, LazyLock},
};

use anyhow::Context as _;
use collections::HashMap;
use editor::{Editor, MultiBufferOffset, ToPoint as _};
use gpui::{App, AppContext as _, Context, Entity, Task, TaskExt, Window};
use project::{Location, TaskContexts, TaskSourceKind, Worktree};
use task::{RevealTarget, TaskContext, TaskId, TaskTemplate, TaskVariables, VariableName};
use tree_sitter::{Query, StreamingIterator as _};
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
        if let Some(pos) = mat.captures().first().map(|m| m.node.byte_range().end) {
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
            if let Some(pos) = mat.captures().first().map(|m| m.node.byte_range().end - 1) {
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
                                        let task_context = task_contexts
                                            .context_for_task(&task_source_kind, &original_task)
                                            .unwrap_or(&default_context);
                                        workspace.schedule_task(
                                            task_source_kind,
                                            &original_task,
                                            task_context,
                                            false,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok()
                            })
                            .detach();
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
        Spawn::ViaModal { reveal_target } => {
            toggle_modal(workspace, *reveal_target, window, cx).detach()
        }
    }
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

pub fn spawn_tasks_filtered<F>(
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
        let mut tasks = workspace
            .update(cx, |workspace, cx| {
                let Some(task_inventory) = workspace
                    .project()
                    .read(cx)
                    .task_store()
                    .read(cx)
                    .task_inventory()
                    .cloned()
                else {
                    return Task::ready(Vec::new());
                };
                let (language, buffer) = task_contexts
                    .location()
                    .map(|location| {
                        let buffer = location.buffer.clone();
                        (
                            buffer.read(cx).language_at(location.range.start),
                            Some(buffer),
                        )
                    })
                    .unwrap_or_default();
                task_inventory
                    .read(cx)
                    .list_tasks(buffer, language, task_contexts.worktree(), cx)
            })?
            .await;

        let did_spawn = workspace
            .update_in(cx, |workspace, window, cx| {
                let default_context = TaskContext::default();

                tasks.retain_mut(|(task_source_kind, target_task)| {
                    if predicate((task_source_kind, target_task)) {
                        if let Some(overrides) = &overrides
                            && let Some(target_override) = overrides.reveal_target
                        {
                            target_task.reveal_target = target_override;
                        }
                        let task_context = task_contexts
                            .context_for_task(task_source_kind, target_task)
                            .unwrap_or(&default_context);

                        workspace.schedule_task(
                            task_source_kind.clone(),
                            target_task,
                            task_context,
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
    let active_item_project_path = active_item.as_ref().and_then(|item| item.project_path(cx));
    let active_item_worktree = active_item_project_path
        .as_ref()
        .map(|project_path| project_path.worktree_id)
        .filter(|worktree_id| {
            workspace
                .project()
                .read(cx)
                .worktree_for_id(*worktree_id, cx)
                .is_some_and(|worktree| is_visible_directory(&worktree, cx))
        });
    let active_worktree = active_item_worktree.or_else(|| {
        workspace
            .visible_worktrees(cx)
            .next()
            .map(|tree| tree.read(cx).id())
    });

    let active_editor = active_item.and_then(|item| item.act_as::<Editor>(cx));
    // Files from single-file worktrees provide no worktree context, but their editors
    // must still provide file variables such as `ZED_FILE`.
    let active_project_editor = active_item_project_path
        .is_some()
        .then(|| active_editor.clone())
        .flatten();

    let editor_context_task = active_project_editor.as_ref().map(|active_editor| {
        active_editor.update(cx, |editor, cx| editor.task_context(window, cx))
    });

    let location = active_project_editor.as_ref().and_then(|editor| {
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

    let lsp_task_sources = active_project_editor
        .as_ref()
        .map(|active_editor| {
            active_editor.update(cx, |editor, cx| editor.lsp_task_sources(false, false, cx))
        })
        .unwrap_or_default();

    let latest_selection = active_project_editor.as_ref().and_then(|active_editor| {
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
            && let Some(mut editor_context) = editor_context_task.await
        {
            if active_item_worktree.is_none()
                && let Some(project_root) =
                    active_worktree.and_then(|worktree_id| worktree_abs_paths.get(&worktree_id))
            {
                use_project_root_for_external_item(&mut editor_context, project_root);
            }
            task_contexts.active_item_context =
                Some((active_item_worktree, location, editor_context));
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

/// Items outside of visible directory worktrees (e.g. the global `tasks.json` opened via
/// `zed: open tasks`) keep their file variables, but use the visible project's root and cwd.
/// Everything else derived from their own worktree is dropped: relative paths would point into
/// an unrelated directory, and its environment would override the one the terminal loads for
/// the project's cwd.
fn use_project_root_for_external_item(editor_context: &mut TaskContext, project_root: &Path) {
    editor_context.task_variables.insert(
        VariableName::WorktreeRoot,
        project_root.to_string_lossy().into_owned(),
    );
    editor_context
        .task_variables
        .remove(&VariableName::RelativeFile);
    editor_context
        .task_variables
        .remove(&VariableName::RelativeDir);
    editor_context.cwd = Some(project_root.to_path_buf());
    editor_context.project_env.clear();
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
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
        sync::Arc,
    };

    use editor::{Editor, MultiBufferOffset, SelectionEffects};
    use gpui::{Entity, TestAppContext, VisualTestContext};
    use language::{Language, LanguageConfig};
    use project::{
        BasicContextProvider, FakeFs, Inventory, Project, TaskContexts, TaskSourceKind, WorktreeId,
        task_store::{TaskSettingsLocation, TaskStore},
    };
    use serde_json::json;
    use task::{ResolvedTask, TaskContext, TaskTemplate, TaskVariables, VariableName};
    use ui::VisualContext;
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, MultiWorkspace, OpenOptions, OpenVisible, Workspace};

    use crate::{
        Rerun, Spawn, task_contexts, use_project_root_for_external_item, worktree_context,
    };

    const GLOBAL_TASKS_JSON: &str = r#"[
        {
            "label": "print global task context",
            "command": "echo",
            "args": ["$ZED_WORKTREE_ROOT", "$ZED_FILENAME", "${ZED_RELATIVE_FILE:no file}"]
        }
    ]"#;

    #[gpui::test]
    async fn test_non_project_active_editor_uses_visible_worktree_context(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let worktree_root = path!("/worktrees/Godot Projects/sample-game");
        fs.insert_tree(
            worktree_root,
            json!({
                ".zed": {
                    "tasks.json": "[]",
                },
                "scenes": {
                    "main_menu.tscn": "",
                }
            }),
        )
        .await;

        let project = Project::test(fs, [worktree_root.as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let global_tasks_editor = cx.new_window_entity(|window, cx| Editor::multi_line(window, cx));

        let contexts = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.add_item_to_center(Box::new(global_tasks_editor), window, cx);
                task_contexts(workspace, window, cx)
            })
            .await;

        assert!(contexts.active_item_context.is_none());
        assert_eq!(
            contexts
                .active_context()
                .expect("visible worktree should provide an active task context"),
            &TaskContext {
                cwd: Some(worktree_root.into()),
                task_variables: TaskVariables::from_iter([(
                    VariableName::WorktreeRoot,
                    worktree_root.into(),
                )]),
                project_env: HashMap::default(),
            }
        );
    }

    #[gpui::test]
    async fn test_single_file_worktree_provides_file_context(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(path!("/dir"), json!({"test.py": "print(1)"}))
            .await;

        let project = Project::test(fs, [path!("/dir/test.py").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/test.py"), cx)
            })
            .await
            .unwrap();
        let editor = cx
            .new_window_entity(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));

        let contexts = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.add_item_to_center(Box::new(editor), window, cx);
                task_contexts(workspace, window, cx)
            })
            .await;

        assert!(contexts.active_worktree_context.is_none());
        let (worktree_id, _, item_context) = contexts
            .active_item_context
            .as_ref()
            .expect("single-file editor should provide an item context");

        assert_eq!(*worktree_id, None);
        assert_eq!(item_context.cwd, None);
        assert_eq!(
            item_context.task_variables.get(&VariableName::File),
            Some(path!("/dir/test.py"))
        );
        assert_eq!(
            item_context.task_variables.get(&VariableName::WorktreeRoot),
            None
        );
    }

    #[gpui::test]
    async fn test_spawn_by_name_resolves_worktree_task_with_single_file_editor_active(
        cx: &mut TestAppContext,
    ) {
        let (_, workspace, cx) = init_project_with_worktree_task(cx).await;
        open_external_file_in_single_file_worktree(&workspace, cx).await;

        assert_task_spawned_by_name_in_project_root(&workspace, cx);
    }

    #[gpui::test]
    async fn test_invisible_directory_worktree_editor_keeps_its_worktree_context(
        cx: &mut TestAppContext,
    ) {
        let (project, workspace, cx) = init_project_with_worktree_task(cx).await;
        open_file_in_invisible_worktree(&project, &workspace, path!("/external"), "notes.txt", cx)
            .await;

        let editor = workspace
            .read_with(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
            .expect("active item should be an editor");

        // Gutter tasks resolve against this context directly, without the workspace's
        // fallback to the visible project, so it must describe the editor's own worktree.
        let item_context = editor
            .update_in(cx, |editor, window, cx| editor.task_context(window, cx))
            .await
            .expect("editor should provide a task context");

        assert_eq!(
            item_context.task_variables.get(&VariableName::RelativeFile),
            Some("notes.txt")
        );
        assert_eq!(
            item_context.task_variables.get(&VariableName::RelativeDir),
            Some(".")
        );
        assert_eq!(
            item_context.cwd.as_deref(),
            Some(Path::new(path!("/external")))
        );
    }

    #[gpui::test]
    async fn test_spawn_by_name_resolves_worktree_task_with_invisible_worktree_editor_active(
        cx: &mut TestAppContext,
    ) {
        let (project, workspace, cx) = init_project_with_worktree_task(cx).await;

        open_file_in_invisible_worktree(&project, &workspace, path!("/external"), "notes.txt", cx)
            .await;

        assert_task_spawned_by_name_in_project_root(&workspace, cx);
    }

    #[gpui::test]
    async fn test_reevaluated_rerun_resolves_worktree_task_with_single_file_editor_active(
        cx: &mut TestAppContext,
    ) {
        let (_, workspace, cx) = init_project_with_worktree_task(cx).await;
        spawn_task_with_project_file_active(&workspace, cx).await;
        open_external_file_in_single_file_worktree(&workspace, cx).await;

        assert_reevaluated_rerun_in_project_root(&workspace, cx);
    }

    #[gpui::test]
    async fn test_reevaluated_rerun_resolves_worktree_task_with_invisible_worktree_editor_active(
        cx: &mut TestAppContext,
    ) {
        let (project, workspace, cx) = init_project_with_worktree_task(cx).await;
        spawn_task_with_project_file_active(&workspace, cx).await;
        open_file_in_invisible_worktree(&project, &workspace, path!("/external"), "notes.txt", cx)
            .await;

        assert_reevaluated_rerun_in_project_root(&workspace, cx);
    }

    #[gpui::test]
    async fn test_spawn_by_name_resolves_global_task_with_global_tasks_file_editor_active(
        cx: &mut TestAppContext,
    ) {
        let (project, workspace, cx) = init_project_with_worktree_task(cx).await;
        task_inventory(&workspace, cx)
            .update(cx, |inventory, _| {
                inventory.update_file_based_tasks(
                    TaskSettingsLocation::Global(Path::new(path!("/zed-config/tasks.json"))),
                    Some(GLOBAL_TASKS_JSON),
                )
            })
            .unwrap();

        open_file_in_invisible_worktree(
            &project,
            &workspace,
            path!("/zed-config"),
            "tasks.json",
            cx,
        )
        .await;

        cx.dispatch_action(Spawn::ByName {
            task_name: "print global task context".to_string(),
            reveal_target: None,
        });
        cx.run_until_parked();

        let scheduled_task = last_scheduled_task(&workspace, cx);
        assert_eq!(
            scheduled_task.resolved.args,
            vec![
                path!("/project").to_string(),
                "tasks.json".to_string(),
                "no file".to_string(),
            ]
        );
        assert_eq!(
            scheduled_task.resolved.cwd.as_deref(),
            Some(Path::new(path!("/project")))
        );
    }

    #[test]
    fn test_external_item_context_does_not_carry_its_worktree_environment() {
        let project_root = Path::new(path!("/project"));
        let project_worktree_id = WorktreeId::from_usize(1);

        let mut external_item_context = TaskContext {
            cwd: Some(PathBuf::from(path!("/zed-config"))),
            task_variables: TaskVariables::from_iter([
                (
                    VariableName::File,
                    path!("/zed-config/tasks.json").to_string(),
                ),
                (VariableName::Filename, "tasks.json".to_string()),
                (VariableName::WorktreeRoot, path!("/zed-config").to_string()),
                (VariableName::RelativeFile, "tasks.json".to_string()),
                (VariableName::RelativeDir, ".".to_string()),
            ]),
            project_env: HashMap::from_iter([
                ("SOURCE_DIRECTORY".to_string(), "zed-config".to_string()),
                ("ZED_CONFIG_ONLY".to_string(), "1".to_string()),
            ]),
        };

        use_project_root_for_external_item(&mut external_item_context, project_root);

        let mut project_worktree_context = worktree_context(project_root);
        project_worktree_context.project_env =
            HashMap::from_iter([("SOURCE_DIRECTORY".to_string(), "project".to_string())]);

        let task_contexts = TaskContexts {
            active_item_context: Some((None, None, external_item_context)),
            active_worktree_context: Some((project_worktree_id, project_worktree_context)),
            ..TaskContexts::default()
        };

        let global_task = task_contexts
            .resolve_task(
                &TaskSourceKind::AbsPath {
                    id_base: "global tasks".into(),
                    abs_path: PathBuf::from(path!("/zed-config/tasks.json")),
                },
                &TaskTemplate {
                    label: "print global task context".to_string(),
                    command: "echo".to_string(),
                    args: vec![
                        "$ZED_WORKTREE_ROOT".to_string(),
                        "$ZED_FILENAME".to_string(),
                        "${ZED_RELATIVE_FILE:no file}".to_string(),
                    ],
                    ..TaskTemplate::default()
                },
            )
            .expect("global task should resolve");

        assert_eq!(
            global_task.resolved.args,
            vec![
                path!("/project").to_string(),
                "tasks.json".to_string(),
                "no file".to_string()
            ]
        );
        assert_eq!(global_task.resolved.cwd.as_deref(), Some(project_root));
        assert_eq!(global_task.resolved.env.get("ZED_CONFIG_ONLY"), None);
        assert_eq!(global_task.resolved.env.get("SOURCE_DIRECTORY"), None);

        let worktree_task = task_contexts
            .resolve_task(
                &TaskSourceKind::Worktree {
                    id: project_worktree_id,
                    directory_in_worktree: rel_path(".zed").into(),
                    id_base: "local worktree tasks".into(),
                },
                &TaskTemplate {
                    label: "print worktree root".to_string(),
                    command: "echo".to_string(),
                    args: vec!["$ZED_WORKTREE_ROOT".to_string()],
                    ..TaskTemplate::default()
                },
            )
            .expect("worktree task should resolve");

        assert_eq!(worktree_task.resolved.cwd.as_deref(), Some(project_root));
        assert_eq!(
            worktree_task
                .resolved
                .env
                .get("SOURCE_DIRECTORY")
                .map(String::as_str),
            Some("project")
        );
    }

    async fn open_external_file_in_single_file_worktree(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) {
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/external/notes.txt")),
                    OpenOptions {
                        visible: Some(OpenVisible::None),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
    }

    async fn open_file_in_invisible_worktree(
        project: &Entity<Project>,
        workspace: &Entity<Workspace>,
        worktree_path: &str,
        file_name: &str,
        cx: &mut VisualTestContext,
    ) {
        let (worktree, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(worktree_path, false, cx)
            })
            .await
            .unwrap();
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
        cx.run_until_parked();

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path(file_name)), None, true, window, cx)
            })
            .await
            .unwrap();
    }

    async fn spawn_task_with_project_file_active(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) {
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/project/main.rs")),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        cx.dispatch_action(Spawn::ByName {
            task_name: "print worktree root and file".to_string(),
            reveal_target: None,
        });
        cx.run_until_parked();

        assert_eq!(
            last_scheduled_task(workspace, cx).resolved.args,
            vec![path!("/project").to_string(), "main.rs".to_string()]
        );
    }

    async fn init_project_with_worktree_task(
        cx: &mut TestAppContext,
    ) -> (Entity<Project>, Entity<Workspace>, &mut VisualTestContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".zed": {
                    "tasks.json": r#"[
                        {
                            "label": "print worktree root",
                            "command": "echo",
                            "args": ["$ZED_WORKTREE_ROOT"]
                        },
                        {
                            "label": "print worktree root and file",
                            "command": "echo",
                            "args": ["$ZED_WORKTREE_ROOT", "${ZED_RELATIVE_FILE:no file}"]
                        }
                    ]"#,
                },
                "main.rs": "",
            }),
        )
        .await;
        fs.insert_tree(path!("/external"), json!({"notes.txt": ""}))
            .await;

        fs.insert_tree(
            path!("/zed-config"),
            json!({ "tasks.json": GLOBAL_TASKS_JSON }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        (project, workspace, cx)
    }

    fn assert_task_spawned_by_name_in_project_root(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) {
        cx.dispatch_action(Spawn::ByName {
            task_name: "print worktree root".to_string(),
            reveal_target: None,
        });
        cx.run_until_parked();

        let scheduled_task = last_scheduled_task(workspace, cx);
        assert_eq!(
            scheduled_task.resolved.args,
            vec![path!("/project").to_string()]
        );
        assert_eq!(
            scheduled_task.resolved.cwd.as_deref(),
            Some(Path::new(path!("/project")))
        );
    }

    fn assert_reevaluated_rerun_in_project_root(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) {
        cx.dispatch_action(Rerun {
            reevaluate_context: true,
            ..Rerun::default()
        });
        cx.run_until_parked();

        let scheduled_task = last_scheduled_task(workspace, cx);
        assert_eq!(
            scheduled_task.resolved.args,
            vec![path!("/project").to_string(), "no file".to_string()]
        );
        assert_eq!(
            scheduled_task.resolved.cwd.as_deref(),
            Some(Path::new(path!("/project")))
        );
    }

    fn task_inventory(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Inventory> {
        workspace
            .read_with(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .task_store()
                    .read(cx)
                    .task_inventory()
                    .cloned()
            })
            .expect("project should have a task inventory")
    }

    fn last_scheduled_task(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> ResolvedTask {
        let (_, scheduled_task) = task_inventory(workspace, cx)
            .read_with(cx, |inventory, _| inventory.last_scheduled_task(None))
            .expect("a task should be scheduled");
        scheduled_task
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
