//! Table Selection Module
//!
//! Handles cell selection using dual coordinate system:
//! - Display coordinates: What user sees (visual positions)
//! - Data coordinates: Original CSV positions (stored selection)
//!
//! Selection follows data when sorting - intuitive behavior.

use std::{collections::HashSet, time::Instant};

use ui::Context;

use crate::{
    CsvPreviewView,
    data_ordering::OrderedIndices,
    types::{AnyColumn, DataCellId, DisplayCellId, DisplayRow},
};

/// Navigation direction for keyboard focus movement
#[derive(Debug, Clone, Copy)]
pub enum NavigationDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Navigation operation type determining how focus/selection should change
#[derive(Debug, Clone, Copy)]
pub enum NavigationOperation {
    /// Move focus only (single cell selection)
    MoveFocus,
    /// Extend current selection from anchor
    ExtendSelection,
    /// Jump focus to table edge
    JumpToEdge,
    /// Extend selection to table edge
    ExtendToEdge,
}

/// Selected cells stored in data coordinates (not display coordinates).
pub type SelectedCells = HashSet<DataCellId>;

/// Manages table cell selection state and behavior.
pub struct TableSelection {
    /// Currently selected cells in data coordinates
    selected_cells: SelectedCells,
    /// Whether user is currently dragging to select
    is_selecting: bool,
    /// Currently focused cell in display coordinates
    focused_cell: Option<DisplayCellId>,
    /// Anchor cell for range selection (both keyboard and mouse) in display coordinates
    selection_anchor: Option<DisplayCellId>,
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
            is_selecting: false,
            focused_cell: None,
            selection_anchor: None,
        }
    }

    /// Start cell selection with option to preserve existing selection (cumulative).
    pub fn start_selection_with_cumulative(
        &mut self,
        display_row: DisplayRow,
        col: AnyColumn,
        ordered_indices: &OrderedIndices,
        preserve_existing: bool,
    ) {
        if !preserve_existing {
            self.selected_cells.clear();
        }

        // Convert display coordinates to data coordinates for storage
        if let Some(data_row) = ordered_indices.get_data_row(display_row) {
            let cell_id = DataCellId::new(data_row, col);
            self.selected_cells.insert(cell_id);
            // Set focus and anchor to the clicked cell in display coordinates
            let display_cell_id = DisplayCellId::new(display_row, col);
            self.focused_cell = Some(display_cell_id);
            self.selection_anchor = Some(display_cell_id);
        }

        self.is_selecting = true;
    }

    /// Extend selection rectangle from start to current position.
    pub fn extend_selection_to(
        &mut self,
        display_row: DisplayRow,
        col: AnyColumn,
        ordered_indices: &OrderedIndices,
        preserve_existing: bool,
    ) {
        if let Some(anchor_cell) = self.selection_anchor {
            if !preserve_existing {
                self.selected_cells.clear();
            }

            // Create rectangle in display coordinates
            let min_display_row = anchor_cell.row.get().min(display_row.get());
            let max_display_row = anchor_cell.row.get().max(display_row.get());
            let min_col = anchor_cell.col.get().min(col.get());
            let max_col = anchor_cell.col.get().max(col.get());

            // Convert each display cell to data coordinates for storage
            for display_r in min_display_row..=max_display_row {
                for c in min_col..=max_col {
                    if let Some(data_row) =
                        ordered_indices.get_data_row(DisplayRow::from(display_r))
                    {
                        self.selected_cells.insert(DataCellId::new(data_row, c));
                    }
                }
            }

            // Update focused cell to follow the current mouse position (selection frontier)
            self.focused_cell = Some(DisplayCellId::new(display_row, col));
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
        col: AnyColumn,
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
    pub fn is_cell_focused(&self, display_row: DisplayRow, col: AnyColumn) -> bool {
        if let Some(focused) = &self.focused_cell {
            focused.row == display_row && focused.col == col
        } else {
            false
        }
    }

    /// Check if cell at display coordinates is the selection anchor.
    pub fn is_cell_anchor(&self, display_row: DisplayRow, col: AnyColumn) -> bool {
        if let Some(anchor) = &self.selection_anchor {
            anchor.row == display_row && anchor.col == col
        } else {
            false
        }
    }

    /// Check if any cell in the given display row has focus
    pub fn is_row_focused(&self, display_row: DisplayRow) -> bool {
        if let Some(focused) = &self.focused_cell {
            focused.row == display_row
        } else {
            false
        }
    }

    /// Initialize focus and selection to top-left cell if not already set
    fn ensure_focus_initialized(&mut self, ordered_indices: &OrderedIndices) {
        let display_cell = DisplayCellId::new(0, 0);
        self.focused_cell = Some(display_cell);
        // Set anchor to the same cell for consistent visual feedback
        self.selection_anchor = Some(display_cell);
        // Update selection to follow focus
        self.selected_cells.clear();
        if let Some(data_row) = ordered_indices.get_data_row(DisplayRow::from(0)) {
            self.selected_cells.insert(DataCellId::new(data_row, 0));
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
                if focused.row.get() > 0 {
                    Some(DisplayCellId::new(focused.row.get() - 1, focused.col))
                } else {
                    None
                }
            }
            NavigationDirection::Down => {
                if focused.row.get() < max_rows.saturating_sub(1) {
                    Some(DisplayCellId::new(focused.row.get() + 1, focused.col))
                } else {
                    None
                }
            }
            NavigationDirection::Left => {
                if focused.col.get() > 0 {
                    Some(DisplayCellId::new(focused.row, focused.col.get() - 1))
                } else {
                    None
                }
            }
            NavigationDirection::Right => {
                if focused.col.get() < max_cols.saturating_sub(1) {
                    Some(DisplayCellId::new(focused.row, focused.col.get() + 1))
                } else {
                    None
                }
            }
        };

        // Update focus and selection if movement was valid
        if let Some(new_cell) = new_cell {
            self.focused_cell = Some(new_cell);
            // Set anchor to the same cell for consistent visual feedback
            self.selection_anchor = Some(new_cell);
            self.selected_cells.clear();
            // Convert to data coordinates for storage
            if let Some(data_row) = ordered_indices.get_data_row(new_cell.row) {
                self.selected_cells
                    .insert(DataCellId::new(data_row, new_cell.col));
            }
        }
    }

    /// Set selection anchor for range selection. Called when starting range selection.
    fn set_selection_anchor(&mut self) {
        if let Some(focused) = self.focused_cell {
            self.selection_anchor = Some(focused);
        }
    }

    /// Update selection from anchor to focused cell (rectangular selection).
    fn update_range_selection(&mut self, ordered_indices: &OrderedIndices) {
        if let (Some(anchor), Some(focused)) = (self.selection_anchor, self.focused_cell) {
            self.selected_cells.clear();

            // Create rectangle in display coordinates (both anchor and focus are already in display coordinates)
            let min_display_row = anchor.row.get().min(focused.row.get());
            let max_display_row = anchor.row.get().max(focused.row.get());
            let min_col = anchor.col.get().min(focused.col.get());
            let max_col = anchor.col.get().max(focused.col.get());

            // Convert each display cell to data coordinates for storage
            for display_r in min_display_row..=max_display_row {
                for c in min_col..=max_col {
                    if let Some(data_row) =
                        ordered_indices.get_data_row(DisplayRow::from(display_r))
                    {
                        self.selected_cells.insert(DataCellId::new(data_row, c));
                    }
                }
            }
        }
    }

    /// Move focus in the specified direction and extend selection.
    /// For range selection (shift + arrow keys).
    fn extend_selection_direction(
        &mut self,
        direction: NavigationDirection,
        ordered_indices: &OrderedIndices,
        max_rows: usize,
        max_cols: usize,
    ) {
        // Initialize focus if not set
        if self.focused_cell.is_none() {
            self.ensure_focus_initialized(ordered_indices);
        }

        // Set anchor if not already set (first range selection)
        if self.selection_anchor.is_none() {
            self.set_selection_anchor();
        }

        let Some(focused) = self.focused_cell else {
            return;
        };

        let new_cell = match direction {
            NavigationDirection::Up => {
                if focused.row.get() > 0 {
                    Some(DisplayCellId::new(focused.row.get() - 1, focused.col))
                } else {
                    None
                }
            }
            NavigationDirection::Down => {
                if focused.row.get() < max_rows.saturating_sub(1) {
                    Some(DisplayCellId::new(focused.row.get() + 1, focused.col))
                } else {
                    None
                }
            }
            NavigationDirection::Left => {
                if focused.col.get() > 0 {
                    Some(DisplayCellId::new(focused.row, focused.col.get() - 1))
                } else {
                    None
                }
            }
            NavigationDirection::Right => {
                if focused.col.get() < max_cols.saturating_sub(1) {
                    Some(DisplayCellId::new(focused.row, focused.col.get() + 1))
                } else {
                    None
                }
            }
        };

        // Update focus and rebuild selection rectangle
        if let Some(new_cell) = new_cell {
            self.focused_cell = Some(new_cell);
            self.update_range_selection(ordered_indices);
        }
    }

    /// Select all visible cells in the table (cmd+a / ctrl+a).
    pub fn select_all(
        &mut self,
        ordered_indices: &OrderedIndices,
        max_rows: usize,
        max_cols: usize,
    ) {
        if max_rows == 0 || max_cols == 0 {
            return;
        }

        self.selected_cells.clear();

        // Select all cells from (0,0) to (max_rows-1, max_cols-1) in display coordinates
        for display_row_index in 0..max_rows {
            for col in 0..max_cols {
                if let Some(data_row) =
                    ordered_indices.get_data_row(DisplayRow::from(display_row_index))
                {
                    self.selected_cells.insert(DataCellId::new(data_row, col));
                }
            }
        }

        // Set focus to the first cell and anchor to the last cell in display coordinates
        self.focused_cell = Some(DisplayCellId::new(0, 0));
        self.selection_anchor = Some(DisplayCellId::new(max_rows - 1, max_cols - 1));
    }

    /// Jump focus to the top row (first row in display order)
    pub fn jump_to_top_edge(&mut self, ordered_indices: &OrderedIndices) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        let new_cell = DisplayCellId::new(0, focused.col);
        self.focused_cell = Some(new_cell);
        self.selection_anchor = Some(new_cell);
        self.selected_cells.clear();
        // Convert to data coordinates for storage
        if let Some(data_row) = ordered_indices.get_data_row(DisplayRow::from(0)) {
            self.selected_cells
                .insert(DataCellId::new(data_row, focused.col));
        }
    }

    /// Jump focus to the bottom row (last row in display order)
    pub fn jump_to_bottom_edge(&mut self, ordered_indices: &OrderedIndices, max_rows: usize) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        if max_rows > 0 {
            let new_cell = DisplayCellId::new(max_rows - 1, focused.col);
            self.focused_cell = Some(new_cell);
            self.selection_anchor = Some(new_cell);
            self.selected_cells.clear();
            // Convert to data coordinates for storage
            if let Some(data_row) = ordered_indices.get_data_row(DisplayRow::from(max_rows - 1)) {
                self.selected_cells
                    .insert(DataCellId::new(data_row, focused.col));
            }
        }
    }

    /// Jump focus to the leftmost column (column 0)
    pub fn jump_to_left_edge(&mut self, ordered_indices: &OrderedIndices) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        let new_cell = DisplayCellId::new(focused.row, 0);
        self.focused_cell = Some(new_cell);
        self.selection_anchor = Some(new_cell);
        self.selected_cells.clear();
        // Convert to data coordinates for storage
        if let Some(data_row) = ordered_indices.get_data_row(focused.row) {
            self.selected_cells.insert(DataCellId::new(data_row, 0));
        }
    }

    /// Jump focus to the rightmost column (last column)
    pub fn jump_to_right_edge(&mut self, ordered_indices: &OrderedIndices, max_cols: usize) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        if max_cols > 0 {
            let new_cell = DisplayCellId::new(focused.row, max_cols - 1);
            self.focused_cell = Some(new_cell);
            self.selection_anchor = Some(new_cell);
            self.selected_cells.clear();
            // Convert to data coordinates for storage
            if let Some(data_row) = ordered_indices.get_data_row(focused.row) {
                self.selected_cells
                    .insert(DataCellId::new(data_row, max_cols - 1));
            }
        }
    }

    /// Extend selection to the top row while keeping anchor
    pub fn extend_selection_to_top_edge(&mut self, ordered_indices: &OrderedIndices) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        let new_cell = DisplayCellId::new(0, focused.col);
        self.focused_cell = Some(new_cell);

        // Set anchor if not already set
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(focused);
        }

        self.update_range_selection(ordered_indices);
    }

    /// Extend selection to the bottom row while keeping anchor
    pub fn extend_selection_to_bottom_edge(
        &mut self,
        ordered_indices: &OrderedIndices,
        max_rows: usize,
    ) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        if max_rows > 0 {
            let new_cell = DisplayCellId::new(max_rows - 1, focused.col);
            self.focused_cell = Some(new_cell);

            // Set anchor if not already set
            if self.selection_anchor.is_none() {
                self.selection_anchor = Some(focused);
            }

            self.update_range_selection(ordered_indices);
        }
    }

    /// Extend selection to the leftmost column while keeping anchor
    pub fn extend_selection_to_left_edge(&mut self, ordered_indices: &OrderedIndices) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        let new_cell = DisplayCellId::new(focused.row, 0);
        self.focused_cell = Some(new_cell);

        // Set anchor if not already set
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(focused);
        }

        self.update_range_selection(ordered_indices);
    }

    /// Extend selection to the rightmost column while keeping anchor
    pub fn extend_selection_to_right_edge(
        &mut self,
        ordered_indices: &OrderedIndices,
        max_cols: usize,
    ) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        if max_cols > 0 {
            let new_cell = DisplayCellId::new(focused.row, max_cols - 1);
            self.focused_cell = Some(new_cell);

            // Set anchor if not already set
            if self.selection_anchor.is_none() {
                self.selection_anchor = Some(focused);
            }

            self.update_range_selection(ordered_indices);
        }
    }

    /// Unified navigation method that handles all direction + operation combinations
    pub fn navigate(
        &mut self,
        direction: NavigationDirection,
        operation: NavigationOperation,
        ordered_indices: &OrderedIndices,
        max_rows: usize,
        max_cols: usize,
    ) {
        match operation {
            NavigationOperation::MoveFocus => {
                self.move_focus_direction(direction, ordered_indices, max_rows, max_cols)
            }
            NavigationOperation::ExtendSelection => {
                self.extend_selection_direction(direction, ordered_indices, max_rows, max_cols)
            }
            NavigationOperation::JumpToEdge => {
                self.jump_to_edge_direction(direction, ordered_indices, max_rows, max_cols)
            }
            NavigationOperation::ExtendToEdge => {
                self.extend_to_edge_direction(direction, ordered_indices, max_rows, max_cols)
            }
        }
    }

    /// Helper method for jump to edge operations
    fn jump_to_edge_direction(
        &mut self,
        direction: NavigationDirection,
        ordered_indices: &OrderedIndices,
        max_rows: usize,
        max_cols: usize,
    ) {
        match direction {
            NavigationDirection::Up => self.jump_to_top_edge(ordered_indices),
            NavigationDirection::Down => self.jump_to_bottom_edge(ordered_indices, max_rows),
            NavigationDirection::Left => self.jump_to_left_edge(ordered_indices),
            NavigationDirection::Right => self.jump_to_right_edge(ordered_indices, max_cols),
        }
    }

    /// Helper method for extend to edge operations
    fn extend_to_edge_direction(
        &mut self,
        direction: NavigationDirection,
        ordered_indices: &OrderedIndices,
        max_rows: usize,
        max_cols: usize,
    ) {
        match direction {
            NavigationDirection::Up => self.extend_selection_to_top_edge(ordered_indices),
            NavigationDirection::Down => {
                self.extend_selection_to_bottom_edge(ordered_indices, max_rows)
            }
            NavigationDirection::Left => self.extend_selection_to_left_edge(ordered_indices),
            NavigationDirection::Right => {
                self.extend_selection_to_right_edge(ordered_indices, max_cols)
            }
        }
    }
}

///// Selection related CsvPreviewView methods /////
impl CsvPreviewView {
    /// Unified navigation handler - eliminates code duplication
    pub(crate) fn handle_navigation(
        &mut self,
        direction: NavigationDirection,
        operation: NavigationOperation,
        cx: &mut Context<Self>,
    ) {
        let start_time = Instant::now();
        let max_rows = self.contents.rows.len();
        let max_cols = self.contents.headers.len();

        self.selection.navigate(
            direction,
            operation,
            &self.ordered_indices,
            max_rows,
            max_cols,
        );

        let selection_duration = start_time.elapsed();
        self.performance_metrics.last_selection_took = Some(selection_duration);
        cx.notify();
    }
}
