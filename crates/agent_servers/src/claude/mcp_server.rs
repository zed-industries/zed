use std::path::PathBuf;

use crate::claude::tools::{ClaudeTool, EditToolParams, ReadToolParams};
use acp_thread::AcpThread;
use agent_client_protocol as acp;
use anyhow::{Context, Result};
use collections::HashMap;
use context_server::listener::{McpServerTool, ToolResponse};
use context_server::types::{
    Implementation, InitializeParams, InitializeResponse, ProtocolVersion, ServerCapabilities,
    ToolAnnotations, ToolResponseContent, ToolsCapabilities, requests,
};
use gpui::{App, AsyncApp, Task, WeakEntity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub struct ClaudeZedMcpServer {
    server: context_server::listener::McpServer,
}

pub const SERVER_NAME: &str = "zed";

impl ClaudeZedMcpServer {
    pub async fn new(
        thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let mut mcp_server = context_server::listener::McpServer::new(cx).await?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);

        mcp_server.add_tool(PermissionTool {
            thread_rx: thread_rx.clone(),
        });
        mcp_server.add_tool(ReadTool {
            thread_rx: thread_rx.clone(),
        });
        mcp_server.add_tool(EditTool {
            thread_rx: thread_rx.clone(),
        });

        Ok(Self { server: mcp_server })
    }

    pub fn server_config(&self) -> Result<McpServerConfig> {
        #[cfg(not(test))]
        let zed_path = std::env::current_exe()
            .context("finding current executable path for use in mcp_server")?;

        #[cfg(test)]
        let zed_path = crate::e2e_tests::get_zed_path();

        Ok(McpServerConfig {
            command: zed_path,
            args: vec![
                "--nc".into(),
                self.server.socket_path().display().to_string(),
            ],
            env: None,
        })
    }

    fn handle_initialize(_: InitializeParams, cx: &App) -> Task<Result<InitializeResponse>> {
        cx.foreground_executor().spawn(async move {
            Ok(InitializeResponse {
                protocol_version: ProtocolVersion("2025-06-18".into()),
                capabilities: ServerCapabilities {
                    experimental: None,
                    logging: None,
                    completions: None,
                    prompts: None,
                    resources: None,
                    tools: Some(ToolsCapabilities {
                        list_changed: Some(false),
                    }),
                },
                server_info: Implementation {
                    name: SERVER_NAME.into(),
                    version: "0.1.0".into(),
                },
                meta: None,
            })
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpConfig {
    pub mcp_servers: HashMap<String, McpServerConfig>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct McpServerConfig {
    pub command: PathBuf,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

// Tools

#[derive(Clone)]
pub struct PermissionTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct PermissionToolParams {
    tool_name: String,
    input: serde_json::Value,
    tool_use_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionToolResponse {
    behavior: PermissionToolBehavior,
    updated_input: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum PermissionToolBehavior {
    Allow,
    Deny,
}

impl McpServerTool for PermissionTool {
    type Input = PermissionToolParams;
    type Output = ();

    const NAME: &'static str = "Confirmation";

    fn description(&self) -> &'static str {
        "Request permission for tool calls"
    }

    async fn run(
        &self,
        input: Self::Input,
        cx: &mut AsyncApp,
    ) -> Result<ToolResponse<Self::Output>> {
        let mut thread_rx = self.thread_rx.clone();
        let Some(thread) = thread_rx.recv().await?.upgrade() else {
            anyhow::bail!("Thread closed");
        };

        let claude_tool = ClaudeTool::infer(&input.tool_name, input.input.clone());
        let tool_call_id = acp::ToolCallId(input.tool_use_id.context("Tool ID required")?.into());
        let allow_option_id = acp::PermissionOptionId("allow".into());
        let reject_option_id = acp::PermissionOptionId("reject".into());

        let chosen_option = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    claude_tool.as_acp(tool_call_id),
                    vec![
                        acp::PermissionOption {
                            id: allow_option_id.clone(),
                            name: "Allow".into(),
                            kind: acp::PermissionOptionKind::AllowOnce,
                        },
                        acp::PermissionOption {
                            id: reject_option_id.clone(),
                            name: "Reject".into(),
                            kind: acp::PermissionOptionKind::RejectOnce,
                        },
                    ],
                    cx,
                )
            })?
            .await?;

        let response = if chosen_option == allow_option_id {
            PermissionToolResponse {
                behavior: PermissionToolBehavior::Allow,
                updated_input: input.input,
            }
        } else {
            debug_assert_eq!(chosen_option, reject_option_id);
            PermissionToolResponse {
                behavior: PermissionToolBehavior::Deny,
                updated_input: input.input,
            }
        };

        Ok(ToolResponse {
            content: vec![ToolResponseContent::Text {
                text: serde_json::to_string(&response)?,
            }],
            structured_content: (),
        })
    }
}

#[derive(Clone)]
pub struct ReadTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl McpServerTool for ReadTool {
    type Input = ReadToolParams;
    type Output = ();

    const NAME: &'static str = "Read";

    fn description(&self) -> &'static str {
        "Read the contents of a file. In sessions with mcp__zed__Read always use it instead of Read as it contains the most up-to-date contents."
    }

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Read file".to_string()),
            read_only_hint: Some(true),
            destructive_hint: Some(false),
            open_world_hint: Some(false),
            idempotent_hint: None,
        }
    }

    async fn run(
        &self,
        input: Self::Input,
        cx: &mut AsyncApp,
    ) -> Result<ToolResponse<Self::Output>> {
        let mut thread_rx = self.thread_rx.clone();
        let Some(thread) = thread_rx.recv().await?.upgrade() else {
            anyhow::bail!("Thread closed");
        };

        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(input.abs_path, input.offset, input.limit, false, cx)
            })?
            .await?;

        Ok(ToolResponse {
            content: vec![ToolResponseContent::Text { text: content }],
            structured_content: (),
        })
    }
}

#[derive(Clone)]
pub struct EditTool {
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

impl McpServerTool for EditTool {
    type Input = EditToolParams;
    type Output = ();

    const NAME: &'static str = "Edit";

    fn description(&self) -> &'static str {
        "Edits a file. In sessions with mcp__zed__Edit always use it instead of Edit as it will show the diff to the user better."
    }

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Edit file".to_string()),
            read_only_hint: Some(false),
            destructive_hint: Some(false),
            open_world_hint: Some(false),
            idempotent_hint: Some(false),
        }
    }

    async fn run(
        &self,
        input: Self::Input,
        cx: &mut AsyncApp,
    ) -> Result<ToolResponse<Self::Output>> {
        let mut thread_rx = self.thread_rx.clone();
        let Some(thread) = thread_rx.recv().await?.upgrade() else {
            anyhow::bail!("Thread closed");
        };

        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(input.abs_path.clone(), None, None, true, cx)
            })?
            .await?;

        let new_content = content.replace(&input.old_text, &input.new_text);
        if new_content == content {
            return Err(anyhow::anyhow!("The old_text was not found in the content"));
        }

        thread
            .update(cx, |thread, cx| {
                thread.write_text_file(input.abs_path, new_content, cx)
            })?
            .await?;

        Ok(ToolResponse {
            content: vec![],
            structured_content: (),
        })
    }
}
