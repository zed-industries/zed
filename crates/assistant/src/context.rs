use crate::{
    prompts::PromptBuilder, slash_command::SlashCommandLine, workflow::WorkflowStepResolution,
    MessageId, MessageStatus,
};
use anyhow::{anyhow, Context as _, Result};
use assistant_slash_command::{
    SlashCommandOutput, SlashCommandOutputSection, SlashCommandRegistry,
};
use client::{self, proto, telemetry::Telemetry};
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use fs::{Fs, RemoveOptions};
use futures::{future::Shared, stream::FuturesUnordered, FutureExt, StreamExt};
use gpui::{
    AppContext, Context as _, EventEmitter, Image, Model, ModelContext, RenderImage, SharedString,
    Subscription, Task,
};

use language::{AnchorRangeExt, Bias, Buffer, LanguageRegistry, OffsetRangeExt, Point, ToOffset};
use language_model::{
    LanguageModelImage, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    Role,
};
use open_ai::Model as OpenAiModel;
use paths::{context_images_dir, contexts_dir};
use project::Project;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    cmp::Ordering,
    collections::hash_map,
    fmt::Debug,
    iter, mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::AssistantKind;
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

pub struct WorkflowStep {
    pub tagged_range: Range<language::Anchor>,
    pub resolution: Model<WorkflowStepResolution>,
    pub _task: Option<Task<()>>,
}

impl Debug for WorkflowStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkflowStep")
            .field("tagged_range", &self.tagged_range)
            .finish_non_exhaustive()
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

    pub fn project(&self) -> Option<Model<Project>> {
        self.project.clone()
    }

    pub fn prompt_builder(&self) -> Arc<PromptBuilder> {
        self.prompt_builder.clone()
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

    fn parse_workflow_steps_in_range(&mut self, range: Range<usize>, cx: &mut ModelContext<Self>) {
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
                                resolution: cx.new_model(|_| {
                                    WorkflowStepResolution::new(tagged_range.clone())
                                }),
                                tagged_range,
                                _task: None,
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
            self.resolve_workflow_step(step_range, cx);
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
        cx: &mut ModelContext<Self>,
    ) {
        let Ok(step_index) = self
            .workflow_steps
            .binary_search_by(|step| step.tagged_range.cmp(&tagged_range, self.buffer.read(cx)))
        else {
            return;
        };

        cx.emit(ContextEvent::WorkflowStepUpdated(tagged_range.clone()));
        cx.notify();

        let task = self.workflow_steps[step_index]
            .resolution
            .update(cx, |resolution, cx| resolution.resolve(self, cx));
        self.workflow_steps[step_index]._task = task.map(|task| {
            cx.spawn(|this, mut cx| async move {
                task.await;
                this.update(&mut cx, |_, cx| {
                    cx.emit(ContextEvent::WorkflowStepUpdated(tagged_range));
                    cx.notify();
                })
                .ok();
            })
        });
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
        let provider = LanguageModelRegistry::read_global(cx).active_provider()?;
        let model = LanguageModelRegistry::read_global(cx).active_model()?;
        let last_message_id = self.message_anchors.iter().rev().find_map(|message| {
            message
                .start
                .is_valid(self.buffer.read(cx))
                .then_some(message.id)
        })?;

        if !provider.is_authenticated(cx) {
            log::info!("completion provider has no credentials");
            return None;
        }

        let request = self.to_completion_request(cx);
        let assistant_message = self
            .insert_message_after(last_message_id, Role::Assistant, MessageStatus::Pending, cx)
            .unwrap();

        // Queue up the user's next reply.
        let user_message = self
            .insert_message_after(assistant_message.id, Role::User, MessageStatus::Done, cx)
            .unwrap();

        let pending_completion_id = post_inc(&mut self.completion_count);

        let task = cx.spawn({
            |this, mut cx| async move {
                let stream = model.stream_completion(request, &cx);
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

                            // Use `inclusive = false` as edits might occur at the end of a parsed step.
                            this.prune_invalid_workflow_steps(false, cx);
                            this.parse_workflow_steps_in_range(message_range, cx);
                            cx.emit(ContextEvent::StreamedCompletion);

                            Some(())
                        })?;
                        smol::future::yield_now().await;
                    }
                    this.update(&mut cx, |this, cx| {
                        this.pending_completions
                            .retain(|completion| completion.id != pending_completion_id);
                        this.summarize(false, cx);
                    })?;

                    anyhow::Ok(())
                };

                let result = stream_completion.await;

                this.update(&mut cx, |this, cx| {
                    let error_message = result
                        .err()
                        .map(|error| error.to_string().trim().to_string());

                    if let Some(error_message) = error_message.as_ref() {
                        cx.emit(ContextEvent::ShowAssistError(SharedString::from(
                            error_message.clone(),
                        )));
                    }

                    this.update_metadata(assistant_message_id, cx, |metadata| {
                        if let Some(error_message) = error_message.as_ref() {
                            metadata.status =
                                MessageStatus::Error(SharedString::from(error_message.clone()));
                        } else {
                            metadata.status = MessageStatus::Done;
                        }
                    });

                    if let Some(telemetry) = this.telemetry.as_ref() {
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
            id: pending_completion_id,
            assistant_message_id: assistant_message.id,
            _task: task,
        });

        Some(user_message)
    }

    pub fn to_completion_request(&self, cx: &AppContext) -> LanguageModelRequest {
        let buffer = self.buffer.read(cx);
        let request_messages = self
            .messages(cx)
            .filter(|message| message.status == MessageStatus::Done)
            .map(|message| message.to_request_message(&buffer))
            .collect();

        LanguageModelRequest {
            messages: request_messages,
            stop: vec![],
            temperature: 1.0,
        }
    }

    pub fn cancel_last_assist(&mut self, cx: &mut ModelContext<Self>) -> bool {
        if let Some(pending_completion) = self.pending_completions.pop() {
            self.update_metadata(pending_completion.assistant_message_id, cx, |metadata| {
                if metadata.status == MessageStatus::Pending {
                    metadata.status = MessageStatus::Canceled;
                }
            });
            true
        } else {
            false
        }
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

    pub fn insert_image(&mut self, image: Image, cx: &mut ModelContext<Self>) -> Option<()> {
        if let hash_map::Entry::Vacant(entry) = self.images.entry(image.id()) {
            entry.insert((
                image.to_image_data(cx).log_err()?,
                LanguageModelImage::from_image(image, cx).shared(),
            ));
        }

        Some(())
    }

    pub fn insert_image_anchor(
        &mut self,
        image_id: u64,
        anchor: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> bool {
        cx.emit(ContextEvent::MessagesEdited);

        let buffer = self.buffer.read(cx);
        let insertion_ix = match self
            .image_anchors
            .binary_search_by(|existing_anchor| anchor.cmp(&existing_anchor.anchor, buffer))
        {
            Ok(ix) => ix,
            Err(ix) => ix,
        };

        if let Some((render_image, image)) = self.images.get(&image_id) {
            self.image_anchors.insert(
                insertion_ix,
                ImageAnchor {
                    anchor,
                    image_id,
                    image: image.clone(),
                    render_image: render_image.clone(),
                },
            );

            true
        } else {
            false
        }
    }

    pub fn images<'a>(&'a self, _cx: &'a AppContext) -> impl 'a + Iterator<Item = ImageAnchor> {
        self.image_anchors.iter().cloned()
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

            // TODO: why did this start panicking?
            if range.start > message.offset_range.start
                && range.end < message.offset_range.end.saturating_sub(1)
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

    pub(super) fn summarize(&mut self, replace_old: bool, cx: &mut ModelContext<Self>) {
        let Some(provider) = LanguageModelRegistry::read_global(cx).active_provider() else {
            return;
        };
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };

        if replace_old || (self.message_anchors.len() >= 2 && self.summary.is_none()) {
            if !provider.is_authenticated(cx) {
                return;
            }

            let messages = self
                .messages(cx)
                .map(|message| message.to_request_message(self.buffer.read(cx)))
                .chain(Some(LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![
                        "Summarize the context into a short title without punctuation.".into(),
                    ],
                }));
            let request = LanguageModelRequest {
                messages: messages.collect(),
                stop: vec![],
                temperature: 1.0,
            };

            self.pending_summary = cx.spawn(|this, mut cx| {
                async move {
                    let stream = model.stream_completion(request, &cx);
                    let mut messages = stream.await?;

                    let mut replaced = !replace_old;
                    while let Some(message) = messages.next().await {
                        let text = message?;
                        let mut lines = text.lines();
                        this.update(&mut cx, |this, cx| {
                            let version = this.version.clone();
                            let timestamp = this.next_timestamp();
                            let summary = this.summary.get_or_insert(ContextSummary::default());
                            if !replaced && replace_old {
                                summary.text.clear();
                                replaced = true;
                            }
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
        let messages = self.message_anchors.iter().enumerate();
        let images = self.image_anchors.iter();

        Self::messages_from_iters(buffer, &self.messages_metadata, messages, images)
    }

    pub fn messages_from_iters<'a>(
        buffer: &'a Buffer,
        metadata: &'a HashMap<MessageId, MessageMetadata>,
        messages: impl Iterator<Item = (usize, &'a MessageAnchor)> + 'a,
        images: impl Iterator<Item = &'a ImageAnchor> + 'a,
    ) -> impl 'a + Iterator<Item = Message> {
        let mut messages = messages.peekable();
        let mut images = images.peekable();

        iter::from_fn(move || {
            if let Some((start_ix, message_anchor)) = messages.next() {
                let metadata = metadata.get(&message_anchor.id)?;

                let message_start = message_anchor.start.to_offset(buffer);
                let mut message_end = None;
                let mut end_ix = start_ix;
                while let Some((_, next_message)) = messages.peek() {
                    if next_message.start.is_valid(buffer) {
                        message_end = Some(next_message.start);
                        break;
                    } else {
                        end_ix += 1;
                        messages.next();
                    }
                }
                let message_end_anchor = message_end.unwrap_or(language::Anchor::MAX);
                let message_end = message_end_anchor.to_offset(buffer);

                let mut image_offsets = SmallVec::new();
                while let Some(image_anchor) = images.peek() {
                    if image_anchor.anchor.cmp(&message_end_anchor, buffer).is_lt() {
                        image_offsets.push((
                            image_anchor.anchor.to_offset(buffer),
                            MessageImage {
                                image_id: image_anchor.image_id,
                                image: image_anchor.image.clone(),
                            },
                        ));
                        images.next();
                    } else {
                        break;
                    }
                }

                return Some(Message {
                    index_range: start_ix..end_ix,
                    offset_range: message_start..message_end,
                    id: message_anchor.id,
                    anchor: message_anchor.start,
                    role: metadata.role,
                    status: metadata.status.clone(),
                    image_offsets,
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
                this.read_with(&cx, |this, cx| this.serialize_images(fs.clone(), cx))?
                    .await;

                let context = this.read_with(&cx, |this, cx| this.serialize(cx))?;
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

                fs.create_dir(contexts_dir().as_ref()).await?;
                fs.atomic_write(new_path.clone(), serde_json::to_string(&context).unwrap())
                    .await?;
                if let Some(old_path) = old_path {
                    if new_path != old_path {
                        fs.remove_file(
                            &old_path,
                            RemoveOptions {
                                recursive: false,
                                ignore_if_not_exists: true,
                            },
                        )
                        .await?;
                    }
                }

                this.update(&mut cx, |this, _| this.path = Some(new_path))?;
            }

            Ok(())
        });
    }

    pub fn serialize_images(&self, fs: Arc<dyn Fs>, cx: &AppContext) -> Task<()> {
        let mut images_to_save = self
            .images
            .iter()
            .map(|(id, (_, llm_image))| {
                let fs = fs.clone();
                let llm_image = llm_image.clone();
                let id = *id;
                async move {
                    if let Some(llm_image) = llm_image.await {
                        let path: PathBuf =
                            context_images_dir().join(&format!("{}.png.base64", id));
                        if fs
                            .metadata(path.as_path())
                            .await
                            .log_err()
                            .flatten()
                            .is_none()
                        {
                            fs.atomic_write(path, llm_image.source.to_string())
                                .await
                                .log_err();
                        }
                    }
                }
            })
            .collect::<FuturesUnordered<_>>();
        cx.background_executor().spawn(async move {
            if fs
                .create_dir(context_images_dir().as_ref())
                .await
                .log_err()
                .is_some()
            {
                while let Some(_) = images_to_save.next().await {}
            }
        })
    }

    pub(crate) fn custom_summary(&mut self, custom_summary: String, cx: &mut ModelContext<Self>) {
        let timestamp = self.next_timestamp();
        let summary = self.summary.get_or_insert(ContextSummary::default());
        summary.timestamp = timestamp;
        summary.done = true;
        summary.text = custom_summary;
        cx.emit(ContextEvent::SummaryChanged);
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

#[derive(Debug, Clone)]
pub struct PendingSlashCommand {
    pub name: String,
    pub arguments: SmallVec<[String; 3]>,
    pub status: PendingSlashCommandStatus,
    pub source_range: Range<language::Anchor>,
}

#[derive(Debug, Clone)]
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
    #[serde(default)]
    // This is defaulted for backwards compatibility with JSON files created before August 2024. We didn't always have this field.
    pub image_offsets: Vec<(usize, u64)>,
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
                        image_offsets: Vec::new(),
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
        assistant_panel, prompt_library, slash_command::file_command, workflow::tool, MessageId,
    };
    use assistant_slash_command::{ArgumentCompletion, SlashCommand};
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
        LanguageModelRegistry::test(cx);
        cx.set_global(settings_store);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let context =
            cx.new_model(|cx| Context::local(registry, None, None, prompt_builder.clone(), cx));
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
        LanguageModelRegistry::test(cx);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let context =
            cx.new_model(|cx| Context::local(registry, None, None, prompt_builder.clone(), cx));
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
        LanguageModelRegistry::test(cx);
        cx.set_global(settings_store);
        assistant_panel::init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let context =
            cx.new_model(|cx| Context::local(registry, None, None, prompt_builder.clone(), cx));
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
        cx.update(LanguageModelRegistry::test);
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

        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let context = cx.new_model(|cx| {
            Context::local(registry.clone(), None, None, prompt_builder.clone(), cx)
        });

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
        cx.update(Project::init_settings);
        let fs = FakeFs::new(cx.executor());
        fs.as_fake()
            .insert_tree(
                "/root",
                json!({
                    "hello.rs": r#"
                    fn hello() {
                        println!("Hello, World!");
                    }
                "#.unindent()
                }),
            )
            .await;
        let project = Project::test(fs, [Path::new("/root")], cx).await;
        cx.update(LanguageModelRegistry::test);

        let model = cx.read(|cx| {
            LanguageModelRegistry::read_global(cx)
                .active_model()
                .unwrap()
        });
        cx.update(assistant_panel::init);
        let registry = Arc::new(LanguageRegistry::test(cx.executor()));

        // Create a new context
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let context = cx.new_model(|cx| {
            Context::local(
                registry.clone(),
                Some(project),
                None,
                prompt_builder.clone(),
                cx,
            )
        });
        let buffer = context.read_with(cx, |context, _| context.buffer.clone());

        // Simulate user input
        let user_message = indoc! {r#"
            Please add unnecessary complexity to this code:

            ```hello.rs
            fn main() {
                println!("Hello, World!");
            }
            ```
        "#};
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, user_message)], None, cx);
        });

        // Simulate LLM response with edit steps
        let llm_response = indoc! {r#"
            Sure, I can help you with that. Here's a step-by-step process:

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
        model
            .as_fake()
            .stream_last_completion_response(llm_response.to_string());
        model.as_fake().end_last_completion_stream();

        // Wait for the completion to be processed
        cx.run_until_parked();

        // Verify that the edit steps were parsed correctly
        context.read_with(cx, |context, cx| {
            assert_eq!(
                workflow_steps(context, cx),
                vec![
                    (
                        Point::new(response_start_row + 2, 0)
                            ..Point::new(response_start_row + 12, 3),
                        WorkflowStepTestStatus::Pending
                    ),
                    (
                        Point::new(response_start_row + 14, 0)
                            ..Point::new(response_start_row + 24, 3),
                        WorkflowStepTestStatus::Pending
                    ),
                ]
            );
        });

        model
            .as_fake()
            .respond_to_last_tool_use(tool::WorkflowStepResolution {
                step_title: "Title".into(),
                suggestions: vec![tool::WorkflowSuggestion {
                    path: "/root/hello.rs".into(),
                    // Simulate a symbol name that's slightly different than our outline query
                    kind: tool::WorkflowSuggestionKind::Update {
                        symbol: "fn main()".into(),
                        description: "Extract a greeting function".into(),
                    },
                }],
            });

        // Wait for tool use to be processed.
        cx.run_until_parked();

        // Verify that the first edit step is not pending anymore.
        context.read_with(cx, |context, cx| {
            assert_eq!(
                workflow_steps(context, cx),
                vec![
                    (
                        Point::new(response_start_row + 2, 0)
                            ..Point::new(response_start_row + 12, 3),
                        WorkflowStepTestStatus::Resolved
                    ),
                    (
                        Point::new(response_start_row + 14, 0)
                            ..Point::new(response_start_row + 24, 3),
                        WorkflowStepTestStatus::Pending
                    ),
                ]
            );
        });

        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        enum WorkflowStepTestStatus {
            Pending,
            Resolved,
            Error,
        }

        fn workflow_steps(
            context: &Context,
            cx: &AppContext,
        ) -> Vec<(Range<Point>, WorkflowStepTestStatus)> {
            context
                .workflow_steps
                .iter()
                .map(|step| {
                    let buffer = context.buffer.read(cx);
                    let status = match &step.resolution.read(cx).result {
                        None => WorkflowStepTestStatus::Pending,
                        Some(Ok(_)) => WorkflowStepTestStatus::Resolved,
                        Some(Err(_)) => WorkflowStepTestStatus::Error,
                    };
                    (step.tagged_range.to_point(buffer), status)
                })
                .collect()
        }
    }

    #[gpui::test]
    async fn test_serialization(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.update(LanguageModelRegistry::test);
        cx.update(assistant_panel::init);
        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let context = cx.new_model(|cx| {
            Context::local(registry.clone(), None, None, prompt_builder.clone(), cx)
        });
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
                prompt_builder.clone(),
                None,
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
        cx.update(LanguageModelRegistry::test);

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
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        for i in 0..num_peers {
            let context = cx.new_model(|cx| {
                Context::new(
                    context_id.clone(),
                    i as ReplicaId,
                    language::Capability::ReadWrite,
                    registry.clone(),
                    prompt_builder.clone(),
                    None,
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
            _arguments: &[String],
            _cancel: Arc<AtomicBool>,
            _workspace: Option<WeakView<Workspace>>,
            _cx: &mut WindowContext,
        ) -> Task<Result<Vec<ArgumentCompletion>>> {
            Task::ready(Ok(vec![]))
        }

        fn requires_argument(&self) -> bool {
            false
        }

        fn run(
            self: Arc<Self>,
            _arguments: &[String],
            _workspace: WeakView<Workspace>,
            _delegate: Option<Arc<dyn LspAdapterDelegate>>,
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
