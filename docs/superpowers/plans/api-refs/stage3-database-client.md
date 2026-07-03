# database_client crate reference for stage 3 (SQL-first table page)

Everything an engineer needs from `crates/database_client` to (a) route the table page
through `run_query`, (b) delete `fetch_rows` and its bind machinery, (c) mock
`run_query` in GPUI tests, and (d) add `FilterOp::IsNotNull`. All paths absolute,
line numbers as of branch `database-viewer` (2026-07-03).

Crate layout: lib root `/Users/user/zed/crates/database_client/src/database_client.rs`
(`[lib] path`), modules `sql` (`pub mod sql;`, line 1), `postgres` (line 5, `pub use
postgres::SessionMode;` line 7), `fake` (line 3-4, gated `#[cfg(any(test, feature =
"test-support"))]`). `database_ui/Cargo.toml:35` already depends on
`database_client = { workspace = true, features = ["test-support"] }` in dev-deps, so
`database_client::fake::FakeDatabaseClient` is importable from `database_ui` tests as-is.

## 1. Core types (`database_client.rs`)

```rust
// database_client.rs:21-26
pub struct TableRef { pub database: String, pub schema: String, pub name: String }
// derives: Debug, Clone, PartialEq, Eq

// database_client.rs:34-43
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,  // information_schema data_type ("integer", "text", ...)
    pub udt_name: String,   // "int4", "text", ... — used to cast bound params
    pub udt_schema: String, // schema-qualifies the udt (types outside search_path)
    pub is_nullable: bool,
    pub default: Option<String>,
    pub is_primary_key: bool,
}

// database_client.rs:66-74  (Copy!)
pub enum FilterOp { Eq, NotEq, Gt, Lt, Contains, IsNull }

// database_client.rs:76-81
pub struct Filter { pub column: String, pub op: FilterOp, pub value: String } // value ignored for IsNull

// database_client.rs:83-93
pub enum SortDirection { Asc, Desc }                       // Copy
pub struct Sort { pub column: String, pub direction: SortDirection }

// database_client.rs:95-101 (Default)
pub struct SelectSpec {
    pub filters: Vec<Filter>,
    pub sort: Option<Sort>,
    pub limit: usize,
    pub offset: usize,
}

// database_client.rs:103-108 (Default)
pub struct RowsPage {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>, // all values as text; None = NULL
    pub has_more: bool,                 // true when the limit+1 probe row came back
}

// database_client.rs:110-116 (Default) — ALL FOUR FIELDS:
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>, // text values; None = NULL
    pub truncated: bool,                // rows dropped to respect max_rows
    pub command_tag: Option<String>,    // e.g. "SELECT 42"
}
```

Edit types (unchanged in stage 3, listed for completeness): `EditCell::{Value(String),
Null}` (120-124), `RowKey` (129-133, `Hash`), `RowUpdate` (135-139), `RowInsert`
(141-144), `RowDelete` (146-149), `TableEdits` (151-156), `AppliedCounts` (158-163).

### The trait — `database_client.rs:165-184`

```rust
#[async_trait::async_trait]
pub trait DatabaseClient: Send + Sync {
    async fn test_connection(&self) -> Result<()>;
    async fn list_databases(&self) -> Result<Vec<String>>;
    async fn list_schemas(&self, database: &str) -> Result<Vec<String>>;
    async fn list_tables(&self, database: &str, schema: &str) -> Result<Vec<TableInfo>>;
    async fn table_structure(&self, table: &TableRef) -> Result<TableStructure>;
    async fn fetch_rows(&self, table: &TableRef, spec: &SelectSpec) -> Result<RowsPage>; // line 172 — TO BE REMOVED
    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult>; // line 173
    async fn apply_edits(&self, table: &TableRef, columns: &[ColumnInfo], edits: &TableEdits) -> Result<AppliedCounts>;
    async fn cancel_running(&self) -> Result<()>;
}
```

`TableStructure` (59-64): `{ columns: Vec<ColumnInfo>, foreign_keys: Vec<ForeignKey>,
indexes: Vec<IndexInfo> }`, `Default`. UI holds clients as `Arc<dyn DatabaseClient>`.

### Adding `FilterOp::IsNotNull`

`FilterOp` is a plain exhaustive enum — adding a variant breaks these matches:
- `sql.rs:102-130` `build_select` filter match (moot if `build_select` is deleted first;
  otherwise add `FilterOp::IsNotNull => predicates.push(format!("{ident} IS NOT NULL"))`).
- `/Users/user/zed/crates/database_ui/src/table_data_view.rs:48-57` `filter_op_label`,
  `:60-68` `all_filter_ops() -> [FilterOp; 6]` (array length!), `:1591` and `:1605`
  draft-filter value gating (`draft_op != FilterOp::IsNull` — must become
  `!matches!(op, IsNull | IsNotNull)` equivalent).

## 2. `sql.rs` — public surface and what dies with `fetch_rows`

File `/Users/user/zed/crates/database_client/src/sql.rs`. Public items:

| Item | Line | Signature | Callers |
|---|---|---|---|
| `BuiltSelect` | 8-11 | `{ pub sql: String, pub params: Vec<String> }` | only `build_select` / `postgres::fetch_rows` |
| `BuiltStatement` | 13-16 | same shape | edit builders + `postgres::execute_statement` |
| `build_select` | 75-151 | `pub fn build_select(table: &TableRef, columns: &[ColumnInfo], spec: &SelectSpec) -> anyhow::Result<BuiltSelect>` | ONLY `postgres.rs:320` (`fetch_rows`) |
| `build_update` | 157-187 | `(table, columns, update: &RowUpdate) -> Result<BuiltStatement>` | `postgres.rs:506` (`execute_edits`) |
| `build_insert` | 195-227 | `(table, columns, insert: &RowInsert) -> Result<BuiltStatement>` | `postgres.rs:517` |
| `build_delete` | 231-245 | `(table, columns, delete: &RowDelete) -> Result<BuiltStatement>` | `postgres.rs:495` |
| `quote_ident` | 294-305 | `pub fn quote_ident(ident: &str) -> String` — wraps in `"`, doubles inner `"` | edit builders, `param_cast`; **no callers outside the crate yet** |
| `escape_like` | 308-317 | `pub fn escape_like(value: &str) -> String` — backslash-escapes `\ % _` | ONLY `build_select`'s `Contains` branch (sql.rs:105) |
| SQL consts | 259-291 | `LIST_DATABASES_SQL`, `LIST_SCHEMAS_SQL`, `LIST_TABLES_SQL`, `COLUMNS_SQL`, `FOREIGN_KEYS_SQL`, `INDEXES_SQL` | `postgres.rs` metadata methods |

Private helpers: `param_cast(column: &ColumnInfo, index: usize) -> String` (25-31),
renders `$N::text::"udt_schema"."udt_name"`; `find_column` (34-39);
`build_key_predicate(columns, key: &RowKey, params: &mut Vec<String>) -> Result<String>`
(43-73); `render_cell` (249-257).

**Dead-code map if the UI stops calling `fetch_rows`** (MCP never used it):
- Dies: trait method `fetch_rows`, `postgres.rs:318-350` impl, `fake.rs:152-163` impl,
  `sql::build_select` + `BuiltSelect` + `escape_like`, types `SelectSpec`, `RowsPage`
  (and `Sort` unless the UI keeps it for `QueryState`; `SortDirection` is worth keeping —
  `table_data_view.rs` uses it for header arrows), sql.rs tests at 353-501
  (`build_select_*`, `escape_like_escapes_metacharacters`), fake test
  `fake.rs:204-227`, postgres smoke-test sections using `fetch_rows`
  (`postgres.rs:686-772`).
- Survives (used by `apply_edits` path): `param_cast`, `find_column`,
  `build_key_predicate`, `render_cell`, `build_update/insert/delete`, `BuiltStatement`,
  `quote_ident`, all SQL consts, `postgres::execute_statement` (529-537 — the last
  remaining bind-parameter user).

**`quote_ident` availability for `database_ui/src/query_state.rs`**: it is `pub` inside
`pub mod sql`, NOT re-exported at the crate root. Import as
`use database_client::sql::quote_ident;`. If a root re-export is preferred, add
`pub use sql::quote_ident;` to `database_client.rs`.

## 3. `postgres.rs` — execution semantics

`/Users/user/zed/crates/database_client/src/postgres.rs`.

```rust
// postgres.rs:26-30
pub enum SessionMode { ReadWrite, ReadOnly }   // Copy; re-exported at crate root (database_client.rs:7)

// postgres.rs:71-86
pub fn new(config: ConnectionConfig, password: String,
           statement_timeout: Duration, mode: SessionMode) -> PostgresClient
```

Who constructs what: UI → `SessionMode::ReadWrite` via `default_client_factory()`
(`/Users/user/zed/crates/database_ui/src/connection_store.rs:546-557`, timeout read
fresh from `DatabaseSettings.query_timeout_seconds` at connect time); MCP →
`SessionMode::ReadOnly` (`/Users/user/zed/crates/database_mcp/src/main.rs:176`).

Session plumbing (shared by every method):
- One cached `tokio_postgres::Client` per database name, `client_for` (153-175);
  reconnects when closed. Connect timeout 10s (`CONNECT_TIMEOUT`, line 22).
- `statement_timeout` applied via startup options `-c statement_timeout=<ms>`
  (`session_options`, 541-549); `ReadOnly` additionally sets
  `-c default_transaction_read_only=on`.
- Cancellation: `register_cancel(&client) -> CancelGuard` (180-189) inserts a
  `CancelToken` into a shared map; `CancelGuard::drop` (62-68) removes it.
  `cancel_running` (456-479) drains the map and calls `token.cancel_query(NoTls)` on
  every in-flight query of this client. `run_query` registers a token (postgres.rs:370),
  so the table page inherits Cancel-button support for free after the switch.
  `apply_edits` deliberately does NOT register (comment at 423-428).

### `fetch_rows` (318-350) — the path being deleted

Extended protocol: re-queries `COLUMNS_SQL` on every call (`self.columns(table)`,
191-211), builds SQL via `sql::build_select`, binds `Vec<String>` params as
`&(dyn ToSql + Sync)` (324-328), runs `client.query(&built.sql, &param_refs)`.
`build_select` emits `LIMIT spec.limit + 1` (sql.rs:149); `has_more = rows.len() >
spec.limit` (334) and the probe row is dropped. Select list casts every column
`"col"::text` so values decode as `Option<String>`.

### `run_query` (368-396) — the new table-page path

```rust
async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult>
```

- `ReadWrite` (UI): `client.simple_query(sql).await.context("running query")` →
  `parse_query_messages(sql, messages, max_rows)`. **Simple protocol**: no bind
  parameters possible (QueryState must inline escaped literals — matches the spec),
  multi-statement text allowed, every value arrives as text.
- `ReadOnly` (MCP): wraps in `BEGIN READ ONLY` … `ROLLBACK` (always rolls back, even on
  success; 377-394). Irrelevant to the table page (UI session is ReadWrite).

`parse_query_messages` (559-613) traps:
- Multi-statement input: a new `RowDescription` **resets** accumulated rows/columns —
  only the LAST result set survives (571-581).
- `truncated` is set only when a row arrives **beyond** `max_rows` (590-593): a result
  with exactly `max_rows` rows has `truncated == false`. The spec's `has_more`
  heuristic (`rows.len() == limit`) is computed by the UI from the SQL LIMIT, not from
  `truncated`; `truncated` fires only at the UI cap (1000).
- `command_tag` is reconstructed as `"{command_verb(sql)} {count}"` (600-602,
  `command_verb` 617-623 uppercases the first token of the whole text — for
  `SELECT * FROM t LIMIT 100` it yields e.g. `Some("SELECT 100")`).
- No timing info: measure elapsed around the await, as `sql_query_view` does.

### Reference UI call pattern (copy this shape into the table page)

`/Users/user/zed/crates/database_ui/src/sql_query_view.rs:155-191`; cap constant
`UI_MAX_QUERY_ROWS: usize = 1000` at `sql_query_view.rs:32` (crate-private to
`database_ui` — reuse/move it for the table page):

```rust
let client = self.client.clone();       // Arc<dyn DatabaseClient>
let database = self.database.clone();
let task = gpui_tokio::Tokio::spawn_result(cx, async move {
    let started = Instant::now();
    let result = client.run_query(&database, &sql, UI_MAX_QUERY_ROWS).await;
    result.map(|result| (result, started.elapsed()))
});
self._run_task = Some(cx.spawn(async move |this, cx| {
    let outcome = task.await;
    this.update(cx, |this, cx| { /* Ok((result, elapsed)) | Err(error) */ cx.notify(); }).log_err();
}));
```

The current table-page equivalent to replace is `reload_data` at
`/Users/user/zed/crates/database_ui/src/table_data_view.rs:1032-1062`
(`client.fetch_rows(&table, &spec)` at :1042, same `Tokio::spawn_result` +
`_data_task` shape).

## 4. `fake.rs` — mocking `run_query` in GPUI tests (critical)

`/Users/user/zed/crates/database_client/src/fake.rs`. Struct (11-20):

```rust
pub struct FakeDatabaseClient {
    pub databases: Vec<String>,
    pub schemas: Vec<String>,
    pub tables: Vec<TableInfo>,
    pub structure: TableStructure,
    pub page: RowsPage,            // canned fetch_rows result — dies with fetch_rows
    pub query_result: QueryResult, // canned run_query result
    pub error: Option<String>,
    calls: Mutex<Vec<String>>,     // private; read via calls()
}
```

API: `new()` (29-88), `with_error(message: &str) -> Self` (91-96),
`calls(&self) -> Vec<String>` (99-104). There are **no setter methods, no queues, no
per-method errors** — fields are plain `pub` and must be assigned **before** wrapping
in `Arc` (no interior mutability afterwards):

```rust
// Stub the run_query result (pattern from database_mcp/src/tools.rs:418-426):
let mut fake = FakeDatabaseClient::new();
fake.query_result = QueryResult {
    columns: vec!["id".into(), "name".into()],
    rows: vec![vec![Some("1".into()), Some("Alice".into())],
               vec![Some("2".into()), None]],
    truncated: false,
    command_tag: Some("SELECT 2".into()),
};
let fake = Arc::new(fake);
let client: Arc<dyn DatabaseClient> = fake.clone(); // keep `fake` to read calls()
```

Every `run_query` call returns `self.query_result.clone()` regardless of the SQL and
ignores `max_rows`/`database` (fake.rs:165-171). **Reading the SQL the view executed**
— the fake records one string per call, format (fake.rs:167-169):
`"run_query {database} max_rows={max_rows} sql={sql}"`. `sql=` is the last field, so
the full text is everything after the first `"sql="`:

```rust
let last_sql = fake.calls().iter().rev()
    .find_map(|call| call.split_once("sql=").map(|(_, sql)| sql.to_string()))
    .expect("run_query was called");
assert_eq!(last_sql, "SELECT * FROM \"public\".\"users\" LIMIT 100 OFFSET 0;");
```

Errors: `FakeDatabaseClient::with_error("boom")` makes **every** method (including
`table_structure`, which the table view loads eagerly) return `Err(anyhow!("boom"))`
via `check_error` (112-117) — there is no way to fail only `run_query`. Canned defaults
from `new()`: structure = `id` (int4, PK, not null) + `name` (text, nullable); `page` =
3 rows with `has_more: true`; `query_result` = 1 column `count`, 1 row `["3"]`,
`command_tag: Some("SELECT 1")`; databases `["app","postgres"]`, schemas `["public"]`,
tables `users` + `orders_view` (view).

**Gaps stage 3 will hit** (extend the fake in `fake.rs` when needed):
1. Single canned `query_result` — pagination/sort tests can't return different pages per
   call. If a test needs that, add e.g. `pub queued_results: Mutex<VecDeque<QueryResult>>`
   falling back to `query_result` (mirror the existing `calls` Mutex pattern).
2. All-or-nothing `error` — a "query fails but structure loads" test needs a
   `run_query`-only error knob.
3. Existing table tests assert on recorded-call strings like
   `call.starts_with("fetch_rows")` — after the switch they must assert on
   `starts_with("run_query")` / the `sql=` payload instead.

## 5. Full `fetch_rows` blast radius (grep, workspace-wide)

Only `crates/database_ui/src/table_data_view.rs` uses it outside the crate:
- Production: `:1042` (inside `reload_data`), doc comment `:225`, `spec: SelectSpec`
  field `:252`, `page: Option<Arc<RowsPage>>` field `:255`, spec init `:304`,
  accessors `spec()` `:338` / `page()` `:342`, imports `:6-7` (`RowsPage, SelectSpec,
  Sort, SortDirection` among others).
- Tests asserting on `"fetch_rows"` call strings: `:2200-2201, 2247, 2293, 2300, 2381,
  2395, 2618, 2625, 3057, 3347`.
- Inside the crate: trait `database_client.rs:172`, `postgres.rs:318-350` +
  smoke-test uses (`postgres.rs:696, 720, 767`), `fake.rs:152-163` + test `:220-227`.
- MCP (`crates/database_mcp`) has zero references — safe.

`build_select`/`param_cast`/`escape_like` call sites outside `database_client`: none
(the `build_select*` hits in `crates/agent_ui/src/config_options.rs` are the unrelated
`build_selectors`). `quote_ident` call sites outside the crate: none yet.

## 6. Misc facts for planning

- `DatabaseSettings` (`/Users/user/zed/crates/database_ui/src/database_settings.rs:4-10`):
  `page_size: u32`, `query_timeout_seconds: u64`, `mcp_max_rows: u32`,
  `connections: Vec<ConnectionConfig>`. The spec's footer page-size default comes from
  `page_size`.
- `RowsPage.rows`/`QueryResult.rows` share the shape `Vec<Vec<Option<String>>>`, so the
  grid render code largely ports over; `QueryResult` has no `has_more` — compute
  `rows.len() == limit` in the new `QueryState` layer.
- `run_query` on the ReadWrite UI session will happily execute non-SELECT text
  (UPDATE/DDL) — the spec accepts this (same semantics as the SQL tab).
- Trait changes require `#[async_trait::async_trait]` on both impls; keep signatures
  in sync in `postgres.rs:214` and `fake.rs:120`.
- Build check: `./script/clippy` (not `cargo clippy`).
