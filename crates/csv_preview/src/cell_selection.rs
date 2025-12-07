//! Table Selection Module
//!
//! Handles cell selection for CSV tables with support for sorting.
//!
//! ## Selection Coordinate System
//!
//! This module uses a dual-coordinate system to provide intuitive selection behavior:
//!
//! - **Display Coordinates**: What the user sees and interacts with (visual positions)
//! - **Data Coordinates**: Original CSV row positions (what gets stored)
//!
//! ## Selection Behavior Examples
//!
//! **Scenario 1: User selects, then sorts (same visual position)**
//! 1. CSV data: [Alice, Bob, Charlie]  (display: Alice=0, Bob=1, Charlie=2)
//! 2. User clicks display row 1 (Bob) → stores data_row=1, col=0
//! 3. User sorts by name → display changes: [Alice=0, Bob=1, Charlie=2]
//! 4. Bob stays selected at display row 1 (same position, data follows)
//!
//! **Scenario 2: User selects, then sorts (different visual position)**
//! 1. CSV data: [Alice:30, Bob:25, Charlie:35]
//! 2. User clicks display row 0 (Alice) → stores data_row=0, col=0
//! 3. User sorts by age → display: [Bob:25=0, Alice:30=1, Charlie:35=2]
//! 4. Alice moves to display row 1, but stays selected (data follows)
//!
//! ## Technical Flow
//! 1. **User Input**: Always display coordinates (what they see/click)
//! 2. **Storage**: Immediately convert to data coordinates
//! 3. **Sorting**: Selection automatically follows the data
//! 4. **Rendering**: Convert data → display coordinates for highlighting

use gpui::{AnyElement, ElementId, Entity, MouseButton};
use std::collections::HashSet;
use ui::{div, prelude::*};

use crate::{CsvPreviewView, data_ordering::generate_ordered_indecies};

/// Selected cells using data coordinates (data_row, col).
///
/// IMPORTANT: This stores CSV data coordinates, NOT display coordinates.
/// - data_row: The row index in the original CSV data (0-based)
/// - col: The CSV data column index (0-based, excluding line number column)
///
/// Selection flow:
/// 1. User clicks/drags using display coordinates (what they see)
/// 2. Immediately convert display → data coordinates for storage
/// 3. Selection follows the data when sorting changes (intuitive behavior)
/// 4. Renderer converts data → display coordinates to highlight correct cells
pub type SelectedCells = HashSet<(usize, usize)>;

/// Manages table cell selection state and behavior.
pub struct TableSelection {
    /// Currently selected cells in data coordinates
    selected_cells: SelectedCells,
    /// Starting position for drag selection in display coordinates
    selection_start_display: Option<(usize, usize)>,
    /// Whether user is currently dragging to select
    is_selecting: bool,
}

impl Default for TableSelection {
    fn default() -> Self {
        Self::new()
    }
}

impl TableSelection {
    /// Create a new empty table selection
    pub fn new() -> Self {
        Self {
            selected_cells: HashSet::new(),
            selection_start_display: None,
            is_selecting: false,
        }
    }

    /// Start cell selection at the given display position.
    ///
    /// **Input**: Display coordinates (what user clicked on screen)
    /// **Storage**: Data coordinates (so selection follows data when sorting)
    ///
    /// Example: User clicks visually displayed row 2, column 1
    /// → Converts to original CSV data row (e.g. row 5) for storage
    /// → If table gets sorted, selection stays with that data row
    pub fn start_selection<F>(
        &mut self,
        display_row: usize,
        col: usize,
        display_to_data_converter: F,
    ) where
        F: Fn(usize) -> Option<usize>,
    {
        self.selected_cells.clear();

        // Convert display coordinates to data coordinates for storage
        if let Some(data_row) = display_to_data_converter(display_row) {
            self.selected_cells.insert((data_row, col));
        }

        // Remember display coordinates for extend_selection_to
        self.selection_start_display = Some((display_row, col));
        self.is_selecting = true;
    }

    /// Extend selection to include cells from start position to current position.
    ///
    /// **Algorithm**:
    /// 1. Create rectangle in display coordinates (what user sees)
    /// 2. Convert each display cell → data coordinates for storage
    /// 3. Selection persisted as data coordinates (follows data on sort)
    ///
    /// **Example**: User drags from display (0,0) to (2,1)
    /// → Creates 3×2 rectangle in display space
    /// → Converts each cell: display(0,0)→data(3,0), display(0,1)→data(3,1), etc.
    /// → Stores data coordinates so selection survives sorting
    pub fn extend_selection_to<F>(
        &mut self,
        display_row: usize,
        col: usize,
        display_to_data_converter: F,
    ) where
        F: Fn(usize) -> Option<usize>,
    {
        if let Some((start_display_row, start_col)) = self.selection_start_display {
            self.selected_cells.clear();

            // Create rectangle in display coordinates
            let min_display_row = start_display_row.min(display_row);
            let max_display_row = start_display_row.max(display_row);
            let min_col = start_col.min(col);
            let max_col = start_col.max(col);

            // Convert each display cell to data coordinates for storage
            for display_r in min_display_row..=max_display_row {
                for c in min_col..=max_col {
                    if let Some(data_row) = display_to_data_converter(display_r) {
                        self.selected_cells.insert((data_row, c));
                    }
                }
            }
        }
    }

    /// End cell selection (user stopped dragging)
    pub fn end_selection(&mut self) {
        self.is_selecting = false;
    }

    /// Check if a cell is currently selected using display coordinates.
    ///
    /// **Input**: Display coordinates (renderer asking "is this visible cell selected?")
    /// **Process**: Convert display → data coordinates, check against stored selection
    /// **Output**: True if the data at this display position is selected
    ///
    /// This ensures selection highlighting follows the data regardless of sort order.
    pub fn is_cell_selected<F>(
        &self,
        display_row: usize,
        col: usize,
        display_to_data_converter: F,
    ) -> bool
    where
        F: Fn(usize) -> Option<usize>,
    {
        if let Some(data_row) = display_to_data_converter(display_row) {
            self.selected_cells.contains(&(data_row, col))
        } else {
            false
        }
    }

    /// Check if user is currently selecting (dragging)
    pub fn is_selecting(&self) -> bool {
        self.is_selecting
    }
}

/// Selection-related UI rendering functions
impl TableSelection {
    /// Create a selectable table cell element with proper event handlers.
    ///
    /// This function creates the interactive cell div with mouse event handlers
    /// that work with the display/data coordinate conversion system.
    pub fn create_selectable_cell(
        display_row: usize,
        col: usize,
        cell_content: impl IntoElement,
        view_entity: Entity<CsvPreviewView>,
        selected_bg_color: gpui::Hsla,
        is_selected: bool,
    ) -> AnyElement {
        div()
            .id(ElementId::NamedInteger(
                format!("csv-display-cell-{}-{}", display_row, col).into(),
                0,
            ))
            .child(cell_content)
            .cursor_pointer()
            .when(is_selected, |div| div.bg(selected_bg_color))
            // Called when user presses mouse button down on a cell
            .on_mouse_down(MouseButton::Left, {
                let view = view_entity.clone();
                move |_event, _window, cx| {
                    view.update(cx, |this, cx| {
                        let ordered_indices =
                            generate_ordered_indecies(this.ordering, &this.contents);
                        let display_to_data_converter =
                            move |dr: usize| ordered_indices.get(dr).copied();

                        this.selection
                            .start_selection(display_row, col, display_to_data_converter);
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
                        // Create converter function without borrowing self
                        let ordered_indices =
                            generate_ordered_indecies(this.ordering, &this.contents);
                        let display_to_data_converter =
                            move |dr: usize| ordered_indices.get(dr).copied();

                        this.selection.extend_selection_to(
                            display_row,
                            col,
                            display_to_data_converter,
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
