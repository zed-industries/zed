mod acp;
mod claude;
mod custom;
mod gemini;
mod settings;

#[cfg(any(test, feature = "test-support"))]
pub mod e2e_tests;

pub use claude::*;
pub use custom::*;
pub use gemini::*;
pub use settings::*;

use acp_thread::AgentConnection;
use acp_thread::LoadError;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use collections::HashMap;
use gpui::AppContext as _;
use gpui::{App, AsyncApp, Entity, SharedString, Task};
use node_runtime::VersionStrategy;
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
    status_tx: watch::Sender<SharedString>,
}

impl AgentServerDelegate {
    pub fn new(project: Entity<Project>, status_tx: watch::Sender<SharedString>) -> Self {
        Self { project, status_tx }
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
            return Task::ready(Err(anyhow!("Missing node runtime")));
        };
        let mut status_tx = self.status_tx;

        cx.spawn(async move |cx| {
            if !ignore_system_version {
                if let Some(bin) = find_bin_in_path(binary_name.clone(), &project, cx).await {
                    return Ok(AgentServerCommand { path: bin, args: Vec::new(), env: Default::default() })
                }
            }

            cx.background_spawn(async move {
                let node_path = node_runtime.binary_path().await?;
                let dir = paths::data_dir().join("external_agents").join(binary_name.as_str());
                fs.create_dir(&dir).await?;
                let local_executable_path = dir.join(entrypoint_path);
                let command = AgentServerCommand {
                    path: node_path,
                    args: vec![local_executable_path.to_string_lossy().to_string()],
                    env: Default::default(),
                };

                let installed_version = node_runtime
                    .npm_package_installed_version(&dir, &package_name)
                    .await?
                    .filter(|version| {
                        Version::from_str(&version)
                            .is_ok_and(|version| Some(version) >= minimum_version)
                    });

                status_tx.send("Checking for latest version…".into())?;
                let latest_version = match node_runtime.npm_package_latest_version(&package_name).await
                {
                    Ok(latest_version) => latest_version,
                    Err(e) => {
                        if let Some(installed_version) = installed_version {
                            log::error!("{e}");
                            log::warn!("failed to fetch latest version of {package_name}, falling back to cached version {installed_version}");
                            return Ok(command);
                        } else {
                            bail!(e);
                        }
                    }
                };

                let should_install = node_runtime
                    .should_install_npm_package(
                        &package_name,
                        &local_executable_path,
                        &dir,
                        VersionStrategy::Latest(&latest_version),
                    )
                    .await;

                if should_install {
                    status_tx.send("Installing latest version…".into())?;
                    node_runtime
                        .npm_install_packages(&dir, &[(&package_name, &latest_version)])
                        .await?;
                }

                Ok(command)
            }).await.map_err(|e| LoadError::FailedToInstall(e.to_string().into()).into())
        })
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
