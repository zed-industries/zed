// crates/context_server/src/rmcp_adapter.rs

use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// Correct RMCP imports based on the API guide
use rmcp::{
    ServiceExt,
    model::{
        CallToolRequestParam, ClientCapabilities, Implementation, ListPromptsRequestParam,
        ListResourcesRequestParam, ListToolsRequestParam, Prompt, Resource, Tool,
    },
    transport::{ConfigureCommandExt, HttpClientTransport, TokioChildProcess},
};

use tokio::process::Command;

use crate::client::{ContextServerPrompt, ContextServerTool, PromptArgument};
use crate::protocol;

static CONTEXT_SERVER_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Adapter to bridge between Zed's existing MCP implementation and the official RMCP SDK
pub struct RmcpAdapter {
    client: Option<Box<dyn ServiceExt>>,
    transport_type: TransportType,
    server_info: Option<protocol::InitializeResponse>,
    capabilities: Option<protocol::ServerCapabilities>,
}

#[derive(Debug, Clone)]
pub enum TransportType {
    Stdio {
        command: String,
        args: Vec<String>,
    },
    Http {
        url: String,
        headers: HashMap<String, String>,
    },
    Sse {
        url: String,
        headers: HashMap<String, String>,
    },
}

impl RmcpAdapter {
    /// Create a new RMCP adapter with stdio transport (backward compatible)
    pub async fn new_stdio(command: impl Into<String>, args: Vec<String>) -> Result<Self> {
        let command = command.into();

        // Create the transport using TokioChildProcess as shown in the guide
        let transport = TokioChildProcess::new(Command::new(&command).configure(|cmd| {
            for arg in &args {
                cmd.arg(arg);
            }
        }))?;

        // Create client info
        let client_info = Implementation {
            name: "zed".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        // Initialize the client
        let client = client_info.serve(transport).await?;

        Ok(Self {
            client: Some(Box::new(client)),
            transport_type: TransportType::Stdio { command, args },
            server_info: None,
            capabilities: None,
        })
    }

    /// Create a new RMCP adapter with HTTP transport (new capability)
    pub async fn new_http(
        url: impl Into<String>,
        headers: HashMap<String, String>,
    ) -> Result<Self> {
        let url = url.into();

        // Create HTTP transport as shown in the guide
        let mut transport = HttpClientTransport::new(&url)?;

        // Add headers if provided
        for (key, value) in &headers {
            transport.add_header(key, value)?;
        }

        // Create client info
        let client_info = Implementation {
            name: "zed".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        // Initialize the client
        let client = client_info.serve(transport).await?;

        Ok(Self {
            client: Some(Box::new(client)),
            transport_type: TransportType::Http { url, headers },
            server_info: None,
            capabilities: None,
        })
    }

    /// Create adapter from a transport configuration
    pub async fn from_config(config: &TransportConfig) -> Result<Self> {
        match config {
            TransportConfig::Stdio { command, args } => {
                Self::new_stdio(command, args.clone()).await
            }
            TransportConfig::Http { url, headers } => Self::new_http(url, headers.clone()).await,
            TransportConfig::Sse { url, headers } => {
                // SSE can use HTTP transport with streaming support
                Self::new_http(url, headers.clone()).await
            }
        }
    }

    /// Initialize the connection with the MCP server
    pub async fn initialize(&mut self, client_info: protocol::ClientInfo) -> Result<()> {
        // The client is already initialized when created via serve()
        // We just need to store server info if needed

        // Note: The RMCP SDK handles initialization internally
        // We might need to call specific methods to get server info

        Ok(())
    }

    /// List available tools from the server
    pub async fn list_tools(&self) -> Result<Vec<ContextServerTool>> {
        if let Some(client) = &self.client {
            let response = client
                .list_tools(ListToolsRequestParam { cursor: None })
                .await?;

            Ok(response.tools.into_iter().map(convert_tool).collect())
        } else {
            Err(anyhow!("Not initialized"))
        }
    }

    /// List available prompts from the server
    pub async fn list_prompts(&self) -> Result<Vec<ContextServerPrompt>> {
        if let Some(client) = &self.client {
            let response = client
                .list_prompts(ListPromptsRequestParam { cursor: None })
                .await?;

            Ok(response.prompts.into_iter().map(convert_prompt).collect())
        } else {
            Err(anyhow!("Not initialized"))
        }
    }

    /// List available resources from the server
    pub async fn list_resources(&self) -> Result<Vec<Value>> {
        if let Some(client) = &self.client {
            let response = client
                .list_resources(ListResourcesRequestParam { cursor: None })
                .await?;

            // Convert resources to JSON values
            Ok(response
                .resources
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
                .collect())
        } else {
            Err(anyhow!("Not initialized"))
        }
    }

    /// Call a tool on the server
    pub async fn call_tool(
        &self,
        name: impl Into<String>,
        arguments: Option<Value>,
    ) -> Result<Value> {
        if let Some(client) = &self.client {
            let response = client
                .call_tool(CallToolRequestParam {
                    name: name.into(),
                    arguments,
                })
                .await?;

            // Convert response to JSON value
            serde_json::to_value(response)
                .map_err(|e| anyhow!("Failed to serialize response: {}", e))
        } else {
            Err(anyhow!("Not initialized"))
        }
    }

    /// Check if the adapter is connected
    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    /// Get server information
    pub fn server_info(&self) -> Option<&protocol::InitializeResponse> {
        self.server_info.as_ref()
    }

    /// Get server capabilities
    pub fn capabilities(&self) -> Option<&protocol::ServerCapabilities> {
        self.capabilities.as_ref()
    }

    /// Shutdown the connection
    pub async fn shutdown(&mut self) -> Result<()> {
        // Take the client to drop it
        self.client.take();
        Ok(())
    }
}

// Configuration structure for transport
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum TransportConfig {
    #[serde(rename = "stdio")]
    Stdio {
        command: String,
        #[serde(default)]
        args: Vec<String>,
    },
    #[serde(rename = "http")]
    Http {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
    #[serde(rename = "sse")]
    Sse {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
}

// Helper functions to convert between RMCP types and Zed types
fn convert_tool(tool: Tool) -> ContextServerTool {
    ContextServerTool {
        name: tool.name,
        description: tool.description.unwrap_or_default(),
        input_schema: tool.input_schema,
    }
}

fn convert_prompt(prompt: Prompt) -> ContextServerPrompt {
    ContextServerPrompt {
        name: prompt.name,
        description: prompt.description.unwrap_or_default(),
        arguments: prompt.arguments.map(|args| {
            args.into_iter()
                .map(|arg| PromptArgument {
                    name: arg.name,
                    description: arg.description.unwrap_or_default(),
                    required: arg.required.unwrap_or(false),
                })
                .collect()
        }),
    }
}

fn generate_request_id() -> String {
    let id = CONTEXT_SERVER_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("zed_rmcp_{}", id)
}
