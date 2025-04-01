use crate::session::DebugSession;
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
    debugger::dap_store::{self, DapStore},
    terminals::TerminalKind,
};
use rpc::proto::{self};
use settings::Settings;
use std::{any::TypeId, path::PathBuf};
use task::DebugTaskDefinition;
use terminal_view::terminal_panel::TerminalPanel;
use ui::prelude::*;
use util::ResultExt;
use workspace::{
    ClearAllBreakpoints, Continue, Disconnect, Pane, Pause, Restart, StepBack, StepInto, StepOut,
    StepOver, Stop, ToggleIgnoreBreakpoints, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    pane,
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
    pane: Entity<Pane>,
    project: WeakEntity<Project>,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
    pub(crate) last_inert_config: Option<DebugTaskDefinition>,
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
            let weak_workspace = workspace.weak_handle();
            let debug_panel = cx.weak_entity();
            let pane = cx.new(|cx| {
                let mut pane = Pane::new(
                    workspace.weak_handle(),
                    project.clone(),
                    Default::default(),
                    None,
                    gpui::NoAction.boxed_clone(),
                    window,
                    cx,
                );
                pane.set_can_split(None);
                pane.set_can_navigate(true, cx);
                pane.display_nav_history_buttons(None);
                pane.set_should_display_tab_bar(|_window, _cx| true);
                pane.set_close_pane_if_empty(true, cx);
                pane.set_render_tab_bar_buttons(cx, {
                    let project = project.clone();
                    let weak_workspace = weak_workspace.clone();
                    let debug_panel = debug_panel.clone();
                    move |_, _, cx| {
                        let project = project.clone();
                        let weak_workspace = weak_workspace.clone();
                        (
                            None,
                            Some(
                                h_flex()
                                    .child(
                                        IconButton::new("new-debug-session", IconName::Plus)
                                            .icon_size(IconSize::Small)
                                            .on_click({
                                                let debug_panel = debug_panel.clone();

                                                cx.listener(move |pane, _, window, cx| {
                                                    let config = debug_panel
                                                        .read_with(cx, |this: &DebugPanel, _| {
                                                            this.last_inert_config.clone()
                                                        })
                                                        .log_err()
                                                        .flatten();

                                                    pane.add_item(
                                                        Box::new(DebugSession::inert(
                                                            project.clone(),
                                                            weak_workspace.clone(),
                                                            debug_panel.clone(),
                                                            config,
                                                            window,
                                                            cx,
                                                        )),
                                                        false,
                                                        false,
                                                        None,
                                                        window,
                                                        cx,
                                                    );
                                                })
                                            }),
                                    )
                                    .into_any_element(),
                            ),
                        )
                    }
                });
                pane.add_item(
                    Box::new(DebugSession::inert(
                        project.clone(),
                        weak_workspace.clone(),
                        debug_panel.clone(),
                        None,
                        window,
                        cx,
                    )),
                    false,
                    false,
                    None,
                    window,
                    cx,
                );
                pane
            });

            let _subscriptions = vec![
                cx.observe(&pane, |_, _, cx| cx.notify()),
                cx.subscribe_in(&pane, window, Self::handle_pane_event),
                cx.subscribe_in(&dap_store, window, Self::handle_dap_store_event),
            ];

            let debug_panel = Self {
                pane,
                size: px(300.),
                _subscriptions,
                last_inert_config: None,
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
                            this.active_session(cx)
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

    pub fn active_session(&self, cx: &App) -> Option<Entity<DebugSession>> {
        self.pane
            .read(cx)
            .active_item()
            .and_then(|panel| panel.downcast::<DebugSession>())
    }

    pub fn debug_panel_items_by_client(
        &self,
        client_id: &SessionId,
        cx: &Context<Self>,
    ) -> Vec<Entity<DebugSession>> {
        self.pane
            .read(cx)
            .items()
            .filter_map(|item| item.downcast::<DebugSession>())
            .filter(|item| item.read(cx).session_id(cx) == Some(*client_id))
            .map(|item| item.clone())
            .collect()
    }

    pub fn debug_panel_item_by_client(
        &self,
        client_id: SessionId,
        cx: &mut Context<Self>,
    ) -> Option<Entity<DebugSession>> {
        self.pane
            .read(cx)
            .items()
            .filter_map(|item| item.downcast::<DebugSession>())
            .find(|item| {
                let item = item.read(cx);

                item.session_id(cx) == Some(client_id)
            })
    }

    fn handle_dap_store_event(
        &mut self,
        dap_store: &Entity<DapStore>,
        event: &dap_store::DapStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            dap_store::DapStoreEvent::DebugClientStarted(session_id) => {
                let Some(session) = dap_store.read(cx).session_by_id(session_id) else {
                    return log::error!(
                        "Couldn't get session with id: {session_id:?} from DebugClientStarted event"
                    );
                };

                let Some(project) = self.project.upgrade() else {
                    return log::error!("Debug Panel out lived it's weak reference to Project");
                };

                if self.pane.read_with(cx, |pane, cx| {
                    pane.items_of_type::<DebugSession>()
                        .any(|item| item.read(cx).session_id(cx) == Some(*session_id))
                }) {
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

                self.pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(session_item), true, true, None, window, cx);
                    window.focus(&pane.focus_handle(cx));
                    cx.notify();
                });
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

    fn handle_pane_event(
        &mut self,
        _: &Entity<Pane>,
        event: &pane::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            pane::Event::Remove { .. } => cx.emit(PanelEvent::Close),
            pane::Event::ZoomIn => cx.emit(PanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(PanelEvent::ZoomOut),
            pane::Event::AddItem { item } => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, self.pane.clone(), window, cx)
                    })
                    .ok();
            }
            pane::Event::RemovedItem { item } => {
                if let Some(debug_session) = item.downcast::<DebugSession>() {
                    debug_session.update(cx, |session, cx| {
                        session.shutdown(cx);
                    })
                }
            }
            pane::Event::ActivateItem {
                local: _,
                focus_changed,
            } => {
                if *focus_changed {
                    if let Some(debug_session) = self
                        .pane
                        .read(cx)
                        .active_item()
                        .and_then(|item| item.downcast::<DebugSession>())
                    {
                        if let Some(running) = debug_session
                            .read_with(cx, |session, _| session.mode().as_running().cloned())
                        {
                            running.update(cx, |running, cx| {
                                running.go_to_selected_stack_frame(window, cx);
                            });
                        }
                    }
                }
            }

            _ => {}
        }
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}
impl EventEmitter<DebugPanelEvent> for DebugPanel {}
impl EventEmitter<project::Event> for DebugPanel {}

impl Focusable for DebugPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

impl Panel for DebugPanel {
    fn pane(&self) -> Option<Entity<Pane>> {
        Some(self.pane.clone())
    }

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

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
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
    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        if active && self.pane.read(cx).items_len() == 0 {
            let Some(project) = self.project.clone().upgrade() else {
                return;
            };
            let config = self.last_inert_config.clone();
            let panel = cx.weak_entity();
            // todo: We need to revisit it when we start adding stopped items to pane (as that'll cause us to add two items).
            self.pane.update(cx, |this, cx| {
                this.add_item(
                    Box::new(DebugSession::inert(
                        project,
                        self.workspace.clone(),
                        panel,
                        config,
                        window,
                        cx,
                    )),
                    false,
                    false,
                    None,
                    window,
                    cx,
                );
            });
        }
    }
}

impl Render for DebugPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DebugPanel")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .child(self.pane.clone())
            .into_any()
    }
}
