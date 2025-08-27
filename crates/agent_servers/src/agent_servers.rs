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
use anyhow::Result;
use collections::HashMap;
use gpui::{App, AsyncApp, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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

pub trait AgentServer: Send {
    fn logo(&self) -> ui::IconName;
    fn name(&self) -> SharedString;
    fn empty_state_headline(&self) -> SharedString;
    fn empty_state_message(&self) -> SharedString;
    fn telemetry_id(&self) -> &'static str;

    fn connect(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>>;

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;

    fn install_command(&self) -> Option<&'static str>;
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

pub enum AgentServerVersion {
    Supported,
    Unsupported {
        error_message: SharedString,
        upgrade_message: SharedString,
        upgrade_command: String,
    },
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
        settings: Option<AgentServerSettings>,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Option<Self> {
        if let Some(agent_settings) = settings {
            Some(Self {
                path: agent_settings.command.path,
                args: agent_settings
                    .command
                    .args
                    .into_iter()
                    .chain(extra_args.iter().map(|arg| arg.to_string()))
                    .collect(),
                env: agent_settings.command.env,
            })
        } else {
            match find_bin_in_path(path_bin_name, project, cx).await {
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
    bin_name: &'static str,
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
                which::which(bin_name)
            } else {
                let env = env_task.await.unwrap_or_default();
                let shell_path = env.get("PATH").cloned();
                which::which_in(bin_name, shell_path.as_ref(), root_dir.as_ref())
            };

            if let Err(which::Error::CannotFindBinaryPath) = which_result {
                return None;
            }

            which_result.log_err()
        })
        .await
}
