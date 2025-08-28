use std::path::PathBuf;
use std::sync::Arc;

use crate::claude::edit_tool::EditTool;
use crate::claude::permission_tool::PermissionTool;
use crate::claude::read_tool::ReadTool;
use crate::claude::write_tool::WriteTool;
use acp_thread::AcpThread;
#[cfg(not(test))]
use anyhow::Context as _;
use anyhow::Result;
use collections::HashMap;
use context_server::types::{
    Implementation, InitializeParams, InitializeResponse, ProtocolVersion, ServerCapabilities,
    ToolsCapabilities, requests,
};
use gpui::{App, AsyncApp, Task, WeakEntity};
use project::Fs;
use serde::Serialize;

pub struct ClaudeZedMcpServer {
    server: context_server::listener::McpServer,
}

pub const SERVER_NAME: &str = "zed";

impl ClaudeZedMcpServer {
    pub async fn new(
        thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
        fs: Arc<dyn Fs>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let mut mcp_server = context_server::listener::McpServer::new(cx).await?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);

        mcp_server.add_tool(PermissionTool::new(fs.clone(), thread_rx.clone()));
        mcp_server.add_tool(ReadTool::new(thread_rx.clone()));
        mcp_server.add_tool(EditTool::new(thread_rx.clone()));
        mcp_server.add_tool(WriteTool::new(thread_rx.clone()));

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
