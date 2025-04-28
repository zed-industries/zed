use crate::persistence::DebuggerPaneItem;
use crate::{
    ClearAllBreakpoints, Continue, CreateDebuggingSession, Disconnect, FocusBreakpointList,
    FocusConsole, FocusFrames, FocusLoadedSources, FocusModules, FocusTerminal, FocusVariables,
    Pause, Restart, StepBack, StepInto, StepOut, StepOver, Stop, ToggleIgnoreBreakpoints,
    persistence,
};
use crate::{new_session_modal::NewSessionModal, session::DebugSession};
use anyhow::{Result, anyhow};
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use dap::DebugRequest;
use dap::{
    ContinuedEvent, LoadedSourceEvent, ModuleEvent, OutputEvent, StoppedEvent, ThreadEvent,
    client::SessionId, debugger_settings::DebuggerSettings,
};
use dap::{StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use futures::{SinkExt as _, channel::mpsc};
use gpui::{
    Action, App, AsyncWindowContext, Context, DismissEvent, Entity, EntityId, EventEmitter,
    FocusHandle, Focusable, MouseButton, MouseDownEvent, Point, Subscription, Task, WeakEntity,
    actions, anchored, deferred,
};

use language::Buffer;
use project::debugger::session::{Session, SessionStateEvent};
use project::{
    Project,
    debugger::{
        dap_store::{self, DapStore},
        session::ThreadStatus,
    },
    terminals::TerminalKind,
};
use rpc::proto::{self};
use settings::Settings;
use std::any::TypeId;
use std::path::Path;
use std::sync::Arc;
use task::{DebugScenario, HideStrategy, RevealStrategy, RevealTarget, TaskContext, TaskId};
use terminal_view::TerminalView;
use ui::{ContextMenu, Divider, DropdownMenu, Tooltip, prelude::*};
use workspace::SplitDirection;
use workspace::{
    Pane, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

pub enum DebugPanelEvent {
    Exited(SessionId),
    Terminated(SessionId),
    Stopped {
        client_id: SessionId,
        event: StoppedEvent,
        go_to_stack_frame: bool,
    },
    Thread((SessionId, ThreadEvent)),
    Continued((SessionId, ContinuedEvent)),
    Output((SessionId, OutputEvent)),
    Module((SessionId, ModuleEvent)),
    LoadedSource((SessionId, LoadedSourceEvent)),
    ClientShutdown(SessionId),
    CapabilitiesChanged(SessionId),
}

actions!(debug_panel, [ToggleFocus]);
pub struct DebugPanel {
    size: Pixels,
    sessions: Vec<Entity<DebugSession>>,
    active_session: Option<Entity<DebugSession>>,
    /// This represents the last debug definition that was created in the new session modal
    pub(crate) past_debug_definition: Option<DebugTaskDefinition>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    _subscriptions: Vec<Subscription>,
}

impl DebugPanel {
    pub fn new(
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let project = workspace.project().clone();
            let dap_store = project.read(cx).dap_store();

            let _subscriptions =
                vec![cx.subscribe_in(&dap_store, window, Self::handle_dap_store_event)];

            let debug_panel = Self {
                size: px(300.),
                sessions: vec![],
                active_session: None,
                _subscriptions,
                past_debug_definition: None,
                focus_handle: cx.focus_handle(),
                project,
                workspace: workspace.weak_handle(),
                context_menu: None,
            };

            debug_panel
        })
    }

    fn filter_action_types(&self, cx: &mut App) {
        let (has_active_session, supports_restart, support_step_back, status) = self
            .active_session()
            .map(|item| {
                let running = item.read(cx).mode().as_running().cloned();

                match running {
                    Some(running) => {
                        let caps = running.read(cx).capabilities(cx);
                        (
                            !running.read(cx).session().read(cx).is_terminated(),
                            caps.supports_restart_request.unwrap_or_default(),
                            caps.supports_step_back.unwrap_or_default(),
                            running.read(cx).thread_status(cx),
                        )
                    }
                    None => (false, false, false, None),
                }
            })
            .unwrap_or((false, false, false, None));

        let filter = CommandPaletteFilter::global_mut(cx);
        let debugger_action_types = [
            TypeId::of::<Disconnect>(),
            TypeId::of::<Stop>(),
            TypeId::of::<ToggleIgnoreBreakpoints>(),
        ];

        let running_action_types = [TypeId::of::<Pause>()];

        let stopped_action_type = [
            TypeId::of::<Continue>(),
            TypeId::of::<StepOver>(),
            TypeId::of::<StepInto>(),
            TypeId::of::<StepOut>(),
            TypeId::of::<editor::actions::DebuggerRunToCursor>(),
            TypeId::of::<editor::actions::DebuggerEvaluateSelectedText>(),
        ];

        let step_back_action_type = [TypeId::of::<StepBack>()];
        let restart_action_type = [TypeId::of::<Restart>()];

        if has_active_session {
            filter.show_action_types(debugger_action_types.iter());

            if supports_restart {
                filter.show_action_types(restart_action_type.iter());
            } else {
                filter.hide_action_types(&restart_action_type);
            }

            if support_step_back {
                filter.show_action_types(step_back_action_type.iter());
            } else {
                filter.hide_action_types(&step_back_action_type);
            }

            match status {
                Some(ThreadStatus::Running) => {
                    filter.show_action_types(running_action_types.iter());
                    filter.hide_action_types(&stopped_action_type);
                }
                Some(ThreadStatus::Stopped) => {
                    filter.show_action_types(stopped_action_type.iter());
                    filter.hide_action_types(&running_action_types);
                }
                _ => {
                    filter.hide_action_types(&running_action_types);
                    filter.hide_action_types(&stopped_action_type);
                }
            }
        } else {
            // show only the `debug: start`
            filter.hide_action_types(&debugger_action_types);
            filter.hide_action_types(&step_back_action_type);
            filter.hide_action_types(&restart_action_type);
            filter.hide_action_types(&running_action_types);
            filter.hide_action_types(&stopped_action_type);
        }
    }

    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            workspace.update_in(cx, |workspace, window, cx| {
                let debug_panel = DebugPanel::new(workspace, window, cx);

                workspace.register_action(|workspace, _: &ClearAllBreakpoints, _, cx| {
                    workspace.project().read(cx).breakpoint_store().update(
                        cx,
                        |breakpoint_store, cx| {
                            breakpoint_store.clear_breakpoints(cx);
                        },
                    )
                });

                cx.observe_new::<DebugPanel>(|debug_panel, _, cx| {
                    Self::filter_action_types(debug_panel, cx);
                })
                .detach();

                cx.observe(&debug_panel, |_, debug_panel, cx| {
                    debug_panel.update(cx, |debug_panel, cx| {
                        Self::filter_action_types(debug_panel, cx);
                    });
                })
                .detach();
                workspace.set_debugger_provider(DebuggerProvider(debug_panel.clone()));

                debug_panel
            })
        })
    }

    fn start_from_definition(
        &mut self,
        definition: DebugTaskDefinition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        cx.spawn_in(window, async move |this, cx| {
            let dap_store = this.update(cx, |this, cx| this.project.read(cx).dap_store())?;
            let (session, task) = dap_store.update(cx, |dap_store, cx| {
                let session = dap_store.new_session(definition, None, cx);

                (session.clone(), dap_store.boot_session(session, cx))
            })?;
            Self::register_session(this.clone(), session.clone(), cx).await?;

            if let Err(e) = task.await {
                this.update(cx, |this, cx| {
                    this.workspace
                        .update(cx, |workspace, cx| {
                            workspace.show_error(&e, cx);
                        })
                        .ok();
                })
                .ok();

                session
                    .update(cx, |session, cx| session.shutdown(cx))?
                    .await;
            }

            anyhow::Ok(())
        })
    }

    pub fn start_session(
        &mut self,
        scenario: DebugScenario,
        task_context: TaskContext,
        active_buffer: Option<Entity<Buffer>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.spawn_in(window, async move |this, cx| {
            let definition = this
                .update_in(cx, |this, window, cx| {
                    this.resolve_scenario(scenario, task_context, active_buffer, window, cx)
                })?
                .await?;
            this.update_in(cx, |this, window, cx| {
                this.start_from_definition(definition, window, cx)
            })?
            .await
        })
        .detach_and_log_err(cx);
    }

    async fn register_session(
        this: WeakEntity<Self>,
        session: Entity<Session>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let adapter_name = session.update(cx, |session, _| session.adapter_name())?;
        this.update_in(cx, |_, window, cx| {
            cx.subscribe_in(
                &session,
                window,
                move |_, session, event: &SessionStateEvent, window, cx| match event {
                    SessionStateEvent::Restart => {
                        let mut curr_session = session.clone();
                        while let Some(parent_session) = curr_session
                            .read_with(cx, |session, _| session.parent_session().cloned())
                        {
                            curr_session = parent_session;
                        }

                        let definition = curr_session.update(cx, |session, _| session.definition());
                        let task = curr_session.update(cx, |session, cx| session.shutdown(cx));

                        cx.spawn_in(window, async move |this, cx| {
                            task.await;

                            this.update_in(cx, |this, window, cx| {
                                this.start_from_definition(definition, window, cx)
                            })?
                            .await
                        })
                        .detach_and_log_err(cx);
                    }
                    _ => {}
                },
            )
            .detach();
        })
        .ok();

        let serialized_layout = persistence::get_serialized_pane_layout(adapter_name).await;

        let workspace = this.update_in(cx, |this, window, cx| {
            this.sessions.retain(|session| {
                session
                    .read(cx)
                    .mode()
                    .as_running()
                    .map_or(false, |running_state| {
                        !running_state.read(cx).session().read(cx).is_terminated()
                    })
            });

            let session_item = DebugSession::running(
                this.project.clone(),
                this.workspace.clone(),
                session,
                cx.weak_entity(),
                serialized_layout,
                window,
                cx,
            );

            if let Some(running) = session_item.read(cx).mode().as_running().cloned() {
                // We might want to make this an event subscription and only notify when a new thread is selected
                // This is used to filter the command menu correctly
                cx.observe(&running, |_, _, cx| cx.notify()).detach();
            }

            this.sessions.push(session_item.clone());
            this.activate_session(session_item, window, cx);
            this.workspace.clone()
        })?;

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.focus_panel::<Self>(window, cx);
        })?;
        Ok(())
    }

    pub fn start_child_session(
        &mut self,
        request: &StartDebuggingRequestArguments,
        parent_session: Entity<Session>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(worktree) = parent_session.read(cx).worktree() else {
            log::error!("Attempted to start a child session from non local debug session");
            return;
        };

        let dap_store_handle = self.project.read(cx).dap_store().clone();
        let breakpoint_store = self.project.read(cx).breakpoint_store();
        let definition = parent_session.read(cx).definition().clone();
        let mut binary = parent_session.read(cx).binary().clone();
        binary.request_args = request.clone();

        cx.spawn_in(window, async move |this, cx| {
            let (session, task) = dap_store_handle.update(cx, |dap_store, cx| {
                let session =
                    dap_store.new_session(definition.clone(), Some(parent_session.clone()), cx);

                let task = session.update(cx, |session, cx| {
                    session.boot(
                        binary,
                        worktree,
                        breakpoint_store,
                        dap_store_handle.downgrade(),
                        cx,
                    )
                });
                (session, task)
            })?;

            match task.await {
                Err(e) => {
                    this.update(cx, |this, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                workspace.show_error(&e, cx);
                            })
                            .ok();
                    })
                    .ok();

                    session
                        .update(cx, |session, cx| session.shutdown(cx))?
                        .await;
                }
                Ok(_) => Self::register_session(this, session, cx).await?,
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn active_session(&self) -> Option<Entity<DebugSession>> {
        self.active_session.clone()
    }

    fn handle_dap_store_event(
        &mut self,
        _dap_store: &Entity<DapStore>,
        event: &dap_store::DapStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            dap_store::DapStoreEvent::RunInTerminal {
                session_id,
                title,
                cwd,
                command,
                args,
                envs,
                sender,
                ..
            } => {
                self.handle_run_in_terminal_request(
                    *session_id,
                    title.clone(),
                    cwd.clone(),
                    command.clone(),
                    args.clone(),
                    envs.clone(),
                    sender.clone(),
                    window,
                    cx,
                )
                .detach_and_log_err(cx);
            }
            dap_store::DapStoreEvent::SpawnChildSession {
                request,
                parent_session,
            } => {
                self.start_child_session(request, parent_session.clone(), window, cx);
            }
            _ => {}
        }
    }

    pub fn resolve_scenario(
        &self,
        scenario: DebugScenario,

        task_context: TaskContext,
        buffer: Option<Entity<Buffer>>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<DebugTaskDefinition>> {
        let project = self.project.read(cx);
        let dap_store = project.dap_store().downgrade();
        let task_store = project.task_store().downgrade();
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            let DebugScenario {
                adapter,
                label,
                build,
                request,
                initialize_args,
                tcp_connection,
                stop_on_entry,
            } = scenario;
            let request = if let Some(mut request) = request {
                // Resolve task variables within the request.
                if let DebugRequest::Launch(_) = &mut request {}

                request
            } else if let Some(build) = build {
                let Some(task) = task_store.update(cx, |this, cx| {
                    this.task_inventory().and_then(|inventory| {
                        inventory
                            .read(cx)
                            .task_template_by_label(buffer, &build, cx)
                    })
                })?
                else {
                    anyhow::bail!("Couldn't find task template for {:?}", build)
                };
                let Some(task) = task.resolve_task("debug-build-task", &task_context) else {
                    anyhow::bail!("Could not resolve task variables within a debug scenario");
                };

                let run_build = workspace.update_in(cx, |workspace, window, cx| {
                    workspace.spawn_in_terminal(task.resolved.clone(), window, cx)
                })?;

                let exit_status = run_build.await?;
                if !exit_status.success() {
                    anyhow::bail!("Build failed");
                }

                dap_store
                    .update(cx, |this, cx| this.run_debug_locator(task.resolved, cx))?
                    .await?
            } else {
                return Err(anyhow!("No request or build provided"));
            };
            Ok(DebugTaskDefinition {
                label,
                adapter,
                request,
                initialize_args,
                stop_on_entry,
                tcp_connection,
            })
        })
    }

    fn handle_run_in_terminal_request(
        &self,
        session_id: SessionId,
        title: Option<String>,
        cwd: Option<Arc<Path>>,
        command: Option<String>,
        args: Vec<String>,
        envs: HashMap<String, String>,
        mut sender: mpsc::Sender<Result<u32>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(session) = self
            .sessions
            .iter()
            .find(|s| s.read(cx).session_id(cx) == session_id)
        else {
            return Task::ready(Err(anyhow!("no session {:?} found", session_id)));
        };
        let running = session.read(cx).running_state();
        let cwd = cwd.map(|p| p.to_path_buf());
        let shell = self
            .project
            .read(cx)
            .terminal_settings(&cwd, cx)
            .shell
            .clone();
        let kind = if let Some(command) = command {
            let title = title.clone().unwrap_or(command.clone());
            TerminalKind::Task(task::SpawnInTerminal {
                id: TaskId("debug".to_string()),
                full_label: title.clone(),
                label: title.clone(),
                command: command.clone(),
                args,
                command_label: title.clone(),
                cwd,
                env: envs,
                use_new_terminal: true,
                allow_concurrent_runs: true,
                reveal: RevealStrategy::NoFocus,
                reveal_target: RevealTarget::Dock,
                hide: HideStrategy::Never,
                shell,
                show_summary: false,
                show_command: false,
                show_rerun: false,
            })
        } else {
            TerminalKind::Shell(cwd.map(|c| c.to_path_buf()))
        };

        let workspace = self.workspace.clone();
        let project = self.project.downgrade();

        let terminal_task = self.project.update(cx, |project, cx| {
            project.create_terminal(kind, window.window_handle(), cx)
        });
        let terminal_task = cx.spawn_in(window, async move |_, cx| {
            let terminal = terminal_task.await?;

            let terminal_view = cx.new_window_entity(|window, cx| {
                TerminalView::new(terminal.clone(), workspace, None, project, window, cx)
            })?;

            running.update_in(cx, |running, window, cx| {
                running.ensure_pane_item(DebuggerPaneItem::Terminal, window, cx);
                running.debug_terminal.update(cx, |debug_terminal, cx| {
                    debug_terminal.terminal = Some(terminal_view);
                    cx.notify();
                });
            })?;

            anyhow::Ok(terminal.read_with(cx, |terminal, _| terminal.pty_info.pid())?)
        });

        cx.background_spawn(async move {
            match terminal_task.await {
                Ok(pid_task) => match pid_task {
                    Some(pid) => sender.send(Ok(pid.as_u32())).await?,
                    None => {
                        sender
                            .send(Err(anyhow!(
                                "Terminal was spawned but PID was not available"
                            )))
                            .await?
                    }
                },
                Err(error) => sender.send(Err(anyhow!(error))).await?,
            };

            Ok(())
        })
    }

    fn close_session(&mut self, entity_id: EntityId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self
            .sessions
            .iter()
            .find(|other| entity_id == other.entity_id())
            .cloned()
        else {
            return;
        };
        session.update(cx, |this, cx| {
            if let Some(running) = this.mode().as_running() {
                running.update(cx, |this, cx| {
                    this.serialize_layout(window, cx);
                });
            }
        });
        let session_id = session.update(cx, |this, cx| this.session_id(cx));
        let should_prompt = self
            .project
            .update(cx, |this, cx| {
                let session = this.dap_store().read(cx).session_by_id(session_id);
                session.map(|session| !session.read(cx).is_terminated())
            })
            .unwrap_or_default();

        cx.spawn_in(window, async move |this, cx| {
            if should_prompt {
                let response = cx.prompt(
                    gpui::PromptLevel::Warning,
                    "This Debug Session is still running. Are you sure you want to terminate it?",
                    None,
                    &["Yes", "No"],
                );
                if response.await == Ok(1) {
                    return;
                }
            }
            session.update(cx, |session, cx| session.shutdown(cx)).ok();
            this.update(cx, |this, cx| {
                this.sessions.retain(|other| entity_id != other.entity_id());

                if let Some(active_session_id) = this
                    .active_session
                    .as_ref()
                    .map(|session| session.entity_id())
                {
                    if active_session_id == entity_id {
                        this.active_session = this.sessions.first().cloned();
                    }
                }
                cx.notify()
            })
            .ok();
        })
        .detach();
    }
    fn sessions_drop_down_menu(
        &self,
        active_session: &Entity<DebugSession>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DropdownMenu {
        let sessions = self.sessions.clone();
        let weak = cx.weak_entity();
        let label = active_session.read(cx).label_element(cx);

        DropdownMenu::new_with_element(
            "debugger-session-list",
            label,
            ContextMenu::build(window, cx, move |mut this, _, cx| {
                let context_menu = cx.weak_entity();
                for session in sessions.into_iter() {
                    let weak_session = session.downgrade();
                    let weak_session_id = weak_session.entity_id();

                    this = this.custom_entry(
                        {
                            let weak = weak.clone();
                            let context_menu = context_menu.clone();
                            move |_, cx| {
                                weak_session
                                    .read_with(cx, |session, cx| {
                                        let context_menu = context_menu.clone();
                                        let id: SharedString =
                                            format!("debug-session-{}", session.session_id(cx).0)
                                                .into();
                                        h_flex()
                                            .w_full()
                                            .group(id.clone())
                                            .justify_between()
                                            .child(session.label_element(cx))
                                            .child(
                                                IconButton::new(
                                                    "close-debug-session",
                                                    IconName::Close,
                                                )
                                                .visible_on_hover(id.clone())
                                                .icon_size(IconSize::Small)
                                                .on_click({
                                                    let weak = weak.clone();
                                                    move |_, window, cx| {
                                                        weak.update(cx, |panel, cx| {
                                                            panel.close_session(
                                                                weak_session_id,
                                                                window,
                                                                cx,
                                                            );
                                                        })
                                                        .ok();
                                                        context_menu
                                                            .update(cx, |this, cx| {
                                                                this.cancel(
                                                                    &Default::default(),
                                                                    window,
                                                                    cx,
                                                                );
                                                            })
                                                            .ok();
                                                    }
                                                }),
                                            )
                                            .into_any_element()
                                    })
                                    .unwrap_or_else(|_| div().into_any_element())
                            }
                        },
                        {
                            let weak = weak.clone();
                            move |window, cx| {
                                weak.update(cx, |panel, cx| {
                                    panel.activate_session(session.clone(), window, cx);
                                })
                                .ok();
                            }
                        },
                    );
                }
                this
            }),
        )
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(running_state) = self
            .active_session
            .as_ref()
            .and_then(|session| session.read(cx).mode().as_running().cloned())
        {
            let pane_items_status = running_state.read(cx).pane_items_status(cx);
            let this = cx.weak_entity();

            let context_menu = ContextMenu::build(window, cx, |mut menu, _window, _cx| {
                for (item_kind, is_visible) in pane_items_status.into_iter() {
                    menu = menu.toggleable_entry(item_kind, is_visible, IconPosition::End, None, {
                        let this = this.clone();
                        move |window, cx| {
                            this.update(cx, |this, cx| {
                                if let Some(running_state) =
                                    this.active_session.as_ref().and_then(|session| {
                                        session.read(cx).mode().as_running().cloned()
                                    })
                                {
                                    running_state.update(cx, |state, cx| {
                                        if is_visible {
                                            state.remove_pane_item(item_kind, window, cx);
                                        } else {
                                            state.add_pane_item(item_kind, position, window, cx);
                                        }
                                    })
                                }
                            })
                            .ok();
                        }
                    });
                }

                menu
            });

            window.focus(&context_menu.focus_handle(cx));
            let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                this.context_menu.take();
                cx.notify();
            });
            self.context_menu = Some((context_menu, position, subscription));
        }
    }

    fn top_controls_strip(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<Div> {
        let active_session = self.active_session.clone();
        let focus_handle = self.focus_handle.clone();

        Some(
            h_flex()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .p_1()
                .justify_between()
                .w_full()
                .child(
                    h_flex().gap_2().w_full().when_some(
                        active_session
                            .as_ref()
                            .and_then(|session| session.read(cx).mode().as_running()),
                        |this, running_session| {
                            let thread_status = running_session
                                .read(cx)
                                .thread_status(cx)
                                .unwrap_or(project::debugger::session::ThreadStatus::Exited);
                            let capabilities = running_session.read(cx).capabilities(cx);
                            this.map(|this| {
                                if thread_status == ThreadStatus::Running {
                                    this.child(
                                        IconButton::new("debug-pause", IconName::DebugPause)
                                            .icon_size(IconSize::XSmall)
                                            .shape(ui::IconButtonShape::Square)
                                            .on_click(window.listener_for(
                                                &running_session,
                                                |this, _, _window, cx| {
                                                    this.pause_thread(cx);
                                                },
                                            ))
                                            .tooltip({
                                                let focus_handle = focus_handle.clone();
                                                move |window, cx| {
                                                    Tooltip::for_action_in(
                                                        "Pause program",
                                                        &Pause,
                                                        &focus_handle,
                                                        window,
                                                        cx,
                                                    )
                                                }
                                            }),
                                    )
                                } else {
                                    this.child(
                                        IconButton::new("debug-continue", IconName::DebugContinue)
                                            .icon_size(IconSize::XSmall)
                                            .shape(ui::IconButtonShape::Square)
                                            .on_click(window.listener_for(
                                                &running_session,
                                                |this, _, _window, cx| this.continue_thread(cx),
                                            ))
                                            .disabled(thread_status != ThreadStatus::Stopped)
                                            .tooltip({
                                                let focus_handle = focus_handle.clone();
                                                move |window, cx| {
                                                    Tooltip::for_action_in(
                                                        "Continue program",
                                                        &Continue,
                                                        &focus_handle,
                                                        window,
                                                        cx,
                                                    )
                                                }
                                            }),
                                    )
                                }
                            })
                            .child(
                                IconButton::new("debug-step-over", IconName::ArrowRight)
                                    .icon_size(IconSize::XSmall)
                                    .shape(ui::IconButtonShape::Square)
                                    .on_click(window.listener_for(
                                        &running_session,
                                        |this, _, _window, cx| {
                                            this.step_over(cx);
                                        },
                                    ))
                                    .disabled(thread_status != ThreadStatus::Stopped)
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                "Step over",
                                                &StepOver,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    }),
                            )
                            .child(
                                IconButton::new("debug-step-out", IconName::ArrowUpRight)
                                    .icon_size(IconSize::XSmall)
                                    .shape(ui::IconButtonShape::Square)
                                    .on_click(window.listener_for(
                                        &running_session,
                                        |this, _, _window, cx| {
                                            this.step_out(cx);
                                        },
                                    ))
                                    .disabled(thread_status != ThreadStatus::Stopped)
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                "Step out",
                                                &StepOut,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    }),
                            )
                            .child(
                                IconButton::new("debug-step-into", IconName::ArrowDownRight)
                                    .icon_size(IconSize::XSmall)
                                    .shape(ui::IconButtonShape::Square)
                                    .on_click(window.listener_for(
                                        &running_session,
                                        |this, _, _window, cx| {
                                            this.step_in(cx);
                                        },
                                    ))
                                    .disabled(thread_status != ThreadStatus::Stopped)
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                "Step in",
                                                &StepInto,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    }),
                            )
                            .child(Divider::vertical())
                            .child(
                                IconButton::new(
                                    "debug-enable-breakpoint",
                                    IconName::DebugDisabledBreakpoint,
                                )
                                .icon_size(IconSize::XSmall)
                                .shape(ui::IconButtonShape::Square)
                                .disabled(thread_status != ThreadStatus::Stopped),
                            )
                            .child(
                                IconButton::new("debug-disable-breakpoint", IconName::CircleOff)
                                    .icon_size(IconSize::XSmall)
                                    .shape(ui::IconButtonShape::Square)
                                    .disabled(thread_status != ThreadStatus::Stopped),
                            )
                            .child(
                                IconButton::new("debug-disable-all-breakpoints", IconName::BugOff)
                                    .icon_size(IconSize::XSmall)
                                    .shape(ui::IconButtonShape::Square)
                                    .disabled(
                                        thread_status == ThreadStatus::Exited
                                            || thread_status == ThreadStatus::Ended,
                                    )
                                    .on_click(window.listener_for(
                                        &running_session,
                                        |this, _, _window, cx| {
                                            this.toggle_ignore_breakpoints(cx);
                                        },
                                    ))
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                "Disable all breakpoints",
                                                &ToggleIgnoreBreakpoints,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    }),
                            )
                            .child(Divider::vertical())
                            .child(
                                IconButton::new("debug-restart", IconName::DebugRestart)
                                    .icon_size(IconSize::XSmall)
                                    .on_click(window.listener_for(
                                        &running_session,
                                        |this, _, _window, cx| {
                                            this.restart_session(cx);
                                        },
                                    ))
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                "Restart",
                                                &Restart,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    }),
                            )
                            .child(
                                IconButton::new("debug-stop", IconName::Power)
                                    .icon_size(IconSize::XSmall)
                                    .on_click(window.listener_for(
                                        &running_session,
                                        |this, _, _window, cx| {
                                            this.stop_thread(cx);
                                        },
                                    ))
                                    .disabled(
                                        thread_status != ThreadStatus::Stopped
                                            && thread_status != ThreadStatus::Running,
                                    )
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        let label = if capabilities
                                            .supports_terminate_threads_request
                                            .unwrap_or_default()
                                        {
                                            "Terminate Thread"
                                        } else {
                                            "Terminate All Threads"
                                        };
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                label,
                                                &Stop,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    }),
                            )
                        },
                    ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .when_some(
                            active_session
                                .as_ref()
                                .and_then(|session| session.read(cx).mode().as_running())
                                .cloned(),
                            |this, session| {
                                this.child(
                                    session.update(cx, |this, cx| this.thread_dropdown(window, cx)),
                                )
                                .child(Divider::vertical())
                            },
                        )
                        .when_some(active_session.as_ref(), |this, session| {
                            let context_menu = self.sessions_drop_down_menu(session, window, cx);
                            this.child(context_menu).child(Divider::vertical())
                        })
                        .child(
                            IconButton::new("debug-new-session", IconName::Plus)
                                .icon_size(IconSize::Small)
                                .on_click({
                                    let workspace = self.workspace.clone();
                                    let weak_panel = cx.weak_entity();
                                    let past_debug_definition = self.past_debug_definition.clone();
                                    move |_, window, cx| {
                                        let weak_panel = weak_panel.clone();
                                        let past_debug_definition = past_debug_definition.clone();

                                        let _ = workspace.update(cx, |this, cx| {
                                            let workspace = cx.weak_entity();
                                            this.toggle_modal(window, cx, |window, cx| {
                                                NewSessionModal::new(
                                                    past_debug_definition,
                                                    weak_panel,
                                                    workspace,
                                                    window,
                                                    cx,
                                                )
                                            });
                                        });
                                    }
                                })
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |window, cx| {
                                        Tooltip::for_action_in(
                                            "New Debug Session",
                                            &CreateDebuggingSession,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                    }
                                }),
                        ),
                ),
        )
    }

    fn activate_pane_in_direction(
        &mut self,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(session) = self.active_session() {
            session.update(cx, |session, cx| {
                if let Some(running) = session.mode().as_running() {
                    running.update(cx, |running, cx| {
                        running.activate_pane_in_direction(direction, window, cx);
                    })
                }
            })
        }
    }

    fn activate_item(
        &mut self,
        item: DebuggerPaneItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(session) = self.active_session() {
            session.update(cx, |session, cx| {
                if let Some(running) = session.mode().as_running() {
                    running.update(cx, |running, cx| {
                        running.activate_item(item, window, cx);
                    })
                }
            })
        }
    }

    fn activate_session(
        &mut self,
        session_item: Entity<DebugSession>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        debug_assert!(self.sessions.contains(&session_item));
        session_item.focus_handle(cx).focus(window);
        session_item.update(cx, |this, cx| {
            if let Some(running) = this.mode().as_running() {
                running.update(cx, |this, cx| {
                    this.go_to_selected_stack_frame(window, cx);
                });
            }
        });
        self.active_session = Some(session_item);
        cx.notify();
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}
impl EventEmitter<DebugPanelEvent> for DebugPanel {}

impl Focusable for DebugPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DebugPanel {
    fn persistent_name() -> &'static str {
        "DebugPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Bottom
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position == DockPosition::Bottom
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn size(&self, _window: &Window, _: &App) -> Pixels {
        self.size
    }

    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.size = size.unwrap();
    }

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::DebugPanel)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Debug)
    }

    fn icon_tooltip(&self, _window: &Window, cx: &App) -> Option<&'static str> {
        if DebuggerSettings::get_global(cx).button {
            Some("Debug Panel")
        } else {
            None
        }
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn pane(&self) -> Option<Entity<Pane>> {
        None
    }

    fn activation_priority(&self) -> u32 {
        9
    }

    fn set_active(&mut self, _: bool, _: &mut Window, _: &mut Context<Self>) {}
}

impl Render for DebugPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_sessions = self.sessions.len() > 0;
        let this = cx.weak_entity();
        debug_assert_eq!(has_sessions, self.active_session.is_some());

        if self
            .active_session
            .as_ref()
            .and_then(|session| session.read(cx).mode().as_running().cloned())
            .map(|state| state.read(cx).has_open_context_menu(cx))
            .unwrap_or(false)
        {
            self.context_menu.take();
        }

        v_flex()
            .size_full()
            .key_context("DebugPanel")
            .child(h_flex().children(self.top_controls_strip(window, cx)))
            .track_focus(&self.focus_handle(cx))
            .on_action({
                let this = this.clone();
                move |_: &workspace::ActivatePaneLeft, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_pane_in_direction(SplitDirection::Left, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &workspace::ActivatePaneRight, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_pane_in_direction(SplitDirection::Right, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &workspace::ActivatePaneUp, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_pane_in_direction(SplitDirection::Up, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &workspace::ActivatePaneDown, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_pane_in_direction(SplitDirection::Down, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &FocusConsole, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_item(DebuggerPaneItem::Console, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &FocusVariables, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_item(DebuggerPaneItem::Variables, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &FocusBreakpointList, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_item(DebuggerPaneItem::BreakpointList, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &FocusFrames, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_item(DebuggerPaneItem::Frames, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &FocusModules, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_item(DebuggerPaneItem::Modules, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &FocusLoadedSources, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_item(DebuggerPaneItem::LoadedSources, window, cx);
                    })
                    .ok();
                }
            })
            .on_action({
                let this = this.clone();
                move |_: &FocusTerminal, window, cx| {
                    this.update(cx, |this, cx| {
                        this.activate_item(DebuggerPaneItem::Terminal, window, cx);
                    })
                    .ok();
                }
            })
            .when(self.active_session.is_some(), |this| {
                this.on_mouse_down(
                    MouseButton::Right,
                    cx.listener(|this, event: &MouseDownEvent, window, cx| {
                        if this
                            .active_session
                            .as_ref()
                            .and_then(|session| {
                                session.read(cx).mode().as_running().map(|state| {
                                    state.read(cx).has_pane_at_position(event.position)
                                })
                            })
                            .unwrap_or(false)
                        {
                            this.deploy_context_menu(event.position, window, cx);
                        }
                    }),
                )
                .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                    deferred(
                        anchored()
                            .position(*position)
                            .anchor(gpui::Corner::TopLeft)
                            .child(menu.clone()),
                    )
                    .with_priority(1)
                }))
            })
            .map(|this| {
                if has_sessions {
                    this.children(self.active_session.clone())
                } else {
                    this.child(
                        v_flex()
                            .h_full()
                            .gap_1()
                            .items_center()
                            .justify_center()
                            .child(
                                h_flex().child(
                                    Label::new("No Debugging Sessions")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                            )
                            .child(
                                h_flex().flex_shrink().child(
                                    Button::new("spawn-new-session-empty-state", "New Session")
                                        .size(ButtonSize::Large)
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(
                                                CreateDebuggingSession.boxed_clone(),
                                                cx,
                                            );
                                        }),
                                ),
                            ),
                    )
                }
            })
            .into_any()
    }
}

struct DebuggerProvider(Entity<DebugPanel>);

impl workspace::DebuggerProvider for DebuggerProvider {
    fn start_session(
        &self,
        definition: DebugScenario,
        context: TaskContext,
        buffer: Option<Entity<Buffer>>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.0.update(cx, |_, cx| {
            cx.defer_in(window, |this, window, cx| {
                this.start_session(definition, context, buffer, window, cx);
            })
        })
    }
}
