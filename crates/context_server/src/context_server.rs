pub mod client;
pub mod listener;
pub mod protocol;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
pub mod transport;
pub mod types;

use collections::HashMap;
use http_client::HttpClient;
use std::path::Path;
use std::sync::Arc;
use std::{fmt::Display, path::PathBuf};

use anyhow::Result;
use client::Client;
use gpui::AsyncApp;
use parking_lot::RwLock;
pub use settings::ContextServerCommand;
use url::Url;

use crate::transport::HttpTransport;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextServerId(pub Arc<str>);

impl Display for ContextServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    pub fn http(
        id: ContextServerId,
        endpoint: &Url,
        headers: HashMap<String, String>,
        http_client: Arc<dyn HttpClient>,
        executor: gpui::BackgroundExecutor,
    ) -> Result<Self> {
        let transport = match endpoint.scheme() {
            "http" | "https" => {
                log::info!("Using HTTP transport for {}", endpoint);
                let transport =
                    HttpTransport::new(http_client, endpoint.to_string(), headers, executor);
                Arc::new(transport) as _
            }
            _ => anyhow::bail!("unsupported MCP url scheme {}", endpoint.scheme()),
        };
        Ok(Self::new(id, transport))
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
            ContextServerTransport::Stdio(command, working_directory) => Client::stdio(
                client::ContextServerId(self.id.0.clone()),
                client::ModelContextServerBinary {
                    executable: Path::new(&command.path).to_path_buf(),
                    args: command.args.clone(),
                    env: command.env.clone(),
                    timeout: command.timeout,
                },
                working_directory,
                cx.clone(),
            )?,
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
