// Translates old acp agents into the new schema
use agent_client_protocol as acp;
use agentic_coding_protocol::{self as acp_old, AgentRequest as _};
use anyhow::Result;
use gpui::{AppContext as _, AsyncApp, Entity, Task, WeakEntity};
use project::Project;
use std::{cell::RefCell, path::Path, rc::Rc};
use ui::App;

use acp_thread::{AcpThread, AgentConnection, AuthRequired};

pub struct OldAcpAgentConnection {
    pub connection: acp_old::AgentConnection,
    pub child_status: Task<Result<()>>,
    pub current_thread: Rc<RefCell<WeakEntity<AcpThread>>>,
    pub auth_methods: [acp::AuthMethod; 1],
}

impl AgentConnection for OldAcpAgentConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        _cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>> {
        let task = self.connection.request_any(
            acp_old::InitializeParams {
                protocol_version: acp_old::ProtocolVersion::latest(),
            }
            .into_any(),
        );
        let current_thread = self.current_thread.clone();
        cx.spawn(async move |cx| {
            let result = task.await?;
            let result = acp_old::InitializeParams::response_from_any(result)?;

            if !result.is_authenticated {
                anyhow::bail!(AuthRequired)
            }

            cx.update(|cx| {
                let thread = cx.new(|cx| {
                    let session_id = acp::SessionId("acp-old-no-id".into());
                    AcpThread::new("Gemini", self.clone(), project, session_id, cx)
                });
                current_thread.replace(thread.downgrade());
                thread
            })
        })
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &self.auth_methods
    }

    fn authenticate(&self, _method_id: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
        let task = self
            .connection
            .request_any(acp_old::AuthenticateParams.into_any());
        cx.foreground_executor().spawn(async move {
            task.await?;
            Ok(())
        })
    }

    fn prompt(&self, params: acp::PromptRequest, cx: &mut App) -> Task<Result<()>> {
        let chunks = params
            .prompt
            .into_iter()
            .filter_map(|block| match block {
                acp::ContentBlock::Text(text) => {
                    Some(acp_old::UserMessageChunk::Text { text: text.text })
                }
                acp::ContentBlock::ResourceLink(link) => Some(acp_old::UserMessageChunk::Path {
                    path: link.uri.into(),
                }),
                _ => None,
            })
            .collect();

        let task = self
            .connection
            .request_any(acp_old::SendUserMessageParams { chunks }.into_any());
        cx.foreground_executor().spawn(async move {
            task.await?;
            anyhow::Ok(())
        })
    }

    fn cancel(&self, _session_id: &acp::SessionId, cx: &mut App) {
        let task = self
            .connection
            .request_any(acp_old::CancelSendMessageParams.into_any());
        cx.foreground_executor()
            .spawn(async move {
                task.await?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx)
    }
}
