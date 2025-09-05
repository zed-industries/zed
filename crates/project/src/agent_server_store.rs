use std::{
    path::{Path, PathBuf},
    str::FromStr as _,
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::StreamExt as _;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, SharedString, Task};
use node_runtime::NodeRuntime;
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, ToProto},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use util::ResultExt as _;

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
        upstream_client: AnyProtoClient,
    },
    Collab,
}

pub struct AgentServerStore {
    state: AgentServerStoreState,
}

impl AgentServerStore {
    pub fn init(session: &AnyProtoClient) {
        session.add_entity_request_handler(Self::handle_get_agent_server_command);
    }

    pub fn local(
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        worktree_store: Entity<WorktreeStore>,
        project_environment: Entity<ProjectEnvironment>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            state: AgentServerStoreState::Local {
                node_runtime,
                fs,
                worktree_store,
                project_environment,
                downstream_client: None,
            },
        }
    }

    pub(crate) fn remote(
        project_id: u64,
        upstream_client: AnyProtoClient,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            state: AgentServerStoreState::Remote {
                project_id,
                upstream_client,
            },
        }
    }

    pub(crate) fn collab(_cx: &mut Context<Self>) -> Self {
        Self {
            state: AgentServerStoreState::Collab,
        }
    }

    pub fn shared(&mut self, project_id: u64, client: AnyProtoClient) {
        match &mut self.state {
            AgentServerStoreState::Local {
                downstream_client, ..
            } => *downstream_client = Some((project_id, client)),
            AgentServerStoreState::Remote { .. } => {}
            AgentServerStoreState::Collab => {}
        }
    }

    async fn handle_get_agent_server_command(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetAgentServerCommand>,
        mut cx: AsyncApp,
    ) -> Result<proto::AgentServerCommand> {
        let command = this
            .update(&mut cx, |this, cx| {
                anyhow::Ok(
                    this.get_agent_server_command(
                        envelope.payload.binary_name.into(),
                        envelope.payload.package_name.into(),
                        envelope.payload.entrypoint_path.into(),
                        envelope.payload.settings_key.into(),
                        envelope.payload.extra_args,
                        envelope.payload.extra_env.into_iter().collect(),
                        envelope
                            .payload
                            .minimum_version
                            .map(|version| semver::Version::from_str(&version))
                            .transpose()?,
                        None,
                        None,
                        Path::new(&envelope.payload.root_dir).into(),
                        cx,
                    ),
                )
            })??
            .await?;
        Ok(command.to_proto())
    }

    pub fn get_agent_server_command(
        &self,
        binary_name: SharedString,
        package_name: SharedString,
        entrypoint_path: PathBuf,
        settings_key: SharedString,
        extra_args: Vec<String>,
        extra_env: HashMap<String, String>,
        minimum_version: Option<semver::Version>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available: Option<watch::Sender<Option<String>>>,
        root_dir: Arc<Path>,
        cx: &mut App,
    ) -> Task<Result<AgentServerCommand>> {
        match &self.state {
            AgentServerStoreState::Local {
                node_runtime,
                fs,
                worktree_store,
                project_environment,
                // TODO: report progress via downstream client
                downstream_client: _,
            } => {
                let (custom_command, ignore_system_version) =
                    cx.read_global(|settings: &SettingsStore, _| {
                        // FIXME read settings
                        (None::<AgentServerCommand>, true)
                    });
                let node_runtime = node_runtime.clone();
                let fs = fs.clone();
                let worktree_store = worktree_store.clone();
                let project_environment = project_environment.clone();

                cx.spawn(async move |cx| {
                    let mut env = project_environment
                        .update(cx, |project_environment, cx| {
                            project_environment.get_directory_environment(root_dir, cx)
                        })?
                        .await
                        .unwrap_or_default();

                    if let Some(mut command) = custom_command {
                        command.args.extend(extra_args);
                        // The project environment is overridden by the
                        // agent-specific environment, which is overridden by
                        // any custom environment variables the user has
                        // specified.
                        env.extend(extra_env);
                        env.extend(command.env.unwrap_or_default());
                        command.env = Some(env);
                        return Ok(command);
                    } else if !ignore_system_version {
                        if let Some(bin) = find_bin_in_path(
                            binary_name.clone(),
                            &worktree_store,
                            &project_environment,
                            cx,
                        )
                        .await
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
                        let newest_version = if let Some((version, file_name)) =
                            versions.last().cloned()
                            && minimum_version
                                .is_none_or(|minimum_version| version >= minimum_version)
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
                                        .await;
                                    if let Ok(latest_version) = latest_version
                                        && &latest_version != &file_name.to_string_lossy()
                                    {
                                        download_latest_version(
                                            fs,
                                            dir.clone(),
                                            node_runtime,
                                            package_name,
                                        )
                                        .await
                                        .log_err();
                                        if let Some(mut new_version_available) =
                                            new_version_available
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
                                package_name,
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
            AgentServerStoreState::Remote {
                project_id,
                upstream_client,
            } => {
                let command = upstream_client.request(proto::GetAgentServerCommand {
                    project_id: *project_id,
                    binary_name: binary_name.to_string(),
                    package_name: package_name.to_string(),
                    entrypoint_path: entrypoint_path.to_proto(),
                    settings_key: settings_key.to_string(),
                    minimum_version: minimum_version.map(|version| version.to_string()),
                    root_dir: root_dir.to_proto(),
                    extra_args,
                    extra_env: extra_env.into_iter().collect(),
                });
                cx.spawn(async move |_| {
                    let command = command.await?;
                    Ok(AgentServerCommand::from_proto(command))
                })
            }
            AgentServerStoreState::Collab => Task::ready(Err(anyhow!(
                "External agents are not supported in projects shared via collab"
            ))),
        }
    }
}

async fn find_bin_in_path(
    bin_name: SharedString,
    worktree_store: &Entity<WorktreeStore>,
    project_environment: &Entity<ProjectEnvironment>,
    cx: &mut AsyncApp,
) -> Option<PathBuf> {
    let (env_task, root_dir) = worktree_store
        .update(cx, |worktree_store, cx| {
            let worktree = worktree_store.visible_worktrees(cx).next();
            match worktree {
                Some(worktree) => {
                    let env_task = project_environment.update(cx, |env, cx| {
                        env.get_worktree_environment(worktree.clone(), cx)
                    });

                    let path = worktree.read(cx).abs_path();
                    (env_task, path)
                }
                None => {
                    let path: Arc<Path> = paths::home_dir().as_path().into();
                    let env_task = project_environment.update(cx, |env, cx| {
                        env.get_directory_environment(path.clone(), cx)
                    });
                    (env_task, path)
                }
            }
        })
        .log_err()?;

    cx.background_executor()
        .spawn(async move {
            let which_result = if cfg!(windows) {
                which::which(bin_name.as_str())
            } else {
                let env = env_task.await.unwrap_or_default();
                let shell_path = env.get("PATH").cloned();
                which::which_in(bin_name.as_str(), shell_path.as_ref(), root_dir.as_ref())
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
