pub mod settings;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use std::path::Path;
use std::sync::Arc;
use std::{fmt::Display, path::PathBuf};

use anyhow::Result;
use collections::HashMap;
use gpui::AsyncApp;
use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::redact::should_redact;

// RMCP imports
#[cfg(feature = "rmcp")]
use rmcp::{
    ServiceExt,
    service::{RoleClient, RunningService},
    transport::{ConfigureCommandExt, TokioChildProcess},
};
#[cfg(feature = "rmcp")]
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextServerId(pub Arc<str>);

impl Display for ContextServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct ContextServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    /// Timeout for tool calls in milliseconds. Defaults to 60000 (60 seconds) if not specified.
    pub timeout: Option<u64>,
}

impl std::fmt::Debug for ContextServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filtered_env = self.env.as_ref().map(|env| {
            env.iter()
                .map(|(k, v)| (k, if should_redact(k) { "[REDACTED]" } else { v }))
                .collect::<Vec<_>>()
        });

        f.debug_struct("ContextServerCommand")
            .field("path", &self.path)
            .field("args", &self.args)
            .field("env", &filtered_env)
            .finish()
    }
}

pub enum ContextServerTransport {
    Stdio(ContextServerCommand, Option<PathBuf>),
    #[cfg(feature = "rmcp")]
    Http {
        url: String,
        headers: HashMap<String, String>,
    },
    #[cfg(feature = "rmcp")]
    Sse {
        url: String,
        headers: HashMap<String, String>,
    },
}

pub struct ContextServer {
    id: ContextServerId,
    #[cfg(feature = "rmcp")]
    service: RwLock<Option<Arc<RunningService<RoleClient, ()>>>>,
    #[cfg(not(feature = "rmcp"))]
    client: RwLock<Option<()>>, // Placeholder when RMCP not enabled
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
            #[cfg(feature = "rmcp")]
            service: RwLock::new(None),
            #[cfg(not(feature = "rmcp"))]
            client: RwLock::new(None),
            configuration: ContextServerTransport::Stdio(
                command,
                working_directory.map(|directory| directory.to_path_buf()),
            ),
        }
    }

    #[cfg(feature = "rmcp")]
    pub fn http(id: ContextServerId, url: String, headers: HashMap<String, String>) -> Self {
        Self {
            id,
            service: RwLock::new(None),
            configuration: ContextServerTransport::Http { url, headers },
        }
    }

    #[cfg(feature = "rmcp")]
    pub fn sse(id: ContextServerId, url: String, headers: HashMap<String, String>) -> Self {
        Self {
            id,
            service: RwLock::new(None),
            configuration: ContextServerTransport::Sse { url, headers },
        }
    }

    pub fn id(&self) -> ContextServerId {
        self.id.clone()
    }

    #[cfg(feature = "rmcp")]
    pub fn service(&self) -> Option<Arc<RunningService<RoleClient, ()>>> {
        self.service.read().clone()
    }

    #[cfg(not(feature = "rmcp"))]
    pub fn service(&self) -> Option<()> {
        None
    }

    // Legacy method for backward compatibility
    #[cfg(feature = "rmcp")]
    pub fn client(&self) -> Option<Arc<RunningService<RoleClient, ()>>> {
        self.service()
    }

    #[cfg(not(feature = "rmcp"))]
    pub fn client(&self) -> Option<()> {
        None
    }

    pub async fn start(&self, _cx: &AsyncApp) -> Result<()> {
        self.initialize().await
    }

    /// Starts the context server, making sure handlers are registered before initialization happens
    pub async fn start_with_handlers(
        &self,
        _notification_handlers: Vec<(
            &'static str,
            Box<dyn 'static + Send + FnMut(serde_json::Value, AsyncApp)>,
        )>,
        cx: &AsyncApp,
    ) -> Result<()> {
        // Note: RMCP handles notifications differently, this might need to be updated
        // when we implement notification handling
        self.start(cx).await
    }

    #[cfg(feature = "rmcp")]
    async fn initialize(&self) -> Result<()> {
        log::debug!("starting context server {}", self.id);

        let service = match &self.configuration {
            ContextServerTransport::Stdio(command, working_directory) => {
                let mut cmd = Command::new(&command.path);
                cmd.args(&command.args);

                if let Some(env) = &command.env {
                    cmd.envs(env);
                }

                if let Some(working_directory) = working_directory {
                    cmd.current_dir(working_directory);
                }

                let transport = TokioChildProcess::new(cmd.configure(|_| {}))?;
                ().serve(transport).await?
            }
            ContextServerTransport::Http { url: _, headers: _ } => {
                // TODO: Implement HTTP transport when needed
                return Err(anyhow::anyhow!("HTTP transport not yet implemented"));
            }
            ContextServerTransport::Sse { url: _, headers: _ } => {
                // TODO: Implement SSE transport when needed
                return Err(anyhow::anyhow!("SSE transport not yet implemented"));
            }
        };

        let server_info = service.peer_info();
        log::debug!("context server {} initialized: {:?}", self.id, server_info);

        *self.service.write() = Some(service);
        Ok(())
    }

    #[cfg(not(feature = "rmcp"))]
    async fn initialize(&self) -> Result<()> {
        Err(anyhow::anyhow!("RMCP feature not enabled"))
    }

    #[cfg(feature = "rmcp")]
    pub async fn list_tools(&self) -> Result<Vec<rmcp::model::Tool>> {
        let service = self.service.read();
        let service = service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Context server not initialized"))?;
        Ok(service.list_all_tools().await?)
    }

    #[cfg(not(feature = "rmcp"))]
    pub async fn list_tools(&self) -> Result<Vec<()>> {
        Err(anyhow::anyhow!("RMCP feature not enabled"))
    }

    #[cfg(feature = "rmcp")]
    pub async fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParam,
    ) -> Result<rmcp::model::CallToolResult> {
        let service = self.service.read();
        let service = service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Context server not initialized"))?;
        Ok(service.call_tool(params).await?)
    }

    #[cfg(not(feature = "rmcp"))]
    pub async fn call_tool(&self, _params: ()) -> Result<()> {
        Err(anyhow::anyhow!("RMCP feature not enabled"))
    }

    #[cfg(feature = "rmcp")]
    pub async fn list_prompts(&self) -> Result<Vec<rmcp::model::Prompt>> {
        let service = self.service.read();
        let service = service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Context server not initialized"))?;
        Ok(service.list_all_prompts().await?)
    }

    #[cfg(not(feature = "rmcp"))]
    pub async fn list_prompts(&self) -> Result<Vec<()>> {
        Err(anyhow::anyhow!("RMCP feature not enabled"))
    }

    #[cfg(feature = "rmcp")]
    pub async fn get_prompt(
        &self,
        params: rmcp::model::GetPromptRequestParam,
    ) -> Result<rmcp::model::GetPromptResult> {
        let service = self.service.read();
        let service = service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Context server not initialized"))?;
        Ok(service.get_prompt(params).await?)
    }

    #[cfg(not(feature = "rmcp"))]
    pub async fn get_prompt(&self, _params: ()) -> Result<()> {
        Err(anyhow::anyhow!("RMCP feature not enabled"))
    }

    #[cfg(feature = "rmcp")]
    pub async fn list_resources(&self) -> Result<Vec<rmcp::model::Resource>> {
        let service = self.service.read();
        let service = service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Context server not initialized"))?;
        Ok(service.list_all_resources().await?)
    }

    #[cfg(not(feature = "rmcp"))]
    pub async fn list_resources(&self) -> Result<Vec<()>> {
        Err(anyhow::anyhow!("RMCP feature not enabled"))
    }

    #[cfg(feature = "rmcp")]
    pub async fn read_resource(
        &self,
        params: rmcp::model::ReadResourceRequestParam,
    ) -> Result<rmcp::model::ReadResourceResult> {
        let service = self.service.read();
        let service = service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Context server not initialized"))?;
        Ok(service.read_resource(params).await?)
    }

    #[cfg(not(feature = "rmcp"))]
    pub async fn read_resource(&self, _params: ()) -> Result<()> {
        Err(anyhow::anyhow!("RMCP feature not enabled"))
    }

    pub async fn stop(&self) -> Result<()> {
        #[cfg(feature = "rmcp")]
        {
            let mut service = self.service.write();
            if let Some(service) = service.take() {
                if let Err(e) = service.cancel().await {
                    log::warn!("Error canceling context server {}: {}", self.id, e);
                }
            }
        }
        Ok(())
    }
}
