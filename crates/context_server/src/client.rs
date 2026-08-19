use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::{FutureExt, StreamExt, channel::oneshot, future, select};
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
        atomic::{AtomicI32, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use util::{ResultExt, TryFutureExt};

use crate::{
    oauth::WwwAuthenticate,
    transport::{StdioTransport, Transport},
    types::{CancelledParams, ClientNotification, Notification as _, notifications::Cancelled},
};

const JSON_RPC_VERSION: &str = "2.0";
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

// Standard JSON-RPC error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

type ResponseHandler = Box<dyn Send + FnOnce(String)>;
type NotificationHandler = Box<dyn Send + FnMut(Value, AsyncApp)>;
type RequestHandler = Box<dyn Send + FnMut(RequestId, &RawValue, AsyncApp)>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Int(i32),
    Str(String),
}

pub(crate) struct Client {
    server_id: ContextServerId,
    next_id: AtomicI32,
    outbound_tx: async_channel::Sender<String>,
    name: Arc<str>,
    subscription_set: Arc<Mutex<NotificationSubscriptionSet>>,
    response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    #[allow(clippy::type_complexity)]
    #[allow(dead_code)]
    io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    output_done_rx: Mutex<Option<barrier::Receiver>>,
    executor: BackgroundExecutor,
    transport: Arc<dyn Transport>,
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

/// A JSON-RPC error returned by the server. Request futures fail with this
/// type (wrapped in `anyhow::Error`), so callers can downcast to inspect the
/// error code — e.g. to recognize the spec-defined negotiation errors.
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub code: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (JSON-RPC error {})", self.message, self.code)
    }
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
            "starting context server (executable={:?}, args={:?})",
            binary.executable,
            &binary.args
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
        // The same transport can outlive a client generation (the store
        // restarts servers over a retained HTTP transport): discard whatever
        // the previous generation left behind before this one attaches.
        transport.reset();

        let (outbound_tx, outbound_rx) = async_channel::unbounded::<String>();
        let (output_done_tx, output_done_rx) = barrier::channel();

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
        let input_task = cx.spawn(async move |_| {
            let (input, err) = futures::join!(receive_input_task, receive_err_task);
            input.or(err)
        });

        let last_transport_error: Arc<Mutex<Option<anyhow::Error>>> = Arc::new(Mutex::new(None));
        let output_task = cx.background_spawn({
            let transport = transport.clone();
            let last_transport_error = last_transport_error.clone();
            Self::handle_output(
                transport,
                outbound_rx,
                output_done_tx,
                response_handlers.clone(),
                last_transport_error,
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
            executor: cx.background_executor().clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            transport,
            request_timeout,
            last_transport_error,
        })
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
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        let mut receiver = transport.receive();

        while let Some(message) = receiver.next().await {
            log::trace!("recv: {}", &message);
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
                    handler(message.to_string());
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
        outbound_rx: async_channel::Receiver<String>,
        output_done_tx: barrier::Sender,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        last_transport_error: Arc<Mutex<Option<anyhow::Error>>>,
    ) -> anyhow::Result<()> {
        let _clear_response_handlers = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                response_handlers.lock().take();
            }
        });
        while let Ok(message) = outbound_rx.recv().await {
            log::trace!("outgoing message: {}", message);
            if let Err(err) = transport.send(message).await {
                log::debug!("transport send failed: {:#}", err);
                *last_transport_error.lock() = Some(err);
                return Ok(());
            }
        }
        drop(output_done_tx);
        Ok(())
    }

    /// A future that resolves once the transport's output loop has terminated
    /// — after a send failure, or when this client is dropped — yielding the
    /// authentication challenge recorded by the transport if it shut down on a
    /// `401 Unauthorized` response.
    ///
    /// Unlike `last_transport_error`, this does not require a request to be in
    /// flight when the transport fails. Returns `None` if the shutdown signal
    /// was already claimed: there is a single signal per client.
    pub(crate) fn wait_for_shutdown(
        &self,
    ) -> Option<future::BoxFuture<'static, Option<WwwAuthenticate>>> {
        let mut output_done = self.output_done_rx.lock().take()?;
        let transport = self.transport.clone();
        Some(
            async move {
                output_done.recv().await;
                transport.auth_challenge()
            }
            .boxed(),
        )
    }

    /// The timeout applied to requests that don't specify one explicitly.
    pub(crate) fn default_request_timeout(&self) -> Option<Duration> {
        self.request_timeout.or(Some(DEFAULT_REQUEST_TIMEOUT))
    }

    /// Sends a JSON-RPC request to the context server and waits for a response.
    /// This function handles serialization, deserialization, timeout, and error handling.
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Serialize,
    ) -> Result<T> {
        self.request_with(method, params, None, self.default_request_timeout())
            .await
    }

    pub async fn request_with<T: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Serialize,
        cancel_rx: Option<&mut oneshot::Receiver<()>>,
        timeout: Option<Duration>,
    ) -> Result<T> {
        let id = self.next_id.fetch_add(1, SeqCst);
        let request = serde_json::to_string(&Request {
            jsonrpc: JSON_RPC_VERSION,
            id: RequestId::Int(id),
            method,
            params,
        })
        .unwrap();

        let (tx, rx) = oneshot::channel();
        let handle_response = self
            .response_handlers
            .lock()
            .as_mut()
            .context("server shut down")
            .map(|handlers| {
                handlers.insert(
                    RequestId::Int(id),
                    Box::new(move |result| {
                        let _ = tx.send(result);
                    }),
                );
            });

        let send = self
            .outbound_tx
            .try_send(request)
            .context("failed to write to context server's stdin");

        let executor = self.executor.clone();
        let started = Instant::now();
        handle_response?;
        send?;

        // Cancel the request server-side whenever this future exits without
        // having received a response: explicit cancellation, timeout, or the
        // caller dropping the future (e.g. a tool call raced against user
        // cancellation). The notification is the cancellation wire signal on
        // stdio; the HTTP transport translates it into closing the request's
        // response stream on modern servers.
        let mut cancel_guard = CancelOnDrop {
            client: Some((
                self.outbound_tx.clone(),
                self.response_handlers.clone(),
                self.transport.clone(),
            )),
            request_id: RequestId::Int(id),
        };

        let mut timeout_fut = pin!(
            match timeout {
                Some(timeout) => future::Either::Left(executor.timer(timeout)),
                None => future::Either::Right(future::pending()),
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

        select! {
            response = rx.fuse() => {
                let elapsed = started.elapsed();
                log::trace!("took {elapsed:?} to receive response to {method:?} id {id}");
                cancel_guard.disarm();
                match response {
                    Ok(response) => {
                        let parsed: AnyResponse = serde_json::from_str(&response)?;
                        if let Some(error) = parsed.error {
                            Err(anyhow::Error::new(error))
                        } else if let Some(result) = parsed.result {
                            Ok(serde_json::from_str(result.get())?)
                        } else {
                            anyhow::bail!("Invalid response: no result or error");
                        }
                    }
                    Err(_canceled) => {
                        if let Some(err) = self.last_transport_error.lock().take() {
                            return Err(err);
                        }
                        anyhow::bail!("cancelled")
                    }
                }
            }
            _ = cancel_fut => {
                drop(cancel_guard);
                anyhow::bail!(RequestCanceled)
            }
            _ = timeout_fut => {
                log::error!("cancelled csp request task for {method:?} id {id} which took over {:?}", timeout.unwrap());
                drop(cancel_guard);
                anyhow::bail!(RequestTimedOut);
            }
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
        .unwrap();
        self.outbound_tx.try_send(notification)?;
        Ok(())
    }

    /// Notify the underlying transport of the negotiated MCP protocol version
    /// so it can stamp subsequent requests (e.g. HTTP's `MCP-Protocol-Version`
    /// header required from 2025-06-18 onward).
    pub(crate) fn set_protocol_version(&self, version: &str) {
        self.transport.set_protocol_version(version);
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

/// Cancels an in-flight request when its future is abandoned before a
/// response arrived: sends `notifications/cancelled` for the request and
/// removes its response handler. Disarmed once a response is received.
struct CancelOnDrop {
    #[allow(clippy::type_complexity)]
    client: Option<(
        async_channel::Sender<String>,
        Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        Arc<dyn Transport>,
    )>,
    request_id: RequestId,
}

impl CancelOnDrop {
    fn disarm(&mut self) {
        self.client.take();
    }
}

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        let Some((outbound_tx, response_handlers, transport)) = self.client.take() else {
            return;
        };
        if let Some(handlers) = response_handlers.lock().as_mut() {
            handlers.remove(&self.request_id);
        }
        // Abort out-of-band first: the queued notification below cannot be
        // delivered while an earlier send is still awaiting its response,
        // and on modern HTTP closing the request's stream — not the
        // notification — is the cancellation signal.
        if let Ok(request_id) = serde_json::to_value(&self.request_id) {
            transport.cancel_request(&request_id);
        }
        let notification = serde_json::to_string(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method: Cancelled::METHOD,
            params: ClientNotification::Cancelled(CancelledParams {
                request_id: self.request_id.clone(),
                reason: None,
            }),
        })
        .unwrap();
        // A closed channel means the transport already shut down; there is
        // nothing left to cancel.
        outbound_tx.try_send(notification).ok();
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

#[derive(Debug)]
pub struct RequestTimedOut;

impl std::error::Error for RequestTimedOut {}

impl std::fmt::Display for RequestTimedOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Context server request timeout")
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

        for handler_id in handler_ids {
            if let Some(handler) = self.handlers.get_mut(*handler_id) {
                handler(payload.clone(), cx.clone());
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
    use crate::transport::Transport;
    use async_trait::async_trait;
    use futures::Stream;
    use gpui::TestAppContext;
    use std::pin::Pin;

    /// Replies to every request with the same canned JSON-RPC error.
    struct ErrorTransport {
        tx: async_channel::Sender<String>,
        rx: async_channel::Receiver<String>,
    }

    impl ErrorTransport {
        fn new() -> Self {
            let (tx, rx) = async_channel::unbounded();
            Self { tx, rx }
        }
    }

    #[async_trait]
    impl Transport for ErrorTransport {
        async fn send(&self, message: String) -> Result<()> {
            let message: Value = serde_json::from_str(&message)?;
            let id = message.get("id").cloned().unwrap_or(Value::Null);
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32022,
                    "message": "Unsupported protocol version",
                    "data": { "supported": ["2026-07-28"] }
                }
            });
            self.tx.send(response.to_string()).await?;
            Ok(())
        }

        fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
            Box::pin(self.rx.clone())
        }

        fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
            Box::pin(futures::stream::empty())
        }
    }

    #[gpui::test]
    async fn test_json_rpc_errors_are_downcastable(cx: &mut TestAppContext) {
        let client = Client::new(
            ContextServerId("test-server".into()),
            "test-server".into(),
            Arc::new(ErrorTransport::new()),
            None,
            cx.to_async(),
        )
        .unwrap();

        let err = client
            .request::<Value>("server/discover", serde_json::json!({}))
            .await
            .unwrap_err();

        let error = err
            .downcast_ref::<Error>()
            .expect("should be a typed Error");
        assert_eq!(error.code, -32022);
        assert_eq!(error.message, "Unsupported protocol version");
        assert_eq!(
            error.data.as_ref().and_then(|data| data.get("supported")),
            Some(&serde_json::json!(["2026-07-28"]))
        );
    }
}
