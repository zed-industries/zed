use crate::{
    prompt_library::PromptStore, slash_command::SlashCommandLine, CompletionProvider, MessageId,
    MessageStatus,
};
use anyhow::{anyhow, Context as _, Result};
use assistant_slash_command::{
    SlashCommandOutput, SlashCommandOutputSection, SlashCommandRegistry,
};
use client::{self, proto, telemetry::Telemetry};
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{
    future::{self, Shared},
    FutureExt, StreamExt,
};
use gpui::{AppContext, Context as _, EventEmitter, Model, ModelContext, Subscription, Task};
use language::{
    AnchorRangeExt, Bias, Buffer, LanguageRegistry, OffsetRangeExt, ParseStatus, Point, ToOffset,
};
use language_model::LanguageModelRequestMessage;
use language_model::{LanguageModelRequest, Role};
use open_ai::Model as OpenAiModel;
use paths::contexts_dir;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{
    cmp,
    fmt::Debug,
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

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ContextId(String);

impl ContextId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_proto(id: String) -> Self {
        Self(id)
    }

    pub fn to_proto(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
pub enum ContextOperation {
    InsertMessage {
        anchor: MessageAnchor,
        metadata: MessageMetadata,
        version: clock::Global,
    },
    UpdateMessage {
        message_id: MessageId,
        metadata: MessageMetadata,
        version: clock::Global,
    },
    UpdateSummary {
        summary: ContextSummary,
        version: clock::Global,
    },
    SlashCommandFinished {
        id: SlashCommandId,
        output_range: Range<language::Anchor>,
        sections: Vec<SlashCommandOutputSection<language::Anchor>>,
        version: clock::Global,
    },
    BufferOperation(language::Operation),
}

impl ContextOperation {
    pub fn from_proto(op: proto::ContextOperation) -> Result<Self> {
        match op.variant.context("invalid variant")? {
            proto::context_operation::Variant::InsertMessage(insert) => {
                let message = insert.message.context("invalid message")?;
                let id = MessageId(language::proto::deserialize_timestamp(
                    message.id.context("invalid id")?,
                ));
                Ok(Self::InsertMessage {
                    anchor: MessageAnchor {
                        id,
                        start: language::proto::deserialize_anchor(
                            message.start.context("invalid anchor")?,
                        )
                        .context("invalid anchor")?,
                    },
                    metadata: MessageMetadata {
                        role: Role::from_proto(message.role),
                        status: MessageStatus::from_proto(
                            message.status.context("invalid status")?,
                        ),
                        timestamp: id.0,
                    },
                    version: language::proto::deserialize_version(&insert.version),
                })
            }
            proto::context_operation::Variant::UpdateMessage(update) => Ok(Self::UpdateMessage {
                message_id: MessageId(language::proto::deserialize_timestamp(
                    update.message_id.context("invalid message id")?,
                )),
                metadata: MessageMetadata {
                    role: Role::from_proto(update.role),
                    status: MessageStatus::from_proto(update.status.context("invalid status")?),
                    timestamp: language::proto::deserialize_timestamp(
                        update.timestamp.context("invalid timestamp")?,
                    ),
                },
                version: language::proto::deserialize_version(&update.version),
            }),
            proto::context_operation::Variant::UpdateSummary(update) => Ok(Self::UpdateSummary {
                summary: ContextSummary {
                    text: update.summary,
                    done: update.done,
                    timestamp: language::proto::deserialize_timestamp(
                        update.timestamp.context("invalid timestamp")?,
                    ),
                },
                version: language::proto::deserialize_version(&update.version),
            }),
            proto::context_operation::Variant::SlashCommandFinished(finished) => {
                Ok(Self::SlashCommandFinished {
                    id: SlashCommandId(language::proto::deserialize_timestamp(
                        finished.id.context("invalid id")?,
                    )),
                    output_range: language::proto::deserialize_anchor_range(
                        finished.output_range.context("invalid range")?,
                    )?,
                    sections: finished
                        .sections
                        .into_iter()
                        .map(|section| {
                            Ok(SlashCommandOutputSection {
                                range: language::proto::deserialize_anchor_range(
                                    section.range.context("invalid range")?,
                                )?,
                                icon: section.icon_name.parse()?,
                                label: section.label.into(),
                            })
                        })
                        .collect::<Result<Vec<_>>>()?,
                    version: language::proto::deserialize_version(&finished.version),
                })
            }
            proto::context_operation::Variant::BufferOperation(op) => Ok(Self::BufferOperation(
                language::proto::deserialize_operation(
                    op.operation.context("invalid buffer operation")?,
                )?,
            )),
        }
    }

    pub fn to_proto(&self) -> proto::ContextOperation {
        match self {
            Self::InsertMessage {
                anchor,
                metadata,
                version,
            } => proto::ContextOperation {
                variant: Some(proto::context_operation::Variant::InsertMessage(
                    proto::context_operation::InsertMessage {
                        message: Some(proto::ContextMessage {
                            id: Some(language::proto::serialize_timestamp(anchor.id.0)),
                            start: Some(language::proto::serialize_anchor(&anchor.start)),
                            role: metadata.role.to_proto() as i32,
                            status: Some(metadata.status.to_proto()),
                        }),
                        version: language::proto::serialize_version(version),
                    },
                )),
            },
            Self::UpdateMessage {
                message_id,
                metadata,
                version,
            } => proto::ContextOperation {
                variant: Some(proto::context_operation::Variant::UpdateMessage(
                    proto::context_operation::UpdateMessage {
                        message_id: Some(language::proto::serialize_timestamp(message_id.0)),
                        role: metadata.role.to_proto() as i32,
                        status: Some(metadata.status.to_proto()),
                        timestamp: Some(language::proto::serialize_timestamp(metadata.timestamp)),
                        version: language::proto::serialize_version(version),
                    },
                )),
            },
            Self::UpdateSummary { summary, version } => proto::ContextOperation {
                variant: Some(proto::context_operation::Variant::UpdateSummary(
                    proto::context_operation::UpdateSummary {
                        summary: summary.text.clone(),
                        done: summary.done,
                        timestamp: Some(language::proto::serialize_timestamp(summary.timestamp)),
                        version: language::proto::serialize_version(version),
                    },
                )),
            },
            Self::SlashCommandFinished {
                id,
                output_range,
                sections,
                version,
            } => proto::ContextOperation {
                variant: Some(proto::context_operation::Variant::SlashCommandFinished(
                    proto::context_operation::SlashCommandFinished {
                        id: Some(language::proto::serialize_timestamp(id.0)),
                        output_range: Some(language::proto::serialize_anchor_range(
                            output_range.clone(),
                        )),
                        sections: sections
                            .iter()
                            .map(|section| {
                                let icon_name: &'static str = section.icon.into();
                                proto::SlashCommandOutputSection {
                                    range: Some(language::proto::serialize_anchor_range(
                                        section.range.clone(),
                                    )),
                                    icon_name: icon_name.to_string(),
                                    label: section.label.to_string(),
                                }
                            })
                            .collect(),
                        version: language::proto::serialize_version(version),
                    },
                )),
            },
            Self::BufferOperation(operation) => proto::ContextOperation {
                variant: Some(proto::context_operation::Variant::BufferOperation(
                    proto::context_operation::BufferOperation {
                        operation: Some(language::proto::serialize_operation(operation)),
                    },
                )),
            },
        }
    }

    fn timestamp(&self) -> clock::Lamport {
        match self {
            Self::InsertMessage { anchor, .. } => anchor.id.0,
            Self::UpdateMessage { metadata, .. } => metadata.timestamp,
            Self::UpdateSummary { summary, .. } => summary.timestamp,
            Self::SlashCommandFinished { id, .. } => id.0,
            Self::BufferOperation(_) => {
                panic!("reading the timestamp of a buffer operation is not supported")
            }
        }
    }

    /// Returns the current version of the context operation.
    pub fn version(&self) -> &clock::Global {
        match self {
            Self::InsertMessage { version, .. }
            | Self::UpdateMessage { version, .. }
            | Self::UpdateSummary { version, .. }
            | Self::SlashCommandFinished { version, .. } => version,
            Self::BufferOperation(_) => {
                panic!("reading the version of a buffer operation is not supported")
            }
        }
    }
}

#[derive(Clone)]
pub enum ContextEvent {
    MessagesEdited,
    SummaryChanged,
    EditStepsChanged,
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

#[derive(Clone, Default, Debug)]
pub struct ContextSummary {
    pub text: String,
    done: bool,
    timestamp: clock::Lamport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageAnchor {
    pub id: MessageId,
    pub start: language::Anchor,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub role: Role,
    status: MessageStatus,
    timestamp: clock::Lamport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SlashCommandId(clock::Lamport);

#[derive(Debug)]
pub struct EditStep {
    pub source_range: Range<language::Anchor>,
    pub operations: Option<EditStepOperations>,
}

#[derive(Debug)]
pub struct EditSuggestionGroup {
    pub context_range: Range<language::Anchor>,
    pub suggestions: Vec<EditSuggestion>,
}

#[derive(Debug)]
pub struct EditSuggestion {
    pub range: Range<language::Anchor>,
    /// If None, assume this is a suggestion to delete the range rather than transform it.
    pub description: Option<String>,
    pub prepend_newline: bool,
}

impl EditStep {
    pub fn edit_suggestions(
        &self,
        project: &Model<Project>,
        cx: &AppContext,
    ) -> Task<HashMap<Model<Buffer>, Vec<EditSuggestionGroup>>> {
        let Some(EditStepOperations::Parsed { operations, .. }) = &self.operations else {
            return Task::ready(HashMap::default());
        };

        let suggestion_tasks: Vec<_> = operations
            .iter()
            .map(|operation| operation.edit_suggestion(project.clone(), cx))
            .collect();

        cx.spawn(|mut cx| async move {
            let suggestions = future::join_all(suggestion_tasks)
                .await
                .into_iter()
                .filter_map(|task| task.log_err())
                .collect::<Vec<_>>();

            let mut suggestions_by_buffer = HashMap::default();
            for (buffer, suggestion) in suggestions {
                suggestions_by_buffer
                    .entry(buffer)
                    .or_insert_with(Vec::new)
                    .push(suggestion);
            }

            let mut suggestion_groups_by_buffer = HashMap::default();
            for (buffer, mut suggestions) in suggestions_by_buffer {
                let mut suggestion_groups = Vec::<EditSuggestionGroup>::new();
                buffer
                    .update(&mut cx, |buffer, _cx| {
                        // Sort suggestions by their range
                        suggestions.sort_by(|a, b| a.range.cmp(&b.range, buffer));

                        // Dedup overlapping suggestions
                        suggestions.dedup_by(|a, b| {
                            let a_range = a.range.to_offset(buffer);
                            let b_range = b.range.to_offset(buffer);
                            if a_range.start <= b_range.end && b_range.start <= a_range.end {
                                if b_range.start < a_range.start {
                                    a.range.start = b.range.start;
                                }
                                if b_range.end > a_range.end {
                                    a.range.end = b.range.end;
                                }

                                if let (Some(a_desc), Some(b_desc)) =
                                    (a.description.as_mut(), b.description.as_mut())
                                {
                                    b_desc.push('\n');
                                    b_desc.push_str(a_desc);
                                } else if a.description.is_some() {
                                    b.description = a.description.take();
                                }

                                true
                            } else {
                                false
                            }
                        });

                        // Create context ranges for each suggestion
                        for suggestion in suggestions {
                            let context_range = {
                                let suggestion_point_range = suggestion.range.to_point(buffer);
                                let start_row = suggestion_point_range.start.row.saturating_sub(5);
                                let end_row = cmp::min(
                                    suggestion_point_range.end.row + 5,
                                    buffer.max_point().row,
                                );
                                let start = buffer.anchor_before(Point::new(start_row, 0));
                                let end = buffer
                                    .anchor_after(Point::new(end_row, buffer.line_len(end_row)));
                                start..end
                            };

                            if let Some(last_group) = suggestion_groups.last_mut() {
                                if last_group
                                    .context_range
                                    .end
                                    .cmp(&context_range.start, buffer)
                                    .is_ge()
                                {
                                    // Merge with the previous group if context ranges overlap
                                    last_group.context_range.end = context_range.end;
                                    last_group.suggestions.push(suggestion);
                                } else {
                                    // Create a new group
                                    suggestion_groups.push(EditSuggestionGroup {
                                        context_range,
                                        suggestions: vec![suggestion],
                                    });
                                }
                            } else {
                                // Create the first group
                                suggestion_groups.push(EditSuggestionGroup {
                                    context_range,
                                    suggestions: vec![suggestion],
                                });
                            }
                        }
                    })
                    .ok();
                suggestion_groups_by_buffer.insert(buffer, suggestion_groups);
            }

            suggestion_groups_by_buffer
        })
    }
}

pub enum EditStepOperations {
    Pending(Task<Result<()>>),
    Parsed {
        operations: Vec<EditOperation>,
        raw_output: String,
    },
}

impl Debug for EditStepOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditStepOperations::Pending(_) => write!(f, "EditStepOperations::Pending"),
            EditStepOperations::Parsed {
                operations,
                raw_output,
            } => f
                .debug_struct("EditStepOperations::Parsed")
                .field("operations", operations)
                .field("raw_output", raw_output)
                .finish(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditOperation {
    pub path: String,
    pub kind: EditOperationKind,
}

impl EditOperation {
    fn edit_suggestion(
        &self,
        project: Model<Project>,
        cx: &AppContext,
    ) -> Task<Result<(Model<language::Buffer>, EditSuggestion)>> {
        let path = self.path.clone();
        let kind = self.kind.clone();
        cx.spawn(move |mut cx| async move {
            let buffer = project
                .update(&mut cx, |project, cx| {
                    let project_path = project
                        .project_path_for_full_path(Path::new(&path), cx)
                        .with_context(|| format!("worktree not found for {:?}", path))?;
                    anyhow::Ok(project.open_buffer(project_path, cx))
                })??
                .await?;

            let mut parse_status = buffer.read_with(&cx, |buffer, _cx| buffer.parse_status())?;
            while *parse_status.borrow() != ParseStatus::Idle {
                parse_status.changed().await?;
            }

            let prepend_newline = kind.prepend_newline();
            let suggestion_range = if let Some(symbol) = kind.symbol() {
                let outline = buffer
                    .update(&mut cx, |buffer, _| buffer.snapshot().outline(None))?
                    .context("no outline for buffer")?;
                let candidate = outline
                    .path_candidates
                    .iter()
                    .find(|item| item.string == symbol)
                    .context("symbol not found")?;
                buffer.update(&mut cx, |buffer, _| {
                    let outline_item = &outline.items[candidate.id];
                    let symbol_range = outline_item.range.to_point(buffer);
                    let body_range = outline_item
                        .body_range
                        .as_ref()
                        .map(|range| range.to_point(buffer))
                        .unwrap_or(symbol_range.clone());

                    match kind {
                        EditOperationKind::PrependChild { .. } => {
                            let position = buffer.anchor_after(body_range.start);
                            position..position
                        }
                        EditOperationKind::AppendChild { .. } => {
                            let position = buffer.anchor_before(body_range.end);
                            position..position
                        }
                        EditOperationKind::InsertSiblingBefore { .. } => {
                            let position = buffer.anchor_before(symbol_range.start);
                            position..position
                        }
                        EditOperationKind::InsertSiblingAfter { .. } => {
                            let position = buffer.anchor_after(symbol_range.end);
                            position..position
                        }
                        EditOperationKind::Update { .. } | EditOperationKind::Delete { .. } => {
                            let start = Point::new(symbol_range.start.row, 0);
                            let end = Point::new(
                                symbol_range.end.row,
                                buffer.line_len(symbol_range.end.row),
                            );
                            buffer.anchor_before(start)..buffer.anchor_after(end)
                        }
                        EditOperationKind::Create { .. } => unreachable!(),
                    }
                })?
            } else {
                match kind {
                    EditOperationKind::PrependChild { .. } => {
                        language::Anchor::MIN..language::Anchor::MIN
                    }
                    EditOperationKind::AppendChild { .. } | EditOperationKind::Create { .. } => {
                        language::Anchor::MAX..language::Anchor::MAX
                    }
                    _ => unreachable!("All other operations should have a symbol"),
                }
            };

            Ok((
                buffer,
                EditSuggestion {
                    range: suggestion_range,
                    description: kind.description().map(ToString::to_string),
                    prepend_newline,
                },
            ))
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditOperationKind {
    Update {
        symbol: String,
        description: String,
    },
    Create {
        description: String,
    },
    InsertSiblingBefore {
        symbol: String,
        description: String,
    },
    InsertSiblingAfter {
        symbol: String,
        description: String,
    },
    PrependChild {
        symbol: Option<String>,
        description: String,
    },
    AppendChild {
        symbol: Option<String>,
        description: String,
    },
    Delete {
        symbol: String,
    },
}

impl EditOperationKind {
    pub fn symbol(&self) -> Option<&str> {
        match self {
            Self::Update { symbol, .. } => Some(symbol),
            Self::InsertSiblingBefore { symbol, .. } => Some(symbol),
            Self::InsertSiblingAfter { symbol, .. } => Some(symbol),
            Self::PrependChild { symbol, .. } => symbol.as_deref(),
            Self::AppendChild { symbol, .. } => symbol.as_deref(),
            Self::Delete { symbol } => Some(symbol),
            Self::Create { .. } => None,
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Update { description, .. } => Some(description),
            Self::Create { description } => Some(description),
            Self::InsertSiblingBefore { description, .. } => Some(description),
            Self::InsertSiblingAfter { description, .. } => Some(description),
            Self::PrependChild { description, .. } => Some(description),
            Self::AppendChild { description, .. } => Some(description),
            Self::Delete { .. } => None,
        }
    }

    pub fn prepend_newline(&self) -> bool {
        match self {
            Self::PrependChild { .. }
            | Self::AppendChild { .. }
            | Self::InsertSiblingAfter { .. }
            | Self::InsertSiblingBefore { .. } => true,
            _ => false,
        }
    }
}

pub struct Context {
    id: ContextId,
    timestamp: clock::Lamport,
    version: clock::Global,
    pending_ops: Vec<ContextOperation>,
    operations: Vec<ContextOperation>,
    buffer: Model<Buffer>,
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
    pending_save: Task<Result<()>>,
    path: Option<PathBuf>,
    _subscriptions: Vec<Subscription>,
    telemetry: Option<Arc<Telemetry>>,
    language_registry: Arc<LanguageRegistry>,
    edit_steps: Vec<EditStep>,
}

impl EventEmitter<ContextEvent> for Context {}

impl Context {
    pub fn local(
        language_registry: Arc<LanguageRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::new(
            ContextId::new(),
            ReplicaId::default(),
            language::Capability::ReadWrite,
            language_registry,
            telemetry,
            cx,
        )
    }

    pub fn new(
        id: ContextId,
        replica_id: ReplicaId,
        capability: language::Capability,
        language_registry: Arc<LanguageRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let buffer = cx.new_model(|_cx| {
            let mut buffer = Buffer::remote(
                language::BufferId::new(1).unwrap(),
                replica_id,
                capability,
                "",
            );
            buffer.set_language_registry(language_registry.clone());
            buffer
        });
        let edits_since_last_slash_command_parse =
            buffer.update(cx, |buffer, _| buffer.subscribe());
        let mut this = Self {
            id,
            timestamp: clock::Lamport::new(replica_id),
            version: clock::Global::new(),
            pending_ops: Vec::new(),
            operations: Vec::new(),
            message_anchors: Default::default(),
            messages_metadata: Default::default(),
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
            _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
            pending_save: Task::ready(Ok(())),
            path: None,
            buffer,
            telemetry,
            language_registry,
            edit_steps: Vec::new(),
        };

        let first_message_id = MessageId(clock::Lamport {
            replica_id: 0,
            value: 0,
        });
        let message = MessageAnchor {
            id: first_message_id,
            start: language::Anchor::MIN,
        };
        this.messages_metadata.insert(
            first_message_id,
            MessageMetadata {
                role: Role::User,
                status: MessageStatus::Done,
                timestamp: first_message_id.0,
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
            id: Some(self.id.clone()),
            zed: "context".into(),
            version: SavedContext::VERSION.into(),
            text: buffer.text(),
            messages: self
                .messages(cx)
                .map(|message| SavedMessage {
                    id: message.id,
                    start: message.offset_range.start,
                    metadata: self.messages_metadata[&message.id].clone(),
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
    pub fn deserialize(
        saved_context: SavedContext,
        path: PathBuf,
        language_registry: Arc<LanguageRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let id = saved_context.id.clone().unwrap_or_else(|| ContextId::new());
        let mut this = Self::new(
            id,
            ReplicaId::default(),
            language::Capability::ReadWrite,
            language_registry,
            telemetry,
            cx,
        );
        this.path = Some(path);
        this.buffer.update(cx, |buffer, cx| {
            buffer.set_text(saved_context.text.as_str(), cx)
        });
        let operations = saved_context.into_ops(&this.buffer, cx);
        this.apply_ops(operations, cx).unwrap();
        this
    }

    pub fn id(&self) -> &ContextId {
        &self.id
    }

    pub fn replica_id(&self) -> ReplicaId {
        self.timestamp.replica_id
    }

    pub fn version(&self, cx: &AppContext) -> ContextVersion {
        ContextVersion {
            context: self.version.clone(),
            buffer: self.buffer.read(cx).version(),
        }
    }

    pub fn set_capability(
        &mut self,
        capability: language::Capability,
        cx: &mut ModelContext<Self>,
    ) {
        self.buffer
            .update(cx, |buffer, cx| buffer.set_capability(capability, cx));
    }

    fn next_timestamp(&mut self) -> clock::Lamport {
        let timestamp = self.timestamp.tick();
        self.version.observe(timestamp);
        timestamp
    }

    pub fn serialize_ops(
        &self,
        since: &ContextVersion,
        cx: &AppContext,
    ) -> Task<Vec<proto::ContextOperation>> {
        let buffer_ops = self
            .buffer
            .read(cx)
            .serialize_ops(Some(since.buffer.clone()), cx);

        let mut context_ops = self
            .operations
            .iter()
            .filter(|op| !since.context.observed(op.timestamp()))
            .cloned()
            .collect::<Vec<_>>();
        context_ops.extend(self.pending_ops.iter().cloned());

        cx.background_executor().spawn(async move {
            let buffer_ops = buffer_ops.await;
            context_ops.sort_unstable_by_key(|op| op.timestamp());
            buffer_ops
                .into_iter()
                .map(|op| proto::ContextOperation {
                    variant: Some(proto::context_operation::Variant::BufferOperation(
                        proto::context_operation::BufferOperation {
                            operation: Some(op),
                        },
                    )),
                })
                .chain(context_ops.into_iter().map(|op| op.to_proto()))
                .collect()
        })
    }

    pub fn apply_ops(
        &mut self,
        ops: impl IntoIterator<Item = ContextOperation>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
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
        let mut messages_changed = false;
        let mut summary_changed = false;

        self.pending_ops.sort_unstable_by_key(|op| op.timestamp());
        for op in mem::take(&mut self.pending_ops) {
            if !self.can_apply_op(&op, cx) {
                self.pending_ops.push(op);
                continue;
            }

            let timestamp = op.timestamp();
            match op.clone() {
                ContextOperation::InsertMessage {
                    anchor, metadata, ..
                } => {
                    if self.messages_metadata.contains_key(&anchor.id) {
                        // We already applied this operation.
                    } else {
                        self.insert_message(anchor, metadata, cx);
                        messages_changed = true;
                    }
                }
                ContextOperation::UpdateMessage {
                    message_id,
                    metadata: new_metadata,
                    ..
                } => {
                    let metadata = self.messages_metadata.get_mut(&message_id).unwrap();
                    if new_metadata.timestamp > metadata.timestamp {
                        *metadata = new_metadata;
                        messages_changed = true;
                    }
                }
                ContextOperation::UpdateSummary {
                    summary: new_summary,
                    ..
                } => {
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
                    ..
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

            self.version.observe(timestamp);
            self.timestamp.observe(timestamp);
            self.operations.push(op);
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

    fn can_apply_op(&self, op: &ContextOperation, cx: &AppContext) -> bool {
        if !self.version.observed_all(op.version()) {
            return false;
        }

        match op {
            ContextOperation::InsertMessage { anchor, .. } => self
                .buffer
                .read(cx)
                .version
                .observed(anchor.start.timestamp),
            ContextOperation::UpdateMessage { message_id, .. } => {
                self.messages_metadata.contains_key(message_id)
            }
            ContextOperation::UpdateSummary { .. } => true,
            ContextOperation::SlashCommandFinished {
                output_range,
                sections,
                ..
            } => {
                let version = &self.buffer.read(cx).version;
                sections
                    .iter()
                    .map(|section| &section.range)
                    .chain([output_range])
                    .all(|range| {
                        let observed_start = range.start == language::Anchor::MIN
                            || range.start == language::Anchor::MAX
                            || version.observed(range.start.timestamp);
                        let observed_end = range.end == language::Anchor::MIN
                            || range.end == language::Anchor::MAX
                            || version.observed(range.end.timestamp);
                        observed_start && observed_end
                    })
            }
            ContextOperation::BufferOperation(_) => {
                panic!("buffer operations should always be applied")
            }
        }
    }

    fn push_op(&mut self, op: ContextOperation, cx: &mut ModelContext<Self>) {
        self.operations.push(op.clone());
        cx.emit(ContextEvent::Operation(op));
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

    pub fn edit_steps(&self) -> &[EditStep] {
        &self.edit_steps
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
                self.reparse_slash_commands(cx);
                self.prune_invalid_edit_steps(cx);
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

    fn prune_invalid_edit_steps(&mut self, cx: &mut ModelContext<Self>) {
        let buffer = self.buffer.read(cx);
        let prev_len = self.edit_steps.len();
        self.edit_steps.retain(|step| {
            step.source_range.start.is_valid(buffer) && step.source_range.end.is_valid(buffer)
        });
        if self.edit_steps.len() != prev_len {
            cx.emit(ContextEvent::EditStepsChanged);
            cx.notify();
        }
    }

    fn parse_edit_steps_in_range(&mut self, range: Range<usize>, cx: &mut ModelContext<Self>) {
        let mut new_edit_steps = Vec::new();

        self.buffer.update(cx, |buffer, _cx| {
            let mut message_lines = buffer.as_rope().chunks_in_range(range).lines();
            let mut in_step = false;
            let mut step_start = 0;
            let mut line_start_offset = message_lines.offset();

            while let Some(line) = message_lines.next() {
                if let Some(step_start_index) = line.find("<step>") {
                    if !in_step {
                        in_step = true;
                        step_start = line_start_offset + step_start_index;
                    }
                }

                if let Some(step_end_index) = line.find("</step>") {
                    if in_step {
                        let start_anchor = buffer.anchor_after(step_start);
                        let end_anchor = buffer
                            .anchor_before(line_start_offset + step_end_index + "</step>".len());
                        let source_range = start_anchor..end_anchor;

                        // Check if a step with the same range already exists
                        let existing_step_index = self.edit_steps.binary_search_by(|probe| {
                            probe.source_range.cmp(&source_range, buffer)
                        });

                        if let Err(ix) = existing_step_index {
                            // Step doesn't exist, so add it
                            new_edit_steps.push((
                                ix,
                                EditStep {
                                    source_range,
                                    operations: None,
                                },
                            ));
                        }

                        in_step = false;
                    }
                }

                line_start_offset = message_lines.offset();
            }
        });

        // Insert new steps and generate their corresponding tasks
        for (index, mut step) in new_edit_steps.into_iter().rev() {
            let task = self.generate_edit_step_operations(&step, cx);
            step.operations = Some(EditStepOperations::Pending(task));
            self.edit_steps.insert(index, step);
        }

        cx.emit(ContextEvent::EditStepsChanged);
        cx.notify();
    }

    fn generate_edit_step_operations(
        &self,
        edit_step: &EditStep,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let mut request = self.to_completion_request(cx);
        let edit_step_range = edit_step.source_range.clone();
        let step_text = self
            .buffer
            .read(cx)
            .text_for_range(edit_step_range.clone())
            .collect::<String>();

        cx.spawn(|this, mut cx| async move {
            let prompt_store = cx.update(|cx| PromptStore::global(cx))?.await?;

            let mut prompt = prompt_store.operations_prompt();
            prompt.push_str(&step_text);

            request.messages.push(LanguageModelRequestMessage {
                role: Role::User,
                content: prompt,
            });

            let raw_output = cx
                .update(|cx| CompletionProvider::global(cx).complete(request, cx))?
                .await?;

            let operations = Self::parse_edit_operations(&raw_output);
            this.update(&mut cx, |this, cx| {
                let step_index = this
                    .edit_steps
                    .binary_search_by(|step| {
                        step.source_range
                            .cmp(&edit_step_range, this.buffer.read(cx))
                    })
                    .map_err(|_| anyhow!("edit step not found"))?;
                if let Some(edit_step) = this.edit_steps.get_mut(step_index) {
                    edit_step.operations = Some(EditStepOperations::Parsed {
                        operations,
                        raw_output,
                    });
                    cx.emit(ContextEvent::EditStepsChanged);
                }
                anyhow::Ok(())
            })?
        })
    }

    fn parse_edit_operations(xml: &str) -> Vec<EditOperation> {
        let Some(start_ix) = xml.find("<operations>") else {
            return Vec::new();
        };
        let Some(end_ix) = xml[start_ix..].find("</operations>") else {
            return Vec::new();
        };
        let end_ix = end_ix + start_ix + "</operations>".len();

        let doc = roxmltree::Document::parse(&xml[start_ix..end_ix]).log_err();
        doc.map_or(Vec::new(), |doc| {
            doc.root_element()
                .children()
                .map(|node| {
                    let tag_name = node.tag_name().name();
                    let path = node
                        .attribute("path")
                        .with_context(|| {
                            format!("invalid node {node:?}, missing attribute 'path'")
                        })?
                        .to_string();
                    let kind = match tag_name {
                        "update" => EditOperationKind::Update {
                            symbol: node
                                .attribute("symbol")
                                .with_context(|| {
                                    format!("invalid node {node:?}, missing attribute 'symbol'")
                                })?
                                .to_string(),
                            description: node
                                .attribute("description")
                                .with_context(|| {
                                    format!(
                                        "invalid node {node:?}, missing attribute 'description'"
                                    )
                                })?
                                .to_string(),
                        },
                        "create" => EditOperationKind::Create {
                            description: node
                                .attribute("description")
                                .with_context(|| {
                                    format!(
                                        "invalid node {node:?}, missing attribute 'description'"
                                    )
                                })?
                                .to_string(),
                        },
                        "insert_sibling_after" => EditOperationKind::InsertSiblingAfter {
                            symbol: node
                                .attribute("symbol")
                                .with_context(|| {
                                    format!("invalid node {node:?}, missing attribute 'symbol'")
                                })?
                                .to_string(),
                            description: node
                                .attribute("description")
                                .with_context(|| {
                                    format!(
                                        "invalid node {node:?}, missing attribute 'description'"
                                    )
                                })?
                                .to_string(),
                        },
                        "insert_sibling_before" => EditOperationKind::InsertSiblingBefore {
                            symbol: node
                                .attribute("symbol")
                                .with_context(|| {
                                    format!("invalid node {node:?}, missing attribute 'symbol'")
                                })?
                                .to_string(),
                            description: node
                                .attribute("description")
                                .with_context(|| {
                                    format!(
                                        "invalid node {node:?}, missing attribute 'description'"
                                    )
                                })?
                                .to_string(),
                        },
                        "prepend_child" => EditOperationKind::PrependChild {
                            symbol: node.attribute("symbol").map(String::from),
                            description: node
                                .attribute("description")
                                .with_context(|| {
                                    format!(
                                        "invalid node {node:?}, missing attribute 'description'"
                                    )
                                })?
                                .to_string(),
                        },
                        "append_child" => EditOperationKind::AppendChild {
                            symbol: node.attribute("symbol").map(String::from),
                            description: node
                                .attribute("description")
                                .with_context(|| {
                                    format!(
                                        "invalid node {node:?}, missing attribute 'description'"
                                    )
                                })?
                                .to_string(),
                        },
                        "delete" => EditOperationKind::Delete {
                            symbol: node
                                .attribute("symbol")
                                .with_context(|| {
                                    format!("invalid node {node:?}, missing attribute 'symbol'")
                                })?
                                .to_string(),
                        },
                        _ => return Err(anyhow!("invalid node {node:?}")),
                    };
                    anyhow::Ok(EditOperation { path, kind })
                })
                .filter_map(|op| op.log_err())
                .collect()
        })
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

                        let version = this.version.clone();
                        let command_id = SlashCommandId(this.next_timestamp());
                        let (operation, event) = this.buffer.update(cx, |buffer, cx| {
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

                            let output_range =
                                buffer.anchor_after(start)..buffer.anchor_before(new_end);
                            this.finished_slash_commands.insert(command_id);

                            (
                                ContextOperation::SlashCommandFinished {
                                    id: command_id,
                                    output_range: output_range.clone(),
                                    sections: sections.clone(),
                                    version,
                                },
                                ContextEvent::SlashCommandFinished {
                                    output_range,
                                    sections,
                                    run_commands_in_output: output.run_commands_in_text,
                                },
                            )
                        });

                        this.push_op(operation, cx);
                        cx.emit(event);
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

    pub fn assist(&mut self, cx: &mut ModelContext<Self>) -> Option<MessageAnchor> {
        let last_message_id = self.message_anchors.iter().rev().find_map(|message| {
            message
                .start
                .is_valid(self.buffer.read(cx))
                .then_some(message.id)
        })?;

        if !CompletionProvider::global(cx).is_authenticated() {
            log::info!("completion provider has no credentials");
            return None;
        }

        let request = self.to_completion_request(cx);
        let stream = CompletionProvider::global(cx).stream_completion(request, cx);
        let assistant_message = self
            .insert_message_after(last_message_id, Role::Assistant, MessageStatus::Pending, cx)
            .unwrap();

        // Queue up the user's next reply.
        let user_message = self
            .insert_message_after(assistant_message.id, Role::User, MessageStatus::Done, cx)
            .unwrap();

        let task = cx.spawn({
            |this, mut cx| async move {
                let assistant_message_id = assistant_message.id;
                let mut response_latency = None;
                let stream_completion = async {
                    let request_start = Instant::now();
                    let mut chunks = stream.await?;

                    while let Some(chunk) = chunks.next().await {
                        if response_latency.is_none() {
                            response_latency = Some(request_start.elapsed());
                        }
                        let chunk = chunk?;

                        this.update(&mut cx, |this, cx| {
                            let message_ix = this
                                .message_anchors
                                .iter()
                                .position(|message| message.id == assistant_message_id)?;
                            let message_range = this.buffer.update(cx, |buffer, cx| {
                                let message_start_offset =
                                    this.message_anchors[message_ix].start.to_offset(buffer);
                                let message_old_end_offset = this.message_anchors[message_ix + 1..]
                                    .iter()
                                    .find(|message| message.start.is_valid(buffer))
                                    .map_or(buffer.len(), |message| {
                                        message.start.to_offset(buffer).saturating_sub(1)
                                    });
                                let message_new_end_offset = message_old_end_offset + chunk.len();
                                buffer.edit(
                                    [(message_old_end_offset..message_old_end_offset, chunk)],
                                    None,
                                    cx,
                                );
                                message_start_offset..message_new_end_offset
                            });
                            this.parse_edit_steps_in_range(message_range, cx);
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
                    let error_message = result
                        .err()
                        .map(|error| error.to_string().trim().to_string());

                    this.update_metadata(assistant_message_id, cx, |metadata| {
                        if let Some(error_message) = error_message.as_ref() {
                            metadata.status =
                                MessageStatus::Error(SharedString::from(error_message.clone()));
                        } else {
                            metadata.status = MessageStatus::Done;
                        }
                    });

                    if let Some(telemetry) = this.telemetry.as_ref() {
                        let model = CompletionProvider::global(cx).model();
                        telemetry.report_assistant_event(
                            Some(this.id.0.clone()),
                            AssistantKind::Panel,
                            model.telemetry_id(),
                            response_latency,
                            error_message,
                        );
                    }
                })
                .ok();
            }
        });

        self.pending_completions.push(PendingCompletion {
            id: post_inc(&mut self.completion_count),
            _task: task,
        });

        Some(user_message)
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
                self.update_metadata(id, cx, |metadata| metadata.role = role);
            }
        }
    }

    pub fn update_metadata(
        &mut self,
        id: MessageId,
        cx: &mut ModelContext<Self>,
        f: impl FnOnce(&mut MessageMetadata),
    ) {
        let version = self.version.clone();
        let timestamp = self.next_timestamp();
        if let Some(metadata) = self.messages_metadata.get_mut(&id) {
            f(metadata);
            metadata.timestamp = timestamp;
            let operation = ContextOperation::UpdateMessage {
                message_id: id,
                metadata: metadata.clone(),
                version,
            };
            self.push_op(operation, cx);
            cx.emit(ContextEvent::MessagesEdited);
            cx.notify();
        }
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
                    .map_or(buffer.len(), |message| {
                        buffer.clip_offset(message.start.to_offset(buffer) - 1, Bias::Left)
                    });
                buffer.edit([(offset..offset, "\n")], None, cx);
                buffer.anchor_before(offset + 1)
            });

            let version = self.version.clone();
            let anchor = MessageAnchor {
                id: MessageId(self.next_timestamp()),
                start,
            };
            let metadata = MessageMetadata {
                role,
                status,
                timestamp: anchor.id.0,
            };
            self.insert_message(anchor.clone(), metadata.clone(), cx);
            self.push_op(
                ContextOperation::InsertMessage {
                    anchor: anchor.clone(),
                    metadata,
                    version,
                },
                cx,
            );
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

            let version = self.version.clone();
            let suffix = if let Some(suffix_start) = suffix_start {
                MessageAnchor {
                    id: MessageId(self.next_timestamp()),
                    start: self.buffer.read(cx).anchor_before(suffix_start),
                }
            } else {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.edit([(range.end..range.end, "\n")], None, cx);
                });
                edited_buffer = true;
                MessageAnchor {
                    id: MessageId(self.next_timestamp()),
                    start: self.buffer.read(cx).anchor_before(range.end + 1),
                }
            };

            let suffix_metadata = MessageMetadata {
                role,
                status: MessageStatus::Done,
                timestamp: suffix.id.0,
            };
            self.insert_message(suffix.clone(), suffix_metadata.clone(), cx);
            self.push_op(
                ContextOperation::InsertMessage {
                    anchor: suffix.clone(),
                    metadata: suffix_metadata,
                    version,
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

                    let version = self.version.clone();
                    let selection = if let Some(prefix_end) = prefix_end {
                        MessageAnchor {
                            id: MessageId(self.next_timestamp()),
                            start: self.buffer.read(cx).anchor_before(prefix_end),
                        }
                    } else {
                        self.buffer.update(cx, |buffer, cx| {
                            buffer.edit([(range.start..range.start, "\n")], None, cx)
                        });
                        edited_buffer = true;
                        MessageAnchor {
                            id: MessageId(self.next_timestamp()),
                            start: self.buffer.read(cx).anchor_before(range.end + 1),
                        }
                    };

                    let selection_metadata = MessageMetadata {
                        role,
                        status: MessageStatus::Done,
                        timestamp: selection.id.0,
                    };
                    self.insert_message(selection.clone(), selection_metadata.clone(), cx);
                    self.push_op(
                        ContextOperation::InsertMessage {
                            anchor: selection.clone(),
                            metadata: selection_metadata,
                            version,
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
        new_anchor: MessageAnchor,
        new_metadata: MessageMetadata,
        cx: &mut ModelContext<Self>,
    ) {
        cx.emit(ContextEvent::MessagesEdited);

        self.messages_metadata.insert(new_anchor.id, new_metadata);

        let buffer = self.buffer.read(cx);
        let insertion_ix = self
            .message_anchors
            .iter()
            .position(|anchor| {
                let comparison = new_anchor.start.cmp(&anchor.start, buffer);
                comparison.is_lt() || (comparison.is_eq() && new_anchor.id > anchor.id)
            })
            .unwrap_or(self.message_anchors.len());
        self.message_anchors.insert(insertion_ix, new_anchor);
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

            let stream = CompletionProvider::global(cx).stream_completion(request, cx);
            self.pending_summary = cx.spawn(|this, mut cx| {
                async move {
                    let mut messages = stream.await?;

                    while let Some(message) = messages.next().await {
                        let text = message?;
                        let mut lines = text.lines();
                        this.update(&mut cx, |this, cx| {
                            let version = this.version.clone();
                            let timestamp = this.next_timestamp();
                            let summary = this.summary.get_or_insert(Default::default());
                            summary.text.extend(lines.next());
                            summary.timestamp = timestamp;
                            let operation = ContextOperation::UpdateSummary {
                                summary: summary.clone(),
                                version,
                            };
                            this.push_op(operation, cx);
                            cx.emit(ContextEvent::SummaryChanged);
                        })?;

                        // Stop if the LLM generated multiple lines.
                        if lines.next().is_some() {
                            break;
                        }
                    }

                    this.update(&mut cx, |this, cx| {
                        let version = this.version.clone();
                        let timestamp = this.next_timestamp();
                        if let Some(summary) = this.summary.as_mut() {
                            summary.done = true;
                            summary.timestamp = timestamp;
                            let operation = ContextOperation::UpdateSummary {
                                summary: summary.clone(),
                                version,
                            };
                            this.push_op(operation, cx);
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
        if self.replica_id() != ReplicaId::default() {
            // Prevent saving a remote context for now.
            return;
        }

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

#[derive(Debug, Default)]
pub struct ContextVersion {
    context: clock::Global,
    buffer: clock::Global,
}

impl ContextVersion {
    pub fn from_proto(proto: &proto::ContextVersion) -> Self {
        Self {
            context: language::proto::deserialize_version(&proto.context_version),
            buffer: language::proto::deserialize_version(&proto.buffer_version),
        }
    }

    pub fn to_proto(&self, context_id: ContextId) -> proto::ContextVersion {
        proto::ContextVersion {
            context_id: context_id.to_proto(),
            context_version: language::proto::serialize_version(&self.context),
            buffer_version: language::proto::serialize_version(&self.buffer),
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
pub struct SavedMessage {
    pub id: MessageId,
    pub start: usize,
    pub metadata: MessageMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct SavedContext {
    pub id: Option<ContextId>,
    pub zed: String,
    pub version: String,
    pub text: String,
    pub messages: Vec<SavedMessage>,
    pub summary: String,
    pub slash_command_output_sections:
        Vec<assistant_slash_command::SlashCommandOutputSection<usize>>,
}

impl SavedContext {
    pub const VERSION: &'static str = "0.4.0";

    pub fn from_json(json: &str) -> Result<Self> {
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

    fn into_ops(
        self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Context>,
    ) -> Vec<ContextOperation> {
        let mut operations = Vec::new();
        let mut version = clock::Global::new();
        let mut next_timestamp = clock::Lamport::new(ReplicaId::default());

        let mut first_message_metadata = None;
        for message in self.messages {
            if message.id == MessageId(clock::Lamport::default()) {
                first_message_metadata = Some(message.metadata);
            } else {
                operations.push(ContextOperation::InsertMessage {
                    anchor: MessageAnchor {
                        id: message.id,
                        start: buffer.read(cx).anchor_before(message.start),
                    },
                    metadata: MessageMetadata {
                        role: message.metadata.role,
                        status: message.metadata.status,
                        timestamp: message.metadata.timestamp,
                    },
                    version: version.clone(),
                });
                version.observe(message.id.0);
                next_timestamp.observe(message.id.0);
            }
        }

        if let Some(metadata) = first_message_metadata {
            let timestamp = next_timestamp.tick();
            operations.push(ContextOperation::UpdateMessage {
                message_id: MessageId(clock::Lamport::default()),
                metadata: MessageMetadata {
                    role: metadata.role,
                    status: metadata.status,
                    timestamp,
                },
                version: version.clone(),
            });
            version.observe(timestamp);
        }

        let timestamp = next_timestamp.tick();
        operations.push(ContextOperation::SlashCommandFinished {
            id: SlashCommandId(timestamp),
            output_range: language::Anchor::MIN..language::Anchor::MAX,
            sections: self
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
            version: version.clone(),
        });
        version.observe(timestamp);

        let timestamp = next_timestamp.tick();
        operations.push(ContextOperation::UpdateSummary {
            summary: ContextSummary {
                text: self.summary,
                done: true,
                timestamp,
            },
            version: version.clone(),
        });
        version.observe(timestamp);

        operations
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct SavedMessageIdPreV0_4_0(usize);

#[derive(Serialize, Deserialize)]
struct SavedMessagePreV0_4_0 {
    id: SavedMessageIdPreV0_4_0,
    start: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct SavedMessageMetadataPreV0_4_0 {
    role: Role,
    status: MessageStatus,
}

#[derive(Serialize, Deserialize)]
struct SavedContextV0_3_0 {
    id: Option<ContextId>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessagePreV0_4_0>,
    message_metadata: HashMap<SavedMessageIdPreV0_4_0, SavedMessageMetadataPreV0_4_0>,
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
                .filter_map(|message| {
                    let metadata = self.message_metadata.get(&message.id)?;
                    let timestamp = clock::Lamport {
                        replica_id: ReplicaId::default(),
                        value: message.id.0 as u32,
                    };
                    Some(SavedMessage {
                        id: MessageId(timestamp),
                        start: message.start,
                        metadata: MessageMetadata {
                            role: metadata.role,
                            status: metadata.status.clone(),
                            timestamp,
                        },
                    })
                })
                .collect(),
            summary: self.summary,
            slash_command_output_sections: self.slash_command_output_sections,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SavedContextV0_2_0 {
    id: Option<ContextId>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessagePreV0_4_0>,
    message_metadata: HashMap<SavedMessageIdPreV0_4_0, SavedMessageMetadataPreV0_4_0>,
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
    id: Option<ContextId>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessagePreV0_4_0>,
    message_metadata: HashMap<SavedMessageIdPreV0_4_0, SavedMessageMetadataPreV0_4_0>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assistant_panel, prompt_library,
        slash_command::{active_command, file_command},
        MessageId,
    };
    use assistant_slash_command::{ArgumentCompletion, SlashCommand};
    use completion::FakeCompletionProvider;
    use fs::FakeFs;
    use gpui::{AppContext, TestAppContext, WeakView};
    use indoc::indoc;
    use language::LspAdapterDelegate;
    use parking_lot::Mutex;
    use project::Project;
    use rand::prelude::*;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{cell::RefCell, env, rc::Rc, sync::atomic::AtomicBool};
    use text::{network::Network, ToPoint};
    use ui::WindowContext;
    use unindent::Unindent;
    use util::{test::marked_text_ranges, RandomCharIter};
    use workspace::Workspace;

    #[gpui::test]
    fn test_inserting_and_removing_messages(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        FakeCompletionProvider::setup_test(cx);
        cx.set_global(settings_store);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

        let context = cx.new_model(|cx| Context::local(registry, None, cx));
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
        FakeCompletionProvider::setup_test(cx);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

        let context = cx.new_model(|cx| Context::local(registry, None, cx));
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
        FakeCompletionProvider::setup_test(cx);
        cx.set_global(settings_store);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let context = cx.new_model(|cx| Context::local(registry, None, cx));
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
        cx.update(FakeCompletionProvider::setup_test);
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
        let context = cx.new_model(|cx| Context::local(registry.clone(), None, cx));

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

    #[gpui::test]
    async fn test_edit_step_parsing(cx: &mut TestAppContext) {
        cx.update(prompt_library::init);
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        let fake_provider = cx.update(FakeCompletionProvider::setup_test);
        cx.update(assistant_panel::init);
        let registry = Arc::new(LanguageRegistry::test(cx.executor()));

        // Create a new context
        let context = cx.new_model(|cx| Context::local(registry.clone(), None, cx));
        let buffer = context.read_with(cx, |context, _| context.buffer.clone());

        // Simulate user input
        let user_message = indoc! {r#"
            Please refactor this code:

            fn main() {
                println!("Hello, World!");
            }
        "#};
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, user_message)], None, cx);
        });

        // Simulate LLM response with edit steps
        let llm_response = indoc! {r#"
            Sure, I can help you refactor that code. Here's a step-by-step process:

            <step>
            First, let's extract the greeting into a separate function:

            ```rust
            fn greet() {
                println!("Hello, World!");
            }

            fn main() {
                greet();
            }
            ```
            </step>

            <step>
            Now, let's make the greeting customizable:

            ```rust
            fn greet(name: &str) {
                println!("Hello, {}!", name);
            }

            fn main() {
                greet("World");
            }
            ```
            </step>

            These changes make the code more modular and flexible.
        "#};

        // Simulate the assist method to trigger the LLM response
        context.update(cx, |context, cx| context.assist(cx));
        cx.run_until_parked();

        // Retrieve the assistant response message's start from the context
        let response_start_row = context.read_with(cx, |context, cx| {
            let buffer = context.buffer.read(cx);
            context.message_anchors[1].start.to_point(buffer).row
        });

        // Simulate the LLM completion
        fake_provider.send_last_completion_chunk(llm_response.to_string());
        fake_provider.finish_last_completion();

        // Wait for the completion to be processed
        cx.run_until_parked();

        // Verify that the edit steps were parsed correctly
        context.read_with(cx, |context, cx| {
            assert_eq!(
                edit_steps(context, cx),
                vec![
                    Point::new(response_start_row + 2, 0)..Point::new(response_start_row + 14, 7),
                    Point::new(response_start_row + 16, 0)..Point::new(response_start_row + 28, 7),
                ]
            );
        });

        fn edit_steps(context: &Context, cx: &AppContext) -> Vec<Range<Point>> {
            context
                .edit_steps
                .iter()
                .map(|step| {
                    let buffer = context.buffer.read(cx);
                    step.source_range.to_point(buffer)
                })
                .collect()
        }
    }

    #[test]
    fn test_parse_edit_operations() {
        let operations = indoc! {r#"
            Here are the operations to make all fields of the Canvas struct private:

            <operations>
                <update path="font-kit/src/canvas.rs" symbol="pub struct Canvas pub pixels" description="Remove pub keyword from pixels field" />
                <update path="font-kit/src/canvas.rs" symbol="pub struct Canvas pub size" description="Remove pub keyword from size field" />
                <update path="font-kit/src/canvas.rs" symbol="pub struct Canvas pub stride" description="Remove pub keyword from stride field" />
                <update path="font-kit/src/canvas.rs" symbol="pub struct Canvas pub format" description="Remove pub keyword from format field" />
            </operations>
        "#};

        let parsed_operations = Context::parse_edit_operations(operations);
        assert_eq!(
            parsed_operations,
            vec![
                EditOperation {
                    path: "font-kit/src/canvas.rs".to_string(),
                    kind: EditOperationKind::Update {
                        symbol: "pub struct Canvas pub pixels".to_string(),
                        description: "Remove pub keyword from pixels field".to_string(),
                    },
                },
                EditOperation {
                    path: "font-kit/src/canvas.rs".to_string(),
                    kind: EditOperationKind::Update {
                        symbol: "pub struct Canvas pub size".to_string(),
                        description: "Remove pub keyword from size field".to_string(),
                    },
                },
                EditOperation {
                    path: "font-kit/src/canvas.rs".to_string(),
                    kind: EditOperationKind::Update {
                        symbol: "pub struct Canvas pub stride".to_string(),
                        description: "Remove pub keyword from stride field".to_string(),
                    },
                },
                EditOperation {
                    path: "font-kit/src/canvas.rs".to_string(),
                    kind: EditOperationKind::Update {
                        symbol: "pub struct Canvas pub format".to_string(),
                        description: "Remove pub keyword from format field".to_string(),
                    },
                },
            ]
        );
    }

    #[gpui::test]
    async fn test_serialization(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.update(FakeCompletionProvider::setup_test);
        cx.update(assistant_panel::init);
        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let context = cx.new_model(|cx| Context::local(registry.clone(), None, cx));
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

        let serialized_context = context.read_with(cx, |context, cx| context.serialize(cx));
        let deserialized_context = cx.new_model(|cx| {
            Context::deserialize(
                serialized_context,
                Default::default(),
                registry.clone(),
                None,
                cx,
            )
        });
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

    #[gpui::test(iterations = 100)]
    async fn test_random_context_collaboration(cx: &mut TestAppContext, mut rng: StdRng) {
        let min_peers = env::var("MIN_PEERS")
            .map(|i| i.parse().expect("invalid `MIN_PEERS` variable"))
            .unwrap_or(2);
        let max_peers = env::var("MAX_PEERS")
            .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
            .unwrap_or(5);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(50);

        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.update(FakeCompletionProvider::setup_test);
        cx.update(assistant_panel::init);
        let slash_commands = cx.update(SlashCommandRegistry::default_global);
        slash_commands.register_command(FakeSlashCommand("cmd-1".into()), false);
        slash_commands.register_command(FakeSlashCommand("cmd-2".into()), false);
        slash_commands.register_command(FakeSlashCommand("cmd-3".into()), false);

        let registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
        let network = Arc::new(Mutex::new(Network::new(rng.clone())));
        let mut contexts = Vec::new();

        let num_peers = rng.gen_range(min_peers..=max_peers);
        let context_id = ContextId::new();
        for i in 0..num_peers {
            let context = cx.new_model(|cx| {
                Context::new(
                    context_id.clone(),
                    i as ReplicaId,
                    language::Capability::ReadWrite,
                    registry.clone(),
                    None,
                    cx,
                )
            });

            cx.update(|cx| {
                cx.subscribe(&context, {
                    let network = network.clone();
                    move |_, event, _| {
                        if let ContextEvent::Operation(op) = event {
                            network
                                .lock()
                                .broadcast(i as ReplicaId, vec![op.to_proto()]);
                        }
                    }
                })
                .detach();
            });

            contexts.push(context);
            network.lock().add_peer(i as ReplicaId);
        }

        let mut mutation_count = operations;

        while mutation_count > 0
            || !network.lock().is_idle()
            || network.lock().contains_disconnected_peers()
        {
            let context_index = rng.gen_range(0..contexts.len());
            let context = &contexts[context_index];

            match rng.gen_range(0..100) {
                0..=29 if mutation_count > 0 => {
                    log::info!("Context {}: edit buffer", context_index);
                    context.update(cx, |context, cx| {
                        context
                            .buffer
                            .update(cx, |buffer, cx| buffer.randomly_edit(&mut rng, 1, cx));
                    });
                    mutation_count -= 1;
                }
                30..=44 if mutation_count > 0 => {
                    context.update(cx, |context, cx| {
                        let range = context.buffer.read(cx).random_byte_range(0, &mut rng);
                        log::info!("Context {}: split message at {:?}", context_index, range);
                        context.split_message(range, cx);
                    });
                    mutation_count -= 1;
                }
                45..=59 if mutation_count > 0 => {
                    context.update(cx, |context, cx| {
                        if let Some(message) = context.messages(cx).choose(&mut rng) {
                            let role = *[Role::User, Role::Assistant, Role::System]
                                .choose(&mut rng)
                                .unwrap();
                            log::info!(
                                "Context {}: insert message after {:?} with {:?}",
                                context_index,
                                message.id,
                                role
                            );
                            context.insert_message_after(message.id, role, MessageStatus::Done, cx);
                        }
                    });
                    mutation_count -= 1;
                }
                60..=74 if mutation_count > 0 => {
                    context.update(cx, |context, cx| {
                        let command_text = "/".to_string()
                            + slash_commands
                                .command_names()
                                .choose(&mut rng)
                                .unwrap()
                                .clone()
                                .as_ref();

                        let command_range = context.buffer.update(cx, |buffer, cx| {
                            let offset = buffer.random_byte_range(0, &mut rng).start;
                            buffer.edit(
                                [(offset..offset, format!("\n{}\n", command_text))],
                                None,
                                cx,
                            );
                            offset + 1..offset + 1 + command_text.len()
                        });

                        let output_len = rng.gen_range(1..=10);
                        let output_text = RandomCharIter::new(&mut rng)
                            .filter(|c| *c != '\r')
                            .take(output_len)
                            .collect::<String>();

                        let num_sections = rng.gen_range(0..=3);
                        let mut sections = Vec::with_capacity(num_sections);
                        for _ in 0..num_sections {
                            let section_start = rng.gen_range(0..output_len);
                            let section_end = rng.gen_range(section_start..=output_len);
                            sections.push(SlashCommandOutputSection {
                                range: section_start..section_end,
                                icon: ui::IconName::Ai,
                                label: "section".into(),
                            });
                        }

                        log::info!(
                            "Context {}: insert slash command output at {:?} with {:?}",
                            context_index,
                            command_range,
                            sections
                        );

                        let command_range =
                            context.buffer.read(cx).anchor_after(command_range.start)
                                ..context.buffer.read(cx).anchor_after(command_range.end);
                        context.insert_command_output(
                            command_range,
                            Task::ready(Ok(SlashCommandOutput {
                                text: output_text,
                                sections,
                                run_commands_in_text: false,
                            })),
                            true,
                            cx,
                        );
                    });
                    cx.run_until_parked();
                    mutation_count -= 1;
                }
                75..=84 if mutation_count > 0 => {
                    context.update(cx, |context, cx| {
                        if let Some(message) = context.messages(cx).choose(&mut rng) {
                            let new_status = match rng.gen_range(0..3) {
                                0 => MessageStatus::Done,
                                1 => MessageStatus::Pending,
                                _ => MessageStatus::Error(SharedString::from("Random error")),
                            };
                            log::info!(
                                "Context {}: update message {:?} status to {:?}",
                                context_index,
                                message.id,
                                new_status
                            );
                            context.update_metadata(message.id, cx, |metadata| {
                                metadata.status = new_status;
                            });
                        }
                    });
                    mutation_count -= 1;
                }
                _ => {
                    let replica_id = context_index as ReplicaId;
                    if network.lock().is_disconnected(replica_id) {
                        network.lock().reconnect_peer(replica_id, 0);

                        let (ops_to_send, ops_to_receive) = cx.read(|cx| {
                            let host_context = &contexts[0].read(cx);
                            let guest_context = context.read(cx);
                            (
                                guest_context.serialize_ops(&host_context.version(cx), cx),
                                host_context.serialize_ops(&guest_context.version(cx), cx),
                            )
                        });
                        let ops_to_send = ops_to_send.await;
                        let ops_to_receive = ops_to_receive
                            .await
                            .into_iter()
                            .map(ContextOperation::from_proto)
                            .collect::<Result<Vec<_>>>()
                            .unwrap();
                        log::info!(
                            "Context {}: reconnecting. Sent {} operations, received {} operations",
                            context_index,
                            ops_to_send.len(),
                            ops_to_receive.len()
                        );

                        network.lock().broadcast(replica_id, ops_to_send);
                        context
                            .update(cx, |context, cx| context.apply_ops(ops_to_receive, cx))
                            .unwrap();
                    } else if rng.gen_bool(0.1) && replica_id != 0 {
                        log::info!("Context {}: disconnecting", context_index);
                        network.lock().disconnect_peer(replica_id);
                    } else if network.lock().has_unreceived(replica_id) {
                        log::info!("Context {}: applying operations", context_index);
                        let ops = network.lock().receive(replica_id);
                        let ops = ops
                            .into_iter()
                            .map(ContextOperation::from_proto)
                            .collect::<Result<Vec<_>>>()
                            .unwrap();
                        context
                            .update(cx, |context, cx| context.apply_ops(ops, cx))
                            .unwrap();
                    }
                }
            }
        }

        cx.read(|cx| {
            let first_context = contexts[0].read(cx);
            for context in &contexts[1..] {
                let context = context.read(cx);
                assert!(context.pending_ops.is_empty());
                assert_eq!(
                    context.buffer.read(cx).text(),
                    first_context.buffer.read(cx).text(),
                    "Context {} text != Context 0 text",
                    context.buffer.read(cx).replica_id()
                );
                assert_eq!(
                    context.message_anchors,
                    first_context.message_anchors,
                    "Context {} messages != Context 0 messages",
                    context.buffer.read(cx).replica_id()
                );
                assert_eq!(
                    context.messages_metadata,
                    first_context.messages_metadata,
                    "Context {} message metadata != Context 0 message metadata",
                    context.buffer.read(cx).replica_id()
                );
                assert_eq!(
                    context.slash_command_output_sections,
                    first_context.slash_command_output_sections,
                    "Context {} slash command output sections != Context 0 slash command output sections",
                    context.buffer.read(cx).replica_id()
                );
            }
        });
    }

    fn messages(context: &Model<Context>, cx: &AppContext) -> Vec<(MessageId, Role, Range<usize>)> {
        context
            .read(cx)
            .messages(cx)
            .map(|message| (message.id, message.role, message.offset_range))
            .collect()
    }

    #[derive(Clone)]
    struct FakeSlashCommand(String);

    impl SlashCommand for FakeSlashCommand {
        fn name(&self) -> String {
            self.0.clone()
        }

        fn description(&self) -> String {
            format!("Fake slash command: {}", self.0)
        }

        fn menu_text(&self) -> String {
            format!("Run fake command: {}", self.0)
        }

        fn complete_argument(
            self: Arc<Self>,
            _query: String,
            _cancel: Arc<AtomicBool>,
            _workspace: Option<WeakView<Workspace>>,
            _cx: &mut AppContext,
        ) -> Task<Result<Vec<ArgumentCompletion>>> {
            Task::ready(Ok(vec![]))
        }

        fn requires_argument(&self) -> bool {
            false
        }

        fn run(
            self: Arc<Self>,
            _argument: Option<&str>,
            _workspace: WeakView<Workspace>,
            _delegate: Arc<dyn LspAdapterDelegate>,
            _cx: &mut WindowContext,
        ) -> Task<Result<SlashCommandOutput>> {
            Task::ready(Ok(SlashCommandOutput {
                text: format!("Executed fake command: {}", self.0),
                sections: vec![],
                run_commands_in_text: false,
            }))
        }
    }
}
