use futures::TryFutureExt;
use gpui::Task;
use project::TaskSourceKind;
use task::{ResolvedTask, TaskContext, TaskTemplate};
use ui::ViewContext;

use crate::workspace_settings::WorkspaceSettings;
use crate::Workspace;
use settings::Settings;

pub fn schedule_task(
    task_source_kind: TaskSourceKind,
    task_to_resolve: &TaskTemplate,
    task_cx: &TaskContext,
    omit_history: bool,
    cx: &mut ViewContext<'_, Workspace>,
) {
    if let Some(spawn_in_terminal) =
        task_to_resolve.resolve_task(&task_source_kind.to_id_base(), task_cx)
    {
        schedule_resolved_task(task_source_kind, spawn_in_terminal, omit_history, cx);
    }
}

pub fn schedule_resolved_task(
    task_source_kind: TaskSourceKind,
    mut resolved_task: ResolvedTask,
    omit_history: bool,
    cx: &mut ViewContext<'_, Workspace>,
) {
    cx.spawn(|workspace, mut cx| async move {
        cx.update(|cx| {
            workspace
                .update(cx, |workspace, cx| {
                    WorkspaceSettings::register(cx);
                    if WorkspaceSettings::get_global(cx).autosave_before_task {
                        workspace.save_all_internal(crate::SaveIntent::SaveAll, cx)
                    } else {
                        Task::ready(Ok(false))
                    }
                })
                .unwrap()
        })
        .unwrap()
        .and_then(|_| async {
            if let Some(workspace) = workspace.upgrade() {
                cx.update(|cx| {
                    workspace.update(cx, |workspace, cx| {
                        if let Some(spawn_in_terminal) = resolved_task.resolved.take() {
                            if !omit_history {
                                resolved_task.resolved = Some(spawn_in_terminal.clone());
                                workspace.project().update(cx, |project, cx| {
                                    project.task_inventory().update(cx, |inventory, _| {
                                        inventory.task_scheduled(task_source_kind, resolved_task);
                                    })
                                });
                            }
                            cx.emit(crate::Event::SpawnTask(spawn_in_terminal));
                        }
                    })
                })
            } else {
                Ok(())
            }
        })
        .await
    })
    .detach_and_log_err(cx)
}
