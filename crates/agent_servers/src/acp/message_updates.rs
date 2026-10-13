use acp_thread::{AcpThread, MessageKind};
use agent_client_protocol::schema::v2 as acp_v2;
use anyhow::Result;
use gpui::Context;

#[derive(Debug)]
pub struct MessageNotification {
    pub session_id: acp_v2::SessionId,
    pub update: MessageUpdate,
    pub meta: Option<acp_v2::Meta>,
}

#[derive(Debug)]
pub enum MessageUpdate {
    UserMessage(acp_v2::UserMessage),
    AgentMessage(acp_v2::AgentMessage),
    AgentThought(acp_v2::AgentThought),
    UserMessageChunk(acp_v2::ContentChunk),
    AgentMessageChunk(acp_v2::ContentChunk),
    AgentThoughtChunk(acp_v2::ContentChunk),
}

impl TryFrom<acp_v2::UpdateSessionNotification> for MessageNotification {
    type Error = acp_v2::UpdateSessionNotification;

    fn try_from(mut notification: acp_v2::UpdateSessionNotification) -> Result<Self, Self::Error> {
        let update = match notification.update {
            acp_v2::SessionUpdate::UserMessage(message) => MessageUpdate::UserMessage(message),
            acp_v2::SessionUpdate::AgentMessage(message) => MessageUpdate::AgentMessage(message),
            acp_v2::SessionUpdate::AgentThought(thought) => MessageUpdate::AgentThought(thought),
            acp_v2::SessionUpdate::UserMessageChunk(chunk) => {
                MessageUpdate::UserMessageChunk(chunk)
            }
            acp_v2::SessionUpdate::AgentMessageChunk(chunk) => {
                MessageUpdate::AgentMessageChunk(chunk)
            }
            acp_v2::SessionUpdate::AgentThoughtChunk(chunk) => {
                MessageUpdate::AgentThoughtChunk(chunk)
            }
            update => {
                notification.update = update;
                return Err(notification);
            }
        };
        Ok(Self {
            session_id: notification.session_id,
            update,
            meta: notification.meta,
        })
    }
}

impl MessageNotification {
    pub fn apply(self, thread: &mut AcpThread, cx: &mut Context<AcpThread>) -> Result<()> {
        let Self {
            session_id,
            update,
            meta: _delivery_metadata,
        } = self;
        anyhow::ensure!(
            &session_id == thread.session_id(),
            "Message notification belongs to a different ACP session"
        );
        // Delivery metadata must not patch the message or its content items.
        match update {
            MessageUpdate::UserMessage(message) => thread.upsert_user_message(message, cx),
            MessageUpdate::AgentMessage(message) => thread.upsert_assistant_message(message, cx),
            MessageUpdate::AgentThought(thought) => thread.upsert_thought(thought, cx),
            MessageUpdate::UserMessageChunk(chunk) => {
                thread.append_message_chunk(MessageKind::User, chunk, cx)
            }
            MessageUpdate::AgentMessageChunk(chunk) => {
                thread.append_message_chunk(MessageKind::Assistant, chunk, cx)
            }
            MessageUpdate::AgentThoughtChunk(chunk) => {
                thread.append_message_chunk(MessageKind::Thought, chunk, cx)
            }
        }
    }
}
