# Data Editing Implementation Plan (stage 2)

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development. Steps use checkbox (`- [ ]`) tracking.

**Goal:** Инлайн-редактирование ячеек, добавление и удаление строк во вкладке данных с сохранением одной транзакцией (read-write сессия UI).

**Architecture:** Расширяем `database_client` (типы правок + `build_update/insert/delete` в sql.rs + `apply_edits` в трейте/Postgres/fake) и `database_ui/table_data_view.rs` (буфер правок, инлайн-редактор, +Row/Delete, Save/Discard). Спека: `docs/superpowers/specs/2026-07-03-data-editing-design.md`.

**Tech Stack:** Rust, GPUI, tokio-postgres; тесты `#[gpui::test]` + `cargo test -p <crate>`, живой Postgres в Docker.

## Global Constraints

- Ветка `database-viewer` (продолжение). `source "$HOME/.cargo/env"` перед cargo. Коммит-трейлер `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
- Сборка zed: `cargo check -p zed --features gpui_platform/runtime_shaders`. Форматирование пиннутым rustfmt: `cargo fmt -p <crate>` перед коммитом; `--check` должен быть чист.
- Никаких `unwrap()`/`expect()` вне тестов и `from_settings`; ошибки — `anyhow::Result` + `?`; никаких `mod.rs`.
- Значения — только параметрами `$N::text::"udt_schema"."udt_name"` (тип из `ColumnInfo`, поля `udt_schema`/`udt_name` уже есть после fix-волны); идентификаторы — только `quote_ident`; конкатенация пользовательских значений запрещена.
- UPDATE/DELETE только по полному PK; каждая должна затрагивать ровно 1 строку. Всё в одной транзакции; откат при любой ошибке. `apply_edits` работает только на `SessionMode::ReadWrite`; на ReadOnly — ошибка.
- Живой Postgres для тестов: 127.0.0.1:5432, postgres/postgres, база `shop` (customers/orders с int PK). Env `ZED_DB_TEST_HOST`/`ZED_DB_TEST_PASSWORD`.
- Действующий код: `database_client` (типы, sql, DatabaseClient трейт с fetch_rows/run_query/table_structure/cancel_running, PostgresClient, FakeDatabaseClient), `database_ui/table_data_view.rs` (TableDataView: поля client/table/mode/spec/page/structure, `_data_task`/`_structure_task`, геттеры, reload_data/reload_structure, рендер через ui::Table). `ColumnInfo { name, data_type, udt_name, udt_schema, is_nullable, default, is_primary_key }`.

## Интерфейсы между задачами (источник истины)

```rust
// database_client/src/database_client.rs — новые типы (Task 1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditCell { Value(String), Null }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]  // Hash: RowKey used as HashMap/HashSet key in Task 3
pub struct RowKey { pub columns: Vec<String>, pub values: Vec<Option<String>> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowUpdate { pub key: RowKey, pub set: Vec<(String, EditCell)> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowInsert { pub values: Vec<(String, EditCell)> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowDelete { pub key: RowKey }

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TableEdits { pub updates: Vec<RowUpdate>, pub inserts: Vec<RowInsert>, pub deletes: Vec<RowDelete> }

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AppliedCounts { pub updated: usize, pub inserted: usize, pub deleted: usize }

// database_client/src/sql.rs — pure builders (Task 1). Возвращают (sql, params: Vec<String>).
pub struct BuiltStatement { pub sql: String, pub params: Vec<String> }
pub fn build_update(table: &TableRef, columns: &[ColumnInfo], update: &RowUpdate) -> anyhow::Result<BuiltStatement>;
pub fn build_insert(table: &TableRef, columns: &[ColumnInfo], insert: &RowInsert) -> anyhow::Result<BuiltStatement>;
pub fn build_delete(table: &TableRef, columns: &[ColumnInfo], delete: &RowDelete) -> anyhow::Result<BuiltStatement>;

// DatabaseClient трейт (Task 2) — новый метод:
async fn apply_edits(&self, table: &TableRef, columns: &[ColumnInfo], edits: &TableEdits) -> Result<AppliedCounts>;
```

Правила построения (Task 1):
- Каждое значение (`EditCell::Value`) → параметр с кастом `$N::text::{quote_ident(udt_schema)}.{quote_ident(udt_name)}` колонки. `EditCell::Null` → литерал `NULL` (без параметра).
- `build_update`: `UPDATE {schema}.{table} SET {col}=<val>, ... WHERE {pk}=<val> AND ...`. SET — колонки из `update.set` (кроме PK — валидация: PK в set → Err). WHERE — по `RowKey` (каждая колонка через параметр-каст к её типу). Пустой set → Err.
- `build_insert`: `INSERT INTO {schema}.{table} ({cols}) VALUES (<vals>)` только по `insert.values`; пустой → Err.
- `build_delete`: `DELETE FROM {schema}.{table} WHERE {pk}=<val> AND ...` по `RowKey`; пустой key → Err.
- Валидация: все имена колонок должны существовать в `columns` (иначе Err); типы кастов берутся из соответствующего `ColumnInfo`.

`apply_edits` (Task 2): в транзакции на read-write клиенте, порядок DELETE → UPDATE → INSERT; для каждой UPDATE/DELETE проверить `rows_affected == 1` (иначе `bail!("row not found or changed concurrently")` → ROLLBACK); при любой ошибке ROLLBACK и вернуть её; при успехе COMMIT и вернуть `AppliedCounts`. На `SessionMode::ReadOnly` — сразу `bail!`.

`database_ui` (Tasks 3–5): буфер `TableEditBuffer` в `TableDataView`; методы `begin_edit_cell/set_cell_value/set_cell_null/add_row/delete_row/discard_edits/save_edits`; гейт `editable = !is_view && has_pk`; сборка `TableEdits` из буфера (RowKey из оригинальных PK-значений строки).

---

### Task 1: Типы правок + `build_update`/`build_insert`/`build_delete` в sql.rs

**Files:**
- Modify: `crates/database_client/src/database_client.rs` (типы из раздела «Интерфейсы»)
- Modify: `crates/database_client/src/sql.rs` (три builder-функции + `BuiltStatement`, тесты)

**Interfaces:** Produces все типы правок + три builder-функции (сигнатуры выше).

- [ ] **Step 1: Тесты** (в sql.rs `#[cfg(test)] mod tests`, рядом с существующими). Хелпер `col(name, udt)` уже есть — расширь, чтобы задавать `udt_schema` (например "pg_catalog") и `is_primary_key`. Написать:
  - `build_update_sets_and_where`: колонки `id(int4,pk)`, `name(text)`, `age(int4)`; update key={columns:["id"],values:[Some("7")]}, set=[("name",Value("Ann")),("age",Null)]. Ожидать SQL `UPDATE "public"."users" SET "name" = $1::text::"pg_catalog"."text", "age" = NULL WHERE "id" = $2::text::"pg_catalog"."int4"` и params `["Ann","7"]`.
  - `build_insert_only_given_columns`: insert values=[("name",Value("Bob")),("id",Value("9"))] → `INSERT INTO "public"."users" ("name", "id") VALUES ($1::text::"pg_catalog"."text", $2::text::"pg_catalog"."int4")`, params `["Bob","9"]`.
  - `build_delete_by_pk`: key columns=["id"],values=[Some("3")] → `DELETE FROM "public"."users" WHERE "id" = $1::text::"pg_catalog"."int4"`, params `["3"]`.
  - `build_update_rejects_pk_in_set` и `build_update_rejects_empty_set` и `build_insert_rejects_empty` и `build_update_rejects_unknown_column` → все `is_err()`.
  (Строки ожиданий собирай одной строкой.)
- [ ] **Step 2:** `cargo test -p database_client` → FAIL (функций нет).
- [ ] **Step 3:** Реализовать типы в database_client.rs и три функции в sql.rs. Переиспользуй существующий приём каста параметров из `build_select` (после fix-волны там `$N::text::"schema"."udt"`); вынеси общий хелпер `fn param_cast(column: &ColumnInfo, index: usize) -> String` если удобно (DRY с build_select). NULL — литерал без параметра. Валидация имён колонок как в build_select (`find_column`).
- [ ] **Step 4:** `cargo test -p database_client` → PASS. `cargo fmt -p database_client`; `cargo clippy -p database_client --all-targets -- -D warnings`.
- [ ] **Step 5: Commit** `database_client: Add row edit types and UPDATE/INSERT/DELETE builders`.

---

### Task 2: `apply_edits` в трейте + Postgres (транзакция) + fake + live-тест

**Files:**
- Modify: `crates/database_client/src/database_client.rs` (метод трейта)
- Modify: `crates/database_client/src/postgres.rs` (реализация + live-тест)
- Modify: `crates/database_client/src/fake.rs` (реализация + журнал)

**Interfaces:** Consumes Task 1. Produces `DatabaseClient::apply_edits`.

- [ ] **Step 1: Тесты fake** (fake.rs): `apply_edits` журналирует `format!("apply_edits u={} i={} d={}", ...)` и возвращает `AppliedCounts` с этими числами (или `error`-режим → Err). Тест `#[tokio::test] fake_apply_edits_records_and_counts` строит TableEdits (1 update, 1 insert, 1 delete) → counts {1,1,1} и запись в `calls()`.
- [ ] **Step 2:** `cargo test -p database_client` → FAIL.
- [ ] **Step 3:** Реализация.
  - Трейт: добавить `async fn apply_edits(&self, table: &TableRef, columns: &[ColumnInfo], edits: &TableEdits) -> Result<AppliedCounts>;`.
  - fake: журнал + канированные counts (по длинам векторов) / error-режим.
  - Postgres (`postgres.rs`): если `self.mode == SessionMode::ReadOnly` → `bail!("apply_edits requires a read-write session")`. Иначе взять клиента для `table.database`, выполнить в транзакции: `client.simple_query("BEGIN")` → для каждого delete/update/insert построить через sql.rs и выполнить `client.execute(&sql, &params_as_dyn)` (execute возвращает u64 rows_affected); для delete/update проверить `== 1` иначе rollback+bail; накопить counts; при любой ошибке `client.simple_query("ROLLBACK")` (лучше — через хелпер/`scopeguard`-подобный порядок: выполнить тело в замыкании, при Err сделать ROLLBACK и вернуть ошибку) → иначе `COMMIT`. Параметры биндить как в fetch_rows (`Vec<String>` → `&[&(dyn ToSql + Sync)]`). Порядок: deletes, updates, inserts.
  - Зарегистрировать cancel-token как в других методах (по желанию; апдейты короткие).
- [ ] **Step 4:** `cargo test -p database_client` → PASS.
- [ ] **Step 5: Live-тест** (`#[ignore]`, postgres.rs) `apply_edits_transaction_smoke`: на временной таблице (`CREATE TEMPORARY TABLE zed_edit(id int primary key, name text)` + пара строк через `run_query`? нет — temp-таблица живёт в сессии; используй обычную с уникальным именем и дропни в конце, ИЛИ выполни всё на реальной `customers`-подобной, но без мусора). Проще: в тесте через отдельный прямой tokio_postgres клиент подготовить таблицу `zed_edit_test`, затем `apply_edits`: update id=1 set name, insert новую, delete id=2; проверить counts и итоговое состояние SELECT'ом; затем негативный кейс — insert с нарушением (дубликат PK) откатывает весь пакет (данные без изменений). В конце `DROP TABLE zed_edit_test`. Все шаги — через ReadWrite-клиента и вспомогательный psql-setup внутри теста.
- [ ] **Step 6:** `ZED_DB_TEST_HOST=127.0.0.1 ZED_DB_TEST_PASSWORD=postgres cargo test -p database_client -- --ignored` → PASS. `cargo fmt`/`clippy` чисто. `cargo check -p zed --features gpui_platform/runtime_shaders` (трейт вырос — убедиться, что fake/impl в других местах не требуются; database_mcp реализует трейт? нет — MCP использует PostgresClient напрямую, трейт-объект только в UI; проверь, что нет других impl DatabaseClient, которым нужен новый метод — если есть, добавь).
- [ ] **Step 7: Commit** `database_client: Add transactional apply_edits with row-affected checks`.

---

### Task 3: Буфер правок + гейт editable + упреждающая загрузка структуры

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`

**Interfaces:** Consumes Task 1–2 типы. Produces буфер и гейт (используются Task 4–5).

- [ ] **Step 1: Тесты** (GPUI, fake): `editable_gate` — для таблицы с PK (`users`, fake structure имеет id PK) `editable()==true`; для вьюхи (`orders_view`, is_view) и таблицы без PK — `false`. `structure_loaded_with_first_page` — после создания view и run_until_parked `structure().is_some()` (упреждающая загрузка). `buffer_edits` — `set_cell_value`/`set_cell_null`/`add_row`/`delete_row` меняют `pending_change_count()`, `discard_edits` обнуляет.
  (Fake structure уже имеет id(PK)/name; для «без PK» добавь в fake способ вернуть структуру без PK — например `FakeDatabaseClient::with_structure(...)` или поле; реши минимально, зафиксируй.)
- [ ] **Step 2:** FAIL.
- [ ] **Step 3:** Реализация.
  - `TableEditBuffer { updates: HashMap<RowKey, HashMap<String, EditCell>>, inserts: Vec<HashMap<String, EditCell>>, deletes: HashSet<RowKey> }` + `pending_change_count()`; RowKey должен быть `Hash+Eq` (добавь derive в Task 1, если ещё нет — обнови план-интерфейс: RowKey нужен `Hash`).
  - Поля `TableDataView`: `edits: TableEditBuffer`, `editable: bool` (вычислять при загрузке структуры).
  - Упреждающая структура: при создании/первой загрузке запускать и `reload_data`, и `reload_structure` (у них теперь раздельные task-поля), чтобы `editable`/PK были известны рано. Не ломать существующее переключение режимов.
  - Методы `set_cell_value(row_key, col, String)`, `set_cell_null(row_key, col)`, `add_row()`, `delete_row(row_key)`, `discard_edits()`, `pending_change_count()`, геттеры буфера. `RowKey` строится из PK-значений строки страницы (хелпер `row_key_for(display_row_index) -> Option<RowKey>` по structure PK + page).
  - Правки PK-ячеек запрещены (метод возвращает без изменений + log::debug).
- [ ] **Step 4:** PASS. fmt/clippy. `cargo check -p zed --features gpui_platform/runtime_shaders`.
- [ ] **Step 5: Commit** `database_ui: Add edit buffer, editability gate, eager structure load`.

---

### Task 4: Инлайн-редактор ячейки + NULL + подсветка + тулбар Save/Discard

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`

**Interfaces:** Consumes Task 3.

- [ ] **Step 1: Тесты** (GPUI, fake): `begin_edit_cell` ставит `editing_cell` для не-PK ячейки редактируемой таблицы и не ставит для PK-ячейки/вьюхи; `commit_cell_edit` кладёт значение в буфер как update; `save_edits` при пустом буфере — no-op (fake.calls без apply_edits); `save_edits` с изменениями вызывает `apply_edits` (в fake.calls) и при успехе очищает буфер + reload; `save_error_keeps_buffer` (fake with error) — буфер цел, ошибка в состоянии.
- [ ] **Step 2:** FAIL.
- [ ] **Step 3:** Реализация.
  - `editing_cell: Option<EditingCell { row: RowRef, column: usize, field: Entity<InputField> }>` (RowRef различает существующую строку по RowKey и новую по индексу в inserts).
  - `begin_edit_cell`: только если `editable` и колонка не PK; создать InputField с текущим значением; Enter → commit в буфер (update для существующей / правка в inserts для новой), Esc → отмена. NULL: кнопка/действие «Set NULL» в редакторе → `EditCell::Null`.
  - Рендер: изменённые ячейки — фон `Color`-акцент (например заметный, но не Error), новые строки inserts рендерятся дополнительными строками снизу с зелёным фоном, удаляемые — зачёркнуты/красный. NULL показывается «NULL» muted. Инлайн-редактор — `.child(field.clone())` в ячейке.
  - Тулбар вкладки: когда `pending_change_count() > 0` — блок «N changes» + Button Save (`save_edits`) + Button Discard (`discard_edits`); при `!editable` — маленький баннер «Read-only: view / no primary key».
  - `save_edits(cx)`: собрать `TableEdits` (updates из HashMap, inserts из Vec, deletes из HashSet; RowKey уже хранится), `Tokio::spawn_result(client.apply_edits(table, structure_columns, edits))`; успех → буфер очистить, reload_data, показать краткий счётчик; ошибка → `{:#}` в состоянии, буфер цел.
- [ ] **Step 4:** PASS. fmt/clippy. `cargo check -p zed --features gpui_platform/runtime_shaders`.
- [ ] **Step 5: Commit** `database_ui: Add inline cell editing, NULL control, save/discard toolbar`.

---

### Task 5: Добавление и удаление строк + сборка TableEdits + полный цикл

**Files:**
- Modify: `crates/database_ui/src/table_data_view.rs`

**Interfaces:** Consumes Task 3–4.

- [ ] **Step 1: Тесты** (GPUI, fake): `add_row_then_save_inserts` — add_row + set значения → save → apply_edits с inserts.len()==1; `delete_row_then_save` — delete_row(pk) → save → deletes.len()==1; `mixed_edits_build_correct_TableEdits` — 1 update + 1 insert + 1 delete → проверить собранный TableEdits (счётчики по секциям); после успешного save буфер пуст и reload вызван.
- [ ] **Step 2:** FAIL.
- [ ] **Step 3:** Реализация.
  - «+ Row» кнопка → `add_row()` добавляет пустую запись в inserts; её ячейки редактируются тем же инлайн-редактором (RowRef::New(index)).
  - «Delete Row» на выделенной существующей строке → `delete_row(row_key)` (в deletes; если строка была в updates — убрать оттуда). На новой (ещё не сохранённой) строке — просто убрать из inserts.
  - Финализировать сборку `TableEdits` в `save_edits` (Task 4 мог оставить заглушку для inserts/deletes) — теперь полная.
  - Выделение строки: минимальный механизм (клик по «gutter»/номеру строки помечает active row) — переиспользуй существующий row-identifier столбец, если есть; иначе кнопка Delete на hover строки.
- [ ] **Step 4:** PASS. fmt/clippy. `cargo check -p zed --features gpui_platform/runtime_shaders`.
- [ ] **Step 5: Commit** `database_ui: Add row insertion and deletion with full edit save cycle`.

---

### Task 6: Сквозная проверка на Docker-базе + clippy + доки

- [ ] **Step 1:** `cargo test -p database_client -p database_ui` зелёные; `ZED_DB_TEST_HOST=127.0.0.1 ZED_DB_TEST_PASSWORD=postgres cargo test -p database_client -- --ignored` PASS; `./script/clippy` (наши крейты) чисто.
- [ ] **Step 2:** Собрать Zed (`cargo build -p zed --features gpui_platform/runtime_shaders`); при доступе через computer use — правка ячейки/добавление/удаление/Save на таблице `customers` базы `shop`, проверить в psql; при недоступности — зафиксировать, что покрыто live-тестом apply_edits и GPUI-тестами.
- [ ] **Step 3:** Дополнить `docs/superpowers/database-viewer-usage.md` разделом про редактирование (правка ячейки, NULL, +Row, Delete, Save/Discard, ограничения: только базовые таблицы с PK, PK не редактируется).
- [ ] **Step 4:** Финальное whole-branch ревью изменений этапа 2 (модель Fable), волна фиксов при находках.
- [ ] **Step 5: Commit** остаточных правок; отчёт пользователю.

## Self-review
Покрытие спеки: типы+builders (Task 1), apply_edits транзакция (Task 2), буфер+гейт+структура (Task 3), инлайн-редактор+NULL+тулбар (Task 4), +Row/Delete+сборка (Task 5), e2e+доки+ревью (Task 6). RowKey требует `Hash` (учтено в Task 3). Все SQL — параметризованы, только PK в WHERE, транзакция с откатом.
