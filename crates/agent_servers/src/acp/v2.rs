use super::{
    AcpDebugLog, AcpDebugMessage, AcpSession, ClientContext, ConfigOptions,
    ContextServerConfiguration, ForegroundWork, UnsupportedVersion, drain_foreground_queue,
    enqueue_notification, first_class_terminal_auth_task, map_acp_error,
    mcp_configurations_for_project, runtime, transport, with_v2_client_ingress,
};
use acp_thread::{
    AcpThread, AgentConnection, AgentSessionConfigOptions, ElicitationStore,
    ReceiptSessionSubmissions, ReceiptSubmission, SubmissionDisconnected,
};
use action_log::ActionLog;
use agent_client_protocol::{
    Agent, Client, ConnectTo, JsonRpcRequest, Lines, V2ConnectionTo,
    schema::{ProtocolVersion, v2 as acp},
};
use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use futures::{
    FutureExt as _,
    channel::mpsc,
    future::{LocalBoxFuture, Shared},
};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, SharedString, Task, WeakEntity};
use project::{AgentId, AgentServerStore, Project, agent_server_store::AgentServerCommand};
use std::{any::Any, cell::RefCell, rc::Rc, sync::Arc};
use task::SpawnInTerminal;
use util::ResultExt as _;
use util::path_list::PathList;

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

/// An unselected v2 backend with auth, initial sessions/config and receipt-driven prompts.
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
    _runtime: runtime::ClientRuntime,
    _stderr_task: Shared<Task<()>>,
    _wait_task: Option<Task<()>>,
    _child: Option<transport::OwnedChild>,
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
        let initialized = {
            let mut cx = cx.clone();
            let debug_log = debug_log.clone();
            let stderr_task = stderr_task.clone();
            async move {
                Self::connect(
                    agent_id,
                    agent_server_store,
                    Lines::new(outgoing, incoming),
                    info,
                    debug_log,
                    stderr_task,
                    &mut cx,
                )
                .await
            }
        }
        .boxed_local();
        let status = child
            .0
            .status()
            .map(|result| {
                result.map_err(|error| anyhow!("failed to wait for agent server exit: {error}"))
            })
            .boxed_local();
        let (mut connection, status, stderr_task) =
            runtime::race_startup(initialized, status, stderr_task, &debug_log, cx).await?;
        let drained = {
            let sdk_connection = connection.connection.clone();
            runtime::drain_transport(
                async move { sdk_connection.incoming_closed().await },
                connection.dispatch_tx.clone(),
                stderr_task,
            )
        };
        connection._wait_task = Some(runtime::watch_process(
            status,
            drained,
            debug_log,
            |error, _cx| {
                log::warn!("ACP v2 agent exited: {error}");
            },
            cx,
        ));
        connection._child = Some(child);
        Ok(connection)
    }

    async fn connect(
        id: AgentId,
        agent_server_store: WeakEntity<AgentServerStore>,
        transport: impl ConnectTo<Client> + 'static,
        info: acp::Implementation,
        debug_log: AcpDebugLog,
        stderr_task: Shared<Task<()>>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let request_elicitations = cx.new(|_| ElicitationStore::default());
        let sessions = Rc::new(RefCell::new(HashMap::default()));
        let context = ClientContext {
            sessions: sessions.clone(),
            session_list: Rc::new(RefCell::new(None)),
            request_elicitations: request_elicitations.clone(),
            active_request_scopes: Some(RefCell::new(Default::default())),
        };
        let (mut runtime, shutdown) = runtime::ClientRuntime::new(context, cx);
        let dispatch_tx = runtime.sender();
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
        runtime.start_io(runtime::finish_connection(connection_future, cleanup), cx);
        let connection = received
            .await
            .context("Failed to receive ACP v2 connection handle")?;
        let supports_terminal_auth = agent_server_store.upgrade().is_some_and(|store| {
            store.update(cx, |store, _| store.get_external_agent(&id).is_some())
        });
        let mut capabilities = acp::ClientCapabilities::new().elicitation(
            acp::ElicitationCapabilities::new()
                .form(acp::ElicitationFormCapabilities::new())
                .url(acp::ElicitationUrlCapabilities::new()),
        );
        if supports_terminal_auth {
            capabilities = capabilities
                .auth(acp::AuthCapabilities::new().terminal(acp::TerminalAuthCapabilities::new()));
        }
        let initialize_request =
            acp::InitializeRequest::new(ProtocolVersion::V2, info).capabilities(capabilities);
        let initialize_response = connection
            .send_request(initialize_request.clone())
            .block_task()
            .await
            .map_err(map_acp_error)?;
        if initialize_response.protocol_version != ProtocolVersion::V2 {
            return Err(UnsupportedVersion.into());
        }
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
            _runtime: runtime,
            _stderr_task: stderr_task,
            _wait_task: None,
            _child: None,
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
        if !self.initialize_response.auth_methods.iter().any(|method| {
            matches!(method, acp::AuthMethod::Agent(agent) if agent.method_id == request.method_id)
        }) {
            return Task::ready(Err(anyhow!(
                "Authentication method is not supported for agent-managed login"
            )));
        }
        self.scoped_request(request, cx)
    }

    pub fn supports_logout(&self) -> bool {
        !self.initialize_response.auth_methods.is_empty()
    }

    pub fn logout(
        &self,
        request: acp::LogoutAuthRequest,
        cx: &mut App,
    ) -> Task<Result<acp::LogoutAuthResponse>> {
        if !self.supports_logout() {
            return Task::ready(Err(anyhow!("Logout is not advertised by this agent")));
        }
        self.scoped_request(request, cx)
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
        &self,
        request: Request,
        cx: &mut App,
    ) -> Task<Result<Request::Response>> {
        let connection = self.connection.clone();
        let dispatch_tx = self.dispatch_tx.clone();
        cx.foreground_executor().spawn(async move {
            let request = connection.prepare_request(request);
            let request_id = request.id().clone();
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
        })
    }

    fn new_session_request(
        &self,
        configurations: Vec<(String, Arc<ContextServerConfiguration>)>,
        work_dirs: &PathList,
        is_absolute: impl Fn(&std::path::Path) -> bool,
    ) -> Result<acp::NewSessionRequest> {
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
        Ok(acp::NewSessionRequest::new(cwd)
            .additional_directories(additional_directories)
            .mcp_servers(servers))
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
        let connection = connection.clone();
        let dispatch_tx = dispatch_tx.clone();
        let (applied, application_finished) = futures::channel::oneshot::channel();
        let response = async move {
            preflight()?;
            let (sender, received) = futures::channel::oneshot::channel();
            let request = connection.prepare_request(request);
            let request_id = request.id().clone();
            let cancellation = request.cancellation_handle();
            enqueue_notification(&dispatch_tx, request_id.clone(), register_request);
            let published = request.on_receiving_result({
                let connection = connection.clone();
                let dispatch_tx = dispatch_tx.clone();
                let request_id = request_id.clone();
                async move |result: Result<Request::Response, agent_client_protocol::Error>| {
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
                        // Register cleanup, but never await later traffic in an ordered callback.
                        connection
                            .prepare_request(acp::CloseSessionRequest::new(session_id))
                            .on_receiving_result(async move |result| {
                                result.log_err();
                                Ok(())
                            })
                            .log_err();
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
        let request = match self.new_session_request(
            mcp_configurations_for_project(&project, cx),
            &work_dirs,
            |path| path_style.is_absolute(path),
        ) {
            Ok(request) => request,
            Err(error) => return Task::ready(Err(error)),
        };
        let work_dirs = PathList::new(
            &std::iter::once(request.cwd.0.clone())
                .chain(
                    request
                        .additional_directories
                        .iter()
                        .map(|path| path.0.clone()),
                )
                .collect::<Vec<_>>(),
        );
        let response = Self::ordered_request(
            &self.connection,
            &self.dispatch_tx,
            request,
            || Ok(()),
            |response| Some(response.session_id.clone()),
            {
                let this = self.clone();
                move |response, cx| {
                    anyhow::ensure!(
                        !this.sessions.borrow().contains_key(&response.session_id),
                        "Agent returned a session ID that is already registered"
                    );
                    let options =
                        ConfigOptions::new(Rc::new(RefCell::new(response.config_options.clone())));
                    let action_log = cx.new(|_| ActionLog::new(project.clone()));
                    let session_id = response.session_id.clone();
                    let capabilities = this
                        .initialize_response
                        .capabilities
                        .session
                        .as_ref()
                        .and_then(|session| session.prompt.clone())
                        .unwrap_or_default();
                    let thread = cx.new(|cx| {
                        AcpThread::new(
                            None,
                            None,
                            Some(work_dirs),
                            this.clone(),
                            project,
                            action_log,
                            session_id.clone(),
                            watch::Receiver::constant(capabilities),
                            cx,
                        )
                    });
                    let release_subscription = cx.update({
                        let this = Rc::downgrade(&this);
                        let session_id = session_id.clone();
                        let thread_id = thread.entity_id();
                        let thread = thread.clone();
                        move |cx| {
                            cx.observe_release(&thread, move |_, _cx| {
                                if let Some(this) = this.upgrade() {
                                    let matches =
                                        this.sessions.borrow().get(&session_id).is_some_and(
                                            |session| session.thread.entity_id() == thread_id,
                                        );
                                    if matches {
                                        this.sessions.borrow_mut().remove(&session_id);
                                    }
                                }
                            })
                        }
                    });
                    this.sessions.borrow_mut().insert(
                        session_id,
                        AcpSession {
                            thread: thread.downgrade(),
                            suppress_abort_err: false,
                            session_modes: None,
                            config_options: Some(options),
                            default_config_task: None,
                            _release_subscription: release_subscription,
                        },
                    );
                    thread.update(cx, |thread, cx| {
                        thread.update_available_commands(response.available_commands, cx);
                        cx.emit(acp_thread::AcpThreadEvent::ConfigOptionsUpdated(
                            response.config_options,
                        ));
                    });
                    Ok(thread)
                }
            },
            cx,
        );
        cx.foreground_executor().spawn(response)
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
        let login = self.login(acp::LoginAuthRequest::new(method), cx);
        cx.foreground_executor().spawn(async move {
            login.await?;
            Ok(())
        })
    }

    fn supports_logout(&self) -> bool {
        AcpV2Connection::supports_logout(self)
    }

    fn logout(&self, cx: &mut App) -> Task<Result<()>> {
        let logout = AcpV2Connection::logout(self, acp::LogoutAuthRequest::new(), cx);
        cx.foreground_executor().spawn(async move {
            logout.await?;
            Ok(())
        })
    }

    fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {
        log::warn!("Session cancellation is not yet implemented by the inactive ACP v2 backend");
    }

    fn request_elicitations(&self) -> Option<Entity<ElicitationStore>> {
        Some(self.request_elicitations.clone())
    }

    fn session_config_options(
        &self,
        session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn AgentSessionConfigOptions>> {
        let sessions = self.sessions.borrow();
        let session = sessions.get(session_id)?;
        Some(Rc::new(V2SessionConfigOptions {
            session_id: session_id.clone(),
            thread: session.thread.clone(),
            sessions: self.sessions.clone(),
            options: session.config_options.clone()?,
            connection: self.connection.clone(),
            dispatch_tx: self.dispatch_tx.clone(),
        }))
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
        if self.thread.upgrade().is_none()
            || !self
                .sessions
                .borrow()
                .get(&self.session_id)
                .is_some_and(|session| session.thread.entity_id() == self.thread.entity_id())
        {
            return Err(anyhow!("Session has been released"));
        }
        let valid = self.options.config_options.borrow().iter().any(|option| {
            if &option.config_id != config_id {
                return false;
            }
            match (&option.kind, value) {
                (
                    acp::SessionConfigKind::Boolean(_),
                    acp::SessionConfigOptionValue::Boolean { .. },
                ) => true,
                (
                    acp::SessionConfigKind::Select(select),
                    acp::SessionConfigOptionValue::Id { value },
                ) => match &select.options {
                    acp::SessionConfigSelectOptions::Ungrouped(options) => {
                        options.iter().any(|option| &option.value == value)
                    }
                    acp::SessionConfigSelectOptions::Grouped(groups) => groups
                        .iter()
                        .any(|group| group.options.iter().any(|option| &option.value == value)),
                    _ => false,
                },
                _ => false,
            }
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
        let session_id = self.session_id.clone();
        let connection = self.connection.clone();
        let (response, applied) = AcpV2Connection::ordered_response(
            &self.connection,
            &self.dispatch_tx,
            acp::PromptRequest::new(self.session_id.clone(), content),
            move || {
                anyhow::ensure!(
                    owner.upgrade().is_some()
                        && sessions
                            .borrow()
                            .get(&session_id)
                            .is_some_and(|session| session.thread.entity_id() == owner.entity_id()),
                    "Session has been released or replaced"
                );
                Ok(())
            },
            |_| None,
        );
        let response = async move {
            response.await.map_err(|error| {
                if error.is::<ResponseCallbackDropped>()
                    || connection.is_incoming_closed()
                    || error
                        .downcast_ref::<agent_client_protocol::Error>()
                        .is_some_and(agent_client_protocol::is_incoming_transport_closed)
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
        new_requests: async_channel::Receiver<(acp::NewSessionRequest, acp::RequestId)>,
        new_responses:
            async_channel::Sender<Result<acp::NewSessionResponse, agent_client_protocol::Error>>,
        config_requests: async_channel::Receiver<acp::SetSessionConfigOptionRequest>,
        config_responses: async_channel::Sender<
            Result<acp::SetSessionConfigOptionResponse, agent_client_protocol::Error>,
        >,
        closes: async_channel::Receiver<acp::SessionId>,
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
            let (config_sender, config_requests) = async_channel::unbounded();
            let (config_responses, config_received) = async_channel::unbounded::<
                Result<acp::SetSessionConfigOptionResponse, agent_client_protocol::Error>,
            >();
            let (close_sender, closes) = async_channel::unbounded();
            let (prompt_sender, prompts) = async_channel::unbounded();
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
                        async move |request: acp::NewSessionRequest,
                                    responder: Responder<acp::NewSessionResponse>,
                                    agent: V2ConnectionTo<Client>| {
                            new_sender
                                .try_send((request, responder.id().clone()))
                                .expect("record creation");
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
                                .try_send(request.session_id)
                                .expect("record abandoned close");
                            responder.respond(acp::CloseSessionResponse::new())
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
                    Task::ready(()).shared(),
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
                new_requests,
                new_responses,
                config_requests,
                config_responses,
                closes,
                agent: received.await.expect("agent"),
                _agent_task: agent_task,
            }
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
            .available_commands(vec![
                acp::AvailableCommand::new("initial", "Initial command")
                    .meta(meta("initial-command")),
            ])
            .meta(meta("delivery-not-session"))
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
                !harness.client.supports_load_session() && !harness.client.supports_close_session()
            );
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
        assert!(bounded(unsupported.create(cx), cx).await.is_err());
        assert!(unsupported.new_requests.is_empty());
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
        let creation = harness.create(cx);
        bounded(harness.new_requests.recv(), cx)
            .await
            .expect("request");
        drop(creation);
        harness
            .new_responses
            .try_send(Ok(initial_response()))
            .expect("late response");
        assert_eq!(
            bounded(harness.closes.recv(), cx)
                .await
                .expect("abandoned close"),
            acp::SessionId::new("session")
        );
        assert!(harness.client.sessions.borrow().is_empty());
        let creation = harness.create(cx);
        bounded(harness.new_requests.recv(), cx)
            .await
            .expect("next request");
        drop(harness._agent_task);
        assert!(bounded(creation, cx).await.is_err());
        assert!(harness.client.sessions.borrow().is_empty());
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
            printf '{"jsonrpc":"2.0","id":%s,"result":{"sessionId":"stdio-session","configOptions":[{"configId":"enabled","name":"Enabled","type":"boolean","currentValue":false}],"availableCommands":[{"name":"initial","description":"Initial"}]}}\n' "$id"
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
                .new_session_request(configurations.clone(), &directories, |path| {
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
                    .new_session_request(
                        Vec::new(),
                        &PathList::new::<std::path::PathBuf>(&[]),
                        |path| path_style.is_absolute(path)
                    )
                    .is_err()
            );
            assert!(
                harness
                    .client
                    .new_session_request(
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
                                let request_id = responder.id().clone();
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
                    Task::ready(()).shared(),
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
        let input = harness.request_input(responder.id().clone(), cx);
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
        let first_id = first_responder.id().clone();
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
        let second_input = harness.request_input(second_responder.id().clone(), cx);
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
        let first = cx.update(|cx| {
            harness
                .client
                .login(acp::LoginAuthRequest::new("login"), cx)
        });
        harness.registered(cx).await;
        let second = cx.update(|cx| {
            harness
                .client
                .login(acp::LoginAuthRequest::new("login"), cx)
        });
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
                Task::ready(()).shared(),
                &mut cx.to_async(),
            ),
            cx,
        )
        .await
        .err()
        .expect("reject unimplemented fallback");
        assert!(error.to_string().contains("protocol"), "{error}");
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
                Task::ready(()).shared(),
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
