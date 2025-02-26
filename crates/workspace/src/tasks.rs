use anyhow::anyhow;
use gpui::Context;
use itertools::Itertools as _;
use project::TaskSourceKind;
use remote::ConnectionState;
use smol::channel::bounded;
use task::{ResolvedTask, TaskContext, TaskTemplate};
use util::ResultExt as _;

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

        let mut all_tasks = pre_task_queue
            .into_iter()
            .filter_map(|mut task| task.resolved.take())
            .collect_vec();

        all_tasks.push(spawn_in_terminal);

        cx.spawn(|workspace, mut cx| async move {
            let mut task_iter = all_tasks.into_iter();
            let mut failed_task_info: Option<(String, i32)> = None;

            for task in task_iter.by_ref() {
                let label = task.full_label.clone();
                let (tx, rx) = bounded(1);

                workspace.update(&mut cx, move |_, cx| {
                    cx.emit(crate::Event::SpawnTask {
                        action: Box::new(task),
                        completion_tx: Box::new(tx)
                    });
                })?;

                let exit_code = rx
                    .recv()
                    .await
                    .anyhow()??;

                if exit_code != 0 {
                    failed_task_info = Some((label, exit_code));
                    break;
                }
            };

            if let Some((label, exit_code)) = failed_task_info {
                let n_remaining = task_iter.count();
                Err(anyhow!(
                    "Task '{label}' exited with non-zero exit code ({exit_code}), aborting {n_remaining} remaining tasks",
                ))
            } else {
                Ok(())
            }
        }).detach_and_log_err(cx);
    }
}
