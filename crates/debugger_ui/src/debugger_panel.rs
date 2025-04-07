use crate::{
    ClearAllBreakpoints, Continue, CreateDebuggingSession, Disconnect, Pause, Restart, StepBack,
    StepInto, StepOut, StepOver, Stop, ToggleIgnoreBreakpoints,
};
use crate::{new_session_modal::NewSessionModal, session::DebugSession};
use anyhow::{Result, anyhow};
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use dap::{
    ContinuedEvent, LoadedSourceEvent, ModuleEvent, OutputEvent, StoppedEvent, ThreadEvent,
    client::SessionId, debugger_settings::DebuggerSettings,
};
use futures::{SinkExt as _, channel::mpsc};
use gpui::{
    Action, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, WeakEntity, actions,
};
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
use std::{any::TypeId, path::PathBuf};
use task::DebugTaskDefinition;
use terminal_view::terminal_panel::TerminalPanel;
use ui::{ContextMenu, Divider, DropdownMenu, Tooltip, prelude::*};
use workspace::{
    Workspace,
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
    project: WeakEntity<Project>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
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
                project: project.downgrade(),
                workspace: workspace.weak_handle(),
            };

            debug_panel
        })
    }

    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
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

                cx.observe(&debug_panel, |_, debug_panel, cx| {
                    let (has_active_session, supports_restart, support_step_back) = debug_panel
                        .update(cx, |this, cx| {
                            this.active_session()
                                .map(|item| {
                                    let running = item.read(cx).mode().as_running().cloned();

                                    match running {
                                        Some(running) => {
                                            let caps = running.read(cx).capabilities(cx);
                                            (
                                                true,
                                                caps.supports_restart_request.unwrap_or_default(),
                                                caps.supports_step_back.unwrap_or_default(),
                                            )
                                        }
                                        None => (false, false, false),
                                    }
                                })
                                .unwrap_or((false, false, false))
                        });

                    let filter = CommandPaletteFilter::global_mut(cx);
                    let debugger_action_types = [
                        TypeId::of::<Continue>(),
                        TypeId::of::<StepOver>(),
                        TypeId::of::<StepInto>(),
                        TypeId::of::<StepOut>(),
                        TypeId::of::<Stop>(),
                        TypeId::of::<Disconnect>(),
                        TypeId::of::<Pause>(),
                        TypeId::of::<ToggleIgnoreBreakpoints>(),
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
                    } else {
                        // show only the `debug: start`
                        filter.hide_action_types(&debugger_action_types);
                        filter.hide_action_types(&step_back_action_type);
                        filter.hide_action_types(&restart_action_type);
                    }
                })
                .detach();

                debug_panel
            })
        })
    }

    pub fn active_session(&self) -> Option<Entity<DebugSession>> {
        self.active_session.clone()
    }

    pub fn debug_panel_items_by_client(
        &self,
        client_id: &SessionId,
        cx: &Context<Self>,
    ) -> Vec<Entity<DebugSession>> {
        self.sessions
            .iter()
            .filter(|item| item.read(cx).session_id(cx) == *client_id)
            .map(|item| item.clone())
            .collect()
    }

    pub fn debug_panel_item_by_client(
        &self,
        client_id: SessionId,
        cx: &mut Context<Self>,
    ) -> Option<Entity<DebugSession>> {
        self.sessions
            .iter()
            .find(|item| {
                let item = item.read(cx);

                item.session_id(cx) == client_id
            })
            .cloned()
    }

    fn handle_dap_store_event(
        &mut self,
        dap_store: &Entity<DapStore>,
        event: &dap_store::DapStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            dap_store::DapStoreEvent::DebugSessionInitialized(session_id) => {
                let Some(session) = dap_store.read(cx).session_by_id(session_id) else {
                    return log::error!(
                        "Couldn't get session with id: {session_id:?} from DebugClientStarted event"
                    );
                };

                let Some(project) = self.project.upgrade() else {
                    return log::error!("Debug Panel out lived it's weak reference to Project");
                };

                if self
                    .sessions
                    .iter()
                    .any(|item| item.read(cx).session_id(cx) == *session_id)
                {
                    // We already have an item for this session.
                    return;
                }
                let session_item = DebugSession::running(
                    project,
                    self.workspace.clone(),
                    session,
                    cx.weak_entity(),
                    window,
                    cx,
                );

                self.sessions.push(session_item.clone());
                self.activate_session(session_item, window, cx);
            }
            dap_store::DapStoreEvent::RunInTerminal {
                title,
                cwd,
                command,
                args,
                envs,
                sender,
                ..
            } => {
                self.handle_run_in_terminal_request(
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
            _ => {}
        }
    }

    fn handle_run_in_terminal_request(
        &self,
        title: Option<String>,
        cwd: PathBuf,
        command: Option<String>,
        args: Vec<String>,
        envs: HashMap<String, String>,
        mut sender: mpsc::Sender<Result<u32>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let terminal_task = self.workspace.update(cx, |workspace, cx| {
            let terminal_panel = workspace.panel::<TerminalPanel>(cx).ok_or_else(|| {
                anyhow!("RunInTerminal DAP request failed because TerminalPanel wasn't found")
            });

            let terminal_panel = match terminal_panel {
                Ok(panel) => panel,
                Err(err) => return Task::ready(Err(err)),
            };

            terminal_panel.update(cx, |terminal_panel, cx| {
                let terminal_task = terminal_panel.add_terminal(
                    TerminalKind::Debug {
                        command,
                        args,
                        envs,
                        cwd,
                        title,
                    },
                    task::RevealStrategy::Always,
                    window,
                    cx,
                );

                cx.spawn(async move |_, cx| {
                    let pid_task = async move {
                        let terminal = terminal_task.await?;

                        terminal.read_with(cx, |terminal, _| terminal.pty_info.pid())
                    };

                    pid_task.await
                })
            })
        });

        cx.background_spawn(async move {
            match terminal_task {
                Ok(pid_task) => match pid_task.await {
                    Ok(Some(pid)) => sender.send(Ok(pid.as_u32())).await?,
                    Ok(None) => {
                        sender
                            .send(Err(anyhow!(
                                "Terminal was spawned but PID was not available"
                            )))
                            .await?
                    }
                    Err(error) => sender.send(Err(anyhow!(error))).await?,
                },
                Err(error) => sender.send(Err(anyhow!(error))).await?,
            };

            Ok(())
        })
    }

    fn top_controls_strip(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<Div> {
        let active_session = self.active_session.clone();

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
                                            .tooltip(move |window, cx| {
                                                Tooltip::text("Pause program")(window, cx)
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
                                            .tooltip(move |window, cx| {
                                                Tooltip::text("Continue program")(window, cx)
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
                                    .tooltip(move |window, cx| {
                                        Tooltip::text("Step over")(window, cx)
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
                                    .tooltip(move |window, cx| {
                                        Tooltip::text("Step out")(window, cx)
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
                                    .tooltip(move |window, cx| {
                                        Tooltip::text("Step in")(window, cx)
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
                                    .tooltip(move |window, cx| {
                                        Tooltip::text("Disable all breakpoints")(window, cx)
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
                                    .disabled(
                                        !capabilities.supports_restart_request.unwrap_or_default(),
                                    )
                                    .tooltip(move |window, cx| {
                                        Tooltip::text("Restart")(window, cx)
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
                                        let label = if capabilities
                                            .supports_terminate_threads_request
                                            .unwrap_or_default()
                                        {
                                            "Terminate Thread"
                                        } else {
                                            "Terminate all Threads"
                                        };
                                        move |window, cx| Tooltip::text(label)(window, cx)
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
                            let sessions = self.sessions.clone();
                            let weak = cx.weak_entity();
                            let label = session.read(cx).label(cx);
                            this.child(DropdownMenu::new(
                                "debugger-session-list",
                                label,
                                ContextMenu::build(window, cx, move |mut this, _, cx| {
                                    for item in sessions {
                                        let weak = weak.clone();
                                        this = this.entry(
                                            session.read(cx).label(cx),
                                            None,
                                            move |window, cx| {
                                                weak.update(cx, |panel, cx| {
                                                    panel.activate_session(
                                                        item.clone(),
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                            },
                                        );
                                    }
                                    this
                                }),
                            ))
                            .child(Divider::vertical())
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
                                .tooltip(|window, cx| {
                                    Tooltip::for_action(
                                        "New Debug Session",
                                        &CreateDebuggingSession,
                                        window,
                                        cx,
                                    )
                                }),
                        ),
                ),
        )
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
impl EventEmitter<project::Event> for DebugPanel {}

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

    fn activation_priority(&self) -> u32 {
        9
    }
    fn set_active(&mut self, _: bool, _: &mut Window, _: &mut Context<Self>) {}
}

impl Render for DebugPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_sessions = self.sessions.len() > 0;
        debug_assert_eq!(has_sessions, self.active_session.is_some());

        v_flex()
            .size_full()
            .key_context("DebugPanel")
            .child(h_flex().children(self.top_controls_strip(window, cx)))
            .track_focus(&self.focus_handle(cx))
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
