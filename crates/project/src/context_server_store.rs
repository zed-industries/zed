pub mod extension;
pub mod registry;

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use context_server::oauth::{self, McpOAuthTokenProvider, OAuthDiscovery, OAuthSession};
use context_server::transport::{HttpTransport, TransportError};
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use credentials_provider::CredentialsProvider;
use futures::future::Either;
use futures::{FutureExt as _, StreamExt as _, future::join_all};
use gpui::{App, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity, actions};
use http_client::HttpClient;
use itertools::Itertools;
use rand::Rng as _;
use registry::ContextServerDescriptorRegistry;
use remote::RemoteClient;
use rpc::{AnyProtoClient, TypedEnvelope, proto};
use settings::{Settings as _, SettingsStore};
use util::{ResultExt as _, rel_path::RelPath};

use crate::{
    DisableAiSettings, Project,
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
    /// The server returned 401 and OAuth authorization is needed. The UI
    /// should show an "Authenticate" button.
    AuthRequired,
    /// The OAuth browser flow is in progress — the user has been redirected
    /// to the authorization server and we're waiting for the callback.
    Authenticating,
}

impl ContextServerStatus {
    fn from_state(state: &ContextServerState) -> Self {
        match state {
            ContextServerState::Starting { .. } => ContextServerStatus::Starting,
            ContextServerState::Running { .. } => ContextServerStatus::Running,
            ContextServerState::Stopped { .. } => ContextServerStatus::Stopped,
            ContextServerState::Error { error, .. } => ContextServerStatus::Error(error.clone()),
            ContextServerState::AuthRequired { .. } => ContextServerStatus::AuthRequired,
            ContextServerState::Authenticating { .. } => ContextServerStatus::Authenticating,
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
    /// The server requires OAuth authorization before it can be used. The
    /// `OAuthDiscovery` holds everything needed to start the browser flow.
    AuthRequired {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        discovery: Arc<OAuthDiscovery>,
    },
    /// The OAuth browser flow is in progress. The user has been redirected
    /// to the authorization server and we're waiting for the callback.
    Authenticating {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        _task: Task<()>,
    },
}

impl ContextServerState {
    pub fn server(&self) -> Arc<ContextServer> {
        match self {
            ContextServerState::Starting { server, .. }
            | ContextServerState::Running { server, .. }
            | ContextServerState::Stopped { server, .. }
            | ContextServerState::Error { server, .. }
            | ContextServerState::AuthRequired { server, .. }
            | ContextServerState::Authenticating { server, .. } => server.clone(),
        }
    }

    pub fn configuration(&self) -> Arc<ContextServerConfiguration> {
        match self {
            ContextServerState::Starting { configuration, .. }
            | ContextServerState::Running { configuration, .. }
            | ContextServerState::Stopped { configuration, .. }
            | ContextServerState::Error { configuration, .. }
            | ContextServerState::AuthRequired { configuration, .. }
            | ContextServerState::Authenticating { configuration, .. } => configuration.clone(),
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

    pub fn has_static_auth_header(&self) -> bool {
        match self {
            ContextServerConfiguration::Http { headers, .. } => headers
                .keys()
                .any(|k| k.eq_ignore_ascii_case("authorization")),
            _ => false,
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
        const EXTENSION_COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

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

                let command_future = descriptor.command(worktree_store, cx);
                let timeout_future = cx.background_executor().timer(EXTENSION_COMMAND_TIMEOUT);

                match futures::future::select(command_future, timeout_future).await {
                    Either::Left((Ok(command), _)) => Some(ContextServerConfiguration::Extension {
                        command,
                        settings,
                        remote,
                    }),
                    Either::Left((Err(e), _)) => {
                        log::error!(
                            "Failed to create context server configuration from settings: {e:#}"
                        );
                        None
                    }
                    Either::Right(_) => {
                        log::error!(
                            "Timed out resolving command for extension context server {id}"
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
    server_ids: Vec<ContextServerId>,
    worktree_store: Entity<WorktreeStore>,
    project: Option<WeakEntity<Project>>,
    registry: Entity<ContextServerDescriptorRegistry>,
    update_servers_task: Option<Task<Result<()>>>,
    context_server_factory: Option<ContextServerFactory>,
    needs_server_update: bool,
    ai_disabled: bool,
    _subscriptions: Vec<Subscription>,
}

pub struct ServerStatusChangedEvent {
    pub server_id: ContextServerId,
    pub status: ContextServerStatus,
}

impl EventEmitter<ServerStatusChangedEvent> for ContextServerStore {}

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

    #[cfg(feature = "test-support")]
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

    #[cfg(feature = "test-support")]
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

    #[cfg(feature = "test-support")]
    pub fn set_context_server_factory(&mut self, factory: ContextServerFactory) {
        self.context_server_factory = Some(factory);
    }

    #[cfg(feature = "test-support")]
    pub fn registry(&self) -> &Entity<ContextServerDescriptorRegistry> {
        &self.registry
    }

    #[cfg(feature = "test-support")]
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
            let ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
            let ai_was_disabled = this.ai_disabled;
            this.ai_disabled = ai_disabled;

            let settings =
                &Self::resolve_project_settings(&this.worktree_store, cx).context_servers;
            let settings_changed = &this.context_server_settings != settings;

            if settings_changed {
                this.context_server_settings = settings.clone();
            }

            // When AI is disabled, stop all running servers
            if ai_disabled {
                let server_ids: Vec<_> = this.servers.keys().cloned().collect();
                for id in server_ids {
                    this.stop_server(&id, cx).log_err();
                }
                return;
            }

            // Trigger updates if AI was re-enabled or settings changed
            if maintain_server_loop && (ai_was_disabled || settings_changed) {
                this.available_context_servers_changed(cx);
            }
        })];

        if maintain_server_loop {
            subscriptions.push(cx.observe(&registry, |this, _registry, cx| {
                if !DisableAiSettings::get_global(cx).disable_ai {
                    this.available_context_servers_changed(cx);
                }
            }));
        }

        let ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
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
            ai_disabled,
            servers: HashMap::default(),
            server_ids: Default::default(),
            update_servers_task: None,
            context_server_factory,
        };
        if maintain_server_loop && !DisableAiSettings::get_global(cx).disable_ai {
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

    /// Returns true if the given server is in a state where OAuth credentials
    /// may exist (Running, AuthRequired, or Authenticating).
    pub fn server_may_have_oauth_credentials(&self, id: &ContextServerId) -> bool {
        matches!(
            self.servers.get(id),
            Some(
                ContextServerState::Running { .. }
                    | ContextServerState::AuthRequired { .. }
                    | ContextServerState::Authenticating { .. }
            )
        )
    }

    pub fn configuration_for_server(
        &self,
        id: &ContextServerId,
    ) -> Option<Arc<ContextServerConfiguration>> {
        self.servers.get(id).map(|state| state.configuration())
    }

    /// Returns a sorted slice of available unique context server IDs. Within the
    /// slice, context servers which have `mcp-server-` as a prefix in their ID will
    /// appear after servers that do not have this prefix in their ID.
    pub fn server_ids(&self) -> &[ContextServerId] {
        self.server_ids.as_slice()
    }

    fn populate_server_ids(&mut self, cx: &App) {
        self.server_ids = self
            .servers
            .keys()
            .cloned()
            .chain(
                self.registry
                    .read(cx)
                    .context_server_descriptors()
                    .into_iter()
                    .map(|(id, _)| ContextServerId(id)),
            )
            .chain(
                self.context_server_settings
                    .keys()
                    .map(|id| ContextServerId(id.clone())),
            )
            .unique()
            .sorted_unstable_by(
                // Sort context servers: ones without mcp-server- prefix first, then prefixed ones
                |a, b| {
                    const MCP_PREFIX: &str = "mcp-server-";
                    match (a.0.strip_prefix(MCP_PREFIX), b.0.strip_prefix(MCP_PREFIX)) {
                        // If one has mcp-server- prefix and other doesn't, non-mcp comes first
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        // If both have same prefix status, sort by appropriate key
                        (Some(a), Some(b)) => a.cmp(b),
                        (None, None) => a.0.cmp(&b.0),
                    }
                },
            )
            .collect();
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
            let id = server.id();
            let settings = this
                .update(cx, |this, _| {
                    this.context_server_settings.get(&id.0).cloned()
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
                id.clone(),
                registry,
                worktree_store,
                cx,
            )
            .await
            .context("Failed to create context server configuration")?;

            let config = Arc::new(configuration);
            let (new_server, config) =
                Self::create_context_server(this.downgrade(), id.clone(), config, cx).await?;

            this.update(cx, |this, cx| this.run_server(new_server, config, cx));
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
            Some(
                ContextServerState::Starting { .. }
                    | ContextServerState::Running { .. }
                    | ContextServerState::Authenticating { .. },
            )
        ) {
            self.stop_server(&id, cx).log_err();
        }
        let task =
            cx.spawn({
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
                            // Check if the error is an OAuth 401 — if so, run
                            // discovery and transition to AuthRequired instead of
                            // the generic Error state. Skip this when the user
                            // configured a static Authorization header, since
                            // that means they're managing auth themselves.
                            if let Some(TransportError::AuthRequired { www_authenticate }) =
                                err.downcast_ref::<TransportError>()
                            {
                                if configuration.has_static_auth_header() {
                                    log::warn!(
                                        "{} received 401 with a static Authorization header configured",
                                        id,
                                    );
                                    this.update(cx, |this, cx| {
                                        this.update_server_state(
                                            id.clone(),
                                            ContextServerState::Error {
                                                configuration,
                                                server,
                                                error: "Server returned 401 Unauthorized. Check your configured Authorization header.".into(),
                                            },
                                            cx,
                                        )
                                    })
                                    .log_err();
                                    return;
                                }

                                let server_url = match &configuration.as_ref() {
                                    ContextServerConfiguration::Http { url, .. } => url.clone(),
                                    _ => {
                                        log::error!("{} got OAuth 401 on a non-HTTP transport", id);
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
                                        .log_err();
                                        return;
                                    }
                                };

                                let http_client = cx.update(|cx| cx.http_client());

                                match context_server::oauth::discover(
                                    &http_client,
                                    &server_url,
                                    www_authenticate,
                                )
                                .await
                                {
                                    Ok(discovery) => {
                                        log::info!(
                                            "{} requires OAuth authorization (auth server: {})",
                                            id,
                                            discovery.auth_server_metadata.issuer,
                                        );
                                        this.update(cx, |this, cx| {
                                            this.update_server_state(
                                                id.clone(),
                                                ContextServerState::AuthRequired {
                                                    server,
                                                    configuration,
                                                    discovery: Arc::new(discovery),
                                                },
                                                cx,
                                            )
                                        })
                                        .log_err();
                                        return;
                                    }
                                    Err(discovery_err) => {
                                        log::error!(
                                            "{} OAuth discovery failed: {}",
                                            id,
                                            discovery_err,
                                        );
                                        this.update(cx, |this, cx| {
                                            this.update_server_state(
                                                id.clone(),
                                                ContextServerState::Error {
                                                    configuration,
                                                    server,
                                                    error: format!(
                                                        "OAuth discovery failed: {}",
                                                        discovery_err
                                                    )
                                                    .into(),
                                                },
                                                cx,
                                            )
                                        })
                                        .log_err();
                                        return;
                                    }
                                }
                            }

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

        if let ContextServerConfiguration::Http { url, .. } = state.configuration().as_ref() {
            let server_url = url.clone();
            let id = id.clone();
            cx.spawn(async move |_this, cx| {
                let credentials_provider = cx.update(|cx| <dyn CredentialsProvider>::global(cx));
                if let Err(err) = Self::clear_session(&credentials_provider, &server_url, &cx).await
                {
                    log::warn!("{} failed to clear OAuth session on removal: {}", id, err);
                }
            })
            .detach();
        }

        drop(state);
        cx.emit(ServerStatusChangedEvent {
            server_id: id.clone(),
            status: ContextServerStatus::Stopped,
        });
        Ok(())
    }

    pub async fn create_context_server(
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

        if let Some(server) = this.update(cx, |this, _| {
            this.context_server_factory
                .as_ref()
                .map(|factory| factory(id.clone(), configuration.clone()))
        })? {
            return Ok((server, configuration));
        }

        let cached_token_provider: Option<Arc<dyn oauth::OAuthTokenProvider>> =
            if let ContextServerConfiguration::Http { url, .. } = configuration.as_ref() {
                if configuration.has_static_auth_header() {
                    None
                } else {
                    let credentials_provider =
                        cx.update(|cx| <dyn CredentialsProvider>::global(cx));
                    let http_client = cx.update(|cx| cx.http_client());

                    match Self::load_session(&credentials_provider, url, &cx).await {
                        Ok(Some(session)) => {
                            log::info!("{} loaded cached OAuth session from keychain", id);
                            Some(Self::create_oauth_token_provider(
                                &id,
                                url,
                                session,
                                http_client,
                                credentials_provider,
                                cx,
                            ))
                        }
                        Ok(None) => None,
                        Err(err) => {
                            log::warn!("{} failed to load cached OAuth session: {}", id, err);
                            None
                        }
                    }
                }
            } else {
                None
            };

        let server: Arc<ContextServer> = this.update(cx, |this, cx| {
            let global_timeout =
                Self::resolve_project_settings(&this.worktree_store, cx).context_server_timeout;

            match configuration.as_ref() {
                ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout,
                } => {
                    let transport = HttpTransport::new_with_token_provider(
                        cx.http_client(),
                        url.to_string(),
                        headers.clone(),
                        cx.background_executor().clone(),
                        cached_token_provider.clone(),
                    );
                    anyhow::Ok(Arc::new(ContextServer::new_with_timeout(
                        id,
                        Arc::new(transport),
                        Some(Duration::from_secs(
                            timeout.unwrap_or(global_timeout).min(MAX_TIMEOUT_SECS),
                        )),
                    )))
                }
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

    fn create_oauth_token_provider(
        id: &ContextServerId,
        server_url: &url::Url,
        session: OAuthSession,
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut AsyncApp,
    ) -> Arc<dyn oauth::OAuthTokenProvider> {
        let (token_refresh_tx, mut token_refresh_rx) = futures::channel::mpsc::unbounded();
        let id = id.clone();
        let server_url = server_url.clone();

        cx.spawn(async move |cx| {
            while let Some(refreshed_session) = token_refresh_rx.next().await {
                if let Err(err) =
                    Self::store_session(&credentials_provider, &server_url, &refreshed_session, &cx)
                        .await
                {
                    log::warn!("{} failed to persist refreshed OAuth session: {}", id, err);
                }
            }
            log::debug!("{} OAuth session persistence task ended", id);
        })
        .detach();

        Arc::new(McpOAuthTokenProvider::new(
            session,
            http_client,
            Some(token_refresh_tx),
        ))
    }

    /// Initiate the OAuth browser flow for a server in the `AuthRequired` state.
    ///
    /// This starts a loopback HTTP callback server on an ephemeral port, builds
    /// the authorization URL, opens the user's browser, waits for the callback,
    /// exchanges the code for tokens, persists them in the keychain, and restarts
    /// the server with the new token provider.
    pub fn authenticate_server(
        &mut self,
        id: &ContextServerId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let state = self.servers.get(id).context("Context server not found")?;

        let (discovery, server, configuration) = match state {
            ContextServerState::AuthRequired {
                discovery,
                server,
                configuration,
            } => (discovery.clone(), server.clone(), configuration.clone()),
            _ => anyhow::bail!("Server is not in AuthRequired state"),
        };

        let id = id.clone();

        let task = cx.spawn({
            let id = id.clone();
            let server = server.clone();
            let configuration = configuration.clone();
            async move |this, cx| {
                let result = Self::run_oauth_flow(
                    this.clone(),
                    id.clone(),
                    discovery.clone(),
                    configuration.clone(),
                    cx,
                )
                .await;

                if let Err(err) = &result {
                    log::error!("{} OAuth authentication failed: {:?}", id, err);
                    // Transition back to AuthRequired so the user can retry
                    // rather than landing in a terminal Error state.
                    this.update(cx, |this, cx| {
                        this.update_server_state(
                            id.clone(),
                            ContextServerState::AuthRequired {
                                server,
                                configuration,
                                discovery,
                            },
                            cx,
                        )
                    })
                    .log_err();
                }
            }
        });

        self.update_server_state(
            id,
            ContextServerState::Authenticating {
                server,
                configuration,
                _task: task,
            },
            cx,
        );

        Ok(())
    }

    async fn run_oauth_flow(
        this: WeakEntity<Self>,
        id: ContextServerId,
        discovery: Arc<OAuthDiscovery>,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let resource = oauth::canonical_server_uri(&discovery.resource_metadata.resource);
        let pkce = oauth::generate_pkce_challenge();

        let mut state_bytes = [0u8; 32];
        rand::rng().fill(&mut state_bytes);
        let state_param: String = state_bytes.iter().map(|b| format!("{:02x}", b)).collect();

        // Start a loopback HTTP server on an ephemeral port. The redirect URI
        // includes this port so the browser sends the callback directly to our
        // process.
        let (redirect_uri, callback_rx) = oauth::start_callback_server()
            .await
            .context("Failed to start OAuth callback server")?;

        let http_client = cx.update(|cx| cx.http_client());
        let credentials_provider = cx.update(|cx| <dyn CredentialsProvider>::global(cx));
        let server_url = match configuration.as_ref() {
            ContextServerConfiguration::Http { url, .. } => url.clone(),
            _ => anyhow::bail!("OAuth authentication only supported for HTTP servers"),
        };

        let client_registration =
            oauth::resolve_client_registration(&http_client, &discovery, &redirect_uri)
                .await
                .context("Failed to resolve OAuth client registration")?;

        let auth_url = oauth::build_authorization_url(
            &discovery.auth_server_metadata,
            &client_registration.client_id,
            &redirect_uri,
            &discovery.scopes,
            &resource,
            &pkce,
            &state_param,
        );

        cx.update(|cx| cx.open_url(auth_url.as_str()));

        let callback = callback_rx
            .await
            .map_err(|_| {
                anyhow::anyhow!("OAuth callback server was shut down before receiving a response")
            })?
            .context("OAuth callback server received an invalid request")?;

        if callback.state != state_param {
            anyhow::bail!("OAuth state parameter mismatch (possible CSRF)");
        }

        let tokens = oauth::exchange_code(
            &http_client,
            &discovery.auth_server_metadata,
            &callback.code,
            &client_registration.client_id,
            &redirect_uri,
            &pkce.verifier,
            &resource,
        )
        .await
        .context("Failed to exchange authorization code for tokens")?;

        let session = OAuthSession {
            token_endpoint: discovery.auth_server_metadata.token_endpoint.clone(),
            resource: discovery.resource_metadata.resource.clone(),
            client_registration,
            tokens,
        };

        Self::store_session(&credentials_provider, &server_url, &session, cx)
            .await
            .context("Failed to persist OAuth session in keychain")?;

        let token_provider = Self::create_oauth_token_provider(
            &id,
            &server_url,
            session,
            http_client.clone(),
            credentials_provider,
            cx,
        );

        let new_server = this.update(cx, |this, cx| {
            let global_timeout =
                Self::resolve_project_settings(&this.worktree_store, cx).context_server_timeout;

            match configuration.as_ref() {
                ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout,
                } => {
                    let transport = HttpTransport::new_with_token_provider(
                        http_client.clone(),
                        url.to_string(),
                        headers.clone(),
                        cx.background_executor().clone(),
                        Some(token_provider.clone()),
                    );
                    Ok(Arc::new(ContextServer::new_with_timeout(
                        id.clone(),
                        Arc::new(transport),
                        Some(Duration::from_secs(
                            timeout.unwrap_or(global_timeout).min(MAX_TIMEOUT_SECS),
                        )),
                    )))
                }
                _ => anyhow::bail!("OAuth authentication only supported for HTTP servers"),
            }
        })??;

        this.update(cx, |this, cx| {
            this.run_server(new_server, configuration, cx);
        })?;

        Ok(())
    }

    /// Store the full OAuth session in the system keychain, keyed by the
    /// server's canonical URI.
    async fn store_session(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        session: &OAuthSession,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::keychain_key(server_url);
        let json = serde_json::to_string(session)?;
        credentials_provider
            .write_credentials(&key, "mcp-oauth", json.as_bytes(), cx)
            .await
    }

    /// Load the full OAuth session from the system keychain for the given
    /// server URL.
    async fn load_session(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<Option<OAuthSession>> {
        let key = Self::keychain_key(server_url);
        match credentials_provider.read_credentials(&key, cx).await? {
            Some((_username, password_bytes)) => {
                let session: OAuthSession = serde_json::from_slice(&password_bytes)?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    /// Clear the stored OAuth session from the system keychain.
    async fn clear_session(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::keychain_key(server_url);
        credentials_provider.delete_credentials(&key, cx).await
    }

    fn keychain_key(server_url: &url::Url) -> String {
        format!("mcp-oauth:{}", oauth::canonical_server_uri(server_url))
    }

    /// Log out of an OAuth-authenticated MCP server: clear the stored OAuth
    /// session from the keychain and stop the server.
    pub fn logout_server(&mut self, id: &ContextServerId, cx: &mut Context<Self>) -> Result<()> {
        let state = self.servers.get(id).context("Context server not found")?;
        let configuration = state.configuration();

        let server_url = match configuration.as_ref() {
            ContextServerConfiguration::Http { url, .. } => url.clone(),
            _ => anyhow::bail!("logout only applies to HTTP servers with OAuth"),
        };

        let id = id.clone();
        self.stop_server(&id, cx)?;

        cx.spawn(async move |_this, cx| {
            let credentials_provider = cx.update(|cx| <dyn CredentialsProvider>::global(cx));
            if let Err(err) = Self::clear_session(&credentials_provider, &server_url, &cx).await {
                log::error!("{} failed to clear OAuth session: {}", id, err);
            }
        })
        .detach();

        Ok(())
    }

    fn update_server_state(
        &mut self,
        id: ContextServerId,
        state: ContextServerState,
        cx: &mut Context<Self>,
    ) {
        let status = ContextServerStatus::from_state(&state);
        self.servers.insert(id.clone(), state);
        cx.emit(ServerStatusChangedEvent {
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
                    this.populate_server_ids(cx);
                    cx.notify();
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
        // Don't start context servers if AI is disabled
        let ai_disabled = this.update(cx, |_, cx| DisableAiSettings::get_global(cx).disable_ai)?;
        if ai_disabled {
            // Stop all running servers when AI is disabled
            this.update(cx, |this, cx| {
                let server_ids: Vec<_> = this.servers.keys().cloned().collect();
                for id in server_ids {
                    let _ = this.stop_server(&id, cx);
                }
            })?;
            return Ok(());
        }

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
            match Self::create_context_server(this.clone(), id.clone(), config, cx).await {
                Ok((server, config)) => {
                    this.update(cx, |this, cx| {
                        this.run_server(server, config, cx);
                    })?;
                }
                Err(err) => {
                    log::error!("{id} context server failed to create: {err:#}");
                    this.update(cx, |_this, cx| {
                        cx.emit(ServerStatusChangedEvent {
                            server_id: id,
                            status: ContextServerStatus::Error(err.to_string().into()),
                        });
                        cx.notify();
                    })?;
                }
            }
        }

        Ok(())
    }
}
