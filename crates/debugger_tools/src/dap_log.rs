use dap::{
    adapters::DebugAdapterName,
    client::SessionId,
    debugger_settings::DebuggerSettings,
    transport::{IoKind, LogKind},
};
use editor::{Editor, EditorEvent};
use futures::{
    StreamExt,
    channel::mpsc::{UnboundedSender, unbounded},
};
use gpui::{
    App, AppContext, Context, Empty, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, SharedString, Styled, Subscription, WeakEntity, Window, actions, div,
};
use project::{
    Project,
    debugger::{dap_store, session::Session},
    search::SearchQuery,
};
use settings::Settings as _;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, VecDeque},
    sync::Arc,
};
use util::maybe;
use workspace::{
    ToolbarItemEvent, ToolbarItemView, Workspace,
    item::Item,
    searchable::{Direction, SearchEvent, SearchableItem, SearchableItemHandle},
    ui::{Button, Clickable, ContextMenu, Label, LabelCommon, PopoverMenu, h_flex},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum View {
    AdapterLogs,
    RpcMessages,
    InitializationSequence,
}

struct DapLogView {
    editor: Entity<Editor>,
    focus_handle: FocusHandle,
    log_store: Entity<LogStore>,
    editor_subscriptions: Vec<Subscription>,
    current_view: Option<(SessionId, View)>,
    project: Entity<Project>,
    _subscriptions: Vec<Subscription>,
}

struct LogStoreEntryIdentifier<'a> {
    session_id: SessionId,
    project: Cow<'a, WeakEntity<Project>>,
}
impl LogStoreEntryIdentifier<'_> {
    fn to_owned(&self) -> LogStoreEntryIdentifier<'static> {
        LogStoreEntryIdentifier {
            session_id: self.session_id,
            project: Cow::Owned(self.project.as_ref().clone()),
        }
    }
}

struct LogStoreMessage {
    id: LogStoreEntryIdentifier<'static>,
    kind: IoKind,
    command: Option<SharedString>,
    message: SharedString,
}

pub struct LogStore {
    projects: HashMap<WeakEntity<Project>, ProjectState>,
    rpc_tx: UnboundedSender<LogStoreMessage>,
    adapter_log_tx: UnboundedSender<LogStoreMessage>,
}

struct ProjectState {
    debug_sessions: BTreeMap<SessionId, DebugAdapterState>,
    _subscriptions: [gpui::Subscription; 2],
}

struct DebugAdapterState {
    id: SessionId,
    log_messages: VecDeque<SharedString>,
    rpc_messages: RpcMessages,
    session_label: SharedString,
    adapter_name: DebugAdapterName,
    has_adapter_logs: bool,
    is_terminated: bool,
}

struct RpcMessages {
    messages: VecDeque<SharedString>,
    last_message_kind: Option<MessageKind>,
    initialization_sequence: Vec<SharedString>,
    last_init_message_kind: Option<MessageKind>,
}

impl RpcMessages {
    const MESSAGE_QUEUE_LIMIT: usize = 255;

    fn new() -> Self {
        Self {
            last_message_kind: None,
            last_init_message_kind: None,
            messages: VecDeque::with_capacity(Self::MESSAGE_QUEUE_LIMIT),
            initialization_sequence: Vec::new(),
        }
    }
}

const SEND: &str = "// Send";
const RECEIVE: &str = "// Receive";

#[derive(Clone, Copy, PartialEq, Eq)]
enum MessageKind {
    Send,
    Receive,
}

impl MessageKind {
    fn label(&self) -> &'static str {
        match self {
            Self::Send => SEND,
            Self::Receive => RECEIVE,
        }
    }
}

impl DebugAdapterState {
    fn new(
        id: SessionId,
        adapter_name: DebugAdapterName,
        session_label: SharedString,
        has_adapter_logs: bool,
    ) -> Self {
        Self {
            id,
            log_messages: VecDeque::new(),
            rpc_messages: RpcMessages::new(),
            adapter_name,
            session_label,
            has_adapter_logs,
            is_terminated: false,
        }
    }
}

impl LogStore {
    pub fn new(cx: &Context<Self>) -> Self {
        let (rpc_tx, mut rpc_rx) = unbounded::<LogStoreMessage>();
        cx.spawn(async move |this, cx| {
            while let Some(message) = rpc_rx.next().await {
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| {
                        this.add_debug_adapter_message(message, cx);
                    });
                }

                smol::future::yield_now().await;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let (adapter_log_tx, mut adapter_log_rx) = unbounded::<LogStoreMessage>();
        cx.spawn(async move |this, cx| {
            while let Some(message) = adapter_log_rx.next().await {
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| {
                        this.add_debug_adapter_log(message, cx);
                    });
                }

                smol::future::yield_now().await;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
        Self {
            rpc_tx,
            adapter_log_tx,
            projects: HashMap::new(),
        }
    }

    pub fn add_project(&mut self, project: &Entity<Project>, cx: &mut Context<Self>) {
        self.projects.insert(
            project.downgrade(),
            ProjectState {
                _subscriptions: [
                    cx.observe_release(project, {
                        let weak_project = project.downgrade();
                        move |this, _, _| {
                            this.projects.remove(&weak_project);
                        }
                    }),
                    cx.subscribe(&project.read(cx).dap_store(), {
                        let weak_project = project.downgrade();
                        move |this, dap_store, event, cx| match event {
                            dap_store::DapStoreEvent::DebugClientStarted(session_id) => {
                                let session = dap_store.read(cx).session_by_id(session_id);
                                if let Some(session) = session {
                                    this.add_debug_session(
                                        LogStoreEntryIdentifier {
                                            project: Cow::Owned(weak_project.clone()),
                                            session_id: *session_id,
                                        },
                                        session,
                                        cx,
                                    );
                                }
                            }
                            dap_store::DapStoreEvent::DebugClientShutdown(session_id) => {
                                let id = LogStoreEntryIdentifier {
                                    project: Cow::Borrowed(&weak_project),
                                    session_id: *session_id,
                                };
                                if let Some(state) = this.get_debug_adapter_state(&id) {
                                    state.is_terminated = true;
                                }

                                this.clean_sessions(cx);
                            }
                            _ => {}
                        }
                    }),
                ],
                debug_sessions: Default::default(),
            },
        );
    }

    fn get_debug_adapter_state(
        &mut self,
        id: &LogStoreEntryIdentifier<'_>,
    ) -> Option<&mut DebugAdapterState> {
        self.projects
            .get_mut(&id.project)
            .and_then(|state| state.debug_sessions.get_mut(&id.session_id))
    }

    fn add_debug_adapter_message(
        &mut self,
        LogStoreMessage {
            id,
            kind: io_kind,
            command,
            message,
        }: LogStoreMessage,
        cx: &mut Context<Self>,
    ) {
        let Some(debug_client_state) = self.get_debug_adapter_state(&id) else {
            return;
        };

        let is_init_seq = command.as_ref().is_some_and(|command| {
            matches!(
                command.as_ref(),
                "attach" | "launch" | "initialize" | "configurationDone"
            )
        });

        let kind = match io_kind {
            IoKind::StdOut | IoKind::StdErr => MessageKind::Receive,
            IoKind::StdIn => MessageKind::Send,
        };

        let rpc_messages = &mut debug_client_state.rpc_messages;

        // Push a separator if the kind has changed
        if rpc_messages.last_message_kind != Some(kind) {
            Self::get_debug_adapter_entry(
                &mut rpc_messages.messages,
                id.to_owned(),
                kind.label().into(),
                LogKind::Rpc,
                cx,
            );
            rpc_messages.last_message_kind = Some(kind);
        }

        let entry = Self::get_debug_adapter_entry(
            &mut rpc_messages.messages,
            id.to_owned(),
            message,
            LogKind::Rpc,
            cx,
        );

        if is_init_seq {
            if rpc_messages.last_init_message_kind != Some(kind) {
                rpc_messages
                    .initialization_sequence
                    .push(SharedString::from(kind.label()));
                rpc_messages.last_init_message_kind = Some(kind);
            }
            rpc_messages.initialization_sequence.push(entry);
        }

        cx.notify();
    }

    fn add_debug_adapter_log(
        &mut self,
        LogStoreMessage {
            id,
            kind: io_kind,
            message,
            ..
        }: LogStoreMessage,
        cx: &mut Context<Self>,
    ) {
        let Some(debug_adapter_state) = self.get_debug_adapter_state(&id) else {
            return;
        };

        let message = match io_kind {
            IoKind::StdErr => format!("stderr: {message}").into(),
            _ => message,
        };

        Self::get_debug_adapter_entry(
            &mut debug_adapter_state.log_messages,
            id.to_owned(),
            message,
            LogKind::Adapter,
            cx,
        );
        cx.notify();
    }

    fn get_debug_adapter_entry(
        log_lines: &mut VecDeque<SharedString>,
        id: LogStoreEntryIdentifier<'static>,
        message: SharedString,
        kind: LogKind,
        cx: &mut Context<Self>,
    ) -> SharedString {
        if let Some(excess) = log_lines
            .len()
            .checked_sub(RpcMessages::MESSAGE_QUEUE_LIMIT)
            && excess > 0
        {
            log_lines.drain(..excess);
        }

        let format_messages = DebuggerSettings::get_global(cx).format_dap_log_messages;

        let entry = if format_messages {
            maybe!({
                serde_json::to_string_pretty::<serde_json::Value>(
                    &serde_json::from_str(&message).ok()?,
                )
                .ok()
            })
            .map(SharedString::from)
            .unwrap_or(message)
        } else {
            message
        };
        log_lines.push_back(entry.clone());

        cx.emit(Event::NewLogEntry {
            id,
            entry: entry.clone(),
            kind,
        });

        entry
    }

    fn add_debug_session(
        &mut self,
        id: LogStoreEntryIdentifier<'static>,
        session: Entity<Session>,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let project_entry = self.projects.get_mut(&id.project)?;
            let std::collections::btree_map::Entry::Vacant(state) =
                project_entry.debug_sessions.entry(id.session_id)
            else {
                return None;
            };

            let (adapter_name, session_label, has_adapter_logs) =
                session.read_with(cx, |session, _| {
                    (
                        session.adapter(),
                        session.label(),
                        session
                            .adapter_client()
                            .is_some_and(|client| client.has_adapter_logs()),
                    )
                });

            state.insert(DebugAdapterState::new(
                id.session_id,
                adapter_name,
                session_label
                    .unwrap_or_else(|| format!("Session {} (child)", id.session_id.0).into()),
                has_adapter_logs,
            ));

            self.clean_sessions(cx);

            let io_tx = self.rpc_tx.clone();

            let client = session.read(cx).adapter_client()?;
            let project = id.project.clone();
            let session_id = id.session_id;
            client.add_log_handler(
                move |kind, command, message| {
                    io_tx
                        .unbounded_send(LogStoreMessage {
                            id: LogStoreEntryIdentifier {
                                session_id,
                                project: project.clone(),
                            },
                            kind,
                            command: command.map(|command| command.to_owned().into()),
                            message: message.to_owned().into(),
                        })
                        .ok();
                },
                LogKind::Rpc,
            );

            let log_io_tx = self.adapter_log_tx.clone();
            let project = id.project;
            client.add_log_handler(
                move |kind, command, message| {
                    log_io_tx
                        .unbounded_send(LogStoreMessage {
                            id: LogStoreEntryIdentifier {
                                session_id,
                                project: project.clone(),
                            },
                            kind,
                            command: command.map(|command| command.to_owned().into()),
                            message: message.to_owned().into(),
                        })
                        .ok();
                },
                LogKind::Adapter,
            );
            Some(())
        });
    }

    fn clean_sessions(&mut self, cx: &mut Context<Self>) {
        self.projects.values_mut().for_each(|project| {
            let mut allowed_terminated_sessions = 10u32;
            project.debug_sessions.retain(|_, session| {
                if !session.is_terminated {
                    return true;
                }
                allowed_terminated_sessions = allowed_terminated_sessions.saturating_sub(1);
                allowed_terminated_sessions > 0
            });
        });

        cx.notify();
    }

    fn log_messages_for_session(
        &mut self,
        id: &LogStoreEntryIdentifier<'_>,
    ) -> Option<&mut VecDeque<SharedString>> {
        self.get_debug_adapter_state(id)
            .map(|state| &mut state.log_messages)
    }

    fn rpc_messages_for_session(
        &mut self,
        id: &LogStoreEntryIdentifier<'_>,
    ) -> Option<&mut VecDeque<SharedString>> {
        self.get_debug_adapter_state(id)
            .map(|state| &mut state.rpc_messages.messages)
    }

    fn initialization_sequence_for_session(
        &mut self,
        id: &LogStoreEntryIdentifier<'_>,
    ) -> Option<&Vec<SharedString>> {
        self.get_debug_adapter_state(id)
            .map(|state| &state.rpc_messages.initialization_sequence)
    }
}

pub struct DapLogToolbarItemView {
    log_view: Option<Entity<DapLogView>>,
}

impl DapLogToolbarItemView {
    pub fn new() -> Self {
        Self { log_view: None }
    }
}

impl Render for DapLogToolbarItemView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(log_view) = self.log_view.clone() else {
            return Empty.into_any_element();
        };

        let (menu_rows, current_session_id, project) = log_view.update(cx, |log_view, cx| {
            (
                log_view.menu_items(cx),
                log_view.current_view.map(|(session_id, _)| session_id),
                log_view.project.downgrade(),
            )
        });

        let current_client = current_session_id
            .and_then(|session_id| menu_rows.iter().find(|row| row.session_id == session_id));

        let dap_menu: PopoverMenu<_> = PopoverMenu::new("DapLogView")
            .anchor(gpui::Corner::TopLeft)
            .trigger(Button::new(
                "debug_client_menu_header",
                current_client
                    .map(|sub_item| {
                        Cow::Owned(format!(
                            "{} - {} - {}",
                            sub_item.adapter_name,
                            sub_item.session_label,
                            match sub_item.selected_entry {
                                View::AdapterLogs => ADAPTER_LOGS,
                                View::RpcMessages => RPC_MESSAGES,
                                View::InitializationSequence => INITIALIZATION_SEQUENCE,
                            }
                        ))
                    })
                    .unwrap_or_else(|| "No adapter selected".into()),
            ))
            .menu(move |window, cx| {
                let log_view = log_view.clone();
                let menu_rows = menu_rows.clone();
                let project = project.clone();
                ContextMenu::build(window, cx, move |mut menu, window, _cx| {
                    for row in menu_rows.into_iter() {
                        menu = menu.custom_row(move |_window, _cx| {
                            div()
                                .w_full()
                                .pl_2()
                                .child(
                                    Label::new(format!(
                                        "{} - {}",
                                        row.adapter_name, row.session_label
                                    ))
                                    .color(workspace::ui::Color::Muted),
                                )
                                .into_any_element()
                        });

                        if row.has_adapter_logs {
                            menu = menu.custom_entry(
                                move |_window, _cx| {
                                    div()
                                        .w_full()
                                        .pl_4()
                                        .child(Label::new(ADAPTER_LOGS))
                                        .into_any_element()
                                },
                                window.handler_for(&log_view, {
                                    let project = project.clone();
                                    let id = LogStoreEntryIdentifier {
                                        project: Cow::Owned(project),
                                        session_id: row.session_id,
                                    };
                                    move |view, window, cx| {
                                        view.show_log_messages_for_adapter(&id, window, cx);
                                    }
                                }),
                            );
                        }

                        menu = menu
                            .custom_entry(
                                move |_window, _cx| {
                                    div()
                                        .w_full()
                                        .pl_4()
                                        .child(Label::new(RPC_MESSAGES))
                                        .into_any_element()
                                },
                                window.handler_for(&log_view, {
                                    let project = project.clone();
                                    let id = LogStoreEntryIdentifier {
                                        project: Cow::Owned(project),
                                        session_id: row.session_id,
                                    };
                                    move |view, window, cx| {
                                        view.show_rpc_trace_for_server(&id, window, cx);
                                    }
                                }),
                            )
                            .custom_entry(
                                move |_window, _cx| {
                                    div()
                                        .w_full()
                                        .pl_4()
                                        .child(Label::new(INITIALIZATION_SEQUENCE))
                                        .into_any_element()
                                },
                                window.handler_for(&log_view, {
                                    let project = project.clone();
                                    let id = LogStoreEntryIdentifier {
                                        project: Cow::Owned(project),
                                        session_id: row.session_id,
                                    };
                                    move |view, window, cx| {
                                        view.show_initialization_sequence_for_server(
                                            &id, window, cx,
                                        );
                                    }
                                }),
                            );
                    }

                    menu
                })
                .into()
            });

        h_flex()
            .size_full()
            .child(dap_menu)
            .child(
                div()
                    .child(
                        Button::new("clear_log_button", "Clear").on_click(cx.listener(
                            |this, _, window, cx| {
                                if let Some(log_view) = this.log_view.as_ref() {
                                    log_view.update(cx, |log_view, cx| {
                                        log_view.editor.update(cx, |editor, cx| {
                                            editor.set_read_only(false);
                                            editor.clear(window, cx);
                                            editor.set_read_only(true);
                                        });
                                    })
                                }
                            },
                        )),
                    )
                    .ml_2(),
            )
            .into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for DapLogToolbarItemView {}

impl ToolbarItemView for DapLogToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::item::ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::ToolbarItemLocation {
        if let Some(item) = active_pane_item
            && let Some(log_view) = item.downcast::<DapLogView>()
        {
            self.log_view = Some(log_view);
            return workspace::ToolbarItemLocation::PrimaryLeft;
        }
        self.log_view = None;

        cx.notify();

        workspace::ToolbarItemLocation::Hidden
    }
}

impl DapLogView {
    pub fn new(
        project: Entity<Project>,
        log_store: Entity<LogStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (editor, editor_subscriptions) = Self::editor_for_logs(String::new(), window, cx);

        let focus_handle = cx.focus_handle();

        let events_subscriptions = cx.subscribe(&log_store, |log_view, _, event, cx| match event {
            Event::NewLogEntry { id, entry, kind } => {
                let is_current_view = match (log_view.current_view, *kind) {
                    (Some((i, View::AdapterLogs)), LogKind::Adapter)
                    | (Some((i, View::RpcMessages)), LogKind::Rpc)
                        if i == id.session_id =>
                    {
                        log_view.project == *id.project
                    }
                    _ => false,
                };
                if is_current_view {
                    log_view.editor.update(cx, |editor, cx| {
                        editor.set_read_only(false);
                        let last_point = editor.buffer().read(cx).len(cx);
                        editor.edit(
                            vec![
                                (last_point..last_point, entry.trim()),
                                (last_point..last_point, "\n"),
                            ],
                            cx,
                        );
                        editor.set_read_only(true);
                    });
                }
            }
        });
        let weak_project = project.downgrade();
        let state_info = log_store
            .read(cx)
            .projects
            .get(&weak_project)
            .and_then(|project| {
                project
                    .debug_sessions
                    .values()
                    .next_back()
                    .map(|session| (session.id, session.has_adapter_logs))
            });

        let mut this = Self {
            editor,
            focus_handle,
            project,
            log_store,
            editor_subscriptions,
            current_view: None,
            _subscriptions: vec![events_subscriptions],
        };

        if let Some((session_id, have_adapter_logs)) = state_info {
            let id = LogStoreEntryIdentifier {
                session_id,
                project: Cow::Owned(weak_project),
            };
            if have_adapter_logs {
                this.show_log_messages_for_adapter(&id, window, cx);
            } else {
                this.show_rpc_trace_for_server(&id, window, cx);
            }
        }

        this
    }

    fn editor_for_logs(
        log_contents: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<Editor>, Vec<Subscription>) {
        let editor = cx.new(|cx| {
            let mut editor = Editor::multi_line(window, cx);
            editor.set_text(log_contents, window, cx);
            editor.move_to_end(&editor::actions::MoveToEnd, window, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_input_enabled(false);
            editor.set_use_autoclose(false);
            editor.set_read_only(true);
            editor.set_show_edit_predictions(Some(false), window, cx);
            editor
        });
        let editor_subscription = cx.subscribe(
            &editor,
            |_, _, event: &EditorEvent, cx: &mut Context<DapLogView>| cx.emit(event.clone()),
        );
        let search_subscription = cx.subscribe(
            &editor,
            |_, _, event: &SearchEvent, cx: &mut Context<DapLogView>| cx.emit(event.clone()),
        );
        (editor, vec![editor_subscription, search_subscription])
    }

    fn menu_items(&self, cx: &App) -> Vec<DapMenuItem> {
        self.log_store
            .read(cx)
            .projects
            .get(&self.project.downgrade())
            .map_or_else(Vec::new, |state| {
                state
                    .debug_sessions
                    .values()
                    .rev()
                    .map(|state| DapMenuItem {
                        session_id: state.id,
                        adapter_name: state.adapter_name.clone(),
                        session_label: state.session_label.clone(),
                        has_adapter_logs: state.has_adapter_logs,
                        selected_entry: self
                            .current_view
                            .map_or(View::AdapterLogs, |(_, kind)| kind),
                    })
                    .collect::<Vec<_>>()
            })
    }

    fn show_rpc_trace_for_server(
        &mut self,
        id: &LogStoreEntryIdentifier<'_>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rpc_log = self.log_store.update(cx, |log_store, _| {
            log_store
                .rpc_messages_for_session(id)
                .map(|state| log_contents(state.iter().cloned()))
        });
        if let Some(rpc_log) = rpc_log {
            self.current_view = Some((id.session_id, View::RpcMessages));
            let (editor, editor_subscriptions) = Self::editor_for_logs(rpc_log, window, cx);
            let language = self.project.read(cx).languages().language_for_name("JSON");
            editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("log buffer should be a singleton")
                .update(cx, |_, cx| {
                    cx.spawn({
                        async move |buffer, cx| {
                            let language = language.await.ok();
                            buffer.update(cx, |buffer, cx| {
                                buffer.set_language(language, cx);
                            })
                        }
                    })
                    .detach_and_log_err(cx);
                });

            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }

        cx.focus_self(window);
    }

    fn show_log_messages_for_adapter(
        &mut self,
        id: &LogStoreEntryIdentifier<'_>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let message_log = self.log_store.update(cx, |log_store, _| {
            log_store
                .log_messages_for_session(id)
                .map(|state| log_contents(state.iter().cloned()))
        });
        if let Some(message_log) = message_log {
            self.current_view = Some((id.session_id, View::AdapterLogs));
            let (editor, editor_subscriptions) = Self::editor_for_logs(message_log, window, cx);
            editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("log buffer should be a singleton");

            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }

        cx.focus_self(window);
    }

    fn show_initialization_sequence_for_server(
        &mut self,
        id: &LogStoreEntryIdentifier<'_>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rpc_log = self.log_store.update(cx, |log_store, _| {
            log_store
                .initialization_sequence_for_session(id)
                .map(|state| log_contents(state.iter().cloned()))
        });
        if let Some(rpc_log) = rpc_log {
            self.current_view = Some((id.session_id, View::InitializationSequence));
            let (editor, editor_subscriptions) = Self::editor_for_logs(rpc_log, window, cx);
            let language = self.project.read(cx).languages().language_for_name("JSON");
            editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("log buffer should be a singleton")
                .update(cx, |_, cx| {
                    cx.spawn({
                        let buffer = cx.entity();
                        async move |_, cx| {
                            let language = language.await.ok();
                            buffer.update(cx, |buffer, cx| {
                                buffer.set_language(language, cx);
                            });
                        }
                    })
                    .detach();
                });

            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }

        cx.focus_self(window);
    }
}

fn log_contents(lines: impl Iterator<Item = SharedString>) -> String {
    lines.fold(String::new(), |mut acc, el| {
        acc.push_str(&el);
        acc.push('\n');
        acc
    })
}

#[derive(Clone, PartialEq)]
struct DapMenuItem {
    session_id: SessionId,
    session_label: SharedString,
    adapter_name: DebugAdapterName,
    has_adapter_logs: bool,
    selected_entry: View,
}

const ADAPTER_LOGS: &str = "Adapter Logs";
const RPC_MESSAGES: &str = "RPC Messages";
const INITIALIZATION_SEQUENCE: &str = "Initialization Sequence";

impl Render for DapLogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.editor.update(cx, |editor, cx| {
            editor.render(window, cx).into_any_element()
        })
    }
}

actions!(
    dev,
    [
        /// Opens the debug adapter protocol logs viewer.
        OpenDebugAdapterLogs
    ]
);

pub fn init(cx: &mut App) {
    let log_store = cx.new(|cx| LogStore::new(cx));

    cx.observe_new(move |workspace: &mut Workspace, window, cx| {
        let Some(_window) = window else {
            return;
        };

        let project = workspace.project();
        log_store.update(cx, |store, cx| {
            store.add_project(project, cx);
        });

        let log_store = log_store.clone();
        workspace.register_action(move |workspace, _: &OpenDebugAdapterLogs, window, cx| {
            workspace.add_item_to_active_pane(
                Box::new(cx.new(|cx| {
                    DapLogView::new(workspace.project().clone(), log_store.clone(), window, cx)
                })),
                None,
                true,
                window,
                cx,
            );
        });
    })
    .detach();
}

impl Item for DapLogView {
    type Event = EditorEvent;

    fn to_item_events(event: &Self::Event, f: impl FnMut(workspace::item::ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "DAP Logs".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn as_searchable(
        &self,
        handle: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }
}

impl SearchableItem for DapLogView {
    type Match = <Editor as SearchableItem>::Match;

    fn clear_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |e, cx| e.clear_matches(window, cx))
    }

    fn update_matches(
        &mut self,
        matches: &[Self::Match],
        active_match_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |e, cx| {
            e.update_matches(matches, active_match_index, window, cx)
        })
    }

    fn query_suggestion(&mut self, window: &mut Window, cx: &mut Context<Self>) -> String {
        self.editor
            .update(cx, |e, cx| e.query_suggestion(window, cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |e, cx| e.activate_match(index, matches, window, cx))
    }

    fn select_matches(
        &mut self,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |e, cx| e.select_matches(matches, window, cx))
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Vec<Self::Match>> {
        self.editor
            .update(cx, |e, cx| e.find_matches(query, window, cx))
    }

    fn replace(
        &mut self,
        _: &Self::Match,
        _: &SearchQuery,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        // Since DAP Log is read-only, it doesn't make sense to support replace operation.
    }

    fn supported_options(&self) -> workspace::searchable::SearchOptions {
        workspace::searchable::SearchOptions {
            case: true,
            word: true,
            regex: true,
            find_in_results: true,
            // DAP log is read-only.
            replacement: false,
            selection: false,
        }
    }
    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        self.editor.update(cx, |e, cx| {
            e.active_match_index(direction, matches, window, cx)
        })
    }
}

impl Focusable for DapLogView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

enum Event {
    NewLogEntry {
        id: LogStoreEntryIdentifier<'static>,
        entry: SharedString,
        kind: LogKind,
    },
}

impl EventEmitter<Event> for LogStore {}
impl EventEmitter<Event> for DapLogView {}
impl EventEmitter<EditorEvent> for DapLogView {}
impl EventEmitter<SearchEvent> for DapLogView {}

#[cfg(any(test, feature = "test-support"))]
impl LogStore {
    pub fn has_projects(&self) -> bool {
        !self.projects.is_empty()
    }

    pub fn contained_session_ids(&self, project: &WeakEntity<Project>) -> Vec<SessionId> {
        self.projects.get(project).map_or(vec![], |state| {
            state.debug_sessions.keys().copied().collect()
        })
    }

    pub fn rpc_messages_for_session_id(
        &self,
        project: &WeakEntity<Project>,
        session_id: SessionId,
    ) -> Vec<SharedString> {
        self.projects.get(project).map_or(vec![], |state| {
            state
                .debug_sessions
                .get(&session_id)
                .expect("This session should exist if a test is calling")
                .rpc_messages
                .messages
                .clone()
                .into()
        })
    }
}
