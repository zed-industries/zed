use agent_client_protocol::{self as acp, Agent as _};
use anyhow::anyhow;
use collections::HashMap;
use futures::channel::oneshot;
use project::Project;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task, WeakEntity};

use crate::{AgentServerCommand, acp::UnsupportedVersion};
use acp_thread::{AcpThread, AgentConnection, AuthRequired};

pub struct AcpConnection {
    server_name: &'static str,
    connection: Rc<acp::ClientSideConnection>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    auth_methods: Vec<acp::AuthMethod>,
    _io_task: Task<Result<()>>,
}

pub struct AcpSession {
    thread: WeakEntity<AcpThread>,
}

const MINIMUM_SUPPORTED_VERSION: acp::ProtocolVersion = acp::V1;

impl AcpConnection {
    pub async fn stdio(
        server_name: &'static str,
        command: AgentServerCommand,
        root_dir: &Path,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let mut child = util::command::new_smol_command(&command.path)
            .args(command.args.iter().map(|arg| arg.as_str()))
            .envs(command.env.iter().flatten())
            .current_dir(root_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .kill_on_drop(true)
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to take stdout");
        let stdin = child.stdin.take().expect("Failed to take stdin");
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

        cx.spawn({
            let sessions = sessions.clone();
            async move |cx| {
                let status = child.status().await?;

                for session in sessions.borrow().values() {
                    session
                        .thread
                        .update(cx, |thread, cx| thread.emit_server_exited(status, cx))
                        .ok();
                }

                anyhow::Ok(())
            }
        })
        .detach();

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
            connection: connection.into(),
            server_name,
            sessions,
            _io_task: io_task,
        })
    }
}

impl AgentConnection for AcpConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>> {
        let conn = self.connection.clone();
        let sessions = self.sessions.clone();
        let cwd = cwd.to_path_buf();
        cx.spawn(async move |cx| {
            let response = conn
                .new_session(acp::NewSessionRequest {
                    mcp_servers: vec![],
                    cwd,
                })
                .await
                .map_err(|err| {
                    if err.code == acp::ErrorCode::AUTH_REQUIRED.code {
                        anyhow!(AuthRequired)
                    } else {
                        anyhow!(err)
                    }
                })?;

            let session_id = response.session_id;

            let thread = cx.new(|cx| {
                AcpThread::new(
                    self.server_name,
                    self.clone(),
                    project,
                    session_id.clone(),
                    cx,
                )
            })?;

            let session = AcpSession {
                thread: thread.downgrade(),
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
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        let conn = self.connection.clone();
        cx.foreground_executor().spawn(async move {
            let response = conn.prompt(params).await?;
            Ok(response)
        })
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
        let conn = self.connection.clone();
        let params = acp::CancelNotification {
            session_id: session_id.clone(),
        };
        cx.foreground_executor()
            .spawn(async move { conn.cancel(params).await })
            .detach();
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

        let result = rx.await;

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
