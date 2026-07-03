use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

use database_client::{
    ColumnInfo, DatabaseClient, EditCell, Filter, FilterOp, RowDelete, RowInsert, RowKey,
    RowUpdate, Sort, SortDirection, TableEdits, TableRef, TableStructure,
};
use gpui::{
    Anchor, AnyElement, App, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    Task, WeakEntity, Window, actions,
};
use settings::Settings as _;
use ui::{
    AbsoluteLength, ColumnWidthConfig, ContextMenu, PopoverMenu, ResizableColumnsState, Table,
    TableInteractionState, TableResizeBehavior, Tooltip, prelude::*,
};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::{Workspace, item::Item};

use crate::DatabaseSettings;
use crate::UI_MAX_QUERY_ROWS;
use crate::query_state::{QueryState, render_sql};

actions!(
    database,
    [
        /// Loads the next page of rows in the table view.
        NextPage,
        /// Loads the previous page of rows in the table view.
        PrevPage,
        /// Switches between the data and structure views of a table.
        ToggleStructure,
        /// Reloads the current table data (and structure if loaded).
        RefreshData,
        /// Commits the value in the inline cell editor to the edit buffer.
        CommitCellEdit,
        /// Cancels the inline cell editor without changing the edit buffer.
        CancelCellEdit,
        /// Sets the cell being edited to NULL and closes the inline editor.
        SetCellNull,
    ]
);

/// The default column width for the resizable data grid.
const COLUMN_WIDTH: f32 = 180.;

/// The short symbol shown in the UI for each filter operator.
fn filter_op_label(op: FilterOp) -> &'static str {
    match op {
        FilterOp::Eq => "=",
        FilterOp::NotEq => "≠",
        FilterOp::Gt => ">",
        FilterOp::Lt => "<",
        FilterOp::Contains => "contains",
        FilterOp::IsNull => "is null",
        FilterOp::IsNotNull => "is not null",
    }
}

/// Every filter operator, in the order they appear in the operator dropdown.
fn all_filter_ops() -> [FilterOp; 7] {
    [
        FilterOp::Eq,
        FilterOp::NotEq,
        FilterOp::Gt,
        FilterOp::Lt,
        FilterOp::Contains,
        FilterOp::IsNull,
        FilterOp::IsNotNull,
    ]
}

/// Which of the two tabs of a table view is currently shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Data,
    Structure,
}

/// Tracks the in-flight state of the current data load.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadState {
    Idle,
    Loading,
    Error(String),
}

/// Tracks the in-flight state of applying buffered edits to the database.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveState {
    /// No save has run, or the last one was superseded by fresh edits.
    Idle,
    /// A save is in flight; further saves are ignored until it settles.
    Saving,
    /// The last save succeeded; holds a brief human-readable summary.
    Done(String),
    /// The last save failed; holds the formatted error and leaves the buffer
    /// intact so the user can retry.
    Error(String),
}

/// A stable identifier for a pending insert row, assigned from a monotonic
/// counter when the row is added. Insert rows are addressed by this id rather
/// than by their position in [`TableEditBuffer::inserts`], so deleting one
/// pending insert never shifts the identity of the others (which a `Vec` index
/// would).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InsertId(u64);

/// Identifies which row an inline edit targets: an existing page row, addressed
/// by its original primary-key values, or a pending insert row, addressed by its
/// stable [`InsertId`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditTarget {
    /// An existing row, identified by its original primary-key values.
    Existing(RowKey),
    /// A pending insert row, identified by its stable insert id.
    New(InsertId),
}

/// The cell currently being edited inline. The target distinguishes an existing
/// page row from a pending insert row; the `field` holds the live text input
/// shown in the cell.
pub struct EditingCell {
    /// Which row (existing or new) this edit targets.
    pub target: EditTarget,
    /// The name of the column being edited.
    pub column: String,
    /// The display-row index (page row for `Existing`, insert index for `New`),
    /// so the editor renders in the right cell.
    pub display_row: usize,
    /// The column index in the current page, so the editor renders in the right
    /// cell.
    pub column_index: usize,
    /// The cell's display value when the editor opened: `Some` for a concrete
    /// value, `None` when the cell was NULL. Used to decide whether a commit
    /// actually changed anything, so an untouched cell (in particular an
    /// untouched NULL) is not degraded into an empty-string update.
    pub original: Option<String>,
    /// The live text input.
    pub field: Entity<InputField>,
}

/// Buffers pending row edits before they are applied to the database as one
/// transaction. Updates are keyed by [`RowKey`] (the row's original primary-key
/// values) so repeated edits to the same row and column coalesce; inserts are
/// new rows that have no key yet; deletes hold the keys of rows to remove.
#[derive(Debug, Default)]
pub struct TableEditBuffer {
    /// Per-row column edits against existing rows, keyed by original PK values.
    updates: HashMap<RowKey, HashMap<String, EditCell>>,
    /// New rows to insert, in display order. Each carries a stable [`InsertId`]
    /// so edits and deletes can address a specific pending row without being
    /// invalidated when an earlier one is removed.
    inserts: Vec<(InsertId, HashMap<String, EditCell>)>,
    /// Keys of existing rows to delete.
    deletes: HashSet<RowKey>,
}

impl TableEditBuffer {
    /// The number of rows affected by the buffered edits: updated rows plus
    /// inserted rows plus deleted rows.
    pub fn pending_change_count(&self) -> usize {
        self.updates.len() + self.inserts.len() + self.deletes.len()
    }

    pub fn updates(&self) -> &HashMap<RowKey, HashMap<String, EditCell>> {
        &self.updates
    }

    pub fn inserts(&self) -> &[(InsertId, HashMap<String, EditCell>)] {
        &self.inserts
    }

    pub fn deletes(&self) -> &HashSet<RowKey> {
        &self.deletes
    }

    /// The column map of the pending insert row with the given id, if present.
    fn insert(&self, id: InsertId) -> Option<&HashMap<String, EditCell>> {
        self.inserts
            .iter()
            .find(|(insert_id, _)| *insert_id == id)
            .map(|(_, columns)| columns)
    }

    /// The mutable column map of the pending insert row with the given id.
    fn insert_mut(&mut self, id: InsertId) -> Option<&mut HashMap<String, EditCell>> {
        self.inserts
            .iter_mut()
            .find(|(insert_id, _)| *insert_id == id)
            .map(|(_, columns)| columns)
    }

    fn clear(&mut self) {
        self.updates.clear();
        self.inserts.clear();
        self.deletes.clear();
    }
}

/// Whether a table's rows can be edited: only base tables (not views) that have
/// a primary key, since every UPDATE/DELETE is addressed by its full PK.
fn compute_editable(is_view: bool, columns: &[ColumnInfo]) -> bool {
    let has_primary_key = columns.iter().any(|column| column.is_primary_key);
    !is_view && has_primary_key
}

/// A bare [`ColumnInfo`] carrying only a name, used as a fallback when the full
/// structure has not loaded. The `text` cast is a safe default for PostgreSQL,
/// which will coerce a text parameter to the target column type on assignment.
fn column_info_from_name(name: &String) -> ColumnInfo {
    ColumnInfo {
        name: name.clone(),
        data_type: "text".to_string(),
        udt_name: "text".to_string(),
        udt_schema: "pg_catalog".to_string(),
        is_nullable: true,
        default: None,
        is_primary_key: false,
    }
}

/// The result of running the current page's SQL, holding just what the grid
/// needs to render: column names, row values, and whether more rows exist
/// beyond this page. Replaces the server-side `RowsPage` probe now that rows
/// come from [`DatabaseClient::run_query`] rather than `fetch_rows`.
struct PageData {
    columns: Vec<String>,
    rows: Vec<Vec<Option<String>>>,
    has_more: bool,
}

/// A workspace tab showing the rows and structure of a single database table.
///
/// The data grid supports sorting and offset pagination by rendering a
/// [`QueryState`] to SQL text and running it through
/// [`DatabaseClient::run_query`]; the structure tab is fetched lazily on first
/// display and cached until an explicit refresh.
pub struct TableDataView {
    focus_handle: FocusHandle,
    client: Arc<dyn DatabaseClient>,
    /// The name of the connection this tab's `client` belongs to. Two tables
    /// with identical coordinates on different connections must not alias to one
    /// tab, so this is part of the tab dedup key (see [`open_table_tab`]).
    connection: String,
    table: TableRef,
    /// Whether this tab's table is a database view. Provided by the tree at open
    /// time; combined with the loaded structure's primary key to gate editing.
    is_view: bool,
    /// Whether rows can be edited. `false` until the structure loads, then set to
    /// `!is_view && has_primary_key` (see [`compute_editable`]).
    editable: bool,
    /// Buffered, not-yet-applied row edits.
    edits: TableEditBuffer,
    /// Monotonic source of stable [`InsertId`]s for pending insert rows. Never
    /// reset within a tab's lifetime, so ids stay unique even after a save
    /// clears the insert buffer.
    next_insert_id: u64,
    /// The cell currently open in the inline editor, if any.
    editing_cell: Option<EditingCell>,
    /// The in-flight state of the most recent save.
    save_state: SaveState,
    mode: ViewMode,
    query: QueryState,
    /// Wrapped in `Arc` so the render hot path (scroll re-renders) hands the
    /// rows to `uniform_list` by cheap clone instead of deep-copying every cell.
    page: Option<Arc<PageData>>,
    structure: Option<TableStructure>,
    load_state: LoadState,
    interaction: Entity<TableInteractionState>,
    /// Recreated whenever the rendered column set changes so the grid keeps the
    /// right number of resize handles.
    column_widths: Option<Entity<ResizableColumnsState>>,
    /// Whether the inline filter-builder row is expanded under the header.
    filter_builder_open: bool,
    /// The column selected in the filter builder, if any.
    draft_column: Option<String>,
    /// The operator selected in the filter builder.
    draft_op: FilterOp,
    /// The value input for the filter builder (ignored for `IsNull`).
    draft_value: Entity<InputField>,
    /// The row count and wall-clock duration of the most recent successful
    /// query run, shown in the footer. `None` before the first page loads.
    last_run: Option<(usize, Duration)>,
    /// Held separately from `_structure_task` so a structure load and a data
    /// reload can be in flight at the same time without one aborting the other.
    _data_task: Option<Task<()>>,
    _structure_task: Option<Task<()>>,
    /// The in-flight save task, if any. Held so `save_state == Saving` reliably
    /// gates against concurrent saves and the work is cancelled on drop.
    _save_task: Option<Task<()>>,
}

impl TableDataView {
    pub fn new(
        client: Arc<dyn DatabaseClient>,
        connection: String,
        table: TableRef,
        is_view: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let page_size = DatabaseSettings::get_global(cx).page_size.max(1) as usize;
        cx.new(|cx| {
            let interaction = cx.new(|cx| TableInteractionState::new(cx));
            let draft_value = cx.new(|cx| InputField::new(window, cx, "Value"));
            let mut view = Self {
                focus_handle: cx.focus_handle(),
                client,
                connection,
                query: QueryState::for_table(table.clone(), page_size),
                table,
                is_view,
                editable: false,
                edits: TableEditBuffer::default(),
                next_insert_id: 0,
                editing_cell: None,
                save_state: SaveState::Idle,
                mode: ViewMode::Data,
                page: None,
                structure: None,
                load_state: LoadState::Idle,
                interaction,
                column_widths: None,
                filter_builder_open: false,
                draft_column: None,
                draft_op: FilterOp::Eq,
                draft_value,
                last_run: None,
                _data_task: None,
                _structure_task: None,
                _save_task: None,
            };
            view.restart_query(window, cx);
            // Load the structure eagerly alongside the first page so the primary
            // key (hence editability) is known without switching to the
            // Structure tab.
            view.reload_structure(cx);
            view
        })
    }

    pub fn table(&self) -> &TableRef {
        &self.table
    }

    pub fn connection(&self) -> &str {
        &self.connection
    }

    pub fn query(&self) -> &QueryState {
        &self.query
    }

    /// The SQL text that the current [`QueryState`] renders to; this is what
    /// the next reload will execute.
    pub fn current_sql(&self) -> String {
        render_sql(&self.query)
    }

    /// The row count and wall-clock duration of the most recent successful
    /// query run, if any page has loaded yet.
    pub fn last_run(&self) -> Option<(usize, Duration)> {
        self.last_run
    }

    /// Test-only: exposes the loaded page so tests can assert on row/column
    /// data without reaching into the private `page` field.
    #[cfg(test)]
    fn page(&self) -> Option<&PageData> {
        self.page.as_deref()
    }

    pub fn structure(&self) -> Option<&TableStructure> {
        self.structure.as_ref()
    }

    pub fn load_state(&self) -> &LoadState {
        &self.load_state
    }

    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// Whether rows in this table can be edited (base table with a primary key).
    /// `false` until the structure has loaded.
    pub fn editable(&self) -> bool {
        self.editable
    }

    /// The buffered, not-yet-applied edits.
    pub fn edits(&self) -> &TableEditBuffer {
        &self.edits
    }

    /// The cell currently open in the inline editor, if any.
    pub fn editing_cell(&self) -> Option<&EditingCell> {
        self.editing_cell.as_ref()
    }

    /// The in-flight state of the most recent save.
    pub fn save_state(&self) -> &SaveState {
        &self.save_state
    }

    /// The number of rows affected by buffered edits.
    pub fn pending_change_count(&self) -> usize {
        self.edits.pending_change_count()
    }

    /// Builds a [`RowKey`] for the row at `display_row` in the current page,
    /// using the primary-key columns from the loaded structure and that row's
    /// values in the page. Returns `None` if the structure or page is missing,
    /// there is no primary key, or a PK column is absent from the page.
    pub fn row_key_for(&self, display_row: usize) -> Option<RowKey> {
        let structure = self.structure.as_ref()?;
        let page = self.page.as_ref()?;
        let row = page.rows.get(display_row)?;

        let mut columns = Vec::new();
        let mut values = Vec::new();
        for column in structure.columns.iter().filter(|c| c.is_primary_key) {
            let column_index = page.columns.iter().position(|name| name == &column.name)?;
            let value = row.get(column_index)?.clone();
            columns.push(column.name.clone());
            values.push(value);
        }
        if columns.is_empty() {
            return None;
        }
        Some(RowKey { columns, values })
    }

    /// Whether `column` is a primary-key column of the loaded structure. Editing
    /// PK cells is disallowed because the PK identifies the row.
    fn is_primary_key_column(&self, column: &str) -> bool {
        self.structure.as_ref().is_some_and(|structure| {
            structure
                .columns
                .iter()
                .any(|c| c.name == column && c.is_primary_key)
        })
    }

    /// Buffers a non-null value edit for `column` of the existing row identified
    /// by `row_key`. Editing a primary-key column is a no-op.
    pub fn set_cell_value(
        &mut self,
        row_key: RowKey,
        column: &str,
        value: String,
        cx: &mut Context<Self>,
    ) {
        self.set_cell(row_key, column, EditCell::Value(value), cx);
    }

    /// Buffers a NULL edit for `column` of the existing row identified by
    /// `row_key`. Editing a primary-key column is a no-op.
    pub fn set_cell_null(&mut self, row_key: RowKey, column: &str, cx: &mut Context<Self>) {
        self.set_cell(row_key, column, EditCell::Null, cx);
    }

    fn set_cell(&mut self, row_key: RowKey, column: &str, cell: EditCell, cx: &mut Context<Self>) {
        if self.is_saving() {
            return;
        }
        if self.is_primary_key_column(column) {
            log::debug!("ignoring edit of primary-key column {column:?}: PK identifies the row");
            return;
        }
        // A row already marked for deletion must not also gather an update: the
        // apply would carry a delete and an update for the same key and fail.
        if self.edits.deletes.contains(&row_key) {
            log::debug!("ignoring edit of row marked for deletion");
            return;
        }
        self.edits
            .updates
            .entry(row_key)
            .or_default()
            .insert(column.to_string(), cell);
        self.clear_finished_save_state();
        cx.notify();
    }

    /// Appends an empty new row to the insert buffer, returning its stable
    /// [`InsertId`].
    pub fn add_row(&mut self, cx: &mut Context<Self>) -> Option<InsertId> {
        if self.is_saving() {
            return None;
        }
        let id = InsertId(self.next_insert_id);
        self.next_insert_id += 1;
        self.edits.inserts.push((id, HashMap::new()));
        self.clear_finished_save_state();
        cx.notify();
        Some(id)
    }

    /// Buffers a value for `column` of the pending insert row `id`. Unlike
    /// existing rows, primary-key columns *are* settable here, since a new row
    /// needs a supplied key. An unknown id is a no-op.
    pub fn set_new_cell_value(
        &mut self,
        id: InsertId,
        column: &str,
        value: String,
        cx: &mut Context<Self>,
    ) {
        self.set_new_cell(id, column, EditCell::Value(value), cx);
    }

    /// Buffers a NULL for `column` of the pending insert row `id`. An unknown id
    /// is a no-op.
    pub fn set_new_cell_null(&mut self, id: InsertId, column: &str, cx: &mut Context<Self>) {
        self.set_new_cell(id, column, EditCell::Null, cx);
    }

    fn set_new_cell(&mut self, id: InsertId, column: &str, cell: EditCell, cx: &mut Context<Self>) {
        if self.is_saving() {
            return;
        }
        let Some(row) = self.edits.insert_mut(id) else {
            log::debug!("ignoring edit of insert row {id:?}: unknown insert id");
            return;
        };
        row.insert(column.to_string(), cell);
        self.clear_finished_save_state();
        cx.notify();
    }

    /// Removes the pending insert row `id` from the buffer (a new, not-yet-saved
    /// row is simply forgotten rather than recorded as a delete). Also closes the
    /// inline editor if it targets that row. An unknown id is a no-op.
    pub fn delete_new_row(&mut self, id: InsertId, cx: &mut Context<Self>) {
        if self.is_saving() {
            return;
        }
        let Some(position) = self
            .edits
            .inserts
            .iter()
            .position(|(insert_id, _)| *insert_id == id)
        else {
            log::debug!("delete_new_row: unknown insert id {id:?}");
            return;
        };
        // Close the editor only if it targets the row being removed; other
        // pending inserts keep their (stable) identity, so their editor is fine.
        if self
            .editing_cell
            .as_ref()
            .is_some_and(|editing| editing.target == EditTarget::New(id))
        {
            self.editing_cell = None;
        }
        self.edits.inserts.remove(position);
        self.clear_finished_save_state();
        cx.notify();
    }

    /// Marks the existing row identified by `row_key` for deletion, dropping any
    /// buffered update for that same row (a delete supersedes an update). Also
    /// closes the inline editor if it is open on that row.
    pub fn delete_row(&mut self, row_key: RowKey, cx: &mut Context<Self>) {
        if self.is_saving() {
            return;
        }
        // The editor for the row being deleted would otherwise commit an update
        // for a now-deleted key on the next page swap, failing the save.
        if self
            .editing_cell
            .as_ref()
            .is_some_and(|editing| editing.target == EditTarget::Existing(row_key.clone()))
        {
            self.editing_cell = None;
        }
        self.edits.updates.remove(&row_key);
        self.edits.deletes.insert(row_key);
        self.clear_finished_save_state();
        cx.notify();
    }

    /// Whether a save is currently in flight. While saving, the edit buffer is
    /// frozen so the snapshot handed to `apply_edits` matches what the success
    /// handler clears; programmatic mutations are ignored (the toolbar already
    /// disables the corresponding buttons).
    fn is_saving(&self) -> bool {
        self.save_state == SaveState::Saving
    }

    /// Resets a finished (Done/Error) save banner back to Idle once the buffer is
    /// made dirty again, so the toolbar reflects the new pending state rather
    /// than a stale outcome. A save in flight is left untouched.
    fn clear_finished_save_state(&mut self) {
        if matches!(self.save_state, SaveState::Done(_) | SaveState::Error(_)) {
            self.save_state = SaveState::Idle;
        }
    }

    /// Clears all buffered edits and the inline editor.
    pub fn discard_edits(&mut self, cx: &mut Context<Self>) {
        self.edits.clear();
        self.editing_cell = None;
        self.save_state = SaveState::Idle;
        cx.notify();
    }

    /// The display value shown for an existing row's cell, taking any buffered
    /// edit into account: a buffered `Value` wins over the page value, and a
    /// buffered or original NULL yields `None`.
    fn cell_display_value(
        &self,
        row_key: &RowKey,
        column: &str,
        page_value: Option<&String>,
    ) -> Option<String> {
        match self
            .edits
            .updates()
            .get(row_key)
            .and_then(|row| row.get(column))
        {
            Some(EditCell::Value(value)) => Some(value.clone()),
            Some(EditCell::Null) => None,
            None => page_value.cloned(),
        }
    }

    /// Opens the inline editor on the cell at `display_row`/`column_index`.
    ///
    /// No-op unless the table is editable, has a loaded page/structure, and the
    /// target column is not part of the primary key (the PK identifies the row
    /// and must not change). The editor is pre-filled with the cell's current
    /// display value (empty for NULL) and focused.
    pub fn begin_edit_cell(
        &mut self,
        display_row: usize,
        column_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable || self.is_saving() {
            return;
        }
        let Some(page) = self.page.clone() else {
            return;
        };
        let Some(column) = page.columns.get(column_index).cloned() else {
            return;
        };
        if self.is_primary_key_column(&column) {
            return;
        }
        let Some(row_key) = self.row_key_for(display_row) else {
            return;
        };
        // A row marked for deletion must not be edited: the apply cannot carry
        // both a delete and an update for the same key.
        if self.edits.deletes.contains(&row_key) {
            return;
        }
        let page_value = page
            .rows
            .get(display_row)
            .and_then(|row| row.get(column_index))
            .and_then(|cell| cell.clone());
        let current = self.cell_display_value(&row_key, &column, page_value.as_ref());

        let field = cx.new(|cx| {
            let field = InputField::new(window, cx, "");
            if let Some(value) = current.as_ref() {
                field.set_text(value, window, cx);
            }
            field
        });
        field.focus_handle(cx).focus(window, cx);

        self.editing_cell = Some(EditingCell {
            target: EditTarget::Existing(row_key),
            column,
            display_row,
            column_index,
            original: current,
            field,
        });
        cx.notify();
    }

    /// Opens the inline editor on a cell of the pending insert row `id`, rendered
    /// at `display_row`.
    ///
    /// Unlike [`begin_edit_cell`], every column is editable here (including the
    /// primary key), since a new row needs a supplied key. No-op unless the table
    /// is editable, the page is loaded (for the column name), and `id` names a
    /// known insert row. The editor is pre-filled with any buffered value.
    pub fn begin_edit_new_cell(
        &mut self,
        id: InsertId,
        display_row: usize,
        column_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable || self.is_saving() {
            return;
        }
        let Some(page) = self.page.clone() else {
            return;
        };
        let Some(column) = page.columns.get(column_index).cloned() else {
            return;
        };
        let Some(row) = self.edits.insert(id) else {
            return;
        };
        let current = match row.get(&column) {
            Some(EditCell::Value(value)) => Some(value.clone()),
            Some(EditCell::Null) | None => None,
        };

        let field = cx.new(|cx| {
            let field = InputField::new(window, cx, "");
            if let Some(value) = current.as_ref() {
                field.set_text(value, window, cx);
            }
            field
        });
        field.focus_handle(cx).focus(window, cx);

        self.editing_cell = Some(EditingCell {
            target: EditTarget::New(id),
            column,
            display_row,
            column_index,
            original: current,
            field,
        });
        cx.notify();
    }

    /// Commits the inline editor's text to the edit buffer (an update for an
    /// existing row, or a new-row cell for an insert row) and closes the editor.
    /// No-op if no cell is being edited.
    ///
    /// The buffer is only touched when the typed text differs from the value the
    /// editor opened with, so pressing Enter on an unchanged cell (in particular
    /// an untouched NULL) does not record a no-op update or degrade a NULL into
    /// an empty string. Setting a value explicitly to NULL still goes through
    /// [`set_editing_cell_null`].
    pub fn commit_cell_edit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.commit_cell_edit_inner(cx);
    }

    /// The windowless core of [`commit_cell_edit`]. Committing only reads the
    /// editor field's text, so no [`Window`] is needed; this lets page-changing
    /// operations finish the editor without threading a window through.
    fn commit_cell_edit_inner(&mut self, cx: &mut Context<Self>) {
        let Some(editing) = self.editing_cell.take() else {
            return;
        };
        let value = editing.field.read(cx).text(cx);
        // Unchanged: the value equals what was shown (both non-NULL and equal),
        // or the cell was NULL and the field was left empty. Buffer nothing.
        let unchanged = match &editing.original {
            Some(original) => original == &value,
            None => value.is_empty(),
        };
        if unchanged {
            cx.notify();
            return;
        }
        match editing.target {
            EditTarget::Existing(row_key) => {
                self.set_cell_value(row_key, &editing.column, value, cx);
            }
            EditTarget::New(id) => {
                self.set_new_cell_value(id, &editing.column, value, cx);
            }
        }
        cx.notify();
    }

    /// Commits the open inline editor (if dirty) into the buffer, then closes it.
    ///
    /// This is the single lifecycle hook run before any operation that changes
    /// the on-screen row set (page swap, sort, filter, pagination, refresh) or
    /// snapshots the buffer to save. Committing is keyed by the stable
    /// [`RowKey`]/[`InsertId`] captured when the editor opened, not by the
    /// editor's display position, so it lands on the intended row even though
    /// the display is about to change. Without this, the editor would visually
    /// "move" to a different row after the swap and its next commit would write
    /// into the wrong (hidden) row.
    ///
    /// A save in flight freezes the buffer, so this is a no-op while saving.
    pub fn finish_editing(&mut self, cx: &mut Context<Self>) {
        if self.is_saving() {
            return;
        }
        if self.editing_cell.is_some() {
            // `commit_cell_edit` does not use the window (it only reads the
            // field's text), so page-changing callers that lack a `Window` can
            // still finish the editor.
            self.commit_cell_edit_inner(cx);
        }
    }

    /// Closes the inline editor without touching the edit buffer.
    pub fn cancel_cell_edit(&mut self, cx: &mut Context<Self>) {
        if self.editing_cell.take().is_some() {
            cx.notify();
        }
    }

    /// Whether the inline cell editor's own text field currently holds focus.
    /// Used to gate Enter/Escape (`menu::Confirm`/`menu::Cancel`) so they only
    /// commit or cancel the cell edit when the user is typing in that field, not
    /// when focus is in another input such as the filter value field.
    fn cell_editor_focused(&self, window: &Window, cx: &App) -> bool {
        self.editing_cell
            .as_ref()
            .is_some_and(|editing| editing.field.focus_handle(cx).contains_focused(window, cx))
    }

    /// Sets the cell currently being edited to NULL and closes the editor.
    pub fn set_editing_cell_null(&mut self, cx: &mut Context<Self>) {
        let Some(editing) = self.editing_cell.take() else {
            return;
        };
        match editing.target {
            EditTarget::Existing(row_key) => {
                self.set_cell_null(row_key, &editing.column, cx);
            }
            EditTarget::New(id) => {
                self.set_new_cell_null(id, &editing.column, cx);
            }
        }
        cx.notify();
    }

    /// Applies the buffered edits to the database in one transaction.
    ///
    /// No-op when the buffer is empty or a save is already in flight. On success
    /// the buffer and inline editor are cleared and the page is reloaded; on
    /// failure the buffer is kept and the error is surfaced in `save_state`.
    pub fn save_edits(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.save_state == SaveState::Saving {
            return;
        }
        // Commit the open editor into the buffer before snapshotting it, so an
        // in-progress cell edit is not silently dropped by the save. Must run
        // before the empty check, since finishing may add the first change.
        self.finish_editing(cx);
        if self.edits.pending_change_count() == 0 {
            return;
        }
        // `apply_edits` addresses rows by full primary key and casts each value to
        // its column type, so it needs the column metadata. Editability implies
        // the structure loaded; fall back to the page header (bare column names)
        // if it somehow has not, so the apply still runs and any error surfaces.
        let columns = match self.structure.as_ref() {
            Some(structure) => structure.columns.clone(),
            None => self
                .page
                .as_ref()
                .map(|page| page.columns.iter().map(column_info_from_name).collect())
                .unwrap_or_default(),
        };
        let edits = self.build_table_edits();

        self.editing_cell = None;
        self.save_state = SaveState::Saving;
        cx.notify();

        let client = self.client.clone();
        let table = self.table.clone();
        let task = gpui_tokio::Tokio::spawn_result(cx, async move {
            client.apply_edits(&table, &columns, &edits).await
        });

        self._save_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = task.await;
            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(counts) => {
                        this.edits.clear();
                        this.editing_cell = None;
                        this.save_state = SaveState::Done(format!(
                            "Saved: {} updated, {} inserted, {} deleted",
                            counts.updated, counts.inserted, counts.deleted
                        ));
                        this.restart_query(window, cx);
                    }
                    Err(error) => {
                        this.save_state = SaveState::Error(format!("{error:#}"));
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    /// Assembles a [`TableEdits`] from the current buffer. Deletes and updates
    /// carry the row's [`RowKey`]; inserts come from the insert buffer (empty in
    /// Task 4, wired up for Task 5).
    fn build_table_edits(&self) -> TableEdits {
        let updates = self
            .edits
            .updates()
            .iter()
            // A delete supersedes any update for the same key. `delete_row`
            // already drops the update, but guard here too so a stray update can
            // never pair with a delete for one key and fail the whole apply.
            .filter(|(key, _)| !self.edits.deletes.contains(key))
            .map(|(key, columns)| RowUpdate {
                key: key.clone(),
                set: columns
                    .iter()
                    .map(|(column, cell)| (column.clone(), cell.clone()))
                    .collect(),
            })
            .collect();
        let inserts = self
            .edits
            .inserts()
            .iter()
            .map(|(_id, columns)| RowInsert {
                values: columns
                    .iter()
                    .map(|(column, cell)| (column.clone(), cell.clone()))
                    .collect(),
            })
            .collect();
        let deletes = self
            .edits
            .deletes()
            .iter()
            .map(|key| RowDelete { key: key.clone() })
            .collect();
        TableEdits {
            updates,
            inserts,
            deletes,
        }
    }

    /// Test-only: exposes the assembled [`TableEdits`] so tests can assert on the
    /// per-section counts without going through a save.
    #[cfg(test)]
    fn build_table_edits_for_test(&self) -> TableEdits {
        self.build_table_edits()
    }

    /// Test-only: emulates a structure fetch that failed by clearing the cached
    /// structure while leaving `mode` untouched, so tests can exercise the
    /// Structure-mode retry path.
    #[cfg(test)]
    fn clear_structure_for_test(&mut self) {
        self.structure = None;
    }

    /// Cycles the sort on `column` (None -> Asc -> Desc -> None), resets the
    /// page offset, and reloads the current page.
    pub fn toggle_sort(&mut self, column: &str, window: &mut Window, cx: &mut Context<Self>) {
        // Commit and close any open editor before the rows on screen change, so
        // its next commit cannot land on a different (now-hidden) row.
        self.finish_editing(cx);
        let next = match &self.query.sort {
            Some(sort) if sort.column == column => match sort.direction {
                SortDirection::Asc => Some(Sort {
                    column: column.to_string(),
                    direction: SortDirection::Desc,
                }),
                SortDirection::Desc => None,
            },
            _ => Some(Sort {
                column: column.to_string(),
                direction: SortDirection::Asc,
            }),
        };
        self.query.sort = next;
        self.query.offset = 0;
        self.restart_query(window, cx);
    }

    /// Appends `filter` to the active filter set, resets the page offset, and
    /// reloads the current page.
    pub fn add_filter(&mut self, filter: Filter, window: &mut Window, cx: &mut Context<Self>) {
        self.finish_editing(cx);
        self.query.filters.push(filter);
        self.query.offset = 0;
        self.restart_query(window, cx);
    }

    /// Removes the filter at `index`, resets the page offset, and reloads. An
    /// out-of-bounds index is a no-op.
    pub fn remove_filter(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.query.filters.len() {
            log::debug!(
                "remove_filter: index {index} out of bounds ({} filters)",
                self.query.filters.len()
            );
            return;
        }
        self.finish_editing(cx);
        self.query.filters.remove(index);
        self.query.offset = 0;
        self.restart_query(window, cx);
    }

    /// Advances to the next page when the current page reports more rows. In
    /// custom-query mode (before any page has been run, so `limit` is not yet
    /// set) this establishes the first page at the settings page size.
    pub fn next_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let has_more = self.page.as_ref().is_some_and(|page| page.has_more);
        if !has_more {
            return;
        }
        self.finish_editing(cx);
        let page_size = DatabaseSettings::get_global(cx).page_size.max(1) as usize;
        let limit = *self.query.limit.get_or_insert(page_size);
        self.query.offset += limit;
        self.restart_query(window, cx);
    }

    /// Moves back one page, clamping the offset at zero. No-op at the first page.
    pub fn prev_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.query.offset == 0 {
            return;
        }
        self.finish_editing(cx);
        let limit = self.query.limit.unwrap_or(1);
        self.query.offset = self.query.offset.saturating_sub(limit);
        self.restart_query(window, cx);
    }

    /// Switches between the data and structure tabs, fetching the structure the
    /// first time it is shown.
    pub fn toggle_structure(&mut self, cx: &mut Context<Self>) {
        self.mode = match self.mode {
            ViewMode::Data => ViewMode::Structure,
            ViewMode::Structure => ViewMode::Data,
        };
        if self.mode == ViewMode::Structure && self.structure.is_none() {
            self.reload_structure(cx);
        }
        cx.notify();
    }

    /// Re-fetches the current page and, if the structure tab has ever been
    /// shown, the structure. Reloading structure when the Structure tab is
    /// active (even if a prior fetch failed and left it `None`) ensures Retry
    /// actually re-issues the request instead of showing "Loading structure…"
    /// forever.
    fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.restart_query(window, cx);
        if self.structure.is_some() || self.mode == ViewMode::Structure {
            self.reload_structure(cx);
        }
    }

    /// The single reload entry point: commits any open cell edit, then renders
    /// the current [`QueryState`] to SQL and spawns the run. Every mutator that
    /// changes what is on screen (sort, filter, paging, refresh, save) funnels
    /// through this one method rather than issuing its own query.
    fn restart_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.finish_editing(cx);
        // Once the SQL bar (Task 4) owns editable text, this is where the
        // editor's buffer is resynchronized from `self.query` when the two have
        // diverged; until then there is no editor to sync, so `window` is only
        // needed to spawn the task below.

        let sql = self.current_sql();
        let database = self.table.database.clone();
        let client = self.client.clone();
        let limit = self.query.limit;

        self.load_state = LoadState::Loading;
        cx.notify();

        self._data_task = Some(cx.spawn_in(window, async move |this, cx| {
            let started = std::time::Instant::now();
            let spawned = cx.update(|_, cx| {
                gpui_tokio::Tokio::spawn_result(cx, async move {
                    client.run_query(&database, &sql, UI_MAX_QUERY_ROWS).await
                })
            });
            let result = match spawned {
                Ok(task) => task.await,
                Err(error) => Err(error),
            };
            let elapsed = started.elapsed();
            this.update_in(cx, |this, _window, cx| {
                match result {
                    Ok(result) => {
                        let has_more = result.truncated
                            || limit.is_some_and(|limit| result.rows.len() == limit);
                        let row_count = result.rows.len();
                        this.set_column_widths(result.columns.len(), cx);
                        this.page = Some(Arc::new(PageData {
                            columns: result.columns,
                            rows: result.rows,
                            has_more,
                        }));
                        this.last_run = Some((row_count, elapsed));
                        this.load_state = LoadState::Idle;
                    }
                    Err(error) => {
                        this.load_state = LoadState::Error(format!("{error:#}"));
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    fn reload_structure(&mut self, cx: &mut Context<Self>) {
        let client = self.client.clone();
        let table = self.table.clone();
        let task =
            gpui_tokio::Tokio::spawn_result(
                cx,
                async move { client.table_structure(&table).await },
            );

        self._structure_task = Some(cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(structure) => {
                        this.editable = compute_editable(this.is_view, &structure.columns);
                        this.structure = Some(structure);
                        this.load_state = LoadState::Idle;
                    }
                    Err(error) => {
                        this.load_state = LoadState::Error(format!("{error:#}"));
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    /// Recreates the resizable-columns state when the number of data columns
    /// changes, so the grid renders the correct number of resize handles.
    fn set_column_widths(&mut self, cols: usize, cx: &mut Context<Self>) {
        if cols == 0 {
            self.column_widths = None;
            return;
        }
        let matches = self
            .column_widths
            .as_ref()
            .is_some_and(|widths| widths.read(cx).cols() == cols);
        if matches {
            return;
        }
        self.column_widths = Some(cx.new(|_cx| {
            ResizableColumnsState::new(
                cols,
                vec![AbsoluteLength::Pixels(px(COLUMN_WIDTH)); cols],
                vec![TableResizeBehavior::Resizable; cols],
            )
        }));
    }

    fn render_data(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let Some(page) = self.page.clone() else {
            return v_flex().into_any_element();
        };
        let Some(widths) = self.column_widths.clone() else {
            return v_flex().into_any_element();
        };

        let headers: Vec<AnyElement> = page
            .columns
            .iter()
            .enumerate()
            .map(|(index, column)| self.render_header(index, column, cx))
            .collect();

        let column_count = page.columns.len();
        let page_row_count = page.rows.len();
        // Pending insert rows are rendered below the fetched page rows, sharing
        // the same virtualized list so column widths and scrolling stay aligned.
        let insert_count = self.edits.inserts.len();
        let total_row_count = page_row_count + insert_count;

        let created_background = created_cell_background(cx);
        let deleted_background = deleted_cell_background(cx);

        Table::new(column_count)
            .interactable(&self.interaction)
            .striped()
            .width_config(ColumnWidthConfig::Resizable(widths))
            .header(headers)
            .uniform_list(
                "db-rows",
                total_row_count,
                cx.processor(move |this, range: Range<usize>, window, cx| {
                    range
                        .map(|row_index| {
                            if row_index < page_row_count {
                                (0..column_count)
                                    .map(|col| {
                                        let value = page
                                            .rows
                                            .get(row_index)
                                            .and_then(|row| row.get(col))
                                            .and_then(|cell| cell.clone());
                                        this.render_data_cell(row_index, col, value, window, cx)
                                    })
                                    .collect()
                            } else {
                                let insert_index = row_index - page_row_count;
                                (0..column_count)
                                    .map(|col| {
                                        this.render_insert_cell(insert_index, col, window, cx)
                                    })
                                    .collect()
                            }
                        })
                        .collect()
                }),
            )
            .map_row(cx.processor(move |this, (row_index, row), _window, cx| {
                this.map_data_row(
                    row_index,
                    row,
                    page_row_count,
                    created_background,
                    deleted_background,
                    cx,
                )
            }))
            .into_any_element()
    }

    /// Wraps a rendered row to signal its edit state: a pending insert row gets a
    /// created (green) tint, an existing row marked for deletion gets a red tint,
    /// and every editable row carries a hover-revealed trash button that deletes
    /// it (dropping a new row from the insert buffer, or marking an existing row
    /// for deletion).
    fn map_data_row(
        &self,
        row_index: usize,
        row: gpui::Stateful<gpui::Div>,
        page_row_count: usize,
        created_background: gpui::Hsla,
        deleted_background: gpui::Hsla,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_insert = row_index >= page_row_count;
        // Resolve the row's stable identity now, during render, rather than in
        // the click handler: a page swap between paint and click would otherwise
        // make the handler mark or discard a different row than the one shown.
        let insert_id = is_insert
            .then(|| {
                self.edits
                    .inserts
                    .get(row_index - page_row_count)
                    .map(|(id, _)| *id)
            })
            .flatten();
        let existing_key = (!is_insert).then(|| self.row_key_for(row_index)).flatten();
        let marked_deleted = existing_key
            .as_ref()
            .is_some_and(|key| self.edits.deletes.contains(key));

        let group_name = SharedString::from(format!("db-row-{row_index}"));
        let delete_button = if self.editable {
            Some(
                h_flex()
                    .absolute()
                    .right_1()
                    .top_0()
                    .bottom_0()
                    .items_center()
                    .visible_on_hover(group_name.clone())
                    .child(
                        IconButton::new(
                            ElementId::NamedInteger("db-row-delete".into(), row_index as u64),
                            IconName::Trash,
                        )
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text(if is_insert {
                            "Discard this new row"
                        } else if marked_deleted {
                            "Row marked for deletion; discard edits to undo"
                        } else {
                            "Delete this row"
                        }))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(id) = insert_id {
                                this.delete_new_row(id, cx);
                            } else if let Some(key) = existing_key.clone() {
                                this.delete_row(key, cx);
                            }
                        })),
                    ),
            )
        } else {
            None
        };

        row.group(group_name)
            .relative()
            .when(is_insert, |row| row.bg(created_background))
            .when(marked_deleted, |row| {
                row.bg(deleted_background).line_through()
            })
            .children(delete_button)
            .into_any_element()
    }

    /// Renders one data cell of an existing row, honouring the inline editor and
    /// any buffered edit: the cell being edited shows the input field with a NULL
    /// button; a cell with a buffered edit shows the new value on a highlighted
    /// background; otherwise the page value (muted `NULL` when absent).
    fn render_data_cell(
        &self,
        display_row: usize,
        column_index: usize,
        page_value: Option<String>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // The cell currently open in the inline editor (only for existing rows;
        // insert rows are rendered by `render_insert_cell`).
        if let Some(editing) = &self.editing_cell
            && matches!(editing.target, EditTarget::Existing(_))
            && editing.display_row == display_row
            && editing.column_index == column_index
        {
            return self.render_cell_editor(editing.field.clone(), cx);
        }

        let column_name = self
            .page
            .as_ref()
            .and_then(|page| page.columns.get(column_index).cloned());
        let row_key = self.row_key_for(display_row);
        let buffered = match (&row_key, &column_name) {
            (Some(key), Some(column)) => self
                .edits
                .updates()
                .get(key)
                .and_then(|columns| columns.get(column)),
            _ => None,
        };

        let modified = buffered.is_some();
        let display = match buffered {
            Some(EditCell::Value(value)) => Some(value.clone()),
            Some(EditCell::Null) => None,
            None => page_value,
        };

        let editable_here = self.editable
            && row_key.is_some()
            && column_name
                .as_ref()
                .is_some_and(|column| !self.is_primary_key_column(column));

        let mut cell = div().w_full();
        if modified {
            cell = cell.bg(modified_cell_background(cx)).rounded_sm().px_1();
        }
        let cell = match display {
            Some(value) => cell.whitespace_nowrap().text_ellipsis().child(value),
            None => cell.child(Label::new("NULL").color(Color::Muted).italic()),
        };

        if editable_here {
            div()
                .id(ElementId::NamedInteger(
                    SharedString::from(format!("db-cell-{column_index}")),
                    display_row as u64,
                ))
                .w_full()
                .cursor_pointer()
                .child(cell)
                .on_click(
                    cx.listener(move |this, event: &gpui::ClickEvent, window, cx| {
                        // A double-click opens the inline editor for this cell.
                        // `begin_edit_cell` re-resolves the row key against the
                        // current page; that is safe because it also records the
                        // key it captured, and `finish_editing` runs before any
                        // page swap, so a swap cannot strand a stale editor.
                        if event.click_count() >= 2 {
                            this.begin_edit_cell(display_row, column_index, window, cx);
                        }
                    }),
                )
                .into_any_element()
        } else {
            cell.into_any_element()
        }
    }

    /// The inline editor UI shared by existing-row and new-row cells: the text
    /// input alongside a button that sets the cell to NULL.
    fn render_cell_editor(&self, field: Entity<InputField>, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .w_full()
            .gap_1()
            .items_center()
            .child(div().flex_1().child(field))
            .child(
                Button::new("db-cell-null", "∅ NULL")
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Subtle)
                    .tooltip(Tooltip::text("Set this cell to NULL"))
                    .on_click(cx.listener(|this, _, _, cx| this.set_editing_cell_null(cx))),
            )
            .into_any_element()
    }

    /// Renders one cell of the pending insert row shown at `insert_index`. The
    /// row's stable [`InsertId`] is resolved here, during render, so the click
    /// handler targets that specific row even if the insert buffer shifts before
    /// the click. Every column is editable (including the primary key); the cell
    /// shows the buffered value, a muted `NULL` for a buffered NULL, or a muted
    /// placeholder when the column has not been set (so a column default applies
    /// on save).
    fn render_insert_cell(
        &self,
        insert_index: usize,
        column_index: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some((insert_id, row)) = self.edits.inserts.get(insert_index) else {
            return div().w_full().into_any_element();
        };
        let insert_id = *insert_id;

        if let Some(editing) = &self.editing_cell
            && editing.target == EditTarget::New(insert_id)
            && editing.column_index == column_index
        {
            return self.render_cell_editor(editing.field.clone(), cx);
        }

        let column_name = self
            .page
            .as_ref()
            .and_then(|page| page.columns.get(column_index).cloned());
        let buffered = column_name.as_ref().and_then(|column| row.get(column));

        let cell = div().w_full();
        let cell = match buffered {
            Some(EditCell::Value(value)) => cell
                .whitespace_nowrap()
                .text_ellipsis()
                .child(value.clone()),
            Some(EditCell::Null) => cell.child(Label::new("NULL").color(Color::Muted).italic()),
            None => cell.child(Label::new("default").color(Color::Muted).italic()),
        };

        if self.editable {
            div()
                .id(ElementId::NamedInteger(
                    SharedString::from(format!("db-insert-cell-{column_index}")),
                    insert_index as u64,
                ))
                .w_full()
                .cursor_pointer()
                .child(cell)
                .on_click(
                    cx.listener(move |this, event: &gpui::ClickEvent, window, cx| {
                        if event.click_count() >= 2 {
                            this.begin_edit_new_cell(
                                insert_id,
                                insert_index,
                                column_index,
                                window,
                                cx,
                            );
                        }
                    }),
                )
                .into_any_element()
        } else {
            cell.into_any_element()
        }
    }

    fn render_header(&self, index: usize, column: &str, cx: &Context<Self>) -> AnyElement {
        let sorted = self
            .query
            .sort
            .as_ref()
            .filter(|sort| sort.column == column)
            .map(|sort| sort.direction);
        let indicator = match sorted {
            Some(SortDirection::Asc) => "↑",
            Some(SortDirection::Desc) => "↓",
            None => "↕",
        };
        let tooltip = match sorted {
            Some(SortDirection::Asc) => "Sorted ascending. Click to sort descending",
            Some(SortDirection::Desc) => "Sorted descending. Click to clear sorting",
            None => "Not sorted. Click to sort ascending",
        };
        let column = column.to_string();

        h_flex()
            .justify_between()
            .items_center()
            .w_full()
            .child(Label::new(column.clone()))
            .child(
                Button::new(
                    ElementId::NamedInteger("db-sort".into(), index as u64),
                    indicator,
                )
                .size(ButtonSize::Compact)
                .style(if sorted.is_some() {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .tooltip(Tooltip::text(tooltip))
                .on_click(cx.listener(move |this, _event, window, cx| {
                    this.toggle_sort(&column, window, cx);
                })),
            )
            .into_any_element()
    }

    fn render_structure(&self) -> AnyElement {
        let Some(structure) = self.structure.as_ref() else {
            return v_flex()
                .p_4()
                .child(Label::new("Loading structure…").color(Color::Muted))
                .into_any_element();
        };

        let mut table = Table::new(6).striped().header(vec![
            "Name".into_any_element(),
            "Type".into_any_element(),
            "Nullable".into_any_element(),
            "Default".into_any_element(),
            "PK".into_any_element(),
            "FK".into_any_element(),
        ]);

        for column in &structure.columns {
            let foreign_key = structure
                .foreign_keys
                .iter()
                .find(|fk| fk.column == column.name)
                .map(|fk| {
                    format!(
                        "→ {}.{}.{}",
                        fk.references_schema, fk.references_table, fk.references_column
                    )
                })
                .unwrap_or_default();
            table = table.row(vec![
                Label::new(column.name.clone()).into_any_element(),
                Label::new(column.data_type.clone()).into_any_element(),
                Label::new(if column.is_nullable { "YES" } else { "NO" }).into_any_element(),
                Label::new(column.default.clone().unwrap_or_default()).into_any_element(),
                Label::new(if column.is_primary_key { "PK" } else { "" }).into_any_element(),
                Label::new(foreign_key).into_any_element(),
            ]);
        }

        let indexes =
            if structure.indexes.is_empty() {
                None
            } else {
                Some(
                    v_flex()
                        .pt_2()
                        .gap_1()
                        .child(Label::new("Indexes").color(Color::Muted))
                        .children(structure.indexes.iter().enumerate().map(
                            |(index_pos, index)| {
                                let definition: SharedString = index.definition.clone().into();
                                div()
                                    .id(ElementId::NamedInteger(
                                        "db-index".into(),
                                        index_pos as u64,
                                    ))
                                    .w_full()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .child(Label::new(definition.clone()).size(LabelSize::Small))
                                    .tooltip(move |_, cx| Tooltip::simple(definition.clone(), cx))
                            },
                        )),
                )
            };

        v_flex()
            .p_2()
            .gap_2()
            .child(table.into_any_element())
            .children(indexes)
            .into_any_element()
    }

    fn render_toggle(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_1()
            .child(
                Button::new("db-mode-data", "Data")
                    .toggle_state(self.mode == ViewMode::Data)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if this.mode != ViewMode::Data {
                            this.toggle_structure(cx);
                        }
                    })),
            )
            .child(
                Button::new("db-mode-structure", "Structure")
                    .toggle_state(self.mode == ViewMode::Structure)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if this.mode != ViewMode::Structure {
                            this.toggle_structure(cx);
                        }
                    })),
            )
            .into_any_element()
    }

    /// The column names offered in the filter builder, taken from the current
    /// page's header row (empty until the first page loads).
    fn available_columns(&self) -> Vec<String> {
        self.page
            .as_ref()
            .map(|page| page.columns.clone())
            .unwrap_or_default()
    }

    /// Whether the current draft is complete enough to apply: a column must be
    /// chosen, and non-`IsNull`/`IsNotNull` operators additionally require a
    /// value.
    fn draft_apply_enabled(&self, cx: &App) -> bool {
        let has_column = self.draft_column.is_some();
        let needs_value = !matches!(self.draft_op, FilterOp::IsNull | FilterOp::IsNotNull);
        let has_value = !self.draft_value.read(cx).text(cx).trim().is_empty();
        has_column && (!needs_value || has_value)
    }

    /// Commits the current draft as a new filter, closing and clearing the
    /// builder. No-op if the draft is incomplete.
    fn apply_draft_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.draft_apply_enabled(cx) {
            return;
        }
        let Some(column) = self.draft_column.clone() else {
            return;
        };
        let value = if matches!(self.draft_op, FilterOp::IsNull | FilterOp::IsNotNull) {
            String::new()
        } else {
            self.draft_value.read(cx).text(cx)
        };
        self.add_filter(
            Filter {
                column,
                op: self.draft_op,
                value,
            },
            window,
            cx,
        );
        self.filter_builder_open = false;
        self.draft_column = None;
        self.draft_op = FilterOp::Eq;
        self.draft_value
            .update(cx, |field, cx| field.clear(window, cx));
    }

    fn render_filter_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let chips = self
            .query
            .filters
            .iter()
            .enumerate()
            .map(|(index, filter)| self.render_filter_chip(index, filter, cx));

        let mut bar = h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_1()
            .flex_wrap()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .children(chips)
            .child(
                Button::new("db-add-filter", "+ Filter")
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(self.filter_builder_open)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.filter_builder_open = !this.filter_builder_open;
                        cx.notify();
                    })),
            );

        if self.filter_builder_open {
            bar = bar.child(self.render_filter_builder(window, cx));
        }

        bar.into_any_element()
    }

    fn render_filter_chip(&self, index: usize, filter: &Filter, cx: &Context<Self>) -> AnyElement {
        let text = if matches!(filter.op, FilterOp::IsNull | FilterOp::IsNotNull) {
            format!("{} {}", filter.column, filter_op_label(filter.op))
        } else {
            format!(
                "{} {} '{}'",
                filter.column,
                filter_op_label(filter.op),
                filter.value
            )
        };

        h_flex()
            .gap_1()
            .px_1p5()
            .py_0p5()
            .rounded_sm()
            .bg(cx.theme().colors().element_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(Label::new(text).size(LabelSize::Small))
            .child(
                IconButton::new(
                    ElementId::NamedInteger("db-filter-remove".into(), index as u64),
                    IconName::Close,
                )
                .icon_size(IconSize::XSmall)
                .tooltip(Tooltip::text("Remove filter"))
                .on_click(
                    cx.listener(move |this, _, window, cx| this.remove_filter(index, window, cx)),
                ),
            )
            .into_any_element()
    }

    fn render_filter_builder(&self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let columns = self.available_columns();
        let column_label = self
            .draft_column
            .clone()
            .unwrap_or_else(|| "Column".to_string());

        let column_dropdown = PopoverMenu::new("db-filter-column")
            .trigger(
                Button::new("db-filter-column-trigger", column_label).size(ButtonSize::Compact),
            )
            .anchor(Anchor::TopLeft)
            .menu({
                let this = cx.weak_entity();
                move |window, cx| {
                    let this = this.clone();
                    let columns = columns.clone();
                    Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                        for column in &columns {
                            let column = column.clone();
                            let this = this.clone();
                            menu = menu.entry(column.clone(), None, move |_, cx| {
                                this.update(cx, |this, cx| {
                                    this.draft_column = Some(column.clone());
                                    cx.notify();
                                })
                                .log_err();
                            });
                        }
                        menu
                    }))
                }
            });

        let op_dropdown = PopoverMenu::new("db-filter-op")
            .trigger(
                Button::new(
                    "db-filter-op-trigger",
                    filter_op_label(self.draft_op).to_string(),
                )
                .size(ButtonSize::Compact),
            )
            .anchor(Anchor::TopLeft)
            .menu({
                let this = cx.weak_entity();
                move |window, cx| {
                    let this = this.clone();
                    Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                        for op in all_filter_ops() {
                            let this = this.clone();
                            menu = menu.entry(filter_op_label(op), None, move |_, cx| {
                                this.update(cx, |this, cx| {
                                    this.draft_op = op;
                                    cx.notify();
                                })
                                .log_err();
                            });
                        }
                        menu
                    }))
                }
            });

        let apply_enabled = self.draft_apply_enabled(cx);
        let show_value = !matches!(self.draft_op, FilterOp::IsNull | FilterOp::IsNotNull);

        h_flex()
            .gap_1()
            .items_center()
            .child(column_dropdown)
            .child(op_dropdown)
            .when(show_value, |this| {
                this.child(div().w_40().child(self.draft_value.clone()))
            })
            .child(
                Button::new("db-filter-apply", "Apply")
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Filled)
                    .disabled(!apply_enabled)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.apply_draft_filter(window, cx);
                        cx.notify();
                    })),
            )
            .into_any_element()
    }

    fn render_footer(&self, cx: &Context<Self>) -> AnyElement {
        let (summary, has_more) = match &self.page {
            Some(page) if page.rows.is_empty() => ("No rows".to_string(), false),
            Some(page) => {
                let start = self.query.offset + 1;
                let end = self.query.offset + page.rows.len();
                let suffix = if page.has_more { "+" } else { "" };
                (format!("rows {start}–{end}{suffix}"), page.has_more)
            }
            None => (String::new(), false),
        };
        let at_start = self.query.offset == 0;

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(summary)
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("db-prev-page", IconName::ChevronLeft)
                            .icon_size(IconSize::Small)
                            .disabled(at_start)
                            .tooltip(Tooltip::text("Previous page"))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.prev_page(window, cx)),
                            ),
                    )
                    .child(
                        IconButton::new("db-refresh", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Refresh"))
                            .on_click(cx.listener(|this, _, window, cx| this.refresh(window, cx))),
                    )
                    .child(
                        IconButton::new("db-next-page", IconName::ChevronRight)
                            .icon_size(IconSize::Small)
                            .disabled(!has_more)
                            .tooltip(Tooltip::text("Next page"))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.next_page(window, cx)),
                            ),
                    ),
            )
            .into_any_element()
    }

    /// The bar shown in the tab header area while there are pending edits (a
    /// change count plus Save/Discard), or, for a read-only table, a muted banner
    /// explaining why editing is off. Returns `None` when there is nothing to
    /// show (an editable table with no pending edits).
    fn render_edit_toolbar(&self, cx: &Context<Self>) -> Option<AnyElement> {
        if !self.editable {
            // Only explain read-only once the structure has loaded, so the banner
            // does not flash before editability is known.
            let reason = if self.structure.is_none() {
                return None;
            } else if self.is_view {
                "Read-only: this is a view"
            } else {
                "Read-only: table has no primary key"
            };
            return Some(
                h_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Label::new(reason)
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .into_any_element(),
            );
        }

        let pending = self.pending_change_count();
        let saving = self.save_state == SaveState::Saving;
        let dirty = pending > 0 || saving;

        // The "+ Row" button is always available on an editable table so a row
        // can be added even when the buffer is otherwise empty.
        let add_row_button = Button::new("db-add-row", "+ Row")
            .size(ButtonSize::Compact)
            .style(ButtonStyle::Subtle)
            .disabled(saving)
            .tooltip(Tooltip::text("Add a new row"))
            .on_click(cx.listener(|this, _, _, cx| {
                this.add_row(cx);
            }));

        // The left cluster shows the change count while dirty, otherwise the last
        // save outcome (success/error) so the user still sees confirmation.
        let status = if dirty {
            let summary = if pending == 1 {
                "1 change".to_string()
            } else {
                format!("{pending} changes")
            };
            Some((summary, Color::Default))
        } else {
            match &self.save_state {
                SaveState::Done(message) => Some((message.clone(), Color::Success)),
                SaveState::Error(message) => Some((message.clone(), Color::Error)),
                _ => None,
            }
        };
        let inline_error = (dirty && matches!(self.save_state, SaveState::Error(_))).then(|| {
            match &self.save_state {
                SaveState::Error(message) => message.clone(),
                _ => String::new(),
            }
        });

        let mut bar = h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_2()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border);
        if dirty {
            bar = bar.bg(modified_cell_background(cx));
        }

        Some(
            bar.child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .when_some(status, |this, (text, color)| {
                        this.child(Label::new(text).color(color).size(LabelSize::Small))
                    })
                    .when_some(inline_error, |this, message| {
                        this.child(
                            Label::new(message)
                                .color(Color::Error)
                                .size(LabelSize::Small),
                        )
                    }),
            )
            .child(h_flex().gap_1().child(add_row_button).when(dirty, |this| {
                this.child(
                    Button::new("db-discard-edits", "Discard")
                        .size(ButtonSize::Compact)
                        .style(ButtonStyle::Subtle)
                        .disabled(saving)
                        .on_click(cx.listener(|this, _, _, cx| this.discard_edits(cx))),
                )
                .child(
                    Button::new("db-save-edits", "Save")
                        .size(ButtonSize::Compact)
                        .style(ButtonStyle::Filled)
                        .disabled(saving)
                        .on_click(cx.listener(|this, _, window, cx| this.save_edits(window, cx))),
                )
            }))
            .into_any_element(),
        )
    }

    fn render_error(&self, message: &str, cx: &Context<Self>) -> AnyElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .child(Label::new(message.to_string()).color(Color::Error))
            .child(
                Button::new("db-retry", "Retry")
                    .on_click(cx.listener(|this, _, window, cx| this.refresh(window, cx))),
            )
            .into_any_element()
    }
}

impl Render for TableDataView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = match (&self.load_state, self.mode) {
            (LoadState::Error(message), _) => self.render_error(&message.clone(), cx),
            (_, ViewMode::Structure) => self.render_structure(),
            (_, ViewMode::Data) => self.render_data(cx),
        };
        let in_data =
            self.mode == ViewMode::Data && !matches!(self.load_state, LoadState::Error(_));
        let filter_bar = in_data.then(|| self.render_filter_bar(window, cx));
        let edit_toolbar = in_data.then(|| self.render_edit_toolbar(cx)).flatten();

        v_flex()
            .key_context("TableDataView")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &NextPage, window, cx| this.next_page(window, cx)))
            .on_action(cx.listener(|this, _: &PrevPage, window, cx| this.prev_page(window, cx)))
            .on_action(cx.listener(|this, _: &ToggleStructure, _, cx| this.toggle_structure(cx)))
            .on_action(cx.listener(|this, _: &RefreshData, window, cx| this.refresh(window, cx)))
            .on_action(
                cx.listener(|this, _: &CommitCellEdit, window, cx| {
                    this.commit_cell_edit(window, cx)
                }),
            )
            .on_action(cx.listener(|this, _: &CancelCellEdit, _, cx| this.cancel_cell_edit(cx)))
            .on_action(cx.listener(|this, _: &SetCellNull, _, cx| this.set_editing_cell_null(cx)))
            // Enter/Escape reach here as menu::Confirm/Cancel. Only claim them
            // when the cell editor's own field is focused, so Enter/Escape in a
            // different input (e.g. the filter value field) is not hijacked to
            // commit or cancel a cell edit; otherwise let them bubble.
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                if this.cell_editor_focused(window, cx) {
                    this.commit_cell_edit(window, cx);
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| {
                if this.cell_editor_focused(window, cx) {
                    this.cancel_cell_edit(cx);
                } else {
                    cx.propagate();
                }
            }))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new(format!(
                        "{}.{}",
                        self.table.schema, self.table.name
                    )))
                    .child(self.render_toggle(cx)),
            )
            .children(edit_toolbar)
            .children(filter_bar)
            .child(v_flex().flex_1().size_full().overflow_hidden().child(body))
            .when(in_data, |this| this.child(self.render_footer(cx)))
    }
}

impl Focusable for TableDataView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for TableDataView {}

impl Item for TableDataView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileTree))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("{}.{}", self.table.schema, self.table.name).into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(
            format!(
                "{}.{}.{}",
                self.table.database, self.table.schema, self.table.name
            )
            .into(),
        )
    }
}

/// The background tint for a cell with a buffered (unsaved) edit. Reuses the
/// version-control "modified" accent (an amber-ish hue, deliberately not the red
/// used for errors) at low opacity so the underlying value stays readable.
fn modified_cell_background(cx: &App) -> gpui::Hsla {
    cx.theme().colors().version_control_modified.opacity(0.2)
}

/// The background tint for a pending insert (new) row. Reuses the version-control
/// "added" accent (a green hue) at low opacity so the row reads as created.
fn created_cell_background(cx: &App) -> gpui::Hsla {
    cx.theme().colors().version_control_added.opacity(0.2)
}

/// The background tint for an existing row marked for deletion. Reuses the
/// version-control "deleted" accent (a red hue) at low opacity so the row reads
/// as pending removal without hiding its still-recoverable content.
fn deleted_cell_background(cx: &App) -> gpui::Hsla {
    cx.theme().colors().version_control_deleted.opacity(0.15)
}

/// Opens (or activates an existing) table data tab in the workspace's active
/// pane, de-duplicating by connection name plus [`TableRef`] (identical table
/// coordinates on different connections must not alias to one tab).
pub fn open_table_tab(
    workspace: &WeakEntity<Workspace>,
    client: Arc<dyn DatabaseClient>,
    connection: String,
    table: TableRef,
    is_view: bool,
    window: &mut Window,
    cx: &mut App,
) {
    workspace
        .update(cx, |workspace, cx| {
            let existing = workspace
                .active_pane()
                .read(cx)
                .items_of_type::<TableDataView>()
                .find(|view| {
                    let view = view.read(cx);
                    view.connection() == connection && view.table() == &table
                });
            let view = existing.unwrap_or_else(|| {
                TableDataView::new(client, connection, table, is_view, window, cx)
            });
            workspace.active_pane().update(cx, |pane, cx| {
                if let Some(index) = pane.index_for_item(&view) {
                    pane.activate_item(index, true, true, window, cx);
                } else {
                    pane.add_item(Box::new(view), true, true, None, window, cx);
                }
            });
        })
        .log_err();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use database_client::fake::FakeDatabaseClient;
    use database_client::{
        ColumnInfo, DatabaseClient, Filter, FilterOp, QueryResult, RowKey, SortDirection, TableRef,
    };
    use gpui::{TestAppContext, VisualTestContext};

    use super::{
        LoadState, SaveState, TableDataView, ViewMode, all_filter_ops, compute_editable,
        filter_op_label,
    };

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            gpui_tokio::init(cx);
            crate::init(cx);
        });
    }

    fn table_ref() -> TableRef {
        TableRef {
            database: "app".into(),
            schema: "public".into(),
            name: "users".into(),
        }
    }

    /// Builds a canned `QueryResult` of `id`/`name` rows, `count` of them, for
    /// seeding the fake so a page comes back with exactly `count` rows (e.g. to
    /// drive the `has_more == rows.len() == limit` heuristic).
    fn rows_result(count: usize) -> QueryResult {
        QueryResult {
            columns: vec!["id".into(), "name".into()],
            rows: (0..count)
                .map(|i| vec![Some((i + 1).to_string()), Some(format!("row{i}"))])
                .collect(),
            truncated: false,
            command_tag: None,
        }
    }

    /// The default three-row `id`/`name` page (row 2 has a NULL `name`) used by
    /// the editing tests, matching the fake's structure (an `id` primary key).
    fn default_rows_result() -> QueryResult {
        QueryResult {
            columns: vec!["id".into(), "name".into()],
            rows: vec![
                vec![Some("1".into()), Some("Alice".into())],
                vec![Some("2".into()), Some("Bob".into())],
                vec![Some("3".into()), None],
            ],
            truncated: false,
            command_tag: Some("SELECT 3".into()),
        }
    }

    /// A fake client seeded with [`default_rows_result`], the row shape the
    /// editing tests key their `RowKey`s and assertions against.
    fn fake_with_default_rows() -> Arc<FakeDatabaseClient> {
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = default_rows_result();
        Arc::new(fake)
    }

    /// Returns the SQL text of the most recent `run_query` call, if any.
    fn last_run_query_sql(fake: &FakeDatabaseClient) -> Option<String> {
        fake.calls()
            .iter()
            .rev()
            .find_map(|call| call.split_once("sql=").map(|(_, sql)| sql.to_string()))
    }

    /// Drives the deterministic scheduler while giving the real tokio runtime a
    /// chance to complete cross-thread work, until `condition` holds or a bound
    /// is reached. Requires `cx.executor().allow_parking()`.
    ///
    /// Operates on a [`VisualTestContext`], which derefs into the underlying
    /// [`TestAppContext`] for scheduler and timer control.
    async fn wait_until(
        cx: &mut VisualTestContext,
        condition: impl Fn(&mut VisualTestContext) -> bool,
    ) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(5))
                .await;
        }
        cx.run_until_parked();
        assert!(
            condition(cx),
            "condition did not become true within the time bound"
        );
    }

    #[gpui::test]
    async fn table_view_loads_first_page(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });

        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.read_with(cx, |view, _| {
            assert!(view.page().is_some(), "first page should be loaded");
            assert_eq!(
                view.query().limit,
                Some(100),
                "limit comes from page_size setting"
            );
            assert_eq!(view.load_state(), &LoadState::Idle);
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.ends_with(r#"SELECT * FROM "public"."users" LIMIT 100 OFFSET 0;"#),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn sort_click_resets_offset_and_reloads(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = rows_result(100);
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Advance to a non-zero offset first so the reset is observable, and
        // let that load settle so its fetch is recorded before we sort.
        view.update_in(cx, |view, window, cx| view.next_page(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().offset == 100 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| view.toggle_sort("name", window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().sort.is_some() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            let sort = view.query().sort.as_ref().expect("sort should be set");
            assert_eq!(sort.column, "name");
            assert_eq!(sort.direction, SortDirection::Asc);
            assert_eq!(view.query().offset, 0, "sorting resets offset to 0");
        });

        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.ends_with(
                r#"SELECT * FROM "public"."users" ORDER BY "name" ASC LIMIT 100 OFFSET 0;"#
            ),
            "unexpected generated SQL: {last}"
        );

        let run_calls = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        assert!(
            run_calls >= 3,
            "expected initial + next_page + sort runs, got {run_calls}"
        );
    }

    #[gpui::test]
    async fn next_prev_page_updates_offset(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        // Exactly `limit` rows -> has_more == true, so next_page advances.
        fake.query_result = rows_result(100);
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Wait for each load to settle so its fetch is recorded (the abort-on-
        // supersede behaviour would otherwise drop an in-flight fetch).
        view.update_in(cx, |view, window, cx| view.next_page(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().offset == 100 && view.load_state() == &LoadState::Idle
            })
        })
        .await;
        view.read_with(cx, |view, _| assert_eq!(view.query().offset, 100));

        view.update_in(cx, |view, window, cx| view.prev_page(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().offset == 0 && view.load_state() == &LoadState::Idle
            })
        })
        .await;
        view.read_with(cx, |view, _| assert_eq!(view.query().offset, 0));

        // prev_page at offset 0 is a no-op.
        let before = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        view.update_in(cx, |view, window, cx| view.prev_page(window, cx));
        cx.run_until_parked();
        let after = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        assert_eq!(before, after, "prev_page at offset 0 should not refetch");
        view.read_with(cx, |view, _| assert_eq!(view.query().offset, 0));
    }

    #[gpui::test]
    async fn structure_mode_fetches_structure_once(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update(cx, |view, cx| view.toggle_structure(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;
        view.read_with(cx, |view, _| {
            assert!(view.structure().is_some());
            assert_eq!(view.mode(), ViewMode::Structure);
        });

        let structure_calls_first = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("table_structure"))
            .count();
        assert_eq!(structure_calls_first, 1);

        // Toggle back to Data and again to Structure: no second structure fetch.
        view.update(cx, |view, cx| view.toggle_structure(cx));
        cx.run_until_parked();
        view.update(cx, |view, cx| view.toggle_structure(cx));
        cx.run_until_parked();

        let structure_calls_second = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("table_structure"))
            .count();
        assert_eq!(
            structure_calls_second, 1,
            "structure should be cached after first fetch"
        );
    }

    #[gpui::test]
    async fn refresh_reloads_data_even_with_cached_structure(cx: &mut TestAppContext) {
        // Regression: a data reload and a structure reload used to share one
        // task field, so refresh() with a cached structure aborted its own data
        // fetch. They now use separate fields and must coexist.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Cache the structure, then return to the Data tab.
        view.update(cx, |view, cx| view.toggle_structure(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;
        view.update(cx, |view, cx| view.toggle_structure(cx));
        cx.run_until_parked();

        let fetches_before = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();

        // refresh() must reload the data (not just the cached structure) and end
        // Idle with a page still present.
        view.update_in(cx, |view, window, cx| view.refresh(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        let fetches_after = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        assert!(
            fetches_after > fetches_before,
            "refresh must issue a new data fetch (before={fetches_before}, after={fetches_after})"
        );
        view.read_with(cx, |view, _| {
            assert!(view.page().is_some(), "data page must survive the refresh");
            assert!(
                view.structure().is_some(),
                "cached structure must also be reloaded"
            );
            assert_eq!(view.load_state(), &LoadState::Idle);
        });
    }

    #[gpui::test]
    async fn refresh_in_structure_mode_refetches_after_error(cx: &mut TestAppContext) {
        // Regression: a failed structure fetch leaves structure=None while mode
        // is Structure; refresh()/Retry used to skip the structure reload because
        // structure.is_some() was false, stranding "Loading structure…" forever.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Switch to Structure (mode=Structure, structure will load).
        view.update(cx, |view, cx| view.toggle_structure(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;

        // Simulate the failed-fetch state: mode is Structure but structure=None.
        view.update(cx, |view, _cx| view.clear_structure_for_test());
        let structure_calls_before = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("table_structure"))
            .count();

        view.update_in(cx, |view, window, cx| view.refresh(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;

        let structure_calls_after = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("table_structure"))
            .count();
        assert!(
            structure_calls_after > structure_calls_before,
            "refresh in Structure mode must refetch the structure even when it is None"
        );
    }

    #[gpui::test]
    async fn tab_dedup_distinguishes_connections(cx: &mut TestAppContext) {
        // Regression: dedup keyed only on TableRef, so identical table
        // coordinates on different connections aliased to one tab. The dedup key
        // now includes the connection name.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let staging = cx.update(|window, cx| {
            TableDataView::new(
                client.clone(),
                "staging".into(),
                table_ref(),
                false,
                window,
                cx,
            )
        });
        let prod = cx.update(|window, cx| {
            TableDataView::new(client, "prod".into(), table_ref(), false, window, cx)
        });

        staging.read_with(cx, |staging, _| {
            prod.read_with(cx, |prod, _| {
                assert_eq!(
                    staging.table(),
                    prod.table(),
                    "the two tabs share table coordinates"
                );
                assert_ne!(
                    staging.connection(),
                    prod.connection(),
                    "but differ by connection, so they must not alias to one tab"
                );
                // The dedup predicate used by open_table_tab.
                let same_tab =
                    staging.connection() == prod.connection() && staging.table() == prod.table();
                assert!(!same_tab, "different connections must yield different tabs");
            });
        });
    }

    #[gpui::test]
    async fn load_error_is_surfaced(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::with_error("connection refused"));
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                matches!(view.load_state(), LoadState::Error(_))
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            let LoadState::Error(message) = view.load_state() else {
                panic!("expected error load state, got {:?}", view.load_state());
            };
            assert!(
                message.contains("connection refused"),
                "unexpected error message: {message}"
            );
        });
    }

    #[gpui::test]
    async fn add_filter_resets_offset_and_reloads(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = rows_result(100);
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Advance to a non-zero offset so the reset is observable, letting the
        // load settle before we add a filter.
        view.update_in(cx, |view, window, cx| view.next_page(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().offset == 100 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.add_filter(
                Filter {
                    column: "name".into(),
                    op: FilterOp::Contains,
                    value: "ali".into(),
                },
                window,
                cx,
            )
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().filters.len() == 1 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.query().filters.len(), 1, "filter should be stored");
            assert_eq!(view.query().offset, 0, "adding a filter resets the offset");
        });

        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.ends_with(
                r#"SELECT * FROM "public"."users" WHERE "name"::text ILIKE '%ali%' LIMIT 100 OFFSET 0;"#
            ),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn remove_filter_reloads_without_filters(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| {
            view.add_filter(
                Filter {
                    column: "name".into(),
                    op: FilterOp::Eq,
                    value: "Alice".into(),
                },
                window,
                cx,
            )
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().filters.len() == 1 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        // Removing an out-of-bounds index is a no-op and does not refetch.
        let before = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        view.update_in(cx, |view, window, cx| view.remove_filter(5, window, cx));
        cx.run_until_parked();
        let after = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        assert_eq!(
            before, after,
            "out-of-bounds remove_filter should not refetch"
        );
        view.read_with(cx, |view, _| assert_eq!(view.query().filters.len(), 1));

        view.update_in(cx, |view, window, cx| view.remove_filter(0, window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().filters.is_empty() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.query().filters.is_empty(), "filter should be removed");
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.ends_with(r#"SELECT * FROM "public"."users" LIMIT 100 OFFSET 0;"#),
            "removing the filter should trigger a query without a WHERE clause: {last}"
        );
    }

    #[gpui::test]
    fn filter_op_labels_cover_all_ops(_cx: &mut TestAppContext) {
        assert_eq!(filter_op_label(FilterOp::Eq), "=");
        assert_eq!(filter_op_label(FilterOp::NotEq), "≠");
        assert_eq!(filter_op_label(FilterOp::Gt), ">");
        assert_eq!(filter_op_label(FilterOp::Lt), "<");
        assert_eq!(filter_op_label(FilterOp::Contains), "contains");
        assert_eq!(filter_op_label(FilterOp::IsNull), "is null");
        assert_eq!(filter_op_label(FilterOp::IsNotNull), "is not null");

        let ops = all_filter_ops();
        assert_eq!(ops.len(), 7, "there should be seven filter operators");
        for op in ops {
            assert!(
                !filter_op_label(op).is_empty(),
                "every operator needs a label"
            );
        }
    }

    fn col(name: &str, is_primary_key: bool) -> ColumnInfo {
        ColumnInfo {
            name: name.into(),
            data_type: "text".into(),
            udt_name: "text".into(),
            udt_schema: "pg_catalog".into(),
            is_nullable: true,
            default: None,
            is_primary_key,
        }
    }

    #[gpui::test]
    fn compute_editable_gate(_cx: &mut TestAppContext) {
        // A base table with a primary key is editable.
        assert!(compute_editable(
            false,
            &[col("id", true), col("name", false)]
        ));
        // A view is never editable, even with a primary key.
        assert!(!compute_editable(
            true,
            &[col("id", true), col("name", false)]
        ));
        // A base table with no primary key is not editable.
        assert!(!compute_editable(
            false,
            &[col("name", false), col("email", false)]
        ));
        // No columns at all is not editable.
        assert!(!compute_editable(false, &[]));
    }

    #[gpui::test]
    async fn editable_gate_true_for_pk_table(cx: &mut TestAppContext) {
        // The fake's structure has an `id` primary key, and `is_view = false`
        // is passed, so the loaded table is editable.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.editable(), "PK base table should be editable");
        });
    }

    #[gpui::test]
    async fn editable_gate_false_for_view(cx: &mut TestAppContext) {
        // Even though the fake structure carries a PK, passing `is_view = true`
        // makes the table read-only.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), true, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(!view.editable(), "a view should never be editable");
        });
    }

    #[gpui::test]
    async fn structure_loaded_with_first_page(cx: &mut TestAppContext) {
        // Structure is loaded eagerly on tab open, alongside the first page,
        // so PK/editability are known without switching to Structure mode.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.read_with(cx, |view, _| {
            assert!(
                view.structure().is_some(),
                "structure should be loaded eagerly with the first page"
            );
            assert_eq!(
                view.mode(),
                ViewMode::Data,
                "eager structure load must not change the active mode"
            );
        });
    }

    #[gpui::test]
    async fn buffer_edits_change_pending_count(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update(cx, |view, cx| {
            assert_eq!(view.pending_change_count(), 0);

            // Row 0 is id=1 in the fake page; the PK column is `id`.
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            assert_eq!(key.columns, vec!["id".to_string()]);
            assert_eq!(key.values, vec![Some("1".to_string())]);

            // Editing a non-PK cell records an update.
            view.set_cell_value(key.clone(), "name", "Alicia".into(), cx);
            assert_eq!(view.pending_change_count(), 1);

            // A second edit on the same row does not add another change.
            view.set_cell_null(key.clone(), "name", cx);
            assert_eq!(view.pending_change_count(), 1);

            // Editing the PK column is a no-op.
            view.set_cell_value(key, "id", "999".into(), cx);
            assert_eq!(view.pending_change_count(), 1);

            // Adding a row and deleting a row each add one pending change.
            view.add_row(cx);
            assert_eq!(view.pending_change_count(), 2);

            let key2 = view.row_key_for(1).expect("row 1 should yield a RowKey");
            view.delete_row(key2, cx);
            assert_eq!(view.pending_change_count(), 3);

            // Discarding clears everything.
            view.discard_edits(cx);
            assert_eq!(view.pending_change_count(), 0);
        });
    }

    #[gpui::test]
    async fn begin_edit_cell_gated_by_pk_and_editability(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        // Column 1 is `name` (not a PK): editing begins on the editable table.
        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            assert!(
                view.editing_cell().is_some(),
                "editing a non-PK cell of an editable table should begin an edit"
            );
            // The editor pre-fills with the cell's current display value.
            let text = view
                .editing_cell()
                .map(|editing| editing.field.read(cx).text(cx));
            assert_eq!(text.as_deref(), Some("Alice"));
        });

        // Column 0 is `id` (the PK): editing is a no-op.
        view.update_in(cx, |view, window, cx| {
            view.cancel_cell_edit(cx);
            view.begin_edit_cell(0, 0, window, cx);
            assert!(
                view.editing_cell().is_none(),
                "editing a primary-key cell must not begin an edit"
            );
        });
    }

    #[gpui::test]
    async fn begin_edit_cell_no_op_for_view(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        // `is_view = true` makes the table read-only even though it carries a PK.
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), true, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            assert!(!view.editable(), "a view should never be editable");
            view.begin_edit_cell(0, 1, window, cx);
            assert!(
                view.editing_cell().is_none(),
                "a read-only view must not begin cell edits"
            );
        });
    }

    #[gpui::test]
    async fn commit_cell_edit_buffers_update(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            let editing = view.editing_cell().expect("edit should be in progress");
            editing.field.update(cx, |field, cx| {
                field.set_text("Alicia", window, cx);
            });
            view.commit_cell_edit(window, cx);
        });

        view.read_with(cx, |view, _| {
            assert!(
                view.editing_cell().is_none(),
                "committing clears the inline editor"
            );
            assert_eq!(view.pending_change_count(), 1);
            let key = RowKey {
                columns: vec!["id".into()],
                values: vec![Some("1".into())],
            };
            let cell = view
                .edits()
                .updates()
                .get(&key)
                .and_then(|row| row.get("name"))
                .expect("the committed update should be buffered for name");
            assert_eq!(cell, &database_client::EditCell::Value("Alicia".into()));
        });
    }

    #[gpui::test]
    async fn set_editing_cell_null_buffers_null(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            view.set_editing_cell_null(cx);
        });

        view.read_with(cx, |view, _| {
            assert!(
                view.editing_cell().is_none(),
                "setting NULL closes the editor"
            );
            let key = RowKey {
                columns: vec!["id".into()],
                values: vec![Some("1".into())],
            };
            let cell = view
                .edits()
                .updates()
                .get(&key)
                .and_then(|row| row.get("name"))
                .expect("NULL should be buffered for name");
            assert_eq!(cell, &database_client::EditCell::Null);
        });
    }

    #[gpui::test]
    async fn save_noop_when_clean(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            assert_eq!(view.pending_change_count(), 0);
            view.save_edits(window, cx);
        });
        cx.run_until_parked();

        assert!(
            !fake
                .calls()
                .iter()
                .any(|call| call.starts_with("apply_edits")),
            "saving a clean buffer must not call apply_edits: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn save_applies_and_clears(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            view.set_cell_value(key, "name", "Alicia".into(), cx);
            assert_eq!(view.pending_change_count(), 1);
            view.save_edits(window, cx);
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.pending_change_count() == 0)
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=1 i=0 d=0"),
            "save must call apply_edits with one update: {:?}",
            fake.calls()
        );
        // A successful save reloads the page (a fresh run follows the apply).
        let runs = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        assert!(runs >= 2, "save success should reload the page");
        view.read_with(cx, |view, _| {
            assert_eq!(view.pending_change_count(), 0, "buffer cleared on success");
            assert!(matches!(view.save_state(), SaveState::Done(_)));
        });
    }

    #[gpui::test]
    async fn save_error_keeps_buffer(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::with_error("permission denied"));
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        // The fake fails every call, so structure never loads and editable stays
        // false; buffer the update directly to exercise the save error path.
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                matches!(view.load_state(), LoadState::Error(_))
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            let key = RowKey {
                columns: vec!["id".into()],
                values: vec![Some("1".into())],
            };
            view.set_cell_value(key, "name", "Alicia".into(), cx);
            assert_eq!(view.pending_change_count(), 1);
            view.save_edits(window, cx);
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                matches!(view.save_state(), SaveState::Error(_))
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(
                view.pending_change_count(),
                1,
                "the buffer must survive a failed save"
            );
            let SaveState::Error(message) = view.save_state() else {
                panic!("expected a save error, got {:?}", view.save_state());
            };
            assert!(
                message.contains("permission denied"),
                "unexpected save error: {message}"
            );
        });
    }

    #[gpui::test]
    async fn delete_row_drops_pending_update(cx: &mut TestAppContext) {
        // Deleting a row that has a buffered update removes the redundant update
        // so it counts once, as a delete.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update(cx, |view, cx| {
            let key = RowKey {
                columns: vec!["id".into()],
                values: vec![Some("1".into())],
            };
            view.set_cell_value(key.clone(), "name", "Alicia".into(), cx);
            assert_eq!(view.pending_change_count(), 1);
            view.delete_row(key, cx);
            assert_eq!(
                view.pending_change_count(),
                1,
                "deleting an updated row collapses to a single delete"
            );
            assert!(view.edits().updates.is_empty());
            assert_eq!(view.edits().deletes.len(), 1);
        });
    }

    #[gpui::test]
    async fn add_row_then_save_inserts(cx: &mut TestAppContext) {
        // Adding a new row and setting one of its cells produces exactly one
        // insert in the applied edits (u=0 i=1 d=0).
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            let id = view.add_row(cx).expect("add_row yields an insert id");
            assert_eq!(view.pending_change_count(), 1);
            // The new row's cells are editable via the same inline editor, and
            // PK cells are editable for new rows (a value can be supplied).
            view.begin_edit_new_cell(id, 0, 0, window, cx);
            let editing = view
                .editing_cell()
                .expect("editing the new row's id cell should begin an edit");
            editing.field.update(cx, |field, cx| {
                field.set_text("42", window, cx);
            });
            view.commit_cell_edit(window, cx);
        });

        view.read_with(cx, |view, _| {
            assert_eq!(view.edits().inserts().len(), 1);
            let cell = view
                .edits()
                .inserts()
                .first()
                .and_then(|(_id, row)| row.get("id"))
                .expect("the new row's id cell should be buffered");
            assert_eq!(cell, &database_client::EditCell::Value("42".into()));
        });

        view.update_in(cx, |view, window, cx| view.save_edits(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.pending_change_count() == 0)
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=0 i=1 d=0"),
            "adding and saving a row must apply one insert: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn delete_existing_row_then_save(cx: &mut TestAppContext) {
        // Deleting an existing page row and saving applies exactly one delete.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            view.delete_row(key, cx);
            assert_eq!(view.pending_change_count(), 1);
            assert_eq!(view.edits().deletes().len(), 1);
            view.save_edits(window, cx);
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.pending_change_count() == 0)
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=0 i=0 d=1"),
            "deleting and saving a row must apply one delete: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn delete_new_row_removes_from_inserts(cx: &mut TestAppContext) {
        // Deleting a not-yet-saved insert row drops it from the insert buffer
        // rather than recording a delete.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update(cx, |view, cx| {
            let id = view.add_row(cx).expect("add_row yields an insert id");
            assert_eq!(view.edits().inserts().len(), 1);
            view.delete_new_row(id, cx);
            assert!(
                view.edits().inserts().is_empty(),
                "deleting a new row must drop it from inserts"
            );
            assert!(
                view.edits().deletes().is_empty(),
                "deleting a new row must not record a delete"
            );
            assert_eq!(view.pending_change_count(), 0);
        });
    }

    #[gpui::test]
    async fn mixed_edits_build_correct(cx: &mut TestAppContext) {
        // One update, one insert, and one delete assemble into a TableEdits with
        // one entry in each section; a successful save clears the buffer and
        // reloads the page.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            // Update row 0 (id=1), delete row 1 (id=2), insert a new row.
            let update_key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            view.set_cell_value(update_key, "name", "Alicia".into(), cx);
            let delete_key = view.row_key_for(1).expect("row 1 should yield a RowKey");
            view.delete_row(delete_key, cx);
            let insert_id = view.add_row(cx).expect("add_row yields an insert id");
            view.set_new_cell_value(insert_id, "name", "Zoe".into(), cx);
            assert_eq!(view.pending_change_count(), 3);

            let edits = view.build_table_edits_for_test();
            assert_eq!(edits.updates.len(), 1, "one update section entry");
            assert_eq!(edits.inserts.len(), 1, "one insert section entry");
            assert_eq!(edits.deletes.len(), 1, "one delete section entry");

            view.save_edits(window, cx);
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.pending_change_count() == 0)
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=1 i=1 d=1"),
            "mixed edits must apply as u=1 i=1 d=1: {:?}",
            fake.calls()
        );
        let runs = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("run_query"))
            .count();
        assert!(runs >= 2, "a successful save should reload the page");
        view.read_with(cx, |view, _| {
            assert_eq!(view.pending_change_count(), 0, "buffer cleared on success");
            assert!(matches!(view.save_state(), SaveState::Done(_)));
        });
    }

    #[gpui::test]
    async fn finish_editing_on_page_change_commits_by_key(cx: &mut TestAppContext) {
        // Regression (Critical): a page change must close the open inline editor,
        // committing its edit keyed by the row's stable RowKey — not leave it open
        // to write into whatever row later lands at the same display position.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        // Open the editor on row 0 (id=1), column `name`, and type a new value
        // without committing.
        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            let editing = view.editing_cell().expect("edit should be open");
            editing.field.update(cx, |field, cx| {
                field.set_text("Alicia", window, cx);
            });
        });

        // A sort (page-changing op) must finish the editor first.
        view.update_in(cx, |view, window, cx| view.toggle_sort("name", window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(
                view.editing_cell().is_none(),
                "a page change must close the inline editor"
            );
            // The edit landed on id=1's name, keyed by its stable RowKey.
            let key = RowKey {
                columns: vec!["id".into()],
                values: vec![Some("1".into())],
            };
            let cell = view
                .edits()
                .updates()
                .get(&key)
                .and_then(|row| row.get("name"))
                .expect("the in-progress edit should be committed keyed by RowKey");
            assert_eq!(cell, &database_client::EditCell::Value("Alicia".into()));
        });
    }

    #[gpui::test]
    async fn finish_editing_on_reload_keeps_editor_closed(cx: &mut TestAppContext) {
        // reload via next_page must also close the editor and commit its edit.
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        // next_page is gated on has_more, which needs a full page of rows.
        fake.query_result = rows_result(100);
        let client: Arc<dyn DatabaseClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            let editing = view.editing_cell().expect("edit should be open");
            editing.field.update(cx, |field, cx| {
                field.set_text("Alicia", window, cx);
            });
            view.next_page(window, cx);
            assert!(
                view.editing_cell().is_none(),
                "paging must close the inline editor"
            );
            assert_eq!(
                view.pending_change_count(),
                1,
                "the in-progress edit is committed to the buffer, not lost"
            );
        });
    }

    #[gpui::test]
    async fn save_commits_open_editor(cx: &mut TestAppContext) {
        // Saving while a cell editor is open must commit its edit before the
        // snapshot, rather than dropping the typed text.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            let editing = view.editing_cell().expect("edit should be open");
            editing.field.update(cx, |field, cx| {
                field.set_text("Alicia", window, cx);
            });
            // Save with the editor still open: the edit must be committed.
            view.save_edits(window, cx);
            assert!(
                view.editing_cell().is_none(),
                "saving closes the inline editor"
            );
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.pending_change_count() == 0)
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=1 i=0 d=0"),
            "save must apply the just-typed edit as one update: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn edits_during_in_flight_save_are_ignored(cx: &mut TestAppContext) {
        // While a save is in flight the edit buffer is frozen, so a programmatic
        // mutation is ignored; the success handler then clears the exact snapshot
        // that was applied.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            view.set_cell_value(key, "name", "Alicia".into(), cx);
            view.save_edits(window, cx);
            // The save is now in flight (state == Saving). Every buffer mutation
            // must be a no-op until it settles.
            assert_eq!(view.save_state(), &SaveState::Saving);
            view.add_row(cx);
            let key1 = view.row_key_for(1).expect("row 1 should yield a RowKey");
            view.delete_row(key1, cx);
            view.set_cell_value(
                view.row_key_for(2).expect("row 2 should yield a RowKey"),
                "name",
                "Nope".into(),
                cx,
            );
            assert_eq!(
                view.pending_change_count(),
                1,
                "no mutation should take effect during an in-flight save"
            );
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.pending_change_count() == 0)
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=1 i=0 d=0"),
            "only the pre-save snapshot (one update) should be applied: {:?}",
            fake.calls()
        );
        view.read_with(cx, |view, _| {
            assert_eq!(
                view.pending_change_count(),
                0,
                "the buffer is cleared to the applied snapshot on success"
            );
        });
    }

    #[gpui::test]
    async fn edit_of_row_marked_for_deletion_is_no_op(cx: &mut TestAppContext) {
        // A row already marked for deletion must not gather an update, or the
        // apply would carry both a delete and an update for the same key and fail.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            view.delete_row(key.clone(), cx);

            // Direct set is a no-op on a deletion-marked row.
            view.set_cell_value(key, "name", "Alicia".into(), cx);
            assert!(
                view.edits().updates().is_empty(),
                "editing a deletion-marked row must not buffer an update"
            );

            // Opening the inline editor on that row is likewise refused.
            view.begin_edit_cell(0, 1, window, cx);
            assert!(
                view.editing_cell().is_none(),
                "the editor must not open on a row marked for deletion"
            );
            assert_eq!(view.pending_change_count(), 1, "still just the one delete");
        });
    }

    #[gpui::test]
    async fn delete_row_closes_open_editor(cx: &mut TestAppContext) {
        // Deleting a row that is currently being edited must close that editor,
        // so a later page swap cannot commit an update for the now-deleted key.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            assert!(view.editing_cell().is_some());
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            view.delete_row(key, cx);
            assert!(
                view.editing_cell().is_none(),
                "deleting the edited row must close its editor"
            );
        });
    }

    #[gpui::test]
    async fn untouched_null_is_not_degraded_to_empty_string(cx: &mut TestAppContext) {
        // Row 2 (id=3) has a NULL `name`. Opening its editor and committing
        // without typing must leave the cell NULL, not record an empty-string
        // update.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(2, 1, window, cx);
            let editing = view.editing_cell().expect("edit should be open");
            // A NULL cell pre-fills empty and was NULL originally.
            assert_eq!(editing.field.read(cx).text(cx), "");
            view.commit_cell_edit(window, cx);
            assert!(
                view.editing_cell().is_none(),
                "committing closes the editor"
            );
            assert_eq!(
                view.pending_change_count(),
                0,
                "committing an untouched NULL must not buffer any change"
            );
        });

        // Also: committing an untouched non-NULL value records nothing.
        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            assert_eq!(
                view.editing_cell().map(|e| e.field.read(cx).text(cx)),
                Some("Alice".to_string())
            );
            view.commit_cell_edit(window, cx);
            assert_eq!(
                view.pending_change_count(),
                0,
                "committing an unchanged value must not buffer a no-op update"
            );
        });
    }

    #[gpui::test]
    async fn build_table_edits_delete_supersedes_update(cx: &mut TestAppContext) {
        // Even if an update ends up buffered for a key that is also in `deletes`,
        // build_table_edits must not emit the update: the delete wins.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update(cx, |view, cx| {
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            // Buffer an update, then mark the same row for deletion. delete_row
            // drops the update; force a stray update back in to exercise the
            // build-time guard directly.
            view.set_cell_value(key.clone(), "name", "Alicia".into(), cx);
            view.delete_row(key.clone(), cx);
            view.edits.updates.entry(key).or_default().insert(
                "name".into(),
                database_client::EditCell::Value("Stray".into()),
            );

            let edits = view.build_table_edits_for_test();
            assert_eq!(
                edits.updates.len(),
                0,
                "a key in deletes must not also appear as an update"
            );
            assert_eq!(edits.deletes.len(), 1, "the delete is emitted");
        });
    }

    #[gpui::test]
    async fn new_row_survives_deletion_of_earlier_new_row(cx: &mut TestAppContext) {
        // Insert rows are addressed by stable id, so deleting an earlier pending
        // insert must not shift or invalidate the identity of a later one.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update(cx, |view, cx| {
            let first = view.add_row(cx).expect("first insert id");
            let second = view.add_row(cx).expect("second insert id");
            assert_ne!(first, second, "each insert gets a distinct id");
            view.set_new_cell_value(first, "name", "First".into(), cx);
            view.set_new_cell_value(second, "name", "Second".into(), cx);
            assert_eq!(view.edits().inserts().len(), 2);

            // Delete the first row by its id. The second must keep its value,
            // addressable by the same id that still resolves after the shift.
            view.delete_new_row(first, cx);
            assert_eq!(view.edits().inserts().len(), 1);

            // Editing the second row via its original id still targets it.
            view.set_new_cell_value(second, "name", "Second!".into(), cx);
            let cell = view
                .edits()
                .inserts()
                .iter()
                .find(|(id, _)| *id == second)
                .and_then(|(_, row)| row.get("name"))
                .expect("second row is still present and addressable by its id");
            assert_eq!(cell, &database_client::EditCell::Value("Second!".into()));

            // A stale id (the deleted first row) is a harmless no-op.
            view.set_new_cell_value(first, "name", "Ghost".into(), cx);
            assert_eq!(view.edits().inserts().len(), 1, "no phantom row appears");
        });
    }
}
