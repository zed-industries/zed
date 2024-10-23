use crate::ProjectPath;
use anyhow::{anyhow, Context as _, Result};
use collections::{HashMap, HashSet};
use dap::client::{DebugAdapterClient, DebugAdapterClientId};
use dap::messages::{Message, Response};
use dap::requests::{
    Attach, Completions, ConfigurationDone, Continue, Disconnect, Evaluate, Initialize, Launch,
    LoadedSources, Modules, Next, Pause, Request as _, RunInTerminal, Scopes, SetBreakpoints,
    SetExpression, SetVariable, StackTrace, StartDebugging, StepIn, StepOut, Terminate,
    TerminateThreads, Variables,
};
use dap::{
    AttachRequestArguments, Capabilities, CompletionItem, CompletionsArguments,
    ConfigurationDoneArguments, ContinueArguments, DisconnectArguments, ErrorResponse,
    EvaluateArguments, EvaluateArgumentsContext, EvaluateResponse, InitializeRequestArguments,
    InitializeRequestArgumentsPathFormat, LaunchRequestArguments, LoadedSourcesArguments, Module,
    ModulesArguments, NextArguments, PauseArguments, RunInTerminalResponse, Scope, ScopesArguments,
    SetBreakpointsArguments, SetExpressionArguments, SetVariableArguments, Source,
    SourceBreakpoint, StackFrame, StackTraceArguments, StartDebuggingRequestArguments,
    StepInArguments, StepOutArguments, SteppingGranularity, TerminateArguments,
    TerminateThreadsArguments, Variable, VariablesArguments,
};
use dap_adapters::build_adapter;
use fs::Fs;
use gpui::{EventEmitter, Model, ModelContext, Task};
use http_client::HttpClient;
use language::{Buffer, BufferSnapshot};
use node_runtime::NodeRuntime;
use serde_json::{json, Value};
use settings::WorktreeId;
use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use task::{DebugAdapterConfig, DebugRequestType};
use text::Point;
use util::{merge_json_value_into, ResultExt};

pub enum DapStoreEvent {
    DebugClientStarted(DebugAdapterClientId),
    DebugClientStopped(DebugAdapterClientId),
    DebugClientEvent {
        client_id: DebugAdapterClientId,
        message: Message,
    },
}

pub enum DebugAdapterClientState {
    Starting(Task<Option<Arc<DebugAdapterClient>>>),
    Running(Arc<DebugAdapterClient>),
}

#[derive(Clone, Debug)]
pub struct DebugPosition {
    pub row: u32,
    pub column: u32,
}

pub struct DapStore {
    next_client_id: AtomicUsize,
    clients: HashMap<DebugAdapterClientId, DebugAdapterClientState>,
    breakpoints: BTreeMap<ProjectPath, HashSet<Breakpoint>>,
    capabilities: HashMap<DebugAdapterClientId, Capabilities>,
    active_debug_line: Option<(ProjectPath, DebugPosition)>,
    http_client: Option<Arc<dyn HttpClient>>,
    node_runtime: Option<NodeRuntime>,
    fs: Arc<dyn Fs>,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    pub fn new(
        http_client: Option<Arc<dyn HttpClient>>,
        node_runtime: Option<NodeRuntime>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        cx.on_app_quit(Self::shutdown_clients).detach();

        Self {
            active_debug_line: None,
            clients: Default::default(),
            breakpoints: Default::default(),
            capabilities: HashMap::default(),
            next_client_id: Default::default(),
            http_client,
            node_runtime,
            fs,
        }
    }

    pub fn next_client_id(&self) -> DebugAdapterClientId {
        DebugAdapterClientId(self.next_client_id.fetch_add(1, SeqCst))
    }

    pub fn running_clients(&self) -> impl Iterator<Item = Arc<DebugAdapterClient>> + '_ {
        self.clients.values().filter_map(|state| match state {
            DebugAdapterClientState::Starting(_) => None,
            DebugAdapterClientState::Running(client) => Some(client.clone()),
        })
    }

    pub fn client_by_id(&self, id: &DebugAdapterClientId) -> Option<Arc<DebugAdapterClient>> {
        self.clients.get(id).and_then(|state| match state {
            DebugAdapterClientState::Starting(_) => None,
            DebugAdapterClientState::Running(client) => Some(client.clone()),
        })
    }

    pub fn capabilities_by_id(&self, client_id: &DebugAdapterClientId) -> Capabilities {
        self.capabilities
            .get(client_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn merge_capabilities_for_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        other: &Capabilities,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(capabilities) = self.capabilities.get_mut(client_id) {
            *capabilities = capabilities.merge(other.clone());

            cx.notify();
        }
    }

    pub fn active_debug_line(&self) -> Option<(ProjectPath, DebugPosition)> {
        self.active_debug_line.clone()
    }

    pub fn set_active_debug_line(
        &mut self,
        project_path: &ProjectPath,
        row: u32,
        column: u32,
        cx: &mut ModelContext<Self>,
    ) {
        self.active_debug_line = Some((project_path.clone(), DebugPosition { row, column }));

        cx.notify();
    }

    pub fn remove_active_debug_line(&mut self) {
        self.active_debug_line.take();
    }

    pub fn breakpoints(&self) -> &BTreeMap<ProjectPath, HashSet<Breakpoint>> {
        &self.breakpoints
    }

    pub fn breakpoint_at_row(
        &self,
        row: u32,
        project_path: &ProjectPath,
        buffer_snapshot: BufferSnapshot,
    ) -> Option<Breakpoint> {
        let breakpoint_set = self.breakpoints.get(project_path)?;

        breakpoint_set
            .iter()
            .find(|bp| bp.point_for_buffer_snapshot(&buffer_snapshot).row == row)
            .cloned()
    }

    pub fn on_open_buffer(
        &mut self,
        project_path: &ProjectPath,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        let entry = self.breakpoints.remove(project_path).unwrap_or_default();
        let mut set_bp: HashSet<Breakpoint> = HashSet::default();

        let buffer = buffer.read(cx);

        for mut bp in entry.into_iter() {
            bp.set_active_position(&buffer);
            set_bp.insert(bp);
        }

        self.breakpoints.insert(project_path.clone(), set_bp);

        cx.notify();
    }

    pub fn deserialize_breakpoints(
        &mut self,
        worktree_id: WorktreeId,
        serialize_breakpoints: Vec<SerializedBreakpoint>,
    ) {
        for serialize_breakpoint in serialize_breakpoints {
            self.breakpoints
                .entry(ProjectPath {
                    worktree_id,
                    path: serialize_breakpoint.path.clone(),
                })
                .or_default()
                .insert(Breakpoint {
                    active_position: None,
                    cache_position: serialize_breakpoint.position.saturating_sub(1u32),
                    kind: serialize_breakpoint.kind,
                });
        }
    }

    pub fn sync_open_breakpoints_to_closed_breakpoints(
        &mut self,
        project_path: &ProjectPath,
        buffer: &mut Buffer,
    ) {
        if let Some(breakpoint_set) = self.breakpoints.remove(project_path) {
            let breakpoint_iter = breakpoint_set.into_iter().map(|mut bp| {
                bp.cache_position = bp.point_for_buffer(&buffer).row;
                bp.active_position = None;
                bp
            });

            self.breakpoints.insert(
                project_path.clone(),
                breakpoint_iter.collect::<HashSet<_>>(),
            );
        }
    }

    pub fn start_client(
        &mut self,
        config: DebugAdapterConfig,
        args: Option<StartDebuggingRequestArguments>,
        cx: &mut ModelContext<Self>,
    ) {
        let client_id = self.next_client_id();
        let adapter_delegate = DapAdapterDelegate::new(
            self.http_client.clone(),
            self.node_runtime.clone(),
            self.fs.clone(),
        );
        let start_client_task = cx.spawn(|this, mut cx| async move {
            let dap_store = this.clone();
            let adapter = Arc::new(
                build_adapter(&config)
                    .context("Creating debug adapter")
                    .log_err()?,
            );

            let mut binary = adapter.fetch_binary(&adapter_delegate, &config).await.ok();

            if binary.is_none() {
                let _ = adapter
                    .install_binary(&adapter_delegate)
                    .await
                    .context("Failed to install debug adapter binary")
                    .log_err()?;

                binary = adapter
                    .fetch_binary(&adapter_delegate, &config)
                    .await
                    .context("Failed to get debug adapter binary")
                    .log_err();
            }

            let mut request_args = json!({});
            if let Some(config_args) = config.initialize_args.clone() {
                merge_json_value_into(config_args, &mut request_args);
            }

            merge_json_value_into(adapter.request_args(&config), &mut request_args);

            if let Some(args) = args {
                merge_json_value_into(args.configuration, &mut request_args);
            }

            let mut client = DebugAdapterClient::new(client_id, request_args, config, adapter);

            client
                .start(
                    &binary?,
                    move |message, cx| {
                        dap_store
                            .update(cx, |_, cx| {
                                cx.emit(DapStoreEvent::DebugClientEvent { client_id, message })
                            })
                            .log_err();
                    },
                    &mut cx,
                )
                .await
                .log_err()?;

            let client = Arc::new(client);

            this.update(&mut cx, |store, cx| {
                let handle = store
                    .clients
                    .get_mut(&client_id)
                    .with_context(|| "Failed to find starting debug client")?;

                *handle = DebugAdapterClientState::Running(client.clone());

                cx.emit(DapStoreEvent::DebugClientStarted(client_id));

                anyhow::Ok(())
            })
            .log_err();

            Some(client)
        });

        self.clients.insert(
            client_id,
            DebugAdapterClientState::Starting(start_client_task),
        );
    }

    pub fn initialize(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|this, mut cx| async move {
            let capabilities = client
                .request::<Initialize>(InitializeRequestArguments {
                    client_id: Some("zed".to_owned()),
                    client_name: Some("Zed".to_owned()),
                    adapter_id: client.adapter_id(),
                    locale: Some("en-US".to_owned()),
                    path_format: Some(InitializeRequestArgumentsPathFormat::Path),
                    supports_variable_type: Some(true),
                    supports_variable_paging: Some(false),
                    supports_run_in_terminal_request: Some(true),
                    supports_memory_references: Some(true),
                    supports_progress_reporting: Some(false),
                    supports_invalidated_event: Some(false),
                    lines_start_at1: Some(false),
                    columns_start_at1: Some(false),
                    supports_memory_event: Some(false),
                    supports_args_can_be_interpreted_by_shell: Some(false),
                    supports_start_debugging_request: Some(true),
                })
                .await?;

            this.update(&mut cx, |store, cx| {
                store.capabilities.insert(client.id(), capabilities);

                cx.notify();
            })?;

            // send correct request based on adapter config
            match client.config().request {
                DebugRequestType::Launch => {
                    client
                        .request::<Launch>(LaunchRequestArguments {
                            raw: client.request_args(),
                        })
                        .await?
                }
                DebugRequestType::Attach => {
                    client
                        .request::<Attach>(AttachRequestArguments {
                            raw: client.request_args(),
                        })
                        .await?
                }
            }

            Ok(())
        })
    }

    pub fn modules(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Module>>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        let capabilities = self.capabilities_by_id(client_id);

        if !capabilities.supports_modules_request.unwrap_or_default() {
            return Task::ready(Ok(Vec::default()));
        }

        cx.spawn(|_, _| async move {
            Ok(client
                .request::<Modules>(ModulesArguments {
                    start_module: None,
                    module_count: None,
                })
                .await?
                .modules)
        })
    }

    pub fn loaded_sources(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Source>>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        let capabilities = self.capabilities_by_id(client_id);

        if !capabilities
            .supports_loaded_sources_request
            .unwrap_or_default()
        {
            return Task::ready(Ok(Vec::default()));
        }

        cx.spawn(|_, _| async move {
            Ok(client
                .request::<LoadedSources>(LoadedSourcesArguments {})
                .await?
                .sources)
        })
    }

    pub fn stack_frames(
        &mut self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<StackFrame>>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        cx.spawn(|_, _| async move {
            Ok(client
                .request::<StackTrace>(StackTraceArguments {
                    thread_id,
                    start_frame: None,
                    levels: None,
                    format: None,
                })
                .await?
                .stack_frames)
        })
    }

    pub fn scopes(
        &mut self,
        client_id: &DebugAdapterClientId,
        stack_frame_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Scope>>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        cx.spawn(|_, _| async move {
            Ok(client
                .request::<Scopes>(ScopesArguments {
                    frame_id: stack_frame_id,
                })
                .await?
                .scopes)
        })
    }

    pub fn configuration_done(
        &self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        let capabilities = self.capabilities_by_id(client_id);

        cx.spawn(|_, _| async move {
            let support_configuration_done_request = capabilities
                .supports_configuration_done_request
                .unwrap_or_default();

            if support_configuration_done_request {
                client
                    .request::<ConfigurationDone>(ConfigurationDoneArguments)
                    .await
            } else {
                Ok(())
            }
        })
    }

    pub fn respond_to_start_debugging(
        &self,
        client_id: &DebugAdapterClientId,
        seq: u64,
        args: Option<StartDebuggingRequestArguments>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|this, mut cx| async move {
            client
                .send_message(Message::Response(Response {
                    seq,
                    request_seq: seq,
                    success: true,
                    command: StartDebugging::COMMAND.to_string(),
                    body: None,
                }))
                .await?;

            this.update(&mut cx, |store, cx| {
                store.start_client(client.config(), args, cx);
            })
        })
    }

    pub fn respond_to_run_in_terminal(
        &self,
        client_id: &DebugAdapterClientId,
        success: bool,
        seq: u64,
        shell_pid: Option<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move {
            if success {
                client
                    .send_message(Message::Response(Response {
                        seq,
                        request_seq: seq,
                        success: true,
                        command: RunInTerminal::COMMAND.to_string(),
                        body: Some(serde_json::to_value(RunInTerminalResponse {
                            process_id: Some(std::process::id() as u64),
                            shell_process_id: shell_pid,
                        })?),
                    }))
                    .await
            } else {
                client
                    .send_message(Message::Response(Response {
                        seq,
                        request_seq: seq,
                        success: false,
                        command: RunInTerminal::COMMAND.to_string(),
                        body: Some(serde_json::to_value(ErrorResponse { error: None })?),
                    }))
                    .await
            }
        })
    }

    pub fn continue_thread(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move {
            client
                .request::<Continue>(ContinueArguments {
                    thread_id,
                    single_thread: Some(true),
                })
                .await?;

            Ok(())
        })
    }

    pub fn step_over(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        granularity: SteppingGranularity,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        let capabilities = self.capabilities_by_id(client_id);

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        cx.spawn(|_, _| async move {
            client
                .request::<Next>(NextArguments {
                    thread_id,
                    granularity: supports_stepping_granularity.then(|| granularity),
                    single_thread: supports_single_thread_execution_requests.then(|| true),
                })
                .await
        })
    }

    pub fn step_in(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        granularity: SteppingGranularity,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        let capabilities = self.capabilities_by_id(client_id);

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        cx.spawn(|_, _| async move {
            client
                .request::<StepIn>(StepInArguments {
                    thread_id,
                    granularity: supports_stepping_granularity.then(|| granularity),
                    single_thread: supports_single_thread_execution_requests.then(|| true),
                    target_id: None,
                })
                .await
        })
    }

    pub fn step_out(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        granularity: SteppingGranularity,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        let capabilities = self.capabilities_by_id(client_id);

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        cx.spawn(|_, _| async move {
            client
                .request::<StepOut>(StepOutArguments {
                    thread_id,
                    granularity: supports_stepping_granularity.then(|| granularity),
                    single_thread: supports_single_thread_execution_requests.then(|| true),
                })
                .await
        })
    }

    pub fn variables(
        &self,
        client_id: &DebugAdapterClientId,
        variables_reference: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Variable>>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move {
            Ok(client
                .request::<Variables>(VariablesArguments {
                    variables_reference,
                    filter: None,
                    start: None,
                    count: None,
                    format: None,
                })
                .await?
                .variables)
        })
    }

    pub fn evaluate(
        &self,
        client_id: &DebugAdapterClientId,
        stack_frame_id: u64,
        expression: String,
        context: EvaluateArgumentsContext,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<EvaluateResponse>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move {
            client
                .request::<Evaluate>(EvaluateArguments {
                    expression: expression.clone(),
                    frame_id: Some(stack_frame_id),
                    context: Some(context),
                    format: None,
                    line: None,
                    column: None,
                    source: None,
                })
                .await
        })
    }

    pub fn completions(
        &self,
        client_id: &DebugAdapterClientId,
        stack_frame_id: u64,
        text: String,
        completion_column: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<CompletionItem>>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move {
            Ok(client
                .request::<Completions>(CompletionsArguments {
                    frame_id: Some(stack_frame_id),
                    line: None,
                    text,
                    column: completion_column,
                })
                .await?
                .targets)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_variable_value(
        &self,
        client_id: &DebugAdapterClientId,
        stack_frame_id: u64,
        variables_reference: u64,
        name: String,
        value: String,
        evaluate_name: Option<String>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        let supports_set_expression = self
            .capabilities_by_id(client_id)
            .supports_set_expression
            .unwrap_or_default();

        cx.spawn(|_, _| async move {
            if let Some(evaluate_name) = supports_set_expression.then(|| evaluate_name).flatten() {
                client
                    .request::<SetExpression>(SetExpressionArguments {
                        expression: evaluate_name,
                        value,
                        frame_id: Some(stack_frame_id),
                        format: None,
                    })
                    .await?;
            } else {
                client
                    .request::<SetVariable>(SetVariableArguments {
                        variables_reference,
                        name,
                        value,
                        format: None,
                    })
                    .await?;
            }

            Ok(())
        })
    }

    pub fn pause_thread(
        &mut self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move { client.request::<Pause>(PauseArguments { thread_id }).await })
    }

    pub fn terminate_threads(
        &mut self,
        client_id: &DebugAdapterClientId,
        thread_ids: Option<Vec<u64>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        let capabilities = self.capabilities_by_id(client_id);

        if capabilities
            .supports_terminate_threads_request
            .unwrap_or_default()
        {
            cx.spawn(|_, _| async move {
                client
                    .request::<TerminateThreads>(TerminateThreadsArguments { thread_ids })
                    .await
            })
        } else {
            self.shutdown_client(client_id, cx)
        }
    }

    pub fn disconnect_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move {
            client
                .request::<Disconnect>(DisconnectArguments {
                    restart: Some(false),
                    terminate_debuggee: Some(true),
                    suspend_debuggee: Some(false),
                })
                .await
        })
    }

    pub fn restart(
        &mut self,
        client_id: &DebugAdapterClientId,
        args: Option<Value>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        let restart_args = args.unwrap_or(Value::Null);

        cx.spawn(|_, _| async move {
            client
                .request::<Disconnect>(DisconnectArguments {
                    restart: Some(true),
                    terminate_debuggee: Some(false),
                    suspend_debuggee: Some(false),
                })
                .await?;

            match client.request_type() {
                DebugRequestType::Launch => {
                    client
                        .request::<Launch>(LaunchRequestArguments { raw: restart_args })
                        .await?
                }
                DebugRequestType::Attach => {
                    client
                        .request::<Attach>(AttachRequestArguments { raw: restart_args })
                        .await?
                }
            }

            Ok(())
        })
    }

    pub fn shutdown_clients(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        let mut tasks = Vec::new();

        for client_id in self.clients.keys().cloned().collect::<Vec<_>>() {
            tasks.push(self.shutdown_client(&client_id, cx));
        }

        cx.background_executor().spawn(async move {
            futures::future::join_all(tasks).await;
        })
    }

    pub fn shutdown_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.clients.remove(&client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.emit(DapStoreEvent::DebugClientStopped(*client_id));

        let capabilities = self.capabilities.remove(client_id);

        cx.notify();

        cx.spawn(|_, _| async move {
            let client = match client {
                DebugAdapterClientState::Starting(task) => task.await,
                DebugAdapterClientState::Running(client) => Some(client),
            };

            let Some(client) = client else {
                return Ok(());
            };

            if capabilities
                .and_then(|c| c.supports_terminate_request)
                .unwrap_or_default()
            {
                let _ = client
                    .request::<Terminate>(TerminateArguments {
                        restart: Some(false),
                    })
                    .await;
            }

            client.shutdown().await
        })
    }

    pub fn toggle_breakpoint_for_buffer(
        &mut self,
        project_path: &ProjectPath,
        breakpoint: Breakpoint,
        buffer_path: PathBuf,
        buffer_snapshot: BufferSnapshot,
        edit_action: BreakpointEditAction,
        cx: &mut ModelContext<Self>,
    ) {
        let breakpoint_set = self.breakpoints.entry(project_path.clone()).or_default();

        match edit_action {
            BreakpointEditAction::Toggle => {
                if !breakpoint_set.remove(&breakpoint) {
                    breakpoint_set.insert(breakpoint);
                }
            }
            BreakpointEditAction::EditLogMessage => {
                breakpoint_set.remove(&breakpoint);
                breakpoint_set.insert(breakpoint);
            }
        }

        cx.notify();

        self.send_changed_breakpoints(project_path, buffer_path, buffer_snapshot, cx)
            .detach();
    }

    pub fn send_breakpoints(
        &self,
        client_id: &DebugAdapterClientId,
        absolute_file_path: Arc<Path>,
        breakpoints: Vec<SourceBreakpoint>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Could not found client")));
        };

        cx.spawn(|_, _| async move {
            client
                .request::<SetBreakpoints>(SetBreakpointsArguments {
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
                .await?;

            Ok(())
        })
    }

    pub fn send_changed_breakpoints(
        &self,
        project_path: &ProjectPath,
        buffer_path: PathBuf,
        buffer_snapshot: BufferSnapshot,
        cx: &mut ModelContext<Self>,
    ) -> Task<()> {
        let clients = self.running_clients().collect::<Vec<_>>();

        if clients.is_empty() {
            return Task::ready(());
        }

        let Some(breakpoints) = self.breakpoints.get(project_path) else {
            return Task::ready(());
        };

        let source_breakpoints = breakpoints
            .iter()
            .map(|bp| bp.source_for_snapshot(&buffer_snapshot))
            .collect::<Vec<_>>();

        let mut tasks = Vec::new();
        for client in clients {
            tasks.push(self.send_breakpoints(
                &client.id(),
                Arc::from(buffer_path.clone()),
                source_breakpoints.clone(),
                cx,
            ))
        }

        cx.background_executor().spawn(async move {
            futures::future::join_all(tasks).await;
        })
    }
}

type LogMessage = Arc<str>;

#[derive(Clone, Debug)]
pub enum BreakpointEditAction {
    Toggle,
    EditLogMessage,
}

#[derive(Clone, Debug)]
pub enum BreakpointKind {
    Standard,
    Log(LogMessage),
}

impl BreakpointKind {
    pub fn to_int(&self) -> i32 {
        match self {
            BreakpointKind::Standard => 0,
            BreakpointKind::Log(_) => 1,
        }
    }

    pub fn log_message(&self) -> Option<LogMessage> {
        match self {
            BreakpointKind::Standard => None,
            BreakpointKind::Log(message) => Some(message.clone()),
        }
    }
}

impl PartialEq for BreakpointKind {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for BreakpointKind {}

impl Hash for BreakpointKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct Breakpoint {
    pub active_position: Option<text::Anchor>,
    pub cache_position: u32,
    pub kind: BreakpointKind,
}

// Custom implementation for PartialEq, Eq, and Hash is done
// to get toggle breakpoint to solely be based on a breakpoint's
// location. Otherwise, a user can get in situation's where there's
// overlapping breakpoint's with them being aware.
impl PartialEq for Breakpoint {
    fn eq(&self, other: &Self) -> bool {
        match (&self.active_position, &other.active_position) {
            (None, None) => self.cache_position == other.cache_position,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(self_position), Some(other_position)) => self_position == other_position,
        }
    }
}

impl Eq for Breakpoint {}

impl Hash for Breakpoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.active_position.is_some() {
            self.active_position.hash(state);
        } else {
            self.cache_position.hash(state);
        }
    }
}

impl Breakpoint {
    pub fn to_source_breakpoint(&self, buffer: &Buffer) -> SourceBreakpoint {
        let line = self
            .active_position
            .map(|position| buffer.summary_for_anchor::<Point>(&position).row)
            .unwrap_or(self.cache_position) as u64;

        SourceBreakpoint {
            line,
            condition: None,
            hit_condition: None,
            log_message: None,
            column: None,
            mode: None,
        }
    }

    pub fn set_active_position(&mut self, buffer: &Buffer) {
        if self.active_position.is_none() {
            self.active_position =
                Some(buffer.breakpoint_anchor(Point::new(self.cache_position, 0)));
        }
    }

    pub fn point_for_buffer(&self, buffer: &Buffer) -> Point {
        self.active_position
            .map(|position| buffer.summary_for_anchor::<Point>(&position))
            .unwrap_or(Point::new(self.cache_position, 0))
    }

    pub fn point_for_buffer_snapshot(&self, buffer_snapshot: &BufferSnapshot) -> Point {
        self.active_position
            .map(|position| buffer_snapshot.summary_for_anchor::<Point>(&position))
            .unwrap_or(Point::new(self.cache_position, 0))
    }

    pub fn source_for_snapshot(&self, snapshot: &BufferSnapshot) -> SourceBreakpoint {
        let line = self
            .active_position
            .map(|position| snapshot.summary_for_anchor::<Point>(&position).row)
            .unwrap_or(self.cache_position) as u64;

        let log_message = match &self.kind {
            BreakpointKind::Standard => None,
            BreakpointKind::Log(log_message) => Some(log_message.clone().to_string()),
        };

        SourceBreakpoint {
            line,
            condition: None,
            hit_condition: None,
            log_message,
            column: None,
            mode: None,
        }
    }

    pub fn to_serialized(&self, buffer: Option<&Buffer>, path: Arc<Path>) -> SerializedBreakpoint {
        match buffer {
            Some(buffer) => SerializedBreakpoint {
                position: self
                    .active_position
                    .map(|position| buffer.summary_for_anchor::<Point>(&position).row + 1u32)
                    .unwrap_or(self.cache_position),
                path,
                kind: self.kind.clone(),
            },
            None => SerializedBreakpoint {
                position: self.cache_position,
                path,
                kind: self.kind.clone(),
            },
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SerializedBreakpoint {
    pub position: u32,
    pub path: Arc<Path>,
    pub kind: BreakpointKind,
}

impl SerializedBreakpoint {
    pub fn to_source_breakpoint(&self) -> SourceBreakpoint {
        let log_message = match &self.kind {
            BreakpointKind::Standard => None,
            BreakpointKind::Log(message) => Some(message.clone().to_string()),
        };

        SourceBreakpoint {
            line: self.position as u64,
            condition: None,
            hit_condition: None,
            log_message,
            column: None,
            mode: None,
        }
    }
}

pub struct DapAdapterDelegate {
    fs: Arc<dyn Fs>,
    http_client: Option<Arc<dyn HttpClient>>,
    node_runtime: Option<NodeRuntime>,
}

impl DapAdapterDelegate {
    pub fn new(
        http_client: Option<Arc<dyn HttpClient>>,
        node_runtime: Option<NodeRuntime>,
        fs: Arc<dyn Fs>,
    ) -> Self {
        Self {
            fs,
            http_client,
            node_runtime,
        }
    }
}

impl dap::adapters::DapDelegate for DapAdapterDelegate {
    fn http_client(&self) -> Option<Arc<dyn HttpClient>> {
        self.http_client.clone()
    }

    fn node_runtime(&self) -> Option<NodeRuntime> {
        self.node_runtime.clone()
    }

    fn fs(&self) -> Arc<dyn Fs> {
        self.fs.clone()
    }
}
