mod connection;
mod diff;
mod mention;
mod terminal;
pub use ::terminal::HeadlessTerminal;
use action_log::{ActionLog, ActionLogTelemetry};
use agent_client_protocol::schema::{MaybeUndefined, v1 as acp};
use anyhow::{Context as _, Result, anyhow};
use collections::HashSet;
pub use connection::*;
pub use diff::*;
use feature_flags::{AcpBetaFeatureFlag, FeatureFlagAppExt as _};
use futures::{FutureExt, channel::oneshot, future::BoxFuture};
use gpui::{
    AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription, Task,
    WeakEntity,
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
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Formatter, Write};
use std::ops::Range;
use std::process::ExitStatus;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{fmt::Display, mem, path::PathBuf, sync::Arc};
use task::{Shell, ShellBuilder};
pub use terminal::*;
use text::Bias;
use ui::App;
use util::markdown::MarkdownEscaped;
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

/// Key used in ACP ToolCall meta to store the tool's programmatic name.
/// This is a workaround since ACP's ToolCall doesn't have a dedicated name field.
pub const TOOL_NAME_META_KEY: &str = "tool_name";

/// Helper to extract tool name from ACP meta
pub fn tool_name_from_meta(meta: &Option<acp::Meta>) -> Option<SharedString> {
    meta.as_ref()
        .and_then(|m| m.get(TOOL_NAME_META_KEY))
        .and_then(|v| v.as_str())
        .map(|s| SharedString::from(s.to_owned()))
}

/// Helper to create meta with tool name
pub fn meta_with_tool_name(tool_name: &str) -> acp::Meta {
    acp::Meta::from_iter([(TOOL_NAME_META_KEY.into(), tool_name.into())])
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

pub fn meta_with_command_category(category: CommandCategory) -> acp::Meta {
    acp::Meta::from_iter([(COMMAND_CATEGORY_META_KEY.into(), category.as_str().into())])
}

pub fn command_category_from_meta(meta: &Option<acp::Meta>) -> Option<CommandCategory> {
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
    pub write_paths: Vec<PathBuf>,
    /// The agent-provided justification for requesting these permissions,
    /// shown to the user (attributed to the agent) in the approval prompt.
    #[serde(default)]
    pub reason: String,
}

pub fn meta_with_sandbox_authorization(details: SandboxAuthorizationDetails) -> acp::Meta {
    acp::Meta::from_iter([(
        SANDBOX_AUTHORIZATION_META_KEY.into(),
        serde_json::to_value(details).unwrap_or_default(),
    )])
}

pub fn sandbox_authorization_details_from_meta(
    meta: &Option<acp::Meta>,
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
}

pub fn meta_with_sandbox_fallback_authorization(
    details: SandboxFallbackAuthorizationDetails,
) -> acp::Meta {
    acp::Meta::from_iter([(
        SANDBOX_FALLBACK_AUTHORIZATION_META_KEY.into(),
        serde_json::to_value(details).unwrap_or_default(),
    )])
}

pub fn sandbox_fallback_authorization_details_from_meta(
    meta: &Option<acp::Meta>,
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

pub fn meta_with_sandbox_not_applied(reason: &SandboxNotAppliedReason) -> acp::Meta {
    acp::Meta::from_iter([(
        SANDBOX_NOT_APPLIED_META_KEY.into(),
        serde_json::to_value(reason).unwrap_or_default(),
    )])
}

pub fn sandbox_not_applied_from_meta(meta: &Option<acp::Meta>) -> Option<SandboxNotAppliedReason> {
    meta.as_ref()
        .and_then(|m| m.get(SANDBOX_NOT_APPLIED_META_KEY))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubagentSessionInfo {
    /// The session id of the subagent sessiont that was spawned
    pub session_id: acp::SessionId,
    /// The index of the message of the start of the "turn" run by this tool call
    pub message_start_index: usize,
    /// The index of the output of the message that the subagent has returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_end_index: Option<usize>,
}

/// Helper to extract subagent session id from ACP meta
pub fn subagent_session_info_from_meta(meta: &Option<acp::Meta>) -> Option<SubagentSessionInfo> {
    meta.as_ref()
        .and_then(|m| m.get(SUBAGENT_SESSION_INFO_META_KEY))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

#[derive(Debug)]
pub struct UserMessage {
    pub protocol_id: Option<acp::MessageId>,
    pub client_id: Option<ClientUserMessageId>,
    pub is_optimistic: bool,
    pub content: ContentBlock,
    pub chunks: Vec<acp::ContentBlock>,
    pub checkpoint: Option<Checkpoint>,
    pub indented: bool,
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
        id: Option<acp::MessageId>,
        block: ContentBlock,
    },
    Thought {
        id: Option<acp::MessageId>,
        block: ContentBlock,
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
            id: None,
            block: ContentBlock::new(chunk.into(), language_registry, path_style, cx),
        }
    }

    fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::Message { block, .. } => block.to_markdown(cx).to_string(),
            Self::Thought { block, .. } => {
                format!("<thinking>\n{}\n</thinking>", block.to_markdown(cx))
            }
        }
    }
}

fn can_merge_message_chunks(
    existing: Option<&acp::MessageId>,
    incoming: Option<&acp::MessageId>,
) -> bool {
    match (existing, incoming) {
        (Some(existing), Some(incoming)) => existing == incoming,
        _ => true,
    }
}

#[derive(Debug)]
pub enum AgentThreadEntry {
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolCall(ToolCall),
    Elicitation(ElicitationEntryId),
    CompletedPlan(Vec<PlanEntry>),
    ContextCompaction(ContextCompaction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElicitationEntryId(pub Arc<str>);

#[derive(Debug)]
pub struct Elicitation {
    pub id: ElicitationEntryId,
    pub request: acp::CreateElicitationRequest,
    pub status: ElicitationStatus,
}

#[derive(Debug)]
pub enum ElicitationStatus {
    Pending {
        respond_tx: oneshot::Sender<acp::CreateElicitationResponse>,
    },
    Accepted,
    Declined,
    Canceled,
    Completed,
}

#[derive(Clone, Debug)]
pub enum ElicitationStoreEvent {
    ElicitationRequested(ElicitationEntryId),
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

    fn validate_request(
        request: &acp::CreateElicitationRequest,
        cx: &App,
    ) -> Result<(), acp::Error> {
        if !cx.has_flag::<AcpBetaFeatureFlag>() {
            return Err(
                acp::Error::invalid_params().data("elicitation support requires the ACP beta flag")
            );
        }

        if let acp::ElicitationMode::Url(mode) = &request.mode {
            url::Url::parse(&mode.url)
                .map_err(|_| acp::Error::invalid_params().data("invalid elicitation URL"))?;
        }

        Ok(())
    }

    fn insert_pending_elicitation(
        &mut self,
        request: acp::CreateElicitationRequest,
    ) -> (
        ElicitationEntryId,
        oneshot::Receiver<acp::CreateElicitationResponse>,
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

    fn response_task<T>(
        id: ElicitationEntryId,
        response_rx: oneshot::Receiver<acp::CreateElicitationResponse>,
        cx: &mut Context<T>,
        emit_responded: impl FnOnce(&mut T, &mut Context<T>, ElicitationEntryId) + 'static,
    ) -> Task<acp::CreateElicitationResponse>
    where
        T: 'static,
    {
        cx.spawn(async move |this, cx| {
            let response = response_rx.await.unwrap_or_else(|oneshot::Canceled| {
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Cancel)
            });
            this.update(cx, |this, cx| emit_responded(this, cx, id))
                .ok();
            response
        })
    }

    fn respond_to_elicitation_entry(
        elicitation: &mut Elicitation,
        response: acp::CreateElicitationResponse,
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
        respond_tx.send(response).ok();
        true
    }

    fn complete_url_elicitation_entry(elicitation: &mut Elicitation) -> bool {
        let previous_status = mem::replace(&mut elicitation.status, ElicitationStatus::Completed);
        match previous_status {
            ElicitationStatus::Pending { respond_tx } => {
                respond_tx
                    .send(acp::CreateElicitationResponse::new(
                        acp::ElicitationAction::Accept(acp::ElicitationAcceptAction::new()),
                    ))
                    .ok();
                true
            }
            ElicitationStatus::Accepted => true,
            ElicitationStatus::Completed => false,
            previous_status @ (ElicitationStatus::Declined | ElicitationStatus::Canceled) => {
                elicitation.status = previous_status;
                false
            }
        }
    }

    fn cancel_elicitation_entry(
        elicitation: &mut Elicitation,
        cancel_accepted_url_elicitations: bool,
    ) -> bool {
        match mem::replace(&mut elicitation.status, ElicitationStatus::Canceled) {
            ElicitationStatus::Pending { respond_tx } => {
                respond_tx
                    .send(acp::CreateElicitationResponse::new(
                        acp::ElicitationAction::Cancel,
                    ))
                    .ok();
                true
            }
            ElicitationStatus::Accepted
                if cancel_accepted_url_elicitations
                    && matches!(&elicitation.request.mode, acp::ElicitationMode::Url(_)) =>
            {
                true
            }
            previous_status => {
                elicitation.status = previous_status;
                false
            }
        }
    }

    fn respond_to_elicitation_by_id(
        &mut self,
        id: &ElicitationEntryId,
        response: acp::CreateElicitationResponse,
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

    fn cancel_elicitation_by_id(
        &mut self,
        id: &ElicitationEntryId,
        cancel_accepted_url_elicitations: bool,
    ) -> bool {
        let Some((_, elicitation)) = self.elicitation_mut(id) else {
            return false;
        };
        Self::cancel_elicitation_entry(elicitation, cancel_accepted_url_elicitations)
    }

    pub fn request_elicitation(
        &mut self,
        request: acp::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<Task<acp::CreateElicitationResponse>, acp::Error> {
        self.request_elicitation_with_id(request, cx)
            .map(|(_, task)| task)
    }

    pub fn request_elicitation_with_id(
        &mut self,
        request: acp::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<(ElicitationEntryId, Task<acp::CreateElicitationResponse>), acp::Error> {
        Self::validate_request(&request, cx)?;
        let (id, response_rx) = self.insert_pending_elicitation(request);
        cx.emit(ElicitationStoreEvent::ElicitationRequested(id.clone()));
        cx.notify();

        let task = Self::response_task(id.clone(), response_rx, cx, |_store, cx, id| {
            cx.emit(ElicitationStoreEvent::ElicitationResponded(id));
            cx.notify();
        });

        Ok((id, task))
    }

    pub fn respond_to_elicitation(
        &mut self,
        id: &ElicitationEntryId,
        response: acp::CreateElicitationResponse,
        cx: &mut Context<Self>,
    ) {
        if !self.respond_to_elicitation_by_id(id, response) {
            return;
        }

        cx.emit(ElicitationStoreEvent::ElicitationUpdated(id.clone()));
        cx.notify();
    }

    pub fn complete_url_elicitation(
        &mut self,
        elicitation_id: &acp::ElicitationId,
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
        if !self.cancel_elicitation_by_id(id, true) {
            return;
        }

        cx.emit(ElicitationStoreEvent::ElicitationUpdated(id.clone()));
        cx.notify();
    }

    pub fn cancel_all(&mut self, cx: &mut Context<Self>) {
        let canceled_ids = self.cancel_pending(|_| true);
        for id in canceled_ids {
            cx.emit(ElicitationStoreEvent::ElicitationUpdated(id));
        }
        cx.notify();
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        let canceled_ids = self.cancel_pending(|_| true);
        self.elicitations.clear();
        for id in canceled_ids {
            cx.emit(ElicitationStoreEvent::ElicitationUpdated(id));
        }
        cx.notify();
    }

    pub fn clear_resolved(&mut self, cx: &mut Context<Self>) -> Vec<ElicitationEntryId> {
        let mut cleared_ids = Vec::new();
        self.elicitations.retain(|elicitation| {
            let keep = matches!(
                (&elicitation.status, &elicitation.request.mode),
                (ElicitationStatus::Pending { .. }, _)
                    | (ElicitationStatus::Accepted, acp::ElicitationMode::Url(_))
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

    pub fn cancel_request(&mut self, request_id: &acp::RequestId, cx: &mut Context<Self>) {
        let canceled_ids = self.cancel_pending(|elicitation| {
            matches!(
                elicitation.request.scope(),
                acp::ElicitationScope::Request(scope) if &scope.request_id == request_id
            )
        });
        for id in canceled_ids {
            cx.emit(ElicitationStoreEvent::ElicitationUpdated(id));
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
        elicitation_id: &acp::ElicitationId,
    ) -> Option<ElicitationEntryId> {
        self.elicitations.iter().rev().find_map(|elicitation| {
            if let acp::ElicitationMode::Url(mode) = &elicitation.request.mode
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
    ) -> Vec<ElicitationEntryId> {
        let mut canceled_ids = Vec::new();
        for elicitation in &mut self.elicitations {
            if should_cancel(elicitation) && Self::cancel_elicitation_entry(elicitation, true) {
                canceled_ids.push(elicitation.id.clone());
            }
        }
        canceled_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextCompactionId(pub Arc<str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextCompactionStatus {
    InProgress,
    Completed,
    Canceled,
}

/// A point in the thread where the conversation history was compacted to free
/// up room in the model's context window. The summary can be expanded to inspect
/// what the model retained.
#[derive(Debug)]
pub struct ContextCompaction {
    pub id: ContextCompactionId,
    pub status: ContextCompactionStatus,
    /// The compaction summary, streamed in as the model produces it. This is
    /// `None` for provider-native compaction, which produces no summary to show.
    pub summary: Option<Entity<Markdown>>,
}

impl ContextCompaction {
    pub fn is_in_progress(&self) -> bool {
        self.status == ContextCompactionStatus::InProgress
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
            Self::CompletedPlan(_) => false,
            Self::ContextCompaction(_) => false,
        }
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::UserMessage(message) => message.to_markdown(cx),
            Self::AssistantMessage(message) => message.to_markdown(cx),
            Self::ToolCall(tool_call) => tool_call.to_markdown(cx),
            Self::Elicitation(_) => "## Input Requested\n\n".to_string(),
            Self::CompletedPlan(entries) => {
                let mut md = String::from("## Plan\n\n");
                for entry in entries {
                    let source = entry.content.read(cx).source().to_string();
                    md.push_str(&format!("- [x] {}\n", source));
                }
                md
            }
            Self::ContextCompaction(_) => "--- Context Compacted ---\n\n".to_string(),
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

    pub fn location(&self, ix: usize) -> Option<(acp::ToolCallLocation, AgentLocation)> {
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

#[derive(Debug)]
pub struct ToolCall {
    pub id: acp::ToolCallId,
    pub label: Entity<Markdown>,
    pub kind: acp::ToolKind,
    pub content: Vec<ToolCallContent>,
    pub status: ToolCallStatus,
    pub locations: Vec<acp::ToolCallLocation>,
    pub resolved_locations: Vec<Option<AgentLocation>>,
    pub raw_input: Option<serde_json::Value>,
    pub raw_input_markdown: Option<Entity<Markdown>>,
    pub raw_output: Option<serde_json::Value>,
    pub tool_name: Option<SharedString>,
    pub subagent_session_info: Option<SubagentSessionInfo>,
    pub sandbox_authorization_details: Option<SandboxAuthorizationDetails>,
    pub sandbox_fallback_authorization_details: Option<SandboxFallbackAuthorizationDetails>,
    /// Why this terminal command ran without the OS sandbox even though
    /// sandboxing was active (see [`SANDBOX_NOT_APPLIED_META_KEY`]). `None` when
    /// the command was sandboxed normally (or sandboxing was off).
    pub sandbox_not_applied: Option<SandboxNotAppliedReason>,
}

impl ToolCall {
    fn from_acp(
        tool_call: acp::ToolCall,
        status: ToolCallStatus,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<Self> {
        let title = if tool_call.kind == acp::ToolKind::Execute {
            tool_call.title
        } else if tool_call.kind == acp::ToolKind::Edit {
            MarkdownEscaped(tool_call.title.as_str()).to_string()
        } else if let Some((first_line, _)) = tool_call.title.split_once("\n") {
            first_line.to_owned() + "…"
        } else {
            tool_call.title
        };
        let mut content = Vec::with_capacity(tool_call.content.len());
        for item in tool_call.content {
            if let Some(item) = ToolCallContent::from_acp(
                item,
                language_registry.clone(),
                path_style,
                terminals,
                cx,
            )? {
                content.push(item);
            }
        }

        let raw_input_markdown = tool_call
            .raw_input
            .as_ref()
            .and_then(|input| markdown_for_raw_output(input, &language_registry, cx));

        let tool_name = tool_name_from_meta(&tool_call.meta);

        let subagent_session_info = subagent_session_info_from_meta(&tool_call.meta);
        let sandbox_authorization_details =
            sandbox_authorization_details_from_meta(&tool_call.meta);
        let sandbox_fallback_authorization_details =
            sandbox_fallback_authorization_details_from_meta(&tool_call.meta);
        let sandbox_not_applied = sandbox_not_applied_from_meta(&tool_call.meta);

        let label = if tool_call.kind == acp::ToolKind::Execute {
            cx.new(|cx| Markdown::new_text(title.into(), cx))
        } else {
            cx.new(|cx| Markdown::new(title.into(), Some(language_registry.clone()), None, cx))
        };

        let result = Self {
            id: tool_call.tool_call_id,
            label,
            kind: tool_call.kind,
            content,
            locations: tool_call.locations,
            resolved_locations: Vec::default(),
            status,
            raw_input: tool_call.raw_input,
            raw_input_markdown,
            raw_output: tool_call.raw_output,
            tool_name,
            subagent_session_info,
            sandbox_authorization_details,
            sandbox_fallback_authorization_details,
            sandbox_not_applied,
        };
        Ok(result)
    }

    fn update_fields(
        &mut self,
        fields: acp::ToolCallUpdateFields,
        meta: Option<acp::Meta>,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<()> {
        let acp::ToolCallUpdateFields {
            kind,
            status,
            title,
            content,
            locations,
            raw_input,
            raw_output,
            ..
        } = fields;

        if let Some(kind) = kind {
            self.kind = kind;
        }

        if let Some(status) = status {
            self.update_acp_status(status);
        }

        if let Some(subagent_session_info) = subagent_session_info_from_meta(&meta) {
            self.subagent_session_info = Some(subagent_session_info);
        }
        if let Some(sandbox_authorization_details) = sandbox_authorization_details_from_meta(&meta)
        {
            self.sandbox_authorization_details = Some(sandbox_authorization_details);
        }
        if let Some(sandbox_fallback_authorization_details) =
            sandbox_fallback_authorization_details_from_meta(&meta)
        {
            self.sandbox_fallback_authorization_details =
                Some(sandbox_fallback_authorization_details);
        }
        if let Some(sandbox_not_applied) = sandbox_not_applied_from_meta(&meta) {
            self.sandbox_not_applied = Some(sandbox_not_applied);
        }

        if let Some(title) = title {
            if self.kind == acp::ToolKind::Execute {
                for terminal in self.terminals() {
                    terminal.update(cx, |terminal, cx| {
                        terminal.update_command_label(&title, cx);
                    });
                }
            }
            self.label.update(cx, |label, cx| {
                if self.kind == acp::ToolKind::Execute {
                    label.replace(title, cx);
                } else if self.kind == acp::ToolKind::Edit {
                    label.replace(MarkdownEscaped(&title).to_string(), cx)
                } else if let Some((first_line, _)) = title.split_once("\n") {
                    label.replace(first_line.to_owned() + "…", cx);
                } else {
                    label.replace(title, cx);
                }
            });
        }

        if let Some(content) = content {
            let mut new_content_len = content.len();
            let mut content = content.into_iter();

            // Reuse existing content if we can
            for (old, new) in self.content.iter_mut().zip(content.by_ref()) {
                let valid_content =
                    old.update_from_acp(new, language_registry.clone(), path_style, terminals, cx)?;
                if !valid_content {
                    new_content_len -= 1;
                }
            }
            for new in content {
                if let Some(new) = ToolCallContent::from_acp(
                    new,
                    language_registry.clone(),
                    path_style,
                    terminals,
                    cx,
                )? {
                    self.content.push(new);
                } else {
                    new_content_len -= 1;
                }
            }
            self.content.truncate(new_content_len);
        }

        if let Some(locations) = locations {
            self.locations = locations;
        }

        if let Some(raw_input) = raw_input {
            self.raw_input_markdown = markdown_for_raw_output(&raw_input, &language_registry, cx);
            self.raw_input = Some(raw_input);
        }

        if let Some(raw_output) = raw_output {
            if self.content.is_empty()
                && let Some(markdown) = markdown_for_raw_output(&raw_output, &language_registry, cx)
            {
                self.content
                    .push(ToolCallContent::ContentBlock(ContentBlock::Markdown {
                        markdown,
                    }));
            }
            self.raw_output = Some(raw_output);
        }
        Ok(())
    }

    fn update_status(&mut self, status: ToolCallStatus) {
        match status {
            ToolCallStatus::Pending => self.update_acp_status(acp::ToolCallStatus::Pending),
            ToolCallStatus::InProgress => self.update_acp_status(acp::ToolCallStatus::InProgress),
            ToolCallStatus::Completed => self.update_acp_status(acp::ToolCallStatus::Completed),
            ToolCallStatus::Failed => self.update_acp_status(acp::ToolCallStatus::Failed),
            status @ (ToolCallStatus::WaitingForConfirmation { .. }
            | ToolCallStatus::Rejected
            | ToolCallStatus::Canceled) => self.status = status,
        }
    }

    fn update_acp_status(&mut self, status: acp::ToolCallStatus) {
        if let ToolCallStatus::WaitingForConfirmation { current_status, .. } = &mut self.status
            && matches!(
                status,
                acp::ToolCallStatus::Pending | acp::ToolCallStatus::InProgress
            )
        {
            *current_status = status;
        } else {
            self.status = status.into();
        }
    }

    pub fn diffs(&self) -> impl Iterator<Item = &Entity<Diff>> {
        self.content.iter().filter_map(|content| match content {
            ToolCallContent::Diff(diff) => Some(diff),
            ToolCallContent::ContentBlock(_) => None,
            ToolCallContent::Terminal(_) => None,
        })
    }

    pub fn terminals(&self) -> impl Iterator<Item = &Entity<Terminal>> {
        self.content.iter().filter_map(|content| match content {
            ToolCallContent::Terminal(terminal) => Some(terminal),
            ToolCallContent::ContentBlock(_) => None,
            ToolCallContent::Diff(_) => None,
        })
    }

    pub fn is_subagent(&self) -> bool {
        self.tool_name.as_ref().is_some_and(|s| s == "spawn_agent")
            || self.subagent_session_info.is_some()
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        let mut markdown = format!(
            "**Tool Call: {}**\nStatus: {}\n\n",
            self.label.read(cx).source(),
            self.status
        );
        for content in &self.content {
            markdown.push_str(content.to_markdown(cx).as_str());
            markdown.push_str("\n\n");
        }
        markdown
    }

    async fn resolve_location(
        location: acp::ToolCallLocation,
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

// Separate so we can hold a strong reference to the buffer
// for saving on the thread
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
    pub option_id: acp::PermissionOptionId,
    pub option_kind: acp::PermissionOptionKind,
    pub params: Option<SelectedPermissionParams>,
}

impl SelectedPermissionOutcome {
    pub fn new(option_id: acp::PermissionOptionId, option_kind: acp::PermissionOptionKind) -> Self {
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

impl From<SelectedPermissionOutcome> for acp::SelectedPermissionOutcome {
    fn from(value: SelectedPermissionOutcome) -> Self {
        Self::new(value.option_id)
    }
}

#[derive(Debug)]
pub enum RequestPermissionOutcome {
    Cancelled,
    Selected(SelectedPermissionOutcome),
}

impl From<RequestPermissionOutcome> for acp::RequestPermissionOutcome {
    fn from(value: RequestPermissionOutcome) -> Self {
        match value {
            RequestPermissionOutcome::Cancelled => Self::Cancelled,
            RequestPermissionOutcome::Selected(outcome) => Self::Selected(outcome.into()),
        }
    }
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

#[derive(Debug)]
pub enum ToolCallStatus {
    /// The tool call hasn't started running yet, but we start showing it to
    /// the user.
    Pending,
    /// The tool call is waiting for confirmation from the user.
    WaitingForConfirmation {
        current_status: acp::ToolCallStatus,
        options: PermissionOptions,
        respond_tx: oneshot::Sender<SelectedPermissionOutcome>,
        kind: AuthorizationKind,
    },
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

impl From<acp::ToolCallStatus> for ToolCallStatus {
    fn from(status: acp::ToolCallStatus) -> Self {
        match status {
            acp::ToolCallStatus::Pending => Self::Pending,
            acp::ToolCallStatus::InProgress => Self::InProgress,
            acp::ToolCallStatus::Completed => Self::Completed,
            acp::ToolCallStatus::Failed => Self::Failed,
            _ => Self::Pending,
        }
    }
}

impl ToolCallStatus {
    fn as_acp_status(&self) -> Option<acp::ToolCallStatus> {
        match self {
            ToolCallStatus::Pending => Some(acp::ToolCallStatus::Pending),
            ToolCallStatus::WaitingForConfirmation { current_status, .. } => Some(*current_status),
            ToolCallStatus::InProgress => Some(acp::ToolCallStatus::InProgress),
            ToolCallStatus::Completed => Some(acp::ToolCallStatus::Completed),
            ToolCallStatus::Failed => Some(acp::ToolCallStatus::Failed),
            ToolCallStatus::Rejected | ToolCallStatus::Canceled => None,
        }
    }

    fn status_after_permission_grant(status: acp::ToolCallStatus) -> ToolCallStatus {
        match ToolCallStatus::from(status) {
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
                ToolCallStatus::WaitingForConfirmation { .. } => "Waiting for confirmation",
                ToolCallStatus::InProgress => "In Progress",
                ToolCallStatus::Completed => "Completed",
                ToolCallStatus::Failed => "Failed",
                ToolCallStatus::Rejected => "Rejected",
                ToolCallStatus::Canceled => "Canceled",
            }
        )
    }
}

fn elicitation_status_for_response(response: &acp::CreateElicitationResponse) -> ElicitationStatus {
    match &response.action {
        acp::ElicitationAction::Accept(_) => ElicitationStatus::Accepted,
        acp::ElicitationAction::Decline => ElicitationStatus::Declined,
        acp::ElicitationAction::Cancel => ElicitationStatus::Canceled,
        _ => ElicitationStatus::Canceled,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContentBlock {
    Empty,
    Markdown {
        markdown: Entity<Markdown>,
    },
    EmbeddedResource {
        resource: acp::EmbeddedResource,
        markdown: Option<Entity<Markdown>>,
    },
    ResourceLink {
        resource_link: acp::ResourceLink,
    },
    Image {
        image: Arc<gpui::Image>,
        dimensions: Option<gpui::Size<u32>>,
    },
}

impl ContentBlock {
    pub fn new(
        block: acp::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        let mut this = Self::Empty;
        this.append(block, language_registry, path_style, cx);
        this
    }

    pub fn new_combined(
        blocks: impl IntoIterator<Item = acp::ContentBlock>,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        let mut this = Self::Empty;
        for block in blocks {
            this.append(block, &language_registry, path_style, cx);
        }
        this
    }

    pub fn new_tool_call_content(
        block: acp::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        match block {
            acp::ContentBlock::Resource(resource) => {
                if let Some((image, dimensions)) = Self::decode_embedded_resource_image(&resource) {
                    Self::Image { image, dimensions }
                } else {
                    let markdown =
                        Self::embedded_resource_markdown(&resource, language_registry, cx);
                    Self::EmbeddedResource { resource, markdown }
                }
            }
            block => Self::new(block, language_registry, path_style, cx),
        }
    }

    pub fn append(
        &mut self,
        block: acp::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) {
        match (&mut *self, &block) {
            (ContentBlock::Empty, acp::ContentBlock::ResourceLink(resource_link)) => {
                *self = ContentBlock::ResourceLink {
                    resource_link: resource_link.clone(),
                };
            }
            (ContentBlock::Empty, acp::ContentBlock::Image(image_content)) => {
                if let Some((image, dimensions)) = Self::decode_image(image_content) {
                    *self = ContentBlock::Image { image, dimensions };
                } else {
                    let new_content = Self::image_md(image_content);
                    *self = Self::create_markdown_block(new_content, language_registry, cx);
                }
            }
            (ContentBlock::Empty, _) => {
                let new_content = Self::block_string_contents(&block, path_style);
                *self = Self::create_markdown_block(new_content, language_registry, cx);
            }
            (ContentBlock::Markdown { markdown }, _) => {
                let new_content = Self::block_string_contents(&block, path_style);
                markdown.update(cx, |markdown, cx| markdown.append(&new_content, cx));
            }
            (ContentBlock::ResourceLink { resource_link }, _) => {
                let existing_content = Self::resource_link_md(&resource_link.uri, path_style);
                let new_content = Self::block_string_contents(&block, path_style);
                let combined = format!("{}\n{}", existing_content, new_content);
                *self = Self::create_markdown_block(combined, language_registry, cx);
            }
            (ContentBlock::EmbeddedResource { resource, .. }, _) => {
                let existing_content =
                    Self::embedded_resource_string_contents(resource, path_style);
                let new_content = Self::block_string_contents(&block, path_style);
                let combined = format!("{}\n{}", existing_content, new_content);
                *self = Self::create_markdown_block(combined, language_registry, cx);
            }
            (ContentBlock::Image { .. }, _) => {
                let new_content = Self::block_string_contents(&block, path_style);
                let combined = format!("`Image`\n{}", new_content);
                *self = Self::create_markdown_block(combined, language_registry, cx);
            }
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
    pub fn update_text_in_place(&mut self, block: &acp::ContentBlock, cx: &mut App) -> bool {
        let ContentBlock::Markdown { markdown } = self else {
            return false;
        };
        let acp::ContentBlock::Text(text_content) = block else {
            return false;
        };
        let new_content = &text_content.text;
        markdown.update(cx, |markdown, cx| {
            let current = markdown.source().to_string();
            match new_content.strip_prefix(&current) {
                Some("") => {}
                Some(suffix) => markdown.append(suffix, cx),
                None => markdown.reset(new_content.clone().into(), cx),
            }
        });
        true
    }

    fn decode_image(
        image_content: &acp::ImageContent,
    ) -> Option<(Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        Self::decode_image_data(&image_content.data, &image_content.mime_type)
    }

    fn decode_embedded_resource_image(
        resource: &acp::EmbeddedResource,
    ) -> Option<(Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        let acp::EmbeddedResourceResource::BlobResourceContents(blob) = &resource.resource else {
            return None;
        };
        let mime_type = blob.mime_type.as_deref()?;
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
        ContentBlock::Markdown {
            markdown: Self::create_markdown(content, language_registry, cx),
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
        resource: &acp::EmbeddedResource,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Option<Entity<Markdown>> {
        match &resource.resource {
            acp::EmbeddedResourceResource::TextResourceContents(text) => Some(
                Self::create_markdown(Self::text_resource_markdown(text), language_registry, cx),
            ),
            acp::EmbeddedResourceResource::BlobResourceContents(_) => None,
            _ => None,
        }
    }

    fn text_resource_markdown(resource: &acp::TextResourceContents) -> String {
        match text_resource_render_mode(resource.mime_type.as_deref()) {
            TextResourceRenderMode::Markdown => resource.text.clone(),
            TextResourceRenderMode::CodeBlock(language) => {
                Self::fenced_code_block(&resource.text, language)
            }
        }
    }

    pub fn text_content<'a>(&'a self, cx: &'a App) -> Option<&'a str> {
        match self {
            ContentBlock::Markdown { markdown } => Some(markdown.read(cx).source()),
            ContentBlock::EmbeddedResource { resource, .. } => match &resource.resource {
                acp::EmbeddedResourceResource::TextResourceContents(text) => Some(&text.text),
                acp::EmbeddedResourceResource::BlobResourceContents(_) => None,
                _ => None,
            },
            ContentBlock::Empty
            | ContentBlock::ResourceLink { .. }
            | ContentBlock::Image { .. } => None,
        }
    }

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
        resource: &acp::EmbeddedResource,
        path_style: PathStyle,
    ) -> String {
        match &resource.resource {
            acp::EmbeddedResourceResource::TextResourceContents(text) => {
                Self::resource_link_md(&text.uri, path_style)
            }
            acp::EmbeddedResourceResource::BlobResourceContents(blob) => {
                Self::resource_link_md(&blob.uri, path_style)
            }
            _ => String::new(),
        }
    }

    fn embedded_resource_text(resource: &acp::EmbeddedResource) -> &str {
        match &resource.resource {
            acp::EmbeddedResourceResource::TextResourceContents(text) => &text.text,
            acp::EmbeddedResourceResource::BlobResourceContents(blob) => &blob.uri,
            _ => "",
        }
    }

    fn embedded_resource_label(resource: &acp::EmbeddedResource) -> &str {
        match &resource.resource {
            acp::EmbeddedResourceResource::TextResourceContents(text) => &text.uri,
            acp::EmbeddedResourceResource::BlobResourceContents(blob) => &blob.uri,
            _ => "",
        }
    }

    pub fn embedded_resource(&self) -> Option<(&acp::EmbeddedResource, Option<&Entity<Markdown>>)> {
        match self {
            ContentBlock::EmbeddedResource { resource, markdown } => {
                Some((resource, markdown.as_ref()))
            }
            _ => None,
        }
    }

    pub fn visible_content(&self, cx: &App) -> bool {
        match self {
            ContentBlock::Empty => false,
            ContentBlock::Markdown { markdown } => !markdown.read(cx).source().trim().is_empty(),
            ContentBlock::EmbeddedResource { resource, markdown } => match markdown {
                Some(markdown) => !markdown.read(cx).source().trim().is_empty(),
                None => !Self::embedded_resource_text(resource).trim().is_empty(),
            },
            ContentBlock::ResourceLink { .. } | ContentBlock::Image { .. } => true,
        }
    }

    fn block_string_contents(block: &acp::ContentBlock, path_style: PathStyle) -> String {
        match block {
            acp::ContentBlock::Text(text_content) => text_content.text.clone(),
            acp::ContentBlock::ResourceLink(resource_link) => {
                Self::resource_link_md(&resource_link.uri, path_style)
            }
            acp::ContentBlock::Resource(acp::EmbeddedResource {
                resource:
                    acp::EmbeddedResourceResource::TextResourceContents(acp::TextResourceContents {
                        uri,
                        ..
                    }),
                ..
            }) => Self::resource_link_md(uri, path_style),
            acp::ContentBlock::Image(image) => Self::image_md(image),
            _ => String::new(),
        }
    }

    fn resource_link_md(uri: &str, path_style: PathStyle) -> String {
        if let Some(uri) = MentionUri::parse(uri, path_style).log_err() {
            uri.as_link().to_string()
        } else {
            uri.to_string()
        }
    }

    fn image_md(_image: &acp::ImageContent) -> String {
        "`Image`".into()
    }

    pub fn to_markdown<'a>(&'a self, cx: &'a App) -> &'a str {
        match self {
            ContentBlock::Empty => "",
            ContentBlock::Markdown { markdown } => markdown.read(cx).source(),
            ContentBlock::EmbeddedResource { resource, markdown } => {
                if let Some(markdown) = markdown {
                    markdown.read(cx).source()
                } else {
                    Self::embedded_resource_label(resource)
                }
            }
            ContentBlock::ResourceLink { resource_link } => &resource_link.uri,
            ContentBlock::Image { .. } => "`Image`",
        }
    }

    pub fn markdown(&self) -> Option<&Entity<Markdown>> {
        match self {
            ContentBlock::Empty => None,
            ContentBlock::Markdown { markdown } => Some(markdown),
            ContentBlock::EmbeddedResource { markdown, .. } => markdown.as_ref(),
            ContentBlock::ResourceLink { .. } => None,
            ContentBlock::Image { .. } => None,
        }
    }

    pub fn resource_link(&self) -> Option<&acp::ResourceLink> {
        match self {
            ContentBlock::ResourceLink { resource_link } => Some(resource_link),
            _ => None,
        }
    }

    pub fn image(&self) -> Option<(&Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        match self {
            ContentBlock::Image { image, dimensions } => Some((image, *dimensions)),
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
    ContentBlock(ContentBlock),
    Diff(Entity<Diff>),
    Terminal(Entity<Terminal>),
}

impl ToolCallContent {
    pub fn from_acp(
        content: acp::ToolCallContent,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<Option<Self>> {
        match content {
            acp::ToolCallContent::Content(acp::Content { content, .. }) => Ok(Some(
                Self::ContentBlock(ContentBlock::new_tool_call_content(
                    content,
                    &language_registry,
                    path_style,
                    cx,
                )),
            )),
            acp::ToolCallContent::Diff(diff) => Ok(Some(Self::Diff(cx.new(|cx| {
                Diff::finalized(
                    diff.path.to_string_lossy().into_owned(),
                    diff.old_text,
                    diff.new_text,
                    language_registry,
                    cx,
                )
            })))),
            acp::ToolCallContent::Terminal(acp::Terminal { terminal_id, .. }) => terminals
                .get(&terminal_id)
                .cloned()
                .map(|terminal| Some(Self::Terminal(terminal)))
                .ok_or_else(|| anyhow::anyhow!("Terminal with id `{}` not found", terminal_id)),
            _ => Ok(None),
        }
    }

    pub fn update_from_acp(
        &mut self,
        new: acp::ToolCallContent,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<bool> {
        // Update streaming text in place so the rendered markdown element is
        // reused across snapshots instead of being recreated (which flickers).
        if let (
            Self::ContentBlock(block),
            acp::ToolCallContent::Content(acp::Content { content, .. }),
        ) = (&mut *self, &new)
            && block.update_text_in_place(content, cx)
        {
            return Ok(true);
        }

        let needs_update = match (&self, &new) {
            (Self::Diff(old_diff), acp::ToolCallContent::Diff(new_diff)) => {
                old_diff.read(cx).needs_update(
                    new_diff.old_text.as_deref().unwrap_or(""),
                    &new_diff.new_text,
                    cx,
                )
            }
            _ => true,
        };

        if let Some(update) = Self::from_acp(new, language_registry, path_style, terminals, cx)? {
            if needs_update {
                *self = update;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::ContentBlock(content) => content.to_markdown(cx).to_string(),
            Self::Diff(diff) => diff.read(cx).to_markdown(cx),
            Self::Terminal(terminal) => terminal.read(cx).to_markdown(cx),
        }
    }

    pub fn image(&self) -> Option<(&Arc<gpui::Image>, Option<gpui::Size<u32>>)> {
        match self {
            Self::ContentBlock(content) => content.image(),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ToolCallUpdate {
    UpdateFields(acp::ToolCallUpdate),
    UpdateDiff(ToolCallUpdateDiff),
    UpdateTerminal(ToolCallUpdateTerminal),
}

impl ToolCallUpdate {
    fn id(&self) -> &acp::ToolCallId {
        match self {
            Self::UpdateFields(update) => &update.tool_call_id,
            Self::UpdateDiff(diff) => &diff.id,
            Self::UpdateTerminal(terminal) => &terminal.id,
        }
    }
}

impl From<acp::ToolCallUpdate> for ToolCallUpdate {
    fn from(update: acp::ToolCallUpdate) -> Self {
        Self::UpdateFields(update)
    }
}

impl From<ToolCallUpdateDiff> for ToolCallUpdate {
    fn from(diff: ToolCallUpdateDiff) -> Self {
        Self::UpdateDiff(diff)
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolCallUpdateDiff {
    pub id: acp::ToolCallId,
    pub diff: Entity<Diff>,
}

impl From<ToolCallUpdateTerminal> for ToolCallUpdate {
    fn from(terminal: ToolCallUpdateTerminal) -> Self {
        Self::UpdateTerminal(terminal)
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolCallUpdateTerminal {
    pub id: acp::ToolCallId,
    pub terminal: Entity<Terminal>,
}

#[derive(Debug, Default)]
pub struct Plan {
    pub entries: Vec<PlanEntry>,
}

#[derive(Debug)]
pub struct PlanStats<'a> {
    pub in_progress_entry: Option<&'a PlanEntry>,
    pub pending: u32,
    pub completed: u32,
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
        };

        for entry in &self.entries {
            match &entry.status {
                acp::PlanEntryStatus::Pending => {
                    stats.pending += 1;
                }
                acp::PlanEntryStatus::InProgress => {
                    stats.in_progress_entry = stats.in_progress_entry.or(Some(entry));
                    stats.pending += 1;
                }
                acp::PlanEntryStatus::Completed => {
                    stats.completed += 1;
                }
                _ => {}
            }
        }

        stats
    }
}

#[derive(Debug)]
pub struct PlanEntry {
    pub content: Entity<Markdown>,
    pub priority: acp::PlanEntryPriority,
    pub status: acp::PlanEntryStatus,
}

impl PlanEntry {
    pub fn from_acp(entry: acp::PlanEntry, cx: &mut App) -> Self {
        Self {
            content: cx.new(|cx| Markdown::new(entry.content.into(), None, None, cx)),
            priority: entry.priority,
            status: entry.status,
        }
    }
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
    pub meta: Option<acp::Meta>,
}

pub const REFUSAL_FALLBACK_MODEL_META_KEY: &str = "refusal_fallback_model";

pub fn meta_with_refusal_fallback(model_name: &str) -> acp::Meta {
    acp::Meta::from_iter([(REFUSAL_FALLBACK_MODEL_META_KEY.into(), model_name.into())])
}

pub fn refusal_fallback_model_from_meta(meta: &Option<acp::Meta>) -> Option<SharedString> {
    meta.as_ref()
        .and_then(|m| m.get(REFUSAL_FALLBACK_MODEL_META_KEY))
        .and_then(|v| v.as_str())
        .map(|s| SharedString::from(s.to_owned()))
}

struct RunningTurn {
    id: u32,
    send_task: Task<()>,
}

pub struct AcpThread {
    session_id: acp::SessionId,
    work_dirs: Option<PathList>,
    parent_session_id: Option<acp::SessionId>,
    title: Option<SharedString>,
    provisional_title: Option<SharedString>,
    entries: Vec<AgentThreadEntry>,
    elicitations: ElicitationStore,
    plan: Plan,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    _git_store_subscription: Subscription,
    update_last_checkpoint_if_changed_task: Option<Task<Result<()>>>,
    shared_buffers: HashMap<Entity<Buffer>, BufferSnapshot>,
    turn_id: u32,
    running_turn: Option<RunningTurn>,
    connection: Rc<dyn AgentConnection>,
    token_usage: Option<TokenUsage>,
    cost: Option<SessionCost>,
    prompt_capabilities: acp::PromptCapabilities,
    available_commands: Vec<acp::AvailableCommand>,
    _observe_prompt_capabilities: Task<anyhow::Result<()>>,
    terminals: HashMap<acp::TerminalId, Entity<Terminal>>,
    pending_terminal_output: HashMap<acp::TerminalId, Vec<Vec<u8>>>,
    pending_terminal_exit: HashMap<acp::TerminalId, acp::TerminalExitStatus>,
    had_error: bool,
    /// The user's unsent prompt text, persisted so it can be restored when reloading the thread.
    draft_prompt: Option<Vec<acp::ContentBlock>>,
    /// The initial scroll position for the thread view, set during session registration.
    ui_scroll_position: Option<gpui::ListOffset>,
    /// Buffer for smooth text streaming. Holds text that has been received from
    /// the model but not yet revealed in the UI. A timer task drains this buffer
    /// gradually to create a fluid typing effect instead of choppy chunk-at-a-time
    /// updates.
    streaming_text_buffer: Option<StreamingTextBuffer>,
}

struct StreamingTextBuffer {
    /// Text received from the model but not yet appended to the Markdown source.
    pending: String,
    /// The number of bytes to reveal per timer turn.
    bytes_to_reveal_per_tick: usize,
    /// The Markdown entity being streamed into.
    target: Entity<Markdown>,
    /// Timer task that periodically moves text from `pending` into `source`.
    _reveal_task: Task<()>,
}

impl StreamingTextBuffer {
    /// The number of milliseconds between each timer tick, controlling how quickly
    /// text is revealed.
    const TASK_UPDATE_MS: u64 = 16;
    /// The time in milliseconds to reveal the entire pending text.
    const REVEAL_TARGET: f32 = 200.0;
}

impl From<&AcpThread> for ActionLogTelemetry {
    fn from(value: &AcpThread) -> Self {
        Self {
            agent_telemetry_id: value.connection().telemetry_id(),
            session_id: value.session_id.0.clone(),
        }
    }
}

#[derive(Debug)]
pub enum AcpThreadEvent {
    StatusChanged,
    PromptUpdated,
    NewEntry,
    TitleUpdated,
    TokenUsageUpdated,
    EntryUpdated(usize),
    EntriesRemoved(Range<usize>),
    ToolAuthorizationRequested(acp::ToolCallId),
    ToolAuthorizationReceived(acp::ToolCallId),
    ElicitationRequested(ElicitationEntryId),
    ElicitationResponded(ElicitationEntryId),
    Retry(RetryStatus),
    SubagentSpawned(acp::SessionId),
    Stopped(acp::StopReason),
    Error,
    LoadError(LoadError),
    PromptCapabilitiesUpdated,
    Refusal,
    AvailableCommandsUpdated(Vec<acp::AvailableCommand>),
    ModeUpdated(acp::SessionModeId),
    ConfigOptionsUpdated(Vec<acp::SessionConfigOption>),
    WorkingDirectoriesUpdated,
}

impl EventEmitter<AcpThreadEvent> for AcpThread {}

#[derive(Debug, Clone)]
pub enum TerminalProviderEvent {
    Created {
        terminal_id: acp::TerminalId,
        label: String,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        terminal: Entity<::terminal::Terminal>,
    },
    Output {
        terminal_id: acp::TerminalId,
        data: Vec<u8>,
    },
    TitleChanged {
        terminal_id: acp::TerminalId,
        title: String,
    },
    Exit {
        terminal_id: acp::TerminalId,
        status: acp::TerminalExitStatus,
    },
}

#[derive(Debug, Clone)]
pub enum TerminalProviderCommand {
    WriteInput {
        terminal_id: acp::TerminalId,
        bytes: Vec<u8>,
    },
    Resize {
        terminal_id: acp::TerminalId,
        cols: u16,
        rows: u16,
    },
    Close {
        terminal_id: acp::TerminalId,
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
        parent_session_id: Option<acp::SessionId>,
        title: Option<SharedString>,
        work_dirs: Option<PathList>,
        connection: Rc<dyn AgentConnection>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        session_id: acp::SessionId,
        mut prompt_capabilities_rx: watch::Receiver<acp::PromptCapabilities>,
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

        Self {
            parent_session_id,
            work_dirs,
            action_log,
            _git_store_subscription,
            update_last_checkpoint_if_changed_task: None,
            shared_buffers: Default::default(),
            entries: Default::default(),
            elicitations: ElicitationStore::default(),
            plan: Default::default(),
            title,
            provisional_title: None,
            project,
            running_turn: None,
            turn_id: 0,
            connection,
            session_id,
            token_usage: None,
            cost: None,
            prompt_capabilities,
            available_commands: Vec::new(),
            _observe_prompt_capabilities: task,
            terminals: HashMap::default(),
            pending_terminal_output: HashMap::default(),
            pending_terminal_exit: HashMap::default(),
            had_error: false,
            draft_prompt: None,
            ui_scroll_position: None,
            streaming_text_buffer: None,
        }
    }

    pub fn parent_session_id(&self) -> Option<&acp::SessionId> {
        self.parent_session_id.as_ref()
    }

    pub fn prompt_capabilities(&self) -> acp::PromptCapabilities {
        self.prompt_capabilities.clone()
    }

    pub fn available_commands(&self) -> &[acp::AvailableCommand] {
        &self.available_commands
    }

    pub fn is_draft_thread(&self) -> bool {
        self.entries().is_empty()
    }

    pub fn draft_prompt(&self) -> Option<&[acp::ContentBlock]> {
        self.draft_prompt.as_deref()
    }

    pub fn set_draft_prompt(
        &mut self,
        prompt: Option<Vec<acp::ContentBlock>>,
        cx: &mut Context<Self>,
    ) {
        cx.emit(AcpThreadEvent::PromptUpdated);
        self.draft_prompt = prompt;
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
        self.title
            .clone()
            .or_else(|| self.provisional_title.clone())
    }

    pub fn has_provisional_title(&self) -> bool {
        self.provisional_title.is_some()
    }

    pub fn entries(&self) -> &[AgentThreadEntry] {
        &self.entries
    }

    pub fn is_compacting(&self) -> bool {
        self.entries.last().is_some_and(|entry| {
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
                if let Some(markdown) = block.markdown() {
                    markdown.update(cx, |markdown, cx| {
                        markdown.invalidate_mermaid_cache(cx);
                    });
                }
            }
        }
    }

    pub fn session_id(&self) -> &acp::SessionId {
        &self.session_id
    }

    pub fn supports_truncate(&self, cx: &App) -> bool {
        self.connection.truncate(&self.session_id, cx).is_some()
    }

    pub fn work_dirs(&self) -> Option<&PathList> {
        self.work_dirs.as_ref()
    }

    pub fn set_work_dirs(&mut self, work_dirs: PathList, cx: &mut Context<Self>) {
        self.work_dirs = Some(work_dirs);
        cx.emit(AcpThreadEvent::WorkingDirectoriesUpdated)
    }

    pub fn status(&self) -> ThreadStatus {
        if self.running_turn.is_some() {
            ThreadStatus::Generating
        } else {
            ThreadStatus::Idle
        }
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn is_waiting_for_confirmation(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::WaitingForConfirmation { .. },
                    ..
                }) => return true,
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
                | AgentThreadEntry::CompletedPlan(_)
                | AgentThreadEntry::ContextCompaction(_) => {}
            }
        }
        false
    }

    pub fn token_usage(&self) -> Option<&TokenUsage> {
        self.token_usage.as_ref()
    }

    pub fn cost(&self) -> Option<&SessionCost> {
        self.cost.as_ref()
    }

    pub fn has_pending_edit_tool_calls(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(
                    call @ ToolCall {
                        status: ToolCallStatus::InProgress | ToolCallStatus::Pending,
                        ..
                    },
                ) if call.diffs().next().is_some() => {
                    return true;
                }
                AgentThreadEntry::ToolCall(_)
                | AgentThreadEntry::Elicitation(_)
                | AgentThreadEntry::AssistantMessage(_)
                | AgentThreadEntry::CompletedPlan(_)
                | AgentThreadEntry::ContextCompaction(_) => {}
            }
        }

        false
    }

    pub fn has_in_progress_tool_calls(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::InProgress | ToolCallStatus::Pending,
                    ..
                }) => {
                    return true;
                }
                AgentThreadEntry::ToolCall(_)
                | AgentThreadEntry::Elicitation(_)
                | AgentThreadEntry::AssistantMessage(_)
                | AgentThreadEntry::CompletedPlan(_)
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
                | AgentThreadEntry::CompletedPlan(..)
                | AgentThreadEntry::ContextCompaction(_)
                | AgentThreadEntry::Elicitation(_) => continue,
                AgentThreadEntry::ToolCall(..) => return true,
            }
        }

        false
    }

    pub fn handle_session_update(
        &mut self,
        update: acp::SessionUpdate,
        cx: &mut Context<Self>,
    ) -> Result<(), acp::Error> {
        match update {
            acp::SessionUpdate::UserMessageChunk(acp::ContentChunk {
                content,
                message_id,
                ..
            }) => {
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
                        let already_in_user_message = message.is_optimistic
                            && message.chunks.contains(&content)
                            && can_merge_message_chunks(
                                message.protocol_id.as_ref(),
                                message_id.as_ref(),
                            );
                        if already_in_user_message && message.protocol_id.is_none() {
                            message.protocol_id = message_id.clone();
                        }
                        already_in_user_message
                    });
                if !already_in_user_message {
                    self.push_user_content_block_from_agent(message_id, content, cx);
                }
            }
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk {
                content,
                message_id,
                ..
            }) => {
                self.push_assistant_content_block_with_message_id(
                    message_id, content, false, false, cx,
                );
            }
            acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk {
                content,
                message_id,
                ..
            }) => {
                self.push_assistant_content_block_with_message_id(
                    message_id, content, true, false, cx,
                );
            }
            acp::SessionUpdate::ToolCall(tool_call) => {
                self.upsert_tool_call(tool_call, cx)?;
            }
            acp::SessionUpdate::ToolCallUpdate(tool_call_update) => {
                self.update_tool_call(tool_call_update, cx)?;
            }
            acp::SessionUpdate::Plan(plan) => {
                self.update_plan(plan, cx);
            }
            acp::SessionUpdate::SessionInfoUpdate(info_update) => {
                if let MaybeUndefined::Value(title) = info_update.title {
                    let had_provisional = self.provisional_title.take().is_some();
                    let title: SharedString = title.into();
                    if self.title.as_ref() != Some(&title) {
                        self.title = Some(title);
                        cx.emit(AcpThreadEvent::TitleUpdated);
                    } else if had_provisional {
                        cx.emit(AcpThreadEvent::TitleUpdated);
                    }
                }
            }
            acp::SessionUpdate::AvailableCommandsUpdate(acp::AvailableCommandsUpdate {
                available_commands,
                ..
            }) => {
                self.available_commands = available_commands.clone();
                cx.emit(AcpThreadEvent::AvailableCommandsUpdated(available_commands));
            }
            acp::SessionUpdate::CurrentModeUpdate(acp::CurrentModeUpdate {
                current_mode_id,
                ..
            }) => cx.emit(AcpThreadEvent::ModeUpdated(current_mode_id)),
            acp::SessionUpdate::ConfigOptionUpdate(acp::ConfigOptionUpdate {
                config_options,
                ..
            }) => cx.emit(AcpThreadEvent::ConfigOptionsUpdated(config_options)),
            acp::SessionUpdate::UsageUpdate(update) => {
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

    pub fn push_user_content_block(
        &mut self,
        client_id: Option<ClientUserMessageId>,
        chunk: acp::ContentBlock,
        cx: &mut Context<Self>,
    ) {
        self.push_user_content_block_with_indent(client_id, chunk, false, cx)
    }

    pub fn push_user_content_block_with_indent(
        &mut self,
        client_id: Option<ClientUserMessageId>,
        chunk: acp::ContentBlock,
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
        id: Option<acp::MessageId>,
        chunk: acp::ContentBlock,
        cx: &mut Context<Self>,
    ) {
        self.push_user_content_block_with_protocol_id(None, false, id, chunk, false, cx)
    }

    fn push_user_content_block_with_protocol_id(
        &mut self,
        incoming_client_id: Option<ClientUserMessageId>,
        is_optimistic: bool,
        protocol_id: Option<acp::MessageId>,
        chunk: acp::ContentBlock,
        indented: bool,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let entries_len = self.entries.len();

        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::UserMessage(UserMessage {
                protocol_id: existing_protocol_id,
                client_id: existing_client_id,
                content,
                chunks,
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
            Self::flush_streaming_text(&mut self.streaming_text_buffer, cx);
            if let Some(incoming_client_id) = incoming_client_id {
                *existing_client_id = Some(incoming_client_id);
            }
            *existing_is_optimistic |= is_optimistic;
            if existing_protocol_id.is_none() {
                *existing_protocol_id = protocol_id;
            }
            content.append(chunk.clone(), &language_registry, path_style, cx);
            chunks.push(chunk);
            let idx = entries_len - 1;
            cx.emit(AcpThreadEvent::EntryUpdated(idx));
        } else {
            let content = ContentBlock::new(chunk.clone(), &language_registry, path_style, cx);
            self.push_entry(
                AgentThreadEntry::UserMessage(UserMessage {
                    protocol_id,
                    client_id: incoming_client_id,
                    is_optimistic,
                    content,
                    chunks: vec![chunk],
                    checkpoint: None,
                    indented,
                }),
                cx,
            );
        }
    }

    pub fn push_assistant_content_block(
        &mut self,
        chunk: acp::ContentBlock,
        is_thought: bool,
        cx: &mut Context<Self>,
    ) {
        self.push_assistant_content_block_with_indent(chunk, is_thought, false, cx)
    }

    pub fn push_assistant_content_block_with_indent(
        &mut self,
        chunk: acp::ContentBlock,
        is_thought: bool,
        indented: bool,
        cx: &mut Context<Self>,
    ) {
        self.push_assistant_content_block_with_message_id(None, chunk, is_thought, indented, cx)
    }

    fn push_assistant_content_block_with_message_id(
        &mut self,
        message_id: Option<acp::MessageId>,
        chunk: acp::ContentBlock,
        is_thought: bool,
        indented: bool,
        cx: &mut Context<Self>,
    ) {
        let path_style = self.project.read(cx).path_style(cx);

        // For text chunks going to an existing Markdown block, buffer for smooth
        // streaming instead of appending all at once which may feel more choppy.
        if let acp::ContentBlock::Text(text_content) = &chunk {
            if let Some(markdown) =
                self.streaming_markdown_target(message_id.as_ref(), is_thought, indented)
            {
                let entries_len = self.entries.len();
                cx.emit(AcpThreadEvent::EntryUpdated(entries_len - 1));
                self.buffer_streaming_text(&markdown, text_content.text.clone(), cx);
                return;
            }
        }

        let language_registry = self.project.read(cx).languages().clone();
        let entries_len = self.entries.len();
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::AssistantMessage(AssistantMessage {
                chunks,
                indented: existing_indented,
                is_subagent_output: _,
            }) = last_entry
            && *existing_indented == indented
        {
            let idx = entries_len - 1;
            Self::flush_streaming_text(&mut self.streaming_text_buffer, cx);
            cx.emit(AcpThreadEvent::EntryUpdated(idx));
            match (chunks.last_mut(), is_thought) {
                (
                    Some(AssistantMessageChunk::Message {
                        id: existing_id,
                        block,
                    }),
                    false,
                )
                | (
                    Some(AssistantMessageChunk::Thought {
                        id: existing_id,
                        block,
                    }),
                    true,
                ) if can_merge_message_chunks(existing_id.as_ref(), message_id.as_ref()) => {
                    if existing_id.is_none() {
                        *existing_id = message_id;
                    }
                    block.append(chunk, &language_registry, path_style, cx)
                }
                _ => {
                    let block = ContentBlock::new(chunk, &language_registry, path_style, cx);
                    if is_thought {
                        chunks.push(AssistantMessageChunk::Thought {
                            id: message_id,
                            block,
                        })
                    } else {
                        chunks.push(AssistantMessageChunk::Message {
                            id: message_id,
                            block,
                        })
                    }
                }
            }
        } else {
            let block = ContentBlock::new(chunk, &language_registry, path_style, cx);
            let chunk = if is_thought {
                AssistantMessageChunk::Thought {
                    id: message_id,
                    block,
                }
            } else {
                AssistantMessageChunk::Message {
                    id: message_id,
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

    fn streaming_markdown_target(
        &mut self,
        message_id: Option<&acp::MessageId>,
        is_thought: bool,
        indented: bool,
    ) -> Option<Entity<Markdown>> {
        let last_entry = self.entries.last_mut()?;
        if let AgentThreadEntry::AssistantMessage(AssistantMessage {
            chunks,
            indented: existing_indented,
            ..
        }) = last_entry
            && *existing_indented == indented
            && let [.., chunk] = chunks.as_mut_slice()
        {
            match (chunk, is_thought) {
                (
                    AssistantMessageChunk::Message {
                        id: existing_id,
                        block: ContentBlock::Markdown { markdown },
                    },
                    false,
                )
                | (
                    AssistantMessageChunk::Thought {
                        id: existing_id,
                        block: ContentBlock::Markdown { markdown },
                    },
                    true,
                ) if can_merge_message_chunks(existing_id.as_ref(), message_id) => {
                    if existing_id.is_none() {
                        *existing_id = message_id.cloned();
                    }
                    Some(markdown.clone())
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
        markdown: &Entity<Markdown>,
        text: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(buffer) = &mut self.streaming_text_buffer {
            if buffer.target.entity_id() == markdown.entity_id() {
                buffer.pending.push_str(&text);

                buffer.bytes_to_reveal_per_tick = (buffer.pending.len() as f32
                    / StreamingTextBuffer::REVEAL_TARGET
                    * StreamingTextBuffer::TASK_UPDATE_MS as f32)
                    .ceil() as usize;
                return;
            }
            Self::flush_streaming_text(&mut self.streaming_text_buffer, cx);
        }

        let target = markdown.clone();
        let _reveal_task = self.start_streaming_reveal(cx);
        let pending_len = text.len();
        let bytes_to_reveal = (pending_len as f32 / StreamingTextBuffer::REVEAL_TARGET
            * StreamingTextBuffer::TASK_UPDATE_MS as f32)
            .ceil() as usize;
        self.streaming_text_buffer = Some(StreamingTextBuffer {
            pending: text,
            bytes_to_reveal_per_tick: bytes_to_reveal,
            target,
            _reveal_task,
        });
    }

    /// Flush all buffered streaming text into the Markdown entity immediately.
    fn flush_streaming_text(
        streaming_text_buffer: &mut Option<StreamingTextBuffer>,
        cx: &mut Context<Self>,
    ) {
        if let Some(buffer) = streaming_text_buffer.take() {
            if !buffer.pending.is_empty() {
                buffer
                    .target
                    .update(cx, |markdown, cx| markdown.append(&buffer.pending, cx));
            }
        }
    }

    /// Spawns a foreground task that periodically drains
    /// `streaming_text_buffer.pending` into the target `Markdown` entity,
    /// producing smooth, continuous text output.
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

                        if buffer.pending.is_empty() {
                            return true;
                        }

                        let pending_len = buffer.pending.len();

                        let byte_boundary = buffer
                            .pending
                            .ceil_char_boundary(buffer.bytes_to_reveal_per_tick)
                            .min(pending_len);

                        buffer.target.update(cx, |markdown: &mut Markdown, cx| {
                            markdown.append(&buffer.pending[..byte_boundary], cx);
                            buffer.pending.drain(..byte_boundary);
                        });

                        true
                    })
                    .unwrap_or(false);

                if !should_continue {
                    break;
                }
            }
        })
    }

    fn push_entry(&mut self, entry: AgentThreadEntry, cx: &mut Context<Self>) {
        Self::flush_streaming_text(&mut self.streaming_text_buffer, cx);
        self.entries.push(entry);
        cx.emit(AcpThreadEvent::NewEntry);
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
            if compaction.summary.is_none() {
                compaction.summary = Some(cx.new(|cx| {
                    Markdown::new(
                        update.summary_delta.into(),
                        Some(language_registry),
                        None,
                        cx,
                    )
                }));
            } else if let Some(summary) = compaction.summary.clone() {
                summary.update(cx, |markdown, cx| {
                    markdown.append(&update.summary_delta, cx)
                });
            }
        }

        if let Some(status) = update.status {
            compaction.status = status;
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
    }

    pub fn can_set_title(&mut self, cx: &mut Context<Self>) -> bool {
        self.connection.set_title(&self.session_id, cx).is_some()
    }

    pub fn set_title(&mut self, title: SharedString, cx: &mut Context<Self>) -> Task<Result<()>> {
        let had_provisional = self.provisional_title.take().is_some();
        if self.title.as_ref() != Some(&title) {
            self.title = Some(title.clone());
            cx.emit(AcpThreadEvent::TitleUpdated);
            if let Some(set_title) = self.connection.set_title(&self.session_id, cx) {
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

    pub fn subagent_spawned(&mut self, session_id: acp::SessionId, cx: &mut Context<Self>) {
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
        let update = update.into();
        let languages = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);

        let ix = match self.index_for_tool_call(update.id()) {
            Some(ix) => ix,
            None => {
                // Tool call not found - create a failed tool call entry
                let failed_tool_call = ToolCall {
                    id: update.id().clone(),
                    label: cx.new(|cx| Markdown::new("Tool call not found".into(), None, None, cx)),
                    kind: acp::ToolKind::Fetch,
                    content: vec![ToolCallContent::ContentBlock(ContentBlock::new(
                        "Tool call not found".into(),
                        &languages,
                        path_style,
                        cx,
                    ))],
                    status: ToolCallStatus::Failed,
                    locations: Vec::new(),
                    resolved_locations: Vec::new(),
                    raw_input: None,
                    raw_input_markdown: None,
                    raw_output: None,
                    tool_name: None,
                    subagent_session_info: None,
                    sandbox_authorization_details: None,
                    sandbox_fallback_authorization_details: None,
                    sandbox_not_applied: None,
                };
                self.push_entry(AgentThreadEntry::ToolCall(failed_tool_call), cx);
                return Ok(());
            }
        };
        let AgentThreadEntry::ToolCall(call) = &mut self.entries[ix] else {
            unreachable!()
        };

        match update {
            ToolCallUpdate::UpdateFields(update) => {
                let location_updated = update.fields.locations.is_some();
                call.update_fields(
                    update.fields,
                    update.meta,
                    languages,
                    path_style,
                    &self.terminals,
                    cx,
                )?;
                if location_updated {
                    self.resolve_locations(update.tool_call_id, cx);
                }
            }
            ToolCallUpdate::UpdateDiff(update) => {
                call.content.clear();
                call.content.push(ToolCallContent::Diff(update.diff));
            }
            ToolCallUpdate::UpdateTerminal(update) => {
                call.content.clear();
                call.content
                    .push(ToolCallContent::Terminal(update.terminal));
            }
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));

        Ok(())
    }

    /// Updates a tool call if id matches an existing entry, otherwise inserts a new one.
    pub fn upsert_tool_call(
        &mut self,
        tool_call: acp::ToolCall,
        cx: &mut Context<Self>,
    ) -> Result<(), acp::Error> {
        let status = tool_call.status.into();
        self.upsert_tool_call_inner(tool_call.into(), status, cx)
    }

    /// Fails if id does not match an existing entry.
    pub fn upsert_tool_call_inner(
        &mut self,
        update: acp::ToolCallUpdate,
        status: ToolCallStatus,
        cx: &mut Context<Self>,
    ) -> Result<(), acp::Error> {
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let id = update.tool_call_id.clone();

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

        if let Some(ix) = self.index_for_tool_call(&id) {
            let AgentThreadEntry::ToolCall(call) = &mut self.entries[ix] else {
                unreachable!()
            };

            call.update_fields(
                update.fields,
                update.meta,
                language_registry,
                path_style,
                &self.terminals,
                cx,
            )?;
            call.update_status(status);

            cx.emit(AcpThreadEvent::EntryUpdated(ix));
        } else {
            let call = ToolCall::from_acp(
                update.try_into()?,
                status,
                language_registry,
                self.project.read(cx).path_style(cx),
                &self.terminals,
                cx,
            )?;
            self.push_entry(AgentThreadEntry::ToolCall(call), cx);
        };

        self.resolve_locations(id, cx);
        Ok(())
    }

    fn index_for_tool_call(&self, id: &acp::ToolCallId) -> Option<usize> {
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

    fn tool_call_mut(&mut self, id: &acp::ToolCallId) -> Option<(usize, &mut ToolCall)> {
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

    pub fn tool_call(&self, id: &acp::ToolCallId) -> Option<(usize, &ToolCall)> {
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

    pub fn tool_call_for_subagent(&self, session_id: &acp::SessionId) -> Option<&ToolCall> {
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

    pub fn resolve_locations(&mut self, id: acp::ToolCallId, cx: &mut Context<Self>) {
        let project = self.project.clone();
        let should_update_agent_location = self.parent_session_id.is_none();
        let Some((_, tool_call)) = self.tool_call_mut(&id) else {
            return;
        };
        let task = tool_call.resolve_locations(project, cx);
        cx.spawn(async move |this, cx| {
            let resolved_locations = task.await;

            this.update(cx, |this, cx| {
                let project = this.project.clone();

                for location in resolved_locations.iter().flatten() {
                    this.shared_buffers
                        .insert(location.buffer.clone(), location.buffer.read(cx).snapshot());
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

    pub fn request_tool_call_authorization(
        &mut self,
        tool_call: acp::ToolCallUpdate,
        options: PermissionOptions,
        kind: AuthorizationKind,
        cx: &mut Context<Self>,
    ) -> Result<Task<RequestPermissionOutcome>> {
        let (tx, rx) = oneshot::channel();

        let current_status = self
            .tool_call(&tool_call.tool_call_id)
            .and_then(|(_, tool_call)| tool_call.status.as_acp_status())
            .or(tool_call.fields.status)
            .unwrap_or(acp::ToolCallStatus::Pending);
        let status = ToolCallStatus::WaitingForConfirmation {
            current_status,
            options,
            respond_tx: tx,
            kind,
        };

        let tool_call_id = tool_call.tool_call_id.clone();
        self.upsert_tool_call_inner(tool_call, status, cx)?;
        cx.emit(AcpThreadEvent::ToolAuthorizationRequested(
            tool_call_id.clone(),
        ));

        Ok(cx.spawn(async move |this, cx| {
            let outcome = match rx.await {
                Ok(outcome) => RequestPermissionOutcome::Selected(outcome),
                Err(oneshot::Canceled) => RequestPermissionOutcome::Cancelled,
            };
            this.update(cx, |_this, cx| {
                cx.emit(AcpThreadEvent::ToolAuthorizationReceived(tool_call_id))
            })
            .ok();
            outcome
        }))
    }

    pub fn cancel_tool_call_authorization(&mut self, id: &acp::ToolCallId, cx: &mut Context<Self>) {
        let Some((ix, call)) = self.tool_call_mut(id) else {
            return;
        };
        if !matches!(call.status, ToolCallStatus::WaitingForConfirmation { .. }) {
            return;
        }

        call.status = ToolCallStatus::Canceled;
        cx.emit(AcpThreadEvent::EntryUpdated(ix));
        cx.emit(AcpThreadEvent::ToolAuthorizationReceived(id.clone()));
    }

    pub fn authorize_tool_call(
        &mut self,
        id: acp::ToolCallId,
        outcome: SelectedPermissionOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some((ix, call)) = self.tool_call_mut(&id) else {
            return;
        };

        let new_status =
            match &call.status {
                ToolCallStatus::WaitingForConfirmation {
                    kind: AuthorizationKind::ActionChoice,
                    ..
                } => ToolCallStatus::InProgress,
                ToolCallStatus::WaitingForConfirmation { current_status, .. } => {
                    match outcome.option_kind {
                        acp::PermissionOptionKind::RejectOnce
                        | acp::PermissionOptionKind::RejectAlways => ToolCallStatus::Rejected,
                        acp::PermissionOptionKind::AllowOnce
                        | acp::PermissionOptionKind::AllowAlways => {
                            ToolCallStatus::status_after_permission_grant(*current_status)
                        }
                        _ => ToolCallStatus::status_after_permission_grant(*current_status),
                    }
                }
                _ => match outcome.option_kind {
                    acp::PermissionOptionKind::RejectOnce
                    | acp::PermissionOptionKind::RejectAlways => ToolCallStatus::Rejected,
                    acp::PermissionOptionKind::AllowOnce
                    | acp::PermissionOptionKind::AllowAlways => ToolCallStatus::InProgress,
                    _ => ToolCallStatus::InProgress,
                },
            };

        let curr_status = mem::replace(&mut call.status, new_status);

        if let ToolCallStatus::WaitingForConfirmation { respond_tx, .. } = curr_status {
            respond_tx.send(outcome).ok();
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
    }

    pub fn request_elicitation(
        &mut self,
        request: acp::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<Task<acp::CreateElicitationResponse>, acp::Error> {
        self.request_elicitation_with_id(request, cx)
            .map(|(_, task)| task)
    }

    pub fn request_elicitation_with_id(
        &mut self,
        request: acp::CreateElicitationRequest,
        cx: &mut Context<Self>,
    ) -> Result<(ElicitationEntryId, Task<acp::CreateElicitationResponse>), acp::Error> {
        ElicitationStore::validate_request(&request, cx)?;

        let (id, response_rx) = self.elicitations.insert_pending_elicitation(request);
        self.push_entry(AgentThreadEntry::Elicitation(id.clone()), cx);
        cx.emit(AcpThreadEvent::ElicitationRequested(id.clone()));

        let task =
            ElicitationStore::response_task(id.clone(), response_rx, cx, |_thread, cx, id| {
                cx.emit(AcpThreadEvent::ElicitationResponded(id))
            });

        Ok((id, task))
    }

    pub fn respond_to_elicitation(
        &mut self,
        id: &ElicitationEntryId,
        response: acp::CreateElicitationResponse,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.elicitation_entry_ix(id) else {
            return;
        };
        if !self.elicitations.respond_to_elicitation_by_id(id, response) {
            return;
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
    }

    pub fn complete_url_elicitation(
        &mut self,
        elicitation_id: &acp::ElicitationId,
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
        if !self.elicitations.cancel_elicitation_by_id(id, true) {
            return;
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
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

    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    pub fn update_plan(&mut self, request: acp::Plan, cx: &mut Context<Self>) {
        let new_entries_len = request.entries.len();
        let mut new_entries = request.entries.into_iter();

        // Reuse existing markdown to prevent flickering
        for (old, new) in self.plan.entries.iter_mut().zip(new_entries.by_ref()) {
            let PlanEntry {
                content,
                priority,
                status,
            } = old;
            content.update(cx, |old, cx| {
                old.replace(new.content, cx);
            });
            *priority = new.priority;
            *status = new.status;
        }
        for new in new_entries {
            self.plan.entries.push(PlanEntry::from_acp(new, cx))
        }
        self.plan.entries.truncate(new_entries_len);

        cx.notify();
    }

    pub fn snapshot_completed_plan(&mut self, cx: &mut Context<Self>) {
        if !self.plan.is_empty() && self.plan.stats().pending == 0 {
            let completed_entries = std::mem::take(&mut self.plan.entries);
            self.push_entry(AgentThreadEntry::CompletedPlan(completed_entries), cx);
        }
    }

    fn clear_completed_plan_entries(&mut self, cx: &mut Context<Self>) {
        self.plan
            .entries
            .retain(|entry| !matches!(entry.status, acp::PlanEntryStatus::Completed));
        cx.notify();
    }

    pub fn clear_plan(&mut self, cx: &mut Context<Self>) {
        self.plan.entries.clear();
        cx.notify();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn send_raw(
        &mut self,
        message: &str,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<Option<acp::PromptResponse>>> {
        self.send(vec![message.into()], cx)
    }

    pub fn send(
        &mut self,
        message: Vec<acp::ContentBlock>,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<Option<acp::PromptResponse>>> {
        self.send_inner(message, true, cx)
    }

    /// Sends a prompt without displaying a user-message bubble for it.
    /// This is used for native slash commands (e.g. `/compact`) that run a turn
    /// which produces its own thread entry (like the compaction summary). The
    /// typed command isn't sent to the model as an ordinary user turn.
    pub fn send_command(
        &mut self,
        message: Vec<acp::ContentBlock>,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<Option<acp::PromptResponse>>> {
        self.send_inner(message, false, cx)
    }

    fn send_inner(
        &mut self,
        message: Vec<acp::ContentBlock>,
        push_user_message: bool,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<Option<acp::PromptResponse>>> {
        let block = ContentBlock::new_combined(
            message.clone(),
            self.project.read(cx).languages().clone(),
            self.project.read(cx).path_style(cx),
            cx,
        );
        let request = acp::PromptRequest::new(self.session_id.clone(), message.clone());
        let git_store = self.project.read(cx).git_store().clone();

        let client_user_message_ids = self.connection.client_user_message_ids(cx);
        let client_id = client_user_message_ids
            .as_ref()
            .map(|client_user_message_ids| client_user_message_ids.new_id());

        self.run_turn(cx, async move |this, cx| {
            if push_user_message {
                this.update(cx, |this, cx| {
                    this.push_entry(
                        AgentThreadEntry::UserMessage(UserMessage {
                            protocol_id: None,
                            client_id: client_id.clone(),
                            is_optimistic: true,
                            content: block,
                            chunks: message,
                            checkpoint: None,
                            indented: false,
                        }),
                        cx,
                    );
                })
                .ok();

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
        self.connection.retry(&self.session_id, cx).is_some()
    }

    pub fn retry(
        &mut self,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<Option<acp::PromptResponse>>> {
        self.run_turn(cx, async move |this, cx| {
            this.update(cx, |this, cx| {
                this.connection
                    .retry(&this.session_id, cx)
                    .map(|retry| retry.run(cx))
            })?
            .context("retrying a session is not supported")?
            .await
        })
    }

    fn run_turn(
        &mut self,
        cx: &mut Context<Self>,
        f: impl 'static + AsyncFnOnce(WeakEntity<Self>, &mut AsyncApp) -> Result<acp::PromptResponse>,
    ) -> BoxFuture<'static, Result<Option<acp::PromptResponse>>> {
        self.clear_completed_plan_entries(cx);
        self.had_error = false;

        let (tx, rx) = oneshot::channel();
        let cancel_task = self.cancel(cx);

        self.turn_id += 1;
        let turn_id = self.turn_id;
        self.running_turn = Some(RunningTurn {
            id: turn_id,
            send_task: cx.spawn(async move |this, cx| {
                cancel_task.await;
                tx.send(f(this, cx).await).ok();
            }),
        });
        cx.emit(AcpThreadEvent::StatusChanged);

        cx.spawn(async move |this, cx| {
            let response = rx.await;

            this.update(cx, |this, cx| this.update_last_checkpoint(cx))?
                .await?;

            this.update(cx, |this, cx| {
                if this.parent_session_id.is_none() {
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
                }

                let Ok(response) = response else {
                    if is_same_turn {
                        cx.emit(AcpThreadEvent::StatusChanged);
                    }
                    // tx dropped, just return
                    return Ok(None);
                };

                match response {
                    Ok(r) => {
                        Self::flush_streaming_text(&mut this.streaming_text_buffer, cx);

                        if r.stop_reason == acp::StopReason::MaxTokens {
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

                        let canceled = matches!(r.stop_reason, acp::StopReason::Cancelled);
                        if canceled && is_same_turn {
                            this.cancel_pending_turn_entries(cx);
                        }

                        if !canceled {
                            this.snapshot_completed_plan(cx);
                        }

                        // Handle refusal - distinguish between user prompt and tool call refusals
                        if let acp::StopReason::Refusal = r.stop_reason {
                            this.had_error = true;
                            if let Some((user_msg_ix, _)) = this.last_user_message() {
                                // Check if there's a completed tool call with results after the last user message
                                // This indicates the refusal is in response to tool output, not the user's prompt
                                let has_completed_tool_call_after_user_msg =
                                    this.entries.iter().skip(user_msg_ix + 1).any(|entry| {
                                        if let AgentThreadEntry::ToolCall(tool_call) = entry {
                                            // Check if the tool call has completed and has output
                                            matches!(tool_call.status, ToolCallStatus::Completed)
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
                                        this.entries.truncate(user_msg_ix);
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
                        cx.emit(AcpThreadEvent::Stopped(r.stop_reason));
                        Ok(Some(r))
                    }
                    Err(e) => {
                        if is_same_turn {
                            cx.emit(AcpThreadEvent::StatusChanged);
                        }
                        Self::flush_streaming_text(&mut this.streaming_text_buffer, cx);
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
        })
        .boxed()
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) -> Task<()> {
        Self::flush_streaming_text(&mut self.streaming_text_buffer, cx);
        self.cancel_outstanding_elicitations(cx);

        let Some(turn) = self.running_turn.take() else {
            return Task::ready(());
        };
        self.mark_pending_entries_as_canceled(cx);
        self.connection.cancel(&self.session_id, cx);
        cx.emit(AcpThreadEvent::StatusChanged);

        // Wait for the send task to complete
        cx.background_spawn(turn.send_task)
    }

    fn cancel_pending_turn_entries(&mut self, cx: &mut Context<Self>) {
        self.mark_pending_entries_as_canceled(cx);
        self.cancel_outstanding_elicitations(cx);
    }

    fn mark_pending_entries_as_canceled(&mut self, cx: &mut Context<Self>) {
        for (ix, entry) in self.entries.iter_mut().enumerate() {
            match entry {
                AgentThreadEntry::ToolCall(call) => {
                    let cancel = matches!(
                        call.status,
                        ToolCallStatus::Pending
                            | ToolCallStatus::WaitingForConfirmation { .. }
                            | ToolCallStatus::InProgress
                    );
                    if cancel {
                        call.status = ToolCallStatus::Canceled;
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
    }

    fn cancel_outstanding_elicitations(&mut self, cx: &mut Context<Self>) {
        for ix in 0..self.entries.len() {
            let Some(AgentThreadEntry::Elicitation(elicitation_id)) = self.entries.get(ix) else {
                continue;
            };
            if self
                .elicitations
                .cancel_elicitation_by_id(elicitation_id, true)
            {
                cx.emit(AcpThreadEvent::EntryUpdated(ix));
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
        let Some(truncate) = self.connection.truncate(&self.session_id, cx) else {
            return Task::ready(Err(anyhow!("not supported")));
        };

        Self::flush_streaming_text(&mut self.streaming_text_buffer, cx);
        let telemetry = ActionLogTelemetry::from(&*self);
        cx.spawn(async move |this, cx| {
            cx.update(|cx| truncate.run(client_id.clone(), cx)).await?;
            this.update(cx, |this, cx| {
                if let Some((ix, _)) = this.user_message_mut(&client_id) {
                    // Collect all terminals from entries that will be removed
                    let terminals_to_remove: Vec<acp::TerminalId> = this.entries[ix..]
                        .iter()
                        .flat_map(|entry| entry.terminals())
                        .filter_map(|terminal| terminal.read(cx).id().clone().into())
                        .collect();

                    let range = ix..this.entries.len();
                    this.entries.truncate(ix);
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

    pub fn read_text_file(
        &self,
        path: PathBuf,
        line: Option<u32>,
        limit: Option<u32>,
        reuse_shared_snapshot: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<String, acp::Error>> {
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
                        acp::Error::resource_not_found(Some(path.display().to_string()))
                    })?;
                Ok::<_, acp::Error>(project.open_buffer(path, cx))
            })?;

            let buffer = load.await?;

            let snapshot = if reuse_shared_snapshot {
                this.read_with(cx, |this, _| {
                    this.shared_buffers.get(&buffer.clone()).cloned()
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
                    this.shared_buffers.insert(buffer.clone(), snapshot.clone());
                })?;
                snapshot
            };

            let max_point = snapshot.max_point();
            let start_position = Point::new(line, 0);

            if start_position > max_point {
                return Err(acp::Error::invalid_params().data(format!(
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
                    .get(&buffer)
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
        extra_env: Vec<acp::EnvVariable>,
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
            // Disables paging for `git` and hopefully other commands
            env.insert("PAGER".into(), "".into());
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

        let terminal_id = acp::TerminalId::new(Uuid::new_v4().to_string());
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
        terminal_id: acp::TerminalId,
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
        terminal_id: acp::TerminalId,
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

    pub fn terminal(&self, terminal_id: acp::TerminalId) -> Result<Entity<Terminal>> {
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
        terminal_id: acp::TerminalId,
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
                let entity = self.register_terminal_created(
                    terminal_id.clone(),
                    label,
                    cwd,
                    output_byte_limit,
                    terminal,
                    cx,
                );

                if let Some(mut chunks) = self.pending_terminal_output.remove(&terminal_id) {
                    for data in chunks.drain(..) {
                        entity.update(cx, |term, cx| {
                            term.inner().update(cx, |inner, cx| {
                                inner.write_output(&data, cx);
                            })
                        });
                    }
                }

                if let Some(_status) = self.pending_terminal_exit.remove(&terminal_id) {
                    entity.update(cx, |term, cx| {
                        term.inner().update(cx, |inner, _| inner.shrink_to_used());
                        cx.notify();
                    });
                }

                cx.notify();
            }
            TerminalProviderEvent::Output { terminal_id, data } => {
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |term, cx| {
                        term.inner().update(cx, |inner, cx| {
                            inner.write_output(&data, cx);
                        })
                    });
                } else {
                    self.pending_terminal_output
                        .entry(terminal_id)
                        .or_default()
                        .push(data);
                }
            }
            TerminalProviderEvent::TitleChanged { terminal_id, title } => {
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
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |term, cx| {
                        term.inner().update(cx, |inner, _| inner.shrink_to_used());
                        cx.notify();
                    });
                } else {
                    self.pending_terminal_exit.insert(terminal_id, status);
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
    match raw_output {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(value) => Some(cx.new(|cx| {
            Markdown::new(
                value.to_string().into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
        serde_json::Value::Number(value) => Some(cx.new(|cx| {
            Markdown::new(
                value.to_string().into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
        serde_json::Value::String(value) => Some(cx.new(|cx| {
            Markdown::new(
                value.clone().into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
        value => Some(cx.new(|cx| {
            let pretty_json = to_string_pretty(value).unwrap_or_else(|_| value.to_string());

            Markdown::new(
                format!("```json\n{}\n```", pretty_json).into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use feature_flags::FeatureFlag as _;
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
            acp::Meta::from_iter([(COMMAND_CATEGORY_META_KEY.into(), "future-category".into())]);
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

    fn enable_acp_beta(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.update_flags(false, vec![AcpBetaFeatureFlag::NAME.to_string()]);
        });
    }

    fn set_acp_beta_override(value: &str, cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |content| {
                    content
                        .feature_flags
                        .get_or_insert_default()
                        .insert(AcpBetaFeatureFlag::NAME.to_string(), value.to_string());
                });
            });
        });
    }

    #[test]
    fn text_resource_markdown_uses_mime_type_for_code_blocks() {
        let shell = acp::TextResourceContents::new("echo 'hello from exec test'", "tool://preview")
            .mime_type("text/x-shellscript".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&shell),
            "```sh\necho 'hello from exec test'\n```"
        );

        let markdown = acp::TextResourceContents::new("**approval** requested", "tool://preview")
            .mime_type("text/markdown".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&markdown),
            "**approval** requested"
        );

        let plain = acp::TextResourceContents::new("plain preview", "tool://preview")
            .mime_type("text/plain".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&plain),
            "```\nplain preview\n```"
        );

        let cpp = acp::TextResourceContents::new("int main() {}", "tool://preview")
            .mime_type("text/x-c++; charset=utf-8".to_string());
        assert_eq!(
            ContentBlock::text_resource_markdown(&cpp),
            "```cpp\nint main() {}\n```"
        );

        let untyped = acp::TextResourceContents::new("# plain preview", "tool://preview");
        assert_eq!(
            ContentBlock::text_resource_markdown(&untyped),
            "```\n# plain preview\n```"
        );
    }

    #[gpui::test]
    async fn test_tool_call_content_preserves_embedded_text_resource(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx);

        cx.update(|cx| {
            let language_registry =
                Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
            let content = acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                acp::EmbeddedResourceResource::TextResourceContents(
                    acp::TextResourceContents::new("echo 'hello from exec test'", "tool://preview")
                        .mime_type("text/x-shellscript".to_string()),
                ),
            ));

            let block = ContentBlock::new_tool_call_content(
                content,
                &language_registry,
                PathStyle::local(),
                cx,
            );

            let ContentBlock::EmbeddedResource { resource, markdown } = &block else {
                panic!("expected embedded resource block, got {block:?}");
            };
            match &resource.resource {
                acp::EmbeddedResourceResource::TextResourceContents(text) => {
                    assert_eq!(text.text, "echo 'hello from exec test'");
                    assert_eq!(text.uri, "tool://preview");
                    assert_eq!(text.mime_type.as_deref(), Some("text/x-shellscript"));
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

            let untyped = ContentBlock::new_tool_call_content(
                acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                    acp::EmbeddedResourceResource::TextResourceContents(
                        acp::TextResourceContents::new("# plain preview", "tool://preview"),
                    ),
                )),
                &language_registry,
                PathStyle::local(),
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
            let image_blob = acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                acp::EmbeddedResourceResource::BlobResourceContents(
                    acp::BlobResourceContents::new(
                        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
                        "tool://preview.png",
                    )
                    .mime_type("image/png".to_string()),
                ),
            ));

            let block = ContentBlock::new_tool_call_content(
                image_blob,
                &language_registry,
                PathStyle::local(),
                cx,
            );

            let ContentBlock::Image { image, dimensions } = &block else {
                panic!("expected image block, got {block:?}");
            };
            assert_eq!(image.format(), gpui::ImageFormat::Png);
            assert_eq!(
                dimensions.as_ref().map(|size| (size.width, size.height)),
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
            let archive_blob = acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                acp::EmbeddedResourceResource::BlobResourceContents(
                    acp::BlobResourceContents::new("not an image", "tool://archive.bin")
                        .mime_type("application/octet-stream".to_string()),
                ),
            ));

            let block = ContentBlock::new_tool_call_content(
                archive_blob,
                &language_registry,
                PathStyle::local(),
                cx,
            );

            let ContentBlock::EmbeddedResource { resource, markdown } = &block else {
                panic!("expected embedded resource block, got {block:?}");
            };
            assert!(markdown.is_none());
            match &resource.resource {
                acp::EmbeddedResourceResource::BlobResourceContents(blob) => {
                    assert_eq!(blob.uri, "tool://archive.bin");
                    assert_eq!(blob.mime_type.as_deref(), Some("application/octet-stream"));
                }
                other => panic!("expected blob resource contents, got {other:?}"),
            }
            assert_eq!(block.to_markdown(cx), "tool://archive.bin");
            assert_eq!(block.text_content(cx), None);

            let invalid_image_blob = acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                acp::EmbeddedResourceResource::BlobResourceContents(
                    acp::BlobResourceContents::new("not-base64", "tool://preview.png")
                        .mime_type("image/png".to_string()),
                ),
            ));
            let invalid = ContentBlock::new_tool_call_content(
                invalid_image_blob,
                &language_registry,
                PathStyle::local(),
                cx,
            );
            let ContentBlock::EmbeddedResource { resource, markdown } = &invalid else {
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

        let terminal_id = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());

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
            let term = thread.terminal(terminal_id.clone()).unwrap();
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

        let terminal_id = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());
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
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id.clone(),
                    status: acp::TerminalExitStatus::new().exit_code(0),
                },
                cx,
            );
        });

        let content = thread.read_with(cx, |thread, cx| {
            let term = thread.terminal(terminal_id.clone()).unwrap();
            term.read_with(cx, |term, cx| term.inner().read(cx).get_content())
        });

        assert!(
            content.contains("line 14999"),
            "expected output to remain visible after terminal exit, got: {content}"
        );
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

        let terminal_id = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());

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
                    status: acp::TerminalExitStatus::new().exit_code(0),
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
            let term = thread.terminal(terminal_id.clone()).unwrap();
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
        use std::collections::HashMap;
        use task::Shell;
        use util::shell_builder::ShellBuilder;

        init_test(cx);
        cx.executor().allow_parking();

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

        let terminal_id = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());

        // Create a real PTY terminal that runs a command which prints output then sleeps
        // We use printf instead of echo and chain with && sleep to ensure proper execution
        let (completion_tx, _completion_rx) = async_channel::unbounded();
        let (program, args) = ShellBuilder::new(&Shell::System, false).build(
            Some("printf 'output_before_kill\\n' && sleep 60".to_owned()),
            &[],
        );

        let builder = cx
            .update(|cx| {
                ::terminal::TerminalBuilder::new(
                    None,
                    None,
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
                    0,
                    false,
                    0,
                    Some(completion_tx),
                    cx,
                    vec![],
                    PathStyle::local(),
                )
            })
            .await
            .unwrap();

        let lower_terminal = cx.new(|cx| builder.subscribe(cx));

        // Create the acp_thread Terminal wrapper
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "printf output_before_kill && sleep 60".to_string(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower_terminal.clone(),
                },
                cx,
            );
        });

        // Poll until the printf command produces output, rather than using a
        // fixed sleep which is flaky on loaded machines.
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

        // Get the acp_thread Terminal and kill it
        let wait_for_exit = thread.update(cx, |thread, cx| {
            let term = thread.terminals.get(&terminal_id).unwrap();
            let wait_for_exit = term.read(cx).wait_for_exit();
            term.update(cx, |term, cx| {
                term.kill(cx);
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

        // Give the system a chance to process any pending updates
        cx.run_until_parked();

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
                assert_eq!(user_msg.protocol_id, None);
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
                assert_eq!(user_msg.protocol_id, None);
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
                assert_eq!(user_msg.protocol_id, None);
                assert_eq!(user_msg.client_id, Some(message_2_id));
                assert_eq!(user_msg.content.to_markdown(cx), "New user message");
            } else {
                panic!("Expected UserMessage at index 2");
            }
        });
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
                    acp::SessionUpdate::UserMessageChunk(
                        acp::ContentChunk::new("First ".into()).message_id("msg_user_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::UserMessageChunk(
                        acp::ContentChunk::new("message".into()).message_id("msg_user_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::UserMessageChunk(
                        acp::ContentChunk::new("Second message".into()).message_id("msg_user_2"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::UserMessageChunk(
                        acp::ContentChunk::new("Echo".into()).message_id("msg_user_3"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::UserMessageChunk(
                        acp::ContentChunk::new("Echo".into()).message_id("msg_user_3"),
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
                first_message
                    .protocol_id
                    .as_ref()
                    .map(ToString::to_string)
                    .as_deref(),
                Some("msg_user_1")
            );

            let AgentThreadEntry::UserMessage(second_message) = &thread.entries[1] else {
                panic!("expected second entry to be a user message")
            };
            assert_eq!(second_message.content.to_markdown(cx), "Second message");
            assert_eq!(
                second_message
                    .protocol_id
                    .as_ref()
                    .map(ToString::to_string)
                    .as_deref(),
                Some("msg_user_2")
            );

            let AgentThreadEntry::UserMessage(third_message) = &thread.entries[2] else {
                panic!("expected third entry to be a user message")
            };
            assert_eq!(third_message.content.to_markdown(cx), "EchoEcho");
            assert_eq!(
                third_message
                    .protocol_id
                    .as_ref()
                    .map(ToString::to_string)
                    .as_deref(),
                Some("msg_user_3")
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
                    acp::SessionUpdate::UserMessageChunk(
                        acp::ContentChunk::new("Agent user chunk".into())
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
            assert!(optimistic_message.protocol_id.is_none());
            assert!(optimistic_message.client_id.is_none());

            let AgentThreadEntry::UserMessage(agent_message) = &thread.entries[1] else {
                panic!("expected second entry to be protocol user chunk")
            };
            assert!(!agent_message.is_optimistic);
            assert_eq!(agent_message.content.to_markdown(cx), "Agent user chunk");
            assert_eq!(
                agent_message
                    .protocol_id
                    .as_ref()
                    .map(ToString::to_string)
                    .as_deref(),
                Some("agent_user_chunk")
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
                    acp::SessionUpdate::AgentThoughtChunk(
                        acp::ContentChunk::new("Thinking ".into()).message_id("msg_thought_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::AgentThoughtChunk(
                        acp::ContentChunk::new("hard".into()).message_id("msg_thought_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::AgentThoughtChunk(
                        acp::ContentChunk::new("A separate thought".into())
                            .message_id("msg_thought_2"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::AgentMessageChunk(
                        acp::ContentChunk::new("Answer ".into()).message_id("msg_agent_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::AgentMessageChunk(
                        acp::ContentChunk::new("done".into()).message_id("msg_agent_1"),
                    ),
                    cx,
                )
                .unwrap();
            thread
                .handle_session_update(
                    acp::SessionUpdate::AgentMessageChunk(
                        acp::ContentChunk::new("Follow-up".into()).message_id("msg_agent_2"),
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

            let AssistantMessageChunk::Thought { id, block } = &message.chunks[0] else {
                panic!("expected first chunk to be a thought")
            };
            assert_eq!(block.to_markdown(cx), "Thinking hard");
            assert_eq!(
                id.as_ref().map(ToString::to_string).as_deref(),
                Some("msg_thought_1")
            );

            let AssistantMessageChunk::Thought { id, block } = &message.chunks[1] else {
                panic!("expected second chunk to be a thought")
            };
            assert_eq!(block.to_markdown(cx), "A separate thought");
            assert_eq!(
                id.as_ref().map(ToString::to_string).as_deref(),
                Some("msg_thought_2")
            );

            let AssistantMessageChunk::Message { id, block } = &message.chunks[2] else {
                panic!("expected third chunk to be a message")
            };
            assert_eq!(block.to_markdown(cx), "Answer done");
            assert_eq!(
                id.as_ref().map(ToString::to_string).as_deref(),
                Some("msg_agent_1")
            );

            let AssistantMessageChunk::Message { id, block } = &message.chunks[3] else {
                panic!("expected fourth chunk to be a message")
            };
            assert_eq!(block.to_markdown(cx), "Follow-up");
            assert_eq!(
                id.as_ref().map(ToString::to_string).as_deref(),
                Some("msg_agent_2")
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
                                acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk::new(
                                    "Thinking ".into(),
                                )),
                                cx,
                            )
                            .unwrap();
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk::new(
                                    "hard!".into(),
                                )),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
                    if let Some(acp::ContentBlock::Text(text)) = request.prompt.first() {
                        *received_prompt.borrow_mut() = Some(text.text.clone());
                    }
                    // Simulate a native command producing its own thread entry
                    // (here a compaction) rather than echoing a user message.
                    thread.update(&mut cx, |thread, cx| {
                        thread.push_context_compaction(
                            ContextCompaction {
                                id: ContextCompactionId("c1".into()),
                                status: ContextCompactionStatus::Completed,
                                summary: None,
                            },
                            cx,
                        );
                    })?;
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
                        let prompt = request.prompt.first().cloned().unwrap_or_else(|| "".into());

                        thread.update(&mut cx, |thread, cx| {
                            thread
                                .handle_session_update(
                                    acp::SessionUpdate::UserMessageChunk(acp::ContentChunk::new(
                                        prompt,
                                    )),
                                    cx,
                                )
                                .unwrap();
                        })?;

                        Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
            assert_eq!(message.protocol_id, None);
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
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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

        assert_eq!(err.code, acp::ErrorCode::ResourceNotFound);
    }

    #[gpui::test]
    async fn test_succeeding_canceled_toolcall(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let id = acp::ToolCallId::new("test");

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let id = id.clone();
            move |_, thread, mut cx| {
                let id = id.clone();
                async move {
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.handle_session_update(
                                acp::SessionUpdate::ToolCall(
                                    acp::ToolCall::new(id.clone(), "Label")
                                        .kind(acp::ToolKind::Fetch)
                                        .status(acp::ToolCallStatus::InProgress),
                                ),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
                thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::InProgress,
                    ..
                })
            ));
        });

        thread.update(cx, |thread, cx| thread.cancel(cx)).await;

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                &thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Canceled,
                    ..
                })
            ));
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                        id,
                        acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::Completed),
                    )),
                    cx,
                )
            })
            .unwrap();

        request.await.unwrap();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Completed,
                    ..
                })
            ));
        });
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
                    acp::SessionUpdate::ToolCall(
                        acp::ToolCall::new("write_file", "Write SKILL.md")
                            .kind(acp::ToolKind::Edit)
                            .status(acp::ToolCallStatus::Completed)
                            .locations(vec![acp::ToolCallLocation::new(skill_path.clone())]),
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
                .expect("resolved location should keep an open buffer");
            assert_eq!(buffer.read(cx).text(), "skill body");
        });
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

        let tool_call_id = acp::ToolCallId::new("toolu_01duplicate");
        let allow_option_id = acp::PermissionOptionId::new("allow");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp::ToolCall::new(tool_call_id.clone(), "Original title")
                        .kind(acp::ToolKind::Execute)
                        .status(acp::ToolCallStatus::Pending)
                        .content(vec!["original content".into()])
                        .into(),
                    PermissionOptions::Flat(vec![acp::PermissionOption::new(
                        allow_option_id.clone(),
                        "Allow",
                        acp::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCall(
                        acp::ToolCall::new(tool_call_id.clone(), "Updated title")
                            .kind(acp::ToolKind::Execute)
                            .status(acp::ToolCallStatus::Pending)
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
                tool_call.status,
                ToolCallStatus::WaitingForConfirmation { .. }
            ));
            assert_eq!(tool_call.content.len(), 1);
            assert_eq!(tool_call.content[0].to_markdown(cx), "updated content");
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                        tool_call_id.clone(),
                        acp::ToolCallUpdateFields::new()
                            .status(acp::ToolCallStatus::InProgress)
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
                tool_call.status,
                ToolCallStatus::WaitingForConfirmation { .. }
            ));
            assert_eq!(tool_call.content.len(), 1);
            assert_eq!(tool_call.content[0].to_markdown(cx), "updated again");
        });

        let selected_outcome = SelectedPermissionOutcome::new(
            allow_option_id.clone(),
            acp::PermissionOptionKind::AllowOnce,
        );
        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id.clone(), selected_outcome, cx);
        });

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(tool_call.status, ToolCallStatus::InProgress));
        });

        match permission_task.await {
            RequestPermissionOutcome::Selected(outcome) => {
                assert_eq!(outcome.option_id, allow_option_id);
                assert_eq!(outcome.option_kind, acp::PermissionOptionKind::AllowOnce);
            }
            RequestPermissionOutcome::Cancelled => {
                panic!("permission request should remain open after duplicate tool call update")
            }
        }

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                        tool_call_id.clone(),
                        acp::ToolCallUpdateFields::new()
                            .status(acp::ToolCallStatus::Completed)
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
            assert!(matches!(tool_call.status, ToolCallStatus::Completed));
            assert_eq!(tool_call.content.len(), 1);
            assert_eq!(tool_call.content[0].to_markdown(cx), "done");
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

        let tool_call_id = acp::ToolCallId::new("toolu_01auto_resolve");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp::ToolCall::new(tool_call_id.clone(), "Original title")
                        .kind(acp::ToolKind::Execute)
                        .status(acp::ToolCallStatus::Pending)
                        .into(),
                    PermissionOptions::Flat(vec![acp::PermissionOption::new(
                        acp::PermissionOptionId::new("allow"),
                        "Allow",
                        acp::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                        tool_call_id.clone(),
                        acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::InProgress),
                    )),
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(
                tool_call.status,
                ToolCallStatus::WaitingForConfirmation {
                    current_status: acp::ToolCallStatus::InProgress,
                    ..
                }
            ));
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_call_id.clone(),
                SelectedPermissionOutcome::new(
                    acp::PermissionOptionId::new("allow"),
                    acp::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(tool_call.status, ToolCallStatus::InProgress));
        });

        match permission_task.await {
            RequestPermissionOutcome::Selected(outcome) => {
                assert_eq!(outcome.option_id, acp::PermissionOptionId::new("allow"));
                assert_eq!(outcome.option_kind, acp::PermissionOptionKind::AllowOnce);
            }
            RequestPermissionOutcome::Cancelled => {
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

        let tool_call_id = acp::ToolCallId::new("toolu_01existing_permission");
        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCall(
                        acp::ToolCall::new(tool_call_id.clone(), "Running title")
                            .kind(acp::ToolKind::Execute)
                            .status(acp::ToolCallStatus::InProgress),
                    ),
                    cx,
                )
            })
            .unwrap();

        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp::ToolCall::new(tool_call_id.clone(), "Needs permission")
                        .kind(acp::ToolKind::Execute)
                        .status(acp::ToolCallStatus::Pending)
                        .into(),
                    PermissionOptions::Flat(vec![acp::PermissionOption::new(
                        acp::PermissionOptionId::new("allow"),
                        "Allow",
                        acp::PermissionOptionKind::AllowOnce,
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
            assert!(matches!(
                tool_call.status,
                ToolCallStatus::WaitingForConfirmation {
                    current_status: acp::ToolCallStatus::InProgress,
                    ..
                }
            ));
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(
                tool_call_id.clone(),
                SelectedPermissionOutcome::new(
                    acp::PermissionOptionId::new("allow"),
                    acp::PermissionOptionKind::AllowOnce,
                ),
                cx,
            );
        });

        match permission_task.await {
            RequestPermissionOutcome::Selected(outcome) => {
                assert_eq!(outcome.option_id, acp::PermissionOptionId::new("allow"));
                assert_eq!(outcome.option_kind, acp::PermissionOptionKind::AllowOnce);
            }
            RequestPermissionOutcome::Cancelled => {
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

        let tool_call_id = acp::ToolCallId::new("toolu_01cancelled_permission");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp::ToolCall::new(tool_call_id.clone(), "Needs permission")
                        .kind(acp::ToolKind::Execute)
                        .status(acp::ToolCallStatus::Pending)
                        .into(),
                    PermissionOptions::Flat(vec![acp::PermissionOption::new(
                        acp::PermissionOptionId::new("allow"),
                        "Allow",
                        acp::PermissionOptionKind::AllowOnce,
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
            assert!(matches!(tool_call.status, ToolCallStatus::Canceled));
        });

        match permission_task.await {
            RequestPermissionOutcome::Cancelled => {}
            RequestPermissionOutcome::Selected(_) => {
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

        let tool_call_id = acp::ToolCallId::new("toolu_01completed_while_waiting");
        let permission_task = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    acp::ToolCall::new(tool_call_id.clone(), "Needs permission")
                        .kind(acp::ToolKind::Execute)
                        .status(acp::ToolCallStatus::Pending)
                        .into(),
                    PermissionOptions::Flat(vec![acp::PermissionOption::new(
                        acp::PermissionOptionId::new("allow"),
                        "Allow",
                        acp::PermissionOptionKind::AllowOnce,
                    )]),
                    AuthorizationKind::PermissionGrant,
                    cx,
                )
            })
            .unwrap();

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                        tool_call_id.clone(),
                        acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::Completed),
                    )),
                    cx,
                )
            })
            .unwrap();

        thread.read_with(cx, |thread, _cx| {
            let (_, tool_call) = thread
                .tool_call(&tool_call_id)
                .expect("tool call should exist");
            assert!(matches!(tool_call.status, ToolCallStatus::Completed));
        });

        match permission_task.await {
            RequestPermissionOutcome::Cancelled => {}
            RequestPermissionOutcome::Selected(_) => {
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
                                acp::SessionUpdate::ToolCall(
                                    acp::ToolCall::new("test", "Label")
                                        .kind(acp::ToolKind::Edit)
                                        .status(acp::ToolCallStatus::Completed)
                                        .content(vec![acp::ToolCallContent::Diff(acp::Diff::new(
                                            "/test/test.txt",
                                            "foo",
                                        ))]),
                                ),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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

                    let acp::ContentBlock::Text(content) = &request.prompt[0] else {
                        panic!("expected text content block");
                    };
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                                    content.text.to_uppercase().into(),
                                )),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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

                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
                                    acp::SessionUpdate::ToolCall(
                                        acp::ToolCall::new("tool1", "Test Tool")
                                            .kind(acp::ToolKind::Fetch)
                                            .status(acp::ToolCallStatus::Completed)
                                            .raw_input(serde_json::json!({"query": "test"}))
                                            .raw_output(serde_json::json!({"result": "inappropriate content"})),
                                    ),
                                    cx,
                                )
                                .unwrap();
                        })?;

                        // Now return refusal because of the tool result
                        Ok(acp::PromptResponse::new(acp::StopReason::Refusal))
                    } else {
                        Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
                    async move { Ok(acp::PromptResponse::new(acp::StopReason::Refusal)) }
                        .boxed_local()
                } else {
                    async move { Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)) }
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
                        return Ok(acp::PromptResponse::new(acp::StopReason::Refusal));
                    }

                    let acp::ContentBlock::Text(content) = &request.prompt[0] else {
                        panic!("expected text content block");
                    };
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                                    content.text.to_uppercase().into(),
                                )),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
    async fn test_elicitation_requires_acp_beta_flag(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec![]);
        });
        set_acp_beta_override("off", cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let result = thread.update(cx, |thread, cx| {
            thread.request_elicitation(
                acp::CreateElicitationRequest::new(
                    acp::ElicitationFormMode::new(
                        acp::ElicitationSessionScope::new(session_id),
                        acp::ElicitationSchema::new().string("name", true),
                    ),
                    "Provide a name",
                ),
                cx,
            )
        });

        assert!(result.is_err());
        thread.read_with(cx, |thread, _| assert!(thread.entries().is_empty()));
    }

    #[gpui::test]
    async fn test_form_elicitation_accepts_response(cx: &mut TestAppContext) {
        init_test(cx);
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let tool_call_id = acp::ToolCallId::new("tool-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationSessionScope::new(session_id.clone())
                                .tool_call_id(tool_call_id.clone()),
                            acp::ElicitationSchema::new().string("name", true),
                        ),
                        "Provide a name",
                    ),
                    cx,
                )
                .unwrap()
        });

        let elicitation_id = thread.read_with(cx, |thread, _| {
            let (elicitation_id, elicitation) = only_thread_elicitation(thread);
            let acp::ElicitationScope::Session(scope) = elicitation.request.scope() else {
                panic!("expected session-scoped elicitation");
            };
            assert_eq!(scope.tool_call_id.as_ref(), Some(&tool_call_id));
            elicitation_id
        });

        let expected_content = std::collections::BTreeMap::from([(
            "name".to_string(),
            acp::ElicitationContentValue::from("Ada"),
        )]);
        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &elicitation_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new().content(expected_content.clone()),
                )),
                cx,
            );
        });

        let response = response_task.await;
        assert_eq!(
            response.action,
            acp::ElicitationAction::Accept(
                acp::ElicitationAcceptAction::new().content(expected_content)
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
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp::ElicitationId::new("url-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationUrlMode::new(
                            acp::ElicitationSessionScope::new(session_id),
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
        assert!(matches!(
            response_task.await.action,
            acp::ElicitationAction::Accept(_)
        ));
        thread.update(cx, |thread, cx| {
            thread.respond_to_elicitation(
                &entry_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
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
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp::ElicitationId::new("url-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationUrlMode::new(
                            acp::ElicitationSessionScope::new(session_id),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp::ElicitationAction::Accept(_)
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
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp::ElicitationId::new("url-1");

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationUrlMode::new(
                            acp::ElicitationSessionScope::new(session_id),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp::ElicitationAction::Accept(_)
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
        enable_acp_beta(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let prompt_count = Rc::new(RefCell::new(0usize));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let prompt_count = prompt_count.clone();
            move |_request, _thread, _cx| {
                let stop_reason = {
                    let mut prompt_count = prompt_count.borrow_mut();
                    let stop_reason = if *prompt_count == 0 {
                        acp::StopReason::EndTurn
                    } else {
                        acp::StopReason::Cancelled
                    };
                    *prompt_count += 1;
                    stop_reason
                };

                async move { Ok(acp::PromptResponse::new(stop_reason)) }.boxed_local()
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
        assert_eq!(response.stop_reason, acp::StopReason::EndTurn);

        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());
        let url_elicitation_id = acp::ElicitationId::new("url-1");
        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationUrlMode::new(
                            acp::ElicitationSessionScope::new(session_id),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp::ElicitationAction::Accept(_)
        ));

        let response = thread
            .update(cx, |thread, cx| thread.send(vec!["second turn".into()], cx))
            .await
            .expect("second turn should succeed")
            .expect("second turn should return a response");
        assert_eq!(response.stop_reason, acp::StopReason::Cancelled);
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
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                            acp::ElicitationSchema::new().string("name", true),
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
            let acp::ElicitationScope::Request(scope) = elicitation.request.scope() else {
                panic!("expected request-scoped elicitation");
            };
            assert_eq!(scope.request_id, acp::RequestId::Number(1));
            elicitation.id.clone()
        });

        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &elicitation_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
        });

        assert_eq!(response_task.await.action, acp::ElicitationAction::Decline);
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Declined));
        });
    }

    #[gpui::test]
    async fn test_request_elicitation_store_ignores_duplicate_response(cx: &mut TestAppContext) {
        init_test(cx);
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                            acp::ElicitationSchema::new().string("name", true),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
            store.respond_to_elicitation(
                &elicitation_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });

        assert_eq!(response_task.await.action, acp::ElicitationAction::Decline);
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
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let (elicitation_id, response_task) = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation_with_id(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationSessionScope::new(session_id),
                            acp::ElicitationSchema::new().string("name", true),
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

        assert_eq!(response_task.await.action, acp::ElicitationAction::Cancel);
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
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationSessionScope::new(session_id),
                            acp::ElicitationSchema::new().string("name", true),
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

        assert_eq!(response_task.await.action, acp::ElicitationAction::Cancel);
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    fn request_test_session_elicitation(
        thread: WeakEntity<AcpThread>,
        session_id: acp::SessionId,
        cx: &mut AsyncApp,
    ) -> Result<Task<acp::CreateElicitationResponse>> {
        thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationSessionScope::new(session_id),
                            acp::ElicitationSchema::new().string("name", true),
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
        enable_acp_beta(cx);
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
            Some(acp::ElicitationAction::Cancel)
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
        enable_acp_beta(cx);
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

                    Ok(acp::PromptResponse::new(acp::StopReason::MaxTokens))
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
            Some(acp::ElicitationAction::Cancel)
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
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let (elicitation_id, response_task) = store.update(cx, |store, cx| {
            store
                .request_elicitation_with_id(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                            acp::ElicitationSchema::new().string("name", true),
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

        assert_eq!(response_task.await.action, acp::ElicitationAction::Cancel);
        store.read_with(cx, |store, _| {
            let Some((_, elicitation)) = store.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Canceled));
        });
    }

    #[gpui::test]
    async fn test_request_elicitation_store_cancel_all_resolves_cancel(cx: &mut TestAppContext) {
        init_test(cx);
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                            acp::ElicitationSchema::new().string("name", true),
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

        assert_eq!(response_task.await.action, acp::ElicitationAction::Cancel);
    }

    #[gpui::test]
    async fn test_request_elicitation_store_clear_removes_answered_and_cancels_pending(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));

        let first_response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                            acp::ElicitationSchema::new().string("name", true),
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
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(2)),
                            acp::ElicitationSchema::new().string("account", true),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
            store.clear(cx);
        });

        assert_eq!(
            first_response_task.await.action,
            acp::ElicitationAction::Decline
        );
        assert_eq!(
            second_response_task.await.action,
            acp::ElicitationAction::Cancel
        );
        store.read_with(cx, |store, _| assert!(store.elicitations().is_empty()));
    }

    #[gpui::test]
    async fn test_request_elicitation_store_clear_resolved_preserves_outstanding(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let url_elicitation_id = acp::ElicitationId::new("url-1");

        let accepted_response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                            acp::ElicitationSchema::new().string("name", true),
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
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(2)),
                            acp::ElicitationSchema::new().string("account", true),
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
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationUrlMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(3)),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
            store.respond_to_elicitation(
                &accepted_url_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            accepted_response_task.await.action,
            acp::ElicitationAction::Accept(_)
        ));
        assert!(matches!(
            accepted_url_response_task.await.action,
            acp::ElicitationAction::Accept(_)
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
            acp::ElicitationAction::Cancel
        );
    }

    #[gpui::test]
    async fn test_request_url_elicitation_store_can_be_completed(cx: &mut TestAppContext) {
        init_test(cx);
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let url_elicitation_id = acp::ElicitationId::new("url-1");

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationUrlMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
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

        assert!(matches!(
            response_task.await.action,
            acp::ElicitationAction::Accept(_)
        ));
        store.update(cx, |store, cx| {
            store.respond_to_elicitation(
                &entry_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
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
        enable_acp_beta(cx);
        let store = cx.update(|cx| cx.new(|_| ElicitationStore::default()));
        let url_elicitation_id = acp::ElicitationId::new("url-1");

        let response_task = store.update(cx, |store, cx| {
            store
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationUrlMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });
        assert!(matches!(
            response_task.await.action,
            acp::ElicitationAction::Accept(_)
        ));
        store.update(cx, |store, cx| {
            store.cancel_all(cx);
        });
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
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationSessionScope::new(session_id),
                            acp::ElicitationSchema::new().string("name", true),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
            thread.cancel(cx).detach();
        });

        assert_eq!(response_task.await.action, acp::ElicitationAction::Decline);
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
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let response_task = thread.update(cx, |thread, cx| {
            thread
                .request_elicitation(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationSessionScope::new(session_id),
                            acp::ElicitationSchema::new().string("name", true),
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
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Decline),
                cx,
            );
            thread.respond_to_elicitation(
                &elicitation_id,
                acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
                    acp::ElicitationAcceptAction::new(),
                )),
                cx,
            );
        });

        assert_eq!(response_task.await.action, acp::ElicitationAction::Decline);
        thread.read_with(cx, |thread, _| {
            let Some((_, elicitation)) = thread.elicitation(&elicitation_id) else {
                panic!("missing elicitation entry");
            };
            assert!(matches!(elicitation.status, ElicitationStatus::Declined));
        });
    }

    #[gpui::test]
    async fn test_url_elicitation_rejects_invalid_url(cx: &mut TestAppContext) {
        init_test(cx);
        enable_acp_beta(cx);
        let thread = new_test_thread(cx).await;
        let session_id = thread.read_with(cx, |thread, _| thread.session_id().clone());

        let result = thread.update(cx, |thread, cx| {
            thread.request_elicitation(
                acp::CreateElicitationRequest::new(
                    acp::ElicitationUrlMode::new(
                        acp::ElicitationSessionScope::new(session_id),
                        "url-1",
                        "not a url",
                    ),
                    "Complete this in the browser",
                ),
                cx,
            )
        });

        assert!(result.is_err());
        thread.read_with(cx, |thread, _| assert!(thread.entries().is_empty()));
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
        auth_methods: Vec<acp::AuthMethod>,
        supports_truncate: bool,
        sessions: Arc<parking_lot::Mutex<HashMap<acp::SessionId, WeakEntity<AcpThread>>>>,
        set_title_calls: Rc<RefCell<Vec<SharedString>>>,
        on_user_message: Option<
            Rc<
                dyn Fn(
                        acp::PromptRequest,
                        WeakEntity<AcpThread>,
                        AsyncApp,
                    ) -> LocalBoxFuture<'static, Result<acp::PromptResponse>>
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
        fn with_auth_methods(mut self, auth_methods: Vec<acp::AuthMethod>) -> Self {
            self.auth_methods = auth_methods;
            self
        }

        fn on_user_message(
            mut self,
            handler: impl Fn(
                acp::PromptRequest,
                WeakEntity<AcpThread>,
                AsyncApp,
            ) -> LocalBoxFuture<'static, Result<acp::PromptResponse>>
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

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &self.auth_methods
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let session_id = acp::SessionId::new(
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
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            });
            self.sessions.lock().insert(session_id, thread.downgrade());
            Task::ready(Ok(thread))
        }

        fn authenticate(&self, method: acp::AuthMethodId, _cx: &mut App) -> Task<gpui::Result<()>> {
            if self.auth_methods().iter().any(|m| m.id() == &method) {
                Task::ready(Ok(()))
            } else {
                Task::ready(Err(anyhow!("Invalid Auth Method")))
            }
        }

        fn prompt(
            &self,
            params: acp::PromptRequest,
            cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            let sessions = self.sessions.lock();
            let thread = sessions.get(&params.session_id).unwrap();
            if let Some(handler) = &self.on_user_message {
                let handler = handler.clone();
                let thread = thread.clone();
                cx.spawn(async move |cx| handler(params, thread, cx.clone()).await)
            } else {
                Task::ready(Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)))
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

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {}

        fn truncate(
            &self,
            session_id: &acp::SessionId,
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
            _session_id: &acp::SessionId,
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
        _session_id: acp::SessionId,
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
            params: acp::PromptRequest,
            cx: &mut App,
        ) -> Task<Result<acp::PromptResponse>> {
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
        let nonexistent_id = acp::ToolCallId::new("nonexistent-tool-call");
        thread.update(cx, |thread, cx| {
            let result = thread.handle_session_update(
                acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                    nonexistent_id.clone(),
                    acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::Completed),
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
                assert!(matches!(tool_call.status, ToolCallStatus::Failed));
                assert_eq!(tool_call.kind, acp::ToolKind::Fetch);

                // Check that the content contains the error message
                assert_eq!(tool_call.content.len(), 1);
                if let ToolCallContent::ContentBlock(content_block) = &tool_call.content[0] {
                    match content_block {
                        ContentBlock::Markdown { markdown } => {
                            let markdown_text = markdown.read(cx).source();
                            assert!(markdown_text.contains("Tool call not found"));
                        }
                        ContentBlock::Empty => panic!("Expected markdown content, got empty"),
                        ContentBlock::ResourceLink { .. } => {
                            panic!("Expected markdown content, got resource link")
                        }
                        ContentBlock::EmbeddedResource { .. } => {
                            panic!("Expected markdown content, got embedded resource")
                        }
                        ContentBlock::Image { .. } => {
                            panic!("Expected markdown content, got image")
                        }
                    }
                } else {
                    panic!("Expected ContentBlock, got: {:?}", tool_call.content[0]);
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
        let terminal_id_1 = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());
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
                    status: acp::TerminalExitStatus::new().exit_code(0),
                },
                cx,
            );
        });

        let terminal_id_2 = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());
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
                    status: acp::TerminalExitStatus::new().exit_code(0),
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
        let terminal_id = acp::TerminalId::new(uuid::Uuid::new_v4().to_string());
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
                    acp::SessionUpdate::ToolCall(
                        acp::ToolCall::new("terminal-tool-1", "Running command")
                            .kind(acp::ToolKind::Execute)
                            .status(acp::ToolCallStatus::InProgress)
                            .content(vec![acp::ToolCallContent::Terminal(acp::Terminal::new(
                                terminal_id.clone(),
                            ))])
                            .raw_input(serde_json::json!({"command": "sleep 1000", "cd": "/test"})),
                    ),
                    cx,
                )
                .unwrap();
        });

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
                async move { Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)) }.boxed_local()
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
            thread.push_entry(
                AgentThreadEntry::UserMessage(UserMessage {
                    protocol_id: None,
                    client_id: Some(ClientUserMessageId::new()),
                    is_optimistic: true,
                    content: ContentBlock::Empty,
                    chunks: vec!["Injected message (no checkpoint)".into()],
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
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            move |params, _thread, _cx| {
                let first_complete_rx = first_complete_rx.borrow_mut().take();
                let is_first = params
                    .prompt
                    .iter()
                    .any(|c| matches!(c, acp::ContentBlock::Text(t) if t.text.contains("first")));

                async move {
                    if is_first {
                        // First handler waits until signaled
                        if let Some(rx) = first_complete_rx {
                            rx.await.ok();
                        }
                    }
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
        assert_eq!(thread.read_with(cx, |t, _| t.turn_id), 1);

        // Send second message (turn_id=2) while first is still blocked
        // This calls cancel() which takes turn 1's running_turn and sets turn 2's
        let second_request = thread.update(cx, |thread, cx| thread.send_raw("second", cx));
        assert_eq!(thread.read_with(cx, |t, _| t.turn_id), 2);

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

        // Second request completes - SHOULD clear running_turn
        second_request.await.unwrap();

        let running_turn_after_second =
            thread.read_with(cx, |thread, _| thread.running_turn.is_some());
        assert!(
            !running_turn_after_second,
            "second turn completing should clear running_turn"
        );
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
                    matches!(content, acp::ContentBlock::Text(text) if text.text.contains("first"))
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
                                    summary: None,
                                },
                                cx,
                            );
                        })?;

                        Ok(acp::PromptResponse::new(acp::StopReason::Cancelled))
                    } else {
                        Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
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
        assert_eq!(response.stop_reason, acp::StopReason::Cancelled);

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
            assert_eq!(message.protocol_id, None);
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
                                acp::SessionUpdate::ToolCall(
                                    acp::ToolCall::new(
                                        acp::ToolCallId::new("test-tool"),
                                        "Test Tool",
                                    )
                                    .kind(acp::ToolKind::Fetch)
                                    .status(acp::ToolCallStatus::InProgress),
                                ),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();

                    Ok(acp::PromptResponse::new(acp::StopReason::Cancelled))
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
            acp::StopReason::Cancelled,
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
                matches!(tool_entry.status, ToolCallStatus::Canceled),
                "tool should be marked as Canceled when response is Cancelled, got {:?}",
                tool_entry.status
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
                acp::SessionUpdate::SessionInfoUpdate(
                    acp::SessionInfoUpdate::new().title("Helping with Rust question"),
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
        assert!(
            connection.set_title_calls.borrow().is_empty(),
            "session info title update should not propagate back to the connection"
        );
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
                    acp::SessionUpdate::UsageUpdate(
                        acp::UsageUpdate::new(5000, 10000).cost(acp::Cost::new(0.42, "USD")),
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
                    acp::SessionUpdate::UsageUpdate(
                        acp::UsageUpdate::new(5000, 10000).cost(acp::Cost::new(0.42, "USD")),
                    ),
                    cx,
                )
                .unwrap();

            thread.push_context_compaction(
                ContextCompaction {
                    id: ContextCompactionId("compaction-1".into()),
                    status: ContextCompactionStatus::InProgress,
                    summary: None,
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
                    acp::SessionUpdate::UsageUpdate(acp::UsageUpdate::new(1000, 10000)),
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
                    acp::SessionUpdate::UsageUpdate(
                        acp::UsageUpdate::new(1000, 10000).cost(acp::Cost::new(0.10, "USD")),
                    ),
                    cx,
                )
                .unwrap();

            thread
                .handle_session_update(
                    acp::SessionUpdate::UsageUpdate(acp::UsageUpdate::new(2000, 10000)),
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
                                acp::SessionUpdate::UsageUpdate(
                                    acp::UsageUpdate::new(3000, 10000)
                                        .cost(acp::Cost::new(0.05, "EUR")),
                                ),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)
                        .usage(acp::Usage::new(500, 200, 300)))
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
                    acp::SessionUpdate::UsageUpdate(
                        acp::UsageUpdate::new(1000, 10000).cost(acp::Cost::new(0.25, "USD")),
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
                async move { futures::future::pending::<Result<acp::PromptResponse>>().await }
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
}
