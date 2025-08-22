use std::rc::Rc;
use std::{any::Any, path::Path};

use crate::{AgentServer, AgentServerCommand};
use acp_thread::{AgentConnection, LoadError};
use anyhow::Result;
use gpui::{Entity, Task};
use language_models::provider::google::GoogleLanguageModelProvider;
use project::Project;
use settings::SettingsStore;
use ui::App;

use crate::AllAgentServersSettings;

#[derive(Clone)]
pub struct Gemini;

const ACP_ARG: &str = "--experimental-acp";

impl AgentServer for Gemini {
    fn name(&self) -> &'static str {
        "Gemini CLI"
    }

    fn empty_state_headline(&self) -> &'static str {
        self.name()
    }

    fn empty_state_message(&self) -> &'static str {
        "Ask questions, edit files, run commands"
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

            let Some(mut command) =
                AgentServerCommand::resolve("gemini", &[ACP_ARG], None, settings, &project, cx).await
            else {
                return Err(LoadError::NotInstalled {
                    error_message: "Failed to find Gemini CLI binary".into(),
                    install_message: "Install Gemini CLI".into(),
                    install_command: "npm install -g @google/gemini-cli@preview".into()
                }.into());
            };

            if let Some(api_key)= cx.update(GoogleLanguageModelProvider::api_key)?.await.ok() {
                command.env.get_or_insert_default().insert("GEMINI_API_KEY".to_owned(), api_key.key);
            }

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
                            "Your installed version of Gemini CLI ({}, version {}) doesn't support the Agentic Coding Protocol (ACP).",
                            command.path.to_string_lossy(),
                            current_version
                        ).into(),
                        upgrade_message: "Upgrade Gemini CLI to latest".into(),
                        upgrade_command: "npm install -g @google/gemini-cli@preview".into(),
                    }.into())
                }
            }
            result
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(async |_, _, _| Gemini, allow_option_id = "proceed_once");

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
