//! Table Cell Rendering
//!
//! Creates interactive cell elements with mouse event handlers for selection.

use gpui::{AnyElement, ElementId, Entity, Hsla, MouseButton};
use ui::{div, prelude::*};

use crate::{
    CsvPreviewView,
    settings::{FontType, VerticalAlignment},
    types::DisplayCellId,
};

/// Colors for cell borders in different selection states.
///
/// This unit struct provides a centralized location for all cell border colors
/// used in the CSV preview table. The colors are designed to be visually distinct
/// and follow common spreadsheet application conventions:
///
/// - **Focus**: Green border for the currently focused cell (keyboard navigation)
/// - **Anchor**: Blue border for the selection anchor (starting point of range selection)
/// - **Focus+Anchor**: Automatically blended color when a cell is both focused and anchor (thicker border)
pub struct CellBorderColors;

impl CellBorderColors {
    /// Red for focused cell only
    pub const FOCUS: Hsla = Hsla {
        h: 0.0, // 0 degrees = pure red
        s: 1.0,
        l: 0.4,
        a: 0.7,
    };

    /// Bright blue for anchor cell only
    pub const ANCHOR: Hsla = Hsla {
        h: 0.6, // 216 degrees = bright blue
        s: 1.0,
        l: 0.5,
        a: 0.7,
    };

    /// Automatically blended color for cells that are both focus and anchor
    pub const FOCUS_ANCHOR: Hsla = Self::blend_colors(Self::FOCUS, Self::ANCHOR);

    /// Blend two HSLA colors at compile time by averaging their components
    const fn blend_colors(color1: Hsla, color2: Hsla) -> Hsla {
        // Handle hue wrapping for proper color wheel blending
        let h1 = color1.h;
        let h2 = color2.h;
        let hue_diff = if (h2 - h1).abs() > 0.5 {
            // Cross the 0/1 boundary - blend through the shorter path
            if h1 > h2 {
                (h1 + h2 + 1.0) / 2.0 % 1.0
            } else {
                (h1 + h2 + 1.0) / 2.0 % 1.0
            }
        } else {
            // Normal blending
            (h1 + h2) / 2.0
        };

        Hsla {
            h: hue_diff,
            s: (color1.s + color2.s) / 2.0,
            l: (color1.l + color2.l) / 2.0,
            a: (color1.a + color2.a) / 2.0,
        }
    }
}

impl CsvPreviewView {
    /// Create selectable table cell with mouse event handlers.
    pub fn create_selectable_cell(
        display_cell_id: DisplayCellId,
        cell_content: impl IntoElement,
        view_entity: Entity<CsvPreviewView>,
        selected_bg_color: gpui::Hsla,
        is_selected: bool,
        is_focused: bool,
        is_anchor: bool,
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
            is_anchor,
            vertical_alignment,
            font_type,
            cx,
        )
        // Called when user presses mouse button down on a cell
        .on_mouse_down(MouseButton::Left, {
            let view = view_entity.clone();
            move |_event, window, cx| {
                view.update(cx, |this, cx| {
                    let ordered_indices = this.get_ordered_indices().clone();
                    let preserve_existing = window.modifiers().secondary(); // cmd/ctrl key
                    this.selection.start_selection_with_cumulative(
                        display_cell_id.row,
                        display_cell_id.col,
                        &ordered_indices,
                        preserve_existing,
                    );
                    cx.notify();
                });
            }
        })
        // Called when user moves mouse over a cell (for drag selection)
        .on_mouse_move({
            let view = view_entity.clone();
            move |_event, window, cx| {
                view.update(cx, |this, cx| {
                    if !this.selection.is_selecting() {
                        return;
                    }
                    let ordered_indices = this.get_ordered_indices().clone();
                    let preserve_existing = window.modifiers().secondary(); // cmd/ctrl key
                    this.selection.extend_selection_to(
                        display_cell_id.row,
                        display_cell_id.col,
                        &ordered_indices,
                        preserve_existing,
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
    is_anchor: bool,
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
        .when(is_focused && is_anchor, |div| {
            div.border_1().border_color(CellBorderColors::FOCUS_ANCHOR) // Focus + Anchor (blended color)
        })
        .when(is_focused && !is_anchor, |div| {
            div.border_1().border_color(CellBorderColors::FOCUS) // Focus only
        })
        .when(is_anchor && !is_focused, |div| {
            div.border_1().border_color(CellBorderColors::ANCHOR) // Anchor only
        })
        .map(|div| match font_type {
            FontType::Ui => div.font_ui(cx),
            FontType::Monospace => div.font_buffer(cx),
        })
        .child(cell_content)
}
