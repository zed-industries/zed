//! Table Selection Module
//!
//! Handles cell selection using dual coordinate system:
//! - Display coordinates: What user sees (visual positions)
//! - Data coordinates: Original CSV positions (stored selection)
//!
//! Selection follows data when sorting - intuitive behavior.

use std::{collections::HashSet, time::Instant};

use gpui::ScrollStrategy;
use ui::{Context, Window};

use crate::{
    CsvPreviewView,
    table_data_engine::sorting_by_column::SortedIndices,
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

/// Internal selection strategy for performance optimization
#[derive(Debug, Clone)]
enum SelectionStrategy {
    /// No cells selected
    Empty,
    /// Whole document selected (CMD+A) - Major optimization for large files
    AllCells,
    /// Single cell selected - Minor optimization for common case
    SingleCell(DataCellId),
    /// Multiple cells - fallback to current HashSet approach
    MultiCell(HashSet<DataCellId>),
}

/// Manages cell selection with optimizations for common cases.
///
/// Uses a hybrid strategy:
/// - `Empty`: No selection (O(1) operations)
/// - `AllCells`: Whole document selected (O(1) select-all and lookup)
/// - `SingleCell`: One cell selected (O(1) lookup)
/// - `MultiCell`: Multiple cells (current HashSet performance)
///
/// This provides immediate optimization for select-all operations while
/// maintaining compatibility and setting up for future range-based optimizations.
#[derive(Debug, Clone)]
pub struct CellSelectionManager {
    strategy: SelectionStrategy,
}

impl Default for CellSelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CellSelectionManager {
    /// Create a new empty selection manager
    pub fn new() -> Self {
        Self {
            strategy: SelectionStrategy::Empty,
        }
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.strategy = SelectionStrategy::Empty;
    }

    /// Select all cells in the document (O(1) operation)
    pub fn select_all(&mut self) {
        self.strategy = SelectionStrategy::AllCells;
    }

    /// Add a single cell to the selection
    ///
    /// # Arguments
    /// * `display_cell` - Cell coordinates in display space
    /// * `ordered_indices` - Mapping between display and data coordinates
    pub fn add_cell(&mut self, display_cell: DisplayCellId, ordered_indices: &SortedIndices) {
        if let Some(data_row) =
            ordered_indices.get_data_row(DisplayRow::from(display_cell.row.get()))
        {
            let data_cell_id = DataCellId::new(data_row, display_cell.col.get());

            match &mut self.strategy {
                SelectionStrategy::Empty => {
                    self.strategy = SelectionStrategy::SingleCell(data_cell_id);
                }
                SelectionStrategy::AllCells => {
                    // Already all selected, nothing to add
                }
                SelectionStrategy::SingleCell(existing) => {
                    if *existing == data_cell_id {
                        return; // Same cell, nothing to change
                    }

                    // Convert to MultiCell strategy
                    let mut cells = HashSet::new();
                    cells.insert(*existing);
                    cells.insert(data_cell_id);
                    self.strategy = SelectionStrategy::MultiCell(cells);
                }
                SelectionStrategy::MultiCell(cells) => {
                    cells.insert(data_cell_id);
                }
            }
        }
    }

    /// Add a rectangular range of cells to the selection
    ///
    /// # Arguments
    /// * `start` - Starting cell in display coordinates
    /// * `end` - Ending cell in display coordinates
    /// * `ordered_indices` - Mapping between display and data coordinates
    pub fn add_range(
        &mut self,
        start: DisplayCellId,
        end: DisplayCellId,
        ordered_indices: &SortedIndices,
    ) {
        // Simple implementation - add individual cells (can optimize with ranges later)
        let min_row = start.row.get().min(end.row.get());
        let max_row = start.row.get().max(end.row.get());
        let min_col = start.col.get().min(end.col.get());
        let max_col = start.col.get().max(end.col.get());

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let display_cell = DisplayCellId::new(row, col);
                self.add_cell(display_cell, ordered_indices);
            }
        }
    }

    /// Check if a cell is selected (optimized for common cases)
    ///
    /// # Arguments
    /// * `display_row` - Row in display coordinates
    /// * `col` - Column index
    /// * `ordered_indices` - Mapping between display and data coordinates
    ///
    /// # Returns
    /// `true` if the cell is selected, `false` otherwise
    pub fn is_selected(
        &self,
        display_row: DisplayRow,
        col: AnyColumn,
        ordered_indices: &SortedIndices,
    ) -> bool {
        match &self.strategy {
            SelectionStrategy::Empty => false,
            SelectionStrategy::AllCells => true, // O(1) optimization for select-all!
            SelectionStrategy::SingleCell(data_cell) => {
                if let Some(data_row) = ordered_indices.get_data_row(display_row) {
                    *data_cell == DataCellId::new(data_row, col)
                } else {
                    false
                }
            }
            SelectionStrategy::MultiCell(cells) => {
                if let Some(data_row) = ordered_indices.get_data_row(display_row) {
                    cells.contains(&DataCellId::new(data_row, col))
                } else {
                    false
                }
            }
        }
    }

    /// Get selected cells as HashSet for compatibility with existing code
    ///
    /// Note: For `AllCells` strategy, this requires materializing all cells
    /// and should be used sparingly (e.g., only for copy operations).
    ///
    /// # Arguments
    /// * `ordered_indices` - Mapping between display and data coordinates
    /// * `max_rows` - Maximum number of rows (for AllCells strategy)
    /// * `max_cols` - Maximum number of columns (for AllCells strategy)
    pub fn get_selected_cells(
        &self,
        ordered_indices: &SortedIndices,
        max_rows: usize,
        max_cols: usize,
    ) -> HashSet<DataCellId> {
        match &self.strategy {
            SelectionStrategy::Empty => HashSet::new(),
            SelectionStrategy::AllCells => {
                // Expensive operation - materialize all cells
                let mut cells = HashSet::new();
                for display_row_index in 0..max_rows {
                    for col in 0..max_cols {
                        if let Some(data_row) =
                            ordered_indices.get_data_row(DisplayRow::from(display_row_index))
                        {
                            cells.insert(DataCellId::new(data_row, col));
                        }
                    }
                }
                cells
            }
            SelectionStrategy::SingleCell(data_cell) => {
                let mut cells = HashSet::new();
                cells.insert(*data_cell);
                cells
            }
            SelectionStrategy::MultiCell(cells) => cells.clone(),
        }
    }

    /// Get selected cells as DisplayCellId set for display-order copying
    ///
    /// Returns cells in their display coordinates, preserving the visual order
    /// that the user sees (after sorting).
    ///
    /// # Arguments
    /// * `ordered_indices` - Mapping between display and data coordinates
    /// * `max_rows` - Maximum number of rows (for AllCells strategy)
    /// * `max_cols` - Maximum number of columns (for AllCells strategy)
    pub fn get_selected_display_cells(
        &self,
        ordered_indices: &SortedIndices,
        max_rows: usize,
        max_cols: usize,
    ) -> HashSet<DisplayCellId> {
        match &self.strategy {
            SelectionStrategy::Empty => HashSet::new(),
            SelectionStrategy::AllCells => {
                // Materialize all cells in display coordinates
                let mut cells = HashSet::new();
                for display_row_index in 0..max_rows {
                    for col in 0..max_cols {
                        cells.insert(DisplayCellId::new(display_row_index, col));
                    }
                }
                cells
            }
            SelectionStrategy::SingleCell(data_cell) => {
                let mut cells = HashSet::new();
                // Convert data cell back to display coordinates
                if let Some(display_row) = ordered_indices.get_display_row(data_cell.row) {
                    cells.insert(DisplayCellId::new(display_row.get(), data_cell.col));
                }
                cells
            }
            SelectionStrategy::MultiCell(data_cells) => {
                let mut display_cells = HashSet::new();
                // Convert each data cell to display coordinates
                for data_cell in data_cells {
                    if let Some(display_row) = ordered_indices.get_display_row(data_cell.row) {
                        display_cells.insert(DisplayCellId::new(display_row.get(), data_cell.col));
                    }
                }
                display_cells
            }
        }
    }
}

/// Manages table cell selection state and behavior.
pub struct TableSelection {
    /// Efficient selection management with optimizations for common cases
    selection_manager: CellSelectionManager,
    /// Whether user is currently dragging to select
    is_selecting: bool,
    /// Currently focused cell in display coordinates
    pub focused_cell: Option<DisplayCellId>,
    /// Anchor cell for range selection (both keyboard and mouse) in display coordinates
    pub selection_anchor: Option<DisplayCellId>,
}

impl Default for TableSelection {
    fn default() -> Self {
        Self::new()
    }
}

impl TableSelection {
    pub fn new() -> Self {
        Self {
            selection_manager: CellSelectionManager::new(),
            is_selecting: false,
            focused_cell: None,
            selection_anchor: None,
        }
    }

    /// Start cell selection with option to preserve existing selection (cumulative).
    pub fn start_mouse_selection(
        &mut self,
        display_row: DisplayRow,
        col: AnyColumn,
        ordered_indices: &SortedIndices,
        preserve_existing: bool,
    ) {
        if !preserve_existing {
            self.selection_manager.clear();
        }

        // Set focus and anchor to the clicked cell in display coordinates
        let display_cell_id = DisplayCellId::new(display_row, col);
        self.focused_cell = Some(display_cell_id);
        self.selection_anchor = Some(display_cell_id);
        self.selection_manager
            .add_cell(display_cell_id, ordered_indices);

        self.is_selecting = true;
    }

    /// Extend selection rectangle from start to current position.
    pub fn extend_mouse_selection(
        &mut self,
        display_row: DisplayRow,
        col: AnyColumn,
        ordered_indices: &SortedIndices,
        preserve_existing: bool,
    ) {
        let Some(anchor_cell) = self.selection_anchor else {
            return;
        };

        if !preserve_existing {
            self.selection_manager.clear();
        }

        // Create rectangle in display coordinates
        let min_display_row = anchor_cell.row.get().min(display_row.get());
        let max_display_row = anchor_cell.row.get().max(display_row.get());
        let min_col = anchor_cell.col.get().min(col.get());
        let max_col = anchor_cell.col.get().max(col.get());

        // Convert each display cell to data coordinates for storage
        for display_r in min_display_row..=max_display_row {
            for c in min_col..=max_col {
                let display_cell = DisplayCellId::new(display_r, c);
                self.selection_manager
                    .add_cell(display_cell, ordered_indices);
            }
        }

        // Update focused cell to follow the current mouse position (selection frontier)
        self.focused_cell = Some(DisplayCellId::new(display_row, col));
    }

    /// End cell selection (user stopped dragging)
    pub fn end_mouse_selection(&mut self) {
        self.is_selecting = false;
    }

    /// Check if cell at display coordinates is selected.
    pub fn is_cell_selected(
        &self,
        display_row: DisplayRow,
        col: AnyColumn,
        ordered_indices: &SortedIndices,
    ) -> bool {
        self.selection_manager
            .is_selected(display_row, col, ordered_indices)
    }

    /// Check if user is currently selecting (dragging)
    pub fn is_selecting(&self) -> bool {
        self.is_selecting
    }

    /// Get the selected cells for copying
    pub fn get_selected_cells(
        &self,
        ordered_indices: &SortedIndices,
        max_rows: usize,
        max_cols: usize,
    ) -> HashSet<DataCellId> {
        self.selection_manager
            .get_selected_cells(ordered_indices, max_rows, max_cols)
    }

    /// Get selected cells in display coordinates for display-order copying
    pub fn get_selected_display_cells(
        &self,
        ordered_indices: &SortedIndices,
        max_rows: usize,
        max_cols: usize,
    ) -> HashSet<DisplayCellId> {
        self.selection_manager
            .get_selected_display_cells(ordered_indices, max_rows, max_cols)
    }

    /// Get the currently focused cell
    pub fn get_focused_cell(&self) -> Option<DisplayCellId> {
        self.focused_cell
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
    fn ensure_focus_initialized(&mut self, ordered_indices: &SortedIndices) {
        let display_cell = DisplayCellId::new(0, 0);
        self.focused_cell = Some(display_cell);
        // Set anchor to the same cell for consistent visual feedback
        self.selection_anchor = Some(display_cell);
        // Update selection to follow focus
        self.selection_manager.clear();
        self.selection_manager
            .add_cell(display_cell, ordered_indices);
    }

    /// Move focus in the specified direction with bounds checking.
    /// Automatically initializes focus if none exists.
    fn move_focus_direction(
        &mut self,
        direction: NavigationDirection,
        ordered_indices: &SortedIndices,
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
            self.selection_manager.clear();
            self.selection_manager.add_cell(new_cell, ordered_indices);
        }
    }

    /// Set selection anchor for range selection. Called when starting range selection.
    fn set_selection_anchor(&mut self) {
        if let Some(focused) = self.focused_cell {
            self.selection_anchor = Some(focused);
        }
    }

    /// Update selection from anchor to focused cell (rectangular selection).
    fn update_range_selection(&mut self, ordered_indices: &SortedIndices) {
        if let (Some(anchor), Some(focused)) = (self.selection_anchor, self.focused_cell) {
            self.selection_manager.clear();
            self.selection_manager
                .add_range(anchor, focused, ordered_indices);
        }
    }

    /// Move focus in the specified direction and extend selection.
    /// For range selection (shift + arrow keys).
    fn extend_selection_direction(
        &mut self,
        direction: NavigationDirection,
        ordered_indices: &SortedIndices,
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
    pub fn select_all(&mut self, max_rows: usize, max_cols: usize) {
        if max_rows == 0 || max_cols == 0 {
            return;
        }

        // Major optimization: O(1) select all instead of O(rows × cols)
        self.selection_manager.select_all();

        // Set focus to the first cell and anchor to the last cell in display coordinates
        self.focused_cell = Some(DisplayCellId::new(0, 0));
        self.selection_anchor = Some(DisplayCellId::new(max_rows - 1, max_cols - 1));
    }

    /// Jump focus to the top row (first row in display order)
    pub fn jump_to_top_edge(&mut self, ordered_indices: &SortedIndices) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        let new_cell = DisplayCellId::new(0, focused.col);
        self.focused_cell = Some(new_cell);
        self.selection_anchor = Some(new_cell);
        self.selection_manager.clear();
        self.selection_manager.add_cell(new_cell, ordered_indices);
    }

    /// Jump focus to the bottom row (last row in display order)
    pub fn jump_to_bottom_edge(&mut self, ordered_indices: &SortedIndices, max_rows: usize) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        if max_rows > 0 {
            let new_cell = DisplayCellId::new(max_rows - 1, focused.col);
            self.focused_cell = Some(new_cell);
            self.selection_anchor = Some(new_cell);
            self.selection_manager.clear();
            self.selection_manager.add_cell(new_cell, ordered_indices);
        }
    }

    /// Jump focus to the leftmost column (column 0)
    pub fn jump_to_left_edge(&mut self, ordered_indices: &SortedIndices) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        let new_cell = DisplayCellId::new(focused.row, 0);
        self.focused_cell = Some(new_cell);
        self.selection_anchor = Some(new_cell);
        self.selection_manager.clear();
        self.selection_manager.add_cell(new_cell, ordered_indices);
    }

    /// Jump focus to the rightmost column (last column)
    pub fn jump_to_right_edge(&mut self, ordered_indices: &SortedIndices, max_cols: usize) {
        let Some(focused) = self.focused_cell else {
            self.ensure_focus_initialized(ordered_indices);
            return;
        };

        if max_cols > 0 {
            let new_cell = DisplayCellId::new(focused.row, max_cols - 1);
            self.focused_cell = Some(new_cell);
            self.selection_anchor = Some(new_cell);
            self.selection_manager.clear();
            self.selection_manager.add_cell(new_cell, ordered_indices);
        }
    }

    /// Extend selection to the top row while keeping anchor
    pub fn extend_selection_to_top_edge(&mut self, ordered_indices: &SortedIndices) {
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
        ordered_indices: &SortedIndices,
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
    pub fn extend_selection_to_left_edge(&mut self, ordered_indices: &SortedIndices) {
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
        ordered_indices: &SortedIndices,
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
        ordered_indices: &SortedIndices,
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
        ordered_indices: &SortedIndices,
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
        ordered_indices: &SortedIndices,
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

/// Enum representing the offset of the focused cell in the list.
/// Used to keep viewport following the focused cell with some buffer, so mouse selection and autoscrollihng works correctly.
/// // TODO: rewrite this nonsence doc to be more clear
#[derive(Debug)]
pub enum ScrollOffset {
    NoOffset,
    Negative,
    Positive,
}

///// Selection related CsvPreviewView methods /////
impl CsvPreviewView {
    /// Unified navigation handler - eliminates code duplication
    pub(crate) fn handle_navigation(
        &mut self,
        direction: NavigationDirection,
        operation: NavigationOperation,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let start_time = Instant::now();
        let max_rows = self.contents.rows.len();
        let max_cols = self.contents.number_of_cols;

        self.selection.navigate(
            direction,
            operation,
            &self.sorted_indices,
            max_rows,
            max_cols,
        );

        self.performance_metrics.last_selection_took = Some(start_time.elapsed());
        let scroll = match direction {
            NavigationDirection::Up => Some(ScrollOffset::Negative),
            NavigationDirection::Down => Some(ScrollOffset::Positive),
            NavigationDirection::Left => None,
            NavigationDirection::Right => None,
        };

        // Update cell editor to show focused cell content
        self.on_selection_changed(window, cx, scroll);
        cx.notify();
    }

    /// Performs actions triggered by selection change
    pub(crate) fn on_selection_changed(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
        apply_scroll: Option<ScrollOffset>,
    ) {
        self.clear_cell_editor();

        // Follow focused cell in list viewport
        if let Some(focused_cell) = self.selection.get_focused_cell()
            && let Some(scroll) = apply_scroll
        {
            let display_row_index = focused_cell.row;
            let ix = display_row_index.0;

            match self.settings.rendering_with {
                crate::settings::RowRenderMechanism::VariableList => {
                    // Variable height list uses ListState::scroll_to_reveal_item
                    let ix_with_offset = match scroll {
                        ScrollOffset::NoOffset => ix,
                        ScrollOffset::Negative => ix.saturating_sub(2), // Avoid overflowing
                        ScrollOffset::Positive => ix + 2,
                    };
                    self.list_state.scroll_to_reveal_item(ix_with_offset);
                }
                crate::settings::RowRenderMechanism::UniformList => {
                    // Uniform list uses UniformListScrollHandle
                    let table_interaction_state = &self.table_interaction_state;
                    table_interaction_state.update(cx, |state, _| {
                        let ix_with_offset = match scroll {
                            ScrollOffset::NoOffset => ix,
                            ScrollOffset::Negative => ix.saturating_sub(2),
                            ScrollOffset::Positive => ix + 2,
                        };
                        // Use ScrollStrategy::Nearest for minimal scrolling (like scroll_to_reveal_item)
                        state.scroll_handle.scroll_to_item_with_offset(
                            ix_with_offset,
                            ScrollStrategy::Nearest,
                            0, // No additional offset since we already calculate it above
                        );
                    });
                }
            }
        }
    }
}
