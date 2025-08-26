use std::{
    cmp::{self},
    ops::{Not as _, Range},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, anyhow};
use collections::{HashMap, HashSet};
use editor::{CompletionProvider, Editor, EditorEvent};
use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, AppContext as _, AsyncApp, Axis, ClickEvent, Context, DismissEvent, Entity,
    EventEmitter, FocusHandle, Focusable, Global, IsZero,
    KeyBindingContextPredicate::{And, Descendant, Equal, Identifier, Not, NotEqual, Or},
    KeyContext, KeybindingKeystroke, Keystroke, MouseButton, PlatformKeyboardMapper, Point,
    ScrollStrategy, ScrollWheelEvent, Stateful, StyledText, Subscription, Task,
    TextStyleRefinement, WeakEntity, actions, anchored, deferred, div,
};
use language::{Language, LanguageConfig, ToOffset as _};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::Project;
use settings::{BaseKeymap, KeybindSource, KeymapFile, Settings as _, SettingsAssets};
use ui::{
    ActiveTheme as _, App, Banner, BorrowAppContext, ContextMenu, IconButtonShape, Indicator,
    Modal, ModalFooter, ModalHeader, ParentElement as _, Render, Section, SharedString,
    Styled as _, Tooltip, Window, prelude::*, right_click_menu,
};
use ui_input::SingleLineInput;
use util::ResultExt;
use workspace::{
    Item, ModalView, SerializableItem, Workspace, notifications::NotifyTaskExt as _,
    register_serializable_item,
};

use crate::{
    keybindings::persistence::KEYBINDING_EDITORS,
    ui_components::{
        keystroke_input::{ClearKeystrokes, KeystrokeInput, StartRecording, StopRecording},
        table::{ColumnWidths, ResizeBehavior, Table, TableInteractionState},
    },
};

const NO_ACTION_ARGUMENTS_TEXT: SharedString = SharedString::new_static("<no arguments>");

actions!(
    zed,
    [
        /// Opens the keymap editor.
        OpenKeymapEditor
    ]
);

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
        /// Toggles exact matching for keystroke search
        ToggleExactKeystrokeMatching,
        /// Shows matching keystrokes for the currently selected binding
        ShowMatchingKeybinds
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
    KeyStroke {
        exact_match: bool,
    },
}

impl SearchMode {
    fn invert(&self) -> Self {
        match self {
            SearchMode::Normal => SearchMode::KeyStroke { exact_match: false },
            SearchMode::KeyStroke { .. } => SearchMode::Normal,
        }
    }

    fn exact_match(&self) -> bool {
        match self {
            SearchMode::Normal => false,
            SearchMode::KeyStroke { exact_match } => *exact_match,
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

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
struct ActionMapping {
    keystrokes: Vec<KeybindingKeystroke>,
    context: Option<SharedString>,
}

#[derive(Debug)]
struct KeybindConflict {
    first_conflict_index: usize,
    remaining_conflict_amount: usize,
}

#[derive(Clone, Copy, PartialEq)]
struct ConflictOrigin {
    override_source: KeybindSource,
    overridden_source: Option<KeybindSource>,
    index: usize,
}

impl ConflictOrigin {
    fn new(source: KeybindSource, index: usize) -> Self {
        Self {
            override_source: source,
            index,
            overridden_source: None,
        }
    }

    fn with_overridden_source(self, source: KeybindSource) -> Self {
        Self {
            overridden_source: Some(source),
            ..self
        }
    }

    fn get_conflict_with(&self, other: &Self) -> Option<Self> {
        if self.override_source == KeybindSource::User
            && other.override_source == KeybindSource::User
        {
            Some(
                Self::new(KeybindSource::User, other.index)
                    .with_overridden_source(self.override_source),
            )
        } else if self.override_source > other.override_source {
            Some(other.with_overridden_source(self.override_source))
        } else {
            None
        }
    }

    fn is_user_keybind_conflict(&self) -> bool {
        self.override_source == KeybindSource::User
            && self.overridden_source == Some(KeybindSource::User)
    }
}

#[derive(Default)]
struct ConflictState {
    conflicts: Vec<Option<ConflictOrigin>>,
    keybind_mapping: ConflictKeybindMapping,
    has_user_conflicts: bool,
}

type ConflictKeybindMapping = HashMap<
    Vec<KeybindingKeystroke>,
    Vec<(
        Option<gpui::KeyBindingContextPredicate>,
        Vec<ConflictOrigin>,
    )>,
>;

impl ConflictState {
    fn new(key_bindings: &[ProcessedBinding]) -> Self {
        let mut action_keybind_mapping = ConflictKeybindMapping::default();

        let mut largest_index = 0;
        for (index, binding) in key_bindings
            .iter()
            .enumerate()
            .flat_map(|(index, binding)| Some(index).zip(binding.keybind_information()))
        {
            let mapping = binding.get_action_mapping();
            let predicate = mapping
                .context
                .and_then(|ctx| gpui::KeyBindingContextPredicate::parse(&ctx).ok());
            let entry = action_keybind_mapping
                .entry(mapping.keystrokes)
                .or_default();
            let origin = ConflictOrigin::new(binding.source, index);
            if let Some((_, origins)) =
                entry
                    .iter_mut()
                    .find(|(other_predicate, _)| match (&predicate, other_predicate) {
                        (None, None) => true,
                        (Some(a), Some(b)) => normalized_ctx_eq(a, b),
                        _ => false,
                    })
            {
                origins.push(origin);
            } else {
                entry.push((predicate, vec![origin]));
            }
            largest_index = index;
        }

        let mut conflicts = vec![None; largest_index + 1];
        let mut has_user_conflicts = false;

        for entries in action_keybind_mapping.values_mut() {
            for (_, indices) in entries.iter_mut() {
                indices.sort_unstable_by_key(|origin| origin.override_source);
                let Some((fst, snd)) = indices.get(0).zip(indices.get(1)) else {
                    continue;
                };

                for origin in indices.iter() {
                    conflicts[origin.index] =
                        origin.get_conflict_with(if origin == fst { snd } else { fst })
                }

                has_user_conflicts |= fst.override_source == KeybindSource::User
                    && snd.override_source == KeybindSource::User;
            }
        }

        Self {
            conflicts,
            keybind_mapping: action_keybind_mapping,
            has_user_conflicts,
        }
    }

    fn conflicting_indices_for_mapping(
        &self,
        action_mapping: &ActionMapping,
        keybind_idx: Option<usize>,
    ) -> Option<KeybindConflict> {
        let ActionMapping {
            keystrokes,
            context,
        } = action_mapping;
        let predicate = context
            .as_deref()
            .and_then(|ctx| gpui::KeyBindingContextPredicate::parse(&ctx).ok());
        self.keybind_mapping.get(keystrokes).and_then(|entries| {
            entries
                .iter()
                .find_map(|(other_predicate, indices)| {
                    match (&predicate, other_predicate) {
                        (None, None) => true,
                        (Some(pred), Some(other)) => normalized_ctx_eq(pred, other),
                        _ => false,
                    }
                    .then_some(indices)
                })
                .and_then(|indices| {
                    let mut indices = indices
                        .iter()
                        .filter(|&conflict| Some(conflict.index) != keybind_idx);
                    indices.next().map(|origin| KeybindConflict {
                        first_conflict_index: origin.index,
                        remaining_conflict_amount: indices.count(),
                    })
                })
        })
    }

    fn conflict_for_idx(&self, idx: usize) -> Option<ConflictOrigin> {
        self.conflicts.get(idx).copied().flatten()
    }

    fn has_user_conflict(&self, candidate_idx: usize) -> bool {
        self.conflict_for_idx(candidate_idx)
            .is_some_and(|conflict| conflict.is_user_keybind_conflict())
    }

    fn any_user_binding_conflicts(&self) -> bool {
        self.has_user_conflicts
    }
}

struct KeymapEditor {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    _keymap_subscription: Subscription,
    keybindings: Vec<ProcessedBinding>,
    keybinding_conflict_state: ConflictState,
    filter_state: FilterState,
    search_mode: SearchMode,
    search_query_debounce: Option<Task<()>>,
    // corresponds 1 to 1 with keybindings
    string_match_candidates: Arc<Vec<StringMatchCandidate>>,
    matches: Vec<StringMatch>,
    table_interaction_state: Entity<TableInteractionState>,
    filter_editor: Entity<Editor>,
    keystroke_editor: Entity<KeystrokeInput>,
    selected_index: Option<usize>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    previous_edit: Option<PreviousEdit>,
    humanized_action_names: HumanizedActionNameCache,
    current_widths: Entity<ColumnWidths<6>>,
    show_hover_menus: bool,
    /// In order for the JSON LSP to run in the actions arguments editor, we
    /// require a backing file In order to avoid issues (primarily log spam)
    /// with drop order between the buffer, file, worktree, etc, we create a
    /// temporary directory for these backing files in the keymap editor struct
    /// instead of here. This has the added benefit of only having to create a
    /// worktree and directory once, although the perf improvement is negligible.
    action_args_temp_dir_worktree: Option<Entity<project::Worktree>>,
    action_args_temp_dir: Option<tempfile::TempDir>,
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
        action_name: &'static str,
        /// The scrollbar position to fallback to if we don't find the keybinding during a refresh
        /// this can happen if there's a filter applied to the search and the keybinding modification
        /// filters the binding from the search results
        fallback: Point<Pixels>,
    },
}

impl EventEmitter<()> for KeymapEditor {}

impl Focusable for KeymapEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        if self.selected_index.is_some() {
            self.focus_handle.clone()
        } else {
            self.filter_editor.focus_handle(cx)
        }
    }
}
/// Helper function to check if two keystroke sequences match exactly
fn keystrokes_match_exactly(
    keystrokes1: &[KeybindingKeystroke],
    keystrokes2: &[KeybindingKeystroke],
) -> bool {
    keystrokes1.len() == keystrokes2.len()
        && keystrokes1.iter().zip(keystrokes2).all(|(k1, k2)| {
            k1.inner.key == k2.inner.key && k1.inner.modifiers == k2.inner.modifiers
        })
}

impl KeymapEditor {
    fn new(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let _keymap_subscription =
            cx.observe_global_in::<KeymapEventChannel>(window, Self::on_keymap_changed);
        let table_interaction_state = TableInteractionState::new(window, cx);

        let keystroke_editor = cx.new(|cx| {
            let mut keystroke_editor = KeystrokeInput::new(None, window, cx);
            keystroke_editor.set_search(true);
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

        cx.spawn({
            let workspace = workspace.clone();
            async move |this, cx| {
                let temp_dir = tempfile::tempdir_in(paths::temp_dir())?;
                let worktree = workspace
                    .update(cx, |ws, cx| {
                        ws.project()
                            .update(cx, |p, cx| p.create_worktree(temp_dir.path(), false, cx))
                    })?
                    .await?;
                this.update(cx, |this, _| {
                    this.action_args_temp_dir = Some(temp_dir);
                    this.action_args_temp_dir_worktree = Some(worktree);
                })
            }
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
            search_query_debounce: None,
            humanized_action_names: HumanizedActionNameCache::new(cx),
            show_hover_menus: true,
            action_args_temp_dir: None,
            action_args_temp_dir_worktree: None,
            current_widths: cx.new(|cx| ColumnWidths::new(cx)),
        };

        this.on_keymap_changed(window, cx);

        this
    }

    fn current_action_query(&self, cx: &App) -> String {
        self.filter_editor.read(cx).text(cx)
    }

    fn current_keystroke_query(&self, cx: &App) -> Vec<KeybindingKeystroke> {
        match self.search_mode {
            SearchMode::KeyStroke { .. } => self.keystroke_editor.read(cx).keystrokes().to_vec(),
            SearchMode::Normal => Default::default(),
        }
    }

    fn on_query_changed(&mut self, cx: &mut Context<Self>) {
        let action_query = self.current_action_query(cx);
        let keystroke_query = self.current_keystroke_query(cx);
        let exact_match = self.search_mode.exact_match();

        let timer = cx.background_executor().timer(Duration::from_secs(1));
        self.search_query_debounce = Some(cx.background_spawn({
            let action_query = action_query.clone();
            let keystroke_query = keystroke_query.clone();
            async move {
                timer.await;

                let keystroke_query = keystroke_query
                    .into_iter()
                    .map(|keystroke| keystroke.inner.unparse())
                    .collect::<Vec<String>>()
                    .join(" ");

                telemetry::event!(
                    "Keystroke Search Completed",
                    action_query = action_query,
                    keystroke_query = keystroke_query,
                    keystroke_exact_match = exact_match
                )
            }
        }));
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
        keystroke_query: Vec<KeybindingKeystroke>,
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
                            .has_user_conflict(candidate.candidate_id)
                    });
                }
                FilterState::All => {}
            }

            match this.search_mode {
                SearchMode::KeyStroke { exact_match } => {
                    matches.retain(|item| {
                        this.keybindings[item.candidate_id]
                            .keystrokes()
                            .is_some_and(|keystrokes| {
                                if exact_match {
                                    keystrokes_match_exactly(&keystroke_query, keystrokes)
                                } else if keystroke_query.len() > keystrokes.len() {
                                    false
                                } else {
                                    for keystroke_offset in 0..keystrokes.len() {
                                        let mut found_count = 0;
                                        let mut query_cursor = 0;
                                        let mut keystroke_cursor = keystroke_offset;
                                        while query_cursor < keystroke_query.len()
                                            && keystroke_cursor < keystrokes.len()
                                        {
                                            let query = &keystroke_query[query_cursor];
                                            let keystroke = &keystrokes[keystroke_cursor];
                                            let matches = query
                                                .inner
                                                .modifiers
                                                .is_subset_of(&keystroke.inner.modifiers)
                                                && ((query.inner.key.is_empty()
                                                    || query.inner.key == keystroke.inner.key)
                                                    && query.inner.key_char.as_ref().is_none_or(
                                                        |q_kc| q_kc == &keystroke.inner.key,
                                                    ));
                                            if matches {
                                                found_count += 1;
                                                query_cursor += 1;
                                            }
                                            keystroke_cursor += 1;
                                        }

                                        if found_count == keystroke_query.len() {
                                            return true;
                                        }
                                    }
                                    false
                                }
                            })
                    });
                }
                SearchMode::Normal => {}
            }

            if action_query.is_empty() {
                matches.sort_by(|item1, item2| {
                    let binding1 = &this.keybindings[item1.candidate_id];
                    let binding2 = &this.keybindings[item2.candidate_id];

                    binding1.cmp(binding2)
                });
            }
            this.selected_index.take();
            this.matches = matches;

            cx.notify();
        })
    }

    fn get_conflict(&self, row_index: usize) -> Option<ConflictOrigin> {
        self.matches.get(row_index).and_then(|candidate| {
            self.keybinding_conflict_state
                .conflict_for_idx(candidate.candidate_id)
        })
    }

    fn process_bindings(
        json_language: Arc<Language>,
        zed_keybind_context_language: Arc<Language>,
        humanized_action_names: &HumanizedActionNameCache,
        cx: &mut App,
    ) -> (Vec<ProcessedBinding>, Vec<StringMatchCandidate>) {
        let key_bindings_ptr = cx.key_bindings();
        let lock = key_bindings_ptr.borrow();
        let key_bindings = lock.bindings();
        let mut unmapped_action_names = HashSet::from_iter(cx.all_action_names().iter().copied());
        let action_documentation = cx.action_documentation();
        let mut generator = KeymapFile::action_schema_generator();
        let actions_with_schemas = HashSet::from_iter(
            cx.action_schemas(&mut generator)
                .into_iter()
                .filter_map(|(name, schema)| schema.is_some().then_some(name)),
        );

        let mut processed_bindings = Vec::new();
        let mut string_match_candidates = Vec::new();

        for key_binding in key_bindings {
            let source = key_binding
                .meta()
                .map(KeybindSource::from_meta)
                .unwrap_or(KeybindSource::Unknown);

            let keystroke_text = ui::text_for_keybinding_keystrokes(key_binding.keystrokes(), cx);
            let ui_key_binding = ui::KeyBinding::new_from_gpui(key_binding.clone(), cx)
                .vim_mode(source == KeybindSource::Vim);

            let context = key_binding
                .predicate()
                .map(|predicate| {
                    KeybindContextString::Local(
                        predicate.to_string().into(),
                        zed_keybind_context_language.clone(),
                    )
                })
                .unwrap_or(KeybindContextString::Global);

            let action_name = key_binding.action().name();
            unmapped_action_names.remove(&action_name);

            let action_arguments = key_binding
                .action_input()
                .map(|arguments| SyntaxHighlightedText::new(arguments, json_language.clone()));
            let action_information = ActionInformation::new(
                action_name,
                action_arguments,
                &actions_with_schemas,
                action_documentation,
                humanized_action_names,
            );

            let index = processed_bindings.len();
            let string_match_candidate =
                StringMatchCandidate::new(index, &action_information.humanized_name);
            processed_bindings.push(ProcessedBinding::new_mapped(
                keystroke_text,
                ui_key_binding,
                context,
                source,
                action_information,
            ));
            string_match_candidates.push(string_match_candidate);
        }

        for action_name in unmapped_action_names.into_iter() {
            let index = processed_bindings.len();
            let action_information = ActionInformation::new(
                action_name,
                None,
                &actions_with_schemas,
                action_documentation,
                humanized_action_names,
            );
            let string_match_candidate =
                StringMatchCandidate::new(index, &action_information.humanized_name);

            processed_bindings.push(ProcessedBinding::Unmapped(action_information));
            string_match_candidates.push(string_match_candidate);
        }

        (processed_bindings, string_match_candidates)
    }

    fn on_keymap_changed(&mut self, window: &mut Window, cx: &mut Context<KeymapEditor>) {
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |this, cx| {
            let json_language = load_json_language(workspace.clone(), cx).await;
            let zed_keybind_context_language =
                load_keybind_context_language(workspace.clone(), cx).await;

            let (action_query, keystroke_query) = this.update(cx, |this, cx| {
                let (key_bindings, string_match_candidates) = Self::process_bindings(
                    json_language,
                    zed_keybind_context_language,
                    &this.humanized_action_names,
                    cx,
                );

                this.keybinding_conflict_state = ConflictState::new(&key_bindings);

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
            this.update_in(cx, |this, window, cx| {
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
                                    if binding.get_action_mapping().is_some_and(|binding_mapping| {
                                        binding_mapping == action_mapping
                                    }) && binding.action().name == action_name
                                    {
                                        Some(index)
                                    } else {
                                        None
                                    }
                                });

                            if let Some(scroll_position) = scroll_position {
                                this.select_index(
                                    scroll_position,
                                    Some(ScrollStrategy::Top),
                                    window,
                                    cx,
                                );
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

    fn key_context(&self) -> KeyContext {
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

    fn selected_keybind_index(&self) -> Option<usize> {
        self.selected_index
            .and_then(|match_index| self.matches.get(match_index))
            .map(|r#match| r#match.candidate_id)
    }

    fn selected_keybind_and_index(&self) -> Option<(&ProcessedBinding, usize)> {
        self.selected_keybind_index()
            .map(|keybind_index| (&self.keybindings[keybind_index], keybind_index))
    }

    fn selected_binding(&self) -> Option<&ProcessedBinding> {
        self.selected_keybind_index()
            .and_then(|keybind_index| self.keybindings.get(keybind_index))
    }

    fn select_index(
        &mut self,
        index: usize,
        scroll: Option<ScrollStrategy>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_index != Some(index) {
            self.selected_index = Some(index);
            if let Some(scroll_strategy) = scroll {
                self.scroll_to_item(index, scroll_strategy, cx);
            }
            window.focus(&self.focus_handle);
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
                .context()
                .and_then(KeybindContextString::local)
                .is_none();

            let selected_binding_is_unbound = selected_binding.is_unbound();

            let context_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
                menu.context(self.focus_handle.clone())
                    .action_disabled_when(
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
                    .separator()
                    .action_disabled_when(
                        selected_binding_has_no_context,
                        "Show Matching Keybindings",
                        Box::new(ShowMatchingKeybinds),
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

    fn create_row_button(
        &self,
        index: usize,
        conflict: Option<ConflictOrigin>,
        cx: &mut Context<Self>,
    ) -> IconButton {
        if self.filter_state != FilterState::Conflicts
            && let Some(conflict) = conflict
        {
            if conflict.is_user_keybind_conflict() {
                base_button_style(index, IconName::Warning)
                    .icon_color(Color::Warning)
                    .tooltip(|window, cx| {
                        Tooltip::with_meta(
                            "View conflicts",
                            Some(&ToggleConflictFilter),
                            "Use alt+click to show all conflicts",
                            window,
                            cx,
                        )
                    })
                    .on_click(cx.listener(move |this, click: &ClickEvent, window, cx| {
                        if click.modifiers().alt {
                            this.set_filter_state(FilterState::Conflicts, cx);
                        } else {
                            this.select_index(index, None, window, cx);
                            this.open_edit_keybinding_modal(false, window, cx);
                            cx.stop_propagation();
                        }
                    }))
            } else if self.search_mode.exact_match() {
                base_button_style(index, IconName::Info)
                    .tooltip(|window, cx| {
                        Tooltip::with_meta(
                            "Edit this binding",
                            Some(&ShowMatchingKeybinds),
                            "This binding is overridden by other bindings.",
                            window,
                            cx,
                        )
                    })
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        this.select_index(index, None, window, cx);
                        this.open_edit_keybinding_modal(false, window, cx);
                        cx.stop_propagation();
                    }))
            } else {
                base_button_style(index, IconName::Info)
                    .tooltip(|window, cx| {
                        Tooltip::with_meta(
                            "Show matching keybinds",
                            Some(&ShowMatchingKeybinds),
                            "This binding is overridden by other bindings.\nUse alt+click to edit this binding",
                            window,
                            cx,
                        )
                    })
                    .on_click(cx.listener(move |this, click: &ClickEvent, window, cx| {
                        if click.modifiers().alt {
                            this.select_index(index, None, window, cx);
                            this.open_edit_keybinding_modal(false, window, cx);
                            cx.stop_propagation();
                        } else {
                            this.show_matching_keystrokes(&Default::default(), window, cx);
                        }
                    }))
            }
        } else {
            base_button_style(index, IconName::Pencil)
                .visible_on_hover(if self.selected_index == Some(index) {
                    "".into()
                } else if self.show_hover_menus {
                    row_group_id(index)
                } else {
                    "never-show".into()
                })
                .when(
                    self.show_hover_menus && !self.context_menu_deployed(),
                    |this| this.tooltip(Tooltip::for_action_title("Edit Keybinding", &EditBinding)),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.select_index(index, None, window, cx);
                    this.open_edit_keybinding_modal(false, window, cx);
                    cx.stop_propagation();
                }))
        }
    }

    fn render_no_matches_hint(&self, _window: &mut Window, _cx: &App) -> AnyElement {
        let hint = match (self.filter_state, &self.search_mode) {
            (FilterState::Conflicts, _) => {
                if self.keybinding_conflict_state.any_user_binding_conflicts() {
                    "No conflicting keybinds found that match the provided query"
                } else {
                    "No conflicting keybinds found"
                }
            }
            (FilterState::All, SearchMode::KeyStroke { .. }) => {
                "No keybinds found matching the entered keystrokes"
            }
            (FilterState::All, SearchMode::Normal) => "No matches found for the provided query",
        };

        Label::new(hint).color(Color::Muted).into_any_element()
    }

    fn select_next(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        self.show_hover_menus = false;
        if let Some(selected) = self.selected_index {
            let selected = selected + 1;
            if selected >= self.matches.len() {
                self.select_last(&Default::default(), window, cx);
            } else {
                self.select_index(selected, Some(ScrollStrategy::Center), window, cx);
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
        self.show_hover_menus = false;
        if let Some(selected) = self.selected_index {
            if selected == 0 {
                return;
            }

            let selected = selected - 1;

            if selected >= self.matches.len() {
                self.select_last(&Default::default(), window, cx);
            } else {
                self.select_index(selected, Some(ScrollStrategy::Center), window, cx);
            }
        } else {
            self.select_last(&Default::default(), window, cx);
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        self.show_hover_menus = false;
        if self.matches.get(0).is_some() {
            self.select_index(0, Some(ScrollStrategy::Center), window, cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        self.show_hover_menus = false;
        if self.matches.last().is_some() {
            let index = self.matches.len() - 1;
            self.select_index(index, Some(ScrollStrategy::Center), window, cx);
        }
    }

    fn open_edit_keybinding_modal(
        &mut self,
        create: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_hover_menus = false;
        let Some((keybind, keybind_index)) = self.selected_keybind_and_index() else {
            return;
        };
        let keybind = keybind.clone();
        let keymap_editor = cx.entity();

        let keystroke = keybind.keystroke_text().cloned().unwrap_or_default();
        let arguments = keybind
            .action()
            .arguments
            .as_ref()
            .map(|arguments| arguments.text.clone());
        let context = keybind
            .context()
            .map(|context| context.local_str().unwrap_or("global"));
        let action = keybind.action().name;
        let source = keybind.keybind_source().map(|source| source.name());

        telemetry::event!(
            "Edit Keybinding Modal Opened",
            keystroke = keystroke,
            action = action,
            source = source,
            context = context,
            arguments = arguments,
        );

        let temp_dir = self.action_args_temp_dir.as_ref().map(|dir| dir.path());

        self.workspace
            .update(cx, |workspace, cx| {
                let fs = workspace.app_state().fs.clone();
                let workspace_weak = cx.weak_entity();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let modal = KeybindingEditorModal::new(
                        create,
                        keybind,
                        keybind_index,
                        keymap_editor,
                        temp_dir,
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

        let std::result::Result::Ok(fs) = self
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
        let keyboard_mapper = cx.keyboard_mapper().clone();
        cx.spawn(async move |_, _| {
            remove_keybinding(to_remove, &fs, tab_size, keyboard_mapper.as_ref()).await
        })
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
            .and_then(|binding| binding.context())
            .and_then(KeybindContextString::local_str)
            .map(|context| context.to_string());
        let Some(context) = context else {
            return;
        };

        telemetry::event!("Keybinding Context Copied", context = context);
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(context));
    }

    fn copy_action_to_clipboard(
        &mut self,
        _: &CopyAction,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let action = self
            .selected_binding()
            .map(|binding| binding.action().name.to_string());
        let Some(action) = action else {
            return;
        };

        telemetry::event!("Keybinding Action Copied", action = action);
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(action));
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

        match self.search_mode {
            SearchMode::KeyStroke { .. } => {
                self.keystroke_editor.update(cx, |editor, cx| {
                    editor.start_recording(&StartRecording, window, cx);
                });
            }
            SearchMode::Normal => {
                self.keystroke_editor.update(cx, |editor, cx| {
                    editor.stop_recording(&StopRecording, window, cx);
                    editor.clear_keystrokes(&ClearKeystrokes, window, cx);
                });
                window.focus(&self.filter_editor.focus_handle(cx));
            }
        }
    }

    fn toggle_exact_keystroke_matching(
        &mut self,
        _: &ToggleExactKeystrokeMatching,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let SearchMode::KeyStroke { exact_match } = &mut self.search_mode else {
            return;
        };

        *exact_match = !(*exact_match);
        self.on_query_changed(cx);
    }

    fn show_matching_keystrokes(
        &mut self,
        _: &ShowMatchingKeybinds,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(selected_binding) = self.selected_binding() else {
            return;
        };

        let keystrokes = selected_binding
            .keystrokes()
            .map(Vec::from)
            .unwrap_or_default();

        self.filter_state = FilterState::All;
        self.search_mode = SearchMode::KeyStroke { exact_match: true };

        self.keystroke_editor.update(cx, |editor, cx| {
            editor.set_keystrokes(keystrokes, cx);
        });
    }
}

struct HumanizedActionNameCache {
    cache: HashMap<&'static str, SharedString>,
}

impl HumanizedActionNameCache {
    fn new(cx: &App) -> Self {
        let cache = HashMap::from_iter(cx.all_action_names().iter().map(|&action_name| {
            (
                action_name,
                command_palette::humanize_action_name(action_name).into(),
            )
        }));
        Self { cache }
    }

    fn get(&self, action_name: &'static str) -> SharedString {
        match self.cache.get(action_name) {
            Some(name) => name.clone(),
            None => action_name.into(),
        }
    }
}

#[derive(Clone)]
struct KeybindInformation {
    keystroke_text: SharedString,
    ui_binding: ui::KeyBinding,
    context: KeybindContextString,
    source: KeybindSource,
}

impl KeybindInformation {
    fn get_action_mapping(&self) -> ActionMapping {
        ActionMapping {
            keystrokes: self.ui_binding.keystrokes.clone(),
            context: self.context.local().cloned(),
        }
    }
}

#[derive(Clone)]
struct ActionInformation {
    name: &'static str,
    humanized_name: SharedString,
    arguments: Option<SyntaxHighlightedText>,
    documentation: Option<&'static str>,
    has_schema: bool,
}

impl ActionInformation {
    fn new(
        action_name: &'static str,
        action_arguments: Option<SyntaxHighlightedText>,
        actions_with_schemas: &HashSet<&'static str>,
        action_documentation: &HashMap<&'static str, &'static str>,
        action_name_cache: &HumanizedActionNameCache,
    ) -> Self {
        Self {
            humanized_name: action_name_cache.get(action_name),
            has_schema: actions_with_schemas.contains(action_name),
            arguments: action_arguments,
            documentation: action_documentation.get(action_name).copied(),
            name: action_name,
        }
    }
}

#[derive(Clone)]
enum ProcessedBinding {
    Mapped(KeybindInformation, ActionInformation),
    Unmapped(ActionInformation),
}

impl ProcessedBinding {
    fn new_mapped(
        keystroke_text: impl Into<SharedString>,
        ui_key_binding: ui::KeyBinding,
        context: KeybindContextString,
        source: KeybindSource,
        action_information: ActionInformation,
    ) -> Self {
        Self::Mapped(
            KeybindInformation {
                keystroke_text: keystroke_text.into(),
                ui_binding: ui_key_binding,
                context,
                source,
            },
            action_information,
        )
    }

    fn is_unbound(&self) -> bool {
        matches!(self, Self::Unmapped(_))
    }

    fn get_action_mapping(&self) -> Option<ActionMapping> {
        self.keybind_information()
            .map(|keybind| keybind.get_action_mapping())
    }

    fn keystrokes(&self) -> Option<&[KeybindingKeystroke]> {
        self.ui_key_binding()
            .map(|binding| binding.keystrokes.as_slice())
    }

    fn keybind_information(&self) -> Option<&KeybindInformation> {
        match self {
            Self::Mapped(keybind_information, _) => Some(keybind_information),
            Self::Unmapped(_) => None,
        }
    }

    fn keybind_source(&self) -> Option<KeybindSource> {
        self.keybind_information().map(|keybind| keybind.source)
    }

    fn context(&self) -> Option<&KeybindContextString> {
        self.keybind_information().map(|keybind| &keybind.context)
    }

    fn ui_key_binding(&self) -> Option<&ui::KeyBinding> {
        self.keybind_information()
            .map(|keybind| &keybind.ui_binding)
    }

    fn keystroke_text(&self) -> Option<&SharedString> {
        self.keybind_information()
            .map(|binding| &binding.keystroke_text)
    }

    fn action(&self) -> &ActionInformation {
        match self {
            Self::Mapped(_, action) | Self::Unmapped(action) => action,
        }
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Self::Mapped(keybind1, action1), Self::Mapped(keybind2, action2)) => {
                match keybind1.source.cmp(&keybind2.source) {
                    cmp::Ordering::Equal => action1.humanized_name.cmp(&action2.humanized_name),
                    ordering => ordering,
                }
            }
            (Self::Mapped(_, _), Self::Unmapped(_)) => cmp::Ordering::Less,
            (Self::Unmapped(_), Self::Mapped(_, _)) => cmp::Ordering::Greater,
            (Self::Unmapped(action1), Self::Unmapped(action2)) => {
                action1.humanized_name.cmp(&action2.humanized_name)
            }
        }
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
                muted_styled_text(KeybindContextString::GLOBAL, cx).into_any_element()
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
    fn render(&mut self, _window: &mut Window, cx: &mut ui::Context<Self>) -> impl ui::IntoElement {
        let row_count = self.matches.len();
        let theme = cx.theme();
        let focus_handle = &self.focus_handle;

        v_flex()
            .id("keymap-editor")
            .track_focus(focus_handle)
            .key_context(self.key_context())
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::focus_search))
            .on_action(cx.listener(Self::edit_binding))
            .on_action(cx.listener(Self::create_binding))
            .on_action(cx.listener(Self::delete_binding))
            .on_action(cx.listener(Self::copy_action_to_clipboard))
            .on_action(cx.listener(Self::copy_context_to_clipboard))
            .on_action(cx.listener(Self::toggle_conflict_filter))
            .on_action(cx.listener(Self::toggle_keystroke_search))
            .on_action(cx.listener(Self::toggle_exact_keystroke_matching))
            .on_action(cx.listener(Self::show_matching_keystrokes))
            .on_mouse_move(cx.listener(|this, _, _window, _cx| {
                this.show_hover_menus = true;
            }))
            .size_full()
            .p_2()
            .gap_1()
            .bg(theme.colors().editor_background)
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                right_click_menu("open-keymap-menu")
                                    .menu(|window, cx| {
                                        ContextMenu::build(window, cx, |menu, _, _| {
                                            menu.header("Open Keymap JSON")
                                                .action("User", zed_actions::OpenKeymap.boxed_clone())
                                                .action("Zed Default", zed_actions::OpenDefaultKeymap.boxed_clone())
                                                .action("Vim Default", vim::OpenDefaultKeymap.boxed_clone())
                                        })
                                    })
                                    .anchor(gpui::Corner::TopLeft)
                                    .trigger(|open, _, _|
                                        IconButton::new(
                                            "OpenKeymapJsonButton",
                                            IconName::Json
                                        )
                                        .shape(ui::IconButtonShape::Square)
                                        .when(!open, |this|
                                            this.tooltip(move |window, cx| {
                                                Tooltip::with_meta("Open Keymap JSON", Some(&zed_actions::OpenKeymap),"Right click to view more options", window, cx)
                                            })
                                        )
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(zed_actions::OpenKeymap.boxed_clone(), cx);
                                        })
                                    )
                            )
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
                                .tooltip({
                                    let focus_handle = focus_handle.clone();

                                    move |window, cx| {
                                        Tooltip::for_action_in(
                                            "Search by Keystroke",
                                            &ToggleKeystrokeSearch,
                                            &focus_handle.clone(),
                                            window,
                                            cx,
                                        )
                                    }
                                })
                                .toggle_state(matches!(
                                    self.search_mode,
                                    SearchMode::KeyStroke { .. }
                                ))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(ToggleKeystrokeSearch.boxed_clone(), cx);
                                }),
                            )
                            .child(
                                IconButton::new("KeymapEditorConflictIcon", IconName::Warning)
                                    .shape(ui::IconButtonShape::Square)
                                    .when(
                                        self.keybinding_conflict_state.any_user_binding_conflicts(),
                                        |this| {
                                            this.indicator(Indicator::dot().color(Color::Warning))
                                        },
                                    )
                                    .tooltip({
                                        let filter_state = self.filter_state;
                                        let focus_handle = focus_handle.clone();

                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                match filter_state {
                                                    FilterState::All => "Show Conflicts",
                                                    FilterState::Conflicts => "Hide Conflicts",
                                                },
                                                &ToggleConflictFilter,
                                                &focus_handle.clone(),
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
                            ),
                    )
                    .when_some(
                        match self.search_mode {
                            SearchMode::Normal => None,
                            SearchMode::KeyStroke { exact_match } => Some(exact_match),
                        },
                        |this, exact_match| {
                            this.child(
                                h_flex()
                                    .map(|this| {
                                        if self
                                            .keybinding_conflict_state
                                            .any_user_binding_conflicts()
                                        {
                                            this.pr(rems_from_px(54.))
                                        } else {
                                            this.pr_7()
                                        }
                                    })
                                    .gap_2()
                                    .child(self.keystroke_editor.clone())
                                    .child(
                                        IconButton::new(
                                            "keystrokes-exact-match",
                                            IconName::CaseSensitive,
                                        )
                                        .tooltip({
                                            let keystroke_focus_handle =
                                                self.keystroke_editor.read(cx).focus_handle(cx);

                                            move |window, cx| {
                                                Tooltip::for_action_in(
                                                    "Toggle Exact Match Mode",
                                                    &ToggleExactKeystrokeMatching,
                                                    &keystroke_focus_handle,
                                                    window,
                                                    cx,
                                                )
                                            }
                                        })
                                        .shape(IconButtonShape::Square)
                                        .toggle_state(exact_match)
                                        .on_click(
                                            cx.listener(|_, _, window, cx| {
                                                window.dispatch_action(
                                                    ToggleExactKeystrokeMatching.boxed_clone(),
                                                    cx,
                                                );
                                            }),
                                        ),
                                    ),
                            )
                        },
                    ),
            )
            .child(
                Table::new()
                    .interactable(&self.table_interaction_state)
                    .striped()
                    .empty_table_callback({
                        let this = cx.entity();
                        move |window, cx| this.read(cx).render_no_matches_hint(window, cx)
                    })
                    .column_widths([
                        DefiniteLength::Absolute(AbsoluteLength::Pixels(px(36.))),
                        DefiniteLength::Fraction(0.25),
                        DefiniteLength::Fraction(0.20),
                        DefiniteLength::Fraction(0.14),
                        DefiniteLength::Fraction(0.45),
                        DefiniteLength::Fraction(0.08),
                    ])
                    .resizable_columns(
                        [
                            ResizeBehavior::None,
                            ResizeBehavior::Resizable,
                            ResizeBehavior::Resizable,
                            ResizeBehavior::Resizable,
                            ResizeBehavior::Resizable,
                            ResizeBehavior::Resizable, // this column doesn't matter
                        ],
                        &self.current_widths,
                        cx,
                    )
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
                                    let action_name = binding.action().name;
                                    let conflict = this.get_conflict(index);
                                    let is_overridden = conflict.is_some_and(|conflict| {
                                        !conflict.is_user_keybind_conflict()
                                    });

                                    let icon = this.create_row_button(index, conflict, cx);

                                    let action = div()
                                        .id(("keymap action", index))
                                        .child({
                                            if action_name != gpui::NoAction.name() {
                                                binding
                                                    .action()
                                                    .humanized_name
                                                    .clone()
                                                    .into_any_element()
                                            } else {
                                                const NULL: SharedString =
                                                    SharedString::new_static("<null>");
                                                muted_styled_text(NULL, cx)
                                                    .into_any_element()
                                            }
                                        })
                                        .when(
                                            !context_menu_deployed
                                                && this.show_hover_menus
                                                && !is_overridden,
                                            |this| {
                                                this.tooltip({
                                                    let action_name = binding.action().name;
                                                    let action_docs =
                                                        binding.action().documentation;
                                                    move |_, cx| {
                                                        let action_tooltip =
                                                            Tooltip::new(action_name);
                                                        let action_tooltip = match action_docs {
                                                            Some(docs) => action_tooltip.meta(docs),
                                                            None => action_tooltip,
                                                        };
                                                        cx.new(|_| action_tooltip).into()
                                                    }
                                                })
                                            },
                                        )
                                        .into_any_element();

                                    let keystrokes = binding.ui_key_binding().cloned().map_or(
                                        binding
                                            .keystroke_text()
                                            .cloned()
                                            .unwrap_or_default()
                                            .into_any_element(),
                                        IntoElement::into_any_element,
                                    );

                                    let action_arguments = match binding.action().arguments.clone()
                                    {
                                        Some(arguments) => arguments.into_any_element(),
                                        None => {
                                            if binding.action().has_schema {
                                                muted_styled_text(NO_ACTION_ARGUMENTS_TEXT, cx)
                                                    .into_any_element()
                                            } else {
                                                gpui::Empty.into_any_element()
                                            }
                                        }
                                    };

                                    let context = binding.context().cloned().map_or(
                                        gpui::Empty.into_any_element(),
                                        |context| {
                                            let is_local = context.local().is_some();

                                            div()
                                                .id(("keymap context", index))
                                                .child(context.clone())
                                                .when(
                                                    is_local
                                                        && !context_menu_deployed
                                                        && !is_overridden
                                                        && this.show_hover_menus,
                                                    |this| {
                                                        this.tooltip(Tooltip::element({
                                                            move |_, _| {
                                                                context.clone().into_any_element()
                                                            }
                                                        }))
                                                    },
                                                )
                                                .into_any_element()
                                        },
                                    );

                                    let source = binding
                                        .keybind_source()
                                        .map(|source| source.name())
                                        .unwrap_or_default()
                                        .into_any_element();

                                    Some([
                                        icon.into_any_element(),
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
                    .map_row(cx.processor(
                        |this, (row_index, row): (usize, Stateful<Div>), _window, cx| {
                        let conflict = this.get_conflict(row_index);
                            let is_selected = this.selected_index == Some(row_index);

                            let row_id = row_group_id(row_index);

                            div()
                                .id(("keymap-row-wrapper", row_index))
                                .child(
                                    row.id(row_id.clone())
                                        .on_any_mouse_down(cx.listener(
                                            move |this,
                                                  mouse_down_event: &gpui::MouseDownEvent,
                                                  window,
                                                  cx| {
                                                if mouse_down_event.button == MouseButton::Right {
                                                    this.select_index(
                                                        row_index, None, window, cx,
                                                    );
                                                    this.create_context_menu(
                                                        mouse_down_event.position,
                                                        window,
                                                        cx,
                                                    );
                                                }
                                            },
                                        ))
                                        .on_click(cx.listener(
                                            move |this, event: &ClickEvent, window, cx| {
                                                this.select_index(row_index, None, window, cx);
                                                if event.click_count() == 2 {
                                                    this.open_edit_keybinding_modal(
                                                        false, window, cx,
                                                    );
                                                }
                                            },
                                        ))
                                        .group(row_id)
                                        .when(
                                            conflict.is_some_and(|conflict| {
                                                !conflict.is_user_keybind_conflict()
                                            }),
                                            |row| {
                                                const OVERRIDDEN_OPACITY: f32 = 0.5;
                                                row.opacity(OVERRIDDEN_OPACITY)
                                            },
                                        )
                                        .when_some(
                                            conflict.filter(|conflict| {
                                                !this.context_menu_deployed() &&
                                                !conflict.is_user_keybind_conflict()
                                            }),
                                            |row, conflict| {
                                                let overriding_binding = this.keybindings.get(conflict.index);
                                                let context = overriding_binding.and_then(|binding| {
                                                    match conflict.override_source {
                                                        KeybindSource::User  => Some("your keymap"),
                                                        KeybindSource::Vim => Some("the vim keymap"),
                                                        KeybindSource::Base => Some("your base keymap"),
                                                        _ => {
                                                            log::error!("Unexpected override from the {} keymap", conflict.override_source.name());
                                                            None
                                                        }
                                                    }.map(|source| format!("This keybinding is overridden by the '{}' binding from {}.", binding.action().humanized_name, source))
                                                }).unwrap_or_else(|| "This binding is overridden.".to_string());

                                                row.tooltip(Tooltip::text(context))},
                                        ),
                                )
                                .border_2()
                                .when(
                                    conflict.is_some_and(|conflict| {
                                        conflict.is_user_keybind_conflict()
                                    }),
                                    |row| row.bg(cx.theme().status().error_background),
                                )
                                .when(is_selected, |row| {
                                    row.border_color(cx.theme().colors().panel_focused_border)
                                })
                                .into_any_element()
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

        StyledText::new(text).with_runs(runs)
    }
}

#[derive(PartialEq)]
struct InputError {
    severity: Severity,
    content: SharedString,
}

impl InputError {
    fn warning(message: impl Into<SharedString>) -> Self {
        Self {
            severity: Severity::Warning,
            content: message.into(),
        }
    }

    fn error(message: anyhow::Error) -> Self {
        Self {
            severity: Severity::Error,
            content: message.to_string().into(),
        }
    }
}

struct KeybindingEditorModal {
    creating: bool,
    editing_keybind: ProcessedBinding,
    editing_keybind_idx: usize,
    keybind_editor: Entity<KeystrokeInput>,
    context_editor: Entity<SingleLineInput>,
    action_arguments_editor: Option<Entity<ActionArgumentsEditor>>,
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
        editing_keybind: ProcessedBinding,
        editing_keybind_idx: usize,
        keymap_editor: Entity<KeymapEditor>,
        action_args_temp_dir: Option<&std::path::Path>,
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
                .context()
                .and_then(KeybindContextString::local)
            {
                input.editor().update(cx, |editor, cx| {
                    editor.set_text(context.clone(), window, cx);
                });
            }

            let editor_entity = input.editor().clone();
            let workspace = workspace.clone();
            cx.spawn(async move |_input_handle, cx| {
                let contexts = cx
                    .background_spawn(async { collect_contexts_from_assets() })
                    .await;

                let language = load_keybind_context_language(workspace, cx).await;
                editor_entity
                    .update(cx, |editor, cx| {
                        if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                            buffer.update(cx, |buffer, cx| {
                                buffer.set_language(Some(language), cx);
                            });
                        }
                        editor.set_completion_provider(Some(std::rc::Rc::new(
                            KeyContextCompletionProvider { contexts },
                        )));
                    })
                    .context("Failed to load completions for keybinding context")
            })
            .detach_and_log_err(cx);

            input
        });

        let action_arguments_editor = editing_keybind.action().has_schema.then(|| {
            let arguments = editing_keybind
                .action()
                .arguments
                .as_ref()
                .map(|args| args.text.clone());
            cx.new(|cx| {
                ActionArgumentsEditor::new(
                    editing_keybind.action().name,
                    arguments,
                    action_args_temp_dir,
                    workspace.clone(),
                    window,
                    cx,
                )
            })
        });

        let focus_state = KeybindingEditorModalFocusState::new(
            keybind_editor.focus_handle(cx),
            action_arguments_editor
                .as_ref()
                .map(|args_editor| args_editor.focus_handle(cx)),
            context_editor.focus_handle(cx),
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
            .is_some_and(|old_error| old_error.severity == Severity::Warning && *old_error == error)
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
            .map(|arguments_editor| arguments_editor.read(cx).editor.read(cx).text(cx))
            .filter(|args| !args.is_empty());

        let value = action_arguments
            .as_ref()
            .map(|args| {
                serde_json::from_str(args).context("Failed to parse action arguments as JSON")
            })
            .transpose()?;

        cx.build_action(self.editing_keybind.action().name, value)
            .context("Failed to validate action arguments")?;
        Ok(action_arguments)
    }

    fn validate_keystrokes(&self, cx: &App) -> anyhow::Result<Vec<KeybindingKeystroke>> {
        let new_keystrokes = self
            .keybind_editor
            .read_with(cx, |editor, _| editor.keystrokes().to_vec());
        anyhow::ensure!(!new_keystrokes.is_empty(), "Keystrokes cannot be empty");
        Ok(new_keystrokes)
    }

    fn validate_context(&self, cx: &App) -> anyhow::Result<Option<String>> {
        let new_context = self
            .context_editor
            .read_with(cx, |input, cx| input.editor().read(cx).text(cx));
        let Some(context) = new_context.is_empty().not().then_some(new_context) else {
            return Ok(None);
        };
        gpui::KeyBindingContextPredicate::parse(&context).context("Failed to parse key context")?;

        Ok(Some(context))
    }

    fn save_or_display_error(&mut self, cx: &mut Context<Self>) {
        self.save(cx).map_err(|err| self.set_error(err, cx)).ok();
    }

    fn save(&mut self, cx: &mut Context<Self>) -> Result<(), InputError> {
        let existing_keybind = self.editing_keybind.clone();
        let fs = self.fs.clone();
        let tab_size = cx.global::<settings::SettingsStore>().json_tab_size();

        let new_keystrokes = self
            .validate_keystrokes(cx)
            .map_err(InputError::error)?
            .into_iter()
            .map(remove_key_char)
            .collect::<Vec<_>>();

        let new_context = self.validate_context(cx).map_err(InputError::error)?;
        let new_action_args = self
            .validate_action_arguments(cx)
            .map_err(InputError::error)?;

        let action_mapping = ActionMapping {
            keystrokes: new_keystrokes,
            context: new_context.map(SharedString::from),
        };

        let conflicting_indices = self
            .keymap_editor
            .read(cx)
            .keybinding_conflict_state
            .conflicting_indices_for_mapping(
                &action_mapping,
                self.creating.not().then_some(self.editing_keybind_idx),
            );

        conflicting_indices.map(|KeybindConflict {
            first_conflict_index,
            remaining_conflict_amount,
        }|
        {
            let conflicting_action_name = self
                .keymap_editor
                .read(cx)
                .keybindings
                .get(first_conflict_index)
                .map(|keybind| keybind.action().name);

            let warning_message = match conflicting_action_name {
                Some(name) => {
                     if remaining_conflict_amount > 0 {
                        format!(
                            "Your keybind would conflict with the \"{}\" action and {} other bindings",
                            name, remaining_conflict_amount
                        )
                    } else {
                        format!("Your keybind would conflict with the \"{}\" action", name)
                    }
                }
                None => {
                    log::info!(
                        "Could not find action in keybindings with index {}",
                        first_conflict_index
                    );
                    "Your keybind would conflict with other actions".to_string()
                }
            };

            let warning = InputError::warning(warning_message);
            if self.error.as_ref().is_some_and(|old_error| *old_error == warning) {
                Ok(())
           } else {
                Err(warning)
            }
        }).unwrap_or(Ok(()))?;

        let create = self.creating;
        let keyboard_mapper = cx.keyboard_mapper().clone();

        cx.spawn(async move |this, cx| {
            let action_name = existing_keybind.action().name;
            let humanized_action_name = existing_keybind.action().humanized_name.clone();

            match save_keybinding_update(
                create,
                existing_keybind,
                &action_mapping,
                new_action_args.as_deref(),
                &fs,
                tab_size,
                keyboard_mapper.as_ref(),
            )
            .await
            {
                Ok(_) => {
                    this.update(cx, |this, cx| {
                        this.keymap_editor.update(cx, |keymap, cx| {
                            keymap.previous_edit = Some(PreviousEdit::Keybinding {
                                action_mapping,
                                action_name,
                                fallback: keymap
                                    .table_interaction_state
                                    .read(cx)
                                    .get_scrollbar_offset(Axis::Vertical),
                            });
                            let status_toast = StatusToast::new(
                                format!("Saved edits to the {} action.", humanized_action_name),
                                cx,
                                move |this, _cx| {
                                    this.icon(ToastIcon::new(IconName::Check).color(Color::Success))
                                        .dismiss_button(true)
                                    // .action("Undo", f) todo: wire the undo functionality
                                },
                            );

                            this.workspace
                                .update(cx, |workspace, cx| {
                                    workspace.toggle_status_toast(status_toast, cx);
                                })
                                .log_err();
                        });
                        cx.emit(DismissEvent);
                    })
                    .ok();
                }
                Err(err) => {
                    this.update(cx, |this, cx| {
                        this.set_error(InputError::error(err), cx);
                    })
                    .log_err();
                }
            }
        })
        .detach();

        Ok(())
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
        self.save_or_display_error(cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn get_matching_bindings_count(&self, cx: &Context<Self>) -> usize {
        let current_keystrokes = self.keybind_editor.read(cx).keystrokes().to_vec();

        if current_keystrokes.is_empty() {
            return 0;
        }

        self.keymap_editor
            .read(cx)
            .keybindings
            .iter()
            .enumerate()
            .filter(|(idx, binding)| {
                // Don't count the binding we're currently editing
                if !self.creating && *idx == self.editing_keybind_idx {
                    return false;
                }

                binding
                    .keystrokes()
                    .map(|keystrokes| keystrokes_match_exactly(keystrokes, &current_keystrokes))
                    .unwrap_or(false)
            })
            .count()
    }

    fn show_matching_bindings(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let keystrokes = self.keybind_editor.read(cx).keystrokes().to_vec();

        // Dismiss the modal
        cx.emit(DismissEvent);

        // Update the keymap editor to show matching keystrokes
        self.keymap_editor.update(cx, |editor, cx| {
            editor.filter_state = FilterState::All;
            editor.search_mode = SearchMode::KeyStroke { exact_match: true };
            editor.keystroke_editor.update(cx, |keystroke_editor, cx| {
                keystroke_editor.set_keystrokes(keystrokes, cx);
            });
        });
    }
}

fn remove_key_char(
    KeybindingKeystroke {
        inner,
        display_modifiers,
        display_key,
    }: KeybindingKeystroke,
) -> KeybindingKeystroke {
    KeybindingKeystroke {
        inner: Keystroke {
            modifiers: inner.modifiers,
            key: inner.key,
            key_char: None,
        },
        display_modifiers,
        display_key,
    }
}

impl Render for KeybindingEditorModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().colors();
        let matching_bindings_count = self.get_matching_bindings_count(cx);

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
                                .w_full()
                                .pb_1p5()
                                .mb_1()
                                .gap_0p5()
                                .border_b_1()
                                .border_color(theme.border_variant)
                                .child(Label::new(
                                    self.editing_keybind.action().humanized_name.clone(),
                                ))
                                .when_some(
                                    self.editing_keybind.action().documentation,
                                    |this, docs| {
                                        this.child(
                                            Label::new(docs)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    },
                                ),
                        ),
                    )
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_2p5()
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(Label::new("Edit Keystroke"))
                                        .child(self.keybind_editor.clone())
                                        .child(h_flex().gap_px().when(
                                            matching_bindings_count > 0,
                                            |this| {
                                                let label = format!(
                                                    "There {} {} {} with the same keystrokes.",
                                                    if matching_bindings_count == 1 {
                                                        "is"
                                                    } else {
                                                        "are"
                                                    },
                                                    matching_bindings_count,
                                                    if matching_bindings_count == 1 {
                                                        "binding"
                                                    } else {
                                                        "bindings"
                                                    }
                                                );

                                                this.child(
                                                    Label::new(label)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                )
                                                .child(
                                                    Button::new("show_matching", "View")
                                                        .label_size(LabelSize::Small)
                                                        .icon(IconName::ArrowUpRight)
                                                        .icon_color(Color::Muted)
                                                        .icon_size(IconSize::Small)
                                                        .on_click(cx.listener(
                                                            |this, _, window, cx| {
                                                                this.show_matching_bindings(
                                                                    window, cx,
                                                                );
                                                            },
                                                        )),
                                                )
                                            },
                                        )),
                                )
                                .when_some(self.action_arguments_editor.clone(), |this, editor| {
                                    this.child(
                                        v_flex()
                                            .gap_1()
                                            .child(Label::new("Edit Arguments"))
                                            .child(editor),
                                    )
                                })
                                .child(self.context_editor.clone())
                                .when_some(self.error.as_ref(), |this, error| {
                                    this.child(
                                        Banner::new()
                                            .severity(error.severity)
                                            .child(Label::new(error.content.clone())),
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
                                        this.save_or_display_error(cx);
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

struct ActionArgumentsEditor {
    editor: Entity<Editor>,
    focus_handle: FocusHandle,
    is_loading: bool,
    /// See documentation in `KeymapEditor` for why a temp dir is needed.
    /// This field exists because the keymap editor temp dir creation may fail,
    /// and rather than implement a complicated retry mechanism, we simply
    /// fallback to trying to create a temporary directory in this editor on
    /// demand. Of note is that the TempDir struct will remove the directory
    /// when dropped.
    backup_temp_dir: Option<tempfile::TempDir>,
}

impl Focusable for ActionArgumentsEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ActionArgumentsEditor {
    fn new(
        action_name: &'static str,
        arguments: Option<SharedString>,
        temp_dir: Option<&std::path::Path>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus_in(&focus_handle, window, |this, window, cx| {
            this.editor.focus_handle(cx).focus(window);
        })
        .detach();
        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height_unbounded(1, window, cx);
            Self::set_editor_text(&mut editor, arguments.clone(), window, cx);
            editor.set_read_only(true);
            editor
        });

        let temp_dir = temp_dir.map(|path| path.to_owned());
        cx.spawn_in(window, async move |this, cx| {
            let result = async {
                let (project, fs) = workspace.read_with(cx, |workspace, _cx| {
                    (
                        workspace.project().downgrade(),
                        workspace.app_state().fs.clone(),
                    )
                })?;

                let file_name =
                    project::lsp_store::json_language_server_ext::normalized_action_file_name(
                        action_name,
                    );

                let (buffer, backup_temp_dir) =
                    Self::create_temp_buffer(temp_dir, file_name.clone(), project.clone(), fs, cx)
                        .await
                        .context(concat!(
                            "Failed to create temporary buffer for action arguments. ",
                            "Auto-complete will not work"
                        ))?;

                let editor = cx.new_window_entity(|window, cx| {
                    let multi_buffer = cx.new(|cx| editor::MultiBuffer::singleton(buffer, cx));
                    let mut editor = Editor::new(
                        editor::EditorMode::Full {
                            scale_ui_elements_with_buffer_font_size: true,
                            show_active_line_background: false,
                            sized_by_content: true,
                        },
                        multi_buffer,
                        project.upgrade(),
                        window,
                        cx,
                    );
                    editor.set_searchable(false);
                    editor.disable_scrollbars_and_minimap(window, cx);
                    editor.set_show_edit_predictions(Some(false), window, cx);
                    editor.set_show_gutter(false, cx);
                    Self::set_editor_text(&mut editor, arguments, window, cx);
                    editor
                })?;

                this.update_in(cx, |this, window, cx| {
                    if this.editor.focus_handle(cx).is_focused(window) {
                        editor.focus_handle(cx).focus(window);
                    }
                    this.editor = editor;
                    this.backup_temp_dir = backup_temp_dir;
                    this.is_loading = false;
                })?;

                anyhow::Ok(())
            }
            .await;
            if result.is_err() {
                let json_language = load_json_language(workspace.clone(), cx).await;
                this.update(cx, |this, cx| {
                    this.editor.update(cx, |editor, cx| {
                        if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                            buffer.update(cx, |buffer, cx| {
                                buffer.set_language(Some(json_language.clone()), cx)
                            });
                        }
                    })
                    // .context("Failed to load JSON language for editing keybinding action arguments input")
                })
                .ok();
                this.update(cx, |this, _cx| {
                    this.is_loading = false;
                })
                .ok();
            }
            result
        })
        .detach_and_log_err(cx);
        Self {
            editor,
            focus_handle,
            is_loading: true,
            backup_temp_dir: None,
        }
    }

    fn set_editor_text(
        editor: &mut Editor,
        arguments: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if let Some(arguments) = arguments {
            editor.set_text(arguments, window, cx);
        } else {
            // TODO: default value from schema?
            editor.set_placeholder_text("Action Arguments", cx);
        }
    }

    async fn create_temp_buffer(
        temp_dir: Option<std::path::PathBuf>,
        file_name: String,
        project: WeakEntity<Project>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<(Entity<language::Buffer>, Option<tempfile::TempDir>)> {
        let (temp_file_path, temp_dir) = {
            let file_name = file_name.clone();
            async move {
                let temp_dir_backup = match temp_dir.as_ref() {
                    Some(_) => None,
                    None => {
                        let temp_dir = paths::temp_dir();
                        let sub_temp_dir = tempfile::Builder::new()
                            .tempdir_in(temp_dir)
                            .context("Failed to create temporary directory")?;
                        Some(sub_temp_dir)
                    }
                };
                let dir_path = temp_dir.as_deref().unwrap_or_else(|| {
                    temp_dir_backup
                        .as_ref()
                        .expect("created backup tempdir")
                        .path()
                });
                let path = dir_path.join(file_name);
                fs.create_file(
                    &path,
                    fs::CreateOptions {
                        ignore_if_exists: true,
                        overwrite: true,
                    },
                )
                .await
                .context("Failed to create temporary file")?;
                anyhow::Ok((path, temp_dir_backup))
            }
        }
        .await
        .context("Failed to create backing file")?;

        project
            .update(cx, |project, cx| {
                project.open_local_buffer(temp_file_path, cx)
            })?
            .await
            .context("Failed to create buffer")
            .map(|buffer| (buffer, temp_dir))
    }
}

impl Render for ActionArgumentsEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let background_color;
        let border_color;
        let text_style = {
            let colors = cx.theme().colors();
            let settings = theme::ThemeSettings::get_global(cx);
            background_color = colors.editor_background;
            border_color = if self.is_loading {
                colors.border_disabled
            } else {
                colors.border_variant
            };
            TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                font_weight: Some(settings.buffer_font.weight),
                line_height: Some(relative(1.2)),
                font_style: Some(gpui::FontStyle::Normal),
                color: self.is_loading.then_some(colors.text_disabled),
                ..Default::default()
            }
        };

        self.editor
            .update(cx, |editor, _| editor.set_text_style_refinement(text_style));

        v_flex().w_full().child(
            h_flex()
                .min_h_8()
                .min_w_48()
                .px_2()
                .py_1p5()
                .flex_grow()
                .rounded_lg()
                .bg(background_color)
                .border_1()
                .border_color(border_color)
                .track_focus(&self.focus_handle)
                .child(self.editor.clone()),
        )
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
        let start_anchor =
            buffer.anchor_before(buffer_position.to_offset(buffer).saturating_sub(count_back));
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
        text.chars()
            .last()
            .is_some_and(|last_char| last_char.is_ascii_alphanumeric() || last_char == '_')
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
    json_language.unwrap_or_else(|| {
        Arc::new(Language::new(
            LanguageConfig {
                name: "JSON".into(),
                ..Default::default()
            },
            Some(tree_sitter_json::LANGUAGE.into()),
        ))
    })
}

async fn load_keybind_context_language(
    workspace: WeakEntity<Workspace>,
    cx: &mut AsyncApp,
) -> Arc<Language> {
    let language_task = workspace
        .read_with(cx, |workspace, cx| {
            workspace
                .project()
                .read(cx)
                .languages()
                .language_for_name("Zed Keybind Context")
        })
        .context("Failed to load Zed Keybind Context language")
        .log_err();
    let language = match language_task {
        Some(task) => task
            .await
            .context("Failed to load Zed Keybind Context language")
            .log_err(),
        None => None,
    };
    language.unwrap_or_else(|| {
        Arc::new(Language::new(
            LanguageConfig {
                name: "Zed Keybind Context".into(),
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        ))
    })
}

async fn save_keybinding_update(
    create: bool,
    existing: ProcessedBinding,
    action_mapping: &ActionMapping,
    new_args: Option<&str>,
    fs: &Arc<dyn Fs>,
    tab_size: usize,
    keyboard_mapper: &dyn PlatformKeyboardMapper,
) -> anyhow::Result<()> {
    let keymap_contents = settings::KeymapFile::load_keymap_file(fs)
        .await
        .context("Failed to load keymap file")?;

    let existing_keystrokes = existing.keystrokes().unwrap_or_default();
    let existing_context = existing.context().and_then(KeybindContextString::local_str);
    let existing_args = existing
        .action()
        .arguments
        .as_ref()
        .map(|args| args.text.as_ref());

    let target = settings::KeybindUpdateTarget {
        context: existing_context,
        keystrokes: existing_keystrokes,
        action_name: existing.action().name,
        action_arguments: existing_args,
    };

    let source = settings::KeybindUpdateTarget {
        context: action_mapping.context.as_ref().map(|a| &***a),
        keystrokes: &action_mapping.keystrokes,
        action_name: existing.action().name,
        action_arguments: new_args,
    };

    let operation = if !create {
        settings::KeybindUpdateOperation::Replace {
            target,
            target_keybind_source: existing.keybind_source().unwrap_or(KeybindSource::User),
            source,
        }
    } else {
        settings::KeybindUpdateOperation::Add {
            source,
            from: Some(target),
        }
    };

    let (new_keybinding, removed_keybinding, source) = operation.generate_telemetry();

    let updated_keymap_contents = settings::KeymapFile::update_keybinding(
        operation,
        keymap_contents,
        tab_size,
        keyboard_mapper,
    )
    .map_err(|err| anyhow::anyhow!("Could not save updated keybinding: {}", err))?;
    fs.write(
        paths::keymap_file().as_path(),
        updated_keymap_contents.as_bytes(),
    )
    .await
    .context("Failed to write keymap file")?;

    telemetry::event!(
        "Keybinding Updated",
        new_keybinding = new_keybinding,
        removed_keybinding = removed_keybinding,
        source = source
    );
    Ok(())
}

async fn remove_keybinding(
    existing: ProcessedBinding,
    fs: &Arc<dyn Fs>,
    tab_size: usize,
    keyboard_mapper: &dyn PlatformKeyboardMapper,
) -> anyhow::Result<()> {
    let Some(keystrokes) = existing.keystrokes() else {
        anyhow::bail!("Cannot remove a keybinding that does not exist");
    };
    let keymap_contents = settings::KeymapFile::load_keymap_file(fs)
        .await
        .context("Failed to load keymap file")?;

    let operation = settings::KeybindUpdateOperation::Remove {
        target: settings::KeybindUpdateTarget {
            context: existing.context().and_then(KeybindContextString::local_str),
            keystrokes,
            action_name: existing.action().name,
            action_arguments: existing
                .action()
                .arguments
                .as_ref()
                .map(|arguments| arguments.text.as_ref()),
        },
        target_keybind_source: existing.keybind_source().unwrap_or(KeybindSource::User),
    };

    let (new_keybinding, removed_keybinding, source) = operation.generate_telemetry();
    let updated_keymap_contents = settings::KeymapFile::update_keybinding(
        operation,
        keymap_contents,
        tab_size,
        keyboard_mapper,
    )
    .context("Failed to update keybinding")?;
    fs.write(
        paths::keymap_file().as_path(),
        updated_keymap_contents.as_bytes(),
    )
    .await
    .context("Failed to write keymap file")?;

    telemetry::event!(
        "Keybinding Removed",
        new_keybinding = new_keybinding,
        removed_keybinding = removed_keybinding,
        source = source
    );
    Ok(())
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
                    Identifier(ident) => {
                        contexts.insert(ident);
                    }
                    Equal(ident_a, ident_b) => {
                        contexts.insert(ident_a);
                        contexts.insert(ident_b);
                    }
                    NotEqual(ident_a, ident_b) => {
                        contexts.insert(ident_a);
                        contexts.insert(ident_b);
                    }
                    Descendant(ctx_a, ctx_b) => {
                        queue.push(*ctx_a);
                        queue.push(*ctx_b);
                    }
                    Not(ctx) => {
                        queue.push(*ctx);
                    }
                    And(ctx_a, ctx_b) => {
                        queue.push(*ctx_a);
                        queue.push(*ctx_b);
                    }
                    Or(ctx_a, ctx_b) => {
                        queue.push(*ctx_a);
                        queue.push(*ctx_b);
                    }
                }
            }
        }
    }

    let mut contexts = contexts.into_iter().collect::<Vec<_>>();
    contexts.sort();

    contexts
}

fn normalized_ctx_eq(
    a: &gpui::KeyBindingContextPredicate,
    b: &gpui::KeyBindingContextPredicate,
) -> bool {
    use gpui::KeyBindingContextPredicate::*;
    return match (a, b) {
        (Identifier(_), Identifier(_)) => a == b,
        (Equal(a_left, a_right), Equal(b_left, b_right)) => {
            (a_left == b_left && a_right == b_right) || (a_left == b_right && a_right == b_left)
        }
        (NotEqual(a_left, a_right), NotEqual(b_left, b_right)) => {
            (a_left == b_left && a_right == b_right) || (a_left == b_right && a_right == b_left)
        }
        (Descendant(a_parent, a_child), Descendant(b_parent, b_child)) => {
            normalized_ctx_eq(a_parent, b_parent) && normalized_ctx_eq(a_child, b_child)
        }
        (Not(a_expr), Not(b_expr)) => normalized_ctx_eq(a_expr, b_expr),
        // Handle double negation: !(!a) == a
        (Not(a_expr), b) if matches!(a_expr.as_ref(), Not(_)) => {
            let Not(a_inner) = a_expr.as_ref() else {
                unreachable!();
            };
            normalized_ctx_eq(b, a_inner)
        }
        (a, Not(b_expr)) if matches!(b_expr.as_ref(), Not(_)) => {
            let Not(b_inner) = b_expr.as_ref() else {
                unreachable!();
            };
            normalized_ctx_eq(a, b_inner)
        }
        (And(a_left, a_right), And(b_left, b_right))
            if matches!(a_left.as_ref(), And(_, _))
                || matches!(a_right.as_ref(), And(_, _))
                || matches!(b_left.as_ref(), And(_, _))
                || matches!(b_right.as_ref(), And(_, _)) =>
        {
            let mut a_operands = Vec::new();
            flatten_and(a, &mut a_operands);
            let mut b_operands = Vec::new();
            flatten_and(b, &mut b_operands);
            compare_operand_sets(&a_operands, &b_operands)
        }
        (And(a_left, a_right), And(b_left, b_right)) => {
            (normalized_ctx_eq(a_left, b_left) && normalized_ctx_eq(a_right, b_right))
                || (normalized_ctx_eq(a_left, b_right) && normalized_ctx_eq(a_right, b_left))
        }
        (Or(a_left, a_right), Or(b_left, b_right))
            if matches!(a_left.as_ref(), Or(_, _))
                || matches!(a_right.as_ref(), Or(_, _))
                || matches!(b_left.as_ref(), Or(_, _))
                || matches!(b_right.as_ref(), Or(_, _)) =>
        {
            let mut a_operands = Vec::new();
            flatten_or(a, &mut a_operands);
            let mut b_operands = Vec::new();
            flatten_or(b, &mut b_operands);
            compare_operand_sets(&a_operands, &b_operands)
        }
        (Or(a_left, a_right), Or(b_left, b_right)) => {
            (normalized_ctx_eq(a_left, b_left) && normalized_ctx_eq(a_right, b_right))
                || (normalized_ctx_eq(a_left, b_right) && normalized_ctx_eq(a_right, b_left))
        }
        _ => false,
    };

    fn flatten_and<'a>(
        pred: &'a gpui::KeyBindingContextPredicate,
        operands: &mut Vec<&'a gpui::KeyBindingContextPredicate>,
    ) {
        use gpui::KeyBindingContextPredicate::*;
        match pred {
            And(left, right) => {
                flatten_and(left, operands);
                flatten_and(right, operands);
            }
            _ => operands.push(pred),
        }
    }

    fn flatten_or<'a>(
        pred: &'a gpui::KeyBindingContextPredicate,
        operands: &mut Vec<&'a gpui::KeyBindingContextPredicate>,
    ) {
        use gpui::KeyBindingContextPredicate::*;
        match pred {
            Or(left, right) => {
                flatten_or(left, operands);
                flatten_or(right, operands);
            }
            _ => operands.push(pred),
        }
    }

    fn compare_operand_sets(
        a: &[&gpui::KeyBindingContextPredicate],
        b: &[&gpui::KeyBindingContextPredicate],
    ) -> bool {
        if a.len() != b.len() {
            return false;
        }

        // For each operand in a, find a matching operand in b
        let mut b_matched = vec![false; b.len()];
        for a_operand in a {
            let mut found = false;
            for (b_idx, b_operand) in b.iter().enumerate() {
                if !b_matched[b_idx] && normalized_ctx_eq(a_operand, b_operand) {
                    b_matched[b_idx] = true;
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        true
    }
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
    use db::{query, sqlez::domain::Domain, sqlez_macros::sql};
    use workspace::WorkspaceDb;

    pub struct KeybindingEditorDb(db::sqlez::thread_safe_connection::ThreadSafeConnection);

    impl Domain for KeybindingEditorDb {
        const NAME: &str = stringify!(KeybindingEditorDb);

        const MIGRATIONS: &[&str] = &[sql!(
                CREATE TABLE keybinding_editors (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
        )];
    }

    db::static_connection!(KEYBINDING_EDITORS, KeybindingEditorDb, [WorkspaceDb]);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_ctx_cmp() {
        #[track_caller]
        fn cmp(a: &str, b: &str) -> bool {
            let a = gpui::KeyBindingContextPredicate::parse(a)
                .expect("Failed to parse keybinding context a");
            let b = gpui::KeyBindingContextPredicate::parse(b)
                .expect("Failed to parse keybinding context b");
            normalized_ctx_eq(&a, &b)
        }

        // Basic equality - identical expressions
        assert!(cmp("a && b", "a && b"));
        assert!(cmp("a || b", "a || b"));
        assert!(cmp("a == b", "a == b"));
        assert!(cmp("a != b", "a != b"));
        assert!(cmp("a > b", "a > b"));
        assert!(cmp("!a", "!a"));

        // AND operator - associative/commutative
        assert!(cmp("a && b", "b && a"));
        assert!(cmp("a && b && c", "c && b && a"));
        assert!(cmp("a && b && c", "b && a && c"));
        assert!(cmp("a && b && c && d", "d && c && b && a"));

        // OR operator - associative/commutative
        assert!(cmp("a || b", "b || a"));
        assert!(cmp("a || b || c", "c || b || a"));
        assert!(cmp("a || b || c", "b || a || c"));
        assert!(cmp("a || b || c || d", "d || c || b || a"));

        // Equality operator - associative/commutative
        assert!(cmp("a == b", "b == a"));
        assert!(cmp("x == y", "y == x"));

        // Inequality operator - associative/commutative
        assert!(cmp("a != b", "b != a"));
        assert!(cmp("x != y", "y != x"));

        // Complex nested expressions with associative operators
        assert!(cmp("(a && b) || c", "c || (a && b)"));
        assert!(cmp("(a && b) || c", "c || (b && a)"));
        assert!(cmp("(a || b) && c", "c && (a || b)"));
        assert!(cmp("(a || b) && c", "c && (b || a)"));
        assert!(cmp("(a && b) || (c && d)", "(c && d) || (a && b)"));
        assert!(cmp("(a && b) || (c && d)", "(d && c) || (b && a)"));

        // Multiple levels of nesting
        assert!(cmp("((a && b) || c) && d", "d && ((a && b) || c)"));
        assert!(cmp("((a && b) || c) && d", "d && (c || (b && a))"));
        assert!(cmp("a && (b || (c && d))", "(b || (c && d)) && a"));
        assert!(cmp("a && (b || (c && d))", "(b || (d && c)) && a"));

        // Negation with associative operators
        assert!(cmp("!a && b", "b && !a"));
        assert!(cmp("!a || b", "b || !a"));
        assert!(cmp("!(a && b) || c", "c || !(a && b)"));
        assert!(cmp("!(a && b) || c", "c || !(b && a)"));

        // Descendant operator (>) - NOT associative/commutative
        assert!(cmp("a > b", "a > b"));
        assert!(!cmp("a > b", "b > a"));
        assert!(!cmp("a > b > c", "c > b > a"));
        assert!(!cmp("a > b > c", "a > c > b"));

        // Mixed operators with descendant
        assert!(cmp("(a > b) && c", "c && (a > b)"));
        assert!(!cmp("(a > b) && c", "c && (b > a)"));
        assert!(cmp("(a > b) || (c > d)", "(c > d) || (a > b)"));
        assert!(!cmp("(a > b) || (c > d)", "(b > a) || (d > c)"));

        // Negative cases - different operators
        assert!(!cmp("a && b", "a || b"));
        assert!(!cmp("a == b", "a != b"));
        assert!(!cmp("a && b", "a > b"));
        assert!(!cmp("a || b", "a > b"));
        assert!(!cmp("a == b", "a && b"));
        assert!(!cmp("a != b", "a || b"));

        // Negative cases - different operands
        assert!(!cmp("a && b", "a && c"));
        assert!(!cmp("a && b", "c && d"));
        assert!(!cmp("a || b", "a || c"));
        assert!(!cmp("a || b", "c || d"));
        assert!(!cmp("a == b", "a == c"));
        assert!(!cmp("a != b", "a != c"));
        assert!(!cmp("a > b", "a > c"));
        assert!(!cmp("a > b", "c > b"));

        // Negative cases - with negation
        assert!(!cmp("!a", "a"));
        assert!(!cmp("!a && b", "a && b"));
        assert!(!cmp("!(a && b)", "a && b"));
        assert!(!cmp("!a || b", "a || b"));
        assert!(!cmp("!(a || b)", "a || b"));

        // Negative cases - complex expressions
        assert!(!cmp("(a && b) || c", "(a || b) && c"));
        assert!(!cmp("a && (b || c)", "a || (b && c)"));
        assert!(!cmp("(a && b) || (c && d)", "(a || b) && (c || d)"));
        assert!(!cmp("a > b && c", "a && b > c"));

        // Edge cases - multiple same operands
        assert!(cmp("a && a", "a && a"));
        assert!(cmp("a || a", "a || a"));
        assert!(cmp("a && a && b", "b && a && a"));
        assert!(cmp("a || a || b", "b || a || a"));

        // Edge cases - deeply nested
        assert!(cmp(
            "((a && b) || (c && d)) && ((e || f) && g)",
            "((e || f) && g) && ((c && d) || (a && b))"
        ));
        assert!(cmp(
            "((a && b) || (c && d)) && ((e || f) && g)",
            "(g && (f || e)) && ((d && c) || (b && a))"
        ));

        // Edge cases - repeated patterns
        assert!(cmp("(a && b) || (a && b)", "(b && a) || (b && a)"));
        assert!(cmp("(a || b) && (a || b)", "(b || a) && (b || a)"));

        // Negative cases - subtle differences
        assert!(!cmp("a && b && c", "a && b"));
        assert!(!cmp("a || b || c", "a || b"));
        assert!(!cmp("(a && b) || c", "a && (b || c)"));

        // a > b > c is not the same as a > c, should not be equal
        assert!(!cmp("a > b > c", "a > c"));

        // Double negation with complex expressions
        assert!(cmp("!(!(a && b))", "a && b"));
        assert!(cmp("!(!(a || b))", "a || b"));
        assert!(cmp("!(!(a > b))", "a > b"));
        assert!(cmp("!(!a) && b", "a && b"));
        assert!(cmp("!(!a) || b", "a || b"));
        assert!(cmp("!(!(a && b)) || c", "(a && b) || c"));
        assert!(cmp("!(!(a && b)) || c", "(b && a) || c"));
        assert!(cmp("!(!a)", "a"));
        assert!(cmp("a", "!(!a)"));
        assert!(cmp("!(!(!a))", "!a"));
        assert!(cmp("!(!(!(!a)))", "a"));
    }
}
