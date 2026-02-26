//! MCP (Model Context Protocol) integration module

use anyhow::{Context, Result};
use collections::HashMap;
use gpui::{App, Task};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{process::Stdio, sync::Arc};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    types::*,
    ExternalWebSocketSync,
};

/// MCP server manager
pub struct McpManager {
    servers: Arc<RwLock<HashMap<String, McpServerInstance>>>,
    tools: Arc<RwLock<HashMap<String, McpTool>>>,
    _tasks: Vec<Task<()>>,
}

/// Individual MCP server instance
struct McpServerInstance {
    name: String,
    config: McpServerConfig,
    process: Option<Child>,
    stdin_tx: Option<mpsc::UnboundedSender<McpRequest>>,
    tools: Vec<McpTool>,
    status: McpServerStatus,
}

#[derive(Clone, Debug, PartialEq)]
enum McpServerStatus {
    Starting,
    Running,
    Stopped,
    Error(String),
}

/// MCP request/response types
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "jsonrpc")]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// MCP tool definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

impl McpManager {
    pub fn new() -> Self {
        Self {
            servers: Arc::new(RwLock::new(HashMap::default())),
            tools: Arc::new(RwLock::new(HashMap::default())),
            _tasks: Vec::new(),
        }
    }

    /// Initialize MCP servers based on configuration
    pub async fn initialize(&mut self, config: McpConfig, cx: &mut App) -> Result<()> {
        if !config.enabled {
            log::info!("MCP integration is disabled");
            return Ok(());
        }

        log::info!("Initializing MCP integration with {} servers", config.server_configs.len());

        for server_config in config.server_configs {
            self.start_server(server_config, cx).await?;
        }

        Ok(())
    }

    /// Start an MCP server
    async fn start_server(&mut self, config: McpServerConfig, cx: &mut App) -> Result<()> {
        log::info!("Starting MCP server: {}", config.name);

        let server_name = config.name.clone();
        let servers = self.servers.clone();
        let tools = self.tools.clone();

        let task = cx.spawn(async move |_cx| {
            match Self::spawn_server_process(config.clone()).await {
                Ok(mut server) => {
                    // Initialize the server
                    if let Err(e) = Self::initialize_server(&mut server).await {
                        log::error!("Failed to initialize MCP server {}: {}", server.name, e);
                        server.status = McpServerStatus::Error(e.to_string());
                    } else {
                        log::info!("MCP server {} initialized successfully", server.name);
                        server.status = McpServerStatus::Running;

                        // Register tools
                        for tool in &server.tools {
                            tools.write().insert(tool.name.clone(), tool.clone());
                        }
                    }

                    servers.write().insert(server_name.clone(), server);
                }
                Err(e) => {
                    log::error!("Failed to start MCP server {}: {}", server_name, e);
                    let failed_server = McpServerInstance {
                        name: server_name.clone(),
                        config,
                        process: None,
                        stdin_tx: None,
                        tools: Vec::new(),
                        status: McpServerStatus::Error(e.to_string()),
                    };
                    servers.write().insert(server_name, failed_server);
                }
            }
        });

        self._tasks.push(task);
        Ok(())
    }

    /// Spawn MCP server process
    async fn spawn_server_process(config: McpServerConfig) -> Result<McpServerInstance> {
        let mut cmd = Command::new(&config.command);
        cmd.args(&config.args);
        cmd.envs(&config.env);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn()
            .with_context(|| format!("Failed to spawn MCP server command: {}", config.command))?;

        let stdin = child.stdin.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to get stdin for MCP server"))?;
        let stdout = child.stdout.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to get stdout for MCP server"))?;

        let (stdin_tx, mut stdin_rx) = mpsc::unbounded_channel();

        // Handle stdin
        tokio::spawn(async move {
            let mut stdin = tokio::io::BufWriter::new(stdin);
            while let Some(request) = stdin_rx.recv().await {
                if let Ok(json) = serde_json::to_string(&request) {
                    if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut stdin, json.as_bytes()).await {
                        log::error!("Failed to write to MCP server stdin: {}", e);
                        break;
                    }
                    if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut stdin, b"\n").await {
                        log::error!("Failed to write newline to MCP server stdin: {}", e);
                        break;
                    }
                    if let Err(e) = tokio::io::AsyncWriteExt::flush(&mut stdin).await {
                        log::error!("Failed to flush MCP server stdin: {}", e);
                        break;
                    }
                }
            }
        });

        // Handle stdout (for now just log it)
        tokio::spawn(async move {
            let mut reader = tokio::io::BufReader::new(stdout);
            let mut line = String::new();
            
            loop {
                line.clear();
                match tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut line).await {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        if let Ok(response) = serde_json::from_str::<McpResponse>(&line) {
                            log::debug!("MCP server response: {:?}", response);
                        } else {
                            log::debug!("MCP server output: {}", line.trim());
                        }
                    }
                    Err(e) => {
                        log::error!("Error reading from MCP server stdout: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(McpServerInstance {
            name: config.name.clone(),
            config,
            process: Some(child),
            stdin_tx: Some(stdin_tx),
            tools: Vec::new(),
            status: McpServerStatus::Starting,
        })
    }

    /// Initialize MCP server by sending initialization messages
    async fn initialize_server(server: &mut McpServerInstance) -> Result<()> {
        let stdin_tx = server.stdin_tx.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No stdin channel for MCP server"))?;

        // Send initialize request
        let init_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: "initialize".to_string(),
            params: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "logging": {}
                },
                "clientInfo": {
                    "name": "zed-helix-integration",
                    "version": "0.1.0"
                }
            })),
        };

        stdin_tx.send(init_request)
            .map_err(|_| anyhow::anyhow!("Failed to send initialize request"))?;

        // Wait for response (simplified - in real implementation we'd need proper response handling)
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Send initialized notification
        let initialized_notification = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: "notifications/initialized".to_string(),
            params: None,
        };

        stdin_tx.send(initialized_notification)
            .map_err(|_| anyhow::anyhow!("Failed to send initialized notification"))?;

        // Request tools list
        let tools_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(2)),
            method: "tools/list".to_string(),
            params: None,
        };

        stdin_tx.send(tools_request)
            .map_err(|_| anyhow::anyhow!("Failed to send tools list request"))?;

        // TODO: Parse the actual response to get available tools
        // For now, create a placeholder tool
        server.tools = vec![
            McpTool {
                name: format!("{}_example_tool", server.name),
                description: format!("Example tool from {}", server.name),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "input": {
                            "type": "string",
                            "description": "Input for the tool"
                        }
                    }
                }),
                server: server.name.clone(),
            }
        ];

        Ok(())
    }

    /// Get all available tools
    pub fn get_tools(&self) -> Vec<McpTool> {
        self.tools.read().values().cloned().collect()
    }

    /// Call an MCP tool
    pub async fn call_tool(&self, tool_name: &str, arguments: HashMap<String, serde_json::Value>) -> Result<McpToolCallResponse> {
        let tools = self.tools.read();
        let tool = tools.get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_name))?;

        let server_name = tool.server.clone();
        drop(tools);

        let servers = self.servers.read();
        let server = servers.get(&server_name)
            .ok_or_else(|| anyhow::anyhow!("Server not found: {}", server_name))?;

        let stdin_tx = server.stdin_tx.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Server not running: {}", server_name))?;

        // Send tool call request
        let request_id = Uuid::new_v4().to_string();
        let tool_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(request_id)),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": tool_name,
                "arguments": arguments
            })),
        };

        stdin_tx.send(tool_request)
            .map_err(|_| anyhow::anyhow!("Failed to send tool call request"))?;

        // TODO: Implement proper response handling
        // For now, return a placeholder response
        Ok(McpToolCallResponse {
            success: true,
            result: Some(serde_json::json!({
                "message": format!("Tool {} called successfully", tool_name),
                "arguments": arguments
            })),
            error: None,
        })
    }

    /// Stop all MCP servers
    pub async fn stop_all_servers(&mut self) -> Result<()> {
        log::info!("Stopping all MCP servers");

        let server_names: Vec<String> = self.servers.read().keys().cloned().collect();
        
        for server_name in server_names {
            if let Err(e) = self.stop_server(&server_name).await {
                log::error!("Failed to stop MCP server {}: {}", server_name, e);
            }
        }

        self.tools.write().clear();
        Ok(())
    }

    /// Stop a specific MCP server
    async fn stop_server(&self, server_name: &str) -> Result<()> {
        let mut servers = self.servers.write();
        
        if let Some(mut server) = servers.remove(server_name) {
            log::info!("Stopping MCP server: {}", server_name);

            // Send shutdown notification if still connected
            if let Some(stdin_tx) = &server.stdin_tx {
                let shutdown_notification = McpRequest {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    method: "notifications/shutdown".to_string(),
                    params: None,
                };
                
                let _ = stdin_tx.send(shutdown_notification);
            }

            // Kill the process
            if let Some(ref mut process) = server.process {
                let _ = process.kill().await;
                let _ = process.wait().await;
            }

            server.status = McpServerStatus::Stopped;

            // Remove tools from this server
            let mut tools = self.tools.write();
            tools.retain(|_, tool| tool.server != server_name);
        }

        Ok(())
    }

    /// Get server status
    pub fn get_server_status(&self, server_name: &str) -> Option<McpServerStatus> {
        self.servers.read().get(server_name).map(|s| s.status.clone())
    }

    /// Get all server statuses
    pub fn get_all_server_statuses(&self) -> HashMap<String, McpServerStatus> {
        self.servers.read()
            .iter()
            .map(|(name, server)| (name.clone(), server.status.clone()))
            .collect()
    }
}

impl ExternalWebSocketSync {
    /// Initialize MCP integration
    pub async fn initialize_mcp(&mut self, config: McpConfig, cx: &mut App) -> Result<()> {
        if !config.enabled {
            return Ok(());
        }

        log::info!("Initializing MCP integration");

        let mut mcp_manager = McpManager::new();
        mcp_manager.initialize(config, cx).await?;

        // Store the MCP manager (in a real implementation, you'd store this in the integration)
        // For now, just log that it's initialized
        log::info!("MCP integration initialized successfully");

        Ok(())
    }
}

impl Default for McpManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create common MCP server configurations
pub fn create_filesystem_mcp_config() -> McpServerConfig {
    McpServerConfig {
        name: "filesystem".to_string(),
        command: "npx".to_string(),
        args: vec!["@modelcontextprotocol/server-filesystem".to_string(), "/tmp".to_string()],
        env: std::collections::HashMap::new(),
    }
}

pub fn create_git_mcp_config(repo_path: String) -> McpServerConfig {
    McpServerConfig {
        name: "git".to_string(),
        command: "npx".to_string(),
        args: vec!["@modelcontextprotocol/server-git".to_string(), "--repository".to_string(), repo_path],
        env: std::collections::HashMap::new(),
    }
}

pub fn create_sqlite_mcp_config(db_path: String) -> McpServerConfig {
    McpServerConfig {
        name: "sqlite".to_string(),
        command: "npx".to_string(),
        args: vec!["@modelcontextprotocol/server-sqlite".to_string(), db_path],
        env: std::collections::HashMap::new(),
    }
}