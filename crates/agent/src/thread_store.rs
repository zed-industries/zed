use crate::{MessageId, ProjectSnapshot};
use agent_settings::{AgentProfileId, CompletionMode};
use anyhow::{Result, anyhow};
use assistant_tool::ToolWorkingSet;
use chrono::{DateTime, Utc};
use fs::{Fs, RemoveOptions};
use futures::{
    FutureExt as _,
    future::{self, BoxFuture, Shared},
};
use gpui::{
    App, BackgroundExecutor, Context, Entity, EventEmitter, Global, ReadGlobal, SharedString, Task,
};
use indoc::indoc;
use language_model::{LanguageModelToolResultContent, LanguageModelToolUseId, Role, TokenUsage};
use prompt_store::{ProjectContext, PromptStore};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlez::{
    bindable::{Bind, Column},
    connection::Connection,
    statement::Statement,
};
use std::{
    cell::{Ref, RefCell},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use zed_env_vars::ZED_STATELESS;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize, JsonSchema,
)]
pub struct ThreadId(Arc<str>);

impl ThreadId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().into())
    }
}

impl std::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ThreadId {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum DetailedSummaryState {
    #[default]
    NotGenerated,
    Generating,
    Generated {
        text: SharedString,
    },
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

pub fn init(fs: Arc<dyn Fs>, cx: &mut App) {
    ThreadsDatabase::init(fs, cx);
}

/// A system prompt shared by all threads created by this ThreadStore
#[derive(Clone, Default)]
pub struct SharedProjectContext(Rc<RefCell<Option<ProjectContext>>>);

impl SharedProjectContext {
    pub fn borrow(&self) -> Ref<'_, Option<ProjectContext>> {
        self.0.borrow()
    }
}

pub type TextThreadStore = assistant_context::ContextStore;

pub struct ThreadStore {
    tools: Entity<ToolWorkingSet>,
    prompt_store: Option<Entity<PromptStore>>,
    threads: Vec<SerializedThreadMetadata>,
}

pub struct RulesLoadingError {
    pub message: SharedString,
}

impl EventEmitter<RulesLoadingError> for ThreadStore {}

impl ThreadStore {
    pub fn new(
        tools: Entity<ToolWorkingSet>,
        prompt_store: Option<Entity<PromptStore>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let this = Self {
            tools,
            prompt_store,
            threads: Vec::new(),
        };
        this.reload(cx).detach_and_log_err(cx);
        this
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(cx: &mut App) -> Self {
        Self {
            tools: cx.new(|_| ToolWorkingSet::default()),
            prompt_store: None,
            threads: Vec::new(),
        }
    }

    pub fn prompt_store(&self) -> &Option<Entity<PromptStore>> {
        &self.prompt_store
    }

    pub fn tools(&self) -> Entity<ToolWorkingSet> {
        self.tools.clone()
    }

    pub fn reload(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(async move |this, cx| {
            let threads = database_future
                .await
                .map_err(|err| anyhow!(err))?
                .list_threads()
                .await?;

            this.update(cx, |this, cx| {
                this.threads = threads;
                cx.notify();
            })
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedThreadMetadata {
    pub id: ThreadId,
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SerializedThread {
    pub version: String,
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<SerializedMessage>,
    #[serde(default)]
    pub initial_project_snapshot: Option<Arc<ProjectSnapshot>>,
    #[serde(default)]
    pub cumulative_token_usage: TokenUsage,
    #[serde(default)]
    pub request_token_usage: Vec<TokenUsage>,
    #[serde(default)]
    pub detailed_summary_state: DetailedSummaryState,
    #[serde(default)]
    pub model: Option<SerializedLanguageModel>,
    #[serde(default)]
    pub completion_mode: Option<CompletionMode>,
    #[serde(default)]
    pub tool_use_limit_reached: bool,
    #[serde(default)]
    pub profile: Option<AgentProfileId>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SerializedLanguageModel {
    pub provider: String,
    pub model: String,
}

impl SerializedThread {
    pub const VERSION: &'static str = "0.2.0";

    pub fn from_json(json: &[u8]) -> Result<Self> {
        let saved_thread_json = serde_json::from_slice::<serde_json::Value>(json)?;
        match saved_thread_json.get("version") {
            Some(serde_json::Value::String(version)) => match version.as_str() {
                SerializedThreadV0_1_0::VERSION => {
                    let saved_thread =
                        serde_json::from_value::<SerializedThreadV0_1_0>(saved_thread_json)?;
                    Ok(saved_thread.upgrade())
                }
                SerializedThread::VERSION => Ok(serde_json::from_value::<SerializedThread>(
                    saved_thread_json,
                )?),
                _ => anyhow::bail!("unrecognized serialized thread version: {version:?}"),
            },
            None => {
                let saved_thread =
                    serde_json::from_value::<LegacySerializedThread>(saved_thread_json)?;
                Ok(saved_thread.upgrade())
            }
            version => anyhow::bail!("unrecognized serialized thread version: {version:?}"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializedThreadV0_1_0(
    // The structure did not change, so we are reusing the latest SerializedThread.
    // When making the next version, make sure this points to SerializedThreadV0_2_0
    SerializedThread,
);

impl SerializedThreadV0_1_0 {
    pub const VERSION: &'static str = "0.1.0";

    pub fn upgrade(self) -> SerializedThread {
        debug_assert_eq!(SerializedThread::VERSION, "0.2.0");

        let mut messages: Vec<SerializedMessage> = Vec::with_capacity(self.0.messages.len());

        for message in self.0.messages {
            if message.role == Role::User
                && !message.tool_results.is_empty()
                && let Some(last_message) = messages.last_mut()
            {
                debug_assert!(last_message.role == Role::Assistant);

                last_message.tool_results = message.tool_results;
                continue;
            }

            messages.push(message);
        }

        SerializedThread {
            messages,
            version: SerializedThread::VERSION.to_string(),
            ..self.0
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedMessage {
    pub id: MessageId,
    pub role: Role,
    #[serde(default)]
    pub segments: Vec<SerializedMessageSegment>,
    #[serde(default)]
    pub tool_uses: Vec<SerializedToolUse>,
    #[serde(default)]
    pub tool_results: Vec<SerializedToolResult>,
    #[serde(default)]
    pub context: String,
    #[serde(default)]
    pub creases: Vec<SerializedCrease>,
    #[serde(default)]
    pub is_hidden: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SerializedMessageSegment {
    #[serde(rename = "text")]
    Text {
        text: String,
    },
    #[serde(rename = "thinking")]
    Thinking {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
    },
    RedactedThinking {
        data: String,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedToolUse {
    pub id: LanguageModelToolUseId,
    pub name: SharedString,
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedToolResult {
    pub tool_use_id: LanguageModelToolUseId,
    pub is_error: bool,
    pub content: LanguageModelToolResultContent,
    pub output: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct LegacySerializedThread {
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<LegacySerializedMessage>,
    #[serde(default)]
    pub initial_project_snapshot: Option<Arc<ProjectSnapshot>>,
}

impl LegacySerializedThread {
    pub fn upgrade(self) -> SerializedThread {
        SerializedThread {
            version: SerializedThread::VERSION.to_string(),
            summary: self.summary,
            updated_at: self.updated_at,
            messages: self.messages.into_iter().map(|msg| msg.upgrade()).collect(),
            initial_project_snapshot: self.initial_project_snapshot,
            cumulative_token_usage: TokenUsage::default(),
            request_token_usage: Vec::new(),
            detailed_summary_state: DetailedSummaryState::default(),
            model: None,
            completion_mode: None,
            tool_use_limit_reached: false,
            profile: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LegacySerializedMessage {
    pub id: MessageId,
    pub role: Role,
    pub text: String,
    #[serde(default)]
    pub tool_uses: Vec<SerializedToolUse>,
    #[serde(default)]
    pub tool_results: Vec<SerializedToolResult>,
}

impl LegacySerializedMessage {
    fn upgrade(self) -> SerializedMessage {
        SerializedMessage {
            id: self.id,
            role: self.role,
            segments: vec![SerializedMessageSegment::Text { text: self.text }],
            tool_uses: self.tool_uses,
            tool_results: self.tool_results,
            context: String::new(),
            creases: Vec::new(),
            is_hidden: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedCrease {
    pub start: usize,
    pub end: usize,
    pub icon_path: SharedString,
    pub label: SharedString,
}

struct GlobalThreadsDatabase(
    Shared<BoxFuture<'static, Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>>,
);

impl Global for GlobalThreadsDatabase {}

pub(crate) struct ThreadsDatabase {
    executor: BackgroundExecutor,
    connection: Arc<Mutex<Connection>>,
}

impl ThreadsDatabase {
    fn connection(&self) -> Arc<Mutex<Connection>> {
        self.connection.clone()
    }

    const COMPRESSION_LEVEL: i32 = 3;
}

impl Bind for ThreadId {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        self.to_string().bind(statement, start_index)
    }
}

impl Column for ThreadId {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (id_str, next_index) = String::column(statement, start_index)?;
        Ok((ThreadId::from(id_str.as_str()), next_index))
    }
}

impl ThreadsDatabase {
    fn global_future(
        cx: &mut App,
    ) -> Shared<BoxFuture<'static, Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>> {
        GlobalThreadsDatabase::global(cx).0.clone()
    }

    fn init(fs: Arc<dyn Fs>, cx: &mut App) {
        let executor = cx.background_executor().clone();
        let database_future = executor
            .spawn({
                let executor = executor.clone();
                let threads_dir = paths::data_dir().join("threads");
                async move { ThreadsDatabase::new(fs, threads_dir, executor).await }
            })
            .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
            .boxed()
            .shared();

        cx.set_global(GlobalThreadsDatabase(database_future));
    }

    pub async fn new(
        fs: Arc<dyn Fs>,
        threads_dir: PathBuf,
        executor: BackgroundExecutor,
    ) -> Result<Self> {
        fs.create_dir(&threads_dir).await?;

        let sqlite_path = threads_dir.join("threads.db");
        let mdb_path = threads_dir.join("threads-db.1.mdb");

        let needs_migration_from_heed = fs.is_file(&mdb_path).await;

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
            executor: executor.clone(),
            connection: Arc::new(Mutex::new(connection)),
        };

        if needs_migration_from_heed {
            let db_connection = db.connection();
            let executor_clone = executor.clone();
            executor
                .spawn(async move {
                    log::info!("Starting threads.db migration");
                    Self::migrate_from_heed(&mdb_path, db_connection, executor_clone)?;
                    fs.remove_dir(
                        &mdb_path,
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await?;
                    log::info!("threads.db migrated to sqlite");
                    Ok::<(), anyhow::Error>(())
                })
                .detach();
        }

        Ok(db)
    }

    // Remove this migration after 2025-09-01
    fn migrate_from_heed(
        mdb_path: &Path,
        connection: Arc<Mutex<Connection>>,
        _executor: BackgroundExecutor,
    ) -> Result<()> {
        use heed::types::SerdeBincode;
        struct SerializedThreadHeed(SerializedThread);

        impl heed::BytesEncode<'_> for SerializedThreadHeed {
            type EItem = SerializedThreadHeed;

            fn bytes_encode(
                item: &Self::EItem,
            ) -> Result<std::borrow::Cow<'_, [u8]>, heed::BoxedError> {
                serde_json::to_vec(&item.0)
                    .map(std::borrow::Cow::Owned)
                    .map_err(Into::into)
            }
        }

        impl<'a> heed::BytesDecode<'a> for SerializedThreadHeed {
            type DItem = SerializedThreadHeed;

            fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
                SerializedThread::from_json(bytes)
                    .map(SerializedThreadHeed)
                    .map_err(Into::into)
            }
        }

        const ONE_GB_IN_BYTES: usize = 1024 * 1024 * 1024;

        let env = unsafe {
            heed::EnvOpenOptions::new()
                .map_size(ONE_GB_IN_BYTES)
                .max_dbs(1)
                .open(mdb_path)?
        };

        let txn = env.write_txn()?;
        let threads: heed::Database<SerdeBincode<ThreadId>, SerializedThreadHeed> = env
            .open_database(&txn, Some("threads"))?
            .ok_or_else(|| anyhow!("threads database not found"))?;

        for result in threads.iter(&txn)? {
            let (thread_id, thread_heed) = result?;
            Self::save_thread_sync(&connection, thread_id, thread_heed.0)?;
        }

        Ok(())
    }

    fn save_thread_sync(
        connection: &Arc<Mutex<Connection>>,
        id: ThreadId,
        thread: SerializedThread,
    ) -> Result<()> {
        let json_data = serde_json::to_string(&thread)?;
        let summary = thread.summary.to_string();
        let updated_at = thread.updated_at.to_rfc3339();

        let connection = connection.lock().unwrap();

        let compressed = zstd::encode_all(json_data.as_bytes(), Self::COMPRESSION_LEVEL)?;
        let data_type = DataType::Zstd;
        let data = compressed;

        let mut insert = connection.exec_bound::<(ThreadId, String, String, DataType, Vec<u8>)>(indoc! {"
            INSERT OR REPLACE INTO threads (id, summary, updated_at, data_type, data) VALUES (?, ?, ?, ?, ?)
        "})?;

        insert((id, summary, updated_at, data_type, data))?;

        Ok(())
    }

    pub fn list_threads(&self) -> Task<Result<Vec<SerializedThreadMetadata>>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock().unwrap();
            let mut select =
                connection.select_bound::<(), (ThreadId, String, String)>(indoc! {"
                SELECT id, summary, updated_at FROM threads ORDER BY updated_at DESC
            "})?;

            let rows = select(())?;
            let mut threads = Vec::new();

            for (id, summary, updated_at) in rows {
                threads.push(SerializedThreadMetadata {
                    id,
                    summary: summary.into(),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
                });
            }

            Ok(threads)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use language_model::{Role, TokenUsage};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_legacy_serialized_thread_upgrade() {
        let updated_at = Utc::now();
        let legacy_thread = LegacySerializedThread {
            summary: "Test conversation".into(),
            updated_at,
            messages: vec![LegacySerializedMessage {
                id: MessageId(1),
                role: Role::User,
                text: "Hello, world!".to_string(),
                tool_uses: vec![],
                tool_results: vec![],
            }],
            initial_project_snapshot: None,
        };

        let upgraded = legacy_thread.upgrade();

        assert_eq!(
            upgraded,
            SerializedThread {
                summary: "Test conversation".into(),
                updated_at,
                messages: vec![SerializedMessage {
                    id: MessageId(1),
                    role: Role::User,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "Hello, world!".to_string()
                    }],
                    tool_uses: vec![],
                    tool_results: vec![],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false
                }],
                version: SerializedThread::VERSION.to_string(),
                initial_project_snapshot: None,
                cumulative_token_usage: TokenUsage::default(),
                request_token_usage: vec![],
                detailed_summary_state: DetailedSummaryState::default(),
                model: None,
                completion_mode: None,
                tool_use_limit_reached: false,
                profile: None
            }
        )
    }

    #[test]
    fn test_serialized_threadv0_1_0_upgrade() {
        let updated_at = Utc::now();
        let thread_v0_1_0 = SerializedThreadV0_1_0(SerializedThread {
            summary: "Test conversation".into(),
            updated_at,
            messages: vec![
                SerializedMessage {
                    id: MessageId(1),
                    role: Role::User,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "Use tool_1".to_string(),
                    }],
                    tool_uses: vec![],
                    tool_results: vec![],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false,
                },
                SerializedMessage {
                    id: MessageId(2),
                    role: Role::Assistant,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "I want to use a tool".to_string(),
                    }],
                    tool_uses: vec![SerializedToolUse {
                        id: "abc".into(),
                        name: "tool_1".into(),
                        input: serde_json::Value::Null,
                    }],
                    tool_results: vec![],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false,
                },
                SerializedMessage {
                    id: MessageId(1),
                    role: Role::User,
                    segments: vec![SerializedMessageSegment::Text {
                        text: "Here is the tool result".to_string(),
                    }],
                    tool_uses: vec![],
                    tool_results: vec![SerializedToolResult {
                        tool_use_id: "abc".into(),
                        is_error: false,
                        content: LanguageModelToolResultContent::Text("abcdef".into()),
                        output: Some(serde_json::Value::Null),
                    }],
                    context: "".to_string(),
                    creases: vec![],
                    is_hidden: false,
                },
            ],
            version: SerializedThreadV0_1_0::VERSION.to_string(),
            initial_project_snapshot: None,
            cumulative_token_usage: TokenUsage::default(),
            request_token_usage: vec![],
            detailed_summary_state: DetailedSummaryState::default(),
            model: None,
            completion_mode: None,
            tool_use_limit_reached: false,
            profile: None,
        });
        let upgraded = thread_v0_1_0.upgrade();

        assert_eq!(
            upgraded,
            SerializedThread {
                summary: "Test conversation".into(),
                updated_at,
                messages: vec![
                    SerializedMessage {
                        id: MessageId(1),
                        role: Role::User,
                        segments: vec![SerializedMessageSegment::Text {
                            text: "Use tool_1".to_string()
                        }],
                        tool_uses: vec![],
                        tool_results: vec![],
                        context: "".to_string(),
                        creases: vec![],
                        is_hidden: false
                    },
                    SerializedMessage {
                        id: MessageId(2),
                        role: Role::Assistant,
                        segments: vec![SerializedMessageSegment::Text {
                            text: "I want to use a tool".to_string(),
                        }],
                        tool_uses: vec![SerializedToolUse {
                            id: "abc".into(),
                            name: "tool_1".into(),
                            input: serde_json::Value::Null,
                        }],
                        tool_results: vec![SerializedToolResult {
                            tool_use_id: "abc".into(),
                            is_error: false,
                            content: LanguageModelToolResultContent::Text("abcdef".into()),
                            output: Some(serde_json::Value::Null),
                        }],
                        context: "".to_string(),
                        creases: vec![],
                        is_hidden: false,
                    },
                ],
                version: SerializedThread::VERSION.to_string(),
                initial_project_snapshot: None,
                cumulative_token_usage: TokenUsage::default(),
                request_token_usage: vec![],
                detailed_summary_state: DetailedSummaryState::default(),
                model: None,
                completion_mode: None,
                tool_use_limit_reached: false,
                profile: None
            }
        )
    }
}
