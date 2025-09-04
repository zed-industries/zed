use crate::{AgentServerCommand, AgentServerDelegate};
use acp_thread::AgentConnection;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use std::{path::Path, rc::Rc};
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
        root_dir: &Path,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let server_name = self.name();
        let mut command = self.command.clone();
        let root_dir = root_dir.to_path_buf();

        // Get the project environment variables for the root directory
        let project_env = delegate.project().update(cx, |project, cx| {
            project.directory_environment(root_dir.as_path().into(), cx)
        });

        cx.spawn(async move |cx| {
            // Start with project environment variables (from shell, .env files, etc.)
            let mut env = project_env.await.unwrap_or_default();

            // Merge with any existing command env (command env takes precedence)
            if let Some(command_env) = &command.env {
                env.extend(command_env.clone());
            }

            // Set the merged environment back on the command
            command.env = Some(env);

            crate::acp::connect(server_name, command, &root_dir, cx).await
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn std::any::Any> {
        self
    }
}
