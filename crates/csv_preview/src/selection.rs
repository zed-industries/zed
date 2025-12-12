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
}
