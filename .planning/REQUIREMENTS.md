# Requirements: Persistent Undo History

**Defined:** 2026-03-01
**Core Value:** Closing and reopening a file must preserve the complete undo/redo history

## v1 Requirements

### Persistence

- [x] **PERS-01**: User can close a tab and reopen the same file with full undo/redo history intact
- [x] **PERS-02**: User can quit Zed entirely, relaunch, and reopen a file with undo/redo history intact
- [x] **PERS-03**: Undo history is written to disk when a buffer is saved or a tab is closed
- [x] **PERS-04**: Undo history is restored from disk when a file is reopened (only if file content unchanged)
- [x] **PERS-05**: File content hash (SHA-256) is stored alongside history and validated before restore
- [x] **PERS-06**: If hash validation fails (file modified externally), persisted history is silently discarded

### Configuration

- [x] **CONF-01**: User can enable/disable persistent undo via `persistent_undo.enabled` setting (default: false)
- [x] **CONF-02**: User can set maximum undo entries via `persistent_undo.max_entries` setting (default: 10,000)

### Maintenance

- [x] **MAINT-01**: History records for files that no longer exist on disk are automatically pruned on startup

### Infrastructure

- [x] **INFRA-01**: Serialization uses postcard with a versioned envelope format (HistoryBlob::V1) for forward compatibility
- [x] **INFRA-02**: History is keyed on (workspace_id, abs_path), not session-scoped item_id
- [x] **INFRA-03**: Only transaction-level undo/redo stacks are serialized (not the full CRDT operation log)
- [x] **INFRA-04**: All serialization and disk I/O occurs off the UI thread via background tasks
- [x] **INFRA-05**: text::Buffer exposes public accessors for undo/redo stack state needed by serialization

## v2 Requirements

### Configuration

- **CONF-03**: User can exclude files by glob pattern via `persistent_undo.exclude` setting

### Maintenance

- **MAINT-02**: History is invalidated immediately when file-watcher detects external modification (runtime, not just on reopen)
- **MAINT-03**: Non-blocking notification shown when history is cleared due to external modification
- **MAINT-04**: Time-based pruning clears entries older than N days

### Remote

- **REMOTE-01**: Undo history survives remote session reconnects

## Out of Scope

| Feature | Reason |
|---------|--------|
| Undo tree visualization | Significant UI investment; Zed uses linear undo model; defer to v2+ |
| Cross-device history sync | Cloud storage, auth, conflict resolution — a separate product |
| Collaborative undo (multi-user) | Fundamentally different problem; handled by collab layer |
| UI indicator for history availability | Visual noise; graceful degradation is sufficient |
| Size-based (MB) limits | Entry count is more predictable; avoids overflow bugs |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| PERS-01 | Phase 3 | Complete |
| PERS-02 | Phase 3 | Complete |
| PERS-03 | Phase 3 | Complete |
| PERS-04 | Phase 3 | Complete |
| PERS-05 | Phase 3 | Complete |
| PERS-06 | Phase 3 | Complete |
| CONF-01 | Phase 2 | Complete |
| CONF-02 | Phase 2 | Complete |
| MAINT-01 | Phase 4 | Complete |
| INFRA-01 | Phase 1 | Complete |
| INFRA-02 | Phase 2 | Complete |
| INFRA-03 | Phase 1 | Complete |
| INFRA-04 | Phase 1 | Complete |
| INFRA-05 | Phase 1 | Complete |

**Coverage:**
- v1 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0

---
*Requirements defined: 2026-03-01*
*Last updated: 2026-03-01 after Plan 01-01 completion (INFRA-05 complete)*
