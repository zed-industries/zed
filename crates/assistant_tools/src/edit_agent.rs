mod edit_parser;

use crate::{Template, Templates};
use anyhow::{Result, anyhow};
use assistant_tool::ActionLog;
use edit_parser::EditParser;
use futures::{Stream, StreamExt, stream};
use gpui::{AsyncApp, Entity};
use language::{Anchor, Bias, Buffer, BufferSnapshot};
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, LanguageModelToolResult,
    MessageContent, Role,
};
use serde::Serialize;
use smallvec::SmallVec;
use std::{ops::Range, path::PathBuf, sync::Arc};

#[derive(Serialize)]
pub struct EditAgentTemplate {
    path: Option<PathBuf>,
    edit_description: String,
}

impl Template for EditAgentTemplate {
    const TEMPLATE_NAME: &'static str = "edit_agent.hbs";
}

pub struct EditAgent {
    model: Arc<dyn LanguageModel>,
    action_log: Entity<ActionLog>,
    templates: Arc<Templates>,
}

impl EditAgent {
    pub fn new(
        model: Arc<dyn LanguageModel>,
        action_log: Entity<ActionLog>,
        templates: Arc<Templates>,
    ) -> Self {
        EditAgent {
            model,
            action_log,
            templates,
        }
    }

    pub async fn edit(
        &self,
        buffer: Entity<Buffer>,
        edit_description: String,
        previous_messages: Vec<LanguageModelRequestMessage>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let edits = self
            .stream_edits(buffer.clone(), edit_description, previous_messages, cx)
            .await?;
        self.apply_edits(buffer, edits, cx).await?;
        Ok(())
    }

    async fn apply_edits(
        &self,
        buffer: Entity<Buffer>,
        edits: impl Stream<Item = Result<(Range<Anchor>, String)>>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        // todo!("group all edits into one transaction")
        // todo!("add tests for this")

        // Ensure the buffer is tracked by the action log.
        self.action_log
            .update(cx, |log, cx| log.track_buffer(buffer.clone(), cx))?;

        futures::pin_mut!(edits);
        while let Some(edit) = edits.next().await {
            let (range, content) = edit?;
            // Edit the buffer and report the edit as part of the same effect cycle, otherwise
            // the edit will be reported as if the user made it.
            cx.update(|cx| {
                buffer.update(cx, |buffer, cx| buffer.edit([(range, content)], None, cx));
                self.action_log
                    .update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx))
            })?;
        }

        Ok(())
    }

    async fn stream_edits(
        &self,
        buffer: Entity<Buffer>,
        edit_description: String,
        mut messages: Vec<LanguageModelRequestMessage>,
        cx: &mut AsyncApp,
    ) -> Result<impl use<> + Stream<Item = Result<(Range<Anchor>, String)>>> {
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
        let path = cx.update(|cx| snapshot.resolve_file_path(cx, true))?;
        let prompt = EditAgentTemplate {
            path,
            edit_description,
        }
        .render(&self.templates)?;

        let mut message_content = Vec::new();
        if let Some(last_message) = messages.last() {
            if last_message.role == Role::Assistant {
                for content in &last_message.content {
                    if let MessageContent::ToolUse(tool) = content {
                        message_content.push(MessageContent::ToolResult(LanguageModelToolResult {
                            tool_use_id: tool.id.clone(),
                            tool_name: tool.name.clone(),
                            is_error: false,
                            content: "In progress...".into(), // todo!("what can we do here?")
                        }));
                    }
                }
            }
        }
        message_content.push(MessageContent::Text(prompt));
        messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: message_content,
            cache: false,
        });

        let request = LanguageModelRequest {
            messages,
            ..Default::default()
        };
        let mut parser = EditParser::new();
        let stream = self.model.stream_completion_text(request, cx).await?.stream;
        Ok(stream.flat_map(move |chunk| {
            let mut edits = SmallVec::new();
            let mut error = None;
            let snapshot = snapshot.clone();
            match chunk {
                Ok(chunk) => {
                    print!("{}", chunk);
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                    edits = parser.push(&chunk);
                }
                Err(err) => {
                    error = Some(Err(anyhow!(err)));
                }
            }
            stream::iter(
                edits
                    .into_iter()
                    .map(move |edit| {
                        let range = Self::resolve_location(&snapshot, &edit.old_text);
                        Ok((range, edit.new_text))
                    })
                    .chain(error),
            )
        }))
    }

    fn resolve_location(buffer: &BufferSnapshot, search_query: &str) -> Range<Anchor> {
        const INSERTION_COST: u32 = 3;
        const DELETION_COST: u32 = 10;
        const WHITESPACE_INSERTION_COST: u32 = 1;
        const WHITESPACE_DELETION_COST: u32 = 1;

        let buffer_len = buffer.len();
        let query_len = search_query.len();
        let mut matrix = SearchMatrix::new(query_len + 1, buffer_len + 1);
        let mut leading_deletion_cost = 0_u32;
        for (row, query_byte) in search_query.bytes().enumerate() {
            let deletion_cost = if query_byte.is_ascii_whitespace() {
                WHITESPACE_DELETION_COST
            } else {
                DELETION_COST
            };

            leading_deletion_cost = leading_deletion_cost.saturating_add(deletion_cost);
            matrix.set(
                row + 1,
                0,
                SearchState::new(leading_deletion_cost, SearchDirection::Diagonal),
            );

            for (col, buffer_byte) in buffer.bytes_in_range(0..buffer.len()).flatten().enumerate() {
                let insertion_cost = if buffer_byte.is_ascii_whitespace() {
                    WHITESPACE_INSERTION_COST
                } else {
                    INSERTION_COST
                };

                let up = SearchState::new(
                    matrix.get(row, col + 1).cost.saturating_add(deletion_cost),
                    SearchDirection::Up,
                );
                let left = SearchState::new(
                    matrix.get(row + 1, col).cost.saturating_add(insertion_cost),
                    SearchDirection::Left,
                );
                let diagonal = SearchState::new(
                    if query_byte == *buffer_byte {
                        matrix.get(row, col).cost
                    } else {
                        matrix
                            .get(row, col)
                            .cost
                            .saturating_add(deletion_cost + insertion_cost)
                    },
                    SearchDirection::Diagonal,
                );
                matrix.set(row + 1, col + 1, up.min(left).min(diagonal));
            }
        }

        // Traceback to find the best match
        let mut best_buffer_end = buffer_len;
        let mut best_cost = u32::MAX;
        for col in 1..=buffer_len {
            let cost = matrix.get(query_len, col).cost;
            if cost < best_cost {
                best_cost = cost;
                best_buffer_end = col;
            }
        }

        let mut query_ix = query_len;
        let mut buffer_ix = best_buffer_end;
        while query_ix > 0 && buffer_ix > 0 {
            let current = matrix.get(query_ix, buffer_ix);
            match current.direction {
                SearchDirection::Diagonal => {
                    query_ix -= 1;
                    buffer_ix -= 1;
                }
                SearchDirection::Up => {
                    query_ix -= 1;
                }
                SearchDirection::Left => {
                    buffer_ix -= 1;
                }
            }
        }

        let mut start = buffer.offset_to_point(buffer.clip_offset(buffer_ix, Bias::Left));
        start.column = 0;
        let mut end = buffer.offset_to_point(buffer.clip_offset(best_buffer_end, Bias::Right));
        if end.column > 0 {
            end.column = buffer.line_len(end.row);
        }

        buffer.anchor_after(start)..buffer.anchor_before(end)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchDirection {
    Up,
    Left,
    Diagonal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    cost: u32,
    direction: SearchDirection,
}

impl SearchState {
    fn new(cost: u32, direction: SearchDirection) -> Self {
        Self { cost, direction }
    }
}

struct SearchMatrix {
    cols: usize,
    data: Vec<SearchState>,
}

impl SearchMatrix {
    fn new(rows: usize, cols: usize) -> Self {
        SearchMatrix {
            cols,
            data: vec![SearchState::new(0, SearchDirection::Diagonal); rows * cols],
        }
    }

    fn get(&self, row: usize, col: usize) -> SearchState {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, cost: SearchState) {
        self.data[row * self.cols + col] = cost;
    }
}
