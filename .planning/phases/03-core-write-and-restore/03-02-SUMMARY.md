---
phase: 03-core-write-and-restore
plan: 02
subsystem: editor
tags: [undo-history, persistence, sqlite, postcard, sha256, gpui, async]

# Dependency graph
requires:
  - phase: 03-01
    provides: write_undo_history, blob_path_for, compute_content_hash in items.rs
  - phase: 02-01
    provides: get_undo_history_meta DB query, undo_history table
  - phase: 01-02
    provides: decode_history, text::Buffer::restore_history
provides:
  - restore_undo_history method on Editor in items.rs
  - restore path wired into added_to_workspace (manual file open)
  - restore path wired into all four deserialize branches (workspace restart)
  - restore_history delegating method on language::Buffer
affects: [04-eviction-and-settings, testing]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "restore_undo_history follows same cx.spawn_in + detach_and_log_err pattern as load_folds_from_db"
    - "language::Buffer.restore_history delegates to self.text.restore_history (same pattern as forget_transaction, merge_transactions)"
    - "Hash validation guard: compute current hash in read_with closure, compare to stored_hash, discard on mismatch"

key-files:
  created: []
  modified:
    - crates/editor/src/items.rs
    - crates/language/src/buffer.rs

key-decisions:
  - "language::Buffer needed a delegating restore_history method — text::Buffer has no DerefMut so direct mutation via Deref is impossible; delegation via self.text is the established pattern in language::Buffer"
  - "restore_history call placed after read_metadata_from_db at every deserialize branch — consistent with the plan's intent that both DB operations happen at editor construction time"
  - "read_with closure returns Option<String> for current_hash — None case handled with early Ok(()) return, not ? propagation, since buffer lacking singleton is a graceful no-op"

patterns-established:
  - "Restore call site pattern: read_metadata_from_db then restore_undo_history, same arguments (workspace_id, window, cx)"
  - "Async restore task: DB meta read (synchronous sqlez) → blob read (async smol::fs) → hash validate → decode → foreground update_in"

requirements-completed: [PERS-01, PERS-02, PERS-04, PERS-06]

# Metrics
duration: 15min
completed: 2026-03-01
---

# Phase 3 Plan 2: Restore Undo History Summary

**restore_undo_history method wired into Editor's added_to_workspace and all four deserialize branches, completing the read-back path for persistent undo/redo across tab close and app restart**

## Performance

- **Duration:** 15 min
- **Started:** 2026-03-01T18:10:00Z
- **Completed:** 2026-03-01T18:25:00Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Added `restore_history` delegating method to `language::Buffer` (required since `language::Buffer` has no `DerefMut` to `text::Buffer`)
- Implemented `restore_undo_history` on Editor: reads DB metadata row, validates SHA-256 content hash, decodes postcard blob, calls `buffer.restore_history` on the foreground thread via `update_in`
- Wired restore call into `added_to_workspace` (manual file open path)
- Wired restore call into all four `deserialize` branches (workspace restart path: no-abs+contents, worktree-abs, standalone-abs, no-abs+no-contents)

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement restore_undo_history and wire into added_to_workspace** - `ec04457882` (feat)
2. **Task 2: Wire restore_undo_history into deserialize for workspace restart path** - `744d7488dc` (feat)

## Files Created/Modified
- `crates/editor/src/items.rs` - Added `restore_undo_history` method, wired into `added_to_workspace` and all four `deserialize` branches
- `crates/language/src/buffer.rs` - Added `restore_history` delegating method to `language::Buffer`

## Decisions Made
- `language::Buffer` does not implement `DerefMut` to `text::Buffer`, so calling `restore_history` on `&mut language::Buffer` requires adding a delegating method. Added `pub fn restore_history(...)` that calls `self.text.restore_history(...)`, consistent with how `forget_transaction` and `merge_transactions` are implemented.
- In the async spawn closure, `this.read_with(cx, ...)` (not `&cx`) and `this.update_in(cx, ...)` (not `&cx`) — `cx` is already `&mut AsyncWindowContext`.
- The `read_with` closure returns `Option<String>`. The `None` case (buffer has no singleton) is handled with an explicit `match` arm returning `Ok(())`, not `??` (which would require `Option` to be `Result`).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added restore_history to language::Buffer**
- **Found during:** Task 1 (implement restore_undo_history)
- **Issue:** The plan's code called `buffer.restore_history(...)` on `&mut language::Buffer` inside an `update` closure. `language::Buffer` implements only `Deref` (not `DerefMut`) to `text::Buffer`, so mutable text methods cannot be called through deref. The code would not compile.
- **Fix:** Added `pub fn restore_history(&mut self, ...)` to `language::Buffer` that delegates to `self.text.restore_history(...)`. This is the established pattern for all mutable text::Buffer methods exposed by language::Buffer (e.g., `forget_transaction`, `merge_transactions`).
- **Files modified:** `crates/language/src/buffer.rs`
- **Verification:** `cargo check -p language` passes
- **Committed in:** `ec04457882` (part of Task 1 commit)

**2. [Rule 1 - Bug] Fixed `&cx` → `cx` in async spawn closures and `??` → explicit match**
- **Found during:** Task 1 (implement restore_undo_history)
- **Issue:** The plan's pseudo-code used `this.read_with(&cx, ...)` and `this.update_in(&cx, ...)`. In a `cx.spawn_in` closure, `cx` is `&mut AsyncWindowContext` — passing `&cx` creates `&&mut AsyncWindowContext` which does not satisfy the `AppContext` bound. Also, `??` to unwrap `Result<Option<T>>` fails because the inner `?` tries to unwrap `Option` in a `Result`-returning closure.
- **Fix:** Changed to `this.read_with(cx, ...)` and `this.update_in(cx, ...)`. Replaced `??` with explicit `match` on `Ok(Some(hash)) => hash, Ok(None) => return Ok(()), Err(err) => return Err(err)`.
- **Files modified:** `crates/editor/src/items.rs`
- **Verification:** `cargo check -p editor` passes
- **Committed in:** `ec04457882` (part of Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 Rule 1 bugs in plan pseudo-code)
**Impact on plan:** Both fixes required for compilation correctness. No scope creep.

## Issues Encountered
- Plan pseudo-code contained type errors (reference mutability mismatch, Option/Result confusion). Fixed inline per deviation Rule 1.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Core write and restore paths are complete. Undo history is saved on serialize/save and restored on added_to_workspace and deserialize.
- Phase 4 (eviction and settings) can now add max-age eviction, storage limits, and UI for the feature flag.
- Outstanding warning: `delete_undo_history` (from Phase 02-01) is unused — Phase 4 eviction logic will use it.

---
*Phase: 03-core-write-and-restore*
*Completed: 2026-03-01*
