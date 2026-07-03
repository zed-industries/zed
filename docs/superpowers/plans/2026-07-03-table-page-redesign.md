# Table Page SQL-First Redesign Implementation Plan (Stage 3)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rework the table data page to be SQL-first: a visible, editable SQL query is the source of truth; sorting/filtering/pagination rewrite its text; filters move to cell context menus and header funnel popovers; the grid gets visual polish.

**Architecture:** A new pure module `query_state.rs` holds `QueryState` (base = table or custom text + filters/sort/limit/offset) and deterministically renders it to SQL. `TableDataView` swaps its data path from `fetch_rows(SelectSpec)` to `run_query(rendered_sql)` and keeps an embedded auto-height SQL editor in sync with the state. Editing (stage 2) and MCP are untouched.

**Tech Stack:** Rust, GPUI, ui::Table, tokio-postgres (via existing `DatabaseClient`), existing `database_client`/`database_ui` crates.

**Spec:** `docs/superpowers/specs/2026-07-03-table-page-redesign-design.md`

**API references (read before implementing — they contain exact signatures, line numbers, and traps):**
- `docs/superpowers/plans/api-refs/stage3-table-data-view.md` — current view structure, edit-buffer invariants, test harness
- `docs/superpowers/plans/api-refs/stage3-sql-editor-embed.md` — embedding the SQL editor, RunQuery action, keymap
- `docs/superpowers/plans/api-refs/stage3-database-client.md` — trait/type definitions, fake client, fetch_rows call sites
- `docs/superpowers/plans/api-refs/stage3-grid-render.md` — ui::Table internals, zebra/hover/align/width recipes
- `docs/superpowers/plans/api-refs/stage3-context-menu-popover.md` — right-click menus, popovers with inputs
- `docs/superpowers/plans/api-refs/stage3-settings-keymap.md` — settings access, key contexts, actions

## Global Constraints

- Build/check zed only with `--features gpui_platform/runtime_shaders` (no Xcode/Metal on this machine).
- Lint with `./script/clippy -p <crate> --all-targets -- -D warnings`; format with `cargo fmt -p <crate>` before every commit.
- No `unwrap()`/`expect()` outside tests; no `let _ =` on fallible ops (use `?`, `.log_err()`, or explicit handling).
- Invariant (spec): **the SQL bar text always equals exactly the query that executes.**
- Invariant (stage 2): call `self.finish_editing(cx)` before ANY operation that changes the displayed row set (Run, sort, filter add/remove/edit, page next/prev, page-size change, Reset, refresh).
- Cell editing allowed only when: base table with PK (existing `compute_editable`) AND `query.base` is `QueryBase::Table` AND the SQL editor is not dirty.
- MCP crate (`database_mcp`), panel tree, SQL tab (`sql_query_view.rs`) behavior, and `apply_edits` are out of scope — do not modify (except the shared `UI_MAX_QUERY_ROWS` const move in Task 3).
- GPUI test timers: use `cx.background_executor().timer(...)`, never `smol::Timer::after`.
- Commits end with trailer: `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.

## File Map

| File | Change |
|---|---|
| `crates/database_client/src/database_client.rs` | Task 1: `FilterOp::IsNotNull`, `pub use sql::quote_ident`. Task 9: delete `SelectSpec`, `RowsPage`, `fetch_rows` |
| `crates/database_client/src/sql.rs` | Task 1: IsNotNull arm. Task 9: delete `build_select`/`BuiltSelect`/`escape_like` |
| `crates/database_client/src/fake.rs` | Task 1: result queue + run_query-only error. Task 9: drop `page`/`fetch_rows` |
| `crates/database_client/src/postgres.rs` | Task 9: delete `fetch_rows` impl |
| `crates/database_ui/src/query_state.rs` | Task 2: NEW — `QueryBase`, `QueryState`, `render_sql`, literal escaping, unit tests |
| `crates/database_ui/src/database_ui.rs` | Task 2: `mod query_state;`. Task 3: shared `UI_MAX_QUERY_ROWS` |
| `crates/database_ui/src/table_data_view.rs` | Tasks 3–8: rewire to run_query, SQL bar, chips/funnel, context menu, footer, polish |
| `crates/database_ui/src/sql_query_view.rs` | Task 3: import shared `UI_MAX_QUERY_ROWS` (delete local const) |
| `crates/ui/src/components/data_table.rs` | Task 8: opt-in `header_background(Hsla)` builder (small fork tweak) |
| `docs/superpowers/database-viewer-usage.md` | Task 9: document new UX |

---

### Task 1: `FilterOp::IsNotNull` + fake client test hooks

**Files:**
- Modify: `crates/database_client/src/database_client.rs` (FilterOp enum ~:67; crate root re-export)
- Modify: `crates/database_client/src/sql.rs` (WHERE builder match, ~:102-130)
- Modify: `crates/database_client/src/fake.rs`
- Modify: `crates/database_ui/src/table_data_view.rs` (`filter_op_label` :48, `all_filter_ops` :60, `draft_apply_enabled` :1591, `apply_draft_filter` :1605, `render_filter_chip` :1661 — keep exhaustive matches compiling; the draft/chip code is deleted later in Task 5, patch minimally now)

**Interfaces:**
- Produces: `FilterOp::IsNotNull` (renders as `IS NOT NULL`, needs no value); `database_client::quote_ident` re-export; `FakeDatabaseClient::push_query_result(QueryResult)` (FIFO queue consumed by `run_query`, falling back to `self.query_result` when empty); `FakeDatabaseClient::run_query_error: Mutex<Option<String>>` (set via `set_run_query_error(Option<String>)`, fails ONLY `run_query`, unlike `error` which fails everything including the eager `table_structure`).

- [ ] **Step 1: Failing unit tests** in `sql.rs` tests module (`build_select` still exists until Task 9) and `fake.rs` tests:

```rust
// sql.rs tests
#[test]
fn build_select_is_not_null() {
    let spec = SelectSpec {
        filters: vec![Filter { column: "notes".into(), op: FilterOp::IsNotNull, value: String::new() }],
        ..Default::default()
    };
    let built = build_select(&table_ref(), &spec, &columns());
    assert!(built.sql.contains(r#""notes" IS NOT NULL"#));
    assert!(built.params.is_empty());
}

// fake.rs tests
#[gpui::test] // or #[tokio::test]/smol block_on style matching existing fake tests
async fn fake_run_query_queue_and_error() {
    let mut fake = FakeDatabaseClient::new();
    fake.query_result = QueryResult { columns: vec!["a".into()], ..Default::default() };
    let fake = Arc::new(fake);
    fake.push_query_result(QueryResult { columns: vec!["first".into()], ..Default::default() });
    fake.push_query_result(QueryResult { columns: vec!["second".into()], ..Default::default() });
    assert_eq!(fake.run_query("db", "SELECT 1", 10).await.unwrap().columns, vec!["first"]);
    assert_eq!(fake.run_query("db", "SELECT 1", 10).await.unwrap().columns, vec!["second"]);
    // queue empty -> falls back to query_result
    assert_eq!(fake.run_query("db", "SELECT 1", 10).await.unwrap().columns, vec!["a"]);
    fake.set_run_query_error(Some("boom".into()));
    assert!(fake.run_query("db", "SELECT 1", 10).await.is_err());
    assert!(fake.table_structure(&table_ref()).await.is_ok()); // other methods unaffected
}
```

- [ ] **Step 2: Run to verify failure** — `cargo test -p database_client` fails to compile (no `IsNotNull`, no `push_query_result`).

- [ ] **Step 3: Implement.**
  - `database_client.rs`: add `IsNotNull` to `FilterOp`; add `pub use sql::quote_ident;` next to existing re-exports.
  - `sql.rs`: in the WHERE-building match add `FilterOp::IsNotNull => format!("{} IS NOT NULL", quote_ident(&filter.column))` (mirror the `IsNull` arm; no param).
  - `fake.rs`: add fields `queued_results: Mutex<VecDeque<QueryResult>>` and `run_query_error: Mutex<Option<String>>` (both default empty; keep plain-struct construction working by adding them in `new()`); methods:

```rust
pub fn push_query_result(&self, result: QueryResult) {
    if let Ok(mut queue) = self.queued_results.lock() {
        queue.push_back(result);
    }
}
pub fn set_run_query_error(&self, error: Option<String>) {
    if let Ok(mut slot) = self.run_query_error.lock() {
        *slot = error;
    }
}
```

  and in `run_query` (after `check_error()` + `record(...)`):

```rust
if let Ok(slot) = self.run_query_error.lock()
    && let Some(message) = slot.as_ref()
{
    return Err(anyhow!("{message}"));
}
if let Ok(mut queue) = self.queued_results.lock()
    && let Some(result) = queue.pop_front()
{
    return Ok(result);
}
Ok(self.query_result.clone())
```

  - `table_data_view.rs` minimal exhaustive-match patches: `filter_op_label` → `FilterOp::IsNotNull => "is not null"`; `all_filter_ops()` → return `[FilterOp; 7]` including it; value-not-needed checks at :1591/:1605 → treat `IsNull | IsNotNull` alike (`!matches!(op, FilterOp::IsNull | FilterOp::IsNotNull)`); chip text at :1661 → same `matches!` for the no-value rendering; update the `filter_op_labels_cover_all_ops` test.

- [ ] **Step 4: Verify** — `cargo test -p database_client && cargo test -p database_ui` PASS.
- [ ] **Step 5: Format, lint, commit** — `cargo fmt -p database_client -p database_ui && ./script/clippy -p database_client -p database_ui --all-targets -- -D warnings`; commit `database_client: Add IS NOT NULL filter operator and fake run_query test hooks`.

---

### Task 2: `query_state.rs` — state → SQL rendering (pure)

**Files:**
- Create: `crates/database_ui/src/query_state.rs`
- Modify: `crates/database_ui/src/database_ui.rs` (add `mod query_state;`)

**Interfaces:**
- Consumes: `database_client::{Filter, FilterOp, Sort, SortDirection, TableRef, quote_ident}`.
- Produces (used by Tasks 3–7):

```rust
pub enum QueryBase { Table(TableRef), Custom(String) }
pub struct QueryState {
    pub base: QueryBase,
    pub filters: Vec<Filter>,
    pub sort: Option<Sort>,
    pub limit: Option<usize>, // always Some for Table base; None for a fresh custom query
    pub offset: usize,
}
impl QueryState {
    pub fn for_table(table: TableRef, page_size: usize) -> Self; // filters/sort empty, limit Some(page_size), offset 0
    pub fn for_custom(text: String) -> Self;                     // filters/sort empty, limit None, offset 0
    pub fn is_custom(&self) -> bool;
    pub fn has_overlay(&self) -> bool; // !filters.is_empty() || sort.is_some() || limit.is_some() || offset > 0
}
pub fn render_sql(state: &QueryState) -> String;
pub fn escape_literal(value: &str) -> String;      // 'it''s' style, always single-quoted
```

- [ ] **Step 1: Write the module with failing tests first.** Test cases (all plain `#[test]`, no GPUI):

```rust
fn orders() -> TableRef {
    TableRef { database: "shop".into(), schema: "public".into(), name: "orders".into() }
}

#[test]
fn renders_plain_table_query() {
    let state = QueryState::for_table(orders(), 100);
    assert_eq!(render_sql(&state), "SELECT * FROM \"public\".\"orders\" LIMIT 100 OFFSET 0;");
}

#[test]
fn renders_filters_sort_and_paging() {
    let mut state = QueryState::for_table(orders(), 50);
    state.filters = vec![
        Filter { column: "status".into(), op: FilterOp::Eq, value: "active".into() },
        Filter { column: "total".into(), op: FilterOp::Gt, value: "10".into() },
    ];
    state.sort = Some(Sort { column: "total".into(), direction: SortDirection::Desc });
    state.offset = 100;
    assert_eq!(
        render_sql(&state),
        "SELECT * FROM \"public\".\"orders\" WHERE \"status\" = 'active' AND \"total\" > '10' \
         ORDER BY \"total\" DESC LIMIT 50 OFFSET 100;"
    );
}

#[test]
fn renders_all_operators() {
    // one filter per op; assert the WHERE fragment for each:
    // Eq => "c" = 'v'    NotEq => "c" <> 'v'    Gt => "c" > 'v'    Lt => "c" < 'v'
    // Contains => "c"::text ILIKE '%v%'   (with % _ \ in v escaped by a backslash)
    // IsNull => "c" IS NULL               IsNotNull => "c" IS NOT NULL
}

#[test]
fn escapes_quotes_in_literals_and_idents() {
    let mut state = QueryState::for_table(orders(), 10);
    state.filters = vec![Filter { column: "we\"ird".into(), op: FilterOp::Eq, value: "it's".into() }];
    let sql = render_sql(&state);
    assert!(sql.contains("\"we\"\"ird\" = 'it''s'"));
}

#[test]
fn escapes_like_metacharacters_in_contains() {
    let mut state = QueryState::for_table(orders(), 10);
    state.filters = vec![Filter { column: "notes".into(), op: FilterOp::Contains, value: "50%_a\\b".into() }];
    assert!(render_sql(&state).contains(r#""notes"::text ILIKE '%50\%\_a\\b%'"#));
}

#[test]
fn custom_without_overlay_is_verbatim() {
    let state = QueryState::for_custom("SELECT 1;".into());
    assert_eq!(render_sql(&state), "SELECT 1;");
}

#[test]
fn custom_with_overlay_wraps_subquery_and_strips_trailing_semicolon() {
    let mut state = QueryState::for_custom("SELECT o.id FROM orders o;  ".into());
    state.sort = Some(Sort { column: "id".into(), direction: SortDirection::Asc });
    state.limit = Some(100);
    state.offset = 200;
    assert_eq!(
        render_sql(&state),
        "SELECT * FROM (\nSELECT o.id FROM orders o\n) AS zed_sub \
         ORDER BY \"id\" ASC LIMIT 100 OFFSET 200;"
    );
}

#[test]
fn custom_with_only_paging_wraps() {
    let mut state = QueryState::for_custom("SELECT 1".into());
    state.limit = Some(100);
    assert_eq!(render_sql(&state), "SELECT * FROM (\nSELECT 1\n) AS zed_sub LIMIT 100 OFFSET 0;");
}
```

- [ ] **Step 2: Run to verify failure** — `cargo test -p database_ui query_state` (module missing → compile error).

- [ ] **Step 3: Implement** (complete rendering core):

```rust
pub fn escape_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn escape_like_pattern(value: &str) -> String {
    value.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")
}

fn filter_fragment(filter: &Filter) -> String {
    let column = quote_ident(&filter.column);
    match filter.op {
        FilterOp::Eq => format!("{column} = {}", escape_literal(&filter.value)),
        FilterOp::NotEq => format!("{column} <> {}", escape_literal(&filter.value)),
        FilterOp::Gt => format!("{column} > {}", escape_literal(&filter.value)),
        FilterOp::Lt => format!("{column} < {}", escape_literal(&filter.value)),
        FilterOp::Contains => format!(
            "{column}::text ILIKE {}",
            escape_literal(&format!("%{}%", escape_like_pattern(&filter.value)))
        ),
        FilterOp::IsNull => format!("{column} IS NULL"),
        FilterOp::IsNotNull => format!("{column} IS NOT NULL"),
    }
}

fn overlay_clauses(state: &QueryState) -> String {
    let mut sql = String::new();
    if !state.filters.is_empty() {
        let predicates: Vec<String> = state.filters.iter().map(filter_fragment).collect();
        sql.push_str(" WHERE ");
        sql.push_str(&predicates.join(" AND "));
    }
    if let Some(sort) = &state.sort {
        let direction = match sort.direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        sql.push_str(&format!(" ORDER BY {} {}", quote_ident(&sort.column), direction));
    }
    if let Some(limit) = state.limit {
        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, state.offset));
    }
    sql
}

pub fn render_sql(state: &QueryState) -> String {
    match &state.base {
        QueryBase::Table(table) => format!(
            "SELECT * FROM {}.{}{};",
            quote_ident(&table.schema),
            quote_ident(&table.name),
            overlay_clauses(state)
        ),
        QueryBase::Custom(text) => {
            if !state.has_overlay() {
                return text.clone();
            }
            let inner = text.trim().trim_end_matches(';').trim_end();
            format!("SELECT * FROM (\n{inner}\n) AS zed_sub{};", overlay_clauses(state))
        }
    }
}
```

  (Adjust test expectations vs. spacing so they match exactly — the tests above assume a single space before WHERE/ORDER BY/LIMIT.)

- [ ] **Step 4: Verify** — `cargo test -p database_ui query_state` PASS.
- [ ] **Step 5: Format, lint, commit** — `database_ui: Add QueryState SQL rendering module`.

---

### Task 3: Rewire the data path to `run_query` (no visible SQL bar yet)

The pivotal internal change: `TableDataView` stops using `fetch_rows`/`SelectSpec`; every reload renders `QueryState` to SQL and runs it. UI behavior stays identical; existing tests are updated to assert generated SQL.

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`
- Modify: `crates/database_ui/src/database_ui.rs` (add `pub(crate) const UI_MAX_QUERY_ROWS: usize = 1000;`)
- Modify: `crates/database_ui/src/sql_query_view.rs` (delete local `UI_MAX_QUERY_ROWS`, import the shared one)

**Interfaces:**
- Consumes: `query_state::{QueryBase, QueryState, render_sql}`; `FakeDatabaseClient::push_query_result`.
- Produces (later tasks rely on):
  - field `query: QueryState` replacing `spec: SelectSpec`;
  - local `struct PageData { columns: Vec<String>, rows: Vec<Vec<Option<String>>>, has_more: bool }` replacing `Arc<RowsPage>` in `page`;
  - `fn current_sql(&self) -> String` (= `render_sql(&self.query)`);
  - `fn restart_query(&mut self, window: &mut Window, cx: &mut Context<Self>)` — the single reload entry point: `finish_editing` → recompute text (no-op until Task 4 wires the editor) → spawn run;
  - `last_run: Option<(usize, std::time::Duration)>` (row count + wall time) for the Task 7 footer;
  - mutators now take `window`: `toggle_sort(&mut self, column: &str, window, cx)`, `add_filter(&mut self, filter: Filter, window, cx)`, `remove_filter(&mut self, index: usize, window, cx)`, `next_page/prev_page(&mut self, window, cx)`.

**Key implementation notes:**
- `reload_data` body becomes (pattern from `sql_query_view.rs:155`, api-ref stage3-sql-editor-embed):

```rust
let sql = self.current_sql();
let database = self.table.database.clone();
let client = self.client.clone();
let limit = self.query.limit;
self.load_state = LoadState::Loading;
self._data_task = Some(cx.spawn_in(window, async move |this, cx| {
    let started = std::time::Instant::now();
    let result = cx
        .update(|_, cx| {
            gpui_tokio::Tokio::spawn_result(cx, async move {
                client.run_query(&database, &sql, crate::UI_MAX_QUERY_ROWS).await
            })
        })?
        .await;
    let elapsed = started.elapsed();
    this.update_in(cx, |this, window, cx| {
        match result {
            Ok(result) => {
                let has_more = result.truncated
                    || limit.is_some_and(|limit| result.rows.len() == limit);
                let row_count = result.rows.len();
                this.page = Some(PageData { columns: result.columns, rows: result.rows, has_more });
                this.last_run = Some((row_count, elapsed));
                this.set_column_widths(window, cx); // signature per current code; adjusted in Task 8
                this.load_state = LoadState::Idle;
            }
            Err(error) => { /* existing error path: LoadState::Failed(message) */ }
        }
        cx.notify();
    })
}));
```

- `has_more` heuristic replaces the server-side limit+1 probe (spec: no hidden `n+1`; the executed LIMIT is exactly what the text says).
- Mutator semantics preserved: sort/filter changes reset `offset = 0`; paging keeps filters; ALL call `finish_editing(cx)` first (they already do — keep those lines).
- `next_page` in custom mode with `limit == None`: set `limit = Some(page_size)` (from `DatabaseSettings::get_global(cx).page_size.max(1) as usize`), `offset = limit`.
- Update every test that asserted `fetch_rows` call strings: fake records `"run_query {db} max_rows={n} sql={sql}"`; assert the full generated SQL, e.g.:

```rust
let calls = fake.calls();
let last = calls.iter().rev().find(|call| call.starts_with("run_query")).unwrap();
assert!(last.ends_with(r#"sql=SELECT * FROM "public"."orders" ORDER BY "total" DESC LIMIT 100 OFFSET 0;"#));
```

- Seed fakes with `query_result`/`push_query_result` instead of `page`. A page-size-worth of rows must be generated to test `has_more == true` (`rows.len() == limit`).
- Do NOT delete `fetch_rows`/`SelectSpec` from `database_client` yet (Task 9); after this task `database_ui` simply no longer calls them.

- [ ] **Step 1:** Update/extend tests first (they define the new call shape): pagination offset rewrite, sort regeneration, filter add/remove, has_more heuristic (exactly `limit` rows → `has_more`, fewer → not), error path, `finish_editing`-before-reload regression test still green.
- [ ] **Step 2:** `cargo test -p database_ui` — expect failures/compile errors.
- [ ] **Step 3:** Implement the rewiring (struct fields, PageData, restart_query, mutators, reload body, UI_MAX_QUERY_ROWS move).
- [ ] **Step 4:** `cargo test -p database_ui` PASS; `cargo check -p zed --features gpui_platform/runtime_shaders` PASS.
- [ ] **Step 5:** Format, lint, commit — `database_ui: Drive table page data loading through rendered SQL`.

---

### Task 4: SQL bar UI + custom-mode state machine

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`

**Interfaces:**
- Consumes: `restart_query`, `current_sql`, `QueryBase`, `render_sql`; editor embedding recipe from api-ref stage3-sql-editor-embed (AutoHeight editor, SQL language, BufferEdited subscription, `set_text` needs `&mut Window`).
- Produces: fields `sql_editor: Entity<Editor>`, `sql_bar_collapsed: bool`, `sql_dirty: bool`, `suppress_editor_events: bool`, `_editor_subscription: Subscription`; methods `sync_editor_text(window, cx)` (set_text(render_sql) under suppress guard, clears `sql_dirty`), `run_from_editor(window, cx)`, `reset_to_table_query(window, cx)`; `render_sql_bar(window, cx)`. Editability gate: existing `editable` computation additionally requires `!self.sql_dirty && matches!(self.query.base, QueryBase::Table(_))`.

**Key implementation notes:**
- Editor creation (in `TableDataView::new`, which has `window`):

```rust
let sql_editor = cx.new(|cx| {
    let buffer = cx.new(|cx| language::Buffer::local(initial_sql.clone(), cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let mut editor = Editor::new(
        EditorMode::AutoHeight { min_lines: 1, max_lines: Some(5) },
        buffer, None, window, cx,
    );
    editor.set_show_gutter(false, cx);
    editor
});
```

  plus the detached `cx.spawn` that resolves `language_for_name("SQL")` and sets it on the buffer — copy the exact pattern from `sql_query_view.rs:96-111` (fallback: plain text, `.ok()` — never fail the view).
- Dirty tracking: `cx.subscribe(&sql_editor, |this, _, event, cx| ...)` on `EditorEvent::BufferEdited`; ignore when `suppress_editor_events` (set around programmatic `set_text` — it fires on those too). On a real edit: `sql_dirty = editor_text != render_sql(&self.query)` (an undo back to the rendered text clears dirtiness), `cx.notify()`.
- `run_from_editor`: `finish_editing`; read `let text = self.sql_editor.read(cx).text(cx);` — if `text == self.current_sql()` → plain `restart_query` (acts as refresh). Else → `self.query = QueryState::for_custom(text)`; `sql_dirty = false`; `restart_query`. (For a custom base, `render_sql` returns the text verbatim — invariant holds without a `set_text`.)
- `reset_to_table_query`: rebuild `QueryState::for_table(self.table.clone(), page_size)`, `sync_editor_text`, `restart_query`.
- `restart_query` (from Task 3) now calls `sync_editor_text(window, cx)` before spawning, so every UI mutation (sort/filter/page) rewrites the visible text — including the wrapped `zed_sub` form in custom mode.
- Action wiring: wrap the SQL bar in `div().key_context("SqlQueryEditor").on_action(cx.listener(Self::handle_run_query))` — reusing the existing `"SqlQueryEditor > Editor"` keymap block means **zero keymap changes** (cmd-enter → `database::RunQuery`, enter → newline already bound on all three platforms). `handle_run_query(&mut self, _: &RunQuery, window, cx)` → `run_from_editor`. Import `RunQuery` from `sql_query_view` (make the action type `pub(crate)` there if not already).
- Trap (api-ref stage3-table-data-view): Enter/Escape reach the view as `menu::Confirm/Cancel` gated by `cell_editor_focused` — the gate must NOT swallow them while the SQL editor is focused (editor handles its own keys; verify the existing gating checks the cell editor's focus handle specifically, which it does — no change expected, add a test).
- `render_sql_bar`: `v_flex` row under the header bar: collapse chevron (`IconButton` ChevronDown/ChevronRight toggling `sql_bar_collapsed`; collapsed → render only the chevron row), the editor (`div().flex_1().child(self.sql_editor.clone())`), Run button (`ButtonStyle::Filled`, label "Run", `.key_binding(...)` optional), and when `self.query.is_custom()`: a warning-colored `Label::new("Custom query · read-only")` badge + `Button::new("db-reset-query", "Reset to table query")` → `reset_to_table_query`.
- Custom-mode overlay mutations: `toggle_sort`/`add_filter`/paging work unchanged — they mutate `self.query` and `restart_query` regenerates the wrapped text (visible per invariant). Manual re-edit of the wrapped text + Run → new custom base with overlay reset (`for_custom` clears filters/sort/limit/offset) — this is the spec's "повторная ручная правка".

**Tests (GPUI, fake client) — write first:**

```rust
#[gpui::test]
async fn run_dirty_text_enters_custom_read_only_mode(cx: &mut TestAppContext) { /*
    open table view, wait Idle; set editor text to "SELECT 1" via
    view.update_in(|this, window, cx| this.sql_editor.update(cx, |e, cx| e.set_text("SELECT 1", window, cx)));
    assert this.sql_dirty && !this.editable(cx);
    dispatch run_from_editor; wait Idle;
    assert matches!(query.base, QueryBase::Custom(_)), filters cleared, badge state, editable == false,
    fake last run_query sql == "SELECT 1" */ }

#[gpui::test]
async fn ui_sort_in_custom_mode_wraps_subquery(cx: &mut TestAppContext) { /*
    enter custom mode with "SELECT a FROM t"; toggle_sort("a");
    assert editor text == render_sql(query) == "SELECT * FROM (\nSELECT a FROM t\n) AS zed_sub ORDER BY \"a\" ASC ...;"
    and fake received the same sql */ }

#[gpui::test]
async fn reset_returns_to_generated_table_query(cx: &mut TestAppContext) { /* custom → reset → base Table,
    editor text == plain SELECT * FROM ... LIMIT ... OFFSET 0;, editable restored */ }

#[gpui::test]
async fn programmatic_sync_does_not_mark_dirty(cx: &mut TestAppContext) { /* toggle_sort on table base →
    text changed but sql_dirty == false */ }

#[gpui::test]
async fn run_commits_open_cell_editor_first(cx: &mut TestAppContext) { /* begin_edit_cell, type, run_from_editor →
    edit buffered under correct RowKey (finish_editing invariant) */ }
```

- [ ] **Step 1:** Write the tests above (adapt to the existing `init_test`/`wait_until` harness, `table_data_view.rs:2133+`).
- [ ] **Step 2:** `cargo test -p database_ui` — fail.
- [ ] **Step 3:** Implement fields, editor, subscription, sync/run/reset, render_sql_bar, editability gate, render stack (header / sql bar / filter chips / body / footer).
- [ ] **Step 4:** `cargo test -p database_ui` PASS; `cargo check -p zed --features gpui_platform/runtime_shaders` PASS.
- [ ] **Step 5:** Format, lint, commit — `database_ui: Add editable SQL bar with custom-query mode to table page`.

---

### Task 5: Header sort/funnel + FilterPopover + chips row; delete old filter builder

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`

**Interfaces:**
- Consumes: `apply_filter_edit` (new, below), `toggle_sort(column, window, cx)`, chips read `self.query.filters`/`self.query.sort`; popover recipe from api-ref stage3-context-menu-popover §3.1 (verbatim `FilterPopover` skeleton, `PopoverMenu` + `ManagedView`, focus-delegation-to-input, `menu::Confirm`/`Cancel`, mouse-down-out).
- Produces:
  - `struct FilterPopover` (entity; fields `column: String`, `op: FilterOp`, `value_field: Entity<InputField>`, `existing_index: Option<usize>`, `table_view: WeakEntity<TableDataView>`);
  - `TableDataView::apply_filter_edit(&mut self, index: Option<usize>, filter: Filter, window, cx)` — `Some(i)` replaces `query.filters[i]` (guard `i < len`), `None` pushes; resets `offset = 0`; `finish_editing`; `restart_query`;
  - `render_chips_row(cx)` (replaces `render_filter_bar`), `render_header` rework.

**Key implementation notes:**
- Header cell (recipe api-ref stage3-grid-render §4): per-column hover group; content = clickable label area cycling sort via `toggle_sort` + sort `Icon` (ArrowUp/ArrowDown) when sorted; funnel `IconButton` with `.visible_on_hover(group)` — but rendered always-visible with `.toggle_state(true)` when that column has a filter. Funnel wrapped in `PopoverMenu::new(("db-col-filter", index))` whose `.menu(...)` builds `FilterPopover::new(column, existing_filter_and_index, weak, window, cx)`.
- Guard the double-click width-reset trap: sort click handler on the label element (not the whole cell), and ignore `event.click_count() > 1` if it proves confusing (test single-click behavior only).
- Chips row: filter chips labelled like today (`filter_op_label` + value; no value shown for IsNull/IsNotNull); each chip is a `Button` acting as PopoverMenu trigger opening the prefilled `FilterPopover` (`existing_index = Some(i)`); `×` `IconButton` calls `remove_filter(i, window, cx)`. Sort chip: `"{column} {asc|desc}"` with `×` clearing `query.sort` (+ offset reset, finish_editing, restart). Row hidden when no chips.
- FilterPopover operator row: flat toggle `Button`s for all 7 ops (trap 7.4: no nested dropdown inside popover); value `InputField` hidden for IsNull/IsNotNull; Enter (`menu::Confirm`) applies; the popover reads dirty value via `value_field.read(cx).text(cx)`.
- DELETE: `filter_builder_open`, `draft_column`, `draft_op`, `draft_value` fields; `render_filter_builder`, `apply_draft_filter`, `draft_apply_enabled`, `available_columns` (if unused after); the "+ Filter" button; their tests. The old sort `Button` in headers is replaced by the new header layout.

**Tests — write first:** popover Apply adds filter + SQL regenerated + offset reset; Apply with `existing_index` replaces not appends; chip × removes; IsNull from popover needs no value; sort chip clears sort; header funnel toggled state reflects active filter; old-builder tests removed.

- [ ] **Step 1:** Tests. **Step 2:** fail. **Step 3:** implement. **Step 4:** `cargo test -p database_ui` + `cargo check -p zed --features gpui_platform/runtime_shaders` PASS. **Step 5:** fmt/clippy/commit — `database_ui: Replace filter builder with header funnels, filter popover, and chips`.

---

### Task 6: Cell context menu (right click) + View value popover

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`

**Interfaces:**
- Consumes: `apply_filter_edit(None, filter, window, cx)`, `begin_edit_cell`, editability gate; right-click recipe from api-ref stage3-context-menu-popover §1 (state tuple field, `on_secondary_mouse_down` + `stop_propagation`, `ContextMenu::build` with `entry` closures, `deferred(anchored().position(p)).with_priority(3)`, DismissEvent subscription) and §3.2 (`ValuePopover` manual pattern).
- Produces: fields `context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>`, `value_popover: Option<(Entity<ValuePopover>, Point<Pixels>, Subscription)>`; `deploy_cell_context_menu(&mut self, position, column: String, value: Option<String>, row_target: Option<EditTarget>, window, cx)`; `struct ValuePopover { value: SharedString, focus_handle: FocusHandle }`; `open_value_popover(&mut self, position, value: String, window, cx)`.

**Key implementation notes:**
- In `render_data_cell`, capture `column` name and the cell's `Option<String>` value (clone) plus the row's `EditTarget` **at render time** (stage-2 lesson: never resolve by index at click time) and attach `.on_secondary_mouse_down(cx.listener(move |this, event: &MouseDownEvent, window, cx| { cx.stop_propagation(); this.deploy_cell_context_menu(event.position, column.clone(), value.clone(), target.clone(), window, cx); }))`.
- Menu entries via `menu.entry(label, None, closure)` with `cx.weak_entity()` (no actions/`menu.context` needed — same style as existing filter dropdowns):
  - non-NULL cell: `Filter: {col} = '{v}'` → `apply_filter_edit(None, Filter { column, op: Eq, value }, ...)`; `Exclude: {col} ≠ '{v}'` → NotEq;
  - NULL cell: `Filter: {col} IS NULL` → IsNull; `Exclude: {col} IS NOT NULL` → IsNotNull (empty value);
  - separator; `View value` → `open_value_popover` (label shows full text; for NULL show `NULL`);
  - `Edit cell` — only pushed when the view is editable and the cell's column is not a PK (same rule as double-click today) → `begin_edit_cell(target, column, window, cx)` (match existing signature).
- Truncate long values in menu labels to ~40 chars with `…` (labels only; the popover shows everything).
- `ValuePopover`: exactly api-ref §3.2 (occlude, elevation_2, max_w_96/max_h_80, scrollable inner, Esc + mouse-down-out dismiss); rendered with the same `deferred(anchored().position(p)).with_priority(3)` children hook as the context menu, appended to the root element in `render` next to the context-menu hook.
- Both hooks appended to the existing root `v_flex` in `impl Render` (pattern `database_panel.rs:851-859`).

**Tests — write first:** deploy on value cell → menu entries apply Eq/NotEq filters with the exact captured value (assert regenerated SQL contains `= 'v'`); NULL cell → IsNull/IsNotNull; View value sets `value_popover` with full text; Edit cell entry absent when read-only (custom mode) and for PK columns; DismissEvent clears the field. (Call `deploy_cell_context_menu` directly in tests — synthesizing right-clicks is unnecessary.)

- [ ] **Step 1:** Tests. **Step 2:** fail. **Step 3:** implement. **Step 4:** `cargo test -p database_ui` + zed check PASS. **Step 5:** fmt/clippy/commit — `database_ui: Add cell context menu with quick filters and value popover`.

---

### Task 7: Footer — pagination, page size, edit controls, timing

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`

**Interfaces:**
- Consumes: `next_page/prev_page`, `last_run`, `has_more` (from `page`), edit-toolbar pieces (`render_edit_toolbar` :1836 — Save/Discard/`N changes`/`+ Row` move here), `finish_editing`.
- Produces: `set_page_size(&mut self, page_size: usize, window, cx)` (`query.limit = Some(page_size)`, `offset = 0`, finish_editing, restart_query); reworked `render_footer(window, cx)`; `render_edit_toolbar` deleted (its contents inlined).

**Footer layout (left→right):** `IconButton` ChevronLeft (disabled at `offset == 0`) · ChevronRight (disabled unless `has_more`) · counter `Label` · page-size `PopoverMenu` (trigger Button `"{limit} / page"`; `ContextMenu` entries `100`, `500`, `1000`, plus the settings default when it differs — build the option list as `let mut sizes = vec![100, 500, 1000]; if !sizes.contains(&default) { sizes.push(default); sizes.sort_unstable(); }`) · `+ Row` button (existing `add_row` handler, visible when editable) · `N changes` + Save/Discard (existing handlers/enabled logic, visible when edit buffer non-empty or saving) · right-aligned: `Label` `"{count} rows · {ms} ms"` from `last_run` · Refresh `IconButton` (`restart_query`).
- Counter: table base or custom-with-limit → `rows {offset+1}–{offset+len}{"+" if has_more}` (empty page → `No rows`); custom with `limit == None` → `{len} rows`.
- Page-size trigger hidden when `limit == None` (fresh custom query).
- The old header-bar Refresh button and `render_edit_toolbar` row are removed; `render()` stack becomes: header bar / sql bar / chips row / body / footer.

**Tests — write first:** `set_page_size(500)` regenerates SQL with `LIMIT 500 OFFSET 0` (offset was 200 before); next/prev enablement mirrors `offset`/`has_more`; page-size change commits an open cell editor (finish_editing); Save/Discard still function after relocation (existing save-cycle tests keep passing — update element ids only if they assert on them); counter formats (`rows 1–100+`, `No rows`, custom `N rows`).

- [ ] **Step 1:** Tests. **Step 2:** fail. **Step 3:** implement. **Step 4:** `cargo test -p database_ui` + zed check PASS. **Step 5:** fmt/clippy/commit — `database_ui: Rework table footer with page size picker and query timing`.

---

### Task 8: Visual polish — mono font, numeric right-align, auto column widths, header background

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`
- Modify: `crates/ui/src/components/data_table.rs` (opt-in header background)

**Interfaces:**
- Consumes: recipes from api-ref stage3-grid-render §2–5 (zebra/hover already active via `.striped()` + default row hover — no work needed; `text_right`, `font_buffer`, `em_advance` measuring, `ResizableColumnsState::new` semantics + the "recreate only on column-count change" guard in `set_column_widths` :1094).
- Produces:
  - `ui::Table::header_background(mut self, color: Hsla) -> Self` — new builder storing `Option<Hsla>`, applied as `.bg(color)` on the header row div in `render_table_header` (data_table.rs:707-822, next to the existing `.border_b_1()`);
  - `numeric_columns: HashSet<String>` field on TableDataView, computed where the structure lands: udt_name ∈ {`int2`,`int4`,`int8`,`numeric`,`float4`,`float8`,`money`,`oid`};
  - `fn measured_column_widths(&self, page: &PageData, window: &Window, cx: &App) -> Option<Vec<Pixels>>` — buffer font + `em_advance` (fall back to `px(8.)` per char on Err — no unwrap), `max_char_count` = max of header `chars().count()` and each value's count (NULL = 4) over the first 100 rows, width = `advance * chars + 8. + 12.` clamped to `60.0..=480.0`.

**Key implementation notes:**
- Mono font: in `render_data_cell` / `render_insert_cell` / `render_cell_editor`'s display path add `.font_buffer(cx)` on the cell's inner div (header and the rest of the page stay UI font). Right-align: `.text_right()` on the same div when `self.numeric_columns.contains(column_name)` AND `matches!(query.base, QueryBase::Table(_))`.
- Auto width: rework `set_column_widths` to take `window` and, when it (re)creates the `ResizableColumnsState` (column count changed or first page), pass measured widths instead of the flat `COLUMN_WIDTH`; when the count is unchanged, keep the existing entity untouched (preserves user resizes; double-click header resets to measured initial). Delete `COLUMN_WIDTH` if unused.
- Header background: `Table::header_background(cx.theme().colors().title_bar_background)` in `render_data` (and optionally the structure table for consistency).
- Editor row in a cell being edited must keep its `h_flex` layout untouched (trap 8 in the api-ref).

**Tests — write first:** pure-fn test for the width math via a extracted helper `fn column_width_for_chars(advance: Pixels, chars: usize) -> Pixels` (clamps at 60/480); numeric-column set derived from fake structure (`int4`/`numeric` → right-aligned set contains them, `text` not); GPUI smoke: page load with long values produces per-column widths ≠ flat default (assert via the entity's `cols()` + behavior not panicking; exact widths via the helper test); custom mode → right-align set unused.

- [ ] **Step 1:** Tests. **Step 2:** fail. **Step 3:** implement (ui fork tweak first, then view changes). **Step 4:** `cargo test -p database_ui` + `./script/clippy -p ui -p database_ui --all-targets -- -D warnings` + zed check PASS. **Step 5:** fmt/commit — `database_ui: Polish data grid with mono font, numeric alignment, and auto column widths`.

---

### Task 9: Remove the dead fetch_rows path, docs, live verification

**Files:**
- Modify: `crates/database_client/src/database_client.rs` (delete `SelectSpec`, `RowsPage`, `fetch_rows` from trait; keep `Sort`/`SortDirection`/`Filter`/`FilterOp` — query_state uses them)
- Modify: `crates/database_client/src/sql.rs` (delete `build_select`, `BuiltSelect`, `escape_like` + their tests; KEEP `quote_ident`, `param_cast`, `build_key_predicate`, `build_update/insert/delete` — apply_edits path)
- Modify: `crates/database_client/src/postgres.rs` (delete `fetch_rows` impl)
- Modify: `crates/database_client/src/fake.rs` (delete `page` field + `fetch_rows` impl)
- Modify: `docs/superpowers/database-viewer-usage.md`

- [ ] **Step 1:** `grep -rn "fetch_rows\|SelectSpec\|RowsPage\|build_select\|escape_like" crates/` — confirm the only remaining references are the ones being deleted in this task (Task 3 removed all `database_ui` consumers). If anything else surfaces, stop and fix the consumer first.
- [ ] **Step 2:** Delete the listed items; fix imports/re-exports at the crate root.
- [ ] **Step 3:** Full verification:

```bash
cargo test -p database_client -p database_ui
./script/clippy --workspace --all-targets -- -D warnings   # or per-crate if workspace-wide is slow: database_client, database_ui, ui, zed
cargo check -p zed --features gpui_platform/runtime_shaders
cargo build -p database_mcp   # MCP must be untouched and still build
```

- [ ] **Step 4: Live check against Docker Postgres** (container `zed-db-test`, db `shop` — start it if stopped: `docker start zed-db-test`). Verify representative generated SQL executes (these strings must match `render_sql` output from the query_state tests):

```bash
docker exec zed-db-test psql -U postgres -d shop -c "SELECT * FROM \"public\".\"orders\" WHERE \"status\" = 'active' AND \"total\" > '10' ORDER BY \"total\" DESC LIMIT 100 OFFSET 0;"
docker exec zed-db-test psql -U postgres -d shop -c "SELECT * FROM (
SELECT o.id, o.total FROM orders o
) AS zed_sub ORDER BY \"total\" DESC LIMIT 5 OFFSET 2;"
docker exec zed-db-test psql -U postgres -d shop -c "SELECT * FROM \"public\".\"orders\" WHERE \"notes\"::text ILIKE '%50\%%' LIMIT 10 OFFSET 0;"
```

  Expected: all three return rows / empty sets without SQL errors (int/numeric columns accept quoted literals via implicit coercion).
- [ ] **Step 5:** Update `docs/superpowers/database-viewer-usage.md`: SQL bar (edit + ⌘⏎ Run, collapse, custom mode + Reset), right-click cell filters, header funnel, chips, page-size picker, footer relocation of + Row/Save/Discard. Keep it user-oriented and short.
- [ ] **Step 6:** fmt/clippy/commit — `database_client: Remove unused fetch_rows path` (+ docs in the same commit is fine).

---

## Execution notes for the controller

- Tasks are strictly ordered 1→9 (each builds on the previous one's interfaces).
- Task 3 and Task 4 are the risky ones (big rewiring / state machine): standard implementer + careful review; Tasks 1, 2, 9 are mechanical.
- Existing test suite (104 tests) must stay green throughout; several stage-1/2 tests assert `fetch_rows` call strings and old UI elements — updating them is part of Tasks 3/5/7, deleting builder tests is part of Task 5.
- Ledger: `.superpowers/sdd/progress.md` (append stage-3 lines; do not re-dispatch completed stage-1/2 tasks listed there).
