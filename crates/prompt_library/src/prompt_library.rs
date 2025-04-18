use anyhow::Result;
use collections::{HashMap, HashSet};
use editor::CompletionProvider;
use editor::{CurrentLineHighlight, Editor, EditorElement, EditorEvent, EditorStyle, actions::Tab};
use gpui::{
    Action, App, Bounds, Entity, EventEmitter, Focusable, PromptLevel, Subscription, Task,
    TextStyle, TitlebarOptions, WindowBounds, WindowHandle, WindowOptions, actions, point, size,
    transparent_black,
};
use language::{Buffer, LanguageRegistry, language_settings::SoftWrap};
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use picker::{Picker, PickerDelegate};
use release_channel::ReleaseChannel;
use rope::Rope;
use settings::Settings;
use std::sync::Arc;
use std::time::Duration;
use theme::ThemeSettings;
use ui::{
    Context, IconButtonShape, KeyBinding, ListItem, ListItemSpacing, ParentElement, Render,
    SharedString, Styled, Tooltip, Window, div, prelude::*,
};
use util::{ResultExt, TryFutureExt};
use workspace::Workspace;
use zed_actions::assistant::InlineAssist;

use prompt_store::*;

pub fn init(cx: &mut App) {
    prompt_store::init(cx);
}

actions!(
    prompt_library,
    [
        NewPrompt,
        DeletePrompt,
        DuplicatePrompt,
        ToggleDefaultPrompt
    ]
);

const BUILT_IN_TOOLTIP_TEXT: &'static str = concat!(
    "This prompt supports special functionality.\n",
    "It's read-only, but you can remove it from your default prompt."
);

pub trait InlineAssistDelegate {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut Context<PromptLibrary>,
    );

    /// Returns whether the Assistant panel was focused.
    fn focus_assistant_panel(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool;
}

/// This function opens a new prompt library window if one doesn't exist already.
/// If one exists, it brings it to the foreground.
///
/// Note that, when opening a new window, this waits for the PromptStore to be
/// initialized. If it was initialized successfully, it returns a window handle
/// to a prompt library.
pub fn open_prompt_library(
    language_registry: Arc<LanguageRegistry>,
    inline_assist_delegate: Box<dyn InlineAssistDelegate>,
    make_completion_provider: Arc<dyn Fn() -> Box<dyn CompletionProvider>>,
    prompt_to_focus: Option<PromptId>,
    cx: &mut App,
) -> Task<Result<WindowHandle<PromptLibrary>>> {
    let store = PromptStore::global(cx);
    cx.spawn(async move |cx| {
        // We query windows in spawn so that all windows have been returned to GPUI
        let existing_window = cx
            .update(|cx| {
                let existing_window = cx
                    .windows()
                    .into_iter()
                    .find_map(|window| window.downcast::<PromptLibrary>());
                if let Some(existing_window) = existing_window {
                    existing_window
                        .update(cx, |prompt_library, window, cx| {
                            if let Some(prompt_to_focus) = prompt_to_focus {
                                prompt_library.load_prompt(prompt_to_focus, true, window, cx);
                            }
                            window.activate_window()
                        })
                        .ok();

                    Some(existing_window)
                } else {
                    None
                }
            })
            .ok()
            .flatten();

        if let Some(existing_window) = existing_window {
            return Ok(existing_window);
        }

        let store = store.await?;
        cx.update(|cx| {
            let app_id = ReleaseChannel::global(cx).app_id();
            let bounds = Bounds::centered(None, size(px(1024.0), px(768.0)), cx);
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Prompt Library".into()),
                        appears_transparent: cfg!(target_os = "macos"),
                        traffic_light_position: Some(point(px(9.0), px(9.0))),
                    }),
                    app_id: Some(app_id.to_owned()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        let mut prompt_library = PromptLibrary::new(
                            store,
                            language_registry,
                            inline_assist_delegate,
                            make_completion_provider,
                            window,
                            cx,
                        );
                        if let Some(prompt_to_focus) = prompt_to_focus {
                            prompt_library.load_prompt(prompt_to_focus, true, window, cx);
                        }
                        prompt_library
                    })
                },
            )
        })?
    })
}

pub struct PromptLibrary {
    store: Entity<PromptStore>,
    language_registry: Arc<LanguageRegistry>,
    prompt_editors: HashMap<PromptId, PromptEditor>,
    active_prompt_id: Option<PromptId>,
    picker: Entity<Picker<PromptPickerDelegate>>,
    pending_load: Task<()>,
    inline_assist_delegate: Box<dyn InlineAssistDelegate>,
    make_completion_provider: Arc<dyn Fn() -> Box<dyn CompletionProvider>>,
    _subscriptions: Vec<Subscription>,
}

struct PromptEditor {
    title_editor: Entity<Editor>,
    body_editor: Entity<Editor>,
    token_count: Option<usize>,
    pending_token_count: Task<Option<()>>,
    next_title_and_body_to_save: Option<(String, Rope)>,
    pending_save: Option<Task<Option<()>>>,
    _subscriptions: Vec<Subscription>,
}

struct PromptPickerDelegate {
    store: Entity<PromptStore>,
    selected_index: usize,
    matches: Vec<PromptMetadata>,
}

enum PromptPickerEvent {
    Selected { prompt_id: PromptId },
    Confirmed { prompt_id: PromptId },
    Deleted { prompt_id: PromptId },
    ToggledDefault { prompt_id: PromptId },
}

impl EventEmitter<PromptPickerEvent> for Picker<PromptPickerDelegate> {}

impl PickerDelegate for PromptPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn no_matches_text(&self, _window: &mut Window, cx: &mut App) -> Option<SharedString> {
        let text = if self.store.read(cx).prompt_count() == 0 {
            "No prompts.".into()
        } else {
            "No prompts found matching your search.".into()
        };
        Some(text)
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        if let Some(prompt) = self.matches.get(self.selected_index) {
            cx.emit(PromptPickerEvent::Selected {
                prompt_id: prompt.id,
            });
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let search = self.store.read(cx).search(query, cx);
        let prev_prompt_id = self.matches.get(self.selected_index).map(|mat| mat.id);
        cx.spawn_in(window, async move |this, cx| {
            let (matches, selected_index) = cx
                .background_spawn(async move {
                    let matches = search.await;

                    let selected_index = prev_prompt_id
                        .and_then(|prev_prompt_id| {
                            matches.iter().position(|entry| entry.id == prev_prompt_id)
                        })
                        .unwrap_or(0);
                    (matches, selected_index)
                })
                .await;

            this.update_in(cx, |this, window, cx| {
                this.delegate.matches = matches;
                this.delegate.set_selected_index(selected_index, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(prompt) = self.matches.get(self.selected_index) {
            cx.emit(PromptPickerEvent::Confirmed {
                prompt_id: prompt.id,
            });
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let prompt = self.matches.get(ix)?;
        let default = prompt.default;
        let prompt_id = prompt.id;
        let element = ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .child(h_flex().h_5().line_height(relative(1.)).child(Label::new(
                prompt.title.clone().unwrap_or("Untitled".into()),
            )))
            .end_slot::<IconButton>(default.then(|| {
                IconButton::new("toggle-default-prompt", IconName::SparkleFilled)
                    .toggle_state(true)
                    .icon_color(Color::Accent)
                    .shape(IconButtonShape::Square)
                    .tooltip(Tooltip::text("Remove from Default Prompt"))
                    .on_click(cx.listener(move |_, _, _, cx| {
                        cx.emit(PromptPickerEvent::ToggledDefault { prompt_id })
                    }))
            }))
            .end_hover_slot(
                h_flex()
                    .gap_2()
                    .child(if prompt_id.is_built_in() {
                        div()
                            .id("built-in-prompt")
                            .child(Icon::new(IconName::FileLock).color(Color::Muted))
                            .tooltip(move |window, cx| {
                                Tooltip::with_meta(
                                    "Built-in prompt",
                                    None,
                                    BUILT_IN_TOOLTIP_TEXT,
                                    window,
                                    cx,
                                )
                            })
                            .into_any()
                    } else {
                        IconButton::new("delete-prompt", IconName::Trash)
                            .icon_color(Color::Muted)
                            .shape(IconButtonShape::Square)
                            .tooltip(Tooltip::text("Delete Prompt"))
                            .on_click(cx.listener(move |_, _, _, cx| {
                                cx.emit(PromptPickerEvent::Deleted { prompt_id })
                            }))
                            .into_any_element()
                    })
                    .child(
                        IconButton::new("toggle-default-prompt", IconName::Sparkle)
                            .toggle_state(default)
                            .selected_icon(IconName::SparkleFilled)
                            .icon_color(if default { Color::Accent } else { Color::Muted })
                            .shape(IconButtonShape::Square)
                            .tooltip(Tooltip::text(if default {
                                "Remove from Default Prompt"
                            } else {
                                "Add to Default Prompt"
                            }))
                            .on_click(cx.listener(move |_, _, _, cx| {
                                cx.emit(PromptPickerEvent::ToggledDefault { prompt_id })
                            })),
                    ),
            );
        Some(element)
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        h_flex()
            .bg(cx.theme().colors().editor_background)
            .rounded_sm()
            .overflow_hidden()
            .flex_none()
            .py_1()
            .px_2()
            .mx_1()
            .child(editor.clone())
    }
}

impl PromptLibrary {
    fn new(
        store: Entity<PromptStore>,
        language_registry: Arc<LanguageRegistry>,
        inline_assist_delegate: Box<dyn InlineAssistDelegate>,
        make_completion_provider: Arc<dyn Fn() -> Box<dyn CompletionProvider>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = PromptPickerDelegate {
            store: store.clone(),
            selected_index: 0,
            matches: Vec::new(),
        };

        let picker = cx.new(|cx| {
            let picker = Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .max_height(None);
            picker.focus(window, cx);
            picker
        });
        Self {
            store: store.clone(),
            language_registry,
            prompt_editors: HashMap::default(),
            active_prompt_id: None,
            pending_load: Task::ready(()),
            inline_assist_delegate,
            make_completion_provider,
            _subscriptions: vec![cx.subscribe_in(&picker, window, Self::handle_picker_event)],
            picker,
        }
    }

    fn handle_picker_event(
        &mut self,
        _: &Entity<Picker<PromptPickerDelegate>>,
        event: &PromptPickerEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            PromptPickerEvent::Selected { prompt_id } => {
                self.load_prompt(*prompt_id, false, window, cx);
            }
            PromptPickerEvent::Confirmed { prompt_id } => {
                self.load_prompt(*prompt_id, true, window, cx);
            }
            PromptPickerEvent::ToggledDefault { prompt_id } => {
                self.toggle_default_for_prompt(*prompt_id, window, cx);
            }
            PromptPickerEvent::Deleted { prompt_id } => {
                self.delete_prompt(*prompt_id, window, cx);
            }
        }
    }

    pub fn new_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // If we already have an untitled prompt, use that instead
        // of creating a new one.
        if let Some(metadata) = self.store.read(cx).first() {
            if metadata.title.is_none() {
                self.load_prompt(metadata.id, true, window, cx);
                return;
            }
        }

        let prompt_id = PromptId::new();
        let save = self.store.update(cx, |store, cx| {
            store.save(prompt_id, None, false, "".into(), cx)
        });
        self.picker
            .update(cx, |picker, cx| picker.refresh(window, cx));
        cx.spawn_in(window, async move |this, cx| {
            save.await?;
            this.update_in(cx, |this, window, cx| {
                this.load_prompt(prompt_id, true, window, cx)
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn save_prompt(
        &mut self,
        prompt_id: PromptId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        const SAVE_THROTTLE: Duration = Duration::from_millis(500);

        if prompt_id.is_built_in() {
            return;
        }

        let prompt_metadata = self.store.read(cx).metadata(prompt_id).unwrap();
        let prompt_editor = self.prompt_editors.get_mut(&prompt_id).unwrap();
        let title = prompt_editor.title_editor.read(cx).text(cx);
        let body = prompt_editor.body_editor.update(cx, |editor, cx| {
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

        prompt_editor.next_title_and_body_to_save = Some((title, body));
        if prompt_editor.pending_save.is_none() {
            prompt_editor.pending_save = Some(cx.spawn_in(window, async move |this, cx| {
                async move {
                    loop {
                        let title_and_body = this.update(cx, |this, _| {
                            this.prompt_editors
                                .get_mut(&prompt_id)?
                                .next_title_and_body_to_save
                                .take()
                        })?;

                        if let Some((title, body)) = title_and_body {
                            let title = if title.trim().is_empty() {
                                None
                            } else {
                                Some(SharedString::from(title))
                            };
                            cx.update(|_window, cx| {
                                store.update(cx, |store, cx| {
                                    store.save(prompt_id, title, prompt_metadata.default, body, cx)
                                })
                            })?
                            .await
                            .log_err();
                            this.update_in(cx, |this, window, cx| {
                                this.picker
                                    .update(cx, |picker, cx| picker.refresh(window, cx));
                                cx.notify();
                            })?;

                            executor.timer(SAVE_THROTTLE).await;
                        } else {
                            break;
                        }
                    }

                    this.update(cx, |this, _cx| {
                        if let Some(prompt_editor) = this.prompt_editors.get_mut(&prompt_id) {
                            prompt_editor.pending_save = None;
                        }
                    })
                }
                .log_err()
                .await
            }));
        }
    }

    pub fn delete_active_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            self.delete_prompt(active_prompt_id, window, cx);
        }
    }

    pub fn duplicate_active_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            self.duplicate_prompt(active_prompt_id, window, cx);
        }
    }

    pub fn toggle_default_for_active_prompt(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            self.toggle_default_for_prompt(active_prompt_id, window, cx);
        }
    }

    pub fn toggle_default_for_prompt(
        &mut self,
        prompt_id: PromptId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.store.update(cx, move |store, cx| {
            if let Some(prompt_metadata) = store.metadata(prompt_id) {
                store
                    .save_metadata(
                        prompt_id,
                        prompt_metadata.title,
                        !prompt_metadata.default,
                        cx,
                    )
                    .detach_and_log_err(cx);
            }
        });
        self.picker
            .update(cx, |picker, cx| picker.refresh(window, cx));
        cx.notify();
    }

    pub fn load_prompt(
        &mut self,
        prompt_id: PromptId,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(prompt_editor) = self.prompt_editors.get(&prompt_id) {
            if focus {
                prompt_editor
                    .body_editor
                    .update(cx, |editor, cx| window.focus(&editor.focus_handle(cx)));
            }
            self.set_active_prompt(Some(prompt_id), window, cx);
        } else if let Some(prompt_metadata) = self.store.read(cx).metadata(prompt_id) {
            let language_registry = self.language_registry.clone();
            let prompt = self.store.read(cx).load(prompt_id, cx);
            let make_completion_provider = self.make_completion_provider.clone();
            self.pending_load = cx.spawn_in(window, async move |this, cx| {
                let prompt = prompt.await;
                let markdown = language_registry.language_for_name("Markdown").await;
                this.update_in(cx, |this, window, cx| match prompt {
                    Ok(prompt) => {
                        let title_editor = cx.new(|cx| {
                            let mut editor = Editor::auto_width(window, cx);
                            editor.set_placeholder_text("Untitled", cx);
                            editor.set_text(prompt_metadata.title.unwrap_or_default(), window, cx);
                            if prompt_id.is_built_in() {
                                editor.set_read_only(true);
                                editor.set_show_edit_predictions(Some(false), window, cx);
                            }
                            editor
                        });
                        let body_editor = cx.new(|cx| {
                            let buffer = cx.new(|cx| {
                                let mut buffer = Buffer::local(prompt, cx);
                                buffer.set_language(markdown.log_err(), cx);
                                buffer.set_language_registry(language_registry);
                                buffer
                            });

                            let mut editor = Editor::for_buffer(buffer, None, window, cx);
                            if prompt_id.is_built_in() {
                                editor.set_read_only(true);
                                editor.set_show_edit_predictions(Some(false), window, cx);
                            }
                            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                            editor.set_show_gutter(false, cx);
                            editor.set_show_wrap_guides(false, cx);
                            editor.set_show_indent_guides(false, cx);
                            editor.set_use_modal_editing(false);
                            editor.set_current_line_highlight(Some(CurrentLineHighlight::None));
                            editor.set_completion_provider(Some(make_completion_provider()));
                            if focus {
                                window.focus(&editor.focus_handle(cx));
                            }
                            editor
                        });
                        let _subscriptions = vec![
                            cx.subscribe_in(
                                &title_editor,
                                window,
                                move |this, editor, event, window, cx| {
                                    this.handle_prompt_title_editor_event(
                                        prompt_id, editor, event, window, cx,
                                    )
                                },
                            ),
                            cx.subscribe_in(
                                &body_editor,
                                window,
                                move |this, editor, event, window, cx| {
                                    this.handle_prompt_body_editor_event(
                                        prompt_id, editor, event, window, cx,
                                    )
                                },
                            ),
                        ];
                        this.prompt_editors.insert(
                            prompt_id,
                            PromptEditor {
                                title_editor,
                                body_editor,
                                next_title_and_body_to_save: None,
                                pending_save: None,
                                token_count: None,
                                pending_token_count: Task::ready(None),
                                _subscriptions,
                            },
                        );
                        this.set_active_prompt(Some(prompt_id), window, cx);
                        this.count_tokens(prompt_id, window, cx);
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

    fn set_active_prompt(
        &mut self,
        prompt_id: Option<PromptId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_prompt_id = prompt_id;
        self.picker.update(cx, |picker, cx| {
            if let Some(prompt_id) = prompt_id {
                if picker
                    .delegate
                    .matches
                    .get(picker.delegate.selected_index())
                    .map_or(true, |old_selected_prompt| {
                        old_selected_prompt.id != prompt_id
                    })
                {
                    if let Some(ix) = picker
                        .delegate
                        .matches
                        .iter()
                        .position(|mat| mat.id == prompt_id)
                    {
                        picker.set_selected_index(ix, None, true, window, cx);
                    }
                }
            } else {
                picker.focus(window, cx);
            }
        });
        cx.notify();
    }

    pub fn delete_prompt(
        &mut self,
        prompt_id: PromptId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(metadata) = self.store.read(cx).metadata(prompt_id) {
            let confirmation = window.prompt(
                PromptLevel::Warning,
                &format!(
                    "Are you sure you want to delete {}",
                    metadata.title.unwrap_or("Untitled".into())
                ),
                None,
                &["Delete", "Cancel"],
                cx,
            );

            cx.spawn_in(window, async move |this, cx| {
                if confirmation.await.ok() == Some(0) {
                    this.update_in(cx, |this, window, cx| {
                        if this.active_prompt_id == Some(prompt_id) {
                            this.set_active_prompt(None, window, cx);
                        }
                        this.prompt_editors.remove(&prompt_id);
                        this.store
                            .update(cx, |store, cx| store.delete(prompt_id, cx))
                            .detach_and_log_err(cx);
                        this.picker
                            .update(cx, |picker, cx| picker.refresh(window, cx));
                        cx.notify();
                    })?;
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }

    pub fn duplicate_prompt(
        &mut self,
        prompt_id: PromptId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(prompt) = self.prompt_editors.get(&prompt_id) {
            const DUPLICATE_SUFFIX: &str = " copy";
            let title_to_duplicate = prompt.title_editor.read(cx).text(cx);
            let existing_titles = self
                .prompt_editors
                .iter()
                .filter(|&(&id, _)| id != prompt_id)
                .map(|(_, prompt_editor)| prompt_editor.title_editor.read(cx).text(cx))
                .filter(|title| title.starts_with(&title_to_duplicate))
                .collect::<HashSet<_>>();

            let title = if existing_titles.is_empty() {
                title_to_duplicate + DUPLICATE_SUFFIX
            } else {
                let mut i = 1;
                loop {
                    let new_title = format!("{title_to_duplicate}{DUPLICATE_SUFFIX} {i}");
                    if !existing_titles.contains(&new_title) {
                        break new_title;
                    }
                    i += 1;
                }
            };

            let new_id = PromptId::new();
            let body = prompt.body_editor.read(cx).text(cx);
            let save = self.store.update(cx, |store, cx| {
                store.save(new_id, Some(title.into()), false, body.into(), cx)
            });
            self.picker
                .update(cx, |picker, cx| picker.refresh(window, cx));
            cx.spawn_in(window, async move |this, cx| {
                save.await?;
                this.update_in(cx, |prompt_library, window, cx| {
                    prompt_library.load_prompt(new_id, true, window, cx)
                })
            })
            .detach_and_log_err(cx);
        }
    }

    fn focus_active_prompt(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_prompt) = self.active_prompt_id {
            self.prompt_editors[&active_prompt]
                .body_editor
                .update(cx, |editor, cx| window.focus(&editor.focus_handle(cx)));
            cx.stop_propagation();
        }
    }

    fn focus_picker(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        self.picker
            .update(cx, |picker, cx| picker.focus(window, cx));
    }

    pub fn inline_assist(
        &mut self,
        action: &InlineAssist,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_prompt_id) = self.active_prompt_id else {
            cx.propagate();
            return;
        };

        let prompt_editor = &self.prompt_editors[&active_prompt_id].body_editor;
        let Some(ConfiguredModel { provider, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        else {
            return;
        };

        let initial_prompt = action.prompt.clone();
        if provider.is_authenticated(cx) {
            self.inline_assist_delegate
                .assist(prompt_editor, initial_prompt, window, cx);
        } else {
            for window in cx.windows() {
                if let Some(workspace) = window.downcast::<Workspace>() {
                    let panel = workspace
                        .update(cx, |workspace, window, cx| {
                            window.activate_window();
                            self.inline_assist_delegate
                                .focus_assistant_panel(workspace, window, cx)
                        })
                        .ok();
                    if panel == Some(true) {
                        return;
                    }
                }
            }
        }
    }

    fn move_down_from_title(
        &mut self,
        _: &editor::actions::MoveDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(prompt_id) = self.active_prompt_id {
            if let Some(prompt_editor) = self.prompt_editors.get(&prompt_id) {
                window.focus(&prompt_editor.body_editor.focus_handle(cx));
            }
        }
    }

    fn move_up_from_body(
        &mut self,
        _: &editor::actions::MoveUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(prompt_id) = self.active_prompt_id {
            if let Some(prompt_editor) = self.prompt_editors.get(&prompt_id) {
                window.focus(&prompt_editor.title_editor.focus_handle(cx));
            }
        }
    }

    fn handle_prompt_title_editor_event(
        &mut self,
        prompt_id: PromptId,
        title_editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::BufferEdited => {
                self.save_prompt(prompt_id, window, cx);
                self.count_tokens(prompt_id, window, cx);
            }
            EditorEvent::Blurred => {
                title_editor.update(cx, |title_editor, cx| {
                    title_editor.change_selections(None, window, cx, |selections| {
                        let cursor = selections.oldest_anchor().head();
                        selections.select_anchor_ranges([cursor..cursor]);
                    });
                });
            }
            _ => {}
        }
    }

    fn handle_prompt_body_editor_event(
        &mut self,
        prompt_id: PromptId,
        body_editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::BufferEdited => {
                self.save_prompt(prompt_id, window, cx);
                self.count_tokens(prompt_id, window, cx);
            }
            EditorEvent::Blurred => {
                body_editor.update(cx, |body_editor, cx| {
                    body_editor.change_selections(None, window, cx, |selections| {
                        let cursor = selections.oldest_anchor().head();
                        selections.select_anchor_ranges([cursor..cursor]);
                    });
                });
            }
            _ => {}
        }
    }

    fn count_tokens(&mut self, prompt_id: PromptId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ConfiguredModel { model, .. }) =
            LanguageModelRegistry::read_global(cx).default_model()
        else {
            return;
        };
        if let Some(prompt) = self.prompt_editors.get_mut(&prompt_id) {
            let editor = &prompt.body_editor.read(cx);
            let buffer = &editor.buffer().read(cx).as_singleton().unwrap().read(cx);
            let body = buffer.as_rope().clone();
            prompt.pending_token_count = cx.spawn_in(window, async move |this, cx| {
                async move {
                    const DEBOUNCE_TIMEOUT: Duration = Duration::from_secs(1);

                    cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
                    let token_count = cx
                        .update(|_, cx| {
                            model.count_tokens(
                                LanguageModelRequest {
                                    thread_id: None,
                                    prompt_id: None,
                                    messages: vec![LanguageModelRequestMessage {
                                        role: Role::System,
                                        content: vec![body.to_string().into()],
                                        cache: false,
                                    }],
                                    tools: Vec::new(),
                                    stop: Vec::new(),
                                    temperature: None,
                                },
                                cx,
                            )
                        })?
                        .await?;

                    this.update(cx, |this, cx| {
                        let prompt_editor = this.prompt_editors.get_mut(&prompt_id).unwrap();
                        prompt_editor.token_count = Some(token_count);
                        cx.notify();
                    })
                }
                .log_err()
                .await
            });
        }
    }

    fn render_prompt_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("prompt-list")
            .capture_action(cx.listener(Self::focus_active_prompt))
            .bg(cx.theme().colors().panel_background)
            .h_full()
            .px_1()
            .w_1_3()
            .overflow_x_hidden()
            .child(
                h_flex()
                    .p(DynamicSpacing::Base04.rems(cx))
                    .h_9()
                    .w_full()
                    .flex_none()
                    .justify_end()
                    .child(
                        IconButton::new("new-prompt", IconName::Plus)
                            .style(ButtonStyle::Transparent)
                            .shape(IconButtonShape::Square)
                            .tooltip(move |window, cx| {
                                Tooltip::for_action("New Prompt", &NewPrompt, window, cx)
                            })
                            .on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(NewPrompt), cx);
                            }),
                    ),
            )
            .child(div().flex_grow().child(self.picker.clone()))
    }

    fn render_active_prompt(&mut self, cx: &mut Context<PromptLibrary>) -> gpui::Stateful<Div> {
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
                let prompt_metadata = self.store.read(cx).metadata(prompt_id)?;
                let prompt_editor = &self.prompt_editors[&prompt_id];
                let focus_handle = prompt_editor.body_editor.focus_handle(cx);
                let model = LanguageModelRegistry::read_global(cx)
                    .default_model()
                    .map(|default| default.model);
                let settings = ThemeSettings::get_global(cx);

                Some(
                    v_flex()
                        .id("prompt-editor-inner")
                        .size_full()
                        .relative()
                        .overflow_hidden()
                        .pl(DynamicSpacing::Base16.rems(cx))
                        .pt(DynamicSpacing::Base08.rems(cx))
                        .on_click(cx.listener(move |_, _, window, _| {
                            window.focus(&focus_handle);
                        }))
                        .child(
                            h_flex()
                                .group("active-editor-header")
                                .pr(DynamicSpacing::Base16.rems(cx))
                                .pt(DynamicSpacing::Base02.rems(cx))
                                .pb(DynamicSpacing::Base08.rems(cx))
                                .justify_between()
                                .child(
                                    h_flex().gap_1().child(
                                        div()
                                            .max_w_80()
                                            .on_action(cx.listener(Self::move_down_from_title))
                                            .border_1()
                                            .border_color(transparent_black())
                                            .rounded_sm()
                                            .group_hover("active-editor-header", |this| {
                                                this.border_color(
                                                    cx.theme().colors().border_variant,
                                                )
                                            })
                                            .child(EditorElement::new(
                                                &prompt_editor.title_editor,
                                                EditorStyle {
                                                    background: cx.theme().system().transparent,
                                                    local_player: cx.theme().players().local(),
                                                    text: TextStyle {
                                                        color: cx
                                                            .theme()
                                                            .colors()
                                                            .editor_foreground,
                                                        font_family: settings
                                                            .ui_font
                                                            .family
                                                            .clone(),
                                                        font_features: settings
                                                            .ui_font
                                                            .features
                                                            .clone(),
                                                        font_size: HeadlineSize::Large
                                                            .rems()
                                                            .into(),
                                                        font_weight: settings.ui_font.weight,
                                                        line_height: relative(
                                                            settings.buffer_line_height.value(),
                                                        ),
                                                        ..Default::default()
                                                    },
                                                    scrollbar_width: Pixels::ZERO,
                                                    syntax: cx.theme().syntax().clone(),
                                                    status: cx.theme().status().clone(),
                                                    inlay_hints_style:
                                                        editor::make_inlay_hints_style(cx),
                                                    inline_completion_styles:
                                                        editor::make_suggestion_styles(cx),
                                                    ..EditorStyle::default()
                                                },
                                            )),
                                    ),
                                )
                                .child(
                                    h_flex()
                                        .h_full()
                                        .child(
                                            h_flex()
                                                .h_full()
                                                .gap(DynamicSpacing::Base16.rems(cx))
                                                .child(div()),
                                        )
                                        .child(
                                            h_flex()
                                                .h_full()
                                                .gap(DynamicSpacing::Base16.rems(cx))
                                                .children(prompt_editor.token_count.map(
                                                    |token_count| {
                                                        let token_count: SharedString =
                                                            token_count.to_string().into();
                                                        let label_token_count: SharedString =
                                                            token_count.to_string().into();

                                                        h_flex()
                                                            .id("token_count")
                                                            .tooltip(move |window, cx| {
                                                                let token_count =
                                                                    token_count.clone();

                                                                Tooltip::with_meta(
                                                                    format!(
                                                                        "{} tokens",
                                                                        token_count.clone()
                                                                    ),
                                                                    None,
                                                                    format!(
                                                                        "Model: {}",
                                                                        model
                                                                            .as_ref()
                                                                            .map(|model| model
                                                                                .name()
                                                                                .0)
                                                                            .unwrap_or_default()
                                                                    ),
                                                                    window,
                                                                    cx,
                                                                )
                                                            })
                                                            .child(
                                                                Label::new(format!(
                                                                    "{} tokens",
                                                                    label_token_count.clone()
                                                                ))
                                                                .color(Color::Muted),
                                                            )
                                                    },
                                                ))
                                                .child(if prompt_id.is_built_in() {
                                                    div()
                                                        .id("built-in-prompt")
                                                        .child(
                                                            Icon::new(IconName::FileLock)
                                                                .color(Color::Muted),
                                                        )
                                                        .tooltip(move |window, cx| {
                                                            Tooltip::with_meta(
                                                                "Built-in prompt",
                                                                None,
                                                                BUILT_IN_TOOLTIP_TEXT,
                                                                window,
                                                                cx,
                                                            )
                                                        })
                                                        .into_any()
                                                } else {
                                                    IconButton::new(
                                                        "delete-prompt",
                                                        IconName::Trash,
                                                    )
                                                    .size(ButtonSize::Large)
                                                    .style(ButtonStyle::Transparent)
                                                    .shape(IconButtonShape::Square)
                                                    .size(ButtonSize::Large)
                                                    .tooltip(move |window, cx| {
                                                        Tooltip::for_action(
                                                            "Delete Prompt",
                                                            &DeletePrompt,
                                                            window,
                                                            cx,
                                                        )
                                                    })
                                                    .on_click(|_, window, cx| {
                                                        window.dispatch_action(
                                                            Box::new(DeletePrompt),
                                                            cx,
                                                        );
                                                    })
                                                    .into_any_element()
                                                })
                                                .child(
                                                    IconButton::new(
                                                        "duplicate-prompt",
                                                        IconName::BookCopy,
                                                    )
                                                    .size(ButtonSize::Large)
                                                    .style(ButtonStyle::Transparent)
                                                    .shape(IconButtonShape::Square)
                                                    .size(ButtonSize::Large)
                                                    .tooltip(move |window, cx| {
                                                        Tooltip::for_action(
                                                            "Duplicate Prompt",
                                                            &DuplicatePrompt,
                                                            window,
                                                            cx,
                                                        )
                                                    })
                                                    .on_click(|_, window, cx| {
                                                        window.dispatch_action(
                                                            Box::new(DuplicatePrompt),
                                                            cx,
                                                        );
                                                    }),
                                                )
                                                .child(
                                                    IconButton::new(
                                                        "toggle-default-prompt",
                                                        IconName::Sparkle,
                                                    )
                                                    .style(ButtonStyle::Transparent)
                                                    .toggle_state(prompt_metadata.default)
                                                    .selected_icon(IconName::SparkleFilled)
                                                    .icon_color(if prompt_metadata.default {
                                                        Color::Accent
                                                    } else {
                                                        Color::Muted
                                                    })
                                                    .shape(IconButtonShape::Square)
                                                    .size(ButtonSize::Large)
                                                    .tooltip(Tooltip::text(
                                                        if prompt_metadata.default {
                                                            "Remove from Default Prompt"
                                                        } else {
                                                            "Add to Default Prompt"
                                                        },
                                                    ))
                                                    .on_click(|_, window, cx| {
                                                        window.dispatch_action(
                                                            Box::new(ToggleDefaultPrompt),
                                                            cx,
                                                        );
                                                    }),
                                                ),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .on_action(cx.listener(Self::focus_picker))
                                .on_action(cx.listener(Self::inline_assist))
                                .on_action(cx.listener(Self::move_up_from_body))
                                .flex_grow()
                                .h_full()
                                .child(prompt_editor.body_editor.clone()),
                        ),
                )
            }))
    }
}

impl Render for PromptLibrary {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);
        let theme = cx.theme().clone();

        h_flex()
            .id("prompt-manager")
            .key_context("PromptLibrary")
            .on_action(cx.listener(|this, &NewPrompt, window, cx| this.new_prompt(window, cx)))
            .on_action(
                cx.listener(|this, &DeletePrompt, window, cx| {
                    this.delete_active_prompt(window, cx)
                }),
            )
            .on_action(cx.listener(|this, &DuplicatePrompt, window, cx| {
                this.duplicate_active_prompt(window, cx)
            }))
            .on_action(cx.listener(|this, &ToggleDefaultPrompt, window, cx| {
                this.toggle_default_for_active_prompt(window, cx)
            }))
            .size_full()
            .overflow_hidden()
            .font(ui_font)
            .text_color(theme.colors().text)
            .child(self.render_prompt_list(cx))
            .map(|el| {
                if self.store.read(cx).prompt_count() == 0 {
                    el.child(
                        v_flex()
                            .w_2_3()
                            .h_full()
                            .items_center()
                            .justify_center()
                            .gap_4()
                            .bg(cx.theme().colors().editor_background)
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Icon::new(IconName::Book)
                                            .size(IconSize::Medium)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        Label::new("No prompts yet")
                                            .size(LabelSize::Large)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .child(h_flex())
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(Label::new("Create your first prompt:"))
                                            .child(
                                                Button::new("create-prompt", "New Prompt")
                                                    .full_width()
                                                    .key_binding(KeyBinding::for_action(
                                                        &NewPrompt, window, cx,
                                                    ))
                                                    .on_click(|_, window, cx| {
                                                        window.dispatch_action(
                                                            NewPrompt.boxed_clone(),
                                                            cx,
                                                        )
                                                    }),
                                            ),
                                    )
                                    .child(h_flex()),
                            ),
                    )
                } else {
                    el.child(self.render_active_prompt(cx))
                }
            })
    }
}
