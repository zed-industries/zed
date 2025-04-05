use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use assistant_settings::{AgentProfile, AgentProfileId, AssistantSettings};
use assistant_tool::{ToolId, ToolSource, ToolWorkingSet};
use chrono::{DateTime, Utc};
use collections::HashMap;
use context_server::manager::ContextServerManager;
use context_server::{ContextServerFactoryRegistry, ContextServerTool};
use futures::FutureExt as _;
use futures::future::{self, BoxFuture, Shared};
use gpui::{
    App, BackgroundExecutor, Context, Entity, Global, ReadGlobal, SharedString, Subscription, Task,
    prelude::*,
};
use heed::Database;
use heed::types::SerdeBincode;
use language_model::{LanguageModelToolUseId, Role, TokenUsage};
use project::Project;
use prompt_store::PromptBuilder;
use serde::{Deserialize, Serialize};
use settings::{Settings as _, SettingsStore};
use util::ResultExt as _;

use crate::thread::{
    DetailedSummaryState, MessageId, ProjectSnapshot, Thread, ThreadEvent, ThreadId,
};

pub fn init(cx: &mut App) {
    ThreadsDatabase::init(cx);
}

pub struct ThreadStore {
    project: Entity<Project>,
    tools: Arc<ToolWorkingSet>,
    prompt_builder: Arc<PromptBuilder>,
    context_server_manager: Entity<ContextServerManager>,
    context_server_tool_ids: HashMap<Arc<str>, Vec<ToolId>>,
    threads: Vec<SerializedThreadMetadata>,
    _subscriptions: Vec<Subscription>,
}

impl ThreadStore {
    pub fn new(
        project: Entity<Project>,
        tools: Arc<ToolWorkingSet>,
        prompt_builder: Arc<PromptBuilder>,
        cx: &mut App,
    ) -> Result<Entity<Self>> {
        let this = cx.new(|cx| {
            let context_server_factory_registry = ContextServerFactoryRegistry::default_global(cx);
            let context_server_manager = cx.new(|cx| {
                ContextServerManager::new(context_server_factory_registry, project.clone(), cx)
            });
            let settings_subscription =
                cx.observe_global::<SettingsStore>(move |this: &mut Self, cx| {
                    this.load_default_profile(cx);
                });

            let this = Self {
                project,
                tools,
                prompt_builder,
                context_server_manager,
                context_server_tool_ids: HashMap::default(),
                threads: Vec::new(),
                _subscriptions: vec![settings_subscription],
            };
            this.load_default_profile(cx);
            this.register_context_server_handlers(cx);
            this.reload(cx).detach_and_log_err(cx);

            this
        });

        Ok(this)
    }

    pub fn context_server_manager(&self) -> Entity<ContextServerManager> {
        self.context_server_manager.clone()
    }

    pub fn tools(&self) -> Arc<ToolWorkingSet> {
        self.tools.clone()
    }

    /// Returns the number of threads.
    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }

    pub fn threads(&self) -> Vec<SerializedThreadMetadata> {
        let mut threads = self.threads.iter().cloned().collect::<Vec<_>>();
        threads.sort_unstable_by_key(|thread| std::cmp::Reverse(thread.updated_at));
        threads
    }

    pub fn recent_threads(&self, limit: usize) -> Vec<SerializedThreadMetadata> {
        self.threads().into_iter().take(limit).collect()
    }

    pub fn create_thread(&mut self, cx: &mut Context<Self>) -> Entity<Thread> {
        cx.new(|cx| {
            Thread::new(
                self.project.clone(),
                self.tools.clone(),
                self.prompt_builder.clone(),
                cx,
            )
        })
    }

    pub fn open_thread(
        &self,
        id: &ThreadId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Thread>>> {
        let id = id.clone();
        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            let thread = database
                .try_find_thread(id.clone())
                .await?
                .ok_or_else(|| anyhow!("no thread found with ID: {id:?}"))?;

            let thread = this.update(cx, |this, cx| {
                cx.new(|cx| {
                    Thread::deserialize(
                        id.clone(),
                        thread,
                        this.project.clone(),
                        this.tools.clone(),
                        this.prompt_builder.clone(),
                        cx,
                    )
                })
            })?;

            let (system_prompt_context, load_error) = thread
                .update(cx, |thread, cx| thread.load_system_prompt_context(cx))?
                .await;
            thread.update(cx, |thread, cx| {
                thread.set_system_prompt_context(system_prompt_context);
                if let Some(load_error) = load_error {
                    cx.emit(ThreadEvent::ShowError(load_error));
                }
            })?;

            Ok(thread)
        })
    }

    pub fn save_thread(&self, thread: &Entity<Thread>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let (metadata, serialized_thread) =
            thread.update(cx, |thread, cx| (thread.id().clone(), thread.serialize(cx)));

        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(async move |this, cx| {
            let serialized_thread = serialized_thread.await?;
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.save_thread(metadata, serialized_thread).await?;

            this.update(cx, |this, cx| this.reload(cx))?.await
        })
    }

    pub fn delete_thread(&mut self, id: &ThreadId, cx: &mut Context<Self>) -> Task<Result<()>> {
        let id = id.clone();
        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_thread(id.clone()).await?;

            this.update(cx, |this, cx| {
                this.threads.retain(|thread| thread.id != id);
                cx.notify();
            })
        })
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

    fn load_default_profile(&self, cx: &Context<Self>) {
        let assistant_settings = AssistantSettings::get_global(cx);

        self.load_profile_by_id(&assistant_settings.default_profile, cx);
    }

    pub fn load_profile_by_id(&self, profile_id: &AgentProfileId, cx: &Context<Self>) {
        let assistant_settings = AssistantSettings::get_global(cx);

        if let Some(profile) = assistant_settings.profiles.get(profile_id) {
            self.load_profile(profile, cx);
        }
    }

    pub fn load_profile(&self, profile: &AgentProfile, cx: &Context<Self>) {
        self.tools.disable_all_tools();
        self.tools.enable(
            ToolSource::Native,
            &profile
                .tools
                .iter()
                .filter_map(|(tool, enabled)| enabled.then(|| tool.clone()))
                .collect::<Vec<_>>(),
        );

        if profile.enable_all_context_servers {
            for context_server in self.context_server_manager.read(cx).all_servers() {
                self.tools.enable_source(
                    ToolSource::ContextServer {
                        id: context_server.id().into(),
                    },
                    cx,
                );
            }
        } else {
            for (context_server_id, preset) in &profile.context_servers {
                self.tools.enable(
                    ToolSource::ContextServer {
                        id: context_server_id.clone().into(),
                    },
                    &preset
                        .tools
                        .iter()
                        .filter_map(|(tool, enabled)| enabled.then(|| tool.clone()))
                        .collect::<Vec<_>>(),
                )
            }
        }
    }

    fn register_context_server_handlers(&self, cx: &mut Context<Self>) {
        cx.subscribe(
            &self.context_server_manager.clone(),
            Self::handle_context_server_event,
        )
        .detach();
    }

    fn handle_context_server_event(
        &mut self,
        context_server_manager: Entity<ContextServerManager>,
        event: &context_server::manager::Event,
        cx: &mut Context<Self>,
    ) {
        let tool_working_set = self.tools.clone();
        match event {
            context_server::manager::Event::ServerStarted { server_id } => {
                if let Some(server) = context_server_manager.read(cx).get_server(server_id) {
                    let context_server_manager = context_server_manager.clone();
                    cx.spawn({
                        let server = server.clone();
                        let server_id = server_id.clone();
                        async move |this, cx| {
                            let Some(protocol) = server.client() else {
                                return;
                            };

                            if protocol.capable(context_server::protocol::ServerCapability::Tools) {
                                if let Some(tools) = protocol.list_tools().await.log_err() {
                                    let tool_ids = tools
                                        .tools
                                        .into_iter()
                                        .map(|tool| {
                                            log::info!(
                                                "registering context server tool: {:?}",
                                                tool.name
                                            );
                                            tool_working_set.insert(Arc::new(
                                                ContextServerTool::new(
                                                    context_server_manager.clone(),
                                                    server.id(),
                                                    tool,
                                                ),
                                            ))
                                        })
                                        .collect::<Vec<_>>();

                                    this.update(cx, |this, cx| {
                                        this.context_server_tool_ids.insert(server_id, tool_ids);
                                        this.load_default_profile(cx);
                                    })
                                    .log_err();
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            context_server::manager::Event::ServerStopped { server_id } => {
                if let Some(tool_ids) = self.context_server_tool_ids.remove(server_id) {
                    tool_working_set.remove(&tool_ids);
                    self.load_default_profile(cx);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedThreadMetadata {
    pub id: ThreadId,
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub detailed_summary_state: DetailedSummaryState,
}

impl SerializedThread {
    pub const VERSION: &'static str = "0.1.0";

    pub fn from_json(json: &[u8]) -> Result<Self> {
        let saved_thread_json = serde_json::from_slice::<serde_json::Value>(json)?;
        match saved_thread_json.get("version") {
            Some(serde_json::Value::String(version)) => match version.as_str() {
                SerializedThread::VERSION => Ok(serde_json::from_value::<SerializedThread>(
                    saved_thread_json,
                )?),
                _ => Err(anyhow!(
                    "unrecognized serialized thread version: {}",
                    version
                )),
            },
            None => {
                let saved_thread =
                    serde_json::from_value::<LegacySerializedThread>(saved_thread_json)?;
                Ok(saved_thread.upgrade())
            }
            version => Err(anyhow!(
                "unrecognized serialized thread version: {:?}",
                version
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SerializedMessageSegment {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { text: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedToolUse {
    pub id: LanguageModelToolUseId,
    pub name: SharedString,
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedToolResult {
    pub tool_use_id: LanguageModelToolUseId,
    pub is_error: bool,
    pub content: Arc<str>,
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
            detailed_summary_state: DetailedSummaryState::default(),
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
        }
    }
}

struct GlobalThreadsDatabase(
    Shared<BoxFuture<'static, Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>>,
);

impl Global for GlobalThreadsDatabase {}

pub(crate) struct ThreadsDatabase {
    executor: BackgroundExecutor,
    env: heed::Env,
    threads: Database<SerdeBincode<ThreadId>, SerializedThread>,
}

impl heed::BytesEncode<'_> for SerializedThread {
    type EItem = SerializedThread;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<[u8]>, heed::BoxedError> {
        serde_json::to_vec(item).map(Cow::Owned).map_err(Into::into)
    }
}

impl<'a> heed::BytesDecode<'a> for SerializedThread {
    type DItem = SerializedThread;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
        // We implement this type manually because we want to call `SerializedThread::from_json`,
        // instead of the Deserialize trait implementation for `SerializedThread`.
        SerializedThread::from_json(bytes).map_err(Into::into)
    }
}

impl ThreadsDatabase {
    fn global_future(
        cx: &mut App,
    ) -> Shared<BoxFuture<'static, Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>> {
        GlobalThreadsDatabase::global(cx).0.clone()
    }

    fn init(cx: &mut App) {
        let executor = cx.background_executor().clone();
        let database_future = executor
            .spawn({
                let executor = executor.clone();
                let database_path = paths::support_dir().join("threads/threads-db.1.mdb");
                async move { ThreadsDatabase::new(database_path, executor) }
            })
            .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
            .boxed()
            .shared();

        cx.set_global(GlobalThreadsDatabase(database_future));
    }

    pub fn new(path: PathBuf, executor: BackgroundExecutor) -> Result<Self> {
        std::fs::create_dir_all(&path)?;

        const ONE_GB_IN_BYTES: usize = 1024 * 1024 * 1024;
        let env = unsafe {
            heed::EnvOpenOptions::new()
                .map_size(ONE_GB_IN_BYTES)
                .max_dbs(1)
                .open(path)?
        };

        let mut txn = env.write_txn()?;
        let threads = env.create_database(&mut txn, Some("threads"))?;
        txn.commit()?;

        Ok(Self {
            executor,
            env,
            threads,
        })
    }

    pub fn list_threads(&self) -> Task<Result<Vec<SerializedThreadMetadata>>> {
        let env = self.env.clone();
        let threads = self.threads;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let mut iter = threads.iter(&txn)?;
            let mut threads = Vec::new();
            while let Some((key, value)) = iter.next().transpose()? {
                threads.push(SerializedThreadMetadata {
                    id: key,
                    summary: value.summary,
                    updated_at: value.updated_at,
                });
            }

            Ok(threads)
        })
    }

    pub fn try_find_thread(&self, id: ThreadId) -> Task<Result<Option<SerializedThread>>> {
        let env = self.env.clone();
        let threads = self.threads;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let thread = threads.get(&txn, &id)?;
            Ok(thread)
        })
    }

    pub fn save_thread(&self, id: ThreadId, thread: SerializedThread) -> Task<Result<()>> {
        let env = self.env.clone();
        let threads = self.threads;

        self.executor.spawn(async move {
            let mut txn = env.write_txn()?;
            threads.put(&mut txn, &id, &thread)?;
            txn.commit()?;
            Ok(())
        })
    }

    pub fn delete_thread(&self, id: ThreadId) -> Task<Result<()>> {
        let env = self.env.clone();
        let threads = self.threads;

        self.executor.spawn(async move {
            let mut txn = env.write_txn()?;
            threads.delete(&mut txn, &id)?;
            txn.commit()?;
            Ok(())
        })
    }
}
