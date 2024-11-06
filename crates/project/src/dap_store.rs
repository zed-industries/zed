use crate::{project_settings::ProjectSettings, ProjectEnvironment, ProjectPath};
use anyhow::{anyhow, Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use dap::adapters::{DapDelegate, DapStatus, DebugAdapterName};
use dap::{
    client::{DebugAdapterClient, DebugAdapterClientId},
    messages::{Message, Response},
    requests::{
        Attach, Completions, ConfigurationDone, Continue, Disconnect, Evaluate, Initialize, Launch,
        LoadedSources, Modules, Next, Pause, Request as _, Restart, RunInTerminal, Scopes,
        SetBreakpoints, SetExpression, SetVariable, StackTrace, StartDebugging, StepIn, StepOut,
        Terminate, TerminateThreads, Variables,
    },
    AttachRequestArguments, Capabilities, CompletionItem, CompletionsArguments,
    ConfigurationDoneArguments, ContinueArguments, DisconnectArguments, ErrorResponse,
    EvaluateArguments, EvaluateArgumentsContext, EvaluateResponse, InitializeRequestArguments,
    InitializeRequestArgumentsPathFormat, LaunchRequestArguments, LoadedSourcesArguments, Module,
    ModulesArguments, NextArguments, PauseArguments, RestartArguments, RunInTerminalResponse,
    Scope, ScopesArguments, SetBreakpointsArguments, SetExpressionArguments, SetVariableArguments,
    Source, SourceBreakpoint, StackFrame, StackTraceArguments, StartDebuggingRequestArguments,
    StartDebuggingRequestArgumentsRequest, StepInArguments, StepOutArguments, SteppingGranularity,
    TerminateArguments, TerminateThreadsArguments, Variable, VariablesArguments,
};
use dap_adapters::build_adapter;
use fs::Fs;
use futures::future::Shared;
use futures::FutureExt;
use gpui::{EventEmitter, Model, ModelContext, SharedString, Task};
use http_client::HttpClient;
use language::{
    Buffer, BufferSnapshot, LanguageRegistry, LanguageServerBinaryStatus, LanguageServerName,
};
use node_runtime::NodeRuntime;
use serde_json::Value;
use settings::{Settings, WorktreeId};
use smol::lock::Mutex;
use std::{
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use task::{AttachConfig, DebugAdapterConfig, DebugRequestType};
use text::Point;
use util::{maybe, merge_json_value_into, ResultExt as _};

pub enum DapStoreEvent {
    DebugClientStarted(DebugAdapterClientId),
    DebugClientStopped(DebugAdapterClientId),
    DebugClientEvent {
        client_id: DebugAdapterClientId,
        message: Message,
    },
    Notification(String),
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
    delegate: DapAdapterDelegate,
    breakpoints: BTreeMap<ProjectPath, HashSet<Breakpoint>>,
    active_debug_line: Option<(ProjectPath, DebugPosition)>,
    capabilities: HashMap<DebugAdapterClientId, Capabilities>,
    environment: Model<ProjectEnvironment>,
    clients: HashMap<DebugAdapterClientId, DebugAdapterClientState>,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    pub fn new(
        http_client: Option<Arc<dyn HttpClient>>,
        node_runtime: Option<NodeRuntime>,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        environment: Model<ProjectEnvironment>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        cx.on_app_quit(Self::shutdown_clients).detach();

        let load_shell_env_task = Task::ready(None).shared();

        Self {
            active_debug_line: None,
            clients: Default::default(),
            breakpoints: Default::default(),
            capabilities: HashMap::default(),
            next_client_id: Default::default(),
            delegate: DapAdapterDelegate::new(
                http_client.clone(),
                node_runtime.clone(),
                fs.clone(),
                languages.clone(),
                load_shell_env_task,
            ),
            environment,
        }
    }

    pub fn delegate(
        &self,
        cwd: &Option<PathBuf>,
        cx: &mut ModelContext<Self>,
    ) -> Arc<DapAdapterDelegate> {
        let worktree_abs_path = cwd.as_ref().map(|p| Arc::from(p.as_path()));
        let task = self.environment.update(cx, |env, cx| {
            env.get_environment(None, worktree_abs_path, cx)
        });

        let mut delegate = self.delegate.clone();
        delegate.refresh_shell_env_task(task);

        Arc::new(delegate)
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

    pub fn start_client(&mut self, config: DebugAdapterConfig, cx: &mut ModelContext<Self>) {
        let client_id = self.next_client_id();
        let adapter_delegate = self.delegate(&config.cwd, cx);

        let start_client_task = cx.spawn(|this, mut cx| async move {
            let dap_store = this.clone();
            let client = maybe!(async {
                let adapter = Arc::new(
                    build_adapter(&config.kind)
                        .await
                        .context("Creating debug adapter")?,
                );

                if !adapter.supports_attach()
                    && matches!(config.request, DebugRequestType::Attach(_))
                {
                    return Err(anyhow!("Debug adapter does not support `attach` request"));
                }

                let path = cx.update(|cx| {
                    let name = LanguageServerName::from(adapter.name().as_ref());

                    ProjectSettings::get_global(cx)
                        .dap
                        .get(&name)
                        .and_then(|s| s.path.as_ref().map(PathBuf::from))
                })?;

                let binary = match adapter
                    .get_binary(adapter_delegate.as_ref(), &config, path)
                    .await
                {
                    Err(error) => {
                        adapter_delegate.update_status(
                            adapter.name(),
                            DapStatus::Failed {
                                error: error.to_string(),
                            },
                        );

                        return Err(error);
                    }
                    Ok(mut binary) => {
                        adapter_delegate.update_status(adapter.name(), DapStatus::None);

                        let shell_env = adapter_delegate.shell_env().await;
                        let mut envs = binary.envs.unwrap_or_default();
                        envs.extend(shell_env);
                        binary.envs = Some(envs);

                        binary
                    }
                };

                let mut client = DebugAdapterClient::new(client_id, config, adapter);

                client
                    .start(
                        &binary,
                        move |message, cx| {
                            dap_store
                                .update(cx, |_, cx| {
                                    cx.emit(DapStoreEvent::DebugClientEvent { client_id, message })
                                })
                                .log_err();
                        },
                        &mut cx,
                    )
                    .await?;

                anyhow::Ok(client)
            })
            .await;

            let client = match client {
                Err(error) => {
                    this.update(&mut cx, |_, cx| {
                        cx.emit(DapStoreEvent::Notification(error.to_string()));
                    })
                    .log_err()?;

                    return None;
                }
                Ok(client) => Arc::new(client),
            };

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
            })
        })
    }

    pub fn launch(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        let mut adapter_args = client.adapter().request_args(&client.config());

        if let Some(args) = client.config().initialize_args.clone() {
            merge_json_value_into(args, &mut adapter_args);
        }

        cx.background_executor().spawn(async move {
            client
                .request::<Launch>(LaunchRequestArguments { raw: adapter_args })
                .await
        })
    }

    pub fn attach(
        &mut self,
        client_id: &DebugAdapterClientId,
        pid: u32,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self.client_by_id(client_id) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        // update the process id on the config, so when the `startDebugging` reverse request
        // comes in we send another `attach` request with the already selected PID
        // If we don't do this the user has to select the process twice if the adapter sends a `startDebugging` request
        client.set_process_id(pid);

        let config = client.config();
        let mut adapter_args = client.adapter().request_args(&config);

        if let Some(args) = config.initialize_args.clone() {
            merge_json_value_into(args, &mut adapter_args);
        }

        cx.background_executor().spawn(async move {
            client
                .request::<Attach>(AttachRequestArguments { raw: adapter_args })
                .await
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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
        args: StartDebuggingRequestArguments,
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

            let config = client.config();

            this.update(&mut cx, |store, cx| {
                store.start_client(
                    DebugAdapterConfig {
                        kind: config.kind.clone(),
                        request: match args.request {
                            StartDebuggingRequestArgumentsRequest::Launch => {
                                DebugRequestType::Launch
                            }
                            StartDebuggingRequestArgumentsRequest::Attach => {
                                DebugRequestType::Attach(
                                    if let DebugRequestType::Attach(attach_config) = config.request
                                    {
                                        attach_config
                                    } else {
                                        AttachConfig::default()
                                    },
                                )
                            }
                        },
                        program: config.program.clone(),
                        cwd: config.cwd.clone(),
                        initialize_args: Some(args.configuration),
                    },
                    cx,
                );
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

        cx.background_executor().spawn(async move {
            client
                .send_message(Message::Response(Response {
                    seq,
                    request_seq: seq,
                    success,
                    command: RunInTerminal::COMMAND.to_string(),
                    body: match success {
                        true => Some(serde_json::to_value(RunInTerminalResponse {
                            process_id: Some(std::process::id() as u64),
                            shell_process_id: shell_pid,
                        })?),
                        false => Some(serde_json::to_value(ErrorResponse { error: None })?),
                    },
                }))
                .await
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor()
            .spawn(async move { client.request::<Pause>(PauseArguments { thread_id }).await })
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
            cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

        let supports_restart = self
            .capabilities_by_id(client_id)
            .supports_restart_request
            .unwrap_or_default();

        let raw = args.unwrap_or(Value::Null);

        cx.background_executor().spawn(async move {
            if supports_restart {
                client.request::<Restart>(RestartArguments { raw }).await?;
            } else {
                client
                    .request::<Disconnect>(DisconnectArguments {
                        restart: Some(false),
                        terminate_debuggee: Some(true),
                        suspend_debuggee: Some(false),
                    })
                    .await?;
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

        cx.background_executor().spawn(async move {
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

        cx.background_executor().spawn(async move {
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

#[derive(Clone)]
pub struct DapAdapterDelegate {
    fs: Arc<dyn Fs>,
    http_client: Option<Arc<dyn HttpClient>>,
    node_runtime: Option<NodeRuntime>,
    updated_adapters: Arc<Mutex<HashSet<DebugAdapterName>>>,
    languages: Arc<LanguageRegistry>,
    load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
}

impl DapAdapterDelegate {
    pub fn new(
        http_client: Option<Arc<dyn HttpClient>>,
        node_runtime: Option<NodeRuntime>,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
    ) -> Self {
        Self {
            fs,
            languages,
            http_client,
            node_runtime,
            load_shell_env_task,
            updated_adapters: Default::default(),
        }
    }

    pub(crate) fn refresh_shell_env_task(
        &mut self,
        load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
    ) {
        self.load_shell_env_task = load_shell_env_task;
    }
}

#[async_trait(?Send)]
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

    fn updated_adapters(&self) -> Arc<Mutex<HashSet<DebugAdapterName>>> {
        self.updated_adapters.clone()
    }

    fn update_status(&self, dap_name: DebugAdapterName, status: dap::adapters::DapStatus) {
        let name = SharedString::from(dap_name.to_string());
        let status = match status {
            DapStatus::None => LanguageServerBinaryStatus::None,
            DapStatus::Downloading => LanguageServerBinaryStatus::Downloading,
            DapStatus::Failed { error } => LanguageServerBinaryStatus::Failed { error },
            DapStatus::CheckingForUpdate => LanguageServerBinaryStatus::CheckingForUpdate,
        };

        self.languages
            .update_dap_status(LanguageServerName(name), status);
    }

    fn which(&self, command: &OsStr) -> Option<PathBuf> {
        which::which(command).ok()
    }

    async fn shell_env(&self) -> HashMap<String, String> {
        let task = self.load_shell_env_task.clone();
        task.await.unwrap_or_default()
    }
}
