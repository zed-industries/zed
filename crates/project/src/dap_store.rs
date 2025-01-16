use crate::{
    dap_command::{
        ContinueCommand, DapCommand, DisconnectCommand, NextCommand, PauseCommand, RestartCommand,
        RestartStackFrameCommand, StepBackCommand, StepCommand, StepInCommand, StepOutCommand,
        TerminateCommand, TerminateThreadsCommand,
    },
    project_settings::ProjectSettings,
    ProjectEnvironment, ProjectItem as _, ProjectPath,
};
use anyhow::{anyhow, bail, Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use dap::{
    adapters::{DapDelegate, DapStatus, DebugAdapter, DebugAdapterBinary, DebugAdapterName},
    client::{DebugAdapterClient, DebugAdapterClientId},
    messages::{Message, Response},
    requests::{
        Attach, Completions, ConfigurationDone, Disconnect, Evaluate, Initialize, Launch,
        LoadedSources, Modules, Request as _, RunInTerminal, Scopes, SetBreakpoints, SetExpression,
        SetVariable, StackTrace, StartDebugging, Terminate, Variables,
    },
    AttachRequestArguments, Capabilities, CompletionItem, CompletionsArguments,
    ConfigurationDoneArguments, ContinueArguments, DisconnectArguments, ErrorResponse,
    EvaluateArguments, EvaluateArgumentsContext, EvaluateResponse, InitializeRequestArguments,
    InitializeRequestArgumentsPathFormat, LaunchRequestArguments, LoadedSourcesArguments, Module,
    ModulesArguments, Scope, ScopesArguments, SetBreakpointsArguments, SetExpressionArguments,
    SetVariableArguments, Source, SourceBreakpoint, StackFrame, StackTraceArguments,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, SteppingGranularity,
    TerminateArguments, Variable, VariablesArguments,
};
use dap::{
    session::{DebugSession, DebugSessionId},
    ContinueResponse,
};
use dap_adapters::build_adapter;
use fs::Fs;
use futures::future::Shared;
use futures::FutureExt;
use gpui::{AsyncAppContext, Context, EventEmitter, Model, ModelContext, SharedString, Task};
use http_client::HttpClient;
use language::{
    proto::{deserialize_anchor, serialize_anchor as serialize_text_anchor},
    Buffer, BufferSnapshot, LanguageRegistry, LanguageServerBinaryStatus,
};
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;
use rpc::{
    proto::{self, SetDebuggerPanelItem, UpdateDebugAdapter, UpdateThreadStatus},
    AnyProtoClient, TypedEnvelope,
};
use serde_json::Value;
use settings::{Settings as _, WorktreeId};
use smol::lock::Mutex;
use std::collections::VecDeque;
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
use util::{merge_json_value_into, ResultExt as _};

pub enum DapStoreEvent {
    DebugClientStarted((DebugSessionId, DebugAdapterClientId)),
    DebugClientShutdown(DebugAdapterClientId),
    DebugClientEvent {
        session_id: DebugSessionId,
        client_id: DebugAdapterClientId,
        message: Message,
    },
    Notification(String),
    BreakpointsChanged(ProjectPath),
    ActiveDebugLineChanged,
    SetDebugPanelItem(SetDebuggerPanelItem),
    UpdateDebugAdapter(UpdateDebugAdapter),
    UpdateThreadStatus(UpdateThreadStatus),
}

#[allow(clippy::large_enum_variant)]
pub enum DapStoreMode {
    Local(LocalDapStore),   // ssh host and collab host
    Remote(RemoteDapStore), // collab guest
}

pub struct LocalDapStore {
    next_client_id: AtomicUsize,
    next_session_id: AtomicUsize,
    delegate: DapAdapterDelegate,
    environment: Model<ProjectEnvironment>,
    sessions: HashMap<DebugSessionId, Model<DebugSession>>,
    client_by_session: HashMap<DebugAdapterClientId, DebugSessionId>,
}

impl LocalDapStore {
    fn next_client_id(&self) -> DebugAdapterClientId {
        DebugAdapterClientId(self.next_client_id.fetch_add(1, SeqCst))
    }

    fn next_session_id(&self) -> DebugSessionId {
        DebugSessionId(self.next_session_id.fetch_add(1, SeqCst))
    }

    pub fn session_by_client_id(
        &self,
        client_id: &DebugAdapterClientId,
    ) -> Option<Model<DebugSession>> {
        self.sessions
            .get(self.client_by_session.get(client_id)?)
            .cloned()
    }
}

pub struct RemoteDapStore {
    upstream_client: Option<AnyProtoClient>,
    upstream_project_id: u64,
    event_queue: Option<VecDeque<DapStoreEvent>>,
}

pub struct DapStore {
    mode: DapStoreMode,
    downstream_client: Option<(AnyProtoClient, u64)>,
    breakpoints: BTreeMap<ProjectPath, HashSet<Breakpoint>>,
    capabilities: HashMap<DebugAdapterClientId, Capabilities>,
    active_debug_line: Option<(DebugAdapterClientId, ProjectPath, u32)>,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    const INDEX_STARTS_AT_ONE: bool = true;

    pub fn init(client: &AnyProtoClient) {
        client.add_model_message_handler(DapStore::handle_remove_active_debug_line);
        client.add_model_message_handler(DapStore::handle_shutdown_debug_client);
        client.add_model_message_handler(DapStore::handle_set_active_debug_line);
        client.add_model_message_handler(DapStore::handle_set_debug_client_capabilities);
        client.add_model_message_handler(DapStore::handle_set_debug_panel_item);
        client.add_model_message_handler(DapStore::handle_synchronize_breakpoints);
        client.add_model_message_handler(DapStore::handle_update_debug_adapter);
        client.add_model_message_handler(DapStore::handle_update_thread_status);

        client.add_model_request_handler(DapStore::handle_dap_command::<NextCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<StepInCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<StepOutCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<StepBackCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<ContinueCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<PauseCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<DisconnectCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<TerminateThreadsCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<TerminateCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<RestartCommand>);
        client.add_model_request_handler(DapStore::handle_dap_command::<RestartStackFrameCommand>);
        client.add_model_request_handler(DapStore::handle_shutdown_session);
    }

    pub fn new_local(
        http_client: Arc<dyn HttpClient>,
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        environment: Model<ProjectEnvironment>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        cx.on_app_quit(Self::shutdown_sessions).detach();

        Self {
            mode: DapStoreMode::Local(LocalDapStore {
                environment,
                sessions: HashMap::default(),
                next_client_id: Default::default(),
                next_session_id: Default::default(),
                delegate: DapAdapterDelegate::new(
                    Some(http_client.clone()),
                    Some(node_runtime.clone()),
                    fs.clone(),
                    languages.clone(),
                    Task::ready(None).shared(),
                ),
                client_by_session: Default::default(),
            }),
            downstream_client: None,
            active_debug_line: None,
            breakpoints: Default::default(),
            capabilities: Default::default(),
        }
    }

    pub fn new_remote(
        project_id: u64,
        upstream_client: AnyProtoClient,
        _: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            mode: DapStoreMode::Remote(RemoteDapStore {
                upstream_client: Some(upstream_client),
                upstream_project_id: project_id,
                event_queue: Some(VecDeque::default()),
            }),
            downstream_client: None,
            active_debug_line: None,
            breakpoints: Default::default(),
            capabilities: Default::default(),
        }
    }

    pub fn as_remote(&self) -> Option<&RemoteDapStore> {
        match &self.mode {
            DapStoreMode::Remote(remote_dap_store) => Some(remote_dap_store),
            _ => None,
        }
    }

    pub fn remote_event_queue(&mut self) -> Option<VecDeque<DapStoreEvent>> {
        if let DapStoreMode::Remote(remote) = &mut self.mode {
            remote.event_queue.take()
        } else {
            None
        }
    }

    pub fn as_local(&self) -> Option<&LocalDapStore> {
        match &self.mode {
            DapStoreMode::Local(local_dap_store) => Some(local_dap_store),
            _ => None,
        }
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalDapStore> {
        match &mut self.mode {
            DapStoreMode::Local(local_dap_store) => Some(local_dap_store),
            _ => None,
        }
    }

    pub fn upstream_client(&self) -> Option<(AnyProtoClient, u64)> {
        match &self.mode {
            DapStoreMode::Remote(RemoteDapStore {
                upstream_client: Some(upstream_client),
                upstream_project_id,
                ..
            }) => Some((upstream_client.clone(), *upstream_project_id)),

            DapStoreMode::Remote(RemoteDapStore {
                upstream_client: None,
                ..
            }) => None,
            DapStoreMode::Local(_) => None,
        }
    }

    pub fn downstream_client(&self) -> Option<&(AnyProtoClient, u64)> {
        self.downstream_client.as_ref()
    }

    pub fn sessions(&self) -> impl Iterator<Item = Model<DebugSession>> + '_ {
        self.as_local().unwrap().sessions.values().cloned()
    }

    pub fn session_by_id(&self, session_id: &DebugSessionId) -> Option<Model<DebugSession>> {
        self.as_local()
            .and_then(|store| store.sessions.get(session_id).cloned())
    }

    pub fn session_by_client_id(
        &self,
        client_id: &DebugAdapterClientId,
    ) -> Option<Model<DebugSession>> {
        self.as_local()
            .and_then(|store| store.session_by_client_id(client_id))
    }

    pub fn client_by_id(
        &self,
        client_id: &DebugAdapterClientId,
        cx: &ModelContext<Self>,
    ) -> Option<(Model<DebugSession>, Arc<DebugAdapterClient>)> {
        let session = self.session_by_client_id(client_id)?;
        let client = session.read(cx).client_by_id(client_id)?;

        Some((session, client))
    }

    pub fn capabilities_by_id(&self, client_id: &DebugAdapterClientId) -> Capabilities {
        self.capabilities
            .get(client_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn update_capabilities_for_client(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        capabilities: &Capabilities,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(old_capabilities) = self.capabilities.get_mut(client_id) {
            *old_capabilities = old_capabilities.merge(capabilities.clone());
        } else {
            self.capabilities.insert(*client_id, capabilities.clone());
        }

        cx.notify();

        if let Some((downstream_client, project_id)) = self.downstream_client.as_ref() {
            downstream_client
                .send(dap::proto_conversions::capabilities_to_proto(
                    &capabilities,
                    *project_id,
                    session_id.to_proto(),
                    client_id.to_proto(),
                ))
                .log_err();
        }
    }

    pub fn active_debug_line(&self) -> Option<(DebugAdapterClientId, ProjectPath, u32)> {
        self.active_debug_line.clone()
    }

    pub fn set_active_debug_line(
        &mut self,
        client_id: &DebugAdapterClientId,
        project_path: &ProjectPath,
        row: u32,
        cx: &mut ModelContext<Self>,
    ) {
        self.active_debug_line = Some((*client_id, project_path.clone(), row));
        cx.emit(DapStoreEvent::ActiveDebugLineChanged);
        cx.notify();
    }

    pub fn remove_active_debug_line_for_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(active_line) = &self.active_debug_line {
            if active_line.0 == *client_id {
                self.active_debug_line.take();
                cx.emit(DapStoreEvent::ActiveDebugLineChanged);
                cx.notify();

                if let Some((client, project_id)) = self.downstream_client.clone() {
                    client
                        .send(client::proto::RemoveActiveDebugLine { project_id })
                        .log_err();
                }
            }
        }
    }

    pub fn on_file_rename(&mut self, old_project_path: ProjectPath, new_project_path: ProjectPath) {
        if let Some(breakpoints) = self.breakpoints.remove(&old_project_path) {
            self.breakpoints.insert(new_project_path, breakpoints);
        }
    }

    pub fn breakpoints(&self) -> &BTreeMap<ProjectPath, HashSet<Breakpoint>> {
        &self.breakpoints
    }

    pub fn ignore_breakpoints(&self, session_id: &DebugSessionId, cx: &ModelContext<Self>) -> bool {
        self.session_by_id(session_id)
            .map(|session| session.read(cx).ignore_breakpoints())
            .unwrap_or_default()
    }

    pub fn toggle_ignore_breakpoints(
        &mut self,
        session_id: &DebugSessionId,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(session) = self.session_by_id(session_id) {
            session.update(cx, |session, cx| {
                session.set_ignore_breakpoints(!session.ignore_breakpoints(), cx);
            });
        }
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
                    cached_position: serialize_breakpoint.position,
                    kind: serialize_breakpoint.kind,
                });
        }
    }

    pub fn sync_open_breakpoints_to_closed_breakpoints(
        &mut self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(project_path) = buffer.read(cx).project_path(cx) else {
            return;
        };

        if let Some(breakpoint_set) = self.breakpoints.remove(&project_path) {
            let breakpoint_iter = breakpoint_set.into_iter().map(|mut bp| {
                bp.cached_position = bp.point_for_buffer(buffer.read(cx)).row;
                bp.active_position = None;
                bp
            });

            self.breakpoints
                .insert(project_path, breakpoint_iter.collect::<HashSet<_>>());

            cx.notify();
        }
    }

    fn reconnect_client(
        &mut self,
        session_id: &DebugSessionId,
        adapter: Arc<dyn DebugAdapter>,
        binary: DebugAdapterBinary,
        config: DebugAdapterConfig,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !adapter.supports_attach() && matches!(config.request, DebugRequestType::Attach(_)) {
            return Task::ready(Err(anyhow!(
                "Debug adapter does not support `attach` request"
            )));
        }

        let session_id = *session_id;
        let client_id = self.as_local().unwrap().next_client_id();

        cx.spawn(|dap_store, mut cx| async move {
            let mut client = DebugAdapterClient::new(client_id, adapter, binary, &cx);

            client
                .reconnect(
                    {
                        let dap_store = dap_store.clone();
                        move |message, cx| {
                            dap_store
                                .update(cx, |_, cx| {
                                    cx.emit(DapStoreEvent::DebugClientEvent {
                                        session_id,
                                        client_id,
                                        message,
                                    })
                                })
                                .log_err();
                        }
                    },
                    &mut cx,
                )
                .await?;

            dap_store.update(&mut cx, |store, cx| {
                store
                    .as_local_mut()
                    .unwrap()
                    .client_by_session
                    .insert(client_id, session_id);

                let session = store.session_by_id(&session_id).unwrap();

                session.update(cx, |session, cx| {
                    session.update_configuration(
                        |old_config| {
                            *old_config = config.clone();
                        },
                        cx,
                    );
                    session.add_client(Arc::new(client), cx);
                });

                // don't emit this event ourself in tests, so we can add request,
                // response and event handlers for this client
                if !cfg!(any(test, feature = "test-support")) {
                    cx.emit(DapStoreEvent::DebugClientStarted((session_id, client_id)));
                }

                cx.notify();
            })
        })
    }

    fn start_client_internal(
        &mut self,
        session_id: DebugSessionId,
        config: DebugAdapterConfig,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Arc<DebugAdapterClient>>> {
        let Some(local_store) = self.as_local_mut() else {
            return Task::ready(Err(anyhow!("cannot start client on remote side")));
        };

        let mut adapter_delegate = local_store.delegate.clone();
        let worktree_abs_path = config.cwd.as_ref().map(|p| Arc::from(p.as_path()));
        adapter_delegate.refresh_shell_env_task(local_store.environment.update(cx, |env, cx| {
            env.get_environment(None, worktree_abs_path, cx)
        }));
        let adapter_delegate = Arc::new(adapter_delegate);

        let client_id = self.as_local().unwrap().next_client_id();

        cx.spawn(|this, mut cx| async move {
            let adapter = build_adapter(&config.kind).await?;

            if !adapter.supports_attach() && matches!(config.request, DebugRequestType::Attach(_)) {
                bail!("Debug adapter does not support `attach` request");
            }

            let binary = cx.update(|cx| {
                let name = DebugAdapterName::from(adapter.name().as_ref());

                ProjectSettings::get_global(cx)
                    .dap
                    .get(&name)
                    .and_then(|s| s.binary.as_ref().map(PathBuf::from))
            })?;

            let (adapter, binary) = match adapter
                .get_binary(adapter_delegate.as_ref(), &config, binary)
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

                    (adapter, binary)
                }
            };

            let mut client = DebugAdapterClient::new(client_id, adapter, binary, &cx);

            client
                .start(
                    {
                        let dap_store = this.clone();
                        move |message, cx| {
                            dap_store
                                .update(cx, |_, cx| {
                                    cx.emit(DapStoreEvent::DebugClientEvent {
                                        session_id,
                                        client_id,
                                        message,
                                    })
                                })
                                .log_err();
                        }
                    },
                    &mut cx,
                )
                .await?;

            Ok(Arc::new(client))
        })
    }

    pub fn start_debug_session(
        &mut self,
        config: DebugAdapterConfig,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(Model<DebugSession>, Arc<DebugAdapterClient>)>> {
        let Some(local_store) = self.as_local() else {
            return Task::ready(Err(anyhow!("cannot start session on remote side")));
        };

        let session_id = local_store.next_session_id();
        let start_client_task = self.start_client_internal(session_id, config.clone(), cx);

        cx.spawn(|this, mut cx| async move {
            let session = cx.new_model(|_| DebugSession::new(session_id, config))?;

            let client = match start_client_task.await {
                Ok(client) => client,
                Err(error) => {
                    this.update(&mut cx, |_, cx| {
                        cx.emit(DapStoreEvent::Notification(error.to_string()));
                    })
                    .log_err();

                    return Err(error);
                }
            };

            this.update(&mut cx, |store, cx| {
                session.update(cx, |session, cx| {
                    session.add_client(client.clone(), cx);
                });

                let client_id = client.id();

                let local_store = store.as_local_mut().unwrap();
                local_store.client_by_session.insert(client_id, session_id);
                local_store.sessions.insert(session_id, session.clone());

                cx.emit(DapStoreEvent::DebugClientStarted((session_id, client_id)));
                cx.notify();

                (session, client)
            })
        })
    }

    pub fn initialize(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!(
                "Could not find debug client: {:?} for session {:?}",
                client_id,
                session_id
            )));
        };

        let session_id = *session_id;
        let client_id = *client_id;

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
                    lines_start_at1: Some(Self::INDEX_STARTS_AT_ONE),
                    columns_start_at1: Some(Self::INDEX_STARTS_AT_ONE),
                    supports_memory_event: Some(false),
                    supports_args_can_be_interpreted_by_shell: Some(false),
                    supports_start_debugging_request: Some(true),
                })
                .await?;

            this.update(&mut cx, |store, cx| {
                store.update_capabilities_for_client(&session_id, &client_id, &capabilities, cx);
            })
        })
    }

    pub fn launch(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some((session, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!(
                "Could not find debug client: {:?} for session {:?}",
                client_id,
                session_id
            )));
        };

        let config = session.read(cx).configuration();
        let mut adapter_args = client.adapter().request_args(&config);
        if let Some(args) = config.initialize_args.clone() {
            merge_json_value_into(args, &mut adapter_args);
        }

        // TODO(debugger): GDB starts the debuggee program on launch instead of configurationDone
        // causing our sent breakpoints to not be valid. This delay should eventually be taken out
        let delay = if &client.adapter_id() == "gdb" {
            Some(
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(20u64)),
            )
        } else {
            None
        };

        cx.background_executor().spawn(async move {
            if let Some(delay) = delay {
                delay.await;
            }

            client
                .request::<Launch>(LaunchRequestArguments { raw: adapter_args })
                .await
        })
    }

    pub fn attach(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        process_id: u32,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some((session, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!(
                "Could not find debug client: {:?} for session {:?}",
                client_id,
                session_id
            )));
        };

        // update the process id on the config, so when the `startDebugging` reverse request
        // comes in we send another `attach` request with the already selected PID
        // If we don't do this the user has to select the process twice if the adapter sends a `startDebugging` request
        session.update(cx, |session, cx| {
            session.update_configuration(
                |config| {
                    config.request = DebugRequestType::Attach(task::AttachConfig {
                        process_id: Some(process_id),
                    });
                },
                cx,
            );
        });

        let config = session.read(cx).configuration();
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
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        if !self
            .capabilities_by_id(client_id)
            .supports_modules_request
            .unwrap_or_default()
        {
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
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Client was not found")));
        };

        if !self
            .capabilities_by_id(client_id)
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
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
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

    pub fn restart_stack_frame(
        &mut self,
        client_id: &DebugAdapterClientId,
        stack_frame_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self
            .capabilities_by_id(client_id)
            .supports_restart_frame
            .unwrap_or_default()
        {
            return Task::ready(Ok(()));
        }

        self.request_dap(client_id, RestartStackFrameCommand { stack_frame_id }, cx)
    }

    pub fn scopes(
        &mut self,
        client_id: &DebugAdapterClientId,
        stack_frame_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Scope>>> {
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
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
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", client_id)));
        };

        if self
            .capabilities_by_id(client_id)
            .supports_configuration_done_request
            .unwrap_or_default()
        {
            cx.background_executor().spawn(async move {
                client
                    .request::<ConfigurationDone>(ConfigurationDoneArguments)
                    .await
            })
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn respond_to_start_debugging(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        seq: u64,
        args: Option<StartDebuggingRequestArguments>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some((session, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!(
                "Could not find debug client: {:?} for session {:?}",
                client_id,
                session_id
            )));
        };

        let session_id = *session_id;
        let config = session.read(cx).configuration().clone();

        let request_args = args.unwrap_or_else(|| StartDebuggingRequestArguments {
            configuration: config.initialize_args.clone().unwrap_or_default(),
            request: match config.request {
                DebugRequestType::Launch => StartDebuggingRequestArgumentsRequest::Launch,
                DebugRequestType::Attach(_) => StartDebuggingRequestArgumentsRequest::Attach,
            },
        });

        // Merge the new configuration over the existing configuration
        let mut initialize_args = config.initialize_args.unwrap_or_default();
        merge_json_value_into(request_args.configuration, &mut initialize_args);

        let new_config = DebugAdapterConfig {
            label: config.label.clone(),
            kind: config.kind.clone(),
            request: match request_args.request {
                StartDebuggingRequestArgumentsRequest::Launch => DebugRequestType::Launch,
                StartDebuggingRequestArgumentsRequest::Attach => DebugRequestType::Attach(
                    if let DebugRequestType::Attach(attach_config) = config.request {
                        attach_config
                    } else {
                        AttachConfig::default()
                    },
                ),
            },
            program: config.program.clone(),
            cwd: config.cwd.clone(),
            initialize_args: Some(initialize_args),
        };

        cx.spawn(|this, mut cx| async move {
            let (success, body) = {
                let reconnect_task = this.update(&mut cx, |store, cx| {
                    if !client.adapter().supports_attach()
                        && matches!(new_config.request, DebugRequestType::Attach(_))
                    {
                        Task::ready(Err(anyhow!(
                            "Debug adapter does not support `attach` request"
                        )))
                    } else {
                        store.reconnect_client(
                            &session_id,
                            client.adapter().clone(),
                            client.binary().clone(),
                            new_config,
                            cx,
                        )
                    }
                });

                match reconnect_task {
                    Ok(task) => match task.await {
                        Ok(_) => (true, None),
                        Err(error) => {
                            this.update(&mut cx, |_, cx| {
                                cx.emit(DapStoreEvent::Notification(error.to_string()));
                            })
                            .log_err();

                            (
                                false,
                                Some(serde_json::to_value(ErrorResponse {
                                    error: Some(dap::Message {
                                        id: seq,
                                        format: error.to_string(),
                                        variables: None,
                                        send_telemetry: None,
                                        show_user: None,
                                        url: None,
                                        url_label: None,
                                    }),
                                })?),
                            )
                        }
                    },
                    Err(error) => (
                        false,
                        Some(serde_json::to_value(ErrorResponse {
                            error: Some(dap::Message {
                                id: seq,
                                format: error.to_string(),
                                variables: None,
                                send_telemetry: None,
                                show_user: None,
                                url: None,
                                url_label: None,
                            }),
                        })?),
                    ),
                }
            };

            client
                .send_message(Message::Response(Response {
                    seq,
                    body,
                    success,
                    request_seq: seq,
                    command: StartDebugging::COMMAND.to_string(),
                }))
                .await
        })
    }

    pub fn respond_to_run_in_terminal(
        &self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        success: bool,
        seq: u64,
        body: Option<Value>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!(
                "Could not find debug client: {:?} for session {:?}",
                client_id,
                session_id
            )));
        };

        cx.background_executor().spawn(async move {
            client
                .send_message(Message::Response(Response {
                    seq,
                    body,
                    success,
                    request_seq: seq,
                    command: RunInTerminal::COMMAND.to_string(),
                }))
                .await
        })
    }

    pub fn continue_thread(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ContinueResponse>> {
        let command = ContinueCommand {
            args: ContinueArguments {
                thread_id,
                single_thread: Some(true),
            },
        };

        self.request_dap(client_id, command, cx)
    }

    fn request_dap<R: DapCommand>(
        &self,
        client_id: &DebugAdapterClientId,
        request: R,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<R::Response>>
    where
        <R::DapRequest as dap::requests::Request>::Response: 'static,
        <R::DapRequest as dap::requests::Request>::Arguments: 'static,
    {
        if let Some((upstream_client, upstream_project_id)) = self.upstream_client() {
            return self.send_proto_client_request::<R>(
                upstream_client,
                upstream_project_id,
                client_id,
                request,
                cx,
            );
        }

        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", client_id)));
        };

        let client_id = *client_id;
        let request_clone = request.clone();

        let request_task = cx.background_executor().spawn(async move {
            let args = request_clone.to_dap();
            client.request::<R::DapRequest>(args).await
        });

        let request_clone = request.clone();
        cx.spawn(|this, mut cx| async move {
            let response = request_clone.response_from_dap(request_task.await?);
            request_clone.handle_response(this, &client_id, response, &mut cx)
        })
    }

    fn send_proto_client_request<R: DapCommand>(
        &self,
        upstream_client: AnyProtoClient,
        upstream_project_id: u64,
        client_id: &DebugAdapterClientId,
        request: R,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<R::Response>> {
        let message = request.to_proto(&client_id, upstream_project_id);
        cx.background_executor().spawn(async move {
            let response = upstream_client.request(message).await?;
            request.response_from_proto(response)
        })
    }

    pub fn step_over(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        granularity: SteppingGranularity,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let capabilities = self.capabilities_by_id(client_id);
        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = NextCommand {
            inner: StepCommand {
                thread_id,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests.then(|| true),
            },
        };

        self.request_dap(client_id, command, cx)
    }

    pub fn step_in(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        granularity: SteppingGranularity,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let capabilities = self.capabilities_by_id(client_id);
        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = StepInCommand {
            inner: StepCommand {
                thread_id,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests.then(|| true),
            },
        };

        self.request_dap(client_id, command, cx)
    }

    pub fn step_out(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        granularity: SteppingGranularity,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let capabilities = self.capabilities_by_id(client_id);
        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = StepOutCommand {
            inner: StepCommand {
                thread_id,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests.then(|| true),
            },
        };

        self.request_dap(client_id, command, cx)
    }

    pub fn step_back(
        &self,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
        granularity: SteppingGranularity,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let capabilities = self.capabilities_by_id(client_id);
        if !capabilities.supports_step_back.unwrap_or_default() {
            return Task::ready(Ok(()));
        }

        let supports_single_thread_execution_requests = capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default();
        let supports_stepping_granularity = capabilities
            .supports_stepping_granularity
            .unwrap_or_default();

        let command = StepBackCommand {
            inner: StepCommand {
                thread_id,
                granularity: supports_stepping_granularity.then(|| granularity),
                single_thread: supports_single_thread_execution_requests.then(|| true),
            },
        };

        self.request_dap(client_id, command, cx)
    }

    pub fn variables(
        &self,
        client_id: &DebugAdapterClientId,
        variables_reference: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Variable>>> {
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", client_id)));
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
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", client_id)));
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
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", client_id)));
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
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", client_id)));
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
        self.request_dap(client_id, PauseCommand { thread_id }, cx)
    }

    pub fn terminate_threads(
        &mut self,
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        thread_ids: Option<Vec<u64>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if self
            .capabilities_by_id(client_id)
            .supports_terminate_threads_request
            .unwrap_or_default()
        {
            self.request_dap(client_id, TerminateThreadsCommand { thread_ids }, cx)
        } else {
            self.shutdown_session(session_id, cx)
        }
    }

    pub fn disconnect_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let command = DisconnectCommand {
            restart: Some(false),
            terminate_debuggee: Some(true),
            suspend_debuggee: Some(false),
        };

        self.request_dap(client_id, command, cx)
    }

    pub fn restart(
        &mut self,
        client_id: &DebugAdapterClientId,
        args: Option<Value>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let supports_restart = self
            .capabilities_by_id(client_id)
            .supports_restart_request
            .unwrap_or_default();

        if supports_restart {
            let command = RestartCommand {
                raw: args.unwrap_or(Value::Null),
            };

            self.request_dap(client_id, command, cx)
        } else {
            let command = DisconnectCommand {
                restart: Some(false),
                terminate_debuggee: Some(true),
                suspend_debuggee: Some(false),
            };

            self.request_dap(client_id, command, cx)
        }
    }

    pub fn shutdown_sessions(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        let Some(local_store) = self.as_local() else {
            if let Some((upstream_client, project_id)) = self.upstream_client() {
                return cx.background_executor().spawn(async move {
                    upstream_client
                        .request(proto::DapShutdownSession {
                            project_id,
                            session_id: None,
                        })
                        .await
                        .log_err();

                    ()
                });
            }
            return Task::ready(());
        };

        let mut tasks = Vec::new();

        for session_id in local_store.sessions.keys().cloned().collect::<Vec<_>>() {
            tasks.push(self.shutdown_session(&session_id, cx));
        }

        cx.background_executor().spawn(async move {
            futures::future::join_all(tasks).await;
        })
    }

    pub fn shutdown_session(
        &mut self,
        session_id: &DebugSessionId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(local_store) = self.as_local_mut() else {
            if let Some((upstream_client, project_id)) = self.upstream_client() {
                let future = upstream_client.request(proto::DapShutdownSession {
                    project_id,
                    session_id: Some(session_id.to_proto()),
                });

                return cx
                    .background_executor()
                    .spawn(async move { future.await.map(|_| ()) });
            }

            return Task::ready(Err(anyhow!("Cannot shutdown session on remote side")));
        };

        let Some(session) = local_store.sessions.remove(session_id) else {
            return Task::ready(Err(anyhow!("Could not find session: {:?}", session_id)));
        };

        let mut tasks = Vec::new();
        for client in session.read(cx).clients().collect::<Vec<_>>() {
            tasks.push(self.shutdown_client(&session, client, cx));
        }

        cx.background_executor().spawn(async move {
            futures::future::join_all(tasks).await;
            Ok(())
        })
    }

    fn shutdown_client(
        &mut self,
        session: &Model<DebugSession>,
        client: Arc<DebugAdapterClient>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(local_store) = self.as_local_mut() else {
            return Task::ready(Err(anyhow!("Cannot shutdown client on remote side")));
        };

        let client_id = client.id();

        cx.emit(DapStoreEvent::DebugClientShutdown(client_id));

        local_store.client_by_session.remove(&client_id);
        let capabilities = self.capabilities.remove(&client_id).unwrap_or_default();

        if let Some((downstream_client, project_id)) = self.downstream_client.as_ref() {
            downstream_client
                .send(proto::ShutdownDebugClient {
                    session_id: session.read(cx).id().to_proto(),
                    client_id: client_id.to_proto(),
                    project_id: *project_id,
                })
                .log_err();
        }

        cx.spawn(|_, _| async move {
            if capabilities.supports_terminate_request.unwrap_or_default() {
                let _ = client
                    .request::<Terminate>(TerminateArguments {
                        restart: Some(false),
                    })
                    .await
                    .log_err();
            } else {
                let _ = client
                    .request::<Disconnect>(DisconnectArguments {
                        restart: Some(false),
                        terminate_debuggee: Some(true),
                        suspend_debuggee: Some(false),
                    })
                    .await
                    .log_err();
            }

            client.shutdown().await
        })
    }

    pub fn set_debug_sessions_from_proto(
        &mut self,
        debug_sessions: Vec<proto::DebuggerSession>,
        cx: &mut ModelContext<Self>,
    ) {
        for (session_id, debug_clients) in debug_sessions
            .into_iter()
            .map(|session| (session.session_id, session.clients))
        {
            for debug_client in debug_clients {
                if let DapStoreMode::Remote(remote) = &mut self.mode {
                    if let Some(queue) = &mut remote.event_queue {
                        debug_client.debug_panel_items.into_iter().for_each(|item| {
                            queue.push_back(DapStoreEvent::SetDebugPanelItem(item));
                        });
                    }
                }

                self.update_capabilities_for_client(
                    &DebugSessionId::from_proto(session_id),
                    &DebugAdapterClientId::from_proto(debug_client.client_id),
                    &dap::proto_conversions::capabilities_from_proto(
                        &debug_client.capabilities.unwrap_or_default(),
                    ),
                    cx,
                );
            }
        }

        cx.notify();
    }

    pub fn set_breakpoints_from_proto(
        &mut self,
        breakpoints: Vec<proto::SynchronizeBreakpoints>,
        cx: &mut ModelContext<Self>,
    ) {
        let mut new_breakpoints = BTreeMap::new();
        for project_breakpoints in breakpoints {
            let Some(project_path) = project_breakpoints.project_path else {
                continue;
            };

            new_breakpoints.insert(
                ProjectPath::from_proto(project_path),
                project_breakpoints
                    .breakpoints
                    .into_iter()
                    .filter_map(Breakpoint::from_proto)
                    .collect::<HashSet<_>>(),
            );
        }

        std::mem::swap(&mut self.breakpoints, &mut new_breakpoints);
        cx.notify();
    }

    async fn handle_shutdown_session(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DapShutdownSession>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        if let Some(session_id) = envelope.payload.session_id {
            this.update(&mut cx, |dap_store, cx| {
                dap_store.shutdown_session(&DebugSessionId::from_proto(session_id), cx)
            })?
            .await?;
        } else {
            this.update(&mut cx, |dap_store, cx| dap_store.shutdown_sessions(cx))?
                .await;
        }

        Ok(proto::Ack {})
    }

    async fn handle_dap_command<T: DapCommand>(
        this: Model<Self>,
        envelope: TypedEnvelope<T::ProtoRequest>,
        mut cx: AsyncAppContext,
    ) -> Result<<T::ProtoRequest as proto::RequestMessage>::Response>
    where
        <T::DapRequest as dap::requests::Request>::Arguments: Send,
        <T::DapRequest as dap::requests::Request>::Response: Send,
    {
        let _sender_id = envelope.original_sender_id().unwrap_or_default();
        let client_id = T::client_id_from_proto(&envelope.payload);

        let request = T::from_proto(&envelope.payload);
        let response = this
            .update(&mut cx, |this, cx| {
                this.request_dap::<T>(&client_id, request, cx)
            })?
            .await?;

        Ok(T::response_to_proto(&client_id, response))
    }

    async fn handle_synchronize_breakpoints(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SynchronizeBreakpoints>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let project_path = ProjectPath::from_proto(
            envelope
                .payload
                .project_path
                .context("Invalid Breakpoint call")?,
        );

        this.update(&mut cx, |store, cx| {
            let breakpoints = envelope
                .payload
                .breakpoints
                .into_iter()
                .filter_map(Breakpoint::from_proto)
                .collect::<HashSet<_>>();

            if breakpoints.is_empty() {
                store.breakpoints.remove(&project_path);
            } else {
                store.breakpoints.insert(project_path.clone(), breakpoints);
            }

            cx.emit(DapStoreEvent::BreakpointsChanged(project_path));

            cx.notify();
        })
    }

    async fn handle_set_debug_panel_item(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SetDebuggerPanelItem>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |_, cx| {
            cx.emit(DapStoreEvent::SetDebugPanelItem(envelope.payload));
        })
    }

    async fn handle_update_debug_adapter(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateDebugAdapter>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |_, cx| {
            cx.emit(DapStoreEvent::UpdateDebugAdapter(envelope.payload));
        })
    }

    async fn handle_update_thread_status(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateThreadStatus>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |_, cx| {
            cx.emit(DapStoreEvent::UpdateThreadStatus(envelope.payload));
        })
    }

    async fn handle_set_debug_client_capabilities(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SetDebugClientCapabilities>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |dap_store, cx| {
            dap_store.update_capabilities_for_client(
                &DebugSessionId::from_proto(envelope.payload.session_id),
                &DebugAdapterClientId::from_proto(envelope.payload.client_id),
                &dap::proto_conversions::capabilities_from_proto(&envelope.payload),
                cx,
            );
        })
    }

    async fn handle_shutdown_debug_client(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ShutdownDebugClient>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |dap_store, cx| {
            let client_id = DebugAdapterClientId::from_proto(envelope.payload.client_id);

            dap_store.capabilities.remove(&client_id);

            cx.emit(DapStoreEvent::DebugClientShutdown(client_id));
            cx.notify();
        })
    }

    async fn handle_set_active_debug_line(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SetActiveDebugLine>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let project_path = ProjectPath::from_proto(
            envelope
                .payload
                .project_path
                .context("Invalid Breakpoint call")?,
        );

        this.update(&mut cx, |store, cx| {
            store.active_debug_line = Some((
                DebugAdapterClientId::from_proto(envelope.payload.client_id),
                project_path,
                envelope.payload.row,
            ));

            cx.emit(DapStoreEvent::ActiveDebugLineChanged);
            cx.notify();
        })
    }

    async fn handle_remove_active_debug_line(
        this: Model<Self>,
        _: TypedEnvelope<proto::RemoveActiveDebugLine>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |store, cx| {
            store.active_debug_line.take();

            cx.emit(DapStoreEvent::ActiveDebugLineChanged);
            cx.notify();
        })
    }

    pub fn toggle_breakpoint_for_buffer(
        &mut self,
        project_path: &ProjectPath,
        mut breakpoint: Breakpoint,
        edit_action: BreakpointEditAction,
        cx: &mut ModelContext<Self>,
    ) {
        let upstream_client = self.upstream_client();

        let breakpoint_set = self.breakpoints.entry(project_path.clone()).or_default();

        match edit_action {
            BreakpointEditAction::Toggle => {
                if !breakpoint_set.remove(&breakpoint) {
                    breakpoint_set.insert(breakpoint);
                }
            }
            BreakpointEditAction::EditLogMessage(log_message) => {
                if !log_message.is_empty() {
                    breakpoint.kind = BreakpointKind::Log(log_message.clone());
                    breakpoint_set.remove(&breakpoint);
                    breakpoint_set.insert(breakpoint);
                } else if matches!(&breakpoint.kind, BreakpointKind::Log(_)) {
                    breakpoint_set.remove(&breakpoint);
                }
            }
        }

        if let Some((client, project_id)) = upstream_client.or(self.downstream_client.clone()) {
            client
                .send(client::proto::SynchronizeBreakpoints {
                    project_id,
                    project_path: Some(project_path.to_proto()),
                    breakpoints: breakpoint_set
                        .iter()
                        .filter_map(|breakpoint| breakpoint.to_proto())
                        .collect(),
                })
                .log_err();
        }

        if breakpoint_set.is_empty() {
            self.breakpoints.remove(project_path);
        }

        cx.emit(DapStoreEvent::BreakpointsChanged(project_path.clone()));
        cx.notify();
    }

    pub fn send_breakpoints(
        &self,
        client_id: &DebugAdapterClientId,
        absolute_file_path: Arc<Path>,
        mut breakpoints: Vec<SourceBreakpoint>,
        ignore: bool,
        cx: &ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some((_, client)) = self.client_by_id(client_id, cx) else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", client_id)));
        };

        if Self::INDEX_STARTS_AT_ONE {
            breakpoints.iter_mut().for_each(|bp| bp.line += 1u64)
        }

        cx.background_executor().spawn(async move {
            client
                .request::<SetBreakpoints>(SetBreakpointsArguments {
                    source: Source {
                        path: Some(String::from(absolute_file_path.to_string_lossy())),
                        name: absolute_file_path
                            .file_name()
                            .map(|name| name.to_string_lossy().to_string()),
                        source_reference: None,
                        presentation_hint: None,
                        origin: None,
                        sources: None,
                        adapter_data: None,
                        checksums: None,
                    },
                    breakpoints: Some(if ignore { Vec::default() } else { breakpoints }),
                    source_modified: Some(false),
                    lines: None,
                })
                .await?;

            Ok(())
        })
    }

    pub fn send_changed_breakpoints(
        &self,
        project_path: &ProjectPath,
        absolute_path: PathBuf,
        buffer_snapshot: BufferSnapshot,
        cx: &ModelContext<Self>,
    ) -> Task<Result<()>> {
        let Some(local_store) = self.as_local() else {
            return Task::ready(Err(anyhow!("cannot start session on remote side")));
        };

        let source_breakpoints = self
            .breakpoints
            .get(project_path)
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|bp| bp.source_for_snapshot(&buffer_snapshot))
            .collect::<Vec<_>>();

        let mut tasks = Vec::new();
        for session in local_store.sessions.values() {
            let session = session.read(cx);
            let ignore_breakpoints = self.ignore_breakpoints(&session.id(), cx);
            for client in session.clients().collect::<Vec<_>>() {
                tasks.push(self.send_breakpoints(
                    &client.id(),
                    Arc::from(absolute_path.clone()),
                    source_breakpoints.clone(),
                    ignore_breakpoints,
                    cx,
                ));
            }
        }

        if tasks.is_empty() {
            return Task::ready(Ok(()));
        }

        cx.background_executor().spawn(async move {
            futures::future::join_all(tasks).await;
            Ok(())
        })
    }

    pub fn shared(
        &mut self,
        project_id: u64,
        downstream_client: AnyProtoClient,
        _: &mut ModelContext<Self>,
    ) {
        self.downstream_client = Some((downstream_client.clone(), project_id));

        for (project_path, breakpoints) in self.breakpoints.iter() {
            downstream_client
                .send(proto::SynchronizeBreakpoints {
                    project_id,
                    project_path: Some(project_path.to_proto()),
                    breakpoints: breakpoints
                        .iter()
                        .filter_map(|breakpoint| breakpoint.to_proto())
                        .collect(),
                })
                .log_err();
        }
    }

    pub fn unshared(&mut self, cx: &mut ModelContext<Self>) {
        self.downstream_client.take();

        cx.notify();
    }
}

type LogMessage = Arc<str>;

#[derive(Clone, Debug)]
pub enum BreakpointEditAction {
    Toggle,
    EditLogMessage(LogMessage),
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
    pub cached_position: u32,
    pub kind: BreakpointKind,
}

// Custom implementation for PartialEq, Eq, and Hash is done
// to get toggle breakpoint to solely be based on a breakpoint's
// location. Otherwise, a user can get in situation's where there's
// overlapping breakpoint's with them being aware.
impl PartialEq for Breakpoint {
    fn eq(&self, other: &Self) -> bool {
        match (&self.active_position, &other.active_position) {
            (None, None) => self.cached_position == other.cached_position,
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
            self.cached_position.hash(state);
        }
    }
}

impl Breakpoint {
    pub fn to_source_breakpoint(&self, buffer: &Buffer) -> SourceBreakpoint {
        let line = self
            .active_position
            .map(|position| buffer.summary_for_anchor::<Point>(&position).row)
            .unwrap_or(self.cached_position) as u64;

        let log_message = match &self.kind {
            BreakpointKind::Standard => None,
            BreakpointKind::Log(message) => Some(message.clone().to_string()),
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

    pub fn set_active_position(&mut self, buffer: &Buffer) {
        if self.active_position.is_none() {
            self.active_position =
                Some(buffer.breakpoint_anchor(Point::new(self.cached_position, 0)));
        }
    }

    pub fn point_for_buffer(&self, buffer: &Buffer) -> Point {
        self.active_position
            .map(|position| buffer.summary_for_anchor::<Point>(&position))
            .unwrap_or(Point::new(self.cached_position, 0))
    }

    pub fn point_for_buffer_snapshot(&self, buffer_snapshot: &BufferSnapshot) -> Point {
        self.active_position
            .map(|position| buffer_snapshot.summary_for_anchor::<Point>(&position))
            .unwrap_or(Point::new(self.cached_position, 0))
    }

    pub fn source_for_snapshot(&self, snapshot: &BufferSnapshot) -> SourceBreakpoint {
        let line = self
            .active_position
            .map(|position| snapshot.summary_for_anchor::<Point>(&position).row)
            .unwrap_or(self.cached_position) as u64;

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
                    .map(|position| buffer.summary_for_anchor::<Point>(&position).row)
                    .unwrap_or(self.cached_position),
                path,
                kind: self.kind.clone(),
            },
            None => SerializedBreakpoint {
                position: self.cached_position,
                path,
                kind: self.kind.clone(),
            },
        }
    }

    pub fn to_proto(&self) -> Option<client::proto::Breakpoint> {
        Some(client::proto::Breakpoint {
            position: if let Some(position) = &self.active_position {
                Some(serialize_text_anchor(position))
            } else {
                None
            },
            cached_position: self.cached_position,
            kind: match self.kind {
                BreakpointKind::Standard => proto::BreakpointKind::Standard.into(),
                BreakpointKind::Log(_) => proto::BreakpointKind::Log.into(),
            },
            message: if let BreakpointKind::Log(message) = &self.kind {
                Some(message.to_string())
            } else {
                None
            },
        })
    }

    pub fn from_proto(breakpoint: client::proto::Breakpoint) -> Option<Self> {
        Some(Self {
            active_position: if let Some(position) = breakpoint.position.clone() {
                deserialize_anchor(position)
            } else {
                None
            },
            cached_position: breakpoint.cached_position,
            kind: match proto::BreakpointKind::from_i32(breakpoint.kind) {
                Some(proto::BreakpointKind::Log) => {
                    BreakpointKind::Log(breakpoint.message.clone().unwrap_or_default().into())
                }
                None | Some(proto::BreakpointKind::Standard) => BreakpointKind::Standard,
            },
        })
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
