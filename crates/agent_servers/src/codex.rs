use crate::stdio_agent_server::StdioAgentServer;
use crate::{AgentServerCommand, AgentServerVersion};
use anyhow::{Context as _, Result};
use gpui::{AsyncApp, Entity};
use project::Project;
use settings::SettingsStore;

use crate::AllAgentServersSettings;

#[derive(Clone)]
pub struct Codex;

const ACP_ARG: &str = "experimental-acp";

impl StdioAgentServer for Codex {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Welcome to Codex"
    }

    fn empty_state_message(&self) -> &'static str {
        ""
    }

    fn supports_always_allow(&self) -> bool {
        true
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    async fn command(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<AgentServerCommand> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).codex.clone()
        })?;

        if let Some(command) =
            AgentServerCommand::resolve("codex", &[ACP_ARG], settings, &project, cx).await
        {
            return Ok(command);
        };

        let (fs, node_runtime) = project.update(cx, |project, _| {
            (project.fs().clone(), project.node_runtime().cloned())
        })?;
        let node_runtime = node_runtime.context("codex not found on path")?;

        let directory = ::paths::agent_servers_dir().join("codex");
        fs.create_dir(&directory).await?;
        node_runtime
            .npm_install_packages(&directory, &[("@openai/codex", "latest")])
            .await?;
        let path = directory.join("node_modules/.bin/codex");

        Ok(AgentServerCommand {
            path,
            args: vec![ACP_ARG.into()],
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(Codex);

    pub fn local_command() -> AgentServerCommand {
        let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../codex/codex-rs/target/debug/codex");

        AgentServerCommand {
            path: cli_path,
            args: vec![],
            env: None,
        }
    }
}
