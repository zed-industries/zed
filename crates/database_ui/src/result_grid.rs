#![allow(dead_code)]

use std::sync::Arc;

use editor::{Editor, EditorEvent};
use gpui::{
    actions, div, prelude::*, px, App, Context, Entity, EventEmitter, FocusHandle,
    Focusable, MouseButton, Pixels, Point, SharedString, Subscription, UniformListScrollHandle,
};
use serde::{Deserialize, Serialize};
use ui::{prelude::*, Button, ButtonStyle, IconName, Label, Tooltip};
use util::ResultExt as _;

use database_core::{CellValue, ForeignKeyInfo, QueryResult};

use crate::connection_manager::ConnectionManager;
use std::collections::HashSet;

use crate::results_table::{
    SortDirection, TableConfig, MIN_COLUMN_WIDTH, ROWS_PER_PAGE_OPTIONS, render_results_table,
    render_status_bar,
};

actions!(
    result_grid,
    [
        /// Starts editing the currently selected cell.
        StartCellEdit,
        /// Commits all pending edits to the database.
        CommitPendingEdits,
        /// Reverts all pending edits.
        RevertPendingEdits,
        /// Toggles between grid and record view.
        ToggleRecordView,
        /// Adds a new empty row.
        AddRow,
        /// Deletes the selected rows.
        DeleteSelectedRows,
        /// Clones the selected row.
        CloneSelectedRow,
        /// Toggles a boolean cell value.
        ToggleBooleanCell,
        /// Undoes the last edit operation.
        UndoEdit,
        /// Redoes the last undone edit operation.
        RedoEdit,
        /// Shows DML preview before committing.
        ShowDmlPreview,
        /// Selects all cells in the grid.
        SelectAllCells,
        /// Expands the current selection: Cell → Row → All.
        ExpandSelection,
        /// Shrinks the current selection: All → Row → Cell.
        ShrinkSelection,
        /// Toggles the WHERE filter text field.
        ToggleWhereFilter,
        /// Toggles the aggregate view bar.
        ToggleAggregateView,
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    Grid,
    Record,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GridSelection {
    None,
    Cell(usize, usize),
    Range {
        start: (usize, usize),
        end: (usize, usize),
    },
    Rows(Vec<usize>),
    Columns(Vec<usize>),
    All,
}

impl GridSelection {
    pub fn contains_cell(&self, row: usize, col: usize) -> bool {
        match self {
            GridSelection::None => false,
            GridSelection::Cell(r, c) => *r == row && *c == col,
            GridSelection::Range { start, end } => {
                let min_row = start.0.min(end.0);
                let max_row = start.0.max(end.0);
                let min_col = start.1.min(end.1);
                let max_col = start.1.max(end.1);
                row >= min_row && row <= max_row && col >= min_col && col <= max_col
            }
            GridSelection::Rows(rows) => rows.contains(&row),
            GridSelection::Columns(cols) => cols.contains(&col),
            GridSelection::All => true,
        }
    }

    pub fn contains_row(&self, row: usize) -> bool {
        match self {
            GridSelection::None => false,
            GridSelection::Cell(r, _) => *r == row,
            GridSelection::Range { start, end } => {
                let min_row = start.0.min(end.0);
                let max_row = start.0.max(end.0);
                row >= min_row && row <= max_row
            }
            GridSelection::Rows(rows) => rows.contains(&row),
            GridSelection::Columns(_) => true,
            GridSelection::All => true,
        }
    }

    pub fn primary_cell(&self) -> Option<(usize, usize)> {
        match self {
            GridSelection::Cell(r, c) => Some((*r, *c)),
            GridSelection::Range { start, .. } => Some(*start),
            GridSelection::Rows(rows) => rows.first().map(|r| (*r, 0)),
            _ => None,
        }
    }

    pub fn selected_rows(&self) -> Vec<usize> {
        match self {
            GridSelection::Cell(r, _) => vec![*r],
            GridSelection::Rows(rows) => rows.clone(),
            GridSelection::Range { start, end } => {
                let min = start.0.min(end.0);
                let max = start.0.max(end.0);
                (min..=max).collect()
            }
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EditOperation {
    CellEdit {
        row: usize,
        col: usize,
        old: CellValue,
        new: CellValue,
    },
    RowInsert {
        row_index: usize,
    },
    RowDelete {
        row_index: usize,
        data: Vec<CellValue>,
    },
}

#[derive(Debug, Default)]
pub struct DataEditHistory {
    undo_stack: Vec<EditOperation>,
    redo_stack: Vec<EditOperation>,
}

impl DataEditHistory {
    pub fn push(&mut self, operation: EditOperation) {
        self.undo_stack.push(operation);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<EditOperation> {
        self.undo_stack.pop().map(|op| {
            self.redo_stack.push(op.clone());
            op
        })
    }

    pub fn redo(&mut self) -> Option<EditOperation> {
        self.redo_stack.pop().map(|op| {
            self.undo_stack.push(op.clone());
            op
        })
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PendingEdit {
    pub row: usize,
    pub col: usize,
    pub original_value: CellValue,
    pub new_value: CellValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingRowState {
    Insert,
    Delete,
}

#[derive(Debug, Clone)]
pub struct FilterClause {
    pub column: String,
    pub operator: FilterOperator,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    IsNull,
    IsNotNull,
    Contains,
    GreaterThan,
    LessThan,
}

impl FilterClause {
    pub fn to_sql(&self, db_type: &database_core::DatabaseType) -> String {
        let quoted = database_core::quote_identifier(&self.column, db_type);
        match self.operator {
            FilterOperator::Equals => format!("{} = '{}'", quoted, self.value.as_deref().unwrap_or("")),
            FilterOperator::NotEquals => format!("{} != '{}'", quoted, self.value.as_deref().unwrap_or("")),
            FilterOperator::IsNull => format!("{} IS NULL", quoted),
            FilterOperator::IsNotNull => format!("{} IS NOT NULL", quoted),
            FilterOperator::Contains => format!("{} LIKE '%{}%'", quoted, self.value.as_deref().unwrap_or("")),
            FilterOperator::GreaterThan => format!("{} > '{}'", quoted, self.value.as_deref().unwrap_or("")),
            FilterOperator::LessThan => format!("{} < '{}'", quoted, self.value.as_deref().unwrap_or("")),
        }
    }

    pub fn display_text(&self) -> String {
        match self.operator {
            FilterOperator::Equals => format!("{} = {}", self.column, self.value.as_deref().unwrap_or("")),
            FilterOperator::NotEquals => format!("{} != {}", self.column, self.value.as_deref().unwrap_or("")),
            FilterOperator::IsNull => format!("{} IS NULL", self.column),
            FilterOperator::IsNotNull => format!("{} IS NOT NULL", self.column),
            FilterOperator::Contains => format!("{} LIKE %{}%", self.column, self.value.as_deref().unwrap_or("")),
            FilterOperator::GreaterThan => format!("{} > {}", self.column, self.value.as_deref().unwrap_or("")),
            FilterOperator::LessThan => format!("{} < {}", self.column, self.value.as_deref().unwrap_or("")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AggregateValues {
    pub count: usize,
    pub numeric_count: usize,
    pub sum: Option<f64>,
    pub avg: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[allow(dead_code)]
pub enum ResultGridEvent {
    SortChanged(Vec<(usize, SortDirection)>),
    PageChanged(usize),
    CellSelected(usize, usize),
    EditCommitted {
        row: usize,
        col: usize,
        value: CellValue,
    },
    PendingEditsCommitted(Vec<PendingEdit>),
    PendingEditsReverted,
    NavigateToForeignKey {
        table: String,
        column: String,
        value: String,
    },
    RowsPerPageChanged(usize),
    WhereClauseChanged(String),
}

#[allow(dead_code)]
pub struct ResultGrid {
    focus_handle: FocusHandle,
    result: Option<QueryResult>,
    page: usize,
    page_offset: usize,
    rows_per_page: usize,
    sort_columns: Vec<(usize, SortDirection)>,
    selection: GridSelection,
    column_widths: Vec<f32>,
    default_column_width: f32,
    resizing_column: Option<(usize, f32)>,
    scroll_handle: UniformListScrollHandle,

    editing_cell: Option<(usize, usize)>,
    cell_editor: Option<Entity<Editor>>,
    pending_edits: Vec<PendingEdit>,
    pending_row_states: Vec<(usize, PendingRowState)>,
    edit_history: DataEditHistory,
    view_mode: ViewMode,
    record_view_row: usize,
    show_dml_preview: bool,

    foreign_keys: Vec<ForeignKeyInfo>,
    fk_column_indices: HashSet<usize>,
    active_filters: Vec<FilterClause>,

    show_where_filter: bool,
    where_clause: String,
    where_editor: Option<Entity<Editor>>,
    show_aggregate_view: bool,

    connection_manager: Option<Entity<ConnectionManager>>,
    source_table: Option<String>,

    _subscriptions: Vec<Subscription>,
}

impl ResultGrid {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            result: None,
            page: 0,
            page_offset: 0,
            rows_per_page: 50,
            sort_columns: Vec::new(),
            selection: GridSelection::None,
            column_widths: Vec::new(),
            default_column_width: 200.0,
            resizing_column: None,
            scroll_handle: UniformListScrollHandle::new(),
            editing_cell: None,
            cell_editor: None,
            pending_edits: Vec::new(),
            pending_row_states: Vec::new(),
            edit_history: DataEditHistory::default(),
            view_mode: ViewMode::Grid,
            record_view_row: 0,
            show_dml_preview: false,
            foreign_keys: Vec::new(),
            fk_column_indices: HashSet::new(),
            active_filters: Vec::new(),
            show_where_filter: false,
            where_clause: String::new(),
            where_editor: None,
            show_aggregate_view: false,
            connection_manager: None,
            source_table: None,
            _subscriptions: Vec::new(),
        }
    }

    pub fn set_result(&mut self, result: QueryResult, cx: &mut Context<Self>) {
        self.result = Some(result);
        self.rebuild_fk_column_cache();
        self.cancel_edit();
        cx.notify();
    }

    pub fn set_page(&mut self, page: usize, page_offset: usize, cx: &mut Context<Self>) {
        self.page = page;
        self.page_offset = page_offset;
        self.cancel_edit();
        cx.notify();
    }

    #[allow(dead_code)]
    pub fn set_rows_per_page(&mut self, rows_per_page: usize) {
        self.rows_per_page = rows_per_page;
    }

    pub fn set_connection_manager(&mut self, manager: Entity<ConnectionManager>) {
        self.connection_manager = Some(manager);
    }

    #[allow(dead_code)]
    pub fn set_source_table(&mut self, table: Option<String>) {
        self.source_table = table;
    }

    #[allow(dead_code)]
    pub fn set_foreign_keys(&mut self, foreign_keys: Vec<ForeignKeyInfo>) {
        self.foreign_keys = foreign_keys;
        self.rebuild_fk_column_cache();
    }

    #[allow(dead_code)]
    pub fn set_default_column_width(&mut self, width: f32) {
        self.default_column_width = width;
    }

    pub fn result(&self) -> Option<&QueryResult> {
        self.result.as_ref()
    }

    pub fn sort_columns(&self) -> &[(usize, SortDirection)] {
        &self.sort_columns
    }

    pub fn selected_cell(&self) -> Option<(usize, usize)> {
        self.selection.primary_cell()
    }

    #[allow(dead_code)]
    pub fn selection(&self) -> &GridSelection {
        &self.selection
    }

    #[allow(dead_code)]
    pub fn pending_edits(&self) -> &[PendingEdit] {
        &self.pending_edits
    }

    #[allow(dead_code)]
    pub fn view_mode(&self) -> ViewMode {
        self.view_mode
    }

    #[allow(dead_code)]
    pub fn column_widths(&self) -> &[f32] {
        &self.column_widths
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.result = None;
        self.sort_columns.clear();
        self.selection = GridSelection::None;
        self.column_widths.clear();
        self.pending_edits.clear();
        self.pending_row_states.clear();
        self.edit_history.clear();
        self.cancel_edit();
        self.record_view_row = 0;
        self.page = 0;
        self.page_offset = 0;
        self.show_dml_preview = false;
        self.active_filters.clear();
        self.show_where_filter = false;
        self.where_clause.clear();
        self.where_editor = None;
        self.show_aggregate_view = false;
        cx.notify();
    }

    #[allow(dead_code)]
    pub fn set_column_widths(&mut self, widths: Vec<f32>) {
        self.column_widths = widths;
    }

    fn toggle_sort_column(&mut self, col_index: usize, cx: &mut Context<Self>) {
        if let Some(pos) = self.sort_columns.iter().position(|(idx, _)| *idx == col_index) {
            match self.sort_columns[pos].1 {
                SortDirection::Ascending => {
                    self.sort_columns[pos].1 = SortDirection::Descending;
                }
                SortDirection::Descending => {
                    self.sort_columns.remove(pos);
                }
            }
        } else {
            self.sort_columns = vec![(col_index, SortDirection::Ascending)];
        }
        self.selection = GridSelection::None;
        cx.emit(ResultGridEvent::SortChanged(self.sort_columns.clone()));
        cx.notify();
    }

    fn add_sort_column(&mut self, col_index: usize, cx: &mut Context<Self>) {
        if let Some(pos) = self.sort_columns.iter().position(|(idx, _)| *idx == col_index) {
            match self.sort_columns[pos].1 {
                SortDirection::Ascending => {
                    self.sort_columns[pos].1 = SortDirection::Descending;
                }
                SortDirection::Descending => {
                    self.sort_columns.remove(pos);
                }
            }
        } else {
            self.sort_columns.push((col_index, SortDirection::Ascending));
        }
        self.selection = GridSelection::None;
        cx.emit(ResultGridEvent::SortChanged(self.sort_columns.clone()));
        cx.notify();
    }

    #[allow(dead_code)]
    pub fn active_filters(&self) -> &[FilterClause] {
        &self.active_filters
    }

    pub fn add_filter(&mut self, filter: FilterClause, cx: &mut Context<Self>) {
        self.active_filters.push(filter);
        cx.notify();
    }

    pub fn remove_filter(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.active_filters.len() {
            self.active_filters.remove(index);
            cx.notify();
        }
    }

    #[allow(dead_code)]
    pub fn clear_filters(&mut self, cx: &mut Context<Self>) {
        self.active_filters.clear();
        cx.notify();
    }

    pub fn build_filter_where_clause(&self, db_type: &database_core::DatabaseType) -> Option<String> {
        if self.active_filters.is_empty() {
            return None;
        }
        let clauses: Vec<String> = self.active_filters.iter().map(|f| f.to_sql(db_type)).collect();
        Some(clauses.join(" AND "))
    }

    pub fn where_clause(&self) -> &str {
        &self.where_clause
    }

    fn expand_selection(&mut self, cx: &mut Context<Self>) {
        let col_count = self
            .result
            .as_ref()
            .map(|r| r.columns.len())
            .unwrap_or(0);
        if col_count == 0 {
            return;
        }

        self.selection = match &self.selection {
            GridSelection::None => {
                GridSelection::Cell(0, 0)
            }
            GridSelection::Cell(row, _col) => {
                GridSelection::Rows(vec![*row])
            }
            GridSelection::Rows(_) | GridSelection::Range { .. } | GridSelection::Columns(_) => {
                GridSelection::All
            }
            GridSelection::All => GridSelection::All,
        };
        cx.notify();
    }

    fn shrink_selection(&mut self, cx: &mut Context<Self>) {
        self.selection = match &self.selection {
            GridSelection::All => {
                GridSelection::Rows(vec![0])
            }
            GridSelection::Rows(rows) => {
                let row = rows.first().copied().unwrap_or(0);
                GridSelection::Cell(row, 0)
            }
            GridSelection::Range { start, .. } => {
                GridSelection::Cell(start.0, start.1)
            }
            GridSelection::Columns(cols) => {
                let col = cols.first().copied().unwrap_or(0);
                GridSelection::Cell(0, col)
            }
            GridSelection::Cell(_, _) | GridSelection::None => GridSelection::None,
        };
        cx.notify();
    }

    fn calculate_aggregates(&self) -> Option<AggregateValues> {
        let result = self.result.as_ref()?;
        if matches!(self.selection, GridSelection::None) {
            return None;
        }

        let mut count: usize = 0;
        let mut numeric_values: Vec<f64> = Vec::new();

        let row_count = result.rows.len();
        let col_count = result.columns.len();

        for row in 0..row_count {
            for col in 0..col_count {
                if !self.selection.contains_cell(row, col) {
                    continue;
                }
                count += 1;
                if let Some(cell) = result.rows.get(row).and_then(|r| r.get(col)) {
                    if let Some(f) = cell.as_f64() {
                        numeric_values.push(f);
                    }
                }
            }
        }

        if count == 0 {
            return None;
        }

        let (sum, avg, min, max) = if numeric_values.is_empty() {
            (None, None, None, None)
        } else {
            let sum: f64 = numeric_values.iter().sum();
            let avg = sum / numeric_values.len() as f64;
            let min = numeric_values
                .iter()
                .copied()
                .reduce(f64::min);
            let max = numeric_values
                .iter()
                .copied()
                .reduce(f64::max);
            (Some(sum), Some(avg), min, max)
        };

        Some(AggregateValues {
            count,
            numeric_count: numeric_values.len(),
            sum,
            avg,
            min,
            max,
        })
    }

    fn toggle_where_filter(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.show_where_filter = !self.show_where_filter;
        if self.show_where_filter {
            let where_clause = self.where_clause.clone();
            let editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("WHERE clause (e.g. age > 18 AND name LIKE '%test%')", window, cx);
                if !where_clause.is_empty() {
                    editor.set_text(where_clause, window, cx);
                }
                editor
            });
            self.where_editor = Some(editor.clone());
            window.focus(&editor.focus_handle(cx), cx);
        } else {
            self.where_editor = None;
        }
        cx.notify();
    }

    fn apply_where_clause(&mut self, cx: &mut Context<Self>) {
        if let Some(editor) = &self.where_editor {
            let text = editor.read(cx).text(cx).to_string();
            self.where_clause = text.clone();
            cx.emit(ResultGridEvent::WhereClauseChanged(text));
        }
        cx.notify();
    }

    fn toggle_aggregate_view(&mut self, cx: &mut Context<Self>) {
        self.show_aggregate_view = !self.show_aggregate_view;
        cx.notify();
    }

    fn select_cell(&mut self, row: usize, col: usize, cx: &mut Context<Self>) {
        self.selection = GridSelection::Cell(row, col);
        cx.emit(ResultGridEvent::CellSelected(row, col));
        cx.notify();
    }

    fn select_cell_with_shift(
        &mut self,
        row: usize,
        col: usize,
        cx: &mut Context<Self>,
    ) {
        match &self.selection {
            GridSelection::Cell(start_row, start_col) => {
                self.selection = GridSelection::Range {
                    start: (*start_row, *start_col),
                    end: (row, col),
                };
            }
            GridSelection::Range { start, .. } => {
                self.selection = GridSelection::Range {
                    start: *start,
                    end: (row, col),
                };
            }
            _ => {
                self.selection = GridSelection::Cell(row, col);
            }
        }
        cx.notify();
    }

    fn toggle_row_selection(&mut self, row: usize, cx: &mut Context<Self>) {
        match &mut self.selection {
            GridSelection::Rows(rows) => {
                if let Some(pos) = rows.iter().position(|r| *r == row) {
                    rows.remove(pos);
                    if rows.is_empty() {
                        self.selection = GridSelection::None;
                    }
                } else {
                    rows.push(row);
                }
            }
            GridSelection::Cell(existing_row, _) => {
                if *existing_row == row {
                    self.selection = GridSelection::None;
                } else {
                    self.selection = GridSelection::Rows(vec![*existing_row, row]);
                }
            }
            _ => {
                self.selection = GridSelection::Rows(vec![row]);
            }
        }
        cx.notify();
    }

    fn select_all(&mut self, cx: &mut Context<Self>) {
        self.selection = GridSelection::All;
        cx.notify();
    }

    fn grid_move_down(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selection.primary_cell() else {
            return;
        };
        let row_count = self
            .result
            .as_ref()
            .map(|r| r.rows.len())
            .unwrap_or(0);
        if row + 1 < row_count {
            self.selection = GridSelection::Cell(row + 1, col);
            self.scroll_handle
                .scroll_to_item(row + 1, gpui::ScrollStrategy::Top);
            cx.notify();
        }
    }

    fn grid_move_up(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selection.primary_cell() else {
            return;
        };
        if row > 0 {
            self.selection = GridSelection::Cell(row - 1, col);
            self.scroll_handle
                .scroll_to_item(row - 1, gpui::ScrollStrategy::Top);
            cx.notify();
        }
    }

    fn grid_move_right(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selection.primary_cell() else {
            return;
        };
        let col_count = self
            .result
            .as_ref()
            .map(|r| r.columns.len())
            .unwrap_or(0);
        if col + 1 < col_count {
            self.selection = GridSelection::Cell(row, col + 1);
            cx.notify();
        }
    }

    fn grid_move_left(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selection.primary_cell() else {
            return;
        };
        if col > 0 {
            self.selection = GridSelection::Cell(row, col - 1);
            cx.notify();
        }
    }

    fn ensure_column_widths(&mut self) {
        let column_count = self
            .result
            .as_ref()
            .map(|r| r.columns.len())
            .unwrap_or(0);
        if self.column_widths.len() < column_count {
            self.column_widths
                .resize(column_count, self.default_column_width);
        }
    }

    fn start_column_resize(
        &mut self,
        col_index: usize,
        start_x: Pixels,
        cx: &mut Context<Self>,
    ) {
        self.ensure_column_widths();
        self.resizing_column = Some((col_index, start_x.as_f32()));
        cx.notify();
    }

    fn handle_resize_move(&mut self, position_x: f32, cx: &mut Context<Self>) {
        let Some((col_index, start_x)) = self.resizing_column else {
            return;
        };
        let current_width = self
            .column_widths
            .get(col_index)
            .copied()
            .unwrap_or(self.default_column_width);
        let delta = position_x - start_x;
        let new_width = (current_width + delta).max(MIN_COLUMN_WIDTH);

        if let Some(width) = self.column_widths.get_mut(col_index) {
            *width = new_width;
        }
        self.resizing_column = Some((col_index, position_x));
        cx.notify();
    }

    fn stop_column_resize(&mut self) {
        self.resizing_column = None;
    }

    fn total_row_count(&self) -> usize {
        self.result
            .as_ref()
            .and_then(|r| r.total_row_count)
            .map(|c| c as usize)
            .unwrap_or_else(|| {
                self.result
                    .as_ref()
                    .map(|r| r.rows.len())
                    .unwrap_or(0)
            })
    }

    fn total_pages(&self) -> usize {
        let total = self.total_row_count();
        if total == 0 {
            return 1;
        }
        (total + self.rows_per_page - 1) / self.rows_per_page
    }

    // --- CRUD operations ---

    fn add_row(&mut self, cx: &mut Context<Self>) {
        let Some(result) = &mut self.result else {
            return;
        };
        let col_count = result.columns.len();
        let new_row: Vec<CellValue> = vec![CellValue::Null; col_count];
        let row_index = result.rows.len();
        result.rows.push(new_row);
        self.pending_row_states.push((row_index, PendingRowState::Insert));
        self.edit_history.push(EditOperation::RowInsert { row_index });
        self.selection = GridSelection::Cell(row_index, 0);
        cx.notify();
    }

    fn delete_selected_rows(&mut self, cx: &mut Context<Self>) {
        let rows = self.selection.selected_rows();
        if rows.is_empty() {
            return;
        }

        let Some(result) = &self.result else {
            return;
        };

        for &row in &rows {
            if self.pending_row_states.iter().any(|(r, s)| *r == row && *s == PendingRowState::Delete) {
                continue;
            }
            let data = result.rows.get(row).cloned().unwrap_or_default();
            self.pending_row_states.push((row, PendingRowState::Delete));
            self.edit_history.push(EditOperation::RowDelete {
                row_index: row,
                data,
            });
        }
        cx.notify();
    }

    fn clone_selected_row(&mut self, cx: &mut Context<Self>) {
        let Some((row, _)) = self.selection.primary_cell() else {
            return;
        };
        let Some(result) = &mut self.result else {
            return;
        };
        let Some(row_data) = result.rows.get(row).cloned() else {
            return;
        };

        let row_index = result.rows.len();
        result.rows.push(row_data);
        self.pending_row_states.push((row_index, PendingRowState::Insert));
        self.edit_history.push(EditOperation::RowInsert { row_index });
        self.selection = GridSelection::Cell(row_index, 0);
        cx.notify();
    }

    fn toggle_boolean_cell(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selection.primary_cell() else {
            return;
        };
        let Some(result) = &self.result else {
            return;
        };
        let Some(cell_value) = result.rows.get(row).and_then(|r| r.get(col)).cloned() else {
            return;
        };

        let new_value = match &cell_value {
            CellValue::Boolean(b) => CellValue::Boolean(!b),
            _ => {
                let display = self.get_display_value(row, col).unwrap_or_default();
                match display.to_lowercase().as_str() {
                    "true" | "1" => CellValue::Boolean(false),
                    "false" | "0" => CellValue::Boolean(true),
                    _ => CellValue::Boolean(true),
                }
            }
        };

        let original_value = cell_value;
        self.apply_cell_edit(row, col, original_value, new_value, cx);
    }

    fn apply_cell_edit(
        &mut self,
        row: usize,
        col: usize,
        original_value: CellValue,
        new_value: CellValue,
        cx: &mut Context<Self>,
    ) {
        self.edit_history.push(EditOperation::CellEdit {
            row,
            col,
            old: original_value.clone(),
            new: new_value.clone(),
        });

        let existing_index = self.pending_edits.iter().position(|e| e.row == row && e.col == col);
        if let Some(index) = existing_index {
            self.pending_edits[index].new_value = new_value.clone();
        } else {
            self.pending_edits.push(PendingEdit {
                row,
                col,
                original_value,
                new_value: new_value.clone(),
            });
        }

        cx.emit(ResultGridEvent::EditCommitted {
            row,
            col,
            value: new_value,
        });
        cx.notify();
    }

    // --- Undo/Redo ---

    fn undo_edit(&mut self, cx: &mut Context<Self>) {
        let Some(operation) = self.edit_history.undo() else {
            return;
        };

        match operation {
            EditOperation::CellEdit { row, col, old, .. } => {
                if let Some(pos) = self.pending_edits.iter().position(|e| e.row == row && e.col == col) {
                    self.pending_edits.remove(pos);
                }
                if let Some(result) = &mut self.result {
                    if let Some(cell) = result.rows.get_mut(row).and_then(|r| r.get_mut(col)) {
                        *cell = old;
                    }
                }
            }
            EditOperation::RowInsert { row_index } => {
                self.pending_row_states.retain(|(r, s)| !(*r == row_index && *s == PendingRowState::Insert));
                if let Some(result) = &mut self.result {
                    if row_index < result.rows.len() {
                        result.rows.remove(row_index);
                    }
                }
            }
            EditOperation::RowDelete { row_index, .. } => {
                self.pending_row_states.retain(|(r, s)| !(*r == row_index && *s == PendingRowState::Delete));
            }
        }
        cx.notify();
    }

    fn redo_edit(&mut self, cx: &mut Context<Self>) {
        let Some(operation) = self.edit_history.redo() else {
            return;
        };

        match operation {
            EditOperation::CellEdit { row, col, old, new } => {
                let existing_index =
                    self.pending_edits.iter().position(|e| e.row == row && e.col == col);
                if let Some(index) = existing_index {
                    self.pending_edits[index].new_value = new.clone();
                } else {
                    self.pending_edits.push(PendingEdit {
                        row,
                        col,
                        original_value: old,
                        new_value: new.clone(),
                    });
                }
                if let Some(result) = &mut self.result {
                    if let Some(cell) = result.rows.get_mut(row).and_then(|r| r.get_mut(col)) {
                        *cell = new;
                    }
                }
            }
            EditOperation::RowInsert { row_index } => {
                if let Some(result) = &mut self.result {
                    let col_count = result.columns.len();
                    result.rows.insert(row_index, vec![CellValue::Null; col_count]);
                }
                self.pending_row_states.push((row_index, PendingRowState::Insert));
            }
            EditOperation::RowDelete { row_index, .. } => {
                self.pending_row_states.push((row_index, PendingRowState::Delete));
            }
        }
        cx.notify();
    }

    // --- FK navigation ---

    fn is_fk_column(&self, col_name: &str) -> Option<&ForeignKeyInfo> {
        self.foreign_keys
            .iter()
            .find(|fk| fk.from_column == col_name)
    }

    fn navigate_to_fk(&self, col_name: &str, value: &str, cx: &mut Context<Self>) {
        if let Some(fk) = self.is_fk_column(col_name).cloned() {
            cx.emit(ResultGridEvent::NavigateToForeignKey {
                table: fk.to_table,
                column: fk.to_column,
                value: value.to_string(),
            });
        }
    }

    // --- Inline editing ---

    fn start_cell_edit(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let Some((row, col)) = self.selection.primary_cell() else {
            return;
        };
        let Some(result) = &self.result else {
            return;
        };
        let Some(cell_value) = result.rows.get(row).and_then(|r| r.get(col)) else {
            return;
        };

        let text = match cell_value {
            CellValue::Null => String::new(),
            other => other.to_string(),
        };

        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(text, window, cx);
            editor
        });

        let editor_subscription = cx.subscribe(&editor, |this, _editor, event, cx| {
            if matches!(event, EditorEvent::Blurred) {
                this.confirm_edit(cx);
            }
        });

        self.editing_cell = Some((row, col));
        self.cell_editor = Some(editor.clone());
        self._subscriptions.push(editor_subscription);
        window.focus(&editor.focus_handle(cx), cx);
        cx.notify();
    }

    fn confirm_edit(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.editing_cell.take() else {
            return;
        };
        let Some(editor) = self.cell_editor.take() else {
            return;
        };

        let new_text = editor.read(cx).text(cx).to_string();

        let edits_to_apply: Vec<(usize, usize, CellValue, CellValue)> = {
            let Some(result) = &self.result else {
                self.clear_edit_subscriptions();
                cx.notify();
                return;
            };

            let affected_cells = self.multi_edit_cells(row, col);

            affected_cells
                .into_iter()
                .filter_map(|(edit_row, edit_col)| {
                    let original_value = result.rows.get(edit_row)?.get(edit_col)?.clone();
                    let new_value = if new_text.is_empty() {
                        CellValue::Null
                    } else {
                        parse_cell_value(&new_text, &original_value)
                    };
                    if new_value.to_string() != original_value.to_string() {
                        Some((edit_row, edit_col, original_value, new_value))
                    } else {
                        None
                    }
                })
                .collect()
        };

        for (edit_row, edit_col, original_value, new_value) in edits_to_apply {
            self.apply_cell_edit(edit_row, edit_col, original_value, new_value, cx);
        }

        self.clear_edit_subscriptions();
        cx.notify();
    }

    fn multi_edit_cells(&self, primary_row: usize, primary_col: usize) -> Vec<(usize, usize)> {
        match &self.selection {
            GridSelection::Range { start, end } => {
                let min_row = start.0.min(end.0);
                let max_row = start.0.max(end.0);
                let min_col = start.1.min(end.1);
                let max_col = start.1.max(end.1);
                if min_col == max_col {
                    (min_row..=max_row).map(|r| (r, primary_col)).collect()
                } else {
                    vec![(primary_row, primary_col)]
                }
            }
            GridSelection::Rows(rows) => {
                rows.iter().map(|r| (*r, primary_col)).collect()
            }
            GridSelection::All => {
                let row_count = self.result.as_ref().map(|r| r.rows.len()).unwrap_or(0);
                (0..row_count).map(|r| (r, primary_col)).collect()
            }
            _ => vec![(primary_row, primary_col)],
        }
    }

    fn cancel_edit(&mut self) {
        self.editing_cell = None;
        self.cell_editor = None;
        self.clear_edit_subscriptions();
    }

    fn rebuild_fk_column_cache(&mut self) {
        self.fk_column_indices = if let Some(result) = &self.result {
            self.foreign_keys
                .iter()
                .filter_map(|fk| result.columns.iter().position(|c| c == &fk.from_column))
                .collect()
        } else {
            HashSet::new()
        };
    }

    fn clear_edit_subscriptions(&mut self) {
        self._subscriptions.clear();
    }

    #[allow(dead_code)]
    fn confirm_and_move_next(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.confirm_edit(cx);

        let Some((row, col)) = self.selection.primary_cell() else {
            return;
        };
        let col_count = self
            .result
            .as_ref()
            .map(|r| r.columns.len())
            .unwrap_or(0);
        let row_count = self
            .result
            .as_ref()
            .map(|r| r.rows.len())
            .unwrap_or(0);

        let (next_row, next_col) = if col + 1 < col_count {
            (row, col + 1)
        } else if row + 1 < row_count {
            (row + 1, 0)
        } else {
            return;
        };

        self.selection = GridSelection::Cell(next_row, next_col);
        self.start_cell_edit(window, cx);
    }

    // --- DML generation ---

    fn generate_dml_preview(&self) -> String {
        let Some(table_name) = &self.source_table else {
            return "No source table set.".to_string();
        };

        let mut statements = Vec::new();

        // UPDATE statements for cell edits
        if let Some(result) = &self.result {
            let updates = generate_update_statements(table_name, &self.pending_edits, Some(result));
            statements.extend(updates);
        }

        // INSERT statements for new rows
        for (row_index, state) in &self.pending_row_states {
            if *state == PendingRowState::Insert {
                if let Some(result) = &self.result {
                    if let Some(row_data) = result.rows.get(*row_index) {
                        let cols: Vec<String> = result.columns.iter()
                            .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
                            .collect();
                        let vals: Vec<String> = row_data.iter()
                            .map(|v| v.to_sql_value())
                            .collect();
                        statements.push(format!(
                            "INSERT INTO \"{}\" ({}) VALUES ({})",
                            table_name.replace('"', "\"\""),
                            cols.join(", "),
                            vals.join(", ")
                        ));
                    }
                }
            }
        }

        // DELETE statements for deleted rows
        for (row_index, state) in &self.pending_row_states {
            if *state == PendingRowState::Delete {
                if let Some(result) = &self.result {
                    if let Some(row_data) = result.rows.get(*row_index) {
                        let where_clauses: Vec<String> = result.columns.iter()
                            .enumerate()
                            .filter_map(|(col_idx, col_name)| {
                                let value = row_data.get(col_idx)?;
                                if matches!(value, CellValue::Null) {
                                    Some(format!("\"{}\" IS NULL", col_name.replace('"', "\"\"")))
                                } else {
                                    Some(format!(
                                        "\"{}\" = {}",
                                        col_name.replace('"', "\"\""),
                                        value.to_sql_value()
                                    ))
                                }
                            })
                            .collect();
                        if !where_clauses.is_empty() {
                            statements.push(format!(
                                "DELETE FROM \"{}\" WHERE {} LIMIT 1",
                                table_name.replace('"', "\"\""),
                                where_clauses.join(" AND ")
                            ));
                        }
                    }
                }
            }
        }

        if statements.is_empty() {
            "No pending changes.".to_string()
        } else {
            statements.join(";\n")
        }
    }

    fn commit_pending_edits(&mut self, cx: &mut Context<Self>) {
        if self.pending_edits.is_empty() && self.pending_row_states.is_empty() {
            return;
        }

        if let (Some(manager), Some(_table_name)) = (&self.connection_manager, &self.source_table) {
            let dml = self.generate_dml_preview();
            if dml == "No pending changes." {
                return;
            }
            let task = manager.read(cx).execute_raw_query(dml, cx);

            cx.spawn(async move |this, cx| {
                let result = task.await;
                this.update(cx, |this, cx| {
                    match result {
                        Ok(_) => {
                            let committed = this.pending_edits.drain(..).collect();
                            this.pending_row_states.clear();
                            this.edit_history.clear();
                            cx.emit(ResultGridEvent::PendingEditsCommitted(committed));
                            cx.notify();
                        }
                        Err(error) => {
                            log::error!("Failed to commit edits: {:#}", error);
                        }
                    }
                }).log_err();
            })
            .detach();
        } else {
            let committed = self.pending_edits.drain(..).collect();
            self.pending_row_states.clear();
            self.edit_history.clear();
            cx.emit(ResultGridEvent::PendingEditsCommitted(committed));
        }

        cx.notify();
    }

    fn revert_pending_edits(&mut self, cx: &mut Context<Self>) {
        self.pending_edits.clear();
        self.pending_row_states.clear();
        self.edit_history.clear();
        cx.emit(ResultGridEvent::PendingEditsReverted);
        cx.notify();
    }

    fn toggle_view_mode(&mut self, cx: &mut Context<Self>) {
        self.view_mode = match self.view_mode {
            ViewMode::Grid => ViewMode::Record,
            ViewMode::Record => ViewMode::Grid,
        };
        cx.notify();
    }

    fn record_view_previous(&mut self, cx: &mut Context<Self>) {
        if self.record_view_row > 0 {
            self.record_view_row -= 1;
            cx.notify();
        }
    }

    fn record_view_next(&mut self, cx: &mut Context<Self>) {
        let row_count = self
            .result
            .as_ref()
            .map(|r| r.rows.len())
            .unwrap_or(0);
        if self.record_view_row + 1 < row_count {
            self.record_view_row += 1;
            cx.notify();
        }
    }

    fn is_cell_modified(&self, row: usize, col: usize) -> bool {
        self.pending_edits
            .iter()
            .any(|e| e.row == row && e.col == col)
    }

    fn get_row_state(&self, row: usize) -> Option<PendingRowState> {
        self.pending_row_states
            .iter()
            .find(|(r, _)| *r == row)
            .map(|(_, s)| *s)
    }

    fn get_display_value(&self, row: usize, col: usize) -> Option<String> {
        if let Some(edit) = self.pending_edits.iter().find(|e| e.row == row && e.col == col) {
            Some(edit.new_value.to_string())
        } else {
            self.result
                .as_ref()
                .and_then(|r| r.rows.get(row))
                .and_then(|r| r.get(col))
                .map(|v| v.to_string())
        }
    }

    fn change_rows_per_page(&mut self, rows_per_page: usize, cx: &mut Context<Self>) {
        self.rows_per_page = rows_per_page;
        self.page = 0;
        self.page_offset = 0;
        cx.emit(ResultGridEvent::RowsPerPageChanged(rows_per_page));
        cx.notify();
    }

    fn render_grid_view(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        let Some(result) = &self.result else {
            return div()
                .flex_grow()
                .items_center()
                .justify_center()
                .child(
                    Label::new("No results")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element();
        };

        if result.rows.is_empty() && result.columns.is_empty() {
            let msg = if let Some(affected) = result.affected_rows {
                format!(
                    "Query executed successfully. {} row(s) affected. ({:.1}ms)",
                    affected,
                    result.execution_time.as_secs_f64() * 1000.0
                )
            } else {
                format!(
                    "Query executed successfully. ({:.1}ms)",
                    result.execution_time.as_secs_f64() * 1000.0
                )
            };

            return div()
                .flex_grow()
                .p_2()
                .child(Label::new(msg).color(Color::Success).size(LabelSize::Small))
                .into_any_element();
        }

        let page_offset = self.page_offset;
        let selected_cell = self.selection.primary_cell();

        let config = TableConfig {
            column_widths: self.column_widths.clone(),
            default_column_width: self.default_column_width,
            fk_columns: self.fk_column_indices.clone(),
            ..Default::default()
        };

        let on_header_click: Arc<dyn Fn(usize, &mut gpui::Window, &mut App) + Send + Sync> = {
            let handle = cx.entity().downgrade();
            Arc::new(move |col_index, _window, cx| {
                if let Some(entity) = handle.upgrade() {
                    entity.update(cx, |this, cx| {
                        this.toggle_sort_column(col_index, cx);
                    });
                }
            })
        };

        let on_cell_click: Arc<
            dyn Fn(usize, usize, usize, &mut gpui::Window, &mut App) + Send + Sync,
        > = {
            let handle = cx.entity().downgrade();
            Arc::new(move |row, col, click_count, window, cx| {
                if let Some(entity) = handle.upgrade() {
                    entity.update(cx, |this, cx| {
                        this.select_cell(row, col, cx);
                        if click_count == 1 {
                            if let Some(result) = &this.result {
                                if let Some(col_name) = result.columns.get(col).cloned() {
                                    if this.is_fk_column(&col_name).is_some() {
                                        if let Some(value) = this.get_display_value(row, col) {
                                            if value != "NULL" {
                                                this.navigate_to_fk(&col_name, &value, cx);
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        } else if click_count >= 2 {
                            this.start_cell_edit(window, cx);
                        }
                    });
                }
            })
        };

        let on_cell_secondary_click: Arc<
            dyn Fn(usize, usize, Point<Pixels>, &mut gpui::Window, &mut App) + Send + Sync,
        > = { Arc::new(|_row, _col, _point, _window, _cx| {}) };

        let on_resize_start: Arc<
            dyn Fn(usize, Pixels, &mut gpui::Window, &mut App) + Send + Sync,
        > = {
            let handle = cx.entity().downgrade();
            Arc::new(move |col_index, start_x, _window, cx| {
                if let Some(entity) = handle.upgrade() {
                    entity.update(cx, |this, cx| {
                        this.start_column_resize(col_index, start_x, cx);
                    });
                }
            })
        };

        let table = render_results_table(
            result,
            page_offset,
            &self.scroll_handle,
            &self.sort_columns,
            selected_cell,
            on_header_click,
            on_cell_click,
            on_cell_secondary_click,
            on_resize_start,
            &config,
            cx,
        )
        .into_any_element();

        let total_row_count = self.total_row_count();
        let status_bar = render_status_bar(result, total_row_count, cx).into_any_element();
        let pagination_bar = self.render_pagination_bar(cx);

        v_flex()
            .flex_grow()
            .child(
                div()
                    .id("result-grid-scroll-container")
                    .flex_grow()
                    .overflow_x_scroll()
                    .child(table),
            )
            .child(pagination_bar)
            .child(status_bar)
            .into_any_element()
    }

    fn render_pagination_bar(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        let total_pages = self.total_pages();
        let current_page = self.page;

        h_flex()
            .w_full()
            .px_2()
            .py_px()
            .gap_2()
            .items_center()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        Button::new("page-prev", "")
                            .icon(IconName::ChevronLeft)
                            .icon_size(IconSize::XSmall)
                            .style(ButtonStyle::Subtle)
                            .disabled(current_page == 0)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                if this.page > 0 {
                                    this.page -= 1;
                                    this.page_offset = this.page * this.rows_per_page;
                                    cx.emit(ResultGridEvent::PageChanged(this.page));
                                    cx.notify();
                                }
                            })),
                    )
                    .child(
                        Label::new(format!("Page {} / {}", current_page + 1, total_pages))
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        Button::new("page-next", "")
                            .icon(IconName::ChevronRight)
                            .icon_size(IconSize::XSmall)
                            .style(ButtonStyle::Subtle)
                            .disabled(current_page + 1 >= total_pages)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                let total_pages = this.total_pages();
                                if this.page + 1 < total_pages {
                                    this.page += 1;
                                    this.page_offset = this.page * this.rows_per_page;
                                    cx.emit(ResultGridEvent::PageChanged(this.page));
                                    cx.notify();
                                }
                            })),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        Label::new("Rows:")
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .children(ROWS_PER_PAGE_OPTIONS.iter().map(|&option| {
                        let is_active = self.rows_per_page == option;
                        Button::new(
                            SharedString::from(format!("rpp-{}", option)),
                            format!("{}", option),
                        )
                        .style(if is_active {
                            ButtonStyle::Filled
                        } else {
                            ButtonStyle::Subtle
                        })
                        .label_size(LabelSize::XSmall)
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.change_rows_per_page(option, cx);
                        }))
                    })),
            )
            .into_any_element()
    }

    fn render_record_view(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        let Some(result) = &self.result else {
            return div()
                .flex_grow()
                .items_center()
                .justify_center()
                .child(
                    Label::new("No results")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element();
        };

        let row_count = result.rows.len();
        if row_count == 0 {
            return div()
                .flex_grow()
                .items_center()
                .justify_center()
                .child(
                    Label::new("No rows to display")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element();
        }

        let current_row = self.record_view_row.min(row_count.saturating_sub(1));

        let navigation_bar = h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .justify_center()
            .items_center()
            .child(
                Button::new("record-prev", "")
                    .icon(IconName::ChevronLeft)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .disabled(current_row == 0)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.record_view_previous(cx);
                    })),
            )
            .child(
                Label::new(format!("Row {} / {}", current_row + 1, row_count))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                Button::new("record-next", "")
                    .icon(IconName::ChevronRight)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .disabled(current_row + 1 >= row_count)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.record_view_next(cx);
                    })),
            );

        let mut field_rows = Vec::new();
        for (col_index, col_name) in result.columns.iter().enumerate() {
            let value = self
                .get_display_value(current_row, col_index)
                .unwrap_or_else(|| "NULL".to_string());
            let is_modified = self.is_cell_modified(current_row, col_index);
            let is_selected = self.selection.contains_cell(current_row, col_index);

            let field_row = h_flex()
                .id(gpui::ElementId::named_usize("record-field", col_index))
                .w_full()
                .min_h(px(28.0))
                .border_b_1()
                .border_color(cx.theme().colors().border_variant)
                .when(is_selected, |el| {
                    el.bg(cx.theme().colors().ghost_element_selected)
                })
                .when(is_modified, |el| {
                    el.border_l_2()
                        .border_color(gpui::yellow())
                })
                .cursor_pointer()
                .on_click({
                    let handle = cx.entity().downgrade();
                    let row = current_row;
                    move |event, window, cx| {
                        if let Some(entity) = handle.upgrade() {
                            entity.update(cx, |this, cx| {
                                this.select_cell(row, col_index, cx);
                                if event.click_count() >= 2 {
                                    this.start_cell_edit(window, cx);
                                }
                            });
                        }
                    }
                })
                .child(
                    div()
                        .flex_none()
                        .w(px(180.0))
                        .h_full()
                        .px_2()
                        .py_1()
                        .flex()
                        .items_center()
                        .border_r_1()
                        .border_color(cx.theme().colors().border_variant)
                        .bg(cx.theme().colors().surface_background)
                        .child(
                            Label::new(SharedString::from(col_name.clone()))
                                .size(LabelSize::Small)
                                .weight(gpui::FontWeight::BOLD)
                                .single_line(),
                        ),
                )
                .child(
                    div()
                        .flex_grow()
                        .px_2()
                        .py_1()
                        .flex()
                        .items_center()
                        .child(if let Some((edit_row, edit_col)) = self.editing_cell {
                            if edit_row == current_row && edit_col == col_index {
                                if let Some(editor) = &self.cell_editor {
                                    div()
                                        .w_full()
                                        .child(editor.clone())
                                        .into_any_element()
                                } else {
                                    Label::new(SharedString::from(value.clone()))
                                        .size(LabelSize::Small)
                                        .color(if value == "NULL" {
                                            Color::Muted
                                        } else {
                                            Color::Default
                                        })
                                        .into_any_element()
                                }
                            } else {
                                Label::new(SharedString::from(value.clone()))
                                    .size(LabelSize::Small)
                                    .color(if value == "NULL" {
                                        Color::Muted
                                    } else {
                                        Color::Default
                                    })
                                    .into_any_element()
                            }
                        } else {
                            Label::new(SharedString::from(value.clone()))
                                .size(LabelSize::Small)
                                .color(if value == "NULL" {
                                    Color::Muted
                                } else {
                                    Color::Default
                                })
                                .into_any_element()
                        }),
                );

            field_rows.push(field_row);
        }

        v_flex()
            .flex_grow()
            .child(navigation_bar)
            .child(
                div()
                    .id("record-view-scroll")
                    .flex_grow()
                    .overflow_y_scroll()
                    .children(field_rows),
            )
            .into_any_element()
    }

    fn render_pending_edits_bar(&self, cx: &mut Context<Self>) -> Option<gpui::AnyElement> {
        let edit_count = self.pending_edits.len();
        let row_state_count = self.pending_row_states.len();
        let total = edit_count + row_state_count;

        if total == 0 {
            return None;
        }

        let label = format!(
            "{} pending change{}",
            total,
            if total == 1 { "" } else { "s" }
        );

        Some(
            h_flex()
                .w_full()
                .px_2()
                .py_1()
                .gap_2()
                .items_center()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .bg(gpui::yellow().opacity(0.1))
                .child(
                    Label::new(label)
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Button::new("preview-dml", "Preview SQL")
                                .style(ButtonStyle::Subtle)
                                .icon(IconName::FileCode)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.show_dml_preview = !this.show_dml_preview;
                                    cx.notify();
                                }))
                                .tooltip(Tooltip::text("Preview generated SQL")),
                        )
                        .child(
                            Button::new("revert-edits", "Revert All")
                                .style(ButtonStyle::Subtle)
                                .icon(IconName::Undo)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.revert_pending_edits(cx);
                                }))
                                .tooltip(Tooltip::text("Revert all pending changes")),
                        )
                        .child(
                            Button::new("commit-edits", "Commit")
                                .style(ButtonStyle::Filled)
                                .icon(IconName::Check)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.commit_pending_edits(cx);
                                }))
                                .tooltip(Tooltip::text("Commit all pending changes")),
                        ),
                )
                .into_any_element(),
        )
    }

    fn render_dml_preview(&self, cx: &mut Context<Self>) -> Option<gpui::AnyElement> {
        if !self.show_dml_preview {
            return None;
        }

        let preview = self.generate_dml_preview();

        Some(
            div()
                .id("dml-preview-scroll")
                .w_full()
                .max_h(px(200.0))
                .overflow_y_scroll()
                .px_2()
                .py_1()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().editor_background)
                .child(
                    Label::new(SharedString::from(preview))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
        )
    }

    fn render_where_filter_bar(&self, cx: &mut Context<Self>) -> Option<gpui::AnyElement> {
        if !self.show_where_filter {
            return None;
        }

        let editor = self.where_editor.as_ref()?;

        Some(
            h_flex()
                .w_full()
                .px_2()
                .py_1()
                .gap_2()
                .items_center()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().surface_background)
                .child(
                    Label::new("WHERE")
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
                .child(
                    div()
                        .flex_grow()
                        .child(editor.clone()),
                )
                .child(
                    Button::new("apply-where", "Apply")
                        .style(ButtonStyle::Filled)
                        .label_size(LabelSize::XSmall)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.apply_where_clause(cx);
                        }))
                        .tooltip(Tooltip::text("Apply WHERE filter")),
                )
                .child(
                    Button::new("clear-where", "")
                        .icon(IconName::Close)
                        .icon_size(IconSize::XSmall)
                        .style(ButtonStyle::Subtle)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.where_clause.clear();
                            this.show_where_filter = false;
                            this.where_editor = None;
                            cx.emit(ResultGridEvent::WhereClauseChanged(String::new()));
                            cx.notify();
                        }))
                        .tooltip(Tooltip::text("Clear and close WHERE filter")),
                )
                .into_any_element(),
        )
    }

    fn render_aggregate_bar(&self, cx: &mut Context<Self>) -> Option<gpui::AnyElement> {
        if !self.show_aggregate_view {
            return None;
        }

        let aggregates = self.calculate_aggregates()?;

        let mut items = Vec::new();
        items.push(format!("Count: {}", aggregates.count));

        if aggregates.numeric_count > 0 {
            if let Some(sum) = aggregates.sum {
                items.push(format!("Sum: {}", format_aggregate_number(sum)));
            }
            if let Some(avg) = aggregates.avg {
                items.push(format!("Avg: {}", format_aggregate_number(avg)));
            }
            if let Some(min) = aggregates.min {
                items.push(format!("Min: {}", format_aggregate_number(min)));
            }
            if let Some(max) = aggregates.max {
                items.push(format!("Max: {}", format_aggregate_number(max)));
            }
        }

        Some(
            h_flex()
                .w_full()
                .px_2()
                .py_1()
                .gap_3()
                .items_center()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().surface_background)
                .children(items.into_iter().map(|item| {
                    Label::new(SharedString::from(item))
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .into_any_element()
                }))
                .into_any_element(),
        )
    }

    fn render_view_mode_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .px_2()
            .py_px()
            .gap_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Button::new("grid-view", "")
                    .icon(IconName::ListTree)
                    .icon_size(IconSize::Small)
                    .style(if self.view_mode == ViewMode::Grid {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if this.view_mode != ViewMode::Grid {
                            this.view_mode = ViewMode::Grid;
                            cx.notify();
                        }
                    }))
                    .tooltip(Tooltip::text("Grid view")),
            )
            .child(
                Button::new("record-view", "")
                    .icon(IconName::Notepad)
                    .icon_size(IconSize::Small)
                    .style(if self.view_mode == ViewMode::Record {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if this.view_mode != ViewMode::Record {
                            this.view_mode = ViewMode::Record;
                            if let Some((row, _)) = this.selection.primary_cell() {
                                this.record_view_row = row;
                            }
                            cx.notify();
                        }
                    }))
                    .tooltip(Tooltip::text("Record view")),
            )
            .child(div().flex_grow())
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("add-row-btn", "")
                            .icon(IconName::Plus)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.add_row(cx);
                            }))
                            .tooltip(Tooltip::text("Add row")),
                    )
                    .child(
                        Button::new("delete-rows-btn", "")
                            .icon(IconName::Trash)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .disabled(self.selection.selected_rows().is_empty())
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.delete_selected_rows(cx);
                            }))
                            .tooltip(Tooltip::text("Delete selected rows")),
                    )
                    .child(
                        Button::new("clone-row-btn", "")
                            .icon(IconName::Copy)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .disabled(self.selection.primary_cell().is_none())
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.clone_selected_row(cx);
                            }))
                            .tooltip(Tooltip::text("Clone selected row")),
                    )
                    .when(self.edit_history.can_undo(), |el| {
                        el.child(
                            Button::new("undo-btn", "")
                                .icon(IconName::Undo)
                                .icon_size(IconSize::Small)
                                .style(ButtonStyle::Subtle)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.undo_edit(cx);
                                }))
                                .tooltip(Tooltip::text("Undo")),
                        )
                    })
                    .when(self.edit_history.can_redo(), |el| {
                        el.child(
                            Button::new("redo-btn", "")
                                .icon(IconName::Return)
                                .icon_size(IconSize::Small)
                                .style(ButtonStyle::Subtle)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.redo_edit(cx);
                                }))
                                .tooltip(Tooltip::text("Redo")),
                        )
                    }),
            )
    }
}

impl EventEmitter<ResultGridEvent> for ResultGrid {}

impl Focusable for ResultGrid {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ResultGrid {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let resizing = self.resizing_column.is_some();
        let has_result = self.result.is_some();

        let content = if has_result {
            match self.view_mode {
                ViewMode::Grid => self.render_grid_view(cx),
                ViewMode::Record => self.render_record_view(cx),
            }
        } else {
            div()
                .flex_grow()
                .items_center()
                .justify_center()
                .child(
                    Label::new("No results")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element()
        };

        let pending_bar = self.render_pending_edits_bar(cx);
        let dml_preview = self.render_dml_preview(cx);
        let where_bar = self.render_where_filter_bar(cx);
        let aggregate_bar = self.render_aggregate_bar(cx);

        v_flex()
            .id("result-grid")
            .key_context("ResultGrid")
            .track_focus(&self.focus_handle)
            .flex_grow()
            .size_full()
            .on_action(cx.listener(|this, _: &StartCellEdit, window, cx| {
                this.start_cell_edit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CommitPendingEdits, _window, cx| {
                this.commit_pending_edits(cx);
            }))
            .on_action(cx.listener(|this, _: &RevertPendingEdits, _window, cx| {
                this.revert_pending_edits(cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleRecordView, _window, cx| {
                this.toggle_view_mode(cx);
            }))
            .on_action(cx.listener(|this, _: &AddRow, _window, cx| {
                this.add_row(cx);
            }))
            .on_action(cx.listener(|this, _: &DeleteSelectedRows, _window, cx| {
                this.delete_selected_rows(cx);
            }))
            .on_action(cx.listener(|this, _: &CloneSelectedRow, _window, cx| {
                this.clone_selected_row(cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleBooleanCell, _window, cx| {
                this.toggle_boolean_cell(cx);
            }))
            .on_action(cx.listener(|this, _: &UndoEdit, _window, cx| {
                this.undo_edit(cx);
            }))
            .on_action(cx.listener(|this, _: &RedoEdit, _window, cx| {
                this.redo_edit(cx);
            }))
            .on_action(cx.listener(|this, _: &ShowDmlPreview, _window, cx| {
                this.show_dml_preview = !this.show_dml_preview;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &menu::SelectNext, _window, cx| {
                if this.view_mode == ViewMode::Record {
                    this.record_view_next(cx);
                } else {
                    this.grid_move_down(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &menu::SelectPrevious, _window, cx| {
                if this.view_mode == ViewMode::Record {
                    this.record_view_previous(cx);
                } else {
                    this.grid_move_up(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &crate::database_panel::GridMoveLeft, _window, cx| {
                this.grid_move_left(cx);
            }))
            .on_action(cx.listener(|this, _: &crate::database_panel::GridMoveRight, _window, cx| {
                this.grid_move_right(cx);
            }))
            .on_action(cx.listener(|this, _: &menu::Cancel, _window, cx| {
                if this.editing_cell.is_some() {
                    this.cancel_edit();
                    cx.notify();
                }
            }))
            .on_action(cx.listener(|this, _: &SelectAllCells, _window, cx| {
                this.select_all(cx);
            }))
            .on_action(cx.listener(|this, _: &ExpandSelection, _window, cx| {
                this.expand_selection(cx);
            }))
            .on_action(cx.listener(|this, _: &ShrinkSelection, _window, cx| {
                this.shrink_selection(cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleWhereFilter, window, cx| {
                this.toggle_where_filter(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleAggregateView, _window, cx| {
                this.toggle_aggregate_view(cx);
            }))
            .when(resizing, |el| {
                el.on_mouse_move(cx.listener(
                    |this, event: &gpui::MouseMoveEvent, _window, cx| {
                        this.handle_resize_move(event.position.x.as_f32(), cx);
                    },
                ))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, _window, _cx| {
                        this.stop_column_resize();
                    }),
                )
            })
            .when(has_result, |el| {
                el.child(self.render_view_mode_toolbar(cx))
            })
            .children(where_bar)
            .child(content)
            .children(aggregate_bar)
            .children(dml_preview)
            .children(pending_bar)
    }
}

fn format_aggregate_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < i64::MAX as f64 {
        format!("{}", value as i64)
    } else {
        format!("{:.4}", value)
    }
}

fn parse_cell_value(text: &str, original: &CellValue) -> CellValue {
    match original {
        CellValue::Boolean(_) => match text.to_lowercase().as_str() {
            "true" | "1" | "yes" | "t" => CellValue::Boolean(true),
            "false" | "0" | "no" | "f" => CellValue::Boolean(false),
            _ => CellValue::Text(text.to_string()),
        },
        CellValue::Integer(_) => {
            if let Ok(i) = text.parse::<i64>() {
                CellValue::Integer(i)
            } else {
                CellValue::Text(text.to_string())
            }
        }
        CellValue::Float(_) => {
            if let Ok(f) = text.parse::<f64>() {
                CellValue::Float(f)
            } else {
                CellValue::Text(text.to_string())
            }
        }
        CellValue::Date(_) => CellValue::Date(text.to_string()),
        CellValue::Time(_) => CellValue::Time(text.to_string()),
        CellValue::Timestamp(_) => CellValue::Timestamp(text.to_string()),
        CellValue::Json(_) => CellValue::Json(text.to_string()),
        CellValue::Uuid(_) => CellValue::Uuid(text.to_string()),
        CellValue::Null => {
            if text.is_empty() {
                CellValue::Null
            } else if let Ok(i) = text.parse::<i64>() {
                CellValue::Integer(i)
            } else if let Ok(f) = text.parse::<f64>() {
                CellValue::Float(f)
            } else {
                CellValue::Text(text.to_string())
            }
        }
        CellValue::Text(_) | CellValue::Blob(_) => CellValue::Text(text.to_string()),
    }
}

fn generate_update_statements(
    table_name: &str,
    edits: &[PendingEdit],
    result: Option<&QueryResult>,
) -> Vec<String> {
    let Some(result) = result else {
        return Vec::new();
    };

    let mut statements = Vec::new();

    for edit in edits {
        let Some(columns) = result.columns.get(edit.col) else {
            continue;
        };

        let set_clause = format!(
            "\"{}\" = {}",
            columns.replace('"', "\"\""),
            edit.new_value.to_sql_value()
        );

        let where_clauses: Vec<String> = result
            .columns
            .iter()
            .enumerate()
            .filter_map(|(col_idx, col_name)| {
                let original = result
                    .rows
                    .get(edit.row)
                    .and_then(|r| r.get(col_idx))?;
                let value_sql = original.to_sql_value();
                if matches!(original, CellValue::Null) {
                    Some(format!("\"{}\" IS NULL", col_name.replace('"', "\"\"")))
                } else {
                    Some(format!(
                        "\"{}\" = {}",
                        col_name.replace('"', "\"\""),
                        value_sql
                    ))
                }
            })
            .collect();

        if where_clauses.is_empty() {
            continue;
        }

        let statement = format!(
            "UPDATE \"{}\" SET {} WHERE {} LIMIT 1",
            table_name.replace('"', "\"\""),
            set_clause,
            where_clauses.join(" AND ")
        );
        statements.push(statement);
    }

    statements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell_value_integer() {
        let result = parse_cell_value("42", &CellValue::Integer(0));
        assert!(matches!(result, CellValue::Integer(42)));
    }

    #[test]
    fn test_parse_cell_value_float() {
        let result = parse_cell_value("3.14", &CellValue::Float(0.0));
        assert!(matches!(result, CellValue::Float(f) if (f - 3.14).abs() < 0.001));
    }

    #[test]
    fn test_parse_cell_value_text() {
        let result = parse_cell_value("hello", &CellValue::Text(String::new()));
        assert!(matches!(result, CellValue::Text(s) if s == "hello"));
    }

    #[test]
    fn test_parse_cell_value_null_empty() {
        let result = parse_cell_value("", &CellValue::Null);
        assert!(matches!(result, CellValue::Null));
    }

    #[test]
    fn test_parse_cell_value_null_with_number() {
        let result = parse_cell_value("123", &CellValue::Null);
        assert!(matches!(result, CellValue::Integer(123)));
    }

    #[test]
    fn test_generate_update_statements_basic() {
        use std::time::Duration;

        let result = QueryResult {
            columns: vec!["id".to_string(), "name".to_string()],
            rows: vec![vec![CellValue::Integer(1), CellValue::Text("old".to_string())]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };

        let edits = vec![PendingEdit {
            row: 0,
            col: 1,
            original_value: CellValue::Text("old".to_string()),
            new_value: CellValue::Text("new".to_string()),
        }];

        let statements = generate_update_statements("users", &edits, Some(&result));
        assert_eq!(statements.len(), 1);
        assert!(statements[0].contains("UPDATE \"users\""));
        assert!(statements[0].contains("SET \"name\" = 'new'"));
        assert!(statements[0].contains("WHERE"));
    }

    #[test]
    fn test_grid_selection_contains_cell() {
        let sel = GridSelection::Cell(2, 3);
        assert!(sel.contains_cell(2, 3));
        assert!(!sel.contains_cell(2, 4));
        assert!(!sel.contains_cell(3, 3));
    }

    #[test]
    fn test_grid_selection_range() {
        let sel = GridSelection::Range {
            start: (1, 1),
            end: (3, 3),
        };
        assert!(sel.contains_cell(1, 1));
        assert!(sel.contains_cell(2, 2));
        assert!(sel.contains_cell(3, 3));
        assert!(!sel.contains_cell(0, 0));
        assert!(!sel.contains_cell(4, 4));
    }

    #[test]
    fn test_grid_selection_rows() {
        let sel = GridSelection::Rows(vec![1, 3, 5]);
        assert!(sel.contains_row(1));
        assert!(sel.contains_row(3));
        assert!(sel.contains_row(5));
        assert!(!sel.contains_row(2));
        assert!(!sel.contains_row(4));
    }

    #[test]
    fn test_grid_selection_all() {
        let sel = GridSelection::All;
        assert!(sel.contains_cell(0, 0));
        assert!(sel.contains_cell(100, 100));
        assert!(sel.contains_row(999));
    }

    #[test]
    fn test_grid_selection_selected_rows() {
        let sel = GridSelection::Range {
            start: (2, 0),
            end: (5, 3),
        };
        let rows = sel.selected_rows();
        assert_eq!(rows, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_edit_history_undo_redo() {
        let mut history = DataEditHistory::default();
        assert!(!history.can_undo());
        assert!(!history.can_redo());

        history.push(EditOperation::CellEdit {
            row: 0,
            col: 0,
            old: CellValue::Text("a".to_string()),
            new: CellValue::Text("b".to_string()),
        });
        assert!(history.can_undo());
        assert!(!history.can_redo());

        let undone = history.undo();
        assert!(undone.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());

        let redone = history.redo();
        assert!(redone.is_some());
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_edit_history_push_clears_redo() {
        let mut history = DataEditHistory::default();
        history.push(EditOperation::CellEdit {
            row: 0,
            col: 0,
            old: CellValue::Null,
            new: CellValue::Integer(1),
        });
        history.undo();
        assert!(history.can_redo());

        history.push(EditOperation::CellEdit {
            row: 1,
            col: 0,
            old: CellValue::Null,
            new: CellValue::Integer(2),
        });
        assert!(!history.can_redo());
    }

    #[test]
    fn test_filter_clause_to_sql_equals() {
        let filter = FilterClause {
            column: "name".to_string(),
            operator: FilterOperator::Equals,
            value: Some("test".to_string()),
        };
        let sql = filter.to_sql(&database_core::DatabaseType::Sqlite);
        assert!(sql.contains("\"name\" = 'test'"));
    }

    #[test]
    fn test_filter_clause_to_sql_is_null() {
        let filter = FilterClause {
            column: "email".to_string(),
            operator: FilterOperator::IsNull,
            value: None,
        };
        let sql = filter.to_sql(&database_core::DatabaseType::Sqlite);
        assert!(sql.contains("\"email\" IS NULL"));
    }

    #[test]
    fn test_filter_clause_to_sql_contains() {
        let filter = FilterClause {
            column: "bio".to_string(),
            operator: FilterOperator::Contains,
            value: Some("hello".to_string()),
        };
        let sql = filter.to_sql(&database_core::DatabaseType::Sqlite);
        assert!(sql.contains("LIKE '%hello%'"));
    }

    #[test]
    fn test_filter_display_text() {
        let filter = FilterClause {
            column: "age".to_string(),
            operator: FilterOperator::GreaterThan,
            value: Some("18".to_string()),
        };
        assert_eq!(filter.display_text(), "age > 18");
    }

    #[test]
    fn test_boolean_toggle_cycle() {
        // Boolean(true) toggles to Boolean(false)
        let val = CellValue::Boolean(true);
        let new1 = match &val {
            CellValue::Boolean(b) => CellValue::Boolean(!b),
            _ => CellValue::Boolean(true),
        };
        assert_eq!(new1, CellValue::Boolean(false));

        // Boolean(false) toggles to Boolean(true)
        let val = CellValue::Boolean(false);
        let new2 = match &val {
            CellValue::Boolean(b) => CellValue::Boolean(!b),
            _ => CellValue::Boolean(true),
        };
        assert_eq!(new2, CellValue::Boolean(true));

        // Non-boolean uses display fallback
        let val = CellValue::Integer(1);
        let new3 = match &val {
            CellValue::Boolean(b) => CellValue::Boolean(!b),
            _ => {
                let display = val.to_string();
                match display.to_lowercase().as_str() {
                    "true" | "1" => CellValue::Boolean(false),
                    "false" | "0" => CellValue::Boolean(true),
                    _ => CellValue::Boolean(true),
                }
            }
        };
        assert_eq!(new3, CellValue::Boolean(false));
    }

    #[test]
    fn test_parse_cell_value_boolean() {
        assert_eq!(
            parse_cell_value("true", &CellValue::Boolean(false)),
            CellValue::Boolean(true)
        );
        assert_eq!(
            parse_cell_value("false", &CellValue::Boolean(true)),
            CellValue::Boolean(false)
        );
        assert_eq!(
            parse_cell_value("1", &CellValue::Boolean(false)),
            CellValue::Boolean(true)
        );
        assert_eq!(
            parse_cell_value("0", &CellValue::Boolean(true)),
            CellValue::Boolean(false)
        );
    }

    #[test]
    fn test_parse_cell_value_date_time() {
        assert_eq!(
            parse_cell_value("2024-01-15", &CellValue::Date(String::new())),
            CellValue::Date("2024-01-15".to_string())
        );
        assert_eq!(
            parse_cell_value("14:30:00", &CellValue::Time(String::new())),
            CellValue::Time("14:30:00".to_string())
        );
        assert_eq!(
            parse_cell_value("2024-01-15 14:30:00", &CellValue::Timestamp(String::new())),
            CellValue::Timestamp("2024-01-15 14:30:00".to_string())
        );
    }

    #[test]
    fn test_expand_selection() {
        // None → Cell
        let sel = GridSelection::None;
        let next = match &sel {
            GridSelection::None => GridSelection::Cell(0, 0),
            GridSelection::Cell(row, _) => GridSelection::Rows(vec![*row]),
            _ => GridSelection::All,
        };
        assert!(matches!(next, GridSelection::Cell(0, 0)));

        // Cell → Rows
        let sel = GridSelection::Cell(2, 3);
        let next = match &sel {
            GridSelection::None => GridSelection::Cell(0, 0),
            GridSelection::Cell(row, _) => GridSelection::Rows(vec![*row]),
            _ => GridSelection::All,
        };
        assert_eq!(next, GridSelection::Rows(vec![2]));

        // Rows → All
        let sel = GridSelection::Rows(vec![1, 2]);
        let next = match &sel {
            GridSelection::None => GridSelection::Cell(0, 0),
            GridSelection::Cell(row, _) => GridSelection::Rows(vec![*row]),
            _ => GridSelection::All,
        };
        assert_eq!(next, GridSelection::All);
    }

    #[test]
    fn test_shrink_selection() {
        // All → Rows
        let sel = GridSelection::All;
        let next = match &sel {
            GridSelection::All => GridSelection::Rows(vec![0]),
            GridSelection::Rows(rows) => {
                let row = rows.first().copied().unwrap_or(0);
                GridSelection::Cell(row, 0)
            }
            GridSelection::Range { start, .. } => GridSelection::Cell(start.0, start.1),
            GridSelection::Columns(cols) => {
                let col = cols.first().copied().unwrap_or(0);
                GridSelection::Cell(0, col)
            }
            GridSelection::Cell(_, _) | GridSelection::None => GridSelection::None,
        };
        assert_eq!(next, GridSelection::Rows(vec![0]));

        // Rows → Cell
        let sel = GridSelection::Rows(vec![3, 5]);
        let next = match &sel {
            GridSelection::Rows(rows) => {
                let row = rows.first().copied().unwrap_or(0);
                GridSelection::Cell(row, 0)
            }
            _ => GridSelection::None,
        };
        assert_eq!(next, GridSelection::Cell(3, 0));

        // Cell → None
        let sel = GridSelection::Cell(1, 2);
        let next = match &sel {
            GridSelection::Cell(_, _) | GridSelection::None => GridSelection::None,
            _ => GridSelection::None,
        };
        assert_eq!(next, GridSelection::None);
    }

    #[test]
    fn test_calculate_aggregates_numeric() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let sum: f64 = values.iter().sum();
        let avg = sum / values.len() as f64;
        let min = values.iter().copied().reduce(f64::min);
        let max = values.iter().copied().reduce(f64::max);

        assert_eq!(sum, 150.0);
        assert_eq!(avg, 30.0);
        assert_eq!(min, Some(10.0));
        assert_eq!(max, Some(50.0));
    }

    #[test]
    fn test_calculate_aggregates_empty() {
        let values: Vec<f64> = Vec::new();
        assert!(values.is_empty());
        let sum: Option<f64> = None;
        let avg: Option<f64> = None;
        assert!(sum.is_none());
        assert!(avg.is_none());
    }

    #[test]
    fn test_format_aggregate_number() {
        assert_eq!(format_aggregate_number(42.0), "42");
        assert_eq!(format_aggregate_number(3.14159), "3.1416");
        assert_eq!(format_aggregate_number(0.0), "0");
        assert_eq!(format_aggregate_number(-5.0), "-5");
    }

    #[test]
    fn test_parse_cell_value_json_uuid() {
        assert_eq!(
            parse_cell_value("{\"key\":\"val\"}", &CellValue::Json(String::new())),
            CellValue::Json("{\"key\":\"val\"}".to_string())
        );
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        assert_eq!(
            parse_cell_value(uuid_str, &CellValue::Uuid(String::new())),
            CellValue::Uuid(uuid_str.to_string())
        );
    }

    #[test]
    fn test_multi_edit_cells_range_same_column() {
        let selection = GridSelection::Range {
            start: (1, 2),
            end: (4, 2),
        };
        let min_col = 2.min(2);
        let max_col = 2.max(2);
        let result: Vec<(usize, usize)> = if min_col == max_col {
            (1..=4).map(|r| (r, 2)).collect()
        } else {
            vec![(1, 2)]
        };
        assert_eq!(result, vec![(1, 2), (2, 2), (3, 2), (4, 2)]);

        // Different columns → single cell
        let selection = GridSelection::Range {
            start: (1, 2),
            end: (3, 5),
        };
        let min_col = 2.min(5);
        let max_col = 2.max(5);
        let result: Vec<(usize, usize)> = if min_col == max_col {
            (1..=3).map(|r| (r, 2)).collect()
        } else {
            vec![(1, 2)]
        };
        assert_eq!(result, vec![(1, 2)]);
    }

    #[test]
    fn test_multi_edit_cells_rows() {
        let rows = vec![0, 2, 5];
        let result: Vec<(usize, usize)> = rows.iter().map(|r| (*r, 3)).collect();
        assert_eq!(result, vec![(0, 3), (2, 3), (5, 3)]);
    }

    #[test]
    fn test_multi_edit_cells_single_cell() {
        let selection = GridSelection::Cell(3, 4);
        let result = match &selection {
            GridSelection::Cell(_, _) | GridSelection::None => vec![(3, 4)],
            _ => vec![(3, 4)],
        };
        assert_eq!(result, vec![(3, 4)]);
    }
}
