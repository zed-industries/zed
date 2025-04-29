mod edit_parser;
#[cfg(test)]
mod evals;

use crate::{Template, Templates};
use anyhow::{Context as _, Result};
use assistant_tool::ActionLog;
use edit_parser::EditParser;
use futures::{Stream, StreamExt, stream::BoxStream};
use gpui::{AsyncApp, Entity};
use language::{Bias, Buffer, BufferSnapshot};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolResult, MessageContent, Role,
};
use serde::Serialize;
use smallvec::SmallVec;
use std::{ops::Range, path::PathBuf, sync::Arc};
use streaming_diff::{CharOperation, StreamingDiff};

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
    ) -> Result<String> {
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
        let edit_chunks = self
            .request_edits(snapshot.clone(), edit_description, previous_messages, cx)
            .await?;
        let chunks = self.apply_edits(buffer, snapshot, edit_chunks, cx).await?;
        Ok(chunks)
    }

    async fn apply_edits(
        &self,
        buffer: Entity<Buffer>,
        mut snapshot: BufferSnapshot,
        edit_chunks: impl Stream<Item = Result<String, LanguageModelCompletionError>>,
        cx: &mut AsyncApp,
    ) -> Result<String> {
        struct PendingEdit {
            start: usize,
            diff: StreamingDiff,
        }

        // todo!("group all edits into one transaction")
        // todo!("move to the background")
        // todo!("add tests for this")

        // Ensure the buffer is tracked by the action log.
        self.action_log
            .update(cx, |log, cx| log.track_buffer(buffer.clone(), cx))?;

        let mut parser = EditParser::new();
        let mut pending_edit = None;
        futures::pin_mut!(edit_chunks);
        let mut output = String::new();

        while let Some(edit_chunk) = edit_chunks.next().await {
            let chunk = edit_chunk?;
            output.push_str(&chunk);

            for event in parser.push(&chunk) {
                match event {
                    edit_parser::EditEvent::OldText(old_text) => {
                        if let Some(range) = Self::resolve_location(&snapshot, &old_text) {
                            pending_edit = Some(PendingEdit {
                                start: range.start,
                                diff: StreamingDiff::new(old_text),
                            });
                        }
                    }
                    edit_parser::EditEvent::NewTextChunk { chunk, done } => {
                        let mut edit = pending_edit.take().context("no pending edit found")?;

                        let mut operations = edit.diff.push_new(&chunk);
                        if done {
                            operations.extend(edit.diff.finish());
                            edit.diff = StreamingDiff::new(String::new());
                        }

                        let edits = operations
                            .into_iter()
                            .filter_map(|operation| match operation {
                                CharOperation::Insert { text } => {
                                    let edit_start = snapshot.anchor_after(edit.start);
                                    Some((edit_start..edit_start, text))
                                }
                                CharOperation::Delete { bytes } => {
                                    let edit_end = edit.start + bytes;
                                    let edit_range = snapshot.anchor_after(edit.start)
                                        ..snapshot.anchor_before(edit_end);
                                    edit.start = edit_end;
                                    Some((edit_range, String::new()))
                                }
                                CharOperation::Keep { bytes } => {
                                    edit.start = edit.start + bytes;
                                    None
                                }
                            })
                            .collect::<SmallVec<[_; 1]>>();

                        // Edit the buffer and report the edit as part of the same effect cycle, otherwise
                        // the edit will be reported as if the user made it.
                        cx.update(|cx| {
                            buffer.update(cx, |buffer, cx| buffer.edit(edits, None, cx));
                            self.action_log
                                .update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx))
                        })?;

                        if done {
                            snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
                        } else {
                            pending_edit = Some(edit);
                        }
                    }
                }
            }
        }

        Ok(output)
    }

    async fn request_edits(
        &self,
        snapshot: BufferSnapshot,
        edit_description: String,
        mut messages: Vec<LanguageModelRequestMessage>,
        cx: &mut AsyncApp,
    ) -> Result<BoxStream<'static, Result<String, LanguageModelCompletionError>>> {
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
        Ok(self.model.stream_completion_text(request, cx).await?.stream)
    }

    fn resolve_location(buffer: &BufferSnapshot, search_query: &str) -> Option<Range<usize>> {
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

        let mut equal_bytes = 0;
        let mut query_ix = query_len;
        let mut buffer_ix = best_buffer_end;
        while query_ix > 0 && buffer_ix > 0 {
            let current = matrix.get(query_ix, buffer_ix);
            match current.direction {
                SearchDirection::Diagonal => {
                    query_ix -= 1;
                    buffer_ix -= 1;
                    equal_bytes += 1;
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

        let score = equal_bytes as f32 / query_len as f32;
        if score >= 0.8 {
            Some(buffer.point_to_offset(start)..buffer.point_to_offset(end))
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{App, AppContext};
    use unindent::Unindent;
    use util::test::{generate_marked_text, marked_text_ranges};

    #[gpui::test]
    fn test_resolve_location(cx: &mut App) {
        assert_location_resolution(
            concat!(
                "    Lorem\n",
                "«    ipsum\n",
                "    dolor sit amet»\n",
                "    consecteur",
            ),
            "ipsum\ndolor",
            cx,
        );

        assert_location_resolution(
            &"
            «fn foo1(a: usize) -> usize {
                40
            }»

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "fn foo1(b: usize) {\n40\n}",
            cx,
        );

        assert_location_resolution(
            &"
            fn main() {
            «    Foo
                    .bar()
                    .baz()
                    .qux()»
            }

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "Foo.bar.baz.qux()",
            cx,
        );

        assert_location_resolution(
            &"
            class Something {
                one() { return 1; }
            «    two() { return 2222; }
                three() { return 333; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            »    seven() { return 7; }
                eight() { return 8; }
            }
            "
            .unindent(),
            &"
                two() { return 2222; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            "
            .unindent(),
            cx,
        );

        assert_location_resolution(
            &"
                use std::ops::Range;
                use std::sync::Mutex;
                use std::{
                    collections::HashMap,
                    env,
                    ffi::{OsStr, OsString},
                    fs,
                    io::{BufRead, BufReader},
                    mem,
                    path::{Path, PathBuf},
                    process::Command,
                    sync::LazyLock,
                    time::SystemTime,
                };
            "
            .unindent(),
            &"
                use std::collections::{HashMap, HashSet};
                use std::ffi::{OsStr, OsString};
                use std::fmt::Write as _;
                use std::fs;
                use std::io::{BufReader, Read, Write};
                use std::mem;
                use std::path::{Path, PathBuf};
                use std::process::Command;
                use std::sync::Arc;
            "
            .unindent(),
            cx,
        );
    }

    #[track_caller]
    fn assert_location_resolution(text_with_expected_range: &str, query: &str, cx: &mut App) {
        let (text, _) = marked_text_ranges(text_with_expected_range, false);
        let buffer = cx.new(|cx| Buffer::local(text.clone(), cx));
        let snapshot = buffer.read(cx).snapshot();
        let mut ranges = Vec::new();
        ranges.extend(EditAgent::resolve_location(&snapshot, query));
        let text_with_actual_range = generate_marked_text(&text, &ranges, false);
        pretty_assertions::assert_eq!(text_with_actual_range, text_with_expected_range);
    }
}
