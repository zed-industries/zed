//! Table Cell Rendering

use gpui::{AnyElement, ClipboardItem, ElementId, MouseButton, StatefulInteractiveElement};
use ui::{Color, Divider, Label, LabelSize, SharedString, Tooltip, div, prelude::*};

use crate::{TabularDataPreviewPane, settings::VerticalAlignment, types::DisplayCellId};

/// Adds a right-click-to-copy handler and a tooltip showing `text` plus `hint`
/// (e.g. "Right click to copy content") to a `Stateful` element.
pub(crate) fn with_copy_on_right_click<E: StatefulInteractiveElement>(
    element: E,
    text: SharedString,
    hint: &'static str,
) -> E {
    element
        .on_mouse_down(MouseButton::Right, {
            let text_to_copy = text.clone();
            move |_event, _window, cx| {
                cx.stop_propagation();
                cx.write_to_clipboard(ClipboardItem::new_string(text_to_copy.to_string()));
            }
        })
        .tooltip(Tooltip::element(move |_window, cx| {
            v_flex()
                .gap_1()
                .child(div().font_buffer(cx).child(text.clone()))
                .child(Divider::horizontal())
                .child(Label::new(hint).size(LabelSize::Small).color(Color::Muted))
                .into_any_element()
        }))
}

impl TabularDataPreviewPane {
    /// Create selectable table cell with mouse event handlers.
    pub fn create_selectable_cell(
        display_cell_id: DisplayCellId,
        cell_content: SharedString,
        vertical_alignment: VerticalAlignment,
        cx: &Context<TabularDataPreviewPane>,
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
    cx: &Context<'_, TabularDataPreviewPane>,
) -> gpui::Stateful<Div> {
    let cell = div()
        .id(ElementId::NamedInteger(
            format!("table-display-cell-{}", *display_cell_id.row).into(),
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
        .font_buffer(cx);
    with_copy_on_right_click(cell, cell_content.clone(), "Right click to copy content")
        .child(div().child(cell_content))
}
