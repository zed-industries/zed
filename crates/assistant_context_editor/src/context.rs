#[cfg(test)]
mod context_tests;

use crate::patch::{AssistantEdit, AssistantPatch, AssistantPatchStatus};
use anyhow::{Context as _, Result, anyhow};
use assistant_slash_command::{
    SlashCommandContent, SlashCommandEvent, SlashCommandLine, SlashCommandOutputSection,
    SlashCommandResult, SlashCommandWorkingSet,
};
use assistant_slash_commands::FileCommandMetadata;
use client::{self, proto, telemetry::Telemetry};
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use fs::{Fs, RemoveOptions};
use futures::{FutureExt, StreamExt, future::Shared};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, RenderImage, SharedString, Subscription,
    Task,
};
use language::{AnchorRangeExt, Bias, Buffer, LanguageRegistry, OffsetRangeExt, Point, ToOffset};
use language_model::{
    LanguageModel, LanguageModelCacheConfiguration, LanguageModelCompletionEvent,
    LanguageModelImage, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolUseId, MaxMonthlySpendReachedError, MessageContent, PaymentRequiredError,
    Role, StopReason, report_assistant_event,
};
use open_ai::Model as OpenAiModel;
use paths::contexts_dir;
use project::Project;
use prompt_store::PromptBuilder;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    cmp::{Ordering, max},
    fmt::Debug,
    iter, mem,
    ops::Range,
    path::Path,
    str::FromStr as _,
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};
use text::{BufferSnapshot, ToPoint};
use ui::IconName;
use util::{ResultExt, TryFutureExt, post_inc};
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MessageId(pub clock::Lamport);

impl MessageId {
    pub fn as_u64(self) -> u64 {
        self.0.as_u64()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessageStatus {
    Pending,
    Done,
    Error(SharedString),
    Canceled,
}

impl MessageStatus {
    pub fn from_proto(status: proto::ContextMessageStatus) -> MessageStatus {
        match status.variant {
            Some(proto::context_message_status::Variant::Pending(_)) => MessageStatus::Pending,
            Some(proto::context_message_status::Variant::Done(_)) => MessageStatus::Done,
            Some(proto::context_message_status::Variant::Error(error)) => {
                MessageStatus::Error(error.message.into())
            }
            Some(proto::context_message_status::Variant::Canceled(_)) => MessageStatus::Canceled,
            None => MessageStatus::Pending,
        }
    }

    pub fn to_proto(&self) -> proto::ContextMessageStatus {
        match self {
            MessageStatus::Pending => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Pending(
                    proto::context_message_status::Pending {},
                )),
            },
            MessageStatus::Done => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Done(
                    proto::context_message_status::Done {},
                )),
            },
            MessageStatus::Error(message) => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Error(
                    proto::context_message_status::Error {
                        message: message.to_string(),
                    },
                )),
            },
            MessageStatus::Canceled => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Canceled(
                    proto::context_message_status::Canceled {},
                )),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestType {
    /// Request a normal chat response from the model.
    Chat,
    /// Add a preamble to the message, which tells the model to return a structured response that suggests edits.
    SuggestEdits,
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
    SlashCommandStarted {
        id: InvokedSlashCommandId,
        output_range: Range<language::Anchor>,
        name: String,
        version: clock::Global,
    },
    SlashCommandFinished {
        id: InvokedSlashCommandId,
        timestamp: clock::Lamport,
        error_message: Option<String>,
        version: clock::Global,
    },
    SlashCommandOutputSectionAdded {
        timestamp: clock::Lamport,
        section: SlashCommandOutputSection<language::Anchor>,
        version: clock::Global,
    },
    ThoughtProcessOutputSectionAdded {
        timestamp: clock::Lamport,
        section: ThoughtProcessOutputSection<language::Anchor>,
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
            proto::context_operation::Variant::SlashCommandStarted(message) => {
                Ok(Self::SlashCommandStarted {
                    id: InvokedSlashCommandId(language::proto::deserialize_timestamp(
                        message.id.context("invalid id")?,
                    )),
                    output_range: language::proto::deserialize_anchor_range(
                        message.output_range.context("invalid range")?,
                    )?,
                    name: message.name,
                    version: language::proto::deserialize_version(&message.version),
                })
            }
            proto::context_operation::Variant::SlashCommandOutputSectionAdded(message) => {
                let section = message.section.context("missing section")?;
                Ok(Self::SlashCommandOutputSectionAdded {
                    timestamp: language::proto::deserialize_timestamp(
                        message.timestamp.context("missing timestamp")?,
                    ),
                    section: SlashCommandOutputSection {
                        range: language::proto::deserialize_anchor_range(
                            section.range.context("invalid range")?,
                        )?,
                        icon: section.icon_name.parse()?,
                        label: section.label.into(),
                        metadata: section
                            .metadata
                            .and_then(|metadata| serde_json::from_str(&metadata).log_err()),
                    },
                    version: language::proto::deserialize_version(&message.version),
                })
            }
            proto::context_operation::Variant::SlashCommandCompleted(message) => {
                Ok(Self::SlashCommandFinished {
                    id: InvokedSlashCommandId(language::proto::deserialize_timestamp(
                        message.id.context("invalid id")?,
                    )),
                    timestamp: language::proto::deserialize_timestamp(
                        message.timestamp.context("missing timestamp")?,
                    ),
                    error_message: message.error_message,
                    version: language::proto::deserialize_version(&message.version),
                })
            }
            proto::context_operation::Variant::ThoughtProcessOutputSectionAdded(message) => {
                let section = message.section.context("missing section")?;
                Ok(Self::ThoughtProcessOutputSectionAdded {
                    timestamp: language::proto::deserialize_timestamp(
                        message.timestamp.context("missing timestamp")?,
                    ),
                    section: ThoughtProcessOutputSection {
                        range: language::proto::deserialize_anchor_range(
                            section.range.context("invalid range")?,
                        )?,
                    },
                    version: language::proto::deserialize_version(&message.version),
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
            Self::SlashCommandStarted {
                id,
                output_range,
                name,
                version,
            } => proto::ContextOperation {
                variant: Some(proto::context_operation::Variant::SlashCommandStarted(
                    proto::context_operation::SlashCommandStarted {
                        id: Some(language::proto::serialize_timestamp(id.0)),
                        output_range: Some(language::proto::serialize_anchor_range(
                            output_range.clone(),
                        )),
                        name: name.clone(),
                        version: language::proto::serialize_version(version),
                    },
                )),
            },
            Self::SlashCommandOutputSectionAdded {
                timestamp,
                section,
                version,
            } => proto::ContextOperation {
                variant: Some(
                    proto::context_operation::Variant::SlashCommandOutputSectionAdded(
                        proto::context_operation::SlashCommandOutputSectionAdded {
                            timestamp: Some(language::proto::serialize_timestamp(*timestamp)),
                            section: Some({
                                let icon_name: &'static str = section.icon.into();
                                proto::SlashCommandOutputSection {
                                    range: Some(language::proto::serialize_anchor_range(
                                        section.range.clone(),
                                    )),
                                    icon_name: icon_name.to_string(),
                                    label: section.label.to_string(),
                                    metadata: section.metadata.as_ref().and_then(|metadata| {
                                        serde_json::to_string(metadata).log_err()
                                    }),
                                }
                            }),
                            version: language::proto::serialize_version(version),
                        },
                    ),
                ),
            },
            Self::SlashCommandFinished {
                id,
                timestamp,
                error_message,
                version,
            } => proto::ContextOperation {
                variant: Some(proto::context_operation::Variant::SlashCommandCompleted(
                    proto::context_operation::SlashCommandCompleted {
                        id: Some(language::proto::serialize_timestamp(id.0)),
                        timestamp: Some(language::proto::serialize_timestamp(*timestamp)),
                        error_message: error_message.clone(),
                        version: language::proto::serialize_version(version),
                    },
                )),
            },
            Self::ThoughtProcessOutputSectionAdded {
                timestamp,
                section,
                version,
            } => proto::ContextOperation {
                variant: Some(
                    proto::context_operation::Variant::ThoughtProcessOutputSectionAdded(
                        proto::context_operation::ThoughtProcessOutputSectionAdded {
                            timestamp: Some(language::proto::serialize_timestamp(*timestamp)),
                            section: Some({
                                proto::ThoughtProcessOutputSection {
                                    range: Some(language::proto::serialize_anchor_range(
                                        section.range.clone(),
                                    )),
                                }
                            }),
                            version: language::proto::serialize_version(version),
                        },
                    ),
                ),
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
            Self::SlashCommandStarted { id, .. } => id.0,
            Self::SlashCommandOutputSectionAdded { timestamp, .. }
            | Self::SlashCommandFinished { timestamp, .. }
            | Self::ThoughtProcessOutputSectionAdded { timestamp, .. } => *timestamp,
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
            | Self::SlashCommandStarted { version, .. }
            | Self::SlashCommandOutputSectionAdded { version, .. }
            | Self::SlashCommandFinished { version, .. }
            | Self::ThoughtProcessOutputSectionAdded { version, .. } => version,
            Self::BufferOperation(_) => {
                panic!("reading the version of a buffer operation is not supported")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContextEvent {
    ShowAssistError(SharedString),
    ShowPaymentRequiredError,
    ShowMaxMonthlySpendReachedError,
    MessagesEdited,
    SummaryChanged,
    SummaryGenerated,
    StreamedCompletion,
    StartedThoughtProcess(Range<language::Anchor>),
    EndedThoughtProcess(language::Anchor),
    PatchesUpdated {
        removed: Vec<Range<language::Anchor>>,
        updated: Vec<Range<language::Anchor>>,
    },
    InvokedSlashCommandChanged {
        command_id: InvokedSlashCommandId,
    },
    ParsedSlashCommandsUpdated {
        removed: Vec<Range<language::Anchor>>,
        updated: Vec<ParsedSlashCommand>,
    },
    SlashCommandOutputSectionAdded {
        section: SlashCommandOutputSection<language::Anchor>,
    },
    Operation(ContextOperation),
}

#[derive(Clone, Default, Debug)]
pub struct ContextSummary {
    pub text: String,
    pub done: bool,
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
    pub timestamp: clock::Lamport,
    #[serde(skip)]
    pub cache: Option<MessageCacheMetadata>,
}

impl From<&Message> for MessageMetadata {
    fn from(message: &Message) -> Self {
        Self {
            role: message.role,
            status: message.status.clone(),
            timestamp: message.id.0,
            cache: message.cache.clone(),
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThoughtProcessOutputSection<T> {
    pub range: Range<T>,
}

impl ThoughtProcessOutputSection<language::Anchor> {
    pub fn is_valid(&self, buffer: &language::TextBuffer) -> bool {
        self.range.start.is_valid(buffer) && !self.range.to_offset(buffer).is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub offset_range: Range<usize>,
    pub index_range: Range<usize>,
    pub anchor_range: Range<language::Anchor>,
    pub id: MessageId,
    pub role: Role,
    pub status: MessageStatus,
    pub cache: Option<MessageCacheMetadata>,
}

#[derive(Debug, Clone)]
pub enum Content {
    Image {
        anchor: language::Anchor,
        image_id: u64,
        render_image: Arc<RenderImage>,
        image: Shared<Task<Option<LanguageModelImage>>>,
    },
}

impl Content {
    fn range(&self) -> Range<language::Anchor> {
        match self {
            Self::Image { anchor, .. } => *anchor..*anchor,
        }
    }

    fn cmp(&self, other: &Self, buffer: &BufferSnapshot) -> Ordering {
        let self_range = self.range();
        let other_range = other.range();
        if self_range.end.cmp(&other_range.start, buffer).is_lt() {
            Ordering::Less
        } else if self_range.start.cmp(&other_range.end, buffer).is_gt() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

struct PendingCompletion {
    id: usize,
    assistant_message_id: MessageId,
    _task: Task<()>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct InvokedSlashCommandId(clock::Lamport);

#[derive(Clone, Debug)]
pub struct XmlTag {
    pub kind: XmlTagKind,
    pub range: Range<text::Anchor>,
    pub is_open_tag: bool,
}

#[derive(Copy, Clone, Debug, strum::EnumString, PartialEq, Eq, strum::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum XmlTagKind {
    Patch,
    Title,
    Edit,
    Path,
    Description,
    OldText,
    NewText,
    Operation,
}

pub struct AssistantContext {
    id: ContextId,
    timestamp: clock::Lamport,
    version: clock::Global,
    pending_ops: Vec<ContextOperation>,
    operations: Vec<ContextOperation>,
    buffer: Entity<Buffer>,
    parsed_slash_commands: Vec<ParsedSlashCommand>,
    invoked_slash_commands: HashMap<InvokedSlashCommandId, InvokedSlashCommand>,
    edits_since_last_parse: language::Subscription,
    slash_commands: Arc<SlashCommandWorkingSet>,
    slash_command_output_sections: Vec<SlashCommandOutputSection<language::Anchor>>,
    thought_process_output_sections: Vec<ThoughtProcessOutputSection<language::Anchor>>,
    message_anchors: Vec<MessageAnchor>,
    contents: Vec<Content>,
    messages_metadata: HashMap<MessageId, MessageMetadata>,
    summary: Option<ContextSummary>,
    summary_task: Task<Option<()>>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    token_count: Option<usize>,
    pending_token_count: Task<Option<()>>,
    pending_save: Task<Result<()>>,
    pending_cache_warming_task: Task<Option<()>>,
    path: Option<Arc<Path>>,
    _subscriptions: Vec<Subscription>,
    telemetry: Option<Arc<Telemetry>>,
    language_registry: Arc<LanguageRegistry>,
    patches: Vec<AssistantPatch>,
    xml_tags: Vec<XmlTag>,
    project: Option<Entity<Project>>,
    prompt_builder: Arc<PromptBuilder>,
}

trait ContextAnnotation {
    fn range(&self) -> &Range<language::Anchor>;
}

impl ContextAnnotation for ParsedSlashCommand {
    fn range(&self) -> &Range<language::Anchor> {
        &self.source_range
    }
}

impl ContextAnnotation for AssistantPatch {
    fn range(&self) -> &Range<language::Anchor> {
        &self.range
    }
}

impl ContextAnnotation for XmlTag {
    fn range(&self) -> &Range<language::Anchor> {
        &self.range
    }
}

impl EventEmitter<ContextEvent> for AssistantContext {}

impl AssistantContext {
    pub fn local(
        language_registry: Arc<LanguageRegistry>,
        project: Option<Entity<Project>>,
        telemetry: Option<Arc<Telemetry>>,
        prompt_builder: Arc<PromptBuilder>,
        slash_commands: Arc<SlashCommandWorkingSet>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(
            ContextId::new(),
            ReplicaId::default(),
            language::Capability::ReadWrite,
            language_registry,
            prompt_builder,
            slash_commands,
            project,
            telemetry,
            cx,
        )
    }

    pub fn new(
        id: ContextId,
        replica_id: ReplicaId,
        capability: language::Capability,
        language_registry: Arc<LanguageRegistry>,
        prompt_builder: Arc<PromptBuilder>,
        slash_commands: Arc<SlashCommandWorkingSet>,
        project: Option<Entity<Project>>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = cx.new(|_cx| {
            let buffer = Buffer::remote(
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
            contents: Default::default(),
            messages_metadata: Default::default(),
            parsed_slash_commands: Vec::new(),
            invoked_slash_commands: HashMap::default(),
            slash_command_output_sections: Vec::new(),
            thought_process_output_sections: Vec::new(),
            edits_since_last_parse: edits_since_last_slash_command_parse,
            summary: None,
            summary_task: Task::ready(None),
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
            slash_commands,
            patches: Vec::new(),
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

    pub(crate) fn serialize(&self, cx: &App) -> SavedContext {
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
                    if section.is_valid(buffer) {
                        let range = section.range.to_offset(buffer);
                        Some(assistant_slash_command::SlashCommandOutputSection {
                            range,
                            icon: section.icon,
                            label: section.label.clone(),
                            metadata: section.metadata.clone(),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
            thought_process_output_sections: self
                .thought_process_output_sections
                .iter()
                .filter_map(|section| {
                    if section.is_valid(buffer) {
                        let range = section.range.to_offset(buffer);
                        Some(ThoughtProcessOutputSection { range })
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    pub fn deserialize(
        saved_context: SavedContext,
        path: Arc<Path>,
        language_registry: Arc<LanguageRegistry>,
        prompt_builder: Arc<PromptBuilder>,
        slash_commands: Arc<SlashCommandWorkingSet>,
        project: Option<Entity<Project>>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let id = saved_context.id.clone().unwrap_or_else(ContextId::new);
        let mut this = Self::new(
            id,
            ReplicaId::default(),
            language::Capability::ReadWrite,
            language_registry,
            prompt_builder,
            slash_commands,
            project,
            telemetry,
            cx,
        );
        this.path = Some(path);
        this.buffer.update(cx, |buffer, cx| {
            buffer.set_text(saved_context.text.as_str(), cx)
        });
        let operations = saved_context.into_ops(&this.buffer, cx);
        this.apply_ops(operations, cx);
        this
    }

    pub fn id(&self) -> &ContextId {
        &self.id
    }

    pub fn replica_id(&self) -> ReplicaId {
        self.timestamp.replica_id
    }

    pub fn version(&self, cx: &App) -> ContextVersion {
        ContextVersion {
            context: self.version.clone(),
            buffer: self.buffer.read(cx).version(),
        }
    }

    pub fn slash_commands(&self) -> &Arc<SlashCommandWorkingSet> {
        &self.slash_commands
    }

    pub fn set_capability(&mut self, capability: language::Capability, cx: &mut Context<Self>) {
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
        cx: &App,
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

        cx.background_spawn(async move {
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
        cx: &mut Context<Self>,
    ) {
        let mut buffer_ops = Vec::new();
        for op in ops {
            match op {
                ContextOperation::BufferOperation(buffer_op) => buffer_ops.push(buffer_op),
                op @ _ => self.pending_ops.push(op),
            }
        }
        self.buffer
            .update(cx, |buffer, cx| buffer.apply_ops(buffer_ops, cx));
        self.flush_ops(cx);
    }

    fn flush_ops(&mut self, cx: &mut Context<AssistantContext>) {
        let mut changed_messages = HashSet::default();
        let mut summary_generated = false;

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
                        summary_generated = true;
                    }
                }
                ContextOperation::SlashCommandStarted {
                    id,
                    output_range,
                    name,
                    ..
                } => {
                    self.invoked_slash_commands.insert(
                        id,
                        InvokedSlashCommand {
                            name: name.into(),
                            range: output_range,
                            run_commands_in_ranges: Vec::new(),
                            status: InvokedSlashCommandStatus::Running(Task::ready(())),
                            transaction: None,
                            timestamp: id.0,
                        },
                    );
                    cx.emit(ContextEvent::InvokedSlashCommandChanged { command_id: id });
                }
                ContextOperation::SlashCommandOutputSectionAdded { section, .. } => {
                    let buffer = self.buffer.read(cx);
                    if let Err(ix) = self
                        .slash_command_output_sections
                        .binary_search_by(|probe| probe.range.cmp(&section.range, buffer))
                    {
                        self.slash_command_output_sections
                            .insert(ix, section.clone());
                        cx.emit(ContextEvent::SlashCommandOutputSectionAdded { section });
                    }
                }
                ContextOperation::ThoughtProcessOutputSectionAdded { section, .. } => {
                    let buffer = self.buffer.read(cx);
                    if let Err(ix) = self
                        .thought_process_output_sections
                        .binary_search_by(|probe| probe.range.cmp(&section.range, buffer))
                    {
                        self.thought_process_output_sections
                            .insert(ix, section.clone());
                    }
                }
                ContextOperation::SlashCommandFinished {
                    id,
                    error_message,
                    timestamp,
                    ..
                } => {
                    if let Some(slash_command) = self.invoked_slash_commands.get_mut(&id) {
                        if timestamp > slash_command.timestamp {
                            slash_command.timestamp = timestamp;
                            match error_message {
                                Some(message) => {
                                    slash_command.status =
                                        InvokedSlashCommandStatus::Error(message.into());
                                }
                                None => {
                                    slash_command.status = InvokedSlashCommandStatus::Finished;
                                }
                            }
                            cx.emit(ContextEvent::InvokedSlashCommandChanged { command_id: id });
                        }
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

        if summary_generated {
            cx.emit(ContextEvent::SummaryChanged);
            cx.emit(ContextEvent::SummaryGenerated);
            cx.notify();
        }
    }

    fn can_apply_op(&self, op: &ContextOperation, cx: &App) -> bool {
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
            ContextOperation::SlashCommandStarted { output_range, .. } => {
                self.has_received_operations_for_anchor_range(output_range.clone(), cx)
            }
            ContextOperation::SlashCommandOutputSectionAdded { section, .. } => {
                self.has_received_operations_for_anchor_range(section.range.clone(), cx)
            }
            ContextOperation::ThoughtProcessOutputSectionAdded { section, .. } => {
                self.has_received_operations_for_anchor_range(section.range.clone(), cx)
            }
            ContextOperation::SlashCommandFinished { .. } => true,
            ContextOperation::BufferOperation(_) => {
                panic!("buffer operations should always be applied")
            }
        }
    }

    fn has_received_operations_for_anchor_range(
        &self,
        range: Range<text::Anchor>,
        cx: &App,
    ) -> bool {
        let version = &self.buffer.read(cx).version;
        let observed_start = range.start == language::Anchor::MIN
            || range.start == language::Anchor::MAX
            || version.observed(range.start.timestamp);
        let observed_end = range.end == language::Anchor::MIN
            || range.end == language::Anchor::MAX
            || version.observed(range.end.timestamp);
        observed_start && observed_end
    }

    fn push_op(&mut self, op: ContextOperation, cx: &mut Context<Self>) {
        self.operations.push(op.clone());
        cx.emit(ContextEvent::Operation(op));
    }

    pub fn buffer(&self) -> &Entity<Buffer> {
        &self.buffer
    }

    pub fn language_registry(&self) -> Arc<LanguageRegistry> {
        self.language_registry.clone()
    }

    pub fn project(&self) -> Option<Entity<Project>> {
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

    pub fn patch_containing(&self, position: Point, cx: &App) -> Option<&AssistantPatch> {
        let buffer = self.buffer.read(cx);
        let index = self.patches.binary_search_by(|patch| {
            let patch_range = patch.range.to_point(&buffer);
            if position < patch_range.start {
                Ordering::Greater
            } else if position > patch_range.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        if let Ok(ix) = index {
            Some(&self.patches[ix])
        } else {
            None
        }
    }

    pub fn patch_ranges(&self) -> impl Iterator<Item = Range<language::Anchor>> + '_ {
        self.patches.iter().map(|patch| patch.range.clone())
    }

    pub fn patch_for_range(
        &self,
        range: &Range<language::Anchor>,
        cx: &App,
    ) -> Option<&AssistantPatch> {
        let buffer = self.buffer.read(cx);
        let index = self.patch_index_for_range(range, buffer).ok()?;
        Some(&self.patches[index])
    }

    fn patch_index_for_range(
        &self,
        tagged_range: &Range<text::Anchor>,
        buffer: &text::BufferSnapshot,
    ) -> Result<usize, usize> {
        self.patches
            .binary_search_by(|probe| probe.range.cmp(&tagged_range, buffer))
    }

    pub fn parsed_slash_commands(&self) -> &[ParsedSlashCommand] {
        &self.parsed_slash_commands
    }

    pub fn invoked_slash_command(
        &self,
        command_id: &InvokedSlashCommandId,
    ) -> Option<&InvokedSlashCommand> {
        self.invoked_slash_commands.get(command_id)
    }

    pub fn slash_command_output_sections(&self) -> &[SlashCommandOutputSection<language::Anchor>] {
        &self.slash_command_output_sections
    }

    pub fn thought_process_output_sections(
        &self,
    ) -> &[ThoughtProcessOutputSection<language::Anchor>] {
        &self.thought_process_output_sections
    }

    pub fn contains_files(&self, cx: &App) -> bool {
        let buffer = self.buffer.read(cx);
        self.slash_command_output_sections.iter().any(|section| {
            section.is_valid(buffer)
                && section
                    .metadata
                    .as_ref()
                    .and_then(|metadata| {
                        serde_json::from_value::<FileCommandMetadata>(metadata.clone()).ok()
                    })
                    .is_some()
        })
    }

    fn set_language(&mut self, cx: &mut Context<Self>) {
        let markdown = self.language_registry.language_for_name("Markdown");
        cx.spawn(async move |this, cx| {
            let markdown = markdown.await?;
            this.update(cx, |this, cx| {
                this.buffer
                    .update(cx, |buffer, cx| buffer.set_language(Some(markdown), cx));
            })
        })
        .detach_and_log_err(cx);
    }

    fn handle_buffer_event(
        &mut self,
        _: Entity<Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            language::BufferEvent::Operation {
                operation,
                is_local: true,
            } => cx.emit(ContextEvent::Operation(ContextOperation::BufferOperation(
                operation.clone(),
            ))),
            language::BufferEvent::Edited => {
                self.count_remaining_tokens(cx);
                self.reparse(cx);
                cx.emit(ContextEvent::MessagesEdited);
            }
            _ => {}
        }
    }

    pub fn token_count(&self) -> Option<usize> {
        self.token_count
    }

    pub(crate) fn count_remaining_tokens(&mut self, cx: &mut Context<Self>) {
        // Assume it will be a Chat request, even though that takes fewer tokens (and risks going over the limit),
        // because otherwise you see in the UI that your empty message has a bunch of tokens already used.
        let request = self.to_completion_request(RequestType::Chat, cx);
        let Some(model) = LanguageModelRegistry::read_global(cx).default_model() else {
            return;
        };
        let debounce = self.token_count.is_some();
        self.pending_token_count = cx.spawn(async move |this, cx| {
            async move {
                if debounce {
                    cx.background_executor()
                        .timer(Duration::from_millis(200))
                        .await;
                }

                let token_count = cx
                    .update(|cx| model.model.count_tokens(request, cx))?
                    .await?;
                this.update(cx, |this, cx| {
                    this.token_count = Some(token_count);
                    this.start_cache_warming(&model.model, cx);
                    cx.notify()
                })
            }
            .log_err()
            .await
        });
    }

    pub fn mark_cache_anchors(
        &mut self,
        cache_configuration: &Option<LanguageModelCacheConfiguration>,
        speculative: bool,
        cx: &mut Context<Self>,
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

    fn start_cache_warming(&mut self, model: &Arc<dyn LanguageModel>, cx: &mut Context<Self>) {
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
            let mut req = self.to_completion_request(RequestType::Chat, cx);
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
        self.pending_cache_warming_task = cx.spawn(async move |this, cx| {
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
                this.update(cx, |this, cx| {
                    this.update_cache_status_for_completion(cx);
                })
                .ok();
                anyhow::Ok(())
            }
            .log_err()
            .await
        });
    }

    pub fn update_cache_status_for_completion(&mut self, cx: &mut Context<Self>) {
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

    pub fn reparse(&mut self, cx: &mut Context<Self>) {
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

        let mut removed_parsed_slash_command_ranges = Vec::new();
        let mut updated_parsed_slash_commands = Vec::new();
        let mut removed_patches = Vec::new();
        let mut updated_patches = Vec::new();
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
                &mut updated_parsed_slash_commands,
                &mut removed_parsed_slash_command_ranges,
                cx,
            );
            self.invalidate_pending_slash_commands(&buffer, cx);
            self.reparse_patches_in_range(
                start..end,
                &buffer,
                &mut updated_patches,
                &mut removed_patches,
                cx,
            );
        }

        if !updated_parsed_slash_commands.is_empty()
            || !removed_parsed_slash_command_ranges.is_empty()
        {
            cx.emit(ContextEvent::ParsedSlashCommandsUpdated {
                removed: removed_parsed_slash_command_ranges,
                updated: updated_parsed_slash_commands,
            });
        }

        if !updated_patches.is_empty() || !removed_patches.is_empty() {
            cx.emit(ContextEvent::PatchesUpdated {
                removed: removed_patches,
                updated: updated_patches,
            });
        }
    }

    fn reparse_slash_commands_in_range(
        &mut self,
        range: Range<text::Anchor>,
        buffer: &BufferSnapshot,
        updated: &mut Vec<ParsedSlashCommand>,
        removed: &mut Vec<Range<text::Anchor>>,
        cx: &App,
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
                if let Some(command) = self.slash_commands.command(name, cx) {
                    if !command.requires_argument() || !arguments.is_empty() {
                        let start_ix = offset + command_line.name.start - 1;
                        let end_ix = offset
                            + command_line
                                .arguments
                                .last()
                                .map_or(command_line.name.end, |argument| argument.end);
                        let source_range =
                            buffer.anchor_after(start_ix)..buffer.anchor_after(end_ix);
                        let pending_command = ParsedSlashCommand {
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

        let removed_commands = self.parsed_slash_commands.splice(old_range, new_commands);
        removed.extend(removed_commands.map(|command| command.source_range));
    }

    fn invalidate_pending_slash_commands(
        &mut self,
        buffer: &BufferSnapshot,
        cx: &mut Context<Self>,
    ) {
        let mut invalidated_command_ids = Vec::new();
        for (&command_id, command) in self.invoked_slash_commands.iter_mut() {
            if !matches!(command.status, InvokedSlashCommandStatus::Finished)
                && (!command.range.start.is_valid(buffer) || !command.range.end.is_valid(buffer))
            {
                command.status = InvokedSlashCommandStatus::Finished;
                cx.emit(ContextEvent::InvokedSlashCommandChanged { command_id });
                invalidated_command_ids.push(command_id);
            }
        }

        for command_id in invalidated_command_ids {
            let version = self.version.clone();
            let timestamp = self.next_timestamp();
            self.push_op(
                ContextOperation::SlashCommandFinished {
                    id: command_id,
                    timestamp,
                    error_message: None,
                    version: version.clone(),
                },
                cx,
            );
        }
    }

    fn reparse_patches_in_range(
        &mut self,
        range: Range<text::Anchor>,
        buffer: &BufferSnapshot,
        updated: &mut Vec<Range<text::Anchor>>,
        removed: &mut Vec<Range<text::Anchor>>,
        cx: &mut Context<Self>,
    ) {
        // Rebuild the XML tags in the edited range.
        let intersecting_tags_range =
            self.indices_intersecting_buffer_range(&self.xml_tags, range.clone(), cx);
        let new_tags = self.parse_xml_tags_in_range(buffer, range.clone(), cx);
        self.xml_tags
            .splice(intersecting_tags_range.clone(), new_tags);

        // Find which patches intersect the changed range.
        let intersecting_patches_range =
            self.indices_intersecting_buffer_range(&self.patches, range.clone(), cx);

        // Reparse all tags after the last unchanged patch before the change.
        let mut tags_start_ix = 0;
        if let Some(preceding_unchanged_patch) =
            self.patches[..intersecting_patches_range.start].last()
        {
            tags_start_ix = match self.xml_tags.binary_search_by(|tag| {
                tag.range
                    .start
                    .cmp(&preceding_unchanged_patch.range.end, buffer)
                    .then(Ordering::Less)
            }) {
                Ok(ix) | Err(ix) => ix,
            };
        }

        // Rebuild the patches in the range.
        let new_patches = self.parse_patches(tags_start_ix, range.end, buffer, cx);
        updated.extend(new_patches.iter().map(|patch| patch.range.clone()));
        let removed_patches = self.patches.splice(intersecting_patches_range, new_patches);
        removed.extend(
            removed_patches
                .map(|patch| patch.range)
                .filter(|range| !updated.contains(&range)),
        );
    }

    fn parse_xml_tags_in_range(
        &self,
        buffer: &BufferSnapshot,
        range: Range<text::Anchor>,
        cx: &App,
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

    fn parse_patches(
        &mut self,
        tags_start_ix: usize,
        buffer_end: text::Anchor,
        buffer: &BufferSnapshot,
        cx: &App,
    ) -> Vec<AssistantPatch> {
        let mut new_patches = Vec::new();
        let mut pending_patch = None;
        let mut patch_tag_depth = 0;
        let mut tags = self.xml_tags[tags_start_ix..].iter().peekable();
        'tags: while let Some(tag) = tags.next() {
            if tag.range.start.cmp(&buffer_end, buffer).is_gt() && patch_tag_depth == 0 {
                break;
            }

            if tag.kind == XmlTagKind::Patch && tag.is_open_tag {
                patch_tag_depth += 1;
                let patch_start = tag.range.start;
                let mut edits = Vec::<Result<AssistantEdit>>::new();
                let mut patch = AssistantPatch {
                    range: patch_start..patch_start,
                    title: String::new().into(),
                    edits: Default::default(),
                    status: crate::AssistantPatchStatus::Pending,
                };

                while let Some(tag) = tags.next() {
                    if tag.kind == XmlTagKind::Patch && !tag.is_open_tag {
                        patch_tag_depth -= 1;
                        if patch_tag_depth == 0 {
                            patch.range.end = tag.range.end;

                            // Include the line immediately after this <patch> tag if it's empty.
                            let patch_end_offset = patch.range.end.to_offset(buffer);
                            let mut patch_end_chars = buffer.chars_at(patch_end_offset);
                            if patch_end_chars.next() == Some('\n')
                                && patch_end_chars.next().map_or(true, |ch| ch == '\n')
                            {
                                let messages = self.messages_for_offsets(
                                    [patch_end_offset, patch_end_offset + 1],
                                    cx,
                                );
                                if messages.len() == 1 {
                                    patch.range.end = buffer.anchor_before(patch_end_offset + 1);
                                }
                            }

                            edits.sort_unstable_by(|a, b| {
                                if let (Ok(a), Ok(b)) = (a, b) {
                                    a.path.cmp(&b.path)
                                } else {
                                    Ordering::Equal
                                }
                            });
                            patch.edits = edits.into();
                            patch.status = AssistantPatchStatus::Ready;
                            new_patches.push(patch);
                            continue 'tags;
                        }
                    }

                    if tag.kind == XmlTagKind::Title && tag.is_open_tag {
                        let content_start = tag.range.end;
                        while let Some(tag) = tags.next() {
                            if tag.kind == XmlTagKind::Title && !tag.is_open_tag {
                                let content_end = tag.range.start;
                                patch.title =
                                    trimmed_text_in_range(buffer, content_start..content_end)
                                        .into();
                                break;
                            }
                        }
                    }

                    if tag.kind == XmlTagKind::Edit && tag.is_open_tag {
                        let mut path = None;
                        let mut old_text = None;
                        let mut new_text = None;
                        let mut operation = None;
                        let mut description = None;

                        while let Some(tag) = tags.next() {
                            if tag.kind == XmlTagKind::Edit && !tag.is_open_tag {
                                edits.push(AssistantEdit::new(
                                    path,
                                    operation,
                                    old_text,
                                    new_text,
                                    description,
                                ));
                                break;
                            }

                            if tag.is_open_tag
                                && [
                                    XmlTagKind::Path,
                                    XmlTagKind::OldText,
                                    XmlTagKind::NewText,
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
                                        let content = trimmed_text_in_range(
                                            buffer,
                                            content_start..content_end,
                                        );
                                        match kind {
                                            XmlTagKind::Path => path = Some(content),
                                            XmlTagKind::Operation => operation = Some(content),
                                            XmlTagKind::OldText => {
                                                old_text = Some(content).filter(|s| !s.is_empty())
                                            }
                                            XmlTagKind::NewText => {
                                                new_text = Some(content).filter(|s| !s.is_empty())
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

                patch.edits = edits.into();
                pending_patch = Some(patch);
            }
        }

        if let Some(mut pending_patch) = pending_patch {
            let patch_start = pending_patch.range.start.to_offset(buffer);
            if let Some(message) = self.message_for_offset(patch_start, cx) {
                if message.anchor_range.end == text::Anchor::MAX {
                    pending_patch.range.end = text::Anchor::MAX;
                } else {
                    let message_end = buffer.anchor_after(message.offset_range.end - 1);
                    pending_patch.range.end = message_end;
                }
            } else {
                pending_patch.range.end = text::Anchor::MAX;
            }

            new_patches.push(pending_patch);
        }

        new_patches
    }

    pub fn pending_command_for_position(
        &mut self,
        position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<&mut ParsedSlashCommand> {
        let buffer = self.buffer.read(cx);
        match self
            .parsed_slash_commands
            .binary_search_by(|probe| probe.source_range.end.cmp(&position, buffer))
        {
            Ok(ix) => Some(&mut self.parsed_slash_commands[ix]),
            Err(ix) => {
                let cmd = self.parsed_slash_commands.get_mut(ix)?;
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
        cx: &App,
    ) -> &[ParsedSlashCommand] {
        let range = self.pending_command_indices_for_range(range, cx);
        &self.parsed_slash_commands[range]
    }

    fn pending_command_indices_for_range(
        &self,
        range: Range<language::Anchor>,
        cx: &App,
    ) -> Range<usize> {
        self.indices_intersecting_buffer_range(&self.parsed_slash_commands, range, cx)
    }

    fn indices_intersecting_buffer_range<T: ContextAnnotation>(
        &self,
        all_annotations: &[T],
        range: Range<language::Anchor>,
        cx: &App,
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
        command_source_range: Range<language::Anchor>,
        name: &str,
        output: Task<SlashCommandResult>,
        ensure_trailing_newline: bool,
        cx: &mut Context<Self>,
    ) {
        let version = self.version.clone();
        let command_id = InvokedSlashCommandId(self.next_timestamp());

        const PENDING_OUTPUT_END_MARKER: &str = "…";

        let (command_range, command_source_range, insert_position, first_transaction) =
            self.buffer.update(cx, |buffer, cx| {
                let command_source_range = command_source_range.to_offset(buffer);
                let mut insertion = format!("\n{PENDING_OUTPUT_END_MARKER}");
                if ensure_trailing_newline {
                    insertion.push('\n');
                }

                buffer.finalize_last_transaction();
                buffer.start_transaction();
                buffer.edit(
                    [(
                        command_source_range.end..command_source_range.end,
                        insertion,
                    )],
                    None,
                    cx,
                );
                let first_transaction = buffer.end_transaction(cx).unwrap();
                buffer.finalize_last_transaction();

                let insert_position = buffer.anchor_after(command_source_range.end + 1);
                let command_range = buffer.anchor_after(command_source_range.start)
                    ..buffer.anchor_before(
                        command_source_range.end + 1 + PENDING_OUTPUT_END_MARKER.len(),
                    );
                let command_source_range = buffer.anchor_before(command_source_range.start)
                    ..buffer.anchor_before(command_source_range.end + 1);
                (
                    command_range,
                    command_source_range,
                    insert_position,
                    first_transaction,
                )
            });
        self.reparse(cx);

        let insert_output_task = cx.spawn(async move |this, cx| {
            let run_command = async {
                let mut stream = output.await?;

                struct PendingSection {
                    start: language::Anchor,
                    icon: IconName,
                    label: SharedString,
                    metadata: Option<serde_json::Value>,
                }

                let mut pending_section_stack: Vec<PendingSection> = Vec::new();
                let mut last_role: Option<Role> = None;
                let mut last_section_range = None;

                while let Some(event) = stream.next().await {
                    let event = event?;
                    this.update(cx, |this, cx| {
                        this.buffer.update(cx, |buffer, _cx| {
                            buffer.finalize_last_transaction();
                            buffer.start_transaction()
                        });

                        match event {
                            SlashCommandEvent::StartMessage {
                                role,
                                merge_same_roles,
                            } => {
                                if !merge_same_roles && Some(role) != last_role {
                                    let offset = this.buffer.read_with(cx, |buffer, _cx| {
                                        insert_position.to_offset(buffer)
                                    });
                                    this.insert_message_at_offset(
                                        offset,
                                        role,
                                        MessageStatus::Pending,
                                        cx,
                                    );
                                }

                                last_role = Some(role);
                            }
                            SlashCommandEvent::StartSection {
                                icon,
                                label,
                                metadata,
                            } => {
                                this.buffer.update(cx, |buffer, cx| {
                                    let insert_point = insert_position.to_point(buffer);
                                    if insert_point.column > 0 {
                                        buffer.edit([(insert_point..insert_point, "\n")], None, cx);
                                    }

                                    pending_section_stack.push(PendingSection {
                                        start: buffer.anchor_before(insert_position),
                                        icon,
                                        label,
                                        metadata,
                                    });
                                });
                            }
                            SlashCommandEvent::Content(SlashCommandContent::Text {
                                text,
                                run_commands_in_text,
                            }) => {
                                let start = this.buffer.read(cx).anchor_before(insert_position);

                                this.buffer.update(cx, |buffer, cx| {
                                    buffer.edit(
                                        [(insert_position..insert_position, text)],
                                        None,
                                        cx,
                                    )
                                });

                                let end = this.buffer.read(cx).anchor_before(insert_position);
                                if run_commands_in_text {
                                    if let Some(invoked_slash_command) =
                                        this.invoked_slash_commands.get_mut(&command_id)
                                    {
                                        invoked_slash_command
                                            .run_commands_in_ranges
                                            .push(start..end);
                                    }
                                }
                            }
                            SlashCommandEvent::EndSection => {
                                if let Some(pending_section) = pending_section_stack.pop() {
                                    let offset_range = (pending_section.start..insert_position)
                                        .to_offset(this.buffer.read(cx));
                                    if !offset_range.is_empty() {
                                        let range = this.buffer.update(cx, |buffer, _cx| {
                                            buffer.anchor_after(offset_range.start)
                                                ..buffer.anchor_before(offset_range.end)
                                        });
                                        this.insert_slash_command_output_section(
                                            SlashCommandOutputSection {
                                                range: range.clone(),
                                                icon: pending_section.icon,
                                                label: pending_section.label,
                                                metadata: pending_section.metadata,
                                            },
                                            cx,
                                        );
                                        last_section_range = Some(range);
                                    }
                                }
                            }
                        }

                        this.buffer.update(cx, |buffer, cx| {
                            if let Some(event_transaction) = buffer.end_transaction(cx) {
                                buffer.merge_transactions(event_transaction, first_transaction);
                            }
                        });
                    })?;
                }

                this.update(cx, |this, cx| {
                    this.buffer.update(cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();

                        let mut deletions = vec![(command_source_range.to_offset(buffer), "")];
                        let insert_position = insert_position.to_offset(buffer);
                        let command_range_end = command_range.end.to_offset(buffer);

                        if buffer.contains_str_at(insert_position, PENDING_OUTPUT_END_MARKER) {
                            deletions.push((
                                insert_position..insert_position + PENDING_OUTPUT_END_MARKER.len(),
                                "",
                            ));
                        }

                        if ensure_trailing_newline
                            && buffer.contains_str_at(command_range_end, "\n")
                        {
                            let newline_offset = insert_position.saturating_sub(1);
                            if buffer.contains_str_at(newline_offset, "\n")
                                && last_section_range.map_or(true, |last_section_range| {
                                    !last_section_range
                                        .to_offset(buffer)
                                        .contains(&newline_offset)
                                })
                            {
                                deletions.push((command_range_end..command_range_end + 1, ""));
                            }
                        }

                        buffer.edit(deletions, None, cx);

                        if let Some(deletion_transaction) = buffer.end_transaction(cx) {
                            buffer.merge_transactions(deletion_transaction, first_transaction);
                        }
                    });
                })?;

                debug_assert!(pending_section_stack.is_empty());

                anyhow::Ok(())
            };

            let command_result = run_command.await;

            this.update(cx, |this, cx| {
                let version = this.version.clone();
                let timestamp = this.next_timestamp();
                let Some(invoked_slash_command) = this.invoked_slash_commands.get_mut(&command_id)
                else {
                    return;
                };
                let mut error_message = None;
                match command_result {
                    Ok(()) => {
                        invoked_slash_command.status = InvokedSlashCommandStatus::Finished;
                    }
                    Err(error) => {
                        let message = error.to_string();
                        invoked_slash_command.status =
                            InvokedSlashCommandStatus::Error(message.clone().into());
                        error_message = Some(message);
                    }
                }

                cx.emit(ContextEvent::InvokedSlashCommandChanged { command_id });
                this.push_op(
                    ContextOperation::SlashCommandFinished {
                        id: command_id,
                        timestamp,
                        error_message,
                        version,
                    },
                    cx,
                );
            })
            .ok();
        });

        self.invoked_slash_commands.insert(
            command_id,
            InvokedSlashCommand {
                name: name.to_string().into(),
                range: command_range.clone(),
                run_commands_in_ranges: Vec::new(),
                status: InvokedSlashCommandStatus::Running(insert_output_task),
                transaction: Some(first_transaction),
                timestamp: command_id.0,
            },
        );
        cx.emit(ContextEvent::InvokedSlashCommandChanged { command_id });
        self.push_op(
            ContextOperation::SlashCommandStarted {
                id: command_id,
                output_range: command_range,
                name: name.to_string(),
                version,
            },
            cx,
        );
    }

    fn insert_slash_command_output_section(
        &mut self,
        section: SlashCommandOutputSection<language::Anchor>,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.buffer.read(cx);
        let insertion_ix = match self
            .slash_command_output_sections
            .binary_search_by(|probe| probe.range.cmp(&section.range, buffer))
        {
            Ok(ix) | Err(ix) => ix,
        };
        self.slash_command_output_sections
            .insert(insertion_ix, section.clone());
        cx.emit(ContextEvent::SlashCommandOutputSectionAdded {
            section: section.clone(),
        });
        let version = self.version.clone();
        let timestamp = self.next_timestamp();
        self.push_op(
            ContextOperation::SlashCommandOutputSectionAdded {
                timestamp,
                section,
                version,
            },
            cx,
        );
    }

    fn insert_thought_process_output_section(
        &mut self,
        section: ThoughtProcessOutputSection<language::Anchor>,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.buffer.read(cx);
        let insertion_ix = match self
            .thought_process_output_sections
            .binary_search_by(|probe| probe.range.cmp(&section.range, buffer))
        {
            Ok(ix) | Err(ix) => ix,
        };
        self.thought_process_output_sections
            .insert(insertion_ix, section.clone());
        // cx.emit(ContextEvent::ThoughtProcessOutputSectionAdded {
        //     section: section.clone(),
        // });
        let version = self.version.clone();
        let timestamp = self.next_timestamp();
        self.push_op(
            ContextOperation::ThoughtProcessOutputSectionAdded {
                timestamp,
                section,
                version,
            },
            cx,
        );
    }

    pub fn completion_provider_changed(&mut self, cx: &mut Context<Self>) {
        self.count_remaining_tokens(cx);
    }

    fn get_last_valid_message_id(&self, cx: &Context<Self>) -> Option<MessageId> {
        self.message_anchors.iter().rev().find_map(|message| {
            message
                .start
                .is_valid(self.buffer.read(cx))
                .then_some(message.id)
        })
    }

    pub fn assist(
        &mut self,
        request_type: RequestType,
        cx: &mut Context<Self>,
    ) -> Option<MessageAnchor> {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let model = model_registry.default_model()?;
        let last_message_id = self.get_last_valid_message_id(cx)?;

        if !model.provider.is_authenticated(cx) {
            log::info!("completion provider has no credentials");
            return None;
        }

        let model = model.model;

        // Compute which messages to cache, including the last one.
        self.mark_cache_anchors(&model.cache_configuration(), false, cx);

        let request = self.to_completion_request(request_type, cx);

        let assistant_message = self
            .insert_message_after(last_message_id, Role::Assistant, MessageStatus::Pending, cx)
            .unwrap();

        // Queue up the user's next reply.
        let user_message = self
            .insert_message_after(assistant_message.id, Role::User, MessageStatus::Done, cx)
            .unwrap();

        let pending_completion_id = post_inc(&mut self.completion_count);

        let task = cx.spawn({
            async move |this, cx| {
                let stream = model.stream_completion(request, &cx);
                let assistant_message_id = assistant_message.id;
                let mut response_latency = None;
                let stream_completion = async {
                    let request_start = Instant::now();
                    let mut events = stream.await?;
                    let mut stop_reason = StopReason::EndTurn;
                    let mut thought_process_stack = Vec::new();

                    const THOUGHT_PROCESS_START_MARKER: &str = "<think>\n";
                    const THOUGHT_PROCESS_END_MARKER: &str = "\n</think>";

                    while let Some(event) = events.next().await {
                        if response_latency.is_none() {
                            response_latency = Some(request_start.elapsed());
                        }
                        let event = event?;

                        let mut context_event = None;
                        let mut thought_process_output_section = None;

                        this.update(cx, |this, cx| {
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

                                match event {
                                    LanguageModelCompletionEvent::StartMessage { .. } => {}
                                    LanguageModelCompletionEvent::Stop(reason) => {
                                        stop_reason = reason;
                                    }
                                    LanguageModelCompletionEvent::Thinking { text: chunk, .. } => {
                                        if thought_process_stack.is_empty() {
                                            let start =
                                                buffer.anchor_before(message_old_end_offset);
                                            thought_process_stack.push(start);
                                            let chunk =
                                                format!("{THOUGHT_PROCESS_START_MARKER}{chunk}{THOUGHT_PROCESS_END_MARKER}");
                                            let chunk_len = chunk.len();
                                            buffer.edit(
                                                [(
                                                    message_old_end_offset..message_old_end_offset,
                                                    chunk,
                                                )],
                                                None,
                                                cx,
                                            );
                                            let end = buffer
                                                .anchor_before(message_old_end_offset + chunk_len);
                                            context_event = Some(
                                                ContextEvent::StartedThoughtProcess(start..end),
                                            );
                                        } else {
                                            // This ensures that all the thinking chunks are inserted inside the thinking tag
                                            let insertion_position =
                                                message_old_end_offset - THOUGHT_PROCESS_END_MARKER.len();
                                            buffer.edit(
                                                [(insertion_position..insertion_position, chunk)],
                                                None,
                                                cx,
                                            );
                                        }
                                    }
                                    LanguageModelCompletionEvent::Text(mut chunk) => {
                                        if let Some(start) = thought_process_stack.pop() {
                                            let end = buffer.anchor_before(message_old_end_offset);
                                            context_event =
                                                Some(ContextEvent::EndedThoughtProcess(end));
                                            thought_process_output_section =
                                                Some(ThoughtProcessOutputSection {
                                                    range: start..end,
                                                });
                                            chunk.insert_str(0, "\n\n");
                                        }

                                        buffer.edit(
                                            [(
                                                message_old_end_offset..message_old_end_offset,
                                                chunk,
                                            )],
                                            None,
                                            cx,
                                        );
                                    }
                                    LanguageModelCompletionEvent::ToolUse(_) => {}
                                    LanguageModelCompletionEvent::UsageUpdate(_) => {}
                                }
                            });

                            if let Some(section) = thought_process_output_section.take() {
                                this.insert_thought_process_output_section(section, cx);
                            }
                            if let Some(context_event) = context_event.take() {
                                cx.emit(context_event);
                            }

                            cx.emit(ContextEvent::StreamedCompletion);

                            Some(())
                        })?;
                        smol::future::yield_now().await;
                    }
                    this.update(cx, |this, cx| {
                        this.pending_completions
                            .retain(|completion| completion.id != pending_completion_id);
                        this.summarize(false, cx);
                        this.update_cache_status_for_completion(cx);
                    })?;

                    anyhow::Ok(stop_reason)
                };

                let result = stream_completion.await;

                this.update(cx, |this, cx| {
                    let error_message = if let Some(error) = result.as_ref().err() {
                        if error.is::<PaymentRequiredError>() {
                            cx.emit(ContextEvent::ShowPaymentRequiredError);
                            this.update_metadata(assistant_message_id, cx, |metadata| {
                                metadata.status = MessageStatus::Canceled;
                            });
                            Some(error.to_string())
                        } else if error.is::<MaxMonthlySpendReachedError>() {
                            cx.emit(ContextEvent::ShowMaxMonthlySpendReachedError);
                            this.update_metadata(assistant_message_id, cx, |metadata| {
                                metadata.status = MessageStatus::Canceled;
                            });
                            Some(error.to_string())
                        } else {
                            let error_message = error
                                .chain()
                                .map(|err| err.to_string())
                                .collect::<Vec<_>>()
                                .join("\n");
                            cx.emit(ContextEvent::ShowAssistError(SharedString::from(
                                error_message.clone(),
                            )));
                            this.update_metadata(assistant_message_id, cx, |metadata| {
                                metadata.status =
                                    MessageStatus::Error(SharedString::from(error_message.clone()));
                            });
                            Some(error_message)
                        }
                    } else {
                        this.update_metadata(assistant_message_id, cx, |metadata| {
                            metadata.status = MessageStatus::Done;
                        });
                        None
                    };

                    let language_name = this
                        .buffer
                        .read(cx)
                        .language()
                        .map(|language| language.name());
                    report_assistant_event(
                        AssistantEventData {
                            conversation_id: Some(this.id.0.clone()),
                            kind: AssistantKind::Panel,
                            phase: AssistantPhase::Response,
                            message_id: None,
                            model: model.telemetry_id(),
                            model_provider: model.provider_id().to_string(),
                            response_latency,
                            error_message,
                            language_name: language_name.map(|name| name.to_proto()),
                        },
                        this.telemetry.clone(),
                        cx.http_client(),
                        model.api_key(cx),
                        cx.background_executor(),
                    );

                    if let Ok(stop_reason) = result {
                        match stop_reason {
                            StopReason::ToolUse => {}
                            StopReason::EndTurn => {}
                            StopReason::MaxTokens => {}
                        }
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

    pub fn to_completion_request(
        &self,
        request_type: RequestType,
        cx: &App,
    ) -> LanguageModelRequest {
        let buffer = self.buffer.read(cx);

        let mut contents = self.contents(cx).peekable();

        fn collect_text_content(buffer: &Buffer, range: Range<usize>) -> Option<String> {
            let text: String = buffer.text_for_range(range.clone()).collect();
            if text.trim().is_empty() {
                None
            } else {
                Some(text)
            }
        }

        let mut completion_request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            messages: Vec::new(),
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
        };
        for message in self.messages(cx) {
            if message.status != MessageStatus::Done {
                continue;
            }

            let mut offset = message.offset_range.start;
            let mut request_message = LanguageModelRequestMessage {
                role: message.role,
                content: Vec::new(),
                cache: message
                    .cache
                    .as_ref()
                    .map_or(false, |cache| cache.is_anchor),
            };

            while let Some(content) = contents.peek() {
                if content
                    .range()
                    .end
                    .cmp(&message.anchor_range.end, buffer)
                    .is_lt()
                {
                    let content = contents.next().unwrap();
                    let range = content.range().to_offset(buffer);
                    request_message.content.extend(
                        collect_text_content(buffer, offset..range.start).map(MessageContent::Text),
                    );

                    match content {
                        Content::Image { image, .. } => {
                            if let Some(image) = image.clone().now_or_never().flatten() {
                                request_message
                                    .content
                                    .push(language_model::MessageContent::Image(image));
                            }
                        }
                    }

                    offset = range.end;
                } else {
                    break;
                }
            }

            request_message.content.extend(
                collect_text_content(buffer, offset..message.offset_range.end)
                    .map(MessageContent::Text),
            );

            if !request_message.contents_empty() {
                completion_request.messages.push(request_message);
            }
        }

        if let RequestType::SuggestEdits = request_type {
            if let Ok(preamble) = self.prompt_builder.generate_suggest_edits_prompt() {
                let last_elem_index = completion_request.messages.len();

                completion_request
                    .messages
                    .push(LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![MessageContent::Text(preamble)],
                        cache: false,
                    });

                // The preamble message should be sent right before the last actual user message.
                completion_request
                    .messages
                    .swap(last_elem_index, last_elem_index.saturating_sub(1));
            }
        }

        completion_request
    }

    pub fn cancel_last_assist(&mut self, cx: &mut Context<Self>) -> bool {
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

    pub fn cycle_message_roles(&mut self, ids: HashSet<MessageId>, cx: &mut Context<Self>) {
        for id in &ids {
            if let Some(metadata) = self.messages_metadata.get(id) {
                let role = metadata.role.cycle();
                self.update_metadata(*id, cx, |metadata| metadata.role = role);
            }
        }

        self.message_roles_updated(ids, cx);
    }

    fn message_roles_updated(&mut self, ids: HashSet<MessageId>, cx: &mut Context<Self>) {
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
            self.reparse_patches_in_range(range, &buffer, &mut updated, &mut removed, cx);
        }

        if !updated.is_empty() || !removed.is_empty() {
            cx.emit(ContextEvent::PatchesUpdated { removed, updated })
        }
    }

    pub fn update_metadata(
        &mut self,
        id: MessageId,
        cx: &mut Context<Self>,
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
        cx: &mut Context<Self>,
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

            let buffer = self.buffer.read(cx);
            let offset = self
                .message_anchors
                .get(next_message_ix)
                .map_or(buffer.len(), |message| {
                    buffer.clip_offset(message.start.to_offset(buffer) - 1, Bias::Left)
                });
            Some(self.insert_message_at_offset(offset, role, status, cx))
        } else {
            None
        }
    }

    fn insert_message_at_offset(
        &mut self,
        offset: usize,
        role: Role,
        status: MessageStatus,
        cx: &mut Context<Self>,
    ) -> MessageAnchor {
        let start = self.buffer.update(cx, |buffer, cx| {
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
        anchor
    }

    pub fn insert_content(&mut self, content: Content, cx: &mut Context<Self>) {
        let buffer = self.buffer.read(cx);
        let insertion_ix = match self
            .contents
            .binary_search_by(|probe| probe.cmp(&content, buffer))
        {
            Ok(ix) => {
                self.contents.remove(ix);
                ix
            }
            Err(ix) => ix,
        };
        self.contents.insert(insertion_ix, content);
        cx.emit(ContextEvent::MessagesEdited);
    }

    pub fn contents<'a>(&'a self, cx: &'a App) -> impl 'a + Iterator<Item = Content> {
        let buffer = self.buffer.read(cx);
        self.contents
            .iter()
            .filter(|content| {
                let range = content.range();
                range.start.is_valid(buffer) && range.end.is_valid(buffer)
            })
            .cloned()
    }

    pub fn split_message(
        &mut self,
        range: Range<usize>,
        cx: &mut Context<Self>,
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
        cx: &mut Context<Self>,
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

    pub fn summarize(&mut self, mut replace_old: bool, cx: &mut Context<Self>) {
        let Some(model) = LanguageModelRegistry::read_global(cx).default_model() else {
            return;
        };

        if replace_old || (self.message_anchors.len() >= 2 && self.summary.is_none()) {
            if !model.provider.is_authenticated(cx) {
                return;
            }

            let mut request = self.to_completion_request(RequestType::Chat, cx);
            request.messages.push(LanguageModelRequestMessage {
                role: Role::User,
                content: vec![
                    "Generate a concise 3-7 word title for this conversation, omitting punctuation. Go straight to the title, without any preamble and prefix like `Here's a concise suggestion:...` or `Title:`"
                        .into(),
                ],
                cache: false,
            });

            // If there is no summary, it is set with `done: false` so that "Loading Summary…" can
            // be displayed.
            if self.summary.is_none() {
                self.summary = Some(ContextSummary {
                    text: "".to_string(),
                    done: false,
                    timestamp: clock::Lamport::default(),
                });
                replace_old = true;
            }

            self.summary_task = cx.spawn(async move |this, cx| {
                async move {
                    let stream = model.model.stream_completion_text(request, &cx);
                    let mut messages = stream.await?;

                    let mut replaced = !replace_old;
                    while let Some(message) = messages.stream.next().await {
                        let text = message?;
                        let mut lines = text.lines();
                        this.update(cx, |this, cx| {
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
                            cx.emit(ContextEvent::SummaryGenerated);
                        })?;

                        // Stop if the LLM generated multiple lines.
                        if lines.next().is_some() {
                            break;
                        }
                    }

                    this.update(cx, |this, cx| {
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
                            cx.emit(ContextEvent::SummaryGenerated);
                        }
                    })?;

                    anyhow::Ok(())
                }
                .log_err()
                .await
            });
        }
    }

    fn message_for_offset(&self, offset: usize, cx: &App) -> Option<Message> {
        self.messages_for_offsets([offset], cx).pop()
    }

    pub fn messages_for_offsets(
        &self,
        offsets: impl IntoIterator<Item = usize>,
        cx: &App,
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
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = Message> {
        let buffer = self.buffer.read(cx);

        Self::messages_from_iters(buffer, &self.messages_metadata, message_anchors.enumerate())
    }

    pub fn messages<'a>(&'a self, cx: &'a App) -> impl 'a + Iterator<Item = Message> {
        self.messages_from_anchors(self.message_anchors.iter(), cx)
    }

    pub fn messages_from_iters<'a>(
        buffer: &'a Buffer,
        metadata: &'a HashMap<MessageId, MessageMetadata>,
        messages: impl Iterator<Item = (usize, &'a MessageAnchor)> + 'a,
    ) -> impl 'a + Iterator<Item = Message> {
        let mut messages = messages.peekable();

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

                return Some(Message {
                    index_range: start_ix..end_ix,
                    offset_range: message_start..message_end,
                    anchor_range: message_anchor.start..message_end_anchor,
                    id: message_anchor.id,
                    role: metadata.role,
                    status: metadata.status.clone(),
                    cache: metadata.cache.clone(),
                });
            }
            None
        })
    }

    pub fn save(
        &mut self,
        debounce: Option<Duration>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<AssistantContext>,
    ) {
        if self.replica_id() != ReplicaId::default() {
            // Prevent saving a remote context for now.
            return;
        }

        self.pending_save = cx.spawn(async move |this, cx| {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }

            let (old_path, summary) = this.read_with(cx, |this, _| {
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
                let context = this.read_with(cx, |this, cx| this.serialize(cx))?;
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
                    if new_path.as_path() != old_path.as_ref() {
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

                this.update(cx, |this, _| this.path = Some(new_path.into()))?;
            }

            Ok(())
        });
    }

    pub fn set_custom_summary(&mut self, custom_summary: String, cx: &mut Context<Self>) {
        let timestamp = self.next_timestamp();
        let summary = self.summary.get_or_insert(ContextSummary::default());
        summary.timestamp = timestamp;
        summary.done = true;
        summary.text = custom_summary;
        cx.emit(ContextEvent::SummaryChanged);
    }

    pub const DEFAULT_SUMMARY: SharedString = SharedString::new_static("New Text Thread");

    pub fn summary_or_default(&self) -> SharedString {
        self.summary
            .as_ref()
            .map(|summary| summary.text.clone().into())
            .unwrap_or(Self::DEFAULT_SUMMARY)
    }
}

fn trimmed_text_in_range(buffer: &BufferSnapshot, range: Range<text::Anchor>) -> String {
    let mut is_start = true;
    let mut content = buffer
        .text_for_range(range)
        .map(|mut chunk| {
            if is_start {
                chunk = chunk.trim_start_matches('\n');
                if !chunk.is_empty() {
                    is_start = false;
                }
            }
            chunk
        })
        .collect::<String>();
    content.truncate(content.trim_end().len());
    content
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
pub struct ParsedSlashCommand {
    pub name: String,
    pub arguments: SmallVec<[String; 3]>,
    pub status: PendingSlashCommandStatus,
    pub source_range: Range<language::Anchor>,
}

#[derive(Debug)]
pub struct InvokedSlashCommand {
    pub name: SharedString,
    pub range: Range<language::Anchor>,
    pub run_commands_in_ranges: Vec<Range<language::Anchor>>,
    pub status: InvokedSlashCommandStatus,
    pub transaction: Option<language::TransactionId>,
    timestamp: clock::Lamport,
}

#[derive(Debug)]
pub enum InvokedSlashCommandStatus {
    Running(Task<()>),
    Error(SharedString),
    Finished,
}

#[derive(Debug, Clone)]
pub enum PendingSlashCommandStatus {
    Idle,
    Running { _task: Shared<Task<()>> },
    Error(String),
}

#[derive(Debug, Clone)]
pub struct PendingToolUse {
    pub id: LanguageModelToolUseId,
    pub name: String,
    pub input: serde_json::Value,
    pub status: PendingToolUseStatus,
    pub source_range: Range<language::Anchor>,
}

#[derive(Debug, Clone)]
pub enum PendingToolUseStatus {
    Idle,
    Running { _task: Shared<Task<()>> },
    Error(String),
}

impl PendingToolUseStatus {
    pub fn is_idle(&self) -> bool {
        matches!(self, PendingToolUseStatus::Idle)
    }
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
    #[serde(default)]
    pub thought_process_output_sections: Vec<ThoughtProcessOutputSection<usize>>,
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
        buffer: &Entity<Buffer>,
        cx: &mut Context<AssistantContext>,
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

        let buffer = buffer.read(cx);
        for section in self.slash_command_output_sections {
            let timestamp = next_timestamp.tick();
            operations.push(ContextOperation::SlashCommandOutputSectionAdded {
                timestamp,
                section: SlashCommandOutputSection {
                    range: buffer.anchor_after(section.range.start)
                        ..buffer.anchor_before(section.range.end),
                    icon: section.icon,
                    label: section.label,
                    metadata: section.metadata,
                },
                version: version.clone(),
            });

            version.observe(timestamp);
        }

        for section in self.thought_process_output_sections {
            let timestamp = next_timestamp.tick();
            operations.push(ContextOperation::ThoughtProcessOutputSectionAdded {
                timestamp,
                section: ThoughtProcessOutputSection {
                    range: buffer.anchor_after(section.range.start)
                        ..buffer.anchor_before(section.range.end),
                },
                version: version.clone(),
            });

            version.observe(timestamp);
        }

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
                    })
                })
                .collect(),
            summary: self.summary,
            slash_command_output_sections: self.slash_command_output_sections,
            thought_process_output_sections: Vec::new(),
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

#[derive(Debug, Clone)]
pub struct SavedContextMetadata {
    pub title: String,
    pub path: Arc<Path>,
    pub mtime: chrono::DateTime<chrono::Local>,
}
