use super::{
    AcpDebugLog, AcpDebugMessage, AcpSession, ClientContext, ConfigOptions,
    ContextServerConfiguration, ForegroundWork, PendingSessionUpdates, ReadOnlyConfigOptions,
    UnsupportedVersion, apply_v2_session_notification, close_undelivered_session,
    drain_foreground_queue, enqueue_notification, first_class_terminal_auth_task,
    is_disconnect_error, map_acp_error, mcp_configurations_for_project, runtime, transport,
    with_v2_client_ingress, work_dirs_from_session_info,
};
use acp_thread::{
    AcpThread, AgentConnection, AgentSessionConfigOptions, AgentSessionInfo, AgentSessionList,
    AgentSessionListRequest, AgentSessionListResponse, ElicitationStore, ProtocolErrorInfo,
    ReceiptSessionSubmissions, ReceiptSubmission, SubmissionDisconnected,
};
use action_log::ActionLog;
use agent_client_protocol::{
    Agent, Client, ConnectTo, InitializeOutcome, JsonRpcRequest, JsonRpcResponse, Lines,
    PreparedRequest, RequestCancellationHandle, V2ConnectionTo,
    schema::{ProtocolVersion, v1 as acp_v1, v2 as acp},
};
use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use futures::{
    FutureExt as _,
    channel::mpsc,
    future::{LocalBoxFuture, Shared},
};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, SharedString, Task, WeakEntity};
use project::{
    AgentId, AgentServerStore, Project,
    agent_server_store::{AgentServerCommand, AllAgentServersSettings},
};
use settings::SettingsStore;
use std::{any::Any, cell::RefCell, rc::Rc, sync::Arc};
use task::SpawnInTerminal;
use util::ResultExt as _;
use util::path_list::PathList;

fn is_connection_closed(connection: &V2ConnectionTo<Agent>) -> bool {
    connection.is_incoming_closed() || connection.termination().is_some()
}

/// The SDK reports JSON-RPC IDs with the v1 type, which allows `null`. It never assigns
/// `null` to a request it sends and rejects incoming v2 requests that use it.
fn v2_request_id(request_id: &acp_v1::RequestId) -> Result<acp::RequestId> {
    match request_id {
        acp_v1::RequestId::Number(id) => Ok(acp::RequestId::Number(*id)),
        acp_v1::RequestId::Str(id) => Ok(acp::RequestId::Str(id.clone())),
        acp_v1::RequestId::Null => Err(anyhow!("v2 requests cannot have a null ID")),
    }
}

fn cancel_request_elicitations(
    request_id: acp::RequestId,
    cx: &mut AsyncApp,
    context: &ClientContext,
) {
    if let Some(scopes) = &context.active_request_scopes {
        scopes.borrow_mut().remove(&request_id);
    }
    context.request_elicitations.update(cx, |store, cx| {
        store.cancel_request(&request_id, cx);
    });
}

fn register_request(request_id: acp::RequestId, _cx: &mut AsyncApp, context: &ClientContext) {
    if let Some(scopes) = &context.active_request_scopes {
        scopes.borrow_mut().insert(request_id);
    }
}

fn finish_request(request_id: acp::RequestId, cx: &mut AsyncApp, context: &ClientContext) {
    if let Some(scopes) = &context.active_request_scopes {
        scopes.borrow_mut().remove(&request_id);
    }
    context.request_elicitations.update(cx, |store, cx| {
        store.finish_request(&request_id, cx);
    });
}

fn begin_session_creation(request_id: acp::RequestId, _cx: &mut AsyncApp, context: &ClientContext) {
    if let Some(pending) = &context.pending_session_updates {
        pending.borrow_mut().begin(request_id);
    }
}

fn finish_session_creation(
    request_id: acp::RequestId,
    _cx: &mut AsyncApp,
    context: &ClientContext,
) {
    if let Some(pending) = &context.pending_session_updates {
        pending.borrow_mut().finish(&request_id);
    }
}

#[derive(Debug, thiserror::Error)]
#[error("ACP ordered response callback ended")]
struct ResponseCallbackDropped;

fn validate_prompt_content(
    capabilities: &acp::PromptCapabilities,
    content: &[acp::ContentBlock],
) -> Result<()> {
    for block in content {
        let supported = match block {
            acp::ContentBlock::Text(_) | acp::ContentBlock::ResourceLink(_) => true,
            acp::ContentBlock::Image(_) => capabilities.image.is_some(),
            acp::ContentBlock::Audio(_) => capabilities.audio.is_some(),
            acp::ContentBlock::Resource(_) => capabilities.embedded_context.is_some(),
            _ => false,
        };
        anyhow::ensure!(supported, "Prompt content is not supported by this agent");
    }
    Ok(())
}

fn config_option_supports_value(
    option: &acp::SessionConfigOption,
    value: &acp::SessionConfigOptionValue,
) -> bool {
    match (&option.kind, value) {
        (acp::SessionConfigKind::Boolean(_), acp::SessionConfigOptionValue::Boolean { .. }) => true,
        (acp::SessionConfigKind::Select(select), acp::SessionConfigOptionValue::Id { value }) => {
            match &select.options {
                acp::SessionConfigSelectOptions::Ungrouped(options) => {
                    options.iter().any(|option| &option.value == value)
                }
                acp::SessionConfigSelectOptions::Grouped(groups) => groups
                    .iter()
                    .any(|group| group.options.iter().any(|option| &option.value == value)),
                _ => false,
            }
        }
        _ => false,
    }
}

#[derive(Clone)]
struct SessionInitialization {
    task: Shared<Task<Result<(), Arc<anyhow::Error>>>>,
    request_id: acp::RequestId,
    cancellation: RequestCancellationHandle,
}

#[derive(Clone, PartialEq)]
struct SessionContext {
    cwd: acp::AbsolutePath,
    additional_directories: Vec<acp::AbsolutePath>,
    mcp_servers: Vec<acp::McpServer>,
}

impl SessionContext {
    fn work_dirs(&self) -> PathList {
        PathList::new(
            &std::iter::once(self.cwd.0.clone())
                .chain(
                    self.additional_directories
                        .iter()
                        .map(|path| path.0.clone()),
                )
                .collect::<Vec<_>>(),
        )
    }
}

struct SessionOwner {
    thread: WeakEntity<AcpThread>,
    project_entity_id: gpui::EntityId,
    context: SessionContext,
    history_loaded: bool,
    initialization: Option<SessionInitialization>,
    closing: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
}

impl SessionOwner {
    fn ensure_ready(&self, thread_id: gpui::EntityId, session: Option<&AcpSession>) -> Result<()> {
        anyhow::ensure!(
            self.thread.entity_id() == thread_id
                && self.thread.upgrade().is_some()
                && session.is_some_and(|session| session.thread.entity_id() == thread_id),
            "Session has been released or replaced"
        );
        anyhow::ensure!(
            self.initialization.is_none(),
            "Session is still initializing"
        );
        anyhow::ensure!(self.closing.is_none(), "Session is closing");
        Ok(())
    }
}

fn shared_session_error(error: Arc<anyhow::Error>) -> anyhow::Error {
    if let Some(protocol_error) = error.downcast_ref::<agent_client_protocol::Error>() {
        map_acp_error(protocol_error.clone())
    } else {
        anyhow!(error)
    }
}

/// An ACP v2 backend with auth, initial sessions/config and receipt-driven prompts.
pub struct AcpV2Connection {
    id: AgentId,
    connection: V2ConnectionTo<Agent>,
    initialize_request: acp::InitializeRequest,
    initialize_response: acp::InitializeResponse,
    agent_server_store: WeakEntity<AgentServerStore>,
    request_elicitations: Entity<ElicitationStore>,
    debug_log: AcpDebugLog,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    pending_session_updates: Rc<RefCell<PendingSessionUpdates>>,
    session_owners: Rc<RefCell<HashMap<acp::SessionId, SessionOwner>>>,
    runtime: Rc<RefCell<Option<ConnectionRuntime>>>,
}

struct ConnectionRuntime {
    _client: runtime::ClientRuntime,
    _stderr_task: Shared<Task<()>>,
    _wait_task: Option<Task<()>>,
    _child: Option<transport::OwnedChild>,
    _quit_subscription: Option<gpui::Subscription>,
}

struct StdioStartup {
    child: transport::OwnedChild,
    stderr_task: Shared<Task<()>>,
    runtime: Rc<RefCell<Option<ConnectionRuntime>>>,
    drain: Option<futures::channel::oneshot::Sender<LocalBoxFuture<'static, ()>>>,
}

impl AcpV2Connection {
    pub async fn stdio(
        agent_id: AgentId,
        project: Entity<Project>,
        command: AgentServerCommand,
        agent_server_store: WeakEntity<AgentServerStore>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let transport::StdioProcess {
            child,
            incoming,
            outgoing,
            stderr,
            debug_log,
        } = transport::spawn_stdio(&project, command, cx)?;
        let mut child = transport::OwnedChild(child);
        let status = child
            .0
            .status()
            .map(|result| {
                result.map_err(|error| anyhow!("failed to wait for agent server exit: {error}"))
            })
            .boxed_local();
        let stderr_task = cx
            .background_spawn(async move {
                stderr.await.log_err();
            })
            .shared();
        let info = cx.update(|cx| {
            acp::Implementation::new("zed", release_channel::AppVersion::global(cx).to_string())
                .title(
                    release_channel::ReleaseChannel::try_global(cx)
                        .map(|channel| channel.display_name().to_owned()),
                )
        });
        // Keep cleanup from killing the child before startup error arbitration can
        // distinguish a protocol rejection from a natural process exit.
        let startup_runtime = Rc::new(RefCell::new(None));
        let (drain_sender, drain_received) = futures::channel::oneshot::channel();
        let initialized = {
            let mut cx = cx.clone();
            let debug_log = debug_log.clone();
            let stderr_task = stderr_task.clone();
            let runtime = startup_runtime.clone();
            async move {
                Self::connect(
                    agent_id,
                    agent_server_store,
                    Lines::new(outgoing, incoming),
                    info,
                    debug_log,
                    Some(StdioStartup {
                        child,
                        stderr_task,
                        runtime,
                        drain: Some(drain_sender),
                    }),
                    &mut cx,
                )
                .await
            }
        }
        .boxed_local();
        let drained = async move {
            match drain_received.await {
                Ok(drained) => drained.await,
                Err(_) => stderr_task.await,
            }
        }
        .boxed_local();
        let result = runtime::race_startup(initialized, status, drained, &debug_log, cx).await;
        drop(startup_runtime);
        let (connection, status, drained) =
            result.map_err(|error| super::initialization_error(error, &debug_log))?;
        let mut runtime = connection.runtime.borrow_mut();
        let owned = runtime
            .as_mut()
            .context("Agent connection shut down during startup")?;
        owned._wait_task = Some(runtime::watch_process(
            status,
            drained,
            debug_log,
            |error, _cx| {
                log::warn!("ACP v2 agent exited: {error}");
            },
            cx,
        ));
        drop(runtime);
        Ok(connection)
    }

    async fn connect(
        id: AgentId,
        agent_server_store: WeakEntity<AgentServerStore>,
        transport: impl ConnectTo<Client> + 'static,
        info: acp::Implementation,
        debug_log: AcpDebugLog,
        startup: Option<StdioStartup>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let (runtime, child, stderr_task, drain_sender) = match startup {
            Some(startup) => (
                startup.runtime,
                Some(startup.child),
                startup.stderr_task,
                startup.drain,
            ),
            None => (
                Rc::new(RefCell::new(None)),
                None,
                Task::ready(()).shared(),
                None,
            ),
        };
        let request_elicitations = cx.new(|_| ElicitationStore::default());
        let sessions = Rc::new(RefCell::new(HashMap::default()));
        let pending_session_updates = Rc::new(RefCell::new(PendingSessionUpdates::default()));
        let context = ClientContext {
            sessions: sessions.clone(),
            pending_session_updates: Some(pending_session_updates.clone()),
            session_list: Rc::new(RefCell::new(None)),
            request_elicitations: request_elicitations.clone(),
            active_request_scopes: Some(RefCell::new(Default::default())),
            accepts_subagent_updates: true,
        };
        let (mut client_runtime, shutdown) = runtime::ClientRuntime::new(context, cx);
        let dispatch_tx = client_runtime.sender();
        let (sender, received) = futures::channel::oneshot::channel();
        let cleanup = runtime::disconnect_cleanup(dispatch_tx.clone());
        let connection_future =
            with_v2_client_ingress(Client.v2().name("zed-v2"), dispatch_tx.clone())
                .on_close({
                    let cleanup = cleanup.clone();
                    async move |_connection: V2ConnectionTo<Agent>| cleanup.await
                })
                .connect_with(transport, async move |connection| {
                    runtime::publish_connection(
                        connection,
                        sender,
                        runtime::shutdown_requested(shutdown),
                    )
                    .await
                });
        client_runtime.start_io(runtime::finish_connection(connection_future, cleanup), cx);
        let stderr_for_drain = stderr_task.clone();
        *runtime.borrow_mut() = Some(ConnectionRuntime {
            _client: client_runtime,
            _stderr_task: stderr_task,
            _wait_task: None,
            _child: child,
            _quit_subscription: None,
        });
        let quit = cx.update(|cx| {
            cx.on_app_quit({
                let runtime = Rc::downgrade(&runtime);
                move |_| {
                    if let Some(runtime) = runtime.upgrade() {
                        let owned = runtime.borrow_mut().take();
                        drop(owned);
                    }
                    async {}
                }
            })
        });
        if let Some(runtime) = runtime.borrow_mut().as_mut() {
            runtime._quit_subscription = Some(quit);
        }
        let connection = received
            .await
            .context("Failed to receive ACP v2 connection handle")?;
        if let Some(sender) = drain_sender {
            let sdk_connection = connection.clone();
            let drained = runtime::drain_transport(
                async move { sdk_connection.incoming_closed().await },
                dispatch_tx.clone(),
                stderr_for_drain,
            )
            .boxed_local();
            if sender.send(drained).is_err() {
                log::debug!("ACP startup drain observer was released");
            }
        }
        let supports_terminal_auth = agent_server_store.upgrade().is_some_and(|store| {
            store.update(cx, |store, _| store.get_external_agent(&id).is_some())
        });
        let mut capabilities = acp::ClientCapabilities::new().elicitation(
            acp::ElicitationCapabilities::new()
                .form(acp::ElicitationFormCapabilities::new())
                .url(acp::ElicitationUrlCapabilities::new()),
        );
        if supports_terminal_auth {
            capabilities = capabilities.auth(
                acp::ClientAuthCapabilities::new().terminal(acp::TerminalAuthCapabilities::new()),
            );
        }
        let initialize_request =
            acp::InitializeRequest::new(ProtocolVersion::V2, info).capabilities(capabilities);
        let initialize_response = match connection
            .send_initialize(initialize_request.clone())
            .block_task()
            .await
        {
            Ok(InitializeOutcome::Initialized(response)) => response,
            Ok(InitializeOutcome::LowerVersion(mismatch)) => {
                return Err(anyhow!(mismatch).context(UnsupportedVersion));
            }
            Ok(outcome) => {
                return Err(anyhow!(
                    "Unexpected ACP initialization outcome: {outcome:?}"
                ));
            }
            Err(error) if super::v1_only_mismatch(&error) => {
                return Err(anyhow!(error).context(UnsupportedVersion));
            }
            Err(error) => return Err(map_acp_error(error)),
        };
        anyhow::ensure!(
            runtime.borrow().is_some(),
            "Agent connection shut down during startup"
        );
        Ok(Self {
            id,
            connection,
            initialize_request,
            initialize_response,
            agent_server_store,
            request_elicitations,
            debug_log,
            dispatch_tx,
            sessions,
            pending_session_updates,
            session_owners: Rc::new(RefCell::new(HashMap::default())),
            runtime,
        })
    }

    pub fn initialize_request(&self) -> &acp::InitializeRequest {
        &self.initialize_request
    }

    pub fn initialize_response(&self) -> &acp::InitializeResponse {
        &self.initialize_response
    }

    pub fn request_elicitations(&self) -> Entity<ElicitationStore> {
        self.request_elicitations.clone()
    }

    pub fn subscribe_debug_messages(
        &self,
    ) -> (
        Vec<AcpDebugMessage>,
        async_channel::Receiver<AcpDebugMessage>,
    ) {
        self.debug_log.subscribe()
    }

    pub fn terminal_auth_task(
        &self,
        method_id: &acp::AuthMethodId,
        cx: &App,
    ) -> Option<Task<Result<SpawnInTerminal>>> {
        self.initialize_request
            .capabilities
            .auth
            .as_ref()?
            .terminal
            .as_ref()?;
        let method = self
            .initialize_response
            .auth_methods
            .iter()
            .find(|method| method.method_id() == method_id)?;
        match method {
            acp::AuthMethod::Terminal(terminal) => Some(first_class_terminal_auth_task(
                self.id.clone(),
                self.agent_server_store.clone(),
                terminal.clone(),
                cx,
            )),
            _ => None,
        }
    }

    pub fn login(
        &self,
        request: acp::LoginAuthRequest,
        cx: &mut App,
    ) -> Task<Result<acp::LoginAuthResponse>> {
        cx.foreground_executor().spawn(self.login_response(request))
    }

    fn login_response(
        &self,
        request: acp::LoginAuthRequest,
    ) -> LocalBoxFuture<'static, Result<acp::LoginAuthResponse>> {
        if !self.initialize_response.auth_methods.iter().any(|method| {
            matches!(method, acp::AuthMethod::Agent(agent) if agent.method_id == request.method_id)
        }) {
            return futures::future::ready(Err(anyhow!(
                "Authentication method is not supported for agent-managed login"
            )))
            .boxed_local();
        }
        Self::scoped_request(&self.connection, &self.dispatch_tx, request)
    }

    pub fn supports_logout(&self) -> bool {
        !self.initialize_response.auth_methods.is_empty()
    }

    pub fn logout(
        &self,
        request: acp::LogoutAuthRequest,
        cx: &mut App,
    ) -> Task<Result<acp::LogoutAuthResponse>> {
        cx.foreground_executor()
            .spawn(self.logout_response(request))
    }

    fn logout_response(
        &self,
        request: acp::LogoutAuthRequest,
    ) -> LocalBoxFuture<'static, Result<acp::LogoutAuthResponse>> {
        if !self.supports_logout() {
            return futures::future::ready(Err(anyhow!("Logout is not advertised by this agent")))
                .boxed_local();
        }
        Self::scoped_request(&self.connection, &self.dispatch_tx, request)
    }

    /// Dropping this acceptance RPC only requests RPC cancellation, not cancellation of
    /// accepted session work. Callers still own session registration and submission tracking.
    pub fn request_prompt_receipt(
        &self,
        request: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        if self.initialize_response.capabilities.session.is_none() {
            return Task::ready(Err(anyhow!("Sessions are not advertised by this agent")));
        }
        if let Err(error) = self.validate_prompt_content(&request.prompt) {
            return Task::ready(Err(error));
        }
        let response = Self::ordered_request(
            &self.connection,
            &self.dispatch_tx,
            request,
            || Ok(()),
            |_| None,
            |response, _| Ok(response),
            cx,
        );
        cx.foreground_executor().spawn(response)
    }

    fn scoped_request<Request: JsonRpcRequest + 'static>(
        connection: &V2ConnectionTo<Agent>,
        dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
        request: Request,
    ) -> LocalBoxFuture<'static, Result<Request::Response>> {
        let connection = connection.clone();
        let dispatch_tx = dispatch_tx.clone();
        // An eagerly spawned child can publish before cancellation drops an unpolled caller.
        async move {
            let request = connection.prepare_request(request);
            let request_id = v2_request_id(request.id())?;
            let cancellation = request.cancellation_handle();
            enqueue_notification(&dispatch_tx, request_id.clone(), register_request);
            let response = request.block_task();
            // Publication must precede caller-drop cancellation; early SDK cancellation is inert.
            let guard = util::defer({
                let dispatch_tx = dispatch_tx.clone();
                let request_id = request_id.clone();
                move || {
                    cancellation.cancel().log_err();
                    enqueue_notification(&dispatch_tx, request_id, cancel_request_elicitations);
                }
            });
            let response = response.await.map_err(map_acp_error)?;
            enqueue_notification(&dispatch_tx, request_id, finish_request);
            guard.abort();
            drain_foreground_queue(&dispatch_tx).await?;
            Ok(response)
        }
        .boxed_local()
    }

    fn session_context(
        &self,
        configurations: Vec<(String, Arc<ContextServerConfiguration>)>,
        work_dirs: &PathList,
        is_absolute: impl Fn(&std::path::Path) -> bool,
    ) -> Result<SessionContext> {
        let capabilities = self
            .initialize_response
            .capabilities
            .session
            .as_ref()
            .context("This agent does not advertise session support")?;
        let mut paths = work_dirs.ordered_paths();
        let cwd = paths
            .next()
            .cloned()
            .context("Working directory cannot be empty")?;
        anyhow::ensure!(is_absolute(&cwd), "Working directory must be absolute");
        let additional_directories = if capabilities.additional_directories.is_some() {
            paths.cloned().collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        anyhow::ensure!(
            additional_directories.iter().all(|path| is_absolute(path)),
            "Additional directories must be absolute"
        );
        let mut servers = Vec::new();
        for (name, configuration) in configurations {
            match &*configuration {
                ContextServerConfiguration::Custom { command, .. }
                | ContextServerConfiguration::Extension { command, .. }
                    if capabilities
                        .mcp
                        .as_ref()
                        .is_some_and(|mcp| mcp.stdio.is_some()) =>
                {
                    anyhow::ensure!(
                        is_absolute(&command.path),
                        "MCP stdio command must be absolute"
                    );
                    servers.push(acp::McpServer::Stdio(
                        acp::McpServerStdio::new(name, command.path.clone())
                            .args(command.args.clone())
                            .env(
                                command
                                    .env
                                    .iter()
                                    .flat_map(|environment| environment.iter())
                                    .map(|(name, value)| acp::EnvVariable::new(name, value))
                                    .collect(),
                            ),
                    ));
                }
                ContextServerConfiguration::Http { url, headers, .. }
                    if capabilities
                        .mcp
                        .as_ref()
                        .is_some_and(|mcp| mcp.http.is_some()) =>
                {
                    servers.push(acp::McpServer::Http(
                        acp::McpServerHttp::new(name, url.to_string()).headers(
                            headers
                                .iter()
                                .map(|(name, value)| acp::HttpHeader::new(name, value))
                                .collect(),
                        ),
                    ));
                }
                _ => {}
            }
        }
        Ok(SessionContext {
            cwd: cwd.into(),
            additional_directories: additional_directories.into_iter().map(Into::into).collect(),
            mcp_servers: servers,
        })
    }

    fn remove_session(&self, session_id: &acp::SessionId, thread_id: gpui::EntityId) {
        let matches = self
            .sessions
            .borrow()
            .get(session_id)
            .is_some_and(|session| session.thread.entity_id() == thread_id);
        if matches {
            self.sessions.borrow_mut().remove(session_id);
        }
        let matches = self
            .session_owners
            .borrow()
            .get(session_id)
            .is_some_and(|owner| owner.thread.entity_id() == thread_id);
        if matches {
            self.session_owners.borrow_mut().remove(session_id);
        }
    }

    fn create_session_thread(
        self: &Rc<Self>,
        session_id: acp::SessionId,
        project: Entity<Project>,
        context: SessionContext,
        title: Option<SharedString>,
        cx: &mut App,
    ) -> Result<Entity<AcpThread>> {
        anyhow::ensure!(
            !self.sessions.borrow().contains_key(&session_id),
            "Agent returned a session ID that is already registered"
        );
        let capabilities = self
            .initialize_response
            .capabilities
            .session
            .as_ref()
            .context("This agent does not advertise session support")?
            .prompt
            .clone()
            .unwrap_or_default();
        let project_entity_id = project.entity_id();
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let thread = cx.new(|cx| {
            AcpThread::new(
                None,
                title,
                Some(context.work_dirs()),
                self.clone(),
                project,
                action_log,
                session_id.clone(),
                watch::Receiver::constant(capabilities),
                cx,
            )
        });
        let release_subscription =
            cx.observe_release(&thread, {
                let this = Rc::downgrade(self);
                let session_id = session_id.clone();
                let thread_id = thread.entity_id();
                move |_, _cx| {
                    if let Some(this) = this.upgrade() {
                        let initialization = this
                            .session_owners
                            .borrow()
                            .get(&session_id)
                            .filter(|owner| owner.thread.entity_id() == thread_id)
                            .and_then(|owner| owner.initialization.clone());
                        if let Some(initialization) = initialization {
                            initialization.cancellation.cancel().log_err();
                            enqueue_notification(
                                &this.dispatch_tx,
                                initialization.request_id,
                                cancel_request_elicitations,
                            );
                        } else if !this.session_owners.borrow().get(&session_id).is_some_and(
                            |owner| {
                                owner.thread.entity_id() == thread_id && owner.closing.is_some()
                            },
                        ) {
                            this.remove_session(&session_id, thread_id);
                        }
                    }
                }
            });
        self.sessions.borrow_mut().insert(
            session_id.clone(),
            AcpSession {
                thread: thread.downgrade(),
                is_subagent: false,
                suppress_abort_err: false,
                session_modes: None,
                config_options: Some(ConfigOptions::new(Rc::new(RefCell::new(Vec::new())))),
                default_config_task: None,
                _release_subscription: release_subscription,
            },
        );
        self.session_owners.borrow_mut().insert(
            session_id,
            SessionOwner {
                thread: thread.downgrade(),
                project_entity_id,
                context,
                history_loaded: true,
                initialization: None,
                closing: None,
            },
        );
        Ok(thread)
    }

    fn apply_initial_state(
        &self,
        thread: &Entity<AcpThread>,
        config_options: Vec<acp::SessionConfigOption>,
        available_commands: Vec<acp::AvailableCommand>,
        cx: &mut App,
    ) -> Result<()> {
        let options = self
            .sessions
            .borrow()
            .get(thread.read(cx).session_id())
            .filter(|session| session.thread.entity_id() == thread.entity_id())
            .and_then(|session| session.config_options.clone())
            .context("Session was released before initialization completed")?;
        *options.config_options.borrow_mut() = config_options.clone();
        options.tx.borrow_mut().send(()).log_err();
        thread.update(cx, |thread, cx| {
            thread.update_available_commands(available_commands, cx);
            cx.emit(acp_thread::AcpThreadEvent::ConfigOptionsUpdated(
                config_options,
            ));
        });
        Ok(())
    }

    async fn apply_new_session_defaults(
        &self,
        thread: &Entity<AcpThread>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let config_options = cx
            .update(|cx| self.session_config_options(thread.read(cx).session_id(), cx))
            .context("New session was released before applying defaults")?;
        let mut attempted = Vec::new();
        loop {
            let next = cx.update(|cx| {
                let settings = cx
                    .try_global::<SettingsStore>()?
                    .get::<AllAgentServersSettings>(None);
                let defaults = settings.get(self.id.as_ref())?;
                config_options
                    .config_options()
                    .into_iter()
                    .find_map(|option| {
                        if attempted.contains(&option.config_id) {
                            return None;
                        }
                        let default =
                            defaults.default_config_option(option.config_id.0.as_ref())?;
                        let value = match &option.kind {
                            acp::SessionConfigKind::Boolean(boolean) => default
                                .as_bool()
                                .filter(|value| *value != boolean.current_value)
                                .map(acp::SessionConfigOptionValue::boolean),
                            acp::SessionConfigKind::Select(select) => default
                                .as_value_id()
                                .filter(|value| *value != select.current_value.0.as_ref())
                                .map(|value| acp::SessionConfigOptionValue::id(value.to_owned())),
                            _ => None,
                        }?;
                        config_option_supports_value(&option, &value)
                            .then_some((option.config_id, value))
                    })
            });
            let Some((config_id, value)) = next else {
                break;
            };
            // Re-read dependent offers after each reply, but never fight the agent
            // by retrying a rejected or normalized default for the same option.
            attempted.push(config_id.clone());
            let result = cx
                .update(|cx| config_options.set_config_option(config_id.clone(), value, cx))
                .await
                .with_context(|| format!("Failed to apply default config option {config_id}"));
            if result.as_ref().err().is_some_and(|error| {
                error.is::<ResponseCallbackDropped>()
                    || is_connection_closed(&self.connection)
                    || is_disconnect_error(&error)
            }) {
                return result.map(|_| ());
            }
            if let Err(error) = result {
                log::warn!("{error:#}");
                let message = ProtocolErrorInfo::from_error(&error)
                    .map(|info| info.message.to_owned())
                    .unwrap_or_else(|| error.to_string());
                cx.update(|cx| {
                    thread.update(cx, |thread, cx| {
                        thread.push_notice(
                            acp::Notice::new(
                                acp::NoticeSeverity::Warning,
                                "Saved config default was not applied",
                            )
                            .description(format!("{config_id}: {message}")),
                            cx,
                        );
                    });
                });
            }
        }
        Ok(())
    }

    fn initialized_thread(
        thread: Entity<AcpThread>,
        initialization: SessionInitialization,
        cx: &App,
    ) -> Task<Result<Entity<AcpThread>>> {
        cx.foreground_executor().spawn(async move {
            initialization.task.await.map_err(shared_session_error)?;
            Ok(thread)
        })
    }

    fn observe_close(
        task: Shared<Task<Result<(), Arc<anyhow::Error>>>>,
        thread: Option<Entity<AcpThread>>,
        cx: &App,
    ) -> Task<Result<()>> {
        cx.foreground_executor().spawn(async move {
            let _thread = thread;
            task.await.map_err(shared_session_error)
        })
    }

    /// The subagents RFD allows no Client-initiated mutation of a subagent session except
    /// the `cancel` its parent advertises, which `AcpThread::cancel` checks.
    fn is_subagent_session(&self, session_id: &acp::SessionId) -> bool {
        self.sessions
            .borrow()
            .get(session_id)
            .is_some_and(|session| session.is_subagent && session.thread.upgrade().is_some())
    }

    fn open_session(
        self: Rc<Self>,
        session_id: acp::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        replay_history: bool,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        if is_connection_closed(&self.connection) {
            return Task::ready(Err(anyhow!("Agent connection is closed")));
        }
        if self.is_subagent_session(&session_id) {
            return Task::ready(Err(anyhow!(
                "ACP subagent sessions are loaded with their parent"
            )));
        }
        let path_style = project.read(cx).path_style(cx);
        let context = match self.session_context(
            mcp_configurations_for_project(&project, cx),
            &work_dirs,
            |path| path_style.is_absolute(path),
        ) {
            Ok(context) => context,
            Err(error) => return Task::ready(Err(error)),
        };
        let existing = self.session_owners.borrow().get(&session_id).map(|owner| {
            (
                owner.thread.clone(),
                owner.project_entity_id,
                owner.context.clone(),
                owner.history_loaded,
                owner.initialization.clone(),
                owner.closing.is_some(),
            )
        });
        if let Some((
            thread,
            project_entity_id,
            previous_context,
            history_loaded,
            initialization,
            closing,
        )) = existing
        {
            if closing {
                return Task::ready(Err(anyhow!("Session is closing")));
            }
            if let Some(thread) = thread.upgrade() {
                if previous_context != context || project_entity_id != project.entity_id() {
                    return Task::ready(Err(anyhow!(
                        "Session is already open with a different context"
                    )));
                }
                if replay_history && !history_loaded {
                    return Task::ready(Err(anyhow!(
                        "Cannot replay history into an already resumed session"
                    )));
                }
                return match initialization {
                    Some(initialization) => Self::initialized_thread(thread, initialization, cx),
                    None => Task::ready(Ok(thread)),
                };
            }
            if let Some(initialization) = initialization {
                return cx.spawn(async move |cx| {
                    if let Err(error) = initialization.task.await {
                        log::debug!("Abandoned session initialization ended: {error:#}");
                    }
                    cx.update(|cx| {
                        self.open_session(session_id, project, work_dirs, title, replay_history, cx)
                    })
                    .await
                });
            }
            self.remove_session(&session_id, thread.entity_id());
        }
        let mut request = acp::ResumeSessionRequest::new(session_id.clone(), context.cwd.clone())
            .additional_directories(context.additional_directories.clone())
            .mcp_servers(context.mcp_servers.clone());
        if replay_history {
            request = request.replay_from(acp::ReplayFrom::Start(acp::ReplayFromStart::new()));
        }
        let thread =
            match self.create_session_thread(session_id.clone(), project, context, title, cx) {
                Ok(thread) => thread,
                Err(error) => return Task::ready(Err(error)),
            };
        thread.update(cx, |thread, cx| thread.mark_activity_unconfirmed(cx));
        let request = self.connection.prepare_request(request);
        let request_id = match v2_request_id(request.id()) {
            Ok(request_id) => request_id,
            Err(error) => return Task::ready(Err(error)),
        };
        let cancellation = request.cancellation_handle();
        let (response, applied) = Self::ordered_prepared_response(
            &self.connection,
            &self.dispatch_tx,
            request,
            || Ok(()),
            |_| None,
        );
        let response = Self::apply_ordered_response(
            response,
            applied,
            &self.dispatch_tx,
            {
                let this = Rc::downgrade(&self);
                let thread = thread.downgrade();
                move |response, cx| {
                    let this = this.upgrade().context("Connection was released")?;
                    let thread = thread.upgrade().context("Session was released")?;
                    cx.update(|cx| {
                        // Ordered responses apply after the replay and before any update sent
                        // after the response, so this is the RFD's freshness boundary.
                        if replay_history {
                            thread.update(cx, |thread, cx| {
                                thread.invalidate_replayed_subagent_state(cx)
                            });
                        }
                        if !thread.read(cx).has_received_session_state() {
                            log::warn!(
                                "Agent resumed the session without reporting its current foreground state; activity remains unconfirmed"
                            );
                        }
                        this.apply_initial_state(
                            &thread,
                            response.config_options,
                            response.available_commands,
                            cx,
                        )
                    })
                }
            },
            cx,
        );
        // RPC cancellation preserves its response. Keep the weak owner reserved until
        // that response and preceding replay settle, without retaining an abandoned view.
        let task = cx
            .spawn({
                let this = Rc::downgrade(&self);
                let thread = thread.downgrade();
                let session_id = session_id.clone();
                let dispatch_tx = self.dispatch_tx.clone();
                async move |_cx| {
                    if thread.upgrade().is_none() {
                        if let Some(this) = this.upgrade() {
                            this.remove_session(&session_id, thread.entity_id());
                        }
                        return Ok(());
                    }
                    let response = response.await;
                    if response.is_err() {
                        drain_foreground_queue(&dispatch_tx).await.log_err();
                    }
                    if let Some(this) = this.upgrade() {
                        if response.is_ok() && thread.upgrade().is_some() {
                            if let Some(owner) = this
                                .session_owners
                                .borrow_mut()
                                .get_mut(&session_id)
                                .filter(|owner| owner.thread.entity_id() == thread.entity_id())
                            {
                                owner.initialization = None;
                            }
                        } else {
                            this.remove_session(&session_id, thread.entity_id());
                        }
                    }
                    response.map_err(Arc::new)
                }
            })
            .shared();
        let initialization = SessionInitialization {
            task,
            request_id,
            cancellation,
        };
        {
            let mut owners = self.session_owners.borrow_mut();
            let Some(owner) = owners
                .get_mut(&session_id)
                .filter(|owner| owner.thread.entity_id() == thread.entity_id())
            else {
                return Task::ready(Err(anyhow!("Session was released before initialization")));
            };
            owner.history_loaded = replay_history;
            owner.initialization = Some(initialization.clone());
        }
        Self::initialized_thread(thread, initialization, cx)
    }

    fn ordered_response<Request: JsonRpcRequest + 'static>(
        connection: &V2ConnectionTo<Agent>,
        dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
        request: Request,
        preflight: impl FnOnce() -> Result<()> + 'static,
        abandoned_session: fn(&Request::Response) -> Option<acp::SessionId>,
    ) -> (
        LocalBoxFuture<'static, Result<Request::Response>>,
        futures::channel::oneshot::Sender<bool>,
    ) {
        Self::ordered_prepared_response(
            connection,
            dispatch_tx,
            connection.prepare_request(request),
            preflight,
            abandoned_session,
        )
    }

    fn ordered_prepared_response<Response: JsonRpcResponse + 'static>(
        connection: &V2ConnectionTo<Agent>,
        dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
        request: PreparedRequest<Response>,
        preflight: impl FnOnce() -> Result<()> + 'static,
        abandoned_session: fn(&Response) -> Option<acp::SessionId>,
    ) -> (
        LocalBoxFuture<'static, Result<Response>>,
        futures::channel::oneshot::Sender<bool>,
    ) {
        let connection = connection.clone();
        let dispatch_tx = dispatch_tx.clone();
        let (applied, application_finished) = futures::channel::oneshot::channel();
        let response = async move {
            preflight()?;
            let (sender, received) = futures::channel::oneshot::channel();
            let request_id = v2_request_id(request.id())?;
            let cancellation = request.cancellation_handle();
            enqueue_notification(&dispatch_tx, request_id.clone(), register_request);
            let published = request.on_receiving_result({
                let connection = connection.clone();
                let dispatch_tx = dispatch_tx.clone();
                let request_id = request_id.clone();
                async move |result: Result<Response, agent_client_protocol::Error>| {
                    let abandoned = result.as_ref().ok().and_then(abandoned_session);
                    let successful_response = result.is_ok();
                    let application = if sender.send(result).is_ok() {
                        application_finished.await.ok()
                    } else {
                        None
                    };
                    enqueue_notification(
                        &dispatch_tx,
                        request_id,
                        if successful_response && application == Some(true) {
                            finish_request
                        } else {
                            cancel_request_elicitations
                        },
                    );
                    if application.is_none()
                        && let Some(session_id) = abandoned
                    {
                        enqueue_notification(
                            &dispatch_tx,
                            (session_id, connection.clone()),
                            close_undelivered_session,
                        );
                    }
                    drain_foreground_queue(&dispatch_tx).await.log_err();
                    Ok(())
                }
            });
            if let Err(error) = published {
                enqueue_notification(&dispatch_tx, request_id, cancel_request_elicitations);
                return Err(map_acp_error(error));
            }
            let guard = util::defer({
                let dispatch_tx = dispatch_tx.clone();
                let request_id = request_id.clone();
                move || {
                    cancellation.cancel().log_err();
                    enqueue_notification(&dispatch_tx, request_id, cancel_request_elicitations);
                }
            });
            let response = received.await.context(ResponseCallbackDropped)?;
            guard.abort();
            response.map_err(map_acp_error)
        }
        .boxed_local();
        (response, applied)
    }

    fn ordered_request<Request: JsonRpcRequest + 'static, Output: 'static>(
        connection: &V2ConnectionTo<Agent>,
        dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
        request: Request,
        preflight: impl FnOnce() -> Result<()> + 'static,
        abandoned_session: fn(&Request::Response) -> Option<acp::SessionId>,
        apply: impl FnOnce(Request::Response, &mut AsyncApp) -> Result<Output> + 'static,
        cx: &App,
    ) -> LocalBoxFuture<'static, Result<Output>> {
        let (response, applied) = Self::ordered_response(
            connection,
            dispatch_tx,
            request,
            preflight,
            abandoned_session,
        );
        Self::apply_ordered_response(response, applied, dispatch_tx, apply, cx)
    }

    fn apply_ordered_response<Response: 'static, Output: 'static>(
        response: LocalBoxFuture<'static, Result<Response>>,
        applied: futures::channel::oneshot::Sender<bool>,
        dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
        apply: impl FnOnce(Response, &mut AsyncApp) -> Result<Output> + 'static,
        cx: &App,
    ) -> LocalBoxFuture<'static, Result<Output>> {
        let dispatch_tx = dispatch_tx.clone();
        let mut cx = cx.to_async();
        async move {
            let result = response.await.and_then(|response| apply(response, &mut cx));
            if result.is_ok() {
                drain_foreground_queue(&dispatch_tx).await?;
            }
            if applied.send(result.is_ok()).is_err() && result.is_ok() {
                return Err(anyhow!(
                    "ACP response ordering callback ended before application"
                ));
            }
            result
        }
        .boxed_local()
    }
}

impl AgentConnection for AcpV2Connection {
    fn shutdown(&self) {
        let runtime = self.runtime.borrow_mut().take();
        drop(runtime);
    }

    fn agent_id(&self) -> AgentId {
        self.id.clone()
    }

    fn telemetry_id(&self) -> SharedString {
        self.initialize_response.info.name.clone().into()
    }

    fn agent_version(&self) -> Option<SharedString> {
        let version = &self.initialize_response.info.version;
        (!version.is_empty()).then(|| version.clone().into())
    }

    fn new_session(
        self: Rc<Self>,
        project: Entity<Project>,
        work_dirs: PathList,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let path_style = project.read(cx).path_style(cx);
        let context = match self.session_context(
            mcp_configurations_for_project(&project, cx),
            &work_dirs,
            |path| path_style.is_absolute(path),
        ) {
            Ok(context) => context,
            Err(error) => return Task::ready(Err(error)),
        };
        let request = acp::NewSessionRequest::new(context.cwd.clone())
            .additional_directories(context.additional_directories.clone())
            .mcp_servers(context.mcp_servers.clone());
        let request = self.connection.prepare_request(request);
        let request_id = match v2_request_id(request.id()) {
            Ok(request_id) => request_id,
            Err(error) => return Task::ready(Err(error)),
        };
        let (response, applied) = Self::ordered_prepared_response(
            &self.connection,
            &self.dispatch_tx,
            request,
            {
                let dispatch_tx = self.dispatch_tx.clone();
                let request_id = request_id.clone();
                move || {
                    enqueue_notification(&dispatch_tx, request_id, begin_session_creation);
                    Ok(())
                }
            },
            |response| Some(response.session_id.clone()),
        );
        let response = {
            let dispatch_tx = self.dispatch_tx.clone();
            async move {
                let response = response.await?;
                drain_foreground_queue(&dispatch_tx).await?;
                Ok(response)
            }
            .boxed_local()
        };
        let response = Self::apply_ordered_response(
            response,
            applied,
            &self.dispatch_tx,
            {
                let this = self.clone();
                let request_id = request_id.clone();
                move |response, cx| {
                    let session_id = response.session_id.clone();
                    let notifications = this
                        .pending_session_updates
                        .borrow_mut()
                        .take(&request_id, &session_id);
                    let notifications = match notifications {
                        Ok(notifications) => notifications,
                        Err(error) => {
                            enqueue_notification(
                                &this.dispatch_tx,
                                (session_id, this.connection.clone()),
                                close_undelivered_session,
                            );
                            return Err(error);
                        }
                    };
                    let thread = cx.update(|cx| {
                        this.create_session_thread(session_id.clone(), project, context, None, cx)
                    })?;
                    let result = (|| {
                        for notification in notifications {
                            // Traffic for the new session's subagents goes to their own threads,
                            // which the announcements before it created.
                            let target = this
                                .sessions
                                .borrow()
                                .get(&notification.session_id)
                                .map(|session| session.thread.clone())
                                .with_context(|| {
                                    format!(
                                        "Session update for unannounced session {}",
                                        notification.session_id
                                    )
                                })?;
                            apply_v2_session_notification(
                                notification,
                                &target,
                                &this.sessions,
                                cx,
                            )?;
                        }
                        cx.update(|cx| {
                            this.apply_initial_state(
                                &thread,
                                response.config_options,
                                response.available_commands,
                                cx,
                            )
                        })
                    })();
                    if let Err(error) = result {
                        this.remove_session(&session_id, thread.entity_id());
                        enqueue_notification(
                            &this.dispatch_tx,
                            (session_id, this.connection.clone()),
                            close_undelivered_session,
                        );
                        return Err(error);
                    }
                    Ok(thread)
                }
            },
            cx,
        );
        let cleanup = util::defer({
            let dispatch_tx = self.dispatch_tx.clone();
            move || enqueue_notification(&dispatch_tx, request_id, finish_session_creation)
        });
        cx.spawn(async move |cx| {
            let thread = {
                let _cleanup = cleanup;
                response.await?
            };
            let session_id = cx.update(|cx| thread.read(cx).session_id().clone());
            let abandoned = util::defer({
                let this = self.clone();
                let thread_id = thread.entity_id();
                move || {
                    this.remove_session(&session_id, thread_id);
                    enqueue_notification(
                        &this.dispatch_tx,
                        (session_id, this.connection.clone()),
                        close_undelivered_session,
                    );
                }
            });
            // Further requests must follow the creation acknowledgement, not await
            // later SDK traffic inside its ordered response callback.
            self.apply_new_session_defaults(&thread, cx).await?;
            abandoned.abort();
            Ok(thread)
        })
    }

    fn supports_load_session(&self) -> bool {
        self.initialize_response.capabilities.session.is_some()
    }

    fn supports_resume_session(&self) -> bool {
        self.initialize_response.capabilities.session.is_some()
    }

    fn load_session(
        self: Rc<Self>,
        session_id: acp::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        self.open_session(session_id, project, work_dirs, title, true, cx)
    }

    fn resume_session(
        self: Rc<Self>,
        session_id: acp::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        self.open_session(session_id, project, work_dirs, title, false, cx)
    }

    fn supports_close_session(&self) -> bool {
        self.initialize_response.capabilities.session.is_some()
    }

    fn announces_subagents(&self) -> bool {
        // Subagents are part of v2's baseline session model.
        true
    }

    fn close_session(
        self: Rc<Self>,
        session_id: &acp::SessionId,
        cx: &mut App,
    ) -> Task<Result<()>> {
        if !self.supports_close_session() {
            return Task::ready(Err(anyhow!("Closing sessions is not supported")));
        }
        if self.is_subagent_session(session_id) {
            // The agent releases a subagent when its parent closes.
            return Task::ready(Err(anyhow!(
                "ACP subagent sessions are closed with their parent"
            )));
        }
        let owner = self.session_owners.borrow().get(session_id).map(|owner| {
            (
                owner.thread.clone(),
                owner.initialization.is_some(),
                owner.closing.clone(),
            )
        });
        let Some((thread, initializing, closing)) = owner else {
            return Task::ready(Err(anyhow!("Session has been released")));
        };
        if let Some(closing) = closing {
            return Self::observe_close(closing, thread.upgrade(), cx);
        }
        if is_connection_closed(&self.connection) {
            return Task::ready(Err(anyhow!("Agent connection is closed")));
        }
        if initializing {
            return Task::ready(Err(anyhow!("Session is still initializing")));
        }
        let Some(thread) = thread.upgrade() else {
            return Task::ready(Err(anyhow!("Session has been released")));
        };
        let thread_id = thread.entity_id();
        let response = Self::ordered_request(
            &self.connection,
            &self.dispatch_tx,
            acp::CloseSessionRequest::new(session_id.clone()),
            {
                let this = Rc::downgrade(&self);
                let session_id = session_id.clone();
                move || {
                    let this = this.upgrade().context("Connection was released")?;
                    anyhow::ensure!(
                        this.session_owners
                            .borrow()
                            .get(&session_id)
                            .is_some_and(|owner| {
                                owner.thread.entity_id() == thread_id && owner.closing.is_some()
                            })
                            && this
                                .sessions
                                .borrow()
                                .get(&session_id)
                                .is_some_and(|session| { session.thread.entity_id() == thread_id }),
                        "Session close was superseded"
                    );
                    Ok(())
                }
            },
            |_| None,
            // Stop must not wake before the response's foreground barrier applies final updates.
            |_, _| Ok(()),
            cx,
        );
        let task = cx
            .spawn({
                let this = Rc::downgrade(&self);
                let thread = thread.downgrade();
                let session_id = session_id.clone();
                let dispatch_tx = self.dispatch_tx.clone();
                async move |cx| {
                    let result = response.await;
                    if result.is_err() {
                        drain_foreground_queue(&dispatch_tx).await.log_err();
                    }
                    if let Some(this) = this.upgrade() {
                        if result.is_ok() {
                            if let Some(thread) = thread.upgrade() {
                                thread.update(cx, |thread, cx| thread.handle_session_closed(cx));
                            }
                            this.remove_session(&session_id, thread_id);
                        } else {
                            let released = {
                                let mut owners = this.session_owners.borrow_mut();
                                owners
                                    .get_mut(&session_id)
                                    .filter(|owner| owner.thread.entity_id() == thread_id)
                                    .is_some_and(|owner| {
                                        owner.closing = None;
                                        owner.thread.upgrade().is_none()
                                    })
                            };
                            if released {
                                this.remove_session(&session_id, thread_id);
                            }
                        }
                    }
                    result.map_err(Arc::new)
                }
            })
            .shared();
        if let Some(owner) = self.session_owners.borrow_mut().get_mut(session_id) {
            owner.closing = Some(task.clone());
        }
        thread.update(cx, |thread, cx| thread.prepare_session_close(cx));
        Self::observe_close(task, Some(thread), cx)
    }

    fn receipt_submissions(
        &self,
        session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn ReceiptSessionSubmissions>> {
        let session = self.initialize_response.capabilities.session.as_ref()?;
        Some(Rc::new(V2SessionSubmissions {
            session_id: session_id.clone(),
            sessions: self.sessions.clone(),
            owners: self.session_owners.clone(),
            capabilities: session.prompt.clone().unwrap_or_default(),
            connection: self.connection.clone(),
            dispatch_tx: self.dispatch_tx.clone(),
        }))
    }

    fn validate_prompt_content(&self, content: &[acp::ContentBlock]) -> Result<()> {
        let session = self
            .initialize_response
            .capabilities
            .session
            .as_ref()
            .context("This agent does not advertise session support")?;
        validate_prompt_content(&session.prompt.clone().unwrap_or_default(), content)
    }

    fn should_close_sessions_on_release(&self) -> bool {
        // V2 close cancels agent-owned work; disposing a view must only detach.
        false
    }

    fn supports_session_additional_directories(&self) -> bool {
        self.initialize_response
            .capabilities
            .session
            .as_ref()
            .is_some_and(|session| session.additional_directories.is_some())
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &self.initialize_response.auth_methods
    }

    fn terminal_auth_task(
        &self,
        method: &acp::AuthMethodId,
        cx: &App,
    ) -> Option<Task<Result<SpawnInTerminal>>> {
        AcpV2Connection::terminal_auth_task(self, method, cx)
    }

    fn authenticate(&self, method: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
        let login = self.login_response(acp::LoginAuthRequest::new(method));
        cx.foreground_executor().spawn(async move {
            login.await?;
            Ok(())
        })
    }

    fn supports_logout(&self) -> bool {
        AcpV2Connection::supports_logout(self)
    }

    fn logout(&self, cx: &mut App) -> Task<Result<()>> {
        let logout = self.logout_response(acp::LogoutAuthRequest::new());
        cx.foreground_executor().spawn(async move {
            logout.await?;
            Ok(())
        })
    }

    fn cancel(&self, session_id: &acp::SessionId, _cx: &mut App) -> Result<()> {
        anyhow::ensure!(
            self.sessions
                .borrow()
                .get(session_id)
                .is_some_and(|session| session.thread.upgrade().is_some()),
            "Session has been released"
        );
        anyhow::ensure!(
            !is_connection_closed(&self.connection),
            "Agent connection is closed"
        );
        self.connection
            .send_notification(acp::CancelSessionNotification::new(session_id.clone()))
            .map_err(map_acp_error)
    }

    fn request_elicitations(&self) -> Option<Entity<ElicitationStore>> {
        Some(self.request_elicitations.clone())
    }

    fn session_config_options(
        &self,
        session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn AgentSessionConfigOptions>> {
        if self.is_subagent_session(session_id) {
            let sessions = self.sessions.borrow();
            return Some(Rc::new(ReadOnlyConfigOptions(
                sessions.get(session_id)?.config_options.clone()?,
            )));
        }
        let sessions = self.sessions.borrow();
        let session = sessions.get(session_id)?;
        Some(Rc::new(V2SessionConfigOptions {
            session_id: session_id.clone(),
            thread: session.thread.clone(),
            sessions: self.sessions.clone(),
            owners: self.session_owners.clone(),
            options: session.config_options.clone()?,
            connection: self.connection.clone(),
            dispatch_tx: self.dispatch_tx.clone(),
        }))
    }

    fn session_list(&self, _cx: &mut App) -> Option<Rc<dyn AgentSessionList>> {
        let capabilities = self.initialize_response.capabilities.session.as_ref()?;
        Some(Rc::new(V2SessionList {
            connection: self.connection.clone(),
            dispatch_tx: self.dispatch_tx.clone(),
            supports_delete: capabilities.delete.is_some(),
        }))
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

struct V2SessionList {
    connection: V2ConnectionTo<Agent>,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
    supports_delete: bool,
}

impl AgentSessionList for V2SessionList {
    fn list_sessions(
        &self,
        request: AgentSessionListRequest,
        cx: &mut App,
    ) -> Task<Result<AgentSessionListResponse>> {
        let response = AcpV2Connection::scoped_request(
            &self.connection,
            &self.dispatch_tx,
            acp::ListSessionsRequest::new()
                .cwd(request.cwd.map(acp::AbsolutePath::new))
                .cursor(request.cursor.map(acp::SessionListCursor::new))
                .meta(request.meta),
        );
        cx.foreground_executor().spawn(async move {
            let response = response.await?;
            Ok(AgentSessionListResponse {
                sessions: response
                    .sessions
                    .into_iter()
                    .map(|session| {
                        let mut information = AgentSessionInfo::new(session.session_id);
                        information.work_dirs = Some(work_dirs_from_session_info(
                            session.cwd.into_inner(),
                            session
                                .additional_directories
                                .into_iter()
                                .map(acp::AbsolutePath::into_inner)
                                .collect(),
                        ));
                        information.apply_update(
                            acp::SessionInfoUpdate::new()
                                .title(session.title)
                                .updated_at(session.updated_at),
                        );
                        // List metadata is a snapshot, so null-valued keys are data.
                        information.meta = session.meta;
                        information
                    })
                    .collect(),
                next_cursor: response.next_cursor.map(|cursor| cursor.0.to_string()),
                meta: response.meta,
            })
        })
    }

    fn supports_delete(&self) -> bool {
        self.supports_delete
    }

    fn delete_session(&self, session_id: &acp::SessionId, cx: &mut App) -> Task<Result<()>> {
        if !self.supports_delete() {
            return Task::ready(Err(anyhow!("Deleting sessions is not supported")));
        }
        let response = AcpV2Connection::scoped_request(
            &self.connection,
            &self.dispatch_tx,
            acp::DeleteSessionRequest::new(session_id.clone()),
        );
        cx.foreground_executor().spawn(async move {
            response.await?;
            Ok(())
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[derive(Clone)]
struct V2SessionConfigOptions {
    session_id: acp::SessionId,
    thread: WeakEntity<AcpThread>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    owners: Rc<RefCell<HashMap<acp::SessionId, SessionOwner>>>,
    options: ConfigOptions,
    connection: V2ConnectionTo<Agent>,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
}

impl V2SessionConfigOptions {
    fn validate(
        &self,
        config_id: &acp::SessionConfigId,
        value: &acp::SessionConfigOptionValue,
    ) -> Result<()> {
        self.owners
            .borrow()
            .get(&self.session_id)
            .context("Session has been released")?
            .ensure_ready(
                self.thread.entity_id(),
                self.sessions.borrow().get(&self.session_id),
            )?;
        let valid = self.options.config_options.borrow().iter().any(|option| {
            &option.config_id == config_id && config_option_supports_value(option, value)
        });
        if !valid {
            return Err(anyhow!(
                "Config option or value is not offered by this session"
            ));
        }
        Ok(())
    }
}

impl AgentSessionConfigOptions for V2SessionConfigOptions {
    fn config_options(&self) -> Vec<acp::SessionConfigOption> {
        self.options.config_options.borrow().clone()
    }

    fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
        Some(self.options.rx.clone())
    }

    fn set_config_option(
        &self,
        config_id: acp::SessionConfigId,
        value: acp::SessionConfigOptionValue,
        cx: &mut App,
    ) -> Task<Result<Vec<acp::SessionConfigOption>>> {
        if let Err(error) = self.validate(&config_id, &value) {
            return Task::ready(Err(error));
        }
        let options = self.options.clone();
        let sessions = self.sessions.clone();
        let session_id = self.session_id.clone();
        let thread = self.thread.clone();
        let preflight = {
            let this = self.clone();
            let config_id = config_id.clone();
            let value = value.clone();
            move || this.validate(&config_id, &value)
        };
        let response = AcpV2Connection::ordered_request(
            &self.connection,
            &self.dispatch_tx,
            acp::SetSessionConfigOptionRequest::new(session_id.clone(), config_id, value),
            preflight,
            |_| None,
            move |response, cx| {
                anyhow::ensure!(
                    sessions
                        .borrow()
                        .get(&session_id)
                        .is_some_and(|session| session.thread.entity_id() == thread.entity_id())
                        && thread.upgrade().is_some(),
                    "Session has been released or replaced"
                );
                *options.config_options.borrow_mut() = response.config_options.clone();
                options
                    .tx
                    .borrow_mut()
                    .send(())
                    .context("Config watcher has closed")?;
                thread.update(cx, |_, cx| {
                    cx.emit(acp_thread::AcpThreadEvent::ConfigOptionsUpdated(
                        response.config_options.clone(),
                    ))
                })?;
                Ok(response.config_options)
            },
            cx,
        );
        cx.foreground_executor().spawn(response)
    }
}

struct V2SessionSubmissions {
    session_id: acp::SessionId,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    owners: Rc<RefCell<HashMap<acp::SessionId, SessionOwner>>>,
    capabilities: acp::PromptCapabilities,
    connection: V2ConnectionTo<Agent>,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
}

impl V2SessionSubmissions {
    fn request(
        &self,
        owner: WeakEntity<AcpThread>,
        content: Vec<acp::ContentBlock>,
    ) -> ReceiptSubmission {
        if let Err(error) = validate_prompt_content(&self.capabilities, &content) {
            return Task::ready(Err(error)).into();
        }
        let sessions = self.sessions.clone();
        let owners = self.owners.clone();
        let session_id = self.session_id.clone();
        let connection = self.connection.clone();
        let (response, applied) = AcpV2Connection::ordered_response(
            &self.connection,
            &self.dispatch_tx,
            acp::PromptRequest::new(self.session_id.clone(), content),
            move || {
                owners
                    .borrow()
                    .get(&session_id)
                    .context("Session has been released or replaced")?
                    .ensure_ready(owner.entity_id(), sessions.borrow().get(&session_id))
            },
            |_| None,
        );
        let response = async move {
            response.await.map_err(|error| {
                if error.is::<ResponseCallbackDropped>()
                    || is_connection_closed(&connection)
                    || is_disconnect_error(&error)
                {
                    error.context(SubmissionDisconnected)
                } else {
                    error
                }
            })
        }
        .boxed_local();
        ReceiptSubmission::ordered(response, applied)
    }
}

impl ReceiptSessionSubmissions for V2SessionSubmissions {
    fn submit(
        &self,
        content: Vec<acp::ContentBlock>,
        cx: &mut Context<AcpThread>,
    ) -> ReceiptSubmission {
        self.request(cx.entity().downgrade(), content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::ElicitationStatus;
    use agent_client_protocol::{
        Responder,
        schema::v1::{self as acp_v1, ErrorCode},
    };
    use gpui::TestAppContext;
    use serde_json::json;
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    struct Harness {
        client: AcpV2Connection,
        agent: V2ConnectionTo<Client>,
        logins: Arc<Mutex<Vec<(acp::LoginAuthRequest, acp::RequestId)>>>,
        logouts: Arc<Mutex<Vec<acp::LogoutAuthRequest>>>,
        prompts: async_channel::Receiver<(acp::PromptRequest, Responder<acp::PromptResponse>)>,
        session_cancellations: Arc<Mutex<Vec<acp::CancelSessionNotification>>>,
        _agent_task: Task<Result<(), agent_client_protocol::Error>>,
    }

    fn meta(scope: &str) -> acp::Meta {
        acp::Meta::from_iter([("scope".to_owned(), json!(scope))])
    }

    fn session_path(relative: &str) -> std::path::PathBuf {
        let root = if cfg!(windows) { "C:\\" } else { "/" };
        std::path::Path::new(root).join(relative)
    }

    struct SessionHarness {
        client: Rc<AcpV2Connection>,
        project: Entity<Project>,
        prompts: async_channel::Receiver<(acp::PromptRequest, Responder<acp::PromptResponse>)>,
        cancellations: async_channel::Receiver<acp::CancelSessionNotification>,
        new_requests: async_channel::Receiver<(acp::NewSessionRequest, acp::RequestId)>,
        new_responses:
            async_channel::Sender<Result<acp::NewSessionResponse, agent_client_protocol::Error>>,
        new_replies:
            async_channel::Receiver<(acp::NewSessionRequest, Responder<acp::NewSessionResponse>)>,
        hold_new_responses: Arc<Mutex<bool>>,
        config_requests: async_channel::Receiver<acp::SetSessionConfigOptionRequest>,
        config_responses: async_channel::Sender<
            Result<acp::SetSessionConfigOptionResponse, agent_client_protocol::Error>,
        >,
        config_replies: async_channel::Receiver<Responder<acp::SetSessionConfigOptionResponse>>,
        hold_config: Arc<Mutex<bool>>,
        list_requests: async_channel::Receiver<(
            acp::ListSessionsRequest,
            Responder<acp::ListSessionsResponse>,
        )>,
        delete_requests: async_channel::Receiver<(
            acp::DeleteSessionRequest,
            Responder<acp::DeleteSessionResponse>,
        )>,
        closes: async_channel::Receiver<acp::SessionId>,
        close_requests: async_channel::Receiver<(
            acp::CloseSessionRequest,
            Responder<acp::CloseSessionResponse>,
        )>,
        hold_closes: Arc<Mutex<bool>>,
        resume_requests: async_channel::Receiver<(
            acp::ResumeSessionRequest,
            Responder<acp::ResumeSessionResponse>,
        )>,
        agent: V2ConnectionTo<Client>,
        _agent_task: Task<Result<(), agent_client_protocol::Error>>,
    }

    impl SessionHarness {
        async fn new(
            capabilities: acp::AgentCapabilities,
            after_new: Vec<acp::SessionUpdate>,
            after_config: Vec<acp::SessionUpdate>,
            cx: &mut TestAppContext,
        ) -> Self {
            cx.update(|cx| {
                let store = settings::SettingsStore::test(cx);
                cx.set_global(store);
            });
            let filesystem = fs::FakeFs::new(cx.executor());
            filesystem
                .insert_tree(session_path(""), json!({"a": {}, "b": {}}))
                .await;
            let project = Project::test(filesystem, [session_path("a").as_path()], cx).await;
            let (new_sender, new_requests) = async_channel::unbounded();
            let (new_responses, new_received) = async_channel::unbounded::<
                Result<acp::NewSessionResponse, agent_client_protocol::Error>,
            >();
            let (new_reply_sender, new_replies) = async_channel::unbounded();
            let hold_new_responses = Arc::new(Mutex::new(false));
            let deferred_new = hold_new_responses.clone();
            let (list_sender, list_requests) = async_channel::unbounded();
            let (delete_sender, delete_requests) = async_channel::unbounded();
            let (config_sender, config_requests) = async_channel::unbounded();
            let (config_responses, config_received) = async_channel::unbounded::<
                Result<acp::SetSessionConfigOptionResponse, agent_client_protocol::Error>,
            >();
            let (config_reply_sender, config_replies) = async_channel::unbounded();
            let hold_config = Arc::new(Mutex::new(false));
            let deferred_config = hold_config.clone();
            let (close_sender, closes) = async_channel::unbounded();
            let (close_request_sender, close_requests) = async_channel::unbounded();
            let hold_closes = Arc::new(Mutex::new(false));
            let deferred_closes = hold_closes.clone();
            let (resume_sender, resume_requests) = async_channel::unbounded();
            let (prompt_sender, prompts) = async_channel::unbounded();
            let (cancel_sender, cancellations) = async_channel::unbounded();
            let (sender, received) = futures::channel::oneshot::channel();
            let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
            let agent_task = cx.background_spawn(
                Agent
                    .v2()
                    .on_receive_request(
                        async move |_: acp::InitializeRequest,
                                    responder: Responder<acp::InitializeResponse>,
                                    _| {
                            responder.respond(
                                acp::InitializeResponse::new(
                                    ProtocolVersion::V2,
                                    acp::Implementation::new("session-agent", "1"),
                                )
                                .capabilities(capabilities.clone()),
                            )
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::ListSessionsRequest, responder, _| {
                            list_sender
                                .try_send((request, responder))
                                .expect("list receiver");
                            Ok(())
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::DeleteSessionRequest, responder, _| {
                            delete_sender
                                .try_send((request, responder))
                                .expect("delete receiver");
                            Ok(())
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::NewSessionRequest,
                                    responder: Responder<acp::NewSessionResponse>,
                                    agent: V2ConnectionTo<Client>| {
                            new_sender
                                .try_send((
                                    request.clone(),
                                    v2_request_id(responder.id()).expect("v2 request ID"),
                                ))
                                .expect("record creation");
                            if *deferred_new.lock().expect("new response mode") {
                                new_reply_sender
                                    .try_send((request, responder))
                                    .expect("new reply receiver");
                                return Ok(());
                            }
                            match new_received.recv().await.expect("new response") {
                                Ok(response) => {
                                    let session_id = response.session_id.clone();
                                    responder.respond(response)?;
                                    for update in after_new.clone() {
                                        agent.send_notification(
                                            acp::UpdateSessionNotification::new(
                                                session_id.clone(),
                                                update,
                                            ),
                                        )?;
                                    }
                                    Ok(())
                                }
                                Err(error) => responder.respond_with_error(error),
                            }
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::SetSessionConfigOptionRequest,
                                    responder: Responder<acp::SetSessionConfigOptionResponse>,
                                    agent: V2ConnectionTo<Client>| {
                            let session_id = request.session_id.clone();
                            config_sender.try_send(request).expect("record config");
                            if *deferred_config.lock().expect("config mode") {
                                config_reply_sender
                                    .try_send(responder)
                                    .expect("config receiver");
                                return Ok(());
                            }
                            match config_received.recv().await.expect("config response") {
                                Ok(response) => {
                                    responder.respond(response)?;
                                    for update in after_config.clone() {
                                        agent.send_notification(
                                            acp::UpdateSessionNotification::new(
                                                session_id.clone(),
                                                update,
                                            ),
                                        )?;
                                    }
                                    Ok(())
                                }
                                Err(error) => responder.respond_with_error(error),
                            }
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::CloseSessionRequest,
                                    responder: Responder<acp::CloseSessionResponse>,
                                    _| {
                            close_sender
                                .try_send(request.session_id.clone())
                                .expect("record abandoned close");
                            if *deferred_closes.lock().expect("close mode") {
                                close_request_sender
                                    .try_send((request, responder))
                                    .expect("close receiver");
                                Ok(())
                            } else {
                                responder.respond(acp::CloseSessionResponse::new())
                            }
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::ResumeSessionRequest, responder, _| {
                            resume_sender
                                .try_send((request, responder))
                                .expect("resume receiver");
                            Ok(())
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::PromptRequest, responder, _| {
                            prompt_sender
                                .try_send((request, responder))
                                .expect("prompt receiver");
                            Ok(())
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_notification(
                        async move |notification: acp::CancelSessionNotification, _| {
                            cancel_sender
                                .try_send(notification)
                                .expect("cancel receiver");
                            Ok(())
                        },
                        agent_client_protocol::on_receive_notification!(),
                    )
                    .connect_with(agent_transport, async move |agent| {
                        assert!(sender.send(agent).is_ok());
                        futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    }),
            );
            let client = bounded(
                AcpV2Connection::connect(
                    AgentId::new("session-test"),
                    WeakEntity::new_invalid(),
                    client_transport,
                    acp::Implementation::new("test", "1"),
                    AcpDebugLog::default(),
                    None,
                    &mut cx.to_async(),
                ),
                cx,
            )
            .await
            .expect("connect session client");
            Self {
                client: Rc::new(client),
                project,
                prompts,
                cancellations,
                new_requests,
                new_responses,
                new_replies,
                hold_new_responses,
                config_requests,
                config_responses,
                config_replies,
                hold_config,
                list_requests,
                delete_requests,
                closes,
                close_requests,
                hold_closes,
                resume_requests,
                agent: received.await.expect("agent"),
                _agent_task: agent_task,
            }
        }

        fn send_update(&self, session_id: &str, update: acp::SessionUpdate) {
            self.agent
                .send_notification(acp::UpdateSessionNotification::new(session_id, update))
                .expect("session update");
        }

        async fn drain_updates(&self, cx: &TestAppContext) {
            // A round trip also covers notifications not yet enqueued on the foreground.
            let error = bounded(
                self.agent
                    .send_request(acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationSessionScope::new("unbound-barrier"),
                            acp::ElicitationSchema::new(),
                        ),
                        "Inbound ordering barrier",
                    ))
                    .block_task(),
                cx,
            )
            .await
            .expect_err("barrier has no session owner");
            assert_eq!(
                error,
                agent_client_protocol::Error::internal_error()
                    .data("unknown session: unbound-barrier")
            );
        }

        fn create(&self, cx: &mut TestAppContext) -> Task<Result<Entity<AcpThread>>> {
            cx.update(|cx| {
                self.client.clone().new_session(
                    self.project.clone(),
                    PathList::new(&[session_path("a"), session_path("b")]),
                    cx,
                )
            })
        }

        fn resume_state(&self, state: acp::StateUpdate) {
            self.agent
                .send_notification(acp::UpdateSessionNotification::new(
                    "saved",
                    acp::SessionUpdate::StateUpdate(state),
                ))
                .expect("current resume state");
        }

        fn reopen(
            &self,
            replay_history: bool,
            cx: &mut TestAppContext,
        ) -> Task<Result<Entity<AcpThread>>> {
            cx.update(|cx| {
                let directories = PathList::new(&[session_path("a"), session_path("b")]);
                if replay_history {
                    self.client.clone().load_session(
                        "saved".into(),
                        self.project.clone(),
                        directories,
                        Some("Cached title".into()),
                        cx,
                    )
                } else {
                    self.client.clone().resume_session(
                        "saved".into(),
                        self.project.clone(),
                        directories,
                        Some("Cached title".into()),
                        cx,
                    )
                }
            })
        }

        async fn thread(&self, cx: &mut TestAppContext) -> Entity<AcpThread> {
            let mut creation = self.create(cx);
            self.new_request(&mut creation, cx).await;
            self.new_responses
                .try_send(Ok(initial_response()))
                .expect("initial response");
            let thread = bounded(creation, cx).await.expect("create thread");
            cx.run_until_parked();
            thread
        }

        fn defer_closes(&self) {
            *self.hold_closes.lock().expect("close mode") = true;
        }

        fn close(&self, cx: &mut TestAppContext) -> Task<Result<()>> {
            cx.update(|cx| self.client.clone().close_session(&"saved".into(), cx))
        }

        async fn resumed_thread(
            &self,
            state: Option<acp::StateUpdate>,
            cx: &mut TestAppContext,
        ) -> Entity<AcpThread> {
            let loading = self.reopen(false, cx);
            let (_, responder) = bounded(self.resume_requests.recv(), cx)
                .await
                .expect("resume");
            if let Some(state) = state {
                self.resume_state(state);
            }
            let initial = initial_response();
            responder
                .respond(
                    acp::ResumeSessionResponse::new()
                        .config_options(initial.config_options)
                        .available_commands(initial.available_commands),
                )
                .expect("resumed");
            bounded(loading, cx).await.expect("resumed owner")
        }

        async fn new_request(
            &self,
            creation: &mut Task<Result<Entity<AcpThread>>>,
            cx: &TestAppContext,
        ) -> (acp::NewSessionRequest, acp::RequestId) {
            match bounded(
                futures::future::select(self.new_requests.recv().boxed_local(), creation),
                cx,
            )
            .await
            {
                futures::future::Either::Left((request, _)) => request.expect("creation request"),
                futures::future::Either::Right((result, _)) => {
                    panic!("Creation ended before its request was observed: {result:?}")
                }
            }
        }
    }

    fn session_capabilities() -> acp::AgentCapabilities {
        acp::AgentCapabilities::new().session(acp::SessionCapabilities::new())
    }

    fn initial_response() -> acp::NewSessionResponse {
        acp::NewSessionResponse::new("session")
            .config_options(vec![
                acp::SessionConfigOption::boolean("enabled", "Enabled", false)
                    .meta(meta("initial-config")),
            ])
            .available_commands(vec![acp::AvailableCommand::Command(
                acp::Command::new("initial", "Initial command").meta(meta("initial-command")),
            )])
            .meta(meta("delivery-not-session"))
    }

    fn set_startup_defaults(
        defaults: HashMap<String, settings::AgentConfigOptionValue>,
        cx: &mut TestAppContext,
    ) {
        use project::agent_server_store::AllAgentServersSettings;
        use settings::Settings;

        cx.update(|cx| {
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::from_iter([(
                    "session-test".into(),
                    settings::CustomAgentServerSettings::Custom {
                        path: "test-agent".into(),
                        args: Vec::new(),
                        env: HashMap::default(),
                        default_mode: Some("legacy-must-not-run".into()),
                        default_config_options: defaults,
                        favorite_config_option_values: HashMap::default(),
                    }
                    .into(),
                )])),
                cx,
            );
        });
    }

    #[gpui::test]
    async fn v2_session_messages_reach_both_transcripts(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let parent = harness.thread(cx).await;
        harness.send_update(
            "session",
            acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("child")),
        );
        let message = acp::SessionMessage::new("delegation")
            .sender_session_id(acp::SessionId::new("session"))
            .recipient_session_id(acp::SessionId::new("child"))
            .content(vec!["Investigate the failure".into()]);
        harness.send_update(
            "session",
            acp::SessionUpdate::SessionMessage(message.clone()),
        );
        harness.send_update("child", acp::SessionUpdate::SessionMessage(message));
        harness.send_update(
            "child",
            acp::SessionUpdate::SessionMessageChunk(acp::SessionMessageChunk::new(
                "delegation",
                " on Windows".into(),
            )),
        );
        harness.drain_updates(cx).await;

        let child = parent
            .read_with(cx, |parent, _| parent.subagent(&"child".into()).cloned())
            .expect("announced subagent");
        for (thread, expected_blocks) in [
            (&parent, vec!["Investigate the failure".into()]),
            (
                &child,
                vec!["Investigate the failure".into(), " on Windows".into()],
            ),
        ] {
            thread.read_with(cx, |thread, _| {
                let [acp_thread::AgentThreadEntry::SessionMessage(message)] = thread.entries()
                else {
                    panic!("expected one session message in {}", thread.session_id());
                };
                assert_eq!(message.sender_session_id, Some("session".into()));
                assert_eq!(message.recipient_session_id, Some("child".into()));
                assert_eq!(message.content.source_blocks(), expected_blocks.as_slice());
            });
        }
    }

    #[gpui::test]
    async fn v2_subagent_config_options_are_read_only(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let parent = harness.thread(cx).await;
        harness.send_update(
            "session",
            acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("child")),
        );
        harness.drain_updates(cx).await;

        let options = cx
            .read(|cx| harness.client.session_config_options(&"child".into(), cx))
            .expect("the provider exists before the first config update");
        assert!(options.is_read_only());
        assert!(options.config_options().is_empty(), "no inherited values");
        let mut updates = cx
            .update(|cx| options.watch(cx))
            .expect("child config watcher");
        let model = |value: &str| {
            acp::SessionConfigOption::select(
                "model",
                "Model",
                value.to_owned(),
                vec![acp::SessionConfigSelectOption::new(value, value)],
            )
        };
        for snapshot in [
            vec![
                model("child-model"),
                acp::SessionConfigOption::boolean("fast", "Fast", true),
            ],
            vec![model("fallback-model")],
            vec![],
        ] {
            harness.send_update(
                "child",
                acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate::new(
                    snapshot.clone(),
                )),
            );
            harness.drain_updates(cx).await;
            bounded(updates.recv(), cx).await.expect("config update");
            assert_eq!(options.config_options(), snapshot);
        }
        for (config_id, value) in [
            ("model", acp::SessionConfigOptionValue::id("fallback-model")),
            ("fast", acp::SessionConfigOptionValue::boolean(false)),
        ] {
            let error = cx
                .update(|cx| options.set_config_option(config_id.into(), value, cx))
                .await
                .expect_err("child options cannot be set");
            assert!(error.to_string().contains("read-only"));
        }
        assert!(
            harness.config_requests.try_recv().is_err(),
            "no wire mutation"
        );
        parent.read_with(cx, |parent, cx| {
            let options = harness
                .client
                .session_config_options(parent.session_id(), cx)
                .expect("parent config");
            assert!(!options.is_read_only());
        });
    }

    #[gpui::test]
    async fn v2_resume_replay_delivers_subagent_history(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let loading = harness.reopen(true, cx);
        let (_, responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("resume");
        harness.send_update(
            "saved",
            acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("child")),
        );
        harness.send_update(
            "child",
            acp::SessionUpdate::AgentMessage(
                acp::AgentMessage::new("child-history").content(vec!["Replayed".into()]),
            ),
        );
        let initial = initial_response();
        responder
            .respond(
                acp::ResumeSessionResponse::new()
                    .config_options(initial.config_options)
                    .available_commands(initial.available_commands),
            )
            .expect("resumed");
        let parent = bounded(loading, cx).await.expect("resumed parent");
        cx.run_until_parked();

        let child = parent
            .read_with(cx, |parent, _| parent.subagent(&"child".into()).cloned())
            .expect("the replayed announcement creates the subagent");
        child.read_with(cx, |child, _| {
            assert_eq!(
                child.entries().len(),
                1,
                "history replayed before the resume response reaches the subagent"
            );
        });
    }

    #[gpui::test]
    async fn v2_replayed_subagent_state_is_history(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let announce_running_child = || {
            harness.send_update(
                "saved",
                acp::SessionUpdate::SubagentUpdate(
                    acp::SubagentUpdate::new("child")
                        .title("Investigator")
                        .capabilities(
                            acp::SubagentSessionCapabilities::new()
                                .cancel(acp::SessionCancelCapabilities::new()),
                        ),
                ),
            );
            harness.send_update(
                "child",
                acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                    acp::RunningStateUpdate::new(),
                )),
            );
        };
        let cancelled = || {
            acp::StateUpdate::Idle(
                acp::IdleStateUpdate::new().stop_reason(acp::StopReason::Cancelled),
            )
        };
        for replay_history in [true, false] {
            let loading = harness.reopen(replay_history, cx);
            let (_, responder) = bounded(harness.resume_requests.recv(), cx)
                .await
                .expect("resume");
            harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
            announce_running_child();
            // A subagent whose replayed history ends with an outcome.
            harness.send_update(
                "saved",
                acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("finished")),
            );
            for state in [
                acp::StateUpdate::Running(acp::RunningStateUpdate::new()),
                cancelled(),
            ] {
                harness.send_update("finished", acp::SessionUpdate::StateUpdate(state));
            }
            responder
                .respond(acp::ResumeSessionResponse::new())
                .expect("resumed");
            let parent = bounded(loading, cx).await.expect("resumed parent");
            let (child, finished) = parent.read_with(cx, |parent, _| {
                (
                    parent.subagent(&"child".into()).cloned(),
                    parent.subagent(&"finished".into()).cloned(),
                )
            });
            let child = child.expect("the subagent was announced before the response");
            let finished = finished.expect("the subagent was announced before the response");
            finished.read_with(cx, |finished, _| {
                if replay_history {
                    assert_eq!(
                        finished.foreground_state(),
                        None,
                        "a replayed outcome is history too"
                    );
                } else {
                    assert_eq!(finished.foreground_state(), Some(&cancelled()));
                }
            });
            child.read_with(cx, |child, _| {
                let association = child.subagent_association().expect("subagent");
                assert_eq!(association.title.as_deref(), Some("Investigator"));
                if replay_history {
                    assert!(!association.can_cancel, "a replayed grant is history");
                    assert_eq!(
                        child.foreground_activity(),
                        acp_thread::ForegroundActivity::Unconfirmed,
                        "replayed activity is history"
                    );
                } else {
                    // Without replay, updates sent before the response are current.
                    assert!(association.can_cancel);
                    assert_eq!(
                        child.foreground_activity(),
                        acp_thread::ForegroundActivity::Running
                    );
                }
            });

            if replay_history {
                // The agent reports current states after the response.
                announce_running_child();
                harness.send_update("finished", acp::SessionUpdate::StateUpdate(cancelled()));
                harness.drain_updates(cx).await;
                child.read_with(cx, |child, _| {
                    assert!(child.subagent_association().expect("subagent").can_cancel);
                    assert_eq!(
                        child.foreground_activity(),
                        acp_thread::ForegroundActivity::Running
                    );
                });
                finished.read_with(cx, |finished, _| {
                    assert_eq!(finished.foreground_state(), Some(&cancelled()));
                });
            }
            drop((parent, child, finished));
            cx.run_until_parked();
        }
    }

    #[gpui::test]
    async fn v2_subagent_updates_route_and_patch_child_sessions(cx: &mut TestAppContext) {
        use acp_thread::{ThreadStatus, subagents::SubagentAssociation};
        use agent_client_protocol::schema::MaybeUndefined;

        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        assert!(harness.client.announces_subagents());
        let parent = harness.thread(cx).await;
        harness.send_update(
            "session",
            acp::SessionUpdate::SubagentUpdate(
                acp::SubagentUpdate::new("child")
                    .title("Investigator")
                    .capabilities(
                        acp::SubagentSessionCapabilities::new()
                            .cancel(acp::SessionCancelCapabilities::new()),
                    )
                    .state(acp::StateUpdate::Running(acp::RunningStateUpdate::new())),
            ),
        );
        harness.send_update(
            "child",
            acp::SessionUpdate::AgentMessage(
                acp::AgentMessage::new("child-message").content(vec!["Looking".into()]),
            ),
        );
        harness.drain_updates(cx).await;

        let child = parent
            .read_with(cx, |parent, _| parent.subagent(&"child".into()).cloned())
            .expect("the parent owns the announced subagent");
        child.read_with(cx, |child, _| {
            assert_eq!(child.parent_session_id(), Some(&"session".into()));
            assert_eq!(
                child.subagent_association(),
                Some(&SubagentAssociation {
                    title: Some("Investigator".into()),
                    description: None,
                    can_cancel: true,
                })
            );
            assert_eq!(child.entries().len(), 1, "child traffic reaches the child");
            assert_eq!(
                child.status(),
                ThreadStatus::Unconfirmed,
                "the parent's mirrored state does not report the child's activity"
            );
        });

        harness.send_update(
            "child",
            acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                acp::RunningStateUpdate::new(),
            )),
        );
        harness.send_update(
            "session",
            acp::SessionUpdate::SubagentUpdate(
                acp::SubagentUpdate::new("child").title(MaybeUndefined::Null),
            ),
        );
        // Neither reparenting nor self-reference creates an association.
        harness.send_update(
            "child",
            acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("session")),
        );
        harness.send_update(
            "session",
            acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("session")),
        );
        let _permission = harness
            .agent
            .send_request(acp::RequestPermissionRequest::new(
                "child",
                "Run tests",
                vec![acp::PermissionOption::new(
                    "allow",
                    "Allow",
                    acp::PermissionOptionKind::AllowOnce,
                )],
            ));
        harness.drain_updates(cx).await;

        child.read_with(cx, |child, _| {
            assert_eq!(child.status(), ThreadStatus::Generating);
            assert_eq!(
                child.subagent_association(),
                Some(&SubagentAssociation {
                    title: None,
                    description: None,
                    can_cancel: true,
                })
            );
            assert_eq!(child.subagents().count(), 0);
            assert_eq!(
                child.pending_permission_requests().count(),
                1,
                "child requests route to the child"
            );
        });
        parent.read_with(cx, |parent, _| {
            assert_eq!(parent.parent_session_id(), None);
            assert_eq!(parent.subagents().collect::<Vec<_>>(), [&child]);
        });
        let error = bounded(
            cx.update(|cx| harness.client.clone().close_session(&"child".into(), cx)),
            cx,
        )
        .await
        .expect_err("subagents cannot be closed");
        assert!(
            error
                .to_string()
                .contains("ACP subagent sessions are closed with their parent")
        );
        harness.send_update(
            "child",
            acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate::new(vec![
                acp::SessionConfigOption::boolean("fast", "Fast", false),
            ])),
        );
        harness.drain_updates(cx).await;
        assert!(
            cx.read(|cx| harness.client.session_config_options(&"child".into(), cx))
                .expect("a subagent's configuration is shown")
                .is_read_only(),
            "a subagent's configuration cannot be changed"
        );
        for replay_history in [true, false] {
            let reopened = cx.update(|cx| {
                let client = harness.client.clone();
                let project = harness.project.clone();
                let directories = PathList::new(&[session_path("a")]);
                if replay_history {
                    client.load_session("child".into(), project, directories, None, cx)
                } else {
                    client.resume_session("child".into(), project, directories, None, cx)
                }
            });
            let error = bounded(reopened, cx)
                .await
                .expect_err("subagents cannot be loaded or resumed");
            assert!(
                error
                    .to_string()
                    .contains("ACP subagent sessions are loaded with their parent")
            );
        }
        assert!(harness.resume_requests.is_empty());

        drop((parent, child));
        cx.run_until_parked();
        cx.update(|_| ());
        assert_eq!(
            harness
                .client
                .sessions
                .borrow()
                .keys()
                .cloned()
                .collect::<Vec<_>>(),
            Vec::<acp::SessionId>::new(),
            "the subagent is released with its parent"
        );
        assert!(
            harness.closes.is_empty(),
            "releasing a subagent never closes it"
        );
    }

    #[gpui::test]
    async fn v2_new_session_applies_defaults_before_delivery(cx: &mut TestAppContext) {
        use settings::AgentConfigOptionValue::{Boolean, ValueId};

        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        set_startup_defaults(
            HashMap::from_iter([
                ("enabled".into(), Boolean(true)),
                ("model".into(), ValueId("large".into())),
                ("already".into(), Boolean(false)),
                ("wrong-type".into(), ValueId("invalid".into())),
                ("future".into(), Boolean(true)),
                ("missing".into(), Boolean(true)),
            ]),
            cx,
        );
        let mut offered = vec![
            acp::SessionConfigOption::select(
                "model",
                "Model",
                "small",
                vec![acp::SessionConfigSelectOption::new("small", "Small")],
            ),
            acp::SessionConfigOption::boolean("enabled", "Enabled", false),
            acp::SessionConfigOption::boolean("already", "Already", false),
            acp::SessionConfigOption::boolean("wrong-type", "Wrong type", false),
            acp::SessionConfigOption::new(
                "future",
                "Future",
                acp::SessionConfigKind::Other(acp::OtherSessionConfigKind::new(
                    "_future",
                    Default::default(),
                )),
            ),
        ];
        let mut creation = harness.create(cx);
        harness.new_request(&mut creation, cx).await;
        harness
            .new_responses
            .try_send(Ok(initial_response().config_options(offered.clone())))
            .expect("new response");
        harness.drain_updates(cx).await;
        assert!(
            (&mut creation).now_or_never().is_none(),
            "defaults finish before delivery"
        );
        let request = bounded(harness.config_requests.recv(), cx)
            .await
            .expect("default request");
        assert_eq!(request.config_id, acp::SessionConfigId::new("enabled"));
        assert_eq!(request.value, acp::SessionConfigOptionValue::boolean(true));
        offered[0] = acp::SessionConfigOption::select(
            "model",
            "Model",
            "small",
            vec![
                acp::SessionConfigSelectOption::new("small", "Small"),
                acp::SessionConfigSelectOption::new("large", "Large"),
            ],
        );
        harness
            .config_responses
            .try_send(Ok(acp::SetSessionConfigOptionResponse::new(
                offered.clone(),
            )))
            .expect("normalized response exposes dependent model");
        let request = bounded(harness.config_requests.recv(), cx)
            .await
            .expect("dependent default");
        assert_eq!(request.config_id, acp::SessionConfigId::new("model"));
        assert_eq!(request.value, acp::SessionConfigOptionValue::id("large"));
        if let acp::SessionConfigKind::Select(model) = &mut offered[0].kind {
            model.current_value = "large".into();
        }
        harness
            .config_responses
            .try_send(Ok(acp::SetSessionConfigOptionResponse::new(
                offered.clone(),
            )))
            .expect("model response");
        let thread = bounded(creation, cx).await.expect("initialized");
        let options = cx
            .read(|cx| {
                harness
                    .client
                    .session_config_options(thread.read(cx).session_id(), cx)
            })
            .expect("config");
        assert_eq!(options.config_options(), offered);
        assert!(
            harness.config_requests.is_empty(),
            "do not retry normalized or invalid defaults"
        );

        for replay in [false, true] {
            let loading = harness.reopen(replay, cx);
            let (_, response) = bounded(harness.resume_requests.recv(), cx)
                .await
                .expect("resume");
            harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
            response
                .respond(
                    acp::ResumeSessionResponse::new()
                        .config_options(initial_response().config_options),
                )
                .expect("agent's saved settings");
            let resumed = bounded(loading, cx).await.expect("resumed");
            let settings = cx
                .read(|cx| {
                    harness
                        .client
                        .session_config_options(resumed.read(cx).session_id(), cx)
                })
                .expect("saved config");
            assert_eq!(settings.config_options(), initial_response().config_options);
            assert!(
                harness.config_requests.is_empty(),
                "resume/load must not apply startup defaults"
            );
            drop(resumed);
            cx.run_until_parked();
        }
    }

    #[gpui::test]
    async fn v2_new_session_defaults_handle_interrupted_startup(cx: &mut TestAppContext) {
        for (abandoned, disconnected) in [(false, false), (true, false), (false, true)] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            *harness.hold_config.lock().expect("config mode") = abandoned || disconnected;
            set_startup_defaults(
                HashMap::from_iter([(
                    "enabled".into(),
                    settings::AgentConfigOptionValue::Boolean(true),
                )]),
                cx,
            );
            let mut creation = harness.create(cx);
            harness.new_request(&mut creation, cx).await;
            harness
                .new_responses
                .try_send(Ok(initial_response()))
                .expect("new response");
            bounded(harness.config_requests.recv(), cx)
                .await
                .expect("startup default");
            if disconnected {
                let _response = bounded(harness.config_replies.recv(), cx)
                    .await
                    .expect("held config response");
                drop(harness._agent_task);
                drop(harness.agent);
                assert!(
                    bounded(creation, cx).await.is_err(),
                    "transport loss must not publish a usable session"
                );
                continue;
            }
            if abandoned {
                let response = bounded(harness.config_replies.recv(), cx)
                    .await
                    .expect("held config response");
                drop(creation);
                cx.run_until_parked();
                assert!(
                    harness.client.sessions.borrow().is_empty(),
                    "abandonment releases the provisional owner"
                );
                assert_eq!(
                    bounded(harness.closes.recv(), cx)
                        .await
                        .expect("undelivered session closed"),
                    acp::SessionId::new("session")
                );
                assert!(harness.client.sessions.borrow().is_empty());
                response
                    .respond(acp::SetSessionConfigOptionResponse::new(
                        initial_response().config_options,
                    ))
                    .expect("late config reply");
                harness.drain_updates(cx).await;
                assert!(harness.client.sessions.borrow().is_empty());
            } else {
                harness
                    .config_responses
                    .try_send(Err(
                        agent_client_protocol::Error::invalid_params().data("default rejected")
                    ))
                    .expect("rejection");
                let thread = bounded(creation, cx)
                    .await
                    .expect("rejected default keeps new session usable");
                assert!(harness.closes.is_empty());
                assert!(
                    harness.config_requests.is_empty(),
                    "do not retry a rejected default"
                );
                thread.read_with(cx, |thread, _| {
                    assert_eq!(
                        thread.foreground_activity(),
                        acp_thread::ForegroundActivity::Idle
                    );
                    assert_eq!(
                        thread.notices().len(),
                        1,
                        "rejected default is visible to the user"
                    );
                });
            }
        }
    }

    #[gpui::test]
    async fn v2_cancel_sdk_preserves_receipt_and_waits_for_reported_idle(cx: &mut TestAppContext) {
        use acp_thread::{ForegroundActivity, SubmissionResponse, SubmissionState};
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                session_id.clone(),
                acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                    acp::RunningStateUpdate::new(),
                )),
            ))
            .expect("reported running");
        cx.run_until_parked();
        let submission = thread.update(cx, |thread, cx| thread.send(vec!["question".into()], cx));
        let id = submission.id;
        let (_, responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("pending prompt");
        let request_cancellation = responder.cancellation();
        let mut cancellation = thread.update(cx, |thread, cx| thread.cancel(cx));
        let notification = bounded(harness.cancellations.recv(), cx)
            .await
            .expect("session cancel");
        assert_eq!(notification.session_id, session_id);
        assert!(
            !request_cancellation.is_cancelled(),
            "session cancel is not RPC withdrawal"
        );
        responder
            .respond(acp::PromptResponse::new("accepted"))
            .expect("accepted during cancellation");
        assert!(matches!(bounded(submission, cx).await.expect("acceptance"),
            Some(SubmissionResponse::Accepted(receipt)) if receipt.message_id == "accepted".into()));
        assert!((&mut cancellation).now_or_never().is_none());
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                session_id,
                acp::SessionUpdate::StateUpdate(acp::StateUpdate::Idle(
                    acp::IdleStateUpdate::new().stop_reason(acp::StopReason::Cancelled),
                )),
            ))
            .expect("confirmed idle");
        bounded(cancellation, cx)
            .await
            .expect("cancellation succeeds");
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert!(matches!(
                thread.submission(id).expect("receipt retained").state,
                SubmissionState::Accepted { echoed: false, .. }
            ));
        });
    }

    #[gpui::test]
    async fn v2_cancel_sdk_eof_fails_observation_without_inventing_idle(cx: &mut TestAppContext) {
        use acp_thread::ForegroundActivity;
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                session_id,
                acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                    acp::RunningStateUpdate::new(),
                )),
            ))
            .expect("reported running");
        cx.run_until_parked();
        let mut cancellation = thread.update(cx, |thread, cx| thread.cancel(cx));
        bounded(harness.cancellations.recv(), cx)
            .await
            .expect("session cancel");
        assert!((&mut cancellation).now_or_never().is_none());
        drop(harness._agent_task);
        drop(harness.agent);
        bounded(cancellation, cx)
            .await
            .expect_err("EOF cannot confirm cancellation");
        bounded(thread.update(cx, |thread, cx| thread.cancel(cx)), cx)
            .await
            .expect_err("new observers must not wait on a disconnected session");
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
    }

    #[gpui::test]
    async fn v2_receipt_sdk_correlates_echoes_without_completing_activity(cx: &mut TestAppContext) {
        use acp_thread::{
            AgentThreadEntry, ForegroundActivity, SubmissionResponse, SubmissionState,
        };
        for echo_first in [true, false] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            let thread = harness.thread(cx).await;
            assert!(thread.read_with(cx, |thread, _| thread.uses_reported_activity()));
            let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
            harness
                .agent
                .send_notification(acp::UpdateSessionNotification::new(
                    session_id.clone(),
                    acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                        acp::RunningStateUpdate::new(),
                    )),
                ))
                .expect("autonomous running");
            cx.run_until_parked();
            assert_eq!(
                thread.read_with(cx, |thread, _| thread.foreground_activity()),
                ForegroundActivity::Running
            );
            let content = vec![acp::ContentBlock::from("question")];
            let submission = thread.update(cx, |thread, cx| thread.send(content.clone(), cx));
            let submission_id = submission.id;
            let (request, responder) = bounded(harness.prompts.recv(), cx).await.expect("prompt");
            assert_eq!(request.prompt, content);
            assert_eq!(request.session_id, session_id);
            let echo = acp::UpdateSessionNotification::new(
                session_id.clone(),
                acp::SessionUpdate::UserMessage(
                    acp::UserMessage::new("message").content(content.clone()),
                ),
            );
            if echo_first {
                harness
                    .agent
                    .send_notification(echo.clone())
                    .expect("early echo");
            }
            let receipt = acp::PromptResponse::new("message").meta(meta("receipt"));
            responder.respond(receipt.clone()).expect("acceptance");
            assert!(matches!(bounded(submission, cx).await.expect("receipt"),
                Some(SubmissionResponse::Accepted(accepted)) if accepted == receipt));
            if !echo_first {
                thread.read_with(cx, |thread, _| {
                    assert!(matches!(
                        thread.submission(submission_id).expect("un-echoed").state,
                        SubmissionState::Accepted { echoed: false, .. }
                    ));
                });
                harness.agent.send_notification(echo).expect("late echo");
            }
            cx.run_until_parked();
            thread.read_with(cx, |thread, _| {
                let [AgentThreadEntry::UserMessage(message)] = thread.entries() else {
                    panic!("one authoritative user message");
                };
                assert_eq!(message.content.source_blocks(), content.as_slice());
                assert!(matches!(
                    thread.submission(submission_id).expect("echoed").state,
                    SubmissionState::Accepted { echoed: true, .. }
                ));
                assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
            });
            for state in [
                acp::StateUpdate::RequiresAction(acp::RequiresActionStateUpdate::new()),
                acp::StateUpdate::Idle(
                    acp::IdleStateUpdate::new().stop_reason(acp::StopReason::EndTurn),
                ),
            ] {
                harness
                    .agent
                    .send_notification(acp::UpdateSessionNotification::new(
                        session_id.clone(),
                        acp::SessionUpdate::StateUpdate(state),
                    ))
                    .expect("reported state with no prompt RPC");
                cx.run_until_parked();
            }
            assert_eq!(
                thread.read_with(cx, |thread, _| thread.foreground_activity()),
                ForegroundActivity::Idle
            );
        }
    }

    #[gpui::test]
    async fn v2_receipt_sdk_holds_later_traffic_until_model_application(cx: &mut TestAppContext) {
        use acp_thread::SubmissionState;
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let content = vec![acp::ContentBlock::from("question")];
        let (response, applied) = AcpV2Connection::ordered_response(
            &harness.client.connection,
            &harness.client.dispatch_tx,
            acp::PromptRequest::new(session_id.clone(), content.clone()),
            || Ok(()),
            |_| None,
        );
        let (delivered, received) = futures::channel::oneshot::channel();
        let (release, held) = futures::channel::oneshot::channel();
        let request = ReceiptSubmission::ordered(
            async move {
                let response = response.await;
                delivered.send(()).expect("receipt delivery");
                held.await.expect("application gate");
                response
            }
            .boxed_local(),
            applied,
        );
        let submission = thread.update(cx, |thread, cx| {
            thread.submit_receipt(content.clone(), request, cx)
        });
        let id = submission.id;
        drop(submission);
        let (_, responder) = bounded(harness.prompts.recv(), cx).await.expect("prompt");
        responder
            .respond(acp::PromptResponse::new("message"))
            .expect("acceptance");
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                session_id,
                acp::SessionUpdate::UserMessage(acp::UserMessage::new("message").content(content)),
            ))
            .expect("following echo");
        bounded(received, cx)
            .await
            .expect("transport received receipt");
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert!(
                thread.entries().is_empty(),
                "later traffic must wait for model application"
            );
            assert!(matches!(
                thread.submission(id).expect("pending").state,
                SubmissionState::Pending
            ));
        });
        release.send(()).expect("apply receipt");
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries().len(), 1);
            assert!(matches!(
                thread.submission(id).expect("echoed").state,
                SubmissionState::Accepted { echoed: true, .. }
            ));
        });
    }

    #[gpui::test]
    async fn v2_receipt_sdk_distinguishes_preflight_rejection_and_transport_loss(
        cx: &mut TestAppContext,
    ) {
        use acp_thread::{SubmissionDisconnected, SubmissionState};
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        let unsupported: acp::ContentBlock = serde_json::from_value(json!({
            "type": "image", "data": "AA==", "mimeType": "image/png"
        }))
        .expect("image content");
        let submission = thread.update(cx, |thread, cx| thread.send(vec![unsupported], cx));
        assert!(bounded(submission, cx).await.is_err());
        assert!(harness.prompts.is_empty(), "preflight must not publish");
        let rejected = thread.update(cx, |thread, cx| thread.send(vec!["rejected".into()], cx));
        let rejected_id = rejected.id;
        let (_, responder) = bounded(harness.prompts.recv(), cx).await.expect("prompt");
        let rejection =
            agent_client_protocol::Error::invalid_params().data(json!({"reason": "not inserted"}));
        responder
            .respond_with_error(rejection.clone())
            .expect("reject");
        let error = bounded(rejected, cx).await.expect_err("wire rejection");
        assert_eq!(
            error.downcast_ref::<agent_client_protocol::Error>(),
            Some(&rejection)
        );
        let pending = thread.update(cx, |thread, cx| thread.send(vec!["unknown".into()], cx));
        let pending_id = pending.id;
        let (_, _responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("pending prompt");
        drop(harness._agent_task);
        drop(harness.agent);
        let error = bounded(pending, cx).await.expect_err("EOF");
        assert!(
            error.is::<SubmissionDisconnected>()
                || error
                    .downcast_ref::<agent_client_protocol::Error>()
                    .is_some_and(agent_client_protocol::is_incoming_transport_closed)
        );
        bounded(harness.client.connection.incoming_closed(), cx).await;
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.submission(rejected_id).expect("rejection").state,
                SubmissionState::Failed(_)
            ));
            assert!(matches!(
                thread.submission(pending_id).expect("uncertain").state,
                SubmissionState::Disconnected { receipt: None }
            ));
            assert!(!thread.has_unsettled_submissions());
        });
        assert!(harness.prompts.is_empty(), "loss must not resend");
    }

    #[gpui::test]
    async fn v2_receipt_sdk_rechecks_owner_before_publication(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        harness.client.sessions.borrow_mut().clear();
        let replacement = harness.thread(cx).await;
        let stale = thread.update(cx, |thread, cx| thread.send(vec!["stale owner".into()], cx));
        cx.run_until_parked();
        assert!(
            harness.prompts.is_empty(),
            "a stale owner cannot submit through its replacement"
        );
        assert!(bounded(stale, cx).await.is_err());

        let submission =
            replacement.update(cx, |thread, cx| thread.send(vec!["not sent".into()], cx));
        harness.client.sessions.borrow_mut().clear();
        assert!(bounded(submission, cx).await.is_err());
        assert!(harness.prompts.is_empty());
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_receipt_stdio_preserves_acceptance_before_immediate_eof(cx: &mut TestAppContext) {
        use acp_thread::{SubmissionResponse, SubmissionState};
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let client = bounded(AcpV2Connection::stdio(
            AgentId::new("receipt-stdio"), project.clone(),
            AgentServerCommand {
                path: "/bin/sh".into(),
                args: vec!["-c".into(), r#"
while IFS= read -r request; do
    id=${request#*\"id\":}
    id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":2,"info":{"name":"receipt-agent","version":"1"},"capabilities":{"session":{}}}}\n' "$id"
            ;;
        *'"method":"session/new"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"stdio-session"}}\n' "$id"
            ;;
        *'"method":"session/prompt"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"messageId":"accepted","_meta":{"scope":"receipt"}}}\n' "$id"
            exit 0
            ;;
        *) exit 43 ;;
    esac
done
"#.into()],
                env: None,
            },
            WeakEntity::new_invalid(), &mut cx.to_async(),
        ), cx).await.expect("stdio connection");
        let client = Rc::new(client);
        let creation = cx.update(|cx| {
            client
                .clone()
                .new_session(project, PathList::new(&[directory.path()]), cx)
        });
        let thread = bounded(creation, cx).await.expect("stdio owner");
        let submission = thread.update(cx, |thread, cx| thread.send(vec!["question".into()], cx));
        let id = submission.id;
        let expected = acp::PromptResponse::new("accepted").meta(meta("receipt"));
        assert!(
            matches!(bounded(submission, cx).await.expect("acceptance before EOF"),
            Some(SubmissionResponse::Accepted(receipt)) if receipt == expected)
        );
        bounded(client.connection.incoming_closed(), cx).await;
        bounded(drain_foreground_queue(&client.dispatch_tx), cx)
            .await
            .expect("disconnect barrier");
        thread.read_with(cx, |thread, _| {
            assert!(
                matches!(&thread.submission(id).expect("retained acceptance").state,
                SubmissionState::Disconnected { receipt: Some(receipt) } if receipt == &expected)
            );
            assert!(!thread.has_unsettled_submissions());
        });
    }

    #[test]
    fn v2_receipt_content_extensions_require_capabilities() {
        for (block, capabilities) in [
            (
                json!({"type":"image","data":"AA==","mimeType":"image/png"}),
                acp::PromptCapabilities::new().image(acp::PromptImageCapabilities::new()),
            ),
            (
                json!({"type":"audio","data":"AA==","mimeType":"audio/wav"}),
                acp::PromptCapabilities::new().audio(acp::PromptAudioCapabilities::new()),
            ),
            (
                json!({"type":"resource","resource":{"uri":"file:///context","text":"context"}}),
                acp::PromptCapabilities::new()
                    .embedded_context(acp::PromptEmbeddedContextCapabilities::new()),
            ),
        ] {
            let block = serde_json::from_value(block).expect("prompt block");
            assert!(
                validate_prompt_content(
                    &acp::PromptCapabilities::default(),
                    std::slice::from_ref(&block)
                )
                .is_err()
            );
            assert!(validate_prompt_content(&capabilities, &[block]).is_ok());
        }
    }

    #[gpui::test]
    async fn v2_list_sdk_preserves_filters_pages_and_session_information(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let list = cx
            .update(|cx| harness.client.session_list(cx))
            .expect("session discovery");
        assert!(!list.supports_delete(), "deletion was not advertised");
        assert!(
            cx.update(|cx| list.watch(cx)).is_none(),
            "discovery is snapshot-only"
        );
        let pending = cx.update(|cx| {
            list.list_sessions(
                AgentSessionListRequest {
                    cwd: Some(session_path("b")),
                    meta: Some(meta("request")),
                    ..Default::default()
                },
                cx,
            )
        });
        let (request, responder) = bounded(harness.list_requests.recv(), cx)
            .await
            .expect("first page");
        assert_eq!(
            request,
            acp::ListSessionsRequest::new()
                .cwd(Some(acp::AbsolutePath::new(session_path("b"))))
                .meta(meta("request"))
        );
        let mut information_metadata = meta("information");
        information_metadata.insert("literal".into(), serde_json::Value::Null);
        responder
            .respond(
                acp::ListSessionsResponse::new(vec![
                    acp::SessionInfo::new("listed", session_path("b"))
                        .additional_directories(vec![
                            session_path("a"),
                            session_path("b"),
                            session_path("a"),
                        ])
                        .title("Saved title")
                        .updated_at("2026-10-01T10:20:30+02:00")
                        .meta(information_metadata.clone()),
                ])
                .next_cursor("opaque/+?&")
                .meta(meta("page")),
            )
            .expect("first page response");
        let page = bounded(pending, cx).await.expect("mapped page");
        let [information] = page.sessions.as_slice() else {
            panic!("one listed session")
        };
        assert_eq!(information.session_id, acp::SessionId::new("listed"));
        assert_eq!(
            information
                .work_dirs
                .as_ref()
                .expect("directories")
                .ordered_paths()
                .cloned()
                .collect::<Vec<_>>(),
            [session_path("b"), session_path("a")]
        );
        assert_eq!(information.title.as_deref(), Some("Saved title"));
        assert_eq!(
            information.updated_at.expect("timestamp").to_rfc3339(),
            "2026-10-01T08:20:30+00:00"
        );
        assert!(information.created_at.is_none());
        assert_eq!(information.meta, Some(information_metadata));
        assert_eq!(page.meta, Some(meta("page")));
        assert_eq!(page.next_cursor.as_deref(), Some("opaque/+?&"));

        let pending = cx.update(|cx| {
            list.list_sessions(
                AgentSessionListRequest {
                    cwd: Some(session_path("b")),
                    cursor: page.next_cursor,
                    ..Default::default()
                },
                cx,
            )
        });
        let (request, responder) = bounded(harness.list_requests.recv(), cx)
            .await
            .expect("next page");
        assert_eq!(
            request,
            acp::ListSessionsRequest::new()
                .cwd(Some(acp::AbsolutePath::new(session_path("b"))))
                .cursor("opaque/+?&")
        );
        responder
            .respond(acp::ListSessionsResponse::new(vec![]))
            .expect("last page");
        let page = bounded(pending, cx).await.expect("last page");
        assert!(page.sessions.is_empty());
        assert!(page.next_cursor.is_none());
        assert!(
            harness.client.sessions.borrow().is_empty(),
            "listing must not bind sessions"
        );
        assert!(harness.new_requests.is_empty());
        assert!(harness.closes.is_empty());
    }

    #[gpui::test]
    async fn v2_list_sdk_preserves_errors_and_scopes_cancellation(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let list = cx
            .update(|cx| harness.client.session_list(cx))
            .expect("session discovery");
        let unpublished =
            cx.update(|cx| list.list_sessions(AgentSessionListRequest::default(), cx));
        drop(unpublished);
        cx.run_until_parked();
        assert!(
            harness.list_requests.is_empty(),
            "an unpolled observer must not publish"
        );
        let failed = cx.update(|cx| list.list_sessions(AgentSessionListRequest::default(), cx));
        let (request, responder) = bounded(harness.list_requests.recv(), cx)
            .await
            .expect("list request");
        assert_eq!(request, acp::ListSessionsRequest::new());
        let rejection = agent_client_protocol::Error::new(-32042, "List unavailable")
            .data(json!({"retain": [null, 7]}));
        responder
            .respond_with_error(rejection.clone())
            .expect("list rejection");
        assert_eq!(
            bounded(failed, cx)
                .await
                .expect_err("agent rejection")
                .downcast_ref::<agent_client_protocol::Error>(),
            Some(&rejection)
        );

        let dropped = cx.update(|cx| list.list_sessions(AgentSessionListRequest::default(), cx));
        let (_, responder) = bounded(harness.list_requests.recv(), cx)
            .await
            .expect("dropped request");
        let dropped_cancellation = responder.cancellation();
        let pending = cx.update(|cx| list.list_sessions(AgentSessionListRequest::default(), cx));
        let (_, retained) = bounded(harness.list_requests.recv(), cx)
            .await
            .expect("retained request");
        let retained_cancellation = retained.cancellation();
        drop(dropped);
        bounded(dropped_cancellation.cancelled(), cx).await;
        assert!(!retained_cancellation.is_cancelled());
        drop(harness._agent_task);
        drop(harness.agent);
        let error = bounded(pending, cx)
            .await
            .expect_err("EOF settles pending list");
        assert!(
            error
                .downcast_ref::<agent_client_protocol::Error>()
                .is_some_and(agent_client_protocol::is_incoming_transport_closed)
        );
        assert!(harness.new_requests.is_empty());
        assert!(harness.closes.is_empty());
    }

    #[gpui::test]
    async fn v2_delete_sdk_requires_advertised_capability(cx: &mut TestAppContext) {
        for capabilities in [json!({"session": {}}), json!({"session": {"delete": null}})] {
            let harness = SessionHarness::new(
                serde_json::from_value(capabilities).expect("capabilities"),
                vec![],
                vec![],
                cx,
            )
            .await;
            let list = cx
                .update(|cx| harness.client.session_list(cx))
                .expect("session discovery");
            assert!(!list.supports_delete());
            let error = bounded(cx.update(|cx| list.delete_session(&"saved".into(), cx)), cx)
                .await
                .expect_err("unsupported deletion");
            assert!(
                error
                    .to_string()
                    .contains("Deleting sessions is not supported")
            );
            cx.run_until_parked();
            assert!(harness.delete_requests.is_empty());
            assert!(harness.list_requests.is_empty());
            assert!(harness.closes.is_empty());
        }
    }

    #[gpui::test]
    async fn v2_delete_sdk_preserves_ids_without_closing_active_sessions(cx: &mut TestAppContext) {
        let capabilities = acp::AgentCapabilities::new()
            .session(acp::SessionCapabilities::new().delete(acp::SessionDeleteCapabilities::new()));
        let harness = SessionHarness::new(capabilities, vec![], vec![], cx).await;
        let list = cx
            .update(|cx| harness.client.session_list(cx))
            .expect("session discovery");
        assert!(list.supports_delete());
        let thread = harness.thread(cx).await;
        let active_id = cx.read(|cx| thread.read(cx).session_id().clone());
        for session_id in [
            acp::SessionId::new(" opaque/会話:007?x=%2F \n"),
            active_id.clone(),
        ] {
            let pending = cx.update(|cx| list.delete_session(&session_id, cx));
            let (request, responder) = bounded(harness.delete_requests.recv(), cx)
                .await
                .expect("delete request");
            assert_eq!(request, acp::DeleteSessionRequest::new(session_id));
            responder
                .respond(acp::DeleteSessionResponse::new().meta(meta("deleted")))
                .expect("delete response");
            bounded(pending, cx).await.expect("delete succeeds");
            assert!(
                harness
                    .client
                    .session_owners
                    .borrow()
                    .contains_key(&active_id)
            );
            assert!(
                harness
                    .client
                    .sessions
                    .borrow()
                    .get(&active_id)
                    .is_some_and(|session| session.thread.entity_id() == thread.entity_id())
            );
        }
        assert!(
            harness.closes.is_empty(),
            "delete must not implicitly close"
        );
        assert!(harness.cancellations.is_empty());
        assert!(harness.resume_requests.is_empty());
        assert!(harness.new_requests.is_empty());
        assert!(harness.list_requests.is_empty());
        assert!(
            bounded(cx.update(|cx| list.delete_sessions(cx)), cx)
                .await
                .is_err()
        );
        assert!(
            harness.delete_requests.is_empty(),
            "there is no bulk delete RPC"
        );
    }

    #[gpui::test]
    async fn v2_delete_sdk_preserves_errors_and_scopes_cancellation(cx: &mut TestAppContext) {
        let capabilities = acp::AgentCapabilities::new()
            .session(acp::SessionCapabilities::new().delete(acp::SessionDeleteCapabilities::new()));
        let harness = SessionHarness::new(capabilities, vec![], vec![], cx).await;
        let list = cx
            .update(|cx| harness.client.session_list(cx))
            .expect("session discovery");
        let unpublished = cx.update(|cx| list.delete_session(&"unpublished".into(), cx));
        drop(unpublished);
        cx.run_until_parked();
        assert!(
            harness.delete_requests.is_empty(),
            "an unpolled task must not publish"
        );
        let failed = cx.update(|cx| list.delete_session(&"rejected".into(), cx));
        let (_, responder) = bounded(harness.delete_requests.recv(), cx)
            .await
            .expect("rejected delete");
        let rejection = agent_client_protocol::Error::new(-32042, "Deletion refused")
            .data(json!({"retain": [null, 7]}));
        responder
            .respond_with_error(rejection.clone())
            .expect("rejection");
        assert_eq!(
            bounded(failed, cx)
                .await
                .expect_err("server rejection")
                .downcast_ref::<agent_client_protocol::Error>(),
            Some(&rejection)
        );
        let dropped = cx.update(|cx| list.delete_session(&"cancelled".into(), cx));
        let (_, responder) = bounded(harness.delete_requests.recv(), cx)
            .await
            .expect("cancellable delete");
        let cancellation = responder.cancellation();
        let pending = cx.update(|cx| list.delete_session(&"retained".into(), cx));
        let (_, retained) = bounded(harness.delete_requests.recv(), cx)
            .await
            .expect("retained delete");
        let retained_cancellation = retained.cancellation();
        drop(dropped);
        bounded(cancellation.cancelled(), cx).await;
        assert!(!retained_cancellation.is_cancelled());
        assert!(
            harness.cancellations.is_empty(),
            "RPC cancellation is not session cancellation"
        );
        drop(harness._agent_task);
        drop(harness.agent);
        let error = bounded(pending, cx).await.expect_err("EOF settles delete");
        assert!(
            error
                .downcast_ref::<agent_client_protocol::Error>()
                .is_some_and(agent_client_protocol::is_incoming_transport_closed)
        );
        assert!(
            harness.delete_requests.is_empty(),
            "failed deletes must not retry"
        );
        assert!(harness.closes.is_empty());
    }

    #[gpui::test]
    async fn v2_new_session_keeps_subagent_traffic_sent_before_its_response(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        *harness.hold_new_responses.lock().expect("new mode") = true;
        let creation = harness.create(cx);
        let (_, response) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("new request");
        let message = |id: &str, text: &str| {
            acp::SessionUpdate::AgentMessage(acp::AgentMessage::new(id).content(vec![text.into()]))
        };
        harness.send_update(
            "session",
            acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("child")),
        );
        harness.send_update("child", message("child-message", "child works"));
        harness.send_update(
            "child",
            acp::SessionUpdate::SubagentUpdate(acp::SubagentUpdate::new("grandchild")),
        );
        harness.send_update(
            "grandchild",
            message("grandchild-message", "grandchild works"),
        );
        // Traffic for a session nobody announced is still not delivered.
        harness.send_update("stranger", message("stranger-message", "unrelated"));
        cx.run_until_parked();
        response
            .respond(initial_response())
            .expect("new session response");
        let parent = bounded(creation, cx).await.expect("created session");
        cx.run_until_parked();

        let child = parent
            .read_with(cx, |parent, _| parent.subagent(&"child".into()).cloned())
            .expect("announced before the response");
        let grandchild = child
            .read_with(cx, |child, _| child.subagent(&"grandchild".into()).cloned())
            .expect("announced before the response");
        for (thread, text) in [(&child, "child works"), (&grandchild, "grandchild works")] {
            thread.read_with(cx, |thread, cx| {
                assert!(
                    thread.to_markdown(cx).contains(text),
                    "a subagent's traffic sent before the response reaches it"
                );
            });
        }
        parent.read_with(cx, |parent, cx| {
            let transcript = parent.to_markdown(cx);
            assert!(!transcript.contains("child works"));
            assert!(!transcript.contains("unrelated"));
        });
        assert!(
            !harness
                .client
                .sessions
                .borrow()
                .contains_key(&acp::SessionId::new("stranger"))
        );
    }

    #[gpui::test]
    async fn v2_new_sdk_correlates_concurrent_prefixes_by_response_id(cx: &mut TestAppContext) {
        use acp_thread::ForegroundActivity;
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        *harness.hold_new_responses.lock().expect("new mode") = true;
        let filesystem = fs::FakeFs::new(cx.executor());
        filesystem
            .insert_tree(session_path(""), json!({"b": {}}))
            .await;
        let second_project = Project::test(filesystem, [session_path("b").as_path()], cx).await;
        let mut first = harness.create(cx);
        let second = cx.update(|cx| {
            harness.client.clone().new_session(
                second_project.clone(),
                PathList::new(&[session_path("b")]),
                cx,
            )
        });
        let (request, response) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("first request");
        let (other_request, other_response) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("second request");
        let (first_response, second_response) = if request.cwd.0 == session_path("a") {
            assert_eq!(other_request.cwd.0, session_path("b"));
            (response, other_response)
        } else {
            assert_eq!(request.cwd.0, session_path("b"));
            assert_eq!(other_request.cwd.0, session_path("a"));
            (other_response, response)
        };
        harness.send_update(
            "first",
            acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                acp::RunningStateUpdate::new(),
            )),
        );
        harness.send_update(
            "second",
            acp::SessionUpdate::StateUpdate(acp::StateUpdate::RequiresAction(
                acp::RequiresActionStateUpdate::new(),
            )),
        );
        harness.send_update(
            "first",
            acp::SessionUpdate::AgentMessage(
                acp::AgentMessage::new("first-message").content(vec!["first only".into()]),
            ),
        );
        harness.send_update(
            "second",
            acp::SessionUpdate::AgentMessage(
                acp::AgentMessage::new("second-message").content(vec!["second only".into()]),
            ),
        );
        harness.send_update(
            "second",
            acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate::new(vec![
                acp::SessionConfigOption::boolean("prefix", "Prefix", false),
            ])),
        );
        harness.send_update(
            "second",
            acp::SessionUpdate::AvailableCommandsUpdate(acp::AvailableCommandsUpdate::new(vec![
                acp::AvailableCommand::Command(acp::Command::new("prefix", "Prefix")),
            ])),
        );
        cx.run_until_parked();
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts().0,
            2
        );
        harness.drain_updates(cx).await;
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts().1,
            6
        );
        let config = vec![acp::SessionConfigOption::boolean(
            "response", "Response", true,
        )];
        let commands = vec![acp::AvailableCommand::Command(acp::Command::new(
            "response", "Response",
        ))];
        second_response
            .respond(
                acp::NewSessionResponse::new("second")
                    .config_options(config.clone())
                    .available_commands(commands.clone()),
            )
            .expect("second replies first");
        let second_thread = bounded(second, cx).await.expect("second created");
        assert!((&mut first).now_or_never().is_none());
        second_thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.foreground_activity(),
                ForegroundActivity::RequiresAction
            );
            assert!(thread.to_markdown(cx).contains("second only"));
            assert!(!thread.to_markdown(cx).contains("first only"));
            assert_eq!(thread.available_commands(), commands);
        });
        let options = cx
            .read(|cx| harness.client.session_config_options(&"second".into(), cx))
            .expect("second config");
        assert_eq!(options.config_options(), config);
        first_response
            .respond(acp::NewSessionResponse::new("first"))
            .expect("first replies last");
        let first_thread = bounded(first, cx).await.expect("first created");
        first_thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
            assert!(thread.to_markdown(cx).contains("first only"));
            assert!(!thread.to_markdown(cx).contains("second only"));
        });
        let owners = harness.client.session_owners.borrow();
        assert_eq!(
            owners
                .get(&"first".into())
                .expect("first owner")
                .project_entity_id,
            harness.project.entity_id()
        );
        assert_eq!(
            owners
                .get(&"second".into())
                .expect("second owner")
                .project_entity_id,
            second_project.entity_id()
        );
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts(),
            (0, 0, 0)
        );
    }

    #[gpui::test]
    async fn v2_new_sdk_concurrent_failure_preserves_successful_prefix(cx: &mut TestAppContext) {
        for abandoned in [false, true] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            *harness.hold_new_responses.lock().expect("new mode") = true;
            let failed = harness.create(cx);
            let (_, failed_response) = bounded(harness.new_replies.recv(), cx)
                .await
                .expect("failed creation");
            let retained = harness.create(cx);
            let (_, retained_response) = bounded(harness.new_replies.recv(), cx)
                .await
                .expect("retained creation");
            harness.send_update(
                "failed",
                acp::SessionUpdate::AgentMessage(
                    acp::AgentMessage::new("discarded").content(vec!["discarded".into()]),
                ),
            );
            harness.send_update(
                "retained",
                acp::SessionUpdate::AgentMessage(
                    acp::AgentMessage::new("retained").content(vec!["retained".into()]),
                ),
            );
            harness.drain_updates(cx).await;
            if abandoned {
                drop(failed);
                failed_response
                    .respond(acp::NewSessionResponse::new("failed"))
                    .expect("abandoned success");
                assert_eq!(
                    bounded(harness.closes.recv(), cx)
                        .await
                        .expect("close abandoned session"),
                    acp::SessionId::new("failed")
                );
            } else {
                let rejection = agent_client_protocol::Error::new(-32042, "Creation rejected")
                    .data(json!({"keep": [null, 7]}));
                failed_response
                    .respond_with_error(rejection.clone())
                    .expect("reject first");
                assert_eq!(
                    bounded(failed, cx)
                        .await
                        .expect_err("rejection")
                        .downcast_ref::<agent_client_protocol::Error>(),
                    Some(&rejection)
                );
            }
            retained_response
                .respond(acp::NewSessionResponse::new("retained"))
                .expect("successful response");
            let thread = bounded(retained, cx)
                .await
                .expect("success survives unrelated rejection");
            thread.read_with(cx, |thread, cx| {
                assert!(thread.to_markdown(cx).contains("retained"));
                assert!(!thread.to_markdown(cx).contains("discarded"));
            });
            cx.run_until_parked();
            assert_eq!(
                harness.client.pending_session_updates.borrow().counts(),
                (0, 0, 0)
            );
            let later = harness.create(cx);
            let (_, response) = bounded(harness.new_replies.recv(), cx)
                .await
                .expect("later creation");
            response
                .respond(acp::NewSessionResponse::new("failed"))
                .expect("later response");
            assert!(
                bounded(later, cx)
                    .await
                    .expect("later owner")
                    .read_with(cx, |thread, _| thread.entries().is_empty())
            );
        }
    }

    #[gpui::test]
    async fn v2_new_sdk_staging_limit_is_observable_and_preserves_live_owners(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        *harness.hold_new_responses.lock().expect("new mode") = true;
        let creation = harness.create(cx);
        let (_, responder) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("new request");
        for _ in 0..super::super::session_updates::MAX_PENDING_SESSION_UPDATES {
            harness.send_update(
                "overflowed",
                acp::SessionUpdate::SessionInfoUpdate(
                    acp::SessionInfoUpdate::new().title("prefix"),
                ),
            );
        }
        harness.drain_updates(cx).await;
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts().1,
            super::super::session_updates::MAX_PENDING_SESSION_UPDATES
        );
        harness.send_update(
            "overflowed",
            acp::SessionUpdate::SessionInfoUpdate(acp::SessionInfoUpdate::new().title("overflow")),
        );
        harness.send_update(
            "session",
            acp::SessionUpdate::SessionInfoUpdate(
                acp::SessionInfoUpdate::new().title("Live owner"),
            ),
        );
        harness.drain_updates(cx).await;
        responder
            .respond(acp::NewSessionResponse::new("overflowed"))
            .expect("new response");
        assert!(
            bounded(creation, cx)
                .await
                .expect_err("staging limit")
                .to_string()
                .contains("limits")
        );
        assert_eq!(
            bounded(harness.closes.recv(), cx)
                .await
                .expect("close failed creation"),
            acp::SessionId::new("overflowed")
        );
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.title()),
            Some("Live owner".into())
        );
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts(),
            (0, 0, 0)
        );
        assert_eq!(
            harness
                .client
                .sessions
                .borrow()
                .get(&"session".into())
                .expect("live")
                .thread
                .entity_id(),
            thread.entity_id()
        );
    }

    #[gpui::test]
    async fn v2_new_sdk_abandoned_duplicate_does_not_close_a_live_owner(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        *harness.hold_new_responses.lock().expect("new mode") = true;
        let creation = harness.create(cx);
        let (_, responder) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("new request");
        drop(creation);
        responder
            .respond(initial_response())
            .expect("duplicate late response");
        harness.drain_updates(cx).await;
        assert!(
            harness.closes.is_empty(),
            "abandoned cleanup cannot close an existing owner"
        );
        assert_eq!(
            harness
                .client
                .sessions
                .borrow()
                .get(&"session".into())
                .expect("live owner")
                .thread
                .entity_id(),
            thread.entity_id()
        );
    }

    #[gpui::test]
    async fn v2_new_sdk_preserves_pre_response_updates(cx: &mut TestAppContext) {
        use acp_thread::ForegroundActivity;
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        *harness.hold_new_responses.lock().expect("new mode") = true;
        let mut creating = harness.create(cx);
        let (_, responder) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("new request");
        for update in [
            acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                acp::RunningStateUpdate::new(),
            )),
            acp::SessionUpdate::SessionInfoUpdate(
                acp::SessionInfoUpdate::new()
                    .title("Early title")
                    .meta(meta("early")),
            ),
            acp::SessionUpdate::AgentMessage(
                acp::AgentMessage::new("early-message").content(vec!["Early content".into()]),
            ),
            serde_json::from_value(json!({
                "sessionUpdate": "tool_call_update", "toolCallId": "early-tool",
                "title": "Early tool", "status": "in_progress",
                "content": [{"type": "content", "content": {"type": "text", "text": "tool prefix"}}]
            }))
            .expect("tool update"),
            serde_json::from_value(json!({
                "sessionUpdate": "tool_call_content_chunk", "toolCallId": "early-tool",
                "content": {"type": "content", "content": {"type": "text", "text": "tool suffix"}}
            }))
            .expect("tool chunk"),
            acp::SessionUpdate::TerminalUpdate(
                acp::TerminalUpdate::new("early-terminal")
                    .command("echo")
                    .output(acp::TerminalOutput::new("b2xk")),
            ),
            acp::SessionUpdate::TerminalOutputChunk(acp::TerminalOutputChunk::new(
                "early-terminal",
                "IQ==",
            )),
        ] {
            harness
                .agent
                .send_notification(acp::UpdateSessionNotification::new("session", update))
                .expect("pre-response update");
        }
        harness.drain_updates(cx).await;
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts().1,
            7
        );
        assert!((&mut creating).now_or_never().is_none());
        responder
            .respond(initial_response())
            .expect("creation response");
        let thread = bounded(creating, cx).await.expect("created owner");
        thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
            assert_eq!(thread.session_info().title.as_deref(), Some("Early title"));
            assert!(thread.to_markdown(cx).contains("Early content"));
            let (_, tool) = thread.tool_call(&"early-tool".into()).expect("early tool");
            let [content] = tool.content() else {
                panic!("the chunk continues the text block");
            };
            assert_eq!(content.to_markdown(cx), "tool prefixtool suffix");
            let terminal = thread
                .terminal("early-terminal".into())
                .expect("early terminal");
            assert_eq!(
                terminal.read(cx).inner().read(cx).get_content().trim_end(),
                "old!"
            );
        });
    }

    #[gpui::test]
    async fn v2_session_sdk_registers_initial_state_before_immediate_notifications(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(
            session_capabilities(),
            vec![
                acp::SessionUpdate::SessionInfoUpdate(
                    acp::SessionInfoUpdate::new()
                        .title("Immediate title".to_string())
                        .meta(meta("session-info")),
                ),
                acp::SessionUpdate::AgentMessage(
                    acp::AgentMessage::new("first-message")
                        .content(vec![acp::ContentBlock::from("Immediate message")]),
                ),
            ],
            Vec::new(),
            cx,
        )
        .await;
        let thread = harness.thread(cx).await;
        cx.read(|cx| {
            let thread = thread.read(cx);
            assert_eq!(thread.session_id(), &acp::SessionId::new("session"));
            assert_eq!(
                thread.session_info().title.as_deref(),
                Some("Immediate title")
            );
            assert_eq!(thread.session_info().meta, Some(meta("session-info")));
            assert_eq!(
                thread
                    .work_dirs()
                    .expect("directories")
                    .ordered_paths()
                    .cloned()
                    .collect::<Vec<_>>(),
                vec![session_path("a")]
            );
            assert_eq!(
                thread.available_commands(),
                initial_response().available_commands
            );
            let options = harness
                .client
                .session_config_options(thread.session_id(), cx)
                .expect("config owner");
            assert_eq!(options.config_options(), initial_response().config_options);
            assert!(
                harness
                    .client
                    .receipt_submissions(thread.session_id(), cx)
                    .is_some()
            );
            assert!(thread.uses_reported_activity());
            assert!(
                harness.client.supports_load_session()
                    && harness.client.supports_resume_session()
                    && harness.client.supports_close_session()
            );
            assert!(!harness.client.should_close_sessions_on_release());
        });
        assert_eq!(thread.read_with(cx, |thread, _| thread.entries().len()), 1);
        drop(thread);
        cx.run_until_parked();
        cx.update(|_| ());
        assert!(harness.client.sessions.borrow().is_empty());
        assert!(
            harness.closes.is_empty(),
            "view release must not choose a remote-close policy"
        );
    }

    #[gpui::test]
    async fn v2_close_sdk_stop_confirmation_waits_for_preceding_updates(cx: &mut TestAppContext) {
        use acp_thread::ForegroundActivity;
        for reported_active in [false, true] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            let thread = harness
                .resumed_thread(
                    reported_active
                        .then(|| acp::StateUpdate::Running(acp::RunningStateUpdate::new())),
                    cx,
                )
                .await;
            let stopping = thread.update(cx, |thread, cx| thread.cancel(cx));
            bounded(harness.cancellations.recv(), cx)
                .await
                .expect("stop requested");
            let observed = cx.spawn({
                let thread = thread.clone();
                async move |cx| {
                    stopping.await.expect("confirmed stop");
                    thread.read_with(&cx, |thread, cx| {
                        (thread.foreground_activity(), thread.to_markdown(cx))
                    })
                }
            });
            harness.defer_closes();
            let closing = harness.close(cx);
            let (_, responder) = bounded(harness.close_requests.recv(), cx)
                .await
                .expect("close");
            harness
                .agent
                .send_notification(acp::UpdateSessionNotification::new(
                    "saved",
                    acp::SessionUpdate::AgentMessage(
                        acp::AgentMessage::new("final").content(vec!["final before close".into()]),
                    ),
                ))
                .expect("final update");
            if reported_active {
                harness.resume_state(acp::StateUpdate::Idle(
                    acp::IdleStateUpdate::new().stop_reason(acp::StopReason::Cancelled),
                ));
            }
            responder
                .respond(acp::CloseSessionResponse::new())
                .expect("closed");
            let (activity, content) = bounded(observed, cx).await;
            bounded(closing, cx).await.expect("close completed");
            assert_eq!(
                activity,
                if reported_active {
                    ForegroundActivity::Idle
                } else {
                    ForegroundActivity::Unconfirmed
                },
                "stop must observe preceding state"
            );
            assert!(
                content.contains("final before close"),
                "stop must observe preceding content"
            );
        }
    }

    #[gpui::test]
    async fn v2_close_sdk_failed_close_retains_only_captured_receipts(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Running(acp::RunningStateUpdate::new())),
                cx,
            )
            .await;
        let original = thread.update(cx, |thread, cx| thread.send(vec!["original".into()], cx));
        let (_, original_response) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("original prompt");
        harness.defer_closes();
        let closing = harness.close(cx);
        let (_, response) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("close");
        response
            .respond_with_error(agent_client_protocol::Error::new(-32042, "Close rejected"))
            .expect("rejected");
        bounded(closing, cx).await.expect_err("close failed");
        let later = thread.update(cx, |thread, cx| thread.send(vec!["later".into()], cx));
        let (_, later_response) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("later prompt");
        drop(later);
        let weak = thread.downgrade();
        drop(thread);
        cx.run_until_parked();
        assert!(
            weak.upgrade().is_some(),
            "original close receipt is still pending"
        );
        original_response
            .respond(acp::PromptResponse::new("original"))
            .expect("original receipt");
        bounded(original, cx).await.expect("original accepted");
        cx.run_until_parked();
        cx.update(|_| ());
        let retained_for_later_prompt = weak.upgrade().is_some();
        later_response
            .respond_with_error(agent_client_protocol::Error::new(
                -32800,
                "Request Cancelled",
            ))
            .expect("settle later prompt");
        cx.run_until_parked();
        assert!(
            !retained_for_later_prompt,
            "failed close must not pin future unrelated receipts"
        );
    }

    #[gpui::test]
    async fn v2_close_sdk_fences_commands_and_reopen_until_success(cx: &mut TestAppContext) {
        use acp_thread::{AcpThreadEvent, ForegroundActivity};
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Running(acp::RunningStateUpdate::new())),
                cx,
            )
            .await;
        harness.defer_closes();
        let options = cx
            .read(|cx| harness.client.session_config_options(&"saved".into(), cx))
            .expect("config");
        let stops = Rc::new(RefCell::new(0));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let stops = stops.clone();
                move |_, event, _| {
                    if matches!(event, AcpThreadEvent::Stopped { .. }) {
                        *stops.borrow_mut() += 1;
                    }
                }
            })
        });
        let (unpublished, setting, closing) = cx.update(|cx| {
            let unpublished =
                thread.update(cx, |thread, cx| thread.send(vec!["unpublished".into()], cx));
            let setting = options.set_config_option(
                "enabled".into(),
                acp::SessionConfigOptionValue::boolean(true),
                cx,
            );
            let closing = harness.client.clone().close_session(&"saved".into(), cx);
            (unpublished, setting, closing)
        });
        let (request, responder) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("close");
        assert_eq!(request.session_id, acp::SessionId::new("saved"));
        let cancellation = responder.cancellation();
        let mut duplicate = harness.close(cx);
        drop(closing);
        cx.run_until_parked();
        assert!(
            !cancellation.is_cancelled(),
            "close is not owned by its observer"
        );
        assert!((&mut duplicate).now_or_never().is_none());
        assert!(
            bounded(unpublished, cx)
                .await
                .expect_err("unpublished prompt fenced")
                .to_string()
                .contains("closing")
        );
        assert!(
            bounded(setting, cx)
                .await
                .expect_err("config preflight fenced")
                .to_string()
                .contains("closing")
        );
        assert!(
            bounded(harness.reopen(false, cx), cx)
                .await
                .expect_err("no rebind while closing")
                .to_string()
                .contains("closing")
        );
        assert!(harness.prompts.is_empty());
        assert!(harness.config_requests.is_empty());
        assert!(harness.resume_requests.is_empty());
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                "saved",
                acp::SessionUpdate::AgentMessage(
                    acp::AgentMessage::new("final").content(vec!["final output".into()]),
                ),
            ))
            .expect("final output");
        harness.resume_state(acp::StateUpdate::Idle(
            acp::IdleStateUpdate::new().stop_reason(acp::StopReason::Cancelled),
        ));
        responder
            .respond(acp::CloseSessionResponse::new())
            .expect("closed");
        bounded(duplicate, cx).await.expect("shared close result");
        cx.run_until_parked();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert!(thread.to_markdown(cx).contains("final output"));
        });
        assert_eq!(*stops.borrow(), 1);
        assert!(harness.client.sessions.borrow().is_empty());
        assert!(harness.client.session_owners.borrow().is_empty());
        assert_eq!(
            bounded(harness.closes.recv(), cx).await.expect("one close"),
            acp::SessionId::new("saved")
        );
        assert!(harness.closes.is_empty());
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                "saved",
                acp::SessionUpdate::SessionInfoUpdate(acp::SessionInfoUpdate::new().title("late")),
            ))
            .expect("late detached update");
        cx.run_until_parked();
        assert_ne!(
            thread.read_with(cx, |thread, _| thread.title()),
            Some("late".into())
        );
        let replacement = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        assert_ne!(replacement.entity_id(), thread.entity_id());
        assert!(
            bounded(
                thread.update(cx, |thread, cx| thread.send(vec!["stale".into()], cx)),
                cx
            )
            .await
            .is_err()
        );
        assert!(
            bounded(
                cx.update(|cx| options.set_config_option(
                    "enabled".into(),
                    acp::SessionConfigOptionValue::boolean(true),
                    cx
                )),
                cx
            )
            .await
            .is_err()
        );
        drop(thread);
        cx.run_until_parked();
        assert_eq!(
            harness
                .client
                .sessions
                .borrow()
                .get(&"saved".into())
                .expect("replacement")
                .thread
                .entity_id(),
            replacement.entity_id()
        );
    }

    #[gpui::test]
    async fn v2_close_sdk_preserves_late_receipts_and_releases_the_original_owner(
        cx: &mut TestAppContext,
    ) {
        use acp_thread::{SubmissionDisconnected, SubmissionResponse, SubmissionState};
        for outcome in [
            "cancelled",
            "echoed",
            "unechoed",
            "dropped",
            "eof",
            "shutdown",
        ] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            let thread = harness
                .resumed_thread(
                    Some(acp::StateUpdate::Running(acp::RunningStateUpdate::new())),
                    cx,
                )
                .await;
            let sending =
                thread.update(cx, |thread, cx| thread.send(vec!["old prompt".into()], cx));
            let submission_id = sending.id;
            let (_, receipt) = bounded(harness.prompts.recv(), cx)
                .await
                .expect("prompt published");
            if outcome == "echoed" {
                harness
                    .agent
                    .send_notification(acp::UpdateSessionNotification::new(
                        "saved",
                        acp::SessionUpdate::UserMessage(
                            acp::UserMessage::new("old-message").content(vec!["old prompt".into()]),
                        ),
                    ))
                    .expect("insertion echo");
            }
            harness.defer_closes();
            let closing = harness.close(cx);
            let (_, responder) = bounded(harness.close_requests.recv(), cx)
                .await
                .expect("close");
            harness.resume_state(acp::StateUpdate::Idle(
                acp::IdleStateUpdate::new().stop_reason(acp::StopReason::Cancelled),
            ));
            responder
                .respond(acp::CloseSessionResponse::new())
                .expect("close precedes receipt");
            bounded(closing, cx)
                .await
                .expect("close does not await prompt reply");
            thread.read_with(cx, |thread, _| {
                assert!(matches!(
                    thread.submission(submission_id).expect("pending").state,
                    SubmissionState::Pending
                ))
            });
            let weak = thread.downgrade();
            let replacement = harness
                .resumed_thread(
                    Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                    cx,
                )
                .await;
            drop(thread);
            cx.run_until_parked();
            assert!(
                weak.upgrade().is_some(),
                "pending receipt retains only its original projection"
            );
            if outcome == "dropped" {
                drop(sending);
                receipt
                    .respond(acp::PromptResponse::new("unobserved"))
                    .expect("unobserved receipt");
            } else if outcome == "eof" || outcome == "shutdown" {
                if outcome == "shutdown" {
                    harness.client.shutdown();
                    harness.client.shutdown();
                } else {
                    drop(harness._agent_task);
                    drop(harness.agent);
                }
                let error = bounded(sending, cx)
                    .await
                    .expect_err("receipt observes transport loss");
                assert!(error.is::<SubmissionDisconnected>());
            } else if outcome != "cancelled" {
                let expected = acp::PromptResponse::new("old-message").meta(meta("receipt"));
                receipt.respond(expected.clone()).expect("late acceptance");
                assert!(
                    matches!(bounded(sending, cx).await.expect("accepted"), Some(SubmissionResponse::Accepted(actual)) if actual == expected)
                );
            } else {
                let rejected = agent_client_protocol::Error::new(-32800, "Request Cancelled")
                    .data(json!({"beforeInsertion": true}));
                receipt
                    .respond_with_error(rejected.clone())
                    .expect("pre-insertion cancellation");
                assert_eq!(
                    bounded(sending, cx)
                        .await
                        .expect_err("not inserted")
                        .downcast_ref::<agent_client_protocol::Error>(),
                    Some(&rejected)
                );
            }
            cx.run_until_parked();
            cx.update(|_| ());
            assert!(
                weak.upgrade().is_none(),
                "settlement releases the retired projection"
            );
            replacement.read_with(cx, |thread, _| {
                assert!(thread.entries().is_empty());
                assert!(thread.latest_submission_id().is_none());
            });
            assert_eq!(
                harness
                    .client
                    .sessions
                    .borrow()
                    .get(&"saved".into())
                    .expect("replacement")
                    .thread
                    .entity_id(),
                replacement.entity_id()
            );
        }
    }

    #[gpui::test]
    async fn v2_close_sdk_preserves_accepted_but_unechoed_evidence(cx: &mut TestAppContext) {
        use acp_thread::{SubmissionResponse, SubmissionState};
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        let sending = thread.update(cx, |thread, cx| thread.send(vec!["retained".into()], cx));
        let submission_id = sending.id;
        let (_, responder) = bounded(harness.prompts.recv(), cx).await.expect("prompt");
        let expected = acp::PromptResponse::new("inserted").meta(meta("acceptance"));
        responder
            .respond(expected.clone())
            .expect("accepted without echo");
        assert!(
            matches!(bounded(sending, cx).await.expect("accepted"), Some(SubmissionResponse::Accepted(receipt)) if receipt == expected)
        );
        bounded(harness.close(cx), cx)
            .await
            .expect("close does not discard acceptance");
        thread.read_with(cx, |thread, _| {
            assert!(
                matches!(&thread.submission(submission_id).expect("retained evidence").state,
                SubmissionState::Accepted { receipt, echoed: false } if receipt == &expected)
            );
            assert_eq!(thread.recoverable_submissions().count(), 1);
        });
    }

    #[gpui::test]
    async fn v2_close_sdk_handles_unconfirmed_and_idle_without_inventing_state(
        cx: &mut TestAppContext,
    ) {
        use acp_thread::{AcpThreadEvent, ForegroundActivity};
        for confirmed_idle in [false, true] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            let thread = harness
                .resumed_thread(
                    confirmed_idle.then(|| acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                    cx,
                )
                .await;
            let stops = Rc::new(RefCell::new(0));
            let _subscription = cx.update(|cx| {
                cx.subscribe(&thread, {
                    let stops = stops.clone();
                    move |_, event, _| {
                        if matches!(event, AcpThreadEvent::Stopped { .. }) {
                            *stops.borrow_mut() += 1;
                        }
                    }
                })
            });
            let stopping = thread.update(cx, |thread, cx| thread.cancel(cx));
            bounded(harness.close(cx), cx)
                .await
                .expect("close needs no extra idle snapshot");
            bounded(stopping, cx)
                .await
                .expect("closed session confirms stop");
            thread.read_with(cx, |thread, _| {
                assert_eq!(
                    thread.foreground_activity(),
                    if confirmed_idle {
                        ForegroundActivity::Idle
                    } else {
                        ForegroundActivity::Unconfirmed
                    }
                );
                assert_eq!(thread.activity_generation(), 0);
                assert!(thread.activity_duration().is_none());
            });
            assert_eq!(*stops.borrow(), 0);
            assert!(
                bounded(thread.update(cx, |thread, cx| thread.cancel(cx)), cx)
                    .await
                    .is_err()
            );
        }
    }

    #[gpui::test]
    async fn v2_close_sdk_rejects_initialization_without_aborting_it(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let loading = harness.reopen(false, cx);
        let (_, responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("initialization");
        let cancellation = responder.cancellation();
        assert!(
            bounded(harness.close(cx), cx)
                .await
                .expect_err("still initializing")
                .to_string()
                .contains("initializing")
        );
        assert!(harness.closes.is_empty());
        assert!(!cancellation.is_cancelled());
        harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
        responder
            .respond(acp::ResumeSessionResponse::new())
            .expect("initialized");
        let _thread = bounded(loading, cx)
            .await
            .expect("initialization remains usable");
        bounded(harness.close(cx), cx)
            .await
            .expect("close initialized owner");
    }

    #[gpui::test]
    async fn v2_close_sdk_dropped_observers_leave_the_fence_until_settlement(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        let weak = thread.downgrade();
        harness.defer_closes();
        let closing = harness.close(cx);
        let (_, responder) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("close");
        let cancellation = responder.cancellation();
        drop(closing);
        drop(thread);
        cx.run_until_parked();
        cx.update(|_| ());
        assert!(weak.upgrade().is_none());
        assert!(!cancellation.is_cancelled());
        assert!(bounded(harness.reopen(false, cx), cx).await.is_err());
        assert!(harness.resume_requests.is_empty());
        let duplicate = harness.close(cx);
        responder
            .respond(acp::CloseSessionResponse::new())
            .expect("settled close");
        bounded(duplicate, cx)
            .await
            .expect("join without a live view");
        assert!(harness.client.session_owners.borrow().is_empty());
        let replacement = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        assert_ne!(replacement.entity_id(), weak.entity_id());
    }

    #[gpui::test]
    async fn v2_close_sdk_abandoned_connection_does_not_cycle(cx: &mut TestAppContext) {
        for published in [false, true] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            let thread = harness
                .resumed_thread(
                    Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                    cx,
                )
                .await;
            harness.defer_closes();
            let closing = harness.close(cx);
            let _responder = if published {
                Some(
                    bounded(harness.close_requests.recv(), cx)
                        .await
                        .expect("published close"),
                )
            } else {
                None
            };
            let weak = Rc::downgrade(&harness.client);
            drop(closing);
            drop(thread);
            drop(harness.client);
            cx.run_until_parked();
            cx.update(|_| ());
            assert!(
                weak.upgrade().is_none(),
                "closing task must not retain its connection"
            );
        }
    }

    #[gpui::test]
    async fn v2_close_sdk_rejection_is_shared_and_retryable(cx: &mut TestAppContext) {
        use acp_thread::ForegroundActivity;
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Running(acp::RunningStateUpdate::new())),
                cx,
            )
            .await;
        harness.defer_closes();
        let closing = harness.close(cx);
        let duplicate = harness.close(cx);
        let (_, responder) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("close");
        let rejection = agent_client_protocol::Error::new(-32042, "Close unavailable")
            .data(json!({"retain": [null, 7]}));
        responder
            .respond_with_error(rejection.clone())
            .expect("rejected");
        for result in [closing, duplicate] {
            assert_eq!(
                bounded(result, cx)
                    .await
                    .expect_err("shared rejection")
                    .downcast_ref::<agent_client_protocol::Error>(),
                Some(&rejection)
            );
        }
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
        let reused = bounded(harness.reopen(false, cx), cx)
            .await
            .expect("failed close retains binding");
        assert_eq!(reused.entity_id(), thread.entity_id());
        let retry = harness.close(cx);
        let (_, responder) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("explicit retry");
        harness.resume_state(acp::StateUpdate::Idle(
            acp::IdleStateUpdate::new().stop_reason(acp::StopReason::Cancelled),
        ));
        responder
            .respond(acp::CloseSessionResponse::new())
            .expect("closed on retry");
        bounded(retry, cx).await.expect("successful retry");
    }

    #[gpui::test]
    async fn v2_close_sdk_cleanup_does_not_remove_or_unlock_a_replacement(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        harness.defer_closes();
        let first = harness.close(cx);
        let (_, first_response) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("original close");
        harness
            .client
            .remove_session(&"saved".into(), thread.entity_id());
        let replacement = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        let second = harness.close(cx);
        let (_, second_response) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("replacement close");
        first_response
            .respond(acp::CloseSessionResponse::new())
            .expect("late original close");
        bounded(first, cx).await.expect("original close result");
        assert_eq!(
            harness
                .client
                .sessions
                .borrow()
                .get(&"saved".into())
                .expect("replacement survives")
                .thread
                .entity_id(),
            replacement.entity_id()
        );
        assert!(
            bounded(harness.reopen(false, cx), cx)
                .await
                .expect_err("replacement remains closing")
                .to_string()
                .contains("closing")
        );
        assert!(
            bounded(
                replacement.update(cx, |thread, cx| thread.send(vec!["fenced".into()], cx)),
                cx
            )
            .await
            .is_err()
        );
        assert!(harness.prompts.is_empty());
        second_response
            .respond(acp::CloseSessionResponse::new())
            .expect("replacement closed");
        bounded(second, cx).await.expect("replacement result");
        assert!(harness.client.sessions.borrow().is_empty());
    }

    #[gpui::test]
    async fn v2_close_sdk_eof_does_not_claim_idle(cx: &mut TestAppContext) {
        use acp_thread::ForegroundActivity;
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Running(acp::RunningStateUpdate::new())),
                cx,
            )
            .await;
        harness.defer_closes();
        let closing = harness.close(cx);
        let (_, _responder) = bounded(harness.close_requests.recv(), cx)
            .await
            .expect("close");
        drop(harness._agent_task);
        drop(harness.agent);
        let error = bounded(closing, cx).await.expect_err("EOF ends close");
        assert!(
            error
                .downcast_ref::<agent_client_protocol::Error>()
                .is_some_and(agent_client_protocol::is_incoming_transport_closed)
        );
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
        assert!(
            bounded(thread.update(cx, |thread, cx| thread.cancel(cx)), cx)
                .await
                .is_err()
        );
    }

    #[gpui::test]
    async fn v2_close_sdk_late_config_cannot_change_a_replacement(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        *harness.hold_config.lock().expect("config mode") = true;
        let thread = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        let options = cx
            .read(|cx| harness.client.session_config_options(&"saved".into(), cx))
            .expect("old config");
        let setting = cx.update(|cx| {
            options.set_config_option(
                "enabled".into(),
                acp::SessionConfigOptionValue::boolean(true),
                cx,
            )
        });
        bounded(harness.config_requests.recv(), cx)
            .await
            .expect("published config");
        let responder = bounded(harness.config_replies.recv(), cx)
            .await
            .expect("deferred config");
        bounded(harness.close(cx), cx).await.expect("closed");
        let replacement = harness
            .resumed_thread(
                Some(acp::StateUpdate::Idle(acp::IdleStateUpdate::new())),
                cx,
            )
            .await;
        let changed = vec![acp::SessionConfigOption::boolean(
            "enabled", "Changed", true,
        )];
        responder
            .respond(acp::SetSessionConfigOptionResponse::new(changed))
            .expect("late config response");
        bounded(setting, cx)
            .await
            .expect_err("old config is fenced");
        let current = cx
            .read(|cx| harness.client.session_config_options(&"saved".into(), cx))
            .expect("replacement config");
        assert_eq!(current.config_options(), initial_response().config_options);
        assert_eq!(options.config_options(), initial_response().config_options);
        drop(thread);
        cx.run_until_parked();
        assert_eq!(
            harness
                .client
                .sessions
                .borrow()
                .get(&"saved".into())
                .expect("replacement")
                .thread
                .entity_id(),
            replacement.entity_id()
        );
    }

    #[gpui::test]
    async fn v2_session_sdk_config_response_precedes_later_updates(cx: &mut TestAppContext) {
        let later = vec![acp::SessionConfigOption::boolean("later", "Later", true)];
        let harness = SessionHarness::new(
            session_capabilities(),
            Vec::new(),
            vec![acp::SessionUpdate::ConfigOptionUpdate(
                acp::ConfigOptionUpdate::new(later.clone()),
            )],
            cx,
        )
        .await;
        let thread = harness.thread(cx).await;
        let options = cx
            .read(|cx| harness.client.session_config_options(&"session".into(), cx))
            .expect("options");
        let updates = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let updates = updates.clone();
                move |_, event, _| {
                    if let acp_thread::AcpThreadEvent::ConfigOptionsUpdated(options) = event {
                        updates.borrow_mut().push(options.clone());
                    }
                }
            })
        });
        let setting = cx.update(|cx| {
            options.set_config_option(
                "enabled".into(),
                acp::SessionConfigOptionValue::boolean(true),
                cx,
            )
        });
        let request = bounded(harness.config_requests.recv(), cx)
            .await
            .expect("config request");
        assert_eq!(request.session_id, acp::SessionId::new("session"));
        assert_eq!(request.value, acp::SessionConfigOptionValue::boolean(true));
        let authoritative = initial_response().config_options;
        harness
            .config_responses
            .try_send(Ok(acp::SetSessionConfigOptionResponse::new(
                authoritative.clone(),
            )))
            .expect("response");
        assert_eq!(
            bounded(setting, cx).await.expect("set config"),
            authoritative
        );
        bounded(drain_foreground_queue(&harness.client.dispatch_tx), cx)
            .await
            .expect("drain updates");
        cx.run_until_parked();
        assert_eq!(options.config_options(), later);
        assert_eq!(*updates.borrow(), vec![authoritative, later]);
        let invalid = cx.update(|cx| {
            options.set_config_option(
                "missing".into(),
                acp::SessionConfigOptionValue::boolean(false),
                cx,
            )
        });
        assert!(bounded(invalid, cx).await.is_err());
        assert!(harness.config_requests.is_empty());
        drop(thread);
        cx.run_until_parked();
        cx.update(|_| ());
        let stale = cx.update(|cx| {
            options.set_config_option(
                "later".into(),
                acp::SessionConfigOptionValue::boolean(false),
                cx,
            )
        });
        assert!(bounded(stale, cx).await.is_err());
        assert!(harness.config_requests.is_empty());
    }

    #[gpui::test]
    async fn v2_session_sdk_rejects_missing_capabilities_and_duplicate_ids(
        cx: &mut TestAppContext,
    ) {
        let unsupported =
            SessionHarness::new(acp::AgentCapabilities::new(), Vec::new(), Vec::new(), cx).await;
        assert!(
            cx.update(|cx| unsupported.client.session_list(cx))
                .is_none()
        );
        assert!(bounded(unsupported.create(cx), cx).await.is_err());
        assert!(unsupported.new_requests.is_empty());
        assert!(!unsupported.client.supports_load_session());
        assert!(!unsupported.client.supports_resume_session());
        assert!(!unsupported.client.supports_close_session());
        bounded(unsupported.close(cx), cx)
            .await
            .expect_err("close requires session support");
        for replay_history in [false, true] {
            bounded(unsupported.reopen(replay_history, cx), cx)
                .await
                .expect_err("session capability required");
        }
        assert!(unsupported.resume_requests.is_empty());
        assert!(unsupported.client.session_owners.borrow().is_empty());
        let harness = SessionHarness::new(session_capabilities(), Vec::new(), Vec::new(), cx).await;
        let thread = harness.thread(cx).await;
        let replacement = harness.create(cx);
        bounded(harness.new_requests.recv(), cx)
            .await
            .expect("second request");
        harness
            .new_responses
            .try_send(Ok(initial_response()))
            .expect("duplicate response");
        assert!(bounded(replacement, cx).await.is_err());
        assert_eq!(
            harness
                .client
                .sessions
                .borrow()
                .get(&"session".into())
                .expect("original")
                .thread
                .entity_id(),
            thread.entity_id()
        );
        assert!(
            harness.closes.is_empty(),
            "duplicate IDs must not close the original session"
        );
    }

    #[gpui::test]
    async fn v2_session_sdk_abandoned_creation_closes_late_response_and_eof_rejects(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), Vec::new(), Vec::new(), cx).await;
        let unpublished = harness.create(cx);
        drop(unpublished);
        cx.run_until_parked();
        assert!(harness.new_requests.is_empty());
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts(),
            (0, 0, 0)
        );
        *harness.hold_new_responses.lock().expect("new mode") = true;
        let creation = harness.create(cx);
        let (_, responder) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("request");
        harness.send_update(
            "session",
            acp::SessionUpdate::AgentMessage(
                acp::AgentMessage::new("staged").content(vec!["staged".into()]),
            ),
        );
        harness.drain_updates(cx).await;
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts().1,
            1
        );
        drop(creation);
        cx.run_until_parked();
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts(),
            (0, 0, 0)
        );
        responder
            .respond(initial_response())
            .expect("late response");
        assert_eq!(
            bounded(harness.closes.recv(), cx)
                .await
                .expect("abandoned close"),
            acp::SessionId::new("session")
        );
        assert!(harness.client.sessions.borrow().is_empty());
        let creation = harness.create(cx);
        let (_, _responder) = bounded(harness.new_replies.recv(), cx)
            .await
            .expect("next request");
        harness.send_update(
            "session",
            acp::SessionUpdate::AgentMessage(
                acp::AgentMessage::new("next").content(vec!["next prefix".into()]),
            ),
        );
        harness.drain_updates(cx).await;
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts().1,
            1
        );
        drop(harness._agent_task);
        drop(harness.agent);
        assert!(bounded(creation, cx).await.is_err());
        cx.run_until_parked();
        assert!(harness.client.sessions.borrow().is_empty());
        assert_eq!(
            harness.client.pending_session_updates.borrow().counts(),
            (0, 0, 0)
        );
    }

    #[gpui::test]
    async fn v2_session_sdk_creation_owns_request_scoped_input(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), Vec::new(), Vec::new(), cx).await;
        let creation = harness.create(cx);
        let (_, request_id) = bounded(harness.new_requests.recv(), cx)
            .await
            .expect("parent request");
        let store = harness.client.request_elicitations();
        let input = {
            let timeout = cx.background_executor.timer(Duration::from_secs(5));
            let registered = store.next_event::<acp_thread::ElicitationStoreEvent>(cx);
            let input = harness
                .agent
                .send_request(acp::CreateElicitationRequest::new(
                    acp::ElicitationFormMode::new(
                        acp::ElicitationRequestScope::new(request_id.clone()),
                        acp::ElicitationSchema::new().string("name", true),
                    ),
                    "Creation input",
                ))
                .block_task();
            futures::pin_mut!(registered, timeout);
            assert!(
                matches!(
                    futures::future::select(registered, timeout).await,
                    futures::future::Either::Left(_)
                ),
                "creation input was not registered"
            );
            input
        };
        assert!(harness.client.sessions.borrow().is_empty());
        drop(creation);
        harness
            .new_responses
            .try_send(Ok(initial_response()))
            .expect("late response");
        assert_eq!(
            bounded(input, cx).await.expect("settled input").action,
            acp::ElicitationAction::Cancel
        );
        bounded(harness.closes.recv(), cx)
            .await
            .expect("close abandoned session");
        let late = harness
            .agent
            .send_request(acp::CreateElicitationRequest::new(
                acp::ElicitationFormMode::new(
                    acp::ElicitationRequestScope::new(request_id),
                    acp::ElicitationSchema::new().string("name", true),
                ),
                "Late creation input",
            ))
            .block_task();
        assert!(bounded(late, cx).await.is_err());
    }

    #[gpui::test]
    async fn v2_session_sdk_rejection_and_released_config_do_not_strand_dispatch(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), Vec::new(), Vec::new(), cx).await;
        let creation = harness.create(cx);
        bounded(harness.new_requests.recv(), cx)
            .await
            .expect("request");
        let rejected =
            agent_client_protocol::Error::invalid_params().data(json!({"scope": "new-rejection"}));
        harness
            .new_responses
            .try_send(Err(rejected.clone()))
            .expect("reject");
        let error = bounded(creation, cx).await.expect_err("creation rejection");
        assert_eq!(
            error.downcast_ref::<agent_client_protocol::Error>(),
            Some(&rejected)
        );
        let thread = harness.thread(cx).await;
        let options = cx
            .read(|cx| harness.client.session_config_options(&"session".into(), cx))
            .expect("config");
        let setting = cx.update(|cx| {
            options.set_config_option(
                "enabled".into(),
                acp::SessionConfigOptionValue::boolean(true),
                cx,
            )
        });
        bounded(harness.config_requests.recv(), cx)
            .await
            .expect("config request");
        drop(thread);
        cx.run_until_parked();
        cx.update(|_| ());
        let response = vec![acp::SessionConfigOption::boolean(
            "enabled", "Changed", true,
        )];
        harness
            .config_responses
            .try_send(Ok(acp::SetSessionConfigOptionResponse::new(response)))
            .expect("late config");
        assert!(bounded(setting, cx).await.is_err());
        assert_eq!(options.config_options(), initial_response().config_options);
        bounded(drain_foreground_queue(&harness.client.dispatch_tx), cx)
            .await
            .expect("dispatch released");
    }

    #[gpui::test]
    async fn v2_session_sdk_drop_after_registration_closes_undelivered_session(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), Vec::new(), Vec::new(), cx).await;
        let creation = harness.create(cx);
        bounded(harness.new_requests.recv(), cx)
            .await
            .expect("creation request");
        harness
            .new_responses
            .try_send(Ok(initial_response()))
            .expect("creation response");
        let executor = cx.executor();
        let dispatcher = executor
            .dispatcher()
            .as_test()
            .expect("deterministic dispatcher");
        for _ in 0..1000 {
            dispatcher.tick(true);
            dispatcher.tick(false);
            if !harness.client.sessions.borrow().is_empty() {
                break;
            }
        }
        assert_eq!(harness.client.sessions.borrow().len(), 1);
        drop(creation);
        assert_eq!(
            bounded(harness.closes.recv(), cx)
                .await
                .expect("close undelivered session"),
            acp::SessionId::new("session")
        );
        cx.run_until_parked();
        cx.update(|_| ());
        assert!(harness.client.sessions.borrow().is_empty());
    }

    #[gpui::test]
    async fn v2_session_sdk_config_preflight_rechecks_before_publication(cx: &mut TestAppContext) {
        let harness = SessionHarness::new(session_capabilities(), Vec::new(), Vec::new(), cx).await;
        let _thread = harness.thread(cx).await;
        let options = cx
            .read(|cx| harness.client.session_config_options(&"session".into(), cx))
            .expect("config");
        let setting = cx.update(|cx| {
            options.set_config_option(
                "enabled".into(),
                acp::SessionConfigOptionValue::boolean(true),
                cx,
            )
        });
        harness
            .client
            .sessions
            .borrow()
            .get(&"session".into())
            .expect("session")
            .config_options
            .as_ref()
            .expect("options")
            .config_options
            .borrow_mut()
            .clear();
        let error = bounded(setting, cx).await.expect_err("stale config");
        assert!(error.to_string().contains("not offered"), "{error}");
        assert!(
            harness.config_requests.is_empty(),
            "stale preflight must not publish a config RPC"
        );
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_new_stdio_preserves_pre_response_updates_before_eof(cx: &mut TestAppContext) {
        use acp_thread::ForegroundActivity;
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec!["-c".into(), r#"
while IFS= read -r request; do
    id=${request#*\"id\":}
    id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":2,"info":{"name":"early-stdio","version":"1"},"capabilities":{"session":{}}}}\n' "$id"
            ;;
        *'"method":"session/new"'*)
            printf '{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"stdio-session","update":{"sessionUpdate":"state_update","state":"running"}}}\n'
            printf '{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"stdio-session","update":{"sessionUpdate":"agent_message","messageId":"first","content":[{"type":"text","text":"Early stdio message"}]}}}\n'
            printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"stdio-session"}}\n' "$id"
            exit 0
            ;;
        *) exit 43 ;;
    esac
done
"#.into()],
            env: None,
        };
        let client = Rc::new(
            bounded(
                AcpV2Connection::stdio(
                    AgentId::new("early-stdio"),
                    project.clone(),
                    command,
                    WeakEntity::new_invalid(),
                    &mut cx.to_async(),
                ),
                cx,
            )
            .await
            .expect("stdio connection"),
        );
        let creation = cx.update(|cx| {
            client
                .clone()
                .new_session(project, PathList::new(&[directory.path()]), cx)
        });
        let thread = bounded(creation, cx)
            .await
            .expect("new response precedes EOF");
        bounded(client.connection.incoming_closed(), cx).await;
        thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
            assert!(thread.to_markdown(cx).contains("Early stdio message"));
        });
        assert_eq!(client.pending_session_updates.borrow().counts(), (0, 0, 0));
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_session_stdio_applies_initial_state_and_authoritative_config(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec!["-c".into(), r#"
while IFS= read -r request; do
    id=${request#*\"id\":}
    id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":2,"info":{"name":"session-stdio","version":"1"},"capabilities":{"session":{}}}}\n' "$id"
            ;;
        *'"method":"session/new"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"stdio-session","configOptions":[{"configId":"enabled","name":"Enabled","type":"boolean","currentValue":false}],"availableCommands":[{"type":"command","name":"initial","description":"Initial"}]}}\n' "$id"
            printf '{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"stdio-session","update":{"sessionUpdate":"session_info_update","title":"Immediate stdio title"}}}\n'
            ;;
        *'"method":"session/set_config_option"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"configOptions":[{"configId":"enabled","name":"Authoritative","type":"boolean","currentValue":false}],"_meta":{"scope":"response"}}}\n' "$id"
            printf 'session final stderr\n' >&2
            exit 0
            ;;
        *) exit 43 ;;
    esac
done
"#.into()],
            env: None,
        };
        let client = Rc::new(
            bounded(
                AcpV2Connection::stdio(
                    AgentId::new("session-stdio"),
                    project.clone(),
                    command,
                    WeakEntity::new_invalid(),
                    &mut cx.to_async(),
                ),
                cx,
            )
            .await
            .expect("initialize stdio"),
        );
        let creation = cx.update(|cx| {
            client
                .clone()
                .new_session(project, PathList::new(&[directory.path()]), cx)
        });
        let thread = bounded(creation, cx).await.expect("create over stdio");
        bounded(
            thread.condition::<acp_thread::AcpThreadEvent>(cx, |thread, _| {
                thread.session_info().title.as_deref() == Some("Immediate stdio title")
            }),
            cx,
        )
        .await;
        bounded(drain_foreground_queue(&client.dispatch_tx), cx)
            .await
            .expect("apply immediate update");
        let options = cx.read(|cx| {
            assert_eq!(
                thread.read(cx).session_info().title.as_deref(),
                Some("Immediate stdio title")
            );
            assert_eq!(thread.read(cx).available_commands().len(), 1);
            client
                .session_config_options(&"stdio-session".into(), cx)
                .expect("config")
        });
        let setting = cx.update(|cx| {
            options.set_config_option(
                "enabled".into(),
                acp::SessionConfigOptionValue::boolean(true),
                cx,
            )
        });
        let expected = vec![acp::SessionConfigOption::boolean(
            "enabled",
            "Authoritative",
            false,
        )];
        assert_eq!(
            bounded(setting, cx).await.expect("set over stdio"),
            expected
        );
        assert_eq!(options.config_options(), expected);
        bounded(client.connection.incoming_closed(), cx).await;
        let (messages, _) = client.subscribe_debug_messages();
        for method_name in ["initialize", "session/new", "session/set_config_option"] {
            assert_eq!(messages.iter().filter(|message| matches!(
                &message.message, super::super::AcpDebugMessageContent::Request { method, .. }
                    if method.as_ref() == method_name
            )).count(), 1);
        }
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_close_stdio_preserves_success_before_immediate_eof(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec!["-c".into(), r#"
while IFS= read -r request; do
    id=${request#*\"id\":}
    id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":2,"info":{"name":"close-stdio","version":"1"},"capabilities":{"session":{}}}}\n' "$id"
            ;;
        *'"method":"session/new"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"stdio-session"}}\n' "$id"
            ;;
        *'"method":"session/close"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{}}\n' "$id"
            exit 0
            ;;
        *) exit 43 ;;
    esac
done
"#.into()],
            env: None,
        };
        let client = Rc::new(
            bounded(
                AcpV2Connection::stdio(
                    AgentId::new("close-stdio"),
                    project.clone(),
                    command,
                    WeakEntity::new_invalid(),
                    &mut cx.to_async(),
                ),
                cx,
            )
            .await
            .expect("stdio connection"),
        );
        let creation = cx.update(|cx| {
            client
                .clone()
                .new_session(project, PathList::new(&[directory.path()]), cx)
        });
        let _thread = bounded(creation, cx).await.expect("stdio owner");
        let closing = cx.update(|cx| client.clone().close_session(&"stdio-session".into(), cx));
        bounded(closing, cx)
            .await
            .expect("successful close precedes EOF");
        bounded(client.connection.incoming_closed(), cx).await;
        assert!(client.sessions.borrow().is_empty());
        assert!(client.session_owners.borrow().is_empty());
    }

    #[gpui::test]
    async fn v2_state_sdk_separates_preinsertion_work_and_local_command_completion(
        cx: &mut TestAppContext,
    ) {
        use acp_thread::{
            AcpThreadEvent, AgentThreadEntry, ForegroundActivity, SubmissionResponse,
        };
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let thread = harness.thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert!(!thread.has_received_session_state());
            thread.session_id().clone()
        });
        let stops = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let stops = stops.clone();
                move |_, event, _| {
                    if let AcpThreadEvent::Stopped { stop_reason, .. } = event {
                        stops.borrow_mut().push(stop_reason.clone());
                    }
                }
            })
        });
        let rejected = thread.update(cx, |thread, cx| thread.send(vec!["/rejected".into()], cx));
        let (_, responder) = bounded(harness.prompts.recv(), cx).await.expect("prompt");
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                session_id.clone(),
                acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                    acp::RunningStateUpdate::new(),
                )),
            ))
            .expect("foreground hook before insertion");
        responder
            .respond_with_error(agent_client_protocol::Error::invalid_params())
            .expect("rejection");
        bounded(rejected, cx).await.expect_err("not inserted");
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
            assert!(thread.entries().is_empty());
        });
        assert!(stops.borrow().is_empty());
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                session_id.clone(),
                acp::SessionUpdate::StateUpdate(
                    acp::StateUpdate::Idle(acp::IdleStateUpdate::new()),
                ),
            ))
            .expect("hook work ends separately");
        cx.run_until_parked();
        assert_eq!(*stops.borrow(), [None]);
        let command = thread.update(cx, |thread, cx| thread.send(vec!["/local".into()], cx));
        let (_, responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("local command");
        for update in [
            acp::SessionUpdate::UserMessage(
                acp::UserMessage::new("local-command").content(vec!["/local".into()]),
            ),
            acp::SessionUpdate::StateUpdate(acp::StateUpdate::Running(
                acp::RunningStateUpdate::new(),
            )),
        ] {
            harness
                .agent
                .send_notification(acp::UpdateSessionNotification::new(
                    session_id.clone(),
                    update,
                ))
                .expect("local command handling");
        }
        responder
            .respond(acp::PromptResponse::new("local-command"))
            .expect("insertion receipt");
        assert!(matches!(
            bounded(command, cx).await.expect("accepted"),
            Some(SubmissionResponse::Accepted(_))
        ));
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                session_id,
                acp::SessionUpdate::StateUpdate(acp::StateUpdate::Idle(
                    acp::IdleStateUpdate::new().stop_reason(acp::StopReason::EndTurn),
                )),
            ))
            .expect("local completion without model output");
        cx.run_until_parked();
        assert_eq!(*stops.borrow(), [None, Some(acp::StopReason::EndTurn)]);
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert!(matches!(
                thread.entries(),
                [AgentThreadEntry::UserMessage(_)]
            ));
        });
    }

    #[gpui::test]
    async fn v2_resume_sdk_synchronizes_current_state_without_inventing_completions(
        cx: &mut TestAppContext,
    ) {
        use acp_thread::{AcpThreadEvent, ForegroundActivity};
        for replay_history in [false, true] {
            for snapshot in [
                json!({"state": "idle", "stopReason": "cancelled"}),
                json!({"state": "running"}),
                json!({"state": "requires_action"}),
            ] {
                let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
                let mut loading = harness.reopen(replay_history, cx);
                let (_, responder) = bounded(harness.resume_requests.recv(), cx)
                    .await
                    .expect("resume");
                let opening = harness
                    .client
                    .sessions
                    .borrow()
                    .get(&"saved".into())
                    .expect("registered before replay")
                    .thread
                    .upgrade()
                    .expect("owner");
                let stops = Rc::new(RefCell::new(Vec::new()));
                let _subscription = cx.update(|cx| {
                    cx.subscribe(&opening, {
                        let stops = stops.clone();
                        move |_, event, _| {
                            if let AcpThreadEvent::Stopped {
                                activity_generation,
                                ..
                            } = event
                            {
                                stops.borrow_mut().push(*activity_generation);
                            }
                        }
                    })
                });
                if replay_history {
                    harness
                        .agent
                        .send_notification(acp::UpdateSessionNotification::new(
                            "saved",
                            acp::SessionUpdate::UserMessage(
                                acp::UserMessage::new("history")
                                    .content(vec!["retained history".into()]),
                            ),
                        ))
                        .expect("history");
                }
                let snapshot: acp::StateUpdate = serde_json::from_value(snapshot).expect("state");
                harness.resume_state(snapshot.clone());
                cx.run_until_parked();
                assert!((&mut loading).now_or_never().is_none());
                opening.read_with(cx, |thread, _| {
                    assert_eq!(thread.foreground_state(), Some(&snapshot));
                    assert!(thread.has_received_session_state());
                    assert_eq!(thread.entries().len(), usize::from(replay_history));
                });
                assert!(
                    stops.borrow().is_empty(),
                    "resume snapshot is not a completion"
                );
                let latest = acp::StateUpdate::RequiresAction(
                    acp::RequiresActionStateUpdate::new().meta(meta("latest")),
                );
                harness.resume_state(latest.clone());
                responder
                    .respond(acp::ResumeSessionResponse::new())
                    .expect("resume response");
                let thread = bounded(loading, cx).await.expect("synchronized owner");
                assert_eq!(
                    thread.read_with(cx, |thread, _| thread.foreground_state().cloned()),
                    Some(latest.clone())
                );
                harness.resume_state(latest.clone());
                cx.run_until_parked();
                thread.read_with(cx, |thread, _| {
                    assert_eq!(thread.foreground_state(), Some(&latest));
                    assert_eq!(
                        thread.foreground_activity(),
                        ForegroundActivity::RequiresAction
                    );
                    assert_eq!(thread.activity_generation(), 1);
                });
                assert!(stops.borrow().is_empty());
                let idle = acp::StateUpdate::Idle(
                    acp::IdleStateUpdate::new().stop_reason(acp::StopReason::EndTurn),
                );
                harness.resume_state(idle.clone());
                harness.resume_state(idle);
                cx.run_until_parked();
                assert_eq!(*stops.borrow(), [1]);
                assert_eq!(
                    thread.read_with(cx, |thread, _| thread.foreground_activity()),
                    ForegroundActivity::Idle
                );
                assert!(harness.closes.is_empty());
            }
        }
    }

    #[gpui::test]
    async fn v2_resume_sdk_keeps_attachment_without_state_snapshot(cx: &mut TestAppContext) {
        use acp_thread::{AcpThreadEvent, ForegroundActivity, SubmissionResponse, ThreadStatus};
        for (replay_history, late_snapshot) in
            [(false, false), (true, false), (false, true), (true, true)]
        {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            let loading = harness.reopen(replay_history, cx);
            let (_, responder) = bounded(harness.resume_requests.recv(), cx)
                .await
                .expect("resume");
            let owner = harness
                .client
                .sessions
                .borrow()
                .get(&"saved".into())
                .expect("owner")
                .thread
                .clone();
            let opening = owner.upgrade().expect("opening owner");
            let stops = Rc::new(RefCell::new(0));
            let _subscription = cx.update(|cx| {
                cx.subscribe(&opening, {
                    let stops = stops.clone();
                    move |_, event, _| {
                        if matches!(event, AcpThreadEvent::Stopped { .. }) {
                            *stops.borrow_mut() += 1;
                        }
                    }
                })
            });
            let input = cx.background_spawn(
                harness
                    .agent
                    .send_request(acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(
                                v2_request_id(responder.id()).expect("v2 request ID"),
                            ),
                            acp::ElicitationSchema::new(),
                        ),
                        "Resume setup input",
                    ))
                    .block_task(),
            );
            cx.run_until_parked();
            if replay_history {
                harness
                    .agent
                    .send_notification(acp::UpdateSessionNotification::new(
                        "saved",
                        acp::SessionUpdate::UserMessage(
                            acp::UserMessage::new("retained")
                                .content(vec!["retained history".into()]),
                        ),
                    ))
                    .expect("replay without state");
            }
            let initial = initial_response();
            responder
                .respond(
                    acp::ResumeSessionResponse::new()
                        .config_options(initial.config_options.clone())
                        .available_commands(initial.available_commands.clone()),
                )
                .expect("missing snapshot response");
            if late_snapshot {
                harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
            }
            let thread = bounded(loading, cx).await.expect("attachment succeeds");
            assert_eq!(thread.entity_id(), opening.entity_id());
            assert_eq!(
                bounded(input, cx)
                    .await
                    .expect("finished resume input cleanup")
                    .action,
                acp::ElicitationAction::Cancel,
            );
            cx.run_until_parked();
            thread.read_with(cx, |thread, _| {
                assert_eq!(thread.entries().len(), usize::from(replay_history));
                assert_eq!(thread.available_commands(), initial.available_commands);
                assert_eq!(thread.activity_generation(), 0);
                assert_eq!(
                    thread.status(),
                    if late_snapshot {
                        ThreadStatus::Idle
                    } else {
                        ThreadStatus::Unconfirmed
                    }
                );
                if !late_snapshot {
                    assert!(thread.foreground_state().is_none());
                    assert!(!thread.is_idle_for_retention());
                }
            });
            let options = cx
                .read(|cx| harness.client.session_config_options(&"saved".into(), cx))
                .expect("config survives missing state");
            assert_eq!(options.config_options(), initial.config_options);
            let reused = bounded(harness.reopen(replay_history, cx), cx)
                .await
                .expect("reuse binding");
            assert_eq!(reused.entity_id(), thread.entity_id());
            assert!(harness.resume_requests.is_empty(), "no forced retry");
            if !late_snapshot {
                let sending =
                    thread.update(cx, |thread, cx| thread.send(vec!["manual".into()], cx));
                let (_, responder) = bounded(harness.prompts.recv(), cx)
                    .await
                    .expect("manual send");
                responder
                    .respond(acp::PromptResponse::new("manual"))
                    .expect("receipt");
                harness
                    .agent
                    .send_notification(acp::UpdateSessionNotification::new(
                        "saved",
                        acp::SessionUpdate::UserMessage(
                            acp::UserMessage::new("manual").content(vec!["manual".into()]),
                        ),
                    ))
                    .expect("echo");
                assert!(matches!(
                    bounded(sending, cx).await.expect("accepted"),
                    Some(SubmissionResponse::Accepted(_))
                ));
                assert_eq!(
                    thread.read_with(cx, |thread, _| thread.foreground_activity()),
                    ForegroundActivity::Unconfirmed
                );
                harness.resume_state(acp::StateUpdate::Idle(
                    acp::IdleStateUpdate::new().stop_reason(acp::StopReason::Cancelled),
                ));
                cx.run_until_parked();
            }
            assert_eq!(
                *stops.borrow(),
                0,
                "confirmation does not create a completion"
            );
            assert_eq!(
                thread.read_with(cx, |thread, _| thread.foreground_activity()),
                ForegroundActivity::Idle
            );
            assert!(harness.closes.is_empty());
        }
    }

    #[gpui::test]
    async fn v2_resume_sdk_applies_replay_before_reply_and_keeps_receipt_transport(
        cx: &mut TestAppContext,
    ) {
        use acp_thread::{AgentThreadEntry, ForegroundActivity, SubmissionResponse};
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let mut loading = harness.reopen(true, cx);
        let opening = harness
            .client
            .sessions
            .borrow()
            .get(&acp::SessionId::new("saved"))
            .expect("registered before publication")
            .thread
            .upgrade()
            .expect("loading owner");
        let changes = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&opening, {
                let changes = changes.clone();
                move |_, event, _| {
                    if let acp_thread::AcpThreadEvent::ConfigOptionsUpdated(options) = event {
                        changes.borrow_mut().push(options.clone());
                    }
                }
            })
        });
        let (request, responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("resume request");
        assert_eq!(request.session_id, acp::SessionId::new("saved"));
        assert_eq!(request.cwd.0, session_path("a"));
        assert_eq!(
            request.replay_from,
            Some(acp::ReplayFrom::Start(acp::ReplayFromStart::new()))
        );
        let replay_options = vec![acp::SessionConfigOption::boolean("replay", "Replay", true)];
        for update in [
            acp::SessionUpdate::UserMessage(
                acp::UserMessage::new("history").content(vec!["old question".into()]),
            ),
            acp::SessionUpdate::SessionInfoUpdate(
                acp::SessionInfoUpdate::new().title("Agent title"),
            ),
            acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate::new(
                replay_options.clone(),
            )),
        ] {
            harness
                .agent
                .send_notification(acp::UpdateSessionNotification::new("saved", update))
                .expect("replay notification");
        }
        harness.resume_state(acp::StateUpdate::Running(acp::RunningStateUpdate::new()));
        cx.run_until_parked();
        assert!(
            (&mut loading).now_or_never().is_none(),
            "replay is not initialization completion"
        );
        opening.read_with(cx, |thread, _| {
            assert!(thread.uses_reported_activity());
            assert_eq!(thread.entries().len(), 1);
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
            assert_eq!(thread.session_info().title.as_deref(), Some("Agent title"));
        });
        let initial = initial_response();
        responder
            .respond(
                acp::ResumeSessionResponse::new()
                    .config_options(initial.config_options.clone())
                    .available_commands(initial.available_commands.clone())
                    .meta(meta("delivery-not-session")),
            )
            .expect("resume response");
        let later_options = vec![acp::SessionConfigOption::boolean("later", "Later", false)];
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                "saved",
                acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate::new(
                    later_options.clone(),
                )),
            ))
            .expect("post-response notification");
        let thread = bounded(loading, cx).await.expect("loaded");
        assert_eq!(thread.entity_id(), opening.entity_id());
        cx.run_until_parked();
        assert_eq!(
            *changes.borrow(),
            vec![
                replay_options,
                initial.config_options,
                later_options.clone()
            ]
        );
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.available_commands().to_vec()),
            initial.available_commands
        );
        let options = cx
            .read(|cx| harness.client.session_config_options(&"saved".into(), cx))
            .expect("options");
        assert_eq!(options.config_options(), later_options);
        let submission = thread.update(cx, |thread, cx| thread.send(vec!["follow-up".into()], cx));
        let (prompt, responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("resumed prompt");
        assert_eq!(prompt.session_id, acp::SessionId::new("saved"));
        responder
            .respond(acp::PromptResponse::new("follow-up"))
            .expect("receipt");
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                "saved",
                acp::SessionUpdate::UserMessage(
                    acp::UserMessage::new("follow-up").content(vec!["follow-up".into()]),
                ),
            ))
            .expect("echo");
        assert!(matches!(
            bounded(submission, cx).await.expect("accepted"),
            Some(SubmissionResponse::Accepted(_))
        ));
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread
                    .entries()
                    .iter()
                    .filter(|entry| matches!(entry, AgentThreadEntry::UserMessage(_)))
                    .count(),
                2
            );
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
        });
        assert!(harness.closes.is_empty());
    }

    #[gpui::test]
    async fn v2_resume_sdk_shares_compatible_opens_and_rejects_context_or_history_changes(
        cx: &mut TestAppContext,
    ) {
        for replay_history in [false, true] {
            let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
            let first = harness.reopen(replay_history, cx);
            let second = harness.reopen(replay_history, cx);
            let (request, responder) = bounded(harness.resume_requests.recv(), cx)
                .await
                .expect("one resume");
            assert_eq!(request.replay_from.is_some(), replay_history);
            let cancellation = responder.cancellation();
            drop(first);
            cx.run_until_parked();
            assert!(
                !cancellation.is_cancelled(),
                "another observer still owns initialization"
            );
            assert!(harness.resume_requests.is_empty());
            if !replay_history {
                bounded(harness.reopen(true, cx), cx)
                    .await
                    .expect_err("cannot promise missing history");
            }
            harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
            responder
                .respond(acp::ResumeSessionResponse::new())
                .expect("resumed");
            let thread = bounded(second, cx).await.expect("remaining observer");
            let reused = thread.update(cx, |_, cx| {
                harness.client.clone().resume_session(
                    "saved".into(),
                    harness.project.clone(),
                    PathList::new(&[session_path("a"), session_path("b")]),
                    None,
                    cx,
                )
            });
            let reused = bounded(reused, cx)
                .await
                .expect("reuse without reborrowing the owner");
            assert_eq!(reused.entity_id(), thread.entity_id());
            if !replay_history {
                bounded(harness.reopen(true, cx), cx)
                    .await
                    .expect_err("ready owner still lacks old history");
            }
            let incompatible = cx.update(|cx| {
                harness.client.clone().resume_session(
                    "saved".into(),
                    harness.project.clone(),
                    PathList::new(&[session_path("b")]),
                    None,
                    cx,
                )
            });
            bounded(incompatible, cx)
                .await
                .expect_err("cannot retarget a live owner");
            assert!(harness.resume_requests.is_empty());
            assert!(harness.closes.is_empty());
        }
    }

    #[gpui::test]
    async fn v2_resume_sdk_fences_abandoned_replay_and_cancels_only_its_input(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let abandoned = harness.reopen(true, cx);
        let (_, responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("old resume");
        let cancellation = responder.cancellation();
        let old_owner = harness
            .client
            .sessions
            .borrow()
            .get(&acp::SessionId::new("saved"))
            .expect("owner")
            .thread
            .clone();
        let input = cx.background_spawn(
            harness
                .agent
                .send_request(acp::CreateElicitationRequest::new(
                    acp::ElicitationFormMode::new(
                        acp::ElicitationRequestScope::new(
                            v2_request_id(responder.id()).expect("v2 request ID"),
                        ),
                        acp::ElicitationSchema::new(),
                    ),
                    "Resume setup",
                ))
                .block_task(),
        );
        cx.run_until_parked();
        drop(abandoned);
        cx.run_until_parked();
        // Dispatcher quiescence alone does not flush entity-release effects.
        cx.update(|_| ());
        assert!(old_owner.upgrade().is_none());
        bounded(cancellation.cancelled(), cx).await;
        assert_eq!(
            bounded(input, cx).await.expect("input cleanup").action,
            acp::ElicitationAction::Cancel
        );
        let replacement = harness.reopen(true, cx);
        cx.run_until_parked();
        assert!(
            harness.resume_requests.is_empty(),
            "old RPC still owns the routing fence"
        );
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                "saved",
                acp::SessionUpdate::UserMessage(
                    acp::UserMessage::new("old-replay").content(vec!["must not leak".into()]),
                ),
            ))
            .expect("late old replay");
        harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
        responder
            .respond(acp::ResumeSessionResponse::new())
            .expect("old RPC settles");
        let (_, responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("replacement may now publish");
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                "saved",
                acp::SessionUpdate::UserMessage(
                    acp::UserMessage::new("fresh-replay").content(vec!["fresh".into()]),
                ),
            ))
            .expect("replacement replay");
        harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
        responder
            .respond(acp::ResumeSessionResponse::new())
            .expect("replacement response");
        let thread = bounded(replacement, cx)
            .await
            .expect("replacement initialized");
        thread.read_with(cx, |thread, _| {
            let [acp_thread::AgentThreadEntry::UserMessage(message)] = thread.entries() else {
                panic!("only replacement history")
            };
            assert_eq!(
                message.content.source_blocks(),
                &[acp::ContentBlock::from("fresh")]
            );
        });
        assert!(
            harness.closes.is_empty(),
            "view release must not close agent-owned work"
        );
    }

    #[gpui::test]
    async fn v2_resume_sdk_rejection_preserves_shared_auth_errors_and_discards_partial_replay(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let first = harness.reopen(true, cx);
        let second = harness.reopen(true, cx);
        let (_, responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("resume");
        let old_owner = harness
            .client
            .sessions
            .borrow()
            .get(&acp::SessionId::new("saved"))
            .expect("owner")
            .thread
            .clone();
        harness
            .agent
            .send_notification(acp::UpdateSessionNotification::new(
                "saved",
                acp::SessionUpdate::UserMessage(
                    acp::UserMessage::new("partial").content(vec!["not initialized".into()]),
                ),
            ))
            .expect("partial replay");
        let rejection = agent_client_protocol::Error::new(
            i32::from(acp::ErrorCode::AuthRequired),
            "Sign in first",
        )
        .data(json!({"retain": [null, 7]}));
        responder
            .respond_with_error(rejection.clone())
            .expect("rejection");
        for observer in [first, second] {
            let error = bounded(observer, cx)
                .await
                .expect_err("failed initialization");
            assert_eq!(
                error.downcast_ref::<agent_client_protocol::Error>(),
                Some(&rejection)
            );
            assert!(acp_thread::AuthRequired::from_error(&error).is_some());
        }
        cx.run_until_parked();
        assert!(old_owner.upgrade().is_none());
        assert!(harness.client.sessions.borrow().is_empty());
        assert!(harness.client.session_owners.borrow().is_empty());
        let retry = harness.reopen(true, cx);
        let (_, responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("explicit retry");
        harness.resume_state(acp::StateUpdate::Idle(acp::IdleStateUpdate::new()));
        responder
            .respond(acp::ResumeSessionResponse::new())
            .expect("retry response");
        let thread = bounded(retry, cx).await.expect("fresh owner");
        assert!(thread.read_with(cx, |thread, _| thread.entries().is_empty()));
        assert!(harness.closes.is_empty());
    }

    #[gpui::test]
    async fn v2_resume_sdk_eof_settles_initialization_and_releases_the_owner(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let loading = harness.reopen(true, cx);
        let (_, _responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("pending resume");
        let owner = harness
            .client
            .sessions
            .borrow()
            .get(&acp::SessionId::new("saved"))
            .expect("owner")
            .thread
            .clone();
        drop(harness._agent_task);
        drop(harness.agent);
        let error = bounded(loading, cx).await.expect_err("EOF");
        assert!(
            error
                .downcast_ref::<agent_client_protocol::Error>()
                .is_some_and(agent_client_protocol::is_incoming_transport_closed)
        );
        cx.run_until_parked();
        assert!(owner.upgrade().is_none());
        assert!(harness.client.sessions.borrow().is_empty());
        assert!(harness.client.session_owners.borrow().is_empty());
    }

    #[gpui::test]
    async fn v2_resume_sdk_abandoned_initialization_does_not_retain_the_connection(
        cx: &mut TestAppContext,
    ) {
        let harness = SessionHarness::new(session_capabilities(), vec![], vec![], cx).await;
        let loading = harness.reopen(true, cx);
        let (_, _responder) = bounded(harness.resume_requests.recv(), cx)
            .await
            .expect("pending resume");
        let connection = Rc::downgrade(&harness.client);
        drop(loading);
        cx.run_until_parked();
        cx.update(|_| ());
        drop(harness.client);
        cx.run_until_parked();
        cx.update(|_| ());
        assert!(
            connection.upgrade().is_none(),
            "session initialization must not form an ownership cycle"
        );
    }

    #[gpui::test]
    async fn v2_session_sdk_gates_mcp_and_additional_roots(cx: &mut TestAppContext) {
        let configurations =
            vec![
            ("stdio".to_string(), Arc::new(ContextServerConfiguration::Custom {
                command: serde_json::from_value(json!({
                    "command": session_path("bin/mcp"), "args": ["serve", ""], "env": {"EXAMPLE": "value"}
                })).expect("stdio configuration"), remote: false, launcher: None,
            })),
            ("http".to_string(), Arc::new(ContextServerConfiguration::Http {
                url: "https://example.com/mcp".parse().expect("URL"),
                headers: HashMap::from_iter([("X-Example".into(), "value".into())]),
                timeout: None, oauth: None,
            })),
        ];
        let directories = PathList::new(&[session_path("a"), session_path("b")]);
        for (stdio, http, additional_roots) in [
            (false, false, false),
            (true, false, true),
            (false, true, false),
            (true, true, true),
        ] {
            let mut mcp = acp::McpCapabilities::new();
            if stdio {
                mcp = mcp.stdio(acp::McpStdioCapabilities::new());
            }
            if http {
                mcp = mcp.http(acp::McpHttpCapabilities::new());
            }
            let mut session = acp::SessionCapabilities::new().mcp(mcp);
            if additional_roots {
                session = session
                    .additional_directories(acp::SessionAdditionalDirectoriesCapabilities::new());
            }
            let harness = SessionHarness::new(
                acp::AgentCapabilities::new().session(session),
                Vec::new(),
                Vec::new(),
                cx,
            )
            .await;
            let path_style = harness
                .project
                .read_with(cx, |project, cx| project.path_style(cx));
            let request = harness
                .client
                .session_context(configurations.clone(), &directories, |path| {
                    path_style.is_absolute(path)
                })
                .expect("build request");
            assert_eq!(request.cwd.0, session_path("a"));
            assert_eq!(
                request.additional_directories.len(),
                usize::from(additional_roots)
            );
            assert_eq!(
                request.mcp_servers.len(),
                usize::from(stdio) + usize::from(http)
            );
            for server in request.mcp_servers {
                match server {
                    acp::McpServer::Stdio(server) => {
                        assert!(stdio);
                        assert_eq!(server.command.0, session_path("bin/mcp"));
                        assert_eq!(server.args, ["serve", ""]);
                        assert_eq!(server.env, [acp::EnvVariable::new("EXAMPLE", "value")]);
                    }
                    acp::McpServer::Http(server) => {
                        assert!(http);
                        assert_eq!(server.url, "https://example.com/mcp");
                        assert_eq!(server.headers, [acp::HttpHeader::new("X-Example", "value")]);
                    }
                    _ => panic!("unsupported MCP variant"),
                }
            }
            assert!(
                harness
                    .client
                    .session_context(
                        Vec::new(),
                        &PathList::new::<std::path::PathBuf>(&[]),
                        |path| path_style.is_absolute(path)
                    )
                    .is_err()
            );
            assert!(
                harness
                    .client
                    .session_context(
                        Vec::new(),
                        &PathList::new(&[std::path::Path::new("relative")]),
                        |path| path_style.is_absolute(path),
                    )
                    .is_err()
            );
            if additional_roots {
                let creation = harness.create(cx);
                let (wire, _) = bounded(harness.new_requests.recv(), cx)
                    .await
                    .expect("wire creation");
                assert_eq!(
                    wire.additional_directories,
                    vec![acp::AbsolutePath::new(session_path("b"))]
                );
                harness
                    .new_responses
                    .try_send(Ok(initial_response()))
                    .expect("response");
                let thread = bounded(creation, cx).await.expect("created");
                assert_eq!(
                    thread.read_with(cx, |thread, _| thread
                        .work_dirs()
                        .expect("directories")
                        .ordered_paths()
                        .count()),
                    2
                );
            }
        }
    }

    async fn bounded<T>(future: impl std::future::Future<Output = T>, cx: &TestAppContext) -> T {
        let timeout = cx.background_executor.timer(Duration::from_secs(5));
        futures::pin_mut!(future, timeout);
        match futures::future::select(future, timeout).await {
            futures::future::Either::Left((result, _)) => result,
            futures::future::Either::Right(_) => panic!("timed out during ACP v2 exchange"),
        }
    }

    impl Harness {
        async fn new(response: acp::InitializeResponse, cx: &mut TestAppContext) -> Result<Self> {
            let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
            let logins = Arc::new(Mutex::new(Vec::new()));
            let logouts = Arc::new(Mutex::new(Vec::new()));
            let (prompt_sender, prompts) = async_channel::unbounded();
            let session_cancellations = Arc::new(Mutex::new(Vec::new()));
            let (sender, received) = futures::channel::oneshot::channel();
            let agent_task = cx.background_spawn(
                Agent
                    .v2()
                    .on_receive_request(
                        async move |request: acp::InitializeRequest,
                                    responder: Responder<acp::InitializeResponse>,
                                    _| {
                            assert_eq!(request.protocol_version, ProtocolVersion::V2);
                            assert!(request.capabilities.auth.is_none());
                            let elicitation = request
                                .capabilities
                                .elicitation
                                .expect("elicitation capability");
                            assert!(elicitation.form.is_some() && elicitation.url.is_some());
                            responder.respond(response.clone())
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        {
                            let logins = logins.clone();
                            async move |request: acp::LoginAuthRequest,
                                        responder: Responder<acp::LoginAuthResponse>,
                                        agent: V2ConnectionTo<Client>| {
                                let request_id =
                                    v2_request_id(responder.id()).expect("v2 request ID");
                                let method_id = request.method_id.clone();
                                logins
                                    .lock()
                                    .expect("login log")
                                    .push((request, request_id.clone()));
                                let mode: acp::ElicitationMode = if method_id.0.as_ref() == "url" {
                                    acp::ElicitationUrlMode::new(
                                        acp::ElicitationRequestScope::new(request_id),
                                        "auth-url",
                                        "https://example.com/login",
                                    )
                                    .into()
                                } else {
                                    acp::ElicitationFormMode::new(
                                        acp::ElicitationRequestScope::new(request_id),
                                        acp::ElicitationSchema::new().string("name", true),
                                    )
                                    .into()
                                };
                                let elicitation =
                                    acp::CreateElicitationRequest::new(mode, "Auth input")
                                        .meta(meta("elicitation"));
                                if method_id.0.as_ref() == "early" {
                                    agent.send_request(elicitation).detach();
                                    return responder.respond(
                                        acp::LoginAuthResponse::new().meta(meta("login-response")),
                                    );
                                }
                                // Caller cleanup must work even when the peer ignores auth cancellation.
                                agent.prepare_request(elicitation).on_receiving_result(
                                    async move |result| match result {
                                        Ok(response) => {
                                            assert!(matches!(
                                                response.action,
                                                acp::ElicitationAction::Accept(_)
                                                    | acp::ElicitationAction::Cancel
                                            ));
                                            if matches!(
                                                response.action,
                                                acp::ElicitationAction::Accept(_)
                                            ) {
                                                assert_eq!(
                                                    response.meta,
                                                    Some(meta("elicitation-response"))
                                                );
                                            }
                                            responder.respond(
                                                acp::LoginAuthResponse::new()
                                                    .meta(meta("login-response")),
                                            )
                                        }
                                        Err(error) => responder.respond_with_error(error),
                                    },
                                )?;
                                Ok(())
                            }
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        {
                            let logouts = logouts.clone();
                            async move |request: acp::LogoutAuthRequest,
                                        responder: Responder<acp::LogoutAuthResponse>,
                                        _| {
                                logouts.lock().expect("logout log").push(request);
                                responder.respond(
                                    acp::LogoutAuthResponse::new().meta(meta("logout-response")),
                                )
                            }
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_request(
                        async move |request: acp::PromptRequest, responder, _| {
                            prompt_sender
                                .try_send((request, responder))
                                .expect("prompt request receiver");
                            Ok(())
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .on_receive_notification(
                        {
                            let session_cancellations = session_cancellations.clone();
                            async move |notification: acp::CancelSessionNotification, _| {
                                session_cancellations
                                    .lock()
                                    .expect("session cancellation log")
                                    .push(notification);
                                Ok(())
                            }
                        },
                        agent_client_protocol::on_receive_notification!(),
                    )
                    .connect_with(agent_transport, async move |agent| {
                        assert!(sender.send(agent).is_ok());
                        futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    }),
            );
            let client = bounded(
                AcpV2Connection::connect(
                    AgentId::new("auth-test"),
                    WeakEntity::new_invalid(),
                    client_transport,
                    acp::Implementation::new("client-test", "1"),
                    AcpDebugLog::default(),
                    None,
                    &mut cx.to_async(),
                ),
                cx,
            )
            .await?;
            let agent = received.await.expect("agent handle");
            Ok(Self {
                client,
                agent,
                logins,
                logouts,
                prompts,
                session_cancellations,
                _agent_task: agent_task,
            })
        }

        async fn registered(&self, cx: &mut TestAppContext) {
            let store = self.client.request_elicitations();
            let timeout = cx.background_executor.timer(Duration::from_secs(5));
            let update = store.next_event::<acp_thread::ElicitationStoreEvent>(cx);
            futures::pin_mut!(update, timeout);
            match futures::future::select(update, timeout).await {
                futures::future::Either::Left(_) => {}
                futures::future::Either::Right(_) => {
                    panic!("timed out registering request elicitation")
                }
            }
        }

        fn request_input(
            &self,
            parent_id: acp::RequestId,
            cx: &mut TestAppContext,
        ) -> Task<Result<acp::CreateElicitationResponse, agent_client_protocol::Error>> {
            let request = self.agent.send_request(acp::CreateElicitationRequest::new(
                acp::ElicitationFormMode::new(
                    acp::ElicitationRequestScope::new(parent_id),
                    acp::ElicitationSchema::new(),
                ),
                "Prompt input",
            ));
            cx.background_spawn(request.block_task())
        }
    }

    fn session_agent_response() -> acp::InitializeResponse {
        acp::InitializeResponse::new(
            ProtocolVersion::V2,
            acp::Implementation::new("prompt-test-agent", "1"),
        )
        .capabilities(acp::AgentCapabilities::new().session(acp::SessionCapabilities::new()))
    }

    #[gpui::test]
    async fn v2_prompt_sdk_returns_receipt_without_waiting_for_execution(cx: &mut TestAppContext) {
        let harness = Harness::new(session_agent_response(), cx)
            .await
            .expect("connect");
        let request: acp::PromptRequest = serde_json::from_value(json!({
            "sessionId": "session-a",
            "prompt": [
                {"type": "text", "text": "Question", "_meta": {"scope": "text"}},
                {"type": "resource_link", "uri": "file:///project/context.rs", "name": "Context",
                    "_meta": {"scope": "link"}}
            ],
            "_meta": {"scope": "prompt"}
        }))
        .expect("prompt request");
        let receipt = cx.update(|cx| harness.client.request_prompt_receipt(request.clone(), cx));
        let (received, responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("published prompt");
        assert_eq!(received, request);
        let cancellation = responder.cancellation();
        let input =
            harness.request_input(v2_request_id(responder.id()).expect("v2 request ID"), cx);
        harness.registered(cx).await;
        let expected = acp::PromptResponse::new("accepted-message").meta(meta("receipt"));
        responder
            .respond(expected.clone())
            .expect("accept the prompt");
        assert_eq!(
            bounded(receipt, cx).await.expect("acceptance receipt"),
            expected
        );
        assert_eq!(
            bounded(input, cx)
                .await
                .expect("finish prompt input")
                .action,
            acp::ElicitationAction::Cancel
        );
        assert!(!cancellation.is_cancelled());
        assert!(harness.prompts.is_empty(), "acceptance must not resubmit");
        assert!(
            harness
                .session_cancellations
                .lock()
                .expect("session cancellation log")
                .is_empty()
        );
    }

    #[gpui::test]
    async fn v2_prompt_sdk_requires_sessions_and_preserves_rejections(cx: &mut TestAppContext) {
        let harness = Harness::new(
            acp::InitializeResponse::new(
                ProtocolVersion::V2,
                acp::Implementation::new("no-sessions", "1"),
            ),
            cx,
        )
        .await
        .expect("connect without sessions");
        let receipt = cx.update(|cx| {
            harness.client.request_prompt_receipt(
                acp::PromptRequest::new("missing-session", vec!["Question".into()]),
                cx,
            )
        });
        assert!(bounded(receipt, cx).await.is_err());
        assert!(
            harness.prompts.is_empty(),
            "missing capability must reject before publication"
        );

        let harness = Harness::new(session_agent_response(), cx)
            .await
            .expect("connect");
        let receipt = cx.update(|cx| {
            harness.client.request_prompt_receipt(
                acp::PromptRequest::new("session-b", vec!["Rejected".into()]),
                cx,
            )
        });
        let (request, responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("published prompt");
        assert_eq!(request.session_id, acp::SessionId::new("session-b"));
        let rejection = agent_client_protocol::Error::new(-32000, "Prompt was not inserted")
            .data(json!({"reason": {"retain": [null, 7]}}));
        responder
            .respond_with_error(rejection.clone())
            .expect("reject the prompt");
        let error = bounded(receipt, cx).await.expect_err("rejected prompt");
        assert_eq!(
            error.downcast_ref::<agent_client_protocol::Error>(),
            Some(&rejection)
        );
        assert!(harness.prompts.is_empty(), "rejection must not retry");
    }

    #[gpui::test]
    async fn v2_prompt_sdk_drop_cancels_only_its_rpc_and_eof_settles_waiters(
        cx: &mut TestAppContext,
    ) {
        let harness = Harness::new(session_agent_response(), cx)
            .await
            .expect("connect");
        let unpublished = cx.update(|cx| {
            harness.client.request_prompt_receipt(
                acp::PromptRequest::new("unpublished", vec!["Never sent".into()]),
                cx,
            )
        });
        drop(unpublished);
        cx.run_until_parked();
        assert!(harness.prompts.is_empty());

        let first = cx.update(|cx| {
            harness
                .client
                .request_prompt_receipt(acp::PromptRequest::new("first", vec!["First".into()]), cx)
        });
        let (_, first_responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("first prompt");
        let first_cancellation = first_responder.cancellation();
        let first_id = v2_request_id(first_responder.id()).expect("v2 request ID");
        let first_input = harness.request_input(first_id.clone(), cx);
        harness.registered(cx).await;
        let second = cx.update(|cx| {
            harness.client.request_prompt_receipt(
                acp::PromptRequest::new("second", vec!["Second".into()]),
                cx,
            )
        });
        let (_, second_responder) = bounded(harness.prompts.recv(), cx)
            .await
            .expect("second prompt");
        let second_cancellation = second_responder.cancellation();
        let second_input = harness.request_input(
            v2_request_id(second_responder.id()).expect("v2 request ID"),
            cx,
        );
        harness.registered(cx).await;
        drop(first);
        bounded(first_cancellation.cancelled(), cx).await;
        assert_eq!(
            bounded(first_input, cx)
                .await
                .expect("cancel first input")
                .action,
            acp::ElicitationAction::Cancel
        );
        assert!(!second_cancellation.is_cancelled());
        let store = harness.client.request_elicitations();
        store.read_with(cx, |store, _| {
            assert_eq!(
                store
                    .elicitations()
                    .iter()
                    .filter(|entry| matches!(entry.status, ElicitationStatus::Pending { .. }))
                    .count(),
                1
            );
        });
        let late_input = harness.request_input(first_id, cx);
        assert_eq!(
            bounded(late_input, cx)
                .await
                .expect_err("closed prompt scope")
                .code,
            ErrorCode::InvalidParams
        );
        first_responder
            .respond(acp::PromptResponse::new("late-acceptance"))
            .expect("late acceptance");
        drop(harness._agent_task);
        drop(harness.agent);
        let error = bounded(second, cx)
            .await
            .expect_err("EOF must settle the pending RPC");
        assert!(
            error
                .downcast_ref::<agent_client_protocol::Error>()
                .is_some()
        );
        bounded(harness.client.connection.incoming_closed(), cx).await;
        bounded(drain_foreground_queue(&harness.client.dispatch_tx), cx)
            .await
            .expect("disconnect cleanup");
        assert!(bounded(second_input, cx).await.is_err());
        store.read_with(cx, |store, _| {
            assert!(
                store
                    .elicitations()
                    .iter()
                    .all(|entry| matches!(entry.status, ElicitationStatus::Canceled))
            );
        });
        assert!(harness.prompts.is_empty(), "disconnect must not resend");
        assert!(
            harness
                .session_cancellations
                .lock()
                .expect("session cancellation log")
                .is_empty()
        );
    }

    #[gpui::test]
    async fn v2_auth_sdk_preserves_offers_and_serves_elicitation_before_sessions(
        cx: &mut TestAppContext,
    ) {
        let offered: acp::InitializeResponse = serde_json::from_value(json!({
            "protocolVersion": 2, "info": {"name": "auth-agent", "version": "1",
                "_meta": {"scope": "info"}},
            "capabilities": {"session": {"_meta": {"scope": "session"}}, "_meta": {"scope": "caps"}},
            "authMethods": [
                {"type": "agent", "methodId": "login", "name": "Login",
                    "_meta": {"scope": "method", "terminal-auth": {"command": "not-executable"}}},
                {"type": "terminal", "methodId": "terminal", "name": "Terminal", "args": ["auth"]},
                {"type": "_future", "methodId": "future", "name": "Future",
                    "payload": {"retain": [null, 7]}}
            ], "_meta": {"scope": "initialize"}
        })).expect("offered initialization");
        let harness = Harness::new(offered.clone(), cx).await.expect("connect v2");
        assert_eq!(harness.client.initialize_response(), &offered);
        assert!(
            harness.client.supports_logout(),
            "auth methods, not capabilities.auth, enable logout"
        );
        assert!(
            cx.read(|cx| harness.client.terminal_auth_task(&"login".into(), cx))
                .is_none()
        );
        let login = cx.update(|cx| {
            harness.client.login(
                acp::LoginAuthRequest::new("login").meta(meta("login-request")),
                cx,
            )
        });
        harness.registered(cx).await;
        let store = harness.client.request_elicitations();
        let entry_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!("one auth elicitation");
            };
            assert_eq!(elicitation.request.meta, Some(meta("elicitation")));
            let acp::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("request scope without a session");
            };
            let logins = harness.logins.lock().expect("login log");
            assert_eq!(scope.request_id, logins.first().expect("login").1);
            assert_eq!(
                logins.first().expect("login").0.meta,
                Some(meta("login-request"))
            );
            elicitation.id.clone()
        });
        // Another RPC must be handled while login is waiting for foreground user input.
        let logout = cx.update(|cx| {
            harness.client.logout(
                acp::LogoutAuthRequest::new().meta(meta("logout-request")),
                cx,
            )
        });
        assert_eq!(
            bounded(logout, cx).await.expect("logout").meta,
            Some(meta("logout-response"))
        );
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &entry_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new().content(std::collections::BTreeMap::from(
                        [("name".to_owned(), acp::ElicitationContentValue::from("Ada"))],
                    )),
                ))
                .meta(meta("elicitation-response")),
                cx,
            )
        });
        assert_eq!(
            bounded(login, cx).await.expect("login").meta,
            Some(meta("login-response"))
        );
        for method_id in ["terminal", "future", "missing"] {
            let login = cx.update(|cx| {
                harness
                    .client
                    .login(acp::LoginAuthRequest::new(method_id), cx)
            });
            assert!(bounded(login, cx).await.is_err());
        }
        assert_eq!(harness.logins.lock().expect("login log").len(), 1);
        assert_eq!(
            harness
                .logouts
                .lock()
                .expect("logout log")
                .first()
                .expect("logout")
                .meta,
            Some(meta("logout-request"))
        );
    }

    #[gpui::test]
    async fn v2_auth_sdk_caller_drop_and_eof_cleanup_only_their_owned_entries(
        cx: &mut TestAppContext,
    ) {
        let offered = acp::InitializeResponse::new(
            ProtocolVersion::V2,
            acp::Implementation::new("agent", "1"),
        )
        .auth_methods(vec![acp::AuthMethod::Agent(acp::AuthMethodAgent::new(
            "login", "Login",
        ))]);
        let harness = Harness::new(offered, cx).await.expect("connect");
        let unpublished_login =
            cx.update(|cx| AgentConnection::authenticate(&harness.client, "login".into(), cx));
        let unpublished_logout = cx.update(|cx| AgentConnection::logout(&harness.client, cx));
        drop(unpublished_login);
        drop(unpublished_logout);
        cx.run_until_parked();
        assert!(harness.logins.lock().expect("login log").is_empty());
        assert!(harness.logouts.lock().expect("logout log").is_empty());
        let first =
            cx.update(|cx| AgentConnection::authenticate(&harness.client, "login".into(), cx));
        harness.registered(cx).await;
        let second =
            cx.update(|cx| AgentConnection::authenticate(&harness.client, "login".into(), cx));
        harness.registered(cx).await;
        let store = harness.client.request_elicitations();
        let unrelated = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(999)),
                            acp::ElicitationSchema::new(),
                        ),
                        "No active auth waiter",
                    ),
                    cx,
                )
                .expect("unrelated pending elicitation")
        });
        let (first_id, second_id) = store.read_with(cx, |store, _| {
            let [first, second, _] = store.elicitations() else {
                panic!("two auth entries and independent feedback");
            };
            (first.id.clone(), second.id.clone())
        });
        drop(first);
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(matches!(
                store.elicitation(&first_id).expect("first").1.status,
                ElicitationStatus::Canceled
            ));
            assert!(matches!(
                store.elicitation(&second_id).expect("second").1.status,
                ElicitationStatus::Pending { .. }
            ));
        });
        let cancelled_parent = harness
            .logins
            .lock()
            .expect("login log")
            .first()
            .expect("cancelled login")
            .1
            .clone();
        let late_request = harness
            .agent
            .send_request(acp::CreateElicitationRequest::new(
                acp::ElicitationFormMode::new(
                    acp::ElicitationRequestScope::new(cancelled_parent.clone()),
                    acp::ElicitationSchema::new(),
                ),
                "Late input for a cancelled auth request",
            ));
        let late_response = cx.background_spawn(late_request.block_task());
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(
                !store.elicitations().iter().any(|entry| {
                    matches!(entry.request.scope(), acp::ElicitationScope::Request(scope)
                        if scope.request_id == cancelled_parent)
                        && matches!(entry.status, ElicitationStatus::Pending { .. })
                }),
                "cancelled auth must not publish fresh pending input"
            );
        });
        assert_eq!(
            bounded(late_response, cx)
                .await
                .expect_err("closed parent scope")
                .code,
            ErrorCode::InvalidParams
        );
        // The server goes away while the client and its displayed store are retained.
        drop(harness._agent_task);
        drop(harness.agent);
        let result = bounded(second, cx).await;
        assert!(result.is_err());
        bounded(harness.client.connection.incoming_closed(), cx).await;
        bounded(drain_foreground_queue(&harness.client.dispatch_tx), cx)
            .await
            .expect("foreground cleanup barrier");
        assert_eq!(
            bounded(unrelated, cx).await.action,
            acp::ElicitationAction::Cancel
        );
        store.read_with(cx, |store, _| {
            assert!(
                store
                    .elicitations()
                    .iter()
                    .all(|entry| matches!(entry.status, ElicitationStatus::Canceled))
            );
        });
    }

    #[gpui::test]
    async fn v2_auth_sdk_success_finishes_pending_input_but_preserves_url_completion(
        cx: &mut TestAppContext,
    ) {
        let offered = acp::InitializeResponse::new(
            ProtocolVersion::V2,
            acp::Implementation::new("agent", "1"),
        )
        .auth_methods(vec![
            acp::AuthMethod::Agent(acp::AuthMethodAgent::new("early", "Early")),
            acp::AuthMethod::Agent(acp::AuthMethodAgent::new("url", "URL")),
        ]);
        let harness = Harness::new(offered, cx).await.expect("connect");
        let early = cx.update(|cx| {
            harness
                .client
                .login(acp::LoginAuthRequest::new("early"), cx)
        });
        bounded(early, cx).await.expect("early parent success");
        cx.run_until_parked();
        let store = harness.client.request_elicitations();
        store.read_with(cx, |store, _| {
            let [entry] = store.elicitations() else {
                panic!("one unanswered auth input");
            };
            assert!(matches!(entry.status, ElicitationStatus::Canceled));
        });
        let login = cx.update(|cx| harness.client.login(acp::LoginAuthRequest::new("url"), cx));
        harness.registered(cx).await;
        let entry_id = store.read_with(cx, |store, _| {
            store.elicitations().last().expect("URL entry").id.clone()
        });
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &entry_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                ))
                .meta(meta("elicitation-response")),
                cx,
            );
        });
        bounded(login, cx).await.expect("URL parent success");
        store.read_with(cx, |store, _| {
            assert!(matches!(
                store.elicitation(&entry_id).expect("URL entry").1.status,
                ElicitationStatus::Accepted
            ));
        });
        let parent_id = harness
            .logins
            .lock()
            .expect("login log")
            .last()
            .expect("URL login")
            .1
            .clone();
        let late = harness
            .agent
            .send_request(acp::CreateElicitationRequest::new(
                acp::ElicitationFormMode::new(
                    acp::ElicitationRequestScope::new(parent_id),
                    acp::ElicitationSchema::new(),
                ),
                "Input after parent success",
            ));
        let late = cx.background_spawn(late.block_task());
        assert_eq!(
            bounded(late, cx)
                .await
                .expect_err("ended parent scope")
                .code,
            ErrorCode::InvalidParams
        );
        harness
            .agent
            .send_notification(acp::CompleteElicitationNotification::new("auth-url"))
            .expect("completion after parent success");
        cx.run_until_parked();
        store.read_with(cx, |store, _| {
            assert!(matches!(
                store.elicitation(&entry_id).expect("URL entry").1.status,
                ElicitationStatus::Completed
            ));
        });
    }

    #[gpui::test]
    async fn v2_auth_sdk_missing_methods_do_not_enable_auth_and_v1_is_rejected(
        cx: &mut TestAppContext,
    ) {
        let offered = acp::InitializeResponse::new(
            ProtocolVersion::V2,
            acp::Implementation::new("agent", "1"),
        )
        .capabilities(acp::AgentCapabilities::new().auth(acp::AgentAuthCapabilities::new()));
        let harness = Harness::new(offered, cx).await.expect("connect");
        assert!(!harness.client.supports_logout());
        let login = cx.update(|cx| {
            harness
                .client
                .login(acp::LoginAuthRequest::new("missing"), cx)
        });
        let logout = cx.update(|cx| harness.client.logout(acp::LogoutAuthRequest::new(), cx));
        assert!(bounded(login, cx).await.is_err());
        assert!(bounded(logout, cx).await.is_err());
        assert!(harness.logins.lock().expect("login log").is_empty());
        assert!(harness.logouts.lock().expect("logout log").is_empty());
        let store = harness.client.request_elicitations();
        let pending = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(999)),
                            acp::ElicitationSchema::new(),
                        ),
                        "Retained across connection destruction",
                    ),
                    cx,
                )
                .expect("retained pending entry")
        });
        drop(harness.client);
        assert_eq!(
            bounded(pending, cx).await.action,
            acp::ElicitationAction::Cancel
        );
        store.read_with(cx, |store, _| {
            assert!(
                store
                    .elicitations()
                    .iter()
                    .all(|entry| matches!(entry.status, ElicitationStatus::Canceled))
            );
        });
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        // A v2 SDK server rewrites replies to its negotiated version; use a real v1 peer.
        let _agent_task = cx.background_spawn(
            Agent
                .builder()
                .on_receive_request(
                    async move |_request: acp_v1::InitializeRequest,
                                responder: Responder<acp_v1::InitializeResponse>,
                                _| {
                        responder.respond(acp_v1::InitializeResponse::new(ProtocolVersion::V1))
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_with(agent_transport, async move |_agent| {
                    futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                }),
        );
        let error = bounded(
            AcpV2Connection::connect(
                AgentId::new("v1-only"),
                WeakEntity::new_invalid(),
                client_transport,
                acp::Implementation::new("client", "1"),
                AcpDebugLog::default(),
                None,
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .err()
        .expect("reject unimplemented fallback");
        assert!(error.is::<UnsupportedVersion>(), "{error:?}");
        assert!(
            error.is::<agent_client_protocol::InitializationVersionMismatch>(),
            "{error:?}"
        );
    }

    #[gpui::test]
    async fn v2_auth_sdk_terminal_methods_reuse_the_configured_invocation(cx: &mut TestAppContext) {
        use project::agent_server_store::AllAgentServersSettings;
        use settings::Settings as _;
        cx.update(|cx| {
            let settings = settings::SettingsStore::test(cx);
            cx.set_global(settings);
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::from_iter([(
                    "terminal-test".to_owned(),
                    settings::CustomAgentServerSettings::Custom {
                        path: "/bin/sh".into(),
                        args: vec!["--base".into()],
                        env: HashMap::from_iter([("BASE".into(), "1".into())]),
                        default_mode: None,
                        default_config_options: HashMap::default(),
                        favorite_config_option_values: HashMap::default(),
                    }
                    .into(),
                )])),
                cx,
            );
        });
        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let store = project.read_with(cx, |project, _| project.agent_server_store().downgrade());
        let method: acp::AuthMethod = serde_json::from_value(json!({
            "type": "terminal", "methodId": "terminal", "name": "Interactive login",
            "args": ["auth", ""], "env": [{"name": "EXTRA", "value": "2",
                "_meta": {"retain": [null, 7]}}], "_meta": {"scope": "terminal"}
        }))
        .expect("terminal method");
        let expected = method.clone();
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        let _agent_task = cx.background_spawn(
            Agent
                .v2()
                .on_receive_request(
                    async move |request: acp::InitializeRequest,
                                responder: Responder<acp::InitializeResponse>,
                                _| {
                        assert!(
                            request
                                .capabilities
                                .auth
                                .as_ref()
                                .and_then(|auth| auth.terminal.as_ref())
                                .is_some()
                        );
                        responder.respond(
                            acp::InitializeResponse::new(
                                ProtocolVersion::V2,
                                acp::Implementation::new("terminal-agent", "1"),
                            )
                            .auth_methods(vec![method.clone()]),
                        )
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_with(agent_transport, async move |_agent| {
                    futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                }),
        );
        let client = bounded(
            AcpV2Connection::connect(
                AgentId::new("terminal-test"),
                store,
                client_transport,
                acp::Implementation::new("test", "1"),
                AcpDebugLog::default(),
                None,
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .expect("connect terminal-capable client");
        assert_eq!(client.initialize_response.auth_methods, [expected]);
        assert!(client.supports_logout());
        let login = cx.update(|cx| client.login(acp::LoginAuthRequest::new("terminal"), cx));
        assert!(bounded(login, cx).await.is_err());
        let task = cx
            .read(|cx| client.terminal_auth_task(&"terminal".into(), cx))
            .expect("separate terminal route");
        let task = bounded(task, cx)
            .await
            .expect("configured terminal invocation");
        assert_eq!(task.command.as_deref(), Some("/bin/sh"));
        assert_eq!(task.args, ["--base", "auth", ""]);
        assert_eq!(task.env.get("BASE").map(String::as_str), Some("1"));
        assert_eq!(task.env.get("EXTRA").map(String::as_str), Some("2"));
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_stdio_shutdown_terminates_a_retained_connection(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        for quitting in [false, true] {
            let client = bounded(
            AcpV2Connection::stdio(
                AgentId::new("shutdown-test"),
                project.clone(),
                AgentServerCommand {
                    path: "/bin/sh".into(),
                    args: vec![
                        "-c".into(),
                        r#"
while IFS= read -r request; do
    id=${request#*\"id\":}
    id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":2,"info":{"name":"shutdown-agent","version":"1"}}}\n' "$id"
            ;;
    esac
done
exec sleep 60
"#
                        .into(),
                    ],
                    env: None,
                },
                WeakEntity::new_invalid(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .expect("initialize stdio");
            let status = client
                .runtime
                .borrow_mut()
                .as_mut()
                .expect("live runtime")
                ._child
                .as_mut()
                .expect("owned process")
                .0
                .status();
            let pending = cx.background_spawn(
                client
                    .connection
                    .send_request(acp::LogoutAuthRequest::new())
                    .block_task(),
            );
            cx.run_until_parked();
            if quitting {
                cx.update(|cx| cx.shutdown());
            } else {
                client.shutdown();
            }
            client.shutdown();
            let status = bounded(status, cx)
                .await
                .expect("wait for terminated agent");
            use std::os::unix::process::ExitStatusExt as _;
            assert_eq!(status.signal(), Some(9));
            assert!(
                bounded(pending, cx).await.is_err(),
                "shutdown settles pending RPCs"
            );
            assert!(client.runtime.borrow().is_none());
        }
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_shutdown_terminates_a_process_during_initialization(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let mut command = std::process::Command::new("/bin/sh");
        command.args(["-c", "IFS= read -r request"]);
        let child = util::process::Child::spawn(
            command,
            std::process::Stdio::piped(),
            std::process::Stdio::null(),
            std::process::Stdio::null(),
        )
        .expect("spawn waiting child");
        let mut child = transport::OwnedChild(child);
        // status() closes the child's stdin; keep it open so only shutdown ends the process.
        let stdin = child.0.stdin.take().expect("child stdin");
        let status = child.0.status();
        let (transport, _peer) = agent_client_protocol::Channel::duplex();
        let connecting = cx.spawn(async move |mut cx| {
            AcpV2Connection::connect(
                AgentId::new("pending-shutdown"),
                WeakEntity::new_invalid(),
                transport,
                acp::Implementation::new("client", "1"),
                AcpDebugLog::default(),
                Some(StdioStartup {
                    child,
                    stderr_task: Task::ready(()).shared(),
                    runtime: Rc::new(RefCell::new(None)),
                    drain: None,
                }),
                &mut cx,
            )
            .await
        });
        cx.run_until_parked();
        cx.update(|cx| cx.shutdown());
        let status = bounded(status, cx)
            .await
            .expect("process terminated during startup");
        use std::os::unix::process::ExitStatusExt as _;
        assert_eq!(status.signal(), Some(9));
        drop(stdin);
        assert!(
            bounded(connecting, cx).await.is_err(),
            "initialization failed on shutdown"
        );
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_stdio_startup_preserves_rejections_before_cleanup(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        for (response, expected) in [
            (
                r#""error":{"code":-32022,"message":"Unsupported protocol version","data":{"requested":2,"supported":[1]}}"#,
                Some(
                    agent_client_protocol::Error::new(-32022, "Unsupported protocol version")
                        .data(json!({"requested": 2, "supported": [1]})),
                ),
            ),
            (
                r#""error":{"code":-32602,"message":"Invalid initialize parameters","data":{"field":"capabilities"}}"#,
                Some(
                    agent_client_protocol::Error::invalid_params()
                        .data(json!({"field": "capabilities"})),
                ),
            ),
            (
                r#""result":{"protocolVersion":1,"info":{"name":"v1-peer","version":"1"}}"#,
                None,
            ),
        ] {
            // Linux pidfd polling needs stable task wakers, not the test root's tracing wakers.
            let connecting = cx.spawn({
                let project = project.clone();
                async move |mut cx| {
                    AcpV2Connection::stdio(
                        AgentId::new("startup-rejection"),
                        project,
                        AgentServerCommand {
                            path: "/bin/sh".into(),
                            args: vec![
                                "-c".into(),
                                r#"
while IFS= read -r request; do
    case "$request" in
        *'"method":"initialize"'*)
            id=${request#*\"id\":}
            id=${id%%,*}
            printf '{"jsonrpc":"2.0","id":%s,%s}\n' "$id" "$ACP_STARTUP_RESPONSE"
            ;;
    esac
done
exec sleep 60
"#
                                .into(),
                            ],
                            env: Some(HashMap::from_iter([(
                                "ACP_STARTUP_RESPONSE".into(),
                                response.into(),
                            )])),
                        },
                        WeakEntity::new_invalid(),
                        &mut cx,
                    )
                    .await
                }
            });
            let error = bounded(connecting, cx)
                .await
                .err()
                .expect("startup rejection");
            if let Some(expected) = expected {
                let actual = error
                    .downcast_ref::<agent_client_protocol::Error>()
                    .unwrap_or_else(|| panic!("startup diagnostic was replaced: {error:?}"));
                assert_eq!(actual.code, expected.code);
                assert_eq!(actual.data, expected.data);
            } else {
                assert!(error.is::<UnsupportedVersion>(), "{error:?}");
                let mismatch = error
                    .downcast_ref::<agent_client_protocol::InitializationVersionMismatch>()
                    .unwrap_or_else(|| panic!("version diagnostic was replaced: {error:?}"));
                assert_eq!(mismatch.requested, ProtocolVersion::V2);
                assert_eq!(mismatch.selected, ProtocolVersion::V1);
            }
        }
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn v2_auth_stdio_uses_one_launcher_and_preserves_errors_and_exit_stderr(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let settings = settings::SettingsStore::test(cx);
            cx.set_global(settings);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("working directory");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec!["-c".into(), r#"
while IFS= read -r request; do
    id=${request#*\"id\":}
    id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*)
            case "$request" in *'"protocolVersion":2'*) ;; *) exit 42 ;; esac
            printf '{"jsonrpc":"2.0","id":%s,"result":{"protocolVersion":2,"info":{"name":"stdio-agent","version":"1"},"authMethods":[{"type":"agent","methodId":"login","name":"Login"}]}}\n' "$id"
            ;;
        *'"method":"auth/login"'*)
            printf '{"jsonrpc":"2.0","id":%s,"error":{"code":-32602,"message":"login rejected","data":{"scope":"wire-error"}}}\n' "$id"
            ;;
        *'"method":"auth/logout"'*)
            printf '{"jsonrpc":"2.0","id":%s,"result":{"_meta":{"scope":"logout-response"}}}\n' "$id"
            printf 'foundation final stderr\n' >&2
            exit 0
            ;;
        *) exit 43 ;;
    esac
done
"#.into()],
            env: None,
        };
        let client = bounded(
            AcpV2Connection::stdio(
                AgentId::new("stdio-test"),
                project.clone(),
                command,
                WeakEntity::new_invalid(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .expect("initialize scripted stdio");
        assert_eq!(client.initialize_response.info.name, "stdio-agent");
        let login = cx.update(|cx| client.login(acp::LoginAuthRequest::new("login"), cx));
        let error = bounded(login, cx).await.expect_err("wire login rejection");
        let error = error
            .downcast_ref::<agent_client_protocol::Error>()
            .expect("runtime error");
        assert_eq!(error.code, ErrorCode::InvalidParams);
        assert_eq!(error.data, Some(json!({"scope": "wire-error"})));
        let logout = cx.update(|cx| client.logout(acp::LogoutAuthRequest::new(), cx));
        assert_eq!(
            bounded(logout, cx).await.expect("logout").meta,
            Some(meta("logout-response"))
        );
        bounded(client.connection.incoming_closed(), cx).await;
        let (messages, _receiver) = client.subscribe_debug_messages();
        for method_name in ["initialize", "auth/login", "auth/logout"] {
            assert_eq!(messages.iter().filter(|message| {
                matches!(&message.message, super::super::AcpDebugMessageContent::Request { method, .. }
                    if method.as_ref() == method_name)
            }).count(), 1, "one wire request per method");
        }
        drop(client);

        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec![
                "-c".into(),
                "printf 'initialize exit stderr\\n' >&2; exit 7".into(),
            ],
            env: None,
        };
        let error = bounded(
            AcpV2Connection::stdio(
                AgentId::new("early-exit"),
                project,
                command,
                WeakEntity::new_invalid(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .err()
        .expect("initialization EOF");
        let acp_thread::LoadError::Exited { status, stderr } = error
            .downcast_ref::<acp_thread::LoadError>()
            .expect("process exit error")
        else {
            panic!("exited error");
        };
        assert_eq!(status.code(), Some(7));
        assert_eq!(stderr.as_deref(), Some("initialize exit stderr"));
    }
}
