# MCP Write Tools Implementation Plan (Stage 4)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let an MCP agent write to a database only through a two-phase `propose_write` → `apply_write` handshake: the server previews the mutation's before/after in a rolled-back transaction and issues a one-shot token; `apply_write` re-runs and commits only that exact SQL.

**Architecture:** Transaction/preview/commit logic lives in `database_client` as new `DatabaseClient` trait methods on a dedicated connection (mirroring `apply_edits`), with a `fake.rs` mirror. The DML classifier, one-shot token store, and the two tools live in `database_mcp`. A per-connection `allow_mcp_writes` settings flag gates everything; it is threaded to the MCP host as a `HashSet<String>` of permitted connection names, leaving the shared `ConnectionConfig` untouched.

**Tech Stack:** Rust, tokio-postgres (`simple_query`, `SimpleQueryMessage::CommandComplete`), `uuid` v4 for tokens, newline-delimited JSON-RPC (existing MCP protocol layer).

**Spec:** `docs/superpowers/specs/2026-07-03-mcp-write-tools-design.md`

## Global Constraints

- DML only: first keyword `INSERT` / `UPDATE` / `DELETE`; exactly one statement (a `;` is allowed only as the trailing character); reject `SELECT`, `WITH`, DDL, multi-statement. No DDL ever.
- Writes are off by default: a connection must have `"allow_mcp_writes": true`; the flag is re-checked in both `propose_write` and `apply_write`.
- No row-count limit on the mutation itself (user decision); the preview shows exact `rows_affected`.
- `propose_write` MUST roll back — the database is never modified during preview.
- The token is one-shot, TTL 5 minutes, bound to the exact SQL text; `apply_write` takes only a token, never SQL.
- Existing `run_query` stays strictly read-only (`SessionMode::ReadOnly`, `BEGIN READ ONLY` wrapper) — do not change it.
- Preview rows are capped by the existing `mcp_max_rows` (default 200); `rows_affected` is always exact.
- No `unwrap()`/`expect()` outside tests; no `let _ =` on fallible ops. Errors surface as MCP `isError` tool results, not JSON-RPC errors.
- Build/check zed only with `--features gpui_platform/runtime_shaders`. Lint with `./script/clippy -p <crate>`. Format before every commit.
- Commit trailer: `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.

## File Map

| File | Change |
|---|---|
| `crates/settings_content/src/database.rs` | Task 1: `allow_mcp_writes: Option<bool>` on `DatabaseConnectionContent` |
| `assets/settings/default.json` | Task 1: document the flag (no value change; connections stays `[]`) |
| `crates/database_mcp/src/main.rs` | Task 1: `parse_write_allowed(settings) -> HashSet<String>`; pass to host |
| `crates/database_mcp/src/write_sql.rs` | Task 2: NEW — DML classifier + UPDATE target extraction (pure) |
| `crates/database_mcp/src/token_store.rs` | Task 3: NEW — one-shot TTL token store |
| `crates/database_client/src/database_client.rs` | Task 4: `WriteKind`, `WritePreview`, `WriteOutcome`, two trait methods |
| `crates/database_client/src/postgres.rs` | Task 4: `preview_write`/`commit_write` impl on dedicated connection |
| `crates/database_client/src/fake.rs` | Task 4: fake mirror + test knobs |
| `crates/database_mcp/src/tools.rs` | Task 5: `propose_write`/`apply_write` tools, gate, definitions, dispatch |
| `crates/database_mcp/src/main.rs` | Task 5: wire `ToolHost` with `write_allowed` + token store |
| `crates/database_mcp/Cargo.toml` | Task 3/5: add `uuid` (v4) dep |
| `docs/superpowers/database-viewer-usage.md` | Task 6: document write flow + flag |

---

### Task 1: Per-connection `allow_mcp_writes` flag (settings + MCP parse)

**Files:**
- Modify: `crates/settings_content/src/database.rs` (`DatabaseConnectionContent`)
- Modify: `assets/settings/default.json` (database section docs)
- Modify: `crates/database_mcp/src/main.rs` (add `parse_write_allowed`)

**Interfaces:**
- Produces: `fn parse_write_allowed(settings: &serde_json::Value) -> std::collections::HashSet<String>` returning the set of connection names whose `allow_mcp_writes == true`.

- [ ] **Step 1: Add the settings field.** In `crates/settings_content/src/database.rs`, add to `DatabaseConnectionContent` (after `user`):

```rust
    /// Allow the MCP `apply_write` tool to commit INSERT/UPDATE/DELETE
    /// statements to this connection. Off by default; leave unset for
    /// read-only access.
    ///
    /// Default: false
    pub allow_mcp_writes: Option<bool>,
```

(Do not add it to `ConnectionConfig` in `database_client` — the flag is enforced in the MCP layer only.)

- [ ] **Step 2: Failing test for `parse_write_allowed`.** In `crates/database_mcp/src/main.rs` tests module:

```rust
#[test]
fn parse_write_allowed_collects_only_flagged_connections() {
    let settings = serde_json::json!({
        "database": { "connections": [
            { "name": "prod", "host": "h", "port": 5432, "database": "d", "user": "u" },
            { "name": "dev", "host": "h", "port": 5432, "database": "d", "user": "u", "allow_mcp_writes": true },
            { "name": "stage", "host": "h", "port": 5432, "database": "d", "user": "u", "allow_mcp_writes": false }
        ]}
    });
    let allowed = parse_write_allowed(&settings);
    assert!(allowed.contains("dev"));
    assert!(!allowed.contains("prod"));
    assert!(!allowed.contains("stage"));
    assert_eq!(allowed.len(), 1);
}

#[test]
fn parse_write_allowed_empty_without_database_section() {
    assert!(parse_write_allowed(&serde_json::json!({})).is_empty());
}
```

- [ ] **Step 3: Run to verify it fails** — `cargo test -p database_mcp parse_write_allowed` → fails (function missing).

- [ ] **Step 4: Implement `parse_write_allowed`** in `main.rs` (mirror `parse_connections`'s navigation):

```rust
fn parse_write_allowed(settings: &serde_json::Value) -> std::collections::HashSet<String> {
    let Some(connections) = settings
        .get("database")
        .and_then(|database| database.get("connections"))
        .and_then(|connections| connections.as_array())
    else {
        return std::collections::HashSet::new();
    };
    connections
        .iter()
        .filter(|connection| {
            connection
                .get("allow_mcp_writes")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .filter_map(|connection| connection.get("name")?.as_str().map(str::to_string))
        .collect()
}
```

- [ ] **Step 5: Document in `assets/settings/default.json`.** The `database.connections` default stays `[]`; add a comment line above it noting the optional `allow_mcp_writes` per-connection key (match the file's existing comment style; if the file has no inline comments for connections, leave the JSON as-is — the settings schema doc-comment from Step 1 is the source of truth). Verify the schema still loads: `cargo test -p settings` compiles.

- [ ] **Step 6: Verify + commit** — `cargo test -p database_mcp parse_write_allowed` PASS; `cargo check -p zed --features gpui_platform/runtime_shaders`; `cargo fmt -p database_mcp -p settings_content`; `./script/clippy -p database_mcp -p settings_content`. Commit `database: Add per-connection allow_mcp_writes setting`.

---

### Task 2: DML statement classifier + UPDATE target extraction

**Files:**
- Create: `crates/database_mcp/src/write_sql.rs`
- Modify: `crates/database_mcp/src/main.rs` (add `mod write_sql;`)

**Interfaces:**
- Produces:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteKind { Insert, Update, Delete }

/// Classifies a single DML statement. Errs (with a user-facing message) for
/// SELECT/WITH/DDL, empty input, or more than one statement.
pub fn classify_dml(sql: &str) -> anyhow::Result<WriteKind>;

/// Best-effort extraction of the UPDATE target table `(schema, table)` for the
/// before-image fetch. Returns None for forms we cannot parse confidently
/// (the caller then omits the before-image with a note).
pub fn extract_update_target(sql: &str) -> Option<(String, String)>;
```

- [ ] **Step 1: Write failing tests** in `write_sql.rs` (`#[cfg(test)] mod tests`):

```rust
use super::*;

#[test]
fn classifies_each_dml_verb() {
    assert_eq!(classify_dml("INSERT INTO t VALUES (1)").unwrap(), WriteKind::Insert);
    assert_eq!(classify_dml("  update t set a = 1 where id = 2 ").unwrap(), WriteKind::Update);
    assert_eq!(classify_dml("DELETE FROM t WHERE id = 1;").unwrap(), WriteKind::Delete);
}

#[test]
fn skips_leading_comments_and_whitespace() {
    assert_eq!(classify_dml("-- a comment\n  DELETE FROM t WHERE id=1").unwrap(), WriteKind::Delete);
    assert_eq!(classify_dml("/* block */\nUPDATE t SET a=1 WHERE id=1").unwrap(), WriteKind::Update);
}

#[test]
fn rejects_non_dml() {
    assert!(classify_dml("SELECT * FROM t").is_err());
    assert!(classify_dml("WITH x AS (SELECT 1) INSERT INTO t SELECT * FROM x").is_err());
    assert!(classify_dml("CREATE TABLE t (id int)").is_err());
    assert!(classify_dml("DROP TABLE t").is_err());
    assert!(classify_dml("TRUNCATE t").is_err());
    assert!(classify_dml("").is_err());
    assert!(classify_dml("   ").is_err());
}

#[test]
fn rejects_multiple_statements() {
    assert!(classify_dml("DELETE FROM t WHERE id=1; DELETE FROM t WHERE id=2").is_err());
    // A trailing semicolon is allowed:
    assert_eq!(classify_dml("DELETE FROM t WHERE id=1;").unwrap(), WriteKind::Delete);
    assert_eq!(classify_dml("DELETE FROM t WHERE id=1;   ").unwrap(), WriteKind::Delete);
}

#[test]
fn does_not_treat_semicolon_in_string_literal_as_separator() {
    // Single statement whose value contains a semicolon.
    assert_eq!(
        classify_dml("UPDATE t SET note = 'a; b' WHERE id = 1").unwrap(),
        WriteKind::Update
    );
}

#[test]
fn extracts_simple_update_targets() {
    assert_eq!(extract_update_target("UPDATE public.orders SET a=1 WHERE id=2"),
        Some(("public".into(), "orders".into())));
    assert_eq!(extract_update_target("update orders set a=1 where id=2"),
        Some(("public".into(), "orders".into())));
    assert_eq!(extract_update_target("UPDATE ONLY \"my schema\".\"my table\" SET a=1"),
        Some(("my schema".into(), "my table".into())));
}

#[test]
fn declines_complex_update_targets() {
    // FROM-join and CTE forms are not confidently parseable -> None (before-image omitted).
    assert_eq!(extract_update_target("UPDATE t SET a = b.x FROM other b WHERE t.id = b.id"),
        Some(("public".into(), "t".into()))); // target table still first token; before-fetch code guards separately
    assert_eq!(extract_update_target("INSERT INTO t VALUES (1)"), None);
}
```

- [ ] **Step 2: Run to verify failure** — `cargo test -p database_mcp write_sql` → compile error (module missing).

- [ ] **Step 3: Implement** `write_sql.rs`. Strategy: tokenize just enough. Provide a helper `strip_leading_noise(sql) -> &str` that skips whitespace and leading `--`/`/* */` comments; `single_statement(sql) -> Result<&str>` that scans respecting `'...'` string literals (double `''` escape) and `"..."` identifiers, rejecting any `;` that is not the final non-whitespace char. Then match the first ASCII-keyword case-insensitively.

```rust
use anyhow::{Result, bail};

pub fn classify_dml(sql: &str) -> Result<WriteKind> {
    let statement = single_statement(sql)?;
    let head = strip_leading_noise(statement);
    let verb = head
        .split(|c: char| c.is_whitespace() || c == '(')
        .next()
        .unwrap_or("")
        .to_ascii_uppercase();
    match verb.as_str() {
        "INSERT" => Ok(WriteKind::Insert),
        "UPDATE" => Ok(WriteKind::Update),
        "DELETE" => Ok(WriteKind::Delete),
        "" => bail!("empty statement; provide one INSERT, UPDATE, or DELETE statement"),
        other => bail!(
            "only INSERT/UPDATE/DELETE are allowed via propose_write (got `{other}`); \
             use run_query for reads, and run DDL yourself"
        ),
    }
}

// single_statement: returns the trimmed statement, erroring if a non-trailing
// `;` is found outside string/identifier quotes.
fn single_statement(sql: &str) -> Result<&str> {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        bail!("empty statement");
    }
    let bytes = trimmed.as_bytes();
    let mut in_string = false;
    let mut in_ident = false;
    for (index, &byte) in bytes.iter().enumerate() {
        match byte {
            b'\'' if !in_ident => in_string = !in_string,
            b'"' if !in_string => in_ident = !in_ident,
            b';' if !in_string && !in_ident => {
                // Allowed only if nothing but whitespace follows.
                if trimmed[index + 1..].trim().is_empty() {
                    return Ok(trimmed[..index].trim());
                }
                bail!("only a single statement is allowed; found more than one");
            }
            _ => {}
        }
    }
    Ok(trimmed)
}

fn strip_leading_noise(mut sql: &str) -> &str {
    loop {
        sql = sql.trim_start();
        if let Some(rest) = sql.strip_prefix("--") {
            sql = rest.splitn(2, '\n').nth(1).unwrap_or("");
        } else if let Some(rest) = sql.strip_prefix("/*") {
            sql = rest.splitn(2, "*/").nth(1).unwrap_or("");
        } else {
            return sql;
        }
    }
}

pub fn extract_update_target(sql: &str) -> Option<(String, String)> {
    let head = strip_leading_noise(single_statement(sql).ok()?);
    let mut rest = head.strip_prefix_ci("UPDATE")?; // see helper below
    rest = rest.trim_start();
    if let Some(after_only) = rest.strip_prefix_ci("ONLY") {
        rest = after_only.trim_start();
    }
    parse_qualified_name(rest)
}
```

Add small helpers: a case-insensitive `strip_prefix_ci` (extension trait or free fn) and `parse_qualified_name(&str) -> Option<(String, String)>` that reads an optionally-quoted `schema.table` or bare `table` (defaulting schema to `"public"`), stopping at whitespace/`(`/`SET`. Keep it conservative: on any ambiguity return `None`. Because `classify_dml` guards the verb, `extract_update_target` only runs for confirmed UPDATEs; the before-image code (Task 4) additionally guards on PK availability.

- [ ] **Step 4: Verify** — `cargo test -p database_mcp write_sql` PASS.
- [ ] **Step 5: Format, lint, commit** — `database_mcp: Add DML statement classifier and UPDATE-target extraction`.

---

### Task 3: One-shot TTL token store

**Files:**
- Create: `crates/database_mcp/src/token_store.rs`
- Modify: `crates/database_mcp/src/main.rs` (`mod token_store;`)
- Modify: `crates/database_mcp/Cargo.toml` (add `uuid` dep)

**Interfaces:**
- Produces:

```rust
#[derive(Debug, Clone)]
pub struct Proposal {
    pub connection: String,
    pub database: String,
    pub sql: String,
}

pub struct TokenStore {
    ttl: std::time::Duration,
    entries: std::collections::HashMap<String, (Proposal, std::time::Instant)>,
}

impl TokenStore {
    pub fn new(ttl: std::time::Duration) -> Self;
    /// Stores a proposal, returns its fresh one-shot token (uuid v4).
    pub fn insert(&mut self, proposal: Proposal, now: std::time::Instant) -> String;
    /// Removes and returns the proposal iff the token exists and is not expired.
    /// Expired or unknown tokens return None. Also prunes expired entries.
    pub fn take(&mut self, token: &str, now: std::time::Instant) -> Option<Proposal>;
    pub fn ttl_seconds(&self) -> u64;
}
```

(The `now: Instant` parameter makes TTL unit-testable without a real clock; production callers pass `Instant::now()`.)

- [ ] **Step 1: Add `uuid` to `crates/database_mcp/Cargo.toml`** dependencies: `uuid = { workspace = true }` (workspace already pins v4 feature).

- [ ] **Step 2: Failing tests** in `token_store.rs`:

```rust
use super::*;
use std::time::{Duration, Instant};

fn proposal() -> Proposal {
    Proposal { connection: "dev".into(), database: "shop".into(), sql: "DELETE FROM t WHERE id=1".into() }
}

#[test]
fn token_is_one_shot() {
    let mut store = TokenStore::new(Duration::from_secs(300));
    let now = Instant::now();
    let token = store.insert(proposal(), now);
    assert!(store.take(&token, now).is_some());
    assert!(store.take(&token, now).is_none(), "second take must fail");
}

#[test]
fn expired_token_is_rejected() {
    let mut store = TokenStore::new(Duration::from_secs(300));
    let now = Instant::now();
    let token = store.insert(proposal(), now);
    let later = now + Duration::from_secs(301);
    assert!(store.take(&token, later).is_none());
}

#[test]
fn unknown_token_is_none() {
    let mut store = TokenStore::new(Duration::from_secs(300));
    assert!(store.take("nope", Instant::now()).is_none());
}

#[test]
fn distinct_tokens_are_independent() {
    let mut store = TokenStore::new(Duration::from_secs(300));
    let now = Instant::now();
    let a = store.insert(proposal(), now);
    let b = store.insert(proposal(), now);
    assert_ne!(a, b);
    assert!(store.take(&a, now).is_some());
    assert!(store.take(&b, now).is_some());
}
```

- [ ] **Step 3: Run to verify failure** — `cargo test -p database_mcp token_store` → compile error.

- [ ] **Step 4: Implement** using `uuid::Uuid::new_v4().to_string()` for tokens; `take` first drops entries whose `now.duration_since(created) > ttl`, then removes-and-returns the requested one if still present.

- [ ] **Step 5: Verify + commit** — `cargo test -p database_mcp token_store` PASS; fmt/clippy; commit `database_mcp: Add one-shot TTL token store`.

---

### Task 4: `database_client` write preview/commit methods

**Files:**
- Modify: `crates/database_client/src/database_client.rs` (types + trait)
- Modify: `crates/database_client/src/postgres.rs` (impl)
- Modify: `crates/database_client/src/fake.rs` (mirror + knobs)

**Interfaces:**
- Produces:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteKind { Insert, Update, Delete }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WritePreview {
    pub rows_affected: u64,
    pub columns: Vec<String>,
    // Per spec: Insert -> after=Some, before=None; Delete -> before=Some (the
    // rows that would be deleted), after=None; Update -> after=Some (post-update
    // via RETURNING), before=Some (pre-update via PK) or None with a note.
    pub before: Option<Vec<Vec<Option<String>>>>,
    pub after: Option<Vec<Vec<Option<String>>>>,
    pub preview_truncated: bool,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WriteOutcome {
    pub rows_affected: u64,
    pub columns: Vec<String>,
    pub returned: Vec<Vec<Option<String>>>,
}

// On the DatabaseClient trait (after apply_edits):
async fn preview_write(
    &self,
    database: &str,
    sql: &str,
    kind: WriteKind,
    update_target: Option<TableRef>, // Some for Update when a target was parsed
    max_rows: usize,
) -> Result<WritePreview>;

async fn commit_write(&self, database: &str, sql: &str) -> Result<WriteOutcome>;
```

**Notes for the implementer:**
- `WriteKind` lives here (shared type); `database_mcp::write_sql::WriteKind` from Task 2 should be REPLACED by re-using `database_client::WriteKind` — update Task 2's module to `pub use database_client::WriteKind;` or import it (do this as part of Task 4 to avoid a duplicate enum; adjust `classify_dml`'s return type accordingly and fix its tests).
- Both methods require `SessionMode::ReadWrite`; bail otherwise (mirror `apply_edits`'s guard).
- Use `connect_dedicated(database)` (short-lived, not registered in `cancel_tokens` — same rationale as `apply_edits`).
- **RETURNING:** apply a deterministic transform `with_returning(sql)` — if the statement already contains a top-level `RETURNING` (case-insensitive word match), use as-is; else append ` RETURNING *`. BOTH preview and commit use the SAME transform so the previewed and committed statements match.
- Reuse the existing message parser: `parse_query_messages` already yields columns/rows and `CommandComplete(count)`. Extend or add a sibling that also returns `rows_affected: u64` (the `count` from `CommandComplete`). Do not truncate `rows_affected`; truncate only the row vectors to `max_rows`, setting `preview_truncated` when more rows existed.
- **preview_write flow:** BEGIN → run `with_returning(sql)` → capture (columns, rows, rows_affected). Then:
  - `Insert` → `after = Some(rows)`, `before = None`.
  - `Delete` → `before = Some(rows)` (the doomed rows, captured by RETURNING before the rollback), `after = None`.
  - `Update` → `after = Some(rows)`. Then ROLLBACK. If `update_target` is Some and `table_structure(target)` has PK columns AND all PK columns are present in `columns` AND the statement's SET clause does not assign a PK column: build `SELECT * FROM "schema"."table" WHERE (pk1, pk2, ...) IN ((v11, v12), ...)` from the PK values in the returned rows, run it (post-rollback → original values), set `before = Some(that)`. Else `before = None` and `note = Some("before-image unavailable (no primary key, PK changed, or unparsed target)")`. Detecting "SET assigns a PK column" can be conservative: if uncertain, still fetch by PK from the RETURNING (the RETURNING PK is the post-update PK; if the PK didn't change it matches the original row, which is what we want; if the PK DID change, the before-fetch finds nothing → emit the note instead of wrong data). Simplest correct rule: fetch before by PK; if the fetched row count != rows_affected, discard `before` and set the note.
  - Always ROLLBACK before returning (even on the Insert/Delete paths, do BEGIN→stmt→ROLLBACK). On any error, best-effort ROLLBACK + return the error.
- **commit_write flow:** BEGIN → run `with_returning(sql)` → capture → COMMIT (ROLLBACK + return error on failure). Return `WriteOutcome { rows_affected, columns, returned: rows }` (returned not truncated here, or truncate to a sane cap — reuse max_rows only in preview; for commit, return all RETURNING rows but the tool layer may cap display).
- **fake.rs:** add public fields `write_preview: WritePreview`, `write_outcome: WriteOutcome`, and error knobs `preview_write_error`/`commit_write_error` (mirrors `set_run_query_error` pattern); record calls (`preview_write {db} kind=.. sql=..`, `commit_write {db} sql=..`) so the tool tests can assert the exact SQL forwarded. `preview_write`/`commit_write` return the canned values (respecting the error knobs) after recording.

- [ ] **Step 1: Failing unit tests** (`sql.rs`/dedicated tests for `with_returning`; `fake.rs` tests for the mirror). Live Postgres tests are `#[ignore]` like the existing ones. Example `with_returning` test:

```rust
#[test]
fn with_returning_appends_when_absent() {
    assert_eq!(with_returning("DELETE FROM t WHERE id=1"), "DELETE FROM t WHERE id=1 RETURNING *");
}
#[test]
fn with_returning_respects_existing_clause() {
    assert_eq!(with_returning("INSERT INTO t VALUES (1) RETURNING id"), "INSERT INTO t VALUES (1) RETURNING id");
}
```

Live (ignored) tests against `zed-db-test`/`shop`: preview of an UPDATE returns before+after and does NOT modify the row (a follow-up SELECT shows the original); DELETE preview returns the doomed rows and the row still exists after preview; commit actually persists; table without PK → `before == None` and `note` set.

- [ ] **Step 2: Run to verify failure.** `cargo test -p database_client` (unit + fake) fails to compile.
- [ ] **Step 3: Implement** types, `with_returning`, trait methods, postgres impl, fake mirror.
- [ ] **Step 4: Verify** — `cargo test -p database_client` PASS (ignored live tests skipped); run the ignored live tests manually against Docker in Task 6.
- [ ] **Step 5: Format, lint, commit** — `database_client: Add transactional write preview and commit`.

---

### Task 5: `propose_write` / `apply_write` MCP tools

**Files:**
- Modify: `crates/database_mcp/src/tools.rs` (definitions, dispatch, two methods, gate)
- Modify: `crates/database_mcp/src/main.rs` (build `ToolHost` with `write_allowed` + `TokenStore`)

**Interfaces:**
- Consumes: `write_sql::{classify_dml, extract_update_target}`, `database_client::WriteKind`, `token_store::{TokenStore, Proposal}`, `DatabaseClient::{preview_write, commit_write}`, `HashSet<String>` write-allowed set.
- `ToolHost` gains fields `write_allowed: HashSet<String>` and `tokens: TokenStore`; `ToolHost::new` signature extends to accept them (update `main.rs` and existing test constructors — pass an empty set and a `TokenStore::new(Duration::from_secs(300))` where writes aren't under test).

**Behavior:**
- `tool_definitions()` gains two entries: `propose_write { connection, database, sql }` and `apply_write { token }`, with descriptions stating: DML only, preview is rolled back, token is one-shot/5-min, apply commits.
- `call()` dispatch adds `"propose_write" => self.propose_write(arguments).await` and `"apply_write" => self.apply_write(arguments).await`.
- `propose_write`: read `connection`, `database`, `sql`. Gate: if `!self.write_allowed.contains(connection)` → `bail!("writes are disabled for connection `{connection}`; set allow_mcp_writes: true in settings")`. `classify_dml(sql)?` → kind; for Update, `update_target` = `extract_update_target(sql)` mapped to `TableRef { database, schema, name }`. Resolve client (`self.client(config, database)?`) and call `preview_write(database, sql, kind, update_target, self.max_rows)`. On success, `let token = self.tokens.insert(Proposal { connection, database, sql }, Instant::now())`. Build the JSON result: `{ token, statement_kind, rows_affected, columns, before, after, preview_truncated, expires_in_seconds: self.tokens.ttl_seconds(), note }` (emit `before`/`after` as JSON `null` when None, per the per-kind rules above).
- `apply_write`: read `token`. `let Some(proposal) = self.tokens.take(token, Instant::now()) else { bail!("unknown or expired token; call propose_write again") }`. Re-check gate: `if !self.write_allowed.contains(&proposal.connection) { bail!("writes are disabled for connection `{}`", proposal.connection) }`. Resolve client for `(proposal.connection, proposal.database)` and call `commit_write(&proposal.database, &proposal.sql)`. Return `{ rows_affected, columns, returned, rows_affected_matches_preview }` — to compute the match flag, store the previewed `rows_affected` in the `Proposal` (add a field) and compare.
- The dedicated client for writes must be built in `SessionMode::ReadWrite`. NOTE: `build_client` in `main.rs` hard-codes `ReadOnly` for the cached read path — the write path needs a ReadWrite client. Simplest: give the write tools their own dedicated client built on demand in ReadWrite mode (do NOT reuse the read-only cache). Add a `write_client_factory` to `ToolHost` (or a mode parameter to the factory) that builds `SessionMode::ReadWrite`. Keep `run_query`/`list_*`/`describe_table` on the existing ReadOnly cached path unchanged.

- [ ] **Step 1: Failing protocol tests** in `tools.rs` tests, using `FakeDatabaseClient` seeded via Task 4's knobs. Cover:
  - `propose_write` on a non-allowed connection → `isError` with "writes are disabled".
  - `propose_write` of a SELECT → `isError` (classifier).
  - `propose_write` happy path → returns a `token`, correct `rows_affected`, `statement_kind`, and `after`/`before`/`deleted` per kind; assert the fake received the exact SQL.
  - `apply_write` with an unknown token → `isError`.
  - `apply_write` with the token from a prior `propose_write` → commits (fake `commit_write` called with the same SQL); second `apply_write` with the same token → `isError` (one-shot).
  - `apply_write` re-checks the gate (flip `write_allowed` off between propose and apply → `isError`).

- [ ] **Step 2: Run to verify failure** — `cargo test -p database_mcp` (compile/logic failures).
- [ ] **Step 3: Implement** the two methods, definitions, dispatch, `ToolHost` fields, `main.rs` wiring (`parse_write_allowed`, `TokenStore::new(query_timeout?-no: fixed Duration::from_secs(300))`, ReadWrite write-client path).
- [ ] **Step 4: Verify** — `cargo test -p database_mcp` PASS; `cargo build -p database_mcp`.
- [ ] **Step 5: Format, lint, commit** — `database_mcp: Add two-phase propose_write and apply_write tools`.

---

### Task 6: E2E live verification, docs, final checks

**Files:**
- Modify: `docs/superpowers/database-viewer-usage.md`

- [ ] **Step 1: Whole-crate checks.**

```bash
cargo test -p database_client -p database_mcp -p settings_content
./script/clippy -p database_client -p database_mcp -p settings_content
cargo check -p zed --features gpui_platform/runtime_shaders
cargo build -p database_mcp
```

- [ ] **Step 2: Run the ignored live tests** against Docker (`docker start zed-db-test`; db `shop`): `cargo test -p database_client -- --ignored preview_write commit_write` (or the exact test names). Confirm: UPDATE preview returns before+after and leaves the row unchanged; DELETE preview leaves the row present; commit persists; no-PK table yields `before=None` + note. Record output in the report.

- [ ] **Step 3: End-to-end MCP smoke** against Docker, driving the built binary over stdio (mirror the existing stage-1 e2e approach). With a temp settings file granting `allow_mcp_writes: true` on a test connection: `initialize` → `propose_write` an `UPDATE ... WHERE id = <x>` → assert the result shows before/after and the token; verify via a separate `run_query` that the row is UNCHANGED (rollback held) → `apply_write` the token → verify via `run_query` the row is now changed → `apply_write` the same token again → `isError`. Also: `propose_write` on a connection WITHOUT the flag → `isError`. Capture the transcript in the report.

- [ ] **Step 4: Docs.** In `docs/superpowers/database-viewer-usage.md`, add an MCP-writes section: the flag (`allow_mcp_writes`, off by default, per connection), the two-phase flow (agent calls `propose_write`, reviews the before/after preview, then `apply_write` with the token; the client's tool-approval prompt is the final gate), DML-only, one-shot 5-minute token, preview is rolled back. Note that `run_query` remains read-only.

- [ ] **Step 5: Commit** — `database_mcp: Document write tools and verify end-to-end` (docs + any test-only touch-ups).

---

## Execution notes for the controller

- Order 1→6; Task 4 back-fills the shared `WriteKind` into Task 2's module (call it out to the Task 4 implementer).
- Security-sensitive feature: the final whole-branch review must specifically probe (a) the rollback guarantee in preview, (b) the gate being re-checked at apply, (c) token one-shot/TTL, (d) DML classifier bypasses (comment tricks, string-literal semicolons, `WITH`-prefixed writes, encoded statements), (e) the write path never reusing the ReadOnly cached client.
- Ledger: append stage-4 lines to `.superpowers/sdd/progress.md`.
