use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, AsyncApp, Entity};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsStore};
use util::paths;

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

impl AgentServerCommand {
    pub async fn gemini(project: &Entity<Project>, cx: &mut AsyncApp) -> Result<Self> {
        const ACP_ARG: &str = "--acp";

        let custom_command = cx.read_global(|settings: &SettingsStore, _| {
            let settings = settings.get::<AllAgentServersSettings>(None);
            settings.gemini.as_ref().map(|gemini| AgentServerCommand {
                path: gemini.command.path.clone(),
                args: gemini
                    .command
                    .args
                    .iter()
                    .cloned()
                    .chain(std::iter::once(ACP_ARG.into()))
                    .collect(),
                env: gemini.command.env.clone(),
            })
        })?;

        if let Some(custom_command) = custom_command {
            return Ok(custom_command);
        }

        let path = Self::find_bin_in_path("gemini", project, cx).await?;

        Ok(Self {
            path,
            args: vec![ACP_ARG.into()],
            env: None,
        })
    }

    async fn find_bin_in_path(
        bin_name: &'static str,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<PathBuf> {
        let (env_task, root_dir) = project.update(cx, |project, cx| {
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
        })?;

        if cfg!(windows) {
            which::which(bin_name)
        } else {
            let env = env_task.await.unwrap_or_default();
            let shell_path = env.get("PATH").cloned();
            which::which_in(bin_name, shell_path.as_ref(), root_dir.as_ref())
        }
        .context(format!("Failed to find `{}` in your PATH", bin_name))
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
