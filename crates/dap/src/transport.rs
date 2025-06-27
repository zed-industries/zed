use anyhow::{Context as _, Result, anyhow, bail};
#[cfg(any(test, feature = "test-support"))]
use async_pipe::{PipeReader, PipeWriter};
use dap_types::{
    ErrorResponse,
    messages::{Message, Response},
};
use futures::{AsyncRead, AsyncReadExt as _, AsyncWrite, FutureExt as _, channel::oneshot, select};
use gpui::{AppContext as _, AsyncApp, BackgroundExecutor, Task};
use parking_lot::Mutex;
use proto::ErrorExt;
use settings::Settings as _;
use smallvec::SmallVec;
use smol::{
    channel::{Receiver, Sender, unbounded},
    io::{AsyncBufReadExt as _, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    process::Stdio,
    sync::Arc,
    time::Duration,
};
use task::TcpArgumentsTemplate;
use util::ConnectionResult;

use crate::{
    adapters::{DebugAdapterBinary, TcpArguments},
    client::DapMessageHandler,
    debugger_settings::DebuggerSettings,
};

pub(crate) type IoMessage = str;
pub(crate) type Command = str;
pub type IoHandler = Box<dyn Send + FnMut(IoKind, Option<&Command>, &IoMessage)>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LogKind {
    Adapter,
    Rpc,
}

#[derive(Clone, Copy)]
pub enum IoKind {
    StdIn,
    StdOut,
    StdErr,
}

type Requests = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Response>>>>>;
type LogHandlers = Arc<Mutex<SmallVec<[(LogKind, IoHandler); 2]>>>;

pub trait Transport: Send + Sync {
    fn has_adapter_logs(&self) -> bool;
    fn tcp_arguments(&self) -> Option<TcpArguments>;
    fn connect(
        &mut self,
    ) -> Task<
        Result<(
            Box<dyn AsyncWrite + Unpin + Send + 'static>,
            Box<dyn AsyncRead + Unpin + Send + 'static>,
        )>,
    >;
    fn kill(&mut self);
    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &FakeTransport {
        unreachable!()
    }
}

async fn start(
    binary: &DebugAdapterBinary,
    log_handlers: LogHandlers,
    cx: &mut AsyncApp,
) -> Result<Box<dyn Transport>> {
    #[cfg(any(test, feature = "test-support"))]
    if cfg!(any(test, feature = "test-support")) {
        return Ok(Box::new(FakeTransport::start(cx).await?));
    }

    if binary.connection.is_some() {
        Ok(Box::new(
            TcpTransport::start(binary, log_handlers, cx).await?,
        ))
    } else {
        Ok(Box::new(
            StdioTransport::start(binary, log_handlers, cx).await?,
        ))
    }
}

pub(crate) struct TransportDelegate {
    log_handlers: LogHandlers,
    pub(crate) pending_requests: Requests,
    pub(crate) transport: Mutex<Box<dyn Transport>>,
    pub(crate) server_tx: smol::lock::Mutex<Option<Sender<Message>>>,
    tasks: Mutex<Vec<Task<()>>>,
}

impl Drop for TransportDelegate {
    fn drop(&mut self) {
        self.transport.lock().kill()
    }
}

impl TransportDelegate {
    pub(crate) async fn start(binary: &DebugAdapterBinary, cx: &mut AsyncApp) -> Result<Self> {
        let log_handlers: LogHandlers = Default::default();
        let transport = start(binary, log_handlers.clone(), cx).await?;
        Ok(Self {
            transport: Mutex::new(transport),
            log_handlers,
            server_tx: Default::default(),
            pending_requests: Default::default(),
            tasks: Default::default(),
        })
    }

    pub async fn connect(
        &self,
        message_handler: DapMessageHandler,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let (server_tx, client_rx) = unbounded::<Message>();
        self.tasks.lock().clear();

        let log_dap_communications =
            cx.update(|cx| DebuggerSettings::get_global(cx).log_dap_communications)
                .with_context(|| "Failed to get Debugger Setting log dap communications error in transport::start_handlers. Defaulting to false")
                .unwrap_or(false);

        let connect = self.transport.lock().connect();
        let (input, output) = connect.await?;

        let log_handler = if log_dap_communications {
            Some(self.log_handlers.clone())
        } else {
            None
        };

        let pending_requests = self.pending_requests.clone();
        let output_log_handler = log_handler.clone();
        {
            let mut tasks = self.tasks.lock();
            tasks.push(cx.background_spawn(async move {
                match Self::recv_from_server(
                    output,
                    message_handler,
                    pending_requests.clone(),
                    output_log_handler,
                )
                .await
                {
                    Ok(()) => {
                        pending_requests.lock().drain().for_each(|(_, request)| {
                            request
                                .send(Err(anyhow!("debugger shutdown unexpectedly")))
                                .ok();
                        });
                    }
                    Err(e) => {
                        pending_requests.lock().drain().for_each(|(_, request)| {
                            request.send(Err(e.cloned())).ok();
                        });
                    }
                }
            }));

            tasks.push(cx.background_spawn(async move {
                match Self::send_to_server(input, client_rx, log_handler).await {
                    Ok(()) => {}
                    Err(e) => log::error!("Error handling debugger input: {e}"),
                }
            }));
        }

        {
            let mut lock = self.server_tx.lock().await;
            *lock = Some(server_tx.clone());
        }

        Ok(())
    }

    pub(crate) fn tcp_arguments(&self) -> Option<TcpArguments> {
        self.transport.lock().tcp_arguments()
    }

    pub(crate) fn add_pending_request(
        &self,
        sequence_id: u64,
        request: oneshot::Sender<Result<Response>>,
    ) {
        let mut pending_requests = self.pending_requests.lock();
        pending_requests.insert(sequence_id, request);
    }

    pub(crate) async fn send_message(&self, message: Message) -> Result<()> {
        if let Some(server_tx) = self.server_tx.lock().await.as_ref() {
            server_tx.send(message).await.context("sending message")
        } else {
            anyhow::bail!("Server tx already dropped")
        }
    }

    async fn handle_adapter_log(
        stdout: impl AsyncRead + Unpin + Send + 'static,
        iokind: IoKind,
        log_handlers: LogHandlers,
    ) {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        loop {
            line.truncate(0);

            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {}
                Err(e) => {
                    log::debug!("handle_adapter_log: {}", e);
                    break;
                }
            }

            for (kind, handler) in log_handlers.lock().iter_mut() {
                if matches!(kind, LogKind::Adapter) {
                    handler(iokind, None, line.as_str());
                }
            }
        }
    }

    fn build_rpc_message(message: String) -> String {
        format!("Content-Length: {}\r\n\r\n{}", message.len(), message)
    }

    async fn send_to_server<Stdin>(
        mut server_stdin: Stdin,
        client_rx: Receiver<Message>,
        log_handlers: Option<LogHandlers>,
    ) -> Result<()>
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
    {
        let result = loop {
            match client_rx.recv().await {
                Ok(message) => {
                    let command = match &message {
                        Message::Request(request) => Some(request.command.as_str()),
                        Message::Response(response) => Some(response.command.as_str()),
                        _ => None,
                    };

                    let message = match serde_json::to_string(&message) {
                        Ok(message) => message,
                        Err(e) => break Err(e.into()),
                    };

                    if let Some(log_handlers) = log_handlers.as_ref() {
                        for (kind, log_handler) in log_handlers.lock().iter_mut() {
                            if matches!(kind, LogKind::Rpc) {
                                log_handler(IoKind::StdIn, command, &message);
                            }
                        }
                    }

                    if let Err(e) = server_stdin
                        .write_all(Self::build_rpc_message(message).as_bytes())
                        .await
                    {
                        break Err(e.into());
                    }

                    if let Err(e) = server_stdin.flush().await {
                        break Err(e.into());
                    }
                }
                Err(error) => break Err(error.into()),
            }
        };

        log::debug!("Handle adapter input dropped");

        result
    }

    async fn recv_from_server<Stdout>(
        server_stdout: Stdout,
        mut message_handler: DapMessageHandler,
        pending_requests: Requests,
        log_handlers: Option<LogHandlers>,
    ) -> Result<()>
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        let mut recv_buffer = String::new();
        let mut reader = BufReader::new(server_stdout);

        let result = loop {
            match Self::receive_server_message(&mut reader, &mut recv_buffer, log_handlers.as_ref())
                .await
            {
                ConnectionResult::Timeout => anyhow::bail!("Timed out when connecting to debugger"),
                ConnectionResult::ConnectionReset => {
                    log::info!("Debugger closed the connection");
                    return Ok(());
                }
                ConnectionResult::Result(Ok(Message::Response(res))) => {
                    let tx = pending_requests.lock().remove(&res.request_seq);
                    if let Some(tx) = tx {
                        if let Err(e) = tx.send(Self::process_response(res)) {
                            log::trace!("Did not send response `{:?}` for a cancelled", e);
                        }
                    } else {
                        message_handler(Message::Response(res))
                    }
                }
                ConnectionResult::Result(Ok(message)) => message_handler(message),
                ConnectionResult::Result(Err(e)) => break Err(e),
            }
        };

        log::debug!("Handle adapter output dropped");

        result
    }

    fn process_response(response: Response) -> Result<Response> {
        if response.success {
            Ok(response)
        } else {
            if let Some(error_message) = response
                .body
                .clone()
                .and_then(|body| serde_json::from_value::<ErrorResponse>(body).ok())
                .and_then(|response| response.error.map(|msg| msg.format))
                .or_else(|| response.message.clone())
            {
                anyhow::bail!(error_message);
            };

            anyhow::bail!(
                "Received error response from adapter. Response: {:?}",
                response
            );
        }
    }

    async fn receive_server_message<Stdout>(
        reader: &mut BufReader<Stdout>,
        buffer: &mut String,
        log_handlers: Option<&LogHandlers>,
    ) -> ConnectionResult<Message>
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        let mut content_length = None;
        loop {
            buffer.truncate(0);
            match reader.read_line(buffer).await {
                Ok(0) => return ConnectionResult::ConnectionReset,
                Ok(_) => {}
                Err(e) => return ConnectionResult::Result(Err(e.into())),
            };

            if buffer == "\r\n" {
                break;
            }

            if let Some(("Content-Length", value)) = buffer.trim().split_once(": ") {
                match value.parse().context("invalid content length") {
                    Ok(length) => content_length = Some(length),
                    Err(e) => return ConnectionResult::Result(Err(e)),
                }
            }
        }

        let content_length = match content_length.context("missing content length") {
            Ok(length) => length,
            Err(e) => return ConnectionResult::Result(Err(e)),
        };

        let mut content = vec![0; content_length];
        if let Err(e) = reader
            .read_exact(&mut content)
            .await
            .with_context(|| "reading after a loop")
        {
            return ConnectionResult::Result(Err(e));
        }

        let message_str = match std::str::from_utf8(&content).context("invalid utf8 from server") {
            Ok(str) => str,
            Err(e) => return ConnectionResult::Result(Err(e)),
        };

        let message =
            serde_json::from_str::<Message>(message_str).context("deserializing server message");

        if let Some(log_handlers) = log_handlers {
            let command = match &message {
                Ok(Message::Request(request)) => Some(request.command.as_str()),
                Ok(Message::Response(response)) => Some(response.command.as_str()),
                _ => None,
            };

            for (kind, log_handler) in log_handlers.lock().iter_mut() {
                if matches!(kind, LogKind::Rpc) {
                    log_handler(IoKind::StdOut, command, message_str);
                }
            }
        }

        ConnectionResult::Result(message)
    }

    pub fn has_adapter_logs(&self) -> bool {
        self.transport.lock().has_adapter_logs()
    }

    pub fn add_log_handler<F>(&self, f: F, kind: LogKind)
    where
        F: 'static + Send + FnMut(IoKind, Option<&Command>, &IoMessage),
    {
        let mut log_handlers = self.log_handlers.lock();
        log_handlers.push((kind, Box::new(f)));
    }
}

pub struct TcpTransport {
    executor: BackgroundExecutor,
    pub port: u16,
    pub host: Ipv4Addr,
    pub timeout: u64,
    process: Arc<Mutex<Option<Child>>>,
    _stderr_task: Option<Task<()>>,
    _stdout_task: Option<Task<()>>,
}

impl TcpTransport {
    /// Get an open port to use with the tcp client when not supplied by debug config
    pub async fn port(host: &TcpArgumentsTemplate) -> Result<u16> {
        if let Some(port) = host.port {
            Ok(port)
        } else {
            Self::unused_port(host.host()).await
        }
    }

    pub async fn unused_port(host: Ipv4Addr) -> Result<u16> {
        Ok(TcpListener::bind(SocketAddrV4::new(host, 0))
            .await?
            .local_addr()?
            .port())
    }

    async fn start(
        binary: &DebugAdapterBinary,
        log_handlers: LogHandlers,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let connection_args = binary
            .connection
            .as_ref()
            .context("No connection arguments provided")?;

        let host = connection_args.host;
        let port = connection_args.port;

        let mut process = None;
        let mut stdout_task = None;
        let mut stderr_task = None;

        if let Some(command) = &binary.command {
            let mut command = util::command::new_std_command(&command);

            if let Some(cwd) = &binary.cwd {
                command.current_dir(cwd);
            }

            command.args(&binary.arguments);
            command.envs(&binary.envs);

            let mut p = Child::spawn(command, Stdio::null())
                .with_context(|| "failed to start debug adapter.")?;

            stdout_task = p.stdout.take().map(|stdout| {
                cx.background_executor()
                    .spawn(TransportDelegate::handle_adapter_log(
                        stdout,
                        IoKind::StdOut,
                        log_handlers.clone(),
                    ))
            });
            stderr_task = p.stderr.take().map(|stderr| {
                cx.background_executor()
                    .spawn(TransportDelegate::handle_adapter_log(
                        stderr,
                        IoKind::StdErr,
                        log_handlers,
                    ))
            });
            process = Some(p);
        };

        let timeout = connection_args.timeout.unwrap_or_else(|| {
            cx.update(|cx| DebuggerSettings::get_global(cx).timeout)
                .unwrap_or(20000u64)
        });

        log::info!(
            "Debug adapter has connected to TCP server {}:{}",
            host,
            port
        );

        let this = Self {
            executor: cx.background_executor().clone(),
            port,
            host,
            process: Arc::new(Mutex::new(process)),
            timeout,
            _stdout_task: stdout_task,
            _stderr_task: stderr_task,
        };

        Ok(this)
    }
}

impl Transport for TcpTransport {
    fn has_adapter_logs(&self) -> bool {
        true
    }

    fn kill(&mut self) {
        if let Some(process) = &mut *self.process.lock() {
            process.kill();
        }
    }

    fn tcp_arguments(&self) -> Option<TcpArguments> {
        Some(TcpArguments {
            host: self.host,
            port: self.port,
            timeout: Some(self.timeout),
        })
    }

    fn connect(
        &mut self,
    ) -> Task<
        Result<(
            Box<dyn AsyncWrite + Unpin + Send + 'static>,
            Box<dyn AsyncRead + Unpin + Send + 'static>,
        )>,
    > {
        let executor = self.executor.clone();
        let timeout = self.timeout;
        let address = SocketAddrV4::new(self.host, self.port);
        let process = self.process.clone();
        executor.clone().spawn(async move {
            select! {
                _ = executor.timer(Duration::from_millis(timeout)).fuse() => {
                    anyhow::bail!("Connection to TCP DAP timeout {address}");
                },
                result = executor.clone().spawn(async move {
                    loop {
                        match TcpStream::connect(address).await {
                            Ok(stream) => {
                                let (read, write) = stream.split();
                                return Ok((Box::new(write) as _, Box::new(read) as _))
                            },
                            Err(_) => {
                                let has_process = process.lock().is_some();
                                if has_process {
                                    let status = process.lock().as_mut().unwrap().try_status();
                                    if let Ok(Some(_)) = status {
                                        let process = process.lock().take().unwrap().into_inner();
                                        let output = process.output().await?;
                                        let output = if output.stderr.is_empty() {
                                            String::from_utf8_lossy(&output.stdout).to_string()
                                        } else {
                                            String::from_utf8_lossy(&output.stderr).to_string()
                                        };
                                        anyhow::bail!("{output}\nerror: process exited before debugger attached.");
                                    }
                                }

                                executor.timer(Duration::from_millis(100)).await;
                            }
                        }
                    }
                }).fuse() => result
            }
        })
    }
}

impl Drop for TcpTransport {
    fn drop(&mut self) {
        if let Some(mut p) = self.process.lock().take() {
            p.kill()
        }
    }
}

pub struct StdioTransport {
    process: Mutex<Option<Child>>,
    _stderr_task: Option<Task<()>>,
}

impl StdioTransport {
    // #[allow(dead_code, reason = "This is used in non test builds of Zed")]
    async fn start(
        binary: &DebugAdapterBinary,
        log_handlers: LogHandlers,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let Some(binary_command) = &binary.command else {
            bail!(
                "When using the `stdio` transport, the path to a debug adapter binary must be set by Zed."
            );
        };
        let mut command = util::command::new_std_command(&binary_command);

        if let Some(cwd) = &binary.cwd {
            command.current_dir(cwd);
        }

        command.args(&binary.arguments);
        command.envs(&binary.envs);

        let mut process = Child::spawn(command, Stdio::piped()).with_context(|| {
            format!(
                "failed to spawn command `{} {}`.",
                binary_command,
                binary.arguments.join(" ")
            )
        })?;

        let err_task = process.stderr.take().map(|stderr| {
            cx.background_spawn(TransportDelegate::handle_adapter_log(
                stderr,
                IoKind::StdErr,
                log_handlers,
            ))
        });

        let process = Mutex::new(Some(process));

        Ok(Self {
            process,
            _stderr_task: err_task,
        })
    }
}

impl Transport for StdioTransport {
    fn has_adapter_logs(&self) -> bool {
        false
    }

    fn kill(&mut self) {
        if let Some(process) = &mut *self.process.lock() {
            process.kill();
        }
    }

    fn connect(
        &mut self,
    ) -> Task<
        Result<(
            Box<dyn AsyncWrite + Unpin + Send + 'static>,
            Box<dyn AsyncRead + Unpin + Send + 'static>,
        )>,
    > {
        let result = util::maybe!({
            let mut guard = self.process.lock();
            let process = guard.as_mut().context("oops")?;
            Ok((
                Box::new(process.stdin.take().context("Cannot reconnect")?) as _,
                Box::new(process.stdout.take().context("Cannot reconnect")?) as _,
            ))
        });
        Task::ready(result)
    }

    fn tcp_arguments(&self) -> Option<TcpArguments> {
        None
    }
}

impl Drop for StdioTransport {
    fn drop(&mut self) {
        if let Some(process) = &mut *self.process.lock() {
            process.kill();
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
type RequestHandler =
    Box<dyn Send + FnMut(u64, serde_json::Value) -> dap_types::messages::Response>;

#[cfg(any(test, feature = "test-support"))]
type ResponseHandler = Box<dyn Send + Fn(Response)>;

#[cfg(any(test, feature = "test-support"))]
pub struct FakeTransport {
    // for sending fake response back from adapter side
    request_handlers: Arc<Mutex<HashMap<&'static str, RequestHandler>>>,
    // for reverse request responses
    response_handlers: Arc<Mutex<HashMap<&'static str, ResponseHandler>>>,

    stdin_writer: Option<PipeWriter>,
    stdout_reader: Option<PipeReader>,
    message_handler: Option<Task<Result<()>>>,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeTransport {
    pub fn on_request<R: dap_types::requests::Request, F>(&self, mut handler: F)
    where
        F: 'static + Send + FnMut(u64, R::Arguments) -> Result<R::Response, ErrorResponse>,
    {
        self.request_handlers.lock().insert(
            R::COMMAND,
            Box::new(move |seq, args| {
                let result = handler(seq, serde_json::from_value(args).unwrap());
                let response = match result {
                    Ok(response) => Response {
                        seq: seq + 1,
                        request_seq: seq,
                        success: true,
                        command: R::COMMAND.into(),
                        body: Some(serde_json::to_value(response).unwrap()),
                        message: None,
                    },
                    Err(response) => Response {
                        seq: seq + 1,
                        request_seq: seq,
                        success: false,
                        command: R::COMMAND.into(),
                        body: Some(serde_json::to_value(response).unwrap()),
                        message: None,
                    },
                };
                response
            }),
        );
    }

    pub fn on_response<R: dap_types::requests::Request, F>(&self, handler: F)
    where
        F: 'static + Send + Fn(Response),
    {
        self.response_handlers
            .lock()
            .insert(R::COMMAND, Box::new(handler));
    }

    async fn start(cx: &mut AsyncApp) -> Result<Self> {
        use dap_types::requests::{Request, RunInTerminal, StartDebugging};
        use serde_json::json;

        let (stdin_writer, stdin_reader) = async_pipe::pipe();
        let (stdout_writer, stdout_reader) = async_pipe::pipe();

        let mut this = Self {
            request_handlers: Arc::new(Mutex::new(HashMap::default())),
            response_handlers: Arc::new(Mutex::new(HashMap::default())),
            stdin_writer: Some(stdin_writer),
            stdout_reader: Some(stdout_reader),
            message_handler: None,
        };

        let request_handlers = this.request_handlers.clone();
        let response_handlers = this.response_handlers.clone();
        let stdout_writer = Arc::new(smol::lock::Mutex::new(stdout_writer));

        this.message_handler = Some(cx.background_spawn(async move {
            let mut reader = BufReader::new(stdin_reader);
            let mut buffer = String::new();

            loop {
                match TransportDelegate::receive_server_message(&mut reader, &mut buffer, None)
                    .await
                {
                    ConnectionResult::Timeout => {
                        anyhow::bail!("Timed out when connecting to debugger");
                    }
                    ConnectionResult::ConnectionReset => {
                        log::info!("Debugger closed the connection");
                        break Ok(());
                    }
                    ConnectionResult::Result(Err(e)) => break Err(e),
                    ConnectionResult::Result(Ok(message)) => {
                        match message {
                            Message::Request(request) => {
                                // redirect reverse requests to stdout writer/reader
                                if request.command == RunInTerminal::COMMAND
                                    || request.command == StartDebugging::COMMAND
                                {
                                    let message =
                                        serde_json::to_string(&Message::Request(request)).unwrap();

                                    let mut writer = stdout_writer.lock().await;
                                    writer
                                        .write_all(
                                            TransportDelegate::build_rpc_message(message)
                                                .as_bytes(),
                                        )
                                        .await
                                        .unwrap();
                                    writer.flush().await.unwrap();
                                } else {
                                    let response = if let Some(handle) =
                                        request_handlers.lock().get_mut(request.command.as_str())
                                    {
                                        handle(request.seq, request.arguments.unwrap_or(json!({})))
                                    } else {
                                        panic!("No request handler for {}", request.command);
                                    };
                                    let message =
                                        serde_json::to_string(&Message::Response(response))
                                            .unwrap();

                                    let mut writer = stdout_writer.lock().await;
                                    writer
                                        .write_all(
                                            TransportDelegate::build_rpc_message(message)
                                                .as_bytes(),
                                        )
                                        .await
                                        .unwrap();
                                    writer.flush().await.unwrap();
                                }
                            }
                            Message::Event(event) => {
                                let message =
                                    serde_json::to_string(&Message::Event(event)).unwrap();

                                let mut writer = stdout_writer.lock().await;
                                writer
                                    .write_all(
                                        TransportDelegate::build_rpc_message(message).as_bytes(),
                                    )
                                    .await
                                    .unwrap();
                                writer.flush().await.unwrap();
                            }
                            Message::Response(response) => {
                                if let Some(handle) =
                                    response_handlers.lock().get(response.command.as_str())
                                {
                                    handle(response);
                                } else {
                                    log::error!("No response handler for {}", response.command);
                                }
                            }
                        }
                    }
                }
            }
        }));

        Ok(this)
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Transport for FakeTransport {
    fn tcp_arguments(&self) -> Option<TcpArguments> {
        None
    }

    fn connect(
        &mut self,
    ) -> Task<
        Result<(
            Box<dyn AsyncWrite + Unpin + Send + 'static>,
            Box<dyn AsyncRead + Unpin + Send + 'static>,
        )>,
    > {
        let result = util::maybe!({
            Ok((
                Box::new(self.stdin_writer.take().context("Cannot reconnect")?) as _,
                Box::new(self.stdout_reader.take().context("Cannot reconnect")?) as _,
            ))
        });
        Task::ready(result)
    }

    fn has_adapter_logs(&self) -> bool {
        false
    }

    fn kill(&mut self) {
        self.message_handler.take();
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &FakeTransport {
        self
    }
}

struct Child {
    process: smol::process::Child,
}

impl std::ops::Deref for Child {
    type Target = smol::process::Child;

    fn deref(&self) -> &Self::Target {
        &self.process
    }
}

impl std::ops::DerefMut for Child {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.process
    }
}

impl Child {
    fn into_inner(self) -> smol::process::Child {
        self.process
    }

    #[cfg(not(windows))]
    fn spawn(mut command: std::process::Command, stdin: Stdio) -> Result<Self> {
        util::set_pre_exec_to_start_new_session(&mut command);
        let process = smol::process::Command::from(command)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        Ok(Self { process })
    }

    #[cfg(windows)]
    fn spawn(command: std::process::Command, stdin: Stdio) -> Result<Self> {
        // TODO(windows): create a job object and add the child process handle to it,
        // see https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects
        let process = smol::process::Command::from(command)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        Ok(Self { process })
    }

    #[cfg(not(windows))]
    fn kill(&mut self) {
        let pid = self.process.id();
        unsafe {
            libc::killpg(pid as i32, libc::SIGKILL);
        }
    }

    #[cfg(windows)]
    fn kill(&mut self) {
        // TODO(windows): terminate the job object in kill
        let _ = self.process.kill();
    }
}
