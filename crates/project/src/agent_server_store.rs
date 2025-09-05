use std::{
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr as _,
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow, bail};
use collections::HashMap;
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::StreamExt as _;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, SharedString, Task};
use node_runtime::NodeRuntime;
use remote::RemoteClient;
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, ToProto},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use util::{ResultExt as _, debug_panic};

use crate::{Project, ProjectEnvironment, worktree_store::WorktreeStore};

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

impl AgentServerCommand {
    fn from_proto(proto: proto::AgentServerCommand) -> Self {
        Self {
            path: proto.path.into(),
            args: proto.args,
            env: Some(proto.env.into_iter().collect()),
        }
    }

    fn to_proto(self) -> proto::AgentServerCommand {
        proto::AgentServerCommand {
            path: self.path.to_string_lossy().to_string(),
            args: self.args,
            env: self
                .env
                .map(|env| env.into_iter().collect())
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternalAgentServerName(pub SharedString);

impl std::fmt::Display for ExternalAgentServerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait ExternalAgentServer {
    // FIXME status_tx et al
    fn get_command(
        &self,
        root_dir: &str,
        extra_env: HashMap<String, String>,
        cx: &mut App,
    ) -> Task<Result<AgentServerCommand>>;
}

enum AgentServerStoreState {
    Local {
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        worktree_store: Entity<WorktreeStore>,
        project_environment: Entity<ProjectEnvironment>,
        downstream_client: Option<(u64, AnyProtoClient)>,
    },
    Remote {
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
    },
    Collab,
}

pub struct AgentServerStore {
    state: AgentServerStoreState,
    external_agents: HashMap<ExternalAgentServerName, Rc<dyn ExternalAgentServer>>,
}

impl AgentServerStore {
    pub fn init_remote(session: &AnyProtoClient) {
        session.add_entity_message_handler(Self::handle_external_agents_updated);
    }

    pub fn init_headless(session: &AnyProtoClient) {
        session.add_entity_request_handler(Self::handle_get_agent_server_command);
    }

    pub fn local(
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        worktree_store: Entity<WorktreeStore>,
        project_environment: Entity<ProjectEnvironment>,
        _cx: &mut Context<Self>,
    ) -> Self {
        // FIXME need to subscribe to settings changed and send ExternalAgentsUpdated

        let external_agents = [
            (
                gemini(),
                Rc::new(LocalGemini {
                    fs: fs.clone(),
                    node_runtime: node_runtime.clone(),
                    project_environment: project_environment.clone(),
                }) as Rc<dyn ExternalAgentServer>,
            ),
            // FIXME claude
        ]
        .into_iter()
        .collect();
        // FIXME read settings at start too

        Self {
            state: AgentServerStoreState::Local {
                node_runtime,
                fs,
                worktree_store,
                project_environment,
                downstream_client: None,
            },
            external_agents,
        }
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
                gemini(),
                Rc::new(RemoteExternalAgentServer {
                    project_id,
                    upstream_client: upstream_client.clone(),
                    name: gemini(),
                }) as Rc<dyn ExternalAgentServer>,
            ),
            // FIXME claude
        ]
        .into_iter()
        .collect();

        Self {
            state: AgentServerStoreState::Remote {
                project_id,
                upstream_client: upstream_client.clone(),
            },
            external_agents,
        }
    }

    pub(crate) fn collab(_cx: &mut Context<Self>) -> Self {
        Self {
            state: AgentServerStoreState::Collab,
            external_agents: Default::default(),
        }
    }

    pub fn shared(&mut self, project_id: u64, client: AnyProtoClient) {
        match &mut self.state {
            AgentServerStoreState::Local {
                downstream_client, ..
            } => {
                // FIXME need a subscription for settings changed so that we can send down the new servers
                *downstream_client = Some((project_id, client));
            }
            AgentServerStoreState::Remote { .. } => {}
            AgentServerStoreState::Collab => {}
        }
    }

    pub fn get_external_agent(
        &self,
        name: &ExternalAgentServerName,
    ) -> Option<Rc<dyn ExternalAgentServer>> {
        self.external_agents.get(name).cloned()
    }

    async fn handle_get_agent_server_command(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetAgentServerCommand>,
        mut cx: AsyncApp,
    ) -> Result<proto::AgentServerCommand> {
        let command = this
            .update(&mut cx, |this, cx| {
                let Some(agent) = this.external_agents.get(&ExternalAgentServerName(
                    envelope.payload.name.clone().into(),
                )) else {
                    return Task::ready(Err(anyhow!(
                        "agent `{}` not found",
                        envelope.payload.name
                    )));
                };
                agent.get_command(&envelope.payload.root_dir, HashMap::default(), cx)
            })?
            .await?;
        Ok(command.to_proto())
    }

    async fn handle_external_agents_updated(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExternalAgentsUpdated>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _cx| {
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

            this.external_agents = envelope
                .payload
                .names
                .into_iter()
                .map(|name| {
                    let agent = RemoteExternalAgentServer {
                        project_id: *project_id,
                        upstream_client: upstream_client.clone(),
                        name: ExternalAgentServerName(name.clone().into()),
                    };
                    (
                        ExternalAgentServerName(name.into()),
                        Rc::new(agent) as Rc<dyn ExternalAgentServer>,
                    )
                })
                .collect();
            Ok(())
            // FIXME emit an event
        })?
    }
}

fn get_or_npm_install_builtin_agent(
    binary_name: SharedString,
    package_name: SharedString,
    entrypoint_path: PathBuf,
    extra_args: Vec<String>,
    minimum_version: Option<semver::Version>,
    root_dir: Arc<Path>,
    status_tx: Option<watch::Sender<SharedString>>,
    new_version_available: Option<watch::Sender<Option<String>>>,
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    cx: &mut App,
) -> Task<std::result::Result<AgentServerCommand, anyhow::Error>> {
    let (custom_command, ignore_system_version) = cx.read_global(|settings: &SettingsStore, _| {
        // FIXME read settings
        (None::<AgentServerCommand>, true)
    });

    cx.spawn(async move |cx| {
        let mut env = project_environment
            .update(cx, |project_environment, cx| {
                project_environment.get_directory_environment(root_dir.clone(), cx)
            })?
            .await
            .unwrap_or_default();

        if let Some(mut command) = custom_command {
            command.args.extend(extra_args);
            env.extend(command.env.unwrap_or_default());
            command.env = Some(env);
            return Ok(command);
        } else if !ignore_system_version {
            if let Some(bin) =
                find_bin_in_path(binary_name.clone(), &root_dir, &project_environment, cx).await
            {
                return Ok(AgentServerCommand {
                    path: bin,
                    args: Vec::new(),
                    env: Some(env),
                });
            }
        }

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
                    && fs.is_file(&dir.join(file_name).join(entrypoint_path)).await
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
                            download_latest_version(
                                fs,
                                dir.clone(),
                                node_runtime,
                                package_name.clone(),
                            )
                            .await
                            .log_err();
                            if let Some(mut new_version_available) = new_version_available {
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
            let mut args = extra_args;
            args.insert(0, agent_server_path.to_string_lossy().to_string());

            anyhow::Ok(AgentServerCommand {
                path: node_path,
                args,
                env: Some(env),
            })
        })
        .await
        // FIXME restore this at a higher level
        // .map_err(|e| LoadError::FailedToInstall(e.to_string().into()).into())
    })
}

async fn find_bin_in_path(
    bin_name: SharedString,
    root_dir: &Path,
    project_environment: &Entity<ProjectEnvironment>,
    cx: &mut AsyncApp,
) -> Option<PathBuf> {
    let env_task = project_environment
        .update(cx, |environment, cx| {
            environment.get_directory_environment(root_dir.into(), cx)
        })
        .log_err()?;

    let root_dir = root_dir.to_path_buf();
    cx.background_executor()
        .spawn(async move {
            let which_result = if cfg!(windows) {
                which::which(bin_name.as_str())
            } else {
                let env = env_task.await.unwrap_or_default();
                let shell_path = env.get("PATH").cloned();
                which::which_in(bin_name.as_str(), shell_path.as_ref(), &root_dir)
            };

            if let Err(which::Error::CannotFindBinaryPath) = which_result {
                return None;
            }

            which_result.log_err()
        })
        .await
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
            overwrite: false,
        },
    )
    .await?;

    anyhow::Ok(version)
}

struct RemoteExternalAgentServer {
    project_id: u64,
    upstream_client: Entity<RemoteClient>,
    name: ExternalAgentServerName,
}

impl ExternalAgentServer for RemoteExternalAgentServer {
    fn get_command(
        &self,
        root_dir: &str,
        extra_env: HashMap<String, String>,
        cx: &mut App,
    ) -> Task<Result<AgentServerCommand>> {
        // FIXME need to get fallback path (when no suitable worktrees) on the remote

        let command =
            self.upstream_client
                .read(cx)
                .proto_client()
                .request(proto::GetAgentServerCommand {
                    project_id: self.project_id,
                    name: self.name.to_string(),
                    root_dir: root_dir.to_string(),
                });

        let upstream_client = self.upstream_client.downgrade();
        let root_dir = root_dir.to_string();
        cx.spawn(async move |cx| {
            let mut command = command.await?;
            command.env.extend(extra_env);
            let command = upstream_client.update(cx, |client, cx| {
                client.build_command(
                    Some(command.path),
                    &command.args,
                    &command.env.into_iter().collect(),
                    Some(root_dir),
                    None,
                )
            })??;
            Ok(AgentServerCommand {
                path: command.program.into(),
                args: command.args,
                env: Some(command.env),
            })
        })
    }
}

struct LocalGemini {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
}

impl ExternalAgentServer for LocalGemini {
    fn get_command(
        &self,
        root_dir: &str,
        extra_env: HashMap<String, String>,
        cx: &mut App,
    ) -> Task<Result<AgentServerCommand>> {
        let command = get_or_npm_install_builtin_agent(
            "gemini".into(),
            "@google/gemini-cli".into(),
            "node_modules/@google/gemini-cli/dist/index.js".into(),
            vec!["--experimental-acp".into()],
            Some("0.2.1".parse().unwrap()),
            Path::new(root_dir).into(),
            // FIXME
            None,
            None,
            self.fs.clone(),
            self.node_runtime.clone(),
            self.project_environment.clone(),
            cx,
        );

        cx.spawn(async move |_| {
            let mut command = command.await?;
            command.env.get_or_insert_default().extend(extra_env);
            Ok(command)
        })
    }
}

struct LocalCustomAgent {
    command: AgentServerCommand,
}

impl ExternalAgentServer for LocalCustomAgent {
    fn get_command(
        &self,
        _root_dir: &str,
        extra_env: HashMap<String, String>,
        cx: &mut App,
    ) -> Task<Result<AgentServerCommand>> {
        let command = self.command.clone();
        cx.spawn(async move |_| {
            let mut command = command;
            command.env.get_or_insert_default().extend(extra_env);
            Ok(command)
        })
    }
}

pub fn gemini() -> ExternalAgentServerName {
    ExternalAgentServerName("gemini".into())
}

pub fn claude_code() -> ExternalAgentServerName {
    ExternalAgentServerName("claude".into())
}

// FIXME claude
