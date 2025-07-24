use std::{error::Error, fmt, path::Path, rc::Rc};

use agent_client_protocol as acp;
use agentic_coding_protocol::{self as acp_old, AgentRequest};
use anyhow::Result;
use gpui::{AppContext, AsyncApp, Entity, Task};
use project::Project;
use ui::App;

use crate::AcpThread;

pub trait AgentConnection {
    fn name(&self) -> &'static str;

    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>>;

    fn authenticate(&self, cx: &mut App) -> Task<Result<()>>;

    fn prompt(&self, params: acp::PromptToolArguments, cx: &mut App) -> Task<Result<()>>;

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);
}

#[derive(Debug)]
pub struct Unauthenticated;

impl Error for Unauthenticated {}
impl fmt::Display for Unauthenticated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unauthenticated")
    }
}

pub struct OldAcpAgentConnection {
    pub name: &'static str,
    pub connection: acp_old::AgentConnection,
    pub child_status: Task<Result<()>>,
}

impl AgentConnection for OldAcpAgentConnection {
    fn name(&self) -> &'static str {
        self.name
    }

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
        cx.spawn(async move |cx| {
            let result = task.await?;
            let result = acp_old::InitializeParams::response_from_any(result)?;

            if !result.is_authenticated {
                anyhow::bail!(Unauthenticated)
            }

            cx.update(|cx| {
                let thread = cx.new(|cx| {
                    let session_id = acp::SessionId("acp-old-no-id".into());
                    AcpThread::new(self.clone(), project, session_id, cx)
                });
                thread
            })
        })
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<()>> {
        let task = self
            .connection
            .request_any(acp_old::AuthenticateParams.into_any());
        cx.foreground_executor().spawn(async move {
            task.await?;
            Ok(())
        })
    }

    fn prompt(&self, params: acp::PromptToolArguments, cx: &mut App) -> Task<Result<()>> {
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
