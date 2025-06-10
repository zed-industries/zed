use std::{fmt::Write as _, time::Duration};

use db::anyhow::anyhow;
use gpui::{
    AnyElement, AppContext as _, Axis, Context, Entity, EventEmitter, FocusHandle, Focusable,
    FontWeight, Global, IntoElement, Length, ListHorizontalSizingBehavior, ListSizingBehavior,
    MouseButton, Subscription, Task, UniformListScrollHandle, actions, div, px, uniform_list,
};

use editor::{EditorSettings, ShowScrollbar, scroll::ScrollbarAutoHide};
use settings::Settings as _;
use ui::{
    ActiveTheme as _, App, BorrowAppContext, ParentElement as _, Render, Scrollbar, ScrollbarState,
    SharedString, Styled as _, Window, prelude::*,
};
use workspace::{Item, SerializableItem, Workspace, register_serializable_item};

use crate::keybindings::persistence::KEYBINDING_EDITORS;

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
    scroll_handle: UniformListScrollHandle,
    horizontal_scrollbar: ScrollbarProperties,
    vertical_scrollbar: ScrollbarProperties,
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

            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                this.hide_scrollbars(window, cx);
            })
            .detach();

            let scroll_handle = UniformListScrollHandle::new();
            let vertical_scrollbar = ScrollbarProperties {
                axis: Axis::Vertical,
                state: ScrollbarState::new(scroll_handle.clone()).parent_entity(&cx.entity()),
                show_scrollbar: false,
                show_track: false,
                auto_hide: false,
                hide_task: None,
            };

            let horizontal_scrollbar = ScrollbarProperties {
                axis: Axis::Horizontal,
                state: ScrollbarState::new(scroll_handle.clone()).parent_entity(&cx.entity()),
                show_scrollbar: false,
                show_track: false,
                auto_hide: false,
                hide_task: None,
            };

            let mut this = Self {
                focus_handle: focus_handle.clone(),
                _keymap_subscription,
                processed_bindings: vec![],
                scroll_handle,
                horizontal_scrollbar,
                vertical_scrollbar,
            };

            this.update_scrollbar_visibility(cx);
            this
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

    fn update_scrollbar_visibility(&mut self, cx: &mut Context<Self>) {
        let show_setting = EditorSettings::get_global(cx).scrollbar.show;

        let scroll_handle = self.scroll_handle.0.borrow();

        let autohide = |show: ShowScrollbar, cx: &mut Context<Self>| match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => cx
                .try_global::<ScrollbarAutoHide>()
                .map_or_else(|| cx.should_auto_hide_scrollbars(), |autohide| autohide.0),
            ShowScrollbar::Always => false,
            ShowScrollbar::Never => false,
        };

        let longest_item_width = scroll_handle.last_item_size.and_then(|size| {
            (size.contents.width > size.item.width).then_some(size.contents.width)
        });

        // is there an item long enough that we should show a horizontal scrollbar?
        let item_wider_than_container = if let Some(longest_item_width) = longest_item_width {
            longest_item_width > px(scroll_handle.base_handle.bounds().size.width.0)
        } else {
            true
        };

        let show_scrollbar = match show_setting {
            ShowScrollbar::Auto | ShowScrollbar::System | ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        };
        let show_vertical = show_scrollbar;

        let show_horizontal = item_wider_than_container && show_scrollbar;

        let show_horizontal_track =
            show_horizontal && matches!(show_setting, ShowScrollbar::Always);

        // TODO: we probably should hide the scroll track when the list doesn't need to scroll
        let show_vertical_track = show_vertical && matches!(show_setting, ShowScrollbar::Always);

        self.vertical_scrollbar = ScrollbarProperties {
            axis: self.vertical_scrollbar.axis,
            state: self.vertical_scrollbar.state.clone(),
            show_scrollbar: show_vertical,
            show_track: show_vertical_track,
            auto_hide: autohide(show_setting, cx),
            hide_task: None,
        };

        self.horizontal_scrollbar = ScrollbarProperties {
            axis: self.horizontal_scrollbar.axis,
            state: self.horizontal_scrollbar.state.clone(),
            show_scrollbar: show_horizontal,
            show_track: show_horizontal_track,
            auto_hide: autohide(show_setting, cx),
            hide_task: None,
        };

        cx.notify();
    }

    fn hide_scrollbars(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.horizontal_scrollbar.hide(window, cx);
        self.vertical_scrollbar.hide(window, cx);
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("keymap-editor-vertical-scroll")
            .occlude()
            .flex_none()
            .h_full()
            .cursor_default()
            .absolute()
            .right_0()
            .top_0()
            .bottom_0()
            .w(px(12.))
            .on_mouse_move(cx.listener(|_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    if !this.vertical_scrollbar.state.is_dragging()
                        && !this.focus_handle.contains_focused(window, cx)
                    {
                        this.vertical_scrollbar.hide(window, cx);
                        cx.notify();
                    }

                    cx.stop_propagation();
                }),
            )
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                cx.notify();
            }))
            .children(Scrollbar::vertical(self.vertical_scrollbar.state.clone()))
    }

    /// Renders the horizontal scrollbar.
    ///
    /// The right offset is used to determine how far to the right the
    /// scrollbar should extend to, useful for ensuring it doesn't collide
    /// with the vertical scrollbar when visible.
    fn render_horizontal_scrollbar(
        &self,
        right_offset: Pixels,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("keymap-editor-horizontal-scroll")
            .occlude()
            .flex_none()
            .w_full()
            .cursor_default()
            .absolute()
            .bottom_neg_px()
            .left_0()
            .right_0()
            .pr(right_offset)
            .on_mouse_move(cx.listener(|_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    if !this.horizontal_scrollbar.state.is_dragging()
                        && !this.focus_handle.contains_focused(window, cx)
                    {
                        this.horizontal_scrollbar.hide(window, cx);
                        cx.notify();
                    }

                    cx.stop_propagation();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                cx.notify();
            }))
            .children(Scrollbar::horizontal(
                // percentage as f32..end_offset as f32,
                self.horizontal_scrollbar.state.clone(),
            ))
    }
}

// computed state related to how to render scrollbars
// one per axis
// on render we just read this off the keymap editor
// we update it when
// - settings change
// - on focus in, on focus out, on hover, etc.
#[derive(Debug)]
struct ScrollbarProperties {
    axis: Axis,
    show_scrollbar: bool,
    show_track: bool,
    auto_hide: bool,
    hide_task: Option<Task<()>>,
    state: ScrollbarState,
}

impl ScrollbarProperties {
    // Shows the scrollbar and cancels any pending hide task
    fn show(&mut self, cx: &mut Context<KeymapEditor>) {
        if !self.auto_hide {
            return;
        }
        self.show_scrollbar = true;
        self.hide_task.take();
        cx.notify();
    }

    fn hide(&mut self, window: &mut Window, cx: &mut Context<KeymapEditor>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);

        if !self.auto_hide {
            return;
        }

        let axis = self.axis;
        self.hide_task = Some(cx.spawn_in(window, async move |keymap_editor, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;

            if let Some(keymap_editor) = keymap_editor.upgrade() {
                keymap_editor
                    .update(cx, |keymap_editor, cx| {
                        match axis {
                            Axis::Vertical => {
                                keymap_editor.vertical_scrollbar.show_scrollbar = false
                            }
                            Axis::Horizontal => {
                                keymap_editor.horizontal_scrollbar.show_scrollbar = false
                            }
                        }
                        cx.notify();
                    })
                    .ok();
            }
        }));
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

        let scroll_track_size = px(16.);
        let h_scroll_offset = if self.vertical_scrollbar.show_scrollbar {
            // magic number
            px(3.)
        } else {
            px(0.)
        };

        let table = Table::new(self.processed_bindings.len());

        let theme = cx.theme();
        let headers = ["Command", "Keystrokes", "Context"].map(Into::into);

        div()
            .size_full()
            .bg(theme.colors().background)
            .id("keymap-editor")
            .track_focus(&self.focus_handle)
            .on_hover(cx.listener(move |this, hovered, window, cx| {
                if *hovered {
                    this.horizontal_scrollbar.show(cx);
                    this.vertical_scrollbar.show(cx);
                    cx.notify();
                } else if !this.focus_handle.contains_focused(window, cx) {
                    this.hide_scrollbars(window, cx);
                }
            }))
            .child(
                table
                    .render()
                    .h_full()
                    .v_flex()
                    .child(table.render_header(headers, cx))
                    .child(
                        div()
                            .flex_grow()
                            .w_full()
                            .relative()
                            .overflow_hidden()
                            .child(
                                uniform_list(
                                    cx.entity(),
                                    "keybindings",
                                    table.row_count,
                                    move |this, range, _, cx| {
                                        range
                                            .map(|index| {
                                                let binding = &this.processed_bindings[index];
                                                let row = [
                                                    binding.action.clone(),
                                                    binding.keystroke_text.clone(),
                                                    binding.context.clone(),
                                                    // TODO: Add a source field
                                                    // string_cell(keybinding.source().to_string()),
                                                ]
                                                .map(string_cell);

                                                table.render_row(index, row, cx)
                                            })
                                            .collect()
                                    },
                                )
                                .size_full()
                                .flex_grow()
                                .track_scroll(self.scroll_handle.clone())
                                .with_sizing_behavior(ListSizingBehavior::Auto)
                                .with_horizontal_sizing_behavior(
                                    ListHorizontalSizingBehavior::Unconstrained,
                                ),
                            )
                            .when(self.vertical_scrollbar.show_track, |this| {
                                this.child(
                                    v_flex()
                                        .h_full()
                                        .flex_none()
                                        .w(scroll_track_size)
                                        .bg(cx.theme().colors().background)
                                        .child(
                                            div()
                                                .size_full()
                                                .flex_1()
                                                .border_l_1()
                                                .border_color(cx.theme().colors().border),
                                        ),
                                )
                            })
                            .when(self.vertical_scrollbar.show_scrollbar, |this| {
                                this.child(self.render_vertical_scrollbar(cx))
                            }),
                    )
                    .when(self.horizontal_scrollbar.show_track, |this| {
                        this.child(
                            h_flex()
                                .w_full()
                                .h(scroll_track_size)
                                .flex_none()
                                .relative()
                                .child(
                                    div()
                                        .w_full()
                                        .flex_1()
                                        // for some reason the horizontal scrollbar is 1px
                                        // taller than the vertical scrollbar??
                                        .h(scroll_track_size - px(1.))
                                        .bg(cx.theme().colors().background)
                                        .border_t_1()
                                        .border_color(cx.theme().colors().border),
                                )
                                .when(self.vertical_scrollbar.show_track, |this| {
                                    this.child(
                                        div()
                                            .flex_none()
                                            // -1px prevents a missing pixel between the two container borders
                                            .w(scroll_track_size - px(1.))
                                            .h_full(),
                                    )
                                    .child(
                                        // HACK: Fill the missing 1px 🥲
                                        div()
                                            .absolute()
                                            .right(scroll_track_size - px(1.))
                                            .bottom(scroll_track_size - px(1.))
                                            .size_px()
                                            .bg(cx.theme().colors().border),
                                    )
                                }),
                        )
                    })
                    .when(self.horizontal_scrollbar.show_scrollbar, |this| {
                        this.child(self.render_horizontal_scrollbar(h_scroll_offset, cx))
                    }),
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
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<gpui::Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            if KEYBINDING_EDITORS
                .get_keybinding_editor(item_id, workspace_id)?
                .is_some()
            {
                cx.update(|window, cx| KeymapEditor::new(window, cx))
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
