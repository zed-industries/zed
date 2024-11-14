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

use anyhow::{bail, Result};
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use gpui::{AsyncAppContext, EventEmitter, Model, ModelContext, Subscription, Task, WeakModel};
use log;
use parking_lot::RwLock;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsStore};
use util::ResultExt as _;

use crate::{
    client::{self, Client},
    types, ContextServerFactoryRegistry, CONTEXT_SERVERS_NAMESPACE,
};

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ContextServerSettings {
    #[serde(default)]
    pub context_servers: HashMap<Arc<str>, ServerConfig>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct ServerConfig {
    pub command: Option<ServerCommand>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ServerCommand {
    pub path: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

impl Settings for ContextServerSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

pub struct ContextServer {
    pub id: Arc<str>,
    pub config: Arc<ServerConfig>,
    pub client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
}

impl ContextServer {
    pub fn new(id: Arc<str>, config: Arc<ServerConfig>) -> Self {
        Self {
            id,
            config,
            client: RwLock::new(None),
        }
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

    pub async fn start(self: Arc<Self>, cx: &AsyncAppContext) -> Result<()> {
        log::info!("starting context server {}", self.id);
        let Some(command) = &self.config.command else {
            bail!("no command specified for server {}", self.id);
        };
        let client = Client::new(
            client::ContextServerId(self.id.clone()),
            client::ModelContextServerBinary {
                executable: Path::new(&command.path).to_path_buf(),
                args: command.args.clone(),
                env: command.env.clone(),
            },
            cx.clone(),
        )?;

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
    project: Model<Project>,
    registry: Model<ContextServerFactoryRegistry>,
    update_servers_task: Option<Task<Result<()>>>,
    needs_server_update: bool,
    _subscriptions: Vec<Subscription>,
}

pub enum Event {
    ServerStarted { server_id: Arc<str> },
    ServerStopped { server_id: Arc<str> },
}

impl EventEmitter<Event> for ContextServerManager {}

impl ContextServerManager {
    pub fn new(
        registry: Model<ContextServerFactoryRegistry>,
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
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
            update_servers_task: None,
        };
        this.available_context_servers_changed(cx);
        this
    }

    fn available_context_servers_changed(&mut self, cx: &mut ModelContext<Self>) {
        if self.update_servers_task.is_some() {
            self.needs_server_update = true;
        } else {
            self.update_servers_task = Some(cx.spawn(|this, mut cx| async move {
                this.update(&mut cx, |this, _| {
                    this.needs_server_update = false;
                })?;

                Self::maintain_servers(this.clone(), cx.clone()).await?;

                this.update(&mut cx, |this, cx| {
                    let has_any_context_servers = !this.servers().is_empty();
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

    pub fn restart_server(
        &mut self,
        id: &Arc<str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        let id = id.clone();
        cx.spawn(|this, mut cx| async move {
            if let Some(server) = this.update(&mut cx, |this, _cx| this.servers.remove(&id))? {
                server.stop()?;
                let config = server.config();
                let new_server = Arc::new(ContextServer::new(id.clone(), config));
                new_server.clone().start(&cx).await?;
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
        self.servers
            .values()
            .filter(|server| server.client().is_some())
            .cloned()
            .collect()
    }

    async fn maintain_servers(this: WeakModel<Self>, mut cx: AsyncAppContext) -> Result<()> {
        let mut desired_servers = HashMap::default();

        let (registry, project) = this.update(&mut cx, |this, cx| {
            let location = this.project.read(cx).worktrees(cx).next().map(|worktree| {
                settings::SettingsLocation {
                    worktree_id: worktree.read(cx).id(),
                    path: Path::new(""),
                }
            });
            let settings = ContextServerSettings::get(location, cx);
            desired_servers = settings.context_servers.clone();

            (this.registry.clone(), this.project.clone())
        })?;

        for (id, factory) in
            registry.read_with(&cx, |registry, _| registry.context_server_factories())?
        {
            let config = desired_servers.entry(id).or_default();
            if config.command.is_none() {
                if let Some(extension_command) = factory(project.clone(), &cx).await.log_err() {
                    config.command = Some(extension_command);
                }
            }
        }

        let mut servers_to_start = HashMap::default();
        let mut servers_to_stop = HashMap::default();

        this.update(&mut cx, |this, _cx| {
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
                    let config = Arc::new(config);
                    let server = Arc::new(ContextServer::new(id.clone(), config));
                    servers_to_start.insert(id.clone(), server.clone());
                    let old_server = this.servers.insert(id.clone(), server);
                    if let Some(old_server) = old_server {
                        servers_to_stop.insert(id, old_server);
                    }
                }
            }
        })?;

        for (id, server) in servers_to_stop {
            server.stop().log_err();
            this.update(&mut cx, |_, cx| {
                cx.emit(Event::ServerStopped { server_id: id })
            })?;
        }

        for (id, server) in servers_to_start {
            if server.start(&cx).await.log_err().is_some() {
                this.update(&mut cx, |_, cx| {
                    cx.emit(Event::ServerStarted { server_id: id })
                })?;
            }
        }

        Ok(())
    }
}
