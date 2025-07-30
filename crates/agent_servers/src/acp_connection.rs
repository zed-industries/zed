use agent_client_protocol as acp;
use anyhow::anyhow;
use collections::HashMap;
use context_server::listener::McpServerTool;
use context_server::types::requests;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use futures::channel::{mpsc, oneshot};
use project::Project;
use smol::stream::StreamExt as _;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::{path::Path, sync::Arc};
use util::{ResultExt, TryFutureExt};

use anyhow::{Context, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task, WeakEntity};

use crate::mcp_server::ZedMcpServer;
use crate::{AgentServerCommand, mcp_server};
use acp_thread::{AcpThread, AgentConnection};

pub struct AcpConnection {
    agent_state: Rc<RefCell<acp::AgentState>>,
    server_name: &'static str,
    context_server: Arc<context_server::ContextServer>,
    sessions: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
    _agent_state_task: Task<()>,
    _session_update_task: Task<()>,
}

impl AcpConnection {
    pub async fn stdio(
        server_name: &'static str,
        command: AgentServerCommand,
        working_directory: Option<Arc<Path>>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let context_server: Arc<ContextServer> = ContextServer::stdio(
            ContextServerId(format!("{}-mcp-server", server_name).into()),
            ContextServerCommand {
                path: command.path,
                args: command.args,
                env: command.env,
            },
            working_directory,
        )
        .into();

        let (mut state_tx, mut state_rx) = watch::channel(acp::AgentState::default());
        let (notification_tx, mut notification_rx) = mpsc::unbounded();

        let sessions = Rc::new(RefCell::new(HashMap::default()));
        let initial_state = state_rx.recv().await?;
        let agent_state = Rc::new(RefCell::new(initial_state));

        let agent_state_task = cx.foreground_executor().spawn({
            let agent_state = agent_state.clone();
            async move {
                while let Some(state) = state_rx.recv().log_err().await {
                    agent_state.replace(state);
                }
            }
        });

        let session_update_handler_task = cx.spawn({
            let sessions = sessions.clone();
            async move |cx| {
                while let Some(notification) = notification_rx.next().await {
                    Self::handle_session_notification(notification, sessions.clone(), cx)
                }
            }
        });

        context_server
            .start_with_handlers(
                vec![
                    (acp::AGENT_METHODS.agent_state, {
                        Box::new(move |notification, _cx| {
                            log::trace!(
                                "ACP Notification: {}",
                                serde_json::to_string_pretty(&notification).unwrap()
                            );

                            if let Some(state) =
                                serde_json::from_value::<acp::AgentState>(notification).log_err()
                            {
                                state_tx.send(state).log_err();
                            }
                        })
                    }),
                    (acp::AGENT_METHODS.session_update, {
                        Box::new(move |notification, _cx| {
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
                        })
                    }),
                ],
                cx,
            )
            .await?;

        Ok(Self {
            server_name,
            context_server,
            sessions,
            agent_state,
            _agent_state_task: agent_state_task,
            _session_update_task: session_update_handler_task,
        })
    }

    pub fn handle_session_notification(
        notification: acp::SessionNotification,
        threads: Rc<RefCell<HashMap<acp::SessionId, AcpSession>>>,
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

pub struct AcpSession {
    thread: WeakEntity<AcpThread>,
    cancel_tx: Option<oneshot::Sender<()>>,
    _mcp_server: ZedMcpServer,
}

impl AgentConnection for AcpConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>> {
        let client = self.context_server.client();
        let sessions = self.sessions.clone();
        let cwd = cwd.to_path_buf();
        cx.spawn(async move |cx| {
            let client = client.context("MCP server is not initialized yet")?;
            let (mut thread_tx, thread_rx) = watch::channel(WeakEntity::new_invalid());

            let mcp_server = ZedMcpServer::new(thread_rx, cx).await?;

            let response = client
                .request::<requests::CallTool>(context_server::types::CallToolParams {
                    name: acp::AGENT_METHODS.new_session.into(),
                    arguments: Some(serde_json::to_value(acp::NewSessionArguments {
                        mcp_servers: vec![mcp_server.server_config()?],
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

            let thread = cx.new(|cx| {
                AcpThread::new(
                    self.server_name,
                    self.clone(),
                    project,
                    result.session_id.clone(),
                    cx,
                )
            })?;

            thread_tx.send(thread.downgrade())?;

            let session = AcpSession {
                thread: thread.downgrade(),
                cancel_tx: None,
                _mcp_server: mcp_server,
            };
            sessions.borrow_mut().insert(result.session_id, session);

            Ok(thread)
        })
    }

    fn state(&self) -> Ref<'_, acp::AgentState> {
        self.agent_state.borrow()
    }

    fn authenticate(&self, method_id: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
        let client = self.context_server.client();
        cx.foreground_executor().spawn(async move {
            let params = acp::AuthenticateArguments { method_id };

            let response = client
                .context("MCP server is not initialized yet")?
                .request::<requests::CallTool>(context_server::types::CallToolParams {
                    name: acp::AGENT_METHODS.authenticate.into(),
                    arguments: Some(serde_json::to_value(params)?),
                    meta: None,
                })
                .await?;

            if response.is_error.unwrap_or_default() {
                Err(anyhow!(response.text_contents()))
            } else {
                Ok(())
            }
        })
    }

    fn prompt(
        &self,
        params: agent_client_protocol::PromptArguments,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let client = self.context_server.client();
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
                        name: acp::AGENT_METHODS.prompt.into(),
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

impl Drop for AcpConnection {
    fn drop(&mut self) {
        self.context_server.stop().log_err();
    }
}
