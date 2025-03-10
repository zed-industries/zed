use crate::session::DebugSession;
use anyhow::Result;
use command_palette_hooks::CommandPaletteFilter;
use dap::{
    client::SessionId, debugger_settings::DebuggerSettings, ContinuedEvent, LoadedSourceEvent,
    ModuleEvent, OutputEvent, StoppedEvent, ThreadEvent,
};
use gpui::{
    actions, Action, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Subscription, Task, WeakEntity,
};
use project::{
    debugger::dap_store::{self, DapStore},
    Project,
};
use rpc::proto::{self};
use settings::Settings;
use std::any::TypeId;
use ui::prelude::*;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    pane, Continue, Disconnect, Pane, Pause, Restart, StepBack, StepInto, StepOut, StepOver, Stop,
    ToggleIgnoreBreakpoints, Workspace,
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
                                            .on_click(cx.listener(move |pane, _, window, cx| {
                                                pane.add_item(
                                                    Box::new(DebugSession::inert(
                                                        project.clone(),
                                                        weak_workspace.clone(),
                                                        window,
                                                        cx,
                                                    )),
                                                    false,
                                                    false,
                                                    None,
                                                    window,
                                                    cx,
                                                );
                                            })),
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
        cx.spawn(|mut cx| async move {
            workspace.update_in(&mut cx, |workspace, window, cx| {
                let debug_panel = DebugPanel::new(workspace, window, cx);

                cx.observe(&debug_panel, |_, debug_panel, cx| {
                    let (has_active_session, support_step_back) =
                        debug_panel.update(cx, |this, cx| {
                            this.active_session(cx)
                                .map(|_item| (true, false))
                                .unwrap_or((false, false))
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
                        TypeId::of::<Restart>(),
                        TypeId::of::<ToggleIgnoreBreakpoints>(),
                    ];

                    let step_back_action_type = [TypeId::of::<StepBack>()];

                    if has_active_session {
                        filter.show_action_types(debugger_action_types.iter());

                        if support_step_back {
                            filter.show_action_types(step_back_action_type.iter());
                        } else {
                            filter.hide_action_types(&step_back_action_type);
                        }
                    } else {
                        // show only the `debug: start`
                        filter.hide_action_types(&debugger_action_types);
                        filter.hide_action_types(&step_back_action_type);
                    }
                })
                .detach();

                debug_panel
            })
        })
    }

    pub fn active_session(&self, cx: &Context<Self>) -> Option<Entity<DebugSession>> {
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
                    return log::error!("Couldn't get session with id: {session_id:?} from DebugClientStarted event");
                };

                let Some(project) = self.project.upgrade() else {
                    return log::error!("Debug Panel out lived it's weak reference to Project");
                };

                let session_item =
                    DebugSession::running(project, self.workspace.clone(), session, window, cx);

                self.pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(session_item), true, true, None, window, cx);
                    window.focus(&pane.focus_handle(cx));
                    cx.notify();
                });
            }
            _ => {}
        }
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
            // todo: We need to revisit it when we start adding stopped items to pane (as that'll cause us to add two items).
            self.pane.update(cx, |this, cx| {
                this.add_item(
                    Box::new(DebugSession::inert(
                        project,
                        self.workspace.clone(),
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
