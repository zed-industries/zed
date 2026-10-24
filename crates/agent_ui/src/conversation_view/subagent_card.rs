use acp_thread::{ToolCall, ToolCallContent, ToolCallStatus};
use gpui::{App, SharedString};
use std::ops::Range;

/// Native subagent tool calls report a user cancellation as output with this text.
const USER_CANCELED_OUTPUT: &str = "User canceled";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SubagentStatus {
    Running,
    Completed,
    Failed,
    Canceled,
}

impl SubagentStatus {
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
    /// The parent's label for the subagent. The card prefers the subagent thread's own title.
    pub label: Option<SharedString>,
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

    pub(crate) fn title(&self, thread_title: Option<SharedString>) -> SharedString {
        thread_title
            .or_else(|| self.label.clone())
            .unwrap_or_else(|| match self.status {
                SubagentStatus::Canceled => "Subagent Canceled".into(),
                SubagentStatus::Failed => "Subagent Failed".into(),
                SubagentStatus::Running | SubagentStatus::Completed => "Spawning Agent…".into(),
            })
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
