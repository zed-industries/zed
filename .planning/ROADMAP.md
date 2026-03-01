# Roadmap: Persistent Undo History

## Overview

This roadmap delivers persistent undo/redo history for Zed in four phases following a strict dependency chain. Phase 1 exposes the text layer API that everything else requires. Phase 2 defines the storage schema and settings before any writes occur. Phase 3 delivers the core user-facing feature: history survives tab close and full restarts. Phase 4 adds pruning and maintenance so storage remains bounded.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Text Layer API** - Expose undo/redo stack accessors and establish serialization format
- [ ] **Phase 2: Persistence Schema and Settings** - Define the SQLite schema and configuration settings
- [ ] **Phase 3: Core Write and Restore** - Save history on buffer events and restore on file open
- [ ] **Phase 4: Pruning and Maintenance** - Auto-prune orphaned history records on startup

## Phase Details

### Phase 1: Text Layer API
**Goal**: Users can serialize and deserialize a complete undo/redo stack via a stable, versioned format
**Depends on**: Nothing (first phase)
**Requirements**: INFRA-01, INFRA-03, INFRA-04, INFRA-05
**Success Criteria** (what must be TRUE):
  1. `text::Buffer` exposes public accessors (`undo_stack()`, `redo_stack()`, `restore_history()`) usable from `crates/editor/`
  2. A `HistoryBlob::V1` versioned envelope encodes and decodes an undo/redo stack via `postcard` without data loss
  3. Round-trip serialization test passes: serialize a stack, deserialize it, and confirm all transactions are identical
  4. Only transaction-level stacks are serialized — the full CRDT operation log is not included in the blob
  5. All encoding work executes on a background thread, not the UI thread
**Plans:** 2 plans
- [x] 01-01-PLAN.md — Add serde/postcard deps and Buffer API accessors (undo_stack, redo_stack, restore_history)
- [x] 01-02-PLAN.md — Create history_serde module with mirror structs, HistoryBlob::V1, encode/decode, and round-trip tests

### Phase 2: Persistence Schema and Settings
**Goal**: A `UndoHistoryDb` SQLite domain exists with the correct schema, and user-facing settings are registered in Zed's settings system
**Depends on**: Phase 1
**Requirements**: INFRA-02, CONF-01, CONF-02
**Success Criteria** (what must be TRUE):
  1. The `undo_history` table exists in the editor database with columns for `workspace_id`, `abs_path`, `content_hash`, `mtime_seconds`, `mtime_nanos`, and `last_accessed_at`
  2. History records are keyed on `(workspace_id, abs_path)`, not session-scoped item IDs
  3. User can set `persistent_undo.enabled` in Zed settings and schema validation accepts it (default: false)
  4. User can set `persistent_undo.max_entries` in Zed settings and schema validation accepts it (default: 10,000)
**Plans:** 1/2 plans executed
- [ ] 02-01-PLAN.md — Add undo_history SQLite migration and query methods to EditorDb
- [ ] 02-02-PLAN.md — Add PersistentUndoSettings to settings system (content struct, resolved struct, default.json, registration)

### Phase 3: Core Write and Restore
**Goal**: Users can close a tab or quit Zed and reopen the same file with their full undo/redo history intact
**Depends on**: Phase 2
**Requirements**: PERS-01, PERS-02, PERS-03, PERS-04, PERS-05, PERS-06
**Success Criteria** (what must be TRUE):
  1. User closes a tab, reopens the same file, and cmd-z / cmd-shift-z restores history from before the close
  2. User quits Zed entirely, relaunches, reopens a file, and undo/redo history is fully intact
  3. If the file is modified externally between sessions, reopening it does not restore stale history — history is silently discarded
  4. When the feature is disabled (`persistent_undo.enabled: false`), no disk writes occur and existing undo behavior is unchanged
  5. History is only restored when the file's content hash matches the stored SHA-256 hash
**Plans**: TBD

### Phase 4: Pruning and Maintenance
**Goal**: Persistent undo storage remains bounded and does not accumulate records for files that no longer exist
**Depends on**: Phase 3
**Requirements**: MAINT-01
**Success Criteria** (what must be TRUE):
  1. On Zed startup, history records for files that no longer exist on disk are automatically deleted
  2. After pruning runs, no orphaned records remain for non-existent files
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Text Layer API | 2/2 | Complete | 2026-03-01 |
| 2. Persistence Schema and Settings | 1/2 | In Progress|  |
| 3. Core Write and Restore | 0/TBD | Not started | - |
| 4. Pruning and Maintenance | 0/TBD | Not started | - |
