use project::Project;
use settings::SettingsStore;
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use gpui::{App, Entity, Task};

use crate::acp_connection::AcpConnection;
use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings};
use acp_thread::AgentConnection;

#[derive(Clone)]
pub struct Gemini;

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
        _root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let project = project.clone();
        let server_name = self.name();
        cx.spawn(async move |cx| {
            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).gemini.clone()
            })?;

            let Some(command) = AgentServerCommand::resolve(
                "gemini",
                &["--experimental-mcp"],
                settings,
                &project,
                cx,
            )
            .await
            else {
                anyhow::bail!("Failed to find gemini binary");
            };

            let conn = AcpConnection::stdio(server_name, command, cx).await?;
            Ok(Rc::new(conn) as _)
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(Gemini, allow_option_id = "allow");

    pub fn local_command() -> AgentServerCommand {
        let cli_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../gemini/packages/cli");

        AgentServerCommand {
            path: "node".into(),
            args: vec![cli_path.to_string_lossy().to_string()],
            env: None,
        }
    }
}
