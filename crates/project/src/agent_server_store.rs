use remote::Interactive;
use std::{
    any::Any,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, Result, bail};
use collections::HashMap;
use fs::Fs;
use gpui::{AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription, Task};
use http_client::{HttpClient, github::AssetKind};
use node_runtime::NodeRuntime;
use percent_encoding::percent_decode_str;
use remote::RemoteClient;
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, ExternalExtensionAgent},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{RegisterSetting, SettingsStore};
use sha2::{Digest, Sha256};
use task::Shell;
use url::Url;
use util::{ResultExt as _, debug_panic};

use crate::ProjectEnvironment;
use crate::agent_registry_store::{AgentRegistryStore, RegistryAgent, RegistryTargetConfig};

use crate::worktree_store::WorktreeStore;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct AgentServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

impl std::fmt::Debug for AgentServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filtered_env = self.env.as_ref().map(|env| {
            env.iter()
                .map(|(k, v)| {
                    (
                        k,
                        if util::redact::should_redact(k) {
                            "[REDACTED]"
                        } else {
                            v
                        },
                    )
                })
                .collect::<Vec<_>>()
        });

        f.debug_struct("AgentServerCommand")
            .field("path", &self.path)
            .field("args", &self.args)
            .field("env", &filtered_env)
            .finish()
    }
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct AgentId(pub SharedString);

impl AgentId {
    pub fn new(id: impl Into<SharedString>) -> Self {
        AgentId(id.into())
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for AgentId {
    fn from(value: &'static str) -> Self {
        AgentId(value.into())
    }
}

impl From<AgentId> for SharedString {
    fn from(value: AgentId) -> Self {
        value.0
    }
}

impl AsRef<str> for AgentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for AgentId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExternalAgentSource {
    #[default]
    Custom,
    Extension,
    Registry,
}

pub trait ExternalAgentServer {
    fn get_command(
        &mut self,
        extra_env: HashMap<String, String>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<AgentServerCommand>>;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn ExternalAgentServer {
    fn downcast_mut<T: ExternalAgentServer + 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

enum AgentServerStoreState {
    Local {
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        project_environment: Entity<ProjectEnvironment>,
        downstream_client: Option<(u64, AnyProtoClient)>,
        settings: Option<AllAgentServersSettings>,
        http_client: Arc<dyn HttpClient>,
        extension_agents: Vec<(
            Arc<str>,
            String,
            HashMap<String, extension::TargetConfig>,
            HashMap<String, String>,
            Option<String>,
            Option<SharedString>,
        )>,
        _subscriptions: Vec<Subscription>,
    },
    Remote {
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
        worktree_store: Entity<WorktreeStore>,
    },
    Collab,
}

pub struct ExternalAgentEntry {
    server: Box<dyn ExternalAgentServer>,
    icon: Option<SharedString>,
    display_name: Option<SharedString>,
    pub source: ExternalAgentSource,
}

impl ExternalAgentEntry {
    pub fn new(
        server: Box<dyn ExternalAgentServer>,
        source: ExternalAgentSource,
        icon: Option<SharedString>,
        display_name: Option<SharedString>,
    ) -> Self {
        Self {
            server,
            icon,
            display_name,
            source,
        }
    }
}

pub struct AgentServerStore {
    state: AgentServerStoreState,
    pub external_agents: HashMap<AgentId, ExternalAgentEntry>,
}

pub struct AgentServersUpdated;

impl EventEmitter<AgentServersUpdated> for AgentServerStore {}

impl AgentServerStore {
    /// Synchronizes extension-provided agent servers with the store.
    pub fn sync_extension_agents<'a, I>(
        &mut self,
        manifests: I,
        extensions_dir: PathBuf,
        cx: &mut Context<Self>,
    ) where
        I: IntoIterator<Item = (&'a str, &'a extension::ExtensionManifest)>,
    {
        // Collect manifests first so we can iterate twice
        let manifests: Vec<_> = manifests.into_iter().collect();

        // Remove all extension-provided agents
        // (They will be re-added below if they're in the currently installed extensions)
        self.external_agents
            .retain(|_, entry| entry.source != ExternalAgentSource::Extension);

        // Insert agent servers from extension manifests
        match &mut self.state {
            AgentServerStoreState::Local {
                extension_agents, ..
            } => {
                extension_agents.clear();
                for (ext_id, manifest) in manifests {
                    for (agent_name, agent_entry) in &manifest.agent_servers {
                        let display_name = SharedString::from(agent_entry.name.clone());
                        let icon_path = agent_entry.icon.as_ref().and_then(|icon| {
                            resolve_extension_icon_path(&extensions_dir, ext_id, icon)
                        });

                        extension_agents.push((
                            agent_name.clone(),
                            ext_id.to_owned(),
                            agent_entry.targets.clone(),
                            agent_entry.env.clone(),
                            icon_path,
                            Some(display_name),
                        ));
                    }
                }
                self.reregister_agents(cx);
            }
            AgentServerStoreState::Remote {
                project_id,
                upstream_client,
                worktree_store,
            } => {
                let mut agents = vec![];
                for (ext_id, manifest) in manifests {
                    for (agent_name, agent_entry) in &manifest.agent_servers {
                        let display_name = SharedString::from(agent_entry.name.clone());
                        let icon_path = agent_entry.icon.as_ref().and_then(|icon| {
                            resolve_extension_icon_path(&extensions_dir, ext_id, icon)
                        });
                        let icon_shared = icon_path
                            .as_ref()
                            .map(|path| SharedString::from(path.clone()));
                        let icon = icon_path;
                        let agent_server_name = AgentId(agent_name.clone().into());
                        self.external_agents
                            .entry(agent_server_name.clone())
                            .and_modify(|entry| {
                                entry.icon = icon_shared.clone();
                                entry.display_name = Some(display_name.clone());
                                entry.source = ExternalAgentSource::Extension;
                            })
                            .or_insert_with(|| {
                                ExternalAgentEntry::new(
                                    Box::new(RemoteExternalAgentServer {
                                        project_id: *project_id,
                                        upstream_client: upstream_client.clone(),
                                        worktree_store: worktree_store.clone(),
                                        name: agent_server_name.clone(),
                                        new_version_available_tx: None,
                                    })
                                        as Box<dyn ExternalAgentServer>,
                                    ExternalAgentSource::Extension,
                                    icon_shared.clone(),
                                    Some(display_name.clone()),
                                )
                            });

                        agents.push(ExternalExtensionAgent {
                            name: agent_name.to_string(),
                            icon_path: icon,
                            extension_id: ext_id.to_string(),
                            targets: agent_entry
                                .targets
                                .iter()
                                .map(|(k, v)| (k.clone(), v.to_proto()))
                                .collect(),
                            env: agent_entry
                                .env
                                .iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect(),
                        });
                    }
                }
                upstream_client
                    .read(cx)
                    .proto_client()
                    .send(proto::ExternalExtensionAgentsUpdated {
                        project_id: *project_id,
                        agents,
                    })
                    .log_err();
            }
            AgentServerStoreState::Collab => {
                // Do nothing
            }
        }

        cx.emit(AgentServersUpdated);
    }

    pub fn agent_icon(&self, id: &AgentId) -> Option<SharedString> {
        self.external_agents
            .get(id)
            .and_then(|entry| entry.icon.clone())
    }

    pub fn agent_source(&self, name: &AgentId) -> Option<ExternalAgentSource> {
        self.external_agents.get(name).map(|entry| entry.source)
    }
}

/// Safely resolves an extension icon path, ensuring it stays within the extension directory.
/// Returns `None` if the path would escape the extension directory (path traversal attack).
pub fn resolve_extension_icon_path(
    extensions_dir: &Path,
    extension_id: &str,
    icon_relative_path: &str,
) -> Option<String> {
    let extension_root = extensions_dir.join(extension_id);
    let icon_path = extension_root.join(icon_relative_path);

    // Canonicalize both paths to resolve symlinks and normalize the paths.
    // For the extension root, we need to handle the case where it might be a symlink
    // (common for dev extensions).
    let canonical_extension_root = extension_root.canonicalize().unwrap_or(extension_root);
    let canonical_icon_path = match icon_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            log::warn!(
                "Failed to canonicalize icon path for extension '{}': {} (path: {})",
                extension_id,
                err,
                icon_relative_path
            );
            return None;
        }
    };

    // Verify the resolved icon path is within the extension directory
    if canonical_icon_path.starts_with(&canonical_extension_root) {
        Some(canonical_icon_path.to_string_lossy().to_string())
    } else {
        log::warn!(
            "Icon path '{}' for extension '{}' escapes extension directory, ignoring for security",
            icon_relative_path,
            extension_id
        );
        None
    }
}

impl AgentServerStore {
    pub fn agent_display_name(&self, name: &AgentId) -> Option<SharedString> {
        self.external_agents
            .get(name)
            .and_then(|entry| entry.display_name.clone())
    }

    pub fn init_remote(session: &AnyProtoClient) {
        session.add_entity_message_handler(Self::handle_external_agents_updated);
        session.add_entity_message_handler(Self::handle_new_version_available);
    }

    pub fn init_headless(session: &AnyProtoClient) {
        session.add_entity_message_handler(Self::handle_external_extension_agents_updated);
        session.add_entity_request_handler(Self::handle_get_agent_server_command);
    }

    fn agent_servers_settings_changed(&mut self, cx: &mut Context<Self>) {
        let AgentServerStoreState::Local {
            settings: old_settings,
            ..
        } = &mut self.state
        else {
            debug_panic!(
                "should not be subscribed to agent server settings changes in non-local project"
            );
            return;
        };

        let new_settings = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None)
            .clone();
        if Some(&new_settings) == old_settings.as_ref() {
            return;
        }

        self.reregister_agents(cx);
    }

    fn reregister_agents(&mut self, cx: &mut Context<Self>) {
        let AgentServerStoreState::Local {
            node_runtime,
            fs,
            project_environment,
            downstream_client,
            settings: old_settings,
            http_client,
            extension_agents,
            ..
        } = &mut self.state
        else {
            debug_panic!("Non-local projects should never attempt to reregister. This is a bug!");

            return;
        };

        let new_settings = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None)
            .clone();

        // If we don't have agents from the registry loaded yet, trigger a
        // refresh, which will cause this function to be called again
        let registry_store = AgentRegistryStore::try_global(cx);
        if new_settings.has_registry_agents()
            && let Some(registry) = registry_store.as_ref()
        {
            registry.update(cx, |registry, cx| registry.refresh_if_stale(cx));
        }

        let registry_agents_by_id = registry_store
            .as_ref()
            .map(|store| {
                store
                    .read(cx)
                    .agents()
                    .iter()
                    .cloned()
                    .map(|agent| (agent.id().to_string(), agent))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        self.external_agents.clear();

        // Insert extension agents before custom/registry so registry entries override extensions.
        for (agent_name, ext_id, targets, env, icon_path, display_name) in extension_agents.iter() {
            let name = AgentId(agent_name.clone().into());
            let mut env = env.clone();
            if let Some(settings_env) =
                new_settings
                    .get(agent_name.as_ref())
                    .and_then(|settings| match settings {
                        CustomAgentServerSettings::Extension { env, .. } => Some(env.clone()),
                        _ => None,
                    })
            {
                env.extend(settings_env);
            }
            let icon = icon_path
                .as_ref()
                .map(|path| SharedString::from(path.clone()));

            self.external_agents.insert(
                name.clone(),
                ExternalAgentEntry::new(
                    Box::new(LocalExtensionArchiveAgent {
                        fs: fs.clone(),
                        http_client: http_client.clone(),
                        node_runtime: node_runtime.clone(),
                        project_environment: project_environment.clone(),
                        extension_id: Arc::from(&**ext_id),
                        targets: targets.clone(),
                        env,
                        agent_id: agent_name.clone(),
                    }) as Box<dyn ExternalAgentServer>,
                    ExternalAgentSource::Extension,
                    icon,
                    display_name.clone(),
                ),
            );
        }

        for (name, settings) in new_settings.iter() {
            match settings {
                CustomAgentServerSettings::Custom { command, .. } => {
                    let agent_name = AgentId(name.clone().into());
                    self.external_agents.insert(
                        agent_name.clone(),
                        ExternalAgentEntry::new(
                            Box::new(LocalCustomAgent {
                                command: command.clone(),
                                project_environment: project_environment.clone(),
                            }) as Box<dyn ExternalAgentServer>,
                            ExternalAgentSource::Custom,
                            None,
                            None,
                        ),
                    );
                }
                CustomAgentServerSettings::Registry { env, .. } => {
                    let Some(agent) = registry_agents_by_id.get(name) else {
                        if registry_store.is_some() {
                            log::debug!("Registry agent '{}' not found in ACP registry", name);
                        }
                        continue;
                    };

                    let agent_name = AgentId(name.clone().into());
                    match agent {
                        RegistryAgent::Binary(agent) => {
                            if !agent.supports_current_platform {
                                log::warn!(
                                    "Registry agent '{}' has no compatible binary for this platform",
                                    name
                                );
                                continue;
                            }

                            self.external_agents.insert(
                                agent_name.clone(),
                                ExternalAgentEntry::new(
                                    Box::new(LocalRegistryArchiveAgent {
                                        fs: fs.clone(),
                                        http_client: http_client.clone(),
                                        node_runtime: node_runtime.clone(),
                                        project_environment: project_environment.clone(),
                                        registry_id: Arc::from(name.as_str()),
                                        targets: agent.targets.clone(),
                                        env: env.clone(),
                                    })
                                        as Box<dyn ExternalAgentServer>,
                                    ExternalAgentSource::Registry,
                                    agent.metadata.icon_path.clone(),
                                    Some(agent.metadata.name.clone()),
                                ),
                            );
                        }
                        RegistryAgent::Npx(agent) => {
                            self.external_agents.insert(
                                agent_name.clone(),
                                ExternalAgentEntry::new(
                                    Box::new(LocalRegistryNpxAgent {
                                        node_runtime: node_runtime.clone(),
                                        project_environment: project_environment.clone(),
                                        package: agent.package.clone(),
                                        args: agent.args.clone(),
                                        distribution_env: agent.env.clone(),
                                        settings_env: env.clone(),
                                    })
                                        as Box<dyn ExternalAgentServer>,
                                    ExternalAgentSource::Registry,
                                    agent.metadata.icon_path.clone(),
                                    Some(agent.metadata.name.clone()),
                                ),
                            );
                        }
                    }
                }
                CustomAgentServerSettings::Extension { .. } => {}
            }
        }

        *old_settings = Some(new_settings);

        if let Some((project_id, downstream_client)) = downstream_client {
            downstream_client
                .send(proto::ExternalAgentsUpdated {
                    project_id: *project_id,
                    names: self
                        .external_agents
                        .keys()
                        .map(|name| name.to_string())
                        .collect(),
                })
                .log_err();
        }
        cx.emit(AgentServersUpdated);
    }

    pub fn node_runtime(&self) -> Option<NodeRuntime> {
        match &self.state {
            AgentServerStoreState::Local { node_runtime, .. } => Some(node_runtime.clone()),
            _ => None,
        }
    }

    pub fn local(
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        project_environment: Entity<ProjectEnvironment>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = vec![cx.observe_global::<SettingsStore>(|this, cx| {
            this.agent_servers_settings_changed(cx);
        })];
        if let Some(registry_store) = AgentRegistryStore::try_global(cx) {
            subscriptions.push(cx.observe(&registry_store, |this, _, cx| {
                this.reregister_agents(cx);
            }));
        }
        let mut this = Self {
            state: AgentServerStoreState::Local {
                node_runtime,
                fs,
                project_environment,
                http_client,
                downstream_client: None,
                settings: None,
                extension_agents: vec![],
                _subscriptions: subscriptions,
            },
            external_agents: HashMap::default(),
        };
        if let Some(_events) = extension::ExtensionEvents::try_global(cx) {}
        this.agent_servers_settings_changed(cx);
        this
    }

    pub(crate) fn remote(
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
        worktree_store: Entity<WorktreeStore>,
    ) -> Self {
        Self {
            state: AgentServerStoreState::Remote {
                project_id,
                upstream_client,
                worktree_store,
            },
            external_agents: HashMap::default(),
        }
    }

    pub fn collab() -> Self {
        Self {
            state: AgentServerStoreState::Collab,
            external_agents: HashMap::default(),
        }
    }

    pub fn shared(&mut self, project_id: u64, client: AnyProtoClient, cx: &mut Context<Self>) {
        match &mut self.state {
            AgentServerStoreState::Local {
                downstream_client, ..
            } => {
                *downstream_client = Some((project_id, client.clone()));
                // Send the current list of external agents downstream, but only after a delay,
                // to avoid having the message arrive before the downstream project's agent server store
                // sets up its handlers.
                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    let names = this.update(cx, |this, _| {
                        this.external_agents()
                            .map(|name| name.to_string())
                            .collect()
                    })?;
                    client
                        .send(proto::ExternalAgentsUpdated { project_id, names })
                        .log_err();
                    anyhow::Ok(())
                })
                .detach();
            }
            AgentServerStoreState::Remote { .. } => {
                debug_panic!(
                    "external agents over collab not implemented, remote project should not be shared"
                );
            }
            AgentServerStoreState::Collab => {
                debug_panic!("external agents over collab not implemented, should not be shared");
            }
        }
    }

    pub fn get_external_agent(
        &mut self,
        name: &AgentId,
    ) -> Option<&mut (dyn ExternalAgentServer + 'static)> {
        self.external_agents
            .get_mut(name)
            .map(|entry| entry.server.as_mut())
    }

    pub fn no_browser(&self) -> bool {
        match &self.state {
            AgentServerStoreState::Local {
                downstream_client, ..
            } => downstream_client
                .as_ref()
                .is_some_and(|(_, client)| !client.has_wsl_interop()),
            _ => false,
        }
    }

    pub fn has_external_agents(&self) -> bool {
        !self.external_agents.is_empty()
    }

    pub fn external_agents(&self) -> impl Iterator<Item = &AgentId> {
        self.external_agents.keys()
    }

    async fn handle_get_agent_server_command(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetAgentServerCommand>,
        mut cx: AsyncApp,
    ) -> Result<proto::AgentServerCommand> {
        let command = this
            .update(&mut cx, |this, cx| {
                let AgentServerStoreState::Local {
                    downstream_client, ..
                } = &this.state
                else {
                    debug_panic!("should not receive GetAgentServerCommand in a non-local project");
                    bail!("unexpected GetAgentServerCommand request in a non-local project");
                };
                let no_browser = this.no_browser();
                let agent = this
                    .external_agents
                    .get_mut(&*envelope.payload.name)
                    .map(|entry| entry.server.as_mut())
                    .with_context(|| format!("agent `{}` not found", envelope.payload.name))?;
                let new_version_available_tx =
                    downstream_client
                        .clone()
                        .map(|(project_id, downstream_client)| {
                            let (new_version_available_tx, mut new_version_available_rx) =
                                watch::channel(None);
                            cx.spawn({
                                let name = envelope.payload.name.clone();
                                async move |_, _| {
                                    if let Some(version) =
                                        new_version_available_rx.recv().await.ok().flatten()
                                    {
                                        downstream_client.send(
                                            proto::NewExternalAgentVersionAvailable {
                                                project_id,
                                                name: name.clone(),
                                                version,
                                            },
                                        )?;
                                    }
                                    anyhow::Ok(())
                                }
                            })
                            .detach_and_log_err(cx);
                            new_version_available_tx
                        });
                let mut extra_env = HashMap::default();
                if no_browser {
                    extra_env.insert("NO_BROWSER".to_owned(), "1".to_owned());
                }
                anyhow::Ok(agent.get_command(
                    extra_env,
                    new_version_available_tx,
                    &mut cx.to_async(),
                ))
            })?
            .await?;
        Ok(proto::AgentServerCommand {
            path: command.path.to_string_lossy().into_owned(),
            args: command.args,
            env: command
                .env
                .map(|env| env.into_iter().collect())
                .unwrap_or_default(),
            root_dir: envelope
                .payload
                .root_dir
                .unwrap_or_else(|| paths::home_dir().to_string_lossy().to_string()),
            login: None,
        })
    }

    async fn handle_external_agents_updated(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExternalAgentsUpdated>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let AgentServerStoreState::Remote {
                project_id,
                upstream_client,
                worktree_store,
            } = &this.state
            else {
                debug_panic!(
                    "handle_external_agents_updated should not be called for a non-remote project"
                );
                bail!("unexpected ExternalAgentsUpdated message")
            };

            let mut previous_entries = std::mem::take(&mut this.external_agents);
            let mut new_version_available_txs = HashMap::default();
            let mut metadata = HashMap::default();

            for (name, mut entry) in previous_entries.drain() {
                if let Some(agent) = entry.server.downcast_mut::<RemoteExternalAgentServer>() {
                    new_version_available_txs
                        .insert(name.clone(), agent.new_version_available_tx.take());
                }

                metadata.insert(name, (entry.icon, entry.display_name, entry.source));
            }

            this.external_agents = envelope
                .payload
                .names
                .into_iter()
                .map(|name| {
                    let agent_id = AgentId(name.into());
                    let (icon, display_name, source) = metadata
                        .remove(&agent_id)
                        .or_else(|| {
                            AgentRegistryStore::try_global(cx)
                                .and_then(|store| store.read(cx).agent(&agent_id))
                                .map(|s| {
                                    (
                                        s.icon_path().cloned(),
                                        Some(s.name().clone()),
                                        ExternalAgentSource::Registry,
                                    )
                                })
                        })
                        .unwrap_or((None, None, ExternalAgentSource::default()));
                    let agent = RemoteExternalAgentServer {
                        project_id: *project_id,
                        upstream_client: upstream_client.clone(),
                        worktree_store: worktree_store.clone(),
                        name: agent_id.clone(),
                        new_version_available_tx: new_version_available_txs
                            .remove(&agent_id)
                            .flatten(),
                    };
                    (
                        agent_id,
                        ExternalAgentEntry::new(
                            Box::new(agent) as Box<dyn ExternalAgentServer>,
                            source,
                            icon,
                            display_name,
                        ),
                    )
                })
                .collect();
            cx.emit(AgentServersUpdated);
            Ok(())
        })
    }

    async fn handle_external_extension_agents_updated(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExternalExtensionAgentsUpdated>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let AgentServerStoreState::Local {
                extension_agents, ..
            } = &mut this.state
            else {
                panic!(
                    "handle_external_extension_agents_updated \
                    should not be called for a non-remote project"
                );
            };

            for ExternalExtensionAgent {
                name,
                icon_path,
                extension_id,
                targets,
                env,
            } in envelope.payload.agents
            {
                extension_agents.push((
                    Arc::from(&*name),
                    extension_id,
                    targets
                        .into_iter()
                        .map(|(k, v)| (k, extension::TargetConfig::from_proto(v)))
                        .collect(),
                    env.into_iter().collect(),
                    icon_path,
                    None,
                ));
            }

            this.reregister_agents(cx);
            cx.emit(AgentServersUpdated);
            Ok(())
        })
    }

    async fn handle_new_version_available(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::NewExternalAgentVersionAvailable>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            if let Some(agent) = this.external_agents.get_mut(&*envelope.payload.name)
                && let Some(agent) = agent.server.downcast_mut::<RemoteExternalAgentServer>()
                && let Some(new_version_available_tx) = &mut agent.new_version_available_tx
            {
                new_version_available_tx
                    .send(Some(envelope.payload.version))
                    .ok();
            }
        });
        Ok(())
    }

    pub fn get_extension_id_for_agent(&mut self, name: &AgentId) -> Option<Arc<str>> {
        self.external_agents.get_mut(name).and_then(|entry| {
            entry
                .server
                .as_any_mut()
                .downcast_ref::<LocalExtensionArchiveAgent>()
                .map(|ext_agent| ext_agent.extension_id.clone())
        })
    }
}

struct RemoteExternalAgentServer {
    project_id: u64,
    upstream_client: Entity<RemoteClient>,
    worktree_store: Entity<WorktreeStore>,
    name: AgentId,
    new_version_available_tx: Option<watch::Sender<Option<String>>>,
}

impl ExternalAgentServer for RemoteExternalAgentServer {
    fn get_command(
        &mut self,
        extra_env: HashMap<String, String>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<AgentServerCommand>> {
        let project_id = self.project_id;
        let name = self.name.to_string();
        let upstream_client = self.upstream_client.downgrade();
        let worktree_store = self.worktree_store.clone();
        self.new_version_available_tx = new_version_available_tx;
        cx.spawn(async move |cx| {
            let root_dir = worktree_store.read_with(cx, |worktree_store, cx| {
                crate::Project::default_visible_worktree_paths(worktree_store, cx)
                    .into_iter()
                    .next()
                    .map(|path| path.display().to_string())
            });

            let mut response = upstream_client
                .update(cx, |upstream_client, _| {
                    upstream_client
                        .proto_client()
                        .request(proto::GetAgentServerCommand {
                            project_id,
                            name,
                            root_dir,
                        })
                })?
                .await?;
            let root_dir = response.root_dir;
            response.env.extend(extra_env);
            let command = upstream_client.update(cx, |client, _| {
                client.build_command_with_options(
                    Some(response.path),
                    &response.args,
                    &response.env.into_iter().collect(),
                    Some(root_dir.clone()),
                    None,
                    Interactive::No,
                )
            })??;
            Ok(AgentServerCommand {
                path: command.program.into(),
                args: command.args,
                env: Some(command.env),
            })
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn asset_kind_for_archive_url(archive_url: &str) -> Result<AssetKind> {
    let archive_path = Url::parse(archive_url)
        .ok()
        .map(|url| url.path().to_string())
        .unwrap_or_else(|| archive_url.to_string());

    if archive_path.ends_with(".zip") {
        Ok(AssetKind::Zip)
    } else if archive_path.ends_with(".tar.gz") || archive_path.ends_with(".tgz") {
        Ok(AssetKind::TarGz)
    } else if archive_path.ends_with(".tar.bz2") || archive_path.ends_with(".tbz2") {
        Ok(AssetKind::TarBz2)
    } else {
        bail!("unsupported archive type in URL: {archive_url}");
    }
}

struct GithubReleaseArchive {
    repo_name_with_owner: String,
    tag: String,
    asset_name: String,
}

fn github_release_archive_from_url(archive_url: &str) -> Option<GithubReleaseArchive> {
    fn decode_path_segment(segment: &str) -> Option<String> {
        percent_decode_str(segment)
            .decode_utf8()
            .ok()
            .map(|segment| segment.into_owned())
    }

    let url = Url::parse(archive_url).ok()?;
    if url.scheme() != "https" || url.host_str()? != "github.com" {
        return None;
    }

    let segments = url.path_segments()?.collect::<Vec<_>>();
    if segments.len() < 6 || segments[2] != "releases" || segments[3] != "download" {
        return None;
    }

    Some(GithubReleaseArchive {
        repo_name_with_owner: format!("{}/{}", segments[0], segments[1]),
        tag: decode_path_segment(segments[4])?,
        asset_name: segments[5..]
            .iter()
            .map(|segment| decode_path_segment(segment))
            .collect::<Option<Vec<_>>>()?
            .join("/"),
    })
}

pub struct LocalExtensionArchiveAgent {
    pub fs: Arc<dyn Fs>,
    pub http_client: Arc<dyn HttpClient>,
    pub node_runtime: NodeRuntime,
    pub project_environment: Entity<ProjectEnvironment>,
    pub extension_id: Arc<str>,
    pub agent_id: Arc<str>,
    pub targets: HashMap<String, extension::TargetConfig>,
    pub env: HashMap<String, String>,
}

impl ExternalAgentServer for LocalExtensionArchiveAgent {
    fn get_command(
        &mut self,
        extra_env: HashMap<String, String>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<AgentServerCommand>> {
        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let extension_id = self.extension_id.clone();
        let agent_id = self.agent_id.clone();
        let targets = self.targets.clone();
        let base_env = self.env.clone();

        cx.spawn(async move |cx| {
            // Get project environment
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        paths::home_dir().as_path().into(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();

            // Merge manifest env and extra env
            env.extend(base_env);
            env.extend(extra_env);

            let cache_key = format!("{}/{}", extension_id, agent_id);
            let dir = paths::external_agents_dir().join(&cache_key);
            fs.create_dir(&dir).await?;

            // Determine platform key
            let os = if cfg!(target_os = "macos") {
                "darwin"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else if cfg!(target_os = "windows") {
                "windows"
            } else {
                anyhow::bail!("unsupported OS");
            };

            let arch = if cfg!(target_arch = "aarch64") {
                "aarch64"
            } else if cfg!(target_arch = "x86_64") {
                "x86_64"
            } else {
                anyhow::bail!("unsupported architecture");
            };

            let platform_key = format!("{}-{}", os, arch);
            let target_config = targets.get(&platform_key).with_context(|| {
                format!(
                    "no target specified for platform '{}'. Available platforms: {}",
                    platform_key,
                    targets
                        .keys()
                        .map(|k| k.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })?;

            let archive_url = &target_config.archive;

            // Use URL as version identifier for caching
            // Hash the URL to get a stable directory name
            let mut hasher = Sha256::new();
            hasher.update(archive_url.as_bytes());
            let url_hash = format!("{:x}", hasher.finalize());
            let version_dir = dir.join(format!("v_{}", url_hash));

            if !fs.is_dir(&version_dir).await {
                // Determine SHA256 for verification
                let sha256 = if let Some(provided_sha) = &target_config.sha256 {
                    // Use provided SHA256
                    Some(provided_sha.clone())
                } else if let Some(github_archive) = github_release_archive_from_url(archive_url) {
                    // Try to fetch SHA256 from GitHub API
                    if let Ok(release) = ::http_client::github::get_release_by_tag_name(
                        &github_archive.repo_name_with_owner,
                        &github_archive.tag,
                        http_client.clone(),
                    )
                    .await
                    {
                        // Find matching asset
                        if let Some(asset) = release
                            .assets
                            .iter()
                            .find(|a| a.name == github_archive.asset_name)
                        {
                            // Strip "sha256:" prefix if present
                            asset.digest.as_ref().map(|d| {
                                d.strip_prefix("sha256:")
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| d.clone())
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let asset_kind = asset_kind_for_archive_url(archive_url)?;

                // Download and extract
                ::http_client::github_download::download_server_binary(
                    &*http_client,
                    archive_url,
                    sha256.as_deref(),
                    &version_dir,
                    asset_kind,
                )
                .await?;
            }

            // Validate and resolve cmd path
            let cmd = &target_config.cmd;

            let cmd_path = if cmd == "node" {
                // Use Zed's managed Node.js runtime
                node_runtime.binary_path().await?
            } else {
                if cmd.contains("..") {
                    anyhow::bail!("command path cannot contain '..': {}", cmd);
                }

                if cmd.starts_with("./") || cmd.starts_with(".\\") {
                    // Relative to extraction directory
                    let cmd_path = version_dir.join(&cmd[2..]);
                    anyhow::ensure!(
                        fs.is_file(&cmd_path).await,
                        "Missing command {} after extraction",
                        cmd_path.to_string_lossy()
                    );
                    cmd_path
                } else {
                    // On PATH
                    anyhow::bail!("command must be relative (start with './'): {}", cmd);
                }
            };

            let command = AgentServerCommand {
                path: cmd_path,
                args: target_config.args.clone(),
                env: Some(env),
            };

            Ok(command)
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalRegistryArchiveAgent {
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    registry_id: Arc<str>,
    targets: HashMap<String, RegistryTargetConfig>,
    env: HashMap<String, String>,
}

impl ExternalAgentServer for LocalRegistryArchiveAgent {
    fn get_command(
        &mut self,
        extra_env: HashMap<String, String>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<AgentServerCommand>> {
        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let registry_id = self.registry_id.clone();
        let targets = self.targets.clone();
        let settings_env = self.env.clone();

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        paths::home_dir().as_path().into(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();

            let dir = paths::external_agents_dir()
                .join("registry")
                .join(registry_id.as_ref());
            fs.create_dir(&dir).await?;

            let os = if cfg!(target_os = "macos") {
                "darwin"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else if cfg!(target_os = "windows") {
                "windows"
            } else {
                anyhow::bail!("unsupported OS");
            };

            let arch = if cfg!(target_arch = "aarch64") {
                "aarch64"
            } else if cfg!(target_arch = "x86_64") {
                "x86_64"
            } else {
                anyhow::bail!("unsupported architecture");
            };

            let platform_key = format!("{}-{}", os, arch);
            let target_config = targets.get(&platform_key).with_context(|| {
                format!(
                    "no target specified for platform '{}'. Available platforms: {}",
                    platform_key,
                    targets
                        .keys()
                        .map(|k| k.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })?;

            env.extend(target_config.env.clone());
            env.extend(extra_env);
            env.extend(settings_env);

            let archive_url = &target_config.archive;

            let mut hasher = Sha256::new();
            hasher.update(archive_url.as_bytes());
            let url_hash = format!("{:x}", hasher.finalize());
            let version_dir = dir.join(format!("v_{}", url_hash));

            if !fs.is_dir(&version_dir).await {
                let sha256 = if let Some(provided_sha) = &target_config.sha256 {
                    Some(provided_sha.clone())
                } else if let Some(github_archive) = github_release_archive_from_url(archive_url) {
                    if let Ok(release) = ::http_client::github::get_release_by_tag_name(
                        &github_archive.repo_name_with_owner,
                        &github_archive.tag,
                        http_client.clone(),
                    )
                    .await
                    {
                        if let Some(asset) = release
                            .assets
                            .iter()
                            .find(|a| a.name == github_archive.asset_name)
                        {
                            asset.digest.as_ref().and_then(|d| {
                                d.strip_prefix("sha256:")
                                    .map(|s| s.to_string())
                                    .or_else(|| Some(d.clone()))
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let asset_kind = asset_kind_for_archive_url(archive_url)?;

                ::http_client::github_download::download_server_binary(
                    &*http_client,
                    archive_url,
                    sha256.as_deref(),
                    &version_dir,
                    asset_kind,
                )
                .await?;
            }

            let cmd = &target_config.cmd;

            let cmd_path = if cmd == "node" {
                node_runtime.binary_path().await?
            } else {
                if cmd.contains("..") {
                    anyhow::bail!("command path cannot contain '..': {}", cmd);
                }

                if cmd.starts_with("./") || cmd.starts_with(".\\") {
                    let cmd_path = version_dir.join(&cmd[2..]);
                    anyhow::ensure!(
                        fs.is_file(&cmd_path).await,
                        "Missing command {} after extraction",
                        cmd_path.to_string_lossy()
                    );
                    cmd_path
                } else {
                    anyhow::bail!("command must be relative (start with './'): {}", cmd);
                }
            };

            let command = AgentServerCommand {
                path: cmd_path,
                args: target_config.args.clone(),
                env: Some(env),
            };

            Ok(command)
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalRegistryNpxAgent {
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    package: SharedString,
    args: Vec<String>,
    distribution_env: HashMap<String, String>,
    settings_env: HashMap<String, String>,
}

impl ExternalAgentServer for LocalRegistryNpxAgent {
    fn get_command(
        &mut self,
        extra_env: HashMap<String, String>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<AgentServerCommand>> {
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let package = self.package.clone();
        let args = self.args.clone();
        let distribution_env = self.distribution_env.clone();
        let settings_env = self.settings_env.clone();

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        paths::home_dir().as_path().into(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();

            let mut exec_args = vec!["--yes".to_string(), "--".to_string(), package.to_string()];
            exec_args.extend(args);

            let npm_command = node_runtime
                .npm_command(
                    "exec",
                    &exec_args.iter().map(|a| a.as_str()).collect::<Vec<_>>(),
                )
                .await?;

            env.extend(npm_command.env);
            env.extend(distribution_env);
            env.extend(extra_env);
            env.extend(settings_env);

            let command = AgentServerCommand {
                path: npm_command.path,
                args: npm_command.args,
                env: Some(env),
            };

            Ok(command)
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalCustomAgent {
    project_environment: Entity<ProjectEnvironment>,
    command: AgentServerCommand,
}

impl ExternalAgentServer for LocalCustomAgent {
    fn get_command(
        &mut self,
        extra_env: HashMap<String, String>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<AgentServerCommand>> {
        let mut command = self.command.clone();
        let project_environment = self.project_environment.downgrade();
        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        paths::home_dir().as_path().into(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();
            env.extend(command.env.unwrap_or_default());
            env.extend(extra_env);
            command.env = Some(env);
            Ok(command)
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Default, Clone, JsonSchema, Debug, PartialEq, RegisterSetting)]
pub struct AllAgentServersSettings(pub HashMap<String, CustomAgentServerSettings>);

impl std::ops::Deref for AllAgentServersSettings {
    type Target = HashMap<String, CustomAgentServerSettings>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AllAgentServersSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AllAgentServersSettings {
    pub fn has_registry_agents(&self) -> bool {
        self.values()
            .any(|s| matches!(s, CustomAgentServerSettings::Registry { .. }))
    }
}

#[derive(Clone, JsonSchema, Debug, PartialEq)]
pub enum CustomAgentServerSettings {
    Custom {
        command: AgentServerCommand,
        /// The default mode to use for this agent.
        ///
        /// Note: Not only all agents support modes.
        ///
        /// Default: None
        default_mode: Option<String>,
        /// The default model to use for this agent.
        ///
        /// This should be the model ID as reported by the agent.
        ///
        /// Default: None
        default_model: Option<String>,
        /// The favorite models for this agent.
        ///
        /// Default: []
        favorite_models: Vec<String>,
        /// Default values for session config options.
        ///
        /// This is a map from config option ID to value ID.
        ///
        /// Default: {}
        default_config_options: HashMap<String, String>,
        /// Favorited values for session config options.
        ///
        /// This is a map from config option ID to a list of favorited value IDs.
        ///
        /// Default: {}
        favorite_config_option_values: HashMap<String, Vec<String>>,
    },
    Extension {
        /// Additional environment variables to pass to the agent.
        ///
        /// Default: {}
        env: HashMap<String, String>,
        /// The default mode to use for this agent.
        ///
        /// Note: Not only all agents support modes.
        ///
        /// Default: None
        default_mode: Option<String>,
        /// The default model to use for this agent.
        ///
        /// This should be the model ID as reported by the agent.
        ///
        /// Default: None
        default_model: Option<String>,
        /// The favorite models for this agent.
        ///
        /// Default: []
        favorite_models: Vec<String>,
        /// Default values for session config options.
        ///
        /// This is a map from config option ID to value ID.
        ///
        /// Default: {}
        default_config_options: HashMap<String, String>,
        /// Favorited values for session config options.
        ///
        /// This is a map from config option ID to a list of favorited value IDs.
        ///
        /// Default: {}
        favorite_config_option_values: HashMap<String, Vec<String>>,
    },
    Registry {
        /// Additional environment variables to pass to the agent.
        ///
        /// Default: {}
        env: HashMap<String, String>,
        /// The default mode to use for this agent.
        ///
        /// Note: Not only all agents support modes.
        ///
        /// Default: None
        default_mode: Option<String>,
        /// The default model to use for this agent.
        ///
        /// This should be the model ID as reported by the agent.
        ///
        /// Default: None
        default_model: Option<String>,
        /// The favorite models for this agent.
        ///
        /// Default: []
        favorite_models: Vec<String>,
        /// Default values for session config options.
        ///
        /// This is a map from config option ID to value ID.
        ///
        /// Default: {}
        default_config_options: HashMap<String, String>,
        /// Favorited values for session config options.
        ///
        /// This is a map from config option ID to a list of favorited value IDs.
        ///
        /// Default: {}
        favorite_config_option_values: HashMap<String, Vec<String>>,
    },
}

impl CustomAgentServerSettings {
    pub fn command(&self) -> Option<&AgentServerCommand> {
        match self {
            CustomAgentServerSettings::Custom { command, .. } => Some(command),
            CustomAgentServerSettings::Extension { .. }
            | CustomAgentServerSettings::Registry { .. } => None,
        }
    }

    pub fn default_mode(&self) -> Option<&str> {
        match self {
            CustomAgentServerSettings::Custom { default_mode, .. }
            | CustomAgentServerSettings::Extension { default_mode, .. }
            | CustomAgentServerSettings::Registry { default_mode, .. } => default_mode.as_deref(),
        }
    }

    pub fn default_model(&self) -> Option<&str> {
        match self {
            CustomAgentServerSettings::Custom { default_model, .. }
            | CustomAgentServerSettings::Extension { default_model, .. }
            | CustomAgentServerSettings::Registry { default_model, .. } => default_model.as_deref(),
        }
    }

    pub fn favorite_models(&self) -> &[String] {
        match self {
            CustomAgentServerSettings::Custom {
                favorite_models, ..
            }
            | CustomAgentServerSettings::Extension {
                favorite_models, ..
            }
            | CustomAgentServerSettings::Registry {
                favorite_models, ..
            } => favorite_models,
        }
    }

    pub fn default_config_option(&self, config_id: &str) -> Option<&str> {
        match self {
            CustomAgentServerSettings::Custom {
                default_config_options,
                ..
            }
            | CustomAgentServerSettings::Extension {
                default_config_options,
                ..
            }
            | CustomAgentServerSettings::Registry {
                default_config_options,
                ..
            } => default_config_options.get(config_id).map(|s| s.as_str()),
        }
    }

    pub fn favorite_config_option_values(&self, config_id: &str) -> Option<&[String]> {
        match self {
            CustomAgentServerSettings::Custom {
                favorite_config_option_values,
                ..
            }
            | CustomAgentServerSettings::Extension {
                favorite_config_option_values,
                ..
            }
            | CustomAgentServerSettings::Registry {
                favorite_config_option_values,
                ..
            } => favorite_config_option_values
                .get(config_id)
                .map(|v| v.as_slice()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_supported_archive_suffixes() {
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.zip"),
            Ok(AssetKind::Zip)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.zip?download=1"),
            Ok(AssetKind::Zip)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tar.gz"),
            Ok(AssetKind::TarGz)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tar.gz?download=1#latest"),
            Ok(AssetKind::TarGz)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tgz"),
            Ok(AssetKind::TarGz)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tgz#download"),
            Ok(AssetKind::TarGz)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tar.bz2"),
            Ok(AssetKind::TarBz2)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tar.bz2?download=1"),
            Ok(AssetKind::TarBz2)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tbz2"),
            Ok(AssetKind::TarBz2)
        ));
        assert!(matches!(
            asset_kind_for_archive_url("https://example.com/agent.tbz2#download"),
            Ok(AssetKind::TarBz2)
        ));
    }

    #[test]
    fn parses_github_release_archive_urls() {
        let github_archive = github_release_archive_from_url(
            "https://github.com/owner/repo/releases/download/release%2F2.3.5/agent.tar.bz2?download=1",
        )
        .unwrap();

        assert_eq!(github_archive.repo_name_with_owner, "owner/repo");
        assert_eq!(github_archive.tag, "release/2.3.5");
        assert_eq!(github_archive.asset_name, "agent.tar.bz2");
    }

    #[test]
    fn rejects_unsupported_archive_suffixes() {
        let error = asset_kind_for_archive_url("https://example.com/agent.tar.xz")
            .err()
            .map(|error| error.to_string());

        assert_eq!(
            error,
            Some("unsupported archive type in URL: https://example.com/agent.tar.xz".to_string())
        );
    }
}

impl From<settings::CustomAgentServerSettings> for CustomAgentServerSettings {
    fn from(value: settings::CustomAgentServerSettings) -> Self {
        match value {
            settings::CustomAgentServerSettings::Custom {
                path,
                args,
                env,
                default_mode,
                default_model,
                favorite_models,
                default_config_options,
                favorite_config_option_values,
            } => CustomAgentServerSettings::Custom {
                command: AgentServerCommand {
                    path: PathBuf::from(shellexpand::tilde(&path.to_string_lossy()).as_ref()),
                    args,
                    env: Some(env),
                },
                default_mode,
                default_model,
                favorite_models,
                default_config_options,
                favorite_config_option_values,
            },
            settings::CustomAgentServerSettings::Extension {
                env,
                default_mode,
                default_model,
                default_config_options,
                favorite_models,
                favorite_config_option_values,
            } => CustomAgentServerSettings::Extension {
                env,
                default_mode,
                default_model,
                default_config_options,
                favorite_models,
                favorite_config_option_values,
            },
            settings::CustomAgentServerSettings::Registry {
                env,
                default_mode,
                default_model,
                default_config_options,
                favorite_models,
                favorite_config_option_values,
            } => CustomAgentServerSettings::Registry {
                env,
                default_mode,
                default_model,
                default_config_options,
                favorite_models,
                favorite_config_option_values,
            },
        }
    }
}

impl settings::Settings for AllAgentServersSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let agent_settings = content.agent_servers.clone().unwrap();
        Self(
            agent_settings
                .0
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        )
    }
}
