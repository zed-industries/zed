use std::rc::Rc;
use std::{any::Any, path::Path};

use crate::{AgentServer, AgentServerDelegate};
use acp_thread::AgentConnection;
use anyhow::Result;
use gpui::{App, SharedString, Task};

#[derive(Clone)]
pub struct GooseAcp;

impl AgentServer for GooseAcp {
    fn telemetry_id(&self) -> &'static str {
        "goose-acp"
    }

    fn name(&self) -> SharedString {
        "Goose ACP".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::Terminal
    }

    fn connect(
        &self,
        root_dir: Option<&Path>,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
        let name = self.name();
        let root_dir = root_dir.map(|root_dir| root_dir.to_string_lossy().to_string());
        let is_remote = delegate.project.read(cx).is_via_remote_server();
        let default_mode = self.default_mode(cx);

        cx.spawn(async move |cx| {
            // Create a command to run "goose acp"
            let command = project::agent_server_store::AgentServerCommand {
                path: "goose".into(),
                args: vec!["acp".to_string()],
                env: None,
            };

            let root_dir = std::path::PathBuf::from(
                root_dir.as_deref().unwrap_or(".")
            );

            let connection = crate::acp::connect(
                name,
                command,
                &root_dir,
                default_mode,
                is_remote,
                cx,
            )
            .await?;
            Ok((connection, None))
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use project::agent_server_store::AgentServerCommand;

    use super::*;
    use std::path::Path;

    pub fn local_command() -> AgentServerCommand {
        AgentServerCommand {
            path: "goose".into(),
            args: vec!["acp".to_string()],
            env: None,
        }
    }
}
