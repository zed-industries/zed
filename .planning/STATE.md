# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-01)

**Core value:** Closing and reopening a file must preserve the complete undo/redo history
**Current focus:** Phase 1 - Text Layer API

## Current Position

Phase: 1 of 4 (Text Layer API)
Plan: 2 of 2 in current phase — PHASE COMPLETE
Status: Phase 1 complete; ready for Phase 2
Last activity: 2026-03-01 — Plan 01-02 complete: history_serde module with encode/decode and round-trip tests

Progress: [██░░░░░░░░] 25%

## Performance Metrics

**Velocity:**
- Total plans completed: 2
- Average duration: 12min
- Total execution time: 25min

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-text-layer-api | 2 | 25min | 12min |

**Recent Trend:**
- Last 5 plans: 20min, 5min
- Trend: faster

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [Init]: SQLite for index/lookup, binary files for history data — pending confirmation
- [Init]: Feature disabled by default (opt-in) — pending confirmation
- [Init]: Invalidate on external edit — pending confirmation
- [Init]: Entry limit (not size limit) — pending confirmation
- [Research]: Use `postcard` (not `bincode` — RUSTSEC-2025-0141) for binary serialization
- [Research]: Key history on `(workspace_id, abs_path)`, not session-scoped `item_id`
- [Research]: Promote `postcard` from transitive to declared workspace dependency before Phase 1
- [Plan 01-01]: suppress_grouping: true on all restored HistoryEntry values — prevents merging restored transactions with new edits
- [Plan 01-01]: Instant::now() for all restored timestamps — original values are session-local, meaningless after restart
- [Plan 01-01]: UndoMap reconstructed by replaying undo_operations via existing undo_map.insert() mechanism
- [Plan 01-02]: HistoryBlob and SerializedUndoHistory use pub(crate) — external callers use encode_history/decode_history
- [Plan 01-02]: UndoOperation.counts serialized as sorted Vec<(SerializedLamport, u32)> for deterministic output
- [Plan 01-02]: clock::Global serialized as Vec<u32> (index = replica_id, value = seq) matching internal SmallVec layout
- [Plan 01-02]: FullOffset serialized as u64 for cross-platform stability (usize is platform-dependent)
- [Plan 01-02]: restore_history must be called on a buffer with matching CRDT fragment state for undo/redo to work

### Pending Todos

None yet.

### Blockers/Concerns

- [Phase 1 - RESOLVED]: `UndoMap` reconstruction confirmed valid by replaying undo_operations via undo_map.insert() in restore_history()
- [Phase 3]: Confirm whether `SerializableItem::serialize()` fires on abrupt app termination (Cmd-Q vs crash) before writing restore path

## Session Continuity

Last session: 2026-03-01
Stopped at: Completed Plan 01-02 (history_serde module); Phase 1 complete — ready for Phase 2
Resume file: None
