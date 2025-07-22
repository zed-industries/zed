use std::{cell::RefCell, rc::Rc};

use acp_thread::AcpClientDelegate;
use agentic_coding_protocol::{self as acp, Client, ReadTextFileParams, WriteTextFileParams};
use anyhow::{Context, Result};
use collections::HashMap;
use context_server::{
    listener::McpServer,
    types::{
        CallToolParams, CallToolResponse, Implementation, InitializeParams, InitializeResponse,
        ListToolsResponse, ProtocolVersion, ServerCapabilities, Tool, ToolAnnotations,
        ToolResponseContent, ToolsCapabilities, requests,
    },
};
use gpui::{App, AsyncApp, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::debug_panic;

use crate::claude::{
    McpServerConfig,
    tools::{ClaudeTool, EditToolParams, ReadToolParams},
};

pub struct ClaudeMcpServer {
    server: McpServer,
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

impl ClaudeMcpServer {
    pub async fn new(
        delegate: watch::Receiver<Option<AcpClientDelegate>>,
        tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let mut mcp_server = McpServer::new(cx).await?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);
        mcp_server.handle_request::<requests::ListTools>(Self::handle_list_tools);
        mcp_server.handle_request::<requests::CallTool>(move |request, cx| {
            Self::handle_call_tool(request, delegate.clone(), tool_id_map.clone(), cx)
        });

        Ok(Self { server: mcp_server })
    }

    pub fn server_config(&self) -> Result<McpServerConfig> {
        let zed_path = std::env::current_exe()
            .context("finding current executable path for use in mcp_server")?
            .to_string_lossy()
            .to_string();

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
        mut delegate_watch: watch::Receiver<Option<AcpClientDelegate>>,
        tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
        cx: &App,
    ) -> Task<Result<CallToolResponse>> {
        cx.spawn(async move |cx| {
            let Some(delegate) = delegate_watch.recv().await? else {
                debug_panic!("Sent None delegate");
                anyhow::bail!("Server not available");
            };

            if request.name.as_str() == PERMISSION_TOOL {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                let result =
                    Self::handle_permissions_tool_call(input, delegate, tool_id_map, cx).await?;
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

                let content = Self::handle_read_tool_call(input, delegate, cx).await?;
                Ok(CallToolResponse {
                    content,
                    is_error: None,
                    meta: None,
                })
            } else if request.name.as_str() == EDIT_TOOL {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                Self::handle_edit_tool_call(input, delegate, cx).await?;
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
        params: ReadToolParams,
        delegate: AcpClientDelegate,
        cx: &AsyncApp,
    ) -> Task<Result<Vec<ToolResponseContent>>> {
        cx.foreground_executor().spawn(async move {
            let response = delegate
                .read_text_file(ReadTextFileParams {
                    path: params.abs_path,
                    line: params.offset,
                    limit: params.limit,
                })
                .await?;

            Ok(vec![ToolResponseContent::Text {
                text: response.content,
            }])
        })
    }

    fn handle_edit_tool_call(
        params: EditToolParams,
        delegate: AcpClientDelegate,
        cx: &AsyncApp,
    ) -> Task<Result<()>> {
        cx.foreground_executor().spawn(async move {
            let response = delegate
                .read_text_file_reusing_snapshot(ReadTextFileParams {
                    path: params.abs_path.clone(),
                    line: None,
                    limit: None,
                })
                .await?;

            let new_content = response.content.replace(&params.old_text, &params.new_text);
            if new_content == response.content {
                return Err(anyhow::anyhow!("The old_text was not found in the content"));
            }

            delegate
                .write_text_file(WriteTextFileParams {
                    path: params.abs_path,
                    content: new_content,
                })
                .await?;

            Ok(())
        })
    }

    fn handle_permissions_tool_call(
        params: PermissionToolParams,
        delegate: AcpClientDelegate,
        tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
        cx: &AsyncApp,
    ) -> Task<Result<PermissionToolResponse>> {
        cx.foreground_executor().spawn(async move {
            let claude_tool = ClaudeTool::infer(&params.tool_name, params.input.clone());

            let tool_call_id = match params.tool_use_id {
                Some(tool_use_id) => tool_id_map
                    .borrow()
                    .get(&tool_use_id)
                    .cloned()
                    .context("Tool call ID not found")?,

                None => delegate.push_tool_call(claude_tool.as_acp()).await?.id,
            };

            let outcome = delegate
                .request_existing_tool_call_confirmation(
                    tool_call_id,
                    claude_tool.confirmation(None),
                )
                .await?;

            match outcome {
                acp::ToolCallConfirmationOutcome::Allow
                | acp::ToolCallConfirmationOutcome::AlwaysAllow
                | acp::ToolCallConfirmationOutcome::AlwaysAllowMcpServer
                | acp::ToolCallConfirmationOutcome::AlwaysAllowTool => Ok(PermissionToolResponse {
                    behavior: PermissionToolBehavior::Allow,
                    updated_input: params.input,
                }),
                acp::ToolCallConfirmationOutcome::Reject
                | acp::ToolCallConfirmationOutcome::Cancel => Ok(PermissionToolResponse {
                    behavior: PermissionToolBehavior::Deny,
                    updated_input: params.input,
                }),
            }
        })
    }
}
