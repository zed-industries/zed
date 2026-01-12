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
use std::time::Duration;
use std::{fmt::Display, path::PathBuf};

use anyhow::Result;
use client::Client;
use gpui::{AsyncApp, BackgroundExecutor};
use parking_lot::RwLock;
pub use settings::ContextServerCommand;
use url::Url;

use crate::transport::{ContextServerCredentials, HttpTransport, OnAuthUpdated};

pub use crate::transport::http::{
    AuthorizeUrl, ContextServerAuth, ContextServerAuthStatus, OAuthCallback,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextServerId(pub Arc<str>);

impl Display for ContextServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum ContextServerTransport {
    Stdio(ContextServerCommand, Option<PathBuf>),
    Http(Arc<HttpTransport>),
    Custom(Arc<dyn crate::transport::Transport>),
}

pub struct ContextServer {
    id: ContextServerId,
    client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
    configuration: ContextServerTransport,
    request_timeout: Option<Duration>,
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
            request_timeout: None,
        }
    }

    pub fn http(
        id: ContextServerId,
        endpoint: &Url,
        headers: HashMap<String, String>,
        http_client: Arc<dyn HttpClient>,
        executor: BackgroundExecutor,
        on_auth_updated: OnAuthUpdated,
        request_timeout: Option<Duration>,
    ) -> Result<Self> {
        let transport = match endpoint.scheme() {
            "http" | "https" => {
                log::info!("Using HTTP transport for {}", endpoint);
                Arc::new(HttpTransport::new(
                    http_client,
                    endpoint.to_string(),
                    headers,
                    executor,
                    on_auth_updated,
                ))
            }
            _ => anyhow::bail!("unsupported MCP url scheme {}", endpoint.scheme()),
        };

        Ok(Self {
            id,
            client: RwLock::new(None),
            configuration: ContextServerTransport::Http(transport),
            request_timeout,
        })
    }

    pub fn new(id: ContextServerId, transport: Arc<dyn crate::transport::Transport>) -> Self {
        Self::new_with_timeout(id, transport, None)
    }

    pub fn new_with_timeout(
        id: ContextServerId,
        transport: Arc<dyn crate::transport::Transport>,
        request_timeout: Option<Duration>,
    ) -> Self {
        Self {
            id,
            client: RwLock::new(None),
            configuration: ContextServerTransport::Custom(transport),
            request_timeout,
        }
    }

    pub fn id(&self) -> ContextServerId {
        self.id.clone()
    }

    pub fn client(&self) -> Option<Arc<crate::protocol::InitializedContextServerProtocol>> {
        self.client.read().clone()
    }

    pub async fn start(
        &self,
        persisted: Option<ContextServerCredentials>,
        cx: &AsyncApp,
    ) -> Result<()> {
        if let ContextServerTransport::Http(http) = &self.configuration {
            if let Some(persisted) = persisted {
                http.restore_credentials(persisted).await;
            }
        }

        self.initialize(self.new_client(cx)?).await
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
            ContextServerTransport::Http(transport) => Client::new(
                client::ContextServerId(self.id.0.clone()),
                self.id().0,
                transport.clone(),
                None,
                cx.clone(),
            )?,
            ContextServerTransport::Custom(transport) => Client::new(
                client::ContextServerId(self.id.0.clone()),
                self.id().0,
                transport.clone(),
                self.request_timeout,
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

    pub async fn start_auth(&self) -> Result<AuthorizeUrl> {
        let ContextServerTransport::Http(http) = &self.configuration else {
            anyhow::bail!("authorization is only supported for HTTP context servers");
        };

        http.start_auth().await
    }

    pub async fn handle_oauth_callback(&self, callback: &OAuthCallback) -> Result<()> {
        let ContextServerTransport::Http(http) = &self.configuration else {
            anyhow::bail!("authorization is only supported for HTTP context servers");
        };

        http.handle_oauth_callback(&callback).await
    }

    pub async fn logout(&self) {
        let ContextServerTransport::Http(http) = &self.configuration else {
            return;
        };

        http.logout().await
    }
}
