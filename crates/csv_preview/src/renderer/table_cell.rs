//! Table Cell Rendering

use gpui::{AnyElement, ClipboardItem, ElementId, MouseButton};
use ui::{Color, Divider, Label, LabelSize, SharedString, Tooltip, div, prelude::*};

use crate::{CsvPreviewView, settings::VerticalAlignment, types::DisplayCellId};

impl CsvPreviewView {
    /// Create selectable table cell with mouse event handlers.
    pub fn create_selectable_cell(
        display_cell_id: DisplayCellId,
        cell_content: SharedString,
        vertical_alignment: VerticalAlignment,
        cx: &Context<CsvPreviewView>,
    ) -> AnyElement {
        create_table_cell(display_cell_id, cell_content, vertical_alignment, cx)
            // Mouse events handlers will be here
            .into_any_element()
    }
}

/// Create styled table cell div element.
fn create_table_cell(
    display_cell_id: DisplayCellId,
    cell_content: SharedString,
    vertical_alignment: VerticalAlignment,
    cx: &Context<'_, CsvPreviewView>,
) -> gpui::Stateful<Div> {
    div()
        .id(ElementId::NamedInteger(
            format!("csv-display-cell-{}", *display_cell_id.row).into(),
            *display_cell_id.col as u64,
        ))
        .flex()
        .h_full()
        .px_1()
        .border_color(cx.theme().colors().border_variant)
        .map(|div| match vertical_alignment {
            VerticalAlignment::Top => div.items_start(),
            VerticalAlignment::Center => div.items_center(),
        })
        .font_buffer(cx)
        .on_mouse_down(MouseButton::Right, {
            let text = cell_content.clone();
            move |_event, _window, cx| {
                cx.stop_propagation();
                cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
            }
        })
        .tooltip(Tooltip::element({
            let text = cell_content.clone();
            move |_window, cx| {
                v_flex()
                    .gap_1()
                    .child(div().font_buffer(cx).child(text.clone()))
                    .child(Divider::horizontal())
                    .child(
                        Label::new("Right click to copy content")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any_element()
            }
        }))
        .child(div().child(cell_content))
}
