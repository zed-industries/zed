use crate::{assistant_panel::Conversation, LanguageModelRequestMessage, Role};
use gpui::{ModelContext, Subscription, Task, WeakModel};
use language::{Buffer, BufferSnapshot, Rope};
use std::{fmt::Write, path::PathBuf, time::Duration};

use super::ContextUpdated;

pub struct RecentBuffersContext {
    pub enabled: bool,
    pub buffers: Vec<RecentBuffer>,
    pub snapshot: RecentBuffersSnapshot,
    pub pending_message: Option<Task<()>>,
}

pub struct RecentBuffer {
    pub buffer: WeakModel<Buffer>,
    pub _subscription: Subscription,
}

impl Default for RecentBuffersContext {
    fn default() -> Self {
        Self {
            enabled: true,
            buffers: Vec::new(),
            snapshot: RecentBuffersSnapshot::default(),
            pending_message: None,
        }
    }
}

impl RecentBuffersContext {
    pub fn update(&mut self, cx: &mut ModelContext<Conversation>) -> ContextUpdated {
        let source_buffers = self
            .buffers
            .iter()
            .filter_map(|recent| {
                let (full_path, snapshot) = recent
                    .buffer
                    .read_with(cx, |buffer, cx| {
                        (
                            buffer.file().map(|file| file.full_path(cx)),
                            buffer.snapshot(),
                        )
                    })
                    .ok()?;
                Some(SourceBufferSnapshot {
                    full_path,
                    model: recent.buffer.clone(),
                    snapshot,
                })
            })
            .collect::<Vec<_>>();

        if !self.enabled || source_buffers.is_empty() {
            self.snapshot.message = Default::default();
            self.snapshot.source_buffers.clear();
            self.pending_message = None;
            cx.notify();
            ContextUpdated::Disabled
        } else {
            self.pending_message = Some(cx.spawn(|this, mut cx| async move {
                const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(100);
                cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;

                let message = if source_buffers.is_empty() {
                    Rope::new()
                } else {
                    cx.background_executor()
                        .spawn({
                            let source_buffers = source_buffers.clone();
                            async move { message_for_recent_buffers(source_buffers) }
                        })
                        .await
                };
                this.update(&mut cx, |this, cx| {
                    this.ambient_context.recent_buffers.snapshot.source_buffers = source_buffers;
                    this.ambient_context.recent_buffers.snapshot.message = message;
                    this.count_remaining_tokens(cx);
                    cx.notify();
                })
                .ok();
            }));

            ContextUpdated::Updating
        }
    }

    /// Returns the [`RecentBuffersContext`] as a message to the language model.
    pub fn to_message(&self) -> Option<LanguageModelRequestMessage> {
        self.enabled.then(|| LanguageModelRequestMessage {
            role: Role::System,
            content: self.snapshot.message.to_string(),
        })
    }
}

#[derive(Clone, Default, Debug)]
pub struct RecentBuffersSnapshot {
    pub message: Rope,
    pub source_buffers: Vec<SourceBufferSnapshot>,
}

#[derive(Clone)]
pub struct SourceBufferSnapshot {
    pub full_path: Option<PathBuf>,
    pub model: WeakModel<Buffer>,
    pub snapshot: BufferSnapshot,
}

impl std::fmt::Debug for SourceBufferSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceBufferSnapshot")
            .field("full_path", &self.full_path)
            .field("model (entity id)", &self.model.entity_id())
            .field("snapshot (text)", &self.snapshot.text())
            .finish()
    }
}

fn message_for_recent_buffers(buffers: Vec<SourceBufferSnapshot>) -> Rope {
    let mut message = String::new();
    writeln!(
        message,
        "The following is a list of recent buffers that the user has opened."
    )
    .unwrap();

    for buffer in buffers {
        if let Some(path) = buffer.full_path {
            writeln!(message, "```{}", path.display()).unwrap();
        } else {
            writeln!(message, "```untitled").unwrap();
        }

        for chunk in buffer.snapshot.chunks(0..buffer.snapshot.len(), false) {
            message.push_str(chunk.text);
        }
        if !message.ends_with('\n') {
            message.push('\n');
        }
        message.push_str("```\n");
    }

    Rope::from(message.as_str())
}
