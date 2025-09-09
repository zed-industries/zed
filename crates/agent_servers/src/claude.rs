use std::path::Path;
use std::rc::Rc;
use std::{any::Any, path::PathBuf};

use anyhow::{Context as _, Result};
use gpui::{App, SharedString, Task};
use project::agent_server_store::CLAUDE_CODE_NAME;

use crate::{AgentServer, AgentServerDelegate};
use acp_thread::AgentConnection;

#[derive(Clone)]
pub struct ClaudeCode;

pub struct AgentServerLoginCommand {
    pub path: PathBuf,
    pub arguments: Vec<String>,
}

impl AgentServer for ClaudeCode {
    fn telemetry_id(&self) -> &'static str {
        "claude-code"
    }

    fn name(&self) -> SharedString {
        "Claude Code".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiClaude
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
        let store = delegate.store.downgrade();

        // Get the project environment variables for the root directory
        let project_env = delegate.project().update(cx, |project, cx| {
            if let Some(path) = project.active_project_directory(cx) {
                Some(project.directory_environment(path, cx))
            } else {
                None
            }
        });

        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&CLAUDE_CODE_NAME.into())
                        .context("Claude Code is not registered")?;
                    anyhow::Ok(agent.get_command(
                        root_dir.as_deref(),
                        Default::default(),
                        delegate.status_tx,
                        delegate.new_version_available,
                        &mut cx.to_async(),
                    ))
                })??
                .await?;
            let connection =
                crate::acp::connect(name, command, root_dir.as_ref(), is_remote, cx).await?;
            Ok((connection, login))
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
