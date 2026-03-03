use std::time::Duration;

use context_server::listener::McpServer;
use context_server::types::{
    Implementation, InitializeResponse, ProtocolVersion, ServerCapabilities, ToolsCapabilities,
    requests::Initialize,
    LATEST_PROTOCOL_VERSION,
};
use database_core::{connection_count, set_mcp_socket_path};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Task, WeakEntity};
use util::ResultExt;

use crate::mcp_tools::{
    McpDescribeObject, McpExecuteQuery, McpExplainQuery, McpGetSchema, McpListObjects,
    McpModifyData,
};

pub struct DatabaseMcpServerManager {
    server: Option<McpServer>,
    _poll_task: Task<()>,
}

impl DatabaseMcpServerManager {
    pub fn start(cx: &mut App) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let poll_task = cx.spawn(async move |this, mut cx| {
                let mut was_active = false;
                loop {
                    cx.background_executor()
                        .timer(Duration::from_secs(2))
                        .await;
                    let count = connection_count();
                    let is_active = count > 0;

                    if is_active && !was_active {
                        Self::start_server(&this, &mut cx).await;
                    } else if !is_active && was_active {
                        log::info!("database_mcp: no active connections, stopping MCP server");
                        set_mcp_socket_path(None);
                        this.update(cx, |this, _cx| {
                            this.server = None;
                        })
                        .log_err();
                    }
                    was_active = is_active;
                }
            });
            Self {
                server: None,
                _poll_task: poll_task,
            }
        })
    }

    async fn start_server(this: &WeakEntity<Self>, cx: &mut AsyncApp) {
        let server_result = McpServer::new(cx).await;
        match server_result {
            Ok(mut server) => {
                server.handle_request::<Initialize>(|_params, _cx| {
                    Task::ready(Ok(InitializeResponse {
                        protocol_version: ProtocolVersion(
                            LATEST_PROTOCOL_VERSION.to_string(),
                        ),
                        capabilities: ServerCapabilities {
                            tools: Some(ToolsCapabilities {
                                list_changed: Some(false),
                            }),
                            ..Default::default()
                        },
                        server_info: Implementation {
                            name: "zed-database".into(),
                            version: "0.1.0".into(),
                        },
                        meta: None,
                    }))
                });

                server.add_tool(McpExecuteQuery);
                server.add_tool(McpDescribeObject);
                server.add_tool(McpListObjects);
                server.add_tool(McpExplainQuery);
                server.add_tool(McpModifyData);
                server.add_tool(McpGetSchema);

                let path = server.socket_path().to_path_buf();

                let stored = this
                    .update(cx, |this, _cx| {
                        this.server = Some(server);
                    })
                    .is_ok();

                if stored {
                    log::info!("database_mcp: MCP server started at {}", path.display());
                    set_mcp_socket_path(Some(path));
                } else {
                    log::error!("database_mcp: failed to store MCP server, entity dropped");
                }
            }
            Err(error) => {
                log::error!("database_mcp: failed to start MCP server: {error}");
            }
        }
    }
}
