//! `Thread`: one OMP session as a message list with streaming assembly.
//!
//! Fed by [`RouterEvent`]s from the shared `EventRouter` (plan §72: never
//! mutate UI from the reader thread — the owner pumps here, GPUI reads
//! snapshots). Streaming text accumulates per `message_id`; `MessageEnd`
//! freezes it. Crash-during-stream (§74 R3): partial buffers are DISCARDED on
//! disconnect, never rendered as complete; `interrupted_tool_calls` marks the
//! orphaned tool cards.

use cedian_omp::{DeltaKind, PromptStatus, RouterEvent};
use std::collections::HashMap;

/// One entry in the thread: user prompt, assistant message (streaming or
/// complete), or tool card anchor.
#[derive(Debug, Clone)]
pub enum ThreadEvent {
    /// Local user prompt (echoed on send; OMP confirms via transcript).
    User { text: String },
    /// Assistant message: `streaming=true` until `MessageEnd`.
    Assistant {
        message_id: String,
        text: String,
        thinking: String,
        streaming: bool,
    },
    /// Tool execution card: status flips `Running → Done | Error | Interrupted`.
    /// `preview` (args one-liner) fills at start; `summary` (result one-liner)
    /// at end. Both human text, never raw JSON (Phase 3 acceptance).
    Tool {
        call_id: String,
        name: String,
        status: ToolCallStatus,
        preview: String,
        summary: String,
    },
    /// Queue chips snapshot (steer/follow-up), rendered under the composer.
    Queue {
        steering: Vec<String>,
        follow_up: Vec<String>,
    },
    /// Turn boundary marker (yielded + terminal flags for settle display).
    TurnEnd { yielded: bool, is_terminal: bool },
}

/// Lifecycle of one tool card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallStatus {
    Running,
    Done,
    Error,
    /// `tool.started` without `tool.completed` at disconnect (§74 R3).
    Interrupted,
}

/// Per-message streaming assembly.
#[derive(Debug, Default, Clone)]
struct StreamingMessage {
    text: String,
    thinking: String,
}

/// One OMP session as an ordered event list. All mutation is `apply` of
/// router classifications — deterministic, unit-testable, no I/O.
#[derive(Debug, Default)]
pub struct Thread {
    events: Vec<ThreadEvent>,
    streaming: HashMap<String, StreamingMessage>,
    tools: HashMap<String, usize>,
}

impl Thread {
    /// Empty thread.
    pub fn new() -> Self {
        Self::default()
    }

    /// Local echo of a user prompt (before OMP admission).
    pub fn push_user(&mut self, text: &str) {
        self.events.push(ThreadEvent::User {
            text: text.to_string(),
        });
    }

    /// Take back the newest user prompt: OMP never ran it.
    pub fn withdraw_user(&mut self) {
        if let Some(n) = self
            .events
            .iter()
            .rposition(|e| matches!(e, ThreadEvent::User { .. }))
        {
            self.events.remove(n);
        }
    }

    /// Apply one router classification. Pure state transition.
    pub fn apply(&mut self, event: &RouterEvent) {
        match event {
            RouterEvent::AgentStart => {}
            RouterEvent::AgentEnd {
                yielded,
                is_terminal,
            } => {
                self.events.push(ThreadEvent::TurnEnd {
                    yielded: *yielded,
                    is_terminal: *is_terminal,
                });
            }
            RouterEvent::MessageDelta {
                message_id,
                kind,
                delta,
            } => {
                let buf = self.streaming.entry(message_id.clone()).or_default();
                match kind {
                    DeltaKind::Text => buf.text.push_str(delta),
                    DeltaKind::Thinking => buf.thinking.push_str(delta),
                    _ => {}
                }
                self.upsert_assistant(message_id);
            }
            RouterEvent::MessageEnd { message_id } => {
                self.streaming.remove(message_id);
                // Freeze: mark the assistant entry complete.
                for entry in self.events.iter_mut().rev() {
                    if let ThreadEvent::Assistant {
                        message_id: id,
                        streaming,
                        ..
                    } = entry
                    {
                        if *id == *message_id {
                            *streaming = false;
                            break;
                        }
                    }
                }
            }
            RouterEvent::ToolStart {
                tool_call_id,
                tool_name,
                args_preview,
                ..
            } => {
                let index = self.events.len();
                self.events.push(ThreadEvent::Tool {
                    call_id: tool_call_id.clone(),
                    name: tool_name.clone(),
                    status: ToolCallStatus::Running,
                    preview: args_preview.clone(),
                    summary: String::new(),
                });
                self.tools.insert(tool_call_id.clone(), index);
            }
            RouterEvent::ToolEnd {
                tool_call_id,
                result_summary,
                is_error,
                ..
            } => {
                if let Some(&index) = self.tools.get(tool_call_id) {
                    if let Some(ThreadEvent::Tool {
                        status, summary, ..
                    }) = self.events.get_mut(index)
                    {
                        *status = if *is_error {
                            ToolCallStatus::Error
                        } else {
                            ToolCallStatus::Done
                        };
                        *summary = result_summary.clone();
                    }
                }
            }
            RouterEvent::PromptResult { status, .. } => {
                if *status == PromptStatus::Error {
                    for entry in self.events.iter_mut().rev() {
                        if let ThreadEvent::Tool { status: s, .. } = entry {
                            if *s == ToolCallStatus::Running {
                                *s = ToolCallStatus::Error;
                            }
                        }
                    }
                }
            }
            RouterEvent::Settled => {}
            RouterEvent::Queue {
                steering,
                follow_up,
            } => {
                self.events.push(ThreadEvent::Queue {
                    steering: steering.clone(),
                    follow_up: follow_up.clone(),
                });
            }
            RouterEvent::UiRequest(_)
            | RouterEvent::Toast(_)
            | RouterEvent::ModelChanged
            | RouterEvent::RunNote { .. }
            | RouterEvent::ThinkingLevel(_)
            | RouterEvent::SessionInfo { .. }
            | RouterEvent::Unknown { .. }
            | RouterEvent::SubagentLifecycle { .. }
            | RouterEvent::SubagentProgress { .. } => {
                // Dialogs render outside the thread, subagents in their own
                // tree (`cedian_agent_ui::subagents`); unknown frames are
                // tolerated. None is a thread entry.
            }
            RouterEvent::Disconnected => {
                let running: Vec<String> = self
                    .events
                    .iter()
                    .filter_map(|e| match e {
                        ThreadEvent::Tool {
                            call_id,
                            status: ToolCallStatus::Running,
                            ..
                        } => Some(call_id.clone()),
                        _ => None,
                    })
                    .collect();
                self.on_disconnect(&running);
            }
        }
    }

    /// Mark orphaned tool cards `Interrupted` after a disconnect, and drop
    /// partial streaming buffers (never render half a message as complete).
    #[allow(clippy::collapsible_match)]
    pub fn on_disconnect(&mut self, interrupted: &[String]) {
        self.streaming.clear();
        for entry in self.events.iter_mut() {
            match entry {
                ThreadEvent::Assistant { streaming, .. } => *streaming = false,
                // Nested if by necessity: guard-match would move `call_id`/`name`
                // out of the borrowed entry; rebuilding the event loses `name`.
                ThreadEvent::Tool {
                    call_id, status, ..
                } => {
                    if *status == ToolCallStatus::Running && interrupted.contains(call_id) {
                        *status = ToolCallStatus::Interrupted;
                    }
                }
                _ => {}
            }
        }
    }

    /// Ordered snapshot for rendering.
    pub fn events(&self) -> &[ThreadEvent] {
        &self.events
    }

    /// Live streaming text for one message (for the delta buffer).
    pub fn streaming_text(&self, message_id: &str) -> Option<&str> {
        self.streaming.get(message_id).map(|b| b.text.as_str())
    }

    fn upsert_assistant(&mut self, message_id: &str) {
        let buf = self.streaming.get(message_id).cloned().unwrap_or_default();
        for entry in self.events.iter_mut().rev() {
            if let ThreadEvent::Assistant {
                message_id: id,
                text,
                thinking,
                streaming,
            } = entry
            {
                if *id == *message_id {
                    *text = buf.text.clone();
                    *thinking = buf.thinking;
                    *streaming = true;
                    return;
                }
            }
        }
        self.events.push(ThreadEvent::Assistant {
            message_id: message_id.to_string(),
            text: buf.text,
            thinking: buf.thinking,
            streaming: true,
        });
    }
}

/// Tool card transition helper (exported for review-phase reuse).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallState {
    Running,
    Done,
    Error,
    Interrupted,
}

impl From<ToolCallStatus> for ToolCallState {
    fn from(s: ToolCallStatus) -> Self {
        match s {
            ToolCallStatus::Running => Self::Running,
            ToolCallStatus::Done => Self::Done,
            ToolCallStatus::Error => Self::Error,
            ToolCallStatus::Interrupted => Self::Interrupted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delta(id: &str, kind: DeltaKind) -> RouterEvent {
        RouterEvent::MessageDelta {
            message_id: id.to_string(),
            kind,
            delta: "ab".to_string(),
        }
    }

    #[test]
    fn stream_assembles_then_freezes() {
        let mut t = Thread::new();
        t.apply(&delta("m1", DeltaKind::Text));
        t.apply(&delta("m1", DeltaKind::Text));
        assert_eq!(t.streaming_text("m1"), Some("abab"));
        assert!(matches!(
            &t.events()[0],
            ThreadEvent::Assistant {
                streaming: true,
                ..
            }
        ));
        t.apply(&RouterEvent::MessageEnd {
            message_id: "m1".to_string(),
        });
        assert!(matches!(
            &t.events()[0],
            ThreadEvent::Assistant {
                streaming: false,
                ..
            }
        ));
        assert!(t.streaming_text("m1").is_none());
    }

    #[test]
    fn tool_lifecycle_running_to_done() {
        let mut t = Thread::new();
        t.apply(&RouterEvent::ToolStart {
            tool_call_id: "c1".to_string(),
            tool_name: "read".to_string(),
            args_preview: "src/main.rs".to_string(),
            paths: Vec::new(),
        });
        t.apply(&RouterEvent::ToolEnd {
            tool_call_id: "c1".to_string(),
            tool_name: "read".to_string(),
            result_summary: "42 lines".to_string(),
            is_error: false,
            before: Vec::new(),
        });
        assert!(matches!(
            &t.events()[0],
            ThreadEvent::Tool {
                status: ToolCallStatus::Done,
                ..
            }
        ));
    }

    #[test]
    fn disconnect_interrupts_orphans_and_drops_partial() {
        let mut t = Thread::new();
        t.apply(&delta("m1", DeltaKind::Text));
        t.apply(&RouterEvent::ToolStart {
            tool_call_id: "c1".to_string(),
            tool_name: "bash".to_string(),
            args_preview: "sleep 1".to_string(),
            paths: Vec::new(),
        });
        t.on_disconnect(&["c1".to_string()]);
        assert!(t.streaming_text("m1").is_none());
        assert!(matches!(
            &t.events()[1],
            ThreadEvent::Tool {
                status: ToolCallStatus::Interrupted,
                ..
            }
        ));
    }
    #[test]
    fn omp_dying_interrupts_running_cards_and_ends_streaming() {
        let mut t = Thread::new();
        t.apply(&RouterEvent::ToolStart {
            tool_call_id: "c1".to_string(),
            tool_name: "bash".to_string(),
            args_preview: "sleep 9".to_string(),
            paths: Vec::new(),
        });
        t.apply(&RouterEvent::Disconnected);
        assert!(matches!(
            &t.events()[0],
            ThreadEvent::Tool {
                status: ToolCallStatus::Interrupted,
                ..
            }
        ));
    }

    #[test]
    fn error_end_marks_card_error() {
        let mut t = Thread::new();
        t.apply(&RouterEvent::ToolStart {
            tool_call_id: "c9".to_string(),
            tool_name: "bash".to_string(),
            args_preview: "exit 1".to_string(),
            paths: Vec::new(),
        });
        t.apply(&RouterEvent::ToolEnd {
            tool_call_id: "c9".to_string(),
            tool_name: "bash".to_string(),
            result_summary: "command failed".to_string(),
            is_error: true,
            before: Vec::new(),
        });
        match &t.events()[0] {
            ThreadEvent::Tool {
                status,
                preview,
                summary,
                ..
            } => {
                assert_eq!(*status, ToolCallStatus::Error);
                assert_eq!(preview, "exit 1");
                assert_eq!(summary, "command failed");
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
