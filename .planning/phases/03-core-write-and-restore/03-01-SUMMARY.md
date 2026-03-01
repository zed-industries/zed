---
phase: 03-core-write-and-restore
plan: 01
subsystem: database
tags: [sha2, hex, undo-history, persistence, rope, sqlite, background-spawn]

# Dependency graph
requires:
  - phase: 02-persistence-schema-and-settings
    provides: save_undo_history_meta DB method, PersistentUndoSettings struct, undo_history SQLite table
  - phase: 01-text-layer-api
    provides: encode_history function in text::history_serde, undo_stack/redo_stack/operations on text::Buffer
provides:
  - write_undo_history method on Editor that serializes undo/redo stack to binary blob and upserts DB row
  - blob_path_for utility: SHA-256 hash of abs_path => deterministic .bin path under database_dir/undo_history/
  - compute_content_hash utility: SHA-256 hash over rope chunks for content fingerprinting
  - serialize lifecycle wiring: write_undo_history called from Editor::serialize before spawn closure
affects:
  - 03-02-restore: reads blob_path_for and content_hash at restore time

# Tech tracking
tech-stack:
  added: [sha2 = "0.10" (workspace dep), hex = "0.4.3" (workspace dep)]
  patterns:
    - "background_spawn for all I/O: dir creation, blob write, DB upsert — foreground only reads entity state"
    - "task.detach() for best-effort fire-and-forget write from serialize lifecycle"
    - "is_dirty() guard before write prevents hash mismatches — only write when buffer matches disk"
    - "max_entries truncation via undo_stack tail slice before encoding"

key-files:
  created: []
  modified:
    - crates/editor/Cargo.toml
    - crates/editor/src/items.rs

key-decisions:
  - "write_undo_history placed in items.rs impl Editor block (not editor.rs) — serialize lifecycle lives in items.rs and write_undo_history is only called from there"
  - "use settings::Settings as _ scoped inside write_undo_history to avoid polluting items.rs namespace with a trait import"
  - "blob_path and compute_content_hash are module-level fns (not methods) — pure utilities with no entity/context access"
  - "Task detach on write_undo_history result — best-effort write; next save overwrites if app exits early"

patterns-established:
  - "SHA-256 path hashing: abs_path.as_os_str().as_encoded_bytes() for cross-platform byte-stable hashing"
  - "Rope chunk hashing: iterate rope.chunks() feeding chunk.as_bytes() into hasher — avoids materializing full string"

requirements-completed: [PERS-03, PERS-05]

# Metrics
duration: 10min
completed: 2026-03-01
---

# Phase 03 Plan 01: Write Undo History Summary

**SHA-256-hashed binary blob write path for undo/redo history on buffer save, gated on PersistentUndoSettings.enabled with dirty-buffer guard and max_entries truncation**

## Performance

- **Duration:** ~10 min
- **Started:** 2026-03-01T18:15:00Z
- **Completed:** 2026-03-01T18:25:00Z
- **Tasks:** 1
- **Files modified:** 3 (Cargo.toml, items.rs, Cargo.lock)

## Accomplishments
- Added sha2 and hex workspace dependencies to editor crate
- Implemented blob_path_for (SHA-256 hash of abs_path -> .bin path) and compute_content_hash (SHA-256 over rope chunks)
- Implemented write_undo_history on Editor: enabled guard, dirty-buffer guard, max_entries truncation, background blob write + DB upsert
- Wired write_undo_history into Editor::serialize before the spawn closure, with task.detach() for fire-and-forget semantics

## Task Commits

Each task was committed atomically:

1. **Task 1: Add sha2/hex deps and implement write_undo_history with blob_path_for utility** - `e32565b60e` (feat)

**Plan metadata:** (docs commit — pending)

## Files Created/Modified
- `crates/editor/Cargo.toml` - Added sha2.workspace and hex.workspace dependencies
- `crates/editor/src/items.rs` - Added blob_path_for, compute_content_hash, write_undo_history, wired into serialize
- `Cargo.lock` - Updated with sha2/hex dependency resolution

## Decisions Made
- `write_undo_history` placed in `items.rs` alongside `serialize` — these two are tightly coupled and serialize is where the call lives
- `use settings::Settings as _` scoped inside `write_undo_history` body to avoid polluting module-level imports with a trait-only import
- `blob_path_for` and `compute_content_hash` are module-level fns (not methods) — pure utilities, no entity/context access needed

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Self-Check: PASSED

All files exist and commit e32565b60e is present.

## Next Phase Readiness
- Write path complete: undo/redo history blobs are written to `database_dir/undo_history/{sha256_of_path}.bin` on every clean buffer save when `persistent_undo.enabled = true`
- Metadata row is upserted in SQLite with content hash and mtime for validation at restore time
- Ready for Phase 03 Plan 02: restore path (read blob on open, decode, call restore_history)

---
*Phase: 03-core-write-and-restore*
*Completed: 2026-03-01*
