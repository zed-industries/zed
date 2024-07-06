use crate::{Context, SavedContext, SavedContextMetadata};
use anyhow::Result;
use client::{telemetry::Telemetry, Client};
use clock::ReplicaId;
use fs::Fs;
use futures::StreamExt;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, Context as _, Model, ModelContext, Task, WeakModel};
use language::LanguageRegistry;
use paths::contexts_dir;
use project::Project;
use regex::Regex;
use std::{
    cmp::Reverse,
    ffi::OsStr,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use util::{ResultExt, TryFutureExt};

pub struct ContextStore {
    contexts: Vec<ContextHandle>,
    contexts_metadata: Vec<SavedContextMetadata>,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    telemetry: Option<Arc<Telemetry>>,
    _watch_updates: Task<Option<()>>,
    client: Arc<Client>,
    project: Model<Project>,
    project_is_shared: bool,
    client_subscription: Option<client::Subscription>,
    _project_subscription: gpui::Subscription,
}

enum ContextHandle {
    Weak(WeakModel<Context>),
    Strong(Model<Context>),
}

impl ContextHandle {
    fn upgrade(&self) -> Option<Model<Context>> {
        match self {
            ContextHandle::Weak(weak) => weak.upgrade(),
            ContextHandle::Strong(strong) => Some(strong.clone()),
        }
    }

    fn downgrade(&self) -> WeakModel<Context> {
        match self {
            ContextHandle::Weak(weak) => weak.clone(),
            ContextHandle::Strong(strong) => strong.downgrade(),
        }
    }
}

impl ContextStore {
    pub fn new(
        project: Model<Project>,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            const CONTEXT_WATCH_DURATION: Duration = Duration::from_millis(100);
            let (mut events, _) = fs.watch(contexts_dir(), CONTEXT_WATCH_DURATION).await;

            let this = cx.new_model(|cx: &mut ModelContext<Self>| {
                let mut this = Self {
                    contexts: Vec::new(),
                    contexts_metadata: Vec::new(),
                    fs,
                    languages,
                    telemetry,
                    _watch_updates: cx.spawn(|this, mut cx| {
                        async move {
                            while events.next().await.is_some() {
                                this.update(&mut cx, |this, cx| this.reload(cx))?
                                    .await
                                    .log_err();
                            }
                            anyhow::Ok(())
                        }
                        .log_err()
                    }),
                    client_subscription: None,
                    _project_subscription: cx.observe(&project, Self::project_changed),
                    project_is_shared: false,
                    client: project.read(cx).client(),
                    project: project.clone(),
                };
                this.register_handlers();
                this.project_changed(project, cx);
                this
            })?;
            this.update(&mut cx, |this, cx| this.reload(cx))?
                .await
                .log_err();
            Ok(this)
        })
    }

    fn register_handlers(&self) {
        todo!();
        // self.client
        //     .add_model_request_handler(Self::handle_open_context);
        // self.client
        //     .add_model_request_handler(Self::handle_context_update);
        // self.client.add_model_request_handler(Self::handle_resync);
    }

    fn project_changed(&mut self, _: Model<Project>, cx: &mut ModelContext<Self>) {
        let is_shared = self.project.read(cx).is_shared();
        let was_shared = mem::replace(&mut self.project_is_shared, is_shared);
        if is_shared == was_shared {
            return;
        }

        if is_shared {
            self.contexts.retain_mut(|context| {
                *context = ContextHandle::Weak(context.downgrade());
                true
            });
            let remote_id = self.project.read(cx).remote_id().unwrap();
            self.client_subscription = self
                .client
                .subscribe_to_entity(remote_id)
                .log_err()
                .map(|subscription| subscription.set_model(&cx.handle(), &mut cx.to_async()));
        } else {
            self.contexts.retain_mut(|context| {
                if let Some(strong_context) = context.upgrade() {
                    *context = ContextHandle::Strong(strong_context);
                    true
                } else {
                    false
                }
            });
            self.client_subscription = None;
        }
    }

    pub fn create(&mut self, cx: &mut ModelContext<Self>) -> Model<Context> {
        let context = cx.new_model(|cx| {
            Context::new(
                ReplicaId::default(),
                self.languages.clone(),
                self.telemetry.clone(),
                cx,
            )
        });
        self.register_context(&context);
        context
    }

    pub fn load(&mut self, path: PathBuf, cx: &ModelContext<Self>) -> Task<Result<Model<Context>>> {
        if let Some(existing_context) = self.loaded_context_for_path(&path, cx) {
            return Task::ready(Ok(existing_context));
        }

        let fs = self.fs.clone();
        let languages = self.languages.clone();
        let telemetry = self.telemetry.clone();
        let load = cx.background_executor().spawn({
            let path = path.clone();
            async move {
                let saved_context = fs.load(&path).await?;
                SavedContext::from_json(&saved_context)
            }
        });

        cx.spawn(|this, mut cx| async move {
            let saved_context = load.await?;
            let context =
                Context::deserialize(saved_context, path.clone(), languages, telemetry, &mut cx)
                    .await?;
            this.update(&mut cx, |this, cx| {
                if let Some(existing_context) = this.loaded_context_for_path(&path, cx) {
                    existing_context
                } else {
                    this.register_context(&context);
                    context
                }
            })
        })
    }

    fn loaded_context_for_path(&self, path: &Path, cx: &AppContext) -> Option<Model<Context>> {
        self.contexts.iter().find_map(|context| {
            let context = context.upgrade()?;
            if context.read(cx).path() == Some(path) {
                Some(context)
            } else {
                None
            }
        })
    }

    fn register_context(&mut self, context: &Model<Context>) {
        let handle = if self.project_is_shared {
            ContextHandle::Strong(context.clone())
        } else {
            ContextHandle::Weak(context.downgrade())
        };
        self.contexts.push(handle);
    }

    pub fn search(&self, query: String, cx: &AppContext) -> Task<Vec<SavedContextMetadata>> {
        let metadata = self.contexts_metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_executor().spawn(async move {
            if query.is_empty() {
                metadata
            } else {
                let candidates = metadata
                    .iter()
                    .enumerate()
                    .map(|(id, metadata)| StringMatchCandidate::new(id, metadata.title.clone()))
                    .collect::<Vec<_>>();
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    executor,
                )
                .await;

                matches
                    .into_iter()
                    .map(|mat| metadata[mat.candidate_id].clone())
                    .collect()
            }
        })
    }

    fn reload(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            fs.create_dir(contexts_dir()).await?;

            let mut paths = fs.read_dir(contexts_dir()).await?;
            let mut contexts = Vec::<SavedContextMetadata>::new();
            while let Some(path) = paths.next().await {
                let path = path?;
                if path.extension() != Some(OsStr::new("json")) {
                    continue;
                }

                let pattern = r" - \d+.zed.json$";
                let re = Regex::new(pattern).unwrap();

                let metadata = fs.metadata(&path).await?;
                if let Some((file_name, metadata)) = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .zip(metadata)
                {
                    // This is used to filter out contexts saved by the new assistant.
                    if !re.is_match(file_name) {
                        continue;
                    }

                    if let Some(title) = re.replace(file_name, "").lines().next() {
                        contexts.push(SavedContextMetadata {
                            title: title.to_string(),
                            path,
                            mtime: metadata.mtime.into(),
                        });
                    }
                }
            }
            contexts.sort_unstable_by_key(|context| Reverse(context.mtime));

            this.update(&mut cx, |this, cx| {
                this.contexts_metadata = contexts;
                cx.notify();
            })
        })
    }
}
