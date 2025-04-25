use super::{
    breakpoint_store::BreakpointStore,
    locators::DapLocator,
    session::{self, Session, SessionStateEvent},
};
use crate::{
    InlayHint, InlayHintLabel, ProjectEnvironment, ResolveState,
    project_settings::ProjectSettings,
    terminals::{SshCommand, wrap_for_ssh},
    worktree_store::WorktreeStore,
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use dap::{
    Capabilities, CompletionItem, CompletionsArguments, DapRegistry, EvaluateArguments,
    EvaluateArgumentsContext, EvaluateResponse, RunInTerminalRequestArguments, Source,
    StackFrameId, StartDebuggingRequestArguments,
    adapters::{DapStatus, DebugAdapterBinary, DebugAdapterName, TcpArguments},
    client::SessionId,
    messages::Message,
    requests::{Completions, Evaluate, Request as _, RunInTerminal, StartDebugging},
};
use fs::Fs;
use futures::{
    channel::mpsc,
    future::{Shared, join_all},
};
use gpui::{App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task};
use http_client::HttpClient;
use language::{
    BinaryStatus, Buffer, LanguageRegistry, LanguageToolchainStore,
    language_settings::InlayHintKind, range_from_lsp,
};
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;

use remote::SshRemoteClient;
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self},
};
use serde_json::Value;
use settings::{Settings, WorktreeId};
use smol::{lock::Mutex, stream::StreamExt};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    net::Ipv4Addr,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{DebugTaskDefinition, DebugTaskTemplate};
use util::ResultExt as _;
use worktree::Worktree;

pub enum DapStoreEvent {
    DebugClientStarted(SessionId),
    DebugSessionInitialized(SessionId),
    DebugClientShutdown(SessionId),
    DebugClientEvent {
        session_id: SessionId,
        message: Message,
    },
    RunInTerminal {
        session_id: SessionId,
        title: Option<String>,
        cwd: Option<Arc<Path>>,
        command: Option<String>,
        args: Vec<String>,
        envs: HashMap<String, String>,
        sender: mpsc::Sender<Result<u32>>,
    },
    SpawnChildSession {
        request: StartDebuggingRequestArguments,
        parent_session: Entity<Session>,
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
    language_registry: Arc<LanguageRegistry>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    locators: HashMap<String, Arc<dyn DapLocator>>,
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
    start_debugging_tx: futures::channel::mpsc::UnboundedSender<(SessionId, Message)>,
    _start_debugging_task: Task<()>,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_run_debug_locator);
        client.add_entity_request_handler(Self::handle_get_debug_adapter_binary);
    }

    #[expect(clippy::too_many_arguments)]
    pub fn new_local(
        http_client: Arc<dyn HttpClient>,
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        language_registry: Arc<LanguageRegistry>,
        environment: Entity<ProjectEnvironment>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        worktree_store: Entity<WorktreeStore>,
        breakpoint_store: Entity<BreakpointStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.on_app_quit(Self::shutdown_sessions).detach();

        let locators = HashMap::from_iter([(
            "cargo".to_string(),
            Arc::new(super::locators::cargo::CargoLocator {}) as _,
        )]);

        let mode = DapStoreMode::Local(LocalDapStore {
            fs,
            environment,
            http_client,
            node_runtime,
            toolchain_store,
            language_registry,
            locators,
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
        cx: &mut Context<Self>,
    ) -> Self {
        let (start_debugging_tx, mut message_rx) =
            futures::channel::mpsc::unbounded::<(SessionId, Message)>();
        let task = cx.spawn(async move |this, cx| {
            while let Some((session_id, message)) = message_rx.next().await {
                match message {
                    Message::Request(request) => {
                        let _ = this
                            .update(cx, |this, cx| {
                                if request.command == StartDebugging::COMMAND {
                                    this.handle_start_debugging_request(session_id, request, cx)
                                        .detach_and_log_err(cx);
                                } else if request.command == RunInTerminal::COMMAND {
                                    this.handle_run_in_terminal_request(session_id, request, cx)
                                        .detach_and_log_err(cx);
                                }
                            })
                            .log_err();
                    }
                    _ => {}
                }
            }
        });

        Self {
            mode,
            _start_debugging_task: task,
            start_debugging_tx,
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

                let delegate = self.delegate(&worktree, cx);
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
                    project_id: ssh.upstream_project_id,
                    task: Some(definition.to_proto()),
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

    pub fn run_debug_locator(
        &mut self,
        template: DebugTaskTemplate,
        cx: &mut Context<Self>,
    ) -> Task<Result<DebugTaskDefinition>> {
        let Some(locator_name) = template.locator else {
            return Task::ready(Ok(template.definition));
        };

        match &self.mode {
            DapStoreMode::Local(local) => {
                if let Some(locator) = local.locators.get(&locator_name).cloned() {
                    cx.background_spawn(
                        async move { locator.run_locator(template.definition).await },
                    )
                } else {
                    Task::ready(Err(anyhow!("Couldn't find locator {}", locator_name)))
                }
            }
            DapStoreMode::Ssh(ssh) => {
                let request = ssh.upstream_client.request(proto::RunDebugLocator {
                    project_id: ssh.upstream_project_id,
                    locator: locator_name,
                    task: Some(template.definition.to_proto()),
                });
                cx.background_spawn(async move {
                    let response = request.await?;
                    DebugTaskDefinition::from_proto(response)
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
        template: DebugTaskDefinition,
        parent_session: Option<Entity<Session>>,
        cx: &mut Context<Self>,
    ) -> Entity<Session> {
        let session_id = SessionId(util::post_inc(&mut self.next_session_id));

        if let Some(session) = &parent_session {
            session.update(cx, |session, _| {
                session.add_child_session_id(session_id);
            });
        }

        let start_debugging_tx = self.start_debugging_tx.clone();

        let session = Session::new(
            self.breakpoint_store.clone(),
            session_id,
            parent_session,
            template.clone(),
            start_debugging_tx,
            cx,
        );

        self.sessions.insert(session_id, session.clone());
        cx.notify();

        cx.subscribe(&session, {
            move |this: &mut DapStore, _, event: &SessionStateEvent, cx| match event {
                SessionStateEvent::Shutdown => {
                    this.shutdown_session(session_id, cx).detach_and_log_err(cx);
                }
                SessionStateEvent::Restart => {}
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
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(worktree) = self.worktree_store.read(cx).visible_worktrees(cx).next() else {
            return Task::ready(Err(anyhow!("Failed to find a worktree")));
        };

        let dap_store = cx.weak_entity();
        let breakpoint_store = self.breakpoint_store.clone();
        let definition = session.read(cx).definition();

        cx.spawn({
            let session = session.clone();
            async move |this, cx| {
                let binary = this
                    .update(cx, |this, cx| {
                        this.get_debug_adapter_binary(definition.clone(), cx)
                    })?
                    .await?;

                session
                    .update(cx, |session, cx| {
                        session.boot(binary, worktree, breakpoint_store, dap_store, cx)
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

    fn delegate(&self, worktree: &Entity<Worktree>, cx: &mut App) -> DapAdapterDelegate {
        let Some(local_store) = self.as_local() else {
            unimplemented!("Starting session on remote side");
        };

        DapAdapterDelegate::new(
            local_store.fs.clone(),
            worktree.read(cx).id(),
            local_store.node_runtime.clone(),
            local_store.http_client.clone(),
            local_store.language_registry.clone(),
            local_store.toolchain_store.clone(),
            local_store.environment.update(cx, |env, cx| {
                env.get_worktree_environment(worktree.clone(), cx)
            }),
        )
    }

    fn handle_start_debugging_request(
        &mut self,
        session_id: SessionId,
        request: dap::messages::Request,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(parent_session) = self.session_by_id(session_id) else {
            return Task::ready(Err(anyhow!("Session not found")));
        };
        let request_seq = request.seq;

        let launch_request: Option<Result<StartDebuggingRequestArguments, _>> = request
            .arguments
            .as_ref()
            .map(|value| serde_json::from_value(value.clone()));

        let mut success = true;
        if let Some(Ok(request)) = launch_request {
            cx.emit(DapStoreEvent::SpawnChildSession {
                request,
                parent_session: parent_session.clone(),
            });
        } else {
            log::error!(
                "Failed to parse launch request arguments: {:?}",
                request.arguments
            );
            success = false;
        }

        cx.spawn(async move |_, cx| {
            parent_session
                .update(cx, |session, cx| {
                    session.respond_to_client(
                        request_seq,
                        success,
                        StartDebugging::COMMAND.to_string(),
                        None,
                        cx,
                    )
                })?
                .await
        })
    }

    fn handle_run_in_terminal_request(
        &mut self,
        session_id: SessionId,
        request: dap::messages::Request,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(session) = self.session_by_id(session_id) else {
            return Task::ready(Err(anyhow!("Session not found")));
        };

        let request_args = serde_json::from_value::<RunInTerminalRequestArguments>(
            request.arguments.unwrap_or_default(),
        )
        .expect("To parse StartDebuggingRequestArguments");

        let seq = request.seq;

        let cwd = Path::new(&request_args.cwd);

        match cwd.try_exists() {
            Ok(false) | Err(_) if !request_args.cwd.is_empty() => {
                return session.update(cx, |session, cx| {
                    session.respond_to_client(
                        seq,
                        false,
                        RunInTerminal::COMMAND.to_string(),
                        serde_json::to_value(dap::ErrorResponse {
                            error: Some(dap::Message {
                                id: seq,
                                format: format!("Received invalid/unknown cwd: {cwd:?}"),
                                variables: None,
                                send_telemetry: None,
                                show_user: None,
                                url: None,
                                url_label: None,
                            }),
                        })
                        .ok(),
                        cx,
                    )
                });
            }
            _ => (),
        }
        let mut args = request_args.args.clone();

        // Handle special case for NodeJS debug adapter
        // If only the Node binary path is provided, we set the command to None
        // This prevents the NodeJS REPL from appearing, which is not the desired behavior
        // The expected usage is for users to provide their own Node command, e.g., `node test.js`
        // This allows the NodeJS debug client to attach correctly
        let command = if args.len() > 1 {
            Some(args.remove(0))
        } else {
            None
        };

        let mut envs: HashMap<String, String> = Default::default();
        if let Some(Value::Object(env)) = request_args.env {
            for (key, value) in env {
                let value_str = match (key.as_str(), value) {
                    (_, Value::String(value)) => value,
                    _ => continue,
                };

                envs.insert(key, value_str);
            }
        }

        let (tx, mut rx) = mpsc::channel::<Result<u32>>(1);
        let cwd = Some(cwd)
            .filter(|cwd| cwd.as_os_str().len() > 0)
            .map(Arc::from)
            .or_else(|| {
                self.session_by_id(session_id)
                    .and_then(|session| session.read(cx).binary().cwd.as_deref().map(Arc::from))
            });
        cx.emit(DapStoreEvent::RunInTerminal {
            session_id,
            title: request_args.title,
            cwd,
            command,
            args,
            envs,
            sender: tx,
        });
        cx.notify();

        let session = session.downgrade();
        cx.spawn(async move |_, cx| {
            let (success, body) = match rx.next().await {
                Some(Ok(pid)) => (
                    true,
                    serde_json::to_value(dap::RunInTerminalResponse {
                        process_id: None,
                        shell_process_id: Some(pid as u64),
                    })
                    .ok(),
                ),
                Some(Err(error)) => (
                    false,
                    serde_json::to_value(dap::ErrorResponse {
                        error: Some(dap::Message {
                            id: seq,
                            format: error.to_string(),
                            variables: None,
                            send_telemetry: None,
                            show_user: None,
                            url: None,
                            url_label: None,
                        }),
                    })
                    .ok(),
                ),
                None => (
                    false,
                    serde_json::to_value(dap::ErrorResponse {
                        error: Some(dap::Message {
                            id: seq,
                            format: "failed to receive response from spawn terminal".to_string(),
                            variables: None,
                            send_telemetry: None,
                            show_user: None,
                            url: None,
                            url_label: None,
                        }),
                    })
                    .ok(),
                ),
            };

            session
                .update(cx, |session, cx| {
                    session.respond_to_client(
                        seq,
                        success,
                        RunInTerminal::COMMAND.to_string(),
                        body,
                        cx,
                    )
                })?
                .await
        })
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

    pub fn resolve_inline_values(
        &self,
        session: Entity<Session>,
        stack_frame_id: StackFrameId,
        buffer_handle: Entity<Buffer>,
        inline_values: Vec<lsp::InlineValue>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<InlayHint>>> {
        let snapshot = buffer_handle.read(cx).snapshot();
        let all_variables = session.read(cx).variables_by_stack_frame_id(stack_frame_id);

        cx.spawn(async move |_, cx| {
            let mut inlay_hints = Vec::with_capacity(inline_values.len());
            for inline_value in inline_values.iter() {
                match inline_value {
                    lsp::InlineValue::Text(text) => {
                        inlay_hints.push(InlayHint {
                            position: snapshot.anchor_after(range_from_lsp(text.range).end),
                            label: InlayHintLabel::String(format!(": {}", text.text)),
                            kind: Some(InlayHintKind::Type),
                            padding_left: false,
                            padding_right: false,
                            tooltip: None,
                            resolve_state: ResolveState::Resolved,
                        });
                    }
                    lsp::InlineValue::VariableLookup(variable_lookup) => {
                        let range = range_from_lsp(variable_lookup.range);

                        let mut variable_name = variable_lookup
                            .variable_name
                            .clone()
                            .unwrap_or_else(|| snapshot.text_for_range(range.clone()).collect());

                        if !variable_lookup.case_sensitive_lookup {
                            variable_name = variable_name.to_ascii_lowercase();
                        }

                        let Some(variable) = all_variables.iter().find(|variable| {
                            if variable_lookup.case_sensitive_lookup {
                                variable.name == variable_name
                            } else {
                                variable.name.to_ascii_lowercase() == variable_name
                            }
                        }) else {
                            continue;
                        };

                        inlay_hints.push(InlayHint {
                            position: snapshot.anchor_after(range.end),
                            label: InlayHintLabel::String(format!(": {}", variable.value)),
                            kind: Some(InlayHintKind::Type),
                            padding_left: false,
                            padding_right: false,
                            tooltip: None,
                            resolve_state: ResolveState::Resolved,
                        });
                    }
                    lsp::InlineValue::EvaluatableExpression(expression) => {
                        let range = range_from_lsp(expression.range);

                        let expression = expression
                            .expression
                            .clone()
                            .unwrap_or_else(|| snapshot.text_for_range(range.clone()).collect());

                        let Ok(eval_task) = session.update(cx, |session, cx| {
                            session.evaluate(
                                expression,
                                Some(EvaluateArgumentsContext::Variables),
                                Some(stack_frame_id),
                                None,
                                cx,
                            )
                        }) else {
                            continue;
                        };

                        if let Some(response) = eval_task.await {
                            inlay_hints.push(InlayHint {
                                position: snapshot.anchor_after(range.end),
                                label: InlayHintLabel::String(format!(": {}", response.result)),
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
        envelope: TypedEnvelope<proto::RunDebugLocator>,
        mut cx: AsyncApp,
    ) -> Result<proto::DebugTaskDefinition> {
        let template = DebugTaskTemplate {
            locator: Some(envelope.payload.locator),
            definition: DebugTaskDefinition::from_proto(
                envelope
                    .payload
                    .task
                    .ok_or_else(|| anyhow!("missing definition"))?,
            )?,
        };
        let definition = this
            .update(&mut cx, |this, cx| this.run_debug_locator(template, cx))?
            .await?;
        Ok(definition.to_proto())
    }

    async fn handle_get_debug_adapter_binary(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetDebugAdapterBinary>,
        mut cx: AsyncApp,
    ) -> Result<proto::DebugAdapterBinary> {
        let definition = DebugTaskDefinition::from_proto(
            envelope
                .payload
                .task
                .ok_or_else(|| anyhow!("missing definition"))?,
        )?;
        let binary = this
            .update(&mut cx, |this, cx| {
                this.get_debug_adapter_binary(definition, cx)
            })?
            .await?;
        Ok(binary.to_proto())
    }
}

#[derive(Clone)]
pub struct DapAdapterDelegate {
    fs: Arc<dyn Fs>,
    worktree_id: WorktreeId,
    node_runtime: NodeRuntime,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    updated_adapters: Arc<Mutex<HashSet<DebugAdapterName>>>,
    load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
}

impl DapAdapterDelegate {
    pub fn new(
        fs: Arc<dyn Fs>,
        worktree_id: WorktreeId,
        node_runtime: NodeRuntime,
        http_client: Arc<dyn HttpClient>,
        language_registry: Arc<LanguageRegistry>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
    ) -> Self {
        Self {
            fs,
            worktree_id,
            http_client,
            node_runtime,
            toolchain_store,
            language_registry,
            load_shell_env_task,
            updated_adapters: Default::default(),
        }
    }
}

#[async_trait(?Send)]
impl dap::adapters::DapDelegate for DapAdapterDelegate {
    fn worktree_id(&self) -> WorktreeId {
        self.worktree_id
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

    fn updated_adapters(&self) -> Arc<Mutex<HashSet<DebugAdapterName>>> {
        self.updated_adapters.clone()
    }

    fn update_status(&self, dap_name: DebugAdapterName, status: dap::adapters::DapStatus) {
        let name = SharedString::from(dap_name.to_string());
        let status = match status {
            DapStatus::None => BinaryStatus::None,
            DapStatus::Downloading => BinaryStatus::Downloading,
            DapStatus::Failed { error } => BinaryStatus::Failed { error },
            DapStatus::CheckingForUpdate => BinaryStatus::CheckingForUpdate,
        };

        self.language_registry
            .update_dap_status(LanguageServerName(name), status);
    }

    fn which(&self, command: &OsStr) -> Option<PathBuf> {
        which::which(command).ok()
    }

    async fn shell_env(&self) -> HashMap<String, String> {
        let task = self.load_shell_env_task.clone();
        task.await.unwrap_or_default()
    }

    fn toolchain_store(&self) -> Arc<dyn LanguageToolchainStore> {
        self.toolchain_store.clone()
    }
}
