use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

use database_client::{
    ColumnInfo, DatabaseClient, EditCell, Filter, FilterOp, RowDelete, RowInsert, RowKey,
    RowUpdate, Sort, SortDirection, TableEdits, TableRef, TableStructure,
};
use editor::{Editor, EditorEvent, EditorMode};
use gpui::{
    Anchor, AnyElement, App, Context, DismissEvent, ElementId, Entity, EventEmitter, FocusHandle,
    Focusable, MouseButton, MouseDownEvent, Pixels, Point, SharedString, Subscription, Task,
    WeakEntity, Window, actions, anchored, deferred,
};
use language::{Buffer, LanguageRegistry};
use multi_buffer::MultiBuffer;
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
use crate::query_state::{QueryBase, QueryState, render_sql};
use crate::sql_query_view::RunQuery;

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

/// Formats the footer's row-range summary from the current page and query
/// state: `rows {offset+1}–{offset+len}{"+" if more rows exist beyond this
/// page}`, `"No rows"` for an empty page, or `"{len} rows"` (or `"{len}+
/// rows"` when the result was truncated) for a custom query that has not yet
/// acquired a `limit` (a fresh custom query has no page size to frame an
/// offset against). The `+` in the truncated case matters: without it the
/// count reads as an exact total, but a custom query capped at
/// `UI_MAX_QUERY_ROWS` may have far more matching rows that were never
/// fetched (finding 5).
fn footer_counter(offset: usize, row_count: usize, has_more: bool, limit: Option<usize>) -> String {
    if row_count == 0 {
        return "No rows".to_string();
    }
    if limit.is_none() {
        let suffix = if has_more { "+" } else { "" };
        return format!("{row_count}{suffix} rows");
    }
    let start = offset + 1;
    let end = offset + row_count;
    let suffix = if has_more { "+" } else { "" };
    format!("rows {start}–{end}{suffix}")
}

/// The footer's save-outcome message and color, if any, given the current
/// `save_state` and pending change count.
///
/// An error is always surfaced: a failed save leaves the buffer intact (see
/// `save_edits`), so the error renders alongside the still-visible
/// change-controls, telling the user why nothing was applied. A success
/// message only makes sense once the buffer that produced it has actually
/// cleared - showing "Saved" next to a fresh, unrelated pending change would
/// misattribute the new edit as already saved (finding 4).
fn footer_save_result(save_state: &SaveState, pending: usize) -> Option<(String, Color)> {
    match save_state {
        SaveState::Error(message) => Some((message.clone(), Color::Error)),
        SaveState::Done(message) if pending == 0 => Some((message.clone(), Color::Success)),
        _ => None,
    }
}

/// The effective page size to use for a new or resized page: the configured
/// `DatabaseSettings::page_size`, clamped to `[1, UI_MAX_QUERY_ROWS]`.
///
/// `run_query` never returns more than `UI_MAX_QUERY_ROWS` rows (the server
/// truncates and reports `has_more`), so a `LIMIT` above that ceiling would
/// render a page size the query can never actually satisfy: pagination would
/// advance the offset by more than a page's worth of rows can ever be
/// fetched, silently skipping the rows in between (finding 0). Clamping here,
/// at every site that turns the setting into a `limit`, keeps the rendered
/// `LIMIT` always achievable.
fn configured_page_size(cx: &App) -> usize {
    (DatabaseSettings::get_global(cx).page_size.max(1) as usize).min(UI_MAX_QUERY_ROWS)
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

/// The `udt_name`s treated as numeric for grid right-alignment purposes.
const NUMERIC_UDT_NAMES: &[&str] = &[
    "int2", "int4", "int8", "numeric", "float4", "float8", "money", "oid",
];

/// Names of columns whose `udt_name` is numeric, used to right-align their
/// values in the data grid. Pure so the mapping can be unit-tested without a
/// live structure fetch.
fn numeric_column_names(columns: &[ColumnInfo]) -> HashSet<String> {
    columns
        .iter()
        .filter(|column| NUMERIC_UDT_NAMES.contains(&column.udt_name.as_str()))
        .map(|column| column.name.clone())
        .collect()
}

/// The minimum and maximum auto-measured column width, in pixels.
const MIN_COLUMN_WIDTH: f32 = 60.0;
const MAX_COLUMN_WIDTH: f32 = 480.0;

/// Converts a measured character count into a column width in pixels: the
/// widest content (`advance` per character times `chars`) plus the grid's
/// horizontal cell padding (`4px` per side, see `render_cell` in
/// `ui::data_table`) and a small slack margin, clamped to a sane range so a
/// single very wide value cannot blow out the whole table and an empty
/// column still has room for its header and resize handle.
fn column_width_for_chars(advance_px: f32, chars: usize) -> f32 {
    let content_width = advance_px * chars as f32;
    (content_width + 8. + 12.).clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH)
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
    /// The visible, editable SQL bar. Its text is always in sync with either
    /// the rendered [`QueryState`] (via [`Self::sync_editor_text`]) or, while
    /// `sql_dirty`, the user's own unsynced edits.
    sql_editor: Entity<Editor>,
    /// Whether the SQL bar is collapsed to just its chevron row.
    sql_bar_collapsed: bool,
    /// Whether the SQL editor's text has diverged from `render_sql(&self.query)`
    /// by a real (non-programmatic) edit. While `true`, row editing is
    /// suspended (see `editable()`). UI-driven query mutators (sort/filter/
    /// paging/refresh) do not silently apply on top of the stale query while
    /// dirty: each one first runs the bar's current text (as `run_from_editor`
    /// would - refreshing in place if unchanged, or promoting to a custom
    /// query if hand-edited) and only then layers its own change on top, so
    /// the user's typed SQL is never discarded by an unrelated click.
    sql_dirty: bool,
    /// A non-navigational notice shown near the SQL bar, e.g. explaining why a
    /// run was refused because it would have entered custom-query mode with a
    /// non-empty edit buffer (see [`Self::run_from_editor`]). Cleared on the
    /// next successful run.
    pending_edits_notice: Option<String>,
    /// Set around programmatic `set_text` calls so the resulting `BufferEdited`
    /// event is not mistaken for a real user edit.
    suppress_editor_events: bool,
    /// Wrapped in `Arc` so the render hot path (scroll re-renders) hands the
    /// rows to `uniform_list` by cheap clone instead of deep-copying every cell.
    page: Option<Arc<PageData>>,
    structure: Option<TableStructure>,
    /// Names of `structure`'s columns whose `udt_name` is numeric, kept in
    /// sync with `structure` so the grid can right-align them without
    /// re-scanning the column list on every cell render.
    numeric_columns: HashSet<String>,
    load_state: LoadState,
    interaction: Entity<TableInteractionState>,
    /// Recreated whenever the rendered column set changes so the grid keeps the
    /// right number of resize handles.
    column_widths: Option<Entity<ResizableColumnsState>>,
    /// The row count and wall-clock duration of the most recent successful
    /// query run, shown in the footer. `None` before the first page loads.
    last_run: Option<(usize, Duration)>,
    /// The right-click menu deployed on a data cell, anchored at the cursor
    /// position it opened at. The `Subscription` watches the menu's
    /// `DismissEvent` to clear this field; dropping the tuple drops both.
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    /// The "View value" popup deployed from the cell context menu, anchored
    /// at the cursor position the menu opened at.
    value_popover: Option<(Entity<ValuePopover>, Point<Pixels>, Subscription)>,
    /// Held separately from `_structure_task` so a structure load and a data
    /// reload can be in flight at the same time without one aborting the other.
    _data_task: Option<Task<()>>,
    _structure_task: Option<Task<()>>,
    /// The in-flight save task, if any. Held so `save_state == Saving` reliably
    /// gates against concurrent saves and the work is cancelled on drop.
    _save_task: Option<Task<()>>,
    /// Watches the SQL editor for user edits to maintain `sql_dirty`.
    _editor_subscription: Subscription,
}

impl TableDataView {
    pub fn new(
        client: Arc<dyn DatabaseClient>,
        connection: String,
        table: TableRef,
        is_view: bool,
        language_registry: Option<Arc<LanguageRegistry>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let page_size = configured_page_size(cx);
        let query = QueryState::for_table(table.clone(), page_size);
        let initial_sql = render_sql(&query);
        cx.new(|cx| {
            let interaction = cx.new(|cx| TableInteractionState::new(cx));

            let sql_editor = cx.new(|cx| {
                let buffer = cx.new(|cx| {
                    let buffer = Buffer::local(initial_sql.clone(), cx);
                    if let Some(language_registry) = language_registry.clone() {
                        buffer.set_language_registry(language_registry);
                    }
                    buffer
                });
                let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                let mut editor = Editor::new(
                    EditorMode::AutoHeight {
                        min_lines: 1,
                        max_lines: Some(5),
                    },
                    buffer,
                    None,
                    window,
                    cx,
                );
                editor.set_show_gutter(false, cx);
                editor
            });
            if let Some(language_registry) = language_registry {
                cx.spawn(async move |this, cx| {
                    let sql = language_registry.language_for_name("SQL").await.ok();
                    if sql.is_none() {
                        log::debug!("SQL language unavailable; SQL bar stays plain text");
                    }
                    // Closing the tab before the language resolves releases the
                    // entity; that is the expected race, so ignore it rather
                    // than logging a spurious error on every quick close.
                    this.update(cx, |this: &mut Self, cx| {
                        if let Some(buffer) =
                            this.sql_editor.read(cx).buffer().read(cx).as_singleton()
                        {
                            buffer.update(cx, |buffer, cx| buffer.set_language(sql, cx));
                        }
                    })
                    .ok();
                })
                .detach();
            }

            let editor_subscription = cx.subscribe(&sql_editor, |this: &mut Self, _, event, cx| {
                let EditorEvent::BufferEdited = event else {
                    return;
                };
                if this.suppress_editor_events {
                    return;
                }
                let text = this.sql_editor.read(cx).text(cx);
                let now_dirty = text != render_sql(&this.query);
                // Transitioning into dirty leaves the editable window (see
                // `editable()`): finish (commit-if-changed) any inline cell
                // editor still open now, while it is still allowed to write to
                // the buffer, rather than leaving it interactively open on top
                // of a row set the dirty SQL bar is about to replace (finding 3
                // in the stage-3 review).
                if now_dirty && !this.sql_dirty {
                    this.finish_editing(cx);
                }
                this.sql_dirty = now_dirty;
                cx.notify();
            });

            let mut view = Self {
                focus_handle: cx.focus_handle(),
                client,
                connection,
                query,
                table,
                is_view,
                editable: false,
                edits: TableEditBuffer::default(),
                next_insert_id: 0,
                editing_cell: None,
                save_state: SaveState::Idle,
                mode: ViewMode::Data,
                sql_editor,
                sql_bar_collapsed: false,
                sql_dirty: false,
                pending_edits_notice: None,
                suppress_editor_events: false,
                page: None,
                structure: None,
                numeric_columns: HashSet::new(),
                load_state: LoadState::Idle,
                interaction,
                column_widths: None,
                last_run: None,
                context_menu: None,
                value_popover: None,
                _data_task: None,
                _structure_task: None,
                _save_task: None,
                _editor_subscription: editor_subscription,
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

    /// Test-only: exposes whether the cell context menu is deployed.
    #[cfg(test)]
    fn context_menu_open(&self) -> bool {
        self.context_menu.is_some()
    }

    /// Test-only: exposes the "View value" popover's current text, if open.
    #[cfg(test)]
    fn value_popover_text(&self, cx: &App) -> Option<SharedString> {
        self.value_popover
            .as_ref()
            .map(|(popover, _, _)| popover.read(cx).value.clone())
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

    /// Whether rows in this table can be edited right now: the structure says
    /// so (base table with a primary key, `false` until the structure has
    /// loaded), the SQL bar has no unsynced edit, and the query is still
    /// table-backed rather than a custom query. Custom SQL results are
    /// read-only because there is no table to address `UPDATE`/`DELETE`
    /// statements against.
    pub fn editable(&self) -> bool {
        self.editable && !self.sql_dirty && matches!(self.query.base, QueryBase::Table(_))
    }

    /// A short, human-readable reason the table's rows cannot be edited right
    /// now, shown as a muted label in the footer; `None` when editing is
    /// possible or when a reason is already surfaced elsewhere (the
    /// "Custom query · read-only" badge in the SQL bar covers the custom-query
    /// and dirty-SQL cases). Distinct from `!editable()`: this stays `None`
    /// until the structure has loaded, since "no primary key" cannot be
    /// claimed before the columns are known.
    fn read_only_reason(&self) -> Option<&'static str> {
        if self.mode != ViewMode::Data
            || self.sql_dirty
            || !matches!(self.query.base, QueryBase::Table(_))
        {
            return None;
        }
        let structure = self.structure.as_ref()?;
        if self.is_view {
            Some("Read-only: view")
        } else if !structure.columns.iter().any(|column| column.is_primary_key) {
            Some("Read-only: no primary key")
        } else {
            None
        }
    }

    /// Whether the SQL bar's text has diverged from `render_sql(&self.query)`
    /// by a user edit not yet run.
    pub fn sql_dirty(&self) -> bool {
        self.sql_dirty
    }

    /// A notice to show near the SQL bar explaining why the last run was
    /// refused, if any (see [`Self::run_from_editor`]).
    pub fn pending_edits_notice(&self) -> Option<&str> {
        self.pending_edits_notice.as_deref()
    }

    /// Whether the SQL bar is collapsed to just its chevron row.
    pub fn sql_bar_collapsed(&self) -> bool {
        self.sql_bar_collapsed
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
        if !self.editable() || self.is_saving() {
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
        if !self.editable() || self.is_saving() {
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

    /// Test-only: exposes whether a buffered update would be layered onto the
    /// cell for `row_key`/`column` under the same table-backed gate
    /// `render_data_cell` applies, without going through the full render path.
    #[cfg(test)]
    fn cell_display_value_for_test(&self, row_key: &RowKey, column: &str) -> Option<String> {
        if !matches!(self.query.base, QueryBase::Table(_)) {
            return None;
        }
        self.cell_display_value(row_key, column, None)
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
        if !self.editable() || self.is_saving() {
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
        if !self.editable() || self.is_saving() {
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
    ///
    /// Defense-in-depth: if editing somehow became disallowed while the editor
    /// was open (`editable()` is `false`), the editor is still closed but its
    /// text is discarded rather than written to the buffer. The primary guard
    /// against this is the `BufferEdited` handler finishing the editor at the
    /// moment the bar goes dirty, while it is still editable; this branch only
    /// guards against that invariant ever slipping (finding 3).
    fn commit_cell_edit_inner(&mut self, cx: &mut Context<Self>) {
        let Some(editing) = self.editing_cell.take() else {
            return;
        };
        if !self.editable() {
            cx.notify();
            return;
        }
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
    ///
    /// Defense-in-depth: see [`Self::commit_cell_edit_inner`] — if editing
    /// somehow became disallowed while the editor was open, the editor is
    /// still closed but the NULL is not buffered.
    pub fn set_editing_cell_null(&mut self, cx: &mut Context<Self>) {
        let Some(editing) = self.editing_cell.take() else {
            return;
        };
        if !self.editable() {
            cx.notify();
            return;
        }
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
    /// No-op when not editable (defense-in-depth: the buffer is supposed to be
    /// empty whenever `!editable()`, see finding 2), the buffer is empty, or a
    /// save is already in flight. On success the buffer and inline editor are
    /// cleared and the page is reloaded; on failure the buffer is kept and the
    /// error is surfaced in `save_state`.
    pub fn save_edits(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
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
    ///
    /// While the bar is dirty, first runs the bar's text (see
    /// [`Self::commit_dirty_bar`]) and applies the sort on top of that result;
    /// a refused run (finding 2) leaves the sort untouched.
    pub fn toggle_sort(&mut self, column: &str, window: &mut Window, cx: &mut Context<Self>) {
        if !self.commit_dirty_bar(window, cx) {
            return;
        }
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

    /// Applies a filter created or edited via [`FilterPopover`]: `Some(index)`
    /// replaces the filter at that position (a no-op if `index` is out of
    /// bounds), `None` appends `filter` as a new one. Either way resets the
    /// page offset and reloads the current page.
    ///
    /// While the bar is dirty, first runs the bar's text (see
    /// [`Self::commit_dirty_bar`]) and applies the filter on top of that
    /// result; a refused run (finding 2) leaves the filter unapplied.
    pub fn apply_filter_edit(
        &mut self,
        index: Option<usize>,
        filter: Filter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.commit_dirty_bar(window, cx) {
            return;
        }
        match index {
            Some(index) => {
                let Some(existing) = self.query.filters.get_mut(index) else {
                    log::debug!(
                        "apply_filter_edit: index {index} out of bounds ({} filters)",
                        self.query.filters.len()
                    );
                    return;
                };
                *existing = filter;
            }
            None => self.query.filters.push(filter),
        }
        self.query.offset = 0;
        self.finish_editing(cx);
        self.restart_query(window, cx);
    }

    /// Removes the filter at `index`, resets the page offset, and reloads. An
    /// out-of-bounds index is a no-op.
    ///
    /// While the bar is dirty, first runs the bar's text (see
    /// [`Self::commit_dirty_bar`]); a refused run (finding 2) leaves the
    /// filter list untouched.
    pub fn remove_filter(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if !self.commit_dirty_bar(window, cx) {
            return;
        }
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
    ///
    /// A previously unpaginated query (fresh custom SQL that has never had a
    /// `limit`) that overran `UI_MAX_QUERY_ROWS` has already shown up to that
    /// many rows with no `OFFSET`; the first explicit page must therefore
    /// continue after what is already on screen (`offset = page.rows.len()`)
    /// rather than jump only `page_size` rows in, which would just re-show
    /// rows already displayed (finding 11). Ordinary table pagination is
    /// unaffected since `limit` is always `Some` there already.
    ///
    /// While the bar is dirty, first runs the bar's text (see
    /// [`Self::commit_dirty_bar`]) and evaluates `has_more` against that
    /// freshly loaded page; a refused run (finding 2) leaves paging untouched.
    pub fn next_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.commit_dirty_bar(window, cx) {
            return;
        }
        let Some(page) = self.page.as_ref() else {
            return;
        };
        if !page.has_more {
            return;
        }
        let was_unpaginated = self.query.limit.is_none();
        let shown_rows = page.rows.len();
        self.finish_editing(cx);
        let page_size = configured_page_size(cx);
        let limit = *self.query.limit.get_or_insert(page_size);
        if was_unpaginated {
            self.query.offset = shown_rows;
        } else {
            self.query.offset += limit;
        }
        self.restart_query(window, cx);
    }

    /// Moves back one page, clamping the offset at zero. No-op at the first
    /// page.
    ///
    /// While the bar is dirty, first runs the bar's text (see
    /// [`Self::commit_dirty_bar`]) and evaluates the first-page guard against
    /// that freshly loaded query; a refused run (finding 2) leaves paging
    /// untouched.
    pub fn prev_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.commit_dirty_bar(window, cx) {
            return;
        }
        if self.query.offset == 0 {
            return;
        }
        self.finish_editing(cx);
        let limit = self.query.limit.unwrap_or(1);
        self.query.offset = self.query.offset.saturating_sub(limit);
        self.restart_query(window, cx);
    }

    /// Changes the page size from the footer's page-size picker: commits any
    /// open cell editor, sets the new limit, resets to the first page, and
    /// reruns the query.
    ///
    /// While the bar is dirty, first runs the bar's text (see
    /// [`Self::commit_dirty_bar`]) and applies the new page size on top of
    /// that result; a refused run (finding 2) leaves the page size untouched.
    pub fn set_page_size(&mut self, page_size: usize, window: &mut Window, cx: &mut Context<Self>) {
        if !self.commit_dirty_bar(window, cx) {
            return;
        }
        self.finish_editing(cx);
        // Clamped defensively (finding 0): `run_query` never returns more
        // than `UI_MAX_QUERY_ROWS` rows, so a larger limit would render a
        // page size pagination can never actually deliver, even though the
        // picker itself already only offers clamped choices.
        self.query.limit = Some(page_size.clamp(1, UI_MAX_QUERY_ROWS));
        self.query.offset = 0;
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
    ///
    /// While the bar is dirty, Refresh means "run what's in the bar" (see
    /// [`Self::commit_dirty_bar`]), not "re-run the stale query" - otherwise it
    /// would silently discard the user's unrun edit (finding 1). A refused run
    /// (finding 2) leaves the query untouched and skips the reload.
    fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.commit_dirty_bar(window, cx) {
            return;
        }
        self.restart_query(window, cx);
        if self.structure.is_some() || self.mode == ViewMode::Structure {
            self.reload_structure(cx);
        }
    }

    /// The single reload entry point: commits any open cell edit, resyncs the
    /// SQL bar's text from `self.query`, then runs the rendered SQL. Every
    /// mutator that changes what is on screen (sort, filter, paging, refresh,
    /// save) funnels through this one method rather than issuing its own query,
    /// which is what keeps the SQL bar's text always equal to the executed
    /// query — the invariant this view is built around.
    ///
    /// Skips resyncing the editor text when the bar is dirty: every mutator
    /// that changes `self.query` runs `commit_dirty_bar` first, so by the time
    /// it reaches here the bar is already clean. The one caller that can still
    /// be dirty here is the save success handler, which reloads data with the
    /// existing `self.query` after a save completes; if the user has since
    /// started typing a new query, overwriting that unsaved text out from
    /// under them would be exactly the bug finding 1 flags, so it is left
    /// alone (only the data reloads).
    fn restart_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.finish_editing(cx);
        if !self.sql_dirty {
            self.sync_editor_text(window, cx);
        }

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
            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(result) => {
                        let has_more = result.truncated
                            || limit.is_some_and(|limit| result.rows.len() == limit);
                        let row_count = result.rows.len();
                        let page = PageData {
                            columns: result.columns,
                            rows: result.rows,
                            has_more,
                        };
                        this.set_column_widths(&page, window, cx);
                        this.page = Some(Arc::new(page));
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

    /// Overwrites the SQL bar's text with `render_sql(&self.query)`, under a
    /// guard that stops the resulting `BufferEdited` event from being taken for
    /// a real user edit, and clears `sql_dirty`. Called whenever `self.query`
    /// changes so the visible text and the executed query never drift apart.
    fn sync_editor_text(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let sql = self.current_sql();
        self.suppress_editor_events = true;
        self.sql_editor.update(cx, |editor, cx| {
            editor.set_text(sql, window, cx);
        });
        self.suppress_editor_events = false;
        self.sql_dirty = false;
    }

    /// Runs the SQL bar's current text (`RunQuery`/cmd-enter): commits any open
    /// cell edit first, then either refreshes the existing query (when the text
    /// is unchanged from `current_sql()`) or, when the text was hand-edited,
    /// enters custom-query mode with a fresh overlay and runs that instead.
    ///
    /// Entering custom mode with a non-empty edit buffer is refused: there is
    /// no way to reconcile buffered updates/inserts/deletes keyed against the
    /// current table with a result set that may not even come from that table.
    /// The buffer and the bar's dirty text are both left untouched and
    /// [`Self::pending_edits_notice`] is set so the UI can explain why the run
    /// did not happen (finding 2). Returns whether the run actually happened
    /// (`false` when refused).
    pub fn run_from_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let text = self.sql_editor.read(cx).text(cx);
        if text == self.current_sql() {
            self.finish_editing(cx);
            self.pending_edits_notice = None;
            self.restart_query(window, cx);
            return true;
        }
        if self.edits.pending_change_count() > 0 {
            let count = self.edits.pending_change_count();
            let plural = if count == 1 { "change" } else { "changes" };
            self.pending_edits_notice = Some(format!(
                "Save or discard your {count} pending {plural} before running a custom query"
            ));
            cx.notify();
            return false;
        }
        self.finish_editing(cx);
        self.pending_edits_notice = None;
        self.query = QueryState::for_custom(text);
        self.sql_dirty = false;
        self.restart_query(window, cx);
        true
    }

    /// The dirty-bar half of every UI query mutator (sort/filter/paging/
    /// refresh): while the bar is clean this is a no-op that returns `true`
    /// immediately. While dirty, it first runs the bar's hand-typed text
    /// exactly as [`Self::run_from_editor`] would - refreshing in place if the
    /// text is unchanged, or promoting to a custom query otherwise - and only
    /// then lets the caller layer its own change on top of the result. This is
    /// the "promote" model chosen for finding 1: a UI action taken while dirty
    /// means "run what's in the bar, then do this", so the user's typed SQL is
    /// never silently discarded by an unrelated click.
    ///
    /// Returns `false` when the run was refused (a non-empty edit buffer
    /// blocked entering custom mode, per finding 2); callers must not apply
    /// their own change in that case; the bar stays dirty and
    /// [`Self::pending_edits_notice`] explains why.
    fn commit_dirty_bar(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if !self.sql_dirty {
            return true;
        }
        self.run_from_editor(window, cx)
    }

    /// Leaves custom-query mode, rebuilding a plain [`QueryState::for_table`]
    /// query over this tab's table at the current page-size setting, and runs
    /// it. The SQL bar's text is resynced to the freshly generated SELECT.
    pub fn reset_to_table_query(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let page_size = configured_page_size(cx);
        self.query = QueryState::for_table(self.table.clone(), page_size);
        self.restart_query(window, cx);
    }

    fn handle_run_query(&mut self, _: &RunQuery, window: &mut Window, cx: &mut Context<Self>) {
        self.run_from_editor(window, cx);
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
                // Structure and data loads run independently and can be in
                // flight at the same time (see `refresh`), so this must not
                // touch `load_state`: that field reflects only the data
                // load's own success/error handler in `restart_query`.
                // Writing `Idle` here used to race a concurrent data fetch
                // and mask its error banner with a stale grid (finding 6);
                // the Structure tab already renders its own "Loading
                // structure…" placeholder from `structure.is_none()` and
                // needs no separate load state.
                match result {
                    Ok(structure) => {
                        this.editable = compute_editable(this.is_view, &structure.columns);
                        this.numeric_columns = numeric_column_names(&structure.columns);
                        this.structure = Some(structure);
                    }
                    Err(error) => {
                        log::error!("failed to load table structure: {error:#}");
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    /// Recreates the resizable-columns state when the number of data columns
    /// changes, so the grid renders the correct number of resize handles.
    /// When it does (re)create the state, seeds it with widths measured from
    /// the page's header and values instead of a flat default, so wide values
    /// are not clipped on first paint. When the column count is unchanged the
    /// existing entity (and any manual resizes the user made) is left alone.
    fn set_column_widths(&mut self, page: &PageData, window: &mut Window, cx: &mut Context<Self>) {
        let cols = page.columns.len();
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
        let widths = self
            .measured_column_widths(page, window, cx)
            .unwrap_or_else(|| vec![px(COLUMN_WIDTH); cols]);
        self.column_widths = Some(cx.new(|_cx| {
            ResizableColumnsState::new(
                cols,
                widths
                    .into_iter()
                    .map(AbsoluteLength::Pixels)
                    .collect::<Vec<_>>(),
                vec![TableResizeBehavior::Resizable; cols],
            )
        }));
    }

    /// Measures each column's width from its header name and the first 100
    /// rows' values, using the buffer font's per-character advance (exact for
    /// a monospace font, a reasonable approximation otherwise). Returns `None`
    /// only if the page has no columns; a `Window`/text-system failure falls
    /// back to a fixed per-character advance rather than failing the whole
    /// measurement.
    fn measured_column_widths(
        &self,
        page: &PageData,
        window: &Window,
        cx: &App,
    ) -> Option<Vec<Pixels>> {
        if page.columns.is_empty() {
            return None;
        }
        const FALLBACK_ADVANCE: f32 = 8.;
        const SAMPLE_ROWS: usize = 100;
        const NULL_CHAR_COUNT: usize = 4; // "NULL"

        let settings = theme::theme_settings(cx);
        let font = settings.buffer_font(cx).clone();
        let font_size = TextSize::default().rems(cx).to_pixels(window.rem_size());
        let font_id = window.text_system().resolve_font(&font);
        let advance = match window.text_system().em_advance(font_id, font_size) {
            Ok(advance) => f32::from(advance),
            Err(error) => {
                log::debug!("em_advance failed, falling back to fixed char width: {error:#}");
                FALLBACK_ADVANCE
            }
        };

        let widths = page
            .columns
            .iter()
            .enumerate()
            .map(|(col, name)| {
                let mut max_chars = name.chars().count();
                for row in page.rows.iter().take(SAMPLE_ROWS) {
                    let chars = match row.get(col).and_then(|cell| cell.as_ref()) {
                        Some(value) => value.chars().count(),
                        None => NULL_CHAR_COUNT,
                    };
                    max_chars = max_chars.max(chars);
                }
                px(column_width_for_chars(advance, max_chars))
            })
            .collect();
        Some(widths)
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
        // Only shown for a table-backed query: a custom query's result set has
        // no relation to the buffer's original table, so inserts must not be
        // layered onto it (see finding 2 in the stage-3 review).
        let insert_count = if matches!(self.query.base, QueryBase::Table(_)) {
            self.edits.inserts.len()
        } else {
            0
        };
        let total_row_count = page_row_count + insert_count;

        let created_background = created_cell_background(cx);
        let deleted_background = deleted_cell_background(cx);

        Table::new(column_count)
            .interactable(&self.interaction)
            .striped()
            .header_background(cx.theme().colors().title_bar_background)
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
        // Buffered deletes are only meaningful for a table-backed query: a
        // custom query's columns may coincidentally line up with the original
        // table's primary-key names, and matching against it would falsely
        // strike through rows the custom query never marked (finding 2).
        let is_table_backed = matches!(self.query.base, QueryBase::Table(_));
        let existing_key = (!is_insert && is_table_backed)
            .then(|| self.row_key_for(row_index))
            .flatten();
        let marked_deleted = existing_key
            .as_ref()
            .is_some_and(|key| self.edits.deletes.contains(key));

        let group_name = SharedString::from(format!("db-row-{row_index}"));
        let delete_button = if self.editable() {
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

    /// Truncates a menu label's value to `max_chars`, appending an ellipsis when
    /// it was cut. Used only for menu entry text; filters and the value popover
    /// always carry the untruncated value.
    fn truncate_for_label(value: &str, max_chars: usize) -> String {
        if value.chars().count() <= max_chars {
            value.to_string()
        } else {
            let mut truncated: String = value.chars().take(max_chars).collect();
            truncated.push('…');
            truncated
        }
    }

    /// Whether the cell context menu should offer an "Edit cell" entry for
    /// `column`: only when the caller found an addressable row (`target`) and
    /// the view is editable (not read-only/dirty-SQL/custom-query). For an
    /// existing row the column must also not be part of the primary key
    /// (same rule `begin_edit_cell` enforces, since the PK identifies the
    /// row); a pending insert row has no such restriction, matching
    /// `begin_edit_new_cell`, where every column of a new row - including the
    /// primary key it will be created with - is editable.
    fn shows_edit_cell_entry(&self, column: &str, target: &Option<EditTarget>) -> bool {
        if !self.editable() {
            return false;
        }
        match target {
            Some(EditTarget::Existing(_)) => !self.is_primary_key_column(column),
            Some(EditTarget::New(_)) => true,
            None => false,
        }
    }

    /// Deploys the right-click menu for a data cell at `position`, offering
    /// quick filters on the cell's value, a "View value" popup, and (when
    /// editable) an "Edit cell" entry.
    ///
    /// `column`, `value`, and `target` must be captured by the caller during
    /// render rather than resolved here by index: by the time this runs, a
    /// concurrent page swap could otherwise make it act on the wrong row.
    /// `target` is `Some` only for rows an inline edit can apply to (an
    /// existing row's `RowKey` or a pending insert's `InsertId`); the "Edit
    /// cell" entry is omitted when it is `None`, the view is not editable, or
    /// the column is part of the primary key.
    fn deploy_cell_context_menu(
        &mut self,
        position: Point<Pixels>,
        column: String,
        value: Option<String>,
        target: Option<EditTarget>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let show_edit = self.shows_edit_cell_entry(&column, &target);
        let table_view = cx.weak_entity();
        let context_menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let mut menu = match value.clone() {
                Some(value) => {
                    let filter_label = format!(
                        "Filter: {column} = '{}'",
                        Self::truncate_for_label(&value, 40)
                    );
                    let exclude_label = format!(
                        "Exclude: {column} ≠ '{}'",
                        Self::truncate_for_label(&value, 40)
                    );
                    menu.entry(filter_label, None, {
                        let table_view = table_view.clone();
                        let column = column.clone();
                        let value = value.clone();
                        move |window, cx| {
                            table_view
                                .update(cx, |table, cx| {
                                    table.apply_filter_edit(
                                        None,
                                        Filter {
                                            column: column.clone(),
                                            op: FilterOp::Eq,
                                            value: value.clone(),
                                        },
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    })
                    .entry(exclude_label, None, {
                        let table_view = table_view.clone();
                        let column = column.clone();
                        move |window, cx| {
                            table_view
                                .update(cx, |table, cx| {
                                    table.apply_filter_edit(
                                        None,
                                        Filter {
                                            column: column.clone(),
                                            op: FilterOp::NotEq,
                                            value: value.clone(),
                                        },
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    })
                }
                None => menu
                    .entry(format!("Filter: {column} IS NULL"), None, {
                        let table_view = table_view.clone();
                        let column = column.clone();
                        move |window, cx| {
                            table_view
                                .update(cx, |table, cx| {
                                    table.apply_filter_edit(
                                        None,
                                        Filter {
                                            column: column.clone(),
                                            op: FilterOp::IsNull,
                                            value: String::new(),
                                        },
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    })
                    .entry(format!("Exclude: {column} IS NOT NULL"), None, {
                        let table_view = table_view.clone();
                        let column = column.clone();
                        move |window, cx| {
                            table_view
                                .update(cx, |table, cx| {
                                    table.apply_filter_edit(
                                        None,
                                        Filter {
                                            column: column.clone(),
                                            op: FilterOp::IsNotNull,
                                            value: String::new(),
                                        },
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    }),
            };
            menu = menu.separator().entry("View value", None, {
                let table_view = table_view.clone();
                let popover_value = value.clone().unwrap_or_else(|| "NULL".to_string());
                move |window, cx| {
                    table_view
                        .update(cx, |table, cx| {
                            table.open_value_popover(position, popover_value.clone(), window, cx);
                        })
                        .log_err();
                }
            });
            if show_edit {
                let Some(target) = target.clone() else {
                    return menu;
                };
                menu = menu.entry("Edit cell", None, {
                    let table_view = table_view.clone();
                    move |window, cx| {
                        table_view
                            .update(cx, |table, cx| match &target {
                                EditTarget::Existing(row_key) => {
                                    if let Some(display_row) = table.display_row_for_key(row_key)
                                        && let Some(column_index) = table.column_index_for(&column)
                                    {
                                        table.begin_edit_cell(
                                            display_row,
                                            column_index,
                                            window,
                                            cx,
                                        );
                                    }
                                }
                                EditTarget::New(id) => {
                                    if let Some(insert_index) = table.insert_index_for_id(*id)
                                        && let Some(column_index) = table.column_index_for(&column)
                                    {
                                        table.begin_edit_new_cell(
                                            *id,
                                            insert_index,
                                            column_index,
                                            window,
                                            cx,
                                        );
                                    }
                                }
                            })
                            .log_err();
                    }
                });
            }
            menu
        });
        window.focus(&context_menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    /// The display-row index of the page row identified by `row_key`, if it is
    /// still present on the current page. Used by the "Edit cell" menu action,
    /// which is built once at deploy time but runs later, so the row's position
    /// must be re-resolved rather than trusted from when the menu opened.
    fn display_row_for_key(&self, row_key: &RowKey) -> Option<usize> {
        let page = self.page.as_ref()?;
        (0..page.rows.len()).find(|&display_row| {
            self.row_key_for(display_row)
                .is_some_and(|key| &key == row_key)
        })
    }

    /// The insert-row index of the pending insert identified by `id`, if it is
    /// still present in the edit buffer. See [`Self::display_row_for_key`] for
    /// why this is re-resolved rather than captured.
    fn insert_index_for_id(&self, id: InsertId) -> Option<usize> {
        self.edits
            .inserts
            .iter()
            .position(|(insert_id, _)| *insert_id == id)
    }

    /// The column index of `column` in the current page, if loaded.
    fn column_index_for(&self, column: &str) -> Option<usize> {
        let page = self.page.as_ref()?;
        page.columns.iter().position(|name| name == column)
    }

    /// Opens the "View value" popup at `position`, showing `value` in full
    /// (already resolved to the literal text `"NULL"` by the caller when the
    /// cell was NULL).
    fn open_value_popover(
        &mut self,
        position: Point<Pixels>,
        value: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let popover = cx.new(|cx| ValuePopover {
            value: value.into(),
            focus_handle: cx.focus_handle(),
        });
        window.focus(&popover.focus_handle(cx), cx);
        let subscription = cx.subscribe(&popover, |this, _, _: &DismissEvent, cx| {
            this.value_popover.take();
            cx.notify();
        });
        self.value_popover = Some((popover, position, subscription));
        cx.notify();
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
        // A buffered update is only meaningful for a table-backed query: a
        // custom query's columns may coincidentally line up with the original
        // table's primary-key names, and matching against it would falsely
        // paint values/highlights the custom query never returned (finding 2).
        let is_table_backed = matches!(self.query.base, QueryBase::Table(_));
        let row_key = is_table_backed
            .then(|| self.row_key_for(display_row))
            .flatten();
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

        let editable_here = self.editable()
            && row_key.is_some()
            && column_name
                .as_ref()
                .is_some_and(|column| !self.is_primary_key_column(column));

        // Captured now, at render time, rather than resolved by index inside
        // the context-menu handler: a page swap between paint and right-click
        // must not make the menu act on a different row than the one shown.
        let context_menu_target = row_key.clone().map(EditTarget::Existing);
        let context_menu_column = column_name.clone().unwrap_or_default();
        let context_menu_value = display.clone();

        let right_align = column_name
            .as_ref()
            .is_some_and(|column| self.numeric_columns.contains(column))
            && matches!(self.query.base, QueryBase::Table(_));

        let mut cell = div()
            .w_full()
            .font_buffer(cx)
            .when(right_align, |this| this.text_right());
        if modified {
            cell = cell.bg(modified_cell_background(cx)).rounded_sm().px_1();
        }
        let cell = match display {
            Some(value) => cell.whitespace_nowrap().text_ellipsis().child(value),
            None => cell.child(Label::new("NULL").color(Color::Muted).italic()),
        };

        let cell = div()
            .id(ElementId::NamedInteger(
                SharedString::from(format!("db-cell-{column_index}")),
                display_row as u64,
            ))
            .w_full()
            .when(editable_here, |this| this.cursor_pointer())
            .child(cell)
            .when(editable_here, |this| {
                this.on_click(
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
            })
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    this.deploy_cell_context_menu(
                        event.position,
                        context_menu_column.clone(),
                        context_menu_value.clone(),
                        context_menu_target.clone(),
                        window,
                        cx,
                    );
                }),
            );

        cell.into_any_element()
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

        let right_align = column_name
            .as_ref()
            .is_some_and(|column| self.numeric_columns.contains(column))
            && matches!(self.query.base, QueryBase::Table(_));

        let cell = div()
            .w_full()
            .font_buffer(cx)
            .when(right_align, |this| this.text_right());
        let cell = match buffered {
            Some(EditCell::Value(value)) => cell
                .whitespace_nowrap()
                .text_ellipsis()
                .child(value.clone()),
            Some(EditCell::Null) => cell.child(Label::new("NULL").color(Color::Muted).italic()),
            None => cell.child(Label::new("default").color(Color::Muted).italic()),
        };

        // Every column of an insert row is editable (including the primary
        // key, unlike an existing row), so the menu's "Edit cell" entry is
        // gated only on `editable()`.
        let editable_here = self.editable();
        let context_menu_target = Some(EditTarget::New(insert_id));
        let context_menu_column = column_name.clone().unwrap_or_default();
        let context_menu_value = match buffered {
            Some(EditCell::Value(value)) => Some(value.clone()),
            Some(EditCell::Null) | None => None,
        };

        let cell = div()
            .id(ElementId::NamedInteger(
                SharedString::from(format!("db-insert-cell-{column_index}")),
                insert_index as u64,
            ))
            .w_full()
            .when(editable_here, |this| this.cursor_pointer())
            .child(cell)
            .when(editable_here, |this| {
                this.on_click(
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
            })
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    this.deploy_cell_context_menu(
                        event.position,
                        context_menu_column.clone(),
                        context_menu_value.clone(),
                        context_menu_target.clone(),
                        window,
                        cx,
                    );
                }),
            );

        cell.into_any_element()
    }

    fn render_header(&self, index: usize, column: &str, cx: &Context<Self>) -> AnyElement {
        let sorted = self
            .query
            .sort
            .as_ref()
            .filter(|sort| sort.column == column)
            .map(|sort| sort.direction);
        let sort_tooltip = match sorted {
            Some(SortDirection::Asc) => "Sorted ascending. Click to sort descending",
            Some(SortDirection::Desc) => "Sorted descending. Click to clear sorting",
            None => "Not sorted. Click to sort ascending",
        };
        let existing_filter_index = self
            .query
            .filters
            .iter()
            .position(|filter| filter.column == column);
        let has_filter = existing_filter_index.is_some();
        let column = column.to_string();
        let group = SharedString::from(format!("db-header-{index}"));

        let sort_label = {
            let column = column.clone();
            div()
                .id(ElementId::NamedInteger(
                    "db-sort-label".into(),
                    index as u64,
                ))
                .flex_1()
                .min_w_0()
                .cursor_pointer()
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .whitespace_nowrap()
                                .text_ellipsis()
                                .child(Label::new(column.clone())),
                        )
                        .when_some(sorted, |this, direction| {
                            this.child(Icon::new(match direction {
                                SortDirection::Asc => IconName::ArrowUp,
                                SortDirection::Desc => IconName::ArrowDown,
                            }))
                        }),
                )
                .tooltip(Tooltip::text(sort_tooltip))
                .on_click(cx.listener(move |this, _event, window, cx| {
                    this.toggle_sort(&column, window, cx);
                }))
        };

        let funnel = PopoverMenu::new(("db-col-filter", index))
            .trigger(
                IconButton::new(("db-col-filter-icon", index), IconName::Filter)
                    .icon_size(IconSize::XSmall)
                    .tooltip(Tooltip::text("Filter this column"))
                    .toggle_state(has_filter)
                    .when(!has_filter, |this| this.visible_on_hover(group.clone())),
            )
            .anchor(Anchor::TopLeft)
            .menu({
                let table_view = cx.weak_entity();
                let existing = existing_filter_index.and_then(|filter_index| {
                    self.query
                        .filters
                        .get(filter_index)
                        .cloned()
                        .map(|filter| (filter_index, filter))
                });
                move |window, cx| {
                    let existing = existing
                        .as_ref()
                        .map(|(filter_index, filter)| (*filter_index, filter));
                    Some(cx.new(|cx| {
                        FilterPopover::new(column.clone(), existing, table_view.clone(), window, cx)
                    }))
                }
            });

        h_flex()
            .group(group)
            .justify_between()
            .items_center()
            .w_full()
            .child(sort_label)
            .child(funnel)
            .into_any_element()
    }

    fn render_structure(&self, cx: &Context<Self>) -> AnyElement {
        let Some(structure) = self.structure.as_ref() else {
            return v_flex()
                .p_4()
                .child(Label::new("Loading structure…").color(Color::Muted))
                .into_any_element();
        };

        let mut table = Table::new(6)
            .striped()
            .header_background(cx.theme().colors().title_bar_background)
            .header(vec![
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

    /// Renders the chips row under the SQL bar: one chip per active filter,
    /// clicking it reopens a prefilled [`FilterPopover`] to edit it, plus a
    /// sort chip when a sort is active. Returns `None` when there is nothing
    /// to show, so the caller can skip rendering the row entirely.
    fn render_chips_row(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if self.query.filters.is_empty() && self.query.sort.is_none() {
            return None;
        }

        let filter_chips = self
            .query
            .filters
            .iter()
            .enumerate()
            .map(|(index, filter)| self.render_filter_chip(index, filter, cx));
        let sort_chip = self
            .query
            .sort
            .as_ref()
            .map(|sort| self.render_sort_chip(sort, cx));

        Some(
            h_flex()
                .w_full()
                .px_2()
                .py_1()
                .gap_1()
                .flex_wrap()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .children(filter_chips)
                .children(sort_chip)
                .into_any_element(),
        )
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
        let column = filter.column.clone();
        let filter = filter.clone();

        h_flex()
            .gap_1()
            .px_1p5()
            .py_0p5()
            .rounded_sm()
            .bg(cx.theme().colors().element_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(
                PopoverMenu::new(("db-filter-chip", index))
                    .trigger(
                        Button::new(("db-filter-chip-trigger", index), text)
                            .size(ButtonSize::Compact)
                            .style(ButtonStyle::Transparent),
                    )
                    .anchor(Anchor::TopLeft)
                    .menu({
                        let table_view = cx.weak_entity();
                        move |window, cx| {
                            Some(cx.new(|cx| {
                                FilterPopover::new(
                                    column.clone(),
                                    Some((index, &filter)),
                                    table_view.clone(),
                                    window,
                                    cx,
                                )
                            }))
                        }
                    }),
            )
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

    /// Renders the sort chip (`"{column} asc|desc"`); its `×` clears the sort
    /// directly, mirroring how a filter chip's `×` calls `remove_filter`.
    fn render_sort_chip(&self, sort: &Sort, cx: &Context<Self>) -> AnyElement {
        let direction = match sort.direction {
            SortDirection::Asc => "asc",
            SortDirection::Desc => "desc",
        };
        let text = format!("{} {direction}", sort.column);

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
                IconButton::new("db-sort-remove", IconName::Close)
                    .icon_size(IconSize::XSmall)
                    .tooltip(Tooltip::text("Clear sort"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        // While dirty, run the bar's text first (finding 1) and
                        // only clear the sort that results from it; a refused
                        // run (finding 2) leaves the sort untouched.
                        if !this.commit_dirty_bar(window, cx) {
                            return;
                        }
                        this.query.sort = None;
                        this.query.offset = 0;
                        this.finish_editing(cx);
                        this.restart_query(window, cx);
                    })),
            )
            .into_any_element()
    }

    /// The footer shown under the data grid: left-to-right, page navigation,
    /// the row-range counter, the page-size picker, edit controls (`+ Row` and,
    /// while dirty, the change count with Save/Discard), then right-aligned the
    /// last query's row count and timing plus a Refresh button. Combines what
    /// used to be a separate pagination footer and edit toolbar into one row
    /// (see the stage-3 table-page-redesign spec).
    fn render_footer(&self, cx: &Context<Self>) -> AnyElement {
        let (row_count, has_more) = match &self.page {
            Some(page) => (page.rows.len(), page.has_more),
            None => (0, false),
        };
        let summary = footer_counter(self.query.offset, row_count, has_more, self.query.limit);
        let at_start = self.query.offset == 0;

        let pending = self.pending_change_count();
        let saving = self.save_state == SaveState::Saving;
        // A save already in flight keeps showing its progress/outcome even if
        // the SQL bar is dirtied meanwhile (it cannot be *started* while
        // `!editable()` - see `save_edits`'s own gate - but one already
        // running should not vanish from the UI mid-flight). Otherwise
        // Save/Discard require `editable()`, not just a non-empty buffer: a
        // buffer left over from a mode the view has since left (finding 2)
        // must not offer to apply against whatever the grid shows now.
        let dirty = (pending > 0 && self.editable()) || saving;
        let show_edit_controls = self.editable();

        let add_row_button = show_edit_controls.then(|| {
            Button::new("db-add-row", "+ Row")
                .size(ButtonSize::Compact)
                .style(ButtonStyle::Subtle)
                .disabled(saving)
                .tooltip(Tooltip::text("Add a new row"))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.add_row(cx);
                }))
        });

        let change_controls = dirty.then(|| {
            let change_label = if pending == 1 {
                "1 change".to_string()
            } else {
                format!("{pending} changes")
            };
            h_flex()
                .gap_1()
                .items_center()
                .child(
                    Label::new(change_label)
                        .color(Color::Default)
                        .size(LabelSize::Small),
                )
                .child(
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
        });

        let save_result = footer_save_result(&self.save_state, pending)
            .map(|(text, color)| Label::new(text).color(color).size(LabelSize::Small));

        let timing = self
            .last_run
            .map(|(count, elapsed)| format!("{count} rows · {} ms", elapsed.as_millis()));

        let mut footer = h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_2()
            .items_center()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border);
        if dirty {
            footer = footer.bg(modified_cell_background(cx));
        }

        footer
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                IconButton::new("db-prev-page", IconName::ChevronLeft)
                                    .icon_size(IconSize::Small)
                                    .disabled(at_start)
                                    .tooltip(Tooltip::text("Previous page"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.prev_page(window, cx)
                                    })),
                            )
                            .child(
                                IconButton::new("db-next-page", IconName::ChevronRight)
                                    .icon_size(IconSize::Small)
                                    .disabled(!has_more)
                                    .tooltip(Tooltip::text("Next page"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.next_page(window, cx)
                                    })),
                            ),
                    )
                    .child(
                        Label::new(summary)
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .when_some(self.query.limit, |this, limit| {
                        this.child(self.render_page_size_picker(limit, cx))
                    })
                    .children(add_row_button)
                    .children(change_controls)
                    .children(save_result)
                    .when_some(self.read_only_reason(), |this, reason| {
                        this.child(
                            Label::new(reason)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .when_some(timing, |this, text| {
                        this.child(Label::new(text).color(Color::Muted).size(LabelSize::Small))
                    })
                    .child(
                        IconButton::new("db-refresh", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Refresh"))
                            .on_click(cx.listener(|this, _, window, cx| this.refresh(window, cx))),
                    ),
            )
            .into_any_element()
    }

    /// The page-size picker shown in the footer: a `"{limit} / page"` trigger
    /// that opens a menu of page-size choices. The option list is always
    /// `100`/`500`/`1000` plus the settings default when it is not already one
    /// of those three, so the configured default is always reachable even if
    /// it is unusual (e.g. `250`). The default is clamped to
    /// `UI_MAX_QUERY_ROWS` (finding 0) before being considered, so a setting
    /// above that ceiling never adds an unreachable option: `run_query` can
    /// never return more rows than that, so a larger `LIMIT` could never be
    /// satisfied.
    fn render_page_size_picker(&self, limit: usize, cx: &Context<Self>) -> AnyElement {
        let default = configured_page_size(cx);
        let mut sizes = vec![100, 500, 1000];
        if !sizes.contains(&default) {
            sizes.push(default);
            sizes.sort_unstable();
        }

        PopoverMenu::new("db-page-size")
            .trigger(
                Button::new("db-page-size-trigger", format!("{limit} / page"))
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Subtle),
            )
            .anchor(Anchor::TopLeft)
            .menu({
                let table_view = cx.weak_entity();
                move |window, cx| {
                    let table_view = table_view.clone();
                    let sizes = sizes.clone();
                    Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                        for size in &sizes {
                            let size = *size;
                            let table_view = table_view.clone();
                            menu = menu.entry(format!("{size} / page"), None, move |window, cx| {
                                table_view
                                    .update(cx, |table, cx| {
                                        table.set_page_size(size, window, cx);
                                    })
                                    .log_err();
                            });
                        }
                        menu
                    }))
                }
            })
            .into_any_element()
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

    /// The visible, editable SQL bar shown under the header: a collapse
    /// chevron, the SQL editor, and a Run button; in custom-query mode, also a
    /// read-only badge and a button back to the plain table query.
    fn render_sql_bar(&self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let chevron = IconButton::new(
            "db-sql-bar-collapse",
            if self.sql_bar_collapsed {
                IconName::ChevronRight
            } else {
                IconName::ChevronDown
            },
        )
        .icon_size(IconSize::Small)
        .tooltip(Tooltip::text(if self.sql_bar_collapsed {
            "Expand SQL"
        } else {
            "Collapse SQL"
        }))
        .on_click(cx.listener(|this, _, _, cx| {
            this.sql_bar_collapsed = !this.sql_bar_collapsed;
            cx.notify();
        }));

        if self.sql_bar_collapsed {
            return h_flex()
                .w_full()
                .px_2()
                .py_1()
                .gap_1()
                .items_center()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .child(chevron)
                .child(Label::new("SQL").size(LabelSize::Small).color(Color::Muted))
                .into_any_element();
        }

        let run_button = Button::new("db-sql-run", "Run")
            .size(ButtonSize::Compact)
            .style(ButtonStyle::Filled)
            .tooltip(move |_window, cx| Tooltip::for_action("Run Query", &RunQuery, cx))
            .on_click(cx.listener(|this, _, window, cx| {
                this.run_from_editor(window, cx);
            }));

        let mut bar = v_flex()
            .key_context("SqlQueryEditor")
            .on_action(cx.listener(Self::handle_run_query))
            .w_full()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .gap_2()
                    .items_start()
                    .child(chevron)
                    .child(div().flex_1().child(self.sql_editor.clone()))
                    .child(run_button),
            );

        if self.query.is_custom() {
            bar = bar.child(
                h_flex()
                    .w_full()
                    .px_2()
                    .pb_1()
                    .gap_2()
                    .items_center()
                    .child(
                        Label::new("Custom query · read-only")
                            .size(LabelSize::Small)
                            .color(Color::Warning),
                    )
                    .child(
                        Button::new("db-reset-query", "Reset to table query")
                            .size(ButtonSize::Compact)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.reset_to_table_query(window, cx);
                            })),
                    ),
            );
        }

        if let Some(notice) = self.pending_edits_notice.as_ref() {
            bar = bar.child(
                h_flex().w_full().px_2().pb_1().child(
                    Label::new(notice.clone())
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                ),
            );
        }

        bar.into_any_element()
    }
}

/// A small popover for adding or editing a single [`Filter`] on one column,
/// anchored to a header funnel icon or a filter chip. Delegates focus to its
/// value input (see `Focusable` impl below) so `PopoverMenu` can focus it
/// automatically without a manual focus call.
pub struct FilterPopover {
    column: String,
    op: FilterOp,
    value_field: Entity<InputField>,
    existing_index: Option<usize>,
    table_view: WeakEntity<TableDataView>,
}

impl FilterPopover {
    fn new(
        column: String,
        existing: Option<(usize, &Filter)>,
        table_view: WeakEntity<TableDataView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let existing_filter = existing.map(|(_, filter)| filter);
        let value_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "Value");
            if let Some(filter) = existing_filter {
                field.set_text(&filter.value, window, cx);
            }
            field
        });
        Self {
            column,
            op: existing_filter.map_or(FilterOp::Eq, |filter| filter.op),
            value_field,
            existing_index: existing.map(|(index, _)| index),
            table_view,
        }
    }

    /// Commits the popover's current operator/value as a [`Filter`] on
    /// `self.column`, via [`TableDataView::apply_filter_edit`], then closes
    /// the popover.
    fn apply(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let value = if matches!(self.op, FilterOp::IsNull | FilterOp::IsNotNull) {
            String::new()
        } else {
            self.value_field.read(cx).text(cx)
        };
        let filter = Filter {
            column: self.column.clone(),
            op: self.op,
            value,
        };
        let existing_index = self.existing_index;
        // `update_in` re-enters `App::with_window` for the target entity's
        // window; since this popover's own window update is already on the
        // stack for the *same* window, that re-entry fails ("entity has no
        // current window"). Reuse the `Window` already borrowed by this
        // callback instead of asking GPUI to look it up again.
        self.table_view
            .update(cx, |table, cx| {
                table.apply_filter_edit(existing_index, filter, window, cx);
            })
            .log_err();
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for FilterPopover {}

impl Focusable for FilterPopover {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.value_field.focus_handle(cx)
    }
}

impl Render for FilterPopover {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let show_value = !matches!(self.op, FilterOp::IsNull | FilterOp::IsNotNull);

        v_flex()
            .key_context("DatabaseFilterPopover")
            .occlude()
            .elevation_2(cx)
            .w_72()
            .p_2()
            .gap_2()
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| this.apply(window, cx)))
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| cx.emit(DismissEvent)))
            .child(Label::new(self.column.clone()).size(LabelSize::Small))
            .child(
                h_flex()
                    .gap_1()
                    .flex_wrap()
                    .children(all_filter_ops().map(|op| {
                        Button::new(("filter-op", op as usize), filter_op_label(op))
                            .size(ButtonSize::Compact)
                            .toggle_state(self.op == op)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.op = op;
                                cx.notify();
                            }))
                    })),
            )
            .when(show_value, |this| this.child(self.value_field.clone()))
            .child(
                h_flex().justify_end().child(
                    Button::new("filter-apply", "Apply")
                        .style(ButtonStyle::Filled)
                        .size(ButtonSize::Compact)
                        .on_click(cx.listener(|this, _, window, cx| this.apply(window, cx))),
                ),
            )
    }
}

/// A read-only popup showing a data cell's full value, opened from the cell
/// context menu's "View value" entry. Values in the grid are truncated for
/// display and menu labels are truncated further; this is the one place the
/// whole value is shown.
pub struct ValuePopover {
    value: SharedString,
    focus_handle: FocusHandle,
}

impl EventEmitter<DismissEvent> for ValuePopover {}

impl Focusable for ValuePopover {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ValuePopover {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DatabaseValuePopover")
            .track_focus(&self.focus_handle)
            .occlude()
            .elevation_2(cx)
            .max_w_96()
            .max_h_80()
            .p_2()
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| cx.emit(DismissEvent)))
            .child(
                div()
                    .id("db-value-scroll")
                    .overflow_y_scroll()
                    .max_h_72()
                    .child(Label::new(self.value.clone()).size(LabelSize::Small)),
            )
    }
}

impl Render for TableDataView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = match (&self.load_state, self.mode) {
            (LoadState::Error(message), _) => self.render_error(&message.clone(), cx),
            (_, ViewMode::Structure) => self.render_structure(cx),
            (_, ViewMode::Data) => self.render_data(cx),
        };
        let in_data =
            self.mode == ViewMode::Data && !matches!(self.load_state, LoadState::Error(_));
        let sql_bar = self.render_sql_bar(window, cx);
        let chips_row = in_data.then(|| self.render_chips_row(cx)).flatten();

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
            .child(sql_bar)
            .children(chips_row)
            .child(v_flex().flex_1().size_full().overflow_hidden().child(body))
            .when(in_data, |this| this.child(self.render_footer(cx)))
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Anchor::TopLeft)
                        .snap_to_window_with_margin(px(8.))
                        .child(menu.clone()),
                )
                .with_priority(3)
            }))
            .children(self.value_popover.as_ref().map(|(popover, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Anchor::TopLeft)
                        .snap_to_window_with_margin(px(8.))
                        .child(popover.clone()),
                )
                .with_priority(3)
            }))
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
    language_registry: Option<Arc<LanguageRegistry>>,
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
                TableDataView::new(
                    client,
                    connection,
                    table,
                    is_view,
                    language_registry,
                    window,
                    cx,
                )
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
    use gpui::{
        AppContext as _, DismissEvent, Entity, Focusable, TestAppContext, UpdateGlobal as _,
        VisualTestContext,
    };

    use super::{
        EditTarget, FilterPopover, LoadState, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH, SaveState,
        TableDataView, ViewMode, all_filter_ops, column_width_for_chars, compute_editable,
        filter_op_label, footer_counter, footer_save_result, numeric_column_names,
    };
    use crate::query_state::{QueryBase, render_sql};
    use ui::Color;

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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
    async fn next_page_on_unpaginated_truncated_custom_continues_after_shown_rows(
        cx: &mut TestAppContext,
    ) {
        // Finding 11: a fresh custom query (limit == None) that overran
        // UI_MAX_QUERY_ROWS has already shown that many rows with no OFFSET.
        // The first explicit page must continue right after what is already
        // on screen, not jump back to `page_size` rows in - which would just
        // re-show rows 0..page_size that were already displayed.
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = QueryResult {
            columns: vec!["id".into()],
            rows: (0..super::UI_MAX_QUERY_ROWS)
                .map(|i| vec![Some(i.to_string())])
                .collect(),
            truncated: true,
            command_tag: None,
        };
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| {
            view.sql_editor.update(cx, |editor, cx| {
                editor.set_text("SELECT id FROM t", window, cx)
            });
            view.run_from_editor(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().is_custom() && view.load_state() == &LoadState::Idle
            })
        })
        .await;
        view.read_with(cx, |view, _| {
            assert_eq!(view.query().limit, None, "sanity: still unpaginated");
            assert_eq!(view.query().offset, 0);
        });

        view.update_in(cx, |view, window, cx| view.next_page(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(
                view.query().offset,
                super::UI_MAX_QUERY_ROWS,
                "must continue after the rows already shown, not jump to page_size"
            );
        });
    }

    #[gpui::test]
    async fn set_page_size_resets_offset_and_reruns_sql(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = rows_result(100);
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Advance to offset 200 first so the reset to 0 is observable.
        view.update_in(cx, |view, window, cx| view.next_page(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().offset == 100 && view.load_state() == &LoadState::Idle
            })
        })
        .await;
        view.update_in(cx, |view, window, cx| view.next_page(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().offset == 200 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| view.set_page_size(500, window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().limit == Some(500) && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.query().limit, Some(500));
            assert_eq!(view.query().offset, 0);
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.ends_with(r#"SELECT * FROM "public"."users" LIMIT 500 OFFSET 0;"#),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn set_page_size_commits_open_editor(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
        });

        view.update_in(cx, |view, window, cx| view.set_page_size(500, window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(
                view.editing_cell().is_none(),
                "changing the page size must close the inline editor"
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
                .expect("the in-progress edit should be committed keyed by RowKey");
            assert_eq!(cell, &database_client::EditCell::Value("Alicia".into()));
        });
    }

    /// Sets `DatabaseSettings.page_size` in the test settings store.
    fn set_page_size_setting(cx: &mut TestAppContext, page_size: u32) {
        cx.update(|cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.database.get_or_insert_default().page_size = Some(page_size);
                });
            });
        });
    }

    #[gpui::test]
    async fn page_size_setting_above_ceiling_is_clamped_in_generated_sql(cx: &mut TestAppContext) {
        // A `page_size` configured above UI_MAX_QUERY_ROWS must not produce a
        // LIMIT the query can never actually satisfy (finding 0): run_query
        // never returns more than UI_MAX_QUERY_ROWS rows, so any larger limit
        // just leaves the extra rows permanently unreachable by pagination.
        init_test(cx);
        set_page_size_setting(cx, 5000);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.read_with(cx, |view, _| {
            assert_eq!(
                view.query().limit,
                Some(super::UI_MAX_QUERY_ROWS),
                "limit must be clamped to UI_MAX_QUERY_ROWS"
            );
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.contains("LIMIT 1000 OFFSET 0"),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn configured_page_size_clamps_to_ceiling(cx: &mut TestAppContext) {
        // `render_page_size_picker` builds its option list from
        // `configured_page_size`, so clamping there is what keeps the picker
        // from ever offering an unreachable choice above UI_MAX_QUERY_ROWS
        // (finding 0).
        init_test(cx);
        set_page_size_setting(cx, 5000);
        cx.update(|cx| {
            assert_eq!(super::configured_page_size(cx), super::UI_MAX_QUERY_ROWS);
        });
    }

    #[gpui::test]
    async fn set_page_size_clamps_above_ceiling(cx: &mut TestAppContext) {
        // Even a direct `set_page_size` call above UI_MAX_QUERY_ROWS (e.g. a
        // stale picker entry) must clamp defensively (finding 0).
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| view.set_page_size(5000, window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.query().limit, Some(super::UI_MAX_QUERY_ROWS));
        });
    }

    #[test]
    fn footer_counter_formats_row_range() {
        assert_eq!(footer_counter(0, 100, true, Some(100)), "rows 1–100+");
        assert_eq!(footer_counter(0, 100, false, Some(100)), "rows 1–100");
        assert_eq!(footer_counter(200, 50, false, Some(100)), "rows 201–250");
    }

    #[test]
    fn footer_counter_empty_page_is_no_rows() {
        assert_eq!(footer_counter(0, 0, false, Some(100)), "No rows");
        assert_eq!(footer_counter(100, 0, false, None), "No rows");
    }

    #[test]
    fn footer_counter_custom_query_without_limit() {
        assert_eq!(footer_counter(0, 42, false, None), "42 rows");
        assert_eq!(footer_counter(0, 1, false, None), "1 rows");
    }

    #[test]
    fn footer_counter_custom_query_truncated_shows_plus() {
        // A fresh custom query with no `limit` that was truncated at
        // UI_MAX_QUERY_ROWS must not present its row count as an exact total
        // (finding 5): there may be more matching rows never fetched.
        assert_eq!(footer_counter(0, 1000, true, None), "1000+ rows");
    }

    #[test]
    fn footer_save_result_shows_error_regardless_of_pending() {
        // A failed save keeps the buffer intact (see `save_edits`), so the
        // error must render even while `change_controls` is also showing the
        // still-pending count (finding 4).
        let state = SaveState::Error("permission denied".to_string());
        assert_eq!(
            footer_save_result(&state, 1),
            Some(("permission denied".to_string(), Color::Error))
        );
        assert_eq!(
            footer_save_result(&state, 0),
            Some(("permission denied".to_string(), Color::Error))
        );
    }

    #[test]
    fn footer_save_result_shows_done_only_once_buffer_is_clear() {
        // A success message must appear once the buffer that produced it has
        // cleared (pending == 0, finding 4)...
        let state = SaveState::Done("Saved: 1 updated, 0 inserted, 0 deleted".to_string());
        assert_eq!(
            footer_save_result(&state, 0),
            Some((
                "Saved: 1 updated, 0 inserted, 0 deleted".to_string(),
                Color::Success
            ))
        );
        // ...but not once a fresh, unrelated edit has made the buffer dirty
        // again, which would misattribute the new edit as already saved.
        assert_eq!(footer_save_result(&state, 1), None);
    }

    #[test]
    fn footer_save_result_hides_idle_and_saving() {
        assert_eq!(footer_save_result(&SaveState::Idle, 0), None);
        assert_eq!(footer_save_result(&SaveState::Saving, 0), None);
    }

    #[gpui::test]
    async fn structure_mode_fetches_structure_once(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
    async fn reload_structure_does_not_clobber_data_load_error(cx: &mut TestAppContext) {
        // Finding 6: `load_state` belongs to the data load alone. A failing
        // data query must leave its error banner up even after a concurrent
        // (or later-completing) structure reload succeeds - the old shared
        // field let a late `reload_structure` success overwrite the data
        // error with `Idle`, silently swapping the error banner for a stale
        // grid.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Visit Structure once so `refresh` also reloads it (see `refresh`'s
        // `structure.is_some()` gate).
        view.update(cx, |view, cx| view.toggle_structure(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;
        view.update(cx, |view, cx| view.toggle_structure(cx));
        cx.run_until_parked();

        // Now make only the data query fail; the structure fetch keeps
        // succeeding.
        fake.set_run_query_error(Some("syntax error".into()));
        view.update_in(cx, |view, window, cx| view.refresh(window, cx));

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                matches!(view.load_state(), LoadState::Error(_))
            })
        })
        .await;
        // Give the (successful) structure reload every chance to complete and
        // settle after the data error already landed.
        cx.run_until_parked();

        view.read_with(cx, |view, _| {
            assert!(
                matches!(view.load_state(), LoadState::Error(_)),
                "a later-completing structure reload must not clear the data error, got {:?}",
                view.load_state()
            );
            assert!(
                view.structure().is_some(),
                "the structure reload itself should still have succeeded"
            );
        });
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
                None,
                window,
                cx,
            )
        });
        let prod = cx.update(|window, cx| {
            TableDataView::new(client, "prod".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
    async fn apply_filter_edit_none_adds_resets_offset_and_reloads(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = rows_result(100);
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            view.apply_filter_edit(
                None,
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
    async fn apply_filter_edit_some_replaces_not_appends(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
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

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                Some(0),
                Filter {
                    column: "name".into(),
                    op: FilterOp::NotEq,
                    value: "Bob".into(),
                },
                window,
                cx,
            )
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(
                view.query().filters.len(),
                1,
                "replacing an existing filter should not append"
            );
            assert_eq!(view.query().filters[0].op, FilterOp::NotEq);
            assert_eq!(view.query().filters[0].value, "Bob");
        });

        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.ends_with(
                r#"SELECT * FROM "public"."users" WHERE "name" <> 'Bob' LIMIT 100 OFFSET 0;"#
            ),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn apply_filter_edit_out_of_bounds_index_is_noop(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                Some(3),
                Filter {
                    column: "name".into(),
                    op: FilterOp::Eq,
                    value: "Alice".into(),
                },
                window,
                cx,
            )
        });
        cx.run_until_parked();

        view.read_with(cx, |view, _| {
            assert!(
                view.query().filters.is_empty(),
                "an out-of-bounds replace index should not add a filter"
            );
        });
    }

    #[gpui::test]
    async fn apply_filter_edit_is_null_needs_no_value(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
                Filter {
                    column: "name".into(),
                    op: FilterOp::IsNull,
                    value: String::new(),
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

        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.ends_with(
                r#"SELECT * FROM "public"."users" WHERE "name" IS NULL LIMIT 100 OFFSET 0;"#
            ),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn remove_filter_via_chip_close_reloads_without_filters(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
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
    async fn clearing_sort_chip_removes_order_by(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| view.toggle_sort("name", window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().sort.is_some() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        // The sort chip's `×` clears the sort directly rather than cycling it,
        // mirroring how a filter chip's `×` calls `remove_filter` directly.
        view.update_in(cx, |view, window, cx| {
            view.query.sort = None;
            view.query.offset = 0;
            view.finish_editing(cx);
            view.restart_query(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().sort.is_none() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            !last.contains("ORDER BY"),
            "clearing the sort chip should drop ORDER BY: {last}"
        );
    }

    #[gpui::test]
    async fn filter_popover_apply_without_existing_index_adds_filter(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        let weak_view = view.downgrade();
        let popover = cx.update(|window, cx| {
            cx.new(|cx| FilterPopover::new("name".to_string(), None, weak_view.clone(), window, cx))
        });
        popover.update_in(cx, |popover, window, cx| {
            popover.op = FilterOp::Eq;
            popover
                .value_field
                .update(cx, |field, cx| field.set_text("Alice", window, cx));
            popover.apply(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().filters.len() == 1 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.query().filters[0].column, "name");
            assert_eq!(view.query().filters[0].op, FilterOp::Eq);
            assert_eq!(view.query().filters[0].value, "Alice");
        });
    }

    #[gpui::test]
    async fn filter_popover_apply_with_existing_index_replaces_filter(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
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

        let existing = view.read_with(cx, |view, _| view.query().filters[0].clone());
        let weak_view = view.downgrade();
        let popover = cx.update(|window, cx| {
            cx.new(|cx| {
                FilterPopover::new(
                    "name".to_string(),
                    Some((0, &existing)),
                    weak_view.clone(),
                    window,
                    cx,
                )
            })
        });
        popover.update_in(cx, |popover, window, cx| {
            popover.op = FilterOp::NotEq;
            popover
                .value_field
                .update(cx, |field, cx| field.set_text("Bob", window, cx));
            popover.apply(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(
                view.query().filters.len(),
                1,
                "applying with an existing index should replace, not append"
            );
            assert_eq!(view.query().filters[0].op, FilterOp::NotEq);
            assert_eq!(view.query().filters[0].value, "Bob");
        });
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

    fn col_with_udt(name: &str, udt_name: &str) -> ColumnInfo {
        ColumnInfo {
            name: name.into(),
            data_type: udt_name.into(),
            udt_name: udt_name.into(),
            udt_schema: "pg_catalog".into(),
            is_nullable: true,
            default: None,
            is_primary_key: false,
        }
    }

    #[gpui::test]
    fn numeric_column_names_covers_numeric_udts(_cx: &mut TestAppContext) {
        let columns = vec![
            col_with_udt("id", "int4"),
            col_with_udt("balance", "numeric"),
            col_with_udt("name", "text"),
            col_with_udt("tags", "varchar"),
        ];
        let numeric = numeric_column_names(&columns);
        assert!(numeric.contains("id"));
        assert!(numeric.contains("balance"));
        assert!(!numeric.contains("name"));
        assert!(!numeric.contains("tags"));
        assert_eq!(numeric.len(), 2);
    }

    #[gpui::test]
    fn numeric_column_names_empty_for_no_columns(_cx: &mut TestAppContext) {
        assert!(numeric_column_names(&[]).is_empty());
    }

    #[gpui::test]
    fn column_width_for_chars_typical_case(_cx: &mut TestAppContext) {
        // advance=8px, 10 chars -> 80 + 8 + 12 = 100, within the clamp range.
        assert_eq!(column_width_for_chars(8., 10), 100.);
    }

    #[gpui::test]
    fn column_width_for_chars_clamps_to_minimum(_cx: &mut TestAppContext) {
        // A short header (e.g. "id", 2 chars) should still clamp up to 60.
        assert_eq!(column_width_for_chars(8., 2), MIN_COLUMN_WIDTH);
        assert_eq!(column_width_for_chars(8., 0), MIN_COLUMN_WIDTH);
    }

    #[gpui::test]
    fn column_width_for_chars_clamps_to_maximum(_cx: &mut TestAppContext) {
        // A very long value should clamp down to 480 rather than blowing out
        // the table width.
        assert_eq!(column_width_for_chars(8., 1000), MAX_COLUMN_WIDTH);
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), true, None, window, cx)
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
    async fn read_only_reason_none_before_structure_loads(cx: &mut TestAppContext) {
        // Before the structure loads, `editable` is `false` (see
        // `compute_editable`'s default), but that is not yet known to be "no
        // primary key" - it could still turn out to have one. The footer must
        // stay silent rather than falsely claim read-only.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });

        view.read_with(cx, |view, _| {
            assert!(
                view.structure().is_none(),
                "structure should not have loaded synchronously"
            );
            assert_eq!(
                view.read_only_reason(),
                None,
                "no reason should be shown before the structure is known"
            );
        });
    }

    #[gpui::test]
    async fn read_only_reason_for_view(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), true, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.read_only_reason(), Some("Read-only: view"));
        });
    }

    #[gpui::test]
    async fn read_only_reason_for_missing_primary_key(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.structure.columns = vec![col("name", false), col("email", false)];
        let client: Arc<dyn DatabaseClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.read_only_reason(), Some("Read-only: no primary key"));
        });
    }

    #[gpui::test]
    async fn read_only_reason_none_for_editable_table(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.editable(), "sanity: PK base table should be editable");
            assert_eq!(view.read_only_reason(), None);
        });
    }

    #[gpui::test]
    async fn read_only_reason_none_in_custom_query_mode(cx: &mut TestAppContext) {
        // Custom-query mode already surfaces its own "Custom query · read-only"
        // badge in the SQL bar; the footer label must not duplicate it.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
        });
        view.read_with(cx, |view, _| {
            assert_eq!(
                view.read_only_reason(),
                None,
                "a dirty SQL bar defers to its own affordance, not the footer label"
            );
        });

        view.update_in(cx, |view, window, cx| view.run_from_editor(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.query().is_custom());
            assert_eq!(
                view.read_only_reason(),
                None,
                "custom-query mode already has its own read-only badge"
            );
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
    async fn column_widths_created_on_page_load(cx: &mut TestAppContext) {
        // After the first page loads, `column_widths` should exist with one
        // width per column (measured from header/values, not necessarily the
        // flat default) and no panics should occur while measuring or
        // rendering with a numeric column and a long text value present.
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = QueryResult {
            columns: vec!["id".into(), "name".into()],
            rows: vec![
                vec![
                    Some("1".into()),
                    Some("a very long value that should widen this column considerably".into()),
                ],
                vec![Some("2".into()), None],
            ],
            truncated: false,
            command_tag: Some("SELECT 2".into()),
        };
        let client: Arc<dyn DatabaseClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update(cx, |view, cx| {
            let widths = view
                .column_widths
                .as_ref()
                .expect("column_widths should be created after the first page loads");
            assert_eq!(widths.read(cx).cols(), 2, "one width per data column");
            // `id` is numeric per the fake's structure, so it should have
            // ended up in `numeric_columns` for right-alignment.
            assert!(view.numeric_columns.contains("id"));
            assert!(!view.numeric_columns.contains("name"));
        });
    }

    #[gpui::test]
    async fn buffer_edits_change_pending_count(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), true, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        let runs_before_save = fake
            .calls()
            .iter()
            .filter(|call| call.starts_with("run_query"))
            .count();

        view.update_in(cx, |view, window, cx| {
            let key = view.row_key_for(0).expect("row 0 should yield a RowKey");
            view.set_cell_value(key, "name", "Alicia".into(), cx);
            assert_eq!(view.pending_change_count(), 1);
            view.save_edits(window, cx);
        });

        // The success handler clears the buffer and merely spawns the
        // reload's tokio task in the same update, so waiting on the buffer
        // alone races that task: it can read as empty before `run_query` for
        // the reload has actually executed. Wait for the reload to actually
        // run instead (finding 7), then assert the settled state.
        wait_until(cx, |cx| {
            view.read_with(cx, |_, _| {
                let runs = fake
                    .calls()
                    .iter()
                    .filter(|call| call.starts_with("run_query"))
                    .count();
                runs > runs_before_save
            })
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=1 i=0 d=0"),
            "save must call apply_edits with one update: {:?}",
            fake.calls()
        );
        view.read_with(cx, |view, _| {
            assert_eq!(view.pending_change_count(), 0, "buffer cleared on success");
            assert!(matches!(view.save_state(), SaveState::Done(_)));
        });
    }

    #[gpui::test]
    async fn save_error_keeps_buffer(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        // Structure and the first page must both succeed so the table is
        // `editable()` (save_edits now gates on it, see finding 2); only
        // `apply_edits` itself fails, to exercise the save error path.
        let fake = fake_with_default_rows();
        fake.set_apply_edits_error(Some("permission denied".into()));
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;
        view.read_with(cx, |view, _| {
            assert!(
                view.editable(),
                "sanity: a PK base table should be editable"
            );
        });

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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
    async fn dirty_sql_bar_blocks_row_mutations(cx: &mut TestAppContext) {
        // A dirty SQL bar suspends editability (see `editable()`), so `add_row`
        // and `delete_row` must be no-ops even though the query base is still
        // `Table` and no save is in flight.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
        });
        view.read_with(cx, |view, _| {
            assert!(
                view.sql_dirty(),
                "hand-editing the bar should mark it dirty"
            );
            assert!(
                !view.editable(),
                "a dirty SQL bar should suspend row editing"
            );
        });

        view.update(cx, |view, cx| {
            let added = view.add_row(cx);
            assert!(
                added.is_none(),
                "add_row must not insert a row while the SQL bar is dirty"
            );
            assert!(view.edits().inserts().is_empty());

            let key = RowKey {
                columns: vec!["id".into()],
                values: vec![Some("1".into())],
            };
            view.delete_row(key, cx);
            assert!(
                view.edits().deletes.is_empty(),
                "delete_row must not mark a delete while the SQL bar is dirty"
            );
        });
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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

    #[gpui::test]
    async fn sql_editor_starts_with_current_sql(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.read_with(cx, |view, cx| {
            let editor_text = view.sql_editor.read(cx).text(cx);
            assert_eq!(
                editor_text,
                view.current_sql(),
                "the SQL bar should start showing the SQL it just ran"
            );
        });
    }

    #[gpui::test]
    async fn run_dirty_text_enters_custom_read_only_mode(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;
        view.read_with(cx, |view, _| {
            assert!(view.editable(), "a PK base table should start editable");
        });

        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
        });
        view.read_with(cx, |view, _| {
            assert!(
                view.sql_dirty(),
                "hand-editing the bar should mark it dirty"
            );
            assert!(
                !view.editable(),
                "a dirty SQL bar should suspend row editing"
            );
        });

        view.update_in(cx, |view, window, cx| view.run_from_editor(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(
                matches!(view.query().base, QueryBase::Custom(_)),
                "running dirty text should switch the query to a custom base"
            );
            assert!(
                view.query().filters.is_empty(),
                "entering custom mode resets the filter overlay"
            );
            assert!(
                view.query().sort.is_none(),
                "entering custom mode resets the sort overlay"
            );
            assert!(!view.sql_dirty(), "running the text clears dirtiness");
            assert!(
                !view.editable(),
                "custom-query results are read-only regardless of structure"
            );
            assert!(view.query().is_custom());
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert_eq!(last, "SELECT 1");
    }

    #[gpui::test]
    async fn ui_sort_in_custom_mode_wraps_subquery(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = QueryResult {
            columns: vec!["a".into()],
            rows: vec![vec![Some("1".into())]],
            truncated: false,
            command_tag: None,
        };
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Enter custom mode with a hand-typed query.
        view.update_in(cx, |view, window, cx| {
            view.sql_editor.update(cx, |editor, cx| {
                editor.set_text("SELECT a FROM t", window, cx)
            });
            view.run_from_editor(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().is_custom() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        // A UI-driven sort (as if the user clicked a header) mutates the
        // overlay and wraps the custom text in a subquery.
        view.update_in(cx, |view, window, cx| view.toggle_sort("a", window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().sort.is_some() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, cx| {
            let expected = render_sql(view.query());
            assert_eq!(
                expected,
                "SELECT * FROM (\nSELECT a FROM t\n) AS zed_sub ORDER BY \"a\" ASC;"
            );
            let editor_text = view.sql_editor.read(cx).text(cx);
            assert_eq!(
                editor_text, expected,
                "the SQL bar must always show exactly the executed query"
            );
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert_eq!(
            last, "SELECT * FROM (\nSELECT a FROM t\n) AS zed_sub ORDER BY \"a\" ASC;",
            "the fake should have received the same wrapped SQL"
        );
    }

    #[gpui::test]
    async fn reset_returns_to_generated_table_query(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = Arc::new(FakeDatabaseClient::new());

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
            view.run_from_editor(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().is_custom() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| view.reset_to_table_query(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                !view.query().is_custom() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, cx| {
            assert!(matches!(view.query().base, QueryBase::Table(_)));
            assert_eq!(
                view.sql_editor.read(cx).text(cx),
                r#"SELECT * FROM "public"."users" LIMIT 100 OFFSET 0;"#
            );
            assert!(!view.sql_dirty());
            assert!(view.editable(), "leaving custom mode restores editability");
        });
    }

    #[gpui::test]
    async fn programmatic_sync_does_not_mark_dirty(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = rows_result(100);
        let client: Arc<dyn DatabaseClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        let before = view.read_with(cx, |view, cx| view.sql_editor.read(cx).text(cx));

        view.update_in(cx, |view, window, cx| view.toggle_sort("name", window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().sort.is_some() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, cx| {
            let after = view.sql_editor.read(cx).text(cx);
            assert_ne!(before, after, "the sort should change the visible SQL text");
            assert_eq!(after, view.current_sql());
            assert!(
                !view.sql_dirty(),
                "a programmatic resync must not be mistaken for a user edit"
            );
        });
    }

    #[gpui::test]
    async fn run_commits_open_cell_editor_first(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        // Open the inline cell editor on row 0 / column `name` and type a new
        // value without committing it.
        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            let editing = view.editing_cell().expect("edit should be in progress");
            editing.field.update(cx, |field, cx| {
                field.set_text("Alicia", window, cx);
            });
        });

        // Running the SQL bar (a refresh, since the text is unchanged) must
        // finish the open cell editor first, committing it by stable RowKey.
        view.update_in(cx, |view, window, cx| view.run_from_editor(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(
                view.editing_cell().is_none(),
                "running should close the inline cell editor"
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
                .expect("the in-progress edit should be committed keyed by RowKey");
            assert_eq!(cell, &database_client::EditCell::Value("Alicia".into()));
        });
    }

    #[gpui::test]
    async fn sql_editor_focus_does_not_trip_cell_editor_confirm_gate(cx: &mut TestAppContext) {
        // menu::Confirm/Cancel are claimed by the view only when the open cell
        // editor's own field is focused (see `cell_editor_focused`, checked by
        // the `on_action` handlers in `Render for TableDataView`). Focusing the
        // SQL bar's editor instead — e.g. to press cmd-enter for `RunQuery` —
        // must leave that gate false, or Enter/Escape typed into the SQL bar
        // would incorrectly commit/cancel an unrelated open cell edit.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.begin_edit_cell(0, 1, window, cx);
            assert!(
                view.cell_editor_focused(window, cx),
                "opening the cell editor should focus its own field"
            );
        });

        view.update_in(cx, |view, window, cx| {
            view.sql_editor.update(cx, |editor, cx| {
                editor.focus_handle(cx).focus(window, cx);
            });
            assert!(
                !view.cell_editor_focused(window, cx),
                "focusing the SQL bar must not read as the cell editor being focused"
            );
        });
    }

    /// Loads a `TableDataView` seeded with [`fake_with_default_rows`] and waits
    /// for both the first page and structure to be loaded (so `row_key_for` and
    /// PK gating are available), returning the fake alongside the view so tests
    /// can assert on regenerated SQL.
    async fn view_with_default_rows(
        cx: &mut TestAppContext,
    ) -> (
        Arc<FakeDatabaseClient>,
        Entity<TableDataView>,
        &mut VisualTestContext,
    ) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;
        (fake, view, cx)
    }

    #[gpui::test]
    async fn context_menu_filter_entry_adds_eq_filter_with_exact_value(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (fake, view, cx) = view_with_default_rows(cx).await;

        // Row 0's `name` is "Alice"; the RowKey for row 0 makes this an
        // existing-row target so "Edit cell" would also be offered.
        let key = view.read_with(cx, |view, _| {
            view.row_key_for(0).expect("row 0 should yield a RowKey")
        });

        view.update_in(cx, |view, window, cx| {
            view.deploy_cell_context_menu(
                gpui::Point::default(),
                "name".into(),
                Some("Alice".into()),
                Some(EditTarget::Existing(key)),
                window,
                cx,
            );
        });
        view.read_with(cx, |view, _| {
            assert!(view.context_menu_open(), "right-click should open a menu");
        });

        // Simulate clicking "Filter: name = 'Alice'" by calling the same code
        // path the menu entry closure calls.
        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
                Filter {
                    column: "name".into(),
                    op: FilterOp::Eq,
                    value: "Alice".into(),
                },
                window,
                cx,
            );
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.query().filters.len(), 1);
            assert_eq!(view.query().filters[0].op, FilterOp::Eq);
            assert_eq!(view.query().filters[0].value, "Alice");
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.contains(r#""name" = 'Alice'"#),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn context_menu_exclude_entry_adds_not_eq_filter(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (fake, view, cx) = view_with_default_rows(cx).await;

        view.update_in(cx, |view, window, cx| {
            view.deploy_cell_context_menu(
                gpui::Point::default(),
                "name".into(),
                Some("Alice".into()),
                None,
                window,
                cx,
            );
        });
        view.read_with(cx, |view, _| assert!(view.context_menu_open()));

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
                Filter {
                    column: "name".into(),
                    op: FilterOp::NotEq,
                    value: "Alice".into(),
                },
                window,
                cx,
            );
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.contains(r#""name" <> 'Alice'"#),
            "unexpected generated SQL: {last}"
        );
        view.read_with(cx, |view, _| {
            assert_eq!(view.query().filters[0].op, FilterOp::NotEq);
        });
    }

    #[gpui::test]
    async fn context_menu_null_cell_offers_is_null_filters(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (fake, view, cx) = view_with_default_rows(cx).await;

        // Row 2's `name` is NULL in `default_rows_result`.
        view.update_in(cx, |view, window, cx| {
            view.deploy_cell_context_menu(
                gpui::Point::default(),
                "name".into(),
                None,
                None,
                window,
                cx,
            );
        });
        view.read_with(cx, |view, _| assert!(view.context_menu_open()));

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
                Filter {
                    column: "name".into(),
                    op: FilterOp::IsNull,
                    value: String::new(),
                },
                window,
                cx,
            );
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.query().filters[0].op, FilterOp::IsNull);
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.contains(r#""name" IS NULL"#),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn context_menu_null_cell_exclude_uses_is_not_null(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (fake, view, cx) = view_with_default_rows(cx).await;

        view.update_in(cx, |view, window, cx| {
            view.apply_filter_edit(
                None,
                Filter {
                    column: "name".into(),
                    op: FilterOp::IsNotNull,
                    value: String::new(),
                },
                window,
                cx,
            );
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.query().filters[0].op, FilterOp::IsNotNull);
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.contains(r#""name" IS NOT NULL"#),
            "unexpected generated SQL: {last}"
        );
    }

    #[gpui::test]
    async fn view_value_populates_popover_with_full_text(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        let long_value = "x".repeat(200);
        view.update_in(cx, |view, window, cx| {
            view.open_value_popover(gpui::Point::default(), long_value.clone(), window, cx);
        });

        view.read_with(cx, |view, cx| {
            assert_eq!(
                view.value_popover_text(cx).as_deref(),
                Some(long_value.as_str()),
                "the popover should hold the full, untruncated value"
            );
        });
    }

    #[gpui::test]
    async fn view_value_shows_null_literal_for_null_cell(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        view.update_in(cx, |view, window, cx| {
            view.open_value_popover(gpui::Point::default(), "NULL".into(), window, cx);
        });

        view.read_with(cx, |view, cx| {
            assert_eq!(view.value_popover_text(cx).as_deref(), Some("NULL"));
        });
    }

    #[gpui::test]
    async fn edit_cell_entry_shown_for_editable_non_pk_target(cx: &mut TestAppContext) {
        // Positive control for the other `shows_edit_cell_entry` tests below:
        // a non-PK column with a valid target on an editable table shows it.
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        let key = view.read_with(cx, |view, _| view.row_key_for(0).unwrap());
        view.read_with(cx, |view, _| {
            assert!(view.shows_edit_cell_entry("name", &Some(EditTarget::Existing(key))));
        });
    }

    #[gpui::test]
    async fn edit_cell_entry_shown_for_pk_column_on_insert_row(cx: &mut TestAppContext) {
        // Unlike an existing row, a pending insert row has no key yet, so
        // every column - including the primary key it will be created with -
        // stays editable (matching `begin_edit_new_cell`'s behavior).
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        let id = view.update(cx, |view, cx| {
            view.add_row(cx).expect("add_row should yield an insert id")
        });
        view.read_with(cx, |view, _| {
            assert!(view.shows_edit_cell_entry("id", &Some(EditTarget::New(id))));
        });
    }

    #[gpui::test]
    async fn edit_cell_entry_absent_when_target_is_none(cx: &mut TestAppContext) {
        // No `target` (e.g. a header/summary cell with no addressable row)
        // means "Edit cell" cannot be offered even on an editable table.
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        view.read_with(cx, |view, _| {
            assert!(!view.shows_edit_cell_entry("name", &None));
        });

        view.update_in(cx, |view, window, cx| {
            view.deploy_cell_context_menu(
                gpui::Point::default(),
                "name".into(),
                Some("Alice".into()),
                None,
                window,
                cx,
            );
            assert!(
                view.context_menu.is_some(),
                "the menu should still deploy without a target"
            );
        });
    }

    #[gpui::test]
    async fn edit_cell_entry_absent_for_primary_key_column(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        let key = view.read_with(cx, |view, _| view.row_key_for(0).unwrap());
        // "id" is the fake structure's primary-key column: even with a valid
        // target, editing it must not be offered (the PK identifies the row).
        view.read_with(cx, |view, _| {
            assert!(view.is_primary_key_column("id"));
            assert!(!view.shows_edit_cell_entry("id", &Some(EditTarget::Existing(key))));
        });
    }

    #[gpui::test]
    async fn edit_cell_entry_absent_when_sql_dirty(cx: &mut TestAppContext) {
        // A dirty SQL bar suspends `editable()` (see
        // `dirty_sql_bar_blocks_row_mutations`), so even a valid target on a
        // non-PK column must not offer "Edit cell".
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        let key = view.read_with(cx, |view, _| view.row_key_for(0).unwrap());
        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
        });
        view.read_with(cx, |view, _| {
            assert!(
                !view.editable(),
                "a dirty SQL bar should suspend row editing, hence Edit cell"
            );
            assert!(!view.shows_edit_cell_entry("name", &Some(EditTarget::Existing(key))));
        });
    }

    #[gpui::test]
    async fn edit_cell_entry_absent_for_custom_query_mode(cx: &mut TestAppContext) {
        // Custom SQL results are read-only (no table to address UPDATE
        // against), independent of the dirty-bar gate above.
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        let key = view.read_with(cx, |view, _| view.row_key_for(0).unwrap());
        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
            view.run_from_editor(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(
                matches!(view.query().base, QueryBase::Custom(_)),
                "running the dirty text should enter custom-query mode"
            );
            assert!(!view.shows_edit_cell_entry("name", &Some(EditTarget::Existing(key))));
        });
    }

    #[gpui::test]
    async fn dismiss_event_clears_context_menu(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        view.update_in(cx, |view, window, cx| {
            view.deploy_cell_context_menu(
                gpui::Point::default(),
                "name".into(),
                Some("Alice".into()),
                None,
                window,
                cx,
            );
        });
        view.read_with(cx, |view, _| {
            assert!(view.context_menu_open(), "menu should be open after deploy");
        });

        let menu = view.read_with(cx, |view, _| {
            view.context_menu
                .as_ref()
                .expect("menu should be set")
                .0
                .clone()
        });
        menu.update(cx, |_, cx| cx.emit(DismissEvent));
        cx.run_until_parked();

        view.read_with(cx, |view, _| {
            assert!(
                !view.context_menu_open(),
                "DismissEvent should clear the context_menu field"
            );
        });
    }

    #[gpui::test]
    async fn dismiss_event_clears_value_popover(cx: &mut TestAppContext) {
        let cx = &mut cx.clone();
        let (_fake, view, cx) = view_with_default_rows(cx).await;

        view.update_in(cx, |view, window, cx| {
            view.open_value_popover(gpui::Point::default(), "Alice".into(), window, cx);
        });
        view.read_with(cx, |view, cx| {
            assert!(view.value_popover_text(cx).is_some());
        });

        let popover = view.read_with(cx, |view, _| {
            view.value_popover
                .as_ref()
                .expect("popover should be set")
                .0
                .clone()
        });
        popover.update(cx, |_, cx| cx.emit(DismissEvent));
        cx.run_until_parked();

        view.read_with(cx, |view, cx| {
            assert!(
                view.value_popover_text(cx).is_none(),
                "DismissEvent should clear the value_popover field"
            );
        });
    }

    #[gpui::test]
    fn truncate_for_label_shortens_long_values(_cx: &mut TestAppContext) {
        assert_eq!(TableDataView::truncate_for_label("short", 40), "short");
        let long = "a".repeat(50);
        let truncated = TableDataView::truncate_for_label(&long, 40);
        assert_eq!(truncated.chars().count(), 41, "40 chars plus the ellipsis");
        assert!(truncated.ends_with('…'));
    }

    // -- Fix cluster A: dirty/custom state machine & edit-buffer reconciliation --

    #[gpui::test]
    async fn toggle_sort_while_dirty_runs_bar_text_then_sorts(cx: &mut TestAppContext) {
        // Finding 1: a UI mutator invoked while the bar is dirty must not
        // silently discard the hand-typed text by reapplying the *old* query.
        // The chosen model promotes the dirty text first (as `run_from_editor`
        // would), then layers the mutator's own change on top.
        init_test(cx);
        cx.executor().allow_parking();
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = rows_result(5);
        let fake = Arc::new(fake);
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.sql_editor.update(cx, |editor, cx| {
                editor.set_text("SELECT * FROM users WHERE id > 1", window, cx)
            });
        });
        view.read_with(cx, |view, _| assert!(view.sql_dirty()));

        view.update_in(cx, |view, window, cx| view.toggle_sort("name", window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(
                matches!(view.query().base, QueryBase::Custom(_)),
                "the dirty text should have been promoted to a custom base"
            );
            assert!(!view.sql_dirty(), "promoting the text clears dirtiness");
            let sort = view.query().sort.as_ref().expect("sort should be applied");
            assert_eq!(sort.column, "name");
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert!(
            last.contains("WHERE id > 1"),
            "the executed SQL must be built from the hand-typed text, not the stale query: {last}"
        );
        assert!(
            last.contains("ORDER BY \"name\""),
            "the sort should also have been applied on top: {last}"
        );
    }

    #[gpui::test]
    async fn refresh_while_dirty_runs_bar_text(cx: &mut TestAppContext) {
        // Finding 1: Refresh at dirty must behave like Run, not like a re-run
        // of the stale query.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
        });
        view.read_with(cx, |view, _| assert!(view.sql_dirty()));

        view.update_in(cx, |view, window, cx| view.refresh(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(
                matches!(view.query().base, QueryBase::Custom(_)),
                "Refresh at dirty must run the bar text, entering custom mode"
            );
            assert!(!view.sql_dirty());
        });
        let last = last_run_query_sql(&fake).expect("run_query should have been called");
        assert_eq!(last, "SELECT 1");
    }

    #[gpui::test]
    async fn save_success_does_not_clobber_dirty_bar_text(cx: &mut TestAppContext) {
        // Finding 1: the save success handler must not resync the editor text
        // (which would overwrite an unsaved hand-typed edit) while dirty.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
        view.read_with(cx, |view, _| assert_eq!(view.pending_change_count(), 1));

        // Kick off the save while still editable (clean bar): it must be
        // in flight, not yet resolved, when the user starts typing below.
        view.update_in(cx, |view, window, cx| view.save_edits(window, cx));
        view.read_with(cx, |view, _| {
            assert_eq!(view.save_state(), &SaveState::Saving);
        });

        // The user starts typing a new query in the bar without running it,
        // racing the in-flight save's completion.
        view.update_in(cx, |view, window, cx| {
            view.sql_editor.update(cx, |editor, cx| {
                editor.set_text("SELECT * FROM users -- work in progress", window, cx)
            });
        });
        view.read_with(cx, |view, _| assert!(view.sql_dirty()));

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                matches!(view.save_state(), SaveState::Done(_))
            })
        })
        .await;

        view.read_with(cx, |view, cx| {
            let editor_text = view.sql_editor.read(cx).text(cx);
            assert_eq!(
                editor_text, "SELECT * FROM users -- work in progress",
                "the save success handler must not overwrite unsaved bar text"
            );
            assert!(
                view.sql_dirty(),
                "the bar should remain dirty since its text was not resynced"
            );
        });
    }

    #[gpui::test]
    async fn run_custom_with_pending_edits_is_blocked(cx: &mut TestAppContext) {
        // Finding 2, part 1: entering custom mode with a non-empty edit buffer
        // must not happen implicitly. The run is refused, the buffer survives,
        // and a notice is shown instead.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
        view.read_with(cx, |view, _| assert_eq!(view.pending_change_count(), 1));

        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
            view.run_from_editor(window, cx);
        });
        cx.run_until_parked();

        view.read_with(cx, |view, _| {
            assert!(
                matches!(view.query().base, QueryBase::Table(_)),
                "the query must not have entered custom mode"
            );
            assert_eq!(
                view.pending_change_count(),
                1,
                "the pending edit must survive the refused run"
            );
            assert!(
                view.pending_edits_notice().is_some(),
                "a notice should explain why the run was refused"
            );
        });
        assert!(
            fake.calls()
                .iter()
                .all(|call| !call.starts_with("run_query sql=SELECT 1")),
            "the custom text must not have been executed: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn refresh_of_unchanged_query_ignores_pending_edits(cx: &mut TestAppContext) {
        // Finding 2, part 1 parenthetical: a plain refresh (bar text ==
        // current_sql) is unaffected by the pending-edits gate, since it does
        // not enter custom mode.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
        view.read_with(cx, |view, _| assert_eq!(view.pending_change_count(), 1));

        view.update_in(cx, |view, window, cx| view.refresh(window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(matches!(view.query().base, QueryBase::Table(_)));
            assert_eq!(
                view.pending_change_count(),
                1,
                "refreshing the same query must not touch the buffer"
            );
        });
    }

    #[gpui::test]
    async fn change_controls_hidden_in_custom_mode(cx: &mut TestAppContext) {
        // Finding 2, part 2: `editable()` gates the Save/Discard controls, not
        // just `pending_change_count() > 0`. Since the buffer is now kept empty
        // across a custom-mode transition (part 1), this exercises the
        // defense-in-depth path directly by entering custom mode from a clean
        // buffer, which must show no edit controls even before a buffer could
        // ever be populated.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
            view.run_from_editor(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.query().is_custom() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(!view.editable(), "custom-mode results must be read-only");
        });
    }

    #[gpui::test]
    async fn save_edits_is_noop_when_not_editable(cx: &mut TestAppContext) {
        // Finding 2, part 2: `save_edits` must early-return when `!editable()`,
        // even if the buffer somehow carries a pending change (defense in
        // depth against the buffer/mode ever desyncing again).
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        // Buffer an update directly (bypassing the now-gated UI path) to
        // simulate a buffer that is non-empty while not editable.
        let key = view.read_with(cx, |view, _| view.row_key_for(0).unwrap());
        view.update_in(cx, |view, _window, cx| {
            view.set_cell_value(key, "name", "Alicia".into(), cx);
        });
        view.read_with(cx, |view, _| assert_eq!(view.pending_change_count(), 1));

        // Make the bar dirty, which suspends `editable()` without touching the
        // buffer directly.
        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
        });
        view.read_with(cx, |view, _| assert!(!view.editable()));

        view.update_in(cx, |view, window, cx| view.save_edits(window, cx));
        cx.run_until_parked();

        assert!(
            !fake
                .calls()
                .iter()
                .any(|call| call.starts_with("apply_edits")),
            "save_edits must not apply while not editable: {:?}",
            fake.calls()
        );
        view.read_with(cx, |view, _| {
            assert_eq!(
                view.pending_change_count(),
                1,
                "the buffer must be left untouched by the refused save"
            );
        });
    }

    #[gpui::test]
    async fn custom_mode_does_not_render_insert_rows_or_buffered_overlays(cx: &mut TestAppContext) {
        // Finding 2, part 3: even if the buffer were somehow non-empty while
        // `base != Table` (belt-and-suspenders against a future desync), the
        // grid must not layer insert rows or update/delete overlays that the
        // custom query never returned.
        init_test(cx);
        cx.executor().allow_parking();
        let fake = fake_with_default_rows();
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.page().is_some() && view.structure().is_some()
            })
        })
        .await;

        let key = view.read_with(cx, |view, _| view.row_key_for(0).unwrap());
        view.update_in(cx, |view, _window, cx| {
            view.set_cell_value(key.clone(), "name", "Alicia".into(), cx);
        });
        view.read_with(cx, |view, _| assert_eq!(view.pending_change_count(), 1));

        // Force a transition to custom mode while bypassing the run-time guard,
        // to exercise the render-time defense directly regardless of how the
        // buffer ended up non-empty.
        view.update_in(cx, |view, window, cx| {
            view.query.base = QueryBase::Custom("SELECT id, name FROM users".into());
            view.restart_query(window, cx);
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.load_state() == &LoadState::Idle)
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(matches!(view.query().base, QueryBase::Custom(_)));
            let value = view.cell_display_value_for_test(&key, "name");
            assert_ne!(
                value.as_deref(),
                Some("Alicia"),
                "a buffered update must not be layered onto a custom-mode grid"
            );
        });
    }

    #[gpui::test]
    async fn buffer_edited_while_open_editor_closes_it_and_blocks_commits(cx: &mut TestAppContext) {
        // Finding 3: an already-open inline editor must not remain interactive
        // once the bar goes dirty. Transitioning to dirty finishes (commits)
        // the in-progress edit and closes the editor; further commit paths are
        // then no-ops because `editable()` is false.
        init_test(cx);
        cx.executor().allow_parking();
        let client: Arc<dyn DatabaseClient> = fake_with_default_rows();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            TableDataView::new(client, "local".into(), table_ref(), false, None, window, cx)
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
        });
        view.read_with(cx, |view, _| {
            assert!(view.editing_cell().is_some(), "the editor should be open");
        });

        // Hand-edit the SQL bar without running it: this is the transition to
        // dirty that must close the still-open cell editor.
        view.update_in(cx, |view, window, cx| {
            view.sql_editor
                .update(cx, |editor, cx| editor.set_text("SELECT 1", window, cx));
        });

        view.read_with(cx, |view, _| {
            assert!(view.sql_dirty());
            assert!(
                view.editing_cell().is_none(),
                "going dirty must close the open inline editor"
            );
            assert_eq!(
                view.pending_change_count(),
                1,
                "the in-progress edit should have been committed, not dropped"
            );
        });

        // Further commit-path calls must be no-ops while not editable.
        view.update_in(cx, |view, _window, cx| {
            view.set_editing_cell_null(cx);
        });
        view.read_with(cx, |view, _| {
            let key = RowKey {
                columns: vec!["id".into()],
                values: vec![Some("1".into())],
            };
            let cell = view
                .edits()
                .updates()
                .get(&key)
                .and_then(|row| row.get("name"));
            assert_eq!(
                cell,
                Some(&database_client::EditCell::Value("Alicia".into())),
                "set_editing_cell_null must not run with no open editor"
            );
        });
    }
}
