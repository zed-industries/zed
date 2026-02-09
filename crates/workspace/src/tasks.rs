use std::process::ExitStatus;

use anyhow::Result;
use gpui::{AppContext, Context, Entity, Task};
use language::Buffer;
use project::{TaskSourceKind, WorktreeId};
use remote::ConnectionState;
use settings::{SaveBeforeTaskRun, Settings};
use task::{
    DebugScenario, ResolvedTask, SharedTaskContext, SpawnInTerminal, TaskContext, TaskTemplate,
};
use ui::Window;
use util::TryFutureExt;

use crate::{SaveIntent, Toast, Workspace, WorkspaceSettings, notifications::NotificationId};

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

        if let Some(terminal_provider) = self.terminal_provider.clone() {
            let save_before_task_run = WorkspaceSettings::get_global(cx).save_before_task_run;
            let task = cx.spawn_in(window, async move |w, cx| {
                let res = async {
                    match save_before_task_run {
                        SaveBeforeTaskRun::Never => (),
                        SaveBeforeTaskRun::Always => {
                            let save_all = w.update_in(cx, |workspace, win, cx| {
                                workspace.save_all_internal(SaveIntent::SaveAll, win, cx)
                            });
                            save_all.ok()?.log_err().await;
                        }
                    }
                    let task_status = w.update_in(cx, |_, win, cx| {
                        terminal_provider.spawn(spawn_in_terminal, win, cx)
                    });
                    cx.background_spawn(task_status.ok()?).await
                }
                .await;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        TerminalProvider,
        item::test::{TestItem, TestProjectItem},
        register_serializable_item,
    };
    use gpui::{App, TestAppContext};
    use parking_lot::Mutex;
    use project::{FakeFs, Project, TaskSourceKind};
    use serde_json::json;
    use std::sync::Arc;
    use task::TaskTemplate;

    struct Fixture {
        workspace: Entity<Workspace>,
        item: Entity<TestItem>,
        task: ResolvedTask,
        terminal_provider: Arc<TestTerminalProvider>,
    }

    #[gpui::test]
    async fn test_schedule_resolved_task_without_save(cx: &mut TestAppContext) {
        let (fixture, cx) = create_fixture(cx, SaveBeforeTaskRun::Never).await;
        fixture.workspace.update_in(cx, |workspace, window, cx| {
            workspace.schedule_resolved_task(
                TaskSourceKind::UserInput,
                fixture.task,
                false,
                window,
                cx,
            );
        });
        cx.executor().run_until_parked();

        assert_eq!(*fixture.terminal_provider.was_dirty.lock(), Some(true));
        assert!(cx.read(|cx| fixture.item.read(cx).is_dirty));
    }

    #[gpui::test]
    async fn test_schedule_resolved_task_with_save(cx: &mut TestAppContext) {
        let (fixture, cx) = create_fixture(cx, SaveBeforeTaskRun::Always).await;
        fixture.workspace.update_in(cx, |workspace, window, cx| {
            workspace.schedule_resolved_task(
                TaskSourceKind::UserInput,
                fixture.task,
                false,
                window,
                cx,
            );
        });
        cx.executor().run_until_parked();

        // Should have saved the item before spawning the task
        assert_eq!(*fixture.terminal_provider.was_dirty.lock(), Some(false));
        assert!(cx.read(|cx| !fixture.item.read(cx).is_dirty));
    }

    async fn create_fixture(
        cx: &mut TestAppContext,
        save_before_task_run: SaveBeforeTaskRun,
    ) -> (Fixture, &mut gpui::VisualTestContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            register_serializable_item::<TestItem>(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({ "file.txt": "dirty" }))
            .await;
        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        cx.update_global(|settings: &mut settings::SettingsStore, cx| {
            settings.update_user_settings(cx, |settings| {
                settings.workspace.save_before_task_run = Some(save_before_task_run);
            });
        });

        // Add a dirty item to the workspace
        let item = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, "file.txt", cx)])
        });
        workspace.update_in(cx, |workspace, window, cx| {
            let pane = workspace.active_pane().clone();
            workspace.add_item(pane, Box::new(item.clone()), None, true, true, window, cx);
        });

        let template = TaskTemplate {
            label: "test".to_string(),
            command: "echo".to_string(),
            ..Default::default()
        };
        let task = template
            .resolve_task("test", &task::TaskContext::default())
            .unwrap();
        let terminal_provider = Arc::new(TestTerminalProvider {
            item: item.clone(),
            was_dirty: Mutex::default(),
        });
        workspace.update(cx, |workspace, _| {
            workspace.terminal_provider = Some(terminal_provider.clone());
        });
        let fixture = Fixture {
            workspace,
            item,
            task,
            terminal_provider,
        };
        (fixture, cx)
    }

    struct TestTerminalProvider {
        item: Entity<TestItem>,
        was_dirty: Mutex<Option<bool>>,
    }

    impl TerminalProvider for TestTerminalProvider {
        fn spawn(
            &self,
            _task: task::SpawnInTerminal,
            _window: &mut ui::Window,
            cx: &mut App,
        ) -> Task<Option<Result<ExitStatus>>> {
            *self.was_dirty.lock() = Some(cx.read_entity(&self.item, |e, _| e.is_dirty));
            Task::ready(Some(Ok(ExitStatus::default())))
        }
    }
}
