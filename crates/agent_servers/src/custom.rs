use crate::{AgentServerCommand, AgentServerSettings};
use acp_thread::AgentConnection;
use anyhow::Result;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use std::{path::Path, rc::Rc};
use ui::IconName;

/// A generic agent server implementation for custom user-defined agents
pub struct CustomAgentServer {
    name: SharedString,
    command: AgentServerCommand,
}

impl CustomAgentServer {
    pub fn new(name: SharedString, settings: &AgentServerSettings) -> Self {
        Self {
            name,
            command: settings.command.clone(),
        }
    }
}

impl crate::AgentServer for CustomAgentServer {
    fn telemetry_id(&self) -> &'static str {
        "custom"
    }

    fn name(&self) -> SharedString {
        self.name.clone()
    }

    fn logo(&self) -> IconName {
        IconName::Terminal
    }

    fn empty_state_headline(&self) -> SharedString {
        "No conversations yet".into()
    }

    fn empty_state_message(&self) -> SharedString {
        format!("Start a conversation with {}", self.name).into()
    }

    fn connect(
        &self,
        root_dir: &Path,
        _project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let server_name = self.name();
        let command = self.command.clone();
        let root_dir = root_dir.to_path_buf();

        cx.spawn(async move |mut cx| {
            crate::acp::connect(server_name, command, &root_dir, &mut cx).await
        })
    }

    fn install_command(&self) -> Option<&'static str> {
        None
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn std::any::Any> {
        self
    }
}
