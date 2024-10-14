use crate::debugger_panel_item::DebugPanelItem;
use anyhow::Result;
use dap::client::DebugAdapterClient;
use dap::client::{DebugAdapterClientId, ThreadStatus};
use dap::debugger_settings::DebuggerSettings;
use dap::messages::{Events, Message};
use dap::requests::{Request, StartDebugging};
use dap::{
    Capabilities, CapabilitiesEvent, ContinuedEvent, ExitedEvent, LoadedSourceEvent, ModuleEvent,
    OutputEvent, StoppedEvent, TerminatedEvent, ThreadEvent, ThreadEventReason,
};
use gpui::{
    actions, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView,
    FontWeight, Model, Subscription, Task, View, ViewContext, WeakView,
};
use project::dap_store::DapStore;
use serde_json::Value;
use settings::Settings;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::u64;
use ui::prelude::*;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};
use workspace::{pane, Pane, Start};

pub enum DebugPanelEvent {
    Exited(DebugAdapterClientId),
    Terminated(DebugAdapterClientId),
    Stopped {
        client_id: DebugAdapterClientId,
        event: StoppedEvent,
        go_to_stack_frame: bool,
    },
    Thread((DebugAdapterClientId, ThreadEvent)),
    Continued((DebugAdapterClientId, ContinuedEvent)),
    Output((DebugAdapterClientId, OutputEvent)),
    Module((DebugAdapterClientId, ModuleEvent)),
    LoadedSource((DebugAdapterClientId, LoadedSourceEvent)),
    ClientStopped(DebugAdapterClientId),
    CapabilitiesChanged(DebugAdapterClientId),
}

actions!(debug_panel, [ToggleFocus]);

#[derive(Debug, Default, Clone)]
pub struct ThreadState {
    pub status: ThreadStatus,
    // we update this value only once we stopped,
    // we will use this to indicated if we should show a warning when debugger thread was exited
    pub stopped: bool,
}

pub struct DebugPanel {
    size: Pixels,
    pane: View<Pane>,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    workspace: WeakView<Workspace>,
    show_did_not_stop_warning: bool,
    _subscriptions: Vec<Subscription>,
    thread_states: BTreeMap<(DebugAdapterClientId, u64), Model<ThreadState>>,
}

impl DebugPanel {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx| {
            let pane = cx.new_view(|cx| {
                let mut pane = Pane::new(
                    workspace.weak_handle(),
                    workspace.project().clone(),
                    Default::default(),
                    None,
                    None,
                    cx,
                );
                pane.set_can_split(false, cx);
                pane.set_can_navigate(true, cx);
                pane.display_nav_history_buttons(None);
                pane.set_should_display_tab_bar(|_| true);
                pane.set_close_pane_if_empty(false, cx);

                pane
            });

            let project = workspace.project().clone();

            let _subscriptions = vec![
                cx.observe(&pane, |_, _, cx| cx.notify()),
                cx.subscribe(&pane, Self::handle_pane_event),
                cx.subscribe(&project, {
                    move |this: &mut Self, _, event, cx| match event {
                        project::Event::DebugClientEvent { message, client_id } => {
                            let Some(client) = this.debug_client_by_id(client_id, cx) else {
                                return cx.emit(DebugPanelEvent::ClientStopped(*client_id));
                            };

                            match message {
                                Message::Event(event) => {
                                    this.handle_debug_client_events(client_id, event, cx);
                                }
                                Message::Request(request) => {
                                    if StartDebugging::COMMAND == request.command {
                                        Self::handle_start_debugging_request(
                                            this,
                                            client,
                                            request.arguments.clone(),
                                            cx,
                                        );
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        project::Event::DebugClientStopped(client_id) => {
                            cx.emit(DebugPanelEvent::ClientStopped(*client_id));

                            this.thread_states
                                .retain(|&(client_id_, _), _| client_id_ != *client_id);

                            cx.notify();
                        }
                        _ => {}
                    }
                }),
            ];

            Self {
                pane,
                size: px(300.),
                _subscriptions,
                dap_store: project.read(cx).dap_store(),
                focus_handle: cx.focus_handle(),
                show_did_not_stop_warning: false,
                thread_states: Default::default(),
                workspace: workspace.weak_handle(),
            }
        })
    }

    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| DebugPanel::new(workspace, cx))
        })
    }

    pub fn active_debug_panel_item(
        &self,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<DebugPanelItem>> {
        self.pane
            .read(cx)
            .active_item()
            .and_then(|panel| panel.downcast::<DebugPanelItem>())
    }

    fn debug_client_by_id(
        &self,
        client_id: &DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Arc<DebugAdapterClient>> {
        self.workspace
            .update(cx, |this, cx| {
                this.project()
                    .read(cx)
                    .dap_store()
                    .read(cx)
                    .client_by_id(client_id)
            })
            .ok()
            .flatten()
    }

    fn handle_pane_event(
        &mut self,
        _: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::RemovedItem { item } => {
                let thread_panel = item.downcast::<DebugPanelItem>().unwrap();

                let thread_id = thread_panel.read(cx).thread_id();
                let client_id = thread_panel.read(cx).client_id();

                self.thread_states.remove(&(client_id, thread_id));

                cx.notify();

                self.dap_store.update(cx, |store, cx| {
                    store
                        .terminate_threads(&client_id, Some(vec![thread_id; 1]), cx)
                        .detach()
                });
            }
            pane::Event::Remove { .. } => cx.emit(PanelEvent::Close),
            pane::Event::ZoomIn => cx.emit(PanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(PanelEvent::ZoomOut),
            pane::Event::AddItem { item } => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, self.pane.clone(), cx)
                    })
                    .ok();
            }
            pane::Event::ActivateItem { local } => {
                if !local {
                    return;
                }

                if let Some(active_item) = self.pane.read(cx).active_item() {
                    if let Some(debug_item) = active_item.downcast::<DebugPanelItem>() {
                        debug_item.update(cx, |panel, cx| {
                            panel.go_to_current_stack_frame(cx);
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_start_debugging_request(
        this: &mut Self,
        client: Arc<DebugAdapterClient>,
        request_args: Option<Value>,
        cx: &mut ViewContext<Self>,
    ) {
        let start_args = if let Some(args) = request_args {
            serde_json::from_value(args.clone()).ok()
        } else {
            None
        };

        this.dap_store.update(cx, |store, cx| {
            store.start_client(client.config(), start_args, cx);
        });
    }

    fn handle_debug_client_events(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &Events,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            Events::Initialized(event) => self.handle_initialized_event(&client_id, event, cx),
            Events::Stopped(event) => self.handle_stopped_event(&client_id, event, cx),
            Events::Continued(event) => self.handle_continued_event(&client_id, event, cx),
            Events::Exited(event) => self.handle_exited_event(&client_id, event, cx),
            Events::Terminated(event) => self.handle_terminated_event(&client_id, event, cx),
            Events::Thread(event) => self.handle_thread_event(&client_id, event, cx),
            Events::Output(event) => self.handle_output_event(&client_id, event, cx),
            Events::Breakpoint(_) => {}
            Events::Module(event) => self.handle_module_event(&client_id, event, cx),
            Events::LoadedSource(event) => self.handle_loaded_source_event(&client_id, event, cx),
            Events::Capabilities(event) => {
                self.handle_capabilities_changed_event(client_id, event, cx);
            }
            Events::Memory(_) => {}
            Events::Process(_) => {}
            Events::ProgressEnd(_) => {}
            Events::ProgressStart(_) => {}
            Events::ProgressUpdate(_) => {}
            Events::Invalidated(_) => {}
            Events::Other(_) => {}
        }
    }

    fn handle_initialized_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        capabilities: &Option<Capabilities>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(capabilities) = capabilities {
            self.dap_store.update(cx, |store, cx| {
                store.merge_capabilities_for_client(&client_id, capabilities, cx);
            });

            cx.emit(DebugPanelEvent::CapabilitiesChanged(*client_id));
        }

        let send_breakpoints_task = self.workspace.update(cx, |workspace, cx| {
            workspace
                .project()
                .update(cx, |project, cx| project.send_breakpoints(&client_id, cx))
        });

        let configuration_done_task = self
            .dap_store
            .update(cx, |store, cx| store.configuration_done(&client_id, cx));

        cx.background_executor()
            .spawn(async move {
                send_breakpoints_task?.await;

                configuration_done_task.await
            })
            .detach_and_log_err(cx);
    }

    fn handle_continued_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ContinuedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(DebugPanelEvent::Continued((*client_id, event.clone())));
    }

    fn handle_stopped_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &StoppedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(thread_id) = event.thread_id else {
            return;
        };

        let Some(client_kind) = self
            .dap_store
            .read(cx)
            .client_by_id(client_id)
            .map(|c| c.config().kind)
        else {
            return; // this can never happen
        };

        let client_id = *client_id;

        cx.spawn({
            let event = event.clone();
            |this, mut cx| async move {
                let workspace = this.update(&mut cx, |this, cx| {
                    let thread_state = this
                        .thread_states
                        .entry((client_id, thread_id))
                        .or_insert(cx.new_model(|_| ThreadState::default()))
                        .clone();

                    thread_state.update(cx, |thread_state, cx| {
                        thread_state.stopped = true;
                        thread_state.status = ThreadStatus::Stopped;

                        cx.notify();
                    });

                    let existing_item = this
                        .pane
                        .read(cx)
                        .items()
                        .filter_map(|item| item.downcast::<DebugPanelItem>())
                        .any(|item| {
                            let item = item.read(cx);

                            item.client_id() == client_id && item.thread_id() == thread_id
                        });

                    if !existing_item {
                        let debug_panel = cx.view().clone();
                        this.pane.update(cx, |pane, cx| {
                            let tab = cx.new_view(|cx| {
                                DebugPanelItem::new(
                                    debug_panel,
                                    this.workspace.clone(),
                                    this.dap_store.clone(),
                                    thread_state.clone(),
                                    &client_id,
                                    &client_kind,
                                    thread_id,
                                    cx,
                                )
                            });

                            pane.add_item(Box::new(tab), true, true, None, cx);
                        });
                    }

                    let go_to_stack_frame = if let Some(item) = this.pane.read(cx).active_item() {
                        item.downcast::<DebugPanelItem>().map_or(false, |pane| {
                            let pane = pane.read(cx);
                            pane.thread_id() == thread_id && pane.client_id() == client_id
                        })
                    } else {
                        true
                    };

                    cx.emit(DebugPanelEvent::Stopped {
                        client_id,
                        event,
                        go_to_stack_frame,
                    });

                    cx.notify();

                    this.workspace.clone()
                })?;

                cx.update(|cx| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.focus_panel::<Self>(cx);
                    })
                })
            }
        })
        .detach_and_log_err(cx);
    }

    fn handle_thread_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ThreadEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let thread_id = event.thread_id;

        if let Some(thread_state) = self.thread_states.get(&(*client_id, thread_id)) {
            if !thread_state.read(cx).stopped && event.reason == ThreadEventReason::Exited {
                self.show_did_not_stop_warning = true;
                cx.notify();
            };
        }

        if event.reason == ThreadEventReason::Started {
            self.thread_states.insert(
                (*client_id, thread_id),
                cx.new_model(|_| ThreadState::default()),
            );
        }

        cx.emit(DebugPanelEvent::Thread((*client_id, event.clone())));
    }

    fn handle_exited_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        _: &ExitedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(DebugPanelEvent::Exited(*client_id));
    }

    fn handle_terminated_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &Option<TerminatedEvent>,
        cx: &mut ViewContext<Self>,
    ) {
        let restart_args = event.clone().and_then(|e| e.restart);

        for (_, thread_state) in self
            .thread_states
            .range_mut(&(*client_id, u64::MIN)..&(*client_id, u64::MAX))
        {
            thread_state.update(cx, |thread_state, cx| {
                thread_state.status = ThreadStatus::Ended;

                cx.notify();
            });
        }

        self.dap_store.update(cx, |store, cx| {
            if restart_args.is_some() {
                store
                    .restart(&client_id, restart_args, cx)
                    .detach_and_log_err(cx);
            } else {
                store.shutdown_client(&client_id, cx).detach_and_log_err(cx);
            }
        });

        cx.emit(DebugPanelEvent::Terminated(*client_id));
    }

    fn handle_output_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &OutputEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(DebugPanelEvent::Output((*client_id, event.clone())));
    }

    fn handle_module_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ModuleEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(DebugPanelEvent::Module((*client_id, event.clone())));
    }

    fn handle_loaded_source_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &LoadedSourceEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(DebugPanelEvent::LoadedSource((*client_id, event.clone())));
    }

    fn handle_capabilities_changed_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &CapabilitiesEvent,
        cx: &mut ViewContext<Self>,
    ) {
        self.dap_store.update(cx, |store, cx| {
            store.merge_capabilities_for_client(client_id, &event.capabilities, cx);
        });

        cx.emit(DebugPanelEvent::CapabilitiesChanged(*client_id));
    }

    fn render_did_not_stop_warning(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        const TITLE: &'static str = "Debug session exited without hitting any breakpoints";
        const DESCRIPTION: &'static str =
            "Try adding a breakpoint, or define the correct path mapping for your debugger.";

        div()
            .absolute()
            .right_3()
            .bottom_12()
            .max_w_96()
            .py_2()
            .px_3()
            .elevation_2(cx)
            .occlude()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .items_center()
                            .child(Icon::new(IconName::Warning).color(Color::Conflict))
                            .child(Label::new(TITLE).weight(FontWeight::MEDIUM)),
                    )
                    .child(
                        Label::new(DESCRIPTION)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        h_flex().justify_end().mt_1().child(
                            Button::new("dismiss", "Dismiss")
                                .color(Color::Muted)
                                .on_click(cx.listener(|this, _, cx| {
                                    this.show_did_not_stop_warning = false;
                                    cx.notify();
                                })),
                        ),
                    ),
            )
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}
impl EventEmitter<DebugPanelEvent> for DebugPanel {}
impl EventEmitter<project::Event> for DebugPanel {}

impl FocusableView for DebugPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DebugPanel {
    fn pane(&self) -> Option<View<Pane>> {
        Some(self.pane.clone())
    }

    fn persistent_name() -> &'static str {
        "DebugPanel"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        DockPosition::Bottom
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position == DockPosition::Bottom
    }

    fn set_position(&mut self, _position: DockPosition, _cx: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &WindowContext) -> Pixels {
        self.size
    }

    fn set_size(&mut self, size: Option<Pixels>, _cx: &mut ViewContext<Self>) {
        self.size = size.unwrap();
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        Some(IconName::Debug)
    }

    fn icon_tooltip(&self, cx: &WindowContext) -> Option<&'static str> {
        if DebuggerSettings::get_global(cx).button {
            Some("Debug Panel")
        } else {
            None
        }
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl Render for DebugPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DebugPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .when(self.show_did_not_stop_warning, |this| {
              this.child(self.render_did_not_stop_warning(cx))
            })
            .map(|this| {
                if self.pane.read(cx).items_len() == 0 {
                    this.child(
                        h_flex().size_full().items_center().justify_center().child(
                            v_flex()
                                .gap_2()
                                .rounded_md()
                                .max_w_64()
                                .items_start()
                                .child(
                                    Label::new("You can create a debug task by creating a new task and setting the `type` key to `debug`")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    h_flex().w_full().justify_end().child(
                                        Button::new(
                                            "start-debugger",
                                            "Choose a debugger",
                                        )
                                        .label_size(LabelSize::Small)
                                        .on_click(move |_, cx| {
                                            cx.dispatch_action(Start.boxed_clone());
                                        })
                                    ),
                                ),
                        ),
                    )
                } else {
                    this.child(self.pane.clone())
                }
            })
            .into_any()
    }
}
