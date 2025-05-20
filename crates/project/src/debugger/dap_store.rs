use super::{
    breakpoint_store::BreakpointStore,
    dap_command::EvaluateCommand,
    locators,
    session::{self, Session, SessionStateEvent},
};
use crate::{
    InlayHint, InlayHintLabel, ProjectEnvironment, ResolveState,
    project_settings::ProjectSettings,
    terminals::{SshCommand, wrap_for_ssh},
    worktree_store::WorktreeStore,
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use dap::{
    Capabilities, CompletionItem, CompletionsArguments, DapRegistry, DebugRequest,
    EvaluateArguments, EvaluateArgumentsContext, EvaluateResponse, Source, StackFrameId,
    adapters::{
        DapDelegate, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition, TcpArguments,
    },
    client::SessionId,
    inline_value::VariableLookupKind,
    messages::Message,
    requests::{Completions, Evaluate},
};
use fs::Fs;
use futures::{
    StreamExt,
    channel::mpsc::{self, UnboundedSender},
    future::{Shared, join_all},
};
use gpui::{App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task};
use http_client::HttpClient;
use language::{Buffer, LanguageToolchainStore, language_settings::InlayHintKind};
use node_runtime::NodeRuntime;

use remote::SshRemoteClient;
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self},
};
use settings::{Settings, WorktreeId};
use std::{
    borrow::Borrow,
    collections::BTreeMap,
    ffi::OsStr,
    net::Ipv4Addr,
    path::{Path, PathBuf},
    sync::{Arc, Once},
};
use task::{DebugScenario, SpawnInTerminal, TaskTemplate};
use util::{ResultExt as _, merge_json_value_into};
use worktree::Worktree;

#[derive(Debug)]
pub enum DapStoreEvent {
    DebugClientStarted(SessionId),
    DebugSessionInitialized(SessionId),
    DebugClientShutdown(SessionId),
    DebugClientEvent {
        session_id: SessionId,
        message: Message,
    },
    Notification(String),
    RemoteHasInitialized,
}

#[allow(clippy::large_enum_variant)]
enum DapStoreMode {
    Local(LocalDapStore),
    Ssh(SshDapStore),
    Collab,
}

pub struct LocalDapStore {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    http_client: Arc<dyn HttpClient>,
    environment: Entity<ProjectEnvironment>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
}

pub struct SshDapStore {
    ssh_client: Entity<SshRemoteClient>,
    upstream_client: AnyProtoClient,
    upstream_project_id: u64,
}

pub struct DapStore {
    mode: DapStoreMode,
    downstream_client: Option<(AnyProtoClient, u64)>,
    breakpoint_store: Entity<BreakpointStore>,
    worktree_store: Entity<WorktreeStore>,
    sessions: BTreeMap<SessionId, Entity<Session>>,
    next_session_id: u32,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    pub fn init(client: &AnyProtoClient, cx: &mut App) {
        static ADD_LOCATORS: Once = Once::new();
        ADD_LOCATORS.call_once(|| {
            DapRegistry::global(cx).add_locator(Arc::new(locators::cargo::CargoLocator {}))
        });
        client.add_entity_request_handler(Self::handle_run_debug_locator);
        client.add_entity_request_handler(Self::handle_get_debug_adapter_binary);
        client.add_entity_message_handler(Self::handle_log_to_debug_console);
    }

    #[expect(clippy::too_many_arguments)]
    pub fn new_local(
        http_client: Arc<dyn HttpClient>,
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        environment: Entity<ProjectEnvironment>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        worktree_store: Entity<WorktreeStore>,
        breakpoint_store: Entity<BreakpointStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mode = DapStoreMode::Local(LocalDapStore {
            fs,
            environment,
            http_client,
            node_runtime,
            toolchain_store,
        });

        Self::new(mode, breakpoint_store, worktree_store, cx)
    }

    pub fn new_ssh(
        project_id: u64,
        ssh_client: Entity<SshRemoteClient>,
        breakpoint_store: Entity<BreakpointStore>,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mode = DapStoreMode::Ssh(SshDapStore {
            upstream_client: ssh_client.read(cx).proto_client(),
            ssh_client,
            upstream_project_id: project_id,
        });

        Self::new(mode, breakpoint_store, worktree_store, cx)
    }

    pub fn new_collab(
        _project_id: u64,
        _upstream_client: AnyProtoClient,
        breakpoint_store: Entity<BreakpointStore>,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(DapStoreMode::Collab, breakpoint_store, worktree_store, cx)
    }

    fn new(
        mode: DapStoreMode,
        breakpoint_store: Entity<BreakpointStore>,
        worktree_store: Entity<WorktreeStore>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            mode,
            next_session_id: 0,
            downstream_client: None,
            breakpoint_store,
            worktree_store,
            sessions: Default::default(),
        }
    }

    pub fn get_debug_adapter_binary(
        &mut self,
        definition: DebugTaskDefinition,
        session_id: SessionId,
        console: UnboundedSender<String>,
        cx: &mut Context<Self>,
    ) -> Task<Result<DebugAdapterBinary>> {
        match &self.mode {
            DapStoreMode::Local(_) => {
                let Some(worktree) = self.worktree_store.read(cx).visible_worktrees(cx).next()
                else {
                    return Task::ready(Err(anyhow!("Failed to find a worktree")));
                };
                let Some(adapter) = DapRegistry::global(cx).adapter(&definition.adapter) else {
                    return Task::ready(Err(anyhow!("Failed to find a debug adapter")));
                };

                let user_installed_path = ProjectSettings::get_global(cx)
                    .dap
                    .get(&adapter.name())
                    .and_then(|s| s.binary.as_ref().map(PathBuf::from));

                let delegate = self.delegate(&worktree, console, cx);
                let cwd: Arc<Path> = definition
                    .cwd()
                    .unwrap_or(worktree.read(cx).abs_path().as_ref())
                    .into();

                cx.spawn(async move |this, cx| {
                    let mut binary = adapter
                        .get_binary(&delegate, &definition, user_installed_path, cx)
                        .await?;

                    let env = this
                        .update(cx, |this, cx| {
                            this.as_local()
                                .unwrap()
                                .environment
                                .update(cx, |environment, cx| {
                                    environment.get_directory_environment(cwd, cx)
                                })
                        })?
                        .await;

                    if let Some(mut env) = env {
                        env.extend(std::mem::take(&mut binary.envs));
                        binary.envs = env;
                    }

                    Ok(binary)
                })
            }
            DapStoreMode::Ssh(ssh) => {
                let request = ssh.upstream_client.request(proto::GetDebugAdapterBinary {
                    session_id: session_id.to_proto(),
                    project_id: ssh.upstream_project_id,
                    definition: Some(definition.to_proto()),
                });
                let ssh_client = ssh.ssh_client.clone();

                cx.spawn(async move |_, cx| {
                    let response = request.await?;
                    let binary = DebugAdapterBinary::from_proto(response)?;
                    let mut ssh_command = ssh_client.update(cx, |ssh, _| {
                        anyhow::Ok(SshCommand {
                            arguments: ssh
                                .ssh_args()
                                .ok_or_else(|| anyhow!("SSH arguments not found"))?,
                        })
                    })??;

                    let mut connection = None;
                    if let Some(c) = binary.connection {
                        let local_bind_addr = Ipv4Addr::new(127, 0, 0, 1);
                        let port =
                            dap::transport::TcpTransport::unused_port(local_bind_addr).await?;

                        ssh_command.add_port_forwarding(port, c.host.to_string(), c.port);
                        connection = Some(TcpArguments {
                            port: c.port,
                            host: local_bind_addr,
                            timeout: c.timeout,
                        })
                    }

                    let (program, args) = wrap_for_ssh(
                        &ssh_command,
                        Some((&binary.command, &binary.arguments)),
                        binary.cwd.as_deref(),
                        binary.envs,
                        None,
                    );

                    Ok(DebugAdapterBinary {
                        command: program,
                        arguments: args,
                        envs: HashMap::default(),
                        cwd: None,
                        connection,
                        request_args: binary.request_args,
                    })
                })
            }
            DapStoreMode::Collab => {
                Task::ready(Err(anyhow!("Debugging is not yet supported via collab")))
            }
        }
    }

    pub fn debug_scenario_for_build_task(
        &self,
        build: TaskTemplate,
        adapter: DebugAdapterName,
        label: SharedString,
        cx: &mut App,
    ) -> Option<DebugScenario> {
        DapRegistry::global(cx)
            .locators()
            .values()
            .find_map(|locator| locator.create_scenario(&build, &label, adapter.clone()))
    }

    pub fn run_debug_locator(
        &mut self,
        locator_name: &str,
        build_command: SpawnInTerminal,
        cx: &mut Context<Self>,
    ) -> Task<Result<DebugRequest>> {
        match &self.mode {
            DapStoreMode::Local(_) => {
                // Pre-resolve args with existing environment.
                let locators = DapRegistry::global(cx).locators();
                let locator = locators.get(locator_name);

                if let Some(locator) = locator.cloned() {
                    cx.background_spawn(async move {
                        let result = locator
                            .run(build_command.clone())
                            .await
                            .log_with_level(log::Level::Error);
                        if let Some(result) = result {
                            return Ok(result);
                        }

                        Err(anyhow!(
                            "None of the locators for task `{}` completed successfully",
                            build_command.label
                        ))
                    })
                } else {
                    Task::ready(Err(anyhow!(
                        "Couldn't find any locator for task `{}`. Specify the `attach` or `launch` arguments in your debug scenario definition",
                        build_command.label
                    )))
                }
            }
            DapStoreMode::Ssh(ssh) => {
                let request = ssh.upstream_client.request(proto::RunDebugLocators {
                    project_id: ssh.upstream_project_id,
                    build_command: Some(build_command.to_proto()),
                    locator: locator_name.to_owned(),
                });
                cx.background_spawn(async move {
                    let response = request.await?;
                    DebugRequest::from_proto(response)
                })
            }
            DapStoreMode::Collab => {
                Task::ready(Err(anyhow!("Debugging is not yet supported via collab")))
            }
        }
    }

    fn as_local(&self) -> Option<&LocalDapStore> {
        match &self.mode {
            DapStoreMode::Local(local_dap_store) => Some(local_dap_store),
            _ => None,
        }
    }

    pub fn new_session(
        &mut self,
        label: SharedString,
        adapter: DebugAdapterName,
        parent_session: Option<Entity<Session>>,
        cx: &mut Context<Self>,
    ) -> Entity<Session> {
        let session_id = SessionId(util::post_inc(&mut self.next_session_id));

        if let Some(session) = &parent_session {
            session.update(cx, |session, _| {
                session.add_child_session_id(session_id);
            });
        }

        let session = Session::new(
            self.breakpoint_store.clone(),
            session_id,
            parent_session,
            label,
            adapter,
            cx,
        );

        self.sessions.insert(session_id, session.clone());
        cx.notify();

        cx.subscribe(&session, {
            move |this: &mut DapStore, _, event: &SessionStateEvent, cx| match event {
                SessionStateEvent::Shutdown => {
                    this.shutdown_session(session_id, cx).detach_and_log_err(cx);
                }
                SessionStateEvent::Restart | SessionStateEvent::SpawnChildSession { .. } => {}
                SessionStateEvent::Running => {
                    cx.emit(DapStoreEvent::DebugClientStarted(session_id));
                }
            }
        })
        .detach();

        session
    }

    pub fn boot_session(
        &self,
        session: Entity<Session>,
        definition: DebugTaskDefinition,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(worktree) = self.worktree_store.read(cx).visible_worktrees(cx).next() else {
            return Task::ready(Err(anyhow!("Failed to find a worktree")));
        };

        let dap_store = cx.weak_entity();
        let console = session.update(cx, |session, cx| session.console_output(cx));
        let session_id = session.read(cx).session_id();

        cx.spawn({
            let session = session.clone();
            async move |this, cx| {
                let mut binary = this
                    .update(cx, |this, cx| {
                        this.get_debug_adapter_binary(definition.clone(), session_id, console, cx)
                    })?
                    .await?;

                if let Some(args) = definition.initialize_args {
                    merge_json_value_into(args, &mut binary.request_args.configuration);
                }

                session
                    .update(cx, |session, cx| {
                        session.boot(binary, worktree, dap_store, cx)
                    })?
                    .await
            }
        })
    }

    pub fn session_by_id(
        &self,
        session_id: impl Borrow<SessionId>,
    ) -> Option<Entity<session::Session>> {
        let session_id = session_id.borrow();
        let client = self.sessions.get(session_id).cloned();

        client
    }
    pub fn sessions(&self) -> impl Iterator<Item = &Entity<Session>> {
        self.sessions.values()
    }

    pub fn capabilities_by_id(
        &self,
        session_id: impl Borrow<SessionId>,
        cx: &App,
    ) -> Option<Capabilities> {
        let session_id = session_id.borrow();
        self.sessions
            .get(session_id)
            .map(|client| client.read(cx).capabilities.clone())
    }

    pub fn breakpoint_store(&self) -> &Entity<BreakpointStore> {
        &self.breakpoint_store
    }

    pub fn worktree_store(&self) -> &Entity<WorktreeStore> {
        &self.worktree_store
    }

    #[allow(dead_code)]
    async fn handle_ignore_breakpoint_state(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::IgnoreBreakpointState>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let session_id = SessionId::from_proto(envelope.payload.session_id);

        this.update(&mut cx, |this, cx| {
            if let Some(session) = this.session_by_id(&session_id) {
                session.update(cx, |session, cx| {
                    session.set_ignore_breakpoints(envelope.payload.ignore, cx)
                })
            } else {
                Task::ready(HashMap::default())
            }
        })?
        .await;

        Ok(())
    }

    fn delegate(
        &self,
        worktree: &Entity<Worktree>,
        console: UnboundedSender<String>,
        cx: &mut App,
    ) -> Arc<dyn DapDelegate> {
        let Some(local_store) = self.as_local() else {
            unimplemented!("Starting session on remote side");
        };

        Arc::new(DapAdapterDelegate::new(
            local_store.fs.clone(),
            worktree.read(cx).snapshot(),
            console,
            local_store.node_runtime.clone(),
            local_store.http_client.clone(),
            local_store.toolchain_store.clone(),
            local_store.environment.update(cx, |env, cx| {
                env.get_worktree_environment(worktree.clone(), cx)
            }),
        ))
    }

    pub fn evaluate(
        &self,
        session_id: &SessionId,
        stack_frame_id: u64,
        expression: String,
        context: EvaluateArgumentsContext,
        source: Option<Source>,
        cx: &mut Context<Self>,
    ) -> Task<Result<EvaluateResponse>> {
        let Some(client) = self
            .session_by_id(session_id)
            .and_then(|client| client.read(cx).adapter_client())
        else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", session_id)));
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
                    source,
                })
                .await
        })
    }

    pub fn completions(
        &self,
        session_id: &SessionId,
        stack_frame_id: u64,
        text: String,
        completion_column: u64,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<CompletionItem>>> {
        let Some(client) = self
            .session_by_id(session_id)
            .and_then(|client| client.read(cx).adapter_client())
        else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", session_id)));
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

    pub fn resolve_inline_value_locations(
        &self,
        session: Entity<Session>,
        stack_frame_id: StackFrameId,
        buffer_handle: Entity<Buffer>,
        inline_value_locations: Vec<dap::inline_value::InlineValueLocation>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<InlayHint>>> {
        let snapshot = buffer_handle.read(cx).snapshot();
        let all_variables = session.read(cx).variables_by_stack_frame_id(stack_frame_id);

        fn format_value(mut value: String) -> String {
            const LIMIT: usize = 100;

            if value.len() > LIMIT {
                value.truncate(LIMIT);
                value.push_str("...");
            }

            format!(": {}", value)
        }

        cx.spawn(async move |_, cx| {
            let mut inlay_hints = Vec::with_capacity(inline_value_locations.len());
            for inline_value_location in inline_value_locations.iter() {
                let point = snapshot.point_to_point_utf16(language::Point::new(
                    inline_value_location.row as u32,
                    inline_value_location.column as u32,
                ));
                let position = snapshot.anchor_after(point);

                match inline_value_location.lookup {
                    VariableLookupKind::Variable => {
                        let Some(variable) = all_variables
                            .iter()
                            .find(|variable| variable.name == inline_value_location.variable_name)
                        else {
                            continue;
                        };

                        inlay_hints.push(InlayHint {
                            position,
                            label: InlayHintLabel::String(format_value(variable.value.clone())),
                            kind: Some(InlayHintKind::Type),
                            padding_left: false,
                            padding_right: false,
                            tooltip: None,
                            resolve_state: ResolveState::Resolved,
                        });
                    }
                    VariableLookupKind::Expression => {
                        let Ok(eval_task) = session.update(cx, |session, _| {
                            session.mode.request_dap(EvaluateCommand {
                                expression: inline_value_location.variable_name.clone(),
                                frame_id: Some(stack_frame_id),
                                source: None,
                                context: Some(EvaluateArgumentsContext::Variables),
                            })
                        }) else {
                            continue;
                        };

                        if let Some(response) = eval_task.await.log_err() {
                            inlay_hints.push(InlayHint {
                                position,
                                label: InlayHintLabel::String(format_value(response.result)),
                                kind: Some(InlayHintKind::Type),
                                padding_left: false,
                                padding_right: false,
                                tooltip: None,
                                resolve_state: ResolveState::Resolved,
                            });
                        };
                    }
                };
            }

            Ok(inlay_hints)
        })
    }

    pub fn shutdown_sessions(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let mut tasks = vec![];
        for session_id in self.sessions.keys().cloned().collect::<Vec<_>>() {
            tasks.push(self.shutdown_session(session_id, cx));
        }

        cx.background_executor().spawn(async move {
            futures::future::join_all(tasks).await;
        })
    }

    pub fn shutdown_session(
        &mut self,
        session_id: SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(session) = self.sessions.remove(&session_id) else {
            return Task::ready(Err(anyhow!("Could not find session: {:?}", session_id)));
        };

        let shutdown_children = session
            .read(cx)
            .child_session_ids()
            .iter()
            .map(|session_id| self.shutdown_session(*session_id, cx))
            .collect::<Vec<_>>();

        let shutdown_parent_task = if let Some(parent_session) = session
            .read(cx)
            .parent_id(cx)
            .and_then(|session_id| self.session_by_id(session_id))
        {
            let shutdown_id = parent_session.update(cx, |parent_session, _| {
                parent_session.remove_child_session_id(session_id);

                if parent_session.child_session_ids().len() == 0 {
                    Some(parent_session.session_id())
                } else {
                    None
                }
            });

            shutdown_id.map(|session_id| self.shutdown_session(session_id, cx))
        } else {
            None
        };

        let shutdown_task = session.update(cx, |this, cx| this.shutdown(cx));

        cx.background_spawn(async move {
            if shutdown_children.len() > 0 {
                let _ = join_all(shutdown_children).await;
            }

            shutdown_task.await;

            if let Some(parent_task) = shutdown_parent_task {
                parent_task.await?;
            }

            Ok(())
        })
    }

    pub fn shared(
        &mut self,
        project_id: u64,
        downstream_client: AnyProtoClient,
        _: &mut Context<Self>,
    ) {
        self.downstream_client = Some((downstream_client.clone(), project_id));
    }

    pub fn unshared(&mut self, cx: &mut Context<Self>) {
        self.downstream_client.take();

        cx.notify();
    }

    async fn handle_run_debug_locator(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RunDebugLocators>,
        mut cx: AsyncApp,
    ) -> Result<proto::DebugRequest> {
        let task = envelope
            .payload
            .build_command
            .ok_or_else(|| anyhow!("missing definition"))?;
        let build_task = SpawnInTerminal::from_proto(task);
        let locator = envelope.payload.locator;
        let request = this
            .update(&mut cx, |this, cx| {
                this.run_debug_locator(&locator, build_task, cx)
            })?
            .await?;

        Ok(request.to_proto())
    }

    async fn handle_get_debug_adapter_binary(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetDebugAdapterBinary>,
        mut cx: AsyncApp,
    ) -> Result<proto::DebugAdapterBinary> {
        let definition = DebugTaskDefinition::from_proto(
            envelope
                .payload
                .definition
                .ok_or_else(|| anyhow!("missing definition"))?,
        )?;
        let (tx, mut rx) = mpsc::unbounded();
        let session_id = envelope.payload.session_id;
        cx.spawn({
            let this = this.clone();
            async move |cx| {
                while let Some(message) = rx.next().await {
                    this.update(cx, |this, _| {
                        if let Some((downstream, project_id)) = this.downstream_client.clone() {
                            downstream
                                .send(proto::LogToDebugConsole {
                                    project_id,
                                    session_id,
                                    message,
                                })
                                .ok();
                        }
                    })
                    .ok();
                }
            }
        })
        .detach();

        let binary = this
            .update(&mut cx, |this, cx| {
                this.get_debug_adapter_binary(definition, SessionId::from_proto(session_id), tx, cx)
            })?
            .await?;
        Ok(binary.to_proto())
    }

    async fn handle_log_to_debug_console(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::LogToDebugConsole>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let session_id = SessionId::from_proto(envelope.payload.session_id);
        this.update(&mut cx, |this, cx| {
            let Some(session) = this.sessions.get(&session_id) else {
                return;
            };
            session.update(cx, |session, cx| {
                session
                    .console_output(cx)
                    .unbounded_send(envelope.payload.message)
                    .ok();
            })
        })
    }
}

#[derive(Clone)]
pub struct DapAdapterDelegate {
    fs: Arc<dyn Fs>,
    console: mpsc::UnboundedSender<String>,
    worktree: worktree::Snapshot,
    node_runtime: NodeRuntime,
    http_client: Arc<dyn HttpClient>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
}

impl DapAdapterDelegate {
    pub fn new(
        fs: Arc<dyn Fs>,
        worktree: worktree::Snapshot,
        status: mpsc::UnboundedSender<String>,
        node_runtime: NodeRuntime,
        http_client: Arc<dyn HttpClient>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
    ) -> Self {
        Self {
            fs,
            console: status,
            worktree,
            http_client,
            node_runtime,
            toolchain_store,
            load_shell_env_task,
        }
    }
}

#[async_trait]
impl dap::adapters::DapDelegate for DapAdapterDelegate {
    fn worktree_id(&self) -> WorktreeId {
        self.worktree.id()
    }

    fn worktree_root_path(&self) -> &Path {
        &self.worktree.abs_path()
    }
    fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    fn node_runtime(&self) -> NodeRuntime {
        self.node_runtime.clone()
    }

    fn fs(&self) -> Arc<dyn Fs> {
        self.fs.clone()
    }

    fn output_to_console(&self, msg: String) {
        self.console.unbounded_send(msg).ok();
    }

    async fn which(&self, command: &OsStr) -> Option<PathBuf> {
        which::which(command).ok()
    }

    async fn shell_env(&self) -> HashMap<String, String> {
        let task = self.load_shell_env_task.clone();
        task.await.unwrap_or_default()
    }

    fn toolchain_store(&self) -> Arc<dyn LanguageToolchainStore> {
        self.toolchain_store.clone()
    }
    async fn read_text_file(&self, path: PathBuf) -> Result<String> {
        let entry = self
            .worktree
            .entry_for_path(&path)
            .with_context(|| format!("no worktree entry for path {path:?}"))?;
        let abs_path = self
            .worktree
            .absolutize(&entry.path)
            .with_context(|| format!("cannot absolutize path {path:?}"))?;

        self.fs.load(&abs_path).await
    }
}
