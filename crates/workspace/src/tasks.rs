use std::process::ExitStatus;

use anyhow::Result;
use collections::HashSet;
use gpui::{AppContext, Context, Entity, Task};
use language::Buffer;
use project::{TaskSourceKind, WorktreeId};
use remote::ConnectionState;
use task::{
    DebugScenario, ResolvedTask, SaveStrategy, SharedTaskContext, SpawnInTerminal, TaskContext,
    TaskHook, TaskTemplate, TaskVariables, VariableName,
};
use ui::Window;
use util::TryFutureExt;

use crate::{SaveIntent, Toast, Workspace, notifications::NotificationId};

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

        if self.terminal_provider.is_some() {
            let task = cx.spawn_in(window, async move |workspace, cx| {
                let save_action = match spawn_in_terminal.save {
                    SaveStrategy::All => {
                        let save_all = workspace.update_in(cx, |workspace, window, cx| {
                            let task = workspace.save_all_internal(SaveIntent::SaveAll, window, cx);
                            // Match the type of the other arm by ignoring the bool value returned
                            cx.background_spawn(async { task.await.map(|_| ()) })
                        });
                        save_all.ok()
                    }
                    SaveStrategy::Current => {
                        let save_current = workspace.update_in(cx, |workspace, window, cx| {
                            workspace.save_active_item(SaveIntent::SaveAll, window, cx)
                        });
                        save_current.ok()
                    }
                    SaveStrategy::None => None,
                };
                if let Some(save_action) = save_action {
                    save_action.log_err().await;
                }

                let spawn_task = workspace.update_in(cx, |workspace, window, cx| {
                    workspace
                        .terminal_provider
                        .as_ref()
                        .map(|terminal_provider| {
                            terminal_provider.spawn(spawn_in_terminal, window, cx)
                        })
                });
                if let Some(spawn_task) = spawn_task.ok().flatten() {
                    let res = cx.background_spawn(spawn_task).await;
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
                            _ = workspace.update(cx, |w, cx| {
                                let id = NotificationId::unique::<ResolvedTask>();
                                w.show_toast(Toast::new(id, format!("Task spawn failed: {e}")), cx);
                            })
                        }
                        None => log::debug!("Task spawn got cancelled"),
                    };
                }
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

    pub fn run_create_worktree_tasks(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let project = self.project().clone();
        let hooks = HashSet::from_iter([TaskHook::CreateWorktree]);

        let worktree_tasks: Vec<(WorktreeId, TaskContext, Vec<TaskTemplate>)> = {
            let project = project.read(cx);
            let task_store = project.task_store();
            let Some(inventory) = task_store.read(cx).task_inventory().cloned() else {
                return;
            };

            let git_store = project.git_store().read(cx);

            let mut worktree_tasks = Vec::new();
            for worktree in project.worktrees(cx) {
                let worktree = worktree.read(cx);
                let worktree_id = worktree.id();
                let worktree_abs_path = worktree.abs_path();

                let templates: Vec<TaskTemplate> = inventory
                    .read(cx)
                    .templates_with_hooks(&hooks, worktree_id)
                    .into_iter()
                    .map(|(_, template)| template)
                    .collect();

                if templates.is_empty() {
                    continue;
                }

                let mut task_variables = TaskVariables::default();
                task_variables.insert(
                    VariableName::WorktreeRoot,
                    worktree_abs_path.to_string_lossy().into_owned(),
                );

                if let Some(path) = git_store.original_repo_path_for_worktree(worktree_id, cx) {
                    task_variables.insert(
                        VariableName::MainGitWorktree,
                        path.to_string_lossy().into_owned(),
                    );
                }

                let task_context = TaskContext {
                    cwd: Some(worktree_abs_path.to_path_buf()),
                    task_variables,
                    project_env: Default::default(),
                };

                worktree_tasks.push((worktree_id, task_context, templates));
            }
            worktree_tasks
        };

        if worktree_tasks.is_empty() {
            return;
        }

        let task = cx.spawn_in(window, async move |workspace, cx| {
            let mut tasks = Vec::new();
            for (worktree_id, task_context, templates) in worktree_tasks {
                let id_base = format!("worktree_setup_{worktree_id}");

                tasks.push(cx.spawn({
                    let workspace = workspace.clone();
                    async move |cx| {
                        for task_template in templates {
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
        dirty_before_spawn: Arc<Mutex<Option<bool>>>,
    }

    #[gpui::test]
    async fn test_schedule_resolved_task_save_all(cx: &mut TestAppContext) {
        let (fixture, cx) = create_fixture(cx, SaveStrategy::All).await;
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

        assert_eq!(*fixture.dirty_before_spawn.lock(), Some(false));
        assert!(cx.read(|cx| !fixture.item.read(cx).is_dirty));
    }

    #[gpui::test]
    async fn test_schedule_resolved_task_save_current(cx: &mut TestAppContext) {
        let (fixture, cx) = create_fixture(cx, SaveStrategy::Current).await;
        // Add a second inactive dirty item
        let inactive = add_test_item(&fixture.workspace, "file2.txt", false, cx);
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

        // The active item (fixture.item) should be saved
        assert_eq!(*fixture.dirty_before_spawn.lock(), Some(false));
        assert!(cx.read(|cx| !fixture.item.read(cx).is_dirty));
        // The inactive item should not be saved
        assert!(cx.read(|cx| inactive.read(cx).is_dirty));
    }

    #[gpui::test]
    async fn test_schedule_resolved_task_save_none(cx: &mut TestAppContext) {
        let (fixture, cx) = create_fixture(cx, SaveStrategy::None).await;
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

        assert_eq!(*fixture.dirty_before_spawn.lock(), Some(true));
        assert!(cx.read(|cx| fixture.item.read(cx).is_dirty));
    }

    async fn create_fixture(
        cx: &mut TestAppContext,
        save_strategy: SaveStrategy,
    ) -> (Fixture, &mut gpui::VisualTestContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            register_serializable_item::<TestItem>(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({ "file.txt": "dirty" }))
            .await;
        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // Add a dirty item to the workspace
        let item = add_test_item(&workspace, "file.txt", true, cx);

        let template = TaskTemplate {
            label: "test".to_string(),
            command: "echo".to_string(),
            save: save_strategy,
            ..Default::default()
        };
        let task = template
            .resolve_task("test", &task::TaskContext::default())
            .unwrap();
        let dirty_before_spawn: Arc<Mutex<Option<bool>>> = Arc::default();
        let terminal_provider = Box::new(TestTerminalProvider {
            item: item.clone(),
            dirty_before_spawn: dirty_before_spawn.clone(),
        });
        workspace.update(cx, |workspace, _| {
            workspace.terminal_provider = Some(terminal_provider);
        });
        let fixture = Fixture {
            workspace,
            item,
            task,
            dirty_before_spawn,
        };
        (fixture, cx)
    }

    fn add_test_item(
        workspace: &Entity<Workspace>,
        name: &str,
        active: bool,
        cx: &mut gpui::VisualTestContext,
    ) -> Entity<TestItem> {
        let item = cx.new(|cx| {
            TestItem::new(cx)
                .with_dirty(true)
                .with_project_items(&[TestProjectItem::new(1, name, cx)])
        });
        workspace.update_in(cx, |workspace, window, cx| {
            let pane = workspace.active_pane().clone();
            workspace.add_item(pane, Box::new(item.clone()), None, true, active, window, cx);
        });
        item
    }

    struct TestTerminalProvider {
        item: Entity<TestItem>,
        dirty_before_spawn: Arc<Mutex<Option<bool>>>,
    }

    impl TerminalProvider for TestTerminalProvider {
        fn spawn(
            &self,
            _task: task::SpawnInTerminal,
            _window: &mut ui::Window,
            cx: &mut App,
        ) -> Task<Option<Result<ExitStatus>>> {
            *self.dirty_before_spawn.lock() = Some(cx.read_entity(&self.item, |e, _| e.is_dirty));
            Task::ready(Some(Ok(ExitStatus::default())))
        }
    }
}
