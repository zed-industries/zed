use std::{
    any::Any,
    borrow::Borrow,
    path::{Path, PathBuf},
    str::FromStr as _,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, Result, bail};
use client::Client;
use collections::HashMap;
use feature_flags::FeatureFlagAppExt as _;
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::StreamExt as _;
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription, Task,
};
use node_runtime::NodeRuntime;
use remote::RemoteClient;
use rpc::{AnyProtoClient, TypedEnvelope, proto};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{SettingsContent, SettingsStore};
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
    _feature_flag_subscription: Option<gpui::Subscription>,
}

pub struct AgentServersUpdated;

impl EventEmitter<AgentServersUpdated> for AgentServerStore {}

impl AgentServerStore {
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
            node_runtime,
            fs,
            project_environment,
            downstream_client,
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
                    .unwrap_or(true),
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

        use feature_flags::FeatureFlagAppExt as _;
        if cx.has_flag::<feature_flags::CodexAcpFeatureFlag>() || new_settings.codex.is_some() {
            self.external_agents.insert(
                CODEX_NAME.into(),
                Box::new(LocalCodex {
                    fs: fs.clone(),
                    project_environment: project_environment.clone(),
                    custom_command: new_settings
                        .codex
                        .clone()
                        .and_then(|settings| settings.custom_command()),
                }),
            );
        }

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

        *old_settings = Some(new_settings.clone());

        if let Some((project_id, downstream_client)) = downstream_client {
            downstream_client
                .send(proto::ExternalAgentsUpdated {
                    project_id: *project_id,
                    names: self
                        .external_agents
                        .keys()
                        .filter(|name| name.0 != CODEX_NAME)
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
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe_global::<SettingsStore>(|this, cx| {
            this.agent_servers_settings_changed(cx);
        });
        let this_handle = cx.weak_entity();
        let feature_flags_subscription =
            cx.observe_flag::<feature_flags::CodexAcpFeatureFlag, _>(move |_enabled, cx| {
                let _ = this_handle.update(cx, |this, cx| {
                    this.agent_servers_settings_changed(cx);
                });
            });
        let mut this = Self {
            state: AgentServerStoreState::Local {
                node_runtime,
                fs,
                project_environment,
                downstream_client: None,
                settings: None,
                _subscriptions: [subscription],
            },
            external_agents: Default::default(),
            _feature_flag_subscription: Some(feature_flags_subscription),
        };
        this.agent_servers_settings_changed(cx);
        this
    }

    pub(crate) fn remote(
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
        _cx: &mut Context<Self>,
    ) -> Self {
        // Set up the builtin agents here so they're immediately available in
        // remote projects--we know that the HeadlessProject on the other end
        // will have them.
        let external_agents = [
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
        ]
        .into_iter()
        .collect();

        Self {
            state: AgentServerStoreState::Remote {
                project_id,
                upstream_client,
            },
            external_agents,
            _feature_flag_subscription: None,
        }
    }

    pub(crate) fn collab(_cx: &mut Context<Self>) -> Self {
        Self {
            state: AgentServerStoreState::Collab,
            external_agents: Default::default(),
            _feature_flag_subscription: None,
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
        let (command, root_dir, login) = this
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
            login: login.map(|login| login.to_proto()),
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
        let dir = paths::data_dir()
            .join("external_agents")
            .join(binary_name.as_str());
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
                    let latest_version =
                        node_runtime.npm_package_latest_version(&package_name).await;
                    if let Ok(latest_version) = latest_version
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
                response
                    .login
                    .map(|login| task::SpawnInTerminal::from_proto(login)),
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
                    project_environment.get_directory_environment(root_dir.clone(), cx)
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
                    Some("0.2.1".parse().unwrap()),
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
                    project_environment.get_directory_environment(root_dir.clone(), cx)
                })?
                .await
                .unwrap_or_default();
            env.insert("ANTHROPIC_API_KEY".into(), "".into());

            let (mut command, login) = if let Some(mut custom_command) = custom_command {
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
            Ok((command, root_dir.to_string_lossy().into_owned(), login))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalCodex {
    fs: Arc<dyn Fs>,
    project_environment: Entity<ProjectEnvironment>,
    custom_command: Option<AgentServerCommand>,
}

impl ExternalAgentServer for LocalCodex {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        _status_tx: Option<watch::Sender<SharedString>>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let project_environment = self.project_environment.downgrade();
        let custom_command = self.custom_command.clone();
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.get_directory_environment(root_dir.clone(), cx)
                })?
                .await
                .unwrap_or_default();

            let mut command = if let Some(mut custom_command) = custom_command {
                env.extend(custom_command.env.unwrap_or_default());
                custom_command.env = Some(env);
                custom_command
            } else {
                let dir = paths::data_dir().join("external_agents").join(CODEX_NAME);
                fs.create_dir(&dir).await?;

                // Find or install the latest Codex release (no update checks for now).
                let http = cx.update(|cx| Client::global(cx).http_client())?;
                let release = ::http_client::github::latest_github_release(
                    "zed-industries/codex-acp",
                    true,
                    false,
                    http.clone(),
                )
                .await
                .context("fetching Codex latest release")?;

                let version_dir = dir.join(&release.tag_name);
                if !fs.is_dir(&version_dir).await {
                    // Assemble release download URL from prefix, tag, and filename based on target triple.
                    // If unsupported, silently skip download.
                    let tag = release.tag_name.clone(); // e.g. "v0.1.0"
                    let version_number = tag.trim_start_matches('v');
                    if let Some(asset_url) = codex_release_url(version_number) {
                        let http = http.clone();
                        let mut response = http
                            .get(&asset_url, Default::default(), true)
                            .await
                            .with_context(|| {
                                format!("downloading Codex binary from {}", asset_url)
                            })?;
                        anyhow::ensure!(
                            response.status().is_success(),
                            "failed to download Codex release: {}",
                            response.status()
                        );

                        // Extract archive into the version directory.
                        if asset_url.ends_with(".zip") {
                            let reader = futures::io::BufReader::new(response.body_mut());
                            util::archive::extract_zip(&version_dir, reader)
                                .await
                                .context("extracting Codex binary from zip")?;
                        } else {
                            // Decompress and extract the tar.gz into the version directory.
                            let reader = futures::io::BufReader::new(response.body_mut());
                            let decoder =
                                async_compression::futures::bufread::GzipDecoder::new(reader);
                            let archive = async_tar::Archive::new(decoder);
                            archive
                                .unpack(&version_dir)
                                .await
                                .context("extracting Codex binary from tar.gz")?;
                        }
                    }
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

/// Assemble Codex release URL for the current OS/arch and the given version number.
/// Returns None if the current target is unsupported.
/// Example output:
/// https://github.com/zed-industries/codex-acp/releases/download/v{version}/codex-acp-{version}-{arch}-{platform}.{ext}
fn codex_release_url(version: &str) -> Option<String> {
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

    let prefix = "https://github.com/zed-industries/codex-acp/releases/download";

    Some(format!(
        "{prefix}/v{version}/codex-acp-{version}-{arch}-{platform}.{ext}"
    ))
}

struct LocalCustomAgent {
    project_environment: Entity<ProjectEnvironment>,
    command: AgentServerCommand,
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
                    project_environment.get_directory_environment(root_dir.clone(), cx)
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

#[cfg(test)]
mod tests {
    #[test]
    fn assembles_codex_release_url_for_current_target() {
        let version_number = "0.1.0";

        // This test fails the build if we are building a version of Zed
        // which does not have a known build of codex-acp, to prevent us
        // from accidentally doing a release on a new target without
        // realizing that codex-acp support will not work on that target!
        //
        // Additionally, it verifies that our logic for assembling URLs
        // correctly resolves to a known-good URL on each of our targets.
        let allowed = [
            "https://github.com/zed-industries/codex-acp/releases/download/v0.1.0/codex-acp-0.1.0-aarch64-apple-darwin.tar.gz",
            "https://github.com/zed-industries/codex-acp/releases/download/v0.1.0/codex-acp-0.1.0-aarch64-pc-windows-msvc.tar.gz",
            "https://github.com/zed-industries/codex-acp/releases/download/v0.1.0/codex-acp-0.1.0-aarch64-unknown-linux-gnu.tar.gz",
            "https://github.com/zed-industries/codex-acp/releases/download/v0.1.0/codex-acp-0.1.0-x86_64-apple-darwin.tar.gz",
            "https://github.com/zed-industries/codex-acp/releases/download/v0.1.0/codex-acp-0.1.0-x86_64-pc-windows-msvc.zip",
            "https://github.com/zed-industries/codex-acp/releases/download/v0.1.0/codex-acp-0.1.0-x86_64-unknown-linux-gnu.tar.gz",
        ];

        if let Some(url) = super::codex_release_url(version_number) {
            assert!(
                allowed.contains(&url.as_str()),
                "Assembled URL {} not in allowed list",
                url
            );
        } else {
            panic!(
                "This target does not have a known codex-acp release! We should fix this by building a release of codex-acp for this target, as otherwise codex-acp will not be usable with this Zed build."
            );
        }
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
            path: value.path,
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
                path: value.path,
                args: value.args,
                env: value.env,
            },
            default_mode: value.default_mode,
        }
    }
}

impl settings::Settings for AllAgentServersSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
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

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut SettingsContent) {}
}
