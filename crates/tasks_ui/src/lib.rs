use std::path::PathBuf;

use gpui::{AppContext, ViewContext, WindowContext};
use modal::TasksModal;
use task::Task;
use util::ResultExt;
use workspace::Workspace;

mod modal;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &modal::Spawn, cx| {
                    let inventory = workspace.project().read(cx).task_inventory().clone();
                    let workspace_handle = workspace.weak_handle();
                    workspace
                        .toggle_modal(cx, |cx| TasksModal::new(inventory, workspace_handle, cx))
                })
                .register_action(move |workspace, _: &modal::Rerun, cx| {
                    if let Some(task) = workspace.project().update(cx, |project, cx| {
                        project
                            .task_inventory()
                            .update(cx, |inventory, cx| inventory.last_scheduled_task(cx))
                    }) {
                        schedule_task(workspace, task.as_ref(), cx)
                    };
                });
        },
    )
    .detach();
}

fn schedule_task(workspace: &Workspace, task: &dyn Task, cx: &mut ViewContext<'_, Workspace>) {
    let cwd = match task.cwd() {
        Some(cwd) => Some(cwd.to_path_buf()),
        None => task_cwd(workspace, cx).log_err().flatten(),
    };
    let spawn_in_terminal = task.exec(cwd);
    if let Some(spawn_in_terminal) = spawn_in_terminal {
        workspace.project().update(cx, |project, cx| {
            project.task_inventory().update(cx, |inventory, _| {
                inventory.task_scheduled(task.id().clone());
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
