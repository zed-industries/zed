use anyhow::Result;
use assistant_tool::{Tool, ToolResultOutput};
use futures::{channel::oneshot, future::BoxFuture, stream::BoxStream};
use gpui::SharedString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    sync::Arc,
};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
pub struct ThreadId(SharedString);

impl ThreadId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<&str> for ThreadId {
    fn from(value: &str) -> Self {
        ThreadId(SharedString::from(value.to_string()))
    }
}

impl Display for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub usize);

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
        id: MessageId,
        chunks: Vec<AgentThreadUserMessageChunk>,
    },
    Assistant {
        id: MessageId,
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

pub struct AgentThreadResponse {
    pub user_message_id: MessageId,
    pub events: BoxStream<'static, Result<AgentThreadResponseEvent>>,
}

pub trait Agent {
    fn create_thread();
    fn list_threads();
    fn load_thread();
}

pub trait AgentThread {
    fn id(&self) -> ThreadId;
    fn title(&self) -> BoxFuture<'static, Result<String>>;
    fn summary(&self) -> BoxFuture<'static, Result<String>>;
    fn messages(&self) -> BoxFuture<'static, Result<Vec<AgentThreadMessage>>>;
    fn truncate(&self, message_id: MessageId) -> BoxFuture<'static, Result<()>>;
    fn edit(
        &self,
        message_id: MessageId,
        content: Vec<AgentThreadUserMessageChunk>,
        max_iterations: usize,
    ) -> BoxFuture<'static, Result<AgentThreadResponse>>;
    fn send(
        &self,
        content: Vec<AgentThreadUserMessageChunk>,
        max_iterations: usize,
    ) -> BoxFuture<'static, Result<AgentThreadResponse>>;
}
