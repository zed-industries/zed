use agent_client_protocol as acp;
use anyhow::anyhow;
use collections::HashMap;
use context_server::types::requests;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use futures::channel::mpsc;
use project::Project;
use settings::SettingsStore;
use smol::stream::StreamExt as _;
use std::cell::RefCell;
use std::rc::Rc;
use std::{path::Path, sync::Arc};
use util::ResultExt;

use anyhow::{Context, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task, WeakEntity};

use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings};
use acp_thread::{AcpThread, AgentConnection, AgentThreadEntry};

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
        ""
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
                            "ACP Notification: {:?}",
                            serde_json::to_string_pretty(&notification)
                        );

                        if let Some(notification) =
                            serde_json::from_value::<acp::SessionNotification>(notification)
                                .log_err()
                        {
                            notification_tx.unbounded_send(notification).ok();
                        }
                    }
                });

            let threads = Rc::new(RefCell::new(HashMap::default()));

            let notification_handler_task = cx.spawn({
                let threads = threads.clone();
                async move |cx| {
                    while let Some(notification) = notification_rx.next().await {
                        CodexConnection::handle_session_notification(
                            notification,
                            threads.clone(),
                            cx,
                        )
                    }
                }
            });

            let connection = CodexConnection {
                client,
                threads,
                _notification_handler_task: notification_handler_task,
            };
            Ok(Rc::new(connection) as _)
        })
    }
}

struct CodexConnection {
    client: Arc<context_server::ContextServer>,
    threads: Rc<RefCell<HashMap<acp::SessionId, WeakEntity<AcpThread>>>>,
    _notification_handler_task: Task<()>,
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
        let threads = self.threads.clone();
        let cwd = cwd.to_path_buf();
        cx.spawn(async move |cx| {
            let client = client.context("MCP server is not initialized yet")?;

            let response = client
                .request::<requests::CallTool>(context_server::types::CallToolParams {
                    name: acp::NEW_SESSION_TOOL_NAME.into(),
                    arguments: Some(serde_json::to_value(acp::NewSessionToolArguments {
                        mcp_servers: Default::default(),
                        client_tools: acp::ClientTools {
                            confirm_permission: None,
                            write_text_file: None,
                            read_text_file: None,
                        },
                        cwd,
                    })?),
                    meta: None,
                })
                .await?;

            if response.is_error.unwrap_or_default() {
                return Err(anyhow!("{:?}", response.content));
            }

            let result = serde_json::from_value::<acp::NewSessionToolResult>(
                response.structured_content.context("Empty response")?,
            )?;

            let thread =
                cx.new(|cx| AcpThread::new(self.clone(), project, result.session_id.clone(), cx))?;

            threads
                .borrow_mut()
                .insert(result.session_id, thread.downgrade());

            Ok(thread)
        })
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<()>> {
        todo!()
    }

    fn prompt(
        &self,
        params: agent_client_protocol::PromptToolArguments,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let client = self.client.client();

        cx.spawn(async move |cx| {
            let client = client.context("MCP server is not initialized yet")?;

            let response = client
                .request::<requests::CallTool>(context_server::types::CallToolParams {
                    name: acp::PROMPT_TOOL_NAME.into(),
                    arguments: Some(serde_json::to_value(params)?),
                    meta: None,
                })
                .await?;

            if response.is_error.unwrap_or_default() {
                return Err(anyhow!("{:?}", response.content));
            }

            Ok(())
        })
    }

    fn cancel(&self, session_id: &agent_client_protocol::SessionId, cx: &mut App) {
        todo!()
    }
}

impl CodexConnection {
    pub fn handle_session_notification(
        notification: acp::SessionNotification,
        threads: Rc<RefCell<HashMap<acp::SessionId, WeakEntity<AcpThread>>>>,
        cx: &mut AsyncApp,
    ) {
        let threads = threads.borrow();
        let Some(thread) = threads
            .get(&notification.session_id)
            .and_then(|thread| thread.upgrade())
        else {
            log::error!(
                "Thread not found for session ID: {}",
                notification.session_id
            );
            return;
        };

        match notification.update {
            acp::SessionUpdate::Started => {}
            acp::SessionUpdate::UserMessage(_) => {
                todo!()
            }
            acp::SessionUpdate::AgentMessageChunk(content_block) => {
                thread
                    .update(cx, |thread, cx| {
                        thread.push_assistant_content_block(content_block, false, cx);
                    })
                    .log_err();
            }
            acp::SessionUpdate::AgentThoughtChunk(content_block) => {
                thread
                    .update(cx, |thread, cx| {
                        thread.push_assistant_content_block(content_block, true, cx);
                    })
                    .log_err();
            }
            acp::SessionUpdate::ToolCall(tool_call) => {
                // todo!
            }
            acp::SessionUpdate::ToolCallUpdate(tool_call_update) => {
                // todo!
            }
            acp::SessionUpdate::Plan(plan) => {
                thread
                    .update(cx, |thread, cx| {
                        thread.update_plan(plan, cx);
                    })
                    .log_err();
            }
        }
    }
}

impl Drop for CodexConnection {
    fn drop(&mut self) {
        self.client.stop().log_err();
    }
}
