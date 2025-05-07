use std::process::ExitStatus;

use anyhow::Result;
use gpui::{AppContext, Context, Entity, Task};
use language::Buffer;
use project::TaskSourceKind;
use remote::ConnectionState;
use task::{DebugScenario, ResolvedTask, SpawnInTerminal, TaskContext, TaskTemplate};
use ui::Window;

use crate::Workspace;

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
        match self.project.read(cx).ssh_connection_state(cx) {
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
            cx.background_spawn(async move {
                match task_status.await {
                    Some(Ok(status)) => {
                        if status.success() {
                            log::debug!("Task spawn succeeded");
                        } else {
                            log::debug!("Task spawn failed, code: {:?}", status.code());
                        }
                    }
                    Some(Err(e)) => log::error!("Task spawn failed: {e}"),
                    None => log::debug!("Task spawn got cancelled"),
                }
            })
            .detach();
        }
    }

    pub fn start_debug_session(
        &mut self,
        scenario: DebugScenario,
        task_context: TaskContext,
        active_buffer: Option<Entity<Buffer>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(provider) = self.debugger_provider.as_mut() {
            provider.start_session(scenario, task_context, active_buffer, window, cx)
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
}
