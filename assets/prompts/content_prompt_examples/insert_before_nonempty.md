Take careful note of the location of the `<rewrite_this>...</rewrite_this>` tags in the document. You'll use them next.

<document language="rust">
use crate::{
    prompts::PromptBuilder, slash_command::SlashCommandLine, AssistantPanel, InitialInsertion,
    InlineAssistId, InlineAssistant, MessageId, MessageStatus,
};
use anyhow::{anyhow, Context as _, Result};
use assistant_slash_command::{
    SlashCommandOutput, SlashCommandOutputSection, SlashCommandRegistry,
};
use client::{self, proto, telemetry::Telemetry};
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use editor::Editor;
use fs::{Fs, RemoveOptions};
use futures::{
    future::{self, Shared},
    stream::FuturesUnordered,
    FutureExt, StreamExt,
};
use gpui::{
    AppContext, Context as _, EventEmitter, Image, Model, ModelContext, RenderImage, Subscription,
    Task, UpdateGlobal, View, WeakView,
};

use language::{
    AnchorRangeExt, Bias, Buffer, BufferSnapshot, LanguageRegistry, OffsetRangeExt, ParseStatus,
    Point, ToOffset,
};
use language_model::{
    LanguageModelImage, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelTool, Role,
};
use open_ai::Model as OpenAiModel;
use paths::{context_images_dir, contexts_dir};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    collections::hash_map,
    fmt::Debug,
    iter, mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::AssistantKind;
use ui::{SharedString, WindowContext};
use util::{post_inc, ResultExt, TryFutureExt};
use uuid::Uuid;
use workspace::Workspace;

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


<rewrite_this>

</rewrite_this>
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

#[derive(Debug, Clone)]
pub enum ContextEvent {
    ShowAssistError(SharedString),
    MessagesEdited,
    SummaryChanged,
    WorkflowStepsRemoved(Vec<Range<language::Anchor>>),
    WorkflowStepUpdated(Range<language::Anchor>),
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

#[derive(Clone, Debug)]
pub struct MessageImage {
    image_id: u64,
    image: Shared<Task<Option<LanguageModelImage>>>,
}

impl PartialEq for MessageImage {
    fn eq(&self, other: &Self) -> bool {
        self.image_id == other.image_id
    }
}

impl Eq for MessageImage {}

#[derive(Clone, Debug)]
pub struct Message {
    pub image_offsets: SmallVec<[(usize, MessageImage); 1]>,
    pub offset_range: Range<usize>,
    pub index_range: Range<usize>,
    pub id: MessageId,
    pub anchor: language::Anchor,
    pub role: Role,
    pub status: MessageStatus,
}

impl Message {
    fn to_request_message(&self, buffer: &Buffer) -> LanguageModelRequestMessage {
        let mut content = Vec::new();

        let mut range_start = self.offset_range.start;
        for (image_offset, message_image) in self.image_offsets.iter() {
            if *image_offset != range_start {
                content.push(
                    buffer
                        .text_for_range(range_start..*image_offset)
                        .collect::<String>()
                        .into(),
                )
            }

            if let Some(image) = message_image.image.clone().now_or_never().flatten() {
                content.push(language_model::MessageContent::Image(image));
            }

            range_start = *image_offset;
        }
        if range_start != self.offset_range.end {
            content.push(
                buffer
                    .text_for_range(range_start..self.offset_range.end)
                    .collect::<String>()
                    .into(),
            )
        }

        LanguageModelRequestMessage {
            role: self.role,
            content,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageAnchor {
    pub anchor: language::Anchor,
    pub image_id: u64,
    pub render_image: Arc<RenderImage>,
    pub image: Shared<Task<Option<LanguageModelImage>>>,
}

impl PartialEq for ImageAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.image_id == other.image_id
    }
}

struct PendingCompletion {
    id: usize,
    assistant_message_id: MessageId,
    _task: Task<()>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SlashCommandId(clock::Lamport);

#[derive(Debug)]
pub struct WorkflowStep {
    pub tagged_range: Range<language::Anchor>,
    pub status: WorkflowStepStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedWorkflowStep {
    pub title: String,
    pub suggestions: HashMap<Model<Buffer>, Vec<WorkflowSuggestionGroup>>,
}

pub enum WorkflowStepStatus {
    Pending(Task<Option<()>>),
    Resolved(ResolvedWorkflowStep),
    Error(Arc<anyhow::Error>),
}

impl WorkflowStepStatus {
    pub fn into_resolved(&self) -> Option<Result<ResolvedWorkflowStep, Arc<anyhow::Error>>> {
        match self {
            WorkflowStepStatus::Resolved(resolved) => Some(Ok(resolved.clone())),
            WorkflowStepStatus::Error(error) => Some(Err(error.clone())),
            WorkflowStepStatus::Pending(_) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowSuggestionGroup {
    pub context_range: Range<language::Anchor>,
    pub suggestions: Vec<WorkflowSuggestion>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowSuggestion {
    Update {
        range: Range<language::Anchor>,
        description: String,
    },
    CreateFile {
        description: String,
    },
    InsertSiblingBefore {
        position: language::Anchor,
        description: String,
    },
    InsertSiblingAfter {
        position: language::Anchor,
        description: String,
    },
    PrependChild {
        position: language::Anchor,
        description: String,
    },
    AppendChild {
        position: language::Anchor,
        description: String,
    },
    Delete {
        range: Range<language::Anchor>,
    },
}

impl WorkflowSuggestion {
    pub fn range(&self) -> Range<language::Anchor> {
        match self {
            WorkflowSuggestion::Update { range, .. } => range.clone(),
            WorkflowSuggestion::CreateFile { .. } => language::Anchor::MIN..language::Anchor::MAX,
            WorkflowSuggestion::InsertSiblingBefore { position, .. }
            | WorkflowSuggestion::InsertSiblingAfter { position, .. }
            | WorkflowSuggestion::PrependChild { position, .. }
            | WorkflowSuggestion::AppendChild { position, .. } => *position..*position,
            WorkflowSuggestion::Delete { range } => range.clone(),
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            WorkflowSuggestion::Update { description, .. }
            | WorkflowSuggestion::CreateFile { description }
            | WorkflowSuggestion::InsertSiblingBefore { description, .. }
            | WorkflowSuggestion::InsertSiblingAfter { description, .. }
            | WorkflowSuggestion::PrependChild { description, .. }
            | WorkflowSuggestion::AppendChild { description, .. } => Some(description),
            WorkflowSuggestion::Delete { .. } => None,
        }
    }

    fn description_mut(&mut self) -> Option<&mut String> {
        match self {
            WorkflowSuggestion::Update { description, .. }
            | WorkflowSuggestion::CreateFile { description }
            | WorkflowSuggestion::InsertSiblingBefore { description, .. }
            | WorkflowSuggestion::InsertSiblingAfter { description, .. }
            | WorkflowSuggestion::PrependChild { description, .. }
            | WorkflowSuggestion::AppendChild { description, .. } => Some(description),
            WorkflowSuggestion::Delete { .. } => None,
        }
    }

    fn try_merge(&mut self, other: &Self, buffer: &BufferSnapshot) -> bool {
        let range = self.range();
        let other_range = other.range();

        // Don't merge if we don't contain the other suggestion.
        if range.start.cmp(&other_range.start, buffer).is_gt()
            || range.end.cmp(&other_range.end, buffer).is_lt()
        {
            return false;
        }

        if let Some(description) = self.description_mut() {
            if let Some(other_description) = other.description() {
                description.push('\n');
                description.push_str(other_description);
            }
        }
        true
    }

    pub fn show(
        &self,
        editor: &View<Editor>,
        excerpt_id: editor::ExcerptId,
        workspace: &WeakView<Workspace>,
        assistant_panel: &View<AssistantPanel>,
        cx: &mut WindowContext,
    ) -> Option<InlineAssistId> {
        let mut initial_transaction_id = None;
        let initial_prompt;
        let suggestion_range;
        let buffer = editor.read(cx).buffer().clone();
        let snapshot = buffer.read(cx).snapshot(cx);

        match self {
            WorkflowSuggestion::Update { range, description } => {
                initial_prompt = description.clone();
                suggestion_range = snapshot.anchor_in_excerpt(excerpt_id, range.start)?
                    ..snapshot.anchor_in_excerpt(excerpt_id, range.end)?;
            }
            WorkflowSuggestion::CreateFile { description } => {
                initial_prompt = description.clone();
                suggestion_range = editor::Anchor::min()..editor::Anchor::min();
            }
            WorkflowSuggestion::InsertSiblingBefore {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::InsertSiblingAfter {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::PrependChild {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, false, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::AppendChild {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, false, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::Delete { range } => {
                initial_prompt = "Delete".to_string();
                suggestion_range = snapshot.anchor_in_excerpt(excerpt_id, range.start)?
                    ..snapshot.anchor_in_excerpt(excerpt_id, range.end)?;
            }
        }

        InlineAssistant::update_global(cx, |inline_assistant, cx| {
            Some(inline_assistant.suggest_assist(
                editor,
                suggestion_range,
                initial_prompt,
                initial_transaction_id,
                Some(workspace.clone()),
                Some(assistant_panel),
                cx,
            ))
        })
    }
}

impl Debug for WorkflowStepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowStepStatus::Pending(_) => write!(f, "WorkflowStepStatus::Pending"),
            WorkflowStepStatus::Resolved(ResolvedWorkflowStep { title, suggestions }) => f
                .debug_struct("WorkflowStepStatus::Resolved")
                .field("title", title)
                .field("suggestions", suggestions)
                .finish(),
            WorkflowStepStatus::Error(error) => f
                .debug_tuple("WorkflowStepStatus::Error")
                .field(error)
                .finish(),
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
    images: HashMap<u64, (Arc<RenderImage>, Shared<Task<Option<LanguageModelImage>>>)>,
    image_anchors: Vec<ImageAnchor>,
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
    workflow_steps: Vec<WorkflowStep>,
    edits_since_last_workflow_step_prune: language::Subscription,
    project: Option<Model<Project>>,
    prompt_builder: Arc<PromptBuilder>,
}

impl EventEmitter<ContextEvent> for Context {}

impl Context {
    pub fn local(
        language_registry: Arc<LanguageRegistry>,
        project: Option<Model<Project>>,
        telemetry: Option<Arc<Telemetry>>,
        prompt_builder: Arc<PromptBuilder>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::new(
            ContextId::new(),
            ReplicaId::default(),
            language::Capability::ReadWrite,
            language_registry,
            prompt_builder,
            project,
            telemetry,
            cx,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ContextId,
        replica_id: ReplicaId,
        capability: language::Capability,
        language_registry: Arc<LanguageRegistry>,
        prompt_builder: Arc<PromptBuilder>,
        project: Option<Model<Project>>,
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
        let edits_since_last_workflow_step_prune =
            buffer.update(cx, |buffer, _| buffer.subscribe());
        let mut this = Self {
            id,
            timestamp: clock::Lamport::new(replica_id),
            version: clock::Global::new(),
            pending_ops: Vec::new(),
            operations: Vec::new(),
            message_anchors: Default::default(),
            image_anchors: Default::default(),
            images: Default::default(),
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
            project,
            language_registry,
            workflow_steps: Vec::new(),
            edits_since_last_workflow_step_prune,
            prompt_builder,
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
                    image_offsets: message
                        .image_offsets
                        .iter()
                        .map(|image_offset| (image_offset.0, image_offset.1.image_id))
                        .collect(),
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
        prompt_builder: Arc<PromptBuilder>,
        project: Option<Model<Project>>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let id = saved_context.id.clone().unwrap_or_else(|| ContextId::new());
        let mut this = Self::new(
            id,
            ReplicaId::default(),
            language::Capability::ReadWrite,
            language_registry,
            prompt_builder,
            project,
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

    pub fn workflow_steps(&self) -> &[WorkflowStep] {
        &self.workflow_steps
    }

    pub fn workflow_step_for_range(&self, range: Range<language::Anchor>) -> Option<&WorkflowStep> {
        self.workflow_steps
            .iter()
            .find(|step| step.tagged_range == range)
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
                // Use `inclusive = true` to invalidate a step when an edit occurs
                // at the start/end of a parsed step.
                self.prune_invalid_workflow_steps(true, cx);
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
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };
        self.pending_token_count = cx.spawn(|this, mut cx| {
            async move {
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;

                let token_count = cx.update(|cx| model.count_tokens(request, cx))?.await?;
                this.update(&mut cx, |this, cx| {
                    this.token_count = Some(token_count);
                    cx.notify()
                })
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
                    let arguments = command_line
                        .arguments
                        .iter()
                        .filter_map(|argument_range| {
                            if argument_range.is_empty() {
                                None
                            } else {
                                line.get(argument_range.clone())
                            }
                        })
                        .map(ToOwned::to_owned)
                        .collect::<SmallVec<_>>();
                    if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
                        if !command.requires_argument() || !arguments.is_empty() {
                            let start_ix = offset + command_line.name.start - 1;
                            let end_ix = offset
                                + command_line
                                    .arguments
                                    .last()
                                    .map_or(command_line.name.end, |argument| argument.end);
                            let source_range =
                                buffer.anchor_after(start_ix)..buffer.anchor_after(end_ix);
                            let pending_command = PendingSlashCommand {
                                name: name.to_string(),
                                arguments,
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

    fn prune_invalid_workflow_steps(&mut self, inclusive: bool, cx: &mut ModelContext<Self>) {
        let mut removed = Vec::new();

        for edit_range in self.edits_since_last_workflow_step_prune.consume() {
            let intersecting_range = self.find_intersecting_steps(edit_range.new, inclusive, cx);
            removed.extend(
                self.workflow_steps
                    .drain(intersecting_range)
                    .map(|step| step.tagged_range),
            );
        }

        if !removed.is_empty() {
            cx.emit(ContextEvent::WorkflowStepsRemoved(removed));
            cx.notify();
        }
    }

    fn find_intersecting_steps(
        &self,
        range: Range<usize>,
        inclusive: bool,
        cx: &AppContext,
    ) -> Range<usize> {
        let buffer = self.buffer.read(cx);
        let start_ix = match self.workflow_steps.binary_search_by(|probe| {
            probe
                .tagged_range
                .end
                .to_offset(buffer)
                .cmp(&range.start)
                .then(if inclusive {
                    Ordering::Greater
                } else {
                    Ordering::Less
                })
        }) {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match self.workflow_steps.binary_search_by(|probe| {
            probe
                .tagged_range
                .start
                .to_offset(buffer)
                .cmp(&range.end)
                .then(if inclusive {
                    Ordering::Less
                } else {
                    Ordering::Greater
                })
        }) {
            Ok(ix) | Err(ix) => ix,
        };
        start_ix..end_ix
    }

    fn parse_workflow_steps_in_range(
        &mut self,
        range: Range<usize>,
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
    ) {
        let mut new_edit_steps = Vec::new();
        let mut edits = Vec::new();

        let buffer = self.buffer.read(cx).snapshot();
        let mut message_lines = buffer.as_rope().chunks_in_range(range).lines();
        let mut in_step = false;
        let mut step_open_tag_start_ix = 0;
        let mut line_start_offset = message_lines.offset();

        while let Some(line) = message_lines.next() {
            if let Some(step_start_index) = line.find("<step>") {
                if !in_step {
                    in_step = true;
                    step_open_tag_start_ix = line_start_offset + step_start_index;
                }
            }

            if let Some(step_end_index) = line.find("</step>") {
                if in_step {
                    let mut step_open_tag_end_ix = step_open_tag_start_ix + "<step>".len();
                    if buffer.chars_at(step_open_tag_end_ix).next() == Some('\n') {
                        step_open_tag_end_ix += 1;
                    }
                    let mut step_end_tag_start_ix = line_start_offset + step_end_index;
                    let step_end_tag_end_ix = step_end_tag_start_ix + "</step>".len();
                    if buffer.reversed_chars_at(step_end_tag_start_ix).next() == Some('\n') {
                        step_end_tag_start_ix -= 1;
                    }
                    edits.push((step_open_tag_start_ix..step_open_tag_end_ix, ""));
                    edits.push((step_end_tag_start_ix..step_end_tag_end_ix, ""));
                    let tagged_range = buffer.anchor_after(step_open_tag_end_ix)
                        ..buffer.anchor_before(step_end_tag_start_ix);

                    // Check if a step with the same range already exists
                    let existing_step_index = self
                        .workflow_steps
                        .binary_search_by(|probe| probe.tagged_range.cmp(&tagged_range, &buffer));

                    if let Err(ix) = existing_step_index {
                        new_edit_steps.push((
                            ix,
                            WorkflowStep {
                                tagged_range,
                                status: WorkflowStepStatus::Pending(Task::ready(None)),
                            },
                        ));
                    }

                    in_step = false;
                }
            }

            line_start_offset = message_lines.offset();
        }

        let mut updated = Vec::new();
        for (index, step) in new_edit_steps.into_iter().rev() {
            let step_range = step.tagged_range.clone();
            updated.push(step_range.clone());
            self.workflow_steps.insert(index, step);
            self.resolve_workflow_step(step_range, project.clone(), cx);
        }

        // Delete <step> tags, making sure we don't accidentally invalidate
        // the step we just parsed.
        self.buffer
            .update(cx, |buffer, cx| buffer.edit(edits, None, cx));
        self.edits_since_last_workflow_step_prune.consume();
    }

    pub fn resolve_workflow_step(
        &mut self,
        tagged_range: Range<language::Anchor>,
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
    ) {
        let Ok(step_index) = self
            .workflow_steps
            .binary_search_by(|step| step.tagged_range.cmp(&tagged_range, self.buffer.read(cx)))
        else {
            return;
        };

        let mut request = self.to_completion_request(cx);
        let Some(edit_step) = self.workflow_steps.get_mut(step_index) else {
            return;
        };

        if let Some(model) = LanguageModelRegistry::read_global(cx).active_model() {
            let step_text = self
                .buffer
                .read(cx)
                .text_for_range(tagged_range.clone())
                .collect::<String>();

            let tagged_range = tagged_range.clone();
            edit_step.status = WorkflowStepStatus::Pending(cx.spawn(|this, mut cx| {
                async move {
                    let result = async {
                        let mut prompt = this.update(&mut cx, |this, _| {

</document>

Let's focus on a subset of the document above. Your task is to rewrite the <rewrite_this> tags, making changes based on a location and a user prompt.

<user_prompt>
add a doc comment
</user_prompt>

<surrounding_context>
    }

    pub fn to_proto(&self) -> String {
        self.0.clone()
    }
}


<rewrite_this>
<insert_here></insert_here>
</rewrite_this>
#[derive(Clone, Debug)]
pub enum ContextOperation {
    InsertMessage {
        anchor: MessageAnchor,
        metadata: MessageMetadata,
        version: clock::Global,
    },
    UpdateMessage {
</surrounding_context>

<guidelines>
1. Rewrite the <rewrite_this> tags and all the content inside, making only the requested changes at the marked locations.
2. Rewrite the full block, even including some parts that remain unchanged. Don't omit anything.
3. Use the correct indentation on the inserted lines.
4. Do NOT output anything outside the <rewrite_this> tags.
5. Only insert content within the <insert_here> tags. Rewrite everything else as unchanged.
6. Do NOT include any `<insert_here>` tags in your response
7. Make only the changes needed to address the prompt, nothing more.
</guidelines>

<expected_output_format>
```
{{rewritten_code}}
```
Where {{rewritten_code}} is replaced with your modified `<rewrite_this>' section.
</expected_output_format>

<examples>
Here is a worked example.
<example_1_document>
use std::collections::HashMap;

<rewrite_this>

</rewrite_this>
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let mut users = HashMap::new();

    users.insert(1, User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    });

    users.insert(2, User {
        id: 2,
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
    });

    // Print all users
    for (_, user) in users.iter() {
        println!("{:?}", user);
    }

    match users.get(&1) {
        Some(user) => println!("Found user: {}", user.name),
        None => println!("User not found"),
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);

    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);
}
</example_1_document>
<example_1_user_prompt>
derive common traits
</example_1_user_prompt>
<example_1_surrounding_context>
use std::collections::HashMap;

<rewrite_this>
<insert_here></insert_here>
</rewrite_this>
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
</example_1_surrounding_context>
<example_1_output>
<discussion>
We'll derive the commonly used Debug trait for the User struct, terminating just before the struct is declared.
</discussion>
<rewrite_this>
#[derive(Debug)]
</rewrite_this>
</example_1_output>
</examples>

Reminder!

<user_prompt>
add a doc comment
</user_prompt>

<surrounding_context>
    }

    pub fn to_proto(&self) -> String {
        self.0.clone()
    }
}


<rewrite_this>
<insert_here></insert_here>
</rewrite_this>
#[derive(Clone, Debug)]
pub enum ContextOperation {
    InsertMessage {
        anchor: MessageAnchor,
        metadata: MessageMetadata,
        version: clock::Global,
    },
    UpdateMessage {
</surrounding_context>

Now, provide your modified `rewrite_this` section using the format shown above. Discuss where this modified content will start and end, including the necessary <rewrite_this> tags themselves, then produce the content between the tags. Do not include any surrounding code outside of the tags. Start immediately with <discussion>.
