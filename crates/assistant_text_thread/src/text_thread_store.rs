use crate::{SavedTextThread, SavedTextThreadMetadata, TextThread, TextThreadEvent, TextThreadId};
use anyhow::Result;
use assistant_slash_command::{SlashCommandId, SlashCommandWorkingSet};
use collections::HashMap;
use context_server::ContextServerId;
use fs::{Fs, RemoveOptions};
use futures::StreamExt;
use fuzzy::StringMatchCandidate;
use gpui::{App, AppContext as _, Context, Entity, Task, TaskExt, WeakEntity};
use itertools::Itertools;
use language::LanguageRegistry;
use project::{
    Project,
    context_server_store::{ContextServerStatus, ContextServerStore, ServerStatusChangedEvent},
};
use prompt_store::PromptBuilder;
use regex::Regex;
use rpc::AnyProtoClient;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::{cmp::Reverse, ffi::OsStr, sync::Arc, time::Duration};
use util::ResultExt;
use zed_env_vars::ZED_STATELESS;

pub(crate) fn init(_client: &AnyProtoClient) {}

pub struct TextThreadStore {
    text_threads: Vec<TextThreadHandle>,
    text_threads_metadata: Vec<SavedTextThreadMetadata>,
    context_server_slash_command_ids: HashMap<ContextServerId, Vec<SlashCommandId>>,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    slash_commands: Arc<SlashCommandWorkingSet>,
    _watch_updates: Task<Option<()>>,
    project: WeakEntity<Project>,
    _project_subscriptions: Vec<gpui::Subscription>,
    prompt_builder: Arc<PromptBuilder>,
}

enum TextThreadHandle {
    Weak(WeakEntity<TextThread>),
}

impl TextThreadHandle {
    fn upgrade(&self) -> Option<Entity<TextThread>> {
        match self {
            TextThreadHandle::Weak(weak) => weak.upgrade(),
        }
    }
}

impl TextThreadStore {
    pub fn new(
        project: Entity<Project>,
        prompt_builder: Arc<PromptBuilder>,
        slash_commands: Arc<SlashCommandWorkingSet>,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let fs = project.read(cx).fs().clone();
        let languages = project.read(cx).languages().clone();
        cx.spawn(async move |cx| {
            const WATCH_DURATION: Duration = Duration::from_millis(100);
            let text_threads_dir = text_threads_dir();
            let (mut events, _) = fs.watch(&text_threads_dir, WATCH_DURATION).await;

            let store = cx.new(|cx: &mut Context<Self>| {
                let mut store = Self {
                    text_threads: Vec::new(),
                    text_threads_metadata: Vec::new(),
                    context_server_slash_command_ids: HashMap::default(),
                    fs,
                    languages,
                    slash_commands,
                    _watch_updates: cx.spawn(async move |this, cx| {
                        while events.next().await.is_some() {
                            if let Some(reload_task) =
                                this.update(cx, |this, cx| this.reload(cx)).log_err()
                            {
                                reload_task.await.log_err();
                            }
                        }
                        None
                    }),
                    project: project.downgrade(),
                    _project_subscriptions: Vec::new(),
                    prompt_builder,
                };
                store.register_context_server_handlers(cx);
                store.reload(cx).detach_and_log_err(cx);
                store
            });

            Ok(store)
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        Self {
            text_threads: Default::default(),
            text_threads_metadata: Default::default(),
            context_server_slash_command_ids: Default::default(),
            fs: project.read(cx).fs().clone(),
            languages: project.read(cx).languages().clone(),
            slash_commands: Arc::default(),
            _watch_updates: Task::ready(None),
            project: project.downgrade(),
            _project_subscriptions: Default::default(),
            prompt_builder: Arc::new(PromptBuilder::new(None).expect("prompt builder")),
        }
    }

    pub fn ordered_text_threads(&self) -> impl Iterator<Item = &SavedTextThreadMetadata> {
        self.text_threads_metadata
            .iter()
            .sorted_by(|left, right| right.mtime.cmp(&left.mtime))
    }

    pub fn has_saved_text_threads(&self) -> bool {
        !self.text_threads_metadata.is_empty()
    }

    pub fn create(&mut self, cx: &mut Context<Self>) -> Entity<TextThread> {
        let text_thread = cx.new(|cx| {
            TextThread::local(
                self.languages.clone(),
                self.prompt_builder.clone(),
                self.slash_commands.clone(),
                cx,
            )
        });
        self.register_text_thread(&text_thread, cx);
        text_thread
    }

    pub fn open_local(
        &mut self,
        path: Arc<Path>,
        cx: &Context<Self>,
    ) -> Task<Result<Entity<TextThread>>> {
        if let Some(existing) = self.loaded_text_thread_for_path(&path, cx) {
            return Task::ready(Ok(existing));
        }

        let fs = self.fs.clone();
        let languages = self.languages.clone();
        let load = cx.background_spawn({
            let path = path.clone();
            async move {
                let saved_text_thread = fs.load(&path).await?;
                SavedTextThread::from_json(&saved_text_thread)
            }
        });
        let prompt_builder = self.prompt_builder.clone();
        let slash_commands = self.slash_commands.clone();

        cx.spawn(async move |this, cx| {
            let saved_text_thread = load.await?;
            let text_thread = cx.new(|cx| {
                TextThread::deserialize(
                    saved_text_thread,
                    path.clone(),
                    languages,
                    prompt_builder,
                    slash_commands,
                    cx,
                )
            });
            this.update(cx, |this, cx| {
                if let Some(existing) = this.loaded_text_thread_for_path(&path, cx) {
                    existing
                } else {
                    this.register_text_thread(&text_thread, cx);
                    text_thread
                }
            })
        })
    }

    pub fn delete_local(&mut self, path: Arc<Path>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();

        cx.spawn(async move |this, cx| {
            fs.remove_file(
                &path,
                RemoveOptions {
                    recursive: false,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

            this.update(cx, |this, cx| {
                this.text_threads.retain(|text_thread| {
                    text_thread
                        .upgrade()
                        .and_then(|text_thread| text_thread.read(cx).path())
                        != Some(&path)
                });
                this.text_threads_metadata
                    .retain(|text_thread| text_thread.path.as_ref() != path.as_ref());
                cx.notify();
            })?;

            Ok(())
        })
    }

    pub fn delete_all_local(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        let paths = self
            .text_threads_metadata
            .iter()
            .map(|metadata| metadata.path.clone())
            .collect::<Vec<_>>();

        cx.spawn(async move |this, cx| {
            for path in paths {
                fs.remove_file(
                    &path,
                    RemoveOptions {
                        recursive: false,
                        ignore_if_not_exists: true,
                    },
                )
                .await?;
            }

            this.update(cx, |this, cx| {
                this.text_threads.clear();
                this.text_threads_metadata.clear();
                cx.notify();
            })?;

            Ok(())
        })
    }

    pub fn loaded_text_thread_for_id(
        &self,
        id: &TextThreadId,
        cx: &App,
    ) -> Option<Entity<TextThread>> {
        self.text_threads.iter().find_map(|text_thread| {
            let text_thread = text_thread.upgrade()?;
            if text_thread.read(cx).id() == id {
                Some(text_thread)
            } else {
                None
            }
        })
    }

    pub fn search(&self, query: String, cx: &App) -> Task<Vec<SavedTextThreadMetadata>> {
        let metadata = self.text_threads_metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
            if query.is_empty() {
                metadata
            } else {
                let candidates = metadata
                    .iter()
                    .enumerate()
                    .map(|(id, metadata)| StringMatchCandidate::new(id, &metadata.title))
                    .collect::<Vec<_>>();
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    executor,
                )
                .await;

                matches
                    .into_iter()
                    .map(|matched| metadata[matched.candidate_id].clone())
                    .collect()
            }
        })
    }

    fn loaded_text_thread_for_path(&self, path: &Path, cx: &App) -> Option<Entity<TextThread>> {
        self.text_threads.iter().find_map(|text_thread| {
            let text_thread = text_thread.upgrade()?;
            if text_thread.read(cx).path().map(Arc::as_ref) == Some(path) {
                Some(text_thread)
            } else {
                None
            }
        })
    }

    fn register_text_thread(&mut self, text_thread: &Entity<TextThread>, cx: &mut Context<Self>) {
        self.text_threads
            .push(TextThreadHandle::Weak(text_thread.downgrade()));
        cx.subscribe(text_thread, Self::handle_text_thread_event)
            .detach();
    }

    fn handle_text_thread_event(
        &mut self,
        _text_thread: Entity<TextThread>,
        event: &TextThreadEvent,
        cx: &mut Context<Self>,
    ) {
        if let TextThreadEvent::PathChanged { old_path, new_path } = event
            && let Some(old_path) = old_path.as_ref()
        {
            for metadata in &mut self.text_threads_metadata {
                if &metadata.path == old_path {
                    metadata.path = new_path.clone();
                    metadata.title = title_for_path(new_path).into();
                    break;
                }
            }
            cx.notify();
        }
    }

    fn reload(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        cx.spawn(async move |this, cx| {
            if *ZED_STATELESS {
                return Ok(());
            }
            let text_threads_dir = text_threads_dir();
            fs.create_dir(&text_threads_dir).await?;

            let mut paths = fs.read_dir(&text_threads_dir).await?;
            let mut text_threads = Vec::<SavedTextThreadMetadata>::new();
            while let Some(path) = paths.next().await {
                let path = path?;
                if path.extension() != Some(OsStr::new("json")) {
                    continue;
                }

                let metadata = fs.metadata(&path).await?;
                if let Some(metadata) = metadata {
                    let title = title_for_path(&path);
                    if !TEXT_THREAD_PATH_REGEX.is_match(&path.to_string_lossy()) {
                        continue;
                    }
                    text_threads.push(SavedTextThreadMetadata {
                        title: title.into(),
                        path: path.into(),
                        mtime: metadata.mtime.timestamp_for_user().into(),
                    });
                }
            }
            text_threads.sort_unstable_by_key(|text_thread| Reverse(text_thread.mtime));

            this.update(cx, |this, cx| {
                this.text_threads_metadata = text_threads;
                cx.notify();
            })
        })
    }

    fn register_context_server_handlers(&mut self, cx: &mut Context<Self>) {
        let Some(project) = self.project.upgrade() else {
            return;
        };
        let context_server_store = project.read(cx).context_server_store();
        self._project_subscriptions
            .push(cx.subscribe(&context_server_store, Self::handle_context_server_event));

        for server in context_server_store.read(cx).running_servers() {
            self.load_context_server_slash_commands(server.id(), context_server_store.clone(), cx);
        }
    }

    fn handle_context_server_event(
        &mut self,
        context_server_store: Entity<ContextServerStore>,
        event: &ServerStatusChangedEvent,
        cx: &mut Context<Self>,
    ) {
        match &event.status {
            ContextServerStatus::Running => {
                self.load_context_server_slash_commands(
                    event.server_id.clone(),
                    context_server_store,
                    cx,
                );
            }
            ContextServerStatus::Stopped | ContextServerStatus::Error(_) => {
                if let Some(slash_command_ids) = self
                    .context_server_slash_command_ids
                    .remove(&event.server_id)
                {
                    self.slash_commands.remove(&slash_command_ids);
                }
            }
            _ => {}
        }
    }

    fn load_context_server_slash_commands(
        &self,
        server_id: ContextServerId,
        context_server_store: Entity<ContextServerStore>,
        cx: &mut Context<Self>,
    ) {
        let Some(server) = context_server_store.read(cx).get_running_server(&server_id) else {
            return;
        };
        let slash_command_working_set = self.slash_commands.clone();
        cx.spawn(async move |this, cx| {
            let Some(protocol) = server.client() else {
                return;
            };

            if protocol.capable(context_server::protocol::ServerCapability::Prompts)
                && let Some(response) = protocol
                    .request::<context_server::types::requests::PromptsList>(())
                    .await
                    .log_err()
            {
                let slash_command_ids = response
                    .prompts
                    .into_iter()
                    .filter(assistant_slash_commands::acceptable_prompt)
                    .map(|prompt| {
                        slash_command_working_set.insert(Arc::new(
                            assistant_slash_commands::ContextServerSlashCommand::new(
                                context_server_store.clone(),
                                server.id(),
                                prompt,
                            ),
                        ))
                    })
                    .collect::<Vec<_>>();

                this.update(cx, |this, _cx| {
                    this.context_server_slash_command_ids
                        .insert(server_id.clone(), slash_command_ids);
                })
                .log_err();
            }
        })
        .detach();
    }
}

static TEXT_THREAD_PATH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r" - \d+\.zed\.json$").unwrap());

fn text_threads_dir() -> PathBuf {
    paths::state_dir().join("text_threads")
}

fn title_for_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| TEXT_THREAD_PATH_REGEX.replace(name, "").to_string())
        .and_then(|name| name.lines().next().map(ToOwned::to_owned))
        .unwrap_or_else(|| "New Thread".to_string())
}
