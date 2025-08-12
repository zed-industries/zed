use std::path::Path;
use std::rc::Rc;

use agent_servers::AgentServer;
use anyhow::Result;
use gpui::{App, Entity, Task};
use project::Project;
use prompt_store::PromptStore;

use crate::{NativeAgent, NativeAgentConnection, templates::Templates};

#[derive(Clone)]
pub struct NativeAgentServer;

impl AgentServer for NativeAgentServer {
    fn name(&self) -> &'static str {
        "Native Agent"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Native Agent"
    }

    fn empty_state_message(&self) -> &'static str {
        "How can I help you today?"
    }

    fn logo(&self) -> ui::IconName {
        // Using the ZedAssistant icon as it's the native built-in agent
        ui::IconName::ZedAssistant
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
        let prompt_store = PromptStore::global(cx);
        cx.spawn(async move |cx| {
            log::debug!("Creating templates for native agent");
            let templates = Templates::new();
            let prompt_store = prompt_store.await?;

            log::debug!("Creating native agent entity");
            let agent = NativeAgent::new(project, templates, Some(prompt_store), cx).await?;

            // Create the connection wrapper
            let connection = NativeAgentConnection(agent);
            log::info!("NativeAgentServer connection established successfully");

            Ok(Rc::new(connection) as Rc<dyn acp_thread::AgentConnection>)
        })
    }
}
