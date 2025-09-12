pub mod client;
pub mod listener;
pub mod protocol;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
pub mod transport;
pub mod types;

use std::path::Path;
use std::sync::Arc;
use std::{fmt::Display, path::PathBuf};

use anyhow::Result;
use client::Client;
use collections::HashMap;
use gpui::AsyncApp;
use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::redact::should_redact;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextServerId(pub Arc<str>);

impl Display for ContextServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Clone, PartialEq, Eq, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContextServerCommand {
    /// Local MCP server with command, args, and env
    Local {
        #[serde(rename = "command")]
        path: PathBuf,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: Option<HashMap<String, String>>,
        /// Timeout for tool calls in milliseconds. Defaults to 60000 (60 seconds) if not specified.
        #[serde(default)]
        timeout: Option<u64>,
    },
    /// Remote MCP server via HTTP/SSE
    Http {
        /// The URL of the remote MCP server
        url: String,
        /// Optional headers to send with requests
        #[serde(default)]
        headers: Option<HashMap<String, String>>,
        /// Timeout for tool calls in milliseconds. Defaults to 60000 (60 seconds) if not specified.
        #[serde(default)]
        timeout: Option<u64>,
    },
}

impl<'de> Deserialize<'de> for ContextServerCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        
        let value = Value::deserialize(deserializer)?;
        
        // Check if this is the old format (no "type" field)
        if let Value::Object(ref map) = value {
            if !map.contains_key("type") && map.contains_key("command") {
                // Old format - convert to Local variant
                #[derive(Deserialize)]
                struct OldFormat {
                    #[serde(rename = "command")]
                    path: PathBuf,
                    #[serde(default)]
                    args: Vec<String>,
                    #[serde(default)]
                    env: Option<HashMap<String, String>>,
                    #[serde(default)]
                    timeout: Option<u64>,
                }
                
                let old: OldFormat = serde_json::from_value(value)
                    .map_err(serde::de::Error::custom)?;
                
                return Ok(ContextServerCommand::Local {
                    path: old.path,
                    args: old.args,
                    env: old.env,
                    timeout: old.timeout,
                });
            }
        }
        
        // New format with type tag
        #[derive(Deserialize)]
        #[serde(tag = "type", rename_all = "lowercase")]
        enum NewFormat {
            Local {
                #[serde(rename = "command")]
                path: PathBuf,
                #[serde(default)]
                args: Vec<String>,
                #[serde(default)]
                env: Option<HashMap<String, String>>,
                #[serde(default)]
                timeout: Option<u64>,
            },
            Http {
                url: String,
                #[serde(default)]
                headers: Option<HashMap<String, String>>,
                #[serde(default)]
                timeout: Option<u64>,
            },
        }
        
        let new: NewFormat = serde_json::from_value(value)
            .map_err(serde::de::Error::custom)?;
            
        match new {
            NewFormat::Local { path, args, env, timeout } => {
                Ok(ContextServerCommand::Local { path, args, env, timeout })
            }
            NewFormat::Http { url, headers, timeout } => {
                Ok(ContextServerCommand::Http { url, headers, timeout })
            }
        }
    }
}

impl ContextServerCommand {
    /// Get the effective command to run, transforming remote servers to use mcp-remote
    pub fn effective_command(&self) -> (PathBuf, Vec<String>, Option<HashMap<String, String>>) {
        match self {
            ContextServerCommand::Local { path, args, env, .. } => {
                (path.clone(), args.clone(), env.clone())
            }
            ContextServerCommand::Http { url, headers, .. } => {
                let args = vec!["mcp-remote".to_string(), url.clone()];
                
                // Add headers as environment variables if provided
                let mut env = HashMap::default();
                if let Some(headers) = headers {
                    for (key, value) in headers {
                        env.insert(format!("MCP_HEADER_{}", key.to_uppercase()), value.clone());
                    }
                }
                
                (
                    PathBuf::from("npx"),
                    args,
                    if env.is_empty() { None } else { Some(env) }
                )
            }
        }
    }
    
    /// Get the timeout value
    pub fn timeout(&self) -> Option<u64> {
        match self {
            ContextServerCommand::Local { timeout, .. } => *timeout,
            ContextServerCommand::Http { timeout, .. } => *timeout,
        }
    }
}

impl std::fmt::Debug for ContextServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextServerCommand::Local { path, args, env, timeout } => {
                let filtered_env = env.as_ref().map(|env| {
                    env.iter()
                        .map(|(k, v)| (k, if should_redact(k) { "[REDACTED]" } else { v }))
                        .collect::<Vec<_>>()
                });

                f.debug_struct("ContextServerCommand::Local")
                    .field("path", path)
                    .field("args", args)
                    .field("env", &filtered_env)
                    .field("timeout", timeout)
                    .finish()
            }
            ContextServerCommand::Http { url, headers, timeout } => {
                let filtered_headers = headers.as_ref().map(|headers| {
                    headers.iter()
                        .map(|(k, v)| (k, if should_redact(k) { "[REDACTED]" } else { v }))
                        .collect::<Vec<_>>()
                });

                f.debug_struct("ContextServerCommand::Http")
                    .field("url", url)
                    .field("headers", &filtered_headers)
                    .field("timeout", timeout)
                    .finish()
            }
        }
    }
}

enum ContextServerTransport {
    Stdio(ContextServerCommand, Option<PathBuf>),
    Custom(Arc<dyn crate::transport::Transport>),
}

pub struct ContextServer {
    id: ContextServerId,
    client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
    configuration: ContextServerTransport,
}

impl ContextServer {
    pub fn stdio(
        id: ContextServerId,
        command: ContextServerCommand,
        working_directory: Option<Arc<Path>>,
    ) -> Self {
        Self {
            id,
            client: RwLock::new(None),
            configuration: ContextServerTransport::Stdio(
                command,
                working_directory.map(|directory| directory.to_path_buf()),
            ),
        }
    }

    pub fn new(id: ContextServerId, transport: Arc<dyn crate::transport::Transport>) -> Self {
        Self {
            id,
            client: RwLock::new(None),
            configuration: ContextServerTransport::Custom(transport),
        }
    }

    pub fn id(&self) -> ContextServerId {
        self.id.clone()
    }

    pub fn client(&self) -> Option<Arc<crate::protocol::InitializedContextServerProtocol>> {
        self.client.read().clone()
    }

    pub async fn start(&self, cx: &AsyncApp) -> Result<()> {
        self.initialize(self.new_client(cx)?).await
    }

    /// Starts the context server, making sure handlers are registered before initialization happens
    pub async fn start_with_handlers(
        &self,
        notification_handlers: Vec<(
            &'static str,
            Box<dyn 'static + Send + FnMut(serde_json::Value, AsyncApp)>,
        )>,
        cx: &AsyncApp,
    ) -> Result<()> {
        let client = self.new_client(cx)?;
        for (method, handler) in notification_handlers {
            client.on_notification(method, handler);
        }
        self.initialize(client).await
    }

    fn new_client(&self, cx: &AsyncApp) -> Result<Client> {
        Ok(match &self.configuration {
            ContextServerTransport::Stdio(command, working_directory) => {
                let (executable, args, env) = command.effective_command();
                Client::stdio(
                    client::ContextServerId(self.id.0.clone()),
                    client::ModelContextServerBinary {
                        executable,
                        args,
                        env,
                        timeout: command.timeout(),
                    },
                    working_directory,
                    cx.clone(),
                )?
            }
            ContextServerTransport::Custom(transport) => Client::new(
                client::ContextServerId(self.id.0.clone()),
                self.id().0,
                transport.clone(),
                None,
                cx.clone(),
            )?,
        })
    }

    async fn initialize(&self, client: Client) -> Result<()> {
        log::debug!("starting context server {}", self.id);
        let protocol = crate::protocol::ModelContextProtocol::new(client);
        let client_info = types::Implementation {
            name: "Zed".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        let initialized_protocol = protocol.initialize(client_info).await?;

        log::debug!(
            "context server {} initialized: {:?}",
            self.id,
            initialized_protocol.initialize,
        );

        *self.client.write() = Some(Arc::new(initialized_protocol));
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        let mut client = self.client.write();
        if let Some(protocol) = client.take() {
            drop(protocol);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_backward_compatibility() {
        // Test old format without "type" field
        let old_format_json = r#"
        {
            "command": "/usr/bin/python3",
            "args": ["server.py"],
            "env": {"API_KEY": "test"}
        }
        "#;
        
        let cmd: ContextServerCommand = serde_json::from_str(old_format_json).unwrap();
        match cmd {
            ContextServerCommand::Local { path, args, env, .. } => {
                assert_eq!(path, PathBuf::from("/usr/bin/python3"));
                assert_eq!(args, vec!["server.py"]);
                assert!(env.unwrap().contains_key("API_KEY"));
            }
            _ => panic!("Expected Local variant"),
        }
    }

    #[test]
    fn test_local_format() {
        let local_format_json = r#"
        {
            "type": "local",
            "command": "/usr/bin/python3",
            "args": ["server.py"],
            "env": {"API_KEY": "test"}
        }
        "#;
        
        let cmd: ContextServerCommand = serde_json::from_str(local_format_json).unwrap();
        let (path, args, env) = cmd.effective_command();
        assert_eq!(path, PathBuf::from("/usr/bin/python3"));
        assert_eq!(args, vec!["server.py"]);
        assert!(env.unwrap().contains_key("API_KEY"));
    }

    #[test]
    fn test_remote_format() {
        let remote_format_json = r#"
        {
            "type": "http",
            "url": "https://mcp.atlassian.com/v1/sse",
            "headers": {"Authorization": "Bearer token"}
        }
        "#;
        
        let cmd: ContextServerCommand = serde_json::from_str(remote_format_json).unwrap();
        let (path, args, env) = cmd.effective_command();
        assert_eq!(path, PathBuf::from("npx"));
        assert_eq!(args, vec!["mcp-remote", "https://mcp.atlassian.com/v1/sse"]);
        assert!(env.unwrap().contains_key("MCP_HEADER_AUTHORIZATION"));
    }

    #[test]
    fn test_remote_without_headers() {
        let remote_format_json = r#"
        {
            "type": "http",
            "url": "https://mcp.example.com/sse"
        }
        "#;
        
        let cmd: ContextServerCommand = serde_json::from_str(remote_format_json).unwrap();
        let (path, args, env) = cmd.effective_command();
        assert_eq!(path, PathBuf::from("npx"));
        assert_eq!(args, vec!["mcp-remote", "https://mcp.example.com/sse"]);
        assert!(env.is_none());
    }
}