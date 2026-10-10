pub mod auth_methods;
pub mod commands;
pub mod compaction;
pub mod config_options;
mod connection;
pub mod content;
mod diff;
pub mod elicitation;
mod mention;
pub mod notices;
pub mod prompt_capabilities;
mod submission;
mod terminal;
pub use ::terminal::HeadlessTerminal;
use action_log::{ActionLog, ActionLogTelemetry};
use agent_client_protocol::schema::{MaybeUndefined, v1 as acp_v1, v2 as acp_v2};
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use collections::{HashSet, IndexMap};
pub use connection::*;
pub use diff::*;
use feature_flags::{AcpBetaFeatureFlag, FeatureFlagAppExt as _};
#[cfg(any(test, feature = "test-support"))]
use futures::future::BoxFuture;
use futures::{FutureExt, channel::oneshot};
use gpui::{
    ActivityGuard, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription,
    Task, WeakEntity,
};
use itertools::Itertools;
use language::language_settings::FormatOnSave;
use language::{
    Anchor, Buffer, BufferEditSource, BufferSnapshot, LanguageRegistry, Point, ToPoint, text_diff,
};
use markdown::{Markdown, MarkdownOptions};
pub use mention::*;
use project::lsp_store::{FormatTrigger, LspFormatTarget};
use project::{
    AgentLocation, Project,
    git_store::{GitStoreCheckpoint, GitStoreEvent, RepositoryEvent},
};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use settings::{Settings, SettingsStore};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Formatter, Write};
use std::ops::Range;
use std::process::ExitStatus;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{fmt::Display, mem, path::PathBuf, sync::Arc};
pub use submission::*;
use task::{Shell, ShellBuilder};
pub use terminal::*;
use text::Bias;
use ui::App;
use util::markdown::{MarkdownCodeBlock, MarkdownEscaped};
use util::path_list::PathList;
use util::{
    ResultExt, get_default_system_shell_preferring_bash,
    paths::{PathStyle, is_absolute},
};
use uuid::Uuid;

/// Returned when the model stops because it exhausted its output token budget.
#[derive(Debug)]
pub struct MaxOutputTokensError;

impl std::fmt::Display for MaxOutputTokensError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "output token limit reached")
    }
}

impl std::error::Error for MaxOutputTokensError {}

/// Legacy ACP metadata key used before tool calls had a dedicated name field.
pub const TOOL_NAME_META_KEY: &str = "tool_name";

/// Extracts a tool name from the legacy ACP metadata field.
pub fn tool_name_from_meta(meta: &Option<acp_v1::Meta>) -> Option<SharedString> {
    meta.as_ref()
        .and_then(|meta| meta.get(TOOL_NAME_META_KEY))
        .and_then(|value| value.as_str())
        .map(|name| SharedString::from(name.to_owned()))
}

/// Creates ACP metadata containing the legacy tool-name field.
pub fn meta_with_tool_name(tool_name: &str) -> acp_v1::Meta {
    acp_v1::Meta::from_iter([(TOOL_NAME_META_KEY.into(), tool_name.into())])
}

/// Key used in ACP `AvailableCommand` meta to record which source produced a
/// slash command, so the completion popup can group commands by category.
pub const COMMAND_CATEGORY_META_KEY: &str = "command_category";

/// The source category of a slash command, used to group commands in the
/// completion popup. Only the native Zed agent annotates its commands; commands
/// from external ACP agents carry no category and are grouped on their own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandCategory {
    /// Built-in Zed agent commands (e.g. `/compact`).
    Native,
    /// Commands sourced from MCP server prompts.
    Mcp,
}

impl CommandCategory {
    fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Mcp => "mcp",
        }
    }

    fn from_str(value: &str) -> Option<Self> {
        match value {
            "native" => Some(Self::Native),
            "mcp" => Some(Self::Mcp),
            _ => None,
        }
    }
}

pub fn meta_with_command_category(category: CommandCategory) -> acp_v2::Meta {
    acp_v2::Meta::from_iter([(COMMAND_CATEGORY_META_KEY.into(), category.as_str().into())])
}

pub fn command_category_from_meta(meta: &Option<acp_v2::Meta>) -> Option<CommandCategory> {
    meta.as_ref()
        .and_then(|m| m.get(COMMAND_CATEGORY_META_KEY))
        .and_then(|v| v.as_str())
        .and_then(CommandCategory::from_str)
}

/// Key used in ACP ToolCall meta to store the session id and message indexes
pub const SUBAGENT_SESSION_INFO_META_KEY: &str = "subagent_session_info";

pub const SANDBOX_AUTHORIZATION_META_KEY: &str = "sandbox_authorization";

/// Stable `PermissionOption` ids for the sandbox-escalation approval prompt.
///
/// These are shared across the option construction (in the agent), the outcome
/// dispatch, and the UI so the distinct grant lifetimes stay in sync. Note
/// that `AllowThread` and `AllowAlways` both use
/// `PermissionOptionKind::AllowAlways`; the id is what distinguishes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SandboxPermission {
    AllowOnce,
    AllowThread,
    AllowAlways,
    Deny,
}

impl SandboxPermission {
    pub fn as_id(self) -> &'static str {
        match self {
            Self::AllowOnce => "allow",
            Self::AllowThread => "allow_thread",
            Self::AllowAlways => "allow_always",
            Self::Deny => "deny",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "allow" => Some(Self::AllowOnce),
            "allow_thread" => Some(Self::AllowThread),
            "allow_always" => Some(Self::AllowAlways),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SandboxAuthorizationDetails {
    #[serde(default)]
    pub command: Option<String>,
    /// Specific hosts the command requested network access to, in canonical
    /// form (`github.com`, `*.npmjs.org`). Empty when no specific hosts were
    /// requested (see `network_all_hosts`).
    #[serde(default)]
    pub network_hosts: Vec<String>,
    /// Whether the command requested access to any host ("arbitrary network
    /// access"). The `network` alias deserializes the field this replaced —
    /// a plain bool meaning "network access" — so details persisted by older
    /// builds still render the network request.
    #[serde(default, alias = "network")]
    pub network_all_hosts: bool,

    #[serde(default)]
    pub allow_fs_write_all: bool,
    #[serde(default)]
    pub unsandboxed: bool,
    #[serde(default)]
    pub write_paths: Vec<settings::GrantedWritePath>,
    /// Windows/WSL only: the command will write to a path on a Windows drive
    /// (DrvFs), whose sandbox-integrity guarantees are weaker. Drives the
    /// weaker-guarantee warning banner in the approval prompt.
    #[serde(default)]
    pub warn_windows_fs: bool,
    /// The agent-provided justification for requesting these permissions,
    /// shown to the user (attributed to the agent) in the approval prompt.
    #[serde(default)]
    pub reason: String,
}

pub fn meta_with_sandbox_authorization(details: SandboxAuthorizationDetails) -> acp_v1::Meta {
    acp_v1::Meta::from_iter([(
        SANDBOX_AUTHORIZATION_META_KEY.into(),
        serde_json::to_value(details).unwrap_or_default(),
    )])
}

pub fn sandbox_authorization_details_from_meta(
    meta: &Option<acp_v1::Meta>,
) -> Option<SandboxAuthorizationDetails> {
    meta.as_ref()
        .and_then(|m| m.get(SANDBOX_AUTHORIZATION_META_KEY))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

pub const SANDBOX_FALLBACK_AUTHORIZATION_META_KEY: &str = "sandbox_fallback_authorization";

/// Stable `PermissionOption` id for the "Retry" choice in the sandbox
/// *fallback* prompt (shown when the OS sandbox can't be created on this
/// system). The remaining choices reuse the [`SandboxPermission`] ids.
pub const SANDBOX_FALLBACK_RETRY_OPTION_ID: &str = "retry";

/// Details shown when the OS sandbox could not be created for a command and
/// the user is asked whether to run it without a sandbox. Distinct from
/// [`SandboxAuthorizationDetails`] (a model-requested *escalation*): here the
/// sandbox itself failed, so the prompt explains why and offers a retry.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SandboxFallbackAuthorizationDetails {
    #[serde(default)]
    pub command: Option<String>,
    /// Human-readable reason the OS sandbox could not be created (for example,
    /// "bwrap not found on PATH"), shown to the user so they can decide
    /// whether to run the command without a sandbox.
    #[serde(default)]
    pub reason: String,
    /// Slug of the sandboxing docs section that best explains how to fix this
    /// failure (see [`crate::LinuxWslSandboxError::docs_section`]), rendered as a
    /// "Learn more" link. `None` when the cause is unknown.
    #[serde(default)]
    pub docs_section: Option<String>,
}

pub fn meta_with_sandbox_fallback_authorization(
    details: SandboxFallbackAuthorizationDetails,
) -> acp_v1::Meta {
    acp_v1::Meta::from_iter([(
        SANDBOX_FALLBACK_AUTHORIZATION_META_KEY.into(),
        serde_json::to_value(details).unwrap_or_default(),
    )])
}

pub fn sandbox_fallback_authorization_details_from_meta(
    meta: &Option<acp_v1::Meta>,
) -> Option<SandboxFallbackAuthorizationDetails> {
    meta.as_ref()
        .and_then(|m| m.get(SANDBOX_FALLBACK_AUTHORIZATION_META_KEY))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

/// Meta key recording why the OS sandbox was not applied to a terminal tool
/// call, even though sandboxing was active for the thread. The value is a
/// serialized [`SandboxNotAppliedReason`]. Surfaced as a warning in the UI and
/// used to explain the situation to both the user and the agent.
pub const SANDBOX_NOT_APPLIED_META_KEY: &str = "sandbox_not_applied";

pub fn meta_with_sandbox_not_applied(reason: &SandboxNotAppliedReason) -> acp_v1::Meta {
    acp_v1::Meta::from_iter([(
        SANDBOX_NOT_APPLIED_META_KEY.into(),
        serde_json::to_value(reason).unwrap_or_default(),
    )])
}

pub fn sandbox_not_applied_from_meta(
    meta: &Option<acp_v1::Meta>,
) -> Option<SandboxNotAppliedReason> {
    meta.as_ref()
        .and_then(|m| m.get(SANDBOX_NOT_APPLIED_META_KEY))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubagentSessionInfo {
    /// The session id of the subagent sessiont that was spawned
    pub session_id: acp_v2::SessionId,
    /// The index of the message of the start of the "turn" run by this tool call
    pub message_start_index: usize,
    /// The index of the output of the message that the subagent has returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_end_index: Option<usize>,
}

/// Helper to extract subagent session id from ACP meta
pub fn subagent_session_info_from_meta(meta: &Option<acp_v1::Meta>) -> Option<SubagentSessionInfo> {
    meta.as_ref()
        .and_then(|m| m.get(SUBAGENT_SESSION_INFO_META_KEY))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

#[derive(Debug)]
pub struct UserMessage {
    pub identity: MessageIdentity,
    pub meta: Option<acp_v2::Meta>,
    pub client_id: Option<ClientUserMessageId>,
    pub is_optimistic: bool,
    pub content: MessageContent,
    pub checkpoint: Option<Checkpoint>,
    pub indented: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageIdentity {
    /// Legacy IDs delimit adjacent chunks; they do not identify a global upsert target.
    Legacy(Option<acp_v1::MessageId>),
    /// Keeps its first kind and transcript position even when updates are non-adjacent.
    Keyed(acp_v2::MessageId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    User,
    Assistant,
    Thought,
}

#[derive(Debug)]
pub struct Checkpoint {
    git_checkpoint: GitStoreCheckpoint,
    pub show: bool,
}

impl UserMessage {
    fn to_markdown(&self, cx: &App) -> String {
        let mut markdown = String::new();
        if self
            .checkpoint
            .as_ref()
            .is_some_and(|checkpoint| checkpoint.show)
        {
            writeln!(markdown, "## User (checkpoint)").unwrap();
        } else {
            writeln!(markdown, "## User").unwrap();
        }
        writeln!(markdown).unwrap();
        writeln!(markdown, "{}", self.content.to_markdown(cx)).unwrap();
        writeln!(markdown).unwrap();
        markdown
    }
}

#[derive(Debug, PartialEq)]
pub struct AssistantMessage {
    pub chunks: Vec<AssistantMessageChunk>,
    pub indented: bool,
    pub is_subagent_output: bool,
}

impl AssistantMessage {
    pub fn to_markdown(&self, cx: &App) -> String {
        format!(
            "## Assistant\n\n{}\n\n",
            self.chunks
                .iter()
                .map(|chunk| chunk.to_markdown(cx))
                .join("\n\n")
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum AssistantMessageChunk {
    Message {
        identity: MessageIdentity,
        meta: Option<acp_v2::Meta>,
        block: MessageContent,
    },
    Thought {
        identity: MessageIdentity,
        meta: Option<acp_v2::Meta>,
        block: MessageContent,
    },
}

impl AssistantMessageChunk {
    pub fn from_str(
        chunk: &str,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        Self::Message {
            identity: MessageIdentity::Legacy(None),
            meta: None,
            block: MessageContent::new(chunk.into(), language_registry, path_style, cx),
        }
    }

    fn identity(&self) -> &MessageIdentity {
        match self {
            Self::Message { identity, .. } | Self::Thought { identity, .. } => identity,
        }
    }

    fn kind(&self) -> MessageKind {
        match self {
            Self::Message { .. } => MessageKind::Assistant,
            Self::Thought { .. } => MessageKind::Thought,
        }
    }

    fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::Message { block, .. } => block.to_markdown(cx),
            Self::Thought { block, .. } => {
                format!("<thinking>\n{}\n</thinking>", block.to_markdown(cx))
            }
        }
    }
}

fn can_merge_message_chunks(
    existing: Option<&acp_v1::MessageId>,
    incoming: Option<&acp_v1::MessageId>,
) -> bool {
    match (existing, incoming) {
        (Some(existing), Some(incoming)) => existing == incoming,
        _ => true,
    }
}

#[derive(Clone, Copy)]
enum MessageLocation {
    User {
        entry_index: usize,
    },
    Assistant {
        entry_index: usize,
        chunk_index: usize,
    },
}

impl MessageLocation {
    fn entry_index(self) -> usize {
        match self {
            Self::User { entry_index } | Self::Assistant { entry_index, .. } => entry_index,
        }
    }

    fn fields_mut(
        self,
        entries: &mut [AgentThreadEntry],
    ) -> Option<(&mut MessageContent, &mut Option<acp_v2::Meta>)> {
        match (self, entries.get_mut(self.entry_index())?) {
            (Self::User { .. }, AgentThreadEntry::UserMessage(message)) => {
                Some((&mut message.content, &mut message.meta))
            }
            (Self::Assistant { chunk_index, .. }, AgentThreadEntry::AssistantMessage(message)) => {
                match message.chunks.get_mut(chunk_index)? {
                    AssistantMessageChunk::Message { block, meta, .. }
                    | AssistantMessageChunk::Thought { block, meta, .. } => Some((block, meta)),
                }
            }
            _ => None,
        }
    }

    fn is_streaming_target(self, target: &StreamingTextTarget) -> bool {
        matches!(
            self,
            Self::Assistant { entry_index, chunk_index }
                if entry_index == target.entry_index && chunk_index == target.chunk_index
        )
    }
}

#[derive(Debug)]
pub enum AgentThreadEntry {
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolCall(ToolCall),
    Elicitation(ElicitationEntryId),
    ContextCompaction(ContextCompaction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElicitationEntryId(pub Arc<str>);

#[derive(Debug)]
pub struct Elicitation {
    pub id: ElicitationEntryId,
    pub request: acp_v2::CreateElicitationRequest,
    pub status: ElicitationStatus,
}

#[derive(Debug)]
pub enum ElicitationStatus {
    Pending {
        respond_tx: oneshot::Sender<acp_v2::CreateElicitationResponse>,
    },
    Accepted,
    Declined,
    Canceled,
    Completed,
}

enum ElicitationChange {
    Responded,
    Updated,
}

#[derive(Clone, Debug)]
pub enum ElicitationStoreEvent {
    ElicitationRequested(ElicitationEntryId),
    /// The request left `Pending`; this does not imply delivery to its response waiter.
    ElicitationResponded(ElicitationEntryId),
    ElicitationUpdated(ElicitationEntryId),
}

#[derive(Default)]
pub struct ElicitationStore {
    elicitations: Vec<Elicitation>,
}

impl EventEmitter<ElicitationStoreEvent> for ElicitationStore {}

impl ElicitationStore {
    pub fn elicitations(&self) -> &[Elicitation] {
        &self.elicitations
    }

    fn validate_request(request: &acp_v2::CreateElicitationRequest) -> Result<(), acp_v2::Error> {
        match &request.mode {
            acp_v2::ElicitationMode::Form(_) => {}
            acp_v2::ElicitationMode::Url(mode) => {
                let url = url::Url::parse(&mode.url)
                    .map_err(|_| acp_v2::Error::invalid_params().data("invalid elicitation URL"))?;
                if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
                    return Err(acp_v2::Error::invalid_params()
                        .data("elicitation URL must use HTTP or HTTPS and include a host"));
                }
            }
            _ => {
                return Err(acp_v2::Error::invalid_params().data("unsupported elicitation mode"));
            }
        }

        Ok(())
    }

    fn insert_pending_elicitation(
        &mut self,
        request: acp_v2::CreateElicitationRequest,
    ) -> (
        ElicitationEntryId,
        oneshot::Receiver<acp_v2::CreateElicitationResponse>,
    ) {
        let (respond_tx, response_rx) = oneshot::channel();
        let id = ElicitationEntryId(Uuid::new_v4().to_string().into());
        self.elicitations.push(Elicitation {
            id: id.clone(),
            request,
            status: ElicitationStatus::Pending { respond_tx },
        });
        (id, response_rx)
    }

    fn response_task(
        response_rx: oneshot::Receiver<acp_v2::CreateElicitationResponse>,
        cx: &App,
    ) -> Task<acp_v2::CreateElicitationResponse> {
        cx.foreground_executor().spawn(async move {
            response_rx.await.unwrap_or_else(|oneshot::Canceled| {
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Cancel)
            })
        })
    }

    fn emit_change(id: ElicitationEntryId, change: ElicitationChange, cx: &mut Context<Self>) {
        cx.emit(ElicitationStoreEvent::ElicitationUpdated(id.clone()));
        if matches!(change, ElicitationChange::Responded) {
            cx.emit(ElicitationStoreEvent::ElicitationResponded(id));
        }
    }

    fn respond_to_elicitation_entry(
        elicitation: &mut Elicitation,
        response: acp_v2::CreateElicitationResponse,
    ) -> bool {
        if !matches!(elicitation.status, ElicitationStatus::Pending { .. }) {
            return false;
        }
        let ElicitationStatus::Pending { respond_tx } = mem::replace(
            &mut elicitation.status,
            elicitation_status_for_response(&response),
        ) else {
            return false;
        };
        if respond_tx.send(response).is_err() {
            log::debug!("Elicitation waiter closed before its response was delivered");
        }
        true
    }

    fn complete_url_elicitation_entry(elicitation: &mut Elicitation) -> bool {
        let previous_status = mem::replace(&mut elicitation.status, ElicitationStatus::Completed);
        match previous_status {
            ElicitationStatus::Accepted => true,
            previous_status @ (ElicitationStatus::Pending { .. }
            | ElicitationStatus::Declined
            | ElicitationStatus::Canceled
            | ElicitationStatus::Completed) => {
                elicitation.status = previous_status;
                false
            }
        }
    }

    fn cancel_elicitation_entry(elicitation: &mut Elicitation) -> Option<ElicitationChange> {
        match mem::replace(&mut elicitation.status, ElicitationStatus::Canceled) {
            ElicitationStatus::Pending { respond_tx } => {
                if respond_tx
                    .send(acp_v2::CreateElicitationResponse::new(
                        acp_v2::ElicitationAction::Cancel,
                    ))
                    .is_err()
                {
                    log::debug!("Elicitation waiter closed before cancellation was delivered");
                }
                Some(ElicitationChange::Responded)
            }
            ElicitationStatus::Accepted
                if matches!(&elicitation.request.mode, acp_v2::ElicitationMode::Url(_)) =>
            {
                Some(ElicitationChange::Updated)
            }
            previous_status => {
                elicitation.status = previous_status;
                None
            }
        }
    }

    fn respond_to_elicitation_by_id(
        &mut self,
        id: &ElicitationEntryId,
        response: acp_v2::CreateElicitationResponse,
    ) -> bool {
        let Some((_, elicitation)) = self.elicitation_mut(id) else {
            return false;
        };
        Self::respond_to_elicitation_entry(elicitation, response)
    }

    fn complete_url_elicitation_by_id(&mut self, id: &ElicitationEntryId) -> bool {
        let Some((_, elicitation)) = self.elicitation_mut(id) else {
            return false;
        };
        Self::complete_url_elicitation_entry(elicitation)
    }

    fn cancel_elicitation_by_id(&mut self, id: &ElicitationEntryId) -> Option<ElicitationChange> {
        let (_, elicitation) = self.elicitation_mut(id)?;
        Self::cancel_elicitation_entry(elicitation)
    }

    pub fn request_elicitation(
        &mut self,
        request: acp_v2::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<Task<acp_v2::CreateElicitationResponse>, acp_v2::Error> {
        self.request_elicitation_with_id(request, cx)
            .map(|(_, task)| task)
    }

    pub fn request_elicitation_with_id(
        &mut self,
        request: acp_v2::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<(ElicitationEntryId, Task<acp_v2::CreateElicitationResponse>), acp_v2::Error> {
        Self::validate_request(&request)?;
        let (id, response_rx) = self.insert_pending_elicitation(request);
        cx.emit(ElicitationStoreEvent::ElicitationRequested(id.clone()));
        cx.notify();

        let task = Self::response_task(response_rx, cx);
        Ok((id, task))
    }

    pub fn respond_to_elicitation(
        &mut self,
        id: &ElicitationEntryId,
        response: acp_v2::CreateElicitationResponse,
        cx: &mut Context<Self>,
    ) {
        if !self.respond_to_elicitation_by_id(id, response) {
            return;
        }

        Self::emit_change(id.clone(), ElicitationChange::Responded, cx);
        cx.notify();
    }

    pub fn complete_url_elicitation(
        &mut self,
        elicitation_id: &acp_v2::ElicitationId,
        cx: &mut Context<Self>,
    ) {
        let Some(entry_id) = self.entry_id_for_url_elicitation(elicitation_id) else {
            return;
        };
        if !self.complete_url_elicitation_by_id(&entry_id) {
            return;
        }

        cx.emit(ElicitationStoreEvent::ElicitationUpdated(entry_id));
        cx.notify();
    }

    pub fn cancel_elicitation(&mut self, id: &ElicitationEntryId, cx: &mut Context<Self>) {
        let Some(change) = self.cancel_elicitation_by_id(id) else {
            return;
        };

        Self::emit_change(id.clone(), change, cx);
        cx.notify();
    }

    pub fn cancel_all(&mut self, cx: &mut Context<Self>) {
        for (id, change) in self.cancel_pending(|_| true) {
            Self::emit_change(id, change, cx);
        }
        cx.notify();
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        let changes = self.cancel_pending(|_| true);
        self.elicitations.clear();
        for (id, change) in changes {
            Self::emit_change(id, change, cx);
        }
        cx.notify();
    }

    pub fn clear_resolved(&mut self, cx: &mut Context<Self>) -> Vec<ElicitationEntryId> {
        let mut cleared_ids = Vec::new();
        self.elicitations.retain(|elicitation| {
            let keep = matches!(
                (&elicitation.status, &elicitation.request.mode),
                (ElicitationStatus::Pending { .. }, _)
                    | (ElicitationStatus::Accepted, acp_v2::ElicitationMode::Url(_))
            );
            if !keep {
                cleared_ids.push(elicitation.id.clone());
            }
            keep
        });

        if !cleared_ids.is_empty() {
            for id in &cleared_ids {
                cx.emit(ElicitationStoreEvent::ElicitationUpdated(id.clone()));
            }
            cx.notify();
        }

        cleared_ids
    }

    pub fn cancel_request(&mut self, request_id: &acp_v2::RequestId, cx: &mut Context<Self>) {
        let changes = self.cancel_pending(|elicitation| {
            matches!(
                elicitation.request.scope(),
                acp_v2::ElicitationScope::Request(scope) if &scope.request_id == request_id
            )
        });
        for (id, change) in changes {
            Self::emit_change(id, change, cx);
        }
        cx.notify();
    }

    pub fn elicitation(&self, id: &ElicitationEntryId) -> Option<(usize, &Elicitation)> {
        self.elicitations
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, elicitation)| {
                (&elicitation.id == id).then_some((index, elicitation))
            })
    }

    fn entry_id_for_url_elicitation(
        &self,
        elicitation_id: &acp_v2::ElicitationId,
    ) -> Option<ElicitationEntryId> {
        self.elicitations.iter().rev().find_map(|elicitation| {
            if let acp_v2::ElicitationMode::Url(mode) = &elicitation.request.mode
                && &mode.elicitation_id == elicitation_id
            {
                Some(elicitation.id.clone())
            } else {
                None
            }
        })
    }

    fn elicitation_mut(&mut self, id: &ElicitationEntryId) -> Option<(usize, &mut Elicitation)> {
        self.elicitations
            .iter_mut()
            .enumerate()
            .rev()
            .find_map(|(index, elicitation)| {
                (&elicitation.id == id).then_some((index, elicitation))
            })
    }

    fn cancel_pending(
        &mut self,
        mut should_cancel: impl FnMut(&Elicitation) -> bool,
    ) -> Vec<(ElicitationEntryId, ElicitationChange)> {
        let mut changes = Vec::new();
        for elicitation in &mut self.elicitations {
            if should_cancel(elicitation)
                && let Some(change) = Self::cancel_elicitation_entry(elicitation)
            {
                changes.push((elicitation.id.clone(), change));
            }
        }
        changes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextCompactionId(pub Arc<str>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextCompactionStatus {
    InProgress,
    Completed,
    Failed,
    Canceled,
    Other(Arc<str>),
}

impl From<acp_v2::CompactionStatus> for ContextCompactionStatus {
    fn from(status: acp_v2::CompactionStatus) -> Self {
        match status {
            acp_v2::CompactionStatus::InProgress => Self::InProgress,
            acp_v2::CompactionStatus::Completed => Self::Completed,
            acp_v2::CompactionStatus::Failed => Self::Failed,
            acp_v2::CompactionStatus::Cancelled => Self::Canceled,
            acp_v2::CompactionStatus::Other(status) => Self::Other(status.into()),
            _ => Self::Other("unknown".into()),
        }
    }
}

/// A point in the thread where the conversation history was compacted to free
/// up room in the model's context window. The summary can be expanded to inspect
/// what the model retained.
#[derive(Debug)]
pub struct ContextCompaction {
    pub id: ContextCompactionId,
    pub status: ContextCompactionStatus,
    pub error: Option<Entity<Markdown>>,
    pub summary: MessageContent,
    pub meta: Option<acp_v2::Meta>,
}

impl ContextCompaction {
    pub fn is_in_progress(&self) -> bool {
        self.status == ContextCompactionStatus::InProgress
    }

    fn apply_update(
        &mut self,
        update: acp_v2::CompactionUpdate,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        self.status = update.status.into();
        match update.summary {
            MaybeUndefined::Undefined => {}
            MaybeUndefined::Null => self.summary = MessageContent::default(),
            MaybeUndefined::Value(blocks) => {
                self.summary = MessageContent::default();
                for block in blocks {
                    self.append_summary(block, language_registry, cx);
                }
            }
        }
        match update.error {
            MaybeUndefined::Undefined => {}
            MaybeUndefined::Null => self.error = None,
            MaybeUndefined::Value(error) => {
                if let Some(markdown) = &self.error {
                    markdown.update(cx, |markdown, cx| markdown.reset(error.into(), cx));
                } else {
                    self.error = Some(cx.new(|cx| Markdown::new_text(error.into(), cx)));
                }
            }
        }
        match update.meta {
            MaybeUndefined::Undefined => {}
            MaybeUndefined::Null => self.meta = None,
            MaybeUndefined::Value(meta) => self.meta = Some(meta),
        }
    }

    fn append_summary(
        &mut self,
        content: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        self.summary
            .append_compaction_summary(content, language_registry, cx);
    }
}

#[derive(Debug)]
pub struct ContextCompactionUpdate {
    pub id: ContextCompactionId,
    pub summary_delta: String,
    pub status: Option<ContextCompactionStatus>,
}

impl AgentThreadEntry {
    pub fn is_indented(&self) -> bool {
        match self {
            Self::UserMessage(message) => message.indented,
            Self::AssistantMessage(message) => message.indented,
            Self::ToolCall(_) => false,
            Self::Elicitation(_) => false,
            Self::ContextCompaction(_) => false,
        }
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::UserMessage(message) => message.to_markdown(cx),
            Self::AssistantMessage(message) => message.to_markdown(cx),
            Self::ToolCall(tool_call) => tool_call.to_markdown(cx),
            Self::Elicitation(_) => "## Input Requested\n\n".to_string(),
            Self::ContextCompaction(compaction) => {
                let status = match &compaction.status {
                    ContextCompactionStatus::InProgress => "In Progress",
                    ContextCompactionStatus::Completed => "Completed",
                    ContextCompactionStatus::Failed => "Failed",
                    ContextCompactionStatus::Canceled => "Canceled",
                    ContextCompactionStatus::Other(status) => status,
                };
                let mut markdown =
                    format!("## Context Compaction ({})\n\n", MarkdownEscaped(status));
                for block in compaction.summary.blocks() {
                    markdown.push_str(block.to_markdown(cx));
                    markdown.push_str("\n\n");
                }
                if let Some(error) = &compaction.error {
                    markdown.push_str("**Error:** ");
                    markdown.push_str(&MarkdownEscaped(error.read(cx).source()).to_string());
                    markdown.push_str("\n\n");
                }
                markdown
            }
        }
    }

    pub fn user_message(&self) -> Option<&UserMessage> {
        if let AgentThreadEntry::UserMessage(message) = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn diffs(&self) -> impl Iterator<Item = &Entity<Diff>> {
        if let AgentThreadEntry::ToolCall(call) = self {
            itertools::Either::Left(call.diffs())
        } else {
            itertools::Either::Right(std::iter::empty())
        }
    }

    pub fn terminals(&self) -> impl Iterator<Item = &Entity<Terminal>> {
        if let AgentThreadEntry::ToolCall(call) = self {
            itertools::Either::Left(call.terminals())
        } else {
            itertools::Either::Right(std::iter::empty())
        }
    }

    pub fn location(&self, ix: usize) -> Option<(ToolCallLocation, AgentLocation)> {
        if let AgentThreadEntry::ToolCall(ToolCall {
            locations,
            resolved_locations,
            ..
        }) = self
        {
            Some((
                locations.get(ix)?.clone(),
                resolved_locations.get(ix)?.clone()?,
            ))
        } else {
            None
        }
    }
}

/// Native tools can supply relative paths, unlike v2 protocol locations.
/// Keep the raw path here; resolved editor locations are a separate projection.
#[derive(Clone, Debug, Eq)]
pub struct ToolCallLocation {
    pub path: PathBuf,
    pub line: Option<u32>,
    pub meta: Option<acp_v2::Meta>,
}

impl PartialEq for ToolCallLocation {
    fn eq(&self, other: &Self) -> bool {
        // Path equality compares components, hiding changes to the raw spelling.
        self.path.as_os_str() == other.path.as_os_str()
            && self.line == other.line
            && self.meta == other.meta
    }
}

impl From<acp_v1::ToolCallLocation> for ToolCallLocation {
    fn from(location: acp_v1::ToolCallLocation) -> Self {
        Self {
            path: location.path,
            line: location.line,
            meta: location.meta,
        }
    }
}

impl From<acp_v2::ToolCallLocation> for ToolCallLocation {
    fn from(location: acp_v2::ToolCallLocation) -> Self {
        Self {
            path: location.path.0,
            line: location.line,
            meta: location.meta,
        }
    }
}

#[derive(Debug)]
pub struct ToolCall {
    pub id: acp_v2::ToolCallId,
    pub label: Entity<Markdown>,
    title: Option<SharedString>,
    pub name: Option<SharedString>,
    pub reported_kind: Option<acp_v2::ToolKind>,
    pub reported_status: Option<acp_v2::ToolCallStatus>,
    pub meta: Option<acp_v2::Meta>,
    structured_content: Vec<ToolCallContent>,
    local_status: Option<ToolCallStatus>,
    authorization: Option<PermissionRequestId>,
    pub locations: Vec<ToolCallLocation>,
    pub resolved_locations: Vec<Option<AgentLocation>>,
    pub raw_input: Option<serde_json::Value>,
    pub raw_input_markdown: Option<Entity<Markdown>>,
    pub raw_output: Option<serde_json::Value>,
    raw_output_content: Option<Box<ToolCallContent>>,
    pub tool_name: Option<SharedString>,
    pub subagent_session_info: Option<SubagentSessionInfo>,
    pub sandbox_authorization_details: Option<SandboxAuthorizationDetails>,
    pub sandbox_fallback_authorization_details: Option<SandboxFallbackAuthorizationDetails>,
    /// Why this terminal command ran without the OS sandbox even though
    /// sandboxing was active (see [`SANDBOX_NOT_APPLIED_META_KEY`]). `None` when
    /// the command was sandboxed normally (or sandboxing was off).
    pub sandbox_not_applied: Option<SandboxNotAppliedReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PermissionRequestId(Uuid);

#[derive(Debug)]
pub struct PermissionRequest {
    pub id: PermissionRequestId,
    data: PermissionRequestData,
}

#[derive(Debug)]
enum PermissionRequestData {
    LegacyTool {
        tool_call_id: acp_v2::ToolCallId,
        options: PermissionOptions,
        kind: AuthorizationKind,
        respond_tx: oneshot::Sender<RequestPermissionOutcome>,
    },
    Generic {
        request: acp_v2::RequestPermissionRequest,
        respond_tx: oneshot::Sender<acp_v2::RequestPermissionOutcome>,
    },
}

impl PermissionRequest {
    pub fn legacy_tool_call_id(&self) -> Option<&acp_v2::ToolCallId> {
        match &self.data {
            PermissionRequestData::LegacyTool { tool_call_id, .. } => Some(tool_call_id),
            PermissionRequestData::Generic { .. } => None,
        }
    }

    pub fn legacy_options(&self) -> Option<&PermissionOptions> {
        match &self.data {
            PermissionRequestData::LegacyTool { options, .. } => Some(options),
            PermissionRequestData::Generic { .. } => None,
        }
    }

    pub fn legacy_kind(&self) -> Option<AuthorizationKind> {
        match &self.data {
            PermissionRequestData::LegacyTool { kind, .. } => Some(*kind),
            PermissionRequestData::Generic { .. } => None,
        }
    }

    pub fn generic_request(&self) -> Option<&acp_v2::RequestPermissionRequest> {
        match &self.data {
            PermissionRequestData::LegacyTool { .. } => None,
            PermissionRequestData::Generic { request, .. } => Some(request),
        }
    }
}

struct ToolCallPatch {
    title: MaybeUndefined<String>,
    name: MaybeUndefined<String>,
    kind: MaybeUndefined<acp_v2::ToolKind>,
    status: MaybeUndefined<acp_v2::ToolCallStatus>,
    content: MaybeUndefined<ToolContentPatch>,
    locations: MaybeUndefined<Vec<ToolCallLocation>>,
    raw_input: MaybeUndefined<serde_json::Value>,
    raw_output: MaybeUndefined<serde_json::Value>,
    meta: ToolMetadataPatch,
}

enum ToolContentPatch {
    V1(Vec<acp_v1::ToolCallContent>),
    V2(Vec<acp_v2::ToolCallContent>),
}

#[derive(Clone, Copy)]
struct ToolTerminalResolver<'a> {
    terminals: &'a HashMap<acp_v2::TerminalId, Entity<Terminal>>,
    client_managed_only: bool,
}

impl<'a> ToolTerminalResolver<'a> {
    fn registered(terminals: &'a HashMap<acp_v2::TerminalId, Entity<Terminal>>) -> Self {
        Self {
            terminals,
            client_managed_only: false,
        }
    }

    fn resolve(self, id: &acp_v2::TerminalId, cx: &App) -> Result<Entity<Terminal>> {
        let terminal = self
            .terminals
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("Terminal with id `{id}` not found"))?;
        if self.client_managed_only {
            // Process ownership survives completion and does not depend on
            // whether the renderer still has an active PTY.
            anyhow::ensure!(
                terminal.read(cx).is_process_backed(),
                "Client-managed tool content cannot reference an agent-owned display terminal"
            );
        }
        Ok(terminal)
    }
}

impl ToolContentPatch {
    fn prepare(
        self,
        terminals: ToolTerminalResolver<'_>,
        cx: &App,
    ) -> Result<Vec<PreparedToolCallContent>> {
        match self {
            Self::V1(content) => PreparedToolCallContent::prepare(content, terminals, cx),
            Self::V2(content) => PreparedToolCallContent::prepare_v2(content, terminals, cx),
        }
    }
}

enum ToolMetadataPatch {
    Legacy(Option<acp_v1::Meta>),
    Protocol(MaybeUndefined<acp_v2::Meta>),
}

fn legacy_tool_field<T>(value: Option<T>) -> MaybeUndefined<T> {
    value.map(MaybeUndefined::Value).unwrap_or_default()
}

fn tool_kind_from_v1(kind: acp_v1::ToolKind) -> Option<acp_v2::ToolKind> {
    Some(match kind {
        acp_v1::ToolKind::Read => acp_v2::ToolKind::Read,
        acp_v1::ToolKind::Edit => acp_v2::ToolKind::Edit,
        acp_v1::ToolKind::Delete => acp_v2::ToolKind::Delete,
        acp_v1::ToolKind::Move => acp_v2::ToolKind::Move,
        acp_v1::ToolKind::Search => acp_v2::ToolKind::Search,
        acp_v1::ToolKind::Execute => acp_v2::ToolKind::Execute,
        acp_v1::ToolKind::Think => acp_v2::ToolKind::Think,
        acp_v1::ToolKind::Fetch => acp_v2::ToolKind::Fetch,
        acp_v1::ToolKind::SwitchMode => acp_v2::ToolKind::SwitchMode,
        acp_v1::ToolKind::Other => acp_v2::ToolKind::Other,
        _ => return None,
    })
}

fn tool_status_from_v1(status: acp_v1::ToolCallStatus) -> Option<acp_v2::ToolCallStatus> {
    Some(match status {
        acp_v1::ToolCallStatus::Pending => acp_v2::ToolCallStatus::Pending,
        acp_v1::ToolCallStatus::InProgress => acp_v2::ToolCallStatus::InProgress,
        acp_v1::ToolCallStatus::Completed => acp_v2::ToolCallStatus::Completed,
        acp_v1::ToolCallStatus::Failed => acp_v2::ToolCallStatus::Failed,
        _ => return None,
    })
}

impl ToolCallPatch {
    fn legacy(fields: acp_v1::ToolCallUpdateFields, meta: Option<acp_v1::Meta>) -> Self {
        Self {
            title: legacy_tool_field(fields.title),
            name: legacy_tool_field(fields.name),
            kind: legacy_tool_field(fields.kind.and_then(tool_kind_from_v1)),
            status: legacy_tool_field(fields.status.and_then(tool_status_from_v1)),
            content: legacy_tool_field(fields.content.map(ToolContentPatch::V1)),
            locations: legacy_tool_field(
                fields
                    .locations
                    .map(|locations| locations.into_iter().map(Into::into).collect()),
            ),
            raw_input: legacy_tool_field(fields.raw_input),
            raw_output: legacy_tool_field(fields.raw_output),
            meta: ToolMetadataPatch::Legacy(meta),
        }
    }

    fn protocol(update: acp_v2::ToolCallUpdate) -> Self {
        Self {
            title: update.title,
            name: update.name,
            kind: update.kind,
            status: update.status,
            content: update.content.map_value(ToolContentPatch::V2),
            locations: update
                .locations
                .map_value(|locations| locations.into_iter().map(Into::into).collect()),
            raw_input: update.raw_input,
            raw_output: update.raw_output,
            meta: ToolMetadataPatch::Protocol(update.meta),
        }
    }
}

impl ToolCall {
    fn from_acp(
        tool_call: acp_v1::ToolCall,
        status: Option<ToolCallStatus>,
        language_registry: Arc<LanguageRegistry>,
        terminals: &HashMap<acp_v2::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<Self> {
        let update = acp_v1::ToolCallUpdate::from(tool_call);
        let mut call = Self::from_patch(
            acp_v2::ToolCallId::new(update.tool_call_id.0),
            ToolCallPatch::legacy(update.fields, update.meta),
            language_registry,
            ToolTerminalResolver::registered(terminals),
            cx,
        )?;
        if let Some(status) = status {
            call.set_legacy_status(status);
        }
        Ok(call)
    }

    fn from_patch(
        id: acp_v2::ToolCallId,
        patch: ToolCallPatch,
        language_registry: Arc<LanguageRegistry>,
        terminals: ToolTerminalResolver<'_>,
        cx: &mut App,
    ) -> Result<Self> {
        let content = patch
            .content
            .take()
            .map(|content| content.prepare(terminals, cx))
            .transpose()?
            .unwrap_or_default()
            .into_iter()
            .map(|item| ToolCallContent::from_prepared(item, &language_registry, cx))
            .collect();
        let title = patch.title.take().map(SharedString::from);
        let name = patch.name.take().map(SharedString::from);
        let kind = patch.kind.take();
        let raw_input = patch.raw_input.take();
        let raw_input_markdown = raw_input
            .as_ref()
            .and_then(|input| markdown_for_raw_output(input, &language_registry, cx));
        let meta = match patch.meta {
            ToolMetadataPatch::Legacy(meta) => meta,
            ToolMetadataPatch::Protocol(meta) => meta.take(),
        };
        let tool_name = name.clone().or_else(|| tool_name_from_meta(&meta));
        let subagent_session_info = subagent_session_info_from_meta(&meta);
        let sandbox_authorization_details = sandbox_authorization_details_from_meta(&meta);
        let sandbox_fallback_authorization_details =
            sandbox_fallback_authorization_details_from_meta(&meta);
        let sandbox_not_applied = sandbox_not_applied_from_meta(&meta);
        let label = Self::new_label(
            title.as_ref().filter(|title| !title.trim().is_empty()),
            tool_name.as_ref(),
            kind.as_ref().unwrap_or(&acp_v2::ToolKind::Other),
            language_registry.clone(),
            cx,
        );

        let mut result = Self {
            id,
            label,
            title,
            name,
            reported_kind: kind,
            reported_status: patch.status.take(),
            meta,
            structured_content: content,
            locations: patch.locations.take().unwrap_or_default(),
            resolved_locations: Vec::default(),
            local_status: None,
            authorization: None,
            raw_input,
            raw_input_markdown,
            raw_output: patch.raw_output.take(),
            raw_output_content: None,
            tool_name,
            subagent_session_info,
            sandbox_authorization_details,
            sandbox_fallback_authorization_details,
            sandbox_not_applied,
        };
        result.update_raw_output_content(&language_registry, cx);
        Ok(result)
    }

    pub fn kind(&self) -> &acp_v2::ToolKind {
        self.reported_kind
            .as_ref()
            .unwrap_or(&acp_v2::ToolKind::Other)
    }

    pub fn status(&self) -> ToolCallStatus {
        if self.authorization.is_some() {
            ToolCallStatus::WaitingForConfirmation
        } else {
            self.underlying_status()
        }
    }

    pub fn authorization_id(&self) -> Option<PermissionRequestId> {
        self.authorization
    }

    fn underlying_status(&self) -> ToolCallStatus {
        self.local_status
            .or_else(|| ToolCallStatus::from_reported(self.reported_status.as_ref()))
            .unwrap_or(ToolCallStatus::Pending)
    }

    fn permission_status(&self) -> Option<acp_v2::ToolCallStatus> {
        self.local_status
            .or_else(|| ToolCallStatus::from_reported(self.reported_status.as_ref()))
            .and_then(|status| status.as_acp_status())
    }

    fn set_legacy_status(&mut self, status: ToolCallStatus) {
        self.local_status = (Some(status)
            != ToolCallStatus::from_reported(self.reported_status.as_ref()))
        .then_some(status);
    }

    fn set_local_status(&mut self, status: ToolCallStatus) {
        // An explicit decision survives a later clear of reported information,
        // even if the two statuses happen to agree right now.
        self.local_status = Some(status);
    }

    fn effective_title(&self) -> Option<&SharedString> {
        self.title.as_ref().filter(|title| !title.trim().is_empty())
    }

    fn label_text(
        title: Option<&SharedString>,
        tool_name: Option<&SharedString>,
        kind: &acp_v2::ToolKind,
    ) -> SharedString {
        let Some(title) = title else {
            return tool_name
                .filter(|name| !name.trim().is_empty())
                .cloned()
                .unwrap_or_else(|| "Tool call".into());
        };

        if kind == &acp_v2::ToolKind::Execute {
            title.clone()
        } else if kind == &acp_v2::ToolKind::Edit {
            MarkdownEscaped(title).to_string().into()
        } else if let Some((first_line, _)) = title.split_once('\n') {
            (first_line.to_owned() + "…").into()
        } else {
            title.clone()
        }
    }

    fn new_label(
        title: Option<&SharedString>,
        tool_name: Option<&SharedString>,
        kind: &acp_v2::ToolKind,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Entity<Markdown> {
        let text = Self::label_text(title, tool_name, kind);
        cx.new(|cx| {
            if title.is_none() || kind == &acp_v2::ToolKind::Execute {
                Markdown::new_text(text, cx)
            } else {
                Markdown::new(text, Some(language_registry), None, cx)
            }
        })
    }

    fn apply_patch(
        &mut self,
        patch: ToolCallPatch,
        language_registry: Arc<LanguageRegistry>,
        terminals: ToolTerminalResolver<'_>,
        cx: &mut App,
    ) -> Result<()> {
        let legacy_terminal_labels = matches!(&patch.meta, ToolMetadataPatch::Legacy(_));
        let ToolCallPatch {
            kind,
            status,
            title,
            name,
            content,
            locations,
            raw_input,
            raw_output,
            meta,
        } = patch;
        let output_changed = !content.is_undefined() || !raw_output.is_undefined();
        // Completion remains authoritative even when output cannot be displayed;
        // otherwise a finished tool could leave its permission request waiting.
        self.apply_reported_status(status);
        // Prepare all content before changing presentation fields or shared
        // display entities; cloning their handles would not isolate mutations.
        let content = match content {
            MaybeUndefined::Undefined => None,
            MaybeUndefined::Null => Some(Vec::new()),
            MaybeUndefined::Value(content) => Some(content.prepare(terminals, cx)?),
        };
        let was_plain_text =
            self.effective_title().is_none() || self.kind() == &acp_v2::ToolKind::Execute;
        let mut label_changed = !title.is_undefined() || !kind.is_undefined();
        if !kind.is_undefined() {
            self.reported_kind = kind.take();
        }
        let name_changed = !name.is_undefined();
        if name_changed {
            self.name = name.take().map(SharedString::from);
        }
        let old_tool_name = self.tool_name.clone();
        match meta {
            ToolMetadataPatch::Legacy(meta) => {
                if name_changed {
                    self.tool_name = self.name.clone();
                } else if self.tool_name.is_none() {
                    self.tool_name = tool_name_from_meta(&meta);
                }
                // Legacy extension hints are sticky; omitted or malformed hints
                // have never cleared values received in an earlier v1 update.
                if let Some(value) = subagent_session_info_from_meta(&meta) {
                    self.subagent_session_info = Some(value);
                }
                if let Some(value) = sandbox_authorization_details_from_meta(&meta) {
                    self.sandbox_authorization_details = Some(value);
                }
                if let Some(value) = sandbox_fallback_authorization_details_from_meta(&meta) {
                    self.sandbox_fallback_authorization_details = Some(value);
                }
                if let Some(value) = sandbox_not_applied_from_meta(&meta) {
                    self.sandbox_not_applied = Some(value);
                }
                if meta.is_some() {
                    self.meta = meta;
                }
            }
            ToolMetadataPatch::Protocol(meta) => {
                let meta_changed = !meta.is_undefined();
                if meta_changed {
                    self.meta = meta.take();
                    self.subagent_session_info = subagent_session_info_from_meta(&self.meta);
                    self.sandbox_authorization_details =
                        sandbox_authorization_details_from_meta(&self.meta);
                    self.sandbox_fallback_authorization_details =
                        sandbox_fallback_authorization_details_from_meta(&self.meta);
                    self.sandbox_not_applied = sandbox_not_applied_from_meta(&self.meta);
                }
                if name_changed || meta_changed {
                    self.tool_name = self
                        .name
                        .clone()
                        .or_else(|| tool_name_from_meta(&self.meta));
                }
            }
        }
        label_changed |= self.tool_name != old_tool_name;

        if !title.is_undefined() {
            self.title = title.take().map(SharedString::from);
            if self.kind() == &acp_v2::ToolKind::Execute
                && let Some(title) = self.effective_title()
            {
                // A missing tool title must not overwrite an actual terminal command.
                for terminal in self.terminals() {
                    if legacy_terminal_labels || terminal.read(cx).is_process_backed() {
                        terminal.update(cx, |terminal, cx| {
                            terminal.update_command_label(title, cx);
                        });
                    }
                }
            }
        }
        if label_changed {
            let is_plain_text =
                self.effective_title().is_none() || self.kind() == &acp_v2::ToolKind::Execute;
            if was_plain_text != is_plain_text {
                self.label = Self::new_label(
                    self.effective_title(),
                    self.tool_name.as_ref(),
                    self.kind(),
                    language_registry.clone(),
                    cx,
                );
            } else {
                let text =
                    Self::label_text(self.effective_title(), self.tool_name.as_ref(), self.kind());
                if self.label.read(cx).source() != &text {
                    self.label.update(cx, |label, cx| label.replace(text, cx));
                }
            }
        }

        if let Some(content) = content {
            let new_content_len = content.len();
            let mut content = content.into_iter();

            for (old, new) in self.structured_content.iter_mut().zip(content.by_ref()) {
                old.update_from_prepared(new, &language_registry, cx);
            }
            for new in content {
                self.structured_content.push(ToolCallContent::from_prepared(
                    new,
                    &language_registry,
                    cx,
                ));
            }
            self.structured_content.truncate(new_content_len);
        }

        if !locations.is_undefined() {
            let locations = locations.take().unwrap_or_default();
            if self.locations != locations {
                self.locations = locations;
                self.resolved_locations.clear();
            }
        }

        if !raw_input.is_undefined() {
            let raw_input = raw_input.take();
            if self.raw_input != raw_input {
                match (
                    self.raw_input_markdown.as_ref(),
                    raw_input.as_ref().and_then(raw_output_text),
                ) {
                    (Some(markdown), Some(text)) => update_markdown_in_place(markdown, &text, cx),
                    (_, Some(text)) => {
                        self.raw_input_markdown = Some(cx.new(|cx| {
                            Markdown::new(text.into(), Some(language_registry.clone()), None, cx)
                        }));
                    }
                    (_, None) => self.raw_input_markdown = None,
                }
                self.raw_input = raw_input;
            }
        }

        if !raw_output.is_undefined() {
            self.raw_output = raw_output.take();
        }
        if output_changed {
            self.update_raw_output_content(&language_registry, cx);
        }
        Ok(())
    }

    fn append_content(
        &mut self,
        content: PreparedToolCallContent,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        self.structured_content.push(ToolCallContent::from_prepared(
            content,
            language_registry,
            cx,
        ));
        self.update_raw_output_content(language_registry, cx);
    }

    fn update_raw_output_content(
        &mut self,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        if !self.structured_content.is_empty() {
            self.raw_output_content = None;
            return;
        }
        let Some(text) = self.raw_output.as_ref().and_then(raw_output_text) else {
            self.raw_output_content = None;
            return;
        };
        if let Some(markdown) = self
            .raw_output_content
            .as_deref()
            .and_then(ToolCallContent::markdown)
        {
            update_markdown_in_place(markdown, &text, cx);
        } else {
            let markdown =
                cx.new(|cx| Markdown::new(text.into(), Some(language_registry.clone()), None, cx));
            self.raw_output_content = Some(Box::new(ToolCallContent::ContentBlock {
                block: ContentBlock::from_markdown(markdown),
                meta: None,
            }));
        }
    }

    pub fn content(&self) -> &[ToolCallContent] {
        if self.structured_content.is_empty() {
            self.raw_output_content
                .as_deref()
                .map(std::slice::from_ref)
                .unwrap_or_default()
        } else {
            &self.structured_content
        }
    }

    fn apply_reported_status(&mut self, status: MaybeUndefined<acp_v2::ToolCallStatus>) {
        match status {
            MaybeUndefined::Undefined => {}
            MaybeUndefined::Null => {
                // Clearing reported information does not resolve a local request
                // or undo a user's local decision.
                self.reported_status = None;
            }
            MaybeUndefined::Value(status) => {
                if matches!(
                    status,
                    acp_v2::ToolCallStatus::Completed
                        | acp_v2::ToolCallStatus::Failed
                        | acp_v2::ToolCallStatus::Cancelled
                ) {
                    self.authorization = None;
                }
                self.reported_status = Some(status);
                self.local_status = None;
            }
        }
    }

    pub fn diffs(&self) -> impl Iterator<Item = &Entity<Diff>> {
        self.structured_content
            .iter()
            .filter_map(|content| match content {
                ToolCallContent::Diff(diff) | ToolCallContent::LegacyDiff { diff, .. } => {
                    Some(diff)
                }
                _ => None,
            })
    }

    pub fn terminals(&self) -> impl Iterator<Item = &Entity<Terminal>> {
        self.structured_content
            .iter()
            .filter_map(|content| match content {
                ToolCallContent::Terminal { terminal, .. } => Some(terminal),
                _ => None,
            })
    }

    pub fn is_subagent(&self) -> bool {
        self.tool_name.as_ref().is_some_and(|s| s == "spawn_agent")
            || self.subagent_session_info.is_some()
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        let label = self.label.read(cx).source();
        let label = if self.effective_title().is_none() {
            MarkdownEscaped(label).to_string()
        } else {
            label.to_string()
        };
        let mut markdown = format!("**Tool Call: {}**\nStatus: {}\n\n", label, self.status());
        for content in self.content() {
            markdown.push_str(content.to_markdown(cx).as_str());
            markdown.push_str("\n\n");
        }
        markdown
    }

    async fn resolve_location(
        location: ToolCallLocation,
        project: WeakEntity<Project>,
        cx: &mut AsyncApp,
    ) -> Option<ResolvedLocation> {
        let buffer = project
            .update(cx, |project, cx| {
                if let Some(path) = project.project_path_for_absolute_path(&location.path, cx) {
                    Some(project.open_buffer(path, cx))
                } else if is_absolute(
                    location.path.to_string_lossy().as_ref(),
                    project.path_style(cx),
                ) {
                    Some(project.open_local_buffer(&location.path, cx))
                } else {
                    None
                }
            })
            .ok()??;
        let buffer = buffer.await.log_err()?;
        let position = buffer.update(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            if let Some(row) = location.line {
                let column = snapshot.indent_size_for_line(row).len;
                let point = snapshot.clip_point(Point::new(row, column), Bias::Left);
                snapshot.anchor_before(point)
            } else {
                Anchor::min_for_buffer(snapshot.remote_id())
            }
        });

        Some(ResolvedLocation { buffer, position })
    }

    fn resolve_locations(
        &self,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Vec<Option<ResolvedLocation>>> {
        let locations = self.locations.clone();
        project.update(cx, |_, cx| {
            cx.spawn(async move |project, cx| {
                let mut new_locations = Vec::new();
                for location in locations {
                    new_locations.push(Self::resolve_location(location, project.clone(), cx).await);
                }
                new_locations
            })
        })
    }
}

// Holds the buffer alive until resolution finishes: `shared_buffers`
// and `AgentLocation` only keep weak handles.
#[derive(Clone, Debug, PartialEq, Eq)]
struct ResolvedLocation {
    buffer: Entity<Buffer>,
    position: Anchor,
}

impl From<&ResolvedLocation> for AgentLocation {
    fn from(value: &ResolvedLocation) -> Self {
        Self {
            buffer: value.buffer.downgrade(),
            position: value.position,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectedPermissionParams {
    Terminal { patterns: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct SelectedPermissionOutcome {
    pub option_id: acp_v2::PermissionOptionId,
    pub option_kind: acp_v2::PermissionOptionKind,
    pub params: Option<SelectedPermissionParams>,
}

impl SelectedPermissionOutcome {
    pub fn new(
        option_id: acp_v2::PermissionOptionId,
        option_kind: acp_v2::PermissionOptionKind,
    ) -> Self {
        Self {
            option_id,
            option_kind,
            params: None,
        }
    }

    pub fn params(mut self, params: Option<SelectedPermissionParams>) -> Self {
        self.params = params;
        self
    }
}

#[derive(Clone, Debug)]
pub enum RequestPermissionOutcome {
    Cancelled,
    InterruptedByFollowUp,
    Selected(SelectedPermissionOutcome),
}

/// What a `WaitingForConfirmation` prompt represents semantically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationKind {
    /// The user is granting or denying permission for the tool call to
    /// proceed. The selected `PermissionOptionKind` determines whether the
    /// tool call transitions to `InProgress` (allow) or `Rejected` (reject).
    /// This is the default for tool authorization prompts.
    PermissionGrant,
    /// The user is choosing between actions for the tool to take next
    /// (for example, "Save" vs "Discard" before editing a dirty buffer).
    /// The tool call always transitions to `InProgress` regardless of the
    /// selected `PermissionOptionKind`; the caller interprets the chosen
    /// `option_id` to decide what to do.
    ActionChoice,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolCallStatus {
    /// The tool call hasn't started running yet, but we start showing it to
    /// the user.
    Pending,
    /// The tool call is waiting for confirmation from the user.
    WaitingForConfirmation,
    /// The tool call is currently running.
    InProgress,
    /// The tool call completed successfully.
    Completed,
    /// The tool call failed.
    Failed,
    /// The user rejected the tool call.
    Rejected,
    /// The user canceled generation so the tool call was canceled.
    Canceled,
}

impl From<acp_v1::ToolCallStatus> for ToolCallStatus {
    fn from(status: acp_v1::ToolCallStatus) -> Self {
        Self::from_reported(tool_status_from_v1(status).as_ref()).unwrap_or(Self::Pending)
    }
}

impl ToolCallStatus {
    fn from_reported(status: Option<&acp_v2::ToolCallStatus>) -> Option<Self> {
        match status? {
            acp_v2::ToolCallStatus::Pending => Some(Self::Pending),
            acp_v2::ToolCallStatus::InProgress => Some(Self::InProgress),
            acp_v2::ToolCallStatus::Completed => Some(Self::Completed),
            acp_v2::ToolCallStatus::Failed => Some(Self::Failed),
            acp_v2::ToolCallStatus::Cancelled => Some(Self::Canceled),
            _ => None,
        }
    }

    fn as_acp_status(&self) -> Option<acp_v2::ToolCallStatus> {
        match self {
            ToolCallStatus::Pending => Some(acp_v2::ToolCallStatus::Pending),
            ToolCallStatus::InProgress => Some(acp_v2::ToolCallStatus::InProgress),
            ToolCallStatus::Completed => Some(acp_v2::ToolCallStatus::Completed),
            ToolCallStatus::Failed => Some(acp_v2::ToolCallStatus::Failed),
            // A new authorization can retry a locally canceled tool; the
            // incoming request then supplies its continuation status.
            ToolCallStatus::Canceled => None,
            ToolCallStatus::WaitingForConfirmation | ToolCallStatus::Rejected => None,
        }
    }

    fn status_after_permission_grant(status: acp_v2::ToolCallStatus) -> ToolCallStatus {
        match Self::from_reported(Some(&status)).unwrap_or(Self::Pending) {
            ToolCallStatus::Pending => ToolCallStatus::InProgress,
            status => status,
        }
    }
}

impl Display for ToolCallStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToolCallStatus::Pending => "Pending",
                ToolCallStatus::WaitingForConfirmation => "Waiting for confirmation",
                ToolCallStatus::InProgress => "In Progress",
                ToolCallStatus::Completed => "Completed",
                ToolCallStatus::Failed => "Failed",
                ToolCallStatus::Rejected => "Rejected",
                ToolCallStatus::Canceled => "Canceled",
            }
        )
    }
}

fn elicitation_status_for_response(
    response: &acp_v2::CreateElicitationResponse,
) -> ElicitationStatus {
    match &response.action {
        acp_v2::ElicitationAction::Accept(_) => ElicitationStatus::Accepted,
        acp_v2::ElicitationAction::Decline => ElicitationStatus::Declined,
        acp_v2::ElicitationAction::Cancel => ElicitationStatus::Canceled,
        _ => ElicitationStatus::Canceled,
    }
}

#[derive(Debug)]
pub struct MessageContent {
    source_blocks: Vec<acp_v2::ContentBlock>,
    source_version: MessageContentVersion,
    blocks: Vec<RenderedMessageBlock>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MessageContentVersion(u64);

impl MessageContentVersion {
    fn next() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_VERSION: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_VERSION.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for MessageContent {
    fn default() -> Self {
        Self {
            source_blocks: Vec::new(),
            source_version: MessageContentVersion::next(),
            blocks: Vec::new(),
        }
    }
}

impl PartialEq for MessageContent {
    fn eq(&self, other: &Self) -> bool {
        self.source_blocks == other.source_blocks && self.blocks == other.blocks
    }
}

#[derive(Debug, PartialEq)]
struct RenderedMessageBlock {
    render: RenderBlock,
    source_index: Option<usize>,
}

enum DesiredMessageBlock {
    Markdown(String),
    Source(usize),
}

impl MessageContent {
    pub fn new(
        block: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        let mut content = Self::default();
        content.append(block, language_registry, path_style, cx);
        content
    }

    /// Original content blocks, including text not yet revealed by streaming.
    pub fn source_blocks(&self) -> &[acp_v2::ContentBlock] {
        &self.source_blocks
    }

    /// Lets readers retain local UI state without caching another copy of the source.
    /// Also distinguishes different content owners; independent of protocol message IDs.
    pub fn source_version(&self) -> MessageContentVersion {
        self.source_version
    }

    fn shrink_source_capacity(&mut self) {
        self.source_blocks.shrink_to_fit();
    }

    pub fn blocks(&self) -> impl ExactSizeIterator<Item = ContentBlockView<'_>> {
        self.blocks.iter().map(|block| ContentBlockView {
            render: &block.render,
            source: block
                .source_index
                .and_then(|index| self.source_blocks.get(index)),
        })
    }

    pub fn markdowns(&self) -> impl Iterator<Item = &Entity<Markdown>> {
        self.blocks().filter_map(|block| block.markdown())
    }

    pub fn visible_content(&self, cx: &App) -> bool {
        self.blocks().any(|block| block.visible_content(cx))
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        self.blocks()
            .map(|block| block.to_markdown(cx))
            .filter(|text| !text.is_empty())
            .join("\n\n")
    }

    fn trailing_text(&self) -> Option<&Entity<Markdown>> {
        match &self.blocks.last()?.render {
            RenderBlock::Markdown { markdown } => Some(markdown),
            _ => None,
        }
    }

    pub fn append(
        &mut self,
        block: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) {
        self.append_rendered(&block, language_registry, path_style, cx);
        self.source_blocks.push(block);
        self.source_version = MessageContentVersion::next();
    }

    fn append_compaction_summary(
        &mut self,
        block: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        // Compaction links remain separate blocks rather than joining streamed message text.
        if let acp_v2::ContentBlock::Text(text) = &block {
            if let Some(markdown) = self.trailing_text() {
                markdown.update(cx, |markdown, cx| markdown.append(&text.text, cx));
            } else {
                self.blocks.push(RenderedMessageBlock {
                    render: RenderBlock::Markdown {
                        markdown: ContentBlock::create_markdown(
                            text.text.clone(),
                            language_registry,
                            cx,
                        ),
                    },
                    source_index: None,
                });
            }
        } else {
            self.blocks.push(RenderedMessageBlock {
                render: ContentBlock::render_from_source(&block, language_registry, cx),
                source_index: Some(self.source_blocks.len()),
            });
        }
        self.source_blocks.push(block);
        self.source_version = MessageContentVersion::next();
    }

    fn append_rendered(
        &mut self,
        block: &acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) {
        if let Some(text) = Self::inline_text(block, path_style, false, self.blocks.is_empty()) {
            self.append_text(&text, language_registry, cx);
        } else {
            let render = ContentBlock::render_from_source(block, language_registry, cx);
            let source_index = match render {
                RenderBlock::EmbeddedResource { .. }
                | RenderBlock::Unsupported { .. }
                | RenderBlock::Image { .. } => Some(self.source_blocks.len()),
                _ => None,
            };
            self.blocks.push(RenderedMessageBlock {
                render,
                source_index,
            });
        }
    }

    fn inline_text(
        block: &acp_v2::ContentBlock,
        path_style: PathStyle,
        prompt: bool,
        leading: bool,
    ) -> Option<Cow<'_, str>> {
        match block {
            acp_v2::ContentBlock::Text(text) => Some(Cow::Borrowed(&text.text)),
            acp_v2::ContentBlock::ResourceLink(resource) => {
                let mut text = ContentBlock::resource_link_md(&resource.uri, path_style);
                if leading {
                    // A leading link separates the next streamed text chunk.
                    text.push('\n');
                }
                Some(Cow::Owned(text))
            }
            acp_v2::ContentBlock::Resource(resource)
                if prompt
                    && matches!(
                        resource.resource,
                        acp_v2::EmbeddedResourceResource::TextResourceContents(_)
                    ) =>
            {
                Some(Cow::Owned(ContentBlock::embedded_resource_string_contents(
                    resource, path_style,
                )))
            }
            _ => None,
        }
    }

    fn append_text(&mut self, text: &str, language_registry: &Arc<LanguageRegistry>, cx: &mut App) {
        if text.is_empty() {
            return;
        }
        if let Some(markdown) = self.trailing_text() {
            markdown.update(cx, |markdown, cx| markdown.append(text, cx));
        } else {
            self.blocks.push(RenderedMessageBlock {
                render: RenderBlock::Markdown {
                    markdown: ContentBlock::create_markdown(text.to_owned(), language_registry, cx),
                },
                source_index: None,
            });
        }
    }

    fn append_deferred_text(&mut self, text: acp_v2::TextContent) -> usize {
        let index = self.source_blocks.len();
        self.source_blocks.push(acp_v2::ContentBlock::Text(text));
        self.source_version = MessageContentVersion::next();
        index
    }

    fn append_prompt(
        &mut self,
        block: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) {
        if let Some(text) = Self::inline_text(&block, path_style, true, self.blocks.is_empty()) {
            self.append_text(&text, language_registry, cx);
        } else {
            self.append_rendered(&block, language_registry, path_style, cx);
        }
        self.source_blocks.push(block);
        self.source_version = MessageContentVersion::next();
    }

    fn replace(
        &mut self,
        blocks: Vec<acp_v2::ContentBlock>,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) {
        self.replace_blocks(blocks, language_registry, path_style, false, cx);
    }

    fn replace_prompt(
        &mut self,
        blocks: Vec<acp_v2::ContentBlock>,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) {
        self.replace_blocks(blocks, language_registry, path_style, true, cx);
    }

    fn replace_blocks(
        &mut self,
        blocks: Vec<acp_v2::ContentBlock>,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        prompt: bool,
        cx: &mut App,
    ) {
        if self.source_blocks != blocks {
            self.source_version = MessageContentVersion::next();
        }
        let mut desired = Vec::<DesiredMessageBlock>::new();
        for (source_index, block) in blocks.iter().enumerate() {
            if let Some(text) = Self::inline_text(block, path_style, prompt, desired.is_empty()) {
                if text.is_empty() {
                    continue;
                }
                if let Some(DesiredMessageBlock::Markdown(previous)) = desired.last_mut() {
                    previous.push_str(&text);
                } else {
                    desired.push(DesiredMessageBlock::Markdown(text.into_owned()));
                }
            } else {
                desired.push(DesiredMessageBlock::Source(source_index));
            }
        }

        let previous_blocks = std::mem::take(&mut self.blocks);
        let previous_sources = std::mem::replace(&mut self.source_blocks, blocks);
        self.blocks = desired
            .into_iter()
            .enumerate()
            .map(|(render_index, desired)| {
                let previous = previous_blocks.get(render_index);
                match desired {
                    DesiredMessageBlock::Markdown(text) => {
                        let markdown = match previous.map(|block| &block.render) {
                            Some(RenderBlock::Markdown { markdown }) => {
                                update_markdown_in_place(markdown, &text, cx);
                                markdown.clone()
                            }
                            _ => ContentBlock::create_markdown(text, language_registry, cx),
                        };
                        RenderedMessageBlock {
                            render: RenderBlock::Markdown { markdown },
                            source_index: None,
                        }
                    }
                    DesiredMessageBlock::Source(source_index) => {
                        let source = &self.source_blocks[source_index];
                        let previous_source = previous
                            .and_then(|block| block.source_index)
                            .and_then(|index| previous_sources.get(index));
                        let render = previous
                            .and_then(|block| {
                                Self::reuse_render(&block.render, previous_source, source, cx)
                            })
                            .unwrap_or_else(|| {
                                ContentBlock::render_from_source(source, language_registry, cx)
                            });
                        RenderedMessageBlock {
                            render,
                            source_index: Some(source_index),
                        }
                    }
                }
            })
            .collect();
    }

    fn reuse_render(
        previous: &RenderBlock,
        previous_source: Option<&acp_v2::ContentBlock>,
        source: &acp_v2::ContentBlock,
        cx: &mut App,
    ) -> Option<RenderBlock> {
        if previous_source == Some(source) {
            return Some(previous.clone());
        }
        match (previous, source) {
            (RenderBlock::Image { .. }, _)
                if Self::image_data(previous_source?) == Self::image_data(source)
                    && Self::image_data(source).is_some() =>
            {
                Some(previous.clone())
            }
            (
                RenderBlock::EmbeddedResource {
                    markdown: Some(markdown),
                },
                acp_v2::ContentBlock::Resource(resource),
            ) if matches!(
                &resource.resource,
                acp_v2::EmbeddedResourceResource::TextResourceContents(_)
            ) =>
            {
                let acp_v2::EmbeddedResourceResource::TextResourceContents(text) =
                    &resource.resource
                else {
                    return None;
                };
                update_markdown_in_place(markdown, &ContentBlock::text_resource_markdown(text), cx);
                Some(previous.clone())
            }
            (
                RenderBlock::EmbeddedResource { markdown: None },
                acp_v2::ContentBlock::Resource(resource),
            ) if matches!(
                &resource.resource,
                acp_v2::EmbeddedResourceResource::BlobResourceContents(_)
            ) && Self::image_data(source).is_none() =>
            {
                Some(previous.clone())
            }
            (RenderBlock::Unsupported { .. }, acp_v2::ContentBlock::Image(_))
                if matches!(previous_source, Some(acp_v2::ContentBlock::Image(_)))
                    && Self::image_data(previous_source?) == Self::image_data(source) =>
            {
                Some(previous.clone())
            }
            (RenderBlock::Unsupported { .. }, acp_v2::ContentBlock::Audio(_))
                if matches!(previous_source, Some(acp_v2::ContentBlock::Audio(_))) =>
            {
                Some(previous.clone())
            }
            (RenderBlock::Unsupported { .. }, acp_v2::ContentBlock::Other(_))
                if matches!(previous_source, Some(acp_v2::ContentBlock::Other(_))) =>
            {
                Some(previous.clone())
            }
            _ => None,
        }
    }

    fn image_data(block: &acp_v2::ContentBlock) -> Option<(&str, &str)> {
        match block {
            acp_v2::ContentBlock::Image(image) => Some((&image.data, image.mime_type.as_ref())),
            acp_v2::ContentBlock::Resource(resource) => match &resource.resource {
                acp_v2::EmbeddedResourceResource::BlobResourceContents(blob) => {
                    Some((&blob.blob, blob.mime_type.as_ref()?.as_ref()))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContentBlock {
    render: RenderBlock,
    source: Option<acp_v2::ContentBlock>,
}

#[derive(Debug, PartialEq, Clone)]
enum RenderBlock {
    Markdown {
        markdown: Entity<Markdown>,
    },
    EmbeddedResource {
        markdown: Option<Entity<Markdown>>,
    },
    ResourceLink,
    Image {
        image: Arc<gpui::Image>,
        dimensions: Option<gpui::Size<u32>>,
    },
    Unsupported {
        markdown: Entity<Markdown>,
    },
}

#[derive(Clone, Copy)]
pub struct ContentBlockView<'a> {
    render: &'a RenderBlock,
    source: Option<&'a acp_v2::ContentBlock>,
}

impl ContentBlock {
    pub fn from_markdown(markdown: Entity<Markdown>) -> Self {
        Self {
            render: RenderBlock::Markdown { markdown },
            source: None,
        }
    }

    #[cfg(test)]
    fn plain_markdown(&self) -> Option<&Entity<Markdown>> {
        match &self.render {
            RenderBlock::Markdown { markdown } => Some(markdown),
            _ => None,
        }
    }

    #[cfg(test)]
    fn unsupported_content(&self) -> Option<&acp_v2::ContentBlock> {
        self.as_view().unsupported_content()
    }

    pub fn new_output(
        block: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match block {
            acp_v2::ContentBlock::Text(text) => {
                Self::create_markdown_block(text.text, language_registry, cx)
            }
            block => {
                let render = Self::render_from_source(&block, language_registry, cx);
                let source = match render {
                    RenderBlock::EmbeddedResource { .. }
                    | RenderBlock::ResourceLink
                    | RenderBlock::Unsupported { .. } => Some(block),
                    RenderBlock::Markdown { .. } | RenderBlock::Image { .. } => None,
                };
                Self { render, source }
            }
        }
    }

    fn new_tool_content(
        block: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let render = Self::render_from_source(&block, language_registry, cx);
        Self {
            render,
            source: Some(block),
        }
    }

    fn update_tool_content(
        &mut self,
        block: acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        let reused = match (self.source.as_ref(), &block) {
            (Some(acp_v2::ContentBlock::Text(old)), acp_v2::ContentBlock::Text(new))
                if matches!(self.render, RenderBlock::Markdown { .. }) =>
            {
                if old.text != new.text {
                    self.update_text_in_place(&block, cx);
                }
                true
            }
            (Some(acp_v2::ContentBlock::Image(old)), acp_v2::ContentBlock::Image(new)) => {
                old.data == new.data && old.mime_type == new.mime_type
            }
            (Some(acp_v2::ContentBlock::Resource(old)), acp_v2::ContentBlock::Resource(new)) => {
                match (&old.resource, &new.resource) {
                    (
                        acp_v2::EmbeddedResourceResource::TextResourceContents(_),
                        acp_v2::EmbeddedResourceResource::TextResourceContents(new),
                    ) => {
                        if let RenderBlock::EmbeddedResource {
                            markdown: Some(markdown),
                        } = &self.render
                        {
                            update_markdown_in_place(
                                markdown,
                                &Self::text_resource_markdown(new),
                                cx,
                            );
                            true
                        } else {
                            false
                        }
                    }
                    (
                        acp_v2::EmbeddedResourceResource::BlobResourceContents(old),
                        acp_v2::EmbeddedResourceResource::BlobResourceContents(new),
                    ) => old.blob == new.blob && old.mime_type == new.mime_type,
                    _ => false,
                }
            }
            (Some(acp_v2::ContentBlock::Audio(_)), acp_v2::ContentBlock::Audio(_))
            | (Some(acp_v2::ContentBlock::Other(_)), acp_v2::ContentBlock::Other(_)) => true,
            (Some(previous), _) => previous == &block,
            _ => false,
        };
        if !reused {
            self.render = Self::render_from_source(&block, language_registry, cx);
        }
        self.source = Some(block);
    }

    fn render_from_source(
        block: &acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> RenderBlock {
        match block {
            acp_v2::ContentBlock::Resource(resource) => {
                if let Some((image, dimensions)) = Self::decode_embedded_resource_image(resource) {
                    return RenderBlock::Image { image, dimensions };
                }
                if matches!(
                    &resource.resource,
                    acp_v2::EmbeddedResourceResource::TextResourceContents(_)
                        | acp_v2::EmbeddedResourceResource::BlobResourceContents(_)
                ) {
                    let markdown =
                        Self::embedded_resource_markdown(resource, language_registry, cx);
                    RenderBlock::EmbeddedResource { markdown }
                } else {
                    Self::unsupported(block, language_registry, cx)
                }
            }
            acp_v2::ContentBlock::Image(image) => {
                if let Some((image, dimensions)) = Self::decode_image(image) {
                    RenderBlock::Image { image, dimensions }
                } else {
                    RenderBlock::Unsupported {
                        markdown: Self::create_markdown(
                            "Image content could not be displayed.".into(),
                            language_registry,
                            cx,
                        ),
                    }
                }
            }
            acp_v2::ContentBlock::ResourceLink(_) => RenderBlock::ResourceLink,
            acp_v2::ContentBlock::Text(text) => RenderBlock::Markdown {
                markdown: Self::create_markdown(text.text.clone(), language_registry, cx),
            },
            acp_v2::ContentBlock::Other(_) => Self::unsupported(block, language_registry, cx),
            _ => Self::unsupported(block, language_registry, cx),
        }
    }

    fn unsupported(
        content: &acp_v2::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> RenderBlock {
        let description = match content {
            acp_v2::ContentBlock::Audio(_) => "Audio content is not supported.",
            acp_v2::ContentBlock::Other(_) => "Unknown content type is not supported.",
            _ => "This content is not supported.",
        };
        RenderBlock::Unsupported {
            markdown: Self::create_markdown(description.into(), language_registry, cx),
        }
    }

    /// Updates a Markdown block in place from a streaming text `block`, reusing
    /// the existing `Markdown` entity rather than recreating it. Appends only the
    /// new suffix when the update is a continuation (the common streaming case),
    /// otherwise re-sets the source. Returns `false` when an in-place update isn't
    /// applicable, so the caller can fall back to replacing the block wholesale.
    ///
    /// Recreating the entity on every streamed snapshot causes the rendered
    /// element to tear down and rebuild, which flickers badly.
    pub fn update_text_in_place(&mut self, block: &acp_v2::ContentBlock, cx: &mut App) -> bool {
        let RenderBlock::Markdown { markdown } = &self.render else {
            return false;
        };
        let acp_v2::ContentBlock::Text(text_content) = block else {
            return false;
        };
        update_markdown_in_place(markdown, &text_content.text, cx);
        true
    }

    fn decode_image(
        image_content: &acp_v2::ImageContent,
    ) -> Option<(Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        Self::decode_image_data(&image_content.data, image_content.mime_type.as_ref())
    }

    fn decode_embedded_resource_image(
        resource: &acp_v2::EmbeddedResource,
    ) -> Option<(Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        let acp_v2::EmbeddedResourceResource::BlobResourceContents(blob) = &resource.resource
        else {
            return None;
        };
        let mime_type = blob.mime_type.as_ref()?.as_ref();
        Self::decode_image_data(&blob.blob, mime_type)
    }

    fn decode_image_data(
        data: &str,
        mime_type: &str,
    ) -> Option<(Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        use base64::Engine as _;

        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data.as_bytes())
            .ok()?;
        let format = gpui::ImageFormat::from_mime_type(mime_type)?;
        let dimensions = Self::image_dimensions(&bytes, format);
        Some((Arc::new(gpui::Image::from_bytes(format, bytes)), dimensions))
    }

    fn image_dimensions(bytes: &[u8], format: gpui::ImageFormat) -> Option<gpui::Size<u32>> {
        let format = match format {
            gpui::ImageFormat::Png => image::ImageFormat::Png,
            gpui::ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            gpui::ImageFormat::Webp => image::ImageFormat::WebP,
            gpui::ImageFormat::Gif => image::ImageFormat::Gif,
            gpui::ImageFormat::Svg => return None,
            gpui::ImageFormat::Bmp => image::ImageFormat::Bmp,
            gpui::ImageFormat::Tiff => image::ImageFormat::Tiff,
            gpui::ImageFormat::Ico => image::ImageFormat::Ico,
            gpui::ImageFormat::Pnm => image::ImageFormat::Pnm,
        };

        image::ImageReader::with_format(std::io::Cursor::new(bytes), format)
            .into_dimensions()
            .ok()
            .map(|(width, height)| gpui::Size { width, height })
    }

    fn create_markdown_block(
        content: String,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> ContentBlock {
        ContentBlock {
            render: RenderBlock::Markdown {
                markdown: Self::create_markdown(content, language_registry, cx),
            },
            source: None,
        }
    }

    fn create_markdown(
        content: String,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Entity<Markdown> {
        cx.new(|cx| {
            Markdown::new_with_options(
                content.into(),
                Some(language_registry.clone()),
                None,
                MarkdownOptions {
                    render_mermaid_diagrams: true,
                    render_metadata_blocks: true,
                    ..Default::default()
                },
                cx,
            )
        })
    }

    fn embedded_resource_markdown(
        resource: &acp_v2::EmbeddedResource,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Option<Entity<Markdown>> {
        match &resource.resource {
            acp_v2::EmbeddedResourceResource::TextResourceContents(text) => Some(
                Self::create_markdown(Self::text_resource_markdown(text), language_registry, cx),
            ),
            acp_v2::EmbeddedResourceResource::BlobResourceContents(_) => None,
            _ => None,
        }
    }

    fn text_resource_markdown(resource: &acp_v2::TextResourceContents) -> String {
        match text_resource_render_mode(resource.mime_type.as_ref().map(AsRef::as_ref)) {
            TextResourceRenderMode::Markdown => resource.text.clone(),
            TextResourceRenderMode::CodeBlock(language) => {
                Self::fenced_code_block(&resource.text, language)
            }
        }
    }

    pub fn as_view(&self) -> ContentBlockView<'_> {
        ContentBlockView {
            render: &self.render,
            source: self.source.as_ref(),
        }
    }

    pub fn text_content<'a>(&'a self, cx: &'a App) -> Option<&'a str> {
        self.as_view().text_content(cx)
    }

    pub fn visible_content(&self, cx: &App) -> bool {
        self.as_view().visible_content(cx)
    }

    pub fn to_markdown<'a>(&'a self, cx: &'a App) -> &'a str {
        self.as_view().to_markdown(cx)
    }

    pub fn markdown(&self) -> Option<&Entity<Markdown>> {
        self.as_view().markdown()
    }

    pub fn image(&self) -> Option<(&Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        self.as_view().image()
    }

    fn resource_link_md(uri: &str, path_style: PathStyle) -> String {
        if let Some(uri) = MentionUri::parse(uri, path_style).log_err() {
            uri.as_link().to_string()
        } else {
            uri.to_string()
        }
    }
}

impl<'a> ContentBlockView<'a> {
    #[cfg(test)]
    fn unsupported_content(&self) -> Option<&'a acp_v2::ContentBlock> {
        match self.render {
            RenderBlock::Unsupported { .. } => self.source,
            _ => None,
        }
    }

    pub fn text_content(&self, cx: &'a App) -> Option<&'a str> {
        match &self.render {
            RenderBlock::Markdown { markdown } | RenderBlock::Unsupported { markdown } => {
                Some(markdown.read(cx).source())
            }
            RenderBlock::EmbeddedResource { .. } => {
                self.embedded_resource()
                    .and_then(|(resource, _)| match &resource.resource {
                        acp_v2::EmbeddedResourceResource::TextResourceContents(text) => {
                            Some(text.text.as_str())
                        }
                        _ => None,
                    })
            }
            RenderBlock::ResourceLink | RenderBlock::Image { .. } => None,
        }
    }
}

impl ContentBlock {
    fn fenced_code_block(text: &str, language: Option<&str>) -> String {
        let fence_len = text
            .as_bytes()
            .chunk_by(|left, right| left == right)
            .filter(|chunk| chunk.first() == Some(&b'`'))
            .map(|chunk| chunk.len() + 1)
            .max()
            .unwrap_or(3)
            .max(3);
        let fence = "`".repeat(fence_len);

        let mut markdown = String::new();
        markdown.push_str(&fence);
        if let Some(language) = language {
            markdown.push_str(language);
        }
        markdown.push('\n');
        markdown.push_str(text);
        if !text.ends_with('\n') {
            markdown.push('\n');
        }
        markdown.push_str(&fence);
        markdown
    }

    fn embedded_resource_string_contents(
        resource: &acp_v2::EmbeddedResource,
        path_style: PathStyle,
    ) -> String {
        match &resource.resource {
            acp_v2::EmbeddedResourceResource::TextResourceContents(text) => {
                Self::resource_link_md(&text.uri, path_style)
            }
            acp_v2::EmbeddedResourceResource::BlobResourceContents(blob) => {
                Self::resource_link_md(&blob.uri, path_style)
            }
            _ => String::new(),
        }
    }

    fn embedded_resource_text(resource: &acp_v2::EmbeddedResource) -> &str {
        match &resource.resource {
            acp_v2::EmbeddedResourceResource::TextResourceContents(text) => &text.text,
            acp_v2::EmbeddedResourceResource::BlobResourceContents(blob) => &blob.uri,
            _ => "",
        }
    }

    fn embedded_resource_label(resource: &acp_v2::EmbeddedResource) -> &str {
        match &resource.resource {
            acp_v2::EmbeddedResourceResource::TextResourceContents(text) => &text.uri,
            acp_v2::EmbeddedResourceResource::BlobResourceContents(blob) => &blob.uri,
            _ => "",
        }
    }
}

impl<'a> ContentBlockView<'a> {
    pub fn embedded_resource(
        &self,
    ) -> Option<(&'a acp_v2::EmbeddedResource, Option<&'a Entity<Markdown>>)> {
        match &self.render {
            RenderBlock::EmbeddedResource { markdown } => match self.source {
                Some(acp_v2::ContentBlock::Resource(resource)) => {
                    Some((resource, markdown.as_ref()))
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub fn visible_content(&self, cx: &App) -> bool {
        match &self.render {
            RenderBlock::Markdown { markdown } | RenderBlock::Unsupported { markdown } => {
                !markdown.read(cx).source().trim().is_empty()
            }
            RenderBlock::EmbeddedResource { markdown, .. } => match markdown {
                Some(markdown) => !markdown.read(cx).source().trim().is_empty(),
                None => self.embedded_resource().is_some_and(|(resource, _)| {
                    !ContentBlock::embedded_resource_text(resource)
                        .trim()
                        .is_empty()
                }),
            },
            RenderBlock::ResourceLink | RenderBlock::Image { .. } => true,
        }
    }

    pub fn to_markdown(&self, cx: &'a App) -> &'a str {
        match &self.render {
            RenderBlock::Markdown { markdown } | RenderBlock::Unsupported { markdown } => {
                markdown.read(cx).source()
            }
            RenderBlock::EmbeddedResource { markdown, .. } => {
                if let Some(markdown) = markdown {
                    markdown.read(cx).source()
                } else {
                    self.embedded_resource().map_or("", |(resource, _)| {
                        ContentBlock::embedded_resource_label(resource)
                    })
                }
            }
            RenderBlock::ResourceLink => match self.source {
                Some(acp_v2::ContentBlock::ResourceLink(resource_link)) => &resource_link.uri,
                _ => "",
            },
            RenderBlock::Image { .. } => "`Image`",
        }
    }

    pub fn markdown(&self) -> Option<&'a Entity<Markdown>> {
        match &self.render {
            RenderBlock::Markdown { markdown } | RenderBlock::Unsupported { markdown } => {
                Some(markdown)
            }
            RenderBlock::EmbeddedResource { markdown, .. } => markdown.as_ref(),
            RenderBlock::ResourceLink | RenderBlock::Image { .. } => None,
        }
    }

    pub fn resource_link(&self) -> Option<&'a acp_v2::ResourceLink> {
        match &self.render {
            RenderBlock::ResourceLink => match self.source {
                Some(acp_v2::ContentBlock::ResourceLink(resource_link)) => Some(resource_link),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn image(&self) -> Option<(&'a Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        match &self.render {
            RenderBlock::Image { image, dimensions } => Some((image, *dimensions)),
            _ => None,
        }
    }
}

enum TextResourceRenderMode {
    Markdown,
    CodeBlock(Option<&'static str>),
}

fn text_resource_render_mode(mime_type: Option<&str>) -> TextResourceRenderMode {
    let Some(mime_type) = mime_type else {
        return TextResourceRenderMode::CodeBlock(None);
    };
    let Ok(mime) = mime_type.parse::<mime::Mime>() else {
        return TextResourceRenderMode::CodeBlock(None);
    };

    let type_ = mime.type_().as_str();
    let subtype = mime.subtype().as_str();
    let suffix = mime.suffix().map(|suffix| suffix.as_str());

    if matches!(
        (type_, subtype),
        ("text", "markdown") | ("text", "x-markdown")
    ) {
        return TextResourceRenderMode::Markdown;
    }

    let language = match (type_, subtype, suffix) {
        (_, "json", _) | (_, _, Some("json")) => Some("json"),
        (_, "xml", _) | (_, _, Some("xml")) => Some("xml"),
        ("text", "html", _) => Some("html"),
        ("text", "css", _) => Some("css"),
        ("text", "csv", _) => Some("csv"),
        ("text", "tab-separated-values", _) => Some("tsv"),
        ("text", "javascript", _) | ("application", "javascript", _) => Some("javascript"),
        ("application", "x-javascript", _) => Some("javascript"),
        ("text", "typescript", _) | ("application", "typescript", _) => Some("typescript"),
        ("text", "x-shellscript", _) | ("application", "x-shellscript", _) => Some("sh"),
        ("application", "x-sh", _) => Some("sh"),
        ("text", "x-python", _) => Some("python"),
        ("text", "x-rust", _) => Some("rust"),
        ("text", "x-go", _) => Some("go"),
        ("text", "x-ruby", _) => Some("ruby"),
        ("text", "x-c", _) => Some("c"),
        // `mime` parses `text/x-c++` as subtype `x-c+` with an empty suffix.
        ("text", "x-c+", Some("")) => Some("cpp"),
        ("text", "plain", _) => None,
        ("text", _, _) => None,
        ("application", "graphql", _) => Some("graphql"),
        ("application", "toml", _) => Some("toml"),
        ("application", "yaml", _) | ("application", "x-yaml", _) => Some("yaml"),
        (_, _, Some("yaml" | "yml")) => Some("yaml"),
        _ => return TextResourceRenderMode::CodeBlock(None),
    };

    TextResourceRenderMode::CodeBlock(language)
}

#[derive(Debug)]
pub enum ToolCallContent {
    ContentBlock {
        block: ContentBlock,
        meta: Option<acp_v2::Meta>,
    },
    Diff(Entity<Diff>),
    LegacyDiff {
        source: acp_v1::Diff,
        diff: Entity<Diff>,
    },
    Terminal {
        terminal: Entity<Terminal>,
        meta: Option<acp_v2::Meta>,
    },
    DiffPatch {
        source: acp_v2::Diff,
        render: DiffPatch,
    },
    Other {
        source: acp_v2::ToolCallContent,
        markdown: Entity<Markdown>,
    },
}

enum PreparedToolCallContent {
    ContentBlock(acp_v2::Content),
    LegacyDiff(acp_v1::Diff),
    Terminal {
        terminal: Entity<Terminal>,
        meta: Option<acp_v2::Meta>,
    },
    DiffPatch(acp_v2::Diff),
    Other(acp_v2::ToolCallContent),
}

impl PreparedToolCallContent {
    fn prepare(
        content: Vec<acp_v1::ToolCallContent>,
        terminals: ToolTerminalResolver<'_>,
        cx: &App,
    ) -> Result<Vec<Self>> {
        let mut prepared = Vec::with_capacity(content.len());
        for content in content {
            let content = match content {
                acp_v1::ToolCallContent::Content(acp_v1::Content { content, meta, .. }) => {
                    Self::ContentBlock(acp_v2::Content::new(content::from_v1(content)?).meta(meta))
                }
                acp_v1::ToolCallContent::Diff(diff) => Self::LegacyDiff(diff),
                acp_v1::ToolCallContent::Terminal(acp_v1::Terminal {
                    terminal_id, meta, ..
                }) => Self::Terminal {
                    terminal: terminals.resolve(&acp_v2::TerminalId::new(terminal_id.0), cx)?,
                    meta,
                },
                _ => continue,
            };
            prepared.push(content);
        }
        Ok(prepared)
    }

    fn prepare_v2(
        content: Vec<acp_v2::ToolCallContent>,
        terminals: ToolTerminalResolver<'_>,
        cx: &App,
    ) -> Result<Vec<Self>> {
        content
            .into_iter()
            .map(|content| Self::from_v2(content, terminals, cx))
            .collect()
    }

    fn from_v2(
        content: acp_v2::ToolCallContent,
        terminals: ToolTerminalResolver<'_>,
        cx: &App,
    ) -> Result<Self> {
        match content {
            acp_v2::ToolCallContent::Content(content) => Ok(Self::ContentBlock(*content)),
            acp_v2::ToolCallContent::Diff(diff) => Ok(Self::DiffPatch(diff)),
            acp_v2::ToolCallContent::Terminal(terminal) => Ok(Self::Terminal {
                terminal: terminals.resolve(&terminal.terminal_id, cx)?,
                meta: terminal.meta,
            }),
            other => Ok(Self::Other(other)),
        }
    }
}

impl ToolCallContent {
    fn patch_preview(diff: &acp_v2::Diff) -> String {
        match diff.patch.as_ref() {
            Some(patch) => MarkdownCodeBlock {
                tag: if patch.format == acp_v2::DiffPatchFormat::GitPatch {
                    "diff"
                } else {
                    ""
                },
                text: &patch.text,
            }
            .to_string(),
            None => "Diff patch preview unavailable.".to_string(),
        }
    }

    fn from_prepared(
        content: PreparedToolCallContent,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match content {
            PreparedToolCallContent::ContentBlock(content) => Self::ContentBlock {
                block: ContentBlock::new_tool_content(content.content, language_registry, cx),
                meta: content.meta,
            },
            PreparedToolCallContent::LegacyDiff(source) => {
                let diff = cx.new(|cx| {
                    Diff::finalized(
                        source.path.to_string_lossy().into_owned(),
                        source.old_text.clone(),
                        source.new_text.clone(),
                        language_registry.clone(),
                        cx,
                    )
                });
                Self::LegacyDiff { source, diff }
            }
            PreparedToolCallContent::Terminal { terminal, meta } => {
                Self::Terminal { terminal, meta }
            }
            PreparedToolCallContent::DiffPatch(source) => Self::DiffPatch {
                render: DiffPatch::new(&source, language_registry, cx),
                source,
            },
            PreparedToolCallContent::Other(source) => Self::Other {
                source,
                markdown: ContentBlock::create_markdown(
                    "Unsupported tool call content.".into(),
                    language_registry,
                    cx,
                ),
            },
        }
    }

    fn update_from_prepared(
        &mut self,
        new: PreparedToolCallContent,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        match (&mut *self, new) {
            (
                Self::ContentBlock { block, meta },
                PreparedToolCallContent::ContentBlock(content),
            ) => {
                block.update_tool_content(content.content, language_registry, cx);
                *meta = content.meta;
            }
            (Self::LegacyDiff { source, .. }, PreparedToolCallContent::LegacyDiff(new_source))
                if source.path == new_source.path
                    && source.old_text == new_source.old_text
                    && source.new_text == new_source.new_text =>
            {
                *source = new_source;
            }
            (
                Self::Terminal { terminal, meta },
                PreparedToolCallContent::Terminal {
                    terminal: new_terminal,
                    meta: new_meta,
                },
            ) if *terminal == new_terminal => *meta = new_meta,
            (
                Self::DiffPatch { source, render },
                PreparedToolCallContent::DiffPatch(new_source),
            ) => {
                if source.patch != new_source.patch
                    || source.changes.len() != new_source.changes.len()
                    || source
                        .changes
                        .iter()
                        .zip(&new_source.changes)
                        .any(|(old, new)| {
                            old.operation != new.operation || old.file_type != new.file_type
                        })
                {
                    *render = DiffPatch::new(&new_source, language_registry, cx);
                }
                *source = new_source;
            }
            (Self::Other { source, .. }, PreparedToolCallContent::Other(new_source)) => {
                *source = new_source;
            }
            (_, new) => *self = Self::from_prepared(new, language_registry, cx),
        }
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::ContentBlock { block, .. } => block.to_markdown(cx).to_string(),
            Self::Diff(diff) | Self::LegacyDiff { diff, .. } => diff.read(cx).to_markdown(cx),
            Self::Terminal { terminal, .. } => terminal.read(cx).to_markdown(cx),
            Self::DiffPatch { source, .. } => format!(
                "{}\n\n{}",
                source.changes.iter().map(diff_change_label).join("\n"),
                Self::patch_preview(source)
            ),
            Self::Other { markdown, .. } => markdown.read(cx).source().to_string(),
        }
    }

    pub fn markdown(&self) -> Option<&Entity<Markdown>> {
        match self {
            Self::ContentBlock { block, .. } => block.markdown(),
            Self::DiffPatch { render, .. } => render.fallback.as_ref(),
            Self::Other { markdown, .. } => Some(markdown),
            Self::Diff(_) | Self::LegacyDiff { .. } | Self::Terminal { .. } => None,
        }
    }

    pub fn image(&self) -> Option<(&Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        match self {
            Self::ContentBlock { block, .. } => block.image(),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ToolCallUpdate {
    V2(acp_v2::ToolCallUpdate),
    V1(acp_v1::ToolCallUpdate),
    UpdateDiff(ToolCallUpdateDiff),
    UpdateTerminal(ToolCallUpdateTerminal),
}

impl From<acp_v2::ToolCallUpdate> for ToolCallUpdate {
    fn from(update: acp_v2::ToolCallUpdate) -> Self {
        Self::V2(update)
    }
}

impl From<acp_v1::ToolCallUpdate> for ToolCallUpdate {
    fn from(update: acp_v1::ToolCallUpdate) -> Self {
        Self::V1(update)
    }
}

impl From<ToolCallUpdateDiff> for ToolCallUpdate {
    fn from(diff: ToolCallUpdateDiff) -> Self {
        Self::UpdateDiff(diff)
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolCallUpdateDiff {
    pub id: acp_v2::ToolCallId,
    pub diff: Entity<Diff>,
}

impl From<ToolCallUpdateTerminal> for ToolCallUpdate {
    fn from(terminal: ToolCallUpdateTerminal) -> Self {
        Self::UpdateTerminal(terminal)
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolCallUpdateTerminal {
    pub id: acp_v2::ToolCallId,
    pub terminal: Entity<Terminal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PlanIdentity {
    Legacy,
    Keyed(acp_v2::PlanId),
}

#[derive(Debug, Default)]
pub struct Plan {
    pub entries: Vec<PlanEntry>,
    pub meta: Option<acp_v2::Meta>,
    pub update_meta: Option<acp_v2::Meta>,
}

#[derive(Debug)]
pub struct PlanStats<'a> {
    pub in_progress_entry: Option<&'a PlanEntry>,
    pub pending: u32,
    pub completed: u32,
    pub cancelled: u32,
}

impl Plan {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn stats(&self) -> PlanStats<'_> {
        let mut stats = PlanStats {
            in_progress_entry: None,
            pending: 0,
            completed: 0,
            cancelled: 0,
        };

        for entry in &self.entries {
            match &entry.source.status {
                acp_v2::PlanEntryStatus::InProgress => {
                    stats.in_progress_entry = stats.in_progress_entry.or(Some(entry));
                    stats.pending += 1;
                }
                acp_v2::PlanEntryStatus::Completed => {
                    stats.completed += 1;
                }
                acp_v2::PlanEntryStatus::Cancelled => {
                    stats.cancelled += 1;
                }
                _ => stats.pending += 1,
            }
        }

        stats
    }

    fn replace(
        &mut self,
        entries: Vec<acp_v2::PlanEntry>,
        meta: Option<acp_v2::Meta>,
        update_meta: Option<acp_v2::Meta>,
        cx: &mut App,
    ) -> bool {
        let changed = self.entries.len() != entries.len()
            || self.entries.iter().zip(&entries).any(|(old, new)| {
                old.source.content != new.content
                    || old.source.priority != new.priority
                    || old.source.status != new.status
            });
        let new_length = entries.len();
        let mut entries = entries.into_iter();
        for (old, new) in self.entries.iter_mut().zip(entries.by_ref()) {
            if old.source.content != new.content {
                update_markdown_in_place(&old.content, &new.content, cx);
            }
            old.source = new;
        }
        self.entries
            .extend(entries.map(|source| PlanEntry::new(source, cx)));
        self.entries.truncate(new_length);
        self.meta = meta;
        self.update_meta = update_meta;
        changed
    }
}

#[derive(Debug)]
pub struct PlanEntry {
    pub source: acp_v2::PlanEntry,
    pub content: Entity<Markdown>,
}

impl PlanEntry {
    fn new(source: acp_v2::PlanEntry, cx: &mut App) -> Self {
        Self {
            content: cx.new(|cx| Markdown::new(source.content.clone().into(), None, None, cx)),
            source,
        }
    }
}

fn plan_entry_from_v1(entry: acp_v1::PlanEntry) -> Result<acp_v2::PlanEntry> {
    let priority = match entry.priority {
        acp_v1::PlanEntryPriority::High => acp_v2::PlanEntryPriority::High,
        acp_v1::PlanEntryPriority::Medium => acp_v2::PlanEntryPriority::Medium,
        acp_v1::PlanEntryPriority::Low => acp_v2::PlanEntryPriority::Low,
        // V2 preserves future wire names through its Other variant.
        other => serde_json::from_value(serde_json::to_value(other)?)?,
    };
    let status = match entry.status {
        acp_v1::PlanEntryStatus::Pending => acp_v2::PlanEntryStatus::Pending,
        acp_v1::PlanEntryStatus::InProgress => acp_v2::PlanEntryStatus::InProgress,
        acp_v1::PlanEntryStatus::Completed => acp_v2::PlanEntryStatus::Completed,
        other => serde_json::from_value(serde_json::to_value(other)?)?,
    };
    Ok(acp_v2::PlanEntry::new(entry.content, priority, status).meta(entry.meta))
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenUsage {
    pub max_tokens: u64,
    pub used_tokens: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub max_output_tokens: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SessionCost {
    pub amount: f64,
    pub currency: SharedString,
}

pub const TOKEN_USAGE_WARNING_THRESHOLD: f32 = 0.8;

impl TokenUsage {
    pub fn ratio(&self) -> TokenUsageRatio {
        #[cfg(debug_assertions)]
        let warning_threshold: f32 = std::env::var("ZED_THREAD_WARNING_THRESHOLD")
            .unwrap_or(TOKEN_USAGE_WARNING_THRESHOLD.to_string())
            .parse()
            .unwrap();
        #[cfg(not(debug_assertions))]
        let warning_threshold: f32 = TOKEN_USAGE_WARNING_THRESHOLD;

        // When the maximum is unknown because there is no selected model,
        // avoid showing the token limit warning.
        if self.max_tokens == 0 {
            TokenUsageRatio::Normal
        } else if self.used_tokens >= self.max_tokens {
            TokenUsageRatio::Exceeded
        } else if self.used_tokens as f32 / self.max_tokens as f32 >= warning_threshold {
            TokenUsageRatio::Warning
        } else {
            TokenUsageRatio::Normal
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenUsageRatio {
    Normal,
    Warning,
    Exceeded,
}

#[derive(Debug, Clone)]
pub struct RetryStatus {
    pub last_error: SharedString,
    pub attempt: usize,
    pub max_attempts: usize,
    pub started_at: Instant,
    pub duration: Duration,
    pub meta: Option<acp_v1::Meta>,
}

pub const REFUSAL_FALLBACK_MODEL_META_KEY: &str = "refusal_fallback_model";

pub fn meta_with_refusal_fallback(model_name: &str) -> acp_v1::Meta {
    acp_v1::Meta::from_iter([(REFUSAL_FALLBACK_MODEL_META_KEY.into(), model_name.into())])
}

pub fn refusal_fallback_model_from_meta(meta: &Option<acp_v1::Meta>) -> Option<SharedString> {
    meta.as_ref()
        .and_then(|m| m.get(REFUSAL_FALLBACK_MODEL_META_KEY))
        .and_then(|v| v.as_str())
        .map(|s| SharedString::from(s.to_owned()))
}

struct RunningTurn {
    id: u32,
    first_entry_index: usize,
    send_task: Task<()>,
}

fn stop_reason_from_v1(reason: &acp_v1::StopReason) -> Option<acp_v2::StopReason> {
    Some(match reason {
        acp_v1::StopReason::EndTurn => acp_v2::StopReason::EndTurn,
        acp_v1::StopReason::MaxTokens => acp_v2::StopReason::MaxTokens,
        acp_v1::StopReason::MaxTurnRequests => acp_v2::StopReason::MaxTurnRequests,
        acp_v1::StopReason::Refusal => acp_v2::StopReason::Refusal,
        acp_v1::StopReason::Cancelled => acp_v2::StopReason::Cancelled,
        _ => return None,
    })
}

pub struct AcpThread {
    session_info: AgentSessionInfo,
    parent_session_id: Option<acp_v2::SessionId>,
    provisional_title: Option<SharedString>,
    entries: Vec<AgentThreadEntry>,
    // Notices stay with the live session, but never enter conversation history or exports.
    notices: Vec<(usize, acp_v2::Notice)>,
    next_notice_id: usize,
    elicitations: ElicitationStore,
    permission_requests: IndexMap<PermissionRequestId, PermissionRequest>,
    plans: HashMap<PlanIdentity, Plan>,
    visible_plan: Option<PlanIdentity>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    _git_store_subscription: Subscription,
    update_last_checkpoint_if_changed_task: Option<Task<Result<()>>>,
    shared_buffers: HashMap<WeakEntity<Buffer>, BufferSnapshot>,
    turn_id: u32,
    running_turn: Option<RunningTurn>,
    connection: Rc<dyn AgentConnection>,
    submissions: SessionSubmissions,
    activity: SessionActivity,
    token_usage: Option<TokenUsage>,
    cost: Option<SessionCost>,
    prompt_capabilities: acp_v2::PromptCapabilities,
    available_commands: Vec<acp_v2::AvailableCommand>,
    _observe_prompt_capabilities: Task<anyhow::Result<()>>,
    _idle_sleep_subscriptions: Vec<Subscription>,
    terminals: HashMap<acp_v2::TerminalId, Entity<Terminal>>,
    pending_terminal_output: HashMap<acp_v2::TerminalId, Vec<Vec<u8>>>,
    pending_terminal_exit: HashMap<acp_v2::TerminalId, acp_v1::TerminalExitStatus>,
    had_error: bool,
    /// The user's unsent prompt text, persisted so it can be restored when reloading the thread.
    draft_prompt: Option<Vec<acp_v2::ContentBlock>>,
    /// Lets observers detect draft changes without comparing prompts.
    draft_prompt_revision: usize,
    /// The initial scroll position for the thread view, set during session registration.
    ui_scroll_position: Option<gpui::ListOffset>,
    /// A cursor over retained source, rather than a second text store, lets the UI
    /// reveal text gradually without changing the authoritative message.
    streaming_text_buffer: Option<StreamingTextBuffer>,
    idle_sleep_prevention: IdleSleepPrevention,
}

enum IdleSleepPrevention {
    Inactive,
    Acquiring { _task: Task<()> },
    Active { _guard: ActivityGuard },
    Failed,
}

#[derive(Default)]
struct TextCursor {
    source_index: usize,
    byte_offset: usize,
    pending_bytes: usize,
}

impl TextCursor {
    fn take(&mut self, sources: &[acp_v2::ContentBlock], max_bytes: usize) -> Option<String> {
        let max_bytes = max_bytes.min(self.pending_bytes);
        let mut revealed = String::with_capacity(max_bytes);
        while revealed.len() < max_bytes {
            let acp_v2::ContentBlock::Text(text) = sources.get(self.source_index)? else {
                return None;
            };
            let pending = text.text.get(self.byte_offset..)?;
            let boundary = pending.ceil_char_boundary(max_bytes - revealed.len());
            self.pending_bytes = self.pending_bytes.checked_sub(boundary)?;
            revealed.push_str(&pending[..boundary]);
            self.byte_offset += boundary;
            if self.byte_offset == text.text.len() {
                self.source_index += 1;
                self.byte_offset = 0;
            }
        }
        Some(revealed)
    }
}

#[derive(PartialEq)]
struct StreamingTextTarget {
    entry_index: usize,
    chunk_index: usize,
    markdown: Entity<Markdown>,
}

impl StreamingTextTarget {
    fn content<'a>(&self, entries: &'a [AgentThreadEntry]) -> Option<&'a MessageContent> {
        let AgentThreadEntry::AssistantMessage(message) = entries.get(self.entry_index)? else {
            return None;
        };
        let (AssistantMessageChunk::Message { block, .. }
        | AssistantMessageChunk::Thought { block, .. }) = message.chunks.get(self.chunk_index)?;
        // A removed entry's position can be reused before the next reveal tick.
        (block.trailing_text()? == &self.markdown).then_some(block)
    }
}

struct StreamingTextBuffer {
    cursor: TextCursor,
    /// The number of bytes to reveal per timer turn.
    bytes_to_reveal_per_tick: usize,
    target: StreamingTextTarget,
    _reveal_task: Task<()>,
}

impl StreamingTextBuffer {
    /// The number of milliseconds between each timer tick, controlling how quickly
    /// text is revealed.
    const TASK_UPDATE_MS: u64 = 16;
    /// The time in milliseconds to reveal the entire pending text.
    const REVEAL_TARGET: f32 = 200.0;

    fn reveal(&mut self, entries: &[AgentThreadEntry], max_bytes: usize, cx: &mut App) -> bool {
        let Some(content) = self.target.content(entries) else {
            return false;
        };
        let Some(revealed) = self.cursor.take(content.source_blocks(), max_bytes) else {
            log::error!("Invalid source cursor for streaming agent text");
            return false;
        };
        if !revealed.is_empty() {
            self.target
                .markdown
                .update(cx, |markdown, cx| markdown.append(&revealed, cx));
        }
        true
    }
}

impl From<&AcpThread> for ActionLogTelemetry {
    fn from(value: &AcpThread) -> Self {
        Self {
            agent_telemetry_id: value.connection().telemetry_id(),
            session_id: value.session_id().0.clone(),
        }
    }
}

#[derive(Debug)]
pub enum AcpThreadEvent {
    StatusChanged,
    SubmissionUpdated(SubmissionId),
    PromptUpdated,
    NewEntry,
    TitleUpdated,
    NoticesUpdated,
    TokenUsageUpdated,
    EntryUpdated(usize),
    EntriesRemoved(Range<usize>),
    ToolAuthorizationRequested(PermissionRequestId),
    ToolAuthorizationReceived(PermissionRequestId),
    ElicitationRequested(ElicitationEntryId),
    /// The request left `Pending`; this does not imply delivery to its response waiter.
    ElicitationResponded(ElicitationEntryId),
    Retry(RetryStatus),
    SubagentSpawned(acp_v2::SessionId),
    Stopped {
        activity_generation: u64,
        activity_duration: Option<Duration>,
        stop_reason: Option<acp_v2::StopReason>,
    },
    Error,
    LoadError(LoadError),
    PromptCapabilitiesUpdated,
    Refusal,
    AvailableCommandsUpdated(Vec<acp_v2::AvailableCommand>),
    ModeUpdated(acp_v1::SessionModeId),
    ConfigOptionsUpdated(Vec<acp_v2::SessionConfigOption>),
    WorkingDirectoriesUpdated,
}

impl EventEmitter<AcpThreadEvent> for AcpThread {}

#[derive(Debug, Clone)]
pub enum TerminalProviderEvent {
    Created {
        terminal_id: acp_v1::TerminalId,
        label: String,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        terminal: Entity<::terminal::Terminal>,
    },
    Output {
        terminal_id: acp_v1::TerminalId,
        data: Vec<u8>,
    },
    TitleChanged {
        terminal_id: acp_v1::TerminalId,
        title: String,
    },
    Exit {
        terminal_id: acp_v1::TerminalId,
        status: acp_v1::TerminalExitStatus,
    },
}

#[derive(Debug, Clone)]
pub enum TerminalProviderCommand {
    WriteInput {
        terminal_id: acp_v2::TerminalId,
        bytes: Vec<u8>,
    },
    Resize {
        terminal_id: acp_v2::TerminalId,
        cols: u16,
        rows: u16,
    },
    Close {
        terminal_id: acp_v2::TerminalId,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub enum ThreadStatus {
    Idle,
    Generating,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    Unsupported {
        command: SharedString,
        current_version: SharedString,
        minimum_version: SharedString,
    },
    FailedToInstall(SharedString),
    Exited {
        status: ExitStatus,
        stderr: Option<SharedString>,
    },
    Other(SharedString),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Unsupported {
                command: path,
                current_version,
                minimum_version,
            } => {
                write!(
                    f,
                    "version {current_version} from {path} is not supported (need at least {minimum_version})"
                )
            }
            LoadError::FailedToInstall(msg) => write!(f, "Failed to install: {msg}"),
            LoadError::Exited { status, .. } => write!(f, "Server exited with status {status}"),
            LoadError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl Error for LoadError {}

impl AcpThread {
    pub fn new(
        parent_session_id: Option<acp_v2::SessionId>,
        title: Option<SharedString>,
        work_dirs: Option<PathList>,
        connection: Rc<dyn AgentConnection>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        session_id: acp_v2::SessionId,
        mut prompt_capabilities_rx: watch::Receiver<acp_v2::PromptCapabilities>,
        cx: &mut Context<Self>,
    ) -> Self {
        let prompt_capabilities = prompt_capabilities_rx.borrow().clone();
        let task = cx.spawn::<_, anyhow::Result<()>>(async move |this, cx| {
            loop {
                let caps = prompt_capabilities_rx.recv().await?;
                this.update(cx, |this, cx| {
                    this.prompt_capabilities = caps;
                    cx.emit(AcpThreadEvent::PromptCapabilitiesUpdated);
                })?;
            }
        });
        let idle_sleep_settings_subscription = cx.observe_global::<SettingsStore>(|this, cx| {
            this.update_idle_sleep_prevention(cx);
        });
        let idle_sleep_event_subscription =
            cx.subscribe_self(|this, event: &AcpThreadEvent, cx| match event {
                AcpThreadEvent::StatusChanged
                | AcpThreadEvent::EntriesRemoved(_)
                | AcpThreadEvent::ToolAuthorizationRequested(_)
                | AcpThreadEvent::ToolAuthorizationReceived(_)
                | AcpThreadEvent::ElicitationRequested(_)
                | AcpThreadEvent::ElicitationResponded(_) => {
                    this.sync_legacy_action_state(cx);
                    this.update_idle_sleep_prevention(cx);
                }
                AcpThreadEvent::PromptUpdated
                | AcpThreadEvent::SubmissionUpdated(_)
                | AcpThreadEvent::NewEntry
                | AcpThreadEvent::TitleUpdated
                | AcpThreadEvent::NoticesUpdated
                | AcpThreadEvent::TokenUsageUpdated
                | AcpThreadEvent::EntryUpdated(_)
                | AcpThreadEvent::Retry(_)
                | AcpThreadEvent::SubagentSpawned(_)
                | AcpThreadEvent::Stopped { .. }
                | AcpThreadEvent::Error
                | AcpThreadEvent::LoadError(_)
                | AcpThreadEvent::PromptCapabilitiesUpdated
                | AcpThreadEvent::Refusal
                | AcpThreadEvent::AvailableCommandsUpdated(_)
                | AcpThreadEvent::ModeUpdated(_)
                | AcpThreadEvent::ConfigOptionsUpdated(_)
                | AcpThreadEvent::WorkingDirectoriesUpdated => {}
            });

        let git_store = project.read(cx).git_store().clone();
        let _git_store_subscription = cx.subscribe(&git_store, |this, _, event, cx| {
            if matches!(
                event,
                GitStoreEvent::RepositoryUpdated(
                    _,
                    RepositoryEvent::StatusesChanged | RepositoryEvent::HeadChanged,
                    _
                )
            ) {
                this.update_last_checkpoint_if_changed_task =
                    Some(this.update_last_checkpoint_if_changed(cx));
            }
        });

        let receipt_submissions = connection.receipt_submissions(&session_id, cx);
        Self {
            parent_session_id,
            session_info: AgentSessionInfo {
                session_id,
                work_dirs,
                title,
                updated_at: None,
                created_at: None,
                meta: None,
            },
            action_log,
            _git_store_subscription,
            update_last_checkpoint_if_changed_task: None,
            shared_buffers: Default::default(),
            entries: Default::default(),
            notices: Vec::new(),
            next_notice_id: 0,
            elicitations: ElicitationStore::default(),
            permission_requests: IndexMap::default(),
            plans: HashMap::default(),
            visible_plan: None,
            provisional_title: None,
            project,
            running_turn: None,
            turn_id: 0,
            submissions: SessionSubmissions::new(receipt_submissions),
            activity: SessionActivity::default(),
            connection,
            token_usage: None,
            cost: None,
            prompt_capabilities,
            available_commands: Vec::new(),
            _observe_prompt_capabilities: task,
            _idle_sleep_subscriptions: vec![
                idle_sleep_settings_subscription,
                idle_sleep_event_subscription,
            ],
            terminals: HashMap::default(),
            pending_terminal_output: HashMap::default(),
            pending_terminal_exit: HashMap::default(),
            had_error: false,
            draft_prompt: None,
            draft_prompt_revision: 0,
            ui_scroll_position: None,
            streaming_text_buffer: None,
            idle_sleep_prevention: IdleSleepPrevention::Inactive,
        }
    }

    pub fn parent_session_id(&self) -> Option<&acp_v2::SessionId> {
        self.parent_session_id.as_ref()
    }

    pub fn prompt_capabilities(&self) -> &acp_v2::PromptCapabilities {
        &self.prompt_capabilities
    }

    pub fn available_commands(&self) -> &[acp_v2::AvailableCommand] {
        &self.available_commands
    }

    pub fn update_available_commands(
        &mut self,
        commands: Vec<acp_v2::AvailableCommand>,
        cx: &mut Context<Self>,
    ) {
        self.available_commands = commands.clone();
        cx.emit(AcpThreadEvent::AvailableCommandsUpdated(commands));
    }

    pub fn is_draft_thread(&self) -> bool {
        self.entries().is_empty()
    }

    pub fn draft_prompt(&self) -> Option<&[acp_v2::ContentBlock]> {
        self.draft_prompt.as_deref()
    }

    pub fn draft_prompt_revision(&self) -> usize {
        self.draft_prompt_revision
    }

    pub fn set_draft_prompt(
        &mut self,
        prompt: Option<Vec<acp_v2::ContentBlock>>,
        cx: &mut Context<Self>,
    ) {
        cx.emit(AcpThreadEvent::PromptUpdated);
        self.draft_prompt = prompt;
        self.draft_prompt_revision += 1;
    }

    pub fn ui_scroll_position(&self) -> Option<gpui::ListOffset> {
        self.ui_scroll_position
    }

    pub fn set_ui_scroll_position(&mut self, position: Option<gpui::ListOffset>) {
        self.ui_scroll_position = position;
    }

    pub fn connection(&self) -> &Rc<dyn AgentConnection> {
        &self.connection
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn title(&self) -> Option<SharedString> {
        self.session_info
            .title
            .clone()
            .or_else(|| self.provisional_title.clone())
    }

    pub fn has_provisional_title(&self) -> bool {
        self.provisional_title.is_some()
    }

    pub fn entries(&self) -> &[AgentThreadEntry] {
        &self.entries
    }

    pub fn notices(&self) -> &[(usize, acp_v2::Notice)] {
        &self.notices
    }

    pub fn push_notice(&mut self, notice: acp_v2::Notice, cx: &mut Context<Self>) {
        let notice_id = self.next_notice_id;
        self.next_notice_id += 1;
        self.notices.push((notice_id, notice));
        cx.emit(AcpThreadEvent::NoticesUpdated);
        cx.notify();
    }

    pub fn dismiss_notice(&mut self, notice_id: usize, cx: &mut Context<Self>) {
        let previous_count = self.notices.len();
        self.notices.retain(|(id, _)| *id != notice_id);
        if self.notices.len() != previous_count {
            cx.emit(AcpThreadEvent::NoticesUpdated);
            cx.notify();
        }
    }

    /// Form elicitations stop rendering after accept, so the associated tool
    /// call is the scroll target for the user's answer.
    pub fn is_user_authored_scroll_target(&self, entry: &AgentThreadEntry) -> bool {
        match entry {
            AgentThreadEntry::UserMessage(_) => true,
            AgentThreadEntry::ToolCall(call) => self.tool_call_has_accepted_user_answer(&call.id),
            _ => false,
        }
    }

    fn tool_call_has_accepted_user_answer(&self, tool_call_id: &acp_v2::ToolCallId) -> bool {
        self.elicitations.elicitations().iter().any(|elicitation| {
            matches!(elicitation.status, ElicitationStatus::Accepted)
                // Accepting a URL elicitation only consents to opening a link,
                // so it isn't an answer.
                && matches!(elicitation.request.mode, acp_v2::ElicitationMode::Form(_))
                && matches!(
                    elicitation.request.scope(),
                    acp_v2::ElicitationScope::Session(scope)
                        if scope.tool_call_id.as_ref() == Some(tool_call_id)
                )
        })
    }

    pub fn is_compacting(&self) -> bool {
        self.entries.iter().rev().any(|entry| {
            matches!(
                entry,
                AgentThreadEntry::ContextCompaction(compaction) if compaction.is_in_progress()
            )
        })
    }

    pub fn invalidate_mermaid_caches(&self, cx: &mut App) {
        for entry in &self.entries {
            let chunks = match entry {
                AgentThreadEntry::AssistantMessage(message) => &message.chunks,
                _ => continue,
            };
            for chunk in chunks {
                let block = match chunk {
                    AssistantMessageChunk::Message { block, .. } => block,
                    AssistantMessageChunk::Thought { block, .. } => block,
                };
                for markdown in block.markdowns() {
                    markdown.update(cx, |markdown, cx| {
                        markdown.invalidate_mermaid_cache(cx);
                    });
                }
            }
        }
    }

    pub fn session_id(&self) -> &acp_v2::SessionId {
        &self.session_info.session_id
    }

    pub fn session_info(&self) -> &AgentSessionInfo {
        &self.session_info
    }

    pub fn supports_truncate(&self, cx: &App) -> bool {
        self.connection.truncate(self.session_id(), cx).is_some()
    }

    /// Gates editing and restoring user messages, and whether sending one takes
    /// a git checkpoint, which only the "Restore Checkpoint" button consumes.
    pub fn can_rewind_to(&self, client_id: Option<&ClientUserMessageId>, cx: &App) -> bool {
        client_id.is_some() && self.parent_session_id.is_none() && self.supports_truncate(cx)
    }

    pub fn work_dirs(&self) -> Option<&PathList> {
        self.session_info.work_dirs.as_ref()
    }

    pub fn set_work_dirs(&mut self, work_dirs: PathList, cx: &mut Context<Self>) {
        self.session_info.work_dirs = Some(work_dirs);
        cx.emit(AcpThreadEvent::WorkingDirectoriesUpdated)
    }

    pub fn status(&self) -> ThreadStatus {
        match self.foreground_activity() {
            ForegroundActivity::Idle => ThreadStatus::Idle,
            _ => ThreadStatus::Generating,
        }
    }

    pub fn uses_reported_activity(&self) -> bool {
        self.submissions.receipt_transport().is_some()
    }

    pub fn foreground_state(&self) -> &acp_v2::StateUpdate {
        self.activity.state()
    }

    pub fn foreground_activity(&self) -> ForegroundActivity {
        self.activity.phase()
    }

    pub fn activity_generation(&self) -> u64 {
        self.activity.generation()
    }

    pub fn activity_started_at(&self) -> Option<Instant> {
        self.activity.started_at()
    }

    pub fn activity_duration(&self) -> Option<Duration> {
        self.activity.duration()
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn is_waiting_for_confirmation(&self) -> bool {
        if self.uses_reported_activity() {
            return self.foreground_activity() == ForegroundActivity::RequiresAction;
        }
        self.has_pending_turn_action()
    }

    fn has_pending_turn_action(&self) -> bool {
        if self
            .permission_requests
            .values()
            .any(|request| request.generic_request().is_some())
        {
            return true;
        }
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(call)
                    if call
                        .authorization_id()
                        .is_some_and(|id| self.permission_request(id).is_some()) =>
                {
                    return true;
                }
                AgentThreadEntry::Elicitation(elicitation_id)
                    if self.elicitations.elicitation(elicitation_id).is_some_and(
                        |(_, elicitation)| {
                            matches!(elicitation.status, ElicitationStatus::Pending { .. })
                        },
                    ) =>
                {
                    return true;
                }
                AgentThreadEntry::ToolCall(_)
                | AgentThreadEntry::Elicitation(_)
                | AgentThreadEntry::AssistantMessage(_)
                | AgentThreadEntry::ContextCompaction(_) => {}
            }
        }
        false
    }

    pub fn token_usage(&self) -> Option<&TokenUsage> {
        self.token_usage.as_ref()
    }

    pub fn submission(&self, id: SubmissionId) -> Option<&SubmissionRecord> {
        self.submissions.get(id)
    }

    pub fn latest_submission_id(&self) -> Option<SubmissionId> {
        self.submissions.latest_id()
    }

    pub fn has_unsettled_submissions(&self) -> bool {
        self.submissions.has_unsettled()
    }

    pub fn recoverable_submissions(
        &self,
    ) -> impl Iterator<Item = (SubmissionId, &SubmissionRecord)> {
        self.submissions.recoverable()
    }

    pub fn is_idle_for_retention(&self) -> bool {
        self.status() == ThreadStatus::Idle
            && !self.has_unsettled_submissions()
            && self.recoverable_submissions().next().is_none()
            && self.permission_requests.is_empty()
            && !self.entries.iter().any(|entry| match entry {
                AgentThreadEntry::Elicitation(id) => {
                    self.elicitations
                        .elicitation(id)
                        .is_some_and(|(_, elicitation)| {
                            matches!(elicitation.status, ElicitationStatus::Pending { .. })
                        })
                }
                _ => false,
            })
    }

    pub fn forget_submission(&mut self, id: SubmissionId, cx: &mut Context<Self>) {
        if self.submissions.forget(id) {
            cx.emit(AcpThreadEvent::SubmissionUpdated(id));
            cx.notify();
        }
    }

    fn register_submission(
        &mut self,
        content: Arc<[acp_v2::ContentBlock]>,
        cx: &mut Context<Self>,
    ) -> SubmissionId {
        let id = self.submissions.register(content);
        cx.emit(AcpThreadEvent::SubmissionUpdated(id));
        cx.notify();
        id
    }

    fn observe_submission_echo(&mut self, message_id: &acp_v2::MessageId, cx: &mut Context<Self>) {
        for id in self.submissions.observe_echo(message_id) {
            cx.emit(AcpThreadEvent::SubmissionUpdated(id));
        }
        cx.notify();
    }

    fn track_submission(
        &mut self,
        id: SubmissionId,
        cx: &mut Context<Self>,
        complete: impl 'static
        + AsyncFnOnce(
            WeakEntity<Self>,
            &mut AsyncApp,
        ) -> Result<Option<SubmissionResponse>>,
    ) -> Submission {
        let (sender, receiver) = oneshot::channel();
        let task = cx.spawn(async move |this, cx| {
            let response = complete(this.clone(), cx).await;
            let update = this.update(cx, |this, cx| {
                let state = match &response {
                    Ok(Some(SubmissionResponse::Accepted(receipt))) => {
                        let echoed = this.entries.iter().any(|entry| {
                            matches!(entry, AgentThreadEntry::UserMessage(message)
                                if matches!(&message.identity, MessageIdentity::Keyed(id) if id == &receipt.message_id))
                        });
                        SubmissionState::Accepted { receipt: receipt.clone(), echoed }
                    }
                    Ok(Some(SubmissionResponse::LegacyCompleted(_))) => SubmissionState::Completed,
                    Ok(None) => SubmissionState::Cancelled,
                    Err(error) => SubmissionState::Failed(format!("{error:#}").into()),
                };
                this.submissions.settle(id, state);
                // The thread owns settlement even when the response observer has gone away.
                if let Err(Err(error)) = sender.send(response) {
                    log::debug!("Submission observer was dropped: {error:#}");
                }
                cx.emit(AcpThreadEvent::SubmissionUpdated(id));
                cx.notify();
            });
            if let Err(error) = update {
                log::debug!("Submission thread was released: {error:#}");
            }
        });
        self.submissions.track(id, task);
        Submission::new(
            id,
            async move {
                receiver
                    .await
                    .context("Submission ended before its response was delivered")?
            }
            .boxed(),
        )
    }

    fn set_foreground_state(&mut self, state: acp_v2::StateUpdate, cx: &mut Context<Self>) {
        let previous = self.foreground_activity();
        self.activity
            .update(state, self.entries.len().saturating_sub(1));
        let current = self.foreground_activity();
        if previous == ForegroundActivity::Idle && current != ForegroundActivity::Idle {
            self.had_error = false;
        }
        cx.notify();
    }

    fn sync_legacy_action_state(&mut self, cx: &mut Context<Self>) {
        if self.uses_reported_activity() || self.foreground_activity() == ForegroundActivity::Idle {
            return;
        }
        let desired = if self.has_pending_turn_action() {
            ForegroundActivity::RequiresAction
        } else {
            ForegroundActivity::Running
        };
        if desired != self.foreground_activity() {
            self.set_foreground_state(
                if desired == ForegroundActivity::RequiresAction {
                    acp_v2::StateUpdate::RequiresAction(acp_v2::RequiresActionStateUpdate::new())
                } else {
                    acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new())
                },
                cx,
            );
            cx.emit(AcpThreadEvent::StatusChanged);
        }
    }

    pub fn update_session_state(
        &mut self,
        state: acp_v2::StateUpdate,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        anyhow::ensure!(
            self.uses_reported_activity(),
            "Session does not report foreground state"
        );
        let previous = self.foreground_activity();
        let was_active = previous != ForegroundActivity::Idle;
        self.set_foreground_state(state, cx);
        if let acp_v2::StateUpdate::Idle(idle) = self.activity.state() {
            let stop_reason = idle.stop_reason.clone();
            if let Some(usage) = &idle.usage {
                let stored = self.token_usage.get_or_insert_with(Default::default);
                stored.input_tokens = usage.input_tokens;
                stored.output_tokens = usage.output_tokens;
                cx.emit(AcpThreadEvent::TokenUsageUpdated);
            }
            if was_active {
                self.flush_streaming_text(cx);
                self.shrink_message_source_capacity(self.activity.first_entry_index());
                if self.parent_session_id.is_none() {
                    self.project
                        .update(cx, |project, cx| project.set_agent_location(None, cx));
                }
                self.had_error = matches!(
                    stop_reason,
                    Some(
                        acp_v2::StopReason::MaxTokens
                            | acp_v2::StopReason::Refusal
                            | acp_v2::StopReason::Error(_)
                    )
                );
                // Reported refusal does not authorize deleting agent-owned history.
                cx.emit(AcpThreadEvent::Stopped {
                    activity_generation: self.activity.generation(),
                    activity_duration: self.activity.duration(),
                    stop_reason,
                });
            }
        }
        if previous != self.foreground_activity() {
            cx.emit(AcpThreadEvent::StatusChanged);
        }
        Ok(())
    }

    pub fn cost(&self) -> Option<&SessionCost> {
        self.cost.as_ref()
    }

    pub fn has_pending_edit_tool_calls(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(call)
                    if matches!(
                        call.status(),
                        ToolCallStatus::InProgress | ToolCallStatus::Pending
                    ) && call.diffs().next().is_some() =>
                {
                    return true;
                }
                AgentThreadEntry::ToolCall(_)
                | AgentThreadEntry::Elicitation(_)
                | AgentThreadEntry::AssistantMessage(_)
                | AgentThreadEntry::ContextCompaction(_) => {}
            }
        }

        false
    }

    pub fn has_in_progress_tool_calls(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(call)
                    if matches!(
                        call.status(),
                        ToolCallStatus::InProgress | ToolCallStatus::Pending
                    ) =>
                {
                    return true;
                }
                AgentThreadEntry::ToolCall(_)
                | AgentThreadEntry::Elicitation(_)
                | AgentThreadEntry::AssistantMessage(_)
                | AgentThreadEntry::ContextCompaction(_) => {}
            }
        }

        false
    }

    pub fn used_tools_since_last_user_message(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(..) => return false,
                AgentThreadEntry::AssistantMessage(..)
                | AgentThreadEntry::ContextCompaction(_)
                | AgentThreadEntry::Elicitation(_) => continue,
                AgentThreadEntry::ToolCall(..) => return true,
            }
        }

        false
    }

    pub fn handle_session_update(
        &mut self,
        update: acp_v1::SessionUpdate,
        cx: &mut Context<Self>,
    ) -> Result<(), acp_v1::Error> {
        match update {
            acp_v1::SessionUpdate::UserMessageChunk(acp_v1::ContentChunk {
                content,
                message_id,
                ..
            }) => {
                let content = content::from_v1(content).map_err(acp_v1::Error::from)?;
                // We optimistically add the full user prompt before calling `prompt`.
                // Some ACP servers echo user chunks back over updates. Skip echoed
                // chunks only when they match the local optimistic message.
                let already_in_user_message = self
                    .entries
                    .last_mut()
                    .and_then(|entry| match entry {
                        AgentThreadEntry::UserMessage(message) => Some(message),
                        _ => None,
                    })
                    .is_some_and(|message| {
                        let MessageIdentity::Legacy(protocol_id) = &mut message.identity else {
                            return false;
                        };
                        let already_in_user_message = message.is_optimistic
                            && message.content.source_blocks().contains(&content)
                            && can_merge_message_chunks(protocol_id.as_ref(), message_id.as_ref());
                        if already_in_user_message && protocol_id.is_none() {
                            *protocol_id = message_id.clone();
                        }
                        already_in_user_message
                    });
                if !already_in_user_message {
                    self.push_user_content_block_from_agent(message_id, content, cx);
                }
            }
            acp_v1::SessionUpdate::AgentMessageChunk(acp_v1::ContentChunk {
                content,
                message_id,
                ..
            }) => {
                self.push_assistant_content_block_with_message_id(
                    message_id,
                    content::from_v1(content).map_err(acp_v1::Error::from)?,
                    false,
                    false,
                    cx,
                );
            }
            acp_v1::SessionUpdate::AgentThoughtChunk(acp_v1::ContentChunk {
                content,
                message_id,
                ..
            }) => {
                self.push_assistant_content_block_with_message_id(
                    message_id,
                    content::from_v1(content).map_err(acp_v1::Error::from)?,
                    true,
                    false,
                    cx,
                );
            }
            acp_v1::SessionUpdate::ToolCall(tool_call) => {
                self.upsert_tool_call(tool_call, cx)?;
            }
            acp_v1::SessionUpdate::ToolCallUpdate(tool_call_update) => {
                self.update_tool_call(tool_call_update, cx)?;
            }
            acp_v1::SessionUpdate::CompactionUpdate(compaction_update) => {
                self.upsert_context_compaction_update(
                    compaction::update_from_v1(compaction_update).map_err(acp_v1::Error::from)?,
                    cx,
                );
            }
            acp_v1::SessionUpdate::CompactionSummaryChunk(summary_chunk) => {
                self.append_context_compaction_summary(
                    compaction::chunk_from_v1(summary_chunk).map_err(acp_v1::Error::from)?,
                    cx,
                );
            }
            acp_v1::SessionUpdate::Plan(plan) => {
                self.update_plan(plan, cx).map_err(acp_v1::Error::from)?;
            }
            acp_v1::SessionUpdate::Notice(notice) => {
                self.push_notice(notices::from_v1(notice).map_err(acp_v1::Error::from)?, cx);
            }
            acp_v1::SessionUpdate::SessionInfoUpdate(info_update) => {
                self.update_session_info(session_info_update_from_v1(info_update), cx);
            }
            acp_v1::SessionUpdate::AvailableCommandsUpdate(acp_v1::AvailableCommandsUpdate {
                available_commands,
                ..
            }) => {
                self.update_available_commands(
                    commands::from_v1(available_commands).map_err(acp_v1::Error::from)?,
                    cx,
                );
            }
            acp_v1::SessionUpdate::CurrentModeUpdate(acp_v1::CurrentModeUpdate {
                current_mode_id,
                ..
            }) => cx.emit(AcpThreadEvent::ModeUpdated(current_mode_id)),
            acp_v1::SessionUpdate::ConfigOptionUpdate(acp_v1::ConfigOptionUpdate {
                config_options,
                ..
            }) => cx.emit(AcpThreadEvent::ConfigOptionsUpdated(
                config_options::from_v1(config_options).map_err(acp_v1::Error::from)?,
            )),
            acp_v1::SessionUpdate::UsageUpdate(update) => {
                let usage = self.token_usage.get_or_insert_with(Default::default);
                usage.max_tokens = update.size;
                usage.used_tokens = update.used;
                if let Some(cost) = update.cost {
                    self.cost = Some(SessionCost {
                        amount: cost.amount,
                        currency: cost.currency.into(),
                    });
                }
                cx.emit(AcpThreadEvent::TokenUsageUpdated);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn upsert_user_message(
        &mut self,
        update: acp_v2::UserMessage,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.update_keyed_message(
            MessageKind::User,
            update.message_id,
            update.content,
            update.meta,
            cx,
        )
    }

    pub fn upsert_assistant_message(
        &mut self,
        update: acp_v2::AgentMessage,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.update_keyed_message(
            MessageKind::Assistant,
            update.message_id,
            update.content,
            update.meta,
            cx,
        )
    }

    pub fn upsert_thought(
        &mut self,
        update: acp_v2::AgentThought,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.update_keyed_message(
            MessageKind::Thought,
            update.message_id,
            update.content,
            update.meta,
            cx,
        )
    }

    fn update_keyed_message(
        &mut self,
        kind: MessageKind,
        id: acp_v2::MessageId,
        content: MaybeUndefined<Vec<acp_v2::ContentBlock>>,
        meta: MaybeUndefined<acp_v2::Meta>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let previous_entry_count = self.entries.len();
        let location = self.keyed_message_location(kind, id, cx)?;
        if !content.is_undefined()
            && self
                .streaming_text_buffer
                .as_ref()
                .is_some_and(|buffer| location.is_streaming_target(&buffer.target))
        {
            // Pending text belongs to the superseded snapshot, even when the
            // replacement reuses its Markdown entity.
            self.streaming_text_buffer.take();
        }

        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let (message_content, message_meta) = location
            .fields_mut(&mut self.entries)
            .context("message disappeared during update")?;
        let replacement = match content {
            MaybeUndefined::Undefined => None,
            MaybeUndefined::Null => Some(Vec::new()),
            MaybeUndefined::Value(content) => Some(content),
        };
        if let Some(replacement) = replacement {
            if kind == MessageKind::User {
                message_content.replace_prompt(replacement, &language_registry, path_style, cx);
            } else {
                message_content.replace(replacement, &language_registry, path_style, cx);
            }
        }
        match meta {
            MaybeUndefined::Undefined => {}
            MaybeUndefined::Null => *message_meta = None,
            MaybeUndefined::Value(meta) => *message_meta = Some(meta),
        }
        self.emit_message_update(location, previous_entry_count, cx);
        Ok(())
    }

    pub fn append_message_chunk(
        &mut self,
        kind: MessageKind,
        chunk: acp_v2::ContentChunk,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        // Chunk-envelope metadata does not patch the message's metadata.
        let acp_v2::ContentChunk {
            message_id,
            content,
            ..
        } = chunk;
        let previous_entry_count = self.entries.len();
        let location = self.keyed_message_location(kind, message_id, cx)?;
        let content = match content {
            acp_v2::ContentBlock::Text(text) => {
                let (content, _) = location
                    .fields_mut(&mut self.entries)
                    .context("message disappeared during append")?;
                if let MessageLocation::Assistant {
                    entry_index,
                    chunk_index,
                } = location
                    && let Some(markdown) = content.trailing_text().cloned()
                {
                    let text_len = text.text.len();
                    let source_index = content.append_deferred_text(text);
                    self.buffer_streaming_text(
                        StreamingTextTarget {
                            entry_index,
                            chunk_index,
                            markdown,
                        },
                        source_index,
                        text_len,
                        cx,
                    );
                    self.emit_message_update(location, previous_entry_count, cx);
                    return Ok(());
                }
                acp_v2::ContentBlock::Text(text)
            }
            content => content,
        };

        if self
            .streaming_text_buffer
            .as_ref()
            .is_some_and(|buffer| location.is_streaming_target(&buffer.target))
        {
            self.flush_streaming_text(cx);
        }
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let (message_content, _) = location
            .fields_mut(&mut self.entries)
            .context("message disappeared during append")?;
        if kind == MessageKind::User {
            message_content.append_prompt(content, &language_registry, path_style, cx);
        } else {
            message_content.append(content, &language_registry, path_style, cx);
        }
        self.emit_message_update(location, previous_entry_count, cx);
        Ok(())
    }

    fn keyed_message_location(
        &mut self,
        kind: MessageKind,
        id: acp_v2::MessageId,
        cx: &mut Context<Self>,
    ) -> Result<MessageLocation> {
        // Scan the authoritative records so rewind and refusal need no secondary
        // index maintenance. Most chunks target the last record.
        let identity = MessageIdentity::Keyed(id);
        for (entry_index, entry) in self.entries.iter().enumerate().rev() {
            let found = match entry {
                AgentThreadEntry::UserMessage(message) if message.identity == identity => {
                    Some((MessageKind::User, MessageLocation::User { entry_index }))
                }
                AgentThreadEntry::AssistantMessage(message) => message
                    .chunks
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, chunk)| chunk.identity() == &identity)
                    .map(|(chunk_index, chunk)| {
                        (
                            chunk.kind(),
                            MessageLocation::Assistant {
                                entry_index,
                                chunk_index,
                            },
                        )
                    }),
                _ => None,
            };
            if let Some((existing_kind, location)) = found {
                anyhow::ensure!(
                    existing_kind == kind,
                    "message {identity:?} changed kind from {existing_kind:?} to {kind:?}"
                );
                return Ok(location);
            }
        }

        self.flush_streaming_text(cx);
        let entry_index = self.entries.len();
        match kind {
            MessageKind::User => {
                self.entries
                    .push(AgentThreadEntry::UserMessage(UserMessage {
                        identity,
                        meta: None,
                        client_id: None,
                        is_optimistic: false,
                        content: MessageContent::default(),
                        checkpoint: None,
                        indented: false,
                    }));
                Ok(MessageLocation::User { entry_index })
            }
            MessageKind::Assistant | MessageKind::Thought => {
                let chunk = if kind == MessageKind::Thought {
                    AssistantMessageChunk::Thought {
                        identity,
                        meta: None,
                        block: MessageContent::default(),
                    }
                } else {
                    AssistantMessageChunk::Message {
                        identity,
                        meta: None,
                        block: MessageContent::default(),
                    }
                };
                if let Some(AgentThreadEntry::AssistantMessage(message)) = self.entries.last_mut()
                    && !message.indented
                    && !message.is_subagent_output
                {
                    let chunk_index = message.chunks.len();
                    message.chunks.push(chunk);
                    Ok(MessageLocation::Assistant {
                        entry_index: entry_index - 1,
                        chunk_index,
                    })
                } else {
                    self.entries
                        .push(AgentThreadEntry::AssistantMessage(AssistantMessage {
                            chunks: vec![chunk],
                            indented: false,
                            is_subagent_output: false,
                        }));
                    Ok(MessageLocation::Assistant {
                        entry_index,
                        chunk_index: 0,
                    })
                }
            }
        }
    }

    fn emit_message_update(
        &mut self,
        location: MessageLocation,
        previous_entry_count: usize,
        cx: &mut Context<Self>,
    ) {
        if self.uses_reported_activity()
            && let Some(AgentThreadEntry::UserMessage(message)) =
                self.entries.get(location.entry_index())
            && let MessageIdentity::Keyed(id) = &message.identity
        {
            let id = id.clone();
            self.observe_submission_echo(&id, cx);
        }
        if self.entries.len() > previous_entry_count {
            cx.emit(AcpThreadEvent::NewEntry);
        } else {
            cx.emit(AcpThreadEvent::EntryUpdated(location.entry_index()));
        }
    }

    pub fn push_user_content_block(
        &mut self,
        client_id: Option<ClientUserMessageId>,
        chunk: acp_v2::ContentBlock,
        cx: &mut Context<Self>,
    ) {
        self.push_user_content_block_with_indent(client_id, chunk, false, cx)
    }

    pub fn push_user_content_block_with_indent(
        &mut self,
        client_id: Option<ClientUserMessageId>,
        chunk: acp_v2::ContentBlock,
        indented: bool,
        cx: &mut Context<Self>,
    ) {
        self.push_user_content_block_with_protocol_id(
            client_id.clone(),
            client_id.is_some(),
            None,
            chunk,
            indented,
            cx,
        )
    }

    fn push_user_content_block_from_agent(
        &mut self,
        id: Option<acp_v1::MessageId>,
        chunk: acp_v2::ContentBlock,
        cx: &mut Context<Self>,
    ) {
        self.push_user_content_block_with_protocol_id(None, false, id, chunk, false, cx)
    }

    fn push_user_content_block_with_protocol_id(
        &mut self,
        incoming_client_id: Option<ClientUserMessageId>,
        is_optimistic: bool,
        protocol_id: Option<acp_v1::MessageId>,
        chunk: acp_v2::ContentBlock,
        indented: bool,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let entries_len = self.entries.len();
        self.flush_streaming_text(cx);

        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::UserMessage(UserMessage {
                identity: MessageIdentity::Legacy(existing_protocol_id),
                client_id: existing_client_id,
                content,
                is_optimistic: existing_is_optimistic,
                indented: existing_indented,
                ..
            }) = last_entry
            && *existing_indented == indented
            && can_merge_message_chunks(existing_protocol_id.as_ref(), protocol_id.as_ref())
            && !(*existing_is_optimistic
                && !is_optimistic
                && existing_protocol_id.is_none()
                && protocol_id.is_some())
        {
            if let Some(incoming_client_id) = incoming_client_id {
                *existing_client_id = Some(incoming_client_id);
            }
            *existing_is_optimistic |= is_optimistic;
            if existing_protocol_id.is_none() {
                *existing_protocol_id = protocol_id;
            }
            content.append_prompt(chunk, &language_registry, path_style, cx);
            let idx = entries_len - 1;
            cx.emit(AcpThreadEvent::EntryUpdated(idx));
        } else {
            let mut content = MessageContent::default();
            content.append_prompt(chunk, &language_registry, path_style, cx);
            self.push_entry(
                AgentThreadEntry::UserMessage(UserMessage {
                    identity: MessageIdentity::Legacy(protocol_id),
                    meta: None,
                    client_id: incoming_client_id,
                    is_optimistic,
                    content,
                    checkpoint: None,
                    indented,
                }),
                cx,
            );
        }
    }

    pub fn push_assistant_content_block(
        &mut self,
        chunk: acp_v2::ContentBlock,
        is_thought: bool,
        cx: &mut Context<Self>,
    ) {
        self.push_assistant_content_block_with_indent(chunk, is_thought, false, cx)
    }

    pub fn push_assistant_content_block_with_indent(
        &mut self,
        chunk: acp_v2::ContentBlock,
        is_thought: bool,
        indented: bool,
        cx: &mut Context<Self>,
    ) {
        self.push_assistant_content_block_with_message_id(None, chunk, is_thought, indented, cx)
    }

    fn push_assistant_content_block_with_message_id(
        &mut self,
        message_id: Option<acp_v1::MessageId>,
        chunk: acp_v2::ContentBlock,
        is_thought: bool,
        indented: bool,
        cx: &mut Context<Self>,
    ) {
        let path_style = self.project.read(cx).path_style(cx);

        // For text chunks going to an existing Markdown block, buffer for smooth
        // streaming instead of appending all at once which may feel more choppy.
        let chunk = match chunk {
            acp_v2::ContentBlock::Text(text) => {
                if let Some((content, target)) =
                    self.streaming_content_target(message_id.as_ref(), is_thought, indented)
                {
                    let text_len = text.text.len();
                    let source_index = content.append_deferred_text(text);
                    cx.emit(AcpThreadEvent::EntryUpdated(target.entry_index));
                    self.buffer_streaming_text(target, source_index, text_len, cx);
                    return;
                }
                acp_v2::ContentBlock::Text(text)
            }
            chunk => chunk,
        };

        let language_registry = self.project.read(cx).languages().clone();
        let entries_len = self.entries.len();
        self.flush_streaming_text(cx);
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::AssistantMessage(AssistantMessage {
                chunks,
                indented: existing_indented,
                is_subagent_output: _,
            }) = last_entry
            && *existing_indented == indented
        {
            let idx = entries_len - 1;
            cx.emit(AcpThreadEvent::EntryUpdated(idx));
            match (chunks.last_mut(), is_thought) {
                (
                    Some(AssistantMessageChunk::Message {
                        identity: MessageIdentity::Legacy(existing_id),
                        block,
                        ..
                    }),
                    false,
                )
                | (
                    Some(AssistantMessageChunk::Thought {
                        identity: MessageIdentity::Legacy(existing_id),
                        block,
                        ..
                    }),
                    true,
                ) if can_merge_message_chunks(existing_id.as_ref(), message_id.as_ref()) => {
                    if existing_id.is_none() {
                        *existing_id = message_id;
                    }
                    block.append(chunk, &language_registry, path_style, cx)
                }
                _ => {
                    let block = MessageContent::new(chunk, &language_registry, path_style, cx);
                    if is_thought {
                        chunks.push(AssistantMessageChunk::Thought {
                            identity: MessageIdentity::Legacy(message_id),
                            meta: None,
                            block,
                        })
                    } else {
                        chunks.push(AssistantMessageChunk::Message {
                            identity: MessageIdentity::Legacy(message_id),
                            meta: None,
                            block,
                        })
                    }
                }
            }
        } else {
            let block = MessageContent::new(chunk, &language_registry, path_style, cx);
            let chunk = if is_thought {
                AssistantMessageChunk::Thought {
                    identity: MessageIdentity::Legacy(message_id),
                    meta: None,
                    block,
                }
            } else {
                AssistantMessageChunk::Message {
                    identity: MessageIdentity::Legacy(message_id),
                    meta: None,
                    block,
                }
            };

            self.push_entry(
                AgentThreadEntry::AssistantMessage(AssistantMessage {
                    chunks: vec![chunk],
                    indented,
                    is_subagent_output: false,
                }),
                cx,
            );
        }
    }

    fn streaming_content_target(
        &mut self,
        message_id: Option<&acp_v1::MessageId>,
        is_thought: bool,
        indented: bool,
    ) -> Option<(&mut MessageContent, StreamingTextTarget)> {
        let (entry_index, last_entry) = self.entries.iter_mut().enumerate().next_back()?;
        if let AgentThreadEntry::AssistantMessage(AssistantMessage {
            chunks,
            indented: existing_indented,
            ..
        }) = last_entry
            && *existing_indented == indented
            && let Some((chunk_index, chunk)) = chunks.iter_mut().enumerate().next_back()
        {
            match (chunk, is_thought) {
                (
                    AssistantMessageChunk::Message {
                        identity: MessageIdentity::Legacy(existing_id),
                        block,
                        ..
                    },
                    false,
                )
                | (
                    AssistantMessageChunk::Thought {
                        identity: MessageIdentity::Legacy(existing_id),
                        block,
                        ..
                    },
                    true,
                ) if can_merge_message_chunks(existing_id.as_ref(), message_id) => {
                    let markdown = block.trailing_text()?.clone();
                    if existing_id.is_none() {
                        *existing_id = message_id.cloned();
                    }
                    Some((
                        block,
                        StreamingTextTarget {
                            entry_index,
                            chunk_index,
                            markdown,
                        },
                    ))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Add text to the streaming buffer. If the target changed (e.g. switching
    /// from thoughts to message text), flush the old buffer first.
    fn buffer_streaming_text(
        &mut self,
        target: StreamingTextTarget,
        source_index: usize,
        text_len: usize,
        cx: &mut Context<Self>,
    ) {
        if let Some(buffer) = &mut self.streaming_text_buffer
            && buffer.target == target
        {
            buffer.cursor.pending_bytes += text_len;
            buffer.bytes_to_reveal_per_tick = (buffer.cursor.pending_bytes as f32
                / StreamingTextBuffer::REVEAL_TARGET
                * StreamingTextBuffer::TASK_UPDATE_MS as f32)
                .ceil() as usize;
            return;
        }
        self.flush_streaming_text(cx);

        let _reveal_task = self.start_streaming_reveal(cx);
        let bytes_to_reveal = (text_len as f32 / StreamingTextBuffer::REVEAL_TARGET
            * StreamingTextBuffer::TASK_UPDATE_MS as f32)
            .ceil() as usize;
        self.streaming_text_buffer = Some(StreamingTextBuffer {
            cursor: TextCursor {
                source_index,
                byte_offset: 0,
                pending_bytes: text_len,
            },
            bytes_to_reveal_per_tick: bytes_to_reveal,
            target,
            _reveal_task,
        });
    }

    /// Flush all buffered streaming text into the Markdown entity immediately.
    fn flush_streaming_text(&mut self, cx: &mut Context<Self>) {
        if let Some(mut buffer) = self.streaming_text_buffer.take() {
            buffer.reveal(&self.entries, usize::MAX, cx);
        }
    }

    fn shrink_message_source_capacity(&mut self, first_entry_index: usize) {
        for entry in self.entries.iter_mut().skip(first_entry_index) {
            match entry {
                AgentThreadEntry::UserMessage(message) => message.content.shrink_source_capacity(),
                AgentThreadEntry::AssistantMessage(message) => {
                    for chunk in &mut message.chunks {
                        let (AssistantMessageChunk::Message { block, .. }
                        | AssistantMessageChunk::Thought { block, .. }) = chunk;
                        block.shrink_source_capacity();
                    }
                }
                _ => {}
            }
        }
    }

    /// Reveals retained source text gradually without changing its content.
    fn start_streaming_reveal(&self, cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(StreamingTextBuffer::TASK_UPDATE_MS))
                    .await;

                let should_continue = this
                    .update(cx, |this, cx| {
                        let Some(buffer) = &mut this.streaming_text_buffer else {
                            return false;
                        };

                        let valid =
                            buffer.reveal(&this.entries, buffer.bytes_to_reveal_per_tick, cx);
                        if !valid {
                            this.streaming_text_buffer.take();
                        }
                        valid
                    })
                    .unwrap_or(false);

                if !should_continue {
                    break;
                }
            }
        })
    }

    fn push_entry(&mut self, entry: AgentThreadEntry, cx: &mut Context<Self>) {
        self.flush_streaming_text(cx);
        self.entries.push(entry);
        cx.emit(AcpThreadEvent::NewEntry);
    }

    fn truncate_entries(&mut self, index: usize, cx: &mut Context<Self>) {
        let removed_requests = self
            .entries
            .iter()
            .skip(index)
            .filter_map(|entry| match entry {
                AgentThreadEntry::ToolCall(call) => call.authorization_id(),
                _ => None,
            })
            .collect::<Vec<_>>();
        self.entries.truncate(index);
        for id in removed_requests {
            self.resolve_permission_request(id, RequestPermissionOutcome::Cancelled, cx);
        }
        // Unanchored prompts cannot remain valid across a transcript rollback.
        self.cancel_generic_permission_requests(cx);
    }

    pub fn push_context_compaction(
        &mut self,
        compaction: ContextCompaction,
        cx: &mut Context<Self>,
    ) {
        if let Some(ix) =
            self.entries
                .iter()
                .enumerate()
                .rev()
                .find_map(|(ix, entry)| match entry {
                    AgentThreadEntry::ContextCompaction(c) if &c.id == &compaction.id => Some(ix),
                    _ => None,
                })
        {
            self.entries[ix] = AgentThreadEntry::ContextCompaction(compaction);
            cx.emit(AcpThreadEvent::EntryUpdated(ix));
        } else {
            self.push_entry(AgentThreadEntry::ContextCompaction(compaction), cx);
        }
    }

    pub fn upsert_context_compaction_update(
        &mut self,
        update: acp_v2::CompactionUpdate,
        cx: &mut Context<Self>,
    ) {
        let id = ContextCompactionId(update.compaction_id.0.clone());
        let language_registry = self.project.read(cx).languages().clone();

        if let Some((entry_index, compaction)) =
            self.entries
                .iter_mut()
                .enumerate()
                .rev()
                .find_map(|(entry_index, entry)| match entry {
                    AgentThreadEntry::ContextCompaction(compaction) if compaction.id == id => {
                        Some((entry_index, compaction))
                    }
                    _ => None,
                })
        {
            compaction.apply_update(update, &language_registry, cx);
            cx.emit(AcpThreadEvent::EntryUpdated(entry_index));
            return;
        }

        let mut compaction = ContextCompaction {
            id,
            status: update.status.clone().into(),
            error: None,
            summary: MessageContent::default(),
            meta: None,
        };
        compaction.apply_update(update, &language_registry, cx);
        self.push_entry(AgentThreadEntry::ContextCompaction(compaction), cx);
    }

    pub fn append_context_compaction_summary(
        &mut self,
        chunk: acp_v2::CompactionSummaryChunk,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        if let Some((entry_index, compaction)) =
            self.entries
                .iter_mut()
                .enumerate()
                .rev()
                .find_map(|(entry_index, entry)| match entry {
                    AgentThreadEntry::ContextCompaction(compaction)
                        if compaction.id.0 == chunk.compaction_id.0 =>
                    {
                        Some((entry_index, compaction))
                    }
                    _ => None,
                })
        {
            // Chunk metadata is delivery-scoped, not a patch to the compaction record.
            compaction.append_summary(chunk.content, &language_registry, cx);
            cx.emit(AcpThreadEvent::EntryUpdated(entry_index));
            return;
        }
        let mut compaction = ContextCompaction {
            id: ContextCompactionId(chunk.compaction_id.0),
            status: ContextCompactionStatus::InProgress,
            error: None,
            summary: MessageContent::default(),
            meta: None,
        };
        compaction.append_summary(chunk.content, &language_registry, cx);
        self.push_entry(AgentThreadEntry::ContextCompaction(compaction), cx);
    }

    pub fn update_context_compaction(
        &mut self,
        update: ContextCompactionUpdate,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        let Some((ix, compaction)) =
            self.entries
                .iter_mut()
                .enumerate()
                .rev()
                .find_map(|(ix, entry)| match entry {
                    AgentThreadEntry::ContextCompaction(c) if &c.id == &update.id => Some((ix, c)),
                    _ => None,
                })
        else {
            return;
        };

        if !update.summary_delta.is_empty() {
            compaction.append_summary(
                acp_v2::ContentBlock::Text(acp_v2::TextContent::new(update.summary_delta)),
                &language_registry,
                cx,
            );
        }

        if let Some(status) = update.status {
            compaction.status = status;
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
    }

    pub fn update_session_info(
        &mut self,
        update: acp_v2::SessionInfoUpdate,
        cx: &mut Context<Self>,
    ) {
        let previous_title = self.session_info.title.clone();
        let had_provisional =
            !update.title.is_undefined() && self.provisional_title.take().is_some();
        let changed = self.session_info.apply_update(update);
        if self.session_info.title != previous_title || had_provisional {
            cx.emit(AcpThreadEvent::TitleUpdated);
        }
        if changed || had_provisional {
            cx.notify();
        }
    }

    pub fn can_set_title(&mut self, cx: &mut Context<Self>) -> bool {
        self.connection.set_title(self.session_id(), cx).is_some()
    }

    pub fn set_title(&mut self, title: SharedString, cx: &mut Context<Self>) -> Task<Result<()>> {
        let had_provisional = self.provisional_title.take().is_some();
        if self.session_info.title.as_ref() != Some(&title) {
            self.session_info.title = Some(title.clone());
            cx.emit(AcpThreadEvent::TitleUpdated);
            if let Some(set_title) = self.connection.set_title(self.session_id(), cx) {
                return set_title.run(title, cx);
            }
        } else if had_provisional {
            cx.emit(AcpThreadEvent::TitleUpdated);
        }
        Task::ready(Ok(()))
    }

    /// Sets a provisional display title without propagating back to the
    /// underlying agent connection. This is used for quick preview titles
    /// (e.g. first 20 chars of the user message) that should be shown
    /// immediately but replaced once the LLM generates a proper title via
    /// `set_title`.
    pub fn set_provisional_title(&mut self, title: SharedString, cx: &mut Context<Self>) {
        self.provisional_title = Some(title);
        cx.emit(AcpThreadEvent::TitleUpdated);
    }

    pub fn subagent_spawned(&mut self, session_id: acp_v2::SessionId, cx: &mut Context<Self>) {
        cx.emit(AcpThreadEvent::SubagentSpawned(session_id));
    }

    pub fn update_token_usage(&mut self, usage: Option<TokenUsage>, cx: &mut Context<Self>) {
        if usage.is_none() {
            self.cost = None;
        }
        self.token_usage = usage;
        cx.emit(AcpThreadEvent::TokenUsageUpdated);
    }

    pub fn update_retry_status(&mut self, status: RetryStatus, cx: &mut Context<Self>) {
        cx.emit(AcpThreadEvent::Retry(status));
    }

    pub fn update_tool_call(
        &mut self,
        update: impl Into<ToolCallUpdate>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        match update.into() {
            ToolCallUpdate::V1(update) => {
                let id = acp_v2::ToolCallId::new(update.tool_call_id.0);
                let Some(index) = self.tool_call_index_for_update(&id, cx)? else {
                    return Ok(());
                };
                let languages = self.project.read(cx).languages().clone();
                let Some(AgentThreadEntry::ToolCall(call)) = self.entries.get_mut(index) else {
                    anyhow::bail!("Tool call entry disappeared while updating");
                };
                let patch = ToolCallPatch::legacy(update.fields, update.meta);
                let location_updated = !patch.locations.is_undefined();
                let authorization_id = call.authorization_id();
                let result = call.apply_patch(
                    patch,
                    languages,
                    ToolTerminalResolver::registered(&self.terminals),
                    cx,
                );
                let detached_id =
                    authorization_id.filter(|id| call.authorization_id() != Some(*id));
                if let Some(id) = detached_id {
                    self.resolve_permission_request(id, RequestPermissionOutcome::Cancelled, cx);
                }
                if let Err(error) = result {
                    cx.emit(AcpThreadEvent::EntryUpdated(index));
                    return Err(error);
                }
                if location_updated {
                    self.resolve_locations(id, cx);
                }
                cx.emit(AcpThreadEvent::EntryUpdated(index));
                Ok(())
            }
            ToolCallUpdate::V2(update) => self.upsert_local_tool_call(update, cx),
            ToolCallUpdate::UpdateDiff(update) => {
                self.update_tool_call_content(update.id, ToolCallContent::Diff(update.diff), cx)
            }
            ToolCallUpdate::UpdateTerminal(update) => self.update_tool_call_content(
                update.id,
                ToolCallContent::Terminal {
                    terminal: update.terminal,
                    meta: None,
                },
                cx,
            ),
        }
    }

    fn tool_call_index_for_update(
        &mut self,
        id: &acp_v2::ToolCallId,
        cx: &mut Context<Self>,
    ) -> Result<Option<usize>> {
        if let Some(index) = self.index_for_tool_call(id) {
            return Ok(Some(index));
        }
        let languages = self.project.read(cx).languages().clone();
        let failed_tool_call = ToolCall::from_acp(
            acp_v1::ToolCall::new(acp_v1::ToolCallId::new(id.0.clone()), "Tool call not found")
                .kind(acp_v1::ToolKind::Fetch)
                .status(acp_v1::ToolCallStatus::Failed)
                .content(vec!["Tool call not found".into()]),
            Some(ToolCallStatus::Failed),
            languages,
            &self.terminals,
            cx,
        )?;
        self.push_entry(AgentThreadEntry::ToolCall(failed_tool_call), cx);
        Ok(None)
    }

    fn update_tool_call_content(
        &mut self,
        id: acp_v2::ToolCallId,
        content: ToolCallContent,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let Some(index) = self.tool_call_index_for_update(&id, cx)? else {
            return Ok(());
        };
        let languages = self.project.read(cx).languages().clone();
        let Some(AgentThreadEntry::ToolCall(call)) = self.entries.get_mut(index) else {
            anyhow::bail!("Tool call entry disappeared while updating content");
        };
        call.structured_content.clear();
        call.structured_content.push(content);
        call.update_raw_output_content(&languages, cx);
        cx.emit(AcpThreadEvent::EntryUpdated(index));
        Ok(())
    }

    /// Updates a tool call if id matches an existing entry, otherwise inserts a new one.
    pub fn upsert_tool_call(
        &mut self,
        tool_call: acp_v1::ToolCall,
        cx: &mut Context<Self>,
    ) -> Result<(), acp_v1::Error> {
        let status = ToolCallStatus::from_reported(tool_status_from_v1(tool_call.status).as_ref());
        if let Some(status) = status {
            self.report_tool_call_completed(status);
        }
        self.upsert_tool_call_inner(tool_call.into(), status, cx)
    }

    fn report_tool_call_completed(&self, status: ToolCallStatus) {
        let agent_telemetry_id = self.connection().telemetry_id();
        let session = self.session_id();
        let parent_session_id = self.parent_session_id();
        if let ToolCallStatus::Completed | ToolCallStatus::Failed = status {
            let status = if matches!(status, ToolCallStatus::Completed) {
                "completed"
            } else {
                "failed"
            };
            telemetry::event!(
                "Agent Tool Call Completed",
                agent_telemetry_id,
                session,
                parent_session_id,
                status
            );
        }
    }

    fn upsert_tool_call_inner(
        &mut self,
        update: acp_v1::ToolCallUpdate,
        status: Option<ToolCallStatus>,
        cx: &mut Context<Self>,
    ) -> Result<(), acp_v1::Error> {
        let id = acp_v2::ToolCallId::new(update.tool_call_id.0.clone());
        let update = if self.index_for_tool_call(&id).is_none() {
            acp_v1::ToolCallUpdate::from(acp_v1::ToolCall::try_from(update)?)
        } else {
            update
        };
        self.upsert_legacy_behavior_tool_call(
            id,
            ToolCallPatch::legacy(update.fields, update.meta),
            status,
            cx,
        )
        .map_err(Into::into)
    }

    fn upsert_legacy_behavior_tool_call(
        &mut self,
        id: acp_v2::ToolCallId,
        patch: ToolCallPatch,
        status: Option<ToolCallStatus>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let language_registry = self.project.read(cx).languages().clone();
        if let Some(ix) = self.index_for_tool_call(&id) {
            let AgentThreadEntry::ToolCall(call) = &mut self.entries[ix] else {
                unreachable!()
            };

            let authorization_id = call.authorization_id();
            let result = call.apply_patch(
                patch,
                language_registry,
                ToolTerminalResolver::registered(&self.terminals),
                cx,
            );
            let detached_id = authorization_id.filter(|id| call.authorization_id() != Some(*id));
            if result.is_ok()
                && let Some(status) = status
            {
                call.set_legacy_status(status);
            }
            if let Some(id) = detached_id {
                self.resolve_permission_request(id, RequestPermissionOutcome::Cancelled, cx);
            }
            if let Err(error) = result {
                cx.emit(AcpThreadEvent::EntryUpdated(ix));
                return Err(error);
            }

            cx.emit(AcpThreadEvent::EntryUpdated(ix));
        } else {
            anyhow::ensure!(
                patch.title.value().is_some(),
                "title is required for a tool call"
            );
            let mut call = ToolCall::from_patch(
                id.clone(),
                patch,
                language_registry,
                ToolTerminalResolver::registered(&self.terminals),
                cx,
            )?;
            if let Some(status) = status {
                call.set_legacy_status(status);
            }
            self.push_entry(AgentThreadEntry::ToolCall(call), cx);
        };

        self.resolve_locations(id, cx);
        Ok(())
    }

    /// Wire updates may create agent-reported display terminals.
    pub fn upsert_wire_tool_call(
        &mut self,
        update: acp_v2::ToolCallUpdate,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Some(content) = update.content.value() {
            for content in content {
                self.ensure_tool_content_terminal(content, cx);
            }
        }
        let id = update.tool_call_id.clone();
        let patch = ToolCallPatch::protocol(update);
        self.upsert_tool_call_patch(id, patch, false, cx)
    }

    /// In-process updates may reference registered client-managed terminals, but
    /// must not create or take ownership of agent-reported display terminals.
    pub fn upsert_local_tool_call(
        &mut self,
        update: acp_v2::ToolCallUpdate,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let id = update.tool_call_id.clone();
        let patch = ToolCallPatch::protocol(update);
        self.upsert_tool_call_patch(id, patch, true, cx)
    }

    fn upsert_tool_call_patch(
        &mut self,
        id: acp_v2::ToolCallId,
        patch: ToolCallPatch,
        client_managed_terminals_only: bool,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let locations_changed = !patch.locations.is_undefined();
        if let Some(status) = ToolCallStatus::from_reported(patch.status.value()) {
            self.report_tool_call_completed(status);
        }
        let languages = self.project.read(cx).languages().clone();
        let terminals = ToolTerminalResolver {
            terminals: &self.terminals,
            client_managed_only: client_managed_terminals_only,
        };
        if let Some(index) = self.index_for_tool_call(&id) {
            let AgentThreadEntry::ToolCall(call) = &mut self.entries[index] else {
                unreachable!()
            };
            let authorization_id = call.authorization_id();
            let result = call.apply_patch(patch, languages, terminals, cx);
            let detached_id = authorization_id.filter(|id| call.authorization_id() != Some(*id));
            if let Some(id) = detached_id {
                self.resolve_permission_request(id, RequestPermissionOutcome::Cancelled, cx);
            }
            cx.emit(AcpThreadEvent::EntryUpdated(index));
            result?;
        } else {
            let call = ToolCall::from_patch(id.clone(), patch, languages, terminals, cx)?;
            self.push_entry(AgentThreadEntry::ToolCall(call), cx);
        }
        if locations_changed {
            self.resolve_locations(id, cx);
        }
        Ok(())
    }

    pub fn append_tool_call_content_chunk(
        &mut self,
        chunk: acp_v2::ToolCallContentChunk,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        // Delivery metadata is not retained as history or promoted into aggregate
        // tool/content metadata, matching the message-chunk boundary.
        let acp_v2::ToolCallContentChunk {
            tool_call_id,
            content,
            ..
        } = chunk;
        self.ensure_tool_content_terminal(&content, cx);
        let content = PreparedToolCallContent::from_v2(
            content,
            ToolTerminalResolver::registered(&self.terminals),
            cx,
        )?;
        let language_registry = self.project.read(cx).languages().clone();
        let id = tool_call_id;

        if let Some((index, call)) = self.tool_call_mut(&id) {
            call.append_content(content, &language_registry, cx);
            cx.emit(AcpThreadEvent::EntryUpdated(index));
        } else {
            let mut call = ToolCall::from_patch(
                id.clone(),
                ToolCallPatch::protocol(acp_v2::ToolCallUpdate::new(id)),
                language_registry.clone(),
                ToolTerminalResolver::registered(&self.terminals),
                cx,
            )?;
            call.append_content(content, &language_registry, cx);
            self.push_entry(AgentThreadEntry::ToolCall(call), cx);
        }
        Ok(())
    }

    fn ensure_tool_content_terminal(
        &mut self,
        content: &acp_v2::ToolCallContent,
        cx: &mut Context<Self>,
    ) {
        if let acp_v2::ToolCallContent::Terminal(terminal) = content {
            self.ensure_display_terminal(terminal.terminal_id.clone(), cx);
        }
    }

    fn index_for_tool_call(&self, id: &acp_v2::ToolCallId) -> Option<usize> {
        self.entries
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, entry)| {
                if let AgentThreadEntry::ToolCall(tool_call) = entry
                    && &tool_call.id == id
                {
                    Some(index)
                } else {
                    None
                }
            })
    }

    fn tool_call_mut(&mut self, id: &acp_v2::ToolCallId) -> Option<(usize, &mut ToolCall)> {
        // The tool call we are looking for is typically the last one, or very close to the end.
        // At the moment, it doesn't seem like a hashmap would be a good fit for this use case.
        self.entries
            .iter_mut()
            .enumerate()
            .rev()
            .find_map(|(index, tool_call)| {
                if let AgentThreadEntry::ToolCall(tool_call) = tool_call
                    && &tool_call.id == id
                {
                    Some((index, tool_call))
                } else {
                    None
                }
            })
    }

    pub fn tool_call(&self, id: &acp_v2::ToolCallId) -> Option<(usize, &ToolCall)> {
        self.entries
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, tool_call)| {
                if let AgentThreadEntry::ToolCall(tool_call) = tool_call
                    && &tool_call.id == id
                {
                    Some((index, tool_call))
                } else {
                    None
                }
            })
    }

    pub fn tool_call_for_subagent(&self, session_id: &acp_v2::SessionId) -> Option<&ToolCall> {
        self.entries.iter().find_map(|entry| match entry {
            AgentThreadEntry::ToolCall(tool_call) => {
                if let Some(subagent_session_info) = &tool_call.subagent_session_info
                    && &subagent_session_info.session_id == session_id
                {
                    Some(tool_call)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    pub fn resolve_locations(&mut self, id: acp_v2::ToolCallId, cx: &mut Context<Self>) {
        let project = self.project.clone();
        let should_update_agent_location = self.parent_session_id.is_none();
        let Some((_, tool_call)) = self.tool_call_mut(&id) else {
            return;
        };
        let expected_locations = tool_call.locations.clone();
        let task = tool_call.resolve_locations(project, cx);
        cx.spawn(async move |this, cx| {
            let resolved_locations = task.await;

            this.update(cx, |this, cx| {
                if this
                    .tool_call(&id)
                    .is_none_or(|(_, call)| call.locations != expected_locations)
                {
                    return;
                }
                let project = this.project.clone();

                this.prune_dead_shared_buffers();
                for location in resolved_locations.iter().flatten() {
                    this.shared_buffers.insert(
                        location.buffer.downgrade(),
                        location.buffer.read(cx).snapshot(),
                    );
                }
                let Some((ix, tool_call)) = this.tool_call_mut(&id) else {
                    return;
                };

                if let Some(Some(location)) = resolved_locations.last() {
                    project.update(cx, |project, cx| {
                        let should_ignore = if let Some(agent_location) = project
                            .agent_location()
                            .filter(|agent_location| agent_location.buffer == location.buffer)
                        {
                            let snapshot = location.buffer.read(cx).snapshot();
                            let old_position = agent_location.position.to_point(&snapshot);
                            let new_position = location.position.to_point(&snapshot);

                            // ignore this so that when we get updates from the edit tool
                            // the position doesn't reset to the startof line
                            old_position.row == new_position.row
                                && old_position.column > new_position.column
                        } else {
                            false
                        };
                        if !should_ignore && should_update_agent_location {
                            project.set_agent_location(Some(location.into()), cx);
                        }
                    });
                }

                let resolved_locations = resolved_locations
                    .iter()
                    .map(|l| l.as_ref().map(|l| AgentLocation::from(l)))
                    .collect::<Vec<_>>();

                if tool_call.resolved_locations != resolved_locations {
                    tool_call.resolved_locations = resolved_locations;
                    cx.emit(AcpThreadEvent::EntryUpdated(ix));
                }
            })
        })
        .detach();
    }

    pub fn permission_request(&self, id: PermissionRequestId) -> Option<&PermissionRequest> {
        self.permission_requests.get(&id)
    }

    pub fn permission_request_for_tool(
        &self,
        tool_call_id: &acp_v2::ToolCallId,
    ) -> Option<&PermissionRequest> {
        let (_, call) = self.tool_call(tool_call_id)?;
        self.permission_request(call.authorization_id()?)
    }

    pub fn pending_permission_requests(&self) -> impl Iterator<Item = &PermissionRequest> {
        self.permission_requests.values()
    }

    pub fn request_permission(
        &mut self,
        request: acp_v2::RequestPermissionRequest,
        cx: &mut Context<Self>,
    ) -> Result<(PermissionRequestId, Task<acp_v2::RequestPermissionOutcome>)> {
        anyhow::ensure!(
            request.session_id == self.session_info.session_id,
            "Permission request belongs to a different session"
        );
        anyhow::ensure!(
            !request.options.is_empty(),
            "Permission request has no offered options"
        );
        let mut option_ids = HashSet::default();
        anyhow::ensure!(
            request
                .options
                .iter()
                .all(|option| option_ids.insert(&option.option_id)),
            "Permission request has duplicate option IDs"
        );

        let (respond_tx, receiver) = oneshot::channel();
        let id = PermissionRequestId(Uuid::new_v4());
        self.permission_requests.insert(
            id,
            PermissionRequest {
                id,
                data: PermissionRequestData::Generic {
                    request,
                    respond_tx,
                },
            },
        );
        cx.emit(AcpThreadEvent::ToolAuthorizationRequested(id));
        Ok((
            id,
            cx.spawn(async move |_, _| {
                receiver
                    .await
                    .unwrap_or(acp_v2::RequestPermissionOutcome::Cancelled)
            }),
        ))
    }

    pub fn select_permission_option(
        &mut self,
        id: PermissionRequestId,
        option_id: acp_v2::PermissionOptionId,
        cx: &mut Context<Self>,
    ) {
        let Some(request) = self
            .permission_request(id)
            .and_then(PermissionRequest::generic_request)
        else {
            return;
        };
        if !request
            .options
            .iter()
            .any(|option| option.option_id == option_id)
        {
            log::debug!("Permission choice is not an offered option");
            return;
        }
        let Some(PermissionRequest {
            data: PermissionRequestData::Generic { respond_tx, .. },
            ..
        }) = self.take_permission_request(id, cx)
        else {
            return;
        };
        if respond_tx
            .send(acp_v2::RequestPermissionOutcome::Selected(
                acp_v2::SelectedPermissionOutcome::new(option_id),
            ))
            .is_err()
        {
            log::debug!("Permission request closed before the outcome was delivered");
        }
    }

    pub fn request_tool_call_authorization(
        &mut self,
        tool_call: acp_v1::ToolCallUpdate,
        options: PermissionOptions,
        kind: AuthorizationKind,
        cx: &mut Context<Self>,
    ) -> Result<Task<RequestPermissionOutcome>> {
        self.request_tool_call_authorization_with_id(tool_call, options, kind, cx)
            .map(|(_, task)| task)
    }

    pub fn request_tool_call_authorization_with_id(
        &mut self,
        tool_call: acp_v1::ToolCallUpdate,
        options: PermissionOptions,
        kind: AuthorizationKind,
        cx: &mut Context<Self>,
    ) -> Result<(PermissionRequestId, Task<RequestPermissionOutcome>)> {
        let tool_call_id = acp_v2::ToolCallId::new(tool_call.tool_call_id.0.clone());
        let current_status = self
            .tool_call(&tool_call_id)
            .and_then(|(_, tool_call)| tool_call.permission_status())
            .or_else(|| tool_call.fields.status.and_then(tool_status_from_v1));
        let current_status = ToolCallStatus::from_reported(current_status.as_ref())
            .unwrap_or(ToolCallStatus::Pending);

        self.upsert_tool_call_inner(tool_call, Some(current_status), cx)?;
        self.install_tool_call_authorization(tool_call_id, options, kind, cx)
    }

    pub fn request_tool_call_update_authorization(
        &mut self,
        tool_call: acp_v2::ToolCallUpdate,
        options: PermissionOptions,
        kind: AuthorizationKind,
        cx: &mut Context<Self>,
    ) -> Result<Task<RequestPermissionOutcome>> {
        self.request_tool_call_update_authorization_with_id(tool_call, options, kind, cx)
            .map(|(_, task)| task)
    }

    pub fn request_tool_call_update_authorization_with_id(
        &mut self,
        tool_call: acp_v2::ToolCallUpdate,
        options: PermissionOptions,
        kind: AuthorizationKind,
        cx: &mut Context<Self>,
    ) -> Result<(PermissionRequestId, Task<RequestPermissionOutcome>)> {
        let tool_call_id = tool_call.tool_call_id.clone();
        let current_status = self
            .tool_call(&tool_call_id)
            .and_then(|(_, call)| call.permission_status())
            .or_else(|| tool_call.status.value().cloned());
        let current_status = ToolCallStatus::from_reported(current_status.as_ref())
            .unwrap_or(ToolCallStatus::Pending);
        self.upsert_local_tool_call(tool_call, cx)?;
        let (_, call) = self
            .tool_call_mut(&tool_call_id)
            .context("tool call disappeared while requesting authorization")?;
        call.set_local_status(current_status);
        self.install_tool_call_authorization(tool_call_id, options, kind, cx)
    }

    fn install_tool_call_authorization(
        &mut self,
        tool_call_id: acp_v2::ToolCallId,
        options: PermissionOptions,
        kind: AuthorizationKind,
        cx: &mut Context<Self>,
    ) -> Result<(PermissionRequestId, Task<RequestPermissionOutcome>)> {
        let (tx, rx) = oneshot::channel();
        if let Some(id) = self
            .tool_call(&tool_call_id)
            .and_then(|(_, call)| call.authorization_id())
        {
            self.resolve_permission_request(id, RequestPermissionOutcome::Cancelled, cx);
        }
        let id = PermissionRequestId(Uuid::new_v4());
        let (_, call) = self
            .tool_call_mut(&tool_call_id)
            .context("tool call disappeared while requesting authorization")?;
        // A new explicit request is installed after applying its descriptive
        // fields, preserving the legacy request's captured continuation status.
        call.authorization = Some(id);
        self.permission_requests.insert(
            id,
            PermissionRequest {
                id,
                data: PermissionRequestData::LegacyTool {
                    tool_call_id,
                    options,
                    respond_tx: tx,
                    kind,
                },
            },
        );
        cx.emit(AcpThreadEvent::ToolAuthorizationRequested(id));

        Ok((
            id,
            cx.spawn(async move |_this, _cx| {
                rx.await.unwrap_or(RequestPermissionOutcome::Cancelled)
            }),
        ))
    }

    pub fn cancel_tool_call_authorization(
        &mut self,
        id: &acp_v2::ToolCallId,
        cx: &mut Context<Self>,
    ) {
        let Some(request_id) = self
            .tool_call(id)
            .and_then(|(_, call)| call.authorization_id())
        else {
            return;
        };
        self.cancel_permission_request(request_id, cx);
    }

    pub fn cancel_permission_request(&mut self, id: PermissionRequestId, cx: &mut Context<Self>) {
        self.cancel_permission_request_with_outcome(id, RequestPermissionOutcome::Cancelled, cx);
    }

    fn cancel_permission_request_with_outcome(
        &mut self,
        id: PermissionRequestId,
        outcome: RequestPermissionOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some(request) = self.permission_requests.get(&id) else {
            return;
        };
        if let Some(tool_call_id) = request.legacy_tool_call_id().cloned()
            && let Some((_, call)) = self.tool_call_mut(&tool_call_id)
            && call.authorization_id() == Some(id)
        {
            call.set_local_status(ToolCallStatus::Canceled);
        }
        self.resolve_permission_request(id, outcome, cx);
    }

    pub fn authorize_tool_call(
        &mut self,
        id: acp_v2::ToolCallId,
        outcome: SelectedPermissionOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some(request_id) = self
            .tool_call(&id)
            .and_then(|(_, call)| call.authorization_id())
        else {
            return;
        };
        self.authorize_permission_request(request_id, outcome, cx);
    }

    pub fn authorize_permission_request(
        &mut self,
        id: PermissionRequestId,
        mut outcome: SelectedPermissionOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some(PermissionRequest {
            data:
                PermissionRequestData::LegacyTool {
                    options,
                    tool_call_id,
                    kind,
                    ..
                },
            ..
        }) = self.permission_requests.get(&id)
        else {
            return;
        };
        let Some(option) = options.option_for_id(&outcome.option_id) else {
            log::debug!("Permission choice is not an offered option");
            return;
        };
        outcome.option_kind = option.kind.clone();
        let tool_call_id = tool_call_id.clone();
        let kind = *kind;
        let Some((_, call)) = self.tool_call_mut(&tool_call_id) else {
            return;
        };
        if call.authorization_id() != Some(id) {
            return;
        }

        let new_status = match kind {
            AuthorizationKind::ActionChoice => ToolCallStatus::InProgress,
            AuthorizationKind::PermissionGrant => {
                let current_status = call.permission_status().unwrap_or_default();
                match outcome.option_kind {
                    acp_v2::PermissionOptionKind::RejectOnce
                    | acp_v2::PermissionOptionKind::RejectAlways => ToolCallStatus::Rejected,
                    acp_v2::PermissionOptionKind::AllowOnce
                    | acp_v2::PermissionOptionKind::AllowAlways => {
                        ToolCallStatus::status_after_permission_grant(current_status)
                    }
                    _ => {
                        log::warn!(
                            "Cannot authorize a tool with an unknown permission option kind"
                        );
                        return;
                    }
                }
            }
        };
        call.set_local_status(new_status);
        self.resolve_permission_request(id, RequestPermissionOutcome::Selected(outcome), cx);
    }

    fn resolve_permission_request(
        &mut self,
        id: PermissionRequestId,
        outcome: RequestPermissionOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some(request) = self.take_permission_request(id, cx) else {
            return;
        };
        let delivered = match request.data {
            PermissionRequestData::LegacyTool { respond_tx, .. } => {
                respond_tx.send(outcome).is_ok()
            }
            PermissionRequestData::Generic { respond_tx, .. } => respond_tx
                .send(acp_v2::RequestPermissionOutcome::Cancelled)
                .is_ok(),
        };
        if !delivered {
            log::debug!("Permission request closed before the outcome was delivered");
        }
    }

    fn take_permission_request(
        &mut self,
        id: PermissionRequestId,
        cx: &mut Context<Self>,
    ) -> Option<PermissionRequest> {
        let request = self.permission_requests.shift_remove(&id)?;
        if let Some(tool_call_id) = request.legacy_tool_call_id()
            && let Some((index, call)) = self.tool_call_mut(tool_call_id)
            && call.authorization_id() == Some(id)
        {
            call.authorization = None;
            cx.emit(AcpThreadEvent::EntryUpdated(index));
        }
        cx.emit(AcpThreadEvent::ToolAuthorizationReceived(id));
        Some(request)
    }

    fn cancel_generic_permission_requests(&mut self, cx: &mut Context<Self>) {
        let ids = self
            .permission_requests
            .values()
            .filter(|request| request.generic_request().is_some())
            .map(|request| request.id)
            .collect::<Vec<_>>();
        for id in ids {
            self.cancel_permission_request(id, cx);
        }
    }

    pub fn request_elicitation(
        &mut self,
        request: acp_v2::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<Task<acp_v2::CreateElicitationResponse>, acp_v2::Error> {
        self.request_elicitation_with_id(request, cx)
            .map(|(_, task)| task)
    }

    pub fn request_elicitation_with_id(
        &mut self,
        request: acp_v2::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<(ElicitationEntryId, Task<acp_v2::CreateElicitationResponse>), acp_v2::Error> {
        ElicitationStore::validate_request(&request)?;

        let (id, response_rx) = self.elicitations.insert_pending_elicitation(request);
        self.push_entry(AgentThreadEntry::Elicitation(id.clone()), cx);
        cx.emit(AcpThreadEvent::ElicitationRequested(id.clone()));

        let task = ElicitationStore::response_task(response_rx, cx);
        Ok((id, task))
    }

    fn emit_elicitation_change(
        entry_index: usize,
        id: &ElicitationEntryId,
        change: ElicitationChange,
        cx: &mut Context<Self>,
    ) {
        cx.emit(AcpThreadEvent::EntryUpdated(entry_index));
        if matches!(change, ElicitationChange::Responded) {
            cx.emit(AcpThreadEvent::ElicitationResponded(id.clone()));
        }
    }

    pub fn respond_to_elicitation(
        &mut self,
        id: &ElicitationEntryId,
        response: acp_v2::CreateElicitationResponse,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.elicitation_entry_ix(id) else {
            return;
        };
        if !self.elicitations.respond_to_elicitation_by_id(id, response) {
            return;
        }

        Self::emit_elicitation_change(ix, id, ElicitationChange::Responded, cx);
    }

    pub fn complete_url_elicitation(
        &mut self,
        elicitation_id: &acp_v2::ElicitationId,
        cx: &mut Context<Self>,
    ) {
        let Some(entry_id) = self
            .elicitations
            .entry_id_for_url_elicitation(elicitation_id)
        else {
            return;
        };
        let Some(ix) = self.elicitation_entry_ix(&entry_id) else {
            return;
        };
        if !self.elicitations.complete_url_elicitation_by_id(&entry_id) {
            return;
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
    }

    pub fn cancel_elicitation(&mut self, id: &ElicitationEntryId, cx: &mut Context<Self>) {
        let Some(ix) = self.elicitation_entry_ix(id) else {
            return;
        };
        let Some(change) = self.elicitations.cancel_elicitation_by_id(id) else {
            return;
        };

        Self::emit_elicitation_change(ix, id, change, cx);
    }

    fn elicitation_entry_ix(&self, id: &ElicitationEntryId) -> Option<usize> {
        self.entries
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, entry)| {
                matches!(entry, AgentThreadEntry::Elicitation(elicitation_id) if elicitation_id == id)
                    .then_some(index)
            })
    }

    pub fn elicitation(&self, id: &ElicitationEntryId) -> Option<(usize, &Elicitation)> {
        let index = self.elicitation_entry_ix(id)?;
        let (_, elicitation) = self.elicitations.elicitation(id)?;
        Some((index, elicitation))
    }

    pub fn plan(&self) -> Option<&Plan> {
        self.plans.get(self.visible_plan.as_ref()?)
    }

    pub fn plan_by_id(&self, id: &acp_v2::PlanId) -> Option<&Plan> {
        self.plans.get(&PlanIdentity::Keyed(id.clone()))
    }

    pub fn update_plan(&mut self, request: acp_v1::Plan, cx: &mut Context<Self>) -> Result<()> {
        let entries = request
            .entries
            .into_iter()
            .map(plan_entry_from_v1)
            .collect::<Result<_>>()?;
        self.replace_plan(PlanIdentity::Legacy, entries, request.meta, None, cx);
        Ok(())
    }

    pub fn upsert_plan_items(
        &mut self,
        plan: acp_v2::PlanItems,
        update_meta: Option<acp_v2::Meta>,
        cx: &mut Context<Self>,
    ) {
        self.replace_plan(
            PlanIdentity::Keyed(plan.plan_id),
            plan.entries,
            plan.meta,
            update_meta,
            cx,
        );
    }

    fn replace_plan(
        &mut self,
        identity: PlanIdentity,
        entries: Vec<acp_v2::PlanEntry>,
        meta: Option<acp_v2::Meta>,
        update_meta: Option<acp_v2::Meta>,
        cx: &mut Context<Self>,
    ) {
        let is_new = !self.plans.contains_key(&identity);
        let changed =
            self.plans
                .entry(identity.clone())
                .or_default()
                .replace(entries, meta, update_meta, cx);
        if is_new || changed || identity == PlanIdentity::Legacy {
            self.visible_plan = Some(identity);
        }
        cx.notify();
    }

    fn clear_completed_plan_entries(&mut self, cx: &mut Context<Self>) {
        // V1's next-turn cleanup is a compatibility policy, not authority to alter keyed plans.
        if let Some(plan) = self.plans.get_mut(&PlanIdentity::Legacy) {
            plan.entries
                .retain(|entry| !matches!(entry.source.status, acp_v2::PlanEntryStatus::Completed));
        }
        cx.notify();
    }

    pub fn clear_plan(&mut self, cx: &mut Context<Self>) {
        self.visible_plan = None;
        cx.notify();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn send_raw(
        &mut self,
        message: &str,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<Option<acp_v1::PromptResponse>>> {
        let submission = self.send(vec![message.into()], cx);
        async move {
            match submission.await? {
                Some(SubmissionResponse::LegacyCompleted(response)) => Ok(Some(response)),
                None => Ok(None),
                Some(SubmissionResponse::Accepted(_)) => {
                    Err(anyhow!("Use send to observe receipt-driven submissions"))
                }
            }
        }
        .boxed()
    }

    pub fn send(
        &mut self,
        message: Vec<acp_v2::ContentBlock>,
        cx: &mut Context<Self>,
    ) -> Submission {
        self.submit(message, true, cx)
    }

    /// Sends a prompt without displaying a user-message bubble for it.
    /// This is used for native slash commands (e.g. `/compact`) that run a turn
    /// which produces its own thread entry (like the compaction summary). The
    /// typed command isn't sent to the model as an ordinary user turn.
    pub fn send_command(
        &mut self,
        message: Vec<acp_v2::ContentBlock>,
        cx: &mut Context<Self>,
    ) -> Submission {
        self.submit(message, false, cx)
    }

    fn submit(
        &mut self,
        message: Vec<acp_v2::ContentBlock>,
        push_user_message: bool,
        cx: &mut Context<Self>,
    ) -> Submission {
        let id = self.register_submission(message.clone().into(), cx);
        if let Some(submissions) = self.submissions.receipt_transport() {
            let response = submissions.prompt(message, cx);
            self.track_submission(id, cx, async move |_, _| {
                Ok(Some(SubmissionResponse::Accepted(response.await?)))
            })
        } else {
            self.send_inner(id, message, push_user_message, cx)
        }
    }

    pub fn validate_prompt_content(&self, content: &[acp_v2::ContentBlock]) -> Result<()> {
        if self.submissions.receipt_transport().is_some() {
            Ok(())
        } else {
            self.connection.validate_prompt_content(content)
        }
    }

    fn send_inner(
        &mut self,
        id: SubmissionId,
        message: Vec<acp_v2::ContentBlock>,
        push_user_message: bool,
        cx: &mut Context<Self>,
    ) -> Submission {
        let request = acp_v2::PromptRequest::new(self.session_id().clone(), message);
        if let Err(error) = self.validate_prompt_content(&request.prompt) {
            return self.track_submission(id, cx, async move |_, _| Err(error));
        }
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let mut block = MessageContent::default();
        for chunk in &request.prompt {
            block.append_prompt(chunk.clone(), &language_registry, path_style, cx);
        }
        let git_store = self.project.read(cx).git_store().clone();

        let client_user_message_ids = self.connection.client_user_message_ids(cx);
        let client_id = client_user_message_ids
            .as_ref()
            .map(|client_user_message_ids| client_user_message_ids.new_id());
        let should_checkpoint = self.can_rewind_to(client_id.as_ref(), cx);

        self.run_turn(id, cx, async move |this, cx| {
            if push_user_message {
                this.update(cx, |this, cx| {
                    this.push_entry(
                        AgentThreadEntry::UserMessage(UserMessage {
                            identity: MessageIdentity::Legacy(None),
                            meta: None,
                            client_id: client_id.clone(),
                            is_optimistic: true,
                            content: block,
                            checkpoint: None,
                            indented: false,
                        }),
                        cx,
                    );
                })
                .ok();

                if should_checkpoint {
                    let old_checkpoint = git_store
                        .update(cx, |git, cx| git.checkpoint(cx))
                        .await
                        .context("failed to get old checkpoint")
                        .log_err();
                    this.update(cx, |this, _cx| {
                        if let Some((_ix, message)) = this.last_user_message() {
                            message.checkpoint = old_checkpoint.map(|git_checkpoint| Checkpoint {
                                git_checkpoint,
                                show: false,
                            });
                        }
                    })
                    .ok();
                }
            }

            this.update(cx, |this, cx| {
                if let (Some(prompt), Some(client_id)) = (client_user_message_ids, client_id) {
                    prompt.prompt(client_id, request, cx)
                } else {
                    this.connection.prompt(request, cx)
                }
            })?
            .await
        })
    }

    pub fn can_retry(&self, cx: &App) -> bool {
        !self.uses_reported_activity()
            && self.connection.retry(self.session_id(), cx).is_some()
            && self
                .submissions
                .latest_id()
                .and_then(|id| self.submissions.get(id))
                .is_none_or(|submission| self.validate_prompt_content(&submission.content).is_ok())
    }

    pub fn retry(&mut self, cx: &mut Context<Self>) -> Submission {
        let content = self
            .submissions
            .latest_id()
            .and_then(|id| self.submissions.get(id))
            .map(|submission| submission.content.clone())
            .unwrap_or_default();
        let id = self.register_submission(content.clone(), cx);
        if self.uses_reported_activity() {
            return self.track_submission(id, cx, async move |_, _| {
                Err(anyhow!("Receipt-driven retry is not supported"))
            });
        }
        if let Err(error) = self.validate_prompt_content(&content) {
            return self.track_submission(id, cx, async move |_, _| Err(error));
        }
        self.run_turn(id, cx, async move |this, cx| {
            this.update(cx, |this, cx| {
                this.connection
                    .retry(this.session_id(), cx)
                    .map(|retry| retry.run(cx))
            })?
            .context("retrying a session is not supported")?
            .await
        })
    }

    fn run_turn(
        &mut self,
        id: SubmissionId,
        cx: &mut Context<Self>,
        f: impl 'static + AsyncFnOnce(WeakEntity<Self>, &mut AsyncApp) -> Result<acp_v1::PromptResponse>,
    ) -> Submission {
        self.clear_completed_plan_entries(cx);
        self.had_error = false;

        let (tx, rx) = oneshot::channel();
        let cancel_task = self.cancel_inner(RequestPermissionOutcome::InterruptedByFollowUp, cx);

        self.turn_id += 1;
        let turn_id = self.turn_id;
        // A continuation can extend the trailing entry rather than create a new one.
        let first_entry_index = self.entries.len().saturating_sub(1);
        self.running_turn = Some(RunningTurn {
            id: turn_id,
            first_entry_index,
            send_task: cx.spawn(async move |this, cx| {
                cancel_task.await;
                tx.send(f(this, cx).await).ok();
            }),
        });
        self.set_foreground_state(
            acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new()),
            cx,
        );
        cx.emit(AcpThreadEvent::StatusChanged);

        let completion = async move |thread: WeakEntity<Self>, cx: &mut AsyncApp| {
            let response = rx.await;

            thread
                .update(cx, |this, cx| {
                    if this.turn_id == turn_id {
                        this.update_last_checkpoint(cx)
                    } else {
                        Task::ready(Ok(()))
                    }
                })?
                .await?;

            thread.update(cx, |this, cx| {
                if this.turn_id == turn_id && this.parent_session_id.is_none() {
                    this.project
                        .update(cx, |project, cx| project.set_agent_location(None, cx));
                }

                let is_same_turn = this
                    .running_turn
                    .as_ref()
                    .is_some_and(|turn| turn_id == turn.id);

                // If the user submitted a follow up message, running_turn might
                // already point to a different turn. Therefore we only want to
                // take the task if it's the same turn. We do this before the
                // dropped-tx guard below so the panel exits its generating
                // state even when the send_task is cancelled before tx.send().
                if is_same_turn {
                    this.running_turn.take();
                    this.set_foreground_state(
                        acp_v2::StateUpdate::Idle(acp_v2::IdleStateUpdate::new()),
                        cx,
                    );
                }

                if this.turn_id == turn_id {
                    this.shrink_message_source_capacity(first_entry_index);
                }

                let Ok(response) = response else {
                    if is_same_turn {
                        cx.emit(AcpThreadEvent::StatusChanged);
                    }
                    // tx dropped, just return
                    return Ok(None);
                };

                if this.turn_id != turn_id {
                    return response.map(Some);
                }

                match response {
                    Ok(r) => {
                        this.flush_streaming_text(cx);

                        if r.stop_reason == acp_v1::StopReason::MaxTokens {
                            if is_same_turn {
                                cx.emit(AcpThreadEvent::StatusChanged);
                            }
                            this.had_error = true;
                            cx.emit(AcpThreadEvent::Error);
                            log::error!("Max tokens reached. Usage: {:?}", this.token_usage);

                            let exceeded_max_output_tokens =
                                this.token_usage.as_ref().is_some_and(|u| {
                                    u.max_output_tokens
                                        .is_some_and(|max| u.output_tokens >= max)
                                });

                            if exceeded_max_output_tokens {
                                log::error!(
                                    "Max output tokens reached. Usage: {:?}",
                                    this.token_usage
                                );
                            } else {
                                log::error!("Max tokens reached. Usage: {:?}", this.token_usage);
                            }
                            if is_same_turn {
                                this.cancel_pending_turn_entries(cx);
                            }
                            return Err(anyhow!(MaxOutputTokensError));
                        }

                        let canceled = matches!(r.stop_reason, acp_v1::StopReason::Cancelled);
                        if canceled && is_same_turn {
                            this.cancel_pending_turn_entries(cx);
                        }

                        // Handle refusal - distinguish between user prompt and tool call refusals
                        if let acp_v1::StopReason::Refusal = r.stop_reason {
                            this.had_error = true;
                            if is_same_turn {
                                this.cancel_generic_permission_requests(cx);
                            }
                            if let Some((user_msg_ix, _)) = this.last_user_message() {
                                // Check if there's a completed tool call with results after the last user message
                                // This indicates the refusal is in response to tool output, not the user's prompt
                                let has_completed_tool_call_after_user_msg =
                                    this.entries.iter().skip(user_msg_ix + 1).any(|entry| {
                                        if let AgentThreadEntry::ToolCall(tool_call) = entry {
                                            // Check if the tool call has completed and has output
                                            matches!(tool_call.status(), ToolCallStatus::Completed)
                                                && tool_call.raw_output.is_some()
                                        } else {
                                            false
                                        }
                                    });

                                if has_completed_tool_call_after_user_msg {
                                    // Refusal is due to tool output - don't truncate, just notify
                                    // The model refused based on what the tool returned
                                    cx.emit(AcpThreadEvent::Refusal);
                                } else {
                                    // User prompt was refused - truncate back to before the user message
                                    let range = user_msg_ix..this.entries.len();
                                    if range.start < range.end {
                                        this.truncate_entries(user_msg_ix, cx);
                                        cx.emit(AcpThreadEvent::EntriesRemoved(range));
                                    }
                                    cx.emit(AcpThreadEvent::Refusal);
                                }
                            } else {
                                // No user message found, treat as general refusal
                                cx.emit(AcpThreadEvent::Refusal);
                            }
                        }

                        if cx.has_flag::<AcpBetaFeatureFlag>()
                            && let Some(response_usage) = &r.usage
                        {
                            let usage = this.token_usage.get_or_insert_with(Default::default);
                            usage.input_tokens = response_usage.input_tokens;
                            usage.output_tokens = response_usage.output_tokens;
                            cx.emit(AcpThreadEvent::TokenUsageUpdated);
                        }

                        if is_same_turn {
                            cx.emit(AcpThreadEvent::StatusChanged);
                        }
                        let stop_reason = stop_reason_from_v1(&r.stop_reason);
                        this.set_foreground_state(
                            acp_v2::StateUpdate::Idle(
                                acp_v2::IdleStateUpdate::new().stop_reason(stop_reason.clone()),
                            ),
                            cx,
                        );
                        cx.emit(AcpThreadEvent::Stopped {
                            activity_generation: this.activity.generation(),
                            activity_duration: this.activity.duration(),
                            stop_reason,
                        });
                        Ok(Some(r))
                    }
                    Err(e) => {
                        if is_same_turn {
                            cx.emit(AcpThreadEvent::StatusChanged);
                        }
                        this.flush_streaming_text(cx);
                        if is_same_turn {
                            this.cancel_pending_turn_entries(cx);
                        }
                        this.had_error = true;
                        cx.emit(AcpThreadEvent::Error);
                        log::error!("Error in run turn: {:?}", e);
                        Err(e)
                    }
                }
            })?
        };
        self.track_submission(id, cx, async move |thread, cx| {
            Ok(completion(thread, cx)
                .await?
                .map(SubmissionResponse::LegacyCompleted))
        })
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) -> Task<()> {
        self.cancel_inner(RequestPermissionOutcome::Cancelled, cx)
    }

    fn cancel_inner(
        &mut self,
        permission_outcome: RequestPermissionOutcome,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        self.flush_streaming_text(cx);
        if self.uses_reported_activity() {
            self.cancel_generic_permission_requests(cx);
            let Some(receiver) = self.activity.wait_for_idle() else {
                return Task::ready(());
            };
            self.connection.cancel(self.session_id(), cx);
            return cx.spawn(async move |_, _| {
                if receiver.await.is_err() {
                    log::debug!("Session released before foreground cancellation completed");
                }
            });
        }
        self.cancel_outstanding_elicitations(cx);

        let Some(turn) = self.running_turn.take() else {
            let request_ids = self.permission_requests.keys().copied().collect::<Vec<_>>();
            for id in request_ids {
                self.cancel_permission_request_with_outcome(id, permission_outcome.clone(), cx);
            }
            return Task::ready(());
        };
        self.shrink_message_source_capacity(turn.first_entry_index);
        self.mark_pending_entries_as_canceled(permission_outcome, cx);
        self.connection.cancel(self.session_id(), cx);
        self.set_foreground_state(
            acp_v2::StateUpdate::Idle(acp_v2::IdleStateUpdate::new()),
            cx,
        );
        cx.emit(AcpThreadEvent::StatusChanged);

        // Wait for the send task to complete
        cx.background_spawn(turn.send_task)
    }

    fn update_idle_sleep_prevention(&mut self, cx: &mut Context<Self>) {
        if !AgentSettings::get_global(cx).prevent_idle_sleep
            || self.foreground_activity() != ForegroundActivity::Running
        {
            self.idle_sleep_prevention = IdleSleepPrevention::Inactive;
            return;
        }

        if !matches!(self.idle_sleep_prevention, IdleSleepPrevention::Inactive) {
            return;
        }

        let acquisition = cx.prevent_idle_sleep("Agent thread in progress");
        self.idle_sleep_prevention = IdleSleepPrevention::Acquiring {
            _task: cx.spawn(async move |thread, cx| {
                let result = acquisition.await;
                thread
                    .update(cx, |thread, _| {
                        thread.idle_sleep_prevention = match result {
                            Ok(guard) => IdleSleepPrevention::Active { _guard: guard },
                            Err(error) => {
                                log::error!("Failed to prevent idle sleep: {error:#}");
                                IdleSleepPrevention::Failed
                            }
                        };
                    })
                    .log_err();
            }),
        };
    }

    fn cancel_pending_turn_entries(&mut self, cx: &mut Context<Self>) {
        self.mark_pending_entries_as_canceled(RequestPermissionOutcome::Cancelled, cx);
        self.cancel_outstanding_elicitations(cx);
    }

    fn mark_pending_entries_as_canceled(
        &mut self,
        permission_outcome: RequestPermissionOutcome,
        cx: &mut Context<Self>,
    ) {
        let mut canceled_requests = Vec::new();
        for (ix, entry) in self.entries.iter_mut().enumerate() {
            match entry {
                AgentThreadEntry::ToolCall(call) => {
                    let cancel = matches!(
                        call.status(),
                        ToolCallStatus::Pending
                            | ToolCallStatus::WaitingForConfirmation
                            | ToolCallStatus::InProgress
                    );
                    if cancel {
                        call.set_local_status(ToolCallStatus::Canceled);
                        if let Some(id) = call.authorization.take() {
                            canceled_requests.push(id);
                        }
                        cx.emit(AcpThreadEvent::EntryUpdated(ix));
                    }
                }
                AgentThreadEntry::ContextCompaction(compaction) => {
                    if compaction.status == ContextCompactionStatus::InProgress {
                        compaction.status = ContextCompactionStatus::Canceled;
                        cx.emit(AcpThreadEvent::EntryUpdated(ix));
                    }
                }
                _ => {}
            }
        }
        for id in canceled_requests {
            self.resolve_permission_request(id, permission_outcome.clone(), cx);
        }
        self.cancel_generic_permission_requests(cx);
    }

    pub fn cancel_outstanding_elicitations(&mut self, cx: &mut Context<Self>) {
        for ix in 0..self.entries.len() {
            let Some(AgentThreadEntry::Elicitation(elicitation_id)) = self.entries.get(ix) else {
                continue;
            };
            if let Some(change) = self.elicitations.cancel_elicitation_by_id(elicitation_id) {
                Self::emit_elicitation_change(ix, elicitation_id, change, cx);
            }
        }
    }

    /// Restores the git working tree to the state at the given checkpoint (if one exists)
    pub fn restore_checkpoint(
        &mut self,
        client_id: ClientUserMessageId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some((_, message)) = self.user_message_mut(&client_id) else {
            return Task::ready(Err(anyhow!("message not found")));
        };

        let checkpoint = message
            .checkpoint
            .as_ref()
            .map(|c| c.git_checkpoint.clone());

        // Cancel any in-progress generation before restoring
        let cancel_task = self.cancel(cx);
        let rewind = self.rewind(client_id.clone(), cx);
        let git_store = self.project.read(cx).git_store().clone();

        cx.spawn(async move |_, cx| {
            cancel_task.await;
            rewind.await?;
            if let Some(checkpoint) = checkpoint {
                git_store
                    .update(cx, |git, cx| git.restore_checkpoint(checkpoint, cx))
                    .await?;
            }

            Ok(())
        })
    }

    /// Rewinds this thread to before the entry at `index`, removing it and all
    /// subsequent entries while rejecting any action_log changes made from that point.
    /// Unlike `restore_checkpoint`, this method does not restore from git.
    pub fn rewind(
        &mut self,
        client_id: ClientUserMessageId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(truncate) = self.connection.truncate(self.session_id(), cx) else {
            return Task::ready(Err(anyhow!("not supported")));
        };

        self.flush_streaming_text(cx);
        let telemetry = ActionLogTelemetry::from(&*self);
        cx.spawn(async move |this, cx| {
            cx.update(|cx| truncate.run(client_id.clone(), cx)).await?;
            this.update(cx, |this, cx| {
                this.flush_streaming_text(cx);
                if let Some((ix, _)) = this.user_message_mut(&client_id) {
                    // Collect all terminals from entries that will be removed
                    let terminals_to_remove: Vec<acp_v2::TerminalId> = this.entries[ix..]
                        .iter()
                        .flat_map(|entry| entry.terminals())
                        .filter_map(|terminal| terminal.read(cx).id().clone().into())
                        .collect();

                    let range = ix..this.entries.len();
                    this.truncate_entries(ix, cx);
                    cx.emit(AcpThreadEvent::EntriesRemoved(range));

                    // Kill and remove the terminals
                    for terminal_id in terminals_to_remove {
                        if let Some(terminal) = this.terminals.remove(&terminal_id) {
                            terminal.update(cx, |terminal, cx| {
                                terminal.kill(cx);
                            });
                        }
                    }
                }
                this.action_log().update(cx, |action_log, cx| {
                    action_log.reject_all_edits(Some(telemetry), cx)
                })
            })?
            .await;
            Ok(())
        })
    }

    fn update_last_checkpoint_if_changed(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let Some(turn_id) = self.running_turn.as_ref().map(|turn| turn.id) else {
            return Task::ready(Ok(()));
        };

        let git_store = self.project.read(cx).git_store().clone();

        let Some((client_id, checkpoint)) = self.last_user_message().and_then(|(_, message)| {
            let id = message.client_id.clone()?;
            let checkpoint = message.checkpoint.as_ref()?;
            Some((id, checkpoint))
        }) else {
            return Task::ready(Ok(()));
        };
        if checkpoint.show {
            return Task::ready(Ok(()));
        }
        let old_checkpoint = checkpoint.git_checkpoint.clone();

        let new_checkpoint = git_store.update(cx, |git, cx| git.checkpoint(cx));
        cx.spawn(async move |this, cx| {
            let Some(new_checkpoint) = new_checkpoint
                .await
                .context("failed to get new checkpoint")
                .log_err()
            else {
                return Ok(());
            };

            let Some(equal) = git_store
                .update(cx, |git, cx| {
                    git.compare_checkpoints(old_checkpoint.clone(), new_checkpoint, cx)
                })
                .await
                .context("failed to compare checkpoints")
                .log_err()
            else {
                return Ok(());
            };

            if equal {
                return Ok(());
            }

            this.update(cx, |this, cx| {
                if !this
                    .running_turn
                    .as_ref()
                    .is_some_and(|turn| turn.id == turn_id)
                {
                    return;
                }

                let Some((ix, message)) = this.last_user_message() else {
                    return;
                };
                if message.client_id.as_ref() != Some(&client_id) {
                    return;
                }
                if let Some(checkpoint) = message.checkpoint.as_mut()
                    && !checkpoint.show
                {
                    checkpoint.show = true;
                    cx.emit(AcpThreadEvent::EntryUpdated(ix));
                }
            })?;

            Ok(())
        })
    }

    fn update_last_checkpoint(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let git_store = self.project.read(cx).git_store().clone();

        let Some((_, message)) = self.last_user_message() else {
            return Task::ready(Ok(()));
        };
        let Some(client_id) = message.client_id.clone() else {
            return Task::ready(Ok(()));
        };
        let Some(checkpoint) = message.checkpoint.as_ref() else {
            return Task::ready(Ok(()));
        };
        let old_checkpoint = checkpoint.git_checkpoint.clone();

        let new_checkpoint = git_store.update(cx, |git, cx| git.checkpoint(cx));
        cx.spawn(async move |this, cx| {
            let Some(new_checkpoint) = new_checkpoint
                .await
                .context("failed to get new checkpoint")
                .log_err()
            else {
                return Ok(());
            };

            let Some(equal) = git_store
                .update(cx, |git, cx| {
                    git.compare_checkpoints(old_checkpoint.clone(), new_checkpoint, cx)
                })
                .await
                .context("failed to compare checkpoints")
                .log_err()
            else {
                return Ok(());
            };

            this.update(cx, |this, cx| {
                if let Some((ix, message)) = this.user_message_mut(&client_id) {
                    if let Some(checkpoint) = message.checkpoint.as_mut() {
                        checkpoint.show = !equal;
                        cx.emit(AcpThreadEvent::EntryUpdated(ix));
                    }
                }
            })?;

            Ok(())
        })
    }

    fn last_user_message(&mut self) -> Option<(usize, &mut UserMessage)> {
        self.entries
            .iter_mut()
            .enumerate()
            .rev()
            .find_map(|(ix, entry)| {
                if let AgentThreadEntry::UserMessage(message) = entry {
                    Some((ix, message))
                } else {
                    None
                }
            })
    }

    fn user_message_mut(
        &mut self,
        client_id: &ClientUserMessageId,
    ) -> Option<(usize, &mut UserMessage)> {
        self.entries.iter_mut().enumerate().find_map(|(ix, entry)| {
            if let AgentThreadEntry::UserMessage(message) = entry {
                if message.client_id.as_ref() == Some(client_id) {
                    Some((ix, message))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn prune_dead_shared_buffers(&mut self) {
        self.shared_buffers
            .retain(|buffer, _| buffer.is_upgradable());
    }

    pub fn read_text_file(
        &self,
        path: PathBuf,
        line: Option<u32>,
        limit: Option<u32>,
        reuse_shared_snapshot: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<String, acp_v1::Error>> {
        // Args are 1-based, move to 0-based
        let line = line.unwrap_or_default().saturating_sub(1);
        let limit = limit.unwrap_or(u32::MAX);
        let project = self.project.clone();
        let action_log = self.action_log.clone();
        let should_update_agent_location = self.parent_session_id.is_none();
        cx.spawn(async move |this, cx| {
            let load = project.update(cx, |project, cx| {
                let path = project
                    .project_path_for_absolute_path(&path, cx)
                    .ok_or_else(|| {
                        acp_v1::Error::resource_not_found(Some(path.display().to_string()))
                    })?;
                Ok::<_, acp_v1::Error>(project.open_buffer(path, cx))
            })?;

            let buffer = load.await?;

            let snapshot = if reuse_shared_snapshot {
                this.read_with(cx, |this, _| {
                    this.shared_buffers.get(&buffer.downgrade()).cloned()
                })
                .log_err()
                .flatten()
            } else {
                None
            };

            let snapshot = if let Some(snapshot) = snapshot {
                snapshot
            } else {
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_read(buffer.clone(), cx);
                });

                let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
                this.update(cx, |this, _| {
                    this.prune_dead_shared_buffers();
                    this.shared_buffers
                        .insert(buffer.downgrade(), snapshot.clone());
                })?;
                snapshot
            };

            let max_point = snapshot.max_point();
            let start_position = Point::new(line, 0);

            if start_position > max_point {
                return Err(acp_v1::Error::invalid_params().data(format!(
                    "Attempting to read beyond the end of the file, line {}:{}",
                    max_point.row + 1,
                    max_point.column
                )));
            }

            let start = snapshot.anchor_before(start_position);
            let end = snapshot.anchor_before(Point::new(line.saturating_add(limit), 0));

            if should_update_agent_location {
                project.update(cx, |project, cx| {
                    project.set_agent_location(
                        Some(AgentLocation {
                            buffer: buffer.downgrade(),
                            position: start,
                        }),
                        cx,
                    );
                });
            }

            Ok(snapshot.text_for_range(start..end).collect::<String>())
        })
    }

    pub fn write_text_file(
        &self,
        path: PathBuf,
        content: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let project = self.project.clone();
        let action_log = self.action_log.clone();
        let should_update_agent_location = self.parent_session_id.is_none();
        cx.spawn(async move |this, cx| {
            let load = project.update(cx, |project, cx| {
                let path = project
                    .project_path_for_absolute_path(&path, cx)
                    .context("invalid path")?;
                anyhow::Ok(project.open_buffer(path, cx))
            });
            let buffer = load?.await?;
            let snapshot = this.update(cx, |this, cx| {
                this.shared_buffers
                    .get(&buffer.downgrade())
                    .cloned()
                    .unwrap_or_else(|| buffer.read(cx).snapshot())
            })?;
            let edits = cx
                .background_executor()
                .spawn(async move {
                    let old_text = snapshot.text();
                    text_diff(old_text.as_str(), &content)
                        .into_iter()
                        .map(|(range, replacement)| {
                            (snapshot.anchor_range_inside(range), replacement)
                        })
                        .collect::<Vec<_>>()
                })
                .await;

            if should_update_agent_location {
                project.update(cx, |project, cx| {
                    project.set_agent_location(
                        Some(AgentLocation {
                            buffer: buffer.downgrade(),
                            position: edits
                                .last()
                                .map(|(range, _)| range.end)
                                .unwrap_or(Anchor::min_for_buffer(buffer.read(cx).remote_id())),
                        }),
                        cx,
                    );
                });
            }

            let format_on_save = cx.update(|cx| {
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_read(buffer.clone(), cx);
                });

                let format_on_save = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction();
                    buffer.edit(edits, None, cx);
                    buffer.end_transaction_with_source(BufferEditSource::Agent, cx);

                    let settings =
                        language::language_settings::LanguageSettings::for_buffer(buffer, cx);

                    settings.format_on_save != FormatOnSave::Off
                });
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_edited(buffer.clone(), cx);
                });
                format_on_save
            });

            if format_on_save {
                let format_task = project.update(cx, |project, cx| {
                    project.format(
                        HashSet::from_iter([buffer.clone()]),
                        LspFormatTarget::Buffers,
                        false,
                        FormatTrigger::Save,
                        cx,
                    )
                });
                format_task.await.log_err();

                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_edited(buffer.clone(), cx);
                });
            }

            project
                .update(cx, |project, cx| project.save_buffer(buffer, cx))
                .await
        })
    }

    pub fn create_terminal(
        &self,
        command: String,
        args: Vec<String>,
        extra_env: Vec<acp_v1::EnvVariable>,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        sandbox_wrap: Option<SandboxWrap>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let env = match &cwd {
            Some(dir) => self.project.update(cx, |project, cx| {
                project.environment().update(cx, |env, cx| {
                    env.directory_environment(dir.as_path().into(), cx)
                })
            }),
            None => Task::ready(None).shared(),
        };
        let env = cx.spawn(async move |_, _| {
            let mut env = env.await.unwrap_or_default();

            disable_pagers_through_env(&mut env);

            for var in extra_env {
                env.insert(var.name, var.value);
            }
            env
        });

        let project = self.project.clone();
        let language_registry = project.read(cx).languages().clone();
        let is_windows = project.read(cx).path_style(cx).is_windows();
        // Headless hosts (e.g. the eval CLI) have no controlling TTY, so PTY
        // setup fails with `ENOTTY`. Run the command non-interactively and
        // without a PTY in that case.
        let headless = HeadlessTerminal::is_enabled(cx);

        let terminal_id = acp_v2::TerminalId::new(Uuid::new_v4().to_string());
        let terminal_task = cx.spawn({
            let terminal_id = terminal_id.clone();
            async move |_this, cx| {
                let env = env.await;
                let shell = project
                    .update(cx, |project, cx| {
                        project
                            .remote_client()
                            .and_then(|r| r.read(cx).default_system_shell())
                    })
                    .unwrap_or_else(|| get_default_system_shell_preferring_bash());

                // The sandbox owns the network proxy (for restricted-network
                // policies) and injects the child's proxy env vars, returning
                // the env to spawn with. On Windows, restricted host access is
                // rejected inside the sandbox before command preparation.
                #[cfg(target_os = "windows")]
                let (task_command, task_args, task_env, sandbox, spawn_cwd) =
                    if sandbox_wrap.is_some() {
                        let (task_command, task_args) = task::ShellBuilder::new(
                            &Shell::Program("/bin/sh".to_string()),
                            false,
                        )
                        .non_interactive()
                        .redirect_stdin_to_dev_null()
                        .build(Some(command.clone()), &args);
                        let wrap = cx.background_spawn(prepare_sandbox_wrap(
                            task_command,
                            task_args,
                            cwd.clone(),
                            sandbox_wrap,
                            env,
                        ));
                        let timeout = cx.background_executor().timer(WSL_SANDBOX_WRAP_TIMEOUT);
                        let (task_command, task_args, task_env, sandbox) = futures::select_biased! {
                            result = wrap.fuse() => result?,
                            _ = timeout.fuse() => return Err(anyhow::Error::new(
                                sandbox::SandboxError::WslUnavailable(format!(
                                    "WSL did not respond within {} seconds while preparing the sandboxed command",
                                    WSL_SANDBOX_WRAP_TIMEOUT.as_secs()
                                )),
                            )),
                        };
                        (task_command, task_args, task_env, sandbox, None)
                    } else {
                        // No sandbox wrap means we're running unsandboxed, and
                        // on Windows that deliberately changes the shell: the
                        // sandboxed path runs under WSL's Linux bash, but this
                        // fallback uses the host's `shell` against the native cwd.
                        let mut builder = ShellBuilder::new(&Shell::Program(shell), is_windows);
                        if headless {
                            builder = builder.non_interactive();
                        }
                        let (task_command, task_args) = builder
                            .redirect_stdin_to_dev_null()
                            .build(Some(command.clone()), &args);
                        (task_command, task_args, env, None, cwd.clone())
                    };

                #[cfg(not(target_os = "windows"))]
                let (task_command, task_args, task_env, sandbox, spawn_cwd) = {
                    let mut builder = ShellBuilder::new(&Shell::Program(shell), is_windows);
                    if headless {
                        builder = builder.non_interactive();
                    }
                    let (task_command, task_args) = builder
                        .redirect_stdin_to_dev_null()
                        .build(Some(command.clone()), &args);
                    let (task_command, task_args, task_env, sandbox) = cx
                        .background_spawn(prepare_sandbox_wrap(
                            task_command,
                            task_args,
                            cwd.clone(),
                            sandbox_wrap,
                            env,
                        ))
                        .await?;
                    (task_command, task_args, task_env, sandbox, cwd.clone())
                };
                let terminal = project
                    .update(cx, |project, cx| {
                        project.create_terminal_task(
                            task::SpawnInTerminal {
                                command: Some(task_command),
                                args: task_args,
                                cwd: spawn_cwd,
                                env: task_env,
                                ..Default::default()
                            },
                            cx,
                        )
                    })
                    .await?;

                anyhow::Ok(cx.new(|cx| {
                    Terminal::new(
                        terminal_id,
                        &format!("{} {}", command, args.join(" ")),
                        cwd,
                        output_byte_limit.map(|l| l as usize),
                        terminal,
                        language_registry,
                        sandbox,
                        cx,
                    )
                }))
            }
        });

        cx.spawn(async move |this, cx| {
            let terminal = terminal_task.await?;
            this.update(cx, |this, _cx| {
                this.terminals.insert(terminal_id, terminal.clone());
                terminal
            })
        })
    }

    pub fn kill_terminal(
        &mut self,
        terminal_id: acp_v2::TerminalId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.terminals
            .get(&terminal_id)
            .context("Terminal not found")?
            .update(cx, |terminal, cx| {
                terminal.kill(cx);
            });

        Ok(())
    }

    pub fn release_terminal(
        &mut self,
        terminal_id: acp_v2::TerminalId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.terminals
            .remove(&terminal_id)
            .context("Terminal not found")?
            .update(cx, |terminal, cx| {
                terminal.kill(cx);
            });

        Ok(())
    }

    pub fn terminal(&self, terminal_id: acp_v2::TerminalId) -> Result<Entity<Terminal>> {
        self.terminals
            .get(&terminal_id)
            .context("Terminal not found")
            .cloned()
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        self.entries
            .iter()
            .map(|entry| match entry {
                AgentThreadEntry::Elicitation(elicitation_id) => self
                    .elicitations
                    .elicitation(elicitation_id)
                    .map(|(_, elicitation)| {
                        format!("## Input Requested\n\n{}\n\n", elicitation.request.message)
                    })
                    .unwrap_or_else(|| entry.to_markdown(cx)),
                _ => entry.to_markdown(cx),
            })
            .collect()
    }

    pub fn emit_load_error(&mut self, error: LoadError, cx: &mut Context<Self>) {
        cx.emit(AcpThreadEvent::LoadError(error));
    }

    pub fn register_terminal_created(
        &mut self,
        terminal_id: acp_v2::TerminalId,
        command_label: String,
        working_dir: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        terminal: Entity<::terminal::Terminal>,
        cx: &mut Context<Self>,
    ) -> Entity<Terminal> {
        let language_registry = self.project.read(cx).languages().clone();

        let entity = cx.new(|cx| {
            Terminal::new(
                terminal_id.clone(),
                &command_label,
                working_dir.clone(),
                output_byte_limit.map(|l| l as usize),
                terminal,
                language_registry,
                // External terminal providers manage their own sandboxing
                // (if any). We don't wrap their commands.
                None,
                cx,
            )
        });
        self.terminals.insert(terminal_id.clone(), entity.clone());
        entity
    }

    pub fn mark_as_subagent_output(&mut self, cx: &mut Context<Self>) {
        for entry in self.entries.iter_mut().rev() {
            if let AgentThreadEntry::AssistantMessage(assistant_message) = entry {
                assistant_message.is_subagent_output = true;
                cx.notify();
                return;
            }
        }
    }

    pub fn upsert_display_terminal(
        &mut self,
        terminal_id: acp_v2::TerminalId,
        patch: DisplayTerminalPatch,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let terminal = self.ensure_display_terminal(terminal_id, cx);
        terminal.update(cx, |terminal, cx| terminal.apply_display_patch(patch, cx))
    }

    pub fn append_display_terminal_output(
        &mut self,
        terminal_id: acp_v2::TerminalId,
        data: &[u8],
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let terminal = self.ensure_display_terminal(terminal_id, cx);
        terminal.update(cx, |terminal, cx| terminal.append_display_bytes(data, cx))
    }

    fn ensure_display_terminal(
        &mut self,
        terminal_id: acp_v2::TerminalId,
        cx: &mut Context<Self>,
    ) -> Entity<Terminal> {
        if let Some(terminal) = self.terminals.get(&terminal_id) {
            return terminal.clone();
        }
        let builder = ::terminal::TerminalBuilder::new_display_only(
            ::terminal::terminal_settings::CursorShape::default(),
            ::terminal::terminal_settings::AlternateScroll::On,
            None,
            0,
            cx.background_executor(),
            self.project.read(cx).path_style(cx),
        );
        let terminal = cx.new(|cx| builder.subscribe(cx));
        self.register_display_terminal(terminal_id, None, None, None, terminal, cx)
    }

    fn register_display_terminal(
        &mut self,
        terminal_id: acp_v2::TerminalId,
        command: Option<&str>,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        terminal: Entity<::terminal::Terminal>,
        cx: &mut Context<Self>,
    ) -> Entity<Terminal> {
        let language_registry = self.project.read(cx).languages().clone();
        let entity = cx.new(|cx| {
            Terminal::new_display(
                terminal_id.clone(),
                command,
                cwd,
                output_byte_limit.map(|limit| limit as usize),
                terminal,
                language_registry,
                cx,
            )
        });
        // Provider state can change without another tool-call update.
        cx.observe(&entity, |this, terminal, cx| {
            for (index, entry) in this.entries.iter().enumerate() {
                if entry
                    .terminals()
                    .any(|entry_terminal| entry_terminal == &terminal)
                {
                    cx.emit(AcpThreadEvent::EntryUpdated(index));
                }
            }
            cx.notify();
        })
        .detach();
        self.terminals.insert(terminal_id.clone(), entity.clone());
        if let Some(chunks) = self.pending_terminal_output.remove(&terminal_id) {
            entity.update(cx, |terminal, cx| {
                for data in chunks {
                    terminal.write_display_output(&data, cx);
                }
            });
        }
        if let Some(status) = self.pending_terminal_exit.remove(&terminal_id) {
            entity.update(cx, |terminal, cx| terminal.finish_display(status, cx));
        }
        entity
    }

    pub fn on_terminal_provider_event(
        &mut self,
        event: TerminalProviderEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            TerminalProviderEvent::Created {
                terminal_id,
                label,
                cwd,
                output_byte_limit,
                terminal,
            } => {
                let terminal_id = acp_v2::TerminalId::new(terminal_id.0);
                let entity = self
                    .terminals
                    .get(&terminal_id)
                    .cloned()
                    .unwrap_or_else(|| {
                        self.register_display_terminal(
                            terminal_id,
                            Some(&label),
                            cwd.clone(),
                            output_byte_limit,
                            terminal,
                            cx,
                        )
                    });
                // A prior reference may already have a view; keep its renderer when metadata arrives.
                entity.update(cx, |terminal, cx| {
                    terminal.initialize_legacy_display(
                        &label,
                        cwd,
                        output_byte_limit.map(|limit| limit as usize),
                        cx,
                    );
                });

                cx.notify();
            }
            TerminalProviderEvent::Output { terminal_id, data } => {
                let terminal_id = acp_v2::TerminalId::new(terminal_id.0);
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |term, cx| {
                        term.write_display_output(&data, cx);
                    });
                } else {
                    self.pending_terminal_output
                        .entry(terminal_id)
                        .or_default()
                        .push(data);
                }
            }
            TerminalProviderEvent::TitleChanged { terminal_id, title } => {
                let terminal_id = acp_v2::TerminalId::new(terminal_id.0);
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |term, cx| {
                        term.inner().update(cx, |inner, cx| {
                            inner.breadcrumb_text = title;
                            cx.emit(::terminal::Event::BreadcrumbsChanged);
                        })
                    });
                }
            }
            TerminalProviderEvent::Exit {
                terminal_id,
                status,
            } => {
                let terminal_id = acp_v2::TerminalId::new(terminal_id.0);
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |term, cx| term.finish_display(status, cx));
                } else {
                    self.pending_terminal_exit
                        .entry(terminal_id)
                        .or_insert(status);
                }
            }
        }
    }
}

fn markdown_for_raw_output(
    raw_output: &serde_json::Value,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut App,
) -> Option<Entity<Markdown>> {
    let text = raw_output_text(raw_output)?;
    Some(cx.new(|cx| Markdown::new(text.into(), Some(language_registry.clone()), None, cx)))
}

fn raw_output_text(raw_output: &serde_json::Value) -> Option<String> {
    match raw_output {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(value) => Some(value.to_string()),
        serde_json::Value::Number(value) => Some(value.to_string()),
        serde_json::Value::String(value) => Some(value.clone()),
        value => {
            let pretty_json = to_string_pretty(value).unwrap_or_else(|_| value.to_string());
            Some(format!("```json\n{}\n```", pretty_json))
        }
    }
}

fn update_markdown_in_place(markdown: &Entity<Markdown>, text: &str, cx: &mut App) {
    markdown.update(cx, |markdown, cx| {
        match text.strip_prefix(markdown.source().as_ref()) {
            Some("") => {}
            Some(suffix) => markdown.append(suffix, cx),
            None => markdown.reset(text.to_owned().into(), cx),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use futures::stream::StreamExt as _;
    use futures::{channel::mpsc, future::LocalBoxFuture, select};
    use gpui::UpdateGlobal as _;
    use gpui::{App, AsyncApp, TestAppContext, WeakEntity};
    use indoc::indoc;
    use project::{AgentId, FakeFs, Fs, RemoveOptions};
    use rand::{distr, prelude::*};
    use serde_json::json;
    use settings::SettingsStore;
    use std::{
        any::Any,
        cell::RefCell,
        path::Path,
        rc::Rc,
        sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
        time::Duration,
    };
    use util::{path, path_list::PathList};

    #[test]
    fn test_text_cursor_reads_source_on_utf8_boundaries() {
        let sources =
            ["visible", "", "aé", "", "🦀b", "", "日本語"].map(acp_v2::ContentBlock::from);
        let whole = "aé🦀b日本語";
        for max_bytes in 1..=whole.len() + 1 {
            let mut cursor = TextCursor {
                source_index: 1,
                byte_offset: 0,
                pending_bytes: whole.len(),
            };
            assert_eq!(cursor.take(&sources, 0).as_deref(), Some(""));

            let mut expected = whole.to_string();
            while !expected.is_empty() {
                assert_eq!(cursor.pending_bytes, expected.len());
                let boundary = expected.ceil_char_boundary(max_bytes);
                assert_eq!(
                    cursor.take(&sources, max_bytes),
                    Some(expected.drain(..boundary).collect::<String>())
                );
            }
            assert_eq!(cursor.pending_bytes, 0);
            assert_eq!(cursor.take(&sources, usize::MAX).as_deref(), Some(""));
        }

        let mut sources = vec!["aé".into()];
        let mut cursor = TextCursor {
            pending_bytes: 3,
            ..Default::default()
        };
        assert_eq!(cursor.take(&sources, 1).as_deref(), Some("a"));
        sources.push("🦀b".into());
        cursor.pending_bytes += 5;
        assert_eq!(cursor.take(&sources, 3).as_deref(), Some("é🦀"));
        sources.push("日本語".into());
        cursor.pending_bytes += 9;
        assert_eq!(
            cursor.take(&sources, usize::MAX).as_deref(),
            Some("b日本語")
        );
        sources.push("aé".into());
        cursor.pending_bytes += 3;
        assert_eq!(cursor.take(&sources, usize::MAX).as_deref(), Some("aé"));
        assert_eq!(cursor.pending_bytes, 0);
    }

    #[gpui::test]
    async fn test_streaming_cursor_is_flushed_before_rewind(cx: &mut TestAppContext) {
        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("session should be created");
        let client_id = ClientUserMessageId::new();

        let previous_markdown = thread.update(cx, |thread, cx| {
            thread.push_user_content_block(Some(client_id.clone()), "prompt".into(), cx);
            thread.push_assistant_content_block("before".into(), false, cx);
            thread.push_assistant_content_block(" buffered".into(), false, cx);
            let (_, target) = thread
                .streaming_content_target(None, false, false)
                .expect("streaming target");
            assert_eq!(target.markdown.read(cx).source(), "before");
            thread.cancel(cx).detach();
            assert_eq!(target.markdown.read(cx).source(), "before buffered");
            assert!(thread.streaming_text_buffer.is_none());
            target.markdown
        });

        let rewind = thread.update(cx, |thread, cx| {
            let rewind = thread.rewind(client_id, cx);
            thread.push_assistant_content_block(" late".into(), false, cx);
            rewind
        });
        rewind.await.expect("rewind should complete");
        thread.read_with(cx, |thread, cx| {
            assert!(thread.entries().is_empty());
            assert!(thread.streaming_text_buffer.is_none());
            assert_eq!(previous_markdown.read(cx).source(), "before buffered late");
        });

        let current_markdown = thread.update(cx, |thread, cx| {
            thread.push_user_content_block(None, "new prompt".into(), cx);
            thread.push_assistant_content_block("fresh".into(), false, cx);
            thread.push_assistant_content_block(" streamed".into(), false, cx);
            let (_, target) = thread
                .streaming_content_target(None, false, false)
                .expect("new streaming target");
            target.markdown
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        current_markdown.read_with(cx, |markdown, _| {
            assert_eq!(markdown.source(), "fresh streamed");
        });
        previous_markdown.read_with(cx, |markdown, _| {
            assert_eq!(markdown.source(), "before buffered late");
        });
    }

    #[test]
    fn command_category_meta_round_trips() {
        // Exhaustive list of variants. The match below has no wildcard arm, so
        // adding a `CommandCategory` variant fails to compile here until it's
        // covered, keeping the `as_str`/`from_str` wire contract in sync.
        let all = [CommandCategory::Native, CommandCategory::Mcp];
        for category in all {
            match category {
                CommandCategory::Native | CommandCategory::Mcp => {}
            }
            let meta = meta_with_command_category(category);
            assert_eq!(command_category_from_meta(&Some(meta)), Some(category));
        }

        // Absent meta and unknown categories both decode to `None`.
        assert_eq!(command_category_from_meta(&None), None);
        let unknown =
            acp_v1::Meta::from_iter([(COMMAND_CATEGORY_META_KEY.into(), "future-category".into())]);
        assert_eq!(command_category_from_meta(&Some(unknown)), None);
    }

    #[test]
    fn client_user_message_id_serializes_as_string() {
        let serialized =
            serde_json::to_value(ClientUserMessageId::new()).expect("serialize client message id");
        assert!(
            serialized.is_string(),
            "expected string, got {serialized:?}"
        );

        let deserialized: ClientUserMessageId =
            serde_json::from_value(json!("client-id")).expect("deserialize client message id");
        assert_eq!(
            serde_json::to_value(deserialized).expect("serialize client message id"),
            json!("client-id")
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let mut settings_store = SettingsStore::test(cx);
            settings_store.register_setting::<feature_flags::FeatureFlagsSettings>();
            cx.set_global(settings_store);
        });
    }

    #[test]
    fn test_legacy_plan_entry_conversion_preserves_fields() {
        for priority in [
            acp_v1::PlanEntryPriority::High,
            acp_v1::PlanEntryPriority::Medium,
            acp_v1::PlanEntryPriority::Low,
        ] {
            for status in [
                acp_v1::PlanEntryStatus::Pending,
                acp_v1::PlanEntryStatus::InProgress,
                acp_v1::PlanEntryStatus::Completed,
            ] {
                for meta in [
                    None,
                    Some(acp_v1::Meta::new()),
                    Some(acp_v1::Meta::from_iter([(
                        "nested".into(),
                        json!({"value": [1, null]}),
                    )])),
                ] {
                    let entry =
                        acp_v1::PlanEntry::new("Task 🦀\n`code`", priority.clone(), status.clone())
                            .meta(meta);
                    let expected = serde_json::to_value(&entry).expect("v1 entry");
                    let converted = plan_entry_from_v1(entry).expect("supported v1 entry");
                    assert_eq!(serde_json::to_value(converted).expect("v2 entry"), expected);
                }
            }
        }
    }

    #[gpui::test]
    async fn test_keyed_plan_metadata_and_unknown_values_are_retained(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let id = acp_v2::PlanId::new("plan");
            let mut items: acp_v2::PlanItems = serde_json::from_value(json!({
                "planId": "plan",
                "entries": [
                    {"content": "Unknown state", "priority": "_urgent", "status": "_blocked", "_meta": {"entry": [1, null]}},
                    {"content": "Finished", "priority": "medium", "status": "completed"},
                    {"content": "Not needed", "priority": "low", "status": "cancelled"}
                ],
                "_meta": {"plan": true}
            })).expect("keyed items");
            let update_meta = Some(acp_v2::Meta::from_iter([("update".into(), json!(true))]));
            thread.upsert_plan_items(items.clone(), update_meta.clone(), cx);
            let plan = thread.plan().expect("visible plan");
            assert_eq!(plan.meta, items.meta);
            assert_eq!(plan.update_meta, update_meta);
            for (entry, source) in plan.entries.iter().zip(&items.entries) {
                assert_eq!(&entry.source, source);
                assert_eq!(entry.content.read(cx).source().as_ref(), source.content);
            }
            let markdown = plan.entries[0].content.clone();
            let stats = plan.stats();
            assert_eq!((stats.pending, stats.completed, stats.cancelled), (1, 1, 1));
            assert!(stats.in_progress_entry.is_none());
            thread.clear_plan(cx);
            thread.upsert_plan_items(items.clone(), update_meta, cx);
            assert!(thread.plan().is_none(), "an identical snapshot does not undo dismissal");
            items.meta = Some(acp_v2::Meta::new());
            items.entries[0].meta = Some(acp_v2::Meta::new());
            thread.upsert_plan_items(items.clone(), Some(acp_v2::Meta::new()), cx);
            assert!(thread.plan().is_none());
            let plan = thread.plan_by_id(&id).expect("retained hidden plan");
            assert_eq!(plan.meta, Some(acp_v2::Meta::new()));
            assert_eq!(plan.update_meta, Some(acp_v2::Meta::new()));
            assert_eq!(plan.entries[0].source.meta, Some(acp_v2::Meta::new()));
            assert_eq!(plan.entries[0].content, markdown);
            items.meta = None;
            items.entries[0].meta = None;
            thread.upsert_plan_items(items.clone(), None, cx);
            let plan = thread.plan_by_id(&id).expect("retained plan");
            assert_eq!(plan.meta, None);
            assert_eq!(plan.update_meta, None);
            assert_eq!(plan.entries[0].source.meta, None);
            assert!(thread.plan().is_none(), "absent metadata replaces, but does not show the plan");
            items.entries[0].status = acp_v2::PlanEntryStatus::Other("_waiting".into());
            thread.upsert_plan_items(items, None, cx);
            let plan = thread.plan().expect("changed unknown status reopens the plan");
            assert_eq!(plan.entries[0].content, markdown);
            assert_eq!(plan.stats().pending, 1);
            assert!(thread.entries().is_empty(), "plans never become transcript entries");
        });
    }

    #[gpui::test]
    async fn test_keyed_plan_task_changes_reopen_dismissed_plan(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let original = vec![
                acp_v2::PlanEntry::new(
                    "First",
                    acp_v2::PlanEntryPriority::Medium,
                    acp_v2::PlanEntryStatus::Pending,
                ),
                acp_v2::PlanEntry::new(
                    "Second",
                    acp_v2::PlanEntryPriority::Low,
                    acp_v2::PlanEntryStatus::Pending,
                ),
            ];
            let mut content = original.clone();
            content[0].content = "First, revised".into();
            let mut status = original.clone();
            status[0].status = acp_v2::PlanEntryStatus::InProgress;
            let mut priority = original.clone();
            priority[0].priority = acp_v2::PlanEntryPriority::High;
            let mut reordered = original.clone();
            reordered.swap(0, 1);
            let mut appended = original.clone();
            appended.push(acp_v2::PlanEntry::new(
                "Third",
                acp_v2::PlanEntryPriority::Low,
                acp_v2::PlanEntryStatus::Pending,
            ));
            for updated in [
                content,
                status,
                priority,
                reordered,
                appended,
                vec![original[0].clone()],
                vec![],
            ] {
                thread.upsert_plan_items(
                    acp_v2::PlanItems::new("plan", original.clone()),
                    None,
                    cx,
                );
                let markdown = thread.plan_by_id(&"plan".into()).expect("plan").entries[0]
                    .content
                    .clone();
                thread.clear_plan(cx);
                thread.upsert_plan_items(acp_v2::PlanItems::new("plan", updated.clone()), None, cx);
                let plan = thread.plan().expect("meaningful changes select the plan");
                assert_eq!(plan.entries.len(), updated.len());
                for (entry, source) in plan.entries.iter().zip(&updated) {
                    assert_eq!(&entry.source, source);
                    assert_eq!(entry.content.read(cx).source().as_ref(), source.content);
                }
                if let Some(first) = plan.entries.first() {
                    assert_eq!(first.content, markdown);
                }
            }
        });
    }

    #[gpui::test]
    async fn test_keyed_plan_selection_and_empty_snapshots(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let entries = vec![acp_v2::PlanEntry::new(
                "Same task",
                acp_v2::PlanEntryPriority::Medium,
                acp_v2::PlanEntryStatus::Pending,
            )];
            thread.upsert_plan_items(acp_v2::PlanItems::new("a", entries.clone()), None, cx);
            let first_markdown = thread.plan().expect("A").entries[0].content.clone();
            thread.upsert_plan_items(acp_v2::PlanItems::new("b", entries.clone()), None, cx);
            let second_markdown = thread
                .plan()
                .expect("new B selects despite identical contents")
                .entries[0]
                .content
                .clone();
            assert_ne!(
                first_markdown, second_markdown,
                "identity owns its render cache"
            );
            thread.upsert_plan_items(
                acp_v2::PlanItems::new("a", entries.clone()).meta(acp_v2::Meta::new()),
                Some(acp_v2::Meta::new()),
                cx,
            );
            assert_eq!(thread.visible_plan, Some(PlanIdentity::Keyed("b".into())));
            thread.upsert_plan_items(acp_v2::PlanItems::new("empty", vec![]), None, cx);
            assert!(thread.plan().expect("selected empty plan").is_empty());
            thread.upsert_plan_items(acp_v2::PlanItems::new("a", entries.clone()), None, cx);
            assert_eq!(
                thread.visible_plan,
                Some(PlanIdentity::Keyed("empty".into()))
            );
            let mut changed = entries;
            changed[0].content = "Changed".into();
            thread.upsert_plan_items(acp_v2::PlanItems::new("a", changed), None, cx);
            assert_eq!(thread.visible_plan, Some(PlanIdentity::Keyed("a".into())));
            assert_eq!(
                thread.plan().expect("A reopens").entries[0].content,
                first_markdown
            );
            thread.upsert_plan_items(acp_v2::PlanItems::new("b", vec![]), None, cx);
            assert_eq!(thread.visible_plan, Some(PlanIdentity::Keyed("b".into())));
            assert!(
                thread
                    .plan()
                    .expect("clearing older B selects empty B")
                    .is_empty()
            );
            assert_eq!(
                thread
                    .plan_by_id(&"a".into())
                    .expect("A retained")
                    .entries
                    .len(),
                1
            );
            assert!(thread.entries().is_empty());
        });
    }

    #[gpui::test]
    async fn test_legacy_plan_cleanup_does_not_mutate_keyed_plans(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let keyed = acp_v2::PlanItems::new(
            "legacy",
            vec![
                acp_v2::PlanEntry::new(
                    "Keyed completion",
                    acp_v2::PlanEntryPriority::High,
                    acp_v2::PlanEntryStatus::Completed,
                ),
                acp_v2::PlanEntry::new(
                    "Keyed cancellation",
                    acp_v2::PlanEntryPriority::Low,
                    acp_v2::PlanEntryStatus::Cancelled,
                ),
            ],
        )
        .meta(acp_v2::Meta::from_iter([("keyed".into(), json!(true))]));
        thread.update(cx, |thread, cx| {
            let legacy = acp_v1::Plan::new(vec![
                acp_v1::PlanEntry::new(
                    "Done",
                    acp_v1::PlanEntryPriority::High,
                    acp_v1::PlanEntryStatus::Completed,
                ),
                acp_v1::PlanEntry::new(
                    "Pending",
                    acp_v1::PlanEntryPriority::Medium,
                    acp_v1::PlanEntryStatus::Pending,
                )
                .meta(acp_v1::Meta::new()),
            ])
            .meta(acp_v1::Meta::from_iter([("legacy".into(), json!(true))]));
            thread
                .update_plan(legacy.clone(), cx)
                .expect("legacy snapshot");
            assert_eq!(thread.plan().expect("legacy plan").meta, legacy.meta);
            assert_eq!(thread.plan().expect("legacy plan").update_meta, None);
            assert_eq!(
                thread.plan().expect("legacy plan").entries[1].source.meta,
                Some(acp_v2::Meta::new())
            );
            thread.clear_plan(cx);
            assert!(thread.plan().is_none());
            thread
                .update_plan(legacy, cx)
                .expect("identical legacy update restores the panel");
            assert_eq!(
                thread.plan().expect("legacy compatibility").entries.len(),
                2
            );
            thread.upsert_plan_items(keyed.clone(), None, cx);
            assert_eq!(thread.plans.len(), 2, "legacy has no reserved protocol ID");
        });
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.visible_plan,
                Some(PlanIdentity::Keyed("legacy".into()))
            );
            let retained = thread.plan_by_id(&keyed.plan_id).expect("keyed plan");
            assert_eq!(retained.meta, keyed.meta);
            assert_eq!(retained.entries.len(), keyed.entries.len());
            for (entry, expected) in retained.entries.iter().zip(&keyed.entries) {
                assert_eq!(&entry.source, expected);
            }
            let legacy = thread
                .plans
                .get(&PlanIdentity::Legacy)
                .expect("legacy record");
            assert_eq!(legacy.entries.len(), 1);
            assert_eq!(legacy.entries[0].source.content, "Pending");
        });
        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn is running");
        request.await.expect("turn completes");
    }

    fn message_test_image() -> acp_v2::ContentBlock {
        acp_v2::ContentBlock::Image(acp_v2::ImageContent::new(
            "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
            "image/png",
        ))
    }

    #[gpui::test]
    fn test_message_content_retains_source_before_rendering(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let annotations = acp_v2::Annotations::new().priority(0.5);
            let blocks = vec![
                acp_v2::ContentBlock::Text(
                    acp_v2::TextContent::new("")
                        .annotations(annotations.clone())
                        .meta(acp_v1::Meta::from_iter([("empty".into(), json!(true))])),
                ),
                acp_v2::ContentBlock::Text(
                    acp_v2::TextContent::new("hello ")
                        .meta(acp_v1::Meta::from_iter([("part".into(), json!(1))])),
                ),
                acp_v2::ContentBlock::Text(
                    acp_v2::TextContent::new("world")
                        .meta(acp_v1::Meta::from_iter([("part".into(), json!(2))])),
                ),
                match message_test_image() {
                    acp_v2::ContentBlock::Image(image) => acp_v2::ContentBlock::Image(
                        image
                            .uri("file:///original.png".to_string())
                            .annotations(annotations)
                            .meta(acp_v1::Meta::from_iter([("image".into(), json!(42))])),
                    ),
                    _ => unreachable!("fixture is an image"),
                },
                acp_v2::ContentBlock::ResourceLink(
                    acp_v2::ResourceLink::new("original name", "https://example.com/file")
                        .description("original description".to_string())
                        .title("original title".to_string())
                        .meta(acp_v1::Meta::from_iter([(
                            "link".into(),
                            json!("original"),
                        )])),
                ),
            ];
            let mut content = MessageContent::default();
            for block in &blocks {
                content.append(block.clone(), &languages, PathStyle::local(), cx);
            }
            assert_eq!(content.source_blocks(), blocks);
            let render_blocks = content.blocks().collect::<Vec<_>>();
            let [first, image, last] = render_blocks.as_slice() else {
                panic!("text coalesces, link renders as a mention, image stays an image");
            };
            assert!(image.image().is_some());
            assert!(last.markdown().is_some());
            assert_eq!(
                first.markdown().expect("text").read(cx).source(),
                "hello world"
            );
            assert_eq!(
                content.to_markdown(cx),
                "hello world\n\n`Image`\n\n[@https://example.com/file](https://example.com/file)"
            );
        });
    }

    #[gpui::test]
    fn test_message_content_replaces_mixed_blocks_in_place(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let path_style = PathStyle::local();
            let image = message_test_image();
            let initial = vec!["before".into(), image.clone(), "after".into()];
            let mut content = MessageContent::default();
            for block in &initial {
                content.append(block.clone(), &languages, path_style, cx);
            }
            let original = content.blocks().collect::<Vec<_>>();
            let before = original[0].markdown().expect("first text").clone();
            let decoded_image = original[1].image().expect("image").0.clone();
            let after = original[2].markdown().expect("last text").clone();

            content.replace(initial.clone(), &languages, path_style, cx);
            assert_eq!(content.source_blocks(), initial);
            assert_eq!(
                content.blocks().nth(0).and_then(|block| block.markdown()),
                Some(&before)
            );
            assert!(Arc::ptr_eq(
                content
                    .blocks()
                    .nth(1)
                    .and_then(|block| block.image())
                    .expect("image")
                    .0,
                &decoded_image
            ));
            assert_eq!(
                content.blocks().nth(2).and_then(|block| block.markdown()),
                Some(&after)
            );

            let modified_image = match image {
                acp_v2::ContentBlock::Image(image) => acp_v2::ContentBlock::Image(
                    image
                        .uri("file:///renamed.png".to_string())
                        .meta(acp_v1::Meta::from_iter([("updated".into(), json!(true))])),
                ),
                _ => panic!("image fixture"),
            };
            let replacement = vec![
                acp_v2::ContentBlock::Text(
                    acp_v2::TextContent::new("new ")
                        .meta(acp_v1::Meta::from_iter([("part".into(), json!(1))])),
                ),
                "text".into(),
                modified_image,
                "changed".into(),
            ];
            content.replace(replacement.clone(), &languages, path_style, cx);
            assert_eq!(content.source_blocks(), replacement);
            assert_eq!(content.to_markdown(cx), "new text\n\n`Image`\n\nchanged");
            assert_eq!(before.read(cx).source(), "new text");
            assert_eq!(after.read(cx).source(), "changed");
            assert_eq!(
                content.blocks().nth(0).and_then(|block| block.markdown()),
                Some(&before)
            );
            assert_eq!(
                content.blocks().nth(2).and_then(|block| block.markdown()),
                Some(&after)
            );
            assert!(Arc::ptr_eq(
                content
                    .blocks()
                    .nth(1)
                    .and_then(|block| block.image())
                    .expect("image")
                    .0,
                &decoded_image
            ));

            content.replace(Vec::new(), &languages, path_style, cx);
            assert!(content.source_blocks().is_empty());
            assert_eq!(content.blocks().len(), 0);
            content.append("again".into(), &languages, path_style, cx);
            assert_eq!(content.to_markdown(cx), "again");
        });
    }

    #[gpui::test]
    fn test_message_content_snapshot_overrides_deferred_source(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let mut content =
                MessageContent::new("visible".into(), &languages, PathStyle::local(), cx);
            let markdown = content.markdowns().next().expect("markdown").clone();
            content.append_deferred_text(acp_v2::TextContent::new("hidden"));
            assert_eq!(markdown.read(cx).source(), "visible");
            content.replace(vec!["snapshot".into()], &languages, PathStyle::local(), cx);
            assert_eq!(content.source_blocks(), &["snapshot".into()]);
            assert_eq!(content.to_markdown(cx), "snapshot");
            assert_eq!(content.markdowns().next(), Some(&markdown));
        });
    }

    #[gpui::test]
    fn test_message_content_source_versions_track_source_not_rendering(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let path_style = PathStyle::local();
            let mut content = MessageContent::default();
            let other_content = MessageContent::default();
            assert_eq!(content, other_content);
            assert_ne!(content.source_version(), other_content.source_version());

            let empty_version = content.source_version();
            content.append("first".into(), &languages, path_style, cx);
            assert_ne!(content.source_version(), empty_version);
            let appended_version = content.source_version();
            content.append_deferred_text(acp_v2::TextContent::new(" hidden"));
            assert_ne!(content.source_version(), appended_version);
            let deferred_version = content.source_version();
            let snapshot = content.source_blocks().to_vec();
            content.replace(snapshot, &languages, path_style, cx);
            assert_eq!(content.to_markdown(cx), "first hidden");
            assert_eq!(content.source_version(), deferred_version);
            content.shrink_source_capacity();
            assert_eq!(content.source_version(), deferred_version);

            content.replace_prompt(Vec::new(), &languages, path_style, cx);
            assert_ne!(content.source_version(), deferred_version);
            let cleared_version = content.source_version();
            content.append_prompt("".into(), &languages, path_style, cx);
            assert_ne!(content.source_version(), cleared_version);
            assert_eq!(content.source_blocks(), &["".into()]);
        });
    }

    #[gpui::test]
    fn test_message_content_snapshot_keeps_image_fallback_until_payload_changes(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let image = acp_v2::ImageContent::new("invalid-base64", "image/png");
            let mut content = MessageContent::new(
                acp_v2::ContentBlock::Image(image.clone()),
                &languages,
                PathStyle::local(),
                cx,
            );
            let fallback = content.markdowns().next().expect("image fallback").clone();
            let updated = acp_v2::ContentBlock::Image(image.meta(acp_v2::Meta::from_iter([(
                "caption".into(),
                json!("updated"),
            )])));
            content.replace(vec![updated.clone()], &languages, PathStyle::local(), cx);
            assert_eq!(content.source_blocks(), &[updated]);
            assert_eq!(content.markdowns().next(), Some(&fallback));
            assert_eq!(
                content.to_markdown(cx),
                "Image content could not be displayed."
            );

            content.replace(
                vec![message_test_image()],
                &languages,
                PathStyle::local(),
                cx,
            );
            assert!(content.blocks().next().expect("image").image().is_some());
            assert_eq!(content.markdowns().count(), 0);
        });
    }

    #[gpui::test]
    fn test_message_content_snapshot_preserves_unknown_source(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let unknown = acp_v2::ContentBlock::Other(acp_v2::OtherContentBlock::new(
                "_future",
                std::collections::BTreeMap::from([
                    ("payload".to_string(), json!({"nested": [1, 2]})),
                    ("_meta".to_string(), json!({"source": "agent"})),
                ]),
            ));
            let blocks = vec![
                acp_v2::ContentBlock::Text(
                    acp_v2::TextContent::new("before")
                        .annotations(acp_v2::Annotations::new().priority(0.5)),
                ),
                unknown.clone(),
                "after".into(),
            ];
            let mut content = MessageContent::default();
            content.replace(blocks.clone(), &languages, PathStyle::local(), cx);
            assert_eq!(content.source_blocks(), blocks);
            assert_eq!(content.blocks().len(), 3);
            let fallback = content.blocks().nth(1).expect("unknown block");
            assert_eq!(fallback.unsupported_content(), Some(&unknown));
            assert!(std::ptr::eq(
                fallback.source.expect("source"),
                &content.source_blocks()[1]
            ));
            assert_eq!(
                content.to_markdown(cx),
                "before\n\nUnknown content type is not supported.\n\nafter"
            );
        });
    }

    #[gpui::test]
    fn test_message_content_snapshot_prompt_and_link_parity(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let path_style = PathStyle::local();
            let resource = acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::TextResourceContents(
                    acp_v2::TextResourceContents::new(
                        "private contents",
                        "https://example.com/file",
                    ),
                ),
            ));
            let link = acp_v2::ContentBlock::ResourceLink(acp_v2::ResourceLink::new(
                "link",
                "https://example.com/link",
            ));
            for prompt in [false, true] {
                let blocks = vec![
                    link.clone(),
                    "read ".into(),
                    resource.clone(),
                    " next".into(),
                ];
                let mut appended = MessageContent::default();
                for block in &blocks {
                    if prompt {
                        appended.append_prompt(block.clone(), &languages, path_style, cx);
                    } else {
                        appended.append(block.clone(), &languages, path_style, cx);
                    }
                }
                let mut replaced = MessageContent::default();
                if prompt {
                    replaced.replace_prompt(blocks.clone(), &languages, path_style, cx);
                } else {
                    replaced.replace(blocks.clone(), &languages, path_style, cx);
                }
                assert_eq!(replaced.source_blocks(), blocks);
                assert_eq!(replaced.to_markdown(cx), appended.to_markdown(cx));
                assert_eq!(replaced.blocks().len(), appended.blocks().len());
                if prompt {
                    assert_eq!(replaced.blocks().len(), 1);
                    assert!(!replaced.to_markdown(cx).contains("private contents"));
                } else {
                    assert_eq!(replaced.blocks().len(), 3);
                    assert!(replaced.to_markdown(cx).contains("private contents"));
                }
            }
        });
    }

    #[gpui::test]
    fn test_message_content_preserves_mixed_order(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            for (chunks, expected_kinds, expected_export) in [
                (
                    vec!["before".into(), message_test_image()],
                    vec!["text", "image"],
                    "before\n\n`Image`",
                ),
                (
                    vec![message_test_image(), "after".into()],
                    vec!["image", "text"],
                    "`Image`\n\nafter",
                ),
                (
                    vec![message_test_image(), message_test_image()],
                    vec!["image", "image"],
                    "`Image`\n\n`Image`",
                ),
                (
                    vec!["".into(), message_test_image()],
                    vec!["image"],
                    "`Image`",
                ),
            ] {
                let mut content = MessageContent::default();
                for chunk in chunks {
                    content.append(chunk, &language_registry, PathStyle::local(), cx);
                }
                assert_eq!(
                    content
                        .blocks()
                        .map(|block| match &block.render {
                            RenderBlock::Markdown { .. } => "text",
                            RenderBlock::Image { .. } => "image",
                            other => panic!("unexpected block {other:?}"),
                        })
                        .collect::<Vec<_>>(),
                    expected_kinds,
                );
                assert_eq!(content.to_markdown(cx), expected_export);
                assert!(content.visible_content(cx));
            }
        });
    }

    #[gpui::test]
    fn test_message_content_resources_and_fallbacks(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let mut content = MessageContent::default();
            for chunk in [
                "before".into(),
                acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                    acp_v2::EmbeddedResourceResource::TextResourceContents(
                        acp_v2::TextResourceContents::new("resource text", "tool://preview"),
                    ),
                )),
                acp_v2::ContentBlock::ResourceLink(acp_v2::ResourceLink::new(
                    "link",
                    "https://example.com/resource",
                )),
                acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                    acp_v2::EmbeddedResourceResource::BlobResourceContents(
                        acp_v2::BlobResourceContents::new(
                            "private-blob-data",
                            "tool://archive.bin",
                        ),
                    ),
                )),
                acp_v2::ContentBlock::Image(acp_v2::ImageContent::new("not-base64", "image/png")),
                acp_v2::ContentBlock::Audio(acp_v2::AudioContent::new(
                    "private-audio-data",
                    "audio/wav",
                )),
                "after".into(),
            ] {
                content.append(chunk, &language_registry, PathStyle::local(), cx);
            }
            assert!(matches!(
                content
                    .blocks()
                    .map(|block| block.render)
                    .collect::<Vec<_>>()
                    .as_slice(),
                [
                    RenderBlock::Markdown { .. },
                    RenderBlock::EmbeddedResource { .. },
                    RenderBlock::Markdown { .. },
                    RenderBlock::EmbeddedResource { markdown: None, .. },
                    RenderBlock::Unsupported { .. },
                    RenderBlock::Unsupported { .. },
                    RenderBlock::Markdown { .. },
                ]
            ));
            assert_eq!(
                content.to_markdown(cx),
                concat!(
                    "before\n\n```\nresource text\n```\n\n",
                    "[@https://example.com/resource](https://example.com/resource)\n\n",
                    "tool://archive.bin\n\n",
                    "Image content could not be displayed.\n\n",
                    "Audio content is not supported.\n\nafter",
                )
            );
            assert_eq!(
                content
                    .markdowns()
                    .map(|markdown| markdown.read(cx).source())
                    .collect::<Vec<_>>(),
                [
                    "before",
                    "```\nresource text\n```",
                    "[@https://example.com/resource](https://example.com/resource)",
                    "Image content could not be displayed.",
                    "Audio content is not supported.",
                    "after",
                ]
            );
            assert!(!content.to_markdown(cx).contains("private-audio-data"));
            assert!(!content.to_markdown(cx).contains("private-blob-data"));
            for (render_index, source_index) in [(1, 1), (3, 3)] {
                let source = &content.source_blocks[source_index];
                let rendered_block = content.blocks().nth(render_index).expect("rendered block");
                assert!(matches!(
                    rendered_block.render,
                    RenderBlock::EmbeddedResource { .. }
                ));
                assert!(std::ptr::eq(
                    source,
                    rendered_block.source.expect("source block")
                ));
            }
        });
    }

    #[gpui::test]
    fn test_v2_other_content_preserves_raw_source_and_renders_fallback(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let raw_fields = std::collections::BTreeMap::from([
                (
                    "payload".to_string(),
                    json!({"nested": [1, {"value": "original"}]}),
                ),
                ("_meta".to_string(), json!({"source": "future-agent"})),
            ]);
            let source = acp_v2::ContentBlock::Other(acp_v2::OtherContentBlock::new(
                "_custom_payload",
                raw_fields.clone(),
            ));

            let content =
                MessageContent::new(source.clone(), &language_registry, PathStyle::local(), cx);
            let rendered_blocks = content.blocks().collect::<Vec<_>>();
            let [rendered] = rendered_blocks.as_slice() else {
                panic!("expected one fallback block");
            };
            assert_eq!(content.source_blocks(), std::slice::from_ref(&source));
            assert!(std::ptr::eq(
                rendered.source.expect("fallback source"),
                &content.source_blocks()[0],
            ));
            assert_eq!(rendered.unsupported_content(), Some(&source));
            assert_eq!(
                rendered.to_markdown(cx),
                "Unknown content type is not supported."
            );

            let output = ContentBlock::new_output(source, &language_registry, cx);
            let Some(acp_v2::ContentBlock::Other(other)) = output.unsupported_content() else {
                panic!("expected original unknown source");
            };
            assert_eq!(other.type_, "_custom_payload");
            assert_eq!(other.fields, raw_fields);
            assert_eq!(
                output.to_markdown(cx),
                "Unknown content type is not supported."
            );
        });
    }

    #[gpui::test]
    async fn test_message_content_preserves_streaming_entities(cx: &mut TestAppContext) {
        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("session should be created");

        let source_blocks = [
            "before".into(),
            acp_v2::ContentBlock::Text(
                acp_v2::TextContent::new(" image")
                    .meta(acp_v1::Meta::from_iter([("sequence".into(), json!(2))])),
            ),
            message_test_image(),
            "after".into(),
            acp_v2::ContentBlock::Text(
                acp_v2::TextContent::new(" image")
                    .meta(acp_v1::Meta::from_iter([("sequence".into(), json!(5))])),
            ),
        ];
        let (before, after, image) = thread.update(cx, |thread, cx| {
            thread.push_assistant_content_block(source_blocks[0].clone(), false, cx);
            let before = thread
                .streaming_content_target(None, false, false)
                .map(|(_, target)| target.markdown)
                .expect("text should have a streaming target");
            thread.push_assistant_content_block(source_blocks[1].clone(), false, cx);
            assert!(thread.streaming_text_buffer.is_some());
            let (content, _) = thread
                .streaming_content_target(None, false, false)
                .expect("buffered text should retain its streaming target");
            assert_eq!(content.source_blocks(), &source_blocks[..2]);
            assert_eq!(before.read(cx).source(), "before");

            thread.push_assistant_content_block(source_blocks[2].clone(), false, cx);
            assert!(thread.streaming_text_buffer.is_none());
            assert_eq!(before.read(cx).source(), "before image");
            assert!(
                thread
                    .streaming_content_target(None, false, false)
                    .is_none()
            );

            thread.push_assistant_content_block(source_blocks[3].clone(), false, cx);
            let after = thread
                .streaming_content_target(None, false, false)
                .map(|(_, target)| target.markdown)
                .expect("text after an image should have its own streaming target");
            assert_ne!(before.entity_id(), after.entity_id());
            thread.push_assistant_content_block(source_blocks[4].clone(), false, cx);
            assert_eq!(after.read(cx).source(), "after");
            assert_eq!(
                thread
                    .streaming_text_buffer
                    .as_ref()
                    .expect("streaming text")
                    .target
                    .markdown,
                after,
            );

            let [AgentThreadEntry::AssistantMessage(message)] = thread.entries() else {
                panic!("expected one assistant message");
            };
            let [AssistantMessageChunk::Message { block, .. }] = message.chunks.as_slice() else {
                panic!("expected one message chunk");
            };
            assert_eq!(block.source_blocks(), source_blocks);
            let render_blocks = block.blocks().collect::<Vec<_>>();
            let [_, image_block, _] = render_blocks.as_slice() else {
                panic!("expected text, image, text");
            };
            let (image, _) = image_block.image().expect("expected image");
            (before, after, image.clone())
        });

        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();

        thread.read_with(cx, |thread, cx| {
            assert_eq!(after.read(cx).source(), "after image");
            let [AgentThreadEntry::AssistantMessage(message)] = thread.entries() else {
                panic!("expected one assistant message");
            };
            let [AssistantMessageChunk::Message { block, .. }] = message.chunks.as_slice() else {
                panic!("expected one message chunk");
            };
            assert_eq!(block.source_blocks(), source_blocks);
            let render_blocks = block.blocks().collect::<Vec<_>>();
            let [first, current_image, last] = render_blocks.as_slice() else {
                panic!("expected text, image, text");
            };
            assert_eq!(first.markdown(), Some(&before));
            assert_eq!(last.markdown(), Some(&after));
            assert!(Arc::ptr_eq(&image, current_image.image().expect("image").0));
        });

        let whole = cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let mut content = MessageContent::default();
            for chunk in [
                "before image".into(),
                message_test_image(),
                "after image".into(),
            ] {
                content.append(chunk, &language_registry, PathStyle::local(), cx);
            }
            content.to_markdown(cx)
        });
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                format!("## Assistant\n\n{whole}\n\n")
            );
        });

        for (content, preview) in [
            (
                acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                    acp_v2::EmbeddedResourceResource::TextResourceContents(
                        acp_v2::TextResourceContents::new("Resource preview", "tool://preview")
                            .mime_type("text/markdown".to_string()),
                    ),
                )),
                "Resource preview",
            ),
            (
                acp_v2::ContentBlock::Audio(acp_v2::AudioContent::new(
                    "private-audio-data",
                    "audio/wav",
                )),
                "Audio content is not supported.",
            ),
        ] {
            thread.update(cx, |thread, cx| {
                thread.push_assistant_content_block(content, false, cx);
                assert!(
                    thread
                        .streaming_content_target(None, false, false)
                        .is_none()
                );
                let Some(AgentThreadEntry::AssistantMessage(message)) = thread.entries().last()
                else {
                    panic!("expected assistant output");
                };
                let Some(AssistantMessageChunk::Message { block, .. }) = message.chunks.last()
                else {
                    panic!("expected message content");
                };
                let preview_markdown = block
                    .blocks()
                    .last()
                    .and_then(|block| block.markdown())
                    .expect("Markdown-backed non-text content")
                    .clone();
                thread.push_assistant_content_block("Following".into(), false, cx);
                let target = thread
                    .streaming_content_target(None, false, false)
                    .map(|(_, target)| target.markdown)
                    .expect("new text target");
                assert_ne!(target, preview_markdown);
                thread.push_assistant_content_block(" text".into(), false, cx);
                thread.flush_streaming_text(cx);
                assert_eq!(target.read(cx).source(), "Following text");
                assert_eq!(preview_markdown.read(cx).source(), preview);
            });
        }

        let (streamed, expected) = thread.update(cx, |thread, cx| {
            let path_style = thread.project.read(cx).path_style(cx);
            let languages = thread.project.read(cx).languages().clone();
            let uri = "file:///project/report.md";
            let link = acp_v2::ContentBlock::ResourceLink(acp_v2::ResourceLink::new("report", uri));
            let expected = format!(
                "{}\n## Details\n",
                ContentBlock::resource_link_md(uri, path_style),
            );
            let mut direct = MessageContent::new(link.clone(), &languages, path_style, cx);
            direct.append("## Details\n".into(), &languages, path_style, cx);
            assert_eq!(direct.to_markdown(cx), expected);

            thread.push_user_content_block(None, "Next response".into(), cx);
            thread.push_assistant_content_block(link, false, cx);
            let target = thread
                .streaming_content_target(None, false, false)
                .map(|(_, target)| target.markdown)
                .expect("leading link should retain its Markdown entity");
            thread.push_assistant_content_block("## Det".into(), false, cx);
            thread.push_assistant_content_block("ails\n".into(), false, cx);
            assert_eq!(
                thread
                    .streaming_text_buffer
                    .as_ref()
                    .expect("buffered text")
                    .target
                    .markdown,
                target,
            );
            (target, expected)
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        streamed.read_with(cx, |markdown, _| assert_eq!(markdown.source(), &expected));
    }

    #[gpui::test]
    async fn test_message_content_preserves_prompt_mentions_and_raw_chunks(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("session should be created");
        let chunks = vec![
            "read ".into(),
            acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::TextResourceContents(
                    acp_v2::TextResourceContents::new(
                        "attached file contents",
                        "https://example.com/file",
                    ),
                ),
            )),
            " and ".into(),
            acp_v2::ContentBlock::ResourceLink(acp_v2::ResourceLink::new(
                "link",
                "https://example.com/link",
            )),
            message_test_image(),
        ];

        thread.update(cx, |thread, cx| {
            for chunk in &chunks {
                thread.push_user_content_block(None, chunk.clone(), cx);
            }
            let [AgentThreadEntry::UserMessage(message)] = thread.entries() else {
                panic!("expected one user message");
            };
            assert_eq!(message.content.source_blocks(), chunks);
            assert!(matches!(
                message
                    .content
                    .blocks()
                    .map(|block| block.render)
                    .collect::<Vec<_>>()
                    .as_slice(),
                [RenderBlock::Markdown { .. }, RenderBlock::Image { .. }]
            ));
            let expected = format!(
                "read {} and {}\n\n`Image`",
                ContentBlock::resource_link_md("https://example.com/file", PathStyle::local()),
                ContentBlock::resource_link_md("https://example.com/link", PathStyle::local()),
            );
            assert_eq!(message.content.to_markdown(cx), expected);
            assert!(
                !message
                    .content
                    .to_markdown(cx)
                    .contains("attached file contents")
            );
        });
    }

    #[test]
    fn text_resource_markdown_uses_mime_type_for_code_blocks() {
        let shell =
            acp_v2::TextResourceContents::new("echo 'hello from exec test'", "tool://preview")
                .mime_type("text/x-shellscript".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&shell),
            "```sh\necho 'hello from exec test'\n```"
        );

        let markdown =
            acp_v2::TextResourceContents::new("**approval** requested", "tool://preview")
                .mime_type("text/markdown".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&markdown),
            "**approval** requested"
        );

        let plain = acp_v2::TextResourceContents::new("plain preview", "tool://preview")
            .mime_type("text/plain".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&plain),
            "```\nplain preview\n```"
        );

        let cpp = acp_v2::TextResourceContents::new("int main() {}", "tool://preview")
            .mime_type("text/x-c++; charset=utf-8".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&cpp),
            "```cpp\nint main() {}\n```"
        );

        let untyped = acp_v2::TextResourceContents::new("# plain preview", "tool://preview");
        assert_eq!(
            ContentBlock::text_resource_markdown(&untyped),
            "```\n# plain preview\n```"
        );
    }

    #[gpui::test]
    fn test_tool_content_retains_and_refreshes_inner_and_envelope_meta(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let make_content = |inner: &str, outer: &str| {
                acp_v2::ToolCallContent::Content(Box::new(
                    acp_v2::Content::new(acp_v2::ContentBlock::Text(
                        acp_v2::TextContent::new("same")
                            .meta(acp_v2::Meta::from_iter([("inner".into(), inner.into())])),
                    ))
                    .meta(acp_v2::Meta::from_iter([("outer".into(), outer.into())])),
                ))
            };
            let prepared = PreparedToolCallContent::prepare_v2(
                vec![make_content("first", "first")],
                ToolTerminalResolver::registered(&HashMap::default()),
                cx,
            )
            .expect("prepare content");
            let mut content = ToolCallContent::from_prepared(
                prepared.into_iter().next().expect("content"),
                &languages,
                cx,
            );
            let original = content.markdown().expect("markdown").clone();
            let prepared = PreparedToolCallContent::prepare_v2(
                vec![make_content("second", "second")],
                ToolTerminalResolver::registered(&HashMap::default()),
                cx,
            )
            .expect("prepare update");
            content.update_from_prepared(
                prepared.into_iter().next().expect("update"),
                &languages,
                cx,
            );
            let ToolCallContent::ContentBlock { block, meta } = &content else {
                panic!("content block");
            };
            assert_eq!(block.markdown(), Some(&original));
            assert_eq!(
                meta.as_ref().and_then(|meta| meta.get("outer")),
                Some(&serde_json::json!("second"))
            );
            let Some(acp_v2::ContentBlock::Text(source)) = block.source.as_ref() else {
                panic!("retained text");
            };
            assert_eq!(
                source.meta.as_ref().and_then(|meta| meta.get("inner")),
                Some(&serde_json::json!("second"))
            );
        });
    }

    #[gpui::test]
    fn test_tool_content_resource_updates_refresh_source_and_reuse_rendering(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let resource = |uri: &str, mime: &str| {
                acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                    acp_v2::EmbeddedResourceResource::TextResourceContents(
                        acp_v2::TextResourceContents::new("**text**", uri)
                            .mime_type(mime)
                            .meta(acp_v2::Meta::from_iter([("uri".into(), json!(uri))])),
                    ),
                ))
            };
            let mut content = ToolCallContent::from_prepared(
                PreparedToolCallContent::ContentBlock(acp_v2::Content::new(resource(
                    "tool://old",
                    "text/markdown",
                ))),
                &languages,
                cx,
            );
            let markdown = content.markdown().expect("preview").clone();
            for mime in ["text/markdown", "text/plain"] {
                let source = resource("tool://new", mime);
                content.update_from_prepared(
                    PreparedToolCallContent::ContentBlock(acp_v2::Content::new(source.clone())),
                    &languages,
                    cx,
                );
                assert_eq!(content.markdown(), Some(&markdown));
                let ToolCallContent::ContentBlock { block, .. } = &content else {
                    panic!("resource");
                };
                assert_eq!(block.source.as_ref(), Some(&source));
                assert_eq!(
                    markdown.read(cx).source().as_ref(),
                    if mime == "text/markdown" {
                        "**text**"
                    } else {
                        "```\n**text**\n```"
                    },
                );
            }

            let acp_v2::ContentBlock::Image(image) = message_test_image() else {
                panic!("image fixture");
            };
            let blob = |uri: &str| {
                acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                    acp_v2::EmbeddedResourceResource::BlobResourceContents(
                        acp_v2::BlobResourceContents::new(image.data.clone(), uri)
                            .mime_type(image.mime_type.clone()),
                    ),
                ))
            };
            content.update_from_prepared(
                PreparedToolCallContent::ContentBlock(acp_v2::Content::new(blob("tool://old.png"))),
                &languages,
                cx,
            );
            assert!(content.markdown().is_none());
            let decoded = content.image().expect("embedded image").0.clone();
            let source = blob("tool://renamed.png");
            content.update_from_prepared(
                PreparedToolCallContent::ContentBlock(acp_v2::Content::new(source.clone())),
                &languages,
                cx,
            );
            assert!(Arc::ptr_eq(content.image().expect("image").0, &decoded));
            let ToolCallContent::ContentBlock { block, .. } = &content else {
                panic!("resource image");
            };
            assert_eq!(block.source.as_ref(), Some(&source));
        });
    }

    #[gpui::test]
    fn test_tool_content_image_metadata_update_reuses_decoded_image(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let acp_v2::ContentBlock::Image(image) = message_test_image() else {
                panic!("image");
            };
            let mut content = ToolCallContent::from_prepared(
                PreparedToolCallContent::ContentBlock(acp_v2::Content::new(
                    acp_v2::ContentBlock::Image(image.clone()),
                )),
                &languages,
                cx,
            );
            let original = content.image().expect("image").0.clone();
            content.update_from_prepared(
                PreparedToolCallContent::ContentBlock(acp_v2::Content::new(
                    acp_v2::ContentBlock::Image(
                        image.meta(acp_v2::Meta::from_iter([("revision".into(), 2.into())])),
                    ),
                )),
                &languages,
                cx,
            );
            assert!(Arc::ptr_eq(&original, content.image().expect("image").0));
            let ToolCallContent::ContentBlock { block, .. } = &content else {
                panic!("image block");
            };
            let Some(acp_v2::ContentBlock::Image(source)) = block.source.as_ref() else {
                panic!("retained image");
            };
            assert_eq!(
                source.meta.as_ref().and_then(|meta| meta.get("revision")),
                Some(&serde_json::json!(2))
            );
        });
    }

    #[gpui::test]
    fn test_tool_content_legacy_diff_preserves_raw_source_and_native_authority(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let mut content = ToolCallContent::from_prepared(
                PreparedToolCallContent::LegacyDiff(acp_v1::Diff::new("first.rs", "new")),
                &languages,
                cx,
            );
            let ToolCallContent::LegacyDiff { diff, .. } = &content else {
                panic!("legacy diff")
            };
            let original = diff.clone();
            content.update_from_prepared(
                PreparedToolCallContent::LegacyDiff(
                    acp_v1::Diff::new("first.rs", "new")
                        .meta(acp_v1::Meta::from_iter([("revision".into(), 1.into())])),
                ),
                &languages,
                cx,
            );
            let ToolCallContent::LegacyDiff { source, diff } = &content else {
                panic!("legacy diff")
            };
            assert_eq!(diff, &original);
            assert_eq!(
                source.meta.as_ref().and_then(|meta| meta.get("revision")),
                Some(&serde_json::json!(1))
            );
            content.update_from_prepared(
                PreparedToolCallContent::LegacyDiff(
                    acp_v1::Diff::new("first.rs", "new").old_text(""),
                ),
                &languages,
                cx,
            );
            let ToolCallContent::LegacyDiff { source, diff } = &content else {
                panic!("legacy diff")
            };
            assert_eq!(source.old_text.as_deref(), Some(""));
            assert_ne!(diff, &original);
            let previous = diff.clone();
            content.update_from_prepared(
                PreparedToolCallContent::LegacyDiff(
                    acp_v1::Diff::new("second.rs", "new").old_text(""),
                ),
                &languages,
                cx,
            );
            let ToolCallContent::LegacyDiff { source, diff } = &content else {
                panic!("legacy diff")
            };
            assert_eq!(source.path, PathBuf::from("second.rs"));
            assert_ne!(diff, &previous);
            let native_diff = cx.new(|cx| {
                Diff::finalized(
                    "native.rs".into(),
                    Some("before".into()),
                    "after".into(),
                    languages.clone(),
                    cx,
                )
            });
            let mut native = ToolCallContent::Diff(native_diff.clone());
            native.update_from_prepared(
                PreparedToolCallContent::LegacyDiff(acp_v1::Diff::new("native.rs", "replacement")),
                &languages,
                cx,
            );
            assert!(matches!(native, ToolCallContent::LegacyDiff { .. }));
            assert_eq!(
                native_diff.read(cx).file_path(cx).as_deref(),
                Some("native.rs")
            );
            assert_ne!(
                native_diff,
                match &native {
                    ToolCallContent::LegacyDiff { diff, .. } => diff.clone(),
                    _ => panic!("legacy diff"),
                }
            );
        });
    }

    #[gpui::test]
    fn test_tool_diff_patch_reuses_hunks_for_metadata_changes(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let mut source: acp_v2::Diff = serde_json::from_value(serde_json::json!({
                "changes": [{"operation": "modify", "path": "/test.rs"}],
                "patch": {
                    "format": "git_patch",
                    "text": "--- /test.rs\n+++ /test.rs\n@@ -5 +5 @@\n-before\n+after\n"
                }
            }))
            .expect("diff");
            let mut content = ToolCallContent::from_prepared(
                PreparedToolCallContent::DiffPatch(source.clone()),
                &languages,
                cx,
            );
            let ToolCallContent::DiffPatch { render, .. } = &content else {
                panic!("patch");
            };
            assert!(render.fallback.is_none());
            let buffer = render.files[0].hunks[0].buffer.clone();
            source.meta = Some(acp_v2::Meta::from_iter([("revision".into(), 2.into())]));
            source.changes[0].meta = source.meta.clone();
            source.changes[0].mime_type = Some(acp_v2::MediaType::new("text/x-rust"));
            content.update_from_prepared(
                PreparedToolCallContent::DiffPatch(source.clone()),
                &languages,
                cx,
            );
            let ToolCallContent::DiffPatch {
                source: retained,
                render,
            } = &content
            else {
                panic!("patch");
            };
            assert_eq!(retained, &source);
            assert_eq!(render.files[0].hunks[0].buffer, buffer);
            assert!(
                content.markdown().is_none(),
                "no hidden Markdown copy of a rendered patch"
            );
            assert!(content.to_markdown(cx).contains("-before\n+after"));
            source.patch.as_mut().expect("patch").text = "@@ -5 +5 @@\n-before\n+updated\n".into();
            content.update_from_prepared(
                PreparedToolCallContent::DiffPatch(source.clone()),
                &languages,
                cx,
            );
            let ToolCallContent::DiffPatch {
                source: retained,
                render,
            } = &content
            else {
                panic!("patch");
            };
            assert_eq!(retained, &source);
            assert_ne!(render.files[0].hunks[0].buffer, buffer);
        });
    }

    #[gpui::test]
    fn test_tool_content_v2_diff_patch_and_unknown_retain_source(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let patch = "diff --git a/one b/one\n+``` nested";
            let diff = acp_v2::Diff::patch(patch, vec![])
                .meta(acp_v2::Meta::from_iter([("revision".into(), 1.into())]));
            let unknown: acp_v2::ToolCallContent =
                serde_json::from_value(serde_json::json!({"type":"future","payload":{"value":1}}))
                    .expect("unknown content");
            let prepared = PreparedToolCallContent::prepare_v2(
                vec![acp_v2::ToolCallContent::Diff(diff.clone()), unknown.clone()],
                ToolTerminalResolver::registered(&HashMap::default()),
                cx,
            )
            .expect("prepare v2 content");
            let mut content: Vec<_> = prepared
                .into_iter()
                .map(|content| ToolCallContent::from_prepared(content, &languages, cx))
                .collect();
            let ToolCallContent::DiffPatch { source, render } = &content[0] else {
                panic!("patch")
            };
            assert_eq!(source, &diff);
            let markdown = render
                .fallback
                .as_ref()
                .expect("unsupported patch fallback");
            assert!(markdown.read(cx).source().contains("````diff\n"));
            let original = markdown.clone();
            content[0].update_from_prepared(
                PreparedToolCallContent::DiffPatch(
                    acp_v2::Diff::patch(patch, vec![])
                        .meta(acp_v2::Meta::from_iter([("revision".into(), 2.into())])),
                ),
                &languages,
                cx,
            );
            let ToolCallContent::DiffPatch { source, render } = &content[0] else {
                panic!("patch")
            };
            assert_eq!(render.fallback.as_ref(), Some(&original));
            assert_eq!(
                source.meta.as_ref().and_then(|meta| meta.get("revision")),
                Some(&serde_json::json!(2))
            );
            let ToolCallContent::Other { source, markdown } = &content[1] else {
                panic!("unknown")
            };
            assert_eq!(source, &unknown);
            assert!(!markdown.read(cx).source().is_empty());
        });
    }

    #[gpui::test]
    async fn test_tool_call_content_preserves_embedded_text_resource(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let content = acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::TextResourceContents(
                    acp_v2::TextResourceContents::new(
                        "echo 'hello from exec test'",
                        "tool://preview",
                    )
                    .mime_type("text/x-shellscript".to_string()),
                ),
            ));

            let block = ContentBlock::new_output(content, &language_registry, cx);

            let Some((resource, markdown)) = block.as_view().embedded_resource() else {
                panic!("expected embedded resource block, got {block:?}");
            };
            match &resource.resource {
                acp_v2::EmbeddedResourceResource::TextResourceContents(text) => {
                    assert_eq!(text.text, "echo 'hello from exec test'");
                    assert_eq!(text.uri, "tool://preview");
                    assert_eq!(
                        text.mime_type.as_ref().map(AsRef::as_ref),
                        Some("text/x-shellscript")
                    );
                }
                other => panic!("expected text resource contents, got {other:?}"),
            }

            let markdown = markdown
                .as_ref()
                .expect("text resources should have renderable markdown")
                .read(cx)
                .source()
                .to_string();
            assert_eq!(markdown, "```sh\necho 'hello from exec test'\n```");
            assert_eq!(
                block.to_markdown(cx),
                "```sh\necho 'hello from exec test'\n```"
            );
            assert_eq!(block.text_content(cx), Some("echo 'hello from exec test'"));

            let untyped = ContentBlock::new_output(
                acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                    acp_v2::EmbeddedResourceResource::TextResourceContents(
                        acp_v2::TextResourceContents::new("# plain preview", "tool://preview"),
                    ),
                )),
                &language_registry,
                cx,
            );
            assert_eq!(untyped.to_markdown(cx), "```\n# plain preview\n```");
            assert_eq!(untyped.text_content(cx), Some("# plain preview"));
        });
    }

    #[gpui::test]
    async fn test_tool_call_content_renders_embedded_image_blob_resource(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let image_blob = acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::BlobResourceContents(
                    acp_v2::BlobResourceContents::new(
                        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
                        "tool://preview.png",
                    )
                    .mime_type("image/png".to_string()),
                ),
            ));

            let block = ContentBlock::new_output(
                image_blob,
                &language_registry,
                cx,
            );

            let Some((image, dimensions)) = block.image() else {
                panic!("expected image block, got {block:?}");
            };
            assert_eq!(image.format(), gpui::ImageFormat::Png);
            assert_eq!(
                dimensions.map(|size| (size.width, size.height)),
                Some((1, 1))
            );
            assert_eq!(block.to_markdown(cx), "`Image`");
            assert_eq!(block.text_content(cx), None);
        });
    }

    #[gpui::test]
    async fn test_tool_call_content_falls_back_for_non_image_blob_resource(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let archive_blob = acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::BlobResourceContents(
                    acp_v2::BlobResourceContents::new("not an image", "tool://archive.bin")
                        .mime_type("application/octet-stream".to_string()),
                ),
            ));

            let block = ContentBlock::new_output(archive_blob, &language_registry, cx);

            let Some((resource, markdown)) = block.as_view().embedded_resource() else {
                panic!("expected embedded resource block, got {block:?}");
            };
            assert!(markdown.is_none());
            match &resource.resource {
                acp_v2::EmbeddedResourceResource::BlobResourceContents(blob) => {
                    assert_eq!(blob.uri, "tool://archive.bin");
                    assert_eq!(
                        blob.mime_type.as_ref().map(AsRef::as_ref),
                        Some("application/octet-stream")
                    );
                }
                other => panic!("expected blob resource contents, got {other:?}"),
            }
            assert_eq!(block.to_markdown(cx), "tool://archive.bin");
            assert_eq!(block.text_content(cx), None);

            let invalid_image_blob = acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::BlobResourceContents(
                    acp_v2::BlobResourceContents::new("not-base64", "tool://preview.png")
                        .mime_type("image/png".to_string()),
                ),
            ));
            let invalid = ContentBlock::new_output(invalid_image_blob, &language_registry, cx);
            let Some((resource, markdown)) = invalid.as_view().embedded_resource() else {
                panic!("expected embedded resource block, got {invalid:?}");
            };
            assert!(markdown.is_none());
            assert_eq!(
                ContentBlock::embedded_resource_label(resource),
                "tool://preview.png"
            );
            assert_eq!(invalid.to_markdown(cx), "tool://preview.png");
        });
    }

    #[test]
    fn sandbox_authorization_details_deserialize_legacy_network_bool() {
        // Older builds persisted `network: bool`; the `alias` on
        // `network_all_hosts` must keep those details rendering as a
        // network request rather than silently dropping it.
        let details: SandboxAuthorizationDetails =
            serde_json::from_value(json!({ "network": true })).unwrap();
        assert!(details.network_all_hosts);
        assert!(details.network_hosts.is_empty());

        let details: SandboxAuthorizationDetails =
            serde_json::from_value(json!({ "network": false })).unwrap();
        assert!(!details.network_all_hosts);
    }

    #[gpui::test]
    async fn test_terminal_output_buffered_before_created_renders(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(
                    project,
                    PathList::new(&[std::path::Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .unwrap();

        let terminal_id = acp_v1::TerminalId::new(uuid::Uuid::new_v4().to_string());

        // Send Output BEFORE Created - should be buffered by acp_thread
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: b"hello buffered".to_vec(),
                },
                cx,
            );
        });

        // Create a display-only terminal and then send Created
        let lower = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            builder.subscribe(cx)
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "Buffered Test".to_string(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower.clone(),
                },
                cx,
            );
        });

        // After Created, buffered Output should have been flushed into the renderer
        let content = thread.read_with(cx, |thread, cx| {
            let term = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .unwrap();
            term.read_with(cx, |t, cx| t.inner().read(cx).get_content())
        });

        assert!(
            content.contains("hello buffered"),
            "expected buffered output to render, got: {content}"
        );
    }

    #[gpui::test]
    async fn test_terminal_exit_preserves_visible_scrollback(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(
                    project,
                    PathList::new(&[std::path::Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .unwrap();

        let terminal_id = acp_v1::TerminalId::new(uuid::Uuid::new_v4().to_string());
        let lower = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            builder.subscribe(cx)
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "Buffered Test".to_string(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower.clone(),
                },
                cx,
            );
        });
        cx.run_until_parked();
        thread.update(cx, |thread, cx| {
            let terminal = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .expect("display terminal");
            terminal.update(cx, |terminal, cx| {
                assert!(!terminal.is_process_backed());
                assert!(terminal.wait_for_exit().is_err());
                terminal.stop_by_user(cx);
                assert!(!terminal.was_stopped_by_user());
                assert!(
                    terminal.output().is_none(),
                    "a display terminal must wait for the provider's exit event",
                );
            });
        });

        let mut output = String::new();
        for line in 0..15_000 {
            output.push_str(&format!("line {line}\n"));
        }

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: output.into_bytes(),
                },
                cx,
            );
        });
        cx.run_until_parked();
        thread.read_with(cx, |thread, cx| {
            let terminal = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .expect("display terminal");
            let output = terminal.read(cx).current_output(cx);
            assert!(output.output.contains("line 14999"));
            assert!(output.exit_status.is_none());
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id.clone(),
                    status: acp_v1::TerminalExitStatus::new().exit_code(7),
                },
                cx,
            );
        });
        cx.run_until_parked();

        let content = thread.read_with(cx, |thread, cx| {
            let term = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .unwrap();
            let output = term.read(cx).current_output(cx);
            assert_eq!(
                output.exit_status.and_then(|status| status.exit_code),
                Some(7)
            );
            assert!(output.output.contains("line 14999"));
            term.read_with(cx, |term, cx| term.inner().read(cx).get_content())
        });

        assert!(
            content.contains("line 14999"),
            "expected output to remain visible after terminal exit, got: {content}"
        );

        let terminal = thread.read_with(cx, |thread, _| {
            thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .expect("display terminal")
        });
        let ended_at = terminal.read_with(cx, |terminal, _| {
            terminal.output().expect("completed output").ended_at
        });
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "Duplicate".to_string(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower.clone(),
                },
                cx,
            );
            assert_eq!(
                thread
                    .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                    .expect("terminal")
                    .entity_id(),
                terminal.entity_id(),
            );
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id.clone(),
                    status: acp_v1::TerminalExitStatus::new().signal("SIGTERM"),
                },
                cx,
            );
        });
        terminal.read_with(cx, |terminal, _| {
            let output = terminal.output().expect("completed output");
            assert_eq!(output.ended_at, ended_at);
            assert_eq!(output.exit_status.exit_code, Some(7));
            assert_eq!(output.exit_status.signal, None);
            assert_eq!(output.content, content);
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id,
                    data: b"late output\n".to_vec(),
                },
                cx,
            );
        });
        terminal.read_with(cx, |terminal, cx| {
            let output = terminal.output().expect("completed output");
            assert_eq!(output.ended_at, ended_at);
            assert_eq!(output.exit_status.exit_code, Some(7));
            assert_eq!(output.exit_status.signal, None);
            assert!(output.content.contains("late output"));
            assert_eq!(output.content, lower.read(cx).get_content());
            assert_eq!(output.original_content_len, output.content.len());
            assert_eq!(output.content_line_count, lower.read(cx).total_lines());
        });
    }

    #[gpui::test]
    async fn test_v2_terminal_reference_and_output_share_display_identity(cx: &mut TestAppContext) {
        init_test(cx);
        for reference_first in [true, false] {
            let thread = new_test_thread(cx).await;
            let terminal_id = acp_v2::TerminalId::new("shared-terminal");
            let lookup_id = terminal_id.clone();
            let tool_id = acp_v2::ToolCallId::new("terminal-tool");
            let reference = acp_v2::ToolCallUpdate::new("terminal-tool")
                .title("Tool caption")
                .kind(acp_v2::ToolKind::Execute)
                .content(vec![acp_v2::ToolCallContent::Terminal(
                    acp_v2::Terminal::new(terminal_id.clone()),
                )]);
            if reference_first {
                thread
                    .update(cx, |thread, cx| {
                        thread.upsert_wire_tool_call(reference.clone(), cx)
                    })
                    .expect("reference creates a placeholder");
            } else {
                thread
                    .update(cx, |thread, cx| {
                        thread.append_display_terminal_output(terminal_id.clone(), b"early\n", cx)
                    })
                    .expect("output creates a display terminal");
            }
            let terminal = thread.read_with(cx, |thread, _| {
                thread
                    .terminal(lookup_id.clone())
                    .expect("display terminal")
            });
            let lower = terminal.read_with(cx, |terminal, _| terminal.inner().clone());
            if reference_first {
                thread
                    .update(cx, |thread, cx| {
                        thread.append_display_terminal_output(terminal_id.clone(), b"early\n", cx)
                    })
                    .expect("append to placeholder");
            } else {
                thread
                    .update(cx, |thread, cx| thread.upsert_wire_tool_call(reference, cx))
                    .expect("reference existing display terminal");
            }
            thread
                .update(cx, |thread, cx| {
                    thread.upsert_display_terminal(
                        terminal_id.clone(),
                        DisplayTerminalPatch {
                            command: MaybeUndefined::Value("actual command".into()),
                            ..Default::default()
                        },
                        cx,
                    )
                })
                .expect("update display terminal");
            thread
                .update(cx, |thread, cx| {
                    thread.upsert_wire_tool_call(
                        acp_v2::ToolCallUpdate::new("terminal-tool").title("New caption"),
                        cx,
                    )
                })
                .expect("tool title is not the terminal command");
            thread.read_with(cx, |thread, cx| {
                let reused = thread.terminal(lookup_id).expect("same terminal");
                assert_eq!(reused.entity_id(), terminal.entity_id());
                assert_eq!(reused.read(cx).inner().entity_id(), lower.entity_id());
                assert_eq!(
                    reused.read(cx).command().read(cx).source(),
                    "```\nactual command\n```"
                );
                assert!(lower.read(cx).get_content().contains("early"));
                let (_, tool) = thread.tool_call(&tool_id).expect("tool");
                assert_eq!(tool.label.read(cx).source(), "New caption");
                assert_eq!(tool.terminals().next(), Some(&terminal));
            });
            thread
                .update(cx, |thread, cx| {
                    thread.update_tool_call(
                        acp_v1::ToolCallUpdate::new(
                            acp_v1::ToolCallId::new(tool_id.0.clone()),
                            acp_v1::ToolCallUpdateFields::new().title("Legacy caption"),
                        ),
                        cx,
                    )
                })
                .expect("legacy caption update");
            terminal.read_with(cx, |terminal, cx| {
                assert_eq!(
                    terminal.command().read(cx).source(),
                    "```\nactual command\n```"
                );
            });
            thread
                .update(cx, |thread, cx| {
                    thread.upsert_display_terminal(
                        terminal_id,
                        DisplayTerminalPatch {
                            command: MaybeUndefined::Null,
                            ..Default::default()
                        },
                        cx,
                    )?;
                    thread.update_tool_call(
                        acp_v1::ToolCallUpdate::new(
                            acp_v1::ToolCallId::new(tool_id.0),
                            acp_v1::ToolCallUpdateFields::new().title("Another legacy caption"),
                        ),
                        cx,
                    )
                })
                .expect("explicit command clear remains authoritative");
            terminal.read_with(cx, |terminal, cx| {
                assert_eq!(terminal.command().read(cx).source(), "```\nTerminal\n```");
            });
            thread
                .update(cx, |thread, cx| {
                    for (id, content) in [
                        (
                            "another-tool",
                            vec![acp_v2::ToolCallContent::Terminal(acp_v2::Terminal::new(
                                "shared-terminal",
                            ))],
                        ),
                        ("unrelated-tool", Vec::new()),
                    ] {
                        thread.upsert_wire_tool_call(
                            acp_v2::ToolCallUpdate::new(id).content(content),
                            cx,
                        )?;
                    }
                    anyhow::Ok(())
                })
                .expect("shared and unrelated tool rows");
            cx.run_until_parked();
            let updated_entries = Rc::new(RefCell::new(Vec::new()));
            let _subscription = cx.update(|cx| {
                cx.subscribe(&thread, {
                    let updated_entries = updated_entries.clone();
                    move |_, event, _| {
                        if let AcpThreadEvent::EntryUpdated(index) = event {
                            updated_entries.borrow_mut().push(*index);
                        }
                    }
                })
            });
            thread
                .update(cx, |thread, cx| {
                    thread.upsert_display_terminal(
                        "shared-terminal".into(),
                        DisplayTerminalPatch {
                            meta: MaybeUndefined::Value(acp_v2::Meta::new()),
                            ..Default::default()
                        },
                        cx,
                    )
                })
                .expect("terminal metadata refresh");
            cx.run_until_parked();
            assert_eq!(&*updated_entries.borrow(), &[0, 1]);
        }
    }

    #[gpui::test]
    async fn test_terminal_placeholder_consumes_legacy_events_in_order(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let terminal_id = acp_v1::TerminalId::new("mixed");
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: b"old\noutput".to_vec(),
                },
                cx,
            );
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id.clone(),
                    status: acp_v1::TerminalExitStatus::new().exit_code(7),
                },
                cx,
            );
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("tool").content(vec![
                        acp_v2::ToolCallContent::Terminal(acp_v2::Terminal::new("mixed")),
                    ]),
                    cx,
                )
                .expect("reference consumes queued legacy output");
            let terminal = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .expect("placeholder");
            let renderer = terminal.read(cx).inner().clone();
            assert!(renderer.read(cx).get_content().contains("old\noutput"));
            let ended_at = terminal.read(cx).output().expect("reported exit").ended_at;
            let meta = acp_v2::Meta::from_iter([("snapshot".into(), true.into())]);
            thread
                .upsert_display_terminal(
                    "mixed".into(),
                    DisplayTerminalPatch {
                        command: MaybeUndefined::Value("reported command".into()),
                        output: MaybeUndefined::Value(DisplayTerminalOutput {
                            data: b"new output".to_vec(),
                            meta: Some(meta.clone()),
                        }),
                        ..Default::default()
                    },
                    cx,
                )
                .expect("snapshot supersedes older bytes");
            let incoming_renderer = cx.new(|cx| {
                ::terminal::TerminalBuilder::new_display_only(
                    Default::default(),
                    ::terminal::terminal_settings::AlternateScroll::On,
                    None,
                    0,
                    cx.background_executor(),
                    PathStyle::local(),
                )
                .subscribe(cx)
            });
            for (label, cwd, output_limit) in [
                ("legacy command", path!("/work"), 3),
                ("ignored duplicate", path!("/ignored"), 1),
            ] {
                thread.on_terminal_provider_event(
                    TerminalProviderEvent::Created {
                        terminal_id: terminal_id.clone(),
                        label: label.into(),
                        cwd: Some(PathBuf::from(cwd)),
                        output_byte_limit: Some(output_limit),
                        terminal: incoming_renderer.clone(),
                    },
                    cx,
                );
            }
            let reused = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0))
                .expect("hydrated terminal");
            assert_eq!(reused, terminal);
            let terminal = reused.read(cx);
            assert_eq!(terminal.inner(), &renderer);
            assert_ne!(terminal.inner(), &incoming_renderer);
            assert_eq!(
                terminal.display_state().expect("display").command(),
                Some("reported command")
            );
            assert_eq!(
                terminal
                    .display_state()
                    .expect("display")
                    .output_meta
                    .as_ref(),
                Some(&meta)
            );
            assert_eq!(
                terminal.working_dir().as_deref(),
                Some(Path::new(path!("/work")))
            );
            let output = terminal.output().expect("completion preserved");
            assert_eq!(output.ended_at, ended_at);
            assert_eq!(output.exit_status.exit_code, Some(7));
            assert_eq!(output.content, "new");
            assert!(!renderer.read(cx).get_content().contains("old"));
            assert!(thread.pending_terminal_output.is_empty());
            assert!(thread.pending_terminal_exit.is_empty());
            thread
                .upsert_display_terminal(
                    "mixed".into(),
                    DisplayTerminalPatch {
                        exit_status: MaybeUndefined::Null,
                        ..Default::default()
                    },
                    cx,
                )
                .expect("clear completion");
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: acp_v1::TerminalId::new("mixed"),
                    status: acp_v1::TerminalExitStatus::new().exit_code(0),
                },
                cx,
            );
            assert!(
                reused.read(cx).output().is_none(),
                "duplicate legacy exit must not restore completion"
            );
        });
    }

    #[gpui::test]
    async fn test_v2_terminal_patch_tristate_metadata_and_output_ordering(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let terminal_id = acp_v2::TerminalId::new("snapshot-terminal");
        let lookup_id = terminal_id.clone();
        let terminal_meta = acp_v2::Meta::from_iter([("terminal".into(), 1.into())]);
        let output_meta = acp_v2::Meta::from_iter([("output".into(), 2.into())]);
        let status_meta = acp_v2::Meta::from_iter([("exit".into(), 3.into())]);
        thread
            .update(cx, |thread, cx| {
                thread.upsert_display_terminal(
                    terminal_id.clone(),
                    DisplayTerminalPatch {
                        command: MaybeUndefined::Value("first command".into()),
                        cwd: MaybeUndefined::Value(acp_v2::AbsolutePath::new(PathBuf::from(
                            path!("/work"),
                        ))),
                        meta: MaybeUndefined::Value(terminal_meta.clone()),
                        output: MaybeUndefined::Value(DisplayTerminalOutput {
                            data: b"old line\r\n".to_vec(),
                            meta: Some(output_meta.clone()),
                        }),
                        exit_status: MaybeUndefined::Value(
                            acp_v2::TerminalExitStatus::new()
                                .exit_code(7)
                                .meta(status_meta.clone()),
                        ),
                    },
                    cx,
                )
            })
            .expect("initial snapshot");
        let terminal = thread.read_with(cx, |thread, _| {
            thread.terminal(lookup_id.clone()).expect("terminal")
        });
        let ended_at = terminal.read_with(cx, |terminal, _| {
            let state = terminal.display_state().expect("display");
            assert_eq!(state.meta.as_ref(), Some(&terminal_meta));
            assert_eq!(state.output_meta.as_ref(), Some(&output_meta));
            let output = terminal.output().expect("completed");
            assert_eq!(output.exit_status.exit_code, Some(7));
            assert_eq!(output.exit_status.meta.as_ref(), Some(&status_meta));
            output.ended_at
        });
        thread
            .update(cx, |thread, cx| {
                thread.upsert_display_terminal(
                    terminal_id.clone(),
                    DisplayTerminalPatch {
                        output: MaybeUndefined::Value(DisplayTerminalOutput {
                            data: b"new line\r\n".to_vec(),
                            meta: None,
                        }),
                        ..Default::default()
                    },
                    cx,
                )?;
                thread.append_display_terminal_output(terminal_id.clone(), b"appended\r\n", cx)?;
                thread.upsert_display_terminal(
                    terminal_id.clone(),
                    DisplayTerminalPatch {
                        exit_status: MaybeUndefined::Value(acp_v2::TerminalExitStatus::new()),
                        ..Default::default()
                    },
                    cx,
                )
            })
            .expect("replacement, append, and exit correction");
        terminal.read_with(cx, |terminal, cx| {
            let state = terminal.display_state().expect("display state");
            assert_eq!(state.command(), Some("first command"));
            assert_eq!(
                terminal.working_dir().as_deref(),
                Some(Path::new(path!("/work")))
            );
            assert_eq!(state.meta.as_ref(), Some(&terminal_meta));
            assert_eq!(state.output_meta, None);
            let output = terminal.output().expect("completed");
            assert_eq!(output.ended_at, ended_at);
            assert_eq!(output.exit_status.exit_code, None);
            assert_eq!(output.exit_status.signal, None);
            assert_eq!(output.exit_status.meta, None);
            assert!(output.content.contains("new line\nappended"));
            assert!(!output.content.contains("old line"));
            assert_eq!(output.content, terminal.inner().read(cx).get_content());
        });
        thread
            .update(cx, |thread, cx| {
                thread.on_terminal_provider_event(
                    TerminalProviderEvent::Exit {
                        terminal_id: acp_v1::TerminalId::new(lookup_id.0.clone()),
                        status: acp_v1::TerminalExitStatus::new().exit_code(5),
                    },
                    cx,
                );
                thread.upsert_display_terminal(
                    terminal_id.clone(),
                    DisplayTerminalPatch {
                        output: MaybeUndefined::Null,
                        exit_status: MaybeUndefined::Null,
                        ..Default::default()
                    },
                    cx,
                )?;
                thread.on_terminal_provider_event(
                    TerminalProviderEvent::Exit {
                        terminal_id: acp_v1::TerminalId::new(lookup_id.0.clone()),
                        status: acp_v1::TerminalExitStatus::new().exit_code(6),
                    },
                    cx,
                );
                anyhow::Ok(())
            })
            .expect("clear snapshot and completion");
        terminal.read_with(cx, |terminal, cx| {
            assert!(terminal.output().is_none());
            assert!(terminal.inner().read(cx).get_content().trim().is_empty());
            assert!(
                terminal
                    .display_state()
                    .expect("display")
                    .output_meta
                    .is_none()
            );
            assert_eq!(
                terminal.display_state().expect("display").meta.as_ref(),
                Some(&terminal_meta)
            );
        });
        thread
            .update(cx, |thread, cx| {
                thread.append_display_terminal_output(terminal_id.clone(), b"fresh\n", cx)?;
                thread.upsert_display_terminal(
                    terminal_id.clone(),
                    DisplayTerminalPatch::default(),
                    cx,
                )
            })
            .expect("append and undefined patch");
        terminal.read_with(cx, |terminal, cx| {
            assert!(terminal.output().is_none());
            assert!(terminal.inner().read(cx).get_content().contains("fresh"));
            assert_eq!(
                terminal.command().read(cx).source(),
                "```\nfirst command\n```"
            );
        });
        thread
            .update(cx, |thread, cx| {
                thread.upsert_display_terminal(
                    terminal_id,
                    DisplayTerminalPatch {
                        command: MaybeUndefined::Null,
                        cwd: MaybeUndefined::Null,
                        meta: MaybeUndefined::Null,
                        ..Default::default()
                    },
                    cx,
                )
            })
            .expect("clear metadata without changing output");
        terminal.read_with(cx, |terminal, cx| {
            let state = terminal.display_state().expect("display");
            assert!(state.command().is_none());
            assert!(state.meta.is_none());
            assert!(terminal.working_dir().is_none());
            assert!(terminal.inner().read(cx).get_content().contains("fresh"));
        });
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.terminal(lookup_id).expect("terminal").entity_id(),
                terminal.entity_id()
            );
        });
    }

    #[gpui::test]
    async fn test_v2_terminal_updates_reject_client_owned_headless_terminal(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let terminal_id = acp_v2::TerminalId::new("native-terminal");
        let lower = cx.new(|cx| {
            ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .subscribe(cx)
        });
        let terminal = thread.update(cx, |thread, cx| {
            thread.register_terminal_created(
                terminal_id.clone(),
                "native command".into(),
                None,
                None,
                lower.clone(),
                cx,
            )
        });
        assert!(terminal.read_with(cx, |terminal, _| terminal.is_process_backed()));
        assert!(!lower.read_with(cx, |lower, _| lower.is_pty()));
        let before = lower.read_with(cx, |lower, _| lower.get_content());
        thread.update(cx, |thread, cx| {
            assert!(
                thread
                    .upsert_display_terminal(
                        acp_v2::TerminalId::new("native-terminal"),
                        DisplayTerminalPatch {
                            command: MaybeUndefined::Value("wrong command".into()),
                            output: MaybeUndefined::Null,
                            exit_status: MaybeUndefined::Value(acp_v2::TerminalExitStatus::new()),
                            ..Default::default()
                        },
                        cx,
                    )
                    .is_err()
            );
            assert!(
                thread
                    .append_display_terminal_output(
                        acp_v2::TerminalId::new("native-terminal"),
                        b"wrong output\n",
                        cx
                    )
                    .is_err()
            );
            assert_eq!(
                thread
                    .terminal(terminal_id)
                    .expect("native terminal")
                    .entity_id(),
                terminal.entity_id()
            );
        });
        terminal.read_with(cx, |terminal, cx| {
            assert!(terminal.display_state().is_none());
            assert_eq!(
                terminal.command().read(cx).source(),
                "```\nnative command\n```"
            );
            assert!(terminal.output().is_none());
            assert_eq!(lower.read(cx).get_content(), before);
        });
    }

    #[gpui::test]
    async fn test_terminal_output_and_exit_buffered_before_created(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(
                    project,
                    PathList::new(&[std::path::Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .unwrap();

        let terminal_id = acp_v1::TerminalId::new(uuid::Uuid::new_v4().to_string());

        // Send Output BEFORE Created
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: b"pre-exit data".to_vec(),
                },
                cx,
            );
        });

        // Send Exit BEFORE Created
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id.clone(),
                    status: acp_v1::TerminalExitStatus::new().signal("SIGTERM"),
                },
                cx,
            );
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id.clone(),
                    status: acp_v1::TerminalExitStatus::new().exit_code(0),
                },
                cx,
            );
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: b"\nlate output".to_vec(),
                },
                cx,
            );
        });

        // Now create a display-only lower-level terminal and send Created
        let lower = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            builder.subscribe(cx)
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "Buffered Exit Test".to_string(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower.clone(),
                },
                cx,
            );
        });

        // Output should be present after Created (flushed from buffer)
        let content = thread.read_with(cx, |thread, cx| {
            let term = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .unwrap();
            let output = term.read(cx).current_output(cx);
            let exit_status = output.exit_status.expect("buffered exit status");
            assert_eq!(exit_status.signal.as_deref(), Some("SIGTERM"));
            assert_eq!(exit_status.exit_code, None);
            assert!(output.output.contains("pre-exit data"));
            assert!(output.output.contains("late output"));
            term.read_with(cx, |t, cx| t.inner().read(cx).get_content())
        });

        assert!(
            content.contains("pre-exit data"),
            "expected pre-exit data to render, got: {content}"
        );
    }

    /// Test that killing a terminal via Terminal::kill properly:
    /// 1. Causes wait_for_exit to complete (doesn't hang forever)
    /// 2. The underlying terminal still has the output that was written before the kill
    ///
    /// This test verifies that the fix to kill_active_task (which now also kills
    /// the shell process in addition to the foreground process) properly allows
    /// wait_for_exit to complete instead of hanging indefinitely.
    #[cfg(unix)]
    #[gpui::test]
    async fn test_terminal_kill_allows_wait_for_exit_to_complete(cx: &mut gpui::TestAppContext) {
        assert_process_terminal_can_stop(false, cx).await;
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn test_headless_terminal_retains_process_control(cx: &mut gpui::TestAppContext) {
        assert_process_terminal_can_stop(true, cx).await;
    }

    #[cfg(unix)]
    async fn assert_process_terminal_can_stop(headless: bool, cx: &mut gpui::TestAppContext) {
        use std::collections::HashMap;
        use task::Shell;
        use util::shell_builder::ShellBuilder;

        init_test(cx);
        cx.executor().allow_parking();
        cx.update(|cx| cx.set_global(::terminal::HeadlessTerminal(headless)));

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(
                    project.clone(),
                    PathList::new(&[Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .unwrap();

        let terminal_id = acp_v2::TerminalId::new(uuid::Uuid::new_v4().to_string());

        // We use printf instead of echo and chain with && sleep to ensure proper execution
        let (program, args) = ShellBuilder::new(&Shell::System, false).build(
            Some("printf 'output_before_kill\\n' && sleep 60".to_owned()),
            &[],
        );
        let terminal_mode = ::terminal::TerminalMode::task(task::SpawnInTerminal {
            command: Some(program.clone()),
            args: args.clone(),
            ..Default::default()
        });

        let builder = cx
            .update(|cx| {
                ::terminal::TerminalBuilder::new(
                    None,
                    terminal_mode,
                    task::Shell::WithArguments {
                        program,
                        args,
                        title_override: None,
                    },
                    HashMap::default(),
                    ::terminal::terminal_settings::CursorShape::default(),
                    ::terminal::terminal_settings::AlternateScroll::On,
                    None,
                    vec![],
                    Duration::ZERO,
                    false,
                    0,
                    cx,
                    vec![],
                    PathStyle::local(),
                )
            })
            .await
            .unwrap();

        let lower_terminal = cx.new(|cx| builder.subscribe(cx));
        assert_eq!(
            lower_terminal.read_with(cx, |terminal, _| terminal.is_pty()),
            !headless
        );

        // Create the acp_thread Terminal wrapper
        thread.update(cx, |thread, cx| {
            let terminal = thread.register_terminal_created(
                terminal_id.clone(),
                "printf output_before_kill && sleep 60".to_string(),
                None,
                None,
                lower_terminal.clone(),
                cx,
            );
            assert!(terminal.read(cx).is_process_backed());
            let tool_id = acp_v2::ToolCallId::new("native-process-tool");
            thread
                .upsert_local_tool_call(
                    acp_v2::ToolCallUpdate::new(tool_id.clone())
                        .title("Native process")
                        .kind(acp_v2::ToolKind::Execute)
                        .content(vec![acp_v2::ToolCallContent::Terminal(
                            acp_v2::Terminal::new(terminal_id.clone()),
                        )]),
                    cx,
                )
                .expect("native reference uses the registered process terminal");
            let (_, call) = thread.tool_call(&tool_id).expect("native tool");
            assert_eq!(call.terminals().next(), Some(&terminal));
            assert_eq!(thread.terminals.len(), 1);
            thread
                .update_tool_call(
                    acp_v2::ToolCallUpdate::new(tool_id).title("Updated native command"),
                    cx,
                )
                .expect("v2 title update changes the client-managed command label");
            assert_eq!(
                terminal.read(cx).command().read(cx).source().as_ref(),
                "```\nUpdated native command\n```"
            );
        });

        // Poll until the printf command produces output, rather than using a
        // fixed sleep which is flaky on loaded machines.
        if !headless {
            let deadline = std::time::Instant::now() + Duration::from_secs(10);
            loop {
                let has_output = thread.read_with(cx, |thread, cx| {
                    let term = thread
                        .terminals
                        .get(&terminal_id)
                        .expect("terminal not found");
                    let content = term.read(cx).inner().read(cx).get_content();
                    content.contains("output_before_kill")
                });
                if has_output {
                    break;
                }
                assert!(
                    std::time::Instant::now() < deadline,
                    "Timed out waiting for printf output to appear in terminal",
                );
                cx.executor().timer(Duration::from_millis(50)).await;
            }
        }

        // Get the acp_thread Terminal and kill it
        let killed_at = std::time::Instant::now();
        let wait_for_exit = thread.update(cx, |thread, cx| {
            let term = thread.terminals.get(&terminal_id).unwrap();
            let wait_for_exit = term.read(cx).wait_for_exit().expect("process terminal");
            term.update(cx, |term, cx| {
                assert!(term.output().is_none(), "the process must still be running");
                if headless {
                    term.stop_by_user(cx);
                    assert!(term.was_stopped_by_user());
                } else {
                    term.kill(cx);
                }
            });
            wait_for_exit
        });

        // KEY ASSERTION: wait_for_exit should complete within a reasonable time (not hang).
        // Before the fix to kill_active_task, this would hang forever because
        // only the foreground process was killed, not the shell, so the PTY
        // child never exited and wait_for_completed_task never completed.
        let exit_result = futures::select! {
            result = futures::FutureExt::fuse(wait_for_exit) => Some(result),
            _ = futures::FutureExt::fuse(cx.background_executor.timer(Duration::from_secs(5))) => None,
        };

        assert!(
            exit_result.is_some(),
            "wait_for_exit should complete after kill, but it timed out. \
            This indicates kill_active_task is not properly killing the shell process."
        );
        assert!(
            killed_at.elapsed() < Duration::from_secs(5),
            "the process must stop without waiting for its natural exit",
        );

        // Give the system a chance to process any pending updates
        cx.run_until_parked();

        if !headless {
            // Verify that the underlying terminal still has the output that was
            // written before the kill. This verifies that killing doesn't lose output.
            let inner_content = thread.read_with(cx, |thread, cx| {
                let term = thread.terminals.get(&terminal_id).unwrap();
                term.read(cx).inner().read(cx).get_content()
            });

            assert!(
                inner_content.contains("output_before_kill"),
                "Underlying terminal should contain output from before kill, got: {}",
                inner_content
            );
        }
    }

    #[gpui::test]
    async fn test_push_user_content_block(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        // Test creating a new user message
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(None, "Hello, ".into(), cx);
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 1);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[0] {
                assert_eq!(user_msg.identity, MessageIdentity::Legacy(None));
                assert_eq!(user_msg.client_id, None);
                assert_eq!(user_msg.content.to_markdown(cx), "Hello, ");
            } else {
                panic!("Expected UserMessage");
            }
        });

        // Test appending to existing user message
        let message_1_id = ClientUserMessageId::new();
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(Some(message_1_id.clone()), "world!".into(), cx);
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 1);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[0] {
                assert_eq!(user_msg.identity, MessageIdentity::Legacy(None));
                assert_eq!(user_msg.client_id, Some(message_1_id));
                assert_eq!(user_msg.content.to_markdown(cx), "Hello, world!");
            } else {
                panic!("Expected UserMessage");
            }
        });

        // Test creating new user message after assistant message
        thread.update(cx, |thread, cx| {
            thread.push_assistant_content_block("Assistant response".into(), false, cx);
        });

        let message_2_id = ClientUserMessageId::new();
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(
                Some(message_2_id.clone()),
                "New user message".into(),
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 3);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[2] {
                assert_eq!(user_msg.identity, MessageIdentity::Legacy(None));
                assert_eq!(user_msg.client_id, Some(message_2_id));
                assert_eq!(user_msg.content.to_markdown(cx), "New user message");
            } else {
                panic!("Expected UserMessage at index 2");
            }
        });
    }

    fn only_keyed_message(
        thread: &AcpThread,
    ) -> (&MessageIdentity, &MessageContent, &Option<acp_v2::Meta>) {
        let [entry] = thread.entries() else {
            panic!("expected one message entry");
        };
        match entry {
            AgentThreadEntry::UserMessage(message) => {
                (&message.identity, &message.content, &message.meta)
            }
            AgentThreadEntry::AssistantMessage(message) => {
                let [chunk] = message.chunks.as_slice() else {
                    panic!("expected one assistant chunk");
                };
                match chunk {
                    AssistantMessageChunk::Message {
                        identity,
                        block,
                        meta,
                    }
                    | AssistantMessageChunk::Thought {
                        identity,
                        block,
                        meta,
                    } => (identity, block, meta),
                }
            }
            _ => panic!("expected a message"),
        }
    }

    #[gpui::test]
    async fn test_unsupported_legacy_prompt_preserves_running_turn(cx: &mut TestAppContext) {
        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection = Rc::new(StubAgentConnection::new().with_retry());
        let thread = cx
            .update(|cx| {
                connection.clone().new_session(
                    project,
                    PathList::new(&[Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .expect("legacy session");
        let complete = connection.defer_next_prompt_response();
        let running = thread.update(cx, |thread, cx| {
            thread.send(vec!["keep running".into()], cx)
        });
        cx.run_until_parked();
        let before = thread.read_with(cx, |thread, _| {
            assert_eq!(thread.status(), ThreadStatus::Generating);
            (
                thread.turn_id,
                thread.entries.len(),
                thread.activity_generation(),
            )
        });
        assert!(thread.read_with(cx, |thread, cx| thread.can_retry(cx)));

        let source = vec![acp_v2::ContentBlock::Other(acp_v2::OtherContentBlock::new(
            "_future",
            std::collections::BTreeMap::from([(
                "payload".into(),
                json!({"private": [null, true]}),
            )]),
        ))];
        let rejected = thread.update(cx, |thread, cx| thread.send(source.clone(), cx));
        let rejected_id = rejected.id;
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.status(), ThreadStatus::Generating);
            assert_eq!(
                (
                    thread.turn_id,
                    thread.entries.len(),
                    thread.activity_generation()
                ),
                before
            );
            let record = thread.submission(rejected_id).expect("rejected submission");
            assert_eq!(record.content.as_ref(), source.as_slice());
            assert!(matches!(record.state, SubmissionState::Failed(_)));
        });
        assert!(rejected.await.is_err());
        assert!(!thread.read_with(cx, |thread, cx| thread.can_retry(cx)));
        let retry = thread.update(cx, |thread, cx| thread.retry(cx));
        let retry_id = retry.id;
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                (
                    thread.turn_id,
                    thread.entries.len(),
                    thread.activity_generation()
                ),
                before
            );
            let record = thread.submission(retry_id).expect("rejected retry");
            assert_eq!(record.content.as_ref(), source.as_slice());
            assert!(matches!(record.state, SubmissionState::Failed(_)));
        });
        assert!(retry.await.is_err());
        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("the original turn must still be pending");
        assert!(matches!(
            running.await.expect("original turn completes"),
            Some(SubmissionResponse::LegacyCompleted(_))
        ));
    }

    async fn new_receipt_test_thread(
        cx: &mut TestAppContext,
    ) -> (Entity<AcpThread>, Rc<StubAgentConnection>) {
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection = Rc::new(StubAgentConnection::new().with_receipt_submissions(true));
        let thread = cx
            .update(|cx| {
                connection.clone().new_session(
                    project,
                    PathList::new(&[Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .expect("receipt session should be created");
        (thread, connection)
    }

    #[gpui::test]
    async fn test_receipt_and_echo_correlate_by_id_without_owning_history_or_activity(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let (thread, connection) = new_receipt_test_thread(cx).await;
        let stopped = Rc::new(RefCell::new(Vec::new()));
        let legacy_errors = Rc::new(RefCell::new(0));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let stopped = stopped.clone();
                let legacy_errors = legacy_errors.clone();
                move |_, event, _| match event {
                    AcpThreadEvent::Stopped {
                        activity_generation,
                        stop_reason,
                        ..
                    } => {
                        stopped
                            .borrow_mut()
                            .push((*activity_generation, stop_reason.clone()));
                    }
                    AcpThreadEvent::Refusal | AcpThreadEvent::Error => {
                        *legacy_errors.borrow_mut() += 1;
                    }
                    _ => {}
                }
            })
        });

        let first_sender = connection.defer_next_receipt_response();
        let outgoing = vec![
            "outgoing".into(),
            acp_v2::ContentBlock::Other(acp_v2::OtherContentBlock::new(
                "_future",
                std::collections::BTreeMap::from([
                    ("payload".into(), json!({"nested": [null, true]})),
                    ("_meta".into(), json!({"origin": "outgoing"})),
                ]),
            )),
        ];
        let first = thread.update(cx, |thread, cx| thread.send(outgoing.clone(), cx));
        assert_eq!(connection.take_receipt_prompt().as_ref(), Some(&outgoing));
        let first_id = first.id;
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.latest_submission_id(), Some(first_id));
            assert!(matches!(
                thread.submission(first_id).expect("first").state,
                SubmissionState::Pending
            ));
            assert_eq!(
                thread.submission(first_id).expect("first").content.as_ref(),
                outgoing.as_slice()
            );
            assert!(thread.entries().is_empty());
            assert!(thread.has_unsettled_submissions());
            assert!(!thread.is_idle_for_retention());
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert_eq!(thread.activity_generation(), 0);
        });
        thread.update(cx, |thread, cx| {
            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("first-id").content(vec!["normalized".into()]),
                    cx,
                )
                .expect("echo before receipt");
        });
        let receipt_meta = acp_v2::Meta::from_iter([("receipt".into(), json!("opaque"))]);
        first_sender
            .send(Ok(
                acp_v2::PromptResponse::new("first-id").meta(receipt_meta.clone())
            ))
            .expect("first receipt receiver");
        assert!(matches!(
            first.await.expect("first response"),
            Some(SubmissionResponse::Accepted(_))
        ));
        thread.read_with(cx, |thread, cx| {
            let (identity, content, meta) = only_keyed_message(thread);
            assert_eq!(*identity, MessageIdentity::Keyed("first-id".into()));
            assert_eq!(content.to_markdown(cx), "normalized");
            assert_ne!(meta.as_ref(), Some(&receipt_meta));
            assert!(matches!(
                &thread.submission(first_id).expect("first").state,
                SubmissionState::Accepted { receipt, echoed: true }
                    if receipt.message_id == "first-id".into() && receipt.meta.as_ref() == Some(&receipt_meta)
            ));
            assert!(!thread.has_unsettled_submissions());
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
        });

        let second_sender = connection.defer_next_receipt_response();
        let second = thread.update(cx, |thread, cx| thread.send(vec!["normalized".into()], cx));
        let second_id = second.id;
        second_sender
            .send(Ok(acp_v2::PromptResponse::new("second-id")))
            .expect("second receipt receiver");
        assert!(matches!(
            second.await.expect("second response"),
            Some(SubmissionResponse::Accepted(_))
        ));
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.submission(second_id).expect("second").state,
                SubmissionState::Accepted { echoed: false, .. }
            ));
            assert_eq!(thread.entries().len(), 1);
            assert!(thread.has_unsettled_submissions());
        });
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Running(
                        acp_v2::RunningStateUpdate::new()
                            .meta(acp_v2::Meta::from_iter([("future".into(), json!(42))])),
                    ),
                    cx,
                )
                .expect("running");
        });
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.activity_generation(), 1);
            assert!(thread.activity_started_at().is_some());
            assert!(thread.activity_duration().is_some());
            assert!(
                matches!(thread.foreground_state(), acp_v2::StateUpdate::Running(running)
                if running.meta.as_ref().and_then(|meta| meta.get("future")) == Some(&json!(42)))
            );
        });
        let late_sender = connection.defer_next_receipt_response();
        let late = thread.update(cx, |thread, cx| {
            thread.send(vec!["late receipt".into()], cx)
        });
        let late_id = late.id;
        thread.update(cx, |thread, cx| {
            thread.forget_submission(late_id, cx);
            assert!(
                thread.submission(late_id).is_some(),
                "pending receipt cannot be forgotten"
            );
        });
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Idle(
                        acp_v2::IdleStateUpdate::new().stop_reason(acp_v2::StopReason::Refusal),
                    ),
                    cx,
                )
                .expect("reported refusal");
        });
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert!(thread.had_error());
            assert_eq!(thread.entries().len(), 1);
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert!(thread.activity_duration().is_some());
            assert!(!thread.is_idle_for_retention());
            assert!(matches!(
                thread.submission(late_id).expect("late").state,
                SubmissionState::Pending
            ));
            assert!(
                matches!(thread.foreground_state(), acp_v2::StateUpdate::Idle(idle)
                if idle.stop_reason == Some(acp_v2::StopReason::Refusal))
            );
        });
        assert_eq!(*stopped.borrow(), [(1, Some(acp_v2::StopReason::Refusal))]);
        assert_eq!(*legacy_errors.borrow(), 0);
        late_sender
            .send(Ok(acp_v2::PromptResponse::new("late-id")))
            .expect("late receipt receiver");
        assert!(matches!(
            late.await.expect("late response"),
            Some(SubmissionResponse::Accepted(_))
        ));
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
            assert!(matches!(
                thread.submission(late_id).expect("late").state,
                SubmissionState::Accepted { echoed: false, .. }
            ));
        });
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Idle(
                        acp_v2::IdleStateUpdate::new().stop_reason(acp_v2::StopReason::Refusal),
                    ),
                    cx,
                )
                .expect("duplicate idle");
            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("second-id").content(vec!["normalized".into()]),
                    cx,
                )
                .expect("echo after receipt");
            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("late-id").content(vec!["agent late echo".into()]),
                    cx,
                )
                .expect("echo after late receipt");
        });
        cx.run_until_parked();
        assert_eq!(*stopped.borrow(), [(1, Some(acp_v2::StopReason::Refusal))]);
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries().len(), 3);
            assert!(
                thread.submission(second_id).is_none(),
                "older echoed receipt is pruned"
            );
            assert!(matches!(
                thread.submission(late_id).expect("late").state,
                SubmissionState::Accepted { echoed: true, .. }
            ));
            assert!(thread.recoverable_submissions().next().is_none());
            assert!(!thread.has_unsettled_submissions());
            assert!(thread.is_idle_for_retention());
        });
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new()),
                    cx,
                )
                .expect("second running generation");
            assert_eq!(thread.activity_generation(), 2);
            assert_eq!(thread.latest_submission_id(), Some(late_id));
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Idle(
                        acp_v2::IdleStateUpdate::new().stop_reason(acp_v2::StopReason::MaxTokens),
                    ),
                    cx,
                )
                .expect("max tokens");
            assert!(thread.had_error());
            assert_eq!(thread.latest_submission_id(), Some(late_id));
            assert!(matches!(
                thread.submission(late_id).expect("late").state,
                SubmissionState::Accepted { echoed: true, .. }
            ));
            assert!(thread.recoverable_submissions().next().is_none());
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Idle(
                        acp_v2::IdleStateUpdate::new().stop_reason(acp_v2::StopReason::MaxTokens),
                    ),
                    cx,
                )
                .expect("duplicate max tokens idle");
        });
        cx.run_until_parked();
        assert_eq!(
            *stopped.borrow(),
            [
                (1, Some(acp_v2::StopReason::Refusal)),
                (2, Some(acp_v2::StopReason::MaxTokens)),
            ]
        );
        for (wire, expected_had_error) in [
            (
                json!({
                    "state": "idle",
                    "stopReason": "error",
                    "error": {"code": -32000, "message": "work failed", "data": {"detail": [1, true]}}
                }),
                true,
            ),
            (json!({"state": "idle", "stopReason": "error"}), true),
            (
                json!({"state": "idle", "stopReason": "error", "error": null}),
                true,
            ),
            (
                json!({"state": "idle", "stopReason": "error", "error": {"message": 42}}),
                true,
            ),
            (
                json!({"state": "idle", "stopReason": "_custom", "detail": {"nested": [null, true]}}),
                false,
            ),
        ] {
            let idle: acp_v2::StateUpdate =
                serde_json::from_value(wire.clone()).expect("idle stop payload");
            let acp_v2::StateUpdate::Idle(details) = &idle else {
                panic!("expected idle");
            };
            let stop_reason = details.stop_reason.clone().expect("stop reason retained");
            if let acp_v2::StopReason::Error(error) = &stop_reason {
                assert_eq!(
                    error
                        .error
                        .as_ref()
                        .map(|error| serde_json::to_value(error).expect("error")),
                    wire.get("error")
                        .filter(|error| error.get("code").is_some())
                        .cloned(),
                );
            } else {
                assert_eq!(
                    stop_reason,
                    acp_v2::StopReason::Other(acp_v2::OtherStopReason::new(
                        "_custom",
                        std::collections::BTreeMap::from([(
                            "detail".into(),
                            json!({"nested": [null, true]}),
                        )]),
                    )),
                );
            }
            let previous_stops = stopped.borrow().len();
            let generation = thread.update(cx, |thread, cx| {
                thread
                    .update_session_state(
                        acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new()),
                        cx,
                    )
                    .expect("running");
                assert!(!thread.had_error());
                thread.update_session_state(idle.clone(), cx).expect("idle");
                assert_eq!(thread.foreground_state(), &idle);
                assert_eq!(thread.had_error(), expected_had_error);
                assert_eq!(thread.entries().len(), 3);
                assert!(matches!(
                    thread.submission(late_id).expect("accepted receipt").state,
                    SubmissionState::Accepted { echoed: true, .. }
                ));
                assert!(thread.recoverable_submissions().next().is_none());
                thread
                    .update_session_state(idle.clone(), cx)
                    .expect("duplicate idle");
                thread.activity_generation()
            });
            cx.run_until_parked();
            assert_eq!(stopped.borrow().len(), previous_stops + 1);
            assert_eq!(
                stopped.borrow().last(),
                Some(&(generation, Some(stop_reason)))
            );
        }
        assert_eq!(*legacy_errors.borrow(), 0);
        thread.update(cx, |thread, cx| {
            thread.forget_submission(late_id, cx);
            assert!(thread.submission(late_id).is_none());
            assert_eq!(thread.latest_submission_id(), None);
            assert_eq!(thread.entries().len(), 3);
        });
    }

    #[gpui::test]
    async fn test_reported_activity_survives_rejection_and_idle_keeps_unrelated_actions(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let (thread, connection) = new_receipt_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new()),
                    cx,
                )
                .expect("autonomous running");
        });
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        thread.read_with(cx, |thread, _| {
            assert!(!thread.is_waiting_for_confirmation());
            assert_eq!(thread.activity_generation(), 1);
        });
        let rejected_sender = connection.defer_next_receipt_response();
        let rejected = thread.update(cx, |thread, cx| thread.send(vec!["reject me".into()], cx));
        let rejected_id = rejected.id;
        let pending_sender = connection.defer_next_receipt_response();
        let pending = thread.update(cx, |thread, cx| thread.send(vec!["pending".into()], cx));
        let pending_id = pending.id;
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.latest_submission_id(), Some(pending_id));
            assert_eq!(
                thread
                    .recoverable_submissions()
                    .map(|(id, _)| id)
                    .collect::<Vec<_>>(),
                [rejected_id, pending_id],
            );
        });
        rejected_sender
            .send(Err(anyhow!("agent rejected: quota")))
            .expect("rejection receiver");
        assert!(
            rejected
                .await
                .expect_err("rejection must reach caller")
                .to_string()
                .contains("agent rejected: quota")
        );
        thread.read_with(cx, |thread, _| {
            let rejected_record = thread.submission(rejected_id).expect("rejected");
            assert!(matches!(&rejected_record.state,
                SubmissionState::Failed(error) if error.contains("agent rejected: quota")));
            assert_eq!(rejected_record.content.as_ref(), &["reject me".into()]);
            assert_eq!(thread.latest_submission_id(), Some(pending_id));
            assert_eq!(
                thread
                    .recoverable_submissions()
                    .map(|(id, _)| id)
                    .collect::<Vec<_>>(),
                [rejected_id, pending_id],
            );
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
            assert!(thread.has_unsettled_submissions());
            assert!(!thread.is_idle_for_retention());
        });
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::RequiresAction(acp_v2::RequiresActionStateUpdate::new()),
                    cx,
                )
                .expect("requires action");
        });
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        assert!(thread.read_with(cx, |thread, _| thread.is_waiting_for_confirmation()));
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new()),
                    cx,
                )
                .expect("resume running");
        });
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.activity_generation()),
            1
        );

        let tool_call_id = acp_v2::ToolCallId::new("unrelated-permission");
        let permission = request_test_permission(&thread, tool_call_id.clone(), cx);
        let cancelled = Rc::new(std::cell::Cell::new(false));
        cx.update(|cx| {
            let cancel = thread.update(cx, |thread, cx| thread.cancel(cx));
            cx.spawn({
                let cancelled = cancelled.clone();
                async move |_| {
                    cancel.await;
                    cancelled.set(true);
                }
            })
            .detach();
        });
        cx.run_until_parked();
        assert!(!cancelled.get());
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.foreground_activity()),
            ForegroundActivity::Running
        );
        thread.update(cx, |thread, cx| {
            thread
                .update_session_state(
                    acp_v2::StateUpdate::Idle(acp_v2::IdleStateUpdate::new()),
                    cx,
                )
                .expect("reported idle");
        });
        cx.run_until_parked();
        assert!(cancelled.get());
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        thread.read_with(cx, |thread, _| {
            assert!(thread.has_unsettled_submissions());
            assert!(!thread.is_idle_for_retention());
            assert!(thread.entries().iter().any(|entry| matches!(entry,
                AgentThreadEntry::ToolCall(call) if call.authorization_id().is_some())));
        });
        pending_sender
            .send(Ok(acp_v2::PromptResponse::new("pending-id")))
            .expect("pending receipt receiver");
        assert!(matches!(
            pending.await.expect("pending response"),
            Some(SubmissionResponse::Accepted(_))
        ));
        thread.update(cx, |thread, cx| {
            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("pending-id").content(vec!["agent echo".into()]),
                    cx,
                )
                .expect("pending echo");
        });
        thread.read_with(cx, |thread, _| {
            assert!(!thread.has_unsettled_submissions());
            assert_eq!(
                thread
                    .recoverable_submissions()
                    .map(|(id, _)| id)
                    .collect::<Vec<_>>(),
                [rejected_id],
            );
            assert!(!thread.is_idle_for_retention());
        });
        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_call_id,
                SelectedPermissionOutcome::new(
                    acp_v2::PermissionOptionId::new("allow"),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });
        assert!(matches!(
            permission.await,
            RequestPermissionOutcome::Selected(_)
        ));
        thread.update(cx, |thread, cx| {
            assert!(
                !thread.is_idle_for_retention(),
                "failed submission still needs recovery"
            );
            assert_eq!(
                thread
                    .submission(rejected_id)
                    .expect("recoverable failure")
                    .content
                    .as_ref(),
                &["reject me".into()]
            );
            thread.forget_submission(rejected_id, cx);
            assert!(thread.submission(rejected_id).is_none());
            assert!(thread.recoverable_submissions().next().is_none());
            assert!(thread.is_idle_for_retention());
        });
    }

    #[gpui::test]
    async fn test_receipt_settlement_survives_observer_drop_without_retaining_thread(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let (thread, connection) = new_receipt_test_thread(cx).await;
        let sender = connection.defer_next_receipt_response();
        let submission = thread.update(cx, |thread, cx| thread.send(vec!["unobserved".into()], cx));
        let id = submission.id;
        drop(submission);
        sender
            .send(Ok(acp_v2::PromptResponse::new("unobserved-id")))
            .expect("unobserved receipt receiver");
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert!(matches!(&thread.submission(id).expect("settled").state,
                SubmissionState::Accepted { receipt, echoed: false }
                    if receipt.message_id == "unobserved-id".into()));
            assert_eq!(
                thread
                    .recoverable_submissions()
                    .map(|(id, _)| id)
                    .collect::<Vec<_>>(),
                [id]
            );
            assert!(!thread.is_idle_for_retention());
        });

        let pending_sender = connection.defer_next_receipt_response();
        let pending = thread.update(cx, |thread, cx| thread.send(vec!["abandoned".into()], cx));
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread
                    .recoverable_submissions()
                    .map(|(id, _)| id)
                    .collect::<Vec<_>>(),
                [id, pending.id],
            );
        });
        let weak_thread = thread.downgrade();
        drop(pending);
        drop(thread);
        cx.run_until_parked();
        assert!(
            weak_thread.upgrade().is_none(),
            "pending receipt must not retain the thread"
        );
        drop(pending_sender);
    }

    #[gpui::test]
    async fn test_keyed_messages_keep_first_seen_order_across_interleaved_updates(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let updates = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let updates = updates.clone();
                move |_, event, _| match event {
                    AcpThreadEvent::NewEntry => updates.borrow_mut().push(None),
                    AcpThreadEvent::EntryUpdated(index) => updates.borrow_mut().push(Some(*index)),
                    _ => {}
                }
            })
        });

        thread.update(cx, |thread, cx| {
            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("user").content(vec!["question".into()]),
                    cx,
                )
                .expect("user");
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new("old answer".into(), "answer"),
                    cx,
                )
                .expect("answer chunk");
            thread
                .upsert_tool_call(acp_v1::ToolCall::new("tool", "Read file"), cx)
                .expect("tool");
            thread
                .upsert_thought(
                    acp_v2::AgentThought::new("thought").content(vec!["thinking".into()]),
                    cx,
                )
                .expect("thought");
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("later").content(vec!["later answer".into()]),
                    cx,
                )
                .expect("later answer");
        });
        cx.run_until_parked();
        assert_eq!(&*updates.borrow(), &[None, None, None, None, Some(3)]);
        updates.borrow_mut().clear();

        thread.update(cx, |thread, cx| {
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("answer")
                        .content(vec!["new ".into(), "answer".into()]),
                    cx,
                )
                .expect("replace historical answer");
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" tail".into(), "answer"),
                    cx,
                )
                .expect("append to replacement");
            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("user").content(None::<Vec<acp_v2::ContentBlock>>),
                    cx,
                )
                .expect("clear historical user");
            thread
                .append_message_chunk(
                    MessageKind::User,
                    acp_v2::ContentChunk::new("new question".into(), "user"),
                    cx,
                )
                .expect("append after clear");
            thread
                .upsert_thought(
                    acp_v2::AgentThought::new("thought").content(vec!["revised thought".into()]),
                    cx,
                )
                .expect("replace thought");
            thread.flush_streaming_text(cx);

            let [
                AgentThreadEntry::UserMessage(user),
                AgentThreadEntry::AssistantMessage(answer),
                AgentThreadEntry::ToolCall(tool),
                AgentThreadEntry::AssistantMessage(later),
            ] = thread.entries()
            else {
                panic!("updates must retain first-seen rows");
            };
            assert_eq!(user.identity, MessageIdentity::Keyed("user".into()));
            assert_eq!(user.content.source_blocks(), &["new question".into()]);
            assert_eq!(tool.id, acp_v2::ToolCallId::new("tool"));
            let [
                AssistantMessageChunk::Message {
                    identity, block, ..
                },
            ] = answer.chunks.as_slice()
            else {
                panic!("one original answer chunk");
            };
            assert_eq!(identity, &MessageIdentity::Keyed("answer".into()));
            assert_eq!(
                block.source_blocks(),
                &["new ".into(), "answer".into(), " tail".into()]
            );
            assert_eq!(block.to_markdown(cx), "new answer tail");
            let [
                AssistantMessageChunk::Thought {
                    identity, block, ..
                },
                AssistantMessageChunk::Message {
                    identity: later_id,
                    block: later_block,
                    ..
                },
            ] = later.chunks.as_slice()
            else {
                panic!("thought and later answer must retain their chunk positions");
            };
            assert_eq!(identity, &MessageIdentity::Keyed("thought".into()));
            assert_eq!(block.to_markdown(cx), "revised thought");
            assert_eq!(later_id, &MessageIdentity::Keyed("later".into()));
            assert_eq!(later_block.to_markdown(cx), "later answer");
        });
        cx.run_until_parked();
        assert_eq!(
            &*updates.borrow(),
            &[Some(1), Some(1), Some(0), Some(0), Some(3)]
        );
    }

    #[gpui::test]
    async fn test_keyed_message_patches_keep_metadata_scopes_and_tristate_fields(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for kind in [
            MessageKind::User,
            MessageKind::Assistant,
            MessageKind::Thought,
        ] {
            let thread = new_test_thread(cx).await;
            let original = json!([{"type": "text", "text": "original", "_meta": {"block": true}}]);
            let replacement = json!([{"type": "text", "text": "replacement"}]);
            for (patch, expected_content, expected_meta) in [
                (json!({"messageId": "patch"}), json!([]), json!(null)),
                (
                    json!({"messageId": "patch", "content": original, "_meta": {"message": true}}),
                    original.clone(),
                    json!({"message": true}),
                ),
                (
                    json!({"messageId": "patch"}),
                    original.clone(),
                    json!({"message": true}),
                ),
                (
                    json!({"messageId": "patch", "_meta": null}),
                    original.clone(),
                    json!(null),
                ),
                (
                    json!({"messageId": "patch", "_meta": {}}),
                    original.clone(),
                    json!({}),
                ),
                (
                    json!({"messageId": "patch", "content": replacement}),
                    replacement.clone(),
                    json!({}),
                ),
                (
                    json!({"messageId": "patch", "content": null}),
                    json!([]),
                    json!({}),
                ),
                (
                    json!({"messageId": "patch", "_meta": null}),
                    json!([]),
                    json!(null),
                ),
                (
                    json!({"messageId": "patch", "content": original, "_meta": {"restored": true}}),
                    original.clone(),
                    json!({"restored": true}),
                ),
                (
                    json!({"messageId": "patch", "content": []}),
                    json!([]),
                    json!({"restored": true}),
                ),
            ] {
                thread.update(cx, |thread, cx| {
                    match kind {
                        MessageKind::User => thread.upsert_user_message(
                            serde_json::from_value(patch).expect("user patch"),
                            cx,
                        ),
                        MessageKind::Assistant => thread.upsert_assistant_message(
                            serde_json::from_value(patch).expect("assistant patch"),
                            cx,
                        ),
                        MessageKind::Thought => thread.upsert_thought(
                            serde_json::from_value(patch).expect("thought patch"),
                            cx,
                        ),
                    }
                    .expect("apply patch");
                    let (identity, content, meta) = only_keyed_message(thread);
                    assert_eq!(identity, &MessageIdentity::Keyed("patch".into()));
                    assert_eq!(
                        serde_json::to_value(content.source_blocks()).expect("source JSON"),
                        expected_content,
                        "{kind:?}",
                    );
                    assert_eq!(
                        serde_json::to_value(meta).expect("metadata JSON"),
                        expected_meta,
                        "{kind:?}",
                    );
                });
            }
            thread.update(cx, |thread, cx| {
                let block = acp_v2::ContentBlock::Text(
                    acp_v2::TextContent::new("chunk")
                        .meta(acp_v2::Meta::from_iter([("block".into(), json!(true))])),
                );
                thread
                    .append_message_chunk(
                        kind,
                        acp_v2::ContentChunk::new(block.clone(), "patch")
                            .meta(acp_v2::Meta::from_iter([("envelope".into(), json!(true))])),
                        cx,
                    )
                    .expect("append scoped metadata");
                let (_, content, meta) = only_keyed_message(thread);
                assert_eq!(content.source_blocks(), &[block]);
                assert_eq!(
                    meta,
                    &Some(acp_v2::Meta::from_iter([("restored".into(), json!(true))]))
                );
            });
        }
    }

    #[gpui::test]
    async fn test_keyed_snapshots_supersede_only_their_own_buffered_text(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let original_markdown = thread.update(cx, |thread, cx| {
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("answer").content(vec!["visible".into()]),
                    cx,
                )
                .expect("answer");
            for text in ["", "é🦀 pending"] {
                thread
                    .append_message_chunk(
                        MessageKind::Assistant,
                        acp_v2::ContentChunk::new(text.into(), "answer"),
                        cx,
                    )
                    .expect("buffer text");
            }
            only_keyed_message(thread)
                .1
                .markdowns()
                .next()
                .expect("markdown")
                .clone()
        });
        cx.run_until_parked();

        thread.update(cx, |thread, cx| {
            let pending_bytes = thread
                .streaming_text_buffer
                .as_ref()
                .expect("buffer")
                .cursor
                .pending_bytes;
            assert_eq!(pending_bytes, "é🦀 pending".len());
            assert_eq!(original_markdown.read(cx).source(), "visible");
            let content = only_keyed_message(thread).1;
            let source_version = content.source_version();
            let snapshot = content.source_blocks().to_vec();
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("answer")
                        .meta(acp_v2::Meta::from_iter([("message".into(), json!(true))])),
                    cx,
                )
                .expect("metadata update");
            let buffer = thread
                .streaming_text_buffer
                .as_ref()
                .expect("metadata preserves buffer");
            assert_eq!(buffer.cursor.pending_bytes, pending_bytes);
            assert_eq!(buffer.target.markdown, original_markdown);
            assert_eq!(
                only_keyed_message(thread).1.source_version(),
                source_version
            );
            thread
                .upsert_assistant_message(acp_v2::AgentMessage::new("answer").content(snapshot), cx)
                .expect("snapshot identical to source, but ahead of display");
            assert!(thread.streaming_text_buffer.is_none());
            assert_eq!(original_markdown.read(cx).source(), "visibleé🦀 pending");
            assert_eq!(
                only_keyed_message(thread).1.source_version(),
                source_version
            );
            assert_eq!(
                only_keyed_message(thread).1.markdowns().next(),
                Some(&original_markdown)
            );

            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" superseded".into(), "answer"),
                    cx,
                )
                .expect("buffer superseded text");
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("answer").content(vec!["replacement".into()]),
                    cx,
                )
                .expect("replace buffered source");
            assert!(thread.streaming_text_buffer.is_none());
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(
            original_markdown.read_with(cx, |markdown, _| markdown.source().to_string()),
            "replacement"
        );

        let current_markdown = thread.update(cx, |thread, cx| {
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" discarded on clear".into(), "answer"),
                    cx,
                )
                .expect("buffer before clear");
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("answer").content(None::<Vec<acp_v2::ContentBlock>>),
                    cx,
                )
                .expect("clear buffered message");
            assert!(thread.streaming_text_buffer.is_none());
            assert!(only_keyed_message(thread).1.source_blocks().is_empty());
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new("fresh".into(), "answer"),
                    cx,
                )
                .expect("append after clear");
            let current = only_keyed_message(thread)
                .1
                .markdowns()
                .next()
                .expect("fresh markdown")
                .clone();
            assert_ne!(current, original_markdown);

            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("other").content(vec!["other".into()]),
                    cx,
                )
                .expect("second keyed record");
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" pending".into(), "other"),
                    cx,
                )
                .expect("other buffer");
            let other_markdown = thread
                .streaming_text_buffer
                .as_ref()
                .expect("other buffer")
                .target
                .markdown
                .clone();
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("answer").content(vec!["fresh replacement".into()]),
                    cx,
                )
                .expect("replace a different record");
            let buffer = thread
                .streaming_text_buffer
                .as_ref()
                .expect("unrelated buffer survives");
            assert_eq!(buffer.target.markdown, other_markdown);
            assert_eq!(buffer.cursor.pending_bytes, " pending".len());
            assert_eq!(other_markdown.read(cx).source(), "other");
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" tail".into(), "answer"),
                    cx,
                )
                .expect("switch to historical target");
            assert_eq!(other_markdown.read(cx).source(), "other pending");
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(
                        acp_v2::ContentBlock::ResourceLink(acp_v2::ResourceLink::new(
                            "link",
                            "https://example.com",
                        )),
                        "answer",
                    ),
                    cx,
                )
                .expect("flush before non-text append");
            assert!(thread.streaming_text_buffer.is_none());
            assert_eq!(
                current.read(cx).source(),
                "fresh replacement tail[@https://example.com/](https://example.com/)"
            );
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" final".into(), "answer"),
                    cx,
                )
                .expect("buffer before cancellation");
            thread.cancel(cx).detach();
            assert!(thread.streaming_text_buffer.is_none());
            current
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(
            original_markdown.read_with(cx, |markdown, _| markdown.source().to_string()),
            "replacement"
        );
        assert_eq!(
            current_markdown.read_with(cx, |markdown, _| markdown.source().to_string()),
            "fresh replacement tail[@https://example.com/](https://example.com/) final",
        );
    }

    #[gpui::test]
    async fn test_keyed_kind_conflicts_are_atomic_and_legacy_ids_stay_separate(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("shared").content(vec!["keyed".into()]),
                    cx,
                )
                .expect("keyed message");
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" buffered".into(), "shared"),
                    cx,
                )
                .expect("pending keyed text");
        });
        let updates = Rc::new(RefCell::new(0));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let updates = updates.clone();
                move |_, event, _| {
                    if matches!(
                        event,
                        AcpThreadEvent::NewEntry | AcpThreadEvent::EntryUpdated(_)
                    ) {
                        *updates.borrow_mut() += 1;
                    }
                }
            })
        });
        thread.update(cx, |thread, cx| {
            let version = only_keyed_message(thread).1.source_version();
            assert!(
                thread
                    .upsert_user_message(
                        acp_v2::UserMessage::new("shared").content(vec!["wrong role".into()]),
                        cx,
                    )
                    .is_err()
            );
            assert!(
                thread
                    .upsert_thought(
                        acp_v2::AgentThought::new("shared")
                            .meta(acp_v2::Meta::from_iter([("wrong".into(), json!(true))])),
                        cx,
                    )
                    .is_err()
            );
            for kind in [MessageKind::User, MessageKind::Thought] {
                assert!(
                    thread
                        .append_message_chunk(
                            kind,
                            acp_v2::ContentChunk::new("wrong chunk".into(), "shared"),
                            cx,
                        )
                        .is_err()
                );
            }
            let (_, content, meta) = only_keyed_message(thread);
            assert_eq!(content.source_version(), version);
            assert_eq!(
                content.source_blocks(),
                &["keyed".into(), " buffered".into()]
            );
            assert_eq!(content.to_markdown(cx), "keyed");
            assert_eq!(meta, &None);
            assert_eq!(
                thread
                    .streaming_text_buffer
                    .as_ref()
                    .expect("buffer survives")
                    .cursor
                    .pending_bytes,
                " buffered".len()
            );
        });
        cx.run_until_parked();
        assert_eq!(*updates.borrow(), 0);

        thread.update(cx, |thread, cx| {
            for chunk in [
                acp_v1::ContentChunk::new("legacy ".into()),
                acp_v1::ContentChunk::new("tail".into()).message_id("shared"),
            ] {
                thread
                    .handle_session_update(acp_v1::SessionUpdate::AgentMessageChunk(chunk), cx)
                    .expect("legacy chunk");
            }
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" again".into(), "shared"),
                    cx,
                )
                .expect("keyed append does not target legacy tail");
            thread.flush_streaming_text(cx);
            let [AgentThreadEntry::AssistantMessage(message)] = thread.entries() else {
                panic!("one assistant row");
            };
            let [
                AssistantMessageChunk::Message {
                    identity, block, ..
                },
                AssistantMessageChunk::Message {
                    identity: legacy_id,
                    block: legacy_block,
                    ..
                },
            ] = message.chunks.as_slice()
            else {
                panic!("separate keyed and legacy chunks");
            };
            assert_eq!(identity, &MessageIdentity::Keyed("shared".into()));
            assert_eq!(block.to_markdown(cx), "keyed buffered again");
            assert_eq!(legacy_id, &MessageIdentity::Legacy(Some("shared".into())));
            assert_eq!(legacy_block.to_markdown(cx), "legacy tail");

            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("user").content(vec!["keyed user".into()]),
                    cx,
                )
                .expect("keyed user");
            for chunk in [
                acp_v1::ContentChunk::new("legacy ".into()),
                acp_v1::ContentChunk::new("user".into()).message_id("user"),
            ] {
                thread
                    .handle_session_update(acp_v1::SessionUpdate::UserMessageChunk(chunk), cx)
                    .expect("legacy user chunk");
            }
            let [
                AgentThreadEntry::AssistantMessage(_),
                AgentThreadEntry::UserMessage(keyed),
                AgentThreadEntry::UserMessage(legacy),
            ] = thread.entries()
            else {
                panic!("legacy user chunks must not merge into keyed user");
            };
            assert_eq!(keyed.identity, MessageIdentity::Keyed("user".into()));
            assert_eq!(keyed.content.to_markdown(cx), "keyed user");
            assert_eq!(
                legacy.identity,
                MessageIdentity::Legacy(Some("user".into()))
            );
            assert_eq!(legacy.content.to_markdown(cx), "legacy user");
        });
    }

    #[gpui::test]
    async fn test_keyed_messages_can_reuse_removed_ids_after_rewind(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let client_id = ClientUserMessageId::new();
        let old_markdown = thread.update(cx, |thread, cx| {
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("kept").content(vec!["kept".into()]),
                    cx,
                )
                .expect("retained prefix");
            thread.push_user_content_block(Some(client_id.clone()), "prompt".into(), cx);
            thread
                .upsert_assistant_message(
                    acp_v2::AgentMessage::new("reuse").content(vec!["old".into()]),
                    cx,
                )
                .expect("old keyed answer");
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" pending".into(), "reuse"),
                    cx,
                )
                .expect("buffered old answer");
            thread
                .streaming_text_buffer
                .as_ref()
                .expect("buffer")
                .target
                .markdown
                .clone()
        });
        let rewind = thread.update(cx, |thread, cx| {
            let rewind = thread.rewind(client_id, cx);
            thread
                .append_message_chunk(
                    MessageKind::Assistant,
                    acp_v2::ContentChunk::new(" late".into(), "reuse"),
                    cx,
                )
                .expect("update before rewind settles");
            rewind
        });
        rewind.await.expect("rewind");
        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries().len(), 1);
            assert!(thread.streaming_text_buffer.is_none());
            assert_eq!(old_markdown.read(cx).source(), "old pending late");
            thread
                .upsert_user_message(
                    acp_v2::UserMessage::new("reuse").content(vec!["fresh user".into()]),
                    cx,
                )
                .expect("removed ID may identify a new kind");
            let [
                AgentThreadEntry::AssistantMessage(_),
                AgentThreadEntry::UserMessage(message),
            ] = thread.entries()
            else {
                panic!("prefix plus fresh record");
            };
            assert_eq!(message.identity, MessageIdentity::Keyed("reuse".into()));
            assert_eq!(message.content.to_markdown(cx), "fresh user");
            assert_eq!(message.meta, None);
            assert_eq!(test_message_content(thread, 0).to_markdown(cx), "kept");
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(
            old_markdown.read_with(cx, |markdown, _| markdown.source().to_string()),
            "old pending late"
        );
    }

    #[gpui::test]
    async fn test_user_message_chunks_use_protocol_message_id_boundaries(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UserMessageChunk(
                        acp_v1::ContentChunk::new("First ".into()).message_id("msg_user_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UserMessageChunk(
                        acp_v1::ContentChunk::new("message".into()).message_id("msg_user_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UserMessageChunk(
                        acp_v1::ContentChunk::new("Second message".into()).message_id("msg_user_2"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UserMessageChunk(
                        acp_v1::ContentChunk::new("Echo".into()).message_id("msg_user_3"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UserMessageChunk(
                        acp_v1::ContentChunk::new("Echo".into()).message_id("msg_user_3"),
                    ),
                    cx,
                )
                .unwrap();
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 3);

            let AgentThreadEntry::UserMessage(first_message) = &thread.entries[0] else {
                panic!("expected first entry to be a user message")
            };
            assert_eq!(first_message.content.to_markdown(cx), "First message");
            assert_eq!(
                first_message.identity,
                MessageIdentity::Legacy(Some("msg_user_1".into()))
            );

            let AgentThreadEntry::UserMessage(second_message) = &thread.entries[1] else {
                panic!("expected second entry to be a user message")
            };
            assert_eq!(second_message.content.to_markdown(cx), "Second message");
            assert_eq!(
                second_message.identity,
                MessageIdentity::Legacy(Some("msg_user_2".into()))
            );

            let AgentThreadEntry::UserMessage(third_message) = &thread.entries[2] else {
                panic!("expected third entry to be a user message")
            };
            assert_eq!(third_message.content.to_markdown(cx), "EchoEcho");
            assert_eq!(
                third_message.identity,
                MessageIdentity::Legacy(Some("msg_user_3".into()))
            );
        });
    }

    #[gpui::test]
    async fn test_protocol_user_chunk_does_not_merge_into_optimistic_prompt(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread.push_user_content_block_with_protocol_id(
                None,
                true,
                None,
                "Typed prompt".into(),
                false,
                cx,
            );
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UserMessageChunk(
                        acp_v1::ContentChunk::new("Agent user chunk".into())
                            .message_id("agent_user_chunk"),
                    ),
                    cx,
                )
                .unwrap();
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 2);

            let AgentThreadEntry::UserMessage(optimistic_message) = &thread.entries[0] else {
                panic!("expected first entry to be optimistic user message")
            };
            assert!(optimistic_message.is_optimistic);
            assert_eq!(optimistic_message.content.to_markdown(cx), "Typed prompt");
            assert_eq!(optimistic_message.identity, MessageIdentity::Legacy(None));
            assert!(optimistic_message.client_id.is_none());

            let AgentThreadEntry::UserMessage(agent_message) = &thread.entries[1] else {
                panic!("expected second entry to be protocol user chunk")
            };
            assert!(!agent_message.is_optimistic);
            assert_eq!(agent_message.content.to_markdown(cx), "Agent user chunk");
            assert_eq!(
                agent_message.identity,
                MessageIdentity::Legacy(Some("agent_user_chunk".into()))
            );
        });
    }

    #[gpui::test]
    async fn test_assistant_chunks_use_protocol_message_id_boundaries(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::AgentThoughtChunk(
                        acp_v1::ContentChunk::new("Thinking ".into()).message_id("msg_thought_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::AgentThoughtChunk(
                        acp_v1::ContentChunk::new("hard".into()).message_id("msg_thought_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::AgentThoughtChunk(
                        acp_v1::ContentChunk::new("A separate thought".into())
                            .message_id("msg_thought_2"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::AgentMessageChunk(
                        acp_v1::ContentChunk::new("Answer ".into()).message_id("msg_agent_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::AgentMessageChunk(
                        acp_v1::ContentChunk::new("done".into()).message_id("msg_agent_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::AgentMessageChunk(
                        acp_v1::ContentChunk::new("Follow-up".into()).message_id("msg_agent_2"),
                    ),
                    cx,
                )
                .unwrap();
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 1);
            let AgentThreadEntry::AssistantMessage(message) = &thread.entries[0] else {
                panic!("expected assistant entry")
            };
            assert_eq!(message.chunks.len(), 4);

            let AssistantMessageChunk::Thought {
                identity, block, ..
            } = &message.chunks[0]
            else {
                panic!("expected first chunk to be a thought")
            };
            assert_eq!(block.to_markdown(cx), "Thinking hard");
            assert_eq!(
                identity,
                &MessageIdentity::Legacy(Some("msg_thought_1".into()))
            );

            let AssistantMessageChunk::Thought {
                identity, block, ..
            } = &message.chunks[1]
            else {
                panic!("expected second chunk to be a thought")
            };
            assert_eq!(block.to_markdown(cx), "A separate thought");
            assert_eq!(
                identity,
                &MessageIdentity::Legacy(Some("msg_thought_2".into()))
            );

            let AssistantMessageChunk::Message {
                identity, block, ..
            } = &message.chunks[2]
            else {
                panic!("expected third chunk to be a message")
            };
            assert_eq!(block.to_markdown(cx), "Answer done");
            assert_eq!(
                identity,
                &MessageIdentity::Legacy(Some("msg_agent_1".into()))
            );

            let AssistantMessageChunk::Message {
                identity, block, ..
            } = &message.chunks[3]
            else {
                panic!("expected fourth chunk to be a message")
            };
            assert_eq!(block.to_markdown(cx), "Follow-up");
            assert_eq!(
                identity,
                &MessageIdentity::Legacy(Some("msg_agent_2".into()))
            );
        });
    }

    #[gpui::test]
    async fn test_thinking_concatenation(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            |_, thread, mut cx| {
                async move {
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp_v1::SessionUpdate::AgentThoughtChunk(
                                    acp_v1::ContentChunk::new("Thinking ".into()),
                                ),
                                cx,
                            )
                            .unwrap();
                        thread
                            .handle_session_update(
                                acp_v1::SessionUpdate::AgentThoughtChunk(
                                    acp_v1::ContentChunk::new("hard!".into()),
                                ),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            },
        ));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread
            .update(cx, |thread, cx| thread.send_raw("Hello from Zed!", cx))
            .await
            .unwrap();

        let output = thread.read_with(cx, |thread, cx| thread.to_markdown(cx));
        assert_eq!(
            output,
            indoc! {r#"
            ## User

            Hello from Zed!

            ## Assistant

            <thinking>
            Thinking hard!
            </thinking>

            "#}
        );
    }

    /// `send_command` runs the turn (the connection receives the typed command)
    /// but never echoes a user-message bubble, so commands like `/compact` don't
    /// show a fake user message implying the text was sent to the model.
    #[gpui::test]
    async fn test_send_command_does_not_echo_user_message(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let received_prompt: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let received_prompt = received_prompt.clone();
            move |request, thread, mut cx| {
                let received_prompt = received_prompt.clone();
                async move {
                    if let Some(acp_v2::ContentBlock::Text(text)) = request.prompt.first() {
                        *received_prompt.borrow_mut() = Some(text.text.clone());
                    }
                    // Simulate a native command producing its own thread entry
                    // (here a compaction) rather than echoing a user message.
                    thread.update(&mut cx, |thread, cx| {
                        thread.push_context_compaction(
                            ContextCompaction {
                                id: ContextCompactionId("c1".into()),
                                status: ContextCompactionStatus::Completed,
                                error: None,
                                summary: MessageContent::default(),
                                meta: None,
                            },
                            cx,
                        );
                    })?;
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        cx.update(|cx| {
            thread.update(cx, |thread, cx| {
                thread.send_command(vec!["/compact".into()], cx)
            })
        })
        .await
        .unwrap();

        // The command turn ran: the connection received the typed command.
        assert_eq!(received_prompt.borrow().as_deref(), Some("/compact"));

        thread.update(cx, |thread, _cx| {
            assert!(
                !thread
                    .entries
                    .iter()
                    .any(|entry| matches!(entry, AgentThreadEntry::UserMessage(_))),
                "send_command must not echo a user message"
            );
            // The command's own entry (here a compaction) is still shown.
            assert!(
                thread
                    .entries
                    .iter()
                    .any(|entry| matches!(entry, AgentThreadEntry::ContextCompaction(_))),
                "the command's own thread entry should still be present"
            );
        });
    }

    #[gpui::test]
    async fn test_ignore_echoed_user_message_chunks_during_active_turn(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(
            FakeAgentConnection::new()
                .without_truncate_support()
                .on_user_message(|request, thread, mut cx| {
                    async move {
                        let prompt = content::to_v1(
                            request.prompt.first().cloned().unwrap_or_else(|| "".into()),
                        )?;

                        thread.update(&mut cx, |thread, cx| {
                            thread
                                .handle_session_update(
                                    acp_v1::SessionUpdate::UserMessageChunk(
                                        acp_v1::ContentChunk::new(prompt),
                                    ),
                                    cx,
                                )
                                .unwrap();
                        })?;

                        Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                    }
                    .boxed_local()
                }),
        );

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread
            .update(cx, |thread, cx| thread.send_raw("Hello from Zed!", cx))
            .await
            .unwrap();

        let output = thread.read_with(cx, |thread, cx| thread.to_markdown(cx));
        assert_eq!(output.matches("Hello from Zed!").count(), 1);
        thread.read_with(cx, |thread, _cx| {
            let Some(AgentThreadEntry::UserMessage(message)) = thread.entries.first() else {
                panic!("expected optimistic user message");
            };
            assert_eq!(message.identity, MessageIdentity::Legacy(None));
            assert_eq!(message.client_id, None);
            assert!(message.is_optimistic);
        });
    }

    #[gpui::test]
    async fn test_edits_concurrently_to_user(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({"foo": "one\ntwo\nthree\n"}))
            .await;
        let project = Project::test(fs.clone(), [], cx).await;
        let (read_file_tx, read_file_rx) = oneshot::channel::<()>();
        let read_file_tx = Rc::new(RefCell::new(Some(read_file_tx)));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            move |_, thread, mut cx| {
                let read_file_tx = read_file_tx.clone();
                async move {
                    let content = thread
                        .update(&mut cx, |thread, cx| {
                            thread.read_text_file(path!("/tmp/foo").into(), None, None, false, cx)
                        })
                        .unwrap()
                        .await
                        .unwrap();
                    assert_eq!(content, "one\ntwo\nthree\n");
                    read_file_tx.take().unwrap().send(()).unwrap();
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.write_text_file(
                                path!("/tmp/foo").into(),
                                "one\ntwo\nthree\nfour\nfive\n".to_string(),
                                cx,
                            )
                        })
                        .unwrap()
                        .await
                        .unwrap();
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            },
        ));

        let (worktree, pathbuf) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp/foo"), true, cx)
            })
            .await
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree.read(cx).id(), pathbuf), cx)
            })
            .await
            .unwrap();

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/tmp"))]), cx)
            })
            .await
            .unwrap();

        let request = thread.update(cx, |thread, cx| {
            thread.send_raw("Extend the count in /tmp/foo", cx)
        });
        read_file_rx.await.ok();
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "zero\n".to_string())], None, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "zero\none\ntwo\nthree\nfour\nfive\n"
        );
        assert_eq!(
            String::from_utf8(fs.read_file_sync(path!("/tmp/foo")).unwrap()).unwrap(),
            "zero\none\ntwo\nthree\nfour\nfive\n"
        );
        request.await.unwrap();
    }

    #[gpui::test]
    async fn test_shared_buffers_do_not_accumulate_dead_entries(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({"foo": "one\ntwo\n"}))
            .await;
        let project = Project::test(fs.clone(), [], cx).await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp/foo"), true, cx)
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/tmp"))]), cx)
            })
            .await
            .unwrap();

        let dead_buffer = cx.new(|cx| Buffer::local("stale", cx));
        let snapshot = dead_buffer.read_with(cx, |buffer, _| buffer.snapshot());
        thread.update(cx, |thread, _| {
            thread
                .shared_buffers
                .insert(dead_buffer.downgrade(), snapshot);
        });
        drop(dead_buffer);

        thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, None, false, cx)
            })
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.shared_buffers.len(), 1);
            assert!(
                thread
                    .shared_buffers
                    .keys()
                    .all(|buffer| buffer.is_upgradable())
            );
        });
    }

    #[gpui::test]
    async fn test_reading_from_line(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({"foo": "one\ntwo\nthree\nfour\n"}))
            .await;
        let project = Project::test(fs.clone(), [], cx).await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp/foo"), true, cx)
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new());

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/tmp"))]), cx)
            })
            .await
            .unwrap();

        // Whole file
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "one\ntwo\nthree\nfour\n");

        // Only start line
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(3), None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "three\nfour\n");

        // Only limit
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, Some(2), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "one\ntwo\n");

        // Range
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(2), Some(2), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "two\nthree\n");

        // Invalid
        let err = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(6), Some(2), false, cx)
            })
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid params: \"Attempting to read beyond the end of the file, line 5:0\""
        );
    }

    #[gpui::test]
    async fn test_reading_empty_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({"foo": ""})).await;
        let project = Project::test(fs.clone(), [], cx).await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp/foo"), true, cx)
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new());

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/tmp"))]), cx)
            })
            .await
            .unwrap();

        // Whole file
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Only start line
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(1), None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Only limit
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, Some(2), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Range
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(1), Some(1), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Invalid
        let err = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(5), Some(2), false, cx)
            })
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid params: \"Attempting to read beyond the end of the file, line 1:0\""
        );
    }
    #[gpui::test]
    async fn test_reading_non_existing_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({})).await;
        let project = Project::test(fs.clone(), [], cx).await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp"), true, cx)
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new());

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/tmp"))]), cx)
            })
            .await
            .unwrap();

        // Out of project file
        let err = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/foo").into(), None, None, false, cx)
            })
            .await
            .unwrap_err();

        assert_eq!(err.code, acp_v1::ErrorCode::ResourceNotFound);
    }

    #[gpui::test]
    async fn test_succeeding_canceled_toolcall(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let id = acp_v2::ToolCallId::new("test");

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let id = id.clone();
            move |_, thread, mut cx| {
                let id = id.clone();
                async move {
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.handle_session_update(
                                acp_v1::SessionUpdate::ToolCall(
                                    acp_v1::ToolCall::new(
                                        acp_v1::ToolCallId::new(id.0.clone()),
                                        "Label",
                                    )
                                    .kind(acp_v1::ToolKind::Fetch)
                                    .status(acp_v1::ToolCallStatus::InProgress),
                                ),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let request = thread.update(cx, |thread, cx| {
            thread.send_raw("Fetch https://example.com", cx)
        });

        run_until_first_tool_call(&thread, cx).await;

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                &thread.entries[1],
                AgentThreadEntry::ToolCall(call) if call.status() == ToolCallStatus::InProgress
            ));
        });

        thread.update(cx, |thread, cx| thread.cancel(cx)).await;

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                &thread.entries[1],
                AgentThreadEntry::ToolCall(call) if call.status() == ToolCallStatus::Canceled
            ));
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(id.0),
                        acp_v1::ToolCallUpdateFields::new()
                            .status(acp_v1::ToolCallStatus::Completed),
                    )),
                    cx,
                )
            })
            .unwrap();

        request.await.unwrap();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                &thread.entries[1],
                AgentThreadEntry::ToolCall(call) if call.status() == ToolCallStatus::Completed
            ));
        });
    }

    #[gpui::test]
    async fn test_context_compaction_session_updates(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("failed to create ACP thread");

        thread.update(cx, |thread, cx| {
            for text in ["first ", "second"] {
                thread
                    .handle_session_update(
                        acp_v1::SessionUpdate::CompactionSummaryChunk(
                            acp_v1::CompactionSummaryChunk::new(
                                "compaction",
                                acp_v1::ContentBlock::Text(acp_v1::TextContent::new(text)),
                            )
                            .meta(acp_v1::Meta::from_iter([("delivery".into(), true.into())])),
                        ),
                        cx,
                    )
                    .expect("summary chunks can precede the first status update");
            }
            let [AgentThreadEntry::ContextCompaction(compaction)] = thread.entries.as_slice()
            else {
                panic!("chunk-first compaction must create one timeline entry");
            };
            assert_eq!(compaction.status, ContextCompactionStatus::InProgress);
            assert!(compaction.meta.is_none());
            assert!(thread.to_markdown(cx).contains("first second"));

            for summary in ["retained context", "replacement summary"] {
                thread
                    .handle_session_update(
                        acp_v1::SessionUpdate::CompactionUpdate(
                            acp_v1::CompactionUpdate::new(
                                "compaction",
                                acp_v1::CompactionStatus::Completed,
                            )
                            .summary(vec![acp_v1::ContentBlock::Text(
                                acp_v1::TextContent::new(summary),
                            )]),
                        ),
                        cx,
                    )
                    .expect("failed to set context compaction summary");
                let Some(AgentThreadEntry::ContextCompaction(compaction)) = thread.entries.last()
                else {
                    panic!("compaction entry must exist");
                };
                assert_eq!(compaction.status, ContextCompactionStatus::Completed);
                assert!(compaction.error.is_none());
                assert_eq!(
                    compaction
                        .summary
                        .blocks()
                        .next()
                        .map(|block| block.to_markdown(cx)),
                    Some(summary)
                );
                assert_eq!(
                    thread.to_markdown(cx),
                    format!("## Context Compaction (Completed)\n\n{summary}\n\n")
                );
            }

            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::CompactionUpdate(
                        acp_v1::CompactionUpdate::new(
                            "compaction",
                            acp_v1::CompactionStatus::Failed,
                        )
                        .summary(None::<Vec<acp_v1::ContentBlock>>)
                        .error("model unavailable"),
                    ),
                    cx,
                )
                .expect("failed to record context compaction failure");
            let Some(AgentThreadEntry::ContextCompaction(compaction)) = thread.entries.last()
            else {
                panic!("compaction entry must still exist");
            };
            assert_eq!(compaction.status, ContextCompactionStatus::Failed);
            assert_eq!(compaction.summary.blocks().len(), 0);
            assert_eq!(
                compaction
                    .error
                    .as_ref()
                    .map(|error| error.read(cx).source().as_ref()),
                Some("model unavailable")
            );
            assert_eq!(
                thread.to_markdown(cx),
                "## Context Compaction (Failed)\n\n**Error:** model unavailable\n\n"
            );

            for error in [
                MaybeUndefined::Undefined,
                MaybeUndefined::Value("model *still* unavailable".to_string()),
                MaybeUndefined::Null,
            ] {
                let expected_error = match &error {
                    MaybeUndefined::Undefined => Some("model unavailable".to_string()),
                    MaybeUndefined::Value(error) => Some(error.clone()),
                    MaybeUndefined::Null => None,
                };
                thread
                    .handle_session_update(
                        acp_v1::SessionUpdate::CompactionUpdate(
                            acp_v1::CompactionUpdate::new(
                                "compaction",
                                acp_v1::CompactionStatus::Failed,
                            )
                            .error(error),
                        ),
                        cx,
                    )
                    .expect("failed to patch compaction error");
                let Some(AgentThreadEntry::ContextCompaction(compaction)) = thread.entries.last()
                else {
                    panic!("compaction entry must still exist");
                };
                assert_eq!(
                    compaction
                        .error
                        .as_ref()
                        .map(|error| error.read(cx).source().to_string()),
                    expected_error
                );
                if let Some(error) = expected_error {
                    assert!(
                        thread
                            .to_markdown(cx)
                            .contains(&format!("**Error:** {}", MarkdownEscaped(&error)))
                    );
                }
            }

            for summary in [
                vec![acp_v1::ContentBlock::Text(acp_v1::TextContent::new(""))],
                Vec::new(),
            ] {
                let expected_length = summary.len();
                thread
                    .handle_session_update(
                        acp_v1::SessionUpdate::CompactionUpdate(
                            acp_v1::CompactionUpdate::new(
                                "compaction",
                                acp_v1::CompactionStatus::Completed,
                            )
                            .summary(summary),
                        ),
                        cx,
                    )
                    .expect("failed to patch compaction summary");
                let Some(AgentThreadEntry::ContextCompaction(compaction)) = thread.entries.last()
                else {
                    panic!("compaction entry must still exist");
                };
                assert_eq!(compaction.summary.blocks().len(), expected_length);
            }
        });
    }

    #[gpui::test]
    async fn test_context_compaction_preserves_summary_content(cx: &mut TestAppContext) {
        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("failed to create ACP thread");
        let image_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";
        let text_resource =
            acp_v1::EmbeddedResource::new(acp_v1::EmbeddedResourceResource::TextResourceContents(
                acp_v1::TextResourceContents::new("Retained resource text", "summary://context")
                    .mime_type("text/markdown".to_string()),
            ));
        let blob_resource =
            acp_v1::EmbeddedResource::new(acp_v1::EmbeddedResourceResource::BlobResourceContents(
                acp_v1::BlobResourceContents::new("c3VtbWFyeQ==", "summary://attachment")
                    .mime_type("application/octet-stream".to_string()),
            ));
        let audio = acp_v1::ContentBlock::Audio(acp_v1::AudioContent::new("YXVkaW8=", "audio/wav"));
        let blocks = vec![
            acp_v1::ContentBlock::Text(acp_v1::TextContent::new("retained ").meta(
                acp_v1::Meta::from_iter([(
                    "text".into(),
                    serde_json::json!({"part": 1, "nested": [null, true]}),
                )]),
            )),
            acp_v1::ContentBlock::Text(acp_v1::TextContent::new("context").meta(
                acp_v1::Meta::from_iter([("text".into(), serde_json::json!({"part": 2}))]),
            )),
            acp_v1::ContentBlock::Resource(text_resource),
            acp_v1::ContentBlock::Image(acp_v1::ImageContent::new(image_data, "image/png").meta(
                acp_v1::Meta::from_iter([(
                    "image".into(),
                    serde_json::json!({"nested": [1, {"value": "retained"}]}),
                )]),
            )),
            acp_v1::ContentBlock::ResourceLink(acp_v1::ResourceLink::new(
                "Reference",
                "https://example.com/context",
            )),
            audio.clone(),
            acp_v1::ContentBlock::Resource(blob_resource),
        ];
        let source_blocks = blocks
            .iter()
            .cloned()
            .map(content::from_v1)
            .collect::<Result<Vec<_>>>()
            .expect("convert retained content");
        let [
            _,
            _,
            acp_v2::ContentBlock::Resource(text_resource),
            _,
            _,
            stored_audio,
            acp_v2::ContentBlock::Resource(blob_resource),
        ] = source_blocks.as_slice()
        else {
            panic!("expected retained summary content");
        };

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::CompactionUpdate(acp_v1::CompactionUpdate::new(
                        "compaction",
                        acp_v1::CompactionStatus::InProgress,
                    )),
                    cx,
                )
                .expect("failed to start compaction");
            for block in &blocks {
                thread
                    .handle_session_update(
                        acp_v1::SessionUpdate::CompactionSummaryChunk(
                            acp_v1::CompactionSummaryChunk::new("compaction", block.clone()),
                        ),
                        cx,
                    )
                    .expect("failed to append summary block");
            }
        });

        for replace_summary in [false, true] {
            thread.update(cx, |thread, cx| {
                let mut update = acp_v1::CompactionUpdate::new(
                    "compaction",
                    acp_v1::CompactionStatus::Completed,
                );
                if replace_summary {
                    update = update.summary(blocks.clone());
                }
                thread
                    .handle_session_update(acp_v1::SessionUpdate::CompactionUpdate(update), cx)
                    .expect("failed to complete compaction");
            });
            thread.read_with(cx, |thread, cx| {
                let Some(AgentThreadEntry::ContextCompaction(compaction)) = thread.entries.first()
                else {
                    panic!("expected compaction entry");
                };
                assert_eq!(compaction.summary.source_blocks(), source_blocks);
                let rendered = compaction.summary.blocks().collect::<Vec<_>>();
                let [text, resource, image, link, unsupported, blob] = rendered.as_slice() else {
                    panic!("expected six retained content blocks");
                };
                assert_eq!(text.to_markdown(cx), "retained context");
                assert_eq!(
                    resource.embedded_resource().map(|(resource, _)| resource),
                    Some(text_resource)
                );
                assert_eq!(resource.to_markdown(cx), "Retained resource text");
                assert_eq!(
                    image
                        .image()
                        .and_then(|(_, dimensions)| dimensions)
                        .map(|size| (size.width, size.height)),
                    Some((1, 1))
                );
                assert_eq!(
                    link.resource_link().map(|link| link.uri.as_str()),
                    Some("https://example.com/context")
                );
                let Some(content) = unsupported.unsupported_content() else {
                    panic!("expected preserved audio with a visible fallback");
                };
                assert_eq!(content, stored_audio);
                assert!(unsupported.visible_content(cx));
                assert_eq!(
                    blob.embedded_resource().map(|(resource, _)| resource),
                    Some(blob_resource)
                );
                assert!(blob.visible_content(cx));
                assert_eq!(
                    thread.to_markdown(cx),
                    concat!(
                        "## Context Compaction (Completed)\n\n",
                        "retained context\n\n",
                        "Retained resource text\n\n",
                        "`Image`\n\n",
                        "https://example.com/context\n\n",
                        "Audio content is not supported.\n\n",
                        "summary://attachment\n\n",
                    )
                );
            });
        }

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::CompactionUpdate(
                        acp_v1::CompactionUpdate::new(
                            "compaction",
                            acp_v1::CompactionStatus::Completed,
                        )
                        .summary(vec![audio.clone()]),
                    ),
                    cx,
                )
                .expect("failed to replace summary with audio");
        });
        thread.read_with(cx, |thread, cx| {
            let Some(AgentThreadEntry::ContextCompaction(compaction)) = thread.entries.first()
            else {
                panic!("expected compaction entry");
            };
            assert_eq!(
                compaction.summary.source_blocks(),
                std::slice::from_ref(stored_audio)
            );
            let rendered = compaction.summary.blocks().collect::<Vec<_>>();
            assert!(matches!(
                rendered.as_slice(),
                [block] if block.unsupported_content() == Some(stored_audio)
            ));
            assert!(
                thread
                    .to_markdown(cx)
                    .contains("Audio content is not supported.")
            );
        });
    }

    #[gpui::test]
    async fn test_context_compaction_updates_preserve_timeline_and_emit_events(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("failed to create ACP thread");
        let events = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            let events = events.clone();
            cx.subscribe(&thread, move |_, event, _| match event {
                AcpThreadEvent::NewEntry => events.borrow_mut().push(None),
                AcpThreadEvent::EntryUpdated(index) => events.borrow_mut().push(Some(*index)),
                _ => {}
            })
        });

        thread.update(cx, |thread, cx| {
            for update in [
                acp_v1::SessionUpdate::CompactionUpdate(
                    acp_v1::CompactionUpdate::new("first", acp_v1::CompactionStatus::InProgress)
                        .meta(acp_v1::Meta::from_iter([("original".into(), true.into())])),
                ),
                acp_v1::SessionUpdate::ToolCall(acp_v1::ToolCall::new("tool", "Unrelated tool")),
                acp_v1::SessionUpdate::CompactionUpdate(acp_v1::CompactionUpdate::new(
                    "second",
                    acp_v1::CompactionStatus::InProgress,
                )),
                acp_v1::SessionUpdate::CompactionUpdate(acp_v1::CompactionUpdate::new(
                    "second",
                    acp_v1::CompactionStatus::Completed,
                )),
                acp_v1::SessionUpdate::CompactionSummaryChunk(
                    acp_v1::CompactionSummaryChunk::new(
                        "first",
                        acp_v1::ContentBlock::Text(acp_v1::TextContent::new("partial summary")),
                    )
                    .meta(acp_v1::Meta::from_iter([("delivery".into(), true.into())])),
                ),
            ] {
                thread
                    .handle_session_update(update, cx)
                    .expect("failed to apply interleaved session update");
            }
            assert!(thread.is_compacting());
        });
        assert_eq!(*events.borrow(), [None, None, None, Some(2), Some(0)]);

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::CompactionUpdate(acp_v1::CompactionUpdate::new(
                        "first",
                        acp_v1::CompactionStatus::Cancelled,
                    )),
                    cx,
                )
                .expect("failed to cancel compaction");
            assert!(!thread.is_compacting());
        });
        assert_eq!(
            *events.borrow(),
            [None, None, None, Some(2), Some(0), Some(0)]
        );
        thread.read_with(cx, |thread, cx| {
            let [
                AgentThreadEntry::ContextCompaction(first),
                AgentThreadEntry::ToolCall(_),
                AgentThreadEntry::ContextCompaction(second),
            ] = thread.entries.as_slice()
            else {
                panic!("updates must preserve the original timeline positions");
            };
            assert_eq!(first.id.0.as_ref(), "first");
            assert_eq!(first.status, ContextCompactionStatus::Canceled);
            assert_eq!(
                first.meta,
                Some(acp_v2::Meta::from_iter([("original".into(), true.into())]))
            );
            assert_eq!(second.id.0.as_ref(), "second");
            assert_eq!(second.status, ContextCompactionStatus::Completed);
            assert!(
                thread
                    .to_markdown(cx)
                    .starts_with("## Context Compaction (Canceled)\n\npartial summary\n\n")
            );
        });

        let mut source = vec![
            acp_v2::ContentBlock::Text(acp_v2::TextContent::new("replacement ").meta(
                acp_v2::Meta::from_iter([(
                    "content".into(),
                    serde_json::json!({"nested": [null, true]}),
                )]),
            )),
            acp_v2::ContentBlock::Other(acp_v2::OtherContentBlock::new(
                "_future_summary",
                std::collections::BTreeMap::from([(
                    "payload".into(),
                    serde_json::json!({"nested": [1, {"value": "retained"}]}),
                )]),
            )),
        ];
        let metadata = acp_v2::Meta::from_iter([(
            "compaction".into(),
            serde_json::json!({"nested": [1, "retained"]}),
        )]);
        thread.update(cx, |thread, cx| {
            thread.upsert_context_compaction_update(
                acp_v2::CompactionUpdate::new("first", acp_v2::CompactionStatus::Completed)
                    .summary(source.clone())
                    .meta(metadata.clone()),
                cx,
            );
            thread.upsert_context_compaction_update(
                acp_v2::CompactionUpdate::new(
                    "first",
                    acp_v2::CompactionStatus::Other("_future".into()),
                ),
                cx,
            );
            let late_text = acp_v2::ContentBlock::from("late text");
            thread.append_context_compaction_summary(
                acp_v2::CompactionSummaryChunk::new("first", late_text.clone())
                    .meta(acp_v2::Meta::from_iter([("delivery".into(), true.into())])),
                cx,
            );
            source.push(late_text);
            let Some(AgentThreadEntry::ContextCompaction(first)) = thread.entries.first() else {
                panic!("the original timeline position must remain");
            };
            assert_eq!(
                first.status,
                ContextCompactionStatus::Other("_future".into())
            );
            assert_eq!(first.summary.source_blocks(), source);
            assert_eq!(first.meta.as_ref(), Some(&metadata));
            assert!(!thread.is_compacting());
        });
        assert_eq!(
            *events.borrow(),
            [
                None,
                None,
                None,
                Some(2),
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                Some(0)
            ]
        );
        thread.update(cx, |thread, cx| {
            thread.upsert_context_compaction_update(
                acp_v2::CompactionUpdate::new("first", acp_v2::CompactionStatus::Failed)
                    .error("failure detail")
                    .meta(acp_v2::Meta::new()),
                cx,
            );
            thread.upsert_context_compaction_update(
                acp_v2::CompactionUpdate::new("first", acp_v2::CompactionStatus::Failed),
                cx,
            );
            let Some(AgentThreadEntry::ContextCompaction(first)) = thread.entries.first() else {
                panic!("the original compaction must remain");
            };
            assert_eq!(first.summary.source_blocks(), source);
            assert_eq!(first.meta, Some(acp_v2::Meta::new()));
            assert_eq!(
                first
                    .error
                    .as_ref()
                    .expect("failure detail")
                    .read(cx)
                    .source(),
                "failure detail"
            );
            thread.upsert_context_compaction_update(
                acp_v2::CompactionUpdate::new("first", acp_v2::CompactionStatus::Failed)
                    .summary(MaybeUndefined::Null)
                    .error(MaybeUndefined::Null)
                    .meta(MaybeUndefined::Null),
                cx,
            );
            let Some(AgentThreadEntry::ContextCompaction(first)) = thread.entries.first() else {
                panic!("clearing details must not remove the compaction");
            };
            assert!(first.summary.source_blocks().is_empty());
            assert_eq!(first.summary.blocks().len(), 0);
            assert!(first.error.is_none());
            assert!(first.meta.is_none());
        });
    }

    #[gpui::test]
    async fn test_legacy_tool_update_id_shares_backing_arc(cx: &mut TestAppContext) {
        init_test(cx);
        for value in ["", "tool/ \0 雪 😀"] {
            let thread = new_test_thread(cx).await;
            thread.update(cx, |thread, cx| {
                let backing: Arc<str> = value.into();
                let wire_id = acp_v1::ToolCallId::new(backing.clone());
                thread
                    .update_tool_call(
                        acp_v1::ToolCallUpdate::new(
                            wire_id.clone(),
                            acp_v1::ToolCallUpdateFields::new(),
                        ),
                        cx,
                    )
                    .expect("legacy update creates the failed placeholder");
                let (_, call) = thread
                    .tool_call(&acp_v2::ToolCallId::new(value))
                    .expect("canonical tool");
                assert_eq!(&call.id, &acp_v2::ToolCallId::new(value));
                assert!(Arc::ptr_eq(&call.id.0, &backing));
                assert_eq!(
                    serde_json::to_value(&call.id).expect("canonical ID"),
                    serde_json::to_value(&wire_id).expect("wire ID"),
                );
            });
        }
    }

    #[test]
    fn test_subagent_session_info_preserves_legacy_serialization() {
        for value in ["", "session/ \0 雪 😀"] {
            for message_end_index in [None, Some(7)] {
                let wire_id = acp_v1::SessionId::new(value);
                let mut wire = json!({
                    "session_id": wire_id,
                    "message_start_index": 3,
                });
                if let Some(message_end_index) = message_end_index {
                    wire["message_end_index"] = json!(message_end_index);
                }
                let info: SubagentSessionInfo =
                    serde_json::from_value(wire.clone()).expect("legacy session metadata");
                assert_eq!(info.session_id, acp_v2::SessionId::new(value));
                assert_eq!(info.message_start_index, 3);
                assert_eq!(info.message_end_index, message_end_index);
                assert_eq!(serde_json::to_value(info).expect("session metadata"), wire);
            }
        }
    }

    #[gpui::test]
    async fn test_native_tool_updates_preserve_json_null(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let id = acp_v2::ToolCallId::new("native");
            thread
                .upsert_local_tool_call(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .raw_input(json!({"input": true}))
                        .raw_output(json!({"output": true})),
                    cx,
                )
                .expect("native creation");
            thread
                .update_tool_call(acp_v2::ToolCallUpdate::new(id.clone()), cx)
                .expect("omitted raw fields preserve native state");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.raw_input, Some(json!({"input": true})));
            assert_eq!(call.raw_output, Some(json!({"output": true})));

            thread
                .update_tool_call(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .raw_input(json!(null))
                        .raw_output(json!(null)),
                    cx,
                )
                .expect("JSON null remains data in a typed update");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.raw_input, Some(json!(null)));
            assert_eq!(call.raw_output, Some(json!(null)));

            thread
                .update_tool_call(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .raw_input(None)
                        .raw_output(None),
                    cx,
                )
                .expect("explicit clear removes raw fields");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert!(call.raw_input.is_none());
            assert!(call.raw_output.is_none());
        });
    }

    #[gpui::test]
    async fn test_native_tool_terminal_references_require_client_processes(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        cx.executor().allow_parking();
        cx.update(|cx| cx.set_global(::terminal::HeadlessTerminal(true)));
        let terminal = thread
            .update(cx, |thread, cx| {
                thread.create_terminal(
                    "echo native-output".into(),
                    Vec::new(),
                    Vec::new(),
                    None,
                    None,
                    None,
                    cx,
                )
            })
            .await
            .expect("real client process");
        let terminal_id = terminal.read_with(cx, |terminal, _| terminal.id().clone());
        let lower = terminal.read_with(cx, |terminal, _| terminal.inner().clone());
        let process_exit = terminal.read_with(cx, |terminal, _| {
            assert!(terminal.is_process_backed());
            terminal.wait_for_exit().expect("client process exit")
        });
        process_exit.await;
        thread.update(cx, |thread, cx| {
            assert!(terminal.read(cx).is_process_backed());
            let inner_meta = acp_v2::Meta::from_iter([("inner".into(), json!({"all": [1, 2]}))]);
            let item_meta = acp_v2::Meta::from_iter([("item".into(), json!({"keep": true}))]);
            let source =
                acp_v2::ContentBlock::Text(acp_v2::TextContent::new("text").meta(inner_meta));
            let content = vec![
                acp_v2::ToolCallContent::Content(Box::new(
                    acp_v2::Content::new(source.clone()).meta(item_meta.clone()),
                )),
                acp_v2::ToolCallContent::Terminal(
                    acp_v2::Terminal::new(terminal_id.clone()).meta(item_meta.clone()),
                ),
            ];
            let id = acp_v2::ToolCallId::new("native-content");
            thread
                .upsert_local_tool_call(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .title("Native tool")
                        .content(content.clone()),
                    cx,
                )
                .expect("native content can reference a completed client process");
            thread
                .update_tool_call(acp_v2::ToolCallUpdate::new(id.clone()).content(content), cx)
                .expect("native content update uses the same conversion");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.structured_content.len(), 2);
            let ToolCallContent::ContentBlock { block, meta } =
                call.structured_content.first().expect("content")
            else {
                panic!("expected retained content block");
            };
            assert_eq!(block.source.as_ref(), Some(&source));
            assert_eq!(meta.as_ref(), Some(&item_meta));
            let ToolCallContent::Terminal {
                terminal: retained,
                meta,
            } = call.structured_content.last().expect("terminal")
            else {
                panic!("expected retained terminal");
            };
            assert_eq!(retained, &terminal);
            assert_eq!(meta.as_ref(), Some(&item_meta));
            assert!(retained.read(cx).is_process_backed());
            assert_eq!(retained.read(cx).inner(), &lower);
            assert_eq!(thread.terminals.len(), 1);

            let missing = acp_v2::ToolCallContent::Terminal(acp_v2::Terminal::new("missing"));
            assert!(
                thread
                    .upsert_local_tool_call(
                        acp_v2::ToolCallUpdate::new("missing-content")
                            .title("Missing")
                            .content(vec![missing.clone()]),
                        cx,
                    )
                    .is_err()
            );
            assert!(thread.tool_call(&"missing-content".into()).is_none());
            assert!(
                thread
                    .update_tool_call(
                        acp_v2::ToolCallUpdate::new(id.clone())
                            .title("Must not mutate")
                            .content(vec![missing]),
                        cx,
                    )
                    .is_err()
            );
            assert!(thread.terminal(acp_v2::TerminalId::new("missing")).is_err());
            assert_eq!(thread.terminals.len(), 1);
            let (_, call) = thread.tool_call(&id).expect("unchanged tool");
            assert_eq!(call.title.as_deref(), Some("Native tool"));
            assert_eq!(call.terminals().next(), Some(&terminal));

            thread
                .upsert_display_terminal(
                    "agent-display".into(),
                    DisplayTerminalPatch::default(),
                    cx,
                )
                .expect("existing agent-owned display");
            let display = acp_v2::ToolCallContent::Terminal(acp_v2::Terminal::new("agent-display"));
            assert!(
                thread
                    .upsert_local_tool_call(
                        acp_v2::ToolCallUpdate::new("display-create")
                            .title("Must not create")
                            .content(vec![display.clone()]),
                        cx,
                    )
                    .is_err()
            );
            assert!(thread.tool_call(&"display-create".into()).is_none());
            assert!(
                thread
                    .update_tool_call(
                        acp_v2::ToolCallUpdate::new(id.clone())
                            .title("Must not mutate")
                            .content(vec![display.clone()]),
                        cx,
                    )
                    .is_err()
            );
            assert!(
                thread
                    .request_tool_call_update_authorization(
                        acp_v2::ToolCallUpdate::new(id.clone())
                            .title("Must not authorize")
                            .content(vec![display]),
                        PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                            "allow",
                            "Allow",
                            acp_v2::PermissionOptionKind::AllowOnce,
                        )]),
                        AuthorizationKind::PermissionGrant,
                        cx,
                    )
                    .is_err()
            );
            let (_, call) = thread.tool_call(&id).expect("unchanged native tool");
            assert_eq!(call.title.as_deref(), Some("Native tool"));
            assert_eq!(call.terminals().next(), Some(&terminal));
            assert!(call.authorization_id().is_none());
            assert!(thread.pending_permission_requests().next().is_none());
            assert_eq!(thread.terminals.len(), 2);
        });
        terminal.read_with(cx, |terminal, _| {
            let output = terminal.output().expect("completed client process");
            assert!(output.content.contains("native-output"));
            assert_eq!(output.exit_status.exit_code, Some(0));
        });
    }

    #[gpui::test]
    async fn test_native_permission_replacement_retains_continuation_and_owner(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let id = acp_v2::ToolCallId::new("native-permission");
        thread.update(cx, |thread, cx| {
            thread
                .upsert_local_tool_call(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .title("Native tool")
                        .status(acp_v2::ToolCallStatus::InProgress),
                    cx,
                )
                .expect("native tool");
        });
        let options = PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
            "allow",
            "Allow",
            acp_v2::PermissionOptionKind::AllowOnce,
        )]);
        let (old_owner, old_response) = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_update_authorization_with_id(
                    acp_v2::ToolCallUpdate::new(id.clone()),
                    options.clone(),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("first request");
        let (new_owner, response) = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_update_authorization_with_id(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .status(acp_v2::ToolCallStatus::Completed),
                    options,
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("replacement captures underlying progress before descriptive completion");
        assert_ne!(old_owner, new_owner);
        assert!(matches!(
            old_response.await,
            RequestPermissionOutcome::Cancelled
        ));
        thread.update(cx, |thread, cx| {
            assert!(thread.permission_request(old_owner).is_none());
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.authorization_id(), Some(new_owner));
            assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
            assert_eq!(
                call.reported_status,
                Some(acp_v2::ToolCallStatus::Completed)
            );
            assert_eq!(
                call.permission_status(),
                Some(acp_v2::ToolCallStatus::InProgress)
            );
            thread.authorize_permission_request(
                new_owner,
                SelectedPermissionOutcome::new(
                    "allow".into(),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.status(), ToolCallStatus::InProgress);
            assert_eq!(
                call.reported_status,
                Some(acp_v2::ToolCallStatus::Completed)
            );
            assert!(call.authorization_id().is_none());
            assert!(thread.permission_request(new_owner).is_none());
            assert!(thread.pending_permission_requests().next().is_none());
        });
        assert!(matches!(
            response.await,
            RequestPermissionOutcome::Selected(_)
        ));
    }

    #[gpui::test]
    async fn test_native_permission_cancellation_and_reported_completion(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let id = acp_v2::ToolCallId::new("native-permission");
        let options = PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
            "allow",
            "Allow",
            acp_v2::PermissionOptionKind::AllowOnce,
        )]);
        let (owner, response) = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_update_authorization_with_id(
                    acp_v2::ToolCallUpdate::new(id.clone()),
                    options.clone(),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("ID-only request creates native tool with client fallback defaults");
        thread.update(cx, |thread, cx| {
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.title, None);
            assert_eq!(call.reported_kind, None);
            assert_eq!(call.reported_status, None);
            assert_eq!(call.kind(), &acp_v2::ToolKind::Other);
            assert_eq!(
                call.permission_status(),
                Some(acp_v2::ToolCallStatus::Pending)
            );
            assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
            thread.cancel_permission_request(owner, cx);
        });
        assert!(matches!(
            response.await,
            RequestPermissionOutcome::Cancelled
        ));
        thread.read_with(cx, |thread, _| {
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.status(), ToolCallStatus::Canceled);
            assert!(call.authorization_id().is_none());
            assert!(thread.permission_request(owner).is_none());
            assert!(thread.pending_permission_requests().next().is_none());
        });
        let response = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_update_authorization(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .status(acp_v2::ToolCallStatus::InProgress),
                    options.clone(),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("local canceled state falls back to the incoming continuation");
        thread.update(cx, |thread, cx| {
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.permission_status(),
                Some(acp_v2::ToolCallStatus::InProgress)
            );
            thread.authorize_tool_call(
                id.clone(),
                SelectedPermissionOutcome::new(
                    "allow".into(),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.status(),
                ToolCallStatus::InProgress
            );
        });
        assert!(matches!(
            response.await,
            RequestPermissionOutcome::Selected(_)
        ));
        let (owner, response) = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_update_authorization_with_id(
                    acp_v2::ToolCallUpdate::new(id.clone()),
                    options,
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("request before completion");
        thread.update(cx, |thread, cx| {
            thread
                .update_tool_call(
                    acp_v2::ToolCallUpdate::new(id.clone())
                        .title("Must not mutate")
                        .status(acp_v2::ToolCallStatus::Completed)
                        .content(vec![acp_v2::ToolCallContent::Terminal(
                            acp_v2::Terminal::new("missing"),
                        )]),
                    cx,
                )
                .expect_err("missing terminal must not be fabricated");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.status(), ToolCallStatus::Completed);
            assert_eq!(
                call.reported_status,
                Some(acp_v2::ToolCallStatus::Completed)
            );
            assert!(call.title.is_none());
            assert!(call.structured_content.is_empty());
            assert!(call.authorization_id().is_none());
            assert!(thread.permission_request(owner).is_none());
            assert!(thread.permission_request_for_tool(&id).is_none());
            assert!(thread.pending_permission_requests().next().is_none());
            assert!(thread.terminals.is_empty());
        });
        assert!(matches!(
            response.await,
            RequestPermissionOutcome::Cancelled
        ));
    }

    #[gpui::test]
    async fn test_tool_ids_preserve_identity_across_legacy_and_v2_updates(cx: &mut TestAppContext) {
        init_test(cx);
        for value in ["", "tool/ \0 雪 😀"] {
            let thread = new_test_thread(cx).await;
            let backing: Arc<str> = value.into();
            let id = acp_v2::ToolCallId::new(backing.clone());
            let wire_id = acp_v1::ToolCallId::new(backing.clone());
            thread.update(cx, |thread, cx| {
                thread
                    .upsert_tool_call(
                        acp_v1::ToolCall::new(wire_id.clone(), "Legacy tool")
                            .raw_input(json!({"retain": true})),
                        cx,
                    )
                    .expect("legacy tool");
                let (_, call) = thread.tool_call(&id).expect("canonical tool");
                assert!(Arc::ptr_eq(&call.id.0, &backing));

                thread
                    .upsert_wire_tool_call(
                        acp_v2::ToolCallUpdate::new(acp_v2::ToolCallId::new(value))
                            .title("V2 patch"),
                        cx,
                    )
                    .expect("v2 patch targets legacy tool");
                thread
                    .append_tool_call_content_chunk(
                        acp_v2::ToolCallContentChunk::new(acp_v2::ToolCallId::new(value), "chunk"),
                        cx,
                    )
                    .expect("v2 chunk targets legacy tool");
                thread
                    .update_tool_call(
                        acp_v1::ToolCallUpdate::new(
                            wire_id.clone(),
                            acp_v1::ToolCallUpdateFields::new().title("Legacy patch"),
                        ),
                        cx,
                    )
                    .expect("legacy patch targets canonical tool");

                assert_eq!(thread.entries().len(), 1);
                let (_, call) = thread.tool_call(&id).expect("same canonical tool");
                assert_eq!(call.id, id);
                assert!(Arc::ptr_eq(&call.id.0, &backing));
                assert_eq!(call.title.as_deref(), Some("Legacy patch"));
                assert_eq!(call.raw_input, Some(json!({"retain": true})));
                assert_eq!(call.content().len(), 1);
                assert_eq!(
                    call.content()
                        .first()
                        .and_then(ToolCallContent::markdown)
                        .expect("chunk markdown")
                        .read(cx)
                        .source(),
                    "chunk",
                );
            });

            let (request_id, response) = request_test_permission_with_id(&thread, id.clone(), cx);
            thread.update(cx, |thread, cx| {
                let permission_id = thread
                    .permission_request(request_id)
                    .and_then(PermissionRequest::legacy_tool_call_id)
                    .expect("legacy permission has canonical tool identity");
                assert_eq!(permission_id, &id);
                assert!(Arc::ptr_eq(&permission_id.0, &backing));
                thread.cancel_tool_call_authorization(&id, cx);
            });
            assert!(matches!(
                response.await,
                RequestPermissionOutcome::Cancelled
            ));
        }
    }

    #[test]
    fn test_legacy_tool_enum_translations_preserve_known_values() {
        for kind in [
            acp_v1::ToolKind::Read,
            acp_v1::ToolKind::Edit,
            acp_v1::ToolKind::Delete,
            acp_v1::ToolKind::Move,
            acp_v1::ToolKind::Search,
            acp_v1::ToolKind::Execute,
            acp_v1::ToolKind::Think,
            acp_v1::ToolKind::Fetch,
            acp_v1::ToolKind::SwitchMode,
            acp_v1::ToolKind::Other,
        ] {
            assert_eq!(
                serde_json::to_value(tool_kind_from_v1(kind).expect("known kind"))
                    .expect("v2 kind"),
                serde_json::to_value(kind).expect("v1 kind"),
            );
        }
        for status in [
            acp_v1::ToolCallStatus::Pending,
            acp_v1::ToolCallStatus::InProgress,
            acp_v1::ToolCallStatus::Completed,
            acp_v1::ToolCallStatus::Failed,
        ] {
            assert_eq!(
                serde_json::to_value(tool_status_from_v1(status).expect("known status"))
                    .expect("v2 status"),
                serde_json::to_value(status).expect("v1 status"),
            );
        }
    }

    #[gpui::test]
    async fn test_skipped_legacy_tool_enums_preserve_current_state(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let id = acp_v2::ToolCallId::new("kept");
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("kept")
                        .kind(acp_v2::ToolKind::Execute)
                        .status(acp_v2::ToolCallStatus::Completed),
                    cx,
                )
                .expect("known state");
            assert!(legacy_tool_field::<acp_v2::ToolKind>(None).is_undefined());
            assert!(legacy_tool_field::<acp_v2::ToolCallStatus>(None).is_undefined());
            thread
                .upsert_tool_call_inner(
                    acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new().title("updated title"),
                    ),
                    None,
                    cx,
                )
                .expect("no translated kind or status");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.reported_kind, Some(acp_v2::ToolKind::Execute));
            assert_eq!(
                call.reported_status,
                Some(acp_v2::ToolCallStatus::Completed)
            );
            assert_eq!(call.status(), ToolCallStatus::Completed);
            assert_eq!(call.local_status, None);
            assert_eq!(call.label.read(cx).source(), "updated title");
            thread
                .update_tool_call(
                    acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(id.0),
                        acp_v1::ToolCallUpdateFields::new().kind(acp_v1::ToolKind::Other),
                    ),
                    cx,
                )
                .expect("Other is a supported explicit value");
            assert_eq!(thread.entries().len(), 1);
            let languages = thread.project.read(cx).languages().clone();
            let empty = ToolCall::from_patch(
                "fallback".into(),
                ToolCallPatch::legacy(acp_v1::ToolCallUpdateFields::new(), None),
                languages,
                ToolTerminalResolver::registered(&thread.terminals),
                cx,
            )
            .expect("missing enums use display fallbacks");
            assert_eq!(empty.reported_kind, None);
            assert_eq!(empty.reported_status, None);
            assert_eq!(empty.kind(), &acp_v2::ToolKind::Other);
            assert_eq!(empty.status(), ToolCallStatus::Pending);
            let (_, call) = thread
                .tool_call(&acp_v2::ToolCallId::new("kept"))
                .expect("tool");
            assert_eq!(call.reported_kind, Some(acp_v2::ToolKind::Other));
            assert_eq!(call.status(), ToolCallStatus::Completed);
        });
    }

    #[gpui::test]
    async fn test_tool_patch_preserves_unknown_values_and_applies_field_clears(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let id = acp_v2::ToolCallId::new("patch");
            thread
                .upsert_wire_tool_call(acp_v2::ToolCallUpdate::new("patch"), cx)
                .expect("ID-only upsert");
            let (_, call) = thread.tool_call(&id).expect("new tool");
            assert_eq!(call.reported_kind, None);
            assert_eq!(call.reported_status, None);
            assert_eq!(call.status(), ToolCallStatus::Pending);
            assert_eq!(call.label.read(cx).source(), "Tool call");
            assert!(call.content().is_empty());

            thread
                .upsert_wire_tool_call(
                    serde_json::from_value(json!({
                        "toolCallId": "patch",
                        "title": "**Title**",
                        "name": "named",
                        "kind": "_future_kind",
                        "status": "_future_status",
                        "content": [{
                            "type": "content",
                            "content": {"type": "text", "text": "body", "_meta": {"block": true}},
                            "_meta": {"envelope": true}
                        }],
                        "rawInput": {"nested": [1, {"value": "input"}]},
                        "rawOutput": "fallback",
                        "_meta": {"opaque": {"nested": [1, 2]}}
                    }))
                    .expect("SDK patch"),
                    cx,
                )
                .expect("populated upsert");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(
                call.reported_kind,
                Some(acp_v2::ToolKind::Unknown("_future_kind".into()))
            );
            assert_eq!(
                call.reported_status,
                Some(acp_v2::ToolCallStatus::Other("_future_status".into()))
            );
            assert_eq!(call.status(), ToolCallStatus::Pending);
            assert_eq!(call.name.as_deref(), Some("named"));
            assert_eq!(
                serde_json::to_value(&call.meta).expect("metadata"),
                json!({"opaque": {"nested": [1, 2]}})
            );
            let [ToolCallContent::ContentBlock { block, meta }] = call.content() else {
                panic!("one protocol content block");
            };
            assert_eq!(
                serde_json::to_value(&block.source).expect("source"),
                json!({"type": "text", "text": "body", "_meta": {"block": true}})
            );
            assert_eq!(
                serde_json::to_value(meta).expect("envelope"),
                json!({"envelope": true})
            );
            let label = call.label.clone();
            let output = block.markdown().expect("body").clone();
            let input = call.raw_input_markdown.clone().expect("input");

            for patch in [
                json!({"toolCallId": "patch"}),
                json!({"toolCallId": "patch", "_meta": {}}),
            ] {
                thread
                    .upsert_wire_tool_call(serde_json::from_value(patch).expect("patch"), cx)
                    .expect("metadata-only update");
                let (_, call) = thread.tool_call(&id).expect("tool");
                assert_eq!(call.label, label);
                assert_eq!(call.content()[0].markdown(), Some(&output));
                assert_eq!(call.raw_input_markdown.as_ref(), Some(&input));
                assert_eq!(output.read(cx).source(), "body");
            }
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.meta,
                Some(acp_v2::Meta::new())
            );
            thread
                .upsert_wire_tool_call(acp_v2::ToolCallUpdate::new("patch").title("  "), cx)
                .expect("whitespace title");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.title.as_deref(), Some("  "));
            assert_eq!(call.label.read(cx).source(), "named");

            thread
                .upsert_wire_tool_call(
                    serde_json::from_value(json!({
                        "toolCallId": "patch",
                        "title": null, "name": null, "kind": null, "status": null,
                        "content": null, "rawInput": null, "_meta": null
                    }))
                    .expect("clear patch"),
                    cx,
                )
                .expect("clear optional fields");
            let (_, call) = thread.tool_call(&id).expect("tool survives clear");
            assert_eq!(call.title, None);
            assert_eq!(call.name, None);
            assert_eq!(call.tool_name, None);
            assert_eq!(call.reported_kind, None);
            assert_eq!(call.reported_status, None);
            assert_eq!(call.meta, None);
            assert_eq!(call.raw_input, None);
            assert!(call.raw_input_markdown.is_none());
            assert_eq!(call.raw_output, Some(json!("fallback")));
            assert_eq!(call.content()[0].to_markdown(cx), "fallback");
            let ToolCallContent::ContentBlock { block, .. } = &call.content()[0] else {
                panic!("derived raw-output fallback");
            };
            assert!(block.source.is_none());
            assert_eq!(thread.entries().len(), 1);
            thread
                .upsert_wire_tool_call(
                    serde_json::from_value(json!({"toolCallId": "patch", "rawOutput": null}))
                        .expect("raw clear"),
                    cx,
                )
                .expect("clear retained raw output");
            assert!(thread.tool_call(&id).expect("tool").1.content().is_empty());
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("patch")
                        .content(vec!["refilled".into()])
                        .status(acp_v2::ToolCallStatus::Completed),
                    cx,
                )
                .expect("refill existing record");
            let (index, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(index, 0);
            assert_eq!(call.content()[0].to_markdown(cx), "refilled");
            assert_eq!(call.status(), ToolCallStatus::Completed);
        });
    }

    #[gpui::test]
    async fn test_tool_content_chunk_ordering_and_targeting(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let events = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            let events = events.clone();
            cx.subscribe(&thread, move |_, event, _| match event {
                AcpThreadEvent::NewEntry => events.borrow_mut().push(None),
                AcpThreadEvent::EntryUpdated(index) => events.borrow_mut().push(Some(*index)),
                _ => {}
            })
        });
        thread.update(cx, |thread, cx| {
            let first_id = acp_v2::ToolCallId::new("first");
            let second_id = acp_v2::ToolCallId::new("first-extra");
            thread
                .append_tool_call_content_chunk(
                    acp_v2::ToolCallContentChunk::new("first", "original")
                        .meta(meta_with_tool_name("delivery_only")),
                    cx,
                )
                .expect("chunk creates a minimal tool");
            let (_, first) = thread.tool_call(&first_id).expect("first tool");
            assert_eq!(first.reported_kind, None);
            assert_eq!(first.reported_status, None);
            assert_eq!(first.meta, None);
            let original = first.content()[0]
                .markdown()
                .expect("original text")
                .clone();
            thread
                .append_tool_call_content_chunk(
                    acp_v2::ToolCallContentChunk::new("first-extra", "second"),
                    cx,
                )
                .expect("chunk for distinct ID");
            thread
                .append_tool_call_content_chunk(
                    acp_v2::ToolCallContentChunk::new("first", "historical"),
                    cx,
                )
                .expect("append to the older tool");
            let (index, first) = thread.tool_call(&first_id).expect("historical tool");
            assert_eq!(index, 0);
            assert_eq!(first.content()[0].markdown(), Some(&original));
            assert_eq!(original.read(cx).source(), "original");
            assert_eq!(
                first
                    .content()
                    .iter()
                    .map(|item| item.to_markdown(cx))
                    .collect::<Vec<_>>(),
                ["original", "historical"]
            );
            let (index, second) = thread.tool_call(&second_id).expect("second tool");
            assert_eq!(index, 1);
            assert_eq!(second.content().len(), 1);
            assert_eq!(second.content()[0].to_markdown(cx), "second");
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("first")
                        .content(vec!["replacement".into()])
                        .raw_output(json!("raw fallback")),
                    cx,
                )
                .expect("snapshot replaces chunks");
            thread
                .append_tool_call_content_chunk(
                    acp_v2::ToolCallContentChunk::new("first", "after replacement"),
                    cx,
                )
                .expect("append after replacement");
            assert_eq!(
                thread
                    .tool_call(&first_id)
                    .expect("tool")
                    .1
                    .content()
                    .iter()
                    .map(|item| item.to_markdown(cx))
                    .collect::<Vec<_>>(),
                ["replacement", "after replacement"]
            );
            thread
                .upsert_wire_tool_call(acp_v2::ToolCallUpdate::new("first").content(Vec::new()), cx)
                .expect("clear structured content");
            assert_eq!(
                thread.tool_call(&first_id).expect("tool").1.content()[0].to_markdown(cx),
                "raw fallback"
            );
            thread
                .append_tool_call_content_chunk(
                    acp_v2::ToolCallContentChunk::new("first", "after clear"),
                    cx,
                )
                .expect("append excludes the raw fallback");
            let (_, first) = thread.tool_call(&first_id).expect("tool");
            assert_eq!(first.content().len(), 1);
            assert_eq!(first.content()[0].to_markdown(cx), "after clear");
            assert_eq!(first.raw_output, Some(json!("raw fallback")));
            assert_eq!(thread.entries().len(), 2);
        });
        assert_eq!(
            *events.borrow(),
            [None, None, Some(0), Some(0), Some(0), Some(0), Some(0)]
        );
    }

    #[gpui::test]
    async fn test_tool_content_chunk_preserves_existing_state(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let id = acp_v2::ToolCallId::new("native");
        let tool_meta = meta_with_tool_name("tool_name");
        let (request_id, mut permission) = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization_with_id(
                    acp_v1::ToolCall::new(acp_v1::ToolCallId::new(id.0.clone()), "Original")
                        .status(acp_v1::ToolCallStatus::Completed)
                        .meta(tool_meta.clone())
                        .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        "allow",
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("legacy permission request");
        thread.update(cx, |thread, cx| {
            let buffer = cx.new(|cx| Buffer::local("native content", cx));
            let diff = cx.new(|cx| Diff::new(buffer, cx));
            thread
                .update_tool_call(
                    ToolCallUpdateDiff {
                        id: id.clone(),
                        diff: diff.clone(),
                    },
                    cx,
                )
                .expect("native diff ownership");
            thread.push_assistant_content_block("visible".into(), false, cx);
            thread.push_assistant_content_block(" buffered".into(), false, cx);
            let (_, target) = thread
                .streaming_content_target(None, false, false)
                .expect("assistant streaming target");
            thread
                .append_tool_call_content_chunk(
                    acp_v2::ToolCallContentChunk::new("native", "appended"),
                    cx,
                )
                .expect("historical native append");
            let (_, call) = thread.tool_call(&id).expect("native tool");
            assert_eq!(call.diffs().next(), Some(&diff));
            assert_eq!(call.content().len(), 2);
            assert_eq!(call.authorization_id(), Some(request_id));
            assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
            assert_eq!(
                call.reported_status,
                Some(acp_v2::ToolCallStatus::Completed)
            );
            assert_eq!(call.meta.as_ref(), Some(&tool_meta));
            assert!(thread.streaming_text_buffer.is_some());
            assert_eq!(target.markdown.read(cx).source(), "visible");
            thread
                .append_tool_call_content_chunk(
                    acp_v2::ToolCallContentChunk::new("new-boundary", "new row"),
                    cx,
                )
                .expect("new row flushes streaming text");
            assert!(thread.streaming_text_buffer.is_none());
            assert_eq!(target.markdown.read(cx).source(), "visible buffered");
        });
        assert!((&mut permission).now_or_never().is_none());
        thread.update(cx, |thread, cx| {
            thread.cancel_permission_request(request_id, cx)
        });
        assert!(matches!(
            permission.await,
            RequestPermissionOutcome::Cancelled
        ));
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_tool_patch_metadata_policy_preserves_legacy_hints_and_clears_v2_hints(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        thread.update(cx, |thread, cx| {
            let id = acp_v2::ToolCallId::new("metadata");
            let meta = acp_v1::Meta::from_iter([
                (TOOL_NAME_META_KEY.into(), json!("legacy_name")),
                (
                    SUBAGENT_SESSION_INFO_META_KEY.into(),
                    json!({"session_id": "child", "message_start_index": 0}),
                ),
                ("opaque".into(), json!({"keep": ["all", "values"]})),
            ]);
            thread
                .upsert_tool_call(
                    acp_v1::ToolCall::new(acp_v1::ToolCallId::new(id.0.clone()), "")
                        .meta(meta.clone()),
                    cx,
                )
                .expect("legacy creation");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.name, None);
            assert_eq!(call.tool_name.as_deref(), Some("legacy_name"));
            assert!(call.subagent_session_info.is_some());
            assert_eq!(call.meta.as_ref(), Some(&meta));

            let newer_meta = acp_v1::Meta::from_iter([("opaque".into(), json!({"new": true}))]);
            thread
                .update_tool_call(
                    acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new(),
                    )
                    .meta(newer_meta.clone()),
                    cx,
                )
                .expect("legacy metadata update");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.meta.as_ref(), Some(&newer_meta));
            assert_eq!(call.tool_name.as_deref(), Some("legacy_name"));
            assert!(call.subagent_session_info.is_some());
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("metadata").meta(acp_v2::Meta::new()),
                    cx,
                )
                .expect("authoritative metadata replacement");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.meta, Some(acp_v2::Meta::new()));
            assert_eq!(call.tool_name, None);
            assert!(call.subagent_session_info.is_none());
            assert_eq!(call.label.read(cx).source(), "Tool call");
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("metadata")
                        .name("first_class")
                        .meta(meta),
                    cx,
                )
                .expect("first-class name wins");
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.tool_name.as_deref(),
                Some("first_class")
            );
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("metadata").meta(None::<acp_v2::Meta>),
                    cx,
                )
                .expect("clear metadata without clearing name");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.meta, None);
            assert_eq!(call.name.as_deref(), Some("first_class"));
            assert_eq!(call.tool_name.as_deref(), Some("first_class"));
            assert!(call.subagent_session_info.is_none());
        });
    }

    #[gpui::test]
    async fn test_tool_reported_status_is_independent_of_local_authorization(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let id = acp_v2::ToolCallId::new("permission");
        let permission = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp_v1::ToolCall::new(acp_v1::ToolCallId::new(id.0.clone()), "Authorize")
                        .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        "allow",
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("request");
        thread.update(cx, |thread, cx| {
            for status in [json!("_future_status"), serde_json::Value::Null] {
                thread
                    .upsert_wire_tool_call(
                        serde_json::from_value(
                            json!({"toolCallId": "permission", "status": status}),
                        )
                        .expect("status patch"),
                        cx,
                    )
                    .expect("nonterminal status");
                let (_, call) = thread.tool_call(&id).expect("tool");
                assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
                let request_id = call.authorization_id().expect("pending request link");
                assert_eq!(
                    thread
                        .permission_request(request_id)
                        .expect("pending request")
                        .legacy_tool_call_id(),
                    Some(&id)
                );
                assert_eq!(
                    serde_json::to_value(&call.reported_status).expect("reported status"),
                    status
                );
            }
            thread.cancel_tool_call_authorization(&id, cx);
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.reported_status, None);
            assert_eq!(call.status(), ToolCallStatus::Canceled);
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("permission")
                        .status(None::<acp_v2::ToolCallStatus>),
                    cx,
                )
                .expect("clear only reported information");
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.status(),
                ToolCallStatus::Canceled
            );
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("permission")
                        .status(acp_v2::ToolCallStatus::InProgress),
                    cx,
                )
                .expect("new reported progress supersedes local outcome");
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.status(),
                ToolCallStatus::InProgress
            );
        });
        assert!(matches!(
            permission.await,
            RequestPermissionOutcome::Cancelled
        ));

        let permission = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new(),
                    ),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        "allow",
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .expect("second request");
        thread.update(cx, |thread, cx| {
            let original_label = thread.tool_call(&id).expect("tool").1.label.clone();
            let result = thread.upsert_wire_tool_call(
                serde_json::from_value(json!({
                    "toolCallId": "permission",
                    "status": "cancelled",
                    "title": "Cancelled command",
                    "content": [{"type": "terminal", "terminalId": "missing"}]
                }))
                .expect("cancelled patch"),
                cx,
            );
            result.expect("a missing v2 terminal creates a placeholder");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(
                call.reported_status,
                Some(acp_v2::ToolCallStatus::Cancelled)
            );
            assert_eq!(call.status(), ToolCallStatus::Canceled);
            assert!(call.authorization_id().is_none());
            assert!(thread.permission_request_for_tool(&id).is_none());
            assert_eq!(call.label, original_label);
            assert_eq!(call.label.read(cx).source(), "Cancelled command");
            assert!(call.terminals().next().is_some());
        });
        assert!(matches!(
            permission.await,
            RequestPermissionOutcome::Cancelled
        ));
    }

    #[gpui::test]
    async fn test_tool_authorization_preserves_legacy_continuation_without_rewriting_reported_status(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for (kind, initial, incoming, expected) in [
            (
                AuthorizationKind::PermissionGrant,
                acp_v1::ToolCallStatus::Pending,
                None,
                ToolCallStatus::InProgress,
            ),
            (
                AuthorizationKind::PermissionGrant,
                acp_v1::ToolCallStatus::InProgress,
                None,
                ToolCallStatus::InProgress,
            ),
            (
                AuthorizationKind::PermissionGrant,
                acp_v1::ToolCallStatus::Completed,
                None,
                ToolCallStatus::Completed,
            ),
            (
                AuthorizationKind::ActionChoice,
                acp_v1::ToolCallStatus::InProgress,
                None,
                ToolCallStatus::InProgress,
            ),
            (
                AuthorizationKind::ActionChoice,
                acp_v1::ToolCallStatus::Completed,
                None,
                ToolCallStatus::InProgress,
            ),
            (
                AuthorizationKind::PermissionGrant,
                acp_v1::ToolCallStatus::InProgress,
                Some(acp_v1::ToolCallStatus::Completed),
                ToolCallStatus::InProgress,
            ),
        ] {
            let thread = new_test_thread(cx).await;
            let id = acp_v2::ToolCallId::new("continuation");
            thread.update(cx, |thread, cx| {
                thread
                    .upsert_tool_call(
                        acp_v1::ToolCall::new(acp_v1::ToolCallId::new(id.0.clone()), "Original")
                            .status(initial),
                        cx,
                    )
                    .expect("initial tool");
            });
            let option_kind = if kind == AuthorizationKind::ActionChoice {
                acp_v2::PermissionOptionKind::RejectOnce
            } else {
                acp_v2::PermissionOptionKind::AllowOnce
            };
            let permission = thread
                .update(cx, |thread, cx| {
                    thread.request_tool_call_authorization(
                        acp_v1::ToolCallUpdate::new(
                            acp_v1::ToolCallId::new(id.0.clone()),
                            acp_v1::ToolCallUpdateFields::new().status(incoming),
                        ),
                        PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                            "selected",
                            "Choose",
                            option_kind.clone(),
                        )]),
                        kind,
                        cx,
                    )
                })
                .expect("explicit legacy authorization request");
            let reported =
                tool_status_from_v1(incoming.unwrap_or(initial)).expect("known test status");
            thread.update(cx, |thread, cx| {
                let (_, call) = thread.tool_call(&id).expect("waiting tool");
                assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
                assert_eq!(call.reported_status.as_ref(), Some(&reported));
                assert_eq!(call.permission_status(), tool_status_from_v1(initial));
                thread.authorize_tool_call(
                    id.clone(),
                    SelectedPermissionOutcome::new("selected".into(), option_kind),
                    cx,
                );
                let (_, call) = thread.tool_call(&id).expect("authorized tool");
                assert_eq!(call.status(), expected);
                assert_eq!(call.reported_status.as_ref(), Some(&reported));
                assert!(call.authorization_id().is_none());
                assert!(thread.permission_request_for_tool(&id).is_none());
            });
            assert!(
                matches!(permission.await, RequestPermissionOutcome::Selected(outcome) if outcome.option_id == "selected".into())
            );
            thread.update(cx, |thread, cx| {
                thread
                    .upsert_wire_tool_call(
                        acp_v2::ToolCallUpdate::new("continuation")
                            .status(None::<acp_v2::ToolCallStatus>),
                        cx,
                    )
                    .expect("clear reported state, not the local decision");
                let (_, call) = thread.tool_call(&id).expect("tool");
                assert_eq!(call.reported_status, None);
                assert_eq!(call.status(), expected);
                thread
                    .upsert_wire_tool_call(
                        acp_v2::ToolCallUpdate::new("continuation").status(reported),
                        cx,
                    )
                    .expect("even repeated reported state supersedes a local outcome");
                assert_eq!(thread.tool_call(&id).expect("tool").1.local_status, None);
            });
        }
    }

    #[test]
    fn test_tool_location_conversions_preserve_raw_paths_and_item_metadata() {
        let meta = acp_v2::Meta::from_iter([("location".into(), json!({"nested": [1, 2]}))]);
        for path in [
            PathBuf::from("src//./../file.rs"),
            PathBuf::from(path!("/project//./../file.rs")),
        ] {
            let location = ToolCallLocation::from(
                acp_v1::ToolCallLocation::new(path.clone())
                    .line(7)
                    .meta(meta.clone()),
            );
            assert_eq!(location.path.as_os_str(), path.as_os_str());
            assert_eq!(location.line, Some(7));
            assert_eq!(location.meta.as_ref(), Some(&meta));
        }

        let path = PathBuf::from(path!("/project//./../file.rs"));
        let location = ToolCallLocation::from(
            acp_v2::ToolCallLocation::new(acp_v2::AbsolutePath::new(path.clone()))
                .line(7)
                .meta(meta.clone()),
        );
        assert_eq!(location.path.as_os_str(), path.as_os_str());
        assert_eq!(location.line, Some(7));
        assert_eq!(location.meta.as_ref(), Some(&meta));

        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;

            let path = PathBuf::from(std::ffi::OsString::from_vec(b"src//./\xff.rs".to_vec()));
            let location = ToolCallLocation::from(
                acp_v1::ToolCallLocation::new(path.clone())
                    .line(7)
                    .meta(meta.clone()),
            );
            assert_eq!(location.path.as_os_str(), path.as_os_str());
            assert_eq!(location.line, Some(7));
            assert_eq!(location.meta.as_ref(), Some(&meta));
        }
    }

    #[test]
    fn test_tool_location_equality_preserves_raw_path_spelling() {
        let location = ToolCallLocation {
            path: PathBuf::from("src/file.rs"),
            line: Some(7),
            meta: Some(acp_v2::Meta::from_iter([("location".into(), json!(true))])),
        };
        assert_eq!(location, location.clone());
        for spelling in ["src//file.rs", "src/./file.rs", "src/../src/file.rs"] {
            let changed = ToolCallLocation {
                path: PathBuf::from(spelling),
                ..location.clone()
            };
            assert_ne!(location, changed, "{spelling}");
        }
        assert_ne!(
            location,
            ToolCallLocation {
                line: None,
                ..location.clone()
            }
        );
        assert_ne!(
            location,
            ToolCallLocation {
                meta: None,
                ..location.clone()
            }
        );
    }

    #[gpui::test]
    async fn test_tool_locations_preserve_item_metadata_and_patch_semantics(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"file.rs": "first\nsecond\n"}))
            .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/project"))]), cx)
            })
            .await
            .expect("session");
        let id = acp_v2::ToolCallId::new("location-migration");
        let relative = ToolCallLocation {
            path: PathBuf::from("src//./file.rs"),
            line: Some(1),
            meta: Some(acp_v2::Meta::from_iter([("location".into(), json!(true))])),
        };
        thread.update(cx, |thread, cx| {
            thread
                .upsert_tool_call(
                    acp_v1::ToolCall::new("location-migration", "Read").locations(vec![
                        acp_v1::ToolCallLocation::new(relative.path.clone())
                            .line(relative.line)
                            .meta(relative.meta.clone()),
                    ]),
                    cx,
                )
                .expect("native relative location");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.locations, vec![relative.clone()]);
            thread
                .upsert_wire_tool_call(acp_v2::ToolCallUpdate::new("location-migration"), cx)
                .expect("omitted locations");
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.locations,
                vec![relative.clone()]
            );
        });
        cx.run_until_parked();

        let absolute = ToolCallLocation {
            path: PathBuf::from(path!("/project/file.rs")),
            ..relative.clone()
        };
        thread.update(cx, |thread, cx| {
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("location-migration").locations(vec![
                        acp_v2::ToolCallLocation::new(acp_v2::AbsolutePath::new(
                            absolute.path.clone(),
                        ))
                        .line(absolute.line)
                        .meta(absolute.meta.clone()),
                    ]),
                    cx,
                )
                .expect("v2 replaces native relative location");
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.locations,
                vec![absolute.clone()]
            );
        });
        cx.run_until_parked();

        let respelled = ToolCallLocation {
            path: PathBuf::from(path!("/project//./file.rs")),
            ..absolute.clone()
        };
        assert_eq!(absolute.path, respelled.path);
        thread.update(cx, |thread, cx| {
            let (index, call) = thread.tool_call(&id).expect("tool");
            let (location, _) = thread.entries[index]
                .location(0)
                .expect("resolved location");
            assert_eq!(location, absolute);
            let resolved_locations = call.resolved_locations.clone();
            thread
                .upsert_wire_tool_call(acp_v2::ToolCallUpdate::new("location-migration"), cx)
                .expect("omission preserves resolved projections");
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.resolved_locations,
                resolved_locations
            );
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("location-migration").locations(vec![
                        acp_v2::ToolCallLocation::new(acp_v2::AbsolutePath::new(
                            respelled.path.clone(),
                        ))
                        .line(respelled.line)
                        .meta(respelled.meta.clone()),
                    ]),
                    cx,
                )
                .expect("component-equivalent spelling replaces canonical location");
            let (index, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.locations, vec![respelled.clone()]);
            assert!(call.resolved_locations.is_empty());
            assert!(thread.entries[index].location(0).is_none());
        });
        cx.run_until_parked();
        thread.update(cx, |thread, cx| {
            let (index, _) = thread.tool_call(&id).expect("tool");
            let (location, _) = thread.entries[index]
                .location(0)
                .expect("refreshed projection");
            assert_eq!(location, respelled);
            thread
                .update_tool_call(
                    acp_v1::ToolCallUpdate::new(
                        "location-migration",
                        acp_v1::ToolCallUpdateFields::new().locations(vec![
                            acp_v1::ToolCallLocation::new(relative.path.clone())
                                .line(relative.line)
                                .meta(relative.meta.clone()),
                        ]),
                    ),
                    cx,
                )
                .expect("v1 replaces absolute location with relative location");
            let (index, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.locations, vec![relative.clone()]);
            assert!(call.resolved_locations.is_empty());
            assert!(thread.entries[index].location(0).is_none());
        });
        cx.run_until_parked();
        thread.update(cx, |thread, cx| {
            thread
                .upsert_wire_tool_call(
                    acp_v2::ToolCallUpdate::new("location-migration")
                        .locations(None::<Vec<acp_v2::ToolCallLocation>>),
                    cx,
                )
                .expect("explicit null clears locations");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert!(call.locations.is_empty());
            assert!(call.resolved_locations.is_empty());
        });
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            let (index, call) = thread.tool_call(&id).expect("tool");
            assert!(call.locations.is_empty());
            assert!(call.resolved_locations.is_empty());
            assert!(thread.entries[index].location(0).is_none());
        });
    }

    #[gpui::test]
    async fn test_tool_location_clear_ignores_pending_resolution(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"file.rs": "first\nsecond\n"}))
            .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/project"))]), cx)
            })
            .await
            .expect("session");
        let id = acp_v2::ToolCallId::new("locations");
        thread.update(cx, |thread, cx| {
            thread.upsert_wire_tool_call(
                serde_json::from_value(json!({
                    "toolCallId": "locations",
                    "locations": [{"path": path!("/project/file.rs"), "line": 2, "_meta": {"location": true}}]
                })).expect("location patch"),
                cx,
            ).expect("new locations");
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.locations[0].path, PathBuf::from(path!("/project/file.rs")));
            assert_eq!(call.locations[0].line, Some(2));
            assert_eq!(call.locations[0].meta, Some(acp_v1::Meta::from_iter([("location".into(), json!(true))])));
            thread.upsert_wire_tool_call(
                serde_json::from_value(json!({"toolCallId": "locations", "locations": null})).expect("clear locations"),
                cx,
            ).expect("clear before the previous lookup finishes");
        });
        cx.run_until_parked();
        thread.read_with(cx, |thread, cx| {
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert!(call.locations.is_empty());
            assert!(call.resolved_locations.is_empty());
            assert!(thread.project.read(cx).agent_location().is_none());
        });
    }

    #[gpui::test]
    fn test_tool_call_label_fallback(cx: &mut TestAppContext) {
        use markdown::parser::{MarkdownEvent, MarkdownTag};

        init_test(cx);
        let languages =
            cx.update(|cx| Arc::new(LanguageRegistry::test(cx.background_executor().clone())));
        for (call, expected, has_strong_text) in [
            (
                acp_v1::ToolCall::new("tool", "Reading **file**").name("read_file"),
                "Reading **file**",
                true,
            ),
            (acp_v1::ToolCall::new("tool", ""), "Tool call", false),
            (
                acp_v1::ToolCall::new("tool", "\n\t ").name(" \t"),
                "Tool call",
                false,
            ),
            (
                acp_v1::ToolCall::new("tool", "").name("**mcp__tool**"),
                "**mcp__tool**",
                false,
            ),
            (
                acp_v1::ToolCall::new("tool", "\n\t ")
                    .name("**mcp__tool**")
                    .kind(acp_v1::ToolKind::Edit),
                "**mcp__tool**",
                false,
            ),
            (
                acp_v1::ToolCall::new("tool", "")
                    .name("**mcp__tool**")
                    .kind(acp_v1::ToolKind::Execute),
                "**mcp__tool**",
                false,
            ),
            (
                acp_v1::ToolCall::new("tool", "").meta(meta_with_tool_name("legacy_tool")),
                "legacy_tool",
                false,
            ),
        ] {
            let call = cx.update(|cx| {
                ToolCall::from_acp(
                    call,
                    Some(ToolCallStatus::Pending),
                    languages.clone(),
                    &HashMap::default(),
                    cx,
                )
                .expect("tool call should convert")
            });
            cx.run_until_parked();
            cx.read(|cx| {
                let label = call.label.read(cx);
                assert_eq!(label.source().as_ref(), expected);
                assert_eq!(
                    label.parsed_markdown().events().iter().any(|(_, event)| {
                        matches!(event, MarkdownEvent::Start(MarkdownTag::Strong))
                    }),
                    has_strong_text
                );
                if call.title.is_none() {
                    assert_eq!(
                        call.to_markdown(cx),
                        format!(
                            "**Tool Call: {}**\nStatus: Pending\n\n",
                            MarkdownEscaped(expected)
                        )
                    );
                }
            });
        }
    }

    #[gpui::test]
    fn test_tool_call_label_updates_preserve_titles(cx: &mut TestAppContext) {
        use markdown::parser::{MarkdownEvent, MarkdownTag};

        init_test(cx);
        let languages =
            cx.update(|cx| Arc::new(LanguageRegistry::test(cx.background_executor().clone())));
        let mut call = cx.update(|cx| {
            ToolCall::from_acp(
                acp_v1::ToolCall::new("tool", ""),
                Some(ToolCallStatus::Pending),
                languages.clone(),
                &HashMap::default(),
                cx,
            )
            .expect("tool call should convert")
        });
        for (update, expected, has_strong_text) in [
            (
                acp_v1::ToolCallUpdateFields::new().name("**tool_name**"),
                "**tool_name**",
                false,
            ),
            (
                acp_v1::ToolCallUpdateFields::new().title("**Readable title**"),
                "**Readable title**",
                true,
            ),
            (
                acp_v1::ToolCallUpdateFields::new().name("renamed_tool"),
                "**Readable title**",
                true,
            ),
            (
                acp_v1::ToolCallUpdateFields::new().kind(acp_v1::ToolKind::Execute),
                "**Readable title**",
                false,
            ),
            (
                acp_v1::ToolCallUpdateFields::new().kind(acp_v1::ToolKind::Read),
                "**Readable title**",
                true,
            ),
            (
                acp_v1::ToolCallUpdateFields::new().title("\n\t "),
                "renamed_tool",
                false,
            ),
            (
                acp_v1::ToolCallUpdateFields::new().name(" \t"),
                "Tool call",
                false,
            ),
            (
                acp_v1::ToolCallUpdateFields::new()
                    .title("**same_name**")
                    .name("**same_name**"),
                "**same_name**",
                true,
            ),
            (
                acp_v1::ToolCallUpdateFields::new().name("different_name"),
                "**same_name**",
                true,
            ),
        ] {
            cx.update(|cx| {
                call.apply_patch(
                    ToolCallPatch::legacy(update, None),
                    languages.clone(),
                    ToolTerminalResolver::registered(&HashMap::default()),
                    cx,
                )
                .expect("tool label update should apply");
            });
            cx.run_until_parked();
            cx.read(|cx| {
                let label = call.label.read(cx);
                assert_eq!(label.source().as_ref(), expected);
                assert_eq!(
                    label.parsed_markdown().events().iter().any(|(_, event)| {
                        matches!(event, MarkdownEvent::Start(MarkdownTag::Strong))
                    }),
                    has_strong_text
                );
            });
        }
    }

    #[gpui::test]
    fn test_tool_call_raw_output_creation_and_updates_export_latest_content(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let languages =
            cx.update(|cx| Arc::new(LanguageRegistry::test(cx.background_executor().clone())));
        let mut created_with_raw_output = cx.update(|cx| {
            ToolCall::from_acp(
                acp_v1::ToolCall::new("created", "Tool").raw_output(serde_json::json!("first")),
                Some(ToolCallStatus::Pending),
                languages.clone(),
                &HashMap::default(),
                cx,
            )
            .expect("raw-only tool call should convert")
        });
        let mut updated_with_raw_output = cx.update(|cx| {
            ToolCall::from_acp(
                acp_v1::ToolCall::new("updated", "Tool"),
                Some(ToolCallStatus::Pending),
                languages.clone(),
                &HashMap::default(),
                cx,
            )
            .expect("empty tool call should convert")
        });

        let created_export = cx.read(|cx| created_with_raw_output.to_markdown(cx));
        cx.update(|cx| {
            updated_with_raw_output
                .apply_patch(
                    ToolCallPatch::legacy(
                        acp_v1::ToolCallUpdateFields::new().raw_output(serde_json::json!("first")),
                        None,
                    ),
                    languages.clone(),
                    ToolTerminalResolver::registered(&HashMap::default()),
                    cx,
                )
                .expect("first raw output update should apply");
        });
        let first_update_export = cx.read(|cx| updated_with_raw_output.to_markdown(cx));
        let Some(ToolCallContent::ContentBlock { block, .. }) =
            updated_with_raw_output.content().first()
        else {
            panic!("expected raw output content");
        };
        let raw_markdown = block
            .markdown()
            .expect("raw output should be Markdown")
            .clone();
        cx.update(|cx| {
            created_with_raw_output
                .apply_patch(
                    ToolCallPatch::legacy(
                        acp_v1::ToolCallUpdateFields::new().raw_output(serde_json::json!("second")),
                        None,
                    ),
                    languages.clone(),
                    ToolTerminalResolver::registered(&HashMap::default()),
                    cx,
                )
                .expect("second raw output update should apply");
            updated_with_raw_output
                .apply_patch(
                    ToolCallPatch::legacy(
                        acp_v1::ToolCallUpdateFields::new().raw_output(serde_json::json!("second")),
                        None,
                    ),
                    languages.clone(),
                    ToolTerminalResolver::registered(&HashMap::default()),
                    cx,
                )
                .expect("second raw output update should apply");
        });
        let latest_exports = cx.read(|cx| {
            assert_eq!(raw_markdown.read(cx).source(), "second");
            (
                created_with_raw_output.to_markdown(cx),
                updated_with_raw_output.to_markdown(cx),
            )
        });
        assert_eq!(
            (created_export, first_update_export, latest_exports),
            (
                "**Tool Call: Tool**\nStatus: Pending\n\nfirst\n\n".to_string(),
                "**Tool Call: Tool**\nStatus: Pending\n\nfirst\n\n".to_string(),
                (
                    "**Tool Call: Tool**\nStatus: Pending\n\nsecond\n\n".to_string(),
                    "**Tool Call: Tool**\nStatus: Pending\n\nsecond\n\n".to_string(),
                ),
            )
        );
    }

    #[gpui::test]
    fn test_tool_call_clearing_structured_content_restores_retained_raw_output(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let languages =
            cx.update(|cx| Arc::new(LanguageRegistry::test(cx.background_executor().clone())));
        let mut call = cx.update(|cx| {
            ToolCall::from_acp(
                acp_v1::ToolCall::new("tool", "Tool")
                    .content(vec!["structured".into()])
                    .raw_output(serde_json::json!("raw")),
                Some(ToolCallStatus::Pending),
                languages.clone(),
                &HashMap::default(),
                cx,
            )
            .expect("tool call should convert")
        });
        cx.read(|cx| {
            assert_eq!(
                call.to_markdown(cx),
                "**Tool Call: Tool**\nStatus: Pending\n\nstructured\n\n"
            );
        });
        cx.update(|cx| {
            call.apply_patch(
                ToolCallPatch::legacy(
                    acp_v1::ToolCallUpdateFields::new().raw_output(serde_json::json!("new raw")),
                    None,
                ),
                languages.clone(),
                ToolTerminalResolver::registered(&HashMap::default()),
                cx,
            )
            .expect("raw output should update without replacing structured content");
            assert_eq!(
                call.to_markdown(cx),
                "**Tool Call: Tool**\nStatus: Pending\n\nstructured\n\n"
            );
        });
        cx.update(|cx| {
            call.apply_patch(
                ToolCallPatch::legacy(acp_v1::ToolCallUpdateFields::new().content(vec![]), None),
                languages.clone(),
                ToolTerminalResolver::registered(&HashMap::default()),
                cx,
            )
            .expect("clearing structured content should apply");
        });
        cx.read(|cx| {
            assert_eq!(call.raw_output, Some(serde_json::json!("new raw")));
            assert_eq!(
                call.to_markdown(cx),
                "**Tool Call: Tool**\nStatus: Pending\n\nnew raw\n\n"
            );
        });
        cx.update(|cx| {
            call.apply_patch(
                ToolCallPatch::legacy(
                    acp_v1::ToolCallUpdateFields::new().content(vec!["replacement".into()]),
                    None,
                ),
                languages,
                ToolTerminalResolver::registered(&HashMap::default()),
                cx,
            )
            .expect("structured content should replace the raw fallback");
            assert_eq!(
                call.to_markdown(cx),
                "**Tool Call: Tool**\nStatus: Pending\n\nreplacement\n\n"
            );
        });
    }

    #[gpui::test]
    async fn test_tool_call_name_precedence_and_updates(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("failed to create ACP thread");

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCall(
                        acp_v1::ToolCall::new("tool-call", "Tool call")
                            .name("first_class")
                            .meta(meta_with_tool_name("legacy")),
                    ),
                    cx,
                )
            })
            .expect("failed to create first-class named tool call");

        thread.read_with(cx, |thread, _| {
            let Some(AgentThreadEntry::ToolCall(tool_call)) = thread.entries.last() else {
                unreachable!("tool call update must create a tool call entry");
            };
            assert_eq!(tool_call.tool_name.as_deref(), Some("first_class"));
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCall(
                        acp_v1::ToolCall::new("legacy-tool-call", "Legacy tool call")
                            .meta(meta_with_tool_name("legacy")),
                    ),
                    cx,
                )
            })
            .expect("failed to create legacy named tool call");

        thread.read_with(cx, |thread, _| {
            let Some(AgentThreadEntry::ToolCall(tool_call)) = thread.entries.last() else {
                unreachable!("tool call update must create a tool call entry");
            };
            assert_eq!(tool_call.tool_name.as_deref(), Some("legacy"));
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(
                        acp_v1::ToolCallUpdate::new(
                            "tool-call",
                            acp_v1::ToolCallUpdateFields::new(),
                        )
                        .meta(meta_with_tool_name("legacy_update")),
                    ),
                    cx,
                )
            })
            .expect("failed to apply stale legacy tool-call name update");

        thread.read_with(cx, |thread, _| {
            let Some(tool_call) = thread.entries.iter().find_map(|entry| match entry {
                AgentThreadEntry::ToolCall(tool_call)
                    if tool_call.id == acp_v2::ToolCallId::new("tool-call") =>
                {
                    Some(tool_call)
                }
                _ => None,
            }) else {
                unreachable!("tool call update must create a tool call entry");
            };
            assert_eq!(tool_call.tool_name.as_deref(), Some("first_class"));
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(
                        acp_v1::ToolCallUpdate::new(
                            "legacy-tool-call",
                            acp_v1::ToolCallUpdateFields::new().name("first_class_update"),
                        )
                        .meta(meta_with_tool_name("ignored_legacy_update")),
                    ),
                    cx,
                )
            })
            .expect("failed to apply first-class tool-call name update");

        thread.read_with(cx, |thread, _| {
            let Some(tool_call) = thread.entries.iter().find_map(|entry| match entry {
                AgentThreadEntry::ToolCall(tool_call)
                    if tool_call.id == acp_v2::ToolCallId::new("legacy-tool-call") =>
                {
                    Some(tool_call)
                }
                _ => None,
            }) else {
                unreachable!("tool call update must create a tool call entry");
            };
            assert_eq!(tool_call.tool_name.as_deref(), Some("first_class_update"));
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCall(acp_v1::ToolCall::new(
                        "late-legacy-tool-call",
                        "Late legacy tool call",
                    )),
                    cx,
                )
            })
            .expect("failed to create unnamed tool call");
        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(
                        acp_v1::ToolCallUpdate::new(
                            "late-legacy-tool-call",
                            acp_v1::ToolCallUpdateFields::new(),
                        )
                        .meta(meta_with_tool_name("late_legacy")),
                    ),
                    cx,
                )
            })
            .expect("failed to apply late legacy tool-call name");

        thread.read_with(cx, |thread, _| {
            let Some(AgentThreadEntry::ToolCall(tool_call)) = thread.entries.last() else {
                unreachable!("tool call update must preserve the tool call entry");
            };
            assert_eq!(tool_call.tool_name.as_deref(), Some("late_legacy"));
        });
    }

    #[gpui::test]
    fn test_context_compaction_exports_status(cx: &mut App) {
        for (status, expected_label) in [
            (acp_v2::CompactionStatus::InProgress, "In Progress"),
            (acp_v2::CompactionStatus::Completed, "Completed"),
            (acp_v2::CompactionStatus::Failed, "Failed"),
            (acp_v2::CompactionStatus::Cancelled, "Canceled"),
            (
                acp_v2::CompactionStatus::Other("interrupted".into()),
                "interrupted",
            ),
        ] {
            let entry = AgentThreadEntry::ContextCompaction(ContextCompaction {
                id: ContextCompactionId("compaction".into()),
                status: status.into(),
                error: None,
                summary: MessageContent::default(),
                meta: None,
            });
            assert_eq!(
                entry.to_markdown(cx),
                format!("## Context Compaction ({expected_label})\n\n")
            );
        }
    }

    #[gpui::test]
    async fn test_tool_call_location_resolves_external_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/tmp/skills/test-skill"),
            json!({ "SKILL.md": "skill body" }),
        )
        .await;
        let project = Project::test(fs, [], cx).await;
        // Keeps the buffer alive: `shared_buffers` holds only weak handles,
        // so it would drop as soon as resolution finishes.
        let external_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/tmp/skills/test-skill/SKILL.md"), cx)
            })
            .await
            .unwrap();
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/project"))]), cx)
            })
            .await
            .unwrap();

        let skill_path = std::path::PathBuf::from(path!("/tmp/skills/test-skill/SKILL.md"));
        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCall(
                        acp_v1::ToolCall::new("write_file", "Write SKILL.md")
                            .kind(acp_v1::ToolKind::Edit)
                            .status(acp_v1::ToolCallStatus::Completed)
                            .locations(vec![acp_v1::ToolCallLocation::new(skill_path.clone())]),
                    ),
                    cx,
                )
            })
            .unwrap();

        cx.run_until_parked();

        thread.read_with(cx, |thread, cx| {
            let (tool_call_location, agent_location) = thread.entries[0]
                .location(0)
                .expect("external tool-call location should resolve");
            assert_eq!(tool_call_location.path, skill_path);

            let buffer = agent_location
                .buffer
                .upgrade()
                .expect("resolved location should reference the opened buffer");
            assert_eq!(buffer.entity_id(), external_buffer.entity_id());
            assert_eq!(buffer.read(cx).text(), "skill body");
        });
    }

    #[gpui::test]
    fn test_tool_patch_reuses_text_and_diff_views_but_updates_changed_paths(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            let languages = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let terminals = HashMap::default();
            let content = vec![
                "output".into(),
                acp_v1::ToolCallContent::Diff(
                    acp_v1::Diff::new("first.rs", "new text").old_text("old text"),
                ),
            ];
            let mut call = ToolCall::from_acp(
                acp_v1::ToolCall::new("tool", "Read")
                    .content(content.clone())
                    .raw_input(json!("input"))
                    .raw_output(json!("raw fallback")),
                Some(ToolCallStatus::InProgress),
                languages.clone(),
                &terminals,
                cx,
            )
            .expect("tool");
            let [
                ToolCallContent::ContentBlock { block, .. },
                ToolCallContent::LegacyDiff { diff, .. },
            ] = call.content()
            else {
                panic!("text and diff");
            };
            let output = block.markdown().expect("output").clone();
            let diff = diff.clone();
            let input = call.raw_input_markdown.clone().expect("input");
            for text in ["input", "input appended", "replacement", ""] {
                call.apply_patch(
                    ToolCallPatch::legacy(
                        acp_v1::ToolCallUpdateFields::new()
                            .content(content.clone())
                            .raw_input(json!(text)),
                        None,
                    ),
                    languages.clone(),
                    ToolTerminalResolver::registered(&terminals),
                    cx,
                )
                .expect("update snapshots");
                assert_eq!(call.raw_input_markdown.as_ref(), Some(&input));
                assert_eq!(input.read(cx).source(), text);
                assert_eq!(call.diffs().next(), Some(&diff));
                let ToolCallContent::ContentBlock { block, .. } = &call.content()[0] else {
                    panic!("text content");
                };
                assert_eq!(block.markdown(), Some(&output));
            }
            call.apply_patch(
                ToolCallPatch::legacy(
                    acp_v1::ToolCallUpdateFields::new().content(vec![
                        "output".into(),
                        acp_v1::ToolCallContent::Diff(
                            acp_v1::Diff::new("second.rs", "new text").old_text("old text"),
                        ),
                    ]),
                    None,
                ),
                languages.clone(),
                ToolTerminalResolver::registered(&terminals),
                cx,
            )
            .expect("same diff text at another path");
            let changed = call.diffs().next().expect("new diff");
            assert_ne!(changed, &diff);
            assert_eq!(changed.read(cx).file_path(cx).as_deref(), Some("second.rs"));
            assert_eq!(diff.read(cx).file_path(cx).as_deref(), Some("first.rs"));

            call.apply_patch(
                ToolCallPatch::legacy(
                    acp_v1::ToolCallUpdateFields::new()
                        .content(Vec::new())
                        .raw_input(serde_json::Value::Null),
                    None,
                ),
                languages.clone(),
                ToolTerminalResolver::registered(&terminals),
                cx,
            )
            .expect("clear structured content and render a typed null input");
            assert!(call.raw_input_markdown.is_none());
            assert_eq!(call.raw_input, Some(serde_json::Value::Null));
            assert_eq!(call.content().len(), 1);
            assert_eq!(call.content()[0].to_markdown(cx), "raw fallback");
            assert!(
                call.apply_patch(
                    ToolCallPatch::legacy(
                        acp_v1::ToolCallUpdateFields::new().content(vec![
                            "partial output".into(),
                            acp_v1::ToolCallContent::Terminal(acp_v1::Terminal::new("missing")),
                        ]),
                        None,
                    ),
                    languages,
                    ToolTerminalResolver::registered(&terminals),
                    cx,
                )
                .is_err()
            );
            assert!(call.structured_content.is_empty());
            assert_eq!(call.content().len(), 1);
            assert_eq!(call.content()[0].to_markdown(cx), "raw fallback");
        });
    }

    #[gpui::test]
    async fn test_tool_patch_retries_creation_and_does_not_relabel_terminal_on_error(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let id = acp_v2::ToolCallId::new("tool");
        let terminal_id = acp_v1::TerminalId::new("terminal");
        let initial = acp_v1::ToolCall::new(acp_v1::ToolCallId::new(id.0.clone()), "Tool caption")
            .kind(acp_v1::ToolKind::Execute)
            .content(vec![acp_v1::ToolCallContent::Terminal(
                acp_v1::Terminal::new(terminal_id.clone()),
            )]);
        thread.update(cx, |thread, cx| {
            assert!(thread.upsert_tool_call(initial.clone(), cx).is_err());
            assert!(thread.entries().is_empty());
        });
        let lower = cx.new(|cx| {
            ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .subscribe(cx)
        });
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "original command".into(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower,
                },
                cx,
            );
            thread
                .upsert_tool_call(initial, cx)
                .expect("retry after terminal arrives");
            let terminal = thread
                .terminal(acp_v2::TerminalId::new(terminal_id.0.clone()))
                .expect("terminal");
            let command = terminal.read(cx).command().clone();
            let (_, call) = thread.tool_call(&id).expect("tool");
            assert_eq!(call.terminals().next(), Some(&terminal));
            assert_eq!(thread.entries().len(), 1);
            assert!(
                thread
                    .update_tool_call(
                        acp_v1::ToolCallUpdate::new(
                            acp_v1::ToolCallId::new(id.0.clone()),
                            acp_v1::ToolCallUpdateFields::new()
                                .title("changed command")
                                .content(vec![
                                    acp_v1::ToolCallContent::Terminal(acp_v1::Terminal::new(
                                        terminal_id.clone()
                                    )),
                                    acp_v1::ToolCallContent::Terminal(acp_v1::Terminal::new(
                                        "missing"
                                    )),
                                ]),
                        ),
                        cx,
                    )
                    .is_err()
            );
            assert_eq!(command.read(cx).source(), "```\noriginal command\n```");
            let (_, call) = thread.tool_call(&id).expect("unchanged tool");
            assert_eq!(call.label.read(cx).source(), "Tool caption");
            assert_eq!(call.terminals().next(), Some(&terminal));
            assert_eq!(call.content().len(), 1);
            thread
                .update_tool_call(
                    acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new()
                            .title("changed command")
                            .content(vec![acp_v1::ToolCallContent::Terminal(
                                acp_v1::Terminal::new(terminal_id),
                            )]),
                    ),
                    cx,
                )
                .expect("valid terminal update");
            assert_eq!(command.read(cx).source(), "```\nchanged command\n```");
            assert_eq!(
                thread.tool_call(&id).expect("tool").1.terminals().next(),
                Some(&terminal)
            );
        });
    }

    #[gpui::test]
    async fn test_failed_tool_patch_preserves_presentation_but_applies_status(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for (upsert, status) in [
            (false, acp_v1::ToolCallStatus::InProgress),
            (false, acp_v1::ToolCallStatus::Completed),
            (true, acp_v1::ToolCallStatus::InProgress),
            (true, acp_v1::ToolCallStatus::Completed),
        ] {
            let thread = new_test_thread(cx).await;
            let id = acp_v2::ToolCallId::new("atomic-tool");
            let (request_id, permission) = thread
                .update(cx, |thread, cx| {
                    thread.request_tool_call_authorization_with_id(
                        acp_v1::ToolCall::new(
                            acp_v1::ToolCallId::new(id.0.clone()),
                            "Original title",
                        )
                        .name("original-tool")
                        .kind(acp_v1::ToolKind::Read)
                        .content(vec!["original output".into()])
                        .raw_input(json!({"original": true}))
                        .raw_output(json!("original raw"))
                        .into(),
                        PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                            "allow",
                            "Allow",
                            acp_v2::PermissionOptionKind::AllowOnce,
                        )]),
                        AuthorizationKind::PermissionGrant,
                        cx,
                    )
                })
                .expect("permission request");
            thread.update(cx, |thread, cx| {
                let (_, call) = thread.tool_call(&id).expect("original tool");
                let label = call.label.clone();
                let original_content = match &call.content()[0] {
                    ToolCallContent::ContentBlock { block, .. } => {
                        block.markdown().expect("text").clone()
                    }
                    _ => panic!("text content"),
                };
                let update = acp_v1::ToolCallUpdate::new(
                    acp_v1::ToolCallId::new(id.0.clone()),
                    acp_v1::ToolCallUpdateFields::new()
                        .title("Changed title")
                        .name("changed-tool")
                        .kind(acp_v1::ToolKind::Execute)
                        .status(status)
                        .content(vec![
                            "changed output".into(),
                            acp_v1::ToolCallContent::Terminal(acp_v1::Terminal::new("missing")),
                        ])
                        .raw_input(json!({"changed": true}))
                        .raw_output(json!("changed raw")),
                )
                .meta(acp_v1::Meta::from_iter([(
                    SUBAGENT_SESSION_INFO_META_KEY.into(),
                    json!({"session_id": "child", "message_start_index": 0}),
                )]));
                let result = if upsert {
                    thread
                        .upsert_tool_call_inner(update, Some(status.into()), cx)
                        .map_err(anyhow::Error::from)
                } else {
                    thread.update_tool_call(update, cx)
                };
                assert!(result.is_err(), "unknown terminal must fail conversion");
                let (_, call) = thread.tool_call(&id).expect("tool must remain");
                assert_eq!(call.permission_status(), tool_status_from_v1(status));
                assert_eq!(call.label, label);
                assert_eq!(call.label.read(cx).source(), "Original title");
                assert_eq!(call.kind(), &acp_v2::ToolKind::Read);
                assert_eq!(call.tool_name.as_deref(), Some("original-tool"));
                assert_eq!(call.raw_input, Some(json!({"original": true})));
                assert_eq!(call.raw_output, Some(json!("original raw")));
                assert!(call.subagent_session_info.is_none());
                assert_eq!(original_content.read(cx).source(), "original output");
                let [ToolCallContent::ContentBlock { block, .. }] = call.content() else {
                    panic!("original content must remain");
                };
                assert_eq!(block.markdown(), Some(&original_content));
                if status == acp_v1::ToolCallStatus::InProgress {
                    assert!(matches!(
                        call.status(),
                        ToolCallStatus::WaitingForConfirmation
                    ));
                    assert_eq!(call.authorization_id(), Some(request_id));
                    assert!(thread.permission_request(request_id).is_some());
                    thread.authorize_tool_call(
                        id.clone(),
                        SelectedPermissionOutcome::new(
                            "allow".into(),
                            acp_v2::PermissionOptionKind::AllowOnce,
                        ),
                        cx,
                    );
                } else {
                    assert!(matches!(call.status(), ToolCallStatus::Completed));
                    assert!(call.authorization_id().is_none());
                }
                assert!(thread.permission_request(request_id).is_none());
                assert!(thread.permission_request_for_tool(&id).is_none());
                assert_eq!(thread.pending_permission_requests().count(), 0);
            });
            if status == acp_v1::ToolCallStatus::InProgress {
                assert!(matches!(
                    permission.await,
                    RequestPermissionOutcome::Selected(outcome) if outcome.option_id == "allow".into()
                ));
            } else {
                assert!(matches!(
                    permission.await,
                    RequestPermissionOutcome::Cancelled
                ));
            }
            thread.update(cx, |thread, cx| {
                thread
                    .update_tool_call(
                        acp_v1::ToolCallUpdate::new(
                            acp_v1::ToolCallId::new(id.0.clone()),
                            acp_v1::ToolCallUpdateFields::new()
                                .status(acp_v1::ToolCallStatus::Completed)
                                .content(vec!["corrected output".into()]),
                        ),
                        cx,
                    )
                    .expect("valid correction after failed patch");
                let (_, call) = thread.tool_call(&id).expect("corrected tool");
                assert!(matches!(call.status(), ToolCallStatus::Completed));
                assert_eq!(call.content()[0].to_markdown(cx), "corrected output");
            });
        }
    }

    #[gpui::test]
    async fn test_duplicate_tool_call_update_preserves_open_permission_request_until_authorized(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let tool_call_id = acp_v2::ToolCallId::new("toolu_01duplicate");
        let allow_option_id = acp_v2::PermissionOptionId::new("allow");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        "Original title",
                    )
                    .kind(acp_v1::ToolKind::Execute)
                    .status(acp_v1::ToolCallStatus::Pending)
                    .content(vec!["original content".into()])
                    .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        allow_option_id.clone(),
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCall(
                        acp_v1::ToolCall::new(
                            acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                            "Updated title",
                        )
                        .kind(acp_v1::ToolKind::Execute)
                        .status(acp_v1::ToolCallStatus::Pending)
                        .content(vec!["updated content".into()]),
                    ),
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert_eq!(tool_call.label.read(cx).source(), "Updated title");
            assert!(matches!(
                tool_call.status(),
                ToolCallStatus::WaitingForConfirmation
            ));
            assert_eq!(tool_call.content().len(), 1);
            assert_eq!(tool_call.content()[0].to_markdown(cx), "updated content");
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new()
                            .status(acp_v1::ToolCallStatus::InProgress)
                            .title("Updated again")
                            .content(vec!["updated again".into()]),
                    )),
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert_eq!(tool_call.label.read(cx).source(), "Updated again");
            assert!(matches!(
                tool_call.status(),
                ToolCallStatus::WaitingForConfirmation
            ));
            assert_eq!(tool_call.content().len(), 1);
            assert_eq!(tool_call.content()[0].to_markdown(cx), "updated again");
        });

        let selected_outcome = SelectedPermissionOutcome::new(
            allow_option_id.clone(),
            acp_v2::PermissionOptionKind::AllowOnce,
        );
        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id.clone(), selected_outcome, cx);
        });

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(tool_call.status(), ToolCallStatus::InProgress));
        });

        match permission_task.await {
            RequestPermissionOutcome::Selected(outcome) => {
                assert_eq!(outcome.option_id, allow_option_id);
                assert_eq!(outcome.option_kind, acp_v2::PermissionOptionKind::AllowOnce);
            }
            RequestPermissionOutcome::Cancelled
            | RequestPermissionOutcome::InterruptedByFollowUp => {
                panic!("permission request should remain open after duplicate tool call update")
            }
        }

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new()
                            .status(acp_v1::ToolCallStatus::Completed)
                            .title("Completed")
                            .content(vec!["done".into()]),
                    )),
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert_eq!(tool_call.label.read(cx).source(), "Completed");
            assert!(matches!(tool_call.status(), ToolCallStatus::Completed));
            assert_eq!(tool_call.content().len(), 1);
            assert_eq!(tool_call.content()[0].to_markdown(cx), "done");
        });
    }

    #[gpui::test]
    async fn test_permission_request_tracks_agent_status_until_resolved(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let tool_call_id = acp_v2::ToolCallId::new("toolu_01auto_resolve");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        "Original title",
                    )
                    .kind(acp_v1::ToolKind::Execute)
                    .status(acp_v1::ToolCallStatus::Pending)
                    .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        acp_v2::PermissionOptionId::new("allow"),
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new()
                            .status(acp_v1::ToolCallStatus::InProgress),
                    )),
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert_eq!(tool_call.status(), ToolCallStatus::WaitingForConfirmation);
            assert_eq!(
                tool_call.permission_status(),
                Some(acp_v2::ToolCallStatus::InProgress)
            );
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_call_id.clone(),
                SelectedPermissionOutcome::new(
                    acp_v2::PermissionOptionId::new("allow"),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(tool_call.status(), ToolCallStatus::InProgress));
        });

        match permission_task.await {
            RequestPermissionOutcome::Selected(outcome) => {
                assert_eq!(outcome.option_id, acp_v2::PermissionOptionId::new("allow"));
                assert_eq!(outcome.option_kind, acp_v2::PermissionOptionKind::AllowOnce);
            }
            RequestPermissionOutcome::Cancelled
            | RequestPermissionOutcome::InterruptedByFollowUp => {
                panic!("resolved permission request should select an outcome")
            }
        }
    }

    #[gpui::test]
    async fn test_permission_request_sets_waiting_status_on_existing_tool_call(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let tool_call_id = acp_v2::ToolCallId::new("toolu_01existing_permission");
        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCall(
                        acp_v1::ToolCall::new(
                            acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                            "Running title",
                        )
                        .kind(acp_v1::ToolKind::Execute)
                        .status(acp_v1::ToolCallStatus::InProgress),
                    ),
                    cx,
                )
            })
            .unwrap();

        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        "Needs permission",
                    )
                    .kind(acp_v1::ToolKind::Execute)
                    .status(acp_v1::ToolCallStatus::Pending)
                    .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        acp_v2::PermissionOptionId::new("allow"),
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert_eq!(tool_call.label.read(cx).source(), "Needs permission");
            assert_eq!(tool_call.status(), ToolCallStatus::WaitingForConfirmation);
            assert_eq!(
                tool_call.permission_status(),
                Some(acp_v2::ToolCallStatus::InProgress)
            );
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_call_id.clone(),
                SelectedPermissionOutcome::new(
                    acp_v2::PermissionOptionId::new("allow"),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });

        match permission_task.await {
            RequestPermissionOutcome::Selected(outcome) => {
                assert_eq!(outcome.option_id, acp_v2::PermissionOptionId::new("allow"));
                assert_eq!(outcome.option_kind, acp_v2::PermissionOptionKind::AllowOnce);
            }
            RequestPermissionOutcome::Cancelled
            | RequestPermissionOutcome::InterruptedByFollowUp => {
                panic!("permission request should resolve after authorization")
            }
        }
    }

    #[gpui::test]
    async fn test_cancel_tool_call_authorization_resolves_permission_request(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let tool_call_id = acp_v2::ToolCallId::new("toolu_01cancelled_permission");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        "Needs permission",
                    )
                    .kind(acp_v1::ToolKind::Execute)
                    .status(acp_v1::ToolCallStatus::Pending)
                    .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        acp_v2::PermissionOptionId::new("allow"),
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread.cancel_tool_call_authorization(&tool_call_id, cx);
        });

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(tool_call.status(), ToolCallStatus::Canceled));
        });

        match permission_task.await {
            RequestPermissionOutcome::Cancelled => {}
            RequestPermissionOutcome::InterruptedByFollowUp
            | RequestPermissionOutcome::Selected(_) => {
                panic!("cancelled permission request should not select an outcome")
            }
        }
    }

    #[gpui::test]
    async fn test_terminal_tool_call_update_closes_open_permission_request(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let tool_call_id = acp_v2::ToolCallId::new("toolu_01completed_while_waiting");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        "Needs permission",
                    )
                    .kind(acp_v1::ToolKind::Execute)
                    .status(acp_v1::ToolCallStatus::Pending)
                    .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        acp_v2::PermissionOptionId::new("allow"),
                        "Allow",
                        acp_v2::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp_v1::SessionUpdate::ToolCallUpdate(acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new()
                            .status(acp_v1::ToolCallStatus::Completed),
                    )),
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(tool_call.status(), ToolCallStatus::Completed));
        });

        match permission_task.await {
            RequestPermissionOutcome::Cancelled => {}
            RequestPermissionOutcome::InterruptedByFollowUp
            | RequestPermissionOutcome::Selected(_) => {
                panic!("terminal tool call update should close pending permission request")
            }
        }
    }

    #[gpui::test]
    async fn test_no_pending_edits_if_tool_calls_are_completed(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/test"), json!({})).await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            move |_, thread, mut cx| {
                async move {
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.handle_session_update(
                                acp_v1::SessionUpdate::ToolCall(
                                    acp_v1::ToolCall::new("test", "Label")
                                        .kind(acp_v1::ToolKind::Edit)
                                        .status(acp_v1::ToolCallStatus::Completed)
                                        .content(vec![acp_v1::ToolCallContent::Diff(
                                            acp_v1::Diff::new("/test/test.txt", "foo"),
                                        )]),
                                ),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["Hi".into()], cx)))
            .await
            .unwrap();

        assert!(cx.read(|cx| !thread.read(cx).has_pending_edit_tool_calls()));
    }

    #[gpui::test(iterations = 10)]
    async fn test_checkpoints(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                ".git": {}
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let simulate_changes = Arc::new(AtomicBool::new(true));
        let next_filename = Arc::new(AtomicUsize::new(0));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let simulate_changes = simulate_changes.clone();
            let next_filename = next_filename.clone();
            let fs = fs.clone();
            move |request, thread, mut cx| {
                let fs = fs.clone();
                let simulate_changes = simulate_changes.clone();
                let next_filename = next_filename.clone();
                async move {
                    if simulate_changes.load(SeqCst) {
                        let filename = format!("/test/file-{}", next_filename.fetch_add(1, SeqCst));
                        fs.write(Path::new(&filename), b"").await?;
                    }

                    let acp_v2::ContentBlock::Text(content) = &request.prompt[0] else {
                        panic!("expected text content block");
                    };
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp_v1::SessionUpdate::AgentMessageChunk(
                                    acp_v1::ContentChunk::new(content.text.to_uppercase().into()),
                                ),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["Lorem".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                "}
            );
        });
        assert_eq!(fs.files(), vec![Path::new(path!("/test/file-0"))]);

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["ipsum".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                    ## User (checkpoint)

                    ipsum

                    ## Assistant

                    IPSUM

                "}
            );
        });
        assert_eq!(
            fs.files(),
            vec![
                Path::new(path!("/test/file-0")),
                Path::new(path!("/test/file-1"))
            ]
        );

        // Checkpoint isn't stored when there are no changes.
        simulate_changes.store(false, SeqCst);
        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["dolor".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                    ## User (checkpoint)

                    ipsum

                    ## Assistant

                    IPSUM

                    ## User

                    dolor

                    ## Assistant

                    DOLOR

                "}
            );
        });
        assert_eq!(
            fs.files(),
            vec![
                Path::new(path!("/test/file-0")),
                Path::new(path!("/test/file-1"))
            ]
        );

        // Rewinding the conversation truncates the history and restores the checkpoint.
        thread
            .update(cx, |thread, cx| {
                let AgentThreadEntry::UserMessage(message) = &thread.entries[2] else {
                    panic!("unexpected entries {:?}", thread.entries)
                };
                thread.restore_checkpoint(message.client_id.clone().unwrap(), cx)
            })
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                "}
            );
        });
        assert_eq!(fs.files(), vec![Path::new(path!("/test/file-0"))]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_checkpoint_shows_when_file_changes_during_pending_message(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                ".git": {}
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let (request_started_tx, request_started_rx) = oneshot::channel::<()>();
        let request_started_tx = Rc::new(RefCell::new(Some(request_started_tx)));
        let (write_file_tx, write_file_rx) = oneshot::channel::<()>();
        let write_file_rx = Rc::new(RefCell::new(Some(write_file_rx)));
        let (file_written_tx, file_written_rx) = oneshot::channel::<()>();
        let file_written_tx = Rc::new(RefCell::new(Some(file_written_tx)));
        let (finish_response_tx, finish_response_rx) = oneshot::channel::<()>();
        let finish_response_tx = Rc::new(RefCell::new(Some(finish_response_tx)));
        let finish_response_rx = Rc::new(RefCell::new(Some(finish_response_rx)));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let request_started_tx = request_started_tx.clone();
            let write_file_rx = write_file_rx.clone();
            let file_written_tx = file_written_tx.clone();
            let finish_response_rx = finish_response_rx.clone();
            move |_request, thread, mut cx| {
                let write_file_rx = write_file_rx.borrow_mut().take();
                let finish_response_rx = finish_response_rx.borrow_mut().take();
                let request_started_tx = request_started_tx.borrow_mut().take();
                let file_written_tx = file_written_tx.borrow_mut().take();
                async move {
                    if let Some(request_started_tx) = request_started_tx {
                        request_started_tx.send(()).ok();
                    }
                    if let Some(write_file_rx) = write_file_rx {
                        write_file_rx.await.ok();
                    }

                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.write_text_file(
                                PathBuf::from(path!("/test/file")),
                                String::new(),
                                cx,
                            )
                        })?
                        .await?;

                    if let Some(file_written_tx) = file_written_tx {
                        file_written_tx.send(()).ok();
                    }
                    if let Some(finish_response_rx) = finish_response_rx {
                        finish_response_rx.await.ok();
                    }

                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let send = thread.update(cx, |thread, cx| thread.send(vec!["hello".into()], cx));
        let send_task = cx.background_executor.spawn(send);
        request_started_rx.await.unwrap();
        cx.run_until_parked();

        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User

                    hello

                "}
            );
        });

        write_file_tx.send(()).ok();
        file_written_rx.await.unwrap();
        cx.run_until_parked();

        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    hello

                "}
            );
        });

        finish_response_tx
            .borrow_mut()
            .take()
            .unwrap()
            .send(())
            .ok();
        send_task.await.unwrap();
    }

    #[gpui::test]
    async fn test_no_checkpoints_when_restore_is_unavailable(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                ".git": {}
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        let checkpoint_jobs = |cx: &mut TestAppContext| {
            project.read_with(cx, |project, cx| {
                let repository = project.git_store().read(cx).active_repository().unwrap();
                let queue = repository.read(cx).job_debug_queue().to_debug_value();
                queue["entries"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|job| job["description"] == "checkpoint")
                    .count()
            })
        };

        let next_filename = Arc::new(AtomicUsize::new(0));
        let finish_turn_rx_slot = Rc::new(RefCell::new(None::<oneshot::Receiver<()>>));
        let write_file_on_prompt = {
            let fs = fs.clone();
            let finish_turn_rx_slot = finish_turn_rx_slot.clone();
            move |_request: acp_v2::PromptRequest,
                  _thread: WeakEntity<AcpThread>,
                  _cx: AsyncApp|
                  -> LocalBoxFuture<'static, Result<acp_v1::PromptResponse>> {
                let fs = fs.clone();
                let path = Path::new(path!("/test"))
                    .join(format!("file-{}", next_filename.fetch_add(1, SeqCst)));
                let finish_turn_rx = finish_turn_rx_slot.borrow_mut().take();
                async move {
                    fs.write(&path, b"").await?;
                    if let Some(finish_turn_rx) = finish_turn_rx {
                        finish_turn_rx.await.ok();
                    }
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        };

        let connection = Rc::new(
            FakeAgentConnection::new()
                .without_truncate_support()
                .on_user_message(write_file_on_prompt.clone()),
        );
        let thread_without_truncate = cx
            .update(|cx| {
                connection.new_session(
                    project.clone(),
                    PathList::new(&[Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new().on_user_message(write_file_on_prompt));
        let parent = cx
            .update(|cx| {
                connection.clone().new_session(
                    project.clone(),
                    PathList::new(&[Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .unwrap();
        let subagent_session_id = acp_v2::SessionId::new("subagent");
        let subagent = cx.update(|cx| {
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            cx.new(|cx| {
                AcpThread::new(
                    Some(parent.read(cx).session_id().clone()),
                    None,
                    None,
                    connection.clone(),
                    project.clone(),
                    action_log,
                    subagent_session_id.clone(),
                    watch::Receiver::constant(acp_v2::PromptCapabilities::new()),
                    cx,
                )
            })
        });
        connection
            .sessions
            .lock()
            .insert(subagent_session_id, subagent.downgrade());

        // The parent shows that sending, repository updates during the turn, and
        // turn completion each take checkpoints, so zero counts elsewhere mean
        // those paths ran without taking any.
        for (thread, can_rewind) in [
            (thread_without_truncate, false),
            (subagent, false),
            (parent, true),
        ] {
            assert_eq!(
                thread.read_with(cx, |thread, cx| {
                    thread.can_rewind_to(Some(&ClientUserMessageId::new()), cx)
                }),
                can_rewind
            );
            let jobs_before_send = checkpoint_jobs(cx);
            let (finish_turn_tx, finish_turn_rx) = oneshot::channel();
            finish_turn_rx_slot.replace(Some(finish_turn_rx));
            let send = thread.update(cx, |thread, cx| thread.send(vec!["hello".into()], cx));
            let send_task = cx.background_executor.spawn(send);
            cx.run_until_parked();

            assert!(thread.read_with(cx, |thread, _| thread.running_turn.is_some()));
            let jobs_while_running = checkpoint_jobs(cx);
            finish_turn_tx.send(()).ok();
            send_task.await.unwrap();
            cx.run_until_parked();
            let jobs_after_turn = checkpoint_jobs(cx);

            if can_rewind {
                assert_eq!(jobs_while_running, jobs_before_send + 2);
                assert_eq!(jobs_after_turn, jobs_while_running + 1);
            } else {
                assert_eq!(jobs_while_running, jobs_before_send);
                assert_eq!(jobs_after_turn, jobs_before_send);
            }
            thread.read_with(cx, |thread, _| {
                let AgentThreadEntry::UserMessage(message) = &thread.entries[0] else {
                    panic!("unexpected entries {:?}", thread.entries)
                };
                assert_eq!(message.checkpoint.is_some(), can_rewind);
            });
        }
        assert_eq!(
            fs.files(),
            vec![
                Path::new(path!("/test/file-0")),
                Path::new(path!("/test/file-1")),
                Path::new(path!("/test/file-2"))
            ]
        );
    }

    #[gpui::test]
    async fn test_tool_result_refusal(cx: &mut TestAppContext) {
        use std::sync::atomic::AtomicUsize;
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None, cx).await;

        // Create a connection that simulates refusal after tool result
        let prompt_count = Arc::new(AtomicUsize::new(0));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let prompt_count = prompt_count.clone();
            move |_request, thread, mut cx| {
                let count = prompt_count.fetch_add(1, SeqCst);
                async move {
                    if count == 0 {
                        // First prompt: Generate a tool call with result
                        thread.update(&mut cx, |thread, cx| {
                            thread
                                .handle_session_update(
                                    acp_v1::SessionUpdate::ToolCall(
                                        acp_v1::ToolCall::new("tool1", "Test Tool")
                                            .kind(acp_v1::ToolKind::Fetch)
                                            .status(acp_v1::ToolCallStatus::Completed)
                                            .raw_input(serde_json::json!({"query": "test"}))
                                            .raw_output(serde_json::json!({"result": "inappropriate content"})),
                                    ),
                                    cx,
                                )
                                .unwrap();
                        })?;

                        // Now return refusal because of the tool result
                        Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::Refusal))
                    } else {
                        Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                    }
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        // Track if we see a Refusal event
        let saw_refusal_event = Arc::new(std::sync::Mutex::new(false));
        let saw_refusal_event_captured = saw_refusal_event.clone();
        thread.update(cx, |_thread, cx| {
            cx.subscribe(
                &thread,
                move |_thread, _event_thread, event: &AcpThreadEvent, _cx| {
                    if matches!(event, AcpThreadEvent::Refusal) {
                        *saw_refusal_event_captured.lock().unwrap() = true;
                    }
                },
            )
            .detach();
        });

        // Send a user message - this will trigger tool call and then refusal
        let send_task = thread.update(cx, |thread, cx| thread.send(vec!["Hello".into()], cx));
        cx.background_executor.spawn(send_task).detach();
        cx.run_until_parked();

        // Verify that:
        // 1. A Refusal event WAS emitted (because it's a tool result refusal, not user prompt)
        // 2. The user message was NOT truncated
        assert!(
            *saw_refusal_event.lock().unwrap(),
            "Refusal event should be emitted for tool result refusals"
        );

        thread.read_with(cx, |thread, _| {
            let entries = thread.entries();
            assert!(entries.len() >= 2, "Should have user message and tool call");

            // Verify user message is still there
            assert!(
                matches!(entries[0], AgentThreadEntry::UserMessage(_)),
                "User message should not be truncated"
            );

            // Verify tool call is there with result
            if let AgentThreadEntry::ToolCall(tool_call) = &entries[1] {
                assert!(
                    tool_call.raw_output.is_some(),
                    "Tool call should have output"
                );
            } else {
                panic!("Expected tool call at index 1");
            }
        });
    }

    #[gpui::test]
    async fn test_user_prompt_refusal_emits_event(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None, cx).await;

        let refuse_next = Arc::new(AtomicBool::new(false));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let refuse_next = refuse_next.clone();
            move |_request, _thread, _cx| {
                if refuse_next.load(SeqCst) {
                    async move { Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::Refusal)) }
                        .boxed_local()
                } else {
                    async move { Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)) }
                        .boxed_local()
                }
            }
        }));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        // Track if we see a Refusal event
        let saw_refusal_event = Arc::new(std::sync::Mutex::new(false));
        let saw_refusal_event_captured = saw_refusal_event.clone();
        thread.update(cx, |_thread, cx| {
            cx.subscribe(
                &thread,
                move |_thread, _event_thread, event: &AcpThreadEvent, _cx| {
                    if matches!(event, AcpThreadEvent::Refusal) {
                        *saw_refusal_event_captured.lock().unwrap() = true;
                    }
                },
            )
            .detach();
        });

        // Send a message that will be refused
        refuse_next.store(true, SeqCst);
        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["hello".into()], cx)))
            .await
            .unwrap();

        // Verify that a Refusal event WAS emitted for user prompt refusal
        assert!(
            *saw_refusal_event.lock().unwrap(),
            "Refusal event should be emitted for user prompt refusals"
        );

        // Verify the message was truncated (user prompt refusal)
        thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.to_markdown(cx), "");
        });
    }

    #[gpui::test]
    async fn test_refusal(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/"), json!({})).await;
        let project = Project::test(fs.clone(), [path!("/").as_ref()], cx).await;

        let refuse_next = Arc::new(AtomicBool::new(false));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let refuse_next = refuse_next.clone();
            move |request, thread, mut cx| {
                let refuse_next = refuse_next.clone();
                async move {
                    if refuse_next.load(SeqCst) {
                        return Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::Refusal));
                    }

                    let acp_v2::ContentBlock::Text(content) = &request.prompt[0] else {
                        panic!("expected text content block");
                    };
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp_v1::SessionUpdate::AgentMessageChunk(
                                    acp_v1::ContentChunk::new(content.text.to_uppercase().into()),
                                ),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["hello".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User

                    hello

                    ## Assistant

                    HELLO

                "}
            );
        });

        // Simulate refusing the second message. The message should be truncated
        // when a user prompt is refused.
        refuse_next.store(true, SeqCst);
        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["world".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User

                    hello

                    ## Assistant

                    HELLO

                "}
            );
        });
    }

    async fn new_test_thread(cx: &mut TestAppContext) -> Entity<AcpThread> {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        cx.update(|cx| {
            connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
        })
        .await
        .unwrap()
    }

    fn only_thread_elicitation(thread: &AcpThread) -> (ElicitationEntryId, &Elicitation) {
        let [entry] = thread.entries() else {
            panic!("expected one elicitation entry, got {:?}", thread.entries());
        };
        let AgentThreadEntry::Elicitation(id) = entry else {
            panic!("expected one elicitation entry, got {:?}", thread.entries());
        };
        let Some((_, elicitation)) = thread.elicitation(id) else {
            panic!("missing elicitation entry");
        };
        (id.clone(), elicitation)
    }

    fn latest_thread_elicitation(thread: &AcpThread) -> (ElicitationEntryId, &Elicitation) {
        let Some(AgentThreadEntry::Elicitation(id)) = thread.entries().last() else {
            panic!("expected latest entry to be an elicitation");
        };
        let Some((_, elicitation)) = thread.elicitation(id) else {
            panic!("missing elicitation entry");
        };
        (id.clone(), elicitation)
    }

    #[gpui::test]
    async fn test_elicitation_is_available(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let result = thread.update(cx, |thread, cx| {
            thread.request_elicitation(
                acp_v2::CreateElicitationRequest::new(
                    acp_v2::ElicitationFormMode::new(
                        acp_v2::ElicitationSessionScope::new(session_id),
                        acp_v2::ElicitationSchema::new().string("name", true),
                    ),
                    "Provide a name",
                ),
                cx,
            )
        });

        assert!(result.is_ok());
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.entries(),
                [AgentThreadEntry::Elicitation(_)]
            ));
        });
    }

    #[gpui::test]
    async fn test_form_elicitation_accepts_response(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let tool_call_id = acp_v2::ToolCallId::new("tool-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id.clone())
                                .tool_call_id(tool_call_id.clone()),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        let elicitation_id = thread.read_with(cx, |thread, _| {
            let (elicitation_id, elicitation) = only_thread_elicitation(thread);
            let acp_v2::ElicitationScope::Session(scope) = elicitation.request.scope() else {
                panic!("expected session-scoped elicitation");
            };
            assert_eq!(scope.tool_call_id.as_ref(), Some(&tool_call_id));
            elicitation_id
        });

        let expected_content = std::collections::BTreeMap::from([(
            "name".to_string(),
            acp_v2::ElicitationContentValue::from("Ada"),
        )]);
        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new().content(expected_content.clone()),
                )),
                cx,
            );
        });

        let response = response_task.await;
        assert_eq!(
            response.action,
            acp_v2::ElicitationAction::Accept(
                acp_v2::ElicitationAcceptAction::new().content(expected_content)
            )
        );
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Accepted));
        });
    }

    #[gpui::test]
    async fn test_url_elicitation_can_be_completed(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp_v2::ElicitationId::new("url-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            url_elicitation_id.clone(),
                            "https://example.com/complete",
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
                .unwrap()
        });

        let entry_id = thread.read_with(cx, |thread, _| {
            let (entry_id, _) = only_thread_elicitation(thread);
            entry_id
        });

        thread.update(cx, |thread, cx| {
            thread.complete_url_elicitation(&url_elicitation_id, cx);
        });
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(
                elicitation.status,
                ElicitationStatus::Pending { .. }
            ));
        });
        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &entry_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));
        thread.update(cx, |thread, cx| {
            thread.complete_url_elicitation(&url_elicitation_id, cx);
        });
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Completed));
        });
    }

    #[gpui::test]
    async fn test_idle_cancel_cancels_accepted_url_elicitation(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp_v2::ElicitationId::new("url-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            url_elicitation_id.clone(),
                            "https://example.com/complete",
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
                .unwrap()
        });

        let entry_id = thread.read_with(cx, |thread, _| {
            let (entry_id, _) = only_thread_elicitation(thread);
            entry_id
        });

        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &entry_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));

        thread.update(cx, |thread, cx| {
            thread.cancel(cx).detach();
        });
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });

        thread.update(cx, |thread, cx| {
            thread.complete_url_elicitation(&url_elicitation_id, cx);
        });
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_cancel_accepted_url_elicitation_marks_canceled(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp_v2::ElicitationId::new("url-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            url_elicitation_id.clone(),
                            "https://example.com/complete",
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
                .unwrap()
        });

        let entry_id = thread.read_with(cx, |thread, _| {
            let (entry_id, _) = only_thread_elicitation(thread);
            entry_id
        });

        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &entry_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Accepted));
        });

        thread.update(cx, |thread, cx| {
            thread.cancel(cx).detach();
        });
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });

        thread.update(cx, |thread, cx| {
            thread.complete_url_elicitation(&url_elicitation_id, cx);
        });
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_turn_cancel_cancels_accepted_url_elicitation_from_previous_turn(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let prompt_count = Rc::new(RefCell::new(0usize));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let prompt_count = prompt_count.clone();
            move |_request, _thread, _cx| {
                let stop_reason = {
                    let mut prompt_count = prompt_count.borrow_mut();
                    let stop_reason = if *prompt_count == 0 {
                        acp_v1::StopReason::EndTurn
                    } else {
                        acp_v1::StopReason::Cancelled
                    };
                    *prompt_count += 1;
                    stop_reason
                };

                async move { Ok(acp_v1::PromptResponse::new(stop_reason)) }.boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("new session should succeed");

        let response = thread
            .update(cx, |thread, cx| thread.send(vec!["first turn".into()], cx))
            .await
            .expect("first turn should succeed")
            .expect("first turn should return a response");
        assert!(
            matches!(response, SubmissionResponse::LegacyCompleted(response)
            if response.stop_reason == acp_v1::StopReason::EndTurn)
        );

        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp_v2::ElicitationId::new("url-1");
        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            url_elicitation_id.clone(),
                            "https://example.com/complete",
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
                .expect("url elicitation should be accepted")
        });

        let entry_id = thread.read_with(cx, |thread, _| {
            let (entry_id, _) = latest_thread_elicitation(thread);
            entry_id
        });

        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &entry_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));

        let response = thread
            .update(cx, |thread, cx| thread.send(vec!["second turn".into()], cx))
            .await
            .expect("second turn should succeed")
            .expect("second turn should return a response");
        assert!(
            matches!(response, SubmissionResponse::LegacyCompleted(response)
            if response.stop_reason == acp_v1::StopReason::Cancelled)
        );
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });

        thread.update(cx, |thread, cx| {
            thread.complete_url_elicitation(&url_elicitation_id, cx);
        });
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_request_scoped_elicitation_store_accepts_response(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let request: acp_v2::CreateElicitationRequest = serde_json::from_value(serde_json::json!({
            "mode": "form",
            "requestId": "0001",
            "message": "Provide details",
            "_meta": {"request": {"opaque": [1, null]}},
            "requestedSchema": {
                "type": "object",
                "required": ["future"],
                "_meta": {"schema": {}},
                "properties": {
                    "future": {"type": "_location", "nested": {"precision": "city"},
                               "_meta": {"property": [null]}},
                    "formatted": {"type": "string", "format": "_future",
                                  "_meta": {"format": true}},
                    "items": {"type": "array",
                              "items": {"type": "_token", "values": ["b", "a"],
                                        "_meta": {"items": {}}}}
                }
            }
        }))
        .unwrap();

        let response_task = store.update(cx, |store, cx| {
            store.request_elicitation(request.clone(), cx).unwrap()
        });

        let elicitation_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one elicitation entry, got {:?}",
                    store.elicitations()
                );
            };
            let acp_v2::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, acp_v2::RequestId::Str("0001".into()));
            assert_eq!(elicitation.request, request);
            elicitation.id.clone()
        });

        let response = acp_v2::CreateElicitationResponse::new(acp_v2::OtherElicitationAction::new(
            "_defer",
            std::collections::BTreeMap::from([(
                "reason".to_string(),
                serde_json::json!({"opaque": [1, null]}),
            )]),
        ))
        .meta(serde_json::Map::from_iter([(
            "response".to_string(),
            serde_json::json!({"nested": {}}),
        )]));
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(&elicitation_id, response.clone(), cx);
            store.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAcceptAction::new()),
                cx,
            );
        });

        assert_eq!(response_task.await, response);
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert_eq!(elicitation.request, request);
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_request_elicitation_store_ignores_duplicate_response(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let responded_ids = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            let responded_ids = responded_ids.clone();
            cx.subscribe(&store, move |_, event, _| {
                if let ElicitationStoreEvent::ElicitationResponded(id) = event {
                    responded_ids.borrow_mut().push(id.clone());
                }
            })
        });

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        let elicitation_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one elicitation entry, got {:?}",
                    store.elicitations()
                );
            };
            elicitation.id.clone()
        });

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Decline),
                cx,
            );
            store.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });

        assert_eq!(
            response_task.await.action,
            acp_v2::ElicitationAction::Decline
        );
        assert_eq!(
            responded_ids.borrow().as_slice(),
            std::slice::from_ref(&elicitation_id)
        );
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Declined));
        });
    }

    #[gpui::test]
    async fn test_cancel_session_elicitation_by_id_resolves_cancel(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let (elicitation_id, response_task) = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation_with_id(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        thread.update(cx, |thread, cx| {
            thread.cancel_elicitation(&elicitation_id, cx);
        });

        assert_eq!(
            response_task.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_cancel_pending_session_elicitation_resolves_cancel(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        let elicitation_id = thread.read_with(cx, |thread, _| {
            let (elicitation_id, _) = only_thread_elicitation(thread);
            elicitation_id
        });

        thread.update(cx, |thread, cx| {
            thread.cancel(cx).detach();
        });

        assert_eq!(
            response_task.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    fn request_test_session_elicitation(
        thread: WeakEntity<AcpThread>,
        session_id: acp_v2::SessionId,
        cx: &mut AsyncApp,
    ) -> Result<Task<acp_v2::CreateElicitationResponse>> {
        thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .map_err(|error| anyhow!(error))
        })?
    }

    #[gpui::test]
    async fn test_prompt_error_cancels_pending_session_elicitation(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let elicitation_action = Rc::new(RefCell::new(None));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let elicitation_action = elicitation_action.clone();
            move |request, thread, mut cx| {
                let elicitation_action = elicitation_action.clone();
                async move {
                    let response_task =
                        request_test_session_elicitation(thread, request.session_id, &mut cx)?;
                    cx.spawn(async move |_cx| {
                        let response = response_task.await;
                        *elicitation_action.borrow_mut() = Some(response.action);
                    })
                    .detach();

                    Err(anyhow!("prompt failed"))
                }
                .boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("new session should succeed");

        let result = thread
            .update(cx, |thread, cx| thread.send(vec!["hello".into()], cx))
            .await;

        assert!(result.is_err());
        cx.run_until_parked();
        assert_eq!(
            *elicitation_action.borrow(),
            Some(acp_v2::ElicitationAction::Cancel)
        );
        thread.read_with(cx, |thread, _| {
            let Some(elicitation) = thread.entries().iter().find_map(|entry| match entry {
                AgentThreadEntry::Elicitation(id) => {
                    thread.elicitation(id).map(|(_, elicitation)| elicitation)
                }
                _ => None,
            }) else {
                panic!("expected an elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_max_tokens_cancels_pending_session_elicitation(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let elicitation_action = Rc::new(RefCell::new(None));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let elicitation_action = elicitation_action.clone();
            move |request, thread, mut cx| {
                let elicitation_action = elicitation_action.clone();
                async move {
                    let response_task =
                        request_test_session_elicitation(thread, request.session_id, &mut cx)?;
                    cx.spawn(async move |_cx| {
                        let response = response_task.await;
                        *elicitation_action.borrow_mut() = Some(response.action);
                    })
                    .detach();

                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::MaxTokens))
                }
                .boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("new session should succeed");

        let result = thread
            .update(cx, |thread, cx| thread.send(vec!["hello".into()], cx))
            .await;

        assert!(result.is_err());
        cx.run_until_parked();
        assert_eq!(
            *elicitation_action.borrow(),
            Some(acp_v2::ElicitationAction::Cancel)
        );
        thread.read_with(cx, |thread, _| {
            let Some(elicitation) = thread.entries().iter().find_map(|entry| match entry {
                AgentThreadEntry::Elicitation(id) => {
                    thread.elicitation(id).map(|(_, elicitation)| elicitation)
                }
                _ => None,
            }) else {
                panic!("expected an elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_cancel_request_scoped_elicitation_resolves_cancel(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let (elicitation_id, response_task) = store.update(cx, |store, cx| {
            store
                .request_elicitation_with_id(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        store.update(cx, |store, cx| {
            store.cancel_elicitation(&elicitation_id, cx);
        });

        assert_eq!(
            response_task.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_request_elicitation_store_emits_response_when_waiter_is_dropped(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let responded_ids = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            let responded_ids = responded_ids.clone();
            cx.subscribe(&store, move |_, event, _| {
                if let ElicitationStoreEvent::ElicitationResponded(id) = event {
                    responded_ids.borrow_mut().push(id.clone());
                }
            })
        });

        let request_id = acp_v1::RequestId::Number(1);
        let (elicitation_id, response_task) = store.update(cx, |store, cx| {
            store
                .request_elicitation_with_id(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(request_id.clone()),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .expect("elicitation should succeed")
        });
        drop(response_task);
        cx.run_until_parked();

        store.update(cx, |store, cx| {
            store.cancel_request(&request_id, cx);
        });
        assert_eq!(
            responded_ids.borrow().as_slice(),
            std::slice::from_ref(&elicitation_id)
        );
        store.read_with(cx, |store, _| {
            let (_, elicitation) = store.elicitation(&elicitation_id).expect("elicitation");
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_request_elicitation_store_cancel_all_resolves_cancel(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        store.update(cx, |store, cx| {
            store.cancel_all(cx);
        });

        assert_eq!(
            response_task.await.action,
            acp_v2::ElicitationAction::Cancel
        );
    }

    #[gpui::test]
    async fn test_request_elicitation_store_clear_removes_answered_and_cancels_pending(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let first_response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });
        let second_response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(2)),
                            acp_v2::ElicitationSchema::new().string("account", true),
                        ),
                        "Provide an account",
                    ),
                    cx,
                )
                .unwrap()
        });

        let first_elicitation_id = store.read_with(cx, |store, _| {
            let [first, _second] = store.elicitations() else {
                panic!("expected two elicitations, got {:?}", store.elicitations());
            };
            first.id.clone()
        });

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &first_elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Decline),
                cx,
            );
            store.clear(cx);
        });

        assert_eq!(
            first_response_task.await.action,
            acp_v2::ElicitationAction::Decline
        );
        assert_eq!(
            second_response_task.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        store.read_with(cx, |store, _| assert!(store.elicitations().is_empty()));
    }

    #[gpui::test]
    async fn test_request_elicitation_store_clear_resolved_preserves_outstanding(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let url_elicitation_id = acp_v2::ElicitationId::new("url-1");

        let accepted_response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });
        let pending_response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(2)),
                            acp_v2::ElicitationSchema::new().string("account", true),
                        ),
                        "Provide an account",
                    ),
                    cx,
                )
                .unwrap()
        });
        let accepted_url_response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(3)),
                            url_elicitation_id,
                            "https://example.com/complete",
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
                .unwrap()
        });

        let (accepted_id, pending_id, accepted_url_id) = store.read_with(cx, |store, _| {
            let [accepted, pending, accepted_url] = store.elicitations() else {
                panic!(
                    "expected three request-scoped elicitations, got {:?}",
                    store.elicitations()
                );
            };
            (
                accepted.id.clone(),
                pending.id.clone(),
                accepted_url.id.clone(),
            )
        });

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &accepted_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
            store.respond_to_elicitation(
                &accepted_url_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            accepted_response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));
        assert!(matches!(
            accepted_url_response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));

        let cleared_ids = store.update(cx, |store, cx| store.clear_resolved(cx));
        assert_eq!(cleared_ids, vec![accepted_id]);
        store.read_with(cx, |store, _| {
            let [pending, accepted_url] = store.elicitations() else {
                panic!(
                    "expected pending and accepted url elicitations, got {:?}",
                    store.elicitations()
                );
            };
            assert_eq!(pending.id, pending_id);
            assert!(matches!(pending.status, ElicitationStatus::Pending { .. }));
            assert_eq!(accepted_url.id, accepted_url_id);
            assert!(matches!(accepted_url.status, ElicitationStatus::Accepted));
        });

        store.update(cx, |store, cx| store.clear(cx));
        assert_eq!(
            pending_response_task.await.action,
            acp_v2::ElicitationAction::Cancel
        );
    }

    #[gpui::test]
    async fn test_request_url_elicitation_store_can_be_completed(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let url_elicitation_id = acp_v2::ElicitationId::new("url-1");

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                            url_elicitation_id.clone(),
                            "https://example.com/complete",
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
                .unwrap()
        });

        let entry_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one request-scoped elicitation, got {:?}",
                    store.elicitations()
                );
            };
            elicitation.id.clone()
        });

        store.update(cx, |store, cx| {
            store.complete_url_elicitation(&url_elicitation_id, cx);
        });
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(
                elicitation.status,
                ElicitationStatus::Pending { .. }
            ));
        });
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &entry_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));
        store.update(cx, |store, cx| {
            store.complete_url_elicitation(&url_elicitation_id, cx);
        });
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Completed));
        });
    }

    #[gpui::test]
    async fn test_request_url_elicitation_store_cancel_all_cancels_accepted_url(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let url_elicitation_id = acp_v2::ElicitationId::new("url-1");
        let responded_ids = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            let responded_ids = responded_ids.clone();
            cx.subscribe(&store, move |_, event, _| {
                if let ElicitationStoreEvent::ElicitationResponded(id) = event {
                    responded_ids.borrow_mut().push(id.clone());
                }
            })
        });

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                            url_elicitation_id.clone(),
                            "https://example.com/complete",
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
                .unwrap()
        });

        let entry_id = store.read_with(cx, |store, _| {
            let [elicitation] = store.elicitations() else {
                panic!(
                    "expected one elicitation entry, got {:?}",
                    store.elicitations()
                );
            };
            elicitation.id.clone()
        });

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &entry_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp_v2::ElicitationAction::Accept(_)
        ));
        assert_eq!(
            responded_ids.borrow().as_slice(),
            std::slice::from_ref(&entry_id)
        );
        store.update(cx, |store, cx| {
            store.cancel_all(cx);
        });
        assert_eq!(
            responded_ids.borrow().as_slice(),
            std::slice::from_ref(&entry_id)
        );
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });

        store.update(cx, |store, cx| {
            store.complete_url_elicitation(&url_elicitation_id, cx);
        });
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&entry_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_cancel_pending_elicitations_preserves_responded_statuses(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        let elicitation_id = thread.read_with(cx, |thread, _| {
            let (elicitation_id, _) = only_thread_elicitation(thread);
            elicitation_id
        });

        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Decline),
                cx,
            );
            thread.cancel(cx).detach();
        });

        assert_eq!(
            response_task.await.action,
            acp_v2::ElicitationAction::Decline
        );
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Declined));
        });
    }

    #[gpui::test]
    async fn test_session_elicitation_ignores_duplicate_response(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id),
                            acp_v2::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        let elicitation_id = thread.read_with(cx, |thread, _| {
            let (elicitation_id, _) = only_thread_elicitation(thread);
            elicitation_id
        });

        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Decline),
                cx,
            );
            thread.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });

        assert_eq!(
            response_task.await.action,
            acp_v2::ElicitationAction::Decline
        );
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Declined));
        });
    }

    #[gpui::test]
    async fn test_url_elicitation_rejects_non_browser_urls(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        for invalid_url in [
            "not a url",
            "file:///tmp/authorize",
            "data:text/plain,authorize",
            "mailto:user@example.com",
            "zed://settings",
        ] {
            let result = thread.update(cx, |thread, cx| {
                thread.request_elicitation(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationUrlMode::new(
                            acp_v2::ElicitationSessionScope::new(session_id.clone()),
                            "url-1",
                            invalid_url,
                        ),
                        "Complete this in the browser",
                    ),
                    cx,
                )
            });

            let Err(error) = result else {
                panic!("{invalid_url} should not be accepted for URL elicitation");
            };
            assert_eq!(error.code, acp_v2::ErrorCode::InvalidParams);
        }
        thread.read_with(cx, |thread, _| assert!(thread.entries().is_empty()));
    }

    #[gpui::test]
    async fn test_elicitation_rejects_unadvertised_mode(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let result = thread.update(cx, |thread, cx| {
            thread.request_elicitation(
                acp_v2::CreateElicitationRequest::new(
                    acp_v2::OtherElicitationMode::new(
                        "future",
                        acp_v2::ElicitationSessionScope::new(session_id),
                        std::collections::BTreeMap::new(),
                    ),
                    "Use a future input mode",
                ),
                cx,
            )
        });

        let Err(error) = result else {
            panic!("unadvertised elicitation mode should be rejected");
        };
        assert_eq!(error.code, acp_v2::ErrorCode::InvalidParams);
        thread.read_with(cx, |thread, _| assert!(thread.entries().is_empty()));
    }

    #[gpui::test]
    async fn test_request_elicitation_store_rejects_unadvertised_mode(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let result = store.update(cx, |store, cx| {
            store.request_elicitation(
                acp_v2::CreateElicitationRequest::new(
                    acp_v2::OtherElicitationMode::new(
                        "future",
                        acp_v2::ElicitationRequestScope::new(acp_v1::RequestId::Number(1)),
                        std::collections::BTreeMap::new(),
                    ),
                    "Use a future input mode",
                ),
                cx,
            )
        });

        let Err(error) = result else {
            panic!("unadvertised elicitation mode should be rejected");
        };
        assert_eq!(error.code, acp_v2::ErrorCode::InvalidParams);
        store.read_with(cx, |store, _| assert!(store.elicitations().is_empty()));
    }

    async fn run_until_first_tool_call(
        thread: &Entity<AcpThread>,
        cx: &mut TestAppContext,
    ) -> usize {
        let (mut tx, mut rx) = mpsc::channel::<usize>(1);

        let subscription = cx.update(|cx| {
            cx.subscribe(thread, move |thread, _, cx| {
                for (ix, entry) in thread.read(cx).entries.iter().enumerate() {
                    if matches!(entry, AgentThreadEntry::ToolCall(_)) {
                        return tx.try_send(ix).unwrap();
                    }
                }
            })
        });

        select! {
            _ = futures::FutureExt::fuse(cx.background_executor.timer(Duration::from_secs(10))) => {
                panic!("Timeout waiting for tool call")
            }
            ix = rx.next().fuse() => {
                drop(subscription);
                ix.unwrap()
            }
        }
    }

    #[derive(Clone, Default)]
    struct FakeAgentConnection {
        auth_methods: Vec<acp_v2::AuthMethod>,
        supports_truncate: bool,
        sessions: Arc<parking_lot::Mutex<HashMap<acp_v2::SessionId, WeakEntity<AcpThread>>>>,
        set_title_calls: Rc<RefCell<Vec<SharedString>>>,
        on_user_message: Option<
            Rc<
                dyn Fn(
                        acp_v2::PromptRequest,
                        WeakEntity<AcpThread>,
                        AsyncApp,
                    )
                        -> LocalBoxFuture<'static, Result<acp_v1::PromptResponse>>
                    + 'static,
            >,
        >,
    }

    impl FakeAgentConnection {
        fn new() -> Self {
            Self {
                auth_methods: Vec::new(),
                supports_truncate: true,
                on_user_message: None,
                sessions: Arc::default(),
                set_title_calls: Default::default(),
            }
        }

        fn without_truncate_support(mut self) -> Self {
            self.supports_truncate = false;
            self
        }

        #[expect(unused)]
        fn with_auth_methods(mut self, auth_methods: Vec<acp_v2::AuthMethod>) -> Self {
            self.auth_methods = auth_methods;
            self
        }

        fn on_user_message(
            mut self,
            handler: impl Fn(
                acp_v2::PromptRequest,
                WeakEntity<AcpThread>,
                AsyncApp,
            ) -> LocalBoxFuture<'static, Result<acp_v1::PromptResponse>>
            + 'static,
        ) -> Self {
            self.on_user_message.replace(Rc::new(handler));
            self
        }
    }

    impl AgentConnection for FakeAgentConnection {
        fn agent_id(&self) -> AgentId {
            AgentId::new("fake")
        }

        fn telemetry_id(&self) -> SharedString {
            "fake".into()
        }

        fn auth_methods(&self) -> &[acp_v2::AuthMethod] {
            &self.auth_methods
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let session_id = acp_v2::SessionId::new(
                rand::rng()
                    .sample_iter(&distr::Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect::<String>(),
            );
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
                    None,
                    None,
                    Some(work_dirs),
                    self.clone(),
                    project,
                    action_log,
                    session_id.clone(),
                    watch::Receiver::constant(
                        acp_v2::PromptCapabilities::new()
                            .image(acp_v2::PromptImageCapabilities::new())
                            .audio(acp_v2::PromptAudioCapabilities::new())
                            .embedded_context(acp_v2::PromptEmbeddedContextCapabilities::new()),
                    ),
                    cx,
                )
            });
            self.sessions.lock().insert(session_id, thread.downgrade());
            Task::ready(Ok(thread))
        }

        fn authenticate(
            &self,
            method: acp_v2::AuthMethodId,
            _cx: &mut App,
        ) -> Task<gpui::Result<()>> {
            if self
                .auth_methods()
                .iter()
                .any(|candidate| candidate.method_id() == &method)
            {
                Task::ready(Ok(()))
            } else {
                Task::ready(Err(anyhow!("Invalid Auth Method")))
            }
        }

        fn prompt(
            &self,
            params: acp_v2::PromptRequest,
            cx: &mut App,
        ) -> Task<gpui::Result<acp_v1::PromptResponse>> {
            let sessions = self.sessions.lock();
            let thread = sessions.get(&params.session_id).unwrap();
            if let Some(handler) = &self.on_user_message {
                let handler = handler.clone();
                let thread = thread.clone();
                cx.spawn(async move |cx| handler(params, thread, cx.clone()).await)
            } else {
                Task::ready(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            }
        }

        fn client_user_message_ids(
            &self,
            _cx: &App,
        ) -> Option<Rc<dyn AgentSessionClientUserMessageIds>> {
            self.supports_truncate.then(|| {
                Rc::new(FakeAgentSessionClientUserMessageIds {
                    connection: self.clone(),
                }) as Rc<dyn AgentSessionClientUserMessageIds>
            })
        }

        fn cancel(&self, _session_id: &acp_v2::SessionId, _cx: &mut App) {}

        fn truncate(
            &self,
            session_id: &acp_v2::SessionId,
            _cx: &App,
        ) -> Option<Rc<dyn AgentSessionTruncate>> {
            self.supports_truncate.then(|| {
                Rc::new(FakeAgentSessionEditor {
                    _session_id: session_id.clone(),
                }) as Rc<dyn AgentSessionTruncate>
            })
        }

        fn set_title(
            &self,
            _session_id: &acp_v2::SessionId,
            _cx: &App,
        ) -> Option<Rc<dyn AgentSessionSetTitle>> {
            Some(Rc::new(FakeAgentSessionSetTitle {
                calls: self.set_title_calls.clone(),
            }))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    struct FakeAgentSessionSetTitle {
        calls: Rc<RefCell<Vec<SharedString>>>,
    }

    impl AgentSessionSetTitle for FakeAgentSessionSetTitle {
        fn run(&self, title: SharedString, _cx: &mut App) -> Task<Result<()>> {
            self.calls.borrow_mut().push(title);
            Task::ready(Ok(()))
        }
    }

    struct FakeAgentSessionEditor {
        _session_id: acp_v2::SessionId,
    }

    impl AgentSessionTruncate for FakeAgentSessionEditor {
        fn run(
            &self,
            _client_user_message_id: ClientUserMessageId,
            _cx: &mut App,
        ) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }
    }

    struct FakeAgentSessionClientUserMessageIds {
        connection: FakeAgentConnection,
    }

    impl AgentSessionClientUserMessageIds for FakeAgentSessionClientUserMessageIds {
        fn prompt(
            &self,
            _client_user_message_id: ClientUserMessageId,
            params: acp_v2::PromptRequest,
            cx: &mut App,
        ) -> Task<Result<acp_v1::PromptResponse>> {
            self.connection.prompt(params, cx)
        }
    }

    #[gpui::test]
    async fn test_tool_call_not_found_creates_failed_entry(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        // Try to update a tool call that doesn't exist
        let nonexistent_id = acp_v2::ToolCallId::new("nonexistent-tool-call");
        thread.update(cx, |thread, cx| {
            let result = thread.handle_session_update(
                acp_v1::SessionUpdate::ToolCallUpdate(acp_v1::ToolCallUpdate::new(
                    acp_v1::ToolCallId::new(nonexistent_id.0.clone()),
                    acp_v1::ToolCallUpdateFields::new().status(acp_v1::ToolCallStatus::Completed),
                )),
                cx,
            );

            // The update should succeed (not return an error)
            assert!(result.is_ok());

            // There should now be exactly one entry in the thread
            assert_eq!(thread.entries.len(), 1);

            // The entry should be a failed tool call
            if let AgentThreadEntry::ToolCall(tool_call) = &thread.entries[0] {
                assert_eq!(tool_call.id, nonexistent_id);
                assert!(matches!(tool_call.status(), ToolCallStatus::Failed));
                assert_eq!(tool_call.kind(), &acp_v2::ToolKind::Fetch);

                // Check that the content contains the error message
                assert_eq!(tool_call.content().len(), 1);
                if let ToolCallContent::ContentBlock {
                    block: content_block,
                    ..
                } = &tool_call.content()[0]
                {
                    let markdown = content_block.plain_markdown().expect("expected markdown");
                    assert!(markdown.read(cx).source().contains("Tool call not found"));
                } else {
                    panic!("Expected ContentBlock, got: {:?}", tool_call.content()[0]);
                }
            } else {
                panic!("Expected ToolCall entry, got: {:?}", thread.entries[0]);
            }
        });
    }

    /// Tests that restoring a checkpoint properly cleans up terminals that were
    /// created after that checkpoint, and cancels any in-progress generation.
    ///
    /// Reproduces issue #35142: When a checkpoint is restored, any terminal processes
    /// that were started after that checkpoint should be terminated, and any in-progress
    /// AI generation should be canceled.
    #[gpui::test]
    async fn test_restore_checkpoint_kills_terminal(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        // Send first user message to create a checkpoint
        cx.update(|cx| {
            thread.update(cx, |thread, cx| {
                thread.send(vec!["first message".into()], cx)
            })
        })
        .await
        .unwrap();

        // Send second message (creates another checkpoint) - we'll restore to this one
        cx.update(|cx| {
            thread.update(cx, |thread, cx| {
                thread.send(vec!["second message".into()], cx)
            })
        })
        .await
        .unwrap();

        // Create 2 terminals BEFORE the checkpoint that have completed running
        let terminal_id_1 = acp_v1::TerminalId::new(uuid::Uuid::new_v4().to_string());
        let mock_terminal_1 = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            builder.subscribe(cx)
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id_1.clone(),
                    label: "echo 'first'".to_string(),
                    cwd: Some(PathBuf::from("/test")),
                    output_byte_limit: None,
                    terminal: mock_terminal_1.clone(),
                },
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id_1.clone(),
                    data: b"first\n".to_vec(),
                },
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id_1.clone(),
                    status: acp_v1::TerminalExitStatus::new().exit_code(0),
                },
                cx,
            );
        });

        let terminal_id_2 = acp_v1::TerminalId::new(uuid::Uuid::new_v4().to_string());
        let mock_terminal_2 = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            builder.subscribe(cx)
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id_2.clone(),
                    label: "echo 'second'".to_string(),
                    cwd: Some(PathBuf::from("/test")),
                    output_byte_limit: None,
                    terminal: mock_terminal_2.clone(),
                },
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id_2.clone(),
                    data: b"second\n".to_vec(),
                },
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id_2.clone(),
                    status: acp_v1::TerminalExitStatus::new().exit_code(0),
                },
                cx,
            );
        });

        // Get the second message ID to restore to
        let second_message_id = thread.read_with(cx, |thread, _| {
            // At this point we have:
            // - Index 0: First user message (with checkpoint)
            // - Index 1: Second user message (with checkpoint)
            // No assistant responses because FakeAgentConnection just returns EndTurn
            let AgentThreadEntry::UserMessage(message) = &thread.entries[1] else {
                panic!("expected user message at index 1");
            };
            message.client_id.clone().unwrap()
        });

        // Create a terminal AFTER the checkpoint we'll restore to.
        // This simulates the AI agent starting a long-running terminal command.
        let terminal_id = acp_v1::TerminalId::new(uuid::Uuid::new_v4().to_string());
        let mock_terminal = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            builder.subscribe(cx)
        });

        // Register the terminal as created
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "sleep 1000".to_string(),
                    cwd: Some(PathBuf::from("/test")),
                    output_byte_limit: None,
                    terminal: mock_terminal.clone(),
                },
                cx,
            );
        });

        // Simulate the terminal producing output (still running)
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: b"terminal is running...\n".to_vec(),
                },
                cx,
            );
        });

        // Create a tool call entry that references this terminal
        // This represents the agent requesting a terminal command
        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::ToolCall(
                        acp_v1::ToolCall::new("terminal-tool-1", "Running command")
                            .kind(acp_v1::ToolKind::Execute)
                            .status(acp_v1::ToolCallStatus::InProgress)
                            .content(vec![acp_v1::ToolCallContent::Terminal(
                                acp_v1::Terminal::new(terminal_id.clone()),
                            )])
                            .raw_input(serde_json::json!({"command": "sleep 1000", "cd": "/test"})),
                    ),
                    cx,
                )
                .unwrap();
        });

        let terminal_id = acp_v2::TerminalId::new(terminal_id.0);
        let terminal_id_1 = acp_v2::TerminalId::new(terminal_id_1.0);
        let terminal_id_2 = acp_v2::TerminalId::new(terminal_id_2.0);
        // Verify terminal exists and is in the thread
        let terminal_exists_before =
            thread.read_with(cx, |thread, _| thread.terminals.contains_key(&terminal_id));
        assert!(
            terminal_exists_before,
            "Terminal should exist before checkpoint restore"
        );

        // Verify the terminal's underlying task is still running (not completed)
        let terminal_running_before = thread.read_with(cx, |thread, _cx| {
            let terminal_entity = thread.terminals.get(&terminal_id).unwrap();
            terminal_entity.read_with(cx, |term, _cx| {
                term.output().is_none() // output is None means it's still running
            })
        });
        assert!(
            terminal_running_before,
            "Terminal should be running before checkpoint restore"
        );

        // Verify we have the expected entries before restore
        let entry_count_before = thread.read_with(cx, |thread, _| thread.entries.len());
        assert!(
            entry_count_before > 1,
            "Should have multiple entries before restore"
        );

        // Restore the checkpoint to the second message.
        // This should:
        // 1. Cancel any in-progress generation (via the cancel() call)
        // 2. Remove the terminal that was created after that point
        thread
            .update(cx, |thread, cx| {
                thread.restore_checkpoint(second_message_id, cx)
            })
            .await
            .unwrap();

        // Verify that no send_task is in progress after restore
        // (cancel() clears the send_task)
        let has_send_task_after = thread.read_with(cx, |thread, _| thread.running_turn.is_some());
        assert!(
            !has_send_task_after,
            "Should not have a send_task after restore (cancel should have cleared it)"
        );

        // Verify the entries were truncated (restoring to index 1 truncates at 1, keeping only index 0)
        let entry_count = thread.read_with(cx, |thread, _| thread.entries.len());
        assert_eq!(
            entry_count, 1,
            "Should have 1 entry after restore (only the first user message)"
        );

        // Verify the 2 completed terminals from before the checkpoint still exist
        let terminal_1_exists = thread.read_with(cx, |thread, _| {
            thread.terminals.contains_key(&terminal_id_1)
        });
        assert!(
            terminal_1_exists,
            "Terminal 1 (from before checkpoint) should still exist"
        );

        let terminal_2_exists = thread.read_with(cx, |thread, _| {
            thread.terminals.contains_key(&terminal_id_2)
        });
        assert!(
            terminal_2_exists,
            "Terminal 2 (from before checkpoint) should still exist"
        );

        // Verify they're still in completed state
        let terminal_1_completed = thread.read_with(cx, |thread, _cx| {
            let terminal_entity = thread.terminals.get(&terminal_id_1).unwrap();
            terminal_entity.read_with(cx, |term, _cx| term.output().is_some())
        });
        assert!(terminal_1_completed, "Terminal 1 should still be completed");

        let terminal_2_completed = thread.read_with(cx, |thread, _cx| {
            let terminal_entity = thread.terminals.get(&terminal_id_2).unwrap();
            terminal_entity.read_with(cx, |term, _cx| term.output().is_some())
        });
        assert!(terminal_2_completed, "Terminal 2 should still be completed");

        // Verify the running terminal (created after checkpoint) was removed
        let terminal_3_exists =
            thread.read_with(cx, |thread, _| thread.terminals.contains_key(&terminal_id));
        assert!(
            !terminal_3_exists,
            "Terminal 3 (created after checkpoint) should have been removed"
        );

        // Verify total count is 2 (the two from before the checkpoint)
        let terminal_count = thread.read_with(cx, |thread, _| thread.terminals.len());
        assert_eq!(
            terminal_count, 2,
            "Should have exactly 2 terminals (the completed ones from before checkpoint)"
        );
    }

    /// Tests that update_last_checkpoint correctly updates the original message's checkpoint
    /// even when a new user message is added while the async checkpoint comparison is in progress.
    ///
    /// This is a regression test for a bug where update_last_checkpoint would fail with
    /// "no checkpoint" if a new user message (without a checkpoint) was added between when
    /// update_last_checkpoint started and when its async closure ran.
    #[gpui::test]
    async fn test_update_last_checkpoint_with_new_message_added(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({".git": {}, "file.txt": "content"}))
            .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/test"))], cx).await;

        let handler_done = Arc::new(AtomicBool::new(false));
        let handler_done_clone = handler_done.clone();
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            move |_, _thread, _cx| {
                handler_done_clone.store(true, SeqCst);
                async move { Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)) }
                    .boxed_local()
            },
        ));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let send_future = thread.update(cx, |thread, cx| thread.send_raw("First message", cx));
        let send_task = cx.background_executor.spawn(send_future);

        // Tick until handler completes, then a few more to let update_last_checkpoint start
        while !handler_done.load(SeqCst) {
            cx.executor().tick();
        }
        for _ in 0..5 {
            cx.executor().tick();
        }

        thread.update(cx, |thread, cx| {
            let languages = thread.project.read(cx).languages().clone();
            let path_style = thread.project.read(cx).path_style(cx);
            let content = MessageContent::new(
                "Injected message (no checkpoint)".into(),
                &languages,
                path_style,
                cx,
            );
            thread.push_entry(
                AgentThreadEntry::UserMessage(UserMessage {
                    identity: MessageIdentity::Legacy(None),
                    meta: None,
                    client_id: Some(ClientUserMessageId::new()),
                    is_optimistic: true,
                    content,
                    checkpoint: None,
                    indented: false,
                }),
                cx,
            );
        });

        cx.run_until_parked();
        let result = send_task.await;

        assert!(
            result.is_ok(),
            "send should succeed even when new message added during update_last_checkpoint: {:?}",
            result.err()
        );
    }

    /// This is a regression test for a bug where update_last_checkpoint would
    /// swallow a checkpoint comparison error and hide an already-visible
    /// "Restore checkpoint" button without logging anything.
    #[gpui::test]
    async fn test_update_last_checkpoint_compare_error_keeps_checkpoint_visible(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), json!({".git": {}, "file.txt": "content"}))
            .await;
        let project = Project::test(fs.clone(), [Path::new(path!("/test"))], cx).await;

        // The handler waits for this signal so the repository can be swapped
        // out while the turn is still running.
        let (complete_tx, complete_rx) = futures::channel::oneshot::channel::<()>();
        let complete_rx = RefCell::new(Some(complete_rx));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            move |_, _thread, _cx| {
                let complete_rx = complete_rx.borrow_mut().take();
                async move {
                    if let Some(rx) = complete_rx {
                        rx.await.ok();
                    }
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            },
        ));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let send_future = thread.update(cx, |thread, cx| thread.send_raw("message", cx));
        let send_task = cx.background_executor.spawn(send_future);
        cx.run_until_parked();

        // Show the checkpoint, as update_last_checkpoint_if_changed does when
        // files change during the turn.
        thread.update(cx, |thread, _| {
            let (_, message) = thread.last_user_message().unwrap();
            message.checkpoint.as_mut().unwrap().show = true;
        });

        // Recreate `.git` so the git store reopens the repository. The fresh
        // fake repository doesn't contain the checkpoint recorded at send
        // time, so the end-of-turn comparison fails.
        fs.remove_dir(
            Path::new(path!("/test/.git")),
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        cx.run_until_parked();
        fs.create_dir(Path::new(path!("/test/.git"))).await.unwrap();
        cx.run_until_parked();

        complete_tx.send(()).unwrap();
        send_task.await.unwrap();
        cx.run_until_parked();

        thread.update(cx, |thread, _| {
            let (_, message) = thread.last_user_message().unwrap();
            assert!(
                message.checkpoint.as_ref().unwrap().show,
                "a checkpoint comparison failure must not hide the restore checkpoint button"
            );
        });
    }

    /// Tests that when a follow-up message is sent during generation,
    /// the first turn completing does NOT clear `running_turn` because
    /// it now belongs to the second turn.
    #[gpui::test]
    async fn test_follow_up_message_during_generation_does_not_clear_turn(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        // First handler waits for this signal before completing
        let (first_complete_tx, first_complete_rx) = futures::channel::oneshot::channel::<()>();
        let first_complete_rx = RefCell::new(Some(first_complete_rx));
        let (second_complete_tx, second_complete_rx) = oneshot::channel::<()>();
        let second_complete_rx = RefCell::new(Some(second_complete_rx));

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            move |params, _thread, _cx| {
                let first_complete_rx = first_complete_rx.borrow_mut().take();
                let is_first = params.prompt.iter().any(
                    |c| matches!(c, acp_v2::ContentBlock::Text(t) if t.text.contains("first")),
                );
                let second_complete_rx = if is_first {
                    None
                } else {
                    second_complete_rx.borrow_mut().take()
                };

                async move {
                    if is_first {
                        // First handler waits until signaled
                        if let Some(rx) = first_complete_rx {
                            rx.await.ok();
                        }
                    } else {
                        second_complete_rx
                            .expect("second completion receiver should be available")
                            .await?;
                    }
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        // Send first message (turn_id=1) - handler will block
        let first_request = thread.update(cx, |thread, cx| thread.send_raw("first", cx));
        cx.run_until_parked();
        assert_eq!(thread.read_with(cx, |t, _| t.turn_id), 1);
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        let (permission_id, permission_task) = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization_with_id(
                    acp_v1::ToolCall::new("permission", "Needs permission").into(),
                    PermissionOptions::Flat(Vec::new()),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        // Send second message (turn_id=2) while first is still blocked
        // This calls cancel() which takes turn 1's running_turn and sets turn 2's
        let second_request = thread.update(cx, |thread, cx| thread.send_raw("second", cx));
        thread.read_with(cx, |thread, _| {
            let tool_id = acp_v2::ToolCallId::new("permission");
            assert!(thread.permission_request(permission_id).is_none());
            assert!(thread.permission_request_for_tool(&tool_id).is_none());
            assert_eq!(thread.pending_permission_requests().count(), 0);
            assert!(
                thread
                    .tool_call(&tool_id)
                    .expect("interrupted tool")
                    .1
                    .authorization_id()
                    .is_none()
            );
        });
        assert!(matches!(
            permission_task.await,
            RequestPermissionOutcome::InterruptedByFollowUp
        ));
        assert_eq!(thread.read_with(cx, |t, _| t.turn_id), 2);
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        let running_turn_after_second_send =
            thread.read_with(cx, |thread, _| thread.running_turn.as_ref().map(|t| t.id));
        assert_eq!(
            running_turn_after_second_send,
            Some(2),
            "running_turn should be set to turn 2 after sending second message"
        );

        // Now signal first handler to complete
        first_complete_tx.send(()).ok();

        // First request completes - should NOT clear running_turn
        // because running_turn now belongs to turn 2
        first_request.await.unwrap();

        let running_turn_after_first =
            thread.read_with(cx, |thread, _| thread.running_turn.as_ref().map(|t| t.id));
        assert_eq!(
            running_turn_after_first,
            Some(2),
            "first turn completing should not clear running_turn (belongs to turn 2)"
        );
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        // Second request completes - SHOULD clear running_turn
        second_complete_tx
            .send(())
            .expect("second completion receiver should still be alive");
        second_request.await.unwrap();

        let running_turn_after_second =
            thread.read_with(cx, |thread, _| thread.running_turn.is_some());
        assert!(
            !running_turn_after_second,
            "second turn completing should clear running_turn"
        );
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_setting_toggle_updates_idle_sleep_prevention_for_running_turn(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        set_prevent_idle_sleep(false, cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        for (enabled, expected_count) in [(true, 1), (true, 1), (false, 0), (false, 0), (true, 1)] {
            set_prevent_idle_sleep(enabled, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), expected_count);
            assert_eq!(
                thread.read_with(cx, |thread, _| thread.status()),
                ThreadStatus::Generating
            );
        }

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        for enabled in [false, true] {
            set_prevent_idle_sleep(enabled, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
    }

    #[gpui::test]
    async fn test_stale_cancelled_response_does_not_cancel_current_compaction(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let (first_complete_tx, first_complete_rx) = futures::channel::oneshot::channel::<()>();
        let first_complete_rx = RefCell::new(Some(first_complete_rx));
        let compaction_id = ContextCompactionId("test-compaction".into());

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let compaction_id = compaction_id.clone();
            move |params, thread, mut cx| {
                let first_complete_rx = first_complete_rx.borrow_mut().take();
                let is_first = params.prompt.iter().any(|content| {
                    matches!(content, acp_v2::ContentBlock::Text(text) if text.text.contains("first"))
                });
                let compaction_id = compaction_id.clone();

                async move {
                    if is_first {
                        if let Some(rx) = first_complete_rx {
                            rx.await
                                .expect("first completion sender should still be alive");
                        }

                        thread.update(&mut cx, |thread, cx| {
                            thread.push_context_compaction(
                                ContextCompaction {
                                    id: compaction_id,
                                    status: ContextCompactionStatus::InProgress,
                                    error: None,
                                    summary: MessageContent::default(),
                                    meta: None,
                                },
                                cx,
                            );
                        })?;

                        Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::Cancelled))
                    } else {
                        Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                    }
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let first_request = thread.update(cx, |thread, cx| thread.send_raw("first", cx));
        assert_eq!(thread.read_with(cx, |thread, _| thread.turn_id), 1);

        let second_request = thread.update(cx, |thread, cx| thread.send_raw("second", cx));
        assert_eq!(thread.read_with(cx, |thread, _| thread.turn_id), 2);

        first_complete_tx
            .send(())
            .expect("first completion receiver should still be alive");

        let response = first_request
            .await
            .expect("first request should complete")
            .expect("first request should have response");
        assert_eq!(response.stop_reason, acp_v1::StopReason::Cancelled);

        thread.read_with(cx, |thread, _| {
            let compaction = thread
                .entries
                .iter()
                .find_map(|entry| {
                    let AgentThreadEntry::ContextCompaction(compaction) = entry else {
                        return None;
                    };
                    (compaction.id == compaction_id).then_some(compaction)
                })
                .expect("compaction entry should exist");

            assert_eq!(
                compaction.status,
                ContextCompactionStatus::InProgress,
                "a stale cancelled response from an older turn should not cancel current compaction"
            );
        });

        second_request
            .await
            .expect("second request should complete");
    }

    #[gpui::test]
    async fn test_send_omits_message_id_without_client_user_message_id_support(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let connection = Rc::new(FakeAgentConnection::new().without_truncate_support());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let response = thread
            .update(cx, |thread, cx| thread.send_raw("test message", cx))
            .await;

        assert!(response.is_ok(), "send should not fail: {response:?}");
        thread.read_with(cx, |thread, _| {
            let AgentThreadEntry::UserMessage(message) = &thread.entries[0] else {
                panic!("expected first entry to be a user message")
            };
            assert_eq!(message.identity, MessageIdentity::Legacy(None));
            assert_eq!(message.client_id, None);
            assert!(message.is_optimistic);
        });
    }

    #[gpui::test]
    async fn test_send_returns_cancelled_response_and_marks_tools_as_cancelled(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            move |_params, thread, mut cx| {
                async move {
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.handle_session_update(
                                acp_v1::SessionUpdate::ToolCall(
                                    acp_v1::ToolCall::new(
                                        acp_v1::ToolCallId::new("test-tool"),
                                        "Test Tool",
                                    )
                                    .kind(acp_v1::ToolKind::Fetch)
                                    .status(acp_v1::ToolCallStatus::InProgress),
                                ),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();

                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::Cancelled))
                }
                .boxed_local()
            },
        ));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let response = thread
            .update(cx, |thread, cx| thread.send_raw("test message", cx))
            .await;

        let response = response
            .expect("send should succeed")
            .expect("should have response");
        assert_eq!(
            response.stop_reason,
            acp_v1::StopReason::Cancelled,
            "response should have Cancelled stop_reason"
        );

        thread.read_with(cx, |thread, _| {
            let tool_entry = thread
                .entries
                .iter()
                .find_map(|e| {
                    if let AgentThreadEntry::ToolCall(call) = e {
                        Some(call)
                    } else {
                        None
                    }
                })
                .expect("should have tool call entry");

            assert!(
                matches!(tool_entry.status(), ToolCallStatus::Canceled),
                "tool should be marked as Canceled when response is Cancelled, got {:?}",
                tool_entry.status()
            );
        });
    }

    #[gpui::test]
    async fn test_provisional_title_replaced_by_real_title(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let set_title_calls = connection.set_title_calls.clone();

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        // Initial title is the default.
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.title(), None);
        });

        // Setting a provisional title updates the display title.
        thread.update(cx, |thread, cx| {
            thread.set_provisional_title("Hello, can you help…".into(), cx);
        });
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.title().as_ref().map(|s| s.as_str()),
                Some("Hello, can you help…")
            );
        });

        // The provisional title should NOT have propagated to the connection.
        assert_eq!(
            set_title_calls.borrow().len(),
            0,
            "provisional title should not propagate to the connection"
        );

        // When the real title arrives via set_title, it replaces the
        // provisional title and propagates to the connection.
        let task = thread.update(cx, |thread, cx| {
            thread.set_title("Helping with Rust question".into(), cx)
        });
        task.await.expect("set_title should succeed");
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.title().as_ref().map(|s| s.as_str()),
                Some("Helping with Rust question")
            );
        });
        assert_eq!(
            set_title_calls.borrow().as_slice(),
            &[SharedString::from("Helping with Rust question")],
            "real title should propagate to the connection"
        );
    }

    #[gpui::test]
    async fn test_session_info_update_replaces_provisional_title_and_emits_event(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());

        let thread = cx
            .update(|cx| {
                connection.clone().new_session(
                    project,
                    PathList::new(&[Path::new(path!("/test"))]),
                    cx,
                )
            })
            .await
            .unwrap();

        let title_updated_events = Rc::new(RefCell::new(0usize));
        let title_updated_events_for_subscription = title_updated_events.clone();
        thread.update(cx, |_thread, cx| {
            cx.subscribe(
                &thread,
                move |_thread, _event_thread, event: &AcpThreadEvent, _cx| {
                    if matches!(event, AcpThreadEvent::TitleUpdated) {
                        *title_updated_events_for_subscription.borrow_mut() += 1;
                    }
                },
            )
            .detach();
        });

        thread.update(cx, |thread, cx| {
            thread.set_provisional_title("Hello, can you help…".into(), cx);
        });
        assert_eq!(
            *title_updated_events.borrow(),
            1,
            "setting a provisional title should emit TitleUpdated"
        );

        let result = thread.update(cx, |thread, cx| {
            thread.handle_session_update(
                acp_v1::SessionUpdate::SessionInfoUpdate(
                    acp_v1::SessionInfoUpdate::new().title("Helping with Rust question"),
                ),
                cx,
            )
        });
        result.expect("session info update should succeed");

        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.title().as_ref().map(|s| s.as_str()),
                Some("Helping with Rust question")
            );
            assert!(
                !thread.has_provisional_title(),
                "session info title update should clear provisional title"
            );
        });

        assert_eq!(
            *title_updated_events.borrow(),
            2,
            "session info title update should emit TitleUpdated"
        );

        cx.run_until_parked();
        let observer_notifications = Rc::new(RefCell::new(0));
        let _observer = thread.update(cx, |_, cx| {
            cx.observe_self({
                let observer_notifications = observer_notifications.clone();
                move |_, _| *observer_notifications.borrow_mut() += 1
            })
        });
        thread.update(cx, |thread, cx| {
            thread.update_session_info(
                acp_v2::SessionInfoUpdate::new().updated_at("2026-03-04T05:06:07+02:30"),
                cx,
            );
            assert_eq!(thread.title(), Some("Helping with Rust question".into()));
            assert!(thread.session_info().updated_at.is_some());
            assert_eq!(thread.session_info().meta, None);
        });
        cx.run_until_parked();
        assert_eq!(*observer_notifications.borrow(), 1);
        assert_eq!(*title_updated_events.borrow(), 2);

        thread.update(cx, |thread, cx| {
            thread.update_session_info(
                acp_v2::SessionInfoUpdate::new().meta(acp_v2::Meta::from_iter([(
                    "session".into(),
                    serde_json::json!({"value": []}),
                )])),
                cx,
            );
            assert_eq!(thread.title(), Some("Helping with Rust question".into()));
            assert_eq!(
                thread.session_info().meta,
                Some(acp_v2::Meta::from_iter([(
                    "session".into(),
                    serde_json::json!({"value": []}),
                )]))
            );
        });
        cx.run_until_parked();
        assert_eq!(*observer_notifications.borrow(), 2);
        assert_eq!(*title_updated_events.borrow(), 2);

        thread.update(cx, |thread, cx| {
            thread.set_provisional_title("Hidden provisional title".into(), cx);
            thread.update_session_info(
                acp_v2::SessionInfoUpdate::new().title("Helping with Rust question"),
                cx,
            );
            assert!(!thread.has_provisional_title());
        });
        assert_eq!(*title_updated_events.borrow(), 4);

        thread.update(cx, |thread, cx| {
            thread.update_session_info(acp_v2::SessionInfoUpdate::new().title(None::<String>), cx);
            assert_eq!(thread.title(), None);
            assert_eq!(thread.session_info().title, None);
            assert!(thread.session_info().updated_at.is_some());
            assert!(thread.session_info().meta.is_some());
        });
        assert_eq!(*title_updated_events.borrow(), 5);

        thread.update(cx, |thread, cx| {
            thread.update_session_info(
                acp_v2::SessionInfoUpdate::new()
                    .title(None::<String>)
                    .updated_at(None::<String>)
                    .meta(None::<acp_v2::Meta>),
                cx,
            );
            assert_eq!(thread.session_info().updated_at, None);
            assert_eq!(thread.session_info().meta, None);
        });
        assert_eq!(*title_updated_events.borrow(), 5);

        thread.update(cx, |thread, cx| {
            thread.set_provisional_title("Visible provisional title".into(), cx);
            thread.update_session_info(acp_v2::SessionInfoUpdate::new(), cx);
            assert_eq!(thread.title(), Some("Visible provisional title".into()));
            thread.update_session_info(acp_v2::SessionInfoUpdate::new().title(None::<String>), cx);
            assert_eq!(thread.title(), None);
            assert!(!thread.has_provisional_title());
        });
        assert_eq!(*title_updated_events.borrow(), 7);

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::SessionInfoUpdate(
                        acp_v1::SessionInfoUpdate::new()
                            .title("Legacy title")
                            .meta(acp_v1::Meta::new()),
                    ),
                    cx,
                )
                .expect("legacy title patch");
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::SessionInfoUpdate(acp_v1::SessionInfoUpdate::new()),
                    cx,
                )
                .expect("omitted legacy patch");
            assert_eq!(thread.title(), Some("Legacy title".into()));
            assert_eq!(thread.session_info().meta, Some(acp_v2::Meta::new()));
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::SessionInfoUpdate(
                        acp_v1::SessionInfoUpdate::new().title(None::<String>),
                    ),
                    cx,
                )
                .expect("legacy title clear");
            assert_eq!(thread.title(), None);
            assert_eq!(thread.session_info().meta, Some(acp_v2::Meta::new()));
        });
        assert_eq!(*title_updated_events.borrow(), 9);

        assert!(
            connection.set_title_calls.borrow().is_empty(),
            "session info title update should not propagate back to the connection"
        );
    }

    #[gpui::test]
    async fn test_session_notices_are_live_and_independently_dismissible(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("session should be created");

        let notice_events = Rc::new(RefCell::new(0));
        let _subscription = thread.update(cx, |_, cx| {
            cx.subscribe_self({
                let notice_events = notice_events.clone();
                move |_, event, _| {
                    assert!(matches!(event, AcpThreadEvent::NoticesUpdated));
                    *notice_events.borrow_mut() += 1;
                }
            })
        });

        let warning =
            acp_v2::Notice::new(acp_v2::NoticeSeverity::Warning, "MCP server unavailable")
                .description("Continuing without it.");
        let notices = vec![
            acp_v2::Notice::new(acp_v2::NoticeSeverity::Info, "Using default configuration"),
            warning.clone(),
            warning.clone(),
            acp_v2::Notice::new(acp_v2::NoticeSeverity::Error, "Optional integration failed")
                .description("")
                .meta(acp_v2::Meta::new()),
            acp_v2::Notice::new(
                acp_v2::NoticeSeverity::Other("critical".into()),
                "Future severity",
            ),
            acp_v2::Notice::new(
                acp_v2::NoticeSeverity::Other("_custom".into()),
                "**Plain text**, not Markdown",
            )
            .meta(acp_v2::Meta::from_iter([(
                "extension".into(),
                serde_json::json!({"nested": [null, true, {"value": "retained"}]}),
            )])),
        ];

        thread.update(cx, |thread, cx| {
            for notice in &notices {
                thread.push_notice(notice.clone(), cx);
            }
            assert_eq!(
                thread.notices(),
                notices.iter().cloned().enumerate().collect::<Vec<_>>()
            );
            assert!(thread.entries().is_empty());
            assert!(thread.to_markdown(cx).is_empty());
            assert!(thread.is_draft_thread());
            assert_eq!(thread.status(), ThreadStatus::Idle);
            assert!(!thread.had_error());
            assert!(!thread.is_waiting_for_confirmation());
            assert!(thread.title().is_none());
        });
        assert_eq!(*notice_events.borrow(), notices.len());

        thread.update(cx, |thread, cx| thread.dismiss_notice(1, cx));
        assert_eq!(*notice_events.borrow(), notices.len() + 1);
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.notices(),
                notices
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter(|(id, _)| *id != 1)
                    .collect::<Vec<_>>()
            );
        });

        thread.update(cx, |thread, cx| thread.dismiss_notice(1, cx));
        assert_eq!(*notice_events.borrow(), notices.len() + 1);

        thread.update(cx, |thread, cx| {
            for notice_id in 0..notices.len() {
                thread.dismiss_notice(notice_id, cx);
            }
            assert!(thread.notices().is_empty());
            thread.push_notice(warning.clone(), cx);
            thread.dismiss_notice(1, cx);
            assert_eq!(thread.notices(), &[(notices.len(), warning)]);
            assert!(thread.entries().is_empty());
            assert!(thread.to_markdown(cx).is_empty());
        });
        assert_eq!(*notice_events.borrow(), notices.len() * 2 + 1);
    }

    #[gpui::test]
    async fn test_session_notice_error_does_not_interrupt_prompt(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            |_, thread, mut cx| {
                async move {
                    thread.update(&mut cx, |thread, cx| {
                        assert_eq!(thread.status(), ThreadStatus::Generating);
                        let history_before_notice = thread.to_markdown(cx);
                        let entry_count = thread.entries().len();
                        thread.handle_session_update(
                            acp_v1::SessionUpdate::Notice(
                                acp_v1::Notice::new(
                                    acp_v1::NoticeSeverity::Error,
                                    "Optional integration failed",
                                )
                                .description("Work will continue without the integration."),
                            ),
                            cx,
                        )?;
                        assert_eq!(thread.status(), ThreadStatus::Generating);
                        assert!(!thread.had_error());
                        assert_eq!(thread.entries().len(), entry_count);
                        assert_eq!(thread.to_markdown(cx), history_before_notice);
                        thread.handle_session_update(
                            acp_v1::SessionUpdate::AgentMessageChunk(acp_v1::ContentChunk::new(
                                "The response continues.".into(),
                            )),
                            cx,
                        )
                    })??;
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
                .boxed_local()
            },
        ));
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .expect("session should be created");

        thread
            .update(cx, |thread, cx| thread.send_raw("hello", cx))
            .await
            .expect("advisory errors must not fail prompts");

        thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.status(), ThreadStatus::Idle);
            assert!(!thread.had_error());
            assert_eq!(thread.notices().len(), 1);
            assert_eq!(thread.entries().len(), 2);
            assert!(thread.to_markdown(cx).contains("The response continues."));
            assert!(
                !thread
                    .to_markdown(cx)
                    .contains("Optional integration failed")
            );
        });
    }

    #[gpui::test]
    async fn test_usage_update_populates_token_usage_and_cost(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UsageUpdate(
                        acp_v1::UsageUpdate::new(5000, 10000).cost(acp_v1::Cost::new(0.42, "USD")),
                    ),
                    cx,
                )
                .unwrap();
        });

        thread.read_with(cx, |thread, _| {
            let usage = thread.token_usage().expect("token_usage should be set");
            assert_eq!(usage.max_tokens, 10000);
            assert_eq!(usage.used_tokens, 5000);

            let cost = thread.cost().expect("cost should be set");
            assert!((cost.amount - 0.42).abs() < f64::EPSILON);
            assert_eq!(cost.currency.as_ref(), "USD");
        });
    }

    #[gpui::test]
    async fn test_context_compaction_preserves_token_usage(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UsageUpdate(
                        acp_v1::UsageUpdate::new(5000, 10000).cost(acp_v1::Cost::new(0.42, "USD")),
                    ),
                    cx,
                )
                .unwrap();

            thread.push_context_compaction(
                ContextCompaction {
                    id: ContextCompactionId("compaction-1".into()),
                    status: ContextCompactionStatus::InProgress,
                    error: None,
                    summary: MessageContent::default(),
                    meta: None,
                },
                cx,
            );
        });

        thread.read_with(cx, |thread, _| {
            let usage = thread
                .token_usage()
                .expect("context compaction should not clear token usage on its own");
            assert_eq!(usage.used_tokens, 5000);
            assert_eq!(usage.max_tokens, 10000);

            let cost = thread
                .cost()
                .expect("context compaction should not clear cost on its own");
            assert!((cost.amount - 0.42).abs() < f64::EPSILON);
        });

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UsageUpdate(acp_v1::UsageUpdate::new(1000, 10000)),
                    cx,
                )
                .unwrap();
        });

        thread.read_with(cx, |thread, _| {
            let usage = thread
                .token_usage()
                .expect("token_usage should be restored by the next usage update");
            assert_eq!(usage.used_tokens, 1000);
            assert_eq!(usage.max_tokens, 10000);
        });
    }

    #[gpui::test]
    async fn test_usage_update_without_cost_preserves_existing_cost(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UsageUpdate(
                        acp_v1::UsageUpdate::new(1000, 10000).cost(acp_v1::Cost::new(0.10, "USD")),
                    ),
                    cx,
                )
                .unwrap();

            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UsageUpdate(acp_v1::UsageUpdate::new(2000, 10000)),
                    cx,
                )
                .unwrap();
        });

        thread.read_with(cx, |thread, _| {
            let usage = thread.token_usage().expect("token_usage should be set");
            assert_eq!(usage.used_tokens, 2000);

            let cost = thread.cost().expect("cost should be preserved");
            assert!((cost.amount - 0.10).abs() < f64::EPSILON);
        });
    }

    #[gpui::test]
    async fn test_response_usage_does_not_clobber_session_usage(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            move |_, thread, mut cx| {
                async move {
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp_v1::SessionUpdate::UsageUpdate(
                                    acp_v1::UsageUpdate::new(3000, 10000)
                                        .cost(acp_v1::Cost::new(0.05, "EUR")),
                                ),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)
                        .usage(acp_v1::Usage::new(500, 200, 300)))
                }
                .boxed_local()
            },
        ));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread
            .update(cx, |thread, cx| thread.send_raw("hello", cx))
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            let usage = thread.token_usage().expect("token_usage should be set");
            assert_eq!(usage.max_tokens, 10000, "max_tokens from UsageUpdate");
            assert_eq!(usage.used_tokens, 3000, "used_tokens from UsageUpdate");
            assert_eq!(usage.input_tokens, 200, "input_tokens from response usage");
            assert_eq!(
                usage.output_tokens, 300,
                "output_tokens from response usage"
            );

            let cost = thread.cost().expect("cost should be set");
            assert!((cost.amount - 0.05).abs() < f64::EPSILON);
            assert_eq!(cost.currency.as_ref(), "EUR");
        });
    }

    #[gpui::test]
    async fn test_clearing_token_usage_also_clears_cost(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        thread.update(cx, |thread, cx| {
            thread
                .handle_session_update(
                    acp_v1::SessionUpdate::UsageUpdate(
                        acp_v1::UsageUpdate::new(1000, 10000).cost(acp_v1::Cost::new(0.25, "USD")),
                    ),
                    cx,
                )
                .unwrap();

            assert!(thread.token_usage().is_some());
            assert!(thread.cost().is_some());

            thread.update_token_usage(None, cx);

            assert!(thread.token_usage().is_none());
            assert!(
                thread.cost().is_none(),
                "cost should be cleared when token usage is cleared"
            );
        });
    }

    /// Regression test: if the inner send_task is cancelled before it can
    /// fire `tx.send(...)` (e.g. because the underlying future was dropped),
    /// the outer task observes `rx.await` returning `Err(Cancelled)` and
    /// must still clear `running_turn` so the panel transitions out of
    /// `Generating`. Without this, the agent thread is wedged in the
    /// loading state until Zed restarts.
    #[gpui::test]
    async fn test_running_turn_cleared_when_send_task_dropped(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        // Handler hangs forever so the spawn at run_turn is parked inside
        // `f(this, cx).await` with `tx` still alive but unsent.
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            |_params, _thread, _cx| {
                async move { futures::future::pending::<Result<acp_v1::PromptResponse>>().await }
                    .boxed_local()
            },
        ));

        let thread = cx
            .update(|cx| {
                connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
            })
            .await
            .unwrap();

        let request = thread.update(cx, |thread, cx| thread.send_raw("hello", cx));
        cx.run_until_parked();

        assert_eq!(
            thread.read_with(cx, |t, _| t.status()),
            ThreadStatus::Generating,
            "thread should be generating while the handler is parked"
        );

        // Replace the in-flight send_task with a no-op. Dropping the original
        // Task cancels its inner future, which drops `tx` without ever calling
        // `tx.send(...)`. This mirrors the production scenario where the
        // send_task future is cancelled before completion.
        thread.update(cx, |thread, _| {
            thread.running_turn.as_mut().unwrap().send_task = Task::ready(());
        });

        let result = request.await;
        assert!(
            matches!(result, Ok(None)),
            "outer task should resolve to Ok(None) on dropped tx, got {result:?}"
        );

        assert_eq!(
            thread.read_with(cx, |t, _| t.status()),
            ThreadStatus::Idle,
            "running_turn must be cleared even when tx was dropped without send"
        );
    }

    #[gpui::test]
    async fn test_dropped_completion_waiter_does_not_leave_thread_generating(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, completion) = oneshot::channel::<()>();
        let backend_finished = Rc::new(AtomicBool::new(false));
        let statuses = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let statuses = statuses.clone();
                move |thread, event, cx| {
                    if matches!(event, AcpThreadEvent::StatusChanged) {
                        statuses.borrow_mut().push(thread.read(cx).status());
                    }
                }
            })
        });
        let request = thread.update(cx, |thread, cx| {
            let id = thread.register_submission(Arc::from([]), cx);
            thread.run_turn(id, cx, {
                let backend_finished = backend_finished.clone();
                async move |_, _| {
                    completion.await?;
                    backend_finished.store(true, SeqCst);
                    Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn))
                }
            })
        });
        cx.run_until_parked();
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.status()),
            ThreadStatus::Generating
        );
        assert!(!backend_finished.load(SeqCst));

        let submission_id = request.id;
        drop(request);
        cx.run_until_parked();
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.status()),
            ThreadStatus::Generating
        );
        complete.send(()).expect("backend should still be running");
        cx.run_until_parked();

        assert!(backend_finished.load(SeqCst));
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.status()),
            ThreadStatus::Idle
        );
        assert_eq!(
            *statuses.borrow(),
            vec![ThreadStatus::Generating, ThreadStatus::Idle]
        );
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread
                    .submission(submission_id)
                    .expect("settled submission")
                    .state,
                SubmissionState::Completed,
            ));
        });
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_released_on_turn_completion(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        assert!(cx.read(|cx| AgentSettings::get_global(cx).prevent_idle_sleep));
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        for (delay, expected_count) in [(Duration::ZERO, 1), (Duration::from_secs(2), 0)] {
            cx.set_idle_sleep_prevention_delay(delay);
            for (backend_result, expected_result, expected_had_error) in [
                (
                    Ok(acp_v1::StopReason::EndTurn),
                    Ok(acp_v1::StopReason::EndTurn),
                    false,
                ),
                (
                    Ok(acp_v1::StopReason::Cancelled),
                    Ok(acp_v1::StopReason::Cancelled),
                    false,
                ),
                (
                    Ok(acp_v1::StopReason::Refusal),
                    Ok(acp_v1::StopReason::Refusal),
                    true,
                ),
                (
                    Ok(acp_v1::StopReason::MaxTokens),
                    Err("output token limit reached"),
                    true,
                ),
                (Err(anyhow!("backend failed")), Err("backend failed"), true),
            ] {
                let (complete, request) = start_test_turn(&thread, cx);
                cx.run_until_parked();
                assert_eq!(cx.active_idle_sleep_preventions(), expected_count);
                assert_eq!(
                    thread.read_with(cx, |thread, _| thread.status()),
                    ThreadStatus::Generating
                );
                assert!(!thread.read_with(cx, |thread, _| thread.had_error()));

                complete
                    .send(backend_result.map(acp_v1::PromptResponse::new))
                    .expect("turn should still be running");
                let result = request
                    .await
                    .map(|response| response.map(|response| response.stop_reason))
                    .map_err(|error| error.to_string());
                assert_eq!(result, expected_result.map(Some).map_err(String::from));
                assert_eq!(cx.active_idle_sleep_preventions(), 0);
                thread.read_with(cx, |thread, _| {
                    assert_eq!(thread.status(), ThreadStatus::Idle);
                    assert_eq!(thread.had_error(), expected_had_error);
                    assert!(matches!(
                        thread.idle_sleep_prevention,
                        IdleSleepPrevention::Inactive
                    ));
                });
                cx.executor().advance_clock(Duration::from_secs(2));
                cx.run_until_parked();
                assert_eq!(cx.active_idle_sleep_preventions(), 0);
            }
        }
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_overlaps_distinct_threads_and_subagent(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let parent = new_test_thread(cx).await;
        let independent = new_test_thread(cx).await;
        let subagent = cx.update(|cx| {
            let parent = parent.read(cx);
            let session_id = parent.session_id().clone();
            let connection = parent.connection.clone();
            let project = parent.project.clone();
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            cx.new(|cx| {
                AcpThread::new(
                    Some(session_id),
                    None,
                    None,
                    connection,
                    project,
                    action_log,
                    acp_v2::SessionId::new("subagent"),
                    watch::Receiver::constant(acp_v2::PromptCapabilities::new()),
                    cx,
                )
            })
        });
        assert_ne!(parent.entity_id(), independent.entity_id());
        assert_ne!(parent.entity_id(), subagent.entity_id());
        assert_ne!(independent.entity_id(), subagent.entity_id());
        assert_eq!(
            subagent.read_with(cx, |thread, _| thread.parent_session_id().cloned()),
            Some(parent.read_with(cx, |thread, _| thread.session_id().clone()))
        );

        let (parent_complete, parent_request) = start_test_turn(&parent, cx);
        let (independent_complete, independent_request) = start_test_turn(&independent, cx);
        let (subagent_complete, subagent_request) = start_test_turn(&subagent, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 3);

        let parent_cancel = parent.update(cx, |thread, cx| thread.cancel(cx));
        assert_eq!(cx.active_idle_sleep_preventions(), 2);
        parent_complete
            .send(Ok(acp_v1::PromptResponse::new(
                acp_v1::StopReason::Cancelled,
            )))
            .expect("parent backend should still be running");
        parent_cancel.await;
        parent_request.await.expect("parent turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 2);

        independent_complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("independent turn should still be running");
        independent_request
            .await
            .expect("independent turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        assert_eq!(
            subagent.read_with(cx, |thread, _| thread.status()),
            ThreadStatus::Generating
        );

        subagent_complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("subagent should still be running");
        subagent_request
            .await
            .expect("subagent turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_cancel_releases_pending_idle_sleep_prevention(cx: &mut TestAppContext) {
        init_test(cx);
        for (delay, acquisition_fails, expected_count) in [
            (Duration::ZERO, false, 1),
            (Duration::from_secs(2), false, 0),
            (Duration::from_secs(2), true, 0),
        ] {
            cx.set_idle_sleep_prevention_delay(delay);
            cx.set_idle_sleep_prevention_fails(acquisition_fails);
            let thread = new_test_thread(cx).await;
            let (complete, request) = start_test_turn(&thread, cx);
            assert_eq!(cx.active_idle_sleep_preventions(), expected_count);
            thread.read_with(cx, |thread, _| {
                assert!(matches!(
                    thread.idle_sleep_prevention,
                    IdleSleepPrevention::Acquiring { .. }
                ));
            });

            let cancel = thread.update(cx, |thread, cx| thread.cancel(cx));
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_secs(2));
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            thread.read_with(cx, |thread, _| {
                assert_eq!(thread.status(), ThreadStatus::Idle);
                assert!(matches!(
                    thread.idle_sleep_prevention,
                    IdleSleepPrevention::Inactive
                ));
            });

            complete
                .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                .expect("backend should still be running");
            cancel.await;
            request.await.expect("turn should complete");
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
    }

    #[gpui::test]
    async fn test_setting_disables_pending_idle_sleep_prevention(cx: &mut TestAppContext) {
        init_test(cx);
        cx.set_idle_sleep_prevention_delay(Duration::from_secs(2));
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        set_prevent_idle_sleep(false, cx);
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.status(), ThreadStatus::Generating);
            assert!(matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Inactive
            ));
        });

        set_prevent_idle_sleep(true, cx);
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        set_prevent_idle_sleep(true, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Active { .. }
            ));
        });

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_follow_up_keeps_pending_idle_sleep_prevention(cx: &mut TestAppContext) {
        init_test(cx);
        cx.set_idle_sleep_prevention_delay(Duration::from_secs(2));
        let thread = new_test_thread(cx).await;
        let (first_complete, first_request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        let (second_complete, second_request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.running_turn.as_ref().map(|turn| turn.id), Some(2));
            assert!(matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Acquiring { .. }
            ));
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Active { .. }
            ));
        });

        first_complete
            .send(Ok(acp_v1::PromptResponse::new(
                acp_v1::StopReason::Cancelled,
            )))
            .expect("first backend should still be running");
        first_request.await.expect("first turn should complete");
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.running_turn.as_ref().map(|turn| turn.id), Some(2));
            assert!(matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Active { .. }
            ));
        });

        second_complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("second backend should still be running");
        second_request.await.expect("second turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_dropping_thread_releases_idle_sleep_prevention(cx: &mut TestAppContext) {
        init_test(cx);
        for (delay, expected_count) in [(Duration::ZERO, 1), (Duration::from_secs(2), 0)] {
            cx.set_idle_sleep_prevention_delay(delay);
            let thread = new_test_thread(cx).await;
            let weak_thread = thread.downgrade();
            let (complete, request) = start_test_turn(&thread, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), expected_count);

            cx.update(|_| drop(thread));
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_secs(2));
            cx.run_until_parked();
            assert!(weak_thread.upgrade().is_none());
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            assert!(request.await.is_err());
            drop(complete);
        }
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_failure_retries_only_on_setting_toggle(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for delay in [Duration::ZERO, Duration::from_secs(2)] {
            cx.set_idle_sleep_prevention_delay(delay);
            cx.set_idle_sleep_prevention_fails(true);
            let thread = new_test_thread(cx).await;
            let (complete, request) = start_test_turn(&thread, cx);
            cx.run_until_parked();
            cx.executor().advance_clock(delay);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            thread.read_with(cx, |thread, _| {
                assert!(matches!(
                    thread.idle_sleep_prevention,
                    IdleSleepPrevention::Failed
                ));
                assert_eq!(thread.status(), ThreadStatus::Generating);
                assert!(!thread.had_error());
            });

            cx.set_idle_sleep_prevention_fails(false);
            set_prevent_idle_sleep(true, cx);
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_secs(4));
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            assert!(thread.read_with(cx, |thread, _| matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Failed
            )));

            set_prevent_idle_sleep(false, cx);
            cx.run_until_parked();
            assert!(!thread.read_with(cx, |thread, _| matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Failed
            )));
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            set_prevent_idle_sleep(true, cx);
            cx.run_until_parked();
            cx.executor().advance_clock(delay);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 1);
            thread.read_with(cx, |thread, _| {
                assert!(matches!(
                    thread.idle_sleep_prevention,
                    IdleSleepPrevention::Active { .. }
                ));
            });

            complete
                .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                .expect("turn should still be running");
            request.await.expect("turn should complete");
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_failure_clears_when_turn_stops(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        for cancel_turn in [false, true] {
            cx.set_idle_sleep_prevention_fails(true);
            let (complete, request) = start_test_turn(&thread, cx);
            cx.run_until_parked();
            assert!(thread.read_with(cx, |thread, _| matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Failed
            )));

            let cancel = if cancel_turn {
                let cancel = thread.update(cx, |thread, cx| thread.cancel(cx));
                thread.read_with(cx, |thread, _| {
                    assert!(matches!(
                        thread.idle_sleep_prevention,
                        IdleSleepPrevention::Inactive
                    ));
                    assert_eq!(thread.status(), ThreadStatus::Idle);
                });
                Some(cancel)
            } else {
                None
            };
            complete
                .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                .expect("backend should still be running");
            if let Some(cancel) = cancel {
                cancel.await;
            }
            request.await.expect("turn should complete");
            thread.read_with(cx, |thread, _| {
                assert!(matches!(
                    thread.idle_sleep_prevention,
                    IdleSleepPrevention::Inactive
                ));
                assert_eq!(thread.status(), ThreadStatus::Idle);
            });
            assert_eq!(cx.active_idle_sleep_preventions(), 0);

            cx.set_idle_sleep_prevention_fails(false);
            let (complete, request) = start_test_turn(&thread, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 1);
            complete
                .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                .expect("next turn should still be running");
            request.await.expect("next turn should complete");
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_waits_for_all_confirmations(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        let first_id = acp_v2::ToolCallId::new("first-permission");
        let second_id = acp_v2::ToolCallId::new("second-permission");
        let first_permission = request_test_permission(&thread, first_id.clone(), cx);
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        let second_permission = request_test_permission(&thread, second_id.clone(), cx);
        let (elicitation_id, elicitation_response) = request_test_form_elicitation(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        for enabled in [false, true] {
            set_prevent_idle_sleep(enabled, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
        thread.update(cx, |thread, cx| {
            thread
                .update_tool_call(
                    acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(first_id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new()
                            .status(acp_v1::ToolCallStatus::InProgress),
                    ),
                    cx,
                )
                .expect("tool update should succeed");
            thread.authorize_tool_call(
                second_id,
                SelectedPermissionOutcome::new(
                    acp_v2::PermissionOptionId::new("reject"),
                    acp_v2::PermissionOptionKind::RejectOnce,
                ),
                cx,
            );
        });
        assert!(matches!(
            second_permission.await,
            RequestPermissionOutcome::Selected(outcome)
                if outcome.option_kind == acp_v2::PermissionOptionKind::RejectOnce
        ));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Decline),
                cx,
            );
        });
        assert_eq!(
            elicitation_response.await.action,
            acp_v2::ElicitationAction::Decline
        );
        cx.run_until_parked();
        assert!(thread.read_with(cx, |thread, _| thread.is_waiting_for_confirmation()));
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                first_id,
                SelectedPermissionOutcome::new(
                    acp_v2::PermissionOptionId::new("allow"),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });
        assert!(matches!(
            first_permission.await,
            RequestPermissionOutcome::Selected(outcome)
                if outcome.option_kind == acp_v2::PermissionOptionKind::AllowOnce
        ));
        cx.run_until_parked();
        assert!(!thread.read_with(cx, |thread, _| thread.is_waiting_for_confirmation()));
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_resumes_after_permission_cancellation_or_terminal_update(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for status in [
            None,
            Some(acp_v1::ToolCallStatus::Completed),
            Some(acp_v1::ToolCallStatus::Failed),
        ] {
            let thread = new_test_thread(cx).await;
            let (complete, request) = start_test_turn(&thread, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 1);
            let tool_call_id = acp_v2::ToolCallId::new("permission");
            let permission = request_test_permission(&thread, tool_call_id.clone(), cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);

            thread.update(cx, |thread, cx| {
                if let Some(status) = status {
                    thread
                        .handle_session_update(
                            acp_v1::SessionUpdate::ToolCallUpdate(acp_v1::ToolCallUpdate::new(
                                acp_v1::ToolCallId::new(tool_call_id.0),
                                acp_v1::ToolCallUpdateFields::new().status(status),
                            )),
                            cx,
                        )
                        .expect("terminal tool update should succeed");
                } else {
                    thread.cancel_tool_call_authorization(&tool_call_id, cx);
                }
            });
            assert!(matches!(
                permission.await,
                RequestPermissionOutcome::Cancelled
            ));
            cx.run_until_parked();
            assert!(!thread.read_with(cx, |thread, _| thread.is_waiting_for_confirmation()));
            assert_eq!(cx.active_idle_sleep_preventions(), 1, "status: {status:?}");

            complete
                .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                .expect("turn should still be running");
            request.await.expect("turn should complete");
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
    }

    #[gpui::test]
    async fn test_permission_suspends_pending_idle_sleep_prevention(cx: &mut TestAppContext) {
        init_test(cx);
        cx.set_idle_sleep_prevention_delay(Duration::from_secs(2));
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        let tool_call_id = acp_v2::ToolCallId::new("permission");
        let permission = request_test_permission(&thread, tool_call_id.clone(), cx);
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        thread.read_with(cx, |thread, _| {
            assert!(thread.is_waiting_for_confirmation());
            assert!(matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Inactive
            ));
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_call_id,
                SelectedPermissionOutcome::new(
                    acp_v2::PermissionOptionId::new("allow"),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });
        assert!(matches!(
            permission.await,
            RequestPermissionOutcome::Selected(_)
        ));
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_cancel_elicitation_after_waiter_drops_resumes_idle_sleep_prevention(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        let (elicitation_id, response) = request_test_form_elicitation(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        drop(response);
        cx.run_until_parked();

        thread.update(cx, |thread, cx| {
            thread.cancel_elicitation(&elicitation_id, cx)
        });
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            let (_, elicitation) = thread.elicitation(&elicitation_id).expect("elicitation");
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
            assert!(!thread.is_waiting_for_confirmation());
            assert_eq!(thread.status(), ThreadStatus::Generating);
        });
        assert_eq!(
            cx.active_idle_sleep_preventions(),
            1,
            "cancelling the request must refresh generation state even without a response waiter"
        );

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_resumes_after_form_elicitation(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        for action in [
            acp_v2::ElicitationAction::Accept(acp_v2::ElicitationAcceptAction::new()),
            acp_v2::ElicitationAction::Decline,
            acp_v2::ElicitationAction::Cancel,
        ] {
            let (elicitation_id, response) = request_test_form_elicitation(&thread, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            assert_eq!(
                thread.read_with(cx, |thread, _| thread.status()),
                ThreadStatus::Generating
            );

            thread.update(cx, |thread, cx| {
                thread.respond_to_elicitation(
                    &elicitation_id,
                    acp_v2::CreateElicitationResponse::new(action.clone()),
                    cx,
                );
            });
            assert_eq!(response.await.action, action);
            cx.run_until_parked();
            assert!(!thread.read_with(cx, |thread, _| thread.is_waiting_for_confirmation()));
            assert_eq!(cx.active_idle_sleep_preventions(), 1);

            thread.update(cx, |thread, cx| {
                thread.respond_to_elicitation(
                    &elicitation_id,
                    acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Cancel),
                    cx,
                );
            });
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 1);
        }

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_idle_sleep_prevention_resumes_after_url_elicitation(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);

        for (url_id, accept) in [("accepted", true), ("canceled", false)] {
            let url_id = acp_v2::ElicitationId::new(url_id);
            let (entry_id, response) = thread.update(cx, |thread, cx| {
                thread
                    .request_elicitation_with_id(
                        acp_v2::CreateElicitationRequest::new(
                            acp_v2::ElicitationUrlMode::new(
                                acp_v2::ElicitationSessionScope::new(thread.session_id().clone()),
                                url_id.clone(),
                                "https://example.com/complete",
                            ),
                            "Complete this in the browser",
                        ),
                        cx,
                    )
                    .expect("URL elicitation should succeed")
            });
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            thread.update(cx, |thread, cx| {
                thread.complete_url_elicitation(&url_id, cx)
            });
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 0);

            let expected_action = if accept {
                acp_v2::ElicitationAction::Accept(acp_v2::ElicitationAcceptAction::new())
            } else {
                acp_v2::ElicitationAction::Cancel
            };
            thread.update(cx, |thread, cx| {
                if accept {
                    thread.respond_to_elicitation(
                        &entry_id,
                        acp_v2::CreateElicitationResponse::new(expected_action.clone()),
                        cx,
                    );
                } else {
                    thread.cancel_elicitation(&entry_id, cx);
                }
            });
            assert_eq!(response.await.action, expected_action);
            cx.run_until_parked();
            assert!(!thread.read_with(cx, |thread, _| thread.is_waiting_for_confirmation()));
            assert_eq!(cx.active_idle_sleep_preventions(), 1);

            thread.update(cx, |thread, cx| {
                thread.complete_url_elicitation(&url_id, cx)
            });
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 1);
        }

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_confirmation_resolution_does_not_reacquire_idle_sleep_prevention_after_cancel(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        let tool_call_id = acp_v2::ToolCallId::new("permission");
        let (permission_id, permission) =
            request_test_permission_with_id(&thread, tool_call_id.clone(), cx);
        let (elicitation_id, elicitation_response) = request_test_form_elicitation(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        let cancel = thread.update(cx, |thread, cx| thread.cancel(cx));
        thread.read_with(cx, |thread, _| {
            assert!(thread.permission_request(permission_id).is_none());
            assert!(thread.permission_request_for_tool(&tool_call_id).is_none());
            assert_eq!(thread.pending_permission_requests().count(), 0);
            assert!(
                thread
                    .tool_call(&tool_call_id)
                    .expect("cancelled tool")
                    .1
                    .authorization_id()
                    .is_none()
            );
        });
        assert!(matches!(
            permission.await,
            RequestPermissionOutcome::Cancelled
        ));
        assert_eq!(
            elicitation_response.await.action,
            acp_v2::ElicitationAction::Cancel
        );
        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_call_id,
                SelectedPermissionOutcome::new(
                    acp_v2::PermissionOptionId::new("allow"),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
            thread.respond_to_elicitation(
                &elicitation_id,
                acp_v2::CreateElicitationResponse::new(acp_v2::ElicitationAction::Accept(
                    acp_v2::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.status()),
            ThreadStatus::Idle
        );
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        complete
            .send(Ok(acp_v1::PromptResponse::new(
                acp_v1::StopReason::Cancelled,
            )))
            .expect("backend should still be running");
        cancel.await;
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_stale_refusal_does_not_affect_follow_up_turn(cx: &mut TestAppContext) {
        assert_stale_completion_does_not_affect_follow_up_turn(Ok(acp_v1::StopReason::Refusal), cx)
            .await;
    }

    #[gpui::test]
    async fn test_stale_error_does_not_affect_follow_up_turn(cx: &mut TestAppContext) {
        assert_stale_completion_does_not_affect_follow_up_turn(Err("first turn failed"), cx).await;
    }

    #[gpui::test]
    async fn test_completed_turn_reclaims_source_capacity_without_changing_content(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let text = acp_v2::ContentBlock::Text(
            acp_v2::TextContent::new("before")
                .meta(acp_v2::Meta::from_iter([("sequence".into(), json!(1))])),
        );
        let image = message_test_image();

        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(None, "older user".into(), cx);
            thread.push_assistant_content_block("older reply".into(), false, cx);
            thread.push_user_content_block(None, "prompt".into(), cx);
            for index in [0, 2] {
                test_message_content_mut(thread, index)
                    .source_blocks
                    .reserve(32);
            }
        });
        let (complete, request) = start_test_turn(&thread, cx);
        let (markdown, rendered_image) = thread.update(cx, |thread, cx| {
            thread.push_user_content_block(None, " continuation".into(), cx);
            thread.push_assistant_content_block(image.clone(), false, cx);
            thread.push_assistant_content_block(text.clone(), false, cx);
            thread.push_assistant_content_block(" buffered".into(), false, cx);
            assert!(thread.streaming_text_buffer.is_some());
            let content = test_message_content(thread, 3);
            let markdown = content
                .blocks()
                .find_map(|block| block.markdown().cloned())
                .expect("assistant text");
            assert_eq!(markdown.read(cx).source(), "before");
            let rendered_image = content
                .blocks()
                .find_map(|block| block.image().map(|(image, _)| image.clone()))
                .expect("assistant image");
            for index in [2, 3] {
                test_message_content_mut(thread, index)
                    .source_blocks
                    .reserve(32);
            }
            for index in [0, 2, 3] {
                let content = test_message_content(thread, index);
                assert!(content.source_blocks.capacity() > content.source_blocks.len());
            }
            (markdown, rendered_image)
        });

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        request.await.expect("turn should complete");
        thread.read_with(cx, |thread, cx| {
            let older = test_message_content(thread, 0);
            assert!(older.source_blocks.capacity() > older.source_blocks.len());
            assert_eq!(older.source_blocks(), &["older user".into()]);
            let user = test_message_content(thread, 2);
            assert_eq!(user.source_blocks.capacity(), user.source_blocks.len());
            assert_eq!(
                user.source_blocks(),
                &["prompt".into(), " continuation".into()]
            );
            let assistant = test_message_content(thread, 3);
            assert_eq!(
                assistant.source_blocks.capacity(),
                assistant.source_blocks.len()
            );
            assert_eq!(
                assistant.source_blocks(),
                &[image.clone(), text.clone(), " buffered".into()]
            );
            assert_eq!(
                assistant.blocks().find_map(|block| block.markdown()),
                Some(&markdown)
            );
            assert_eq!(markdown.read(cx).source(), "before buffered");
            assert!(assistant.blocks().any(|block| {
                block
                    .image()
                    .is_some_and(|(image, _)| Arc::ptr_eq(image, &rendered_image))
            }));
        });

        thread.update(cx, |thread, cx| {
            thread.push_assistant_content_block(" after".into(), false, cx);
            let assistant = test_message_content(thread, 3);
            assert_eq!(
                assistant.source_blocks(),
                &[
                    image.clone(),
                    text.clone(),
                    " buffered".into(),
                    " after".into()
                ]
            );
            assert_eq!(
                assistant.blocks().find_map(|block| block.markdown()),
                Some(&markdown)
            );
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        markdown.read_with(cx, |markdown, _| {
            assert_eq!(markdown.source(), "before buffered after");
        });
    }

    #[gpui::test]
    async fn test_cancelled_turn_reclaims_source_capacity_before_backend_finishes(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(None, "prompt".into(), cx);
            thread.push_assistant_content_block("reply".into(), false, cx);
            thread.push_assistant_content_block(" pending".into(), false, cx);
            for index in [0, 1] {
                test_message_content_mut(thread, index)
                    .source_blocks
                    .reserve(32);
                let content = test_message_content(thread, index);
                assert!(content.source_blocks.capacity() > content.source_blocks.len());
            }
        });
        let cancel = thread.update(cx, |thread, cx| thread.cancel(cx));
        thread.read_with(cx, |thread, cx| {
            for index in [0, 1] {
                let content = test_message_content(thread, index);
                assert_eq!(
                    content.source_blocks.capacity(),
                    content.source_blocks.len()
                );
            }
            assert_eq!(
                test_message_content(thread, 0).source_blocks(),
                &["prompt".into()]
            );
            assert_eq!(
                test_message_content(thread, 1).source_blocks(),
                &["reply".into(), " pending".into()]
            );
            assert_eq!(
                test_message_content(thread, 1).to_markdown(cx),
                "reply pending"
            );
        });
        complete
            .send(Ok(acp_v1::PromptResponse::new(
                acp_v1::StopReason::Cancelled,
            )))
            .expect("backend should still be running");
        cancel.await;
        request.await.expect("turn should complete");
    }

    fn test_message_content(thread: &AcpThread, entry_index: usize) -> &MessageContent {
        match thread.entries.get(entry_index).expect("message entry") {
            AgentThreadEntry::UserMessage(message) => &message.content,
            AgentThreadEntry::AssistantMessage(message) => {
                let Some(AssistantMessageChunk::Message { block, .. }) = message.chunks.last()
                else {
                    panic!("expected assistant message chunk");
                };
                block
            }
            _ => panic!("expected message entry"),
        }
    }

    fn test_message_content_mut(thread: &mut AcpThread, entry_index: usize) -> &mut MessageContent {
        match thread.entries.get_mut(entry_index).expect("message entry") {
            AgentThreadEntry::UserMessage(message) => &mut message.content,
            AgentThreadEntry::AssistantMessage(message) => {
                let Some(AssistantMessageChunk::Message { block, .. }) = message.chunks.last_mut()
                else {
                    panic!("expected assistant message chunk");
                };
                block
            }
            _ => panic!("expected message entry"),
        }
    }

    #[gpui::test]
    async fn test_failed_tool_update_resumes_sleep_prevention(cx: &mut TestAppContext) {
        assert_failed_tool_update_resumes_sleep_prevention(false, cx).await;
    }

    #[gpui::test]
    async fn test_failed_tool_upsert_resumes_sleep_prevention(cx: &mut TestAppContext) {
        assert_failed_tool_update_resumes_sleep_prevention(true, cx).await;
    }

    #[gpui::test]
    async fn test_v2_terminal_reference_settles_only_its_permission(cx: &mut TestAppContext) {
        init_test(cx);
        for status in [
            acp_v2::ToolCallStatus::Completed,
            acp_v2::ToolCallStatus::Failed,
            acp_v2::ToolCallStatus::Cancelled,
        ] {
            let thread = new_test_thread(cx).await;
            let (complete, request) = start_test_turn(&thread, cx);
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 1);
            let first_id = acp_v2::ToolCallId::new("first");
            let second_id = acp_v2::ToolCallId::new("second");
            let first_permission = request_test_permission(&thread, first_id.clone(), cx);
            let second_permission = request_test_permission(&thread, second_id.clone(), cx);
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
            let updated_entries = Rc::new(RefCell::new(Vec::new()));
            let _subscription = cx.update(|cx| {
                cx.subscribe(&thread, {
                    let updated_entries = updated_entries.clone();
                    move |_, event, _| {
                        if let AcpThreadEvent::EntryUpdated(index) = event {
                            updated_entries.borrow_mut().push(*index);
                        }
                    }
                })
            });
            let first_index = thread.update(cx, |thread, cx| {
                let (index, first) = thread.tool_call(&first_id).expect("first tool");
                let original_label = first.label.clone();
                thread
                    .upsert_wire_tool_call(
                        acp_v2::ToolCallUpdate::new("first")
                            .status(status.clone())
                            .title("Terminal result")
                            .content(vec![acp_v2::ToolCallContent::Terminal(
                                acp_v2::Terminal::new("unseen"),
                            )]),
                        cx,
                    )
                    .expect("unseen terminal references create placeholders");
                let (_, first) = thread.tool_call(&first_id).expect("updated tool");
                assert_eq!(first.reported_status.as_ref(), Some(&status));
                assert!(first.authorization_id().is_none());
                assert!(thread.permission_request_for_tool(&first_id).is_none());
                assert_eq!(first.label, original_label);
                assert_eq!(first.label.read(cx).source(), "Terminal result");
                let terminal = first.terminals().next().expect("terminal placeholder");
                assert_eq!(terminal.read(cx).id(), &acp_v2::TerminalId::new("unseen"));
                assert!(!terminal.read(cx).is_process_backed());
                assert!(
                    thread
                        .tool_call(&second_id)
                        .expect("second tool")
                        .1
                        .authorization_id()
                        .is_some()
                );
                index
            });
            assert!(matches!(
                first_permission.await,
                RequestPermissionOutcome::Cancelled
            ));
            cx.run_until_parked();
            assert!(updated_entries.borrow().contains(&first_index));
            assert_eq!(
                cx.active_idle_sleep_preventions(),
                0,
                "another permission still needs input"
            );
            thread.update(cx, |thread, cx| {
                thread.cancel_tool_call_authorization(&second_id, cx)
            });
            assert!(matches!(
                second_permission.await,
                RequestPermissionOutcome::Cancelled
            ));
            assert_eq!(cx.active_idle_sleep_preventions(), 1);
            complete
                .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                .expect("backend still running");
            request.await.expect("turn completes");
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
    }

    #[gpui::test]
    async fn test_failed_permission_replacement_keeps_status_settlement_contract(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for status in [
            acp_v1::ToolCallStatus::InProgress,
            acp_v1::ToolCallStatus::Completed,
        ] {
            let thread = new_test_thread(cx).await;
            let tool_id = acp_v2::ToolCallId::new("replacement");
            let (events, _subscription) = track_permission_events(&thread, cx);
            let (old_id, old_response) =
                request_test_permission_with_id(&thread, tool_id.clone(), cx);
            let result = thread.update(cx, |thread, cx| {
                thread.request_tool_call_authorization_with_id(
                    acp_v1::ToolCallUpdate::new(
                        acp_v1::ToolCallId::new(tool_id.0.clone()),
                        acp_v1::ToolCallUpdateFields::new()
                            .status(status)
                            .title("Must not replace presentation")
                            .content(vec![acp_v1::ToolCallContent::Terminal(
                                acp_v1::Terminal::new("missing"),
                            )]),
                    ),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        "new-choice",
                        "New choice",
                        acp_v2::PermissionOptionKind::RejectOnce,
                    )]),
                    AuthorizationKind::ActionChoice,
                    cx,
                )
            });
            assert!(result.is_err());
            thread.read_with(cx, |thread, _| {
                let (_, call) = thread.tool_call(&tool_id).expect("existing tool");
                if status == acp_v1::ToolCallStatus::Completed {
                    assert!(thread.permission_request(old_id).is_none());
                    assert!(call.authorization_id().is_none());
                    assert_eq!(call.status(), ToolCallStatus::Completed);
                    assert_eq!(*events.borrow(), [(true, old_id), (false, old_id)]);
                } else {
                    let request = thread
                        .permission_request(old_id)
                        .expect("old request remains");
                    assert_eq!(
                        request.legacy_kind(),
                        Some(AuthorizationKind::PermissionGrant)
                    );
                    let options = request.legacy_options().expect("legacy permission options");
                    assert!(options.option_for_id(&"allow".into()).is_some());
                    assert!(options.option_for_id(&"new-choice".into()).is_none());
                    assert_eq!(call.authorization_id(), Some(old_id));
                    assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
                    assert_eq!(*events.borrow(), [(true, old_id)]);
                }
            });
            thread.update(cx, |thread, cx| {
                thread.cancel_permission_request(old_id, cx)
            });
            assert!(matches!(
                old_response.await,
                RequestPermissionOutcome::Cancelled
            ));
            assert_eq!(*events.borrow(), [(true, old_id), (false, old_id)]);
        }
    }

    #[gpui::test]
    async fn test_permission_response_task_does_not_keep_thread_alive(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (id, response) = request_test_permission_with_id(&thread, "same-tool".into(), cx);
        let released = thread.downgrade();
        drop(thread);
        cx.run_until_parked();
        assert!(released.upgrade().is_none());
        assert!(matches!(
            response.await,
            RequestPermissionOutcome::Cancelled
        ));

        let replacement = new_test_thread(cx).await;
        let (new_id, new_response) =
            request_test_permission_with_id(&replacement, "same-tool".into(), cx);
        assert_ne!(id, new_id);
        replacement.update(cx, |thread, cx| {
            thread.cancel_permission_request(id, cx);
            assert!(thread.permission_request(new_id).is_some());
            thread.cancel_permission_request(new_id, cx);
        });
        assert!(matches!(
            new_response.await,
            RequestPermissionOutcome::Cancelled
        ));
    }

    #[gpui::test]
    async fn test_idle_cancel_settles_permissions_without_canceling_other_tools(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let tool_id = acp_v2::ToolCallId::new("idle-permission");
        let unrelated_id = acp_v2::ToolCallId::new("unrelated");
        let (events, _subscription) = track_permission_events(&thread, cx);
        thread.update(cx, |thread, cx| {
            assert!(thread.running_turn.is_none());
            thread
                .upsert_tool_call(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(unrelated_id.0.clone()),
                        "Unrelated work",
                    )
                    .status(acp_v1::ToolCallStatus::InProgress),
                    cx,
                )
                .expect("unrelated tool");
        });
        let (id, response) = request_test_permission_with_id(&thread, tool_id.clone(), cx);
        let cancellation = thread.update(cx, |thread, cx| {
            assert!(!thread.is_idle_for_retention());
            let cancellation = thread.cancel(cx);
            assert!(thread.permission_request(id).is_none());
            assert!(thread.permission_request_for_tool(&tool_id).is_none());
            assert_eq!(
                thread.tool_call(&tool_id).expect("tool").1.status(),
                ToolCallStatus::Canceled
            );
            assert_eq!(
                thread
                    .tool_call(&unrelated_id)
                    .expect("unrelated tool")
                    .1
                    .status(),
                ToolCallStatus::InProgress
            );
            assert!(thread.is_idle_for_retention());
            cancellation
        });
        cancellation.await;
        assert!(matches!(
            response.await,
            RequestPermissionOutcome::Cancelled
        ));
        thread.update(cx, |thread, cx| thread.cancel(cx)).await;
        assert_eq!(*events.borrow(), [(true, id), (false, id)]);
    }

    #[gpui::test]
    async fn test_permission_settlement_does_not_depend_on_response_waiter(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, turn) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        let (events, _subscription) = track_permission_events(&thread, cx);
        let first_tool = acp_v2::ToolCallId::new("first");
        let second_tool = acp_v2::ToolCallId::new("second");
        let (first_id, first_response) =
            request_test_permission_with_id(&thread, first_tool.clone(), cx);
        let (second_id, second_response) =
            request_test_permission_with_id(&thread, second_tool.clone(), cx);
        drop(first_response);
        drop(second_response);
        cx.run_until_parked();
        assert_eq!(*events.borrow(), [(true, first_id), (true, second_id)]);
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.foreground_activity(),
                ForegroundActivity::RequiresAction
            );
            assert_eq!(
                thread
                    .pending_permission_requests()
                    .map(|request| request.id)
                    .collect::<Vec<_>>(),
                [first_id, second_id]
            );
            assert_eq!(
                thread
                    .permission_request_for_tool(&first_tool)
                    .expect("first request")
                    .id,
                first_id
            );
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_permission_request(
                first_id,
                SelectedPermissionOutcome::new(
                    "allow".into(),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
            assert!(thread.permission_request(first_id).is_none());
            assert!(thread.permission_request_for_tool(&first_tool).is_none());
            let (_, call) = thread.tool_call(&first_tool).expect("selected tool");
            assert!(call.authorization_id().is_none());
            assert_eq!(call.status(), ToolCallStatus::InProgress);
            assert_eq!(
                thread.foreground_activity(),
                ForegroundActivity::RequiresAction
            );
            assert_eq!(
                thread
                    .pending_permission_requests()
                    .map(|request| request.id)
                    .collect::<Vec<_>>(),
                [second_id]
            );
        });
        assert_eq!(
            *events.borrow(),
            [(true, first_id), (true, second_id), (false, first_id)]
        );
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        thread.update(cx, |thread, cx| {
            thread.cancel_permission_request(second_id, cx);
            thread.cancel_permission_request(second_id, cx);
            assert!(thread.permission_request(second_id).is_none());
            assert!(thread.permission_request_for_tool(&second_tool).is_none());
            assert_eq!(thread.pending_permission_requests().count(), 0);
            let (_, call) = thread.tool_call(&second_tool).expect("cancelled tool");
            assert!(call.authorization_id().is_none());
            assert_eq!(call.status(), ToolCallStatus::Canceled);
        });
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
        });
        assert_eq!(
            *events.borrow(),
            [
                (true, first_id),
                (true, second_id),
                (false, first_id),
                (false, second_id)
            ]
        );
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("turn should still be running");
        turn.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }

    #[gpui::test]
    async fn test_replaced_permission_request_cannot_settle_its_successor(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (events, _subscription) = track_permission_events(&thread, cx);
        let tool_id = acp_v2::ToolCallId::new("reused");
        let (old_id, old_response) = request_test_permission_with_id(&thread, tool_id.clone(), cx);
        let (new_id, new_response) = request_test_permission_with_id(&thread, tool_id.clone(), cx);
        assert_ne!(old_id, new_id);
        thread.update(cx, |thread, cx| {
            thread.cancel_permission_request(old_id, cx);
            thread.authorize_permission_request(
                old_id,
                SelectedPermissionOutcome::new(
                    "reject".into(),
                    acp_v2::PermissionOptionKind::RejectOnce,
                ),
                cx,
            );
            assert!(thread.permission_request(old_id).is_none());
            assert_eq!(
                thread
                    .permission_request_for_tool(&tool_id)
                    .expect("replacement request")
                    .id,
                new_id
            );
            let (_, call) = thread.tool_call(&tool_id).expect("replacement tool");
            assert_eq!(call.authorization_id(), Some(new_id));
            assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
        });
        assert!(matches!(
            old_response.await,
            RequestPermissionOutcome::Cancelled
        ));
        assert_eq!(
            *events.borrow(),
            [(true, old_id), (false, old_id), (true, new_id)]
        );
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread
                    .pending_permission_requests()
                    .map(|request| request.id)
                    .collect::<Vec<_>>(),
                [new_id]
            );
            assert_eq!(
                thread
                    .tool_call(&tool_id)
                    .expect("replacement tool")
                    .1
                    .authorization_id(),
                Some(new_id)
            );
        });

        let other_thread = new_test_thread(cx).await;
        let (other_id, other_response) =
            request_test_permission_with_id(&other_thread, tool_id.clone(), cx);
        assert_ne!(other_id, old_id);
        assert_ne!(other_id, new_id);
        other_thread.update(cx, |thread, cx| {
            thread.cancel_permission_request(new_id, cx);
            assert_eq!(
                thread
                    .tool_call(&tool_id)
                    .expect("other thread tool")
                    .1
                    .authorization_id(),
                Some(other_id)
            );
            thread.cancel_permission_request(other_id, cx);
        });
        assert!(matches!(
            other_response.await,
            RequestPermissionOutcome::Cancelled
        ));
        thread.update(cx, |thread, cx| {
            thread.cancel_permission_request(other_id, cx);
            thread.authorize_permission_request(
                new_id,
                SelectedPermissionOutcome::new(
                    "allow".into(),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });
        assert!(matches!(
            new_response.await,
            RequestPermissionOutcome::Selected(outcome)
                if outcome.option_id == "allow".into()
                    && outcome.option_kind == acp_v2::PermissionOptionKind::AllowOnce
        ));
        thread.read_with(cx, |thread, _| {
            assert!(thread.permission_request(new_id).is_none());
            assert_eq!(thread.pending_permission_requests().count(), 0);
            let (_, call) = thread.tool_call(&tool_id).expect("selected replacement");
            assert!(call.authorization_id().is_none());
            assert_eq!(call.status(), ToolCallStatus::InProgress);
        });
        assert_eq!(
            *events.borrow(),
            [
                (true, old_id),
                (false, old_id),
                (true, new_id),
                (false, new_id)
            ]
        );
    }

    #[gpui::test]
    async fn test_permission_selection_validates_offered_id_and_derives_kind(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for (kind, option_id, offered_kind, spoofed_kind, expected) in [
            (
                AuthorizationKind::PermissionGrant,
                "allow",
                acp_v2::PermissionOptionKind::AllowOnce,
                acp_v2::PermissionOptionKind::RejectOnce,
                ToolCallStatus::InProgress,
            ),
            (
                AuthorizationKind::PermissionGrant,
                "reject",
                acp_v2::PermissionOptionKind::RejectOnce,
                acp_v2::PermissionOptionKind::AllowOnce,
                ToolCallStatus::Rejected,
            ),
            (
                AuthorizationKind::ActionChoice,
                "reject",
                acp_v2::PermissionOptionKind::RejectOnce,
                acp_v2::PermissionOptionKind::AllowOnce,
                ToolCallStatus::InProgress,
            ),
        ] {
            let allow = acp_v2::PermissionOption::new(
                "allow",
                "Allow",
                acp_v2::PermissionOptionKind::AllowOnce,
            );
            let reject = acp_v2::PermissionOption::new(
                "reject",
                "Reject",
                acp_v2::PermissionOptionKind::RejectOnce,
            );
            let choice = PermissionOptionChoice {
                allow: allow.clone(),
                deny: reject.clone(),
                sub_patterns: vec!["^cargo build$".into()],
            };
            for options in [
                PermissionOptions::Flat(vec![allow, reject]),
                PermissionOptions::Dropdown(vec![choice.clone()]),
                PermissionOptions::DropdownWithPatterns {
                    choices: vec![choice],
                    patterns: vec![
                        PermissionPattern {
                            pattern: "^cargo build$".into(),
                            display_name: "cargo build".into(),
                        },
                        PermissionPattern {
                            pattern: "^git status$".into(),
                            display_name: "git status".into(),
                        },
                    ],
                    tool_name: "test".into(),
                },
            ] {
                let (mut selected_outcome, expected_patterns) = match &options {
                    PermissionOptions::Flat(_) => (
                        SelectedPermissionOutcome::new(option_id.into(), offered_kind.clone()),
                        None,
                    ),
                    PermissionOptions::Dropdown(choices) => (
                        choices
                            .first()
                            .expect("choice")
                            .build_outcome(option_id == "allow"),
                        Some(vec!["^cargo build$".to_owned()]),
                    ),
                    PermissionOptions::DropdownWithPatterns { .. } => (
                        options
                            .build_outcome_for_checked_patterns(&[1], option_id == "allow")
                            .expect("checked pattern"),
                        Some(vec!["^git status$".to_owned()]),
                    ),
                };
                selected_outcome.option_kind = spoofed_kind.clone();
                let thread = new_test_thread(cx).await;
                let (events, _subscription) = track_permission_events(&thread, cx);
                let tool_id = acp_v2::ToolCallId::new("choice");
                let (request_id, response) = thread.update(cx, |thread, cx| {
                    thread
                        .request_tool_call_authorization_with_id(
                            acp_v1::ToolCall::new(
                                acp_v1::ToolCallId::new(tool_id.0.clone()),
                                "Choose",
                            )
                            .into(),
                            options,
                            kind,
                            cx,
                        )
                        .expect("permission request")
                });
                thread.update(cx, |thread, cx| {
                    thread.authorize_permission_request(
                        request_id,
                        SelectedPermissionOutcome::new("not-offered".into(), spoofed_kind.clone()),
                        cx,
                    );
                    let request = thread
                        .permission_request(request_id)
                        .expect("invalid choice stays pending");
                    assert_eq!(request.legacy_kind(), Some(kind));
                    assert_eq!(request.legacy_tool_call_id(), Some(&tool_id));
                    assert_eq!(thread.pending_permission_requests().count(), 1);
                    let (_, call) = thread.tool_call(&tool_id).expect("waiting tool");
                    assert_eq!(call.authorization_id(), Some(request_id));
                    assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
                });
                assert_eq!(*events.borrow(), [(true, request_id)]);
                thread.update(cx, |thread, cx| {
                    thread.authorize_permission_request(request_id, selected_outcome, cx);
                    assert!(thread.permission_request(request_id).is_none());
                    assert!(thread.permission_request_for_tool(&tool_id).is_none());
                    assert_eq!(thread.pending_permission_requests().count(), 0);
                    let (_, call) = thread.tool_call(&tool_id).expect("selected tool");
                    assert!(call.authorization_id().is_none());
                    assert_eq!(call.status(), expected);
                });
                assert_eq!(*events.borrow(), [(true, request_id), (false, request_id)]);
                let RequestPermissionOutcome::Selected(outcome) = response.await else {
                    panic!("expected selected permission");
                };
                assert_eq!(outcome.option_id, option_id.into());
                assert_eq!(outcome.option_kind, offered_kind);
                match (outcome.params, expected_patterns) {
                    (Some(SelectedPermissionParams::Terminal { patterns }), Some(expected)) => {
                        assert_eq!(patterns, expected);
                    }
                    (None, None) => {}
                    _ => panic!("native permission parameters were not preserved"),
                }
            }
        }
    }

    #[gpui::test]
    async fn test_unknown_permission_kind_keeps_authorization_pending(cx: &mut TestAppContext) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let tool_call_id = acp_v2::ToolCallId::new("unknown-permission-kind");
        let option_id = acp_v2::PermissionOptionId::new("future-choice");
        let option_kind = acp_v2::PermissionOptionKind::Other("_future_choice".into());
        let (request_id, mut response) = thread.update(cx, |thread, cx| {
            thread
                .request_tool_call_authorization_with_id(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                        "Choose",
                    )
                    .into(),
                    PermissionOptions::Flat(vec![acp_v2::PermissionOption::new(
                        option_id.clone(),
                        "Future choice",
                        option_kind.clone(),
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
                .expect("permission request")
        });
        thread.update(cx, |thread, cx| {
            thread.authorize_permission_request(
                request_id,
                SelectedPermissionOutcome::new(
                    option_id.clone(),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
            let request = thread
                .permission_request(request_id)
                .expect("unknown choice stays pending");
            let option = request
                .legacy_options()
                .and_then(|options| options.option_for_id(&option_id))
                .expect("offered option");
            assert_eq!(option.kind, option_kind);
            assert!(Arc::ptr_eq(&option.option_id.0, &option_id.0));
            let (_, call) = thread.tool_call(&tool_call_id).expect("waiting tool");
            assert_eq!(call.authorization_id(), Some(request_id));
            assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
        });
        assert!((&mut response).now_or_never().is_none());
        thread.update(cx, |thread, cx| {
            thread.cancel_permission_request(request_id, cx);
        });
        assert!(matches!(
            response.await,
            RequestPermissionOutcome::Cancelled
        ));
    }

    #[gpui::test]
    async fn test_entry_removal_cleans_permission_records_without_reusing_identity(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for refuse_prompt in [false, true] {
            let thread = new_test_thread(cx).await;
            thread.update(cx, |thread, cx| {
                thread.push_user_content_block(None, "retained".into(), cx);
            });
            let refusal_turn = refuse_prompt.then(|| start_test_turn(&thread, cx));
            let retained_tool = acp_v2::ToolCallId::new("retained");
            let (retained_id, retained_response) =
                request_test_permission_with_id(&thread, retained_tool.clone(), cx);
            let client_id = ClientUserMessageId::new();
            thread.update(cx, |thread, cx| {
                thread.push_user_content_block(Some(client_id.clone()), "remove me".into(), cx);
            });
            let removed_tool = acp_v2::ToolCallId::new("reused");
            let (removed_id, removed_response) =
                request_test_permission_with_id(&thread, removed_tool.clone(), cx);
            if let Some((complete, turn)) = refusal_turn {
                complete
                    .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::Refusal)))
                    .expect("turn should still be running");
                turn.await.expect("refusal should complete");
            } else {
                thread
                    .update(cx, |thread, cx| thread.rewind(client_id, cx))
                    .await
                    .expect("rewind should complete");
            }
            assert!(matches!(
                removed_response.await,
                RequestPermissionOutcome::Cancelled
            ));
            thread.read_with(cx, |thread, _| {
                assert!(thread.tool_call(&removed_tool).is_none());
                assert!(thread.permission_request(removed_id).is_none());
                assert!(thread.permission_request_for_tool(&removed_tool).is_none());
                assert_eq!(
                    thread
                        .pending_permission_requests()
                        .map(|request| request.id)
                        .collect::<Vec<_>>(),
                    [retained_id]
                );
                assert_eq!(
                    thread
                        .tool_call(&retained_tool)
                        .expect("retained tool")
                        .1
                        .authorization_id(),
                    Some(retained_id)
                );
            });
            let (reopened_id, reopened_response) =
                request_test_permission_with_id(&thread, removed_tool.clone(), cx);
            assert_ne!(reopened_id, removed_id);
            assert_ne!(reopened_id, retained_id);
            thread.update(cx, |thread, cx| {
                thread.cancel_permission_request(removed_id, cx);
                thread.authorize_permission_request(
                    removed_id,
                    SelectedPermissionOutcome::new(
                        "reject".into(),
                        acp_v2::PermissionOptionKind::RejectOnce,
                    ),
                    cx,
                );
                let (_, call) = thread.tool_call(&removed_tool).expect("reopened tool");
                assert_eq!(call.authorization_id(), Some(reopened_id));
                assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
                assert_eq!(
                    thread
                        .pending_permission_requests()
                        .map(|request| request.id)
                        .collect::<Vec<_>>(),
                    [retained_id, reopened_id]
                );
                thread.cancel_permission_request(retained_id, cx);
                thread.cancel_permission_request(reopened_id, cx);
                assert!(thread.permission_request(retained_id).is_none());
                assert!(thread.permission_request(reopened_id).is_none());
                assert_eq!(thread.pending_permission_requests().count(), 0);
                for tool_id in [&retained_tool, &removed_tool] {
                    let (_, call) = thread.tool_call(tool_id).expect("cancelled tool");
                    assert!(call.authorization_id().is_none());
                    assert!(thread.permission_request_for_tool(tool_id).is_none());
                }
            });
            assert!(matches!(
                retained_response.await,
                RequestPermissionOutcome::Cancelled
            ));
            assert!(matches!(
                reopened_response.await,
                RequestPermissionOutcome::Cancelled
            ));
        }
    }

    #[gpui::test]
    async fn test_generic_permission_retains_sdk_request_without_transcript_entries(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (events, _subscription) = track_permission_events(&thread, cx);
        let subjects = [
            Some(
                acp_v2::CommandPermissionSubject::new("cargo test", path!("/test"))
                    .tool_call_id(acp_v2::ToolCallId::new("context-only"))
                    .terminal_id(acp_v2::TerminalId::new("context-only-terminal"))
                    .meta(acp_v2::Meta::from_iter([(
                        "command-data".into(),
                        json!({"nested": [1, true, null]}),
                    )]))
                    .into(),
            ),
            None,
            Some(acp_v2::RequestPermissionSubject::Other(
                acp_v2::OtherRequestPermissionSubject::new(
                    "future_operation",
                    [
                        ("payload".into(), json!({"resource": ["one", "two"]})),
                        ("_meta".into(), json!({"opaque": true})),
                    ]
                    .into_iter()
                    .collect(),
                ),
            )),
            Some(
                acp_v2::ToolCallUpdate::new("not-a-transcript-tool")
                    .title("Context, not a tool update")
                    .status(acp_v2::ToolCallStatus::Completed)
                    .content(vec!["Context, not transcript content".into()])
                    .meta(acp_v2::Meta::from_iter([(
                        "tool-data".into(),
                        json!({"opaque": ["retained"]}),
                    )]))
                    .into(),
            ),
        ];
        let mut pending = Vec::new();
        for subject in subjects {
            let request = thread.read_with(cx, |thread, _| {
                test_generic_permission_request(thread, "future-choice").subject(subject)
            });
            let (id, response) = thread.update(cx, |thread, cx| {
                thread
                    .request_permission(request.clone(), cx)
                    .expect("generic permission request")
            });
            thread.read_with(cx, |thread, _| {
                let record = thread.permission_request(id).expect("pending record");
                assert_eq!(record.generic_request(), Some(&request));
                assert!(record.legacy_tool_call_id().is_none());
                assert!(record.legacy_options().is_none());
                assert!(record.legacy_kind().is_none());
                assert!(thread.entries().is_empty());
                assert!(thread.tool_call(&"context-only".into()).is_none());
                assert!(thread.tool_call(&"not-a-transcript-tool".into()).is_none());
                assert!(!thread.is_idle_for_retention());
            });
            pending.push((id, response));
        }
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.pending_permission_requests().count(), 4);
        });
        assert_eq!(
            *events.borrow(),
            pending
                .iter()
                .map(|(id, _)| (true, *id))
                .collect::<Vec<_>>()
        );
        for (id, response) in pending {
            thread.update(cx, |thread, cx| thread.cancel_permission_request(id, cx));
            assert_eq!(response.await, acp_v2::RequestPermissionOutcome::Cancelled);
        }
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.pending_permission_requests().count(), 0);
            assert!(thread.entries().is_empty());
            assert!(thread.is_idle_for_retention());
        });
    }

    #[gpui::test]
    async fn test_generic_permissions_for_same_subject_have_independent_choices(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (events, _subscription) = track_permission_events(&thread, cx);
        let subject = acp_v2::RequestPermissionSubject::from(
            acp_v2::CommandPermissionSubject::new("cargo test", path!("/test")),
        );
        let (first_id, first_response) = thread.update(cx, |thread, cx| {
            thread
                .request_permission(
                    test_generic_permission_request(thread, "first-choice")
                        .subject(subject.clone()),
                    cx,
                )
                .expect("first permission")
        });
        let (second_id, second_response) = thread.update(cx, |thread, cx| {
            thread
                .request_permission(
                    test_generic_permission_request(thread, "second-choice").subject(subject),
                    cx,
                )
                .expect("second permission")
        });
        assert_ne!(first_id, second_id);
        thread.update(cx, |thread, cx| {
            thread.select_permission_option(first_id, "second-choice".into(), cx);
            thread.select_permission_option(second_id, "not-offered".into(), cx);
            assert_eq!(
                thread
                    .pending_permission_requests()
                    .map(|request| request.id)
                    .collect::<Vec<_>>(),
                [first_id, second_id]
            );
        });
        assert_eq!(*events.borrow(), [(true, first_id), (true, second_id)]);
        thread.update(cx, |thread, cx| {
            thread.select_permission_option(first_id, "first-choice".into(), cx);
            assert!(thread.permission_request(first_id).is_none());
            thread.select_permission_option(first_id, "second-choice".into(), cx);
            thread.cancel_permission_request(first_id, cx);
            assert!(thread.permission_request(second_id).is_some());
            assert_eq!(thread.pending_permission_requests().count(), 1);
        });
        assert_eq!(
            *events.borrow(),
            [(true, first_id), (true, second_id), (false, first_id)]
        );
        assert_eq!(
            first_response.await,
            acp_v2::RequestPermissionOutcome::Selected(acp_v2::SelectedPermissionOutcome::new(
                "first-choice"
            ))
        );
        thread.update(cx, |thread, cx| {
            thread.cancel_permission_request(second_id, cx);
            thread.cancel_permission_request(second_id, cx);
            assert_eq!(thread.pending_permission_requests().count(), 0);
            assert!(thread.entries().is_empty());
        });
        assert_eq!(
            *events.borrow(),
            [
                (true, first_id),
                (true, second_id),
                (false, first_id),
                (false, second_id)
            ]
        );
        assert_eq!(
            second_response.await,
            acp_v2::RequestPermissionOutcome::Cancelled
        );
    }

    #[gpui::test]
    async fn test_generic_tool_permissions_do_not_replace_or_mutate_legacy_authorization(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let tool_id = acp_v2::ToolCallId::new("shared-tool");
        let (legacy_id, legacy_response) =
            request_test_permission_with_id(&thread, tool_id.clone(), cx);
        let (events, _subscription) = track_permission_events(&thread, cx);
        let transcript_events = Rc::new(RefCell::new(0));
        let _transcript_subscription = cx.update(|cx| {
            cx.subscribe(&thread, {
                let transcript_events = transcript_events.clone();
                move |_, event, _| {
                    if matches!(
                        event,
                        AcpThreadEvent::NewEntry
                            | AcpThreadEvent::EntryUpdated(_)
                            | AcpThreadEvent::EntriesRemoved(_)
                    ) {
                        *transcript_events.borrow_mut() += 1;
                    }
                }
            })
        });
        let request = thread.read_with(cx, |thread, _| {
            test_generic_permission_request(thread, "future-choice").subject(
                acp_v2::RequestPermissionSubject::from(
                    acp_v2::ToolCallUpdate::new("shared-tool")
                        .title("Must not replace the label")
                        .status(acp_v2::ToolCallStatus::Completed)
                        .content(vec!["Must not replace tool output".into()]),
                ),
            )
        });
        let (selected_id, selected_response) = thread.update(cx, |thread, cx| {
            thread
                .request_permission(request.clone(), cx)
                .expect("first generic tool permission")
        });
        let (cancelled_id, cancelled_response) = thread.update(cx, |thread, cx| {
            thread
                .request_permission(request, cx)
                .expect("second generic tool permission")
        });
        thread.update(cx, |thread, cx| {
            thread.select_permission_option(selected_id, "future-choice".into(), cx);
            thread.cancel_permission_request(cancelled_id, cx);
            let (_, call) = thread.tool_call(&tool_id).expect("legacy tool");
            assert_eq!(call.authorization_id(), Some(legacy_id));
            assert_eq!(call.status(), ToolCallStatus::WaitingForConfirmation);
            assert_eq!(call.label.read(cx).source(), "Needs permission");
            assert!(call.content().is_empty());
            assert_eq!(thread.entries().len(), 1);
            assert_eq!(
                thread
                    .permission_request_for_tool(&tool_id)
                    .expect("legacy link survives")
                    .id,
                legacy_id
            );
            assert!(
                thread
                    .permission_request(legacy_id)
                    .expect("legacy record")
                    .generic_request()
                    .is_none()
            );
        });
        assert_eq!(*transcript_events.borrow(), 0);
        assert_eq!(
            selected_response.await,
            acp_v2::RequestPermissionOutcome::Selected(acp_v2::SelectedPermissionOutcome::new(
                "future-choice"
            ))
        );
        assert_eq!(
            cancelled_response.await,
            acp_v2::RequestPermissionOutcome::Cancelled
        );
        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_id.clone(),
                SelectedPermissionOutcome::new(
                    "allow".into(),
                    acp_v2::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
            assert_eq!(thread.pending_permission_requests().count(), 0);
            assert_eq!(
                thread
                    .tool_call(&tool_id)
                    .expect("authorized tool")
                    .1
                    .status(),
                ToolCallStatus::InProgress
            );
        });
        assert!(matches!(
            legacy_response.await,
            RequestPermissionOutcome::Selected(outcome) if outcome.option_id == "allow".into()
        ));
        assert_eq!(
            *events.borrow(),
            [
                (true, selected_id),
                (true, cancelled_id),
                (false, selected_id),
                (false, cancelled_id),
                (false, legacy_id)
            ]
        );
    }

    #[gpui::test]
    async fn test_generic_permission_completion_is_state_owned_not_waiter_owned(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, turn) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        let (events, _subscription) = track_permission_events(&thread, cx);
        let (first_id, first_response) = thread.update(cx, |thread, cx| {
            thread
                .request_permission(test_generic_permission_request(thread, "first-choice"), cx)
                .expect("first permission")
        });
        let (second_id, second_response) = thread.update(cx, |thread, cx| {
            thread
                .request_permission(test_generic_permission_request(thread, "second-choice"), cx)
                .expect("second permission")
        });
        drop(first_response);
        drop(second_response);
        cx.run_until_parked();
        assert_eq!(*events.borrow(), [(true, first_id), (true, second_id)]);
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
        thread.update(cx, |thread, cx| {
            assert_eq!(
                thread.foreground_activity(),
                ForegroundActivity::RequiresAction
            );
            thread.select_permission_option(first_id, "first-choice".into(), cx);
            assert!(thread.permission_request(first_id).is_none());
            assert!(thread.permission_request(second_id).is_some());
            assert_eq!(
                thread.foreground_activity(),
                ForegroundActivity::RequiresAction
            );
            thread.cancel_permission_request(second_id, cx);
            thread.cancel_permission_request(second_id, cx);
            assert_eq!(thread.pending_permission_requests().count(), 0);
        });
        cx.run_until_parked();
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.foreground_activity(), ForegroundActivity::Running);
        });
        assert_eq!(
            *events.borrow(),
            [
                (true, first_id),
                (true, second_id),
                (false, first_id),
                (false, second_id)
            ]
        );
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("backend still running");
        turn.await.expect("turn completes");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        let released_thread = new_test_thread(cx).await;
        let (_, response) = released_thread.update(cx, |thread, cx| {
            thread
                .request_permission(
                    test_generic_permission_request(thread, "release-choice"),
                    cx,
                )
                .expect("permission before release")
        });
        let released = released_thread.downgrade();
        drop(released_thread);
        cx.run_until_parked();
        assert!(released.upgrade().is_none());
        assert_eq!(response.await, acp_v2::RequestPermissionOutcome::Cancelled);
    }

    #[gpui::test]
    async fn test_generic_permission_validation_has_no_records_events_or_tool_mutations(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (events, _subscription) = track_permission_events(&thread, cx);
        let valid = thread.read_with(cx, |thread, _| {
            test_generic_permission_request(thread, "valid-choice").subject(
                acp_v2::RequestPermissionSubject::from(
                    acp_v2::ToolCallUpdate::new("must-not-create")
                        .title("Must not create tool")
                        .status(acp_v2::ToolCallStatus::InProgress),
                ),
            )
        });
        let mut wrong_session = valid.clone();
        wrong_session.session_id = acp_v2::SessionId::new("different-session");
        let mut empty_options = valid.clone();
        empty_options.options.clear();
        let mut duplicate_options = valid.clone();
        duplicate_options
            .options
            .push(acp_v2::PermissionOption::new(
                "valid-choice",
                "Different label, same ID",
                acp_v2::PermissionOptionKind::AllowAlways,
            ));
        for request in [wrong_session, empty_options, duplicate_options] {
            let result = thread.update(cx, |thread, cx| thread.request_permission(request, cx));
            assert!(result.is_err());
            thread.read_with(cx, |thread, _| {
                assert_eq!(thread.pending_permission_requests().count(), 0);
                assert!(thread.entries().is_empty());
                assert!(thread.tool_call(&"must-not-create".into()).is_none());
                assert!(thread.is_idle_for_retention());
            });
            assert!(events.borrow().is_empty());
        }
        let (id, response) = thread.update(cx, |thread, cx| {
            thread
                .request_permission(valid.clone(), cx)
                .expect("valid request after rejection")
        });
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread
                    .permission_request(id)
                    .expect("valid record")
                    .generic_request(),
                Some(&valid)
            );
        });
        thread.update(cx, |thread, cx| thread.cancel_permission_request(id, cx));
        assert_eq!(response.await, acp_v2::RequestPermissionOutcome::Cancelled);
        assert_eq!(*events.borrow(), [(true, id), (false, id)]);
    }

    #[gpui::test]
    async fn test_generic_permissions_settle_on_cancellation_refusal_and_rewind(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for lifecycle in [
            "idle cancel",
            "turn cancel",
            "reported cancel",
            "reported idle cancel",
            "refusal",
            "refusal without user",
            "refusal after tool",
            "rewind",
        ] {
            let thread = if matches!(lifecycle, "reported cancel" | "reported idle cancel") {
                new_receipt_test_thread(cx).await.0
            } else {
                new_test_thread(cx).await
            };
            let (events, _subscription) = track_permission_events(&thread, cx);
            let client_id = ClientUserMessageId::new();
            thread.update(cx, |thread, cx| {
                if lifecycle != "refusal without user" {
                    thread.push_user_content_block(None, "retained".into(), cx);
                }
                if lifecycle == "reported cancel" {
                    thread
                        .update_session_state(
                            acp_v2::StateUpdate::Running(acp_v2::RunningStateUpdate::new()),
                            cx,
                        )
                        .expect("reported running activity");
                }
            });
            let running_turn = matches!(
                lifecycle,
                "turn cancel" | "refusal" | "refusal without user" | "refusal after tool"
            )
            .then(|| start_test_turn(&thread, cx));
            let (first_id, first_response) = thread.update(cx, |thread, cx| {
                thread
                    .request_permission(test_generic_permission_request(thread, "first-choice"), cx)
                    .expect("permission before rollback boundary")
            });
            thread.update(cx, |thread, cx| {
                if lifecycle != "refusal without user" {
                    thread.push_user_content_block(Some(client_id.clone()), "remove me".into(), cx);
                }
                if lifecycle == "refusal after tool" {
                    thread
                        .upsert_tool_call(
                            acp_v1::ToolCall::new("finished-tool", "Finished operation")
                                .status(acp_v1::ToolCallStatus::Completed)
                                .raw_output(json!("completed output")),
                            cx,
                        )
                        .expect("completed tool before refusal");
                }
            });
            let (second_id, second_response) = thread.update(cx, |thread, cx| {
                thread
                    .request_permission(
                        test_generic_permission_request(thread, "second-choice"),
                        cx,
                    )
                    .expect("permission after rollback boundary")
            });
            if lifecycle == "reported idle cancel" {
                thread.update(cx, |thread, cx| {
                    thread
                        .update_session_state(
                            acp_v2::StateUpdate::Idle(
                                acp_v2::IdleStateUpdate::new()
                                    .stop_reason(acp_v2::StopReason::Refusal),
                            ),
                            cx,
                        )
                        .expect("reported idle is independent of pending requests");
                    assert_eq!(thread.foreground_activity(), ForegroundActivity::Idle);
                    assert_eq!(thread.pending_permission_requests().count(), 2);
                });
            }
            let entry_count_before_stop = thread.read_with(cx, |thread, _| thread.entries().len());
            if let Some((complete, turn)) = running_turn {
                if matches!(
                    lifecycle,
                    "refusal" | "refusal without user" | "refusal after tool"
                ) {
                    complete
                        .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::Refusal)))
                        .expect("backend still running");
                } else {
                    let cancellation = thread.update(cx, |thread, cx| thread.cancel(cx));
                    thread.read_with(cx, |thread, _| {
                        assert_eq!(thread.pending_permission_requests().count(), 0);
                    });
                    complete
                        .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                        .expect("backend still running after local cancellation");
                    cancellation.await;
                }
                turn.await.expect("turn settles");
            } else if lifecycle == "rewind" {
                thread
                    .update(cx, |thread, cx| thread.rewind(client_id, cx))
                    .await
                    .expect("rewind succeeds");
            } else if lifecycle == "reported cancel" {
                let cancellation = thread.update(cx, |thread, cx| thread.cancel(cx));
                thread.update(cx, |thread, cx| {
                    assert_eq!(thread.pending_permission_requests().count(), 0);
                    thread
                        .update_session_state(
                            acp_v2::StateUpdate::Idle(acp_v2::IdleStateUpdate::new()),
                            cx,
                        )
                        .expect("reported cancellation completion");
                });
                cancellation.await;
            } else {
                thread.update(cx, |thread, cx| thread.cancel(cx)).await;
            }
            thread.update(cx, |thread, cx| {
                assert_eq!(
                    thread.pending_permission_requests().count(),
                    0,
                    "{lifecycle}"
                );
                if lifecycle == "refusal without user" {
                    assert!(thread.entries().is_empty());
                } else if lifecycle == "refusal after tool" {
                    assert_eq!(thread.entries().len(), entry_count_before_stop);
                    let (_, call) = thread
                        .tool_call(&"finished-tool".into())
                        .expect("refusal retains completed tool");
                    assert_eq!(call.status(), ToolCallStatus::Completed);
                }
                thread.cancel_permission_request(first_id, cx);
                thread.cancel_permission_request(second_id, cx);
            });
            assert_eq!(
                *events.borrow(),
                [
                    (true, first_id),
                    (true, second_id),
                    (false, first_id),
                    (false, second_id)
                ],
                "{lifecycle}"
            );
            assert_eq!(
                first_response.await,
                acp_v2::RequestPermissionOutcome::Cancelled
            );
            assert_eq!(
                second_response.await,
                acp_v2::RequestPermissionOutcome::Cancelled
            );
        }

        for asynchronous_failure in [false, true] {
            let fs = FakeFs::new(cx.executor());
            let project = Project::test(fs, [], cx).await;
            let (connection, truncate_gate): (Rc<dyn AgentConnection>, _) = if asynchronous_failure
            {
                let connection = Rc::new(StubAgentConnection::new());
                let gate = connection.defer_next_truncate();
                (connection, Some(gate))
            } else {
                (
                    Rc::new(FakeAgentConnection::new().without_truncate_support()),
                    None,
                )
            };
            let thread = cx
                .update(|cx| {
                    connection.new_session(project, PathList::new(&[Path::new(path!("/test"))]), cx)
                })
                .await
                .expect("rewind test thread");
            let (events, _subscription) = track_permission_events(&thread, cx);
            let client_id = ClientUserMessageId::new();
            let (id, response) = thread.update(cx, |thread, cx| {
                thread.push_user_content_block(Some(client_id.clone()), "keep me".into(), cx);
                thread
                    .request_permission(test_generic_permission_request(thread, "keep-choice"), cx)
                    .expect("permission before failed rewind")
            });
            let mut rewind = thread.update(cx, |thread, cx| thread.rewind(client_id.clone(), cx));
            cx.run_until_parked();
            assert_eq!(*events.borrow(), [(true, id)]);
            if asynchronous_failure {
                assert!((&mut rewind).now_or_never().is_none());
            }
            drop(truncate_gate);
            assert!(rewind.await.is_err());
            thread.read_with(cx, |thread, _| {
                assert_eq!(thread.entries().len(), 1);
                assert!(thread.permission_request(id).is_some());
                assert_eq!(thread.pending_permission_requests().count(), 1);
            });
            assert_eq!(*events.borrow(), [(true, id)]);
            if asynchronous_failure {
                thread
                    .update(cx, |thread, cx| thread.rewind(client_id, cx))
                    .await
                    .expect("rewind succeeds after the backend failure");
                thread.read_with(cx, |thread, _| assert!(thread.entries().is_empty()));
            } else {
                thread.update(cx, |thread, cx| thread.cancel_permission_request(id, cx));
            }
            assert_eq!(*events.borrow(), [(true, id), (false, id)]);
            assert_eq!(response.await, acp_v2::RequestPermissionOutcome::Cancelled);
        }
    }

    fn test_generic_permission_request(
        thread: &AcpThread,
        option_id: &str,
    ) -> acp_v2::RequestPermissionRequest {
        acp_v2::RequestPermissionRequest::new(
            thread.session_id().clone(),
            "Approve operation",
            vec![
                acp_v2::PermissionOption::new(
                    option_id,
                    "Future choice",
                    acp_v2::PermissionOptionKind::Other("future_choice".into()),
                )
                .meta(acp_v2::Meta::from_iter([(
                    "option-data".into(),
                    json!({"opaque": [1, null]}),
                )])),
            ],
        )
        .description("Explanation for the permission prompt".to_owned())
        .meta(acp_v2::Meta::from_iter([(
            "request-data".into(),
            json!({"opaque": {"preserved": true}}),
        )]))
    }

    fn track_permission_events(
        thread: &Entity<AcpThread>,
        cx: &mut TestAppContext,
    ) -> (Rc<RefCell<Vec<(bool, PermissionRequestId)>>>, Subscription) {
        let events = Rc::new(RefCell::new(Vec::new()));
        let subscription = cx.update(|cx| {
            cx.subscribe(thread, {
                let events = events.clone();
                move |_, event, _| match event {
                    AcpThreadEvent::ToolAuthorizationRequested(id) => {
                        events.borrow_mut().push((true, *id))
                    }
                    AcpThreadEvent::ToolAuthorizationReceived(id) => {
                        events.borrow_mut().push((false, *id))
                    }
                    _ => {}
                }
            })
        });
        (events, subscription)
    }

    fn start_test_turn(
        thread: &Entity<AcpThread>,
        cx: &mut TestAppContext,
    ) -> (
        oneshot::Sender<Result<acp_v1::PromptResponse>>,
        BoxFuture<'static, Result<Option<acp_v1::PromptResponse>>>,
    ) {
        let (complete, completion) = oneshot::channel::<Result<acp_v1::PromptResponse>>();
        let request = thread.update(cx, |thread, cx| {
            let id = thread.register_submission(Arc::from([]), cx);
            thread.run_turn(id, cx, async move |_, _| completion.await?)
        });
        (
            complete,
            async move {
                match request.await? {
                    Some(SubmissionResponse::LegacyCompleted(response)) => Ok(Some(response)),
                    None => Ok(None),
                    Some(SubmissionResponse::Accepted(_)) => {
                        Err(anyhow!("Expected legacy test turn completion"))
                    }
                }
            }
            .boxed(),
        )
    }

    fn request_test_permission(
        thread: &Entity<AcpThread>,
        tool_call_id: acp_v2::ToolCallId,
        cx: &mut TestAppContext,
    ) -> Task<RequestPermissionOutcome> {
        request_test_permission_with_id(thread, tool_call_id, cx).1
    }

    fn request_test_permission_with_id(
        thread: &Entity<AcpThread>,
        tool_call_id: acp_v2::ToolCallId,
        cx: &mut TestAppContext,
    ) -> (PermissionRequestId, Task<RequestPermissionOutcome>) {
        thread.update(cx, |thread, cx| {
            thread
                .request_tool_call_authorization_with_id(
                    acp_v1::ToolCall::new(
                        acp_v1::ToolCallId::new(tool_call_id.0),
                        "Needs permission",
                    )
                    .into(),
                    PermissionOptions::Flat(vec![
                        acp_v2::PermissionOption::new(
                            acp_v2::PermissionOptionId::new("allow"),
                            "Allow",
                            acp_v2::PermissionOptionKind::AllowOnce,
                        ),
                        acp_v2::PermissionOption::new(
                            acp_v2::PermissionOptionId::new("reject"),
                            "Reject",
                            acp_v2::PermissionOptionKind::RejectOnce,
                        ),
                    ]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
                .expect("permission request should succeed")
        })
    }

    fn request_test_form_elicitation(
        thread: &Entity<AcpThread>,
        cx: &mut TestAppContext,
    ) -> (ElicitationEntryId, Task<acp_v2::CreateElicitationResponse>) {
        thread.update(cx, |thread, cx| {
            thread
                .request_elicitation_with_id(
                    acp_v2::CreateElicitationRequest::new(
                        acp_v2::ElicitationFormMode::new(
                            acp_v2::ElicitationSessionScope::new(thread.session_id().clone()),
                            acp_v2::ElicitationSchema::new().string("name", false),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .expect("form elicitation should succeed")
        })
    }

    fn set_prevent_idle_sleep(enabled: bool, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content.agent.get_or_insert_default().prevent_idle_sleep = Some(enabled);
                });
            });
        });
    }

    async fn assert_stale_completion_does_not_affect_follow_up_turn(
        backend_result: Result<acp_v1::StopReason, &'static str>,
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        for drop_waiter in [false, true] {
            let thread = new_test_thread(cx).await;
            let (first_complete, first_request) = start_test_turn(&thread, cx);
            thread.update(cx, |thread, cx| {
                thread.push_user_content_block(None, "first".into(), cx);
                thread.push_assistant_content_block("first reply".into(), false, cx);
            });
            cx.run_until_parked();
            assert_eq!(cx.active_idle_sleep_preventions(), 1);

            let first_request = if drop_waiter {
                drop(first_request);
                None
            } else {
                Some(first_request)
            };
            first_complete
                .send(
                    backend_result
                        .map(acp_v1::PromptResponse::new)
                        .map_err(|message| anyhow!(message)),
                )
                .expect("first backend should still be running");
            let (second_complete, second_request) = start_test_turn(&thread, cx);
            let capacities = thread.update(cx, |thread, cx| {
                thread.push_user_content_block(None, "second".into(), cx);
                thread.push_assistant_content_block("second reply".into(), false, cx);
                assert_eq!(thread.running_turn.as_ref().map(|turn| turn.id), Some(2));
                assert!(!thread.had_error());
                [2, 3].map(|index| {
                    let content = test_message_content_mut(thread, index);
                    content.source_blocks.reserve(32);
                    content.source_blocks.capacity()
                })
            });
            let events = Rc::new(RefCell::new(Vec::new()));
            let _subscription = cx.update(|cx| {
                cx.subscribe(&thread, {
                    let events = events.clone();
                    move |_, event, _| {
                        let event = match event {
                            AcpThreadEvent::StatusChanged => "status changed",
                            AcpThreadEvent::EntriesRemoved(_) => "entries removed",
                            AcpThreadEvent::Stopped { .. } => "stopped",
                            AcpThreadEvent::Refusal => "refusal",
                            AcpThreadEvent::Error => "error",
                            _ => return,
                        };
                        events.borrow_mut().push(event);
                    }
                })
            });
            cx.run_until_parked();
            if let Some(first_request) = first_request {
                assert_eq!(
                    first_request
                        .await
                        .map(|response| response.map(|response| response.stop_reason))
                        .map_err(|error| error.to_string()),
                    backend_result.map(Some).map_err(String::from)
                );
            }

            thread.read_with(cx, |thread, cx| {
                assert_eq!(thread.entries().len(), 4);
                for (index, capacity) in [2, 3].into_iter().zip(capacities) {
                    assert_eq!(
                        test_message_content(thread, index).source_blocks.capacity(),
                        capacity
                    );
                }
                assert_eq!(
                    thread.to_markdown(cx),
                    concat!(
                        "## User\n\nfirst\n\n",
                        "## Assistant\n\nfirst reply\n\n",
                        "## User\n\nsecond\n\n",
                        "## Assistant\n\nsecond reply\n\n",
                    )
                );
                assert!(!thread.had_error());
                assert_eq!(thread.turn_id, 2);
                assert_eq!(thread.running_turn.as_ref().map(|turn| turn.id), Some(2));
                assert_eq!(thread.status(), ThreadStatus::Generating);
                assert!(matches!(
                    thread.idle_sleep_prevention,
                    IdleSleepPrevention::Active { .. }
                ));
            });
            assert_eq!(*events.borrow(), Vec::<&str>::new());
            assert_eq!(cx.active_idle_sleep_preventions(), 1);

            second_complete
                .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
                .expect("second backend should still be running");
            second_request.await.expect("second turn should complete");
            assert_eq!(cx.active_idle_sleep_preventions(), 0);
        }
    }

    async fn assert_failed_tool_update_resumes_sleep_prevention(
        upsert: bool,
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let thread = new_test_thread(cx).await;
        let (complete, request) = start_test_turn(&thread, cx);
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        let tool_call_id = acp_v2::ToolCallId::new("permission");
        let (permission_id, permission) =
            request_test_permission_with_id(&thread, tool_call_id.clone(), cx);
        cx.run_until_parked();
        assert!(thread.read_with(cx, |thread, _| thread.is_waiting_for_confirmation()));
        assert_eq!(cx.active_idle_sleep_preventions(), 0);

        thread.update(cx, |thread, cx| {
            let update = acp_v1::ToolCallUpdate::new(
                acp_v1::ToolCallId::new(tool_call_id.0.clone()),
                acp_v1::ToolCallUpdateFields::new()
                    .status(acp_v1::ToolCallStatus::Completed)
                    .content(vec![acp_v1::ToolCallContent::Terminal(
                        acp_v1::Terminal::new(acp_v1::TerminalId::new("unknown-terminal")),
                    )]),
            );
            if upsert {
                assert!(
                    thread
                        .upsert_tool_call_inner(update, Some(ToolCallStatus::Completed), cx)
                        .is_err()
                );
            } else {
                assert_eq!(
                    thread
                        .update_tool_call(update, cx)
                        .map_err(|error| error.to_string()),
                    Err(String::from(
                        "Terminal with id `unknown-terminal` not found"
                    ))
                );
            }
            assert_eq!(
                thread
                    .tool_call(&tool_call_id)
                    .expect("tool call should remain present")
                    .1
                    .permission_status(),
                Some(acp_v2::ToolCallStatus::Completed)
            );
            assert!(!thread.is_waiting_for_confirmation());
            assert!(thread.permission_request(permission_id).is_none());
            assert!(thread.permission_request_for_tool(&tool_call_id).is_none());
            assert_eq!(thread.pending_permission_requests().count(), 0);
            assert!(
                thread
                    .tool_call(&tool_call_id)
                    .expect("terminal tool")
                    .1
                    .authorization_id()
                    .is_none()
            );
        });
        assert!(matches!(
            permission.await,
            RequestPermissionOutcome::Cancelled
        ));
        cx.run_until_parked();
        assert_eq!(cx.active_idle_sleep_preventions(), 1);
        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.status(), ThreadStatus::Generating);
            assert_eq!(thread.running_turn.as_ref().map(|turn| turn.id), Some(1));
            assert!(!thread.had_error());
            assert!(matches!(
                thread.idle_sleep_prevention,
                IdleSleepPrevention::Active { .. }
            ));
        });

        complete
            .send(Ok(acp_v1::PromptResponse::new(acp_v1::StopReason::EndTurn)))
            .expect("backend should still be running");
        request.await.expect("turn should complete");
        assert_eq!(cx.active_idle_sleep_preventions(), 0);
    }
}
