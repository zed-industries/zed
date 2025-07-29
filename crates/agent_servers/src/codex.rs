use agent_client_protocol as acp;
use anyhow::anyhow;
use collections::HashMap;
use context_server::listener::McpServerTool;
use context_server::types::requests;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use futures::channel::{mpsc, oneshot};
use project::Project;
use settings::SettingsStore;
use smol::stream::StreamExt as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::{path::Path, sync::Arc};
use util::ResultExt;

use anyhow::{Context, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task, WeakEntity};

use crate::mcp_server::ZedMcpServer;
use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings, mcp_server};
use acp_thread::{AcpThread, AgentConnection};

#[derive(Clone)]
pub struct Codex;

impl AgentServer for Codex {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Welcome to Codex"
    }

    fn empty_state_message(&self) -> &'static str {
        "What can I help with?"
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    fn connect(
        &self,
        _root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let project = project.clone();
        let working_directory = project.read(cx).active_project_directory(cx);
        cx.spawn(async move |cx| {
            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).codex.clone()
            })?;

            let Some(command) =
                AgentServerCommand::resolve("codex", &["mcp"], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find codex binary");
            };

            let client: Arc<ContextServer> = ContextServer::stdio(
                ContextServerId("codex-mcp-server".into()),
                ContextServerCommand {
                    path: command.path,
                    args: command.args,
                    env: command.env,
                },
                working_directory,
            )
            .into();
            ContextServer::start(client.clone(), cx).await?;

            let (notification_tx, mut notification_rx) = mpsc::unbounded();
            client
                .client()
                .context("Failed to subscribe")?
                .on_notification(acp::SESSION_UPDATE_METHOD_NAME, {
                    move |notification, _cx| {
                        let notification_tx = notification_tx.clone();
                        log::trace!(
                            "ACP Notification: {}",
                            serde_json::to_string_pretty(&notification).unwrap()
                        );

                        if let Some(notification) =
                            serde_json::from_value::<acp::SessionNotification>(notification)
                                .log_err()
                        {
                            notification_tx.unbounded_send(notification).ok();
                        }
                    }
                });

            let sessions = Rc::new(RefCell::new(HashMap::default()));

            let notification_handler_task = cx.spawn({
                let sessions = sessions.clone();
                async move |cx| {
                    while let Some(notification) = notification_rx.next().await {
                        CodexConnection::handle_session_notification(
                            notification,
                            sessions.clone(),
                            cx,
                        )
                    }
                }
            });

            let connection = CodexConnection {
                client,
                sessions,
                _notification_handler_task: notification_handler_task,
            };
            Ok(Rc::new(connection) as _)
        })
    }
}

struct CodexConnection {
    client: Arc<context_server::ContextServer>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, CodexSession>>>,
    _notification_handler_task: Task<()>,
}

struct CodexSession {
    thread: WeakEntity<AcpThread>,
    cancel_tx: Option<oneshot::Sender<()>>,
    _mcp_server: ZedMcpServer,
}

impl AgentConnection for CodexConnection {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>> {
        let client = self.client.client();
        let sessions = self.sessions.clone();
        let cwd = cwd.to_path_buf();
        cx.spawn(async move |cx| {
            let client = client.context("MCP server is not initialized yet")?;
            let (mut thread_tx, thread_rx) = watch::channel(WeakEntity::new_invalid());

            let mcp_server = ZedMcpServer::new(thread_rx, cx).await?;

            let response = client
                .request::<requests::CallTool>(context_server::types::CallToolParams {
                    name: acp::NEW_SESSION_TOOL_NAME.into(),
                    arguments: Some(serde_json::to_value(acp::NewSessionArguments {
                        mcp_servers: [(
                            mcp_server::SERVER_NAME.to_string(),
                            mcp_server.server_config()?,
                        )]
                        .into(),
                        client_tools: acp::ClientTools {
                            request_permission: Some(acp::McpToolId {
                                mcp_server: mcp_server::SERVER_NAME.into(),
                                tool_name: mcp_server::RequestPermissionTool::NAME.into(),
                            }),
                            read_text_file: Some(acp::McpToolId {
                                mcp_server: mcp_server::SERVER_NAME.into(),
                                tool_name: mcp_server::ReadTextFileTool::NAME.into(),
                            }),
                            write_text_file: Some(acp::McpToolId {
                                mcp_server: mcp_server::SERVER_NAME.into(),
                                tool_name: mcp_server::WriteTextFileTool::NAME.into(),
                            }),
                        },
                        cwd,
                    })?),
                    meta: None,
                })
                .await?;

            if response.is_error.unwrap_or_default() {
                return Err(anyhow!(response.text_contents()));
            }

            let result = serde_json::from_value::<acp::NewSessionOutput>(
                response.structured_content.context("Empty response")?,
            )?;

            let thread =
                cx.new(|cx| AcpThread::new(self.clone(), project, result.session_id.clone(), cx))?;

            thread_tx.send(thread.downgrade())?;

            let session = CodexSession {
                thread: thread.downgrade(),
                cancel_tx: None,
                _mcp_server: mcp_server,
            };
            sessions.borrow_mut().insert(result.session_id, session);

            Ok(thread)
        })
    }

    fn authenticate(&self, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("Authentication not supported")))
    }

    fn prompt(
        &self,
        params: agent_client_protocol::PromptArguments,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let client = self.client.client();
        let sessions = self.sessions.clone();

        cx.foreground_executor().spawn(async move {
            let client = client.context("MCP server is not initialized yet")?;

            let (new_cancel_tx, cancel_rx) = oneshot::channel();
            {
                let mut sessions = sessions.borrow_mut();
                let session = sessions
                    .get_mut(&params.session_id)
                    .context("Session not found")?;
                session.cancel_tx.replace(new_cancel_tx);
            }

            let result = client
                .request_with::<requests::CallTool>(
                    context_server::types::CallToolParams {
                        name: acp::PROMPT_TOOL_NAME.into(),
                        arguments: Some(serde_json::to_value(params)?),
                        meta: None,
                    },
                    Some(cancel_rx),
                    None,
                )
                .await;

            if let Err(err) = &result
                && err.is::<context_server::client::RequestCanceled>()
            {
                return Ok(());
            }

            let response = result?;

            if response.is_error.unwrap_or_default() {
                return Err(anyhow!(response.text_contents()));
            }

            Ok(())
        })
    }

    fn cancel(&self, session_id: &agent_client_protocol::SessionId, _cx: &mut App) {
        let mut sessions = self.sessions.borrow_mut();

        if let Some(cancel_tx) = sessions
            .get_mut(session_id)
            .and_then(|session| session.cancel_tx.take())
        {
            cancel_tx.send(()).ok();
        }
    }
}

impl CodexConnection {
    pub fn handle_session_notification(
        notification: acp::SessionNotification,
        threads: Rc<RefCell<HashMap<acp::SessionId, CodexSession>>>,
        cx: &mut AsyncApp,
    ) {
        let threads = threads.borrow();
        let Some(thread) = threads
            .get(&notification.session_id)
            .and_then(|session| session.thread.upgrade())
        else {
            log::error!(
                "Thread not found for session ID: {}",
                notification.session_id
            );
            return;
        };

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(notification.update, cx)
            })
            .log_err();
    }
}

impl Drop for CodexConnection {
    fn drop(&mut self) {
        self.client.stop().log_err();
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(Codex, allow_option_id = "approve");

    pub fn local_command() -> AgentServerCommand {
        let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../codex/codex-rs/target/debug/codex");

        AgentServerCommand {
            path: cli_path,
            args: vec![],
            env: None,
        }
    }
}
