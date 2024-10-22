use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use dap_types::{
    messages::{Message, Response},
    ErrorResponse,
};
use futures::{select, AsyncBufRead, AsyncReadExt as _, AsyncWrite, FutureExt as _};
use gpui::AsyncAppContext;
use smallvec::SmallVec;
use smol::{
    channel::{unbounded, Receiver, Sender},
    io::{AsyncBufReadExt as _, AsyncWriteExt, BufReader},
    lock::Mutex,
    net::{TcpListener, TcpStream},
    process::{self, Child, ChildStderr, ChildStdout},
};
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    process::Stdio,
    sync::Arc,
    time::Duration,
};
use task::TCPHost;

use crate::adapters::DebugAdapterBinary;

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

type Requests = Arc<Mutex<HashMap<u64, Sender<Result<Response>>>>>;
type LogHandlers = Arc<parking_lot::Mutex<SmallVec<[(LogKind, IoHandler); 2]>>>;

pub(crate) struct TransportDelegate {
    log_handlers: LogHandlers,
    current_requests: Requests,
    pending_requests: Requests,
    transport: Box<dyn Transport>,
    process: Arc<Mutex<Option<Child>>>,
    server_tx: Option<Sender<Message>>,
}

impl TransportDelegate {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        Self {
            transport,
            server_tx: None,
            process: Default::default(),
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

        if let Some(stdout) = params.process.stdout.take() {
            cx.background_executor()
                .spawn(Self::handle_adapter_log(stdout, self.log_handlers.clone()))
                .detach();
        }

        cx.background_executor()
            .spawn(Self::handle_output(
                params.output,
                client_tx,
                self.pending_requests.clone(),
                self.log_handlers.clone(),
            ))
            .detach();

        if let Some(stderr) = params.process.stderr.take() {
            cx.background_executor()
                .spawn(Self::handle_error(stderr, self.log_handlers.clone()))
                .detach();
        }

        cx.background_executor()
            .spawn(Self::handle_input(
                params.input,
                client_rx,
                self.current_requests.clone(),
                self.pending_requests.clone(),
                self.log_handlers.clone(),
            ))
            .detach();

        self.process = Arc::new(Mutex::new(Some(params.process)));
        self.server_tx = Some(server_tx.clone());

        Ok((server_rx, server_tx))
    }

    pub(crate) async fn add_pending_request(
        &self,
        sequence_id: u64,
        request: Sender<Result<Response>>,
    ) {
        let mut pending_requests = self.pending_requests.lock().await;
        pending_requests.insert(sequence_id, request);
    }

    pub(crate) async fn send_message(&self, message: Message) -> Result<()> {
        if let Some(server_tx) = self.server_tx.as_ref() {
            server_tx
                .send(message)
                .await
                .map_err(|e| anyhow!("Failed to send response back: {}", e))
        } else {
            Err(anyhow!("Server tx already dropped"))
        }
    }

    async fn handle_adapter_log(stdout: ChildStdout, log_handlers: LogHandlers) -> Result<()> {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        while reader.read_line(&mut line).await? > 0 {
            for (kind, handler) in log_handlers.lock().iter_mut() {
                if matches!(kind, LogKind::Adapter) {
                    handler(IoKind::StdOut, line.as_str());
                }
            }
            line.truncate(0);
        }

        Ok(())
    }

    async fn handle_input(
        mut server_stdin: Box<dyn AsyncWrite + Unpin + Send>,
        client_rx: Receiver<Message>,
        current_requests: Requests,
        pending_requests: Requests,
        log_handlers: LogHandlers,
    ) -> Result<()> {
        while let Ok(mut payload) = client_rx.recv().await {
            if let Message::Request(request) = payload.borrow_mut() {
                if let Some(sender) = current_requests.lock().await.remove(&request.seq) {
                    pending_requests.lock().await.insert(request.seq, sender);
                }
            }

            let message = serde_json::to_string(&payload)?;

            for (kind, log_handler) in log_handlers.lock().iter_mut() {
                if matches!(kind, LogKind::Rpc) {
                    log_handler(IoKind::StdIn, &message);
                }
            }

            server_stdin
                .write_all(
                    format!("Content-Length: {}\r\n\r\n{}", message.len(), message).as_bytes(),
                )
                .await?;

            server_stdin.flush().await?;
        }

        Ok(())
    }

    async fn handle_output(
        mut server_stdout: Box<dyn AsyncBufRead + Unpin + Send>,
        client_tx: Sender<Message>,
        pending_requests: Requests,
        log_handlers: LogHandlers,
    ) -> Result<()> {
        let mut recv_buffer = String::new();

        while let Ok(message) =
            Self::receive_server_message(&mut server_stdout, &mut recv_buffer, &log_handlers).await
        {
            match message {
                Message::Response(res) => {
                    if let Some(tx) = pending_requests.lock().await.remove(&res.request_seq) {
                        tx.send(Self::process_response(res)).await?;
                    } else {
                        client_tx.send(Message::Response(res)).await?;
                    };
                }
                Message::Request(_) => {
                    client_tx.send(message).await?;
                }
                Message::Event(_) => {
                    client_tx.send(message).await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_error(stderr: ChildStderr, log_handlers: LogHandlers) -> Result<()> {
        let mut buffer = String::new();

        let mut reader = BufReader::new(stderr);

        loop {
            buffer.truncate(0);
            if reader.read_line(&mut buffer).await? == 0 {
                return Err(anyhow!("debugger error stream closed"));
            }

            for (kind, log_handler) in log_handlers.lock().iter_mut() {
                if matches!(kind, LogKind::Adapter) {
                    log_handler(IoKind::StdErr, buffer.as_str());
                }
            }
        }
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
        if let Some(server_tx) = self.server_tx.as_ref() {
            server_tx.close();
        }

        let mut adapter = self.process.lock().await.take();
        let mut current_requests = self.current_requests.lock().await;
        let mut pending_requests = self.pending_requests.lock().await;

        current_requests.clear();
        pending_requests.clear();

        if let Some(mut adapter) = adapter.take() {
            adapter.kill()?;
        }

        drop(current_requests);
        drop(pending_requests);
        drop(adapter);

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
}

pub struct TcpTransport {
    config: TCPHost,
}

impl TcpTransport {
    pub fn new(config: TCPHost) -> Self {
        Self { config }
    }

    /// Get an open port to use with the tcp client when not supplied by debug config
    async fn get_open_port(host: Ipv4Addr) -> Option<u16> {
        Some(
            TcpListener::bind(SocketAddrV4::new(host, 0))
                .await
                .ok()?
                .local_addr()
                .ok()?
                .port(),
        )
    }
}

#[async_trait(?Send)]
impl Transport for TcpTransport {
    async fn start(
        &mut self,
        binary: &DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        let host_address = self
            .config
            .host
            .unwrap_or_else(|| Ipv4Addr::new(127, 0, 0, 1));

        let mut port = self.config.port;
        if port.is_none() {
            port = Self::get_open_port(host_address).await;
        }

        let mut command = process::Command::new(&binary.command);

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

        let address = SocketAddrV4::new(
            host_address,
            port.ok_or(anyhow!("Port is required to connect to TCP server"))?,
        );

        let timeout = self.config.timeout.unwrap_or(2000);

        let (rx, tx) = select! {
            _ = cx.background_executor().timer(Duration::from_millis(timeout)).fuse() => {
                return Err(anyhow!("Connection to tcp DAP timeout"))
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
        log::info!("Debug adapter has connected to tcp server");

        Ok(TransportParams::new(
            Box::new(tx),
            Box::new(BufReader::new(rx)),
            process,
        ))
    }

    fn has_adapter_logs(&self) -> bool {
        true
    }
}

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
}
