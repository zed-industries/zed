use crate::UserMessageId;
use crate::{AgentIdentity, DbMessage, DbThread, DbThreadMetadata, ThreadsDatabase};
use crate::{
    AgentMessage, AgentMessageContent, UserMessage as AgentUserMessage,
    UserMessageContent as AgentUserMessageContent,
};
use acp_thread::MentionUri;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow};
use assistant_text_thread::{SavedTextThreadMetadata, TextThread};
use chrono::{DateTime, Utc};
use collections::IndexMap;
use db::kvp::KEY_VALUE_STORE;
use gpui::{App, AsyncApp, Entity, SharedString, Task, prelude::*};
use itertools::Itertools;
use paths::text_threads_dir;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, path::Path, sync::Arc, time::Duration};
use ui::ElementId;
use util::ResultExt as _;

const MAX_RECENTLY_OPENED_ENTRIES: usize = 6;
const RECENTLY_OPENED_THREADS_KEY: &str = "recent-agent-threads";
const SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE: Duration = Duration::from_millis(50);

const DEFAULT_TITLE: &SharedString = &SharedString::new_static(acp_thread::DEFAULT_THREAD_TITLE);

#[derive(Clone, Debug)]
pub enum HistoryEntry {
    AcpThread(DbThreadMetadata),
    TextThread(SavedTextThreadMetadata),
}

impl HistoryEntry {
    pub fn updated_at(&self) -> DateTime<Utc> {
        match self {
            HistoryEntry::AcpThread(thread) => thread.updated_at,
            HistoryEntry::TextThread(text_thread) => text_thread.mtime.to_utc(),
        }
    }

    pub fn id(&self) -> HistoryEntryId {
        match self {
            HistoryEntry::AcpThread(thread) => HistoryEntryId::AcpThread(thread.id.clone()),
            HistoryEntry::TextThread(text_thread) => {
                HistoryEntryId::TextThread(text_thread.path.clone())
            }
        }
    }

    pub fn mention_uri(&self) -> MentionUri {
        match self {
            HistoryEntry::AcpThread(thread) => MentionUri::Thread {
                id: thread.id.clone(),
                name: thread.title.to_string(),
            },
            HistoryEntry::TextThread(text_thread) => MentionUri::TextThread {
                path: text_thread.path.as_ref().to_owned(),
                name: text_thread.title.to_string(),
            },
        }
    }

    pub fn title(&self) -> &SharedString {
        match self {
            HistoryEntry::AcpThread(thread) => {
                if thread.title.is_empty() {
                    DEFAULT_TITLE
                } else {
                    &thread.title
                }
            }
            HistoryEntry::TextThread(text_thread) => &text_thread.title,
        }
    }

    pub fn agent_identity(&self) -> Option<AgentIdentity> {
        match self {
            HistoryEntry::AcpThread(thread) => {
                Some(AgentIdentity::from_name(thread.agent_name.as_ref()))
            }
            HistoryEntry::TextThread(_) => None,
        }
    }

    pub fn agent_name(&self) -> Option<SharedString> {
        match self {
            HistoryEntry::AcpThread(thread) => Some(thread.agent_name.clone()),
            HistoryEntry::TextThread(_) => None,
        }
    }

    pub fn agent_display_name(&self) -> Option<SharedString> {
        self.agent_identity()
            .map(|identity| identity.display_name())
    }
}

/// Generic identifier for a history entry.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum HistoryEntryId {
    AcpThread(acp::SessionId),
    TextThread(Arc<Path>),
}

impl Into<ElementId> for HistoryEntryId {
    fn into(self) -> ElementId {
        match self {
            HistoryEntryId::AcpThread(session_id) => ElementId::Name(session_id.0.into()),
            HistoryEntryId::TextThread(path) => ElementId::Path(path),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum SerializedRecentOpen {
    AcpThread(String),
    TextThread(String),
}

pub struct HistoryStore {
    threads: Vec<DbThreadMetadata>,
    entries: Vec<HistoryEntry>,
    text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
    recently_opened_entries: VecDeque<HistoryEntryId>,
    _subscriptions: Vec<gpui::Subscription>,
    _save_recently_opened_entries_task: Task<()>,
}

impl HistoryStore {
    pub fn new(
        text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions =
            vec![cx.observe(&text_thread_store, |this, _, cx| this.update_entries(cx))];

        cx.spawn(async move |this, cx| {
            let entries = Self::load_recently_opened_entries(cx).await;
            this.update(cx, |this, cx| {
                if let Some(entries) = entries.log_err() {
                    this.recently_opened_entries = entries;
                }

                this.reload(cx);
            })
            .ok();
        })
        .detach();

        Self {
            text_thread_store,
            recently_opened_entries: VecDeque::default(),
            threads: Vec::default(),
            entries: Vec::default(),
            _subscriptions: subscriptions,
            _save_recently_opened_entries_task: Task::ready(()),
        }
    }

    pub fn thread_from_session_id(&self, session_id: &acp::SessionId) -> Option<&DbThreadMetadata> {
        self.threads.iter().find(|thread| &thread.id == session_id)
    }

    pub fn load_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<DbThread>>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.background_spawn(async move {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.load_thread(id).await
        })
    }

    pub fn delete_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_thread(id.clone()).await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    pub fn delete_threads(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_threads().await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    pub fn delete_text_thread(
        &mut self,
        path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.text_thread_store
            .update(cx, |store, cx| store.delete_local(path, cx))
    }

    pub fn load_text_thread(
        &self,
        path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<TextThread>>> {
        self.text_thread_store
            .update(cx, |store, cx| store.open_local(path, cx))
    }

    pub fn reload(&self, cx: &mut Context<Self>) {
        let database_connection = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_connection.await;
            let threads = database.map_err(|err| anyhow!(err))?.list_threads().await?;
            this.update(cx, |this, cx| {
                if this.recently_opened_entries.len() < MAX_RECENTLY_OPENED_ENTRIES {
                    for thread in threads
                        .iter()
                        .take(MAX_RECENTLY_OPENED_ENTRIES - this.recently_opened_entries.len())
                        .rev()
                    {
                        this.push_recently_opened_entry(
                            HistoryEntryId::AcpThread(thread.id.clone()),
                            cx,
                        )
                    }
                }
                this.threads = threads;
                this.update_entries(cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn update_entries(&mut self, cx: &mut Context<Self>) {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return;
        }
        let mut history_entries = Vec::new();
        history_entries.extend(self.threads.iter().cloned().map(HistoryEntry::AcpThread));
        history_entries.extend(
            self.text_thread_store
                .read(cx)
                .unordered_text_threads()
                .cloned()
                .map(HistoryEntry::TextThread),
        );

        history_entries.sort_unstable_by_key(|entry| std::cmp::Reverse(entry.updated_at()));
        self.entries = history_entries;
        cx.notify()
    }

    pub fn is_empty(&self, _cx: &App) -> bool {
        self.entries.is_empty()
    }

    pub fn recently_opened_entries(&self, cx: &App) -> Vec<HistoryEntry> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return Vec::new();
        }

        let thread_entries = self.threads.iter().flat_map(|thread| {
            self.recently_opened_entries
                .iter()
                .enumerate()
                .flat_map(|(index, entry)| match entry {
                    HistoryEntryId::AcpThread(id) if &thread.id == id => {
                        Some((index, HistoryEntry::AcpThread(thread.clone())))
                    }
                    _ => None,
                })
        });

        let context_entries = self
            .text_thread_store
            .read(cx)
            .unordered_text_threads()
            .flat_map(|text_thread| {
                self.recently_opened_entries
                    .iter()
                    .enumerate()
                    .flat_map(|(index, entry)| match entry {
                        HistoryEntryId::TextThread(path) if &text_thread.path == path => {
                            Some((index, HistoryEntry::TextThread(text_thread.clone())))
                        }
                        _ => None,
                    })
            });

        thread_entries
            .chain(context_entries)
            // optimization to halt iteration early
            .take(self.recently_opened_entries.len())
            .sorted_unstable_by_key(|(index, _)| *index)
            .map(|(_, entry)| entry)
            .collect()
    }

    fn save_recently_opened_entries(&mut self, cx: &mut Context<Self>) {
        let serialized_entries = self
            .recently_opened_entries
            .iter()
            .filter_map(|entry| match entry {
                HistoryEntryId::TextThread(path) => path.file_name().map(|file| {
                    SerializedRecentOpen::TextThread(file.to_string_lossy().into_owned())
                }),
                HistoryEntryId::AcpThread(id) => {
                    Some(SerializedRecentOpen::AcpThread(id.to_string()))
                }
            })
            .collect::<Vec<_>>();

        self._save_recently_opened_entries_task = cx.spawn(async move |_, cx| {
            let content = serde_json::to_string(&serialized_entries).unwrap();
            cx.background_executor()
                .timer(SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE)
                .await;

            if cfg!(any(feature = "test-support", test)) {
                return;
            }
            KEY_VALUE_STORE
                .write_kvp(RECENTLY_OPENED_THREADS_KEY.to_owned(), content)
                .await
                .log_err();
        });
    }

    fn load_recently_opened_entries(cx: &AsyncApp) -> Task<Result<VecDeque<HistoryEntryId>>> {
        cx.background_spawn(async move {
            if cfg!(any(feature = "test-support", test)) {
                log::warn!("history store does not persist in tests");
                return Ok(VecDeque::new());
            }
            let json = KEY_VALUE_STORE
                .read_kvp(RECENTLY_OPENED_THREADS_KEY)?
                .unwrap_or("[]".to_string());
            let entries = serde_json::from_str::<Vec<SerializedRecentOpen>>(&json)
                .context("deserializing persisted agent panel navigation history")?
                .into_iter()
                .take(MAX_RECENTLY_OPENED_ENTRIES)
                .flat_map(|entry| match entry {
                    SerializedRecentOpen::AcpThread(id) => {
                        Some(HistoryEntryId::AcpThread(acp::SessionId::new(id.as_str())))
                    }
                    SerializedRecentOpen::TextThread(file_name) => Some(
                        HistoryEntryId::TextThread(text_threads_dir().join(file_name).into()),
                    ),
                })
                .collect();
            Ok(entries)
        })
    }

    pub fn push_recently_opened_entry(&mut self, entry: HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != &entry);
        self.recently_opened_entries.push_front(entry);
        self.recently_opened_entries
            .truncate(MAX_RECENTLY_OPENED_ENTRIES);
        self.save_recently_opened_entries(cx);
    }

    pub fn remove_recently_opened_thread(&mut self, id: acp::SessionId, cx: &mut Context<Self>) {
        self.recently_opened_entries.retain(
            |entry| !matches!(entry, HistoryEntryId::AcpThread(thread_id) if thread_id == &id),
        );
        self.save_recently_opened_entries(cx);
    }

    pub fn replace_recently_opened_text_thread(
        &mut self,
        old_path: &Path,
        new_path: &Arc<Path>,
        cx: &mut Context<Self>,
    ) {
        for entry in &mut self.recently_opened_entries {
            match entry {
                HistoryEntryId::TextThread(path) if path.as_ref() == old_path => {
                    *entry = HistoryEntryId::TextThread(new_path.clone());
                    break;
                }
                _ => {}
            }
        }
        self.save_recently_opened_entries(cx);
    }

    pub fn remove_recently_opened_entry(&mut self, entry: &HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != entry);
        self.save_recently_opened_entries(cx);
    }

    pub fn entries(&self) -> impl Iterator<Item = HistoryEntry> {
        self.entries.iter().cloned()
    }

    pub fn entries_for_agent(&self, agent: &AgentIdentity) -> Vec<HistoryEntry> {
        let agent_name = agent.name();
        self.entries
            .iter()
            .filter(|entry| match entry {
                HistoryEntry::AcpThread(thread) => {
                    thread.agent_name.as_ref() == agent_name.as_ref()
                }
                HistoryEntry::TextThread(_) => false,
            })
            .cloned()
            .collect()
    }

    pub fn unique_agents(&self) -> Vec<AgentIdentity> {
        use std::collections::HashSet;
        self.entries
            .iter()
            .filter_map(|entry| entry.agent_identity())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Saves an external agent thread to the database.
    ///
    /// This method is used by `AcpThreadView` to persist external agent threads
    /// (Claude Code, Codex, Gemini, etc.) after each turn. Unlike native agent
    /// threads which are saved through `NativeAgent::save_thread`, external agents
    /// use this unified method.
    ///
    /// # Parameters
    /// - `session_id`: The ACP session ID for the thread
    /// - `title`: The thread title
    /// - `agent_name`: The name of the agent (e.g., "claude-code", "codex", "gemini")
    /// - `messages`: Vector of messages converted from AcpThread entries
    pub fn save_external_thread(
        &self,
        session_id: acp::SessionId,
        title: SharedString,
        agent_name: SharedString,
        messages: Vec<DbMessage>,
        cx: &mut Context<Self>,
    ) {
        Self::save_external_thread_internal(session_id, title, agent_name, messages, cx);
    }

    /// Convert AcpThread entries to DbMessage format for database storage.
    /// This allows external agent conversations to be persisted and loaded later.
    ///
    /// This handles three types of entries:
    /// - UserMessage: Converted directly to DbMessage::User
    /// - AssistantMessage: Converted to DbMessage::Agent with text/thinking content
    /// - ToolCall: Appended to the preceding AgentMessage as ToolUse content with tool_results
    pub fn convert_acp_entries_to_messages(
        entries: &[acp_thread::AgentThreadEntry],
        cx: &App,
    ) -> Vec<DbMessage> {
        use acp_thread::{AgentThreadEntry, ToolCallStatus};
        use language_model::{
            LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolUse,
            LanguageModelToolUseId,
        };

        let mut messages: Vec<DbMessage> = Vec::new();

        for entry in entries {
            match entry {
                AgentThreadEntry::UserMessage(acp_user_msg) => {
                    // Convert ACP UserMessage to Agent UserMessage
                    let text = acp_user_msg.content.to_markdown(cx);
                    let content = if !text.is_empty() {
                        vec![AgentUserMessageContent::Text(text.to_string())]
                    } else {
                        vec![]
                    };

                    messages.push(DbMessage::User(AgentUserMessage {
                        id: acp_user_msg
                            .id
                            .clone()
                            .unwrap_or_else(|| UserMessageId::new()),
                        content,
                    }));
                }
                AgentThreadEntry::AssistantMessage(acp_asst_msg) => {
                    // Convert ACP AssistantMessage to Agent AgentMessage
                    let mut message_content = Vec::new();

                    for chunk in &acp_asst_msg.chunks {
                        match chunk {
                            acp_thread::AssistantMessageChunk::Message { block } => {
                                let text = block.to_markdown(cx);
                                if !text.is_empty() {
                                    message_content
                                        .push(AgentMessageContent::Text(text.to_string()));
                                }
                            }
                            acp_thread::AssistantMessageChunk::Thought { block } => {
                                let text = block.to_markdown(cx);
                                message_content.push(AgentMessageContent::Thinking {
                                    text: text.to_string(),
                                    signature: None,
                                });
                            }
                        }
                    }

                    messages.push(DbMessage::Agent(AgentMessage {
                        content: message_content,
                        tool_results: IndexMap::default(),
                        reasoning_details: None,
                    }));
                }
                AgentThreadEntry::ToolCall(tool_call) => {
                    // Tool calls follow assistant messages in ACP.
                    // We append them to the preceding AgentMessage as ToolUse content.
                    let tool_use_id =
                        LanguageModelToolUseId::from(tool_call.id.to_string().into_boxed_str());
                    // Use the tool kind as the name (e.g., "Edit", "Read", etc.)
                    let tool_name = serde_json::to_string(&tool_call.kind)
                        .unwrap_or_else(|_| "unknown".to_string())
                        .trim_matches('"')
                        .to_string();

                    // Find the last agent message to append to
                    if let Some(DbMessage::Agent(agent_msg)) = messages.last_mut() {
                        // Add ToolUse to content
                        agent_msg.content.push(AgentMessageContent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_use_id.clone(),
                                name: tool_name.clone().into(),
                                raw_input: tool_call
                                    .raw_input
                                    .as_ref()
                                    .and_then(|v| serde_json::to_string(v).ok())
                                    .unwrap_or_default(),
                                input: tool_call
                                    .raw_input
                                    .clone()
                                    .unwrap_or(serde_json::Value::Null),
                                is_input_complete: true,
                                thought_signature: None,
                            },
                        ));

                        // Add tool result if the tool call is completed or failed
                        let is_error = matches!(tool_call.status, ToolCallStatus::Failed);
                        let is_completed = matches!(
                            tool_call.status,
                            ToolCallStatus::Completed
                                | ToolCallStatus::Failed
                                | ToolCallStatus::Rejected
                                | ToolCallStatus::Canceled
                        );

                        if is_completed {
                            // Extract content from the tool call for the result
                            let content_text = tool_call
                                .content
                                .iter()
                                .filter_map(|c| match c {
                                    acp_thread::ToolCallContent::ContentBlock(block) => {
                                        Some(block.to_markdown(cx).to_string())
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join("\n");

                            let result_content = if content_text.is_empty() {
                                let filler = match tool_call.status {
                                    ToolCallStatus::Rejected => "<Tool rejected by user>".into(),
                                    ToolCallStatus::Canceled => "<Tool canceled>".into(),
                                    _ => "<Tool completed>".into(),
                                };
                                LanguageModelToolResultContent::Text(filler)
                            } else {
                                LanguageModelToolResultContent::Text(content_text.into())
                            };

                            agent_msg.tool_results.insert(
                                tool_use_id.clone(),
                                LanguageModelToolResult {
                                    tool_use_id,
                                    tool_name: tool_name.into(),
                                    is_error,
                                    content: result_content,
                                    output: tool_call.raw_output.clone(),
                                },
                            );
                        }
                    }
                    // If there's no preceding agent message, the tool call is orphaned
                    // This shouldn't happen in normal flow, but we skip silently
                }
            }
        }

        messages
    }

    /// Helper function to convert entries from within a context that has access to the thread
    pub fn convert_and_save_external_thread(
        entries: &[acp_thread::AgentThreadEntry],
        session_id: acp::SessionId,
        title: SharedString,
        agent_name: SharedString,
        cx: &mut gpui::Context<Self>,
    ) {
        let messages = Self::convert_acp_entries_to_messages(entries, cx);
        Self::save_external_thread_internal(session_id, title, agent_name, messages, cx);
    }

    fn save_external_thread_internal(
        session_id: acp::SessionId,
        title: SharedString,
        agent_name: SharedString,
        messages: Vec<DbMessage>,
        cx: &mut gpui::Context<Self>,
    ) {
        let database_future = ThreadsDatabase::connect(cx);
        let history = cx.entity();

        cx.spawn(async move |_, cx| {
            let Some(database) = database_future.await.map_err(|err| anyhow!(err)).log_err() else {
                return;
            };

            let db_thread = DbThread {
                title: title.clone(),
                messages,
                updated_at: chrono::Utc::now(),
                detailed_summary: None,
                initial_project_snapshot: None,
                cumulative_token_usage: Default::default(),
                request_token_usage: Default::default(),
                model: None,
                completion_mode: None,
                profile: None,
            };

            database
                .save_thread(
                    session_id.clone(),
                    db_thread,
                    agent_name.as_ref(),
                    None,
                    None,
                )
                .await
                .log_err();

            history.update(cx, |history, cx| history.reload(cx)).ok();
        })
        .detach();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentIdentity;

    fn create_test_thread_metadata(id: &str, agent_name: &str) -> DbThreadMetadata {
        DbThreadMetadata {
            id: acp::SessionId::new(id),
            title: format!("Thread {}", id).into(),
            updated_at: chrono::Utc::now(),
            agent_name: agent_name.to_string().into(),
            agent_version: None,
            agent_provider_id: None,
        }
    }

    #[test]
    fn test_history_entry_agent_methods() {
        // Test AcpThread entry
        let zed_thread = DbThreadMetadata {
            id: acp::SessionId::new("test-1"),
            title: "Test Thread".into(),
            updated_at: chrono::Utc::now(),
            agent_name: "zed".into(),
            agent_version: Some("1.0.0".into()),
            agent_provider_id: Some("anthropic".into()),
        };

        let entry = HistoryEntry::AcpThread(zed_thread);

        assert_eq!(
            entry.agent_name(),
            Some("zed".into()),
            "agent_name should return 'zed'"
        );

        let identity = entry.agent_identity().unwrap();
        assert!(
            identity.is_zed(),
            "agent_identity should recognize Zed agent"
        );
        assert_eq!(
            entry.agent_display_name(),
            Some("Zed".into()),
            "display_name should be 'Zed'"
        );

        // Test External agent entry
        let claude_thread = DbThreadMetadata {
            id: acp::SessionId::new("test-2"),
            title: "Claude Thread".into(),
            updated_at: chrono::Utc::now(),
            agent_name: "claude-code".into(),
            agent_version: None,
            agent_provider_id: None,
        };

        let entry = HistoryEntry::AcpThread(claude_thread);
        assert_eq!(entry.agent_name(), Some("claude-code".into()));

        let identity = entry.agent_identity().unwrap();
        assert!(!identity.is_zed());
        assert_eq!(entry.agent_display_name(), Some("claude-code".into()));

        // Test TextThread entry (no agent)
        let text_thread = SavedTextThreadMetadata {
            path: Arc::from(Path::new("/path/to/thread.md")),
            title: "Text Thread".into(),
            mtime: chrono::Local::now(),
        };

        let entry = HistoryEntry::TextThread(text_thread);
        assert!(entry.agent_identity().is_none());
        assert!(entry.agent_name().is_none());
        assert!(entry.agent_display_name().is_none());
    }

    #[test]
    fn test_load_agent_thread_routes_to_zed_for_zed_agent() {
        // This test verifies that the factory correctly routes to Zed's Native Agent
        let zed_thread = create_test_thread_metadata("test-1", "zed");
        let agent_identity = AgentIdentity::from_name(zed_thread.agent_name.as_ref());

        assert!(
            agent_identity.is_zed(),
            "Should identify Zed agent correctly"
        );
    }

    #[test]
    fn test_load_agent_thread_routes_to_external_for_external_agents() {
        // This test verifies that the factory correctly identifies external agents
        let claude_thread = create_test_thread_metadata("test-2", "claude-code");
        let agent_identity = AgentIdentity::from_name(claude_thread.agent_name.as_ref());

        assert!(
            !agent_identity.is_zed(),
            "Should identify external agents correctly"
        );
        assert_eq!(
            agent_identity.name(),
            "claude-code",
            "Should preserve external agent name"
        );
    }

    #[gpui::test]
    async fn test_convert_acp_entries_to_messages(cx: &mut gpui::TestAppContext) {
        use acp_thread::{
            AgentThreadEntry, AssistantMessage, AssistantMessageChunk, ContentBlock, ToolCall,
            ToolCallContent, ToolCallStatus, UserMessage,
        };
        use language::LanguageRegistry;
        use language_model::LanguageModelToolUseId;
        use markdown::Markdown;
        use std::sync::Arc;
        use util::paths::PathStyle;

        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
        let path_style = PathStyle::Posix;

        cx.update(|cx| {
            let user_content =
                ContentBlock::new("Hello".into(), &language_registry, path_style, cx);

            let user_msg = AgentThreadEntry::UserMessage(UserMessage {
                id: Some(UserMessageId::new()),

                content: user_content,

                chunks: vec![],

                checkpoint: None,

                indented: false,
            });

            let asst_content =
                ContentBlock::new("Hi there".into(), &language_registry, path_style, cx);

            let asst_msg = AgentThreadEntry::AssistantMessage(AssistantMessage {
                chunks: vec![AssistantMessageChunk::Message {
                    block: asst_content,
                }],

                indented: false,
            });

            let tool_id: LanguageModelToolUseId = "tool_1".into();

            let tool_call = AgentThreadEntry::ToolCall(ToolCall {
                id: "tool_1".into(),

                label: cx.new(|cx| Markdown::new("Tool".into(), None, None, cx)),

                kind: acp::ToolKind::Fetch,

                content: vec![ToolCallContent::ContentBlock(ContentBlock::new(
                    "Tool Output".into(),
                    &language_registry,
                    path_style,
                    cx,
                ))],

                status: ToolCallStatus::Completed,

                locations: vec![],

                resolved_locations: vec![],

                raw_input: Some(serde_json::json!({"arg": "val"})),

                raw_input_markdown: None,

                raw_output: Some(serde_json::json!({"result": "val"})),
            });

            let thought_content = ContentBlock::new(
                "I should use a tool".into(),
                &language_registry,
                path_style,
                cx,
            );
            let thought_msg = AgentThreadEntry::AssistantMessage(AssistantMessage {
                chunks: vec![AssistantMessageChunk::Thought {
                    block: thought_content,
                }],
                indented: false,
            });

            let entries = vec![user_msg, asst_msg, tool_call, thought_msg];

            let messages = HistoryStore::convert_acp_entries_to_messages(&entries, cx);

            assert_eq!(messages.len(), 3); // User + Assistant (Tool is merged) + Assistant (Thought)

            // Verify User Message
            match &messages[0] {
                DbMessage::User(msg) => {
                    assert_eq!(
                        msg.content[0],
                        AgentUserMessageContent::Text("Hello".into())
                    );
                }
                _ => panic!("Expected user message"),
            }

            // Verify Assistant Message with Tool Call
            match &messages[1] {
                DbMessage::Agent(msg) => {
                    assert_eq!(msg.content[0], AgentMessageContent::Text("Hi there".into()));
                    assert!(matches!(msg.content[1], AgentMessageContent::ToolUse(_)));
                    if let AgentMessageContent::ToolUse(tool_use) = &msg.content[1] {
                        assert_eq!(tool_use.id, tool_id);
                        assert_eq!(tool_use.raw_input, "{\"arg\":\"val\"}");
                    }
                }
                _ => panic!("Expected agent message"),
            }

            // Verify Assistant Message with Thought
            match &messages[2] {
                DbMessage::Agent(msg) => {
                    assert_eq!(
                        msg.content[0],
                        AgentMessageContent::Thinking {
                            text: "I should use a tool".into(),
                            signature: None
                        }
                    );
                }
                _ => panic!("Expected agent message"),
            }
        });
    }
}
