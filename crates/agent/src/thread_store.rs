use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use agent_settings::{AgentProfile, AgentProfileId, AgentSettings, CompletionMode};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ToolId, ToolSource, ToolWorkingSet};
use chrono::{DateTime, Utc};
use collections::HashMap;
use context_server::ContextServerId;
use db::sqlez::bindable::Column;
use db::sqlez::statement::Statement;
use db::sqlez_macros::sql;
use db::{define_connection, query};
use futures::channel::{mpsc, oneshot};
use futures::future::{self, BoxFuture, Shared};
use futures::{FutureExt as _, StreamExt as _};
use gpui::{
    App, BackgroundExecutor, Context, Entity, EventEmitter, Global, ReadGlobal, SharedString,
    Subscription, Task, prelude::*,
};
use heed;
use language_model::{LanguageModelToolResultContent, LanguageModelToolUseId, Role, TokenUsage};
use project::context_server_store::{ContextServerStatus, ContextServerStore};
use project::{Project, ProjectItem, ProjectPath, Worktree};
use prompt_store::{
    ProjectContext, PromptBuilder, PromptId, PromptStore, PromptsUpdatedEvent, RulesFileContext,
    UserRulesContext, WorktreeContext,
};
use serde::{Deserialize, Serialize};
use settings::{Settings as _, SettingsStore};
use ui::Window;
use util::ResultExt as _;

use crate::context_server_tool::ContextServerTool;
use crate::thread::{
    DetailedSummaryState, ExceededWindowError, MessageId, ProjectSnapshot, Thread, ThreadId,
};

// Implement Bind trait for ThreadId to use in SQL queries
// impl db::sqlez::bindable::Bind for ThreadId {
//     fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
//         self.to_string().bind(statement, start_index)
//     }
// }

// Implement Column trait for SerializedThreadMetadata
impl Column for SerializedThreadMetadata {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (id_str, next_index): (String, i32) = Column::column(statement, start_index)?;
        let (summary, next_index): (String, i32) = Column::column(statement, next_index)?;
        let (updated_at_timestamp, next_index): (i64, i32) = Column::column(statement, next_index)?;

        Ok((
            Self {
                id: ThreadId::from(id_str.as_str()),
                summary: summary.into(),
                updated_at: DateTime::from_timestamp(updated_at_timestamp, 0)
                    .unwrap_or_else(DateTime::default),
            },
            next_index,
        ))
    }
}

const RULES_FILE_NAMES: [&'static str; 6] = [
    ".rules",
    ".cursorrules",
    ".windsurfrules",
    ".clinerules",
    ".github/copilot-instructions.md",
    "CLAUDE.md",
];

pub fn init(cx: &mut App) {
    ThreadsDatabase::init(cx);
}

/// A system prompt shared by all threads created by this ThreadStore
#[derive(Clone, Default)]
pub struct SharedProjectContext(Rc<RefCell<Option<ProjectContext>>>);

impl SharedProjectContext {
    pub fn borrow(&self) -> Ref<Option<ProjectContext>> {
        self.0.borrow()
    }
}

pub type TextThreadStore = assistant_context_editor::ContextStore;

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
        let mut subscriptions = vec![
            cx.observe_global::<SettingsStore>(move |this: &mut Self, cx| {
                this.load_default_profile(cx);
            }),
            cx.subscribe(&project, Self::handle_project_event),
        ];

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
        this.load_default_profile(cx);
        this.register_context_server_handlers(cx);
        this.reload(cx).detach_and_log_err(cx);
        (this, ready_rx)
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
        let root_name = worktree.read(cx).root_name().into();

        let rules_task = Self::load_worktree_rules_file(worktree, project, cx);
        let Some(rules_task) = rules_task else {
            return Task::ready((
                WorktreeContext {
                    root_name,
                    rules_file: None,
                },
                None,
            ));
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
            let worktree_info = WorktreeContext {
                root_name,
                rules_file,
            };
            (worktree_info, rules_file_error)
        })
    }

    fn load_worktree_rules_file(
        worktree: Entity<Worktree>,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Option<Task<Result<RulesFileContext>>> {
        let worktree_ref = worktree.read(cx);
        let worktree_id = worktree_ref.id();
        let selected_rules_file = RULES_FILE_NAMES
            .into_iter()
            .filter_map(|name| {
                worktree_ref
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

    pub fn unordered_threads(&self) -> impl Iterator<Item = &SerializedThreadMetadata> {
        self.threads.iter()
    }

    pub fn reverse_chronological_threads(&self) -> Vec<SerializedThreadMetadata> {
        let mut threads = self.threads.iter().cloned().collect::<Vec<_>>();
        threads.sort_unstable_by_key(|thread| std::cmp::Reverse(thread.updated_at));
        threads
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

    fn load_default_profile(&self, cx: &mut Context<Self>) {
        let assistant_settings = AgentSettings::get_global(cx);

        self.load_profile_by_id(assistant_settings.default_profile.clone(), cx);
    }

    pub fn load_profile_by_id(&self, profile_id: AgentProfileId, cx: &mut Context<Self>) {
        let assistant_settings = AgentSettings::get_global(cx);

        if let Some(profile) = assistant_settings.profiles.get(&profile_id) {
            self.load_profile(profile.clone(), cx);
        }
    }

    pub fn load_profile(&self, profile: AgentProfile, cx: &mut Context<Self>) {
        self.tools.update(cx, |tools, cx| {
            tools.disable_all_tools(cx);
            tools.enable(
                ToolSource::Native,
                &profile
                    .tools
                    .into_iter()
                    .filter_map(|(tool, enabled)| enabled.then(|| tool))
                    .collect::<Vec<_>>(),
                cx,
            );
        });

        if profile.enable_all_context_servers {
            for context_server_id in self
                .project
                .read(cx)
                .context_server_store()
                .read(cx)
                .all_server_ids()
            {
                self.tools.update(cx, |tools, cx| {
                    tools.enable_source(
                        ToolSource::ContextServer {
                            id: context_server_id.0.into(),
                        },
                        cx,
                    );
                });
            }
            // Enable all the tools from all context servers, but disable the ones that are explicitly disabled
            for (context_server_id, preset) in profile.context_servers {
                self.tools.update(cx, |tools, cx| {
                    tools.disable(
                        ToolSource::ContextServer {
                            id: context_server_id.into(),
                        },
                        &preset
                            .tools
                            .into_iter()
                            .filter_map(|(tool, enabled)| (!enabled).then(|| tool))
                            .collect::<Vec<_>>(),
                        cx,
                    )
                })
            }
        } else {
            for (context_server_id, preset) in profile.context_servers {
                self.tools.update(cx, |tools, cx| {
                    tools.enable(
                        ToolSource::ContextServer {
                            id: context_server_id.into(),
                        },
                        &preset
                            .tools
                            .into_iter()
                            .filter_map(|(tool, enabled)| enabled.then(|| tool))
                            .collect::<Vec<_>>(),
                        cx,
                    )
                })
            }
        }
    }

    fn register_context_server_handlers(&self, cx: &mut Context<Self>) {
        cx.subscribe(
            &self.project.read(cx).context_server_store(),
            Self::handle_context_server_event,
        )
        .detach();
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
                    ContextServerStatus::Running => {
                        if let Some(server) =
                            context_server_store.read(cx).get_running_server(server_id)
                        {
                            let context_server_manager = context_server_store.clone();
                            cx.spawn({
                                let server = server.clone();
                                let server_id = server_id.clone();
                                async move |this, cx| {
                                    let Some(protocol) = server.client() else {
                                        return;
                                    };

                                    if protocol.capable(context_server::protocol::ServerCapability::Tools) {
                                        if let Some(tools) = protocol.list_tools().await.log_err() {
                                            let tool_ids = tool_working_set
                                                .update(cx, |tool_working_set, _| {
                                                    tools
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
                                                        .collect::<Vec<_>>()
                                                })
                                                .log_err();

                                            if let Some(tool_ids) = tool_ids {
                                                this.update(cx, |this, cx| {
                                                    this.context_server_tool_ids
                                                        .insert(server_id, tool_ids);
                                                    this.load_default_profile(cx);
                                                })
                                                .log_err();
                                            }
                                        }
                                    }
                                }
                            })
                            .detach();
                        }
                    }
                    ContextServerStatus::Stopped | ContextServerStatus::Error(_) => {
                        if let Some(tool_ids) = self.context_server_tool_ids.remove(server_id) {
                            tool_working_set.update(cx, |tool_working_set, _| {
                                tool_working_set.remove(&tool_ids);
                            });
                            self.load_default_profile(cx);
                        }
                    }
                    _ => {}
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            if message.role == Role::User && !message.tool_results.is_empty() {
                if let Some(last_message) = messages.last_mut() {
                    debug_assert!(last_message.role == Role::Assistant);

                    last_message.tool_results = message.tool_results;
                    continue;
                }
            }

            messages.push(message);
        }

        SerializedThread { messages, ..self.0 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
        data: Vec<u8>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerializedToolUse {
    pub id: LanguageModelToolUseId,
    pub name: SharedString,
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
                async move { ThreadsDatabase::new(executor).await }
            })
            .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
            .boxed()
            .shared();

        cx.set_global(GlobalThreadsDatabase(database_future));
    }

    pub async fn new(executor: BackgroundExecutor) -> Result<Self> {
        Ok(Self { executor })
    }

    pub fn list_threads(&self) -> Task<Result<Vec<SerializedThreadMetadata>>> {
        self.executor
            .spawn(async move { AGENT_THREADS.all_threads().await })
    }

    pub fn try_find_thread(&self, id: ThreadId) -> Task<Result<Option<SerializedThread>>> {
        self.executor
            .spawn(async move { AGENT_THREADS.get_thread(id).await })
    }

    pub fn save_thread(&self, id: ThreadId, thread: SerializedThread) -> Task<Result<()>> {
        self.executor
            .spawn(async move { AGENT_THREADS.save_thread(id, thread).await })
    }

    pub fn delete_thread(&self, id: ThreadId) -> Task<Result<()>> {
        self.executor
            .spawn(async move { AGENT_THREADS.delete_thread_by_id(id).await })
    }

    /// Migrate a legacy `heed` LMDB database to SQLite
    pub async fn migrate_from_heed(heed_path: &Path) -> Result<()> {
        Self::migrate_from_heed_to_db(heed_path, &AGENT_THREADS).await
    }

    /// Migrate a legacy `heed` LMDB database to a specific SQLite database
    pub async fn migrate_from_heed_to_db(heed_path: &Path, db: &ThreadStoreDB) -> Result<()> {
        if !heed_path.exists() {
            return Ok(()); // No migration needed
        }

        // Open the old heed database
        let env = unsafe {
            heed::EnvOpenOptions::new()
                .map_size(1024 * 1024 * 1024) // 1GB
                .max_dbs(1)
                .open(&heed_path)?
        };

        let txn = env.read_txn()?;
        let old_threads: heed::Database<heed::types::SerdeBincode<ThreadId>, SerializedThread> =
            env.open_database(&txn, Some("threads"))?
                .ok_or_else(|| anyhow!("threads database not found"))?;

        // Migrate all threads
        for result in old_threads.iter(&txn)? {
            if let Some((id, thread)) = result.log_err() {
                db.save_thread(id, thread).await.log_err();
            }
        }

        drop(txn);
        drop(env);

        // TODO: delete the old heed db
        Ok(())
    }
}

// Heed serialization helpers for migration
impl heed::BytesEncode<'_> for SerializedThread {
    type EItem = SerializedThread;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<[u8]>, heed::BoxedError> {
        serde_json::to_vec(item).map(Cow::Owned).map_err(Into::into)
    }
}

impl<'a> heed::BytesDecode<'a> for SerializedThread {
    type DItem = SerializedThread;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
        SerializedThread::from_json(bytes).map_err(Into::into)
    }
}

define_connection!(pub static ref AGENT_THREADS: ThreadStoreDB<()> =
    &[sql!(
        CREATE TABLE IF NOT EXISTS agent_threads(
            id TEXT PRIMARY KEY,
            summary TEXT NOT NULL,
            updated_at INTEGER NOT NULL,
            data TEXT NOT NULL
        ) STRICT;
    )];
);

impl ThreadStoreDB {
    query! {
        pub async fn all_threads() -> Result<Vec<SerializedThreadMetadata>> {
            SELECT id, summary, updated_at
            FROM agent_threads
            ORDER BY updated_at DESC
        }
    }

    query! {
        async fn get_thread_data(id: String) -> Result<Option<String>> {
            SELECT data FROM agent_threads WHERE id = (?)
        }
    }

    query! {
        async fn save_thread_data(id: String, summary: String, updated_at: i64, data: String) -> Result<()> {
            INSERT OR REPLACE INTO agent_threads (id, summary, updated_at, data)
            VALUES ((?), (?), (?), (?))
        }
    }

    query! {
        async fn delete_thread_data(id: String) -> Result<()> {
            DELETE FROM agent_threads WHERE id = (?)
        }
    }

    pub async fn get_thread(&self, id: ThreadId) -> Result<Option<SerializedThread>> {
        let id_str = id.to_string();
        let result = self.get_thread_data(id_str).await?;

        match result {
            Some(json_str) => {
                let thread = SerializedThread::from_json(json_str.as_bytes())?;
                Ok(Some(thread))
            }
            None => Ok(None),
        }
    }

    pub async fn save_thread(&self, id: ThreadId, thread: SerializedThread) -> Result<()> {
        let thread_json = serde_json::to_string(&thread)?;
        let updated_at = thread.updated_at.timestamp();
        let id_str = id.to_string();
        let summary = thread.summary.clone();

        self.save_thread_data(id_str, summary.to_string(), updated_at, thread_json)
            .await
    }

    pub async fn delete_thread_by_id(&self, id: ThreadId) -> Result<()> {
        let id_str = id.to_string();
        self.delete_thread_data(id_str).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use gpui::TestAppContext;
    use std::sync::Arc;
    use tempfile::TempDir;

    #[gpui::test]
    async fn test_save_load_delete_threads(_cx: &mut TestAppContext) {
        let db = ThreadStoreDB::open_test_db("test_save_load_delete_threads").await;

        // Test that no threads exist initially
        let threads = db.all_threads().await.unwrap();
        assert_eq!(threads.len(), 0);

        // Create test thread data
        let thread_id = ThreadId::from("test-thread-1");
        let thread = SerializedThread {
            version: SerializedThread::VERSION.to_string(),
            summary: SharedString::from("Test thread summary"),
            updated_at: Utc::now(),
            messages: vec![],
            initial_project_snapshot: None,
            cumulative_token_usage: TokenUsage::default(),
            request_token_usage: vec![],
            detailed_summary_state: DetailedSummaryState::NotGenerated,
            exceeded_window_error: None,
            model: None,
            completion_mode: Some(CompletionMode::Normal),
            tool_use_limit_reached: false,
        };

        // Save thread
        db.save_thread(thread_id.clone(), thread.clone())
            .await
            .unwrap();

        // Load all threads
        let threads = db.all_threads().await.unwrap();
        assert_eq!(threads.len(), 1);
        assert_eq!(threads[0].id, thread_id);
        assert_eq!(threads[0].summary, thread.summary);

        // Load specific thread
        let loaded_thread = db.get_thread(thread_id.clone()).await.unwrap();
        assert!(loaded_thread.is_some());
        let loaded_thread = loaded_thread.unwrap();
        assert_eq!(loaded_thread.summary, thread.summary);
        assert_eq!(loaded_thread.version, thread.version);

        // Update thread
        let updated_thread = SerializedThread {
            summary: SharedString::from("Updated summary"),
            updated_at: Utc::now(),
            ..thread
        };
        db.save_thread(thread_id.clone(), updated_thread.clone())
            .await
            .unwrap();

        // Verify update
        let loaded_thread = db.get_thread(thread_id.clone()).await.unwrap().unwrap();
        assert_eq!(loaded_thread.summary, SharedString::from("Updated summary"));

        // Delete thread
        db.delete_thread_by_id(thread_id.clone()).await.unwrap();

        // Verify deletion
        let loaded_thread = db.get_thread(thread_id.clone()).await.unwrap();
        assert!(loaded_thread.is_none());

        let threads = db.all_threads().await.unwrap();
        assert_eq!(threads.len(), 0);
    }

    #[gpui::test]
    async fn test_multiple_threads(_cx: &mut TestAppContext) {
        let db = ThreadStoreDB::open_test_db("test_multiple_threads").await;

        // Create multiple threads
        let thread_ids = [
            ThreadId::from("thread-1"),
            ThreadId::from("thread-2"),
            ThreadId::from("thread-3"),
        ];

        for (i, thread_id) in thread_ids.iter().enumerate() {
            let thread = SerializedThread {
                version: SerializedThread::VERSION.to_string(),
                summary: SharedString::from(format!("Thread {}", i + 1)),
                updated_at: Utc::now() - chrono::Duration::hours(i as i64),
                messages: vec![],
                initial_project_snapshot: None,
                cumulative_token_usage: TokenUsage::default(),
                request_token_usage: vec![],
                detailed_summary_state: DetailedSummaryState::NotGenerated,
                exceeded_window_error: None,
                model: None,
                completion_mode: Some(CompletionMode::Normal),
                tool_use_limit_reached: false,
            };
            db.save_thread(thread_id.clone(), thread).await.unwrap();
        }

        // Load all threads - should be ordered by updated_at DESC
        let threads = db.all_threads().await.unwrap();
        assert_eq!(threads.len(), 3);
        assert_eq!(threads[0].summary.as_ref(), "Thread 1");
        assert_eq!(threads[1].summary.as_ref(), "Thread 2");
        assert_eq!(threads[2].summary.as_ref(), "Thread 3");

        // Delete middle thread
        db.delete_thread_by_id(thread_ids[1].clone()).await.unwrap();

        let threads = db.all_threads().await.unwrap();
        assert_eq!(threads.len(), 2);
        assert_eq!(threads[0].summary.as_ref(), "Thread 1");
        assert_eq!(threads[1].summary.as_ref(), "Thread 3");
    }

    #[gpui::test]
    async fn test_heed_to_sqlite_migration(_cx: &mut TestAppContext) {
        use heed::types::SerdeBincode;

        // Create a temporary directory for the heed database
        let temp_dir = TempDir::new().unwrap();
        let heed_path = temp_dir.path().join("test-heed-db");

        // Create and populate heed database
        {
            std::fs::create_dir_all(&heed_path).unwrap();
            let env = unsafe {
                heed::EnvOpenOptions::new()
                    .map_size(1024 * 1024 * 1024)
                    .max_dbs(1)
                    .open(&heed_path)
                    .unwrap()
            };

            let mut txn = env.write_txn().unwrap();
            let threads: heed::Database<SerdeBincode<ThreadId>, SerializedThread> =
                env.create_database(&mut txn, Some("threads")).unwrap();

            // Insert test data
            let thread_ids = [
                ThreadId::from("legacy-thread-1"),
                ThreadId::from("legacy-thread-2"),
                ThreadId::from("legacy-thread-3"),
            ];

            for (i, thread_id) in thread_ids.iter().enumerate() {
                let thread = SerializedThread {
                    version: SerializedThread::VERSION.to_string(),
                    summary: SharedString::from(format!("Legacy Thread {}", i + 1)),
                    updated_at: Utc::now() - chrono::Duration::days(i as i64),
                    messages: vec![SerializedMessage {
                        id: MessageId(i),
                        role: Role::User,
                        segments: vec![SerializedMessageSegment::Text {
                            text: format!("Test message {}", i),
                        }],
                        tool_uses: vec![],
                        tool_results: vec![],
                        context: String::new(),
                        creases: vec![],
                        is_hidden: false,
                    }],
                    initial_project_snapshot: None,
                    cumulative_token_usage: TokenUsage {
                        input_tokens: ((i + 1) * 100) as u32,
                        output_tokens: ((i + 1) * 50) as u32,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    request_token_usage: vec![],
                    detailed_summary_state: DetailedSummaryState::NotGenerated,
                    exceeded_window_error: None,
                    model: None,
                    completion_mode: Some(CompletionMode::Normal),
                    tool_use_limit_reached: false,
                };
                threads.put(&mut txn, thread_id, &thread).unwrap();
            }

            txn.commit().unwrap();
        }

        // Clear any existing SQLite data
        let db = ThreadStoreDB::open_test_db("test_heed_to_sqlite_migration").await;

        // Verify SQLite is empty
        let threads_before = db.all_threads().await.unwrap();
        assert_eq!(threads_before.len(), 0);

        // Run migration
        ThreadsDatabase::migrate_from_heed_to_db(&heed_path, &db)
            .await
            .unwrap();

        // Verify all threads were migrated
        let threads_after = db.all_threads().await.unwrap();
        assert_eq!(threads_after.len(), 3);

        // Verify thread metadata
        let thread_summaries: Vec<_> = threads_after.iter().map(|t| t.summary.as_ref()).collect();
        assert!(thread_summaries.contains(&"Legacy Thread 1"));
        assert!(thread_summaries.contains(&"Legacy Thread 2"));
        assert!(thread_summaries.contains(&"Legacy Thread 3"));

        // Verify full thread data
        for i in 1..=3 {
            let thread_id = ThreadId::from(&format!("legacy-thread-{}", i) as &str);
            let thread = db.get_thread(thread_id).await.unwrap().unwrap();
            assert_eq!(thread.summary.as_ref(), format!("Legacy Thread {}", i));
            assert_eq!(thread.messages.len(), 1);
            assert_eq!(
                thread.messages[0].segments[0],
                SerializedMessageSegment::Text {
                    text: format!("Test message {}", i - 1)
                }
            );
            assert_eq!(thread.cumulative_token_usage.input_tokens, (i * 100) as u32);
            assert_eq!(thread.cumulative_token_usage.output_tokens, (i * 50) as u32);
        }

        // Verify heed database still exists
        assert!(heed_path.exists());
    }

    #[gpui::test]
    async fn test_thread_serialization_deserialization(_cx: &mut TestAppContext) {
        let db = ThreadStoreDB::open_test_db("test_thread_serialization_deserialization").await;

        let thread_id = ThreadId::from("serialization-test");
        let original_thread = SerializedThread {
            version: SerializedThread::VERSION.to_string(),
            summary: SharedString::from("Serialization test thread"),
            updated_at: Utc::now(),
            messages: vec![
                SerializedMessage {
                    id: MessageId(1),
                    role: Role::User,
                    segments: vec![
                        SerializedMessageSegment::Text {
                            text: "Hello".to_string(),
                        },
                        SerializedMessageSegment::Thinking {
                            text: "Thinking about the response".to_string(),
                            signature: Some("sig123".to_string()),
                        },
                    ],
                    tool_uses: vec![SerializedToolUse {
                        id: LanguageModelToolUseId::from("tool-1"),
                        name: SharedString::from("test_tool"),
                        input: serde_json::json!({"key": "value"}),
                    }],
                    tool_results: vec![SerializedToolResult {
                        tool_use_id: LanguageModelToolUseId::from("tool-1"),
                        is_error: false,
                        content: LanguageModelToolResultContent::Text("Result".into()),
                        output: None,
                    }],
                    context: String::new(),
                    creases: vec![SerializedCrease {
                        start: 0,
                        end: 5,
                        icon_path: SharedString::from("icon.png"),
                        label: SharedString::from("test-crease"),
                    }],
                    is_hidden: false,
                },
                SerializedMessage {
                    id: MessageId(2),
                    role: Role::Assistant,
                    segments: vec![SerializedMessageSegment::RedactedThinking {
                        data: vec![1, 2, 3, 4, 5],
                    }],
                    tool_uses: vec![],
                    tool_results: vec![],
                    context: String::new(),
                    creases: vec![],
                    is_hidden: true,
                },
            ],
            initial_project_snapshot: Some(Arc::new(ProjectSnapshot {
                worktree_snapshots: vec![],
                unsaved_buffer_paths: vec![],
                timestamp: Utc::now(),
            })),
            cumulative_token_usage: TokenUsage {
                input_tokens: 1000,
                output_tokens: 500,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            },
            request_token_usage: vec![TokenUsage {
                input_tokens: 100,
                output_tokens: 50,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            }],
            detailed_summary_state: DetailedSummaryState::Generated {
                text: SharedString::from("Detailed summary"),
                message_id: MessageId(1),
            },
            exceeded_window_error: None,
            model: Some(SerializedLanguageModel {
                provider: "test-provider".to_string(),
                model: "test-model".to_string(),
            }),
            completion_mode: Some(CompletionMode::Normal),
            tool_use_limit_reached: true,
        };

        // Save thread
        db.save_thread(thread_id.clone(), original_thread.clone())
            .await
            .unwrap();

        // Load thread
        let loaded_thread = db.get_thread(thread_id).await.unwrap().unwrap();

        // Verify all fields
        assert_eq!(loaded_thread.version, original_thread.version);
        assert_eq!(loaded_thread.summary, original_thread.summary);
        assert_eq!(loaded_thread.messages.len(), original_thread.messages.len());
        assert_eq!(loaded_thread.messages[0].segments.len(), 2);
        assert_eq!(loaded_thread.messages[0].tool_uses.len(), 1);
        assert_eq!(loaded_thread.messages[0].tool_results.len(), 1);
        assert_eq!(loaded_thread.messages[0].creases.len(), 1);
        assert_eq!(loaded_thread.messages[1].is_hidden, true);
        assert!(loaded_thread.initial_project_snapshot.is_some());
        assert_eq!(
            loaded_thread.cumulative_token_usage.input_tokens,
            original_thread.cumulative_token_usage.input_tokens
        );
        assert_eq!(loaded_thread.exceeded_window_error.is_none(), original_thread.exceeded_window_error.is_none());
        assert!(loaded_thread.model.is_some());
        assert_eq!(loaded_thread.tool_use_limit_reached, true);
    }
}
