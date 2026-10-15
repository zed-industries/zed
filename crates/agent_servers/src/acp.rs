mod debug_log;
mod runtime;
mod session_updates;
mod terminal_updates;
mod transport;
mod v1;
mod v2;

use debug_log::AcpDebugLog;
pub use debug_log::{AcpDebugMessage, AcpDebugMessageContent, AcpDebugMessageDirection};
use session_updates::{InboundSessionNotification, InboundSessionUpdate};
pub use terminal_updates::{DecodedTerminalNotification, DecodedTerminalUpdate};
#[cfg(any(test, feature = "test-support"))]
pub use v1::test_support;
pub use v1::{AcpConnection, AcpSessionList, GEMINI_TERMINAL_AUTH_METHOD_ID, connect};
pub use v2::AcpV2Connection;

use acp_thread::{AcpThread, AuthRequired, ElicitationStore, LoadError};
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
use futures::{Future, channel::mpsc};
use gpui::{App, AsyncApp, Entity, SharedString, Subscription, Task, WeakEntity};
use project::AgentId;
use project::agent_server_store::{AgentServerCommand, AgentServerStore};
use std::{cell::RefCell, process::ExitStatus, rc::Rc};
use task::SpawnInTerminal;
use thiserror::Error;
use util::ResultExt as _;

const EXIT_DRAIN_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(250);

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
    session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>>,
    request_elicitations: Entity<ElicitationStore>,
    // Legacy and ingress-only contexts do not track caller ownership of outgoing RPCs.
    active_request_scopes: Option<RefCell<HashSet<acp_v2::RequestId>>>,
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

struct ElicitationDisconnectWork {
    acknowledgment: futures::channel::oneshot::Sender<()>,
}

impl ForegroundWorkItem for ElicitationDisconnectWork {
    fn run(self: Box<Self>, cx: &mut AsyncApp, context: &ClientContext) {
        if let Some(scopes) = &context.active_request_scopes {
            scopes.borrow_mut().clear();
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

async fn cancel_elicitations_after_disconnect(
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
) -> Result<(), acp::Error> {
    let (acknowledgment, received) = futures::channel::oneshot::channel();
    let work: ForegroundWork = Box::new(ElicitationDisconnectWork { acknowledgment });
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
    dispatch: impl FnMut(DecodedTerminalNotification) -> Result<(), agent_client_protocol::Error>
    + Send
    + 'static,
) -> V2Builder<Client, impl HandleDispatchFrom<Agent>> {
    with_v2_terminal_notifications(Client.v2(), dispatch)
}

fn with_v2_terminal_notifications<Handler: HandleDispatchFrom<Agent>>(
    builder: V2Builder<Client, Handler>,
    mut dispatch: impl FnMut(DecodedTerminalNotification) -> Result<(), agent_client_protocol::Error>
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
            let decoded = terminal_updates::decode_terminal_notification(notification)
                .map_err(|error| {
                    agent_client_protocol::Error::invalid_params().data(format!("{error:#}"))
                })?
                .ok_or_else(|| {
                    agent_client_protocol::Error::internal_error()
                        .data("Terminal notification was not decoded")
                })?;
            dispatch(decoded)?;
            Ok(Handled::Yes)
        },
        agent_client_protocol::on_receive_notification!(),
    )
}

fn with_v2_session_notifications<Handler: HandleDispatchFrom<Agent>>(
    builder: V2Builder<Client, Handler>,
    mut dispatch: impl FnMut(InboundSessionNotification) -> Result<(), agent_client_protocol::Error>
    + Send
    + 'static,
) -> V2Builder<Client, impl HandleDispatchFrom<Agent>> {
    builder.on_receive_notification(
        async move |notification: acp_v2::UpdateSessionNotification,
                    connection: V2ConnectionTo<Agent>| {
            let notification = match InboundSessionNotification::try_from(notification) {
                Ok(notification) => notification,
                Err(notification) => {
                    return Ok(Handled::No {
                        message: (notification, connection),
                        retry: false,
                    });
                }
            };
            dispatch(notification)?;
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
        move |notification| {
            enqueue_notification(
                &dispatch_sender,
                notification,
                handle_v2_session_notification,
            );
            Ok(())
        }
    };
    let builder = with_v2_terminal_notifications(builder, {
        let dispatch = dispatch.clone();
        move |notification| dispatch(notification.into())
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
    if let InboundSessionUpdate::ConfigOptions(update) = &notification.update {
        let config_options = {
            let mut sessions = context.sessions.borrow_mut();
            let Some(session) = sessions.get_mut(&notification.session_id) else {
                return;
            };
            session
                .config_options
                .get_or_insert_with(|| ConfigOptions::new(Rc::new(RefCell::new(Vec::new()))))
                .clone()
        };
        *config_options.config_options.borrow_mut() = update.config_options.clone();
        config_options.tx.borrow_mut().send(()).log_err();
    }
    thread
        .update(cx, |thread, cx| notification.apply(thread, cx))
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
