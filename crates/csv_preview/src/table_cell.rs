//! Table Cell Rendering and Position Tracking
//!
//! Creates interactive cell elements with mouse event handlers for selection.
//! Also defines cell position tracking for span-based parsing.

use std::time::Instant;

use gpui::{AnyElement, ElementId, Entity, Hsla, MouseButton};
use text::Anchor;
use ui::{SharedString, Tooltip, div, prelude::*};

use crate::{
    CsvPreviewView,
    settings::{FontType, VerticalAlignment},
    table_data_engine::{
        selection::ScrollOffset,
        sorting_by_column::{AppliedSorting, SortDirection},
    },
    types::{AnyColumn, DisplayCellId},
};

/// Position of a cell within the source CSV buffer
#[derive(Clone, Debug)]
pub struct CellContentSpan {
    /// Start anchor of the cell content in the source buffer
    pub start: Anchor,
    /// End anchor of the cell content in the source buffer
    pub end: Anchor,
}

/// A table cell with its content and position in the source buffer
#[derive(Clone, Debug)]
pub enum TableCell {
    /// Cell existing in the CSV
    Real {
        /// Position of this cell in the source buffer
        position: CellContentSpan,
        /// Cached display value (for performance)
        cached_value: SharedString,
    },
    /// Virtual cell, created to pad malformed row
    Virtual,
}

impl TableCell {
    /// Create a TableCell with buffer position tracking
    pub fn from_buffer_position(
        content: SharedString,
        start_offset: usize,
        end_offset: usize,
        buffer_snapshot: &text::BufferSnapshot,
    ) -> Self {
        let start_anchor = buffer_snapshot.anchor_before(start_offset);
        let end_anchor = buffer_snapshot.anchor_after(end_offset);

        Self::Real {
            position: CellContentSpan {
                start: start_anchor,
                end: end_anchor,
            },
            cached_value: content,
        }
    }

    /// Get the display value for this cell
    pub fn display_value(&self) -> Option<&SharedString> {
        match self {
            TableCell::Real { cached_value, .. } => Some(cached_value),
            TableCell::Virtual => None,
        }
    }

    pub(crate) fn position(&self) -> Option<&CellContentSpan> {
        match self {
            TableCell::Real { position, .. } => Some(position),
            TableCell::Virtual => None,
        }
    }
}

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
        s: 0.2,
        l: 0.4,
        a: 0.7,
    };

    /// Bright blue for anchor cell only
    pub const ANCHOR: Hsla = Hsla {
        h: 0.6, // 216 degrees = bright blue
        s: 0.1,
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
    pub(crate) fn create_sort_button(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: AnyColumn,
    ) -> Button {
        let sort_btn = Button::new(
            ElementId::NamedInteger("sort-button".into(), col_idx.get() as u64),
            match self.engine.applied_sorting {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    SortDirection::Asc => "↓",
                    SortDirection::Desc => "↑",
                },
                _ => "↕", // Unsorted/available for sorting
            },
        )
        .size(ButtonSize::Compact)
        .style(
            if self
                .engine
                .applied_sorting
                .is_some_and(|o| o.col_idx == col_idx)
            {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Subtle
            },
        )
        .tooltip(Tooltip::text(match self.engine.applied_sorting {
            Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                SortDirection::Asc => "Sorted A-Z. Click to sort Z-A",
                SortDirection::Desc => "Sorted Z-A. Click to disable sorting",
            },
            _ => "Not sorted. Click to sort A-Z",
        }))
        .on_click(cx.listener(move |this, _event, _window, cx| {
            let new_sorting = match this.engine.applied_sorting {
                Some(ordering) if ordering.col_idx == col_idx => {
                    // Same column clicked - cycle through states
                    match ordering.direction {
                        SortDirection::Asc => Some(AppliedSorting {
                            col_idx,
                            direction: SortDirection::Desc,
                        }),
                        SortDirection::Desc => None, // Clear sorting
                    }
                }
                _ => {
                    // Different column or no sorting - start with ascending
                    Some(AppliedSorting {
                        col_idx,
                        direction: SortDirection::Asc,
                    })
                }
            };

            this.engine.applied_sorting = new_sorting;
            this.re_sort_indices();
            cx.notify();
        }));
        sort_btn
    }

    /// Create selectable table cell with mouse event handlers.
    pub fn create_selectable_cell(
        display_cell_id: DisplayCellId,
        cell_content: SharedString,
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
                    let start_time = Instant::now();

                    // Calculate scroll direction by comparing current vs focused cell
                    let scroll =
                        if let Some(focused_cell) = this.engine.selection.get_focused_cell() {
                            match focused_cell.row.0.cmp(&display_cell_id.row.0) {
                                std::cmp::Ordering::Less => ScrollOffset::Positive, // Moving down
                                std::cmp::Ordering::Equal => ScrollOffset::NoOffset,
                                std::cmp::Ordering::Greater => ScrollOffset::Negative, // Moving up
                            }
                        } else {
                            ScrollOffset::NoOffset
                        };

                    let ordered_indices = this.engine.d2d_mapping.clone();
                    let preserve_existing = window.modifiers().secondary(); // cmd/ctrl key
                    this.engine.selection.start_mouse_selection(
                        display_cell_id.row,
                        display_cell_id.col,
                        &ordered_indices,
                        preserve_existing,
                    );
                    let selection_duration = start_time.elapsed();
                    this.performance_metrics.last_selection_took = Some(selection_duration);

                    // Update cell editor to show focused cell content
                    this.on_selection_changed(window, cx, Some(scroll));
                    cx.notify();
                });
            }
        })
        // Called when user moves mouse over a cell (for drag selection)
        .on_mouse_move({
            let view = view_entity.clone();
            move |event, window, cx| {
                view.update(cx, |this, cx| {
                    if !this.engine.selection.is_selecting() {
                        return;
                    }
                    if !event.dragging() {
                        // Workaround to stop selection if:
                        // 1. mouse was dragging,
                        // 2. went outside of table bounds
                        // 3. released lmb
                        // 4. returned back.
                        // Without this guard, it keeps extending selection despite not dragging anymore
                        this.engine.selection.end_mouse_selection();
                        return;
                    }
                    let start_time = Instant::now();
                    // Calculate scroll direction by comparing current vs focused cell
                    let scroll =
                        if let Some(focused_cell) = this.engine.selection.get_focused_cell() {
                            match focused_cell.row.0.cmp(&display_cell_id.row.0) {
                                std::cmp::Ordering::Less => ScrollOffset::Positive, // Moving down
                                std::cmp::Ordering::Equal => ScrollOffset::NoOffset,
                                std::cmp::Ordering::Greater => ScrollOffset::Negative, // Moving up
                            }
                        } else {
                            ScrollOffset::NoOffset
                        };

                    let ordered_indices = this.engine.d2d_mapping.clone();
                    let preserve_existing = window.modifiers().secondary(); // cmd/ctrl key
                    this.engine.selection.extend_mouse_selection(
                        display_cell_id.row,
                        display_cell_id.col,
                        &ordered_indices,
                        preserve_existing,
                    );
                    let selection_duration = start_time.elapsed();
                    this.performance_metrics.last_selection_took = Some(selection_duration);

                    // Update cell editor to show focused cell content during drag
                    let scroll = Some(scroll);
                    this.on_selection_changed(window, cx, scroll);
                    cx.notify();
                });
            }
        })
        // Called when user releases mouse button
        .on_mouse_up(MouseButton::Left, {
            let view = view_entity;
            move |_event, _window, cx| {
                view.update(cx, |this, cx| {
                    this.engine.selection.end_mouse_selection();
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
    cell_content: SharedString,
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
        .when(is_selected, |div| div.bg(selected_bg_color))
        .when(is_focused && is_anchor, |div| {
            div.bg(CellBorderColors::FOCUS_ANCHOR) // Focus + Anchor (blended color)
        })
        .when(is_focused && !is_anchor, |div| {
            div.bg(CellBorderColors::FOCUS) // Focus only
        })
        .when(is_anchor && !is_focused, |div| {
            div.bg(CellBorderColors::ANCHOR) // Anchor only
        })
        .map(|div| match font_type {
            FontType::Ui => div.font_ui(cx),
            FontType::Monospace => div.font_buffer(cx),
        })
        .tooltip(Tooltip::text(cell_content.clone()))
        .child(div().child(cell_content))
}
