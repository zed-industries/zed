# Roadmap: Rename undo_history to persist_history

## Overview

Rename all persistence-related `undo_history` identifiers to `persist_history` across the DB layer and editor integration, then validate the codebase compiles with no unintended changes to core undo/redo code.

## Phases

**Phase Numbering:**
- Integer phases (1, 2): Planned milestone work
- Decimal phases (1.1, 1.2): Urgent insertions (marked with INSERTED)

- [ ] **Phase 1: Rename Persistence Identifiers** - Rename all undo_history references to persist_history in DB and editor code
- [ ] **Phase 2: Validation** - Verify compilation and confirm core undo/redo code is untouched

## Phase Details

### Phase 1: Rename Persistence Identifiers
**Goal**: All persistence-related code uses `persist_history` naming instead of `undo_history`
**Depends on**: Nothing (first phase)
**Requirements**: DB-01, DB-02, DB-03, DB-04, DB-05, DB-06, EDIT-01, EDIT-02, EDIT-03, EDIT-04, EDIT-05
**Success Criteria** (what must be TRUE):
  1. The SQL table for persisted history is named `persist_history`
  2. All DB accessor methods in persistence.rs use `persist_history` naming
  3. All editor methods for writing, restoring, and pruning persisted history use `persist_history` naming
  4. The blob directory for persisted history is named `persist_history`
  5. Log and warning messages reference `persist_history` instead of `undo_history`
**Plans:** 1 plan

Plans:
- [ ] 01-01-PLAN.md — Rename all undo_history identifiers to persist_history in persistence.rs and items.rs

### Phase 2: Validation
**Goal**: Codebase compiles cleanly and no core undo/redo code was modified
**Depends on**: Phase 1
**Requirements**: VAL-01, VAL-02
**Success Criteria** (what must be TRUE):
  1. `cargo check` passes with zero errors
  2. No changes exist in core undo/redo methods (buffer.rs undo/redo, editor.rs undo/redo actions, vim undo code)
**Plans:** 1 plan

Plans:
- [ ] 02-01-PLAN.md — Validate compilation and confirm only persistence files were modified

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Rename Persistence Identifiers | 0/1 | Not started | - |
| 2. Validation | 0/1 | Not started | - |
