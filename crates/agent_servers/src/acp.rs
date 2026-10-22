mod debug_log;
mod runtime;
mod session_updates;
mod terminal_updates;
mod transport;
mod v1;
mod v2;

use debug_log::AcpDebugLog;
pub use debug_log::{AcpDebugMessage, AcpDebugMessageContent, AcpDebugMessageDirection};
use session_updates::{InboundSessionNotification, InboundSessionUpdate, PendingSessionUpdates};
pub use terminal_updates::{DecodedTerminalNotification, DecodedTerminalUpdate};
#[cfg(any(test, feature = "test-support"))]
pub use v1::test_support;
pub use v1::{AcpConnection, AcpSessionList, GEMINI_TERMINAL_AUTH_METHOD_ID};
pub use v2::AcpV2Connection;

use acp_thread::{AcpThread, AuthRequired, ElicitationStore, LoadError};
use action_log::ActionLog;
use agent_client_protocol::schema::{
    v1::{self as acp, ErrorCode},
    v2 as acp_v2,
};
use agent_client_protocol::{
    Agent, Client, HandleDispatchFrom, Handled, JsonRpcMessage, JsonRpcResponse, Responder,
    UntypedMessage, V2Builder, V2ConnectionTo,
};
use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use feature_flags::{AcpBetaFeatureFlag, FeatureFlagAppExt as _};
use futures::{Future, channel::mpsc};
use gpui::{App, AppContext as _, AsyncApp, Entity, SharedString, Subscription, Task, WeakEntity};
use project::agent_server_store::{AgentServerCommand, AgentServerStore};
use project::context_server_store::ContextServerConfiguration;
use project::{AgentId, Project};
use std::{cell::RefCell, path::PathBuf, process::ExitStatus, rc::Rc, sync::Arc};
use task::SpawnInTerminal;
use thiserror::Error;
use util::ResultExt as _;
use util::path_list::PathList;

const EXIT_DRAIN_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(250);

fn mcp_configurations_for_project(
    project: &Entity<Project>,
    cx: &App,
) -> Vec<(String, Arc<ContextServerConfiguration>)> {
    let store = project.read(cx).context_server_store().read(cx);
    let is_local = project.read(cx).is_local();
    store
        .configured_server_ids()
        .iter()
        .filter_map(|id| {
            let configuration = store.configuration_for_server(id)?;
            match &*configuration {
                ContextServerConfiguration::Custom { remote, .. }
                | ContextServerConfiguration::Extension { remote, .. }
                    if !is_local && !remote =>
                {
                    None
                }
                _ => Some((id.0.to_string(), configuration)),
            }
        })
        .collect()
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

async fn exited_load_error_after_drain(
    status: ExitStatus,
    drained: impl Future<Output = ()>,
    debug_log: &AcpDebugLog,
    cx: &AsyncApp,
) -> LoadError {
    // Descendants can keep stdout or stderr open after the direct child exits.
    let timeout = cx.background_executor().timer(EXIT_DRAIN_TIMEOUT);
    futures::pin_mut!(drained, timeout);
    if let futures::future::Either::Right(_) = futures::future::select(drained, timeout).await {
        log::warn!("Timed out draining ACP output after agent exit");
    }

    LoadError::Exited {
        status,
        stderr: debug_log.trailing_stderr().map(SharedString::from),
    }
}

#[derive(Debug, Error)]
#[error("Unsupported version")]
pub struct UnsupportedVersion;

#[derive(Debug, Error)]
#[error("ACP v1 fallback failed: {failure}; v2 negotiation: {negotiation:#}")]
struct V1FallbackFailure {
    failure: String,
    negotiation: anyhow::Error,
}

fn fallback_failure(failure: anyhow::Error, negotiation: anyhow::Error) -> anyhow::Error {
    let context = V1FallbackFailure {
        failure: format!("{failure:#}"),
        negotiation,
    };
    // The final attempt's typed error must remain authoritative for UI error classification.
    failure.context(context)
}

pub async fn connect(
    agent_id: AgentId,
    project: Entity<Project>,
    command: AgentServerCommand,
    agent_server_store: WeakEntity<AgentServerStore>,
    default_mode: Option<acp_v2::SessionConfigValueId>,
    default_config_options: HashMap<String, settings::AgentConfigOptionValue>,
    cx: &mut AsyncApp,
) -> Result<Rc<dyn acp_thread::AgentConnection>> {
    let negotiation_error = if cx.update(|cx| cx.has_flag::<AcpBetaFeatureFlag>()) {
        match AcpV2Connection::stdio(
            agent_id.clone(),
            project.clone(),
            command.clone(),
            agent_server_store.clone(),
            cx,
        )
        .await
        {
            Ok(connection) => return Ok(Rc::new(connection)),
            Err(error) if error.is::<UnsupportedVersion>() => Some(error),
            Err(error) => return Err(error),
        }
    } else {
        None
    };
    v1::connect(
        agent_id,
        project,
        command,
        agent_server_store,
        default_mode,
        default_config_options,
        cx,
    )
    .await
    .map_err(|error| match negotiation_error {
        Some(negotiation) => fallback_failure(error, negotiation),
        None => error,
    })
}

fn v1_only_mismatch(error: &agent_client_protocol::Error) -> bool {
    if error.code != agent_client_protocol::Error::new(-32022, "").code {
        return false;
    }
    let Some(data) = error.data.as_ref() else {
        return false;
    };
    let Some(supported) = data.get("supported").and_then(serde_json::Value::as_array) else {
        return false;
    };
    data.get("requested").and_then(serde_json::Value::as_u64) == Some(2)
        && supported
            .iter()
            .all(|value| value.as_u64().is_some_and(|version| version > 0))
        && supported.iter().any(|value| value.as_u64() == Some(1))
        && !supported.iter().any(|value| value.as_u64() == Some(2))
}

fn initialization_error(error: anyhow::Error, debug_log: &AcpDebugLog) -> anyhow::Error {
    // `connect` classifies the agent's answer in-band, but an agent that exits right after
    // answering surfaces as a process exit, so its answer is recovered from the debug log.
    if error.is::<UnsupportedVersion>() || error.is::<agent_client_protocol::Error>() {
        return error;
    }
    match debug_log.initialization_response() {
        Some(Ok(Some(value))) => {
            if serde_json::from_value::<acp::InitializeResponse>(value).is_ok_and(|response| {
                response.protocol_version == agent_client_protocol::schema::ProtocolVersion::V1
            }) {
                error.context(UnsupportedVersion)
            } else {
                error
            }
        }
        Some(Err(protocol_error)) => {
            let retry_v1 = v1_only_mismatch(&protocol_error);
            let error =
                anyhow!(protocol_error).context(format!("ACP initialization failed: {error}"));
            if retry_v1 {
                error.context(UnsupportedVersion)
            } else {
                error
            }
        }
        _ => error,
    }
}

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
    sessions: Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>>,
    pending_session_updates: Option<Rc<RefCell<PendingSessionUpdates>>>,
    session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>>,
    request_elicitations: Entity<ElicitationStore>,
    // Legacy and ingress-only contexts do not track caller ownership of outgoing RPCs.
    active_request_scopes: Option<RefCell<HashSet<acp_v2::RequestId>>>,
    /// A v1 agent may only send `subagent_update` after Zed advertises the `subagents`
    /// client capability. v2 includes subagents in every session.
    accepts_subagent_updates: bool,
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

struct ForegroundBarrier {
    acknowledgment: futures::channel::oneshot::Sender<()>,
}

impl ForegroundWorkItem for ForegroundBarrier {
    fn run(self: Box<Self>, _cx: &mut AsyncApp, _context: &ClientContext) {
        if self.acknowledgment.send(()).is_err() {
            log::debug!("ACP foreground barrier was cancelled before acknowledgment");
        }
    }

    fn reject(self: Box<Self>) {
        log::debug!("ACP foreground dispatch queue closed before acknowledgment");
    }
}

struct ClientDisconnectWork {
    acknowledgment: futures::channel::oneshot::Sender<()>,
}

impl ForegroundWorkItem for ClientDisconnectWork {
    fn run(self: Box<Self>, cx: &mut AsyncApp, context: &ClientContext) {
        if let Some(scopes) = &context.active_request_scopes {
            scopes.borrow_mut().clear();
        }
        if let Some(pending) = &context.pending_session_updates {
            pending.borrow_mut().clear();
        }
        let threads = context
            .sessions
            .borrow()
            .values()
            .map(|session| session.thread.clone())
            .collect::<Vec<_>>();
        for thread in threads {
            if let Some(thread) = thread.upgrade() {
                thread.update(cx, |thread, cx| {
                    thread.cancel_outstanding_elicitations(cx);
                    thread.handle_connection_lost(cx);
                    cx.notify();
                });
            }
        }
        context.request_elicitations.update(cx, |store, cx| {
            store.cancel_all(cx);
        });
        if self.acknowledgment.send(()).is_err() {
            log::debug!("ACP disconnect cleanup waiter was cancelled before acknowledgment");
        }
    }

    fn reject(self: Box<Self>) {
        log::debug!("ACP foreground dispatch queue closed before disconnect cleanup");
    }
}

async fn settle_after_disconnect(
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
) -> Result<(), acp::Error> {
    let (acknowledgment, received) = futures::channel::oneshot::channel();
    let work: ForegroundWork = Box::new(ClientDisconnectWork { acknowledgment });
    if let Err(error) = dispatch_tx.unbounded_send(work) {
        error.into_inner().reject();
        return Err(dispatch_queue_closed_error());
    }
    received.await.map_err(|_| dispatch_queue_closed_error())
}

async fn drain_foreground_queue(dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>) -> Result<()> {
    // The SDK response can precede application of the old session's queued replay.
    let (acknowledgment, received) = futures::channel::oneshot::channel();
    let barrier: ForegroundWork = Box::new(ForegroundBarrier { acknowledgment });
    if let Err(error) = dispatch_tx.unbounded_send(barrier) {
        error.into_inner().reject();
        return Err(anyhow!(dispatch_queue_closed_error()));
    }
    received
        .await
        .context("ACP foreground dispatch queue closed")
}

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
        } = *self;
        handler(notification, cx, ctx);
    }

    fn reject(self: Box<Self>) {
        log::error!("ACP foreground dispatch queue closed while handling inbound notification");
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
    handler: fn(Notif, &mut AsyncApp, &ClientContext),
) where
    Notif: Send + 'static,
{
    let work: ForegroundWork = Box::new(NotificationForegroundWork {
        notification,
        handler,
    });
    if let Err(err) = dispatch_tx.unbounded_send(work) {
        err.into_inner().reject();
    }
}

#[derive(Clone)]
struct ConfigOptions {
    config_options: Rc<RefCell<Vec<acp_v2::SessionConfigOption>>>,
    tx: Rc<RefCell<watch::Sender<()>>>,
    rx: watch::Receiver<()>,
}

impl ConfigOptions {
    fn new(config_options: Rc<RefCell<Vec<acp_v2::SessionConfigOption>>>) -> Self {
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
    default_config_task: Option<Task<()>>,
    _release_subscription: Subscription,
}

/// Builds a v2 terminal-notification handler without selecting it for application connections.
/// Other session updates remain unclaimed so later inbound handlers can process them.
pub fn v2_terminal_client_builder(
    mut dispatch: impl FnMut(DecodedTerminalNotification) -> Result<(), agent_client_protocol::Error>
    + Send
    + 'static,
) -> V2Builder<Client, impl HandleDispatchFrom<Agent>> {
    with_v2_terminal_notifications(Client.v2(), false, move |notification, _| {
        dispatch(notification)
    })
}

struct NotificationByteCounter(usize);

impl std::io::Write for NotificationByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0 = self.0.saturating_add(bytes.len());
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn notification_size(
    notification: &impl serde::Serialize,
) -> Result<usize, agent_client_protocol::Error> {
    let mut counter = NotificationByteCounter(0);
    serde_json::to_writer(&mut counter, notification).map_err(|_| {
        agent_client_protocol::Error::internal_error().data("Failed to size ACP session update")
    })?;
    Ok(counter.0)
}

fn with_v2_terminal_notifications<Handler: HandleDispatchFrom<Agent>>(
    builder: V2Builder<Client, Handler>,
    measure_payload: bool,
    mut dispatch: impl FnMut(
        DecodedTerminalNotification,
        usize,
    ) -> Result<(), agent_client_protocol::Error>
    + Send
    + 'static,
) -> V2Builder<Client, impl HandleDispatchFrom<Agent>> {
    builder.on_receive_notification(
        async move |message: UntypedMessage, connection: V2ConnectionTo<Agent>| {
            let kind = message
                .params()
                .get("update")
                .and_then(|update| update.get("sessionUpdate"))
                .and_then(serde_json::Value::as_str);
            if !acp_v2::UpdateSessionNotification::matches_method(message.method())
                || !matches!(kind, Some("terminal_update" | "terminal_output_chunk"))
            {
                return Ok(Handled::No {
                    message: (message, connection),
                    retry: false,
                });
            }
            let notification = terminal_updates::parse_terminal_notification(&message)?;
            let encoded_bytes = if measure_payload {
                notification_size(&notification)?
            } else {
                0
            };
            let decoded = terminal_updates::decode_terminal_notification(notification)
                .map_err(|error| {
                    agent_client_protocol::Error::invalid_params().data(format!("{error:#}"))
                })?
                .ok_or_else(|| {
                    agent_client_protocol::Error::internal_error()
                        .data("Terminal notification was not decoded")
                })?;
            dispatch(decoded, encoded_bytes)?;
            Ok(Handled::Yes)
        },
        agent_client_protocol::on_receive_notification!(),
    )
}

fn with_v2_session_notifications<Handler: HandleDispatchFrom<Agent>>(
    builder: V2Builder<Client, Handler>,
    mut dispatch: impl FnMut(
        InboundSessionNotification,
        usize,
    ) -> Result<(), agent_client_protocol::Error>
    + Send
    + 'static,
) -> V2Builder<Client, impl HandleDispatchFrom<Agent>> {
    builder.on_receive_notification(
        async move |notification: acp_v2::UpdateSessionNotification,
                    connection: V2ConnectionTo<Agent>| {
            let encoded_bytes = notification_size(&notification)?;
            let notification = match InboundSessionNotification::try_from(notification) {
                Ok(notification) => notification,
                Err(notification) => {
                    return Ok(Handled::No {
                        message: (notification, connection),
                        retry: false,
                    });
                }
            };
            dispatch(notification, encoded_bytes)?;
            Ok(Handled::Yes)
        },
        agent_client_protocol::on_receive_notification!(),
    )
}

fn with_v2_client_ingress<Handler: HandleDispatchFrom<Agent>>(
    builder: V2Builder<Client, Handler>,
    dispatch_sender: mpsc::UnboundedSender<ForegroundWork>,
) -> V2Builder<Client, impl HandleDispatchFrom<Agent>> {
    let dispatch = {
        let dispatch_sender = dispatch_sender.clone();
        move |notification, encoded_bytes| {
            enqueue_notification(
                &dispatch_sender,
                (notification, encoded_bytes),
                route_v2_session_notification,
            );
            Ok(())
        }
    };
    let builder = with_v2_terminal_notifications(builder, true, {
        let dispatch = dispatch.clone();
        move |notification, encoded_bytes| dispatch(notification.into(), encoded_bytes)
    });
    with_v2_session_notifications(builder, dispatch)
        .on_receive_request(
            {
                let dispatch_sender = dispatch_sender.clone();
                async move |request: acp_v2::RequestPermissionRequest, responder, _connection| {
                    enqueue_request(
                        &dispatch_sender,
                        request,
                        responder,
                        handle_v2_request_permission,
                    );
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            {
                let dispatch_sender = dispatch_sender.clone();
                async move |request: acp_v2::CreateElicitationRequest, responder, _connection| {
                    enqueue_request(
                        &dispatch_sender,
                        request,
                        responder,
                        handle_v2_create_elicitation,
                    );
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_notification(
            {
                let dispatch_sender = dispatch_sender.clone();
                async move |notification: acp_v2::CompleteElicitationNotification, _connection| {
                    enqueue_notification(
                        &dispatch_sender,
                        notification,
                        handle_v2_complete_elicitation,
                    );
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_notification!(),
        )
}

fn terminal_auth_task_id(agent_id: &AgentId, method_id: &acp_v2::AuthMethodId) -> String {
    format!("external-agent-{}-{}-login", agent_id.0, method_id.0)
}

fn terminal_auth_task(
    command: &AgentServerCommand,
    agent_id: &AgentId,
    method: &acp_v2::AuthMethodTerminal,
) -> SpawnInTerminal {
    acp_thread::build_terminal_auth_task(
        terminal_auth_task_id(agent_id, &method.method_id),
        method.name.clone(),
        command.path.to_string_lossy().into_owned(),
        command.args.clone(),
        command.env.clone().unwrap_or_default(),
    )
}

fn first_class_terminal_auth_task(
    agent_id: AgentId,
    store: WeakEntity<AgentServerStore>,
    terminal: acp_v2::AuthMethodTerminal,
    cx: &App,
) -> Task<Result<SpawnInTerminal>> {
    cx.spawn(async move |cx| {
        let command = store
            .update(cx, |store, cx| {
                let agent = store
                    .get_external_agent(&agent_id)
                    .context("Agent server not found")?;
                let environment = terminal
                    .env
                    .iter()
                    .map(|variable| (variable.name.clone(), variable.value.clone()))
                    .collect();
                anyhow::Ok(agent.get_command(
                    terminal.args.clone(),
                    environment,
                    &mut cx.to_async(),
                ))
            })?
            .context("Failed to get agent command")?
            .await?;
        Ok(terminal_auth_task(&command, &agent_id, &terminal))
    })
}

fn is_disconnect_error(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<agent_client_protocol::Error>()
        .is_some_and(|error| {
            agent_client_protocol::is_incoming_transport_closed(error)
                || agent_client_protocol::is_connection_terminated(error)
        })
}

fn map_acp_error(err: acp::Error) -> anyhow::Error {
    let error = anyhow!(err);
    match AuthRequired::from_error(&error) {
        Some(auth_required) => error.context(auth_required),
        None => error,
    }
}

fn session_thread(
    ctx: &ClientContext,
    session_id: &acp_v2::SessionId,
) -> Result<WeakEntity<AcpThread>, acp::Error> {
    let sessions = ctx.sessions.borrow();
    sessions
        .get(session_id)
        .map(|session| session.thread.clone())
        .ok_or_else(|| acp::Error::internal_error().data(format!("unknown session: {session_id}")))
}

fn route_v2_session_notification(
    (notification, encoded_bytes): (InboundSessionNotification, usize),
    cx: &mut AsyncApp,
    context: &ClientContext,
) {
    let registered = context
        .sessions
        .borrow()
        .contains_key(&notification.session_id);
    let notification = if !registered && let Some(pending) = &context.pending_session_updates {
        match pending.borrow_mut().stage(notification, encoded_bytes) {
            None => return,
            Some(notification) => notification,
        }
    } else {
        notification
    };
    handle_v2_session_notification(notification, cx, context);
}

fn close_undelivered_session(
    (session_id, connection): (acp_v2::SessionId, V2ConnectionTo<Agent>),
    _cx: &mut AsyncApp,
    context: &ClientContext,
) {
    if let Some(pending) = &context.pending_session_updates {
        pending.borrow_mut().discard_session(&session_id);
    }
    if context.sessions.borrow().contains_key(&session_id) {
        return;
    }
    connection
        .prepare_request(acp_v2::CloseSessionRequest::new(session_id))
        .on_receiving_result(async move |result| {
            result.log_err();
            Ok(())
        })
        .log_err();
}

/// Applies a `subagent_update` received on `parent`'s stream. The first update for a
/// session announces it: Zed creates its thread, routes the session's traffic on this
/// connection, and hands the thread to the parent. Later updates patch the association.
/// The update's `state` is left to the caller, since only v1 uses it as the subagent's
/// activity.
fn apply_subagent_update(
    parent: &Entity<AcpThread>,
    update: &acp_v2::SubagentUpdate,
    sessions: &Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>>,
    cx: &mut App,
) -> Result<Entity<AcpThread>> {
    let parent_session_id = parent.read(cx).session_id().clone();
    let session_id = update.session_id.clone();
    if let Some(subagent) = parent.read(cx).subagent(&session_id).cloned() {
        subagent.update(cx, |subagent, cx| {
            subagent.update_subagent_association(update, cx)
        })?;
        return Ok(subagent);
    }
    // Announcing any other session this connection routes, including the parent itself,
    // would reparent it, so this also rules out cycles.
    anyhow::ensure!(
        !sessions.borrow().contains_key(&session_id),
        "Session {session_id} cannot become a subagent of {parent_session_id}"
    );

    let (connection, project, parent_action_log) = {
        let parent = parent.read(cx);
        (
            parent.connection().clone(),
            parent.project().clone(),
            parent.action_log().clone(),
        )
    };
    let action_log =
        cx.new(|_| ActionLog::new(project.clone()).with_linked_action_log(parent_action_log));
    let subagent = cx.new(|cx| {
        AcpThread::new(
            Some(parent_session_id),
            None,
            None,
            connection,
            project,
            action_log,
            session_id.clone(),
            watch::Receiver::constant(acp_v2::PromptCapabilities::default()),
            cx,
        )
        .agent_owned()
    });
    subagent.update(cx, |subagent, cx| {
        subagent.update_subagent_association(update, cx)
    })?;
    // Unlike a session Zed opened, releasing a subagent must not close it: the agent
    // releases subagents when their parent closes.
    let release_subscription = cx.observe_release(&subagent, {
        let sessions = Rc::downgrade(sessions);
        let session_id = session_id.clone();
        let thread_id = subagent.entity_id();
        move |_, _| {
            let Some(sessions) = sessions.upgrade() else {
                return;
            };
            let mut sessions = sessions.borrow_mut();
            if sessions
                .get(&session_id)
                .is_some_and(|session| session.thread.entity_id() == thread_id)
            {
                sessions.remove(&session_id);
            }
        }
    });
    sessions.borrow_mut().insert(
        session_id,
        AcpSession {
            thread: subagent.downgrade(),
            suppress_abort_err: false,
            session_modes: None,
            config_options: None,
            default_config_task: None,
            _release_subscription: release_subscription,
        },
    );
    parent.update(cx, |parent, cx| {
        parent.register_subagent(subagent.clone(), cx)
    });
    Ok(subagent)
}

fn apply_v2_session_notification(
    notification: InboundSessionNotification,
    thread: &WeakEntity<AcpThread>,
    sessions: &Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>>,
    cx: &mut AsyncApp,
) -> Result<()> {
    if let InboundSessionUpdate::Subagent(update) = &notification.update {
        let parent = thread
            .upgrade()
            .context("Session was released before its subagent was announced")?;
        let parent_session_id = parent.read_with(cx, |parent, _| parent.session_id().clone());
        anyhow::ensure!(
            parent_session_id == notification.session_id,
            "Subagent update belongs to a different ACP session"
        );
        // In v2 the subagent's own `state_update` reports its activity; the parent's copy
        // only mirrors it.
        cx.update(|cx| apply_subagent_update(&parent, update, sessions, cx))?;
        return Ok(());
    }
    if let InboundSessionUpdate::ConfigOptions(update) = &notification.update {
        let config_options = {
            let mut sessions = sessions.borrow_mut();
            let session = sessions
                .get_mut(&notification.session_id)
                .filter(|session| session.thread.entity_id() == thread.entity_id())
                .context("Session was released or replaced before applying configuration")?;
            session
                .config_options
                .get_or_insert_with(|| ConfigOptions::new(Rc::new(RefCell::new(Vec::new()))))
                .clone()
        };
        *config_options.config_options.borrow_mut() = update.config_options.clone();
        config_options.tx.borrow_mut().send(()).log_err();
    }
    thread.update(cx, |thread, cx| notification.apply(thread, cx))?
}

fn handle_v2_session_notification(
    notification: InboundSessionNotification,
    cx: &mut AsyncApp,
    context: &ClientContext,
) {
    let thread = match session_thread(context, &notification.session_id) {
        Ok(thread) => thread,
        Err(error) => {
            log::warn!("Received ACP v2 update for an unknown session: {error}");
            return;
        }
    };
    let Some(thread) = thread.upgrade() else {
        log::warn!("Received ACP v2 update for a released session");
        return;
    };
    if let InboundSessionUpdate::SessionInfo(update) = &notification.update
        && let Some(session_list) = context.session_list.borrow().as_ref()
    {
        session_list.send_info_update(notification.session_id.clone(), update.clone());
    }
    apply_v2_session_notification(notification, &thread.downgrade(), &context.sessions, cx)
        .log_err();
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

fn handle_v2_request_permission(
    request: acp_v2::RequestPermissionRequest,
    responder: Responder<acp_v2::RequestPermissionResponse>,
    cx: &mut AsyncApp,
    context: &ClientContext,
) {
    let cancellation = responder.cancellation();
    if cancellation.is_cancelled() {
        return respond_err(responder, acp::Error::request_cancelled());
    }
    let thread = match session_thread(context, &request.session_id) {
        Ok(thread) => thread,
        Err(error) => return respond_err(responder, error),
    };
    let (request_id, task) = match thread
        .update(cx, |thread, cx| {
            thread
                .request_permission(request, cx)
                .map_err(|error| acp::Error::invalid_params().data(error.to_string()))
        })
        .flatten_acp()
    {
        Ok(request) => request,
        Err(error) => return respond_err(responder, error),
    };
    cx.spawn(async move |cx| {
        match cancellation
            .run_until_cancelled(async { Ok(task.await) })
            .await
        {
            Ok(outcome) => {
                responder
                    .respond(acp_v2::RequestPermissionResponse::new(outcome))
                    .log_err();
            }
            Err(error) => {
                if error.code == ErrorCode::RequestCancelled {
                    thread
                        .update(cx, |thread, cx| {
                            thread.cancel_permission_request(request_id, cx)
                        })
                        .log_err();
                }
                respond_err(responder, error);
            }
        }
    })
    .detach();
}

fn handle_v2_create_elicitation(
    args: acp_v2::CreateElicitationRequest,
    responder: Responder<acp_v2::CreateElicitationResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let cancellation = responder.cancellation();
    if cancellation.is_cancelled() {
        return respond_err(responder, acp::Error::request_cancelled());
    }
    match args.scope() {
        acp_v2::ElicitationScope::Session(scope) => {
            let thread = match session_thread(ctx, &scope.session_id) {
                Ok(t) => t,
                Err(e) => return respond_err(responder, e),
            };

            let (elicitation_id, task) = match thread
                .update(cx, |thread, cx| {
                    thread
                        .request_elicitation_with_id(args, cx)
                        .map_err(acp_thread::elicitation::error_to_v1)
                })
                .flatten_acp()
            {
                Ok(task) => task,
                Err(e) => return respond_err(responder, e),
            };

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
        acp_v2::ElicitationScope::Request(scope) => {
            if let Some(scopes) = &ctx.active_request_scopes
                && !scopes.borrow().contains(&scope.request_id)
            {
                return respond_err(
                    responder,
                    acp::Error::invalid_params()
                        .data("Elicitation parent request is not owned by an active caller"),
                );
            }
            let store = ctx.request_elicitations.clone();
            let (elicitation_id, task) = match store.update(cx, |store, cx| {
                store
                    .request_elicitation_with_id(args, cx)
                    .map_err(acp_thread::elicitation::error_to_v1)
            }) {
                Ok(task) => task,
                Err(e) => return respond_err(responder, e),
            };
            let store = store.downgrade();

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

fn handle_v2_complete_elicitation(
    args: acp_v2::CompleteElicitationNotification,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let threads = ctx
        .sessions
        .borrow()
        .values()
        .map(|session| session.thread.clone())
        .collect::<Vec<_>>();
    let elicitation_id = args.elicitation_id;

    // Complete accepted notifications in queue order before disconnect cleanup can run.
    for thread in threads {
        if let Some(thread) = thread.upgrade() {
            thread.update(cx, |thread, cx| {
                thread.complete_url_elicitation(&elicitation_id, cx);
            });
        }
    }
    ctx.request_elicitations.update(cx, |store, cx| {
        store.complete_url_elicitation(&elicitation_id, cx);
    })
}

#[cfg(all(test, unix))]
mod selection_tests {
    use super::*;
    use gpui::{TestAppContext, UpdateGlobal as _};
    use serde_json::json;

    fn set_beta(enabled: bool, cx: &mut App) {
        cx.update_flags(
            false,
            if enabled {
                vec!["acp-beta".into()]
            } else {
                vec![]
            },
        );
        settings::SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .feature_flags
                    .get_or_insert_default()
                    .insert("acp-beta".into(), if enabled { "on" } else { "off" }.into());
            });
        });
        assert_eq!(cx.has_flag::<AcpBetaFeatureFlag>(), enabled);
    }

    #[gpui::test]
    async fn beta_selection_uses_fresh_v1_fallback_only_for_version_mismatch(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let mut store = settings::SettingsStore::test(cx);
            store.register_setting::<feature_flags::FeatureFlagsSettings>();
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let v1 = r#""result":{"protocolVersion":1,"agentCapabilities":{},"agentInfo":{"name":"selected-v1","version":"1"}}"#;
        let v2 = r#""result":{"protocolVersion":2,"info":{"name":"selected-v2","version":"1"},"capabilities":{"session":{}}}"#;
        for (beta, response, exit_after_response, expected, requests) in [
            (false, v2, false, Some(false), "1"),
            (true, v2, false, Some(true), "2"),
            (true, v1, false, Some(false), "2 1"),
            (
                true,
                r#""error":{"code":-32022,"message":"Unsupported protocol version","data":{"requested":2,"supported":[1]}}"#,
                false,
                Some(false),
                "2 1",
            ),
            (
                true,
                r#""error":{"code":-32022,"message":"Unsupported protocol version","data":{"requested":2,"supported":[1]}}"#,
                true,
                Some(false),
                "2 1",
            ),
            (
                true,
                r#""error":{"code":-32022,"message":"Unsupported protocol version","data":{"requested":2,"supported":[3]}}"#,
                true,
                None,
                "2",
            ),
            (
                true,
                r#""error":{"code":-32022,"message":"Unsupported protocol version","data":{"requested":2,"supported":[1,2]}}"#,
                false,
                None,
                "2",
            ),
            (
                true,
                r#""error":{"code":-32602,"message":"Bad initialize parameters"}"#,
                false,
                None,
                "2",
            ),
            (
                true,
                r#""result":{"protocolVersion":1},"error":{"code":-32022,"message":"Invalid envelope","data":{"requested":2,"supported":[1]}}"#,
                true,
                None,
                "2",
            ),
            (
                true,
                r#""result":{"info":{"name":"malformed","version":"1"}}"#,
                false,
                None,
                "2",
            ),
            (
                true,
                r#""result":{"protocolVersion":"malformed","agentCapabilities":{}}"#,
                false,
                None,
                "2",
            ),
            (true, "", true, None, "2"),
        ] {
            cx.update(|cx| {
                set_beta(beta, cx);
            });
            let trace = directory.path().join("connections");
            std::fs::write(&trace, "").expect("clear trace");
            let command = AgentServerCommand {
                path: "/bin/sh".into(),
                args: vec!["-c".into(), r#"
printf 'start %s\n' "$$" >> "$ACP_SELECTION_TRACE"
while IFS= read -r request; do
    case "$request" in
        *'"method":"initialize"'*)
            id=${request#*\"id\":}; id=${id%%,*}
            case "$request" in
                *'"protocolVersion":2'*)
                    printf 'initialize 2\n' >> "$ACP_SELECTION_TRACE"
                    if [ -n "$ACP_SELECTION_RESPONSE" ]; then
                        printf '{"jsonrpc":"2.0","id":%s,%s}\n' "$id" "$ACP_SELECTION_RESPONSE"
                    fi
                    if [ "$ACP_SELECTION_EXIT" = 1 ]; then exit 7; fi
                    ;;
                *)
                    printf 'initialize 1\n' >> "$ACP_SELECTION_TRACE"
                    printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":1,"agentCapabilities":{},"agentInfo":{"name":"selected-v1","version":"1"}}}\n' "$id"
                    ;;
            esac
            ;;
    esac
done
"#.into()],
                env: Some(HashMap::from_iter([
                    ("ACP_SELECTION_TRACE".into(), trace.to_string_lossy().into_owned()),
                    ("ACP_SELECTION_RESPONSE".into(), response.into()),
                    ("ACP_SELECTION_EXIT".into(), if exit_after_response { "1" } else { "0" }.into()),
                ])),
            };
            let connecting = cx.spawn({
                let project = project.clone();
                async move |mut cx| {
                    connect(
                        AgentId::new("selection"),
                        project,
                        command,
                        WeakEntity::new_invalid(),
                        None,
                        HashMap::default(),
                        &mut cx,
                    )
                    .await
                }
            });
            let timeout = cx
                .background_executor
                .timer(std::time::Duration::from_secs(10));
            futures::pin_mut!(connecting, timeout);
            let result = match futures::future::select(connecting, timeout).await {
                futures::future::Either::Left((result, _)) => result,
                _ => panic!("selection timed out: {response}"),
            };
            if let Some(v2) = expected {
                let connection = result.expect("selected connection");
                assert_eq!(
                    connection.clone().downcast::<AcpV2Connection>().is_some(),
                    v2
                );
                cx.update(|cx| set_beta(false, cx));
                assert_eq!(
                    connection.clone().downcast::<AcpV2Connection>().is_some(),
                    v2
                );
                connection.shutdown();
                drop(connection);
            } else {
                let error = result
                    .err()
                    .unwrap_or_else(|| panic!("unexpected successful selection for {response}"));
                if response.starts_with(r#""error":{"code":-32022"#) {
                    let protocol_error = error
                        .downcast_ref::<agent_client_protocol::Error>()
                        .expect("structured mismatch retained");
                    assert_eq!(
                        protocol_error.code,
                        agent_client_protocol::Error::new(-32022, "").code
                    );
                    assert!(protocol_error.data.is_some());
                }
            }
            let trace = std::fs::read_to_string(&trace).expect("connection trace");
            let actual = trace
                .lines()
                .filter_map(|line| line.strip_prefix("initialize "))
                .collect::<Vec<_>>()
                .join(" ");
            assert_eq!(actual, requests, "{response}");
            let processes = trace
                .lines()
                .filter_map(|line| line.strip_prefix("start "))
                .collect::<Vec<_>>();
            assert_eq!(processes.len(), requests.split_whitespace().count());
            if let [first, second] = processes.as_slice() {
                assert_ne!(first, second, "fallback must launch a fresh process");
            }
        }
    }

    #[test]
    fn version_mismatch_requires_valid_v1_only_support() {
        for data in [
            json!({"requested": 1, "supported": [1]}),
            json!({"requested": 2, "supported": [1, 2]}),
            json!({"requested": 2, "supported": [0, 1]}),
            json!({"requested": 2, "supported": [1, "2"]}),
            json!({"requested": 2}),
        ] {
            assert!(!v1_only_mismatch(
                &agent_client_protocol::Error::new(-32022, "").data(data)
            ));
        }
        assert!(!v1_only_mismatch(
            &agent_client_protocol::Error::invalid_params()
                .data(json!({"requested": 2, "supported": [1]}))
        ));
    }

    #[test]
    fn fallback_failure_keeps_the_final_error_authoritative() {
        let negotiation_data = json!({"requested": 2, "supported": [1]});
        let final_data = json!({"attempt": "v1"});
        let error = fallback_failure(
            anyhow!(agent_client_protocol::Error::invalid_params().data(final_data.clone())),
            anyhow!(
                agent_client_protocol::Error::new(-32022, "Unsupported")
                    .data(negotiation_data.clone())
            )
            .context(UnsupportedVersion),
        );
        let info = acp_thread::ProtocolErrorInfo::from_error(&error).expect("final protocol error");
        assert_eq!(info.code, -32602);
        assert_eq!(info.data, Some(&final_data));
        assert!(!error.is::<UnsupportedVersion>());
        let context = error
            .downcast_ref::<V1FallbackFailure>()
            .expect("negotiation context");
        assert_eq!(
            context
                .negotiation
                .downcast_ref::<agent_client_protocol::Error>()
                .expect("original negotiation")
                .data,
            Some(negotiation_data)
        );
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "e2e"), ignore)]
    async fn claude_adapter_initialize_smoke(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let mut store = settings::SettingsStore::test(cx);
            store.register_setting::<feature_flags::FeatureFlagsSettings>();
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("isolated working directory");
        let config = directory.path().join("claude-config");
        std::fs::create_dir(&config).expect("isolated Claude configuration");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        for (beta, adapter_v2, expected_v2) in [
            (false, true, false),
            (true, true, true),
            (true, false, false),
        ] {
            cx.update(|cx| set_beta(beta, cx));
            let command = AgentServerCommand {
                path: "npx".into(),
                args: vec![
                    "--yes".into(),
                    "@agentclientprotocol/claude-agent-acp@0.94.0".into(),
                ],
                env: Some(HashMap::from_iter([
                    (
                        "CLAUDE_CONFIG_DIR".into(),
                        config.to_string_lossy().into_owned(),
                    ),
                    (
                        "CLAUDE_AGENT_ACP_EXPERIMENTAL_V2".into(),
                        if adapter_v2 { "1" } else { "0" }.into(),
                    ),
                    ("ANTHROPIC_API_KEY".into(), "".into()),
                    ("CLAUDE_CODE_OAUTH_TOKEN".into(), "".into()),
                ])),
            };
            let connecting = cx.spawn({
                let project = project.clone();
                async move |mut cx| {
                    connect(
                        AgentId::new("claude-acp"),
                        project,
                        command,
                        WeakEntity::new_invalid(),
                        None,
                        HashMap::default(),
                        &mut cx,
                    )
                    .await
                }
            });
            let timeout = cx
                .background_executor
                .timer(std::time::Duration::from_secs(30));
            futures::pin_mut!(connecting, timeout);
            let connection = match futures::future::select(connecting, timeout).await {
                futures::future::Either::Left((result, _)) => {
                    result.expect("Claude adapter handshake")
                }
                _ => panic!("Claude adapter handshake timed out"),
            };
            let actual_v2 = connection.clone().downcast::<AcpV2Connection>().is_some();
            assert_eq!(
                actual_v2, expected_v2,
                "beta={beta} adapter_v2={adapter_v2}"
            );
            println!(
                "Claude ACP 0.94.0: beta={beta}, adapter_v2={adapter_v2}, selected_v2={actual_v2}"
            );
            connection.shutdown();
            drop(connection);
            cx.run_until_parked();
        }
    }
}
