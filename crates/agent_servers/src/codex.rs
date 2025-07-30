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
pub struct Codex;

impl AgentServer for Codex {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Welcome to Codex"
    }

    fn empty_state_message(&self) -> &'static str {
        "What can I help with?"
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    fn connect(
        &self,
        _root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let project = project.clone();
        let server_name = self.name();
        let working_directory = project.read(cx).active_project_directory(cx);
        cx.spawn(async move |cx| {
            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).codex.clone()
            })?;

            let Some(command) =
                AgentServerCommand::resolve("codex", &["mcp"], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find codex binary");
            };
            // todo! check supported version

            let conn = AcpConnection::stdio(server_name, command, working_directory, cx).await?;
            Ok(Rc::new(conn) as _)
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(Codex, allow_option_id = "approve");

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
