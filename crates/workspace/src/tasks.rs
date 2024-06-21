use project::TaskSourceKind;
use task::{ResolvedTask, TaskContext, TaskTemplate};
use ui::ViewContext;

use crate::Workspace;

pub fn schedule_task(
    workspace: &Workspace,
    task_source_kind: TaskSourceKind,
    task_to_resolve: &TaskTemplate,
    task_cx: &TaskContext,
    omit_history: bool,
    cx: &mut ViewContext<'_, Workspace>,
) {
    if let Some(spawn_in_terminal) =
        task_to_resolve.resolve_task(&task_source_kind.to_id_base(), task_cx)
    {
        schedule_resolved_task(
            workspace,
            task_source_kind,
            spawn_in_terminal,
            omit_history,
            cx,
        );
    }
}

pub fn schedule_resolved_task(
    workspace: &Workspace,
    task_source_kind: TaskSourceKind,
    mut resolved_task: ResolvedTask,
    omit_history: bool,
    cx: &mut ViewContext<'_, Workspace>,
) {
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
}
