use agentic_coding_protocol as acp_old;
use anyhow::Result;
use futures::future::{FutureExt as _, LocalBoxFuture};

pub trait AgentConnection {
    fn request_any(
        &self,
        params: acp_old::AnyAgentRequest,
    ) -> LocalBoxFuture<'static, Result<acp_old::AnyAgentResult>>;
}

impl AgentConnection for acp_old::AgentConnection {
    fn request_any(
        &self,
        params: acp_old::AnyAgentRequest,
    ) -> LocalBoxFuture<'static, Result<acp_old::AnyAgentResult>> {
        let task = self.request_any(params);
        async move { Ok(task.await?) }.boxed_local()
    }
}
