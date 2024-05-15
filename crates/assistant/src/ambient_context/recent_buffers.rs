use std::fmt::Write;
use std::iter;
use std::path::PathBuf;
use std::time::Duration;

use gpui::{ModelContext, Subscription, Task, WeakModel};
use language::{Buffer, BufferSnapshot, DiagnosticEntry, Point};

use crate::ambient_context::ContextUpdated;
use crate::assistant_panel::Conversation;
use crate::{LanguageModelRequestMessage, Role};

pub struct RecentBuffersContext {
    pub enabled: bool,
    pub buffers: Vec<RecentBuffer>,
    pub message: String,
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
            message: String::new(),
            pending_message: None,
        }
    }
}

impl RecentBuffersContext {
    /// Returns the [`RecentBuffersContext`] as a message to the language model.
    pub fn to_message(&self) -> Option<LanguageModelRequestMessage> {
        self.enabled.then(|| LanguageModelRequestMessage {
            role: Role::System,
            content: self.message.clone(),
        })
    }

    pub fn update(&mut self, cx: &mut ModelContext<Conversation>) -> ContextUpdated {
        let buffers = self
            .buffers
            .iter()
            .filter_map(|recent| {
                recent
                    .buffer
                    .read_with(cx, |buffer, cx| {
                        (
                            buffer.file().map(|file| file.full_path(cx)),
                            buffer.snapshot(),
                        )
                    })
                    .ok()
            })
            .collect::<Vec<_>>();

        if !self.enabled || buffers.is_empty() {
            self.message.clear();
            self.pending_message = None;
            cx.notify();
            ContextUpdated::Disabled
        } else {
            self.pending_message = Some(cx.spawn(|this, mut cx| async move {
                const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(100);
                cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;

                let message = cx
                    .background_executor()
                    .spawn(async move { Self::build_message(&buffers) })
                    .await;
                this.update(&mut cx, |conversation, cx| {
                    conversation.ambient_context.recent_buffers.message = message;
                    conversation.count_remaining_tokens(cx);
                    cx.notify();
                })
                .ok();
            }));

            ContextUpdated::Updating
        }
    }

    fn build_message(buffers: &[(Option<PathBuf>, BufferSnapshot)]) -> String {
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
        writeln!(message, "path/to/file.md").unwrap();
        writeln!(message, "```markdown").unwrap();
        writeln!(message, "1 The quick brown fox").unwrap();
        writeln!(message, "2 jumps over one active").unwrap();
        writeln!(message, "             --- error: should be 'the'").unwrap();
        writeln!(message, "                 ------ error: should be 'lazy'").unwrap();
        writeln!(message, "3 dog").unwrap();
        writeln!(message, "```").unwrap();

        message.push('\n');
        writeln!(message, "Here's the actual recent buffer list:").unwrap();
        for (path, buffer) in buffers {
            if let Some(path) = path {
                writeln!(message, "{}", path.display()).unwrap();
            } else {
                writeln!(message, "untitled").unwrap();
            }

            if let Some(language) = buffer.language() {
                writeln!(message, "```{}", language.name().to_lowercase()).unwrap();
            } else {
                writeln!(message, "```").unwrap();
            }

            let mut diagnostics = buffer
                .diagnostics_in_range::<_, Point>(
                    language::Anchor::MIN..language::Anchor::MAX,
                    false,
                )
                .peekable();

            let mut active_diagnostics = Vec::<DiagnosticEntry<Point>>::new();
            const GUTTER_PADDING: usize = 4;
            let gutter_width =
                ((buffer.max_point().row + 1) as f32).log10() as usize + 1 + GUTTER_PADDING;
            for buffer_row in 0..=buffer.max_point().row {
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

                for chunk in buffer.text_for_range(
                    Point::new(buffer_row, 0)..Point::new(buffer_row, buffer.line_len(buffer_row)),
                ) {
                    message.push_str(chunk);
                }
                message.push('\n');

                for diagnostic in &active_diagnostics {
                    message.extend(iter::repeat(' ').take(gutter_width));

                    let start_column = if diagnostic.range.start.row == buffer_row {
                        message
                            .extend(iter::repeat(' ').take(diagnostic.range.start.column as usize));
                        diagnostic.range.start.column
                    } else {
                        0
                    };
                    let end_column = if diagnostic.range.end.row == buffer_row {
                        diagnostic.range.end.column
                    } else {
                        buffer.line_len(buffer_row)
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

        message
    }
}
