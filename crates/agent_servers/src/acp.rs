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
use util::ResultExt as _;

use std::path::PathBuf;
use std::{any::Any, cell::RefCell};
use std::{path::Path, rc::Rc};
use thiserror::Error;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, SharedString, Task, WeakEntity};

use acp_thread::{AcpThread, AuthRequired, LoadError};

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
    _io_task: Task<Result<()>>,
    _wait_task: Task<Result<()>>,
    _stderr_task: Task<Result<()>>,
}

pub struct AcpSession {
    thread: WeakEntity<AcpThread>,
    suppress_abort_err: bool,
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
        let mut child = util::command::new_smol_command(command.path);
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
                    },
                    terminal: true,
                },
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
                .new_session(acp::NewSessionRequest { mcp_servers, cwd })
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
                    watch::Receiver::constant(self.agent_capabilities.prompt_capabilities),
                    cx,
                )
            })?;

            let session = AcpSession {
                thread: thread.downgrade(),
                suppress_abort_err: false,
                session_modes: modes
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
            let result = conn
                .authenticate(acp::AuthenticateRequest {
                    method_id: method_id.clone(),
                })
                .await?;

            Ok(result)
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

struct ClientDelegate {
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    cx: AsyncApp,
}

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

        Ok(acp::RequestPermissionResponse { outcome })
    }

    async fn write_text_file(
        &self,
        arguments: acp::WriteTextFileRequest,
    ) -> Result<(), acp::Error> {
        let cx = &mut self.cx.clone();
        let task = self
            .session_thread(&arguments.session_id)?
            .update(cx, |thread, cx| {
                thread.write_text_file(arguments.path, arguments.content, cx)
            })?;

        task.await?;

        Ok(())
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

        Ok(acp::ReadTextFileResponse { content })
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

        session.thread.update(&mut self.cx.clone(), |thread, cx| {
            thread.handle_session_update(notification.update, cx)
        })??;

        Ok(())
    }

    async fn create_terminal(
        &self,
        args: acp::CreateTerminalRequest,
    ) -> Result<acp::CreateTerminalResponse, acp::Error> {
        let terminal = self
            .session_thread(&args.session_id)?
            .update(&mut self.cx.clone(), |thread, cx| {
                thread.create_terminal(
                    args.command,
                    args.args,
                    args.env,
                    args.cwd,
                    args.output_byte_limit,
                    cx,
                )
            })?
            .await?;
        Ok(
            terminal.read_with(&self.cx, |terminal, _| acp::CreateTerminalResponse {
                terminal_id: terminal.id().clone(),
            })?,
        )
    }

    async fn kill_terminal(&self, args: acp::KillTerminalRequest) -> Result<(), acp::Error> {
        self.session_thread(&args.session_id)?
            .update(&mut self.cx.clone(), |thread, cx| {
                thread.kill_terminal(args.terminal_id, cx)
            })??;

        Ok(())
    }

    async fn release_terminal(&self, args: acp::ReleaseTerminalRequest) -> Result<(), acp::Error> {
        self.session_thread(&args.session_id)?
            .update(&mut self.cx.clone(), |thread, cx| {
                thread.release_terminal(args.terminal_id, cx)
            })??;

        Ok(())
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

        Ok(acp::WaitForTerminalExitResponse { exit_status })
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
