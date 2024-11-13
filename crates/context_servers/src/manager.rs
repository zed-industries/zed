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
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{bail, Result};
use async_trait::async_trait;
use collections::{HashMap, HashSet};
use futures::{channel::mpsc, Future, FutureExt};
use gpui::{AsyncAppContext, EventEmitter, Model, ModelContext, Task};
use log;
use parking_lot::RwLock;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsStore};
use smol::stream::StreamExt;

use crate::{
    client::{self, Client},
    types, ContextServerFactoryRegistry,
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

#[async_trait(?Send)]
pub trait ContextServer: Send + Sync + 'static {
    fn id(&self) -> Arc<str>;
    fn config(&self) -> Arc<ServerConfig>;
    fn client(&self) -> Option<Arc<crate::protocol::InitializedContextServerProtocol>>;
    fn start<'a>(
        self: Arc<Self>,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<()>>>>;
    fn stop(&self) -> Result<()>;
}

pub struct NativeContextServer {
    pub id: Arc<str>,
    pub config: Arc<ServerConfig>,
    pub client: RwLock<Option<Arc<crate::protocol::InitializedContextServerProtocol>>>,
}

impl NativeContextServer {
    pub fn new(id: Arc<str>, config: Arc<ServerConfig>) -> Self {
        Self {
            id,
            config,
            client: RwLock::new(None),
        }
    }
}

#[async_trait(?Send)]
impl ContextServer for NativeContextServer {
    fn id(&self) -> Arc<str> {
        self.id.clone()
    }

    fn config(&self) -> Arc<ServerConfig> {
        self.config.clone()
    }

    fn client(&self) -> Option<Arc<crate::protocol::InitializedContextServerProtocol>> {
        self.client.read().clone()
    }

    fn start<'a>(
        self: Arc<Self>,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<()>>>> {
        async move {
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
        .boxed_local()
    }

    fn stop(&self) -> Result<()> {
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
    project: Model<Project>,
    servers: HashMap<Arc<str>, (ServerCommand, Arc<dyn ContextServer>)>,
    pending_servers: HashSet<Arc<str>>,
    registry: Model<ContextServerFactoryRegistry>,
    _maintain_context_servers: Task<Result<()>>,
    _subscriptions: [gpui::Subscription; 2],
}

pub enum Event {
    ServerStarted { server_id: Arc<str> },
    ServerStopped { server_id: Arc<str> },
}

impl EventEmitter<Event> for ContextServerManager {}

impl ContextServerManager {
    pub fn new(
        project: Model<Project>,
        registry: Model<ContextServerFactoryRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let (tx, mut rx) = mpsc::unbounded::<()>();
        Self {
            project,
            servers: Default::default(),
            pending_servers: HashSet::default(),
            _subscriptions: [
                cx.observe(&registry, {
                    let tx = tx.clone();
                    move |_this, _, _cx| {
                        tx.unbounded_send(()).ok();
                    }
                }),
                cx.observe_global::<SettingsStore>({
                    let tx = tx.clone();
                    move |_this, _cx| {
                        tx.unbounded_send(()).ok();
                    }
                }),
            ],
            registry,
            _maintain_context_servers: cx.spawn(|this, mut cx| async move {
                while let Some(_) = rx.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.registered_servers_changed(cx);
                    })?;
                }
                Ok(())
            }),
        }
    }

    pub fn add_server(
        &mut self,
        server: Arc<dyn ContextServer>,
        cx: &ModelContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        let server_id = server.id();

        if self.servers.contains_key(&server_id) || self.pending_servers.contains(&server_id) {
            return Task::ready(Ok(()));
        }

        let task = {
            let server_id = server_id.clone();
            cx.spawn(|this, mut cx| async move {
                server.clone().start(&cx).await?;
                this.update(&mut cx, |this, cx| {
                    this.servers.insert(server_id.clone(), server);
                    this.pending_servers.remove(&server_id);
                    cx.emit(Event::ServerStarted {
                        server_id: server_id.clone(),
                    });
                })?;
                Ok(())
            })
        };

        self.pending_servers.insert(server_id);
        task
    }

    pub fn get_server(&self, id: &str) -> Option<Arc<dyn ContextServer>> {
        self.servers.get(id).cloned()
    }

    pub fn remove_server(
        &mut self,
        id: &Arc<str>,
        cx: &ModelContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        let id = id.clone();
        cx.spawn(|this, mut cx| async move {
            if let Some(server) =
                this.update(&mut cx, |this, _cx| this.servers.remove(id.as_ref()))?
            {
                server.stop()?;
            }
            this.update(&mut cx, |this, cx| {
                this.pending_servers.remove(id.as_ref());
                cx.emit(Event::ServerStopped {
                    server_id: id.clone(),
                })
            })?;
            Ok(())
        })
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
                let new_server = Arc::new(NativeContextServer::new(id.clone(), config));
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

    pub fn servers(&self) -> Vec<Arc<dyn ContextServer>> {
        self.servers.values().cloned().collect()
    }

    fn registered_servers_changed(
        &mut self,
        cx: &mut ModelContext<ContextServerManager>,
    ) -> Task<()> {
        let worktree_id = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).id());
        let settings = ContextServerSettings::get(
            worktree_id.map(|worktree_id| settings::SettingsLocation {
                worktree_id,
                path: Path::new(""),
            }),
            cx,
        );

        let registry = self.registry.read(cx);

        let mut settings_iter = settings.context_servers.iter().peekable();
        let mut registry_iter = registry.context_servers.iter().peekable();

        // loop {
        //     let mut setting_command = None;
        //     let mut registered_command = None;
        //     let mut server_id;
        //     match (settings_iter.peek(), registry_iter.peek()) {
        //         (None, None) => break,
        //         (None, Some((id, value))) => {
        //             server_id = id.clone();
        //             registered_command = value;
        //         }
        //         (Some(_), None) => continue,
        //         (Some(_), Some(_)) => continue,
        //     }
        // }
    }

    pub fn maintain_servers(&mut self, settings: &ContextServerSettings, cx: &ModelContext<Self>) {
        let current_servers = self
            .servers()
            .into_iter()
            .map(|server| (server.id(), server.config()))
            .collect::<HashMap<_, _>>();

        let new_servers = settings
            .context_servers
            .iter()
            .map(|(id, config)| (id.clone(), config.clone()))
            .collect::<HashMap<_, _>>();

        let servers_to_add = new_servers
            .iter()
            .filter(|(id, _)| !current_servers.contains_key(id.as_ref()))
            .map(|(id, config)| (id.clone(), config.clone()))
            .collect::<Vec<_>>();

        let servers_to_remove = current_servers
            .keys()
            .filter(|id| !new_servers.contains_key(id.as_ref()))
            .cloned()
            .collect::<Vec<_>>();

        log::trace!("servers_to_add={:?}", servers_to_add);
        for (id, config) in servers_to_add {
            if config.command.is_some() {
                let server = Arc::new(NativeContextServer::new(id, Arc::new(config)));
                self.add_server(server, cx).detach_and_log_err(cx);
            }
        }

        for id in servers_to_remove {
            self.remove_server(&id, cx).detach_and_log_err(cx);
        }
    }
}
