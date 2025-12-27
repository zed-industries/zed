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
    domain::Domain,
    statement::Statement,
};
use sqlez_macros::sql;
use std::sync::Arc;
use ui::{App, SharedString};
use zed_env_vars::ZED_STATELESS;

pub type DbMessage = crate::Message;
pub type DbSummary = crate::legacy_thread::DetailedSummaryState;
pub type DbLanguageModel = crate::legacy_thread::SerializedLanguageModel;

/// Identifier for different agent types (Zed Native Agent vs external agents)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AgentIdentity {
    Zed,
    External(SharedString),
}

impl AgentIdentity {
    /// Default agent name for Zed's native agent
    pub const ZED_AGENT_NAME: &'static str = "zed";

    /// Create an AgentIdentity from an agent name string
    pub fn from_name(name: &str) -> Self {
        if name == Self::ZED_AGENT_NAME {
            AgentIdentity::Zed
        } else {
            AgentIdentity::External(name.to_string().into())
        }
    }

    /// Get the string representation of this agent identity
    pub fn name(&self) -> SharedString {
        match self {
            AgentIdentity::Zed => SharedString::from(Self::ZED_AGENT_NAME),
            AgentIdentity::External(name) => name.clone(),
        }
    }

    /// Get a human-readable display name for this agent
    pub fn display_name(&self) -> SharedString {
        match self {
            AgentIdentity::Zed => SharedString::from("Zed"),
            AgentIdentity::External(name) => name.clone(),
        }
    }

    /// Check if this is the Zed native agent
    pub fn is_zed(&self) -> bool {
        matches!(self, AgentIdentity::Zed)
    }
}

impl From<AgentIdentity> for SharedString {
    fn from(identity: AgentIdentity) -> Self {
        identity.name()
    }
}

impl<'a> From<&'a AgentIdentity> for SharedString {
    fn from(identity: &'a AgentIdentity) -> Self {
        identity.name()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbThreadMetadata {
    pub id: acp::SessionId,
    #[serde(alias = "summary")]
    pub title: SharedString,
    pub updated_at: DateTime<Utc>,
    pub agent_name: SharedString,
    pub agent_version: Option<SharedString>,
    pub agent_provider_id: Option<SharedString>,
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
                        reasoning_details: None,
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

pub(crate) struct ThreadsDatabase {
    executor: BackgroundExecutor,
    connection: Arc<Mutex<Connection>>,
}

struct GlobalThreadsDatabase(Shared<Task<Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>>);

impl Global for GlobalThreadsDatabase {}

impl Domain for ThreadsDatabase {
    const NAME: &str = stringify!(ThreadsDatabase);

    const MIGRATIONS: &[&str] = &[sql!(
        ALTER TABLE threads ADD COLUMN agent_name TEXT NOT NULL DEFAULT "zed";
        ALTER TABLE threads ADD COLUMN agent_version TEXT;
        ALTER TABLE threads ADD COLUMN agent_provider_id TEXT;
    )];
}

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

        // Check if the migration was already applied outside the framework
        // (e.g., during development or a previous build that added columns directly)
        let agent_name_column_exists = {
            let table_info = connection.select::<String>(
                "SELECT name FROM pragma_table_info('threads') WHERE name = 'agent_name'",
            );
            match table_info {
                Ok(mut query) => query().map(|rows| !rows.is_empty()).unwrap_or(false),
                Err(_) => false,
            }
        };

        // Create migrations table if needed and check if migration is already tracked
        connection.exec(indoc! {"
            CREATE TABLE IF NOT EXISTS migrations (
                domain TEXT,
                step INTEGER,
                migration TEXT
            )"})?()
        .map_err(|e| anyhow!("Failed to create migrations table: {}", e))?;

        // Check if the migration is already tracked
        let migration_tracked = {
            let result = connection.select_bound::<(&str, usize), String>(
                "SELECT migration FROM migrations WHERE domain = ? AND step = ?",
            );
            match result {
                Ok(mut query) => query((<ThreadsDatabase as Domain>::NAME, 0))
                    .map(|rows| !rows.is_empty())
                    .unwrap_or(false),
                Err(_) => false,
            }
        };

        // If columns exist but migration isn't tracked, manually record it as complete
        // Store the raw migration SQL - the migrate() function will format it when comparing
        if agent_name_column_exists && !migration_tracked {
            let migration_sql = <ThreadsDatabase as Domain>::MIGRATIONS[0].to_string();
            connection.exec_bound::<(&str, usize, String)>(
                "INSERT INTO migrations (domain, step, migration) VALUES (?, ?, ?)",
            )?((<ThreadsDatabase as Domain>::NAME, 0, migration_sql))
            .map_err(|e| anyhow!("Failed to record migration: {}", e))?;
        }

        // Run migrations using sqlez Domain pattern
        connection.migrate(
            <ThreadsDatabase as Domain>::NAME,
            <ThreadsDatabase as Domain>::MIGRATIONS,
            |_, _, _| false,
        )?;

        let db = Self {
            executor,
            connection: Arc::new(Mutex::new(connection)),
        };

        Ok(db)
    }

    fn save_thread_sync(
        connection: &Arc<Mutex<Connection>>,
        id: acp::SessionId,
        thread: DbThread,
        agent_name: &str,
        agent_version: Option<&str>,
        agent_provider_id: Option<&str>,
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
        let json_data = serde_json::to_string(&SerializedThread {
            thread,
            version: DbThread::VERSION,
        })?;

        let connection = connection.lock();

        let compressed = zstd::encode_all(json_data.as_bytes(), COMPRESSION_LEVEL)?;
        let data_type = DataType::Zstd;
        let data = compressed;

        let mut insert = connection.exec_bound::<(Arc<str>, String, String, DataType, Vec<u8>, String, Option<String>, Option<String>)>(indoc! {"
            INSERT OR REPLACE INTO threads (id, summary, updated_at, data_type, data, agent_name, agent_version, agent_provider_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "})?;

        insert((
            id.0,
            title,
            updated_at,
            data_type,
            data,
            agent_name.to_string(),
            agent_version.map(|v| v.to_string()),
            agent_provider_id.map(|p| p.to_string()),
        ))?;

        Ok(())
    }

    pub fn list_threads(&self) -> Task<Result<Vec<DbThreadMetadata>>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();

            let mut select =
                connection.select_bound::<(), (Arc<str>, String, String, String, Option<String>, Option<String>)>(indoc! {"
                SELECT id, summary, updated_at, agent_name, agent_version, agent_provider_id FROM threads ORDER BY updated_at DESC
            "})?;

            let rows = select(())?;
            let mut threads = Vec::new();

            for (id, summary, updated_at, agent_name, agent_version, agent_provider_id) in rows {
                threads.push(DbThreadMetadata {
                    id: acp::SessionId::new(id),
                    title: summary.into(),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
                    agent_name: agent_name.into(),
                    agent_version: agent_version.map(|v| v.into()),
                    agent_provider_id: agent_provider_id.map(|p| p.into()),
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

    pub fn save_thread(
        &self,
        id: acp::SessionId,
        thread: DbThread,
        agent_name: &str,
        agent_version: Option<&str>,
        agent_provider_id: Option<&str>,
    ) -> Task<Result<()>> {
        let connection = self.connection.clone();
        let agent_name = agent_name.to_string();
        let agent_version = agent_version.map(|v| v.to_string());
        let agent_provider_id = agent_provider_id.map(|p| p.to_string());

        self.executor.spawn(async move {
            Self::save_thread_sync(
                &connection,
                id,
                thread,
                &agent_name,
                agent_version.as_deref(),
                agent_provider_id.as_deref(),
            )
        })
    }

    pub fn delete_thread(&self, id: acp::SessionId) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();

            let mut delete = connection.exec_bound::<Arc<str>>(indoc! {"
                DELETE FROM threads WHERE id = ?
            "})?;

            delete(id.0)?;

            Ok(())
        })
    }

    pub fn delete_threads(&self) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock();

            let mut delete = connection.exec_bound::<()>(indoc! {"
                DELETE FROM threads
            "})?;

            delete(())?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_identity_from_name() {
        // Test Zed agent
        let zed_identity = AgentIdentity::from_name("zed");
        assert!(zed_identity.is_zed());
        assert_eq!(zed_identity.name(), "zed");
        assert_eq!(zed_identity.display_name(), "Zed");

        // Test external agents
        let claude_identity = AgentIdentity::from_name("claude-code");
        assert!(!claude_identity.is_zed());
        assert_eq!(claude_identity.name(), "claude-code");
        assert_eq!(claude_identity.display_name(), "claude-code");

        let codex_identity = AgentIdentity::from_name("codex");
        assert!(!codex_identity.is_zed());
        assert_eq!(codex_identity.name(), "codex");
    }

    #[test]
    fn test_migrations() {
        use super::*;

        let connection = Connection::open_memory(Some("test_migration_entry"));

        connection
            .exec(indoc! {"
                CREATE TABLE threads (
                    id TEXT PRIMARY KEY,
                    summary TEXT NOT NULL
                );
            "})
            .unwrap()()
        .unwrap();

        connection
            .migrate(
                <ThreadsDatabase as Domain>::NAME,
                <ThreadsDatabase as Domain>::MIGRATIONS,
                |_, _, _| false,
            )
            .unwrap();

        let migration_sql: Option<(usize, String)> =
            connection
                .select_bound::<&str, (usize, String)>(
                    "SELECT step, migration FROM migrations WHERE domain = ?",
                )
                .unwrap()(<ThreadsDatabase as Domain>::NAME)
            .unwrap()
            .into_iter()
            .next();

        assert!(
            migration_sql.is_some(),
            "Migration should be stored in migrations table"
        );

        let (step, sql) = migration_sql.unwrap();
        assert_eq!(step, 0, "First migration should have step 0");
        assert!(sql.contains("agent_name"), "SQL: {}", sql);
        assert!(sql.contains("agent_version"), "SQL: {}", sql);
        assert!(sql.contains("agent_provider_id"), "SQL: {}", sql);
    }
}
