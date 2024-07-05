use crate::{
    assistant_settings::OpenAiModel, slash_command::SlashCommandLine, CompletionProvider,
    LanguageModelRequest, LanguageModelRequestMessage, MessageId, MessageStatus, Role,
};
use anyhow::{anyhow, Result};
use assistant_slash_command::{
    SlashCommandOutput, SlashCommandOutputSection, SlashCommandRegistry,
};
use client::{telemetry::Telemetry, Client};
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{future::Shared, FutureExt, StreamExt};
use fuzzy::StringMatchCandidate;
use gpui::{
    AppContext, AsyncAppContext, Context as _, EventEmitter, Model, ModelContext, Subscription,
    Task, WeakModel,
};
use language::{AnchorRangeExt, Buffer, LanguageRegistry, OffsetRangeExt, Point, ToOffset};
use paths::contexts_dir;
use project::Project;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, Reverse},
    ffi::OsStr,
    iter, mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::AssistantKind;
use ui::SharedString;
use util::{post_inc, ResultExt, TryFutureExt};
use uuid::Uuid;

#[derive(Clone)]
pub enum ContextOperation {
    InsertMessage {
        id: MessageId,
        left_sibling: MessageId,
        start: language::Anchor,
        metadata: MessageMetadata,
    },
    UpdateMessage {
        message_id: MessageId,
        metadata: MessageMetadata,
    },
    UpdateSummary(ContextSummary),
    SlashCommandFinished {
        id: SlashCommandId,
        output_range: Range<language::Anchor>,
        sections: Vec<SlashCommandOutputSection<language::Anchor>>,
    },
    BufferOperation(language::Operation),
}

impl ContextOperation {
    fn timestamp(&self) -> clock::Lamport {
        match self {
            Self::InsertMessage { id, .. } => id.0,
            Self::UpdateMessage { metadata, .. } => metadata.timestamp,
            Self::UpdateSummary(summary) => summary.timestamp,
            Self::SlashCommandFinished { id, .. } => id.0,
            Self::BufferOperation(_) => {
                panic!("reading the timestamp of a buffer operation is not supported")
            }
        }
    }
}

#[derive(Clone)]
pub enum ContextEvent {
    MessagesEdited,
    SummaryChanged,
    EditSuggestionsChanged,
    StreamedCompletion,
    PendingSlashCommandsUpdated {
        removed: Vec<Range<language::Anchor>>,
        updated: Vec<PendingSlashCommand>,
    },
    SlashCommandFinished {
        output_range: Range<language::Anchor>,
        sections: Vec<SlashCommandOutputSection<language::Anchor>>,
        run_commands_in_output: bool,
    },
    Operation(ContextOperation),
}

#[derive(Clone, Default)]
pub struct ContextSummary {
    pub text: String,
    done: bool,
    timestamp: clock::Lamport,
}

#[derive(Clone, Debug)]
pub struct MessageAnchor {
    pub id: MessageId,
    pub start: language::Anchor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageMetadata {
    role: Role,
    status: MessageStatus,
    timestamp: clock::Lamport,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub offset_range: Range<usize>,
    pub index_range: Range<usize>,
    pub id: MessageId,
    pub anchor: language::Anchor,
    pub role: Role,
    pub status: MessageStatus,
}

impl Message {
    fn to_request_message(&self, buffer: &Buffer) -> LanguageModelRequestMessage {
        LanguageModelRequestMessage {
            role: self.role,
            content: buffer.text_for_range(self.offset_range.clone()).collect(),
        }
    }
}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct SlashCommandId(clock::Lamport);

pub struct Context {
    id: Option<String>,
    timestamp: clock::Lamport,
    pending_ops: Vec<ContextOperation>,
    buffer: Model<Buffer>,
    edit_suggestions: Vec<EditSuggestion>,
    pending_slash_commands: Vec<PendingSlashCommand>,
    edits_since_last_slash_command_parse: language::Subscription,
    finished_slash_commands: HashSet<SlashCommandId>,
    slash_command_output_sections: Vec<SlashCommandOutputSection<language::Anchor>>,
    message_anchors: Vec<MessageAnchor>,
    messages_metadata: HashMap<MessageId, MessageMetadata>,
    summary: Option<ContextSummary>,
    pending_summary: Task<Option<()>>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    token_count: Option<usize>,
    pending_token_count: Task<Option<()>>,
    pending_edit_suggestion_parse: Option<Task<()>>,
    pending_save: Task<Result<()>>,
    path: Option<PathBuf>,
    _subscriptions: Vec<Subscription>,
    telemetry: Option<Arc<Telemetry>>,
    language_registry: Arc<LanguageRegistry>,
}

impl EventEmitter<ContextEvent> for Context {}

impl Context {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let buffer = cx.new_model(|cx| {
            let mut buffer = Buffer::local("", cx);
            buffer.set_language_registry(language_registry.clone());
            buffer
        });
        let edits_since_last_slash_command_parse =
            buffer.update(cx, |buffer, _| buffer.subscribe());
        let mut this = Self {
            id: Some(Uuid::new_v4().to_string()),
            timestamp: clock::Lamport::new(0),
            pending_ops: Vec::new(),
            message_anchors: Default::default(),
            messages_metadata: Default::default(),
            edit_suggestions: Vec::new(),
            pending_slash_commands: Vec::new(),
            finished_slash_commands: HashSet::default(),
            slash_command_output_sections: Vec::new(),
            edits_since_last_slash_command_parse,
            summary: None,
            pending_summary: Task::ready(None),
            completion_count: Default::default(),
            pending_completions: Default::default(),
            token_count: None,
            pending_token_count: Task::ready(None),
            pending_edit_suggestion_parse: None,
            _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
            pending_save: Task::ready(Ok(())),
            path: None,
            buffer,
            telemetry,
            language_registry,
        };

        let message = MessageAnchor {
            id: MessageId(this.timestamp.tick()),
            start: language::Anchor::MIN,
        };
        this.messages_metadata.insert(
            message.id,
            MessageMetadata {
                role: Role::User,
                status: MessageStatus::Done,
                timestamp: message.id.0,
            },
        );
        this.message_anchors.push(message);

        this.set_language(cx);
        this.count_remaining_tokens(cx);
        this
    }

    fn serialize(&self, cx: &AppContext) -> SavedContext {
        let buffer = self.buffer.read(cx);
        SavedContext {
            id: self.id.clone(),
            zed: "context".into(),
            version: SavedContext::VERSION.into(),
            text: buffer.text(),
            message_metadata: self.messages_metadata.clone(),
            messages: self
                .messages(cx)
                .map(|message| SavedMessage {
                    id: message.id,
                    start: message.offset_range.start,
                })
                .collect(),
            summary: self
                .summary
                .as_ref()
                .map(|summary| summary.text.clone())
                .unwrap_or_default(),
            slash_command_output_sections: self
                .slash_command_output_sections
                .iter()
                .filter_map(|section| {
                    let range = section.range.to_offset(buffer);
                    if section.range.start.is_valid(buffer) && !range.is_empty() {
                        Some(assistant_slash_command::SlashCommandOutputSection {
                            range,
                            icon: section.icon,
                            label: section.label.clone(),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn deserialize(
        saved_context: SavedContext,
        path: PathBuf,
        language_registry: Arc<LanguageRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut AsyncAppContext,
    ) -> Result<Model<Self>> {
        let id = match saved_context.id {
            Some(id) => Some(id),
            None => Some(Uuid::new_v4().to_string()),
        };

        let markdown = language_registry.language_for_name("Markdown");
        let mut message_anchors = Vec::new();
        let mut timestamp = clock::Lamport::new(ReplicaId::default());
        let buffer = cx.new_model(|cx| {
            let mut buffer = Buffer::local(saved_context.text, cx);
            for message in saved_context.messages {
                message_anchors.push(MessageAnchor {
                    id: message.id,
                    start: buffer.anchor_before(message.start),
                });
                timestamp.observe(message.id.0);
            }
            buffer.set_language_registry(language_registry.clone());
            cx.spawn(|buffer, mut cx| async move {
                let markdown = markdown.await?;
                buffer.update(&mut cx, |buffer: &mut Buffer, cx| {
                    buffer.set_language(Some(markdown), cx)
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            buffer
        })?;

        cx.new_model(move |cx| {
            let edits_since_last_slash_command_parse =
                buffer.update(cx, |buffer, _| buffer.subscribe());
            let mut this = Self {
                id,
                timestamp,
                pending_ops: Vec::new(),
                message_anchors,
                messages_metadata: saved_context.message_metadata,
                edit_suggestions: Vec::new(),
                pending_slash_commands: Vec::new(),
                finished_slash_commands: HashSet::default(),
                slash_command_output_sections: saved_context
                    .slash_command_output_sections
                    .into_iter()
                    .map(|section| {
                        let buffer = buffer.read(cx);
                        SlashCommandOutputSection {
                            range: buffer.anchor_after(section.range.start)
                                ..buffer.anchor_before(section.range.end),
                            icon: section.icon,
                            label: section.label,
                        }
                    })
                    .collect(),
                edits_since_last_slash_command_parse,
                summary: Some(ContextSummary {
                    text: saved_context.summary,
                    done: true,
                    timestamp,
                }),
                pending_summary: Task::ready(None),
                completion_count: Default::default(),
                pending_completions: Default::default(),
                token_count: None,
                pending_edit_suggestion_parse: None,
                pending_token_count: Task::ready(None),
                _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
                pending_save: Task::ready(Ok(())),
                path: Some(path),
                buffer,
                telemetry,
                language_registry,
            };
            this.set_language(cx);
            this.reparse_edit_suggestions(cx);
            this.count_remaining_tokens(cx);
            this
        })
    }

    fn apply_ops(&mut self, ops: Vec<ContextOperation>, cx: &mut ModelContext<Self>) -> Result<()> {
        let mut buffer_ops = Vec::new();
        for op in ops {
            match op {
                ContextOperation::BufferOperation(buffer_op) => buffer_ops.push(buffer_op),
                op @ _ => self.pending_ops.push(op),
            }
        }
        self.buffer
            .update(cx, |buffer, cx| buffer.apply_ops(buffer_ops, cx))?;
        self.flush_ops(cx);

        Ok(())
    }

    fn flush_ops(&mut self, cx: &mut ModelContext<Context>) {
        // Ensure that context operations always refer to buffer anchors that
        // are present in the local replica.
        if !self.buffer.read(cx).has_deferred_ops() {
            return;
        }

        let mut messages_changed = false;
        let mut summary_changed = false;

        self.pending_ops.sort_unstable_by_key(|op| op.timestamp());
        for op in mem::take(&mut self.pending_ops) {
            if !self.can_apply_op(&op) {
                self.pending_ops.push(op);
                continue;
            }

            let timestamp = op.timestamp();
            match op {
                ContextOperation::InsertMessage {
                    id,
                    left_sibling,
                    start,
                    metadata,
                } => {
                    if self.messages_metadata.contains_key(&id) {
                        // We already applied this operation.
                    } else {
                        let left_sibling_ix = self
                            .message_anchors
                            .iter()
                            .position(|anchor| anchor.id == left_sibling)
                            .unwrap();

                        let mut insertion_ix = left_sibling_ix + 1;
                        for message in &self.message_anchors[insertion_ix..] {
                            if id > message.id {
                                break;
                            } else {
                                insertion_ix += 1;
                            }
                        }

                        self.message_anchors
                            .insert(insertion_ix, MessageAnchor { id, start });
                        self.messages_metadata.insert(id, metadata);
                        messages_changed = true;
                    }
                }
                ContextOperation::UpdateMessage {
                    message_id,
                    metadata: new_metadata,
                } => {
                    let metadata = self.messages_metadata.get_mut(&message_id).unwrap();
                    if new_metadata.timestamp > metadata.timestamp {
                        *metadata = new_metadata;
                        messages_changed = true;
                    }
                }
                ContextOperation::UpdateSummary(new_summary) => {
                    if self
                        .summary
                        .as_ref()
                        .map_or(true, |summary| new_summary.timestamp > summary.timestamp)
                    {
                        self.summary = Some(new_summary);
                        summary_changed = true;
                    }
                }
                ContextOperation::SlashCommandFinished {
                    id,
                    output_range,
                    sections,
                } => {
                    if self.finished_slash_commands.insert(id) {
                        let buffer = self.buffer.read(cx);
                        self.slash_command_output_sections
                            .extend(sections.iter().cloned());
                        self.slash_command_output_sections
                            .sort_by(|a, b| a.range.cmp(&b.range, buffer));
                        cx.emit(ContextEvent::SlashCommandFinished {
                            output_range,
                            sections,
                            run_commands_in_output: false,
                        });
                    }
                }
                ContextOperation::BufferOperation(_) => unreachable!(),
            }

            self.timestamp.observe(timestamp);
        }

        if messages_changed {
            cx.emit(ContextEvent::MessagesEdited);
            cx.notify();
        }

        if summary_changed {
            cx.emit(ContextEvent::SummaryChanged);
            cx.notify();
        }
    }

    fn can_apply_op(&self, op: &ContextOperation) -> bool {
        match op {
            ContextOperation::InsertMessage { left_sibling, .. } => {
                self.messages_metadata.contains_key(left_sibling)
            }
            ContextOperation::UpdateMessage { message_id, .. } => {
                self.messages_metadata.contains_key(message_id)
            }
            ContextOperation::UpdateSummary { .. } => true,
            ContextOperation::SlashCommandFinished { .. } => true,
            ContextOperation::BufferOperation(_) => {
                panic!("buffer operations should always be applied")
            }
        }
    }

    pub fn buffer(&self) -> &Model<Buffer> {
        &self.buffer
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn summary(&self) -> Option<&ContextSummary> {
        self.summary.as_ref()
    }

    pub fn edit_suggestions(&self) -> &[EditSuggestion] {
        &self.edit_suggestions
    }

    pub fn pending_slash_commands(&self) -> &[PendingSlashCommand] {
        &self.pending_slash_commands
    }

    pub fn slash_command_output_sections(&self) -> &[SlashCommandOutputSection<language::Anchor>] {
        &self.slash_command_output_sections
    }

    fn set_language(&mut self, cx: &mut ModelContext<Self>) {
        let markdown = self.language_registry.language_for_name("Markdown");
        cx.spawn(|this, mut cx| async move {
            let markdown = markdown.await?;
            this.update(&mut cx, |this, cx| {
                this.buffer
                    .update(cx, |buffer, cx| buffer.set_language(Some(markdown), cx));
            })
        })
        .detach_and_log_err(cx);
    }

    fn handle_buffer_event(
        &mut self,
        _: Model<Buffer>,
        event: &language::Event,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            language::Event::Operation(operation) => cx.emit(ContextEvent::Operation(
                ContextOperation::BufferOperation(operation.clone()),
            )),
            language::Event::Edited => {
                self.count_remaining_tokens(cx);
                self.reparse_edit_suggestions(cx);
                self.reparse_slash_commands(cx);
                cx.emit(ContextEvent::MessagesEdited);
            }
            _ => {}
        }
    }

    pub(crate) fn token_count(&self) -> Option<usize> {
        self.token_count
    }

    pub(crate) fn count_remaining_tokens(&mut self, cx: &mut ModelContext<Self>) {
        let request = self.to_completion_request(cx);
        self.pending_token_count = cx.spawn(|this, mut cx| {
            async move {
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;

                let token_count = cx
                    .update(|cx| CompletionProvider::global(cx).count_tokens(request, cx))?
                    .await?;

                this.update(&mut cx, |this, cx| {
                    this.token_count = Some(token_count);
                    cx.notify()
                })?;
                anyhow::Ok(())
            }
            .log_err()
        });
    }

    pub fn reparse_slash_commands(&mut self, cx: &mut ModelContext<Self>) {
        let buffer = self.buffer.read(cx);
        let mut row_ranges = self
            .edits_since_last_slash_command_parse
            .consume()
            .into_iter()
            .map(|edit| {
                let start_row = buffer.offset_to_point(edit.new.start).row;
                let end_row = buffer.offset_to_point(edit.new.end).row + 1;
                start_row..end_row
            })
            .peekable();

        let mut removed = Vec::new();
        let mut updated = Vec::new();
        while let Some(mut row_range) = row_ranges.next() {
            while let Some(next_row_range) = row_ranges.peek() {
                if row_range.end >= next_row_range.start {
                    row_range.end = next_row_range.end;
                    row_ranges.next();
                } else {
                    break;
                }
            }

            let start = buffer.anchor_before(Point::new(row_range.start, 0));
            let end = buffer.anchor_after(Point::new(
                row_range.end - 1,
                buffer.line_len(row_range.end - 1),
            ));

            let old_range = self.pending_command_indices_for_range(start..end, cx);

            let mut new_commands = Vec::new();
            let mut lines = buffer.text_for_range(start..end).lines();
            let mut offset = lines.offset();
            while let Some(line) = lines.next() {
                if let Some(command_line) = SlashCommandLine::parse(line) {
                    let name = &line[command_line.name.clone()];
                    let argument = command_line.argument.as_ref().and_then(|argument| {
                        (!argument.is_empty()).then_some(&line[argument.clone()])
                    });
                    if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
                        if !command.requires_argument() || argument.is_some() {
                            let start_ix = offset + command_line.name.start - 1;
                            let end_ix = offset
                                + command_line
                                    .argument
                                    .map_or(command_line.name.end, |argument| argument.end);
                            let source_range =
                                buffer.anchor_after(start_ix)..buffer.anchor_after(end_ix);
                            let pending_command = PendingSlashCommand {
                                name: name.to_string(),
                                argument: argument.map(ToString::to_string),
                                source_range,
                                status: PendingSlashCommandStatus::Idle,
                            };
                            updated.push(pending_command.clone());
                            new_commands.push(pending_command);
                        }
                    }
                }

                offset = lines.offset();
            }

            let removed_commands = self.pending_slash_commands.splice(old_range, new_commands);
            removed.extend(removed_commands.map(|command| command.source_range));
        }

        if !updated.is_empty() || !removed.is_empty() {
            cx.emit(ContextEvent::PendingSlashCommandsUpdated { removed, updated });
        }
    }

    fn reparse_edit_suggestions(&mut self, cx: &mut ModelContext<Self>) {
        self.pending_edit_suggestion_parse = Some(cx.spawn(|this, mut cx| async move {
            cx.background_executor()
                .timer(Duration::from_millis(200))
                .await;

            this.update(&mut cx, |this, cx| {
                this.reparse_edit_suggestions_in_range(0..this.buffer.read(cx).len(), cx);
            })
            .ok();
        }));
    }

    fn reparse_edit_suggestions_in_range(
        &mut self,
        range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) {
        self.buffer.update(cx, |buffer, _| {
            let range_start = buffer.anchor_before(range.start);
            let range_end = buffer.anchor_after(range.end);
            let start_ix = self
                .edit_suggestions
                .binary_search_by(|probe| {
                    probe
                        .source_range
                        .end
                        .cmp(&range_start, buffer)
                        .then(Ordering::Greater)
                })
                .unwrap_err();
            let end_ix = self
                .edit_suggestions
                .binary_search_by(|probe| {
                    probe
                        .source_range
                        .start
                        .cmp(&range_end, buffer)
                        .then(Ordering::Less)
                })
                .unwrap_err();

            let mut new_edit_suggestions = Vec::new();
            let mut message_lines = buffer.as_rope().chunks_in_range(range).lines();
            while let Some(suggestion) = parse_next_edit_suggestion(&mut message_lines) {
                let start_anchor = buffer.anchor_after(suggestion.outer_range.start);
                let end_anchor = buffer.anchor_before(suggestion.outer_range.end);
                new_edit_suggestions.push(EditSuggestion {
                    source_range: start_anchor..end_anchor,
                    full_path: suggestion.path,
                });
            }
            self.edit_suggestions
                .splice(start_ix..end_ix, new_edit_suggestions);
        });
        cx.emit(ContextEvent::EditSuggestionsChanged);
        cx.notify();
    }

    pub fn pending_command_for_position(
        &mut self,
        position: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Option<&mut PendingSlashCommand> {
        let buffer = self.buffer.read(cx);
        match self
            .pending_slash_commands
            .binary_search_by(|probe| probe.source_range.end.cmp(&position, buffer))
        {
            Ok(ix) => Some(&mut self.pending_slash_commands[ix]),
            Err(ix) => {
                let cmd = self.pending_slash_commands.get_mut(ix)?;
                if position.cmp(&cmd.source_range.start, buffer).is_ge()
                    && position.cmp(&cmd.source_range.end, buffer).is_le()
                {
                    Some(cmd)
                } else {
                    None
                }
            }
        }
    }

    pub fn pending_commands_for_range(
        &self,
        range: Range<language::Anchor>,
        cx: &AppContext,
    ) -> &[PendingSlashCommand] {
        let range = self.pending_command_indices_for_range(range, cx);
        &self.pending_slash_commands[range]
    }

    fn pending_command_indices_for_range(
        &self,
        range: Range<language::Anchor>,
        cx: &AppContext,
    ) -> Range<usize> {
        let buffer = self.buffer.read(cx);
        let start_ix = match self
            .pending_slash_commands
            .binary_search_by(|probe| probe.source_range.end.cmp(&range.start, &buffer))
        {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match self
            .pending_slash_commands
            .binary_search_by(|probe| probe.source_range.start.cmp(&range.end, &buffer))
        {
            Ok(ix) => ix + 1,
            Err(ix) => ix,
        };
        start_ix..end_ix
    }

    pub fn insert_command_output(
        &mut self,
        command_range: Range<language::Anchor>,
        output: Task<Result<SlashCommandOutput>>,
        insert_trailing_newline: bool,
        cx: &mut ModelContext<Self>,
    ) {
        self.reparse_slash_commands(cx);

        let insert_output_task = cx.spawn(|this, mut cx| {
            let command_range = command_range.clone();
            async move {
                let output = output.await;
                this.update(&mut cx, |this, cx| match output {
                    Ok(mut output) => {
                        if insert_trailing_newline {
                            output.text.push('\n');
                        }

                        let events = this.buffer.update(cx, |buffer, cx| {
                            let start = command_range.start.to_offset(buffer);
                            let old_end = command_range.end.to_offset(buffer);
                            let new_end = start + output.text.len();
                            buffer.edit([(start..old_end, output.text)], None, cx);

                            let mut sections = output
                                .sections
                                .into_iter()
                                .map(|section| SlashCommandOutputSection {
                                    range: buffer.anchor_after(start + section.range.start)
                                        ..buffer.anchor_before(start + section.range.end),
                                    icon: section.icon,
                                    label: section.label,
                                })
                                .collect::<Vec<_>>();
                            sections.sort_by(|a, b| a.range.cmp(&b.range, buffer));

                            this.slash_command_output_sections
                                .extend(sections.iter().cloned());
                            this.slash_command_output_sections
                                .sort_by(|a, b| a.range.cmp(&b.range, buffer));

                            let command_id = SlashCommandId(this.timestamp.tick());
                            this.finished_slash_commands.insert(command_id);

                            let output_range =
                                buffer.anchor_after(start)..buffer.anchor_before(new_end);
                            [
                                ContextEvent::Operation(ContextOperation::SlashCommandFinished {
                                    id: command_id,
                                    output_range: output_range.clone(),
                                    sections: sections.clone(),
                                }),
                                ContextEvent::SlashCommandFinished {
                                    output_range,
                                    sections,
                                    run_commands_in_output: output.run_commands_in_text,
                                },
                            ]
                        });

                        for event in events {
                            cx.emit(event);
                        }
                    }
                    Err(error) => {
                        if let Some(pending_command) =
                            this.pending_command_for_position(command_range.start, cx)
                        {
                            pending_command.status =
                                PendingSlashCommandStatus::Error(error.to_string());
                            cx.emit(ContextEvent::PendingSlashCommandsUpdated {
                                removed: vec![pending_command.source_range.clone()],
                                updated: vec![pending_command.clone()],
                            });
                        }
                    }
                })
                .ok();
            }
        });

        if let Some(pending_command) = self.pending_command_for_position(command_range.start, cx) {
            pending_command.status = PendingSlashCommandStatus::Running {
                _task: insert_output_task.shared(),
            };
            cx.emit(ContextEvent::PendingSlashCommandsUpdated {
                removed: vec![pending_command.source_range.clone()],
                updated: vec![pending_command.clone()],
            });
        }
    }

    pub fn completion_provider_changed(&mut self, cx: &mut ModelContext<Self>) {
        self.count_remaining_tokens(cx);
    }

    pub fn assist(
        &mut self,
        selected_messages: HashSet<MessageId>,
        cx: &mut ModelContext<Self>,
    ) -> Vec<MessageAnchor> {
        let mut user_messages = Vec::new();

        let last_message_id = if let Some(last_message_id) =
            self.message_anchors.iter().rev().find_map(|message| {
                message
                    .start
                    .is_valid(self.buffer.read(cx))
                    .then_some(message.id)
            }) {
            last_message_id
        } else {
            return Default::default();
        };

        let mut should_assist = false;
        for selected_message_id in selected_messages {
            let selected_message_role =
                if let Some(metadata) = self.messages_metadata.get(&selected_message_id) {
                    metadata.role
                } else {
                    continue;
                };

            if selected_message_role == Role::Assistant {
                if let Some(user_message) = self.insert_message_after(
                    selected_message_id,
                    Role::User,
                    MessageStatus::Done,
                    cx,
                ) {
                    user_messages.push(user_message);
                }
            } else {
                should_assist = true;
            }
        }

        if should_assist {
            if !CompletionProvider::global(cx).is_authenticated() {
                log::info!("completion provider has no credentials");
                return Default::default();
            }

            let request = self.to_completion_request(cx);
            let stream = CompletionProvider::global(cx).complete(request);
            let assistant_message = self
                .insert_message_after(last_message_id, Role::Assistant, MessageStatus::Pending, cx)
                .unwrap();

            // Queue up the user's next reply.
            let user_message = self
                .insert_message_after(assistant_message.id, Role::User, MessageStatus::Done, cx)
                .unwrap();
            user_messages.push(user_message);

            let task = cx.spawn({
                |this, mut cx| async move {
                    let assistant_message_id = assistant_message.id;
                    let mut response_latency = None;
                    let stream_completion = async {
                        let request_start = Instant::now();
                        let mut messages = stream.await?;

                        while let Some(message) = messages.next().await {
                            if response_latency.is_none() {
                                response_latency = Some(request_start.elapsed());
                            }
                            let text = message?;

                            this.update(&mut cx, |this, cx| {
                                let message_ix = this
                                    .message_anchors
                                    .iter()
                                    .position(|message| message.id == assistant_message_id)?;
                                let message_range = this.buffer.update(cx, |buffer, cx| {
                                    let message_start_offset =
                                        this.message_anchors[message_ix].start.to_offset(buffer);
                                    let message_old_end_offset = this.message_anchors
                                        [message_ix + 1..]
                                        .iter()
                                        .find(|message| message.start.is_valid(buffer))
                                        .map_or(buffer.len(), |message| {
                                            message.start.to_offset(buffer).saturating_sub(1)
                                        });
                                    let message_new_end_offset =
                                        message_old_end_offset + text.len();
                                    buffer.edit(
                                        [(message_old_end_offset..message_old_end_offset, text)],
                                        None,
                                        cx,
                                    );
                                    message_start_offset..message_new_end_offset
                                });
                                this.reparse_edit_suggestions_in_range(message_range, cx);
                                cx.emit(ContextEvent::StreamedCompletion);

                                Some(())
                            })?;
                            smol::future::yield_now().await;
                        }

                        this.update(&mut cx, |this, cx| {
                            this.pending_completions
                                .retain(|completion| completion.id != this.completion_count);
                            this.summarize(cx);
                        })?;

                        anyhow::Ok(())
                    };

                    let result = stream_completion.await;

                    this.update(&mut cx, |this, cx| {
                        if let Some(metadata) =
                            this.messages_metadata.get_mut(&assistant_message.id)
                        {
                            let error_message = result
                                .err()
                                .map(|error| error.to_string().trim().to_string());
                            if let Some(error_message) = error_message.as_ref() {
                                metadata.status =
                                    MessageStatus::Error(SharedString::from(error_message.clone()));
                            } else {
                                metadata.status = MessageStatus::Done;
                            }
                            metadata.timestamp = this.timestamp.tick();

                            if let Some(telemetry) = this.telemetry.as_ref() {
                                let model = CompletionProvider::global(cx).model();
                                telemetry.report_assistant_event(
                                    this.id.clone(),
                                    AssistantKind::Panel,
                                    model.telemetry_id(),
                                    response_latency,
                                    error_message,
                                );
                            }

                            cx.emit(ContextEvent::Operation(ContextOperation::UpdateMessage {
                                message_id: assistant_message.id,
                                metadata: metadata.clone(),
                            }));
                            cx.emit(ContextEvent::MessagesEdited);
                        }
                    })
                    .ok();
                }
            });

            self.pending_completions.push(PendingCompletion {
                id: post_inc(&mut self.completion_count),
                _task: task,
            });
        }

        user_messages
    }

    pub fn to_completion_request(&self, cx: &AppContext) -> LanguageModelRequest {
        let messages = self
            .messages(cx)
            .filter(|message| matches!(message.status, MessageStatus::Done))
            .map(|message| message.to_request_message(self.buffer.read(cx)));

        LanguageModelRequest {
            model: CompletionProvider::global(cx).model(),
            messages: messages.collect(),
            stop: vec![],
            temperature: 1.0,
        }
    }

    pub fn cancel_last_assist(&mut self) -> bool {
        self.pending_completions.pop().is_some()
    }

    pub fn cycle_message_roles(&mut self, ids: HashSet<MessageId>, cx: &mut ModelContext<Self>) {
        for id in ids {
            if let Some(metadata) = self.messages_metadata.get(&id) {
                let role = metadata.role.cycle();
                self.set_message_role(id, role, cx);
            }
        }
    }

    pub fn set_message_role(&mut self, id: MessageId, role: Role, cx: &mut ModelContext<Self>) {
        let metadata = self.messages_metadata.get_mut(&id).unwrap();
        metadata.role = role;
        metadata.timestamp = self.timestamp.tick();
        cx.emit(ContextEvent::Operation(ContextOperation::UpdateMessage {
            message_id: id,
            metadata: metadata.clone(),
        }));
        cx.emit(ContextEvent::MessagesEdited);
        cx.notify();
    }

    fn insert_message_after(
        &mut self,
        message_id: MessageId,
        role: Role,
        status: MessageStatus,
        cx: &mut ModelContext<Self>,
    ) -> Option<MessageAnchor> {
        if let Some(prev_message_ix) = self
            .message_anchors
            .iter()
            .position(|message| message.id == message_id)
        {
            // Find the next valid message after the one we were given.
            let mut next_message_ix = prev_message_ix + 1;
            while let Some(next_message) = self.message_anchors.get(next_message_ix) {
                if next_message.start.is_valid(self.buffer.read(cx)) {
                    break;
                }
                next_message_ix += 1;
            }

            let start = self.buffer.update(cx, |buffer, cx| {
                let offset = self
                    .message_anchors
                    .get(next_message_ix)
                    .map_or(buffer.len(), |message| message.start.to_offset(buffer) - 1);
                buffer.edit([(offset..offset, "\n")], None, cx);
                buffer.anchor_before(offset + 1)
            });

            let anchor = MessageAnchor {
                id: MessageId(self.timestamp.tick()),
                start,
            };
            let metadata = MessageMetadata {
                role,
                status,
                timestamp: anchor.id.0,
            };
            self.insert_message(next_message_ix, anchor.clone(), metadata, cx);
            Some(anchor)
        } else {
            None
        }
    }

    pub fn split_message(
        &mut self,
        range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) -> (Option<MessageAnchor>, Option<MessageAnchor>) {
        let start_message = self.message_for_offset(range.start, cx);
        let end_message = self.message_for_offset(range.end, cx);
        if let Some((start_message, end_message)) = start_message.zip(end_message) {
            // Prevent splitting when range spans multiple messages.
            if start_message.id != end_message.id {
                return (None, None);
            }

            let message = start_message;
            let role = message.role;
            let mut edited_buffer = false;

            let mut suffix_start = None;
            if range.start > message.offset_range.start && range.end < message.offset_range.end - 1
            {
                if self.buffer.read(cx).chars_at(range.end).next() == Some('\n') {
                    suffix_start = Some(range.end + 1);
                } else if self.buffer.read(cx).reversed_chars_at(range.end).next() == Some('\n') {
                    suffix_start = Some(range.end);
                }
            }

            let suffix = if let Some(suffix_start) = suffix_start {
                MessageAnchor {
                    id: MessageId(self.timestamp.tick()),
                    start: self.buffer.read(cx).anchor_before(suffix_start),
                }
            } else {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.edit([(range.end..range.end, "\n")], None, cx);
                });
                edited_buffer = true;
                MessageAnchor {
                    id: MessageId(self.timestamp.tick()),
                    start: self.buffer.read(cx).anchor_before(range.end + 1),
                }
            };

            self.insert_message(
                message.index_range.end + 1,
                suffix.clone(),
                MessageMetadata {
                    role,
                    status: MessageStatus::Done,
                    timestamp: suffix.id.0,
                },
                cx,
            );

            let new_messages =
                if range.start == range.end || range.start == message.offset_range.start {
                    (None, Some(suffix))
                } else {
                    let mut prefix_end = None;
                    if range.start > message.offset_range.start
                        && range.end < message.offset_range.end - 1
                    {
                        if self.buffer.read(cx).chars_at(range.start).next() == Some('\n') {
                            prefix_end = Some(range.start + 1);
                        } else if self.buffer.read(cx).reversed_chars_at(range.start).next()
                            == Some('\n')
                        {
                            prefix_end = Some(range.start);
                        }
                    }

                    let selection = if let Some(prefix_end) = prefix_end {
                        MessageAnchor {
                            id: MessageId(self.timestamp.tick()),
                            start: self.buffer.read(cx).anchor_before(prefix_end),
                        }
                    } else {
                        self.buffer.update(cx, |buffer, cx| {
                            buffer.edit([(range.start..range.start, "\n")], None, cx)
                        });
                        edited_buffer = true;
                        MessageAnchor {
                            id: MessageId(self.timestamp.tick()),
                            start: self.buffer.read(cx).anchor_before(range.end + 1),
                        }
                    };

                    self.insert_message(
                        message.index_range.end + 1,
                        selection.clone(),
                        MessageMetadata {
                            role,
                            status: MessageStatus::Done,
                            timestamp: selection.id.0,
                        },
                        cx,
                    );
                    (Some(selection), Some(suffix))
                };

            if !edited_buffer {
                cx.emit(ContextEvent::MessagesEdited);
            }
            new_messages
        } else {
            (None, None)
        }
    }

    fn insert_message(
        &mut self,
        index: usize,
        anchor: MessageAnchor,
        metadata: MessageMetadata,
        cx: &mut ModelContext<Self>,
    ) {
        assert!(index > 0, "inserting before the first message is forbidden");
        cx.emit(ContextEvent::Operation(ContextOperation::InsertMessage {
            id: anchor.id,
            left_sibling: self.message_anchors[index - 1].id,
            start: anchor.start,
            metadata: metadata.clone(),
        }));
        cx.emit(ContextEvent::MessagesEdited);
        self.messages_metadata.insert(anchor.id, metadata);
        self.message_anchors.insert(index, anchor);
    }

    fn summarize(&mut self, cx: &mut ModelContext<Self>) {
        if self.message_anchors.len() >= 2 && self.summary.is_none() {
            if !CompletionProvider::global(cx).is_authenticated() {
                return;
            }

            let messages = self
                .messages(cx)
                .map(|message| message.to_request_message(self.buffer.read(cx)))
                .chain(Some(LanguageModelRequestMessage {
                    role: Role::User,
                    content: "Summarize the context into a short title without punctuation.".into(),
                }));
            let request = LanguageModelRequest {
                model: CompletionProvider::global(cx).model(),
                messages: messages.collect(),
                stop: vec![],
                temperature: 1.0,
            };

            let stream = CompletionProvider::global(cx).complete(request);
            self.pending_summary = cx.spawn(|this, mut cx| {
                async move {
                    let mut messages = stream.await?;

                    while let Some(message) = messages.next().await {
                        let text = message?;
                        let mut lines = text.lines();
                        this.update(&mut cx, |this, cx| {
                            let summary = this.summary.get_or_insert(Default::default());
                            summary.text.extend(lines.next());
                            summary.timestamp = this.timestamp.tick();
                            cx.emit(ContextEvent::Operation(ContextOperation::UpdateSummary(
                                summary.clone(),
                            )));
                            cx.emit(ContextEvent::SummaryChanged);
                        })?;

                        // Stop if the LLM generated multiple lines.
                        if lines.next().is_some() {
                            break;
                        }
                    }

                    this.update(&mut cx, |this, cx| {
                        if let Some(summary) = this.summary.as_mut() {
                            summary.done = true;
                            summary.timestamp = this.timestamp.tick();
                            cx.emit(ContextEvent::Operation(ContextOperation::UpdateSummary(
                                summary.clone(),
                            )));
                            cx.emit(ContextEvent::SummaryChanged);
                        }
                    })?;

                    anyhow::Ok(())
                }
                .log_err()
            });
        }
    }

    fn message_for_offset(&self, offset: usize, cx: &AppContext) -> Option<Message> {
        self.messages_for_offsets([offset], cx).pop()
    }

    pub fn messages_for_offsets(
        &self,
        offsets: impl IntoIterator<Item = usize>,
        cx: &AppContext,
    ) -> Vec<Message> {
        let mut result = Vec::new();

        let mut messages = self.messages(cx).peekable();
        let mut offsets = offsets.into_iter().peekable();
        let mut current_message = messages.next();
        while let Some(offset) = offsets.next() {
            // Locate the message that contains the offset.
            while current_message.as_ref().map_or(false, |message| {
                !message.offset_range.contains(&offset) && messages.peek().is_some()
            }) {
                current_message = messages.next();
            }
            let Some(message) = current_message.as_ref() else {
                break;
            };

            // Skip offsets that are in the same message.
            while offsets.peek().map_or(false, |offset| {
                message.offset_range.contains(offset) || messages.peek().is_none()
            }) {
                offsets.next();
            }

            result.push(message.clone());
        }
        result
    }

    pub fn messages<'a>(&'a self, cx: &'a AppContext) -> impl 'a + Iterator<Item = Message> {
        let buffer = self.buffer.read(cx);
        let mut message_anchors = self.message_anchors.iter().enumerate().peekable();
        iter::from_fn(move || {
            if let Some((start_ix, message_anchor)) = message_anchors.next() {
                let metadata = self.messages_metadata.get(&message_anchor.id)?;
                let message_start = message_anchor.start.to_offset(buffer);
                let mut message_end = None;
                let mut end_ix = start_ix;
                while let Some((_, next_message)) = message_anchors.peek() {
                    if next_message.start.is_valid(buffer) {
                        message_end = Some(next_message.start);
                        break;
                    } else {
                        end_ix += 1;
                        message_anchors.next();
                    }
                }
                let message_end = message_end
                    .unwrap_or(language::Anchor::MAX)
                    .to_offset(buffer);

                return Some(Message {
                    index_range: start_ix..end_ix,
                    offset_range: message_start..message_end,
                    id: message_anchor.id,
                    anchor: message_anchor.start,
                    role: metadata.role,
                    status: metadata.status.clone(),
                });
            }
            None
        })
    }

    pub fn save(
        &mut self,
        debounce: Option<Duration>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Context>,
    ) {
        // todo!("if it's remote, don't save or maybe send a Save message.")

        self.pending_save = cx.spawn(|this, mut cx| async move {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }

            let (old_path, summary) = this.read_with(&cx, |this, _| {
                let path = this.path.clone();
                let summary = if let Some(summary) = this.summary.as_ref() {
                    if summary.done {
                        Some(summary.text.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };
                (path, summary)
            })?;

            if let Some(summary) = summary {
                let context = this.read_with(&cx, |this, cx| this.serialize(cx))?;
                let path = if let Some(old_path) = old_path {
                    old_path
                } else {
                    let mut discriminant = 1;
                    let mut new_path;
                    loop {
                        new_path = contexts_dir().join(&format!(
                            "{} - {}.zed.json",
                            summary.trim(),
                            discriminant
                        ));
                        if fs.is_file(&new_path).await {
                            discriminant += 1;
                        } else {
                            break;
                        }
                    }
                    new_path
                };

                fs.create_dir(contexts_dir().as_ref()).await?;
                fs.atomic_write(path.clone(), serde_json::to_string(&context).unwrap())
                    .await?;
                this.update(&mut cx, |this, _| this.path = Some(path))?;
            }

            Ok(())
        });
    }
}

#[derive(Debug)]
enum EditParsingState {
    None,
    InOldText {
        path: PathBuf,
        start_offset: usize,
        old_text_start_offset: usize,
    },
    InNewText {
        path: PathBuf,
        start_offset: usize,
        old_text_range: Range<usize>,
        new_text_start_offset: usize,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct EditSuggestion {
    pub source_range: Range<language::Anchor>,
    pub full_path: PathBuf,
}

pub struct ParsedEditSuggestion {
    pub path: PathBuf,
    pub outer_range: Range<usize>,
    pub old_text_range: Range<usize>,
    pub new_text_range: Range<usize>,
}

pub fn parse_next_edit_suggestion(lines: &mut rope::Lines) -> Option<ParsedEditSuggestion> {
    let mut state = EditParsingState::None;
    loop {
        let offset = lines.offset();
        let message_line = lines.next()?;
        match state {
            EditParsingState::None => {
                if let Some(rest) = message_line.strip_prefix("```edit ") {
                    let path = rest.trim();
                    if !path.is_empty() {
                        state = EditParsingState::InOldText {
                            path: PathBuf::from(path),
                            start_offset: offset,
                            old_text_start_offset: lines.offset(),
                        };
                    }
                }
            }
            EditParsingState::InOldText {
                path,
                start_offset,
                old_text_start_offset,
            } => {
                if message_line == "---" {
                    state = EditParsingState::InNewText {
                        path,
                        start_offset,
                        old_text_range: old_text_start_offset..offset,
                        new_text_start_offset: lines.offset(),
                    };
                } else {
                    state = EditParsingState::InOldText {
                        path,
                        start_offset,
                        old_text_start_offset,
                    };
                }
            }
            EditParsingState::InNewText {
                path,
                start_offset,
                old_text_range,
                new_text_start_offset,
            } => {
                if message_line == "```" {
                    return Some(ParsedEditSuggestion {
                        path,
                        outer_range: start_offset..offset + "```".len(),
                        old_text_range,
                        new_text_range: new_text_start_offset..offset,
                    });
                } else {
                    state = EditParsingState::InNewText {
                        path,
                        start_offset,
                        old_text_range,
                        new_text_start_offset,
                    };
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PendingSlashCommand {
    pub name: String,
    pub argument: Option<String>,
    pub status: PendingSlashCommandStatus,
    pub source_range: Range<language::Anchor>,
}

#[derive(Clone)]
pub enum PendingSlashCommandStatus {
    Idle,
    Running { _task: Shared<Task<()>> },
    Error(String),
}

#[derive(Serialize, Deserialize)]
struct SavedMessage {
    id: MessageId,
    start: usize,
}

#[derive(Serialize, Deserialize)]
struct SavedContext {
    id: Option<String>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessage>,
    message_metadata: HashMap<MessageId, MessageMetadata>,
    summary: String,
    slash_command_output_sections: Vec<assistant_slash_command::SlashCommandOutputSection<usize>>,
}

impl SavedContext {
    const VERSION: &'static str = "0.4.0";

    fn deserialize(json: &str) -> Result<Self> {
        let saved_context_json = serde_json::from_str::<serde_json::Value>(json)?;
        match saved_context_json
            .get("version")
            .ok_or_else(|| anyhow!("version not found"))?
        {
            serde_json::Value::String(version) => match version.as_str() {
                SavedContext::VERSION => {
                    Ok(serde_json::from_value::<SavedContext>(saved_context_json)?)
                }
                SavedContextV0_3_0::VERSION => {
                    let saved_context =
                        serde_json::from_value::<SavedContextV0_3_0>(saved_context_json)?;
                    Ok(saved_context.upgrade())
                }
                SavedContextV0_2_0::VERSION => {
                    let saved_context =
                        serde_json::from_value::<SavedContextV0_2_0>(saved_context_json)?;
                    Ok(saved_context.upgrade())
                }
                SavedContextV0_1_0::VERSION => {
                    let saved_context =
                        serde_json::from_value::<SavedContextV0_1_0>(saved_context_json)?;
                    Ok(saved_context.upgrade())
                }
                _ => Err(anyhow!("unrecognized saved context version: {}", version)),
            },
            _ => Err(anyhow!("version not found on saved context")),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct SavedMessageIdPreV0_4_0(usize);

#[derive(Serialize, Deserialize)]
struct SavedMessagePreV0_4_0 {
    id: SavedMessageIdPreV0_4_0,
    start: usize,
}

#[derive(Serialize, Deserialize)]
struct SavedContextV0_3_0 {
    id: Option<String>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessagePreV0_4_0>,
    message_metadata: HashMap<SavedMessageIdPreV0_4_0, MessageMetadata>,
    summary: String,
    slash_command_output_sections: Vec<assistant_slash_command::SlashCommandOutputSection<usize>>,
}

impl SavedContextV0_3_0 {
    const VERSION: &'static str = "0.3.0";

    fn upgrade(self) -> SavedContext {
        SavedContext {
            id: self.id,
            zed: self.zed,
            version: SavedContext::VERSION.into(),
            text: self.text,
            messages: self
                .messages
                .into_iter()
                .map(|message| SavedMessage {
                    id: MessageId(clock::Lamport {
                        replica_id: ReplicaId::default(),
                        value: message.id.0 as u32,
                    }),
                    start: message.start,
                })
                .collect(),
            message_metadata: self
                .message_metadata
                .into_iter()
                .map(|(id, metadata)| {
                    (
                        MessageId(clock::Lamport {
                            replica_id: ReplicaId::default(),
                            value: id.0 as u32,
                        }),
                        metadata,
                    )
                })
                .collect(),
            summary: self.summary,
            slash_command_output_sections: self.slash_command_output_sections,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SavedContextV0_2_0 {
    id: Option<String>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessagePreV0_4_0>,
    message_metadata: HashMap<SavedMessageIdPreV0_4_0, MessageMetadata>,
    summary: String,
}

impl SavedContextV0_2_0 {
    const VERSION: &'static str = "0.2.0";

    fn upgrade(self) -> SavedContext {
        SavedContextV0_3_0 {
            id: self.id,
            zed: self.zed,
            version: SavedContextV0_3_0::VERSION.to_string(),
            text: self.text,
            messages: self.messages,
            message_metadata: self.message_metadata,
            summary: self.summary,
            slash_command_output_sections: Vec::new(),
        }
        .upgrade()
    }
}

#[derive(Serialize, Deserialize)]
struct SavedContextV0_1_0 {
    id: Option<String>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessagePreV0_4_0>,
    message_metadata: HashMap<SavedMessageIdPreV0_4_0, MessageMetadata>,
    summary: String,
    api_url: Option<String>,
    model: OpenAiModel,
}

impl SavedContextV0_1_0 {
    const VERSION: &'static str = "0.1.0";

    fn upgrade(self) -> SavedContext {
        SavedContextV0_2_0 {
            id: self.id,
            zed: self.zed,
            version: SavedContextV0_2_0::VERSION.to_string(),
            text: self.text,
            messages: self.messages,
            message_metadata: self.message_metadata,
            summary: self.summary,
        }
        .upgrade()
    }
}

#[derive(Clone)]
pub struct SavedContextMetadata {
    pub title: String,
    pub path: PathBuf,
    pub mtime: chrono::DateTime<chrono::Local>,
}

pub struct ContextStore {
    contexts: Vec<ContextHandle>,
    contexts_metadata: Vec<SavedContextMetadata>,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    telemetry: Option<Arc<Telemetry>>,
    _watch_updates: Task<Option<()>>,
    client: Arc<Client>,
    project: Model<Project>,
    project_is_shared: bool,
    client_subscription: Option<client::Subscription>,
    _project_subscription: gpui::Subscription,
}

enum ContextHandle {
    Weak(WeakModel<Context>),
    Strong(Model<Context>),
}

impl ContextHandle {
    fn upgrade(&self) -> Option<Model<Context>> {
        match self {
            ContextHandle::Weak(weak) => weak.upgrade(),
            ContextHandle::Strong(strong) => Some(strong.clone()),
        }
    }

    fn downgrade(&self) -> WeakModel<Context> {
        match self {
            ContextHandle::Weak(weak) => weak.clone(),
            ContextHandle::Strong(strong) => strong.downgrade(),
        }
    }
}

impl ContextStore {
    pub fn new(
        project: Model<Project>,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            const CONTEXT_WATCH_DURATION: Duration = Duration::from_millis(100);
            let (mut events, _) = fs.watch(contexts_dir(), CONTEXT_WATCH_DURATION).await;

            let this = cx.new_model(|cx: &mut ModelContext<Self>| {
                let mut this = Self {
                    contexts: Vec::new(),
                    contexts_metadata: Vec::new(),
                    fs,
                    languages,
                    telemetry,
                    _watch_updates: cx.spawn(|this, mut cx| {
                        async move {
                            while events.next().await.is_some() {
                                this.update(&mut cx, |this, cx| this.reload(cx))?
                                    .await
                                    .log_err();
                            }
                            anyhow::Ok(())
                        }
                        .log_err()
                    }),
                    client_subscription: None,
                    _project_subscription: cx.observe(&project, Self::project_changed),
                    project_is_shared: false,
                    client: project.read(cx).client(),
                    project: project.clone(),
                };
                this.register_handlers();
                this.project_changed(project, cx);
                this
            })?;
            this.update(&mut cx, |this, cx| this.reload(cx))?
                .await
                .log_err();
            Ok(this)
        })
    }

    fn register_handlers(&self) {
        todo!();
        // self.client
        //     .add_model_request_handler(Self::handle_open_context);
        // self.client
        //     .add_model_request_handler(Self::handle_context_update);
        // self.client.add_model_request_handler(Self::handle_resync);
    }

    fn project_changed(&mut self, _: Model<Project>, cx: &mut ModelContext<Self>) {
        let is_shared = self.project.read(cx).is_shared();
        let was_shared = mem::replace(&mut self.project_is_shared, is_shared);
        if is_shared == was_shared {
            return;
        }

        if is_shared {
            self.contexts.retain_mut(|context| {
                *context = ContextHandle::Weak(context.downgrade());
                true
            });
            let remote_id = self.project.read(cx).remote_id().unwrap();
            self.client_subscription = self
                .client
                .subscribe_to_entity(remote_id)
                .log_err()
                .map(|subscription| subscription.set_model(&cx.handle(), &mut cx.to_async()));
        } else {
            self.contexts.retain_mut(|context| {
                if let Some(strong_context) = context.upgrade() {
                    *context = ContextHandle::Strong(strong_context);
                    true
                } else {
                    false
                }
            });
            self.client_subscription = None;
        }
    }

    pub fn create(&mut self, cx: &mut ModelContext<Self>) -> Model<Context> {
        let context =
            cx.new_model(|cx| Context::new(self.languages.clone(), self.telemetry.clone(), cx));
        self.register_context(&context);
        context
    }

    pub fn load(&mut self, path: PathBuf, cx: &ModelContext<Self>) -> Task<Result<Model<Context>>> {
        if let Some(existing_context) = self.loaded_context_for_path(&path, cx) {
            return Task::ready(Ok(existing_context));
        }

        let fs = self.fs.clone();
        let languages = self.languages.clone();
        let telemetry = self.telemetry.clone();
        let load = cx.background_executor().spawn({
            let path = path.clone();
            async move {
                let saved_context = fs.load(&path).await?;
                SavedContext::deserialize(&saved_context)
            }
        });

        cx.spawn(|this, mut cx| async move {
            let saved_context = load.await?;
            let context =
                Context::deserialize(saved_context, path.clone(), languages, telemetry, &mut cx)
                    .await?;
            this.update(&mut cx, |this, cx| {
                if let Some(existing_context) = this.loaded_context_for_path(&path, cx) {
                    existing_context
                } else {
                    this.register_context(&context);
                    context
                }
            })
        })
    }

    fn loaded_context_for_path(&self, path: &Path, cx: &AppContext) -> Option<Model<Context>> {
        self.contexts.iter().find_map(|context| {
            let context = context.upgrade()?;
            if context.read(cx).path.as_deref() == Some(path) {
                Some(context)
            } else {
                None
            }
        })
    }

    fn register_context(&mut self, context: &Model<Context>) {
        let handle = if self.project_is_shared {
            ContextHandle::Strong(context.clone())
        } else {
            ContextHandle::Weak(context.downgrade())
        };
        self.contexts.push(handle);
    }

    pub fn search(&self, query: String, cx: &AppContext) -> Task<Vec<SavedContextMetadata>> {
        let metadata = self.contexts_metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_executor().spawn(async move {
            if query.is_empty() {
                metadata
            } else {
                let candidates = metadata
                    .iter()
                    .enumerate()
                    .map(|(id, metadata)| StringMatchCandidate::new(id, metadata.title.clone()))
                    .collect::<Vec<_>>();
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    executor,
                )
                .await;

                matches
                    .into_iter()
                    .map(|mat| metadata[mat.candidate_id].clone())
                    .collect()
            }
        })
    }

    fn reload(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            fs.create_dir(contexts_dir()).await?;

            let mut paths = fs.read_dir(contexts_dir()).await?;
            let mut contexts = Vec::<SavedContextMetadata>::new();
            while let Some(path) = paths.next().await {
                let path = path?;
                if path.extension() != Some(OsStr::new("json")) {
                    continue;
                }

                let pattern = r" - \d+.zed.json$";
                let re = Regex::new(pattern).unwrap();

                let metadata = fs.metadata(&path).await?;
                if let Some((file_name, metadata)) = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .zip(metadata)
                {
                    // This is used to filter out contexts saved by the new assistant.
                    if !re.is_match(file_name) {
                        continue;
                    }

                    if let Some(title) = re.replace(file_name, "").lines().next() {
                        contexts.push(SavedContextMetadata {
                            title: title.to_string(),
                            path,
                            mtime: metadata.mtime.into(),
                        });
                    }
                }
            }
            contexts.sort_unstable_by_key(|context| Reverse(context.mtime));

            this.update(&mut cx, |this, cx| {
                this.contexts_metadata = contexts;
                cx.notify();
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assistant_panel,
        slash_command::{active_command, file_command},
        FakeCompletionProvider, MessageId,
    };
    use fs::FakeFs;
    use gpui::{AppContext, TestAppContext};
    use project::Project;
    use rope::Rope;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{cell::RefCell, path::Path, rc::Rc};
    use unindent::Unindent;
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_inserting_and_removing_messages(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.set_global(settings_store);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

        let context = cx.new_model(|cx| Context::new(registry, None, cx));
        let buffer = context.read(cx).buffer.clone();

        let message_1 = context.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&context, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        let message_2 = context.update(cx, |context, cx| {
            context
                .insert_message_after(message_1.id, Role::Assistant, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..1),
                (message_2.id, Role::Assistant, 1..1)
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "1"), (1..1, "2")], None, cx)
        });
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..3)
            ]
        );

        let message_3 = context.update(cx, |context, cx| {
            context
                .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_3.id, Role::User, 4..4)
            ]
        );

        let message_4 = context.update(cx, |context, cx| {
            context
                .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..5),
                (message_3.id, Role::User, 5..5),
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(4..4, "C"), (5..5, "D")], None, cx)
        });
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..6),
                (message_3.id, Role::User, 6..7),
            ]
        );

        // Deleting across message boundaries merges the messages.
        buffer.update(cx, |buffer, cx| buffer.edit([(1..4, "")], None, cx));
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Undoing the deletion should also undo the merge.
        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..6),
                (message_3.id, Role::User, 6..7),
            ]
        );

        // Redoing the deletion should also redo the merge.
        buffer.update(cx, |buffer, cx| buffer.redo(cx));
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Ensure we can still insert after a merged message.
        let message_5 = context.update(cx, |context, cx| {
            context
                .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_5.id, Role::System, 3..4),
                (message_3.id, Role::User, 4..5)
            ]
        );
    }

    #[gpui::test]
    fn test_message_splitting(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

        let context = cx.new_model(|cx| Context::new(registry, None, cx));
        let buffer = context.read(cx).buffer.clone();

        let message_1 = context.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&context, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "aaa\nbbb\nccc\nddd\n")], None, cx)
        });

        let (_, message_2) = context.update(cx, |context, cx| context.split_message(3..3, cx));
        let message_2 = message_2.unwrap();

        // We recycle newlines in the middle of a split message
        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..16),
            ]
        );

        let (_, message_3) = context.update(cx, |context, cx| context.split_message(3..3, cx));
        let message_3 = message_3.unwrap();

        // We don't recycle newlines at the end of a split message
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..17),
            ]
        );

        let (_, message_4) = context.update(cx, |context, cx| context.split_message(9..9, cx));
        let message_4 = message_4.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..17),
            ]
        );

        let (_, message_5) = context.update(cx, |context, cx| context.split_message(9..9, cx));
        let message_5 = message_5.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\nddd\n");
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..10),
                (message_5.id, Role::User, 10..18),
            ]
        );

        let (message_6, message_7) =
            context.update(cx, |context, cx| context.split_message(14..16, cx));
        let message_6 = message_6.unwrap();
        let message_7 = message_7.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\ndd\nd\n");
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..10),
                (message_5.id, Role::User, 10..14),
                (message_6.id, Role::User, 14..17),
                (message_7.id, Role::User, 17..19),
            ]
        );
    }

    #[gpui::test]
    fn test_messages_for_offsets(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.set_global(settings_store);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let context = cx.new_model(|cx| Context::new(registry, None, cx));
        let buffer = context.read(cx).buffer.clone();

        let message_1 = context.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&context, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "aaa")], None, cx));
        let message_2 = context
            .update(cx, |context, cx| {
                context.insert_message_after(message_1.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "bbb")], None, cx));

        let message_3 = context
            .update(cx, |context, cx| {
                context.insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(8..8, "ccc")], None, cx));

        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc");
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..8),
                (message_3.id, Role::User, 8..11)
            ]
        );

        assert_eq!(
            message_ids_for_offsets(&context, &[0, 4, 9], cx),
            [message_1.id, message_2.id, message_3.id]
        );
        assert_eq!(
            message_ids_for_offsets(&context, &[0, 1, 11], cx),
            [message_1.id, message_3.id]
        );

        let message_4 = context
            .update(cx, |context, cx| {
                context.insert_message_after(message_3.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\n");
        assert_eq!(
            messages(&context, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..8),
                (message_3.id, Role::User, 8..12),
                (message_4.id, Role::User, 12..12)
            ]
        );
        assert_eq!(
            message_ids_for_offsets(&context, &[0, 4, 8, 12], cx),
            [message_1.id, message_2.id, message_3.id, message_4.id]
        );

        fn message_ids_for_offsets(
            context: &Model<Context>,
            offsets: &[usize],
            cx: &AppContext,
        ) -> Vec<MessageId> {
            context
                .read(cx)
                .messages_for_offsets(offsets.iter().copied(), cx)
                .into_iter()
                .map(|message| message.id)
                .collect()
        }
    }

    #[gpui::test]
    async fn test_slash_commands(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.update(Project::init_settings);
        cx.update(assistant_panel::init);
        let fs = FakeFs::new(cx.background_executor.clone());

        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }",
                    "main.rs": "
                        use crate::one;
                        fn main() { one(); }
                    ".unindent(),
                }
            }),
        )
        .await;

        let slash_command_registry = cx.update(SlashCommandRegistry::default_global);
        slash_command_registry.register_command(file_command::FileSlashCommand, false);
        slash_command_registry.register_command(active_command::ActiveSlashCommand, false);

        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let context = cx.new_model(|cx| Context::new(registry.clone(), None, cx));

        let output_ranges = Rc::new(RefCell::new(HashSet::default()));
        context.update(cx, |_, cx| {
            cx.subscribe(&context, {
                let ranges = output_ranges.clone();
                move |_, _, event, _| match event {
                    ContextEvent::PendingSlashCommandsUpdated { removed, updated } => {
                        for range in removed {
                            ranges.borrow_mut().remove(range);
                        }
                        for command in updated {
                            ranges.borrow_mut().insert(command.source_range.clone());
                        }
                    }
                    _ => {}
                }
            })
            .detach();
        });

        let buffer = context.read_with(cx, |context, _| context.buffer.clone());

        // Insert a slash command
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "/file src/lib.rs")], None, cx);
        });
        assert_text_and_output_ranges(
            &buffer,
            &output_ranges.borrow(),
            "
            «/file src/lib.rs»
            "
            .unindent()
            .trim_end(),
            cx,
        );

        // Edit the argument of the slash command.
        buffer.update(cx, |buffer, cx| {
            let edit_offset = buffer.text().find("lib.rs").unwrap();
            buffer.edit([(edit_offset..edit_offset + "lib".len(), "main")], None, cx);
        });
        assert_text_and_output_ranges(
            &buffer,
            &output_ranges.borrow(),
            "
            «/file src/main.rs»
            "
            .unindent()
            .trim_end(),
            cx,
        );

        // Edit the name of the slash command, using one that doesn't exist.
        buffer.update(cx, |buffer, cx| {
            let edit_offset = buffer.text().find("/file").unwrap();
            buffer.edit(
                [(edit_offset..edit_offset + "/file".len(), "/unknown")],
                None,
                cx,
            );
        });
        assert_text_and_output_ranges(
            &buffer,
            &output_ranges.borrow(),
            "
            /unknown src/main.rs
            "
            .unindent()
            .trim_end(),
            cx,
        );

        #[track_caller]
        fn assert_text_and_output_ranges(
            buffer: &Model<Buffer>,
            ranges: &HashSet<Range<language::Anchor>>,
            expected_marked_text: &str,
            cx: &mut TestAppContext,
        ) {
            let (expected_text, expected_ranges) = marked_text_ranges(expected_marked_text, false);
            let (actual_text, actual_ranges) = buffer.update(cx, |buffer, _| {
                let mut ranges = ranges
                    .iter()
                    .map(|range| range.to_offset(buffer))
                    .collect::<Vec<_>>();
                ranges.sort_by_key(|a| a.start);
                (buffer.text(), ranges)
            });

            assert_eq!(actual_text, expected_text);
            assert_eq!(actual_ranges, expected_ranges);
        }
    }

    #[test]
    fn test_parse_next_edit_suggestion() {
        let text = "
            some output:

            ```edit src/foo.rs
                let a = 1;
                let b = 2;
            ---
                let w = 1;
                let x = 2;
                let y = 3;
                let z = 4;
            ```

            some more output:

            ```edit src/foo.rs
                let c = 1;
            ---
            ```

            and the conclusion.
        "
        .unindent();

        let rope = Rope::from(text.as_str());
        let mut lines = rope.chunks().lines();
        let mut suggestions = vec![];
        while let Some(suggestion) = parse_next_edit_suggestion(&mut lines) {
            suggestions.push((
                suggestion.path.clone(),
                text[suggestion.old_text_range].to_string(),
                text[suggestion.new_text_range].to_string(),
            ));
        }

        assert_eq!(
            suggestions,
            vec![
                (
                    Path::new("src/foo.rs").into(),
                    [
                        "    let a = 1;", //
                        "    let b = 2;",
                        "",
                    ]
                    .join("\n"),
                    [
                        "    let w = 1;",
                        "    let x = 2;",
                        "    let y = 3;",
                        "    let z = 4;",
                        "",
                    ]
                    .join("\n"),
                ),
                (
                    Path::new("src/foo.rs").into(),
                    [
                        "    let c = 1;", //
                        "",
                    ]
                    .join("\n"),
                    String::new(),
                )
            ]
        );
    }

    #[gpui::test]
    async fn test_serialization(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.update(assistant_panel::init);
        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let context = cx.new_model(|cx| Context::new(registry.clone(), None, cx));
        let buffer = context.read_with(cx, |context, _| context.buffer.clone());
        let message_0 = context.read_with(cx, |context, _| context.message_anchors[0].id);
        let message_1 = context.update(cx, |context, cx| {
            context
                .insert_message_after(message_0, Role::Assistant, MessageStatus::Done, cx)
                .unwrap()
        });
        let message_2 = context.update(cx, |context, cx| {
            context
                .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "a"), (1..1, "b\nc")], None, cx);
            buffer.finalize_last_transaction();
        });
        let _message_3 = context.update(cx, |context, cx| {
            context
                .insert_message_after(message_2.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(buffer.read_with(cx, |buffer, _| buffer.text()), "a\nb\nc\n");
        assert_eq!(
            cx.read(|cx| messages(&context, cx)),
            [
                (message_0, Role::User, 0..2),
                (message_1.id, Role::Assistant, 2..6),
                (message_2.id, Role::System, 6..6),
            ]
        );

        let deserialized_context = Context::deserialize(
            context.read_with(cx, |context, cx| context.serialize(cx)),
            Default::default(),
            registry.clone(),
            None,
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        let deserialized_buffer =
            deserialized_context.read_with(cx, |context, _| context.buffer.clone());
        assert_eq!(
            deserialized_buffer.read_with(cx, |buffer, _| buffer.text()),
            "a\nb\nc\n"
        );
        assert_eq!(
            cx.read(|cx| messages(&deserialized_context, cx)),
            [
                (message_0, Role::User, 0..2),
                (message_1.id, Role::Assistant, 2..6),
                (message_2.id, Role::System, 6..6),
            ]
        );
    }

    fn messages(context: &Model<Context>, cx: &AppContext) -> Vec<(MessageId, Role, Range<usize>)> {
        context
            .read(cx)
            .messages(cx)
            .map(|message| (message.id, message.role, message.offset_range))
            .collect()
    }
}
