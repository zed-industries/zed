use crate::{AgentMessage, AgentMessageContent, UserMessage, UserMessageContent};
use acp_thread::ClientUserMessageId;
use agent_client_protocol::schema::v1 as acp;
use agent_settings::AgentProfileId;
use anyhow::Result;
use chrono::{DateTime, Utc};
use collections::{HashMap, IndexMap};
use futures::{FutureExt, future::Shared};
use gpui::{BackgroundExecutor, Global, Task};
use indoc::indoc;
use language_model::Speed;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sqlez::{
    bindable::{Bind, Column},
    connection::Connection,
    statement::Statement,
};
use std::{io::ErrorKind, path::PathBuf, sync::Arc};
use ui::{App, SharedString};
use util::path_list::PathList;
use zed_env_vars::ZED_STATELESS;

pub type DbMessage = crate::Message;
pub type DbSummary = crate::legacy_thread::DetailedSummaryState;
pub type DbLanguageModel = crate::legacy_thread::SerializedLanguageModel;

#[derive(Debug, Clone)]
pub struct DbThreadMetadata {
    pub id: acp::SessionId,
    pub parent_session_id: Option<acp::SessionId>,
    pub title: SharedString,
    pub updated_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    /// The workspace folder paths this thread was created against, sorted
    /// lexicographically. Used for grouping threads by project in the sidebar.
    pub folder_paths: PathList,
}

impl From<&DbThreadMetadata> for acp_thread::AgentSessionInfo {
    fn from(meta: &DbThreadMetadata) -> Self {
        Self {
            session_id: meta.id.clone(),
            work_dirs: Some(meta.folder_paths.clone()),
            title: Some(meta.title.clone()),
            updated_at: Some(meta.updated_at),
            created_at: meta.created_at,
            meta: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbThread {
    pub title: SharedString,
    pub messages: Vec<Arc<DbMessage>>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub detailed_summary: Option<SharedString>,
    #[serde(default)]
    pub initial_project_snapshot: Option<Arc<crate::ProjectSnapshot>>,
    #[serde(default)]
    pub cumulative_token_usage: language_model::TokenUsage,
    #[serde(default)]
    pub request_token_usage: HashMap<acp_thread::ClientUserMessageId, language_model::TokenUsage>,
    #[serde(default)]
    pub model: Option<DbLanguageModel>,
    #[serde(default)]
    pub profile: Option<AgentProfileId>,
    #[serde(default)]
    pub subagent_context: Option<crate::SubagentContext>,
    #[serde(default)]
    pub speed: Option<Speed>,
    #[serde(default)]
    pub thinking_enabled: bool,
    #[serde(default)]
    pub thinking_effort: Option<String>,
    #[serde(default)]
    pub draft_prompt: Option<Vec<acp::ContentBlock>>,
    #[serde(default)]
    pub ui_scroll_position: Option<SerializedScrollPosition>,
    #[serde(default)]
    pub sandboxed_terminal_temp_dir: Option<PathBuf>,
    /// Sandbox escalations the user approved "for the rest of this thread".
    /// Persisted so reopening a thread keeps its grants. See
    /// [`crate::sandboxing::ThreadSandboxGrants`].
    #[serde(default)]
    pub sandbox_grants: DbSandboxGrants,
}

/// Serialized form of the sandbox permissions the user granted "for the rest of
/// this thread" (the "Allow for this thread" prompt option). Stored inside the
/// thread blob; round-trips with [`crate::sandboxing::ThreadSandboxGrants`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DbSandboxGrants {
    /// Paths granted write access, each paired with the canonical
    /// (symlink-resolved) target established when the grant was approved; each
    /// covers its whole subtree. Legacy rows stored a bare path string per
    /// entry, which still deserializes (as a grant with no resolved canonical)
    /// via [`settings::GrantedWritePath`]'s string-or-object format.
    #[serde(default)]
    pub write_paths: Vec<settings::GrantedWritePath>,
    /// Host patterns granted network access, in canonical string form (e.g.
    /// `github.com`, `*.npmjs.org`). Parsed back into patterns on load.
    #[serde(default)]
    pub network_hosts: Vec<String>,
    /// Whether arbitrary-host network access was granted.
    #[serde(default)]
    pub network_any_host: bool,
    /// Whether unrestricted filesystem writes (the broad escape hatch) were
    /// granted.
    #[serde(default)]
    pub allow_fs_write_all: bool,

    /// Whether the model-requested fully-unsandboxed escape was granted.
    #[serde(default)]
    pub unsandboxed: bool,
    /// Whether running commands unsandboxed was allowed because the OS sandbox
    /// could not be created (the fallback prompt's "for this thread" option).
    #[serde(default)]
    pub sandbox_fallback: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SerializedScrollPosition {
    pub item_ix: usize,
    pub offset_in_item: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedThread {
    pub title: SharedString,
    pub messages: Vec<Arc<DbMessage>>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub model: Option<DbLanguageModel>,
    pub version: String,
}

impl SharedThread {
    pub const VERSION: &'static str = "1.0.0";

    pub fn from_db_thread(thread: &DbThread) -> Self {
        Self {
            title: thread.title.clone(),
            messages: thread.messages.clone(),
            updated_at: thread.updated_at,
            model: thread.model.clone(),
            version: Self::VERSION.to_string(),
        }
    }

    pub fn to_db_thread(self) -> DbThread {
        DbThread {
            title: format!("🔗 {}", self.title).into(),
            messages: self.messages,
            updated_at: self.updated_at,
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: Default::default(),
            model: self.model,
            profile: None,
            subagent_context: None,
            speed: None,
            thinking_enabled: false,
            thinking_effort: None,
            draft_prompt: None,
            ui_scroll_position: None,
            sandboxed_terminal_temp_dir: None,
            sandbox_grants: DbSandboxGrants::default(),
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        const COMPRESSION_LEVEL: i32 = 3;
        let json = serde_json::to_vec(self)?;
        let compressed = zstd::encode_all(json.as_slice(), COMPRESSION_LEVEL)?;
        Ok(compressed)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let decompressed = zstd::decode_all(data)?;
        Ok(serde_json::from_slice(&decompressed)?)
    }
}

impl DbThread {
    pub const VERSION: &'static str = "0.3.0";

    pub fn to_markdown(&self) -> String {
        crate::messages_to_markdown(&self.messages)
    }

    pub fn from_json(json: &[u8]) -> Result<Self> {
        let saved_thread_json = serde_json::from_slice::<serde_json::Value>(json)?;
        match saved_thread_json.get("version") {
            Some(serde_json::Value::String(version)) => match version.as_str() {
                Self::VERSION => Ok(serde_json::from_value(saved_thread_json)?),
                _ => Self::upgrade_from_agent_1(crate::legacy_thread::SerializedThread::from_json(
                    json,
                )?),
            },
            _ => {
                Self::upgrade_from_agent_1(crate::legacy_thread::SerializedThread::from_json(json)?)
            }
        }
    }

    fn upgrade_from_agent_1(thread: crate::legacy_thread::SerializedThread) -> Result<Self> {
        let mut messages = Vec::new();
        let mut request_token_usage = HashMap::default();

        let mut last_user_message_id = None;
        for (ix, msg) in thread.messages.into_iter().enumerate() {
            let message = match msg.role {
                language_model::Role::User => {
                    let mut content = Vec::new();

                    // Convert segments to content
                    for segment in msg.segments {
                        match segment {
                            crate::legacy_thread::SerializedMessageSegment::Text { text } => {
                                content.push(UserMessageContent::Text(text));
                            }
                            crate::legacy_thread::SerializedMessageSegment::Thinking {
                                text,
                                ..
                            } => {
                                // User messages don't have thinking segments, but handle gracefully
                                content.push(UserMessageContent::Text(text));
                            }
                            crate::legacy_thread::SerializedMessageSegment::RedactedThinking {
                                ..
                            } => {
                                // User messages don't have redacted thinking, skip.
                            }
                        }
                    }

                    // If no content was added, add context as text if available
                    if content.is_empty() && !msg.context.is_empty() {
                        content.push(UserMessageContent::Text(msg.context));
                    }

                    let id = ClientUserMessageId::new();
                    last_user_message_id = Some(id.clone());

                    crate::Message::User(UserMessage {
                        // MessageId from old format can't be meaningfully converted, so generate a new one
                        id,
                        content: Arc::from(content),
                    })
                }
                language_model::Role::Assistant => {
                    let mut content = Vec::new();

                    // Convert segments to content
                    for segment in msg.segments {
                        match segment {
                            crate::legacy_thread::SerializedMessageSegment::Text { text } => {
                                content.push(AgentMessageContent::Text(text));
                            }
                            crate::legacy_thread::SerializedMessageSegment::Thinking {
                                text,
                                signature,
                            } => {
                                content.push(AgentMessageContent::Thinking { text, signature });
                            }
                            crate::legacy_thread::SerializedMessageSegment::RedactedThinking {
                                data,
                            } => {
                                content.push(AgentMessageContent::RedactedThinking(data));
                            }
                        }
                    }

                    // Convert tool uses
                    let mut tool_names_by_id = HashMap::default();
                    for tool_use in msg.tool_uses {
                        tool_names_by_id.insert(tool_use.id.clone(), tool_use.name.clone());
                        content.push(AgentMessageContent::ToolUse(
                            language_model::LanguageModelToolUse {
                                id: tool_use.id,
                                name: tool_use.name.into(),
                                raw_input: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                                input: language_model::LanguageModelToolUseInput::Json(
                                    tool_use.input,
                                ),
                                is_input_complete: true,
                                thought_signature: None,
                            },
                        ));
                    }

                    // Convert tool results
                    let mut tool_results = IndexMap::default();
                    for tool_result in msg.tool_results {
                        let name = tool_names_by_id
                            .remove(&tool_result.tool_use_id)
                            .unwrap_or_else(|| SharedString::from("unknown"));
                        tool_results.insert(
                            tool_result.tool_use_id.clone(),
                            language_model::LanguageModelToolResult {
                                tool_use_id: tool_result.tool_use_id,
                                tool_name: name.into(),
                                is_error: tool_result.is_error,
                                content: vec![tool_result.content],
                                output: tool_result.output,
                            },
                        );
                    }

                    if let Some(last_user_message_id) = &last_user_message_id
                        && let Some(token_usage) = thread.request_token_usage.get(ix).copied()
                    {
                        request_token_usage.insert(last_user_message_id.clone(), token_usage);
                    }

                    crate::Message::Agent(AgentMessage {
                        content,
                        tool_results,
                        reasoning_details: None,
                    })
                }
                language_model::Role::System => {
                    // Skip system messages as they're not supported in the new format
                    continue;
                }
            };

            messages.push(Arc::new(message));
        }

        Ok(Self {
            title: thread.summary,
            messages,
            updated_at: thread.updated_at,
            detailed_summary: match thread.detailed_summary_state {
                crate::legacy_thread::DetailedSummaryState::NotGenerated
                | crate::legacy_thread::DetailedSummaryState::Generating => None,
                crate::legacy_thread::DetailedSummaryState::Generated { text, .. } => Some(text),
            },
            initial_project_snapshot: thread.initial_project_snapshot,
            cumulative_token_usage: thread.cumulative_token_usage,
            request_token_usage,
            model: thread.model,
            profile: thread.profile,
            subagent_context: None,
            speed: None,
            thinking_enabled: false,
            thinking_effort: None,
            draft_prompt: None,
            ui_scroll_position: None,
            sandboxed_terminal_temp_dir: None,
            sandbox_grants: DbSandboxGrants::default(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "zstd")]
    Zstd,
}

impl Bind for DataType {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let value = match self {
            DataType::Json => "json",
            DataType::Zstd => "zstd",
        };
        value.bind(statement, start_index)
    }
}

impl Column for DataType {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (value, next_index) = String::column(statement, start_index)?;
        let data_type = match value.as_str() {
            "json" => DataType::Json,
            "zstd" => DataType::Zstd,
            _ => anyhow::bail!("Unknown data type: {}", value),
        };
        Ok((data_type, next_index))
    }
}

pub(crate) struct ThreadsDatabase {
    executor: BackgroundExecutor,
    connection: Arc<Mutex<Connection>>,
    /// In production, saves take real time (serialization, zstd, disk I/O) while
    /// the user keeps typing, so new save requests routinely arrive mid-write.
    /// The test executor completes writes instantly, so tests use this gate to
    /// hold a write in flight and interleave more save requests with it.
    #[cfg(test)]
    write_gate: Mutex<Option<Shared<futures::channel::oneshot::Receiver<()>>>>,
}

struct GlobalThreadsDatabase(Shared<Task<Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>>);

impl Global for GlobalThreadsDatabase {}

impl ThreadsDatabase {
    pub fn connect(cx: &mut App) -> Shared<Task<Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>> {
        if cx.has_global::<GlobalThreadsDatabase>() {
            return cx.global::<GlobalThreadsDatabase>().0.clone();
        }
        let executor = cx.background_executor().clone();
        let task = executor
            .spawn({
                let executor = executor.clone();
                async move {
                    match ThreadsDatabase::new(executor) {
                        Ok(db) => Ok(Arc::new(db)),
                        Err(err) => Err(Arc::new(err)),
                    }
                }
            })
            .shared();

        cx.set_global(GlobalThreadsDatabase(task.clone()));
        task
    }

    pub fn new(executor: BackgroundExecutor) -> Result<Self> {
        let connection = if *ZED_STATELESS {
            Connection::open_memory(Some("THREAD_FALLBACK_DB"))
        } else if cfg!(any(feature = "test-support", test)) {
            // rust stores the name of the test on the current thread.
            // We use this to automatically create a database that will
            // be shared within the test (for the test_retrieve_old_thread)
            // but not with concurrent tests.
            let thread = std::thread::current();
            let test_name = thread.name();
            Connection::open_memory(Some(&format!(
                "THREAD_FALLBACK_{}",
                test_name.unwrap_or_default()
            )))
        } else {
            let threads_dir = paths::data_dir().join("threads");
            std::fs::create_dir_all(&threads_dir)?;
            let sqlite_path = threads_dir.join("threads.db");
            Connection::open_file(&sqlite_path.to_string_lossy())
        };

        connection.exec(indoc! {"
            CREATE TABLE IF NOT EXISTS threads (
                id TEXT PRIMARY KEY,
                summary TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                data_type TEXT NOT NULL,
                data BLOB NOT NULL
            )
        "})?()
        .map_err(|e| e.context("Failed to create threads table"))?;

        if let Ok(mut s) = connection.exec(indoc! {"
            ALTER TABLE threads ADD COLUMN parent_id TEXT
        "})
        {
            s().ok();
        }

        if let Ok(mut s) = connection.exec(indoc! {"
            ALTER TABLE threads ADD COLUMN folder_paths TEXT;
            ALTER TABLE threads ADD COLUMN folder_paths_order TEXT;
        "})
        {
            s().ok();
        }

        if let Ok(mut s) = connection.exec(indoc! {"
            ALTER TABLE threads ADD COLUMN created_at TEXT;
        "})
        {
            if s().is_ok() {
                connection.exec(indoc! {"
                    UPDATE threads SET created_at = updated_at WHERE created_at IS NULL
                "})?()?;
            }
        }

        let db = Self {
            executor,
            connection: Arc::new(Mutex::new(connection)),
            #[cfg(test)]
            write_gate: Mutex::new(None),
        };

        Ok(db)
    }

    fn save_thread_sync(
        connection: &Arc<Mutex<Connection>>,
        id: acp::SessionId,
        thread: DbThread,
        folder_paths: &PathList,
    ) -> Result<()> {
        const COMPRESSION_LEVEL: i32 = 3;

        #[derive(Serialize)]
        struct SerializedThread {
            #[serde(flatten)]
            thread: DbThread,
            version: &'static str,
        }

        let title = thread.title.to_string();
        let updated_at = thread.updated_at.to_rfc3339();
        let parent_id = thread
            .subagent_context
            .as_ref()
            .map(|ctx| ctx.parent_thread_id.0.clone());
        let serialized_folder_paths = folder_paths.serialize();
        let (folder_paths_str, folder_paths_order_str): (Option<String>, Option<String>) =
            if folder_paths.is_empty() {
                (None, None)
            } else {
                (
                    Some(serialized_folder_paths.paths),
                    Some(serialized_folder_paths.order),
                )
            };
        // Serialize into the compressor while holding the connection so
        // concurrent saves cannot each retain an uncompressed snapshot while
        // waiting on this lock. Streaming also avoids a second full copy of
        // the JSON before compression.
        let connection = connection.lock();

        let mut encoder = zstd::stream::write::Encoder::new(Vec::new(), COMPRESSION_LEVEL)?;
        serde_json::to_writer(
            &mut encoder,
            &SerializedThread {
                thread,
                version: DbThread::VERSION,
            },
        )?;
        let data = encoder.finish()?;
        let data_type = DataType::Zstd;

        // Use the thread's updated_at as created_at for new threads.
        // This ensures the creation time reflects when the thread was conceptually
        // created, not when it was saved to the database.
        let created_at = updated_at.clone();

        let mut insert = connection.exec_bound::<(Arc<str>, Option<Arc<str>>, Option<String>, Option<String>, String, String, DataType, Vec<u8>, String)>(indoc! {"
            INSERT INTO threads (id, parent_id, folder_paths, folder_paths_order, summary, updated_at, data_type, data, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                parent_id = excluded.parent_id,
                folder_paths = excluded.folder_paths,
                folder_paths_order = excluded.folder_paths_order,
                summary = excluded.summary,
                updated_at = excluded.updated_at,
                data_type = excluded.data_type,
                data = excluded.data
        "})?;

        insert((
            id.0,
            parent_id,
            folder_paths_str,
            folder_paths_order_str,
            title,
            updated_at,
            data_type,
            data,
            created_at,
        ))?;

        Ok(())
    }

    pub fn list_threads(&self) -> Task<Result<Vec<DbThreadMetadata>>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();

            let mut select = connection
                .select_bound::<(), (Arc<str>, Option<Arc<str>>, Option<String>, Option<String>, String, String, Option<String>)>(indoc! {"
                SELECT id, parent_id, folder_paths, folder_paths_order, summary, updated_at, created_at FROM threads ORDER BY updated_at DESC, created_at DESC
            "})?;

            let rows = select(())?;
            let mut threads = Vec::new();

            for (id, parent_id, folder_paths, folder_paths_order, summary, updated_at, created_at) in rows {
                let folder_paths = folder_paths
                    .map(|paths| {
                        PathList::deserialize(&util::path_list::SerializedPathList {
                            paths,
                            order: folder_paths_order.unwrap_or_default(),
                        })
                    })
                    .unwrap_or_default();
                let created_at = created_at
                    .as_deref()
                    .map(DateTime::parse_from_rfc3339)
                    .transpose()?
                    .map(|dt| dt.with_timezone(&Utc));

                threads.push(DbThreadMetadata {
                    id: acp::SessionId::new(id),
                    parent_session_id: parent_id.map(acp::SessionId::new),
                    title: summary.into(),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
                    created_at,
                    folder_paths,
                });
            }

            Ok(threads)
        })
    }

    pub fn load_thread(&self, id: acp::SessionId) -> Task<Result<Option<DbThread>>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();
            let mut select = connection.select_bound::<Arc<str>, (DataType, Vec<u8>)>(indoc! {"
                SELECT data_type, data FROM threads WHERE id = ? LIMIT 1
            "})?;

            let rows = select(id.0)?;
            if let Some((data_type, data)) = rows.into_iter().next() {
                Ok(Some(Self::deserialize_thread(data_type, data)?))
            } else {
                Ok(None)
            }
        })
    }

    pub fn save_thread(
        &self,
        id: acp::SessionId,
        thread: DbThread,
        folder_paths: PathList,
    ) -> Task<Result<()>> {
        let connection = self.connection.clone();
        #[cfg(test)]
        let write_gate = self.write_gate.lock().clone();

        self.executor.spawn(async move {
            #[cfg(test)]
            if let Some(write_gate) = write_gate {
                write_gate.await.ok();
            }
            Self::save_thread_sync(&connection, id, thread, &folder_paths)
        })
    }

    #[cfg(test)]
    pub fn set_write_gate(&self, gate: futures::channel::oneshot::Receiver<()>) {
        *self.write_gate.lock() = Some(gate.shared());
    }

    fn deserialize_thread(data_type: DataType, data: Vec<u8>) -> Result<DbThread> {
        let json_data = match data_type {
            DataType::Zstd => {
                let decompressed = zstd::decode_all(&data[..])?;
                String::from_utf8(decompressed)?
            }
            DataType::Json => String::from_utf8(data)?,
        };
        DbThread::from_json(json_data.as_bytes())
    }

    fn sandboxed_terminal_temp_dir(data_type: DataType, data: Vec<u8>) -> Option<PathBuf> {
        match Self::deserialize_thread(data_type, data) {
            Ok(thread) => thread.sandboxed_terminal_temp_dir,
            Err(error) => {
                log::warn!("failed to deserialize thread before deleting it: {error:#}");
                None
            }
        }
    }

    fn remove_sandboxed_terminal_temp_dir(temp_dir: PathBuf) {
        match std::fs::remove_dir_all(&temp_dir) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => {
                log::warn!(
                    "failed to remove sandboxed terminal temp directory {}: {error}",
                    temp_dir.display()
                );
            }
        }
    }

    pub fn delete_thread(&self, id: acp::SessionId) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let sandboxed_terminal_temp_dirs = {
                let connection = connection.lock();

                let mut select_children =
                    connection.select_bound::<Arc<str>, Arc<str>>(indoc! {"
                    SELECT id FROM threads WHERE parent_id = ?
                "})?;

                // Collect target thread together with all of its transitive
                // subagent threads
                let mut ids_to_delete = vec![id.0.clone()];
                let mut frontier = vec![id.0.clone()];
                while let Some(parent) = frontier.pop() {
                    for child in select_children(parent)? {
                        ids_to_delete.push(child.clone());
                        frontier.push(child);
                    }
                }

                let mut select =
                    connection.select_bound::<Arc<str>, (DataType, Vec<u8>)>(indoc! {"
                    SELECT data_type, data FROM threads WHERE id = ? LIMIT 1
                "})?;

                let mut delete = connection.exec_bound::<Arc<str>>(indoc! {"
                    DELETE FROM threads WHERE id = ?
                "})?;

                let mut sandboxed_terminal_temp_dirs = Vec::new();
                for thread_id in ids_to_delete {
                    if let Some(temp_dir) = select(thread_id.clone())?.into_iter().next().and_then(
                        |(data_type, data)| Self::sandboxed_terminal_temp_dir(data_type, data),
                    ) {
                        sandboxed_terminal_temp_dirs.push(temp_dir);
                    }
                    delete(thread_id)?;
                }

                sandboxed_terminal_temp_dirs
            };

            for temp_dir in sandboxed_terminal_temp_dirs {
                Self::remove_sandboxed_terminal_temp_dir(temp_dir);
            }

            Ok(())
        })
    }

    pub fn delete_threads(&self) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let sandboxed_terminal_temp_dirs = {
                let connection = connection.lock();

                let mut select = connection.select_bound::<(), (DataType, Vec<u8>)>(indoc! {"
                    SELECT data_type, data FROM threads
                "})?;

                let sandboxed_terminal_temp_dirs = select(())?
                    .into_iter()
                    .filter_map(|(data_type, data)| {
                        Self::sandboxed_terminal_temp_dir(data_type, data)
                    })
                    .collect::<Vec<_>>();

                let mut delete = connection.exec_bound::<()>(indoc! {"
                    DELETE FROM threads
                "})?;

                delete(())?;

                sandboxed_terminal_temp_dirs
            };

            for temp_dir in sandboxed_terminal_temp_dirs {
                Self::remove_sandboxed_terminal_temp_dir(temp_dir);
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    use collections::HashMap;
    use gpui::TestAppContext;
    use indoc::indoc;
    use serde::Serialize;
    use std::sync::Arc;

    #[test]
    fn test_shared_thread_roundtrip() {
        let original = SharedThread {
            title: "Test Thread".into(),
            messages: vec![],
            updated_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            model: None,
            version: SharedThread::VERSION.to_string(),
        };

        let bytes = original.to_bytes().expect("Failed to serialize");
        let restored = SharedThread::from_bytes(&bytes).expect("Failed to deserialize");

        assert_eq!(restored.title, original.title);
        assert_eq!(restored.version, original.version);
        assert_eq!(restored.updated_at, original.updated_at);
    }

    fn session_id(value: &str) -> acp::SessionId {
        acp::SessionId::new(Arc::<str>::from(value))
    }

    fn make_thread(title: &str, updated_at: DateTime<Utc>) -> DbThread {
        DbThread {
            title: title.to_string().into(),
            messages: Vec::new(),
            updated_at,
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: HashMap::default(),
            model: None,
            profile: None,
            subagent_context: None,
            speed: None,
            thinking_enabled: false,
            thinking_effort: None,
            draft_prompt: None,
            ui_scroll_position: None,
            sandboxed_terminal_temp_dir: None,
            sandbox_grants: DbSandboxGrants::default(),
        }
    }

    #[gpui::test]
    async fn test_list_threads_orders_by_created_at(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let older_id = session_id("thread-a");
        let newer_id = session_id("thread-b");

        let older_thread = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        let newer_thread = make_thread(
            "Thread B",
            Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
        );

        database
            .save_thread(older_id.clone(), older_thread, PathList::default())
            .await
            .unwrap();
        database
            .save_thread(newer_id.clone(), newer_thread, PathList::default())
            .await
            .unwrap();

        let entries = database.list_threads().await.unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, newer_id);
        assert_eq!(entries[1].id, older_id);
    }

    #[gpui::test]
    async fn test_save_thread_replaces_metadata(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let thread_id = session_id("thread-a");
        let original_thread = make_thread(
            "Thread A",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        let updated_thread = make_thread(
            "Thread B",
            Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
        );

        database
            .save_thread(thread_id.clone(), original_thread, PathList::default())
            .await
            .unwrap();
        database
            .save_thread(thread_id.clone(), updated_thread, PathList::default())
            .await
            .unwrap();

        let entries = database.list_threads().await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, thread_id);
        assert_eq!(entries[0].title.as_ref(), "Thread B");
        assert_eq!(
            entries[0].updated_at,
            Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
        );
        assert!(
            entries[0].created_at.is_some(),
            "created_at should be populated"
        );
    }

    #[test]
    fn test_subagent_context_defaults_to_none() {
        let json = r#"{
            "title": "Old Thread",
            "messages": [],
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;

        let db_thread: DbThread = serde_json::from_str(json).expect("Failed to deserialize");

        assert!(
            db_thread.subagent_context.is_none(),
            "Legacy threads without subagent_context should default to None"
        );
    }

    #[test]
    fn test_draft_prompt_defaults_to_none() {
        let json = r#"{
            "title": "Old Thread",
            "messages": [],
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;

        let db_thread: DbThread = serde_json::from_str(json).expect("Failed to deserialize");

        assert!(
            db_thread.draft_prompt.is_none(),
            "Legacy threads without draft_prompt field should default to None"
        );
    }

    #[test]
    fn test_sandboxed_terminal_temp_dir_defaults_to_none() {
        let json = r#"{
            "title": "Old Thread",
            "messages": [],
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;

        let db_thread: DbThread = serde_json::from_str(json).expect("Failed to deserialize");

        assert!(
            db_thread.sandboxed_terminal_temp_dir.is_none(),
            "Legacy threads without sandboxed_terminal_temp_dir should default to None"
        );
    }

    #[test]
    fn test_sandbox_grants_default_when_absent() {
        let json = r#"{
            "title": "Old Thread",
            "messages": [],
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;

        let db_thread: DbThread = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(
            db_thread.sandbox_grants,
            DbSandboxGrants::default(),
            "Legacy threads without sandbox_grants should default to empty grants"
        );
    }

    #[gpui::test]
    async fn test_sandbox_grants_roundtrip_through_save_load(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();
        let thread_id = session_id("sandbox-grants-thread");
        let mut thread = make_thread(
            "Sandbox Grants Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        let grants = DbSandboxGrants {
            write_paths: vec![
                // A legacy bare-string grant (no resolved canonical) and a grant
                // carrying its resolved canonical, to exercise both forms of the
                // string-or-object round-trip.
                settings::GrantedWritePath::from_requested(PathBuf::from("/tmp/build")),
                settings::GrantedWritePath::resolved(
                    PathBuf::from("/tmp/link"),
                    PathBuf::from("/tmp/real"),
                ),
            ],
            network_hosts: vec!["github.com".to_string(), "*.npmjs.org".to_string()],
            network_any_host: false,
            allow_fs_write_all: false,
            unsandboxed: true,
            sandbox_fallback: true,
        };
        thread.sandbox_grants = grants.clone();

        database
            .save_thread(thread_id.clone(), thread, PathList::default())
            .await
            .unwrap();

        let loaded = database
            .load_thread(thread_id)
            .await
            .unwrap()
            .expect("thread should exist");
        assert_eq!(loaded.sandbox_grants, grants);
    }

    #[gpui::test]
    async fn test_sandboxed_terminal_temp_dir_roundtrips_through_save_load(
        cx: &mut TestAppContext,
    ) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();
        let thread_id = session_id("sandbox-temp-dir-thread");
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-agent-terminal-test-")
            .tempdir()
            .unwrap()
            .keep();
        let mut thread = make_thread(
            "Sandbox Temp Dir Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        thread.sandboxed_terminal_temp_dir = Some(temp_dir.clone());

        database
            .save_thread(thread_id.clone(), thread, PathList::default())
            .await
            .unwrap();

        let loaded = database
            .load_thread(thread_id)
            .await
            .unwrap()
            .expect("thread should exist");
        assert_eq!(loaded.sandboxed_terminal_temp_dir, Some(temp_dir.clone()));
        std::fs::remove_dir_all(temp_dir).unwrap();
    }

    #[gpui::test]
    async fn test_delete_thread_removes_sandboxed_terminal_temp_dir(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();
        let thread_id = session_id("sandbox-temp-dir-delete-thread");
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-agent-terminal-test-")
            .tempdir()
            .unwrap()
            .keep();
        std::fs::write(temp_dir.join("sentinel"), b"content").unwrap();
        let mut thread = make_thread(
            "Sandbox Temp Dir Delete Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        thread.sandboxed_terminal_temp_dir = Some(temp_dir.clone());

        database
            .save_thread(thread_id.clone(), thread, PathList::default())
            .await
            .unwrap();
        database.delete_thread(thread_id).await.unwrap();

        assert!(!temp_dir.exists());
    }

    #[gpui::test]
    async fn test_delete_thread_deletes_subagent_threads(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let parent_id = session_id("parent-thread");
        let child_id = session_id("child-thread");
        let grandchild_id = session_id("grandchild-thread");
        let unrelated_id = session_id("unrelated-thread");

        let parent_thread = make_thread(
            "Parent Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );

        let mut child_thread = make_thread(
            "Child Subagent Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        child_thread.subagent_context = Some(crate::SubagentContext {
            parent_thread_id: parent_id.clone(),
            depth: 1,
        });

        let mut grandchild_thread = make_thread(
            "Grandchild Subagent Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        grandchild_thread.subagent_context = Some(crate::SubagentContext {
            parent_thread_id: child_id.clone(),
            depth: 2,
        });

        let unrelated_thread = make_thread(
            "Unrelated Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );

        for (id, thread) in [
            (parent_id.clone(), parent_thread),
            (child_id.clone(), child_thread),
            (grandchild_id.clone(), grandchild_thread),
            (unrelated_id.clone(), unrelated_thread),
        ] {
            database
                .save_thread(id, thread, PathList::default())
                .await
                .unwrap();
        }

        database.delete_thread(parent_id.clone()).await.unwrap();

        let remaining = database.list_threads().await.unwrap();
        let remaining_ids: Vec<_> = remaining.iter().map(|thread| thread.id.clone()).collect();
        assert_eq!(remaining_ids, vec![unrelated_id]);
    }

    #[gpui::test]
    async fn test_subagent_context_roundtrips_through_save_load(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let parent_id = session_id("parent-thread");
        let child_id = session_id("child-thread");

        let mut child_thread = make_thread(
            "Subagent Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        child_thread.subagent_context = Some(crate::SubagentContext {
            parent_thread_id: parent_id.clone(),
            depth: 2,
        });

        database
            .save_thread(child_id.clone(), child_thread, PathList::default())
            .await
            .unwrap();

        let loaded = database
            .load_thread(child_id)
            .await
            .unwrap()
            .expect("thread should exist");

        let context = loaded
            .subagent_context
            .expect("subagent_context should be restored");
        assert_eq!(context.parent_thread_id, parent_id);
        assert_eq!(context.depth, 2);
    }

    #[gpui::test]
    async fn test_non_subagent_thread_has_no_subagent_context(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let thread_id = session_id("regular-thread");
        let thread = make_thread(
            "Regular Thread",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );

        database
            .save_thread(thread_id.clone(), thread, PathList::default())
            .await
            .unwrap();

        let loaded = database
            .load_thread(thread_id)
            .await
            .unwrap()
            .expect("thread should exist");

        assert!(
            loaded.subagent_context.is_none(),
            "Regular threads should have no subagent_context"
        );
    }

    #[gpui::test]
    async fn test_folder_paths_roundtrip(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let thread_id = session_id("folder-thread");
        let thread = make_thread(
            "Folder Thread",
            Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap(),
        );

        let folder_paths = PathList::new(&[
            std::path::PathBuf::from("/home/user/project-a"),
            std::path::PathBuf::from("/home/user/project-b"),
        ]);

        database
            .save_thread(thread_id.clone(), thread, folder_paths.clone())
            .await
            .unwrap();

        let threads = database.list_threads().await.unwrap();
        assert_eq!(threads.len(), 1);
    }

    #[gpui::test]
    async fn test_folder_paths_empty_when_not_set(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let thread_id = session_id("no-folder-thread");
        let thread = make_thread(
            "No Folder Thread",
            Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap(),
        );

        database
            .save_thread(thread_id.clone(), thread, PathList::default())
            .await
            .unwrap();

        let threads = database.list_threads().await.unwrap();
        assert_eq!(threads.len(), 1);
    }

    #[test]
    fn test_scroll_position_defaults_to_none() {
        let json = r#"{
            "title": "Old Thread",
            "messages": [],
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;

        let db_thread: DbThread = serde_json::from_str(json).expect("Failed to deserialize");

        assert!(
            db_thread.ui_scroll_position.is_none(),
            "Legacy threads without scroll_position field should default to None"
        );
    }

    #[gpui::test]
    async fn test_scroll_position_roundtrips_through_save_load(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();

        let thread_id = session_id("thread-with-scroll");

        let mut thread = make_thread(
            "Thread With Scroll",
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        );
        thread.ui_scroll_position = Some(SerializedScrollPosition {
            item_ix: 42,
            offset_in_item: 13.5,
        });

        database
            .save_thread(thread_id.clone(), thread, PathList::default())
            .await
            .unwrap();

        let loaded = database
            .load_thread(thread_id)
            .await
            .unwrap()
            .expect("thread should exist");

        let scroll = loaded
            .ui_scroll_position
            .expect("scroll_position should be restored");
        assert_eq!(scroll.item_ix, 42);
        assert!((scroll.offset_in_item - 13.5).abs() < f32::EPSILON);
    }

    const USER_IMAGE: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=";
    const TOOL_IMAGE: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
    const REVISED_USER_IMAGE: &str = "revised-user-image-payload";
    const REVISED_TOOL_IMAGE: &str = "revised-tool-image-payload";
    const OTHER_USER_IMAGE: &str = "other-session-user-image";
    const OTHER_TOOL_IMAGE: &str = "other-session-tool-image";

    fn image_bearing_thread(
        title: &str,
        updated_at: DateTime<Utc>,
        user_text: &str,
        user_image: &str,
        tool_image: &str,
        summary: &str,
        compaction: &str,
    ) -> DbThread {
        let user_message_id = acp_thread::ClientUserMessageId::new();
        let tool_use_id = language_model::LanguageModelToolUseId::from("tool-use-1");
        let mut tool_results = collections::IndexMap::default();
        tool_results.insert(
            tool_use_id.clone(),
            language_model::LanguageModelToolResult {
                tool_use_id: tool_use_id.clone(),
                tool_name: "read_image".into(),
                is_error: false,
                content: vec![language_model::LanguageModelToolResultContent::Image(
                    language_model::LanguageModelImage {
                        source: tool_image.into(),
                    },
                )],
                output: None,
            },
        );

        let mut thread = make_thread(title, updated_at);
        thread.messages = vec![
            Arc::new(crate::Message::User(crate::UserMessage {
                id: user_message_id.clone(),
                content: Arc::from([
                    crate::UserMessageContent::Text(user_text.to_string()),
                    crate::UserMessageContent::Image(language_model::LanguageModelImage {
                        source: user_image.into(),
                    }),
                ]),
            })),
            Arc::new(crate::Message::Agent(crate::AgentMessage {
                content: vec![
                    crate::AgentMessageContent::Text("Here is the tool image".into()),
                    crate::AgentMessageContent::ToolUse(language_model::LanguageModelToolUse {
                        id: tool_use_id,
                        name: "read_image".into(),
                        raw_input: "{\"path\":\"café.png\"}".into(),
                        input: language_model::LanguageModelToolUseInput::Json(serde_json::json!({
                            "path": "café.png"
                        })),
                        is_input_complete: true,
                        thought_signature: None,
                    }),
                ],
                tool_results,
                reasoning_details: None,
            })),
            Arc::new(crate::Message::Compaction(crate::CompactionInfo::Summary(
                compaction.into(),
            ))),
        ];
        thread.detailed_summary = Some(summary.into());
        thread.cumulative_token_usage = language_model::TokenUsage {
            input_tokens: 11,
            output_tokens: 22,
            cache_creation_input_tokens: 3,
            cache_read_input_tokens: 4,
        };
        thread.request_token_usage.insert(
            user_message_id,
            language_model::TokenUsage {
                input_tokens: 5,
                output_tokens: 6,
                cache_creation_input_tokens: 1,
                cache_read_input_tokens: 2,
            },
        );
        thread
    }

    fn stored_thread_row(database: &ThreadsDatabase, id: &acp::SessionId) -> (String, Vec<u8>) {
        let connection = database.connection.lock();
        let mut select = connection
            .select_bound::<Arc<str>, (String, Vec<u8>)>(
                "SELECT data_type, data FROM threads WHERE id = ?",
            )
            .expect("prepare stored row query");
        select(id.0.clone())
            .expect("read stored row")
            .into_iter()
            .next()
            .expect("stored row")
    }

    fn insert_raw_thread(
        database: &ThreadsDatabase,
        id: &acp::SessionId,
        summary: &str,
        updated_at: &str,
        data_type: DataType,
        data: Vec<u8>,
    ) {
        let connection = database.connection.lock();
        let mut insert = connection
            .exec_bound::<(
                Arc<str>,
                Option<Arc<str>>,
                Option<String>,
                Option<String>,
                String,
                String,
                DataType,
                Vec<u8>,
                String,
            )>(indoc! {"
                INSERT INTO threads (
                    id, parent_id, folder_paths, folder_paths_order, summary,
                    updated_at, data_type, data, created_at
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "})
            .expect("prepare raw insert");
        insert((
            id.0.clone(),
            None,
            None,
            None,
            summary.to_string(),
            updated_at.to_string(),
            data_type,
            data,
            updated_at.to_string(),
        ))
        .expect("insert raw thread");
    }

    #[derive(Serialize)]
    struct VersionedThread<'a> {
        #[serde(flatten)]
        thread: &'a DbThread,
        version: &'static str,
    }

    fn versioned_thread_json(thread: &DbThread) -> Vec<u8> {
        serde_json::to_vec(&VersionedThread {
            thread,
            version: DbThread::VERSION,
        })
        .expect("serialize versioned thread")
    }

    #[gpui::test]
    async fn test_image_thread_round_trips_content_metadata_and_zstd_envelope(
        cx: &mut TestAppContext,
    ) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();
        let thread_id = session_id("image-thread");
        let updated_at = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let user_text = "Screenshot of café menu — 日本語の説明";
        let summary = "Image thread summary";
        let compaction = "Compacted: kept the café photo";
        let thread = image_bearing_thread(
            "Photos", updated_at, user_text, USER_IMAGE, TOOL_IMAGE, summary, compaction,
        );
        let expected_messages = thread.messages.clone();
        let expected_request_usage = thread.request_token_usage.clone();
        let expected_usage = thread.cumulative_token_usage;
        let folder_paths = PathList::new(&[
            std::path::PathBuf::from("/home/user/project-a"),
            std::path::PathBuf::from("/home/user/project-b"),
        ]);

        database
            .save_thread(thread_id.clone(), thread, folder_paths.clone())
            .await
            .unwrap();

        let (data_type, data) = stored_thread_row(&database, &thread_id);
        assert_eq!(data_type, "zstd");
        let json = zstd::decode_all(data.as_slice()).expect("decompress saved thread");
        let value: serde_json::Value =
            serde_json::from_slice(&json).expect("saved thread should be json");
        assert_eq!(value["version"], DbThread::VERSION);
        assert_eq!(value["title"], "Photos");
        assert_eq!(value["detailed_summary"], summary);
        let json_text = String::from_utf8(json).expect("saved json is utf-8");
        assert!(json_text.contains(USER_IMAGE));
        assert!(json_text.contains(TOOL_IMAGE));
        assert!(json_text.contains(user_text));
        assert!(json_text.contains(compaction));

        let loaded = database
            .load_thread(thread_id.clone())
            .await
            .unwrap()
            .expect("thread should exist");
        assert_eq!(loaded.title.as_ref(), "Photos");
        assert_eq!(loaded.messages, expected_messages);
        assert_eq!(loaded.detailed_summary.as_deref(), Some(summary));
        assert_eq!(loaded.cumulative_token_usage, expected_usage);
        assert_eq!(loaded.request_token_usage, expected_request_usage);
        assert_eq!(loaded.updated_at, updated_at);

        let entries = database.list_threads().await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, thread_id);
        assert_eq!(entries[0].title.as_ref(), "Photos");
        assert_eq!(entries[0].folder_paths, folder_paths);
        assert_eq!(entries[0].created_at, Some(updated_at));
    }

    #[gpui::test]
    async fn test_image_thread_revision_preserves_creation_and_other_sessions(
        cx: &mut TestAppContext,
    ) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();
        let thread_id = session_id("image-thread");
        let other_id = session_id("other-session");
        let created_at = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let revised_at = Utc.with_ymd_and_hms(2024, 6, 16, 8, 30, 0).unwrap();
        let original_folders = PathList::new(&[std::path::PathBuf::from("/home/user/project-a")]);
        let revised_folders = PathList::new(&[
            std::path::PathBuf::from("/home/user/project-a"),
            std::path::PathBuf::from("/home/user/project-b"),
        ]);
        let other_folders = PathList::new(&[std::path::PathBuf::from("/tmp/other-session")]);

        let original = image_bearing_thread(
            "Original title",
            created_at,
            "first café note — 日本語",
            USER_IMAGE,
            TOOL_IMAGE,
            "original summary",
            "original compaction",
        );
        database
            .save_thread(thread_id.clone(), original, original_folders)
            .await
            .unwrap();

        let revised = image_bearing_thread(
            "Revised title",
            revised_at,
            "revised café note — 日本語",
            REVISED_USER_IMAGE,
            REVISED_TOOL_IMAGE,
            "revised summary",
            "revised compaction",
        );
        let revised_messages = revised.messages.clone();
        let revised_usage = revised.cumulative_token_usage;
        database
            .save_thread(thread_id.clone(), revised, revised_folders.clone())
            .await
            .unwrap();

        let other = image_bearing_thread(
            "Other session",
            revised_at,
            "other session — 日本語",
            OTHER_USER_IMAGE,
            OTHER_TOOL_IMAGE,
            "other summary",
            "other compaction",
        );
        let other_messages = other.messages.clone();
        database
            .save_thread(other_id.clone(), other, other_folders.clone())
            .await
            .unwrap();

        let entries = database.list_threads().await.unwrap();
        let revised_entry = entries
            .iter()
            .find(|entry| entry.id == thread_id)
            .expect("revised thread metadata");
        assert_eq!(revised_entry.title.as_ref(), "Revised title");
        assert_eq!(revised_entry.updated_at, revised_at);
        assert_eq!(revised_entry.created_at, Some(created_at));
        assert_eq!(revised_entry.folder_paths, revised_folders);

        let other_entry = entries
            .iter()
            .find(|entry| entry.id == other_id)
            .expect("other session metadata");
        assert_eq!(other_entry.title.as_ref(), "Other session");
        assert_eq!(other_entry.folder_paths, other_folders);
        assert_eq!(other_entry.created_at, Some(revised_at));

        let loaded = database
            .load_thread(thread_id)
            .await
            .unwrap()
            .expect("revised thread should exist");
        assert_eq!(loaded.title.as_ref(), "Revised title");
        assert_eq!(loaded.messages, revised_messages);
        assert_eq!(loaded.detailed_summary.as_deref(), Some("revised summary"));
        assert_eq!(loaded.cumulative_token_usage, revised_usage);
        assert_eq!(loaded.updated_at, revised_at);
        let loaded_json = serde_json::to_string(&loaded.messages).unwrap();
        assert!(loaded_json.contains(REVISED_USER_IMAGE));
        assert!(loaded_json.contains(REVISED_TOOL_IMAGE));
        assert!(!loaded_json.contains(USER_IMAGE));
        assert!(!loaded_json.contains(OTHER_USER_IMAGE));

        let loaded_other = database
            .load_thread(other_id)
            .await
            .unwrap()
            .expect("other session should exist");
        assert_eq!(loaded_other.title.as_ref(), "Other session");
        assert_eq!(loaded_other.messages, other_messages);
        assert_eq!(
            loaded_other.detailed_summary.as_deref(),
            Some("other summary")
        );
    }

    #[gpui::test]
    async fn test_loader_reads_previous_zstd_and_uncompressed_json_rows(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2024, 3, 2, 4, 5, 6).unwrap();
        let updated_at_text = updated_at.to_rfc3339();

        let previous_writer_thread = image_bearing_thread(
            "Previous writer",
            updated_at,
            "previous writer café — 日本語",
            USER_IMAGE,
            TOOL_IMAGE,
            "previous summary",
            "previous compaction",
        );
        let previous_messages = previous_writer_thread.messages.clone();
        let previous_usage = previous_writer_thread.cumulative_token_usage;
        let previous_request_usage = previous_writer_thread.request_token_usage.clone();
        let previous_id = session_id("previous-zstd");
        let previous_json = versioned_thread_json(&previous_writer_thread);
        let previous_zstd = zstd::encode_all(previous_json.as_slice(), 3).expect("encode_all");
        insert_raw_thread(
            &database,
            &previous_id,
            "Previous writer",
            &updated_at_text,
            DataType::Zstd,
            previous_zstd,
        );

        let mut uncompressed_thread = make_thread("Uncompressed JSON", updated_at);
        uncompressed_thread.messages = vec![Arc::new(crate::Message::User(crate::UserMessage {
            id: acp_thread::ClientUserMessageId::new(),
            content: Arc::from([crate::UserMessageContent::Text(
                "plain json café — 日本語".into(),
            )]),
        }))];
        uncompressed_thread.detailed_summary = Some("plain summary".into());
        uncompressed_thread.cumulative_token_usage = language_model::TokenUsage {
            input_tokens: 9,
            output_tokens: 8,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 1,
        };
        let uncompressed_messages = uncompressed_thread.messages.clone();
        let uncompressed_usage = uncompressed_thread.cumulative_token_usage;
        let uncompressed_id = session_id("uncompressed-json");
        insert_raw_thread(
            &database,
            &uncompressed_id,
            "Uncompressed JSON",
            &updated_at_text,
            DataType::Json,
            versioned_thread_json(&uncompressed_thread),
        );

        let legacy = crate::legacy_thread::SerializedThread {
            version: crate::legacy_thread::SerializedThread::VERSION.to_string(),
            summary: "Legacy thread".into(),
            updated_at,
            messages: vec![crate::legacy_thread::SerializedMessage {
                id: crate::legacy_thread::MessageId(1),
                role: language_model::Role::User,
                segments: vec![crate::legacy_thread::SerializedMessageSegment::Text {
                    text: "legacy こんにちは".into(),
                }],
                tool_uses: vec![],
                tool_results: vec![],
                context: String::new(),
                creases: vec![],
                is_hidden: false,
            }],
            initial_project_snapshot: None,
            cumulative_token_usage: language_model::TokenUsage {
                input_tokens: 7,
                output_tokens: 8,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            },
            request_token_usage: vec![],
            detailed_summary_state: crate::legacy_thread::DetailedSummaryState::Generated {
                text: "legacy summary".into(),
            },
            model: None,
            tool_use_limit_reached: false,
            profile: None,
        };
        let legacy_json = serde_json::to_vec(&legacy).expect("serialize legacy thread");
        let legacy_zstd = zstd::encode_all(legacy_json.as_slice(), 3).expect("encode legacy");
        let legacy_id = session_id("legacy-zstd");
        insert_raw_thread(
            &database,
            &legacy_id,
            "Legacy thread",
            &updated_at_text,
            DataType::Zstd,
            legacy_zstd,
        );

        let loaded_previous = database
            .load_thread(previous_id)
            .await
            .unwrap()
            .expect("previous zstd row should load");
        assert_eq!(loaded_previous.title.as_ref(), "Previous writer");
        assert_eq!(loaded_previous.messages, previous_messages);
        assert_eq!(
            loaded_previous.detailed_summary.as_deref(),
            Some("previous summary")
        );
        assert_eq!(loaded_previous.cumulative_token_usage, previous_usage);
        assert_eq!(loaded_previous.request_token_usage, previous_request_usage);
        assert_eq!(loaded_previous.updated_at, updated_at);

        let loaded_plain = database
            .load_thread(uncompressed_id)
            .await
            .unwrap()
            .expect("uncompressed json row should load");
        assert_eq!(loaded_plain.title.as_ref(), "Uncompressed JSON");
        assert_eq!(loaded_plain.messages, uncompressed_messages);
        assert_eq!(
            loaded_plain.detailed_summary.as_deref(),
            Some("plain summary")
        );
        assert_eq!(loaded_plain.cumulative_token_usage, uncompressed_usage);

        let loaded_legacy = database
            .load_thread(legacy_id)
            .await
            .unwrap()
            .expect("legacy zstd row should load");
        assert_eq!(loaded_legacy.title.as_ref(), "Legacy thread");
        assert_eq!(
            loaded_legacy.detailed_summary.as_deref(),
            Some("legacy summary")
        );
        assert_eq!(loaded_legacy.cumulative_token_usage.input_tokens, 7);
        assert_eq!(loaded_legacy.cumulative_token_usage.output_tokens, 8);
        assert_eq!(loaded_legacy.updated_at, updated_at);
        let crate::Message::User(user_message) = loaded_legacy.messages[0].as_ref() else {
            panic!("legacy row should upgrade to a user message");
        };
        assert_eq!(
            user_message.content.as_ref(),
            [crate::UserMessageContent::Text("legacy こんにちは".into())]
        );
    }

    #[gpui::test]
    async fn test_failed_save_leaves_committed_thread_readable(cx: &mut TestAppContext) {
        let database = ThreadsDatabase::new(cx.executor()).unwrap();
        let thread_id = session_id("durable-thread");
        let created_at = Utc.with_ymd_and_hms(2024, 4, 1, 0, 0, 0).unwrap();
        let thread = image_bearing_thread(
            "Committed title",
            created_at,
            "committed café note — 日本語",
            USER_IMAGE,
            TOOL_IMAGE,
            "committed summary",
            "committed compaction",
        );
        let expected_messages = thread.messages.clone();
        let expected_usage = thread.cumulative_token_usage;
        let folder_paths = PathList::new(&[std::path::PathBuf::from("/home/user/project-a")]);

        database
            .save_thread(thread_id.clone(), thread, folder_paths.clone())
            .await
            .unwrap();
        let committed_row = stored_thread_row(&database, &thread_id);

        {
            let connection = database.connection.lock();
            connection
                .exec(indoc! {"
                    CREATE TRIGGER abort_thread_updates
                    BEFORE UPDATE ON threads
                    BEGIN
                        SELECT RAISE(ABORT, 'forced write failure');
                    END;
                "})
                .expect("prepare abort trigger")()
            .expect("install abort trigger");
        }

        let revised = image_bearing_thread(
            "Should not persist",
            Utc.with_ymd_and_hms(2024, 4, 2, 0, 0, 0).unwrap(),
            "replacement text",
            REVISED_USER_IMAGE,
            REVISED_TOOL_IMAGE,
            "replacement summary",
            "replacement compaction",
        );
        let save_result = database
            .save_thread(thread_id.clone(), revised, folder_paths)
            .await;
        assert!(
            save_result.is_err(),
            "aborting update trigger should fail the save, got {save_result:?}"
        );
        assert_eq!(stored_thread_row(&database, &thread_id), committed_row);

        let loaded = database
            .load_thread(thread_id.clone())
            .await
            .unwrap()
            .expect("committed thread should remain readable");
        assert_eq!(loaded.title.as_ref(), "Committed title");
        assert_eq!(loaded.messages, expected_messages);
        assert_eq!(
            loaded.detailed_summary.as_deref(),
            Some("committed summary")
        );
        assert_eq!(loaded.cumulative_token_usage, expected_usage);
        assert_eq!(loaded.updated_at, created_at);

        let entries = database.list_threads().await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, thread_id);
        assert_eq!(entries[0].title.as_ref(), "Committed title");
        assert_eq!(entries[0].created_at, Some(created_at));
        assert_eq!(entries[0].updated_at, created_at);
    }
}
