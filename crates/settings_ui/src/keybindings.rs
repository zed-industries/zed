use std::{
    ops::{Not, Range},
    sync::Arc,
};

use anyhow::{Context as _, anyhow};
use collections::HashSet;
use editor::{CompletionProvider, Editor, EditorEvent};
use feature_flags::FeatureFlagViewExt;
use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AppContext as _, AsyncApp, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Global, KeyContext, Keystroke, ModifiersChangedEvent, ScrollStrategy, StyledText, Subscription,
    WeakEntity, actions, div, transparent_black,
};
use language::{Language, LanguageConfig, ToOffset as _};
use settings::{BaseKeymap, KeybindSource, KeymapFile, SettingsAssets};

use util::ResultExt;

use ui::{
    ActiveTheme as _, App, BorrowAppContext, ContextMenu, ParentElement as _, Render, SharedString,
    Styled as _, Tooltip, Window, prelude::*, right_click_menu,
};
use workspace::{Item, ModalView, SerializableItem, Workspace, register_serializable_item};

use crate::{
    SettingsUiFeatureFlag,
    keybindings::persistence::KEYBINDING_EDITORS,
    ui_components::table::{Table, TableInteractionState},
};

actions!(
    zed,
    [
        /// Opens the keymap editor.
        OpenKeymapEditor
    ]
);

const KEYMAP_EDITOR_NAMESPACE: &'static str = "keymap_editor";
actions!(
    keymap_editor,
    [
        /// Edits the selected key binding.
        EditBinding,
        /// Copies the action name to clipboard.
        CopyAction,
        /// Copies the context predicate to clipboard.
        CopyContext
    ]
);

pub fn init(cx: &mut App) {
    let keymap_event_channel = KeymapEventChannel::new();
    cx.set_global(keymap_event_channel);

    cx.on_action(|_: &OpenKeymapEditor, cx| {
        workspace::with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let existing = workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<KeymapEditor>());

            if let Some(existing) = existing {
                workspace.activate_item(&existing, true, true, window, cx);
            } else {
                let keymap_editor =
                    cx.new(|cx| KeymapEditor::new(workspace.weak_handle(), window, cx));
                workspace.add_item_to_active_pane(Box::new(keymap_editor), None, true, window, cx);
            }
        });
    });

    cx.observe_new(|_workspace: &mut Workspace, window, cx| {
        let Some(window) = window else { return };

        let keymap_ui_actions = [std::any::TypeId::of::<OpenKeymapEditor>()];

        command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.hide_action_types(&keymap_ui_actions);
            filter.hide_namespace(KEYMAP_EDITOR_NAMESPACE);
        });

        cx.observe_flag::<SettingsUiFeatureFlag, _>(
            window,
            move |is_enabled, _workspace, _, cx| {
                if is_enabled {
                    command_palette_hooks::CommandPaletteFilter::update_global(
                        cx,
                        |filter, _cx| {
                            filter.show_action_types(keymap_ui_actions.iter());
                            filter.show_namespace(KEYMAP_EDITOR_NAMESPACE);
                        },
                    );
                } else {
                    command_palette_hooks::CommandPaletteFilter::update_global(
                        cx,
                        |filter, _cx| {
                            filter.hide_action_types(&keymap_ui_actions);
                            filter.hide_namespace(KEYMAP_EDITOR_NAMESPACE);
                        },
                    );
                }
            },
        )
        .detach();
    })
    .detach();

    register_serializable_item::<KeymapEditor>(cx);
}

pub struct KeymapEventChannel {}

impl Global for KeymapEventChannel {}

impl KeymapEventChannel {
    fn new() -> Self {
        Self {}
    }

    pub fn trigger_keymap_changed(cx: &mut App) {
        let Some(_event_channel) = cx.try_global::<Self>() else {
            // don't panic if no global defined. This usually happens in tests
            return;
        };
        cx.update_global(|_event_channel: &mut Self, _| {
            /* triggers observers in KeymapEditors */
        });
    }
}

struct KeymapEditor {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    _keymap_subscription: Subscription,
    keybindings: Vec<ProcessedKeybinding>,
    // corresponds 1 to 1 with keybindings
    string_match_candidates: Arc<Vec<StringMatchCandidate>>,
    matches: Vec<StringMatch>,
    table_interaction_state: Entity<TableInteractionState>,
    filter_editor: Entity<Editor>,
    selected_index: Option<usize>,
}

impl EventEmitter<()> for KeymapEditor {}

impl Focusable for KeymapEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        return self.filter_editor.focus_handle(cx);
    }
}

impl KeymapEditor {
    fn new(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let _keymap_subscription =
            cx.observe_global::<KeymapEventChannel>(Self::update_keybindings);
        let table_interaction_state = TableInteractionState::new(window, cx);

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Filter action names…", cx);
            editor
        });

        cx.subscribe(&filter_editor, |this, _, e: &EditorEvent, cx| {
            if !matches!(e, EditorEvent::BufferEdited) {
                return;
            }

            this.update_matches(cx);
        })
        .detach();

        let mut this = Self {
            workspace,
            keybindings: vec![],
            string_match_candidates: Arc::new(vec![]),
            matches: vec![],
            focus_handle: focus_handle.clone(),
            _keymap_subscription,
            table_interaction_state,
            filter_editor,
            selected_index: None,
        };

        this.update_keybindings(cx);

        this
    }

    fn current_query(&self, cx: &mut Context<Self>) -> String {
        self.filter_editor.read(cx).text(cx)
    }

    fn update_matches(&self, cx: &mut Context<Self>) {
        let query = self.current_query(cx);

        cx.spawn(async move |this, cx| Self::process_query(this, query, cx).await)
            .detach();
    }

    async fn process_query(
        this: WeakEntity<Self>,
        query: String,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        let query = command_palette::normalize_action_query(&query);
        let (string_match_candidates, keybind_count) = this.read_with(cx, |this, _| {
            (this.string_match_candidates.clone(), this.keybindings.len())
        })?;
        let executor = cx.background_executor().clone();
        let mut matches = fuzzy::match_strings(
            &string_match_candidates,
            &query,
            true,
            true,
            keybind_count,
            &Default::default(),
            executor,
        )
        .await;
        this.update(cx, |this, cx| {
            if query.is_empty() {
                // apply default sort
                // sorts by source precedence, and alphabetically by action name within each source
                matches.sort_by_key(|match_item| {
                    let keybind = &this.keybindings[match_item.candidate_id];
                    let source = keybind.source.as_ref().map(|s| s.0);
                    use KeybindSource::*;
                    let source_precedence = match source {
                        Some(User) => 0,
                        Some(Vim) => 1,
                        Some(Base) => 2,
                        Some(Default) => 3,
                        None => 4,
                    };
                    return (source_precedence, keybind.action.as_ref());
                });
            }
            this.selected_index.take();
            this.scroll_to_item(0, ScrollStrategy::Top, cx);
            this.matches = matches;
            cx.notify();
        })
    }

    fn process_bindings(
        json_language: Arc<Language>,
        rust_language: Arc<Language>,
        cx: &mut App,
    ) -> (Vec<ProcessedKeybinding>, Vec<StringMatchCandidate>) {
        let key_bindings_ptr = cx.key_bindings();
        let lock = key_bindings_ptr.borrow();
        let key_bindings = lock.bindings();
        let mut unmapped_action_names =
            HashSet::from_iter(cx.all_action_names().into_iter().copied());
        let action_documentation = cx.action_documentation();

        let mut processed_bindings = Vec::new();
        let mut string_match_candidates = Vec::new();

        for key_binding in key_bindings {
            let source = key_binding.meta().map(settings::KeybindSource::from_meta);

            let keystroke_text = ui::text_for_keystrokes(key_binding.keystrokes(), cx);
            let ui_key_binding = Some(
                ui::KeyBinding::new_from_gpui(key_binding.clone(), cx)
                    .vim_mode(source == Some(settings::KeybindSource::Vim)),
            );

            let context = key_binding
                .predicate()
                .map(|predicate| {
                    KeybindContextString::Local(predicate.to_string().into(), rust_language.clone())
                })
                .unwrap_or(KeybindContextString::Global);

            let source = source.map(|source| (source, source.name().into()));

            let action_name = key_binding.action().name();
            unmapped_action_names.remove(&action_name);
            let action_input = key_binding
                .action_input()
                .map(|input| SyntaxHighlightedText::new(input, json_language.clone()));
            let action_docs = action_documentation.get(action_name).copied();

            let index = processed_bindings.len();
            let string_match_candidate = StringMatchCandidate::new(index, &action_name);
            processed_bindings.push(ProcessedKeybinding {
                keystroke_text: keystroke_text.into(),
                ui_key_binding,
                action: action_name.into(),
                action_input,
                action_docs,
                context: Some(context),
                source,
            });
            string_match_candidates.push(string_match_candidate);
        }

        let empty = SharedString::new_static("");
        for action_name in unmapped_action_names.into_iter() {
            let index = processed_bindings.len();
            let string_match_candidate = StringMatchCandidate::new(index, &action_name);
            processed_bindings.push(ProcessedKeybinding {
                keystroke_text: empty.clone(),
                ui_key_binding: None,
                action: action_name.into(),
                action_input: None,
                action_docs: action_documentation.get(action_name).copied(),
                context: None,
                source: None,
            });
            string_match_candidates.push(string_match_candidate);
        }

        (processed_bindings, string_match_candidates)
    }

    fn update_keybindings(&mut self, cx: &mut Context<KeymapEditor>) {
        let workspace = self.workspace.clone();
        cx.spawn(async move |this, cx| {
            let json_language = Self::load_json_language(workspace.clone(), cx).await;
            let rust_language = Self::load_rust_language(workspace.clone(), cx).await;

            let query = this.update(cx, |this, cx| {
                let (key_bindings, string_match_candidates) =
                    Self::process_bindings(json_language, rust_language, cx);
                this.keybindings = key_bindings;
                this.string_match_candidates = Arc::new(string_match_candidates);
                this.matches = this
                    .string_match_candidates
                    .iter()
                    .enumerate()
                    .map(|(ix, candidate)| StringMatch {
                        candidate_id: ix,
                        score: 0.0,
                        positions: vec![],
                        string: candidate.string.clone(),
                    })
                    .collect();
                this.current_query(cx)
            })?;
            // calls cx.notify
            Self::process_query(this, query, cx).await
        })
        .detach_and_log_err(cx);
    }

    async fn load_json_language(
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncApp,
    ) -> Arc<Language> {
        let json_language_task = workspace
            .read_with(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .languages()
                    .language_for_name("JSON")
            })
            .context("Failed to load JSON language")
            .log_err();
        let json_language = match json_language_task {
            Some(task) => task.await.context("Failed to load JSON language").log_err(),
            None => None,
        };
        return json_language.unwrap_or_else(|| {
            Arc::new(Language::new(
                LanguageConfig {
                    name: "JSON".into(),
                    ..Default::default()
                },
                Some(tree_sitter_json::LANGUAGE.into()),
            ))
        });
    }

    async fn load_rust_language(
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncApp,
    ) -> Arc<Language> {
        let rust_language_task = workspace
            .read_with(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .languages()
                    .language_for_name("Rust")
            })
            .context("Failed to load Rust language")
            .log_err();
        let rust_language = match rust_language_task {
            Some(task) => task.await.context("Failed to load Rust language").log_err(),
            None => None,
        };
        return rust_language.unwrap_or_else(|| {
            Arc::new(Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            ))
        });
    }

    fn dispatch_context(&self, _window: &Window, _cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("KeymapEditor");
        dispatch_context.add("menu");

        dispatch_context
    }

    fn scroll_to_item(&self, index: usize, strategy: ScrollStrategy, cx: &mut App) {
        let index = usize::min(index, self.matches.len().saturating_sub(1));
        self.table_interaction_state.update(cx, |this, _cx| {
            this.scroll_handle.scroll_to_item(index, strategy);
        });
    }

    fn focus_search(
        &mut self,
        _: &search::FocusSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self
            .filter_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            window.focus(&self.filter_editor.focus_handle(cx));
        } else {
            self.filter_editor.update(cx, |editor, cx| {
                editor.select_all(&Default::default(), window, cx);
            });
        }
        self.selected_index.take();
    }

    fn selected_binding(&self) -> Option<&ProcessedKeybinding> {
        self.selected_index
            .and_then(|match_index| self.matches.get(match_index))
            .map(|r#match| r#match.candidate_id)
            .and_then(|keybind_index| self.keybindings.get(keybind_index))
    }

    fn select_next(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected) = self.selected_index {
            let selected = selected + 1;
            if selected >= self.matches.len() {
                self.select_last(&Default::default(), window, cx);
            } else {
                self.selected_index = Some(selected);
                self.scroll_to_item(selected, ScrollStrategy::Center, cx);
                cx.notify();
            }
        } else {
            self.select_first(&Default::default(), window, cx);
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected) = self.selected_index {
            if selected == 0 {
                return;
            }

            let selected = selected - 1;

            if selected >= self.matches.len() {
                self.select_last(&Default::default(), window, cx);
            } else {
                self.selected_index = Some(selected);
                self.scroll_to_item(selected, ScrollStrategy::Center, cx);
                cx.notify();
            }
        } else {
            self.select_last(&Default::default(), window, cx);
        }
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.matches.get(0).is_some() {
            self.selected_index = Some(0);
            self.scroll_to_item(0, ScrollStrategy::Center, cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        if self.matches.last().is_some() {
            let index = self.matches.len() - 1;
            self.selected_index = Some(index);
            self.scroll_to_item(index, ScrollStrategy::Center, cx);
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.edit_selected_keybinding(window, cx);
    }

    fn edit_selected_keybinding(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(keybind) = self.selected_binding() else {
            return;
        };
        self.workspace
            .update(cx, |workspace, cx| {
                let fs = workspace.app_state().fs.clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let modal = KeybindingEditorModal::new(keybind.clone(), fs, window, cx);
                    window.focus(&modal.focus_handle(cx));
                    modal
                });
            })
            .log_err();
    }

    fn edit_binding(&mut self, _: &EditBinding, window: &mut Window, cx: &mut Context<Self>) {
        self.edit_selected_keybinding(window, cx);
    }

    fn copy_context_to_clipboard(
        &mut self,
        _: &CopyContext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context = self
            .selected_binding()
            .and_then(|binding| binding.context.as_ref())
            .and_then(KeybindContextString::local_str)
            .map(|context| context.to_string());
        let Some(context) = context else {
            return;
        };
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(context.clone()));
    }

    fn copy_action_to_clipboard(
        &mut self,
        _: &CopyAction,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let action = self
            .selected_binding()
            .map(|binding| binding.action.to_string());
        let Some(action) = action else {
            return;
        };
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(action.clone()));
    }
}

#[derive(Clone)]
struct ProcessedKeybinding {
    keystroke_text: SharedString,
    ui_key_binding: Option<ui::KeyBinding>,
    action: SharedString,
    action_input: Option<SyntaxHighlightedText>,
    action_docs: Option<&'static str>,
    context: Option<KeybindContextString>,
    source: Option<(KeybindSource, SharedString)>,
}

#[derive(Clone, Debug, IntoElement)]
enum KeybindContextString {
    Global,
    Local(SharedString, Arc<Language>),
}

impl KeybindContextString {
    const GLOBAL: SharedString = SharedString::new_static("<global>");

    pub fn local(&self) -> Option<&SharedString> {
        match self {
            KeybindContextString::Global => None,
            KeybindContextString::Local(name, _) => Some(name),
        }
    }

    pub fn local_str(&self) -> Option<&str> {
        match self {
            KeybindContextString::Global => None,
            KeybindContextString::Local(name, _) => Some(name),
        }
    }
}

impl RenderOnce for KeybindContextString {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        match self {
            KeybindContextString::Global => StyledText::new(KeybindContextString::GLOBAL.clone())
                .with_highlights([(
                    0..KeybindContextString::GLOBAL.len(),
                    gpui::HighlightStyle::color(_cx.theme().colors().text_muted),
                )])
                .into_any_element(),
            KeybindContextString::Local(name, language) => {
                SyntaxHighlightedText::new(name, language).into_any_element()
            }
        }
    }
}

impl Item for KeymapEditor {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> ui::SharedString {
        "Keymap Editor".into()
    }
}

impl Render for KeymapEditor {
    fn render(&mut self, window: &mut Window, cx: &mut ui::Context<Self>) -> impl ui::IntoElement {
        let row_count = self.matches.len();
        let theme = cx.theme();

        v_flex()
            .id("keymap-editor")
            .track_focus(&self.focus_handle)
            .key_context(self.dispatch_context(window, cx))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::focus_search))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::edit_binding))
            .on_action(cx.listener(Self::copy_action_to_clipboard))
            .on_action(cx.listener(Self::copy_context_to_clipboard))
            .size_full()
            .p_2()
            .gap_1()
            .bg(theme.colors().editor_background)
            .child(
                h_flex()
                    .key_context({
                        let mut context = KeyContext::new_with_defaults();
                        context.add("BufferSearchBar");
                        context
                    })
                    .h_8()
                    .pl_2()
                    .pr_1()
                    .py_1()
                    .border_1()
                    .border_color(theme.colors().border)
                    .rounded_lg()
                    .child(self.filter_editor.clone()),
            )
            .child(
                Table::new()
                    .interactable(&self.table_interaction_state)
                    .striped()
                    .column_widths([rems(16.), rems(16.), rems(16.), rems(32.), rems(8.)])
                    .header(["Action", "Arguments", "Keystrokes", "Context", "Source"])
                    .uniform_list(
                        "keymap-editor-table",
                        row_count,
                        cx.processor(move |this, range: Range<usize>, _window, _cx| {
                            range
                                .filter_map(|index| {
                                    let candidate_id = this.matches.get(index)?.candidate_id;
                                    let binding = &this.keybindings[candidate_id];

                                    let action = div()
                                        .child(binding.action.clone())
                                        .id(("keymap action", index))
                                        .tooltip({
                                            let action_name = binding.action.clone();
                                            let action_docs = binding.action_docs;
                                            move |_, cx| {
                                                let action_tooltip = Tooltip::new(
                                                    command_palette::humanize_action_name(
                                                        &action_name,
                                                    ),
                                                );
                                                let action_tooltip = match action_docs {
                                                    Some(docs) => action_tooltip.meta(docs),
                                                    None => action_tooltip,
                                                };
                                                cx.new(|_| action_tooltip).into()
                                            }
                                        })
                                        .into_any_element();
                                    let keystrokes = binding.ui_key_binding.clone().map_or(
                                        binding.keystroke_text.clone().into_any_element(),
                                        IntoElement::into_any_element,
                                    );
                                    let action_input = binding
                                        .action_input
                                        .clone()
                                        .map_or(gpui::Empty.into_any_element(), |input| {
                                            input.into_any_element()
                                        });
                                    let context = binding
                                        .context
                                        .clone()
                                        .map_or(gpui::Empty.into_any_element(), |context| {
                                            context.into_any_element()
                                        });
                                    let source = binding
                                        .source
                                        .clone()
                                        .map(|(_source, name)| name)
                                        .unwrap_or_default()
                                        .into_any_element();
                                    Some([action, action_input, keystrokes, context, source])
                                })
                                .collect()
                        }),
                    )
                    .map_row(
                        cx.processor(|this, (row_index, row): (usize, Div), _window, cx| {
                            let is_selected = this.selected_index == Some(row_index);
                            let row = row
                                .id(("keymap-table-row", row_index))
                                .on_click(cx.listener(move |this, _event, _window, _cx| {
                                    this.selected_index = Some(row_index);
                                }))
                                .border_2()
                                .border_color(transparent_black())
                                .when(is_selected, |row| {
                                    row.border_color(cx.theme().colors().panel_focused_border)
                                });

                            right_click_menu(("keymap-table-row-menu", row_index))
                                .trigger({
                                    let this = cx.weak_entity();
                                    move |is_menu_open: bool, _window, cx| {
                                        if is_menu_open {
                                            this.update(cx, |this, cx| {
                                                if this.selected_index != Some(row_index) {
                                                    this.selected_index = Some(row_index);
                                                    cx.notify();
                                                }
                                            })
                                            .ok();
                                        }
                                        row
                                    }
                                })
                                .menu({
                                    let this = cx.weak_entity();
                                    move |window, cx| build_keybind_context_menu(&this, window, cx)
                                })
                                .into_any_element()
                        }),
                    ),
            )
    }
}

#[derive(Debug, Clone, IntoElement)]
struct SyntaxHighlightedText {
    text: SharedString,
    language: Arc<Language>,
}

impl SyntaxHighlightedText {
    pub fn new(text: impl Into<SharedString>, language: Arc<Language>) -> Self {
        Self {
            text: text.into(),
            language,
        }
    }
}

impl RenderOnce for SyntaxHighlightedText {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text_style = window.text_style();
        let syntax_theme = cx.theme().syntax();

        let text = self.text.clone();

        let highlights = self
            .language
            .highlight_text(&text.as_ref().into(), 0..text.len());
        let mut runs = Vec::with_capacity(highlights.len());
        let mut offset = 0;

        for (highlight_range, highlight_id) in highlights {
            // Add un-highlighted text before the current highlight
            if highlight_range.start > offset {
                runs.push(text_style.to_run(highlight_range.start - offset));
            }

            let mut run_style = text_style.clone();
            if let Some(highlight_style) = highlight_id.style(syntax_theme) {
                run_style = run_style.highlight(highlight_style);
            }
            // add the highlighted range
            runs.push(run_style.to_run(highlight_range.len()));
            offset = highlight_range.end;
        }

        // Add any remaining un-highlighted text
        if offset < text.len() {
            runs.push(text_style.to_run(text.len() - offset));
        }

        return StyledText::new(text).with_runs(runs);
    }
}

struct KeybindingEditorModal {
    editing_keybind: ProcessedKeybinding,
    keybind_editor: Entity<KeystrokeInput>,
    context_editor: Entity<Editor>,
    fs: Arc<dyn Fs>,
    error: Option<String>,
}

impl ModalView for KeybindingEditorModal {}

impl EventEmitter<DismissEvent> for KeybindingEditorModal {}

impl Focusable for KeybindingEditorModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.keybind_editor.focus_handle(cx)
    }
}

impl KeybindingEditorModal {
    pub fn new(
        editing_keybind: ProcessedKeybinding,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let keybind_editor = cx.new(KeystrokeInput::new);

        let context_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);

            if let Some(context) = editing_keybind
                .context
                .as_ref()
                .and_then(KeybindContextString::local)
            {
                editor.set_text(context.clone(), window, cx);
            } else {
                editor.set_placeholder_text("Keybinding context", cx);
            }

            cx.spawn(async |editor, cx| {
                let contexts = cx
                    .background_spawn(async { collect_contexts_from_assets() })
                    .await;

                editor
                    .update(cx, |editor, _cx| {
                        editor.set_completion_provider(Some(std::rc::Rc::new(
                            KeyContextCompletionProvider { contexts },
                        )));
                    })
                    .context("Failed to load completions for keybinding context")
            })
            .detach_and_log_err(cx);

            editor
        });
        Self {
            editing_keybind,
            fs,
            keybind_editor,
            context_editor,
            error: None,
        }
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        let existing_keybind = self.editing_keybind.clone();
        let fs = self.fs.clone();
        let new_keystrokes = self
            .keybind_editor
            .read_with(cx, |editor, _| editor.keystrokes().to_vec());
        if new_keystrokes.is_empty() {
            self.error = Some("Keystrokes cannot be empty".to_string());
            cx.notify();
            return;
        }
        let tab_size = cx.global::<settings::SettingsStore>().json_tab_size();
        let new_context = self
            .context_editor
            .read_with(cx, |editor, cx| editor.text(cx));
        let new_context = new_context.is_empty().not().then_some(new_context);
        let new_context_err = new_context.as_deref().and_then(|context| {
            gpui::KeyBindingContextPredicate::parse(context)
                .context("Failed to parse key context")
                .err()
        });
        if let Some(err) = new_context_err {
            // TODO: store and display as separate error
            // TODO: also, should be validating on keystroke
            self.error = Some(err.to_string());
            cx.notify();
            return;
        }

        cx.spawn(async move |this, cx| {
            if let Err(err) = save_keybinding_update(
                existing_keybind,
                &new_keystrokes,
                new_context.as_deref(),
                &fs,
                tab_size,
            )
            .await
            {
                this.update(cx, |this, cx| {
                    this.error = Some(err.to_string());
                    cx.notify();
                })
                .log_err();
            } else {
                this.update(cx, |_this, cx| {
                    cx.emit(DismissEvent);
                })
                .ok();
            }
        })
        .detach();
    }
}

impl Render for KeybindingEditorModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().colors();

        return v_flex()
            .w(rems(34.))
            .elevation_3(cx)
            .child(
                v_flex()
                    .p_3()
                    .gap_2()
                    .child(
                        v_flex().child(Label::new("Edit Keystroke")).child(
                            Label::new("Input the desired keystroke for the selected action.")
                                .color(Color::Muted),
                        ),
                    )
                    .child(self.keybind_editor.clone()),
            )
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    .child(
                        v_flex().child(Label::new("Edit Keystroke")).child(
                            Label::new("Input the desired keystroke for the selected action.")
                                .color(Color::Muted),
                        ),
                    )
                    .child(
                        div()
                            .w_full()
                            .border_color(cx.theme().colors().border_variant)
                            .border_1()
                            .py_2()
                            .px_3()
                            .min_h_8()
                            .rounded_md()
                            .bg(theme.editor_background)
                            .child(self.context_editor.clone()),
                    ),
            )
            .child(
                h_flex()
                    .p_2()
                    .w_full()
                    .gap_1()
                    .justify_end()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Button::new("cancel", "Cancel")
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    )
                    .child(
                        Button::new("save-btn", "Save").on_click(
                            cx.listener(|this, _event, _window, cx| Self::save(this, cx)),
                        ),
                    ),
            )
            .when_some(self.error.clone(), |this, error| {
                this.child(
                    div()
                        .bg(theme.background)
                        .border_color(theme.border)
                        .border_2()
                        .rounded_md()
                        .child(error),
                )
            });
    }
}

struct KeyContextCompletionProvider {
    contexts: Vec<SharedString>,
}

impl CompletionProvider for KeyContextCompletionProvider {
    fn completions(
        &self,
        _excerpt_id: editor::ExcerptId,
        buffer: &Entity<language::Buffer>,
        buffer_position: language::Anchor,
        _trigger: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> gpui::Task<anyhow::Result<Vec<project::CompletionResponse>>> {
        let buffer = buffer.read(cx);
        let mut count_back = 0;
        for char in buffer.reversed_chars_at(buffer_position) {
            if char.is_ascii_alphanumeric() || char == '_' {
                count_back += 1;
            } else {
                break;
            }
        }
        let start_anchor = buffer.anchor_before(
            buffer_position
                .to_offset(&buffer)
                .saturating_sub(count_back),
        );
        let replace_range = start_anchor..buffer_position;
        gpui::Task::ready(Ok(vec![project::CompletionResponse {
            completions: self
                .contexts
                .iter()
                .map(|context| project::Completion {
                    replace_range: replace_range.clone(),
                    label: language::CodeLabel::plain(context.to_string(), None),
                    new_text: context.to_string(),
                    documentation: None,
                    source: project::CompletionSource::Custom,
                    icon_path: None,
                    insert_text_mode: None,
                    confirm: None,
                })
                .collect(),
            is_incomplete: false,
        }]))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<language::Buffer>,
        _position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        _menu_is_open: bool,
        _cx: &mut Context<Editor>,
    ) -> bool {
        text.chars().last().map_or(false, |last_char| {
            last_char.is_ascii_alphanumeric() || last_char == '_'
        })
    }
}

async fn save_keybinding_update(
    existing: ProcessedKeybinding,
    new_keystrokes: &[Keystroke],
    new_context: Option<&str>,
    fs: &Arc<dyn Fs>,
    tab_size: usize,
) -> anyhow::Result<()> {
    let keymap_contents = settings::KeymapFile::load_keymap_file(fs)
        .await
        .context("Failed to load keymap file")?;

    let existing_keystrokes = existing
        .ui_key_binding
        .as_ref()
        .map(|keybinding| keybinding.keystrokes.as_slice())
        .unwrap_or_default();

    let existing_context = existing
        .context
        .as_ref()
        .and_then(KeybindContextString::local_str);

    let input = existing
        .action_input
        .as_ref()
        .map(|input| input.text.as_ref());

    let operation = if existing.ui_key_binding.is_some() {
        settings::KeybindUpdateOperation::Replace {
            target: settings::KeybindUpdateTarget {
                context: existing_context,
                keystrokes: existing_keystrokes,
                action_name: &existing.action,
                use_key_equivalents: false,
                input,
            },
            target_keybind_source: existing
                .source
                .map(|(source, _name)| source)
                .unwrap_or(KeybindSource::User),
            source: settings::KeybindUpdateTarget {
                context: new_context,
                keystrokes: new_keystrokes,
                action_name: &existing.action,
                use_key_equivalents: false,
                input,
            },
        }
    } else {
        anyhow::bail!("Adding new bindings not implemented yet");
    };
    let updated_keymap_contents =
        settings::KeymapFile::update_keybinding(operation, keymap_contents, tab_size)
            .context("Failed to update keybinding")?;
    fs.atomic_write(paths::keymap_file().clone(), updated_keymap_contents)
        .await
        .context("Failed to write keymap file")?;
    Ok(())
}

struct KeystrokeInput {
    keystrokes: Vec<Keystroke>,
    focus_handle: FocusHandle,
}

impl KeystrokeInput {
    fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            keystrokes: Vec::new(),
            focus_handle,
        }
    }

    fn on_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(last) = self.keystrokes.last_mut()
            && last.key.is_empty()
        {
            if !event.modifiers.modified() {
                self.keystrokes.pop();
            } else {
                last.modifiers = event.modifiers;
            }
        } else {
            self.keystrokes.push(Keystroke {
                modifiers: event.modifiers,
                key: "".to_string(),
                key_char: None,
            });
        }
        cx.stop_propagation();
        cx.notify();
    }

    fn on_key_down(
        &mut self,
        event: &gpui::KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.is_held {
            return;
        }
        if let Some(last) = self.keystrokes.last_mut()
            && last.key.is_empty()
        {
            *last = event.keystroke.clone();
        } else {
            self.keystrokes.push(event.keystroke.clone());
        }
        cx.stop_propagation();
        cx.notify();
    }

    fn on_key_up(
        &mut self,
        event: &gpui::KeyUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(last) = self.keystrokes.last_mut()
            && !last.key.is_empty()
            && last.modifiers == event.keystroke.modifiers
        {
            self.keystrokes.push(Keystroke {
                modifiers: event.keystroke.modifiers,
                key: "".to_string(),
                key_char: None,
            });
        }
        cx.stop_propagation();
        cx.notify();
    }

    fn keystrokes(&self) -> &[Keystroke] {
        if self
            .keystrokes
            .last()
            .map_or(false, |last| last.key.is_empty())
        {
            return &self.keystrokes[..self.keystrokes.len() - 1];
        }
        return &self.keystrokes;
    }
}

impl Focusable for KeystrokeInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for KeystrokeInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        return h_flex()
            .id("keybinding_input")
            .track_focus(&self.focus_handle)
            .on_modifiers_changed(cx.listener(Self::on_modifiers_changed))
            .on_key_down(cx.listener(Self::on_key_down))
            .on_key_up(cx.listener(Self::on_key_up))
            .focus(|mut style| {
                style.border_color = Some(colors.border_focused);
                style
            })
            .py_2()
            .px_3()
            .gap_2()
            .min_h_8()
            .w_full()
            .justify_between()
            .bg(colors.editor_background)
            .border_1()
            .rounded_md()
            .flex_1()
            .overflow_hidden()
            .child(
                h_flex()
                    .w_full()
                    .min_w_0()
                    .justify_center()
                    .flex_wrap()
                    .gap(ui::DynamicSpacing::Base04.rems(cx))
                    .children(self.keystrokes.iter().map(|keystroke| {
                        h_flex().children(ui::render_keystroke(
                            keystroke,
                            None,
                            Some(rems(0.875).into()),
                            ui::PlatformStyle::platform(),
                            false,
                        ))
                    })),
            )
            .child(
                h_flex()
                    .gap_0p5()
                    .flex_none()
                    .child(
                        IconButton::new("backspace-btn", IconName::Delete)
                            .tooltip(Tooltip::text("Delete Keystroke"))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.keystrokes.pop();
                                cx.notify();
                            })),
                    )
                    .child(
                        IconButton::new("clear-btn", IconName::Eraser)
                            .tooltip(Tooltip::text("Clear Keystrokes"))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.keystrokes.clear();
                                cx.notify();
                            })),
                    ),
            );
    }
}

fn build_keybind_context_menu(
    this: &WeakEntity<KeymapEditor>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ContextMenu> {
    ContextMenu::build(window, cx, |menu, _window, cx| {
        let Some(this) = this.upgrade() else {
            return menu;
        };
        let selected_binding = this.read_with(cx, |this, _cx| this.selected_binding().cloned());
        let Some(selected_binding) = selected_binding else {
            return menu;
        };

        let selected_binding_has_context = selected_binding
            .context
            .as_ref()
            .and_then(KeybindContextString::local)
            .is_some();

        menu.action("Edit Binding", Box::new(EditBinding))
            .action("Copy action", Box::new(CopyAction))
            .action_disabled_when(
                !selected_binding_has_context,
                "Copy Context",
                Box::new(CopyContext),
            )
    })
}

fn collect_contexts_from_assets() -> Vec<SharedString> {
    let mut keymap_assets = vec![
        util::asset_str::<SettingsAssets>(settings::DEFAULT_KEYMAP_PATH),
        util::asset_str::<SettingsAssets>(settings::VIM_KEYMAP_PATH),
    ];
    keymap_assets.extend(
        BaseKeymap::OPTIONS
            .iter()
            .filter_map(|(_, base_keymap)| base_keymap.asset_path())
            .map(util::asset_str::<SettingsAssets>),
    );

    let mut contexts = HashSet::default();

    for keymap_asset in keymap_assets {
        let Ok(keymap) = KeymapFile::parse(&keymap_asset) else {
            continue;
        };

        for section in keymap.sections() {
            let context_expr = &section.context;
            let mut queue = Vec::new();
            let Ok(root_context) = gpui::KeyBindingContextPredicate::parse(context_expr) else {
                continue;
            };

            queue.push(root_context);
            while let Some(context) = queue.pop() {
                match context {
                    gpui::KeyBindingContextPredicate::Identifier(ident) => {
                        contexts.insert(ident);
                    }
                    gpui::KeyBindingContextPredicate::Equal(ident_a, ident_b) => {
                        contexts.insert(ident_a);
                        contexts.insert(ident_b);
                    }
                    gpui::KeyBindingContextPredicate::NotEqual(ident_a, ident_b) => {
                        contexts.insert(ident_a);
                        contexts.insert(ident_b);
                    }
                    gpui::KeyBindingContextPredicate::Child(ctx_a, ctx_b) => {
                        queue.push(*ctx_a);
                        queue.push(*ctx_b);
                    }
                    gpui::KeyBindingContextPredicate::Not(ctx) => {
                        queue.push(*ctx);
                    }
                    gpui::KeyBindingContextPredicate::And(ctx_a, ctx_b) => {
                        queue.push(*ctx_a);
                        queue.push(*ctx_b);
                    }
                    gpui::KeyBindingContextPredicate::Or(ctx_a, ctx_b) => {
                        queue.push(*ctx_a);
                        queue.push(*ctx_b);
                    }
                }
            }
        }
    }

    let mut contexts = contexts.into_iter().collect::<Vec<_>>();
    contexts.sort();

    return contexts;
}

impl SerializableItem for KeymapEditor {
    fn serialized_item_kind() -> &'static str {
        "KeymapEditor"
    }

    fn cleanup(
        workspace_id: workspace::WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "keybinding_editors",
            &KEYBINDING_EDITORS,
            cx,
        )
    }

    fn deserialize(
        _project: Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            if KEYBINDING_EDITORS
                .get_keybinding_editor(item_id, workspace_id)?
                .is_some()
            {
                cx.update(|window, cx| cx.new(|cx| KeymapEditor::new(workspace, window, cx)))
            } else {
                Err(anyhow!("No keybinding editor to deserialize"))
            }
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut ui::Context<Self>,
    ) -> Option<gpui::Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        Some(cx.background_spawn(async move {
            KEYBINDING_EDITORS
                .save_keybinding_editor(item_id, workspace_id)
                .await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

mod persistence {
    use db::{define_connection, query, sqlez_macros::sql};
    use workspace::WorkspaceDb;

    define_connection! {
        pub static ref KEYBINDING_EDITORS: KeybindingEditorDb<WorkspaceDb> =
            &[sql!(
                CREATE TABLE keybinding_editors (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
            )];
    }

    impl KeybindingEditorDb {
        query! {
            pub async fn save_keybinding_editor(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<()> {
                INSERT OR REPLACE INTO keybinding_editors(item_id, workspace_id)
                VALUES (?, ?)
            }
        }

        query! {
            pub fn get_keybinding_editor(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<Option<workspace::ItemId>> {
                SELECT item_id
                FROM keybinding_editors
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
