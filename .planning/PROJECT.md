# Persistent Undo History

## What This Is

A feature for Zed editor that persists undo/redo history to disk, so closing a tab or quitting Zed doesn't lose edit history. When a file is reopened, cmd-z and cmd-shift-z restore the full undo/redo stack from before the close. Configurable per-user with entry limits and file pattern exclusions.

## Core Value

Closing and reopening a file must preserve the complete undo/redo history — the user should never lose work context because they closed a tab or restarted the editor.

## Requirements

### Validated

<!-- Existing capabilities in Zed that this feature builds on. -->

- ✓ In-memory undo/redo via Buffer's undo_map and transaction history — existing
- ✓ Buffer serialization infrastructure (text::Buffer) — existing
- ✓ SQLite-backed persistence for workspace state (sqlez crate) — existing
- ✓ Settings system with schema validation and per-project overrides — existing
- ✓ File watching for external change detection — existing

### Active

- [ ] Persist undo/redo history to disk when a buffer is closed or saved
- [ ] Restore undo/redo history from disk when a file is reopened
- [ ] Survive full Zed restarts (not just tab close/reopen within a session)
- [ ] Invalidate persisted history when a file is modified externally (buffer no longer matches last saved state)
- [ ] Configurable toggle: `persistent_undo.enabled` (default: false)
- [ ] Configurable entry limit: `persistent_undo.max_entries` (default: 10,000)
- [ ] Configurable file exclusions: `persistent_undo.exclude` (glob patterns, e.g., `["*.lock", "*.csv"]`)
- [ ] Auto-pruning of history for files that no longer exist or have been moved

### Out of Scope

- Collaborative undo history (multi-user) — separate concern, handled by collab layer
- UI indicator showing persistent history availability — not needed for v1
- Cross-device history sync — too complex, local-only
- Undo tree visualization — nice-to-have future feature, not core

## Context

Zed's current undo system lives entirely in `text::Buffer` via `UndoMap` and transaction history. When a buffer is dropped (tab closed), the history is lost. The workspace already uses SQLite (via the `sqlez` crate) to persist workspace layout, open files, and other state across restarts. The buffer system serializes/deserializes for operations like collaboration, so serialization primitives exist but aren't currently wired to persistence.

The `crates/text/` crate contains the core buffer and undo infrastructure. The `crates/editor/` crate manages buffer lifecycle. The `crates/workspace/` crate handles item persistence. The `crates/settings/` crate provides the configuration system.

File watching is already implemented — when an external change is detected, this feature should clear the persisted history for that file.

## Constraints

- **Codebase**: Must follow Zed's existing patterns — GPUI entity model, async-aware, no panics
- **Performance**: Serialization/deserialization must not block the UI thread
- **Storage**: History stored in Zed's local data directory alongside existing SQLite databases
- **Compatibility**: Must not break existing undo/redo behavior when feature is disabled

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| SQLite for index/lookup, binary files for history data | SQLite handles metadata and path-to-history mapping; actual history blobs stored as files to avoid bloating the DB | — Pending |
| Feature disabled by default | Avoid surprising users with new disk usage; opt-in ensures intentional adoption | — Pending |
| Invalidate on external edit | If the file changes outside Zed, the undo history is no longer meaningful — clearing it prevents corruption | — Pending |
| Entry limit (not size limit) | More predictable for users — "10,000 undos" is easier to reason about than "10MB" | — Pending |

---
*Last updated: 2026-03-01 after initialization*
