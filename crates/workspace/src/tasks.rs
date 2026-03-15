use std::process::ExitStatus;

use anyhow::Result;
use gpui::{AppContext, Context, Entity, Task};
use language::Buffer;
use project::{TaskSourceKind, WorktreeId};
use remote::ConnectionState;
use task::{
    DebugScenario, ResolvedTask, SharedTaskContext, SpawnInTerminal, TaskContext, TaskTemplate,
    TaskVariables, VariableName, WorktreeTaskDefinition,
};
use ui::Window;

use crate::{Toast, Workspace, notifications::NotificationId};

impl Workspace {
    pub fn schedule_task(
        self: &mut Workspace,
        task_source_kind: TaskSourceKind,
        task_to_resolve: &TaskTemplate,
        task_cx: &TaskContext,
        omit_history: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.project.read(cx).remote_connection_state(cx) {
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
            self.schedule_resolved_task(
                task_source_kind,
                spawn_in_terminal,
                omit_history,
                window,
                cx,
            );
        }
    }

    pub fn schedule_resolved_task(
        self: &mut Workspace,
        task_source_kind: TaskSourceKind,
        resolved_task: ResolvedTask,
        omit_history: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let spawn_in_terminal = resolved_task.resolved.clone();
        if !omit_history {
            if let Some(debugger_provider) = self.debugger_provider.as_ref() {
                debugger_provider.task_scheduled(cx);
            }

            self.project().update(cx, |project, cx| {
                if let Some(task_inventory) =
                    project.task_store().read(cx).task_inventory().cloned()
                {
                    task_inventory.update(cx, |inventory, _| {
                        inventory.task_scheduled(task_source_kind, resolved_task);
                    })
                }
            });
        }

        if let Some(terminal_provider) = self.terminal_provider.as_ref() {
            let task_status = terminal_provider.spawn(spawn_in_terminal, window, cx);

            let task = cx.spawn(async |w, cx| {
                let res = cx.background_spawn(task_status).await;
                match res {
                    Some(Ok(status)) => {
                        if status.success() {
                            log::debug!("Task spawn succeeded");
                        } else {
                            log::debug!("Task spawn failed, code: {:?}", status.code());
                        }
                    }
                    Some(Err(e)) => {
                        log::error!("Task spawn failed: {e:#}");
                        _ = w.update(cx, |w, cx| {
                            let id = NotificationId::unique::<ResolvedTask>();
                            w.show_toast(Toast::new(id, format!("Task spawn failed: {e}")), cx);
                        })
                    }
                    None => log::debug!("Task spawn got cancelled"),
                };
            });
            self.scheduled_tasks.push(task);
        }
    }

    pub fn start_debug_session(
        &mut self,
        scenario: DebugScenario,
        task_context: SharedTaskContext,
        active_buffer: Option<Entity<Buffer>>,
        worktree_id: Option<WorktreeId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(provider) = self.debugger_provider.as_mut() {
            provider.start_session(
                scenario,
                task_context,
                active_buffer,
                worktree_id,
                window,
                cx,
            )
        }
    }

    pub fn spawn_in_terminal(
        self: &mut Workspace,
        spawn_in_terminal: SpawnInTerminal,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Option<Result<ExitStatus>>> {
        if let Some(terminal_provider) = self.terminal_provider.as_ref() {
            terminal_provider.spawn(spawn_in_terminal, window, cx)
        } else {
            Task::ready(None)
        }
    }

    pub fn run_git_worktree_tasks(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        dbg!("In run git worktree tasks");
        let project = self.project().clone();

        let worktree_tasks: Vec<(WorktreeId, TaskContext, Vec<WorktreeTaskDefinition>)> = {
            let project = project.read(cx);
            let task_store = project.task_store();
            let Some(inventory) = task_store.read(cx).task_inventory().cloned() else {
                dbg!("No task inventory");
                return;
            };

            let mut worktree_tasks = Vec::new();
            for worktree in project.worktrees(cx) {
                let worktree = worktree.read(cx);
                let worktree_id = worktree.id();
                let worktree_abs_path = worktree.abs_path();

                let definitions: Vec<WorktreeTaskDefinition> = inventory
                    .read(cx)
                    .list_git_worktree_scripts(worktree_id)
                    .into_iter()
                    .flat_map(|(_, scripts)| scripts.setup)
                    .collect();

                if definitions.is_empty() {
                    dbg!("Task inventory has no definitions");
                    continue;
                }

                let mut task_variables = TaskVariables::default();
                task_variables.insert(
                    VariableName::WorktreeRoot,
                    worktree_abs_path.to_string_lossy().into_owned(),
                );
                let task_context = TaskContext {
                    cwd: Some(worktree_abs_path.to_path_buf()),
                    task_variables,
                    project_env: Default::default(),
                };

                worktree_tasks.push((worktree_id, task_context, definitions));
            }
            worktree_tasks
        };

        if worktree_tasks.is_empty() {
            dbg!("worktree tasks is empty");
            return;
        }

        let inventory = project
            .read(cx)
            .task_store()
            .read(cx)
            .task_inventory()
            .cloned();

        let task = cx.spawn_in(window, async move |workspace, cx| {
            let mut tasks = Vec::new();
            for (worktree_id, task_context, definitions) in worktree_tasks {
                let id_base = format!("worktree_setup_{worktree_id}");
                dbg!("getting running", definitions.len());

                tasks.push(cx.spawn({
                    let workspace = workspace.clone();
                    let inventory = inventory.clone();
                    async move |cx| {
                        for definition in definitions {
                            let task_template = match definition {
                                WorktreeTaskDefinition::Template { task_template } => task_template,
                                WorktreeTaskDefinition::ByName(label) => {
                                    let Some(ref inventory) = inventory else {
                                        continue;
                                    };
                                    let lookup = inventory.read_with(cx, |inventory, cx| {
                                        inventory.task_template_by_label(
                                            None,
                                            Some(worktree_id),
                                            &label,
                                            cx,
                                        )
                                    });
                                    match lookup.await {
                                        Some(template) => template,
                                        None => {
                                            log::warn!(
                                                "Could not find task template named '{label}' \
                                                 for git worktree setup"
                                            );
                                            continue;
                                        }
                                    }
                                }
                            };

                            let Some(resolved) =
                                task_template.resolve_task(&id_base, &task_context)
                            else {
                                continue;
                            };

                            let status = workspace.update_in(cx, |workspace, window, cx| {
                                workspace.spawn_in_terminal(resolved.resolved, window, cx)
                            })?;

                            if let Some(result) = status.await {
                                match result {
                                    Ok(exit_status) if !exit_status.success() => {
                                        log::error!(
                                            "Git worktree setup task failed with status: {:?}",
                                            exit_status.code()
                                        );
                                        break;
                                    }
                                    Err(error) => {
                                        log::error!("Git worktree setup task error: {error:#}");
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        anyhow::Ok(())
                    }
                }));
            }

            futures::future::join_all(tasks).await;
            anyhow::Ok(())
        });
        task.detach_and_log_err(cx);
    }
}
