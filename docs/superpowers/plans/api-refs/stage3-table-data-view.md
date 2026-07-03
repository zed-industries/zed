# TableDataView — current state map for the stage-3 SQL-first rework

Target file: `/Users/user/zed/crates/database_ui/src/table_data_view.rs` (3776 lines).
Sibling files referenced: `crates/database_client/src/database_client.rs` (types + trait),
`crates/database_client/src/fake.rs` (test client), `crates/database_ui/src/sql_query_view.rs`
(the `run_query` pattern stage 3 adopts), `crates/database_ui/src/database_settings.rs`.

## 1. Actions and constants

```rust
// table_data_view.rs:24-42
actions!(database, [NextPage, PrevPage, ToggleStructure, RefreshData,
                    CommitCellEdit, CancelCellEdit, SetCellNull]);

const COLUMN_WIDTH: f32 = 180.;                       // line 45, default resizable col width
fn filter_op_label(op: FilterOp) -> &'static str      // line 48  ("=", "≠", ">", "<", "contains", "is null")
fn all_filter_ops() -> [FilterOp; 6]                  // line 60  (dropdown order)
```

`filter_op_label`/`all_filter_ops` feed the old filter builder; stage 3 removes the
builder but the funnel popup will need an equivalent (plus new `FilterOp::IsNotNull`).

## 2. Support types

```rust
pub enum ViewMode { Data, Structure }                             // line 73
pub enum LoadState { Idle, Loading, Error(String) }               // line 80
pub enum SaveState { Idle, Saving, Done(String), Error(String) }  // line 88
pub struct InsertId(u64);                                         // line 106, stable id for a pending insert row
pub enum EditTarget { Existing(RowKey), New(InsertId) }           // line 112

pub struct EditingCell {                                          // line 122
    pub target: EditTarget,
    pub column: String,
    pub display_row: usize,      // page row for Existing, insert index for New
    pub column_index: usize,
    pub original: Option<String>, // display value when editor opened; None == NULL
    pub field: Entity<InputField>,
}

#[derive(Debug, Default)]
pub struct TableEditBuffer {                                      // line 147
    updates: HashMap<RowKey, HashMap<String, EditCell>>,  // keyed by original PK values
    inserts: Vec<(InsertId, HashMap<String, EditCell>)>,  // display order
    deletes: HashSet<RowKey>,
}
impl TableEditBuffer {
    pub fn pending_change_count(&self) -> usize                   // line 161: updates+inserts+deletes lens
    pub fn updates(&self) -> &HashMap<RowKey, HashMap<String, EditCell>>  // line 165
    pub fn inserts(&self) -> &[(InsertId, HashMap<String, EditCell>)]     // line 169
    pub fn deletes(&self) -> &HashSet<RowKey>                     // line 173
    fn insert(&self, id) / insert_mut(&mut self, id)              // lines 178/186 (lookup by InsertId)
    fn clear(&mut self)                                           // line 193
}

fn compute_editable(is_view: bool, columns: &[ColumnInfo]) -> bool  // line 202: !is_view && any PK
fn column_info_from_name(name: &String) -> ColumnInfo               // line 210: text-typed fallback ColumnInfo
```

## 3. `struct TableDataView` — all fields (lines 227–277)

```rust
pub struct TableDataView {
    focus_handle: FocusHandle,
    client: Arc<dyn DatabaseClient>,
    connection: String,            // part of tab dedup key (with TableRef)
    table: TableRef,
    is_view: bool,                 // provided by the tree at open time
    editable: bool,                // false until structure loads, then compute_editable
    edits: TableEditBuffer,
    next_insert_id: u64,           // monotonic, never reset within tab lifetime
    editing_cell: Option<EditingCell>,
    save_state: SaveState,
    mode: ViewMode,
    spec: SelectSpec,              // <-- stage 3 replaces with QueryState-driven text
    page: Option<Arc<RowsPage>>,   // Arc so uniform_list re-renders clone cheaply
    structure: Option<TableStructure>,
    load_state: LoadState,
    interaction: Entity<TableInteractionState>,          // ui::Table scroll/resize state
    column_widths: Option<Entity<ResizableColumnsState>>, // recreated when col count changes
    filter_builder_open: bool,     // stage 3: delete
    draft_column: Option<String>,  // stage 3: delete
    draft_op: FilterOp,            // stage 3: delete
    draft_value: Entity<InputField>, // stage 3: delete
    _data_task: Option<Task<()>>,       // separate from _structure_task on purpose
    _structure_task: Option<Task<()>>,
    _save_task: Option<Task<()>>,
}
```

## 4. Constructor and accessors

```rust
pub fn new(client: Arc<dyn DatabaseClient>, connection: String, table: TableRef,
           is_view: bool, window: &mut Window, cx: &mut App) -> Entity<Self>   // line 280
```
- Reads `DatabaseSettings::get_global(cx).page_size.max(1) as usize` into `spec.limit` (line 288;
  setting defined at `database_settings.rs:6`, default 100).
- Calls `reload_data(cx)` and `reload_structure(cx)` eagerly (lines 321–325) — structure loads
  with the first page so editability is known without opening the Structure tab.

Read accessors (lines 330–382): `table()`, `connection()`, `spec() -> &SelectSpec`,
`page() -> Option<&RowsPage>`, `structure()`, `load_state()`, `mode()`, `editable()`,
`edits() -> &TableEditBuffer`, `editing_cell()`, `save_state()`, `pending_change_count()`.
Tests use these heavily — keep equivalents (e.g. a `sql_text()` accessor) in stage 3.

## 5. Data lifecycle

```rust
fn reload_data(&mut self, cx: &mut Context<Self>)        // line 1032
fn reload_structure(&mut self, cx: &mut Context<Self>)   // line 1064
fn refresh(&mut self, cx: &mut Context<Self>)            // line 1024 (finish_editing + reload_data + maybe reload_structure)
fn set_column_widths(&mut self, cols: usize, cx: &mut Context<Self>) // line 1094
```

`reload_data` pattern (the one to convert from `fetch_rows(&table, &spec)` to
`run_query(&database, &sql, max_rows)`):

```rust
self.load_state = LoadState::Loading;
cx.notify();
let client = self.client.clone();
let table = self.table.clone();
let spec = self.spec.clone();
let task = gpui_tokio::Tokio::spawn_result(cx, async move { client.fetch_rows(&table, &spec).await });
self._data_task = Some(cx.spawn(async move |this, cx| {
    let result = task.await;
    this.update(cx, |this, cx| {
        match result {
            Ok(page) => {
                this.set_column_widths(page.columns.len(), cx);   // line 1050
                this.page = Some(Arc::new(page));
                this.load_state = LoadState::Idle;
            }
            Err(error) => this.load_state = LoadState::Error(format!("{error:#}")),
        }
        cx.notify();
    }).log_err();
}));
```

Traps:
- DB calls MUST run through `gpui_tokio::Tokio::spawn_result` (tokio runtime), not
  `cx.background_spawn`.
- Assigning `_data_task` drops the previous task → an in-flight fetch is aborted when
  superseded. Tests therefore wait for `LoadState::Idle` between operations.
- `_data_task` and `_structure_task` are deliberately separate fields; merging them
  regressed once (`refresh_reloads_data_even_with_cached_structure`, line 2354).
- `set_column_widths` only recreates `ResizableColumnsState` when the column count
  changes (line 1099–1104), so manual resize survives reloads of the same shape. The
  stage-3 auto-width must respect that interplay (`ResizableColumnsState::new(cols,
  vec![AbsoluteLength::Pixels(px(COLUMN_WIDTH)); cols], vec![TableResizeBehavior::Resizable; cols])`).
- `reload_structure` sets `self.editable = compute_editable(this.is_view, &structure.columns)`
  on success (line 1078).

### Spec mutations (all call `finish_editing` first, reset `offset = 0` except paging)

```rust
pub fn toggle_sort(&mut self, column: &str, cx: &mut Context<Self>)   // line 938: None→Asc→Desc→None, offset=0, reload
pub fn add_filter(&mut self, filter: Filter, cx: &mut Context<Self>)  // line 962: push, offset=0, reload
pub fn remove_filter(&mut self, index: usize, cx: &mut Context<Self>) // line 971: OOB index = logged no-op
pub fn next_page(&mut self, cx: &mut Context<Self>)                   // line 986: gated on page.has_more; offset += limit
pub fn prev_page(&mut self, cx: &mut Context<Self>)                   // line 997: no-op at offset 0; saturating_sub(limit)
pub fn toggle_structure(&mut self, cx: &mut Context<Self>)            // line 1008: flips mode; fetches structure if None
```

`has_more` today comes from the server-side `limit+1` probe inside
`fetch_rows` (`RowsPage.has_more`, database_client.rs:107). Stage 3 replaces this with
the client-side `rows.len() == limit` heuristic since `run_query` executes LIMIT verbatim.

## 6. Editing machinery (stage 2 — behavior must be preserved)

```rust
pub fn row_key_for(&self, display_row: usize) -> Option<RowKey>       // line 388: PK cols from structure + page values
fn is_primary_key_column(&self, column: &str) -> bool                 // line 409
pub fn set_cell_value(&mut self, row_key: RowKey, column: &str, value: String, cx) // line 420
pub fn set_cell_null(&mut self, row_key: RowKey, column: &str, cx)    // line 432
fn set_cell(&mut self, row_key, column, cell: EditCell, cx)           // line 436: no-op if saving / PK col / row in deletes
pub fn add_row(&mut self, cx) -> Option<InsertId>                     // line 461: None if saving
pub fn set_new_cell_value(&mut self, id: InsertId, column, value, cx) // line 476: PK cols ARE settable on inserts
pub fn set_new_cell_null(&mut self, id: InsertId, column, cx)         // line 488
pub fn delete_new_row(&mut self, id: InsertId, cx)                    // line 508: removes insert, closes its editor
pub fn delete_row(&mut self, row_key: RowKey, cx)                     // line 538: drops buffered update, closes editor, adds delete
fn is_saving(&self) -> bool                                           // line 561
fn clear_finished_save_state(&mut self)                               // line 568: Done/Error -> Idle on new dirt
pub fn discard_edits(&mut self, cx)                                   // line 575
fn cell_display_value(&self, row_key, column, page_value) -> Option<String> // line 585: buffered edit wins over page
pub fn begin_edit_cell(&mut self, display_row, column_index, window, cx)    // line 609
pub fn begin_edit_new_cell(&mut self, id, display_row, column_index, window, cx) // line 670
pub fn commit_cell_edit(&mut self, _window: &mut Window, cx)          // line 724 (thin wrapper)
fn commit_cell_edit_inner(&mut self, cx)                              // line 731: windowless; skips unchanged/untouched-NULL
pub fn finish_editing(&mut self, cx)                                  // line 769  <-- THE invariant hook
pub fn cancel_cell_edit(&mut self, cx)                                // line 782
fn cell_editor_focused(&self, window: &Window, cx: &App) -> bool      // line 792: gates Enter/Escape claiming
pub fn set_editing_cell_null(&mut self, cx)                           // line 799
pub fn save_edits(&mut self, cx)                                      // line 819
fn build_table_edits(&self) -> TableEdits                             // line 880: delete supersedes update (filter at 888)
```

Invariants (each has a regression test):
- **`finish_editing(cx)` must run before every operation that changes the on-screen row
  set** (sort, filter add/remove, next/prev page, refresh) and before `save_edits`
  snapshots the buffer. Commit is keyed by the stable `RowKey`/`InsertId` captured at
  editor open, not display position. All new stage-3 restart paths (Run, page-size
  change, Reset-to-table, filters from context menu/funnel) must call it too.
- While `SaveState::Saving` the buffer is frozen: every mutator early-returns via
  `is_saving()`; `finish_editing` is also a no-op then (line 770).
- A row in `deletes` can neither be edited (`set_cell` line 446, `begin_edit_cell`
  line 633) nor double-counted at apply (`build_table_edits` line 888).
- PK cells of existing rows are not editable; every column of a pending insert is.
- `begin_edit_cell` creates `InputField::new(window, cx, "")`, `field.set_text(value,
  window, cx)`, then `field.focus_handle(cx).focus(window, cx)` (lines 643–650).
- `save_edits` (line 819): needs `ColumnInfo`s — takes `structure.columns` or falls back
  to `column_info_from_name` over page columns; on Ok clears buffer, sets
  `SaveState::Done("Saved: N updated, ...")` and calls `reload_data`; on Err keeps the
  buffer and sets `SaveState::Error`.
- Test-only helpers: `build_table_edits_for_test()` line 924, `clear_structure_for_test()`
  line 932.

## 7. Render pipeline

`impl Render` at line 1965. Section order inside `render()` (lines 1966–2029):

1. `body` = `render_error` (LoadState::Error) | `render_structure` (Structure mode)
   | `render_data` (Data mode).
2. `v_flex().key_context("TableDataView").track_focus(&self.focus_handle)` with
   `.on_action` for all 7 actions plus `menu::Confirm`/`menu::Cancel` (lines 1995–2008):
   Confirm/Cancel are claimed ONLY when `cell_editor_focused`, else `cx.propagate()`.
   **Trap for stage 3: the SQL editor input must not get its Enter hijacked — keep the
   focus gating pattern.**
3. Children in order: header bar (`schema.name` label + `render_toggle`, lines 2011–2024)
   → `render_edit_toolbar` (Option, Data mode only) → `render_filter_bar` (Data mode only)
   → body in `v_flex().flex_1().size_full().overflow_hidden()` → `render_footer`
   (Data mode only, line 2028).

### Grid: `render_data` (line 1115)

Uses `ui::Table` (see `api-refs/table.md`) with virtualization:

```rust
Table::new(column_count)
    .interactable(&self.interaction)      // Entity<TableInteractionState>
    .striped()                             // zebra already on
    .width_config(ColumnWidthConfig::Resizable(widths))  // Entity<ResizableColumnsState>
    .header(headers)                       // Vec<AnyElement> from render_header
    .uniform_list("db-rows", total_row_count, cx.processor(move |this, range: Range<usize>, window, cx| {
        // page rows -> render_data_cell(row, col, value, window, cx)
        // then insert rows -> render_insert_cell(insert_index, col, window, cx)
    }))
    .map_row(cx.processor(move |this, (row_index, row), _window, cx| {
        this.map_data_row(row_index, row, page_row_count, created_bg, deleted_bg, cx)
    }))
```

`total_row_count = page.rows.len() + edits.inserts.len()` — pending inserts render below
page rows in the same virtualized list (line 1134). Returns empty `v_flex()` when `page`
or `column_widths` is `None` (lines 1116–1121).

Row/cell renderers:
- `fn map_data_row(&self, row_index, row: gpui::Stateful<gpui::Div>, page_row_count,
  created_background: Hsla, deleted_background: Hsla, cx) -> AnyElement` — line 1192.
  Resolves `InsertId`/`RowKey` at render time (not in the click handler), wraps the row
  in `.group("db-row-{i}")` with a hover-revealed Trash `IconButton`; tints inserts green
  (`created_cell_background`, line 2071) and deletes red + `line_through`
  (`deleted_cell_background`, line 2078).
- `fn render_data_cell(&self, display_row, column_index, page_value: Option<String>,
  _window, cx) -> AnyElement` — line 1268. Order: (a) if `editing_cell` matches by
  `display_row`+`column_index` and target is `Existing` → `render_cell_editor`; (b)
  buffered edit → highlighted value (`modified_cell_background`, line 2065); (c) value
  with `.whitespace_nowrap().text_ellipsis()`; NULL → `Label::new("NULL")
  .color(Color::Muted).italic()` (line 1319). Editable cells get an id
  `ElementId::NamedInteger("db-cell-{col}", row)` and an `on_click` that opens the
  editor on `event.click_count() >= 2` (line 1338).
- `fn render_cell_editor(&self, field: Entity<InputField>, cx) -> AnyElement` — line 1351.
  `h_flex` of the field (flex_1) + "∅ NULL" compact button → `set_editing_cell_null`.
- `fn render_insert_cell(&self, insert_index, column_index, _window, cx) -> AnyElement` —
  line 1374. Same editor embed; unset column shows muted italic `default`.
- `fn render_header(&self, index: usize, column: &str, cx) -> AnyElement` — line 1437.
  Currently: label + a separate sort `Button` ("↑"/"↓"/"↕", Filled when sorted) whose
  `on_click` calls `toggle_sort`. Stage 3 replaces this with whole-header click cycling
  and a hover funnel popup.
- `fn render_structure(&self) -> AnyElement` — line 1480 (6-col `Table` + indexes list;
  unchanged in stage 3).
- `fn render_toggle(&self, cx) -> AnyElement` — line 1554 (Data/Structure buttons).

### Chrome to be reworked/removed in stage 3

- `fn available_columns(&self) -> Vec<String>` — line 1580 (from page header; may survive
  for the funnel popup).
- `fn draft_apply_enabled(&self, cx: &App) -> bool` — line 1589 — DELETE.
- `fn apply_draft_filter(&mut self, window, cx)` — line 1598 — DELETE.
- `fn render_filter_bar(&self, window, cx) -> AnyElement` — line 1625 — becomes the
  chips row (chips survive, "+ Filter" button goes away).
- `fn render_filter_chip(&self, index, filter: &Filter, cx) -> AnyElement` — line 1660 —
  keep/extend (click-to-edit popup, sort chip).
- `fn render_filter_builder(&self, _window, cx) -> AnyElement` — line 1693 — DELETE
  (uses `PopoverMenu::new(...).trigger(Button...).anchor(Anchor::TopLeft).menu(...)`
  with `ContextMenu::build` — reusable pattern for the funnel popup).
- `fn render_footer(&self, cx) -> AnyElement` — line 1780 — rework: today it is
  `rows {start}–{end}{+}` summary + prev/refresh/next `IconButton`s; stage 3 adds page
  size dropdown, "+ Row", Save/Discard, `N rows · M ms`, Refresh.
- `fn render_edit_toolbar(&self, cx) -> Option<AnyElement>` — line 1836 — absorbed into
  the footer per spec (read-only banner logic at lines 1837–1861: `None` until structure
  loads, then "Read-only: this is a view" / "Read-only: table has no primary key").
- `fn render_error(&self, message: &str, cx) -> AnyElement` — line 1950 (centered error +
  Retry → `refresh`).
- Struct fields to delete: `filter_builder_open`, `draft_column`, `draft_op`,
  `draft_value` (lines 263–269).

## 8. Item impl and tab opening

```rust
impl Focusable for TableDataView      // line 2032
impl EventEmitter<()> for TableDataView {}   // line 2038
impl Item for TableDataView {         // line 2040
    type Event = ();
    fn tab_icon(..) -> Option<Icon>            // IconName::FileTree
    fn tab_content_text(..) -> SharedString    // "{schema}.{name}"
    fn tab_tooltip_text(..) -> Option<SharedString>  // "{database}.{schema}.{name}"
}

pub fn open_table_tab(workspace: &WeakEntity<Workspace>, client: Arc<dyn DatabaseClient>,
    connection: String, table: TableRef, is_view: bool,
    window: &mut Window, cx: &mut App)         // line 2085
```
Dedup predicate: `view.connection() == connection && view.table() == &table` over
`workspace.active_pane().read(cx).items_of_type::<TableDataView>()` (lines 2096–2103).

## 9. database_client types (database_client.rs)

```rust
pub enum FilterOp { Eq, NotEq, Gt, Lt, Contains, IsNull }   // line 67; stage 3 adds IsNotNull
pub struct Filter { pub column: String, pub op: FilterOp, pub value: String }  // line 77
pub enum SortDirection { Asc, Desc }                        // line 84
pub struct Sort { pub column: String, pub direction: SortDirection }  // line 90
pub struct SelectSpec { pub filters: Vec<Filter>, pub sort: Option<Sort>,
                        pub limit: usize, pub offset: usize }  // line 96 (Default)
pub struct RowsPage { pub columns: Vec<String>,
                      pub rows: Vec<Vec<Option<String>>>,   // None = NULL
                      pub has_more: bool }                  // line 104
pub struct QueryResult { pub columns: Vec<String>, pub rows: Vec<Vec<Option<String>>>,
                         pub truncated: bool, pub command_tag: Option<String> }  // line 111
pub enum EditCell { Value(String), Null }                   // line 121
pub struct RowKey { pub columns: Vec<String>, pub values: Vec<Option<String>> } // line 130 (Hash)
pub struct TableEdits { updates: Vec<RowUpdate>, inserts: Vec<RowInsert>, deletes: Vec<RowDelete> } // line 152
pub struct AppliedCounts { updated, inserted, deleted: usize }  // line 159

#[async_trait::async_trait]
pub trait DatabaseClient: Send + Sync {                     // line 166
    async fn table_structure(&self, table: &TableRef) -> Result<TableStructure>;  // line 171
    async fn fetch_rows(&self, table: &TableRef, spec: &SelectSpec) -> Result<RowsPage>;  // line 172, stage 3 removes if unused
    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult>;  // line 173
    async fn apply_edits(&self, table: &TableRef, columns: &[ColumnInfo], edits: &TableEdits) -> Result<AppliedCounts>;  // line 176
    async fn cancel_running(&self) -> Result<()>;           // line 183
    // + test_connection, list_databases, list_schemas, list_tables
}
```

`SqlQueryView` reference for the `run_query` conversion (`sql_query_view.rs`):
`const UI_MAX_QUERY_ROWS: usize = 1000` (line 32); `run_query` at line 155 times with
`Instant::now()` and stores `elapsed: Option<Duration>` (rendered as `{} ms`, line 249);
`cancel_query` at line 195 calls `client.cancel_running()`.

## 10. FakeDatabaseClient (database_client/src/fake.rs)

```rust
pub struct FakeDatabaseClient {           // line 11; all result fields are pub — override by struct mutation before Arc::new
    pub databases, pub schemas, pub tables,
    pub structure: TableStructure,        // id (int4, PK, NOT NULL) + name (text, nullable)
    pub page: RowsPage,                   // cols [id,name]; rows (1,Alice) (2,Bob) (3,NULL); has_more: true
    pub query_result: QueryResult,        // cols [count]; rows [[3]]; command_tag "SELECT 1"
    pub error: Option<String>,
    calls: Mutex<Vec<String>>,
}
pub fn new() -> Self                      // line 29
pub fn with_error(message: &str) -> Self  // line 91: every method fails with message
pub fn calls(&self) -> Vec<String>        // line 99: recorded call log, in order
```

Call-log formats asserted by tests:
- `fetch_rows {name} limit={} offset={} sort={:?} filters={}` (fake.rs:154)
- `run_query {database} max_rows={n} sql={sql}` (fake.rs:167) — stage-3 tests can assert
  the exact generated SQL text through this string.
- `table_structure {name}`, `apply_edits u={} i={} d={}`, `cancel_running`.

## 11. Test module patterns (table_data_view.rs tests, line 2119)

Setup boilerplate:

```rust
fn init_test(cx: &mut TestAppContext) {          // line 2133
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        gpui_tokio::init(cx);
        crate::init(cx);
    });
}
fn table_ref() -> TableRef                        // line 2144: app/public/users
async fn wait_until(cx: &mut VisualTestContext, condition: impl Fn(&mut VisualTestContext) -> bool)  // line 2158
fn col(name: &str, is_primary_key: bool) -> ColumnInfo  // line 2670
```

Canonical test skeleton (every test follows it):

```rust
#[gpui::test]
async fn my_test(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();                      // REQUIRED: real tokio work crosses threads
    let fake = Arc::new(FakeDatabaseClient::new());
    let client: Arc<dyn DatabaseClient> = fake.clone();
    let cx = cx.add_empty_window();                     // shadows as &mut VisualTestContext
    let view = cx.update(|window, cx| {
        TableDataView::new(client, "local".into(), table_ref(), false, window, cx)
    });
    wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;
    // act: view.update(cx, |view, cx| ...) or view.update_in(cx, |view, window, cx| ...)
    // wait for LoadState::Idle before counting fake.calls()
}
```

Representative existing tests:
- `table_view_loads_first_page` (2179): first page loads, `spec().limit == 100` from the
  page_size setting, `fetch_rows users` appears in `fake.calls()`.
- `sort_click_resets_offset_and_reloads` (2207): next_page to offset 100, `toggle_sort("name")`
  → Asc + offset 0 + ≥3 fetches. Waits `LoadState::Idle` between steps because a new
  `_data_task` aborts the in-flight fetch.
- `next_prev_page_updates_offset` (2256): offsets 0→100→0; prev at 0 does not refetch
  (compares fetch counts before/after).
- `structure_mode_fetches_structure_once` (2307): structure cached after one fetch.
- `load_error_is_surfaced` (2504): `FakeDatabaseClient::with_error("connection refused")`
  → `LoadState::Error` containing the message.
- `buffer_edits_change_pending_count` (2775): `row_key_for(0)` == `{columns:[id], values:[Some(1)]}`;
  set/null/PK-noop/add_row/delete_row/discard pending-count arithmetic.
- `commit_cell_edit_buffers_update` (2896): `begin_edit_cell(0,1)` → `field.set_text("Alicia")`
  → `commit_cell_edit` → buffered `EditCell::Value("Alicia")` under the RowKey.
- `finish_editing_on_page_change_commits_by_key` (3357): typed-but-uncommitted edit +
  `toggle_sort` → editor closed, edit committed keyed by RowKey. **Any new stage-3
  restart path needs the same coverage.**
- `save_applies_and_clears` (3017) / `save_error_keeps_buffer` (3067) /
  `edits_during_in_flight_save_are_ignored` (3499).

## 12. Stage-3 change map (delete / rework / keep)

DELETE outright:
- Fields `filter_builder_open`, `draft_column`, `draft_op`, `draft_value` + their `new()` init.
- `available_columns` (1580, unless reused for the funnel), `draft_apply_enabled` (1589),
  `apply_draft_filter` (1598), `render_filter_builder` (1693), `all_filter_ops` (60).
- The "+ Filter" button inside `render_filter_bar` (1643–1651).
- `spec: SelectSpec` field → replaced by `QueryState` (new module
  `crates/database_ui/src/query_state.rs`) + rendered SQL text; `fetch_rows` call in
  `reload_data` → `run_query(&table.database, &sql, cap)` with the 1000-row UI cap.
- After conversion, `fetch_rows` + its bind machinery in `database_client`
  (`database_client.rs:172`, `sql.rs`, `postgres.rs`, `fake.rs:152`) if no consumers remain.

REWORK:
- `render_header` (1437): sort button → header-click cycle + hover funnel popup.
- `render_footer` (1780): add page-size dropdown, "+ Row", Save/Discard, `N rows · M ms`, Refresh.
- `render_edit_toolbar` (1836): absorbed by the footer (keep the read-only-reason logic).
- `render_filter_chip` (1660): add click-to-edit popup and a sort chip.
- `render_data_cell` (1268): right-click context menu (Filter/Exclude/View value/Edit cell),
  monospace values, right-aligned numerics, NULL vs empty-string distinction (already
  correct: NULL renders muted italic via `Option::None`).
- `has_more`: from `RowsPage.has_more` (server probe) → `rows.len() == limit` on
  `QueryResult`; footer counter `1–100+`.

KEEP unchanged: the whole editing engine (sections 6), `Table`/`uniform_list`/`map_row`
grid skeleton, `open_table_tab` dedup, structure tab, eager structure load, task-field
lifecycle, action registration + `cell_editor_focused` Enter/Escape gating.
