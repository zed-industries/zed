use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use collections::HashMap;
use editor::Editor;
use futures::{
    future::{self, BoxFuture, Shared},
    FutureExt,
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, BackgroundExecutor, Global, ReadGlobal, Task, View, WindowHandle,
    WindowOptions,
};
use heed::{types::SerdeBincode, Database};
use parking_lot::Mutex;
use picker::{Picker, PickerDelegate};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{
    div, prelude::*, IconButtonShape, ListItem, ListItemSpacing, ParentElement, Render,
    SharedString, Styled, StyledExt, Tooltip, ViewContext, VisualContext,
};
use util::paths::PROMPTS_DIR;
use uuid::Uuid;

actions!(prompt_library, [NewPrompt, SavePrompt]);

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
    store: Arc<PromptStore>,
    prompt_editors: HashMap<PromptId, View<Editor>>,
    active_prompt: Option<PromptId>,
    picker: View<Picker<PromptPickerDelegate>>,
}

struct PromptPickerDelegate {
    store: Arc<PromptStore>,
    selected_index: usize,
    matches: Vec<PromptMetadata>,
}

impl PickerDelegate for PromptPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search prompts".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search = self.store.search(query);
        cx.spawn(|this, mut cx| async move {
            let matches = search.await;
            this.update(&mut cx, |this, cx| {
                this.delegate.matches = matches;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        todo!()
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let prompt = self.matches.get(ix)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(Label::new(
                    prompt.title.clone().unwrap_or("Untitled".into()),
                )),
        )
        // .end_slot(div().when(is_dirty, |this| this.child(Indicator::dot()))),
    }
}

impl PromptLibrary {
    pub fn new(store: Arc<PromptStore>, cx: &mut ViewContext<Self>) -> Self {
        let delegate = PromptPickerDelegate {
            store: store.clone(),
            selected_index: 0,
            matches: Vec::new(),
        };
        let active_prompt = store.most_recently_saved();
        Self {
            store,
            prompt_editors: HashMap::default(),
            active_prompt,
            picker: cx.new_view(|cx| Picker::uniform_list(delegate, cx).modal(false)),
        }
    }

    pub fn new_prompt(&mut self, cx: &mut ViewContext<Self>) {
        let id = PromptId::new();
        self.store.save(id, None, "".into()).detach_and_log_err(cx);

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
                .save(active_prompt_id, title.clone(), body.text())
                .detach_and_log_err(cx);

            cx.notify();
        }
    }

    fn render_prompt_list(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let picker = self.picker.clone();

        v_flex()
            .id("prompt-list")
            .bg(cx.theme().colors().surface_background)
            .h_full()
            .w_1_3()
            .overflow_hidden()
            .child(
                h_flex()
                    .bg(cx.theme().colors().background)
                    .p(Spacing::Small.rems(cx))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .h(rems(1.75))
                    .w_full()
                    .flex_none()
                    .justify_between()
                    .child(Label::new("Prompt Library").size(LabelSize::Small))
                    .child(
                        IconButton::new("new-prompt", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .tooltip(move |cx| Tooltip::text("New Prompt", cx))
                            .on_click(|_, cx| {
                                // cx.dispatch_action(NewPrompt.boxed_clone());
                            }),
                    ),
            )
            .child(
                v_flex()
                    .h(rems(38.25))
                    .flex_grow()
                    .justify_start()
                    .child(picker),
            )
    }

    fn active_prompt_editor(&self) -> Option<View<Editor>> {
        self.active_prompt
            .map(|id| self.prompt_editors.get(&id).unwrap().clone())
    }
}

impl Render for PromptLibrary {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .id("prompt-manager")
            .key_context("PromptLibrary")
            // .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, &NewPrompt, cx| this.new_prompt(cx)))
            .on_action(cx.listener(|this, &SavePrompt, cx| this.save_active_prompt(cx)))
            .elevation_3(cx)
            .size_full()
            .flex_none()
            .overflow_hidden()
            .child(self.render_prompt_list(cx))
            .child(
                div()
                    .w_2_3()
                    .h_full()
                    .id("prompt-editor")
                    .border_l_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .size_full()
                    .flex_none()
                    .min_w_64()
                    .h_full()
                    .overflow_hidden()
                    .children(self.active_prompt_editor().map(|editor| {
                        div()
                            .flex_1()
                            .py(Spacing::Large.rems(cx))
                            .px(Spacing::XLarge.rems(cx))
                            .child(editor)
                    })),
            )
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
    cached_metadata: Mutex<Vec<PromptMetadata>>,
}

impl PromptStore {
    pub fn new(db_path: PathBuf, executor: BackgroundExecutor) -> Task<Result<Self>> {
        executor.spawn({
            let executor = executor.clone();
            async move {
                std::fs::create_dir_all(&db_path)?;

                let db_env = unsafe {
                    heed::EnvOpenOptions::new()
                        .map_size(1024 * 1024 * 1024) // 1GB
                        .max_dbs(2) // bodies and metadata
                        .open(db_path)?
                };

                let mut txn = db_env.write_txn()?;
                let bodies = db_env.create_database(&mut txn, Some("bodies"))?;
                let metadata = db_env.create_database(&mut txn, Some("metadata"))?;

                let iter = metadata.iter(&txn)?;
                let cached_metadata = iter
                    .map(|result| Ok(result?.1))
                    .collect::<Result<Vec<_>>>()?;

                txn.commit()?;

                Ok(PromptStore {
                    executor,
                    env: db_env,
                    bodies,
                    metadata,
                    cached_metadata: Mutex::new(cached_metadata),
                })
            }
        })
    }

    fn count(&self) -> usize {
        self.cached_metadata.lock().len()
    }

    fn load(&self, id: PromptId) -> Task<Result<String>> {
        let env = self.env.clone();
        let bodies = self.bodies;
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            Ok(bodies
                .get(&txn, &id)?
                .ok_or_else(|| anyhow!("prompt not found"))?)
        })
    }

    fn search(&self, query: String) -> Task<Vec<PromptMetadata>> {
        let cached_metadata = self.cached_metadata.lock().clone();
        let executor = self.executor.clone();
        self.executor.spawn(async move {
            let candidates = cached_metadata
                .iter()
                .enumerate()
                .filter_map(|(ix, metadata)| {
                    Some(StringMatchCandidate::new(
                        ix,
                        metadata.title.as_ref()?.to_string(),
                    ))
                })
                .collect::<Vec<_>>();
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                100,
                &AtomicBool::default(),
                executor,
            )
            .await;

            matches
                .into_iter()
                .map(|mat| cached_metadata[mat.candidate_id].clone())
                .collect()
        })
    }

    fn save(&self, id: PromptId, title: Option<SharedString>, body: String) -> Task<Result<()>> {
        let mut cached_metadata = self.cached_metadata.lock();
        if let Some(metadata) = cached_metadata.iter_mut().find(|m| m.id == id) {
            metadata.saved_at = Utc::now();
            metadata.title = title.clone();
        } else {
            cached_metadata.push(PromptMetadata {
                id,
                title: title.clone(),
                saved_at: Utc::now(),
            })
        }
        cached_metadata.sort_by_key(|m| Reverse(m.saved_at));

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

    fn most_recently_saved(&self) -> Option<PromptId> {
        self.cached_metadata
            .lock()
            .first()
            .map(|metadata| metadata.id)
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
