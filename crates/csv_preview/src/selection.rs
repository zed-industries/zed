//! Table Selection Module
//!
//! Handles cell selection using dual coordinate system:
//! - Display coordinates: What user sees (visual positions)
//! - Data coordinates: Original CSV positions (stored selection)
//!
//! Selection follows data when sorting - intuitive behavior.

use std::collections::HashSet;

use crate::{
    data_ordering::OrderedIndices,
    types::{DataCellId, DisplayRow},
};

/// Navigation direction for keyboard focus movement
#[derive(Debug, Clone, Copy)]
pub enum NavigationDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Selected cells stored in data coordinates (not display coordinates).
pub type SelectedCells = HashSet<DataCellId>;

/// Manages table cell selection state and behavior.
pub struct TableSelection {
    /// Currently selected cells in data coordinates
    selected_cells: SelectedCells,
    /// Starting position for drag selection in display coordinates
    selection_start_display: Option<(usize, usize)>,
    /// Whether user is currently dragging to select
    is_selecting: bool,
    /// Currently focused cell in data coordinates
    focused_cell: Option<DataCellId>,
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
            focused_cell: None,
        }
    }

    /// Start cell selection at display position, stored as data coordinates.
    pub fn start_selection(
        &mut self,
        display_row: DisplayRow,
        col: usize,
        ordered_indices: &OrderedIndices,
    ) {
        self.selected_cells.clear();

        // Convert display coordinates to data coordinates for storage
        if let Some(data_row) = ordered_indices.get_data_row(display_row) {
            self.selected_cells.insert(DataCellId::new(data_row, col));
            // Set focus to the clicked cell
            self.focused_cell = Some(DataCellId::new(data_row, col));
        }

        // Remember display coordinates for extend_selection_to
        self.selection_start_display = Some((display_row.get(), col));
        self.is_selecting = true;
    }

    /// Extend selection rectangle from start to current position.
    pub fn extend_selection_to(
        &mut self,
        display_row: DisplayRow,
        col: usize,
        ordered_indices: &OrderedIndices,
    ) {
        if let Some((start_display_row, start_col)) = self.selection_start_display {
            self.selected_cells.clear();

            // Create rectangle in display coordinates
            let min_display_row = start_display_row.min(display_row.get());
            let max_display_row = start_display_row.max(display_row.get());
            let min_col = start_col.min(col);
            let max_col = start_col.max(col);

            // Convert each display cell to data coordinates for storage
            for display_r in min_display_row..=max_display_row {
                for c in min_col..=max_col {
                    if let Some(data_row) = ordered_indices.get_data_row(DisplayRow::new(display_r))
                    {
                        self.selected_cells.insert(DataCellId::new(data_row, c));
                    }
                }
            }

            // Update focused cell to follow the current mouse position (selection frontier)
            if let Some(data_row) = ordered_indices.get_data_row(display_row) {
                self.focused_cell = Some(DataCellId::new(data_row, col));
            }
        }
    }

    /// End cell selection (user stopped dragging)
    pub fn end_selection(&mut self) {
        self.is_selecting = false;
    }

    /// Check if cell at display coordinates is selected.
    pub fn is_cell_selected(
        &self,
        display_row: DisplayRow,
        col: usize,
        ordered_indices: &OrderedIndices,
    ) -> bool {
        if let Some(data_row) = ordered_indices.get_data_row(display_row) {
            self.selected_cells
                .contains(&DataCellId::new(data_row, col))
        } else {
            false
        }
    }

    /// Check if user is currently selecting (dragging)
    pub fn is_selecting(&self) -> bool {
        self.is_selecting
    }

    /// Get the selected cells for copying
    pub fn get_selected_cells(&self) -> &SelectedCells {
        &self.selected_cells
    }

    /// Check if cell at display coordinates is focused.
    pub fn is_cell_focused(
        &self,
        display_row: DisplayRow,
        col: usize,
        ordered_indices: &OrderedIndices,
    ) -> bool {
        if let (Some(focused), Some(data_row)) = (
            &self.focused_cell,
            ordered_indices.get_data_row(display_row),
        ) {
            focused.row == data_row && focused.col == col
        } else {
            false
        }
    }

    /// Initialize focus and selection to top-left cell if not already set
    fn ensure_focus_initialized(&mut self, ordered_indices: &OrderedIndices) {
        if let Some(data_row) = ordered_indices.get_data_row(DisplayRow::new(0)) {
            let new_cell = DataCellId::new(data_row, 0);
            self.focused_cell = Some(new_cell);
            // Update selection to follow focus
            self.selected_cells.clear();
            self.selected_cells.insert(new_cell);
        }
    }

    /// Move focus in the specified direction with bounds checking.
    /// Automatically initializes focus if none exists.
    fn move_focus_direction(
        &mut self,
        direction: NavigationDirection,
        ordered_indices: &OrderedIndices,
        max_rows: usize,
        max_cols: usize,
    ) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };
        let new_cell = match direction {
            NavigationDirection::Up => {
                if let Some(display_row) = ordered_indices.get_display_row(focused.row) {
                    if display_row.get() > 0 {
                        let new_display_row = DisplayRow::new(display_row.get() - 1);
                        if let Some(new_data_row) = ordered_indices.get_data_row(new_display_row) {
                            Some(DataCellId::new(new_data_row, focused.col))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            NavigationDirection::Down => {
                if let Some(display_row) = ordered_indices.get_display_row(focused.row) {
                    if display_row.get() < max_rows.saturating_sub(1) {
                        let new_display_row = DisplayRow::new(display_row.get() + 1);
                        if let Some(new_data_row) = ordered_indices.get_data_row(new_display_row) {
                            Some(DataCellId::new(new_data_row, focused.col))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            NavigationDirection::Left => {
                if focused.col > 0 {
                    Some(DataCellId::new(focused.row, focused.col - 1))
                } else {
                    None
                }
            }
            NavigationDirection::Right => {
                if focused.col < max_cols.saturating_sub(1) {
                    Some(DataCellId::new(focused.row, focused.col + 1))
                } else {
                    None
                }
            }
        };

        // Update focus and selection if movement was valid
        if let Some(new_cell) = new_cell {
            self.focused_cell = Some(new_cell);
            self.selected_cells.clear();
            self.selected_cells.insert(new_cell);
        }
    }

    /// Move focus up by one row.
    pub fn move_focus_up(&mut self, ordered_indices: &OrderedIndices) {
        self.move_focus_direction(
            NavigationDirection::Up,
            ordered_indices,
            usize::MAX,
            usize::MAX,
        );
    }

    /// Move focus down by one row.
    pub fn move_focus_down(&mut self, ordered_indices: &OrderedIndices, max_rows: usize) {
        self.move_focus_direction(
            NavigationDirection::Down,
            ordered_indices,
            max_rows,
            usize::MAX,
        );
    }

    /// Move focus left by one column.
    pub fn move_focus_left(&mut self, ordered_indices: &OrderedIndices) {
        self.move_focus_direction(
            NavigationDirection::Left,
            ordered_indices,
            usize::MAX,
            usize::MAX,
        );
    }

    /// Move focus right by one column.
    pub fn move_focus_right(&mut self, ordered_indices: &OrderedIndices, max_cols: usize) {
        self.move_focus_direction(
            NavigationDirection::Right,
            ordered_indices,
            usize::MAX,
            max_cols,
        );
    }
}
