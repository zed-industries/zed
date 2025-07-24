use project::Project;
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use gpui::{App, AsyncApp, Entity, Task};

use crate::AgentServer;
use acp_thread::{AcpThread, AgentConnection};

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
        let connection = CodexConnection;

        Task::ready(Ok(Rc::new(connection) as _))
    }
}

struct CodexConnection;

impl AgentConnection for CodexConnection {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>> {
        todo!()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<()>> {
        todo!()
    }

    fn prompt(
        &self,
        params: agent_client_protocol::PromptToolArguments,
        cx: &mut App,
    ) -> Task<Result<()>> {
        todo!()
    }

    fn cancel(&self, session_id: &agent_client_protocol::SessionId, cx: &mut App) {
        todo!()
    }
}
