use anyhow::Result;
use collections::{HashMap, HashSet};
use editor::{CompletionProvider, SelectionEffects};
use editor::{CurrentLineHighlight, Editor, EditorElement, EditorEvent, EditorStyle, actions::Tab};
use gpui::{
    App, Bounds, DEFAULT_ADDITIONAL_WINDOW_SIZE, Entity, EventEmitter, Focusable, PromptLevel,
    Subscription, Task, TextStyle, TitlebarOptions, WindowBounds, WindowHandle, WindowOptions,
    actions, point, size, transparent_black,
};
use language::{Buffer, LanguageRegistry, language_settings::SoftWrap};
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use picker::{Picker, PickerDelegate};
use release_channel::ReleaseChannel;
use rope::Rope;
use settings::Settings;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use theme::ThemeSettings;
use title_bar::platform_title_bar::PlatformTitleBar;
use ui::{Divider, ListItem, ListItemSpacing, ListSubHeader, Tooltip, prelude::*};
use util::{ResultExt, TryFutureExt};
use workspace::{Workspace, WorkspaceSettings, client_side_decorations};
use zed_actions::assistant::InlineAssist;

use prompt_store::*;

pub fn init(cx: &mut App) {
    prompt_store::init(cx);
}

actions!(
    rules_library,
    [
        /// Creates a new rule in the rules library.
        NewRule,
        /// Deletes the selected rule.
        DeleteRule,
        /// Duplicates the selected rule.
        DuplicateRule,
        /// Toggles whether the selected rule is a default rule.
        ToggleDefaultRule,
        /// Restores a built-in rule to its default content.
        RestoreDefaultContent
    ]
);

pub trait InlineAssistDelegate {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut Context<RulesLibrary>,
    );

    /// Returns whether the Agent panel was focused.
    fn focus_agent_panel(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool;
}

/// This function opens a new rules library window if one doesn't exist already.
/// If one exists, it brings it to the foreground.
///
/// Note that, when opening a new window, this waits for the PromptStore to be
/// initialized. If it was initialized successfully, it returns a window handle
/// to a rules library.
pub fn open_rules_library(
    language_registry: Arc<LanguageRegistry>,
    inline_assist_delegate: Box<dyn InlineAssistDelegate>,
    make_completion_provider: Rc<dyn Fn() -> Rc<dyn CompletionProvider>>,
    prompt_to_select: Option<PromptId>,
    cx: &mut App,
) -> Task<Result<WindowHandle<RulesLibrary>>> {
    let store = PromptStore::global(cx);
    cx.spawn(async move |cx| {
        // We query windows in spawn so that all windows have been returned to GPUI
        let existing_window = cx.update(|cx| {
            let existing_window = cx
                .windows()
                .into_iter()
                .find_map(|window| window.downcast::<RulesLibrary>());
            if let Some(existing_window) = existing_window {
                existing_window
                    .update(cx, |rules_library, window, cx| {
                        if let Some(prompt_to_select) = prompt_to_select {
                            rules_library.load_rule(prompt_to_select, true, window, cx);
                        }
                        window.activate_window()
                    })
                    .ok();

                Some(existing_window)
            } else {
                None
            }
        });

        if let Some(existing_window) = existing_window {
            return Ok(existing_window);
        }

        let store = store.await?;
        cx.update(|cx| {
            let app_id = ReleaseChannel::global(cx).app_id();
            let bounds = Bounds::centered(None, size(px(1024.0), px(768.0)), cx);
            let window_decorations = match std::env::var("ZED_WINDOW_DECORATIONS") {
                Ok(val) if val == "server" => gpui::WindowDecorations::Server,
                Ok(val) if val == "client" => gpui::WindowDecorations::Client,
                _ => match WorkspaceSettings::get_global(cx).window_decorations {
                    settings::WindowDecorations::Server => gpui::WindowDecorations::Server,
                    settings::WindowDecorations::Client => gpui::WindowDecorations::Client,
                },
            };
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Rules Library".into()),
                        appears_transparent: true,
                        traffic_light_position: Some(point(px(12.0), px(12.0))),
                    }),
                    app_id: Some(app_id.to_owned()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_background: cx.theme().window_background_appearance(),
                    window_decorations: Some(window_decorations),
                    window_min_size: Some(DEFAULT_ADDITIONAL_WINDOW_SIZE),
                    kind: gpui::WindowKind::Floating,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        RulesLibrary::new(
                            store,
                            language_registry,
                            inline_assist_delegate,
                            make_completion_provider,
                            prompt_to_select,
                            window,
                            cx,
                        )
                    })
                },
            )
        })
    })
}

pub struct RulesLibrary {
    title_bar: Option<Entity<PlatformTitleBar>>,
    store: Entity<PromptStore>,
    language_registry: Arc<LanguageRegistry>,
    rule_editors: HashMap<PromptId, RuleEditor>,
    active_rule_id: Option<PromptId>,
    picker: Entity<Picker<RulePickerDelegate>>,
    pending_load: Task<()>,
    inline_assist_delegate: Box<dyn InlineAssistDelegate>,
    make_completion_provider: Rc<dyn Fn() -> Rc<dyn CompletionProvider>>,
    _subscriptions: Vec<Subscription>,
}

struct RuleEditor {
    title_editor: Entity<Editor>,
    body_editor: Entity<Editor>,
    token_count: Option<u64>,
    pending_token_count: Task<Option<()>>,
    next_title_and_body_to_save: Option<(String, Rope)>,
    pending_save: Option<Task<Option<()>>>,
    _subscriptions: Vec<Subscription>,
}

enum RulePickerEntry {
    Header(SharedString),
    Rule(PromptMetadata),
    Separator,
}

struct RulePickerDelegate {
    store: Entity<PromptStore>,
    selected_index: usize,
    filtered_entries: Vec<RulePickerEntry>,
}

enum RulePickerEvent {
    Selected { prompt_id: PromptId },
    Confirmed { prompt_id: PromptId },
    Deleted { prompt_id: PromptId },
    ToggledDefault { prompt_id: PromptId },
}

impl EventEmitter<RulePickerEvent> for Picker<RulePickerDelegate> {}

impl PickerDelegate for RulePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No rules found matching your search.".into())
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_entries.len().saturating_sub(1));

        if let Some(RulePickerEntry::Rule(rule)) = self.filtered_entries.get(self.selected_index) {
            cx.emit(RulePickerEvent::Selected { prompt_id: rule.id });
        }

        cx.notify();
    }

    fn can_select(&mut self, ix: usize, _: &mut Window, _: &mut Context<Picker<Self>>) -> bool {
        match self.filtered_entries.get(ix) {
            Some(RulePickerEntry::Rule(_)) => true,
            Some(RulePickerEntry::Header(_)) | Some(RulePickerEntry::Separator) | None => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let cancellation_flag = Arc::new(AtomicBool::default());
        let search = self.store.read(cx).search(query, cancellation_flag, cx);

        let prev_prompt_id = self
            .filtered_entries
            .get(self.selected_index)
            .and_then(|entry| {
                if let RulePickerEntry::Rule(rule) = entry {
                    Some(rule.id)
                } else {
                    None
                }
            });

        cx.spawn_in(window, async move |this, cx| {
            let (filtered_entries, selected_index) = cx
                .background_spawn(async move {
                    let matches = search.await;

                    let (built_in_rules, user_rules): (Vec<_>, Vec<_>) =
                        matches.into_iter().partition(|rule| rule.id.is_built_in());
                    let (default_rules, other_rules): (Vec<_>, Vec<_>) =
                        user_rules.into_iter().partition(|rule| rule.default);

                    let mut filtered_entries = Vec::new();

                    if !built_in_rules.is_empty() {
                        filtered_entries.push(RulePickerEntry::Header("Built-in Rules".into()));

                        for rule in built_in_rules {
                            filtered_entries.push(RulePickerEntry::Rule(rule));
                        }

                        filtered_entries.push(RulePickerEntry::Separator);
                    }

                    if !default_rules.is_empty() {
                        filtered_entries.push(RulePickerEntry::Header("Default Rules".into()));

                        for rule in default_rules {
                            filtered_entries.push(RulePickerEntry::Rule(rule));
                        }

                        filtered_entries.push(RulePickerEntry::Separator);
                    }

                    for rule in other_rules {
                        filtered_entries.push(RulePickerEntry::Rule(rule));
                    }

                    let selected_index = prev_prompt_id
                        .and_then(|prev_prompt_id| {
                            filtered_entries.iter().position(|entry| {
                                if let RulePickerEntry::Rule(rule) = entry {
                                    rule.id == prev_prompt_id
                                } else {
                                    false
                                }
                            })
                        })
                        .unwrap_or_else(|| {
                            filtered_entries
                                .iter()
                                .position(|entry| matches!(entry, RulePickerEntry::Rule(_)))
                                .unwrap_or(0)
                        });

                    (filtered_entries, selected_index)
                })
                .await;

            this.update_in(cx, |this, window, cx| {
                this.delegate.filtered_entries = filtered_entries;
                this.set_selected_index(
                    selected_index,
                    Some(picker::Direction::Down),
                    true,
                    window,
                    cx,
                );
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(RulePickerEntry::Rule(rule)) = self.filtered_entries.get(self.selected_index) {
            cx.emit(RulePickerEvent::Confirmed { prompt_id: rule.id });
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
        match self.filtered_entries.get(ix)? {
            RulePickerEntry::Header(title) => {
                let tooltip_text = if title.as_ref() == "Built-in Rules" {
                    "Built-in rules are those included out of the box with Zed."
                } else {
                    "Default Rules are attached by default with every new thread."
                };

                Some(
                    ListSubHeader::new(title.clone())
                        .end_slot(
                            IconButton::new("info", IconName::Info)
                                .style(ButtonStyle::Transparent)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(Tooltip::text(tooltip_text))
                                .into_any_element(),
                        )
                        .inset(true)
                        .into_any_element(),
                )
            }
            RulePickerEntry::Separator => Some(
                h_flex()
                    .py_1()
                    .child(Divider::horizontal())
                    .into_any_element(),
            ),
            RulePickerEntry::Rule(rule) => {
                let default = rule.default;
                let prompt_id = rule.id;

                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(
                            Label::new(rule.title.clone().unwrap_or("Untitled".into()))
                                .truncate()
                                .mr_10(),
                        )
                        .end_slot::<IconButton>((default && !prompt_id.is_built_in()).then(|| {
                            IconButton::new("toggle-default-rule", IconName::Paperclip)
                                .toggle_state(true)
                                .icon_color(Color::Accent)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Remove from Default Rules"))
                                .on_click(cx.listener(move |_, _, _, cx| {
                                    cx.emit(RulePickerEvent::ToggledDefault { prompt_id })
                                }))
                        }))
                        .when(!prompt_id.is_built_in(), |this| {
                            this.end_hover_slot(
                                h_flex()
                                    .child(
                                        IconButton::new("delete-rule", IconName::Trash)
                                            .icon_color(Color::Muted)
                                            .icon_size(IconSize::Small)
                                            .tooltip(Tooltip::text("Delete Rule"))
                                            .on_click(cx.listener(move |_, _, _, cx| {
                                                cx.emit(RulePickerEvent::Deleted { prompt_id })
                                            })),
                                    )
                                    .child(
                                        IconButton::new("toggle-default-rule", IconName::Plus)
                                            .selected_icon(IconName::Dash)
                                            .toggle_state(default)
                                            .icon_size(IconSize::Small)
                                            .icon_color(if default {
                                                Color::Accent
                                            } else {
                                                Color::Muted
                                            })
                                            .map(|this| {
                                                if default {
                                                    this.tooltip(Tooltip::text(
                                                        "Remove from Default Rules",
                                                    ))
                                                } else {
                                                    this.tooltip(move |_window, cx| {
                                                        Tooltip::with_meta(
                                                            "Add to Default Rules",
                                                            None,
                                                            "Always included in every thread.",
                                                            cx,
                                                        )
                                                    })
                                                }
                                            })
                                            .on_click(cx.listener(move |_, _, _, cx| {
                                                cx.emit(RulePickerEvent::ToggledDefault {
                                                    prompt_id,
                                                })
                                            })),
                                    ),
                            )
                        })
                        .into_any_element(),
                )
            }
        }
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        h_flex()
            .py_1()
            .px_1p5()
            .mx_1()
            .gap_1p5()
            .rounded_sm()
            .bg(cx.theme().colors().editor_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
            .child(editor.clone())
    }
}

impl RulesLibrary {
    fn new(
        store: Entity<PromptStore>,
        language_registry: Arc<LanguageRegistry>,
        inline_assist_delegate: Box<dyn InlineAssistDelegate>,
        make_completion_provider: Rc<dyn Fn() -> Rc<dyn CompletionProvider>>,
        rule_to_select: Option<PromptId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (_selected_index, _matches) = if let Some(rule_to_select) = rule_to_select {
            let matches = store.read(cx).all_prompt_metadata();
            let selected_index = matches
                .iter()
                .enumerate()
                .find(|(_, metadata)| metadata.id == rule_to_select)
                .map_or(0, |(ix, _)| ix);
            (selected_index, matches)
        } else {
            (0, vec![])
        };

        let picker_delegate = RulePickerDelegate {
            store: store.clone(),
            selected_index: 0,
            filtered_entries: Vec::new(),
        };

        let picker = cx.new(|cx| {
            let picker = Picker::list(picker_delegate, window, cx)
                .modal(false)
                .max_height(None);
            picker.focus(window, cx);
            picker
        });

        Self {
            title_bar: if !cfg!(target_os = "macos") {
                Some(cx.new(|cx| PlatformTitleBar::new("rules-library-title-bar", cx)))
            } else {
                None
            },
            store,
            language_registry,
            rule_editors: HashMap::default(),
            active_rule_id: None,
            pending_load: Task::ready(()),
            inline_assist_delegate,
            make_completion_provider,
            _subscriptions: vec![cx.subscribe_in(&picker, window, Self::handle_picker_event)],
            picker,
        }
    }

    fn handle_picker_event(
        &mut self,
        _: &Entity<Picker<RulePickerDelegate>>,
        event: &RulePickerEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            RulePickerEvent::Selected { prompt_id } => {
                self.load_rule(*prompt_id, false, window, cx);
            }
            RulePickerEvent::Confirmed { prompt_id } => {
                self.load_rule(*prompt_id, true, window, cx);
            }
            RulePickerEvent::ToggledDefault { prompt_id } => {
                self.toggle_default_for_rule(*prompt_id, window, cx);
            }
            RulePickerEvent::Deleted { prompt_id } => {
                self.delete_rule(*prompt_id, window, cx);
            }
        }
    }

    pub fn new_rule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // If we already have an untitled rule, use that instead
        // of creating a new one.
        if let Some(metadata) = self.store.read(cx).first()
            && metadata.title.is_none()
        {
            self.load_rule(metadata.id, true, window, cx);
            return;
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
                this.load_rule(prompt_id, true, window, cx)
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn save_rule(&mut self, prompt_id: PromptId, window: &mut Window, cx: &mut Context<Self>) {
        const SAVE_THROTTLE: Duration = Duration::from_millis(500);

        if !prompt_id.can_edit() {
            return;
        }

        let rule_metadata = self.store.read(cx).metadata(prompt_id).unwrap();
        let rule_editor = self.rule_editors.get_mut(&prompt_id).unwrap();
        let title = rule_editor.title_editor.read(cx).text(cx);
        let body = rule_editor.body_editor.update(cx, |editor, cx| {
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

        rule_editor.next_title_and_body_to_save = Some((title, body));
        if rule_editor.pending_save.is_none() {
            rule_editor.pending_save = Some(cx.spawn_in(window, async move |this, cx| {
                async move {
                    loop {
                        let title_and_body = this.update(cx, |this, _| {
                            this.rule_editors
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
                                    store.save(prompt_id, title, rule_metadata.default, body, cx)
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
                        if let Some(rule_editor) = this.rule_editors.get_mut(&prompt_id) {
                            rule_editor.pending_save = None;
                        }
                    })
                }
                .log_err()
                .await
            }));
        }
    }

    pub fn delete_active_rule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_rule_id) = self.active_rule_id {
            self.delete_rule(active_rule_id, window, cx);
        }
    }

    pub fn duplicate_active_rule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_rule_id) = self.active_rule_id {
            self.duplicate_rule(active_rule_id, window, cx);
        }
    }

    pub fn toggle_default_for_active_rule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_rule_id) = self.active_rule_id {
            self.toggle_default_for_rule(active_rule_id, window, cx);
        }
    }

    pub fn restore_default_content_for_active_rule(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active_rule_id) = self.active_rule_id {
            self.restore_default_content(active_rule_id, window, cx);
        }
    }

    pub fn restore_default_content(
        &mut self,
        prompt_id: PromptId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(built_in) = prompt_id.as_built_in() else {
            return;
        };

        if let Some(rule_editor) = self.rule_editors.get(&prompt_id) {
            rule_editor.body_editor.update(cx, |editor, cx| {
                editor.set_text(built_in.default_content(), window, cx);
            });
        }
    }

    pub fn toggle_default_for_rule(
        &mut self,
        prompt_id: PromptId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.store.update(cx, move |store, cx| {
            if let Some(rule_metadata) = store.metadata(prompt_id) {
                store
                    .save_metadata(prompt_id, rule_metadata.title, !rule_metadata.default, cx)
                    .detach_and_log_err(cx);
            }
        });
        self.picker
            .update(cx, |picker, cx| picker.refresh(window, cx));
        cx.notify();
    }

    pub fn load_rule(
        &mut self,
        prompt_id: PromptId,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(rule_editor) = self.rule_editors.get(&prompt_id) {
            if focus {
                rule_editor
                    .body_editor
                    .update(cx, |editor, cx| window.focus(&editor.focus_handle(cx), cx));
            }
            self.set_active_rule(Some(prompt_id), window, cx);
        } else if let Some(rule_metadata) = self.store.read(cx).metadata(prompt_id) {
            let language_registry = self.language_registry.clone();
            let rule = self.store.read(cx).load(prompt_id, cx);
            let make_completion_provider = self.make_completion_provider.clone();
            self.pending_load = cx.spawn_in(window, async move |this, cx| {
                let rule = rule.await;
                let markdown = language_registry.language_for_name("Markdown").await;
                this.update_in(cx, |this, window, cx| match rule {
                    Ok(rule) => {
                        let title_editor = cx.new(|cx| {
                            let mut editor = Editor::single_line(window, cx);
                            editor.set_placeholder_text("Untitled", window, cx);
                            editor.set_text(rule_metadata.title.unwrap_or_default(), window, cx);
                            if prompt_id.is_built_in() {
                                editor.set_read_only(true);
                                editor.set_show_edit_predictions(Some(false), window, cx);
                            }
                            editor
                        });
                        let body_editor = cx.new(|cx| {
                            let buffer = cx.new(|cx| {
                                let mut buffer = Buffer::local(rule, cx);
                                buffer.set_language(markdown.log_err(), cx);
                                buffer.set_language_registry(language_registry);
                                buffer
                            });

                            let mut editor = Editor::for_buffer(buffer, None, window, cx);
                            if !prompt_id.can_edit() {
                                editor.set_read_only(true);
                                editor.set_show_edit_predictions(Some(false), window, cx);
                            }
                            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                            editor.set_show_gutter(false, cx);
                            editor.set_show_wrap_guides(false, cx);
                            editor.set_show_indent_guides(false, cx);
                            editor.set_use_modal_editing(true);
                            editor.set_current_line_highlight(Some(CurrentLineHighlight::None));
                            editor.set_completion_provider(Some(make_completion_provider()));
                            if focus {
                                window.focus(&editor.focus_handle(cx), cx);
                            }
                            editor
                        });
                        let _subscriptions = vec![
                            cx.subscribe_in(
                                &title_editor,
                                window,
                                move |this, editor, event, window, cx| {
                                    this.handle_rule_title_editor_event(
                                        prompt_id, editor, event, window, cx,
                                    )
                                },
                            ),
                            cx.subscribe_in(
                                &body_editor,
                                window,
                                move |this, editor, event, window, cx| {
                                    this.handle_rule_body_editor_event(
                                        prompt_id, editor, event, window, cx,
                                    )
                                },
                            ),
                        ];
                        this.rule_editors.insert(
                            prompt_id,
                            RuleEditor {
                                title_editor,
                                body_editor,
                                next_title_and_body_to_save: None,
                                pending_save: None,
                                token_count: None,
                                pending_token_count: Task::ready(None),
                                _subscriptions,
                            },
                        );
                        this.set_active_rule(Some(prompt_id), window, cx);
                        this.count_tokens(prompt_id, window, cx);
                    }
                    Err(error) => {
                        // TODO: we should show the error in the UI.
                        log::error!("error while loading rule: {:?}", error);
                    }
                })
                .ok();
            });
        }
    }

    fn set_active_rule(
        &mut self,
        prompt_id: Option<PromptId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_rule_id = prompt_id;
        self.picker.update(cx, |picker, cx| {
            if let Some(prompt_id) = prompt_id {
                if picker
                    .delegate
                    .filtered_entries
                    .get(picker.delegate.selected_index())
                    .is_none_or(|old_selected_prompt| {
                        if let RulePickerEntry::Rule(rule) = old_selected_prompt {
                            rule.id != prompt_id
                        } else {
                            true
                        }
                    })
                    && let Some(ix) = picker.delegate.filtered_entries.iter().position(|mat| {
                        if let RulePickerEntry::Rule(rule) = mat {
                            rule.id == prompt_id
                        } else {
                            false
                        }
                    })
                {
                    picker.set_selected_index(ix, None, true, window, cx);
                }
            } else {
                picker.focus(window, cx);
            }
        });
        cx.notify();
    }

    pub fn delete_rule(
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
                        if this.active_rule_id == Some(prompt_id) {
                            this.set_active_rule(None, window, cx);
                        }
                        this.rule_editors.remove(&prompt_id);
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

    pub fn duplicate_rule(
        &mut self,
        prompt_id: PromptId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(rule) = self.rule_editors.get(&prompt_id) {
            const DUPLICATE_SUFFIX: &str = " copy";
            let title_to_duplicate = rule.title_editor.read(cx).text(cx);
            let existing_titles = self
                .rule_editors
                .iter()
                .filter(|&(&id, _)| id != prompt_id)
                .map(|(_, rule_editor)| rule_editor.title_editor.read(cx).text(cx))
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
            let body = rule.body_editor.read(cx).text(cx);
            let save = self.store.update(cx, |store, cx| {
                store.save(new_id, Some(title.into()), false, body.into(), cx)
            });
            self.picker
                .update(cx, |picker, cx| picker.refresh(window, cx));
            cx.spawn_in(window, async move |this, cx| {
                save.await?;
                this.update_in(cx, |rules_library, window, cx| {
                    rules_library.load_rule(new_id, true, window, cx)
                })
            })
            .detach_and_log_err(cx);
        }
    }

    fn focus_active_rule(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_rule) = self.active_rule_id {
            self.rule_editors[&active_rule]
                .body_editor
                .update(cx, |editor, cx| window.focus(&editor.focus_handle(cx), cx));
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
        let Some(active_rule_id) = self.active_rule_id else {
            cx.propagate();
            return;
        };

        let rule_editor = &self.rule_editors[&active_rule_id].body_editor;
        let Some(ConfiguredModel { provider, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        else {
            return;
        };

        let initial_prompt = action.prompt.clone();
        if provider.is_authenticated(cx) {
            self.inline_assist_delegate
                .assist(rule_editor, initial_prompt, window, cx);
        } else {
            for window in cx.windows() {
                if let Some(workspace) = window.downcast::<Workspace>() {
                    let panel = workspace
                        .update(cx, |workspace, window, cx| {
                            window.activate_window();
                            self.inline_assist_delegate
                                .focus_agent_panel(workspace, window, cx)
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
        if let Some(rule_id) = self.active_rule_id
            && let Some(rule_editor) = self.rule_editors.get(&rule_id)
        {
            window.focus(&rule_editor.body_editor.focus_handle(cx), cx);
        }
    }

    fn move_up_from_body(
        &mut self,
        _: &editor::actions::MoveUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(rule_id) = self.active_rule_id
            && let Some(rule_editor) = self.rule_editors.get(&rule_id)
        {
            window.focus(&rule_editor.title_editor.focus_handle(cx), cx);
        }
    }

    fn handle_rule_title_editor_event(
        &mut self,
        prompt_id: PromptId,
        title_editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::BufferEdited => {
                self.save_rule(prompt_id, window, cx);
                self.count_tokens(prompt_id, window, cx);
            }
            EditorEvent::Blurred => {
                title_editor.update(cx, |title_editor, cx| {
                    title_editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            let cursor = selections.oldest_anchor().head();
                            selections.select_anchor_ranges([cursor..cursor]);
                        },
                    );
                });
            }
            _ => {}
        }
    }

    fn handle_rule_body_editor_event(
        &mut self,
        prompt_id: PromptId,
        body_editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::BufferEdited => {
                self.save_rule(prompt_id, window, cx);
                self.count_tokens(prompt_id, window, cx);
            }
            EditorEvent::Blurred => {
                body_editor.update(cx, |body_editor, cx| {
                    body_editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            let cursor = selections.oldest_anchor().head();
                            selections.select_anchor_ranges([cursor..cursor]);
                        },
                    );
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
        if let Some(rule) = self.rule_editors.get_mut(&prompt_id) {
            let editor = &rule.body_editor.read(cx);
            let buffer = &editor.buffer().read(cx).as_singleton().unwrap().read(cx);
            let body = buffer.as_rope().clone();
            rule.pending_token_count = cx.spawn_in(window, async move |this, cx| {
                async move {
                    const DEBOUNCE_TIMEOUT: Duration = Duration::from_secs(1);

                    cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
                    let token_count = cx
                        .update(|_, cx| {
                            model.count_tokens(
                                LanguageModelRequest {
                                    thread_id: None,
                                    prompt_id: None,
                                    intent: None,
                                    messages: vec![LanguageModelRequestMessage {
                                        role: Role::System,
                                        content: vec![body.to_string().into()],
                                        cache: false,
                                        reasoning_details: None,
                                    }],
                                    tools: Vec::new(),
                                    tool_choice: None,
                                    stop: Vec::new(),
                                    temperature: None,
                                    thinking_allowed: true,
                                },
                                cx,
                            )
                        })?
                        .await?;

                    this.update(cx, |this, cx| {
                        let rule_editor = this.rule_editors.get_mut(&prompt_id).unwrap();
                        rule_editor.token_count = Some(token_count);
                        cx.notify();
                    })
                }
                .log_err()
                .await
            });
        }
    }

    fn render_rule_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("rule-list")
            .capture_action(cx.listener(Self::focus_active_rule))
            .px_1p5()
            .h_full()
            .w_64()
            .overflow_x_hidden()
            .bg(cx.theme().colors().panel_background)
            .map(|this| {
                if cfg!(target_os = "macos") {
                    this.child(
                        h_flex()
                            .p(DynamicSpacing::Base04.rems(cx))
                            .h_9()
                            .w_full()
                            .flex_none()
                            .justify_end()
                            .child(
                                IconButton::new("new-rule", IconName::Plus)
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action("New Rule", &NewRule, cx)
                                    })
                                    .on_click(|_, window, cx| {
                                        window.dispatch_action(Box::new(NewRule), cx);
                                    }),
                            ),
                    )
                } else {
                    this.child(
                        h_flex().p_1().w_full().child(
                            Button::new("new-rule", "New Rule")
                                .full_width()
                                .style(ButtonStyle::Outlined)
                                .icon(IconName::Plus)
                                .icon_size(IconSize::Small)
                                .icon_position(IconPosition::Start)
                                .icon_color(Color::Muted)
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(NewRule), cx);
                                }),
                        ),
                    )
                }
            })
            .child(div().flex_grow().child(self.picker.clone()))
    }

    fn render_active_rule_editor(
        &self,
        editor: &Entity<Editor>,
        read_only: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_color = if read_only {
            cx.theme().colors().text_muted
        } else {
            cx.theme().colors().text
        };

        div()
            .w_full()
            .pl_1()
            .border_1()
            .border_color(transparent_black())
            .rounded_sm()
            .when(!read_only, |this| {
                this.group_hover("active-editor-header", |this| {
                    this.border_color(cx.theme().colors().border_variant)
                })
            })
            .on_action(cx.listener(Self::move_down_from_title))
            .child(EditorElement::new(
                &editor,
                EditorStyle {
                    background: cx.theme().system().transparent,
                    local_player: cx.theme().players().local(),
                    text: TextStyle {
                        color: text_color,
                        font_family: settings.ui_font.family.clone(),
                        font_features: settings.ui_font.features.clone(),
                        font_size: HeadlineSize::Medium.rems().into(),
                        font_weight: settings.ui_font.weight,
                        line_height: relative(settings.buffer_line_height.value()),
                        ..Default::default()
                    },
                    scrollbar_width: Pixels::ZERO,
                    syntax: cx.theme().syntax().clone(),
                    status: cx.theme().status().clone(),
                    inlay_hints_style: editor::make_inlay_hints_style(cx),
                    edit_prediction_styles: editor::make_suggestion_styles(cx),
                    ..EditorStyle::default()
                },
            ))
    }

    fn render_duplicate_rule_button(&self) -> impl IntoElement {
        IconButton::new("duplicate-rule", IconName::BookCopy)
            .tooltip(move |_window, cx| Tooltip::for_action("Duplicate Rule", &DuplicateRule, cx))
            .on_click(|_, window, cx| {
                window.dispatch_action(Box::new(DuplicateRule), cx);
            })
    }

    fn render_built_in_rule_controls(&self) -> impl IntoElement {
        h_flex()
            .gap_1()
            .child(self.render_duplicate_rule_button())
            .child(
                IconButton::new("restore-default", IconName::RotateCcw)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action(
                            "Restore to Default Content",
                            &RestoreDefaultContent,
                            cx,
                        )
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(RestoreDefaultContent), cx);
                    }),
            )
    }

    fn render_regular_rule_controls(&self, default: bool) -> impl IntoElement {
        h_flex()
            .gap_1()
            .child(
                IconButton::new("toggle-default-rule", IconName::Paperclip)
                    .toggle_state(default)
                    .when(default, |this| this.icon_color(Color::Accent))
                    .map(|this| {
                        if default {
                            this.tooltip(Tooltip::text("Remove from Default Rules"))
                        } else {
                            this.tooltip(move |_window, cx| {
                                Tooltip::with_meta(
                                    "Add to Default Rules",
                                    None,
                                    "Always included in every thread.",
                                    cx,
                                )
                            })
                        }
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(ToggleDefaultRule), cx);
                    }),
            )
            .child(self.render_duplicate_rule_button())
            .child(
                IconButton::new("delete-rule", IconName::Trash)
                    .tooltip(move |_window, cx| Tooltip::for_action("Delete Rule", &DeleteRule, cx))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(DeleteRule), cx);
                    }),
            )
    }

    fn render_active_rule(&mut self, cx: &mut Context<RulesLibrary>) -> gpui::Stateful<Div> {
        div()
            .id("rule-editor")
            .h_full()
            .flex_grow()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .children(self.active_rule_id.and_then(|prompt_id| {
                let rule_metadata = self.store.read(cx).metadata(prompt_id)?;
                let rule_editor = &self.rule_editors[&prompt_id];
                let focus_handle = rule_editor.body_editor.focus_handle(cx);
                let registry = LanguageModelRegistry::read_global(cx);
                let model = registry.default_model().map(|default| default.model);
                let built_in = prompt_id.is_built_in();

                Some(
                    v_flex()
                        .id("rule-editor-inner")
                        .size_full()
                        .relative()
                        .overflow_hidden()
                        .on_click(cx.listener(move |_, _, window, cx| {
                            window.focus(&focus_handle, cx);
                        }))
                        .child(
                            h_flex()
                                .group("active-editor-header")
                                .h_12()
                                .px_2()
                                .gap_2()
                                .justify_between()
                                .child(self.render_active_rule_editor(
                                    &rule_editor.title_editor,
                                    built_in,
                                    cx,
                                ))
                                .child(
                                    h_flex()
                                        .h_full()
                                        .flex_shrink_0()
                                        .children(rule_editor.token_count.map(|token_count| {
                                            let token_count: SharedString =
                                                token_count.to_string().into();
                                            let label_token_count: SharedString =
                                                token_count.to_string().into();

                                            div()
                                                .id("token_count")
                                                .mr_1()
                                                .flex_shrink_0()
                                                .tooltip(move |_window, cx| {
                                                    Tooltip::with_meta(
                                                        "Token Estimation",
                                                        None,
                                                        format!(
                                                            "Model: {}",
                                                            model
                                                                .as_ref()
                                                                .map(|model| model.name().0)
                                                                .unwrap_or_default()
                                                        ),
                                                        cx,
                                                    )
                                                })
                                                .child(
                                                    Label::new(format!(
                                                        "{} tokens",
                                                        label_token_count
                                                    ))
                                                    .color(Color::Muted),
                                                )
                                        }))
                                        .map(|this| {
                                            if built_in {
                                                this.child(self.render_built_in_rule_controls())
                                            } else {
                                                this.child(self.render_regular_rule_controls(
                                                    rule_metadata.default,
                                                ))
                                            }
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .on_action(cx.listener(Self::focus_picker))
                                .on_action(cx.listener(Self::inline_assist))
                                .on_action(cx.listener(Self::move_up_from_body))
                                .h_full()
                                .flex_grow()
                                .child(
                                    h_flex()
                                        .py_2()
                                        .pl_2p5()
                                        .h_full()
                                        .flex_1()
                                        .child(rule_editor.body_editor.clone()),
                                ),
                        ),
                )
            }))
    }
}

impl Render for RulesLibrary {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);
        let theme = cx.theme().clone();

        client_side_decorations(
            v_flex()
                .id("rules-library")
                .key_context("RulesLibrary")
                .on_action(cx.listener(|this, &NewRule, window, cx| this.new_rule(window, cx)))
                .on_action(
                    cx.listener(|this, &DeleteRule, window, cx| {
                        this.delete_active_rule(window, cx)
                    }),
                )
                .on_action(cx.listener(|this, &DuplicateRule, window, cx| {
                    this.duplicate_active_rule(window, cx)
                }))
                .on_action(cx.listener(|this, &ToggleDefaultRule, window, cx| {
                    this.toggle_default_for_active_rule(window, cx)
                }))
                .on_action(cx.listener(|this, &RestoreDefaultContent, window, cx| {
                    this.restore_default_content_for_active_rule(window, cx)
                }))
                .size_full()
                .overflow_hidden()
                .font(ui_font)
                .text_color(theme.colors().text)
                .children(self.title_bar.clone())
                .bg(theme.colors().background)
                .child(
                    h_flex()
                        .flex_1()
                        .when(!cfg!(target_os = "macos"), |this| {
                            this.border_t_1().border_color(cx.theme().colors().border)
                        })
                        .child(self.render_rule_list(cx))
                        .child(self.render_active_rule(cx)),
                ),
            window,
            cx,
        )
    }
}
