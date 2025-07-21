use collections::HashMap;
use context_server::types::CallToolParams;
use context_server::types::requests::CallTool;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use futures::channel::{mpsc, oneshot};
use project::Project;
use settings::SettingsStore;
use smol::stream::StreamExt;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use agentic_coding_protocol::{
    self as acp, AnyAgentRequest, AnyAgentResult, Client as _, ProtocolVersion,
};
use anyhow::{Context, Result, anyhow};
use futures::future::LocalBoxFuture;
use futures::{AsyncWriteExt, FutureExt, SinkExt as _};
use gpui::{App, AppContext, Entity, Task};
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::mcp_server::{McpConfig, ZedMcpServer};
use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings};
use acp_thread::{AcpClientDelegate, AcpThread, AgentConnection};

#[derive(Clone)]
pub struct Codex;

impl AgentServer for Codex {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn empty_state_headline(&self) -> &'static str {
        self.name()
    }

    fn empty_state_message(&self) -> &'static str {
        ""
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    fn supports_always_allow(&self) -> bool {
        false
    }

    fn new_thread(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let project = project.clone();
        let root_dir = root_dir.to_path_buf();
        let title = self.name().into();
        cx.spawn(async move |cx| {
            let (mut delegate_tx, delegate_rx) = watch::channel(None);
            let tool_id_map = Rc::new(RefCell::new(HashMap::default()));

            let zed_mcp_server = ZedMcpServer::new(delegate_rx, tool_id_map.clone(), cx).await?;

            let mut mcp_servers = HashMap::default();
            mcp_servers.insert(
                crate::mcp_server::SERVER_NAME.to_string(),
                zed_mcp_server.server_config()?,
            );
            let mcp_config = McpConfig { mcp_servers };

            // todo! pass zed mcp server to codex tool
            let mcp_config_file = tempfile::NamedTempFile::new()?;
            let (mcp_config_file, _mcp_config_path) = mcp_config_file.into_parts();

            let mut mcp_config_file = smol::fs::File::from(mcp_config_file);
            mcp_config_file
                .write_all(serde_json::to_string(&mcp_config)?.as_bytes())
                .await?;
            mcp_config_file.flush().await?;

            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).codex.clone()
            })?;

            let Some(command) =
                AgentServerCommand::resolve("codex", &["mcp"], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find codex binary");
            };

            let codex_mcp_client: Arc<ContextServer> = ContextServer::stdio(
                ContextServerId("codex-mcp-server".into()),
                ContextServerCommand {
                    // todo! should we change ContextServerCommand to take a PathBuf?
                    path: command.path.to_string_lossy().to_string(),
                    args: command.args,
                    env: command.env,
                },
            )
            .into();

            ContextServer::start(codex_mcp_client.clone(), cx).await?;
            // todo! stop

            let (notification_tx, mut notification_rx) = mpsc::unbounded();

            codex_mcp_client
                .client()
                .context("Failed to subscribe to server")?
                .on_notification("codex/event", {
                    move |event, cx| {
                        let mut notification_tx = notification_tx.clone();
                        cx.background_spawn(async move {
                            log::trace!("Notification: {:?}", event);
                            if let Some(event) =
                                serde_json::from_value::<CodexEvent>(event).log_err()
                            {
                                notification_tx.send(event.msg).await.log_err();
                            }
                        })
                        .detach();
                    }
                });

            cx.new(|cx| {
                // todo! handle notifications
                let delegate = AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async());
                delegate_tx.send(Some(delegate.clone())).log_err();

                let handler_task = cx.spawn({
                    let delegate = delegate.clone();
                    async move |_, _cx| {
                        while let Some(notification) = notification_rx.next().await {
                            CodexAgentConnection::handle_acp_notification(&delegate, notification)
                                .await
                                .log_err();
                        }
                    }
                });

                let connection = CodexAgentConnection {
                    root_dir,
                    codex_mcp: codex_mcp_client,
                    cancel_request_tx: Default::default(),
                    _handler_task: handler_task,
                    _zed_mcp: zed_mcp_server,
                };

                acp_thread::AcpThread::new(connection, title, None, project.clone(), cx)
            })
        })
    }
}

impl AgentConnection for CodexAgentConnection {
    /// Send a request to the agent and wait for a response.
    fn request_any(
        &self,
        params: AnyAgentRequest,
    ) -> LocalBoxFuture<'static, Result<acp::AnyAgentResult>> {
        let client = self.codex_mcp.client();
        let root_dir = self.root_dir.clone();
        let cancel_request_tx = self.cancel_request_tx.clone();
        async move {
            let client = client.context("Codex MCP server is not initialized")?;

            match params {
                // todo: consider sending an empty request so we get the init response?
                AnyAgentRequest::InitializeParams(_) => Ok(AnyAgentResult::InitializeResponse(
                    acp::InitializeResponse {
                        is_authenticated: true,
                        protocol_version: ProtocolVersion::latest(),
                    },
                )),
                AnyAgentRequest::AuthenticateParams(_) => {
                    Err(anyhow!("Authentication not supported"))
                }
                AnyAgentRequest::SendUserMessageParams(message) => {
                    let (new_cancel_tx, cancel_rx) = oneshot::channel();
                    cancel_request_tx.borrow_mut().replace(new_cancel_tx);

                    client
                        .cancellable_request::<CallTool>(
                            CallToolParams {
                                name: "codex".into(),
                                arguments: Some(serde_json::to_value(CodexToolCallParam {
                                    prompt: message
                                        .chunks
                                        .into_iter()
                                        .filter_map(|chunk| match chunk {
                                            acp::UserMessageChunk::Text { text } => Some(text),
                                            acp::UserMessageChunk::Path { .. } => {
                                                // todo!
                                                None
                                            }
                                        })
                                        .collect(),
                                    cwd: root_dir,
                                })?),
                                meta: None,
                            },
                            cancel_rx,
                        )
                        .await?;

                    Ok(AnyAgentResult::SendUserMessageResponse(
                        acp::SendUserMessageResponse,
                    ))
                }
                AnyAgentRequest::CancelSendMessageParams(_) => {
                    if let Ok(mut borrow) = cancel_request_tx.try_borrow_mut() {
                        if let Some(cancel_tx) = borrow.take() {
                            cancel_tx.send(()).ok();
                        }
                    }

                    Ok(AnyAgentResult::CancelSendMessageResponse(
                        acp::CancelSendMessageResponse,
                    ))
                }
            }
        }
        .boxed_local()
    }
}

struct CodexAgentConnection {
    codex_mcp: Arc<context_server::ContextServer>,
    root_dir: PathBuf,
    cancel_request_tx: Rc<RefCell<Option<oneshot::Sender<()>>>>,
    _handler_task: Task<()>,
    _zed_mcp: ZedMcpServer,
}

impl CodexAgentConnection {
    async fn handle_acp_notification(
        delegate: &AcpClientDelegate,
        event: AcpNotification,
    ) -> Result<()> {
        match event {
            AcpNotification::AgentMessage(message) => {
                delegate
                    .stream_assistant_message_chunk(acp::StreamAssistantMessageChunkParams {
                        chunk: acp::AssistantMessageChunk::Text {
                            text: message.message,
                        },
                    })
                    .await?;
            }
            AcpNotification::AgentReasoning(message) => {
                delegate
                    .stream_assistant_message_chunk(acp::StreamAssistantMessageChunkParams {
                        chunk: acp::AssistantMessageChunk::Thought {
                            thought: message.text,
                        },
                    })
                    .await?
            }
            AcpNotification::Other => {}
        }

        Ok(())
    }
}

/// todo! use types from h2a crate when we have one

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CodexToolCallParam {
    pub prompt: String,
    pub cwd: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodexEvent {
    pub msg: AcpNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AcpNotification {
    AgentMessage(AgentMessageEvent),
    AgentReasoning(AgentReasoningEvent),
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessageEvent {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentReasoningEvent {
    pub text: String,
}
