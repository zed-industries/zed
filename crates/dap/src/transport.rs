use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use dap_types::{
    messages::{Message, Response},
    ErrorResponse,
};
use futures::{
    channel::oneshot, select, AsyncBufRead, AsyncReadExt as _, AsyncWrite, FutureExt as _,
};
use gpui::AsyncAppContext;
use settings::Settings as _;
use smallvec::SmallVec;
use smol::{
    channel::{unbounded, Receiver, Sender},
    io::{AsyncBufReadExt as _, AsyncWriteExt, BufReader},
    lock::Mutex,
    net::{TcpListener, TcpStream},
    process::{self, Child, ChildStderr, ChildStdout},
};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    process::Stdio,
    sync::Arc,
    time::Duration,
};
use task::TCPHost;
use util::ResultExt as _;

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

pub struct TransportParams {
    input: Box<dyn AsyncWrite + Unpin + Send>,
    output: Box<dyn AsyncBufRead + Unpin + Send>,
    process: Child,
}

impl TransportParams {
    pub fn new(
        input: Box<dyn AsyncWrite + Unpin + Send>,
        output: Box<dyn AsyncBufRead + Unpin + Send>,
        process: Child,
    ) -> Self {
        TransportParams {
            input,
            output,
            process,
        }
    }
}

type Requests = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Response>>>>>;
type LogHandlers = Arc<parking_lot::Mutex<SmallVec<[(LogKind, IoHandler); 2]>>>;

pub(crate) struct TransportDelegate {
    log_handlers: LogHandlers,
    current_requests: Requests,
    pending_requests: Requests,
    transport: Box<dyn Transport>,
    process: Arc<Mutex<Option<Child>>>,
    server_tx: Arc<Mutex<Option<Sender<Message>>>>,
}

impl TransportDelegate {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        Self {
            transport,
            process: Default::default(),
            server_tx: Default::default(),
            log_handlers: Default::default(),
            current_requests: Default::default(),
            pending_requests: Default::default(),
        }
    }

    pub(crate) async fn start(
        &mut self,
        binary: &DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> Result<(Receiver<Message>, Sender<Message>)> {
        let mut params = self.transport.start(binary, cx).await?;

        let (client_tx, server_rx) = unbounded::<Message>();
        let (server_tx, client_rx) = unbounded::<Message>();

        cx.update(|cx| {
            if let Some(stdout) = params.process.stdout.take() {
                cx.background_executor()
                    .spawn(Self::handle_adapter_log(stdout, self.log_handlers.clone()))
                    .detach_and_log_err(cx);
            }

            cx.background_executor()
                .spawn(Self::handle_output(
                    params.output,
                    client_tx,
                    self.pending_requests.clone(),
                    self.log_handlers.clone(),
                ))
                .detach_and_log_err(cx);

            if let Some(stderr) = params.process.stderr.take() {
                cx.background_executor()
                    .spawn(Self::handle_error(stderr, self.log_handlers.clone()))
                    .detach_and_log_err(cx);
            }

            cx.background_executor()
                .spawn(Self::handle_input(
                    params.input,
                    client_rx,
                    self.current_requests.clone(),
                    self.pending_requests.clone(),
                    self.log_handlers.clone(),
                ))
                .detach_and_log_err(cx);
        })?;

        {
            let mut lock = self.process.lock().await;
            *lock = Some(params.process);

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

    pub(crate) async fn cancel_pending_request(&self, sequence_id: &u64) {
        let mut pending_requests = self.pending_requests.lock().await;
        pending_requests.remove(sequence_id);
    }

    pub(crate) async fn send_message(&self, message: Message) -> Result<()> {
        if let Some(server_tx) = self.server_tx.lock().await.as_ref() {
            server_tx
                .send(message)
                .await
                .map_err(|e| anyhow!("Failed to send message: {}", e))
        } else {
            Err(anyhow!("Server tx already dropped"))
        }
    }

    async fn handle_adapter_log(stdout: ChildStdout, log_handlers: LogHandlers) -> Result<()> {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        let result = loop {
            line.truncate(0);

            let bytes_read = match reader.read_line(&mut line).await {
                Ok(bytes_read) => bytes_read,
                Err(e) => break Err(e.into()),
            };

            if bytes_read == 0 {
                break Err(anyhow!("Debugger log stream closed"));
            }

            for (kind, handler) in log_handlers.lock().iter_mut() {
                if matches!(kind, LogKind::Adapter) {
                    handler(IoKind::StdOut, line.as_str());
                }
            }
        };

        log::debug!("Handle adapter log dropped");

        result
    }

    async fn handle_input(
        mut server_stdin: Box<dyn AsyncWrite + Unpin + Send>,
        client_rx: Receiver<Message>,
        current_requests: Requests,
        pending_requests: Requests,
        log_handlers: LogHandlers,
    ) -> Result<()> {
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

                    for (kind, log_handler) in log_handlers.lock().iter_mut() {
                        if matches!(kind, LogKind::Rpc) {
                            log_handler(IoKind::StdIn, &message);
                        }
                    }

                    if let Err(e) = server_stdin
                        .write_all(
                            format!("Content-Length: {}\r\n\r\n{}", message.len(), message)
                                .as_bytes(),
                        )
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

    async fn handle_output(
        mut server_stdout: Box<dyn AsyncBufRead + Unpin + Send>,
        client_tx: Sender<Message>,
        pending_requests: Requests,
        log_handlers: LogHandlers,
    ) -> Result<()> {
        let mut recv_buffer = String::new();

        let result = loop {
            let message =
                Self::receive_server_message(&mut server_stdout, &mut recv_buffer, &log_handlers)
                    .await;

            match message {
                Ok(Message::Response(res)) => {
                    if let Some(tx) = pending_requests.lock().await.remove(&res.request_seq) {
                        if let Err(e) = tx.send(Self::process_response(res)) {
                            break Err(anyhow!("Failed to send response: {:?}", e));
                        }
                    } else {
                        client_tx.send(Message::Response(res)).await?;
                    };
                }
                Ok(message) => {
                    client_tx.send(message).await?;
                }
                Err(e) => break Err(e),
            }
        };

        drop(client_tx);

        log::debug!("Handle adapter output dropped");

        result
    }

    async fn handle_error(stderr: ChildStderr, log_handlers: LogHandlers) -> Result<()> {
        let mut buffer = String::new();

        let mut reader = BufReader::new(stderr);

        let result = loop {
            match reader.read_line(&mut buffer).await {
                Ok(0) => break Err(anyhow!("debugger error stream closed")),
                Ok(_) => {
                    for (kind, log_handler) in log_handlers.lock().iter_mut() {
                        if matches!(kind, LogKind::Adapter) {
                            log_handler(IoKind::StdErr, buffer.as_str());
                        }
                    }

                    buffer.truncate(0);
                }
                Err(error) => break Err(error.into()),
            }
        };

        log::debug!("Handle adapter error dropped");

        result
    }

    fn process_response(response: Response) -> Result<Response> {
        if response.success {
            Ok(response)
        } else {
            if let Some(body) = response.body {
                if let Ok(error) = serde_json::from_value::<ErrorResponse>(body) {
                    if let Some(message) = error.error {
                        return Err(anyhow!(message.format));
                    };
                };
            }

            Err(anyhow!("Received error response from adapter"))
        }
    }

    async fn receive_server_message(
        reader: &mut Box<dyn AsyncBufRead + Unpin + Send>,
        buffer: &mut String,
        log_handlers: &LogHandlers,
    ) -> Result<Message> {
        let mut content_length = None;
        loop {
            buffer.truncate(0);

            if reader
                .read_line(buffer)
                .await
                .with_context(|| "reading a message from server")?
                == 0
            {
                return Err(anyhow!("debugger reader stream closed"));
            };

            if buffer == "\r\n" {
                break;
            }

            let parts = buffer.trim().split_once(": ");

            match parts {
                Some(("Content-Length", value)) => {
                    content_length = Some(value.parse().context("invalid content length")?);
                }
                _ => {}
            }
        }

        let content_length = content_length.context("missing content length")?;

        let mut content = vec![0; content_length];
        reader
            .read_exact(&mut content)
            .await
            .with_context(|| "reading after a loop")?;

        let message = std::str::from_utf8(&content).context("invalid utf8 from server")?;

        for (kind, log_handler) in log_handlers.lock().iter_mut() {
            if matches!(kind, LogKind::Rpc) {
                log_handler(IoKind::StdOut, &message);
            }
        }

        Ok(serde_json::from_str::<Message>(message)?)
    }

    pub async fn shutdown(&self) -> Result<()> {
        log::debug!("Start shutdown client");

        if let Some(server_tx) = self.server_tx.lock().await.take().as_ref() {
            server_tx.close();
        }

        let mut adapter = self.process.lock().await.take();
        let mut current_requests = self.current_requests.lock().await;
        let mut pending_requests = self.pending_requests.lock().await;

        current_requests.clear();
        pending_requests.clear();

        if let Some(mut adapter) = adapter.take() {
            let _ = adapter.kill().log_err();
        }

        drop(current_requests);
        drop(pending_requests);
        drop(adapter);

        log::debug!("Shutdown client completed");

        anyhow::Ok(())
    }

    pub fn has_adapter_logs(&self) -> bool {
        self.transport.has_adapter_logs()
    }

    pub fn add_log_handler<F>(&self, f: F, kind: LogKind)
    where
        F: 'static + Send + FnMut(IoKind, &str),
    {
        let mut log_handlers = self.log_handlers.lock();
        log_handlers.push((kind, Box::new(f)));
    }
}

#[async_trait(?Send)]
pub trait Transport: 'static + Send + Sync {
    async fn start(
        &mut self,
        binary: &DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> Result<TransportParams>;

    fn has_adapter_logs(&self) -> bool;

    fn clone_box(&self) -> Box<dyn Transport>;
}

#[derive(Clone)]
pub struct TcpTransport {
    port: u16,
    host: Ipv4Addr,
    timeout: Option<u64>,
}

impl TcpTransport {
    pub fn new(host: Ipv4Addr, port: u16, timeout: Option<u64>) -> Self {
        Self {
            port,
            host,
            timeout,
        }
    }

    /// Get an open port to use with the tcp client when not supplied by debug config
    pub async fn port(host: &TCPHost) -> Result<u16> {
        if let Some(port) = host.port {
            Ok(port)
        } else {
            Ok(TcpListener::bind(SocketAddrV4::new(host.host(), 0))
                .await?
                .local_addr()?
                .port())
        }
    }
}

#[async_trait(?Send)]
impl Transport for TcpTransport {
    async fn start(
        &mut self,
        binary: &DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        let mut command = process::Command::new(&binary.command);

        if let Some(cwd) = &binary.cwd {
            command.current_dir(cwd);
        }

        if let Some(args) = &binary.arguments {
            command.args(args);
        }

        if let Some(envs) = &binary.envs {
            command.envs(envs);
        }

        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let process = command
            .spawn()
            .with_context(|| "failed to start debug adapter.")?;

        let address = SocketAddrV4::new(self.host, self.port);

        let timeout = self.timeout.unwrap_or_else(|| {
            cx.update(|cx| DebuggerSettings::get_global(cx).timeout)
                .unwrap_or(2000u64)
        });

        let (rx, tx) = select! {
            _ = cx.background_executor().timer(Duration::from_millis(timeout)).fuse() => {
                return Err(anyhow!(format!("Connection to TCP DAP timeout {}:{}", self.host, self.port)))
            },
            result = cx.spawn(|cx| async move {
                loop {
                    match TcpStream::connect(address).await {
                        Ok(stream) => return stream.split(),
                        Err(_) => {
                            cx.background_executor().timer(Duration::from_millis(100)).await;
                        }
                    }
                }
            }).fuse() => result
        };
        log::info!(
            "Debug adapter has connected to TCP server {}:{}",
            self.host,
            self.port
        );

        Ok(TransportParams::new(
            Box::new(tx),
            Box::new(BufReader::new(rx)),
            process,
        ))
    }

    fn has_adapter_logs(&self) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Transport> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct StdioTransport {}

impl StdioTransport {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl Transport for StdioTransport {
    async fn start(
        &mut self,
        binary: &DebugAdapterBinary,
        _: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        let mut command = process::Command::new(&binary.command);

        if let Some(cwd) = &binary.cwd {
            command.current_dir(cwd);
        }

        if let Some(args) = &binary.arguments {
            command.args(args);
        }

        if let Some(envs) = &binary.envs {
            command.envs(envs);
        }

        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let mut process = command
            .spawn()
            .with_context(|| "failed to spawn command.")?;

        let stdin = process
            .stdin
            .take()
            .ok_or_else(|| anyhow!("Failed to open stdin"))?;
        let stdout = process
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to open stdout"))?;

        log::info!("Debug adapter has connected to stdio adapter");

        Ok(TransportParams::new(
            Box::new(stdin),
            Box::new(BufReader::new(stdout)),
            process,
        ))
    }

    fn has_adapter_logs(&self) -> bool {
        false
    }

    fn clone_box(&self) -> Box<dyn Transport> {
        Box::new(self.clone())
    }
}
