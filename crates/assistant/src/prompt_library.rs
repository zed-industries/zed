use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use collections::HashMap;
use editor::Editor;
use futures::{
    future::{self, BoxFuture, Shared},
    FutureExt,
};
use gpui::{
    uniform_list, AppContext, BackgroundExecutor, Global, ReadGlobal, Task, View, WindowHandle,
    WindowOptions,
};
use heed::{types::SerdeBincode, Database};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    cmp::Reverse,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use ui::{
    div, prelude::IntoElement, ParentElement, Render, SharedString, Styled, StyledExt, ViewContext,
    VisualContext,
};
use util::{paths::PROMPTS_DIR, ResultExt};
use uuid::Uuid;

/// Init starts loading the PromptStore in the background and assigns
/// a shared future to a global.
pub fn init(cx: &mut AppContext) {
    let db_path = PROMPTS_DIR.join("prompts-library-db.0.mdb");
    let prompt_store_future = PromptStore::new(db_path, cx.background_executor().clone())
        .then(|result| future::ready(result.map(Arc::new).map_err(Arc::new)))
        .boxed()
        .shared();
    cx.set_global(GlobalPromptStore(prompt_store_future))
}

/// This method waits for the PromptStore to be initialized.
/// If it was initialized successfully, it returns a window handle to a prompt library.
pub fn open_prompt_library(cx: &mut AppContext) -> Task<Result<WindowHandle<PromptLibrary>>> {
    let store = GlobalPromptStore::global(cx).0.clone();
    cx.spawn(|cx| async move {
        let store = store.await.map_err(|e| anyhow!(e))?;
        Ok(cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| PromptLibrary::new(store, cx))
        })?)
    })
}

pub struct PromptLibrary {
    metadata: Vec<PromptMetadata>,
    store: Arc<PromptStore>,
    prompt_editors: HashMap<PromptId, View<Editor>>,
    active_prompt: Option<PromptId>,
}

impl PromptLibrary {
    pub fn new(store: Arc<PromptStore>, cx: &mut ViewContext<Self>) -> Self {
        // Fetch metadata from database in background upon construction
        let metadata = store.load_metadata();
        cx.spawn(|this, mut cx| async move {
            let mut metadata = metadata.await.log_err()?;
            metadata.sort_by_key(|m| m.saved_at);
            this.update(&mut cx, |this, cx| {
                this.active_prompt = metadata.first().map(|m| m.id);
                this.metadata = metadata;
                cx.notify();
            })
            .ok()
        })
        .detach();

        Self {
            store,
            metadata: Vec::new(),
            prompt_editors: HashMap::default(),
            active_prompt: None,
        }
    }

    pub fn new_prompt(&mut self, cx: &mut ViewContext<Self>) {
        let id = PromptId::new();
        let editor = cx.new_view(|cx| Editor::multi_line(cx));
        self.prompt_editors.insert(id, editor);
        self.active_prompt = Some(id);
        cx.notify();
    }

    pub fn save_active_prompt(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt {
            let body = self
                .prompt_editors
                .get_mut(&active_prompt_id)
                .unwrap()
                .update(cx, |editor, cx| editor.snapshot(cx));

            let title = title_from_body(body.buffer_chars_at(0).map(|(c, _)| c));
            self.store
                .save_prompt(active_prompt_id, title.clone(), body.text())
                .detach_and_log_err(cx);

            let metadata = self
                .metadata
                .iter_mut()
                .find(|m| m.id == active_prompt_id)
                .unwrap();
            metadata.saved_at = Utc::now();
            metadata.title = title;
            self.metadata.sort_by_key(|m| Reverse(m.saved_at));
            cx.notify();
        }
    }

    fn render_list(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        uniform_list(
            cx.view().clone(),
            "prompt-list",
            self.metadata.len(),
            |view, range, _cx| {
                view.metadata[range]
                    .iter()
                    .map(|metadata| {
                        metadata
                            .title
                            .clone()
                            .unwrap_or_else(|| SharedString::from("Untitled"))
                    })
                    .collect()
            },
        )
    }

    fn active_prompt_editor(&self) -> Option<View<Editor>> {
        self.active_prompt
            .map(|id| self.prompt_editors.get(&id).unwrap().clone())
    }
}

impl Render for PromptLibrary {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .h_flex()
            .child(self.render_list(cx))
            .children(self.active_prompt_editor())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub id: PromptId,
    pub title: Option<SharedString>,
    pub saved_at: DateTime<Utc>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PromptId(Uuid);

impl PromptId {
    pub fn new() -> PromptId {
        PromptId(Uuid::new_v4())
    }
}

struct PromptStore {
    executor: BackgroundExecutor,
    env: heed::Env,
    bodies: Database<SerdeBincode<PromptId>, SerdeBincode<String>>,
    metadata: Database<SerdeBincode<PromptId>, SerdeBincode<PromptMetadata>>,
}

impl PromptStore {
    pub fn new(db_path: PathBuf, executor: BackgroundExecutor) -> Task<Result<Self>> {
        executor.spawn({
            let executor = executor.clone();
            async move {
                let db_env = unsafe {
                    heed::EnvOpenOptions::new()
                        .map_size(1024 * 1024 * 1024) // 1GB
                        .max_dbs(1)
                        .open(db_path)?
                };

                let mut txn = db_env.write_txn()?;
                let bodies = db_env.create_database(&mut txn, Some("bodies"))?;
                let metadata = db_env.create_database(&mut txn, Some("metadata"))?;
                txn.commit()?;
                Ok(PromptStore {
                    executor,
                    env: db_env,
                    bodies,
                    metadata,
                })
            }
        })
    }
}

impl PromptStore {
    fn load_metadata(&self) -> Task<Result<Vec<PromptMetadata>>> {
        let env = self.env.clone();
        let metadata = self.metadata.clone();
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let iter = metadata.iter(&txn)?;
            Ok(iter
                .map(|result| Ok(result?.1))
                .collect::<Result<Vec<_>>>()?)
        })
    }

    fn load_prompt_body(&self, id: PromptId) -> Task<Result<String>> {
        let env = self.env.clone();
        let bodies = self.bodies;
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            Ok(bodies
                .get(&txn, &id)?
                .ok_or_else(|| anyhow!("prompt not found"))?)
        })
    }

    fn save_prompt(
        &self,
        id: PromptId,
        title: Option<SharedString>,
        body: String,
    ) -> Task<Result<()>> {
        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.put(
                &mut txn,
                &id,
                &PromptMetadata {
                    id,
                    title,
                    saved_at: Utc::now(),
                },
            )?;
            bodies.put(&mut txn, &id, &body)?;

            txn.commit()?;
            Ok(())
        })
    }
}

/// Wraps a shared future to a prompt store so it can be assigned as a context global.
pub struct GlobalPromptStore(
    Shared<BoxFuture<'static, Result<Arc<PromptStore>, Arc<anyhow::Error>>>>,
);

impl Global for GlobalPromptStore {}

fn title_from_body<'a>(body: impl IntoIterator<Item = char>) -> Option<SharedString> {
    let mut chars = body.into_iter().take_while(|c| *c != '\n').peekable();

    let mut level = 0;
    while let Some('#') = chars.peek() {
        level += 1;
        chars.next();
    }

    if level > 0 {
        Some(chars.collect::<String>().trim().to_string().into())
    } else {
        None
    }
}
