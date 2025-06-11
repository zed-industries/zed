use std::{fmt::Write as _, ops::Range};

use db::anyhow::anyhow;
use gpui::{
    AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable, Global, Subscription,
    actions, div,
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
            let open_keymap_editor = KeymapEditor::new(window, cx);
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
    processed_bindings: Vec<ProcessedKeybinding>,
    table_interaction_state: Entity<TableInteractionState>,
}

impl EventEmitter<()> for KeymapEditor {}

impl Focusable for KeymapEditor {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl KeymapEditor {
    fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let this = cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            let _keymap_subscription = cx.observe_global::<KeymapEventChannel>(|this, cx| {
                let key_bindings = Self::process_bindings(cx);
                this.processed_bindings = key_bindings;
            });

            let table_interaction_state = TableInteractionState::new(window, cx);

            Self {
                focus_handle: focus_handle.clone(),
                _keymap_subscription,
                processed_bindings: vec![],
                table_interaction_state,
            }
        });
        this
    }

    fn process_bindings(cx: &mut Context<Self>) -> Vec<ProcessedKeybinding> {
        let key_bindings_ptr = cx.key_bindings();
        let lock = key_bindings_ptr.borrow();
        let key_bindings = lock.bindings();

        let mut processed_bindings = Vec::new();

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

            processed_bindings.push(ProcessedKeybinding {
                keystroke_text: keystroke_text.into(),
                action: key_binding.action().name().into(),
                context: context.into(),
            })
        }
        processed_bindings
    }
}

struct ProcessedKeybinding {
    keystroke_text: SharedString,
    action: SharedString,
    context: SharedString,
}

impl Item for KeymapEditor {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> ui::SharedString {
        "Keymap Editor".into()
    }
}

impl Render for KeymapEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut ui::Context<Self>) -> impl ui::IntoElement {
        if self.processed_bindings.is_empty() {
            self.processed_bindings = Self::process_bindings(cx);
        }

        let row_count = self.processed_bindings.len();

        let theme = cx.theme();

        div()
            .size_full()
            .bg(theme.colors().background)
            .id("keymap-editor")
            .track_focus(&self.focus_handle)
            .child(
                Table::new()
                    .interactable(&self.table_interaction_state)
                    .header(["Command", "Keystrokes", "Context"])
                    .uniform_list(
                        "keymap-editor-table",
                        row_count,
                        cx.processor(move |this, range: Range<usize>, _window, cx| {
                            range
                                .map(|index| {
                                    let binding = &this.processed_bindings[index];
                                    let row = [
                                        binding.action.clone(),
                                        binding.keystroke_text.clone(),
                                        binding.context.clone(),
                                        // TODO: Add a source field
                                        // binding.source.clone(),
                                    ];

                                    // fixme: pass through callback as a row_cx param
                                    let striped = false;

                                    crate::ui_components::table::render_row(
                                        index, row, row_count, striped, cx,
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
                cx.update(KeymapEditor::new)
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
        let Some(workspace_id) = workspace.database_id() else {
            return None;
        };
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
