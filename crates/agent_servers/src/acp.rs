use acp_thread::{
    AgentConnection, AgentSessionInfo, AgentSessionList, AgentSessionListRequest,
    AgentSessionListResponse, ElicitationStore,
};
use action_log::ActionLog;
use agent_client_protocol::schema::{
    ProtocolVersion,
    v1::{self as acp, ErrorCode},
};
use agent_client_protocol::{Agent, Client, ConnectionTo, JsonRpcResponse, Lines, Responder};
use anyhow::anyhow;
use async_channel;
use collections::{HashMap, HashSet};
use feature_flags::{AcpBetaFeatureFlag, FeatureFlagAppExt as _};
use futures::channel::mpsc;
use futures::future::Shared;
use futures::io::BufReader;
use futures::{AsyncBufReadExt as _, Future, FutureExt as _, StreamExt as _};
use project::agent_server_store::{
    AgentServerCommand, AgentServerStore, AllAgentServersSettings, CustomAgentServerSettings,
};
use project::{AgentId, Project};
use remote::remote_client::Interactive;
use serde::Deserialize;
use settings::{AgentConfigOptionValue, SettingsStore};
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{any::Any, cell::RefCell, collections::VecDeque};
use task::{Shell, ShellBuilder, SpawnInTerminal};
use thiserror::Error;
use util::ResultExt as _;
use util::path_list::PathList;
use util::process::Child;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, SharedString, Subscription, Task, WeakEntity};

use acp_thread::{AcpThread, AuthRequired, LoadError, TerminalProviderEvent};
use terminal::TerminalBuilder;
use terminal::terminal_settings::{AlternateScroll, CursorShape};

use crate::{CURSOR_ID, GEMINI_ID};

pub const GEMINI_TERMINAL_AUTH_METHOD_ID: &str = "spawn-gemini-cli";
const PARAMETERIZED_MODEL_PICKER_META_KEY: &str = "parameterizedModelPicker";
const MAX_DEBUG_BACKLOG_MESSAGES: usize = 2000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcpDebugMessageDirection {
    Incoming,
    Outgoing,
    Stderr,
}

#[derive(Clone)]
pub enum AcpDebugMessageContent {
    Request {
        id: acp::RequestId,
        method: Arc<str>,
        params: Option<serde_json::Value>,
    },
    Response {
        id: acp::RequestId,
        result: Result<Option<serde_json::Value>, acp::Error>,
    },
    Notification {
        method: Arc<str>,
        params: Option<serde_json::Value>,
    },
    Stderr {
        line: Arc<str>,
    },
}

#[derive(Clone)]
pub struct AcpDebugMessage {
    pub direction: AcpDebugMessageDirection,
    pub message: AcpDebugMessageContent,
}

impl AcpDebugMessage {
    fn parse(direction: AcpDebugMessageDirection, line: &str) -> Option<Self> {
        if direction == AcpDebugMessageDirection::Stderr {
            return Some(Self {
                direction,
                message: AcpDebugMessageContent::Stderr {
                    line: Arc::from(line),
                },
            });
        }

        let value: serde_json::Value = serde_json::from_str(line).ok()?;
        let object = value.as_object()?;

        let parsed_id = object
            .get("id")
            .map(|raw| serde_json::from_value::<acp::RequestId>(raw.clone()));

        let message = if let Some(method) = object.get("method").and_then(|method| method.as_str())
        {
            match parsed_id {
                Some(Ok(id)) => AcpDebugMessageContent::Request {
                    id,
                    method: method.into(),
                    params: object.get("params").cloned(),
                },
                Some(Err(err)) => {
                    log::warn!("Skipping JSON-RPC message with unparsable id: {err}");
                    return None;
                }
                None => AcpDebugMessageContent::Notification {
                    method: method.into(),
                    params: object.get("params").cloned(),
                },
            }
        } else if let Some(parsed_id) = parsed_id {
            let id = match parsed_id {
                Ok(id) => id,
                Err(err) => {
                    log::warn!("Skipping JSON-RPC response with unparsable id: {err}");
                    return None;
                }
            };

            if let Some(error) = object.get("error") {
                let acp_error =
                    serde_json::from_value::<acp::Error>(error.clone()).unwrap_or_else(|err| {
                        log::warn!("Failed to deserialize ACP error: {err}");
                        acp::Error::internal_error().data(error.to_string())
                    });

                AcpDebugMessageContent::Response {
                    id,
                    result: Err(acp_error),
                }
            } else {
                AcpDebugMessageContent::Response {
                    id,
                    result: Ok(object.get("result").cloned()),
                }
            }
        } else {
            return None;
        };

        Some(Self { direction, message })
    }
}

#[derive(Default)]
struct AcpDebugLogState {
    messages: VecDeque<AcpDebugMessage>,
    subscribers: Vec<async_channel::Sender<AcpDebugMessage>>,
}

#[derive(Clone, Default)]
struct AcpDebugLog {
    state: Arc<Mutex<AcpDebugLogState>>,
}

impl AcpDebugLog {
    fn subscribe(
        &self,
    ) -> (
        Vec<AcpDebugMessage>,
        async_channel::Receiver<AcpDebugMessage>,
    ) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let backlog = state.messages.iter().cloned().collect();
        let (sender, receiver) = async_channel::unbounded();
        state.subscribers.push(sender);
        (backlog, receiver)
    }

    fn record_line(&self, direction: AcpDebugMessageDirection, line: &str) {
        let Some(message) = AcpDebugMessage::parse(direction, line) else {
            return;
        };
        self.record_message(message);
    }

    fn record_message(&self, message: AcpDebugMessage) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if state.messages.len() == MAX_DEBUG_BACKLOG_MESSAGES {
            state.messages.pop_front();
        }
        state.messages.push_back(message.clone());

        state.subscribers.retain(|sender| !sender.is_closed());
        for sender in &state.subscribers {
            sender.try_send(message.clone()).log_err();
        }
    }

    fn trailing_stderr(&self) -> Option<String> {
        let state = self.state.lock().ok()?;
        let mut lines = state
            .messages
            .iter()
            .rev()
            .take_while(|message| matches!(&message.message, AcpDebugMessageContent::Stderr { .. }))
            .filter_map(|message| match &message.message {
                AcpDebugMessageContent::Stderr { line } if !line.is_empty() => Some(line.as_ref()),
                _ => None,
            })
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return None;
        }

        lines.reverse();
        Some(lines.join("\n"))
    }
}

fn exited_load_error_with_stderr(status: ExitStatus, debug_log: &AcpDebugLog) -> LoadError {
    LoadError::Exited {
        status,
        stderr: debug_log.trailing_stderr().map(SharedString::from),
    }
}

#[derive(Debug, Error)]
#[error("Unsupported version")]
pub struct UnsupportedVersion;

/// Helper for flattening the nested `Result` shapes that come out of
/// `entity.update(cx, |_, cx| fallible_op(cx))` into a single `Result<T,
/// acp::Error>`.
///
/// `anyhow::Error` values get converted via `acp::Error::from`, which
/// downcasts an `acp::Error` back out of `anyhow` when present, so typed
/// errors like auth-required survive the trip.
trait FlattenAcpResult<T> {
    fn flatten_acp(self) -> Result<T, acp::Error>;
}

impl<T> FlattenAcpResult<T> for Result<Result<T, anyhow::Error>, anyhow::Error> {
    fn flatten_acp(self) -> Result<T, acp::Error> {
        match self {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(err)) => Err(err.into()),
            Err(err) => Err(err.into()),
        }
    }
}

impl<T> FlattenAcpResult<T> for Result<Result<T, acp::Error>, anyhow::Error> {
    fn flatten_acp(self) -> Result<T, acp::Error> {
        match self {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(err)) => Err(err),
            Err(err) => Err(err.into()),
        }
    }
}

/// Holds state needed by foreground work dispatched from background handler closures.
struct ClientContext {
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>>,
    request_elicitations: Entity<ElicitationStore>,
}

fn dispatch_queue_closed_error() -> acp::Error {
    acp::Error::internal_error().data("ACP foreground dispatch queue closed")
}

/// Work items sent from `Send` handler closures to the `!Send` foreground thread.
trait ForegroundWorkItem: Send {
    fn run(self: Box<Self>, cx: &mut AsyncApp, ctx: &ClientContext);
    fn reject(self: Box<Self>);
}

type ForegroundWork = Box<dyn ForegroundWorkItem>;

struct RequestForegroundWork<Req, Res>
where
    Req: Send + 'static,
    Res: JsonRpcResponse + Send + 'static,
{
    request: Req,
    responder: Responder<Res>,
    handler: fn(Req, Responder<Res>, &mut AsyncApp, &ClientContext),
}

impl<Req, Res> ForegroundWorkItem for RequestForegroundWork<Req, Res>
where
    Req: Send + 'static,
    Res: JsonRpcResponse + Send + 'static,
{
    fn run(self: Box<Self>, cx: &mut AsyncApp, ctx: &ClientContext) {
        let Self {
            request,
            responder,
            handler,
        } = *self;
        handler(request, responder, cx, ctx);
    }

    fn reject(self: Box<Self>) {
        let Self { responder, .. } = *self;
        log::error!("ACP foreground dispatch queue closed while handling inbound request");
        responder
            .respond_with_error(dispatch_queue_closed_error())
            .log_err();
    }
}

struct NotificationForegroundWork<Notif>
where
    Notif: Send + 'static,
{
    notification: Notif,
    connection: ConnectionTo<Agent>,
    handler: fn(Notif, &mut AsyncApp, &ClientContext),
}

impl<Notif> ForegroundWorkItem for NotificationForegroundWork<Notif>
where
    Notif: Send + 'static,
{
    fn run(self: Box<Self>, cx: &mut AsyncApp, ctx: &ClientContext) {
        let Self {
            notification,
            handler,
            ..
        } = *self;
        handler(notification, cx, ctx);
    }

    fn reject(self: Box<Self>) {
        let Self { connection, .. } = *self;
        log::error!("ACP foreground dispatch queue closed while handling inbound notification");
        connection
            .send_error_notification(dispatch_queue_closed_error())
            .log_err();
    }
}

fn enqueue_request<Req, Res>(
    dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
    request: Req,
    responder: Responder<Res>,
    handler: fn(Req, Responder<Res>, &mut AsyncApp, &ClientContext),
) where
    Req: Send + 'static,
    Res: JsonRpcResponse + Send + 'static,
{
    let work: ForegroundWork = Box::new(RequestForegroundWork {
        request,
        responder,
        handler,
    });
    if let Err(err) = dispatch_tx.unbounded_send(work) {
        err.into_inner().reject();
    }
}

fn enqueue_notification<Notif>(
    dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
    notification: Notif,
    connection: ConnectionTo<Agent>,
    handler: fn(Notif, &mut AsyncApp, &ClientContext),
) where
    Notif: Send + 'static,
{
    let work: ForegroundWork = Box::new(NotificationForegroundWork {
        notification,
        connection,
        handler,
    });
    if let Err(err) = dispatch_tx.unbounded_send(work) {
        err.into_inner().reject();
    }
}

pub struct AcpConnection {
    id: AgentId,
    telemetry_id: SharedString,
    agent_version: Option<SharedString>,
    connection: ConnectionTo<Agent>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    pending_sessions: Rc<RefCell<HashMap<acp::SessionId, PendingAcpSession>>>,
    auth_methods: Vec<acp::AuthMethod>,
    agent_server_store: WeakEntity<AgentServerStore>,
    agent_capabilities: acp::AgentCapabilities,
    request_elicitations: Entity<ElicitationStore>,
    defaults: AcpConnectionDefaults,
    child: Option<Child>,
    session_list: Option<Rc<AcpSessionList>>,
    debug_log: AcpDebugLog,
    _settings_subscription: Subscription,
    _io_task: Task<()>,
    _dispatch_task: Task<()>,
    _wait_task: Task<Result<()>>,
    _stderr_task: Task<Result<()>>,
}

#[derive(Clone, Default)]
struct AcpConnectionDefaults {
    mode: Rc<RefCell<Option<acp::SessionModeId>>>,
    config_options: Rc<RefCell<HashMap<String, AgentConfigOptionValue>>>,
}

impl AcpConnectionDefaults {
    fn new(
        mode: Option<acp::SessionModeId>,
        config_options: HashMap<String, AgentConfigOptionValue>,
    ) -> Self {
        Self {
            mode: Rc::new(RefCell::new(mode)),
            config_options: Rc::new(RefCell::new(config_options)),
        }
    }

    fn mode(&self) -> Option<acp::SessionModeId> {
        self.mode.borrow().clone()
    }

    fn config_option(&self, config_id: &str) -> Option<AgentConfigOptionValue> {
        self.config_options.borrow().get(config_id).cloned()
    }

    fn set(
        &self,
        mode: Option<acp::SessionModeId>,
        config_options: HashMap<String, AgentConfigOptionValue>,
    ) {
        *self.mode.borrow_mut() = mode;
        *self.config_options.borrow_mut() = config_options;
    }

    fn refresh_from_settings(&self, agent_id: &AgentId, cx: &App) {
        let Some(settings_store) = cx.try_global::<SettingsStore>() else {
            self.set(None, HashMap::default());
            return;
        };
        let settings = settings_store.get::<AllAgentServersSettings>(None);
        let Some(agent_settings) = settings.get(agent_id.as_ref()) else {
            self.set(None, HashMap::default());
            return;
        };

        let default_config_options = match agent_settings {
            CustomAgentServerSettings::Custom {
                default_config_options,
                ..
            }
            | CustomAgentServerSettings::Registry {
                default_config_options,
                ..
            } => default_config_options.clone(),
        };
        self.set(
            agent_settings.default_mode().map(acp::SessionModeId::new),
            default_config_options,
        );
    }

    fn observe_settings(&self, agent_id: AgentId, cx: &mut App) -> Subscription {
        if cx.try_global::<SettingsStore>().is_none() {
            return Subscription::new(|| {});
        }

        self.refresh_from_settings(&agent_id, cx);
        let defaults = self.clone();
        cx.observe_global::<SettingsStore>(move |cx| {
            defaults.refresh_from_settings(&agent_id, cx);
        })
    }
}

struct PendingAcpSession {
    task: Shared<Task<Result<Entity<AcpThread>, Arc<anyhow::Error>>>>,
    ref_count: usize,
}

struct SessionConfigResponse {
    modes: Option<acp::SessionModeState>,
    config_options: Option<Vec<acp::SessionConfigOption>>,
}

#[derive(Clone)]
struct ConfigOptions {
    config_options: Rc<RefCell<Vec<acp::SessionConfigOption>>>,
    tx: Rc<RefCell<watch::Sender<()>>>,
    rx: watch::Receiver<()>,
}

impl ConfigOptions {
    fn new(config_options: Rc<RefCell<Vec<acp::SessionConfigOption>>>) -> Self {
        let (tx, rx) = watch::channel(());
        Self {
            config_options,
            tx: Rc::new(RefCell::new(tx)),
            rx,
        }
    }
}

pub struct AcpSession {
    thread: WeakEntity<AcpThread>,
    suppress_abort_err: bool,
    session_modes: Option<Rc<RefCell<acp::SessionModeState>>>,
    config_options: Option<ConfigOptions>,
    ref_count: usize,
}

pub struct AcpSessionList {
    connection: ConnectionTo<Agent>,
    supports_delete: bool,
    updates_tx: async_channel::Sender<acp_thread::SessionListUpdate>,
    updates_rx: async_channel::Receiver<acp_thread::SessionListUpdate>,
}

impl AcpSessionList {
    fn new(connection: ConnectionTo<Agent>, supports_delete: bool) -> Self {
        let (tx, rx) = async_channel::unbounded();
        Self {
            connection,
            supports_delete,
            updates_tx: tx,
            updates_rx: rx,
        }
    }

    fn notify_update(&self) {
        self.updates_tx
            .try_send(acp_thread::SessionListUpdate::Refresh)
            .log_err();
    }

    fn send_info_update(&self, session_id: acp::SessionId, update: acp::SessionInfoUpdate) {
        self.updates_tx
            .try_send(acp_thread::SessionListUpdate::SessionInfo { session_id, update })
            .log_err();
    }
}

impl AgentSessionList for AcpSessionList {
    fn list_sessions(
        &self,
        request: AgentSessionListRequest,
        cx: &mut App,
    ) -> Task<Result<AgentSessionListResponse>> {
        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            let acp_request = acp::ListSessionsRequest::new()
                .cwd(request.cwd)
                .cursor(request.cursor);
            let response = conn
                .send_request(acp_request)
                .block_task()
                .await
                .map_err(map_acp_error)?;
            Ok(AgentSessionListResponse {
                sessions: response
                    .sessions
                    .into_iter()
                    .map(|s| AgentSessionInfo {
                        session_id: s.session_id,
                        work_dirs: Some(work_dirs_from_session_info(
                            s.cwd,
                            s.additional_directories,
                        )),
                        title: s.title.map(Into::into),
                        updated_at: s.updated_at.and_then(|date_str| {
                            chrono::DateTime::parse_from_rfc3339(&date_str)
                                .ok()
                                .map(|dt| dt.with_timezone(&chrono::Utc))
                        }),
                        created_at: None,
                        meta: s.meta,
                    })
                    .collect(),
                next_cursor: response.next_cursor,
                meta: response.meta,
            })
        })
    }

    fn supports_delete(&self) -> bool {
        self.supports_delete
    }

    fn delete_session(&self, session_id: &acp::SessionId, cx: &mut App) -> Task<Result<()>> {
        if !self.supports_delete() {
            return Task::ready(Err(anyhow::anyhow!("delete_session not supported")));
        }

        let conn = self.connection.clone();
        let updates_tx = self.updates_tx.clone();
        let session_id = session_id.clone();
        cx.foreground_executor().spawn(async move {
            conn.send_request(acp::DeleteSessionRequest::new(session_id))
                .block_task()
                .await
                .map_err(map_acp_error)?;
            updates_tx
                .try_send(acp_thread::SessionListUpdate::Refresh)
                .log_err();
            Ok(())
        })
    }

    fn watch(
        &self,
        _cx: &mut App,
    ) -> Option<async_channel::Receiver<acp_thread::SessionListUpdate>> {
        Some(self.updates_rx.clone())
    }

    fn notify_refresh(&self) {
        self.notify_update();
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

pub async fn connect(
    agent_id: AgentId,
    project: Entity<Project>,
    command: AgentServerCommand,
    agent_server_store: WeakEntity<AgentServerStore>,
    default_mode: Option<acp::SessionModeId>,
    default_config_options: HashMap<String, AgentConfigOptionValue>,
    cx: &mut AsyncApp,
) -> Result<Rc<dyn AgentConnection>> {
    let conn = AcpConnection::stdio(
        agent_id,
        project,
        command.clone(),
        agent_server_store,
        default_mode,
        default_config_options,
        cx,
    )
    .await?;
    Ok(Rc::new(conn) as _)
}

const MINIMUM_SUPPORTED_VERSION: ProtocolVersion = ProtocolVersion::V1;

/// Build a `Client` connection over `transport` with Zed's full
/// agent→client handler set wired up.
///
/// All incoming requests and notifications are forwarded to the foreground
/// dispatch queue via `dispatch_tx`, where they are handled by the
/// `handle_*` functions on a GPUI context. The returned future drives the
/// connection and completes when the transport closes; callers are expected
/// to spawn it on a background executor and hold the task for the lifetime
/// of the connection. The `connection_tx` oneshot receives the
/// `ConnectionTo<Agent>` handle as soon as the builder runs its `main_fn`.
fn connect_client_future(
    name: &'static str,
    transport: impl agent_client_protocol::ConnectTo<Client> + 'static,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
    connection_tx: futures::channel::oneshot::Sender<ConnectionTo<Agent>>,
) -> impl Future<Output = Result<(), acp::Error>> {
    // Each handler forwards its inputs onto the foreground dispatch queue.
    // The SDK requires the closure to be `Send`, so we move a clone of
    // `dispatch_tx` into each one.
    macro_rules! on_request {
        ($handler:ident) => {{
            let dispatch_tx = dispatch_tx.clone();
            async move |req, responder, _connection| {
                enqueue_request(&dispatch_tx, req, responder, $handler);
                Ok(())
            }
        }};
    }
    macro_rules! on_notification {
        ($handler:ident) => {{
            let dispatch_tx = dispatch_tx.clone();
            async move |notif, connection| {
                enqueue_notification(&dispatch_tx, notif, connection, $handler);
                Ok(())
            }
        }};
    }

    Client
        .builder()
        .name(name)
        // --- Request handlers (agent→client) ---
        .on_receive_request(
            on_request!(handle_request_permission),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_write_text_file),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_read_text_file),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_create_terminal),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_kill_terminal),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_release_terminal),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_terminal_output),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_wait_for_terminal_exit),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_create_elicitation),
            agent_client_protocol::on_receive_request!(),
        )
        // --- Notification handlers (agent→client) ---
        .on_receive_notification(
            on_notification!(handle_session_notification),
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_notification(
            on_notification!(handle_complete_elicitation),
            agent_client_protocol::on_receive_notification!(),
        )
        .connect_with(
            transport,
            move |connection: ConnectionTo<Agent>| async move {
                if connection_tx.send(connection).is_err() {
                    log::error!("failed to send ACP connection handle — receiver was dropped");
                }
                // Keep the connection alive until the transport closes.
                futures::future::pending::<Result<(), acp::Error>>().await
            },
        )
}

fn client_capabilities_for_agent(agent_id: &AgentId) -> acp::ClientCapabilities {
    let mut meta = acp::Meta::from_iter([
        ("terminal_output".into(), true.into()),
        ("terminal-auth".into(), true.into()),
    ]);

    if agent_id.as_ref() == CURSOR_ID {
        meta.insert(PARAMETERIZED_MODEL_PICKER_META_KEY.into(), true.into());
    }

    acp::ClientCapabilities::new()
        .fs(acp::FileSystemCapabilities::new()
            .read_text_file(true)
            .write_text_file(true))
        .terminal(true)
        .auth(acp::AuthCapabilities::new().terminal(true))
        .session(
            acp::ClientSessionCapabilities::new().config_options(
                acp::SessionConfigOptionsCapabilities::new()
                    .boolean(acp::BooleanConfigOptionCapabilities::new()),
            ),
        )
        .elicitation(
            acp::ElicitationCapabilities::new()
                .form(acp::ElicitationFormCapabilities::new())
                .url(acp::ElicitationUrlCapabilities::new()),
        )
        .meta(meta)
}

impl AcpConnection {
    pub fn subscribe_debug_messages(
        &self,
    ) -> (
        Vec<AcpDebugMessage>,
        async_channel::Receiver<AcpDebugMessage>,
    ) {
        self.debug_log.subscribe()
    }

    pub async fn stdio(
        agent_id: AgentId,
        project: Entity<Project>,
        command: AgentServerCommand,
        agent_server_store: WeakEntity<AgentServerStore>,
        default_mode: Option<acp::SessionModeId>,
        default_config_options: HashMap<String, AgentConfigOptionValue>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let root_dir = project.read_with(cx, |project, cx| {
            project
                .default_path_list(cx)
                .ordered_paths()
                .next()
                .cloned()
        });
        let original_command = command.clone();
        let (path, args, env) = project
            .read_with(cx, |project, cx| {
                project.remote_client().and_then(|client| {
                    let template = client
                        .read(cx)
                        .build_command(
                            Some(command.path.display().to_string()),
                            &command.args,
                            &command.env.clone().into_iter().flatten().collect(),
                            root_dir.as_ref().map(|path| path.display().to_string()),
                            None,
                            Interactive::No,
                        )
                        .log_err()?;
                    Some((template.program, template.args, template.env))
                })
            })
            .unwrap_or_else(|| {
                (
                    command.path.display().to_string(),
                    command.args,
                    command.env.unwrap_or_default(),
                )
            });

        let builder = ShellBuilder::new(&Shell::System, cfg!(windows)).non_interactive();
        let mut child = builder.build_std_command(Some(path.clone()), &args);
        child.envs(env.clone());
        if let Some(cwd) = project.read_with(cx, |project, _cx| {
            if project.is_local() {
                root_dir.as_ref()
            } else {
                None
            }
        }) {
            child.current_dir(cwd);
        }
        let mut child = Child::spawn(child, Stdio::piped(), Stdio::piped(), Stdio::piped())?;

        let stdout = child.stdout.take().context("Failed to take stdout")?;
        let stdin = child.stdin.take().context("Failed to take stdin")?;
        let stderr = child.stderr.take().context("Failed to take stderr")?;
        log::debug!("Spawning external agent server: {:?}, {:?}", path, args);
        log::trace!("Spawned (pid: {})", child.id());

        let sessions = Rc::new(RefCell::new(HashMap::default()));
        let debug_log = AcpDebugLog::default();

        let (release_channel, version): (Option<&str>, String) = cx.update(|cx| {
            (
                release_channel::ReleaseChannel::try_global(cx)
                    .map(|release_channel| release_channel.display_name()),
                release_channel::AppVersion::global(cx).to_string(),
            )
        });

        let client_session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>> =
            Rc::new(RefCell::new(None));
        let request_elicitations = cx.new(|_| ElicitationStore::default());

        // Set up the foreground dispatch channel for bridging Send handler
        // closures to the !Send foreground thread.
        let (dispatch_tx, dispatch_rx) = mpsc::unbounded::<ForegroundWork>();

        let incoming_lines = futures::io::BufReader::new(stdout).lines();
        let tapped_incoming = incoming_lines.inspect({
            let debug_log = debug_log.clone();
            move |result| match result {
                Ok(line) => debug_log.record_line(AcpDebugMessageDirection::Incoming, line),
                Err(err) => {
                    log::warn!("ACP transport read error: {err}");
                }
            }
        });

        let tapped_outgoing = futures::sink::unfold(
            (Box::pin(stdin), debug_log.clone()),
            async move |(mut writer, debug_log), line: String| {
                use futures::AsyncWriteExt;
                debug_log.record_line(AcpDebugMessageDirection::Outgoing, &line);
                let mut bytes = line.into_bytes();
                bytes.push(b'\n');
                writer.write_all(&bytes).await?;
                Ok::<_, std::io::Error>((writer, debug_log))
            },
        );

        let transport = Lines::new(tapped_outgoing, tapped_incoming);

        let stderr_task = cx.background_spawn({
            let debug_log = debug_log.clone();
            async move {
                let mut stderr = BufReader::new(stderr);
                let mut line = String::new();
                while let Ok(n) = stderr.read_line(&mut line).await
                    && n > 0
                {
                    let trimmed = line.trim_end_matches(['\n', '\r']);
                    log::warn!("agent stderr: {trimmed}");
                    debug_log.record_line(AcpDebugMessageDirection::Stderr, trimmed);
                    line.clear();
                }
                Ok(())
            }
        });

        // `connect_client_future` installs the production handler set and
        // hands us back both the connection-future (to run on a background
        // executor) and a oneshot receiver that produces the
        // `ConnectionTo<Agent>` once the transport handshake is ready.
        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let connection_future =
            connect_client_future("zed", transport, dispatch_tx.clone(), connection_tx);
        let io_task = cx.background_spawn(async move {
            if let Err(err) = connection_future.await {
                log::error!("ACP connection error: {err}");
            }
        });

        let connection_rx = async move {
            connection_rx
                .await
                .context("Failed to receive ACP connection handle")
        }
        .boxed_local();
        let status_fut = child
            .status()
            .map({
                let debug_log = debug_log.clone();
                move |status| match status {
                    Ok(status) => Ok(exited_load_error_with_stderr(status, &debug_log)),
                    Err(err) => Err(anyhow!("failed to wait for agent server exit: {err}")),
                }
            })
            .boxed_local();
        let (connection, status_fut) = match futures::future::select(connection_rx, status_fut)
            .await
        {
            futures::future::Either::Left((connection, status_fut)) => (connection?, status_fut),
            futures::future::Either::Right((load_error, _connection_rx)) => {
                return Err(load_error?.into());
            }
        };

        // Set up the foreground dispatch loop to process work items from handlers.
        let dispatch_context = ClientContext {
            sessions: sessions.clone(),
            session_list: client_session_list.clone(),
            request_elicitations: request_elicitations.clone(),
        };
        let dispatch_task = cx.spawn({
            let mut dispatch_rx = dispatch_rx;
            async move |cx| {
                while let Some(work) = dispatch_rx.next().await {
                    work.run(cx, &dispatch_context);
                }
            }
        });

        let initialize_response = connection
            .send_request(
                acp::InitializeRequest::new(ProtocolVersion::V1)
                    .client_capabilities(client_capabilities_for_agent(&agent_id))
                    .client_info(
                        acp::Implementation::new("zed", version)
                            .title(release_channel.map(ToOwned::to_owned)),
                    ),
            )
            .block_task()
            .boxed_local();
        let (response, status_fut) =
            match futures::future::select(initialize_response, status_fut).await {
                futures::future::Either::Left((Ok(response), status_fut)) => (response, status_fut),
                futures::future::Either::Left((Err(error), status_fut)) => {
                    let timer = cx
                        .background_executor()
                        .timer(std::time::Duration::from_millis(250))
                        .boxed_local();
                    if let futures::future::Either::Left((load_error, _timer)) =
                        futures::future::select(status_fut, timer).await
                    {
                        return Err(load_error?.into());
                    }

                    return Err(error.into());
                }
                futures::future::Either::Right((load_error, _initialize_response)) => {
                    return Err(load_error?.into());
                }
            };

        if response.protocol_version < MINIMUM_SUPPORTED_VERSION {
            return Err(UnsupportedVersion.into());
        }

        let wait_task = cx.spawn({
            let sessions = sessions.clone();
            async move |cx| {
                let load_error = status_fut.await?;
                emit_load_error_to_all_sessions(&sessions, load_error, cx);
                anyhow::Ok(())
            }
        });

        let agent_info = response.agent_info;
        let telemetry_id = agent_info
            .as_ref()
            // Use the one the agent provides if we have one
            .map(|info| SharedString::from(info.name.clone()))
            // Otherwise, just use the name
            .unwrap_or_else(|| agent_id.0.clone());
        let agent_version = agent_info
            .and_then(|info| (!info.version.is_empty()).then(|| SharedString::from(info.version)));
        let agent_supports_delete = response
            .agent_capabilities
            .session_capabilities
            .delete
            .is_some();

        let session_list = if response
            .agent_capabilities
            .session_capabilities
            .list
            .is_some()
        {
            let list = Rc::new(AcpSessionList::new(
                connection.clone(),
                agent_supports_delete,
            ));
            *client_session_list.borrow_mut() = Some(list.clone());
            Some(list)
        } else {
            None
        };

        // TODO: Remove this override once Google team releases their official auth methods
        let auth_methods = if agent_id.0.as_ref() == GEMINI_ID {
            let mut gemini_args = original_command.args.clone();
            gemini_args.retain(|a| a != "--experimental-acp" && a != "--acp");
            let value = serde_json::json!({
                "label": "gemini /auth",
                "command": original_command.path.to_string_lossy(),
                "args": gemini_args,
                "env": original_command.env.unwrap_or_default(),
            });
            let meta = acp::Meta::from_iter([("terminal-auth".to_string(), value)]);
            vec![acp::AuthMethod::Agent(
                acp::AuthMethodAgent::new(GEMINI_TERMINAL_AUTH_METHOD_ID, "Login")
                    .description("Login with your Google or Vertex AI account")
                    .meta(meta),
            )]
        } else {
            response.auth_methods
        };
        let defaults = AcpConnectionDefaults::new(default_mode, default_config_options);
        let settings_subscription = cx.update({
            let agent_id = agent_id.clone();
            let defaults = defaults.clone();
            move |cx| defaults.observe_settings(agent_id, cx)
        });

        Ok(Self {
            id: agent_id,
            auth_methods,
            agent_server_store,
            connection,
            telemetry_id,
            agent_version,
            sessions,
            pending_sessions: Rc::new(RefCell::new(HashMap::default())),
            agent_capabilities: response.agent_capabilities,
            request_elicitations,
            defaults,
            session_list,
            debug_log,
            _settings_subscription: settings_subscription,
            _io_task: io_task,
            _dispatch_task: dispatch_task,
            _wait_task: wait_task,
            _stderr_task: stderr_task,
            child: Some(child),
        })
    }

    pub fn prompt_capabilities(&self) -> &acp::PromptCapabilities {
        &self.agent_capabilities.prompt_capabilities
    }

    #[cfg(any(test, feature = "test-support"))]
    fn new_for_test(
        connection: ConnectionTo<Agent>,
        sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
        agent_capabilities: acp::AgentCapabilities,
        request_elicitations: Entity<ElicitationStore>,
        agent_server_store: WeakEntity<AgentServerStore>,
        io_task: Task<()>,
        dispatch_task: Task<()>,
        cx: &mut App,
    ) -> Self {
        let agent_id = AgentId::new("test");
        let defaults = AcpConnectionDefaults::default();
        let settings_subscription = defaults.observe_settings(agent_id.clone(), cx);

        Self {
            id: agent_id,
            telemetry_id: "test".into(),
            agent_version: None,
            connection,
            sessions,
            pending_sessions: Rc::new(RefCell::new(HashMap::default())),
            auth_methods: vec![],
            agent_server_store,
            agent_capabilities,
            request_elicitations,
            defaults,
            child: None,
            session_list: None,
            debug_log: AcpDebugLog::default(),
            _settings_subscription: settings_subscription,
            _io_task: io_task,
            _dispatch_task: dispatch_task,
            _wait_task: Task::ready(Ok(())),
            _stderr_task: Task::ready(Ok(())),
        }
    }

    fn session_directories_from_work_dirs(
        &self,
        work_dirs: &PathList,
    ) -> Result<SessionDirectories> {
        let supports_additional_directories = self.supports_session_additional_directories();
        session_directories_from_work_dirs(work_dirs, supports_additional_directories)
    }

    fn open_or_create_session(
        self: Rc<Self>,
        session_id: acp::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        rpc_call: impl FnOnce(
            ConnectionTo<Agent>,
            acp::SessionId,
            SessionDirectories,
        )
            -> futures::future::LocalBoxFuture<'static, Result<SessionConfigResponse>>
        + 'static,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        // Check `pending_sessions` before `sessions` because the session is now
        // inserted into `sessions` before the load RPC completes (so that
        // notifications dispatched during history replay can find the thread).
        // Concurrent loads should still wait for the in-flight task so that
        // ref-counting happens in one place and the caller sees a fully loaded
        // session.
        if let Some(pending) = self.pending_sessions.borrow_mut().get_mut(&session_id) {
            pending.ref_count += 1;
            let task = pending.task.clone();
            return cx
                .foreground_executor()
                .spawn(async move { task.await.map_err(|err| anyhow!(err)) });
        }

        if let Some(session) = self.sessions.borrow_mut().get_mut(&session_id) {
            session.ref_count += 1;
            if let Some(thread) = session.thread.upgrade() {
                return Task::ready(Ok(thread));
            }
        }

        let directories = match self.session_directories_from_work_dirs(&work_dirs) {
            Ok(directories) => directories,
            Err(error) => return Task::ready(Err(error)),
        };

        let shared_task = cx
            .spawn({
                let session_id = session_id.clone();
                let this = self.clone();
                async move |cx| {
                    let action_log = cx.new(|_| ActionLog::new(project.clone()));
                    let thread: Entity<AcpThread> = cx.new(|cx| {
                        AcpThread::new(
                            None,
                            title,
                            Some(work_dirs),
                            this.clone(),
                            project,
                            action_log,
                            session_id.clone(),
                            watch::Receiver::constant(
                                this.agent_capabilities.prompt_capabilities.clone(),
                            ),
                            cx,
                        )
                    });

                    // Register the session before awaiting the RPC so that any
                    // `session/update` notifications that arrive during the call
                    // (e.g. history replay during `session/load`) can find the thread.
                    // Modes/config are filled in once the response arrives.
                    this.sessions.borrow_mut().insert(
                        session_id.clone(),
                        AcpSession {
                            thread: thread.downgrade(),
                            suppress_abort_err: false,
                            session_modes: None,
                            config_options: None,
                            ref_count: 1,
                        },
                    );

                    let response =
                        match rpc_call(this.connection.clone(), session_id.clone(), directories)
                            .await
                        {
                            Ok(response) => response,
                            Err(err) => {
                                this.sessions.borrow_mut().remove(&session_id);
                                this.pending_sessions.borrow_mut().remove(&session_id);
                                return Err(Arc::new(err));
                            }
                        };

                    let (modes, config_options) =
                        config_state(response.modes, response.config_options);

                    if let Some(config_opts) = config_options.as_ref() {
                        this.apply_default_config_options(&session_id, config_opts, cx);
                    }

                    let ref_count = this
                        .pending_sessions
                        .borrow_mut()
                        .remove(&session_id)
                        .map_or(1, |pending| pending.ref_count);

                    // If `close_session` ran to completion while the load RPC was in
                    // flight, it will have removed both the pending entry and the
                    // sessions entry (and dispatched the ACP close RPC). In that case
                    // the thread has no live session to attach to, so fail the load
                    // instead of handing back an orphaned thread.
                    {
                        let mut sessions = this.sessions.borrow_mut();
                        let Some(session) = sessions.get_mut(&session_id) else {
                            return Err(Arc::new(anyhow!(
                                "session was closed before load completed"
                            )));
                        };
                        session.session_modes = modes;
                        session.config_options = config_options.map(ConfigOptions::new);
                        session.ref_count = ref_count;
                    }

                    Ok(thread)
                }
            })
            .shared();

        self.pending_sessions.borrow_mut().insert(
            session_id,
            PendingAcpSession {
                task: shared_task.clone(),
                ref_count: 1,
            },
        );

        cx.foreground_executor()
            .spawn(async move { shared_task.await.map_err(|err| anyhow!(err)) })
    }

    fn apply_default_config_options(
        &self,
        session_id: &acp::SessionId,
        config_options: &Rc<RefCell<Vec<acp::SessionConfigOption>>>,
        cx: &mut AsyncApp,
    ) {
        let id = self.id.clone();
        let defaults_to_apply: Vec<_> = {
            let config_opts_ref = config_options.borrow();
            config_opts_ref
                .iter()
                .filter_map(|config_option| {
                    let default_value = self.defaults.config_option(config_option.id.0.as_ref())?;

                    let value_to_apply = match &config_option.kind {
                        acp::SessionConfigKind::Select(select) => {
                            let value_id = default_value.as_value_id()?;
                            match &select.options {
                                acp::SessionConfigSelectOptions::Ungrouped(options) => options
                                    .iter()
                                    .any(|opt| &*opt.value.0 == value_id)
                                    .then(|| {
                                        acp::SessionConfigOptionValue::value_id(
                                            value_id.to_string(),
                                        )
                                    }),
                                acp::SessionConfigSelectOptions::Grouped(groups) => groups
                                    .iter()
                                    .any(|group| {
                                        group.options.iter().any(|opt| &*opt.value.0 == value_id)
                                    })
                                    .then(|| {
                                        acp::SessionConfigOptionValue::value_id(
                                            value_id.to_string(),
                                        )
                                    }),
                                _ => None,
                            }
                        }
                        acp::SessionConfigKind::Boolean(_) => default_value
                            .as_bool()
                            .map(acp::SessionConfigOptionValue::boolean),
                        _ => None,
                    };

                    if let Some(value_to_apply) = value_to_apply {
                        let initial_value = match &config_option.kind {
                            acp::SessionConfigKind::Select(select) => {
                                acp::SessionConfigOptionValue::value_id(
                                    select.current_value.clone(),
                                )
                            }
                            acp::SessionConfigKind::Boolean(boolean) => {
                                acp::SessionConfigOptionValue::boolean(boolean.current_value)
                            }
                            _ => return None,
                        };

                        Some((config_option.id.clone(), value_to_apply, initial_value))
                    } else {
                        log::warn!(
                            "`{}` is not a valid value for config option `{}` in {}",
                            default_value,
                            config_option.id.0,
                            id
                        );
                        None
                    }
                })
                .collect()
        };

        for (config_id, default_value, initial_value) in defaults_to_apply {
            cx.spawn({
                let default_value_for_request = default_value.clone();
                let session_id = session_id.clone();
                let config_id_clone = config_id.clone();
                let config_opts = config_options.clone();
                let conn = self.connection.clone();
                async move |_| {
                    let result = conn
                        .send_request(acp::SetSessionConfigOptionRequest::new(
                            session_id,
                            config_id_clone.clone(),
                            default_value_for_request,
                        ))
                        .block_task()
                        .await
                        .log_err();

                    if result.is_none() {
                        let mut opts = config_opts.borrow_mut();
                        if let Some(opt) = opts.iter_mut().find(|o| o.id == config_id_clone) {
                            match (&mut opt.kind, &initial_value) {
                                (
                                    acp::SessionConfigKind::Select(select),
                                    acp::SessionConfigOptionValue::ValueId { value },
                                ) => {
                                    select.current_value = value.clone();
                                }
                                (
                                    acp::SessionConfigKind::Boolean(boolean),
                                    acp::SessionConfigOptionValue::Boolean { value },
                                ) => {
                                    boolean.current_value = *value;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            })
            .detach();

            let mut opts = config_options.borrow_mut();
            if let Some(opt) = opts.iter_mut().find(|o| o.id == config_id) {
                match (&mut opt.kind, &default_value) {
                    (
                        acp::SessionConfigKind::Select(select),
                        acp::SessionConfigOptionValue::ValueId { value },
                    ) => {
                        select.current_value = value.clone();
                    }
                    (
                        acp::SessionConfigKind::Boolean(boolean),
                        acp::SessionConfigOptionValue::Boolean { value },
                    ) => {
                        boolean.current_value = *value;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SessionDirectories {
    cwd: PathBuf,
    additional_directories: Vec<PathBuf>,
}

impl SessionDirectories {
    fn into_new_session_request(self, mcp_servers: Vec<acp::McpServer>) -> acp::NewSessionRequest {
        acp::NewSessionRequest::new(self.cwd)
            .additional_directories(self.additional_directories)
            .mcp_servers(mcp_servers)
    }

    fn into_load_session_request(
        self,
        session_id: acp::SessionId,
        mcp_servers: Vec<acp::McpServer>,
    ) -> acp::LoadSessionRequest {
        acp::LoadSessionRequest::new(session_id, self.cwd)
            .additional_directories(self.additional_directories)
            .mcp_servers(mcp_servers)
    }

    fn into_resume_session_request(
        self,
        session_id: acp::SessionId,
        mcp_servers: Vec<acp::McpServer>,
    ) -> acp::ResumeSessionRequest {
        acp::ResumeSessionRequest::new(session_id, self.cwd)
            .additional_directories(self.additional_directories)
            .mcp_servers(mcp_servers)
    }
}

fn session_directories_from_work_dirs(
    work_dirs: &PathList,
    supports_additional_directories: bool,
) -> Result<SessionDirectories> {
    let mut ordered_paths = work_dirs.ordered_paths();
    let cwd = ordered_paths
        .next()
        .cloned()
        .ok_or_else(|| anyhow!("Working directory cannot be empty"))?;
    let additional_directories = if supports_additional_directories {
        ordered_paths.cloned().collect()
    } else {
        Vec::new()
    };

    Ok(SessionDirectories {
        cwd,
        additional_directories,
    })
}

fn work_dirs_from_session_info(cwd: PathBuf, additional_directories: Vec<PathBuf>) -> PathList {
    let mut seen_paths = HashSet::default();
    let mut paths = Vec::with_capacity(1 + additional_directories.len());

    seen_paths.insert(cwd.clone());
    paths.push(cwd);

    for path in additional_directories {
        if seen_paths.insert(path.clone()) {
            paths.push(path);
        }
    }

    PathList::new(&paths)
}

fn emit_load_error_to_all_sessions(
    sessions: &Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    error: LoadError,
    cx: &mut AsyncApp,
) {
    let threads: Vec<_> = sessions
        .borrow()
        .values()
        .map(|session| session.thread.clone())
        .collect();

    for thread in threads {
        thread
            .update(cx, |thread, cx| thread.emit_load_error(error.clone(), cx))
            .ok();
    }
}

impl Drop for AcpConnection {
    fn drop(&mut self) {
        if let Some(ref mut child) = self.child {
            child.kill().log_err();
        }
    }
}

fn terminal_auth_task_id(agent_id: &AgentId, method_id: &acp::AuthMethodId) -> String {
    format!("external-agent-{}-{}-login", agent_id.0, method_id.0)
}

fn terminal_auth_task(
    command: &AgentServerCommand,
    agent_id: &AgentId,
    method: &acp::AuthMethodTerminal,
) -> SpawnInTerminal {
    acp_thread::build_terminal_auth_task(
        terminal_auth_task_id(agent_id, &method.id),
        method.name.clone(),
        command.path.to_string_lossy().into_owned(),
        command.args.clone(),
        command.env.clone().unwrap_or_default(),
    )
}

/// Used to support the _meta method prior to stabilization
fn meta_terminal_auth_task(
    agent_id: &AgentId,
    method_id: &acp::AuthMethodId,
    method: &acp::AuthMethod,
) -> Option<SpawnInTerminal> {
    #[derive(Deserialize)]
    struct MetaTerminalAuth {
        label: String,
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    }

    let meta = match method {
        acp::AuthMethod::EnvVar(env_var) => env_var.meta.as_ref(),
        acp::AuthMethod::Terminal(terminal) => terminal.meta.as_ref(),
        acp::AuthMethod::Agent(agent) => agent.meta.as_ref(),
        _ => None,
    }?;
    let terminal_auth =
        serde_json::from_value::<MetaTerminalAuth>(meta.get("terminal-auth")?.clone()).ok()?;

    Some(acp_thread::build_terminal_auth_task(
        terminal_auth_task_id(agent_id, method_id),
        terminal_auth.label.clone(),
        terminal_auth.command,
        terminal_auth.args,
        terminal_auth.env,
    ))
}

impl AgentConnection for AcpConnection {
    fn agent_id(&self) -> AgentId {
        self.id.clone()
    }

    fn telemetry_id(&self) -> SharedString {
        self.telemetry_id.clone()
    }

    fn agent_version(&self) -> Option<SharedString> {
        self.agent_version.clone()
    }

    fn new_session(
        self: Rc<Self>,
        project: Entity<Project>,
        work_dirs: PathList,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let directories = match self.session_directories_from_work_dirs(&work_dirs) {
            Ok(directories) => directories,
            Err(error) => return Task::ready(Err(error)),
        };
        let name = self.id.0.clone();
        let mcp_servers = mcp_servers_for_project(&project, cx);

        cx.spawn(async move |cx| {
            let response = self
                .connection
                .send_request(directories.into_new_session_request(mcp_servers))
                .block_task()
            .await
            .map_err(map_acp_error)?;

            let (modes, config_options) = config_state(response.modes, response.config_options);

            let default_mode = self.defaults.mode();
            if let Some(default_mode) = default_mode {
                if let Some(modes) = modes.as_ref() {
                    let mut modes_ref = modes.borrow_mut();
                    let has_mode = modes_ref
                        .available_modes
                        .iter()
                        .any(|mode| mode.id == default_mode);

                    if has_mode {
                        let initial_mode_id = modes_ref.current_mode_id.clone();

                        cx.spawn({
                            let default_mode = default_mode.clone();
                            let session_id = response.session_id.clone();
                            let modes = modes.clone();
                            let conn = self.connection.clone();
                            async move |_| {
                                let result = conn
                                    .send_request(acp::SetSessionModeRequest::new(
                                        session_id,
                                        default_mode,
                                    ))
                                    .block_task()
                                .await
                                .log_err();

                                if result.is_none() {
                                    modes.borrow_mut().current_mode_id = initial_mode_id;
                                }
                            }
                        })
                        .detach();

                        modes_ref.current_mode_id = default_mode;
                    } else {
                        let available_modes = modes_ref
                            .available_modes
                            .iter()
                            .map(|mode| format!("- `{}`: {}", mode.id, mode.name))
                            .collect::<Vec<_>>()
                            .join("\n");

                        log::warn!(
                            "`{default_mode}` is not valid {name} mode. Available options:\n{available_modes}",
                        );
                    }
                }
            }

            if let Some(config_opts) = config_options.as_ref() {
                self.apply_default_config_options(&response.session_id, config_opts, cx);
            }

            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread: Entity<AcpThread> = cx.new(|cx| {
                AcpThread::new(
                    None,
                    None,
                    Some(work_dirs),
                    self.clone(),
                    project,
                    action_log,
                    response.session_id.clone(),
                    // ACP doesn't currently support per-session prompt capabilities or changing capabilities dynamically.
                    watch::Receiver::constant(
                        self.agent_capabilities.prompt_capabilities.clone(),
                    ),
                    cx,
                )
            });

            self.sessions.borrow_mut().insert(
                response.session_id,
                AcpSession {
                    thread: thread.downgrade(),
                    suppress_abort_err: false,
                    session_modes: modes,
                    config_options: config_options.map(ConfigOptions::new),
                    ref_count: 1,
                },
            );

            Ok(thread)
        })
    }

    fn supports_load_session(&self) -> bool {
        self.agent_capabilities.load_session
    }

    fn supports_resume_session(&self) -> bool {
        self.agent_capabilities
            .session_capabilities
            .resume
            .is_some()
    }

    fn supports_session_additional_directories(&self) -> bool {
        self.agent_capabilities
            .session_capabilities
            .additional_directories
            .is_some()
    }

    fn load_session(
        self: Rc<Self>,
        session_id: acp::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        if !self.agent_capabilities.load_session {
            return Task::ready(Err(anyhow!(LoadError::Other(
                "Loading sessions is not supported by this agent.".into()
            ))));
        }

        let mcp_servers = mcp_servers_for_project(&project, cx);
        self.open_or_create_session(
            session_id,
            project,
            work_dirs,
            title,
            move |connection, session_id, directories| {
                Box::pin(async move {
                    let response = connection
                        .send_request(
                            directories.into_load_session_request(session_id.clone(), mcp_servers),
                        )
                        .block_task()
                        .await
                        .map_err(map_acp_error)?;
                    Ok(SessionConfigResponse {
                        modes: response.modes,
                        config_options: response.config_options,
                    })
                })
            },
            cx,
        )
    }

    fn resume_session(
        self: Rc<Self>,
        session_id: acp::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        if self
            .agent_capabilities
            .session_capabilities
            .resume
            .is_none()
        {
            return Task::ready(Err(anyhow!(LoadError::Other(
                "Resuming sessions is not supported by this agent.".into()
            ))));
        }

        let mcp_servers = mcp_servers_for_project(&project, cx);
        self.open_or_create_session(
            session_id,
            project,
            work_dirs,
            title,
            move |connection, session_id, directories| {
                Box::pin(async move {
                    let response = connection
                        .send_request(
                            directories
                                .into_resume_session_request(session_id.clone(), mcp_servers),
                        )
                        .block_task()
                        .await
                        .map_err(map_acp_error)?;
                    Ok(SessionConfigResponse {
                        modes: response.modes,
                        config_options: response.config_options,
                    })
                })
            },
            cx,
        )
    }

    fn supports_close_session(&self) -> bool {
        self.agent_capabilities.session_capabilities.close.is_some()
    }

    fn close_session(
        self: Rc<Self>,
        session_id: &acp::SessionId,
        cx: &mut App,
    ) -> Task<Result<()>> {
        if !self.supports_close_session() {
            return Task::ready(Err(anyhow!(LoadError::Other(
                "Closing sessions is not supported by this agent.".into()
            ))));
        }

        // If a load is still in flight, decrement its ref count. The pending
        // entry is the source of truth for how many handles exist during a
        // load, so we must tick it down here as well as the `sessions` entry
        // that was pre-registered to receive history-replay notifications.
        // Only once the pending ref count hits zero do we actually close the
        // session; the load task will observe the missing sessions entry and
        // fail with "session was closed before load completed".
        let pending_ref_count = {
            let mut pending_sessions = self.pending_sessions.borrow_mut();
            pending_sessions.get_mut(session_id).map(|pending| {
                pending.ref_count = pending.ref_count.saturating_sub(1);
                pending.ref_count
            })
        };
        match pending_ref_count {
            Some(0) => {
                self.pending_sessions.borrow_mut().remove(session_id);
                self.sessions.borrow_mut().remove(session_id);

                let conn = self.connection.clone();
                let session_id = session_id.clone();
                return cx.foreground_executor().spawn(async move {
                    conn.send_request(acp::CloseSessionRequest::new(session_id))
                        .block_task()
                        .await?;
                    Ok(())
                });
            }
            Some(_) => return Task::ready(Ok(())),
            None => {}
        }

        let mut sessions = self.sessions.borrow_mut();
        let Some(session) = sessions.get_mut(session_id) else {
            return Task::ready(Ok(()));
        };

        session.ref_count = session.ref_count.saturating_sub(1);
        if session.ref_count > 0 {
            return Task::ready(Ok(()));
        }

        sessions.remove(session_id);
        drop(sessions);

        let conn = self.connection.clone();
        let session_id = session_id.clone();
        cx.foreground_executor().spawn(async move {
            conn.send_request(acp::CloseSessionRequest::new(session_id.clone()))
                .block_task()
                .await?;
            Ok(())
        })
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &self.auth_methods
    }

    fn terminal_auth_task(
        &self,
        method_id: &acp::AuthMethodId,
        cx: &App,
    ) -> Option<Task<Result<SpawnInTerminal>>> {
        let method = self
            .auth_methods
            .iter()
            .find(|method| method.id() == method_id)?;

        match method {
            acp::AuthMethod::Terminal(terminal) if cx.has_flag::<AcpBetaFeatureFlag>() => {
                let agent_id = self.id.clone();
                let terminal = terminal.clone();
                let store = self.agent_server_store.clone();
                Some(cx.spawn(async move |cx| {
                    let command = store
                        .update(cx, |store, cx| {
                            let agent = store
                                .get_external_agent(&agent_id)
                                .context("Agent server not found")?;
                            anyhow::Ok(agent.get_command(
                                terminal.args.clone(),
                                HashMap::from_iter(terminal.env.clone()),
                                &mut cx.to_async(),
                            ))
                        })?
                        .context("Failed to get agent command")?
                        .await?;
                    Ok(terminal_auth_task(&command, &agent_id, &terminal))
                }))
            }
            _ => meta_terminal_auth_task(&self.id, method_id, method)
                .map(|task| Task::ready(Ok(task))),
        }
    }

    fn authenticate(&self, method_id: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            conn.send_request(acp::AuthenticateRequest::new(method_id))
                .block_task()
                .await?;
            Ok(())
        })
    }

    fn supports_logout(&self) -> bool {
        self.agent_capabilities.auth.logout.is_some()
    }

    fn logout(&self, cx: &mut App) -> Task<Result<()>> {
        if !self.supports_logout() {
            return Task::ready(Err(anyhow!("Logout is not supported by this agent.")));
        }

        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            conn.send_request(acp::LogoutRequest::new())
                .block_task()
                .await?;
            Ok(())
        })
    }

    fn prompt(
        &self,
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        let conn = self.connection.clone();
        let sessions = self.sessions.clone();
        let session_id = params.session_id.clone();
        cx.foreground_executor().spawn(async move {
            let result = conn.send_request(params).block_task().await;

            let mut suppress_abort_err = false;

            if let Some(session) = sessions.borrow_mut().get_mut(&session_id) {
                suppress_abort_err = session.suppress_abort_err;
                session.suppress_abort_err = false;
            }

            match result {
                Ok(response) => Ok(response),
                Err(err) => {
                    if err.code == acp::ErrorCode::AuthRequired {
                        return Err(anyhow!(acp::Error::auth_required()));
                    }

                    if err.code != ErrorCode::InternalError {
                        anyhow::bail!(err)
                    }

                    let Some(data) = &err.data else {
                        anyhow::bail!(err)
                    };

                    // Temporary workaround until the following PR is generally available:
                    // https://github.com/google-gemini/gemini-cli/pull/6656

                    #[derive(Deserialize)]
                    #[serde(deny_unknown_fields)]
                    struct ErrorDetails {
                        details: Box<str>,
                    }

                    match serde_json::from_value(data.clone()) {
                        Ok(ErrorDetails { details }) => {
                            if suppress_abort_err
                                && (details.contains("This operation was aborted")
                                    || details.contains("The user aborted a request"))
                            {
                                Ok(acp::PromptResponse::new(acp::StopReason::Cancelled))
                            } else {
                                Err(anyhow!(details))
                            }
                        }
                        Err(_) => Err(anyhow!(err)),
                    }
                }
            }
        })
    }

    fn cancel(&self, session_id: &acp::SessionId, _cx: &mut App) {
        if let Some(session) = self.sessions.borrow_mut().get_mut(session_id) {
            session.suppress_abort_err = true;
        }
        let params = acp::CancelNotification::new(session_id.clone());
        self.connection.send_notification(params).log_err();
    }

    fn request_elicitations(&self) -> Option<Entity<ElicitationStore>> {
        Some(self.request_elicitations.clone())
    }

    fn session_modes(
        &self,
        session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionModes>> {
        let sessions = self.sessions.clone();
        let sessions_ref = sessions.borrow();
        let Some(session) = sessions_ref.get(session_id) else {
            return None;
        };

        if let Some(modes) = session.session_modes.as_ref() {
            Some(Rc::new(AcpSessionModes {
                connection: self.connection.clone(),
                session_id: session_id.clone(),
                state: modes.clone(),
            }) as _)
        } else {
            None
        }
    }

    fn session_config_options(
        &self,
        session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionConfigOptions>> {
        let sessions = self.sessions.borrow();
        let session = sessions.get(session_id)?;

        let config_opts = session.config_options.as_ref()?;

        Some(Rc::new(AcpSessionConfigOptions {
            session_id: session_id.clone(),
            connection: self.connection.clone(),
            state: config_opts.config_options.clone(),
            watch_tx: config_opts.tx.clone(),
            watch_rx: config_opts.rx.clone(),
        }) as _)
    }

    fn session_list(&self, _cx: &mut App) -> Option<Rc<dyn AgentSessionList>> {
        self.session_list.clone().map(|s| s as _)
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

fn map_acp_error(err: acp::Error) -> anyhow::Error {
    if err.code == acp::ErrorCode::AuthRequired {
        let mut error = AuthRequired::new();

        if err.message != acp::ErrorCode::AuthRequired.to_string() {
            error = error.with_description(err.message);
        }

        anyhow!(error)
    } else {
        anyhow!(err)
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use acp_thread::{
        AgentSessionClientUserMessageIds, AgentSessionConfigOptions, AgentSessionModes,
        AgentSessionRetry, AgentSessionSetTitle, AgentSessionTruncate, AgentTelemetry,
    };

    use super::*;

    #[derive(Clone, Default)]
    pub struct FakeAcpAgentServer {
        load_session_count: Arc<AtomicUsize>,
        close_session_count: Arc<AtomicUsize>,
        fail_next_prompt: Arc<AtomicBool>,
        auth_elicitation_request: Arc<Mutex<Option<acp::CreateElicitationRequest>>>,
        auth_elicitation_response:
            Arc<Mutex<Option<async_channel::Sender<acp::CreateElicitationResponse>>>>,
        auth_elicitation_completion: Arc<Mutex<Option<acp::CompleteElicitationNotification>>>,
        exit_status_sender:
            Arc<std::sync::Mutex<Option<async_channel::Sender<std::process::ExitStatus>>>>,
    }

    impl FakeAcpAgentServer {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn load_session_count(&self) -> Arc<AtomicUsize> {
            self.load_session_count.clone()
        }

        pub fn close_session_count(&self) -> Arc<AtomicUsize> {
            self.close_session_count.clone()
        }

        pub fn simulate_server_exit(&self) {
            let sender = self
                .exit_status_sender
                .lock()
                .expect("exit status sender lock should not be poisoned")
                .clone()
                .expect("fake ACP server must be connected before simulating exit");
            sender
                .try_send(std::process::ExitStatus::default())
                .expect("fake ACP server exit receiver should still be alive");
        }

        pub fn fail_next_prompt(&self) {
            self.fail_next_prompt.store(true, Ordering::SeqCst);
        }

        pub fn request_elicitation_during_auth(
            &self,
            request: acp::CreateElicitationRequest,
        ) -> async_channel::Receiver<acp::CreateElicitationResponse> {
            let (response_tx, response_rx) = async_channel::bounded(1);
            *self
                .auth_elicitation_request
                .lock()
                .expect("auth elicitation request lock should not be poisoned") = Some(request);
            *self
                .auth_elicitation_response
                .lock()
                .expect("auth elicitation response lock should not be poisoned") =
                Some(response_tx);
            response_rx
        }
    }

    impl crate::AgentServer for FakeAcpAgentServer {
        fn logo(&self) -> ui::IconName {
            ui::IconName::ZedAgent
        }

        fn agent_id(&self) -> AgentId {
            AgentId::new("Test")
        }

        fn connect(
            &self,
            _delegate: crate::AgentServerDelegate,
            project: Entity<Project>,
            cx: &mut App,
        ) -> Task<anyhow::Result<Rc<dyn AgentConnection>>> {
            let load_session_count = self.load_session_count.clone();
            let close_session_count = self.close_session_count.clone();
            let fail_next_prompt = self.fail_next_prompt.clone();
            let auth_elicitation_request = self.auth_elicitation_request.clone();
            let auth_elicitation_response = self.auth_elicitation_response.clone();
            let auth_elicitation_completion = self.auth_elicitation_completion.clone();
            let exit_status_sender = self.exit_status_sender.clone();
            cx.spawn(async move |cx| {
                let harness = build_fake_acp_connection(
                    project,
                    load_session_count,
                    close_session_count,
                    fail_next_prompt,
                    auth_elicitation_request,
                    auth_elicitation_response,
                    auth_elicitation_completion,
                    cx,
                )
                .await?;
                let (exit_tx, exit_rx) = async_channel::bounded(1);
                *exit_status_sender
                    .lock()
                    .expect("exit status sender lock should not be poisoned") = Some(exit_tx);
                let connection = harness.connection.clone();
                let simulate_exit_task = cx.spawn(async move |cx| {
                    while let Ok(status) = exit_rx.recv().await {
                        emit_load_error_to_all_sessions(
                            &connection.sessions,
                            LoadError::Exited {
                                status,
                                stderr: None,
                            },
                            cx,
                        );
                    }
                    Ok(())
                });
                Ok(Rc::new(FakeAcpAgentConnection {
                    inner: harness.connection,
                    _keep_agent_alive: harness.keep_agent_alive,
                    _simulate_exit_task: simulate_exit_task,
                }) as Rc<dyn AgentConnection>)
            })
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    pub struct FakeAcpConnectionHarness {
        pub connection: Rc<AcpConnection>,
        pub load_session_count: Arc<AtomicUsize>,
        pub close_session_count: Arc<AtomicUsize>,
        pub logout_count: Arc<AtomicUsize>,
        pub keep_agent_alive: Task<anyhow::Result<()>>,
    }

    struct FakeAcpAgentConnection {
        inner: Rc<AcpConnection>,
        _keep_agent_alive: Task<anyhow::Result<()>>,
        _simulate_exit_task: Task<anyhow::Result<()>>,
    }

    impl AgentConnection for FakeAcpAgentConnection {
        fn agent_id(&self) -> AgentId {
            self.inner.agent_id()
        }

        fn telemetry_id(&self) -> SharedString {
            self.inner.telemetry_id()
        }

        fn agent_version(&self) -> Option<SharedString> {
            self.inner.agent_version()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            self.inner.clone().new_session(project, work_dirs, cx)
        }

        fn supports_load_session(&self) -> bool {
            self.inner.supports_load_session()
        }

        fn load_session(
            self: Rc<Self>,
            session_id: acp::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            self.inner
                .clone()
                .load_session(session_id, project, work_dirs, title, cx)
        }

        fn supports_close_session(&self) -> bool {
            self.inner.supports_close_session()
        }

        fn close_session(
            self: Rc<Self>,
            session_id: &acp::SessionId,
            cx: &mut App,
        ) -> Task<Result<()>> {
            self.inner.clone().close_session(session_id, cx)
        }

        fn supports_resume_session(&self) -> bool {
            self.inner.supports_resume_session()
        }

        fn supports_session_additional_directories(&self) -> bool {
            self.inner.supports_session_additional_directories()
        }

        fn resume_session(
            self: Rc<Self>,
            session_id: acp::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            self.inner
                .clone()
                .resume_session(session_id, project, work_dirs, title, cx)
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            self.inner.auth_methods()
        }

        fn terminal_auth_task(
            &self,
            method: &acp::AuthMethodId,
            cx: &App,
        ) -> Option<Task<Result<SpawnInTerminal>>> {
            self.inner.terminal_auth_task(method, cx)
        }

        fn authenticate(&self, method: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
            self.inner.authenticate(method, cx)
        }

        fn supports_logout(&self) -> bool {
            self.inner.supports_logout()
        }

        fn logout(&self, cx: &mut App) -> Task<Result<()>> {
            self.inner.logout(cx)
        }

        fn client_user_message_ids(
            &self,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionClientUserMessageIds>> {
            self.inner.client_user_message_ids(cx)
        }

        fn prompt(
            &self,
            params: acp::PromptRequest,
            cx: &mut App,
        ) -> Task<Result<acp::PromptResponse>> {
            self.inner.prompt(params, cx)
        }

        fn retry(
            &self,
            session_id: &acp::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionRetry>> {
            self.inner.retry(session_id, cx)
        }

        fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
            self.inner.cancel(session_id, cx)
        }

        fn request_elicitations(&self) -> Option<Entity<ElicitationStore>> {
            self.inner.request_elicitations()
        }

        fn truncate(
            &self,
            session_id: &acp::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionTruncate>> {
            self.inner.truncate(session_id, cx)
        }

        fn set_title(
            &self,
            session_id: &acp::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionSetTitle>> {
            self.inner.set_title(session_id, cx)
        }

        fn telemetry(&self) -> Option<Rc<dyn AgentTelemetry>> {
            self.inner.telemetry()
        }

        fn session_modes(
            &self,
            session_id: &acp::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionModes>> {
            self.inner.session_modes(session_id, cx)
        }

        fn session_config_options(
            &self,
            session_id: &acp::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionConfigOptions>> {
            self.inner.session_config_options(session_id, cx)
        }

        fn session_list(&self, cx: &mut App) -> Option<Rc<dyn AgentSessionList>> {
            self.inner.session_list(cx)
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    async fn build_fake_acp_connection(
        project: Entity<Project>,
        load_session_count: Arc<AtomicUsize>,
        close_session_count: Arc<AtomicUsize>,
        fail_next_prompt: Arc<AtomicBool>,
        auth_elicitation_request: Arc<Mutex<Option<acp::CreateElicitationRequest>>>,
        auth_elicitation_response: Arc<
            Mutex<Option<async_channel::Sender<acp::CreateElicitationResponse>>>,
        >,
        auth_elicitation_completion: Arc<Mutex<Option<acp::CompleteElicitationNotification>>>,
        cx: &mut AsyncApp,
    ) -> Result<FakeAcpConnectionHarness> {
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();

        let logout_count = Arc::new(AtomicUsize::new(0));
        let sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>> =
            Rc::new(RefCell::new(HashMap::default()));
        let client_session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>> =
            Rc::new(RefCell::new(None));

        let agent_future = Agent
            .builder()
            .name("fake-agent")
            .on_receive_request(
                async move |req: acp::InitializeRequest, responder, _cx| {
                    responder.respond(
                        acp::InitializeResponse::new(req.protocol_version).agent_capabilities(
                            acp::AgentCapabilities::default()
                                .load_session(true)
                                .session_capabilities(
                                    acp::SessionCapabilities::default()
                                        .close(acp::SessionCloseCapabilities::new()),
                                ),
                        ),
                    )
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let auth_elicitation_request = auth_elicitation_request.clone();
                    let auth_elicitation_response = auth_elicitation_response.clone();
                    let auth_elicitation_completion = auth_elicitation_completion.clone();
                    async move |_req: acp::AuthenticateRequest, responder, cx| {
                        let request = auth_elicitation_request
                            .lock()
                            .expect("auth elicitation request lock should not be poisoned")
                            .take();
                        let response_tx = auth_elicitation_response
                            .lock()
                            .expect("auth elicitation response lock should not be poisoned")
                            .take();
                        let completion = auth_elicitation_completion
                            .lock()
                            .expect("auth elicitation completion lock should not be poisoned")
                            .take();

                        if let Some(request) = request {
                            cx.send_request(request)
                                .on_receiving_result(async move |result| {
                                    if let (Ok(response), Some(response_tx)) = (result, response_tx)
                                    {
                                        response_tx.send(response).await.ok();
                                    }
                                    responder.respond(Default::default())
                                })?;
                            if let Some(completion) = completion {
                                cx.send_notification(completion)?;
                            }
                            Ok(())
                        } else {
                            responder.respond(Default::default())
                        }
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::NewSessionRequest, responder, _cx| {
                    responder.respond(acp::NewSessionResponse::new(acp::SessionId::new("unused")))
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let fail_next_prompt = fail_next_prompt.clone();
                    async move |_req: acp::PromptRequest, responder, _cx| {
                        if fail_next_prompt.swap(false, Ordering::SeqCst) {
                            responder.respond_with_error(acp::ErrorCode::InternalError.into())
                        } else {
                            responder.respond(acp::PromptResponse::new(acp::StopReason::EndTurn))
                        }
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let load_session_count = load_session_count.clone();
                    async move |_req: acp::LoadSessionRequest, responder, _cx| {
                        load_session_count.fetch_add(1, Ordering::SeqCst);
                        responder.respond(acp::LoadSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let close_session_count = close_session_count.clone();
                    async move |_req: acp::CloseSessionRequest, responder, _cx| {
                        close_session_count.fetch_add(1, Ordering::SeqCst);
                        responder.respond(acp::CloseSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let logout_count = logout_count.clone();
                    async move |_req: acp::LogoutRequest, responder, _cx| {
                        logout_count.fetch_add(1, Ordering::SeqCst);
                        responder.respond(acp::LogoutResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_notification(
                async move |_notif: acp::CancelNotification, _cx| Ok(()),
                agent_client_protocol::on_receive_notification!(),
            )
            .connect_to(agent_transport);

        let agent_io_task = cx.background_spawn(agent_future);

        // Wire the production handler set into the fake client so inbound
        // requests/notifications from the fake agent are dispatched the
        // same way the real `stdio` path does.
        let (dispatch_tx, dispatch_rx) = mpsc::unbounded::<ForegroundWork>();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let client_future = connect_client_future(
            "zed-test",
            client_transport,
            dispatch_tx.clone(),
            connection_tx,
        );
        let client_io_task = cx.background_spawn(async move {
            client_future.await.ok();
        });

        let client_conn: ConnectionTo<Agent> = connection_rx
            .await
            .context("failed to receive fake ACP connection handle")?;

        let response = client_conn
            .send_request(acp::InitializeRequest::new(ProtocolVersion::V1))
            .block_task()
            .await?;

        let agent_capabilities = response.agent_capabilities;

        let request_elicitations = cx.new(|_| ElicitationStore::default());
        let dispatch_context = ClientContext {
            sessions: sessions.clone(),
            session_list: client_session_list.clone(),
            request_elicitations: request_elicitations.clone(),
        };
        let dispatch_task = cx.spawn({
            let mut dispatch_rx = dispatch_rx;
            async move |cx| {
                while let Some(work) = dispatch_rx.next().await {
                    work.run(cx, &dispatch_context);
                }
            }
        });

        let agent_server_store =
            project.read_with(cx, |project, _| project.agent_server_store().downgrade());

        let connection = cx.update(|cx| {
            AcpConnection::new_for_test(
                client_conn,
                sessions,
                agent_capabilities,
                request_elicitations,
                agent_server_store,
                client_io_task,
                dispatch_task,
                cx,
            )
        });

        let keep_agent_alive = cx.background_spawn(async move {
            agent_io_task.await.ok();
            anyhow::Ok(())
        });

        Ok(FakeAcpConnectionHarness {
            connection: Rc::new(connection),
            load_session_count,
            close_session_count,
            logout_count,
            keep_agent_alive,
        })
    }

    pub async fn connect_fake_acp_connection(
        project: Entity<Project>,
        cx: &mut gpui::TestAppContext,
    ) -> FakeAcpConnectionHarness {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        build_fake_acp_connection(
            project,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(None)),
            &mut cx.to_async(),
        )
        .await
        .expect("failed to initialize ACP connection")
    }

    #[cfg(test)]
    pub async fn connect_fake_acp_connection_with_auth_elicitation(
        project: Entity<Project>,
        request: acp::CreateElicitationRequest,
        cx: &mut gpui::TestAppContext,
    ) -> (
        FakeAcpConnectionHarness,
        async_channel::Receiver<acp::CreateElicitationResponse>,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let (response_tx, response_rx) = async_channel::bounded(1);
        let harness = build_fake_acp_connection(
            project,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(Some(request))),
            Arc::new(Mutex::new(Some(response_tx))),
            Arc::new(Mutex::new(None)),
            &mut cx.to_async(),
        )
        .await
        .expect("failed to initialize ACP connection");

        (harness, response_rx)
    }

    #[cfg(test)]
    pub async fn connect_fake_acp_connection_with_auth_elicitation_completion(
        project: Entity<Project>,
        request: acp::CreateElicitationRequest,
        completion: acp::CompleteElicitationNotification,
        cx: &mut gpui::TestAppContext,
    ) -> (
        FakeAcpConnectionHarness,
        async_channel::Receiver<acp::CreateElicitationResponse>,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let (response_tx, response_rx) = async_channel::bounded(1);
        let harness = build_fake_acp_connection(
            project,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(Some(request))),
            Arc::new(Mutex::new(Some(response_tx))),
            Arc::new(Mutex::new(Some(completion))),
            &mut cx.to_async(),
        )
        .await
        .expect("failed to initialize ACP connection");

        (harness, response_rx)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use feature_flags::FeatureFlag as _;
    use settings::Settings as _;

    fn init_feature_flags_test(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut settings_store = SettingsStore::test(cx);
            settings_store.register_setting::<feature_flags::FeatureFlagsSettings>();
            cx.set_global(settings_store);
            cx.update_flags(false, vec![]);
        });
    }

    #[gpui::test]
    async fn client_capabilities_include_elicitation_without_acp_beta(
        cx: &mut gpui::TestAppContext,
    ) {
        init_feature_flags_test(cx);
        let capabilities = client_capabilities_for_agent(&AgentId::new("codex-acp"));
        let elicitation = capabilities
            .elicitation
            .expect("elicitation should always be advertised");

        assert!(elicitation.form.is_some());
        assert!(elicitation.url.is_some());
    }

    #[gpui::test]
    async fn request_scoped_elicitation_during_auth_uses_connection_store(
        cx: &mut gpui::TestAppContext,
    ) {
        init_feature_flags_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec![AcpBetaFeatureFlag::NAME.to_string()]);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let request_id = acp::RequestId::Number(1);
        let (harness, response_rx) =
            test_support::connect_fake_acp_connection_with_auth_elicitation(
                project,
                acp::CreateElicitationRequest::new(
                    acp::ElicitationFormMode::new(
                        acp::ElicitationRequestScope::new(request_id.clone()),
                        acp::ElicitationSchema::new().string("name", true),
                    ),
                    "Provide a name",
                ),
                cx,
            )
            .await;
        let connection = harness.connection.clone();
        let auth_task =
            cx.update(|cx| connection.authenticate(acp::AuthMethodId::new("login"), cx));
        cx.run_until_parked();

        let store = connection
            .request_elicitations()
            .expect("ACP connections expose request-scoped elicitations");
        let elicitation_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one request-scoped elicitation, got {:?}",
                    store.elicitations()
                );
            };
            let acp::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, request_id);
            elicitation.id.clone()
        });
        assert!(
            connection.sessions.borrow().is_empty(),
            "auth-time request-scoped elicitations must not require a session"
        );

        let expected_content = std::collections::BTreeMap::from([(
            "name".to_string(),
            acp::ElicitationContentValue::from("Ada"),
        )]);
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &elicitation_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new().content(expected_content.clone()),
                )),
                cx,
            );
        });

        let response = response_rx
            .recv()
            .await
            .expect("fake auth flow should receive elicitation response");
        assert_eq!(
            response.action,
            acp::ElicitationAction::Accept(
                acp::ElicitationAcceptAction::new().content(expected_content)
            )
        );
        auth_task.await.expect("auth should complete");
    }

    #[gpui::test]
    async fn request_scoped_url_elicitation_completion_after_create_is_observed(
        cx: &mut gpui::TestAppContext,
    ) {
        init_feature_flags_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec![AcpBetaFeatureFlag::NAME.to_string()]);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let request_id = acp::RequestId::Number(1);
        let url_elicitation_id = acp::ElicitationId::new("auth-url");
        let (harness, response_rx) =
            test_support::connect_fake_acp_connection_with_auth_elicitation_completion(
                project,
                acp::CreateElicitationRequest::new(
                    acp::ElicitationUrlMode::new(
                        acp::ElicitationRequestScope::new(request_id.clone()),
                        url_elicitation_id.clone(),
                        "https://auth.example.com/device",
                    ),
                    "Authorize Zed in your browser",
                ),
                acp::CompleteElicitationNotification::new(url_elicitation_id),
                cx,
            )
            .await;
        let connection = harness.connection.clone();
        let auth_task =
            cx.update(|cx| connection.authenticate(acp::AuthMethodId::new("login"), cx));
        cx.run_until_parked();

        let response = response_rx
            .recv()
            .await
            .expect("fake auth flow should receive elicitation response");
        assert_eq!(
            response.action,
            acp::ElicitationAction::Accept(acp::ElicitationAcceptAction::new())
        );

        let store = connection
            .request_elicitations()
            .expect("ACP connections expose request-scoped elicitations");
        store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one request-scoped elicitation, got {:?}",
                    store.elicitations()
                );
            };
            let acp::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, request_id);
            assert!(matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Completed
            ));
        });

        auth_task.await.expect("auth should complete");
    }

    #[gpui::test]
    async fn request_scoped_elicitation_ignores_open_sessions(cx: &mut gpui::TestAppContext) {
        init_feature_flags_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec![AcpBetaFeatureFlag::NAME.to_string()]);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let request_id = acp::RequestId::Number(1);
        let (harness, response_rx) =
            test_support::connect_fake_acp_connection_with_auth_elicitation(
                project.clone(),
                acp::CreateElicitationRequest::new(
                    acp::ElicitationFormMode::new(
                        acp::ElicitationRequestScope::new(request_id.clone()),
                        acp::ElicitationSchema::new().string("name", true),
                    ),
                    "Provide a name",
                ),
                cx,
            )
            .await;
        let connection = harness.connection.clone();
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        let first_thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    acp::SessionId::new("session-1"),
                    project.clone(),
                    work_dirs.clone(),
                    None,
                    cx,
                )
            })
            .await
            .expect("first load_session should succeed");
        let second_thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    acp::SessionId::new("session-2"),
                    project,
                    work_dirs,
                    None,
                    cx,
                )
            })
            .await
            .expect("second load_session should succeed");
        cx.run_until_parked();
        assert_eq!(
            connection.sessions.borrow().len(),
            2,
            "test setup should have multiple open sessions"
        );

        let auth_task =
            cx.update(|cx| connection.authenticate(acp::AuthMethodId::new("login"), cx));
        cx.run_until_parked();

        let store = connection
            .request_elicitations()
            .expect("ACP connections expose request-scoped elicitations");
        let elicitation_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one request-scoped elicitation, got {:?}",
                    store.elicitations()
                );
            };
            let acp::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, request_id);
            elicitation.id.clone()
        });

        for thread in [first_thread, second_thread] {
            thread.read_with(cx, |thread, _| {
                assert!(
                    thread.entries().iter().all(|entry| !matches!(
                        entry,
                        acp_thread::AgentThreadEntry::Elicitation(_)
                    )),
                    "request-scoped elicitation should not be inserted into a session thread"
                );
            });
        }

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &elicitation_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
        });

        let response = response_rx
            .recv()
            .await
            .expect("fake auth flow should receive elicitation response");
        assert_eq!(response.action, acp::ElicitationAction::Decline);
        auth_task.await.expect("auth should complete");
    }

    #[test]
    fn cursor_client_capabilities_include_parameterized_model_picker_meta() {
        let capabilities = client_capabilities_for_agent(&AgentId::new(CURSOR_ID));
        let meta = capabilities
            .meta
            .expect("expected client capabilities meta");

        assert_eq!(
            meta.get(PARAMETERIZED_MODEL_PICKER_META_KEY),
            Some(&serde_json::json!(true))
        );
        assert_eq!(meta.get("terminal_output"), Some(&serde_json::json!(true)));
        assert_eq!(meta.get("terminal-auth"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn non_cursor_client_capabilities_do_not_include_parameterized_model_picker_meta() {
        let capabilities = client_capabilities_for_agent(&AgentId::new("codex-acp"));
        let meta = capabilities
            .meta
            .expect("expected client capabilities meta");

        assert!(!meta.contains_key(PARAMETERIZED_MODEL_PICKER_META_KEY));
    }

    #[test]
    fn client_capabilities_include_boolean_config_options() {
        let capabilities = client_capabilities_for_agent(&AgentId::new("codex-acp"));

        assert!(
            capabilities
                .session
                .and_then(|session| session.config_options)
                .and_then(|config_options| config_options.boolean)
                .is_some()
        );
    }

    #[test]
    fn terminal_auth_task_builds_spawn_from_prebuilt_command() {
        let command = AgentServerCommand {
            path: "/path/to/agent".into(),
            args: vec!["--acp".into(), "--verbose".into(), "/auth".into()],
            env: Some(HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("SHARED".into(), "override".into()),
                ("EXTRA".into(), "2".into()),
            ])),
        };
        let method = acp::AuthMethodTerminal::new("login", "Login");

        let task = terminal_auth_task(&command, &AgentId::new("test-agent"), &method);

        assert_eq!(task.command.as_deref(), Some("/path/to/agent"));
        assert_eq!(task.args, vec!["--acp", "--verbose", "/auth"]);
        assert_eq!(
            task.env,
            HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("SHARED".into(), "override".into()),
                ("EXTRA".into(), "2".into()),
            ])
        );
        assert_eq!(task.label, "Login");
        assert_eq!(task.command_label, "Login");
    }

    #[test]
    fn legacy_terminal_auth_task_parses_meta_and_retries_session() {
        let method_id = acp::AuthMethodId::new("legacy-login");
        let method = acp::AuthMethod::Agent(
            acp::AuthMethodAgent::new(method_id.clone(), "Login").meta(acp::Meta::from_iter([(
                "terminal-auth".to_string(),
                serde_json::json!({
                    "label": "legacy /auth",
                    "command": "legacy-agent",
                    "args": ["auth", "--interactive"],
                    "env": {
                        "AUTH_MODE": "interactive",
                    },
                }),
            )])),
        );

        let task = meta_terminal_auth_task(&AgentId::new("test-agent"), &method_id, &method)
            .expect("expected legacy terminal auth task");

        assert_eq!(task.id.0, "external-agent-test-agent-legacy-login-login");
        assert_eq!(task.command.as_deref(), Some("legacy-agent"));
        assert_eq!(task.args, vec!["auth", "--interactive"]);
        assert_eq!(
            task.env,
            HashMap::from_iter([("AUTH_MODE".into(), "interactive".into())])
        );
        assert_eq!(task.label, "legacy /auth");
    }

    #[test]
    fn legacy_terminal_auth_task_returns_none_for_invalid_meta() {
        let method_id = acp::AuthMethodId::new("legacy-login");
        let method = acp::AuthMethod::Agent(
            acp::AuthMethodAgent::new(method_id.clone(), "Login").meta(acp::Meta::from_iter([(
                "terminal-auth".to_string(),
                serde_json::json!({
                    "label": "legacy /auth",
                }),
            )])),
        );

        assert!(
            meta_terminal_auth_task(&AgentId::new("test-agent"), &method_id, &method).is_none()
        );
    }

    #[test]
    fn first_class_terminal_auth_takes_precedence_over_legacy_meta() {
        let method_id = acp::AuthMethodId::new("login");
        let method = acp::AuthMethod::Terminal(
            acp::AuthMethodTerminal::new(method_id, "Login")
                .args(vec!["/auth".into()])
                .env(std::collections::HashMap::from_iter([(
                    "AUTH_MODE".into(),
                    "first-class".into(),
                )]))
                .meta(acp::Meta::from_iter([(
                    "terminal-auth".to_string(),
                    serde_json::json!({
                        "label": "legacy /auth",
                        "command": "legacy-agent",
                        "args": ["legacy-auth"],
                        "env": {
                            "AUTH_MODE": "legacy",
                        },
                    }),
                )])),
        );

        let command = AgentServerCommand {
            path: "/path/to/agent".into(),
            args: vec!["--acp".into(), "/auth".into()],
            env: Some(HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("AUTH_MODE".into(), "first-class".into()),
            ])),
        };

        let task = match &method {
            acp::AuthMethod::Terminal(terminal) => {
                terminal_auth_task(&command, &AgentId::new("test-agent"), terminal)
            }
            _ => unreachable!(),
        };

        assert_eq!(task.command.as_deref(), Some("/path/to/agent"));
        assert_eq!(task.args, vec!["--acp", "/auth"]);
        assert_eq!(
            task.env,
            HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("AUTH_MODE".into(), "first-class".into()),
            ])
        );
        assert_eq!(task.label, "Login");
    }

    #[test]
    fn trailing_stderr_only_uses_final_stderr_block() {
        let debug_log = AcpDebugLog::default();
        debug_log.record_line(AcpDebugMessageDirection::Stderr, "stale stderr");
        debug_log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"{"method":"initialized"}"#,
        );

        assert_eq!(debug_log.trailing_stderr(), None);

        debug_log.record_line(AcpDebugMessageDirection::Stderr, "recent stderr");
        assert_eq!(
            debug_log.trailing_stderr().as_deref(),
            Some("recent stderr")
        );
    }

    #[test]
    fn session_directories_use_ordered_paths_when_supported() {
        let work_dirs = PathList::new(&[
            std::path::PathBuf::from("/workspace-b"),
            std::path::PathBuf::from("/workspace-a"),
            std::path::PathBuf::from("/workspace-c"),
        ]);

        let directories =
            session_directories_from_work_dirs(&work_dirs, true).expect("work dirs should convert");

        assert_eq!(
            directories,
            SessionDirectories {
                cwd: std::path::PathBuf::from("/workspace-b"),
                additional_directories: vec![
                    std::path::PathBuf::from("/workspace-a"),
                    std::path::PathBuf::from("/workspace-c")
                ],
            }
        );

        let session_id = acp::SessionId::new("session-1");
        let new_session_request = directories.clone().into_new_session_request(Vec::new());
        let load_session_request = directories
            .clone()
            .into_load_session_request(session_id.clone(), Vec::new());
        let resume_session_request =
            directories.into_resume_session_request(session_id, Vec::new());

        assert_eq!(
            new_session_request.cwd,
            std::path::PathBuf::from("/workspace-b")
        );
        assert_eq!(
            new_session_request.additional_directories,
            vec![
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c")
            ]
        );
        assert_eq!(
            load_session_request.additional_directories,
            new_session_request.additional_directories
        );
        assert_eq!(
            resume_session_request.additional_directories,
            new_session_request.additional_directories
        );
    }

    #[test]
    fn session_directories_drop_additional_paths_when_unsupported() {
        let work_dirs = PathList::new(&[
            std::path::PathBuf::from("/workspace-b"),
            std::path::PathBuf::from("/workspace-a"),
        ]);

        let directories = session_directories_from_work_dirs(&work_dirs, false)
            .expect("work dirs should convert");

        assert_eq!(
            directories,
            SessionDirectories {
                cwd: std::path::PathBuf::from("/workspace-b"),
                additional_directories: Vec::new(),
            }
        );
    }

    #[test]
    fn session_info_work_dirs_preserve_cwd_then_additional_directories() {
        let work_dirs = work_dirs_from_session_info(
            std::path::PathBuf::from("/workspace-b"),
            vec![
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ],
        );

        assert_eq!(
            work_dirs.ordered_paths().cloned().collect::<Vec<_>>(),
            vec![
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ]
        );
    }

    #[test]
    fn session_info_work_dirs_deduplicate_cwd_and_additional_directories() {
        let work_dirs = work_dirs_from_session_info(
            std::path::PathBuf::from("/workspace-b"),
            vec![
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ],
        );

        assert_eq!(
            work_dirs.ordered_paths().cloned().collect::<Vec<_>>(),
            vec![
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ]
        );
    }

    #[gpui::test]
    async fn session_list_includes_additional_directories_in_work_dirs(
        cx: &mut gpui::TestAppContext,
    ) {
        let connection = connect_session_list_test_agent(
            vec![
                acp::SessionInfo::new("session-1", "/workspace-b").additional_directories(vec![
                    std::path::PathBuf::from("/workspace-a"),
                    std::path::PathBuf::from("/workspace-b"),
                    std::path::PathBuf::from("/workspace-a"),
                    std::path::PathBuf::from("/workspace-c"),
                ]),
            ],
            cx,
        )
        .await;
        let session_list = AcpSessionList::new(connection, false);

        let response = cx
            .update(|cx| session_list.list_sessions(AgentSessionListRequest::default(), cx))
            .await
            .expect("session list should load");
        let session = response
            .sessions
            .first()
            .expect("session list should include the returned session");
        let work_dirs = session
            .work_dirs
            .as_ref()
            .expect("session should include work dirs");

        assert_eq!(
            work_dirs.ordered_paths().cloned().collect::<Vec<_>>(),
            vec![
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ]
        );
    }

    async fn connect_session_list_test_agent(
        sessions: Vec<acp::SessionInfo>,
        cx: &mut gpui::TestAppContext,
    ) -> ConnectionTo<Agent> {
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        let sessions = Arc::new(sessions);

        cx.background_spawn(
            Agent
                .builder()
                .name("list-test-agent")
                .on_receive_request(
                    {
                        let sessions = sessions.clone();
                        async move |_request: acp::ListSessionsRequest, responder, _cx| {
                            responder.respond(acp::ListSessionsResponse::new((*sessions).clone()))
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        )
        .detach();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        cx.background_spawn(Client.builder().name("list-test-client").connect_with(
            client_transport,
            move |connection: ConnectionTo<Agent>| async move {
                connection_tx.send(connection).ok();
                futures::future::pending::<Result<(), acp::Error>>().await
            },
        ))
        .detach();

        connection_rx
            .await
            .expect("failed to receive ACP connection")
    }

    #[gpui::test]
    async fn additional_directories_support_respects_agent_capability(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {}, "b": {} }))
            .await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;
        let mut harness = test_support::connect_fake_acp_connection(project, cx).await;

        let work_dirs = PathList::new(&[
            std::path::PathBuf::from("/workspace-b"),
            std::path::PathBuf::from("/workspace-a"),
        ]);

        let missing_capability = harness
            .connection
            .session_directories_from_work_dirs(&work_dirs)
            .expect("work dirs should convert");
        assert!(missing_capability.additional_directories.is_empty());

        Rc::get_mut(&mut harness.connection)
            .expect("test harness should own the only ACP connection handle")
            .agent_capabilities
            .session_capabilities
            .additional_directories = Some(acp::SessionAdditionalDirectoriesCapabilities::new());

        let supported = harness
            .connection
            .session_directories_from_work_dirs(&work_dirs)
            .expect("work dirs should convert");
        assert_eq!(
            supported,
            SessionDirectories {
                cwd: std::path::PathBuf::from("/workspace-b"),
                additional_directories: vec![std::path::PathBuf::from("/workspace-a")],
            }
        );
    }

    async fn connect_session_delete_test_agent(
        deleted_sessions: Arc<std::sync::Mutex<Vec<acp::SessionId>>>,
        cx: &mut gpui::TestAppContext,
    ) -> ConnectionTo<Agent> {
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();

        cx.background_spawn(
            Agent
                .builder()
                .name("delete-test-agent")
                .on_receive_request(
                    {
                        let deleted_sessions = deleted_sessions.clone();
                        async move |request: acp::DeleteSessionRequest, responder, _cx| {
                            deleted_sessions
                                .lock()
                                .expect("deleted sessions lock should not be poisoned")
                                .push(request.session_id);
                            responder.respond(acp::DeleteSessionResponse::default())
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        )
        .detach();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        cx.background_spawn(Client.builder().name("delete-test-client").connect_with(
            client_transport,
            move |connection: ConnectionTo<Agent>| async move {
                connection_tx.send(connection).ok();
                futures::future::pending::<Result<(), acp::Error>>().await
            },
        ))
        .detach();

        connection_rx
            .await
            .expect("failed to receive ACP connection")
    }

    #[gpui::test]
    async fn settings_changes_refresh_active_connection_defaults(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;
        let harness = test_support::connect_fake_acp_connection(project, cx).await;

        cx.update(|cx| {
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::from_iter([(
                    "test".to_string(),
                    settings::CustomAgentServerSettings::Custom {
                        path: PathBuf::from("test-agent"),
                        args: Vec::new(),
                        env: HashMap::default(),
                        default_mode: Some("manual".to_string()),
                        default_config_options: HashMap::from_iter([(
                            "mode".to_string(),
                            AgentConfigOptionValue::from("manual"),
                        )]),
                        favorite_config_option_values: HashMap::default(),
                    }
                    .into(),
                )])),
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            harness.connection.defaults.mode(),
            Some(acp::SessionModeId::new("manual"))
        );
        assert_eq!(
            harness
                .connection
                .defaults
                .config_option("mode")
                .as_ref()
                .and_then(AgentConfigOptionValue::as_value_id),
            Some("manual"),
        );

        cx.update(|cx| {
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::default()),
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(harness.connection.defaults.mode(), None);
        assert_eq!(harness.connection.defaults.config_option("mode"), None);
    }

    #[gpui::test]
    async fn default_config_options_apply_boolean_defaults(cx: &mut gpui::TestAppContext) {
        let (connection, set_config_requests) = connect_config_defaults_test_agent(cx).await;
        connection.defaults.set(
            None,
            HashMap::from_iter([(
                "web_search".to_string(),
                AgentConfigOptionValue::Boolean(true),
            )]),
        );
        let config_options = Rc::new(RefCell::new(vec![acp::SessionConfigOption::boolean(
            "web_search",
            "Web Search",
            false,
        )]));

        let mut async_cx = cx.to_async();
        connection.apply_default_config_options(
            &acp::SessionId::new("session-config-defaults"),
            &config_options,
            &mut async_cx,
        );
        drop(async_cx);
        cx.run_until_parked();

        let requests = set_config_requests
            .lock()
            .expect("set config requests mutex poisoned");
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].config_id,
            acp::SessionConfigId::new("web_search")
        );
        assert_eq!(
            requests[0].value,
            acp::SessionConfigOptionValue::boolean(true)
        );

        let options = config_options.borrow();
        assert!(
            matches!(&options[0].kind, acp::SessionConfigKind::Boolean(boolean) if boolean.current_value)
        );
    }

    async fn connect_config_defaults_test_agent(
        cx: &mut gpui::TestAppContext,
    ) -> (
        AcpConnection,
        Arc<Mutex<Vec<acp::SetSessionConfigOptionRequest>>>,
    ) {
        let set_config_requests = Arc::new(Mutex::new(Vec::new()));
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();

        cx.background_spawn(
            Agent
                .builder()
                .name("config-defaults-test-agent")
                .on_receive_request(
                    {
                        let set_config_requests = set_config_requests.clone();
                        async move |req: acp::SetSessionConfigOptionRequest, responder, _cx| {
                            set_config_requests
                                .lock()
                                .expect("set config requests mutex poisoned")
                                .push(req);

                            responder.respond(acp::SetSessionConfigOptionResponse::new(Vec::new()))
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        )
        .detach();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let client_io_task = cx.background_spawn(async move {
            Client
                .builder()
                .name("config-defaults-test-client")
                .connect_with(
                    client_transport,
                    move |connection: ConnectionTo<Agent>| async move {
                        connection_tx.send(connection).ok();
                        futures::future::pending::<Result<(), acp::Error>>().await
                    },
                )
                .await
                .ok();
        });

        let client_conn = connection_rx
            .await
            .expect("failed to receive ACP connection");
        let sessions = Rc::new(RefCell::new(HashMap::default()));

        let connection = cx.update(|cx| {
            let request_elicitations = cx.new(|_| ElicitationStore::default());
            AcpConnection::new_for_test(
                client_conn,
                sessions,
                acp::AgentCapabilities::default(),
                request_elicitations,
                WeakEntity::new_invalid(),
                client_io_task,
                Task::ready(()),
                cx,
            )
        });

        (connection, set_config_requests)
    }

    #[gpui::test]
    async fn session_list_delete_sends_session_delete_when_supported(
        cx: &mut gpui::TestAppContext,
    ) {
        let deleted_sessions = Arc::new(std::sync::Mutex::new(Vec::new()));
        let connection = connect_session_delete_test_agent(deleted_sessions.clone(), cx).await;
        let session_list = AcpSessionList::new(connection, true);
        let session_id = acp::SessionId::new("session-to-delete");

        cx.update(|cx| session_list.delete_session(&session_id, cx))
            .await
            .expect("delete_session failed");

        assert_eq!(
            *deleted_sessions
                .lock()
                .expect("deleted sessions lock should not be poisoned"),
            vec![session_id]
        );
    }

    #[gpui::test]
    async fn session_list_delete_does_not_send_when_unsupported(cx: &mut gpui::TestAppContext) {
        let deleted_sessions = Arc::new(std::sync::Mutex::new(Vec::new()));
        let connection = connect_session_delete_test_agent(deleted_sessions.clone(), cx).await;
        let session_list = AcpSessionList::new(connection, false);
        let session_id = acp::SessionId::new("session-to-delete");

        let error = cx
            .update(|cx| session_list.delete_session(&session_id, cx))
            .await
            .expect_err("delete_session should fail when unsupported");

        assert!(
            error.to_string().contains("delete_session not supported"),
            "unexpected error: {error}"
        );
        assert!(
            deleted_sessions
                .lock()
                .expect("deleted sessions lock should not be poisoned")
                .is_empty()
        );
    }

    #[cfg(not(windows))]
    #[gpui::test]
    async fn startup_returns_error_when_agent_exits_before_initialization(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });
        cx.executor().allow_parking();

        let temp_dir = tempfile::tempdir().unwrap();
        let project = project::Project::example([temp_dir.path()], &mut cx.to_async()).await;
        let agent_server_store =
            project.read_with(cx, |project, _| project.agent_server_store().downgrade());
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec![
                "-c".into(),
                r#"printf '%s\n' 'npm error code ETARGET' 'npm error notarget No matching version found for @agentclientprotocol/claude-agent-acp@0.32.0 with a date before 4/28/2026, 12:11:38 PM.' >&2; exit 1"#.into(),
            ],
            env: None,
        };

        let mut async_cx = cx.to_async();
        let startup = AcpConnection::stdio(
            AgentId::new("test-agent"),
            project,
            command,
            agent_server_store,
            None,
            HashMap::default(),
            &mut async_cx,
        )
        .fuse();
        let timeout = cx
            .background_executor
            .timer(std::time::Duration::from_secs(5))
            .fuse();
        futures::pin_mut!(startup, timeout);

        let result = futures::select! {
            result = startup => result,
            _ = timeout => panic!("timed out waiting for failed ACP startup"),
        };

        let Err(error) = result else {
            panic!("expected ACP startup to fail");
        };
        let load_error = error
            .downcast::<LoadError>()
            .expect("startup failure should preserve the typed load error");
        match load_error {
            LoadError::Exited { status, .. } => {
                assert!(!status.success(), "expected non-zero exit status");
            }
            error => panic!("expected exited load error, got: {error:?}"),
        };
    }

    async fn connect_fake_agent(
        cx: &mut gpui::TestAppContext,
    ) -> (
        Rc<AcpConnection>,
        Entity<project::Project>,
        Arc<AtomicUsize>,
        Arc<AtomicUsize>,
        Arc<std::sync::Mutex<Vec<acp::SessionUpdate>>>,
        Arc<std::sync::Mutex<Option<async_channel::Receiver<()>>>>,
        Task<anyhow::Result<()>>,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let load_count = Arc::new(AtomicUsize::new(0));
        let close_count = Arc::new(AtomicUsize::new(0));
        let load_session_updates: Arc<std::sync::Mutex<Vec<acp::SessionUpdate>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let load_session_gate: Arc<std::sync::Mutex<Option<async_channel::Receiver<()>>>> =
            Arc::new(std::sync::Mutex::new(None));

        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();

        let sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>> =
            Rc::new(RefCell::new(HashMap::default()));
        let client_session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>> =
            Rc::new(RefCell::new(None));

        // Build the fake agent side. It handles the requests issued by
        // `AcpConnection` during the test and tracks load/close counts.
        let agent_future = Agent
            .builder()
            .name("fake-agent")
            .on_receive_request(
                async move |req: acp::InitializeRequest, responder, _cx| {
                    responder.respond(
                        acp::InitializeResponse::new(req.protocol_version).agent_capabilities(
                            acp::AgentCapabilities::default()
                                .load_session(true)
                                .session_capabilities(
                                    acp::SessionCapabilities::default()
                                        .close(acp::SessionCloseCapabilities::new()),
                                ),
                        ),
                    )
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::AuthenticateRequest, responder, _cx| {
                    responder.respond(Default::default())
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::NewSessionRequest, responder, _cx| {
                    responder.respond(acp::NewSessionResponse::new(acp::SessionId::new("unused")))
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::PromptRequest, responder, _cx| {
                    responder.respond(acp::PromptResponse::new(acp::StopReason::EndTurn))
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let load_count = load_count.clone();
                    let load_session_updates = load_session_updates.clone();
                    let load_session_gate = load_session_gate.clone();
                    async move |req: acp::LoadSessionRequest, responder, cx| {
                        load_count.fetch_add(1, Ordering::SeqCst);

                        // Simulate spec-compliant history replay: send
                        // notifications to the client before responding to the
                        // load request.
                        let updates = std::mem::take(
                            &mut *load_session_updates
                                .lock()
                                .expect("load_session_updates mutex poisoned"),
                        );
                        for update in updates {
                            cx.send_notification(acp::SessionNotification::new(
                                req.session_id.clone(),
                                update,
                            ))?;
                        }

                        // If a gate was installed, park on it before responding
                        // so tests can interleave other work (e.g.
                        // `close_session`) with an in-flight load.
                        let gate = load_session_gate
                            .lock()
                            .expect("load_session_gate mutex poisoned")
                            .take();
                        if let Some(gate) = gate {
                            gate.recv().await.ok();
                        }

                        responder.respond(acp::LoadSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let close_count = close_count.clone();
                    async move |_req: acp::CloseSessionRequest, responder, _cx| {
                        close_count.fetch_add(1, Ordering::SeqCst);
                        responder.respond(acp::CloseSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_notification(
                async move |_notif: acp::CancelNotification, _cx| Ok(()),
                agent_client_protocol::on_receive_notification!(),
            )
            .connect_to(agent_transport);

        let agent_io_task = cx.background_spawn(agent_future);

        // Wire the production handler set into the fake client so inbound
        // requests/notifications from the fake agent reach the same
        // dispatcher that the real `stdio` path uses.
        let (dispatch_tx, dispatch_rx) = mpsc::unbounded::<ForegroundWork>();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let client_future = connect_client_future(
            "zed-test",
            client_transport,
            dispatch_tx.clone(),
            connection_tx,
        );
        let client_io_task = cx.background_spawn(async move {
            client_future.await.ok();
        });

        let client_conn: ConnectionTo<Agent> = connection_rx
            .await
            .expect("failed to receive ACP connection handle");

        let response = client_conn
            .send_request(acp::InitializeRequest::new(ProtocolVersion::V1))
            .block_task()
            .await
            .expect("failed to initialize ACP connection");

        let agent_capabilities = response.agent_capabilities;

        let request_elicitations = cx.new(|_| ElicitationStore::default());
        let dispatch_context = ClientContext {
            sessions: sessions.clone(),
            session_list: client_session_list.clone(),
            request_elicitations: request_elicitations.clone(),
        };
        // `TestAppContext::spawn` hands out an `AsyncApp` by value, whereas the
        // production path uses `Context::spawn` which hands out `&mut AsyncApp`.
        // Bind the value-form to a local and take `&mut` of it to reuse the
        // same dispatch loop shape.
        let dispatch_task = cx.spawn({
            let mut dispatch_rx = dispatch_rx;
            move |cx| async move {
                let mut cx = cx;
                while let Some(work) = dispatch_rx.next().await {
                    work.run(&mut cx, &dispatch_context);
                }
            }
        });

        let agent_server_store =
            project.read_with(cx, |project, _| project.agent_server_store().downgrade());

        let connection = cx.update(|cx| {
            AcpConnection::new_for_test(
                client_conn,
                sessions,
                agent_capabilities,
                request_elicitations,
                agent_server_store,
                client_io_task,
                dispatch_task,
                cx,
            )
        });

        let keep_agent_alive = cx.background_spawn(async move {
            agent_io_task.await.ok();
            anyhow::Ok(())
        });

        (
            Rc::new(connection),
            project,
            load_count,
            close_count,
            load_session_updates,
            load_session_gate,
            keep_agent_alive,
        )
    }

    #[gpui::test]
    async fn test_loaded_sessions_keep_state_until_last_close(cx: &mut gpui::TestAppContext) {
        let (
            connection,
            project,
            load_count,
            close_count,
            _load_session_updates,
            _load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(cx).await;

        let session_id = acp::SessionId::new("session-1");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        // Load the same session twice concurrently — the second call should join
        // the pending task rather than issuing a second ACP load_session RPC.
        let first_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });
        let second_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });

        let first_thread = first_load.await.expect("first load failed");
        let second_thread = second_load.await.expect("second load failed");
        cx.run_until_parked();

        assert_eq!(
            first_thread.entity_id(),
            second_thread.entity_id(),
            "concurrent loads for the same session should share one AcpThread"
        );
        assert_eq!(
            load_count.load(Ordering::SeqCst),
            1,
            "underlying ACP load_session should be called exactly once for concurrent loads"
        );

        // The session has ref_count 2. The first close should not send the ACP
        // close_session RPC — the session is still referenced.
        cx.update(|cx| connection.clone().close_session(&session_id, cx))
            .await
            .expect("first close failed");

        assert_eq!(
            close_count.load(Ordering::SeqCst),
            0,
            "ACP close_session should not be sent while ref_count > 0"
        );
        assert!(
            connection.sessions.borrow().contains_key(&session_id),
            "session should still be tracked after first close"
        );

        // The second close drops ref_count to 0 — now the ACP RPC must be sent.
        cx.update(|cx| connection.clone().close_session(&session_id, cx))
            .await
            .expect("second close failed");
        cx.run_until_parked();

        assert_eq!(
            close_count.load(Ordering::SeqCst),
            1,
            "ACP close_session should be sent exactly once when ref_count reaches 0"
        );
        assert!(
            !connection.sessions.borrow().contains_key(&session_id),
            "session should be removed after final close"
        );
    }

    // Regression test: per the ACP spec, an agent replays the entire conversation
    // history as `session/update` notifications *before* responding to the
    // `session/load` request. These notifications must be applied to the
    // reconstructed thread, not dropped because the session hasn't been
    // registered yet.
    #[gpui::test]
    async fn test_load_session_replays_notifications_sent_before_response(
        cx: &mut gpui::TestAppContext,
    ) {
        let (
            connection,
            project,
            _load_count,
            _close_count,
            load_session_updates,
            _load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(cx).await;

        // Queue up some history updates that the fake agent will stream to
        // the client during the `load_session` call, before responding.
        *load_session_updates
            .lock()
            .expect("load_session_updates mutex poisoned") = vec![
            acp::SessionUpdate::UserMessageChunk(acp::ContentChunk::new(acp::ContentBlock::Text(
                acp::TextContent::new(String::from("hello agent")),
            ))),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(acp::ContentBlock::Text(
                acp::TextContent::new(String::from("hi user")),
            ))),
        ];

        let session_id = acp::SessionId::new("session-replay");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project.clone(),
                    work_dirs,
                    None,
                    cx,
                )
            })
            .await
            .expect("load_session failed");
        cx.run_until_parked();

        let entries = thread.read_with(cx, |thread, _| {
            thread
                .entries()
                .iter()
                .map(|entry| match entry {
                    acp_thread::AgentThreadEntry::UserMessage(_) => "user",
                    acp_thread::AgentThreadEntry::AssistantMessage(_) => "assistant",
                    acp_thread::AgentThreadEntry::ToolCall(_) => "tool_call",
                    acp_thread::AgentThreadEntry::Elicitation(_) => "elicitation",
                    acp_thread::AgentThreadEntry::CompletedPlan(_) => "plan",
                    acp_thread::AgentThreadEntry::ContextCompaction(_) => "compaction",
                })
                .collect::<Vec<_>>()
        });

        assert_eq!(
            entries,
            vec!["user", "assistant"],
            "replayed notifications should be applied to the thread"
        );
    }

    // Regression test: if `close_session` is issued while a `load_session`
    // RPC is still in flight, the close must take effect cleanly — the load
    // must fail with a recognizable error (not return an orphaned thread),
    // no entry must remain in `sessions` or `pending_sessions`, and the ACP
    // `close_session` RPC must be dispatched.
    #[gpui::test]
    async fn test_close_session_during_in_flight_load(cx: &mut gpui::TestAppContext) {
        let (
            connection,
            project,
            load_count,
            close_count,
            _load_session_updates,
            load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(cx).await;

        // Install a gate so the fake agent's `load_session` handler parks
        // before sending its response. We'll close the session while the
        // load is parked.
        let (gate_tx, gate_rx) = async_channel::bounded::<()>(1);
        *load_session_gate
            .lock()
            .expect("load_session_gate mutex poisoned") = Some(gate_rx);

        let session_id = acp::SessionId::new("session-close-during-load");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        let load_task = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs,
                None,
                cx,
            )
        });

        // Let the load RPC reach the agent and park on the gate.
        cx.run_until_parked();
        assert_eq!(
            load_count.load(Ordering::SeqCst),
            1,
            "load_session RPC should have been dispatched"
        );
        assert!(
            connection
                .pending_sessions
                .borrow()
                .contains_key(&session_id),
            "pending_sessions entry should exist while load is in flight"
        );
        assert!(
            connection.sessions.borrow().contains_key(&session_id),
            "sessions entry should be pre-registered to receive replay notifications"
        );

        // Close the session while the load is still parked. This should take
        // the pending path and dispatch the ACP close RPC.
        let close_task = cx.update(|cx| connection.clone().close_session(&session_id, cx));

        // Release the gate so the load RPC can finally respond.
        gate_tx.send(()).await.expect("gate send failed");
        drop(gate_tx);

        let load_result = load_task.await;
        close_task.await.expect("close failed");
        cx.run_until_parked();

        let err = load_result.expect_err("load should fail after close-during-load");
        assert!(
            err.to_string()
                .contains("session was closed before load completed"),
            "expected close-during-load error, got: {err}"
        );

        assert_eq!(
            close_count.load(Ordering::SeqCst),
            1,
            "ACP close_session should be sent exactly once"
        );
        assert!(
            !connection.sessions.borrow().contains_key(&session_id),
            "sessions entry should be removed after close-during-load"
        );
        assert!(
            !connection
                .pending_sessions
                .borrow()
                .contains_key(&session_id),
            "pending_sessions entry should be removed after close-during-load"
        );
    }

    // Regression test: when two concurrent `load_session` calls share a pending
    // task and one of them issues `close_session` before the load RPC
    // resolves, the remaining load must still succeed and the session must
    // stay live. If `close_session` incorrectly short-circuits via the
    // `sessions` path (removing the entry while a load is still in flight),
    // the pending task will fail and both concurrent loaders will lose
    // their handle.
    #[gpui::test]
    async fn test_close_during_load_preserves_other_concurrent_loader(
        cx: &mut gpui::TestAppContext,
    ) {
        let (
            connection,
            project,
            load_count,
            close_count,
            _load_session_updates,
            load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(cx).await;

        let (gate_tx, gate_rx) = async_channel::bounded::<()>(1);
        *load_session_gate
            .lock()
            .expect("load_session_gate mutex poisoned") = Some(gate_rx);

        let session_id = acp::SessionId::new("session-concurrent-close");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        // Kick off two concurrent loads; the second must join the first's pending
        // task rather than issuing a second RPC.
        let first_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });
        let second_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });

        cx.run_until_parked();
        assert_eq!(
            load_count.load(Ordering::SeqCst),
            1,
            "load_session RPC should only be dispatched once for concurrent loads"
        );

        // Close one of the two handles while the shared load is still parked.
        // Because a second loader still holds a pending ref, this should be a
        // no-op on the wire.
        cx.update(|cx| connection.clone().close_session(&session_id, cx))
            .await
            .expect("close during load failed");
        assert_eq!(
            close_count.load(Ordering::SeqCst),
            0,
            "close_session RPC must not be dispatched while another load handle remains"
        );

        // Release the gate so the load RPC can finally respond.
        gate_tx.send(()).await.expect("gate send failed");
        drop(gate_tx);

        let first_thread = first_load.await.expect("first load should still succeed");
        let second_thread = second_load.await.expect("second load should still succeed");
        cx.run_until_parked();

        assert_eq!(
            first_thread.entity_id(),
            second_thread.entity_id(),
            "concurrent loads should share one AcpThread"
        );
        assert!(
            connection.sessions.borrow().contains_key(&session_id),
            "session must remain tracked while a load handle is still outstanding"
        );
        assert!(
            !connection
                .pending_sessions
                .borrow()
                .contains_key(&session_id),
            "pending_sessions entry should be cleared once the load resolves"
        );

        // Final close drops ref_count to 0 and dispatches the ACP close RPC.
        cx.update(|cx| connection.clone().close_session(&session_id, cx))
            .await
            .expect("final close failed");
        cx.run_until_parked();
        assert_eq!(
            close_count.load(Ordering::SeqCst),
            1,
            "close_session RPC should fire exactly once when the last handle is released"
        );
        assert!(
            !connection.sessions.borrow().contains_key(&session_id),
            "session should be removed after final close"
        );
    }
}

fn mcp_servers_for_project(project: &Entity<Project>, cx: &App) -> Vec<acp::McpServer> {
    let context_server_store = project.read(cx).context_server_store().read(cx);
    let is_local = project.read(cx).is_local();
    context_server_store
        .configured_server_ids()
        .iter()
        .filter_map(|id| {
            let configuration = context_server_store.configuration_for_server(id)?;
            match &*configuration {
                project::context_server_store::ContextServerConfiguration::Custom {
                    command,
                    remote,
                    ..
                }
                | project::context_server_store::ContextServerConfiguration::Extension {
                    command,
                    remote,
                    ..
                } if is_local || *remote => Some(acp::McpServer::Stdio(
                    acp::McpServerStdio::new(id.0.to_string(), &command.path)
                        .args(command.args.clone())
                        .env(if let Some(env) = command.env.as_ref() {
                            env.iter()
                                .map(|(name, value)| acp::EnvVariable::new(name, value))
                                .collect()
                        } else {
                            vec![]
                        }),
                )),
                project::context_server_store::ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout: _,
                    oauth: _,
                } => Some(acp::McpServer::Http(
                    acp::McpServerHttp::new(id.0.to_string(), url.to_string()).headers(
                        headers
                            .iter()
                            .map(|(name, value)| acp::HttpHeader::new(name, value))
                            .collect(),
                    ),
                )),
                _ => None,
            }
        })
        .collect()
}

fn config_state(
    modes: Option<acp::SessionModeState>,
    config_options: Option<Vec<acp::SessionConfigOption>>,
) -> (
    Option<Rc<RefCell<acp::SessionModeState>>>,
    Option<Rc<RefCell<Vec<acp::SessionConfigOption>>>>,
) {
    if let Some(opts) = config_options {
        return (None, Some(Rc::new(RefCell::new(opts))));
    }

    let modes = modes.map(|modes| Rc::new(RefCell::new(modes)));
    (modes, None)
}

struct AcpSessionModes {
    session_id: acp::SessionId,
    connection: ConnectionTo<Agent>,
    state: Rc<RefCell<acp::SessionModeState>>,
}

impl acp_thread::AgentSessionModes for AcpSessionModes {
    fn current_mode(&self) -> acp::SessionModeId {
        self.state.borrow().current_mode_id.clone()
    }

    fn all_modes(&self) -> Vec<acp::SessionMode> {
        self.state.borrow().available_modes.clone()
    }

    fn set_mode(&self, mode_id: acp::SessionModeId, cx: &mut App) -> Task<Result<()>> {
        let connection = self.connection.clone();
        let session_id = self.session_id.clone();
        let old_mode_id;
        {
            let mut state = self.state.borrow_mut();
            old_mode_id = state.current_mode_id.clone();
            state.current_mode_id = mode_id.clone();
        };
        let state = self.state.clone();
        cx.foreground_executor().spawn(async move {
            let result = connection
                .send_request(acp::SetSessionModeRequest::new(session_id, mode_id))
                .block_task()
                .await;

            if result.is_err() {
                state.borrow_mut().current_mode_id = old_mode_id;
            }

            result?;

            Ok(())
        })
    }
}

struct AcpSessionConfigOptions {
    session_id: acp::SessionId,
    connection: ConnectionTo<Agent>,
    state: Rc<RefCell<Vec<acp::SessionConfigOption>>>,
    watch_tx: Rc<RefCell<watch::Sender<()>>>,
    watch_rx: watch::Receiver<()>,
}

impl acp_thread::AgentSessionConfigOptions for AcpSessionConfigOptions {
    fn config_options(&self) -> Vec<acp::SessionConfigOption> {
        self.state.borrow().clone()
    }

    fn set_config_option(
        &self,
        config_id: acp::SessionConfigId,
        value: acp::SessionConfigOptionValue,
        cx: &mut App,
    ) -> Task<Result<Vec<acp::SessionConfigOption>>> {
        let connection = self.connection.clone();
        let session_id = self.session_id.clone();
        let state = self.state.clone();

        let watch_tx = self.watch_tx.clone();

        cx.foreground_executor().spawn(async move {
            let response = connection
                .send_request(acp::SetSessionConfigOptionRequest::new(
                    session_id, config_id, value,
                ))
                .block_task()
                .await?;

            *state.borrow_mut() = response.config_options.clone();
            watch_tx.borrow_mut().send(()).ok();
            Ok(response.config_options)
        })
    }

    fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
        Some(self.watch_rx.clone())
    }
}

// ---------------------------------------------------------------------------
// Handler functions dispatched from background handler closures to the
// foreground thread via the ForegroundWork channel.
// ---------------------------------------------------------------------------

fn session_thread(
    ctx: &ClientContext,
    session_id: &acp::SessionId,
) -> Result<WeakEntity<AcpThread>, acp::Error> {
    let sessions = ctx.sessions.borrow();
    sessions
        .get(session_id)
        .map(|session| session.thread.clone())
        .ok_or_else(|| acp::Error::internal_error().data(format!("unknown session: {session_id}")))
}

fn respond_err<T: JsonRpcResponse>(responder: Responder<T>, err: acp::Error) {
    // Log the actual error we're returning — otherwise agents that hit an
    // error path (e.g. unknown session) would see only the generic internal
    // error returned over the wire with no trace of why on the client side.
    log::warn!(
        "Responding to ACP request `{method}` with error: {err:?}",
        method = responder.method()
    );
    responder.respond_with_error(err).log_err();
}

fn respond_result<T: JsonRpcResponse>(responder: Responder<T>, result: Result<T, acp::Error>) {
    match result {
        Ok(response) => {
            responder.respond(response).log_err();
        }
        Err(err) => respond_err(responder, err),
    }
}

fn handle_request_permission(
    args: acp::RequestPermissionRequest,
    responder: Responder<acp::RequestPermissionResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    let cancellation = responder.cancellation();
    let tool_call_id = args.tool_call.tool_call_id.clone();
    cx.spawn(async move |cx| {
        let result: Result<_, acp::Error> = async {
            let task = thread
                .update(cx, |thread, cx| {
                    thread.request_tool_call_authorization(
                        args.tool_call,
                        acp_thread::PermissionOptions::Flat(args.options),
                        acp_thread::AuthorizationKind::PermissionGrant,
                        cx,
                    )
                })
                .flatten_acp()?;
            cancellation
                .run_until_cancelled(async { Ok(task.await) })
                .await
        }
        .await;

        match result {
            Ok(outcome) => {
                responder
                    .respond(acp::RequestPermissionResponse::new(outcome.into()))
                    .log_err();
            }
            Err(e) => {
                if e.code == ErrorCode::RequestCancelled {
                    thread
                        .update(cx, |thread, cx| {
                            thread.cancel_tool_call_authorization(&tool_call_id, cx)
                        })
                        .log_err();
                }
                respond_err(responder, e)
            }
        }
    })
    .detach();
}

fn handle_create_elicitation(
    args: acp::CreateElicitationRequest,
    responder: Responder<acp::CreateElicitationResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    match args.scope() {
        acp::ElicitationScope::Session(scope) => {
            let thread = match session_thread(ctx, &scope.session_id) {
                Ok(t) => t,
                Err(e) => return respond_err(responder, e),
            };

            let (elicitation_id, task) = match thread
                .update(cx, |thread, cx| {
                    thread.request_elicitation_with_id(args, cx)
                })
                .flatten_acp()
            {
                Ok(task) => task,
                Err(e) => return respond_err(responder, e),
            };

            let cancellation = responder.cancellation();
            cx.spawn(async move |cx| {
                let result: Result<_, acp::Error> = cancellation
                    .run_until_cancelled(async { Ok(task.await) })
                    .await;

                match result {
                    Ok(response) => {
                        responder.respond(response).log_err();
                    }
                    Err(e) => {
                        if e.code == ErrorCode::RequestCancelled {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.cancel_elicitation(&elicitation_id, cx)
                                })
                                .log_err();
                        }
                        respond_err(responder, e);
                    }
                }
            })
            .detach();
        }
        acp::ElicitationScope::Request(_) => {
            let store = ctx.request_elicitations.clone();
            let (elicitation_id, task) =
                match store.update(cx, |store, cx| store.request_elicitation_with_id(args, cx)) {
                    Ok(task) => task,
                    Err(e) => return respond_err(responder, e),
                };
            let store = store.downgrade();

            let cancellation = responder.cancellation();
            cx.spawn(async move |cx| {
                let result: Result<_, acp::Error> = cancellation
                    .run_until_cancelled(async { Ok(task.await) })
                    .await;

                match result {
                    Ok(response) => {
                        responder.respond(response).log_err();
                    }
                    Err(e) => {
                        if e.code == ErrorCode::RequestCancelled {
                            store
                                .update(cx, |store, cx| {
                                    store.cancel_elicitation(&elicitation_id, cx)
                                })
                                .log_err();
                        }
                        respond_err(responder, e);
                    }
                }
            })
            .detach();
        }
        _ => {
            respond_err(
                responder,
                acp::Error::invalid_params().data("unknown elicitation scope"),
            );
        }
    }
}

fn handle_complete_elicitation(
    args: acp::CompleteElicitationNotification,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let threads = ctx
        .sessions
        .borrow()
        .values()
        .map(|session| session.thread.clone())
        .collect::<Vec<_>>();
    let request_elicitations = ctx.request_elicitations.clone();
    let elicitation_id = args.elicitation_id;

    cx.spawn(async move |cx| {
        for thread in threads {
            thread
                .update(cx, |thread, cx| {
                    thread.complete_url_elicitation(&elicitation_id, cx);
                })
                .ok();
        }
        request_elicitations.update(cx, |store, cx| {
            store.complete_url_elicitation(&elicitation_id, cx);
        });
    })
    .detach();
}

fn handle_write_text_file(
    args: acp::WriteTextFileRequest,
    responder: Responder<acp::WriteTextFileResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let result: Result<_, acp::Error> = async {
            thread
                .update(cx, |thread, cx| {
                    thread.write_text_file(args.path, args.content, cx)
                })
                .map_err(acp::Error::from)?
                .await?;
            Ok(())
        }
        .await;

        match result {
            Ok(()) => {
                responder
                    .respond(acp::WriteTextFileResponse::default())
                    .log_err();
            }
            Err(e) => respond_err(responder, e),
        }
    })
    .detach();
}

fn handle_read_text_file(
    args: acp::ReadTextFileRequest,
    responder: Responder<acp::ReadTextFileResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let cancellation = responder.cancellation();
        let result = cancellation
            .run_until_cancelled(async {
                thread
                    .update(cx, |thread, cx| {
                        thread.read_text_file(args.path, args.line, args.limit, false, cx)
                    })
                    .map_err(acp::Error::from)?
                    .await
            })
            .await;

        respond_result(responder, result.map(acp::ReadTextFileResponse::new));
    })
    .detach();
}

fn handle_session_notification(
    notification: acp::SessionNotification,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    // Extract everything we need from the session while briefly borrowing.
    let (thread, session_modes, config_opts_data) = {
        let sessions = ctx.sessions.borrow();
        let Some(session) = sessions.get(&notification.session_id) else {
            log::warn!(
                "Received session notification for unknown session: {:?}",
                notification.session_id
            );
            return;
        };
        (
            session.thread.clone(),
            session.session_modes.clone(),
            session
                .config_options
                .as_ref()
                .map(|opts| (opts.config_options.clone(), opts.tx.clone())),
        )
    };
    // Borrow is dropped here.

    // Apply mode/config/session_list updates without holding the borrow.
    if let acp::SessionUpdate::CurrentModeUpdate(acp::CurrentModeUpdate {
        current_mode_id, ..
    }) = &notification.update
    {
        if let Some(session_modes) = &session_modes {
            session_modes.borrow_mut().current_mode_id = current_mode_id.clone();
        }
    }

    if let acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate {
        config_options, ..
    }) = &notification.update
    {
        if let Some((config_opts_cell, tx_cell)) = &config_opts_data {
            *config_opts_cell.borrow_mut() = config_options.clone();
            tx_cell.borrow_mut().send(()).ok();
        }
    }

    if let acp::SessionUpdate::SessionInfoUpdate(info_update) = &notification.update
        && let Some(session_list) = ctx.session_list.borrow().as_ref()
    {
        session_list.send_info_update(notification.session_id.clone(), info_update.clone());
    }

    // Pre-handle: if a ToolCall carries terminal_info, create/register a display-only terminal.
    if let acp::SessionUpdate::ToolCall(tc) = &notification.update {
        if let Some(meta) = &tc.meta {
            if let Some(terminal_info) = meta.get("terminal_info") {
                if let Some(id_str) = terminal_info.get("terminal_id").and_then(|v| v.as_str()) {
                    let terminal_id = acp::TerminalId::new(id_str);
                    let cwd = terminal_info
                        .get("cwd")
                        .and_then(|v| v.as_str().map(PathBuf::from));

                    thread
                        .update(cx, |thread, cx| {
                            let builder = TerminalBuilder::new_display_only(
                                CursorShape::default(),
                                AlternateScroll::On,
                                None,
                                0,
                                cx.background_executor(),
                                thread.project().read(cx).path_style(cx),
                            );
                            let lower = cx.new(|cx| builder.subscribe(cx));
                            thread.on_terminal_provider_event(
                                TerminalProviderEvent::Created {
                                    terminal_id,
                                    label: tc.title.clone(),
                                    cwd,
                                    output_byte_limit: None,
                                    terminal: lower,
                                },
                                cx,
                            );
                        })
                        .log_err();
                }
            }
        }
    }

    // Forward the update to the acp_thread as usual.
    if let Err(err) = thread
        .update(cx, |thread, cx| {
            thread.handle_session_update(notification.update.clone(), cx)
        })
        .flatten_acp()
    {
        log::error!(
            "Failed to handle session update for {:?}: {err:?}",
            notification.session_id
        );
    }

    // Post-handle: stream terminal output/exit if present on ToolCallUpdate meta.
    if let acp::SessionUpdate::ToolCallUpdate(tcu) = &notification.update {
        if let Some(meta) = &tcu.meta {
            if let Some(term_out) = meta.get("terminal_output") {
                if let Some(id_str) = term_out.get("terminal_id").and_then(|v| v.as_str()) {
                    let terminal_id = acp::TerminalId::new(id_str);
                    if let Some(s) = term_out.get("data").and_then(|v| v.as_str()) {
                        let data = s.as_bytes().to_vec();
                        thread
                            .update(cx, |thread, cx| {
                                thread.on_terminal_provider_event(
                                    TerminalProviderEvent::Output { terminal_id, data },
                                    cx,
                                );
                            })
                            .log_err();
                    }
                }
            }

            if let Some(term_exit) = meta.get("terminal_exit") {
                if let Some(id_str) = term_exit.get("terminal_id").and_then(|v| v.as_str()) {
                    let terminal_id = acp::TerminalId::new(id_str);
                    let status = acp::TerminalExitStatus::new()
                        .exit_code(
                            term_exit
                                .get("exit_code")
                                .and_then(|v| v.as_u64())
                                .map(|i| i as u32),
                        )
                        .signal(
                            term_exit
                                .get("signal")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        );

                    thread
                        .update(cx, |thread, cx| {
                            thread.on_terminal_provider_event(
                                TerminalProviderEvent::Exit {
                                    terminal_id,
                                    status,
                                },
                                cx,
                            );
                        })
                        .log_err();
                }
            }
        }
    }
}

fn handle_create_terminal(
    args: acp::CreateTerminalRequest,
    responder: Responder<acp::CreateTerminalResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };
    let project = match thread
        .read_with(cx, |thread, _cx| thread.project().clone())
        .map_err(acp::Error::from)
    {
        Ok(p) => p,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let result: Result<_, acp::Error> = async {
            let terminal_entity = acp_thread::create_terminal_entity(
                args.command.clone(),
                &args.args,
                args.env
                    .into_iter()
                    .map(|env| (env.name, env.value))
                    .collect(),
                args.cwd.clone(),
                &project,
                cx,
            )
            .await?;

            let terminal_entity = thread.update(cx, |thread, cx| {
                thread.register_terminal_created(
                    acp::TerminalId::new(uuid::Uuid::new_v4().to_string()),
                    format!("{} {}", args.command, args.args.join(" ")),
                    args.cwd.clone(),
                    args.output_byte_limit,
                    terminal_entity,
                    cx,
                )
            })?;
            let terminal_id = terminal_entity.read_with(cx, |terminal, _| terminal.id().clone());
            Ok(terminal_id)
        }
        .await;

        match result {
            Ok(terminal_id) => {
                responder
                    .respond(acp::CreateTerminalResponse::new(terminal_id))
                    .log_err();
            }
            Err(e) => respond_err(responder, e),
        }
    })
    .detach();
}

fn handle_kill_terminal(
    args: acp::KillTerminalRequest,
    responder: Responder<acp::KillTerminalResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    match thread
        .update(cx, |thread, cx| thread.kill_terminal(args.terminal_id, cx))
        .flatten_acp()
    {
        Ok(()) => {
            responder
                .respond(acp::KillTerminalResponse::default())
                .log_err();
        }
        Err(e) => respond_err(responder, e),
    }
}

fn handle_release_terminal(
    args: acp::ReleaseTerminalRequest,
    responder: Responder<acp::ReleaseTerminalResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    match thread
        .update(cx, |thread, cx| {
            thread.release_terminal(args.terminal_id, cx)
        })
        .flatten_acp()
    {
        Ok(()) => {
            responder
                .respond(acp::ReleaseTerminalResponse::default())
                .log_err();
        }
        Err(e) => respond_err(responder, e),
    }
}

fn handle_terminal_output(
    args: acp::TerminalOutputRequest,
    responder: Responder<acp::TerminalOutputResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    match thread
        .read_with(cx, |thread, cx| -> anyhow::Result<_> {
            let out = thread
                .terminal(args.terminal_id)?
                .read(cx)
                .current_output(cx);
            Ok(out)
        })
        .flatten_acp()
    {
        Ok(output) => {
            responder.respond(output).log_err();
        }
        Err(e) => respond_err(responder, e),
    }
}

fn handle_wait_for_terminal_exit(
    args: acp::WaitForTerminalExitRequest,
    responder: Responder<acp::WaitForTerminalExitResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &args.session_id) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let cancellation = responder.cancellation();
        let result = cancellation
            .run_until_cancelled(async {
                let exit_status = thread
                    .update(cx, |thread, cx| {
                        anyhow::Ok(thread.terminal(args.terminal_id)?.read(cx).wait_for_exit())
                    })
                    .flatten_acp()?
                    .await;
                Ok(exit_status)
            })
            .await;

        respond_result(responder, result.map(acp::WaitForTerminalExitResponse::new));
    })
    .detach();
}
