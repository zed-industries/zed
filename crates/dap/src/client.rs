use crate::transport::{Payload, Response, Transport};
use anyhow::{anyhow, Context, Result};

use dap_types::{
    requests::{
        Attach, ConfigurationDone, Continue, Disconnect, Initialize, Launch, Next, Pause, Request,
        Restart, SetBreakpoints, StepBack, StepIn, StepOut,
    },
    AttachRequestArguments, ConfigurationDoneArguments, ContinueArguments, ContinueResponse,
    DisconnectArguments, InitializeRequestArgumentsPathFormat, LaunchRequestArguments,
    NextArguments, PauseArguments, RestartArguments, Scope, SetBreakpointsArguments,
    SetBreakpointsResponse, Source, SourceBreakpoint, StackFrame, StepBackArguments,
    StepInArguments, StepOutArguments, SteppingGranularity, Variable,
};
use futures::{AsyncBufRead, AsyncReadExt, AsyncWrite};
use gpui::{AppContext, AsyncAppContext};
use language::Buffer;
use parking_lot::{Mutex, MutexGuard};
use serde_json::Value;
use smol::{
    channel::{bounded, unbounded, Receiver, Sender},
    io::BufReader,
    net::{TcpListener, TcpStream},
    process::{self, Child},
};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
    process::Stdio,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use task::{DebugAdapterConfig, DebugConnectionType, DebugRequestType, TCPHost};
use text::Point;
use util::ResultExt;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThreadStatus {
    #[default]
    Running,
    Stopped,
    Exited,
    Ended,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugAdapterClientId(pub usize);

#[derive(Debug, Default, Clone)]
pub struct ThreadState {
    pub status: ThreadStatus,
    pub stack_frames: Vec<StackFrame>,
    pub scopes: HashMap<u64, Vec<Scope>>, // stack_frame_id -> scopes
    pub variables: HashMap<u64, Vec<Variable>>, // scope.variable_reference -> variables
    pub current_stack_frame_id: Option<u64>,
}

pub struct DebugAdapterClient {
    id: DebugAdapterClientId,
    pub args: Vec<String>,
    pub command: String,
    pub cwd: PathBuf,
    pub request_args: Option<Value>,
    _process: Option<Child>,
    server_tx: Sender<Payload>,
    sequence_count: AtomicU64,
    config: DebugAdapterConfig,
    thread_states: Arc<Mutex<HashMap<u64, ThreadState>>>, // thread_id -> thread_state
    capabilities: Arc<Mutex<Option<dap_types::Capabilities>>>,
}

pub struct TransportParams {
    rx: Box<dyn AsyncBufRead + Unpin + Send>,
    tx: Box<dyn AsyncWrite + Unpin + Send>,
    err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
    process: Option<Child>,
}

impl DebugAdapterClient {
    /// Creates & returns a new debug adapter client
    ///
    /// # Parameters
    /// - `id`: The id that [`Project`](project::Project) uses to keep track of specific clients
    /// - `config`: The adapter specific configurations from debugger task that is starting
    /// - `command`: The command that starts the debugger
    /// - `args`: Arguments of the command that starts the debugger
    /// - `cwd`: The absolute path of the project that is being debugged
    /// - `cx`: The context that the new client belongs too
    #[allow(clippy::too_many_arguments)]
    pub async fn new<F>(
        id: DebugAdapterClientId,
        config: DebugAdapterConfig,
        command: &String,
        args: &Vec<String>,
        cwd: &PathBuf,
        request_args: Option<Value>,
        event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<Arc<Self>>
    where
        F: FnMut(Payload, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let transport_params = match config.connection.clone() {
            DebugConnectionType::TCP(host) => {
                Self::create_tcp_client(host, command, args, cwd, cx).await?
            }
            DebugConnectionType::STDIO => Self::create_stdio_client(command, args, cwd).await?,
        };

        let server_tx = Self::handle_transport(
            transport_params.rx,
            transport_params.tx,
            transport_params.err,
            event_handler,
            cx,
        )?;

        Ok(Arc::new(Self {
            id,
            config,
            server_tx,
            request_args,
            cwd: cwd.clone(),
            args: args.clone(),
            command: command.clone(),
            capabilities: Default::default(),
            thread_states: Default::default(),
            sequence_count: AtomicU64::new(1),
            _process: transport_params.process,
        }))
    }

    /// Creates a debug client that connects to an adapter through tcp
    ///
    /// TCP clients don't have an error communication stream with an adapter
    ///
    /// # Parameters
    /// - `command`: The command that starts the debugger
    /// - `args`: Arguments of the command that starts the debugger
    /// - `cwd`: The absolute path of the project that is being debugged
    /// - `cx`: The context that the new client belongs too
    async fn create_tcp_client(
        host: TCPHost,
        command: &String,
        args: &Vec<String>,
        cwd: &PathBuf,
        cx: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        let host_address = host.host.unwrap_or_else(|| Ipv4Addr::new(127, 0, 0, 1));

        let mut port = host.port;
        if port.is_none() {
            port = Self::get_port(host_address).await;
        }

        let mut command = process::Command::new(command);
        command
            .current_dir(cwd)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);

        let process = command
            .spawn()
            .with_context(|| "failed to start debug adapter.")?;

        if let Some(delay) = host.delay {
            // some debug adapters need some time to start the TCP server
            // so we have to wait few milliseconds before we can connect to it
            cx.background_executor()
                .timer(Duration::from_millis(delay))
                .await;
        }

        let address = SocketAddrV4::new(
            host_address,
            port.ok_or(anyhow!("Port is required to connect to TCP server"))?,
        );

        let (rx, tx) = TcpStream::connect(address).await?.split();

        Ok(TransportParams {
            rx: Box::new(BufReader::new(rx)),
            tx: Box::new(tx),
            err: None,
            process: Some(process),
        })
    }

    /// Get an open port to use with the tcp client when not supplied by debug config
    async fn get_port(host: Ipv4Addr) -> Option<u16> {
        Some(
            TcpListener::bind(SocketAddrV4::new(host, 0))
                .await
                .ok()?
                .local_addr()
                .ok()?
                .port(),
        )
    }

    /// Creates a debug client that connects to an adapter through std input/output
    ///
    /// # Parameters
    /// - `command`: The command that starts the debugger
    /// - `args`: Arguments of the command that starts the debugger
    /// - `cwd`: The absolute path of the project that is being debugged
    async fn create_stdio_client(
        command: &String,
        args: &Vec<String>,
        cwd: &PathBuf,
    ) -> Result<TransportParams> {
        let mut command = process::Command::new(command);
        command
            .current_dir(cwd)
            .args(args)
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
        let stderr = process
            .stderr
            .take()
            .ok_or_else(|| anyhow!("Failed to open stderr"))?;

        Ok(TransportParams {
            rx: Box::new(BufReader::new(stdout)),
            tx: Box::new(stdin),
            err: Some(Box::new(BufReader::new(stderr))),
            process: Some(process),
        })
    }

    pub fn handle_transport<F>(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<Sender<Payload>>
    where
        F: FnMut(Payload, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let (server_rx, server_tx) = Transport::start(rx, tx, err, cx);
        let (client_tx, client_rx) = unbounded::<Payload>();

        cx.update(|cx| {
            cx.background_executor()
                .spawn(Self::handle_recv(server_rx, client_tx.clone()))
                .detach_and_log_err(cx);

            cx.spawn({
                |mut cx| async move { Self::handle_events(client_rx, event_handler, &mut cx).await }
            })
            .detach_and_log_err(cx);

            server_tx
        })
    }

    /// Set's up a client's event handler.
    ///
    /// This function should only be called once or else errors will arise
    /// # Parameters
    /// `client`: A pointer to the client to pass the event handler too
    /// `event_handler`: The function that is called to handle events
    ///     should be DebugPanel::handle_debug_client_events
    /// `cx`: The context that this task will run in
    pub async fn handle_events<F>(
        client_rx: Receiver<Payload>,
        mut event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<()>
    where
        F: FnMut(Payload, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        while let Ok(payload) = client_rx.recv().await {
            cx.update(|cx| event_handler(payload, cx))?;
        }

        anyhow::Ok(())
    }

    // async fn handle_run_in_terminal_request(
    //     this: &Arc<Self>,
    //     request: crate::transport::Request,
    //     cx: &mut AsyncAppContext,
    // ) -> Result<()> {
    //     let arguments: RunInTerminalRequestArguments =
    //         serde_json::from_value(request.arguments.unwrap_or_default())?;

    //     let mut args = arguments.args.clone();
    //     let mut command = process::Command::new(args.remove(0));

    //     let envs = arguments.env.as_ref().and_then(|e| e.as_object()).map(|e| {
    //         e.iter()
    //             .map(|(key, value)| ((key.clone(), value.clone().to_string())))
    //             .collect::<Vec<(String, String)>>()
    //     });

    //     if let Some(envs) = envs {
    //         command.envs(envs);
    //     }

    //     let process = command
    //         .current_dir(arguments.cwd)
    //         .args(args)
    //         .spawn()
    //         .with_context(|| "failed to spawn run in terminal command.")?;

    //     this.server_tx
    //         .send(Payload::Response(Response {
    //             request_seq: request.seq,
    //             success: true,
    //             command: RunInTerminal::COMMAND.into(),
    //             message: None,
    //             body: Some(serde_json::to_value(RunInTerminalResponse {
    //                 process_id: Some(process.id() as u64),
    //                 shell_process_id: None,
    //             })?),
    //         }))
    //         .await?;

    //     anyhow::Ok(())
    // }

    async fn handle_recv(server_rx: Receiver<Payload>, client_tx: Sender<Payload>) -> Result<()> {
        while let Ok(payload) = server_rx.recv().await {
            match payload {
                Payload::Event(ev) => client_tx.send(Payload::Event(ev)).await?,
                Payload::Response(_) => unreachable!(),
                Payload::Request(req) => client_tx.send(Payload::Request(req)).await?,
            };
        }

        anyhow::Ok(())
    }

    /// Send a request to an adapter and get a response back
    /// Note: This function will block until a response is sent back from the adapter
    pub async fn request<R: Request>(&self, arguments: R::Arguments) -> Result<R::Response> {
        let serialized_arguments = serde_json::to_value(arguments)?;

        let (callback_tx, callback_rx) = bounded::<Result<Response>>(1);

        let request = crate::transport::Request {
            back_ch: Some(callback_tx),
            seq: self.next_sequence_id(),
            command: R::COMMAND.to_string(),
            arguments: Some(serialized_arguments),
        };

        self.server_tx.send(Payload::Request(request)).await?;

        let response = callback_rx.recv().await??;
        let _ = self.next_sequence_id();

        match response.success {
            true => Ok(serde_json::from_value(response.body.unwrap_or_default())?),
            false => Err(anyhow!("Request failed")),
        }
    }

    pub fn id(&self) -> DebugAdapterClientId {
        self.id
    }

    pub fn config(&self) -> DebugAdapterConfig {
        self.config.clone()
    }

    pub fn request_args(&self) -> Option<Value> {
        self.request_args.clone()
    }

    pub fn request_type(&self) -> DebugRequestType {
        self.config.request.clone()
    }

    pub fn capabilities(&self) -> dap_types::Capabilities {
        self.capabilities.lock().clone().unwrap_or_default()
    }

    /// Get the next sequence id to be used in a request
    pub fn next_sequence_id(&self) -> u64 {
        self.sequence_count.fetch_add(1, Ordering::Relaxed)
    }

    pub fn update_thread_state_status(&self, thread_id: u64, status: ThreadStatus) {
        if let Some(thread_state) = self.thread_states().get_mut(&thread_id) {
            thread_state.status = status;
        };
    }

    pub fn thread_states(&self) -> MutexGuard<HashMap<u64, ThreadState>> {
        self.thread_states.lock()
    }

    pub fn thread_state_by_id(&self, thread_id: u64) -> ThreadState {
        self.thread_states.lock().get(&thread_id).cloned().unwrap()
    }

    pub async fn initialize(&self) -> Result<dap_types::Capabilities> {
        let args = dap_types::InitializeRequestArguments {
            client_id: Some("zed".to_owned()),
            client_name: Some("Zed".to_owned()),
            adapter_id: self.config.id.clone(),
            locale: Some("en-us".to_owned()),
            path_format: Some(InitializeRequestArgumentsPathFormat::Path),
            supports_variable_type: Some(true),
            supports_variable_paging: Some(false),
            supports_run_in_terminal_request: Some(true),
            supports_memory_references: Some(true),
            supports_progress_reporting: Some(true),
            supports_invalidated_event: Some(true),
            lines_start_at1: Some(true),
            columns_start_at1: Some(true),
            supports_memory_event: Some(true),
            supports_args_can_be_interpreted_by_shell: Some(true),
            supports_start_debugging_request: Some(true),
        };

        let capabilities = self.request::<Initialize>(args).await?;

        *self.capabilities.lock() = Some(capabilities.clone());

        Ok(capabilities)
    }

    pub async fn launch(&self, args: Option<Value>) -> Result<()> {
        self.request::<Launch>(LaunchRequestArguments {
            raw: args.unwrap_or(Value::Null),
        })
        .await
    }

    pub async fn attach(&self, args: Option<Value>) -> Result<()> {
        self.request::<Attach>(AttachRequestArguments {
            raw: args.unwrap_or(Value::Null),
        })
        .await
    }

    pub async fn resume(&self, thread_id: u64) -> Result<ContinueResponse> {
        self.request::<Continue>(ContinueArguments {
            thread_id,
            single_thread: Some(true),
        })
        .await
    }

    pub async fn step_over(&self, thread_id: u64) -> Result<()> {
        self.request::<Next>(NextArguments {
            thread_id,
            granularity: Some(SteppingGranularity::Statement),
            single_thread: Some(true),
        })
        .await
    }

    pub async fn step_in(&self, thread_id: u64) -> Result<()> {
        self.request::<StepIn>(StepInArguments {
            thread_id,
            target_id: None,
            granularity: Some(SteppingGranularity::Statement),
            single_thread: Some(true),
        })
        .await
    }

    pub async fn step_out(&self, thread_id: u64) -> Result<()> {
        self.request::<StepOut>(StepOutArguments {
            thread_id,
            granularity: Some(SteppingGranularity::Statement),
            single_thread: Some(true),
        })
        .await
    }

    pub async fn step_back(&self, thread_id: u64) -> Result<()> {
        self.request::<StepBack>(StepBackArguments {
            thread_id,
            single_thread: Some(true),
            granularity: Some(SteppingGranularity::Statement),
        })
        .await
    }

    pub async fn restart(&self) {
        self.request::<Restart>(RestartArguments {
            raw: self
                .config
                .request_args
                .as_ref()
                .map(|v| v.args.clone())
                .unwrap_or(Value::Null),
        })
        .await
        .log_err();
    }

    pub async fn pause(&self, thread_id: u64) {
        self.request::<Pause>(PauseArguments { thread_id })
            .await
            .log_err();
    }

    pub async fn stop(&self) {
        self.request::<Disconnect>(DisconnectArguments {
            restart: Some(false),
            terminate_debuggee: Some(false),
            suspend_debuggee: Some(false),
        })
        .await
        .log_err();
    }

    pub async fn set_breakpoints(
        &self,
        path: PathBuf,
        breakpoints: Option<Vec<SourceBreakpoint>>,
    ) -> Result<SetBreakpointsResponse> {
        let adapter_data = self.config.request_args.clone().map(|c| c.args);

        self.request::<SetBreakpoints>(SetBreakpointsArguments {
            source: Source {
                path: Some(String::from(path.to_string_lossy())),
                name: None,
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data,
                checksums: None,
            },
            breakpoints,
            source_modified: None,
            lines: None,
        })
        .await
    }

    pub async fn configuration_done(&self) -> Result<()> {
        self.request::<ConfigurationDone>(ConfigurationDoneArguments)
            .await
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Breakpoint {
    pub position: multi_buffer::Anchor,
}

impl Breakpoint {
    pub fn to_source_breakpoint(&self, buffer: &Buffer) -> SourceBreakpoint {
        SourceBreakpoint {
            line: (buffer
                .summary_for_anchor::<Point>(&self.position.text_anchor)
                .row
                + 1) as u64,
            condition: None,
            hit_condition: None,
            log_message: None,
            column: None,
            mode: None,
        }
    }
}
