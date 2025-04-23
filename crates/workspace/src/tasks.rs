use std::process::ExitStatus;

use anyhow::{Result, anyhow};
use gpui::{Context, Task};
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
        mut resolved_task: ResolvedTask,
        omit_history: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(spawn_in_terminal) = resolved_task.resolved.take() {
            if !omit_history {
                resolved_task.resolved = Some(spawn_in_terminal.clone());
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
                terminal_provider
                    .spawn(spawn_in_terminal, window, cx)
                    .detach_and_log_err(cx);
            }
        }
    }

    pub fn start_debug_session(
        &mut self,
        definition: DebugScenario,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(provider) = self.debugger_provider.as_mut() {
            provider.start_session(definition, window, cx)
        }
    }

    pub fn spawn_in_terminal(
        self: &mut Workspace,
        spawn_in_terminal: SpawnInTerminal,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<ExitStatus>> {
        if let Some(terminal_provider) = self.terminal_provider.as_ref() {
            terminal_provider.spawn(spawn_in_terminal, window, cx)
        } else {
            Task::ready(Err(anyhow!("No terminal provider")))
        }
    }
}
