use std::{cell::RefCell, fmt::Write as _, rc::Rc};

use db::anyhow::anyhow;
use gpui::{
    AnyElement, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    Global, IntoElement, Keymap, Length, ListHorizontalSizingBehavior, ListSizingBehavior,
    Subscription, Task, actions, div, uniform_list,
};

use ui::{
    ActiveTheme as _, App, BorrowAppContext, Indicator, ParentElement as _, Render, SharedString,
    Styled as _, Window, prelude::*,
};
use workspace::{Item, SerializableItem, Workspace, register_serializable_item};

use crate::keybindings::persistence::KEYBINDING_EDITORS;

actions!(zed, [OpenKeymapEditor]);

pub fn init(cx: &mut App) {
    let keymap_event_channel = KeymapEventChannel::new();
    cx.set_global(keymap_event_channel);

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &OpenKeymapEditor, window, cx| {
            let open_keymap_editor = cx.new(|cx| KeymapEditor::new(cx));
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
            dbg!("updating global");
            *_event_channel = Self::new();
        });
    }
}

struct KeymapEditor {
    focus_handle: FocusHandle,
    _keymap_subscription: Subscription,
    processed_bindings: Vec<ProcessedKeybinding>,
}

impl EventEmitter<()> for KeymapEditor {}

impl Focusable for KeymapEditor {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl KeymapEditor {
    fn new(cx: &mut gpui::Context<Self>) -> Self {
        let _keymap_subscription = cx.observe_global::<KeymapEventChannel>(|this, cx| {
            let key_bindings = Self::process_bindings(cx);
            this.processed_bindings = key_bindings;
        });
        Self {
            focus_handle: cx.focus_handle(),
            _keymap_subscription,
            processed_bindings: vec![],
        }
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
        dbg!("rendering");
        if self.processed_bindings.is_empty() {
            self.processed_bindings = Self::process_bindings(cx);
        }

        let table = Table::new(self.processed_bindings.len());

        let theme = cx.theme();
        let headers = ["Command", "Keystrokes", "Context"].map(Into::into);

        div().size_full().bg(theme.colors().background).child(
            table
                .render()
                .h_full()
                .child(table.render_header(headers, cx))
                .child(
                    uniform_list(
                        cx.entity(),
                        "keybindings",
                        table.row_count,
                        move |this, range, _, cx| {
                            return range
                                .map(|index| {
                                    table.render_row(
                                        index,
                                        [
                                            string_cell(
                                                this.processed_bindings[index].action.clone(),
                                            ),
                                            string_cell(
                                                this.processed_bindings[index]
                                                    .keystroke_text
                                                    .clone(),
                                            ),
                                            string_cell(
                                                this.processed_bindings[index].context.clone(),
                                            ),
                                            // TODO: Add a source field
                                            // string_cell(keybinding.source().to_string()),
                                        ],
                                        cx,
                                    )
                                })
                                .collect();
                        },
                    )
                    .size_full()
                    .flex_grow()
                    .with_sizing_behavior(ListSizingBehavior::Auto)
                    .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained),
                ),
        )
    }
}

/// A table component
#[derive(Clone, Copy)]
pub struct Table<const COLS: usize> {
    striped: bool,
    width: Length,
    row_count: usize,
}

impl<const COLS: usize> Table<COLS> {
    /// Create a new table with a column count equal to the
    /// number of headers provided.
    pub fn new(row_count: usize) -> Self {
        Table {
            striped: false,
            width: Length::Auto,
            row_count,
        }
    }

    /// Enables row striping.
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }

    /// Sets the width of the table.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    fn base_cell_style(cx: &App) -> Div {
        div()
            .px_1p5()
            .flex_1()
            .justify_start()
            .text_ui(cx)
            .whitespace_nowrap()
            .text_ellipsis()
            .overflow_hidden()
    }

    pub fn render_row(&self, row_index: usize, items: [TableCell; COLS], cx: &App) -> AnyElement {
        let is_last = row_index == self.row_count - 1;
        let bg = if row_index % 2 == 1 && self.striped {
            Some(cx.theme().colors().text.opacity(0.05))
        } else {
            None
        };
        div()
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_1p5()
            .py_1()
            .when_some(bg, |row, bg| row.bg(bg))
            .when(!is_last, |row| {
                row.border_b_1().border_color(cx.theme().colors().border)
            })
            .children(items.into_iter().map(|cell| match cell {
                TableCell::String(s) => Self::base_cell_style(cx).child(s),
                TableCell::Element(e) => Self::base_cell_style(cx).child(e),
            }))
            .into_any_element()
    }

    fn render_header(&self, headers: [SharedString; COLS], cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .children(headers.into_iter().map(|h| {
                Self::base_cell_style(cx)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(h.clone())
            }))
    }

    fn render(&self) -> Div {
        div().w(self.width).overflow_hidden()
    }
}

/// Represents a cell in a table.
pub enum TableCell {
    /// A cell containing a string value.
    String(SharedString),
    /// A cell containing a UI element.
    Element(AnyElement),
}

/// Creates a `TableCell` containing a string value.
pub fn string_cell(s: impl Into<SharedString>) -> TableCell {
    TableCell::String(s.into())
}

/// Creates a `TableCell` containing an element.
pub fn element_cell(e: impl Into<AnyElement>) -> TableCell {
    TableCell::Element(e.into())
}

impl<E> From<E> for TableCell
where
    E: Into<SharedString>,
{
    fn from(e: E) -> Self {
        TableCell::String(e.into())
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
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<gpui::Entity<Self>>> {
        cx.spawn(async move |cx| {
            if KEYBINDING_EDITORS
                .get_keybinding_editor(item_id, workspace_id)?
                .is_some()
            {
                cx.new(|cx| KeymapEditor::new(cx))
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
