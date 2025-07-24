use agent_client_protocol as acp;
use context_server::types::requests;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use project::Project;
use serde::Serialize;
use settings::SettingsStore;
use std::rc::Rc;
use std::{path::Path, sync::Arc};
use util::ResultExt;

use anyhow::{Context, Result};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task};

use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings};
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
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let project = project.clone();
        cx.spawn(async move |cx| {
            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).codex.clone()
            })?;

            let Some(command) =
                AgentServerCommand::resolve("codex", &["mcp"], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find codex binary");
            };

            let client: Arc<ContextServer> = ContextServer::stdio(
                ContextServerId("codex-mcp-server".into()),
                ContextServerCommand {
                    path: command.path,
                    args: command.args,
                    env: command.env,
                },
            )
            .into();
            ContextServer::start(client.clone(), cx).await?;

            let connection = CodexConnection { client };
            Ok(Rc::new(connection) as _)
        })
    }
}

struct CodexConnection {
    client: Arc<context_server::ContextServer>,
}

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
        let client = self.client.client();
        let cwd = cwd.to_path_buf();
        cx.spawn(async move |cx| {
            let client = client.context("MCP server is not initialized yet")?;

            let response = client
                .request::<requests::CallTool>(context_server::types::CallToolParams {
                    name: acp::NEW_SESSION_TOOL_NAME.into(),
                    arguments: Some(serde_json::to_value(acp::NewSessionToolArguments {
                        mcp_servers: Default::default(),
                        client_tools: acp::ClientTools {
                            confirm_permission: None,
                            write_text_file: None,
                            read_text_file: None,
                        },
                        cwd,
                    })?),
                    meta: None,
                })
                .await?;

            let result = serde_json::from_value::<acp::NewSessionToolResult>(
                response.structured_content.context("Empty response")?,
            )?;

            let thread =
                cx.new(|cx| AcpThread::new(self.clone(), project, result.session_id, cx))?;

            Ok(thread)
        })
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

impl Drop for CodexConnection {
    fn drop(&mut self) {
        self.client.stop().log_err();
    }
}
