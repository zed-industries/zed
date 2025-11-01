use crate::{AgentMessage, AgentMessageContent, UserMessage, UserMessageContent};
use acp_thread::UserMessageId;
use agent_client_protocol as acp;
use agent_settings::{AgentProfileId, CompletionMode};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use collections::{HashMap, IndexMap};
use futures::{FutureExt, future::Shared};
use gpui::{BackgroundExecutor, Global, Task};
use indoc::indoc;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sqlez::{
    bindable::{Bind, Column},
    connection::Connection,
    statement::Statement,
};
use std::sync::Arc;
use ui::{App, SharedString};
use zed_env_vars::ZED_STATELESS;

pub type DbMessage = crate::Message;
pub type DbSummary = crate::legacy_thread::DetailedSummaryState;
pub type DbLanguageModel = crate::legacy_thread::SerializedLanguageModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbThreadMetadata {
    pub id: acp::SessionId,
    #[serde(alias = "summary")]
    pub title: SharedString,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbThread {
    pub title: SharedString,
    pub messages: Vec<DbMessage>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub detailed_summary: Option<SharedString>,
    #[serde(default)]
    pub initial_project_snapshot: Option<Arc<crate::ProjectSnapshot>>,
    #[serde(default)]
    pub cumulative_token_usage: language_model::TokenUsage,
    #[serde(default)]
    pub request_token_usage: HashMap<acp_thread::UserMessageId, language_model::TokenUsage>,
    #[serde(default)]
    pub model: Option<DbLanguageModel>,
    #[serde(default)]
    pub completion_mode: Option<CompletionMode>,
    #[serde(default)]
    pub profile: Option<AgentProfileId>,
}

impl DbThread {
    pub const VERSION: &'static str = "0.3.0";

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

                    let id = UserMessageId::new();
                    last_user_message_id = Some(id.clone());

                    crate::Message::User(UserMessage {
                        // MessageId from old format can't be meaningfully converted, so generate a new one
                        id,
                        content,
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
                                input: tool_use.input,
                                is_input_complete: true,
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
                                content: tool_result.content,
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
                    })
                }
                language_model::Role::System => {
                    // Skip system messages as they're not supported in the new format
                    continue;
                }
            };

            messages.push(message);
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
            completion_mode: thread.completion_mode,
            profile: thread.profile,
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

pub struct ThreadsDatabase {
    executor: BackgroundExecutor,
    connection: Arc<Mutex<Connection>>,
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
        .map_err(|e| anyhow!("Failed to create threads table: {}", e))?;

        // Create FTS5 virtual table for full text search
        connection.exec(indoc! {"
            CREATE VIRTUAL TABLE IF NOT EXISTS threads_fts USING fts5(
                id UNINDEXED,
                title,
                content
            )
        "})?()
        .map_err(|e| anyhow!("Failed to create threads_fts table: {}", e))?;

        let db = Self {
            executor,
            connection: Arc::new(Mutex::new(connection)),
        };

        Ok(db)
    }

    fn extract_text_content(thread: &DbThread) -> String {
        let mut content = String::new();

        for message in &thread.messages {
            match message {
                crate::Message::User(user_msg) => {
                    for msg_content in &user_msg.content {
                        match msg_content {
                            UserMessageContent::Text(text) => {
                                content.push_str(text);
                                content.push(' ');
                            }
                            UserMessageContent::Mention { .. } => {
                                // Skip mentions for full text search
                            }
                            UserMessageContent::Image(_) => {
                                // Skip images for full text search
                            }
                        }
                    }
                }
                crate::Message::Agent(agent_msg) => {
                    for msg_content in &agent_msg.content {
                        match msg_content {
                            AgentMessageContent::Text(text) => {
                                content.push_str(text);
                                content.push(' ');
                            }
                            AgentMessageContent::Thinking { text, .. } => {
                                content.push_str(text);
                                content.push(' ');
                            }
                            _ => {}
                        }
                    }
                }
                crate::Message::Resume => {
                    // Skip resume messages for full text search.
                    // Resume messages are a marker used to indicate the conversation should continue
                    // from where it left off. They are not real user content and should not be indexed.
                }
            }
        }

        content
    }

    fn save_thread_sync(
        connection: &Arc<Mutex<Connection>>,
        id: acp::SessionId,
        thread: DbThread,
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
        let text_content = Self::extract_text_content(&thread);
        let json_data = serde_json::to_string(&SerializedThread {
            thread,
            version: DbThread::VERSION,
        })?;

        let connection = connection.lock();

        // Begin explicit transaction for atomicity
        connection.exec("BEGIN IMMEDIATE")?()
            .map_err(|e| anyhow!("Failed to begin transaction: {}", e))?;

        // Helper to rollback on error
        let result = (|| -> Result<()> {
            let compressed = zstd::encode_all(json_data.as_bytes(), COMPRESSION_LEVEL)?;
            let data_type = DataType::Zstd;
            let data = compressed;

            let mut insert = connection.exec_bound::<(Arc<str>, String, String, DataType, Vec<u8>)>(indoc! {"
                INSERT OR REPLACE INTO threads (id, summary, updated_at, data_type, data) VALUES (?, ?, ?, ?, ?)
            "})?;

            insert((id.0.clone(), title.clone(), updated_at, data_type, data))?;

            // Update FTS5 index with extracted text content
            // FTS5 doesn't support traditional UPDATE statements for content changes
            // DELETE and INSERT is the recommended approach
            let mut delete_fts = connection.exec_bound::<Arc<str>>(indoc! {"
                DELETE FROM threads_fts WHERE id = ?
            "})?;
            delete_fts(id.0.clone())?;

            let mut insert_fts = connection.exec_bound::<(Arc<str>, String, String)>(indoc! {"
                INSERT INTO threads_fts (id, title, content) VALUES (?, ?, ?)
            "})?;
            insert_fts((id.0, title, text_content))?;

            Ok(())
        })();

        // Commit or rollback based on result
        match result {
            Ok(()) => {
                connection.exec("COMMIT")?()
                    .map_err(|e| anyhow!("Failed to commit transaction: {}", e))?;
                Ok(())
            }
            Err(e) => {
                // Attempt to rollback, but return the original error
                let _ = connection.exec("ROLLBACK")?();
                Err(e)
            }
        }
    }

    pub fn list_threads(&self) -> Task<Result<Vec<DbThreadMetadata>>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();

            let mut select =
                connection.select_bound::<(), (Arc<str>, String, String)>(indoc! {"
                SELECT id, summary, updated_at FROM threads ORDER BY updated_at DESC
            "})?;

            let rows = select(())?;
            let mut threads = Vec::new();

            for (id, summary, updated_at) in rows {
                threads.push(DbThreadMetadata {
                    id: acp::SessionId(id),
                    title: summary.into(),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
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
                let json_data = match data_type {
                    DataType::Zstd => {
                        let decompressed = zstd::decode_all(&data[..])?;
                        String::from_utf8(decompressed)?
                    }
                    DataType::Json => String::from_utf8(data)?,
                };
                let thread = DbThread::from_json(json_data.as_bytes())?;
                Ok(Some(thread))
            } else {
                Ok(None)
            }
        })
    }

    pub fn save_thread(&self, id: acp::SessionId, thread: DbThread) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor
            .spawn(async move { Self::save_thread_sync(&connection, id, thread) })
    }

    pub fn delete_thread(&self, id: acp::SessionId) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();

            // Delete from main table
            let mut delete = connection.exec_bound::<Arc<str>>(indoc! {"
                DELETE FROM threads WHERE id = ?
            "})?;
            delete(id.0.clone())?;

            // Delete from FTS5 index
            let mut delete_fts = connection.exec_bound::<Arc<str>>(indoc! {"
                DELETE FROM threads_fts WHERE id = ?
            "})?;
            delete_fts(id.0)?;

            Ok(())
        })
    }

    pub fn search_threads(&self, query: String) -> Task<Result<Vec<DbThreadMetadata>>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();

            // Use FTS5 MATCH query to search both title and content
            let mut select =
                connection.select_bound::<String, (Arc<str>, String, String)>(indoc! {"
                SELECT t.id, t.summary, t.updated_at
                FROM threads t
                INNER JOIN threads_fts fts ON t.id = fts.id
                WHERE threads_fts MATCH ?
                ORDER BY rank, t.updated_at DESC
            "})?;

            let rows = select(query)?;
            let mut threads = Vec::new();

            for (id, summary, updated_at) in rows {
                threads.push(DbThreadMetadata {
                    id: acp::SessionId(id),
                    title: summary.into(),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
                });
            }

            Ok(threads)
        })
    }

    /// Rebuilds the full text search index for all threads.
    /// This is useful for migrating existing threads after enabling FTS.
    pub fn rebuild_search_index(&self) -> Task<Result<usize>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            // Get all threads
            let rows = {
                let connection_guard = connection.lock();
                let mut select = connection_guard
                    .select_bound::<(), (Arc<str>, DataType, Vec<u8>)>(indoc! {"
                    SELECT id, data_type, data FROM threads
                "})?;
                select(())?
            };

            let mut indexed_count = 0;

            for (id, data_type, data) in rows {
                // Decompress and parse thread
                let json_data = match data_type {
                    DataType::Zstd => {
                        let decompressed = zstd::decode_all(&data[..])?;
                        String::from_utf8(decompressed)?
                    }
                    DataType::Json => String::from_utf8(data)?,
                };

                let thread = DbThread::from_json(json_data.as_bytes())?;
                let title = thread.title.to_string();
                let text_content = Self::extract_text_content(&thread);

                // Update FTS index
                {
                    let connection_guard = connection.lock();

                    let mut delete_fts = connection_guard.exec_bound::<Arc<str>>(indoc! {"
                        DELETE FROM threads_fts WHERE id = ?
                    "})?;
                    delete_fts(id.clone())?;

                    let mut insert_fts = connection_guard
                        .exec_bound::<(Arc<str>, String, String)>(indoc! {"
                        INSERT INTO threads_fts (id, title, content) VALUES (?, ?, ?)
                    "})?;
                    insert_fts((id, title, text_content))?;
                }

                indexed_count += 1;
            }

            Ok(indexed_count)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_full_text_search(cx: &mut TestAppContext) {
        let db = ThreadsDatabase::new(cx.executor()).unwrap();

        // Create test threads with different content
        let thread1 = DbThread {
            title: "First Thread".into(),
            messages: vec![
                crate::Message::User(UserMessage {
                    id: UserMessageId::new(),
                    content: vec![UserMessageContent::Text(
                        "How do I implement Rust async?".into(),
                    )],
                }),
                crate::Message::Agent(AgentMessage {
                    content: vec![AgentMessageContent::Text(
                        "To implement async in Rust, you use async/await syntax.".into(),
                    )],
                    tool_results: IndexMap::default(),
                }),
            ],
            updated_at: Utc::now(),
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: HashMap::default(),
            model: None,
            completion_mode: None,
            profile: None,
        };

        let thread2 = DbThread {
            title: "Second Thread".into(),
            messages: vec![
                crate::Message::User(UserMessage {
                    id: UserMessageId::new(),
                    content: vec![UserMessageContent::Text(
                        "What are the best Python frameworks?".into(),
                    )],
                }),
                crate::Message::Agent(AgentMessage {
                    content: vec![AgentMessageContent::Text(
                        "Django and Flask are popular Python web frameworks.".into(),
                    )],
                    tool_results: IndexMap::default(),
                }),
            ],
            updated_at: Utc::now(),
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: HashMap::default(),
            model: None,
            completion_mode: None,
            profile: None,
        };

        // Save threads
        let id1 = acp::SessionId(Arc::from("test-thread-1"));
        let id2 = acp::SessionId(Arc::from("test-thread-2"));

        db.save_thread(id1.clone(), thread1).await.unwrap();
        db.save_thread(id2.clone(), thread2).await.unwrap();

        // Search for "Rust" - should find thread1
        let results = db.search_threads("Rust".to_string()).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, id1);

        // Search for "Python" - should find thread2
        let results = db.search_threads("Python".to_string()).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, id2);

        // Search for "frameworks" - should find thread2
        let results = db.search_threads("frameworks".to_string()).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, id2);

        // Search for "async" - should find thread1 (searching content, not just title)
        let results = db.search_threads("async".to_string()).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, id1);

        // Clean up
        db.delete_thread(id1).await.unwrap();
        db.delete_thread(id2).await.unwrap();
    }

    #[gpui::test]
    async fn test_rebuild_search_index(cx: &mut TestAppContext) {
        let db = ThreadsDatabase::new(cx.executor()).unwrap();

        // Create a thread
        let thread = DbThread {
            title: "Migration Test".into(),
            messages: vec![
                crate::Message::User(UserMessage {
                    id: UserMessageId::new(),
                    content: vec![UserMessageContent::Text(
                        "This thread needs to be indexed".into(),
                    )],
                }),
                crate::Message::Agent(AgentMessage {
                    content: vec![AgentMessageContent::Text(
                        "Response with unique keyword xyzabc123".into(),
                    )],
                    tool_results: IndexMap::default(),
                }),
            ],
            updated_at: Utc::now(),
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: HashMap::default(),
            model: None,
            completion_mode: None,
            profile: None,
        };

        let id = acp::SessionId(Arc::from("migration-test-thread"));

        // Save the thread (this will index it)
        db.save_thread(id.clone(), thread).await.unwrap();

        // Manually clear the FTS index to simulate old data
        {
            let connection = db.connection.lock();
            let mut delete = connection
                .exec_bound::<Arc<str>>(indoc! {"
                DELETE FROM threads_fts WHERE id = ?
            "})
                .unwrap();
            delete(id.0.clone()).unwrap();
        }

        // Verify thread is not searchable
        let results = db.search_threads("xyzabc123".to_string()).await.unwrap();
        assert_eq!(
            results.len(),
            0,
            "Thread should not be found before rebuild"
        );

        // Rebuild the index
        let count = db.rebuild_search_index().await.unwrap();
        assert_eq!(count, 1, "Should have indexed 1 thread");

        // Verify thread is now searchable
        let results = db.search_threads("xyzabc123".to_string()).await.unwrap();
        assert_eq!(results.len(), 1, "Thread should be found after rebuild");
        assert_eq!(results[0].id, id);

        // Clean up
        db.delete_thread(id).await.unwrap();
    }
}
