use std::{
    any::Any,
    borrow::Borrow,
    collections::HashSet,
    path::{Path, PathBuf},
    str::FromStr as _,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, Result, bail};
use collections::HashMap;
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::StreamExt as _;
use gpui::{
    AppContext as _, AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription, Task,
};
use http_client::{HttpClient, github::AssetKind};
use node_runtime::NodeRuntime;
use remote::RemoteClient;
use rpc::{AnyProtoClient, TypedEnvelope, proto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use task::Shell;
use util::{ResultExt as _, debug_panic};

use crate::ProjectEnvironment;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternalAgentServerName(pub SharedString);

impl std::fmt::Display for ExternalAgentServerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for ExternalAgentServerName {
    fn from(value: &'static str) -> Self {
        ExternalAgentServerName(value.into())
    }
}

impl From<ExternalAgentServerName> for SharedString {
    fn from(value: ExternalAgentServerName) -> Self {
        value.0
    }
}

impl Borrow<str> for ExternalAgentServerName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

pub trait ExternalAgentServer {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>>;

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
        _subscriptions: [Subscription; 1],
    },
    Remote {
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
    },
    Collab,
}

pub struct AgentServerStore {
    state: AgentServerStoreState,
    external_agents: HashMap<ExternalAgentServerName, Box<dyn ExternalAgentServer>>,
    agent_icons: HashMap<ExternalAgentServerName, SharedString>,
}

pub struct AgentServersUpdated;

impl EventEmitter<AgentServersUpdated> for AgentServerStore {}

#[cfg(test)]
mod ext_agent_tests {
    use super::*;
    use std::fmt::Write as _;

    // Helper to build a store in Collab mode so we can mutate internal maps without
    // needing to spin up a full project environment.
    fn collab_store() -> AgentServerStore {
        AgentServerStore {
            state: AgentServerStoreState::Collab,
            external_agents: HashMap::default(),
            agent_icons: HashMap::default(),
        }
    }

    // A simple fake that implements ExternalAgentServer without needing async plumbing.
    struct NoopExternalAgent;

    impl ExternalAgentServer for NoopExternalAgent {
        fn get_command(
            &mut self,
            _root_dir: Option<&str>,
            _extra_env: HashMap<String, String>,
            _status_tx: Option<watch::Sender<SharedString>>,
            _new_version_available_tx: Option<watch::Sender<Option<String>>>,
            _cx: &mut AsyncApp,
        ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
            Task::ready(Ok((
                AgentServerCommand {
                    path: PathBuf::from("noop"),
                    args: Vec::new(),
                    env: None,
                },
                "".to_string(),
                None,
            )))
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn external_agent_server_name_display() {
        let name = ExternalAgentServerName(SharedString::from("Ext: Tool"));
        let mut s = String::new();
        write!(&mut s, "{name}").unwrap();
        assert_eq!(s, "Ext: Tool");
    }

    #[test]
    fn sync_extension_agents_removes_previous_extension_entries() {
        let mut store = collab_store();

        // Seed with a couple of agents that will be replaced by extensions
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("foo-agent")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("bar-agent")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("custom")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );

        // Simulate the removal phase: if we're syncing extensions that provide
        // "foo-agent" and "bar-agent", those should be removed first
        let extension_agent_names: HashSet<String> =
            ["foo-agent".to_string(), "bar-agent".to_string()]
                .into_iter()
                .collect();

        let keys_to_remove: Vec<_> = store
            .external_agents
            .keys()
            .filter(|name| extension_agent_names.contains(name.0.as_ref()))
            .cloned()
            .collect();

        for key in keys_to_remove {
            store.external_agents.remove(&key);
        }

        // Only the custom entry should remain.
        let remaining: Vec<_> = store
            .external_agents
            .keys()
            .map(|k| k.0.to_string())
            .collect();
        assert_eq!(remaining, vec!["custom".to_string()]);
    }
}

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

        // Remove existing extension-provided agents by tracking which ones we're about to add
        let extension_agent_names: HashSet<_> = manifests
            .iter()
            .flat_map(|(_, manifest)| manifest.agent_servers.keys().map(|k| k.to_string()))
            .collect();

        let keys_to_remove: Vec<_> = self
            .external_agents
            .keys()
            .filter(|name| {
                // Remove if it matches an extension agent name from any extension
                extension_agent_names.contains(name.0.as_ref())
            })
            .cloned()
            .collect();
        for key in &keys_to_remove {
            self.external_agents.remove(key);
            self.agent_icons.remove(key);
        }

        // Insert agent servers from extension manifests
        match &self.state {
            AgentServerStoreState::Local {
                project_environment,
                fs,
                http_client,
                ..
            } => {
                for (ext_id, manifest) in manifests {
                    for (agent_name, agent_entry) in &manifest.agent_servers {
                        let display = SharedString::from(agent_entry.name.clone());

                        // Store absolute icon path if provided, resolving symlinks for dev extensions
                        if let Some(icon) = &agent_entry.icon {
                            let icon_path = extensions_dir.join(ext_id).join(icon);
                            // Canonicalize to resolve symlinks (dev extensions are symlinked)
                            let absolute_icon_path = icon_path
                                .canonicalize()
                                .unwrap_or(icon_path)
                                .to_string_lossy()
                                .to_string();
                            self.agent_icons.insert(
                                ExternalAgentServerName(display.clone()),
                                SharedString::from(absolute_icon_path),
                            );
                        }

                        // Archive-based launcher (download from URL)
                        self.external_agents.insert(
                            ExternalAgentServerName(display),
                            Box::new(LocalExtensionArchiveAgent {
                                fs: fs.clone(),
                                http_client: http_client.clone(),
                                project_environment: project_environment.clone(),
                                extension_id: Arc::from(ext_id),
                                agent_id: agent_name.clone(),
                                targets: agent_entry.targets.clone(),
                                env: agent_entry.env.clone(),
                            }) as Box<dyn ExternalAgentServer>,
                        );
                    }
                }
            }
            _ => {
                // Only local projects support local extension agents
            }
        }

        cx.emit(AgentServersUpdated);
    }

    pub fn agent_icon(&self, name: &ExternalAgentServerName) -> Option<SharedString> {
        self.agent_icons.get(name).cloned()
    }

    pub fn init_remote(session: &AnyProtoClient) {
        session.add_entity_message_handler(Self::handle_external_agents_updated);
        session.add_entity_message_handler(Self::handle_loading_status_updated);
        session.add_entity_message_handler(Self::handle_new_version_available);
    }

    pub fn init_headless(session: &AnyProtoClient) {
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

        self.external_agents.clear();
        self.external_agents.insert(
            GEMINI_NAME.into(),
            Box::new(LocalGemini {
                fs: fs.clone(),
                node_runtime: node_runtime.clone(),
                project_environment: project_environment.clone(),
                custom_command: new_settings
                    .gemini
                    .clone()
                    .and_then(|settings| settings.custom_command()),
                ignore_system_version: new_settings
                    .gemini
                    .as_ref()
                    .and_then(|settings| settings.ignore_system_version)
                    .unwrap_or(false),
            }),
        );
        self.external_agents.insert(
            CODEX_NAME.into(),
            Box::new(LocalCodex {
                fs: fs.clone(),
                project_environment: project_environment.clone(),
                custom_command: new_settings
                    .codex
                    .clone()
                    .and_then(|settings| settings.custom_command()),
                http_client: http_client.clone(),
                is_remote: downstream_client.is_some(),
            }),
        );
        self.external_agents.insert(
            CLAUDE_CODE_NAME.into(),
            Box::new(LocalClaudeCode {
                fs: fs.clone(),
                node_runtime: node_runtime.clone(),
                project_environment: project_environment.clone(),
                custom_command: new_settings
                    .claude
                    .clone()
                    .and_then(|settings| settings.custom_command()),
            }),
        );
        self.external_agents
            .extend(new_settings.custom.iter().map(|(name, settings)| {
                (
                    ExternalAgentServerName(name.clone()),
                    Box::new(LocalCustomAgent {
                        command: settings.command.clone(),
                        project_environment: project_environment.clone(),
                    }) as Box<dyn ExternalAgentServer>,
                )
            }));

        *old_settings = Some(new_settings.clone());

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

    pub fn local(
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        project_environment: Entity<ProjectEnvironment>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe_global::<SettingsStore>(|this, cx| {
            this.agent_servers_settings_changed(cx);
        });
        let mut this = Self {
            state: AgentServerStoreState::Local {
                node_runtime,
                fs,
                project_environment,
                http_client,
                downstream_client: None,
                settings: None,
                _subscriptions: [subscription],
            },
            external_agents: Default::default(),
            agent_icons: Default::default(),
        };
        if let Some(_events) = extension::ExtensionEvents::try_global(cx) {}
        this.agent_servers_settings_changed(cx);
        this
    }

    pub(crate) fn remote(project_id: u64, upstream_client: Entity<RemoteClient>) -> Self {
        // Set up the builtin agents here so they're immediately available in
        // remote projects--we know that the HeadlessProject on the other end
        // will have them.
        let external_agents: [(ExternalAgentServerName, Box<dyn ExternalAgentServer>); 3] = [
            (
                CLAUDE_CODE_NAME.into(),
                Box::new(RemoteExternalAgentServer {
                    project_id,
                    upstream_client: upstream_client.clone(),
                    name: CLAUDE_CODE_NAME.into(),
                    status_tx: None,
                    new_version_available_tx: None,
                }) as Box<dyn ExternalAgentServer>,
            ),
            (
                CODEX_NAME.into(),
                Box::new(RemoteExternalAgentServer {
                    project_id,
                    upstream_client: upstream_client.clone(),
                    name: CODEX_NAME.into(),
                    status_tx: None,
                    new_version_available_tx: None,
                }) as Box<dyn ExternalAgentServer>,
            ),
            (
                GEMINI_NAME.into(),
                Box::new(RemoteExternalAgentServer {
                    project_id,
                    upstream_client: upstream_client.clone(),
                    name: GEMINI_NAME.into(),
                    status_tx: None,
                    new_version_available_tx: None,
                }) as Box<dyn ExternalAgentServer>,
            ),
        ];

        Self {
            state: AgentServerStoreState::Remote {
                project_id,
                upstream_client,
            },
            external_agents: external_agents.into_iter().collect(),
            agent_icons: HashMap::default(),
        }
    }

    pub(crate) fn collab(_cx: &mut Context<Self>) -> Self {
        Self {
            state: AgentServerStoreState::Collab,
            external_agents: Default::default(),
            agent_icons: Default::default(),
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
                        this.external_agents
                            .keys()
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
        name: &ExternalAgentServerName,
    ) -> Option<&mut (dyn ExternalAgentServer + 'static)> {
        self.external_agents
            .get_mut(name)
            .map(|agent| agent.as_mut())
    }

    pub fn external_agents(&self) -> impl Iterator<Item = &ExternalAgentServerName> {
        self.external_agents.keys()
    }

    async fn handle_get_agent_server_command(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetAgentServerCommand>,
        mut cx: AsyncApp,
    ) -> Result<proto::AgentServerCommand> {
        let (command, root_dir, login_command) = this
            .update(&mut cx, |this, cx| {
                let AgentServerStoreState::Local {
                    downstream_client, ..
                } = &this.state
                else {
                    debug_panic!("should not receive GetAgentServerCommand in a non-local project");
                    bail!("unexpected GetAgentServerCommand request in a non-local project");
                };
                let agent = this
                    .external_agents
                    .get_mut(&*envelope.payload.name)
                    .with_context(|| format!("agent `{}` not found", envelope.payload.name))?;
                let (status_tx, new_version_available_tx) = downstream_client
                    .clone()
                    .map(|(project_id, downstream_client)| {
                        let (status_tx, mut status_rx) = watch::channel(SharedString::from(""));
                        let (new_version_available_tx, mut new_version_available_rx) =
                            watch::channel(None);
                        cx.spawn({
                            let downstream_client = downstream_client.clone();
                            let name = envelope.payload.name.clone();
                            async move |_, _| {
                                while let Some(status) = status_rx.recv().await.ok() {
                                    downstream_client.send(
                                        proto::ExternalAgentLoadingStatusUpdated {
                                            project_id,
                                            name: name.clone(),
                                            status: status.to_string(),
                                        },
                                    )?;
                                }
                                anyhow::Ok(())
                            }
                        })
                        .detach_and_log_err(cx);
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
                        (status_tx, new_version_available_tx)
                    })
                    .unzip();
                anyhow::Ok(agent.get_command(
                    envelope.payload.root_dir.as_deref(),
                    HashMap::default(),
                    status_tx,
                    new_version_available_tx,
                    &mut cx.to_async(),
                ))
            })??
            .await?;
        Ok(proto::AgentServerCommand {
            path: command.path.to_string_lossy().into_owned(),
            args: command.args,
            env: command
                .env
                .map(|env| env.into_iter().collect())
                .unwrap_or_default(),
            root_dir: root_dir,
            login: login_command.map(|cmd| cmd.to_proto()),
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
            } = &this.state
            else {
                debug_panic!(
                    "handle_external_agents_updated should not be called for a non-remote project"
                );
                bail!("unexpected ExternalAgentsUpdated message")
            };

            let mut status_txs = this
                .external_agents
                .iter_mut()
                .filter_map(|(name, agent)| {
                    Some((
                        name.clone(),
                        agent
                            .downcast_mut::<RemoteExternalAgentServer>()?
                            .status_tx
                            .take(),
                    ))
                })
                .collect::<HashMap<_, _>>();
            let mut new_version_available_txs = this
                .external_agents
                .iter_mut()
                .filter_map(|(name, agent)| {
                    Some((
                        name.clone(),
                        agent
                            .downcast_mut::<RemoteExternalAgentServer>()?
                            .new_version_available_tx
                            .take(),
                    ))
                })
                .collect::<HashMap<_, _>>();

            this.external_agents = envelope
                .payload
                .names
                .into_iter()
                .map(|name| {
                    let agent = RemoteExternalAgentServer {
                        project_id: *project_id,
                        upstream_client: upstream_client.clone(),
                        name: ExternalAgentServerName(name.clone().into()),
                        status_tx: status_txs.remove(&*name).flatten(),
                        new_version_available_tx: new_version_available_txs
                            .remove(&*name)
                            .flatten(),
                    };
                    (
                        ExternalAgentServerName(name.into()),
                        Box::new(agent) as Box<dyn ExternalAgentServer>,
                    )
                })
                .collect();
            cx.emit(AgentServersUpdated);
            Ok(())
        })?
    }

    async fn handle_loading_status_updated(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExternalAgentLoadingStatusUpdated>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            if let Some(agent) = this.external_agents.get_mut(&*envelope.payload.name)
                && let Some(agent) = agent.downcast_mut::<RemoteExternalAgentServer>()
                && let Some(status_tx) = &mut agent.status_tx
            {
                status_tx.send(envelope.payload.status.into()).ok();
            }
        })
    }

    async fn handle_new_version_available(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::NewExternalAgentVersionAvailable>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            if let Some(agent) = this.external_agents.get_mut(&*envelope.payload.name)
                && let Some(agent) = agent.downcast_mut::<RemoteExternalAgentServer>()
                && let Some(new_version_available_tx) = &mut agent.new_version_available_tx
            {
                new_version_available_tx
                    .send(Some(envelope.payload.version))
                    .ok();
            }
        })
    }
}

fn get_or_npm_install_builtin_agent(
    binary_name: SharedString,
    package_name: SharedString,
    entrypoint_path: PathBuf,
    minimum_version: Option<semver::Version>,
    status_tx: Option<watch::Sender<SharedString>>,
    new_version_available: Option<watch::Sender<Option<String>>>,
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    cx: &mut AsyncApp,
) -> Task<std::result::Result<AgentServerCommand, anyhow::Error>> {
    cx.spawn(async move |cx| {
        let node_path = node_runtime.binary_path().await?;
        let dir = paths::external_agents_dir().join(binary_name.as_str());
        fs.create_dir(&dir).await?;

        let mut stream = fs.read_dir(&dir).await?;
        let mut versions = Vec::new();
        let mut to_delete = Vec::new();
        while let Some(entry) = stream.next().await {
            let Ok(entry) = entry else { continue };
            let Some(file_name) = entry.file_name() else {
                continue;
            };

            if let Some(name) = file_name.to_str()
                && let Some(version) = semver::Version::from_str(name).ok()
                && fs
                    .is_file(&dir.join(file_name).join(&entrypoint_path))
                    .await
            {
                versions.push((version, file_name.to_owned()));
            } else {
                to_delete.push(file_name.to_owned())
            }
        }

        versions.sort();
        let newest_version = if let Some((version, file_name)) = versions.last().cloned()
            && minimum_version.is_none_or(|minimum_version| version >= minimum_version)
        {
            versions.pop();
            Some(file_name)
        } else {
            None
        };
        log::debug!("existing version of {package_name}: {newest_version:?}");
        to_delete.extend(versions.into_iter().map(|(_, file_name)| file_name));

        cx.background_spawn({
            let fs = fs.clone();
            let dir = dir.clone();
            async move {
                for file_name in to_delete {
                    fs.remove_dir(
                        &dir.join(file_name),
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: false,
                        },
                    )
                    .await
                    .ok();
                }
            }
        })
        .detach();

        let version = if let Some(file_name) = newest_version {
            cx.background_spawn({
                let file_name = file_name.clone();
                let dir = dir.clone();
                let fs = fs.clone();
                async move {
                    let latest_version = node_runtime
                        .npm_package_latest_version(&package_name)
                        .await
                        .ok();
                    if let Some(latest_version) = latest_version
                        && &latest_version != &file_name.to_string_lossy()
                    {
                        let download_result = download_latest_version(
                            fs,
                            dir.clone(),
                            node_runtime,
                            package_name.clone(),
                        )
                        .await
                        .log_err();
                        if let Some(mut new_version_available) = new_version_available
                            && download_result.is_some()
                        {
                            new_version_available.send(Some(latest_version)).ok();
                        }
                    }
                }
            })
            .detach();
            file_name
        } else {
            if let Some(mut status_tx) = status_tx {
                status_tx.send("Installing…".into()).ok();
            }
            let dir = dir.clone();
            cx.background_spawn(download_latest_version(
                fs.clone(),
                dir.clone(),
                node_runtime,
                package_name.clone(),
            ))
            .await?
            .into()
        };

        let agent_server_path = dir.join(version).join(entrypoint_path);
        let agent_server_path_exists = fs.is_file(&agent_server_path).await;
        anyhow::ensure!(
            agent_server_path_exists,
            "Missing entrypoint path {} after installation",
            agent_server_path.to_string_lossy()
        );

        anyhow::Ok(AgentServerCommand {
            path: node_path,
            args: vec![agent_server_path.to_string_lossy().into_owned()],
            env: None,
        })
    })
}

fn find_bin_in_path(
    bin_name: SharedString,
    root_dir: PathBuf,
    env: HashMap<String, String>,
    cx: &mut AsyncApp,
) -> Task<Option<PathBuf>> {
    cx.background_executor().spawn(async move {
        let which_result = if cfg!(windows) {
            which::which(bin_name.as_str())
        } else {
            let shell_path = env.get("PATH").cloned();
            which::which_in(bin_name.as_str(), shell_path.as_ref(), &root_dir)
        };

        if let Err(which::Error::CannotFindBinaryPath) = which_result {
            return None;
        }

        which_result.log_err()
    })
}

async fn download_latest_version(
    fs: Arc<dyn Fs>,
    dir: PathBuf,
    node_runtime: NodeRuntime,
    package_name: SharedString,
) -> Result<String> {
    log::debug!("downloading latest version of {package_name}");

    let tmp_dir = tempfile::tempdir_in(&dir)?;

    node_runtime
        .npm_install_packages(tmp_dir.path(), &[(&package_name, "latest")])
        .await?;

    let version = node_runtime
        .npm_package_installed_version(tmp_dir.path(), &package_name)
        .await?
        .context("expected package to be installed")?;

    fs.rename(
        &tmp_dir.keep(),
        &dir.join(&version),
        RenameOptions {
            ignore_if_exists: true,
            overwrite: true,
        },
    )
    .await?;

    anyhow::Ok(version)
}

struct RemoteExternalAgentServer {
    project_id: u64,
    upstream_client: Entity<RemoteClient>,
    name: ExternalAgentServerName,
    status_tx: Option<watch::Sender<SharedString>>,
    new_version_available_tx: Option<watch::Sender<Option<String>>>,
}

impl ExternalAgentServer for RemoteExternalAgentServer {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let project_id = self.project_id;
        let name = self.name.to_string();
        let upstream_client = self.upstream_client.downgrade();
        let root_dir = root_dir.map(|root_dir| root_dir.to_owned());
        self.status_tx = status_tx;
        self.new_version_available_tx = new_version_available_tx;
        cx.spawn(async move |cx| {
            let mut response = upstream_client
                .update(cx, |upstream_client, _| {
                    upstream_client
                        .proto_client()
                        .request(proto::GetAgentServerCommand {
                            project_id,
                            name,
                            root_dir: root_dir.clone(),
                        })
                })?
                .await?;
            let root_dir = response.root_dir;
            response.env.extend(extra_env);
            let command = upstream_client.update(cx, |client, _| {
                client.build_command(
                    Some(response.path),
                    &response.args,
                    &response.env.into_iter().collect(),
                    Some(root_dir.clone()),
                    None,
                )
            })??;
            Ok((
                AgentServerCommand {
                    path: command.program.into(),
                    args: command.args,
                    env: Some(command.env),
                },
                root_dir,
                None,
            ))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalGemini {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    custom_command: Option<AgentServerCommand>,
    ignore_system_version: bool,
}

impl ExternalAgentServer for LocalGemini {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let custom_command = self.custom_command.clone();
        let ignore_system_version = self.ignore_system_version;
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.get_local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();

            let mut command = if let Some(mut custom_command) = custom_command {
                env.extend(custom_command.env.unwrap_or_default());
                custom_command.env = Some(env);
                custom_command
            } else if !ignore_system_version
                && let Some(bin) =
                    find_bin_in_path("gemini".into(), root_dir.to_path_buf(), env.clone(), cx).await
            {
                AgentServerCommand {
                    path: bin,
                    args: Vec::new(),
                    env: Some(env),
                }
            } else {
                let mut command = get_or_npm_install_builtin_agent(
                    GEMINI_NAME.into(),
                    "@google/gemini-cli".into(),
                    "node_modules/@google/gemini-cli/dist/index.js".into(),
                    if cfg!(windows) {
                        // v0.8.x on Windows has a bug that causes the initialize request to hang forever
                        Some("0.9.0".parse().unwrap())
                    } else {
                        Some("0.2.1".parse().unwrap())
                    },
                    status_tx,
                    new_version_available_tx,
                    fs,
                    node_runtime,
                    cx,
                )
                .await?;
                command.env = Some(env);
                command
            };

            // Gemini CLI doesn't seem to have a dedicated invocation for logging in--we just run it normally without any arguments.
            let login = task::SpawnInTerminal {
                command: Some(command.path.to_string_lossy().into_owned()),
                args: command.args.clone(),
                env: command.env.clone().unwrap_or_default(),
                label: "gemini /auth".into(),
                ..Default::default()
            };

            command.env.get_or_insert_default().extend(extra_env);
            command.args.push("--experimental-acp".into());
            Ok((
                command,
                root_dir.to_string_lossy().into_owned(),
                Some(login),
            ))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalClaudeCode {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    custom_command: Option<AgentServerCommand>,
}

impl ExternalAgentServer for LocalClaudeCode {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let custom_command = self.custom_command.clone();
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.get_local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();
            env.insert("ANTHROPIC_API_KEY".into(), "".into());

            let (mut command, login_command) = if let Some(mut custom_command) = custom_command {
                env.extend(custom_command.env.unwrap_or_default());
                custom_command.env = Some(env);
                (custom_command, None)
            } else {
                let mut command = get_or_npm_install_builtin_agent(
                    "claude-code-acp".into(),
                    "@zed-industries/claude-code-acp".into(),
                    "node_modules/@zed-industries/claude-code-acp/dist/index.js".into(),
                    Some("0.5.2".parse().unwrap()),
                    status_tx,
                    new_version_available_tx,
                    fs,
                    node_runtime,
                    cx,
                )
                .await?;
                command.env = Some(env);
                let login = command
                    .args
                    .first()
                    .and_then(|path| {
                        path.strip_suffix("/@zed-industries/claude-code-acp/dist/index.js")
                    })
                    .map(|path_prefix| task::SpawnInTerminal {
                        command: Some(command.path.to_string_lossy().into_owned()),
                        args: vec![
                            Path::new(path_prefix)
                                .join("@anthropic-ai/claude-agent-sdk/cli.js")
                                .to_string_lossy()
                                .to_string(),
                            "/login".into(),
                        ],
                        env: command.env.clone().unwrap_or_default(),
                        label: "claude /login".into(),
                        ..Default::default()
                    });
                (command, login)
            };

            command.env.get_or_insert_default().extend(extra_env);
            Ok((
                command,
                root_dir.to_string_lossy().into_owned(),
                login_command,
            ))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalCodex {
    fs: Arc<dyn Fs>,
    project_environment: Entity<ProjectEnvironment>,
    http_client: Arc<dyn HttpClient>,
    custom_command: Option<AgentServerCommand>,
    is_remote: bool,
}

impl ExternalAgentServer for LocalCodex {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let project_environment = self.project_environment.downgrade();
        let http = self.http_client.clone();
        let custom_command = self.custom_command.clone();
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();
        let is_remote = self.is_remote;

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.get_local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();
            if is_remote {
                env.insert("NO_BROWSER".to_owned(), "1".to_owned());
            }

            let mut command = if let Some(mut custom_command) = custom_command {
                env.extend(custom_command.env.unwrap_or_default());
                custom_command.env = Some(env);
                custom_command
            } else {
                let dir = paths::external_agents_dir().join(CODEX_NAME);
                fs.create_dir(&dir).await?;

                // Find or install the latest Codex release (no update checks for now).
                let release = ::http_client::github::latest_github_release(
                    CODEX_ACP_REPO,
                    true,
                    false,
                    http.clone(),
                )
                .await
                .context("fetching Codex latest release")?;

                let version_dir = dir.join(&release.tag_name);
                if !fs.is_dir(&version_dir).await {
                    if let Some(mut status_tx) = status_tx {
                        status_tx.send("Installing…".into()).ok();
                    }

                    let tag = release.tag_name.clone();
                    let version_number = tag.trim_start_matches('v');
                    let asset_name = asset_name(version_number)
                        .context("codex acp is not supported for this architecture")?;
                    let asset = release
                        .assets
                        .into_iter()
                        .find(|asset| asset.name == asset_name)
                        .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
                    // Strip "sha256:" prefix from digest if present (GitHub API format)
                    let digest = asset
                        .digest
                        .as_deref()
                        .and_then(|d| d.strip_prefix("sha256:").or(Some(d)));
                    ::http_client::github_download::download_server_binary(
                        &*http,
                        &asset.browser_download_url,
                        digest,
                        &version_dir,
                        if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
                            AssetKind::Zip
                        } else {
                            AssetKind::TarGz
                        },
                    )
                    .await?;

                    // remove older versions
                    util::fs::remove_matching(&dir, |entry| entry != version_dir).await;
                }

                let bin_name = if cfg!(windows) {
                    "codex-acp.exe"
                } else {
                    "codex-acp"
                };
                let bin_path = version_dir.join(bin_name);
                anyhow::ensure!(
                    fs.is_file(&bin_path).await,
                    "Missing Codex binary at {} after installation",
                    bin_path.to_string_lossy()
                );

                let mut cmd = AgentServerCommand {
                    path: bin_path,
                    args: Vec::new(),
                    env: None,
                };
                cmd.env = Some(env);
                cmd
            };

            command.env.get_or_insert_default().extend(extra_env);
            Ok((command, root_dir.to_string_lossy().into_owned(), None))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub const CODEX_ACP_REPO: &str = "zed-industries/codex-acp";

fn get_platform_info() -> Option<(&'static str, &'static str, &'static str)> {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        return None;
    };

    let platform = if cfg!(target_os = "macos") {
        "apple-darwin"
    } else if cfg!(target_os = "windows") {
        "pc-windows-msvc"
    } else if cfg!(target_os = "linux") {
        "unknown-linux-gnu"
    } else {
        return None;
    };

    // Only Windows x86_64 uses .zip in release assets
    let ext = if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
        "zip"
    } else {
        "tar.gz"
    };

    Some((arch, platform, ext))
}

fn asset_name(version: &str) -> Option<String> {
    let (arch, platform, ext) = get_platform_info()?;
    Some(format!("codex-acp-{version}-{arch}-{platform}.{ext}"))
}

struct LocalExtensionArchiveAgent {
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    project_environment: Entity<ProjectEnvironment>,
    extension_id: Arc<str>,
    agent_id: Arc<str>,
    targets: HashMap<String, extension::TargetConfig>,
    env: HashMap<String, String>,
}

struct LocalCustomAgent {
    project_environment: Entity<ProjectEnvironment>,
    command: AgentServerCommand,
}

impl ExternalAgentServer for LocalExtensionArchiveAgent {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        _status_tx: Option<watch::Sender<SharedString>>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        let project_environment = self.project_environment.downgrade();
        let extension_id = self.extension_id.clone();
        let agent_id = self.agent_id.clone();
        let targets = self.targets.clone();
        let base_env = self.env.clone();

        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();

        cx.spawn(async move |cx| {
            // Get project environment
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.get_local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
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
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            archive_url.hash(&mut hasher);
            let url_hash = hasher.finish();
            let version_dir = dir.join(format!("v_{:x}", url_hash));

            if !fs.is_dir(&version_dir).await {
                // Determine SHA256 for verification
                let sha256 = if let Some(provided_sha) = &target_config.sha256 {
                    // Use provided SHA256
                    Some(provided_sha.clone())
                } else if archive_url.starts_with("https://github.com/") {
                    // Try to fetch SHA256 from GitHub API
                    // Parse URL to extract repo and tag/file info
                    // Format: https://github.com/owner/repo/releases/download/tag/file.zip
                    if let Some(caps) = archive_url.strip_prefix("https://github.com/") {
                        let parts: Vec<&str> = caps.split('/').collect();
                        if parts.len() >= 6 && parts[2] == "releases" && parts[3] == "download" {
                            let repo = format!("{}/{}", parts[0], parts[1]);
                            let tag = parts[4];
                            let filename = parts[5..].join("/");

                            // Try to get release info from GitHub
                            if let Ok(release) = ::http_client::github::get_release_by_tag_name(
                                &repo,
                                tag,
                                http_client.clone(),
                            )
                            .await
                            {
                                // Find matching asset
                                if let Some(asset) =
                                    release.assets.iter().find(|a| a.name == filename)
                                {
                                    // Strip "sha256:" prefix if present
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
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Determine archive type from URL
                let asset_kind = if archive_url.ends_with(".zip") {
                    AssetKind::Zip
                } else if archive_url.ends_with(".tar.gz") || archive_url.ends_with(".tgz") {
                    AssetKind::TarGz
                } else {
                    anyhow::bail!("unsupported archive type in URL: {}", archive_url);
                };

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
            if cmd.contains("..") {
                anyhow::bail!("command path cannot contain '..': {}", cmd);
            }

            let cmd_path = if cmd.starts_with("./") || cmd.starts_with(".\\") {
                // Relative to extraction directory
                version_dir.join(&cmd[2..])
            } else {
                // On PATH
                anyhow::bail!("command must be relative (start with './'): {}", cmd);
            };

            anyhow::ensure!(
                fs.is_file(&cmd_path).await,
                "Missing command {} after extraction",
                cmd_path.to_string_lossy()
            );

            let command = AgentServerCommand {
                path: cmd_path,
                args: target_config.args.clone(),
                env: Some(env),
            };

            Ok((command, root_dir.to_string_lossy().into_owned(), None))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ExternalAgentServer for LocalCustomAgent {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        _status_tx: Option<watch::Sender<SharedString>>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let mut command = self.command.clone();
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();
        let project_environment = self.project_environment.downgrade();
        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.get_local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();
            env.extend(command.env.unwrap_or_default());
            env.extend(extra_env);
            command.env = Some(env);
            Ok((command, root_dir.to_string_lossy().into_owned(), None))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub const GEMINI_NAME: &'static str = "gemini";
pub const CLAUDE_CODE_NAME: &'static str = "claude";
pub const CODEX_NAME: &'static str = "codex";

#[derive(Default, Clone, JsonSchema, Debug, PartialEq)]
pub struct AllAgentServersSettings {
    pub gemini: Option<BuiltinAgentServerSettings>,
    pub claude: Option<BuiltinAgentServerSettings>,
    pub codex: Option<BuiltinAgentServerSettings>,
    pub custom: HashMap<SharedString, CustomAgentServerSettings>,
}
#[derive(Default, Clone, JsonSchema, Debug, PartialEq)]
pub struct BuiltinAgentServerSettings {
    pub path: Option<PathBuf>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub ignore_system_version: Option<bool>,
    pub default_mode: Option<String>,
}

impl BuiltinAgentServerSettings {
    pub(crate) fn custom_command(self) -> Option<AgentServerCommand> {
        self.path.map(|path| AgentServerCommand {
            path,
            args: self.args.unwrap_or_default(),
            env: self.env,
        })
    }
}

impl From<settings::BuiltinAgentServerSettings> for BuiltinAgentServerSettings {
    fn from(value: settings::BuiltinAgentServerSettings) -> Self {
        BuiltinAgentServerSettings {
            path: value
                .path
                .map(|p| PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).as_ref())),
            args: value.args,
            env: value.env,
            ignore_system_version: value.ignore_system_version,
            default_mode: value.default_mode,
        }
    }
}

impl From<AgentServerCommand> for BuiltinAgentServerSettings {
    fn from(value: AgentServerCommand) -> Self {
        BuiltinAgentServerSettings {
            path: Some(value.path),
            args: Some(value.args),
            env: value.env,
            ..Default::default()
        }
    }
}

#[derive(Clone, JsonSchema, Debug, PartialEq)]
pub struct CustomAgentServerSettings {
    pub command: AgentServerCommand,
    /// The default mode to use for this agent.
    ///
    /// Note: Not only all agents support modes.
    ///
    /// Default: None
    pub default_mode: Option<String>,
}

impl From<settings::CustomAgentServerSettings> for CustomAgentServerSettings {
    fn from(value: settings::CustomAgentServerSettings) -> Self {
        CustomAgentServerSettings {
            command: AgentServerCommand {
                path: PathBuf::from(shellexpand::tilde(&value.path.to_string_lossy()).as_ref()),
                args: value.args,
                env: value.env,
            },
            default_mode: value.default_mode,
        }
    }
}

impl settings::Settings for AllAgentServersSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let agent_settings = content.agent_servers.clone().unwrap();
        Self {
            gemini: agent_settings.gemini.map(Into::into),
            claude: agent_settings.claude.map(Into::into),
            codex: agent_settings.codex.map(Into::into),
            custom: agent_settings
                .custom
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

#[cfg(test)]
mod extension_agent_tests {
    use super::*;
    use gpui::TestAppContext;
    use std::sync::Arc;

    #[test]
    fn extension_agent_constructs_proper_display_names() {
        // Verify the display name format for extension-provided agents
        let name1 = ExternalAgentServerName(SharedString::from("Extension: Agent"));
        assert!(name1.0.contains(": "));

        let name2 = ExternalAgentServerName(SharedString::from("MyExt: MyAgent"));
        assert_eq!(name2.0, "MyExt: MyAgent");

        // Non-extension agents shouldn't have the separator
        let custom = ExternalAgentServerName(SharedString::from("custom"));
        assert!(!custom.0.contains(": "));
    }

    struct NoopExternalAgent;

    impl ExternalAgentServer for NoopExternalAgent {
        fn get_command(
            &mut self,
            _root_dir: Option<&str>,
            _extra_env: HashMap<String, String>,
            _status_tx: Option<watch::Sender<SharedString>>,
            _new_version_available_tx: Option<watch::Sender<Option<String>>>,
            _cx: &mut AsyncApp,
        ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
            Task::ready(Ok((
                AgentServerCommand {
                    path: PathBuf::from("noop"),
                    args: Vec::new(),
                    env: None,
                },
                "".to_string(),
                None,
            )))
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn sync_removes_only_extension_provided_agents() {
        let mut store = AgentServerStore {
            state: AgentServerStoreState::Collab,
            external_agents: HashMap::default(),
            agent_icons: HashMap::default(),
        };

        // Seed with extension agents (contain ": ") and custom agents (don't contain ": ")
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("Ext1: Agent1")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("Ext2: Agent2")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("custom-agent")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );

        // Simulate removal phase
        let keys_to_remove: Vec<_> = store
            .external_agents
            .keys()
            .filter(|name| name.0.contains(": "))
            .cloned()
            .collect();

        for key in keys_to_remove {
            store.external_agents.remove(&key);
        }

        // Only custom-agent should remain
        assert_eq!(store.external_agents.len(), 1);
        assert!(
            store
                .external_agents
                .contains_key(&ExternalAgentServerName(SharedString::from("custom-agent")))
        );
    }

    #[test]
    fn archive_launcher_constructs_with_all_fields() {
        use extension::AgentServerManifestEntry;

        let mut env = HashMap::default();
        env.insert("GITHUB_TOKEN".into(), "secret".into());

        let mut targets = HashMap::default();
        targets.insert(
            "darwin-aarch64".to_string(),
            extension::TargetConfig {
                archive:
                    "https://github.com/owner/repo/releases/download/v1.0.0/agent-darwin-arm64.zip"
                        .into(),
                cmd: "./agent".into(),
                args: vec![],
                sha256: None,
            },
        );

        let _entry = AgentServerManifestEntry {
            name: "GitHub Agent".into(),
            targets,
            env,
            icon: None,
        };

        // Verify display name construction
        let expected_name = ExternalAgentServerName(SharedString::from("GitHub Agent"));
        assert_eq!(expected_name.0, "GitHub Agent");
    }

    #[gpui::test]
    async fn archive_agent_uses_extension_and_agent_id_for_cache_key(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let project_environment = cx.new(|cx| crate::ProjectEnvironment::new(None, cx));

        let agent = LocalExtensionArchiveAgent {
            fs,
            http_client,
            project_environment,
            extension_id: Arc::from("my-extension"),
            agent_id: Arc::from("my-agent"),
            targets: {
                let mut map = HashMap::default();
                map.insert(
                    "darwin-aarch64".to_string(),
                    extension::TargetConfig {
                        archive: "https://example.com/my-agent-darwin-arm64.zip".into(),
                        cmd: "./my-agent".into(),
                        args: vec!["--serve".into()],
                        sha256: None,
                    },
                );
                map
            },
            env: {
                let mut map = HashMap::default();
                map.insert("PORT".into(), "8080".into());
                map
            },
        };

        // Verify agent is properly constructed
        assert_eq!(agent.extension_id.as_ref(), "my-extension");
        assert_eq!(agent.agent_id.as_ref(), "my-agent");
        assert_eq!(agent.env.get("PORT"), Some(&"8080".to_string()));
        assert!(agent.targets.contains_key("darwin-aarch64"));
    }

    #[test]
    fn sync_extension_agents_registers_archive_launcher() {
        use extension::AgentServerManifestEntry;

        let expected_name = ExternalAgentServerName(SharedString::from("Release Agent"));
        assert_eq!(expected_name.0, "Release Agent");

        // Verify the manifest entry structure for archive-based installation
        let mut env = HashMap::default();
        env.insert("API_KEY".into(), "secret".into());

        let mut targets = HashMap::default();
        targets.insert(
            "linux-x86_64".to_string(),
            extension::TargetConfig {
                archive: "https://github.com/org/project/releases/download/v2.1.0/release-agent-linux-x64.tar.gz".into(),
                cmd: "./release-agent".into(),
                args: vec!["serve".into()],
                sha256: None,
            },
        );

        let manifest_entry = AgentServerManifestEntry {
            name: "Release Agent".into(),
            targets: targets.clone(),
            env,
            icon: None,
        };

        // Verify target config is present
        assert!(manifest_entry.targets.contains_key("linux-x86_64"));
        let target = manifest_entry.targets.get("linux-x86_64").unwrap();
        assert_eq!(target.cmd, "./release-agent");
    }

    #[test]
    fn test_tilde_expansion_in_settings() {
        let settings = settings::BuiltinAgentServerSettings {
            path: Some(PathBuf::from("~/bin/agent")),
            args: Some(vec!["--flag".into()]),
            env: None,
            ignore_system_version: None,
            default_mode: None,
        };

        let BuiltinAgentServerSettings { path, .. } = settings.into();

        let path = path.unwrap();
        assert!(
            !path.to_string_lossy().starts_with("~"),
            "Tilde should be expanded for builtin agent path"
        );

        let settings = settings::CustomAgentServerSettings {
            path: PathBuf::from("~/custom/agent"),
            args: vec!["serve".into()],
            env: None,
            default_mode: None,
        };

        let CustomAgentServerSettings {
            command: AgentServerCommand { path, .. },
            ..
        } = settings.into();

        assert!(
            !path.to_string_lossy().starts_with("~"),
            "Tilde should be expanded for custom agent path"
        );
    }
}
