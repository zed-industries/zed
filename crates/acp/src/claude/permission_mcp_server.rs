use anyhow::Result;
use context_server::{
    listener::McpServer,
    types::{
        CallToolParams, CallToolResponse, Implementation, InitializeParams, InitializeResponse,
        ListToolsResponse, ProtocolVersion, ServerCapabilities, Tool, ToolsCapabilities, requests,
    },
};
use gpui::{App, Task};
use serde_json::json;

use crate::claude::McpServerConfig;

pub struct PermissionMcpServer {
    server: McpServer,
}

pub const SERVER_NAME: &str = "zed";
pub const TOOL_NAME: &str = "request_confirmation";

impl PermissionMcpServer {
    pub fn new(cx: &App) -> Result<Self> {
        let mut mcp_server = McpServer::new(cx)?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);
        mcp_server.handle_request::<requests::ListTools>(Self::handle_list_tools);
        mcp_server.handle_request::<requests::CallTool>(Self::handle_call_tool);

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
                    name: "Zed MCP Server".into(),
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
                    // todo!
                    input_schema: json!({}),
                    description: None,
                    annotations: None,
                }],
                next_cursor: None,
                meta: None,
            })
        })
    }

    fn handle_call_tool(request: CallToolParams, cx: &App) -> Task<Result<CallToolResponse>> {
        dbg!(&request);
        cx.spawn(async move |_cx| {
            Ok(CallToolResponse {
                content: vec![],
                is_error: None,
                meta: None,
            })
        })
    }
}
