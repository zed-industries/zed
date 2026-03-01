---
phase: 02-persistence-schema-and-settings
plan: 01
subsystem: database
tags: [sqlite, sqlez, editor, undo-history, migrations]

# Dependency graph
requires: []
provides:
  - "undo_history SQLite table in EditorDb with composite PRIMARY KEY(workspace_id, abs_path)"
  - "get_undo_history_meta: query by (workspace_id, abs_path) returning content_hash, mtime, last_accessed_at"
  - "save_undo_history_meta: upsert with CURRENT_TIMESTAMP for last_accessed_at"
  - "delete_undo_history: remove single row by (workspace_id, abs_path)"
affects: [03-save-and-restore, 04-settings-and-pruning]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "abs_path stored as BLOB in sqlez tables (sqlez Path bind uses as_encoded_bytes, not text)"
    - "INSERT ON CONFLICT DO UPDATE for upsert in sqlez write closures"
    - "query! macro for SELECT, write() closure with exec_bound for INSERT/DELETE with CURRENT_TIMESTAMP"

key-files:
  created: []
  modified:
    - "crates/editor/src/persistence.rs"

key-decisions:
  - "abs_path stored as BLOB not TEXT: sqlez &Path bind impl uses as_encoded_bytes() (BLOB), so STRICT mode with TEXT column causes runtime failure — removed STRICT, used BLOB column type to match sqlez behavior"
  - "delete_undo_history takes Arc<Path> not &Path: move closure in self.write() requires owned value, consistent with delete_file_folds pattern"
  - "No STRICT table constraint: file_folds also omits STRICT because path binding is BLOB; adding STRICT would break at runtime for any path-keyed table using sqlez Bind"

patterns-established:
  - "Upsert pattern: INSERT INTO ... VALUES ... ON CONFLICT(workspace_id, abs_path) DO UPDATE SET ... last_accessed_at = CURRENT_TIMESTAMP"
  - "TDD RED commit before implementing methods: test(02-01) commit preceded feat(02-01) commit"

requirements-completed: [INFRA-02]

# Metrics
duration: 13min
completed: 2026-03-01
---

# Phase 2 Plan 1: undo_history SQLite table and query methods Summary

**undo_history table in EditorDb with get/save/delete methods using sqlez query! macro and write() closures, keyed on (workspace_id, abs_path) with CASCADE delete and last_accessed_at timestamp**

## Performance

- **Duration:** 13 min
- **Started:** 2026-03-01T17:48:35Z
- **Completed:** 2026-03-01T18:01:35Z
- **Tasks:** 1 (TDD: 2 commits — test then feat)
- **Files modified:** 1

## Accomplishments
- Added 10th migration entry to EditorDb::MIGRATIONS creating the undo_history table
- Implemented get_undo_history_meta via query! macro returning (content_hash, mtime_seconds, mtime_nanos, last_accessed_at)
- Implemented save_undo_history_meta with INSERT ON CONFLICT DO UPDATE upsert setting CURRENT_TIMESTAMP
- Implemented delete_undo_history for single-row removal
- All 5 test cases pass: insert+get, upsert, delete returns None, two-path independence, last_accessed_at non-empty

## Task Commits

TDD task committed in two steps:

1. **RED - Failing tests** - `9fee9be0d8` (test)
2. **GREEN - Implementation** - `babcfe4612` (feat)

## Files Created/Modified
- `crates/editor/src/persistence.rs` - Added undo_history migration (migration index 9), get_undo_history_meta, save_undo_history_meta, delete_undo_history methods, and test_save_and_get_undo_history_meta test

## Decisions Made
- **abs_path as BLOB not TEXT**: sqlez's `Bind for &Path` uses `as_encoded_bytes()` which binds as SQLite BLOB. The original plan specified TEXT with STRICT mode, but that causes a runtime error "cannot store BLOB value in TEXT column". Switching to BLOB (without STRICT) matches the sqlez pattern used by the editors table path column and file_folds path column.
- **delete_undo_history takes Arc<Path>**: The `self.write(move |conn| ...)` closure requires owned data. `Arc<Path>` is the established pattern in this file (see delete_file_folds), avoiding a PathBuf clone when an Arc is already available from callers.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Changed abs_path column from TEXT to BLOB; removed STRICT**
- **Found during:** Task 1 (GREEN phase - test run)
- **Issue:** Plan specified `abs_path TEXT NOT NULL` with STRICT mode. sqlez's `Bind for &Path` calls `as_encoded_bytes()` which binds as BLOB. SQLite STRICT mode enforces exact type matching, causing runtime error on INSERT.
- **Fix:** Changed column definition to `abs_path BLOB NOT NULL` and removed `STRICT` from CREATE TABLE
- **Files modified:** crates/editor/src/persistence.rs
- **Verification:** Test passes: `cargo test -p editor persistence::tests::test_save_and_get_undo_history_meta`
- **Committed in:** babcfe4612 (GREEN commit)

**2. [Rule 1 - Bug] Changed delete_undo_history signature from &Path to Arc<Path>**
- **Found during:** Task 1 (GREEN phase - cargo check)
- **Issue:** Plan showed `delete_undo_history(workspace_id: WorkspaceId, abs_path: &Path)` but `self.write(move |conn| ...)` requires captured values to be `'static`. Converting to PathBuf caused type inference ambiguity in exec_bound.
- **Fix:** Changed parameter to `Arc<Path>` matching the delete_file_folds pattern; updated test call sites
- **Files modified:** crates/editor/src/persistence.rs
- **Verification:** cargo check -p editor passes, test passes
- **Committed in:** babcfe4612 (GREEN commit)

---

**Total deviations:** 2 auto-fixed (both Rule 1 - bugs in plan's implementation spec)
**Impact on plan:** Both fixes necessary for correctness. The BLOB/TEXT mismatch is a fundamental sqlez binding constraint. No scope creep.

## Issues Encountered
- cmake was not installed, blocking compilation of tree-sitter (wasmtime-c-api-impl build dep). Resolved by installing cmake via brew. This was a pre-existing environment gap unrelated to the plan.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 3 (save-and-restore) can use get_undo_history_meta, save_undo_history_meta, and delete_undo_history directly
- Phase 4 (settings-and-pruning) can query last_accessed_at for LRU pruning
- No blockers

---
*Phase: 02-persistence-schema-and-settings*
*Completed: 2026-03-01*
