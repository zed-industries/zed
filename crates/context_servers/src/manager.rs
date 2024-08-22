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

use collections::{HashMap, HashSet};
use gpui::{AppContext, AsyncAppContext, Context, EventEmitter, Global, Model, ModelContext, Task};
use log;
use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsStore};
use std::path::Path;
use std::sync::Arc;

use crate::{
    client::{self, Client},
    types,
};

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ContextServerSettings {
    pub servers: Vec<ServerConfig>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ServerConfig {
    pub id: String,
    pub executable: String,
    pub args: Vec<String>,
}

impl Settings for ContextServerSettings {
    const KEY: Option<&'static str> = Some("experimental.context_servers");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

pub struct ContextServer {
    pub id: String,
    pub config: ServerConfig,
    pub client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
}

impl ContextServer {
    fn new(config: ServerConfig) -> Self {
        Self {
            id: config.id.clone(),
            config,
            client: RwLock::new(None),
        }
    }

    async fn start(&self, cx: &AsyncAppContext) -> anyhow::Result<()> {
        log::info!("starting context server {}", self.config.id);
        let client = Client::new(
            client::ContextServerId(self.config.id.clone()),
            client::ModelContextServerBinary {
                executable: Path::new(&self.config.executable).to_path_buf(),
                args: self.config.args.clone(),
                env: None,
            },
            cx.clone(),
        )?;

        let protocol = crate::protocol::ModelContextProtocol::new(client);
        let client_info = types::EntityInfo {
            name: "Zed".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        let initialized_protocol = protocol.initialize(client_info).await?;

        log::debug!(
            "context server {} initialized: {:?}",
            self.config.id,
            initialized_protocol.initialize,
        );

        *self.client.write() = Some(Arc::new(initialized_protocol));
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut client = self.client.write();
        if let Some(protocol) = client.take() {
            drop(protocol);
        }
        Ok(())
    }
}

/// A Context server manager manages the starting and stopping
/// of all servers. To obtain a server to interact with, a crate
/// must go through the `GlobalContextServerManager` which holds
/// a model to the ContextServerManager.
pub struct ContextServerManager {
    servers: HashMap<String, Arc<ContextServer>>,
    pending_servers: HashSet<String>,
}

pub enum Event {
    ServerStarted { server_id: String },
    ServerStopped { server_id: String },
}

impl Global for ContextServerManager {}
impl EventEmitter<Event> for ContextServerManager {}

impl ContextServerManager {
    pub fn new() -> Self {
        Self {
            servers: HashMap::default(),
            pending_servers: HashSet::default(),
        }
    }
    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalContextServerManager>().0.clone()
    }

    pub fn add_server(
        &mut self,
        config: ServerConfig,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        let server_id = config.id.clone();
        let server_id2 = config.id.clone();

        if self.servers.contains_key(&server_id) || self.pending_servers.contains(&server_id) {
            return Task::ready(Ok(()));
        }

        let task = cx.spawn(|this, mut cx| async move {
            let server = Arc::new(ContextServer::new(config));
            server.start(&cx).await?;
            this.update(&mut cx, |this, cx| {
                this.servers.insert(server_id.clone(), server);
                this.pending_servers.remove(&server_id);
                cx.emit(Event::ServerStarted {
                    server_id: server_id.clone(),
                });
            })?;
            Ok(())
        });

        self.pending_servers.insert(server_id2);
        task
    }

    pub fn get_server(&self, id: &str) -> Option<Arc<ContextServer>> {
        self.servers.get(id).cloned()
    }

    pub fn remove_server(
        &mut self,
        id: &str,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        let id = id.to_string();
        cx.spawn(|this, mut cx| async move {
            if let Some(server) = this.update(&mut cx, |this, _cx| this.servers.remove(&id))? {
                server.stop().await?;
            }
            this.update(&mut cx, |this, cx| {
                this.pending_servers.remove(&id);
                cx.emit(Event::ServerStopped {
                    server_id: id.clone(),
                })
            })?;
            Ok(())
        })
    }

    pub fn restart_server(
        &mut self,
        id: &str,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        let id = id.to_string();
        cx.spawn(|this, mut cx| async move {
            if let Some(server) = this.update(&mut cx, |this, _cx| this.servers.remove(&id))? {
                server.stop().await?;
                let config = server.config.clone();
                let new_server = Arc::new(ContextServer::new(config));
                new_server.start(&cx).await?;
                this.update(&mut cx, |this, cx| {
                    this.servers.insert(id.clone(), new_server);
                    cx.emit(Event::ServerStopped {
                        server_id: id.clone(),
                    });
                    cx.emit(Event::ServerStarted {
                        server_id: id.clone(),
                    });
                })?;
            }
            Ok(())
        })
    }

    pub fn servers(&self) -> Vec<Arc<ContextServer>> {
        self.servers.values().cloned().collect()
    }

    pub fn model(cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_cx| ContextServerManager::new())
    }
}

pub struct GlobalContextServerManager(Model<ContextServerManager>);
impl Global for GlobalContextServerManager {}

impl GlobalContextServerManager {
    fn register(cx: &mut AppContext) {
        let model = ContextServerManager::model(cx);
        cx.set_global(Self(model));
    }
}

pub fn init(cx: &mut AppContext) {
    ContextServerSettings::register(cx);
    GlobalContextServerManager::register(cx);
    cx.observe_global::<SettingsStore>(|cx| {
        let manager = ContextServerManager::global(cx);
        cx.update_model(&manager, |manager, cx| {
            let settings = ContextServerSettings::get_global(cx);
            let current_servers: HashMap<String, ServerConfig> = manager
                .servers()
                .into_iter()
                .map(|server| (server.id.clone(), server.config.clone()))
                .collect();

            let new_servers = settings
                .servers
                .iter()
                .map(|config| (config.id.clone(), config.clone()))
                .collect::<HashMap<_, _>>();

            let servers_to_add = new_servers
                .values()
                .filter(|config| !current_servers.contains_key(&config.id))
                .cloned()
                .collect::<Vec<_>>();

            let servers_to_remove = current_servers
                .keys()
                .filter(|id| !new_servers.contains_key(*id))
                .cloned()
                .collect::<Vec<_>>();

            log::trace!("servers_to_add={:?}", servers_to_add);
            for config in servers_to_add {
                manager.add_server(config, cx).detach_and_log_err(cx);
            }

            for id in servers_to_remove {
                manager.remove_server(&id, cx).detach_and_log_err(cx);
            }
        })
    })
    .detach();
}
