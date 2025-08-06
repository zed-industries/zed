use std::path::Path;
use std::rc::Rc;

use crate::{AgentServer, AgentServerCommand};
use acp_thread::{AgentConnection, LoadError};
use anyhow::Result;
use gpui::{Entity, Task};
use project::Project;
use settings::SettingsStore;
use ui::App;

use crate::AllAgentServersSettings;

#[derive(Clone)]
pub struct Gemini;

const ACP_ARG: &str = "--experimental-acp";

impl AgentServer for Gemini {
    fn name(&self) -> &'static str {
        "Gemini"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Welcome to Gemini"
    }

    fn empty_state_message(&self) -> &'static str {
        "Ask questions, edit files, run commands.\nBe specific for the best results."
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiGemini
    }

    fn connect(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let project = project.clone();
        let root_dir = root_dir.to_path_buf();
        let server_name = self.name();
        cx.spawn(async move |cx| {
            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).gemini.clone()
            })?;

            let Some(command) =
                AgentServerCommand::resolve("gemini", &[ACP_ARG], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find gemini binary");
            };

            let result = crate::acp::connect(server_name, command.clone(), &root_dir, cx).await;
            if result.is_err() {
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

                if !supported {
                    return Err(LoadError::Unsupported {
                        error_message: format!(
                            "Your installed version of Gemini {} doesn't support the Agentic Coding Protocol (ACP).",
                            current_version
                        ).into(),
                        upgrade_message: "Upgrade Gemini to Latest".into(),
                        upgrade_command: "npm install -g @google/gemini-cli@latest".into(),
                    }.into())
                }
            }
            result
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(Gemini, allow_option_id = "proceed_once");

    pub fn local_command() -> AgentServerCommand {
        let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../gemini-cli/packages/cli")
            .to_string_lossy()
            .to_string();

        AgentServerCommand {
            path: "node".into(),
            args: vec![cli_path],
            env: None,
        }
    }
}
