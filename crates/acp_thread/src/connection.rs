use agentic_coding_protocol as acp;
use anyhow::Result;
use futures::future::{FutureExt as _, LocalBoxFuture};

pub trait AgentConnection {
    fn request_any(
        &self,
        params: acp::AnyAgentRequest,
    ) -> LocalBoxFuture<'static, Result<acp::AnyAgentResult>>;
}

impl AgentConnection for acp::AgentConnection {
    fn request_any(
        &self,
        params: acp::AnyAgentRequest,
    ) -> LocalBoxFuture<'static, Result<acp::AnyAgentResult>> {
        let task = self.request_any(params);
        async move { Ok(task.await?) }.boxed_local()
    }
}
