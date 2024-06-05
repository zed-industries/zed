use crate::{
    slash_command::SlashCommandCompletionProvider, CompletionProvider, LanguageModelRequest,
    LanguageModelRequestMessage, Role,
};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandRegistry;
use chrono::{DateTime, Utc};
use collections::HashMap;
use editor::{actions::Tab, Editor, EditorEvent};
use futures::{
    future::{self, BoxFuture, Shared},
    FutureExt,
};
use fuzzy::StringMatchCandidate;
use gpui::{
    actions, point, size, AnyElement, AppContext, BackgroundExecutor, Bounds, DevicePixels,
    EventEmitter, Global, PromptLevel, ReadGlobal, Subscription, Task, TitlebarOptions, View,
    WindowBounds, WindowHandle, WindowOptions,
};
use heed::{types::SerdeBincode, Database, RoTxn};
use language::{language_settings::SoftWrap, Buffer, LanguageRegistry};
use parking_lot::RwLock;
use picker::{Picker, PickerDelegate};
use rope::Rope;
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};
use ui::{
    div, prelude::*, IconButtonShape, ListHeader, ListItem, ListItemSpacing, ListSubHeader,
    ParentElement, Render, SharedString, Styled, TitleBar, Tooltip, ViewContext, VisualContext,
};
use util::{paths::PROMPTS_DIR, ResultExt, TryFutureExt};
use uuid::Uuid;

actions!(
    prompt_library,
    [NewPrompt, DeletePrompt, ToggleDefaultPrompt]
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
                            title: Some("Prompt Library".into()),
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
    prompt_editors: HashMap<PromptId, PromptEditor>,
    active_prompt_id: Option<PromptId>,
    picker: View<Picker<PromptPickerDelegate>>,
    pending_load: Task<()>,
    _subscriptions: Vec<Subscription>,
}

struct PromptEditor {
    editor: View<Editor>,
    token_count: Option<usize>,
    pending_token_count: Task<Option<()>>,
    next_body_to_save: Option<Rope>,
    pending_save: Option<Task<Option<()>>>,
    _subscription: Subscription,
}

struct PromptPickerDelegate {
    store: Arc<PromptStore>,
    selected_index: usize,
    entries: Vec<PromptPickerEntry>,
}

enum PromptPickerEvent {
    Selected { prompt_id: PromptId },
    Confirmed { prompt_id: PromptId },
    Deleted { prompt_id: PromptId },
    ToggledDefault { prompt_id: PromptId },
}

#[derive(Debug)]
enum PromptPickerEntry {
    DefaultPromptsHeader,
    DefaultPromptsEmpty,
    AllPromptsHeader,
    AllPromptsEmpty,
    Prompt(PromptMetadata),
}

impl PromptPickerEntry {
    fn prompt_id(&self) -> Option<PromptId> {
        match self {
            PromptPickerEntry::Prompt(metadata) => Some(metadata.id),
            _ => None,
        }
    }
}

impl EventEmitter<PromptPickerEvent> for Picker<PromptPickerDelegate> {}

impl PickerDelegate for PromptPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
        if let Some(PromptPickerEntry::Prompt(prompt)) = self.entries.get(self.selected_index) {
            cx.emit(PromptPickerEvent::Selected {
                prompt_id: prompt.id,
            });
        }
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search = self.store.search(query);
        let prev_prompt_id = self
            .entries
            .get(self.selected_index)
            .and_then(|mat| mat.prompt_id());
        cx.spawn(|this, mut cx| async move {
            let (entries, selected_index) = cx
                .background_executor()
                .spawn(async move {
                    let prompts = search.await;
                    let (default_prompts, prompts) = prompts
                        .into_iter()
                        .partition::<Vec<_>, _>(|prompt| prompt.default);

                    let mut entries = Vec::new();
                    entries.push(PromptPickerEntry::DefaultPromptsHeader);
                    if default_prompts.is_empty() {
                        entries.push(PromptPickerEntry::DefaultPromptsEmpty);
                    } else {
                        entries.extend(default_prompts.into_iter().map(PromptPickerEntry::Prompt));
                    }

                    entries.push(PromptPickerEntry::AllPromptsHeader);
                    if prompts.is_empty() {
                        entries.push(PromptPickerEntry::AllPromptsEmpty);
                    } else {
                        entries.extend(prompts.into_iter().map(PromptPickerEntry::Prompt));
                    }

                    let selected_index = prev_prompt_id
                        .and_then(|prev_prompt_id| {
                            entries
                                .iter()
                                .position(|entry| entry.prompt_id() == Some(prev_prompt_id))
                        })
                        .or_else(|| entries.iter().position(|entry| entry.prompt_id().is_some()))
                        .unwrap_or(0);
                    (entries, selected_index)
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.delegate.entries = entries;
                this.delegate.set_selected_index(selected_index, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(PromptPickerEntry::Prompt(prompt)) = self.entries.get(self.selected_index) {
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
        let prompt = self.entries.get(ix)?;
        let element = match prompt {
            PromptPickerEntry::DefaultPromptsHeader => ListHeader::new("Default Prompts")
                .inset(true)
                .start_slot(Icon::new(IconName::ZedAssistant))
                .selected(selected)
                .into_any_element(),
            PromptPickerEntry::DefaultPromptsEmpty => {
                ListSubHeader::new("Star a prompt to add it to the default context")
                    .inset(true)
                    .selected(selected)
                    .into_any_element()
            }
            PromptPickerEntry::AllPromptsHeader => ListHeader::new("All Prompts")
                .inset(true)
                .start_slot(Icon::new(IconName::Library))
                .selected(selected)
                .into_any_element(),
            PromptPickerEntry::AllPromptsEmpty => ListSubHeader::new("No prompts")
                .inset(true)
                .selected(selected)
                .into_any_element(),
            PromptPickerEntry::Prompt(prompt) => {
                let default = prompt.default;
                let prompt_id = prompt.id;
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .selected(selected)
                    .child(Label::new(
                        prompt.title.clone().unwrap_or("Untitled".into()),
                    ))
                    .end_hover_slot(
                        h_flex()
                            .gap_2()
                            .child(
                                IconButton::new("delete-prompt", IconName::Trash)
                                    .shape(IconButtonShape::Square)
                                    .tooltip(move |cx| Tooltip::text("Delete Prompt", cx))
                                    .on_click(cx.listener(move |_, _, cx| {
                                        cx.emit(PromptPickerEvent::Deleted { prompt_id })
                                    })),
                            )
                            .child(
                                IconButton::new(
                                    "toggle-default-prompt",
                                    if default {
                                        IconName::ZedAssistantFilled
                                    } else {
                                        IconName::ZedAssistant
                                    },
                                )
                                .shape(IconButtonShape::Square)
                                .tooltip(move |cx| {
                                    Tooltip::text(
                                        if default {
                                            "Remove from Default Prompt"
                                        } else {
                                            "Add to Default Prompt"
                                        },
                                        cx,
                                    )
                                })
                                .on_click(cx.listener(
                                    move |_, _, cx| {
                                        cx.emit(PromptPickerEvent::ToggledDefault { prompt_id })
                                    },
                                )),
                            ),
                    )
                    .into_any_element()
            }
        };
        Some(element)
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
            entries: Vec::new(),
        };

        let picker = cx.new_view(|cx| {
            let picker = Picker::list(delegate, cx).modal(false).max_height(None);
            picker.focus(cx);
            picker
        });
        Self {
            store: store.clone(),
            language_registry,
            prompt_editors: HashMap::default(),
            active_prompt_id: None,
            pending_load: Task::ready(()),
            _subscriptions: vec![cx.subscribe(&picker, Self::handle_picker_event)],
            picker,
        }
    }

    fn handle_picker_event(
        &mut self,
        _: View<Picker<PromptPickerDelegate>>,
        event: &PromptPickerEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            PromptPickerEvent::Selected { prompt_id } => {
                self.load_prompt(*prompt_id, false, cx);
            }
            PromptPickerEvent::Confirmed { prompt_id } => {
                self.load_prompt(*prompt_id, true, cx);
            }
            PromptPickerEvent::ToggledDefault { prompt_id } => {
                self.toggle_default_for_prompt(*prompt_id, cx);
            }
            PromptPickerEvent::Deleted { prompt_id } => {
                self.delete_prompt(*prompt_id, cx);
            }
        }
    }

    pub fn new_prompt(&mut self, cx: &mut ViewContext<Self>) {
        // If we already have an untitled prompt, use that instead
        // of creating a new one.
        if let Some(metadata) = self.store.first() {
            if metadata.title.is_none() {
                self.load_prompt(metadata.id, true, cx);
                return;
            }
        }

        let prompt_id = PromptId::new();
        let save = self.store.save(prompt_id, None, false, "".into());
        self.picker.update(cx, |picker, cx| picker.refresh(cx));
        cx.spawn(|this, mut cx| async move {
            save.await?;
            this.update(&mut cx, |this, cx| this.load_prompt(prompt_id, true, cx))
        })
        .detach_and_log_err(cx);
    }

    pub fn save_prompt(&mut self, prompt_id: PromptId, cx: &mut ViewContext<Self>) {
        const SAVE_THROTTLE: Duration = Duration::from_millis(500);

        let prompt_metadata = self.store.metadata(prompt_id).unwrap();
        let prompt_editor = self.prompt_editors.get_mut(&prompt_id).unwrap();
        let body = prompt_editor.editor.update(cx, |editor, cx| {
            editor
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap()
                .read(cx)
                .as_rope()
                .clone()
        });

        let store = self.store.clone();
        let executor = cx.background_executor().clone();

        prompt_editor.next_body_to_save = Some(body);
        if prompt_editor.pending_save.is_none() {
            prompt_editor.pending_save = Some(cx.spawn(|this, mut cx| {
                async move {
                    loop {
                        let next_body_to_save = this.update(&mut cx, |this, _| {
                            this.prompt_editors
                                .get_mut(&prompt_id)?
                                .next_body_to_save
                                .take()
                        })?;

                        if let Some(body) = next_body_to_save {
                            let title = title_from_body(body.chars_at(0));
                            store
                                .save(prompt_id, title, prompt_metadata.default, body)
                                .await
                                .log_err();
                            this.update(&mut cx, |this, cx| {
                                this.picker.update(cx, |picker, cx| picker.refresh(cx));
                                cx.notify();
                            })?;

                            executor.timer(SAVE_THROTTLE).await;
                        } else {
                            break;
                        }
                    }

                    this.update(&mut cx, |this, _cx| {
                        if let Some(prompt_editor) = this.prompt_editors.get_mut(&prompt_id) {
                            prompt_editor.pending_save = None;
                        }
                    })
                }
                .log_err()
            }));
        }
    }

    pub fn delete_active_prompt(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            self.delete_prompt(active_prompt_id, cx);
        }
    }

    pub fn toggle_default_for_active_prompt(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            self.toggle_default_for_prompt(active_prompt_id, cx);
        }
    }

    pub fn toggle_default_for_prompt(&mut self, prompt_id: PromptId, cx: &mut ViewContext<Self>) {
        if let Some(prompt_metadata) = self.store.metadata(prompt_id) {
            self.store
                .save_metadata(prompt_id, prompt_metadata.title, !prompt_metadata.default)
                .detach_and_log_err(cx);
            self.picker.update(cx, |picker, cx| picker.refresh(cx));
            cx.notify();
        }
    }

    pub fn load_prompt(&mut self, prompt_id: PromptId, focus: bool, cx: &mut ViewContext<Self>) {
        if let Some(prompt_editor) = self.prompt_editors.get(&prompt_id) {
            if focus {
                prompt_editor
                    .editor
                    .update(cx, |editor, cx| editor.focus(cx));
            }
            self.set_active_prompt(Some(prompt_id), cx);
        } else {
            let language_registry = self.language_registry.clone();
            let commands = SlashCommandRegistry::global(cx);
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
                            editor.set_completion_provider(Box::new(
                                SlashCommandCompletionProvider::new(commands, None, None),
                            ));
                            if focus {
                                editor.focus(cx);
                            }
                            editor
                        });
                        let _subscription =
                            cx.subscribe(&editor, move |this, _editor, event, cx| {
                                this.handle_prompt_editor_event(prompt_id, event, cx)
                            });
                        this.prompt_editors.insert(
                            prompt_id,
                            PromptEditor {
                                editor,
                                next_body_to_save: None,
                                pending_save: None,
                                token_count: None,
                                pending_token_count: Task::ready(None),
                                _subscription,
                            },
                        );
                        this.set_active_prompt(Some(prompt_id), cx);
                        this.count_tokens(prompt_id, cx);
                    }
                    Err(error) => {
                        // TODO: we should show the error in the UI.
                        log::error!("error while loading prompt: {:?}", error);
                    }
                })
                .ok();
            });
        }
    }

    fn set_active_prompt(&mut self, prompt_id: Option<PromptId>, cx: &mut ViewContext<Self>) {
        self.active_prompt_id = prompt_id;
        self.picker.update(cx, |picker, cx| {
            if let Some(prompt_id) = prompt_id {
                if picker
                    .delegate
                    .entries
                    .get(picker.delegate.selected_index())
                    .map_or(true, |old_selected_prompt| {
                        old_selected_prompt.prompt_id() != Some(prompt_id)
                    })
                {
                    if let Some(ix) = picker
                        .delegate
                        .entries
                        .iter()
                        .position(|mat| mat.prompt_id() == Some(prompt_id))
                    {
                        picker.set_selected_index(ix, true, cx);
                    }
                }
            }
        });
        cx.notify();
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
                            this.set_active_prompt(None, cx);
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

    fn focus_active_prompt(&mut self, _: &Tab, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt) = self.active_prompt_id {
            self.prompt_editors[&active_prompt]
                .editor
                .update(cx, |editor, cx| editor.focus(cx));
            cx.stop_propagation();
        }
    }

    fn focus_picker(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        self.picker.update(cx, |picker, cx| picker.focus(cx));
    }

    fn handle_prompt_editor_event(
        &mut self,
        prompt_id: PromptId,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let EditorEvent::BufferEdited = event {
            let prompt_editor = self.prompt_editors.get(&prompt_id).unwrap();
            let buffer = prompt_editor
                .editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap();

            buffer.update(cx, |buffer, cx| {
                let mut chars = buffer.chars_at(0);
                match chars.next() {
                    Some('#') => {
                        if chars.next() != Some(' ') {
                            drop(chars);
                            buffer.edit([(1..1, " ")], None, cx);
                        }
                    }
                    Some(' ') => {
                        drop(chars);
                        buffer.edit([(0..0, "#")], None, cx);
                    }
                    _ => {
                        drop(chars);
                        buffer.edit([(0..0, "# ")], None, cx);
                    }
                }
            });

            self.save_prompt(prompt_id, cx);
            self.count_tokens(prompt_id, cx);
        }
    }

    fn count_tokens(&mut self, prompt_id: PromptId, cx: &mut ViewContext<Self>) {
        if let Some(prompt) = self.prompt_editors.get_mut(&prompt_id) {
            let editor = &prompt.editor.read(cx);
            let buffer = &editor.buffer().read(cx).as_singleton().unwrap().read(cx);
            let body = buffer.as_rope().clone();
            prompt.pending_token_count = cx.spawn(|this, mut cx| {
                async move {
                    const DEBOUNCE_TIMEOUT: Duration = Duration::from_secs(1);

                    cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
                    let token_count = cx
                        .update(|cx| {
                            let provider = CompletionProvider::global(cx);
                            let model = provider.model();
                            provider.count_tokens(
                                LanguageModelRequest {
                                    model,
                                    messages: vec![LanguageModelRequestMessage {
                                        role: Role::System,
                                        content: body.to_string(),
                                    }],
                                    stop: Vec::new(),
                                    temperature: 1.,
                                },
                                cx,
                            )
                        })?
                        .await?;
                    this.update(&mut cx, |this, cx| {
                        let prompt_editor = this.prompt_editors.get_mut(&prompt_id).unwrap();
                        prompt_editor.token_count = Some(token_count);
                        cx.notify();
                    })
                }
                .log_err()
            });
        }
    }

    fn render_prompt_list(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("prompt-list")
            .capture_action(cx.listener(Self::focus_active_prompt))
            .bg(cx.theme().colors().panel_background)
            .h_full()
            .w_1_3()
            .overflow_x_hidden()
            .child(
                h_flex()
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
                let prompt_editor = &self.prompt_editors[&prompt_id];
                Some(
                    h_flex()
                        .size_full()
                        .items_start()
                        .child(
                            div()
                                .on_action(cx.listener(Self::focus_picker))
                                .flex_grow()
                                .h_full()
                                .pt(Spacing::Large.rems(cx))
                                .pl(Spacing::Large.rems(cx))
                                .child(prompt_editor.editor.clone()),
                        )
                        .child(
                            v_flex()
                                .w_12()
                                .py(Spacing::Large.rems(cx))
                                .justify_start()
                                .items_center()
                                .gap_4()
                                .child(
                                    IconButton::new(
                                        "toggle-default-prompt",
                                        if prompt_metadata.default {
                                            IconName::ZedAssistantFilled
                                        } else {
                                            IconName::ZedAssistant
                                        },
                                    )
                                    .size(ButtonSize::Large)
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
                                    .on_click(|_, cx| {
                                        cx.dispatch_action(Box::new(ToggleDefaultPrompt));
                                    }),
                                )
                                .child(
                                    IconButton::new("delete-prompt", IconName::Trash)
                                        .shape(IconButtonShape::Square)
                                        .tooltip(move |cx| {
                                            Tooltip::for_action("Delete Prompt", &DeletePrompt, cx)
                                        })
                                        .on_click(|_, cx| {
                                            cx.dispatch_action(Box::new(DeletePrompt));
                                        }),
                                )
                                .children(prompt_editor.token_count.map(|token_count| {
                                    h_flex()
                                        .justify_center()
                                        .child(Label::new(token_count.to_string()))
                                })),
                        ),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub id: PromptId,
    pub title: Option<SharedString>,
    pub default: bool,
    pub saved_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    metadata_cache: RwLock<MetadataCache>,
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
        cache.sort();
        Ok(cache)
    }

    fn insert(&mut self, metadata: PromptMetadata) {
        self.metadata_by_id.insert(metadata.id, metadata.clone());
        if let Some(old_metadata) = self.metadata.iter_mut().find(|m| m.id == metadata.id) {
            *old_metadata = metadata;
        } else {
            self.metadata.push(metadata);
        }
        self.sort();
    }

    fn remove(&mut self, id: PromptId) {
        self.metadata.retain(|metadata| metadata.id != id);
        self.metadata_by_id.remove(&id);
    }

    fn sort(&mut self) {
        self.metadata.sort_unstable_by(|a, b| {
            a.title
                .cmp(&b.title)
                .then_with(|| b.saved_at.cmp(&a.saved_at))
        });
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
                    metadata_cache: RwLock::new(metadata_cache),
                })
            }
        })
    }

    pub fn load(&self, id: PromptId) -> Task<Result<String>> {
        let env = self.env.clone();
        let bodies = self.bodies;
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            bodies
                .get(&txn, &id)?
                .ok_or_else(|| anyhow!("prompt not found"))
        })
    }

    pub fn default_prompt_metadata(&self) -> Vec<PromptMetadata> {
        return self
            .metadata_cache
            .read()
            .metadata
            .iter()
            .filter(|metadata| metadata.default)
            .cloned()
            .collect::<Vec<_>>();
    }

    pub fn delete(&self, id: PromptId) -> Task<Result<()>> {
        self.metadata_cache.write().remove(id);

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
        self.metadata_cache.read().metadata_by_id.get(&id).cloned()
    }

    pub fn id_for_title(&self, title: &str) -> Option<PromptId> {
        let metadata_cache = self.metadata_cache.read();
        let metadata = metadata_cache
            .metadata
            .iter()
            .find(|metadata| metadata.title.as_ref().map(|title| &***title) == Some(title))?;
        Some(metadata.id)
    }

    pub fn search(&self, query: String) -> Task<Vec<PromptMetadata>> {
        let cached_metadata = self.metadata_cache.read().metadata.clone();
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
        body: Rope,
    ) -> Task<Result<()>> {
        let prompt_metadata = PromptMetadata {
            id,
            title,
            default,
            saved_at: Utc::now(),
        };
        self.metadata_cache.write().insert(prompt_metadata.clone());

        let db_connection = self.env.clone();
        let bodies = self.bodies;
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;

            metadata.put(&mut txn, &id, &prompt_metadata)?;
            bodies.put(&mut txn, &id, &body.to_string())?;

            txn.commit()?;

            Ok(())
        })
    }

    fn save_metadata(
        &self,
        id: PromptId,
        title: Option<SharedString>,
        default: bool,
    ) -> Task<Result<()>> {
        let prompt_metadata = PromptMetadata {
            id,
            title,
            default,
            saved_at: Utc::now(),
        };
        self.metadata_cache.write().insert(prompt_metadata.clone());

        let db_connection = self.env.clone();
        let metadata = self.metadata;

        self.executor.spawn(async move {
            let mut txn = db_connection.write_txn()?;
            metadata.put(&mut txn, &id, &prompt_metadata)?;
            txn.commit()?;

            Ok(())
        })
    }

    fn first(&self) -> Option<PromptMetadata> {
        self.metadata_cache.read().metadata.first().cloned()
    }
}

/// Wraps a shared future to a prompt store so it can be assigned as a context global.
pub struct GlobalPromptStore(
    Shared<BoxFuture<'static, Result<Arc<PromptStore>, Arc<anyhow::Error>>>>,
);

impl Global for GlobalPromptStore {}

fn title_from_body(body: impl IntoIterator<Item = char>) -> Option<SharedString> {
    let mut chars = body.into_iter().take_while(|c| *c != '\n').peekable();

    let mut level = 0;
    while let Some('#') = chars.peek() {
        level += 1;
        chars.next();
    }

    if level > 0 {
        let title = chars.collect::<String>().trim().to_string();
        if title.is_empty() {
            None
        } else {
            Some(title.into())
        }
    } else {
        None
    }
}
