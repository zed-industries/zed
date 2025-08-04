use std::path::Path;
use std::rc::Rc;

use crate::{
    AgentServer, AgentServerCommand, acp_connection::AcpConnection,
    old_acp_support::OldAcpAgentConnection,
};
use acp_thread::AgentConnection;
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

            // todo! check supported version
            Ok(if true {
                let conn: Rc<dyn AgentConnection> =
                    Rc::new(OldAcpAgentConnection::new(server_name, command, &root_dir, cx).await?);
                conn
            } else {
                let conn: Rc<dyn AgentConnection> =
                    Rc::new(AcpConnection::stdio(server_name, command, &root_dir, cx).await?);
                conn
            })
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(Gemini, allow_option_id = "0");

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
