use anyhow::{Context, Result, anyhow};
use collections::HashMap;
use futures::{FutureExt, StreamExt, channel::oneshot, select};
use gpui::{AppContext as _, AsyncApp, BackgroundExecutor, Task};
use parking_lot::Mutex;
use postage::barrier;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, value::RawValue};
use smol::channel;
use std::{
    fmt,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicI32, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use util::TryFutureExt;

use crate::transport::{StdioTransport, Transport};

const JSON_RPC_VERSION: &str = "2.0";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

// Standard JSON-RPC error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

type ResponseHandler = Box<dyn Send + FnOnce(Result<String, Error>)>;
type NotificationHandler = Box<dyn Send + FnMut(Value, AsyncApp)>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Int(i32),
    Str(String),
}

pub struct Client {
    server_id: ContextServerId,
    next_id: AtomicI32,
    outbound_tx: channel::Sender<String>,
    name: Arc<str>,
    notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    #[allow(clippy::type_complexity)]
    #[allow(dead_code)]
    io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    #[allow(dead_code)]
    output_done_rx: Mutex<Option<barrier::Receiver>>,
    executor: BackgroundExecutor,
    #[allow(dead_code)]
    transport: Arc<dyn Transport>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ContextServerId(pub Arc<str>);

fn is_null_value<T: Serialize>(value: &T) -> bool {
    if let Ok(Value::Null) = serde_json::to_value(value) {
        true
    } else {
        false
    }
}

#[derive(Serialize, Deserialize)]
struct Request<'a, T> {
    jsonrpc: &'static str,
    id: RequestId,
    method: &'a str,
    #[serde(skip_serializing_if = "is_null_value")]
    params: T,
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

#[derive(Deserialize)]
#[allow(dead_code)]
struct Response<T> {
    jsonrpc: &'static str,
    id: RequestId,
    #[serde(flatten)]
    value: CspResult<T>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum CspResult<T> {
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
    params: T,
}

#[derive(Debug, Clone, Deserialize)]
struct AnyNotification<'a> {
    jsonrpc: &'a str,
    method: String,
    #[serde(default)]
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelContextServerBinary {
    pub executable: PathBuf,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
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
        cx: AsyncApp,
    ) -> Result<Self> {
        log::info!(
            "starting context server (executable={:?}, args={:?})",
            binary.executable,
            &binary.args
        );

        let server_name = binary
            .executable
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(String::new);

        let transport = Arc::new(StdioTransport::new(binary, &cx)?);
        Self::new(server_id, server_name.into(), transport, cx)
    }

    /// Creates a new Client instance for a context server.
    pub fn new(
        server_id: ContextServerId,
        server_name: Arc<str>,
        transport: Arc<dyn Transport>,
        cx: AsyncApp,
    ) -> Result<Self> {
        let (outbound_tx, outbound_rx) = channel::unbounded::<String>();
        let (output_done_tx, output_done_rx) = barrier::channel();

        let notification_handlers =
            Arc::new(Mutex::new(HashMap::<_, NotificationHandler>::default()));
        let response_handlers =
            Arc::new(Mutex::new(Some(HashMap::<_, ResponseHandler>::default())));

        let receive_input_task = cx.spawn({
            let notification_handlers = notification_handlers.clone();
            let response_handlers = response_handlers.clone();
            let transport = transport.clone();
            async move |cx| {
                Self::handle_input(transport, notification_handlers, response_handlers, cx)
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

        let output_task = cx.background_spawn({
            let transport = transport.clone();
            Self::handle_output(
                transport,
                outbound_rx,
                output_done_tx,
                response_handlers.clone(),
            )
            .log_err()
        });

        Ok(Self {
            server_id,
            notification_handlers,
            response_handlers,
            name: server_name,
            next_id: Default::default(),
            outbound_tx,
            executor: cx.background_executor().clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            transport,
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
        notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        let mut receiver = transport.receive();

        while let Some(message) = receiver.next().await {
            if let Ok(response) = serde_json::from_str::<AnyResponse>(&message) {
                if let Some(handlers) = response_handlers.lock().as_mut() {
                    if let Some(handler) = handlers.remove(&response.id) {
                        handler(Ok(message.to_string()));
                    }
                }
            } else if let Ok(notification) = serde_json::from_str::<AnyNotification>(&message) {
                let mut notification_handlers = notification_handlers.lock();
                if let Some(handler) = notification_handlers.get_mut(notification.method.as_str()) {
                    handler(notification.params.unwrap_or(Value::Null), cx.clone());
                }
            }
        }

        smol::future::yield_now().await;

        Ok(())
    }

    /// Handles the stderr output from the context server.
    /// Continuously reads and logs any error messages from the server.
    async fn handle_err(transport: Arc<dyn Transport>) -> anyhow::Result<()> {
        while let Some(err) = transport.receive_err().next().await {
            log::warn!("context server stderr: {}", err.trim());
        }

        Ok(())
    }

    /// Handles the output to the context server's stdin.
    /// This function continuously receives messages from the outbound channel,
    /// writes them to the server's stdin, and manages the lifecycle of response handlers.
    async fn handle_output(
        transport: Arc<dyn Transport>,
        outbound_rx: channel::Receiver<String>,
        output_done_tx: barrier::Sender,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    ) -> anyhow::Result<()> {
        let _clear_response_handlers = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                response_handlers.lock().take();
            }
        });
        while let Ok(message) = outbound_rx.recv().await {
            log::trace!("outgoing message: {}", message);
            transport.send(message).await?;
        }
        drop(output_done_tx);
        Ok(())
    }

    /// Sends a JSON-RPC request to the context server and waits for a response.
    /// This function handles serialization, deserialization, timeout, and error handling.
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Serialize,
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
            .ok_or_else(|| anyhow!("server shut down"))
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

        let mut timeout = executor.timer(REQUEST_TIMEOUT).fuse();
        select! {
            response = rx.fuse() => {
                let elapsed = started.elapsed();
                log::trace!("took {elapsed:?} to receive response to {method:?} id {id}");
                match response? {
                    Ok(response) => {
                        let parsed: AnyResponse = serde_json::from_str(&response)?;
                        if let Some(error) = parsed.error {
                            Err(anyhow!(error.message))
                        } else if let Some(result) = parsed.result {
                            Ok(serde_json::from_str(result.get())?)
                        } else {
                            Err(anyhow!("Invalid response: no result or error"))
                        }
                    }
                    Err(_) => anyhow::bail!("cancelled")
                }
            }
            _ = timeout => {
                log::error!("cancelled csp request task for {method:?} id {id} which took over {:?}", REQUEST_TIMEOUT);
                anyhow::bail!("Context server request timeout");
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

    pub fn on_notification<F>(&self, method: &'static str, f: F)
    where
        F: 'static + Send + FnMut(Value, AsyncApp),
    {
        self.notification_handlers
            .lock()
            .insert(method, Box::new(f));
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server_id(&self) -> ContextServerId {
        self.server_id.clone()
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
