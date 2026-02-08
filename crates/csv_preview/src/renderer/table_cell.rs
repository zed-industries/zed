//! Table Cell Rendering

use gpui::{AnyElement, ElementId};
use ui::{SharedString, Tooltip, div, prelude::*};

use crate::{
    CsvPreviewView,
    settings::{FontType, VerticalAlignment},
    types::DisplayCellId,
};

impl CsvPreviewView {
    /// Create selectable table cell with mouse event handlers.
    pub fn create_selectable_cell(
        display_cell_id: DisplayCellId,
        cell_content: SharedString,
        vertical_alignment: VerticalAlignment,
        font_type: FontType,
        cx: &Context<CsvPreviewView>,
    ) -> AnyElement {
        create_table_cell(
            display_cell_id,
            cell_content,
            vertical_alignment,
            font_type,
            cx,
        )
        // Mouse events handlers will be here
        .into_any_element()
    }
}

/// Create styled table cell div element.
fn create_table_cell(
    display_cell_id: DisplayCellId,
    cell_content: SharedString,
    vertical_alignment: VerticalAlignment,
    font_type: FontType,
    cx: &Context<'_, CsvPreviewView>,
) -> gpui::Stateful<Div> {
    div()
        .id(ElementId::NamedInteger(
            format!(
                "csv-display-cell-{}-{}",
                *display_cell_id.row, *display_cell_id.col
            )
            .into(),
            0,
        ))
        .cursor_pointer()
        .flex()
        .h_full()
        .px_1()
        .bg(cx.theme().colors().editor_background)
        .border_b_1()
        .border_r_1()
        .border_color(cx.theme().colors().border_variant)
        .map(|div| match vertical_alignment {
            VerticalAlignment::Top => div.items_start(),
            VerticalAlignment::Center => div.items_center(),
        })
        .map(|div| match vertical_alignment {
            VerticalAlignment::Top => div.content_start(),
            VerticalAlignment::Center => div.content_center(),
        })
        .map(|div| match font_type {
            FontType::Ui => div.font_ui(cx),
            FontType::Monospace => div.font_buffer(cx),
        })
        .tooltip(Tooltip::text(cell_content.clone()))
        .child(div().child(cell_content))
}
