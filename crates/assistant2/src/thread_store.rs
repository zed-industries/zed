use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ToolId, ToolWorkingSet};
use chrono::{DateTime, Utc};
use collections::HashMap;
use context_server::manager::ContextServerManager;
use context_server::{ContextServerFactoryRegistry, ContextServerTool};
use futures::future::{self, BoxFuture, Shared};
use futures::FutureExt as _;
use gpui::{
    prelude::*, App, BackgroundExecutor, Context, Entity, Global, ReadGlobal, SharedString, Task,
};
use heed::types::SerdeBincode;
use heed::Database;
use language_model::Role;
use project::Project;
use serde::{Deserialize, Serialize};
use util::ResultExt as _;

use crate::thread::{MessageId, Thread, ThreadId};

pub fn init(cx: &mut App) {
    ThreadsDatabase::init(cx);
}

pub struct ThreadStore {
    #[allow(unused)]
    project: Entity<Project>,
    tools: Arc<ToolWorkingSet>,
    context_server_manager: Entity<ContextServerManager>,
    context_server_tool_ids: HashMap<Arc<str>, Vec<ToolId>>,
    threads: Vec<SavedThreadMetadata>,
}

impl ThreadStore {
    pub fn new(
        project: Entity<Project>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut App,
    ) -> Result<Entity<Self>> {
        let this = cx.new(|cx| {
            let context_server_factory_registry = ContextServerFactoryRegistry::default_global(cx);
            let context_server_manager = cx.new(|cx| {
                ContextServerManager::new(context_server_factory_registry, project.clone(), cx)
            });

            let this = Self {
                project,
                tools,
                context_server_manager,
                context_server_tool_ids: HashMap::default(),
                threads: Vec::new(),
            };
            this.register_context_server_handlers(cx);
            this.reload(cx).detach_and_log_err(cx);

            this
        });

        Ok(this)
    }

    /// Returns the number of threads.
    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }

    pub fn threads(&self) -> Vec<SavedThreadMetadata> {
        let mut threads = self.threads.iter().cloned().collect::<Vec<_>>();
        threads.sort_unstable_by_key(|thread| std::cmp::Reverse(thread.updated_at));
        threads
    }

    pub fn recent_threads(&self, limit: usize) -> Vec<SavedThreadMetadata> {
        self.threads().into_iter().take(limit).collect()
    }

    pub fn create_thread(&mut self, cx: &mut Context<Self>) -> Entity<Thread> {
        cx.new(|cx| Thread::new(self.tools.clone(), cx))
    }

    pub fn open_thread(
        &self,
        id: &ThreadId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Thread>>> {
        let id = id.clone();
        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(|this, mut cx| async move {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            let thread = database
                .try_find_thread(id.clone())
                .await?
                .ok_or_else(|| anyhow!("no thread found with ID: {id:?}"))?;

            this.update(&mut cx, |this, cx| {
                cx.new(|cx| Thread::from_saved(id.clone(), thread, this.tools.clone(), cx))
            })
        })
    }

    pub fn save_thread(&self, thread: &Entity<Thread>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let (metadata, thread) = thread.update(cx, |thread, _cx| {
            let id = thread.id().clone();
            let thread = SavedThread {
                summary: thread.summary_or_default(),
                updated_at: thread.updated_at(),
                messages: thread
                    .messages()
                    .map(|message| SavedMessage {
                        id: message.id,
                        role: message.role,
                        text: message.text.clone(),
                    })
                    .collect(),
            };

            (id, thread)
        });

        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(|this, mut cx| async move {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.save_thread(metadata, thread).await?;

            this.update(&mut cx, |this, cx| this.reload(cx))?.await
        })
    }

    pub fn delete_thread(&mut self, id: &ThreadId, cx: &mut Context<Self>) -> Task<Result<()>> {
        let id = id.clone();
        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(|this, mut cx| async move {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_thread(id.clone()).await?;

            this.update(&mut cx, |this, _cx| {
                this.threads.retain(|thread| thread.id != id)
            })
        })
    }

    pub fn reload(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::global_future(cx);
        cx.spawn(|this, mut cx| async move {
            let threads = database_future
                .await
                .map_err(|err| anyhow!(err))?
                .list_threads()
                .await?;

            this.update(&mut cx, |this, cx| {
                this.threads = threads;
                cx.notify();
            })
        })
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
                        |this, mut cx| async move {
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

                                    this.update(&mut cx, |this, _cx| {
                                        this.context_server_tool_ids.insert(server_id, tool_ids);
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
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedThreadMetadata {
    pub id: ThreadId,
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct SavedThread {
    pub summary: SharedString,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<SavedMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct SavedMessage {
    pub id: MessageId,
    pub role: Role,
    pub text: String,
}

struct GlobalThreadsDatabase(
    Shared<BoxFuture<'static, Result<Arc<ThreadsDatabase>, Arc<anyhow::Error>>>>,
);

impl Global for GlobalThreadsDatabase {}

pub(crate) struct ThreadsDatabase {
    executor: BackgroundExecutor,
    env: heed::Env,
    threads: Database<SerdeBincode<ThreadId>, SerdeBincode<SavedThread>>,
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
                let database_path = paths::support_dir().join("threads/threads-db.0.mdb");
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

    pub fn list_threads(&self) -> Task<Result<Vec<SavedThreadMetadata>>> {
        let env = self.env.clone();
        let threads = self.threads;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let mut iter = threads.iter(&txn)?;
            let mut threads = Vec::new();
            while let Some((key, value)) = iter.next().transpose()? {
                threads.push(SavedThreadMetadata {
                    id: key,
                    summary: value.summary,
                    updated_at: value.updated_at,
                });
            }

            Ok(threads)
        })
    }

    pub fn try_find_thread(&self, id: ThreadId) -> Task<Result<Option<SavedThread>>> {
        let env = self.env.clone();
        let threads = self.threads;

        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let thread = threads.get(&txn, &id)?;
            Ok(thread)
        })
    }

    pub fn save_thread(&self, id: ThreadId, thread: SavedThread) -> Task<Result<()>> {
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
