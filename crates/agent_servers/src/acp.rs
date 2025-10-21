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
use settings::{Settings as _, SettingsLocation};
use task::Shell;
use util::{ResultExt as _, get_default_system_shell_preferring_bash};

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
    connection: Rc<acp::ClientSideConnection>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    auth_methods: Vec<acp::AuthMethod>,
    agent_capabilities: acp::AgentCapabilities,
    default_mode: Option<acp::SessionModeId>,
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
    is_remote: bool,
    cx: &mut AsyncApp,
) -> Result<Rc<dyn AgentConnection>> {
    let conn = AcpConnection::stdio(
        server_name,
        command.clone(),
        root_dir,
        default_mode,
        is_remote,
        cx,
    )
    .await?;
    Ok(Rc::new(conn) as _)
}

const MINIMUM_SUPPORTED_VERSION: acp::ProtocolVersion = acp::V1;

impl AcpConnection {
    pub async fn stdio(
        server_name: SharedString,
        command: AgentServerCommand,
        root_dir: &Path,
        default_mode: Option<acp::SessionModeId>,
        is_remote: bool,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let mut child = util::command::new_smol_command(&command.path);
        child
            .args(command.args.iter().map(|arg| arg.as_str()))
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
                log::warn!("agent stderr: {}", &line);
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
            .initialize(acp::InitializeRequest {
                protocol_version: acp::VERSION,
                client_capabilities: acp::ClientCapabilities {
                    fs: acp::FileSystemCapability {
                        read_text_file: true,
                        write_text_file: true,
                        meta: None,
                    },
                    terminal: true,
                    meta: Some(serde_json::json!({
                        // Experimental: Allow for rendering terminal output from the agents
                        "terminal_output": true,
                    })),
                },
                meta: None,
            })
            .await?;

        if response.protocol_version < MINIMUM_SUPPORTED_VERSION {
            return Err(UnsupportedVersion.into());
        }

        Ok(Self {
            auth_methods: response.auth_methods,
            root_dir: root_dir.to_owned(),
            connection,
            server_name,
            sessions,
            agent_capabilities: response.agent_capabilities,
            default_mode,
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
        let cwd = cwd.to_path_buf();
        let context_server_store = project.read(cx).context_server_store().read(cx);
        let mcp_servers = if project.read(cx).is_local() {
            context_server_store
                .configured_server_ids()
                .iter()
                .filter_map(|id| {
                    let configuration = context_server_store.configuration_for_server(id)?;
                    let command = configuration.command();
                    Some(acp::McpServer::Stdio {
                        name: id.0.to_string(),
                        command: command.path.clone(),
                        args: command.args.clone(),
                        env: if let Some(env) = command.env.as_ref() {
                            env.iter()
                                .map(|(name, value)| acp::EnvVariable {
                                    name: name.clone(),
                                    value: value.clone(),
                                    meta: None,
                                })
                                .collect()
                        } else {
                            vec![]
                        },
                    })
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
                .new_session(acp::NewSessionRequest { mcp_servers, cwd, meta: None })
                .await
                .map_err(|err| {
                    if err.code == acp::ErrorCode::AUTH_REQUIRED.code {
                        let mut error = AuthRequired::new();

                        if err.message != acp::ErrorCode::AUTH_REQUIRED.message {
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
                            async move |_| {
                                let result = conn.set_session_mode(acp::SetSessionModeRequest {
                                    session_id,
                                    mode_id: default_mode,
                                    meta: None,
                                })
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
            conn.authenticate(acp::AuthenticateRequest {
                method_id: method_id.clone(),
                meta: None,
            })
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
                    if err.code == acp::ErrorCode::AUTH_REQUIRED.code {
                        return Err(anyhow!(acp::Error::auth_required()));
                    }

                    if err.code != ErrorCode::INTERNAL_ERROR.code {
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
                                Ok(acp::PromptResponse {
                                    stop_reason: acp::StopReason::Cancelled,
                                    meta: None,
                                })
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
        let params = acp::CancelNotification {
            session_id: session_id.clone(),
            meta: None,
        };
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
                .set_session_mode(acp::SetSessionModeRequest {
                    session_id,
                    mode_id,
                    meta: None,
                })
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
                .set_session_model(acp::SetSessionModelRequest {
                    session_id,
                    model_id,
                    meta: None,
                })
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

        Ok(acp::RequestPermissionResponse {
            outcome,
            meta: None,
        })
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

        Ok(acp::ReadTextFileResponse {
            content,
            meta: None,
        })
    }

    async fn session_notification(
        &self,
        notification: acp::SessionNotification,
    ) -> Result<(), acp::Error> {
        let sessions = self.sessions.borrow();
        let session = sessions
            .get(&notification.session_id)
            .context("Failed to get session")?;

        if let acp::SessionUpdate::CurrentModeUpdate { current_mode_id } = &notification.update {
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
                        let terminal_id = acp::TerminalId(id_str.into());
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
                                    terminal_id: terminal_id.clone(),
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
                        let terminal_id = acp::TerminalId(id_str.into());
                        if let Some(s) = term_out.get("data").and_then(|v| v.as_str()) {
                            let data = s.as_bytes().to_vec();
                            let _ = session.thread.update(&mut self.cx.clone(), |thread, cx| {
                                thread.on_terminal_provider_event(
                                    TerminalProviderEvent::Output {
                                        terminal_id: terminal_id.clone(),
                                        data,
                                    },
                                    cx,
                                );
                            });
                        }
                    }
                }

                // terminal_exit
                if let Some(term_exit) = meta.get("terminal_exit") {
                    if let Some(id_str) = term_exit.get("terminal_id").and_then(|v| v.as_str()) {
                        let terminal_id = acp::TerminalId(id_str.into());
                        let status = acp::TerminalExitStatus {
                            exit_code: term_exit
                                .get("exit_code")
                                .and_then(|v| v.as_u64())
                                .map(|i| i as u32),
                            signal: term_exit
                                .get("signal")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            meta: None,
                        };
                        let _ = session.thread.update(&mut self.cx.clone(), |thread, cx| {
                            thread.on_terminal_provider_event(
                                TerminalProviderEvent::Exit {
                                    terminal_id: terminal_id.clone(),
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

        let mut env = if let Some(dir) = &args.cwd {
            project
                .update(&mut self.cx.clone(), |project, cx| {
                    let worktree = project.find_worktree(dir.as_path(), cx);
                    let shell = TerminalSettings::get(
                        worktree.as_ref().map(|(worktree, path)| SettingsLocation {
                            worktree_id: worktree.read(cx).id(),
                            path: &path,
                        }),
                        cx,
                    )
                    .shell
                    .clone();
                    project.directory_environment(&shell, dir.clone().into(), cx)
                })?
                .await
                .unwrap_or_default()
        } else {
            Default::default()
        };
        // Disables paging for `git` and hopefully other commands
        env.insert("PAGER".into(), "".into());
        for var in args.env {
            env.insert(var.name, var.value);
        }

        // Use remote shell or default system shell, as appropriate
        let shell = project
            .update(&mut self.cx.clone(), |project, cx| {
                project
                    .remote_client()
                    .and_then(|r| r.read(cx).default_system_shell())
                    .map(Shell::Program)
            })?
            .unwrap_or_else(|| Shell::Program(get_default_system_shell_preferring_bash()));
        let is_windows = project
            .read_with(&self.cx, |project, cx| project.path_style(cx).is_windows())
            .unwrap_or(cfg!(windows));
        let (task_command, task_args) = task::ShellBuilder::new(&shell, is_windows)
            .redirect_stdin_to_dev_null()
            .build(Some(args.command.clone()), &args.args);

        let terminal_entity = project
            .update(&mut self.cx.clone(), |project, cx| {
                project.create_terminal_task(
                    task::SpawnInTerminal {
                        command: Some(task_command),
                        args: task_args,
                        cwd: args.cwd.clone(),
                        env,
                        ..Default::default()
                    },
                    cx,
                )
            })?
            .await?;

        // Register with renderer
        let terminal_entity = thread.update(&mut self.cx.clone(), |thread, cx| {
            thread.register_terminal_created(
                acp::TerminalId(uuid::Uuid::new_v4().to_string().into()),
                format!("{} {}", args.command, args.args.join(" ")),
                args.cwd.clone(),
                args.output_byte_limit,
                terminal_entity,
                cx,
            )
        })?;
        let terminal_id =
            terminal_entity.read_with(&self.cx, |terminal, _| terminal.id().clone())?;
        Ok(acp::CreateTerminalResponse {
            terminal_id,
            meta: None,
        })
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

        Ok(acp::WaitForTerminalExitResponse {
            exit_status,
            meta: None,
        })
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
