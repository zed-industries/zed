use super::{
    breakpoint_store::BreakpointStore,
    // Will need to uncomment this once we implement rpc message handler again
    // dap_command::{
    //     ContinueCommand, DapCommand, DisconnectCommand, NextCommand, PauseCommand, RestartCommand,
    //     RestartStackFrameCommand, StepBackCommand, StepCommand, StepInCommand, StepOutCommand,
    //     TerminateCommand, TerminateThreadsCommand, VariablesCommand,
    // },
    session::{self, Session},
};
use crate::{debugger, worktree_store::WorktreeStore, ProjectEnvironment};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use dap::{
    adapters::{DapStatus, DebugAdapterName},
    client::SessionId,
    messages::Message,
    requests::{
        Completions, Evaluate, Request as _, RunInTerminal, SetExpression, SetVariable,
        StartDebugging,
    },
    Capabilities, CompletionItem, CompletionsArguments, DapRegistry, ErrorResponse,
    EvaluateArguments, EvaluateArgumentsContext, EvaluateResponse, RunInTerminalRequestArguments,
    SetExpressionArguments, SetVariableArguments, Source, StartDebuggingRequestArguments,
};
use fs::Fs;
use futures::{
    channel::{mpsc, oneshot},
    future::{join_all, Shared},
};
use gpui::{App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task};
use http_client::HttpClient;
use language::{BinaryStatus, LanguageRegistry, LanguageToolchainStore};
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;

use rpc::{
    proto::{self},
    AnyProtoClient, TypedEnvelope,
};
use serde_json::Value;
use settings::WorktreeId;
use smol::{lock::Mutex, stream::StreamExt};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    path::PathBuf,
    sync::{atomic::Ordering::SeqCst, Arc},
};
use std::{collections::VecDeque, sync::atomic::AtomicU32};
use task::{DebugAdapterConfig, DebugRequestDisposition};
use util::ResultExt as _;
use worktree::Worktree;

pub enum DapStoreEvent {
    DebugClientStarted(SessionId),
    DebugClientShutdown(SessionId),
    DebugClientEvent {
        session_id: SessionId,
        message: Message,
    },
    RunInTerminal {
        session_id: SessionId,
        title: Option<String>,
        cwd: PathBuf,
        command: Option<String>,
        args: Vec<String>,
        envs: HashMap<String, String>,
        sender: mpsc::Sender<Result<u32>>,
    },
    Notification(String),
    RemoteHasInitialized,
}

#[allow(clippy::large_enum_variant)]
pub enum DapStoreMode {
    Local(LocalDapStore),   // ssh host and collab host
    Remote(RemoteDapStore), // collab guest
}

pub struct LocalDapStore {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    next_session_id: AtomicU32,
    http_client: Arc<dyn HttpClient>,
    worktree_store: Entity<WorktreeStore>,
    environment: Entity<ProjectEnvironment>,
    language_registry: Arc<LanguageRegistry>,
    debug_adapters: Arc<DapRegistry>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    start_debugging_tx: futures::channel::mpsc::UnboundedSender<(SessionId, Message)>,
    _start_debugging_task: Task<()>,
}

impl LocalDapStore {
    fn next_session_id(&self) -> SessionId {
        SessionId(self.next_session_id.fetch_add(1, SeqCst))
    }
}

pub struct RemoteDapStore {
    upstream_client: AnyProtoClient,
    upstream_project_id: u64,
    event_queue: Option<VecDeque<DapStoreEvent>>,
}

pub struct DapStore {
    mode: DapStoreMode,
    downstream_client: Option<(AnyProtoClient, u64)>,
    breakpoint_store: Entity<BreakpointStore>,
    sessions: BTreeMap<SessionId, Entity<Session>>,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    pub fn init(_client: &AnyProtoClient) {
        // todo(debugger): Reenable these after we finish handle_dap_command refactor
        // client.add_entity_request_handler(Self::handle_dap_command::<NextCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<StepInCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<StepOutCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<StepBackCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<ContinueCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<PauseCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<DisconnectCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<TerminateThreadsCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<TerminateCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<RestartCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<VariablesCommand>);
        // client.add_entity_request_handler(Self::handle_dap_command::<RestartStackFrameCommand>);
    }

    #[expect(clippy::too_many_arguments)]
    pub fn new_local(
        http_client: Arc<dyn HttpClient>,
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        language_registry: Arc<LanguageRegistry>,
        debug_adapters: Arc<DapRegistry>,
        environment: Entity<ProjectEnvironment>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        breakpoint_store: Entity<BreakpointStore>,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.on_app_quit(Self::shutdown_sessions).detach();

        let (start_debugging_tx, mut message_rx) =
            futures::channel::mpsc::unbounded::<(SessionId, Message)>();

        let _start_debugging_task = cx.spawn(async move |this, cx| {
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
            mode: DapStoreMode::Local(LocalDapStore {
                fs,
                environment,
                http_client,
                node_runtime,
                worktree_store,
                toolchain_store,
                language_registry,
                debug_adapters,
                start_debugging_tx,
                _start_debugging_task,
                next_session_id: Default::default(),
            }),
            downstream_client: None,
            breakpoint_store,
            sessions: Default::default(),
        }
    }

    pub fn new_remote(
        project_id: u64,
        upstream_client: AnyProtoClient,
        breakpoint_store: Entity<BreakpointStore>,
    ) -> Self {
        Self {
            mode: DapStoreMode::Remote(RemoteDapStore {
                upstream_client,
                upstream_project_id: project_id,
                event_queue: Some(VecDeque::default()),
            }),
            downstream_client: None,
            breakpoint_store,
            sessions: Default::default(),
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
                upstream_client,
                upstream_project_id,
                ..
            }) => Some((upstream_client.clone(), *upstream_project_id)),

            DapStoreMode::Local(_) => None,
        }
    }

    pub fn downstream_client(&self) -> Option<&(AnyProtoClient, u64)> {
        self.downstream_client.as_ref()
    }

    pub fn add_remote_client(
        &mut self,
        session_id: SessionId,
        ignore: Option<bool>,
        cx: &mut Context<Self>,
    ) {
        if let DapStoreMode::Remote(remote) = &self.mode {
            self.sessions.insert(
                session_id,
                cx.new(|_| {
                    debugger::session::Session::remote(
                        session_id,
                        remote.upstream_client.clone(),
                        remote.upstream_project_id,
                        ignore.unwrap_or(false),
                    )
                }),
            );
        } else {
            debug_assert!(false);
        }
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
                Task::ready(())
            }
        })?
        .await;

        Ok(())
    }

    pub fn new_session(
        &mut self,
        config: DebugAdapterConfig,
        worktree: &Entity<Worktree>,
        parent_session: Option<Entity<Session>>,
        cx: &mut Context<Self>,
    ) -> (SessionId, Task<Result<Entity<Session>>>) {
        let Some(local_store) = self.as_local() else {
            unimplemented!("Starting session on remote side");
        };

        let delegate = DapAdapterDelegate::new(
            local_store.fs.clone(),
            worktree.read(cx).id(),
            local_store.node_runtime.clone(),
            local_store.http_client.clone(),
            local_store.language_registry.clone(),
            local_store.toolchain_store.clone(),
            local_store.environment.update(cx, |env, cx| {
                let worktree = worktree.read(cx);
                env.get_environment(Some(worktree.id()), Some(worktree.abs_path()), cx)
            }),
        );
        let session_id = local_store.next_session_id();

        if let Some(session) = &parent_session {
            session.update(cx, |session, _| {
                session.add_child_session_id(session_id);
            });
        }

        let (initialized_tx, initialized_rx) = oneshot::channel();

        let start_client_task = Session::local(
            self.breakpoint_store.clone(),
            session_id,
            parent_session,
            delegate,
            config,
            local_store.start_debugging_tx.clone(),
            initialized_tx,
            local_store.debug_adapters.clone(),
            cx,
        );

        let task = create_new_session(session_id, initialized_rx, start_client_task, cx);
        (session_id, task)
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn new_fake_session(
        &mut self,
        config: DebugAdapterConfig,
        worktree: &Entity<Worktree>,
        parent_session: Option<Entity<Session>>,
        caps: Capabilities,
        fails: bool,
        cx: &mut Context<Self>,
    ) -> (SessionId, Task<Result<Entity<Session>>>) {
        let Some(local_store) = self.as_local() else {
            unimplemented!("Starting session on remote side");
        };

        let delegate = DapAdapterDelegate::new(
            local_store.fs.clone(),
            worktree.read(cx).id(),
            local_store.node_runtime.clone(),
            local_store.http_client.clone(),
            local_store.language_registry.clone(),
            local_store.toolchain_store.clone(),
            local_store.environment.update(cx, |env, cx| {
                let worktree = worktree.read(cx);
                env.get_environment(Some(worktree.id()), Some(worktree.abs_path()), cx)
            }),
        );
        let session_id = local_store.next_session_id();

        if let Some(session) = &parent_session {
            session.update(cx, |session, _| {
                session.add_child_session_id(session_id);
            });
        }

        let (initialized_tx, initialized_rx) = oneshot::channel();

        let start_client_task = Session::fake(
            self.breakpoint_store.clone(),
            session_id,
            parent_session,
            delegate,
            config,
            local_store.start_debugging_tx.clone(),
            initialized_tx,
            caps,
            fails,
            cx,
        );

        let task = create_new_session(session_id, initialized_rx, start_client_task, cx);
        (session_id, task)
    }

    fn handle_start_debugging_request(
        &mut self,
        session_id: SessionId,
        request: dap::messages::Request,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(local_store) = self.as_local() else {
            unreachable!("Cannot response for non-local session");
        };

        let Some(parent_session) = self.session_by_id(session_id) else {
            return Task::ready(Err(anyhow!("Session not found")));
        };

        let args = serde_json::from_value::<StartDebuggingRequestArguments>(
            request.arguments.unwrap_or_default(),
        )
        .expect("To parse StartDebuggingRequestArguments");
        let worktree = local_store
            .worktree_store
            .update(cx, |this, _| this.worktrees().next())
            .expect("worktree-less project");

        let Some(config) = parent_session.read(cx).configuration() else {
            unreachable!("there must be a config for local sessions");
        };

        let debug_config = DebugAdapterConfig {
            label: config.label,
            adapter: config.adapter,
            request: DebugRequestDisposition::ReverseRequest(args),
            initialize_args: config.initialize_args.clone(),
            tcp_connection: config.tcp_connection.clone(),
        };
        #[cfg(any(test, feature = "test-support"))]
        let new_session_task = {
            let caps = parent_session.read(cx).capabilities.clone();
            self.new_fake_session(
                debug_config,
                &worktree,
                Some(parent_session.clone()),
                caps,
                false,
                cx,
            )
            .1
        };
        #[cfg(not(any(test, feature = "test-support")))]
        let new_session_task = self
            .new_session(debug_config, &worktree, Some(parent_session.clone()), cx)
            .1;

        let request_seq = request.seq;
        cx.spawn(async move |_, cx| {
            let (success, body) = match new_session_task.await {
                Ok(_) => (true, None),
                Err(error) => (
                    false,
                    Some(serde_json::to_value(ErrorResponse {
                        error: Some(dap::Message {
                            id: request_seq,
                            format: error.to_string(),
                            variables: None,
                            send_telemetry: None,
                            show_user: None,
                            url: None,
                            url_label: None,
                        }),
                    })?),
                ),
            };

            parent_session
                .update(cx, |session, cx| {
                    session.respond_to_client(
                        request_seq,
                        success,
                        StartDebugging::COMMAND.to_string(),
                        body,
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

        let cwd = PathBuf::from(request_args.cwd);
        match cwd.try_exists() {
            Ok(true) => (),
            Ok(false) | Err(_) => {
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
                })
            }
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

    #[allow(clippy::too_many_arguments)]
    pub fn set_variable_value(
        &self,
        session_id: &SessionId,
        stack_frame_id: u64,
        variables_reference: u64,
        name: String,
        value: String,
        evaluate_name: Option<String>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(client) = self
            .session_by_id(session_id)
            .and_then(|client| client.read(cx).adapter_client())
        else {
            return Task::ready(Err(anyhow!("Could not find client: {:?}", session_id)));
        };

        let supports_set_expression = self
            .capabilities_by_id(session_id, cx)
            .and_then(|caps| caps.supports_set_expression)
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

    // .. get the client and what not
    // let _ = client.modules(); // This can fire a request to a dap adapter or be a cheap getter.
    // client.wait_for_request(request::Modules); // This ensures that the request that we've fired off runs to completions
    // let returned_value = client.modules(); // this is a cheap getter.

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
        let Some(_) = self.as_local_mut() else {
            return Task::ready(Err(anyhow!("Cannot shutdown session on remote side")));
        };

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
            .parent_id()
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
}

fn create_new_session(
    session_id: SessionId,
    initialized_rx: oneshot::Receiver<()>,
    start_client_task: Task<Result<Entity<Session>, anyhow::Error>>,
    cx: &mut Context<'_, DapStore>,
) -> Task<Result<Entity<Session>>> {
    let task = cx.spawn(async move |this, cx| {
        let session = match start_client_task.await {
            Ok(session) => session,
            Err(error) => {
                this.update(cx, |_, cx| {
                    cx.emit(DapStoreEvent::Notification(error.to_string()));
                })
                .log_err();

                return Err(error);
            }
        };

        // we have to insert the session early, so we can handle reverse requests
        // that need the session to be available
        this.update(cx, |store, cx| {
            store.sessions.insert(session_id, session.clone());
            cx.emit(DapStoreEvent::DebugClientStarted(session_id));
            cx.notify();
        })?;

        match session
            .update(cx, |session, cx| {
                session.initialize_sequence(initialized_rx, cx)
            })?
            .await
        {
            Ok(_) => {}
            Err(error) => {
                this.update(cx, |this, cx| {
                    cx.emit(DapStoreEvent::Notification(error.to_string()));

                    this.shutdown_session(session_id, cx)
                })?
                .await
                .log_err();

                return Err(error);
            }
        }

        Ok(session)
    });
    task
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
