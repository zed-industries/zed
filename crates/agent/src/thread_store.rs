use crate::{
    context_server_tool::ContextServerTool,
    thread::{
        DetailedSummaryState, ExceededWindowError, MessageId, ProjectSnapshot, Thread, ThreadId,
    },
};
use agent_settings::{AgentProfileId, CompletionMode};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{Tool, ToolId, ToolWorkingSet};
use chrono::{DateTime, Utc};
use collections::HashMap;
use context_server::ContextServerId;
use futures::{
    FutureExt as _, StreamExt as _,
    channel::{mpsc, oneshot},
    future::{self, BoxFuture, Shared},
};
use gpui::{
    App, BackgroundExecutor, Context, Entity, EventEmitter, Global, ReadGlobal, SharedString,
    Subscription, Task, Window, prelude::*,
};
use indoc::indoc;
use language_model::{LanguageModelToolResultContent, LanguageModelToolUseId, Role, TokenUsage};
use project::context_server_store::{ContextServerStatus, ContextServerStore};
use project::{Project, ProjectItem, ProjectPath, Worktree};
use prompt_store::{
    ProjectContext, PromptBuilder, PromptId, PromptStore, PromptsUpdatedEvent, RulesFileContext,
    UserRulesContext, WorktreeContext,
};
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
use util::ResultExt as _;

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

const RULES_FILE_NAMES: [&str; 9] = [
    ".rules",
    ".cursorrules",
    ".windsurfrules",
    ".clinerules",
    ".github/copilot-instructions.md",
    "CLAUDE.md",
    "AGENT.md",
    "AGENTS.md",
    "GEMINI.md",
];

pub fn init(cx: &mut App) {
    ThreadsDatabase::init(cx);
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
    project: Entity<Project>,
    tools: Entity<ToolWorkingSet>,
    prompt_builder: Arc<PromptBuilder>,
    prompt_store: Option<Entity<PromptStore>>,
    context_server_tool_ids: HashMap<ContextServerId, Vec<ToolId>>,
    threads: Vec<SerializedThreadMetadata>,
    project_context: SharedProjectContext,
    reload_system_prompt_tx: mpsc::Sender<()>,
    _reload_system_prompt_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

pub struct RulesLoadingError {
    pub message: SharedString,
}

impl EventEmitter<RulesLoadingError> for ThreadStore {}

impl ThreadStore {
    pub fn load(
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        prompt_store: Option<Entity<PromptStore>>,
        prompt_builder: Arc<PromptBuilder>,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            let (thread_store, ready_rx) = cx.update(|cx| {
                let mut option_ready_rx = None;
                let thread_store = cx.new(|cx| {
                    let (thread_store, ready_rx) =
                        Self::new(project, tools, prompt_builder, prompt_store, cx);
                    option_ready_rx = Some(ready_rx);
                    thread_store
                });
                (thread_store, option_ready_rx.take().unwrap())
            })?;
            ready_rx.await?;
            Ok(thread_store)
        })
    }

    fn new(
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        prompt_builder: Arc<PromptBuilder>,
        prompt_store: Option<Entity<PromptStore>>,
        cx: &mut Context<Self>,
    ) -> (Self, oneshot::Receiver<()>) {
        let mut subscriptions = vec![cx.subscribe(&project, Self::handle_project_event)];

        if let Some(prompt_store) = prompt_store.as_ref() {
            subscriptions.push(cx.subscribe(
                prompt_store,
                |this, _prompt_store, PromptsUpdatedEvent, _cx| {
                    this.enqueue_system_prompt_reload();
                },
            ))
        }

        // This channel and task prevent concurrent and redundant loading of the system prompt.
        let (reload_system_prompt_tx, mut reload_system_prompt_rx) = mpsc::channel(1);
        let (ready_tx, ready_rx) = oneshot::channel();
        let mut ready_tx = Some(ready_tx);
        let reload_system_prompt_task = cx.spawn({
            let prompt_store = prompt_store.clone();
            async move |thread_store, cx| {
                loop {
                    let Some(reload_task) = thread_store
                        .update(cx, |thread_store, cx| {
                            thread_store.reload_system_prompt(prompt_store.clone(), cx)
                        })
                        .ok()
                    else {
                        return;
                    };
                    reload_task.await;
                    if let Some(ready_tx) = ready_tx.take() {
                        ready_tx.send(()).ok();
                    }
                    reload_system_prompt_rx.next().await;
                }
            }
        });

        let this = Self {
            project,
            tools,
            prompt_builder,
            prompt_store,
            context_server_tool_ids: HashMap::default(),
            threads: Vec::new(),
            project_context: SharedProjectContext::default(),
            reload_system_prompt_tx,
            _reload_system_prompt_task: reload_system_prompt_task,
            _subscriptions: subscriptions,
        };
        this.register_context_server_handlers(cx);
        this.reload(cx).detach_and_log_err(cx);
        (this, ready_rx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(project: Entity<Project>, cx: &mut App) -> Self {
        Self {
            project,
            tools: cx.new(|_| ToolWorkingSet::default()),
            prompt_builder: Arc::new(PromptBuilder::new(None).unwrap()),
            prompt_store: None,
            context_server_tool_ids: HashMap::default(),
            threads: Vec::new(),
            project_context: SharedProjectContext::default(),
            reload_system_prompt_tx: mpsc::channel(0).0,
            _reload_system_prompt_task: Task::ready(()),
            _subscriptions: vec![],
        }
    }

    fn handle_project_event(
        &mut self,
        _project: Entity<Project>,
        event: &project::Event,
        _cx: &mut Context<Self>,
    ) {
        match event {
            project::Event::WorktreeAdded(_) | project::Event::WorktreeRemoved(_) => {
                self.enqueue_system_prompt_reload();
            }
            project::Event::WorktreeUpdatedEntries(_, items) => {
                if items.iter().any(|(path, _, _)| {
                    RULES_FILE_NAMES
                        .iter()
                        .any(|name| path.as_ref() == Path::new(name))
                }) {
                    self.enqueue_system_prompt_reload();
                }
            }
            _ => {}
        }
    }

    fn enqueue_system_prompt_reload(&mut self) {
        self.reload_system_prompt_tx.try_send(()).ok();
    }

    // Note that this should only be called from `reload_system_prompt_task`.
    fn reload_system_prompt(
        &self,
        prompt_store: Option<Entity<PromptStore>>,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let worktrees = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();
        let worktree_tasks = worktrees
            .into_iter()
            .map(|worktree| {
                Self::load_worktree_info_for_system_prompt(worktree, self.project.clone(), cx)
            })
            .collect::<Vec<_>>();
        let default_user_rules_task = match prompt_store {
            None => Task::ready(vec![]),
            Some(prompt_store) => prompt_store.read_with(cx, |prompt_store, cx| {
                let prompts = prompt_store.default_prompt_metadata();
                let load_tasks = prompts.into_iter().map(|prompt_metadata| {
                    let contents = prompt_store.load(prompt_metadata.id, cx);
                    async move { (contents.await, prompt_metadata) }
                });
                cx.background_spawn(future::join_all(load_tasks))
            }),
        };

        cx.spawn(async move |this, cx| {
            let (worktrees, default_user_rules) =
                future::join(future::join_all(worktree_tasks), default_user_rules_task).await;

            let worktrees = worktrees
                .into_iter()
                .map(|(worktree, rules_error)| {
                    if let Some(rules_error) = rules_error {
                        this.update(cx, |_, cx| cx.emit(rules_error)).ok();
                    }
                    worktree
                })
                .collect::<Vec<_>>();

            let default_user_rules = default_user_rules
                .into_iter()
                .flat_map(|(contents, prompt_metadata)| match contents {
                    Ok(contents) => Some(UserRulesContext {
                        uuid: match prompt_metadata.id {
                            PromptId::User { uuid } => uuid,
                            PromptId::EditWorkflow => return None,
                        },
                        title: prompt_metadata.title.map(|title| title.to_string()),
                        contents,
                    }),
                    Err(err) => {
                        this.update(cx, |_, cx| {
                            cx.emit(RulesLoadingError {
                                message: format!("{err:?}").into(),
                            });
                        })
                        .ok();
                        None
                    }
                })
                .collect::<Vec<_>>();

            this.update(cx, |this, _cx| {
                *this.project_context.0.borrow_mut() =
                    Some(ProjectContext::new(worktrees, default_user_rules));
            })
            .ok();
        })
    }

    fn load_worktree_info_for_system_prompt(
        worktree: Entity<Worktree>,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<(WorktreeContext, Option<RulesLoadingError>)> {
        let tree = worktree.read(cx);
        let root_name = tree.root_name().into();
        let abs_path = tree.abs_path();

        let mut context = WorktreeContext {
            root_name,
            abs_path,
            rules_file: None,
        };

        let rules_task = Self::load_worktree_rules_file(worktree, project, cx);
        let Some(rules_task) = rules_task else {
            return Task::ready((context, None));
        };

        cx.spawn(async move |_| {
            let (rules_file, rules_file_error) = match rules_task.await {
                Ok(rules_file) => (Some(rules_file), None),
                Err(err) => (
                    None,
                    Some(RulesLoadingError {
                        message: format!("{err}").into(),
                    }),
                ),
            };
            context.rules_file = rules_file;
            (context, rules_file_error)
        })
    }

    fn load_worktree_rules_file(
        worktree: Entity<Worktree>,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Option<Task<Result<RulesFileContext>>> {
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let selected_rules_file = RULES_FILE_NAMES
            .into_iter()
            .filter_map(|name| {
                worktree
                    .entry_for_path(name)
                    .filter(|entry| entry.is_file())
                    .map(|entry| entry.path.clone())
            })
            .next();

        // Note that Cline supports `.clinerules` being a directory, but that is not currently
        // supported. This doesn't seem to occur often in GitHub repositories.
        selected_rules_file.map(|path_in_worktree| {
            let project_path = ProjectPath {
                worktree_id,
                path: path_in_worktree.clone(),
            };
            let buffer_task =
                project.update(cx, |project, cx| project.open_buffer(project_path, cx));
            let rope_task = cx.spawn(async move |cx| {
                buffer_task.await?.read_with(cx, |buffer, cx| {
                    let project_entry_id = buffer.entry_id(cx).context("buffer has no file")?;
                    anyhow::Ok((project_entry_id, buffer.as_rope().clone()))
                })?
            });
            // Build a string from the rope on a background thread.
            cx.background_spawn(async move {
                let (project_entry_id, rope) = rope_task.await?;
                anyhow::Ok(RulesFileContext {
                    path_in_worktree,
                    text: rope.to_string().trim().to_string(),
                    project_entry_id: project_entry_id.to_usize(),
                })
            })
        })
    }

    pub fn prompt_store(&self) -> &Option<Entity<PromptStore>> {
        &self.prompt_store
    }

    pub fn tools(&self) -> Entity<ToolWorkingSet> {
        self.tools.clone()
    }

    /// Returns the number of threads.
    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }

    pub fn reverse_chronological_threads(&self) -> impl Iterator<Item = &SerializedThreadMetadata> {
        // ordering is from "ORDER BY" in `list_threads`
        self.threads.iter()
    }

    pub fn create_thread(&mut self, cx: &mut Context<Self>) -> Entity<Thread> {
        cx.new(|cx| {
            Thread::new(
                self.project.clone(),
                self.tools.clone(),
                self.prompt_builder.clone(),
                self.project_context.clone(),
                cx,
            )
        })
    }

    pub fn create_thread_from_serialized(
        &mut self,
        serialized: SerializedThread,
        cx: &mut Context<Self>,
    ) -> Entity<Thread> {
        cx.new(|cx| {
            Thread::deserialize(
                ThreadId::new(),
                serialized,
                self.project.clone(),
                self.tools.clone(),
                self.prompt_builder.clone(),
                self.project_context.clone(),
                None,
                cx,
            )
        })
    }

    pub fn open_thread(
        &self,
        id: &ThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Thread>>> {
        let id = id.clone();
        let database_future = ThreadsDatabase::global_future(cx);
        let this = cx.weak_entity();
        window.spawn(cx, async move |cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            let thread = database
                .try_find_thread(id.clone())
                .await?
                .with_context(|| format!("no thread found with ID: {id:?}"))?;

            let thread = this.update_in(cx, |this, window, cx| {
                cx.new(|cx| {
                    Thread::deserialize(
                        id.clone(),
                        thread,
                        this.project.clone(),
                        this.tools.clone(),
                        this.prompt_builder.clone(),
                        this.project_context.clone(),
                        Some(window),
                        cx,
                    )
                })
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

    fn register_context_server_handlers(&self, cx: &mut Context<Self>) {
        let context_server_store = self.project.read(cx).context_server_store();
        cx.subscribe(&context_server_store, Self::handle_context_server_event)
            .detach();

        // Check for any servers that were already running before the handler was registered
        for server in context_server_store.read(cx).running_servers() {
            self.load_context_server_tools(server.id(), context_server_store.clone(), cx);
        }
    }

    fn handle_context_server_event(
        &mut self,
        context_server_store: Entity<ContextServerStore>,
        event: &project::context_server_store::Event,
        cx: &mut Context<Self>,
    ) {
        let tool_working_set = self.tools.clone();
        match event {
            project::context_server_store::Event::ServerStatusChanged { server_id, status } => {
                match status {
                    ContextServerStatus::Starting => {}
                    ContextServerStatus::Running => {
                        self.load_context_server_tools(server_id.clone(), context_server_store, cx);
                    }
                    ContextServerStatus::Stopped | ContextServerStatus::Error(_) => {
                        if let Some(tool_ids) = self.context_server_tool_ids.remove(server_id) {
                            tool_working_set.update(cx, |tool_working_set, cx| {
                                tool_working_set.remove(&tool_ids, cx);
                            });
                        }
                    }
                }
            }
        }
    }

    fn load_context_server_tools(
        &self,
        server_id: ContextServerId,
        context_server_store: Entity<ContextServerStore>,
        cx: &mut Context<Self>,
    ) {
        let Some(server) = context_server_store.read(cx).get_running_server(&server_id) else {
            return;
        };
        let tool_working_set = self.tools.clone();
        cx.spawn(async move |this, cx| {
            let Some(protocol) = server.client() else {
                return;
            };

            if protocol.capable(context_server::protocol::ServerCapability::Tools)
                && let Some(response) = protocol
                    .request::<context_server::types::requests::ListTools>(())
                    .await
                    .log_err()
            {
                let tool_ids = tool_working_set
                    .update(cx, |tool_working_set, cx| {
                        tool_working_set.extend(
                            response.tools.into_iter().map(|tool| {
                                Arc::new(ContextServerTool::new(
                                    context_server_store.clone(),
                                    server.id(),
                                    tool,
                                )) as Arc<dyn Tool>
                            }),
                            cx,
                        )
                    })
                    .log_err();

                if let Some(tool_ids) = tool_ids {
                    this.update(cx, |this, _| {
                        this.context_server_tool_ids.insert(server_id, tool_ids);
                    })
                    .log_err();
                }
            }
        })
        .detach();
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
    pub exceeded_window_error: Option<ExceededWindowError>,
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
            exceeded_window_error: None,
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

    fn init(cx: &mut App) {
        let executor = cx.background_executor().clone();
        let database_future = executor
            .spawn({
                let executor = executor.clone();
                let threads_dir = paths::data_dir().join("threads");
                async move { ThreadsDatabase::new(threads_dir, executor) }
            })
            .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
            .boxed()
            .shared();

        cx.set_global(GlobalThreadsDatabase(database_future));
    }

    pub fn new(threads_dir: PathBuf, executor: BackgroundExecutor) -> Result<Self> {
        std::fs::create_dir_all(&threads_dir)?;

        let sqlite_path = threads_dir.join("threads.db");
        let mdb_path = threads_dir.join("threads-db.1.mdb");

        let needs_migration_from_heed = mdb_path.exists();

        let connection = if *ZED_STATELESS || cfg!(any(feature = "test-support", test)) {
            Connection::open_memory(Some("THREAD_FALLBACK_DB"))
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
                    std::fs::remove_dir_all(mdb_path)?;
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

    pub fn try_find_thread(&self, id: ThreadId) -> Task<Result<Option<SerializedThread>>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock().unwrap();
            let mut select = connection.select_bound::<ThreadId, (DataType, Vec<u8>)>(indoc! {"
                SELECT data_type, data FROM threads WHERE id = ? LIMIT 1
            "})?;

            let rows = select(id)?;
            if let Some((data_type, data)) = rows.into_iter().next() {
                let json_data = match data_type {
                    DataType::Zstd => {
                        let decompressed = zstd::decode_all(&data[..])?;
                        String::from_utf8(decompressed)?
                    }
                    DataType::Json => String::from_utf8(data)?,
                };

                let thread = SerializedThread::from_json(json_data.as_bytes())?;
                Ok(Some(thread))
            } else {
                Ok(None)
            }
        })
    }

    pub fn save_thread(&self, id: ThreadId, thread: SerializedThread) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor
            .spawn(async move { Self::save_thread_sync(&connection, id, thread) })
    }

    pub fn delete_thread(&self, id: ThreadId) -> Task<Result<()>> {
        let connection = self.connection.clone();

        self.executor.spawn(async move {
            let connection = connection.lock().unwrap();

            let mut delete = connection.exec_bound::<ThreadId>(indoc! {"
                DELETE FROM threads WHERE id = ?
            "})?;

            delete(id)?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thread::{DetailedSummaryState, MessageId};
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
                exceeded_window_error: None,
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
            exceeded_window_error: None,
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
                exceeded_window_error: None,
                model: None,
                completion_mode: None,
                tool_use_limit_reached: false,
                profile: None
            }
        )
    }
}
