use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::{channel::oneshot, io::BufWriter, select, AsyncRead, AsyncWrite, FutureExt};
use gpui::{AsyncAppContext, BackgroundExecutor, Task};
use parking_lot::Mutex;
use postage::barrier;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{value::RawValue, Value};
use smol::{
    channel,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{self, Child},
};
use std::{
    fmt,
    path::PathBuf,
    sync::{
        atomic::{AtomicI32, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, Instant},
};
use util::TryFutureExt;

const JSON_RPC_VERSION: &str = "2.0";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

type ResponseHandler = Box<dyn Send + FnOnce(Result<String, Error>)>;
type NotificationHandler = Box<dyn Send + FnMut(RequestId, Value, AsyncAppContext)>;

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
    server: Arc<Mutex<Option<Child>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ContextServerId(pub String);

#[derive(Serialize, Deserialize)]
struct Request<'a, T> {
    jsonrpc: &'static str,
    id: RequestId,
    method: &'a str,
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
    id: RequestId,
    #[serde(borrow)]
    method: &'a str,
    params: T,
}

#[derive(Debug, Clone, Deserialize)]
struct AnyNotification<'a> {
    jsonrpc: &'a str,
    id: RequestId,
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
    pub fn new(
        server_id: ContextServerId,
        binary: ModelContextServerBinary,
        cx: AsyncAppContext,
    ) -> Result<Self> {
        log::info!(
            "starting context server (executable={:?}, args={:?})",
            binary.executable,
            &binary.args
        );

        let mut command = process::Command::new(&binary.executable);
        command
            .args(&binary.args)
            .envs(binary.env.unwrap_or_default())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);

        let mut server = command.spawn().with_context(|| {
            format!(
                "failed to spawn command. (path={:?}, args={:?})",
                binary.executable, &binary.args
            )
        })?;

        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        let stderr = server.stderr.take().unwrap();

        let (outbound_tx, outbound_rx) = channel::unbounded::<String>();
        let (output_done_tx, output_done_rx) = barrier::channel();

        let notification_handlers =
            Arc::new(Mutex::new(HashMap::<_, NotificationHandler>::default()));
        let response_handlers =
            Arc::new(Mutex::new(Some(HashMap::<_, ResponseHandler>::default())));

        let stdout_input_task = cx.spawn({
            let notification_handlers = notification_handlers.clone();
            let response_handlers = response_handlers.clone();
            move |cx| {
                Self::handle_input(stdout, notification_handlers, response_handlers, cx).log_err()
            }
        });
        let stderr_input_task = cx.spawn(|_| Self::handle_stderr(stderr).log_err());
        let input_task = cx.spawn(|_| async move {
            let (stdout, stderr) = futures::join!(stdout_input_task, stderr_input_task);
            stdout.or(stderr)
        });
        let output_task = cx.background_executor().spawn({
            Self::handle_output(
                stdin,
                outbound_rx,
                output_done_tx,
                response_handlers.clone(),
            )
            .log_err()
        });

        let mut context_server = Self {
            server_id,
            notification_handlers,
            response_handlers,
            name: "".into(),
            next_id: Default::default(),
            outbound_tx,
            executor: cx.background_executor().clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            server: Arc::new(Mutex::new(Some(server))),
        };

        if let Some(name) = binary.executable.file_name() {
            context_server.name = name.to_string_lossy().into();
        }

        Ok(context_server)
    }

    /// Handles input from the server's stdout.
    ///
    /// This function continuously reads lines from the provided stdout stream,
    /// parses them as JSON-RPC responses or notifications, and dispatches them
    /// to the appropriate handlers. It processes both responses (which are matched
    /// to pending requests) and notifications (which trigger registered handlers).
    async fn handle_input<Stdout>(
        stdout: Stdout,
        notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<()>
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        let mut stdout = BufReader::new(stdout);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            if stdout.read_line(&mut buffer).await? == 0 {
                return Ok(());
            }

            let content = buffer.trim();

            if !content.is_empty() {
                if let Ok(response) = serde_json::from_str::<AnyResponse>(&content) {
                    if let Some(handlers) = response_handlers.lock().as_mut() {
                        if let Some(handler) = handlers.remove(&response.id) {
                            handler(Ok(content.to_string()));
                        }
                    }
                } else if let Ok(notification) = serde_json::from_str::<AnyNotification>(&content) {
                    let mut notification_handlers = notification_handlers.lock();
                    if let Some(handler) =
                        notification_handlers.get_mut(notification.method.as_str())
                    {
                        handler(
                            notification.id,
                            notification.params.unwrap_or(Value::Null),
                            cx.clone(),
                        );
                    }
                }
            }

            smol::future::yield_now().await;
        }
    }

    /// Handles the stderr output from the context server.
    /// Continuously reads and logs any error messages from the server.
    async fn handle_stderr<Stderr>(stderr: Stderr) -> anyhow::Result<()>
    where
        Stderr: AsyncRead + Unpin + Send + 'static,
    {
        let mut stderr = BufReader::new(stderr);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            if stderr.read_line(&mut buffer).await? == 0 {
                return Ok(());
            }
            log::warn!("context server stderr: {}", buffer.trim());
            smol::future::yield_now().await;
        }
    }

    /// Handles the output to the context server's stdin.
    /// This function continuously receives messages from the outbound channel,
    /// writes them to the server's stdin, and manages the lifecycle of response handlers.
    async fn handle_output<Stdin>(
        stdin: Stdin,
        outbound_rx: channel::Receiver<String>,
        output_done_tx: barrier::Sender,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    ) -> anyhow::Result<()>
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
    {
        let mut stdin = BufWriter::new(stdin);
        let _clear_response_handlers = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                response_handlers.lock().take();
            }
        });
        while let Ok(message) = outbound_rx.recv().await {
            log::trace!("outgoing message: {}", message);

            stdin.write_all(message.as_bytes()).await?;
            stdin.write_all(b"\n").await?;
            stdin.flush().await?;
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
        let id = self.next_id.fetch_add(1, SeqCst);
        let notification = serde_json::to_string(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            id: RequestId::Int(id),
            method,
            params,
        })
        .unwrap();
        self.outbound_tx.try_send(notification)?;
        Ok(())
    }

    pub fn on_notification<F>(&self, method: &'static str, mut f: F)
    where
        F: 'static + Send + FnMut(Value, AsyncAppContext),
    {
        self.notification_handlers
            .lock()
            .insert(method, Box::new(move |_, params, cx| f(params, cx)));
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server_id(&self) -> ContextServerId {
        self.server_id.clone()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(mut server) = self.server.lock().take() {
            let _ = server.kill();
        }
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
