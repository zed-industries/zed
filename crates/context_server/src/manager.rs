//! This module implements a context server management system for Zed.
//!
//! It provides functionality to:
//! - Define and load context server settings
//! - Manage individual context servers (start, stop, restart)
//! - Maintain a global manager for all context servers
//!
//! Key components:
//! - `ContextServerSettings`: Defines the structure for server configurations
//! - `ContextServer`: Represents an individual context server
//! - `ContextServerManager`: Manages multiple context servers
//! - `GlobalContextServerManager`: Provides global access to the ContextServerManager
//!
//! The module also includes initialization logic to set up the context server system
//! and react to changes in settings.

use std::path::Path;
use std::sync::Arc;

use anyhow::{Result, bail};
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use gpui::{AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity};
use log;
use parking_lot::RwLock;
use project::Project;
use settings::{Settings, SettingsStore};
use util::ResultExt as _;

use crate::transport::Transport;
use crate::{ContextServerSettings, ServerConfig};

use crate::{
    CONTEXT_SERVERS_NAMESPACE, ContextServerDescriptorRegistry,
    client::{self, Client},
    types,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextServerStatus {
    Starting,
    Running,
    Error(Arc<str>),
}

pub struct ContextServer {
    pub id: Arc<str>,
    pub config: Arc<ServerConfig>,
    pub client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
    transport: Option<Arc<dyn Transport>>,
}

impl ContextServer {
    pub fn new(id: Arc<str>, config: Arc<ServerConfig>) -> Self {
        Self {
            id,
            config,
            client: RwLock::new(None),
            transport: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(id: Arc<str>, transport: Arc<dyn crate::transport::Transport>) -> Arc<Self> {
        Arc::new(Self {
            id,
            client: RwLock::new(None),
            config: Arc::new(ServerConfig::default()),
            transport: Some(transport),
        })
    }

    pub fn id(&self) -> Arc<str> {
        self.id.clone()
    }

    pub fn config(&self) -> Arc<ServerConfig> {
        self.config.clone()
    }

    pub fn client(&self) -> Option<Arc<crate::protocol::InitializedContextServerProtocol>> {
        self.client.read().clone()
    }

    pub async fn start(self: Arc<Self>, cx: &AsyncApp) -> Result<()> {
        let client = if let Some(transport) = self.transport.clone() {
            Client::new(
                client::ContextServerId(self.id.clone()),
                self.id(),
                transport,
                cx.clone(),
            )?
        } else {
            let Some(command) = &self.config.command else {
                bail!("no command specified for server {}", self.id);
            };
            Client::stdio(
                client::ContextServerId(self.id.clone()),
                client::ModelContextServerBinary {
                    executable: Path::new(&command.path).to_path_buf(),
                    args: command.args.clone(),
                    env: command.env.clone(),
                },
                cx.clone(),
            )?
        };
        self.initialize(client).await
    }

    async fn initialize(&self, client: Client) -> Result<()> {
        log::info!("starting context server {}", self.id);
        dbg!("Settign up");
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

pub struct ContextServerManager {
    servers: HashMap<Arc<str>, Arc<ContextServer>>,
    server_status: HashMap<Arc<str>, ContextServerStatus>,
    project: Entity<Project>,
    registry: Entity<ContextServerDescriptorRegistry>,
    update_servers_task: Option<Task<Result<()>>>,
    needs_server_update: bool,
    _subscriptions: Vec<Subscription>,
}

pub enum Event {
    ServerStatusChanged {
        server_id: Arc<str>,
        status: Option<ContextServerStatus>,
    },
}

impl EventEmitter<Event> for ContextServerManager {}

impl ContextServerManager {
    pub fn new(
        registry: Entity<ContextServerDescriptorRegistry>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            _subscriptions: vec![
                cx.observe(&registry, |this, _registry, cx| {
                    this.available_context_servers_changed(cx);
                }),
                cx.observe_global::<SettingsStore>(|this, cx| {
                    this.available_context_servers_changed(cx);
                }),
            ],
            project,
            registry,
            needs_server_update: false,
            servers: HashMap::default(),
            server_status: HashMap::default(),
            update_servers_task: None,
        };
        this.available_context_servers_changed(cx);
        this
    }

    fn available_context_servers_changed(&mut self, cx: &mut Context<Self>) {
        if self.update_servers_task.is_some() {
            self.needs_server_update = true;
        } else {
            self.update_servers_task = Some(cx.spawn(async move |this, cx| {
                this.update(cx, |this, _| {
                    this.needs_server_update = false;
                })?;

                if let Err(err) = Self::maintain_servers(this.clone(), cx).await {
                    log::error!("Error maintaining context servers: {}", err);
                }

                this.update(cx, |this, cx| {
                    let has_any_context_servers = !this.running_servers().is_empty();
                    if has_any_context_servers {
                        CommandPaletteFilter::update_global(cx, |filter, _cx| {
                            filter.show_namespace(CONTEXT_SERVERS_NAMESPACE);
                        });
                    }

                    this.update_servers_task.take();
                    if this.needs_server_update {
                        this.available_context_servers_changed(cx);
                    }
                })?;

                Ok(())
            }));
        }
    }

    pub fn get_server(&self, id: &str) -> Option<Arc<ContextServer>> {
        self.servers
            .get(id)
            .filter(|server| server.client().is_some())
            .cloned()
    }

    pub fn status_for_server(&self, id: &str) -> Option<ContextServerStatus> {
        self.server_status.get(id).cloned()
    }

    pub fn start_server(
        &self,
        server: Arc<ContextServer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        cx.spawn(async move |this, cx| Self::run_server(this, server, cx).await)
    }

    pub fn stop_server(
        &mut self,
        server: Arc<ContextServer>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        server.stop().log_err();
        self.update_server_status(server.id().clone(), None, cx);
        Ok(())
    }

    pub fn restart_server(&mut self, id: &Arc<str>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let id = id.clone();
        cx.spawn(async move |this, cx| {
            if let Some(server) = this.update(cx, |this, _cx| this.servers.remove(&id))? {
                let config = server.config();

                this.update(cx, |this, cx| this.stop_server(server, cx))??;
                let new_server = Arc::new(ContextServer::new(id.clone(), config));
                Self::run_server(this, new_server, cx).await?;
            }
            Ok(())
        })
    }

    pub fn all_servers(&self) -> Vec<Arc<ContextServer>> {
        self.servers.values().cloned().collect()
    }

    pub fn running_servers(&self) -> Vec<Arc<ContextServer>> {
        self.servers
            .values()
            .filter(|server| server.client().is_some())
            .cloned()
            .collect()
    }

    async fn maintain_servers(this: WeakEntity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let mut desired_servers = HashMap::default();

        let (registry, project) = this.update(cx, |this, cx| {
            let location = this
                .project
                .read(cx)
                .visible_worktrees(cx)
                .next()
                .map(|worktree| settings::SettingsLocation {
                    worktree_id: worktree.read(cx).id(),
                    path: Path::new(""),
                });
            let settings = ContextServerSettings::get(location, cx);
            desired_servers = settings.context_servers.clone();

            (this.registry.clone(), this.project.clone())
        })?;

        for (id, descriptor) in
            registry.read_with(cx, |registry, _| registry.context_server_descriptors())?
        {
            let config = desired_servers.entry(id).or_default();
            if config.command.is_none() {
                if let Some(extension_command) =
                    descriptor.command(project.clone(), &cx).await.log_err()
                {
                    config.command = Some(extension_command);
                }
            }
        }

        let mut servers_to_start = HashMap::default();
        let mut servers_to_stop = HashMap::default();

        this.update(cx, |this, _cx| {
            this.servers.retain(|id, server| {
                if desired_servers.contains_key(id) {
                    true
                } else {
                    servers_to_stop.insert(id.clone(), server.clone());
                    false
                }
            });

            for (id, config) in desired_servers {
                let existing_config = this.servers.get(&id).map(|server| server.config());
                if existing_config.as_deref() != Some(&config) {
                    let server = Arc::new(ContextServer::new(id.clone(), Arc::new(config)));
                    servers_to_start.insert(id.clone(), server.clone());
                    if let Some(old_server) = this.servers.remove(&id) {
                        servers_to_stop.insert(id, old_server);
                    }
                }
            }
        })?;

        for (_, server) in servers_to_stop {
            this.update(cx, |this, cx| this.stop_server(server, cx))??;
        }

        for (_, server) in servers_to_start {
            Self::run_server(this.clone(), server, cx).await?;
        }

        Ok(())
    }

    async fn run_server(
        this: WeakEntity<Self>,
        server: Arc<ContextServer>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let id = server.id();

        this.update(cx, |this, cx| {
            this.update_server_status(id.clone(), Some(ContextServerStatus::Starting), cx)
        })?;

        let new_server = server.clone();

        match server.start(&cx).await {
            Ok(_) => {
                log::debug!("`{}` context server started", id);
                this.update(cx, |this, cx| {
                    this.servers.insert(id.clone(), new_server);
                    this.update_server_status(id.clone(), Some(ContextServerStatus::Running), cx)
                })?;
                Ok(())
            }
            Err(err) => {
                log::error!("`{}` context server failed to start\n{}", id, err);
                this.update(cx, |this, cx| {
                    this.update_server_status(
                        id.clone(),
                        Some(ContextServerStatus::Error(err.to_string().into())),
                        cx,
                    )
                })?;
                Err(err)
            }
        }
    }

    fn update_server_status(
        &mut self,
        id: Arc<str>,
        status: Option<ContextServerStatus>,
        cx: &mut Context<Self>,
    ) {
        if let Some(status) = status.clone() {
            self.server_status.insert(id.clone(), status);
        } else {
            self.server_status.remove(&id);
        }

        cx.emit(Event::ServerStatusChanged {
            server_id: id,
            status,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::{pin::Pin, time::Duration};

    use crate::types::{
        Implementation, InitializeResponse, ProtocolVersion, RequestType, ServerCapabilities,
    };

    use super::*;
    use futures::Stream;
    use gpui::{AppContext as _, TestAppContext};
    use project::FakeFs;
    use serde_json::json;
    use util::path;

    #[gpui::test]
    async fn test_context_server_status(cx: &mut TestAppContext) {
        init_test_settings(cx);
        let project = create_test_project(cx, json!({"code.rs": ""})).await;

        let transport = Arc::new(FakeTransport::new(|request_type, _| match request_type {
            Some(RequestType::Initialize) => Some(
                serde_json::to_string(&InitializeResponse {
                    protocol_version: ProtocolVersion(types::LATEST_PROTOCOL_VERSION.to_string()),
                    server_info: Implementation {
                        name: "mcp".to_string(),
                        version: "1.0.0".to_string(),
                    },
                    capabilities: ServerCapabilities::default(),
                    meta: None,
                })
                .unwrap(),
            ),
            _ => None,
        }));

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let manager = cx.new(|cx| ContextServerManager::new(registry.clone(), project, cx));

        let server_1_id: Arc<str> = "mcp-1".into();
        let server_2_id: Arc<str> = "mcp-2".into();

        let server_1 = ContextServer::test(server_1_id.clone(), transport.clone());
        let server_2 = ContextServer::test(server_2_id.clone(), transport.clone());

        manager
            .update(cx, |manager, cx| manager.start_server(server_1, cx))
            .await
            .unwrap();

        cx.update(|cx| {
            assert_eq!(
                manager.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(manager.read(cx).status_for_server(&server_2_id), None);
        });

        manager
            .update(cx, |manager, cx| manager.start_server(server_2, cx))
            .await
            .unwrap();

        cx.update(|cx| {
            assert_eq!(
                manager.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(
                manager.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
        });
    }

    async fn create_test_project(
        cx: &mut TestAppContext,
        files: serde_json::Value,
    ) -> Entity<Project> {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), files).await;
        Project::test(fs, [path!("/test").as_ref()], cx).await
    }

    fn init_test_settings(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            ContextServerSettings::register(cx);
        });
    }

    struct FakeTransport {
        messages: Arc<RwLock<Vec<String>>>,
        on_request:
            Arc<dyn Fn(Option<RequestType>, serde_json::Value) -> Option<String> + Send + Sync>,
    }

    impl FakeTransport {
        fn new(
            on_request: impl Fn(Option<RequestType>, serde_json::Value) -> Option<String>
            + 'static
            + Send
            + Sync,
        ) -> Self {
            Self {
                messages: Arc::new(RwLock::new(Vec::new())),
                on_request: Arc::new(on_request),
            }
        }
    }

    #[async_trait::async_trait]
    impl Transport for FakeTransport {
        async fn send(&self, message: String) -> Result<()> {
            println!("Got {}", &message);
            if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&message) {
                if let Some(method) = msg.get("method") {
                    let request_type = method
                        .as_str()
                        .and_then(|method| types::RequestType::try_from(method).ok());
                    println!(
                        "Request type: {:?}",
                        request_type.as_ref().map(|s| s.as_str())
                    );
                    if let Some(response) = (self.on_request.as_ref())(request_type, msg) {
                        println!("Response with {}", &response);
                        self.messages.write().push(response);
                    }
                }
            }
            Ok(())
        }

        fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
            let messages = self.messages.clone();
            println!("Messages: {:?}", &messages);
            Box::pin(futures::stream::unfold(messages, |messages| async move {
                let message = {
                    let mut messages = messages.write();
                    if messages.is_empty() {
                        return None;
                    }
                    messages.remove(0)
                };
                println!("Receiving: {}", &message);
                Some((message, messages))
            }))
        }

        fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
            Box::pin(futures::stream::empty())
        }
    }
}
