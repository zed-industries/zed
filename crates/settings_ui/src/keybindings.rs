use std::{fmt::Write as _, ops::Range, sync::Arc};

use db::anyhow::anyhow;
use editor::{Editor, EditorEvent};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable, Global, ScrollStrategy,
    Subscription, actions, div,
};

use ui::{
    ActiveTheme as _, App, BorrowAppContext, ParentElement as _, Render, SharedString, Styled as _,
    Window, prelude::*,
};
use workspace::{Item, SerializableItem, Workspace, register_serializable_item};

use crate::{
    keybindings::persistence::KEYBINDING_EDITORS,
    ui_components::table::{Table, TableInteractionState},
};

actions!(zed, [OpenKeymapEditor]);

pub fn init(cx: &mut App) {
    let keymap_event_channel = KeymapEventChannel::new();
    cx.set_global(keymap_event_channel);

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &OpenKeymapEditor, window, cx| {
            let open_keymap_editor = cx.new(|cx| KeymapEditor::new(window, cx));
            workspace.add_item_to_center(Box::new(open_keymap_editor), window, cx);
        });
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
        cx.update_global(|_event_channel: &mut Self, _| {
            /* triggers observers in KeymapEditors */
        });
    }
}

struct KeymapEditor {
    focus_handle: FocusHandle,
    _keymap_subscription: Subscription,
    keybindings: Vec<ProcessedKeybinding>,
    // corresponds 1 to 1 with keybindings
    string_match_candidates: Arc<Vec<StringMatchCandidate>>,
    matches: Vec<StringMatch>,
    table_interaction_state: Entity<TableInteractionState>,
    filter_editor: Entity<Editor>,
}

impl EventEmitter<()> for KeymapEditor {}

impl Focusable for KeymapEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        return self.filter_editor.focus_handle(cx);
    }
}

impl KeymapEditor {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let _keymap_subscription =
            cx.observe_global::<KeymapEventChannel>(Self::update_keybindings);
        let table_interaction_state = TableInteractionState::new(window, cx);

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Filter action names...", cx);
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
            keybindings: vec![],
            string_match_candidates: Arc::new(vec![]),
            matches: vec![],
            focus_handle: focus_handle.clone(),
            _keymap_subscription,
            table_interaction_state,
            filter_editor,
        };

        this.update_keybindings(cx);

        this
    }

    fn update_matches(&mut self, cx: &mut Context<Self>) {
        let query = self.filter_editor.read(cx).text(cx);
        let string_match_candidates = self.string_match_candidates.clone();
        let executor = cx.background_executor().clone();
        let keybind_count = self.keybindings.len();
        let query = command_palette::normalize_action_query(&query);
        let fuzzy_match = cx.background_spawn(async move {
            fuzzy::match_strings(
                &string_match_candidates,
                &query,
                true,
                true,
                keybind_count,
                &Default::default(),
                executor,
            )
            .await
        });

        cx.spawn(async move |this, cx| {
            let matches = fuzzy_match.await;
            this.update(cx, |this, cx| {
                this.table_interaction_state.update(cx, |this, _cx| {
                    this.scroll_handle.scroll_to_item(0, ScrollStrategy::Top);
                });
                this.matches = matches;
                cx.notify();
            })
        })
        .detach();
    }

    fn process_bindings(
        cx: &mut Context<Self>,
    ) -> (Vec<ProcessedKeybinding>, Vec<StringMatchCandidate>) {
        let key_bindings_ptr = cx.key_bindings();
        let lock = key_bindings_ptr.borrow();
        let key_bindings = lock.bindings();

        let mut processed_bindings = Vec::new();
        let mut string_match_candidates = Vec::new();

        for key_binding in key_bindings {
            let mut keystroke_text = String::new();
            for keystroke in key_binding.keystrokes() {
                write!(&mut keystroke_text, "{} ", keystroke.unparse()).ok();
            }
            let keystroke_text = keystroke_text.trim().to_string();

            let context = key_binding
                .predicate()
                .map(|predicate| predicate.to_string())
                .unwrap_or_else(|| "<global>".to_string());

            let source = key_binding
                .meta()
                .map(|meta| settings::KeybindSource::from_meta(meta).name().into());

            let action_name = key_binding.action().name();

            let index = processed_bindings.len();
            let string_match_candidate = StringMatchCandidate::new(index, &action_name);
            processed_bindings.push(ProcessedKeybinding {
                keystroke_text: keystroke_text.into(),
                action: action_name.into(),
                context: context.into(),
                source,
            });
            string_match_candidates.push(string_match_candidate);
        }
        (processed_bindings, string_match_candidates)
    }

    fn update_keybindings(self: &mut KeymapEditor, cx: &mut Context<KeymapEditor>) {
        let (key_bindings, string_match_candidates) = Self::process_bindings(cx);
        self.keybindings = key_bindings;
        self.string_match_candidates = Arc::new(string_match_candidates);
        self.matches = self
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

        self.update_matches(cx);
        cx.notify();
    }
}

#[derive(Clone)]
struct ProcessedKeybinding {
    keystroke_text: SharedString,
    action: SharedString,
    context: SharedString,
    source: Option<SharedString>,
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

        div()
            .size_full()
            .bg(theme.colors().background)
            .id("keymap-editor")
            .track_focus(&self.focus_handle)
            .px_4()
            .v_flex()
            .pb_4()
            .child(
                h_flex()
                    .w_full()
                    .h_12()
                    .px_4()
                    .my_4()
                    .border_2()
                    .border_color(theme.colors().border)
                    .child(self.filter_editor.clone()),
            )
            .child(
                Table::new()
                    .interactable(&self.table_interaction_state)
                    .striped()
                    .column_widths([rems(24.), rems(16.), rems(32.), rems(8.)])
                    .header(["Command", "Keystrokes", "Context", "Source"])
                    .uniform_list(
                        "keymap-editor-table",
                        row_count,
                        cx.processor(move |this, range: Range<usize>, _window, _cx| {
                            range
                                .filter_map(|index| {
                                    let candidate_id = this.matches.get(index)?.candidate_id;
                                    let binding = &this.keybindings[candidate_id];
                                    Some(
                                        [
                                            binding.action.clone(),
                                            binding.keystroke_text.clone(),
                                            binding.context.clone(),
                                            binding.source.clone().unwrap_or_default(),
                                        ]
                                        .map(IntoElement::into_any_element),
                                    )
                                })
                                .collect()
                        }),
                    ),
            )
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
        _project: gpui::Entity<project::Project>,
        _workspace: gpui::WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<gpui::Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            if KEYBINDING_EDITORS
                .get_keybinding_editor(item_id, workspace_id)?
                .is_some()
            {
                cx.update(|window, cx| cx.new(|cx| KeymapEditor::new(window, cx)))
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
