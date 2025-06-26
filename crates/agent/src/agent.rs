pub mod agent_profile;
pub mod context;
pub mod context_server_tool;
pub mod context_store;
pub mod history_store;
pub mod thread;
pub mod thread_store;

pub use context::{AgentContext, ContextId, ContextLoadResult};
pub use context_store::ContextStore;
pub use thread::{
    LastRestoreCheckpoint, Message, MessageCrease, MessageId, MessageSegment, ThreadError,
    ThreadEvent, ThreadFeedback, ThreadId, ThreadSummary, TokenUsageRatio, ZedAgent,
};
pub use thread_store::{SerializedThread, TextThreadStore, ThreadStore};

pub fn init(cx: &mut gpui::App) {
    thread_store::init(cx);
}
