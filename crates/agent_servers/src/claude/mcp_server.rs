use std::path::PathBuf;

use acp_thread::AcpThread;
use agent_client_protocol as acp;
use anyhow::{Context, Result};
use collections::HashMap;
use context_server::types::{
    CallToolParams, CallToolResponse, Implementation, InitializeParams, InitializeResponse,
    ListToolsResponse, ProtocolVersion, ServerCapabilities, Tool, ToolAnnotations,
    ToolResponseContent, ToolsCapabilities, requests,
};
use gpui::{App, AsyncApp, Entity, Task, WeakEntity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::claude::tools::{ClaudeTool, EditToolParams, ReadToolParams};

pub struct ClaudeZedMcpServer {
    server: context_server::listener::McpServer,
}

pub const SERVER_NAME: &str = "zed";
pub const READ_TOOL: &str = "Read";
pub const EDIT_TOOL: &str = "Edit";
pub const PERMISSION_TOOL: &str = "Confirmation";

#[derive(Deserialize, JsonSchema, Debug)]
struct PermissionToolParams {
    tool_name: String,
    input: serde_json::Value,
    tool_use_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PermissionToolResponse {
    behavior: PermissionToolBehavior,
    updated_input: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum PermissionToolBehavior {
    Allow,
    Deny,
}

impl ClaudeZedMcpServer {
    pub async fn new(
        thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let mut mcp_server = context_server::listener::McpServer::new(cx).await?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);
        mcp_server.handle_request::<requests::ListTools>(Self::handle_list_tools);
        mcp_server.handle_request::<requests::CallTool>(move |request, cx| {
            Self::handle_call_tool(request, thread_rx.clone(), cx)
        });

        Ok(Self { server: mcp_server })
    }

    pub fn server_config(&self) -> Result<McpServerConfig> {
        let zed_path = std::env::current_exe()
            .context("finding current executable path for use in mcp_server")?;

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

    fn handle_list_tools(_: (), cx: &App) -> Task<Result<ListToolsResponse>> {
        cx.foreground_executor().spawn(async move {
            Ok(ListToolsResponse {
                tools: vec![
                    Tool {
                        name: PERMISSION_TOOL.into(),
                        input_schema: schemars::schema_for!(PermissionToolParams).into(),
                        description: None,
                        annotations: None,
                    },
                    Tool {
                        name: READ_TOOL.into(),
                        input_schema: schemars::schema_for!(ReadToolParams).into(),
                        description: Some("Read the contents of a file. In sessions with mcp__zed__Read always use it instead of Read as it contains the most up-to-date contents.".to_string()),
                        annotations: Some(ToolAnnotations {
                            title: Some("Read file".to_string()),
                            read_only_hint: Some(true),
                            destructive_hint: Some(false),
                            open_world_hint: Some(false),
                            // if time passes the contents might change, but it's not going to do anything different
                            // true or false seem too strong, let's try a none.
                            idempotent_hint: None,
                        }),
                    },
                    Tool {
                        name: EDIT_TOOL.into(),
                        input_schema: schemars::schema_for!(EditToolParams).into(),
                        description: Some("Edits a file. In sessions with mcp__zed__Edit always use it instead of Edit as it will show the diff to the user better.".to_string()),
                        annotations: Some(ToolAnnotations {
                            title: Some("Edit file".to_string()),
                            read_only_hint: Some(false),
                            destructive_hint: Some(false),
                            open_world_hint: Some(false),
                            idempotent_hint: Some(false),
                        }),
                    },
                ],
                next_cursor: None,
                meta: None,
            })
        })
    }

    fn handle_call_tool(
        request: CallToolParams,
        mut thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
        cx: &App,
    ) -> Task<Result<CallToolResponse>> {
        cx.spawn(async move |cx| {
            let Some(thread) = thread_rx.recv().await?.upgrade() else {
                anyhow::bail!("Thread closed");
            };

            if request.name.as_str() == PERMISSION_TOOL {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                let result = Self::handle_permissions_tool_call(input, thread, cx).await?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: serde_json::to_string(&result)?,
                    }],
                    is_error: None,
                    meta: None,
                })
            } else if request.name.as_str() == READ_TOOL {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                let content = Self::handle_read_tool_call(input, thread, cx).await?;
                Ok(CallToolResponse {
                    content,
                    is_error: None,
                    meta: None,
                })
            } else if request.name.as_str() == EDIT_TOOL {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                Self::handle_edit_tool_call(input, thread, cx).await?;
                Ok(CallToolResponse {
                    content: vec![],
                    is_error: None,
                    meta: None,
                })
            } else {
                anyhow::bail!("Unsupported tool");
            }
        })
    }

    fn handle_read_tool_call(
        ReadToolParams {
            abs_path,
            offset,
            limit,
        }: ReadToolParams,
        thread: Entity<AcpThread>,
        cx: &AsyncApp,
    ) -> Task<Result<Vec<ToolResponseContent>>> {
        cx.spawn(async move |cx| {
            let content = thread
                .update(cx, |thread, cx| {
                    thread.read_text_file(abs_path, offset, limit, false, cx)
                })?
                .await?;

            Ok(vec![ToolResponseContent::Text { text: content }])
        })
    }

    fn handle_edit_tool_call(
        params: EditToolParams,
        thread: Entity<AcpThread>,
        cx: &AsyncApp,
    ) -> Task<Result<()>> {
        cx.spawn(async move |cx| {
            let content = thread
                .update(cx, |threads, cx| {
                    threads.read_text_file(params.abs_path.clone(), None, None, true, cx)
                })?
                .await?;

            let new_content = content.replace(&params.old_text, &params.new_text);
            if new_content == content {
                return Err(anyhow::anyhow!("The old_text was not found in the content"));
            }

            thread
                .update(cx, |threads, cx| {
                    threads.write_text_file(params.abs_path, new_content, cx)
                })?
                .await?;

            Ok(())
        })
    }

    fn handle_permissions_tool_call(
        params: PermissionToolParams,
        thread: Entity<AcpThread>,
        cx: &AsyncApp,
    ) -> Task<Result<PermissionToolResponse>> {
        cx.spawn(async move |cx| {
            let claude_tool = ClaudeTool::infer(&params.tool_name, params.input.clone());

            let tool_call_id =
                acp::ToolCallId(params.tool_use_id.context("Tool ID required")?.into());

            let allow_option_id = acp::PermissionOptionId("allow".into());
            let reject_option_id = acp::PermissionOptionId("reject".into());

            let chosen_option = thread
                .update(cx, |thread, cx| {
                    thread.request_tool_call_permission(
                        claude_tool.as_acp(tool_call_id),
                        vec![
                            acp::PermissionOption {
                                id: allow_option_id.clone(),
                                label: "Allow".into(),
                                kind: acp::PermissionOptionKind::AllowOnce,
                            },
                            acp::PermissionOption {
                                id: reject_option_id,
                                label: "Reject".into(),
                                kind: acp::PermissionOptionKind::RejectOnce,
                            },
                        ],
                        cx,
                    )
                })?
                .await?;

            if chosen_option == allow_option_id {
                Ok(PermissionToolResponse {
                    behavior: PermissionToolBehavior::Allow,
                    updated_input: params.input,
                })
            } else {
                Ok(PermissionToolResponse {
                    behavior: PermissionToolBehavior::Deny,
                    updated_input: params.input,
                })
            }
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
