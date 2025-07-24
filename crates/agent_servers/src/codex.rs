use project::Project;
use std::{path::Path, rc::Rc};

use anyhow::Result;
use gpui::{App, Entity, Task};

use crate::AgentServer;
use acp_thread::AgentConnection;

#[derive(Clone)]
pub struct Codex;

impl AgentServer for Codex {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn empty_state_headline(&self) -> &'static str {
        self.name()
    }

    fn empty_state_message(&self) -> &'static str {
        ""
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    fn connect(
        &self,
        _root_dir: &Path,
        _project: &Entity<Project>,
        _cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        // re-implement using ACP
        todo!()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::AgentServerCommand;

    use super::*;

    crate::common_e2e_tests!(Codex);

    pub fn local_command() -> AgentServerCommand {
        let cli_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../codex/code-rs/target/debug/codex");

        AgentServerCommand {
            path: cli_path,
            args: vec!["mcp".into()],
            env: None,
        }
    }
}
