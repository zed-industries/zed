pub mod client;
pub mod listener;
pub mod log_store;
pub mod protocol;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
pub mod transport;
pub mod types;

pub use log_store::init;

use collections::HashMap;
use http_client::HttpClient;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
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

pub struct ContextServerIoSubscription {
    id: i32,
    io_subscribers: std::sync::Weak<parking_lot::Mutex<HashMap<i32, client::IoHandler>>>,
}

impl Drop for ContextServerIoSubscription {
    fn drop(&mut self) {
        if let Some(subscribers) = self.io_subscribers.upgrade() {
            subscribers.lock().remove(&self.id);
        }
    }
}

pub struct ContextServer {
    id: ContextServerId,
    client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
    configuration: ContextServerTransport,
    request_timeout: Option<Duration>,
    /// A list of external subscribers listening to this server's I/O events.
    io_subscribers: Arc<parking_lot::Mutex<HashMap<i32, client::IoHandler>>>,
    /// The next ID to assign to a new I/O subscriber.
    next_io_subscription_id: std::sync::atomic::AtomicI32,
    /// The subscription to the currently active inner client's I/O stream.
    /// Replaced when the server is restarted.
    _client_io_subscription: parking_lot::Mutex<Option<client::IoSubscription>>,
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
            io_subscribers: Arc::new(parking_lot::Mutex::new(collections::HashMap::default())),
            next_io_subscription_id: std::sync::atomic::AtomicI32::new(0),
            _client_io_subscription: parking_lot::Mutex::new(None),
        }
    }

    pub fn http(
        id: ContextServerId,
        endpoint: &Url,
        headers: HashMap<String, String>,
        http_client: Arc<dyn HttpClient>,
        executor: gpui::BackgroundExecutor,
        request_timeout: Option<Duration>,
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
        Ok(Self::new_with_timeout(id, transport, request_timeout))
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
            io_subscribers: Arc::new(parking_lot::Mutex::new(collections::HashMap::default())),
            next_io_subscription_id: std::sync::atomic::AtomicI32::new(0),
            _client_io_subscription: parking_lot::Mutex::new(None),
        }
    }

    pub fn on_io<F>(&self, f: F) -> ContextServerIoSubscription
    where
        F: 'static + Send + FnMut(client::IoKind, &str),
    {
        let id = self
            .next_io_subscription_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.io_subscribers.lock().insert(id, Box::new(f));
        ContextServerIoSubscription {
            id,
            io_subscribers: Arc::downgrade(&self.io_subscribers),
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

    fn new_client(&self, cx: &AsyncApp) -> Result<Client> {
        let client = match &self.configuration {
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
                self.id.0.clone(),
                transport.clone(),
                self.request_timeout,
                cx.clone(),
            )?,
        };

        let io_subscribers = self.io_subscribers.clone();
        *self._client_io_subscription.lock() = Some(client.on_io(move |kind, msg| {
            for subscriber in io_subscribers.lock().values_mut() {
                subscriber(kind, msg);
            }
        }));

        Ok(client)
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
