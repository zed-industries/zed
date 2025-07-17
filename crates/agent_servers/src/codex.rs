use crate::stdio_agent_server::{StdioAgentServer, find_bin_in_path};
use crate::{AgentServerCommand, AgentServerVersion};
use anyhow::{Context as _, Result};
use gpui::{AsyncApp, Entity};
use project::Project;
use settings::SettingsStore;

use crate::AllAgentServersSettings;

#[derive(Clone)]
pub struct Codex;

const ACP_ARG: &str = "acp";

impl StdioAgentServer for Codex {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Welcome to Codex"
    }

    fn empty_state_message(&self) -> &'static str {
        "Ask questions, edit files, run commands.\nBe specific for the best results."
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    async fn command(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<AgentServerCommand> {
        let custom_command = cx.read_global(|settings: &SettingsStore, _| {
            let settings = settings.get::<AllAgentServersSettings>(None);
            settings
                .codex
                .as_ref()
                .map(|codex_settings| AgentServerCommand {
                    path: codex_settings.command.path.clone(),
                    args: codex_settings
                        .command
                        .args
                        .iter()
                        .cloned()
                        .chain(std::iter::once(ACP_ARG.into()))
                        .collect(),
                    env: codex_settings.command.env.clone(),
                })
        })?;

        if let Some(custom_command) = custom_command {
            return Ok(custom_command);
        }

        if let Some(path) = find_bin_in_path("codex", project, cx).await {
            return Ok(AgentServerCommand {
                path,
                args: vec![ACP_ARG.into()],
                env: None,
            });
        }

        todo!()
        // let (fs, node_runtime) = project.update(cx, |project, _| {
        //     (project.fs().clone(), project.node_runtime().cloned())
        // })?;
        // let node_runtime = node_runtime.context("codex not found on path")?;

        // let directory = ::paths::agent_servers_dir().join("codex");
        // fs.create_dir(&directory).await?;
        // node_runtime
        //     .npm_install_packages(&directory, &[("@google/gemini-cli", "latest")])
        //     .await?;
        // let path = directory.join("node_modules/.bin/gemini");

        // Ok(AgentServerCommand {
        //     path,
        //     args: vec![ACP_ARG.into()],
        //     env: None,
        // })
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

        let current_version = String::from_utf8(version_output?.stdout)?;
        let supported = String::from_utf8(help_output?.stdout)?.contains(ACP_ARG);

        if supported {
            Ok(AgentServerVersion::Supported)
        } else {
            Ok(AgentServerVersion::Unsupported {
                error_message: format!(
                    "Your installed version of Codex {} doesn't support the Agentic Coding Protocol (ACP).",
                    current_version
                ).into(),
                upgrade_message: "Upgrade Codex to Latest".into(),
                upgrade_command: "npm install -g @openai/codex@latest".into(),
            })
        }
    }
}
