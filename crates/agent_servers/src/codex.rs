use collections::HashMap;
use context_server::types::CallToolParams;
use context_server::types::requests::CallTool;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use project::Project;
use settings::SettingsStore;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use agentic_coding_protocol::{self as acp, AnyAgentRequest, AnyAgentResult, ProtocolVersion};
use anyhow::{Context, Result, anyhow};
use futures::future::LocalBoxFuture;
use futures::{AsyncWriteExt, FutureExt};
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

            cx.new(|cx| {
                // todo! handle notifications
                let delegate = AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async());
                delegate_tx.send(Some(delegate.clone())).log_err();

                let connection = CodexAgentConnection {
                    root_dir,
                    codex_mcp_client,
                    _zed_mcp_server: zed_mcp_server,
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
        let client = self.codex_mcp_client.client();
        let root_dir = self.root_dir.clone();
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
                    client
                        .request::<CallTool>(CallToolParams {
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
                        })
                        .await?;

                    Ok(AnyAgentResult::SendUserMessageResponse(
                        acp::SendUserMessageResponse,
                    ))
                }
                AnyAgentRequest::CancelSendMessageParams(_) => Ok(
                    AnyAgentResult::CancelSendMessageResponse(acp::CancelSendMessageResponse),
                ),
            }
        }
        .boxed_local()
    }
}

struct CodexAgentConnection {
    codex_mcp_client: Arc<context_server::ContextServer>,
    root_dir: PathBuf,
    _zed_mcp_server: ZedMcpServer,
}

/// todo! use types from h2a crate when we have one

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CodexToolCallParam {
    pub prompt: String,
    pub cwd: PathBuf,
}
