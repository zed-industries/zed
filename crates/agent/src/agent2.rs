use anyhow::Result;
use assistant_tool::{Tool, ToolResultOutput};
use futures::{channel::oneshot, future::BoxFuture, stream::BoxStream};
use gpui::SharedString;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AgentThreadId(SharedString);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AgentThreadMessageId(usize);

#[derive(Debug, Clone)]
pub struct AgentThreadToolCallId(SharedString);

pub enum AgentThreadResponseEvent {
    Text(String),
    Thinking(String),
    ToolCallChunk {
        id: AgentThreadToolCallId,
        tool: Arc<dyn Tool>,
        input: serde_json::Value,
    },
    ToolCall {
        id: AgentThreadToolCallId,
        tool: Arc<dyn Tool>,
        input: serde_json::Value,
        response_tx: oneshot::Sender<Result<ToolResultOutput>>,
    },
}

pub enum AgentThreadMessage {
    User {
        id: AgentThreadMessageId,
        chunks: Vec<AgentThreadUserMessageChunk>,
    },
    Assistant {
        id: AgentThreadMessageId,
        chunks: Vec<AgentThreadAssistantMessageChunk>,
    },
}

pub enum AgentThreadUserMessageChunk {
    Text(String),
    // here's where we would put mentions, etc.
}

pub enum AgentThreadAssistantMessageChunk {
    Text(String),
    Thinking(String),
    ToolResult {
        id: AgentThreadToolCallId,
        tool: Arc<dyn Tool>,
        input: serde_json::Value,
        output: Result<ToolResultOutput>,
    },
}

struct AgentThreadResponse {
    user_message_id: AgentThreadMessageId,
    events: BoxStream<'static, Result<AgentThreadResponseEvent>>,
}

pub trait AgentThread {
    fn id(&self) -> AgentThreadId;
    fn title(&self) -> BoxFuture<'static, Result<String>>;
    fn summary(&self) -> BoxFuture<'static, Result<String>>;
    fn messages(&self) -> BoxFuture<'static, Result<Vec<AgentThreadMessage>>>;
    fn truncate(&self, message_id: AgentThreadMessageId) -> BoxFuture<'static, Result<()>>;
    fn edit(
        &self,
        message_id: AgentThreadMessageId,
        content: Vec<AgentThreadUserMessageChunk>,
        max_iterations: usize,
    ) -> BoxFuture<'static, Result<AgentThreadResponse>>;
    fn send(
        &self,
        content: Vec<AgentThreadUserMessageChunk>,
        max_iterations: usize,
    ) -> BoxFuture<'static, Result<AgentThreadResponse>>;
}
