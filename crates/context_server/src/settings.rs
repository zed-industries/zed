// crates/context_server/src/settings.rs

use anyhow::Result;
use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::path::PathBuf;

/// Settings for context servers
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ContextServerSettings {
    /// List of configured context servers
    #[serde(default)]
    pub servers: Vec<ContextServerDefinition>,

    /// Global timeout for server operations (in seconds)
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Whether to automatically start servers on Zed launch
    #[serde(default = "default_auto_start")]
    pub auto_start: bool,

    /// Whether to use the legacy implementation
    #[serde(default)]
    pub use_legacy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ContextServerDefinition {
    /// Legacy stdio-based server (backward compatible)
    Legacy(LegacyServerConfig),

    /// New transport-based server configuration
    #[serde(rename_all = "snake_case")]
    Transport(TransportServerConfig),
}

/// Legacy server configuration (backward compatible)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegacyServerConfig {
    /// Server name/identifier
    pub name: String,

    /// Command to execute
    pub command: String,

    /// Arguments for the command
    #[serde(default)]
    pub args: Vec<String>,

    /// Environment variables
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<PathBuf>,

    /// Whether this server is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

/// New transport-based server configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TransportServerConfig {
    /// Server name/identifier
    pub name: String,

    /// Transport type
    pub transport: TransportType,

    /// Whether this server is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Optional timeout override for this server (in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransportType {
    /// Standard I/O transport (local process)
    Stdio {
        /// Command to execute
        command: String,

        /// Arguments for the command
        #[serde(default)]
        args: Vec<String>,

        /// Environment variables
        #[serde(default)]
        env: HashMap<String, String>,

        /// Working directory
        #[serde(skip_serializing_if = "Option::is_none")]
        cwd: Option<PathBuf>,
    },

    /// HTTP transport (remote server)
    Http {
        /// Server URL
        url: String,

        /// Optional HTTP headers
        #[serde(default)]
        headers: HashMap<String, String>,

        /// Optional authentication
        #[serde(skip_serializing_if = "Option::is_none")]
        auth: Option<AuthConfig>,
    },

    /// Server-Sent Events transport (for streaming)
    Sse {
        /// Server URL
        url: String,

        /// Optional HTTP headers
        #[serde(default)]
        headers: HashMap<String, String>,

        /// Optional authentication
        #[serde(skip_serializing_if = "Option::is_none")]
        auth: Option<AuthConfig>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthConfig {
    /// Bearer token authentication
    Bearer { token: String },

    /// Basic authentication
    Basic { username: String, password: String },

    /// API key authentication
    ApiKey {
        key: String,
        #[serde(default = "default_api_key_header")]
        header: String,
    },
}

fn default_timeout() -> u64 {
    30
}

fn default_auto_start() -> bool {
    true
}

fn default_enabled() -> bool {
    true
}

fn default_api_key_header() -> String {
    "X-API-Key".to_string()
}

impl Settings for ContextServerSettings {
    const KEY: Option<&'static str> = Some("context_servers");
}

impl ContextServerSettings {
    /// Get all enabled servers
    pub fn enabled_servers(&self) -> Vec<&ContextServerDefinition> {
        self.servers
            .iter()
            .filter(|server| match server {
                ContextServerDefinition::Legacy(config) => config.enabled,
                ContextServerDefinition::Transport(config) => config.enabled,
            })
            .collect()
    }

    /// Find a server by name
    pub fn find_server(&self, name: &str) -> Option<&ContextServerDefinition> {
        self.servers.iter().find(|server| match server {
            ContextServerDefinition::Legacy(config) => config.name == name,
            ContextServerDefinition::Transport(config) => config.name == name,
        })
    }
}

// Example settings.json configurations:
#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example_settings() {
        let legacy_example = r#"
{
  "context_servers": {
    "servers": [
      {
        "name": "filesystem-mcp",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-filesystem"],
        "env": {
          "ALLOWED_PATHS": "/home/user/projects"
        }
      }
    ]
  }
}"#;

        let modern_example = r#"
{
  "context_servers": {
    "servers": [
      {
        "name": "local-filesystem",
        "transport": {
          "type": "stdio",
          "command": "mcp-server-filesystem",
          "args": ["--root", "/home/user/projects"]
        }
      },
      {
        "name": "remote-api",
        "transport": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "User-Agent": "Zed-MCP-Client"
          },
          "auth": {
            "type": "bearer",
            "token": "your-api-token-here"
          }
        }
      },
      {
        "name": "streaming-server",
        "transport": {
          "type": "sse",
          "url": "https://stream.example.com/mcp",
          "auth": {
            "type": "api_key",
            "key": "sk-1234567890"
          }
        }
      }
    ],
    "timeout": 60,
    "auto_start": true
  }
}"#;

        // Verify that both formats can be parsed
        let legacy: serde_json::Value = serde_json::from_str(legacy_example).unwrap();
        let modern: serde_json::Value = serde_json::from_str(modern_example).unwrap();

        println!("Legacy config: {:#?}", legacy);
        println!("Modern config: {:#?}", modern);
    }
}
