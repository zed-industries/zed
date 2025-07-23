use agent_client_protocol as acp;
use agentic_coding_protocol::{self as acp_old, AgentRequest};
use anyhow::Result;
use futures::future::{FutureExt as _, LocalBoxFuture};

pub trait AgentConnection {
    fn new_session(
        &self,
        params: acp::NewSessionToolArguments,
    ) -> LocalBoxFuture<'static, Result<acp::SessionId>>;

    fn authenticate(&self) -> LocalBoxFuture<'static, Result<()>>;

    fn prompt(&self, params: acp::PromptToolArguments) -> LocalBoxFuture<'static, Result<()>>;

    fn cancel(&self) -> LocalBoxFuture<'static, Result<()>>;
}

impl AgentConnection for acp_old::AgentConnection {
    fn new_session(
        &self,
        _params: acp::NewSessionToolArguments,
    ) -> LocalBoxFuture<'static, Result<acp::SessionId>> {
        let task = self.request_any(
            acp_old::InitializeParams {
                protocol_version: acp_old::ProtocolVersion::latest(),
            }
            .into_any(),
        );
        async move {
            let result = task.await?;
            let result = acp_old::InitializeParams::response_from_any(result)?;

            if !result.is_authenticated {
                anyhow::bail!("Not authenticated");
            }

            Ok(acp::SessionId("acp-old-no-id".into()))
        }
        .boxed_local()
    }

    fn authenticate(&self) -> LocalBoxFuture<'static, Result<()>> {
        let task = self.request_any(acp_old::AuthenticateParams.into_any());
        async move {
            task.await?;
            anyhow::Ok(())
        }
        .boxed_local()
    }

    fn prompt(&self, params: acp::PromptToolArguments) -> LocalBoxFuture<'static, Result<()>> {
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

        let task = self.request_any(acp_old::SendUserMessageParams { chunks }.into_any());
        async move {
            task.await?;
            anyhow::Ok(())
        }
        .boxed_local()
    }

    fn cancel(&self) -> LocalBoxFuture<'static, Result<()>> {
        let task = self.request_any(acp_old::CancelSendMessageParams.into_any());
        async move {
            task.await?;
            anyhow::Ok(())
        }
        .boxed_local()
    }
}
