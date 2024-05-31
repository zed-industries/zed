use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use collections::HashMap;
use editor::Editor;
use futures::{
    future::{self, BoxFuture, Shared},
    FutureExt,
};
use fuzzy::StringMatchCandidate;
use gpui::{
    actions, point, size, AppContext, BackgroundExecutor, Bounds, DevicePixels, EventEmitter,
    Global, PromptLevel, ReadGlobal, Subscription, Task, TitlebarOptions, View, WindowBounds,
    WindowHandle, WindowOptions,
};
use heed::{types::SerdeBincode, Database, RoTxn};
use language::{language_settings::SoftWrap, Buffer, LanguageRegistry};
use parking_lot::Mutex;
use picker::{Picker, PickerDelegate};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    future::Future,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{
    div, prelude::*, IconButtonShape, ListItem, ListItemSpacing, ParentElement, Render,
    SharedString, Styled, TitleBar, Tooltip, ViewContext, VisualContext,
};
use util::{paths::PROMPTS_DIR, ResultExt};
use uuid::Uuid;

actions!(
    prompt_library,
    [NewPrompt, SavePrompt, DeletePrompt, ToggleDefaultPrompt]
);

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

/// This function opens a new prompt library window if one doesn't exist already.
/// If one exists, it brings it to the foreground.
///
/// Note that, when opening a new window, this waits for the PromptStore to be
/// initialized. If it was initialized successfully, it returns a window handle
/// to a prompt library.
pub fn open_prompt_library(
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AppContext,
) -> Task<Result<WindowHandle<PromptLibrary>>> {
    let existing_window = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<PromptLibrary>());
    if let Some(existing_window) = existing_window {
        existing_window
            .update(cx, |_, cx| cx.activate_window())
            .ok();
        Task::ready(Ok(existing_window))
    } else {
        let store = PromptStore::global(cx);
        cx.spawn(|cx| async move {
            let store = store.await?;
            cx.update(|cx| {
                let bounds = Bounds::centered(
                    None,
                    size(DevicePixels::from(1024), DevicePixels::from(768)),
                    cx,
                );
                cx.open_window(
                    WindowOptions {
                        titlebar: Some(TitlebarOptions {
                            title: None,
                            appears_transparent: true,
                            traffic_light_position: Some(point(px(9.0), px(9.0))),
                        }),
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        ..Default::default()
                    },
                    |cx| cx.new_view(|cx| PromptLibrary::new(store, language_registry, cx)),
                )
            })
        })
    }
}

pub struct PromptLibrary {
    store: Arc<PromptStore>,
    language_registry: Arc<LanguageRegistry>,
    prompt_editors: HashMap<PromptId, View<Editor>>,
    active_prompt_id: Option<PromptId>,
    picker: View<Picker<PromptPickerDelegate>>,
    pending_load: Task<()>,
    _subscriptions: Vec<Subscription>,
}

struct PromptPickerDelegate {
    store: Arc<PromptStore>,
    selected_index: usize,
    matches: Vec<PromptMetadata>,
}

enum PromptPickerEvent {
    Confirmed { prompt_id: PromptId },
    Deleted { prompt_id: PromptId },
}

impl EventEmitter<PromptPickerEvent> for Picker<PromptPickerDelegate> {}

impl PickerDelegate for PromptPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search = self.store.search(query);
        cx.spawn(|this, mut cx| async move {
            let matches = search.await;
            this.update(&mut cx, |this, cx| {
                this.delegate.selected_index = 0;
                this.delegate.matches = matches;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(prompt) = self.matches.get(self.selected_index) {
            cx.emit(PromptPickerEvent::Confirmed {
                prompt_id: prompt.id,
            });
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let prompt = self.matches.get(ix)?;
        let prompt_id = prompt.id;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(Label::new(
                    prompt.title.clone().unwrap_or("Untitled".into()),
                ))
                .end_hover_slot(
                    IconButton::new("delete-prompt", IconName::Trash)
                        .shape(IconButtonShape::Square)
                        .tooltip(move |cx| Tooltip::text("Delete Prompt", cx))
                        .on_click(cx.listener(move |_, _, cx| {
                            cx.emit(PromptPickerEvent::Deleted { prompt_id })
                        })),
                ),
        )
    }
}

impl PromptLibrary {
    fn new(
        store: Arc<PromptStore>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = PromptPickerDelegate {
            store: store.clone(),
            selected_index: 0,
            matches: Vec::new(),
        };

        let picker = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx)
                .modal(false)
                .max_height(None);
            picker.focus(cx);
            picker
        });
        let mut this = Self {
            store: store.clone(),
            language_registry,
            prompt_editors: HashMap::default(),
            active_prompt_id: None,
            pending_load: Task::ready(()),
            _subscriptions: vec![cx.subscribe(&picker, Self::handle_picker_event)],
            picker,
        };
        if let Some(prompt_id) = store.most_recently_saved() {
            this.load_prompt(prompt_id, false, cx);
        }
        this
    }

    fn handle_picker_event(
        &mut self,
        _: View<Picker<PromptPickerDelegate>>,
        event: &PromptPickerEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            PromptPickerEvent::Confirmed { prompt_id } => {
                self.load_prompt(*prompt_id, true, cx);
            }
            PromptPickerEvent::Deleted { prompt_id } => {
                self.delete_prompt(*prompt_id, cx);
            }
        }
    }

    pub fn new_prompt(&mut self, cx: &mut ViewContext<Self>) {
        let prompt_id = PromptId::new();
        let save = self.store.save(prompt_id, None, false, "".into());
        self.picker.update(cx, |picker, cx| picker.refresh(cx));
        cx.spawn(|this, mut cx| async move {
            save.await?;
            this.update(&mut cx, |this, cx| this.load_prompt(prompt_id, true, cx))
        })
        .detach_and_log_err(cx);
    }

    pub fn save_active_prompt(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            let prompt_metadata = self.store.metadata(active_prompt_id).unwrap();
            let body = self
                .prompt_editors
                .get_mut(&active_prompt_id)
                .unwrap()
                .update(cx, |editor, cx| editor.snapshot(cx));

            let title = title_from_body(body.buffer_chars_at(0).map(|(c, _)| c));
            self.store
                .save(
                    active_prompt_id,
                    title,
                    prompt_metadata.default,
                    body.text(),
                )
                .detach_and_log_err(cx);
            self.picker.update(cx, |picker, cx| picker.refresh(cx));
            cx.notify();
        }
    }

    pub fn delete_active_prompt(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            self.delete_prompt(active_prompt_id, cx);
        }
    }

    pub fn toggle_default_for_active_prompt(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            let prompt_metadata = self.store.metadata(active_prompt_id).unwrap();
            let body = self
                .prompt_editors
                .get_mut(&active_prompt_id)
                .unwrap()
                .update(cx, |editor, cx| editor.snapshot(cx));

            let title = title_from_body(body.buffer_chars_at(0).map(|(c, _)| c));
            self.store
                .save(
                    active_prompt_id,
                    title,
                    !prompt_metadata.default,
                    body.text(),
                )
                .detach_and_log_err(cx);
            self.picker.update(cx, |picker, cx| picker.refresh(cx));
            cx.notify();
        }
    }

    pub fn load_prompt(&mut self, prompt_id: PromptId, focus: bool, cx: &mut ViewContext<Self>) {
        if let Some(prompt_editor) = self.prompt_editors.get(&prompt_id) {
            if focus {
                prompt_editor.update(cx, |editor, cx| editor.focus(cx));
            }
            self.active_prompt_id = Some(prompt_id);
        } else {
            let language_registry = self.language_registry.clone();
            let prompt = self.store.load(prompt_id);
            self.pending_load = cx.spawn(|this, mut cx| async move {
                let prompt = prompt.await;
                let markdown = language_registry.language_for_name("Markdown").await;
                this.update(&mut cx, |this, cx| match prompt {
                    Ok(prompt) => {
                        let buffer = cx.new_model(|cx| {
                            let mut buffer = Buffer::local(prompt, cx);
                            buffer.set_language(markdown.log_err(), cx);
                            buffer.set_language_registry(language_registry);
                            buffer
                        });
                        let editor = cx.new_view(|cx| {
                            let mut editor = Editor::for_buffer(buffer, None, cx);
                            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                            editor.set_show_gutter(false, cx);
                            editor.set_show_wrap_guides(false, cx);
                            editor.set_show_indent_guides(false, cx);
                            if focus {
                                editor.focus(cx);
                            }
                            editor
                        });
                        this.prompt_editors.insert(prompt_id, editor);
                        this.active_prompt_id = Some(prompt_id);
                        cx.notify();
                    }
                    Err(error) => {
                        // todo!("show an error in the UI somewhere")
                    }
                })
                .ok();
            });
        }
    }

    pub fn delete_prompt(&mut self, prompt_id: PromptId, cx: &mut ViewContext<Self>) {
        if let Some(metadata) = self.store.metadata(prompt_id) {
            let confirmation = cx.prompt(
                PromptLevel::Warning,
                &format!(
                    "Are you sure you want to delete {}",
                    metadata.title.unwrap_or("Untitled".into())
                ),
                None,
                &["Delete", "Cancel"],
            );

            cx.spawn(|this, mut cx| async move {
                if confirmation.await.ok() == Some(0) {
                    this.update(&mut cx, |this, cx| {
                        if this.active_prompt_id == Some(prompt_id) {
                            this.active_prompt_id = None;
                        }
                        this.prompt_editors.remove(&prompt_id);
                        this.store.delete(prompt_id).detach_and_log_err(cx);
                        this.picker.update(cx, |picker, cx| picker.refresh(cx));
                        cx.notify();
                    })?;
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn render_prompt_list(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("prompt-list")
            .bg(cx.theme().colors().surface_background)
            .h_full()
            .w_1_3()
            .overflow_x_hidden()
            .child(
                h_flex()
                    .bg(cx.theme().colors().background)
                    .p(Spacing::Small.rems(cx))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .h(TitleBar::height(cx))
                    .w_full()
                    .flex_none()
                    .justify_end()
                    .child(
                        IconButton::new("new-prompt", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .tooltip(move |cx| Tooltip::for_action("New Prompt", &NewPrompt, cx))
                            .on_click(|_, cx| {
                                cx.dispatch_action(Box::new(NewPrompt));
                            }),
                    ),
            )
            .child(div().flex_grow().child(self.picker.clone()))
    }

    fn render_active_prompt(&mut self, cx: &mut ViewContext<PromptLibrary>) -> gpui::Stateful<Div> {
        div()
            .w_2_3()
            .h_full()
            .id("prompt-editor")
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .flex_none()
            .min_w_64()
            .children(self.active_prompt_id.and_then(|prompt_id| {
                let prompt_metadata = self.store.metadata(prompt_id)?;
                let editor = self.prompt_editors[&prompt_id].clone();
                Some(
                    v_flex()
                        .size_full()
                        .child(
                            h_flex()
                                .h(TitleBar::height(cx))
                                .px(Spacing::Large.rems(cx))
                                .justify_between()
                                .child(
                                    Label::new(prompt_metadata.title.unwrap_or("Untitled".into()))
                                        .size(LabelSize::Large),
                                )
                                .child(
                                    h_flex()
                                        .gap_4()
                                        .child(
                                            IconButton::new("save-prompt", IconName::Save)
                                                .shape(IconButtonShape::Square)
                                                .tooltip(move |cx| {
                                                    Tooltip::for_action(
                                                        "Save Prompt",
                                                        &SavePrompt,
                                                        cx,
                                                    )
                                                })
                                                .on_click(|_, cx| {
                                                    cx.dispatch_action(Box::new(SavePrompt));
                                                }),
                                        )
                                        .child(
                                            IconButton::new(
                                                "toggle-default-prompt",
                                                if prompt_metadata.default {
                                                    IconName::StarFilled
                                                } else {
                                                    IconName::Star
                                                },
                                            )
                                            .shape(IconButtonShape::Square)
                                            .tooltip(move |cx| {
                                                Tooltip::for_action(
                                                    if prompt_metadata.default {
                                                        "Remove from Default Prompt"
                                                    } else {
                                                        "Add to Default Prompt"
                                                    },
                                                    &ToggleDefaultPrompt,
                                                    cx,
                                                )
                                            })
                                            .on_click(
                                                |_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        ToggleDefaultPrompt,
                                                    ));
                                                },
                                            ),
                                        )
                                        .child(
                                            IconButton::new("delete-prompt", IconName::Trash)
                                                .shape(IconButtonShape::Square)
                                                .tooltip(move |cx| {
                                                    Tooltip::for_action(
                                                        "Delete Prompt",
                                                        &DeletePrompt,
                                                        cx,
                                                    )
                                                })
                                                .on_click(|_, cx| {
                                                    cx.dispatch_action(Box::new(DeletePrompt));
                                                }),
                                        ),
                                ),
                        )
                        .child(div().flex_grow().p(Spacing::Large.rems(cx)).child(editor)),
                )
            }))
    }
}

impl Render for PromptLibrary {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .id("prompt-manager")
            .key_context("PromptLibrary")
            .on_action(cx.listener(|this, &NewPrompt, cx| this.new_prompt(cx)))
            .on_action(cx.listener(|this, &SavePrompt, cx| this.save_active_prompt(cx)))
            .on_action(cx.listener(|this, &DeletePrompt, cx| this.delete_active_prompt(cx)))
            .on_action(cx.listener(|this, &ToggleDefaultPrompt, cx| {
                this.toggle_default_for_active_prompt(cx)
            }))
            .size_full()
            .overflow_hidden()
            .child(self.render_prompt_list(cx))
            .child(self.render_active_prompt(cx))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub id: PromptId,
    pub title: Option<SharedString>,
    pub default: bool,
    pub saved_at: DateTime<Utc>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PromptId(Uuid);

impl PromptId {
    pub fn new() -> PromptId {
        PromptId(Uuid::new_v4())
    }
}

pub struct PromptStore {
    executor: BackgroundExecutor,
    env: heed::Env,
    bodies: Database<SerdeBincode<PromptId>, SerdeBincode<String>>,
    metadata: Database<SerdeBincode<PromptId>, SerdeBincode<PromptMetadata>>,
    metadata_cache: Mutex<MetadataCache>,
}

#[derive(Default)]
struct MetadataCache {
    metadata: Vec<PromptMetadata>,
    metadata_by_id: HashMap<PromptId, PromptMetadata>,
}

impl MetadataCache {
    fn from_db(
        db: Database<SerdeBincode<PromptId>, SerdeBincode<PromptMetadata>>,
        txn: &RoTxn,
    ) -> Result<Self> {
        let mut cache = MetadataCache::default();
        for result in db.iter(txn)? {
            let (prompt_id, metadata) = result?;
            cache.metadata.push(metadata.clone());
            cache.metadata_by_id.insert(prompt_id, metadata);
        }
        cache
            .metadata
            .sort_unstable_by_key(|metadata| Reverse(metadata.saved_at));
        Ok(cache)
    }

    fn insert(&mut self, metadata: PromptMetadata) {
        self.metadata_by_id.insert(metadata.id, metadata.clone());
        if let Some(old_metadata) = self.metadata.iter_mut().find(|m| m.id == metadata.id) {
            *old_metadata = metadata;
        } else {
            self.metadata.push(metadata);
        }
        self.metadata.sort_by_key(|m| Reverse(m.saved_at));
    }

    fn remove(&mut self, id: PromptId) {
        self.metadata.retain(|metadata| metadata.id != id);
        self.metadata_by_id.remove(&id);
    }
}

impl PromptStore {
    pub fn global(cx: &AppContext) -> impl Future<Output = Result<Arc<Self>>> {
        let store = GlobalPromptStore::global(cx).0.clone();
        async move { store.await.map_err(|err| anyhow!(err)) }
    }

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
                let metadata_cache = MetadataCache::from_db(metadata, &txn)?;
                txn.commit()?;

                Ok(PromptStore {
                    executor,
                    env: db_env,
                    bodies,
                    metadata,
                    metadata_cache: Mutex::new(metadata_cache),
                })
            }
        })
    }

    pub fn load(&self, id: PromptId) -> Task<Result<String>> {
        let env = self.env.clone();
        let bodies = self.bodies;
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            Ok(bodies
                .get(&txn, &id)?
                .ok_or_else(|| anyhow!("prompt not found"))?)
        })
    }

    pub fn delete(&self, id: PromptId) -> Task<Result<()>> {
        self.metadata_cache.lock().remove(id);

        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.delete(&mut txn, &id)?;
            bodies.delete(&mut txn, &id)?;

            txn.commit()?;
            Ok(())
        })
    }

    fn metadata(&self, id: PromptId) -> Option<PromptMetadata> {
        self.metadata_cache.lock().metadata_by_id.get(&id).cloned()
    }

    pub fn id_for_title(&self, title: &str) -> Option<PromptId> {
        let metadata_cache = self.metadata_cache.lock();
        let metadata = metadata_cache
            .metadata
            .iter()
            .find(|metadata| metadata.title.as_ref().map(|title| &***title) == Some(title))?;
        Some(metadata.id)
    }

    pub fn search(&self, query: String) -> Task<Vec<PromptMetadata>> {
        let cached_metadata = self.metadata_cache.lock().metadata.clone();
        let executor = self.executor.clone();
        self.executor.spawn(async move {
            if query.is_empty() {
                cached_metadata
            } else {
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
            }
        })
    }

    fn save(
        &self,
        id: PromptId,
        title: Option<SharedString>,
        default: bool,
        body: String,
    ) -> Task<Result<()>> {
        let prompt_metadata = PromptMetadata {
            id,
            title,
            default,
            saved_at: Utc::now(),
        };
        self.metadata_cache.lock().insert(prompt_metadata.clone());

        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.put(&mut txn, &id, &prompt_metadata)?;
            bodies.put(&mut txn, &id, &body)?;

            txn.commit()?;
            Ok(())
        })
    }

    fn most_recently_saved(&self) -> Option<PromptId> {
        self.metadata_cache
            .lock()
            .metadata
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
