use acp_thread::{AcpThread, MessageKind};
use agent_client_protocol::schema::v2 as acp_v2;
use anyhow::Result;
use collections::HashMap;
use gpui::Context;
use std::collections::VecDeque;

use super::terminal_updates::{DecodedTerminalNotification, DecodedTerminalUpdate};

pub(super) const MAX_PENDING_SESSION_UPDATES: usize = 1024;
const MAX_PENDING_SESSION_UPDATE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Default)]
pub(super) struct PendingSessionUpdates {
    generation: u64,
    creations: HashMap<acp_v2::RequestId, PendingCreation>,
    updates: VecDeque<PendingUpdate>,
    encoded_bytes: usize,
}

struct PendingCreation {
    generation: u64,
    overflowed: bool,
}

struct PendingUpdate {
    generation: u64,
    encoded_bytes: usize,
    notification: InboundSessionNotification,
}

impl PendingSessionUpdates {
    #[cfg(test)]
    pub(super) fn counts(&self) -> (usize, usize, usize) {
        (self.creations.len(), self.updates.len(), self.encoded_bytes)
    }

    pub(super) fn begin(&mut self, request_id: acp_v2::RequestId) {
        let generation = self.generation.checked_add(1);
        if let Some(generation) = generation {
            self.generation = generation;
        }
        self.creations.insert(
            request_id,
            PendingCreation {
                generation: self.generation,
                overflowed: generation.is_none(),
            },
        );
    }

    pub(super) fn stage(
        &mut self,
        notification: InboundSessionNotification,
        encoded_bytes: usize,
    ) -> Option<InboundSessionNotification> {
        if !self.creations.values().any(|creation| !creation.overflowed) {
            return Some(notification);
        }
        if self.updates.len() >= MAX_PENDING_SESSION_UPDATES
            || encoded_bytes > MAX_PENDING_SESSION_UPDATE_BYTES.saturating_sub(self.encoded_bytes)
        {
            for creation in self.creations.values_mut() {
                creation.overflowed = true;
            }
            self.updates.clear();
            self.encoded_bytes = 0;
            log::warn!("ACP pre-response session update staging exceeded its limits");
            return None;
        }
        self.encoded_bytes += encoded_bytes;
        self.updates.push_back(PendingUpdate {
            generation: self.generation,
            encoded_bytes,
            notification,
        });
        None
    }

    pub(super) fn take(
        &mut self,
        request_id: &acp_v2::RequestId,
        session_id: &acp_v2::SessionId,
    ) -> Result<Vec<InboundSessionNotification>> {
        let creation = self
            .creations
            .remove(request_id)
            .ok_or_else(|| anyhow::anyhow!("Session creation is no longer pending"))?;
        if creation.overflowed {
            self.prune();
            anyhow::bail!("ACP pre-response session update staging exceeded its limits");
        }
        let mut notifications = Vec::new();
        // Session IDs correlate concurrent creations. The wire carries no request ID
        // on these notifications, so provisional ID reuse by the agent remains ambiguous.
        for update in std::mem::take(&mut self.updates) {
            if &update.notification.session_id == session_id
                && update.generation >= creation.generation
            {
                notifications.push(update.notification);
            } else {
                self.updates.push_back(update);
            }
        }
        self.prune();
        Ok(notifications)
    }

    pub(super) fn finish(&mut self, request_id: &acp_v2::RequestId) {
        self.creations.remove(request_id);
        self.prune();
    }

    pub(super) fn discard_session(&mut self, session_id: &acp_v2::SessionId) {
        self.updates
            .retain(|update| &update.notification.session_id != session_id);
        self.prune();
    }

    pub(super) fn clear(&mut self) {
        self.creations.clear();
        self.updates.clear();
        self.encoded_bytes = 0;
    }

    fn prune(&mut self) {
        let oldest = self
            .creations
            .values()
            .filter(|creation| !creation.overflowed)
            .map(|creation| creation.generation)
            .min();
        let mut encoded_bytes = 0;
        self.updates.retain(|update| {
            let keep = oldest.is_some_and(|oldest| update.generation >= oldest);
            if keep {
                encoded_bytes += update.encoded_bytes;
            }
            keep
        });
        self.encoded_bytes = encoded_bytes;
    }
}

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
    SessionMessage(acp_v2::SessionMessage),
    SessionMessageChunk(acp_v2::SessionMessageChunk),
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
    /// Creates or patches a subagent session, so the connection applies it rather than
    /// the parent thread.
    Subagent(acp_v2::SubagentUpdate),
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
            acp_v2::SessionUpdate::SessionMessage(message) => {
                InboundSessionUpdate::SessionMessage(message)
            }
            acp_v2::SessionUpdate::SessionMessageChunk(chunk) => {
                InboundSessionUpdate::SessionMessageChunk(chunk)
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
            acp_v2::SessionUpdate::SubagentUpdate(update) => InboundSessionUpdate::Subagent(update),
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
            InboundSessionUpdate::SessionMessage(message) => {
                thread.upsert_session_message(message, cx)
            }
            InboundSessionUpdate::SessionMessageChunk(chunk) => {
                thread.append_session_message_chunk(chunk, cx)
            }
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
            InboundSessionUpdate::Subagent(_) => {
                anyhow::bail!("Subagent updates must be applied by the ACP connection")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn request_id(value: i64) -> acp_v2::RequestId {
        serde_json::from_value(json!(value)).expect("request ID")
    }

    fn notification(session_id: &str, text: &str) -> InboundSessionNotification {
        acp_v2::UpdateSessionNotification::new(
            session_id,
            acp_v2::SessionUpdate::AgentMessage(
                acp_v2::AgentMessage::new(text).content(vec![text.into()]),
            ),
        )
        .try_into()
        .expect("supported notification")
    }

    #[test]
    fn later_creation_does_not_adopt_an_older_prefix() {
        let mut pending = PendingSessionUpdates::default();
        pending.begin(request_id(1));
        assert!(pending.stage(notification("reused", "old"), 10).is_none());
        pending.begin(request_id(2));
        assert!(
            pending
                .take(&request_id(2), &"reused".into())
                .expect("new creation")
                .is_empty()
        );
        pending.finish(&request_id(1));
        assert_eq!(pending.counts(), (0, 0, 0));
        assert!(
            pending
                .stage(notification("unknown", "no owner"), 10)
                .is_some()
        );
    }

    #[test]
    fn byte_limit_is_bounded_and_does_not_poison_later_creations() {
        let wire = acp_v2::UpdateSessionNotification::new(
            "session",
            acp_v2::SessionUpdate::AgentMessage(
                acp_v2::AgentMessage::new("message")
                    .content(vec!["unicode é and \"quotes\"".into()]),
            ),
        );
        assert_eq!(
            super::super::notification_size(&wire).expect("counted"),
            serde_json::to_vec(&wire).expect("serialized").len()
        );
        let mut pending = PendingSessionUpdates::default();
        pending.begin(request_id(1));
        assert!(
            pending
                .stage(
                    notification("first", "entry"),
                    MAX_PENDING_SESSION_UPDATE_BYTES
                )
                .is_none()
        );
        assert!(
            pending
                .stage(notification("first", "overflow"), 1)
                .is_none()
        );
        pending.begin(request_id(2));
        assert!(pending.stage(notification("second", "fresh"), 3).is_none());
        assert!(pending.take(&request_id(1), &"first".into()).is_err());
        assert_eq!(
            pending
                .take(&request_id(2), &"second".into())
                .expect("fresh creation")
                .len(),
            1
        );
        assert_eq!(pending.counts(), (0, 0, 0));
    }
}
