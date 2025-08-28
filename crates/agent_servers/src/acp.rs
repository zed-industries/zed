use crate::AgentServerCommand;
use acp_thread::AgentConnection;
use acp_tools::AcpConnectionRegistry;
use action_log::ActionLog;
use agent_client_protocol::{self as acp, Agent as _, ErrorCode};
use anyhow::anyhow;
use collections::HashMap;
use futures::AsyncBufReadExt as _;
use futures::channel::oneshot;
use futures::io::BufReader;
use project::Project;
use serde::Deserialize;
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
    prompt_capabilities: acp::PromptCapabilities,
    _io_task: Task<Result<()>>,
    _wait_task: Task<Result<()>>,
    _stderr_task: Task<Result<()>>,
}

pub struct AcpSession {
    thread: WeakEntity<AcpThread>,
    suppress_abort_err: bool,
}

pub async fn connect(
    server_name: SharedString,
    command: AgentServerCommand,
    root_dir: &Path,
    cx: &mut AsyncApp,
) -> Result<Rc<dyn AgentConnection>> {
    let conn = AcpConnection::stdio(server_name, command.clone(), root_dir, cx).await?;
    Ok(Rc::new(conn) as _)
}

const MINIMUM_SUPPORTED_VERSION: acp::ProtocolVersion = acp::V1;

impl AcpConnection {
    pub async fn stdio(
        server_name: SharedString,
        command: AgentServerCommand,
        root_dir: &Path,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let mut child = util::command::new_smol_command(command.path)
            .args(command.args.iter().map(|arg| arg.as_str()))
            .envs(command.env.iter().flatten())
            .current_dir(root_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

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
            async move |cx| {
                let status = child.status().await?;

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
                },
            })
            .await?;

        if response.protocol_version < MINIMUM_SUPPORTED_VERSION {
            return Err(UnsupportedVersion.into());
        }

        Ok(Self {
            auth_methods: response.auth_methods,
            connection,
            server_name,
            sessions,
            prompt_capabilities: response.agent_capabilities.prompt_capabilities,
            _io_task: io_task,
            _wait_task: wait_task,
            _stderr_task: stderr_task,
        })
    }

    pub fn prompt_capabilities(&self) -> &acp::PromptCapabilities {
        &self.prompt_capabilities
    }
}

impl AgentConnection for AcpConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let conn = self.connection.clone();
        let sessions = self.sessions.clone();
        let cwd = cwd.to_path_buf();
        let context_server_store = project.read(cx).context_server_store().read(cx);
        let mcp_servers = context_server_store
            .configured_server_ids()
            .iter()
            .filter_map(|id| {
                let configuration = context_server_store.configuration_for_server(id)?;
                let command = configuration.command();
                Some(acp::McpServer {
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
            .collect();

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
                    watch::Receiver::constant(self.prompt_capabilities),
                    cx,
                )
            })?;

            let session = AcpSession {
                thread: thread.downgrade(),
                suppress_abort_err: false,
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

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
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
        let cx = &mut self.cx.clone();
        let rx = self
            .sessions
            .borrow()
            .get(&arguments.session_id)
            .context("Failed to get session")?
            .thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(arguments.tool_call, arguments.options, cx)
            })?;

        let result = rx?.await;

        let outcome = match result {
            Ok(option) => acp::RequestPermissionOutcome::Selected { option_id: option },
            Err(oneshot::Canceled) => acp::RequestPermissionOutcome::Cancelled,
        };

        Ok(acp::RequestPermissionResponse { outcome })
    }

    async fn write_text_file(
        &self,
        arguments: acp::WriteTextFileRequest,
    ) -> Result<(), acp::Error> {
        let cx = &mut self.cx.clone();
        let task = self
            .sessions
            .borrow()
            .get(&arguments.session_id)
            .context("Failed to get session")?
            .thread
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
        let cx = &mut self.cx.clone();
        let task = self
            .sessions
            .borrow()
            .get(&arguments.session_id)
            .context("Failed to get session")?
            .thread
            .update(cx, |thread, cx| {
                thread.read_text_file(arguments.path, arguments.line, arguments.limit, false, cx)
            })?;

        let content = task.await?;

        Ok(acp::ReadTextFileResponse { content })
    }

    async fn session_notification(
        &self,
        notification: acp::SessionNotification,
    ) -> Result<(), acp::Error> {
        let cx = &mut self.cx.clone();
        let sessions = self.sessions.borrow();
        let session = sessions
            .get(&notification.session_id)
            .context("Failed to get session")?;

        session.thread.update(cx, |thread, cx| {
            thread.handle_session_update(notification.update, cx)
        })??;

        Ok(())
    }
}
