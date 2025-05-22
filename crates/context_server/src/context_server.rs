pub mod client;
pub mod protocol;
pub mod transport;
pub mod types;

use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use client::Client;
use collections::HashMap;
use gpui::AsyncApp;
use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextServerId(pub Arc<str>);

impl Display for ContextServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ContextServerCommand {
    pub path: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

enum ContextServerTransport {
    Stdio(ContextServerCommand),
    Custom(Arc<dyn crate::transport::Transport>),
}

pub struct ContextServer {
    id: ContextServerId,
    client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
    configuration: ContextServerTransport,
}

impl ContextServer {
    pub fn stdio(id: ContextServerId, command: ContextServerCommand) -> Self {
        Self {
            id,
            client: RwLock::new(None),
            configuration: ContextServerTransport::Stdio(command),
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

    pub async fn start(self: Arc<Self>, cx: &AsyncApp) -> Result<()> {
        let client = match &self.configuration {
            ContextServerTransport::Stdio(command) => Client::stdio(
                client::ContextServerId(self.id.0.clone()),
                client::ModelContextServerBinary {
                    executable: Path::new(&command.path).to_path_buf(),
                    args: command.args.clone(),
                    env: command.env.clone(),
                },
                cx.clone(),
            )?,
            ContextServerTransport::Custom(transport) => Client::new(
                client::ContextServerId(self.id.0.clone()),
                self.id().0,
                transport.clone(),
                cx.clone(),
            )?,
        };
        self.initialize(client).await
    }

    async fn initialize(&self, client: Client) -> Result<()> {
        log::info!("starting context server {}", self.id);
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
