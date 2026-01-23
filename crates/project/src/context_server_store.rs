pub mod extension;
pub mod registry;

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use futures::{FutureExt as _, future::join_all};
use gpui::{App, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity, actions};
use registry::ContextServerDescriptorRegistry;
use remote::RemoteClient;
use rpc::{AnyProtoClient, TypedEnvelope, proto};
use settings::{Settings as _, SettingsStore};
use util::{ResultExt as _, rel_path::RelPath};

use crate::{
    Project,
    project_settings::{ContextServerSettings, ProjectSettings},
    worktree_store::WorktreeStore,
};

/// Maximum timeout for context server requests
/// Prevents extremely large timeout values from tying up resources indefinitely.
const MAX_TIMEOUT_SECS: u64 = 600; // 10 minutes

pub fn init(cx: &mut App) {
    extension::init(cx);
}

actions!(
    context_server,
    [
        /// Restarts the context server.
        Restart
    ]
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextServerStatus {
    Starting,
    Running,
    Stopped,
    Error(Arc<str>),
}

impl ContextServerStatus {
    fn from_state(state: &ContextServerState) -> Self {
        match state {
            ContextServerState::Starting { .. } => ContextServerStatus::Starting,
            ContextServerState::Running { .. } => ContextServerStatus::Running,
            ContextServerState::Stopped { .. } => ContextServerStatus::Stopped,
            ContextServerState::Error { error, .. } => ContextServerStatus::Error(error.clone()),
        }
    }
}

enum ContextServerState {
    Starting {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        _task: Task<()>,
    },
    Running {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
    },
    Stopped {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
    },
    Error {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        error: Arc<str>,
    },
}

impl ContextServerState {
    pub fn server(&self) -> Arc<ContextServer> {
        match self {
            ContextServerState::Starting { server, .. } => server.clone(),
            ContextServerState::Running { server, .. } => server.clone(),
            ContextServerState::Stopped { server, .. } => server.clone(),
            ContextServerState::Error { server, .. } => server.clone(),
        }
    }

    pub fn configuration(&self) -> Arc<ContextServerConfiguration> {
        match self {
            ContextServerState::Starting { configuration, .. } => configuration.clone(),
            ContextServerState::Running { configuration, .. } => configuration.clone(),
            ContextServerState::Stopped { configuration, .. } => configuration.clone(),
            ContextServerState::Error { configuration, .. } => configuration.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContextServerConfiguration {
    Custom {
        command: ContextServerCommand,
        remote: bool,
    },
    Extension {
        command: ContextServerCommand,
        settings: serde_json::Value,
        remote: bool,
    },
    Http {
        url: url::Url,
        headers: HashMap<String, String>,
        timeout: Option<u64>,
    },
}

impl ContextServerConfiguration {
    pub fn command(&self) -> Option<&ContextServerCommand> {
        match self {
            ContextServerConfiguration::Custom { command, .. } => Some(command),
            ContextServerConfiguration::Extension { command, .. } => Some(command),
            ContextServerConfiguration::Http { .. } => None,
        }
    }

    pub fn remote(&self) -> bool {
        match self {
            ContextServerConfiguration::Custom { remote, .. } => *remote,
            ContextServerConfiguration::Extension { remote, .. } => *remote,
            ContextServerConfiguration::Http { .. } => false,
        }
    }

    pub async fn from_settings(
        settings: ContextServerSettings,
        id: ContextServerId,
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        cx: &AsyncApp,
    ) -> Option<Self> {
        match settings {
            ContextServerSettings::Stdio {
                enabled: _,
                command,
                remote,
            } => Some(ContextServerConfiguration::Custom { command, remote }),
            ContextServerSettings::Extension {
                enabled: _,
                settings,
                remote,
            } => {
                let descriptor =
                    cx.update(|cx| registry.read(cx).context_server_descriptor(&id.0))?;

                match descriptor.command(worktree_store, cx).await {
                    Ok(command) => Some(ContextServerConfiguration::Extension {
                        command,
                        settings,
                        remote,
                    }),
                    Err(e) => {
                        log::error!(
                            "Failed to create context server configuration from settings: {e:#}"
                        );
                        None
                    }
                }
            }
            ContextServerSettings::Http {
                enabled: _,
                url,
                headers: auth,
                timeout,
            } => {
                let url = url::Url::parse(&url).log_err()?;
                Some(ContextServerConfiguration::Http {
                    url,
                    headers: auth,
                    timeout,
                })
            }
        }
    }
}

pub type ContextServerFactory =
    Box<dyn Fn(ContextServerId, Arc<ContextServerConfiguration>) -> Arc<ContextServer>>;

enum ContextServerStoreState {
    Local {
        downstream_client: Option<(u64, AnyProtoClient)>,
        is_headless: bool,
    },
    Remote {
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
    },
}

pub struct ContextServerStore {
    state: ContextServerStoreState,
    context_server_settings: HashMap<Arc<str>, ContextServerSettings>,
    servers: HashMap<ContextServerId, ContextServerState>,
    worktree_store: Entity<WorktreeStore>,
    project: Option<WeakEntity<Project>>,
    registry: Entity<ContextServerDescriptorRegistry>,
    update_servers_task: Option<Task<Result<()>>>,
    context_server_factory: Option<ContextServerFactory>,
    needs_server_update: bool,
    _subscriptions: Vec<Subscription>,
}

pub enum Event {
    ServerStatusChanged {
        server_id: ContextServerId,
        status: ContextServerStatus,
    },
}

impl EventEmitter<Event> for ContextServerStore {}

impl ContextServerStore {
    pub fn local(
        worktree_store: Entity<WorktreeStore>,
        weak_project: Option<WeakEntity<Project>>,
        headless: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(
            !headless,
            None,
            ContextServerDescriptorRegistry::default_global(cx),
            worktree_store,
            weak_project,
            ContextServerStoreState::Local {
                downstream_client: None,
                is_headless: headless,
            },
            cx,
        )
    }

    pub fn remote(
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
        worktree_store: Entity<WorktreeStore>,
        weak_project: Option<WeakEntity<Project>>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(
            true,
            None,
            ContextServerDescriptorRegistry::default_global(cx),
            worktree_store,
            weak_project,
            ContextServerStoreState::Remote {
                project_id,
                upstream_client,
            },
            cx,
        )
    }

    pub fn init_headless(session: &AnyProtoClient) {
        session.add_entity_request_handler(Self::handle_get_context_server_command);
    }

    pub fn shared(&mut self, project_id: u64, client: AnyProtoClient) {
        if let ContextServerStoreState::Local {
            downstream_client, ..
        } = &mut self.state
        {
            *downstream_client = Some((project_id, client));
        }
    }

    pub fn is_remote_project(&self) -> bool {
        matches!(self.state, ContextServerStoreState::Remote { .. })
    }

    /// Returns all configured context server ids, excluding the ones that are disabled
    pub fn configured_server_ids(&self) -> Vec<ContextServerId> {
        self.context_server_settings
            .iter()
            .filter(|(_, settings)| settings.enabled())
            .map(|(id, _)| ContextServerId(id.clone()))
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        weak_project: Option<WeakEntity<Project>>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(
            false,
            None,
            registry,
            worktree_store,
            weak_project,
            ContextServerStoreState::Local {
                downstream_client: None,
                is_headless: false,
            },
            cx,
        )
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_maintain_server_loop(
        context_server_factory: Option<ContextServerFactory>,
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        weak_project: Option<WeakEntity<Project>>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(
            true,
            context_server_factory,
            registry,
            worktree_store,
            weak_project,
            ContextServerStoreState::Local {
                downstream_client: None,
                is_headless: false,
            },
            cx,
        )
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_context_server_factory(&mut self, factory: ContextServerFactory) {
        self.context_server_factory = Some(factory);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn registry(&self) -> &Entity<ContextServerDescriptorRegistry> {
        &self.registry
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_start_server(&mut self, server: Arc<ContextServer>, cx: &mut Context<Self>) {
        let configuration = Arc::new(ContextServerConfiguration::Custom {
            command: ContextServerCommand {
                path: "test".into(),
                args: vec![],
                env: None,
                timeout: None,
            },
            remote: false,
        });
        self.run_server(server, configuration, cx);
    }

    fn new_internal(
        maintain_server_loop: bool,
        context_server_factory: Option<ContextServerFactory>,
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        weak_project: Option<WeakEntity<Project>>,
        state: ContextServerStoreState,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = vec![cx.observe_global::<SettingsStore>(move |this, cx| {
            let settings =
                &Self::resolve_project_settings(&this.worktree_store, cx).context_servers;
            if &this.context_server_settings == settings {
                return;
            }
            this.context_server_settings = settings.clone();
            if maintain_server_loop {
                this.available_context_servers_changed(cx);
            }
        })];

        if maintain_server_loop {
            subscriptions.push(cx.observe(&registry, |this, _registry, cx| {
                this.available_context_servers_changed(cx);
            }));
        }

        let mut this = Self {
            state,
            _subscriptions: subscriptions,
            context_server_settings: Self::resolve_project_settings(&worktree_store, cx)
                .context_servers
                .clone(),
            worktree_store,
            project: weak_project,
            registry,
            needs_server_update: false,
            servers: HashMap::default(),
            update_servers_task: None,
            context_server_factory,
        };
        if maintain_server_loop {
            this.available_context_servers_changed(cx);
        }
        this
    }

    pub fn get_server(&self, id: &ContextServerId) -> Option<Arc<ContextServer>> {
        self.servers.get(id).map(|state| state.server())
    }

    pub fn get_running_server(&self, id: &ContextServerId) -> Option<Arc<ContextServer>> {
        if let Some(ContextServerState::Running { server, .. }) = self.servers.get(id) {
            Some(server.clone())
        } else {
            None
        }
    }

    pub fn status_for_server(&self, id: &ContextServerId) -> Option<ContextServerStatus> {
        self.servers.get(id).map(ContextServerStatus::from_state)
    }

    pub fn configuration_for_server(
        &self,
        id: &ContextServerId,
    ) -> Option<Arc<ContextServerConfiguration>> {
        self.servers.get(id).map(|state| state.configuration())
    }

    pub fn server_ids(&self, cx: &App) -> HashSet<ContextServerId> {
        self.servers
            .keys()
            .cloned()
            .chain(
                self.registry
                    .read(cx)
                    .context_server_descriptors()
                    .into_iter()
                    .map(|(id, _)| ContextServerId(id)),
            )
            .collect()
    }

    pub fn running_servers(&self) -> Vec<Arc<ContextServer>> {
        self.servers
            .values()
            .filter_map(|state| {
                if let ContextServerState::Running { server, .. } = state {
                    Some(server.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn start_server(&mut self, server: Arc<ContextServer>, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let this = this.upgrade().context("Context server store dropped")?;
            let settings = this
                .update(cx, |this, _| {
                    this.context_server_settings.get(&server.id().0).cloned()
                })
                .context("Failed to get context server settings")?;

            if !settings.enabled() {
                return anyhow::Ok(());
            }

            let (registry, worktree_store) = this.update(cx, |this, _| {
                (this.registry.clone(), this.worktree_store.clone())
            });
            let configuration = ContextServerConfiguration::from_settings(
                settings,
                server.id(),
                registry,
                worktree_store,
                cx,
            )
            .await
            .context("Failed to create context server configuration")?;

            this.update(cx, |this, cx| {
                this.run_server(server, Arc::new(configuration), cx)
            });
            Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn stop_server(&mut self, id: &ContextServerId, cx: &mut Context<Self>) -> Result<()> {
        if matches!(
            self.servers.get(id),
            Some(ContextServerState::Stopped { .. })
        ) {
            return Ok(());
        }

        let state = self
            .servers
            .remove(id)
            .context("Context server not found")?;

        let server = state.server();
        let configuration = state.configuration();
        let mut result = Ok(());
        if let ContextServerState::Running { server, .. } = &state {
            result = server.stop();
        }
        drop(state);

        self.update_server_state(
            id.clone(),
            ContextServerState::Stopped {
                configuration,
                server,
            },
            cx,
        );

        result
    }

    fn run_server(
        &mut self,
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut Context<Self>,
    ) {
        let id = server.id();
        if matches!(
            self.servers.get(&id),
            Some(ContextServerState::Starting { .. } | ContextServerState::Running { .. })
        ) {
            self.stop_server(&id, cx).log_err();
        }
        let task = cx.spawn({
            let id = server.id();
            let server = server.clone();
            let configuration = configuration.clone();

            async move |this, cx| {
                match server.clone().start(cx).await {
                    Ok(_) => {
                        debug_assert!(server.client().is_some());

                        this.update(cx, |this, cx| {
                            this.update_server_state(
                                id.clone(),
                                ContextServerState::Running {
                                    server,
                                    configuration,
                                },
                                cx,
                            )
                        })
                        .log_err()
                    }
                    Err(err) => {
                        log::error!("{} context server failed to start: {}", id, err);
                        this.update(cx, |this, cx| {
                            this.update_server_state(
                                id.clone(),
                                ContextServerState::Error {
                                    configuration,
                                    server,
                                    error: err.to_string().into(),
                                },
                                cx,
                            )
                        })
                        .log_err()
                    }
                };
            }
        });

        self.update_server_state(
            id.clone(),
            ContextServerState::Starting {
                configuration,
                _task: task,
                server,
            },
            cx,
        );
    }

    fn remove_server(&mut self, id: &ContextServerId, cx: &mut Context<Self>) -> Result<()> {
        let state = self
            .servers
            .remove(id)
            .context("Context server not found")?;
        drop(state);
        cx.emit(Event::ServerStatusChanged {
            server_id: id.clone(),
            status: ContextServerStatus::Stopped,
        });
        Ok(())
    }

    async fn create_context_server(
        this: WeakEntity<Self>,
        id: ContextServerId,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut AsyncApp,
    ) -> Result<(Arc<ContextServer>, Arc<ContextServerConfiguration>)> {
        let remote = configuration.remote();
        let needs_remote_command = match configuration.as_ref() {
            ContextServerConfiguration::Custom { .. }
            | ContextServerConfiguration::Extension { .. } => remote,
            ContextServerConfiguration::Http { .. } => false,
        };

        let (remote_state, is_remote_project) = this.update(cx, |this, _| {
            let remote_state = match &this.state {
                ContextServerStoreState::Remote {
                    project_id,
                    upstream_client,
                } if needs_remote_command => Some((*project_id, upstream_client.clone())),
                _ => None,
            };
            (remote_state, this.is_remote_project())
        })?;

        let root_path: Option<Arc<Path>> = this.update(cx, |this, cx| {
            this.project
                .as_ref()
                .and_then(|project| {
                    project
                        .read_with(cx, |project, cx| project.active_project_directory(cx))
                        .ok()
                        .flatten()
                })
                .or_else(|| {
                    this.worktree_store.read_with(cx, |store, cx| {
                        store.visible_worktrees(cx).fold(None, |acc, item| {
                            if acc.is_none() {
                                item.read(cx).root_dir()
                            } else {
                                acc
                            }
                        })
                    })
                })
        })?;

        let configuration = if let Some((project_id, upstream_client)) = remote_state {
            let root_dir = root_path.as_ref().map(|p| p.display().to_string());

            let response = upstream_client
                .update(cx, |client, _| {
                    client
                        .proto_client()
                        .request(proto::GetContextServerCommand {
                            project_id,
                            server_id: id.0.to_string(),
                            root_dir: root_dir.clone(),
                        })
                })
                .await?;

            let remote_command = upstream_client.update(cx, |client, _| {
                client.build_command(
                    Some(response.path),
                    &response.args,
                    &response.env.into_iter().collect(),
                    root_dir,
                    None,
                )
            })?;

            let command = ContextServerCommand {
                path: remote_command.program.into(),
                args: remote_command.args,
                env: Some(remote_command.env.into_iter().collect()),
                timeout: None,
            };

            Arc::new(ContextServerConfiguration::Custom { command, remote })
        } else {
            configuration
        };

        let server: Arc<ContextServer> = this.update(cx, |this, cx| {
            let global_timeout =
                Self::resolve_project_settings(&this.worktree_store, cx).context_server_timeout;

            if let Some(factory) = this.context_server_factory.as_ref() {
                return anyhow::Ok(factory(id.clone(), configuration.clone()));
            }

            match configuration.as_ref() {
                ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout,
                } => anyhow::Ok(Arc::new(ContextServer::http(
                    id,
                    url,
                    headers.clone(),
                    cx.http_client(),
                    cx.background_executor().clone(),
                    Some(Duration::from_secs(
                        timeout.unwrap_or(global_timeout).min(MAX_TIMEOUT_SECS),
                    )),
                )?)),
                _ => {
                    let mut command = configuration
                        .command()
                        .context("Missing command configuration for stdio context server")?
                        .clone();
                    command.timeout = Some(
                        command
                            .timeout
                            .unwrap_or(global_timeout)
                            .min(MAX_TIMEOUT_SECS),
                    );

                    // Don't pass remote paths as working directory for locally-spawned processes
                    let working_directory = if is_remote_project { None } else { root_path };
                    anyhow::Ok(Arc::new(ContextServer::stdio(
                        id,
                        command,
                        working_directory,
                    )))
                }
            }
        })??;

        Ok((server, configuration))
    }

    async fn handle_get_context_server_command(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetContextServerCommand>,
        mut cx: AsyncApp,
    ) -> Result<proto::ContextServerCommand> {
        let server_id = ContextServerId(envelope.payload.server_id.into());

        let (settings, registry, worktree_store) = this.update(&mut cx, |this, inner_cx| {
            let ContextServerStoreState::Local {
                is_headless: true, ..
            } = &this.state
            else {
                anyhow::bail!("unexpected GetContextServerCommand request in a non-local project");
            };

            let settings = this
                .context_server_settings
                .get(&server_id.0)
                .cloned()
                .or_else(|| {
                    this.registry
                        .read(inner_cx)
                        .context_server_descriptor(&server_id.0)
                        .map(|_| ContextServerSettings::default_extension())
                })
                .with_context(|| format!("context server `{}` not found", server_id))?;

            anyhow::Ok((settings, this.registry.clone(), this.worktree_store.clone()))
        })?;

        let configuration = ContextServerConfiguration::from_settings(
            settings,
            server_id.clone(),
            registry,
            worktree_store,
            &cx,
        )
        .await
        .with_context(|| format!("failed to build configuration for `{}`", server_id))?;

        let command = configuration
            .command()
            .context("context server has no command (HTTP servers don't need RPC)")?;

        Ok(proto::ContextServerCommand {
            path: command.path.display().to_string(),
            args: command.args.clone(),
            env: command
                .env
                .clone()
                .map(|env| env.into_iter().collect())
                .unwrap_or_default(),
        })
    }

    fn resolve_project_settings<'a>(
        worktree_store: &'a Entity<WorktreeStore>,
        cx: &'a App,
    ) -> &'a ProjectSettings {
        let location = worktree_store
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| settings::SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path: RelPath::empty(),
            });
        ProjectSettings::get(location, cx)
    }

    fn update_server_state(
        &mut self,
        id: ContextServerId,
        state: ContextServerState,
        cx: &mut Context<Self>,
    ) {
        let status = ContextServerStatus::from_state(&state);
        self.servers.insert(id.clone(), state);
        cx.emit(Event::ServerStatusChanged {
            server_id: id,
            status,
        });
    }

    fn available_context_servers_changed(&mut self, cx: &mut Context<Self>) {
        if self.update_servers_task.is_some() {
            self.needs_server_update = true;
        } else {
            self.needs_server_update = false;
            self.update_servers_task = Some(cx.spawn(async move |this, cx| {
                if let Err(err) = Self::maintain_servers(this.clone(), cx).await {
                    log::error!("Error maintaining context servers: {}", err);
                }

                this.update(cx, |this, cx| {
                    this.update_servers_task.take();
                    if this.needs_server_update {
                        this.available_context_servers_changed(cx);
                    }
                })?;

                Ok(())
            }));
        }
    }

    async fn maintain_servers(this: WeakEntity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let (mut configured_servers, registry, worktree_store) = this.update(cx, |this, _| {
            (
                this.context_server_settings.clone(),
                this.registry.clone(),
                this.worktree_store.clone(),
            )
        })?;

        for (id, _) in registry.read_with(cx, |registry, _| registry.context_server_descriptors()) {
            configured_servers
                .entry(id)
                .or_insert(ContextServerSettings::default_extension());
        }

        let (enabled_servers, disabled_servers): (HashMap<_, _>, HashMap<_, _>) =
            configured_servers
                .into_iter()
                .partition(|(_, settings)| settings.enabled());

        let configured_servers = join_all(enabled_servers.into_iter().map(|(id, settings)| {
            let id = ContextServerId(id);
            ContextServerConfiguration::from_settings(
                settings,
                id.clone(),
                registry.clone(),
                worktree_store.clone(),
                cx,
            )
            .map(move |config| (id, config))
        }))
        .await
        .into_iter()
        .filter_map(|(id, config)| config.map(|config| (id, config)))
        .collect::<HashMap<_, _>>();

        let mut servers_to_start = Vec::new();
        let mut servers_to_remove = HashSet::default();
        let mut servers_to_stop = HashSet::default();

        this.update(cx, |this, _cx| {
            for server_id in this.servers.keys() {
                // All servers that are not in desired_servers should be removed from the store.
                // This can happen if the user removed a server from the context server settings.
                if !configured_servers.contains_key(server_id) {
                    if disabled_servers.contains_key(&server_id.0) {
                        servers_to_stop.insert(server_id.clone());
                    } else {
                        servers_to_remove.insert(server_id.clone());
                    }
                }
            }

            for (id, config) in configured_servers {
                let state = this.servers.get(&id);
                let is_stopped = matches!(state, Some(ContextServerState::Stopped { .. }));
                let existing_config = state.as_ref().map(|state| state.configuration());
                if existing_config.as_deref() != Some(&config) || is_stopped {
                    let config = Arc::new(config);
                    servers_to_start.push((id.clone(), config));
                    if this.servers.contains_key(&id) {
                        servers_to_stop.insert(id);
                    }
                }
            }

            anyhow::Ok(())
        })??;

        this.update(cx, |this, inner_cx| {
            for id in servers_to_stop {
                this.stop_server(&id, inner_cx)?;
            }
            for id in servers_to_remove {
                this.remove_server(&id, inner_cx)?;
            }
            anyhow::Ok(())
        })??;

        for (id, config) in servers_to_start {
            let (server, config) =
                Self::create_context_server(this.clone(), id, config, cx).await?;
            this.update(cx, |this, cx| {
                this.run_server(server, config, cx);
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        FakeFs, Project, context_server_store::registry::ContextServerDescriptor,
        project_settings::ProjectSettings,
    };
    use context_server::test::create_fake_transport;
    use gpui::{AppContext, TestAppContext, UpdateGlobal as _};
    use http_client::{FakeHttpClient, Response};
    use serde_json::json;
    use std::{cell::RefCell, path::PathBuf, rc::Rc};
    use util::path;

    #[gpui::test]
    async fn test_context_server_status(cx: &mut TestAppContext) {
        const SERVER_1_ID: &str = "mcp-1";
        const SERVER_2_ID: &str = "mcp-2";

        let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(
                registry.clone(),
                project.read(cx).worktree_store(),
                Some(project.downgrade()),
                cx,
            )
        });

        let server_1_id = ContextServerId(SERVER_1_ID.into());
        let server_2_id = ContextServerId(SERVER_2_ID.into());

        let server_1 = Arc::new(ContextServer::new(
            server_1_id.clone(),
            Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
        ));
        let server_2 = Arc::new(ContextServer::new(
            server_2_id.clone(),
            Arc::new(create_fake_transport(SERVER_2_ID, cx.executor())),
        ));

        store.update(cx, |store, cx| store.test_start_server(server_1, cx));

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
        });

        store.update(cx, |store, cx| {
            store.test_start_server(server_2.clone(), cx)
        });

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(
                store.read(cx).status_for_server(&server_2_id),
                Some(ContextServerStatus::Running)
            );
        });

        store
            .update(cx, |store, cx| store.stop_server(&server_2_id, cx))
            .unwrap();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(
                store.read(cx).status_for_server(&server_2_id),
                Some(ContextServerStatus::Stopped)
            );
        });
    }

    #[gpui::test]
    async fn test_context_server_status_events(cx: &mut TestAppContext) {
        const SERVER_1_ID: &str = "mcp-1";
        const SERVER_2_ID: &str = "mcp-2";

        let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(
                registry.clone(),
                project.read(cx).worktree_store(),
                Some(project.downgrade()),
                cx,
            )
        });

        let server_1_id = ContextServerId(SERVER_1_ID.into());
        let server_2_id = ContextServerId(SERVER_2_ID.into());

        let server_1 = Arc::new(ContextServer::new(
            server_1_id.clone(),
            Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
        ));
        let server_2 = Arc::new(ContextServer::new(
            server_2_id.clone(),
            Arc::new(create_fake_transport(SERVER_2_ID, cx.executor())),
        ));

        let _server_events = assert_server_events(
            &store,
            vec![
                (server_1_id.clone(), ContextServerStatus::Starting),
                (server_1_id, ContextServerStatus::Running),
                (server_2_id.clone(), ContextServerStatus::Starting),
                (server_2_id.clone(), ContextServerStatus::Running),
                (server_2_id.clone(), ContextServerStatus::Stopped),
            ],
            cx,
        );

        store.update(cx, |store, cx| store.test_start_server(server_1, cx));

        cx.run_until_parked();

        store.update(cx, |store, cx| {
            store.test_start_server(server_2.clone(), cx)
        });

        cx.run_until_parked();

        store
            .update(cx, |store, cx| store.stop_server(&server_2_id, cx))
            .unwrap();
    }

    #[gpui::test(iterations = 25)]
    async fn test_context_server_concurrent_starts(cx: &mut TestAppContext) {
        const SERVER_1_ID: &str = "mcp-1";

        let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(
                registry.clone(),
                project.read(cx).worktree_store(),
                Some(project.downgrade()),
                cx,
            )
        });

        let server_id = ContextServerId(SERVER_1_ID.into());

        let server_with_same_id_1 = Arc::new(ContextServer::new(
            server_id.clone(),
            Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
        ));
        let server_with_same_id_2 = Arc::new(ContextServer::new(
            server_id.clone(),
            Arc::new(create_fake_transport(SERVER_1_ID, cx.executor())),
        ));

        // If we start another server with the same id, we should report that we stopped the previous one
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Stopped),
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );

        store.update(cx, |store, cx| {
            store.test_start_server(server_with_same_id_1.clone(), cx)
        });
        store.update(cx, |store, cx| {
            store.test_start_server(server_with_same_id_2.clone(), cx)
        });

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_id),
                Some(ContextServerStatus::Running)
            );
        });
    }

    #[gpui::test]
    async fn test_context_server_maintain_servers_loop(cx: &mut TestAppContext) {
        const SERVER_1_ID: &str = "mcp-1";
        const SERVER_2_ID: &str = "mcp-2";

        let server_1_id = ContextServerId(SERVER_1_ID.into());
        let server_2_id = ContextServerId(SERVER_2_ID.into());

        let fake_descriptor_1 = Arc::new(FakeContextServerDescriptor::new(SERVER_1_ID));

        let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

        let executor = cx.executor();
        let store = project.read_with(cx, |project, _| project.context_server_store());
        store.update(cx, |store, cx| {
            store.set_context_server_factory(Box::new(move |id, _| {
                Arc::new(ContextServer::new(
                    id.clone(),
                    Arc::new(create_fake_transport(id.0.to_string(), executor.clone())),
                ))
            }));
            store.registry().update(cx, |registry, cx| {
                registry.register_context_server_descriptor(
                    SERVER_1_ID.into(),
                    fake_descriptor_1,
                    cx,
                );
            });
        });

        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Extension {
                    enabled: true,
                    remote: false,
                    settings: json!({
                        "somevalue": true
                    }),
                },
            )],
            cx,
        );

        // Ensure that mcp-1 starts up
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_1_id.clone(), ContextServerStatus::Starting),
                    (server_1_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            cx.run_until_parked();
        }

        // Ensure that mcp-1 is restarted when the configuration was changed
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_1_id.clone(), ContextServerStatus::Stopped),
                    (server_1_id.clone(), ContextServerStatus::Starting),
                    (server_1_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            set_context_server_configuration(
                vec![(
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Extension {
                        enabled: true,
                        remote: false,
                        settings: json!({
                            "somevalue": false
                        }),
                    },
                )],
                cx,
            );

            cx.run_until_parked();
        }

        // Ensure that mcp-1 is not restarted when the configuration was not changed
        {
            let _server_events = assert_server_events(&store, vec![], cx);
            set_context_server_configuration(
                vec![(
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Extension {
                        enabled: true,
                        remote: false,
                        settings: json!({
                            "somevalue": false
                        }),
                    },
                )],
                cx,
            );

            cx.run_until_parked();
        }

        // Ensure that mcp-2 is started once it is added to the settings
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_2_id.clone(), ContextServerStatus::Starting),
                    (server_2_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            set_context_server_configuration(
                vec![
                    (
                        server_1_id.0.clone(),
                        settings::ContextServerSettingsContent::Extension {
                            enabled: true,
                            remote: false,
                            settings: json!({
                                "somevalue": false
                            }),
                        },
                    ),
                    (
                        server_2_id.0.clone(),
                        settings::ContextServerSettingsContent::Stdio {
                            enabled: true,
                            remote: false,
                            command: ContextServerCommand {
                                path: "somebinary".into(),
                                args: vec!["arg".to_string()],
                                env: None,
                                timeout: None,
                            },
                        },
                    ),
                ],
                cx,
            );

            cx.run_until_parked();
        }

        // Ensure that mcp-2 is restarted once the args have changed
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_2_id.clone(), ContextServerStatus::Stopped),
                    (server_2_id.clone(), ContextServerStatus::Starting),
                    (server_2_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            set_context_server_configuration(
                vec![
                    (
                        server_1_id.0.clone(),
                        settings::ContextServerSettingsContent::Extension {
                            enabled: true,
                            remote: false,
                            settings: json!({
                                "somevalue": false
                            }),
                        },
                    ),
                    (
                        server_2_id.0.clone(),
                        settings::ContextServerSettingsContent::Stdio {
                            enabled: true,
                            remote: false,
                            command: ContextServerCommand {
                                path: "somebinary".into(),
                                args: vec!["anotherArg".to_string()],
                                env: None,
                                timeout: None,
                            },
                        },
                    ),
                ],
                cx,
            );

            cx.run_until_parked();
        }

        // Ensure that mcp-2 is removed once it is removed from the settings
        {
            let _server_events = assert_server_events(
                &store,
                vec![(server_2_id.clone(), ContextServerStatus::Stopped)],
                cx,
            );
            set_context_server_configuration(
                vec![(
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Extension {
                        enabled: true,
                        remote: false,
                        settings: json!({
                            "somevalue": false
                        }),
                    },
                )],
                cx,
            );

            cx.run_until_parked();

            cx.update(|cx| {
                assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
            });
        }

        // Ensure that nothing happens if the settings do not change
        {
            let _server_events = assert_server_events(&store, vec![], cx);
            set_context_server_configuration(
                vec![(
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Extension {
                        enabled: true,
                        remote: false,
                        settings: json!({
                            "somevalue": false
                        }),
                    },
                )],
                cx,
            );

            cx.run_until_parked();

            cx.update(|cx| {
                assert_eq!(
                    store.read(cx).status_for_server(&server_1_id),
                    Some(ContextServerStatus::Running)
                );
                assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
            });
        }
    }

    #[gpui::test]
    async fn test_context_server_enabled_disabled(cx: &mut TestAppContext) {
        const SERVER_1_ID: &str = "mcp-1";

        let server_1_id = ContextServerId(SERVER_1_ID.into());

        let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

        let executor = cx.executor();
        let store = project.read_with(cx, |project, _| project.context_server_store());
        store.update(cx, |store, _| {
            store.set_context_server_factory(Box::new(move |id, _| {
                Arc::new(ContextServer::new(
                    id.clone(),
                    Arc::new(create_fake_transport(id.0.to_string(), executor.clone())),
                ))
            }));
        });

        set_context_server_configuration(
            vec![(
                server_1_id.0.clone(),
                settings::ContextServerSettingsContent::Stdio {
                    enabled: true,
                    remote: false,
                    command: ContextServerCommand {
                        path: "somebinary".into(),
                        args: vec!["arg".to_string()],
                        env: None,
                        timeout: None,
                    },
                },
            )],
            cx,
        );

        // Ensure that mcp-1 starts up
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_1_id.clone(), ContextServerStatus::Starting),
                    (server_1_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            cx.run_until_parked();
        }

        // Ensure that mcp-1 is stopped once it is disabled.
        {
            let _server_events = assert_server_events(
                &store,
                vec![(server_1_id.clone(), ContextServerStatus::Stopped)],
                cx,
            );
            set_context_server_configuration(
                vec![(
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Stdio {
                        enabled: false,
                        remote: false,
                        command: ContextServerCommand {
                            path: "somebinary".into(),
                            args: vec!["arg".to_string()],
                            env: None,
                            timeout: None,
                        },
                    },
                )],
                cx,
            );

            cx.run_until_parked();
        }

        // Ensure that mcp-1 is started once it is enabled again.
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_1_id.clone(), ContextServerStatus::Starting),
                    (server_1_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            set_context_server_configuration(
                vec![(
                    server_1_id.0.clone(),
                    settings::ContextServerSettingsContent::Stdio {
                        enabled: true,
                        remote: false,
                        command: ContextServerCommand {
                            path: "somebinary".into(),
                            args: vec!["arg".to_string()],
                            timeout: None,
                            env: None,
                        },
                    },
                )],
                cx,
            );

            cx.run_until_parked();
        }
    }

    fn set_context_server_configuration(
        context_servers: Vec<(Arc<str>, settings::ContextServerSettingsContent)>,
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.project.context_servers.clear();
                    for (id, config) in context_servers {
                        content.project.context_servers.insert(id, config);
                    }
                });
            })
        });
    }

    #[gpui::test]
    async fn test_remote_context_server(cx: &mut TestAppContext) {
        const SERVER_ID: &str = "remote-server";
        let server_id = ContextServerId(SERVER_ID.into());
        let server_url = "http://example.com/api";

        let client = FakeHttpClient::create(|_| async move {
            use http_client::AsyncBody;

            let response = Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(AsyncBody::from(
                    serde_json::to_string(&json!({
                        "jsonrpc": "2.0",
                        "id": 0,
                        "result": {
                            "protocolVersion": "2024-11-05",
                            "capabilities": {},
                            "serverInfo": {
                                "name": "test-server",
                                "version": "1.0.0"
                            }
                        }
                    }))
                    .unwrap(),
                ))
                .unwrap();
            Ok(response)
        });
        cx.update(|cx| cx.set_http_client(client));

        let (_fs, project) = setup_context_server_test(cx, json!({ "code.rs": "" }), vec![]).await;

        let store = project.read_with(cx, |project, _| project.context_server_store());

        set_context_server_configuration(
            vec![(
                server_id.0.clone(),
                settings::ContextServerSettingsContent::Http {
                    enabled: true,
                    url: server_url.to_string(),
                    headers: Default::default(),
                    timeout: None,
                },
            )],
            cx,
        );

        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );
        cx.run_until_parked();
    }

    struct ServerEvents {
        received_event_count: Rc<RefCell<usize>>,
        expected_event_count: usize,
        _subscription: Subscription,
    }

    impl Drop for ServerEvents {
        fn drop(&mut self) {
            let actual_event_count = *self.received_event_count.borrow();
            assert_eq!(
                actual_event_count, self.expected_event_count,
                "
                Expected to receive {} context server store events, but received {} events",
                self.expected_event_count, actual_event_count
            );
        }
    }

    #[gpui::test]
    async fn test_context_server_global_timeout(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            SettingsStore::update_global(cx, |store, cx| {
                store
                    .set_user_settings(r#"{"context_server_timeout": 90}"#, cx)
                    .expect("Failed to set test user settings");
            });
        });

        let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(
                registry.clone(),
                project.read(cx).worktree_store(),
                Some(project.downgrade()),
                cx,
            )
        });

        let mut async_cx = cx.to_async();
        let result = ContextServerStore::create_context_server(
            store.downgrade(),
            ContextServerId("test-server".into()),
            Arc::new(ContextServerConfiguration::Http {
                url: url::Url::parse("http://localhost:8080").expect("Failed to parse test URL"),
                headers: Default::default(),
                timeout: None,
            }),
            &mut async_cx,
        )
        .await;

        assert!(
            result.is_ok(),
            "Server should be created successfully with global timeout"
        );
    }

    #[gpui::test]
    async fn test_context_server_per_server_timeout_override(cx: &mut TestAppContext) {
        const SERVER_ID: &str = "test-server";

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            SettingsStore::update_global(cx, |store, cx| {
                store
                    .set_user_settings(r#"{"context_server_timeout": 60}"#, cx)
                    .expect("Failed to set test user settings");
            });
        });

        let (_fs, project) = setup_context_server_test(
            cx,
            json!({"code.rs": ""}),
            vec![(
                SERVER_ID.into(),
                ContextServerSettings::Http {
                    enabled: true,
                    url: "http://localhost:8080".to_string(),
                    headers: Default::default(),
                    timeout: Some(120),
                },
            )],
        )
        .await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(
                registry.clone(),
                project.read(cx).worktree_store(),
                Some(project.downgrade()),
                cx,
            )
        });

        let mut async_cx = cx.to_async();
        let result = ContextServerStore::create_context_server(
            store.downgrade(),
            ContextServerId("test-server".into()),
            Arc::new(ContextServerConfiguration::Http {
                url: url::Url::parse("http://localhost:8080").expect("Failed to parse test URL"),
                headers: Default::default(),
                timeout: Some(120),
            }),
            &mut async_cx,
        )
        .await;

        assert!(
            result.is_ok(),
            "Server should be created successfully with per-server timeout override"
        );
    }

    #[gpui::test]
    async fn test_context_server_stdio_timeout(cx: &mut TestAppContext) {
        let (_fs, project) = setup_context_server_test(cx, json!({"code.rs": ""}), vec![]).await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(
                registry.clone(),
                project.read(cx).worktree_store(),
                Some(project.downgrade()),
                cx,
            )
        });

        let mut async_cx = cx.to_async();
        let result = ContextServerStore::create_context_server(
            store.downgrade(),
            ContextServerId("stdio-server".into()),
            Arc::new(ContextServerConfiguration::Custom {
                command: ContextServerCommand {
                    path: "/usr/bin/node".into(),
                    args: vec!["server.js".into()],
                    env: None,
                    timeout: Some(180000),
                },
                remote: false,
            }),
            &mut async_cx,
        )
        .await;

        assert!(
            result.is_ok(),
            "Stdio server should be created successfully with timeout"
        );
    }

    fn assert_server_events(
        store: &Entity<ContextServerStore>,
        expected_events: Vec<(ContextServerId, ContextServerStatus)>,
        cx: &mut TestAppContext,
    ) -> ServerEvents {
        cx.update(|cx| {
            let mut ix = 0;
            let received_event_count = Rc::new(RefCell::new(0));
            let expected_event_count = expected_events.len();
            let subscription = cx.subscribe(store, {
                let received_event_count = received_event_count.clone();
                move |_, event, _| match event {
                    Event::ServerStatusChanged {
                        server_id: actual_server_id,
                        status: actual_status,
                    } => {
                        let (expected_server_id, expected_status) = &expected_events[ix];

                        assert_eq!(
                            actual_server_id, expected_server_id,
                            "Expected different server id at index {}",
                            ix
                        );
                        assert_eq!(
                            actual_status, expected_status,
                            "Expected different status at index {}",
                            ix
                        );
                        ix += 1;
                        *received_event_count.borrow_mut() += 1;
                    }
                }
            });
            ServerEvents {
                expected_event_count,
                received_event_count,
                _subscription: subscription,
            }
        })
    }

    async fn setup_context_server_test(
        cx: &mut TestAppContext,
        files: serde_json::Value,
        context_server_configurations: Vec<(Arc<str>, ContextServerSettings)>,
    ) -> (Arc<FakeFs>, Entity<Project>) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            let mut settings = ProjectSettings::get_global(cx).clone();
            for (id, config) in context_server_configurations {
                settings.context_servers.insert(id, config);
            }
            ProjectSettings::override_global(settings, cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), files).await;
        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        (fs, project)
    }

    struct FakeContextServerDescriptor {
        path: PathBuf,
    }

    impl FakeContextServerDescriptor {
        fn new(path: impl Into<PathBuf>) -> Self {
            Self { path: path.into() }
        }
    }

    impl ContextServerDescriptor for FakeContextServerDescriptor {
        fn command(
            &self,
            _worktree_store: Entity<WorktreeStore>,
            _cx: &AsyncApp,
        ) -> Task<Result<ContextServerCommand>> {
            Task::ready(Ok(ContextServerCommand {
                path: self.path.clone(),
                args: vec!["arg1".to_string(), "arg2".to_string()],
                env: None,
                timeout: None,
            }))
        }

        fn configuration(
            &self,
            _worktree_store: Entity<WorktreeStore>,
            _cx: &AsyncApp,
        ) -> Task<Result<Option<::extension::ContextServerConfiguration>>> {
            Task::ready(Ok(None))
        }
    }
}
