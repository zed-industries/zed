# Database Viewer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Встроенный просмотрщик PostgreSQL в форке Zed: панель с деревом коннекшенов, вкладки данных/структуры таблиц с сортировкой/фильтрами/пагинацией, SQL-вкладка, плюс MCP-бинарь для агентов.

**Architecture:** Три новых крейта: `database_client` (типы + SQL-билдер + `DatabaseClient` трейт + `PostgresClient` на tokio-postgres, без GPUI), `database_ui` (панель `Panel`, вкладки `Item`, модальная форма, настройки), `database_mcp` (stdio JSON-RPC MCP-бинарь). UI вызывает клиент через `gpui_tokio::Tokio::spawn_result`. Спека: `docs/superpowers/specs/2026-07-02-database-viewer-design.md`.

**Tech Stack:** Rust, GPUI, tokio-postgres 0.7, serde, async-trait; тесты — `#[gpui::test]` + `cargo test -p <crate>`.

## Global Constraints

- Ветка: `database-viewer`. Коммиты частые, после каждой задачи. Трейлер коммита: `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
- Сборка бинаря zed на этой машине ТОЛЬКО с флагом: `cargo build -p zed --features gpui_platform/runtime_shaders` (нет Xcode → нет `xcrun metal`). Для `cargo check`/`cargo test` отдельных крейтов (`database_client`, `database_ui`, `database_mcp`) флаг не нужен.
- Перед cargo-командами: `source "$HOME/.cargo/env"`.
- Никаких `unwrap()`/`expect()` вне тестов и `from_settings` (там panic — конвенция Zed); ошибки — `anyhow::Result` + `?`; игнорирование только через `.log_err()`.
- Никаких `mod.rs`; корень библиотеки — `src/<crate_name>.rs` через `[lib] path`.
- Новые крейты: `edition.workspace = true`, `publish.workspace = true`, `license = "GPL-3.0-or-later"`, `[lints] workspace = true`; членство в корневом `Cargo.toml` (`members` — алфавитно) + запись в `[workspace.dependencies]`.
- Значения фильтров — только параметрами запроса (`$N`); имена таблиц/колонок — только через `quote_ident`. Конкатенация пользовательских значений в SQL запрещена.
- Сессии Postgres: `default_transaction_read_only=on` + `statement_timeout`.
- Линт перед финальным коммитом задачи: `./script/clippy -p <изменённые крейты>` (скрипт принимает аргументы cargo clippy; если нет — `./script/clippy` целиком в финальной задаче).
- Справочники API (ОБЯЗАТЕЛЬНО читать указанные в задаче перед кодом): `docs/superpowers/plans/api-refs/*.md` — там точные сигнатуры и реальные сниппеты из этого репозитория с путями/строками.

## Интерфейсы между задачами (единый источник истины)

Публичный API `database_client` (используется задачами 4–10):

```rust
// crates/database_client/src/database_client.rs (корень библиотеки, re-exports)
pub mod sql;
pub mod postgres;
#[cfg(any(test, feature = "test-support"))]
pub mod fake;

use std::time::Duration;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String, // стартовая база
    pub user: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRef {
    pub database: String,
    pub schema: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableInfo {
    pub name: String,
    pub is_view: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,      // information_schema.columns.data_type ("integer", "text", ...)
    pub udt_name: String,       // udt_name ("int4", "text", ...) — для кастов параметров
    pub is_nullable: bool,
    pub default: Option<String>,
    pub is_primary_key: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignKey {
    pub column: String,
    pub references_schema: String,
    pub references_table: String,
    pub references_column: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexInfo {
    pub name: String,
    pub definition: String, // pg_indexes.indexdef целиком
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableStructure {
    pub columns: Vec<ColumnInfo>,
    pub foreign_keys: Vec<ForeignKey>,
    pub indexes: Vec<IndexInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOp { Eq, NotEq, Gt, Lt, Contains, IsNull }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    pub column: String,
    pub op: FilterOp,
    pub value: String, // игнорируется для IsNull
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection { Asc, Desc }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sort {
    pub column: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SelectSpec {
    pub filters: Vec<Filter>,
    pub sort: Option<Sort>,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RowsPage {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>, // все значения текстом; None = NULL
    pub has_more: bool,                 // запрашивали limit+1
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
    pub truncated: bool,                // обрезано по max_rows
    pub command_tag: Option<String>,    // например "SELECT 42"
}

#[async_trait::async_trait]
pub trait DatabaseClient: Send + Sync {
    async fn test_connection(&self) -> Result<()>;
    async fn list_databases(&self) -> Result<Vec<String>>;
    async fn list_schemas(&self, database: &str) -> Result<Vec<String>>;
    async fn list_tables(&self, database: &str, schema: &str) -> Result<Vec<TableInfo>>;
    async fn table_structure(&self, table: &TableRef) -> Result<TableStructure>;
    async fn fetch_rows(&self, table: &TableRef, spec: &SelectSpec) -> Result<RowsPage>;
    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult>;
    /// Шлёт cancel-сигнал серверу для всех активных запросов этого клиента.
    async fn cancel_running(&self) -> Result<()>;
}
```

```rust
// crates/database_client/src/sql.rs
pub fn quote_ident(ident: &str) -> String;          // "co""l" ← co"l
pub fn escape_like(value: &str) -> String;          // экранирует \ % _ (для ILIKE ... ESCAPE '\')
pub struct BuiltSelect { pub sql: String, pub params: Vec<String> }
pub fn build_select(table: &TableRef, columns: &[ColumnInfo], spec: &SelectSpec) -> anyhow::Result<BuiltSelect>;
// Константы метаданных: LIST_DATABASES_SQL, LIST_SCHEMAS_SQL, LIST_TABLES_SQL, COLUMNS_SQL, FOREIGN_KEYS_SQL, INDEXES_SQL
```

```rust
// crates/database_client/src/postgres.rs
pub struct PostgresClient { /* config, password, statement_timeout, clients: Mutex<HashMap<db, Arc<tokio_postgres::Client>>>, cancel_tokens: Mutex<Vec<CancelToken>> */ }
impl PostgresClient {
    pub fn new(config: ConnectionConfig, password: String, statement_timeout: Duration) -> Self;
}
// impl DatabaseClient for PostgresClient
```

```rust
// crates/database_client/src/fake.rs (feature = "test-support")
pub struct FakeDatabaseClient { /* канированные данные + журнал вызовов */ }
impl FakeDatabaseClient {
    pub fn new() -> Self;                                  // разумные дефолты: 2 базы, 1 схема, 2 таблицы, страница 3x2
    pub fn with_error(message: &str) -> Self;              // все методы возвращают Err(anyhow!(message))
    pub fn calls(&self) -> Vec<String>;                    // ["list_databases", "fetch_rows users limit=100 offset=0 sort=None", ...]
}
// impl DatabaseClient for FakeDatabaseClient
```

Ключевые имена `database_ui` (задачи 4–9): крейт `database_ui`, панель `DatabasePanel`, настройки `DatabaseSettings` (+ контент `DatabaseSettingsContent` в `settings_content`), модалка `ConnectionModal`, вкладки `TableDataView` и `SqlQueryView`, actions `database_panel::{ToggleFocus, Toggle, AddConnection, EditConnection, RemoveConnection, RefreshConnection, NewSqlQuery}` и `database::{RunQuery, CancelQuery}`. Keychain-URL коннекшена: `format!("zed-database://{}", config.name)`, username = `config.user`.

MCP-бинарь (задача 10): `zed-database-mcp`, инструменты `list_connections`, `list_tables`, `describe_table`, `run_query`.

---

### Task 1: Каркас `database_client` + типы + `quote_ident`/`escape_like`

**Files:**
- Modify: `/Users/user/zed/Cargo.toml` (members + workspace.dependencies)
- Create: `crates/database_client/Cargo.toml`
- Create: `crates/database_client/src/database_client.rs`
- Create: `crates/database_client/src/sql.rs`
- Read first: `docs/superpowers/plans/api-refs/cargo-mcp.md` (раздел Topic A — конвенции Cargo)

**Interfaces:**
- Produces: все типы из раздела «Интерфейсы» выше (кроме трейта/postgres/fake — они в задачах 2–3), `sql::quote_ident`, `sql::escape_like`.

- [ ] **Step 1: Корневой Cargo.toml**

В `[workspace] members` добавить `"crates/database_client",` (алфавитно, после `"crates/db",`). В `[workspace.dependencies]` в блок path-зависимостей добавить (алфавитно):

```toml
database_client = { path = "crates/database_client" }
```

и во внешние зависимости (алфавитно; проверь, что `async-trait` уже есть в workspace.dependencies — есть, используется другими крейтами; если вдруг нет — добавь `async-trait = "0.1"`):

```toml
tokio-postgres = "0.7"
```

- [ ] **Step 2: Cargo.toml крейта**

```toml
[package]
name = "database_client"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lib]
path = "src/database_client.rs"
doctest = false

[features]
test-support = []

[dependencies]
anyhow.workspace = true
async-trait.workspace = true
serde.workspace = true
tokio.workspace = true
tokio-postgres.workspace = true

[lints]
workspace = true
```

(`parking_lot` не нужен: используем `std::sync::Mutex`.)

- [ ] **Step 3: Написать падающие тесты для `quote_ident`/`escape_like`** — в конец `src/sql.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_ident_wraps_and_doubles_quotes() {
        assert_eq!(quote_ident("users"), "\"users\"");
        assert_eq!(quote_ident("weird\"name"), "\"weird\"\"name\"");
        assert_eq!(quote_ident("Mixed Case"), "\"Mixed Case\"");
    }

    #[test]
    fn escape_like_escapes_metacharacters() {
        assert_eq!(escape_like("100%"), "100\\%");
        assert_eq!(escape_like("a_b"), "a\\_b");
        assert_eq!(escape_like("back\\slash"), "back\\\\slash");
        assert_eq!(escape_like("plain"), "plain");
    }
}
```

`src/database_client.rs` на этом шаге: все типы из раздела «Интерфейсы» (БЕЗ трейта `DatabaseClient`, без `pub mod postgres;`/`pub mod fake;` — они появятся в задачах 2–3), плюс `pub mod sql;`. В `src/sql.rs` — заглушки `pub fn quote_ident(_: &str) -> String { String::new() }`, `pub fn escape_like(_: &str) -> String { String::new() }`.

- [ ] **Step 4: Убедиться, что тесты падают**

Run: `source "$HOME/.cargo/env" && cargo test -p database_client`
Expected: FAIL (assert_eq mismatch), компиляция проходит.

- [ ] **Step 5: Реализовать**

```rust
/// Quotes a PostgreSQL identifier: wraps in double quotes, doubles inner quotes.
pub fn quote_ident(ident: &str) -> String {
    let mut out = String::with_capacity(ident.len() + 2);
    out.push('"');
    for ch in ident.chars() {
        if ch == '"' {
            out.push('"');
        }
        out.push(ch);
    }
    out.push('"');
    out
}

/// Escapes `\`, `%`, `_` for use in `ILIKE ... ESCAPE '\'` patterns.
pub fn escape_like(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if matches!(ch, '\\' | '%' | '_') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}
```

- [ ] **Step 6: Тесты зелёные**

Run: `cargo test -p database_client`
Expected: PASS (2 passed)

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock crates/database_client
git commit -m "database_client: Add crate scaffold, core types, identifier quoting"
```

---

### Task 2: SQL-билдер `build_select` + константы метаданных

**Files:**
- Modify: `crates/database_client/src/sql.rs`
- Test: там же, `#[cfg(test)] mod tests`

**Interfaces:**
- Consumes: типы из Task 1 (`TableRef`, `ColumnInfo`, `SelectSpec`, `Filter`, `FilterOp`, `Sort`, `SortDirection`).
- Produces: `BuiltSelect { sql: String, params: Vec<String> }`, `build_select(...)`, константы `LIST_DATABASES_SQL`, `LIST_SCHEMAS_SQL`, `LIST_TABLES_SQL`, `COLUMNS_SQL`, `FOREIGN_KEYS_SQL`, `INDEXES_SQL`.

Правила генерации (зафиксированы спекой):
- Каждая колонка выборки кастуется к тексту: `SELECT "id"::text, "name"::text FROM "public"."users"` — так `tokio_postgres` вернёт все значения как `Option<String>` без пер-типовой обработки.
- Фильтры: `Eq`→`=`, `NotEq`→`<>`, `Gt`→`>`, `Lt`→`<`; сравнение с кастом параметра к типу колонки: `"age" > $1::"int4"` (udt_name из `ColumnInfo`, через `quote_ident`). `Contains` → `"name"::text ILIKE $1 ESCAPE '\'`, параметр = `format!("%{}%", escape_like(value))`. `IsNull` → `"col" IS NULL` (без параметра).
- Несколько фильтров объединяются `AND`.
- Сортировка: `ORDER BY "col" ASC|DESC` (только если колонка есть в `columns`).
- Всегда `LIMIT {limit + 1} OFFSET {offset}` — лишняя строка сигнализирует `has_more` (числа не параметризуем — они `usize` из кода, не из пользовательского ввода).
- Валидация: колонка фильтра/сортировки должна существовать в `columns`, иначе `Err(anyhow!("unknown column: {name}"))`. `limit == 0` → `Err`.

- [ ] **Step 1: Написать падающие тесты**

```rust
    fn col(name: &str, udt: &str) -> ColumnInfo {
        ColumnInfo {
            name: name.to_string(),
            data_type: udt.to_string(),
            udt_name: udt.to_string(),
            is_nullable: true,
            default: None,
            is_primary_key: false,
        }
    }

    fn users_table() -> TableRef {
        TableRef { database: "app".into(), schema: "public".into(), name: "users".into() }
    }

    #[test]
    fn build_select_plain_page() {
        let columns = vec![col("id", "int4"), col("name", "text")];
        let spec = SelectSpec { filters: vec![], sort: None, limit: 100, offset: 0 };
        let built = build_select(&users_table(), &columns, &spec).unwrap();
        assert_eq!(
            built.sql,
            "SELECT \"id\"::text, \"name\"::text FROM \"public\".\"users\" LIMIT 101 OFFSET 0"
        );
        assert!(built.params.is_empty());
    }

    #[test]
    fn build_select_with_filters_sort_offset() {
        let columns = vec![col("id", "int4"), col("name", "text")];
        let spec = SelectSpec {
            filters: vec![
                Filter { column: "id".into(), op: FilterOp::Gt, value: "5".into() },
                Filter { column: "name".into(), op: FilterOp::Contains, value: "a%b".into() },
                Filter { column: "name".into(), op: FilterOp::IsNull, value: String::new() },
            ],
            sort: Some(Sort { column: "name".into(), direction: SortDirection::Desc }),
            limit: 50,
            offset: 100,
        };
        let built = build_select(&users_table(), &columns, &spec).unwrap();
        assert_eq!(
            built.sql,
            "SELECT \"id\"::text, \"name\"::text FROM \"public\".\"users\" \
             WHERE \"id\" > $1::\"int4\" AND \"name\"::text ILIKE $2 ESCAPE '\\' AND \"name\" IS NULL \
             ORDER BY \"name\" DESC LIMIT 51 OFFSET 100"
        );
        assert_eq!(built.params, vec!["5".to_string(), "%a\\%b%".to_string()]);
    }

    #[test]
    fn build_select_rejects_unknown_columns_and_zero_limit() {
        let columns = vec![col("id", "int4")];
        let bad_filter = SelectSpec {
            filters: vec![Filter { column: "nope".into(), op: FilterOp::Eq, value: "1".into() }],
            sort: None, limit: 10, offset: 0,
        };
        assert!(build_select(&users_table(), &columns, &bad_filter).is_err());

        let bad_sort = SelectSpec {
            filters: vec![],
            sort: Some(Sort { column: "nope".into(), direction: SortDirection::Asc }),
            limit: 10, offset: 0,
        };
        assert!(build_select(&users_table(), &columns, &bad_sort).is_err());

        let zero = SelectSpec { filters: vec![], sort: None, limit: 0, offset: 0 };
        assert!(build_select(&users_table(), &columns, &zero).is_err());
    }
```

(Строку с `WHERE` в тесте собери без переносов — литерал одной строкой; `\` в конце строк в примере выше только для читаемости плана.)

- [ ] **Step 2: Убедиться, что тесты падают** (заглушка `build_select` → `Err(anyhow!("todo"))` не подойдёт из-за `deny(todo)` в clippy — верни `Err(anyhow::anyhow!("unimplemented"))`).

Run: `cargo test -p database_client`
Expected: FAIL — 3 новых теста падают.

- [ ] **Step 3: Реализовать `build_select`**

```rust
use anyhow::{Context as _, bail};
use crate::{ColumnInfo, SelectSpec, SortDirection, TableRef, FilterOp};

pub struct BuiltSelect {
    pub sql: String,
    pub params: Vec<String>,
}

pub fn build_select(
    table: &TableRef,
    columns: &[ColumnInfo],
    spec: &SelectSpec,
) -> anyhow::Result<BuiltSelect> {
    if spec.limit == 0 {
        bail!("page size must be greater than zero");
    }
    let find_column = |name: &str| -> anyhow::Result<&ColumnInfo> {
        columns
            .iter()
            .find(|column| column.name == name)
            .with_context(|| format!("unknown column: {name}"))
    };

    let select_list = columns
        .iter()
        .map(|column| format!("{}::text", quote_ident(&column.name)))
        .collect::<Vec<_>>()
        .join(", ");

    let mut sql = format!(
        "SELECT {} FROM {}.{}",
        select_list,
        quote_ident(&table.schema),
        quote_ident(&table.name),
    );

    let mut params = Vec::new();
    let mut predicates = Vec::new();
    for filter in &spec.filters {
        let column = find_column(&filter.column)?;
        let ident = quote_ident(&column.name);
        match filter.op {
            FilterOp::IsNull => predicates.push(format!("{ident} IS NULL")),
            FilterOp::Contains => {
                params.push(format!("%{}%", escape_like(&filter.value)));
                predicates.push(format!("{ident}::text ILIKE ${} ESCAPE '\\'", params.len()));
            }
            FilterOp::Eq | FilterOp::NotEq | FilterOp::Gt | FilterOp::Lt => {
                let op = match filter.op {
                    FilterOp::Eq => "=",
                    FilterOp::NotEq => "<>",
                    FilterOp::Gt => ">",
                    FilterOp::Lt => "<",
                    FilterOp::Contains | FilterOp::IsNull => unreachable!(),
                };
                params.push(filter.value.clone());
                predicates.push(format!(
                    "{ident} {op} ${}::{}",
                    params.len(),
                    quote_ident(&column.udt_name)
                ));
            }
        }
    }
    if !predicates.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&predicates.join(" AND "));
    }

    if let Some(sort) = &spec.sort {
        let column = find_column(&sort.column)?;
        let direction = match sort.direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        sql.push_str(&format!(" ORDER BY {} {direction}", quote_ident(&column.name)));
    }

    sql.push_str(&format!(" LIMIT {} OFFSET {}", spec.limit + 1, spec.offset));
    Ok(BuiltSelect { sql, params })
}
```

- [ ] **Step 4: Добавить константы метаданных** (без тестов — это статические строки; проверятся живым Postgres в Task 11):

```rust
pub const LIST_DATABASES_SQL: &str =
    "SELECT datname FROM pg_database WHERE NOT datistemplate AND datallowconn ORDER BY datname";

pub const LIST_SCHEMAS_SQL: &str = "SELECT schema_name FROM information_schema.schemata \
     WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast') \
     ORDER BY schema_name";

pub const LIST_TABLES_SQL: &str = "SELECT table_name, table_type FROM information_schema.tables \
     WHERE table_schema = $1 AND table_type IN ('BASE TABLE', 'VIEW') \
     ORDER BY table_name";

pub const COLUMNS_SQL: &str = "SELECT c.column_name, c.data_type, c.udt_name, \
     c.is_nullable = 'YES' AS is_nullable, c.column_default, \
     EXISTS (SELECT 1 FROM information_schema.table_constraints tc \
       JOIN information_schema.key_column_usage kcu \
         ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema \
       WHERE tc.constraint_type = 'PRIMARY KEY' AND tc.table_schema = c.table_schema \
         AND tc.table_name = c.table_name AND kcu.column_name = c.column_name) AS is_primary_key \
     FROM information_schema.columns c \
     WHERE c.table_schema = $1 AND c.table_name = $2 \
     ORDER BY c.ordinal_position";

pub const FOREIGN_KEYS_SQL: &str = "SELECT kcu.column_name, ccu.table_schema, ccu.table_name, ccu.column_name \
     FROM information_schema.table_constraints tc \
     JOIN information_schema.key_column_usage kcu \
       ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema \
     JOIN information_schema.constraint_column_usage ccu \
       ON tc.constraint_name = ccu.constraint_name AND tc.table_schema = ccu.table_schema \
     WHERE tc.constraint_type = 'FOREIGN KEY' AND tc.table_schema = $1 AND tc.table_name = $2 \
     ORDER BY kcu.ordinal_position";

pub const INDEXES_SQL: &str = "SELECT indexname, indexdef FROM pg_indexes \
     WHERE schemaname = $1 AND tablename = $2 ORDER BY indexname";
```

- [ ] **Step 5: Тесты зелёные**

Run: `cargo test -p database_client`
Expected: PASS (все тесты Task 1 + Task 2)

- [ ] **Step 6: Commit**

```bash
git add crates/database_client
git commit -m "database_client: Add SELECT builder with parameterized filters and metadata queries"
```

---

### Task 3: Трейт `DatabaseClient`, `PostgresClient`, `FakeDatabaseClient`

**Files:**
- Modify: `crates/database_client/src/database_client.rs` (добавить трейт + `pub mod postgres;` + `pub mod fake;`)
- Create: `crates/database_client/src/postgres.rs`
- Create: `crates/database_client/src/fake.rs`
- Read first: `docs/superpowers/plans/api-refs/gpui-tokio.md` (раздел 4 — про tokio в воркспейсе); документация tokio-postgres 0.7 (context7/docs.rs: `Config`, `connect`, `query`, `simple_query`, `CancelToken`)

**Interfaces:**
- Consumes: типы и SQL из задач 1–2.
- Produces: `DatabaseClient` (сигнатуры — в разделе «Интерфейсы»), `PostgresClient::new(config, password, statement_timeout)`, `FakeDatabaseClient::{new, with_error, calls}`.

Ключевые решения реализации `PostgresClient`:
- `tokio_postgres::Config`: `.host(&config.host).port(config.port).user(&config.user).password(&password).dbname(db).application_name("zed-database").options(format!("-c default_transaction_read_only=on -c statement_timeout={}", timeout_ms))`. TLS: `tokio_postgres::NoTls` (v1 — прямые подключения).
- `connect` возвращает `(client, connection)`; `connection` НУЖНО заспавнить: `tokio::spawn(async move { if let Err(error) = connection.await { log::warn!(...) } })` — но у нас нет `log` в депсах; используй `eprintln!` нельзя — добавь `log.workspace = true` в Cargo.toml и `log::warn!`.
- Кэш клиентов по базе: `clients: tokio::sync::Mutex<HashMap<String, Arc<tokio_postgres::Client>>>`; перед использованием проверять `client.is_closed()` → пересоздать.
- Перед каждым запросом регистрировать `client.cancel_token()` в `cancel_tokens: std::sync::Mutex<Vec<CancelToken>>`; `cancel_running` дренирует вектор и для каждого `token.cancel_query(NoTls).await` (ошибки — `.log_err()`-подобно: собрать и вернуть первую).
- `fetch_rows`: сначала колонки (`COLUMNS_SQL` через `query`), потом `build_select`, потом `client.query_raw` не нужен — `client.query(&sql, &params_as_dyn)` где параметры `&[&(dyn ToSql + Sync)]` из `Vec<String>`: `let param_refs: Vec<&(dyn ToSql + Sync)> = built.params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();`. Все колонки в результате — text (мы кастовали), читать `row.get::<_, Option<String>>(i)`.
- `has_more`: `rows.len() > spec.limit` → отрезать лишнюю строку.
- `run_query`: `client.simple_query(sql).await` → итерировать `SimpleQueryMessage::Row` (значения `row.get(i) -> Option<&str>`), колонки из `SimpleQueryMessage::RowDescription` (или из первой строки: `row.columns()`), `CommandComplete(tag)` → `command_tag`. Обрезать по `max_rows` → `truncated = true`. ВАЖНО: `simple_query` выполняет строку как есть (несколько statement'ов разрешены протоколом) — read-only сессия защищает от записи.
- `list_tables` возвращает `TableInfo { name, is_view: table_type == "VIEW" }`.
- `table_structure`: три запроса (COLUMNS_SQL, FOREIGN_KEYS_SQL, INDEXES_SQL).
- Все методы, принимающие `database: &str` / `table.database`, получают клиента через кэш `self.client_for(db).await?`.
- `is_nullable` в COLUMNS_SQL уже приведён к bool (`c.is_nullable = 'YES'`), читать как `bool`.

`FakeDatabaseClient` (в `fake.rs`, компилируется при `#[cfg(any(test, feature = "test-support"))]`):

```rust
use std::sync::Mutex;
use anyhow::{Result, anyhow};
use crate::*;

pub struct FakeDatabaseClient {
    pub databases: Vec<String>,
    pub schemas: Vec<String>,
    pub tables: Vec<TableInfo>,
    pub structure: TableStructure,
    pub page: RowsPage,
    pub query_result: QueryResult,
    pub error: Option<String>,
    calls: Mutex<Vec<String>>,
}
```

`new()` наполняет: databases `["app", "postgres"]`, schemas `["public"]`, tables `[users (таблица), orders_view (вьюха)]`, structure с колонками `id (int4, PK)`, `name (text, nullable)`, page 2 колонки × 3 строки (`has_more: true`), query_result 1×1. Каждый метод: если `error` — `Err(anyhow!(...))`; иначе пишет вызов в журнал (`fetch_rows` пишет `format!("fetch_rows {} limit={} offset={} sort={:?} filters={}", table.name, spec.limit, spec.offset, spec.sort.as_ref().map(|s| &s.column), spec.filters.len())`) и возвращает клон поля. `cancel_running` пишет `"cancel_running"` и возвращает `Ok(())`.

- [ ] **Step 1: Написать падающий юнит-тест на `FakeDatabaseClient`** (в `fake.rs`; заодно фиксирует контракт трейта; async-тест — `#[tokio::test]` нельзя (tokio без макросов в депсах) → используй `futures::executor::block_on`? НЕТ — просто добавь в `[dev-dependencies]` крейта `tokio = { workspace = true, features = ["rt", "macros"] }` и пиши `#[tokio::test]`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fake_client_returns_canned_data_and_logs_calls() {
        let fake = FakeDatabaseClient::new();
        assert_eq!(fake.list_databases().await.unwrap(), vec!["app", "postgres"]);
        let spec = SelectSpec { limit: 100, ..Default::default() };
        let table = TableRef { database: "app".into(), schema: "public".into(), name: "users".into() };
        let page = fake.fetch_rows(&table, &spec).await.unwrap();
        assert_eq!(page.rows.len(), 3);
        assert!(fake.calls().iter().any(|call| call.starts_with("fetch_rows users")));
    }

    #[tokio::test]
    async fn fake_client_error_mode() {
        let fake = FakeDatabaseClient::with_error("boom");
        let error = fake.list_databases().await.unwrap_err();
        assert!(error.to_string().contains("boom"));
    }
}
```

- [ ] **Step 2: Тесты падают** — Run: `cargo test -p database_client`. Expected: FAIL/compile error (типов ещё нет).

- [ ] **Step 3: Реализовать трейт + `fake.rs` + `postgres.rs`** по решениям выше. В `Cargo.toml` добавить `log.workspace = true` в `[dependencies]` и `[dev-dependencies] tokio = { workspace = true, features = ["rt", "macros"] }`, `futures.workspace = true` (если понадобится).

- [ ] **Step 4: Тесты зелёные** — Run: `cargo test -p database_client`. Expected: PASS.

- [ ] **Step 5: Smoke-тест на живом Postgres, `#[ignore]`** (выполнится в Task 11):

```rust
// в postgres.rs, mod tests
#[tokio::test]
#[ignore = "requires live postgres: ZED_DB_TEST_HOST/PORT/USER/PASSWORD"]
async fn postgres_client_smoke() {
    let host = std::env::var("ZED_DB_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let config = ConnectionConfig {
        name: "test".into(), host, port: 5432, database: "postgres".into(), user: "postgres".into(),
    };
    let password = std::env::var("ZED_DB_TEST_PASSWORD").unwrap_or_else(|_| "postgres".into());
    let client = postgres::PostgresClient::new(config, password, std::time::Duration::from_secs(30));
    client.test_connection().await.unwrap();
    assert!(!client.list_databases().await.unwrap().is_empty());
    // read-only: запись должна упасть
    let error = client.run_query("postgres", "CREATE TABLE zed_should_fail(id int)", 10).await;
    assert!(error.is_err() || error.unwrap().command_tag.is_none(), "write must be rejected");
}
```

(Проверь точное поведение: в read-only сессии CREATE TABLE возвращает ошибку из `simple_query` — тогда `is_err()` истинно; оставь только эту ветку, если так.)

- [ ] **Step 6: Компиляция + clippy** — Run: `cargo test -p database_client && ./script/clippy -p database_client` (если скрипт не принимает `-p`, запусти `cargo clippy -p database_client -- -D warnings` с оговоркой, что финальная проверка — в Task 11). Expected: PASS, без warnings.

- [ ] **Step 7: Commit**

```bash
git add crates/database_client Cargo.lock
git commit -m "database_client: Add DatabaseClient trait, tokio-postgres implementation, fake client"
```

---

### Task 4: Настройки `database` + каркас `database_ui` с пустой панелью, зарегистрированной в Zed

**Files:**
- Create: `crates/settings_content/src/database.rs`; Modify: `crates/settings_content/src/settings_content.rs` (mod + поле)
- Modify: `assets/settings/default.json` (секция `"database"`)
- Create: `crates/database_ui/Cargo.toml`, `crates/database_ui/src/database_ui.rs`, `crates/database_ui/src/database_settings.rs`, `crates/database_ui/src/database_panel.rs`
- Modify: `/Users/user/zed/Cargo.toml` (member + workspace dep `database_ui`), `crates/zed/Cargo.toml` (dep), `crates/zed/src/main.rs` (`database_ui::init(cx);` рядом с `git_ui::init(cx);` ~строка 773), `crates/zed/src/zed.rs` (import + `DatabasePanel::load` + `add_panel_when_ready` в `initialize_panels`, строки ~748–785; и `database_ui::init` в тестовом init ~5561–5608 рядом с остальными)
- Read first: `docs/superpowers/plans/api-refs/settings.md`, `docs/superpowers/plans/api-refs/panel.md`, `docs/superpowers/plans/api-refs/gpui-tests.md`

**Interfaces:**
- Consumes: `database_client::ConnectionConfig`.
- Produces: `DatabaseSettingsContent` (settings_content), `DatabaseSettings { page_size: u32, query_timeout_seconds: u64, mcp_max_rows: u32, connections: Vec<ConnectionConfig> }` c `DatabaseSettings::get_global(cx)`, `DatabasePanel` (типа `Panel`), actions `database_panel::{Toggle, ToggleFocus, AddConnection}` (остальные actions добавит Task 5), `database_ui::init(cx: &mut App)`, `DatabasePanel::load(workspace, cx) -> Result<Entity<DatabasePanel>>`.

Содержимое `crates/settings_content/src/database.rs` (шаблон — соседние модули, дерайвы обязательны, см. api-refs/settings.md раздел 4):

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DatabaseSettingsContent {
    /// Number of rows per page in the database table data view.
    ///
    /// Default: 100
    pub page_size: Option<u32>,
    /// Statement timeout for database queries, in seconds.
    ///
    /// Default: 30
    pub query_timeout_seconds: Option<u64>,
    /// Maximum number of rows the MCP run_query tool returns.
    ///
    /// Default: 200
    pub mcp_max_rows: Option<u32>,
    /// Configured database connections. Passwords are stored in the system keychain.
    ///
    /// Default: []
    pub connections: Option<Vec<DatabaseConnectionContent>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DatabaseConnectionContent {
    /// Unique display name of the connection.
    pub name: String,
    /// Server host name or IP address.
    pub host: String,
    /// Server port.
    pub port: u16,
    /// Initial database to connect to.
    pub database: String,
    /// User name.
    pub user: String,
}
```

В `settings_content.rs`: `mod database; pub use database::*;` (рядом с остальными mod) и поле `pub database: Option<DatabaseSettingsContent>,` в `SettingsContent` (алфавитная позиция, рядом с существующими полями). В `assets/settings/default.json` (алфавитно, рядом с другими секциями верхнего уровня):

```jsonc
"database": {
  // Number of rows per page in the database table data view.
  "page_size": 100,
  // Statement timeout for database queries, in seconds.
  "query_timeout_seconds": 30,
  // Maximum number of rows the MCP run_query tool returns.
  "mcp_max_rows": 200,
  // Database connections (passwords live in the system keychain).
  "connections": []
},
```

`crates/database_ui/Cargo.toml`:

```toml
[package]
name = "database_ui"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lib]
path = "src/database_ui.rs"
doctest = false

[features]
test-support = ["database_client/test-support"]

[dependencies]
anyhow.workspace = true
database_client.workspace = true
editor.workspace = true
fs.workspace = true
futures.workspace = true
gpui.workspace = true
gpui_tokio.workspace = true
language.workspace = true
log.workspace = true
menu.workspace = true
project.workspace = true
schemars.workspace = true
serde.workspace = true
settings.workspace = true
ui.workspace = true
ui_input.workspace = true
util.workspace = true
workspace.workspace = true
zed_credentials_provider.workspace = true

[dev-dependencies]
database_client = { workspace = true, features = ["test-support"] }
gpui = { workspace = true, features = ["test-support"] }
project = { workspace = true, features = ["test-support"] }
settings = { workspace = true, features = ["test-support"] }
theme_settings.workspace = true
theme = { workspace = true, features = ["test-support"] }
workspace = { workspace = true, features = ["test-support"] }
zlog.workspace = true

[lints]
workspace = true
```

(Проверь имена фич test-support у theme/settings по соседним крейтам — образец: `crates/project_panel/Cargo.toml` `[dev-dependencies]`.)

`database_settings.rs`:

```rust
use gpui::App;
use settings::{RegisterSetting, Settings};
use database_client::ConnectionConfig;

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct DatabaseSettings {
    pub page_size: u32,
    pub query_timeout_seconds: u64,
    pub mcp_max_rows: u32,
    pub connections: Vec<ConnectionConfig>,
}

impl Settings for DatabaseSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let database = content.database.clone().unwrap();
        Self {
            page_size: database.page_size.unwrap(),
            query_timeout_seconds: database.query_timeout_seconds.unwrap(),
            mcp_max_rows: database.mcp_max_rows.unwrap(),
            connections: database
                .connections
                .unwrap_or_default()
                .into_iter()
                .map(|connection| ConnectionConfig {
                    name: connection.name,
                    host: connection.host,
                    port: connection.port,
                    database: connection.database,
                    user: connection.user,
                })
                .collect(),
        }
    }
}
```

(Если `RegisterSetting` дерайв требует `Deserialize` — добавь; проверь по `ProjectPanelSettings`. Панельные настройки dock/width для v1 не делаем: позиция Left фиксирована, ширина 240px.)

`database_panel.rs` — минимальная панель (полные обязательные методы `Panel` — в api-refs/panel.md, раздел 1):

```rust
use gpui::{
    App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels,
    WeakEntity, Window, actions, px,
};
use ui::prelude::*;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

actions!(
    database_panel,
    [
        /// Toggles the database panel.
        Toggle,
        /// Toggles focus on the database panel.
        ToggleFocus,
        /// Opens the new connection dialog.
        AddConnection,
    ]
);

pub struct DatabasePanel {
    focus_handle: FocusHandle,
    // Поля workspace: WeakEntity<Workspace>, fs: Arc<dyn Fs>, store: Entity<ConnectionStore>
    // добавит Task 5 — в Task 4 их не заводить, чтобы не ловить dead_code warnings.
}

impl DatabasePanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| Self::new(workspace, window, cx))
    }

    fn new(
        _workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| DatabasePanel { focus_handle: cx.focus_handle() })
    }
}

impl Render for DatabasePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DatabasePanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(
                v_flex().size_full().items_center().justify_center().gap_2()
                    .child(Label::new("No connections").color(Color::Muted))
                    .child(
                        Button::new("add-connection", "Add Connection")
                            .on_click(|_, window, cx| {
                                window.dispatch_action(AddConnection.boxed_clone(), cx);
                            }),
                    ),
            )
    }
}

impl Focusable for DatabasePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for DatabasePanel {}

impl Panel for DatabasePanel {
    fn persistent_name() -> &'static str { "DatabasePanel" }
    fn panel_key() -> &'static str { "DatabasePanel" }
    fn position(&self, _: &Window, _: &App) -> DockPosition { DockPosition::Left }
    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }
    fn set_position(&mut self, _: DockPosition, _: &mut Window, _: &mut Context<Self>) {}
    fn default_size(&self, _: &Window, _: &App) -> Pixels { px(240.) }
    fn icon(&self, _: &Window, _: &App) -> Option<ui::IconName> { Some(ui::IconName::Database) }
    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> { Some("Database Panel") }
    fn toggle_action(&self) -> Box<dyn gpui::Action> { Box::new(ToggleFocus) }
    fn activation_priority(&self) -> u32 { 6 }
}
```

ВАЖНО про иконку: сначала выполни `grep -n "Database" crates/ui/src/components/icon.rs crates/icons/src/*.rs 2>/dev/null` (реальное расположение enum `IconName` найди grep'ом `enum IconName`). Если варианта `Database` нет — возьми существующий подходящий (например `Server`, `Box`, grep по списку), НЕ добавляй новый SVG в этой задаче; замена иконки — косметика для Task 11.

`database_ui.rs` (корень):

```rust
mod database_panel;
mod database_settings;

pub use database_panel::{DatabasePanel, Toggle, ToggleFocus};
pub use database_settings::DatabaseSettings;

use gpui::App;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<DatabasePanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            if !workspace.toggle_panel_focus::<DatabasePanel>(window, cx) {
                workspace.close_panel::<DatabasePanel>(window, cx);
            }
        });
    })
    .detach();
}
```

Wiring в zed (см. api-refs/panel.md раздел 3): main.rs — `database_ui::init(cx);` после `git_ui::init(cx);`; zed.rs — `use database_ui::DatabasePanel;`, в `initialize_panels`: `let database_panel = DatabasePanel::load(workspace_handle.clone(), cx.clone());` + строка в `futures::join!`: `add_panel_when_ready(database_panel, workspace_handle.clone(), cx.clone()),`. `crates/zed/Cargo.toml`: `database_ui.workspace = true` (алфавитно). Тестовый init в zed.rs (~5561–5608): добавь `database_ui::init(cx);` рядом с `git_ui::init`-аналогами, если там есть такие вызовы (проверь по факту).

- [ ] **Step 1: settings_content + default.json** (код выше). Run: `cargo check -p settings_content -p settings` → PASS.
- [ ] **Step 2: Каркас database_ui** (Cargo.toml, три файла кода выше) + корневой Cargo.toml. Run: `cargo check -p database_ui` → PASS (иконку подставить по факту grep'а).
- [ ] **Step 3: Написать GPUI-тест** — `crates/database_ui/src/database_ui.rs`, в конец:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    fn database_settings_resolve_from_defaults(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let settings = DatabaseSettings::get_global(cx);
            assert_eq!(settings.page_size, 100);
            assert_eq!(settings.query_timeout_seconds, 30);
            assert_eq!(settings.mcp_max_rows, 200);
            assert!(settings.connections.is_empty());
        });
    }
}
```

- [ ] **Step 4: Тест зелёный** — Run: `cargo test -p database_ui`. Expected: PASS.
- [ ] **Step 5: Wiring в zed + полная проверка** — Run: `cargo check -p zed --features gpui_platform/runtime_shaders`. Expected: PASS.
- [ ] **Step 6: Commit**

```bash
git add Cargo.toml Cargo.lock assets/settings/default.json crates/settings_content crates/database_ui crates/zed
git commit -m "database_ui: Add database settings section and register empty database panel"
```

---

### Task 5: Дерево коннекшенов в панели

**Files:**
- Modify: `crates/database_ui/src/database_panel.rs` (основная работа)
- Create: `crates/database_ui/src/connection_store.rs`
- Modify: `crates/database_ui/src/database_ui.rs` (mod + re-exports + новые actions регистрация не нужна — они панельные)
- Read first: `docs/superpowers/plans/api-refs/tree-list.md`, `docs/superpowers/plans/api-refs/gpui-tokio.md`, `docs/superpowers/plans/api-refs/credentials.md`

**Interfaces:**
- Consumes: `DatabaseClient`, `PostgresClient`, `FakeDatabaseClient`, `DatabaseSettings`.
- Produces (используется задачами 6–9):

```rust
// connection_store.rs — entity с деревом и клиентами
pub type ClientFactory = std::sync::Arc<
    dyn Fn(&ConnectionConfig, &str) -> std::sync::Arc<dyn DatabaseClient> + Send + Sync,
>;

pub struct ConnectionStore {
    connections: Vec<ConnectionState>,
    client_factory: ClientFactory,
}

pub struct ConnectionState {
    pub config: ConnectionConfig,
    pub client: Option<std::sync::Arc<dyn DatabaseClient>>,
    pub status: ConnectionStatus,
    pub databases: Option<Vec<DatabaseNode>>, // None = ещё не загружали
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus { Disconnected, Connecting, Connected, Error(String) }

pub struct DatabaseNode { pub name: String, pub schemas: Option<Vec<SchemaNode>> }
pub struct SchemaNode { pub name: String, pub tables: Option<Vec<database_client::TableInfo>> }

impl ConnectionStore {
    pub fn new(client_factory: ClientFactory, cx: &mut Context<Self>) -> Self; // читает DatabaseSettings + observe SettingsStore (diff по connections)
    pub fn connections(&self) -> &[ConnectionState];
    pub fn client_for(&self, connection_name: &str) -> Option<std::sync::Arc<dyn DatabaseClient>>;
    // Загрузки; каждая по завершении обновляет состояние и cx.notify():
    /// Прод-путь: читает пароль из keychain (zed_credentials_provider, cx.spawn foreground),
    /// затем делегирует connect_with_password. Ok(None) из keychain → ConnectionStatus::Error.
    pub fn connect(&mut self, connection_name: &str, cx: &mut Context<Self>);
    /// Общее ядро (и вход для тестов — keychain не трогает): factory → test_connection → list_databases.
    pub fn connect_with_password(&mut self, connection_name: &str, password: String, cx: &mut Context<Self>);
    pub fn load_schemas(&mut self, connection_name: &str, database: &str, cx: &mut Context<Self>);
    pub fn load_tables(&mut self, connection_name: &str, database: &str, schema: &str, cx: &mut Context<Self>);
    pub fn refresh(&mut self, connection_name: &str, cx: &mut Context<Self>);        // сброс поддерева + connect заново
}
// database_ui.rs: pub use connection_store::*; — типы доступны тестам и задачам 6–9.
```

Панель: `DatabasePanel` держит `store: Entity<ConnectionStore>`, `expanded: HashSet<TreeNodeId>`, `context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>`, `scroll_handle: UniformListScrollHandle`.

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TreeNodeId {
    Connection(String),
    Database(String, String),
    Schema(String, String, String),
}
```

Дерево флэттенится в `Vec<TreeRow>` перед рендером (паттерн git_panel/outline_panel — api-refs/tree-list.md):

```rust
enum TreeRow {
    Connection { name: String, status: ConnectionStatus, expanded: bool },
    Database { connection: String, name: String, expanded: bool, loading: bool },
    Schema { connection: String, database: String, name: String, expanded: bool, loading: bool },
    Table { connection: String, database: String, schema: String, info: TableInfo },
    Loading { depth: usize },
    Error { depth: usize, message: String },
}
```

Поведение:
- Клик по строке коннекшена: toggle expanded; если `databases.is_none()` и не Connecting → `store.connect(...)`. Аналогично database→schemas, schema→tables (ленивая загрузка).
- Клик по строке таблицы: в Task 5 — no-op с `log::debug!` (Task 7 подключит открытие вкладки).
- `connect`: получить пароль из keychain на foreground (`zed_credentials_provider::global(cx)`, `cx.spawn`, url `zed-database://{name}` — api-refs/credentials.md, разделы 2–3), затем `gpui_tokio::Tokio::spawn_result(cx, async move { client.test_connection().await?; client.list_databases().await })`, результат — в entity через `this.update(cx, ...)` с `cx.notify()`. Пароль отсутствует в keychain (`Ok(None)`) → `ConnectionStatus::Error("no saved password — edit the connection".into())`.
- Иконки строк: connection — `IconName::Database`-аналог + красный `Color::Error` при `Error` (+ `Tooltip::text(message)`); database/schema — chevron через `ListItem::toggle(Some(expanded))`; table — `IconName::Table`-аналог, вьюха — другая иконка (grep IconName; fallback — одинаковая иконка + суффикс « (view)» в лейбле).
- Заголовок панели: `h_flex().justify_between()` c `Label::new("Databases")` и `IconButton::new("add-connection", IconName::Plus)` → dispatch `AddConnection` (паттерн — api-refs/tree-list.md раздел 2).
- Контекстное меню (правый клик, api-refs/tree-list.md раздел 3): на коннекшене — `Refresh`, `Edit Connection…`, `Remove Connection`, `New SQL Query`; на таблице — пункты добавит Task 7. Новые actions: `RefreshConnection`, `EditConnection`, `RemoveConnection`, `NewSqlQuery` (в `actions!(database_panel, [...])`; обработчики через `.on_action(cx.listener(...))` на корне панели; `Edit`/`New SQL Query` в Task 5 — заглушки `log::debug!`, их подключат Task 6/9; `RemoveConnection` — реализуй: `update_settings_file` удаляет коннекшен по имени + `provider.delete_credentials`).
- `uniform_list("database-tree", rows.len(), cx.processor(...))` + `.track_scroll(&self.scroll_handle)` (api-refs/tree-list.md раздел 1). Пустой список коннекшенов → текущий плейсхолдер «No connections».

ConnectionStore следит за настройками: `cx.observe_global::<SettingsStore>(...)` — если список connections из `DatabaseSettings` изменился, синхронизировать `self.connections` (новые — добавить как Disconnected, удалённые — убрать), `cx.notify()`.

- [ ] **Step 1: Написать падающие GPUI-тесты** (в `database_panel.rs` или `connection_store.rs`):

```rust
#[gpui::test]
async fn connect_populates_databases_from_client(cx: &mut TestAppContext) {
    init_test(cx); // как в Task 4 + gpui_tokio::init(cx) внутри cx.update!
    cx.update(|cx| {
        cx.update_global::<settings::SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.database.get_or_insert_default().connections = Some(vec![
                    settings::DatabaseConnectionContent {
                        name: "local".into(), host: "127.0.0.1".into(), port: 5432,
                        database: "postgres".into(), user: "postgres".into(),
                    },
                ]);
            });
        });
    });
    let fake = std::sync::Arc::new(database_client::fake::FakeDatabaseClient::new());
    let factory: ClientFactory = {
        let fake = fake.clone();
        std::sync::Arc::new(move |_, _| fake.clone() as std::sync::Arc<dyn DatabaseClient>)
    };
    let store = cx.new(|cx| ConnectionStore::new(factory, cx));
    store.update(cx, |store, cx| store.connect_with_password("local", "pw".into(), cx));
    cx.run_until_parked();
    store.read_with(cx, |store, _| {
        let connection = &store.connections()[0];
        assert_eq!(connection.status, ConnectionStatus::Connected);
        assert_eq!(
            connection.databases.as_ref().unwrap().iter().map(|d| d.name.as_str()).collect::<Vec<_>>(),
            vec!["app", "postgres"]
        );
    });
}

#[gpui::test]
async fn connect_error_sets_error_status(cx: &mut TestAppContext) {
    // как выше, но FakeDatabaseClient::with_error("connection refused")
    // assert: matches!(status, ConnectionStatus::Error(m) if m.contains("connection refused"))
}
```

Про keychain в тестах: тесты зовут `connect_with_password` напрямую и keychain не трогают; `connect` (прод) читает keychain и делегирует туда же.

Дополнительно к красной иконке: при переходе коннекшена в `ConnectionStatus::Error` показать workspace-уведомление с текстом ошибки (панель держит `workspace: WeakEntity<Workspace>`; используй хелпер уведомлений workspace — `grep -rn "fn show_error" crates/workspace/src` и возьми существующий механизм, например `workspace::notifications`).

- [ ] **Step 2: Тесты падают** — Run: `cargo test -p database_ui`. Expected: compile FAIL (типов нет) — это допустимая «красная» фаза.
- [ ] **Step 3: Реализовать ConnectionStore + дерево + рендер + контекстное меню** по описанию выше.
- [ ] **Step 4: Тесты зелёные** — Run: `cargo test -p database_ui`. Expected: PASS.
- [ ] **Step 5: Ручная проверка компиляции всего app** — Run: `cargo check -p zed --features gpui_platform/runtime_shaders`. Expected: PASS.
- [ ] **Step 6: Commit**

```bash
git add crates/database_ui
git commit -m "database_ui: Add connections tree with lazy loading and context menu"
```

---

### Task 6: Модальная форма коннекшена + Keychain

**Files:**
- Create: `crates/database_ui/src/connection_modal.rs`
- Modify: `crates/database_ui/src/database_ui.rs` (mod), `crates/database_ui/src/database_panel.rs` (обработчики `AddConnection`/`EditConnection` открывают модалку)
- Read first: `docs/superpowers/plans/api-refs/modal-form.md`, `docs/superpowers/plans/api-refs/credentials.md`

**Interfaces:**
- Consumes: `ConnectionConfig`, `DatabaseClient` (для Test Connection — через ту же `ClientFactory`), `update_settings_file`, `zed_credentials_provider`.
- Produces: `ConnectionModal::new(existing: Option<ConnectionConfig>, client_factory: ClientFactory, fs: Arc<dyn Fs>, window, cx) -> Self`; открытие: `workspace.toggle_modal(window, cx, |window, cx| ConnectionModal::new(...))`.

Поля формы — `Entity<InputField>` (крейт `ui_input`, api-refs/modal-form.md раздел 5): name, host, port (плейсхолдер "5432"), database, user, password (`.masked(true)`). tab_index 1..6, `.tab_group()` на корне, обработчики `menu::SelectNext/SelectPrevious` → `window.focus_next/focus_prev` (раздел 4 справочника). Кнопки: `Test Connection`, `Save` (+ `Cancel` = `menu::Cancel`). `menu::Confirm` = Save.

Поведение Save:
1. Валидация: `validate(&FormValues) -> Result<ConnectionConfig, String>` — pure fn: непустые name/host/database/user; порт парсится в u16 (пустой → 5432); при создании имя не должно совпадать с существующим коннекшеном (список передаётся в модалку). Ошибка → `InputField::set_error` на соответствующем поле.
2. `update_settings_file(fs, cx, move |content, _| { ... })`: заменить/добавить `DatabaseConnectionContent` в `content.database.get_or_insert_default().connections` (get_or_insert_default для Option<Vec>).
3. Пароль: если поле непустое → `provider.write_credentials(&format!("zed-database://{name}"), &user, password.as_bytes(), cx)` в `cx.spawn` (foreground, api-refs/credentials.md). Пустое поле при редактировании → пароль не трогаем.
4. `cx.emit(DismissEvent)`.

Test Connection: собрать `ConnectionConfig` из полей + пароль из поля → `Tokio::spawn_result(cx, async move { factory_client.test_connection().await })`; результат в поле статуса модалки: `Label` зелёный «Connection OK» / красный текст ошибки. Кнопка disabled на время проверки.

EditConnection из панели: открывает модалку с `existing = Some(config)` (имя read-only — `InputField` без обработчиков не сделать read-only легко; проще: при редактировании поле name заблокировано через `editor.set_read_only(true)` на внутреннем редакторе, см. api-refs/modal-form.md раздел 6, или просто перезаписывай коннекшен с тем же именем).

- [ ] **Step 1: Падающие тесты для `validate`** (pure fn, обычный `#[test]`): пустое имя → Err; кривой порт ("abc", "70000") → Err; пустой порт → 5432; дубликат имени при создании → Err; валидные значения → Ok(config).
- [ ] **Step 2: Тесты падают** — `cargo test -p database_ui`.
- [ ] **Step 3: Реализовать модалку + открытие из панели** (обработчики `AddConnection`, `EditConnection` — `workspace.toggle_modal`; у панели есть `workspace: WeakEntity<Workspace>`).
- [ ] **Step 4: Тесты зелёные + компиляция** — `cargo test -p database_ui && cargo check -p zed --features gpui_platform/runtime_shaders`.
- [ ] **Step 5: Commit**

```bash
git add crates/database_ui
git commit -m "database_ui: Add connection modal with keychain-backed password storage"
```

---

### Task 7: Вкладка таблицы — «Данные» и «Структура», сортировка, пагинация

**Files:**
- Create: `crates/database_ui/src/table_data_view.rs`
- Modify: `crates/database_ui/src/database_ui.rs` (mod), `crates/database_ui/src/database_panel.rs` (клик по таблице открывает вкладку)
- Read first: `docs/superpowers/plans/api-refs/table.md`, `docs/superpowers/plans/api-refs/item-tab.md`

**Interfaces:**
- Consumes: `Arc<dyn DatabaseClient>` (из `ConnectionStore::client_for`), `TableRef`, `SelectSpec`, `RowsPage`, `TableStructure`, `DatabaseSettings::page_size`.
- Produces: `TableDataView::new(client: Arc<dyn DatabaseClient>, table: TableRef, window, cx) -> Entity<Self>` (Item-вкладка); actions `database::{NextPage, PrevPage, ToggleStructure, RefreshData}` в `actions!(database, [...])` внутри `table_data_view.rs`.

Состояние:

```rust
pub struct TableDataView {
    focus_handle: FocusHandle,
    client: Arc<dyn DatabaseClient>,
    table: TableRef,
    mode: ViewMode,                       // Data | Structure
    spec: SelectSpec,                     // limit = DatabaseSettings::page_size на момент создания
    page: Option<RowsPage>,
    structure: Option<TableStructure>,
    load_state: LoadState,                // Idle | Loading | Error(String)
    interaction: Entity<TableInteractionState>,
    column_widths: Option<Entity<ResizableColumnsState>>, // пересоздаётся при смене набора колонок
    _load_task: Option<Task<()>>,
}
```

Поведение:
- Конструктор запускает первую загрузку `reload(cx)`: `Tokio::spawn_result` → `fetch_rows` (+ `table_structure` при первом показе Structure). Результат через `this.update(cx, ...)`, `cx.notify()`. Ошибка → `load_state = Error(message)` → красный `Label` по центру + кнопка Retry.
- `Item`: `tab_content_text` = `format!("{}.{}", table.schema, table.name)`, `tab_icon` — та же табличная иконка, `type Event = ()` (api-refs/item-tab.md, раздел 2 — минимальный рецепт).
- Рендер Data: `ui::Table::new(cols)` c `.interactable(&self.interaction)`, `.striped()`, `.width_config(ColumnWidthConfig::Resizable(widths.clone()))`, `.uniform_list("db-rows", row_count, cx.processor(...))` — образец 1-в-1 в api-refs/table.md, раздел 2. Заголовки — `create_header_element_with_sort_button`-подобные: `h_flex().justify_between().child(Label::new(col)).child(Button "↕|↓|↑")` с циклом None→Asc→Desc→None по клику (сниппет в api-refs/table.md раздел 3); смена сортировки → `spec.sort = ...; spec.offset = 0; reload(cx)`.
- Ячейки: `div().whitespace_nowrap().text_ellipsis().child(value)`; NULL → `Label::new("NULL").color(Color::Muted).italic()`.
- Футер: `h_flex().justify_between()`: слева `Label` `rows {offset+1}–{offset+n}{ has_more ? "+" : "" }`; справа IconButton'ы `ChevronLeft`/`ChevronRight` (`PrevPage`: offset -= limit, clamp 0; `NextPage`: только если `has_more`), между ними — кнопка Refresh.
- Переключатель Data/Structure в шапке вкладки: два `Button` с `.toggle_state(...)` (или `ToggleButton` — grep по `ToggleButton::new` в crates/ui; при сомнении — два обычных Button).
- Рендер Structure: `ui::Table::new(6)` (static rows, `.row(...)`) — Name, Type, Nullable, Default, PK, FK (FK-колонка: `→ schema.table.column` если есть); ниже `Label::new("Indexes")` и по строке на индекс (`IndexInfo.definition`, `text_ellipsis` + tooltip с полным текстом).
- Клик по таблице в дереве панели (Task 5 заглушка) теперь: `workspace.update(cx, |workspace, cx| { let view = TableDataView::new(client, table_ref, window, cx); workspace.active_pane().update(cx, |pane, cx| pane.add_item(Box::new(view), true, true, None, window, cx)); })` — точные сигнатуры в api-refs/item-tab.md раздел 4. Дедупликация: если вкладка этой таблицы уже открыта (`pane.items_of_type::<TableDataView>()` + сравнение `TableRef`) — активировать её.

- [ ] **Step 1: Падающие GPUI-тесты** (fake client, `init_test` + `gpui_tokio::init`):

```rust
#[gpui::test]
async fn table_view_loads_first_page(cx: &mut TestAppContext) { /* new → run_until_parked → page.is_some(), spec.limit == 100, вызов fetch_rows в fake.calls() */ }

#[gpui::test]
async fn sort_click_resets_offset_and_reloads(cx: &mut TestAppContext) {
    /* view.update: toggle_sort("name") → run_until_parked →
       spec.sort == Some(Sort{name, Asc}), spec.offset == 0, второй fetch_rows в журнале */
}

#[gpui::test]
async fn next_prev_page_updates_offset(cx: &mut TestAppContext) {
    /* has_more=true у fake → next_page → offset == limit; prev_page → offset == 0; prev на 0 — no-op */
}

#[gpui::test]
async fn structure_mode_fetches_structure_once(cx: &mut TestAppContext) { /* toggle → structure.is_some(), повторный toggle не дёргает клиент */ }

#[gpui::test]
async fn load_error_is_surfaced(cx: &mut TestAppContext) { /* with_error → LoadState::Error содержит сообщение */ }
```

Тесты дёргают публичные методы view (`toggle_sort(column, cx)`, `next_page(cx)`, `prev_page(cx)`, `toggle_structure(cx)`), не симулируя клики — рендер-структуру не ассертим.

- [ ] **Step 2: Тесты падают** — `cargo test -p database_ui`.
- [ ] **Step 3: Реализовать view + открытие из панели.**
- [ ] **Step 4: Тесты зелёные + `cargo check -p zed --features gpui_platform/runtime_shaders`.**
- [ ] **Step 5: Commit**

```bash
git add crates/database_ui
git commit -m "database_ui: Add table data tab with structure view, sorting, pagination"
```

---

### Task 8: Фильтры по полям

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`
- Read first: `docs/superpowers/plans/api-refs/tree-list.md` (раздел 2 — PopoverMenu/ContextMenu как дропдаун)

**Interfaces:**
- Consumes: `Filter`, `FilterOp` из database_client; колонки текущей страницы/структуры.
- Produces: строка фильтров в UI; результирующие фильтры кладутся в `self.spec.filters` + `reload`.

UI: панель фильтров между шапкой и таблицей. Каждый активный фильтр — «чип»: `h_flex` с текстом `name contains 'abc'` и IconButton `X` (удалить). Кнопка `+ Filter` открывает поповер-конструктор: дропдаун колонки (PopoverMenu+ContextMenu, пункты из текущих колонок), дропдаун оператора (`=`, `≠`, `>`, `<`, `contains`, `is null`), `InputField` значения (скрыт для `is null`), кнопка Apply. Apply → `spec.filters.push(...)`, `spec.offset = 0`, `reload(cx)`. Удаление чипа → убрать из `spec.filters`, `reload`.

Маппинг операторов UI ↔ `FilterOp`: `=`→Eq, `≠`→NotEq, `>`→Gt, `<`→Lt, `contains`→Contains, `is null`→IsNull. Вынеси в pure fn `filter_op_label(op) -> &'static str` + `all_filter_ops() -> [FilterOp; 6]`.

- [ ] **Step 1: Падающие тесты**: `add_filter(...)` сбрасывает offset и вызывает reload (fake.calls: `filters=1`); `remove_filter(0)` возвращает `filters=0`; `filter_op_label` покрыт на все 6 значений.
- [ ] **Step 2: Тесты падают.**
- [ ] **Step 3: Реализовать** (публичные методы `add_filter(filter, cx)`, `remove_filter(index, cx)` + рендер).
- [ ] **Step 4: Тесты зелёные + компиляция zed.**
- [ ] **Step 5: Commit**

```bash
git add crates/database_ui
git commit -m "database_ui: Add column filters to table data view"
```

---

### Task 9: SQL-вкладка

**Files:**
- Create: `crates/database_ui/src/sql_query_view.rs`
- Modify: `crates/database_ui/src/database_ui.rs` (mod), `crates/database_ui/src/database_panel.rs` (`NewSqlQuery` в контекстном меню коннекшена и базы)
- Modify: `assets/keymaps/default-macos.json`, `assets/keymaps/default-linux.json` (биндинг RunQuery)
- Read first: `docs/superpowers/plans/api-refs/editor-embed.md` (ВЕСЬ — там готовый рецепт SQL-редактора), `docs/superpowers/plans/api-refs/table.md`

**Interfaces:**
- Consumes: `Arc<dyn DatabaseClient>`, `QueryResult`, `DatabaseSettings`.
- Produces: `SqlQueryView::new(client, connection_name: String, database: String, project: Entity<Project>, window, cx)`; actions `database::{RunQuery, CancelQuery}`.

Структура: сверху `Editor` (multi-line, высота ~40% через `v_flex` + `.h(rems(12.))` или `flex` пропорции), снизу — область результата (`ui::Table` uniform_list, как в Task 7, но колонки из `QueryResult`; без сортировки/фильтров/пагинации), между ними — тулбар: кнопка Run (`IconName::Play`-аналог) с тултипом `Tooltip::for_action("Run Query", &RunQuery, cx)`, кнопка Cancel (видна при выполнении), `Label` статуса (`command_tag`, `N rows`, `truncated…`, время выполнения через `std::time::Instant` замер вокруг await).

Ключевое из api-refs/editor-embed.md:
- Буфер с `set_language_registry(project.read(cx).languages().clone())`; язык: `language_for_name("SQL").await.ok()` — расширение может быть не установлено → остаётся plain text (это ОК; лог debug).
- Рендер редактора: просто `.child(self.editor.clone())` (вариант 2 из справочника) — достаточно для v1.
- `key_context("SqlQueryEditor")` на обёртке + `.on_action(cx.listener(Self::run_query))`; keymap:

```json
{
  "context": "SqlQueryEditor > Editor",
  "use_key_equivalents": true,
  "bindings": { "cmd-enter": "database::RunQuery", "enter": "editor::Newline" }
}
```

(в default-linux.json — `ctrl-enter`; вставить рядом с блоком `CommitEditor > Editor`, grep по файлу.)

Выполнение: `run_query`: если пусто — no-op; `running = true`; `Tokio::spawn_result(cx, { let client = ...; let sql = editor.read(cx).text(cx); async move { client.run_query(&database, &sql, 1000).await } })`; max_rows для UI = 1000 (константа `UI_MAX_QUERY_ROWS`). Результат/ошибка → состояние + `cx.notify()`. `CancelQuery` → `Tokio::spawn_result(cx, async move { client.cancel_running().await })` + отбросить `_run_task`.
`Item::tab_content_text` = `format!("SQL: {connection_name}/{database}")`; `is_dirty` = false (v1 без сохранения .sql).

Панель: `NewSqlQuery` из контекстного меню коннекшена (база = стартовая) и узла базы (база = выбранная) → открыть вкладку через `workspace`.

- [ ] **Step 1: Падающие GPUI-тесты**: `run executes and stores result` (fake: `query_result`→ view.result.is_some(), calls содержит `run_query`); `run error surfaces message`; `cancel calls client` (calls содержит `cancel_running`).
- [ ] **Step 2: Тесты падают.**
- [ ] **Step 3: Реализовать view + keymap + пункты меню.**
- [ ] **Step 4: Тесты зелёные + `cargo check -p zed --features gpui_platform/runtime_shaders`.**
- [ ] **Step 5: Commit**

```bash
git add crates/database_ui assets/keymaps
git commit -m "database_ui: Add SQL query tab with cancellation and keybinding"
```

---

### Task 10: MCP-бинарь `zed-database-mcp`

**Files:**
- Create: `crates/database_mcp/Cargo.toml`, `crates/database_mcp/src/main.rs`, `crates/database_mcp/src/protocol.rs`, `crates/database_mcp/src/tools.rs`
- Modify: `/Users/user/zed/Cargo.toml` (member; workspace dep НЕ нужен — бинарь никто не импортирует)
- Read first: `docs/superpowers/plans/api-refs/cargo-mcp.md` (Topic B: фрейминг = JSON-RPC 2.0 по строкам stdio, БЕЗ Content-Length; раздел 4 — как Zed регистрирует context servers)

**Interfaces:**
- Consumes: `database_client` (PostgresClient, типы), настройки из `paths::settings_file()` (крейт `paths` — workspace member; grep `pub fn settings_file` чтобы удостовериться в имени), пароли из macOS Keychain через `/usr/bin/security find-internet-password -s zed-database://<name> -w`.
- Produces: бинарь `zed-database-mcp` c инструментами `list_connections`, `list_tables`, `describe_table`, `run_query`.

`Cargo.toml`:

```toml
[package]
name = "database_mcp"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lints]
workspace = true

[[bin]]
name = "zed-database-mcp"
path = "src/main.rs"

[dependencies]
anyhow.workspace = true
database_client.workspace = true
paths.workspace = true
serde.workspace = true
serde_json.workspace = true
serde_json_lenient.workspace = true
tokio = { workspace = true, features = ["rt-multi-thread", "macros", "io-std", "io-util"] }

[dev-dependencies]
database_client = { workspace = true, features = ["test-support"] }
```

(`serde_json_lenient` — проверь наличие в workspace.dependencies grep'ом; это парсер JSONC, которым Zed читает settings.json. Если его нет — найди, чем settings-крейт парсит user settings (`grep -rn "parse_json_with_comments\|json_lenient" crates/settings/src`) и используй то же.)

`protocol.rs` — минимальный JSON-RPC/MCP (свои типы, ~60 строк; фрейминг: одна JSON-строка на сообщение, `\n` в конце):

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RpcRequest {
    pub id: Option<serde_json::Value>, // None = notification
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Serialize)]
pub struct RpcResponse {
    pub jsonrpc: &'static str, // "2.0"
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

#[derive(Serialize)]
pub struct RpcError {
    pub code: i32,   // -32601 method not found, -32602 invalid params, -32603 internal
    pub message: String,
}
```

Методы: `initialize` → `{"protocolVersion": "2025-06-18", "capabilities": {"tools": {}}, "serverInfo": {"name": "zed-database-mcp", "version": "0.1.0"}}` (если клиент прислал более старую версию — эхо его версии); `notifications/initialized` — игнор; `ping` → `{}`; `tools/list` → 4 инструмента с JSON Schema входа; `tools/call` → диспетчеризация; незнакомый метод с id → error -32601.

`tools.rs` — ядро, тестируемое без stdio:

```rust
pub struct ToolHost {
    pub connections: Vec<database_client::ConnectionConfig>,
    pub max_rows: usize,
    clients: std::collections::HashMap<String, std::sync::Arc<dyn database_client::DatabaseClient>>,
    client_factory: Box<dyn Fn(&ConnectionConfig, &str) -> Arc<dyn DatabaseClient> + Send>,
    password_source: Box<dyn Fn(&ConnectionConfig) -> anyhow::Result<String> + Send>,
}

impl ToolHost {
    pub fn tool_definitions() -> serde_json::Value; // массив для tools/list
    pub async fn call(&mut self, name: &str, arguments: &serde_json::Value) -> anyhow::Result<serde_json::Value>;
}
```

Инструменты (вход/выход):
- `list_connections {}` → `[{name, host, port, database, user}]` (без паролей!).
- `list_tables {connection: string, database?: string}` → `{database, schemas: [{name, tables: [{name, is_view}]}]}` (database по умолчанию — стартовая).
- `describe_table {connection, table: string ("schema.table" или "table" → public), database?}` → структура (колонки/PK/FK/индексы) как JSON.
- `run_query {connection, sql: string, database?}` → `{columns, rows, truncated, command_tag}`; rows ≤ `max_rows` (из настроек `database.mcp_max_rows`, default 200).

`tools/call` ответ MCP: `{"content": [{"type": "text", "text": <serde_json::to_string_pretty(результата)>}], "isError": false}`; ошибка инструмента → `{"content": [{"type": "text", "text": message}], "isError": true}` (НЕ JSON-RPC error — это ошибка инструмента, не протокола).

`main.rs`: `#[tokio::main]` → загрузить настройки (`paths::settings_file()` → `serde_json_lenient::from_str::<serde_json::Value>` → секция `database`), собрать `ToolHost` (client_factory = PostgresClient с таймаутом из настроек; password_source = вызов `security`), цикл: `BufReader::new(tokio::io::stdin()).lines()` → на каждую строку `serde_json::from_str::<RpcRequest>` → обработка → `stdout.write_all(line + "\n")` + flush. Ошибки парсинга строки → error -32700 c id null. stderr — для логов.

Регистрация (только документация, шаг Task 11): settings.json →

```json
"context_servers": {
  "zed-database": { "command": "/Users/user/zed/target/release/zed-database-mcp", "args": [] }
}
```

- [ ] **Step 1: Падающие тесты `tools.rs`** (fake client через factory, password_source = `Ok("pw")`):

```rust
#[tokio::test]
async fn list_connections_returns_configs_without_passwords() { /* ... */ }
#[tokio::test]
async fn run_query_truncates_to_max_rows() { /* fake query_result 3 строки, max_rows=2 → rows.len()==2, truncated==true */ }
#[tokio::test]
async fn describe_table_parses_schema_qualified_name() { /* "public.users" и "users" дают одинаковый вызов */ }
#[tokio::test]
async fn unknown_connection_is_error() { /* call с connection="nope" → Err с внятным текстом */ }
```

и тест диспетчера протокола: `initialize` → корректный JSON; `tools/list` → 4 имени; незнакомый метод → -32601 (функция `handle_request(request, host) -> Option<RpcResponse>` — pure относительно stdio).

- [ ] **Step 2: Тесты падают.** — `cargo test -p database_mcp`
- [ ] **Step 3: Реализовать.**
- [ ] **Step 4: Тесты зелёные; сборка бинаря** — `cargo build -p database_mcp` → появился `target/debug/zed-database-mcp`. Smoke вручную: `printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}\n' | ./target/debug/zed-database-mcp` → в stdout строка с `serverInfo`.
- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock crates/database_mcp
git commit -m "database_mcp: Add stdio MCP server exposing database tools"
```

---

### Task 11: Сквозная проверка на живом Postgres + линт + финал

**Files:**
- Никаких новых исходников; правки — только фиксы найденных багов.
- Read first: скилл `verify` не требуется — шаги ниже самодостаточны.

- [ ] **Step 1: Поднять тестовый Postgres.** Проверить `docker info`; если Docker недоступен — `brew install postgresql@17 && brew services start postgresql@17` (тогда user = `$USER`, без пароля — создать: `createuser -s postgres || true; psql -c "ALTER USER postgres PASSWORD 'postgres'"`). Docker-путь:

```bash
docker run -d --name zed-db-test -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:16
sleep 5
docker exec -i zed-db-test psql -U postgres <<'SQL'
CREATE DATABASE shop;
\c shop
CREATE TABLE customers (id serial PRIMARY KEY, name text NOT NULL, email text UNIQUE, created_at timestamptz DEFAULT now());
CREATE TABLE orders (id serial PRIMARY KEY, customer_id int REFERENCES customers(id), total numeric(10,2), status text, note text);
INSERT INTO customers (name, email) SELECT 'Customer ' || i, 'c' || i || '@example.com' FROM generate_series(1, 250) i;
INSERT INTO orders (customer_id, total, status, note) SELECT (i % 250) + 1, (i * 3.5)::numeric(10,2), CASE i % 3 WHEN 0 THEN 'new' WHEN 1 THEN 'paid' ELSE 'shipped' END, CASE WHEN i % 10 = 0 THEN NULL ELSE 'note ' || i END FROM generate_series(1, 1000) i;
CREATE VIEW paid_orders AS SELECT * FROM orders WHERE status = 'paid';
SQL
```

- [ ] **Step 2: Live-тест драйвера** — `ZED_DB_TEST_HOST=127.0.0.1 ZED_DB_TEST_PASSWORD=postgres cargo test -p database_client -- --ignored` → PASS (в т.ч. проверка read-only).
- [ ] **Step 3: Все тесты новых крейтов** — `cargo test -p database_client -p database_ui -p database_mcp` → PASS.
- [ ] **Step 4: Clippy** — `./script/clippy` (весь воркспейс; чинить все warnings в НАШИХ крейтах; чужие не трогать) → чисто.
- [ ] **Step 5: Собрать и запустить Zed** — `cargo run -p zed --features gpui_platform/runtime_shaders` (или `cargo build` + запуск бинаря). Через computer use (запросить доступ к приложению Zed / скриншоты):
  1. Иконка панели видна в доке; открыть панель.
  2. Add Connection: name=local, host=127.0.0.1, port=5432, database=shop, user=postgres, password=postgres → Test Connection OK → Save.
  3. Дерево: local → shop → public → customers/orders/paid_orders.
  4. Клик orders: данные (100 строк), сортировка по total desc, страница 2, фильтр `status = paid`, фильтр `note is null`, режим Structure (PK/FK/indexes видны).
  5. SQL-вкладка: `SELECT status, count(*) FROM orders GROUP BY status;` → результат; `DELETE FROM orders;` → ошибка read-only (видна в UI!); Cmd+Enter работает.
  6. Перезапуск Zed → коннекшен на месте, пароль из Keychain (без повторного ввода).
- [ ] **Step 6: MCP end-to-end** — `cargo build --release -p database_mcp`; прописать в `~/.config/zed/settings.json` секцию `context_servers` (путь к release-бинарю); smoke через pipe: initialize → tools/list → tools/call list_connections → tools/call run_query (`SELECT count(*) FROM orders`) — ответы валидны. Затем проверить в Zed: Agent Panel → settings → сервер zed-database виден/запущен.
- [ ] **Step 7: Документация для пользователя** — короткий `docs/superpowers/database-viewer-usage.md`: как открыть панель, добавить коннекшен, горячие клавиши, как подключить MCP к Claude Code (`claude mcp add zed-database /Users/user/zed/target/release/zed-database-mcp`).
- [ ] **Step 8: Финальный коммит + отчёт пользователю** (скриншоты, что работает, что отложено).

```bash
git add -A && git commit -m "database: Final fixes after end-to-end verification"
```

## Чего НЕ делаем в v1 (напоминание из спеки)

MySQL/SQLite, SSH-туннели, TLS/sslmode, редактирование данных, экспорт CSV, сохранение .sql файлов, dock-настройки панели (позиция/ширина фиксированы), иконки-кастомизация.
