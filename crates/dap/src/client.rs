use crate::transport::{Payload, Response, Transport};
use anyhow::{anyhow, Context, Result};

use crate::adapters::{build_adapter, DebugAdapter};
use dap_types::{
    requests::{
        Attach, ConfigurationDone, Continue, Disconnect, Initialize, Launch, Next, Pause, Request,
        Restart, SetBreakpoints, StepBack, StepIn, StepOut, Terminate, TerminateThreads, Variables,
    },
    AttachRequestArguments, ConfigurationDoneArguments, ContinueArguments, ContinueResponse,
    DisconnectArguments, InitializeRequestArgumentsPathFormat, LaunchRequestArguments,
    NextArguments, PauseArguments, RestartArguments, Scope, SetBreakpointsArguments,
    SetBreakpointsResponse, Source, SourceBreakpoint, StackFrame, StepBackArguments,
    StepInArguments, StepOutArguments, SteppingGranularity, TerminateArguments,
    TerminateThreadsArguments, Variable, VariablesArguments,
};
use futures::{AsyncBufRead, AsyncWrite};
use gpui::{AppContext, AsyncAppContext};
use parking_lot::{Mutex, MutexGuard};
use serde_json::Value;
use smol::{
    channel::{bounded, Receiver, Sender},
    process::Child,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use task::{DebugAdapterConfig, DebugRequestType};

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

#[derive(Debug, Clone)]
pub struct VariableContainer {
    pub container_reference: u64,
    pub variable: Variable,
    pub depth: usize,
}

#[derive(Debug, Default, Clone)]
pub struct ThreadState {
    pub status: ThreadStatus,
    pub stack_frames: Vec<StackFrame>,
    /// HashMap<stack_frame_id, Vec<Scope>>
    pub scopes: HashMap<u64, Vec<Scope>>,
    /// BTreeMap<scope.variables_reference, Vec<VariableContainer>>
    pub variables: BTreeMap<u64, Vec<VariableContainer>>,
    pub fetched_variable_ids: HashSet<u64>,
    pub current_stack_frame_id: u64,
    // we update this value only once we stopped,
    // we will use this to indicated if we should show a warning when debugger thread was exited
    pub stopped: bool,
}

pub struct DebugAdapterClient {
    id: DebugAdapterClientId,
    adapter: Arc<Box<dyn DebugAdapter>>,
    transport: Arc<Transport>,
    _process: Arc<Mutex<Option<Child>>>,
    sequence_count: AtomicU64,
    config: DebugAdapterConfig,
    /// thread_id -> thread_state
    thread_states: Arc<Mutex<HashMap<u64, ThreadState>>>,
    capabilities: Arc<Mutex<Option<dap_types::Capabilities>>>,
}

pub struct TransportParams {
    rx: Box<dyn AsyncBufRead + Unpin + Send>,
    tx: Box<dyn AsyncWrite + Unpin + Send>,
    err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
    process: Option<Child>,
}

impl TransportParams {
    pub fn new(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        process: Option<Child>,
    ) -> Self {
        TransportParams {
            rx,
            tx,
            err,
            process,
        }
    }
}

impl DebugAdapterClient {
    pub async fn new<F>(
        id: DebugAdapterClientId,
        config: DebugAdapterConfig,
        event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<Arc<Self>>
    where
        F: FnMut(Payload, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let adapter = Arc::new(build_adapter(&config).context("Creating debug adapter")?);
        let transport_params = adapter.connect(cx).await?;

        let transport = Self::handle_transport(
            transport_params.rx,
            transport_params.tx,
            transport_params.err,
            event_handler,
            cx,
        );

        Ok(Arc::new(Self {
            id,
            config,
            adapter,
            transport,
            capabilities: Default::default(),
            thread_states: Default::default(),
            sequence_count: AtomicU64::new(1),
            _process: Arc::new(Mutex::new(transport_params.process)),
        }))
    }

    pub fn handle_transport<F>(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Arc<Transport>
    where
        F: FnMut(Payload, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let transport = Transport::start(rx, tx, err, cx);

        let server_rx = transport.server_rx.clone();
        let server_tr = transport.server_tx.clone();
        cx.spawn(|mut cx| async move {
            Self::handle_recv(server_rx, server_tr, event_handler, &mut cx).await
        })
        .detach();

        transport
    }

    async fn handle_recv<F>(
        server_rx: Receiver<Payload>,
        client_tx: Sender<Payload>,
        mut event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<()>
    where
        F: FnMut(Payload, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        while let Ok(payload) = server_rx.recv().await {
            match payload {
                Payload::Event(ev) => cx.update(|cx| event_handler(Payload::Event(ev), cx))?,
                Payload::Response(_) => unreachable!(),
                Payload::Request(req) => {
                    cx.update(|cx| event_handler(Payload::Request(req), cx))?
                }
            };
        }

        drop(client_tx);

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

        self.transport
            .server_tx
            .send(Payload::Request(request))
            .await?;

        let response = callback_rx.recv().await??;

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
        // TODO Debugger: Get request args from adapter
        Some(self.adapter.request_args())
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

    pub fn update_current_stack_frame(&self, thread_id: u64, stack_frame_id: u64) {
        if let Some(thread_state) = self.thread_states().get_mut(&thread_id) {
            thread_state.current_stack_frame_id = stack_frame_id;
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
            adapter_id: self.adapter.id(),
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
        let supports_single_thread_execution_requests = self
            .capabilities()
            .supports_single_thread_execution_requests
            .unwrap_or_default();

        self.request::<Continue>(ContinueArguments {
            thread_id,
            single_thread: supports_single_thread_execution_requests.then(|| true),
        })
        .await
    }

    pub async fn step_over(&self, thread_id: u64, granularity: SteppingGranularity) -> Result<()> {
        let capabilities = self.capabilities();

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        self.request::<Next>(NextArguments {
            thread_id,
            granularity: supports_stepping_granularity.then(|| granularity),
            single_thread: supports_single_thread_execution_requests.then(|| true),
        })
        .await
    }

    pub async fn step_in(&self, thread_id: u64, granularity: SteppingGranularity) -> Result<()> {
        let capabilities = self.capabilities();

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        self.request::<StepIn>(StepInArguments {
            thread_id,
            target_id: None,
            granularity: supports_stepping_granularity.then(|| granularity),
            single_thread: supports_single_thread_execution_requests.then(|| true),
        })
        .await
    }

    pub async fn step_out(&self, thread_id: u64, granularity: SteppingGranularity) -> Result<()> {
        let capabilities = self.capabilities();

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        self.request::<StepOut>(StepOutArguments {
            thread_id,
            granularity: supports_stepping_granularity.then(|| granularity),
            single_thread: supports_single_thread_execution_requests.then(|| true),
        })
        .await
    }

    pub async fn step_back(&self, thread_id: u64, granularity: SteppingGranularity) -> Result<()> {
        let capabilities = self.capabilities();

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        self.request::<StepBack>(StepBackArguments {
            thread_id,
            granularity: supports_stepping_granularity.then(|| granularity),
            single_thread: supports_single_thread_execution_requests.then(|| true),
        })
        .await
    }

    pub async fn restart(&self) -> Result<()> {
        self.request::<Restart>(RestartArguments {
            raw: self.adapter.request_args(),
        })
        .await
    }

    pub async fn pause(&self, thread_id: u64) -> Result<()> {
        self.request::<Pause>(PauseArguments { thread_id }).await
    }

    pub async fn disconnect(
        &self,
        restart: Option<bool>,
        terminate: Option<bool>,
        suspend: Option<bool>,
    ) -> Result<()> {
        let supports_terminate_debuggee = self
            .capabilities()
            .support_terminate_debuggee
            .unwrap_or_default();

        let supports_suspend_debuggee = self
            .capabilities()
            .support_terminate_debuggee
            .unwrap_or_default();

        self.request::<Disconnect>(DisconnectArguments {
            restart,
            terminate_debuggee: if supports_terminate_debuggee {
                terminate
            } else {
                None
            },
            suspend_debuggee: if supports_suspend_debuggee {
                suspend
            } else {
                None
            },
        })
        .await
    }

    pub async fn set_breakpoints(
        &self,
        absolute_file_path: Arc<Path>,
        breakpoints: Vec<SourceBreakpoint>,
    ) -> Result<SetBreakpointsResponse> {
        self.request::<SetBreakpoints>(SetBreakpointsArguments {
            source: Source {
                path: Some(String::from(absolute_file_path.to_string_lossy())),
                name: None,
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            },
            breakpoints: Some(breakpoints),
            source_modified: None,
            lines: None,
        })
        .await
    }

    pub async fn configuration_done(&self) -> Result<()> {
        let support_configuration_done_request = self
            .capabilities()
            .supports_configuration_done_request
            .unwrap_or_default();

        if support_configuration_done_request {
            self.request::<ConfigurationDone>(ConfigurationDoneArguments)
                .await
        } else {
            Ok(())
        }
    }

    pub async fn shutdown(&self) -> Result<()> {
        let _ = self.terminate().await;

        self.transport.server_tx.close();
        self.transport.server_rx.close();

        let mut adapter = self._process.lock().take();

        async move {
            let mut pending_requests = self.transport.pending_requests.lock().await;

            pending_requests.clear();

            if let Some(mut adapter) = adapter.take() {
                adapter.kill()?;
            }

            drop(pending_requests);
            drop(adapter);

            anyhow::Ok(())
        }
        .await
    }

    pub async fn terminate(&self) -> Result<()> {
        let support_terminate_request = self
            .capabilities()
            .supports_terminate_request
            .unwrap_or_default();

        if support_terminate_request {
            self.request::<Terminate>(TerminateArguments {
                restart: Some(false),
            })
            .await
        } else {
            self.disconnect(None, Some(true), None).await
        }
    }

    pub async fn terminate_threads(&self, thread_ids: Option<Vec<u64>>) -> Result<()> {
        let support_terminate_threads = self
            .capabilities()
            .supports_terminate_threads_request
            .unwrap_or_default();

        if support_terminate_threads {
            self.request::<TerminateThreads>(TerminateThreadsArguments { thread_ids })
                .await
        } else {
            self.terminate().await
        }
    }

    pub async fn variables(&self, variables_reference: u64) -> Result<Vec<Variable>> {
        anyhow::Ok(
            self.request::<Variables>(VariablesArguments {
                variables_reference,
                filter: None,
                start: None,
                count: None,
                format: None,
            })
            .await?
            .variables,
        )
    }
}
