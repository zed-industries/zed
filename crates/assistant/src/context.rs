#[cfg(test)]
mod context_tests;

use crate::{
    prompts::PromptBuilder, slash_command::SlashCommandLine, MessageId, MessageStatus,
    WorkflowStep, WorkflowStepEdit, WorkflowStepResolution, WorkflowSuggestionGroup,
};
use anyhow::{anyhow, Context as _, Result};
use assistant_slash_command::{
    SlashCommandOutput, SlashCommandOutputSection, SlashCommandRegistry,
};
use client::{self, proto, telemetry::Telemetry};
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use fs::{Fs, RemoveOptions};
use futures::{
    future::{self, Shared},
    stream::FuturesUnordered,
    FutureExt, StreamExt,
};
use gpui::{
    AppContext, AsyncAppContext, Context as _, EventEmitter, Image, Model, ModelContext,
    RenderImage, SharedString, Subscription, Task,
};

use language::{AnchorRangeExt, Bias, Buffer, LanguageRegistry, OffsetRangeExt, Point, ToOffset};
use language_model::{
    LanguageModel, LanguageModelCacheConfiguration, LanguageModelImage, LanguageModelRegistry,
    LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};
use open_ai::Model as OpenAiModel;
use paths::{context_images_dir, contexts_dir};
use project::Project;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    cmp::{self, max, Ordering},
    collections::hash_map,
    fmt::Debug,
    iter, mem,
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr as _,
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::AssistantKind;
use text::BufferSnapshot;
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
                        cache: None,
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
                    cache: None,
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
    StreamedCompletion,
    WorkflowStepsUpdated {
        removed: Vec<Range<language::Anchor>>,
        updated: Vec<Range<language::Anchor>>,
    },
    PendingSlashCommandsUpdated {
        removed: Vec<Range<language::Anchor>>,
        updated: Vec<PendingSlashCommand>,
    },
    SlashCommandFinished {
        output_range: Range<language::Anchor>,
        sections: Vec<SlashCommandOutputSection<language::Anchor>>,
        run_commands_in_output: bool,
        expand_result: bool,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CacheStatus {
    Pending,
    Cached,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageCacheMetadata {
    pub is_anchor: bool,
    pub is_final_anchor: bool,
    pub status: CacheStatus,
    pub cached_at: clock::Global,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub role: Role,
    pub status: MessageStatus,
    pub(crate) timestamp: clock::Lamport,
    #[serde(skip)]
    pub cache: Option<MessageCacheMetadata>,
    #[serde(default)]
    pub model_info: Option<ModelInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModelInfo {
    pub model_id: String,
    pub model_name: String,
    pub provider_id: String,
}

impl From<&Message> for MessageMetadata {
    fn from(message: &Message) -> Self {
        Self {
            role: message.role,
            status: message.status.clone(),
            timestamp: message.id.0,
            cache: message.cache.clone(),
            model_info: None, // This needs to be set separately
        }
    }
}

impl MessageMetadata {
    pub fn is_cache_valid(&self, buffer: &BufferSnapshot, range: &Range<usize>) -> bool {
        let result = match &self.cache {
            Some(MessageCacheMetadata { cached_at, .. }) => !buffer.has_edits_since_in_range(
                &cached_at,
                Range {
                    start: buffer.anchor_at(range.start, Bias::Right),
                    end: buffer.anchor_at(range.end, Bias::Left),
                },
            ),
            _ => false,
        };
        result
    }
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
    pub anchor_range: Range<language::Anchor>,
    pub id: MessageId,
    pub role: Role,
    pub status: MessageStatus,
    pub cache: Option<MessageCacheMetadata>,
}

impl Message {
    fn to_request_message(&self, buffer: &Buffer) -> Option<LanguageModelRequestMessage> {
        let mut content = Vec::new();

        let mut range_start = self.offset_range.start;
        for (image_offset, message_image) in self.image_offsets.iter() {
            if *image_offset != range_start {
                if let Some(text) = Self::collect_text_content(buffer, range_start..*image_offset) {
                    content.push(text);
                }
            }

            if let Some(image) = message_image.image.clone().now_or_never().flatten() {
                content.push(language_model::MessageContent::Image(image));
            }

            range_start = *image_offset;
        }
        if range_start != self.offset_range.end {
            if let Some(text) =
                Self::collect_text_content(buffer, range_start..self.offset_range.end)
            {
                content.push(text);
            }
        }

        if content.is_empty() {
            return None;
        }

        Some(LanguageModelRequestMessage {
            role: self.role,
            content,
            cache: self.cache.as_ref().map_or(false, |cache| cache.is_anchor),
        })
    }

    fn collect_text_content(buffer: &Buffer, range: Range<usize>) -> Option<MessageContent> {
        let text: String = buffer.text_for_range(range.clone()).collect();
        if text.trim().is_empty() {
            None
        } else {
            Some(MessageContent::Text(text))
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

struct PendingCompletion {
    id: usize,
    assistant_message_id: MessageId,
    _task: Task<()>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SlashCommandId(clock::Lamport);

#[derive(Clone, Debug)]
pub struct XmlTag {
    pub kind: XmlTagKind,
    pub range: Range<text::Anchor>,
    pub is_open_tag: bool,
}

#[derive(Copy, Clone, Debug, strum::EnumString, PartialEq, Eq, strum::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum XmlTagKind {
    Step,
    Edit,
    Path,
    Search,
    Within,
    Operation,
    Description,
}

pub struct Context {
    id: ContextId,
    timestamp: clock::Lamport,
    version: clock::Global,
    pending_ops: Vec<ContextOperation>,
    operations: Vec<ContextOperation>,
    buffer: Model<Buffer>,
    pending_slash_commands: Vec<PendingSlashCommand>,
    edits_since_last_parse: language::Subscription,
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
    pending_cache_warming_task: Task<Option<()>>,
    path: Option<PathBuf>,
    _subscriptions: Vec<Subscription>,
    telemetry: Option<Arc<Telemetry>>,
    language_registry: Arc<LanguageRegistry>,
    workflow_steps: Vec<WorkflowStep>,
    xml_tags: Vec<XmlTag>,
    project: Option<Model<Project>>,
    prompt_builder: Arc<PromptBuilder>,
}

trait ContextAnnotation {
    fn range(&self) -> &Range<language::Anchor>;
}

impl ContextAnnotation for PendingSlashCommand {
    fn range(&self) -> &Range<language::Anchor> {
        &self.source_range
    }
}

impl ContextAnnotation for WorkflowStep {
    fn range(&self) -> &Range<language::Anchor> {
        &self.range
    }
}

impl ContextAnnotation for XmlTag {
    fn range(&self) -> &Range<language::Anchor> {
        &self.range
    }
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
            edits_since_last_parse: edits_since_last_slash_command_parse,
            summary: None,
            pending_summary: Task::ready(None),
            completion_count: Default::default(),
            pending_completions: Default::default(),
            token_count: None,
            pending_token_count: Task::ready(None),
            pending_cache_warming_task: Task::ready(None),
            _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
            pending_save: Task::ready(Ok(())),
            path: None,
            buffer,
            telemetry,
            project,
            language_registry,
            workflow_steps: Vec::new(),
            xml_tags: Vec::new(),
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
                cache: None,
            },
        );
        this.message_anchors.push(message);

        this.set_language(cx);
        this.count_remaining_tokens(cx);
        this
    }

    pub(crate) fn serialize(&self, cx: &AppContext) -> SavedContext {
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
        let mut changed_messages = HashSet::default();
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
                        changed_messages.insert(anchor.id);
                        self.insert_message(anchor, metadata, cx);
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
                        changed_messages.insert(message_id);
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
                            expand_result: false,
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

        if !changed_messages.is_empty() {
            self.message_roles_updated(changed_messages, cx);
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

    pub fn language_registry(&self) -> Arc<LanguageRegistry> {
        self.language_registry.clone()
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

    pub(crate) fn workflow_step_containing(
        &self,
        offset: usize,
        cx: &AppContext,
    ) -> Option<&WorkflowStep> {
        let buffer = self.buffer.read(cx);
        let index = self
            .workflow_steps
            .binary_search_by(|step| {
                let step_range = step.range.to_offset(&buffer);
                if offset < step_range.start {
                    Ordering::Greater
                } else if offset > step_range.end {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .ok()?;
        Some(&self.workflow_steps[index])
    }

    pub fn workflow_step_ranges(&self) -> impl Iterator<Item = Range<language::Anchor>> + '_ {
        self.workflow_steps.iter().map(|step| step.range.clone())
    }

    pub(crate) fn workflow_step_for_range(
        &self,
        range: &Range<language::Anchor>,
        cx: &AppContext,
    ) -> Option<&WorkflowStep> {
        let buffer = self.buffer.read(cx);
        let index = self.workflow_step_index_for_range(range, buffer).ok()?;
        Some(&self.workflow_steps[index])
    }

    fn workflow_step_index_for_range(
        &self,
        tagged_range: &Range<text::Anchor>,
        buffer: &text::BufferSnapshot,
    ) -> Result<usize, usize> {
        self.workflow_steps
            .binary_search_by(|probe| probe.range.cmp(&tagged_range, buffer))
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
                self.reparse(cx);
                // Use `inclusive = true` to invalidate a step when an edit occurs
                // at the start/end of a parsed step.
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
                    this.start_cache_warming(&model, cx);
                    cx.notify()
                })
            }
            .log_err()
        });
    }

    pub fn mark_cache_anchors(
        &mut self,
        cache_configuration: &Option<LanguageModelCacheConfiguration>,
        speculative: bool,
        cx: &mut ModelContext<Self>,
    ) -> bool {
        let cache_configuration =
            cache_configuration
                .as_ref()
                .unwrap_or(&LanguageModelCacheConfiguration {
                    max_cache_anchors: 0,
                    should_speculate: false,
                    min_total_token: 0,
                });

        let messages: Vec<Message> = self.messages(cx).collect();

        let mut sorted_messages = messages.clone();
        if speculative {
            // Avoid caching the last message if this is a speculative cache fetch as
            // it's likely to change.
            sorted_messages.pop();
        }
        sorted_messages.retain(|m| m.role == Role::User);
        sorted_messages.sort_by(|a, b| b.offset_range.len().cmp(&a.offset_range.len()));

        let cache_anchors = if self.token_count.unwrap_or(0) < cache_configuration.min_total_token {
            // If we have't hit the minimum threshold to enable caching, don't cache anything.
            0
        } else {
            // Save 1 anchor for the inline assistant to use.
            max(cache_configuration.max_cache_anchors, 1) - 1
        };
        sorted_messages.truncate(cache_anchors);

        let anchors: HashSet<MessageId> = sorted_messages
            .into_iter()
            .map(|message| message.id)
            .collect();

        let buffer = self.buffer.read(cx).snapshot();
        let invalidated_caches: HashSet<MessageId> = messages
            .iter()
            .scan(false, |encountered_invalid, message| {
                let message_id = message.id;
                let is_invalid = self
                    .messages_metadata
                    .get(&message_id)
                    .map_or(true, |metadata| {
                        !metadata.is_cache_valid(&buffer, &message.offset_range)
                            || *encountered_invalid
                    });
                *encountered_invalid |= is_invalid;
                Some(if is_invalid { Some(message_id) } else { None })
            })
            .flatten()
            .collect();

        let last_anchor = messages.iter().rev().find_map(|message| {
            if anchors.contains(&message.id) {
                Some(message.id)
            } else {
                None
            }
        });

        let mut new_anchor_needs_caching = false;
        let current_version = &buffer.version;
        // If we have no anchors, mark all messages as not being cached.
        let mut hit_last_anchor = last_anchor.is_none();

        for message in messages.iter() {
            if hit_last_anchor {
                self.update_metadata(message.id, cx, |metadata| metadata.cache = None);
                continue;
            }

            if let Some(last_anchor) = last_anchor {
                if message.id == last_anchor {
                    hit_last_anchor = true;
                }
            }

            new_anchor_needs_caching = new_anchor_needs_caching
                || (invalidated_caches.contains(&message.id) && anchors.contains(&message.id));

            self.update_metadata(message.id, cx, |metadata| {
                let cache_status = if invalidated_caches.contains(&message.id) {
                    CacheStatus::Pending
                } else {
                    metadata
                        .cache
                        .as_ref()
                        .map_or(CacheStatus::Pending, |cm| cm.status.clone())
                };
                metadata.cache = Some(MessageCacheMetadata {
                    is_anchor: anchors.contains(&message.id),
                    is_final_anchor: hit_last_anchor,
                    status: cache_status,
                    cached_at: current_version.clone(),
                });
            });
        }
        new_anchor_needs_caching
    }

    fn start_cache_warming(&mut self, model: &Arc<dyn LanguageModel>, cx: &mut ModelContext<Self>) {
        let cache_configuration = model.cache_configuration();

        if !self.mark_cache_anchors(&cache_configuration, true, cx) {
            return;
        }
        if !self.pending_completions.is_empty() {
            return;
        }
        if let Some(cache_configuration) = cache_configuration {
            if !cache_configuration.should_speculate {
                return;
            }
        }

        let request = {
            let mut req = self.to_completion_request(cx);
            // Skip the last message because it's likely to change and
            // therefore would be a waste to cache.
            req.messages.pop();
            req.messages.push(LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Respond only with OK, nothing else.".into()],
                cache: false,
            });
            req
        };

        let model = Arc::clone(model);
        self.pending_cache_warming_task = cx.spawn(|this, mut cx| {
            async move {
                match model.stream_completion(request, &cx).await {
                    Ok(mut stream) => {
                        stream.next().await;
                        log::info!("Cache warming completed successfully");
                    }
                    Err(e) => {
                        log::warn!("Cache warming failed: {}", e);
                    }
                };
                this.update(&mut cx, |this, cx| {
                    this.update_cache_status_for_completion(cx);
                })
                .ok();
                anyhow::Ok(())
            }
            .log_err()
        });
    }

    pub fn update_cache_status_for_completion(&mut self, cx: &mut ModelContext<Self>) {
        let cached_message_ids: Vec<MessageId> = self
            .messages_metadata
            .iter()
            .filter_map(|(message_id, metadata)| {
                metadata.cache.as_ref().and_then(|cache| {
                    if cache.status == CacheStatus::Pending {
                        Some(*message_id)
                    } else {
                        None
                    }
                })
            })
            .collect();

        for message_id in cached_message_ids {
            self.update_metadata(message_id, cx, |metadata| {
                if let Some(cache) = &mut metadata.cache {
                    cache.status = CacheStatus::Cached;
                }
            });
        }
        cx.notify();
    }

    pub fn reparse(&mut self, cx: &mut ModelContext<Self>) {
        let buffer = self.buffer.read(cx).text_snapshot();
        let mut row_ranges = self
            .edits_since_last_parse
            .consume()
            .into_iter()
            .map(|edit| {
                let start_row = buffer.offset_to_point(edit.new.start).row;
                let end_row = buffer.offset_to_point(edit.new.end).row + 1;
                start_row..end_row
            })
            .peekable();

        let mut removed_slash_command_ranges = Vec::new();
        let mut updated_slash_commands = Vec::new();
        let mut removed_steps = Vec::new();
        let mut updated_steps = Vec::new();
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

            self.reparse_slash_commands_in_range(
                start..end,
                &buffer,
                &mut updated_slash_commands,
                &mut removed_slash_command_ranges,
                cx,
            );
            self.reparse_workflow_steps_in_range(
                start..end,
                &buffer,
                &mut updated_steps,
                &mut removed_steps,
                cx,
            );
        }

        if !updated_slash_commands.is_empty() || !removed_slash_command_ranges.is_empty() {
            cx.emit(ContextEvent::PendingSlashCommandsUpdated {
                removed: removed_slash_command_ranges,
                updated: updated_slash_commands,
            });
        }

        if !updated_steps.is_empty() || !removed_steps.is_empty() {
            cx.emit(ContextEvent::WorkflowStepsUpdated {
                removed: removed_steps,
                updated: updated_steps,
            });
        }
    }

    fn reparse_slash_commands_in_range(
        &mut self,
        range: Range<text::Anchor>,
        buffer: &BufferSnapshot,
        updated: &mut Vec<PendingSlashCommand>,
        removed: &mut Vec<Range<text::Anchor>>,
        cx: &AppContext,
    ) {
        let old_range = self.pending_command_indices_for_range(range.clone(), cx);

        let mut new_commands = Vec::new();
        let mut lines = buffer.text_for_range(range).lines();
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

    fn reparse_workflow_steps_in_range(
        &mut self,
        range: Range<text::Anchor>,
        buffer: &BufferSnapshot,
        updated: &mut Vec<Range<text::Anchor>>,
        removed: &mut Vec<Range<text::Anchor>>,
        cx: &mut ModelContext<Self>,
    ) {
        // Rebuild the XML tags in the edited range.
        let intersecting_tags_range =
            self.indices_intersecting_buffer_range(&self.xml_tags, range.clone(), cx);
        let new_tags = self.parse_xml_tags_in_range(buffer, range.clone(), cx);
        self.xml_tags
            .splice(intersecting_tags_range.clone(), new_tags);

        // Find which steps intersect the changed range.
        let intersecting_steps_range =
            self.indices_intersecting_buffer_range(&self.workflow_steps, range.clone(), cx);

        // Reparse all tags after the last unchanged step before the change.
        let mut tags_start_ix = 0;
        if let Some(preceding_unchanged_step) =
            self.workflow_steps[..intersecting_steps_range.start].last()
        {
            tags_start_ix = match self.xml_tags.binary_search_by(|tag| {
                tag.range
                    .start
                    .cmp(&preceding_unchanged_step.range.end, buffer)
                    .then(Ordering::Less)
            }) {
                Ok(ix) | Err(ix) => ix,
            };
        }

        // Rebuild the edit suggestions in the range.
        let mut new_steps = self.parse_steps(tags_start_ix, range.end, buffer);

        if let Some(project) = self.project() {
            for step in &mut new_steps {
                Self::resolve_workflow_step_internal(step, &project, cx);
            }
        }

        updated.extend(new_steps.iter().map(|step| step.range.clone()));
        let removed_steps = self
            .workflow_steps
            .splice(intersecting_steps_range, new_steps);
        removed.extend(
            removed_steps
                .map(|step| step.range)
                .filter(|range| !updated.contains(&range)),
        );
    }

    fn parse_xml_tags_in_range(
        &self,
        buffer: &BufferSnapshot,
        range: Range<text::Anchor>,
        cx: &AppContext,
    ) -> Vec<XmlTag> {
        let mut messages = self.messages(cx).peekable();

        let mut tags = Vec::new();
        let mut lines = buffer.text_for_range(range).lines();
        let mut offset = lines.offset();

        while let Some(line) = lines.next() {
            while let Some(message) = messages.peek() {
                if offset < message.offset_range.end {
                    break;
                } else {
                    messages.next();
                }
            }

            let is_assistant_message = messages
                .peek()
                .map_or(false, |message| message.role == Role::Assistant);
            if is_assistant_message {
                for (start_ix, _) in line.match_indices('<') {
                    let mut name_start_ix = start_ix + 1;
                    let closing_bracket_ix = line[start_ix..].find('>').map(|i| start_ix + i);
                    if let Some(closing_bracket_ix) = closing_bracket_ix {
                        let end_ix = closing_bracket_ix + 1;
                        let mut is_open_tag = true;
                        if line[name_start_ix..closing_bracket_ix].starts_with('/') {
                            name_start_ix += 1;
                            is_open_tag = false;
                        }
                        let tag_inner = &line[name_start_ix..closing_bracket_ix];
                        let tag_name_len = tag_inner
                            .find(|c: char| c.is_whitespace())
                            .unwrap_or(tag_inner.len());
                        if let Ok(kind) = XmlTagKind::from_str(&tag_inner[..tag_name_len]) {
                            tags.push(XmlTag {
                                range: buffer.anchor_after(offset + start_ix)
                                    ..buffer.anchor_before(offset + end_ix),
                                is_open_tag,
                                kind,
                            });
                        };
                    }
                }
            }

            offset = lines.offset();
        }
        tags
    }

    fn parse_steps(
        &mut self,
        tags_start_ix: usize,
        buffer_end: text::Anchor,
        buffer: &BufferSnapshot,
    ) -> Vec<WorkflowStep> {
        let mut new_steps = Vec::new();
        let mut pending_step = None;
        let mut edit_step_depth = 0;
        let mut tags = self.xml_tags[tags_start_ix..].iter().peekable();
        'tags: while let Some(tag) = tags.next() {
            if tag.range.start.cmp(&buffer_end, buffer).is_gt() && edit_step_depth == 0 {
                break;
            }

            if tag.kind == XmlTagKind::Step && tag.is_open_tag {
                edit_step_depth += 1;
                let edit_start = tag.range.start;
                let mut edits = Vec::new();
                let mut step = WorkflowStep {
                    range: edit_start..edit_start,
                    leading_tags_end: tag.range.end,
                    trailing_tag_start: None,
                    edits: Default::default(),
                    resolution: None,
                    resolution_task: None,
                };

                while let Some(tag) = tags.next() {
                    step.trailing_tag_start.get_or_insert(tag.range.start);

                    if tag.kind == XmlTagKind::Step && !tag.is_open_tag {
                        // step.trailing_tag_start = Some(tag.range.start);
                        edit_step_depth -= 1;
                        if edit_step_depth == 0 {
                            step.range.end = tag.range.end;
                            step.edits = edits.into();
                            new_steps.push(step);
                            continue 'tags;
                        }
                    }

                    if tag.kind == XmlTagKind::Edit && tag.is_open_tag {
                        let mut path = None;
                        let mut search = None;
                        let mut operation = None;
                        let mut description = None;

                        while let Some(tag) = tags.next() {
                            if tag.kind == XmlTagKind::Edit && !tag.is_open_tag {
                                edits.push(WorkflowStepEdit::new(
                                    path,
                                    operation,
                                    search,
                                    description,
                                ));
                                break;
                            }

                            if tag.is_open_tag
                                && [
                                    XmlTagKind::Path,
                                    XmlTagKind::Search,
                                    XmlTagKind::Operation,
                                    XmlTagKind::Description,
                                ]
                                .contains(&tag.kind)
                            {
                                let kind = tag.kind;
                                let content_start = tag.range.end;
                                if let Some(tag) = tags.peek() {
                                    if tag.kind == kind && !tag.is_open_tag {
                                        let tag = tags.next().unwrap();
                                        let content_end = tag.range.start;
                                        let mut content = buffer
                                            .text_for_range(content_start..content_end)
                                            .collect::<String>();
                                        content.truncate(content.trim_end().len());
                                        match kind {
                                            XmlTagKind::Path => path = Some(content),
                                            XmlTagKind::Operation => operation = Some(content),
                                            XmlTagKind::Search => {
                                                search = Some(content).filter(|s| !s.is_empty())
                                            }
                                            XmlTagKind::Description => {
                                                description =
                                                    Some(content).filter(|s| !s.is_empty())
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                pending_step = Some(step);
            }
        }

        if let Some(mut pending_step) = pending_step {
            pending_step.range.end = text::Anchor::MAX;
            new_steps.push(pending_step);
        }

        new_steps
    }

    pub fn resolve_workflow_step(
        &mut self,
        tagged_range: Range<text::Anchor>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let index = self
            .workflow_step_index_for_range(&tagged_range, self.buffer.read(cx))
            .ok()?;
        let step = &mut self.workflow_steps[index];
        let project = self.project.as_ref()?;
        step.resolution.take();
        Self::resolve_workflow_step_internal(step, project, cx);
        None
    }

    fn resolve_workflow_step_internal(
        step: &mut WorkflowStep,
        project: &Model<Project>,
        cx: &mut ModelContext<'_, Context>,
    ) {
        step.resolution_task = Some(cx.spawn({
            let range = step.range.clone();
            let edits = step.edits.clone();
            let project = project.clone();
            |this, mut cx| async move {
                let suggestion_groups =
                    Self::compute_step_resolution(project, edits, &mut cx).await;

                this.update(&mut cx, |this, cx| {
                    let buffer = this.buffer.read(cx).text_snapshot();
                    let ix = this.workflow_step_index_for_range(&range, &buffer).ok();
                    if let Some(ix) = ix {
                        let step = &mut this.workflow_steps[ix];

                        let resolution = suggestion_groups.map(|suggestion_groups| {
                            let mut title = String::new();
                            for mut chunk in buffer.text_for_range(
                                step.leading_tags_end
                                    ..step.trailing_tag_start.unwrap_or(step.range.end),
                            ) {
                                if title.is_empty() {
                                    chunk = chunk.trim_start();
                                }
                                if let Some((prefix, _)) = chunk.split_once('\n') {
                                    title.push_str(prefix);
                                    break;
                                } else {
                                    title.push_str(chunk);
                                }
                            }

                            WorkflowStepResolution {
                                title,
                                suggestion_groups,
                            }
                        });

                        step.resolution = Some(Arc::new(resolution));
                        cx.emit(ContextEvent::WorkflowStepsUpdated {
                            removed: vec![],
                            updated: vec![range],
                        })
                    }
                })
                .ok();
            }
        }));
    }

    async fn compute_step_resolution(
        project: Model<Project>,
        edits: Arc<[Result<WorkflowStepEdit>]>,
        cx: &mut AsyncAppContext,
    ) -> Result<HashMap<Model<Buffer>, Vec<WorkflowSuggestionGroup>>> {
        let mut suggestion_tasks = Vec::new();
        for edit in edits.iter() {
            let edit = edit.as_ref().map_err(|e| anyhow!("{e}"))?;
            suggestion_tasks.push(edit.resolve(project.clone(), cx.clone()));
        }

        // Expand the context ranges of each suggestion and group suggestions with overlapping context ranges.
        let suggestions = future::try_join_all(suggestion_tasks).await?;

        let mut suggestions_by_buffer = HashMap::default();
        for (buffer, suggestion) in suggestions {
            suggestions_by_buffer
                .entry(buffer)
                .or_insert_with(Vec::new)
                .push(suggestion);
        }

        let mut suggestion_groups_by_buffer = HashMap::default();
        for (buffer, mut suggestions) in suggestions_by_buffer {
            let mut suggestion_groups = Vec::<WorkflowSuggestionGroup>::new();
            let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot())?;
            // Sort suggestions by their range so that earlier, larger ranges come first
            suggestions.sort_by(|a, b| a.range().cmp(&b.range(), &snapshot));

            // Merge overlapping suggestions
            suggestions.dedup_by(|a, b| b.try_merge(a, &snapshot));

            // Create context ranges for each suggestion
            for suggestion in suggestions {
                let context_range = {
                    let suggestion_point_range = suggestion.range().to_point(&snapshot);
                    let start_row = suggestion_point_range.start.row.saturating_sub(5);
                    let end_row =
                        cmp::min(suggestion_point_range.end.row + 5, snapshot.max_point().row);
                    let start = snapshot.anchor_before(Point::new(start_row, 0));
                    let end =
                        snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
                    start..end
                };

                if let Some(last_group) = suggestion_groups.last_mut() {
                    if last_group
                        .context_range
                        .end
                        .cmp(&context_range.start, &snapshot)
                        .is_ge()
                    {
                        // Merge with the previous group if context ranges overlap
                        last_group.context_range.end = context_range.end;
                        last_group.suggestions.push(suggestion);
                    } else {
                        // Create a new group
                        suggestion_groups.push(WorkflowSuggestionGroup {
                            context_range,
                            suggestions: vec![suggestion],
                        });
                    }
                } else {
                    // Create the first group
                    suggestion_groups.push(WorkflowSuggestionGroup {
                        context_range,
                        suggestions: vec![suggestion],
                    });
                }
            }

            suggestion_groups_by_buffer.insert(buffer, suggestion_groups);
        }

        Ok(suggestion_groups_by_buffer)
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
        self.indices_intersecting_buffer_range(&self.pending_slash_commands, range, cx)
    }

    fn indices_intersecting_buffer_range<T: ContextAnnotation>(
        &self,
        all_annotations: &[T],
        range: Range<language::Anchor>,
        cx: &AppContext,
    ) -> Range<usize> {
        let buffer = self.buffer.read(cx);
        let start_ix = match all_annotations
            .binary_search_by(|probe| probe.range().end.cmp(&range.start, &buffer))
        {
            Ok(ix) | Err(ix) => ix,
        };
        let end_ix = match all_annotations
            .binary_search_by(|probe| probe.range().start.cmp(&range.end, &buffer))
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
        ensure_trailing_newline: bool,
        expand_result: bool,
        cx: &mut ModelContext<Self>,
    ) {
        self.reparse(cx);

        let insert_output_task = cx.spawn(|this, mut cx| {
            let command_range = command_range.clone();
            async move {
                let output = output.await;
                this.update(&mut cx, |this, cx| match output {
                    Ok(mut output) => {
                        // Ensure section ranges are valid.
                        for section in &mut output.sections {
                            section.range.start = section.range.start.min(output.text.len());
                            section.range.end = section.range.end.min(output.text.len());
                            while !output.text.is_char_boundary(section.range.start) {
                                section.range.start -= 1;
                            }
                            while !output.text.is_char_boundary(section.range.end) {
                                section.range.end += 1;
                            }
                        }

                        // Ensure there is a newline after the last section.
                        if ensure_trailing_newline {
                            let has_newline_after_last_section =
                                output.sections.last().map_or(false, |last_section| {
                                    output.text[last_section.range.end..].ends_with('\n')
                                });
                            if !has_newline_after_last_section {
                                output.text.push('\n');
                            }
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
                                    expand_result,
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

    fn get_last_valid_message_id(&self, cx: &ModelContext<Self>) -> Option<MessageId> {
        self.message_anchors.iter().rev().find_map(|message| {
            message
                .start
                .is_valid(self.buffer.read(cx))
                .then_some(message.id)
        })
    }

    pub fn assist(&mut self, cx: &mut ModelContext<Self>) -> Option<MessageAnchor> {
        let provider = LanguageModelRegistry::read_global(cx).active_provider()?;
        let model = LanguageModelRegistry::read_global(cx).active_model()?;
        let last_message_id = self.get_last_valid_message_id(cx)?;

        if !provider.is_authenticated(cx) {
            log::info!("completion provider has no credentials");
            return None;
        }
        // Compute which messages to cache, including the last one.
        self.mark_cache_anchors(&model.cache_configuration(), false, cx);

        let request = self.to_completion_request(cx);
        let assistant_message = self
            .insert_message_after(last_message_id, Role::Assistant, MessageStatus::Pending, cx)
            .unwrap();
            
        // Store model information in the message's metadata
        if let Some(metadata) = self.messages_metadata.get_mut(&assistant_message.id) {
            metadata.model_info = Some(ModelInfo {
                model_id: model.id().0.to_string(),
                model_name: model.name().0.to_string(),
                provider_id: provider.id().0.to_string(),
            });
        }

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
                            this.buffer.update(cx, |buffer, cx| {
                                let message_old_end_offset = this.message_anchors[message_ix + 1..]
                                    .iter()
                                    .find(|message| message.start.is_valid(buffer))
                                    .map_or(buffer.len(), |message| {
                                        message.start.to_offset(buffer).saturating_sub(1)
                                    });
                                buffer.edit(
                                    [(message_old_end_offset..message_old_end_offset, chunk)],
                                    None,
                                    cx,
                                );
                            });

                            cx.emit(ContextEvent::StreamedCompletion);

                            Some(())
                        })?;
                        smol::future::yield_now().await;
                    }
                    this.update(&mut cx, |this, cx| {
                        this.pending_completions
                            .retain(|completion| completion.id != pending_completion_id);
                        this.summarize(false, cx);
                        this.update_cache_status_for_completion(cx);
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
            .filter_map(|message| message.to_request_message(&buffer))
            .collect();

        LanguageModelRequest {
            messages: request_messages,
            tools: Vec::new(),
            stop: Vec::new(),
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
        for id in &ids {
            if let Some(metadata) = self.messages_metadata.get(id) {
                let role = metadata.role.cycle();
                self.update_metadata(*id, cx, |metadata| metadata.role = role);
            }
        }

        self.message_roles_updated(ids, cx);
    }

    fn message_roles_updated(&mut self, ids: HashSet<MessageId>, cx: &mut ModelContext<Self>) {
        let mut ranges = Vec::new();
        for message in self.messages(cx) {
            if ids.contains(&message.id) {
                ranges.push(message.anchor_range.clone());
            }
        }

        let buffer = self.buffer.read(cx).text_snapshot();
        let mut updated = Vec::new();
        let mut removed = Vec::new();
        for range in ranges {
            self.reparse_workflow_steps_in_range(range, &buffer, &mut updated, &mut removed, cx);
        }

        if !updated.is_empty() || !removed.is_empty() {
            cx.emit(ContextEvent::WorkflowStepsUpdated { removed, updated })
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

    pub fn insert_message_after(
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
                cache: None,
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
                cache: None,
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
                        cache: None,
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
                .filter_map(|message| message.to_request_message(self.buffer.read(cx)))
                .chain(Some(LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![
                        "Summarize the context into a short title without punctuation.".into(),
                    ],
                    cache: false,
                }));
            let request = LanguageModelRequest {
                messages: messages.collect(),
                tools: Vec::new(),
                stop: Vec::new(),
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

    fn messages_from_anchors<'a>(
        &'a self,
        message_anchors: impl Iterator<Item = &'a MessageAnchor> + 'a,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = Message> {
        let buffer = self.buffer.read(cx);
        let messages = message_anchors.enumerate();
        let images = self.image_anchors.iter();

        Self::messages_from_iters(buffer, &self.messages_metadata, messages, images)
    }

    pub fn messages<'a>(&'a self, cx: &'a AppContext) -> impl 'a + Iterator<Item = Message> {
        self.messages_from_anchors(self.message_anchors.iter(), cx)
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
                    anchor_range: message_anchor.start..message_end_anchor,
                    id: message_anchor.id,
                    role: metadata.role,
                    status: metadata.status.clone(),
                    cache: metadata.cache.clone(),
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
                        cache: None,
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
                    cache: None,
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
                            cache: None,
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
