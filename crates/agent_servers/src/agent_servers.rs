use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, AsyncApp, Entity, SharedString};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsStore};
use util::{ResultExt, paths};

pub fn init(cx: &mut App) {
    AllAgentServersSettings::register(cx);
}

#[derive(Default, Deserialize, Serialize, Clone, JsonSchema, Debug)]
pub struct AllAgentServersSettings {
    gemini: Option<AgentServerSettings>,
}

#[derive(Deserialize, Serialize, Clone, JsonSchema, Debug)]
pub struct AgentServerSettings {
    #[serde(flatten)]
    command: AgentServerCommand,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct AgentServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

pub struct Gemini;

pub struct AgentServerVersion {
    pub current_version: SharedString,
    pub supported: bool,
}

pub trait AgentServer: Send {
    fn command(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<AgentServerCommand>>;

    fn version(
        &self,
        command: &AgentServerCommand,
    ) -> impl Future<Output = Result<AgentServerVersion>> + Send;
}

const GEMINI_ACP_ARG: &str = "--acp";

impl AgentServer for Gemini {
    async fn command(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<AgentServerCommand> {
        let custom_command = cx.read_global(|settings: &SettingsStore, _| {
            let settings = settings.get::<AllAgentServersSettings>(None);
            settings
                .gemini
                .as_ref()
                .map(|gemini_settings| AgentServerCommand {
                    path: gemini_settings.command.path.clone(),
                    args: gemini_settings
                        .command
                        .args
                        .iter()
                        .cloned()
                        .chain(std::iter::once(GEMINI_ACP_ARG.into()))
                        .collect(),
                    env: gemini_settings.command.env.clone(),
                })
        })?;

        if let Some(custom_command) = custom_command {
            return Ok(custom_command);
        }

        if let Some(path) = find_bin_in_path("gemini", project, cx).await {
            return Ok(AgentServerCommand {
                path,
                args: vec![GEMINI_ACP_ARG.into()],
                env: None,
            });
        }

        let (fs, node_runtime) = project.update(cx, |project, _| {
            (project.fs().clone(), project.node_runtime().cloned())
        })?;
        let node_runtime = node_runtime.context("gemini not found on path")?;

        let directory = ::paths::agent_servers_dir().join("gemini");
        fs.create_dir(&directory).await?;
        node_runtime
            .npm_install_packages(&directory, &[("@google/gemini-cli", "latest")])
            .await?;
        let path = directory.join("node_modules/.bin/gemini");

        Ok(AgentServerCommand {
            path,
            args: vec![GEMINI_ACP_ARG.into()],
            env: None,
        })
    }

    async fn version(&self, command: &AgentServerCommand) -> Result<AgentServerVersion> {
        let version_fut = util::command::new_smol_command(&command.path)
            .args(command.args.iter())
            .arg("--version")
            .kill_on_drop(true)
            .output();

        let help_fut = util::command::new_smol_command(&command.path)
            .args(command.args.iter())
            .arg("--help")
            .kill_on_drop(true)
            .output();

        let (version_output, help_output) = futures::future::join(version_fut, help_fut).await;

        let current_version = String::from_utf8(version_output?.stdout)?.into();
        let supported = String::from_utf8(help_output?.stdout)?.contains(GEMINI_ACP_ARG);

        Ok(AgentServerVersion {
            current_version,
            supported,
        })
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

impl settings::Settings for AllAgentServersSettings {
    const KEY: Option<&'static str> = Some("agent_servers");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let mut settings = AllAgentServersSettings::default();

        for value in sources.defaults_and_customizations() {
            if value.gemini.is_some() {
                settings.gemini = value.gemini.clone();
            }
        }

        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
