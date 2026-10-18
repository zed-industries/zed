use super::{
    AcpDebugLog, AcpDebugMessage, AcpSession, ClientContext, ConfigOptions, FlattenAcpResult as _,
    ForegroundWork, UnsupportedVersion, drain_foreground_queue, enqueue_notification,
    enqueue_request, first_class_terminal_auth_task, handle_v2_complete_elicitation,
    handle_v2_create_elicitation, map_acp_error, mcp_configurations_for_project, respond_err,
    respond_result, runtime, session_thread, terminal_auth_task_id, transport,
    with_v2_client_ingress, work_dirs_from_session_info,
};
use acp_thread::{
    AgentConnection, AgentSessionInfo, AgentSessionList, AgentSessionListRequest,
    AgentSessionListResponse, ElicitationStore,
};
use action_log::ActionLog;
use agent_client_protocol::schema::{
    ProtocolVersion,
    v1::{self as acp, ErrorCode},
    v2 as acp_v2,
};
use agent_client_protocol::{
    Agent, Builder, Client, ConnectionTo, HandleDispatchFrom, Lines, Responder, V2Builder,
};
use anyhow::anyhow;
use async_channel;
use collections::HashMap;
use futures::channel::mpsc;
use futures::future::Shared;
use futures::{Future, FutureExt as _};
use project::agent_server_store::{
    AgentServerCommand, AgentServerStore, AllAgentServersSettings, CustomAgentServerSettings,
};
use project::{AgentId, Project};
use serde::Deserialize;
use settings::{AgentConfigOptionValue, SettingsStore};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::{any::Any, cell::RefCell};
use task::SpawnInTerminal;
use util::ResultExt as _;
use util::path_list::PathList;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, SharedString, Subscription, Task, WeakEntity};

use acp_thread::{AcpThread, LoadError, TerminalProviderEvent};
use terminal::TerminalBuilder;
use terminal::terminal_settings::{AlternateScroll, CursorShape};

use crate::{CURSOR_ID, GEMINI_ID};

pub const GEMINI_TERMINAL_AUTH_METHOD_ID: &str = "spawn-gemini-cli";
const PARAMETERIZED_MODEL_PICKER_META_KEY: &str = "parameterizedModelPicker";
async fn close_session_and_drain(
    connection: &ConnectionTo<Agent>,
    session_id: acp_v2::SessionId,
    supports_close: bool,
    dispatch_tx: &mpsc::UnboundedSender<ForegroundWork>,
) -> Result<()> {
    if supports_close {
        connection
            .send_request(acp::CloseSessionRequest::new(acp::SessionId::new(
                session_id.0,
            )))
            .block_task()
            .await
            .log_err();
    }
    drain_foreground_queue(dispatch_tx).await
}

pub struct AcpConnection {
    id: AgentId,
    telemetry_id: SharedString,
    agent_version: Option<SharedString>,
    connection: ConnectionTo<Agent>,
    sessions: Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>>,
    pending_sessions: RefCell<HashMap<acp_v2::SessionId, PendingAcpSession>>,
    auth_methods: Vec<acp_v2::AuthMethod>,
    agent_server_store: WeakEntity<AgentServerStore>,
    agent_capabilities: acp::AgentCapabilities,
    request_elicitations: Entity<ElicitationStore>,
    defaults: AcpConnectionDefaults,
    _child: Option<transport::OwnedChild>,
    session_list: Option<Rc<AcpSessionList>>,
    debug_log: AcpDebugLog,
    _settings_subscription: Subscription,
    _runtime: runtime::ClientRuntime,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
    _wait_task: Task<()>,
    _stderr_task: Shared<Task<()>>,
}

#[derive(Clone, Default)]
struct AcpConnectionDefaults {
    mode: Rc<RefCell<Option<acp_v2::SessionConfigValueId>>>,
    config_options: Rc<RefCell<HashMap<String, AgentConfigOptionValue>>>,
}

impl AcpConnectionDefaults {
    fn new(
        mode: Option<acp_v2::SessionConfigValueId>,
        config_options: HashMap<String, AgentConfigOptionValue>,
    ) -> Self {
        Self {
            mode: Rc::new(RefCell::new(mode)),
            config_options: Rc::new(RefCell::new(config_options)),
        }
    }

    fn mode(&self) -> Option<acp_v2::SessionConfigValueId> {
        self.mode.borrow().clone()
    }

    fn config_option(&self, config_id: &str) -> Option<AgentConfigOptionValue> {
        self.config_options.borrow().get(config_id).cloned()
    }

    fn set(
        &self,
        mode: Option<acp_v2::SessionConfigValueId>,
        config_options: HashMap<String, AgentConfigOptionValue>,
    ) {
        *self.mode.borrow_mut() = mode;
        *self.config_options.borrow_mut() = config_options;
    }

    fn refresh_from_settings(&self, agent_id: &AgentId, cx: &App) {
        let Some(settings_store) = cx.try_global::<SettingsStore>() else {
            self.set(None, HashMap::default());
            return;
        };
        let settings = settings_store.get::<AllAgentServersSettings>(None);
        let Some(agent_settings) = settings.get(agent_id.as_ref()) else {
            self.set(None, HashMap::default());
            return;
        };

        let default_config_options = match agent_settings {
            CustomAgentServerSettings::Custom {
                default_config_options,
                ..
            }
            | CustomAgentServerSettings::Registry {
                default_config_options,
                ..
            } => default_config_options.clone(),
        };
        self.set(
            agent_settings
                .default_mode()
                .map(acp_v2::SessionConfigValueId::new),
            default_config_options,
        );
    }

    fn observe_settings(&self, agent_id: AgentId, cx: &mut App) -> Subscription {
        if cx.try_global::<SettingsStore>().is_none() {
            return Subscription::new(|| {});
        }

        self.refresh_from_settings(&agent_id, cx);
        let defaults = self.clone();
        cx.observe_global::<SettingsStore>(move |cx| {
            defaults.refresh_from_settings(&agent_id, cx);
        })
    }
}

struct PendingAcpSession {
    thread: WeakEntity<AcpThread>,
    task: Shared<Task<Result<(), Arc<anyhow::Error>>>>,
}

struct SessionConfigResponse {
    modes: Option<acp::SessionModeState>,
    config_options: Option<Vec<acp::SessionConfigOption>>,
}

impl ConfigOptions {
    async fn set_config_option(
        &self,
        connection: &ConnectionTo<Agent>,
        session_id: acp_v2::SessionId,
        config_id: acp_v2::SessionConfigId,
        value: acp_v2::SessionConfigOptionValue,
    ) -> Result<Vec<acp_v2::SessionConfigOption>> {
        let value = acp_thread::config_options::value_to_v1(value)?;
        let response = connection
            .send_request(acp::SetSessionConfigOptionRequest::new(
                acp::SessionId::new(session_id.0),
                acp::SessionConfigId::new(config_id.0),
                value,
            ))
            .block_task()
            .await?;
        let config_options = acp_thread::config_options::from_v1(response.config_options)?;
        *self.config_options.borrow_mut() = config_options.clone();
        self.tx.borrow_mut().send(()).ok();
        Ok(config_options)
    }
}

pub struct AcpSessionList {
    connection: ConnectionTo<Agent>,
    supports_delete: bool,
    updates_tx: async_channel::Sender<acp_thread::SessionListUpdate>,
    updates_rx: async_channel::Receiver<acp_thread::SessionListUpdate>,
}

impl AcpSessionList {
    fn new(connection: ConnectionTo<Agent>, supports_delete: bool) -> Self {
        let (tx, rx) = async_channel::unbounded();
        Self {
            connection,
            supports_delete,
            updates_tx: tx,
            updates_rx: rx,
        }
    }

    fn notify_update(&self) {
        self.updates_tx
            .try_send(acp_thread::SessionListUpdate::Refresh)
            .log_err();
    }

    pub(super) fn send_info_update(
        &self,
        session_id: acp_v2::SessionId,
        update: impl Into<acp_thread::AgentSessionInfoUpdate>,
    ) {
        let update = update.into();
        self.updates_tx
            .try_send(acp_thread::SessionListUpdate::SessionInfo { session_id, update })
            .log_err();
    }
}

impl AgentSessionList for AcpSessionList {
    fn list_sessions(
        &self,
        request: AgentSessionListRequest,
        cx: &mut App,
    ) -> Task<Result<AgentSessionListResponse>> {
        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            let acp_request = acp::ListSessionsRequest::new()
                .cwd(request.cwd)
                .cursor(request.cursor);
            let response = conn
                .send_request(acp_request)
                .block_task()
                .await
                .map_err(map_acp_error)?;
            Ok(AgentSessionListResponse {
                sessions: response
                    .sessions
                    .into_iter()
                    .map(|s| AgentSessionInfo {
                        session_id: acp_v2::SessionId::new(s.session_id.0),
                        work_dirs: Some(work_dirs_from_session_info(
                            s.cwd,
                            s.additional_directories,
                        )),
                        title: s.title.map(Into::into),
                        updated_at: s.updated_at.and_then(|date_str| {
                            chrono::DateTime::parse_from_rfc3339(&date_str)
                                .ok()
                                .map(|dt| dt.with_timezone(&chrono::Utc))
                        }),
                        created_at: None,
                        meta: s.meta,
                    })
                    .collect(),
                next_cursor: response.next_cursor,
                meta: response.meta,
            })
        })
    }

    fn supports_delete(&self) -> bool {
        self.supports_delete
    }

    fn delete_session(&self, session_id: &acp_v2::SessionId, cx: &mut App) -> Task<Result<()>> {
        if !self.supports_delete() {
            return Task::ready(Err(anyhow::anyhow!("delete_session not supported")));
        }

        let conn = self.connection.clone();
        let updates_tx = self.updates_tx.clone();
        let session_id = acp::SessionId::new(session_id.0.clone());
        cx.foreground_executor().spawn(async move {
            conn.send_request(acp::DeleteSessionRequest::new(session_id))
                .block_task()
                .await
                .map_err(map_acp_error)?;
            updates_tx
                .try_send(acp_thread::SessionListUpdate::Refresh)
                .log_err();
            Ok(())
        })
    }

    fn watch(
        &self,
        _cx: &mut App,
    ) -> Option<async_channel::Receiver<acp_thread::SessionListUpdate>> {
        Some(self.updates_rx.clone())
    }

    fn notify_refresh(&self) {
        self.notify_update();
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

pub async fn connect(
    agent_id: AgentId,
    project: Entity<Project>,
    command: AgentServerCommand,
    agent_server_store: WeakEntity<AgentServerStore>,
    default_mode: Option<acp_v2::SessionConfigValueId>,
    default_config_options: HashMap<String, AgentConfigOptionValue>,
    cx: &mut AsyncApp,
) -> Result<Rc<dyn AgentConnection>> {
    let conn = AcpConnection::stdio(
        agent_id,
        project,
        command.clone(),
        agent_server_store,
        default_mode,
        default_config_options,
        cx,
    )
    .await?;
    Ok(Rc::new(conn) as _)
}

const MINIMUM_SUPPORTED_VERSION: ProtocolVersion = ProtocolVersion::V1;

pub(super) fn connect_client_future(
    name: &'static str,
    transport: impl agent_client_protocol::ConnectTo<Client> + 'static,
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
    connection_tx: futures::channel::oneshot::Sender<ConnectionTo<Agent>>,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> impl Future<Output = Result<(), acp::Error>> {
    async move {
        let cleanup = runtime::disconnect_cleanup(dispatch_tx.clone());
        let future = client_builder(name, dispatch_tx)
            .on_close({
                let cleanup = cleanup.clone();
                async move |_connection: ConnectionTo<Agent>| cleanup.await
            })
            .connect_with(
                transport,
                move |connection: ConnectionTo<Agent>| async move {
                    runtime::publish_connection(connection, connection_tx, shutdown).await
                },
            );
        runtime::finish_connection(future, cleanup).await
    }
}

fn client_builder(
    name: &'static str,
    dispatch_sender: mpsc::UnboundedSender<ForegroundWork>,
) -> Builder<Client, impl HandleDispatchFrom<Agent>> {
    // Each handler forwards its inputs onto the foreground dispatch queue.
    // The SDK requires the closure to be `Send`, so we move a clone of
    // `dispatch_tx` into each one.
    macro_rules! on_request {
        ($handler:ident) => {{
            let dispatch_sender = dispatch_sender.clone();
            async move |request, responder, _connection| {
                enqueue_request(&dispatch_sender, request, responder, $handler);
                Ok(())
            }
        }};
    }
    macro_rules! on_notification {
        ($handler:ident) => {{
            let dispatch_sender = dispatch_sender.clone();
            async move |notification, _connection| {
                enqueue_notification(&dispatch_sender, notification, $handler);
                Ok(())
            }
        }};
    }

    Client
        .builder()
        .name(name)
        // --- Request handlers (agent→client) ---
        .on_receive_request(
            on_request!(handle_request_permission),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_write_text_file),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_read_text_file),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_create_terminal),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_kill_terminal),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_release_terminal),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_terminal_output),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_wait_for_terminal_exit),
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            on_request!(handle_create_elicitation),
            agent_client_protocol::on_receive_request!(),
        )
        // --- Notification handlers (agent→client) ---
        .on_receive_notification(
            on_notification!(handle_session_notification),
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_notification(
            on_notification!(handle_complete_elicitation),
            agent_client_protocol::on_receive_notification!(),
        )
}

fn client_capabilities_for_agent(agent_id: &AgentId) -> acp::ClientCapabilities {
    let mut meta = acp::Meta::from_iter([
        ("terminal_output".into(), true.into()),
        ("terminal-auth".into(), true.into()),
    ]);

    if agent_id.as_ref() == CURSOR_ID {
        meta.insert(PARAMETERIZED_MODEL_PICKER_META_KEY.into(), true.into());
    }

    let session_capabilities = acp::ClientSessionCapabilities::new()
        .config_options(
            acp::SessionConfigOptionsCapabilities::new()
                .boolean(acp::BooleanConfigOptionCapabilities::new()),
        )
        .compaction(acp::CompactionCapabilities::new())
        .notices(acp::NoticeCapabilities::new());

    acp::ClientCapabilities::new()
        .fs(acp::FileSystemCapabilities::new()
            .read_text_file(true)
            .write_text_file(true))
        .terminal(true)
        .auth(acp::AuthCapabilities::new().terminal(true))
        .session(session_capabilities)
        .elicitation(
            acp::ElicitationCapabilities::new()
                .form(acp::ElicitationFormCapabilities::new())
                .url(acp::ElicitationUrlCapabilities::new()),
        )
        .meta(meta)
}

impl AcpConnection {
    /// Binds supported v2 client handlers to the existing foreground owners.
    /// This does not change the protocol used by the connection's transport.
    pub fn with_v2_client_ingress<Handler: HandleDispatchFrom<Agent>>(
        &self,
        builder: V2Builder<Client, Handler>,
    ) -> V2Builder<Client, impl HandleDispatchFrom<Agent> + use<Handler>> {
        with_v2_client_ingress(builder, self.dispatch_tx.clone())
    }

    pub fn subscribe_debug_messages(
        &self,
    ) -> (
        Vec<AcpDebugMessage>,
        async_channel::Receiver<AcpDebugMessage>,
    ) {
        self.debug_log.subscribe()
    }

    pub async fn stdio(
        agent_id: AgentId,
        project: Entity<Project>,
        command: AgentServerCommand,
        agent_server_store: WeakEntity<AgentServerStore>,
        default_mode: Option<acp_v2::SessionConfigValueId>,
        default_config_options: HashMap<String, AgentConfigOptionValue>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let original_command = command.clone();
        let transport::StdioProcess {
            child,
            incoming,
            outgoing,
            stderr,
            debug_log,
        } = transport::spawn_stdio(&project, command, cx)?;
        let mut child = transport::OwnedChild(child);
        let sessions = Rc::new(RefCell::new(HashMap::default()));

        let (release_channel, version): (Option<&str>, String) = cx.update(|cx| {
            (
                release_channel::ReleaseChannel::try_global(cx)
                    .map(|release_channel| release_channel.display_name()),
                release_channel::AppVersion::global(cx).to_string(),
            )
        });

        let client_session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>> =
            Rc::new(RefCell::new(None));
        let request_elicitations = cx.new(|_| ElicitationStore::default());

        let dispatch_context = ClientContext {
            sessions: sessions.clone(),
            session_list: client_session_list.clone(),
            request_elicitations: request_elicitations.clone(),
            active_request_scopes: None,
        };
        let (mut runtime, shutdown) = runtime::ClientRuntime::new(dispatch_context, cx);
        let dispatch_tx = runtime.sender();

        let transport = Lines::new(outgoing, incoming);
        let stderr_task = cx
            .background_spawn(async move {
                stderr.await.log_err();
            })
            .shared();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let connection_future = connect_client_future(
            "zed",
            transport,
            dispatch_tx.clone(),
            connection_tx,
            runtime::shutdown_requested(shutdown),
        );
        runtime.start_io(connection_future, cx);

        let connection_rx = async move {
            connection_rx
                .await
                .context("Failed to receive ACP connection handle")
        }
        .boxed_local();
        let status_fut = child
            .0
            .status()
            .map(|status| {
                status.map_err(|error| anyhow!("failed to wait for agent server exit: {error}"))
            })
            .boxed_local();
        let (connection, status_fut, _) = runtime::race_startup(
            connection_rx,
            status_fut,
            stderr_task.clone(),
            &debug_log,
            cx,
        )
        .await?;
        let drained = {
            let connection = connection.clone();
            runtime::drain_transport(
                async move { connection.incoming_closed().await },
                dispatch_tx.clone(),
                stderr_task.clone(),
            )
        };

        let initialize_response = connection
            .send_request(
                acp::InitializeRequest::new(ProtocolVersion::V1)
                    .client_capabilities(client_capabilities_for_agent(&agent_id))
                    .client_info(
                        acp::Implementation::new("zed", version)
                            .title(release_channel.map(ToOwned::to_owned)),
                    ),
            )
            .block_task()
            .map(|response| response.map_err(anyhow::Error::from))
            .boxed_local();
        let (response, status_fut, drained) =
            runtime::race_startup(initialize_response, status_fut, drained, &debug_log, cx).await?;

        if response.protocol_version < MINIMUM_SUPPORTED_VERSION {
            return Err(UnsupportedVersion.into());
        }

        let wait_task = runtime::watch_process(
            status_fut,
            drained,
            debug_log.clone(),
            {
                let sessions = sessions.clone();
                move |load_error, cx| {
                    emit_load_error_to_all_sessions(&sessions, load_error, cx);
                }
            },
            cx,
        );

        let agent_info = response.agent_info;
        let telemetry_id = agent_info
            .as_ref()
            // Use the one the agent provides if we have one
            .map(|info| SharedString::from(info.name.clone()))
            // Otherwise, just use the name
            .unwrap_or_else(|| agent_id.0.clone());
        let agent_version = agent_info
            .and_then(|info| (!info.version.is_empty()).then(|| SharedString::from(info.version)));
        let agent_supports_delete = response
            .agent_capabilities
            .session_capabilities
            .delete
            .is_some();

        let session_list = if response
            .agent_capabilities
            .session_capabilities
            .list
            .is_some()
        {
            let list = Rc::new(AcpSessionList::new(
                connection.clone(),
                agent_supports_delete,
            ));
            *client_session_list.borrow_mut() = Some(list.clone());
            Some(list)
        } else {
            None
        };

        // TODO: Remove this override once Google team releases their official auth methods
        let auth_methods = if agent_id.0.as_ref() == GEMINI_ID {
            let mut gemini_args = original_command.args.clone();
            gemini_args.retain(|a| a != "--experimental-acp" && a != "--acp");
            let value = serde_json::json!({
                "label": "gemini /auth",
                "command": original_command.path.to_string_lossy(),
                "args": gemini_args,
                "env": original_command.env.unwrap_or_default(),
            });
            let meta = acp_v2::Meta::from_iter([("terminal-auth".to_string(), value)]);
            vec![acp_v2::AuthMethod::Agent(
                acp_v2::AuthMethodAgent::new(GEMINI_TERMINAL_AUTH_METHOD_ID, "Login")
                    .description("Login with your Google or Vertex AI account")
                    .meta(meta),
            )]
        } else {
            acp_thread::auth_methods::from_v1(response.auth_methods)?
        };
        let defaults = AcpConnectionDefaults::new(default_mode, default_config_options);
        let settings_subscription = cx.update({
            let agent_id = agent_id.clone();
            let defaults = defaults.clone();
            move |cx| defaults.observe_settings(agent_id, cx)
        });

        Ok(Self {
            id: agent_id,
            auth_methods,
            agent_server_store,
            connection,
            telemetry_id,
            agent_version,
            sessions,
            pending_sessions: RefCell::new(HashMap::default()),
            agent_capabilities: response.agent_capabilities,
            request_elicitations,
            defaults,
            session_list,
            debug_log,
            _settings_subscription: settings_subscription,
            _runtime: runtime,
            dispatch_tx,
            _wait_task: wait_task,
            _stderr_task: stderr_task,
            _child: Some(child),
        })
    }

    pub fn prompt_capabilities(&self) -> acp_v2::PromptCapabilities {
        acp_thread::prompt_capabilities::from_v1(
            self.agent_capabilities.prompt_capabilities.clone(),
        )
    }

    #[cfg(any(test, feature = "test-support"))]
    fn new_for_test(
        connection: ConnectionTo<Agent>,
        sessions: Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>>,
        agent_capabilities: acp::AgentCapabilities,
        request_elicitations: Entity<ElicitationStore>,
        agent_server_store: WeakEntity<AgentServerStore>,
        io_task: Task<()>,
        dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
        dispatch_task: Task<()>,
        cx: &mut App,
    ) -> Self {
        let agent_id = AgentId::new("test");
        let defaults = AcpConnectionDefaults::default();
        let settings_subscription = defaults.observe_settings(agent_id.clone(), cx);

        Self {
            id: agent_id,
            telemetry_id: "test".into(),
            agent_version: None,
            connection,
            sessions,
            pending_sessions: RefCell::new(HashMap::default()),
            auth_methods: vec![],
            agent_server_store,
            agent_capabilities,
            request_elicitations,
            defaults,
            _child: None,
            session_list: None,
            debug_log: AcpDebugLog::default(),
            _settings_subscription: settings_subscription,
            _runtime: runtime::ClientRuntime::from_test_tasks(
                dispatch_tx.clone(),
                io_task,
                dispatch_task,
            ),
            dispatch_tx,
            _wait_task: Task::ready(()),
            _stderr_task: Task::ready(()).shared(),
        }
    }

    fn session_directories_from_work_dirs(
        &self,
        work_dirs: &PathList,
    ) -> Result<SessionDirectories> {
        let supports_additional_directories = self.supports_session_additional_directories();
        session_directories_from_work_dirs(work_dirs, supports_additional_directories)
    }

    fn register_session(
        self: &Rc<Self>,
        session_id: acp_v2::SessionId,
        thread: &Entity<AcpThread>,
        session_modes: Option<Rc<RefCell<acp::SessionModeState>>>,
        config_options: Option<ConfigOptions>,
        default_config_task: Option<Task<()>>,
        cx: &mut App,
    ) {
        let release_subscription = cx.observe_release(thread, {
            let session_id = session_id.clone();
            let thread = thread.downgrade();
            let this = Rc::downgrade(self);
            move |_, cx| {
                if let Some(this) = this.upgrade() {
                    this.release_session(session_id, thread, cx);
                }
            }
        });
        self.sessions.borrow_mut().insert(
            session_id,
            AcpSession {
                thread: thread.downgrade(),
                suppress_abort_err: false,
                session_modes,
                config_options,
                default_config_task,
                _release_subscription: release_subscription,
            },
        );
    }

    fn remove_session(&self, session_id: &acp_v2::SessionId, thread_id: gpui::EntityId) {
        let mut sessions = self.sessions.borrow_mut();
        if sessions
            .get(session_id)
            .is_some_and(|session| session.thread.entity_id() == thread_id)
        {
            sessions.remove(session_id);
        }
    }

    fn remove_pending_session(&self, session_id: &acp_v2::SessionId, thread_id: gpui::EntityId) {
        let mut sessions = self.pending_sessions.borrow_mut();
        if sessions
            .get(session_id)
            .is_some_and(|session| session.thread.entity_id() == thread_id)
        {
            sessions.remove(session_id);
        }
    }

    fn release_session(
        self: &Rc<Self>,
        session_id: acp_v2::SessionId,
        thread: WeakEntity<AcpThread>,
        cx: &mut App,
    ) {
        if self.pending_sessions.borrow().contains_key(&session_id)
            || !self
                .sessions
                .borrow()
                .get(&session_id)
                .is_some_and(|session| session.thread.entity_id() == thread.entity_id())
        {
            return;
        }

        if let Some(session) = self.sessions.borrow_mut().get_mut(&session_id) {
            session.default_config_task.take();
        }

        let task = cx
            .spawn({
                let this = Rc::downgrade(self);
                let connection = self.connection.clone();
                let dispatch_tx = self.dispatch_tx.clone();
                let supports_close = self.agent_supports_session_close();
                let session_id = session_id.clone();
                let thread_id = thread.entity_id();
                async move |_| {
                    let result = close_session_and_drain(
                        &connection,
                        session_id.clone(),
                        supports_close,
                        &dispatch_tx,
                    )
                    .await;
                    if let Some(this) = this.upgrade() {
                        this.remove_session(&session_id, thread_id);
                        this.remove_pending_session(&session_id, thread_id);
                    }
                    result.map_err(Arc::new)
                }
            })
            .shared();
        self.pending_sessions
            .borrow_mut()
            .insert(session_id, PendingAcpSession { thread, task });
    }

    fn agent_supports_session_close(&self) -> bool {
        self.agent_capabilities.session_capabilities.close.is_some()
    }

    fn open_or_create_session(
        self: Rc<Self>,
        session_id: acp_v2::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        rpc_call: impl FnOnce(
            ConnectionTo<Agent>,
            acp_v2::SessionId,
            SessionDirectories,
        )
            -> futures::future::LocalBoxFuture<'static, Result<SessionConfigResponse>>
        + 'static,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        // Active callers share the thread; an abandoned operation must finish
        // before a replacement can receive notifications under the same ID.
        let pending = self
            .pending_sessions
            .borrow()
            .get(&session_id)
            .map(|pending| (pending.thread.clone(), pending.task.clone()));
        if let Some((thread, task)) = pending {
            if let Some(thread) = thread.upgrade() {
                return cx.foreground_executor().spawn(async move {
                    task.await.map_err(|err| anyhow!(err))?;
                    Ok(thread)
                });
            }
            return cx.spawn(async move |cx| {
                task.await.log_err();
                cx.update(|cx| {
                    self.open_or_create_session(session_id, project, work_dirs, title, rpc_call, cx)
                })
                .await
            });
        }

        let previous_thread = self
            .sessions
            .borrow()
            .get(&session_id)
            .map(|session| session.thread.clone());
        if let Some(thread) = previous_thread {
            if let Some(thread) = thread.upgrade() {
                return Task::ready(Ok(thread));
            }
            // Entity-release effects may not have run yet when a view reopens.
            self.release_session(session_id.clone(), thread, cx);
            return self
                .open_or_create_session(session_id, project, work_dirs, title, rpc_call, cx);
        }

        let directories = match self.session_directories_from_work_dirs(&work_dirs) {
            Ok(directories) => directories,
            Err(error) => return Task::ready(Err(error)),
        };

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let thread = cx.new(|cx| {
            AcpThread::new(
                None,
                title,
                Some(work_dirs),
                self.clone(),
                project,
                action_log,
                session_id.clone(),
                watch::Receiver::constant(self.prompt_capabilities()),
                cx,
            )
        });
        self.register_session(session_id.clone(), &thread, None, None, None, cx);

        // The connection owns this task, so retaining the thread (which owns the
        // connection) here would keep both alive after all callers abandon the load.
        let shared_task: Shared<Task<Result<(), Arc<anyhow::Error>>>> = cx
            .spawn({
                let session_id = session_id.clone();
                let this = Rc::downgrade(&self);
                let thread = thread.downgrade();
                let connection = self.connection.clone();
                let dispatch_tx = self.dispatch_tx.clone();
                let supports_close = self.agent_supports_session_close();
                async move |cx| {
                    if thread.upgrade().is_none() {
                        if let Some(this) = this.upgrade() {
                            this.remove_session(&session_id, thread.entity_id());
                            this.remove_pending_session(&session_id, thread.entity_id());
                        }
                        return Ok(());
                    }

                    let response =
                        rpc_call(connection.clone(), session_id.clone(), directories).await;
                    if response.is_err() {
                        // Do not let replay from a failed load reach its replacement.
                        drain_foreground_queue(&dispatch_tx).await.log_err();
                    }
                    let Some(connection_state) = this.upgrade() else {
                        return Ok(());
                    };
                    let (modes, config_options) = match response
                        .and_then(|response| config_state(response.modes, response.config_options))
                    {
                        Ok(state) => state,
                        Err(error) => {
                            connection_state.remove_session(&session_id, thread.entity_id());
                            connection_state
                                .remove_pending_session(&session_id, thread.entity_id());
                            return Err(Arc::new(error));
                        }
                    };

                    if let Some(_thread) = thread.upgrade() {
                        {
                            let mut sessions = connection_state.sessions.borrow_mut();
                            let Some(session) = sessions
                                .get_mut(&session_id)
                                .filter(|session| session.thread.entity_id() == thread.entity_id())
                            else {
                                connection_state
                                    .remove_pending_session(&session_id, thread.entity_id());
                                return Err(Arc::new(anyhow!(
                                    "session was closed or replaced before load completed"
                                )));
                            };
                            session.session_modes = modes;
                            session.config_options = config_options.clone();
                            session.default_config_task = config_options.as_ref().map(|options| {
                                connection_state.apply_default_config_options(
                                    &session_id,
                                    options,
                                    cx,
                                )
                            });
                        }
                        connection_state.remove_pending_session(&session_id, thread.entity_id());
                        return Ok(());
                    }

                    drop(connection_state);
                    let result = close_session_and_drain(
                        &connection,
                        session_id.clone(),
                        supports_close,
                        &dispatch_tx,
                    )
                    .await;
                    if let Some(this) = this.upgrade() {
                        this.remove_session(&session_id, thread.entity_id());
                        this.remove_pending_session(&session_id, thread.entity_id());
                    }
                    result.map_err(Arc::new)
                }
            })
            .shared();

        self.pending_sessions.borrow_mut().insert(
            session_id,
            PendingAcpSession {
                thread: thread.downgrade(),
                task: shared_task.clone(),
            },
        );

        cx.foreground_executor().spawn(async move {
            shared_task.await.map_err(|err| anyhow!(err))?;
            Ok(thread)
        })
    }

    fn apply_default_config_options(
        &self,
        session_id: &acp_v2::SessionId,
        config_options: &ConfigOptions,
        cx: &mut AsyncApp,
    ) -> Task<()> {
        let id = self.id.clone();
        let defaults = self.defaults.clone();
        let session_id = session_id.clone();
        let config_options = config_options.clone();
        let connection = self.connection.clone();
        cx.spawn(async move |_| {
            let mut attempted_defaults = Vec::new();
            loop {
                let current_options = config_options.config_options.borrow().clone();
                let next_default = current_options.iter().find_map(|option| {
                    // Retry deferred or reset defaults after dependent changes, but
                    // do not repeat an attempt in the same state if defaults conflict.
                    if attempted_defaults.iter().any(|(config_id, options)| {
                        config_id == &option.config_id && options == &current_options
                    }) {
                        return None;
                    }
                    let value = defaults.config_option(option.config_id.0.as_ref())?;
                    let already_set = match &option.kind {
                        acp_v2::SessionConfigKind::Select(select) => {
                            value.as_value_id() == Some(select.current_value.0.as_ref())
                        }
                        acp_v2::SessionConfigKind::Boolean(boolean) => {
                            value.as_bool() == Some(boolean.current_value)
                        }
                        _ => false,
                    };
                    if already_set {
                        return None;
                    }
                    Some((option.clone(), value))
                });
                let Some((config_option, default_value)) = next_default else {
                    break;
                };
                attempted_defaults.push((config_option.config_id.clone(), current_options));

                let value_to_apply = match &config_option.kind {
                    acp_v2::SessionConfigKind::Select(select) => default_value
                        .as_value_id()
                        .and_then(|value_id| match &select.options {
                            acp_v2::SessionConfigSelectOptions::Ungrouped(options) => options
                                .iter()
                                .any(|opt| &*opt.value.0 == value_id)
                                .then(|| {
                                    acp_v2::SessionConfigOptionValue::id(value_id.to_string())
                                }),
                            acp_v2::SessionConfigSelectOptions::Grouped(groups) => groups
                                .iter()
                                .any(|group| {
                                    group.options.iter().any(|opt| &*opt.value.0 == value_id)
                                })
                                .then(|| {
                                    acp_v2::SessionConfigOptionValue::id(value_id.to_string())
                                }),
                            _ => None,
                        }),
                    acp_v2::SessionConfigKind::Boolean(_) => default_value
                        .as_bool()
                        .map(acp_v2::SessionConfigOptionValue::boolean),
                    _ => None,
                };

                let Some(value_to_apply) = value_to_apply else {
                    log::warn!(
                        "`{}` is not a valid value for config option `{}` in {}",
                        default_value,
                        config_option.config_id.0,
                        id
                    );
                    continue;
                };
                config_options
                    .set_config_option(
                        &connection,
                        session_id.clone(),
                        config_option.config_id,
                        value_to_apply,
                    )
                    .await
                    .log_err();
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SessionDirectories {
    cwd: PathBuf,
    additional_directories: Vec<PathBuf>,
}

impl SessionDirectories {
    fn into_new_session_request(self, mcp_servers: Vec<acp::McpServer>) -> acp::NewSessionRequest {
        acp::NewSessionRequest::new(self.cwd)
            .additional_directories(self.additional_directories)
            .mcp_servers(mcp_servers)
    }

    fn into_load_session_request(
        self,
        session_id: acp_v2::SessionId,
        mcp_servers: Vec<acp::McpServer>,
    ) -> acp::LoadSessionRequest {
        acp::LoadSessionRequest::new(acp::SessionId::new(session_id.0), self.cwd)
            .additional_directories(self.additional_directories)
            .mcp_servers(mcp_servers)
    }

    fn into_resume_session_request(
        self,
        session_id: acp_v2::SessionId,
        mcp_servers: Vec<acp::McpServer>,
    ) -> acp::ResumeSessionRequest {
        acp::ResumeSessionRequest::new(acp::SessionId::new(session_id.0), self.cwd)
            .additional_directories(self.additional_directories)
            .mcp_servers(mcp_servers)
    }
}

fn session_directories_from_work_dirs(
    work_dirs: &PathList,
    supports_additional_directories: bool,
) -> Result<SessionDirectories> {
    let mut ordered_paths = work_dirs.ordered_paths();
    let cwd = ordered_paths
        .next()
        .cloned()
        .ok_or_else(|| anyhow!("Working directory cannot be empty"))?;
    let additional_directories = if supports_additional_directories {
        ordered_paths.cloned().collect()
    } else {
        Vec::new()
    };

    Ok(SessionDirectories {
        cwd,
        additional_directories,
    })
}

fn emit_load_error_to_all_sessions(
    sessions: &Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>>,
    error: LoadError,
    cx: &mut AsyncApp,
) {
    let threads: Vec<_> = sessions
        .borrow()
        .values()
        .map(|session| session.thread.clone())
        .collect();

    for thread in threads {
        thread
            .update(cx, |thread, cx| thread.emit_load_error(error.clone(), cx))
            .ok();
    }
}

/// Used to support the _meta method prior to stabilization
fn meta_terminal_auth_task(
    agent_id: &AgentId,
    method_id: &acp_v2::AuthMethodId,
    method: &acp_v2::AuthMethod,
) -> Option<SpawnInTerminal> {
    #[derive(Deserialize)]
    struct MetaTerminalAuth {
        label: String,
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    }

    let meta = method.meta()?;
    let terminal_auth =
        serde_json::from_value::<MetaTerminalAuth>(meta.get("terminal-auth")?.clone()).ok()?;

    Some(acp_thread::build_terminal_auth_task(
        terminal_auth_task_id(agent_id, method_id),
        terminal_auth.label.clone(),
        terminal_auth.command,
        terminal_auth.args,
        terminal_auth.env,
    ))
}

impl AgentConnection for AcpConnection {
    fn agent_id(&self) -> AgentId {
        self.id.clone()
    }

    fn telemetry_id(&self) -> SharedString {
        self.telemetry_id.clone()
    }

    fn agent_version(&self) -> Option<SharedString> {
        self.agent_version.clone()
    }

    fn new_session(
        self: Rc<Self>,
        project: Entity<Project>,
        work_dirs: PathList,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let directories = match self.session_directories_from_work_dirs(&work_dirs) {
            Ok(directories) => directories,
            Err(error) => return Task::ready(Err(error)),
        };
        let name = self.id.0.clone();
        let mcp_servers = mcp_servers_for_project(&project, cx);

        cx.spawn(async move |cx| {
            let (response_sender, response_received) = futures::channel::oneshot::channel();
            let (registered, registration_finished) = futures::channel::oneshot::channel();
            let request = self
                .connection
                .prepare_request(directories.into_new_session_request(mcp_servers));
            let cancellation = request.cancellation_handle();
            request
                .on_receiving_result({
                    let connection = self.connection.clone();
                    let supports_close = self
                        .agent_capabilities
                        .session_capabilities
                        .close
                        .is_some();
                    async move |result: Result<acp::NewSessionResponse, acp::Error>| {
                        let session_id = result
                            .as_ref()
                            .ok()
                            .map(|response| response.session_id.clone());
                        let registered = response_sender.send(result).is_ok()
                            && registration_finished.await.is_ok();
                        if !registered && supports_close {
                            if let Some(session_id) = session_id {
                                connection
                                    .prepare_request(acp::CloseSessionRequest::new(session_id))
                                    .on_receiving_result(
                                        async move |result: Result<
                                            acp::CloseSessionResponse,
                                            acp::Error,
                                        >| {
                                            if let Err(error) = result {
                                                log::warn!(
                                                    "ACP abandoned-session close failed with code {}",
                                                    error.code
                                                );
                                            }
                                            Ok(())
                                        },
                                    )
                                    .log_err();
                            }
                        }
                        Ok(())
                    }
                })
                .map_err(map_acp_error)?;
            // Early SDK cancellation is not latched, so arm caller-drop cleanup
            // only after the prepared request's publishing call returns.
            let _request_guard = util::defer(move || {
                cancellation.cancel().log_err();
            });
            let response = response_received
                .await
                .context("ACP new-session response callback was cancelled")?
                .map_err(map_acp_error)?;

            let (modes, config_options) = config_state(response.modes, response.config_options)?;

            let default_mode = self.defaults.mode();
            if let Some(default_mode) = default_mode {
                let default_mode = acp::SessionModeId::new(default_mode.0);
                if let Some(modes) = modes.as_ref() {
                    let mut modes_ref = modes.borrow_mut();
                    let has_mode = modes_ref
                        .available_modes
                        .iter()
                        .any(|mode| mode.id == default_mode);

                    if has_mode {
                        let initial_mode_id = modes_ref.current_mode_id.clone();

                        cx.spawn({
                            let default_mode = default_mode.clone();
                            let session_id = response.session_id.clone();
                            let modes = modes.clone();
                            let conn = self.connection.clone();
                            async move |_| {
                                let result = conn
                                    .send_request(acp::SetSessionModeRequest::new(
                                        session_id,
                                        default_mode,
                                    ))
                                    .block_task()
                                .await
                                .log_err();

                                if result.is_none() {
                                    modes.borrow_mut().current_mode_id = initial_mode_id;
                                }
                            }
                        })
                        .detach();

                        modes_ref.current_mode_id = default_mode;
                    } else {
                        let available_modes = modes_ref
                            .available_modes
                            .iter()
                            .map(|mode| format!("- `{}`: {}", mode.id, mode.name))
                            .collect::<Vec<_>>()
                            .join("\n");

                        log::warn!(
                            "`{default_mode}` is not valid {name} mode. Available options:\n{available_modes}",
                        );
                    }
                }
            }

            let session_id = acp_v2::SessionId::new(response.session_id.0);
            let default_config_task = config_options.as_ref().map(|options| {
                self.apply_default_config_options(&session_id, options, cx)
            });

            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread: Entity<AcpThread> = cx.new(|cx| {
                AcpThread::new(
                    None,
                    None,
                    Some(work_dirs),
                    self.clone(),
                    project,
                    action_log,
                    session_id.clone(),
                    // ACP doesn't currently support per-session prompt capabilities or changing capabilities dynamically.
                    watch::Receiver::constant(self.prompt_capabilities()),
                    cx,
                )
            });

            cx.update(|cx| {
                self.register_session(
                    session_id,
                    &thread,
                    modes,
                    config_options,
                    default_config_task,
                    cx,
                )
            });

            registered
                .send(())
                .map_err(|_| anyhow!("ACP new-session ordering callback ended before registration"))?;
            Ok(thread)
        })
    }

    fn supports_load_session(&self) -> bool {
        self.agent_capabilities.load_session
    }

    fn supports_resume_session(&self) -> bool {
        self.agent_capabilities
            .session_capabilities
            .resume
            .is_some()
    }

    fn supports_session_additional_directories(&self) -> bool {
        self.agent_capabilities
            .session_capabilities
            .additional_directories
            .is_some()
    }

    fn load_session(
        self: Rc<Self>,
        session_id: acp_v2::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        if !self.agent_capabilities.load_session {
            return Task::ready(Err(anyhow!(LoadError::Other(
                "Loading sessions is not supported by this agent.".into()
            ))));
        }

        let mcp_servers = mcp_servers_for_project(&project, cx);
        self.open_or_create_session(
            session_id,
            project,
            work_dirs,
            title,
            move |connection, session_id, directories| {
                Box::pin(async move {
                    let response = connection
                        .send_request(
                            directories.into_load_session_request(session_id.clone(), mcp_servers),
                        )
                        .block_task()
                        .await
                        .map_err(map_acp_error)?;
                    Ok(SessionConfigResponse {
                        modes: response.modes,
                        config_options: response.config_options,
                    })
                })
            },
            cx,
        )
    }

    fn resume_session(
        self: Rc<Self>,
        session_id: acp_v2::SessionId,
        project: Entity<Project>,
        work_dirs: PathList,
        title: Option<SharedString>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        if self
            .agent_capabilities
            .session_capabilities
            .resume
            .is_none()
        {
            return Task::ready(Err(anyhow!(LoadError::Other(
                "Resuming sessions is not supported by this agent.".into()
            ))));
        }

        let mcp_servers = mcp_servers_for_project(&project, cx);
        self.open_or_create_session(
            session_id,
            project,
            work_dirs,
            title,
            move |connection, session_id, directories| {
                Box::pin(async move {
                    let response = connection
                        .send_request(
                            directories
                                .into_resume_session_request(session_id.clone(), mcp_servers),
                        )
                        .block_task()
                        .await
                        .map_err(map_acp_error)?;
                    Ok(SessionConfigResponse {
                        modes: response.modes,
                        config_options: response.config_options,
                    })
                })
            },
            cx,
        )
    }

    fn auth_methods(&self) -> &[acp_v2::AuthMethod] {
        &self.auth_methods
    }

    fn terminal_auth_task(
        &self,
        method_id: &acp_v2::AuthMethodId,
        cx: &App,
    ) -> Option<Task<Result<SpawnInTerminal>>> {
        let method = self
            .auth_methods
            .iter()
            .find(|method| method.method_id() == method_id)?;

        match method {
            acp_v2::AuthMethod::Terminal(terminal) => Some(first_class_terminal_auth_task(
                self.id.clone(),
                self.agent_server_store.clone(),
                terminal.clone(),
                cx,
            )),
            acp_v2::AuthMethod::Agent(_) => meta_terminal_auth_task(&self.id, method_id, method)
                .map(|task| Task::ready(Ok(task))),
            _ => None,
        }
    }

    fn authenticate(&self, method_id: acp_v2::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
        if !self.auth_methods.iter().any(|method| {
            matches!(method, acp_v2::AuthMethod::Agent(agent) if agent.method_id == method_id)
        }) {
            return Task::ready(Err(anyhow!(
                "Authentication method is not supported for agent-managed login"
            )));
        }

        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            conn.send_request(acp::AuthenticateRequest::new(acp::AuthMethodId::new(
                method_id.0,
            )))
            .block_task()
            .await?;
            Ok(())
        })
    }

    fn supports_logout(&self) -> bool {
        self.agent_capabilities.auth.logout.is_some()
    }

    fn logout(&self, cx: &mut App) -> Task<Result<()>> {
        if !self.supports_logout() {
            return Task::ready(Err(anyhow!("Logout is not supported by this agent.")));
        }

        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            conn.send_request(acp::LogoutRequest::new())
                .block_task()
                .await?;
            Ok(())
        })
    }

    fn validate_prompt_content(&self, content: &[acp_v2::ContentBlock]) -> Result<()> {
        acp_thread::content::validate_prompt_content_for_v1(content)
    }

    fn prompt(
        &self,
        params: acp_v2::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        let session_id = params.session_id.clone();
        let params = match acp_thread::content::prompt_to_v1(params) {
            Ok(params) => params,
            Err(error) => return Task::ready(Err(error)),
        };
        let conn = self.connection.clone();
        let sessions = self.sessions.clone();
        cx.foreground_executor().spawn(async move {
            let result = conn.send_request(params).block_task().await;

            let mut suppress_abort_err = false;

            if let Some(session) = sessions.borrow_mut().get_mut(&session_id) {
                suppress_abort_err = session.suppress_abort_err;
                session.suppress_abort_err = false;
            }

            match result {
                Ok(response) => Ok(response),
                Err(err) => {
                    if err.code != ErrorCode::InternalError {
                        anyhow::bail!(err)
                    }

                    let Some(data) = &err.data else {
                        anyhow::bail!(err)
                    };

                    // Temporary workaround until the following PR is generally available:
                    // https://github.com/google-gemini/gemini-cli/pull/6656

                    #[derive(Deserialize)]
                    #[serde(deny_unknown_fields)]
                    struct ErrorDetails {
                        details: Box<str>,
                    }

                    match serde_json::from_value(data.clone()) {
                        Ok(ErrorDetails { details }) => {
                            if suppress_abort_err
                                && (details.contains("This operation was aborted")
                                    || details.contains("The user aborted a request"))
                            {
                                Ok(acp::PromptResponse::new(acp::StopReason::Cancelled))
                            } else {
                                Err(anyhow!(details))
                            }
                        }
                        Err(_) => Err(anyhow!(err)),
                    }
                }
            }
        })
    }

    fn cancel(&self, session_id: &acp_v2::SessionId, _cx: &mut App) -> Result<()> {
        let params = acp::CancelNotification::new(acp::SessionId::new(session_id.0.clone()));
        self.connection
            .send_notification(params)
            .map_err(map_acp_error)?;
        if let Some(session) = self.sessions.borrow_mut().get_mut(session_id) {
            session.suppress_abort_err = true;
        }
        Ok(())
    }

    fn request_elicitations(&self) -> Option<Entity<ElicitationStore>> {
        Some(self.request_elicitations.clone())
    }

    fn session_modes(
        &self,
        session_id: &acp_v2::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionModes>> {
        let sessions = self.sessions.clone();
        let sessions_ref = sessions.borrow();
        let Some(session) = sessions_ref.get(session_id) else {
            return None;
        };

        if let Some(modes) = session.session_modes.as_ref() {
            Some(Rc::new(AcpSessionModes {
                connection: self.connection.clone(),
                session_id: session_id.clone(),
                state: modes.clone(),
            }) as _)
        } else {
            None
        }
    }

    fn session_config_options(
        &self,
        session_id: &acp_v2::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionConfigOptions>> {
        let sessions = self.sessions.borrow();
        let session = sessions.get(session_id)?;

        let config_opts = session.config_options.as_ref()?;

        Some(Rc::new(AcpSessionConfigOptions {
            session_id: session_id.clone(),
            connection: self.connection.clone(),
            config_options: config_opts.clone(),
        }) as _)
    }

    fn session_list(&self, _cx: &mut App) -> Option<Rc<dyn AgentSessionList>> {
        self.session_list.clone().map(|s| s as _)
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use acp_thread::{
        AgentSessionClientUserMessageIds, AgentSessionConfigOptions, AgentSessionModes,
        AgentSessionRetry, AgentSessionSetTitle, AgentSessionTruncate, AgentTelemetry,
    };

    use super::*;
    use futures::StreamExt as _;

    #[derive(Clone, Default)]
    pub struct FakeAcpAgentServer {
        load_session_count: Arc<AtomicUsize>,
        close_session_count: Arc<AtomicUsize>,
        fail_next_prompt: Arc<AtomicBool>,
        auth_elicitation_request: Arc<Mutex<Option<acp::CreateElicitationRequest>>>,
        auth_elicitation_response:
            Arc<Mutex<Option<async_channel::Sender<acp::CreateElicitationResponse>>>>,
        auth_elicitation_completion: Arc<Mutex<Option<acp::CompleteElicitationNotification>>>,
        exit_status_sender:
            Arc<std::sync::Mutex<Option<async_channel::Sender<std::process::ExitStatus>>>>,
    }

    impl FakeAcpAgentServer {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn load_session_count(&self) -> Arc<AtomicUsize> {
            self.load_session_count.clone()
        }

        pub fn close_session_count(&self) -> Arc<AtomicUsize> {
            self.close_session_count.clone()
        }

        pub fn simulate_server_exit(&self) {
            let sender = self
                .exit_status_sender
                .lock()
                .expect("exit status sender lock should not be poisoned")
                .clone()
                .expect("fake ACP server must be connected before simulating exit");
            sender
                .try_send(std::process::ExitStatus::default())
                .expect("fake ACP server exit receiver should still be alive");
        }

        pub fn fail_next_prompt(&self) {
            self.fail_next_prompt.store(true, Ordering::SeqCst);
        }

        pub fn request_elicitation_during_auth(
            &self,
            request: acp::CreateElicitationRequest,
        ) -> async_channel::Receiver<acp::CreateElicitationResponse> {
            let (response_tx, response_rx) = async_channel::bounded(1);
            *self
                .auth_elicitation_request
                .lock()
                .expect("auth elicitation request lock should not be poisoned") = Some(request);
            *self
                .auth_elicitation_response
                .lock()
                .expect("auth elicitation response lock should not be poisoned") =
                Some(response_tx);
            response_rx
        }
    }

    impl crate::AgentServer for FakeAcpAgentServer {
        fn logo(&self) -> ui::IconName {
            ui::IconName::ZedAgent
        }

        fn agent_id(&self) -> AgentId {
            AgentId::new("Test")
        }

        fn connect(
            &self,
            _delegate: crate::AgentServerDelegate,
            project: Entity<Project>,
            cx: &mut App,
        ) -> Task<anyhow::Result<Rc<dyn AgentConnection>>> {
            let load_session_count = self.load_session_count.clone();
            let close_session_count = self.close_session_count.clone();
            let fail_next_prompt = self.fail_next_prompt.clone();
            let auth_elicitation_request = self.auth_elicitation_request.clone();
            let auth_elicitation_response = self.auth_elicitation_response.clone();
            let auth_elicitation_completion = self.auth_elicitation_completion.clone();
            let exit_status_sender = self.exit_status_sender.clone();
            cx.spawn(async move |cx| {
                let harness = build_fake_acp_connection(
                    project,
                    load_session_count,
                    close_session_count,
                    fail_next_prompt,
                    auth_elicitation_request,
                    auth_elicitation_response,
                    auth_elicitation_completion,
                    cx,
                )
                .await?;
                let (exit_tx, exit_rx) = async_channel::bounded(1);
                *exit_status_sender
                    .lock()
                    .expect("exit status sender lock should not be poisoned") = Some(exit_tx);
                let connection = harness.connection.clone();
                let simulate_exit_task = cx.spawn(async move |cx| {
                    while let Ok(status) = exit_rx.recv().await {
                        emit_load_error_to_all_sessions(
                            &connection.sessions,
                            LoadError::Exited {
                                status,
                                stderr: None,
                            },
                            cx,
                        );
                    }
                    Ok(())
                });
                Ok(Rc::new(FakeAcpAgentConnection {
                    inner: harness.connection,
                    _keep_agent_alive: harness.keep_agent_alive,
                    _simulate_exit_task: simulate_exit_task,
                }) as Rc<dyn AgentConnection>)
            })
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    pub struct FakeAcpConnectionHarness {
        pub connection: Rc<AcpConnection>,
        pub load_session_count: Arc<AtomicUsize>,
        pub close_session_count: Arc<AtomicUsize>,
        pub authenticate_count: Arc<AtomicUsize>,
        pub logout_count: Arc<AtomicUsize>,
        pub keep_agent_alive: Task<anyhow::Result<()>>,
    }

    struct FakeAcpAgentConnection {
        inner: Rc<AcpConnection>,
        _keep_agent_alive: Task<anyhow::Result<()>>,
        _simulate_exit_task: Task<anyhow::Result<()>>,
    }

    impl AgentConnection for FakeAcpAgentConnection {
        fn agent_id(&self) -> AgentId {
            self.inner.agent_id()
        }

        fn telemetry_id(&self) -> SharedString {
            self.inner.telemetry_id()
        }

        fn agent_version(&self) -> Option<SharedString> {
            self.inner.agent_version()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            self.inner.clone().new_session(project, work_dirs, cx)
        }

        fn supports_load_session(&self) -> bool {
            self.inner.supports_load_session()
        }

        fn load_session(
            self: Rc<Self>,
            session_id: acp_v2::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            self.inner
                .clone()
                .load_session(session_id, project, work_dirs, title, cx)
        }

        fn supports_resume_session(&self) -> bool {
            self.inner.supports_resume_session()
        }

        fn supports_session_additional_directories(&self) -> bool {
            self.inner.supports_session_additional_directories()
        }

        fn resume_session(
            self: Rc<Self>,
            session_id: acp_v2::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            self.inner
                .clone()
                .resume_session(session_id, project, work_dirs, title, cx)
        }

        fn auth_methods(&self) -> &[acp_v2::AuthMethod] {
            self.inner.auth_methods()
        }

        fn terminal_auth_task(
            &self,
            method: &acp_v2::AuthMethodId,
            cx: &App,
        ) -> Option<Task<Result<SpawnInTerminal>>> {
            self.inner.terminal_auth_task(method, cx)
        }

        fn authenticate(&self, method: acp_v2::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
            self.inner.authenticate(method, cx)
        }

        fn supports_logout(&self) -> bool {
            self.inner.supports_logout()
        }

        fn logout(&self, cx: &mut App) -> Task<Result<()>> {
            self.inner.logout(cx)
        }

        fn client_user_message_ids(
            &self,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionClientUserMessageIds>> {
            self.inner.client_user_message_ids(cx)
        }

        fn validate_prompt_content(&self, content: &[acp_v2::ContentBlock]) -> Result<()> {
            self.inner.validate_prompt_content(content)
        }

        fn prompt(
            &self,
            params: acp_v2::PromptRequest,
            cx: &mut App,
        ) -> Task<Result<acp::PromptResponse>> {
            self.inner.prompt(params, cx)
        }

        fn retry(
            &self,
            session_id: &acp_v2::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionRetry>> {
            self.inner.retry(session_id, cx)
        }

        fn cancel(&self, session_id: &acp_v2::SessionId, cx: &mut App) -> Result<()> {
            self.inner.cancel(session_id, cx)
        }

        fn request_elicitations(&self) -> Option<Entity<ElicitationStore>> {
            self.inner.request_elicitations()
        }

        fn truncate(
            &self,
            session_id: &acp_v2::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionTruncate>> {
            self.inner.truncate(session_id, cx)
        }

        fn set_title(
            &self,
            session_id: &acp_v2::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionSetTitle>> {
            self.inner.set_title(session_id, cx)
        }

        fn telemetry(&self) -> Option<Rc<dyn AgentTelemetry>> {
            self.inner.telemetry()
        }

        fn session_modes(
            &self,
            session_id: &acp_v2::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionModes>> {
            self.inner.session_modes(session_id, cx)
        }

        fn session_config_options(
            &self,
            session_id: &acp_v2::SessionId,
            cx: &App,
        ) -> Option<Rc<dyn AgentSessionConfigOptions>> {
            self.inner.session_config_options(session_id, cx)
        }

        fn session_list(&self, cx: &mut App) -> Option<Rc<dyn AgentSessionList>> {
            self.inner.session_list(cx)
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    async fn build_fake_acp_connection(
        project: Entity<Project>,
        load_session_count: Arc<AtomicUsize>,
        close_session_count: Arc<AtomicUsize>,
        fail_next_prompt: Arc<AtomicBool>,
        auth_elicitation_request: Arc<Mutex<Option<acp::CreateElicitationRequest>>>,
        auth_elicitation_response: Arc<
            Mutex<Option<async_channel::Sender<acp::CreateElicitationResponse>>>,
        >,
        auth_elicitation_completion: Arc<Mutex<Option<acp::CompleteElicitationNotification>>>,
        cx: &mut AsyncApp,
    ) -> Result<FakeAcpConnectionHarness> {
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();

        let authenticate_count = Arc::new(AtomicUsize::new(0));
        let logout_count = Arc::new(AtomicUsize::new(0));
        let sessions: Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>> =
            Rc::new(RefCell::new(HashMap::default()));
        let client_session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>> =
            Rc::new(RefCell::new(None));

        let agent_future = Agent
            .builder()
            .name("fake-agent")
            .on_receive_request(
                async move |req: acp::InitializeRequest, responder, _cx| {
                    responder.respond(
                        acp::InitializeResponse::new(req.protocol_version)
                            .agent_capabilities(
                                acp::AgentCapabilities::default()
                                    .load_session(true)
                                    .session_capabilities(
                                        acp::SessionCapabilities::default()
                                            .close(acp::SessionCloseCapabilities::new()),
                                    ),
                            )
                            .auth_methods(vec![acp::AuthMethod::Agent(acp::AuthMethodAgent::new(
                                "login", "Login",
                            ))]),
                    )
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let authenticate_count = authenticate_count.clone();
                    let auth_elicitation_request = auth_elicitation_request.clone();
                    let auth_elicitation_response = auth_elicitation_response.clone();
                    let auth_elicitation_completion = auth_elicitation_completion.clone();
                    async move |req: acp::AuthenticateRequest, responder, cx| {
                        assert_eq!(req.method_id, acp::AuthMethodId::new("login"));
                        authenticate_count.fetch_add(1, Ordering::SeqCst);
                        let request = auth_elicitation_request
                            .lock()
                            .expect("auth elicitation request lock should not be poisoned")
                            .take();
                        let response_tx = auth_elicitation_response
                            .lock()
                            .expect("auth elicitation response lock should not be poisoned")
                            .take();
                        let completion = auth_elicitation_completion
                            .lock()
                            .expect("auth elicitation completion lock should not be poisoned")
                            .take();

                        if let Some(request) = request {
                            cx.send_request(request)
                                .on_receiving_result(async move |result| {
                                    if let (Ok(response), Some(response_tx)) = (result, response_tx)
                                    {
                                        response_tx.send(response).await.ok();
                                    }
                                    responder.respond(Default::default())
                                })?;
                            if let Some(completion) = completion {
                                cx.send_notification(completion)?;
                            }
                            Ok(())
                        } else {
                            responder.respond(Default::default())
                        }
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::NewSessionRequest, responder, _cx| {
                    responder.respond(acp::NewSessionResponse::new(acp::SessionId::new("unused")))
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let fail_next_prompt = fail_next_prompt.clone();
                    async move |_req: acp::PromptRequest, responder, _cx| {
                        if fail_next_prompt.swap(false, Ordering::SeqCst) {
                            responder.respond_with_error(acp::ErrorCode::InternalError.into())
                        } else {
                            responder.respond(acp::PromptResponse::new(acp::StopReason::EndTurn))
                        }
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let load_session_count = load_session_count.clone();
                    async move |_req: acp::LoadSessionRequest, responder, _cx| {
                        load_session_count.fetch_add(1, Ordering::SeqCst);
                        responder.respond(acp::LoadSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let close_session_count = close_session_count.clone();
                    async move |_req: acp::CloseSessionRequest, responder, _cx| {
                        close_session_count.fetch_add(1, Ordering::SeqCst);
                        responder.respond(acp::CloseSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let logout_count = logout_count.clone();
                    async move |_req: acp::LogoutRequest, responder, _cx| {
                        logout_count.fetch_add(1, Ordering::SeqCst);
                        responder.respond(acp::LogoutResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_notification(
                async move |_notif: acp::CancelNotification, _cx| Ok(()),
                agent_client_protocol::on_receive_notification!(),
            )
            .connect_to(agent_transport);

        let agent_io_task = cx.background_spawn(agent_future);

        // Wire the production handler set into the fake client so inbound
        // requests/notifications from the fake agent are dispatched the
        // same way the real `stdio` path does.
        let (dispatch_tx, dispatch_rx) = mpsc::unbounded::<ForegroundWork>();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let client_future = connect_client_future(
            "zed-test",
            client_transport,
            dispatch_tx.clone(),
            connection_tx,
            futures::future::pending(),
        );
        let client_io_task = cx.background_spawn(async move {
            client_future.await.ok();
        });

        let client_conn: ConnectionTo<Agent> = connection_rx
            .await
            .context("failed to receive fake ACP connection handle")?;

        let response = client_conn
            .send_request(acp::InitializeRequest::new(ProtocolVersion::V1))
            .block_task()
            .await?;

        let agent_capabilities = response.agent_capabilities;
        let auth_methods = acp_thread::auth_methods::from_v1(response.auth_methods)?;

        let request_elicitations = cx.new(|_| ElicitationStore::default());
        let dispatch_context = ClientContext {
            sessions: sessions.clone(),
            session_list: client_session_list.clone(),
            request_elicitations: request_elicitations.clone(),
            active_request_scopes: None,
        };
        let dispatch_task = cx.spawn({
            let mut dispatch_rx = dispatch_rx;
            async move |cx| {
                while let Some(work) = dispatch_rx.next().await {
                    work.run(cx, &dispatch_context);
                }
            }
        });

        let agent_server_store =
            project.read_with(cx, |project, _| project.agent_server_store().downgrade());

        let connection = cx.update(|cx| {
            AcpConnection::new_for_test(
                client_conn,
                sessions,
                agent_capabilities,
                request_elicitations,
                agent_server_store,
                client_io_task,
                dispatch_tx,
                dispatch_task,
                cx,
            )
        });

        let keep_agent_alive = cx.background_spawn(async move {
            agent_io_task.await.ok();
            anyhow::Ok(())
        });

        let mut connection = connection;
        connection.auth_methods = auth_methods;
        Ok(FakeAcpConnectionHarness {
            connection: Rc::new(connection),
            load_session_count,
            close_session_count,
            authenticate_count,
            logout_count,
            keep_agent_alive,
        })
    }

    pub async fn connect_fake_acp_connection(
        project: Entity<Project>,
        cx: &mut gpui::TestAppContext,
    ) -> FakeAcpConnectionHarness {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        build_fake_acp_connection(
            project,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(None)),
            &mut cx.to_async(),
        )
        .await
        .expect("failed to initialize ACP connection")
    }

    #[cfg(test)]
    pub async fn connect_fake_acp_connection_with_auth_elicitation(
        project: Entity<Project>,
        request: acp::CreateElicitationRequest,
        cx: &mut gpui::TestAppContext,
    ) -> (
        FakeAcpConnectionHarness,
        async_channel::Receiver<acp::CreateElicitationResponse>,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let (response_tx, response_rx) = async_channel::bounded(1);
        let harness = build_fake_acp_connection(
            project,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(Some(request))),
            Arc::new(Mutex::new(Some(response_tx))),
            Arc::new(Mutex::new(None)),
            &mut cx.to_async(),
        )
        .await
        .expect("failed to initialize ACP connection");

        (harness, response_rx)
    }

    #[cfg(test)]
    pub async fn connect_fake_acp_connection_with_auth_elicitation_completion(
        project: Entity<Project>,
        request: acp::CreateElicitationRequest,
        completion: acp::CompleteElicitationNotification,
        cx: &mut gpui::TestAppContext,
    ) -> (
        FakeAcpConnectionHarness,
        async_channel::Receiver<acp::CreateElicitationResponse>,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let (response_tx, response_rx) = async_channel::bounded(1);
        let harness = build_fake_acp_connection(
            project,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(Some(request))),
            Arc::new(Mutex::new(Some(response_tx))),
            Arc::new(Mutex::new(Some(completion))),
            &mut cx.to_async(),
        )
        .await
        .expect("failed to initialize ACP connection");

        (harness, response_rx)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::super::*;
    use super::*;
    use futures::StreamExt as _;
    use settings::Settings as _;

    #[test]
    fn test_auth_error_mapping_retains_sdk_record_and_description() {
        let original = acp::Error::new(-32000, "Sign in again")
            .data(serde_json::json!({"detail": {"retain": [null, 7]}}));
        let mapped = map_acp_error(original.clone());
        assert_eq!(
            mapped.downcast_ref::<agent_client_protocol::Error>(),
            Some(&original),
        );
        assert_eq!(
            mapped
                .downcast_ref::<AuthRequired>()
                .expect("authentication marker")
                .description
                .as_deref(),
            Some("Sign in again"),
        );
    }

    #[gpui::test]
    async fn prompt_auth_error_retains_sdk_record_and_classification(
        cx: &mut gpui::TestAppContext,
    ) {
        init_settings_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;
        let mut harness = test_support::connect_fake_acp_connection(project.clone(), cx).await;
        let response_error = Arc::new(Mutex::new(acp::Error::auth_required()));
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        let _agent_io_task = cx.background_spawn(
            Agent
                .builder()
                .name("prompt-auth-test-agent")
                .on_receive_request(
                    async move |_request: acp::NewSessionRequest, responder, _cx| {
                        responder.respond(acp::NewSessionResponse::new("auth-session"))
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .on_receive_request(
                    {
                        let response_error = response_error.clone();
                        async move |request: acp::PromptRequest, responder, _cx| {
                            assert_eq!(request.session_id, acp::SessionId::new("auth-session"));
                            responder.respond_with_error(
                                response_error.lock().expect("response error").clone(),
                            )
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        );
        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let _client_io_task = cx.background_spawn(connect_client_future(
            "prompt-auth-test-client",
            client_transport,
            harness.connection.dispatch_tx.clone(),
            connection_tx,
            futures::future::pending(),
        ));
        Rc::get_mut(&mut harness.connection)
            .expect("test harness should own the only ACP connection handle")
            .connection = connection_rx.await.expect("receive SDK connection");

        let thread = cx
            .update(|cx| {
                harness.connection.clone().new_session(
                    project,
                    PathList::new(&[std::path::PathBuf::from("/a")]),
                    cx,
                )
            })
            .await
            .expect("registered session");
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        for data in [
            serde_json::json!({"auth": {"provider": "test", "retry": [null, false, 7]}}),
            serde_json::json!({"details": "The user aborted a request"}),
        ] {
            let original = acp::Error::new(
                acp::ErrorCode::AuthRequired.into(),
                "Sign in to continue this prompt",
            )
            .data(data);
            *response_error.lock().expect("response error") = original.clone();
            cx.update(|cx| harness.connection.cancel(&session_id, cx))
                .expect("cancellation succeeds");
            assert!(
                harness
                    .connection
                    .sessions
                    .borrow()
                    .get(&session_id)
                    .expect("registered session")
                    .suppress_abort_err
            );
            let error = cx
                .update(|cx| {
                    harness.connection.prompt(
                        acp_v2::PromptRequest::new(session_id.clone(), vec!["Hello".into()]),
                        cx,
                    )
                })
                .await
                .expect_err("authentication errors must not be suppressed by cancellation")
                .context("sending the initial prompt");
            assert_eq!(
                error.downcast_ref::<agent_client_protocol::Error>(),
                Some(&original),
            );
            assert_eq!(
                AuthRequired::from_error(&error)
                    .expect("initial prompt should be classified as authentication required")
                    .description
                    .as_deref(),
                Some("Sign in to continue this prompt"),
            );
            assert!(
                !harness
                    .connection
                    .sessions
                    .borrow()
                    .get(&session_id)
                    .expect("registered session")
                    .suppress_abort_err
            );
        }
    }

    #[test]
    fn test_legacy_modes_use_config_choices_without_overriding_config_options() {
        let id = acp::SessionModeId::new("opaque/手動");
        let meta =
            acp::Meta::from_iter([("custom".into(), serde_json::json!({"keep": [null, 7]}))]);
        let mode = acp::SessionMode::new(id.clone(), "Manual label")
            .description("Mode description")
            .meta(meta.clone());
        let choice = mode_from_v1(&mode);
        assert!(Arc::ptr_eq(&choice.value.0, &id.0));
        assert_eq!(choice.name, mode.name);
        assert_eq!(choice.description, mode.description);
        assert_eq!(choice.meta, Some(meta));

        let modes =
            acp::SessionModeState::new(id, vec![mode, acp::SessionMode::new("auto", "Auto")]);
        let (legacy, config) = config_state(Some(modes.clone()), None).expect("legacy modes");
        assert!(config.is_none());
        let choices = legacy
            .expect("legacy fallback")
            .borrow()
            .available_modes
            .iter()
            .map(mode_from_v1)
            .map(|choice| choice.value.to_string())
            .collect::<Vec<_>>();
        assert_eq!(choices, ["opaque/手動", "auto"]);

        let (legacy, config) =
            config_state(Some(modes), Some(Vec::new())).expect("config options take precedence");
        assert!(legacy.is_none());
        assert!(config.is_some());
    }

    #[test]
    fn test_process_terminal_responses_preserve_v1_payloads() {
        use acp_thread::ProcessOutput;
        use serde_json::json;

        let pending = process_output_to_v1(ProcessOutput::new("partial\n", false));
        assert_eq!(
            serde_json::to_value(pending).expect("pending output response"),
            json!({"output": "partial\n", "truncated": false}),
        );
        let unknown_exit = process_output_to_v1(
            ProcessOutput::new("", false).exit_status(acp_v2::TerminalExitStatus::new()),
        );
        assert_eq!(
            serde_json::to_value(unknown_exit).expect("unknown exit response"),
            json!({"output": "", "truncated": false, "exitStatus": {}}),
        );

        let meta =
            acp_v2::Meta::from_iter([("custom".into(), json!({"values": [null, 7, "keep"]}))]);
        let status = acp_v2::TerminalExitStatus::new()
            .exit_code(u32::MAX)
            .signal("SIG_CUSTOM")
            .meta(meta);
        let expected_status = json!({
            "exitCode": u32::MAX,
            "signal": "SIG_CUSTOM",
            "_meta": {"custom": {"values": [null, 7, "keep"]}},
        });
        let output =
            process_output_to_v1(ProcessOutput::new("héllo\n", true).exit_status(status.clone()));
        assert_eq!(
            serde_json::to_value(output).expect("completed output response"),
            json!({"output": "héllo\n", "truncated": true, "exitStatus": expected_status}),
        );
        let wait = acp::WaitForTerminalExitResponse::new(terminal_exit_status_to_v1(status));
        assert_eq!(
            serde_json::to_value(wait).expect("wait response"),
            expected_status,
        );
    }

    #[test]
    fn test_v1_permission_conversion_preserves_options_and_arc_ids() {
        for (legacy_kind, kind) in [
            (
                acp::PermissionOptionKind::AllowOnce,
                acp_v2::PermissionOptionKind::AllowOnce,
            ),
            (
                acp::PermissionOptionKind::AllowAlways,
                acp_v2::PermissionOptionKind::AllowAlways,
            ),
            (
                acp::PermissionOptionKind::RejectOnce,
                acp_v2::PermissionOptionKind::RejectOnce,
            ),
            (
                acp::PermissionOptionKind::RejectAlways,
                acp_v2::PermissionOptionKind::RejectAlways,
            ),
        ] {
            let option_id = acp::PermissionOptionId::new("agent-defined-choice");
            let meta = acp::Meta::from_iter([("custom".into(), serde_json::json!({"keep": true}))]);
            let option = permission_option_from_v1(
                acp::PermissionOption::new(option_id.clone(), "Agent's label", legacy_kind)
                    .meta(meta.clone()),
            )
            .expect("known permission option kind");
            assert!(Arc::ptr_eq(&option.option_id.0, &option_id.0));
            assert_eq!(option.name, "Agent's label");
            assert_eq!(option.kind, kind);
            assert_eq!(option.meta, Some(meta));

            let outcome = permission_outcome_to_v1(acp_thread::RequestPermissionOutcome::Selected(
                acp_thread::SelectedPermissionOutcome::new(option.option_id, kind),
            ));
            let acp::RequestPermissionOutcome::Selected(outcome) = outcome else {
                panic!("selected permission outcome");
            };
            assert!(Arc::ptr_eq(&outcome.option_id.0, &option_id.0));
            assert_eq!(outcome.option_id, option_id);
        }
    }

    #[test]
    fn test_v1_permission_conversion_preserves_cancellation() {
        for outcome in [
            acp_thread::RequestPermissionOutcome::Cancelled,
            acp_thread::RequestPermissionOutcome::InterruptedByFollowUp,
        ] {
            assert_eq!(
                permission_outcome_to_v1(outcome),
                acp::RequestPermissionOutcome::Cancelled,
            );
        }
    }

    #[derive(Debug, PartialEq)]
    struct V2TerminalReceive {
        envelope_meta: Option<acp_v2::Meta>,
        chunk_meta: Option<acp_v2::Meta>,
        bytes: Option<Vec<u8>>,
    }

    struct V2SessionHarness {
        connection: Rc<AcpConnection>,
        thread: Entity<AcpThread>,
        agent: V2ConnectionTo<Client>,
        received: Arc<Mutex<Vec<V2TerminalReceive>>>,
        fallback: Arc<Mutex<Vec<agent_client_protocol::UntypedMessage>>>,
        acknowledgment: Arc<Mutex<Option<futures::channel::oneshot::Sender<()>>>>,
        _client_task: Task<Result<(), agent_client_protocol::Error>>,
        _agent_task: Task<Result<(), agent_client_protocol::Error>>,
        _legacy_agent_task: Task<Result<()>>,
    }

    fn v2_terminal_meta(scope: &str) -> acp_v2::Meta {
        acp_v2::Meta::from_iter([("scope".to_owned(), serde_json::json!(scope))])
    }

    async fn v2_response<Response: 'static>(
        response: Task<Result<Response, acp::Error>>,
        cx: &mut gpui::TestAppContext,
        description: &str,
    ) -> Result<Response, acp::Error> {
        let timeout = cx
            .background_executor
            .timer(std::time::Duration::from_secs(5));
        futures::pin_mut!(response, timeout);
        match futures::future::select(response, timeout).await {
            futures::future::Either::Left((result, _)) => result,
            futures::future::Either::Right(_) => panic!("timed out waiting for {description}"),
        }
    }

    impl V2SessionHarness {
        fn request<Request: agent_client_protocol::JsonRpcRequest>(
            &self,
            request: Request,
            cx: &mut gpui::TestAppContext,
        ) -> (
            Task<Result<Request::Response, acp::Error>>,
            agent_client_protocol::RequestCancellationHandle,
        )
        where
            Request::Response: Send + 'static,
        {
            let request = self.agent.send_request(request);
            let cancellation = request.cancellation_handle();
            (cx.background_spawn(request.block_task()), cancellation)
        }

        fn elicitation_id(
            &self,
            session_scoped: bool,
            cx: &gpui::TestAppContext,
        ) -> acp_thread::ElicitationEntryId {
            if session_scoped {
                self.thread.read_with(cx, |thread, _| {
                    let Some(acp_thread::AgentThreadEntry::Elicitation(id)) =
                        thread.entries().last()
                    else {
                        panic!("session elicitation entry");
                    };
                    id.clone()
                })
            } else {
                self.connection
                    .request_elicitations
                    .read_with(cx, |store, _| {
                        store
                            .elicitations()
                            .last()
                            .expect("request elicitation")
                            .id
                            .clone()
                    })
            }
        }

        fn respond_to_elicitation(
            &self,
            session_scoped: bool,
            id: &acp_thread::ElicitationEntryId,
            response: acp_v2::CreateElicitationResponse,
            cx: &mut gpui::TestAppContext,
        ) {
            if session_scoped {
                self.thread.update(cx, |thread, cx| {
                    thread.respond_to_elicitation(id, response, cx);
                });
            } else {
                self.connection
                    .request_elicitations
                    .update(cx, |store, cx| {
                        store.respond_to_elicitation(id, response, cx);
                    });
            }
        }

        async fn new(cx: &mut gpui::TestAppContext) -> Self {
            let (connection, project, _, _, _, _, legacy_agent_task) =
                connect_fake_agent(None, cx).await;
            let thread = cx
                .update(|cx| {
                    connection.clone().load_session(
                        acp_v2::SessionId::new("v2-terminal-session"),
                        project,
                        PathList::new(&[std::path::Path::new("/a")]),
                        None,
                        cx,
                    )
                })
                .await
                .expect("load the existing model session");

            let received = Arc::new(Mutex::new(Vec::new()));
            let fallback = Arc::new(Mutex::new(Vec::new()));
            let acknowledgment =
                Arc::new(Mutex::new(None::<futures::channel::oneshot::Sender<()>>));
            let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
            let (client_sender, client_receiver) = futures::channel::oneshot::channel();
            let observed_builder = Client.v2().on_receive_notification(
                {
                    let received = received.clone();
                    async move |message: UntypedMessage, connection: V2ConnectionTo<Agent>| {
                        if acp_v2::UpdateSessionNotification::matches_method(message.method()) {
                            let decoded = terminal_updates::parse_terminal_notification(&message)
                                .map_err(anyhow::Error::from)
                                .and_then(terminal_updates::decode_terminal_notification);
                            match decoded {
                                Ok(Some(notification)) => {
                                    let (chunk_meta, bytes) = match notification.update {
                                        DecodedTerminalUpdate::Patch { .. } => (None, None),
                                        DecodedTerminalUpdate::Output { data, meta, .. } => {
                                            (meta, Some(data))
                                        }
                                    };
                                    received.lock().expect("receive mutex").push(
                                        V2TerminalReceive {
                                            envelope_meta: notification.meta,
                                            chunk_meta,
                                            bytes,
                                        },
                                    );
                                }
                                Ok(None) => {}
                                Err(error) => {
                                    log::debug!("Terminal observation was not decoded: {error:#}");
                                }
                            }
                        }
                        // Only the production binding may claim and apply the update.
                        Ok(Handled::No {
                            message: (message, connection),
                            retry: false,
                        })
                    }
                },
                agent_client_protocol::on_receive_notification!(),
            );
            let client_builder = connection
                .with_v2_client_ingress(observed_builder)
                .on_receive_notification(
                    {
                        let fallback = fallback.clone();
                        let acknowledgment = acknowledgment.clone();
                        async move |message: agent_client_protocol::UntypedMessage,
                                    _connection: V2ConnectionTo<Agent>| {
                            if message.method() == "_test/session_barrier" {
                                acknowledgment
                                    .lock()
                                    .expect("acknowledgment mutex")
                                    .take()
                                    .expect("installed receive barrier")
                                    .send(())
                                    .expect("waiting receive barrier");
                            } else {
                                fallback.lock().expect("fallback mutex").push(message);
                            }
                            Ok(())
                        }
                    },
                    agent_client_protocol::on_receive_notification!(),
                );
            let client_task = cx.background_spawn(client_builder.connect_with(
                client_transport,
                async move |client| {
                    assert!(client_sender.send(client).is_ok(), "receive v2 client");
                    futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                },
            ));
            let (agent_sender, agent_receiver) = futures::channel::oneshot::channel();
            let agent_task = cx.background_spawn(
                Agent
                    .v2()
                    .on_receive_request(
                        async move |request: acp_v2::InitializeRequest, responder, _connection| {
                            assert_eq!(request.protocol_version, ProtocolVersion::V2);
                            responder.respond(acp_v2::InitializeResponse::new(
                                ProtocolVersion::V2,
                                acp_v2::Implementation::new("terminal-test-agent", "1.0.0"),
                            ))
                        },
                        agent_client_protocol::on_receive_request!(),
                    )
                    .connect_with(agent_transport, async move |agent| {
                        assert!(agent_sender.send(agent).is_ok(), "receive v2 agent");
                        futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
                    }),
            );
            let client = client_receiver.await.expect("v2 client handle");
            let agent = agent_receiver.await.expect("v2 agent handle");
            let response = client
                .send_request(acp_v2::InitializeRequest::new(
                    ProtocolVersion::V2,
                    acp_v2::Implementation::new("terminal-test-client", "1.0.0"),
                ))
                .block_task()
                .await
                .expect("initialize the real SDK v2 channel");
            assert_eq!(response.protocol_version, ProtocolVersion::V2);

            Self {
                connection,
                thread,
                agent,
                received,
                fallback,
                acknowledgment,
                _client_task: client_task,
                _agent_task: agent_task,
                _legacy_agent_task: legacy_agent_task,
            }
        }

        async fn receipt_session(
            &self,
            session_id: &str,
            cx: &mut gpui::TestAppContext,
        ) -> Entity<AcpThread> {
            let (project, work_dirs) = self.thread.read_with(cx, |thread, _| {
                (
                    thread.project().clone(),
                    thread.work_dirs().cloned().expect("working directories"),
                )
            });
            let session_id = acp_v2::SessionId::new(session_id);
            let connection = Rc::new(
                acp_thread::StubAgentConnection::new()
                    .with_receipt_submissions(true)
                    .with_supports_load_session(true),
            );
            let thread = cx
                .update(|cx| {
                    connection.load_session(session_id.clone(), project, work_dirs, None, cx)
                })
                .await
                .expect("receipt-capable model session");
            cx.update(|cx| {
                self.connection
                    .register_session(session_id, &thread, None, None, None, cx);
            });
            thread
        }

        fn send(&self, update: serde_json::Value, envelope_meta: Option<acp_v2::Meta>) {
            self.send_to("v2-terminal-session", update, envelope_meta);
        }

        fn send_to(
            &self,
            session_id: &str,
            update: serde_json::Value,
            envelope_meta: Option<acp_v2::Meta>,
        ) {
            let update: acp_v2::SessionUpdate =
                serde_json::from_value(update).expect("valid SDK session update fixture");
            self.agent
                .send_notification(
                    acp_v2::UpdateSessionNotification::new(session_id, update).meta(envelope_meta),
                )
                .expect("send SDK v2 session notification");
        }

        async fn drain(&self, cx: &mut gpui::TestAppContext) {
            let (sender, receiver) = futures::channel::oneshot::channel();
            assert!(
                self.acknowledgment
                    .lock()
                    .expect("acknowledgment mutex")
                    .replace(sender)
                    .is_none()
            );
            self.agent
                .send_notification(
                    UntypedMessage::new("_test/session_barrier", serde_json::json!({}))
                        .expect("SDK barrier notification"),
                )
                .expect("send SDK barrier");
            // Notification errors have no reply. This separate packet acknowledges SDK
            // dispatch before the foreground barrier acknowledges model application.
            let delivered = async {
                receiver.await.expect("SDK receive barrier");
                drain_foreground_queue(&self.connection.dispatch_tx)
                    .await
                    .expect("foreground application barrier");
            };
            let timeout = cx
                .background_executor
                .timer(std::time::Duration::from_secs(5));
            futures::pin_mut!(delivered, timeout);
            if let futures::future::Either::Right(_) =
                futures::future::select(delivered, timeout).await
            {
                panic!("timed out receiving and applying v2 terminal notifications");
            }
        }

        fn terminal(&self, id: &str, cx: &gpui::TestAppContext) -> Entity<acp_thread::Terminal> {
            self.thread.read_with(cx, |thread, _| {
                thread
                    .terminal(acp_v2::TerminalId::new(id))
                    .expect("display terminal")
            })
        }

        async fn session_list_updates(
            &self,
        ) -> async_channel::Receiver<acp_thread::SessionListUpdate> {
            fn install(
                (connection, sender): (
                    ConnectionTo<Agent>,
                    futures::channel::oneshot::Sender<
                        async_channel::Receiver<acp_thread::SessionListUpdate>,
                    >,
                ),
                cx: &mut AsyncApp,
                context: &ClientContext,
            ) {
                let list = Rc::new(AcpSessionList::new(connection, false));
                let updates = cx.update(|cx| list.watch(cx).expect("list updates"));
                *context.session_list.borrow_mut() = Some(list);
                assert!(sender.send(updates).is_ok(), "list update receiver");
            }
            let (sender, receiver) = futures::channel::oneshot::channel();
            enqueue_notification(
                &self.connection.dispatch_tx,
                (self.connection.connection.clone(), sender),
                install,
            );
            receiver.await.expect("installed session list")
        }
    }

    #[gpui::test]
    async fn v2_permission_sdk_preserves_requests_and_returns_offered_outcomes(
        cx: &mut gpui::TestAppContext,
    ) {
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        for subject in [
            json!(null),
            json!({"type": "tool_call", "toolCall": {"toolCallId": "unseen",
                "title": "Subject title", "kind": "_future", "rawInput": {"retain": [null, 7]},
                "_meta": {"scope": "tool"}}}),
            json!({"type": "command", "command": "echo 'opaque'", "cwd": "/a",
                "toolCallId": "unseen", "terminalId": "unseen", "_meta": {"scope": "subject"}}),
            json!({"type": "_future", "payload": {"retain": [false, null]},
                "_meta": {"scope": "subject"}}),
        ] {
            let source = json!({"sessionId": "v2-terminal-session", "title": "Permission title",
                "description": "Separate explanation", "subject": subject,
                "options": [
                    {"optionId": "allow", "name": "Allow", "kind": "allow_once",
                        "_meta": {"scope": "allow"}},
                    {"optionId": "opaque", "name": "Unknown hint", "kind": "_future",
                        "_meta": {"scope": "option", "retain": [null, 7]}}
                ], "_meta": {"scope": "request"}});
            let request: acp_v2::RequestPermissionRequest =
                serde_json::from_value(source.clone()).expect("permission request");
            let (mut response, _cancellation) = harness.request(request.clone(), cx);
            harness.drain(cx).await;
            let id = harness.thread.read_with(cx, |thread, _| {
                let requests = thread.pending_permission_requests().collect::<Vec<_>>();
                let [permission] = requests.as_slice() else {
                    panic!("one generic permission");
                };
                assert_eq!(permission.generic_request(), Some(&request));
                assert!(permission.legacy_tool_call_id().is_none());
                assert!(
                    thread.entries().is_empty(),
                    "permission subjects do not create tools"
                );
                permission.id
            });
            assert!(
                (&mut response).now_or_never().is_none(),
                "no inferred approval"
            );
            harness.thread.update(cx, |thread, cx| {
                thread.select_permission_option(id, "not-offered".into(), cx);
            });
            cx.run_until_parked();
            assert!((&mut response).now_or_never().is_none());
            harness.thread.update(cx, |thread, cx| {
                thread.select_permission_option(id, "opaque".into(), cx);
            });
            let result = v2_response(response, cx, "permission selection")
                .await
                .expect("permission response");
            assert_eq!(
                result.outcome,
                acp_v2::RequestPermissionOutcome::Selected(acp_v2::SelectedPermissionOutcome::new(
                    "opaque"
                ))
            );
            assert!(
                result.meta.is_none(),
                "request metadata is not response metadata"
            );
        }
        for (session_id, options, expected_code) in [
            (
                "unknown",
                json!([{"optionId": "allow", "name": "Allow", "kind": "allow_once"}]),
                ErrorCode::InternalError,
            ),
            ("v2-terminal-session", json!([]), ErrorCode::InvalidParams),
            (
                "v2-terminal-session",
                json!([
                {"optionId": "same", "name": "Allow", "kind": "allow_once"},
                {"optionId": "same", "name": "Reject", "kind": "reject_once"}]),
                ErrorCode::InvalidParams,
            ),
        ] {
            let request = serde_json::from_value::<acp_v2::RequestPermissionRequest>(
                json!({"sessionId": session_id, "title": "Invalid", "options": options}),
            )
            .expect("typed invalid permission");
            let (response, _cancellation) = harness.request(request, cx);
            assert_eq!(
                v2_response(response, cx, "invalid permission response")
                    .await
                    .expect_err("reject invalid permission")
                    .code,
                expected_code
            );
        }
        harness.thread.read_with(cx, |thread, _| {
            assert_eq!(thread.pending_permission_requests().count(), 0);
            assert!(thread.entries().is_empty());
        });
    }

    #[gpui::test]
    async fn v2_permission_sdk_cancellation_is_entry_scoped_and_does_not_retain_owner(
        cx: &mut gpui::TestAppContext,
    ) {
        let harness = V2SessionHarness::new(cx).await;
        let request = acp_v2::RequestPermissionRequest::new(
            "v2-terminal-session",
            "Same subject",
            vec![acp_v2::PermissionOption::new(
                "allow",
                "Allow",
                acp_v2::PermissionOptionKind::AllowOnce,
            )],
        )
        .subject(acp_v2::RequestPermissionSubject::Command(
            acp_v2::CommandPermissionSubject::new("echo test", "/a"),
        ));
        let published = Rc::new(RefCell::new(0));
        let subscription = cx.update(|cx| {
            cx.subscribe(&harness.thread, {
                let published = published.clone();
                move |_, event, _| {
                    if matches!(
                        event,
                        acp_thread::AcpThreadEvent::ToolAuthorizationRequested(_)
                    ) {
                        *published.borrow_mut() += 1;
                    }
                }
            })
        });
        let (response, cancellation) = harness.request(request.clone(), cx);
        cancellation
            .cancel()
            .expect("cancel before foreground publication");
        drain_sdk_background_tasks(cx);
        harness.thread.read_with(cx, |thread, _| {
            assert_eq!(thread.pending_permission_requests().count(), 0);
        });
        assert_eq!(
            v2_response(response, cx, "pre-cancelled permission")
                .await
                .expect_err("RPC cancellation before registration")
                .code,
            ErrorCode::RequestCancelled
        );
        cx.run_until_parked();
        assert_eq!(
            *published.borrow(),
            0,
            "no transient permission publication"
        );
        drop(subscription);

        let (first, cancellation) = harness.request(request.clone(), cx);
        harness.drain(cx).await;
        let first_id = harness.thread.read_with(cx, |thread, _| {
            thread
                .pending_permission_requests()
                .next()
                .expect("first permission")
                .id
        });
        let (second, _second_cancellation) = harness.request(request.clone(), cx);
        harness.drain(cx).await;
        let second_id = harness.thread.read_with(cx, |thread, _| {
            thread
                .pending_permission_requests()
                .find(|request| request.id != first_id)
                .expect("same-subject successor")
                .id
        });
        cancellation.cancel().expect("cancel first SDK request");
        assert_eq!(
            v2_response(first, cx, "permission RPC cancellation")
                .await
                .expect_err("RPC cancellation")
                .code,
            ErrorCode::RequestCancelled
        );
        harness.thread.update(cx, |thread, cx| {
            assert!(thread.permission_request(first_id).is_none());
            assert!(thread.permission_request(second_id).is_some());
            thread.cancel_permission_request(second_id, cx);
        });
        assert_eq!(
            v2_response(second, cx, "local permission cancellation")
                .await
                .expect("local cancellation response")
                .outcome,
            acp_v2::RequestPermissionOutcome::Cancelled
        );

        let (response, _cancellation) = harness.request(request, cx);
        harness.drain(cx).await;
        let weak_thread = harness.thread.downgrade();
        let V2SessionHarness {
            connection,
            thread,
            agent,
            _client_task,
            _agent_task,
            _legacy_agent_task,
            ..
        } = harness;
        let _transports = (
            connection,
            agent,
            _client_task,
            _agent_task,
            _legacy_agent_task,
        );
        cx.update(|_| drop(thread));
        assert!(weak_thread.upgrade().is_none());
        assert_eq!(
            v2_response(response, cx, "permission owner-drop response")
                .await
                .expect("dropped-owner response")
                .outcome,
            acp_v2::RequestPermissionOutcome::Cancelled
        );
    }

    #[gpui::test]
    async fn v2_elicitation_sdk_routes_scopes_preserves_source_and_completes_urls(
        cx: &mut gpui::TestAppContext,
    ) {
        use acp_thread::{Elicitation, ElicitationStatus};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        for session_scoped in [false, true] {
            let scope: acp_v2::ElicitationScope = if session_scoped {
                acp_v2::ElicitationSessionScope::new("v2-terminal-session")
                    .tool_call_id("unseen")
                    .into()
            } else {
                acp_v2::ElicitationRequestScope::new(
                    serde_json::from_value::<acp_v2::RequestId>(json!("auth-request"))
                        .expect("string request ID"),
                )
                .into()
            };
            let schema = serde_json::from_value::<acp_v2::ElicitationSchema>(json!({
                "type": "object", "properties": {"name": {"type": "string",
                    "_meta": {"scope": "property", "retain": [null, 7]}}},
                "required": ["name"], "_meta": {"scope": "schema"}
            }))
            .expect("form schema");
            let request = acp_v2::CreateElicitationRequest::new(
                acp_v2::ElicitationFormMode::new(scope, schema),
                "Provide a name",
            )
            .meta(v2_terminal_meta("request"));
            let (response, _cancellation) = harness.request(request.clone(), cx);
            harness.drain(cx).await;
            let entry_id = harness.elicitation_id(session_scoped, cx);
            let check = |elicitation: &Elicitation| {
                assert_eq!(elicitation.request, request);
                assert!(matches!(
                    elicitation.status,
                    ElicitationStatus::Pending { .. }
                ));
            };
            if session_scoped {
                harness.thread.read_with(cx, |thread, _| {
                    check(thread.elicitation(&entry_id).expect("session entry").1);
                });
            } else {
                harness
                    .connection
                    .request_elicitations
                    .read_with(cx, |store, _| {
                        check(store.elicitation(&entry_id).expect("request entry").1);
                    });
                harness
                    .thread
                    .read_with(cx, |thread, _| assert!(thread.entries().is_empty()));
            }
            let expected = acp_v2::CreateElicitationResponse::new(
                acp_v2::ElicitationAction::Accept(acp_v2::ElicitationAcceptAction::new().content(
                    std::collections::BTreeMap::from([(
                        "name".to_owned(),
                        acp_v2::ElicitationContentValue::from("Ada"),
                    )]),
                )),
            )
            .meta(v2_terminal_meta("response"));
            harness.respond_to_elicitation(session_scoped, &entry_id, expected.clone(), cx);
            assert_eq!(
                v2_response(response, cx, "form response")
                    .await
                    .expect("form response"),
                expected
            );

            let scope: acp_v2::ElicitationScope = if session_scoped {
                acp_v2::ElicitationSessionScope::new("v2-terminal-session").into()
            } else {
                acp_v2::ElicitationRequestScope::new(acp_v2::RequestId::Number(99)).into()
            };
            let wire_id = if session_scoped {
                "session-url"
            } else {
                "request-url"
            };
            let request = acp_v2::CreateElicitationRequest::new(
                acp_v2::ElicitationUrlMode::new(scope, wire_id, "https://example.com/consent"),
                "Consent in browser",
            )
            .meta(v2_terminal_meta("url-request"));
            let (mut response, _cancellation) = harness.request(request, cx);
            harness.drain(cx).await;
            let entry_id = harness.elicitation_id(session_scoped, cx);
            let complete = acp_v2::CompleteElicitationNotification::new(wire_id)
                .meta(v2_terminal_meta("completion"));
            harness
                .agent
                .send_notification(complete.clone())
                .expect("complete before consent");
            harness.drain(cx).await;
            assert!((&mut response).now_or_never().is_none());
            let expected = acp_v2::CreateElicitationResponse::new(
                acp_v2::ElicitationAction::Accept(acp_v2::ElicitationAcceptAction::new()),
            );
            harness.respond_to_elicitation(session_scoped, &entry_id, expected.clone(), cx);
            assert_eq!(
                v2_response(response, cx, "URL consent response")
                    .await
                    .expect("URL consent response"),
                expected
            );
            harness
                .agent
                .send_notification(complete)
                .expect("complete after consent");
            harness.drain(cx).await;
            let check = |elicitation: &Elicitation| {
                assert!(matches!(elicitation.status, ElicitationStatus::Completed));
                assert_eq!(
                    elicitation.request.meta,
                    Some(v2_terminal_meta("url-request"))
                );
            };
            if session_scoped {
                harness.thread.read_with(cx, |thread, _| {
                    check(thread.elicitation(&entry_id).expect("session URL").1);
                });
            } else {
                harness
                    .connection
                    .request_elicitations
                    .read_with(cx, |store, _| {
                        check(store.elicitation(&entry_id).expect("request URL").1);
                    });
            }
        }
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_elicitation_sdk_cancellation_validation_and_owner_drop(
        cx: &mut gpui::TestAppContext,
    ) {
        use acp_thread::ElicitationStatus;
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        for session_scoped in [false, true] {
            let scope: acp_v2::ElicitationScope = if session_scoped {
                acp_v2::ElicitationSessionScope::new("v2-terminal-session").into()
            } else {
                acp_v2::ElicitationRequestScope::new(acp_v2::RequestId::Number(99)).into()
            };
            let request = acp_v2::CreateElicitationRequest::new(
                acp_v2::ElicitationUrlMode::new(
                    scope,
                    "same-wire-id",
                    "https://example.com/consent",
                ),
                "Same scope and URL ID",
            );
            let entry_count = harness
                .thread
                .read_with(cx, |thread, _| thread.entries().len());
            let request_count = harness
                .connection
                .request_elicitations
                .read_with(cx, |store, _| store.elicitations().len());
            let (response, cancellation) = harness.request(request.clone(), cx);
            cancellation
                .cancel()
                .expect("cancel before foreground publication");
            drain_sdk_background_tasks(cx);
            assert_eq!(
                v2_response(response, cx, "pre-cancelled elicitation")
                    .await
                    .expect_err("RPC cancellation before registration")
                    .code,
                ErrorCode::RequestCancelled
            );
            harness.thread.read_with(cx, |thread, _| {
                assert_eq!(thread.entries().len(), entry_count);
            });
            harness
                .connection
                .request_elicitations
                .read_with(cx, |store, _| {
                    assert_eq!(store.elicitations().len(), request_count);
                });

            let (first, cancellation) = harness.request(request.clone(), cx);
            harness.drain(cx).await;
            let first_id = harness.elicitation_id(session_scoped, cx);
            let (mut second, _second_cancellation) = harness.request(request, cx);
            harness.drain(cx).await;
            let second_id = harness.elicitation_id(session_scoped, cx);
            assert_ne!(first_id, second_id);
            cancellation.cancel().expect("cancel first elicitation RPC");
            assert_eq!(
                v2_response(first, cx, "elicitation RPC cancellation")
                    .await
                    .expect_err("RPC cancellation")
                    .code,
                ErrorCode::RequestCancelled
            );
            assert!((&mut second).now_or_never().is_none());
            let check = |first_status: &ElicitationStatus, second_status: &ElicitationStatus| {
                assert!(matches!(first_status, ElicitationStatus::Canceled));
                assert!(matches!(second_status, ElicitationStatus::Pending { .. }));
            };
            if session_scoped {
                harness.thread.read_with(cx, |thread, _| {
                    check(
                        &thread.elicitation(&first_id).expect("first entry").1.status,
                        &thread
                            .elicitation(&second_id)
                            .expect("second entry")
                            .1
                            .status,
                    );
                });
            } else {
                harness
                    .connection
                    .request_elicitations
                    .read_with(cx, |store, _| {
                        check(
                            &store.elicitation(&first_id).expect("first entry").1.status,
                            &store
                                .elicitation(&second_id)
                                .expect("second entry")
                                .1
                                .status,
                        );
                    });
            }
            let expected =
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Decline);
            harness.respond_to_elicitation(session_scoped, &second_id, expected.clone(), cx);
            assert_eq!(
                v2_response(second, cx, "successor response")
                    .await
                    .expect("successor response"),
                expected
            );
        }
        for (source, expected_code) in [
            (
                json!({"sessionId": "unknown", "mode": "form", "message": "Unknown session",
                "requestedSchema": {"type": "object", "properties": {}}}),
                ErrorCode::InternalError,
            ),
            (
                json!({"sessionId": "v2-terminal-session", "mode": "url", "message": "Unsafe URL",
                "elicitationId": "unsafe", "url": "file:///private/path"}),
                ErrorCode::InvalidParams,
            ),
            (
                json!({"requestId": 99, "mode": "_future", "message": "Unknown mode",
                "payload": {"retain": [null, 7]}}),
                ErrorCode::InvalidParams,
            ),
        ] {
            let request = serde_json::from_value::<acp_v2::CreateElicitationRequest>(source)
                .expect("typed invalid elicitation");
            let (response, _cancellation) = harness.request(request, cx);
            assert_eq!(
                v2_response(response, cx, "invalid elicitation response")
                    .await
                    .expect_err("invalid elicitation")
                    .code,
                expected_code
            );
        }
        let (session_response, _session_cancellation) = harness.request(
            acp_v2::CreateElicitationRequest::new(
                acp_v2::ElicitationFormMode::new(
                    acp_v2::ElicitationSessionScope::new("v2-terminal-session"),
                    acp_v2::ElicitationSchema::new(),
                ),
                "Pending session",
            ),
            cx,
        );
        let (mut request_response, _request_cancellation) = harness.request(
            acp_v2::CreateElicitationRequest::new(
                acp_v2::ElicitationFormMode::new(
                    acp_v2::ElicitationRequestScope::new(acp_v2::RequestId::Number(100)),
                    acp_v2::ElicitationSchema::new(),
                ),
                "Pending request",
            ),
            cx,
        );
        harness.drain(cx).await;
        let weak_thread = harness.thread.downgrade();
        let V2SessionHarness {
            connection,
            thread,
            agent,
            _client_task,
            _agent_task,
            _legacy_agent_task,
            ..
        } = harness;
        let _transports = (agent, _client_task, _agent_task, _legacy_agent_task);
        cx.update(|_| drop(thread));
        assert!(weak_thread.upgrade().is_none());
        assert_eq!(
            v2_response(session_response, cx, "elicitation owner-drop response")
                .await
                .expect("owner-drop response")
                .action,
            acp_v2::ElicitationAction::Cancel
        );
        assert!(
            (&mut request_response).now_or_never().is_none(),
            "session destruction does not cancel request-scoped feedback"
        );
        settle_after_disconnect(connection.dispatch_tx.clone())
            .await
            .expect("canonical disconnect cleanup");
        assert_eq!(
            v2_response(request_response, cx, "elicitation disconnect response")
                .await
                .expect("disconnect response")
                .action,
            acp_v2::ElicitationAction::Cancel
        );
    }

    #[gpui::test]
    async fn v2_session_info_sdk_updates_thread_and_list_projection(cx: &mut gpui::TestAppContext) {
        use acp_thread::{AcpThreadEvent, AgentSessionInfo, SessionListUpdate};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let updates = harness.session_list_updates().await;
        let session_id = harness
            .thread
            .read_with(cx, |thread, _| thread.session_id().clone());
        let mut listed = AgentSessionInfo::new(session_id.clone());
        listed.title = Some("Saved list title".into());
        harness.thread.update(cx, |thread, cx| {
            thread.set_provisional_title("Preview".into(), cx);
        });
        let titles = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&harness.thread, {
                let titles = titles.clone();
                move |thread, event, cx| {
                    if matches!(event, AcpThreadEvent::TitleUpdated) {
                        titles.borrow_mut().push(thread.read(cx).title());
                    }
                }
            })
        });
        for (patch, expected_title, expected_list_title, expected_meta, expected_timestamp) in [
            (
                json!({"updatedAt": "2026-10-01T10:20:30Z", "_meta": {"scope": "session"}}),
                Some("Preview"),
                Some("Saved list title"),
                Some(v2_terminal_meta("session")),
                Some("2026-10-01T10:20:30+00:00"),
            ),
            (
                json!({"title": "Agent title"}),
                Some("Agent title"),
                Some("Agent title"),
                Some(v2_terminal_meta("session")),
                Some("2026-10-01T10:20:30+00:00"),
            ),
            (
                json!({"updatedAt": null, "_meta": {}}),
                Some("Agent title"),
                Some("Agent title"),
                Some(v2_terminal_meta("session")),
                None,
            ),
            (
                json!({"title": null, "_meta": null}),
                None,
                None,
                None,
                None,
            ),
        ] {
            let expected_update: acp_v2::SessionInfoUpdate =
                serde_json::from_value(patch.clone()).expect("session-info patch");
            let mut update = patch;
            update
                .as_object_mut()
                .expect("update object")
                .insert("sessionUpdate".into(), json!("session_info_update"));
            harness.send(update, Some(v2_terminal_meta("delivery")));
            harness.drain(cx).await;
            let SessionListUpdate::SessionInfo {
                session_id: updated_id,
                update,
            } = updates.try_recv().expect("list projection update")
            else {
                panic!("session-info list update");
            };
            assert_eq!(updated_id, session_id);
            assert_eq!(update, expected_update.into());
            listed.apply_update(update);
            assert_eq!(listed.title.as_deref(), expected_list_title);
            assert_eq!(listed.meta, expected_meta);
            assert_eq!(
                listed.updated_at.map(|time| time.to_rfc3339()).as_deref(),
                expected_timestamp
            );
            harness.thread.read_with(cx, |thread, _| {
                assert_eq!(thread.title().as_deref(), expected_title);
                assert_eq!(thread.session_info().meta, listed.meta);
                assert_eq!(thread.session_info().updated_at, listed.updated_at);
                assert!(thread.entries().is_empty());
            });
        }
        cx.run_until_parked();
        assert_eq!(
            titles
                .borrow()
                .iter()
                .map(|title| title.as_deref())
                .collect::<Vec<_>>(),
            [Some("Agent title"), None]
        );
        assert!(updates.try_recv().is_err());
    }

    #[gpui::test]
    async fn v2_commands_sdk_preserves_source_and_replaces_the_list(cx: &mut gpui::TestAppContext) {
        use acp_thread::AcpThreadEvent;
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let events = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&harness.thread, {
                let events = events.clone();
                move |_, event, _| {
                    if let AcpThreadEvent::AvailableCommandsUpdated(commands) = event {
                        events.borrow_mut().push(commands.clone());
                    }
                }
            })
        });
        let first = json!([
            {"name": "opaque/手動", "description": "First command",
                "input": {"type": "text", "hint": "Describe the task", "_meta": {"scope": "input"}},
                "_meta": {"scope": "command", "retain": [null, 7]}},
            {"name": "future", "description": "Unknown input",
                "input": {"type": "_future", "payload": {"retain": [false, null]}}}
        ]);
        let mut expected_events = Vec::new();
        for commands in [
            first,
            json!([{"name": "only", "description": "Replacement command"}]),
            json!([]),
        ] {
            let expected: Vec<acp_v2::AvailableCommand> =
                serde_json::from_value(commands.clone()).expect("command source");
            harness.send(
                json!({"sessionUpdate": "available_commands_update", "availableCommands": commands,
                    "_meta": {"scope": "update"}}),
                Some(v2_terminal_meta("delivery")),
            );
            harness.drain(cx).await;
            harness.thread.read_with(cx, |thread, _| {
                assert_eq!(thread.available_commands(), expected);
                assert!(thread.entries().is_empty());
            });
            expected_events.push(expected);
        }
        cx.run_until_parked();
        assert_eq!(*events.borrow(), expected_events);
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_config_sdk_replaces_owner_and_notifies_subscribers(cx: &mut gpui::TestAppContext) {
        use acp_thread::AcpThreadEvent;
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let session_id = harness
            .thread
            .read_with(cx, |thread, _| thread.session_id().clone());
        assert!(
            cx.read(|cx| harness.connection.session_config_options(&session_id, cx))
                .is_none()
        );
        let events = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&harness.thread, {
                let events = events.clone();
                let connection = harness.connection.clone();
                let session_id = session_id.clone();
                move |_, event, cx| {
                    if let AcpThreadEvent::ConfigOptionsUpdated(options) = event {
                        let owner = connection
                            .session_config_options(&session_id, cx)
                            .expect("config owner installed before the event");
                        assert_eq!(owner.config_options(), *options);
                        events.borrow_mut().push(options.clone());
                    }
                }
            })
        });
        let first = json!([
            {"configId": "opaque/手動", "name": "Mode", "description": "Select a mode",
                "category": "mode", "type": "select", "currentValue": "custom",
                "options": [{"value": "custom", "name": "Custom", "description": "Offered",
                    "_meta": {"scope": "value", "retain": [null, 7]}}],
                "_meta": {"scope": "option"}},
            {"configId": "future", "name": "Future", "type": "_future",
                "payload": {"retain": [false, null]}, "_meta": {"scope": "future"}}
        ]);
        let mut expected_events = Vec::new();
        harness.send(
            json!({"sessionUpdate": "config_option_update", "configOptions": first,
                "_meta": {"scope": "update"}}),
            Some(v2_terminal_meta("delivery")),
        );
        harness.drain(cx).await;
        expected_events.push(
            serde_json::from_value::<Vec<acp_v2::SessionConfigOption>>(first.clone())
                .expect("initial options"),
        );
        let owner = cx
            .read(|cx| harness.connection.session_config_options(&session_id, cx))
            .expect("config owner");
        assert_eq!(owner.config_options(), expected_events[0]);
        let mut updates = cx.update(|cx| owner.watch(cx).expect("config subscription"));
        drop(updates.borrow());
        for options in [
            json!([{"configId": "enabled", "name": "Enabled", "type": "boolean",
                "currentValue": true, "_meta": {"scope": "replacement"}}]),
            json!([]),
        ] {
            let expected: Vec<acp_v2::SessionConfigOption> =
                serde_json::from_value(options.clone()).expect("replacement options");
            harness.send(
                json!({"sessionUpdate": "config_option_update", "configOptions": options}),
                None,
            );
            harness.drain(cx).await;
            updates
                .changed()
                .now_or_never()
                .expect("config replacement notification")
                .expect("live config subscription");
            assert_eq!(owner.config_options(), expected);
            expected_events.push(expected);
        }
        harness.send_to(
            "unknown-session",
            json!({"sessionUpdate": "config_option_update", "configOptions": first}),
            None,
        );
        harness.drain(cx).await;
        assert!(updates.changed().now_or_never().is_none());
        assert!(owner.config_options().is_empty());
        assert!(
            !harness
                .connection
                .sessions
                .borrow()
                .contains_key(&"unknown-session".into())
        );
        harness.thread.read_with(cx, |thread, _| {
            assert!(thread.entries().is_empty());
        });
        cx.run_until_parked();
        assert_eq!(*events.borrow(), expected_events);
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_usage_sdk_preserves_counters_and_optional_cost(cx: &mut gpui::TestAppContext) {
        use acp_thread::{AcpThreadEvent, TokenUsage};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let events = Rc::new(RefCell::new(0));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&harness.thread, {
                let events = events.clone();
                move |_, event, _| {
                    if matches!(event, AcpThreadEvent::TokenUsageUpdated) {
                        *events.borrow_mut() += 1;
                    }
                    assert!(!matches!(event, AcpThreadEvent::Error));
                }
            })
        });
        harness.send(
            json!({"sessionUpdate": "usage_update", "used": 0, "size": 10000}),
            None,
        );
        harness.drain(cx).await;
        harness.thread.update(cx, |thread, cx| {
            assert_eq!(
                thread.token_usage().expect("initial usage").max_tokens,
                10000
            );
            assert!(thread.cost().is_none());
            thread.update_token_usage(
                Some(TokenUsage {
                    max_tokens: 10000,
                    used_tokens: 0,
                    input_tokens: 123,
                    output_tokens: 456,
                    max_output_tokens: Some(789),
                }),
                cx,
            );
        });
        for (update, expected_amount, expected_currency) in [
            (
                json!({"used": 1000, "size": 10000,
                "cost": {"amount": 0.42, "currency": "USD", "_meta": {"scope": "cost"}}}),
                0.42,
                "USD",
            ),
            (json!({"used": 2000, "size": 20000}), 0.42, "USD"),
            (json!({"used": 0, "size": 0, "cost": null}), 0.42, "USD"),
            (
                json!({"used": 3000, "size": 30000, "cost": {"amount": 0, "currency": "EUR"}}),
                0.0,
                "EUR",
            ),
        ] {
            let expected: acp_v2::UsageUpdate =
                serde_json::from_value(update.clone()).expect("usage source");
            let mut update = update;
            let object = update.as_object_mut().expect("usage object");
            object.insert("sessionUpdate".into(), json!("usage_update"));
            object.insert("_meta".into(), json!({"scope": "update"}));
            harness.send(update, Some(v2_terminal_meta("delivery")));
            harness.drain(cx).await;
            harness.thread.read_with(cx, |thread, _| {
                assert_eq!(
                    thread.token_usage(),
                    Some(&TokenUsage {
                        max_tokens: expected.size,
                        used_tokens: expected.used,
                        input_tokens: 123,
                        output_tokens: 456,
                        max_output_tokens: Some(789),
                    })
                );
                let cost = thread.cost().expect("retained cost");
                assert_eq!(cost.amount, expected_amount);
                assert_eq!(cost.currency.as_ref(), expected_currency);
                assert_eq!(
                    cost.meta,
                    (expected_currency == "USD").then(|| v2_terminal_meta("cost"))
                );
                assert!(thread.entries().is_empty());
                assert!(!thread.had_error());
            });
        }
        harness.send_to(
            "unknown-session",
            json!({"sessionUpdate": "usage_update", "used": 999, "size": 999,
                "cost": {"amount": 999, "currency": "USD"}}),
            None,
        );
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, _| {
            assert_eq!(thread.token_usage().expect("usage").used_tokens, 3000);
            assert_eq!(thread.cost().expect("cost").amount, 0.0);
        });
        cx.run_until_parked();
        assert_eq!(*events.borrow(), 6);
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_tool_sdk_patches_chunks_and_display_references(cx: &mut gpui::TestAppContext) {
        use acp_thread::{ToolCallContent, ToolCallStatus};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let unknown = json!({"type": "_future", "payload": {"retain": [null, 7]},
            "_meta": {"scope": "unknown"}});
        for update in [
            json!({"sessionUpdate": "tool_call_content_chunk", "toolCallId": "first",
                "content": {"type": "content", "content": {"type": "text", "text": "original"},
                    "_meta": {"scope": "item"}}, "_meta": {"scope": "chunk"}}),
            json!({"sessionUpdate": "tool_call_update", "toolCallId": "first-extra",
                "title": "Other tool", "content": [{"type": "content",
                    "content": {"type": "text", "text": "other"}}]}),
            json!({"sessionUpdate": "tool_call_update", "toolCallId": "first",
                "title": "First tool", "kind": "_future_kind", "status": "in_progress",
                "rawInput": {"path": "./a/../b"}, "rawOutput": {"retain": [null, true]},
                "locations": [{"path": "/a/../file.rs", "line": 3, "_meta": {"scope": "location"}}],
                "_meta": {"scope": "tool", "old": true}}),
            json!({"sessionUpdate": "tool_call_content_chunk", "toolCallId": "first",
                "content": {"type": "content", "content": {"type": "text", "text": "historical"}},
                "_meta": {"scope": "chunk"}}),
            json!({"sessionUpdate": "tool_call_content_chunk", "toolCallId": "first",
                "content": unknown, "_meta": {"scope": "chunk"}}),
        ] {
            harness.send(update, Some(v2_terminal_meta("delivery")));
        }
        harness.drain(cx).await;
        let first_id = acp_v2::ToolCallId::new("first");
        harness.thread.read_with(cx, |thread, cx| {
            let (index, tool) = thread.tool_call(&first_id).expect("first tool");
            assert_eq!(index, 0);
            assert_eq!(tool.label.read(cx).source(), "First tool");
            assert_eq!(tool.status(), ToolCallStatus::InProgress);
            assert_eq!(
                serde_json::to_value(tool.kind()).expect("kind"),
                json!("_future_kind")
            );
            assert_eq!(tool.raw_input, Some(json!({"path": "./a/../b"})));
            assert_eq!(tool.raw_output, Some(json!({"retain": [null, true]})));
            assert_eq!(
                tool.meta.as_ref().and_then(|meta| meta.get("old")),
                Some(&json!(true))
            );
            let [location] = tool.locations.as_slice() else {
                panic!("one raw location");
            };
            assert_eq!(location.path, std::path::PathBuf::from("/a/../file.rs"));
            assert_eq!(location.line, Some(3));
            assert_eq!(location.meta, Some(v2_terminal_meta("location")));
            let [
                ToolCallContent::ContentBlock { meta, .. },
                ToolCallContent::ContentBlock { .. },
                ToolCallContent::Other { source, .. },
            ] = tool.content()
            else {
                panic!("ordered retained chunks");
            };
            assert_eq!(meta, &Some(v2_terminal_meta("item")));
            assert_eq!(
                serde_json::to_value(source).expect("unknown content"),
                unknown
            );
            assert_eq!(
                tool.content()
                    .iter()
                    .take(2)
                    .map(|item| item.to_markdown(cx))
                    .collect::<Vec<_>>(),
                ["original", "historical"]
            );
            let (index, other) = thread.tool_call(&"first-extra".into()).expect("other tool");
            assert_eq!(index, 1);
            assert_eq!(
                other.content().first().expect("content").to_markdown(cx),
                "other"
            );
        });

        for update in [
            json!({"sessionUpdate": "tool_call_update", "toolCallId": "first",
                "content": [{"type": "content", "content": {"type": "text", "text": "replacement"}},
                    {"type": "terminal", "terminalId": "patch-display"}],
                "_meta": {"scope": "replacement"}}),
            json!({"sessionUpdate": "tool_call_content_chunk", "toolCallId": "first",
                "content": {"type": "terminal", "terminalId": "chunk-display"}}),
        ] {
            harness.send(update, Some(v2_terminal_meta("later delivery")));
        }
        harness.drain(cx).await;
        let terminals = [
            harness.terminal("patch-display", cx),
            harness.terminal("chunk-display", cx),
        ];
        for terminal in &terminals {
            terminal.read_with(cx, |terminal, _| {
                assert!(terminal.display_state().is_some());
                assert!(!terminal.is_process_backed());
            });
        }
        harness.thread.read_with(cx, |thread, cx| {
            let (index, tool) = thread.tool_call(&first_id).expect("replaced tool");
            assert_eq!(index, 0);
            assert_eq!(
                tool.content().first().expect("content").to_markdown(cx),
                "replacement"
            );
            assert_eq!(tool.terminals().cloned().collect::<Vec<_>>(), terminals);
            assert_eq!(
                tool.meta,
                Some(
                    serde_json::from_value(json!({"scope": "replacement", "old": true}))
                        .expect("patched metadata")
                )
            );
            assert_eq!(tool.raw_input, Some(json!({"path": "./a/../b"})));
            assert_eq!(tool.locations.len(), 1);
        });
        harness.send(
            json!({"sessionUpdate": "tool_call_update", "toolCallId": "first",
                "content": null, "locations": null, "rawInput": null, "rawOutput": null,
                "_meta": null, "status": "completed"}),
            Some(v2_terminal_meta("clear delivery")),
        );
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, _| {
            let (_, tool) = thread.tool_call(&first_id).expect("cleared tool");
            assert!(tool.content().is_empty());
            assert!(tool.locations.is_empty());
            assert_eq!(tool.raw_input, None);
            assert_eq!(tool.raw_output, None);
            assert_eq!(tool.meta, None);
            assert_eq!(tool.status(), ToolCallStatus::Completed);
            assert_eq!(
                thread
                    .terminal("patch-display".into())
                    .expect("retained terminal"),
                terminals.first().expect("first terminal").clone()
            );
            assert_eq!(thread.entries().len(), 2);
        });
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_plan_sdk_keyed_snapshots_and_metadata_scopes(cx: &mut gpui::TestAppContext) {
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let entry = json!({"content": "Original", "priority": "_urgent", "status": "_blocked",
            "_meta": {"scope": "entry", "retain": [null, 7]}});
        for update in [
            json!({"sessionUpdate": "plan_update",
                "plan": {"type": "items", "planId": "plan", "entries": [entry], "_meta": {"scope": "plan"}},
                "_meta": {"scope": "update"}}),
            json!({"sessionUpdate": "plan_update",
                "plan": {"type": "items", "planId": "plan-extra", "entries": [
                    {"content": "Other plan", "priority": "low", "status": "pending"}]}}),
        ] {
            harness.send(update, Some(v2_terminal_meta("delivery")));
        }
        harness.drain(cx).await;
        let plan_id = acp_v2::PlanId::new("plan");
        let markdown = harness.thread.read_with(cx, |thread, _| {
            let plan = thread.plan_by_id(&plan_id).expect("first plan");
            assert_eq!(plan.meta, Some(v2_terminal_meta("plan")));
            assert_eq!(plan.update_meta, Some(v2_terminal_meta("update")));
            let [retained] = plan.entries.as_slice() else {
                panic!("one retained entry");
            };
            assert_eq!(
                serde_json::to_value(&retained.source).expect("entry source"),
                entry
            );
            retained.content.clone()
        });
        harness
            .thread
            .update(cx, |thread, cx| thread.clear_plan(cx));
        harness.send(
            json!({"sessionUpdate": "plan_update",
                "plan": {"type": "items", "planId": "plan", "entries": [entry], "_meta": {}},
                "_meta": {}}),
            Some(v2_terminal_meta("metadata delivery")),
        );
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, _| {
            assert!(
                thread.plan().is_none(),
                "metadata changes must not undo dismissal"
            );
            let plan = thread.plan_by_id(&plan_id).expect("retained hidden plan");
            assert_eq!(plan.meta, Some(acp_v2::Meta::new()));
            assert_eq!(plan.update_meta, Some(acp_v2::Meta::new()));
            assert_eq!(plan.entries.first().expect("entry").content, markdown);
        });
        harness.send(
            json!({"sessionUpdate": "plan_update",
                "plan": {"type": "items", "planId": "plan", "entries": [
                    {"content": "Revised", "priority": "low", "status": "completed"}]}}),
            Some(v2_terminal_meta("replacement delivery")),
        );
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, cx| {
            let plan = thread.plan().expect("changed tasks reopen the plan");
            assert_eq!(plan.meta, None);
            assert_eq!(plan.update_meta, None);
            let [entry] = plan.entries.as_slice() else {
                panic!("replacement entry");
            };
            assert_eq!(entry.source.status, acp_v2::PlanEntryStatus::Completed);
            assert_eq!(entry.source.meta, None);
            assert_eq!(entry.content, markdown);
            assert_eq!(markdown.read(cx).source(), "Revised");
        });
        harness.send(
            json!({"sessionUpdate": "plan_update",
                "plan": {"type": "items", "planId": "plan", "entries": []}}),
            None,
        );
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, _| {
            assert!(
                thread
                    .plan_by_id(&plan_id)
                    .expect("empty plan")
                    .entries
                    .is_empty()
            );
            assert_eq!(
                thread
                    .plan_by_id(&"plan-extra".into())
                    .expect("other plan")
                    .entries
                    .len(),
                1
            );
            assert!(thread.entries().is_empty(), "plans are not transcript rows");
        });
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_message_sdk_snapshots_chunks_and_historical_updates(cx: &mut gpui::TestAppContext) {
        use acp_thread::{AgentThreadEntry, AssistantMessageChunk, MessageIdentity};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let user_content = json!([
            {"type": "text", "text": "question", "_meta": {"scope": "item"}},
            {"type": "_future", "payload": {"retain": [null, 7]}, "_meta": {"scope": "unknown"}}
        ]);
        for update in [
            json!({"sessionUpdate": "user_message", "messageId": "shared",
                "content": user_content, "_meta": {"scope": "user"}}),
            json!({"sessionUpdate": "agent_message_chunk", "messageId": "shared",
                "content": {"type": "text", "text": "answer"}, "_meta": {"scope": "chunk"}}),
            json!({"sessionUpdate": "agent_thought", "messageId": "shared",
                "content": [{"type": "text", "text": "thought"}], "_meta": {"scope": "thought"}}),
            json!({"sessionUpdate": "user_message_chunk", "messageId": "shared",
                "content": {"type": "text", "text": " tail"}, "_meta": {"scope": "chunk"}}),
            json!({"sessionUpdate": "agent_message", "messageId": "shared",
                "content": [{"type": "text", "text": "replacement"}], "_meta": {"scope": "assistant"}}),
            json!({"sessionUpdate": "agent_thought_chunk", "messageId": "shared",
                "content": {"type": "text", "text": " tail"}, "_meta": {"scope": "chunk"}}),
        ] {
            harness.send(update, Some(v2_terminal_meta("delivery")));
        }
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, cx| {
            let [
                AgentThreadEntry::UserMessage(user),
                AgentThreadEntry::AssistantMessage(assistant),
            ] = thread.entries()
            else {
                panic!("user and assistant rows");
            };
            let mut expected: Vec<acp_v2::ContentBlock> =
                serde_json::from_value(user_content).expect("source content");
            expected.push(" tail".into());
            assert_eq!(user.content.source_blocks(), expected);
            assert_eq!(user.meta, Some(v2_terminal_meta("user")));
            let [
                AssistantMessageChunk::Message {
                    identity,
                    block,
                    meta,
                },
                AssistantMessageChunk::Thought {
                    identity: thought_id,
                    block: thought,
                    meta: thought_meta,
                },
            ] = assistant.chunks.as_slice()
            else {
                panic!("same-ID message and thought chunks");
            };
            assert_eq!(identity, &MessageIdentity::Keyed("shared".into()));
            assert_eq!(thought_id, identity);
            assert_eq!(block.to_markdown(cx), "replacement");
            assert_eq!(meta, &Some(v2_terminal_meta("assistant")));
            assert_eq!(thought.source_blocks(), &["thought".into(), " tail".into()]);
            assert_eq!(thought_meta, &Some(v2_terminal_meta("thought")));
        });

        for update in [
            json!({"sessionUpdate": "user_message", "messageId": "later",
                "content": [{"type": "text", "text": "later"}]}),
            json!({"sessionUpdate": "agent_message_chunk", "messageId": "shared",
                "content": {"type": "text", "text": " historical"}}),
            json!({"sessionUpdate": "agent_thought", "messageId": "shared",
                "content": null, "_meta": null}),
            json!({"sessionUpdate": "user_message", "messageId": "shared", "_meta": {}}),
        ] {
            harness.send(update, Some(v2_terminal_meta("later delivery")));
        }
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, cx| {
            let [
                AgentThreadEntry::UserMessage(user),
                AgentThreadEntry::AssistantMessage(assistant),
                AgentThreadEntry::UserMessage(later),
            ] = thread.entries()
            else {
                panic!("historical updates retain first-seen rows");
            };
            assert_eq!(user.content.source_blocks().len(), 3);
            assert_eq!(user.meta, Some(v2_terminal_meta("user")));
            assert_eq!(later.content.to_markdown(cx), "later");
            let [
                AssistantMessageChunk::Message { block, meta, .. },
                AssistantMessageChunk::Thought {
                    block: thought,
                    meta: thought_meta,
                    ..
                },
            ] = assistant.chunks.as_slice()
            else {
                panic!("historical chunks retain their positions");
            };
            assert_eq!(
                block.source_blocks(),
                &["replacement".into(), " historical".into()]
            );
            assert_eq!(meta, &Some(v2_terminal_meta("assistant")));
            assert!(thought.source_blocks().is_empty());
            assert_eq!(thought_meta, &None);
        });
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_session_sdk_routes_updates_and_ignores_released_owners(
        cx: &mut gpui::TestAppContext,
    ) {
        use acp_thread::AgentThreadEntry;
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let project = harness
            .thread
            .read_with(cx, |thread, _| thread.project().clone());
        let child_id = acp_v2::SessionId::new("child-session");
        let child = cx
            .update(|cx| {
                harness.connection.clone().load_session(
                    child_id.clone(),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    cx,
                )
            })
            .await
            .expect("register another session");
        for (session_id, text, encoded_output) in [
            ("v2-terminal-session", "main", "bWFpbg=="),
            ("child-session", "child", "Y2hpbGQ="),
            ("unknown-session", "ignored", "aWdub3JlZA=="),
        ] {
            harness.send_to(
                session_id,
                json!({"sessionUpdate": "user_message", "messageId": "shared",
                    "content": [{"type": "text", "text": text}]}),
                None,
            );
            harness.send_to(
                session_id,
                json!({"sessionUpdate": "terminal_update", "terminalId": "shared-display",
                    "output": {"data": encoded_output}, "exitStatus": {"exitCode": 0}}),
                None,
            );
            harness.send_to(
                session_id,
                json!({"sessionUpdate": "config_option_update", "configOptions": [
                    {"configId": "shared", "name": text, "type": "boolean", "currentValue": true}
                ]}),
                None,
            );
            harness.send_to(
                session_id,
                json!({"sessionUpdate": "usage_update", "used": text.len(), "size": 10000}),
                None,
            );
        }
        harness.drain(cx).await;
        for (thread, expected) in [(&harness.thread, "main"), (&child, "child")] {
            thread.read_with(cx, |thread, cx| {
                let [AgentThreadEntry::UserMessage(user)] = thread.entries() else {
                    panic!("one message in the routed session");
                };
                assert_eq!(user.content.to_markdown(cx), expected);
                assert_eq!(
                    thread.token_usage().expect("routed usage").used_tokens,
                    expected.len() as u64
                );
                let config = harness
                    .connection
                    .session_config_options(thread.session_id(), cx)
                    .expect("routed config");
                assert_eq!(config.config_options().len(), 1);
                assert_eq!(
                    config.config_options().first().expect("config option").name,
                    expected
                );
                let terminal = thread
                    .terminal("shared-display".into())
                    .expect("routed terminal");
                assert_eq!(
                    terminal
                        .read(cx)
                        .output()
                        .expect("terminal snapshot")
                        .content
                        .trim_end(),
                    expected
                );
            });
        }
        let child_terminal = child
            .read_with(cx, |thread, _| thread.terminal("shared-display".into()))
            .expect("child terminal");
        let child_config = cx
            .read(|cx| harness.connection.session_config_options(&child_id, cx))
            .expect("child config owner");
        let child_options = child_config.config_options();
        assert_eq!(harness.connection.sessions.borrow().len(), 2);
        let released = cx.update(|cx| {
            harness
                .connection
                .release_session(child_id.clone(), child.downgrade(), cx);
            harness
                .connection
                .pending_sessions
                .borrow()
                .get(&child_id)
                .expect("closing session")
                .task
                .clone()
        });
        released
            .await
            .expect("close and drain the released session");
        harness.send_to(
            "child-session",
            json!({"sessionUpdate": "user_message", "messageId": "shared",
                "content": [{"type": "text", "text": "late replacement"}]}),
            None,
        );
        for update in [
            json!({"sessionUpdate": "terminal_update", "terminalId": "shared-display",
                "command": "late command", "output": {"data": "bGF0ZQ=="},
                "exitStatus": {"exitCode": 7}}),
            json!({"sessionUpdate": "terminal_output_chunk", "terminalId": "shared-display",
                "data": "bGF0ZQ=="}),
            json!({"sessionUpdate": "config_option_update", "configOptions": []}),
            json!({"sessionUpdate": "usage_update", "used": 999, "size": 999}),
        ] {
            harness.send_to("child-session", update, None);
        }
        harness.drain(cx).await;
        assert!(!harness.connection.sessions.borrow().contains_key(&child_id));
        child.read_with(cx, |thread, cx| {
            let [AgentThreadEntry::UserMessage(user)] = thread.entries() else {
                panic!("released owner's retained message");
            };
            assert_eq!(user.content.to_markdown(cx), "child");
            assert_eq!(thread.token_usage().expect("retained usage").used_tokens, 5);
        });
        assert_eq!(child_config.config_options(), child_options);
        child_terminal.read_with(cx, |terminal, _| {
            let output = terminal.output().expect("retained child output");
            assert_eq!(output.content.trim_end(), "child");
            assert_eq!(output.exit_status.exit_code, Some(0));
        });

        let V2SessionHarness {
            connection,
            thread,
            _client_task,
            _agent_task,
            _legacy_agent_task,
            ..
        } = harness;
        let _transport_tasks = (_client_task, _agent_task, _legacy_agent_task);
        let weak_thread = thread.downgrade();
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let config = cx
            .read(|cx| connection.session_config_options(&session_id, cx))
            .expect("config owner before drop");
        let options = config.config_options();
        let notification =
            InboundSessionNotification::try_from(acp_v2::UpdateSessionNotification::new(
                session_id.clone(),
                acp_v2::SessionUpdate::AgentMessage(
                    acp_v2::AgentMessage::new("queued-before-drop").content(vec!["queued".into()]),
                ),
            ))
            .expect("message notification");
        let terminal_notification =
            terminal_updates::decode_terminal_notification(acp_v2::UpdateSessionNotification::new(
                session_id.clone(),
                acp_v2::SessionUpdate::TerminalOutputChunk(acp_v2::TerminalOutputChunk::new(
                    "shared-display",
                    "bGF0ZQ==",
                )),
            ))
            .expect("decoded terminal bytes")
            .expect("terminal notification");
        cx.update(|_| {
            enqueue_notification(
                &connection.dispatch_tx,
                notification,
                handle_v2_session_notification,
            );
            enqueue_notification(
                &connection.dispatch_tx,
                terminal_notification.into(),
                handle_v2_session_notification,
            );
            for update in [
                acp_v2::SessionUpdate::ConfigOptionUpdate(acp_v2::ConfigOptionUpdate::new(
                    Vec::new(),
                )),
                acp_v2::SessionUpdate::UsageUpdate(acp_v2::UsageUpdate::new(999, 999)),
            ] {
                enqueue_notification(
                    &connection.dispatch_tx,
                    InboundSessionNotification::try_from(acp_v2::UpdateSessionNotification::new(
                        session_id.clone(),
                        update,
                    ))
                    .expect("queued config/usage notification"),
                    handle_v2_session_notification,
                );
            }
            drop(thread);
        });
        drain_foreground_queue(&connection.dispatch_tx)
            .await
            .expect("apply queued work after owner drop");
        assert!(weak_thread.upgrade().is_none());
        assert_eq!(config.config_options(), options);
    }

    #[gpui::test]
    async fn v2_notice_sdk_preserves_feedback_without_failing_the_turn(
        cx: &mut gpui::TestAppContext,
    ) {
        use acp_thread::{AcpThreadEvent, ForegroundActivity};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let notice_events = Rc::new(RefCell::new(0));
        let turn_errors = Rc::new(RefCell::new(0));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&harness.thread, {
                let notice_events = notice_events.clone();
                let turn_errors = turn_errors.clone();
                move |_, event, _| match event {
                    AcpThreadEvent::NoticesUpdated => *notice_events.borrow_mut() += 1,
                    AcpThreadEvent::Error
                    | AcpThreadEvent::Refusal
                    | AcpThreadEvent::Stopped { .. } => {
                        *turn_errors.borrow_mut() += 1;
                    }
                    _ => {}
                }
            })
        });
        let sources = [
            json!({"severity": "error", "title": "Optional integration failed",
                "description": "Continue with the available tools",
                "_meta": {"scope": "notice", "retain": [null, 7]}}),
            json!({"severity": "_future", "title": "Optional integration failed",
                "_meta": {"scope": "second notice"}}),
        ];
        for source in &sources {
            let mut update = source.clone();
            update
                .as_object_mut()
                .expect("notice object")
                .insert("sessionUpdate".into(), json!("notice"));
            harness.send(update, Some(v2_terminal_meta("delivery")));
        }
        harness.send_to(
            "unknown-session",
            json!({"sessionUpdate": "notice", "severity": "error", "title": "Ignored"}),
            None,
        );
        harness.drain(cx).await;
        let first_id = harness.thread.read_with(cx, |thread, _| {
            let [(first_id, first), (second_id, second)] = thread.notices() else {
                panic!("two independent notices");
            };
            assert_ne!(first_id, second_id);
            assert_eq!(
                &serde_json::to_value(first).expect("first notice"),
                sources.first().expect("source")
            );
            assert_eq!(
                &serde_json::to_value(second).expect("second notice"),
                sources.get(1).expect("source")
            );
            assert!(thread.entries().is_empty());
            assert!(!thread.had_error());
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            *first_id
        });
        harness
            .thread
            .update(cx, |thread, cx| thread.dismiss_notice(first_id, cx));
        cx.run_until_parked();
        harness.thread.read_with(cx, |thread, _| {
            let [(_, notice)] = thread.notices() else {
                panic!("only the selected notice is dismissed");
            };
            assert_eq!(
                &serde_json::to_value(notice).expect("retained notice"),
                sources.get(1).expect("source")
            );
        });
        assert_eq!(*notice_events.borrow(), 3);
        assert_eq!(*turn_errors.borrow(), 0);
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_compaction_sdk_chunks_snapshots_and_historical_targeting(
        cx: &mut gpui::TestAppContext,
    ) {
        use acp_thread::{AgentThreadEntry, ContextCompactionStatus};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let original = json!({"type": "text", "text": "original", "_meta": {"scope": "item"}});
        harness.send(
            json!({"sessionUpdate": "compaction_summary_chunk", "compactionId": "first",
                "content": original, "_meta": {"scope": "chunk"}}),
            Some(v2_terminal_meta("delivery")),
        );
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, _| {
            let [AgentThreadEntry::ContextCompaction(compaction)] = thread.entries() else {
                panic!("chunk-first compaction");
            };
            assert_eq!(compaction.status, ContextCompactionStatus::InProgress);
            assert_eq!(compaction.meta, None);
            assert!(thread.is_compacting());
        });
        for update in [
            json!({"sessionUpdate": "compaction_update", "compactionId": "first-extra",
                "status": "completed", "summary": [{"type": "text", "text": "other"}]}),
            json!({"sessionUpdate": "compaction_summary_chunk", "compactionId": "first",
                "content": {"type": "text", "text": " tail"}, "_meta": {"scope": "chunk"}}),
            json!({"sessionUpdate": "compaction_update", "compactionId": "first",
                "status": "failed", "error": "Retry compaction", "_meta": {"scope": "compaction"}}),
        ] {
            harness.send(update, Some(v2_terminal_meta("later delivery")));
        }
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, cx| {
            let [
                AgentThreadEntry::ContextCompaction(first),
                AgentThreadEntry::ContextCompaction(other),
            ] = thread.entries()
            else {
                panic!("first-seen compaction rows");
            };
            let mut expected: Vec<acp_v2::ContentBlock> =
                vec![serde_json::from_value(original).expect("item source")];
            expected.push(" tail".into());
            assert_eq!(first.summary.source_blocks(), expected);
            assert_eq!(first.status, ContextCompactionStatus::Failed);
            assert_eq!(
                first.error.as_ref().expect("error").read(cx).source(),
                "Retry compaction"
            );
            assert_eq!(first.meta, Some(v2_terminal_meta("compaction")));
            assert_eq!(other.summary.to_markdown(cx), "other");
            assert!(!thread.is_compacting());
        });
        let replacement = json!([
            {"type": "text", "text": "replacement", "_meta": {"scope": "replacement item"}},
            {"type": "_future", "payload": {"retain": [false, null]}}
        ]);
        for update in [
            json!({"sessionUpdate": "compaction_update", "compactionId": "first",
                "status": "_future", "summary": replacement}),
            json!({"sessionUpdate": "compaction_summary_chunk", "compactionId": "first",
                "content": {"type": "text", "text": " appended"}, "_meta": {"scope": "chunk"}}),
        ] {
            harness.send(update, Some(v2_terminal_meta("replacement delivery")));
        }
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, cx| {
            let [
                AgentThreadEntry::ContextCompaction(first),
                AgentThreadEntry::ContextCompaction(_),
            ] = thread.entries()
            else {
                panic!("replacement retains its historical row");
            };
            let mut expected: Vec<acp_v2::ContentBlock> =
                serde_json::from_value(replacement).expect("replacement source");
            expected.push(" appended".into());
            assert_eq!(first.summary.source_blocks(), expected);
            assert_eq!(
                first.status,
                ContextCompactionStatus::Other("_future".into())
            );
            assert_eq!(
                first
                    .error
                    .as_ref()
                    .expect("omitted error survives")
                    .read(cx)
                    .source(),
                "Retry compaction"
            );
            assert_eq!(first.meta, Some(v2_terminal_meta("compaction")));
        });
        harness.send(
            json!({"sessionUpdate": "compaction_update", "compactionId": "first",
                "status": "completed", "summary": null, "error": null, "_meta": null}),
            Some(v2_terminal_meta("clear delivery")),
        );
        harness.drain(cx).await;
        harness.thread.read_with(cx, |thread, cx| {
            let [
                AgentThreadEntry::ContextCompaction(first),
                AgentThreadEntry::ContextCompaction(other),
            ] = thread.entries()
            else {
                panic!("clears do not remove compaction identities");
            };
            assert_eq!(first.id.0.as_ref(), "first");
            assert_eq!(first.status, ContextCompactionStatus::Completed);
            assert!(first.summary.source_blocks().is_empty());
            assert_eq!(first.error, None);
            assert_eq!(first.meta, None);
            assert_eq!(other.id.0.as_ref(), "first-extra");
            assert_eq!(other.summary.to_markdown(cx), "other");
        });
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_terminal_sdk_snapshots_chunks_and_metadata(cx: &mut gpui::TestAppContext) {
        let harness = V2SessionHarness::new(cx).await;
        harness
            .thread
            .update(cx, |thread, cx| {
                thread.upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("terminal-tool").content(vec![
                        acp_v2::ToolCallContent::Terminal(acp_v2::Terminal::new("terminal-1")),
                    ]),
                    cx,
                )
            })
            .expect("tool reference creates terminal before wire update");
        let terminal = harness.terminal("terminal-1", cx);
        let renderer = terminal.read_with(cx, |terminal, _| terminal.inner().clone());
        harness.send(
            serde_json::json!({
                "sessionUpdate": "terminal_update", "terminalId": "terminal-1",
                "command": "wire command", "cwd": "/tmp",
                "output": {"data": "b2xk", "_meta": {"scope": "snapshot"}},
                "exitStatus": {"exitCode": 7, "signal": "SIGTERM", "_meta": {"scope": "exit"}},
                "_meta": {"scope": "terminal"}
            }),
            Some(v2_terminal_meta("snapshot-envelope")),
        );
        harness.send(
            serde_json::json!({
                "sessionUpdate": "terminal_output_chunk", "terminalId": "terminal-1",
                "data": "IQ==", "_meta": {"scope": "chunk"}
            }),
            Some(v2_terminal_meta("chunk-envelope")),
        );
        harness.drain(cx).await;
        let ended_at = terminal.read_with(cx, |terminal, cx| {
            assert_eq!(terminal.inner(), &renderer);
            assert_eq!(
                terminal.command().read(cx).source(),
                "```\nwire command\n```"
            );
            assert_eq!(
                terminal.working_dir().as_deref(),
                Some(std::path::Path::new("/tmp"))
            );
            let display = terminal.display_state().expect("display state");
            assert_eq!(display.meta, Some(v2_terminal_meta("terminal")));
            assert_eq!(display.output_meta, Some(v2_terminal_meta("snapshot")));
            let output = terminal.output().expect("exited snapshot");
            assert_eq!(output.exit_status.exit_code, Some(7));
            assert_eq!(output.exit_status.signal.as_deref(), Some("SIGTERM"));
            assert_eq!(output.exit_status.meta, Some(v2_terminal_meta("exit")));
            assert_eq!(output.content.trim_end(), "old!");
            assert_eq!(renderer.read(cx).get_content().trim_end(), "old!");
            output.ended_at
        });
        assert_eq!(
            *harness.received.lock().expect("receive mutex"),
            vec![
                V2TerminalReceive {
                    envelope_meta: Some(v2_terminal_meta("snapshot-envelope")),
                    chunk_meta: None,
                    bytes: None,
                },
                V2TerminalReceive {
                    envelope_meta: Some(v2_terminal_meta("chunk-envelope")),
                    chunk_meta: Some(v2_terminal_meta("chunk")),
                    bytes: Some(b"!".to_vec()),
                },
            ]
        );

        harness.send(
            serde_json::json!({
                "sessionUpdate": "terminal_update", "terminalId": "terminal-1",
                "output": {"data": "bmV3DQo=", "_meta": {"scope": "replacement"}}
            }),
            None,
        );
        for data in ["G1s=", "MzFtwg==", "og==", "Clo="] {
            harness.send(
                serde_json::json!({
                    "sessionUpdate": "terminal_output_chunk", "terminalId": "terminal-1",
                    "data": data, "_meta": {}
                }),
                Some(acp_v2::Meta::new()),
            );
        }
        harness.drain(cx).await;
        assert_eq!(harness.terminal("terminal-1", cx), terminal);
        terminal.read_with(cx, |terminal, cx| {
            assert_eq!(terminal.inner(), &renderer);
            assert_eq!(renderer.read(cx).get_content().trim_end(), "new\n¢\n Z");
            let display = terminal.display_state().expect("display state");
            assert_eq!(display.command(), Some("wire command"));
            assert_eq!(display.meta, Some(v2_terminal_meta("terminal")));
            assert_eq!(display.output_meta, Some(v2_terminal_meta("replacement")));
            let output = terminal
                .output()
                .expect("omitted exit preserves completion");
            assert_eq!(output.ended_at, ended_at);
            assert_eq!(output.exit_status.meta, Some(v2_terminal_meta("exit")));
            assert_eq!(output.content.trim_end(), "new\n¢\n Z");
        });
        {
            let received = harness.received.lock().expect("receive mutex");
            assert_eq!(
                received
                    .iter()
                    .skip(3)
                    .map(|event| event.bytes.clone())
                    .collect::<Vec<_>>(),
                vec![
                    Some(b"\x1b[".to_vec()),
                    Some(b"31m\xc2".to_vec()),
                    Some(vec![0xa2]),
                    Some(b"\nZ".to_vec()),
                ]
            );
            assert!(received.iter().skip(3).all(|event| {
                event.envelope_meta == Some(acp_v2::Meta::new())
                    && event.chunk_meta == Some(acp_v2::Meta::new())
            }));
        }

        harness.send(
            serde_json::json!({
                "sessionUpdate": "terminal_update", "terminalId": "terminal-1",
                "exitStatus": {}
            }),
            None,
        );
        harness.drain(cx).await;
        terminal.read_with(cx, |terminal, _| {
            let output = terminal
                .output()
                .expect("empty exit object still means exited");
            assert_eq!(output.exit_status, acp_v2::TerminalExitStatus::new());
            assert_eq!(output.ended_at, ended_at);
        });
        harness.send(
            serde_json::json!({
                "sessionUpdate": "terminal_update", "terminalId": "terminal-1",
                "command": null, "cwd": null, "output": {"data": "", "_meta": {}},
                "exitStatus": null, "_meta": null
            }),
            None,
        );
        harness.drain(cx).await;
        terminal.read_with(cx, |terminal, cx| {
            let display = terminal.display_state().expect("display state");
            assert_eq!(display.command(), None);
            assert_eq!(display.meta, None);
            assert_eq!(display.output_meta, Some(acp_v2::Meta::new()));
            assert_eq!(terminal.working_dir(), &None);
            assert!(terminal.output().is_none());
            assert!(renderer.read(cx).get_content().trim().is_empty());
            assert_eq!(terminal.inner(), &renderer);
        });
        assert_eq!(harness.terminal("terminal-1", cx), terminal);
        harness.thread.read_with(cx, |thread, _| {
            let (_, tool) = thread
                .tool_call(&acp_v2::ToolCallId::new("terminal-tool"))
                .expect("tool");
            assert_eq!(tool.terminals().next(), Some(&terminal));
        });
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_terminal_sdk_invalid_base64_is_atomic(cx: &mut gpui::TestAppContext) {
        let harness = V2SessionHarness::new(cx).await;
        harness.send(
            serde_json::json!({
                "sessionUpdate": "terminal_update", "terminalId": "terminal-1",
                "command": "original", "cwd": "/tmp",
                "output": {"data": "b2xk", "_meta": {"scope": "snapshot"}},
                "exitStatus": {"exitCode": 7, "_meta": {"scope": "exit"}},
                "_meta": {"scope": "terminal"}
            }),
            None,
        );
        harness.drain(cx).await;
        let terminal = harness.terminal("terminal-1", cx);
        let observe = |cx: &gpui::TestAppContext| {
            terminal.read_with(cx, |terminal, cx| {
                let display = terminal.display_state().expect("display state");
                let output = terminal.output().expect("completed terminal");
                (
                    display.command().map(str::to_owned),
                    terminal.working_dir().clone(),
                    display.meta.clone(),
                    display.output_meta.clone(),
                    output.exit_status.clone(),
                    output.ended_at,
                    output.content.clone(),
                    terminal.inner().read(cx).get_content(),
                )
            })
        };
        let before = observe(cx);
        let received_before = harness.received.lock().expect("receive mutex").len();
        for terminal_id in ["terminal-1", "unseen"] {
            harness.send(
                serde_json::json!({
                    "sessionUpdate": "terminal_update", "terminalId": terminal_id,
                    "command": "must not apply", "cwd": "/changed",
                    "output": {"data": "not base64!", "_meta": {"scope": "bad snapshot"}},
                    "exitStatus": {"exitCode": 99, "_meta": {"scope": "bad exit"}},
                    "_meta": {"scope": "bad terminal"}
                }),
                Some(v2_terminal_meta("bad envelope")),
            );
            harness
                .agent
                .send_notification(
                    agent_client_protocol::UntypedMessage::new(
                        "session/update",
                        serde_json::json!({
                            "sessionId": "v2-terminal-session",
                            "_meta": {"scope": "malformed envelope"},
                            "update": {
                                "sessionUpdate": "terminal_update", "terminalId": terminal_id,
                                "command": "must not apply", "cwd": "/changed",
                                "output": {"data": 42},
                                "exitStatus": {"exitCode": 99, "_meta": {"scope": "bad exit"}},
                                "_meta": {"scope": "bad terminal"}
                            }
                        }),
                    )
                    .expect("malformed snapshot SDK fixture"),
                )
                .expect("send malformed snapshot through SDK");
            // Together these strings form valid base64, but neither is a valid
            // independently encoded chunk.
            for data in ["b2", "s=", "not base64!"] {
                harness.send(
                    serde_json::json!({
                        "sessionUpdate": "terminal_output_chunk", "terminalId": terminal_id,
                        "data": data, "_meta": {"scope": "bad chunk"}
                    }),
                    Some(v2_terminal_meta("bad chunk envelope")),
                );
            }
        }
        harness.drain(cx).await;
        assert_eq!(observe(cx), before);
        assert_eq!(harness.terminal("terminal-1", cx), terminal);
        harness.thread.read_with(cx, |thread, _| {
            assert!(thread.terminal(acp_v2::TerminalId::new("unseen")).is_err());
        });
        assert_eq!(
            harness.received.lock().expect("receive mutex").len(),
            received_before
        );
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
        harness.send(
            serde_json::json!({
                "sessionUpdate": "terminal_output_chunk", "terminalId": "terminal-1",
                "data": "IQ=="
            }),
            None,
        );
        harness.drain(cx).await;
        terminal.read_with(cx, |terminal, cx| {
            assert_eq!(terminal.inner().read(cx).get_content().trim_end(), "old!");
            assert_eq!(
                terminal
                    .output()
                    .expect("still completed")
                    .content
                    .trim_end(),
                "old!"
            );
            assert_eq!(
                terminal.display_state().expect("display").meta,
                Some(v2_terminal_meta("terminal"))
            );
        });
        assert_eq!(
            harness.received.lock().expect("receive mutex").len(),
            received_before + 1
        );
    }

    #[gpui::test]
    async fn v2_state_sdk_reports_activity_without_prompt_submission(
        cx: &mut gpui::TestAppContext,
    ) {
        use acp_thread::{AcpThreadEvent, ForegroundActivity};
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let thread = harness.receipt_session("reported-session", cx).await;
        let stops = Rc::new(RefCell::new(Vec::new()));
        let phases = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            let stops = stops.clone();
            let phases = phases.clone();
            cx.subscribe(&thread, move |thread, event, cx| match event {
                AcpThreadEvent::Stopped { stop_reason, .. } => {
                    stops.borrow_mut().push(stop_reason.clone());
                }
                AcpThreadEvent::StatusChanged => {
                    phases
                        .borrow_mut()
                        .push(thread.read(cx).foreground_activity());
                }
                _ => {}
            })
        });
        for (state, phase) in [
            (
                json!({"state": "running", "_meta": {"scope": "state"}}),
                ForegroundActivity::Running,
            ),
            (
                json!({"state": "requires_action"}),
                ForegroundActivity::RequiresAction,
            ),
            (json!({"state": "running"}), ForegroundActivity::Running),
            (
                json!({"state": "idle", "stopReason": "end_turn",
                "usage": {"totalTokens": 26, "inputTokens": 17, "outputTokens": 9},
                "_meta": {"scope": "idle"}}),
                ForegroundActivity::Idle,
            ),
            (json!({"state": "idle"}), ForegroundActivity::Idle),
            (
                json!({"state": "_future_activity", "payload": {"retain": [null, 7]},
                "_meta": {"scope": "future"}}),
                ForegroundActivity::Running,
            ),
            (
                json!({"state": "idle", "stopReason": "error",
                "error": {"code": -32603, "message": "execution failed"}}),
                ForegroundActivity::Idle,
            ),
        ] {
            let expected: acp_v2::StateUpdate =
                serde_json::from_value(state.clone()).expect("reported state");
            let mut update = state;
            update["sessionUpdate"] = json!("state_update");
            harness.send_to(
                "reported-session",
                update,
                Some(v2_terminal_meta("delivery")),
            );
            harness.drain(cx).await;
            thread.read_with(cx, |thread, _| {
                assert!(thread.uses_reported_activity());
                assert_eq!(thread.foreground_activity(), phase);
                assert_eq!(thread.foreground_state(), &expected);
                assert!(thread.latest_submission_id().is_none());
                assert!(!thread.has_unsettled_submissions());
                assert!(thread.entries().is_empty());
            });
        }
        assert_eq!(
            *phases.borrow(),
            [
                ForegroundActivity::Running,
                ForegroundActivity::RequiresAction,
                ForegroundActivity::Running,
                ForegroundActivity::Idle,
                ForegroundActivity::Running,
                ForegroundActivity::Idle,
            ]
        );
        let stops = stops.borrow();
        assert_eq!(stops.len(), 2);
        assert_eq!(stops.first(), Some(&Some(acp_v2::StopReason::EndTurn)));
        assert!(matches!(
            stops.last(),
            Some(Some(acp_v2::StopReason::Error(_)))
        ));
        thread.read_with(cx, |thread, _| {
            assert!(thread.had_error());
            assert_eq!(thread.activity_generation(), 2);
            let usage = thread.token_usage().expect("reported idle usage");
            assert_eq!(usage.input_tokens, 17);
            assert_eq!(usage.output_tokens, 9);
        });
        assert!(harness.fallback.lock().expect("fallback mutex").is_empty());
    }

    #[gpui::test]
    async fn v2_state_sdk_routes_only_to_live_receipt_owners(cx: &mut gpui::TestAppContext) {
        use acp_thread::ForegroundActivity;
        use serde_json::json;

        let harness = V2SessionHarness::new(cx).await;
        let first = harness.receipt_session("first", cx).await;
        let second = harness.receipt_session("second", cx).await;
        for (session_id, state) in [
            ("first", "running"),
            ("second", "requires_action"),
            ("unknown", "running"),
            ("v2-terminal-session", "running"),
        ] {
            harness.send_to(
                session_id,
                json!({"sessionUpdate": "state_update", "state": state}),
                None,
            );
        }
        harness.drain(cx).await;
        assert_eq!(
            first.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
        assert_eq!(
            second.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::RequiresAction
        );
        harness.thread.read_with(cx, |thread, _| {
            assert!(!thread.uses_reported_activity());
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert_eq!(thread.activity_generation(), 0);
        });
        let wrong_session =
            InboundSessionNotification::try_from(acp_v2::UpdateSessionNotification::new(
                "first",
                acp_v2::SessionUpdate::StateUpdate(acp_v2::StateUpdate::Idle(
                    acp_v2::IdleStateUpdate::new(),
                )),
            ))
            .expect("state notification");
        second.update(cx, |thread, cx| {
            assert!(wrong_session.apply(thread, cx).is_err());
            assert_eq!(
                thread.foreground_activity(),
                ForegroundActivity::RequiresAction
            );
        });

        let released = cx.update(|cx| {
            let session_id = acp_v2::SessionId::new("first");
            harness
                .connection
                .release_session(session_id.clone(), first.downgrade(), cx);
            harness
                .connection
                .pending_sessions
                .borrow()
                .get(&session_id)
                .expect("closing receipt owner")
                .task
                .clone()
        });
        released.await.expect("release and drain the receipt owner");
        harness.send_to(
            "first",
            json!({"sessionUpdate": "state_update", "state": "idle"}),
            None,
        );
        harness.drain(cx).await;
        assert_eq!(
            first.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );

        let weak_second = second.downgrade();
        cx.update(|_| {
            enqueue_notification(
                &harness.connection.dispatch_tx,
                InboundSessionNotification::try_from(acp_v2::UpdateSessionNotification::new(
                    "second",
                    acp_v2::SessionUpdate::StateUpdate(acp_v2::StateUpdate::Idle(
                        acp_v2::IdleStateUpdate::new(),
                    )),
                ))
                .expect("queued state notification"),
                handle_v2_session_notification,
            );
            drop(second);
        });
        harness.drain(cx).await;
        assert!(weak_second.upgrade().is_none());
        assert_eq!(
            first.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
    }

    #[gpui::test]
    async fn v2_session_sdk_falls_through_unhandled_updates(cx: &mut gpui::TestAppContext) {
        let harness = V2SessionHarness::new(cx).await;
        harness.send(
            serde_json::json!({"sessionUpdate": "plan_update",
                "plan": {"type": "items", "planId": "unhandled-plan", "entries": [
                    {"content": "Retain", "priority": "low", "status": "pending"}]}}),
            None,
        );
        harness.drain(cx).await;
        let params = [
            serde_json::json!({
                "sessionId": "v2-terminal-session", "_meta": {"scope": "other envelope"},
                "futureOuter": "keep",
                "update": {
                    "sessionUpdate": "_future_terminal_neighbor",
                    "payload": {"bytes": ["not", "base64!"], "nested": {"unknown": null}},
                    "_meta": {"scope": "other update"}
                }
            }),
            serde_json::json!({
                "sessionId": "v2-terminal-session", "_meta": {"scope": "known envelope"},
                "futureOuter": {"preserve": [1, null, true]},
                "update": {"sessionUpdate": "plan_update",
                    "plan": {"type": "file", "planId": "unhandled-plan", "uri": "file:///tmp/plan.md"},
                    "_meta": {"scope": "known update"}, "futureKnownField": {"nested": "keep"}}
            }),
            serde_json::json!({
                "sessionId": "v2-terminal-session",
                "update": {"sessionUpdate": "plan_update",
                    "plan": {"type": "markdown", "planId": "unhandled-plan", "content": "Unsupported"}}
            }),
            serde_json::json!({
                "sessionId": "v2-terminal-session",
                "update": {"sessionUpdate": "plan_update",
                    "plan": {"type": "_future", "planId": "unhandled-plan", "payload": [null, 7]}}
            }),
        ];
        let sent = params
            .iter()
            .map(|params| {
                agent_client_protocol::UntypedMessage::new("session/update", params)
                    .expect("raw SDK notification")
            })
            .collect::<Vec<_>>();
        let mut expected = params
            .iter()
            .map(|params| {
                let notification: acp_v2::UpdateSessionNotification =
                    serde_json::from_value(params.clone()).expect("typed SDK notification");
                notification
                    .to_untyped_message()
                    .expect("SDK typed fallthrough")
            })
            .collect::<Vec<_>>();
        for message in &sent {
            harness
                .agent
                .send_notification(message.clone())
                .expect("send unrelated SDK update");
        }
        let unrelated_method = agent_client_protocol::UntypedMessage::new(
            "_other/notification",
            serde_json::json!({
                "update": {"sessionUpdate": "terminal_update", "output": {"data": "invalid!"}},
                "_meta": {"scope": "unrelated method"}
            }),
        )
        .expect("unrelated SDK notification");
        harness
            .agent
            .send_notification(unrelated_method.clone())
            .expect("send unrelated method");
        expected.push(unrelated_method);
        harness.drain(cx).await;
        let fallback = harness.fallback.lock().expect("fallback mutex");
        assert_eq!(*fallback, expected);
        assert!(harness.received.lock().expect("receive mutex").is_empty());
        harness.thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.foreground_activity(),
                acp_thread::ForegroundActivity::Idle
            );
            let plan = thread
                .plan_by_id(&"unhandled-plan".into())
                .expect("retained plan");
            assert_eq!(
                plan.entries.first().expect("entry").source.content,
                "Retain"
            );
            assert!(thread.entries().is_empty());
        });
    }

    fn init_settings_test(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    #[test]
    fn client_capabilities_include_elicitation() {
        let capabilities = client_capabilities_for_agent(&AgentId::new("codex-acp"));
        let elicitation = capabilities
            .elicitation
            .expect("elicitation should always be advertised");

        assert!(elicitation.form.is_some());
        assert!(elicitation.url.is_some());
    }

    #[gpui::test]
    async fn request_scoped_elicitation_during_auth_uses_connection_store(
        cx: &mut gpui::TestAppContext,
    ) {
        init_settings_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let request_id = acp::RequestId::Number(1);
        let request = acp::CreateElicitationRequest::new(
            acp::ElicitationFormMode::new(
                acp::ElicitationRequestScope::new(request_id.clone()),
                acp::ElicitationSchema::new()
                    .property(
                        "name",
                        acp::StringPropertySchema::new().meta(acp::Meta::from_iter([(
                            "property".into(),
                            serde_json::json!({"nested": [null, true]}),
                        )])),
                        true,
                    )
                    .meta(acp::Meta::from_iter([(
                        "schema".into(),
                        serde_json::json!({"nested": [1, "retained"]}),
                    )])),
            ),
            "Provide a name",
        )
        .meta(acp::Meta::from_iter([(
            "request".into(),
            serde_json::json!({"nested": [null, {"value": "retained"}]}),
        )]));
        let expected_request = serde_json::to_value(&request).expect("legacy request");
        let (harness, response_rx) =
            test_support::connect_fake_acp_connection_with_auth_elicitation(project, request, cx)
                .await;
        let connection = harness.connection.clone();
        let auth_task =
            cx.update(|cx| connection.authenticate(acp_v2::AuthMethodId::new("login"), cx));
        cx.run_until_parked();

        let store = connection
            .request_elicitations()
            .expect("ACP connections expose request-scoped elicitations");
        let elicitation_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one request-scoped elicitation, got {:?}",
                    store.elicitations()
                );
            };
            assert_eq!(
                serde_json::to_value(&elicitation.request).expect("shared request"),
                expected_request
            );
            let acp_v2::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, request_id);
            elicitation.id.clone()
        });
        assert!(
            connection.sessions.borrow().is_empty(),
            "auth-time request-scoped elicitations must not require a session"
        );

        let expected_content = std::collections::BTreeMap::from([(
            "name".to_string(),
            acp::ElicitationContentValue::from("Ada"),
        )]);
        let response_metadata = acp_v2::Meta::from_iter([(
            "response".into(),
            serde_json::json!({"nested": [null, {"value": "retained"}]}),
        )]);
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new().content(
                        std::collections::BTreeMap::from([(
                            "name".to_string(),
                            acp_v2::ElicitationContentValue::from("Ada"),
                        )]),
                    ),
                ))
                .meta(response_metadata.clone()),
                cx,
            );
        });

        let response = response_rx
            .recv()
            .await
            .expect("fake auth flow should receive elicitation response");
        assert_eq!(response.meta.as_ref(), Some(&response_metadata));
        assert_eq!(
            response.action,
            acp::ElicitationAction::Accept(
                acp::ElicitationAcceptAction::new().content(expected_content)
            )
        );
        auth_task.await.expect("auth should complete");
    }

    #[gpui::test]
    async fn request_scoped_url_elicitation_completion_before_consent_is_ignored(
        cx: &mut gpui::TestAppContext,
    ) {
        init_settings_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let request_id = acp::RequestId::Number(1);
        let url_elicitation_id = acp::ElicitationId::new("auth-url");
        let (harness, response_rx) =
            test_support::connect_fake_acp_connection_with_auth_elicitation_completion(
                project,
                acp::CreateElicitationRequest::new(
                    acp::ElicitationUrlMode::new(
                        acp::ElicitationRequestScope::new(request_id.clone()),
                        url_elicitation_id.clone(),
                        "https://auth.example.com/device",
                    ),
                    "Authorize Zed in your browser",
                ),
                acp::CompleteElicitationNotification::new(url_elicitation_id),
                cx,
            )
            .await;
        let connection = harness.connection.clone();
        let auth_task =
            cx.update(|cx| connection.authenticate(acp_v2::AuthMethodId::new("login"), cx));
        cx.run_until_parked();

        assert!(
            matches!(
                response_rx.try_recv(),
                Err(async_channel::TryRecvError::Empty)
            ),
            "completion before consent must not answer the elicitation request"
        );

        let store = connection
            .request_elicitations()
            .expect("ACP connections expose request-scoped elicitations");
        let entry_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one request-scoped elicitation, got {:?}",
                    store.elicitations()
                );
            };
            let acp_v2::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, request_id);
            assert!(matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Pending { .. }
            ));
            elicitation.id.clone()
        });

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &entry_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        let response = response_rx
            .recv()
            .await
            .expect("fake auth flow should receive elicitation response");
        assert_eq!(
            response.action,
            acp::ElicitationAction::Accept(acp::ElicitationAcceptAction::new())
        );
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&entry_id) else {
                panic!("missing request-scoped elicitation");
            };
            assert!(matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Accepted
            ));
        });

        auth_task.await.expect("auth should complete");
    }

    #[gpui::test]
    async fn request_scoped_elicitation_ignores_open_sessions(cx: &mut gpui::TestAppContext) {
        init_settings_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let request_id = acp::RequestId::Number(1);
        let (harness, response_rx) =
            test_support::connect_fake_acp_connection_with_auth_elicitation(
                project.clone(),
                acp::CreateElicitationRequest::new(
                    acp::ElicitationFormMode::new(
                        acp::ElicitationRequestScope::new(request_id.clone()),
                        acp::ElicitationSchema::new().string("name", true),
                    ),
                    "Provide a name",
                ),
                cx,
            )
            .await;
        let connection = harness.connection.clone();
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        let first_thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    acp_v2::SessionId::new("session-1"),
                    project.clone(),
                    work_dirs.clone(),
                    None,
                    cx,
                )
            })
            .await
            .expect("first load_session should succeed");
        let second_thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    acp_v2::SessionId::new("session-2"),
                    project,
                    work_dirs,
                    None,
                    cx,
                )
            })
            .await
            .expect("second load_session should succeed");
        cx.run_until_parked();
        assert_eq!(
            connection.sessions.borrow().len(),
            2,
            "test setup should have multiple open sessions"
        );

        let auth_task =
            cx.update(|cx| connection.authenticate(acp_v2::AuthMethodId::new("login"), cx));
        cx.run_until_parked();

        let store = connection
            .request_elicitations()
            .expect("ACP connections expose request-scoped elicitations");
        let elicitation_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one request-scoped elicitation, got {:?}",
                    store.elicitations()
                );
            };
            let acp_v2::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, request_id);
            elicitation.id.clone()
        });

        for thread in [first_thread, second_thread] {
            thread.read_with(cx, |thread, _| {
                assert!(
                    thread.entries().iter().all(|entry| !matches!(
                        entry,
                        acp_thread::AgentThreadEntry::Elicitation(_)
                    )),
                    "request-scoped elicitation should not be inserted into a session thread"
                );
            });
        }

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Decline),
                cx,
            );
        });

        let response = response_rx
            .recv()
            .await
            .expect("fake auth flow should receive elicitation response");
        assert_eq!(response.action, acp::ElicitationAction::Decline);
        auth_task.await.expect("auth should complete");
    }

    #[test]
    fn cursor_client_capabilities_include_parameterized_model_picker_meta() {
        let capabilities = client_capabilities_for_agent(&AgentId::new(CURSOR_ID));
        let meta = capabilities
            .meta
            .expect("expected client capabilities meta");

        assert_eq!(
            meta.get(PARAMETERIZED_MODEL_PICKER_META_KEY),
            Some(&serde_json::json!(true))
        );
        assert_eq!(meta.get("terminal_output"), Some(&serde_json::json!(true)));
        assert_eq!(meta.get("terminal-auth"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn non_cursor_client_capabilities_do_not_include_parameterized_model_picker_meta() {
        let capabilities = client_capabilities_for_agent(&AgentId::new("codex-acp"));
        let meta = capabilities
            .meta
            .expect("expected client capabilities meta");

        assert!(!meta.contains_key(PARAMETERIZED_MODEL_PICKER_META_KEY));
    }

    #[test]
    fn client_capabilities_include_boolean_config_options() {
        let capabilities = client_capabilities_for_agent(&AgentId::new("codex-acp"));

        assert!(
            capabilities
                .session
                .and_then(|session| session.config_options)
                .and_then(|config_options| config_options.boolean)
                .is_some()
        );
    }

    #[test]
    fn client_capabilities_include_compaction_and_notices() {
        let capabilities = client_capabilities_for_agent(&AgentId::new("codex-acp"));
        let capabilities =
            serde_json::to_value(capabilities).expect("client capabilities should serialize");
        let session = capabilities
            .get("session")
            .expect("session capabilities should be advertised");
        for capability in ["compaction", "notices"] {
            assert_eq!(
                session.get(capability),
                Some(&serde_json::json!({})),
                "{capability} should be advertised"
            );
        }
    }

    #[gpui::test]
    async fn connection_routes_terminal_auth(cx: &mut gpui::TestAppContext) {
        init_settings_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "project": {} }))
            .await;
        let project = project::Project::test(fs, [std::path::Path::new("/project")], cx).await;
        let mut harness = test_support::connect_fake_acp_connection(project.clone(), cx).await;
        cx.update(|cx| {
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::from_iter([(
                    "test".to_string(),
                    settings::CustomAgentServerSettings::Custom {
                        path: PathBuf::from("test-agent"),
                        args: vec!["--acp".into(), "--verbose".into()],
                        env: HashMap::from_iter([
                            ("BASE".into(), "1".into()),
                            ("SHARED".into(), "base".into()),
                        ]),
                        default_mode: None,
                        default_config_options: HashMap::default(),
                        favorite_config_option_values: HashMap::default(),
                    }
                    .into(),
                )])),
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            harness.connection.auth_methods(),
            &[acp_v2::AuthMethod::Agent(acp_v2::AuthMethodAgent::new(
                "login", "Login"
            ))]
        );
        let method_id = acp_v2::AuthMethodId::new("login");
        let mut methods = acp_thread::auth_methods::from_v1(vec![acp::AuthMethod::Terminal(
            acp::AuthMethodTerminal::new(method_id.0.clone(), "First-class login")
                .args(vec!["first-class-auth".into(), "".into()])
                .env(std::collections::HashMap::from_iter([
                    ("SHARED".into(), "override".into()),
                    ("EMPTY".into(), "".into()),
                ]))
                .meta(acp::Meta::from_iter([(
                    "terminal-auth".to_string(),
                    serde_json::json!({
                        "label": "Legacy login",
                        "command": "legacy-agent",
                        "args": ["legacy-auth"],
                    }),
                )])),
        )])
        .expect("terminal authentication should adapt");
        let Some(acp_v2::AuthMethod::Terminal(terminal)) = methods.first_mut() else {
            panic!("expected a shared terminal authentication method");
        };
        terminal.env.push(
            acp_v2::EnvVariable::new("EXTRA", "2").meta(acp_v2::Meta::from_iter([(
                "extension".into(),
                serde_json::json!({"nested": [null, true]}),
            )])),
        );
        let advertised_methods = methods.clone();
        Rc::get_mut(&mut harness.connection)
            .expect("test harness should have the only connection handle")
            .auth_methods = methods;

        let terminal_task = cx
            .update(|cx| harness.connection.terminal_auth_task(&method_id, cx))
            .expect("first-class terminal auth should be routed");
        let terminal_task = terminal_task
            .await
            .expect("first-class routing should resolve the test agent's external command");
        assert_eq!(terminal_task.command.as_deref(), Some("test-agent"));
        assert_eq!(
            terminal_task.args,
            vec!["--acp", "--verbose", "first-class-auth", ""]
        );
        assert_eq!(terminal_task.env.get("BASE").map(String::as_str), Some("1"));
        assert_eq!(
            terminal_task.env.get("SHARED").map(String::as_str),
            Some("override")
        );
        assert_eq!(terminal_task.env.get("EMPTY").map(String::as_str), Some(""));
        assert_eq!(
            terminal_task.env.get("EXTRA").map(String::as_str),
            Some("2")
        );
        assert_eq!(terminal_task.label, "First-class login");
        assert_eq!(harness.connection.auth_methods(), advertised_methods);
        cx.update(|cx| harness.connection.authenticate(method_id.clone(), cx))
            .await
            .expect_err("terminal auth must not use the agent-managed login RPC");

        let unknown = serde_json::json!({
            "type": "_future_login",
            "methodId": "unknown",
            "name": "Unknown login",
            "custom": {"nested": [null, true]},
            "_meta": {"terminal-auth": {"label": "Not executable", "command": "agent"}}
        });
        Rc::get_mut(&mut harness.connection)
            .expect("test harness should have the only connection handle")
            .auth_methods = vec![serde_json::from_value(unknown.clone()).expect("unknown method")];
        let unknown_id = acp_v2::AuthMethodId::new("unknown");
        cx.update(|cx| {
            assert!(
                harness
                    .connection
                    .terminal_auth_task(&unknown_id, cx)
                    .is_none()
            );
            harness.connection.authenticate(unknown_id, cx)
        })
        .await
        .expect_err("unknown auth must not use the agent-managed login RPC");
        assert_eq!(
            serde_json::to_value(harness.connection.auth_methods()).expect("shared methods"),
            serde_json::json!([unknown])
        );

        Rc::get_mut(&mut harness.connection)
            .expect("test harness should have the only connection handle")
            .auth_methods
            .clear();
        cx.update(|cx| harness.connection.authenticate(method_id, cx))
            .await
            .expect_err("unadvertised auth must not use the agent-managed login RPC");

        assert_eq!(harness.authenticate_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn terminal_auth_task_builds_spawn_from_prebuilt_command() {
        let command = AgentServerCommand {
            path: "/path/to/agent".into(),
            args: vec!["--acp".into(), "--verbose".into(), "/auth".into()],
            env: Some(HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("SHARED".into(), "override".into()),
                ("EXTRA".into(), "2".into()),
            ])),
        };
        let method = acp_v2::AuthMethodTerminal::new("login", "Login");

        let task = terminal_auth_task(&command, &AgentId::new("test-agent"), &method);

        assert_eq!(task.command.as_deref(), Some("/path/to/agent"));
        assert_eq!(task.args, vec!["--acp", "--verbose", "/auth"]);
        assert_eq!(
            task.env,
            HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("SHARED".into(), "override".into()),
                ("EXTRA".into(), "2".into()),
            ])
        );
        assert_eq!(task.label, "Login");
        assert_eq!(task.command_label, "Login");
    }

    #[test]
    fn legacy_terminal_auth_task_parses_meta_and_retries_session() {
        let method_id = acp_v2::AuthMethodId::new("legacy-login");
        let method = acp_v2::AuthMethod::Agent(
            acp_v2::AuthMethodAgent::new(method_id.clone(), "Login").meta(acp_v2::Meta::from_iter(
                [(
                    "terminal-auth".to_string(),
                    serde_json::json!({
                        "label": "legacy /auth",
                        "command": "legacy-agent",
                        "args": ["auth", "--interactive"],
                        "env": {
                            "AUTH_MODE": "interactive",
                        },
                    }),
                )],
            )),
        );

        let task = meta_terminal_auth_task(&AgentId::new("test-agent"), &method_id, &method)
            .expect("expected legacy terminal auth task");

        assert_eq!(task.id.0, "external-agent-test-agent-legacy-login-login");
        assert_eq!(task.command.as_deref(), Some("legacy-agent"));
        assert_eq!(task.args, vec!["auth", "--interactive"]);
        assert_eq!(
            task.env,
            HashMap::from_iter([("AUTH_MODE".into(), "interactive".into())])
        );
        assert_eq!(task.label, "legacy /auth");
    }

    #[test]
    fn legacy_terminal_auth_task_returns_none_for_invalid_meta() {
        let method_id = acp_v2::AuthMethodId::new("legacy-login");
        let method = acp_v2::AuthMethod::Agent(
            acp_v2::AuthMethodAgent::new(method_id.clone(), "Login").meta(acp_v2::Meta::from_iter(
                [(
                    "terminal-auth".to_string(),
                    serde_json::json!({
                        "label": "legacy /auth",
                    }),
                )],
            )),
        );

        assert!(
            meta_terminal_auth_task(&AgentId::new("test-agent"), &method_id, &method).is_none()
        );
    }

    #[test]
    fn first_class_terminal_auth_takes_precedence_over_legacy_meta() {
        let method_id = acp_v2::AuthMethodId::new("login");
        let method = acp_v2::AuthMethod::Terminal(
            acp_v2::AuthMethodTerminal::new(method_id, "Login")
                .args(vec!["/auth".into()])
                .env(vec![acp_v2::EnvVariable::new("AUTH_MODE", "first-class")])
                .meta(acp_v2::Meta::from_iter([(
                    "terminal-auth".to_string(),
                    serde_json::json!({
                        "label": "legacy /auth",
                        "command": "legacy-agent",
                        "args": ["legacy-auth"],
                        "env": {
                            "AUTH_MODE": "legacy",
                        },
                    }),
                )])),
        );

        let command = AgentServerCommand {
            path: "/path/to/agent".into(),
            args: vec!["--acp".into(), "/auth".into()],
            env: Some(HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("AUTH_MODE".into(), "first-class".into()),
            ])),
        };

        let task = match &method {
            acp_v2::AuthMethod::Terminal(terminal) => {
                terminal_auth_task(&command, &AgentId::new("test-agent"), terminal)
            }
            _ => unreachable!(),
        };

        assert_eq!(task.command.as_deref(), Some("/path/to/agent"));
        assert_eq!(task.args, vec!["--acp", "/auth"]);
        assert_eq!(
            task.env,
            HashMap::from_iter([
                ("BASE".into(), "1".into()),
                ("AUTH_MODE".into(), "first-class".into()),
            ])
        );
        assert_eq!(task.label, "Login");
    }

    #[test]
    fn session_directories_use_ordered_paths_when_supported() {
        let work_dirs = PathList::new(&[
            std::path::PathBuf::from("/workspace-b"),
            std::path::PathBuf::from("/workspace-a"),
            std::path::PathBuf::from("/workspace-c"),
        ]);

        let directories =
            session_directories_from_work_dirs(&work_dirs, true).expect("work dirs should convert");

        assert_eq!(
            directories,
            SessionDirectories {
                cwd: std::path::PathBuf::from("/workspace-b"),
                additional_directories: vec![
                    std::path::PathBuf::from("/workspace-a"),
                    std::path::PathBuf::from("/workspace-c")
                ],
            }
        );

        let session_id = acp_v2::SessionId::new("session-1");
        let new_session_request = directories.clone().into_new_session_request(Vec::new());
        let load_session_request = directories
            .clone()
            .into_load_session_request(session_id.clone(), Vec::new());
        let resume_session_request =
            directories.into_resume_session_request(session_id, Vec::new());

        assert_eq!(
            new_session_request.cwd,
            std::path::PathBuf::from("/workspace-b")
        );
        assert_eq!(
            new_session_request.additional_directories,
            vec![
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c")
            ]
        );
        assert_eq!(
            load_session_request.additional_directories,
            new_session_request.additional_directories
        );
        assert_eq!(
            resume_session_request.additional_directories,
            new_session_request.additional_directories
        );
    }

    #[test]
    fn legacy_session_requests_preserve_opaque_ids_and_backing_storage() {
        for value in ["", "001", " opaque/会話:007?x=%2F \n", "e\u{301}", "é"] {
            let backing: Arc<str> = value.into();
            let session_id = acp_v2::SessionId::new(backing.clone());
            let directories = SessionDirectories {
                cwd: PathBuf::from("/workspace"),
                additional_directories: Vec::new(),
            };
            let load = directories
                .clone()
                .into_load_session_request(session_id.clone(), Vec::new());
            let resume = directories.into_resume_session_request(session_id, Vec::new());

            for wire_id in [&load.session_id, &resume.session_id] {
                assert_eq!(wire_id.0.as_ref(), value);
                assert!(Arc::ptr_eq(&backing, &wire_id.0));
                assert_eq!(
                    serde_json::to_value(wire_id).expect("serialize legacy ID"),
                    serde_json::json!(value)
                );
            }
        }
    }

    #[test]
    fn session_directories_drop_additional_paths_when_unsupported() {
        let work_dirs = PathList::new(&[
            std::path::PathBuf::from("/workspace-b"),
            std::path::PathBuf::from("/workspace-a"),
        ]);

        let directories = session_directories_from_work_dirs(&work_dirs, false)
            .expect("work dirs should convert");

        assert_eq!(
            directories,
            SessionDirectories {
                cwd: std::path::PathBuf::from("/workspace-b"),
                additional_directories: Vec::new(),
            }
        );
    }

    #[test]
    fn session_info_work_dirs_preserve_cwd_then_additional_directories() {
        let work_dirs = work_dirs_from_session_info(
            std::path::PathBuf::from("/workspace-b"),
            vec![
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ],
        );

        assert_eq!(
            work_dirs.ordered_paths().cloned().collect::<Vec<_>>(),
            vec![
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ]
        );
    }

    #[test]
    fn session_info_work_dirs_deduplicate_cwd_and_additional_directories() {
        let work_dirs = work_dirs_from_session_info(
            std::path::PathBuf::from("/workspace-b"),
            vec![
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ],
        );

        assert_eq!(
            work_dirs.ordered_paths().cloned().collect::<Vec<_>>(),
            vec![
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ]
        );
    }

    #[gpui::test]
    async fn session_list_includes_additional_directories_in_work_dirs(
        cx: &mut gpui::TestAppContext,
    ) {
        let connection = connect_session_list_test_agent(
            vec![
                acp::SessionInfo::new("session-1", "/workspace-b").additional_directories(vec![
                    std::path::PathBuf::from("/workspace-a"),
                    std::path::PathBuf::from("/workspace-b"),
                    std::path::PathBuf::from("/workspace-a"),
                    std::path::PathBuf::from("/workspace-c"),
                ]),
            ],
            cx,
        )
        .await;
        let session_list = AcpSessionList::new(connection, false);

        let response = cx
            .update(|cx| session_list.list_sessions(AgentSessionListRequest::default(), cx))
            .await
            .expect("session list should load");
        let session = response
            .sessions
            .first()
            .expect("session list should include the returned session");
        let work_dirs = session
            .work_dirs
            .as_ref()
            .expect("session should include work dirs");

        assert_eq!(
            work_dirs.ordered_paths().cloned().collect::<Vec<_>>(),
            vec![
                std::path::PathBuf::from("/workspace-b"),
                std::path::PathBuf::from("/workspace-a"),
                std::path::PathBuf::from("/workspace-c"),
            ]
        );
    }

    async fn connect_session_list_test_agent(
        sessions: Vec<acp::SessionInfo>,
        cx: &mut gpui::TestAppContext,
    ) -> ConnectionTo<Agent> {
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        let sessions = Arc::new(sessions);

        cx.background_spawn(
            Agent
                .builder()
                .name("list-test-agent")
                .on_receive_request(
                    {
                        let sessions = sessions.clone();
                        async move |_request: acp::ListSessionsRequest, responder, _cx| {
                            responder.respond(acp::ListSessionsResponse::new((*sessions).clone()))
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        )
        .detach();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        cx.background_spawn(Client.builder().name("list-test-client").connect_with(
            client_transport,
            move |connection: ConnectionTo<Agent>| async move {
                connection_tx.send(connection).ok();
                futures::future::pending::<Result<(), acp::Error>>().await
            },
        ))
        .detach();

        connection_rx
            .await
            .expect("failed to receive ACP connection")
    }

    #[gpui::test]
    async fn additional_directories_support_respects_agent_capability(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {}, "b": {} }))
            .await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;
        let mut harness = test_support::connect_fake_acp_connection(project, cx).await;

        let work_dirs = PathList::new(&[
            std::path::PathBuf::from("/workspace-b"),
            std::path::PathBuf::from("/workspace-a"),
        ]);

        let missing_capability = harness
            .connection
            .session_directories_from_work_dirs(&work_dirs)
            .expect("work dirs should convert");
        assert!(missing_capability.additional_directories.is_empty());

        Rc::get_mut(&mut harness.connection)
            .expect("test harness should own the only ACP connection handle")
            .agent_capabilities
            .session_capabilities
            .additional_directories = Some(acp::SessionAdditionalDirectoriesCapabilities::new());

        let supported = harness
            .connection
            .session_directories_from_work_dirs(&work_dirs)
            .expect("work dirs should convert");
        assert_eq!(
            supported,
            SessionDirectories {
                cwd: std::path::PathBuf::from("/workspace-b"),
                additional_directories: vec![std::path::PathBuf::from("/workspace-a")],
            }
        );
    }

    async fn connect_session_delete_test_agent(
        deleted_sessions: Arc<std::sync::Mutex<Vec<acp::SessionId>>>,
        cx: &mut gpui::TestAppContext,
    ) -> ConnectionTo<Agent> {
        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();

        cx.background_spawn(
            Agent
                .builder()
                .name("delete-test-agent")
                .on_receive_request(
                    {
                        let deleted_sessions = deleted_sessions.clone();
                        async move |request: acp::DeleteSessionRequest, responder, _cx| {
                            deleted_sessions
                                .lock()
                                .expect("deleted sessions lock should not be poisoned")
                                .push(request.session_id);
                            responder.respond(acp::DeleteSessionResponse::default())
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        )
        .detach();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        cx.background_spawn(Client.builder().name("delete-test-client").connect_with(
            client_transport,
            move |connection: ConnectionTo<Agent>| async move {
                connection_tx.send(connection).ok();
                futures::future::pending::<Result<(), acp::Error>>().await
            },
        ))
        .detach();

        connection_rx
            .await
            .expect("failed to receive ACP connection")
    }

    #[gpui::test]
    async fn legacy_mode_set_preserves_wire_ids_and_rolls_back_on_failure(
        cx: &mut gpui::TestAppContext,
    ) {
        use acp_thread::AgentSessionModes as _;

        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();
        let requests = Arc::new(Mutex::new(Vec::new()));
        cx.background_spawn(
            Agent
                .builder()
                .on_receive_request(
                    {
                        let requests = requests.clone();
                        async move |request: acp::SetSessionModeRequest, responder, _cx| {
                            let rejected = request.mode_id.0.as_ref() == "reject";
                            requests.lock().expect("mode requests").push(request);
                            if rejected {
                                responder.respond_with_error(acp::Error::invalid_params())
                            } else {
                                responder.respond(acp::SetSessionModeResponse::default())
                            }
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        )
        .detach();
        let (sender, receiver) = futures::channel::oneshot::channel();
        cx.background_spawn(Client.builder().connect_with(
            client_transport,
            move |connection: ConnectionTo<Agent>| async move {
                sender.send(connection).expect("client connection");
                futures::future::pending::<Result<(), acp::Error>>().await
            },
        ))
        .detach();
        let modes = AcpSessionModes {
            session_id: acp_v2::SessionId::new("mode-session"),
            connection: receiver.await.expect("client connected"),
            state: Rc::new(RefCell::new(acp::SessionModeState::new(
                "auto",
                vec![
                    acp::SessionMode::new("auto", "Auto"),
                    acp::SessionMode::new("opaque/手動", "Manual"),
                ],
            ))),
        };
        let selected = acp_v2::SessionConfigValueId::new("opaque/手動");
        let change = cx.update(|cx| modes.set_mode(selected.clone(), cx));
        assert_eq!(modes.current_mode(), selected);
        change.await.expect("mode changed");
        let rejected = acp_v2::SessionConfigValueId::new("reject");
        let change = cx.update(|cx| modes.set_mode(rejected.clone(), cx));
        assert_eq!(modes.current_mode(), rejected);
        change.await.expect_err("peer rejected mode change");
        assert_eq!(modes.current_mode(), selected);
        let requests = requests.lock().expect("mode requests");
        assert_eq!(
            requests
                .iter()
                .map(|request| (request.session_id.to_string(), request.mode_id.to_string()))
                .collect::<Vec<_>>(),
            [
                ("mode-session".into(), "opaque/手動".into()),
                ("mode-session".into(), "reject".into())
            ],
        );
    }

    #[gpui::test]
    async fn settings_changes_refresh_active_connection_defaults(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;
        let harness = test_support::connect_fake_acp_connection(project, cx).await;

        cx.update(|cx| {
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::from_iter([(
                    "test".to_string(),
                    settings::CustomAgentServerSettings::Custom {
                        path: PathBuf::from("test-agent"),
                        args: Vec::new(),
                        env: HashMap::default(),
                        default_mode: Some("manual".to_string()),
                        default_config_options: HashMap::from_iter([(
                            "mode".to_string(),
                            AgentConfigOptionValue::from("config-value"),
                        )]),
                        favorite_config_option_values: HashMap::default(),
                    }
                    .into(),
                )])),
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            harness.connection.defaults.mode(),
            Some(acp_v2::SessionConfigValueId::new("manual"))
        );
        assert_eq!(
            harness
                .connection
                .defaults
                .config_option("mode")
                .as_ref()
                .and_then(AgentConfigOptionValue::as_value_id),
            Some("config-value"),
        );

        cx.update(|cx| {
            AllAgentServersSettings::override_global(
                AllAgentServersSettings(HashMap::default()),
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(harness.connection.defaults.mode(), None);
        assert_eq!(harness.connection.defaults.config_option("mode"), None);
    }

    #[gpui::test]
    async fn default_config_options_apply_boolean_defaults(cx: &mut gpui::TestAppContext) {
        let (connection, set_config_requests) = connect_config_defaults_test_agent_with_handler(
            |_| {
                Ok(acp::SetSessionConfigOptionResponse::new(vec![
                    acp::SessionConfigOption::boolean("web_search", "Web Search", true),
                ]))
            },
            cx,
        )
        .await;
        connection.defaults.set(
            None,
            HashMap::from_iter([(
                "web_search".to_string(),
                AgentConfigOptionValue::Boolean(true),
            )]),
        );
        let config_options = ConfigOptions::new(Rc::new(RefCell::new(vec![
            acp_v2::SessionConfigOption::boolean("web_search", "Web Search", false),
        ])));

        let mut async_cx = cx.to_async();
        let _default_config_task = connection.apply_default_config_options(
            &acp_v2::SessionId::new("session-config-defaults"),
            &config_options,
            &mut async_cx,
        );
        drop(async_cx);
        cx.run_until_parked();

        let requests = set_config_requests
            .lock()
            .expect("set config requests mutex poisoned");
        assert_eq!(
            requests.as_slice(),
            &[serde_json::json!({
                "sessionId": "session-config-defaults", "configId": "web_search",
                "type": "boolean", "value": true
            })]
        );

        let options = config_options.config_options.borrow();
        assert!(
            matches!(&options[0].kind, acp_v2::SessionConfigKind::Boolean(boolean) if boolean.current_value)
        );
    }

    #[gpui::test]
    async fn default_config_options_refresh_dependent_options(cx: &mut gpui::TestAppContext) {
        for (initial_efforts, updated_efforts, default_effort, reasoning_first) in [
            (vec!["medium", "high"], vec!["medium", "ultra"], None, false),
            (
                vec!["medium", "high"],
                vec!["medium", "ultra"],
                Some("ultra"),
                false,
            ),
            (vec![], vec!["medium", "ultra"], Some("ultra"), false),
            (
                vec!["medium", "high"],
                vec!["medium", "ultra"],
                Some("high"),
                false,
            ),
            (vec!["medium", "high"], vec![], Some("high"), false),
            (
                vec!["medium", "high"],
                vec!["medium", "ultra"],
                Some("ultra"),
                true,
            ),
            (
                vec!["medium", "ultra"],
                vec!["medium", "ultra"],
                Some("ultra"),
                true,
            ),
        ] {
            let expected_effort = if default_effort == Some("ultra") {
                "ultra"
            } else {
                "medium"
            };
            let mut initial_options =
                model_config_options("initial-model", "medium", &initial_efforts);
            if reasoning_first {
                initial_options.reverse();
            }
            let initially_supported = reasoning_first && initial_efforts.contains(&"ultra");
            let expected_options =
                model_config_options("new-model", expected_effort, &updated_efforts);
            let selected_model = Mutex::new("initial-model");
            let (connection, requests) = connect_config_defaults_test_agent_with_handler(
                move |request| {
                    let mut selected_model = selected_model.lock().expect("selected model");
                    let effort = if request.config_id == acp::SessionConfigId::new("model") {
                        assert_eq!(
                            request.value,
                            acp::SessionConfigOptionValue::value_id("new-model")
                        );
                        *selected_model = "new-model";
                        "medium"
                    } else {
                        assert_eq!(
                            request.config_id,
                            acp::SessionConfigId::new("reasoning_effort")
                        );
                        assert_eq!(
                            request.value,
                            acp::SessionConfigOptionValue::value_id("ultra")
                        );
                        "ultra"
                    };
                    let efforts = if *selected_model == "initial-model" {
                        &initial_efforts
                    } else {
                        &updated_efforts
                    };
                    Ok(acp::SetSessionConfigOptionResponse::new(
                        model_config_options(*selected_model, effort, efforts),
                    ))
                },
                cx,
            )
            .await;
            let mut defaults = HashMap::from_iter([(
                "model".to_string(),
                AgentConfigOptionValue::from("new-model"),
            )]);
            if let Some(effort) = default_effort {
                defaults.insert(
                    "reasoning_effort".to_string(),
                    AgentConfigOptionValue::from(effort),
                );
            }
            connection.defaults.set(None, defaults);
            let (_, config_options) =
                config_state(None, Some(initial_options)).expect("initial config");
            let config_options = config_options.expect("config options");
            let mut receiver = config_options.rx.clone();
            let _default_config_task = connection.apply_default_config_options(
                &acp_v2::SessionId::new("session"),
                &config_options,
                &mut cx.to_async(),
            );
            cx.run_until_parked();

            assert_eq!(
                *config_options.config_options.borrow(),
                acp_thread::config_options::from_v1(expected_options).expect("expected config"),
            );
            assert_eq!(receiver.changed().now_or_never(), Some(Ok(())));
            let mut expected_requests = vec![serde_json::json!({
                "sessionId": "session", "configId": "model", "value": "new-model"
            })];
            if initially_supported {
                expected_requests.insert(
                    0,
                    serde_json::json!({
                        "sessionId": "session", "configId": "reasoning_effort", "value": "ultra"
                    }),
                );
            }
            if default_effort == Some("ultra") {
                expected_requests.push(serde_json::json!({
                    "sessionId": "session", "configId": "reasoning_effort", "value": "ultra"
                }));
            }
            assert_eq!(*requests.lock().expect("requests"), expected_requests);
        }
    }

    fn model_config_options(
        model: &str,
        effort: &str,
        efforts: &[&str],
    ) -> Vec<acp::SessionConfigOption> {
        let mut options = vec![serde_json::json!({
            "id": "model", "name": "Model", "category": "model", "type": "select",
            "currentValue": model,
            "options": [
                {"value": "initial-model", "name": "Initial model"},
                {"value": "new-model", "name": "New model"}
            ]
        })];
        if !efforts.is_empty() {
            options.push(serde_json::json!({
                "id": "reasoning_effort", "name": "Reasoning effort", "category": "thought_level",
                "type": "select", "currentValue": effort,
                "options": efforts.iter().map(|value| serde_json::json!({
                    "value": value, "name": value
                })).collect::<Vec<_>>()
            }));
        }
        serde_json::from_value(serde_json::json!(options)).expect("model config options")
    }

    #[gpui::test]
    async fn default_config_options_preserve_state_on_error(cx: &mut gpui::TestAppContext) {
        let (connection, requests) = connect_config_defaults_test_agent_with_handler(
            |_| Err(acp::Error::invalid_params()),
            cx,
        )
        .await;
        connection.defaults.set(
            None,
            HashMap::from_iter([(
                "web_search".to_string(),
                AgentConfigOptionValue::Boolean(true),
            )]),
        );
        let original_options = vec![acp_v2::SessionConfigOption::boolean(
            "web_search",
            "Web Search",
            false,
        )];
        let config_options = ConfigOptions::new(Rc::new(RefCell::new(original_options.clone())));
        let mut receiver = config_options.rx.clone();
        let _default_config_task = connection.apply_default_config_options(
            &acp_v2::SessionId::new("session"),
            &config_options,
            &mut cx.to_async(),
        );
        cx.run_until_parked();

        assert_eq!(*config_options.config_options.borrow(), original_options);
        assert!(receiver.changed().now_or_never().is_none());
        assert_eq!(requests.lock().expect("requests").len(), 1);
    }

    #[gpui::test]
    async fn default_config_options_stop_when_defaults_conflict(cx: &mut gpui::TestAppContext) {
        let (connection, requests) = connect_config_defaults_test_agent_with_handler(
            |request| {
                Ok(acp::SetSessionConfigOptionResponse::new(vec![
                    acp::SessionConfigOption::boolean(
                        "first",
                        "First",
                        request.config_id.0.as_ref() == "first",
                    ),
                    acp::SessionConfigOption::boolean(
                        "second",
                        "Second",
                        request.config_id.0.as_ref() == "second",
                    ),
                ]))
            },
            cx,
        )
        .await;
        connection.defaults.set(
            None,
            HashMap::from_iter([
                ("first".to_string(), AgentConfigOptionValue::Boolean(true)),
                ("second".to_string(), AgentConfigOptionValue::Boolean(true)),
            ]),
        );
        let config_options = ConfigOptions::new(Rc::new(RefCell::new(vec![
            acp_v2::SessionConfigOption::boolean("first", "First", false),
            acp_v2::SessionConfigOption::boolean("second", "Second", false),
        ])));
        let _default_config_task = connection.apply_default_config_options(
            &acp_v2::SessionId::new("session"),
            &config_options,
            &mut cx.to_async(),
        );
        cx.run_until_parked();
        assert_eq!(requests.lock().expect("requests").len(), 3);
    }

    #[gpui::test]
    async fn default_config_options_cancel_on_session_release(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({"a": {}})).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;
        let (response_sender, response_receiver) = async_channel::bounded::<()>(1);
        let (connection, requests) = connect_config_defaults_test_agent_with_async_handler(
            move |request| {
                let response_receiver = response_receiver.clone();
                async move {
                    assert_eq!(request.config_id, acp::SessionConfigId::new("model"));
                    response_receiver
                        .recv()
                        .await
                        .expect("release model response");
                    Ok(acp::SetSessionConfigOptionResponse::new(
                        model_config_options("new-model", "medium", &["medium", "ultra"]),
                    ))
                }
            },
            cx,
        )
        .await;
        let connection = Rc::new(connection);
        connection.defaults.set(
            None,
            HashMap::from_iter([
                (
                    "model".to_string(),
                    AgentConfigOptionValue::from("new-model"),
                ),
                (
                    "reasoning_effort".to_string(),
                    AgentConfigOptionValue::from("ultra"),
                ),
            ]),
        );
        let session_id = acp_v2::SessionId::new("session");
        let initial_options = model_config_options("initial-model", "medium", &["medium"]);
        let thread = cx
            .update(|cx| {
                connection.clone().open_or_create_session(
                    session_id.clone(),
                    project.clone(),
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    move |_, _, _| {
                        async move {
                            Ok(SessionConfigResponse {
                                modes: None,
                                config_options: Some(initial_options),
                            })
                        }
                        .boxed_local()
                    },
                    cx,
                )
            })
            .await
            .expect("load initial session");
        cx.run_until_parked();
        assert_eq!(requests.lock().expect("requests").len(), 1);
        drop(thread);
        release_dropped_entities(cx);

        let replacement_options = vec![acp::SessionConfigOption::boolean(
            "web_search",
            "Web Search",
            false,
        )];
        let expected_options = acp_thread::config_options::from_v1(replacement_options.clone())
            .expect("replacement config");
        let _replacement_thread = cx
            .update(|cx| {
                connection.clone().open_or_create_session(
                    session_id.clone(),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    move |_, _, _| {
                        async move {
                            Ok(SessionConfigResponse {
                                modes: None,
                                config_options: Some(replacement_options),
                            })
                        }
                        .boxed_local()
                    },
                    cx,
                )
            })
            .await
            .expect("load replacement session");
        response_sender
            .send(())
            .await
            .expect("unblock stale response");
        cx.run_until_parked();

        assert_eq!(requests.lock().expect("requests").len(), 1);
        let sessions = connection.sessions.borrow();
        let config_options = sessions
            .get(&session_id)
            .expect("replacement session")
            .config_options
            .as_ref()
            .expect("replacement config");
        assert_eq!(*config_options.config_options.borrow(), expected_options);
    }

    #[gpui::test]
    async fn shared_config_values_use_v1_wire_shapes(cx: &mut gpui::TestAppContext) {
        use acp_thread::AgentSessionConfigOptions;

        let (connection, requests) = connect_config_defaults_test_agent(cx).await;
        let config = ConfigOptions::new(Rc::new(RefCell::new(vec![
            acp_v2::SessionConfigOption::boolean("config", "Config", false),
        ])));
        let options = AcpSessionConfigOptions {
            session_id: acp_v2::SessionId::new("session"),
            connection: connection.connection.clone(),
            config_options: config,
        };
        let original = options.config_options();
        let unknown =
            acp_v2::SessionConfigOptionValue::Other(acp_v2::OtherSessionConfigOptionValue::new(
                "_custom",
                serde_json::json!("private-value"),
                Default::default(),
            ));
        assert!(
            cx.update(|cx| options.set_config_option("config".into(), unknown, cx))
                .await
                .is_err()
        );
        assert_eq!(options.config_options(), original);
        assert!(requests.lock().expect("requests").is_empty());

        for value in [
            acp_v2::SessionConfigOptionValue::id("choice"),
            acp_v2::SessionConfigOptionValue::boolean(true),
        ] {
            let updated = cx
                .update(|cx| options.set_config_option("config".into(), value, cx))
                .await
                .expect("supported v1 value");
            assert!(updated.is_empty());
            assert_eq!(options.config_options(), updated);
        }
        assert_eq!(
            requests.lock().expect("requests").as_slice(),
            &[
                serde_json::json!({"sessionId": "session", "configId": "config", "value": "choice"}),
                serde_json::json!({
                    "sessionId": "session", "configId": "config", "type": "boolean", "value": true
                })
            ]
        );
    }

    async fn connect_config_defaults_test_agent(
        cx: &mut gpui::TestAppContext,
    ) -> (AcpConnection, Arc<Mutex<Vec<serde_json::Value>>>) {
        connect_config_defaults_test_agent_with_handler(
            |_| Ok(acp::SetSessionConfigOptionResponse::new(Vec::new())),
            cx,
        )
        .await
    }

    async fn connect_config_defaults_test_agent_with_handler(
        handler: impl Fn(
            acp::SetSessionConfigOptionRequest,
        ) -> Result<acp::SetSessionConfigOptionResponse, acp::Error>
        + Send
        + Sync
        + 'static,
        cx: &mut gpui::TestAppContext,
    ) -> (AcpConnection, Arc<Mutex<Vec<serde_json::Value>>>) {
        connect_config_defaults_test_agent_with_async_handler(
            move |request| futures::future::ready(handler(request)),
            cx,
        )
        .await
    }

    async fn connect_config_defaults_test_agent_with_async_handler<
        F: Future<Output = Result<acp::SetSessionConfigOptionResponse, acp::Error>> + Send + 'static,
    >(
        handler: impl Fn(acp::SetSessionConfigOptionRequest) -> F + Send + Sync + 'static,
        cx: &mut gpui::TestAppContext,
    ) -> (AcpConnection, Arc<Mutex<Vec<serde_json::Value>>>) {
        let set_config_requests = Arc::new(Mutex::new(Vec::new()));
        let (client_transport, client_inspection) = agent_client_protocol::Channel::duplex();
        let (agent_inspection, agent_transport) = agent_client_protocol::Channel::duplex();
        cx.background_spawn({
            let set_config_requests = set_config_requests.clone();
            async move {
                agent_client_protocol::Channel::bridge_with_inspection(
                    client_inspection,
                    agent_inspection,
                    move |message| {
                        if let agent_client_protocol::RawJsonRpcMessage::Request(request) = message
                            && request.method.as_ref() == "session/set_config_option"
                        {
                            set_config_requests
                                .lock()
                                .expect("set config requests mutex poisoned")
                                .push(
                                    serde_json::to_value(&request.params)
                                        .expect("raw config request parameters"),
                                );
                        }
                        Ok(())
                    },
                    |_| Ok(()),
                )
                .await
                .log_err();
            }
        })
        .detach();

        cx.background_spawn(
            Agent
                .builder()
                .name("config-defaults-test-agent")
                .on_receive_request(
                    async move |request: acp::SetSessionConfigOptionRequest, responder, _cx| {
                        match handler(request).await {
                            Ok(response) => responder.respond(response),
                            Err(error) => responder.respond_with_error(error),
                        }
                    },
                    agent_client_protocol::on_receive_request!(),
                )
                .connect_to(agent_transport),
        )
        .detach();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let client_io_task = cx.background_spawn(async move {
            Client
                .builder()
                .name("config-defaults-test-client")
                .connect_with(
                    client_transport,
                    move |connection: ConnectionTo<Agent>| async move {
                        connection_tx.send(connection).ok();
                        futures::future::pending::<Result<(), acp::Error>>().await
                    },
                )
                .await
                .ok();
        });

        let client_conn = connection_rx
            .await
            .expect("failed to receive ACP connection");
        let sessions = Rc::new(RefCell::new(HashMap::default()));

        let connection = cx.update(|cx| {
            let request_elicitations = cx.new(|_| ElicitationStore::default());
            AcpConnection::new_for_test(
                client_conn,
                sessions,
                acp::AgentCapabilities::default(),
                request_elicitations,
                WeakEntity::new_invalid(),
                client_io_task,
                mpsc::unbounded().0,
                Task::ready(()),
                cx,
            )
        });

        (connection, set_config_requests)
    }

    #[gpui::test]
    async fn session_list_delete_sends_session_delete_when_supported(
        cx: &mut gpui::TestAppContext,
    ) {
        let deleted_sessions = Arc::new(std::sync::Mutex::new(Vec::new()));
        let connection = connect_session_delete_test_agent(deleted_sessions.clone(), cx).await;
        let session_list = AcpSessionList::new(connection, true);
        let session_id = acp_v2::SessionId::new(" opaque/会話:007?x=%2F \n");

        cx.update(|cx| session_list.delete_session(&session_id, cx))
            .await
            .expect("delete_session failed");

        assert_eq!(
            *deleted_sessions
                .lock()
                .expect("deleted sessions lock should not be poisoned"),
            vec![acp::SessionId::new(session_id.0)]
        );
    }

    #[gpui::test]
    async fn session_list_delete_does_not_send_when_unsupported(cx: &mut gpui::TestAppContext) {
        let deleted_sessions = Arc::new(std::sync::Mutex::new(Vec::new()));
        let connection = connect_session_delete_test_agent(deleted_sessions.clone(), cx).await;
        let session_list = AcpSessionList::new(connection, false);
        let session_id = acp_v2::SessionId::new("session-to-delete");

        let error = cx
            .update(|cx| session_list.delete_session(&session_id, cx))
            .await
            .expect_err("delete_session should fail when unsupported");

        assert!(
            error.to_string().contains("delete_session not supported"),
            "unexpected error: {error}"
        );
        assert!(
            deleted_sessions
                .lock()
                .expect("deleted sessions lock should not be poisoned")
                .is_empty()
        );
    }

    #[cfg(not(windows))]
    #[gpui::test]
    async fn startup_returns_error_when_agent_exits_before_initialization(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });
        cx.executor().allow_parking();

        let temp_dir = tempfile::tempdir().unwrap();
        let project = project::Project::example([temp_dir.path()], &mut cx.to_async()).await;
        let agent_server_store =
            project.read_with(cx, |project, _| project.agent_server_store().downgrade());
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec![
                "-c".into(),
                r#"printf '%s\n' 'npm error code ETARGET' 'npm error notarget No matching version found for @agentclientprotocol/claude-agent-acp@0.32.0 with a date before 4/28/2026, 12:11:38 PM.' >&2; exit 1"#.into(),
            ],
            env: None,
        };

        let mut async_cx = cx.to_async();
        let startup = AcpConnection::stdio(
            AgentId::new("test-agent"),
            project,
            command,
            agent_server_store,
            None,
            HashMap::default(),
            &mut async_cx,
        )
        .fuse();
        let timeout = cx
            .background_executor
            .timer(std::time::Duration::from_secs(5))
            .fuse();
        futures::pin_mut!(startup, timeout);

        let result = futures::select! {
            result = startup => result,
            _ = timeout => panic!("timed out waiting for failed ACP startup"),
        };

        let Err(error) = result else {
            panic!("expected ACP startup to fail");
        };
        let load_error = error
            .downcast::<LoadError>()
            .expect("startup failure should preserve the typed load error");
        match load_error {
            LoadError::Exited { status, stderr } => {
                assert!(!status.success(), "expected non-zero exit status");
                assert_eq!(
                    stderr.as_deref(),
                    Some(
                        "npm error code ETARGET\nnpm error notarget No matching version found for @agentclientprotocol/claude-agent-acp@0.32.0 with a date before 4/28/2026, 12:11:38 PM."
                    ),
                    "startup failure should retain the agent's final stderr"
                );
            }
            error => panic!("expected exited load error, got: {error:?}"),
        };
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn agent_exit_preserves_final_updates_and_diagnostics(cx: &mut gpui::TestAppContext) {
        assert_agent_exit_preserves_final_output(false, cx).await;
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn agent_exit_with_inherited_handles_preserves_final_output(
        cx: &mut gpui::TestAppContext,
    ) {
        assert_agent_exit_preserves_final_output(true, cx).await;
    }

    #[cfg(unix)]
    async fn assert_agent_exit_preserves_final_output(
        inherit_handles: bool,
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });
        cx.executor().allow_parking();

        let directory = tempfile::tempdir().expect("create working directory");
        let project = project::Project::example([directory.path()], &mut cx.to_async()).await;
        let agent_server_store =
            project.read_with(cx, |project, _| project.agent_server_store().downgrade());
        let session_id = acp::SessionId::new("exiting-agent");
        let initialize_response =
            serde_json::to_string(&acp::InitializeResponse::new(ProtocolVersion::V1))
                .expect("serialize initialization");
        let session_response =
            serde_json::to_string(&acp::NewSessionResponse::new(session_id.clone()))
                .expect("serialize session");
        let prompt_response =
            serde_json::to_string(&acp::PromptResponse::new(acp::StopReason::EndTurn))
                .expect("serialize prompt response");
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": acp::SessionNotification::new(
                session_id,
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                    "final answer before exit".into(),
                )),
            ),
        });
        let inherited_handle_script = inherit_handles.then_some(
            r#"
exec 3<&0
cat <&3 &
pipe_holder=$!
(sleep 2; kill "$pipe_holder" 2>/dev/null) </dev/null >/dev/null 2>&1 &"#,
        );
        let script = format!(
            r#"
read_request() {{
    IFS= read -r request || exit 2
    request_id=$(printf '%s' "$request" | sed -n 's/.*"id":\("[^"]*"\).*/\1/p')
    test -n "$request_id" || exit 3
}}
respond() {{
    printf '{{"jsonrpc":"2.0","id":%s,"result":%s}}\n' "$request_id" "$1"
}}
read_request
respond '{initialize_response}'
read_request
respond '{session_response}'
read_request
printf '%s\n' '{notification}'
respond '{prompt_response}'
printf '%s\n' 'final diagnostic before exit' >&2
{}
exit 7
"#,
            inherited_handle_script.unwrap_or_default(),
        );
        let command = AgentServerCommand {
            path: "/bin/sh".into(),
            args: vec!["-c".into(), script],
            env: None,
        };
        let timeout = cx
            .background_executor
            .timer(std::time::Duration::from_secs(5))
            .fuse();
        let exchange = async {
            let connection = Rc::new(
                AcpConnection::stdio(
                    AgentId::new("exiting-agent"),
                    project.clone(),
                    command,
                    agent_server_store,
                    None,
                    HashMap::default(),
                    &mut cx.to_async(),
                )
                .await
                .expect("initialize scripted agent"),
            );
            let thread = cx
                .update(|cx| {
                    connection.clone().new_session(
                        project,
                        PathList::new(&[directory.path()]),
                        cx,
                    )
                })
                .await
                .expect("create scripted session");
            let (exit_sender, exit_receiver) = async_channel::unbounded();
            let _subscription = cx.update(|cx| {
                let debug_log = connection.debug_log.clone();
                let connection = connection.connection.clone();
                cx.subscribe(&thread, move |thread, event, cx| {
                    if let acp_thread::AcpThreadEvent::LoadError(error) = event {
                        let (messages, _receiver) = debug_log.subscribe();
                        let stderr_captured = messages.iter().any(|message| {
                            matches!(
                                &message.message,
                                AcpDebugMessageContent::Stderr { line }
                                    if line.as_ref() == "final diagnostic before exit"
                            )
                        });
                        exit_sender
                            .try_send((
                                thread.read(cx).to_markdown(cx),
                                error.clone(),
                                connection.is_incoming_closed(),
                                stderr_captured,
                            ))
                            .expect("capture exit event");
                    }
                })
            });
            let response = thread
                .update(cx, |thread, cx| thread.send_raw("Respond, then exit", cx))
                .await
                .expect("the final prompt response should arrive before closure")
                .expect("prompt should finish");
            assert_eq!(response.stop_reason, acp::StopReason::EndTurn);

            let (transcript_at_exit, error, incoming_closed_at_exit, stderr_captured_at_exit) =
                exit_receiver.recv().await.expect("receive process exit");
            if inherit_handles {
                assert!(
                    !incoming_closed_at_exit,
                    "the descendant should still hold the agent's stdout open"
                );
            } else {
                assert!(
                    incoming_closed_at_exit,
                    "normal process exit should reach clean stdout EOF before the exit event"
                );
            }
            cx.run_until_parked();
            let final_transcript = thread.read_with(cx, |thread, cx| thread.to_markdown(cx));
            assert!(
                final_transcript.contains("final answer before exit"),
                "the scripted agent's final notification should reach the thread"
            );
            assert!(
                transcript_at_exit.contains("final answer before exit"),
                "final notification should be applied before reporting exit: at exit {transcript_at_exit:?}, after settling {final_transcript:?}"
            );
            let LoadError::Exited { status, .. } = error else {
                panic!("expected process exit, got {error:?}");
            };
            assert_eq!(status.code(), Some(7));
            // Pipe reads can interleave, so stderr need not be the debug log's final block.
            assert!(
                stderr_captured_at_exit,
                "the final diagnostic should be logged before reporting exit"
            );
            if inherit_handles {
                connection.connection.incoming_closed().await;
            }
        }
        .fuse();
        futures::pin_mut!(exchange, timeout);
        futures::select! {
            _ = exchange => {}
            _ = timeout => panic!("timed out waiting for scripted agent exit"),
        }
    }

    async fn connect_fake_agent(
        close_session_gate: Option<async_channel::Receiver<()>>,
        cx: &mut gpui::TestAppContext,
    ) -> (
        Rc<AcpConnection>,
        Entity<project::Project>,
        Arc<AtomicUsize>,
        Arc<AtomicUsize>,
        Arc<std::sync::Mutex<Vec<acp::SessionUpdate>>>,
        Arc<std::sync::Mutex<Option<async_channel::Receiver<()>>>>,
        Task<anyhow::Result<()>>,
    ) {
        connect_fake_agent_with_handle(close_session_gate, None, cx).await
    }

    async fn connect_fake_agent_with_handle(
        close_session_gate: Option<async_channel::Receiver<()>>,
        agent_sender: Option<futures::channel::oneshot::Sender<ConnectionTo<Client>>>,
        cx: &mut gpui::TestAppContext,
    ) -> (
        Rc<AcpConnection>,
        Entity<project::Project>,
        Arc<AtomicUsize>,
        Arc<AtomicUsize>,
        Arc<std::sync::Mutex<Vec<acp::SessionUpdate>>>,
        Arc<std::sync::Mutex<Option<async_channel::Receiver<()>>>>,
        Task<anyhow::Result<()>>,
    ) {
        connect_fake_agent_with_new_session_behavior(close_session_gate, agent_sender, None, cx)
            .await
    }

    struct NewSessionTestBehavior {
        received: async_channel::Sender<agent_client_protocol::RequestCancellation>,
        respond: async_channel::Receiver<Result<acp::NewSessionResponse, acp::Error>>,
        updates: Vec<acp::SessionUpdate>,
    }

    async fn connect_fake_agent_with_new_session_behavior(
        close_session_gate: Option<async_channel::Receiver<()>>,
        agent_sender: Option<futures::channel::oneshot::Sender<ConnectionTo<Client>>>,
        new_session_behavior: Option<NewSessionTestBehavior>,
        cx: &mut gpui::TestAppContext,
    ) -> (
        Rc<AcpConnection>,
        Entity<project::Project>,
        Arc<AtomicUsize>,
        Arc<AtomicUsize>,
        Arc<std::sync::Mutex<Vec<acp::SessionUpdate>>>,
        Arc<std::sync::Mutex<Option<async_channel::Receiver<()>>>>,
        Task<anyhow::Result<()>>,
    ) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/", serde_json::json!({ "a": {} })).await;
        let project = project::Project::test(fs, [std::path::Path::new("/a")], cx).await;

        let load_count = Arc::new(AtomicUsize::new(0));
        let close_count = Arc::new(AtomicUsize::new(0));
        let load_session_updates: Arc<std::sync::Mutex<Vec<acp::SessionUpdate>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let load_session_gate: Arc<std::sync::Mutex<Option<async_channel::Receiver<()>>>> =
            Arc::new(std::sync::Mutex::new(None));
        let close_session_gate = Arc::new(std::sync::Mutex::new(close_session_gate));
        let new_session_behavior = Arc::new(Mutex::new(new_session_behavior));

        let (client_transport, agent_transport) = agent_client_protocol::Channel::duplex();

        let sessions: Rc<RefCell<HashMap<acp_v2::SessionId, AcpSession>>> =
            Rc::new(RefCell::new(HashMap::default()));
        let client_session_list: Rc<RefCell<Option<Rc<AcpSessionList>>>> =
            Rc::new(RefCell::new(None));

        // Build the fake agent side. It handles the requests issued by
        // `AcpConnection` during the test and tracks load/close counts.
        let agent_future = Agent
            .builder()
            .name("fake-agent")
            .on_receive_request(
                async move |req: acp::InitializeRequest, responder, _cx| {
                    responder.respond(
                        acp::InitializeResponse::new(req.protocol_version).agent_capabilities(
                            acp::AgentCapabilities::default()
                                .load_session(true)
                                .session_capabilities(
                                    acp::SessionCapabilities::default()
                                        .close(acp::SessionCloseCapabilities::new()),
                                ),
                        ),
                    )
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::AuthenticateRequest, responder, _cx| {
                    responder.respond(Default::default())
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::NewSessionRequest, responder, cx| {
                    let behavior = new_session_behavior
                        .lock()
                        .expect("new session behavior")
                        .take();
                    if let Some(behavior) = behavior {
                        let connection = cx.clone();
                        cx.spawn(async move {
                            behavior
                                .received
                                .send(responder.cancellation())
                                .await
                                .expect("notify new request");
                            let response =
                                behavior.respond.recv().await.expect("release new response");
                            let session_id = response
                                .as_ref()
                                .ok()
                                .map(|response| response.session_id.clone());
                            responder.respond_with_result(response)?;
                            if let Some(session_id) = session_id {
                                for update in behavior.updates {
                                    connection.send_notification(acp::SessionNotification::new(
                                        session_id.clone(),
                                        update,
                                    ))?;
                                }
                            }
                            Ok(())
                        })
                    } else {
                        responder
                            .respond(acp::NewSessionResponse::new(acp::SessionId::new("unused")))
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                async move |_req: acp::PromptRequest, responder, _cx| {
                    responder.respond(acp::PromptResponse::new(acp::StopReason::EndTurn))
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let load_count = load_count.clone();
                    let load_session_updates = load_session_updates.clone();
                    let load_session_gate = load_session_gate.clone();
                    async move |req: acp::LoadSessionRequest, responder, cx| {
                        load_count.fetch_add(1, Ordering::SeqCst);

                        // Simulate spec-compliant history replay: send
                        // notifications to the client before responding to the
                        // load request.
                        let updates = std::mem::take(
                            &mut *load_session_updates
                                .lock()
                                .expect("load_session_updates mutex poisoned"),
                        );
                        for update in updates {
                            cx.send_notification(acp::SessionNotification::new(
                                req.session_id.clone(),
                                update,
                            ))?;
                        }

                        // If a gate was installed, park on it before responding
                        // so tests can interleave other work (e.g.
                        // `close_session`) with an in-flight load.
                        let gate = load_session_gate
                            .lock()
                            .expect("load_session_gate mutex poisoned")
                            .take();
                        if let Some(gate) = gate {
                            gate.recv().await.ok();
                        }

                        responder.respond(acp::LoadSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_request(
                {
                    let close_count = close_count.clone();
                    let close_session_gate = close_session_gate.clone();
                    async move |_req: acp::CloseSessionRequest, responder, _cx| {
                        close_count.fetch_add(1, Ordering::SeqCst);
                        let gate = close_session_gate
                            .lock()
                            .expect("close_session_gate mutex poisoned")
                            .take();
                        if let Some(gate) = gate {
                            gate.recv().await.ok();
                        }
                        responder.respond(acp::CloseSessionResponse::new())
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .on_receive_notification(
                async move |_notif: acp::CancelNotification, _cx| Ok(()),
                agent_client_protocol::on_receive_notification!(),
            )
            .connect_with(agent_transport, async move |agent| {
                if let Some(sender) = agent_sender {
                    assert!(sender.send(agent).is_ok(), "receive fake agent handle");
                }
                futures::future::pending::<Result<(), agent_client_protocol::Error>>().await
            });

        let agent_io_task = cx.background_spawn(agent_future);

        // Wire the production handler set into the fake client so inbound
        // requests/notifications from the fake agent reach the same
        // dispatcher that the real `stdio` path uses.
        let (dispatch_tx, dispatch_rx) = mpsc::unbounded::<ForegroundWork>();

        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let client_future = connect_client_future(
            "zed-test",
            client_transport,
            dispatch_tx.clone(),
            connection_tx,
            futures::future::pending(),
        );
        let client_io_task = cx.background_spawn(async move {
            client_future.await.ok();
        });

        let client_conn: ConnectionTo<Agent> = connection_rx
            .await
            .expect("failed to receive ACP connection handle");

        let response = client_conn
            .send_request(
                acp::InitializeRequest::new(ProtocolVersion::V1).client_capabilities(
                    client_capabilities_for_agent(&AgentId::new("fake-agent")),
                ),
            )
            .block_task()
            .await
            .expect("failed to initialize ACP connection");

        let agent_capabilities = response.agent_capabilities;

        let request_elicitations = cx.new(|_| ElicitationStore::default());
        let dispatch_context = ClientContext {
            sessions: sessions.clone(),
            session_list: client_session_list.clone(),
            request_elicitations: request_elicitations.clone(),
            active_request_scopes: None,
        };
        // `TestAppContext::spawn` hands out an `AsyncApp` by value, whereas the
        // production path uses `Context::spawn` which hands out `&mut AsyncApp`.
        // Bind the value-form to a local and take `&mut` of it to reuse the
        // same dispatch loop shape.
        let dispatch_task = cx.spawn({
            let mut dispatch_rx = dispatch_rx;
            move |cx| async move {
                let mut cx = cx;
                while let Some(work) = dispatch_rx.next().await {
                    work.run(&mut cx, &dispatch_context);
                }
            }
        });

        let agent_server_store =
            project.read_with(cx, |project, _| project.agent_server_store().downgrade());

        let connection = cx.update(|cx| {
            AcpConnection::new_for_test(
                client_conn,
                sessions,
                agent_capabilities,
                request_elicitations,
                agent_server_store,
                client_io_task,
                dispatch_tx,
                dispatch_task,
                cx,
            )
        });

        let keep_agent_alive = cx.background_spawn(async move {
            agent_io_task.await.ok();
            anyhow::Ok(())
        });

        (
            Rc::new(connection),
            project,
            load_count,
            close_count,
            load_session_updates,
            load_session_gate,
            keep_agent_alive,
        )
    }

    fn drain_sdk_background_tasks(cx: &gpui::TestAppContext) {
        let executor = cx.executor();
        let dispatcher = executor
            .dispatcher()
            .as_test()
            .expect("deterministic test dispatcher");
        for _ in 0..1000 {
            if !dispatcher.tick(true) {
                return;
            }
        }
        panic!("SDK dispatch did not drain within bounded ticks");
    }

    #[gpui::test]
    async fn eof_cancels_elicitations_for_retained_idle_owners(cx: &mut gpui::TestAppContext) {
        let (agent_sender, agent_receiver) = futures::channel::oneshot::channel();
        let (connection, project, _, _, _, _, keep_agent_alive) =
            connect_fake_agent_with_handle(None, Some(agent_sender), cx).await;
        let agent = agent_receiver.await.expect("fake agent handle");
        let session_id = acp_v2::SessionId::new("retained-idle-session");
        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    cx,
                )
            })
            .await
            .expect("load retained idle session");
        let store = connection
            .request_elicitations()
            .expect("connection elicitation store");
        let session_request = acp::CreateElicitationRequest::new(
            acp::ElicitationFormMode::new(
                acp::ElicitationSessionScope::new(acp::SessionId::new(session_id.0)),
                acp::ElicitationSchema::new().string("name", true),
            ),
            "Session question",
        );
        let request_request = acp::CreateElicitationRequest::new(
            acp::ElicitationFormMode::new(
                acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                acp::ElicitationSchema::new().string("name", true),
            ),
            "Connection question",
        );
        let session_rpc = agent.send_request(session_request.clone());
        let request_rpc = agent.send_request(request_request.clone());
        cx.run_until_parked();

        let session_entry = thread.read_with(cx, |thread, _| {
            assert_eq!(thread.status(), acp_thread::ThreadStatus::Idle);
            thread
                .entries()
                .iter()
                .find_map(|entry| match entry {
                    acp_thread::AgentThreadEntry::Elicitation(id) => Some(id.clone()),
                    _ => None,
                })
                .expect("session elicitation")
        });
        let request_entry = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!("expected one connection elicitation");
            };
            assert!(matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Pending { .. }
            ));
            elicitation.id.clone()
        });
        let session_waiter = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_thread::elicitation::request_from_v1(session_request.clone())
                        .expect("shared session request"),
                    cx,
                )
                .expect("session response waiter")
        });
        let request_waiter = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_thread::elicitation::request_from_v1(request_request.clone())
                        .expect("shared connection request"),
                    cx,
                )
                .expect("connection response waiter")
        });
        let unrelated_store = cx.new(|_| ElicitationStore::default());
        let unrelated_waiter = unrelated_store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_thread::elicitation::request_from_v1(request_request.clone())
                        .expect("unrelated request"),
                    cx,
                )
                .expect("unrelated response waiter")
        });
        let queued_session_rpc = agent.send_request(session_request);
        let queued_request_rpc = agent.send_request(request_request);
        drain_sdk_background_tasks(cx);
        assert_eq!(
            thread.read_with(cx, |thread, _| {
                thread
                    .entries()
                    .iter()
                    .filter(|entry| matches!(entry, acp_thread::AgentThreadEntry::Elicitation(_)))
                    .count()
            }),
            2,
            "queued session request must not be applied before foreground dispatch"
        );
        assert_eq!(
            store.read_with(cx, |store, _| store.elicitations().len()),
            2
        );

        drop(keep_agent_alive);
        connection.connection.incoming_closed().await;
        cx.run_until_parked();

        thread.read_with(cx, |thread, _| {
            let (_, elicitation) = thread.elicitation(&session_entry).expect("retained entry");
            assert!(matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Canceled
            ));
            let entries = thread
                .entries()
                .iter()
                .filter_map(|entry| match entry {
                    acp_thread::AgentThreadEntry::Elicitation(id) => {
                        Some(thread.elicitation(id).expect("elicitation entry").1)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                entries.len(),
                3,
                "queued request must register before cleanup"
            );
            assert!(
                entries
                    .iter()
                    .all(|entry| matches!(entry.status, acp_thread::ElicitationStatus::Canceled))
            );
            assert_eq!(thread.status(), acp_thread::ThreadStatus::Idle);
        });
        store.read_with(cx, |store, _| {
            let (_, elicitation) = store.elicitation(&request_entry).expect("retained entry");
            assert!(matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Canceled
            ));
            assert_eq!(store.elicitations().len(), 3);
            assert!(
                store
                    .elicitations()
                    .iter()
                    .all(|entry| matches!(entry.status, acp_thread::ElicitationStatus::Canceled))
            );
        });
        assert_eq!(
            session_waiter.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        assert_eq!(
            request_waiter.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        unrelated_store.update(cx, |store, cx| {
            assert!(matches!(
                store
                    .elicitations()
                    .first()
                    .expect("unrelated entry")
                    .status,
                acp_thread::ElicitationStatus::Pending { .. }
            ));
            store.cancel_all(cx);
        });
        assert_eq!(
            unrelated_waiter.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        drop((
            session_rpc,
            request_rpc,
            queued_session_rpc,
            queued_request_rpc,
        ));
    }

    #[gpui::test]
    async fn eof_preserves_queued_elicitation_completion(cx: &mut gpui::TestAppContext) {
        let (agent_sender, agent_receiver) = futures::channel::oneshot::channel();
        let (connection, project, _, _, _, _, keep_agent_alive) =
            connect_fake_agent_with_handle(None, Some(agent_sender), cx).await;
        let agent = agent_receiver.await.expect("fake agent handle");
        let session_id = acp_v2::SessionId::new("completion-session");
        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    cx,
                )
            })
            .await
            .expect("load completion session");
        let store = connection
            .request_elicitations()
            .expect("connection elicitation store");
        let session_scope =
            acp_v2::ElicitationScope::Session(acp_v2::ElicitationSessionScope::new(session_id));
        let request_scope = acp_v2::ElicitationScope::Request(
            acp_v2::ElicitationRequestScope::new(acp_v2::RequestId::Number(1)),
        );
        let url_request = |scope: acp_v2::ElicitationScope, id: &str| {
            acp_v2::CreateElicitationRequest::new(
                acp_v2::ElicitationUrlMode::new(
                    scope,
                    acp_v2::ElicitationId::new(id),
                    "https://auth.example.com/device",
                ),
                "Authorize in your browser",
            )
        };
        let accept = || {
            acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                acp_v2::ElicitationAcceptAction::new(),
            ))
        };
        let mut session_entries = Vec::new();
        for id in ["session-completed", "session-incomplete"] {
            let (entry, response) = thread.update(cx, |thread, cx| {
                thread
                    .request_elicitation_with_id(url_request(session_scope.clone(), id), cx)
                    .expect("session URL elicitation")
            });
            thread.update(cx, |thread, cx| {
                thread.respond_to_elicitation(&entry, accept(), cx);
            });
            assert_eq!(response.await, accept());
            session_entries.push(entry);
        }
        let mut request_entries = Vec::new();
        for id in ["request-completed", "request-incomplete"] {
            let (entry, response) = store.update(cx, |store, cx| {
                store
                    .request_elicitation_with_id(url_request(request_scope.clone(), id), cx)
                    .expect("request URL elicitation")
            });
            store.update(cx, |store, cx| {
                store.respond_to_elicitation(&entry, accept(), cx);
            });
            assert_eq!(response.await, accept());
            request_entries.push(entry);
        }
        cx.run_until_parked();

        for id in ["session-completed", "request-completed"] {
            agent
                .send_notification(acp::CompleteElicitationNotification::new(
                    acp::ElicitationId::new(id),
                ))
                .expect("send final completion");
        }
        drain_sdk_background_tasks(cx);
        drop(keep_agent_alive);
        drain_sdk_background_tasks(cx);
        cx.run_until_parked();
        connection.connection.incoming_closed().await;

        let [session_completed, session_incomplete] = session_entries.as_slice() else {
            panic!("expected two session URL elicitations");
        };
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread
                    .elicitation(session_completed)
                    .expect("completed URL")
                    .1
                    .status,
                acp_thread::ElicitationStatus::Completed
            ));
            assert!(matches!(
                thread
                    .elicitation(session_incomplete)
                    .expect("incomplete URL")
                    .1
                    .status,
                acp_thread::ElicitationStatus::Canceled
            ));
        });
        let [request_completed, request_incomplete] = request_entries.as_slice() else {
            panic!("expected two connection URL elicitations");
        };
        store.read_with(cx, |store, _| {
            assert!(matches!(
                store
                    .elicitation(request_completed)
                    .expect("completed URL")
                    .1
                    .status,
                acp_thread::ElicitationStatus::Completed
            ));
            assert!(matches!(
                store
                    .elicitation(request_incomplete)
                    .expect("incomplete URL")
                    .1
                    .status,
                acp_thread::ElicitationStatus::Canceled
            ));
        });
    }

    #[gpui::test]
    async fn transport_error_cancels_retained_elicitations(cx: &mut gpui::TestAppContext) {
        let store = cx.new(|_| ElicitationStore::default());
        let context = ClientContext {
            sessions: Rc::new(RefCell::new(HashMap::default())),
            session_list: Rc::new(RefCell::new(None)),
            request_elicitations: store.clone(),
            active_request_scopes: None,
        };
        let (dispatch_tx, mut dispatch_rx) = mpsc::unbounded::<ForegroundWork>();
        let dispatch_task = cx.spawn(move |mut cx| async move {
            while let Some(work) = dispatch_rx.next().await {
                work.run(&mut cx, &context);
            }
        });
        let (incoming_tx, incoming_rx) = mpsc::unbounded::<std::io::Result<String>>();
        let outgoing = futures::sink::unfold((), async move |(), _line: String| {
            Ok::<_, std::io::Error>(())
        });
        let (connection_tx, connection_rx) = futures::channel::oneshot::channel();
        let io_task = cx.background_spawn(connect_client_future(
            "failing-transport",
            Lines::new(Box::pin(outgoing), incoming_rx),
            dispatch_tx,
            connection_tx,
            futures::future::pending(),
        ));
        let _connection = connection_rx.await.expect("SDK connection handle");
        let (entry, response) = store.update(cx, |store, cx| {
            store
                .request_elicitation_with_id(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v2::RequestId::Number(1)),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Retained question",
                    ),
                    cx,
                )
                .expect("retained elicitation")
        });
        incoming_tx
            .unbounded_send(Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "test transport failure",
            )))
            .expect("inject transport error");
        assert!(io_task.await.is_err(), "transport error must propagate");
        store.read_with(cx, |store, _| {
            assert!(matches!(
                store.elicitation(&entry).expect("retained entry").1.status,
                acp_thread::ElicitationStatus::Canceled
            ));
        });
        assert_eq!(response.await.action, acp_v2::ElicitationAction::Cancel);
        drop(dispatch_task);
    }

    #[gpui::test]
    async fn receipt_disconnect_sdk_eof_settles_retained_submission_owners(
        cx: &mut gpui::TestAppContext,
    ) {
        let (client, project, _, _, _, _, agent_task) = connect_fake_agent(None, cx).await;
        let connection =
            Rc::new(acp_thread::StubAgentConnection::new().with_receipt_submissions(true));
        let thread = cx
            .update(|cx| {
                connection
                    .clone()
                    .new_session(project, PathList::default(), cx)
            })
            .await
            .expect("receipt model owner");
        cx.update(|cx| {
            let session_id = thread.read(cx).session_id().clone();
            client.register_session(session_id, &thread, None, None, None, cx);
        });
        let _pending_sender = connection.defer_next_receipt_response();
        let pending = thread.update(cx, |thread, cx| thread.send(vec!["pending".into()], cx));
        let pending_id = pending.id;
        cx.run_until_parked();
        drop(agent_task);
        client.connection.incoming_closed().await;
        drain_foreground_queue(&client.dispatch_tx)
            .await
            .expect("disconnect barrier");
        assert!(
            pending
                .await
                .expect_err("lost transport")
                .is::<acp_thread::SubmissionDisconnected>()
        );
        thread.read_with(cx, |thread, _| {
            assert!(!thread.has_unsettled_submissions());
            assert!(matches!(
                thread
                    .submission(pending_id)
                    .expect("unknown acceptance")
                    .state,
                acp_thread::SubmissionState::Disconnected { receipt: None }
            ));
        });
    }

    #[test]
    fn disconnect_cleanup_reports_queue_failures() {
        let (sender, receiver) = mpsc::unbounded::<ForegroundWork>();
        drop(receiver);
        let error = futures::executor::block_on(settle_after_disconnect(sender))
            .expect_err("closed dispatch queue must reject cleanup");
        assert_eq!(error.code, ErrorCode::InternalError);

        let (sender, mut receiver) = mpsc::unbounded::<ForegroundWork>();
        let mut cleanup = Box::pin(settle_after_disconnect(sender));
        assert!(cleanup.as_mut().now_or_never().is_none());
        let accepted = futures::executor::block_on(receiver.next()).expect("accepted cleanup");
        drop(accepted);
        let error = futures::executor::block_on(cleanup)
            .expect_err("dropped cleanup acknowledgment must fail");
        assert_eq!(error.code, ErrorCode::InternalError);
    }

    #[gpui::test]
    async fn disconnect_cleanup_runs_once_for_both_completion_paths(cx: &mut gpui::TestAppContext) {
        let context = ClientContext {
            sessions: Rc::new(RefCell::new(HashMap::default())),
            session_list: Rc::new(RefCell::new(None)),
            request_elicitations: cx.new(|_| ElicitationStore::default()),
            active_request_scopes: None,
        };
        let (sender, mut receiver) = mpsc::unbounded::<ForegroundWork>();
        let cleanup = settle_after_disconnect(sender.clone()).boxed().shared();
        let mut close_callback = Box::pin(cleanup.clone());
        let mut connection_finished = Box::pin(cleanup.clone());
        assert!(close_callback.as_mut().now_or_never().is_none());
        assert!(connection_finished.as_mut().now_or_never().is_none());
        let work = receiver.next().await.expect("one cleanup work item");
        assert!(
            receiver.next().now_or_never().is_none(),
            "both completion paths must share one queued cleanup"
        );
        work.run(&mut cx.to_async(), &context);
        close_callback.await.expect("close callback cleanup");
        connection_finished
            .await
            .expect("connection completion cleanup");
        assert!(receiver.next().now_or_never().is_none());
    }

    #[gpui::test]
    async fn new_session_registers_before_immediate_update(cx: &mut gpui::TestAppContext) {
        let (received, request_received) = async_channel::bounded(1);
        let (respond, response_released) = async_channel::bounded(1);
        let (connection, project, _, _, _, _, _keep_agent_alive) =
            connect_fake_agent_with_new_session_behavior(
                None,
                None,
                Some(NewSessionTestBehavior {
                    received,
                    respond: response_released,
                    updates: vec![acp::SessionUpdate::AgentMessageChunk(
                        acp::ContentChunk::new("First update immediately after creation".into()),
                    )],
                }),
                cx,
            )
            .await;
        let creation = cx.update(|cx| {
            connection.clone().new_session(
                project,
                PathList::new(&[std::path::Path::new("/a")]),
                cx,
            )
        });
        request_received.recv().await.expect("new-session request");

        let (acknowledgment, mut barrier_received) = futures::channel::oneshot::channel();
        let barrier: ForegroundWork = Box::new(ForegroundBarrier { acknowledgment });
        assert!(connection.dispatch_tx.unbounded_send(barrier).is_ok());
        respond
            .try_send(Ok(acp::NewSessionResponse::new(acp::SessionId::new(
                "unused",
            ))))
            .expect("release response and first update");
        let executor = cx.executor();
        let dispatcher = executor
            .dispatcher()
            .as_test()
            .expect("deterministic test dispatcher");
        let mut background_drained = false;
        for _ in 0..1000 {
            if !dispatcher.tick(true) {
                background_drained = true;
                break;
            }
        }
        assert!(background_drained, "SDK dispatch must quiesce");
        assert!(connection.sessions.borrow().is_empty());
        assert!(
            barrier_received
                .try_recv()
                .expect("queued barrier")
                .is_none()
        );

        cx.run_until_parked();
        let thread = creation.await.expect("new session");
        assert!(
            thread
                .read_with(cx, |thread, cx| thread.to_markdown(cx))
                .contains("First update immediately after creation"),
            "the first update must not be dropped before session registration"
        );
    }

    #[gpui::test]
    async fn abandoned_new_session_closes_late_response(cx: &mut gpui::TestAppContext) {
        let (received, request_received) = async_channel::bounded(1);
        let (respond, response_released) = async_channel::bounded(1);
        let (connection, project, _, close_count, _, _, _keep_agent_alive) =
            connect_fake_agent_with_new_session_behavior(
                None,
                None,
                Some(NewSessionTestBehavior {
                    received,
                    respond: response_released,
                    updates: Vec::new(),
                }),
                cx,
            )
            .await;
        let creation = cx.update(|cx| {
            connection.clone().new_session(
                project.clone(),
                PathList::new(&[std::path::Path::new("/a")]),
                cx,
            )
        });
        let cancellation = request_received.recv().await.expect("new-session request");
        drop(creation);
        cx.run_until_parked();
        assert!(
            cancellation.is_cancelled(),
            "caller drop must cancel the RPC"
        );
        assert!(connection.sessions.borrow().is_empty());
        assert_eq!(close_count.load(Ordering::SeqCst), 0);

        respond
            .try_send(Ok(acp::NewSessionResponse::new(acp::SessionId::new(
                "unused",
            ))))
            .expect("release late response");
        cx.run_until_parked();
        assert!(connection.sessions.borrow().is_empty());
        assert_eq!(close_count.load(Ordering::SeqCst), 1);
        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    acp_v2::SessionId::new("after-abandonment"),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    cx,
                )
            })
            .await
            .expect("abandonment must not end the connection");
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.session_id().clone()),
            acp_v2::SessionId::new("after-abandonment")
        );
    }

    #[gpui::test]
    async fn abandoned_new_session_releases_registration_barrier(cx: &mut gpui::TestAppContext) {
        let (received, request_received) = async_channel::bounded(1);
        let (respond, response_released) = async_channel::bounded(1);
        let (connection, project, _, close_count, _, _, _keep_agent_alive) =
            connect_fake_agent_with_new_session_behavior(
                None,
                None,
                Some(NewSessionTestBehavior {
                    received,
                    respond: response_released,
                    updates: Vec::new(),
                }),
                cx,
            )
            .await;
        let creation = cx.update(|cx| {
            connection.clone().new_session(
                project,
                PathList::new(&[std::path::Path::new("/a")]),
                cx,
            )
        });
        request_received.recv().await.expect("new-session request");
        respond
            .try_send(Ok(acp::NewSessionResponse::new(acp::SessionId::new(
                "unused",
            ))))
            .expect("release response");
        let executor = cx.executor();
        let dispatcher = executor
            .dispatcher()
            .as_test()
            .expect("deterministic test dispatcher");
        let mut background_drained = false;
        for _ in 0..1000 {
            if !dispatcher.tick(true) {
                background_drained = true;
                break;
            }
        }
        assert!(
            background_drained,
            "ordered response must await foreground setup"
        );
        assert!(connection.sessions.borrow().is_empty());
        drop(creation);
        cx.run_until_parked();
        assert_eq!(close_count.load(Ordering::SeqCst), 1);
        assert!(connection.sessions.borrow().is_empty());
    }

    #[gpui::test]
    async fn new_session_rejection_releases_ordered_dispatch(cx: &mut gpui::TestAppContext) {
        let (received, request_received) = async_channel::bounded(1);
        let (respond, response_released) = async_channel::bounded(1);
        let (connection, project, _, close_count, _, _, _keep_agent_alive) =
            connect_fake_agent_with_new_session_behavior(
                None,
                None,
                Some(NewSessionTestBehavior {
                    received,
                    respond: response_released,
                    updates: Vec::new(),
                }),
                cx,
            )
            .await;
        let creation = cx.update(|cx| {
            connection.clone().new_session(
                project.clone(),
                PathList::new(&[std::path::Path::new("/a")]),
                cx,
            )
        });
        request_received.recv().await.expect("new-session request");
        let rejection = acp::Error::invalid_params();
        respond
            .try_send(Err(rejection.clone()))
            .expect("reject session creation");
        let error = creation.await.expect_err("creation must reject");
        assert_eq!(error.downcast_ref::<acp::Error>(), Some(&rejection));
        cx.run_until_parked();
        assert!(connection.sessions.borrow().is_empty());
        assert_eq!(close_count.load(Ordering::SeqCst), 0);
        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    acp_v2::SessionId::new("after-rejection"),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    cx,
                )
            })
            .await
            .expect("rejection must not end or block the connection");
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.session_id().clone()),
            acp_v2::SessionId::new("after-rejection")
        );
    }

    #[gpui::test]
    async fn new_session_eof_releases_ordered_dispatch(cx: &mut gpui::TestAppContext) {
        let (received, request_received) = async_channel::bounded(1);
        let (_respond, response_released) = async_channel::bounded(1);
        let (connection, project, _, close_count, _, _, keep_agent_alive) =
            connect_fake_agent_with_new_session_behavior(
                None,
                None,
                Some(NewSessionTestBehavior {
                    received,
                    respond: response_released,
                    updates: Vec::new(),
                }),
                cx,
            )
            .await;
        let creation = cx.update(|cx| {
            connection.clone().new_session(
                project,
                PathList::new(&[std::path::Path::new("/a")]),
                cx,
            )
        });
        request_received.recv().await.expect("new-session request");
        drop(keep_agent_alive);
        cx.run_until_parked();
        assert!(creation.await.is_err(), "EOF must fail session creation");
        cx.run_until_parked();
        assert!(connection.connection.is_incoming_closed());
        assert!(connection.sessions.borrow().is_empty());
        assert_eq!(close_count.load(Ordering::SeqCst), 0);
        let (acknowledgment, barrier_received) = futures::channel::oneshot::channel();
        let barrier: ForegroundWork = Box::new(ForegroundBarrier { acknowledgment });
        assert!(connection.dispatch_tx.unbounded_send(barrier).is_ok());
        barrier_received
            .await
            .expect("foreground dispatch after EOF");
    }

    #[gpui::test]
    async fn test_permission_rpc_cancellation_does_not_cancel_successor(
        cx: &mut gpui::TestAppContext,
    ) {
        let (agent_sender, agent_receiver) = futures::channel::oneshot::channel();
        let (connection, project, _, _, _, _, _keep_agent_alive) =
            connect_fake_agent_with_handle(None, Some(agent_sender), cx).await;
        let agent = agent_receiver.await.expect("fake agent handle");
        let session_id = acp_v2::SessionId::new("permission-session");
        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    cx,
                )
            })
            .await
            .expect("load permission session");
        cx.background_executor.allow_parking();

        let timeout = cx
            .background_executor
            .timer(std::time::Duration::from_secs(5));
        let exchange = async {
            let tool_id = acp_v2::ToolCallId::new("reused-tool");
            let permission = |option_id: &str| {
                acp::RequestPermissionRequest::new(
                    acp::SessionId::new(session_id.0.clone()),
                    acp::ToolCallUpdate::new(
                        acp::ToolCallId::new(tool_id.0.clone()),
                        acp::ToolCallUpdateFields::default()
                            .title("Run a command")
                            .status(acp::ToolCallStatus::Pending),
                    ),
                    vec![acp::PermissionOption::new(
                        option_id.to_owned(),
                        "Allow once",
                        acp::PermissionOptionKind::AllowOnce,
                    )],
                )
            };
            let old = agent.send_request(permission("allow-old"));
            cx.run_until_parked();
            let old_id = thread.read_with(cx, |thread, _| {
                let request = thread
                    .permission_request_for_tool(&tool_id)
                    .expect("first permission request");
                assert_eq!(request.legacy_tool_call_id(), Some(&tool_id));
                request.id
            });

            let successor = agent.send_request(permission("allow-successor"));
            let executor = cx.executor();
            let dispatcher = executor
                .dispatcher()
                .as_test()
                .expect("deterministic test dispatcher");
            let drain_sdk = || {
                for _ in 0..1000 {
                    if !dispatcher.tick(true) {
                        return;
                    }
                }
                panic!("SDK dispatch did not drain within bounded ticks");
            };
            drain_sdk();
            let (acknowledgment, mut dispatched) = futures::channel::oneshot::channel();
            let barrier: ForegroundWork = Box::new(ForegroundBarrier { acknowledgment });
            assert!(
                connection.dispatch_tx.unbounded_send(barrier).is_ok(),
                "enqueue permission setup barrier"
            );
            let mut setup_queued = false;
            for _ in 0..1000 {
                if dispatched
                    .try_recv()
                    .expect("live permission setup barrier")
                    .is_some()
                {
                    setup_queued = true;
                    break;
                }
                assert!(executor.tick(), "permission dispatch must make progress");
            }
            assert!(
                setup_queued,
                "permission setup queued within bounded dispatch"
            );
            thread.read_with(cx, |thread, _| {
                assert_eq!(
                    thread
                        .permission_request_for_tool(&tool_id)
                        .expect("old request still active before successor setup")
                        .id,
                    old_id
                );
            });

            // Foreground FIFO puts successor setup before old cancellation cleanup,
            // and cleanup before the old waiter consumes its superseded outcome.
            old.cancel().expect("send old RPC cancellation");
            drain_sdk();

            let error = old
                .block_task()
                .await
                .expect_err("old RPC must exercise RequestCancelled cleanup");
            assert_eq!(error.code, ErrorCode::RequestCancelled);
            let successor_id = thread.read_with(cx, |thread, _| {
                assert!(thread.permission_request(old_id).is_none());
                let successor_id = thread
                    .permission_request_for_tool(&tool_id)
                    .expect("successor survives old RPC cancellation")
                    .id;
                assert_ne!(successor_id, old_id);
                assert_eq!(thread.pending_permission_requests().count(), 1);
                let (_, call) = thread.tool_call(&tool_id).expect("tool presentation");
                assert_eq!(call.authorization_id(), Some(successor_id));
                assert_eq!(
                    call.status(),
                    acp_thread::ToolCallStatus::WaitingForConfirmation
                );
                successor_id
            });
            thread.update(cx, |thread, cx| {
                thread.authorize_permission_request(
                    successor_id,
                    acp_thread::SelectedPermissionOutcome::new(
                        acp_v2::PermissionOptionId::new("allow-successor"),
                        acp_v2::PermissionOptionKind::AllowOnce,
                    ),
                    cx,
                );
            });
            let response = successor
                .block_task()
                .await
                .expect("successor RPC receives its exact offered choice");
            assert_eq!(
                response.outcome,
                acp::RequestPermissionOutcome::Selected(acp::SelectedPermissionOutcome::new(
                    "allow-successor"
                ))
            );
            thread.read_with(cx, |thread, _| {
                assert!(thread.permission_request(successor_id).is_none());
                assert_eq!(thread.pending_permission_requests().count(), 0);
                let (_, call) = thread.tool_call(&tool_id).expect("tool presentation");
                assert_eq!(call.authorization_id(), None);
                assert_eq!(call.status(), acp_thread::ToolCallStatus::InProgress);
            });
            thread
                .update(cx, |thread, cx| thread.send_raw("channel still usable", cx))
                .await
                .expect("next SDK prompt succeeds after old RPC cancellation");
        };
        futures::pin_mut!(exchange, timeout);
        if let futures::future::Either::Right(_) = futures::future::select(exchange, timeout).await
        {
            panic!("timed out exercising permission RPC supersession and cancellation");
        }
    }

    #[gpui::test]
    async fn test_loaded_sessions_keep_state_until_last_handle_drops(
        cx: &mut gpui::TestAppContext,
    ) {
        let (
            connection,
            project,
            load_count,
            close_count,
            _load_session_updates,
            _load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(None, cx).await;

        let session_id = acp_v2::SessionId::new("session-1");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        // Load the same session twice concurrently — the second call should join
        // the pending task rather than issuing a second ACP load_session RPC.
        let first_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });
        let second_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });

        let first_thread = first_load.await.expect("first load failed");
        let second_thread = second_load.await.expect("second load failed");
        cx.run_until_parked();

        assert_eq!(
            first_thread.entity_id(),
            second_thread.entity_id(),
            "concurrent loads for the same session should share one AcpThread"
        );
        assert_eq!(
            load_count.load(Ordering::SeqCst),
            1,
            "underlying ACP load_session should be called exactly once for concurrent loads"
        );

        drop(first_thread);
        release_dropped_entities(cx);

        assert_eq!(
            close_count.load(Ordering::SeqCst),
            0,
            "ACP close_session should not be sent while a thread handle is alive"
        );
        assert!(
            connection.sessions.borrow().contains_key(&session_id),
            "session should still be tracked while a thread handle is alive"
        );

        drop(second_thread);
        release_dropped_entities(cx);

        assert_eq!(
            close_count.load(Ordering::SeqCst),
            1,
            "ACP close_session should be sent exactly once when the last handle drops"
        );
        assert!(
            !connection.sessions.borrow().contains_key(&session_id),
            "session should be removed after the last handle drops"
        );
    }

    #[gpui::test]
    async fn test_session_notices_are_scoped_and_not_restored_on_reload(
        cx: &mut gpui::TestAppContext,
    ) {
        let (
            connection,
            project,
            _load_count,
            _close_count,
            load_session_updates,
            _load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(None, cx).await;

        let notice_update: acp::SessionUpdate = serde_json::from_value(serde_json::json!({
            "sessionUpdate": "notice",
            "severity": "_custom",
            "title": "A current advisory",
            "description": null,
            "_meta": null,
        }))
        .expect("unknown notice severity should deserialize");
        // An agent may emit a fresh notice about a current condition while reconnecting.
        *load_session_updates
            .lock()
            .expect("load_session_updates mutex poisoned") = vec![notice_update];

        let session_id = acp_v2::SessionId::new("session-with-notice");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);
        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project.clone(),
                    work_dirs.clone(),
                    None,
                    cx,
                )
            })
            .await
            .expect("session should load");
        cx.run_until_parked();

        thread.read_with(cx, |thread, cx| {
            let [(notice_id, notice)] = thread.notices() else {
                panic!("the live notice should be received before a view is created");
            };
            assert_eq!(*notice_id, 0);
            assert_eq!(notice.title, "A current advisory");
            assert_eq!(
                notice.severity,
                acp_v2::NoticeSeverity::Other("_custom".into())
            );
            assert_eq!(notice.description, None);
            assert_eq!(notice.meta, None);
            assert!(thread.entries().is_empty());
            assert!(thread.to_markdown(cx).is_empty());
            assert!(!thread.had_error());
        });

        let other_thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    acp_v2::SessionId::new("other-session"),
                    project.clone(),
                    work_dirs.clone(),
                    None,
                    cx,
                )
            })
            .await
            .expect("other session should load");
        other_thread.read_with(cx, |thread, _| assert!(thread.notices().is_empty()));

        drop(thread);
        release_dropped_entities(cx);
        let reloaded_thread = cx
            .update(|cx| {
                connection
                    .clone()
                    .load_session(session_id, project, work_dirs, None, cx)
            })
            .await
            .expect("session should reload");
        cx.run_until_parked();
        reloaded_thread.read_with(cx, |thread, _| assert!(thread.notices().is_empty()));
    }

    // Regression test: per the ACP spec, an agent replays the entire conversation
    // history as `session/update` notifications *before* responding to the
    // `session/load` request. These notifications must be applied to the
    // reconstructed thread, not dropped because the session hasn't been
    // registered yet.
    #[gpui::test]
    async fn test_load_session_replays_notifications_sent_before_response(
        cx: &mut gpui::TestAppContext,
    ) {
        let (
            connection,
            project,
            _load_count,
            _close_count,
            load_session_updates,
            _load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(None, cx).await;

        // Queue up some history updates that the fake agent will stream to
        // the client during the `load_session` call, before responding.
        *load_session_updates
            .lock()
            .expect("load_session_updates mutex poisoned") = vec![
            acp::SessionUpdate::UserMessageChunk(acp::ContentChunk::new(acp::ContentBlock::Text(
                acp::TextContent::new(String::from("hello agent")),
            ))),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(acp::ContentBlock::Text(
                acp::TextContent::new(String::from("hi user")),
            ))),
            acp::SessionUpdate::CompactionUpdate(acp::CompactionUpdate::new(
                "replayed-compaction",
                acp::CompactionStatus::Completed,
            )),
        ];

        let session_id = acp_v2::SessionId::new("session-replay");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        let thread = cx
            .update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project.clone(),
                    work_dirs,
                    None,
                    cx,
                )
            })
            .await
            .expect("load_session failed");
        cx.run_until_parked();

        let entries = thread.read_with(cx, |thread, _| {
            thread
                .entries()
                .iter()
                .map(|entry| match entry {
                    acp_thread::AgentThreadEntry::UserMessage(_) => "user",
                    acp_thread::AgentThreadEntry::AssistantMessage(_) => "assistant",
                    acp_thread::AgentThreadEntry::ToolCall(_) => "tool_call",
                    acp_thread::AgentThreadEntry::Elicitation(_) => "elicitation",
                    acp_thread::AgentThreadEntry::ContextCompaction(_) => "compaction",
                })
                .collect::<Vec<_>>()
        });

        assert_eq!(
            entries,
            vec!["user", "assistant", "compaction"],
            "replayed notifications should be applied to the thread"
        );
    }

    #[gpui::test]
    async fn test_load_initializes_config_before_draining_queued_updates(
        cx: &mut gpui::TestAppContext,
    ) {
        let (connection, project, _, _, _, _, _keep_agent_alive) =
            connect_fake_agent(None, cx).await;
        let session_id = acp_v2::SessionId::new("session-config-update");
        let updated_options = vec![acp::SessionConfigOption::boolean(
            "web_search",
            "Web Search",
            true,
        )];
        let thread = cx
            .update(|cx| {
                connection.clone().open_or_create_session(
                    session_id.clone(),
                    project,
                    PathList::new(&[std::path::Path::new("/a")]),
                    None,
                    {
                        let dispatch_tx = connection.dispatch_tx.clone();
                        let updated_options = updated_options.clone();
                        move |_, session_id, _| {
                            async move {
                                // Later notifications can already be queued when the response is ready.
                                enqueue_notification(
                                    &dispatch_tx,
                                    acp::SessionNotification::new(
                                        acp::SessionId::new(session_id.0),
                                        acp::SessionUpdate::ConfigOptionUpdate(
                                            acp::ConfigOptionUpdate::new(updated_options),
                                        ),
                                    ),
                                    handle_session_notification,
                                );
                                Ok(SessionConfigResponse {
                                    modes: None,
                                    config_options: Some(vec![acp::SessionConfigOption::boolean(
                                        "web_search",
                                        "Web Search",
                                        false,
                                    )]),
                                })
                            }
                            .boxed_local()
                        }
                    },
                    cx,
                )
            })
            .await
            .expect("session should load");
        cx.run_until_parked();

        let sessions = connection.sessions.borrow();
        let session = sessions.get(&session_id).expect("loaded session");
        assert_eq!(session.thread.entity_id(), thread.entity_id());
        let options = session.config_options.as_ref().expect("loaded config");
        assert_eq!(
            *options.config_options.borrow(),
            acp_thread::config_options::from_v1(updated_options).expect("shared config options")
        );
    }

    #[gpui::test]
    async fn test_dropping_loader_during_in_flight_load_closes_session(
        cx: &mut gpui::TestAppContext,
    ) {
        let (
            connection,
            project,
            load_count,
            close_count,
            _load_session_updates,
            load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(None, cx).await;

        // Install a gate so the fake agent's `load_session` handler parks
        // before sending its response. We'll close the session while the
        // load is parked.
        let (gate_tx, gate_rx) = async_channel::bounded::<()>(1);
        *load_session_gate
            .lock()
            .expect("load_session_gate mutex poisoned") = Some(gate_rx);

        let session_id = acp_v2::SessionId::new("session-close-during-load");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        let load_task = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs,
                None,
                cx,
            )
        });

        // Let the load RPC reach the agent and park on the gate.
        cx.run_until_parked();
        assert_eq!(
            load_count.load(Ordering::SeqCst),
            1,
            "load_session RPC should have been dispatched"
        );
        assert!(
            connection
                .pending_sessions
                .borrow()
                .contains_key(&session_id),
            "pending_sessions entry should exist while load is in flight"
        );
        assert!(
            connection.sessions.borrow().contains_key(&session_id),
            "sessions entry should be pre-registered to receive replay notifications"
        );

        drop(load_task);
        cx.run_until_parked();
        assert_eq!(
            close_count.load(Ordering::SeqCst),
            0,
            "ACP close_session must not be sent while the load RPC is in flight"
        );

        // Release the gate so the load RPC can finally respond.
        gate_tx.send(()).await.expect("gate send failed");
        drop(gate_tx);
        cx.run_until_parked();
        release_dropped_entities(cx);

        assert_eq!(
            close_count.load(Ordering::SeqCst),
            1,
            "ACP close_session should be sent exactly once"
        );
        assert!(
            !connection.sessions.borrow().contains_key(&session_id),
            "sessions entry should be removed once the loaded thread has no holders"
        );
        assert!(
            !connection
                .pending_sessions
                .borrow()
                .contains_key(&session_id),
            "pending_sessions entry should be removed once the load resolves"
        );
    }

    #[gpui::test]
    async fn test_abandoned_load_releases_connection_before_rpc_response(
        cx: &mut gpui::TestAppContext,
    ) {
        for poll_load in [false, true] {
            let (
                connection,
                project,
                load_count,
                close_count,
                _load_session_updates,
                load_session_gate,
                keep_agent_alive,
            ) = connect_fake_agent(None, cx).await;

            let (gate_tx, gate_rx) = async_channel::bounded::<()>(1);
            *load_session_gate
                .lock()
                .expect("load_session_gate mutex poisoned") = Some(gate_rx);

            let session_id = acp_v2::SessionId::new("session-abandoned-during-load");
            let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);
            let load_task = cx.update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project.clone(),
                    work_dirs,
                    None,
                    cx,
                )
            });

            if poll_load {
                cx.run_until_parked();
                assert_eq!(load_count.load(Ordering::SeqCst), 1);
            }
            let weak_thread = connection
                .sessions
                .borrow()
                .get(&session_id)
                .expect("in-flight load should register its thread")
                .thread
                .clone();
            let weak_connection = Rc::downgrade(&connection);

            drop(load_task);
            drop(connection);
            release_dropped_entities(cx);
            let connection_retained_before_response = weak_connection.upgrade().is_some();
            let thread_retained_before_response = weak_thread.upgrade().is_some();
            let close_count_before_response = close_count.load(Ordering::SeqCst);

            // Releasing the connection can also tear down the peer's gated handler.
            if let Err(error) = gate_tx.try_send(()) {
                assert!(error.is_closed(), "unexpected gate send failure: {error}");
            }
            drop(gate_tx);
            release_dropped_entities(cx);
            drop(keep_agent_alive);
            cx.run_until_parked();

            assert_eq!(
                close_count_before_response, 0,
                "closing the ACP session must wait for the in-flight load response"
            );
            assert!(
                !connection_retained_before_response && !thread_retained_before_response,
                "abandoned load retained objects while the fake agent held its response: connection={connection_retained_before_response}, thread={thread_retained_before_response}"
            );
        }
    }

    #[gpui::test]
    async fn test_reopening_session_waits_for_close_acknowledgement(cx: &mut gpui::TestAppContext) {
        for abandon_load in [false, true] {
            let (close_gate_tx, close_gate_rx) = async_channel::bounded::<()>(1);
            let (
                connection,
                project,
                load_count,
                close_count,
                load_session_updates,
                load_session_gate,
                _keep_agent_alive,
            ) = connect_fake_agent(Some(close_gate_rx), cx).await;
            let (load_gate_tx, load_gate_rx) = async_channel::bounded::<()>(1);
            *load_session_gate
                .lock()
                .expect("load_session_gate mutex poisoned") = Some(load_gate_rx);
            *load_session_updates
                .lock()
                .expect("load_session_updates mutex poisoned") =
                vec![acp::SessionUpdate::AgentMessageChunk(
                    acp::ContentChunk::new(acp::ContentBlock::Text(acp::TextContent::new(
                        "old".to_string(),
                    ))),
                )];

            let session_id = acp_v2::SessionId::new("session-reopen-abandoned-load");
            let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);
            let first_load = cx.update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project.clone(),
                    work_dirs.clone(),
                    None,
                    cx,
                )
            });
            cx.run_until_parked();
            assert_eq!(load_count.load(Ordering::SeqCst), 1);
            let old_thread = connection
                .sessions
                .borrow()
                .get(&session_id)
                .expect("first load should register its replay recipient")
                .thread
                .clone();
            let old_thread_id = old_thread.entity_id();

            if abandon_load {
                drop(first_load);
            } else {
                load_gate_tx.send(()).await.expect("load gate send failed");
                let loaded_thread = first_load.await.expect("initial load failed");
                drop(loaded_thread);
            }
            release_dropped_entities(cx);
            assert!(
                old_thread.upgrade().is_none(),
                "old owner should release its thread"
            );
            *load_session_updates
                .lock()
                .expect("load_session_updates mutex poisoned") =
                vec![acp::SessionUpdate::AgentMessageChunk(
                    acp::ContentChunk::new(acp::ContentBlock::Text(acp::TextContent::new(
                        "new".to_string(),
                    ))),
                )];
            let replacement_load = cx.update(|cx| {
                connection.clone().load_session(
                    session_id.clone(),
                    project.clone(),
                    work_dirs.clone(),
                    None,
                    cx,
                )
            });
            cx.run_until_parked();
            assert_eq!(load_count.load(Ordering::SeqCst), 1);
            assert_eq!(
                close_count.load(Ordering::SeqCst),
                usize::from(!abandon_load)
            );

            if abandon_load {
                load_gate_tx.send(()).await.expect("load gate send failed");
            }
            drop(load_gate_tx);
            release_dropped_entities(cx);
            assert_eq!(
                close_count.load(Ordering::SeqCst),
                1,
                "old session should close before its replacement starts"
            );
            assert_eq!(
                load_count.load(Ordering::SeqCst),
                1,
                "replacement must wait for the old close acknowledgement"
            );
            assert!(
                connection
                    .sessions
                    .borrow()
                    .get(&session_id)
                    .is_none_or(|session| session.thread.entity_id() == old_thread_id),
                "replacement thread must not be registered before close acknowledgement"
            );

            close_gate_tx
                .send(())
                .await
                .expect("close gate send failed");
            drop(close_gate_tx);
            let timeout = cx
                .background_executor
                .timer(std::time::Duration::from_secs(5))
                .fuse();
            let replacement_load = replacement_load.fuse();
            futures::pin_mut!(replacement_load, timeout);
            let replacement = futures::select! {
                result = replacement_load => result.expect("replacement load failed"),
                _ = timeout => panic!("timed out waiting for replacement load"),
            };
            cx.run_until_parked();
            assert_ne!(replacement.entity_id(), old_thread_id);
            assert_eq!(load_count.load(Ordering::SeqCst), 2);
            assert_eq!(close_count.load(Ordering::SeqCst), 1);
            assert_eq!(
                connection
                    .sessions
                    .borrow()
                    .get(&session_id)
                    .map(|session| session.thread.entity_id()),
                Some(replacement.entity_id()),
                "old cleanup must not unregister the replacement"
            );
            let transcript = replacement.read_with(cx, |thread, cx| thread.to_markdown(cx));
            assert_eq!(transcript, "## Assistant\n\nnew\n\n");

            drop(replacement);
            release_dropped_entities(cx);
            assert_eq!(close_count.load(Ordering::SeqCst), 2);
            assert!(!connection.sessions.borrow().contains_key(&session_id));
            assert!(
                !connection
                    .pending_sessions
                    .borrow()
                    .contains_key(&session_id)
            );
        }
    }

    #[gpui::test]
    async fn test_reopening_after_failed_load_discards_old_replay(cx: &mut gpui::TestAppContext) {
        let (connection, project, load_count, close_count, _, _, _keep_agent_alive) =
            connect_fake_agent(None, cx).await;
        let session_id = acp_v2::SessionId::new("session-failed-load");
        let work_dirs = PathList::new(&[std::path::Path::new("/a")]);
        let (response_tx, response_rx) = futures::channel::oneshot::channel::<()>();
        let first_load = cx.update(|cx| {
            connection.clone().open_or_create_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                {
                    let dispatch_tx = connection.dispatch_tx.clone();
                    move |_, session_id, _| {
                        async move {
                            response_rx.await?;
                            enqueue_notification(
                                &dispatch_tx,
                                acp::SessionNotification::new(
                                    acp::SessionId::new(session_id.0),
                                    acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                                        "old replay".into(),
                                    )),
                                ),
                                handle_session_notification,
                            );
                            Err(anyhow!("load failed"))
                        }
                        .boxed_local()
                    }
                },
                cx,
            )
        });
        cx.run_until_parked();
        let old_thread = connection
            .sessions
            .borrow()
            .get(&session_id)
            .expect("loading session")
            .thread
            .clone();
        drop(first_load);
        release_dropped_entities(cx);
        assert!(old_thread.upgrade().is_none());

        let replacement_load = cx.update(|cx| {
            connection
                .clone()
                .load_session(session_id.clone(), project, work_dirs, None, cx)
        });
        cx.run_until_parked();
        assert_eq!(load_count.load(Ordering::SeqCst), 0);
        response_tx
            .send(())
            .expect("failed load should still be waiting");

        let replacement = replacement_load.await.expect("replacement should load");
        cx.run_until_parked();
        assert_ne!(replacement.entity_id(), old_thread.entity_id());
        assert!(replacement.read_with(cx, |thread, _| thread.entries().is_empty()));
        assert_eq!(load_count.load(Ordering::SeqCst), 1);
        assert_eq!(close_count.load(Ordering::SeqCst), 0);
        assert!(
            !connection
                .pending_sessions
                .borrow()
                .contains_key(&session_id)
        );
        assert_eq!(
            connection
                .sessions
                .borrow()
                .get(&session_id)
                .expect("replacement session")
                .thread
                .entity_id(),
            replacement.entity_id()
        );
    }

    #[gpui::test]
    async fn test_dropping_one_loader_during_load_preserves_other_concurrent_loader(
        cx: &mut gpui::TestAppContext,
    ) {
        let (
            connection,
            project,
            load_count,
            close_count,
            _load_session_updates,
            load_session_gate,
            _keep_agent_alive,
        ) = connect_fake_agent(None, cx).await;

        let (gate_tx, gate_rx) = async_channel::bounded::<()>(1);
        *load_session_gate
            .lock()
            .expect("load_session_gate mutex poisoned") = Some(gate_rx);

        let session_id = acp_v2::SessionId::new("session-concurrent-close");
        let work_dirs = util::path_list::PathList::new(&[std::path::Path::new("/a")]);

        // Kick off two concurrent loads; the second must join the first's pending
        // task rather than issuing a second RPC.
        let first_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });
        let second_load = cx.update(|cx| {
            connection.clone().load_session(
                session_id.clone(),
                project.clone(),
                work_dirs.clone(),
                None,
                cx,
            )
        });

        cx.run_until_parked();
        assert_eq!(
            load_count.load(Ordering::SeqCst),
            1,
            "load_session RPC should only be dispatched once for concurrent loads"
        );

        drop(first_load);
        cx.run_until_parked();
        assert_eq!(
            close_count.load(Ordering::SeqCst),
            0,
            "close_session RPC must not be dispatched while another loader remains"
        );

        // Release the gate so the load RPC can finally respond.
        gate_tx.send(()).await.expect("gate send failed");
        drop(gate_tx);

        let second_thread = second_load.await.expect("second load should still succeed");
        cx.run_until_parked();

        assert_eq!(
            close_count.load(Ordering::SeqCst),
            0,
            "close_session RPC must not be dispatched while a thread handle is alive"
        );
        assert!(
            connection.sessions.borrow().contains_key(&session_id),
            "session must remain tracked while a thread handle is alive"
        );
        assert!(
            !connection
                .pending_sessions
                .borrow()
                .contains_key(&session_id),
            "pending_sessions entry should be cleared once the load resolves"
        );

        drop(second_thread);
        release_dropped_entities(cx);
        assert_eq!(
            close_count.load(Ordering::SeqCst),
            1,
            "close_session RPC should fire exactly once when the last handle is released"
        );
        assert!(
            !connection.sessions.borrow().contains_key(&session_id),
            "session should be removed after the last handle is released"
        );
    }

    fn release_dropped_entities(cx: &mut gpui::TestAppContext) {
        // Cancelled task futures may own entities until the executor drops them.
        cx.run_until_parked();
        cx.update(|_| ());
        cx.run_until_parked();
    }
}

fn mcp_servers_for_project(project: &Entity<Project>, cx: &App) -> Vec<acp::McpServer> {
    mcp_configurations_for_project(project, cx)
        .into_iter()
        .map(|(name, configuration)| match &*configuration {
            project::context_server_store::ContextServerConfiguration::Custom {
                command, ..
            }
            | project::context_server_store::ContextServerConfiguration::Extension {
                command,
                ..
            } => acp::McpServer::Stdio(
                acp::McpServerStdio::new(name, &command.path)
                    .args(command.args.clone())
                    .env(if let Some(env) = command.env.as_ref() {
                        env.iter()
                            .map(|(name, value)| acp::EnvVariable::new(name, value))
                            .collect()
                    } else {
                        vec![]
                    }),
            ),
            project::context_server_store::ContextServerConfiguration::Http {
                url,
                headers,
                timeout: _,
                oauth: _,
            } => acp::McpServer::Http(
                acp::McpServerHttp::new(name, url.to_string()).headers(
                    headers
                        .iter()
                        .map(|(name, value)| acp::HttpHeader::new(name, value))
                        .collect(),
                ),
            ),
        })
        .collect()
}

fn config_state(
    modes: Option<acp::SessionModeState>,
    config_options: Option<Vec<acp::SessionConfigOption>>,
) -> Result<(
    Option<Rc<RefCell<acp::SessionModeState>>>,
    Option<ConfigOptions>,
)> {
    if let Some(opts) = config_options {
        let opts = acp_thread::config_options::from_v1(opts)?;
        let opts = ConfigOptions::new(Rc::new(RefCell::new(opts)));
        return Ok((None, Some(opts)));
    }

    let modes = modes.map(|modes| Rc::new(RefCell::new(modes)));
    Ok((modes, None))
}

struct AcpSessionModes {
    session_id: acp_v2::SessionId,
    connection: ConnectionTo<Agent>,
    state: Rc<RefCell<acp::SessionModeState>>,
}

fn mode_from_v1(mode: &acp::SessionMode) -> acp_v2::SessionConfigSelectOption {
    acp_v2::SessionConfigSelectOption::new(
        acp_v2::SessionConfigValueId::new(mode.id.0.clone()),
        mode.name.clone(),
    )
    .description(mode.description.clone())
    .meta(mode.meta.clone())
}

impl acp_thread::AgentSessionModes for AcpSessionModes {
    fn current_mode(&self) -> acp_v2::SessionConfigValueId {
        acp_v2::SessionConfigValueId::new(self.state.borrow().current_mode_id.0.clone())
    }

    fn all_modes(&self) -> Vec<acp_v2::SessionConfigSelectOption> {
        self.state
            .borrow()
            .available_modes
            .iter()
            .map(mode_from_v1)
            .collect()
    }

    fn set_mode(&self, mode_id: acp_v2::SessionConfigValueId, cx: &mut App) -> Task<Result<()>> {
        let mode_id = acp::SessionModeId::new(mode_id.0);
        let connection = self.connection.clone();
        let session_id = acp::SessionId::new(self.session_id.0.clone());
        let old_mode_id;
        {
            let mut state = self.state.borrow_mut();
            old_mode_id = state.current_mode_id.clone();
            state.current_mode_id = mode_id.clone();
        };
        let state = self.state.clone();
        cx.foreground_executor().spawn(async move {
            let result = connection
                .send_request(acp::SetSessionModeRequest::new(session_id, mode_id))
                .block_task()
                .await;

            if result.is_err() {
                state.borrow_mut().current_mode_id = old_mode_id;
            }

            result?;

            Ok(())
        })
    }
}

struct AcpSessionConfigOptions {
    session_id: acp_v2::SessionId,
    connection: ConnectionTo<Agent>,
    config_options: ConfigOptions,
}

impl acp_thread::AgentSessionConfigOptions for AcpSessionConfigOptions {
    fn config_options(&self) -> Vec<acp_v2::SessionConfigOption> {
        self.config_options.config_options.borrow().clone()
    }

    fn set_config_option(
        &self,
        config_id: acp_v2::SessionConfigId,
        value: acp_v2::SessionConfigOptionValue,
        cx: &mut App,
    ) -> Task<Result<Vec<acp_v2::SessionConfigOption>>> {
        let connection = self.connection.clone();
        let session_id = self.session_id.clone();
        let config_options = self.config_options.clone();

        cx.foreground_executor().spawn(async move {
            config_options
                .set_config_option(&connection, session_id, config_id, value)
                .await
        })
    }

    fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
        Some(self.config_options.rx.clone())
    }
}

// ---------------------------------------------------------------------------
// Handler functions dispatched from background handler closures to the
// foreground thread via the ForegroundWork channel.
// ---------------------------------------------------------------------------

fn permission_option_from_v1(
    option: acp::PermissionOption,
) -> Result<acp_v2::PermissionOption, acp::Error> {
    let kind = match option.kind {
        acp::PermissionOptionKind::AllowOnce => acp_v2::PermissionOptionKind::AllowOnce,
        acp::PermissionOptionKind::AllowAlways => acp_v2::PermissionOptionKind::AllowAlways,
        acp::PermissionOptionKind::RejectOnce => acp_v2::PermissionOptionKind::RejectOnce,
        acp::PermissionOptionKind::RejectAlways => acp_v2::PermissionOptionKind::RejectAlways,
        _ => {
            return Err(acp::Error::invalid_params().data("unsupported permission option kind"));
        }
    };
    Ok(acp_v2::PermissionOption::new(
        acp_v2::PermissionOptionId::new(option.option_id.0),
        option.name,
        kind,
    )
    .meta(option.meta))
}

fn permission_outcome_to_v1(
    outcome: acp_thread::RequestPermissionOutcome,
) -> acp::RequestPermissionOutcome {
    match outcome {
        acp_thread::RequestPermissionOutcome::Cancelled
        | acp_thread::RequestPermissionOutcome::InterruptedByFollowUp => {
            acp::RequestPermissionOutcome::Cancelled
        }
        acp_thread::RequestPermissionOutcome::Selected(outcome) => {
            acp::RequestPermissionOutcome::Selected(acp::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new(outcome.option_id.0),
            ))
        }
    }
}

fn handle_request_permission(
    args: acp::RequestPermissionRequest,
    responder: Responder<acp::RequestPermissionResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    let cancellation = responder.cancellation();
    cx.spawn(async move |cx| {
        let options = match args
            .options
            .into_iter()
            .map(permission_option_from_v1)
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(options) => options,
            Err(error) => return respond_err(responder, error),
        };
        let (request_id, task) = match thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization_with_id(
                    args.tool_call,
                    acp_thread::PermissionOptions::Flat(options),
                    acp_thread::AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .flatten_acp()
        {
            Ok(request) => request,
            Err(error) => return respond_err(responder, error),
        };
        let result = cancellation
            .run_until_cancelled(async { Ok(task.await) })
            .await;

        match result {
            Ok(outcome) => {
                responder
                    .respond(acp::RequestPermissionResponse::new(
                        permission_outcome_to_v1(outcome),
                    ))
                    .log_err();
            }
            Err(e) => {
                if e.code == ErrorCode::RequestCancelled {
                    thread
                        .update(cx, |thread, cx| {
                            thread.cancel_permission_request(request_id, cx)
                        })
                        .log_err();
                }
                respond_err(responder, e)
            }
        }
    })
    .detach();
}

fn handle_create_elicitation(
    args: acp::CreateElicitationRequest,
    responder: Responder<acp::CreateElicitationResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let args = match acp_thread::elicitation::request_from_v1(args) {
        Ok(args) => args,
        Err(error) => return respond_err(responder, acp::Error::from(error)),
    };
    let responder = responder.wrap_params(|_, response| {
        response.and_then(|response| {
            acp_thread::elicitation::response_to_v1(response).map_err(acp::Error::from)
        })
    });
    handle_v2_create_elicitation(args, responder, cx, ctx);
}

fn handle_complete_elicitation(
    args: acp::CompleteElicitationNotification,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    handle_v2_complete_elicitation(
        acp_v2::CompleteElicitationNotification::new(acp_v2::ElicitationId::new(
            args.elicitation_id.0,
        ))
        .meta(args.meta),
        cx,
        ctx,
    );
}

fn handle_write_text_file(
    args: acp::WriteTextFileRequest,
    responder: Responder<acp::WriteTextFileResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let result: Result<_, acp::Error> = async {
            thread
                .update(cx, |thread, cx| {
                    thread.write_text_file(args.path, args.content, cx)
                })
                .map_err(acp::Error::from)?
                .await?;
            Ok(())
        }
        .await;

        match result {
            Ok(()) => {
                responder
                    .respond(acp::WriteTextFileResponse::default())
                    .log_err();
            }
            Err(e) => respond_err(responder, e),
        }
    })
    .detach();
}

fn handle_read_text_file(
    args: acp::ReadTextFileRequest,
    responder: Responder<acp::ReadTextFileResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let cancellation = responder.cancellation();
        let result = cancellation
            .run_until_cancelled(async {
                thread
                    .update(cx, |thread, cx| {
                        thread.read_text_file(args.path, args.line, args.limit, false, cx)
                    })
                    .map_err(acp::Error::from)?
                    .await
            })
            .await;

        respond_result(responder, result.map(acp::ReadTextFileResponse::new));
    })
    .detach();
}

fn handle_session_notification(
    notification: acp::SessionNotification,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let session_id = acp_v2::SessionId::new(notification.session_id.0);
    // Extract everything we need from the session while briefly borrowing.
    let (thread, session_modes, config_opts_data) = {
        let sessions = ctx.sessions.borrow();
        let Some(session) = sessions.get(&session_id) else {
            log::warn!(
                "Received session notification for unknown session: {:?}",
                session_id
            );
            return;
        };
        (
            session.thread.clone(),
            session.session_modes.clone(),
            session
                .config_options
                .as_ref()
                .map(|opts| (opts.config_options.clone(), opts.tx.clone())),
        )
    };
    // Borrow is dropped here.

    // Apply mode/config/session_list updates without holding the borrow.
    if let acp::SessionUpdate::CurrentModeUpdate(acp::CurrentModeUpdate {
        current_mode_id, ..
    }) = &notification.update
    {
        if let Some(session_modes) = &session_modes {
            session_modes.borrow_mut().current_mode_id = current_mode_id.clone();
        }
    }

    if let acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate {
        config_options, ..
    }) = &notification.update
    {
        if let Some((config_opts_cell, tx_cell)) = &config_opts_data {
            let Some(config_options) =
                acp_thread::config_options::from_v1(config_options.clone()).log_err()
            else {
                return;
            };
            *config_opts_cell.borrow_mut() = config_options;
            tx_cell.borrow_mut().send(()).ok();
        }
    }

    if let acp::SessionUpdate::SessionInfoUpdate(info_update) = &notification.update
        && let Some(session_list) = ctx.session_list.borrow().as_ref()
    {
        session_list.send_info_update(session_id.clone(), info_update.clone());
    }

    // Abandoned loads can keep replaying until their wire operation settles.
    let Some(_thread) = thread.upgrade() else {
        return;
    };

    // Pre-handle: if a ToolCall carries terminal_info, create/register a display-only terminal.
    if let acp::SessionUpdate::ToolCall(tc) = &notification.update {
        if let Some(meta) = &tc.meta {
            if let Some(terminal_info) = meta.get("terminal_info") {
                if let Some(id_str) = terminal_info.get("terminal_id").and_then(|v| v.as_str()) {
                    let terminal_id = acp::TerminalId::new(id_str);
                    let cwd = terminal_info
                        .get("cwd")
                        .and_then(|v| v.as_str().map(PathBuf::from));

                    thread
                        .update(cx, |thread, cx| {
                            let builder = TerminalBuilder::new_display_only(
                                CursorShape::default(),
                                AlternateScroll::On,
                                None,
                                0,
                                cx.background_executor(),
                                thread.project().read(cx).path_style(cx),
                            );
                            let lower = cx.new(|cx| builder.subscribe(cx));
                            thread.on_terminal_provider_event(
                                TerminalProviderEvent::Created {
                                    terminal_id,
                                    label: tc.title.clone(),
                                    cwd,
                                    output_byte_limit: None,
                                    terminal: lower,
                                },
                                cx,
                            );
                        })
                        .log_err();
                }
            }
        }
    }

    // Forward the update to the acp_thread as usual.
    if let Err(err) = thread
        .update(cx, |thread, cx| {
            thread.handle_session_update(notification.update.clone(), cx)
        })
        .flatten_acp()
    {
        log::error!(
            "Failed to handle session update for {:?}: {err:?}",
            session_id
        );
    }

    // Post-handle: stream terminal output/exit if present on ToolCallUpdate meta.
    if let acp::SessionUpdate::ToolCallUpdate(tcu) = &notification.update {
        if let Some(meta) = &tcu.meta {
            if let Some(term_out) = meta.get("terminal_output") {
                if let Some(id_str) = term_out.get("terminal_id").and_then(|v| v.as_str()) {
                    let terminal_id = acp::TerminalId::new(id_str);
                    if let Some(s) = term_out.get("data").and_then(|v| v.as_str()) {
                        let data = s.as_bytes().to_vec();
                        thread
                            .update(cx, |thread, cx| {
                                thread.on_terminal_provider_event(
                                    TerminalProviderEvent::Output { terminal_id, data },
                                    cx,
                                );
                            })
                            .log_err();
                    }
                }
            }

            if let Some(term_exit) = meta.get("terminal_exit") {
                if let Some(id_str) = term_exit.get("terminal_id").and_then(|v| v.as_str()) {
                    let terminal_id = acp::TerminalId::new(id_str);
                    let status = acp::TerminalExitStatus::new()
                        .exit_code(
                            term_exit
                                .get("exit_code")
                                .and_then(|v| v.as_u64())
                                .map(|i| i as u32),
                        )
                        .signal(
                            term_exit
                                .get("signal")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        );

                    thread
                        .update(cx, |thread, cx| {
                            thread.on_terminal_provider_event(
                                TerminalProviderEvent::Exit {
                                    terminal_id,
                                    status,
                                },
                                cx,
                            );
                        })
                        .log_err();
                }
            }
        }
    }
}

fn handle_create_terminal(
    args: acp::CreateTerminalRequest,
    responder: Responder<acp::CreateTerminalResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };
    let project = match thread
        .read_with(cx, |thread, _cx| thread.project().clone())
        .map_err(acp::Error::from)
    {
        Ok(p) => p,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let result: Result<_, acp::Error> = async {
            let terminal_entity = acp_thread::create_terminal_entity(
                args.command.clone(),
                &args.args,
                args.env
                    .into_iter()
                    .map(|env| (env.name, env.value))
                    .collect(),
                args.cwd.clone(),
                &project,
                cx,
            )
            .await?;

            let terminal_entity = thread.update(cx, |thread, cx| {
                thread.register_terminal_created(
                    acp_v2::TerminalId::new(uuid::Uuid::new_v4().to_string()),
                    format!("{} {}", args.command, args.args.join(" ")),
                    args.cwd.clone(),
                    args.output_byte_limit,
                    terminal_entity,
                    cx,
                )
            })?;
            let terminal_id = terminal_entity.read_with(cx, |terminal, _| terminal.id().clone());
            Ok(terminal_id)
        }
        .await;

        match result {
            Ok(terminal_id) => {
                responder
                    .respond(acp::CreateTerminalResponse::new(acp::TerminalId::new(
                        terminal_id.0,
                    )))
                    .log_err();
            }
            Err(e) => respond_err(responder, e),
        }
    })
    .detach();
}

fn handle_kill_terminal(
    args: acp::KillTerminalRequest,
    responder: Responder<acp::KillTerminalResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    match thread
        .update(cx, |thread, cx| {
            thread.kill_terminal(acp_v2::TerminalId::new(args.terminal_id.0), cx)
        })
        .flatten_acp()
    {
        Ok(()) => {
            responder
                .respond(acp::KillTerminalResponse::default())
                .log_err();
        }
        Err(e) => respond_err(responder, e),
    }
}

fn handle_release_terminal(
    args: acp::ReleaseTerminalRequest,
    responder: Responder<acp::ReleaseTerminalResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    match thread
        .update(cx, |thread, cx| {
            thread.release_terminal(acp_v2::TerminalId::new(args.terminal_id.0), cx)
        })
        .flatten_acp()
    {
        Ok(()) => {
            responder
                .respond(acp::ReleaseTerminalResponse::default())
                .log_err();
        }
        Err(e) => respond_err(responder, e),
    }
}

fn handle_terminal_output(
    args: acp::TerminalOutputRequest,
    responder: Responder<acp::TerminalOutputResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    match thread
        .read_with(cx, |thread, cx| -> anyhow::Result<_> {
            let out = thread
                .terminal(acp_v2::TerminalId::new(args.terminal_id.0))?
                .read(cx)
                .current_output(cx);
            Ok(out)
        })
        .flatten_acp()
    {
        Ok(output) => {
            responder.respond(process_output_to_v1(output)).log_err();
        }
        Err(e) => respond_err(responder, e),
    }
}

fn process_output_to_v1(output: acp_thread::ProcessOutput) -> acp::TerminalOutputResponse {
    acp::TerminalOutputResponse::new(output.output, output.truncated)
        .exit_status(output.exit_status.map(terminal_exit_status_to_v1))
}

fn terminal_exit_status_to_v1(status: acp_v2::TerminalExitStatus) -> acp::TerminalExitStatus {
    acp::TerminalExitStatus::new()
        .exit_code(status.exit_code)
        .signal(status.signal)
        .meta(status.meta)
}

fn handle_wait_for_terminal_exit(
    args: acp::WaitForTerminalExitRequest,
    responder: Responder<acp::WaitForTerminalExitResponse>,
    cx: &mut AsyncApp,
    ctx: &ClientContext,
) {
    let thread = match session_thread(ctx, &acp_v2::SessionId::new(args.session_id.0)) {
        Ok(t) => t,
        Err(e) => return respond_err(responder, e),
    };

    cx.spawn(async move |cx| {
        let cancellation = responder.cancellation();
        let result = cancellation
            .run_until_cancelled(async {
                let exit_status = thread
                    .update(cx, |thread, cx| {
                        thread
                            .terminal(acp_v2::TerminalId::new(args.terminal_id.0))?
                            .read(cx)
                            .wait_for_exit()
                    })
                    .flatten_acp()?
                    .await;
                Ok(exit_status)
            })
            .await;

        respond_result(
            responder,
            result.map(|status| {
                acp::WaitForTerminalExitResponse::new(terminal_exit_status_to_v1(status))
            }),
        );
    })
    .detach();
}
