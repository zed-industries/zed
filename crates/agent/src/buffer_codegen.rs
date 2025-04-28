use crate::context::ContextLoadResult;
use crate::inline_prompt_editor::CodegenStatus;
use crate::{context::load_context, context_store::ContextStore};
use anyhow::Result;
use client::telemetry::Telemetry;
use collections::HashSet;
use editor::{Anchor, AnchorRangeExt, MultiBuffer, MultiBufferSnapshot, ToOffset as _, ToPoint};
use futures::{
    SinkExt, Stream, StreamExt, TryStreamExt as _, channel::mpsc, future::LocalBoxFuture, join,
};
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Subscription, Task, WeakEntity};
use language::{Buffer, IndentKind, Point, TransactionId, line_diff};
use language_model::{
    LanguageModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelTextStream, Role, report_assistant_event,
};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use project::Project;
use prompt_store::PromptBuilder;
use prompt_store::PromptStore;
use rope::Rope;
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
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};

pub struct BufferCodegen {
    alternatives: Vec<Entity<CodegenAlternative>>,
    pub active_alternative: usize,
    seen_alternatives: HashSet<usize>,
    subscriptions: Vec<Subscription>,
    buffer: Entity<MultiBuffer>,
    range: Range<Anchor>,
    initial_transaction_id: Option<TransactionId>,
    context_store: Entity<ContextStore>,
    project: WeakEntity<Project>,
    prompt_store: Option<Entity<PromptStore>>,
    telemetry: Arc<Telemetry>,
    builder: Arc<PromptBuilder>,
    pub is_insertion: bool,
}

impl BufferCodegen {
    pub fn new(
        buffer: Entity<MultiBuffer>,
        range: Range<Anchor>,
        initial_transaction_id: Option<TransactionId>,
        context_store: Entity<ContextStore>,
        project: WeakEntity<Project>,
        prompt_store: Option<Entity<PromptStore>>,
        telemetry: Arc<Telemetry>,
        builder: Arc<PromptBuilder>,
        cx: &mut Context<Self>,
    ) -> Self {
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                false,
                Some(context_store.clone()),
                project.clone(),
                prompt_store.clone(),
                Some(telemetry.clone()),
                builder.clone(),
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
            context_store,
            project,
            prompt_store,
            telemetry,
            builder,
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

    pub fn active_alternative(&self) -> &Entity<CodegenAlternative> {
        &self.alternatives[self.active_alternative]
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
                    Some(self.context_store.clone()),
                    self.project.clone(),
                    self.prompt_store.clone(),
                    Some(self.telemetry.clone()),
                    self.builder.clone(),
                    cx,
                )
            }));
        }

        for (model, alternative) in iter::once(primary_model)
            .chain(alternative_models)
            .zip(&self.alternatives)
        {
            alternative.update(cx, |alternative, cx| {
                alternative.start(user_prompt.clone(), model.clone(), cx)
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
    context_store: Option<Entity<ContextStore>>,
    project: WeakEntity<Project>,
    prompt_store: Option<Entity<PromptStore>>,
    telemetry: Option<Arc<Telemetry>>,
    _subscription: gpui::Subscription,
    builder: Arc<PromptBuilder>,
    active: bool,
    edits: Vec<(Range<Anchor>, String)>,
    line_operations: Vec<LineOperation>,
    elapsed_time: Option<f64>,
    completion: Option<String>,
    pub message_id: Option<String>,
}

impl EventEmitter<CodegenEvent> for CodegenAlternative {}

impl CodegenAlternative {
    pub fn new(
        buffer: Entity<MultiBuffer>,
        range: Range<Anchor>,
        active: bool,
        context_store: Option<Entity<ContextStore>>,
        project: WeakEntity<Project>,
        prompt_store: Option<Entity<PromptStore>>,
        telemetry: Option<Arc<Telemetry>>,
        builder: Arc<PromptBuilder>,
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
                buffer.set_language_registry(language_registry)
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
            context_store,
            project,
            prompt_store,
            telemetry,
            _subscription: cx.subscribe(&buffer, Self::handle_buffer_event),
            builder,
            active,
            edits: Vec::new(),
            line_operations: Vec::new(),
            range,
            elapsed_time: None,
            completion: None,
        }
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
        if let multi_buffer::Event::TransactionUndone { transaction_id } = event {
            if self.transformation_transaction_id == Some(*transaction_id) {
                self.transformation_transaction_id = None;
                self.generation = Task::ready(());
                cx.emit(CodegenEvent::Undone);
            }
        }
    }

    pub fn last_equal_ranges(&self) -> &[Range<Anchor>] {
        &self.last_equal_ranges
    }

    pub fn start(
        &mut self,
        user_prompt: String,
        model: Arc<dyn LanguageModel>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Some(transformation_transaction_id) = self.transformation_transaction_id.take() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.undo_transaction(transformation_transaction_id, cx);
            });
        }

        self.edit_position = Some(self.range.start.bias_right(&self.snapshot));

        let api_key = model.api_key(cx);
        let telemetry_id = model.telemetry_id();
        let provider_id = model.provider_id();
        let stream: LocalBoxFuture<Result<LanguageModelTextStream>> =
            if user_prompt.trim().to_lowercase() == "delete" {
                async { Ok(LanguageModelTextStream::default()) }.boxed_local()
            } else {
                let request = self.build_request(user_prompt, cx)?;
                cx.spawn(async move |_, cx| model.stream_completion_text(request.await, &cx).await)
                    .boxed_local()
            };
        self.handle_stream(telemetry_id, provider_id.to_string(), api_key, stream, cx);
        Ok(())
    }

    fn build_request(
        &self,
        user_prompt: String,
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
                return Err(anyhow::anyhow!("invalid transformation range"));
            }
        } else {
            return Err(anyhow::anyhow!("invalid transformation range"));
        };

        let prompt = self
            .builder
            .generate_inline_transformation_prompt(user_prompt, language_name, buffer, range)
            .map_err(|e| anyhow::anyhow!("Failed to generate content prompt: {}", e))?;

        let context_task = self.context_store.as_ref().map(|context_store| {
            if let Some(project) = self.project.upgrade() {
                let context = context_store
                    .read(cx)
                    .context()
                    .cloned()
                    .collect::<Vec<_>>();
                load_context(context, &project, &self.prompt_store, cx)
            } else {
                Task::ready(ContextLoadResult::default())
            }
        });

        Ok(cx.spawn(async move |_cx| {
            let mut request_message = LanguageModelRequestMessage {
                role: Role::User,
                content: Vec::new(),
                cache: false,
            };

            if let Some(context_task) = context_task {
                context_task
                    .await
                    .loaded_context
                    .add_to_request_message(&mut request_message);
            }

            request_message.content.push(prompt.into());

            LanguageModelRequest {
                thread_id: None,
                prompt_id: None,
                tools: Vec::new(),
                stop: Vec::new(),
                temperature: None,
                messages: vec![request_message],
            }
        }))
    }

    pub fn handle_stream(
        &mut self,
        model_telemetry_id: String,
        model_provider_id: String,
        model_api_key: Option<String>,
        stream: impl 'static + Future<Output = Result<LanguageModelTextStream>>,
        cx: &mut Context<Self>,
    ) {
        let start_time = Instant::now();
        let snapshot = self.snapshot.clone();
        let selected_text = snapshot
            .text_for_range(self.range.start..self.range.end)
            .collect::<Rope>();

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

        let http_client = cx.http_client();
        let telemetry = self.telemetry.clone();
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

        self.generation = cx.spawn(async move |codegen, cx| {
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
                let executor = cx.background_executor().clone();
                let message_id = message_id.clone();
                let line_based_stream_diff: Task<anyhow::Result<()>> =
                    cx.background_spawn(async move {
                        let mut response_latency = None;
                        let request_start = Instant::now();
                        let diff = async {
                            let chunks = StripInvalidSpans::new(
                                stream?.stream.map_err(|error| error.into()),
                            );
                            futures::pin_mut!(chunks);
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
                                    if line_indent.is_none() {
                                        if let Some(non_whitespace_ch_ix) =
                                            new_text.find(|ch: char| !ch.is_whitespace())
                                        {
                                            line_indent = Some(non_whitespace_ch_ix);
                                            base_indent = base_indent.or(line_indent);

                                            let line_indent = line_indent.unwrap();
                                            let base_indent = base_indent.unwrap();
                                            let indent_delta =
                                                line_indent as i32 - base_indent as i32;
                                            let mut corrected_indent_len = cmp::max(
                                                0,
                                                suggested_line_indent.len as i32 + indent_delta,
                                            )
                                                as usize;
                                            if first_line {
                                                corrected_indent_len = corrected_indent_len
                                                    .saturating_sub(
                                                        selection_start.column as usize,
                                                    );
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
                        report_assistant_event(
                            AssistantEventData {
                                conversation_id: None,
                                message_id,
                                kind: AssistantKind::Inline,
                                phase: AssistantPhase::Response,
                                model: model_telemetry_id,
                                model_provider: model_provider_id,
                                response_latency,
                                error_message,
                                language_name: language_name.map(|name| name.to_proto()),
                            },
                            telemetry,
                            http_client,
                            model_api_key,
                            &executor,
                        );

                        result?;
                        Ok(())
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
        });
        cx.notify();
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
            // Avoid grouping assistant edits with user edits.
            buffer.finalize_last_transaction(cx);
            buffer.start_transaction(cx);
            buffer.edit(edits, None, cx);
            buffer.end_transaction(cx)
        });

        if let Some(transaction) = transaction {
            if let Some(first_transaction) = self.transformation_transaction_id {
                // Group all assistant edits into the first transaction.
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

                            chunk.push_str(&line);
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
    use fs::FakeFs;
    use futures::{
        Stream,
        stream::{self},
    };
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{
        Buffer, Language, LanguageConfig, LanguageMatcher, Point, language_settings,
        tree_sitter_rust,
    };
    use language_model::{LanguageModelRegistry, TokenUsage};
    use rand::prelude::*;
    use serde::Serialize;
    use settings::SettingsStore;
    use std::{future, sync::Arc};

    #[derive(Serialize)]
    pub struct DummyCompletionRequest {
        pub name: String,
    }

    #[gpui::test(iterations = 10)]
    async fn test_transform_autoindent(cx: &mut TestAppContext, mut rng: StdRng) {
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_model::LanguageModelRegistry::test);
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                let x = 0;
                for _ in 0..10 {
                    x += 1;
                }
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(4, 5))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, vec![], cx).await;
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                project.downgrade(),
                None,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);

        let mut new_text = concat!(
            "       let mut x = 0;\n",
            "       while x < 10 {\n",
            "           x += 1;\n",
            "       }",
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.gen_range(1..=max_len);
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
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                le
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 6))..snapshot.anchor_after(Point::new(1, 6))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, vec![], cx).await;
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                project.downgrade(),
                None,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);

        cx.background_executor.run_until_parked();

        let mut new_text = concat!(
            "t mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.gen_range(1..=max_len);
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
        cx.update(LanguageModelRegistry::test);
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = concat!(
            "fn main() {\n",
            "  \n",
            "}\n" //
        );
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 2))..snapshot.anchor_after(Point::new(1, 2))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, vec![], cx).await;
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                project.downgrade(),
                None,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);

        cx.background_executor.run_until_parked();

        let mut new_text = concat!(
            "let mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.gen_range(1..=max_len);
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
        cx.update(LanguageModelRegistry::test);
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

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
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, vec![], cx).await;
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                project.downgrade(),
                None,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);
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
        cx.update(LanguageModelRegistry::test);
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                let x = 0;
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(1, 14))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, vec![], cx).await;
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                false,
                None,
                project.downgrade(),
                None,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);
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

    fn simulate_response_stream(
        codegen: Entity<CodegenAlternative>,
        cx: &mut TestAppContext,
    ) -> mpsc::UnboundedSender<String> {
        let (chunks_tx, chunks_rx) = mpsc::unbounded();
        codegen.update(cx, |codegen, cx| {
            codegen.handle_stream(
                String::new(),
                String::new(),
                None,
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

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_indents_query(
            r#"
            (call_expression) @indent
            (field_expression) @indent
            (_ "(" ")" @end) @indent
            (_ "{" "}" @end) @indent
            "#,
        )
        .unwrap()
    }
}
