mod edit_parser;
#[cfg(test)]
mod evals;

use crate::{Template, Templates};
use aho_corasick::AhoCorasick;
use anyhow::Result;
use assistant_tool::ActionLog;
use edit_parser::{EditParser, EditParserEvent, EditParserMetrics};
use futures::{
    Stream, StreamExt,
    channel::mpsc::{self, UnboundedReceiver},
    stream::BoxStream,
};
use gpui::{AppContext, AsyncApp, Entity, Task};
use language::{Bias, Buffer, BufferSnapshot, LineIndent, Point};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelRequest, LanguageModelRequestMessage,
    MessageContent, Role,
};
use serde::Serialize;
use std::{cmp, iter, mem, ops::Range, path::PathBuf, sync::Arc, task::Poll};
use streaming_diff::{CharOperation, StreamingDiff};

#[derive(Serialize)]
pub struct EditAgentTemplate {
    path: Option<PathBuf>,
    edit_description: String,
}

impl Template for EditAgentTemplate {
    const TEMPLATE_NAME: &'static str = "edit_agent.hbs";
}

#[derive(Clone, Debug)]
pub struct EditAgentOutput {
    pub _raw_edits: String,
    pub _parser_metrics: EditParserMetrics,
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
    ) -> Result<EditAgentOutput> {
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
        let edit_chunks = self
            .request_edits(snapshot, edit_description, previous_messages, cx)
            .await?;
        let output = self.apply_edits(buffer, edit_chunks, cx).await?;
        Ok(output)
    }

    async fn apply_edits(
        &self,
        buffer: Entity<Buffer>,
        edit_chunks: impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>>,
        cx: &mut AsyncApp,
    ) -> Result<EditAgentOutput> {
        // Ensure the buffer is tracked by the action log.
        self.action_log
            .update(cx, |log, cx| log.track_buffer(buffer.clone(), cx))?;

        let (output, mut edit_events) = Self::parse_edit_chunks(edit_chunks, cx);
        while let Some(edit_event) = edit_events.next().await {
            let EditParserEvent::OldText(old_text_query) = edit_event? else {
                continue;
            };

            let (edits_tx, edits_rx) = mpsc::unbounded();
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let compute_edits = cx.background_spawn(async move {
                if let Some(range) = Self::resolve_location(&snapshot, &old_text_query) {
                    let buffer_start_indent =
                        snapshot.line_indent_for_row(snapshot.offset_to_point(range.start).row);
                    let old_text_start_indent = old_text_query
                        .lines()
                        .next()
                        .map_or(buffer_start_indent, |line| {
                            LineIndent::from_iter(line.chars())
                        });
                    let indent_delta = if buffer_start_indent.tabs > 0 {
                        IndentDelta::Tabs(
                            buffer_start_indent.tabs as isize - old_text_start_indent.tabs as isize,
                        )
                    } else {
                        IndentDelta::Spaces(
                            buffer_start_indent.spaces as isize
                                - old_text_start_indent.spaces as isize,
                        )
                    };

                    let old_text = snapshot.text_for_range(range.clone()).collect::<String>();
                    let mut diff = StreamingDiff::new(old_text);
                    let mut edit_start = range.start;
                    let mut new_text_chunks =
                        Self::reindent_new_text_chunks(indent_delta, &mut edit_events);
                    let mut done = false;
                    while !done {
                        let char_operations =
                            if let Some(new_text_chunk) = new_text_chunks.next().await {
                                diff.push_new(&new_text_chunk?)
                            } else {
                                done = true;
                                mem::take(&mut diff).finish()
                            };

                        for op in char_operations {
                            match op {
                                CharOperation::Insert { text } => {
                                    let edit_start = snapshot.anchor_after(edit_start);
                                    edits_tx.unbounded_send((edit_start..edit_start, text))?;
                                }
                                CharOperation::Delete { bytes } => {
                                    let edit_end = edit_start + bytes;
                                    let edit_range = snapshot.anchor_after(edit_start)
                                        ..snapshot.anchor_before(edit_end);
                                    edit_start = edit_end;
                                    edits_tx.unbounded_send((edit_range, String::new()))?;
                                }
                                CharOperation::Keep { bytes } => edit_start += bytes,
                            }
                        }
                    }
                }

                anyhow::Ok(edit_events)
            });

            // todo!("group all edits into one transaction")
            let mut edits_rx = edits_rx.ready_chunks(32);
            while let Some(edits) = edits_rx.next().await {
                // Edit the buffer and report edits to the action log as part of the
                // same effect cycle, otherwise the edit will be reported as if the
                // user made it.
                cx.update(|cx| {
                    buffer.update(cx, |buffer, cx| buffer.edit(edits, None, cx));
                    self.action_log
                        .update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx))
                })?;
            }

            edit_events = compute_edits.await?;
        }

        output.await
    }

    fn parse_edit_chunks(
        chunks: impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>>,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<EditAgentOutput>>,
        UnboundedReceiver<Result<EditParserEvent>>,
    ) {
        let (tx, rx) = mpsc::unbounded();
        let output = cx.background_spawn(async move {
            futures::pin_mut!(chunks);

            let mut parser = EditParser::new();
            let mut raw_edits = String::new();
            while let Some(chunk) = chunks.next().await {
                match chunk {
                    Ok(chunk) => {
                        raw_edits.push_str(&chunk);
                        for event in parser.push(&chunk) {
                            tx.unbounded_send(Ok(event))?;
                        }
                    }
                    Err(error) => {
                        tx.unbounded_send(Err(error.into()))?;
                    }
                }
            }
            Ok(EditAgentOutput {
                _raw_edits: raw_edits,
                _parser_metrics: parser.finish(),
            })
        });
        (output, rx)
    }

    fn reindent_new_text_chunks(
        delta: IndentDelta,
        mut stream: impl Unpin + Stream<Item = Result<EditParserEvent>>,
    ) -> impl Stream<Item = Result<String>> {
        let mut buffer = String::new();
        let mut in_leading_whitespace = true;
        let mut done = false;
        futures::stream::poll_fn(move |cx| {
            while !done {
                let (chunk, is_last_chunk) = match stream.poll_next_unpin(cx) {
                    Poll::Ready(Some(Ok(EditParserEvent::NewTextChunk { chunk, done }))) => {
                        (chunk, done)
                    }
                    Poll::Ready(Some(Err(err))) => return Poll::Ready(Some(Err(err))),
                    Poll::Pending => return Poll::Pending,
                    _ => return Poll::Ready(None),
                };

                buffer.push_str(&chunk);

                let mut indented_new_text = String::new();
                let mut start_ix = 0;
                let mut newlines = buffer.match_indices('\n').peekable();
                loop {
                    let (line_end, is_pending_line) = match newlines.next() {
                        Some((ix, _)) => (ix, false),
                        None => (buffer.len(), true),
                    };
                    let line = &buffer[start_ix..line_end];

                    if in_leading_whitespace {
                        if let Some(non_whitespace_ix) = line.find(|c| delta.character() != c) {
                            // We found a non-whitespace character, adjust
                            // indentation based on the delta.
                            let new_indent_len =
                                cmp::max(0, non_whitespace_ix as isize + delta.len()) as usize;
                            indented_new_text
                                .extend(iter::repeat(delta.character()).take(new_indent_len));
                            indented_new_text.push_str(&line[non_whitespace_ix..]);
                            in_leading_whitespace = false;
                        } else if is_pending_line {
                            // We're still in leading whitespace and this line is incomplete.
                            // Stop processing until we receive more input.
                            break;
                        } else {
                            // This line is entirely whitespace. Push it without indentation.
                            indented_new_text.push_str(line);
                        }
                    } else {
                        indented_new_text.push_str(line);
                    }

                    if is_pending_line {
                        start_ix = line_end;
                        break;
                    } else {
                        in_leading_whitespace = true;
                        indented_new_text.push('\n');
                        start_ix = line_end + 1;
                    }
                }
                buffer.replace_range(..start_ix, "");

                // This was the last chunk, push all the buffered content as-is.
                if is_last_chunk {
                    indented_new_text.push_str(&buffer);
                    buffer.clear();
                    done = true;
                }

                if !indented_new_text.is_empty() {
                    return Poll::Ready(Some(Ok(indented_new_text)));
                }
            }

            Poll::Ready(None)
        })
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
        if let Some(last_message) = messages.last_mut() {
            if last_message.role == Role::Assistant {
                last_message
                    .content
                    .retain(|content| !matches!(content, MessageContent::ToolUse(_)));
                if last_message.content.is_empty() {
                    messages.pop();
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
        let range = Self::resolve_location_exact(buffer, search_query)
            .or_else(|| Self::resolve_location_fuzzy(buffer, search_query))?;

        // Expand the range to include entire lines.
        let mut start = buffer.offset_to_point(buffer.clip_offset(range.start, Bias::Left));
        start.column = 0;
        let mut end = buffer.offset_to_point(buffer.clip_offset(range.end, Bias::Right));
        if end.column > 0 {
            end.column = buffer.line_len(end.row);
        }

        Some(buffer.point_to_offset(start)..buffer.point_to_offset(end))
    }

    fn resolve_location_exact(buffer: &BufferSnapshot, search_query: &str) -> Option<Range<usize>> {
        let search = AhoCorasick::new([search_query]).ok()?;
        let mat = search
            .stream_find_iter(buffer.bytes_in_range(0..buffer.len()))
            .next()?
            .expect("buffer can't error");
        Some(mat.range())
    }

    fn resolve_location_fuzzy(buffer: &BufferSnapshot, search_query: &str) -> Option<Range<usize>> {
        const INSERTION_COST: u32 = 3;
        const DELETION_COST: u32 = 10;

        let buffer_line_count = buffer.max_point().row as usize + 1;
        let query_line_count = search_query.lines().count();
        let mut matrix = SearchMatrix::new(query_line_count + 1, buffer_line_count + 1);
        let mut leading_deletion_cost = 0_u32;
        for (row, query_line) in search_query.lines().enumerate() {
            let query_line = query_line.trim();
            leading_deletion_cost = leading_deletion_cost.saturating_add(DELETION_COST);
            matrix.set(
                row + 1,
                0,
                SearchState::new(leading_deletion_cost, SearchDirection::Diagonal),
            );

            let mut buffer_lines = buffer.as_rope().chunks().lines();
            let mut col = 0;
            while let Some(buffer_line) = buffer_lines.next() {
                let buffer_line = buffer_line.trim();
                let up = SearchState::new(
                    matrix.get(row, col + 1).cost.saturating_add(DELETION_COST),
                    SearchDirection::Up,
                );
                let left = SearchState::new(
                    matrix.get(row + 1, col).cost.saturating_add(INSERTION_COST),
                    SearchDirection::Left,
                );
                let diagonal = SearchState::new(
                    if fuzzy_eq(query_line, buffer_line) {
                        matrix.get(row, col).cost
                    } else {
                        matrix
                            .get(row, col)
                            .cost
                            .saturating_add(DELETION_COST + INSERTION_COST)
                    },
                    SearchDirection::Diagonal,
                );
                matrix.set(row + 1, col + 1, up.min(left).min(diagonal));
                col += 1;
            }
        }

        // Traceback to find the best match
        let mut buffer_row_end = buffer_line_count as u32;
        let mut best_cost = u32::MAX;
        for col in 1..=buffer_line_count {
            let cost = matrix.get(query_line_count, col).cost;
            if cost < best_cost {
                best_cost = cost;
                buffer_row_end = col as u32;
            }
        }

        let mut matched_lines = 0;
        let mut query_row = query_line_count;
        let mut buffer_row_start = buffer_row_end;
        while query_row > 0 && buffer_row_start > 0 {
            let current = matrix.get(query_row, buffer_row_start as usize);
            match current.direction {
                SearchDirection::Diagonal => {
                    query_row -= 1;
                    buffer_row_start -= 1;
                    matched_lines += 1;
                }
                SearchDirection::Up => {
                    query_row -= 1;
                }
                SearchDirection::Left => {
                    buffer_row_start -= 1;
                }
            }
        }

        let matched_buffer_row_count = buffer_row_end - buffer_row_start;
        let matched_ratio =
            matched_lines as f32 / (matched_buffer_row_count as f32).max(query_line_count as f32);
        if matched_ratio >= 0.8 {
            let buffer_start_ix = buffer.point_to_offset(Point::new(buffer_row_start, 0));
            let buffer_end_ix = buffer.point_to_offset(Point::new(
                buffer_row_end - 1,
                buffer.line_len(buffer_row_end - 1),
            ));
            Some(buffer_start_ix..buffer_end_ix)
        } else {
            None
        }
    }
}

fn fuzzy_eq(left: &str, right: &str) -> bool {
    let min_levenshtein = left.len().abs_diff(right.len());
    let min_normalized_levenshtein =
        1. - (min_levenshtein as f32 / cmp::max(left.len(), right.len()) as f32);
    if min_normalized_levenshtein < 0.8 {
        return false;
    }

    strsim::normalized_levenshtein(left, right) >= 0.8
}

#[derive(Copy, Clone, Debug)]
enum IndentDelta {
    Spaces(isize),
    Tabs(isize),
}

impl IndentDelta {
    fn character(&self) -> char {
        match self {
            IndentDelta::Spaces(_) => ' ',
            IndentDelta::Tabs(_) => '\t',
        }
    }

    fn len(&self) -> isize {
        match self {
            IndentDelta::Spaces(n) => *n,
            IndentDelta::Tabs(n) => *n,
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
    use fs::FakeFs;
    use futures::stream;
    use gpui::{App, AppContext, TestAppContext};
    use indoc::indoc;
    use language_model::fake_provider::FakeLanguageModel;
    use project::Project;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use std::cmp;
    use unindent::Unindent;
    use util::test::{generate_marked_text, marked_text_ranges};

    #[gpui::test(iterations = 100)]
    async fn test_indentation(cx: &mut TestAppContext, mut rng: StdRng) {
        let agent = init_test(cx).await;
        let buffer = cx.new(|cx| {
            Buffer::local(
                indoc! {"
                    lorem
                            ipsum
                            dolor
                            sit
                "},
                cx,
            )
        });
        let raw_edits = simulate_llm_output(
            indoc! {"
                <old_text>
                    ipsum
                    dolor
                    sit
                </old_text>
                <new_text>
                    ipsum
                    dolor
                    sit
                amet
                </new_text>
            "},
            &mut rng,
            cx,
        );
        agent
            .apply_edits(buffer.clone(), raw_edits, &mut cx.to_async())
            .await
            .unwrap();
        pretty_assertions::assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            indoc! {"
                lorem
                        ipsum
                        dolor
                        sit
                    amet
            "}
        );
    }

    #[gpui::test(iterations = 100)]
    async fn test_dependent_edits(cx: &mut TestAppContext, mut rng: StdRng) {
        let agent = init_test(cx).await;
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi", cx));
        let raw_edits = simulate_llm_output(
            indoc! {"
                <old_text>
                def
                </old_text>
                <new_text>
                DEF
                </new_text>

                <old_text>
                DEF
                </old_text>
                <new_text>
                DeF
                </new_text>
            "},
            &mut rng,
            cx,
        );
        agent
            .apply_edits(buffer.clone(), raw_edits, &mut cx.to_async())
            .await
            .unwrap();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abc\nDeF\nghi"
        );
    }

    #[gpui::test(iterations = 100)]
    async fn test_old_text_hallucination(cx: &mut TestAppContext, mut rng: StdRng) {
        let agent = init_test(cx).await;
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi", cx));
        let raw_edits = simulate_llm_output(
            indoc! {"
                <old_text>
                jkl
                </old_text>
                <new_text>
                mno
                </new_text>

                <old_text>
                abc
                </old_text>
                <new_text>
                ABC
                </new_text>
            "},
            &mut rng,
            cx,
        );
        agent
            .apply_edits(buffer.clone(), raw_edits, &mut cx.to_async())
            .await
            .unwrap();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "ABC\ndef\nghi"
        );
    }

    #[gpui::test]
    fn test_resolve_location(cx: &mut App) {
        assert_location_resolution(
            concat!(
                "    Lorem\n",
                "«    ipsum»\n",
                "    dolor sit amet\n",
                "    consecteur",
            ),
            "ipsum",
            cx,
        );

        assert_location_resolution(
            concat!(
                "    Lorem\n",
                "«    ipsum\n",
                "    dolor sit amet»\n",
                "    consecteur",
            ),
            "ipsum\ndolor sit amet",
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
            "fn foo1(a: usize) -> u32 {\n40\n}",
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
                six() { return 6666; }»
                seven() { return 7; }
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

        assert_location_resolution(
            indoc! {"
                impl Foo {
                    fn new() -> Self {
                        Self {
                            subscriptions: vec![
                                cx.observe_window_activation(window, |editor, window, cx| {
                                    let active = window.is_window_active();
                                    editor.blink_manager.update(cx, |blink_manager, cx| {
                                        if active {
                                            blink_manager.enable(cx);
                                        } else {
                                            blink_manager.disable(cx);
                                        }
                                    });
                                }),
                            ];
                        }
                    }
                }
            "},
            concat!(
                "                    editor.blink_manager.update(cx, |blink_manager, cx| {\n",
                "                        blink_manager.enable(cx);\n",
                "                    });",
            ),
            cx,
        );

        assert_location_resolution(
            indoc! {r#"
                let tool = cx
                    .update(|cx| working_set.tool(&tool_name, cx))
                    .map_err(|err| {
                        anyhow!("Failed to look up tool '{}': {}", tool_name, err)
                    })?;

                let Some(tool) = tool else {
                    return Err(anyhow!("Tool '{}' not found", tool_name));
                };

                let project = project.clone();
                let action_log = action_log.clone();
                let messages = messages.clone();
                let tool_result = cx
                    .update(|cx| tool.run(invocation.input, &messages, project, action_log, cx))
                    .map_err(|err| anyhow!("Failed to start tool '{}': {}", tool_name, err))?;

                tasks.push(tool_result.output);
            "#},
            concat!(
                "let tool_result = cx\n",
                "    .update(|cx| tool.run(invocation.input, &messages, project, action_log, cx))\n",
                "    .output;",
            ),
            cx,
        );
    }

    #[gpui::test(iterations = 100)]
    async fn test_indent_new_text_chunks(mut rng: StdRng) {
        let chunks = to_random_chunks(&mut rng, "    abc\n  def\n      ghi");
        let new_text_chunks = stream::iter(chunks.iter().enumerate().map(|(index, chunk)| {
            Ok(EditParserEvent::NewTextChunk {
                chunk: chunk.clone(),
                done: index == chunks.len() - 1,
            })
        }));
        let indented_chunks =
            EditAgent::reindent_new_text_chunks(IndentDelta::Spaces(2), new_text_chunks)
                .collect::<Vec<_>>()
                .await;
        let new_text = indented_chunks
            .into_iter()
            .collect::<Result<String>>()
            .unwrap();
        assert_eq!(new_text, "      abc\n    def\n        ghi");
    }

    #[gpui::test(iterations = 100)]
    async fn test_outdent_new_text_chunks(mut rng: StdRng) {
        let chunks = to_random_chunks(&mut rng, "\t\t\t\tabc\n\t\tdef\n\t\t\t\t\t\tghi");
        let new_text_chunks = stream::iter(chunks.iter().enumerate().map(|(index, chunk)| {
            Ok(EditParserEvent::NewTextChunk {
                chunk: chunk.clone(),
                done: index == chunks.len() - 1,
            })
        }));
        let indented_chunks =
            EditAgent::reindent_new_text_chunks(IndentDelta::Tabs(-2), new_text_chunks)
                .collect::<Vec<_>>()
                .await;
        let new_text = indented_chunks
            .into_iter()
            .collect::<Result<String>>()
            .unwrap();
        assert_eq!(new_text, "\t\tabc\ndef\n\t\t\t\tghi");
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_indents(mut rng: StdRng) {
        let len = rng.gen_range(1..=100);
        let new_text = util::RandomCharIter::new(&mut rng)
            .with_simple_text()
            .take(len)
            .collect::<String>();
        let new_text = new_text
            .split('\n')
            .map(|line| format!("{}{}", " ".repeat(rng.gen_range(0..=8)), line))
            .collect::<Vec<_>>()
            .join("\n");
        let delta = IndentDelta::Spaces(rng.gen_range(-4..=4));

        let chunks = to_random_chunks(&mut rng, &new_text);
        let new_text_chunks = stream::iter(chunks.iter().enumerate().map(|(index, chunk)| {
            Ok(EditParserEvent::NewTextChunk {
                chunk: chunk.clone(),
                done: index == chunks.len() - 1,
            })
        }));
        let reindented_chunks = EditAgent::reindent_new_text_chunks(delta, new_text_chunks)
            .collect::<Vec<_>>()
            .await;
        let actual_reindented_text = reindented_chunks
            .into_iter()
            .collect::<Result<String>>()
            .unwrap();
        let expected_reindented_text = new_text
            .split('\n')
            .map(|line| {
                if let Some(ix) = line.find(|c| c != ' ') {
                    let new_indent = cmp::max(0, ix as isize + delta.len()) as usize;
                    format!("{}{}", " ".repeat(new_indent), &line[ix..])
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual_reindented_text, expected_reindented_text);
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

    fn to_random_chunks(rng: &mut StdRng, input: &str) -> Vec<String> {
        let chunk_count = rng.gen_range(1..=cmp::min(input.len(), 50));
        let mut chunk_indices = (0..input.len()).choose_multiple(rng, chunk_count);
        chunk_indices.sort();
        chunk_indices.push(input.len());

        let mut chunks = Vec::new();
        let mut last_ix = 0;
        for chunk_ix in chunk_indices {
            chunks.push(input[last_ix..chunk_ix].to_string());
            last_ix = chunk_ix;
        }
        chunks
    }

    fn simulate_llm_output(
        output: &str,
        rng: &mut StdRng,
        cx: &mut TestAppContext,
    ) -> impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>> {
        let executor = cx.executor();
        stream::iter(to_random_chunks(rng, output).into_iter().map(Ok)).then(move |chunk| {
            let executor = executor.clone();
            async move {
                executor.simulate_random_delay().await;
                chunk
            }
        })
    }

    async fn init_test(cx: &mut TestAppContext) -> EditAgent {
        cx.update(settings::init);
        cx.update(Project::init_settings);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let model = Arc::new(FakeLanguageModel::default());
        let action_log = cx.new(|_| ActionLog::new(project));
        EditAgent::new(model, action_log, Templates::new())
    }
}
