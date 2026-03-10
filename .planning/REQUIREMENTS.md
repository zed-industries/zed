# Requirements: Rename undo_history to persist_history

**Defined:** 2026-03-10
**Core Value:** Clean separation of persistence layer naming from core undo/redo system

## v1 Requirements

### Persistence DB

- [x] **DB-01**: Rename SQL table `undo_history` to `persist_history`
- [x] **DB-02**: Rename `get_undo_history_meta()` to `get_persist_history_meta()`
- [x] **DB-03**: Rename `get_undo_history_paths()` to `get_persist_history_paths()`
- [x] **DB-04**: Rename `save_undo_history_meta()` to `save_persist_history_meta()`
- [x] **DB-05**: Rename `delete_undo_history()` to `delete_persist_history()`
- [x] **DB-06**: Rename test function `test_save_and_get_undo_history_meta()`

### Editor Integration

- [x] **EDIT-01**: Rename `prune_undo_history()` to `prune_persist_history()`
- [x] **EDIT-02**: Rename `write_undo_history()` to `write_persist_history()`
- [x] **EDIT-03**: Rename `restore_undo_history()` to `restore_persist_history()`
- [x] **EDIT-04**: Update blob directory reference from `"undo_history"` to `"persist_history"`
- [x] **EDIT-05**: Update log/warning messages referencing undo_history

### Validation

- [x] **VAL-01**: `cargo check` passes after all renames
- [x] **VAL-02**: No pre-existing core undo/redo code was modified

## v2 Requirements

(None — this is a one-time rename)

## Out of Scope

| Feature | Reason |
|---------|--------|
| Core undo/redo methods in buffer.rs | Pre-existing, unrelated to persistence |
| Proto serialization of undo maps | Pre-existing serialization layer |
| Vim/action_log/agent_ui undo code | Separate systems entirely |
| Renaming `restore_history()` in buffer.rs | Generic method name, not persistence-specific |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| DB-01 | Phase 1 | Complete |
| DB-02 | Phase 1 | Complete |
| DB-03 | Phase 1 | Complete |
| DB-04 | Phase 1 | Complete |
| DB-05 | Phase 1 | Complete |
| DB-06 | Phase 1 | Complete |
| EDIT-01 | Phase 1 | Complete |
| EDIT-02 | Phase 1 | Complete |
| EDIT-03 | Phase 1 | Complete |
| EDIT-04 | Phase 1 | Complete |
| EDIT-05 | Phase 1 | Complete |
| VAL-01 | Phase 2 | Complete |
| VAL-02 | Phase 2 | Complete |

**Coverage:**
- v1 requirements: 13 total
- Mapped to phases: 13
- Unmapped: 0

---
*Requirements defined: 2026-03-10*
*Last updated: 2026-03-10 after roadmap creation*
