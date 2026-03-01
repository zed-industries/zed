---
phase: 04-pruning-and-maintenance
plan: "01"
subsystem: database
tags: [persistent-undo, pruning, sqlez, editor, cleanup]

dependency_graph:
  requires:
    - phase: 02-persistence-schema-and-settings
      provides: delete_undo_history DB method and PersistentUndoSettings gate
    - phase: 03-core-write-and-restore
      provides: write_undo_history and restore_undo_history lifecycle wiring
  provides:
    - get_undo_history_paths query in persistence.rs returning all stored paths for a workspace
    - prune_undo_history free function deleting DB rows and blob files for absent files
    - Editor::cleanup wired to run prune_undo_history at workspace restore
    - delete_undo_history has a production caller (prune_undo_history)
    - Standalone-abs deserialize branch calls restore_undo_history exactly once (via added_to_workspace)
  affects:
    - Phase 4 (maintenance): this is the pruning plan; subsequent plans build on clean state

tech-stack:
  added: []
  patterns:
    - Free function prune_undo_history follows same settings-gate-before-background-spawn pattern as write_undo_history
    - smol::fs::metadata used for filesystem existence check (any error treated as absent)
    - blob removal ignores NotFound (blob may not exist if write was interrupted)

key-files:
  created: []
  modified:
    - crates/editor/src/persistence.rs
    - crates/editor/src/items.rs
    - .planning/phases/02-persistence-schema-and-settings/02-02-SUMMARY.md

key-decisions:
  - "prune_undo_history is a free function (not an Editor method) — cleanup is a static method on SerializableItem with no &self, so free function placement matches blob_path_for and compute_content_hash"
  - "smol::fs::metadata any-error-is-absent: treats all IO errors (not just NotFound) as file missing — avoids silently skipping files on permission errors while keeping pruning best-effort"
  - "query! macro requires no trailing comma after last arg: get_undo_history_paths(workspace_id: WorkspaceId) not workspace_id: WorkspaceId,"

patterns-established:
  - "prune_undo_history pattern: settings gate on foreground thread -> background_spawn -> DB query all paths -> per-path fs check -> delete DB row + blob for each absent path"

requirements-completed: [MAINT-01]

duration: ~10min
completed: "2026-03-01"
---

# Phase 4 Plan 1: Pruning and Maintenance Summary

**Startup pruning of orphaned undo history via prune_undo_history wired into Editor::cleanup, deleting DB rows and blob files for files no longer on disk, plus closure of double restore call and missing SUMMARY frontmatter.**

## Performance

- **Duration:** ~10 min
- **Started:** 2026-03-01T20:21:31Z
- **Completed:** 2026-03-01T20:31:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Added `get_undo_history_paths` query to `persistence.rs` returning all stored `abs_path` values for a workspace
- Added `prune_undo_history` free function that gates on settings, then background-spawns a loop checking each path against the filesystem and deleting DB rows + blob files for absent files
- Wired `prune_undo_history` into `Editor::cleanup` alongside `delete_unloaded_items`, giving `delete_undo_history` its first production call site
- Removed duplicate `restore_undo_history` call from the standalone-abs `None =>` deserialize branch (it was already called via `added_to_workspace`)
- Added `requirements-completed: [CONF-01, CONF-02]` to `02-02-SUMMARY.md` frontmatter to match all other SUMMARY files

## Task Commits

Each task was committed atomically:

1. **Task 1: Add get_undo_history_paths query and prune_undo_history, wire into cleanup** - `a7c7803e55` (feat)
2. **Task 2: Fix double restore_undo_history call and 02-02-SUMMARY.md frontmatter** - `0fd09db0dc` (fix)

## Files Created/Modified

- `crates/editor/src/persistence.rs` - Added `get_undo_history_paths` query returning `Vec<PathBuf>` for workspace
- `crates/editor/src/items.rs` - Added `prune_undo_history` free function; updated `cleanup` to call it; removed duplicate `restore_undo_history` from standalone-abs branch
- `.planning/phases/02-persistence-schema-and-settings/02-02-SUMMARY.md` - Added `requirements-completed: [CONF-01, CONF-02]` to frontmatter

## Decisions Made

1. **prune_undo_history as free function:** Placed alongside `blob_path_for` and `compute_content_hash` at module level rather than in `impl Editor`, because `cleanup` is a static method on `SerializableItem` — it receives no `&self` and cannot call instance methods directly.

2. **Any-error-is-absent for smol::fs::metadata:** Treats all IO errors as file absent, not just `NotFound`. This is correct for pruning: if a file is inaccessible we should not accumulate stale records, and the consequence of a false positive (pruning a live file's history) is low for a best-effort operation.

3. **query! macro trailing comma:** The `query!` macro's `$($arg:ident: $arg_type:ty),+` pattern does not accept a trailing comma after the last argument. Fixed by removing the trailing comma from `get_undo_history_paths(workspace_id: WorkspaceId,)`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed trailing comma from query! macro argument**
- **Found during:** Task 1 (cargo check -p editor)
- **Issue:** The plan's `query!` block showed `workspace_id: WorkspaceId,` with trailing comma, but the macro requires `$($arg:ident: $arg_type:ty),+` which does not tolerate trailing commas — compiler error: "no rules expected `)`"
- **Fix:** Removed the trailing comma: `workspace_id: WorkspaceId` (no trailing comma)
- **Files modified:** `crates/editor/src/persistence.rs`
- **Verification:** `cargo check -p editor` passes after fix
- **Committed in:** a7c7803e55 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug — macro syntax)
**Impact on plan:** Trivial one-character fix, no scope change.

## Issues Encountered

None beyond the trailing comma macro issue documented above.

## Next Phase Readiness

- Pruning infrastructure complete: orphaned undo history will be cleaned up at each workspace restore
- MAINT-01 satisfied; Phase 4 plan 1 complete
- No blockers for any subsequent Phase 4 plans

## Self-Check: PASSED

Files verified:
- FOUND: crates/editor/src/persistence.rs (get_undo_history_paths query)
- FOUND: crates/editor/src/items.rs (prune_undo_history function + cleanup wiring)
- FOUND: .planning/phases/04-pruning-and-maintenance/04-01-SUMMARY.md

Commits verified:
- FOUND: a7c7803e55 (Task 1 - feat: add get_undo_history_paths and prune_undo_history)
- FOUND: 0fd09db0dc (Task 2 - fix: remove duplicate restore call, fix frontmatter)

---
*Phase: 04-pruning-and-maintenance*
*Completed: 2026-03-01*
