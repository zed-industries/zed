use anyhow::{Context, Result};
use context_server::{
    listener::McpServer,
    types::{
        CallToolParams, CallToolResponse, Implementation, InitializeParams, InitializeResponse,
        ListToolsResponse, ProtocolVersion, ServerCapabilities, Tool, ToolResponseContent,
        ToolsCapabilities, requests,
    },
};
use gpui::{App, AsyncApp, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AcpClientDelegate, claude::McpServerConfig};

pub struct PermissionMcpServer {
    server: McpServer,
}

pub const SERVER_NAME: &str = "zed";
pub const TOOL_NAME: &str = "request_confirmation";

#[derive(Deserialize, JsonSchema, Debug)]
struct PermissionToolInput {
    tool_name: String,
    input: serde_json::Value,
    tool_use_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PermissionToolOutput {
    behavior: PermissionToolBehavior,
    updated_input: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum PermissionToolBehavior {
    Allow,
    Deny,
}

impl PermissionMcpServer {
    pub fn new(cx: &App, delegate: AcpClientDelegate) -> Result<Self> {
        let mut mcp_server = McpServer::new(cx)?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);
        mcp_server.handle_request::<requests::ListTools>(Self::handle_list_tools);
        mcp_server.handle_request::<requests::CallTool>(move |request, cx| {
            Self::handle_call_tool(request, delegate.clone(), cx)
        });

        Ok(Self { server: mcp_server })
    }

    pub fn server_config(&self) -> Result<McpServerConfig> {
        let zed_path = util::get_shell_safe_zed_path()?;

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
        cx.spawn(async move |_cx| {
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
        cx.spawn(async move |_cx| {
            Ok(ListToolsResponse {
                tools: vec![Tool {
                    name: TOOL_NAME.into(),
                    input_schema: schemars::schema_for!(PermissionToolInput).into(),
                    description: None,
                    annotations: None,
                }],
                next_cursor: None,
                meta: None,
            })
        })
    }

    fn handle_call_tool(
        request: CallToolParams,
        delegate: AcpClientDelegate,
        cx: &App,
    ) -> Task<Result<CallToolResponse>> {
        cx.spawn(async move |cx| {
            if request.name.as_str() == TOOL_NAME {
                let input = serde_json::from_value::<PermissionToolInput>(
                    request.arguments.context("Arguments required")?,
                )?;

                let result = Self::handle_permissions_tool_call(input, delegate, cx).await?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: serde_json::to_string(&result)?,
                    }],
                    is_error: None,
                    meta: None,
                })
            } else {
                anyhow::bail!("Unsupported tool");
            }
        })
    }

    fn handle_permissions_tool_call(
        input: PermissionToolInput,
        delegate: AcpClientDelegate,
        cx: &AsyncApp,
    ) -> Task<Result<PermissionToolOutput>> {
        dbg!(&input);
        return Task::ready(Ok(PermissionToolOutput {
            behavior: PermissionToolBehavior::Allow,
            updated_input: input.input,
        }));
    }
}
