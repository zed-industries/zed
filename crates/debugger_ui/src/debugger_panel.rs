use crate::{attach_modal::AttachModal, debugger_panel_item::DebugPanelItem};
use anyhow::Result;
use client::proto;
use collections::{BTreeMap, HashMap};
use command_palette_hooks::CommandPaletteFilter;
use dap::{
    client::DebugAdapterClientId,
    debugger_settings::DebuggerSettings,
    messages::{Events, Message},
    requests::{Request, RunInTerminal, StartDebugging},
    session::DebugSessionId,
    Capabilities, CapabilitiesEvent, ContinuedEvent, ErrorResponse, ExitedEvent, LoadedSourceEvent,
    ModuleEvent, OutputEvent, RunInTerminalRequestArguments, RunInTerminalResponse, StoppedEvent,
    TerminatedEvent, ThreadEvent, ThreadEventReason,
};
use gpui::{
    actions, Action, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Subscription, Task, WeakEntity,
};
use project::{
    dap_store::{DapStore, DapStoreEvent},
    terminals::TerminalKind,
};
use rpc::proto::{SetDebuggerPanelItem, UpdateDebugAdapter};
use serde_json::Value;
use settings::Settings;
use std::{any::TypeId, collections::VecDeque, path::PathBuf, u64};
use task::DebugRequestType;
use terminal_view::terminal_panel::TerminalPanel;
use ui::prelude::*;
use util::ResultExt as _;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    pane, Continue, Disconnect, Pane, Pause, Restart, Start, StepBack, StepInto, StepOut, StepOver,
    Stop, ToggleIgnoreBreakpoints, Workspace,
};

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
    ClientShutdown(DebugAdapterClientId),
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

impl ThreadState {
    pub fn from_proto(thread_state: proto::DebuggerThreadState) -> Self {
        let status = ThreadStatus::from_proto(thread_state.thread_status());

        Self {
            status,
            stopped: thread_state.stopped,
        }
    }

    pub fn to_proto(&self) -> proto::DebuggerThreadState {
        let status = self.status.to_proto();

        proto::DebuggerThreadState {
            thread_status: status,
            stopped: self.stopped,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThreadStatus {
    #[default]
    Running,
    Stopped,
    Exited,
    Ended,
}

impl ThreadStatus {
    pub fn from_proto(status: proto::DebuggerThreadStatus) -> Self {
        match status {
            proto::DebuggerThreadStatus::Running => Self::Running,
            proto::DebuggerThreadStatus::Stopped => Self::Stopped,
            proto::DebuggerThreadStatus::Exited => Self::Exited,
            proto::DebuggerThreadStatus::Ended => Self::Ended,
        }
    }

    pub fn to_proto(&self) -> i32 {
        match self {
            Self::Running => proto::DebuggerThreadStatus::Running.into(),
            Self::Stopped => proto::DebuggerThreadStatus::Stopped.into(),
            Self::Exited => proto::DebuggerThreadStatus::Exited.into(),
            Self::Ended => proto::DebuggerThreadStatus::Ended.into(),
        }
    }
}

pub struct DebugPanel {
    size: Pixels,
    pane: Entity<Pane>,
    focus_handle: FocusHandle,
    dap_store: Entity<DapStore>,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
    message_queue: HashMap<DebugAdapterClientId, VecDeque<OutputEvent>>,
    thread_states: BTreeMap<(DebugAdapterClientId, u64), Entity<ThreadState>>,
}

impl DebugPanel {
    pub fn new(
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let pane = cx.new(|cx| {
                let mut pane = Pane::new(
                    workspace.weak_handle(),
                    workspace.project().clone(),
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
                pane.set_close_pane_if_empty(false, cx);

                pane
            });

            let project = workspace.project().clone();
            let dap_store = project.read(cx).dap_store();

            let _subscriptions = vec![
                cx.observe(&pane, |_, _, cx| cx.notify()),
                cx.subscribe_in(&pane, window, Self::handle_pane_event),
                cx.subscribe_in(&dap_store, window, Self::on_dap_store_event),
                cx.subscribe_in(&project, window, {
                    move |this: &mut Self, _, event, window, cx| match event {
                        project::Event::DebugClientStarted((session_id, client_id)) => {
                            this.handle_debug_client_started(session_id, client_id, window, cx);
                        }
                        project::Event::DebugClientEvent {
                            session_id,
                            client_id,
                            message,
                        } => match message {
                            Message::Event(event) => {
                                this.handle_debug_client_events(
                                    session_id, client_id, event, window, cx,
                                );
                            }
                            Message::Request(request) => {
                                if StartDebugging::COMMAND == request.command {
                                    this.handle_start_debugging_request(
                                        session_id,
                                        client_id,
                                        request.seq,
                                        request.arguments.clone(),
                                        cx,
                                    );
                                } else if RunInTerminal::COMMAND == request.command {
                                    this.handle_run_in_terminal_request(
                                        session_id,
                                        client_id,
                                        request.seq,
                                        request.arguments.clone(),
                                        window,
                                        cx,
                                    );
                                }
                            }
                            _ => unreachable!(),
                        },
                        project::Event::DebugClientShutdown(client_id) => {
                            cx.emit(DebugPanelEvent::ClientShutdown(*client_id));

                            this.message_queue.remove(client_id);
                            this.thread_states
                                .retain(|&(client_id_, _), _| client_id_ != *client_id);

                            cx.notify();
                        }
                        project::Event::SetDebugClient(set_debug_client) => {
                            this.handle_set_debug_panel_item(set_debug_client, window, cx);
                        }
                        _ => {}
                    }
                }),
            ];

            let dap_store = project.read(cx).dap_store();

            let mut debug_panel = Self {
                pane,
                size: px(300.),
                _subscriptions,
                focus_handle: cx.focus_handle(),
                thread_states: Default::default(),
                message_queue: Default::default(),
                workspace: workspace.weak_handle(),
                dap_store: dap_store.clone(),
            };

            if let Some(mut dap_event_queue) = debug_panel
                .dap_store
                .clone()
                .update(cx, |this, _| this.remote_event_queue())
            {
                while let Some(dap_event) = dap_event_queue.pop_front() {
                    debug_panel.on_dap_store_event(
                        &debug_panel.dap_store.clone(),
                        &dap_event,
                        window,
                        cx,
                    );
                }
            }

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
                            this.active_debug_panel_item(cx)
                                .map(|item| {
                                    (
                                        true,
                                        item.update(cx, |this, cx| this.capabilities(cx))
                                            .supports_step_back
                                            .unwrap_or(false),
                                    )
                                })
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn message_queue(&self) -> &HashMap<DebugAdapterClientId, VecDeque<OutputEvent>> {
        &self.message_queue
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn dap_store(&self) -> Entity<DapStore> {
        self.dap_store.clone()
    }

    pub fn active_debug_panel_item(&self, cx: &Context<Self>) -> Option<Entity<DebugPanelItem>> {
        self.pane
            .read(cx)
            .active_item()
            .and_then(|panel| panel.downcast::<DebugPanelItem>())
    }

    pub fn debug_panel_item_by_client(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        cx: &mut Context<Self>,
    ) -> Option<Entity<DebugPanelItem>> {
        self.pane
            .read(cx)
            .items()
            .filter_map(|item| item.downcast::<DebugPanelItem>())
            .find(|item| {
                let item = item.read(cx);

                &item.client_id() == client_id && item.thread_id() == thread_id
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
            pane::Event::RemovedItem { item } => {
                let thread_panel = item.downcast::<DebugPanelItem>().unwrap();

                let thread_id = thread_panel.read(cx).thread_id();
                let session_id = thread_panel.read(cx).session().read(cx).id();
                let client_id = thread_panel.read(cx).client_id();

                self.thread_states.remove(&(client_id, thread_id));

                cx.notify();

                self.dap_store.update(cx, |store, cx| {
                    store
                        .terminate_threads(&session_id, &client_id, Some(vec![thread_id; 1]), cx)
                        .detach()
                });
            }
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
            pane::Event::ActivateItem { local, .. } => {
                if !local {
                    return;
                }

                if let Some(active_item) = self.pane.read(cx).active_item() {
                    if let Some(debug_item) = active_item.downcast::<DebugPanelItem>() {
                        debug_item.update(cx, |panel, cx| {
                            panel.go_to_current_stack_frame(window, cx);
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_start_debugging_request(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        seq: u64,
        request_args: Option<Value>,
        cx: &mut Context<Self>,
    ) {
        let args = if let Some(args) = request_args {
            serde_json::from_value(args.clone()).ok()
        } else {
            None
        };

        self.dap_store.update(cx, |store, cx| {
            store
                .respond_to_start_debugging(session_id, client_id, seq, args, cx)
                .detach_and_log_err(cx);
        });
    }

    fn handle_run_in_terminal_request(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        seq: u64,
        request_args: Option<Value>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let request_args = request_args.and_then(|request_args| {
            serde_json::from_value::<RunInTerminalRequestArguments>(request_args).ok()
        });
        let Some(request_args) = request_args else {
            self.dap_store.update(cx, |store, cx| {
                store
                    .respond_to_run_in_terminal(
                        session_id,
                        client_id,
                        false,
                        seq,
                        serde_json::to_value(ErrorResponse {
                            error: Some(dap::Message {
                                id: seq,
                                format:
                                    "Request arguments must be provided when spawnng debug terminal"
                                        .into(),
                                variables: None,
                                send_telemetry: None,
                                show_user: None,
                                url: None,
                                url_label: None,
                            }),
                        })
                        .ok(),
                        cx,
                    )
                    .detach_and_log_err(cx);
            });
            return;
        };

        let mut envs: HashMap<String, String> = Default::default();
        if let Some(Value::Object(env)) = request_args.env {
            for (key, value) in env {
                let value_str = match (key.as_str(), value) {
                    (_, Value::String(value)) => value,
                    _ => continue,
                };

                envs.insert(key, value_str);
            }
        }

        let terminal_task = self.workspace.update(cx, |workspace, cx| {
            let terminal_panel = workspace.panel::<TerminalPanel>(cx).unwrap();

            terminal_panel.update(cx, |terminal_panel, cx| {
                let mut args = request_args.args.clone();

                // Handle special case for NodeJS debug adapter
                // If only the Node binary path is provided, we set the command to None
                // This prevents the NodeJS REPL from appearing, which is not the desired behavior
                // The expected usage is for users to provide their own Node command, e.g., `node test.js`
                // This allows the NodeJS debug client to attach correctly
                let command = if args.len() > 1 {
                    Some(args.remove(0))
                } else {
                    None
                };

                let terminal_task = terminal_panel.add_terminal(
                    TerminalKind::Debug {
                        command,
                        args,
                        envs,
                        cwd: PathBuf::from(request_args.cwd),
                        title: request_args.title,
                    },
                    task::RevealStrategy::Always,
                    window,
                    cx,
                );

                cx.spawn(|_, mut cx| async move {
                    let pid_task = async move {
                        let terminal = terminal_task.await?;

                        terminal.read_with(&mut cx, |terminal, _| terminal.pty_info.pid())
                    };

                    pid_task.await
                })
            })
        });

        let session_id = *session_id;
        let client_id = *client_id;
        cx.spawn(|this, mut cx| async move {
            // Ensure a response is always sent, even in error cases,
            // to maintain proper communication with the debug adapter
            let (success, body) = match terminal_task {
                Ok(pid_task) => match pid_task.await {
                    Ok(pid) => (
                        true,
                        serde_json::to_value(RunInTerminalResponse {
                            process_id: None,
                            shell_process_id: pid.map(|pid| pid.as_u32() as u64),
                        })
                        .ok(),
                    ),
                    Err(error) => {
                        this.update(&mut cx, |this, cx| {
                            this.dap_store.update(cx, |_, cx| {
                                cx.emit(DapStoreEvent::Notification(error.to_string()));
                            })
                        })
                        .log_err();

                        (
                            false,
                            serde_json::to_value(ErrorResponse {
                                error: Some(dap::Message {
                                    id: seq,
                                    format: error.to_string(),
                                    variables: None,
                                    send_telemetry: None,
                                    show_user: None,
                                    url: None,
                                    url_label: None,
                                }),
                            })
                            .ok(),
                        )
                    }
                },
                Err(error) => (
                    false,
                    serde_json::to_value(ErrorResponse {
                        error: Some(dap::Message {
                            id: seq,
                            format: error.to_string(),
                            variables: None,
                            send_telemetry: None,
                            show_user: None,
                            url: None,
                            url_label: None,
                        }),
                    })
                    .ok(),
                ),
            };

            let respond_task = this.update(&mut cx, |this, cx| {
                this.dap_store.update(cx, |store, cx| {
                    store.respond_to_run_in_terminal(
                        &session_id,
                        &client_id,
                        success,
                        seq,
                        body,
                        cx,
                    )
                })
            });

            respond_task?.await
        })
        .detach_and_log_err(cx);
    }

    fn handle_debug_client_started(
        &self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(session) = self
            .dap_store
            .read(cx)
            .session_by_id(session_id)
            .and_then(|session| session.read(cx).as_local())
        else {
            return;
        };

        let session_id = *session_id;
        let client_id = *client_id;
        let workspace = self.workspace.clone();
        let request_type = session.configuration().request.clone();
        cx.spawn_in(window, |this, mut cx| async move {
            let task = this.update(&mut cx, |this, cx| {
                this.dap_store.update(cx, |store, cx| {
                    store.initialize(&session_id, &client_id, cx)
                })
            })?;

            task.await?;

            let result = match request_type {
                DebugRequestType::Launch => {
                    let task = this.update(&mut cx, |this, cx| {
                        this.dap_store
                            .update(cx, |store, cx| store.launch(&session_id, &client_id, cx))
                    });

                    task?.await
                }
                DebugRequestType::Attach(config) => {
                    if let Some(process_id) = config.process_id {
                        let task = this.update(&mut cx, |this, cx| {
                            this.dap_store.update(cx, |store, cx| {
                                store.attach(&session_id, &client_id, process_id, cx)
                            })
                        })?;

                        task.await
                    } else {
                        this.update_in(&mut cx, |this, window, cx| {
                            workspace.update(cx, |workspace, cx| {
                                workspace.toggle_modal(window, cx, |window, cx| {
                                    AttachModal::new(
                                        &session_id,
                                        &client_id,
                                        this.dap_store.clone(),
                                        window,
                                        cx,
                                    )
                                })
                            })
                        })?
                    }
                }
            };

            if result.is_err() {
                this.update(&mut cx, |debug_panel, cx| {
                    debug_panel.dap_store.update(cx, |store, cx| {
                        cx.emit(DapStoreEvent::Notification(
                            "Failed to start debug session".into(),
                        ));

                        store
                            .shutdown_session(&session_id, cx)
                            .detach_and_log_err(cx);
                    });
                })?;
            }

            result
        })
        .detach_and_log_err(cx);
    }

    fn handle_debug_client_events(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        event: &Events,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            Events::Initialized(event) => {
                self.handle_initialized_event(&session_id, &client_id, event, cx)
            }
            Events::Stopped(event) => {
                self.handle_stopped_event(&session_id, &client_id, event, window, cx)
            }
            Events::Continued(event) => self.handle_continued_event(&client_id, event, cx),
            Events::Exited(event) => self.handle_exited_event(&client_id, event, cx),
            Events::Terminated(event) => {
                self.handle_terminated_event(&session_id, &client_id, event, cx)
            }
            Events::Thread(event) => self.handle_thread_event(&client_id, event, cx),
            Events::Output(event) => self.handle_output_event(&client_id, event, cx),
            Events::Breakpoint(_) => {}
            Events::Module(event) => self.handle_module_event(&client_id, event, cx),
            Events::LoadedSource(event) => self.handle_loaded_source_event(&client_id, event, cx),
            Events::Capabilities(event) => {
                self.handle_capabilities_changed_event(session_id, client_id, event, cx);
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
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        capabilities: &Option<Capabilities>,
        cx: &mut Context<Self>,
    ) {
        if let Some(capabilities) = capabilities {
            self.dap_store.update(cx, |store, cx| {
                store.update_capabilities_for_client(&session_id, &client_id, capabilities, cx);
            });

            cx.emit(DebugPanelEvent::CapabilitiesChanged(*client_id));
        }

        let session_id = *session_id;
        let client_id = *client_id;

        cx.spawn(|this, mut cx| async move {
            this.update(&mut cx, |debug_panel, cx| {
                debug_panel.workspace.update(cx, |workspace, cx| {
                    workspace.project().update(cx, |project, cx| {
                        project.initial_send_breakpoints(&session_id, &client_id, cx)
                    })
                })
            })??
            .await;

            this.update(&mut cx, |debug_panel, cx| {
                debug_panel
                    .dap_store
                    .update(cx, |store, cx| store.configuration_done(&client_id, cx))
            })?
            .await
        })
        .detach_and_log_err(cx);
    }

    fn handle_continued_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ContinuedEvent,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DebugPanelEvent::Continued((*client_id, event.clone())));
    }

    fn handle_stopped_event(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        event: &StoppedEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread_id) = event.thread_id else {
            return;
        };

        let Some(session) = self
            .dap_store
            .read(cx)
            .session_by_id(session_id)
            .map(|session| session)
        else {
            return; // this can/should never happen
        };

        let client_id = *client_id;

        cx.spawn_in(window, {
            let event = event.clone();
            |this, mut cx| async move {
                let workspace = this.update_in(&mut cx, |this, window, cx| {
                    let thread_state = this
                        .thread_states
                        .entry((client_id, thread_id))
                        .or_insert(cx.new(|_| ThreadState::default()))
                        .clone();

                    thread_state.update(cx, |thread_state, _| {
                        thread_state.stopped = true;
                        thread_state.status = ThreadStatus::Stopped;
                    });

                    let existing_item = this.debug_panel_item_by_client(&client_id, thread_id, cx);
                    if existing_item.is_none() {
                        let debug_panel = cx.entity();
                        this.pane.update(cx, |pane, cx| {
                            let tab = cx.new(|cx| {
                                DebugPanelItem::new(
                                    session,
                                    &client_id,
                                    thread_id,
                                    thread_state,
                                    this.dap_store.clone(),
                                    &debug_panel,
                                    this.workspace.clone(),
                                    window,
                                    cx,
                                )
                            });

                            pane.add_item(Box::new(tab), true, true, None, window, cx);
                        });

                        if let Some(message_queue) = this.message_queue.get(&client_id) {
                            for output in message_queue.iter() {
                                cx.emit(DebugPanelEvent::Output((client_id, output.clone())));
                            }
                        }
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

                    this.workspace.clone()
                })?;

                cx.update(|window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.focus_panel::<Self>(window, cx);
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
        cx: &mut Context<Self>,
    ) {
        let thread_id = event.thread_id;

        if let Some(thread_state) = self.thread_states.get(&(*client_id, thread_id)) {
            if !thread_state.read(cx).stopped && event.reason == ThreadEventReason::Exited {
                const MESSAGE: &'static str = "Debug session exited without hitting breakpoints\n\nTry adding a breakpoint, or define the correct path mapping for your debugger.";

                self.dap_store.update(cx, |_, cx| {
                    cx.emit(DapStoreEvent::Notification(MESSAGE.into()));
                });
            };
        }

        if event.reason == ThreadEventReason::Started {
            self.thread_states
                .insert((*client_id, thread_id), cx.new(|_| ThreadState::default()));
        }

        cx.emit(DebugPanelEvent::Thread((*client_id, event.clone())));
        cx.notify();
    }

    fn handle_exited_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        _: &ExitedEvent,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DebugPanelEvent::Exited(*client_id));
    }

    fn handle_terminated_event(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        event: &Option<TerminatedEvent>,
        cx: &mut Context<Self>,
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
            if restart_args
                .as_ref()
                .is_some_and(|v| v.as_bool().unwrap_or(true))
            {
                store
                    .restart(&client_id, restart_args, cx)
                    .detach_and_log_err(cx);
            } else {
                store
                    .shutdown_session(&session_id, cx)
                    .detach_and_log_err(cx);
            }
        });

        cx.emit(DebugPanelEvent::Terminated(*client_id));
    }

    fn handle_output_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &OutputEvent,
        cx: &mut Context<Self>,
    ) {
        self.message_queue
            .entry(*client_id)
            .or_default()
            .push_back(event.clone());

        cx.emit(DebugPanelEvent::Output((*client_id, event.clone())));
    }

    fn on_dap_store_event(
        &mut self,
        _: &Entity<DapStore>,
        event: &project::dap_store::DapStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::dap_store::DapStoreEvent::SetDebugPanelItem(set_debug_panel_item) => {
                self.handle_set_debug_panel_item(set_debug_panel_item, window, cx);
            }
            project::dap_store::DapStoreEvent::UpdateDebugAdapter(debug_adapter_update) => {
                self.handle_debug_adapter_update(debug_adapter_update, window, cx);
            }
            project::dap_store::DapStoreEvent::UpdateThreadStatus(thread_status_update) => {
                self.handle_thread_status_update(thread_status_update, cx);
            }
            _ => {}
        }
    }

    pub(crate) fn handle_thread_status_update(
        &mut self,
        update: &proto::UpdateThreadStatus,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread_state) = self.thread_states.get_mut(&(
            DebugAdapterClientId::from_proto(update.client_id),
            update.thread_id,
        )) {
            thread_state.update(cx, |thread_state, _| {
                thread_state.status = ThreadStatus::from_proto(update.status());
            });

            cx.notify();
        }
    }

    pub(crate) fn handle_debug_adapter_update(
        &mut self,
        update: &UpdateDebugAdapter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let client_id = DebugAdapterClientId::from_proto(update.client_id);
        let thread_id = update.thread_id;

        let active_item = self
            .pane
            .read(cx)
            .active_item()
            .and_then(|item| item.downcast::<DebugPanelItem>());

        let search = self
            .pane
            .read(cx)
            .items()
            .filter_map(|item| item.downcast::<DebugPanelItem>())
            .find_map(|item_view| {
                let item = item_view.read(cx);

                if item.client_id() == client_id
                    && thread_id.map(|id| id == item.thread_id()).unwrap_or(true)
                {
                    Some((
                        item_view.clone(),
                        active_item
                            .as_ref()
                            .map_or(false, |this| this == &item_view),
                    ))
                } else {
                    None
                }
            });

        if let Some((debug_panel_item, is_active_item)) = search {
            debug_panel_item.update(cx, |this, cx| {
                this.update_adapter(update, window, cx);

                if is_active_item {
                    this.go_to_current_stack_frame(window, cx);
                }
            });
        }
    }

    pub(crate) fn handle_set_debug_panel_item(
        &mut self,
        payload: &SetDebuggerPanelItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let session_id = DebugSessionId::from_proto(payload.session_id);
        let Some(session) = self.dap_store.read(cx).session_by_id(&session_id) else {
            return;
        };

        let client_id = DebugAdapterClientId::from_proto(payload.client_id);
        let thread_id = payload.thread_id;
        let thread_state = payload.thread_state.clone().unwrap();
        let thread_state = cx.new(|_| ThreadState::from_proto(thread_state));

        let mut existing_item = self
            .pane
            .read(cx)
            .items()
            .filter_map(|item| item.downcast::<DebugPanelItem>())
            .find(|item| {
                let item = item.read(cx);

                item.client_id() == client_id && item.thread_id() == thread_id
            });

        let debug_panel_item = existing_item.get_or_insert_with(|| {
            self.thread_states
                .insert((client_id, thread_id), thread_state.clone());

            let debug_panel = cx.entity();
            let debug_panel_item = self.pane.update(cx, |pane, cx| {
                let debug_panel_item = cx.new(|cx| {
                    DebugPanelItem::new(
                        session,
                        &client_id,
                        thread_id,
                        thread_state,
                        self.dap_store.clone(),
                        &debug_panel,
                        self.workspace.clone(),
                        window,
                        cx,
                    )
                });

                self.dap_store.update(cx, |dap_store, cx| {
                    dap_store.add_remote_session(session_id, None, cx);
                    dap_store.add_client_to_session(session_id, client_id);
                });

                pane.add_item(
                    Box::new(debug_panel_item.clone()),
                    true,
                    true,
                    None,
                    window,
                    cx,
                );
                debug_panel_item
            });

            debug_panel_item
        });

        debug_panel_item.update(cx, |this, cx| {
            this.from_proto(payload, cx);
        });

        cx.notify();
    }

    fn handle_module_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &ModuleEvent,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DebugPanelEvent::Module((*client_id, event.clone())));
    }

    fn handle_loaded_source_event(
        &mut self,
        client_id: &DebugAdapterClientId,
        event: &LoadedSourceEvent,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DebugPanelEvent::LoadedSource((*client_id, event.clone())));
    }

    fn handle_capabilities_changed_event(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        event: &CapabilitiesEvent,
        cx: &mut Context<Self>,
    ) {
        self.dap_store.update(cx, |store, cx| {
            store.update_capabilities_for_client(session_id, client_id, &event.capabilities, cx);
        });

        cx.emit(DebugPanelEvent::CapabilitiesChanged(*client_id));
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}
impl EventEmitter<DebugPanelEvent> for DebugPanel {}
impl EventEmitter<project::Event> for DebugPanel {}

impl Focusable for DebugPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
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
}

impl Render for DebugPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DebugPanel")
            .track_focus(&self.focus_handle)
            .size_full()
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
                                        .on_click(move |_, window, cx| {
                                            window.dispatch_action(Box::new(Start), cx);
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
