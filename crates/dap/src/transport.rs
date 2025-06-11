use anyhow::{Context as _, Result, bail};
use dap_types::{
    ErrorResponse,
    messages::{Message, Response},
};
use futures::{AsyncRead, AsyncReadExt as _, AsyncWrite, FutureExt as _, channel::oneshot, select};
use gpui::{AppContext as _, AsyncApp, Task};
use settings::Settings as _;
use smallvec::SmallVec;
use smol::{
    channel::{Receiver, Sender, unbounded},
    io::{AsyncBufReadExt as _, AsyncWriteExt, BufReader},
    lock::Mutex,
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

use crate::{adapters::DebugAdapterBinary, debugger_settings::DebuggerSettings};

pub type IoHandler = Box<dyn Send + FnMut(IoKind, &str)>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LogKind {
    Adapter,
    Rpc,
}

pub enum IoKind {
    StdIn,
    StdOut,
    StdErr,
}

pub struct TransportPipe {
    input: Box<dyn AsyncWrite + Unpin + Send + 'static>,
    output: Box<dyn AsyncRead + Unpin + Send + 'static>,
    stdout: Option<Box<dyn AsyncRead + Unpin + Send + 'static>>,
    stderr: Option<Box<dyn AsyncRead + Unpin + Send + 'static>>,
}

impl TransportPipe {
    pub fn new(
        input: Box<dyn AsyncWrite + Unpin + Send + 'static>,
        output: Box<dyn AsyncRead + Unpin + Send + 'static>,
        stdout: Option<Box<dyn AsyncRead + Unpin + Send + 'static>>,
        stderr: Option<Box<dyn AsyncRead + Unpin + Send + 'static>>,
    ) -> Self {
        TransportPipe {
            input,
            output,
            stdout,
            stderr,
        }
    }
}

type Requests = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Response>>>>>;
type LogHandlers = Arc<parking_lot::Mutex<SmallVec<[(LogKind, IoHandler); 2]>>>;

pub enum Transport {
    Stdio(StdioTransport),
    Tcp(TcpTransport),
    #[cfg(any(test, feature = "test-support"))]
    Fake(FakeTransport),
}

impl Transport {
    async fn start(binary: &DebugAdapterBinary, cx: AsyncApp) -> Result<(TransportPipe, Self)> {
        #[cfg(any(test, feature = "test-support"))]
        if cfg!(any(test, feature = "test-support")) {
            return FakeTransport::start(cx)
                .await
                .map(|(transports, fake)| (transports, Self::Fake(fake)));
        }

        if binary.connection.is_some() {
            TcpTransport::start(binary, cx)
                .await
                .map(|(transports, tcp)| (transports, Self::Tcp(tcp)))
        } else {
            StdioTransport::start(binary, cx)
                .await
                .map(|(transports, stdio)| (transports, Self::Stdio(stdio)))
        }
    }

    fn has_adapter_logs(&self) -> bool {
        match self {
            Transport::Stdio(stdio_transport) => stdio_transport.has_adapter_logs(),
            Transport::Tcp(tcp_transport) => tcp_transport.has_adapter_logs(),
            #[cfg(any(test, feature = "test-support"))]
            Transport::Fake(fake_transport) => fake_transport.has_adapter_logs(),
        }
    }

    async fn kill(&self) {
        match self {
            Transport::Stdio(stdio_transport) => stdio_transport.kill().await,
            Transport::Tcp(tcp_transport) => tcp_transport.kill().await,
            #[cfg(any(test, feature = "test-support"))]
            Transport::Fake(fake_transport) => fake_transport.kill().await,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn as_fake(&self) -> &FakeTransport {
        match self {
            Transport::Fake(fake_transport) => fake_transport,
            _ => panic!("Not a fake transport layer"),
        }
    }
}

pub(crate) struct TransportDelegate {
    log_handlers: LogHandlers,
    current_requests: Requests,
    pending_requests: Requests,
    transport: Transport,
    server_tx: Arc<Mutex<Option<Sender<Message>>>>,
    _tasks: Vec<Task<()>>,
}

impl TransportDelegate {
    pub(crate) async fn start(
        binary: &DebugAdapterBinary,
        cx: AsyncApp,
    ) -> Result<((Receiver<Message>, Sender<Message>), Self)> {
        let (transport_pipes, transport) = Transport::start(binary, cx.clone()).await?;
        let mut this = Self {
            transport,
            server_tx: Default::default(),
            log_handlers: Default::default(),
            current_requests: Default::default(),
            pending_requests: Default::default(),
            _tasks: Vec::new(),
        };
        let messages = this.start_handlers(transport_pipes, cx).await?;
        Ok((messages, this))
    }

    async fn start_handlers(
        &mut self,
        mut params: TransportPipe,
        cx: AsyncApp,
    ) -> Result<(Receiver<Message>, Sender<Message>)> {
        let (client_tx, server_rx) = unbounded::<Message>();
        let (server_tx, client_rx) = unbounded::<Message>();

        let log_dap_communications =
            cx.update(|cx| DebuggerSettings::get_global(cx).log_dap_communications)
                .with_context(|| "Failed to get Debugger Setting log dap communications error in transport::start_handlers. Defaulting to false")
                .unwrap_or(false);

        let log_handler = if log_dap_communications {
            Some(self.log_handlers.clone())
        } else {
            None
        };

        let adapter_log_handler = log_handler.clone();
        cx.update(|cx| {
            if let Some(stdout) = params.stdout.take() {
                self._tasks.push(cx.background_spawn(async move {
                    match Self::handle_adapter_log(stdout, adapter_log_handler).await {
                        ConnectionResult::Timeout => {
                            log::error!("Timed out when handling debugger log");
                        }
                        ConnectionResult::ConnectionReset => {
                            log::info!("Debugger logs connection closed");
                        }
                        ConnectionResult::Result(Ok(())) => {}
                        ConnectionResult::Result(Err(e)) => {
                            log::error!("Error handling debugger log: {e}");
                        }
                    }
                }));
            }

            let pending_requests = self.pending_requests.clone();
            let output_log_handler = log_handler.clone();
            self._tasks.push(cx.background_spawn(async move {
                match Self::handle_output(
                    params.output,
                    client_tx,
                    pending_requests,
                    output_log_handler,
                )
                .await
                {
                    Ok(()) => {}
                    Err(e) => log::error!("Error handling debugger output: {e}"),
                }
            }));

            if let Some(stderr) = params.stderr.take() {
                let log_handlers = self.log_handlers.clone();
                self._tasks.push(cx.background_spawn(async move {
                    match Self::handle_error(stderr, log_handlers).await {
                        ConnectionResult::Timeout => {
                            log::error!("Timed out reading debugger error stream")
                        }
                        ConnectionResult::ConnectionReset => {
                            log::info!("Debugger closed its error stream")
                        }
                        ConnectionResult::Result(Ok(())) => {}
                        ConnectionResult::Result(Err(e)) => {
                            log::error!("Error handling debugger error: {e}")
                        }
                    }
                }));
            }

            let current_requests = self.current_requests.clone();
            let pending_requests = self.pending_requests.clone();
            let log_handler = log_handler.clone();
            self._tasks.push(cx.background_spawn(async move {
                match Self::handle_input(
                    params.input,
                    client_rx,
                    current_requests,
                    pending_requests,
                    log_handler,
                )
                .await
                {
                    Ok(()) => {}
                    Err(e) => log::error!("Error handling debugger input: {e}"),
                }
            }));
        })?;

        {
            let mut lock = self.server_tx.lock().await;
            *lock = Some(server_tx.clone());
        }

        Ok((server_rx, server_tx))
    }

    pub(crate) async fn add_pending_request(
        &self,
        sequence_id: u64,
        request: oneshot::Sender<Result<Response>>,
    ) {
        let mut pending_requests = self.pending_requests.lock().await;
        pending_requests.insert(sequence_id, request);
    }

    pub(crate) async fn send_message(&self, message: Message) -> Result<()> {
        if let Some(server_tx) = self.server_tx.lock().await.as_ref() {
            server_tx.send(message).await.context("sending message")
        } else {
            anyhow::bail!("Server tx already dropped")
        }
    }

    async fn handle_adapter_log<Stdout>(
        stdout: Stdout,
        log_handlers: Option<LogHandlers>,
    ) -> ConnectionResult<()>
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        let result = loop {
            line.truncate(0);

            match reader
                .read_line(&mut line)
                .await
                .context("reading adapter log line")
            {
                Ok(0) => break ConnectionResult::ConnectionReset,
                Ok(_) => {}
                Err(e) => break ConnectionResult::Result(Err(e)),
            }

            if let Some(log_handlers) = log_handlers.as_ref() {
                for (kind, handler) in log_handlers.lock().iter_mut() {
                    if matches!(kind, LogKind::Adapter) {
                        handler(IoKind::StdOut, line.as_str());
                    }
                }
            }
        };

        log::debug!("Handle adapter log dropped");

        result
    }

    fn build_rpc_message(message: String) -> String {
        format!("Content-Length: {}\r\n\r\n{}", message.len(), message)
    }

    async fn handle_input<Stdin>(
        mut server_stdin: Stdin,
        client_rx: Receiver<Message>,
        current_requests: Requests,
        pending_requests: Requests,
        log_handlers: Option<LogHandlers>,
    ) -> Result<()>
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
    {
        let result = loop {
            match client_rx.recv().await {
                Ok(message) => {
                    if let Message::Request(request) = &message {
                        if let Some(sender) = current_requests.lock().await.remove(&request.seq) {
                            pending_requests.lock().await.insert(request.seq, sender);
                        }
                    }

                    let message = match serde_json::to_string(&message) {
                        Ok(message) => message,
                        Err(e) => break Err(e.into()),
                    };

                    if let Some(log_handlers) = log_handlers.as_ref() {
                        for (kind, log_handler) in log_handlers.lock().iter_mut() {
                            if matches!(kind, LogKind::Rpc) {
                                log_handler(IoKind::StdIn, &message);
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

    async fn handle_output<Stdout>(
        server_stdout: Stdout,
        client_tx: Sender<Message>,
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
                    if let Some(tx) = pending_requests.lock().await.remove(&res.request_seq) {
                        if let Err(e) = tx.send(Self::process_response(res)) {
                            log::trace!("Did not send response `{:?}` for a cancelled", e);
                        }
                    } else {
                        client_tx.send(Message::Response(res)).await?;
                    }
                }
                ConnectionResult::Result(Ok(message)) => client_tx.send(message).await?,
                ConnectionResult::Result(Err(e)) => break Err(e),
            }
        };

        drop(client_tx);
        log::debug!("Handle adapter output dropped");

        result
    }

    async fn handle_error<Stderr>(stderr: Stderr, log_handlers: LogHandlers) -> ConnectionResult<()>
    where
        Stderr: AsyncRead + Unpin + Send + 'static,
    {
        log::debug!("Handle error started");
        let mut buffer = String::new();

        let mut reader = BufReader::new(stderr);

        let result = loop {
            match reader
                .read_line(&mut buffer)
                .await
                .context("reading error log line")
            {
                Ok(0) => break ConnectionResult::ConnectionReset,
                Ok(_) => {
                    for (kind, log_handler) in log_handlers.lock().iter_mut() {
                        if matches!(kind, LogKind::Adapter) {
                            log_handler(IoKind::StdErr, buffer.as_str());
                        }
                    }

                    buffer.truncate(0);
                }
                Err(error) => break ConnectionResult::Result(Err(error)),
            }
        };

        log::debug!("Handle adapter error dropped");

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

            match reader
                .read_line(buffer)
                .await
                .with_context(|| "reading a message from server")
            {
                Ok(0) => return ConnectionResult::ConnectionReset,
                Ok(_) => {}
                Err(e) => return ConnectionResult::Result(Err(e)),
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

        if let Some(log_handlers) = log_handlers {
            for (kind, log_handler) in log_handlers.lock().iter_mut() {
                if matches!(kind, LogKind::Rpc) {
                    log_handler(IoKind::StdOut, message_str);
                }
            }
        }

        ConnectionResult::Result(
            serde_json::from_str::<Message>(message_str).context("deserializing server message"),
        )
    }

    pub async fn shutdown(&self) -> Result<()> {
        log::debug!("Start shutdown client");

        if let Some(server_tx) = self.server_tx.lock().await.take().as_ref() {
            server_tx.close();
        }

        let mut current_requests = self.current_requests.lock().await;
        let mut pending_requests = self.pending_requests.lock().await;

        current_requests.clear();
        pending_requests.clear();

        self.transport.kill().await;

        drop(current_requests);
        drop(pending_requests);

        log::debug!("Shutdown client completed");

        anyhow::Ok(())
    }

    pub fn has_adapter_logs(&self) -> bool {
        self.transport.has_adapter_logs()
    }

    pub fn transport(&self) -> &Transport {
        &self.transport
    }

    pub fn add_log_handler<F>(&self, f: F, kind: LogKind)
    where
        F: 'static + Send + FnMut(IoKind, &str),
    {
        let mut log_handlers = self.log_handlers.lock();
        log_handlers.push((kind, Box::new(f)));
    }
}

pub struct TcpTransport {
    pub port: u16,
    pub host: Ipv4Addr,
    pub timeout: u64,
    process: Mutex<Child>,
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

    async fn start(binary: &DebugAdapterBinary, cx: AsyncApp) -> Result<(TransportPipe, Self)> {
        let connection_args = binary
            .connection
            .as_ref()
            .context("No connection arguments provided")?;

        let host = connection_args.host;
        let port = connection_args.port;

        let mut command = util::command::new_std_command(&binary.command);

        if let Some(cwd) = &binary.cwd {
            command.current_dir(cwd);
        }

        command.args(&binary.arguments);
        command.envs(&binary.envs);

        let mut process = Child::spawn(command, Stdio::null())
            .with_context(|| "failed to start debug adapter.")?;

        let address = SocketAddrV4::new(host, port);

        let timeout = connection_args.timeout.unwrap_or_else(|| {
            cx.update(|cx| DebuggerSettings::get_global(cx).timeout)
                .unwrap_or(2000u64)
        });

        let (mut process, (rx, tx)) = select! {
            _ = cx.background_executor().timer(Duration::from_millis(timeout)).fuse() => {
                anyhow::bail!("Connection to TCP DAP timeout {host}:{port}");
            },
            result = cx.spawn(async move |cx| {
                loop {
                    match TcpStream::connect(address).await {
                        Ok(stream) => return Ok((process, stream.split())),
                        Err(_) => {
                            if let Ok(Some(_)) = process.try_status() {
                                let output = process.into_inner().output().await?;
                                let output = if output.stderr.is_empty() {
                                    String::from_utf8_lossy(&output.stdout).to_string()
                                } else {
                                    String::from_utf8_lossy(&output.stderr).to_string()
                                };
                                anyhow::bail!("{output}\nerror: process exited before debugger attached.");
                            }
                            cx.background_executor().timer(Duration::from_millis(100)).await;
                        }
                    }
                }
            }).fuse() => result?
        };

        log::info!(
            "Debug adapter has connected to TCP server {}:{}",
            host,
            port
        );
        let stdout = process.stdout.take();
        let stderr = process.stderr.take();

        let this = Self {
            port,
            host,
            process: Mutex::new(process),
            timeout,
        };

        let pipe = TransportPipe::new(
            Box::new(tx),
            Box::new(BufReader::new(rx)),
            stdout.map(|s| Box::new(s) as Box<dyn AsyncRead + Unpin + Send>),
            stderr.map(|s| Box::new(s) as Box<dyn AsyncRead + Unpin + Send>),
        );

        Ok((pipe, this))
    }

    fn has_adapter_logs(&self) -> bool {
        true
    }

    async fn kill(&self) {
        let mut process = self.process.lock().await;
        Child::kill(&mut process);
    }
}

impl Drop for TcpTransport {
    fn drop(&mut self) {
        self.process.get_mut().kill();
    }
}

pub struct StdioTransport {
    process: Mutex<Child>,
}

impl StdioTransport {
    #[allow(dead_code, reason = "This is used in non test builds of Zed")]
    async fn start(binary: &DebugAdapterBinary, _: AsyncApp) -> Result<(TransportPipe, Self)> {
        let mut command = util::command::new_std_command(&binary.command);

        if let Some(cwd) = &binary.cwd {
            command.current_dir(cwd);
        }

        command.args(&binary.arguments);
        command.envs(&binary.envs);

        let mut process = Child::spawn(command, Stdio::piped()).with_context(|| {
            format!(
                "failed to spawn command `{} {}`.",
                binary.command,
                binary.arguments.join(" ")
            )
        })?;

        let stdin = process.stdin.take().context("Failed to open stdin")?;
        let stdout = process.stdout.take().context("Failed to open stdout")?;
        let stderr = process
            .stderr
            .take()
            .map(|io_err| Box::new(io_err) as Box<dyn AsyncRead + Unpin + Send>);

        if stderr.is_none() {
            bail!(
                "Failed to connect to stderr for debug adapter command {}",
                &binary.command
            );
        }

        log::info!("Debug adapter has connected to stdio adapter");

        let process = Mutex::new(process);

        Ok((
            TransportPipe::new(
                Box::new(stdin),
                Box::new(BufReader::new(stdout)),
                None,
                stderr,
            ),
            Self { process },
        ))
    }

    fn has_adapter_logs(&self) -> bool {
        false
    }

    async fn kill(&self) {
        let mut process = self.process.lock().await;
        Child::kill(&mut process);
    }
}

impl Drop for StdioTransport {
    fn drop(&mut self) {
        self.process.get_mut().kill();
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
    request_handlers: Arc<parking_lot::Mutex<HashMap<&'static str, RequestHandler>>>,
    // for reverse request responses
    response_handlers: Arc<parking_lot::Mutex<HashMap<&'static str, ResponseHandler>>>,
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

    pub async fn on_response<R: dap_types::requests::Request, F>(&self, handler: F)
    where
        F: 'static + Send + Fn(Response),
    {
        self.response_handlers
            .lock()
            .insert(R::COMMAND, Box::new(handler));
    }

    async fn start(cx: AsyncApp) -> Result<(TransportPipe, Self)> {
        let this = Self {
            request_handlers: Arc::new(parking_lot::Mutex::new(HashMap::default())),
            response_handlers: Arc::new(parking_lot::Mutex::new(HashMap::default())),
        };
        use dap_types::requests::{Request, RunInTerminal, StartDebugging};
        use serde_json::json;

        let (stdin_writer, stdin_reader) = async_pipe::pipe();
        let (stdout_writer, stdout_reader) = async_pipe::pipe();

        let request_handlers = this.request_handlers.clone();
        let response_handlers = this.response_handlers.clone();
        let stdout_writer = Arc::new(Mutex::new(stdout_writer));

        cx.background_spawn(async move {
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
        })
        .detach();

        Ok((
            TransportPipe::new(Box::new(stdin_writer), Box::new(stdout_reader), None, None),
            this,
        ))
    }

    fn has_adapter_logs(&self) -> bool {
        false
    }

    async fn kill(&self) {}
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
    fn spawn(mut command: smol::process::Command, stdin: Stdio) -> Result<Self> {
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
    fn kill(&self) {
        // TODO(windows): terminate the job object in kill
        self.process.kill();
    }
}
