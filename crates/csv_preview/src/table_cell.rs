//! Table Cell Rendering
//!
//! Creates interactive cell elements with mouse event handlers for selection.

use gpui::{AnyElement, ElementId, Entity, MouseButton};
use ui::{div, prelude::*};

use crate::{
    CsvPreviewView,
    data_ordering::generate_ordered_indices,
    settings::{FontType, VerticalAlignment},
    types::DisplayCellId,
};

impl CsvPreviewView {
    /// Create selectable table cell with mouse event handlers.
    pub fn create_selectable_cell(
        display_cell_id: DisplayCellId,
        cell_content: impl IntoElement,
        view_entity: Entity<CsvPreviewView>,
        selected_bg_color: gpui::Hsla,
        is_selected: bool,
        is_focused: bool,
        vertical_alignment: VerticalAlignment,
        font_type: FontType,
        cx: &Context<CsvPreviewView>,
    ) -> AnyElement {
        create_table_cell(
            display_cell_id,
            cell_content,
            selected_bg_color,
            is_selected,
            is_focused,
            vertical_alignment,
            font_type,
            cx,
        )
        // Called when user presses mouse button down on a cell
        .on_mouse_down(MouseButton::Left, {
            let view = view_entity.clone();
            move |_event, _window, cx| {
                view.update(cx, |this, cx| {
                    this.selection.start_selection(
                        display_cell_id.row,
                        display_cell_id.col,
                        &generate_ordered_indices(this.ordering, &this.contents),
                    );
                    cx.notify();
                });
            }
        })
        // Called when user moves mouse over a cell (for drag selection)
        .on_mouse_move({
            let view = view_entity.clone();
            move |_event, _window, cx| {
                view.update(cx, |this, cx| {
                    if !this.selection.is_selecting() {
                        return;
                    }
                    this.selection.extend_selection_to(
                        display_cell_id.row,
                        display_cell_id.col,
                        &generate_ordered_indices(this.ordering, &this.contents),
                    );
                    cx.notify();
                });
            }
        })
        // Called when user releases mouse button
        .on_mouse_up(MouseButton::Left, {
            let view = view_entity;
            move |_event, _window, cx| {
                view.update(cx, |this, cx| {
                    this.selection.end_selection();
                    cx.notify();
                });
            }
        })
        .into_any_element()
    }
}

/// Create styled table cell div element.
fn create_table_cell(
    display_cell_id: DisplayCellId,
    cell_content: impl IntoElement,
    selected_bg_color: gpui::Hsla,
    is_selected: bool,
    is_focused: bool,
    vertical_alignment: VerticalAlignment,
    font_type: FontType,
    cx: &Context<'_, CsvPreviewView>,
) -> gpui::Stateful<Div> {
    div()
        .id(ElementId::NamedInteger(
            format!(
                "csv-display-cell-{}-{}",
                display_cell_id.row.get(),
                display_cell_id.col
            )
            .into(),
            0,
        ))
        .cursor_pointer()
        .flex()
        .h_full()
        .map(|div| match vertical_alignment {
            VerticalAlignment::Top => div.items_start(),
            VerticalAlignment::Center => div.items_center(),
        })
        .map(|div| match vertical_alignment {
            VerticalAlignment::Top => div.content_start(),
            VerticalAlignment::Center => div.content_center(),
        })
        .when(is_selected, |div| div.bg(selected_bg_color))
        .when(is_focused, |div| div.border_1().border_color(gpui::green()))
        .map(|div| match font_type {
            FontType::Ui => div.font_ui(cx),
            FontType::Monospace => div.font_buffer(cx),
        })
        .child(cell_content)
}
