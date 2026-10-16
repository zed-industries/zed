use acp_thread::{AcpThread, MessageKind};
use agent_client_protocol::schema::v2 as acp_v2;
use anyhow::Result;
use gpui::Context;

use super::terminal_updates::{DecodedTerminalNotification, DecodedTerminalUpdate};

#[derive(Debug)]
pub struct InboundSessionNotification {
    pub session_id: acp_v2::SessionId,
    pub update: InboundSessionUpdate,
    pub meta: Option<acp_v2::Meta>,
}

#[derive(Debug)]
pub enum InboundSessionUpdate {
    UserMessage(acp_v2::UserMessage),
    AgentMessage(acp_v2::AgentMessage),
    AgentThought(acp_v2::AgentThought),
    UserMessageChunk(acp_v2::ContentChunk),
    AgentMessageChunk(acp_v2::ContentChunk),
    AgentThoughtChunk(acp_v2::ContentChunk),
    ToolCallUpdate(acp_v2::ToolCallUpdate),
    ToolCallContentChunk(acp_v2::ToolCallContentChunk),
    PlanItems {
        plan: acp_v2::PlanItems,
        update_meta: Option<acp_v2::Meta>,
    },
    Terminal(DecodedTerminalUpdate),
    SessionInfo(acp_v2::SessionInfoUpdate),
    AvailableCommands(acp_v2::AvailableCommandsUpdate),
    ConfigOptions(acp_v2::ConfigOptionUpdate),
    Usage(acp_v2::UsageUpdate),
    State(acp_v2::StateUpdate),
    Notice(acp_v2::Notice),
    Compaction(acp_v2::CompactionUpdate),
    CompactionSummary(acp_v2::CompactionSummaryChunk),
}

impl From<DecodedTerminalNotification> for InboundSessionNotification {
    fn from(notification: DecodedTerminalNotification) -> Self {
        Self {
            session_id: notification.session_id,
            update: InboundSessionUpdate::Terminal(notification.update),
            meta: notification.meta,
        }
    }
}

impl TryFrom<acp_v2::UpdateSessionNotification> for InboundSessionNotification {
    type Error = acp_v2::UpdateSessionNotification;

    fn try_from(mut notification: acp_v2::UpdateSessionNotification) -> Result<Self, Self::Error> {
        let update = match notification.update {
            acp_v2::SessionUpdate::UserMessage(message) => {
                InboundSessionUpdate::UserMessage(message)
            }
            acp_v2::SessionUpdate::AgentMessage(message) => {
                InboundSessionUpdate::AgentMessage(message)
            }
            acp_v2::SessionUpdate::AgentThought(thought) => {
                InboundSessionUpdate::AgentThought(thought)
            }
            acp_v2::SessionUpdate::UserMessageChunk(chunk) => {
                InboundSessionUpdate::UserMessageChunk(chunk)
            }
            acp_v2::SessionUpdate::AgentMessageChunk(chunk) => {
                InboundSessionUpdate::AgentMessageChunk(chunk)
            }
            acp_v2::SessionUpdate::AgentThoughtChunk(chunk) => {
                InboundSessionUpdate::AgentThoughtChunk(chunk)
            }
            acp_v2::SessionUpdate::ToolCallUpdate(update) => {
                InboundSessionUpdate::ToolCallUpdate(update)
            }
            acp_v2::SessionUpdate::ToolCallContentChunk(chunk) => {
                InboundSessionUpdate::ToolCallContentChunk(chunk)
            }
            acp_v2::SessionUpdate::PlanUpdate(mut update) => {
                let plan = match update.plan {
                    acp_v2::PlanUpdateContent::Items(plan) => plan,
                    content => {
                        update.plan = content;
                        notification.update = acp_v2::SessionUpdate::PlanUpdate(update);
                        return Err(notification);
                    }
                };
                InboundSessionUpdate::PlanItems {
                    plan,
                    update_meta: update.meta,
                }
            }
            acp_v2::SessionUpdate::SessionInfoUpdate(update) => {
                InboundSessionUpdate::SessionInfo(update)
            }
            acp_v2::SessionUpdate::AvailableCommandsUpdate(update) => {
                InboundSessionUpdate::AvailableCommands(update)
            }
            acp_v2::SessionUpdate::ConfigOptionUpdate(update) => {
                InboundSessionUpdate::ConfigOptions(update)
            }
            acp_v2::SessionUpdate::UsageUpdate(update) => InboundSessionUpdate::Usage(update),
            acp_v2::SessionUpdate::StateUpdate(update) => InboundSessionUpdate::State(update),
            acp_v2::SessionUpdate::Notice(notice) => InboundSessionUpdate::Notice(notice),
            acp_v2::SessionUpdate::CompactionUpdate(update) => {
                InboundSessionUpdate::Compaction(update)
            }
            acp_v2::SessionUpdate::CompactionSummaryChunk(chunk) => {
                InboundSessionUpdate::CompactionSummary(chunk)
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

impl InboundSessionNotification {
    pub fn apply(self, thread: &mut AcpThread, cx: &mut Context<AcpThread>) -> Result<()> {
        let Self {
            session_id,
            update,
            meta: delivery_metadata,
        } = self;
        anyhow::ensure!(
            &session_id == thread.session_id(),
            "Session notification belongs to a different ACP session"
        );
        // Delivery metadata must not patch retained payload fields.
        match update {
            InboundSessionUpdate::UserMessage(message) => thread.upsert_user_message(message, cx),
            InboundSessionUpdate::AgentMessage(message) => {
                thread.upsert_assistant_message(message, cx)
            }
            InboundSessionUpdate::AgentThought(thought) => thread.upsert_thought(thought, cx),
            InboundSessionUpdate::UserMessageChunk(chunk) => {
                thread.append_message_chunk(MessageKind::User, chunk, cx)
            }
            InboundSessionUpdate::AgentMessageChunk(chunk) => {
                thread.append_message_chunk(MessageKind::Assistant, chunk, cx)
            }
            InboundSessionUpdate::AgentThoughtChunk(chunk) => {
                thread.append_message_chunk(MessageKind::Thought, chunk, cx)
            }
            InboundSessionUpdate::ToolCallUpdate(update) => {
                thread.upsert_wire_tool_call(update, cx)
            }
            InboundSessionUpdate::ToolCallContentChunk(chunk) => {
                thread.append_tool_call_content_chunk(chunk, cx)
            }
            InboundSessionUpdate::PlanItems { plan, update_meta } => {
                thread.upsert_plan_items(plan, update_meta, cx);
                Ok(())
            }
            InboundSessionUpdate::Terminal(update) => DecodedTerminalNotification {
                session_id,
                meta: delivery_metadata,
                update,
            }
            .apply(thread, cx),
            InboundSessionUpdate::SessionInfo(update) => {
                thread.update_session_info(update, cx);
                Ok(())
            }
            InboundSessionUpdate::AvailableCommands(update) => {
                thread.update_available_commands(update.available_commands, cx);
                Ok(())
            }
            InboundSessionUpdate::ConfigOptions(update) => {
                cx.emit(acp_thread::AcpThreadEvent::ConfigOptionsUpdated(
                    update.config_options,
                ));
                Ok(())
            }
            InboundSessionUpdate::Usage(update) => {
                thread.update_usage(update, cx);
                Ok(())
            }
            InboundSessionUpdate::State(update) => thread.update_session_state(update, cx),
            InboundSessionUpdate::Notice(notice) => {
                thread.push_notice(notice, cx);
                Ok(())
            }
            InboundSessionUpdate::Compaction(update) => {
                thread.upsert_context_compaction_update(update, cx);
                Ok(())
            }
            InboundSessionUpdate::CompactionSummary(chunk) => {
                thread.append_context_compaction_summary(chunk, cx);
                Ok(())
            }
        }
    }
}
