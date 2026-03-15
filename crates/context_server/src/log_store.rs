use std::collections::VecDeque;

use collections::HashMap;
use futures::StreamExt as _;
use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};

use crate::ContextServerId;
use crate::types::{LoggingLevel, RpcDirection};

const SEND_LINE: &str = "\n// Send:";
const RECEIVE_LINE: &str = "\n// Receive:";
const MAX_STORED_LOG_ENTRIES: usize = 2000;

/// Initializes the global log store for context servers.
///
/// Unlike LSP logs (which are initialized as a side effect inside `language_tools::init`),
/// we use a more decoupled approach for MCP. By separating the core logging storage
/// from the UI component, the MCP client can safely log messages to its global store regardless
/// of whether the developer tools UI is ever registered or opened, preventing potential `cx.update_global` panics.
pub fn init(cx: &mut App) -> Entity<ContextServerLogStore> {
    let (io_tx, mut io_rx) =
        futures::channel::mpsc::unbounded::<(ContextServerId, crate::client::IoKind, String)>();

    let log_store = cx.new(|cx| {
        cx.spawn(async move |log_store, cx| {
            while let Some((server_id, io_kind, message)) = io_rx.next().await {
                if let Some(log_store) = log_store.upgrade() {
                    log_store.update(cx, |log_store: &mut ContextServerLogStore, cx| {
                        log_store.on_io(server_id, io_kind, &message, cx);
                    });
                }
            }
        })
        .detach();

        ContextServerLogStore::new(io_tx)
    });
    cx.set_global(GlobalContextServerLogStore(log_store.clone()));
    log_store
}

pub struct GlobalContextServerLogStore(pub Entity<ContextServerLogStore>);

impl Global for GlobalContextServerLogStore {}

#[derive(Debug)]
pub enum Event {
    NewContextServerLogEntry {
        id: ContextServerId,
        kind: LogKind,
        text: String,
    },
}

impl EventEmitter<Event> for ContextServerLogStore {}

pub struct ContextServerLogStore {
    pub servers: HashMap<ContextServerId, ContextServerState>,
    io_tx:
        futures::channel::mpsc::UnboundedSender<(ContextServerId, crate::client::IoKind, String)>,
}

#[derive(Clone)]
pub struct ContextServerState {
    pub name: String,
    pub log_messages: VecDeque<LogMessage>,
    pub stderr_messages: VecDeque<StderrMessage>,
    pub rpc_state: Option<RpcState>,
    pub toggled_log_kind: Option<LogKind>,
    pub log_level: LoggingLevel,
    _io_logs_subscription: Option<std::sync::Arc<crate::ContextServerIoSubscription>>,
}

impl std::fmt::Debug for ContextServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextServerState")
            .field("name", &self.name)
            .field("log_messages", &self.log_messages)
            .field("stderr_messages", &self.stderr_messages)
            .field("rpc_state", &self.rpc_state)
            .field("toggled_log_kind", &self.toggled_log_kind)
            .field("log_level", &self.log_level)
            .finish_non_exhaustive()
    }
}

pub trait Message: AsRef<str> {
    type Level: Copy + std::fmt::Debug;
    fn should_include(&self, _: Self::Level) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct LogMessage {
    message: String,
    pub level: LoggingLevel,
}

impl AsRef<str> for LogMessage {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl Message for LogMessage {
    type Level = LoggingLevel;

    fn should_include(&self, level: Self::Level) -> bool {
        self.level >= level
    }
}

#[derive(Debug, Clone)]
pub struct RpcMessage {
    message: String,
}

impl AsRef<str> for RpcMessage {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl Message for RpcMessage {
    type Level = ();
}

#[derive(Debug, Clone)]
pub struct StderrMessage {
    message: String,
}

impl AsRef<str> for StderrMessage {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl Message for StderrMessage {
    type Level = ();
}

#[derive(Debug, Clone)]
pub struct RpcState {
    pub rpc_messages: VecDeque<RpcMessage>,
    last_message_kind: Option<MessageKind>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MessageKind {
    Send,
    Receive,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LogKind {
    Rpc,
    Stderr,
    Trace,
    #[default]
    Logs,
    ServerInfo,
}

impl ContextServerLogStore {
    pub fn new(
        io_tx: futures::channel::mpsc::UnboundedSender<(
            ContextServerId,
            crate::client::IoKind,
            String,
        )>,
    ) -> Self {
        Self {
            servers: HashMap::default(),
            io_tx,
        }
    }

    pub fn add_context_server(
        &mut self,
        server: std::sync::Arc<crate::ContextServer>,
        name: String,
        cx: &mut Context<Self>,
    ) {
        let id = server.id();
        let io_tx = self.io_tx.clone();
        let server_id = id.clone();

        let io_logs_subscription = Some(std::sync::Arc::new(server.on_io(
            move |io_kind, message| {
                let _ = io_tx.unbounded_send((server_id.clone(), io_kind, message.to_string()));
            },
        )));

        self.servers
            .entry(id)
            .and_modify(|state| {
                state.name = name.clone();
                state._io_logs_subscription = io_logs_subscription.clone();
            })
            .or_insert_with(|| {
                cx.notify();
                ContextServerState {
                    name,
                    rpc_state: Some(RpcState {
                        rpc_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                        last_message_kind: None,
                    }),
                    log_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                    stderr_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                    toggled_log_kind: None,
                    log_level: LoggingLevel::Debug,
                    _io_logs_subscription: io_logs_subscription,
                }
            });
    }

    fn on_io(
        &mut self,
        id: ContextServerId,
        io_kind: crate::client::IoKind,
        message: &str,
        cx: &mut Context<Self>,
    ) {
        let is_received = match io_kind {
            crate::client::IoKind::Recv => true,
            crate::client::IoKind::Send => false,
            crate::client::IoKind::StdErr => {
                self.add_stderr(id, message.to_string(), cx);
                return;
            }
            crate::client::IoKind::ProtocolLog(level) => {
                self.add_log(id, level, message.to_string(), cx);
                return;
            }
        };

        let direction = if is_received {
            RpcDirection::Receive
        } else {
            RpcDirection::Send
        };

        let pretty_message = if let Ok(value) = serde_json::from_str::<serde_json::Value>(message) {
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| message.to_owned())
        } else {
            message.to_owned()
        };

        self.add_rpc(id, direction, pretty_message, cx);
    }

    pub fn remove_context_server(&mut self, id: ContextServerId, cx: &mut Context<Self>) {
        self.servers.remove(&id);
        cx.notify();
    }

    pub fn add_log(
        &mut self,
        id: ContextServerId,
        level: LoggingLevel,
        message: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(state) = self.servers.get_mut(&id) {
            let current_level = state.log_level;
            Self::push_new_message(
                &mut state.log_messages,
                LogMessage {
                    message: message.clone(),
                    level,
                },
                current_level,
            );
            cx.emit(Event::NewContextServerLogEntry {
                id,
                kind: LogKind::Logs,
                text: message,
            });
        }
    }

    pub fn add_rpc(
        &mut self,
        id: ContextServerId,
        direction: RpcDirection,
        message: String,
        cx: &mut Context<Self>,
    ) {
        let Some(state) = self
            .servers
            .get_mut(&id)
            .and_then(|state| state.rpc_state.as_mut())
        else {
            return;
        };

        let kind = match direction {
            RpcDirection::Send => MessageKind::Send,
            RpcDirection::Receive => MessageKind::Receive,
        };

        let rpc_log_lines = &mut state.rpc_messages;
        if state.last_message_kind != Some(kind) {
            while rpc_log_lines.len() >= MAX_STORED_LOG_ENTRIES {
                rpc_log_lines.pop_front();
            }
            let line_before_message = match kind {
                MessageKind::Send => SEND_LINE,
                MessageKind::Receive => RECEIVE_LINE,
            };

            rpc_log_lines.push_back(RpcMessage {
                message: line_before_message.to_string(),
            });

            cx.emit(Event::NewContextServerLogEntry {
                id: id.clone(),
                kind: LogKind::Rpc,
                text: line_before_message.to_string(),
            });
        }

        state.last_message_kind = Some(kind);

        while rpc_log_lines.len() >= MAX_STORED_LOG_ENTRIES {
            rpc_log_lines.pop_front();
        }

        rpc_log_lines.push_back(RpcMessage {
            message: message.trim().to_owned(),
        });

        cx.emit(Event::NewContextServerLogEntry {
            id,
            kind: LogKind::Rpc,
            text: message,
        });
    }

    pub fn enable_rpc_trace(&mut self, id: ContextServerId) {
        if let Some(state) = self.servers.get_mut(&id) {
            state.rpc_state.get_or_insert_with(|| RpcState {
                rpc_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                last_message_kind: None,
            });
        }
    }

    pub fn disable_rpc_trace(&mut self, id: ContextServerId) {
        if let Some(state) = self.servers.get_mut(&id) {
            state.rpc_state = None;
        }
    }

    pub fn get_server_state(&mut self, id: ContextServerId) -> Option<&mut ContextServerState> {
        self.servers.get_mut(&id)
    }

    pub fn server_logs(&self, id: &ContextServerId) -> Option<&VecDeque<LogMessage>> {
        Some(&self.servers.get(id)?.log_messages)
    }

    pub fn server_rpc(&self, id: &ContextServerId) -> Option<&VecDeque<RpcMessage>> {
        self.servers
            .get(id)?
            .rpc_state
            .as_ref()
            .map(|s| &s.rpc_messages)
    }

    pub fn server_stderr(&self, id: &ContextServerId) -> Option<&VecDeque<StderrMessage>> {
        Some(&self.servers.get(id)?.stderr_messages)
    }

    pub fn add_stderr(&mut self, id: ContextServerId, message: String, cx: &mut Context<Self>) {
        if let Some(state) = self.servers.get_mut(&id) {
            Self::push_new_message(
                &mut state.stderr_messages,
                StderrMessage {
                    message: message.clone(),
                },
                (),
            );
            cx.emit(Event::NewContextServerLogEntry {
                id,
                kind: LogKind::Stderr,
                text: message,
            });
        }
    }

    pub fn toggle_logs(
        &mut self,
        server_id: ContextServerId,
        enabled: bool,
        toggled_log_kind: LogKind,
    ) {
        if let Some(server_state) = self.servers.get_mut(&server_id) {
            if enabled {
                server_state.toggled_log_kind = Some(toggled_log_kind);
            } else {
                server_state.toggled_log_kind = None;
            }
        }
    }

    fn push_new_message<T: Message>(
        log_lines: &mut VecDeque<T>,
        message: T,
        current_severity: <T as Message>::Level,
    ) -> Option<String> {
        while log_lines.len() >= MAX_STORED_LOG_ENTRIES {
            log_lines.pop_front();
        }
        let visible = message.should_include(current_severity);

        let visible_message = visible.then(|| message.as_ref().to_string());
        log_lines.push_back(message);
        visible_message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ContextServerId;

    fn new_test_store(
        cx: &mut gpui::TestAppContext,
    ) -> (
        Entity<ContextServerLogStore>,
        futures::channel::mpsc::UnboundedReceiver<(ContextServerId, crate::client::IoKind, String)>,
    ) {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        let store = cx.new(|_| ContextServerLogStore::new(tx));
        (store, rx)
    }

    fn insert_test_server(store: &mut ContextServerLogStore, id: &ContextServerId, name: &str) {
        store.servers.insert(
            id.clone(),
            ContextServerState {
                name: name.into(),
                rpc_state: Some(RpcState {
                    rpc_messages: VecDeque::new(),
                    last_message_kind: None,
                }),
                log_messages: VecDeque::new(),
                stderr_messages: VecDeque::new(),
                toggled_log_kind: None,
                log_level: LoggingLevel::Debug,
                _io_logs_subscription: None,
            },
        );
    }

    #[test]
    fn test_should_include() {
        let debug_msg = LogMessage {
            message: "msg".into(),
            level: LoggingLevel::Debug,
        };
        let info_msg = LogMessage {
            message: "msg".into(),
            level: LoggingLevel::Info,
        };
        let error_msg = LogMessage {
            message: "msg".into(),
            level: LoggingLevel::Error,
        };

        assert!(debug_msg.should_include(LoggingLevel::Debug));
        assert!(!debug_msg.should_include(LoggingLevel::Info));
        assert!(!debug_msg.should_include(LoggingLevel::Error));

        assert!(info_msg.should_include(LoggingLevel::Debug));
        assert!(info_msg.should_include(LoggingLevel::Info));
        assert!(!info_msg.should_include(LoggingLevel::Error));

        assert!(error_msg.should_include(LoggingLevel::Debug));
        assert!(error_msg.should_include(LoggingLevel::Info));
        assert!(error_msg.should_include(LoggingLevel::Error));
    }

    #[test]
    fn test_rpc_message_always_included() {
        let rpc = RpcMessage {
            message: "anything".into(),
        };
        assert!(rpc.should_include(()));
    }

    #[gpui::test]
    fn test_log_store_limits(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            for i in 0..MAX_STORED_LOG_ENTRIES + 10 {
                store.add_log(id.clone(), LoggingLevel::Info, format!("Message {}", i), cx);
            }

            let logs = store.server_logs(&id).unwrap();
            assert_eq!(logs.len(), MAX_STORED_LOG_ENTRIES);
            assert_eq!(logs.front().unwrap().message, "Message 10");
            assert_eq!(
                logs.back().unwrap().message,
                format!("Message {}", MAX_STORED_LOG_ENTRIES + 9)
            );
        });
    }

    #[gpui::test]
    fn test_rpc_store_limits(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            for i in 0..MAX_STORED_LOG_ENTRIES + 10 {
                store.add_rpc(id.clone(), RpcDirection::Send, format!("rpc {}", i), cx);
            }

            let rpc = store.server_rpc(&id).unwrap();
            assert!(rpc.len() <= MAX_STORED_LOG_ENTRIES);
        });
    }

    #[gpui::test]
    fn test_rpc_message_boundaries(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.add_rpc(id.clone(), RpcDirection::Send, "req 1".into(), cx);
            store.add_rpc(id.clone(), RpcDirection::Send, "req 2".into(), cx);
            store.add_rpc(id.clone(), RpcDirection::Receive, "res 1".into(), cx);

            let rpc = store.server_rpc(&id).unwrap();
            // Should contain: SEND_LINE, "req 1", "req 2", RECEIVE_LINE, "res 1"
            assert_eq!(rpc.len(), 5);
            assert_eq!(rpc[0].message, SEND_LINE);
            assert_eq!(rpc[1].message, "req 1");
            assert_eq!(rpc[2].message, "req 2");
            assert_eq!(rpc[3].message, RECEIVE_LINE);
            assert_eq!(rpc[4].message, "res 1");
        });
    }

    #[gpui::test]
    fn test_rpc_direction_headers_not_repeated_for_same_direction(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.add_rpc(id.clone(), RpcDirection::Send, "a".into(), cx);
            store.add_rpc(id.clone(), RpcDirection::Send, "b".into(), cx);
            store.add_rpc(id.clone(), RpcDirection::Send, "c".into(), cx);

            let rpc = store.server_rpc(&id).unwrap();
            // One SEND header + 3 messages
            assert_eq!(rpc.len(), 4);
            assert_eq!(rpc[0].message, SEND_LINE);
        });
    }

    #[gpui::test]
    fn test_rpc_direction_alternating_inserts_headers(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.add_rpc(id.clone(), RpcDirection::Send, "s1".into(), cx);
            store.add_rpc(id.clone(), RpcDirection::Receive, "r1".into(), cx);
            store.add_rpc(id.clone(), RpcDirection::Send, "s2".into(), cx);
            store.add_rpc(id.clone(), RpcDirection::Receive, "r2".into(), cx);

            let rpc = store.server_rpc(&id).unwrap();
            // SEND, s1, RECEIVE, r1, SEND, s2, RECEIVE, r2
            assert_eq!(rpc.len(), 8);
            assert_eq!(rpc[0].message, SEND_LINE);
            assert_eq!(rpc[1].message, "s1");
            assert_eq!(rpc[2].message, RECEIVE_LINE);
            assert_eq!(rpc[3].message, "r1");
            assert_eq!(rpc[4].message, SEND_LINE);
            assert_eq!(rpc[5].message, "s2");
            assert_eq!(rpc[6].message, RECEIVE_LINE);
            assert_eq!(rpc[7].message, "r2");
        });
    }

    #[gpui::test]
    fn test_add_rpc_noop_when_rpc_disabled(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");
            store.disable_rpc_trace(id.clone());

            store.add_rpc(id.clone(), RpcDirection::Send, "ignored".into(), cx);

            assert!(store.server_rpc(&id).is_none());
        });
    }

    #[gpui::test]
    fn test_add_rpc_noop_for_unknown_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("nonexistent".into());

        store.update(cx, |store, cx| {
            // Should not panic
            store.add_rpc(id.clone(), RpcDirection::Send, "msg".into(), cx);
        });
    }

    #[gpui::test]
    fn test_add_log_noop_for_unknown_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("nonexistent".into());

        store.update(cx, |store, cx| {
            // Should not panic
            store.add_log(id.clone(), LoggingLevel::Info, "msg".into(), cx);
            assert!(store.server_logs(&id).is_none());
        });
    }

    #[gpui::test]
    fn test_remove_context_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");
            store.add_log(id.clone(), LoggingLevel::Info, "hello".into(), cx);
            store.add_stderr(id.clone(), "stderr line".into(), cx);
            assert!(store.server_logs(&id).is_some());
            assert!(store.server_stderr(&id).is_some());

            store.remove_context_server(id.clone(), cx);

            assert!(store.server_logs(&id).is_none());
            assert!(store.server_rpc(&id).is_none());
            assert!(store.server_stderr(&id).is_none());
            assert!(store.get_server_state(id).is_none());
        });
    }

    #[gpui::test]
    fn test_toggle_logs(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, _cx| {
            insert_test_server(store, &id, "test");

            assert_eq!(store.servers.get(&id).unwrap().toggled_log_kind, None);

            store.toggle_logs(id.clone(), true, LogKind::Rpc);
            assert_eq!(
                store.servers.get(&id).unwrap().toggled_log_kind,
                Some(LogKind::Rpc),
            );

            store.toggle_logs(id.clone(), true, LogKind::Logs);
            assert_eq!(
                store.servers.get(&id).unwrap().toggled_log_kind,
                Some(LogKind::Logs),
            );

            store.toggle_logs(id.clone(), false, LogKind::Logs);
            assert_eq!(store.servers.get(&id).unwrap().toggled_log_kind, None);
        });
    }

    #[gpui::test]
    fn test_toggle_logs_noop_for_unknown_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("nonexistent".into());

        store.update(cx, |store, _cx| {
            // Should not panic
            store.toggle_logs(id, true, LogKind::Rpc);
        });
    }

    #[gpui::test]
    fn test_enable_disable_rpc_trace(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");
            assert!(store.server_rpc(&id).is_some());

            store.disable_rpc_trace(id.clone());
            assert!(store.server_rpc(&id).is_none());

            store.enable_rpc_trace(id.clone());
            assert!(store.server_rpc(&id).is_some());
            assert!(store.server_rpc(&id).unwrap().is_empty());

            // Adding a message after re-enable should work
            store.add_rpc(id.clone(), RpcDirection::Send, "after re-enable".into(), cx);
            let rpc = store.server_rpc(&id).unwrap();
            assert_eq!(rpc.len(), 2); // header + message
        });
    }

    #[gpui::test]
    fn test_enable_rpc_trace_noop_for_unknown_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("nonexistent".into());

        store.update(cx, |store, _cx| {
            // Should not panic
            store.enable_rpc_trace(id.clone());
            store.disable_rpc_trace(id);
        });
    }

    #[gpui::test]
    fn test_on_io_recv_routes_to_rpc(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.on_io(
                id.clone(),
                crate::client::IoKind::Recv,
                r#"{"jsonrpc":"2.0"}"#,
                cx,
            );

            let rpc = store.server_rpc(&id).unwrap();
            assert!(!rpc.is_empty());
            assert_eq!(rpc[0].message, RECEIVE_LINE);
        });
    }

    #[gpui::test]
    fn test_on_io_send_routes_to_rpc(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.on_io(id.clone(), crate::client::IoKind::Send, "outgoing", cx);

            let rpc = store.server_rpc(&id).unwrap();
            assert!(!rpc.is_empty());
            assert_eq!(rpc[0].message, SEND_LINE);
            assert_eq!(rpc[1].message, "outgoing");
        });
    }

    #[gpui::test]
    fn test_on_io_stderr_routes_to_stderr(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.on_io(
                id.clone(),
                crate::client::IoKind::StdErr,
                "error output",
                cx,
            );

            let stderr = store.server_stderr(&id).unwrap();
            assert_eq!(stderr.len(), 1);
            assert_eq!(stderr[0].as_ref(), "error output");

            // Logs should be untouched
            let logs = store.server_logs(&id).unwrap();
            assert!(logs.is_empty());

            // RPC should be untouched
            let rpc = store.server_rpc(&id).unwrap();
            assert!(rpc.is_empty());
        });
    }

    #[gpui::test]
    fn test_on_io_protocol_log_routes_to_log(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.on_io(
                id.clone(),
                crate::client::IoKind::ProtocolLog(LoggingLevel::Warning),
                "protocol warning",
                cx,
            );

            let logs = store.server_logs(&id).unwrap();
            assert_eq!(logs.len(), 1);
            assert_eq!(logs[0].message, "protocol warning");
            assert_eq!(logs[0].level, LoggingLevel::Warning);
        });
    }

    #[gpui::test]
    fn test_on_io_pretty_prints_json(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.on_io(
                id.clone(),
                crate::client::IoKind::Recv,
                r#"{"a":1,"b":2}"#,
                cx,
            );

            let rpc = store.server_rpc(&id).unwrap();
            // The message should be pretty-printed (contains newlines)
            let message = &rpc[1].message;
            assert!(
                message.contains('\n'),
                "expected pretty-printed JSON, got: {}",
                message
            );
        });
    }

    #[gpui::test]
    fn test_on_io_non_json_passed_through(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.on_io(id.clone(), crate::client::IoKind::Send, "not json {{{", cx);

            let rpc = store.server_rpc(&id).unwrap();
            assert_eq!(rpc[1].message, "not json {{{");
        });
    }

    #[gpui::test]
    fn test_push_new_message_returns_none_when_filtered(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            // Set log level to Error so Debug messages are filtered
            store.servers.get_mut(&id).unwrap().log_level = LoggingLevel::Error;

            // add_log still stores the message (push_new_message stores all)
            store.add_log(id.clone(), LoggingLevel::Debug, "debug msg".into(), cx);

            let logs = store.server_logs(&id).unwrap();
            assert_eq!(logs.len(), 1);
            assert_eq!(logs[0].level, LoggingLevel::Debug);
            // The message is stored but should_include returns false at Error threshold
            assert!(!logs[0].should_include(LoggingLevel::Error));
        });
    }

    #[gpui::test]
    fn test_server_logs_and_rpc_return_none_for_unknown_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("nonexistent".into());

        store.update(cx, |store, _cx| {
            assert!(store.server_logs(&id).is_none());
            assert!(store.server_rpc(&id).is_none());
            assert!(store.server_stderr(&id).is_none());
        });
    }

    #[gpui::test]
    fn test_get_server_state_returns_none_for_unknown_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);

        store.update(cx, |store, _cx| {
            assert!(
                store
                    .get_server_state(ContextServerId("nope".into()))
                    .is_none()
            );
        });
    }

    #[gpui::test]
    fn test_stderr_store_limits(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            for i in 0..MAX_STORED_LOG_ENTRIES + 10 {
                store.add_stderr(id.clone(), format!("stderr {}", i), cx);
            }

            let stderr = store.server_stderr(&id).unwrap();
            assert_eq!(stderr.len(), MAX_STORED_LOG_ENTRIES);
            assert_eq!(stderr.front().unwrap().as_ref(), "stderr 10");
            assert_eq!(
                stderr.back().unwrap().as_ref(),
                format!("stderr {}", MAX_STORED_LOG_ENTRIES + 9)
            );
        });
    }

    #[gpui::test]
    fn test_add_stderr_noop_for_unknown_server(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("nonexistent".into());

        store.update(cx, |store, cx| {
            store.add_stderr(id.clone(), "msg".into(), cx);
            assert!(store.server_stderr(&id).is_none());
        });
    }

    #[gpui::test]
    fn test_add_rpc_trims_message_whitespace(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);
        let id = ContextServerId("test".into());

        store.update(cx, |store, cx| {
            insert_test_server(store, &id, "test");

            store.add_rpc(id.clone(), RpcDirection::Send, "  padded  \n".into(), cx);

            let rpc = store.server_rpc(&id).unwrap();
            assert_eq!(rpc[1].message, "padded");
        });
    }

    #[gpui::test]
    async fn test_add_server_restart(cx: &mut gpui::TestAppContext) {
        let (store, _rx) = new_test_store(cx);

        let server_cmd = crate::ContextServerCommand {
            path: "test".into(),
            args: vec![],
            env: None,
            timeout: None,
        };
        let id = ContextServerId("test_server".into());
        let server1 = std::sync::Arc::new(crate::ContextServer::stdio(
            id.clone(),
            server_cmd.clone(),
            None,
        ));

        store.update(cx, |store, cx| {
            store.add_context_server(server1, "Initial Name".to_string(), cx);

            // Add a log message to ensure it doesn't get cleared on restart
            store.add_log(id.clone(), LoggingLevel::Info, "First run log".into(), cx);
        });

        // Verify state after first add
        store.update(cx, |store, _| {
            let state = store.servers.get(&id).unwrap();
            assert_eq!(state.name, "Initial Name");
            assert_eq!(state.log_messages.len(), 1);
            assert!(state._io_logs_subscription.is_some());
        });

        // Simulate server restart (new ContextServer instance with the same ID)
        let server2 =
            std::sync::Arc::new(crate::ContextServer::stdio(id.clone(), server_cmd, None));

        store.update(cx, |store, cx| {
            store.add_context_server(server2, "Restarted Name".to_string(), cx);
        });

        // Verify state after restart
        store.update(cx, |store, _| {
            let state = store.servers.get(&id).unwrap();
            // Name should be updated
            assert_eq!(state.name, "Restarted Name");
            // The old logs should still be there (not cleared/overwritten by a new ContextServerState)
            assert_eq!(state.log_messages.len(), 1);
            assert_eq!(state.log_messages[0].message, "First run log");
            // There should still be an IO subscription active
            assert!(state._io_logs_subscription.is_some());
        });
    }
}
