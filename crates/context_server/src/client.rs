use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use futures::{FutureExt, StreamExt, channel::oneshot, future, select, stream::FuturesUnordered};
use futures_lite::future::yield_now;
use gpui::{AppContext as _, AsyncApp, BackgroundExecutor, Task};
use parking_lot::Mutex;
use postage::{barrier, prelude::Stream as _};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, value::RawValue};
use slotmap::SlotMap;
use std::{
    fmt,
    path::PathBuf,
    pin::pin,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use util::{ResultExt, TryFutureExt};

use crate::{
    transport::{StdioTransport, Transport, TransportShutdownReason},
    types::{
        CancelledParams, ClientNotification, Notification as _,
        notifications::{Cancelled, Initialized},
    },
};

const JSON_RPC_VERSION: &str = "2.0";
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

// Standard JSON-RPC error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

type ResponseHandler = Box<dyn Send + FnOnce(Result<String, Arc<str>>)>;
type NotificationHandler = Box<dyn Send + FnMut(Value, AsyncApp)>;
type RequestHandler = Box<dyn Send + FnMut(RequestId, &RawValue, AsyncApp)>;

struct OutboundMessage {
    message: String,
    started_tx: Option<oneshot::Sender<()>>,
    issued_tx: Option<oneshot::Sender<()>>,
    started: Option<Arc<AtomicBool>>,
    cancel_rx: Option<oneshot::Receiver<()>>,
}

impl OutboundMessage {
    fn notification(message: String) -> Self {
        Self {
            message,
            started_tx: None,
            issued_tx: None,
            started: None,
            cancel_rx: None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Int(i32),
    Str(String),
}

pub(crate) struct Client {
    server_id: ContextServerId,
    next_id: AtomicI32,
    outbound_tx: async_channel::Sender<OutboundMessage>,
    priority_outbound_tx: async_channel::Sender<OutboundMessage>,
    name: Arc<str>,
    subscription_set: Arc<Mutex<NotificationSubscriptionSet>>,
    response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    io_tasks: Mutex<Option<[Task<Option<()>>; 3]>>,
    shutdown_rx: Mutex<Option<(barrier::Receiver, barrier::Receiver)>>,
    executor: BackgroundExecutor,
    transport: Mutex<Option<Arc<dyn Transport>>>,
    request_timeout: Option<Duration>,
    /// Single-slot side channel for the last transport-level error. When the
    /// output task encounters a send failure it stashes the error here and
    /// exits; the next request to observe cancellation `.take()`s it so it
    /// can fail with the underlying cause (e.g. "connection refused") instead
    /// of a generic "cancelled". This is best-effort diagnostics: with
    /// concurrent requests in flight, a single arbitrary one receives the
    /// stashed error. Nothing may depend on it for correctness —
    /// authentication challenges are observed via [`Self::wait_for_shutdown`]
    /// and [`Transport::auth_challenge`], which do not involve requests.
    last_transport_error: Arc<Mutex<Option<anyhow::Error>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct ContextServerId(pub Arc<str>);

fn is_null_value<T: Serialize>(value: &T) -> bool {
    matches!(serde_json::to_value(value), Ok(Value::Null))
}

#[derive(Serialize, Deserialize)]
pub struct Request<'a, T> {
    pub jsonrpc: &'static str,
    pub id: RequestId,
    pub method: &'a str,
    #[serde(skip_serializing_if = "is_null_value")]
    pub params: T,
}

#[derive(Serialize, Deserialize)]
pub struct AnyRequest<'a> {
    pub jsonrpc: &'a str,
    pub id: RequestId,
    pub method: &'a str,
    #[serde(skip_serializing_if = "is_null_value")]
    pub params: Option<&'a RawValue>,
}

#[derive(Serialize, Deserialize)]
struct AnyResponse<'a> {
    jsonrpc: &'a str,
    id: RequestId,
    #[serde(default)]
    error: Option<Error>,
    #[serde(borrow)]
    result: Option<&'a RawValue>,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Response<T> {
    pub jsonrpc: &'static str,
    pub id: RequestId,
    #[serde(flatten)]
    pub value: CspResult<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CspResult<T> {
    #[serde(rename = "result")]
    Ok(Option<T>),
    #[allow(dead_code)]
    Error(Option<Error>),
}

#[derive(Serialize, Deserialize)]
struct Notification<'a, T> {
    jsonrpc: &'static str,
    #[serde(borrow)]
    method: &'a str,
    #[serde(skip_serializing_if = "is_null_value")]
    params: T,
}

#[derive(Debug, Clone, Deserialize)]
struct AnyNotification<'a> {
    #[expect(
        unused,
        reason = "Part of the JSON-RPC protocol - we expect the field to be present in a valid JSON-RPC notification"
    )]
    jsonrpc: &'a str,
    method: String,
    #[serde(default)]
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Error {
    pub message: String,
    pub code: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelContextServerBinary {
    pub executable: PathBuf,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    pub timeout: Option<u64>,
}

impl Client {
    /// Creates a new Client instance for a context server.
    ///
    /// This function initializes a new Client by spawning a child process for the context server,
    /// setting up communication channels, and initializing handlers for input/output operations.
    /// It takes a server ID, binary information, and an async app context as input.
    pub fn stdio(
        server_id: ContextServerId,
        binary: ModelContextServerBinary,
        working_directory: &Option<PathBuf>,
        cx: AsyncApp,
    ) -> Result<Self> {
        log::debug!(
            "starting context server (executable={:?}, argument_count={})",
            binary.executable,
            binary.args.len()
        );

        let server_name = binary
            .executable
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(String::new);

        let timeout = binary.timeout.map(Duration::from_secs);
        let transport = Arc::new(StdioTransport::new(binary, working_directory, &cx)?);
        Self::new(server_id, server_name.into(), transport, timeout, cx)
    }

    /// Creates a new Client instance for a context server.
    pub fn new(
        server_id: ContextServerId,
        server_name: Arc<str>,
        transport: Arc<dyn Transport>,
        request_timeout: Option<Duration>,
        cx: AsyncApp,
    ) -> Result<Self> {
        let (outbound_tx, outbound_rx) = async_channel::unbounded::<OutboundMessage>();
        let (priority_outbound_tx, priority_outbound_rx) =
            async_channel::unbounded::<OutboundMessage>();
        let (output_done_tx, output_done_rx) = barrier::channel();
        let (input_done_tx, input_done_rx) = barrier::channel();

        let subscription_set = Arc::new(Mutex::new(NotificationSubscriptionSet::default()));
        let response_handlers =
            Arc::new(Mutex::new(Some(HashMap::<_, ResponseHandler>::default())));
        let request_handlers = Arc::new(Mutex::new(HashMap::<_, RequestHandler>::default()));

        let receive_input_task = cx.spawn({
            let subscription_set = subscription_set.clone();
            let response_handlers = response_handlers.clone();
            let request_handlers = request_handlers.clone();
            let transport = transport.clone();
            async move |cx| {
                Self::handle_input(
                    transport,
                    subscription_set,
                    request_handlers,
                    response_handlers,
                    input_done_tx,
                    cx,
                )
                .log_err()
                .await
            }
        });
        let receive_err_task = cx.spawn({
            let transport = transport.clone();
            async move |_| Self::handle_err(transport).log_err().await
        });

        let last_transport_error: Arc<Mutex<Option<anyhow::Error>>> = Arc::new(Mutex::new(None));
        let effective_request_timeout = request_timeout.unwrap_or(DEFAULT_REQUEST_TIMEOUT);
        // Notifications share the serialized transport with requests, so leave
        // queued requests part of their own deadline after a notification stalls.
        let notification_timeout = effective_request_timeout / 2;
        let output_task = cx.background_spawn({
            let transport = transport.clone();
            let last_transport_error = last_transport_error.clone();
            let executor = cx.background_executor().clone();
            Self::handle_output(
                transport,
                outbound_rx,
                priority_outbound_rx,
                output_done_tx,
                response_handlers.clone(),
                last_transport_error,
                executor,
                notification_timeout,
            )
            .log_err()
        });

        Ok(Self {
            server_id,
            subscription_set,
            response_handlers,
            name: server_name,
            next_id: Default::default(),
            outbound_tx,
            priority_outbound_tx,
            executor: cx.background_executor().clone(),
            io_tasks: Mutex::new(Some([receive_input_task, receive_err_task, output_task])),
            shutdown_rx: Mutex::new(Some((input_done_rx, output_done_rx))),
            transport: Mutex::new(Some(transport)),
            request_timeout,
            last_transport_error,
        })
    }

    pub(crate) fn stop(&self) {
        let handlers = self.response_handlers.lock().take();
        self.outbound_tx.close();
        self.priority_outbound_tx.close();
        // Cancel the readers directly, even if callers retain this client. Deferring their
        // cancellation through another task lets them consume a replacement client's responses.
        drop(self.io_tasks.lock().take());
        // Releasing a stdio transport also terminates its process.
        drop(self.transport.lock().take());

        if let Some(handlers) = handlers {
            let error: Arc<str> = "Context server stopped".into();
            for handler in handlers.into_values() {
                handler(Err(error.clone()));
            }
        }
    }

    /// Handles input from the server's stdout.
    ///
    /// This function continuously reads lines from the provided stdout stream,
    /// parses them as JSON-RPC responses or notifications, and dispatches them
    /// to the appropriate handlers. It processes both responses (which are matched
    /// to pending requests) and notifications (which trigger registered handlers).
    async fn handle_input(
        transport: Arc<dyn Transport>,
        subscription_set: Arc<Mutex<NotificationSubscriptionSet>>,
        request_handlers: Arc<Mutex<HashMap<&'static str, RequestHandler>>>,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        _input_done_tx: barrier::Sender,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        let _fail_pending_requests = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                if let Some(handlers) = response_handlers.lock().take() {
                    let error: Arc<str> = "Context server disconnected".into();
                    for handler in handlers.into_values() {
                        handler(Err(error.clone()));
                    }
                }
            }
        });
        let mut receiver = transport.receive();

        while let Some(message) = receiver.next().await {
            log::trace!("recv: {message}");
            if let Ok(request) = serde_json::from_str::<AnyRequest>(&message) {
                let mut request_handlers = request_handlers.lock();
                if let Some(handler) = request_handlers.get_mut(request.method) {
                    handler(
                        request.id,
                        request.params.unwrap_or(RawValue::NULL),
                        cx.clone(),
                    );
                }
            } else if let Ok(response) = serde_json::from_str::<AnyResponse>(&message) {
                if let Some(handlers) = response_handlers.lock().as_mut()
                    && let Some(handler) = handlers.remove(&response.id)
                {
                    handler(Ok(message.to_string()));
                }
            } else if let Ok(notification) = serde_json::from_str::<AnyNotification>(&message) {
                subscription_set.lock().notify(
                    &notification.method,
                    notification.params.unwrap_or(Value::Null),
                    cx,
                )
            } else {
                log::error!("Unhandled JSON from context_server: {}", message);
            }
        }

        yield_now().await;

        Ok(())
    }

    /// Handles the stderr output from the context server.
    /// Continuously reads and logs any error messages from the server.
    async fn handle_err(transport: Arc<dyn Transport>) -> anyhow::Result<()> {
        while let Some(err) = transport.receive_err().next().await {
            log::debug!("context server stderr: {}", err.trim());
        }

        Ok(())
    }

    /// Handles the output to the context server's stdin.
    /// This function continuously receives messages from the outbound channel,
    /// writes them to the server's stdin, and manages the lifecycle of response handlers.
    async fn handle_output(
        transport: Arc<dyn Transport>,
        outbound_rx: async_channel::Receiver<OutboundMessage>,
        priority_outbound_rx: async_channel::Receiver<OutboundMessage>,
        output_done_tx: barrier::Sender,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        last_transport_error: Arc<Mutex<Option<anyhow::Error>>>,
        executor: BackgroundExecutor,
        notification_timeout: Duration,
    ) -> anyhow::Result<()> {
        let _clear_response_handlers = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                response_handlers.lock().take();
            }
        });
        let concurrent = transport.supports_concurrent_sends();
        let mut pending_sends: FuturesUnordered<
            future::BoxFuture<'static, Option<(Option<RequestId>, Result<()>)>>,
        > = FuturesUnordered::new();
        loop {
            let mut priority_recv = pin!(priority_outbound_rx.recv().fuse());
            let mut outbound_recv = pin!(outbound_rx.recv().fuse());
            let completed_send = if pending_sends.is_empty() {
                future::Either::Left(future::pending())
            } else {
                future::Either::Right(pending_sends.next())
            };
            let mut completed_send = pin!(completed_send.fuse());
            let next_outbound = futures::select_biased! {
                completed = completed_send => {
                    if let Some(Some((request_id, Err(err)))) = completed {
                        Self::fail_output_send(err, request_id, &response_handlers, &last_transport_error);
                        return Ok(());
                    }
                    continue;
                },
                outbound = priority_recv => outbound,
                outbound = outbound_recv => outbound,
            };
            let Ok(mut outbound) = next_outbound else {
                break;
            };
            let is_request = outbound.started_tx.is_some();
            if let Some(started_tx) = outbound.started_tx.take()
                && started_tx.send(()).is_err()
            {
                continue;
            }
            if let Some(started) = outbound.started.take() {
                started.store(true, SeqCst);
            }
            let message = serde_json::from_str::<Value>(&outbound.message).ok();
            let is_initialized = message
                .as_ref()
                .and_then(|message| message.get("method"))
                .and_then(Value::as_str)
                == Some(Initialized::METHOD);
            let request_id = message
                .and_then(|message| message.get("id").cloned())
                .and_then(|id| serde_json::from_value(id).ok());
            log::trace!("outgoing message: {}", outbound.message);
            let send = {
                let transport = transport.clone();
                let executor = executor.clone();
                async move {
                    let send = transport.send_cancellable(
                        outbound.message,
                        outbound.cancel_rx,
                        outbound.issued_tx,
                    );
                    let result = if is_request {
                        Some(send.await)
                    } else {
                        let mut send = pin!(send.fuse());
                        let mut timer = pin!(executor.timer(notification_timeout).fuse());
                        select! {
                            result = send => Some(result),
                            _ = timer => {
                                log::error!("context server notification transport exceeded {notification_timeout:?}");
                                if is_initialized {
                                    Some(Err(anyhow!("Context server initialized notification timeout")))
                                } else {
                                    None
                                }
                            }
                        }
                    };
                    result.map(|result| (request_id, result))
                }
            };
            // The server must accept the initialized notification before later HTTP requests arrive.
            if concurrent && !is_initialized {
                pending_sends.push(send.boxed());
            } else if let Some((request_id, Err(err))) = send.await {
                Self::fail_output_send(err, request_id, &response_handlers, &last_transport_error);
                return Ok(());
            }
        }
        drop(output_done_tx);
        Ok(())
    }

    fn fail_output_send(
        err: anyhow::Error,
        request_id: Option<RequestId>,
        response_handlers: &Mutex<Option<HashMap<RequestId, ResponseHandler>>>,
        last_transport_error: &Mutex<Option<anyhow::Error>>,
    ) {
        log::debug!("transport send failed: {:#}", err);
        let error_message: Arc<str> = format!("{err:#}").into();
        *last_transport_error.lock() = Some(err);
        if let Some(mut handlers) = response_handlers.lock().take() {
            let initiating_handler = request_id.and_then(|request_id| handlers.remove(&request_id));
            for handler in handlers.into_values() {
                handler(Err(error_message.clone()));
            }
            drop(initiating_handler);
        }
    }

    /// A future that resolves once the transport's output loop has terminated
    /// — after a send failure, or when this client is dropped — yielding the
    /// reason recorded by the transport.
    ///
    /// Unlike `last_transport_error`, this does not require a request to be in
    /// flight when the transport fails. Returns `None` if the shutdown signal
    /// was already claimed: there is a single signal per client.
    pub(crate) fn wait_for_shutdown(
        &self,
    ) -> Option<future::BoxFuture<'static, TransportShutdownReason>> {
        let (mut input_done, mut output_done) = self.shutdown_rx.lock().take()?;
        let transport = self.transport.lock().clone()?;
        Some(
            async move {
                let mut input_done = pin!(input_done.recv().fuse());
                let mut output_done = pin!(output_done.recv().fuse());
                select! {
                    _ = input_done => TransportShutdownReason::Disconnected,
                    _ = output_done => transport.shutdown_reason(),
                }
            }
            .boxed(),
        )
    }

    /// Sends a JSON-RPC request to the context server and waits for a response.
    /// This function handles serialization, deserialization, timeout, and error handling.
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Serialize,
    ) -> Result<T> {
        self.request_with(
            method,
            params,
            None,
            self.request_timeout.or(Some(DEFAULT_REQUEST_TIMEOUT)),
        )
        .await
    }

    pub async fn request_with<T: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Serialize,
        cancel_rx: Option<oneshot::Receiver<()>>,
        timeout: Option<Duration>,
    ) -> Result<T> {
        let id = self.next_id.fetch_add(1, SeqCst);
        let request = serde_json::to_string(&Request {
            jsonrpc: JSON_RPC_VERSION,
            id: RequestId::Int(id),
            method,
            params,
        })
        .context("serializing context server request")?;

        let (tx, rx) = oneshot::channel();
        let request_id = RequestId::Int(id);
        self.response_handlers
            .lock()
            .as_mut()
            .context("server shut down")?
            .insert(
                request_id.clone(),
                Box::new(move |result| {
                    if tx.send(result).is_err() {
                        log::trace!("context server response receiver was dropped");
                    }
                }),
            );

        let _remove_response_handler = util::defer({
            let request_id = request_id.clone();
            let response_handlers = self.response_handlers.clone();
            move || {
                if let Some(handlers) = response_handlers.lock().as_mut() {
                    handlers.remove(&request_id);
                }
            }
        });

        let (started_tx, started_rx) = oneshot::channel();
        let (issued_tx, issued_rx) = oneshot::channel();
        let request_started = Arc::new(AtomicBool::new(false));
        let (transport_cancel_tx, transport_cancel_rx) = oneshot::channel();
        self.outbound_tx
            .try_send(OutboundMessage {
                message: request,
                started_tx: Some(started_tx),
                issued_tx: Some(issued_tx),
                started: Some(request_started.clone()),
                cancel_rx: Some(transport_cancel_rx),
            })
            .context("failed to write to context server's stdin")?;

        let cancel_transport_on_drop = util::defer({
            move || {
                if transport_cancel_tx.send(()).is_err() {
                    log::trace!("context server transport cancellation receiver was dropped");
                }
            }
        });

        let executor = self.executor.clone();
        let started = Instant::now();
        let mut timeout_fut = pin!(
            async move {
                match timeout {
                    Some(timeout) => {
                        let mut queue_timer = pin!(executor.timer(timeout).fuse());
                        let mut started_rx = pin!(started_rx.fuse());
                        select! {
                            started = started_rx => {
                                if started.is_err() {
                                    future::pending::<bool>().await;
                                }
                                executor.timer(timeout).await;
                                false
                            }
                            _ = queue_timer => true,
                        }
                    }
                    None => future::pending().await,
                }
            }
            .fuse()
        );
        let mut cancel_fut = pin!(
            match cancel_rx {
                Some(rx) => future::Either::Left(async {
                    rx.await.log_err();
                }),
                None => future::Either::Right(future::pending()),
            }
            .fuse()
        );
        let mut issued_rx = Some(issued_rx);

        select! {
            response = rx.fuse() => {
                let elapsed = started.elapsed();
                log::trace!("took {elapsed:?} to receive response to {method:?} id {id}");
                cancel_transport_on_drop.abort();
                match response {
                    Ok(Ok(response)) => {
                        let parsed: AnyResponse = serde_json::from_str(&response)?;
                        if let Some(error) = parsed.error {
                            Err(anyhow!(error.message))
                        } else if let Some(result) = parsed.result {
                            Ok(serde_json::from_str(result.get())?)
                        } else {
                            anyhow::bail!("Invalid response: no result or error");
                        }
                    }
                    Ok(Err(error)) => Err(anyhow!(error)),
                    Err(_canceled) => {
                        if let Some(err) = self.last_transport_error.lock().take() {
                            return Err(err);
                        }
                        anyhow::bail!("cancelled")
                    }
                }
            }
            _ = cancel_fut => {
                if let Some(issued_rx) = issued_rx.take() {
                    self.queue_cancellation_if_issued(
                        method,
                        &request_id,
                        &request_started,
                        issued_rx,
                    );
                }
                anyhow::bail!(RequestCanceled)
            }
            timed_out_in_queue = timeout_fut => {
                if let Some(timeout) = timeout {
                    let phase = if timed_out_in_queue { "queue" } else { "response" };
                    log::error!("cancelled csp request task for {method:?} id {id} after the {phase} exceeded {timeout:?}");
                }
                if let Some(issued_rx) = issued_rx.take() {
                    self.queue_cancellation_if_issued(
                        method,
                        &request_id,
                        &request_started,
                        issued_rx,
                    );
                }
                if timed_out_in_queue {
                    anyhow::bail!("Context server request queue timeout");
                }
                anyhow::bail!("Context server request timeout");
            }
        }
    }

    fn queue_cancellation_if_issued(
        &self,
        method: &str,
        request_id: &RequestId,
        request_started: &AtomicBool,
        mut issued_rx: oneshot::Receiver<()>,
    ) {
        if method == "initialize"
            || !request_started.load(SeqCst)
            || !matches!(issued_rx.try_recv(), Ok(Some(())))
        {
            return;
        }
        let notification = serde_json::to_string(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method: Cancelled::METHOD,
            params: ClientNotification::Cancelled(CancelledParams {
                request_id: request_id.clone(),
                reason: None,
            }),
        });
        let notification = match notification {
            Ok(notification) => notification,
            Err(error) => {
                log::error!("failed to serialize context server cancellation: {error}");
                return;
            }
        };
        if let Err(error) = self
            .priority_outbound_tx
            .try_send(OutboundMessage::notification(notification))
        {
            log::error!("failed to queue context server cancellation: {error}");
        }
    }

    /// Sends a notification to the context server without expecting a response.
    /// This function serializes the notification and sends it through the outbound channel.
    pub fn notify(&self, method: &str, params: impl Serialize) -> Result<()> {
        let notification = serde_json::to_string(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method,
            params,
        })
        .context("serializing context server notification")?;
        self.outbound_tx
            .try_send(OutboundMessage::notification(notification))?;
        Ok(())
    }

    /// Notify the underlying transport of the negotiated MCP protocol version
    /// so it can stamp subsequent requests (e.g. HTTP's `MCP-Protocol-Version`
    /// header required from 2025-06-18 onward).
    pub(crate) fn set_protocol_version(&self, version: &str) {
        if let Some(transport) = self.transport.lock().as_ref() {
            transport.set_protocol_version(version);
        }
    }

    #[must_use]
    pub fn on_notification(
        &self,
        method: &'static str,
        f: Box<dyn 'static + Send + FnMut(Value, AsyncApp)>,
    ) -> NotificationSubscription {
        let mut notification_subscriptions = self.subscription_set.lock();

        NotificationSubscription {
            id: notification_subscriptions.add_handler(method, f),
            set: self.subscription_set.clone(),
        }
    }
}

#[derive(Debug)]
pub struct RequestCanceled;

impl std::error::Error for RequestCanceled {}

impl std::fmt::Display for RequestCanceled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Context server request was canceled")
    }
}

impl fmt::Display for ContextServerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context Server Client")
            .field("id", &self.server_id.0)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

slotmap::new_key_type! {
    struct NotificationSubscriptionId;
}

#[derive(Default)]
pub struct NotificationSubscriptionSet {
    // we have very few subscriptions at the moment
    methods: Vec<(&'static str, Vec<NotificationSubscriptionId>)>,
    handlers: SlotMap<NotificationSubscriptionId, NotificationHandler>,
}

impl NotificationSubscriptionSet {
    #[must_use]
    fn add_handler(
        &mut self,
        method: &'static str,
        handler: NotificationHandler,
    ) -> NotificationSubscriptionId {
        let id = self.handlers.insert(handler);
        if let Some((_, handler_ids)) = self
            .methods
            .iter_mut()
            .find(|(probe_method, _)| method == *probe_method)
        {
            debug_assert!(
                handler_ids.len() < 20,
                "Too many MCP handlers for {}. Consider using a different data structure.",
                method
            );

            handler_ids.push(id);
        } else {
            self.methods.push((method, vec![id]));
        };
        id
    }

    fn notify(&mut self, method: &str, payload: Value, cx: &mut AsyncApp) {
        let Some((_, handler_ids)) = self
            .methods
            .iter_mut()
            .find(|(probe_method, _)| method == *probe_method)
        else {
            return;
        };

        if let Some((last_handler_id, handler_ids)) = handler_ids.split_last() {
            for handler_id in handler_ids {
                if let Some(handler) = self.handlers.get_mut(*handler_id) {
                    handler(payload.clone(), cx.clone());
                }
            }
            if let Some(handler) = self.handlers.get_mut(*last_handler_id) {
                handler(payload, cx.clone());
            }
        }
    }
}

pub struct NotificationSubscription {
    id: NotificationSubscriptionId,
    set: Arc<Mutex<NotificationSubscriptionSet>>,
}

impl Drop for NotificationSubscription {
    fn drop(&mut self) {
        let mut set = self.set.lock();
        set.handlers.remove(self.id);
        set.methods.retain_mut(|(_, handler_ids)| {
            handler_ids.retain(|id| *id != self.id);
            !handler_ids.is_empty()
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use gpui::TestAppContext;
    use std::{
        pin::Pin,
        sync::atomic::{AtomicBool, Ordering},
        time::Duration,
    };

    struct RecordingTransport {
        incoming_tx: async_channel::Sender<String>,
        incoming_rx: async_channel::Receiver<String>,
        sent_messages: Mutex<Vec<Value>>,
        block_next_request: AtomicBool,
        block_next_notification: AtomicBool,
    }

    impl RecordingTransport {
        fn new() -> Self {
            let (incoming_tx, incoming_rx) = async_channel::unbounded();
            Self {
                incoming_tx,
                incoming_rx,
                sent_messages: Mutex::new(Vec::new()),
                block_next_request: AtomicBool::new(false),
                block_next_notification: AtomicBool::new(false),
            }
        }

        fn blocking_once() -> Self {
            let transport = Self::new();
            transport.block_next_request.store(true, Ordering::SeqCst);
            transport
        }

        fn blocking_notification_once() -> Self {
            let transport = Self::new();
            transport
                .block_next_notification
                .store(true, Ordering::SeqCst);
            transport
        }

        fn disconnect(&self) {
            self.incoming_tx.close();
        }

        fn send_incoming(&self, message: Value) {
            self.incoming_tx
                .try_send(message.to_string())
                .expect("incoming message should be sent");
        }

        fn sent_messages(&self) -> Vec<Value> {
            self.sent_messages.lock().clone()
        }
    }

    #[async_trait]
    impl Transport for RecordingTransport {
        async fn send(&self, message: String) -> Result<()> {
            self.sent_messages
                .lock()
                .push(serde_json::from_str(&message)?);
            Ok(())
        }

        async fn send_cancellable(
            &self,
            message: String,
            cancel_rx: Option<oneshot::Receiver<()>>,
            issued_tx: Option<oneshot::Sender<()>>,
        ) -> Result<()> {
            self.send(message).await?;
            if let Some(issued_tx) = issued_tx
                && issued_tx.send(()).is_err()
            {
                log::trace!("test request-issued receiver was dropped");
            }
            if self.block_next_notification.swap(false, Ordering::SeqCst) && cancel_rx.is_none() {
                future::pending::<()>().await;
            }
            if self.block_next_request.swap(false, Ordering::SeqCst)
                && let Some(cancel_rx) = cancel_rx
            {
                if cancel_rx.await.is_err() {
                    log::trace!("test transport cancellation sender was dropped");
                }
            }
            Ok(())
        }

        fn receive(&self) -> Pin<Box<dyn futures::Stream<Item = String> + Send>> {
            Box::pin(self.incoming_rx.clone())
        }

        fn receive_err(&self) -> Pin<Box<dyn futures::Stream<Item = String> + Send>> {
            Box::pin(futures::stream::pending())
        }
    }

    #[gpui::test]
    async fn request_timeout_removes_handler_and_aborts_transport(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(1)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/list", ()).await }
        });

        cx.executor().run_until_parked();
        assert_eq!(transport.sent_messages().len(), 1);

        cx.executor().advance_clock(Duration::from_secs(2));
        let error = request.await.expect_err("request should time out");
        cx.executor().run_until_parked();

        assert_eq!(error.to_string(), "Context server request timeout");
        assert_eq!(
            client.response_handlers.lock().as_ref().map(HashMap::len),
            Some(0)
        );
        let messages = transport.sent_messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0]["method"], "tools/list");
        assert_eq!(messages[1]["method"], Cancelled::METHOD);
        assert_eq!(messages[1]["params"]["requestId"], 0);

        transport.send_incoming(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 0,
            "result": { "late": true }
        }));
        cx.executor().run_until_parked();
        assert_eq!(
            client.response_handlers.lock().as_ref().map(HashMap::len),
            Some(0)
        );
    }

    #[gpui::test]
    async fn initialize_timeout_does_not_notify_server(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(1)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("initialize", ()).await }
        });

        cx.executor().run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(2));
        let error = request.await.expect_err("initialize should time out");
        cx.executor().run_until_parked();

        assert_eq!(error.to_string(), "Context server request timeout");
        assert_eq!(transport.sent_messages().len(), 1);
        assert_eq!(transport.sent_messages()[0]["method"], "initialize");
    }

    #[gpui::test]
    async fn dropping_request_aborts_transport_and_removes_handler(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/list", ()).await }
        });

        cx.executor().run_until_parked();
        assert_eq!(transport.sent_messages().len(), 1);
        drop(request);
        cx.executor().run_until_parked();

        assert_eq!(transport.sent_messages().len(), 1);
        assert_eq!(
            client.response_handlers.lock().as_ref().map(HashMap::len),
            Some(0)
        );
    }

    #[gpui::test]
    async fn stopping_client_cancels_io_and_fails_pending_requests(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let blocking_request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/list", ()).await }
        });
        cx.executor().run_until_parked();

        let queued_request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/list", ()).await }
        });
        cx.executor().run_until_parked();
        assert_eq!(transport.sent_messages().len(), 1);

        client.stop();
        client.stop();
        transport.send_incoming(serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/tools/list_changed"
        }));
        cx.executor().run_until_parked();

        for request in [blocking_request, queued_request] {
            let error = request
                .now_or_never()
                .expect("stopping should fail requests without waiting for their timeout")
                .expect_err("a stopped client's request should fail");
            assert_eq!(error.to_string(), "Context server stopped");
        }
        assert!(client.io_tasks.lock().is_none());
        assert!(client.response_handlers.lock().is_none());
        assert_eq!(transport.incoming_rx.len(), 1);
        assert_eq!(transport.sent_messages().len(), 1);
        assert!(
            client
                .notify("notifications/tools/list_changed", ())
                .is_err()
        );
        assert!(client.request::<Value>("tools/list", ()).await.is_err());
        let weak_transport = Arc::downgrade(&transport);
        drop(transport);
        assert!(
            weak_transport.upgrade().is_none(),
            "retaining a stopped client must not keep its transport alive"
        );
    }

    #[gpui::test]
    async fn request_queue_wait_has_its_own_timeout(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let blocking_request = cx.spawn({
            let client = client.clone();
            move |_| async move {
                client
                    .request_with::<Value>("tools/list", (), None, Some(Duration::from_secs(60)))
                    .await
            }
        });
        cx.executor().run_until_parked();
        let queued_request = cx.spawn({
            let client = client.clone();
            move |_| async move {
                client
                    .request_with::<Value>("prompts/list", (), None, Some(Duration::from_secs(1)))
                    .await
            }
        });

        cx.executor().run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(2));
        let error = queued_request
            .await
            .expect_err("queued request should have its own timeout");

        assert_eq!(error.to_string(), "Context server request queue timeout");
        assert_eq!(transport.sent_messages().len(), 1);
        assert_eq!(transport.sent_messages()[0]["method"], "tools/list");
        drop(blocking_request);
        cx.executor().run_until_parked();
        assert_eq!(
            client.response_handlers.lock().as_ref().map(HashMap::len),
            Some(0)
        );
    }

    #[gpui::test]
    async fn cancellation_is_not_sent_for_a_request_still_in_the_queue(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_notification_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        client
            .notify("notifications/test", ())
            .expect("notification should be queued");
        cx.executor().run_until_parked();
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let request = cx.spawn({
            let client = client.clone();
            move |_| async move {
                client
                    .request_with::<Value>("tools/list", (), Some(cancel_rx), None)
                    .await
            }
        });
        cx.executor().run_until_parked();

        cancel_tx
            .send(())
            .expect("request cancellation should be sent");
        cx.executor().run_until_parked();
        let error = request.await.expect_err("request should be canceled");

        assert_eq!(error.to_string(), RequestCanceled.to_string());
        assert_eq!(transport.sent_messages().len(), 1);
        assert_eq!(transport.sent_messages()[0]["method"], "notifications/test");
    }

    #[gpui::test]
    async fn cancellation_is_prioritized_after_the_issued_request(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let first_request = cx.spawn({
            let client = client.clone();
            move |_| async move {
                client
                    .request_with::<Value>("tools/list", (), Some(cancel_rx), None)
                    .await
            }
        });
        cx.executor().run_until_parked();
        let second_request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("prompts/list", ()).await }
        });
        cx.executor().run_until_parked();

        cancel_tx
            .send(())
            .expect("request cancellation should be sent");
        cx.executor().run_until_parked();
        let error = first_request
            .await
            .expect_err("first request should be canceled");

        assert_eq!(error.to_string(), RequestCanceled.to_string());
        let messages = transport.sent_messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0]["method"], "tools/list");
        assert_eq!(messages[1]["method"], Cancelled::METHOD);
        assert_eq!(messages[1]["params"]["requestId"], 0);
        assert_eq!(messages[2]["method"], "prompts/list");
        drop(second_request);
        cx.executor().run_until_parked();
    }

    #[gpui::test]
    async fn notification_timeout_unblocks_later_requests(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::blocking_notification_once());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(2)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        client
            .notify("notifications/test", ())
            .expect("notification should be queued");
        cx.executor().run_until_parked();
        let request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/list", ()).await }
        });

        cx.executor().run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        transport.send_incoming(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 0,
            "result": { "recovered": true }
        }));
        cx.executor().run_until_parked();

        assert_eq!(
            request
                .await
                .expect("request should run after the notification timeout"),
            serde_json::json!({"recovered": true})
        );
        assert_eq!(transport.sent_messages().len(), 2);
        assert_eq!(transport.sent_messages()[0]["method"], "notifications/test");
        assert_eq!(transport.sent_messages()[1]["method"], "tools/list");
        assert_eq!(
            client.response_handlers.lock().as_ref().map(HashMap::len),
            Some(0)
        );
    }

    #[gpui::test]
    async fn input_disconnect_fails_pending_request(cx: &mut TestAppContext) {
        let transport = Arc::new(RecordingTransport::new());
        let client = Arc::new(
            Client::new(
                ContextServerId("test-server".into()),
                "test-server".into(),
                transport.clone(),
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("initialize", ()).await }
        });

        cx.executor().run_until_parked();
        assert_eq!(transport.sent_messages().len(), 1);
        transport.disconnect();
        cx.executor().run_until_parked();

        let error = request
            .await
            .expect_err("disconnect should fail the pending request");
        assert_eq!(error.to_string(), "Context server disconnected");
        assert!(client.response_handlers.lock().is_none());
    }
}
