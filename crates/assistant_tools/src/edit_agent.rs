mod create_file_parser;
mod edit_parser;
#[cfg(test)]
mod evals;
mod streaming_fuzzy_matcher;

use crate::{Template, Templates};
use action_log::ActionLog;
use anyhow::Result;
use cloud_llm_client::CompletionIntent;
use create_file_parser::{CreateFileParser, CreateFileParserEvent};
pub use edit_parser::EditFormat;
use edit_parser::{EditParser, EditParserEvent, EditParserMetrics};
use futures::{
    Stream, StreamExt,
    channel::mpsc::{self, UnboundedReceiver},
    pin_mut,
    stream::BoxStream,
};
use gpui::{AppContext, AsyncApp, Entity, Task};
use language::{Anchor, Buffer, BufferSnapshot, LineIndent, Point, TextBufferSnapshot};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolChoice, MessageContent, Role,
};
use project::{AgentLocation, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, iter, mem, ops::Range, pin::Pin, sync::Arc, task::Poll};
use streaming_diff::{CharOperation, StreamingDiff};
use streaming_fuzzy_matcher::StreamingFuzzyMatcher;

#[derive(Serialize)]
struct CreateFilePromptTemplate {
    path: Option<String>,
    edit_description: String,
}

impl Template for CreateFilePromptTemplate {
    const TEMPLATE_NAME: &'static str = "create_file_prompt.hbs";
}

#[derive(Serialize)]
struct EditFileXmlPromptTemplate {
    path: Option<String>,
    edit_description: String,
}

impl Template for EditFileXmlPromptTemplate {
    const TEMPLATE_NAME: &'static str = "edit_file_prompt_xml.hbs";
}

#[derive(Serialize)]
struct EditFileDiffFencedPromptTemplate {
    path: Option<String>,
    edit_description: String,
}

impl Template for EditFileDiffFencedPromptTemplate {
    const TEMPLATE_NAME: &'static str = "edit_file_prompt_diff_fenced.hbs";
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditAgentOutputEvent {
    ResolvingEditRange(Range<Anchor>),
    UnresolvedEditRange,
    AmbiguousEditRange(Vec<Range<usize>>),
    Edited(Range<Anchor>),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditAgentOutput {
    pub raw_edits: String,
    pub parser_metrics: EditParserMetrics,
}

#[derive(Clone)]
pub struct EditAgent {
    model: Arc<dyn LanguageModel>,
    action_log: Entity<ActionLog>,
    project: Entity<Project>,
    templates: Arc<Templates>,
    edit_format: EditFormat,
}

impl EditAgent {
    pub fn new(
        model: Arc<dyn LanguageModel>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        templates: Arc<Templates>,
        edit_format: EditFormat,
    ) -> Self {
        EditAgent {
            model,
            project,
            action_log,
            templates,
            edit_format,
        }
    }

    pub fn overwrite(
        &self,
        buffer: Entity<Buffer>,
        edit_description: String,
        conversation: &LanguageModelRequest,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<EditAgentOutput>>,
        mpsc::UnboundedReceiver<EditAgentOutputEvent>,
    ) {
        let this = self.clone();
        let (events_tx, events_rx) = mpsc::unbounded();
        let conversation = conversation.clone();
        let output = cx.spawn(async move |cx| {
            let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
            let path = cx.update(|cx| snapshot.resolve_file_path(true, cx))?;
            let prompt = CreateFilePromptTemplate {
                path,
                edit_description,
            }
            .render(&this.templates)?;
            let new_chunks = this
                .request(conversation, CompletionIntent::CreateFile, prompt, cx)
                .await?;

            let (output, mut inner_events) = this.overwrite_with_chunks(buffer, new_chunks, cx);
            while let Some(event) = inner_events.next().await {
                events_tx.unbounded_send(event).ok();
            }
            output.await
        });
        (output, events_rx)
    }

    fn overwrite_with_chunks(
        &self,
        buffer: Entity<Buffer>,
        edit_chunks: impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>>,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<EditAgentOutput>>,
        mpsc::UnboundedReceiver<EditAgentOutputEvent>,
    ) {
        let (output_events_tx, output_events_rx) = mpsc::unbounded();
        let (parse_task, parse_rx) = Self::parse_create_file_chunks(edit_chunks, cx);
        let this = self.clone();
        let task = cx.spawn(async move |cx| {
            this.action_log
                .update(cx, |log, cx| log.buffer_created(buffer.clone(), cx))?;
            this.overwrite_with_chunks_internal(buffer, parse_rx, output_events_tx, cx)
                .await?;
            parse_task.await
        });
        (task, output_events_rx)
    }

    async fn overwrite_with_chunks_internal(
        &self,
        buffer: Entity<Buffer>,
        mut parse_rx: UnboundedReceiver<Result<CreateFileParserEvent>>,
        output_events_tx: mpsc::UnboundedSender<EditAgentOutputEvent>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        cx.update(|cx| {
            buffer.update(cx, |buffer, cx| buffer.set_text("", cx));
            self.action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx);
            });
            self.project.update(cx, |project, cx| {
                project.set_agent_location(
                    Some(AgentLocation {
                        buffer: buffer.downgrade(),
                        position: language::Anchor::MAX,
                    }),
                    cx,
                )
            });
            output_events_tx
                .unbounded_send(EditAgentOutputEvent::Edited(
                    language::Anchor::MIN..language::Anchor::MAX,
                ))
                .ok();
        })?;

        while let Some(event) = parse_rx.next().await {
            match event? {
                CreateFileParserEvent::NewTextChunk { chunk } => {
                    cx.update(|cx| {
                        buffer.update(cx, |buffer, cx| buffer.append(chunk, cx));
                        self.action_log
                            .update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
                        self.project.update(cx, |project, cx| {
                            project.set_agent_location(
                                Some(AgentLocation {
                                    buffer: buffer.downgrade(),
                                    position: language::Anchor::MAX,
                                }),
                                cx,
                            )
                        });
                    })?;
                    output_events_tx
                        .unbounded_send(EditAgentOutputEvent::Edited(
                            language::Anchor::MIN..language::Anchor::MAX,
                        ))
                        .ok();
                }
            }
        }

        Ok(())
    }

    pub fn edit(
        &self,
        buffer: Entity<Buffer>,
        edit_description: String,
        conversation: &LanguageModelRequest,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<EditAgentOutput>>,
        mpsc::UnboundedReceiver<EditAgentOutputEvent>,
    ) {
        let this = self.clone();
        let (events_tx, events_rx) = mpsc::unbounded();
        let conversation = conversation.clone();
        let edit_format = self.edit_format;
        let output = cx.spawn(async move |cx| {
            let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
            let path = cx.update(|cx| snapshot.resolve_file_path(true, cx))?;
            let prompt = match edit_format {
                EditFormat::XmlTags => EditFileXmlPromptTemplate {
                    path,
                    edit_description,
                }
                .render(&this.templates)?,
                EditFormat::DiffFenced => EditFileDiffFencedPromptTemplate {
                    path,
                    edit_description,
                }
                .render(&this.templates)?,
            };

            let edit_chunks = this
                .request(conversation, CompletionIntent::EditFile, prompt, cx)
                .await?;
            this.apply_edit_chunks(buffer, edit_chunks, events_tx, cx)
                .await
        });
        (output, events_rx)
    }

    async fn apply_edit_chunks(
        &self,
        buffer: Entity<Buffer>,
        edit_chunks: impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>>,
        output_events: mpsc::UnboundedSender<EditAgentOutputEvent>,
        cx: &mut AsyncApp,
    ) -> Result<EditAgentOutput> {
        self.action_log
            .update(cx, |log, cx| log.buffer_read(buffer.clone(), cx))?;

        let (output, edit_events) = Self::parse_edit_chunks(edit_chunks, self.edit_format, cx);
        let mut edit_events = edit_events.peekable();
        while let Some(edit_event) = Pin::new(&mut edit_events).peek().await {
            // Skip events until we're at the start of a new edit.
            let Ok(EditParserEvent::OldTextChunk { .. }) = edit_event else {
                edit_events.next().await.unwrap()?;
                continue;
            };

            let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

            // Resolve the old text in the background, updating the agent
            // location as we keep refining which range it corresponds to.
            let (resolve_old_text, mut old_range) =
                Self::resolve_old_text(snapshot.text.clone(), edit_events, cx);
            while let Ok(old_range) = old_range.recv().await {
                if let Some(old_range) = old_range {
                    let old_range = snapshot.anchor_before(old_range.start)
                        ..snapshot.anchor_before(old_range.end);
                    self.project.update(cx, |project, cx| {
                        project.set_agent_location(
                            Some(AgentLocation {
                                buffer: buffer.downgrade(),
                                position: old_range.end,
                            }),
                            cx,
                        );
                    })?;
                    output_events
                        .unbounded_send(EditAgentOutputEvent::ResolvingEditRange(old_range))
                        .ok();
                }
            }

            let (edit_events_, mut resolved_old_text) = resolve_old_text.await?;
            edit_events = edit_events_;

            // If we can't resolve the old text, restart the loop waiting for a
            // new edit (or for the stream to end).
            let resolved_old_text = match resolved_old_text.len() {
                1 => resolved_old_text.pop().unwrap(),
                0 => {
                    output_events
                        .unbounded_send(EditAgentOutputEvent::UnresolvedEditRange)
                        .ok();
                    continue;
                }
                _ => {
                    let ranges = resolved_old_text
                        .into_iter()
                        .map(|text| {
                            let start_line =
                                (snapshot.offset_to_point(text.range.start).row + 1) as usize;
                            let end_line =
                                (snapshot.offset_to_point(text.range.end).row + 1) as usize;
                            start_line..end_line
                        })
                        .collect();
                    output_events
                        .unbounded_send(EditAgentOutputEvent::AmbiguousEditRange(ranges))
                        .ok();
                    continue;
                }
            };

            // Compute edits in the background and apply them as they become
            // available.
            let (compute_edits, edits) =
                Self::compute_edits(snapshot, resolved_old_text, edit_events, cx);
            let mut edits = edits.ready_chunks(32);
            while let Some(edits) = edits.next().await {
                if edits.is_empty() {
                    continue;
                }

                // Edit the buffer and report edits to the action log as part of the
                // same effect cycle, otherwise the edit will be reported as if the
                // user made it.
                let (min_edit_start, max_edit_end) = cx.update(|cx| {
                    let (min_edit_start, max_edit_end) = buffer.update(cx, |buffer, cx| {
                        buffer.edit(edits.iter().cloned(), None, cx);
                        let max_edit_end = buffer
                            .summaries_for_anchors::<Point, _>(
                                edits.iter().map(|(range, _)| &range.end),
                            )
                            .max()
                            .unwrap();
                        let min_edit_start = buffer
                            .summaries_for_anchors::<Point, _>(
                                edits.iter().map(|(range, _)| &range.start),
                            )
                            .min()
                            .unwrap();
                        (
                            buffer.anchor_after(min_edit_start),
                            buffer.anchor_before(max_edit_end),
                        )
                    });
                    self.action_log
                        .update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
                    self.project.update(cx, |project, cx| {
                        project.set_agent_location(
                            Some(AgentLocation {
                                buffer: buffer.downgrade(),
                                position: max_edit_end,
                            }),
                            cx,
                        );
                    });
                    (min_edit_start, max_edit_end)
                })?;
                output_events
                    .unbounded_send(EditAgentOutputEvent::Edited(min_edit_start..max_edit_end))
                    .ok();
            }

            edit_events = compute_edits.await?;
        }

        output.await
    }

    fn parse_edit_chunks(
        chunks: impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>>,
        edit_format: EditFormat,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<EditAgentOutput>>,
        UnboundedReceiver<Result<EditParserEvent>>,
    ) {
        let (tx, rx) = mpsc::unbounded();
        let output = cx.background_spawn(async move {
            pin_mut!(chunks);

            let mut parser = EditParser::new(edit_format);
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
                raw_edits,
                parser_metrics: parser.finish(),
            })
        });
        (output, rx)
    }

    fn parse_create_file_chunks(
        chunks: impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>>,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<EditAgentOutput>>,
        UnboundedReceiver<Result<CreateFileParserEvent>>,
    ) {
        let (tx, rx) = mpsc::unbounded();
        let output = cx.background_spawn(async move {
            pin_mut!(chunks);

            let mut parser = CreateFileParser::new();
            let mut raw_edits = String::new();
            while let Some(chunk) = chunks.next().await {
                match chunk {
                    Ok(chunk) => {
                        raw_edits.push_str(&chunk);
                        for event in parser.push(Some(&chunk)) {
                            tx.unbounded_send(Ok(event))?;
                        }
                    }
                    Err(error) => {
                        tx.unbounded_send(Err(error.into()))?;
                    }
                }
            }
            // Send final events with None to indicate completion
            for event in parser.push(None) {
                tx.unbounded_send(Ok(event))?;
            }
            Ok(EditAgentOutput {
                raw_edits,
                parser_metrics: EditParserMetrics::default(),
            })
        });
        (output, rx)
    }

    fn resolve_old_text<T>(
        snapshot: TextBufferSnapshot,
        mut edit_events: T,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<(T, Vec<ResolvedOldText>)>>,
        watch::Receiver<Option<Range<usize>>>,
    )
    where
        T: 'static + Send + Unpin + Stream<Item = Result<EditParserEvent>>,
    {
        let (mut old_range_tx, old_range_rx) = watch::channel(None);
        let task = cx.background_spawn(async move {
            let mut matcher = StreamingFuzzyMatcher::new(snapshot);
            while let Some(edit_event) = edit_events.next().await {
                let EditParserEvent::OldTextChunk {
                    chunk,
                    done,
                    line_hint,
                } = edit_event?
                else {
                    break;
                };

                old_range_tx.send(matcher.push(&chunk, line_hint))?;
                if done {
                    break;
                }
            }

            let matches = matcher.finish();
            let best_match = matcher.select_best_match();

            old_range_tx.send(best_match.clone())?;

            let indent = LineIndent::from_iter(
                matcher
                    .query_lines()
                    .first()
                    .unwrap_or(&String::new())
                    .chars(),
            );

            let resolved_old_texts = if let Some(best_match) = best_match {
                vec![ResolvedOldText {
                    range: best_match,
                    indent,
                }]
            } else {
                matches
                    .into_iter()
                    .map(|range| ResolvedOldText { range, indent })
                    .collect::<Vec<_>>()
            };

            Ok((edit_events, resolved_old_texts))
        });

        (task, old_range_rx)
    }

    fn compute_edits<T>(
        snapshot: BufferSnapshot,
        resolved_old_text: ResolvedOldText,
        mut edit_events: T,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<T>>,
        UnboundedReceiver<(Range<Anchor>, Arc<str>)>,
    )
    where
        T: 'static + Send + Unpin + Stream<Item = Result<EditParserEvent>>,
    {
        let (edits_tx, edits_rx) = mpsc::unbounded();
        let compute_edits = cx.background_spawn(async move {
            let buffer_start_indent = snapshot
                .line_indent_for_row(snapshot.offset_to_point(resolved_old_text.range.start).row);
            let indent_delta = if buffer_start_indent.tabs > 0 {
                IndentDelta::Tabs(
                    buffer_start_indent.tabs as isize - resolved_old_text.indent.tabs as isize,
                )
            } else {
                IndentDelta::Spaces(
                    buffer_start_indent.spaces as isize - resolved_old_text.indent.spaces as isize,
                )
            };

            let old_text = snapshot
                .text_for_range(resolved_old_text.range.clone())
                .collect::<String>();
            let mut diff = StreamingDiff::new(old_text);
            let mut edit_start = resolved_old_text.range.start;
            let mut new_text_chunks =
                Self::reindent_new_text_chunks(indent_delta, &mut edit_events);
            let mut done = false;
            while !done {
                let char_operations = if let Some(new_text_chunk) = new_text_chunks.next().await {
                    diff.push_new(&new_text_chunk?)
                } else {
                    done = true;
                    mem::take(&mut diff).finish()
                };

                for op in char_operations {
                    match op {
                        CharOperation::Insert { text } => {
                            let edit_start = snapshot.anchor_after(edit_start);
                            edits_tx.unbounded_send((edit_start..edit_start, Arc::from(text)))?;
                        }
                        CharOperation::Delete { bytes } => {
                            let edit_end = edit_start + bytes;
                            let edit_range =
                                snapshot.anchor_after(edit_start)..snapshot.anchor_before(edit_end);
                            edit_start = edit_end;
                            edits_tx.unbounded_send((edit_range, Arc::from("")))?;
                        }
                        CharOperation::Keep { bytes } => edit_start += bytes,
                    }
                }
            }

            drop(new_text_chunks);
            anyhow::Ok(edit_events)
        });

        (compute_edits, edits_rx)
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

    async fn request(
        &self,
        mut conversation: LanguageModelRequest,
        intent: CompletionIntent,
        prompt: String,
        cx: &mut AsyncApp,
    ) -> Result<BoxStream<'static, Result<String, LanguageModelCompletionError>>> {
        let mut messages_iter = conversation.messages.iter_mut();
        if let Some(last_message) = messages_iter.next_back()
            && last_message.role == Role::Assistant
        {
            let old_content_len = last_message.content.len();
            last_message
                .content
                .retain(|content| !matches!(content, MessageContent::ToolUse(_)));
            let new_content_len = last_message.content.len();

            // We just removed pending tool uses from the content of the
            // last message, so it doesn't make sense to cache it anymore
            // (e.g., the message will look very different on the next
            // request). Thus, we move the flag to the message prior to it,
            // as it will still be a valid prefix of the conversation.
            if old_content_len != new_content_len
                && last_message.cache
                && let Some(prev_message) = messages_iter.next_back()
            {
                last_message.cache = false;
                prev_message.cache = true;
            }

            if last_message.content.is_empty() {
                conversation.messages.pop();
            }
        }

        conversation.messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text(prompt)],
            cache: false,
        });

        // Include tools in the request so that we can take advantage of
        // caching when ToolChoice::None is supported.
        let mut tool_choice = None;
        let mut tools = Vec::new();
        if !conversation.tools.is_empty()
            && self
                .model
                .supports_tool_choice(LanguageModelToolChoice::None)
        {
            tool_choice = Some(LanguageModelToolChoice::None);
            tools = conversation.tools.clone();
        }

        let request = LanguageModelRequest {
            thread_id: conversation.thread_id,
            prompt_id: conversation.prompt_id,
            intent: Some(intent),
            mode: conversation.mode,
            messages: conversation.messages,
            tool_choice,
            tools,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: true,
        };

        Ok(self.model.stream_completion_text(request, cx).await?.stream)
    }
}

struct ResolvedOldText {
    range: Range<usize>,
    indent: LineIndent,
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use futures::stream;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use language_model::fake_provider::FakeLanguageModel;
    use pretty_assertions::assert_matches;
    use project::{AgentLocation, Project};
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use std::cmp;

    #[gpui::test(iterations = 100)]
    async fn test_empty_old_text(cx: &mut TestAppContext, mut rng: StdRng) {
        let agent = init_test(cx).await;
        let buffer = cx.new(|cx| {
            Buffer::local(
                indoc! {"
                    abc
                    def
                    ghi
                "},
                cx,
            )
        });
        let (apply, _events) = agent.edit(
            buffer.clone(),
            String::new(),
            &LanguageModelRequest::default(),
            &mut cx.to_async(),
        );
        cx.run_until_parked();

        simulate_llm_output(
            &agent,
            indoc! {"
                <old_text></old_text>
                <new_text>jkl</new_text>
                <old_text>def</old_text>
                <new_text>DEF</new_text>
            "},
            &mut rng,
            cx,
        );
        apply.await.unwrap();

        pretty_assertions::assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            indoc! {"
                abc
                DEF
                ghi
            "}
        );
    }

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
        let (apply, _events) = agent.edit(
            buffer.clone(),
            String::new(),
            &LanguageModelRequest::default(),
            &mut cx.to_async(),
        );
        cx.run_until_parked();

        simulate_llm_output(
            &agent,
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
        apply.await.unwrap();

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
        let (apply, _events) = agent.edit(
            buffer.clone(),
            String::new(),
            &LanguageModelRequest::default(),
            &mut cx.to_async(),
        );
        cx.run_until_parked();

        simulate_llm_output(
            &agent,
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
        apply.await.unwrap();

        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abc\nDeF\nghi"
        );
    }

    #[gpui::test(iterations = 100)]
    async fn test_old_text_hallucination(cx: &mut TestAppContext, mut rng: StdRng) {
        let agent = init_test(cx).await;
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi", cx));
        let (apply, _events) = agent.edit(
            buffer.clone(),
            String::new(),
            &LanguageModelRequest::default(),
            &mut cx.to_async(),
        );
        cx.run_until_parked();

        simulate_llm_output(
            &agent,
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
        apply.await.unwrap();

        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "ABC\ndef\nghi"
        );
    }

    #[gpui::test]
    async fn test_edit_events(cx: &mut TestAppContext) {
        let agent = init_test(cx).await;
        let model = agent.model.as_fake();
        let project = agent
            .action_log
            .read_with(cx, |log, _| log.project().clone());
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi\njkl", cx));

        let mut async_cx = cx.to_async();
        let (apply, mut events) = agent.edit(
            buffer.clone(),
            String::new(),
            &LanguageModelRequest::default(),
            &mut async_cx,
        );
        cx.run_until_parked();

        model.send_last_completion_stream_text_chunk("<old_text>a");
        cx.run_until_parked();
        assert_eq!(drain_events(&mut events), vec![]);
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abc\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            None
        );

        model.send_last_completion_stream_text_chunk("bc</old_text>");
        cx.run_until_parked();
        assert_eq!(
            drain_events(&mut events),
            vec![EditAgentOutputEvent::ResolvingEditRange(buffer.read_with(
                cx,
                |buffer, _| buffer.anchor_before(Point::new(0, 0))
                    ..buffer.anchor_before(Point::new(0, 3))
            ))]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abc\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(0, 3)))
            })
        );

        model.send_last_completion_stream_text_chunk("<new_text>abX");
        cx.run_until_parked();
        assert_matches!(
            drain_events(&mut events).as_slice(),
            [EditAgentOutputEvent::Edited(_)]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXc\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(0, 3)))
            })
        );

        model.send_last_completion_stream_text_chunk("cY");
        cx.run_until_parked();
        assert_matches!(
            drain_events(&mut events).as_slice(),
            [EditAgentOutputEvent::Edited { .. }]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(0, 5)))
            })
        );

        model.send_last_completion_stream_text_chunk("</new_text>");
        model.send_last_completion_stream_text_chunk("<old_text>hall");
        cx.run_until_parked();
        assert_eq!(drain_events(&mut events), vec![]);
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(0, 5)))
            })
        );

        model.send_last_completion_stream_text_chunk("ucinated old</old_text>");
        model.send_last_completion_stream_text_chunk("<new_text>");
        cx.run_until_parked();
        assert_eq!(
            drain_events(&mut events),
            vec![EditAgentOutputEvent::UnresolvedEditRange]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(0, 5)))
            })
        );

        model.send_last_completion_stream_text_chunk("hallucinated new</new_");
        model.send_last_completion_stream_text_chunk("text>");
        cx.run_until_parked();
        assert_eq!(drain_events(&mut events), vec![]);
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(0, 5)))
            })
        );

        model.send_last_completion_stream_text_chunk("<old_text>\nghi\nj");
        cx.run_until_parked();
        assert_eq!(
            drain_events(&mut events),
            vec![EditAgentOutputEvent::ResolvingEditRange(buffer.read_with(
                cx,
                |buffer, _| buffer.anchor_before(Point::new(2, 0))
                    ..buffer.anchor_before(Point::new(2, 3))
            ))]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(2, 3)))
            })
        );

        model.send_last_completion_stream_text_chunk("kl</old_text>");
        model.send_last_completion_stream_text_chunk("<new_text>");
        cx.run_until_parked();
        assert_eq!(
            drain_events(&mut events),
            vec![EditAgentOutputEvent::ResolvingEditRange(buffer.read_with(
                cx,
                |buffer, _| buffer.anchor_before(Point::new(2, 0))
                    ..buffer.anchor_before(Point::new(3, 3))
            ))]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nghi\njkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(3, 3)))
            })
        );

        model.send_last_completion_stream_text_chunk("GHI</new_text>");
        cx.run_until_parked();
        assert_matches!(
            drain_events(&mut events).as_slice(),
            [EditAgentOutputEvent::Edited { .. }]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nGHI"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(2, 3)))
            })
        );

        model.end_last_completion_stream();
        apply.await.unwrap();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "abXcY\ndef\nGHI"
        );
        assert_eq!(drain_events(&mut events), vec![]);
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(2, 3)))
            })
        );
    }

    #[gpui::test]
    async fn test_overwrite_events(cx: &mut TestAppContext) {
        let agent = init_test(cx).await;
        let project = agent
            .action_log
            .read_with(cx, |log, _| log.project().clone());
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi", cx));
        let (chunks_tx, chunks_rx) = mpsc::unbounded();
        let (apply, mut events) = agent.overwrite_with_chunks(
            buffer.clone(),
            chunks_rx.map(|chunk: &str| Ok(chunk.to_string())),
            &mut cx.to_async(),
        );

        cx.run_until_parked();
        assert_matches!(
            drain_events(&mut events).as_slice(),
            [EditAgentOutputEvent::Edited(_)]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            ""
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: language::Anchor::MAX
            })
        );

        chunks_tx.unbounded_send("```\njkl\n").unwrap();
        cx.run_until_parked();
        assert_matches!(
            drain_events(&mut events).as_slice(),
            [EditAgentOutputEvent::Edited { .. }]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "jkl"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: language::Anchor::MAX
            })
        );

        chunks_tx.unbounded_send("mno\n").unwrap();
        cx.run_until_parked();
        assert_matches!(
            drain_events(&mut events).as_slice(),
            [EditAgentOutputEvent::Edited { .. }]
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "jkl\nmno"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: language::Anchor::MAX
            })
        );

        chunks_tx.unbounded_send("pqr\n```").unwrap();
        cx.run_until_parked();
        assert_matches!(
            drain_events(&mut events).as_slice(),
            [EditAgentOutputEvent::Edited(_)],
        );
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "jkl\nmno\npqr"
        );
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: language::Anchor::MAX
            })
        );

        drop(chunks_tx);
        apply.await.unwrap();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.snapshot().text()),
            "jkl\nmno\npqr"
        );
        assert_eq!(drain_events(&mut events), vec![]);
        assert_eq!(
            project.read_with(cx, |project, _| project.agent_location()),
            Some(AgentLocation {
                buffer: buffer.downgrade(),
                position: language::Anchor::MAX
            })
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
        let len = rng.random_range(1..=100);
        let new_text = util::RandomCharIter::new(&mut rng)
            .with_simple_text()
            .take(len)
            .collect::<String>();
        let new_text = new_text
            .split('\n')
            .map(|line| format!("{}{}", " ".repeat(rng.random_range(0..=8)), line))
            .collect::<Vec<_>>()
            .join("\n");
        let delta = IndentDelta::Spaces(rng.random_range(-4i8..=4i8) as isize);

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

    fn to_random_chunks(rng: &mut StdRng, input: &str) -> Vec<String> {
        let chunk_count = rng.random_range(1..=cmp::min(input.len(), 50));
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
        agent: &EditAgent,
        output: &str,
        rng: &mut StdRng,
        cx: &mut TestAppContext,
    ) {
        let executor = cx.executor();
        let chunks = to_random_chunks(rng, output);
        let model = agent.model.clone();
        cx.background_spawn(async move {
            for chunk in chunks {
                executor.simulate_random_delay().await;
                model
                    .as_fake()
                    .send_last_completion_stream_text_chunk(chunk);
            }
            model.as_fake().end_last_completion_stream();
        })
        .detach();
    }

    async fn init_test(cx: &mut TestAppContext) -> EditAgent {
        cx.update(settings::init);
        cx.update(Project::init_settings);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let model = Arc::new(FakeLanguageModel::default());
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        EditAgent::new(
            model,
            project,
            action_log,
            Templates::new(),
            EditFormat::XmlTags,
        )
    }

    #[gpui::test(iterations = 10)]
    async fn test_non_unique_text_error(cx: &mut TestAppContext, mut rng: StdRng) {
        let agent = init_test(cx).await;
        let original_text = indoc! {"
                function foo() {
                    return 42;
                }

                function bar() {
                    return 42;
                }

                function baz() {
                    return 42;
                }
            "};
        let buffer = cx.new(|cx| Buffer::local(original_text, cx));
        let (apply, mut events) = agent.edit(
            buffer.clone(),
            String::new(),
            &LanguageModelRequest::default(),
            &mut cx.to_async(),
        );
        cx.run_until_parked();

        // When <old_text> matches text in more than one place
        simulate_llm_output(
            &agent,
            indoc! {"
                <old_text>
                    return 42;
                }
                </old_text>
                <new_text>
                    return 100;
                }
                </new_text>
            "},
            &mut rng,
            cx,
        );
        apply.await.unwrap();

        // Then the text should remain unchanged
        let result_text = buffer.read_with(cx, |buffer, _| buffer.snapshot().text());
        assert_eq!(
            result_text,
            indoc! {"
                function foo() {
                    return 42;
                }

                function bar() {
                    return 42;
                }

                function baz() {
                    return 42;
                }
            "},
            "Text should remain unchanged when there are multiple matches"
        );

        // And AmbiguousEditRange even should be emitted
        let events = drain_events(&mut events);
        let ambiguous_ranges = vec![2..3, 6..7, 10..11];
        assert!(
            events.contains(&EditAgentOutputEvent::AmbiguousEditRange(ambiguous_ranges)),
            "Should emit AmbiguousEditRange for non-unique text"
        );
    }

    fn drain_events(
        stream: &mut UnboundedReceiver<EditAgentOutputEvent>,
    ) -> Vec<EditAgentOutputEvent> {
        let mut events = Vec::new();
        while let Ok(Some(event)) = stream.try_next() {
            events.push(event);
        }
        events
    }
}
