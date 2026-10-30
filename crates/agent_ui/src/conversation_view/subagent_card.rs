use acp_thread::{
    AcpThread, AgentThreadEntry, MessageIdentity, SessionMessage, SessionMessageSide, ToolCall,
    ToolCallContent, ToolCallStatus,
};
use agent_client_protocol::schema::v2 as acp_v2;
use gpui::{App, Entity, SharedString};
use std::{fmt, ops::Range};

use super::thread_view::subagent_label;

/// Native subagent tool calls report a user cancellation as output with this text.
const USER_CANCELED_OUTPUT: &str = "User canceled";

/// The transcript entry a subagent card stands for. It keys the card's expansion state
/// and element IDs, since the same subagent can have more than one card.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SubagentCardId {
    /// A native `spawn_agent` tool call.
    ToolCall(acp_v2::ToolCallId),
    /// An ACP parent's outgoing session message to the subagent.
    SessionMessage(acp_v2::MessageId),
}

impl fmt::Display for SubagentCardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolCall(id) => write!(f, "tool-call-{id}"),
            Self::SessionMessage(id) => write!(f, "session-message-{id}"),
        }
    }
}

/// A session message that an ACP parent sent to one of its own subagents, drawn as a
/// subagent card in place of its message row.
pub(crate) struct AcpSubagentCard {
    pub id: SubagentCardId,
    pub subagent: Entity<AcpThread>,
    /// The subagent entries for this message; `None` covers the whole subagent.
    pub entries: Option<SubagentEntries>,
}

/// The subagent card that a session message in `thread` is drawn as, if any.
///
/// A message linked to the subagent's view of it gets a card for its part of the
/// subagent's transcript: from that view up to the next message from `thread`. Without a
/// link, only the first message to a subagent gets a card, which covers the whole subagent,
/// and later ones stay message rows.
pub(crate) fn acp_subagent_card(
    thread: &AcpThread,
    entry_ix: usize,
    message: &SessionMessage,
    cx: &App,
) -> Option<AcpSubagentCard> {
    let own_session_id = thread.session_id();
    let SessionMessageSide::Outgoing {
        recipient: Some(recipient),
    } = message.side(own_session_id)
    else {
        return None;
    };
    let subagent = thread.subagent(recipient)?.clone();
    let MessageIdentity::Keyed(message_id) = &message.identity else {
        return None;
    };
    let id = SubagentCardId::SessionMessage(message_id.clone());

    let subagent_thread = subagent.read(cx);
    if let Some(start) = subagent_thread.linked_session_message(message, own_session_id) {
        let next_message_ix = subagent_thread
            .entries()
            .iter()
            .enumerate()
            .skip(start + 1)
            .find_map(|(ix, entry)| {
                matches!(
                    entry,
                    AgentThreadEntry::SessionMessage(later)
                        if later.side(recipient)
                            == SessionMessageSide::Incoming { sender: Some(own_session_id) }
                )
                .then_some(ix)
            });
        return Some(AcpSubagentCard {
            id,
            subagent,
            entries: Some(SubagentEntries {
                start,
                end: next_message_ix.map(|next_message_ix| next_message_ix - 1),
            }),
        });
    }

    let sent_earlier = thread.entries().get(..entry_ix)?.iter().any(|entry| {
        matches!(
            entry,
            AgentThreadEntry::SessionMessage(earlier)
                if earlier.side(own_session_id)
                    == SessionMessageSide::Outgoing { recipient: Some(recipient) }
        )
    });
    (!sent_earlier).then_some(AcpSubagentCard {
        id,
        subagent,
        entries: None,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SubagentStatus {
    Running,
    Completed,
    Failed,
    Canceled,
    /// An ACP subagent that has reported no state yet, or reported that its state is unknown.
    Unknown,
}

impl SubagentStatus {
    /// An ACP subagent's status from the state it last reported, with the error to show for a
    /// failure. Idle with no stop reason, or with one this version does not know, is a normal
    /// end, as it is for any ACP session.
    pub(crate) fn from_reported_state(
        state: Option<&acp_v2::StateUpdate>,
    ) -> (Self, Option<SharedString>) {
        let Some(state) = state else {
            return (Self::Unknown, None);
        };
        match state {
            acp_v2::StateUpdate::Running(_) | acp_v2::StateUpdate::RequiresAction(_) => {
                (Self::Running, None)
            }
            acp_v2::StateUpdate::Idle(idle) => match &idle.stop_reason {
                Some(acp_v2::StopReason::Cancelled) => (Self::Canceled, None),
                Some(acp_v2::StopReason::Error(error)) => (
                    Self::Failed,
                    error
                        .error
                        .as_ref()
                        .map(|error| error.message.trim())
                        .filter(|message| !message.is_empty())
                        .map(|message| message.to_string().into()),
                ),
                Some(acp_v2::StopReason::MaxTokens) => (
                    Self::Failed,
                    Some("The subagent reached its token limit.".into()),
                ),
                Some(acp_v2::StopReason::MaxTurnRequests) => (
                    Self::Failed,
                    Some("The subagent reached its limit of model requests.".into()),
                ),
                Some(acp_v2::StopReason::Refusal) => (
                    Self::Failed,
                    Some("The subagent's model refused to continue.".into()),
                ),
                _ => (Self::Completed, None),
            },
            _ => (Self::Unknown, None),
        }
    }

    pub(crate) fn from_tool_call(tool_call: &ToolCall, cx: &App) -> Self {
        match tool_call.status() {
            ToolCallStatus::Pending
            | ToolCallStatus::InProgress
            | ToolCallStatus::WaitingForConfirmation => Self::Running,
            _ if has_user_canceled_output(tool_call, cx) => Self::Canceled,
            ToolCallStatus::Canceled => Self::Canceled,
            ToolCallStatus::Failed | ToolCallStatus::Rejected => Self::Failed,
            ToolCallStatus::Completed => Self::Completed,
        }
    }

    pub(crate) fn is_failed_or_canceled(self) -> bool {
        matches!(self, Self::Failed | Self::Canceled)
    }
}

/// What a subagent card shows, independent of the source it was built from.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SubagentCardState {
    /// The parent's label for the subagent. A native card prefers the subagent thread's own
    /// title; see `label_wins`.
    pub label: Option<SharedString>,
    /// An ACP parent's own label for the subagent describes it in the parent's view, so it
    /// wins over the subagent's title for itself.
    pub label_wins: bool,
    pub status: SubagentStatus,
    pub error: Option<SharedString>,
    pub can_stop: bool,
    /// The subagent entries this card covers; `None` covers all of them.
    pub entries: Option<SubagentEntries>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SubagentEntries {
    pub start: usize,
    /// Inclusive; `None` extends through the latest entry.
    pub end: Option<usize>,
}

impl SubagentCardState {
    /// Builds the card for a native `spawn_agent` tool call. Each call covers the
    /// subagent entries produced for that one message.
    pub(crate) fn from_tool_call(tool_call: &ToolCall, cx: &App) -> Self {
        let status = SubagentStatus::from_tool_call(tool_call, cx);
        let label = tool_call.label.read(cx).source();
        Self {
            label: (!label.is_empty()).then(|| label.to_string().into()),
            label_wins: false,
            status,
            error: tool_call_error(tool_call, cx),
            can_stop: status == SubagentStatus::Running,
            entries: tool_call
                .subagent_session_info
                .as_ref()
                .map(|info| SubagentEntries {
                    start: info.message_start_index,
                    end: info.message_end_index,
                }),
        }
    }

    /// Builds the card for a subagent tool call. Once the call has finished, a subagent that
    /// recorded an outcome for the call's run (as native subagents do) shows that outcome.
    /// Calls saved before outcomes were recorded, and subagents of agents that don't report
    /// state, show the tool call's status.
    pub(crate) fn for_subagent_tool_call(
        tool_call: &ToolCall,
        subagent: Option<&AcpThread>,
        cx: &App,
    ) -> Self {
        let card = Self::from_tool_call(tool_call, cx);
        let Some(SubagentEntries {
            start,
            end: Some(_),
        }) = card.entries
        else {
            return card;
        };
        let Some(outcome) = subagent
            .filter(|subagent| subagent.subagent_association().is_some())
            .and_then(|subagent| subagent.subagent_outcome_after(start))
        else {
            return card;
        };
        let (status, error) = SubagentStatus::from_reported_state(Some(outcome));
        Self {
            status,
            error,
            can_stop: false,
            ..card
        }
    }

    /// Builds the card for an ACP subagent, covering `entries` of its transcript or all of
    /// it. The subagent reports one state for all its work, so a card for an earlier message
    /// shows the first outcome the subagent reported after receiving it. Until then, that
    /// work continues alongside the later message's, and the card shows the current state.
    pub(crate) fn from_acp_subagent(thread: &AcpThread, entries: Option<SubagentEntries>) -> Self {
        let earlier_outcome = entries
            .filter(|entries| entries.end.is_some())
            .and_then(|entries| thread.subagent_outcome_after(entries.start));
        let (status, error) =
            SubagentStatus::from_reported_state(earlier_outcome.or(thread.foreground_state()));
        let can_cancel = thread
            .subagent_association()
            .is_some_and(|association| association.can_cancel);
        Self {
            label: Some(subagent_label(thread)),
            label_wins: true,
            status,
            error,
            can_stop: status == SubagentStatus::Running && can_cancel,
            entries,
        }
    }

    pub(crate) fn title(&self, thread_title: Option<SharedString>) -> SharedString {
        let label = self.label.clone();
        let title = if self.label_wins {
            label.or(thread_title)
        } else {
            thread_title.or(label)
        };
        title.unwrap_or_else(|| match self.status {
            SubagentStatus::Canceled => "Subagent Canceled".into(),
            SubagentStatus::Failed => "Subagent Failed".into(),
            SubagentStatus::Running | SubagentStatus::Completed | SubagentStatus::Unknown => {
                "Spawning Agent…".into()
            }
        })
    }

    /// The subagent entry that full screen should open at: the start of this card's part
    /// once its work has stopped, or `None` while it runs, so the subagent's view keeps
    /// following its latest entries.
    pub(crate) fn fullscreen_entry(&self) -> Option<usize> {
        (self.status != SubagentStatus::Running)
            .then(|| self.entries.map_or(0, |entries| entries.start))
    }

    /// The last `max_entries` of the entries this card covers.
    pub(crate) fn preview_range(&self, total_entries: usize, max_entries: usize) -> Range<usize> {
        let mut range = match self.entries {
            Some(entries) => {
                entries.start
                    ..entries
                        .end
                        .map(|end| (end + 1).min(total_entries))
                        .unwrap_or(total_entries)
            }
            None => 0..total_entries,
        };
        range.start = range.end.saturating_sub(max_entries).max(range.start);
        range
    }
}

fn has_user_canceled_output(tool_call: &ToolCall, cx: &App) -> bool {
    tool_call.content().iter().any(|content| match content {
        ToolCallContent::ContentBlock { block, .. } => {
            block.text_content(cx) == Some(USER_CANCELED_OUTPUT)
        }
        _ => false,
    })
}

fn tool_call_error(tool_call: &ToolCall, cx: &App) -> Option<SharedString> {
    if tool_call.status() != ToolCallStatus::Failed {
        return None;
    }
    tool_call.content().iter().find_map(|content| {
        let ToolCallContent::ContentBlock { block, .. } = content else {
            return None;
        };
        block
            .text_content(cx)
            .filter(|text| !text.is_empty() && *text != USER_CANCELED_OUTPUT)
            .map(SharedString::from)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(
        label: Option<&str>,
        status: SubagentStatus,
        entries: Option<SubagentEntries>,
    ) -> SubagentCardState {
        SubagentCardState {
            label: label.map(|label| label.to_string().into()),
            label_wins: false,
            status,
            error: None,
            can_stop: false,
            entries,
        }
    }

    #[test]
    fn title_prefers_the_thread_title_then_the_label_then_the_status() {
        let labeled = card(Some("Explore"), SubagentStatus::Running, None);
        assert_eq!(
            labeled.title(Some("Fix the parser".into())).as_ref(),
            "Fix the parser"
        );
        assert_eq!(labeled.title(None).as_ref(), "Explore");
        for (status, title) in [
            (SubagentStatus::Running, "Spawning Agent…"),
            (SubagentStatus::Completed, "Spawning Agent…"),
            (SubagentStatus::Failed, "Subagent Failed"),
            (SubagentStatus::Canceled, "Subagent Canceled"),
        ] {
            assert_eq!(card(None, status, None).title(None).as_ref(), title);
        }
    }

    #[test]
    fn an_acp_parents_label_wins_over_the_subagents_title() {
        let acp_card = SubagentCardState {
            label_wins: true,
            ..card(Some("Investigator"), SubagentStatus::Running, None)
        };
        assert_eq!(
            acp_card.title(Some("Own title".into())).as_ref(),
            "Investigator"
        );
        let unlabeled = SubagentCardState {
            label_wins: true,
            ..card(None, SubagentStatus::Running, None)
        };
        assert_eq!(
            unlabeled.title(Some("Own title".into())).as_ref(),
            "Own title"
        );
    }

    #[test]
    fn reported_states_map_to_card_statuses() {
        let idle = |stop_reason: Option<acp_v2::StopReason>| {
            acp_v2::StateUpdate::Idle(acp_v2::IdleStateUpdate::new().stop_reason(stop_reason))
        };
        let mut error = acp_v2::Error::internal_error();
        error.message = "Context too small".into();
        for (state, expected_status, expected_error) in [
            (None, SubagentStatus::Unknown, None),
            (
                Some(acp_v2::StateUpdate::Running(
                    acp_v2::RunningStateUpdate::new(),
                )),
                SubagentStatus::Running,
                None,
            ),
            (
                Some(acp_v2::StateUpdate::RequiresAction(
                    acp_v2::RequiresActionStateUpdate::new(),
                )),
                SubagentStatus::Running,
                None,
            ),
            (
                Some(acp_v2::StateUpdate::Unknown(
                    acp_v2::UnknownStateUpdate::new(),
                )),
                SubagentStatus::Unknown,
                None,
            ),
            (Some(idle(None)), SubagentStatus::Completed, None),
            (
                Some(idle(Some(acp_v2::StopReason::EndTurn))),
                SubagentStatus::Completed,
                None,
            ),
            (
                Some(idle(Some(acp_v2::StopReason::Cancelled))),
                SubagentStatus::Canceled,
                None,
            ),
            (
                Some(idle(Some(acp_v2::StopReason::Error(
                    acp_v2::ErrorStopReason::new().error(error),
                )))),
                SubagentStatus::Failed,
                Some("Context too small"),
            ),
            (
                Some(idle(Some(acp_v2::StopReason::Error(
                    acp_v2::ErrorStopReason::new(),
                )))),
                SubagentStatus::Failed,
                None,
            ),
            (
                Some(idle(Some(acp_v2::StopReason::Refusal))),
                SubagentStatus::Failed,
                Some("The subagent's model refused to continue."),
            ),
        ] {
            let (status, error) = SubagentStatus::from_reported_state(state.as_ref());
            assert_eq!(status, expected_status, "{state:?}");
            assert_eq!(error.as_deref(), expected_error, "{state:?}");
        }
    }

    #[test]
    fn fullscreen_opens_a_stopped_card_at_the_start_of_its_part() {
        let part = Some(SubagentEntries {
            start: 4,
            end: Some(6),
        });
        assert_eq!(
            card(None, SubagentStatus::Running, part).fullscreen_entry(),
            None
        );
        for status in [
            SubagentStatus::Completed,
            SubagentStatus::Canceled,
            SubagentStatus::Failed,
            SubagentStatus::Unknown,
        ] {
            assert_eq!(card(None, status, part).fullscreen_entry(), Some(4));
            assert_eq!(card(None, status, None).fullscreen_entry(), Some(0));
        }
    }

    #[test]
    fn preview_range_shows_the_last_entries_the_card_covers() {
        let whole_thread = card(None, SubagentStatus::Running, None);
        assert_eq!(whole_thread.preview_range(20, 8), 12..20);
        assert_eq!(whole_thread.preview_range(3, 8), 0..3);

        let open_ended = card(
            None,
            SubagentStatus::Running,
            Some(SubagentEntries {
                start: 4,
                end: None,
            }),
        );
        assert_eq!(open_ended.preview_range(6, 8), 4..6);
        assert_eq!(open_ended.preview_range(20, 8), 12..20);

        let finished = card(
            None,
            SubagentStatus::Completed,
            Some(SubagentEntries {
                start: 4,
                end: Some(6),
            }),
        );
        assert_eq!(finished.preview_range(20, 8), 4..7);
        assert_eq!(
            finished.preview_range(5, 8),
            4..5,
            "an end past the thread is clamped"
        );
    }
}
