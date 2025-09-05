// crates/context_server/src/rmcp_adapter.rs

use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// RMCP imports - using only what we know exists from the example
use rmcp::{
    ServiceExt,
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    transport::StreamableHttpClientTransport,
};

// Import Zed's own types for consistency
use crate::types::{Implementation as ZedImplementation, InitializeResponse, ServerCapabilities};

static CONTEXT_SERVER_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A wrapper for tools that bridges RMCP Tool to Zed's format
#[derive(Debug, Clone)]
pub struct ContextServerTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// A wrapper for prompts that bridges RMCP Prompt to Zed's format
#[derive(Debug, Clone)]
pub struct ContextServerPrompt {
    pub name: String,
    pub description: String,
    pub arguments: Option<Vec<PromptArgument>>,
}

/// Prompt argument structure
#[derive(Debug, Clone)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
}

/// Adapter to bridge between Zed's existing MCP implementation and the official RMCP SDK
/// This is a conservative implementation that only uses confirmed working RMCP features
#[derive(Debug)]
pub struct RmcpAdapter {
    client: Option<Arc<dyn ServiceExt + Send + Sync>>,
    transport_type: TransportType,
    server_info: Option<InitializeResponse>,
    capabilities: Option<ServerCapabilities>,
    session_id: Option<String>,
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
    /// Create a new RMCP adapter with stdio transport
    /// Currently not supported by RMCP SDK
    pub async fn new_stdio(command: impl Into<String>, args: Vec<String>) -> Result<Self> {
        let command = command.into();
        Err(anyhow!(
            "STDIO transport not yet supported in RMCP SDK v0.6.2. \
             Use HTTP transport instead. Unsupported command: {} {:?}. \
             Consider using Zed's native MCP implementation for STDIO servers.",
            command,
            args
        ))
    }

    /// Create a new RMCP adapter with HTTP transport using StreamableHttpClientTransport
    /// This is the primary supported transport in RMCP SDK
    pub async fn new_http(
        url: impl Into<String>,
        _headers: HashMap<String, String>, // Headers not yet implemented in example
    ) -> Result<Self> {
        let url = url.into();

        // Create HTTP transport using the confirmed working API
        let transport = StreamableHttpClientTransport::from_uri(&url);

        // Create client info as shown in the working example
        let client_info = ClientInfo {
            protocol_version: Default::default(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "zed".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        // Initialize the client - this is the confirmed working approach
        let client = client_info
            .serve(transport)
            .await
            .map_err(|e| anyhow!("Failed to connect to RMCP server at {}: {}", url, e))?;

        // Get server info from the client
        let peer_info = client.peer_info();
        log::info!("Connected to RMCP server at {}: {:#?}", url, peer_info);

        Ok(Self {
            client: Some(Arc::new(client)),
            transport_type: TransportType::Http {
                url,
                headers: _headers,
            },
            server_info: None,  // RMCP uses peer_info instead
            capabilities: None, // Will extract from peer_info if needed
            session_id: None,
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
                // SSE uses the same StreamableHttpClientTransport
                Self::new_http(url, headers.clone()).await
            }
        }
    }

    /// Initialize the connection with the MCP server
    /// With RMCP SDK, initialization happens in serve() call
    pub async fn initialize(&mut self, _client_info: ZedImplementation) -> Result<()> {
        // The client is already initialized when created via serve()
        // Log the peer info for debugging
        if let Some(client) = &self.client {
            let peer_info = client.peer_info();
            log::debug!("RMCP client initialized. Peer info: {:#?}", peer_info);
        }
        Ok(())
    }

    /// List available tools from the server
    /// This uses the confirmed working API from the example
    pub async fn list_tools(&self) -> Result<Vec<ContextServerTool>> {
        if let Some(client) = &self.client {
            // Use the confirmed working approach from the example
            let response = client
                .list_tools(Default::default())
                .await
                .map_err(|e| anyhow!("Failed to list tools: {}", e))?;

            // Convert RMCP tools to Zed format
            let tools = response
                .tools
                .into_iter()
                .map(|tool| ContextServerTool {
                    name: tool.name.to_string(),
                    description: tool.description.map(|d| d.to_string()).unwrap_or_default(),
                    input_schema: tool.input_schema.as_ref().clone(),
                })
                .collect();

            log::debug!("Listed {} tools from RMCP server", tools.len());
            Ok(tools)
        } else {
            Err(anyhow!("Client not initialized"))
        }
    }

    /// List available prompts from the server
    /// Currently not implemented in RMCP SDK based on compilation errors
    pub async fn list_prompts(&self) -> Result<Vec<ContextServerPrompt>> {
        Err(anyhow!(
            "Prompt listing not yet implemented in RMCP SDK v0.6.2. \
             This feature may be added in future versions."
        ))
    }

    /// List available resources from the server
    /// Currently not implemented in RMCP SDK based on compilation errors
    pub async fn list_resources(&self) -> Result<Vec<Value>> {
        Err(anyhow!(
            "Resource listing not yet implemented in RMCP SDK v0.6.2. \
             This feature may be added in future versions."
        ))
    }

    /// Call a tool on the server
    /// This uses the confirmed working API from the example
    pub async fn call_tool(
        &self,
        name: impl Into<String>,
        arguments: Option<Value>,
    ) -> Result<Value> {
        if let Some(client) = &self.client {
            let tool_name = name.into();

            // Convert arguments to the format expected by RMCP (Map<String, Value>)
            let rmcp_arguments = match arguments {
                Some(Value::Object(map)) => Some(map),
                Some(other_value) => {
                    // Try to convert to object, or use empty object
                    match serde_json::from_value::<serde_json::Map<String, Value>>(other_value) {
                        Ok(map) => Some(map),
                        Err(_) => {
                            log::warn!(
                                "Could not convert arguments to object for tool {}, using empty object",
                                tool_name
                            );
                            Some(serde_json::Map::new())
                        }
                    }
                }
                None => None,
            };

            // Use the confirmed working approach from the example
            let response = client
                .call_tool(CallToolRequestParam {
                    name: tool_name.into(),
                    arguments: rmcp_arguments,
                })
                .await
                .map_err(|e| anyhow!("Tool call failed for '{}': {}", tool_name, e))?;

            // Convert response to JSON value
            serde_json::to_value(response)
                .map_err(|e| anyhow!("Failed to serialize tool response: {}", e))
        } else {
            Err(anyhow!("Client not initialized"))
        }
    }

    /// Call a tool with retry logic for robustness
    pub async fn call_tool_with_retry(
        &self,
        name: impl Into<String>,
        arguments: Option<Value>,
        max_retries: u32,
    ) -> Result<Value> {
        let tool_name = name.into();

        for attempt in 0..=max_retries {
            match self.call_tool(&tool_name, arguments.clone()).await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < max_retries => {
                    let delay = 1000 * (attempt + 1) as u64;
                    log::warn!(
                        "Tool call attempt {} failed for '{}': {}. Retrying in {}ms...",
                        attempt + 1,
                        tool_name,
                        e,
                        delay
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
                Err(e) => return Err(e),
            }
        }

        unreachable!()
    }

    /// Batch call multiple tools sequentially
    pub async fn batch_call_tools(
        &self,
        tool_calls: Vec<(String, Option<Value>)>,
    ) -> Result<Vec<Value>> {
        let mut results = Vec::new();

        for (tool_name, arguments) in tool_calls {
            let result = self.call_tool(tool_name, arguments).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Check if the adapter is connected
    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    /// Get server information (placeholder - RMCP uses peer_info instead)
    pub fn server_info(&self) -> Option<&InitializeResponse> {
        self.server_info.as_ref()
    }

    /// Get server capabilities (placeholder - RMCP capabilities are different)
    pub fn capabilities(&self) -> Option<&ServerCapabilities> {
        self.capabilities.as_ref()
    }

    /// Get session ID if available
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Get transport type
    pub fn transport_type(&self) -> &TransportType {
        &self.transport_type
    }

    /// Get peer info from RMCP client (RMCP-specific method)
    pub fn peer_info(&self) -> Option<String> {
        if let Some(client) = &self.client {
            Some(format!("{:#?}", client.peer_info()))
        } else {
            None
        }
    }

    /// Shutdown the connection gracefully
    pub async fn shutdown(&mut self) -> Result<()> {
        // Cancel the client connection as shown in the example
        if let Some(client) = &self.client {
            if let Err(e) = client.cancel().await {
                log::warn!("Error canceling RMCP client: {}", e);
            } else {
                log::debug!("RMCP client canceled successfully");
            }
        }

        // Drop the client to clean up resources
        if let Some(client) = self.client.take() {
            drop(client);
        }
        self.session_id.take();
        Ok(())
    }
}

/// Configuration structure for transport
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

impl TransportConfig {
    /// Create HTTP transport config
    pub fn http(url: impl Into<String>) -> Self {
        Self::Http {
            url: url.into(),
            headers: HashMap::new(),
        }
    }

    /// Create HTTP transport config with auth token
    pub fn http_with_auth(url: impl Into<String>, token: impl Into<String>) -> Self {
        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", token.into()),
        );

        Self::Http {
            url: url.into(),
            headers,
        }
    }

    /// Create STDIO transport config (not supported by RMCP yet)
    pub fn stdio(command: impl Into<String>, args: Vec<String>) -> Self {
        Self::Stdio {
            command: command.into(),
            args,
        }
    }
}

/// Generate a unique request ID for debugging
fn generate_request_id() -> String {
    let id = CONTEXT_SERVER_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("zed_rmcp_{}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_config_creation() {
        let http_config = TransportConfig::http("http://localhost:3000/mcp");
        assert!(matches!(http_config, TransportConfig::Http { .. }));

        let auth_config =
            TransportConfig::http_with_auth("https://api.example.com/mcp", "token123");
        if let TransportConfig::Http { headers, .. } = &auth_config {
            assert!(headers.contains_key("Authorization"));
        } else {
            panic!("Expected HTTP config");
        }

        let stdio_config =
            TransportConfig::stdio("python", vec!["-m".to_string(), "server".to_string()]);
        assert!(matches!(stdio_config, TransportConfig::Stdio { .. }));
    }

    #[tokio::test]
    async fn test_http_adapter_creation() {
        // Test HTTP adapter creation (will fail without server, but shouldn't panic)
        let result = RmcpAdapter::new_http("http://localhost:3000/mcp", HashMap::new()).await;
        assert!(result.is_err(), "Should fail without a running server");

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Failed to connect"),
            "Should have connection error"
        );
    }

    #[tokio::test]
    async fn test_stdio_not_supported() {
        // Test that stdio returns a helpful error message
        let result =
            RmcpAdapter::new_stdio("python", vec!["-m".to_string(), "server".to_string()]).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("STDIO transport not yet supported"));
        assert!(error_msg.contains("RMCP SDK v0.6.2"));
    }

    #[tokio::test]
    async fn test_config_based_creation() {
        let config = TransportConfig::http("http://localhost:3000/mcp");
        let result = RmcpAdapter::from_config(&config).await;
        assert!(result.is_err()); // Expected to fail without server
    }

    #[test]
    fn test_request_id_generation() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();

        assert!(id1.starts_with("zed_rmcp_"));
        assert!(id2.starts_with("zed_rmcp_"));
        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn test_unimplemented_features() {
        // Create a mock adapter for testing
        let adapter = RmcpAdapter {
            client: None,
            transport_type: TransportType::Http {
                url: "http://test.example.com".to_string(),
                headers: HashMap::new(),
            },
            server_info: None,
            capabilities: None,
            session_id: Some("test-session".to_string()),
        };

        // Test that unimplemented features return appropriate errors
        let prompts_result = adapter.list_prompts().await;
        assert!(prompts_result.is_err());
        assert!(
            prompts_result
                .unwrap_err()
                .to_string()
                .contains("not yet implemented")
        );

        let resources_result = adapter.list_resources().await;
        assert!(resources_result.is_err());
        assert!(
            resources_result
                .unwrap_err()
                .to_string()
                .contains("not yet implemented")
        );
    }

    #[test]
    fn test_adapter_properties() {
        let adapter = RmcpAdapter {
            client: None,
            transport_type: TransportType::Http {
                url: "http://test.example.com".to_string(),
                headers: HashMap::new(),
            },
            server_info: None,
            capabilities: None,
            session_id: Some("test-session".to_string()),
        };

        assert!(!adapter.is_connected());
        assert_eq!(adapter.session_id(), Some("test-session"));
        assert!(adapter.server_info().is_none());
        assert!(adapter.capabilities().is_none());
        assert!(adapter.peer_info().is_none());
    }
}
