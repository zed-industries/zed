use std::path::Path;
use std::rc::Rc;

use agent_servers::AgentServer;
use anyhow::Result;
use gpui::{App, AppContext, Entity, Task};
use project::Project;

use crate::{templates::Templates, NativeAgent, NativeAgentConnection};

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
        _project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn acp_thread::AgentConnection>>> {
        log::info!(
            "NativeAgentServer::connect called for path: {:?}",
            _root_dir
        );
        cx.spawn(async move |cx| {
            log::debug!("Creating templates for native agent");
            // Create templates (you might want to load these from files or resources)
            let templates = Templates::new();

            // Create the native agent
            log::debug!("Creating native agent entity");
            let agent = cx.update(|cx| cx.new(|_| NativeAgent::new(templates)))?;

            // Create the connection wrapper
            let connection = NativeAgentConnection(agent);
            log::info!("NativeAgentServer connection established successfully");

            Ok(Rc::new(connection) as Rc<dyn acp_thread::AgentConnection>)
        })
    }
}
