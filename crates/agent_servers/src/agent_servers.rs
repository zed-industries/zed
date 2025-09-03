mod acp;
mod claude;
mod custom;
mod gemini;
mod settings;

#[cfg(any(test, feature = "test-support"))]
pub mod e2e_tests;

use anyhow::Context as _;
pub use claude::*;
pub use custom::*;
use fs::Fs;
use fs::RemoveOptions;
use fs::RenameOptions;
use futures::StreamExt as _;
pub use gemini::*;
use gpui::AppContext;
use node_runtime::NodeRuntime;
pub use settings::*;

use acp_thread::AgentConnection;
use acp_thread::LoadError;
use anyhow::Result;
use anyhow::anyhow;
use collections::HashMap;
use gpui::{App, AsyncApp, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::str::FromStr as _;
use std::{
    any::Any,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use util::ResultExt as _;

pub fn init(cx: &mut App) {
    settings::init(cx);
}

pub struct AgentServerDelegate {
    project: Entity<Project>,
    status_tx: Option<watch::Sender<SharedString>>,
    new_version_available: Option<watch::Sender<Option<String>>>,
}

impl AgentServerDelegate {
    pub fn new(
        project: Entity<Project>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_tx: Option<watch::Sender<Option<String>>>,
    ) -> Self {
        Self {
            project,
            status_tx,
            new_version_available: new_version_tx,
        }
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    fn get_or_npm_install_builtin_agent(
        self,
        binary_name: SharedString,
        package_name: SharedString,
        entrypoint_path: PathBuf,
        ignore_system_version: bool,
        minimum_version: Option<Version>,
        cx: &mut App,
    ) -> Task<Result<AgentServerCommand>> {
        let project = self.project;
        let fs = project.read(cx).fs().clone();
        let Some(node_runtime) = project.read(cx).node_runtime().cloned() else {
            return Task::ready(Err(anyhow!(
                "External agents are not yet available in remote projects."
            )));
        };
        let status_tx = self.status_tx;
        let new_version_available = self.new_version_available;

        cx.spawn(async move |cx| {
            if !ignore_system_version {
                if let Some(bin) = find_bin_in_path(binary_name.clone(), &project, cx).await {
                    return Ok(AgentServerCommand {
                        path: bin,
                        args: Vec::new(),
                        env: Default::default(),
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
                                Self::download_latest_version(
                                    fs,
                                    dir.clone(),
                                    node_runtime,
                                    package_name,
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
                    cx.background_spawn(Self::download_latest_version(
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

                anyhow::Ok(AgentServerCommand {
                    path: node_path,
                    args: vec![agent_server_path.to_string_lossy().to_string()],
                    env: Default::default(),
                })
            })
            .await
            .map_err(|e| LoadError::FailedToInstall(e.to_string().into()).into())
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
                overwrite: false,
            },
        )
        .await?;

        anyhow::Ok(version)
    }
}

pub trait AgentServer: Send {
    fn logo(&self) -> ui::IconName;
    fn name(&self) -> SharedString;
    fn telemetry_id(&self) -> &'static str;

    fn connect(
        &self,
        root_dir: &Path,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>>;

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl dyn AgentServer {
    pub fn downcast<T: 'static + AgentServer + Sized>(self: Rc<Self>) -> Option<Rc<T>> {
        self.into_any().downcast().ok()
    }
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

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct AgentServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

impl AgentServerCommand {
    pub async fn resolve(
        path_bin_name: &'static str,
        extra_args: &[&'static str],
        fallback_path: Option<&Path>,
        settings: Option<BuiltinAgentServerSettings>,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Option<Self> {
        if let Some(settings) = settings
            && let Some(command) = settings.custom_command()
        {
            Some(command)
        } else {
            match find_bin_in_path(path_bin_name.into(), project, cx).await {
                Some(path) => Some(Self {
                    path,
                    args: extra_args.iter().map(|arg| arg.to_string()).collect(),
                    env: None,
                }),
                None => fallback_path.and_then(|path| {
                    if path.exists() {
                        Some(Self {
                            path: path.to_path_buf(),
                            args: extra_args.iter().map(|arg| arg.to_string()).collect(),
                            env: None,
                        })
                    } else {
                        None
                    }
                }),
            }
        }
    }
}

async fn find_bin_in_path(
    bin_name: SharedString,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Option<PathBuf> {
    let (env_task, root_dir) = project
        .update(cx, |project, cx| {
            let worktree = project.visible_worktrees(cx).next();
            match worktree {
                Some(worktree) => {
                    let env_task = project.environment().update(cx, |env, cx| {
                        env.get_worktree_environment(worktree.clone(), cx)
                    });

                    let path = worktree.read(cx).abs_path();
                    (env_task, path)
                }
                None => {
                    let path: Arc<Path> = paths::home_dir().as_path().into();
                    let env_task = project.environment().update(cx, |env, cx| {
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
