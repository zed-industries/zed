use acp_thread::AgentConnection;
use acp_tools::AcpConnectionRegistry;
use action_log::ActionLog;
use agent_client_protocol::{self as acp, Agent as _, ErrorCode};
use anyhow::anyhow;
use collections::HashMap;
use futures::AsyncBufReadExt as _;
use futures::io::BufReader;
use project::Project;
use project::agent_server_store::AgentServerCommand;
use serde::Deserialize;
use settings::Settings as _;
use task::ShellBuilder;
use util::ResultExt as _;

use std::path::PathBuf;
use std::{any::Any, cell::RefCell};
use std::{path::Path, rc::Rc};
use thiserror::Error;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, SharedString, Task, WeakEntity};

use acp_thread::{AcpThread, AuthRequired, LoadError, TerminalProviderEvent};
use terminal::TerminalBuilder;
use terminal::terminal_settings::{AlternateScroll, CursorShape, TerminalSettings};

#[derive(Debug, Error)]
#[error("Unsupported version")]
pub struct UnsupportedVersion;

pub struct AcpConnection {
    server_name: SharedString,
    telemetry_id: SharedString,
    connection: Rc<acp::ClientSideConnection>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    auth_methods: Vec<acp::AuthMethod>,
    agent_capabilities: acp::AgentCapabilities,
    default_mode: Option<acp::SessionModeId>,
    default_model: Option<acp::ModelId>,
    root_dir: PathBuf,
    // NB: Don't move this into the wait_task, since we need to ensure the process is
    // killed on drop (setting kill_on_drop on the command seems to not always work).
    child: smol::process::Child,
    _io_task: Task<Result<(), acp::Error>>,
    _wait_task: Task<Result<()>>,
    _stderr_task: Task<Result<()>>,
}

pub struct AcpSession {
    thread: WeakEntity<AcpThread>,
    suppress_abort_err: bool,
    models: Option<Rc<RefCell<acp::SessionModelState>>>,
    session_modes: Option<Rc<RefCell<acp::SessionModeState>>>,
}

pub async fn connect(
    server_name: SharedString,
    command: AgentServerCommand,
    root_dir: &Path,
    default_mode: Option<acp::SessionModeId>,
    default_model: Option<acp::ModelId>,
    is_remote: bool,
    cx: &mut AsyncApp,
) -> Result<Rc<dyn AgentConnection>> {
    let conn = AcpConnection::stdio(
        server_name,
        command.clone(),
        root_dir,
        default_mode,
        default_model,
        is_remote,
        cx,
    )
    .await?;
    Ok(Rc::new(conn) as _)
}

const MINIMUM_SUPPORTED_VERSION: acp::ProtocolVersion = acp::ProtocolVersion::V1;

impl AcpConnection {
    pub async fn stdio(
        server_name: SharedString,
        command: AgentServerCommand,
        root_dir: &Path,
        default_mode: Option<acp::SessionModeId>,
        default_model: Option<acp::ModelId>,
        is_remote: bool,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let shell = cx.update(|cx| TerminalSettings::get(None, cx).shell.clone())?;
        let builder = ShellBuilder::new(&shell, cfg!(windows)).non_interactive();
        let mut child =
            builder.build_command(Some(command.path.display().to_string()), &command.args);
        child
            .envs(command.env.iter().flatten())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        if !is_remote {
            child.current_dir(root_dir);
        }
        let mut child = child.spawn()?;

        let stdout = child.stdout.take().context("Failed to take stdout")?;
        let stdin = child.stdin.take().context("Failed to take stdin")?;
        let stderr = child.stderr.take().context("Failed to take stderr")?;
        log::debug!(
            "Spawning external agent server: {:?}, {:?}",
            command.path,
            command.args
        );
        log::trace!("Spawned (pid: {})", child.id());

        let sessions = Rc::new(RefCell::new(HashMap::default()));

        let (release_channel, version) = cx.update(|cx| {
            (
                release_channel::ReleaseChannel::try_global(cx)
                    .map(|release_channel| release_channel.display_name()),
                release_channel::AppVersion::global(cx).to_string(),
            )
        })?;

        let client = ClientDelegate {
            sessions: sessions.clone(),
            cx: cx.clone(),
        };
        let (connection, io_task) = acp::ClientSideConnection::new(client, stdin, stdout, {
            let foreground_executor = cx.foreground_executor().clone();
            move |fut| {
                foreground_executor.spawn(fut).detach();
            }
        });

        let io_task = cx.background_spawn(io_task);

        let stderr_task = cx.background_spawn(async move {
            let mut stderr = BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(n) = stderr.read_line(&mut line).await
                && n > 0
            {
                log::warn!("agent stderr: {}", line.trim());
                line.clear();
            }
            Ok(())
        });

        let wait_task = cx.spawn({
            let sessions = sessions.clone();
            let status_fut = child.status();
            async move |cx| {
                let status = status_fut.await?;

                for session in sessions.borrow().values() {
                    session
                        .thread
                        .update(cx, |thread, cx| {
                            thread.emit_load_error(LoadError::Exited { status }, cx)
                        })
                        .ok();
                }

                anyhow::Ok(())
            }
        });

        let connection = Rc::new(connection);

        cx.update(|cx| {
            AcpConnectionRegistry::default_global(cx).update(cx, |registry, cx| {
                registry.set_active_connection(server_name.clone(), &connection, cx)
            });
        })?;

        let response = connection
            .initialize(
                acp::InitializeRequest::new(acp::ProtocolVersion::V1)
                    .client_capabilities(
                        acp::ClientCapabilities::new()
                            .fs(acp::FileSystemCapability::new()
                                .read_text_file(true)
                                .write_text_file(true))
                            .terminal(true)
                            // Experimental: Allow for rendering terminal output from the agents
                            .meta(acp::Meta::from_iter([
                                ("terminal_output".into(), true.into()),
                                ("terminal-auth".into(), true.into()),
                            ])),
                    )
                    .client_info(
                        acp::Implementation::new("zed", version)
                            .title(release_channel.map(ToOwned::to_owned)),
                    ),
            )
            .await?;

        if response.protocol_version < MINIMUM_SUPPORTED_VERSION {
            return Err(UnsupportedVersion.into());
        }

        let telemetry_id = response
            .agent_info
            // Use the one the agent provides if we have one
            .map(|info| info.name.into())
            // Otherwise, just use the name
            .unwrap_or_else(|| server_name.clone());

        Ok(Self {
            auth_methods: response.auth_methods,
            root_dir: root_dir.to_owned(),
            connection,
            server_name,
            telemetry_id,
            sessions,
            agent_capabilities: response.agent_capabilities,
            default_mode,
            default_model,
            _io_task: io_task,
            _wait_task: wait_task,
            _stderr_task: stderr_task,
            child,
        })
    }

    pub fn prompt_capabilities(&self) -> &acp::PromptCapabilities {
        &self.agent_capabilities.prompt_capabilities
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}

impl Drop for AcpConnection {
    fn drop(&mut self) {
        // See the comment on the child field.
        self.child.kill().log_err();
    }
}

impl AgentConnection for AcpConnection {
    fn telemetry_id(&self) -> SharedString {
        self.telemetry_id.clone()
    }

    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let name = self.server_name.clone();
        let conn = self.connection.clone();
        let sessions = self.sessions.clone();
        let default_mode = self.default_mode.clone();
        let default_model = self.default_model.clone();
        let cwd = cwd.to_path_buf();
        let context_server_store = project.read(cx).context_server_store().read(cx);
        let mcp_servers = if project.read(cx).is_local() {
            context_server_store
                .configured_server_ids()
                .iter()
                .filter_map(|id| {
                    let configuration = context_server_store.configuration_for_server(id)?;
                    match &*configuration {
                        project::context_server_store::ContextServerConfiguration::Custom {
                            command,
                            ..
                        }
                        | project::context_server_store::ContextServerConfiguration::Extension {
                            command,
                            ..
                        } => Some(acp::McpServer::Stdio(
                            acp::McpServerStdio::new(id.0.to_string(), &command.path)
                                .args(command.args.clone())
                                .env(if let Some(env) = command.env.as_ref() {
                                    env.iter()
                                        .map(|(name, value)| acp::EnvVariable::new(name, value))
                                        .collect()
                                } else {
                                    vec![]
                                }),
                        )),
                        project::context_server_store::ContextServerConfiguration::Http {
                            url,
                            headers,
                        } => Some(acp::McpServer::Http(
                            acp::McpServerHttp::new(id.0.to_string(), url.to_string()).headers(
                                headers
                                    .iter()
                                    .map(|(name, value)| acp::HttpHeader::new(name, value))
                                    .collect(),
                            ),
                        )),
                    }
                })
                .collect()
        } else {
            // In SSH projects, the external agent is running on the remote
            // machine, and currently we only run MCP servers on the local
            // machine. So don't pass any MCP servers to the agent in that case.
            Vec::new()
        };

        cx.spawn(async move |cx| {
            let response = conn
                .new_session(acp::NewSessionRequest::new(cwd).mcp_servers(mcp_servers))
                .await
                .map_err(|err| {
                    if err.code == acp::ErrorCode::AuthRequired {
                        let mut error = AuthRequired::new();

                        if err.message != acp::ErrorCode::AuthRequired.to_string() {
                            error = error.with_description(err.message);
                        }

                        anyhow!(error)
                    } else {
                        anyhow!(err)
                    }
                })?;

            let modes = response.modes.map(|modes| Rc::new(RefCell::new(modes)));
            let models = response.models.map(|models| Rc::new(RefCell::new(models)));

            if let Some(default_mode) = default_mode {
                if let Some(modes) = modes.as_ref() {
                    let mut modes_ref = modes.borrow_mut();
                    let has_mode = modes_ref.available_modes.iter().any(|mode| mode.id == default_mode);

                    if has_mode {
                        let initial_mode_id = modes_ref.current_mode_id.clone();

                        cx.spawn({
                            let default_mode = default_mode.clone();
                            let session_id = response.session_id.clone();
                            let modes = modes.clone();
                            let conn = conn.clone();
                            async move |_| {
                                let result = conn.set_session_mode(acp::SetSessionModeRequest::new(session_id, default_mode))
                                .await.log_err();

                                if result.is_none() {
                                    modes.borrow_mut().current_mode_id = initial_mode_id;
                                }
                            }
                        }).detach();

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
                } else {
                    log::warn!(
                        "`{name}` does not support modes, but `default_mode` was set in settings.",
                    );
                }
            }

            if let Some(default_model) = default_model {
                if let Some(models) = models.as_ref() {
                    let mut models_ref = models.borrow_mut();
                    let has_model = models_ref.available_models.iter().any(|model| model.model_id == default_model);

                    if has_model {
                        let initial_model_id = models_ref.current_model_id.clone();

                        cx.spawn({
                            let default_model = default_model.clone();
                            let session_id = response.session_id.clone();
                            let models = models.clone();
                            let conn = conn.clone();
                            async move |_| {
                                let result = conn.set_session_model(acp::SetSessionModelRequest::new(session_id, default_model))
                                .await.log_err();

                                if result.is_none() {
                                    models.borrow_mut().current_model_id = initial_model_id;
                                }
                            }
                        }).detach();

                        models_ref.current_model_id = default_model;
                    } else {
                        let available_models = models_ref
                            .available_models
                            .iter()
                            .map(|model| format!("- `{}`: {}", model.model_id, model.name))
                            .collect::<Vec<_>>()
                            .join("\n");

                        log::warn!(
                            "`{default_model}` is not a valid {name} model. Available options:\n{available_models}",
                        );
                    }
                } else {
                    log::warn!(
                        "`{name}` does not support model selection, but `default_model` was set in settings.",
                    );
                }
            }

            let session_id = response.session_id;
            let action_log = cx.new(|_| ActionLog::new(project.clone()))?;
            let thread = cx.new(|cx| {
                AcpThread::new(
                    self.server_name.clone(),
                    self.clone(),
                    project,
                    action_log,
                    session_id.clone(),
                    // ACP doesn't currently support per-session prompt capabilities or changing capabilities dynamically.
                    watch::Receiver::constant(self.agent_capabilities.prompt_capabilities.clone()),
                    cx,
                )
            })?;


            let session = AcpSession {
                thread: thread.downgrade(),
                suppress_abort_err: false,
                session_modes: modes,
                models,
            };
            sessions.borrow_mut().insert(session_id, session);

            Ok(thread)
        })
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &self.auth_methods
    }

    fn authenticate(&self, method_id: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            conn.authenticate(acp::AuthenticateRequest::new(method_id))
                .await?;
            Ok(())
        })
    }

    fn prompt(
        &self,
        _id: Option<acp_thread::UserMessageId>,
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        let conn = self.connection.clone();
        let sessions = self.sessions.clone();
        let session_id = params.session_id.clone();
        cx.foreground_executor().spawn(async move {
            let result = conn.prompt(params).await;

            let mut suppress_abort_err = false;

            if let Some(session) = sessions.borrow_mut().get_mut(&session_id) {
                suppress_abort_err = session.suppress_abort_err;
                session.suppress_abort_err = false;
            }

            match result {
                Ok(response) => Ok(response),
                Err(err) => {
                    if err.code == acp::ErrorCode::AuthRequired {
                        return Err(anyhow!(acp::Error::auth_required()));
                    }

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

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
        if let Some(session) = self.sessions.borrow_mut().get_mut(session_id) {
            session.suppress_abort_err = true;
        }
        let conn = self.connection.clone();
        let params = acp::CancelNotification::new(session_id.clone());
        cx.foreground_executor()
            .spawn(async move { conn.cancel(params).await })
            .detach();
    }

    fn session_modes(
        &self,
        session_id: &acp::SessionId,
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

    fn model_selector(
        &self,
        session_id: &acp::SessionId,
    ) -> Option<Rc<dyn acp_thread::AgentModelSelector>> {
        let sessions = self.sessions.clone();
        let sessions_ref = sessions.borrow();
        let Some(session) = sessions_ref.get(session_id) else {
            return None;
        };

        if let Some(models) = session.models.as_ref() {
            Some(Rc::new(AcpModelSelector::new(
                session_id.clone(),
                self.connection.clone(),
                models.clone(),
            )) as _)
        } else {
            None
        }
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

struct AcpSessionModes {
    session_id: acp::SessionId,
    connection: Rc<acp::ClientSideConnection>,
    state: Rc<RefCell<acp::SessionModeState>>,
}

impl acp_thread::AgentSessionModes for AcpSessionModes {
    fn current_mode(&self) -> acp::SessionModeId {
        self.state.borrow().current_mode_id.clone()
    }

    fn all_modes(&self) -> Vec<acp::SessionMode> {
        self.state.borrow().available_modes.clone()
    }

    fn set_mode(&self, mode_id: acp::SessionModeId, cx: &mut App) -> Task<Result<()>> {
        let connection = self.connection.clone();
        let session_id = self.session_id.clone();
        let old_mode_id;
        {
            let mut state = self.state.borrow_mut();
            old_mode_id = state.current_mode_id.clone();
            state.current_mode_id = mode_id.clone();
        };
        let state = self.state.clone();
        cx.foreground_executor().spawn(async move {
            let result = connection
                .set_session_mode(acp::SetSessionModeRequest::new(session_id, mode_id))
                .await;

            if result.is_err() {
                state.borrow_mut().current_mode_id = old_mode_id;
            }

            result?;

            Ok(())
        })
    }
}

struct AcpModelSelector {
    session_id: acp::SessionId,
    connection: Rc<acp::ClientSideConnection>,
    state: Rc<RefCell<acp::SessionModelState>>,
}

impl AcpModelSelector {
    fn new(
        session_id: acp::SessionId,
        connection: Rc<acp::ClientSideConnection>,
        state: Rc<RefCell<acp::SessionModelState>>,
    ) -> Self {
        Self {
            session_id,
            connection,
            state,
        }
    }
}

impl acp_thread::AgentModelSelector for AcpModelSelector {
    fn list_models(&self, _cx: &mut App) -> Task<Result<acp_thread::AgentModelList>> {
        Task::ready(Ok(acp_thread::AgentModelList::Flat(
            self.state
                .borrow()
                .available_models
                .clone()
                .into_iter()
                .map(acp_thread::AgentModelInfo::from)
                .collect(),
        )))
    }

    fn select_model(&self, model_id: acp::ModelId, cx: &mut App) -> Task<Result<()>> {
        let connection = self.connection.clone();
        let session_id = self.session_id.clone();
        let old_model_id;
        {
            let mut state = self.state.borrow_mut();
            old_model_id = state.current_model_id.clone();
            state.current_model_id = model_id.clone();
        };
        let state = self.state.clone();
        cx.foreground_executor().spawn(async move {
            let result = connection
                .set_session_model(acp::SetSessionModelRequest::new(session_id, model_id))
                .await;

            if result.is_err() {
                state.borrow_mut().current_model_id = old_model_id;
            }

            result?;

            Ok(())
        })
    }

    fn selected_model(&self, _cx: &mut App) -> Task<Result<acp_thread::AgentModelInfo>> {
        let state = self.state.borrow();
        Task::ready(
            state
                .available_models
                .iter()
                .find(|m| m.model_id == state.current_model_id)
                .cloned()
                .map(acp_thread::AgentModelInfo::from)
                .ok_or_else(|| anyhow::anyhow!("Model not found")),
        )
    }
}

struct ClientDelegate {
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    cx: AsyncApp,
}

#[async_trait::async_trait(?Send)]
impl acp::Client for ClientDelegate {
    async fn request_permission(
        &self,
        arguments: acp::RequestPermissionRequest,
    ) -> Result<acp::RequestPermissionResponse, acp::Error> {
        let respect_always_allow_setting;
        let thread;
        {
            let sessions_ref = self.sessions.borrow();
            let session = sessions_ref
                .get(&arguments.session_id)
                .context("Failed to get session")?;
            respect_always_allow_setting = session.session_modes.is_none();
            thread = session.thread.clone();
        }

        let cx = &mut self.cx.clone();

        let task = thread.update(cx, |thread, cx| {
            thread.request_tool_call_authorization(
                arguments.tool_call,
                arguments.options,
                respect_always_allow_setting,
                cx,
            )
        })??;

        let outcome = task.await;

        Ok(acp::RequestPermissionResponse::new(outcome))
    }

    async fn write_text_file(
        &self,
        arguments: acp::WriteTextFileRequest,
    ) -> Result<acp::WriteTextFileResponse, acp::Error> {
        let cx = &mut self.cx.clone();
        let task = self
            .session_thread(&arguments.session_id)?
            .update(cx, |thread, cx| {
                thread.write_text_file(arguments.path, arguments.content, cx)
            })?;

        task.await?;

        Ok(Default::default())
    }

    async fn read_text_file(
        &self,
        arguments: acp::ReadTextFileRequest,
    ) -> Result<acp::ReadTextFileResponse, acp::Error> {
        let task = self.session_thread(&arguments.session_id)?.update(
            &mut self.cx.clone(),
            |thread, cx| {
                thread.read_text_file(arguments.path, arguments.line, arguments.limit, false, cx)
            },
        )?;

        let content = task.await?;

        Ok(acp::ReadTextFileResponse::new(content))
    }

    async fn session_notification(
        &self,
        notification: acp::SessionNotification,
    ) -> Result<(), acp::Error> {
        let sessions = self.sessions.borrow();
        let session = sessions
            .get(&notification.session_id)
            .context("Failed to get session")?;

        if let acp::SessionUpdate::CurrentModeUpdate(acp::CurrentModeUpdate {
            current_mode_id,
            ..
        }) = &notification.update
        {
            if let Some(session_modes) = &session.session_modes {
                session_modes.borrow_mut().current_mode_id = current_mode_id.clone();
            } else {
                log::error!(
                    "Got a `CurrentModeUpdate` notification, but they agent didn't specify `modes` during setting setup."
                );
            }
        }

        // Clone so we can inspect meta both before and after handing off to the thread
        let update_clone = notification.update.clone();

        // Pre-handle: if a ToolCall carries terminal_info, create/register a display-only terminal.
        if let acp::SessionUpdate::ToolCall(tc) = &update_clone {
            if let Some(meta) = &tc.meta {
                if let Some(terminal_info) = meta.get("terminal_info") {
                    if let Some(id_str) = terminal_info.get("terminal_id").and_then(|v| v.as_str())
                    {
                        let terminal_id = acp::TerminalId::new(id_str);
                        let cwd = terminal_info
                            .get("cwd")
                            .and_then(|v| v.as_str().map(PathBuf::from));

                        // Create a minimal display-only lower-level terminal and register it.
                        let _ = session.thread.update(&mut self.cx.clone(), |thread, cx| {
                            let builder = TerminalBuilder::new_display_only(
                                CursorShape::default(),
                                AlternateScroll::On,
                                None,
                                0,
                            )?;
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
                            anyhow::Ok(())
                        });
                    }
                }
            }
        }

        // Forward the update to the acp_thread as usual.
        session.thread.update(&mut self.cx.clone(), |thread, cx| {
            thread.handle_session_update(notification.update.clone(), cx)
        })??;

        // Post-handle: stream terminal output/exit if present on ToolCallUpdate meta.
        if let acp::SessionUpdate::ToolCallUpdate(tcu) = &update_clone {
            if let Some(meta) = &tcu.meta {
                if let Some(term_out) = meta.get("terminal_output") {
                    if let Some(id_str) = term_out.get("terminal_id").and_then(|v| v.as_str()) {
                        let terminal_id = acp::TerminalId::new(id_str);
                        if let Some(s) = term_out.get("data").and_then(|v| v.as_str()) {
                            let data = s.as_bytes().to_vec();
                            let _ = session.thread.update(&mut self.cx.clone(), |thread, cx| {
                                thread.on_terminal_provider_event(
                                    TerminalProviderEvent::Output { terminal_id, data },
                                    cx,
                                );
                            });
                        }
                    }
                }

                // terminal_exit
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

                        let _ = session.thread.update(&mut self.cx.clone(), |thread, cx| {
                            thread.on_terminal_provider_event(
                                TerminalProviderEvent::Exit {
                                    terminal_id,
                                    status,
                                },
                                cx,
                            );
                        });
                    }
                }
            }
        }

        Ok(())
    }

    async fn create_terminal(
        &self,
        args: acp::CreateTerminalRequest,
    ) -> Result<acp::CreateTerminalResponse, acp::Error> {
        let thread = self.session_thread(&args.session_id)?;
        let project = thread.read_with(&self.cx, |thread, _cx| thread.project().clone())?;

        let terminal_entity = acp_thread::create_terminal_entity(
            args.command.clone(),
            &args.args,
            args.env
                .into_iter()
                .map(|env| (env.name, env.value))
                .collect(),
            args.cwd.clone(),
            &project,
            &mut self.cx.clone(),
        )
        .await?;

        // Register with renderer
        let terminal_entity = thread.update(&mut self.cx.clone(), |thread, cx| {
            thread.register_terminal_created(
                acp::TerminalId::new(uuid::Uuid::new_v4().to_string()),
                format!("{} {}", args.command, args.args.join(" ")),
                args.cwd.clone(),
                args.output_byte_limit,
                terminal_entity,
                cx,
            )
        })?;
        let terminal_id =
            terminal_entity.read_with(&self.cx, |terminal, _| terminal.id().clone())?;
        Ok(acp::CreateTerminalResponse::new(terminal_id))
    }

    async fn kill_terminal_command(
        &self,
        args: acp::KillTerminalCommandRequest,
    ) -> Result<acp::KillTerminalCommandResponse, acp::Error> {
        self.session_thread(&args.session_id)?
            .update(&mut self.cx.clone(), |thread, cx| {
                thread.kill_terminal(args.terminal_id, cx)
            })??;

        Ok(Default::default())
    }

    async fn ext_method(&self, _args: acp::ExtRequest) -> Result<acp::ExtResponse, acp::Error> {
        Err(acp::Error::method_not_found())
    }

    async fn ext_notification(&self, _args: acp::ExtNotification) -> Result<(), acp::Error> {
        Err(acp::Error::method_not_found())
    }

    async fn release_terminal(
        &self,
        args: acp::ReleaseTerminalRequest,
    ) -> Result<acp::ReleaseTerminalResponse, acp::Error> {
        self.session_thread(&args.session_id)?
            .update(&mut self.cx.clone(), |thread, cx| {
                thread.release_terminal(args.terminal_id, cx)
            })??;

        Ok(Default::default())
    }

    async fn terminal_output(
        &self,
        args: acp::TerminalOutputRequest,
    ) -> Result<acp::TerminalOutputResponse, acp::Error> {
        self.session_thread(&args.session_id)?
            .read_with(&mut self.cx.clone(), |thread, cx| {
                let out = thread
                    .terminal(args.terminal_id)?
                    .read(cx)
                    .current_output(cx);

                Ok(out)
            })?
    }

    async fn wait_for_terminal_exit(
        &self,
        args: acp::WaitForTerminalExitRequest,
    ) -> Result<acp::WaitForTerminalExitResponse, acp::Error> {
        let exit_status = self
            .session_thread(&args.session_id)?
            .update(&mut self.cx.clone(), |thread, cx| {
                anyhow::Ok(thread.terminal(args.terminal_id)?.read(cx).wait_for_exit())
            })??
            .await;

        Ok(acp::WaitForTerminalExitResponse::new(exit_status))
    }
}

impl ClientDelegate {
    fn session_thread(&self, session_id: &acp::SessionId) -> Result<WeakEntity<AcpThread>> {
        let sessions = self.sessions.borrow();
        sessions
            .get(session_id)
            .context("Failed to get session")
            .map(|session| session.thread.clone())
    }
}
