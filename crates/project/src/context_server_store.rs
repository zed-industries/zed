pub mod extension;
pub mod registry;

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use context_server::oauth::{self, McpOAuthTokenProvider, OAuthDiscovery, OAuthSession};
use context_server::transport::{HttpTransport, TransportShutdownReason};
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use credentials_provider::CredentialsProvider;
use feature_flags::{FeatureFlagAppExt as _, FeatureFlagStore, McpRegistryFeatureFlag};
use fs::Fs;
use futures::future::Either;
use futures::{FutureExt as _, StreamExt as _, future::join_all};
#[cfg(feature = "test-support")]
use gpui::AppContext as _;
use gpui::{
    App, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, TaskExt, WeakEntity, actions,
};
use http_client::HttpClient;
use itertools::Itertools;
use node_runtime::NodeRuntime;
use rand::Rng as _;
use registry::ContextServerDescriptorRegistry;
use remote::{Interactive, RemoteClient};
use rpc::{AnyProtoClient, TypedEnvelope, proto};
use settings::{
    ContextServerSettingsContent, ProfileBase, Settings as _, SettingsFile, SettingsLocation,
    SettingsStore, UserSettingsContentExt as _, WorktreeId,
};
use util::{ResultExt as _, rel_path::RelPath};

use crate::{
    DisableAiSettings, McpRegistryStore, Project, ProjectEnvironment,
    mcp_registry_store::{
        McpRegistryInstallationSource, ResolvedMcpRegistryServer, read_server_secrets,
        registry_npm_package_name, resolve_server_configuration, run_with_registry_enabled,
        validate_npm_environment, validate_npm_runtime_arguments,
    },
    project_settings::{ContextServerSettings, OAuthClientSettings, ProjectSettings},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
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
    /// The server has a pre-registered OAuth client_id, but a client_secret
    /// is needed and not available in settings or the keychain.
    ClientSecretRequired {
        error: Option<Arc<str>>,
    },
    /// The OAuth browser flow is in progress — the user has been redirected
    /// to the authorization server and we're waiting for the callback.
    Authenticating,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextServerSource {
    Custom,
    Extension,
    Registry,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistrySettingsOwnership {
    User,
    Project,
    Managed,
}

pub fn registry_settings_ownership(
    server_name: &str,
    configured_in_project: bool,
    cx: &App,
) -> RegistrySettingsOwnership {
    if configured_in_project {
        return RegistrySettingsOwnership::Project;
    }
    let settings_store = cx.global::<SettingsStore>();
    if settings_store
        .get_content_for_file(SettingsFile::Server)
        .and_then(|settings| settings.project.context_servers.get(server_name))
        .is_some()
    {
        return RegistrySettingsOwnership::Managed;
    }
    let Some(user_settings) = settings_store.raw_user_settings() else {
        return RegistrySettingsOwnership::Managed;
    };
    if !matches!(
        user_settings
            .content
            .project
            .context_servers
            .get(server_name),
        Some(ContextServerSettingsContent::Registry { .. })
    ) {
        return RegistrySettingsOwnership::Managed;
    }
    if user_settings.for_profile(cx).is_some_and(|profile| {
        profile.base == ProfileBase::Default
            || profile
                .settings
                .project
                .context_servers
                .contains_key(server_name)
    }) || user_settings
        .for_os()
        .is_some_and(|settings| settings.project.context_servers.contains_key(server_name))
        || user_settings
            .for_release_channel()
            .is_some_and(|settings| settings.project.context_servers.contains_key(server_name))
    {
        RegistrySettingsOwnership::Managed
    } else {
        RegistrySettingsOwnership::User
    }
}

pub fn registry_user_settings_destination_is_active(cx: &App) -> bool {
    cx.global::<SettingsStore>()
        .raw_user_settings()
        .and_then(|settings| settings.for_profile(cx))
        .is_none_or(|profile| profile.base == ProfileBase::User)
}

impl ContextServerStatus {
    fn from_state(state: &ContextServerState) -> Self {
        match state {
            ContextServerState::Starting { .. } => ContextServerStatus::Starting,
            ContextServerState::Running { .. } => ContextServerStatus::Running,
            ContextServerState::Stopped { .. } => ContextServerStatus::Stopped,
            ContextServerState::Error { error, .. } => ContextServerStatus::Error(error.clone()),
            ContextServerState::AuthRequired { .. } => ContextServerStatus::AuthRequired,
            ContextServerState::ClientSecretRequired { error, .. } => {
                ContextServerStatus::ClientSecretRequired {
                    error: error.clone(),
                }
            }
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
        /// Handles transport lifecycle transitions; cancelled by any state
        /// transition.
        _transport_watch: Task<()>,
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
    /// A pre-registered client_id is configured but no client_secret was found
    /// in settings or the keychain.
    ClientSecretRequired {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        discovery: Arc<OAuthDiscovery>,
        error: Option<Arc<str>>,
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
            | ContextServerState::ClientSecretRequired { server, .. }
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
            | ContextServerState::ClientSecretRequired { configuration, .. }
            | ContextServerState::Authenticating { configuration, .. } => configuration.clone(),
        }
    }
}

#[derive(PartialEq, Eq)]
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
        oauth: Option<OAuthClientSettings>,
    },
    RemoteRegistryNpm {
        package_spec: String,
        runtime_arguments: Vec<String>,
        package_arguments: Vec<String>,
        environment: HashMap<String, String>,
    },
}

impl std::fmt::Debug for ContextServerConfiguration {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom { command, remote } => formatter
                .debug_struct("Custom")
                .field("command", command)
                .field("remote", remote)
                .finish(),
            Self::Extension {
                command, remote, ..
            } => formatter
                .debug_struct("Extension")
                .field("command", command)
                .field("settings", &"[REDACTED]")
                .field("remote", remote)
                .finish(),
            Self::Http {
                url,
                headers,
                timeout,
                oauth,
            } => {
                let mut header_names = headers.keys().collect::<Vec<_>>();
                header_names.sort_unstable();
                formatter
                    .debug_struct("Http")
                    .field("origin", &url.origin().ascii_serialization())
                    .field("header_names", &header_names)
                    .field("timeout", timeout)
                    .field("has_oauth", &oauth.is_some())
                    .finish()
            }
            Self::RemoteRegistryNpm {
                package_spec,
                runtime_arguments,
                package_arguments,
                environment,
            } => {
                let mut environment_names = environment.keys().collect::<Vec<_>>();
                environment_names.sort_unstable();
                formatter
                    .debug_struct("RemoteRegistryNpm")
                    .field("package_spec", package_spec)
                    .field("runtime_argument_count", &runtime_arguments.len())
                    .field("package_argument_count", &package_arguments.len())
                    .field("environment_names", &environment_names)
                    .finish()
            }
        }
    }
}

impl ContextServerConfiguration {
    pub fn command(&self) -> Option<&ContextServerCommand> {
        match self {
            ContextServerConfiguration::Custom { command, .. } => Some(command),
            ContextServerConfiguration::Extension { command, .. } => Some(command),
            ContextServerConfiguration::Http { .. }
            | ContextServerConfiguration::RemoteRegistryNpm { .. } => None,
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
            ContextServerConfiguration::RemoteRegistryNpm { .. } => true,
        }
    }

    pub async fn from_settings(
        settings: ContextServerSettings,
        id: ContextServerId,
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        project_environment: Entity<ProjectEnvironment>,
        node_runtime: Option<NodeRuntime>,
        defer_remote_registry_npm: bool,
        cx: &AsyncApp,
    ) -> Result<Self> {
        const EXTENSION_COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

        match settings {
            ContextServerSettings::Stdio {
                enabled: _,
                command,
                remote,
            } => Ok(ContextServerConfiguration::Custom { command, remote }),
            ContextServerSettings::Extension {
                enabled: _,
                settings,
                remote,
            } => {
                let descriptor = cx
                    .update(|cx| registry.read(cx).context_server_descriptor(&id.0))
                    .with_context(|| format!("extension context server `{id}` was not found"))?;

                let command_future = descriptor.command(worktree_store, cx);
                let timeout_future = cx.background_executor().timer(EXTENSION_COMMAND_TIMEOUT);

                match futures::future::select(command_future, timeout_future).await {
                    Either::Left((Ok(command), _)) => Ok(ContextServerConfiguration::Extension {
                        command,
                        settings,
                        remote,
                    }),
                    Either::Left((Err(error), _)) => Err(error).with_context(|| {
                        format!("resolving command for extension context server `{id}`")
                    }),
                    Either::Right(_) => anyhow::bail!(
                        "timed out resolving command for extension context server `{id}`"
                    ),
                }
            }
            ContextServerSettings::Http {
                enabled: _,
                url,
                headers: auth,
                timeout,
                oauth,
            } => {
                let url = url::Url::parse(&url)
                    .with_context(|| format!("invalid URL for context server `{id}`"))?;
                Ok(ContextServerConfiguration::Http {
                    url,
                    headers: auth,
                    timeout,
                    oauth,
                })
            }
            ContextServerSettings::Registry {
                enabled: _,
                remote,
                registry,
            } => {
                run_with_registry_enabled(
                    cx,
                    Self::from_registry_settings(
                        registry,
                        id,
                        remote,
                        node_runtime,
                        project_environment,
                        defer_remote_registry_npm,
                        cx,
                    ),
                )
                .await
            }
        }
    }

    async fn from_registry_settings(
        registry_settings: settings::McpRegistryServerSettings,
        id: ContextServerId,
        remote: bool,
        node_runtime: Option<NodeRuntime>,
        project_environment: Entity<ProjectEnvironment>,
        defer_remote_registry_npm: bool,
        cx: &AsyncApp,
    ) -> Result<Self> {
        anyhow::ensure!(
            cx.update(|cx| cx.has_flag::<McpRegistryFeatureFlag>()),
            "MCP Registry feature is not enabled"
        );
        let registry_store = cx
            .update(|cx| McpRegistryStore::try_global(cx))
            .context("MCP Registry store is not initialized")?;
        let installation_task = cx.update(|cx| {
            registry_store.update(cx, |store, cx| {
                store.server_installation(&id.0, registry_settings.source.clone(), cx)
            })
        });
        let (server, source) = installation_task
            .await
            .with_context(|| format!("loading MCP Registry installation for `{id}`"))?;
        anyhow::ensure!(
            cx.update(|cx| cx.has_flag::<McpRegistryFeatureFlag>()),
            "MCP Registry feature is not enabled"
        );
        if server.name() != id.0.as_ref() {
            anyhow::bail!("MCP Registry returned details for an unexpected server");
        }

        if should_reject_remote_registry_package_credentials(
            &source,
            registry_settings.credential_id.is_some(),
            remote,
            defer_remote_registry_npm,
        ) {
            anyhow::bail!("MCP Registry packages with secret inputs cannot run on a remote host");
        }

        let secret_inputs = if let Some(credential_id) = registry_settings.credential_id.as_deref()
        {
            read_server_secrets(credential_id, cx)
                .await
                .with_context(|| format!("loading secret inputs for `{id}`"))?
        } else {
            HashMap::default()
        };
        anyhow::ensure!(
            cx.update(|cx| cx.has_flag::<McpRegistryFeatureFlag>()),
            "MCP Registry feature is not enabled"
        );
        let resolved = resolve_server_configuration(
            &server,
            &source,
            &registry_settings.inputs,
            &secret_inputs,
        )?;

        match resolved {
            ResolvedMcpRegistryServer::Http { url, headers } => {
                Ok(ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout: None,
                    oauth: None,
                })
            }
            ResolvedMcpRegistryServer::Npm {
                package_spec,
                runtime_arguments,
                package_arguments,
                environment,
            } => {
                if defer_remote_registry_npm && remote {
                    return Ok(ContextServerConfiguration::RemoteRegistryNpm {
                        package_spec,
                        runtime_arguments,
                        package_arguments,
                        environment,
                    });
                }

                let command = Self::registry_npm_command(
                    &id,
                    package_spec,
                    runtime_arguments,
                    package_arguments,
                    environment,
                    node_runtime,
                    project_environment,
                    defer_remote_registry_npm,
                    cx,
                )
                .await?;
                Ok(ContextServerConfiguration::Custom { command, remote })
            }
        }
    }

    async fn registry_npm_command(
        id: &ContextServerId,
        package_spec: String,
        runtime_arguments: Vec<String>,
        package_arguments: Vec<String>,
        environment: HashMap<String, String>,
        node_runtime: Option<NodeRuntime>,
        project_environment: Entity<ProjectEnvironment>,
        use_local_execution_environment: bool,
        cx: &AsyncApp,
    ) -> Result<ContextServerCommand> {
        registry_npm_package_name(&package_spec)?;
        validate_npm_runtime_arguments(&runtime_arguments)?;
        validate_npm_environment(&environment)?;
        let node_runtime =
            node_runtime.context("Node.js is unavailable for this MCP Registry package")?;
        let fs = cx.update(|cx| <dyn Fs>::global(cx));
        let prefix_directory = paths::data_dir()
            .join("mcp_registry")
            .join("npm")
            .join(sanitize_registry_path_component(&id.0));
        fs.create_dir(&prefix_directory)
            .await
            .context("creating MCP Registry npm cache directory")?;

        let mut npm_arguments = vec!["--yes".to_owned()];
        npm_arguments.extend(runtime_arguments);
        npm_arguments.push("--".to_owned());
        npm_arguments.push(package_spec);
        npm_arguments.extend(package_arguments);
        let npm_argument_references = npm_arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let npm_command = node_runtime
            .npm_command_with_user_configuration(
                Some(&prefix_directory),
                "exec",
                &npm_argument_references,
            )
            .await
            .context("building npm command for MCP Registry package")?;

        let mut async_cx = cx.clone();
        let mut command_environment = project_environment
            .update(&mut async_cx, |project_environment, cx| {
                if use_local_execution_environment {
                    project_environment.local_execution_environment(cx)
                } else {
                    project_environment.default_environment(cx)
                }
            })
            .await
            .unwrap_or_default();
        command_environment.extend(npm_command.env);
        command_environment.extend(environment);

        Ok(ContextServerCommand {
            path: npm_command.path,
            args: npm_command.args,
            env: Some(command_environment),
            timeout: None,
        })
    }
}

fn should_reject_remote_registry_package_credentials(
    source: &McpRegistryInstallationSource,
    has_credentials: bool,
    remote: bool,
    defer_remote_registry_npm: bool,
) -> bool {
    defer_remote_registry_npm
        && remote
        && has_credentials
        && matches!(source, McpRegistryInstallationSource::Package { .. })
}

fn sanitize_registry_path_component(input: &str) -> String {
    let sanitized = input
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' => character,
            _ => '-',
        })
        .collect::<String>();
    if sanitized.is_empty() {
        "server".to_owned()
    } else {
        sanitized
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

#[derive(Clone, PartialEq)]
struct ContextServerSettingsEntry {
    worktree_id: Option<WorktreeId>,
    configured_in_project: bool,
    settings: ContextServerSettings,
}

pub struct ContextServerStore {
    state: ContextServerStoreState,
    context_server_settings: HashMap<Arc<str>, ContextServerSettingsEntry>,
    servers: HashMap<ContextServerId, ContextServerState>,
    desired_configurations: HashMap<ContextServerId, Arc<ContextServerConfiguration>>,
    agent_configurations: HashMap<ContextServerId, Arc<ContextServerConfiguration>>,
    configuration_errors: HashMap<ContextServerId, Arc<str>>,
    server_ids: Vec<ContextServerId>,
    worktree_store: Entity<WorktreeStore>,
    project_environment: Entity<ProjectEnvironment>,
    project: Option<WeakEntity<Project>>,
    node_runtime: Option<NodeRuntime>,
    registry: Entity<ContextServerDescriptorRegistry>,
    update_servers_task: Option<Task<Result<()>>>,
    context_server_factory: Option<ContextServerFactory>,
    /// The working directory each server was last started with. The working
    /// directory of a stdio server depends on the resolved project root, which
    /// can only become available after the server has already started (e.g. a
    /// worktree is added moments after launch). Tracking it lets
    /// `maintain_servers` restart a server when its working directory changes,
    /// since the working directory is not part of `ContextServerConfiguration`.
    server_working_directories: HashMap<ContextServerId, Option<Arc<Path>>>,
    needs_server_update: bool,
    ai_disabled: bool,
    mcp_registry_enabled: bool,
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
        project_environment: Entity<ProjectEnvironment>,
        weak_project: Option<WeakEntity<Project>>,
        node_runtime: Option<NodeRuntime>,
        headless: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(
            !headless,
            None,
            ContextServerDescriptorRegistry::default_global(cx),
            worktree_store,
            project_environment,
            weak_project,
            node_runtime,
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
        project_environment: Entity<ProjectEnvironment>,
        weak_project: Option<WeakEntity<Project>>,
        node_runtime: Option<NodeRuntime>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(
            true,
            None,
            ContextServerDescriptorRegistry::default_global(cx),
            worktree_store,
            project_environment,
            weak_project,
            node_runtime,
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
            .filter(|(_, entry)| entry.settings.enabled())
            .filter(|(_, entry)| {
                self.mcp_registry_enabled
                    || !matches!(entry.settings, ContextServerSettings::Registry { .. })
            })
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
        let project_environment =
            cx.new(|cx| ProjectEnvironment::new(None, worktree_store.downgrade(), None, false, cx));
        Self::new_internal(
            false,
            None,
            registry,
            worktree_store,
            project_environment,
            weak_project,
            None,
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
        let project_environment =
            cx.new(|cx| ProjectEnvironment::new(None, worktree_store.downgrade(), None, false, cx));
        Self::new_internal(
            true,
            context_server_factory,
            registry,
            worktree_store,
            project_environment,
            weak_project,
            None,
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
    pub fn test_set_context_server_settings(
        &mut self,
        id: Arc<str>,
        settings: ContextServerSettings,
    ) {
        self.context_server_settings.insert(
            id,
            ContextServerSettingsEntry {
                worktree_id: None,
                configured_in_project: false,
                settings,
            },
        );
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
        self.desired_configurations
            .insert(server.id(), configuration.clone());
        self.run_server(server, configuration, cx);
    }

    #[cfg(feature = "test-support")]
    pub async fn test_create_context_server(
        this: WeakEntity<Self>,
        id: Arc<str>,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut AsyncApp,
    ) -> Result<Option<Arc<ContextServerConfiguration>>> {
        let (_, _, agent_configuration) =
            Self::create_context_server(this, ContextServerId(id), configuration, cx).await?;
        Ok(agent_configuration)
    }

    fn new_internal(
        maintain_server_loop: bool,
        context_server_factory: Option<ContextServerFactory>,
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        project_environment: Entity<ProjectEnvironment>,
        weak_project: Option<WeakEntity<Project>>,
        node_runtime: Option<NodeRuntime>,
        state: ContextServerStoreState,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = vec![cx.observe_global::<SettingsStore>(move |this, cx| {
            let ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
            let ai_was_disabled = this.ai_disabled;
            this.ai_disabled = ai_disabled;

            let settings = Self::resolve_all_context_server_settings(&this.worktree_store, cx);
            let settings_changed = this.context_server_settings != settings;

            if settings_changed {
                this.context_server_settings = settings;
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

        subscriptions.push(cx.observe_global::<FeatureFlagStore>(move |this, cx| {
            let enabled = cx.has_flag::<McpRegistryFeatureFlag>();
            if enabled == this.mcp_registry_enabled {
                return;
            }
            this.mcp_registry_enabled = enabled;
            if !enabled {
                this.update_servers_task.take();
                this.needs_server_update = false;
                let server_ids = this
                    .servers
                    .keys()
                    .filter(|id| this.is_registry_server(id))
                    .cloned()
                    .collect::<Vec<_>>();
                for id in server_ids {
                    this.stop_server(&id, cx).log_err();
                    this.desired_configurations.remove(&id);
                    this.agent_configurations.remove(&id);
                    this.set_configuration_error(
                        id,
                        "MCP Registry feature is not enabled".into(),
                        cx,
                    );
                }
            }
            if maintain_server_loop && !DisableAiSettings::get_global(cx).disable_ai {
                this.available_context_servers_changed(cx);
            }
            cx.notify();
        }));

        if maintain_server_loop {
            subscriptions.push(cx.observe(&registry, |this, _registry, cx| {
                if !DisableAiSettings::get_global(cx).disable_ai {
                    this.available_context_servers_changed(cx);
                }
            }));
            if let Some(mcp_registry_store) = McpRegistryStore::try_global(cx) {
                subscriptions.push(cx.observe(&mcp_registry_store, |this, _registry, cx| {
                    if cx.has_flag::<McpRegistryFeatureFlag>()
                        && !DisableAiSettings::get_global(cx).disable_ai
                    {
                        this.available_context_servers_changed(cx);
                    }
                }));
            }
            subscriptions.push(cx.subscribe(&worktree_store, |this, _store, event, cx| {
                if matches!(
                    event,
                    WorktreeStoreEvent::WorktreeAdded(_)
                        | WorktreeStoreEvent::WorktreeRemoved(_, _)
                ) && !DisableAiSettings::get_global(cx).disable_ai
                {
                    this.context_server_settings =
                        Self::resolve_all_context_server_settings(&this.worktree_store, cx);
                    this.available_context_servers_changed(cx);
                }
            }));
        }

        let ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
        let mut this = Self {
            state,
            _subscriptions: subscriptions,
            context_server_settings: Self::resolve_all_context_server_settings(&worktree_store, cx),
            worktree_store,
            project_environment,
            project: weak_project,
            node_runtime,
            registry,
            needs_server_update: false,
            ai_disabled,
            mcp_registry_enabled: cx.has_flag::<McpRegistryFeatureFlag>(),
            servers: HashMap::default(),
            desired_configurations: HashMap::default(),
            agent_configurations: HashMap::default(),
            configuration_errors: HashMap::default(),
            server_ids: Default::default(),
            update_servers_task: None,
            context_server_factory,
            server_working_directories: HashMap::default(),
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
        self.configuration_errors
            .get(id)
            .cloned()
            .map(ContextServerStatus::Error)
            .or_else(|| self.servers.get(id).map(ContextServerStatus::from_state))
    }

    pub fn configuration_for_server(
        &self,
        id: &ContextServerId,
    ) -> Option<Arc<ContextServerConfiguration>> {
        self.servers.get(id).map(|state| state.configuration())
    }

    pub fn configuration_for_agent(
        &self,
        id: &ContextServerId,
    ) -> Option<Arc<ContextServerConfiguration>> {
        if !self.mcp_registry_enabled && self.is_registry_server(id) {
            return None;
        }
        select_agent_configuration(
            self.is_remote_project(),
            self.configuration_errors.contains_key(id),
            self.agent_configurations.get(id).cloned(),
            self.configuration_for_server(id),
        )
    }

    /// Returns the configured settings for a server, if it is present in the user
    /// or project settings. This is available regardless of whether the server is
    /// currently running, unlike [`Self::configuration_for_server`].
    pub fn settings_for_server(&self, id: &ContextServerId) -> Option<&ContextServerSettings> {
        self.context_server_settings
            .get(&id.0)
            .map(|entry| &entry.settings)
    }

    pub fn registry_settings_ownership(
        &self,
        id: &ContextServerId,
        cx: &App,
    ) -> RegistrySettingsOwnership {
        registry_settings_ownership(&id.0, self.is_server_configured_locally(id), cx)
    }

    fn is_registry_server(&self, id: &ContextServerId) -> bool {
        matches!(
            self.settings_for_server(id),
            Some(ContextServerSettings::Registry { .. })
        )
    }

    pub fn is_server_configured_locally(&self, id: &ContextServerId) -> bool {
        self.context_server_settings
            .get(&id.0)
            .is_some_and(|entry| entry.configured_in_project)
    }

    /// Returns whether a server is provided by an extension (as opposed to a
    /// custom Stdio/HTTP server configured directly in settings).
    ///
    /// This is derived from the configured settings rather than the runtime
    /// configuration, so it stays correct even when a custom server is disabled
    /// or has not been started yet (in which case it has no runtime state).
    pub fn is_extension_provided(&self, id: &ContextServerId, cx: &App) -> bool {
        self.source_for_server(id, cx) == ContextServerSource::Extension
    }

    pub fn source_for_server(&self, id: &ContextServerId, cx: &App) -> ContextServerSource {
        match self.settings_for_server(id) {
            Some(ContextServerSettings::Stdio { .. } | ContextServerSettings::Http { .. }) => {
                ContextServerSource::Custom
            }
            Some(ContextServerSettings::Extension { .. }) => ContextServerSource::Extension,
            Some(ContextServerSettings::Registry { .. }) => ContextServerSource::Registry,
            // No custom settings entry: the server can only originate from an
            // extension descriptor in the registry.
            None if self
                .registry
                .read(cx)
                .context_server_descriptor(&id.0)
                .is_some() =>
            {
                ContextServerSource::Extension
            }
            None => ContextServerSource::Custom,
        }
    }

    /// Returns whether a server is enabled.
    /// Servers with no settings entry only originate from an extension
    /// descriptor in the registry, and those are enabled by default
    /// ([`ContextServerSettings::default_extension`]).
    pub fn is_server_enabled(&self, id: &ContextServerId, cx: &App) -> bool {
        if self.is_registry_server(id) && !cx.has_flag::<McpRegistryFeatureFlag>() {
            return false;
        }
        match self.settings_for_server(id) {
            Some(settings) => settings.enabled(),
            None => self
                .registry
                .read(cx)
                .context_server_descriptor(&id.0)
                .is_some(),
        }
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
            let settings_entry = this
                .update(cx, |this, _| {
                    this.context_server_settings.get(&id.0).cloned()
                })
                .context("Failed to get context server settings")?;

            if !settings_entry.settings.enabled() {
                return anyhow::Ok(());
            }

            let (
                registry,
                worktree_store,
                project_environment,
                node_runtime,
                defer_remote_registry_npm,
            ) = this.update(cx, |this, _| {
                (
                    this.registry.clone(),
                    this.worktree_store.clone(),
                    this.project_environment.clone(),
                    this.node_runtime.clone(),
                    this.is_remote_project(),
                )
            });
            let configuration = match ContextServerConfiguration::from_settings(
                settings_entry.settings,
                id.clone(),
                registry,
                worktree_store,
                project_environment,
                node_runtime,
                defer_remote_registry_npm,
                cx,
            )
            .await
            {
                Ok(configuration) => configuration,
                Err(error) => {
                    let error: Arc<str> =
                        format!("Failed to create context server configuration: {error:#}").into();
                    this.update(cx, |this, cx| {
                        this.set_configuration_error(id, error, cx);
                    });
                    return Ok(());
                }
            };

            this.update(cx, |this, cx| {
                let desired_configuration = Arc::new(configuration);
                this.desired_configurations
                    .insert(id.clone(), desired_configuration.clone());
                let effective_configuration = this
                    .servers
                    .get(&id)
                    .map(ContextServerState::configuration)
                    .unwrap_or(desired_configuration);
                this.run_server(server, effective_configuration, cx)
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
        let result = server.stop();
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
        if self.is_registry_server(&id) && !cx.has_flag::<McpRegistryFeatureFlag>() {
            server.stop().log_err();
            self.desired_configurations.remove(&id);
            self.agent_configurations.remove(&id);
            self.set_configuration_error(id, "MCP Registry feature is not enabled".into(), cx);
            return;
        }
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
        let task = cx.spawn({
            let id = server.id();
            let server = server.clone();
            let configuration = configuration.clone();

            async move |this, cx| {
                let new_state = match server.clone().start(cx).await {
                    Ok(_) => {
                        debug_assert!(server.client().is_some());
                        let _transport_watch =
                            Self::watch_transport_shutdown(this.clone(), server.clone(), cx);
                        ContextServerState::Running {
                            server,
                            configuration,
                            _transport_watch,
                        }
                    }
                    Err(err) => resolve_start_failure(&id, err, server, configuration, cx).await,
                };
                this.update(cx, |this, cx| {
                    this.update_server_state(id.clone(), new_state, cx)
                })
                .log_err();
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

    /// Watches a running server's transport for shutdowns that require a
    /// lifecycle transition.
    ///
    /// MCP servers may accept `initialize` unauthenticated and only send a 401
    /// with a `WWW-Authenticate` challenge on a later request or notification.
    /// The HTTP transport records the challenge, and the failed send tears
    /// down the client's output loop. Observing that shutdown — rather than
    /// relying on some request to carry a typed error back to a caller — is
    /// what lets any post-initialize 401 move the server into `AuthRequired`
    /// instead of leaving it `Running` with a dead client.
    fn watch_transport_shutdown(
        this: WeakEntity<Self>,
        server: Arc<ContextServer>,
        cx: &mut AsyncApp,
    ) -> Task<()> {
        let Some(shutdown) = server
            .client()
            .and_then(|client| client.wait_for_shutdown())
        else {
            return Task::ready(());
        };
        cx.spawn(async move |cx| match shutdown.await {
            TransportShutdownReason::AuthRequired(www_authenticate) => {
                this.update(cx, |this, cx| {
                    this.handle_auth_challenge(server, www_authenticate, cx);
                })
                .log_err();
            }
            TransportShutdownReason::SessionExpired => {
                this.update(cx, |this, cx| {
                    this.handle_expired_session(server, cx);
                })
                .log_err();
            }
            TransportShutdownReason::Disconnected | TransportShutdownReason::Other => {
                this.update(cx, |this, cx| {
                    this.handle_transport_disconnect(server, cx);
                })
                .log_err();
            }
        })
    }

    fn handle_transport_disconnect(&mut self, server: Arc<ContextServer>, cx: &mut Context<Self>) {
        let id = server.id();
        let Some(ContextServerState::Running {
            server: running_server,
            configuration,
            ..
        }) = self.servers.get(&id)
        else {
            return;
        };
        if !Arc::ptr_eq(running_server, &server) {
            return;
        }
        let configuration = configuration.clone();

        server.stop().log_err();
        self.update_server_state(
            id,
            ContextServerState::Error {
                server,
                configuration,
                error: "Context server disconnected".into(),
            },
            cx,
        );
    }

    fn handle_expired_session(&mut self, server: Arc<ContextServer>, cx: &mut Context<Self>) {
        let id = server.id();
        let Some(ContextServerState::Running {
            server: running_server,
            configuration,
            ..
        }) = self.servers.get(&id)
        else {
            return;
        };
        if !Arc::ptr_eq(running_server, &server) {
            return;
        }
        let configuration = configuration.clone();

        log::info!("{id} MCP session expired; reinitializing the context server");
        self.run_server(server, configuration, cx);
    }

    fn handle_auth_challenge(
        &mut self,
        server: Arc<ContextServer>,
        www_authenticate: oauth::WwwAuthenticate,
        cx: &mut Context<Self>,
    ) {
        let id = server.id();

        // Act only if this exact server is still the one we consider running.
        // If the state has changed since the challenge was recorded, whoever
        // changed it owns the lifecycle now.
        let Some(ContextServerState::Running {
            server: running_server,
            configuration,
            ..
        }) = self.servers.get(&id)
        else {
            return;
        };
        if !Arc::ptr_eq(running_server, &server) {
            return;
        }
        let configuration = configuration.clone();

        log::info!("{id} received 401 after initialization; initiating OAuth authorization");

        // The 401 already tore down the client's output loop. Stop the dead
        // client, then resolve auth using the captured `WWW-Authenticate` — do
        // not restart via `run_server`, as a fresh `initialize` would succeed
        // and lose the challenge.
        server.stop().log_err();

        let task = cx.spawn({
            let id = id.clone();
            let server = server.clone();
            let configuration = configuration.clone();
            async move |this, cx| {
                let new_state =
                    resolve_auth_required(&id, &www_authenticate, server, configuration, cx).await;
                this.update(cx, |this, cx| {
                    this.update_server_state(id.clone(), new_state, cx)
                })
                .log_err();
            }
        });

        self.update_server_state(
            id,
            ContextServerState::Starting {
                configuration,
                _task: task,
                server,
            },
            cx,
        );
    }

    fn remove_server(&mut self, id: &ContextServerId, cx: &mut Context<Self>) -> Result<()> {
        self.configuration_errors.remove(id);
        self.desired_configurations.remove(id);
        self.agent_configurations.remove(id);
        let state = self
            .servers
            .remove(id)
            .context("Context server not found")?;
        self.server_working_directories.remove(id);

        if let ContextServerConfiguration::Http { url, .. } = state.configuration().as_ref() {
            let server_url = url.clone();
            let id = id.clone();
            cx.spawn(async move |_this, cx| {
                let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
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
        cx.notify();
        Ok(())
    }

    /// The project root a locally-spawned stdio server should use as its working
    /// directory: the active project directory, falling back to the first visible
    /// worktree. Resolves to `None` before any worktree is available.
    fn resolve_root_path(&self, cx: &App) -> Option<Arc<Path>> {
        self.project
            .as_ref()
            .and_then(|project| {
                project
                    .read_with(cx, |project, cx| project.active_project_directory(cx))
                    .ok()
                    .flatten()
            })
            .or_else(|| {
                self.worktree_store.read_with(cx, |store, cx| {
                    store.visible_worktrees(cx).fold(None, |acc, item| {
                        if acc.is_none() {
                            item.read(cx).root_dir()
                        } else {
                            acc
                        }
                    })
                })
            })
    }

    pub async fn create_context_server(
        this: WeakEntity<Self>,
        id: ContextServerId,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut AsyncApp,
    ) -> Result<(
        Arc<ContextServer>,
        Arc<ContextServerConfiguration>,
        Option<Arc<ContextServerConfiguration>>,
    )> {
        let remote = configuration.remote();
        let is_registry_server = matches!(
            configuration.as_ref(),
            ContextServerConfiguration::RemoteRegistryNpm { .. }
        ) || this.update(cx, |this, _| this.is_registry_server(&id))?;
        let mcp_registry_enabled = cx.update(|cx| cx.has_flag::<McpRegistryFeatureFlag>());
        if is_registry_server && !mcp_registry_enabled {
            anyhow::bail!("MCP Registry is disabled");
        }
        let needs_remote_command = match configuration.as_ref() {
            ContextServerConfiguration::Custom { .. }
            | ContextServerConfiguration::Extension { .. }
            | ContextServerConfiguration::RemoteRegistryNpm { .. } => remote,
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

        let root_path: Option<Arc<Path>> =
            this.update(cx, |this, cx| this.resolve_root_path(cx))?;

        let (effective_configuration, agent_configuration) =
            if let Some((project_id, upstream_client)) = remote_state {
                let root_dir = root_path.as_ref().map(|p| p.display().to_string());
                let resolved_registry_npm = match configuration.as_ref() {
                    ContextServerConfiguration::RemoteRegistryNpm {
                        package_spec,
                        runtime_arguments,
                        package_arguments,
                        environment,
                    } => Some(proto::ResolvedRegistryNpmContextServer {
                        package_spec: package_spec.clone(),
                        runtime_arguments: runtime_arguments.clone(),
                        package_arguments: package_arguments.clone(),
                        environment: environment.clone().into_iter().collect(),
                    }),
                    _ => None,
                };

                let response = upstream_client
                    .update(cx, |client, _| {
                        client
                            .proto_client()
                            .request(proto::GetContextServerCommand {
                                project_id,
                                server_id: id.0.to_string(),
                                root_dir: root_dir.clone(),
                                resolved_registry_npm,
                                mcp_registry_enabled,
                            })
                    })
                    .await?;

                if is_registry_server && !cx.update(|cx| cx.has_flag::<McpRegistryFeatureFlag>()) {
                    anyhow::bail!("MCP Registry is disabled");
                }

                let agent_command = ContextServerCommand {
                    path: response.path.clone().into(),
                    args: response.args.clone(),
                    env: Some(
                        response
                            .env
                            .iter()
                            .map(|(name, value)| (name.clone(), value.clone()))
                            .collect(),
                    ),
                    timeout: None,
                };
                let agent_configuration = Arc::new(ContextServerConfiguration::Custom {
                    command: agent_command,
                    remote,
                });

                let remote_command = upstream_client.update(cx, |client, _| {
                    client.build_command(
                        Some(response.path),
                        &response.args,
                        &response.env.into_iter().collect(),
                        root_dir,
                        None,
                        Interactive::Yes,
                    )
                })?;

                let command = ContextServerCommand {
                    path: remote_command.program.into(),
                    args: remote_command.args,
                    env: Some(remote_command.env.into_iter().collect()),
                    timeout: None,
                };

                (
                    Arc::new(ContextServerConfiguration::Custom { command, remote }),
                    Some(agent_configuration),
                )
            } else {
                (configuration, None)
            };

        if let Some(server) = this.update(cx, |this, _| {
            this.context_server_factory
                .as_ref()
                .map(|factory| factory(id.clone(), effective_configuration.clone()))
        })? {
            return Ok((server, effective_configuration, agent_configuration));
        }

        let cached_token_provider: Option<Arc<dyn oauth::OAuthTokenProvider>> =
            if let ContextServerConfiguration::Http { url, .. } = effective_configuration.as_ref() {
                if effective_configuration.has_static_auth_header() {
                    None
                } else {
                    let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
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

        if is_registry_server && !cx.update(|cx| cx.has_flag::<McpRegistryFeatureFlag>()) {
            anyhow::bail!("MCP Registry is disabled");
        }
        let server: Arc<ContextServer> = this.update(cx, |this, cx| {
            let global_timeout = this.timeout_for_server(&id, cx);

            match effective_configuration.as_ref() {
                ContextServerConfiguration::Http {
                    url,
                    headers,
                    timeout,
                    oauth: _,
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
                ContextServerConfiguration::Custom { .. }
                | ContextServerConfiguration::Extension { .. } => {
                    let mut command = effective_configuration
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
                ContextServerConfiguration::RemoteRegistryNpm { .. } => {
                    anyhow::bail!("remote MCP Registry npm configuration was not delegated")
                }
            }
        })??;

        Ok((server, effective_configuration, agent_configuration))
    }

    async fn handle_get_context_server_command(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetContextServerCommand>,
        mut cx: AsyncApp,
    ) -> Result<proto::ContextServerCommand> {
        let payload = envelope.payload;
        let server_id = ContextServerId(payload.server_id.into());

        let (settings_entry, registry, worktree_store, project_environment, node_runtime) = this
            .update(&mut cx, |this, inner_cx| {
                let ContextServerStoreState::Local {
                    is_headless: true, ..
                } = &this.state
                else {
                    anyhow::bail!(
                        "unexpected GetContextServerCommand request in a non-local project"
                    );
                };

                let settings = this
                    .context_server_settings
                    .get(&server_id.0)
                    .cloned()
                    .or_else(|| {
                        this.registry
                            .read(inner_cx)
                            .context_server_descriptor(&server_id.0)
                            .map(|_| ContextServerSettingsEntry {
                                worktree_id: None,
                                configured_in_project: false,
                                settings: ContextServerSettings::default_extension(),
                            })
                    })
                    .with_context(|| format!("context server `{}` not found", server_id))?;

                anyhow::Ok((
                    settings,
                    this.registry.clone(),
                    this.worktree_store.clone(),
                    this.project_environment.clone(),
                    this.node_runtime.clone(),
                ))
            })?;

        if (payload.resolved_registry_npm.is_some()
            || matches!(
                settings_entry.settings,
                ContextServerSettings::Registry { .. }
            ))
            && !payload.mcp_registry_enabled
        {
            anyhow::bail!("MCP Registry is disabled for this request");
        }

        let command = if let Some(resolved_registry_npm) = payload.resolved_registry_npm {
            let ContextServerSettings::Registry {
                enabled: true,
                remote: true,
                registry,
            } = &settings_entry.settings
            else {
                anyhow::bail!(
                    "resolved Registry npm configuration was provided for an ineligible context server"
                );
            };
            let package_name = registry_npm_package_name(&resolved_registry_npm.package_spec)?;
            anyhow::ensure!(
                matches!(
                    registry.source.as_ref(),
                    Some(McpRegistryInstallationSource::Package { registry_type, identifier })
                        if registry_type == "npm" && identifier == package_name
                ),
                "resolved Registry npm configuration does not match the approved installation source"
            );

            ContextServerConfiguration::registry_npm_command(
                &server_id,
                resolved_registry_npm.package_spec,
                resolved_registry_npm.runtime_arguments,
                resolved_registry_npm.package_arguments,
                resolved_registry_npm.environment.into_iter().collect(),
                node_runtime,
                project_environment,
                false,
                &cx,
            )
            .await
            .with_context(|| format!("failed to build configuration for `{}`", server_id))?
        } else {
            let configuration = ContextServerConfiguration::from_settings(
                settings_entry.settings,
                server_id.clone(),
                registry,
                worktree_store,
                project_environment,
                node_runtime,
                false,
                &cx,
            )
            .await
            .with_context(|| format!("failed to build configuration for `{}`", server_id))?;

            configuration
                .command()
                .context("context server has no command (HTTP servers don't need RPC)")?
                .clone()
        };

        Ok(proto::ContextServerCommand {
            path: command.path.display().to_string(),
            args: command.args,
            env: command
                .env
                .map(|env| env.into_iter().collect())
                .unwrap_or_default(),
        })
    }

    /// Merges context server settings from all visible worktrees so that servers defined
    /// in any project folder in a multi-root workspace are picked up.
    fn resolve_all_context_server_settings(
        worktree_store: &Entity<WorktreeStore>,
        cx: &App,
    ) -> HashMap<Arc<str>, ContextServerSettingsEntry> {
        let mut merged = HashMap::default();
        for worktree in worktree_store.read(cx).visible_worktrees(cx) {
            let worktree_id = worktree.read(cx).id();
            let project_server_ids = cx
                .global::<SettingsStore>()
                .local_settings(worktree_id)
                .filter(|(path, _)| path.as_ref() == RelPath::empty())
                .flat_map(|(_, settings)| settings.context_servers.keys().cloned())
                .collect::<HashSet<_>>();

            let location = settings::SettingsLocation {
                worktree_id,
                path: RelPath::empty(),
            };
            for (id, settings) in &ProjectSettings::get(Some(location), cx).context_servers {
                merged
                    .entry(id.clone())
                    .or_insert_with(|| ContextServerSettingsEntry {
                        worktree_id: Some(worktree_id),
                        configured_in_project: project_server_ids.contains(id),
                        settings: settings.clone(),
                    });
            }
        }
        merged
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

    fn timeout_for_server(&self, id: &ContextServerId, cx: &App) -> u64 {
        let worktree_id = self
            .context_server_settings
            .get(&id.0)
            .as_ref()
            .and_then(|entry| entry.worktree_id);

        ProjectSettings::get(
            worktree_id.map(|id| SettingsLocation {
                worktree_id: id,
                path: &RelPath::empty(),
            }),
            cx,
        )
        .context_server_timeout
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
        let global_timeout = self.timeout_for_server(id, cx);

        let (discovery, server, configuration) = match state {
            ContextServerState::AuthRequired {
                discovery,
                server,
                configuration,
            } => (discovery.clone(), server.clone(), configuration.clone()),
            _ => anyhow::bail!("Server is not in AuthRequired state"),
        };

        let needs_keychain_check = match configuration.as_ref() {
            ContextServerConfiguration::Http {
                url,
                oauth: Some(oauth_settings),
                ..
            } if oauth_settings.client_secret.is_none() => Some(url.clone()),
            _ => None,
        };

        let id = id.clone();

        let task = cx.spawn({
            let id = id.clone();
            let server = server.clone();
            let configuration = configuration.clone();
            async move |this, cx| {
                if let Some(server_url) = needs_keychain_check {
                    let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
                    let has_keychain_secret =
                        Self::load_client_secret(&credentials_provider, &server_url, cx)
                            .await
                            .ok()
                            .flatten()
                            .is_some();

                    if !has_keychain_secret {
                        this.update(cx, |this, cx| {
                            this.update_server_state(
                                id.clone(),
                                ContextServerState::ClientSecretRequired {
                                    server,
                                    configuration,
                                    discovery,
                                    error: None,
                                },
                                cx,
                            );
                        })
                        .log_err();
                        return;
                    }
                }

                let result = Self::run_oauth_flow(
                    this.clone(),
                    id.clone(),
                    discovery.clone(),
                    configuration.clone(),
                    global_timeout,
                    cx,
                )
                .await;

                if let Err(err) = &result {
                    log::error!("{} OAuth authentication failed: {:?}", id, err);
                    this.update(cx, |this, cx| {
                        this.update_server_state(
                            id.clone(),
                            ContextServerState::Error {
                                server,
                                configuration,
                                error: format!("{err:#}").into(),
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

    /// Store the client secret and proceed with authentication.
    pub fn submit_client_secret(
        &mut self,
        id: &ContextServerId,
        secret: String,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let state = self.servers.get(id).context("Context server not found")?;
        let global_timeout = self.timeout_for_server(id, cx);

        let (server, configuration, discovery) = match state {
            ContextServerState::ClientSecretRequired {
                server,
                configuration,
                discovery,
                ..
            } => (server.clone(), configuration.clone(), discovery.clone()),
            _ => anyhow::bail!("Server is not in ClientSecretRequired state"),
        };

        let server_url = match configuration.as_ref() {
            ContextServerConfiguration::Http { url, .. } => url.clone(),
            _ => anyhow::bail!("OAuth only supported for HTTP servers"),
        };

        let id = id.clone();

        let task = cx.spawn({
            let id = id.clone();
            let server = server.clone();
            let configuration = configuration.clone();
            async move |this, cx| {
                // Store the secret if non-empty (empty means public client / skip).
                if !secret.is_empty() {
                    let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
                    if let Err(err) =
                        Self::store_client_secret(&credentials_provider, &server_url, &secret, cx)
                            .await
                    {
                        log::error!(
                            "{} failed to store client secret in keychain: {:?}",
                            id,
                            err
                        );
                    }
                }

                let result = Self::run_oauth_flow(
                    this.clone(),
                    id.clone(),
                    discovery.clone(),
                    configuration.clone(),
                    global_timeout,
                    cx,
                )
                .await;

                if let Err(err) = &result {
                    log::error!("{} OAuth authentication failed: {:?}", id, err);

                    let is_bad_client_credentials = err
                        .downcast_ref::<oauth::OAuthTokenError>()
                        .is_some_and(|e| e.error == "unauthorized_client");

                    if is_bad_client_credentials {
                        // Clear the bad secret from the keychain so the user
                        // gets a fresh prompt.
                        let credentials_provider =
                            cx.update(|cx| zed_credentials_provider::global(cx));
                        Self::clear_client_secret(&credentials_provider, &server_url, cx)
                            .await
                            .log_err();

                        this.update(cx, |this, cx| {
                            this.update_server_state(
                                id.clone(),
                                ContextServerState::ClientSecretRequired {
                                    server,
                                    configuration,
                                    discovery,
                                    error: Some(format!("{err:#}").into()),
                                },
                                cx,
                            );
                        })
                        .log_err();
                    } else {
                        this.update(cx, |this, cx| {
                            this.update_server_state(
                                id.clone(),
                                ContextServerState::Error {
                                    server,
                                    configuration,
                                    error: format!("{err:#}").into(),
                                },
                                cx,
                            )
                        })
                        .log_err();
                    }
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
        global_timeout: u64,
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
        let (redirect_uri, callback_rx) =
            oauth::start_callback_server().context("Failed to start OAuth callback server")?;

        let http_client = cx.update(|cx| cx.http_client());
        let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
        let server_url = match configuration.as_ref() {
            ContextServerConfiguration::Http { url, .. } => url.clone(),
            _ => anyhow::bail!("OAuth authentication only supported for HTTP servers"),
        };

        let client_registration = match configuration.as_ref() {
            ContextServerConfiguration::Http {
                url,
                oauth: Some(oauth_settings),
                ..
            } => {
                // Pre-registered client. Resolve the secret from settings, then keychain.
                let client_secret = if oauth_settings.client_secret.is_some() {
                    oauth_settings.client_secret.clone()
                } else {
                    Self::load_client_secret(&credentials_provider, url, cx)
                        .await
                        .ok()
                        .flatten()
                };
                oauth::OAuthClientRegistration {
                    client_id: oauth_settings.client_id.clone(),
                    client_secret,
                }
            }
            _ => oauth::resolve_client_registration(&http_client, &discovery, &redirect_uri)
                .await
                .context("Failed to resolve OAuth client registration")?,
        };

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
            client_registration.client_secret.as_deref(),
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

        let new_server = this.update(cx, |_this, cx| match configuration.as_ref() {
            ContextServerConfiguration::Http {
                url,
                headers,
                timeout,
                oauth: _,
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

    fn client_secret_keychain_key(server_url: &url::Url) -> String {
        format!(
            "mcp-oauth-client-secret:{}",
            oauth::canonical_server_uri(server_url)
        )
    }

    async fn load_client_secret(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<Option<String>> {
        let key = Self::client_secret_keychain_key(server_url);
        match credentials_provider.read_credentials(&key, cx).await? {
            Some((_username, secret_bytes)) => Ok(Some(String::from_utf8(secret_bytes)?)),
            None => Ok(None),
        }
    }

    pub async fn store_client_secret(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        secret: &str,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::client_secret_keychain_key(server_url);
        credentials_provider
            .write_credentials(&key, "mcp-oauth-client-secret", secret.as_bytes(), cx)
            .await
    }

    async fn clear_client_secret(
        credentials_provider: &Arc<dyn CredentialsProvider>,
        server_url: &url::Url,
        cx: &AsyncApp,
    ) -> Result<()> {
        let key = Self::client_secret_keychain_key(server_url);
        credentials_provider.delete_credentials(&key, cx).await
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

        cx.spawn(async move |this, cx| {
            let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
            if let Err(err) = Self::clear_session(&credentials_provider, &server_url, &cx).await {
                log::error!("{} failed to clear OAuth session: {}", id, err);
            }
            // Also clear any client secret so the user gets a fresh prompt on
            // the next authentication attempt.
            Self::clear_client_secret(&credentials_provider, &server_url, &cx)
                .await
                .log_err();
            // Trigger server recreation so the next start uses a fresh
            // transport without the old (now-invalidated) token provider.
            this.update(cx, |this, cx| {
                this.available_context_servers_changed(cx);
            })
            .log_err();
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
        self.configuration_errors.remove(&id);
        let status = ContextServerStatus::from_state(&state);
        self.servers.insert(id.clone(), state);
        cx.emit(ServerStatusChangedEvent {
            server_id: id,
            status,
        });
        cx.notify();
    }

    fn set_configuration_error(
        &mut self,
        id: ContextServerId,
        error: Arc<str>,
        cx: &mut Context<Self>,
    ) {
        self.configuration_errors.insert(id.clone(), error.clone());
        cx.emit(ServerStatusChangedEvent {
            server_id: id,
            status: ContextServerStatus::Error(error),
        });
        cx.notify();
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
                    this.stop_server(&id, cx).log_err();
                }
                this.configuration_errors.clear();
            })?;
            return Ok(());
        }

        let (
            mut configured_servers,
            registry,
            worktree_store,
            project_environment,
            node_runtime,
            defer_remote_registry_npm,
        ) = this.update(cx, |this, _| {
            (
                this.context_server_settings.clone(),
                this.registry.clone(),
                this.worktree_store.clone(),
                this.project_environment.clone(),
                this.node_runtime.clone(),
                this.is_remote_project(),
            )
        })?;

        for (id, _) in registry.read_with(cx, |registry, _| registry.context_server_descriptors()) {
            configured_servers
                .entry(id)
                .or_insert(ContextServerSettingsEntry {
                    worktree_id: None,
                    configured_in_project: false,
                    settings: ContextServerSettings::default_extension(),
                });
        }

        let (enabled_servers, disabled_servers): (HashMap<_, _>, HashMap<_, _>) =
            configured_servers
                .into_iter()
                .partition(|(_, entry)| entry.settings.enabled());

        let resolved_servers = join_all(enabled_servers.into_iter().map(|(id, settings_entry)| {
            let id = ContextServerId(id);
            ContextServerConfiguration::from_settings(
                settings_entry.settings,
                id.clone(),
                registry.clone(),
                worktree_store.clone(),
                project_environment.clone(),
                node_runtime.clone(),
                defer_remote_registry_npm,
                cx,
            )
            .map(move |config| (id, config))
        }))
        .await;

        let mut configured_servers = HashMap::default();
        let mut configuration_errors = HashMap::default();
        for (id, result) in resolved_servers {
            match result {
                Ok(configuration) => {
                    configured_servers.insert(id, configuration);
                }
                Err(error) => {
                    log::error!("{id} context server configuration failed: {error:#}");
                    configuration_errors.insert(id, Arc::from(format!("{error:#}")));
                }
            }
        }

        let mut servers_to_start = Vec::new();
        let mut servers_to_remove = HashSet::default();
        let mut servers_to_stop = HashSet::default();

        this.update(cx, |this, cx| {
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

            let is_remote_project = this.is_remote_project();
            let root_path = this.resolve_root_path(cx);

            for (id, config) in configured_servers {
                let state = this.servers.get(&id);
                let is_stopped = matches!(state, Some(ContextServerState::Stopped { .. }));
                let existing_config = this.desired_configurations.get(&id);
                let working_directory =
                    working_directory_for(&config, root_path.clone(), is_remote_project);
                // A running server that was started before the project root became
                // available keeps its stale working directory, since the working
                // directory is not part of `ContextServerConfiguration`. Restart it
                // when the resolved working directory no longer matches.
                let working_directory_changed = state.is_some()
                    && !is_stopped
                    && this.server_working_directories.get(&id) != Some(&working_directory);
                if existing_config.map(AsRef::as_ref) != Some(&config)
                    || is_stopped
                    || working_directory_changed
                {
                    let config = Arc::new(config);
                    servers_to_start.push((id.clone(), config, working_directory));
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
            for (id, _, _) in &servers_to_start {
                this.agent_configurations.remove(id);
            }
            anyhow::Ok(())
        })??;

        this.update(cx, |this, cx| {
            this.configuration_errors.clear();
            for (id, error) in configuration_errors {
                this.set_configuration_error(id, error, cx);
            }
        })?;

        for (id, config, working_directory) in servers_to_start {
            match Self::create_context_server(this.clone(), id.clone(), config.clone(), cx).await {
                Ok((server, effective_configuration, agent_configuration)) => {
                    this.update(cx, |this, cx| {
                        this.server_working_directories
                            .insert(id.clone(), working_directory);
                        this.desired_configurations.insert(id.clone(), config);
                        if let Some(agent_configuration) = agent_configuration {
                            this.agent_configurations
                                .insert(id.clone(), agent_configuration);
                        } else {
                            this.agent_configurations.remove(&id);
                        }
                        this.run_server(server, effective_configuration, cx);
                    })?;
                }
                Err(err) => {
                    log::error!("{id} context server failed to create: {err:#}");
                    this.update(cx, |this, cx| {
                        this.set_configuration_error(id, err.to_string().into(), cx);
                    })?;
                }
            }
        }

        Ok(())
    }
}

/// The working directory a server will be spawned with, mirroring the choice
/// made in [`ContextServerStore::create_context_server`]: only locally-spawned
/// stdio servers use the project root; HTTP and remote servers use none.
fn working_directory_for(
    configuration: &ContextServerConfiguration,
    root_path: Option<Arc<Path>>,
    is_remote_project: bool,
) -> Option<Arc<Path>> {
    match configuration {
        ContextServerConfiguration::Http { .. } => None,
        _ if is_remote_project => None,
        _ => root_path,
    }
}

fn select_agent_configuration(
    is_remote_project: bool,
    has_configuration_error: bool,
    agent_configuration: Option<Arc<ContextServerConfiguration>>,
    effective_configuration: Option<Arc<ContextServerConfiguration>>,
) -> Option<Arc<ContextServerConfiguration>> {
    if has_configuration_error {
        return None;
    }

    if is_remote_project
        && effective_configuration
            .as_deref()
            .is_some_and(ContextServerConfiguration::remote)
    {
        return agent_configuration;
    }

    agent_configuration.or(effective_configuration)
}

/// Determines the appropriate server state after a start attempt fails.
///
/// When the error is an HTTP 401 with no static auth header configured,
/// attempts OAuth discovery so the UI can offer an authentication flow.
async fn resolve_start_failure(
    id: &ContextServerId,
    err: anyhow::Error,
    server: Arc<ContextServer>,
    configuration: Arc<ContextServerConfiguration>,
    cx: &AsyncApp,
) -> ContextServerState {
    // Read the challenge from the transport rather than downcasting `err`: it
    // is recorded before the failed send's error propagates, so a 401 is
    // recognized even when another error (e.g. the request timeout) wins the
    // race to become the reported startup failure.
    let www_authenticate = server.auth_challenge();

    // When the error is NOT a 401 but there is a cached OAuth session in the
    // keychain, the session is likely stale/expired and caused the failure
    // (e.g. timeout because the server rejected the token silently). Clear it
    // so the next start attempt can get a clean 401 and trigger the auth flow.
    // If there is no such session this is an ordinary startup error.
    if www_authenticate.is_none() {
        let server_url = match configuration.as_ref() {
            ContextServerConfiguration::Http { url, .. }
                if !configuration.has_static_auth_header() =>
            {
                url.clone()
            }
            _ => {
                log::error!("{id} context server failed to start: {err}");
                return ContextServerState::Error {
                    configuration,
                    server,
                    error: err.to_string().into(),
                };
            }
        };

        let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
        match ContextServerStore::load_session(&credentials_provider, &server_url, cx).await {
            Ok(Some(_)) => {
                log::info!("{id} start failed with a cached OAuth session present; clearing it");
                ContextServerStore::clear_session(&credentials_provider, &server_url, cx)
                    .await
                    .log_err();
            }
            _ => {
                log::error!("{id} context server failed to start: {err}");
                return ContextServerState::Error {
                    configuration,
                    server,
                    error: err.to_string().into(),
                };
            }
        }
    }

    let default_www_authenticate = oauth::WwwAuthenticate {
        resource_metadata: None,
        scope: None,
        error: None,
        error_description: None,
    };
    let www_authenticate = www_authenticate
        .as_ref()
        .unwrap_or(&default_www_authenticate);

    resolve_auth_required(id, www_authenticate, server, configuration, cx).await
}

/// Runs OAuth discovery for a server that returned a 401 and produces the
/// appropriate state (`AuthRequired`, `ClientSecretRequired`, or `Error`).
///
/// Shared by the startup path ([`resolve_start_failure`]) and the
/// post-initialize path ([`ContextServerStore::handle_auth_challenge`]) so
/// that a 401 at any point — not only during `initialize` — can initiate the
/// OAuth flow.
async fn resolve_auth_required(
    id: &ContextServerId,
    www_authenticate: &oauth::WwwAuthenticate,
    server: Arc<ContextServer>,
    configuration: Arc<ContextServerConfiguration>,
    cx: &AsyncApp,
) -> ContextServerState {
    if configuration.has_static_auth_header() {
        log::warn!("{id} received 401 with a static Authorization header configured");
        return ContextServerState::Error {
            configuration,
            server,
            error: "Server returned 401 Unauthorized. Check your configured Authorization header."
                .into(),
        };
    }

    let server_url = match configuration.as_ref() {
        ContextServerConfiguration::Http { url, .. } => url.clone(),
        _ => {
            log::error!("{id} got OAuth 401 on a non-HTTP transport");
            return ContextServerState::Error {
                configuration,
                server,
                error: "Server returned 401 Unauthorized on a non-HTTP transport".into(),
            };
        }
    };

    let http_client = cx.update(|cx| cx.http_client());

    match context_server::oauth::discover(&http_client, &server_url, www_authenticate).await {
        Ok(discovery) => {
            use context_server::oauth::{
                ClientRegistrationStrategy, determine_registration_strategy,
            };

            let has_preregistered_client_id = matches!(
                configuration.as_ref(),
                ContextServerConfiguration::Http { oauth: Some(_), .. }
            );

            let strategy = determine_registration_strategy(&discovery.auth_server_metadata);

            if matches!(strategy, ClientRegistrationStrategy::Unavailable)
                && !has_preregistered_client_id
            {
                log::error!(
                    "{id} authorization server supports neither CIMD nor DCR, \
                     and no pre-registered client_id is configured"
                );
                return ContextServerState::Error {
                    configuration,
                    server,
                    error: "Authorization server supports neither CIMD nor DCR. \
                            Configure a pre-registered client_id in your settings \
                            under the \"oauth\" key."
                        .into(),
                };
            }

            log::info!(
                "{id} requires OAuth authorization (auth server: {})",
                discovery.auth_server_metadata.issuer,
            );
            ContextServerState::AuthRequired {
                server,
                configuration,
                discovery: Arc::new(discovery),
            }
        }
        Err(discovery_err) => {
            log::error!("{id} OAuth discovery failed: {discovery_err}");
            ContextServerState::Error {
                configuration,
                server,
                error: format!("OAuth discovery failed: {discovery_err}").into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stdio_configuration(path: &str, remote: bool) -> Arc<ContextServerConfiguration> {
        Arc::new(ContextServerConfiguration::Custom {
            command: ContextServerCommand {
                path: path.into(),
                args: Vec::new(),
                env: None,
                timeout: None,
            },
            remote,
        })
    }

    #[test]
    fn selects_native_agent_configuration_for_remote_servers() {
        let native_configuration = stdio_configuration("/remote/bin/server", true);
        let effective_configuration = stdio_configuration("ssh", true);

        assert_eq!(
            select_agent_configuration(
                true,
                false,
                Some(native_configuration.clone()),
                Some(effective_configuration.clone()),
            ),
            Some(native_configuration)
        );
        assert_eq!(
            select_agent_configuration(true, false, None, Some(effective_configuration.clone())),
            None
        );
        assert_eq!(
            select_agent_configuration(
                true,
                true,
                Some(stdio_configuration("/remote/bin/server", true)),
                Some(effective_configuration),
            ),
            None
        );

        let local_configuration = stdio_configuration("/usr/bin/server", false);
        assert_eq!(
            select_agent_configuration(false, false, None, Some(local_configuration.clone()),),
            Some(local_configuration)
        );
    }

    #[test]
    fn rejects_registry_package_credentials_only_for_remote_headless_execution() {
        let source = McpRegistryInstallationSource::Package {
            registry_type: "npm".to_owned(),
            identifier: "@example/secret-package".to_owned(),
        };

        assert!(!should_reject_remote_registry_package_credentials(
            &source, true, true, false,
        ));
        assert!(should_reject_remote_registry_package_credentials(
            &source, true, true, true,
        ));
        assert!(!should_reject_remote_registry_package_credentials(
            &source, true, false, true,
        ));
    }
}
