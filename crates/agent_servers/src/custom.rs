use crate::{AgentServerCommand, AgentServerDelegate};
use acp_thread::AgentConnection;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use std::rc::Rc;
use ui::IconName;

/// A generic agent server implementation for custom user-defined agents
pub struct CustomAgentServer {
    name: SharedString,
    command: AgentServerCommand,
}

impl CustomAgentServer {
    pub fn new(name: SharedString, command: AgentServerCommand) -> Self {
        Self { name, command }
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

    fn connect(
        &self,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let server_name = self.name();
        let command = self.command.clone();
        let project = delegate.project().clone();
        cx.spawn(async move |cx| crate::acp::connect(server_name, command, &project, cx).await)
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn std::any::Any> {
        self
    }
}
