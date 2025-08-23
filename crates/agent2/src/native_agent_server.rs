use std::{any::Any, path::Path, rc::Rc, sync::Arc};

use agent_servers::AgentServer;
use anyhow::Result;
use fs::Fs;
use gpui::{App, Entity, Task};
use project::Project;
use prompt_store::PromptStore;

use crate::{HistoryStore, NativeAgent, NativeAgentConnection, templates::Templates};

#[derive(Clone)]
pub struct NativeAgentServer {
    fs: Arc<dyn Fs>,
    history: Entity<HistoryStore>,
}

impl NativeAgentServer {
    pub fn new(fs: Arc<dyn Fs>, history: Entity<HistoryStore>) -> Self {
        Self { fs, history }
    }
}

impl AgentServer for NativeAgentServer {
    fn name(&self) -> &'static str {
        "Native Agent"
    }

    fn empty_state_headline(&self) -> &'static str {
        ""
    }

    fn empty_state_message(&self) -> &'static str {
        ""
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::ZedAgent
    }

    fn connect(
        &self,
        _root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn acp_thread::AgentConnection>>> {
        log::info!(
            "NativeAgentServer::connect called for path: {:?}",
            _root_dir
        );
        let project = project.clone();
        let fs = self.fs.clone();
        let history = self.history.clone();
        let prompt_store = PromptStore::global(cx);
        cx.spawn(async move |cx| {
            log::debug!("Creating templates for native agent");
            let templates = Templates::new();
            let prompt_store = prompt_store.await?;

            log::debug!("Creating native agent entity");
            let agent =
                NativeAgent::new(project, history, templates, Some(prompt_store), fs, cx).await?;

            // Create the connection wrapper
            let connection = NativeAgentConnection(agent);
            log::info!("NativeAgentServer connection established successfully");

            Ok(Rc::new(connection) as Rc<dyn acp_thread::AgentConnection>)
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
