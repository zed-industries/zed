use std::{
    ops::{Not, Range},
    sync::Arc,
};

use anyhow::{Context as _, anyhow};
use collections::{HashMap, HashSet};
use editor::{CompletionProvider, Editor, EditorEvent};
use feature_flags::FeatureFlagViewExt;
use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, Animation, AnimationExt, AppContext as _, AsyncApp, Axis, ClickEvent, Context,
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, Global, IsZero,
    KeyContext, Keystroke, Modifiers, ModifiersChangedEvent, MouseButton, Point, ScrollStrategy,
    ScrollWheelEvent, StyledText, Subscription, WeakEntity, actions, anchored, deferred, div,
};
use language::{Language, LanguageConfig, ToOffset as _};
use notifications::status_toast::{StatusToast, ToastIcon};
use settings::{BaseKeymap, KeybindSource, KeymapFile, SettingsAssets};

use util::ResultExt;

use ui::{
    ActiveTheme as _, App, Banner, BorrowAppContext, ContextMenu, IconButtonShape, Modal,
    ModalFooter, ModalHeader, ParentElement as _, Render, Section, SharedString, Styled as _,
    Tooltip, Window, prelude::*,
};
use ui_input::SingleLineInput;
use workspace::{
    Item, ModalView, SerializableItem, Workspace, notifications::NotifyTaskExt as _,
    register_serializable_item,
};

use crate::{
    SettingsUiFeatureFlag,
    keybindings::persistence::KEYBINDING_EDITORS,
    ui_components::table::{Table, TableInteractionState},
};

const NO_ACTION_ARGUMENTS_TEXT: SharedString = SharedString::new_static("<no arguments>");

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
        /// Creates a new key binding for the selected action.
        CreateBinding,
        /// Deletes the selected key binding.
        DeleteBinding,
        /// Copies the action name to clipboard.
        CopyAction,
        /// Copies the context predicate to clipboard.
        CopyContext,
        /// Toggles Conflict Filtering
        ToggleConflictFilter,
        /// Toggle Keystroke search
        ToggleKeystrokeSearch,
    ]
);

actions!(
    keystroke_input,
    [
        /// Starts recording keystrokes
        StartRecording,
        /// Stops recording keystrokes
        StopRecording,
        /// Clears the recorded keystrokes
        ClearKeystrokes,
    ]
);

pub fn init(cx: &mut App) {
    let keymap_event_channel = KeymapEventChannel::new();
    cx.set_global(keymap_event_channel);

    cx.on_action(|_: &OpenKeymapEditor, cx| {
        workspace::with_active_or_new_workspace(cx, move |workspace, window, cx| {
            workspace
                .with_local_workspace(window, cx, |workspace, window, cx| {
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
                        workspace.add_item_to_active_pane(
                            Box::new(keymap_editor),
                            None,
                            true,
                            window,
                            cx,
                        );
                    }
                })
                .detach();
        })
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

#[derive(Default, PartialEq)]
enum SearchMode {
    #[default]
    Normal,
    KeyStroke,
}

impl SearchMode {
    fn invert(&self) -> Self {
        match self {
            SearchMode::Normal => SearchMode::KeyStroke,
            SearchMode::KeyStroke => SearchMode::Normal,
        }
    }
}

#[derive(Default, PartialEq, Copy, Clone)]
enum FilterState {
    #[default]
    All,
    Conflicts,
}

impl FilterState {
    fn invert(&self) -> Self {
        match self {
            FilterState::All => FilterState::Conflicts,
            FilterState::Conflicts => FilterState::All,
        }
    }
}

type ActionMapping = (SharedString, Option<SharedString>);

#[derive(Default)]
struct ConflictState {
    conflicts: Vec<usize>,
    action_keybind_mapping: HashMap<ActionMapping, Vec<usize>>,
}

impl ConflictState {
    fn new(key_bindings: &[ProcessedKeybinding]) -> Self {
        let mut action_keybind_mapping: HashMap<_, Vec<usize>> = HashMap::default();

        key_bindings
            .iter()
            .enumerate()
            .filter(|(_, binding)| {
                !binding.keystroke_text.is_empty()
                    && binding
                        .source
                        .as_ref()
                        .is_some_and(|source| matches!(source.0, KeybindSource::User))
            })
            .for_each(|(index, binding)| {
                action_keybind_mapping
                    .entry(binding.get_action_mapping())
                    .or_default()
                    .push(index);
            });

        Self {
            conflicts: action_keybind_mapping
                .values()
                .filter(|indices| indices.len() > 1)
                .flatten()
                .copied()
                .collect(),
            action_keybind_mapping,
        }
    }

    fn conflicting_indices_for_mapping(
        &self,
        action_mapping: ActionMapping,
        keybind_idx: usize,
    ) -> Option<Vec<usize>> {
        self.action_keybind_mapping
            .get(&action_mapping)
            .and_then(|indices| {
                let mut indices = indices.iter().filter(|&idx| *idx != keybind_idx).peekable();
                indices.peek().is_some().then(|| indices.copied().collect())
            })
    }

    fn has_conflict(&self, candidate_idx: &usize) -> bool {
        self.conflicts.contains(candidate_idx)
    }

    fn any_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

struct KeymapEditor {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    _keymap_subscription: Subscription,
    keybindings: Vec<ProcessedKeybinding>,
    keybinding_conflict_state: ConflictState,
    filter_state: FilterState,
    search_mode: SearchMode,
    // corresponds 1 to 1 with keybindings
    string_match_candidates: Arc<Vec<StringMatchCandidate>>,
    matches: Vec<StringMatch>,
    table_interaction_state: Entity<TableInteractionState>,
    filter_editor: Entity<Editor>,
    keystroke_editor: Entity<KeystrokeInput>,
    selected_index: Option<usize>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    previous_edit: Option<PreviousEdit>,
}

enum PreviousEdit {
    /// When deleting, we want to maintain the same scroll position
    ScrollBarOffset(Point<Pixels>),
    /// When editing or creating, because the new keybinding could be in a different position in the sort order
    /// we store metadata about the new binding (either the modified version or newly created one)
    /// and upon reload, we search for this binding in the list of keybindings, and if we find the one that matches
    /// this metadata, we set the selected index to it and scroll to it,
    /// and if we don't find it, we scroll to 0 and don't set a selected index
    Keybinding {
        action_mapping: ActionMapping,
        action_name: SharedString,
        /// The scrollbar position to fallback to if we don't find the keybinding during a refresh
        /// this can happen if there's a filter applied to the search and the keybinding modification
        /// filters the binding from the search results
        fallback: Point<Pixels>,
    },
}

impl EventEmitter<()> for KeymapEditor {}

impl Focusable for KeymapEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        return self.filter_editor.focus_handle(cx);
    }
}

impl KeymapEditor {
    fn new(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let _keymap_subscription = cx.observe_global::<KeymapEventChannel>(Self::on_keymap_changed);
        let table_interaction_state = TableInteractionState::new(window, cx);

        let keystroke_editor = cx.new(|cx| {
            let mut keystroke_editor = KeystrokeInput::new(None, window, cx);
            keystroke_editor.highlight_on_focus = false;
            keystroke_editor
        });

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Filter action names…", cx);
            editor
        });

        cx.subscribe(&filter_editor, |this, _, e: &EditorEvent, cx| {
            if !matches!(e, EditorEvent::BufferEdited) {
                return;
            }

            this.on_query_changed(cx);
        })
        .detach();

        cx.subscribe(&keystroke_editor, |this, _, _, cx| {
            if matches!(this.search_mode, SearchMode::Normal) {
                return;
            }

            this.on_query_changed(cx);
        })
        .detach();

        let mut this = Self {
            workspace,
            keybindings: vec![],
            keybinding_conflict_state: ConflictState::default(),
            filter_state: FilterState::default(),
            search_mode: SearchMode::default(),
            string_match_candidates: Arc::new(vec![]),
            matches: vec![],
            focus_handle: cx.focus_handle(),
            _keymap_subscription,
            table_interaction_state,
            filter_editor,
            keystroke_editor,
            selected_index: None,
            context_menu: None,
            previous_edit: None,
        };

        this.on_keymap_changed(cx);

        this
    }

    fn current_action_query(&self, cx: &App) -> String {
        self.filter_editor.read(cx).text(cx)
    }

    fn current_keystroke_query(&self, cx: &App) -> Vec<Keystroke> {
        match self.search_mode {
            SearchMode::KeyStroke => self
                .keystroke_editor
                .read(cx)
                .keystrokes()
                .iter()
                .cloned()
                .collect(),
            SearchMode::Normal => Default::default(),
        }
    }

    fn on_query_changed(&self, cx: &mut Context<Self>) {
        let action_query = self.current_action_query(cx);
        let keystroke_query = self.current_keystroke_query(cx);

        cx.spawn(async move |this, cx| {
            Self::update_matches(this.clone(), action_query, keystroke_query, cx).await?;
            this.update(cx, |this, cx| {
                this.scroll_to_item(0, ScrollStrategy::Top, cx)
            })
        })
        .detach();
    }

    async fn update_matches(
        this: WeakEntity<Self>,
        action_query: String,
        keystroke_query: Vec<Keystroke>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        let action_query = command_palette::normalize_action_query(&action_query);
        let (string_match_candidates, keybind_count) = this.read_with(cx, |this, _| {
            (this.string_match_candidates.clone(), this.keybindings.len())
        })?;
        let executor = cx.background_executor().clone();
        let mut matches = fuzzy::match_strings(
            &string_match_candidates,
            &action_query,
            true,
            true,
            keybind_count,
            &Default::default(),
            executor,
        )
        .await;
        this.update(cx, |this, cx| {
            match this.filter_state {
                FilterState::Conflicts => {
                    matches.retain(|candidate| {
                        this.keybinding_conflict_state
                            .has_conflict(&candidate.candidate_id)
                    });
                }
                FilterState::All => {}
            }

            match this.search_mode {
                SearchMode::KeyStroke => {
                    matches.retain(|item| {
                        this.keybindings[item.candidate_id]
                            .keystrokes()
                            .is_some_and(|keystrokes| {
                                keystroke_query.iter().all(|key| {
                                    keystrokes.iter().any(|keystroke| {
                                        keystroke.key == key.key
                                            && keystroke.modifiers == key.modifiers
                                    })
                                })
                            })
                    });
                }
                SearchMode::Normal => {}
            }

            if action_query.is_empty() {
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
                    return (source_precedence, keybind.action_name.as_ref());
                });
            }
            this.selected_index.take();
            this.matches = matches;
            cx.notify();
        })
    }

    fn has_conflict(&self, row_index: usize) -> bool {
        self.matches
            .get(row_index)
            .map(|candidate| candidate.candidate_id)
            .is_some_and(|id| self.keybinding_conflict_state.has_conflict(&id))
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
        let mut generator = KeymapFile::action_schema_generator();
        let action_schema = HashMap::from_iter(
            cx.action_schemas(&mut generator)
                .into_iter()
                .filter_map(|(name, schema)| schema.map(|schema| (name, schema))),
        );

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
            let action_arguments = key_binding
                .action_input()
                .map(|arguments| SyntaxHighlightedText::new(arguments, json_language.clone()));
            let action_docs = action_documentation.get(action_name).copied();

            let index = processed_bindings.len();
            let string_match_candidate = StringMatchCandidate::new(index, &action_name);
            processed_bindings.push(ProcessedKeybinding {
                keystroke_text: keystroke_text.into(),
                ui_key_binding,
                action_name: action_name.into(),
                action_arguments,
                action_docs,
                action_schema: action_schema.get(action_name).cloned(),
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
                action_name: action_name.into(),
                action_arguments: None,
                action_docs: action_documentation.get(action_name).copied(),
                action_schema: action_schema.get(action_name).cloned(),
                context: None,
                source: None,
            });
            string_match_candidates.push(string_match_candidate);
        }

        (processed_bindings, string_match_candidates)
    }

    fn on_keymap_changed(&mut self, cx: &mut Context<KeymapEditor>) {
        let workspace = self.workspace.clone();
        cx.spawn(async move |this, cx| {
            let json_language = load_json_language(workspace.clone(), cx).await;
            let rust_language = load_rust_language(workspace.clone(), cx).await;

            let (action_query, keystroke_query) = this.update(cx, |this, cx| {
                let (key_bindings, string_match_candidates) =
                    Self::process_bindings(json_language, rust_language, cx);

                this.keybinding_conflict_state = ConflictState::new(&key_bindings);

                if !this.keybinding_conflict_state.any_conflicts() {
                    this.filter_state = FilterState::All;
                }

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
                (
                    this.current_action_query(cx),
                    this.current_keystroke_query(cx),
                )
            })?;
            // calls cx.notify
            Self::update_matches(this.clone(), action_query, keystroke_query, cx).await?;
            this.update(cx, |this, cx| {
                if let Some(previous_edit) = this.previous_edit.take() {
                    match previous_edit {
                        // should remove scroll from process_query
                        PreviousEdit::ScrollBarOffset(offset) => {
                            this.table_interaction_state.update(cx, |table, _| {
                                table.set_scrollbar_offset(Axis::Vertical, offset)
                            })
                            // set selected index and scroll
                        }
                        PreviousEdit::Keybinding {
                            action_mapping,
                            action_name,
                            fallback,
                        } => {
                            let scroll_position =
                                this.matches.iter().enumerate().find_map(|(index, item)| {
                                    let binding = &this.keybindings[item.candidate_id];
                                    if binding.get_action_mapping() == action_mapping
                                        && binding.action_name == action_name
                                    {
                                        Some(index)
                                    } else {
                                        None
                                    }
                                });

                            if let Some(scroll_position) = scroll_position {
                                this.scroll_to_item(scroll_position, ScrollStrategy::Top, cx);
                                this.selected_index = Some(scroll_position);
                            } else {
                                this.table_interaction_state.update(cx, |table, _| {
                                    table.set_scrollbar_offset(Axis::Vertical, fallback)
                                });
                            }
                            cx.notify();
                        }
                    }
                }
            })
        })
        .detach_and_log_err(cx);
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

    fn selected_keybind_idx(&self) -> Option<usize> {
        self.selected_index
            .and_then(|match_index| self.matches.get(match_index))
            .map(|r#match| r#match.candidate_id)
    }

    fn selected_binding(&self) -> Option<&ProcessedKeybinding> {
        self.selected_keybind_idx()
            .and_then(|keybind_index| self.keybindings.get(keybind_index))
    }

    fn select_index(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.selected_index != Some(index) {
            self.selected_index = Some(index);
            cx.notify();
        }
    }

    fn create_context_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_menu = self.selected_binding().map(|selected_binding| {
            let selected_binding_has_no_context = selected_binding
                .context
                .as_ref()
                .and_then(KeybindContextString::local)
                .is_none();

            let selected_binding_is_unbound = selected_binding.keystrokes().is_none();

            let context_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
                menu.action_disabled_when(
                    selected_binding_is_unbound,
                    "Edit",
                    Box::new(EditBinding),
                )
                .action("Create", Box::new(CreateBinding))
                .action_disabled_when(
                    selected_binding_is_unbound,
                    "Delete",
                    Box::new(DeleteBinding),
                )
                .separator()
                .action("Copy Action", Box::new(CopyAction))
                .action_disabled_when(
                    selected_binding_has_no_context,
                    "Copy Context",
                    Box::new(CopyContext),
                )
            });

            let context_menu_handle = context_menu.focus_handle(cx);
            window.defer(cx, move |window, _cx| window.focus(&context_menu_handle));
            let subscription = cx.subscribe_in(
                &context_menu,
                window,
                |this, _, _: &DismissEvent, window, cx| {
                    this.dismiss_context_menu(window, cx);
                },
            );
            (context_menu, position, subscription)
        });

        cx.notify();
    }

    fn dismiss_context_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.context_menu.take();
        window.focus(&self.focus_handle);
        cx.notify();
    }

    fn context_menu_deployed(&self) -> bool {
        self.context_menu.is_some()
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
        self.open_edit_keybinding_modal(false, window, cx);
    }

    fn open_edit_keybinding_modal(
        &mut self,
        create: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((keybind_idx, keybind)) = self
            .selected_keybind_idx()
            .zip(self.selected_binding().cloned())
        else {
            return;
        };
        let keymap_editor = cx.entity();
        self.workspace
            .update(cx, |workspace, cx| {
                let fs = workspace.app_state().fs.clone();
                let workspace_weak = cx.weak_entity();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let modal = KeybindingEditorModal::new(
                        create,
                        keybind,
                        keybind_idx,
                        keymap_editor,
                        workspace_weak,
                        fs,
                        window,
                        cx,
                    );
                    window.focus(&modal.focus_handle(cx));
                    modal
                });
            })
            .log_err();
    }

    fn edit_binding(&mut self, _: &EditBinding, window: &mut Window, cx: &mut Context<Self>) {
        self.open_edit_keybinding_modal(false, window, cx);
    }

    fn create_binding(&mut self, _: &CreateBinding, window: &mut Window, cx: &mut Context<Self>) {
        self.open_edit_keybinding_modal(true, window, cx);
    }

    fn delete_binding(&mut self, _: &DeleteBinding, window: &mut Window, cx: &mut Context<Self>) {
        let Some(to_remove) = self.selected_binding().cloned() else {
            return;
        };

        let Ok(fs) = self
            .workspace
            .read_with(cx, |workspace, _| workspace.app_state().fs.clone())
        else {
            return;
        };
        let tab_size = cx.global::<settings::SettingsStore>().json_tab_size();
        self.previous_edit = Some(PreviousEdit::ScrollBarOffset(
            self.table_interaction_state
                .read(cx)
                .get_scrollbar_offset(Axis::Vertical),
        ));
        cx.spawn(async move |_, _| remove_keybinding(to_remove, &fs, tab_size).await)
            .detach_and_notify_err(window, cx);
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
            .map(|binding| binding.action_name.to_string());
        let Some(action) = action else {
            return;
        };
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(action.clone()));
    }

    fn toggle_conflict_filter(
        &mut self,
        _: &ToggleConflictFilter,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_filter_state(self.filter_state.invert(), cx);
    }

    fn set_filter_state(&mut self, filter_state: FilterState, cx: &mut Context<Self>) {
        if self.filter_state != filter_state {
            self.filter_state = filter_state;
            self.on_query_changed(cx);
        }
    }

    fn toggle_keystroke_search(
        &mut self,
        _: &ToggleKeystrokeSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.search_mode = self.search_mode.invert();
        self.on_query_changed(cx);

        // Update the keystroke editor to turn the `search` bool on
        self.keystroke_editor.update(cx, |keystroke_editor, cx| {
            keystroke_editor.set_search_mode(self.search_mode == SearchMode::KeyStroke);
            cx.notify();
        });

        match self.search_mode {
            SearchMode::KeyStroke => {
                window.focus(&self.keystroke_editor.read(cx).recording_focus_handle(cx));
            }
            SearchMode::Normal => {}
        }
    }
}

#[derive(Clone)]
struct ProcessedKeybinding {
    keystroke_text: SharedString,
    ui_key_binding: Option<ui::KeyBinding>,
    action_name: SharedString,
    action_arguments: Option<SyntaxHighlightedText>,
    action_docs: Option<&'static str>,
    action_schema: Option<schemars::Schema>,
    context: Option<KeybindContextString>,
    source: Option<(KeybindSource, SharedString)>,
}

impl ProcessedKeybinding {
    fn get_action_mapping(&self) -> ActionMapping {
        (
            self.keystroke_text.clone(),
            self.context
                .as_ref()
                .and_then(|context| context.local())
                .cloned(),
        )
    }

    fn keystrokes(&self) -> Option<&[Keystroke]> {
        self.ui_key_binding
            .as_ref()
            .map(|binding| binding.keystrokes.as_slice())
    }
}

#[derive(Clone, Debug, IntoElement, PartialEq, Eq, Hash)]
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        match self {
            KeybindContextString::Global => {
                muted_styled_text(KeybindContextString::GLOBAL.clone(), cx).into_any_element()
            }
            KeybindContextString::Local(name, language) => {
                SyntaxHighlightedText::new(name, language).into_any_element()
            }
        }
    }
}

fn muted_styled_text(text: SharedString, cx: &App) -> StyledText {
    let len = text.len();
    StyledText::new(text).with_highlights([(
        0..len,
        gpui::HighlightStyle::color(cx.theme().colors().text_muted),
    )])
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
            .on_action(cx.listener(Self::create_binding))
            .on_action(cx.listener(Self::delete_binding))
            .on_action(cx.listener(Self::copy_action_to_clipboard))
            .on_action(cx.listener(Self::copy_context_to_clipboard))
            .on_action(cx.listener(Self::toggle_conflict_filter))
            .on_action(cx.listener(Self::toggle_keystroke_search))
            .size_full()
            .p_2()
            .gap_1()
            .bg(theme.colors().editor_background)
            .child(
                v_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                div()
                                    .key_context({
                                        let mut context = KeyContext::new_with_defaults();
                                        context.add("BufferSearchBar");
                                        context
                                    })
                                    .size_full()
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
                                IconButton::new(
                                    "KeymapEditorToggleFiltersIcon",
                                    IconName::Keyboard,
                                )
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(|window, cx| {
                                    Tooltip::for_action(
                                        "Search by Keystroke",
                                        &ToggleKeystrokeSearch,
                                        window,
                                        cx,
                                    )
                                })
                                .toggle_state(matches!(self.search_mode, SearchMode::KeyStroke))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(ToggleKeystrokeSearch.boxed_clone(), cx);
                                }),
                            )
                            .when(self.keybinding_conflict_state.any_conflicts(), |this| {
                                this.child(
                                    IconButton::new("KeymapEditorConflictIcon", IconName::Warning)
                                        .shape(ui::IconButtonShape::Square)
                                        .tooltip({
                                            let filter_state = self.filter_state;

                                            move |window, cx| {
                                                Tooltip::for_action(
                                                    match filter_state {
                                                        FilterState::All => "Show Conflicts",
                                                        FilterState::Conflicts => "Hide Conflicts",
                                                    },
                                                    &ToggleConflictFilter,
                                                    window,
                                                    cx,
                                                )
                                            }
                                        })
                                        .selected_icon_color(Color::Warning)
                                        .toggle_state(matches!(
                                            self.filter_state,
                                            FilterState::Conflicts
                                        ))
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(
                                                ToggleConflictFilter.boxed_clone(),
                                                cx,
                                            );
                                        }),
                                )
                            }),
                    )
                    .when(matches!(self.search_mode, SearchMode::KeyStroke), |this| {
                        this.child(
                            div()
                                .map(|this| {
                                    if self.keybinding_conflict_state.any_conflicts() {
                                        this.pr(rems_from_px(54.))
                                    } else {
                                        this.pr_7()
                                    }
                                })
                                .child(self.keystroke_editor.clone()),
                        )
                    }),
            )
            .child(
                Table::new()
                    .interactable(&self.table_interaction_state)
                    .striped()
                    .column_widths([
                        rems(2.5),
                        rems(16.),
                        rems(16.),
                        rems(16.),
                        rems(32.),
                        rems(8.),
                    ])
                    .header(["", "Action", "Arguments", "Keystrokes", "Context", "Source"])
                    .uniform_list(
                        "keymap-editor-table",
                        row_count,
                        cx.processor(move |this, range: Range<usize>, _window, cx| {
                            let context_menu_deployed = this.context_menu_deployed();
                            range
                                .filter_map(|index| {
                                    let candidate_id = this.matches.get(index)?.candidate_id;
                                    let binding = &this.keybindings[candidate_id];
                                    let action_name = binding.action_name.clone();

                                    let icon = (this.filter_state != FilterState::Conflicts
                                        && this.has_conflict(index))
                                    .then(|| {
                                        base_button_style(index, IconName::Warning)
                                            .icon_color(Color::Warning)
                                            .tooltip(|window, cx| {
                                                Tooltip::with_meta(
                                                    "Edit Keybinding",
                                                    None,
                                                    "Use alt+click to show conflicts",
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .on_click(cx.listener(
                                                move |this, click: &ClickEvent, window, cx| {
                                                    if click.modifiers().alt {
                                                        this.set_filter_state(
                                                            FilterState::Conflicts,
                                                            cx,
                                                        );
                                                    } else {
                                                        this.select_index(index, cx);
                                                        this.open_edit_keybinding_modal(
                                                            false, window, cx,
                                                        );
                                                        cx.stop_propagation();
                                                    }
                                                },
                                            ))
                                    })
                                    .unwrap_or_else(|| {
                                        base_button_style(index, IconName::Pencil)
                                            .visible_on_hover(row_group_id(index))
                                            .tooltip(Tooltip::text("Edit Keybinding"))
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.select_index(index, cx);
                                                this.open_edit_keybinding_modal(false, window, cx);
                                                cx.stop_propagation();
                                            }))
                                    })
                                    .into_any_element();

                                    let action = div()
                                        .id(("keymap action", index))
                                        .child(command_palette::humanize_action_name(&action_name))
                                        .when(!context_menu_deployed, |this| {
                                            this.tooltip({
                                                let action_name = binding.action_name.clone();
                                                let action_docs = binding.action_docs;
                                                move |_, cx| {
                                                    let action_tooltip = Tooltip::new(&action_name);
                                                    let action_tooltip = match action_docs {
                                                        Some(docs) => action_tooltip.meta(docs),
                                                        None => action_tooltip,
                                                    };
                                                    cx.new(|_| action_tooltip).into()
                                                }
                                            })
                                        })
                                        .into_any_element();
                                    let keystrokes = binding.ui_key_binding.clone().map_or(
                                        binding.keystroke_text.clone().into_any_element(),
                                        IntoElement::into_any_element,
                                    );
                                    let action_arguments = match binding.action_arguments.clone() {
                                        Some(arguments) => arguments.into_any_element(),
                                        None => {
                                            if binding.action_schema.is_some() {
                                                muted_styled_text(NO_ACTION_ARGUMENTS_TEXT, cx)
                                                    .into_any_element()
                                            } else {
                                                gpui::Empty.into_any_element()
                                            }
                                        }
                                    };
                                    let context = binding.context.clone().map_or(
                                        gpui::Empty.into_any_element(),
                                        |context| {
                                            let is_local = context.local().is_some();

                                            div()
                                                .id(("keymap context", index))
                                                .child(context.clone())
                                                .when(is_local && !context_menu_deployed, |this| {
                                                    this.tooltip(Tooltip::element({
                                                        move |_, _| {
                                                            context.clone().into_any_element()
                                                        }
                                                    }))
                                                })
                                                .into_any_element()
                                        },
                                    );
                                    let source = binding
                                        .source
                                        .clone()
                                        .map(|(_source, name)| name)
                                        .unwrap_or_default()
                                        .into_any_element();
                                    Some([
                                        icon,
                                        action,
                                        action_arguments,
                                        keystrokes,
                                        context,
                                        source,
                                    ])
                                })
                                .collect()
                        }),
                    )
                    .map_row(
                        cx.processor(|this, (row_index, row): (usize, Div), _window, cx| {
                            let is_conflict = this.has_conflict(row_index);
                            let is_selected = this.selected_index == Some(row_index);

                            let row_id = row_group_id(row_index);

                            let row = row
                                .id(row_id.clone())
                                .on_any_mouse_down(cx.listener(
                                    move |this,
                                          mouse_down_event: &gpui::MouseDownEvent,
                                          window,
                                          cx| {
                                        match mouse_down_event.button {
                                            MouseButton::Right => {
                                                this.select_index(row_index, cx);
                                                this.create_context_menu(
                                                    mouse_down_event.position,
                                                    window,
                                                    cx,
                                                );
                                            }
                                            _ => {}
                                        }
                                    },
                                ))
                                .on_click(cx.listener(
                                    move |this, event: &ClickEvent, window, cx| {
                                        this.select_index(row_index, cx);
                                        if event.up.click_count == 2 {
                                            this.open_edit_keybinding_modal(false, window, cx);
                                        }
                                    },
                                ))
                                .group(row_id)
                                .border_2()
                                .when(is_conflict, |row| {
                                    row.bg(cx.theme().status().error_background)
                                })
                                .when(is_selected, |row| {
                                    row.border_color(cx.theme().colors().panel_focused_border)
                                });

                            row.into_any_element()
                        }),
                    ),
            )
            .on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, _, cx| {
                // This ensures that the menu is not dismissed in cases where scroll events
                // with a delta of zero are emitted
                if !event.delta.pixel_delta(px(1.)).y.is_zero() {
                    this.context_menu.take();
                    cx.notify();
                }
            }))
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

fn row_group_id(row_index: usize) -> SharedString {
    SharedString::new(format!("keymap-table-row-{}", row_index))
}

fn base_button_style(row_index: usize, icon: IconName) -> IconButton {
    IconButton::new(("keymap-icon", row_index), icon)
        .shape(IconButtonShape::Square)
        .size(ButtonSize::Compact)
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

#[derive(PartialEq)]
enum InputError {
    Warning(SharedString),
    Error(SharedString),
}

impl InputError {
    fn warning(message: impl Into<SharedString>) -> Self {
        Self::Warning(message.into())
    }

    fn error(message: impl Into<SharedString>) -> Self {
        Self::Error(message.into())
    }

    fn content(&self) -> &SharedString {
        match self {
            InputError::Warning(content) | InputError::Error(content) => content,
        }
    }

    fn is_warning(&self) -> bool {
        matches!(self, InputError::Warning(_))
    }
}

struct KeybindingEditorModal {
    creating: bool,
    editing_keybind: ProcessedKeybinding,
    editing_keybind_idx: usize,
    keybind_editor: Entity<KeystrokeInput>,
    context_editor: Entity<SingleLineInput>,
    action_arguments_editor: Option<Entity<Editor>>,
    fs: Arc<dyn Fs>,
    error: Option<InputError>,
    keymap_editor: Entity<KeymapEditor>,
    workspace: WeakEntity<Workspace>,
    focus_state: KeybindingEditorModalFocusState,
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
        create: bool,
        editing_keybind: ProcessedKeybinding,
        editing_keybind_idx: usize,
        keymap_editor: Entity<KeymapEditor>,
        workspace: WeakEntity<Workspace>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let keybind_editor = cx
            .new(|cx| KeystrokeInput::new(editing_keybind.keystrokes().map(Vec::from), window, cx));

        let context_editor: Entity<SingleLineInput> = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "Keybinding Context")
                .label("Edit Context")
                .label_size(LabelSize::Default);

            if let Some(context) = editing_keybind
                .context
                .as_ref()
                .and_then(KeybindContextString::local)
            {
                input.editor().update(cx, |editor, cx| {
                    editor.set_text(context.clone(), window, cx);
                });
            }

            let editor_entity = input.editor().clone();
            cx.spawn(async move |_input_handle, cx| {
                let contexts = cx
                    .background_spawn(async { collect_contexts_from_assets() })
                    .await;

                editor_entity
                    .update(cx, |editor, _cx| {
                        editor.set_completion_provider(Some(std::rc::Rc::new(
                            KeyContextCompletionProvider { contexts },
                        )));
                    })
                    .context("Failed to load completions for keybinding context")
            })
            .detach_and_log_err(cx);

            input
        });

        let action_arguments_editor = editing_keybind.action_schema.clone().map(|_schema| {
            cx.new(|cx| {
                let mut editor = Editor::auto_height_unbounded(1, window, cx);
                let workspace = workspace.clone();

                if let Some(arguments) = editing_keybind.action_arguments.clone() {
                    editor.set_text(arguments.text, window, cx);
                } else {
                    // TODO: default value from schema?
                    editor.set_placeholder_text("Action Arguments", cx);
                }
                cx.spawn(async |editor, cx| {
                    let json_language = load_json_language(workspace, cx).await;
                    editor
                        .update(cx, |editor, cx| {
                            if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                                buffer.update(cx, |buffer, cx| {
                                    buffer.set_language(Some(json_language), cx)
                                });
                            }
                        })
                        .context("Failed to load JSON language for editing keybinding action arguments input")
                })
                .detach_and_log_err(cx);
                editor
            })
        });

        let focus_state = KeybindingEditorModalFocusState::new(
            keybind_editor.read_with(cx, |keybind_editor, cx| keybind_editor.focus_handle(cx)),
            action_arguments_editor.as_ref().map(|args_editor| {
                args_editor.read_with(cx, |args_editor, cx| args_editor.focus_handle(cx))
            }),
            context_editor.read_with(cx, |context_editor, cx| context_editor.focus_handle(cx)),
        );

        Self {
            creating: create,
            editing_keybind,
            editing_keybind_idx,
            fs,
            keybind_editor,
            context_editor,
            action_arguments_editor,
            error: None,
            keymap_editor,
            workspace,
            focus_state,
        }
    }

    fn set_error(&mut self, error: InputError, cx: &mut Context<Self>) -> bool {
        if self
            .error
            .as_ref()
            .is_some_and(|old_error| old_error.is_warning() && *old_error == error)
        {
            false
        } else {
            self.error = Some(error);
            cx.notify();
            true
        }
    }

    fn validate_action_arguments(&self, cx: &App) -> anyhow::Result<Option<String>> {
        let action_arguments = self
            .action_arguments_editor
            .as_ref()
            .map(|editor| editor.read(cx).text(cx));

        let value = action_arguments
            .as_ref()
            .map(|args| {
                serde_json::from_str(args).context("Failed to parse action arguments as JSON")
            })
            .transpose()?;

        cx.build_action(&self.editing_keybind.action_name, value)
            .context("Failed to validate action arguments")?;
        Ok(action_arguments)
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        let existing_keybind = self.editing_keybind.clone();
        let fs = self.fs.clone();
        let new_keystrokes = self
            .keybind_editor
            .read_with(cx, |editor, _| editor.keystrokes().to_vec());
        if new_keystrokes.is_empty() {
            self.set_error(InputError::error("Keystrokes cannot be empty"), cx);
            return;
        }
        let tab_size = cx.global::<settings::SettingsStore>().json_tab_size();
        let new_context = self
            .context_editor
            .read_with(cx, |input, cx| input.editor().read(cx).text(cx));
        let new_context = new_context.is_empty().not().then_some(new_context);
        let new_context_err = new_context.as_deref().and_then(|context| {
            gpui::KeyBindingContextPredicate::parse(context)
                .context("Failed to parse key context")
                .err()
        });
        if let Some(err) = new_context_err {
            // TODO: store and display as separate error
            // TODO: also, should be validating on keystroke
            self.set_error(InputError::error(err.to_string()), cx);
            return;
        }

        let new_action_args = match self.validate_action_arguments(cx) {
            Err(input_err) => {
                self.set_error(InputError::error(input_err.to_string()), cx);
                return;
            }
            Ok(input) => input,
        };

        let action_mapping: ActionMapping = (
            ui::text_for_keystrokes(&new_keystrokes, cx).into(),
            new_context
                .as_ref()
                .map(Into::into)
                .or_else(|| existing_keybind.get_action_mapping().1),
        );

        if let Some(conflicting_indices) = self
            .keymap_editor
            .read(cx)
            .keybinding_conflict_state
            .conflicting_indices_for_mapping(action_mapping, self.editing_keybind_idx)
        {
            let first_conflicting_index = conflicting_indices[0];
            let conflicting_action_name = self
                .keymap_editor
                .read(cx)
                .keybindings
                .get(first_conflicting_index)
                .map(|keybind| keybind.action_name.clone());

            let warning_message = match conflicting_action_name {
                Some(name) => {
                    let confliction_action_amount = conflicting_indices.len() - 1;
                    if confliction_action_amount > 0 {
                        format!(
                            "Your keybind would conflict with the \"{}\" action and {} other bindings",
                            name, confliction_action_amount
                        )
                    } else {
                        format!("Your keybind would conflict with the \"{}\" action", name)
                    }
                }
                None => {
                    log::info!(
                        "Could not find action in keybindings with index {}",
                        first_conflicting_index
                    );
                    "Your keybind would conflict with other actions".to_string()
                }
            };

            if self.set_error(InputError::warning(warning_message), cx) {
                return;
            }
        }

        let create = self.creating;

        let status_toast = StatusToast::new(
            format!(
                "Saved edits to the {} action.",
                command_palette::humanize_action_name(&self.editing_keybind.action_name)
            ),
            cx,
            move |this, _cx| {
                this.icon(ToastIcon::new(IconName::Check).color(Color::Success))
                    .dismiss_button(true)
                // .action("Undo", f) todo: wire the undo functionality
            },
        );

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_status_toast(status_toast, cx);
            })
            .log_err();

        cx.spawn(async move |this, cx| {
            let action_name = existing_keybind.action_name.clone();

            if let Err(err) = save_keybinding_update(
                create,
                existing_keybind,
                &new_keystrokes,
                new_context.as_deref(),
                new_action_args.as_deref(),
                &fs,
                tab_size,
            )
            .await
            {
                this.update(cx, |this, cx| {
                    this.set_error(InputError::error(err.to_string()), cx);
                })
                .log_err();
            } else {
                this.update(cx, |this, cx| {
                    let action_mapping = (
                        ui::text_for_keystrokes(new_keystrokes.as_slice(), cx).into(),
                        new_context.map(SharedString::from),
                    );

                    this.keymap_editor.update(cx, |keymap, cx| {
                        keymap.previous_edit = Some(PreviousEdit::Keybinding {
                            action_mapping,
                            action_name,
                            fallback: keymap
                                .table_interaction_state
                                .read(cx)
                                .get_scrollbar_offset(Axis::Vertical),
                        })
                    });
                    cx.emit(DismissEvent);
                })
                .ok();
            }
        })
        .detach();
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("KeybindEditorModal");
        key_context
    }

    fn focus_next(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        self.focus_state.focus_next(window, cx);
    }

    fn focus_prev(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_state.focus_previous(window, cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        self.save(cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }
}

impl Render for KeybindingEditorModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().colors();
        let action_name =
            command_palette::humanize_action_name(&self.editing_keybind.action_name).to_string();

        v_flex()
            .w(rems(34.))
            .elevation_3(cx)
            .key_context(self.key_context())
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus_prev))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .child(
                Modal::new("keybinding_editor_modal", None)
                    .header(
                        ModalHeader::new().child(
                            v_flex()
                                .pb_1p5()
                                .mb_1()
                                .gap_0p5()
                                .border_b_1()
                                .border_color(theme.border_variant)
                                .child(Label::new(action_name))
                                .when_some(self.editing_keybind.action_docs, |this, docs| {
                                    this.child(
                                        Label::new(docs).size(LabelSize::Small).color(Color::Muted),
                                    )
                                }),
                        ),
                    )
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_2()
                                .child(
                                    v_flex()
                                        .child(Label::new("Edit Keystroke"))
                                        .gap_1()
                                        .child(self.keybind_editor.clone()),
                                )
                                .when_some(self.action_arguments_editor.clone(), |this, editor| {
                                    this.child(
                                        v_flex()
                                            .mt_1p5()
                                            .gap_1()
                                            .child(Label::new("Edit Arguments"))
                                            .child(
                                                div()
                                                    .w_full()
                                                    .py_1()
                                                    .px_1p5()
                                                    .rounded_lg()
                                                    .bg(theme.editor_background)
                                                    .border_1()
                                                    .border_color(theme.border_variant)
                                                    .child(editor),
                                            ),
                                    )
                                })
                                .child(self.context_editor.clone())
                                .when_some(self.error.as_ref(), |this, error| {
                                    this.child(
                                        Banner::new()
                                            .map(|banner| match error {
                                                InputError::Error(_) => {
                                                    banner.severity(ui::Severity::Error)
                                                }
                                                InputError::Warning(_) => {
                                                    banner.severity(ui::Severity::Warning)
                                                }
                                            })
                                            // For some reason, the div overflows its container to the
                                            //right. The padding accounts for that.
                                            .child(
                                                div()
                                                    .size_full()
                                                    .pr_2()
                                                    .child(Label::new(error.content())),
                                            ),
                                    )
                                }),
                        ),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("cancel", "Cancel")
                                        .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                                )
                                .child(Button::new("save-btn", "Save").on_click(cx.listener(
                                    |this, _event, _window, cx| {
                                        this.save(cx);
                                    },
                                ))),
                        ),
                    ),
            )
    }
}

struct KeybindingEditorModalFocusState {
    handles: Vec<FocusHandle>,
}

impl KeybindingEditorModalFocusState {
    fn new(
        keystrokes: FocusHandle,
        action_input: Option<FocusHandle>,
        context: FocusHandle,
    ) -> Self {
        Self {
            handles: Vec::from_iter(
                [Some(keystrokes), action_input, Some(context)]
                    .into_iter()
                    .flatten(),
            ),
        }
    }

    fn focused_index(&self, window: &Window, cx: &App) -> Option<i32> {
        self.handles
            .iter()
            .position(|handle| handle.contains_focused(window, cx))
            .map(|i| i as i32)
    }

    fn focus_index(&self, mut index: i32, window: &mut Window) {
        if index < 0 {
            index = self.handles.len() as i32 - 1;
        }
        if index >= self.handles.len() as i32 {
            index = 0;
        }
        window.focus(&self.handles[index as usize]);
    }

    fn focus_next(&self, window: &mut Window, cx: &App) {
        let index_to_focus = if let Some(index) = self.focused_index(window, cx) {
            index + 1
        } else {
            0
        };
        self.focus_index(index_to_focus, window);
    }

    fn focus_previous(&self, window: &mut Window, cx: &App) {
        let index_to_focus = if let Some(index) = self.focused_index(window, cx) {
            index - 1
        } else {
            self.handles.len() as i32 - 1
        };
        self.focus_index(index_to_focus, window);
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

async fn load_json_language(workspace: WeakEntity<Workspace>, cx: &mut AsyncApp) -> Arc<Language> {
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

async fn load_rust_language(workspace: WeakEntity<Workspace>, cx: &mut AsyncApp) -> Arc<Language> {
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

async fn save_keybinding_update(
    create: bool,
    existing: ProcessedKeybinding,
    new_keystrokes: &[Keystroke],
    new_context: Option<&str>,
    new_args: Option<&str>,
    fs: &Arc<dyn Fs>,
    tab_size: usize,
) -> anyhow::Result<()> {
    let keymap_contents = settings::KeymapFile::load_keymap_file(fs)
        .await
        .context("Failed to load keymap file")?;

    let operation = if !create {
        let existing_keystrokes = existing.keystrokes().unwrap_or_default();
        let existing_context = existing
            .context
            .as_ref()
            .and_then(KeybindContextString::local_str);
        let existing_args = existing
            .action_arguments
            .as_ref()
            .map(|args| args.text.as_ref());

        settings::KeybindUpdateOperation::Replace {
            target: settings::KeybindUpdateTarget {
                context: existing_context,
                keystrokes: existing_keystrokes,
                action_name: &existing.action_name,
                use_key_equivalents: false,
                action_arguments: existing_args,
            },
            target_keybind_source: existing
                .source
                .as_ref()
                .map(|(source, _name)| *source)
                .unwrap_or(KeybindSource::User),
            source: settings::KeybindUpdateTarget {
                context: new_context,
                keystrokes: new_keystrokes,
                action_name: &existing.action_name,
                use_key_equivalents: false,
                action_arguments: new_args,
            },
        }
    } else {
        settings::KeybindUpdateOperation::Add(settings::KeybindUpdateTarget {
            context: new_context,
            keystrokes: new_keystrokes,
            action_name: &existing.action_name,
            use_key_equivalents: false,
            action_arguments: new_args,
        })
    };
    let updated_keymap_contents =
        settings::KeymapFile::update_keybinding(operation, keymap_contents, tab_size)
            .context("Failed to update keybinding")?;
    fs.write(
        paths::keymap_file().as_path(),
        updated_keymap_contents.as_bytes(),
    )
    .await
    .context("Failed to write keymap file")?;
    Ok(())
}

async fn remove_keybinding(
    existing: ProcessedKeybinding,
    fs: &Arc<dyn Fs>,
    tab_size: usize,
) -> anyhow::Result<()> {
    let Some(keystrokes) = existing.keystrokes() else {
        anyhow::bail!("Cannot remove a keybinding that does not exist");
    };
    let keymap_contents = settings::KeymapFile::load_keymap_file(fs)
        .await
        .context("Failed to load keymap file")?;

    let operation = settings::KeybindUpdateOperation::Remove {
        target: settings::KeybindUpdateTarget {
            context: existing
                .context
                .as_ref()
                .and_then(KeybindContextString::local_str),
            keystrokes,
            action_name: &existing.action_name,
            use_key_equivalents: false,
            action_arguments: existing
                .action_arguments
                .as_ref()
                .map(|arguments| arguments.text.as_ref()),
        },
        target_keybind_source: existing
            .source
            .as_ref()
            .map(|(source, _name)| *source)
            .unwrap_or(KeybindSource::User),
    };

    let updated_keymap_contents =
        settings::KeymapFile::update_keybinding(operation, keymap_contents, tab_size)
            .context("Failed to update keybinding")?;
    fs.write(
        paths::keymap_file().as_path(),
        updated_keymap_contents.as_bytes(),
    )
    .await
    .context("Failed to write keymap file")?;
    Ok(())
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum CloseKeystrokeResult {
    Partial,
    Close,
    None,
}

struct KeystrokeInput {
    keystrokes: Vec<Keystroke>,
    placeholder_keystrokes: Option<Vec<Keystroke>>,
    highlight_on_focus: bool,
    outer_focus_handle: FocusHandle,
    inner_focus_handle: FocusHandle,
    intercept_subscription: Option<Subscription>,
    _focus_subscriptions: [Subscription; 2],
    search: bool,
    close_keystrokes: Option<Vec<Keystroke>>,
    close_keystrokes_start: Option<usize>,
}

impl KeystrokeInput {
    const KEYSTROKE_COUNT_MAX: usize = 3;

    fn new(
        placeholder_keystrokes: Option<Vec<Keystroke>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let outer_focus_handle = cx.focus_handle();
        let inner_focus_handle = cx.focus_handle();
        let _focus_subscriptions = [
            cx.on_focus_in(&inner_focus_handle, window, Self::on_inner_focus_in),
            cx.on_focus_out(&inner_focus_handle, window, Self::on_inner_focus_out),
        ];
        Self {
            keystrokes: Vec::new(),
            placeholder_keystrokes,
            highlight_on_focus: true,
            inner_focus_handle,
            outer_focus_handle,
            intercept_subscription: None,
            _focus_subscriptions,
            search: false,
            close_keystrokes: None,
            close_keystrokes_start: None,
        }
    }

    fn dummy(modifiers: Modifiers) -> Keystroke {
        return Keystroke {
            modifiers,
            key: "".to_string(),
            key_char: None,
        };
    }

    fn keystrokes_changed(&self, cx: &mut Context<Self>) {
        cx.emit(());
        cx.notify();
    }

    fn key_context() -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("KeystrokeInput");
        key_context
    }

    fn handle_possible_close_keystroke(
        &mut self,
        keystroke: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> CloseKeystrokeResult {
        let Some(keybind_for_close_action) = window
            .highest_precedence_binding_for_action_in_context(&StopRecording, Self::key_context())
        else {
            log::trace!("No keybinding to stop recording keystrokes in keystroke input");
            self.close_keystrokes.take();
            return CloseKeystrokeResult::None;
        };
        let action_keystrokes = keybind_for_close_action.keystrokes();

        if let Some(mut close_keystrokes) = self.close_keystrokes.take() {
            let mut index = 0;

            while index < action_keystrokes.len() && index < close_keystrokes.len() {
                if !close_keystrokes[index].should_match(&action_keystrokes[index]) {
                    break;
                }
                index += 1;
            }
            if index == close_keystrokes.len() {
                if index >= action_keystrokes.len() {
                    self.close_keystrokes_start.take();
                    return CloseKeystrokeResult::None;
                }
                if keystroke.should_match(&action_keystrokes[index]) {
                    if action_keystrokes.len() >= 1 && index == action_keystrokes.len() - 1 {
                        self.stop_recording(&StopRecording, window, cx);
                        return CloseKeystrokeResult::Close;
                    } else {
                        close_keystrokes.push(keystroke.clone());
                        self.close_keystrokes = Some(close_keystrokes);
                        return CloseKeystrokeResult::Partial;
                    }
                } else {
                    self.close_keystrokes_start.take();
                    return CloseKeystrokeResult::None;
                }
            }
        } else if let Some(first_action_keystroke) = action_keystrokes.first()
            && keystroke.should_match(first_action_keystroke)
        {
            self.close_keystrokes = Some(vec![keystroke.clone()]);
            return CloseKeystrokeResult::Partial;
        }
        self.close_keystrokes_start.take();
        return CloseKeystrokeResult::None;
    }

    fn on_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keystrokes_len = self.keystrokes.len();

        if let Some(last) = self.keystrokes.last_mut()
            && last.key.is_empty()
            && keystrokes_len <= Self::KEYSTROKE_COUNT_MAX
        {
            if !event.modifiers.modified() {
                self.keystrokes.pop();
            } else {
                last.modifiers = event.modifiers;
            }
            self.keystrokes_changed(cx);
        } else if keystrokes_len < Self::KEYSTROKE_COUNT_MAX {
            self.keystrokes.push(Self::dummy(event.modifiers));
            self.keystrokes_changed(cx);
        }
        cx.stop_propagation();
    }

    fn handle_keystroke(
        &mut self,
        keystroke: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let close_keystroke_result = self.handle_possible_close_keystroke(keystroke, window, cx);
        if close_keystroke_result != CloseKeystrokeResult::Close {
            if let Some(last) = self.keystrokes.last()
                && last.key.is_empty()
                && self.keystrokes.len() <= Self::KEYSTROKE_COUNT_MAX
            {
                self.keystrokes.pop();
            }
            if self.keystrokes.len() < Self::KEYSTROKE_COUNT_MAX {
                if close_keystroke_result == CloseKeystrokeResult::Partial
                    && self.close_keystrokes_start.is_none()
                {
                    self.close_keystrokes_start = Some(self.keystrokes.len());
                }
                self.keystrokes.push(keystroke.clone());
                if self.keystrokes.len() < Self::KEYSTROKE_COUNT_MAX {
                    self.keystrokes.push(Self::dummy(keystroke.modifiers));
                }
            }
        }
        self.keystrokes_changed(cx);
        cx.stop_propagation();
    }

    fn on_inner_focus_in(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.intercept_subscription.is_none() {
            let listener = cx.listener(|this, event: &gpui::KeystrokeEvent, window, cx| {
                this.handle_keystroke(&event.keystroke, window, cx);
            });
            self.intercept_subscription = Some(cx.intercept_keystrokes(listener))
        }
    }

    fn on_inner_focus_out(
        &mut self,
        _event: gpui::FocusOutEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.intercept_subscription.take();
        cx.notify();
    }

    fn keystrokes(&self) -> &[Keystroke] {
        if let Some(placeholders) = self.placeholder_keystrokes.as_ref()
            && self.keystrokes.is_empty()
        {
            return placeholders;
        }
        if self
            .keystrokes
            .last()
            .map_or(false, |last| last.key.is_empty())
        {
            return &self.keystrokes[..self.keystrokes.len() - 1];
        }
        return &self.keystrokes;
    }

    fn render_keystrokes(&self, is_recording: bool) -> impl Iterator<Item = Div> {
        let keystrokes = if let Some(placeholders) = self.placeholder_keystrokes.as_ref()
            && self.keystrokes.is_empty()
        {
            if is_recording {
                &[]
            } else {
                placeholders.as_slice()
            }
        } else {
            &self.keystrokes
        };
        keystrokes.iter().map(move |keystroke| {
            h_flex().children(ui::render_keystroke(
                keystroke,
                Some(Color::Default),
                Some(rems(0.875).into()),
                ui::PlatformStyle::platform(),
                false,
            ))
        })
    }

    fn recording_focus_handle(&self, _cx: &App) -> FocusHandle {
        self.inner_focus_handle.clone()
    }

    fn set_search_mode(&mut self, search: bool) {
        self.search = search;
    }

    fn start_recording(&mut self, _: &StartRecording, window: &mut Window, cx: &mut Context<Self>) {
        if !self.outer_focus_handle.is_focused(window) {
            return;
        }
        self.clear_keystrokes(&ClearKeystrokes, window, cx);
        window.focus(&self.inner_focus_handle);
        cx.notify();
    }

    fn stop_recording(&mut self, _: &StopRecording, window: &mut Window, cx: &mut Context<Self>) {
        if !self.inner_focus_handle.is_focused(window) {
            return;
        }
        window.focus(&self.outer_focus_handle);
        if let Some(close_keystrokes_start) = self.close_keystrokes_start.take() {
            self.keystrokes.drain(close_keystrokes_start..);
        }
        self.close_keystrokes.take();
        cx.notify();
    }

    fn clear_keystrokes(
        &mut self,
        _: &ClearKeystrokes,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.outer_focus_handle.is_focused(window) {
            return;
        }
        self.keystrokes.clear();
        cx.notify();
    }
}

impl EventEmitter<()> for KeystrokeInput {}

impl Focusable for KeystrokeInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.outer_focus_handle.clone()
    }
}

impl Render for KeystrokeInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let is_focused = self.outer_focus_handle.contains_focused(window, cx);
        let is_recording = self.inner_focus_handle.is_focused(window);

        let horizontal_padding = rems_from_px(64.);

        let recording_bg_color = colors
            .editor_background
            .blend(colors.text_accent.opacity(0.1));

        let recording_pulse = || {
            Icon::new(IconName::Circle)
                .size(IconSize::Small)
                .color(Color::Error)
                .with_animation(
                    "recording-pulse",
                    Animation::new(std::time::Duration::from_secs(2))
                        .repeat()
                        .with_easing(gpui::pulsating_between(0.4, 0.8)),
                    {
                        let color = Color::Error.color(cx);
                        move |this, delta| this.color(Color::Custom(color.opacity(delta)))
                    },
                )
        };

        let recording_indicator = h_flex()
            .h_4()
            .pr_1()
            .gap_0p5()
            .border_1()
            .border_color(colors.border)
            .bg(colors
                .editor_background
                .blend(colors.text_accent.opacity(0.1)))
            .rounded_sm()
            .child(recording_pulse())
            .child(
                Label::new("REC")
                    .size(LabelSize::XSmall)
                    .weight(FontWeight::SEMIBOLD)
                    .color(Color::Error),
            );

        let search_indicator = h_flex()
            .h_4()
            .pr_1()
            .gap_0p5()
            .border_1()
            .border_color(colors.border)
            .bg(colors
                .editor_background
                .blend(colors.text_accent.opacity(0.1)))
            .rounded_sm()
            .child(recording_pulse())
            .child(
                Label::new("SEARCH")
                    .size(LabelSize::XSmall)
                    .weight(FontWeight::SEMIBOLD)
                    .color(Color::Accent),
            );

        let record_icon = if self.search {
            IconName::MagnifyingGlass
        } else {
            IconName::PlayFilled
        };

        return h_flex()
            .id("keystroke-input")
            .track_focus(&self.outer_focus_handle)
            .py_2()
            .px_3()
            .gap_2()
            .min_h_10()
            .w_full()
            .flex_1()
            .justify_between()
            .rounded_lg()
            .overflow_hidden()
            .map(|this| {
                if is_recording {
                    this.bg(recording_bg_color)
                } else {
                    this.bg(colors.editor_background)
                }
            })
            .border_1()
            .border_color(colors.border_variant)
            .when(is_focused, |parent| {
                parent.border_color(colors.border_focused)
            })
            .key_context(Self::key_context())
            .on_action(cx.listener(Self::start_recording))
            .on_action(cx.listener(Self::stop_recording))
            .child(
                h_flex()
                    .w(horizontal_padding)
                    .gap_0p5()
                    .justify_start()
                    .flex_none()
                    .when(is_recording, |this| {
                        this.map(|this| {
                            if self.search {
                                this.child(search_indicator)
                            } else {
                                this.child(recording_indicator)
                            }
                        })
                    }),
            )
            .child(
                h_flex()
                    .id("keystroke-input-inner")
                    .track_focus(&self.inner_focus_handle)
                    .on_modifiers_changed(cx.listener(Self::on_modifiers_changed))
                    .size_full()
                    .when(self.highlight_on_focus, |this| {
                        this.focus(|mut style| {
                            style.border_color = Some(colors.border_focused);
                            style
                        })
                    })
                    .w_full()
                    .min_w_0()
                    .justify_center()
                    .flex_wrap()
                    .gap(ui::DynamicSpacing::Base04.rems(cx))
                    .children(self.render_keystrokes(is_recording)),
            )
            .child(
                h_flex()
                    .w(horizontal_padding)
                    .gap_0p5()
                    .justify_end()
                    .flex_none()
                    .map(|this| {
                        if is_recording {
                            this.child(
                                IconButton::new("stop-record-btn", IconName::StopFilled)
                                    .shape(ui::IconButtonShape::Square)
                                    .map(|this| {
                                        this.tooltip(Tooltip::for_action_title(
                                            if self.search {
                                                "Stop Searching"
                                            } else {
                                                "Stop Recording"
                                            },
                                            &StopRecording,
                                        ))
                                    })
                                    .icon_color(Color::Error)
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        this.stop_recording(&StopRecording, window, cx);
                                    })),
                            )
                        } else {
                            this.child(
                                IconButton::new("record-btn", record_icon)
                                    .shape(ui::IconButtonShape::Square)
                                    .map(|this| {
                                        this.tooltip(Tooltip::for_action_title(
                                            if self.search {
                                                "Start Searching"
                                            } else {
                                                "Start Recording"
                                            },
                                            &StartRecording,
                                        ))
                                    })
                                    .when(!is_focused, |this| this.icon_color(Color::Muted))
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        this.start_recording(&StartRecording, window, cx);
                                    })),
                            )
                        }
                    })
                    .child(
                        IconButton::new("clear-btn", IconName::Delete)
                            .shape(ui::IconButtonShape::Square)
                            .tooltip(Tooltip::for_action_title(
                                "Clear Keystrokes",
                                &ClearKeystrokes,
                            ))
                            .when(!is_recording || !is_focused, |this| {
                                this.icon_color(Color::Muted)
                            })
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.clear_keystrokes(&ClearKeystrokes, window, cx);
                            })),
                    ),
            );
    }
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
