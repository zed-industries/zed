use crate::{AgentMessage, AgentMessageContent, UserMessage, UserMessageContent};
use acp_thread::UserMessageId;
use agent::{thread::DetailedSummaryState, thread_store};
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

pub type DbMessage = crate::Message;
pub type DbSummary = DetailedSummaryState;
pub type DbLanguageModel = thread_store::SerializedLanguageModel;

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
    pub initial_project_snapshot: Option<Arc<agent::thread::ProjectSnapshot>>,
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
                _ => Self::upgrade_from_agent_1(agent::SerializedThread::from_json(json)?),
            },
            _ => Self::upgrade_from_agent_1(agent::SerializedThread::from_json(json)?),
        }
    }

    fn upgrade_from_agent_1(thread: agent::SerializedThread) -> Result<Self> {
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
                            thread_store::SerializedMessageSegment::Text { text } => {
                                content.push(UserMessageContent::Text(text));
                            }
                            thread_store::SerializedMessageSegment::Thinking { text, .. } => {
                                // User messages don't have thinking segments, but handle gracefully
                                content.push(UserMessageContent::Text(text));
                            }
                            thread_store::SerializedMessageSegment::RedactedThinking { .. } => {
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
                            thread_store::SerializedMessageSegment::Text { text } => {
                                content.push(AgentMessageContent::Text(text));
                            }
                            thread_store::SerializedMessageSegment::Thinking {
                                text,
                                signature,
                            } => {
                                content.push(AgentMessageContent::Thinking { text, signature });
                            }
                            thread_store::SerializedMessageSegment::RedactedThinking { data } => {
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
                DetailedSummaryState::NotGenerated | DetailedSummaryState::Generating { .. } => {
                    None
                }
                DetailedSummaryState::Generated { text, .. } => Some(text),
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

pub static ZED_STATELESS: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| std::env::var("ZED_STATELESS").is_ok_and(|v| !v.is_empty()));

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
        let connection = if *ZED_STATELESS || cfg!(any(feature = "test-support", test)) {
            Connection::open_memory(Some("THREAD_FALLBACK_DB"))
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

        let mut insert = connection.exec_bound::<(Arc<str>, String, String, DataType, Vec<u8>)>(indoc! {"
            INSERT OR REPLACE INTO threads (id, summary, updated_at, data_type, data) VALUES (?, ?, ?, ?, ?)
        "})?;

        insert((id.0, title, updated_at, data_type, data))?;

        Ok(())
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

            let mut delete = connection.exec_bound::<Arc<str>>(indoc! {"
                DELETE FROM threads WHERE id = ?
            "})?;

            delete(id.0)?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use agent::MessageSegment;
    use agent::context::LoadedContext;
    use client::Client;
    use fs::FakeFs;
    use gpui::AppContext;
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use language_model::Role;
    use project::Project;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);

            let http_client = FakeHttpClient::with_404_response();
            let clock = Arc::new(clock::FakeSystemClock::new());
            let client = Client::new(clock, http_client, cx);
            agent::init(cx);
            agent_settings::init(cx);
            language_model::init(client, cx);
        });
    }

    #[gpui::test]
    async fn test_retrieving_old_thread(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        // Save a thread using the old agent.
        let thread_store = cx.new(|cx| agent::ThreadStore::fake(project, cx));
        let thread = thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx));
        thread.update(cx, |thread, cx| {
            thread.insert_message(
                Role::User,
                vec![MessageSegment::Text("Hey!".into())],
                LoadedContext::default(),
                vec![],
                false,
                cx,
            );
            thread.insert_message(
                Role::Assistant,
                vec![MessageSegment::Text("How're you doing?".into())],
                LoadedContext::default(),
                vec![],
                false,
                cx,
            )
        });
        thread_store
            .update(cx, |thread_store, cx| thread_store.save_thread(&thread, cx))
            .await
            .unwrap();

        // Open that same thread using the new agent.
        let db = cx.update(ThreadsDatabase::connect).await.unwrap();
        let threads = db.list_threads().await.unwrap();
        assert_eq!(threads.len(), 1);
        let thread = db
            .load_thread(threads[0].id.clone())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(thread.messages[0].to_markdown(), "## User\n\nHey!\n");
        assert_eq!(
            thread.messages[1].to_markdown(),
            "## Assistant\n\nHow're you doing?\n"
        );
    }
}
