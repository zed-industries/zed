use gpui::Context;
use itertools::Itertools as _;
use project::TaskSourceKind;
use remote::ConnectionState;
use task::{ResolvedTask, TaskContext, TaskTemplate};

use crate::{notifications::NotifyResultExt as _, Workspace};

pub fn schedule_task(
    workspace: &mut Workspace,
    task_source_kind: TaskSourceKind,
    task_to_resolve: &TaskTemplate,
    task_cx: &TaskContext,
    omit_history: bool,
    cx: &mut Context<Workspace>,
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
        task_to_resolve.resolve_task(&task_source_kind.to_id_base(), task_cx)
    {
        let inventory = workspace
            .project()
            .read(cx)
            .task_store()
            .read(cx)
            .task_inventory();

        let Some(inventory) = inventory else {
            return;
        };

        let pre_tasks = inventory
            .read(cx)
            .resolve_file_based_task_queue(&spawn_in_terminal, &task_source_kind, task_cx)
            .notify_err(workspace, cx)
            .unwrap_or(vec![])
            .into_iter()
            .map(|(_, task)| task)
            .collect_vec();

        schedule_resolved_tasks(
            workspace,
            task_source_kind,
            pre_tasks,
            spawn_in_terminal,
            omit_history,
            cx,
        );
    }
}

pub fn schedule_resolved_tasks(
    workspace: &mut Workspace,
    task_source_kind: TaskSourceKind,
    pre_task_queue: Vec<ResolvedTask>,
    mut resolved_task: ResolvedTask,
    omit_history: bool,
    cx: &mut Context<Workspace>,
) {
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

        let pre_tasks = pre_task_queue
            .into_iter()
            .filter_map(|mut task| task.resolved.take())
            .collect_vec();

        cx.emit(crate::Event::SpawnTask {
            pre_actions: pre_tasks,
            action: Box::new(spawn_in_terminal),
        });
    }
}
