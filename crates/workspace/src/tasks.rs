use project::TaskSourceKind;
use remote::ConnectionState;
use task::{NewCenterTask, ResolvedTask, TaskContext, TaskTemplate};
use ui::ViewContext;
use zed_actions::TaskSpawnTarget;

use crate::Workspace;

pub fn schedule_task(
    workspace: &mut Workspace,
    task_source_kind: TaskSourceKind,
    task_to_resolve: &TaskTemplate,
    task_cx: &TaskContext,
    task_target: zed_actions::TaskSpawnTarget,
    omit_history: bool,
    cx: &mut ViewContext<'_, Workspace>,
) {
    match workspace.project.read(cx).ssh_connection_state(cx) {
        None | Some(ConnectionState::Connected) => {}
        Some(
            ConnectionState::Connecting
            | ConnectionState::Disconnected
            | ConnectionState::HeartbeatMissed
            | ConnectionState::Reconnecting,
        ) => {
            log::warn!("Cannot schedule tasks when disconnected from a remote host");
            return;
        }
    }

    if let Some(spawn_in_terminal) =
        task_to_resolve.resolve_task(&task_source_kind.to_id_base(), task_target, task_cx)
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
    workspace: &mut Workspace,
    task_source_kind: TaskSourceKind,
    mut resolved_task: ResolvedTask,
    omit_history: bool,
    cx: &mut ViewContext<'_, Workspace>,
) {
    let target = resolved_task.target;
    if let Some(spawn_in_terminal) = resolved_task.resolved.take() {
        if !omit_history {
            resolved_task.resolved = Some(spawn_in_terminal.clone());
            workspace.project().update(cx, |project, cx| {
                if let Some(task_inventory) =
                    project.task_store().read(cx).task_inventory().cloned()
                {
                    task_inventory.update(cx, |inventory, _| {
                        inventory.task_scheduled(task_source_kind, resolved_task);
                    })
                }
            });
        }

        match target {
            TaskSpawnTarget::Center => {
                cx.dispatch_action(Box::new(NewCenterTask {
                    action: spawn_in_terminal,
                }));
            }
            TaskSpawnTarget::Dock => {
                cx.emit(crate::Event::SpawnTask {
                    action: Box::new(spawn_in_terminal),
                });
            }
        }
    }
}
