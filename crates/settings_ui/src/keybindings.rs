use std::{cell::RefCell, fmt::Write as _, rc::Rc};

use db::anyhow::anyhow;
use gpui::{
    AnyElement, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    Global, IntoElement, Keymap, Length, Subscription, Task, actions, div,
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

impl EventEmitter<KeymapEvent> for KeymapEventChannel {}
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

        let mut table = Table::new(vec!["Command", "Keystrokes", "Context"]);
        for key_binding in &self.processed_bindings {
            table = table.row(vec![
                string_cell(key_binding.action.clone()),
                string_cell(key_binding.keystroke_text.clone()),
                string_cell(key_binding.context.clone()),
                // TODO: Add a source field
                // string_cell(keybinding.source().to_string()),
            ]);
        }

        let theme = cx.theme();

        div()
            .size_full()
            .bg(theme.colors().background)
            .child(table.striped())
    }
}

/// A table component
#[derive(IntoElement)]
pub struct Table {
    column_headers: Vec<SharedString>,
    rows: Vec<Vec<TableCell>>,
    column_count: usize,
    striped: bool,
    width: Length,
}

impl Table {
    /// Create a new table with a column count equal to the
    /// number of headers provided.
    pub fn new(headers: Vec<impl Into<SharedString>>) -> Self {
        let column_count = headers.len();

        Table {
            column_headers: headers.into_iter().map(Into::into).collect(),
            column_count,
            rows: Vec::new(),
            striped: false,
            width: Length::Auto,
        }
    }

    /// Adds a row to the table.
    ///
    /// The row must have the same number of columns as the table.
    pub fn row(mut self, items: Vec<impl Into<TableCell>>) -> Self {
        if items.len() == self.column_count {
            self.rows.push(items.into_iter().map(Into::into).collect());
        } else {
            log::error!("Row length mismatch");
        }
        self
    }

    /// Adds multiple rows to the table.
    ///
    /// Each row must have the same number of columns as the table.
    /// Rows that don't match the column count are ignored.
    pub fn rows(mut self, rows: Vec<Vec<impl Into<TableCell>>>) -> Self {
        for row in rows {
            self = self.row(row);
        }
        self
    }

    fn base_cell_style(cx: &mut App) -> Div {
        div()
            .px_1p5()
            .flex_1()
            .justify_start()
            .text_ui(cx)
            .whitespace_nowrap()
            .text_ellipsis()
            .overflow_hidden()
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
}

impl RenderOnce for Table {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .children(self.column_headers.into_iter().map(|h| {
                Table::base_cell_style(cx)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(h.clone())
            }));

        let row_count = self.rows.len();
        let rows = self.rows.into_iter().enumerate().map(|(ix, row)| {
            let is_last = ix == row_count - 1;
            let bg = if ix % 2 == 1 && self.striped {
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
                .children(row.into_iter().map(|cell| match cell {
                    TableCell::String(s) => Self::base_cell_style(cx).child(s),
                    TableCell::Element(e) => Self::base_cell_style(cx).child(e),
                }))
        });

        div()
            .w(self.width)
            .overflow_hidden()
            .child(header)
            .children(rows)
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
