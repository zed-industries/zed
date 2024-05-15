use gpui::{Model, ModelContext, Subscription, Task, WeakModel};
use language::{Buffer, BufferSnapshot, DiagnosticEntry, Point, Rope};
use std::{fmt::Write, iter, path::PathBuf, time::Duration};

use crate::{assistant_panel::Conversation, LanguageModelRequestMessage, Role};

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
    pub fn toggle(&mut self, cx: &mut ModelContext<Conversation>) {
        self.enabled = !self.enabled;
        self.update(cx);
    }

    pub fn reset(
        &mut self,
        buffers: impl IntoIterator<Item = Model<Buffer>>,
        cx: &mut ModelContext<Conversation>,
    ) {
        self.buffers.clear();
        self.buffers
            .extend(buffers.into_iter().map(|buffer| RecentBuffer {
                buffer: buffer.downgrade(),
                _subscription: cx.observe(&buffer, |this, _, cx| {
                    this.ambient_context.recent_buffers.update(cx);
                }),
            }));
        self.update(cx);
    }

    fn update(&mut self, cx: &mut ModelContext<Conversation>) {
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
    writeln!(
        message,
        "For every line in the buffer, I will include a row number that line corresponds to."
    )
    .unwrap();
    writeln!(
        message,
        "Lines that don't have a number correspond to errors and warnings. For example:"
    )
    .unwrap();
    writeln!(message, "```path/to/file.md").unwrap();
    writeln!(message, "1 The quick brown fox").unwrap();
    writeln!(message, "2 jumps over one active").unwrap();
    writeln!(message, "             --- error: should be 'the'").unwrap();
    writeln!(message, "                 ------ error: should be 'lazy'").unwrap();
    writeln!(message, "3 dog").unwrap();
    writeln!(message, "```").unwrap();

    message.push('\n');
    writeln!(message, "Here's the actual recent buffer list:").unwrap();
    for buffer in buffers {
        if let Some(path) = buffer.full_path {
            writeln!(message, "```{}", path.display()).unwrap();
        } else {
            writeln!(message, "```untitled").unwrap();
        }

        let mut diagnostics = buffer
            .snapshot
            .diagnostics_in_range::<_, Point>(language::Anchor::MIN..language::Anchor::MAX, false)
            .peekable();

        let mut active_diagnostics = Vec::<DiagnosticEntry<Point>>::new();
        const GUTTER_PADDING: usize = 4;
        let gutter_width =
            ((buffer.snapshot.max_point().row + 1) as f32).log10() as usize + 1 + GUTTER_PADDING;
        for buffer_row in 0..=buffer.snapshot.max_point().row {
            let display_row = buffer_row + 1;
            active_diagnostics.retain(|diagnostic| {
                (diagnostic.range.start.row..=diagnostic.range.end.row).contains(&buffer_row)
            });
            while diagnostics.peek().map_or(false, |diagnostic| {
                (diagnostic.range.start.row..=diagnostic.range.end.row).contains(&buffer_row)
            }) {
                active_diagnostics.push(diagnostics.next().unwrap());
            }

            let row_width = (display_row as f32).log10() as usize + 1;
            write!(message, "{}", display_row).unwrap();
            if row_width < gutter_width {
                message.extend(iter::repeat(' ').take(gutter_width - row_width));
            }

            for chunk in buffer.snapshot.text_for_range(
                Point::new(buffer_row, 0)
                    ..Point::new(buffer_row, buffer.snapshot.line_len(buffer_row)),
            ) {
                message.push_str(chunk);
            }
            message.push('\n');

            for diagnostic in &active_diagnostics {
                message.extend(iter::repeat(' ').take(gutter_width));

                let start_column = if diagnostic.range.start.row == buffer_row {
                    message.extend(iter::repeat(' ').take(diagnostic.range.start.column as usize));
                    diagnostic.range.start.column
                } else {
                    0
                };
                let end_column = if diagnostic.range.end.row == buffer_row {
                    diagnostic.range.end.column
                } else {
                    buffer.snapshot.line_len(buffer_row)
                };

                message.extend(iter::repeat('-').take((end_column - start_column) as usize));
                writeln!(message, " {}", diagnostic.diagnostic.message).unwrap();
            }
        }

        message.push('\n');
    }

    writeln!(
        message,
        "When quoting the above code, mention which rows the code occurs at."
    )
    .unwrap();
    writeln!(
        message,
        "Never include rows in the quoted code itself and only report lines that didn't start with a row number."
    )
    .unwrap();

    Rope::from(message.as_str())
}
