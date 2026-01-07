use crate::{context::LoadedContext, inline_prompt_editor::CodegenStatus};
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result};
use uuid::Uuid;

use cloud_llm_client::CompletionIntent;
use collections::HashSet;
use editor::{Anchor, AnchorRangeExt, MultiBuffer, MultiBufferSnapshot, ToOffset as _, ToPoint};
use feature_flags::{FeatureFlagAppExt as _, InlineAssistantUseToolFeatureFlag};
use futures::{
    SinkExt, Stream, StreamExt, TryStreamExt as _,
    channel::mpsc,
    future::{LocalBoxFuture, Shared},
    join,
    stream::BoxStream,
};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task};
use language::{Buffer, IndentKind, LanguageName, Point, TransactionId, line_diff};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelRequestTool, LanguageModelTextStream, LanguageModelToolChoice,
    LanguageModelToolUse, Role, TokenUsage,
};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use prompt_store::PromptBuilder;
use rope::Rope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use smol::future::FutureExt;
use std::{
    cmp,
    future::Future,
    iter,
    ops::{Range, RangeInclusive},
    pin::Pin,
    sync::Arc,
    task::{self, Poll},
    time::Instant,
};
use streaming_diff::{CharOperation, LineDiff, LineOperation, StreamingDiff};

/// Use this tool when you cannot or should not make a rewrite. This includes:
/// - The user's request is unclear, ambiguous, or nonsensical
/// - The requested change cannot be made by only editing the <rewrite_this> section
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FailureMessageInput {
    /// A brief message to the user explaining why you're unable to fulfill the request or to ask a question about the request.
    #[serde(default)]
    pub message: String,
}

/// Replaces text in <rewrite_this></rewrite_this> tags with your replacement_text.
/// Only use this tool when you are confident you understand the user's request and can fulfill it
/// by editing the marked section.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RewriteSectionInput {
    /// The text to replace the section with.
    #[serde(default)]
    pub replacement_text: String,
}

pub struct BufferCodegen {
    alternatives: Vec<Entity<CodegenAlternative>>,
    pub active_alternative: usize,
    seen_alternatives: HashSet<usize>,
    subscriptions: Vec<Subscription>,
    buffer: Entity<MultiBuffer>,
    range: Range<Anchor>,
    initial_transaction_id: Option<TransactionId>,
    builder: Arc<PromptBuilder>,
    pub is_insertion: bool,
    session_id: Uuid,
}

pub const REWRITE_SECTION_TOOL_NAME: &str = "rewrite_section";
pub const FAILURE_MESSAGE_TOOL_NAME: &str = "failure_message";

impl BufferCodegen {
    pub fn new(
        buffer: Entity<MultiBuffer>,
        range: Range<Anchor>,
        initial_transaction_id: Option<TransactionId>,
        session_id: Uuid,
        builder: Arc<PromptBuilder>,
        cx: &mut Context<Self>,
    ) -> Self {
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                false,
                builder.clone(),
                session_id,
                cx,
            )
        });
        let mut this = Self {
            is_insertion: range.to_offset(&buffer.read(cx).snapshot(cx)).is_empty(),
            alternatives: vec![codegen],
            active_alternative: 0,
            seen_alternatives: HashSet::default(),
            subscriptions: Vec::new(),
            buffer,
            range,
            initial_transaction_id,
            builder,
            session_id,
        };
        this.activate(0, cx);
        this
    }

    fn subscribe_to_alternative(&mut self, cx: &mut Context<Self>) {
        let codegen = self.active_alternative().clone();
        self.subscriptions.clear();
        self.subscriptions
            .push(cx.observe(&codegen, |_, _, cx| cx.notify()));
        self.subscriptions
            .push(cx.subscribe(&codegen, |_, _, event, cx| cx.emit(*event)));
    }

    pub fn active_completion(&self, cx: &App) -> Option<String> {
        self.active_alternative().read(cx).current_completion()
    }

    pub fn active_alternative(&self) -> &Entity<CodegenAlternative> {
        &self.alternatives[self.active_alternative]
    }

    pub fn language_name(&self, cx: &App) -> Option<LanguageName> {
        self.active_alternative().read(cx).language_name(cx)
    }

    pub fn status<'a>(&self, cx: &'a App) -> &'a CodegenStatus {
        &self.active_alternative().read(cx).status
    }

    pub fn alternative_count(&self, cx: &App) -> usize {
        LanguageModelRegistry::read_global(cx)
            .inline_alternative_models()
            .len()
            + 1
    }

    pub fn cycle_prev(&mut self, cx: &mut Context<Self>) {
        let next_active_ix = if self.active_alternative == 0 {
            self.alternatives.len() - 1
        } else {
            self.active_alternative - 1
        };
        self.activate(next_active_ix, cx);
    }

    pub fn cycle_next(&mut self, cx: &mut Context<Self>) {
        let next_active_ix = (self.active_alternative + 1) % self.alternatives.len();
        self.activate(next_active_ix, cx);
    }

    fn activate(&mut self, index: usize, cx: &mut Context<Self>) {
        self.active_alternative()
            .update(cx, |codegen, cx| codegen.set_active(false, cx));
        self.seen_alternatives.insert(index);
        self.active_alternative = index;
        self.active_alternative()
            .update(cx, |codegen, cx| codegen.set_active(true, cx));
        self.subscribe_to_alternative(cx);
        cx.notify();
    }

    pub fn start(
        &mut self,
        primary_model: Arc<dyn LanguageModel>,
        user_prompt: String,
        context_task: Shared<Task<Option<LoadedContext>>>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let alternative_models = LanguageModelRegistry::read_global(cx)
            .inline_alternative_models()
            .to_vec();

        self.active_alternative()
            .update(cx, |alternative, cx| alternative.undo(cx));
        self.activate(0, cx);
        self.alternatives.truncate(1);

        for _ in 0..alternative_models.len() {
            self.alternatives.push(cx.new(|cx| {
                CodegenAlternative::new(
                    self.buffer.clone(),
                    self.range.clone(),
                    false,
                    self.builder.clone(),
                    self.session_id,
                    cx,
                )
            }));
        }

        for (model, alternative) in iter::once(primary_model)
            .chain(alternative_models)
            .zip(&self.alternatives)
        {
            alternative.update(cx, |alternative, cx| {
                alternative.start(user_prompt.clone(), context_task.clone(), model.clone(), cx)
            })?;
        }

        Ok(())
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        for codegen in &self.alternatives {
            codegen.update(cx, |codegen, cx| codegen.stop(cx));
        }
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        self.active_alternative()
            .update(cx, |codegen, cx| codegen.undo(cx));

        self.buffer.update(cx, |buffer, cx| {
            if let Some(transaction_id) = self.initial_transaction_id.take() {
                buffer.undo_transaction(transaction_id, cx);
                buffer.refresh_preview(cx);
            }
        });
    }

    pub fn buffer(&self, cx: &App) -> Entity<MultiBuffer> {
        self.active_alternative().read(cx).buffer.clone()
    }

    pub fn old_buffer(&self, cx: &App) -> Entity<Buffer> {
        self.active_alternative().read(cx).old_buffer.clone()
    }

    pub fn snapshot(&self, cx: &App) -> MultiBufferSnapshot {
        self.active_alternative().read(cx).snapshot.clone()
    }

    pub fn edit_position(&self, cx: &App) -> Option<Anchor> {
        self.active_alternative().read(cx).edit_position
    }

    pub fn diff<'a>(&self, cx: &'a App) -> &'a Diff {
        &self.active_alternative().read(cx).diff
    }

    pub fn last_equal_ranges<'a>(&self, cx: &'a App) -> &'a [Range<Anchor>] {
        self.active_alternative().read(cx).last_equal_ranges()
    }

    pub fn selected_text<'a>(&self, cx: &'a App) -> Option<&'a str> {
        self.active_alternative().read(cx).selected_text()
    }

    pub fn session_id(&self) -> Uuid {
        self.session_id
    }
}

impl EventEmitter<CodegenEvent> for BufferCodegen {}

pub struct CodegenAlternative {
    buffer: Entity<MultiBuffer>,
    old_buffer: Entity<Buffer>,
    snapshot: MultiBufferSnapshot,
    edit_position: Option<Anchor>,
    range: Range<Anchor>,
    last_equal_ranges: Vec<Range<Anchor>>,
    transformation_transaction_id: Option<TransactionId>,
    status: CodegenStatus,
    generation: Task<()>,
    diff: Diff,
    _subscription: gpui::Subscription,
    builder: Arc<PromptBuilder>,
    active: bool,
    edits: Vec<(Range<Anchor>, String)>,
    line_operations: Vec<LineOperation>,
    elapsed_time: Option<f64>,
    completion: Option<String>,
    selected_text: Option<String>,
    pub message_id: Option<String>,
    session_id: Uuid,
    pub description: Option<String>,
    pub failure: Option<String>,
}

impl EventEmitter<CodegenEvent> for CodegenAlternative {}

impl CodegenAlternative {
    pub fn new(
        buffer: Entity<MultiBuffer>,
        range: Range<Anchor>,
        active: bool,
        builder: Arc<PromptBuilder>,
        session_id: Uuid,
        cx: &mut Context<Self>,
    ) -> Self {
        let snapshot = buffer.read(cx).snapshot(cx);

        let (old_buffer, _, _) = snapshot
            .range_to_buffer_ranges(range.clone())
            .pop()
            .unwrap();
        let old_buffer = cx.new(|cx| {
            let text = old_buffer.as_rope().clone();
            let line_ending = old_buffer.line_ending();
            let language = old_buffer.language().cloned();
            let language_registry = buffer
                .read(cx)
                .buffer(old_buffer.remote_id())
                .unwrap()
                .read(cx)
                .language_registry();

            let mut buffer = Buffer::local_normalized(text, line_ending, cx);
            buffer.set_language(language, cx);
            if let Some(language_registry) = language_registry {
                buffer.set_language_registry(language_registry);
            }
            buffer
        });

        Self {
            buffer: buffer.clone(),
            old_buffer,
            edit_position: None,
            message_id: None,
            snapshot,
            last_equal_ranges: Default::default(),
            transformation_transaction_id: None,
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
            diff: Diff::default(),
            builder,
            active: active,
            edits: Vec::new(),
            line_operations: Vec::new(),
            range,
            elapsed_time: None,
            completion: None,
            selected_text: None,
            session_id,
            description: None,
            failure: None,
            _subscription: cx.subscribe(&buffer, Self::handle_buffer_event),
        }
    }

    pub fn language_name(&self, cx: &App) -> Option<LanguageName> {
        self.old_buffer
            .read(cx)
            .language()
            .map(|language| language.name())
    }

    pub fn set_active(&mut self, active: bool, cx: &mut Context<Self>) {
        if active != self.active {
            self.active = active;

            if self.active {
                let edits = self.edits.clone();
                self.apply_edits(edits, cx);
                if matches!(self.status, CodegenStatus::Pending) {
                    let line_operations = self.line_operations.clone();
                    self.reapply_line_based_diff(line_operations, cx);
                } else {
                    self.reapply_batch_diff(cx).detach();
                }
            } else if let Some(transaction_id) = self.transformation_transaction_id.take() {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.undo_transaction(transaction_id, cx);
                    buffer.forget_transaction(transaction_id, cx);
                });
            }
        }
    }

    fn handle_buffer_event(
        &mut self,
        _buffer: Entity<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut Context<Self>,
    ) {
        if let multi_buffer::Event::TransactionUndone { transaction_id } = event
            && self.transformation_transaction_id == Some(*transaction_id)
        {
            self.transformation_transaction_id = None;
            self.generation = Task::ready(());
            cx.emit(CodegenEvent::Undone);
        }
    }

    pub fn last_equal_ranges(&self) -> &[Range<Anchor>] {
        &self.last_equal_ranges
    }

    pub fn use_streaming_tools(model: &dyn LanguageModel, cx: &App) -> bool {
        model.supports_streaming_tools()
            && cx.has_flag::<InlineAssistantUseToolFeatureFlag>()
            && AgentSettings::get_global(cx).inline_assistant_use_streaming_tools
    }

    pub fn start(
        &mut self,
        user_prompt: String,
        context_task: Shared<Task<Option<LoadedContext>>>,
        model: Arc<dyn LanguageModel>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        // Clear the model explanation since the user has started a new generation.
        self.description = None;

        if let Some(transformation_transaction_id) = self.transformation_transaction_id.take() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.undo_transaction(transformation_transaction_id, cx);
            });
        }

        self.edit_position = Some(self.range.start.bias_right(&self.snapshot));

        if Self::use_streaming_tools(model.as_ref(), cx) {
            let request = self.build_request(&model, user_prompt, context_task, cx)?;
            let completion_events = cx.spawn({
                let model = model.clone();
                async move |_, cx| model.stream_completion(request.await, cx).await
            });
            self.generation = self.handle_completion(model, completion_events, cx);
        } else {
            let stream: LocalBoxFuture<Result<LanguageModelTextStream>> =
                if user_prompt.trim().to_lowercase() == "delete" {
                    async { Ok(LanguageModelTextStream::default()) }.boxed_local()
                } else {
                    let request = self.build_request(&model, user_prompt, context_task, cx)?;
                    cx.spawn({
                        let model = model.clone();
                        async move |_, cx| {
                            Ok(model.stream_completion_text(request.await, cx).await?)
                        }
                    })
                    .boxed_local()
                };
            self.generation =
                self.handle_stream(model, /* strip_invalid_spans: */ true, stream, cx);
        }

        Ok(())
    }

    fn build_request_tools(
        &self,
        model: &Arc<dyn LanguageModel>,
        user_prompt: String,
        context_task: Shared<Task<Option<LoadedContext>>>,
        cx: &mut App,
    ) -> Result<Task<LanguageModelRequest>> {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let language = buffer.language_at(self.range.start);
        let language_name = if let Some(language) = language.as_ref() {
            if Arc::ptr_eq(language, &language::PLAIN_TEXT) {
                None
            } else {
                Some(language.name())
            }
        } else {
            None
        };

        let language_name = language_name.as_ref();
        let start = buffer.point_to_buffer_offset(self.range.start);
        let end = buffer.point_to_buffer_offset(self.range.end);
        let (buffer, range) = if let Some((start, end)) = start.zip(end) {
            let (start_buffer, start_buffer_offset) = start;
            let (end_buffer, end_buffer_offset) = end;
            if start_buffer.remote_id() == end_buffer.remote_id() {
                (start_buffer.clone(), start_buffer_offset..end_buffer_offset)
            } else {
                anyhow::bail!("invalid transformation range");
            }
        } else {
            anyhow::bail!("invalid transformation range");
        };

        let system_prompt = self
            .builder
            .generate_inline_transformation_prompt_tools(
                language_name,
                buffer,
                range.start.0..range.end.0,
            )
            .context("generating content prompt")?;

        let temperature = AgentSettings::temperature_for_model(model, cx);

        let tool_input_format = model.tool_input_format();
        let tool_choice = model
            .supports_tool_choice(LanguageModelToolChoice::Any)
            .then_some(LanguageModelToolChoice::Any);

        Ok(cx.spawn(async move |_cx| {
            let mut messages = vec![LanguageModelRequestMessage {
                role: Role::System,
                content: vec![system_prompt.into()],
                cache: false,
                reasoning_details: None,
            }];

            let mut user_message = LanguageModelRequestMessage {
                role: Role::User,
                content: Vec::new(),
                cache: false,
                reasoning_details: None,
            };

            if let Some(context) = context_task.await {
                context.add_to_request_message(&mut user_message);
            }

            user_message.content.push(user_prompt.into());
            messages.push(user_message);

            let tools = vec![
                LanguageModelRequestTool {
                    name: REWRITE_SECTION_TOOL_NAME.to_string(),
                    description: "Replaces text in <rewrite_this></rewrite_this> tags with your replacement_text.".to_string(),
                    input_schema: language_model::tool_schema::root_schema_for::<RewriteSectionInput>(tool_input_format).to_value(),
                },
                LanguageModelRequestTool {
                    name: FAILURE_MESSAGE_TOOL_NAME.to_string(),
                    description: "Use this tool to provide a message to the user when you're unable to complete a task.".to_string(),
                    input_schema: language_model::tool_schema::root_schema_for::<FailureMessageInput>(tool_input_format).to_value(),
                },
            ];

            LanguageModelRequest {
                thread_id: None,
                prompt_id: None,
                intent: Some(CompletionIntent::InlineAssist),
                mode: None,
                tools,
                tool_choice,
                stop: Vec::new(),
                temperature,
                messages,
                thinking_allowed: false,
            }
        }))
    }

    fn build_request(
        &self,
        model: &Arc<dyn LanguageModel>,
        user_prompt: String,
        context_task: Shared<Task<Option<LoadedContext>>>,
        cx: &mut App,
    ) -> Result<Task<LanguageModelRequest>> {
        if Self::use_streaming_tools(model.as_ref(), cx) {
            return self.build_request_tools(model, user_prompt, context_task, cx);
        }

        let buffer = self.buffer.read(cx).snapshot(cx);
        let language = buffer.language_at(self.range.start);
        let language_name = if let Some(language) = language.as_ref() {
            if Arc::ptr_eq(language, &language::PLAIN_TEXT) {
                None
            } else {
                Some(language.name())
            }
        } else {
            None
        };

        let language_name = language_name.as_ref();
        let start = buffer.point_to_buffer_offset(self.range.start);
        let end = buffer.point_to_buffer_offset(self.range.end);
        let (buffer, range) = if let Some((start, end)) = start.zip(end) {
            let (start_buffer, start_buffer_offset) = start;
            let (end_buffer, end_buffer_offset) = end;
            if start_buffer.remote_id() == end_buffer.remote_id() {
                (start_buffer.clone(), start_buffer_offset..end_buffer_offset)
            } else {
                anyhow::bail!("invalid transformation range");
            }
        } else {
            anyhow::bail!("invalid transformation range");
        };

        let prompt = self
            .builder
            .generate_inline_transformation_prompt(
                user_prompt,
                language_name,
                buffer,
                range.start.0..range.end.0,
            )
            .context("generating content prompt")?;

        let temperature = AgentSettings::temperature_for_model(model, cx);

        Ok(cx.spawn(async move |_cx| {
            let mut request_message = LanguageModelRequestMessage {
                role: Role::User,
                content: Vec::new(),
                cache: false,
                reasoning_details: None,
            };

            if let Some(context) = context_task.await {
                context.add_to_request_message(&mut request_message);
            }

            request_message.content.push(prompt.into());

            LanguageModelRequest {
                thread_id: None,
                prompt_id: None,
                intent: Some(CompletionIntent::InlineAssist),
                mode: None,
                tools: Vec::new(),
                tool_choice: None,
                stop: Vec::new(),
                temperature,
                messages: vec![request_message],
                thinking_allowed: false,
            }
        }))
    }

    pub fn handle_stream(
        &mut self,
        model: Arc<dyn LanguageModel>,
        strip_invalid_spans: bool,
        stream: impl 'static + Future<Output = Result<LanguageModelTextStream>>,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let anthropic_reporter = language_model::AnthropicEventReporter::new(&model, cx);
        let session_id = self.session_id;
        let model_telemetry_id = model.telemetry_id();
        let model_provider_id = model.provider_id().to_string();
        let start_time = Instant::now();

        // Make a new snapshot and re-resolve anchor in case the document was modified.
        // This can happen often if the editor loses focus and is saved + reformatted,
        // as in https://github.com/zed-industries/zed/issues/39088
        self.snapshot = self.buffer.read(cx).snapshot(cx);
        self.range = self.snapshot.anchor_after(self.range.start)
            ..self.snapshot.anchor_after(self.range.end);

        let snapshot = self.snapshot.clone();
        let selected_text = snapshot
            .text_for_range(self.range.start..self.range.end)
            .collect::<Rope>();

        self.selected_text = Some(selected_text.to_string());

        let selection_start = self.range.start.to_point(&snapshot);

        // Start with the indentation of the first line in the selection
        let mut suggested_line_indent = snapshot
            .suggested_indents(selection_start.row..=selection_start.row, cx)
            .into_values()
            .next()
            .unwrap_or_else(|| snapshot.indent_size_for_line(MultiBufferRow(selection_start.row)));

        // If the first line in the selection does not have indentation, check the following lines
        if suggested_line_indent.len == 0 && suggested_line_indent.kind == IndentKind::Space {
            for row in selection_start.row..=self.range.end.to_point(&snapshot).row {
                let line_indent = snapshot.indent_size_for_line(MultiBufferRow(row));
                // Prefer tabs if a line in the selection uses tabs as indentation
                if line_indent.kind == IndentKind::Tab {
                    suggested_line_indent.kind = IndentKind::Tab;
                    break;
                }
            }
        }

        let language_name = {
            let multibuffer = self.buffer.read(cx);
            let snapshot = multibuffer.snapshot(cx);
            let ranges = snapshot.range_to_buffer_ranges(self.range.clone());
            ranges
                .first()
                .and_then(|(buffer, _, _)| buffer.language())
                .map(|language| language.name())
        };

        self.diff = Diff::default();
        self.status = CodegenStatus::Pending;
        let mut edit_start = self.range.start.to_offset(&snapshot);
        let completion = Arc::new(Mutex::new(String::new()));
        let completion_clone = completion.clone();

        cx.notify();
        cx.spawn(async move |codegen, cx| {
            let stream = stream.await;

            let token_usage = stream
                .as_ref()
                .ok()
                .map(|stream| stream.last_token_usage.clone());
            let message_id = stream
                .as_ref()
                .ok()
                .and_then(|stream| stream.message_id.clone());
            let generate = async {
                let model_telemetry_id = model_telemetry_id.clone();
                let model_provider_id = model_provider_id.clone();
                let (mut diff_tx, mut diff_rx) = mpsc::channel(1);
                let message_id = message_id.clone();
                let line_based_stream_diff: Task<anyhow::Result<()>> = cx.background_spawn({
                    let anthropic_reporter = anthropic_reporter.clone();
                    let language_name = language_name.clone();
                    async move {
                        let mut response_latency = None;
                        let request_start = Instant::now();
                        let diff = async {
                            let raw_stream = stream?.stream.map_err(|error| error.into());

                            let stripped;
                            let mut chunks: Pin<Box<dyn Stream<Item = Result<String>> + Send>> =
                                if strip_invalid_spans {
                                    stripped = StripInvalidSpans::new(raw_stream);
                                    Box::pin(stripped)
                                } else {
                                    Box::pin(raw_stream)
                                };

                            let mut diff = StreamingDiff::new(selected_text.to_string());
                            let mut line_diff = LineDiff::default();

                            let mut new_text = String::new();
                            let mut base_indent = None;
                            let mut line_indent = None;
                            let mut first_line = true;

                            while let Some(chunk) = chunks.next().await {
                                if response_latency.is_none() {
                                    response_latency = Some(request_start.elapsed());
                                }
                                let chunk = chunk?;
                                completion_clone.lock().push_str(&chunk);

                                let mut lines = chunk.split('\n').peekable();
                                while let Some(line) = lines.next() {
                                    new_text.push_str(line);
                                    if line_indent.is_none()
                                        && let Some(non_whitespace_ch_ix) =
                                            new_text.find(|ch: char| !ch.is_whitespace())
                                    {
                                        line_indent = Some(non_whitespace_ch_ix);
                                        base_indent = base_indent.or(line_indent);

                                        let line_indent = line_indent.unwrap();
                                        let base_indent = base_indent.unwrap();
                                        let indent_delta = line_indent as i32 - base_indent as i32;
                                        let mut corrected_indent_len = cmp::max(
                                            0,
                                            suggested_line_indent.len as i32 + indent_delta,
                                        )
                                            as usize;
                                        if first_line {
                                            corrected_indent_len = corrected_indent_len
                                                .saturating_sub(selection_start.column as usize);
                                        }

                                        let indent_char = suggested_line_indent.char();
                                        let mut indent_buffer = [0; 4];
                                        let indent_str =
                                            indent_char.encode_utf8(&mut indent_buffer);
                                        new_text.replace_range(
                                            ..line_indent,
                                            &indent_str.repeat(corrected_indent_len),
                                        );
                                    }

                                    if line_indent.is_some() {
                                        let char_ops = diff.push_new(&new_text);
                                        line_diff.push_char_operations(&char_ops, &selected_text);
                                        diff_tx
                                            .send((char_ops, line_diff.line_operations()))
                                            .await?;
                                        new_text.clear();
                                    }

                                    if lines.peek().is_some() {
                                        let char_ops = diff.push_new("\n");
                                        line_diff.push_char_operations(&char_ops, &selected_text);
                                        diff_tx
                                            .send((char_ops, line_diff.line_operations()))
                                            .await?;
                                        if line_indent.is_none() {
                                            // Don't write out the leading indentation in empty lines on the next line
                                            // This is the case where the above if statement didn't clear the buffer
                                            new_text.clear();
                                        }
                                        line_indent = None;
                                        first_line = false;
                                    }
                                }
                            }

                            let mut char_ops = diff.push_new(&new_text);
                            char_ops.extend(diff.finish());
                            line_diff.push_char_operations(&char_ops, &selected_text);
                            line_diff.finish(&selected_text);
                            diff_tx
                                .send((char_ops, line_diff.line_operations()))
                                .await?;

                            anyhow::Ok(())
                        };

                        let result = diff.await;

                        let error_message = result.as_ref().err().map(|error| error.to_string());
                        telemetry::event!(
                            "Assistant Responded",
                            kind = "inline",
                            phase = "response",
                            session_id = session_id.to_string(),
                            model = model_telemetry_id,
                            model_provider = model_provider_id,
                            language_name = language_name.as_ref().map(|n| n.to_string()),
                            message_id = message_id.as_deref(),
                            response_latency = response_latency,
                            error_message = error_message.as_deref(),
                        );

                        anthropic_reporter.report(language_model::AnthropicEventData {
                            completion_type: language_model::AnthropicCompletionType::Editor,
                            event: language_model::AnthropicEventType::Response,
                            language_name: language_name.map(|n| n.to_string()),
                            message_id,
                        });

                        result?;
                        Ok(())
                    }
                });

                while let Some((char_ops, line_ops)) = diff_rx.next().await {
                    codegen.update(cx, |codegen, cx| {
                        codegen.last_equal_ranges.clear();

                        let edits = char_ops
                            .into_iter()
                            .filter_map(|operation| match operation {
                                CharOperation::Insert { text } => {
                                    let edit_start = snapshot.anchor_after(edit_start);
                                    Some((edit_start..edit_start, text))
                                }
                                CharOperation::Delete { bytes } => {
                                    let edit_end = edit_start + bytes;
                                    let edit_range = snapshot.anchor_after(edit_start)
                                        ..snapshot.anchor_before(edit_end);
                                    edit_start = edit_end;
                                    Some((edit_range, String::new()))
                                }
                                CharOperation::Keep { bytes } => {
                                    let edit_end = edit_start + bytes;
                                    let edit_range = snapshot.anchor_after(edit_start)
                                        ..snapshot.anchor_before(edit_end);
                                    edit_start = edit_end;
                                    codegen.last_equal_ranges.push(edit_range);
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        if codegen.active {
                            codegen.apply_edits(edits.iter().cloned(), cx);
                            codegen.reapply_line_based_diff(line_ops.iter().cloned(), cx);
                        }
                        codegen.edits.extend(edits);
                        codegen.line_operations = line_ops;
                        codegen.edit_position = Some(snapshot.anchor_after(edit_start));

                        cx.notify();
                    })?;
                }

                // Streaming stopped and we have the new text in the buffer, and a line-based diff applied for the whole new buffer.
                // That diff is not what a regular diff is and might look unexpected, ergo apply a regular diff.
                // It's fine to apply even if the rest of the line diffing fails, as no more hunks are coming through `diff_rx`.
                let batch_diff_task =
                    codegen.update(cx, |codegen, cx| codegen.reapply_batch_diff(cx))?;
                let (line_based_stream_diff, ()) = join!(line_based_stream_diff, batch_diff_task);
                line_based_stream_diff?;

                anyhow::Ok(())
            };

            let result = generate.await;
            let elapsed_time = start_time.elapsed().as_secs_f64();

            codegen
                .update(cx, |this, cx| {
                    this.message_id = message_id;
                    this.last_equal_ranges.clear();
                    if let Err(error) = result {
                        this.status = CodegenStatus::Error(error);
                    } else {
                        this.status = CodegenStatus::Done;
                    }
                    this.elapsed_time = Some(elapsed_time);
                    this.completion = Some(completion.lock().clone());
                    if let Some(usage) = token_usage {
                        let usage = usage.lock();
                        telemetry::event!(
                            "Inline Assistant Completion",
                            model = model_telemetry_id,
                            model_provider = model_provider_id,
                            input_tokens = usage.input_tokens,
                            output_tokens = usage.output_tokens,
                        )
                    }

                    cx.emit(CodegenEvent::Finished);
                    cx.notify();
                })
                .ok();
        })
    }

    pub fn current_completion(&self) -> Option<String> {
        self.completion.clone()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn current_description(&self) -> Option<String> {
        self.description.clone()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn current_failure(&self) -> Option<String> {
        self.failure.clone()
    }

    pub fn selected_text(&self) -> Option<&str> {
        self.selected_text.as_deref()
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.last_equal_ranges.clear();
        if self.diff.is_empty() {
            self.status = CodegenStatus::Idle;
        } else {
            self.status = CodegenStatus::Done;
        }
        self.generation = Task::ready(());
        cx.emit(CodegenEvent::Finished);
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        self.buffer.update(cx, |buffer, cx| {
            if let Some(transaction_id) = self.transformation_transaction_id.take() {
                buffer.undo_transaction(transaction_id, cx);
                buffer.refresh_preview(cx);
            }
        });
    }

    fn apply_edits(
        &mut self,
        edits: impl IntoIterator<Item = (Range<Anchor>, String)>,
        cx: &mut Context<CodegenAlternative>,
    ) {
        let transaction = self.buffer.update(cx, |buffer, cx| {
            // Avoid grouping agent edits with user edits.
            buffer.finalize_last_transaction(cx);
            buffer.start_transaction(cx);
            buffer.edit(edits, None, cx);
            buffer.end_transaction(cx)
        });

        if let Some(transaction) = transaction {
            if let Some(first_transaction) = self.transformation_transaction_id {
                // Group all agent edits into the first transaction.
                self.buffer.update(cx, |buffer, cx| {
                    buffer.merge_transactions(transaction, first_transaction, cx)
                });
            } else {
                self.transformation_transaction_id = Some(transaction);
                self.buffer
                    .update(cx, |buffer, cx| buffer.finalize_last_transaction(cx));
            }
        }
    }

    fn reapply_line_based_diff(
        &mut self,
        line_operations: impl IntoIterator<Item = LineOperation>,
        cx: &mut Context<Self>,
    ) {
        let old_snapshot = self.snapshot.clone();
        let old_range = self.range.to_point(&old_snapshot);
        let new_snapshot = self.buffer.read(cx).snapshot(cx);
        let new_range = self.range.to_point(&new_snapshot);

        let mut old_row = old_range.start.row;
        let mut new_row = new_range.start.row;

        self.diff.deleted_row_ranges.clear();
        self.diff.inserted_row_ranges.clear();
        for operation in line_operations {
            match operation {
                LineOperation::Keep { lines } => {
                    old_row += lines;
                    new_row += lines;
                }
                LineOperation::Delete { lines } => {
                    let old_end_row = old_row + lines - 1;
                    let new_row = new_snapshot.anchor_before(Point::new(new_row, 0));

                    if let Some((_, last_deleted_row_range)) =
                        self.diff.deleted_row_ranges.last_mut()
                    {
                        if *last_deleted_row_range.end() + 1 == old_row {
                            *last_deleted_row_range = *last_deleted_row_range.start()..=old_end_row;
                        } else {
                            self.diff
                                .deleted_row_ranges
                                .push((new_row, old_row..=old_end_row));
                        }
                    } else {
                        self.diff
                            .deleted_row_ranges
                            .push((new_row, old_row..=old_end_row));
                    }

                    old_row += lines;
                }
                LineOperation::Insert { lines } => {
                    let new_end_row = new_row + lines - 1;
                    let start = new_snapshot.anchor_before(Point::new(new_row, 0));
                    let end = new_snapshot.anchor_before(Point::new(
                        new_end_row,
                        new_snapshot.line_len(MultiBufferRow(new_end_row)),
                    ));
                    self.diff.inserted_row_ranges.push(start..end);
                    new_row += lines;
                }
            }

            cx.notify();
        }
    }

    fn reapply_batch_diff(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let old_snapshot = self.snapshot.clone();
        let old_range = self.range.to_point(&old_snapshot);
        let new_snapshot = self.buffer.read(cx).snapshot(cx);
        let new_range = self.range.to_point(&new_snapshot);

        cx.spawn(async move |codegen, cx| {
            let (deleted_row_ranges, inserted_row_ranges) = cx
                .background_spawn(async move {
                    let old_text = old_snapshot
                        .text_for_range(
                            Point::new(old_range.start.row, 0)
                                ..Point::new(
                                    old_range.end.row,
                                    old_snapshot.line_len(MultiBufferRow(old_range.end.row)),
                                ),
                        )
                        .collect::<String>();
                    let new_text = new_snapshot
                        .text_for_range(
                            Point::new(new_range.start.row, 0)
                                ..Point::new(
                                    new_range.end.row,
                                    new_snapshot.line_len(MultiBufferRow(new_range.end.row)),
                                ),
                        )
                        .collect::<String>();

                    let old_start_row = old_range.start.row;
                    let new_start_row = new_range.start.row;
                    let mut deleted_row_ranges: Vec<(Anchor, RangeInclusive<u32>)> = Vec::new();
                    let mut inserted_row_ranges = Vec::new();
                    for (old_rows, new_rows) in line_diff(&old_text, &new_text) {
                        let old_rows = old_start_row + old_rows.start..old_start_row + old_rows.end;
                        let new_rows = new_start_row + new_rows.start..new_start_row + new_rows.end;
                        if !old_rows.is_empty() {
                            deleted_row_ranges.push((
                                new_snapshot.anchor_before(Point::new(new_rows.start, 0)),
                                old_rows.start..=old_rows.end - 1,
                            ));
                        }
                        if !new_rows.is_empty() {
                            let start = new_snapshot.anchor_before(Point::new(new_rows.start, 0));
                            let new_end_row = new_rows.end - 1;
                            let end = new_snapshot.anchor_before(Point::new(
                                new_end_row,
                                new_snapshot.line_len(MultiBufferRow(new_end_row)),
                            ));
                            inserted_row_ranges.push(start..end);
                        }
                    }
                    (deleted_row_ranges, inserted_row_ranges)
                })
                .await;

            codegen
                .update(cx, |codegen, cx| {
                    codegen.diff.deleted_row_ranges = deleted_row_ranges;
                    codegen.diff.inserted_row_ranges = inserted_row_ranges;
                    cx.notify();
                })
                .ok();
        })
    }

    fn handle_completion(
        &mut self,
        model: Arc<dyn LanguageModel>,
        completion_stream: Task<
            Result<
                BoxStream<
                    'static,
                    Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                >,
                LanguageModelCompletionError,
            >,
        >,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        self.diff = Diff::default();
        self.status = CodegenStatus::Pending;

        cx.notify();
        // Leaving this in generation so that STOP equivalent events are respected even
        // while we're still pre-processing the completion event
        cx.spawn(async move |codegen, cx| {
            let finish_with_status = |status: CodegenStatus, cx: &mut AsyncApp| {
                let _ = codegen.update(cx, |this, cx| {
                    this.status = status;
                    cx.emit(CodegenEvent::Finished);
                    cx.notify();
                });
            };

            let mut completion_events = match completion_stream.await {
                Ok(events) => events,
                Err(err) => {
                    finish_with_status(CodegenStatus::Error(err.into()), cx);
                    return;
                }
            };

            enum ToolUseOutput {
                Rewrite {
                    text: String,
                    description: Option<String>,
                },
                Failure(String),
            }

            enum ModelUpdate {
                Description(String),
                Failure(String),
            }

            let chars_read_so_far = Arc::new(Mutex::new(0usize));
            let process_tool_use = move |tool_use: LanguageModelToolUse| -> Option<ToolUseOutput> {
                let mut chars_read_so_far = chars_read_so_far.lock();
                match tool_use.name.as_ref() {
                    REWRITE_SECTION_TOOL_NAME => {
                        let Ok(input) =
                            serde_json::from_value::<RewriteSectionInput>(tool_use.input)
                        else {
                            return None;
                        };
                        let text = input.replacement_text[*chars_read_so_far..].to_string();
                        *chars_read_so_far = input.replacement_text.len();
                        Some(ToolUseOutput::Rewrite {
                            text,
                            description: None,
                        })
                    }
                    FAILURE_MESSAGE_TOOL_NAME => {
                        let Ok(mut input) =
                            serde_json::from_value::<FailureMessageInput>(tool_use.input)
                        else {
                            return None;
                        };
                        Some(ToolUseOutput::Failure(std::mem::take(&mut input.message)))
                    }
                    _ => None,
                }
            };

            let (message_tx, mut message_rx) = futures::channel::mpsc::unbounded::<ModelUpdate>();

            cx.spawn({
                let codegen = codegen.clone();
                async move |cx| {
                    while let Some(update) = message_rx.next().await {
                        let _ = codegen.update(cx, |this, _cx| match update {
                            ModelUpdate::Description(d) => this.description = Some(d),
                            ModelUpdate::Failure(f) => this.failure = Some(f),
                        });
                    }
                }
            })
            .detach();

            let mut message_id = None;
            let mut first_text = None;
            let last_token_usage = Arc::new(Mutex::new(TokenUsage::default()));
            let total_text = Arc::new(Mutex::new(String::new()));

            loop {
                if let Some(first_event) = completion_events.next().await {
                    match first_event {
                        Ok(LanguageModelCompletionEvent::StartMessage { message_id: id }) => {
                            message_id = Some(id);
                        }
                        Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                            if let Some(output) = process_tool_use(tool_use) {
                                let (text, update) = match output {
                                    ToolUseOutput::Rewrite { text, description } => {
                                        (Some(text), description.map(ModelUpdate::Description))
                                    }
                                    ToolUseOutput::Failure(message) => {
                                        (None, Some(ModelUpdate::Failure(message)))
                                    }
                                };
                                if let Some(update) = update {
                                    let _ = message_tx.unbounded_send(update);
                                }
                                first_text = text;
                                if first_text.is_some() {
                                    break;
                                }
                            }
                        }
                        Ok(LanguageModelCompletionEvent::UsageUpdate(token_usage)) => {
                            *last_token_usage.lock() = token_usage;
                        }
                        Ok(LanguageModelCompletionEvent::Text(text)) => {
                            let mut lock = total_text.lock();
                            lock.push_str(&text);
                        }
                        Ok(e) => {
                            log::warn!("Unexpected event: {:?}", e);
                            break;
                        }
                        Err(e) => {
                            finish_with_status(CodegenStatus::Error(e.into()), cx);
                            break;
                        }
                    }
                }
            }

            let Some(first_text) = first_text else {
                finish_with_status(CodegenStatus::Done, cx);
                return;
            };

            let move_last_token_usage = last_token_usage.clone();

            let text_stream = Box::pin(futures::stream::once(async { Ok(first_text) }).chain(
                completion_events.filter_map(move |e| {
                    let process_tool_use = process_tool_use.clone();
                    let last_token_usage = move_last_token_usage.clone();
                    let total_text = total_text.clone();
                    let mut message_tx = message_tx.clone();
                    async move {
                        match e {
                            Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                                let Some(output) = process_tool_use(tool_use) else {
                                    return None;
                                };
                                let (text, update) = match output {
                                    ToolUseOutput::Rewrite { text, description } => {
                                        (Some(text), description.map(ModelUpdate::Description))
                                    }
                                    ToolUseOutput::Failure(message) => {
                                        (None, Some(ModelUpdate::Failure(message)))
                                    }
                                };
                                if let Some(update) = update {
                                    let _ = message_tx.send(update).await;
                                }
                                text.map(Ok)
                            }
                            Ok(LanguageModelCompletionEvent::UsageUpdate(token_usage)) => {
                                *last_token_usage.lock() = token_usage;
                                None
                            }
                            Ok(LanguageModelCompletionEvent::Text(text)) => {
                                let mut lock = total_text.lock();
                                lock.push_str(&text);
                                None
                            }
                            Ok(LanguageModelCompletionEvent::Stop(_reason)) => None,
                            e => {
                                log::error!("UNEXPECTED EVENT {:?}", e);
                                None
                            }
                        }
                    }
                }),
            ));

            let language_model_text_stream = LanguageModelTextStream {
                message_id: message_id,
                stream: text_stream,
                last_token_usage,
            };

            let Some(task) = codegen
                .update(cx, move |codegen, cx| {
                    codegen.handle_stream(
                        model,
                        /* strip_invalid_spans: */ false,
                        async { Ok(language_model_text_stream) },
                        cx,
                    )
                })
                .ok()
            else {
                return;
            };

            task.await;
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CodegenEvent {
    Finished,
    Undone,
}

struct StripInvalidSpans<T> {
    stream: T,
    stream_done: bool,
    buffer: String,
    first_line: bool,
    line_end: bool,
    starts_with_code_block: bool,
}

impl<T> StripInvalidSpans<T>
where
    T: Stream<Item = Result<String>>,
{
    fn new(stream: T) -> Self {
        Self {
            stream,
            stream_done: false,
            buffer: String::new(),
            first_line: true,
            line_end: false,
            starts_with_code_block: false,
        }
    }
}

impl<T> Stream for StripInvalidSpans<T>
where
    T: Stream<Item = Result<String>>,
{
    type Item = Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Option<Self::Item>> {
        const CODE_BLOCK_DELIMITER: &str = "```";
        const CURSOR_SPAN: &str = "<|CURSOR|>";

        let this = unsafe { self.get_unchecked_mut() };
        loop {
            if !this.stream_done {
                let mut stream = unsafe { Pin::new_unchecked(&mut this.stream) };
                match stream.as_mut().poll_next(cx) {
                    Poll::Ready(Some(Ok(chunk))) => {
                        this.buffer.push_str(&chunk);
                    }
                    Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(error))),
                    Poll::Ready(None) => {
                        this.stream_done = true;
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }

            let mut chunk = String::new();
            let mut consumed = 0;
            if !this.buffer.is_empty() {
                let mut lines = this.buffer.split('\n').enumerate().peekable();
                while let Some((line_ix, line)) = lines.next() {
                    if line_ix > 0 {
                        this.first_line = false;
                    }

                    if this.first_line {
                        let trimmed_line = line.trim();
                        if lines.peek().is_some() {
                            if trimmed_line.starts_with(CODE_BLOCK_DELIMITER) {
                                consumed += line.len() + 1;
                                this.starts_with_code_block = true;
                                continue;
                            }
                        } else if trimmed_line.is_empty()
                            || prefixes(CODE_BLOCK_DELIMITER)
                                .any(|prefix| trimmed_line.starts_with(prefix))
                        {
                            break;
                        }
                    }

                    let line_without_cursor = line.replace(CURSOR_SPAN, "");
                    if lines.peek().is_some() {
                        if this.line_end {
                            chunk.push('\n');
                        }

                        chunk.push_str(&line_without_cursor);
                        this.line_end = true;
                        consumed += line.len() + 1;
                    } else if this.stream_done {
                        if !this.starts_with_code_block
                            || !line_without_cursor.trim().ends_with(CODE_BLOCK_DELIMITER)
                        {
                            if this.line_end {
                                chunk.push('\n');
                            }

                            chunk.push_str(line);
                        }

                        consumed += line.len();
                    } else {
                        let trimmed_line = line.trim();
                        if trimmed_line.is_empty()
                            || prefixes(CURSOR_SPAN).any(|prefix| trimmed_line.ends_with(prefix))
                            || prefixes(CODE_BLOCK_DELIMITER)
                                .any(|prefix| trimmed_line.ends_with(prefix))
                        {
                            break;
                        } else {
                            if this.line_end {
                                chunk.push('\n');
                                this.line_end = false;
                            }

                            chunk.push_str(&line_without_cursor);
                            consumed += line.len();
                        }
                    }
                }
            }

            this.buffer = this.buffer.split_off(consumed);
            if !chunk.is_empty() {
                return Poll::Ready(Some(Ok(chunk)));
            } else if this.stream_done {
                return Poll::Ready(None);
            }
        }
    }
}

fn prefixes(text: &str) -> impl Iterator<Item = &str> {
    (0..text.len() - 1).map(|ix| &text[..ix + 1])
}

#[derive(Default)]
pub struct Diff {
    pub deleted_row_ranges: Vec<(Anchor, RangeInclusive<u32>)>,
    pub inserted_row_ranges: Vec<Range<Anchor>>,
}

impl Diff {
    fn is_empty(&self) -> bool {
        self.deleted_row_ranges.is_empty() && self.inserted_row_ranges.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{
        Stream,
        stream::{self},
    };
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{Buffer, Point};
    use language_model::fake_provider::FakeLanguageModel;
    use language_model::{
        LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelRegistry,
        LanguageModelToolUse, StopReason, TokenUsage,
    };
    use languages::rust_lang;
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{future, sync::Arc};

    #[gpui::test(iterations = 10)]
    async fn test_transform_autoindent(cx: &mut TestAppContext, mut rng: StdRng) {
        init_test(cx);

        let text = indoc! {"
            fn main() {
                let x = 0;
                for _ in 0..10 {
                    x += 1;
                }
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(rust_lang(), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(4, 5))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                prompt_builder,
                Uuid::new_v4(),
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(&codegen, cx);

        let mut new_text = concat!(
            "       let mut x = 0;\n",
            "       while x < 10 {\n",
            "           x += 1;\n",
            "       }",
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.random_range(1..=max_len);
            let (chunk, suffix) = new_text.split_at(len);
            chunks_tx.unbounded_send(chunk.to_string()).unwrap();
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    while x < 10 {
                        x += 1;
                    }
                }
            "}
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_when_generating_past_indentation(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        init_test(cx);

        let text = indoc! {"
            fn main() {
                le
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(rust_lang(), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 6))..snapshot.anchor_after(Point::new(1, 6))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                prompt_builder,
                Uuid::new_v4(),
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(&codegen, cx);

        cx.background_executor.run_until_parked();

        let mut new_text = concat!(
            "t mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.random_range(1..=max_len);
            let (chunk, suffix) = new_text.split_at(len);
            chunks_tx.unbounded_send(chunk.to_string()).unwrap();
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    while x < 10 {
                        x += 1;
                    }
                }
            "}
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_when_generating_before_indentation(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        init_test(cx);

        let text = concat!(
            "fn main() {\n",
            "  \n",
            "}\n" //
        );
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(rust_lang(), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 2))..snapshot.anchor_after(Point::new(1, 2))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                prompt_builder,
                Uuid::new_v4(),
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(&codegen, cx);

        cx.background_executor.run_until_parked();

        let mut new_text = concat!(
            "let mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.random_range(1..=max_len);
            let (chunk, suffix) = new_text.split_at(len);
            chunks_tx.unbounded_send(chunk.to_string()).unwrap();
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    while x < 10 {
                        x += 1;
                    }
                }
            "}
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_respects_tabs_in_selection(cx: &mut TestAppContext) {
        init_test(cx);

        let text = indoc! {"
            func main() {
            \tx := 0
            \tfor i := 0; i < 10; i++ {
            \t\tx++
            \t}
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(0, 0))..snapshot.anchor_after(Point::new(4, 2))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                prompt_builder,
                Uuid::new_v4(),
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(&codegen, cx);
        let new_text = concat!(
            "func main() {\n",
            "\tx := 0\n",
            "\tfor x < 10 {\n",
            "\t\tx++\n",
            "\t}", //
        );
        chunks_tx.unbounded_send(new_text.to_string()).unwrap();
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                func main() {
                \tx := 0
                \tfor x < 10 {
                \t\tx++
                \t}
                }
            "}
        );
    }

    #[gpui::test]
    async fn test_inactive_codegen_alternative(cx: &mut TestAppContext) {
        init_test(cx);

        let text = indoc! {"
            fn main() {
                let x = 0;
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(rust_lang(), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(1, 14))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                false,
                prompt_builder,
                Uuid::new_v4(),
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(&codegen, cx);
        chunks_tx
            .unbounded_send("let mut x = 0;\nx += 1;".to_string())
            .unwrap();
        drop(chunks_tx);
        cx.run_until_parked();

        // The codegen is inactive, so the buffer doesn't get modified.
        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            text
        );

        // Activating the codegen applies the changes.
        codegen.update(cx, |codegen, cx| codegen.set_active(true, cx));
        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    x += 1;
                }
            "}
        );

        // Deactivating the codegen undoes the changes.
        codegen.update(cx, |codegen, cx| codegen.set_active(false, cx));
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            text
        );
    }

    // When not streaming tool calls, we strip backticks as part of parsing the model's
    // plain text response. This is a regression test for a bug where we stripped
    // backticks incorrectly.
    #[gpui::test]
    async fn test_allows_model_to_output_backticks(cx: &mut TestAppContext) {
        init_test(cx);
        let text = "- Improved; `cmd+click` behavior. Now requires `cmd` to be pressed before the click starts or it doesn't run. ([#44579](https://github.com/zed-industries/zed/pull/44579); thanks [Zachiah](https://github.com/Zachiah))";
        let buffer = cx.new(|cx| Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(0, 0))..snapshot.anchor_after(Point::new(0, 0))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                prompt_builder,
                Uuid::new_v4(),
                cx,
            )
        });

        let events_tx = simulate_tool_based_completion(&codegen, cx);
        let chunk_len = text.find('`').unwrap();
        events_tx
            .unbounded_send(rewrite_tool_use("tool_1", &text[..chunk_len], false))
            .unwrap();
        events_tx
            .unbounded_send(rewrite_tool_use("tool_2", &text, true))
            .unwrap();
        events_tx
            .unbounded_send(LanguageModelCompletionEvent::Stop(StopReason::EndTurn))
            .unwrap();
        drop(events_tx);
        cx.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            text
        );
    }

    #[gpui::test]
    async fn test_strip_invalid_spans_from_codeblock() {
        assert_chunks("Lorem ipsum dolor", "Lorem ipsum dolor").await;
        assert_chunks("```\nLorem ipsum dolor", "Lorem ipsum dolor").await;
        assert_chunks("```\nLorem ipsum dolor\n```", "Lorem ipsum dolor").await;
        assert_chunks(
            "```html\n```js\nLorem ipsum dolor\n```\n```",
            "```js\nLorem ipsum dolor\n```",
        )
        .await;
        assert_chunks("``\nLorem ipsum dolor\n```", "``\nLorem ipsum dolor\n```").await;
        assert_chunks("Lorem<|CURSOR|> ipsum", "Lorem ipsum").await;
        assert_chunks("Lorem ipsum", "Lorem ipsum").await;
        assert_chunks("```\n<|CURSOR|>Lorem ipsum\n```", "Lorem ipsum").await;

        async fn assert_chunks(text: &str, expected_text: &str) {
            for chunk_size in 1..=text.len() {
                let actual_text = StripInvalidSpans::new(chunks(text, chunk_size))
                    .map(|chunk| chunk.unwrap())
                    .collect::<String>()
                    .await;
                assert_eq!(
                    actual_text, expected_text,
                    "failed to strip invalid spans, chunk size: {}",
                    chunk_size
                );
            }
        }

        fn chunks(text: &str, size: usize) -> impl Stream<Item = Result<String>> {
            stream::iter(
                text.chars()
                    .collect::<Vec<_>>()
                    .chunks(size)
                    .map(|chunk| Ok(chunk.iter().collect::<String>()))
                    .collect::<Vec<_>>(),
            )
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(LanguageModelRegistry::test);
        cx.set_global(cx.update(SettingsStore::test));
    }

    fn simulate_response_stream(
        codegen: &Entity<CodegenAlternative>,
        cx: &mut TestAppContext,
    ) -> mpsc::UnboundedSender<String> {
        let (chunks_tx, chunks_rx) = mpsc::unbounded();
        let model = Arc::new(FakeLanguageModel::default());
        codegen.update(cx, |codegen, cx| {
            codegen.generation = codegen.handle_stream(
                model,
                /* strip_invalid_spans: */ false,
                future::ready(Ok(LanguageModelTextStream {
                    message_id: None,
                    stream: chunks_rx.map(Ok).boxed(),
                    last_token_usage: Arc::new(Mutex::new(TokenUsage::default())),
                })),
                cx,
            );
        });
        chunks_tx
    }

    fn simulate_tool_based_completion(
        codegen: &Entity<CodegenAlternative>,
        cx: &mut TestAppContext,
    ) -> mpsc::UnboundedSender<LanguageModelCompletionEvent> {
        let (events_tx, events_rx) = mpsc::unbounded();
        let model = Arc::new(FakeLanguageModel::default());
        codegen.update(cx, |codegen, cx| {
            let completion_stream = Task::ready(Ok(events_rx.map(Ok).boxed()
                as BoxStream<
                    'static,
                    Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                >));
            codegen.generation = codegen.handle_completion(model, completion_stream, cx);
        });
        events_tx
    }

    fn rewrite_tool_use(
        id: &str,
        replacement_text: &str,
        is_complete: bool,
    ) -> LanguageModelCompletionEvent {
        let input = RewriteSectionInput {
            replacement_text: replacement_text.into(),
        };
        LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
            id: id.into(),
            name: REWRITE_SECTION_TOOL_NAME.into(),
            raw_input: serde_json::to_string(&input).unwrap(),
            input: serde_json::to_value(&input).unwrap(),
            is_input_complete: is_complete,
            thought_signature: None,
        })
    }
}
