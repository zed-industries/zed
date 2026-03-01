# Persistent Undo History

## What This Is

A feature for Zed editor that persists undo/redo history to disk, so closing a tab or quitting Zed doesn't lose edit history. When a file is reopened, cmd-z and cmd-shift-z restore the full undo/redo stack from before the close. Configurable per-user with entry limits. Shipped as opt-in via `persistent_undo.enabled` setting.

## Core Value

Closing and reopening a file must preserve the complete undo/redo history — the user should never lose work context because they closed a tab or restarted the editor.

## Requirements

### Validated

- ✓ In-memory undo/redo via Buffer's undo_map and transaction history — existing
- ✓ Buffer serialization infrastructure (text::Buffer) — existing
- ✓ SQLite-backed persistence for workspace state (sqlez crate) — existing
- ✓ Settings system with schema validation and per-project overrides — existing
- ✓ File watching for external change detection — existing
- ✓ Persist undo/redo history to disk when buffer is closed or saved — v1.0
- ✓ Restore undo/redo history from disk when file is reopened — v1.0
- ✓ Survive full Zed restarts (not just tab close/reopen) — v1.0
- ✓ Invalidate persisted history when file is modified externally — v1.0
- ✓ Configurable toggle: `persistent_undo.enabled` (default: false) — v1.0
- ✓ Configurable entry limit: `persistent_undo.max_entries` (default: 10,000) — v1.0
- ✓ Auto-pruning of history for files that no longer exist — v1.0

### Active

- [ ] Configurable file exclusions: `persistent_undo.exclude` (glob patterns)
- [ ] Runtime invalidation on file-watcher external modification detection
- [ ] Non-blocking notification when history cleared due to external modification
- [ ] Time-based pruning (entries older than N days)
- [ ] Undo history survives remote session reconnects

### Out of Scope

- Collaborative undo history (multi-user) — separate concern, handled by collab layer
- Cross-device history sync — too complex, local-only
- Undo tree visualization — nice-to-have future feature, not core
- UI indicator showing persistent history availability — graceful degradation is sufficient
- Size-based (MB) limits — entry count is more predictable

## Context

Shipped v1.0 with 2,948 lines of Rust across 27 files. Tech stack: postcard for serialization, sqlez/SQLite for metadata, SHA-256 for content hashing. Feature is opt-in via `persistent_undo.enabled: false` default.

Key crates modified: `crates/text/` (serialization, buffer accessors), `crates/editor/` (write/restore lifecycle, settings, pruning), `crates/language/` (restore_history delegation), `crates/settings_content/` (settings UI schema).

Architecture: SQLite stores metadata (workspace_id, abs_path, content_hash, mtime). Binary blobs stored as files in `database_dir/undo_history/{sha256_of_path}.bin`. All I/O on background threads.

## Constraints

- **Codebase**: Must follow Zed's existing patterns — GPUI entity model, async-aware, no panics
- **Performance**: Serialization/deserialization must not block the UI thread
- **Storage**: History stored in Zed's local data directory alongside existing SQLite databases
- **Compatibility**: Must not break existing undo/redo behavior when feature is disabled

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| SQLite for index/lookup, binary files for history data | SQLite handles metadata and path-to-history mapping; actual history blobs stored as files to avoid bloating the DB | ✓ Good |
| Feature disabled by default | Avoid surprising users with new disk usage; opt-in ensures intentional adoption | ✓ Good |
| Invalidate on external edit | If the file changes outside Zed, the undo history is no longer meaningful — clearing it prevents corruption | ✓ Good |
| Entry limit (not size limit) | More predictable for users — "10,000 undos" is easier to reason about than "10MB" | ✓ Good |
| postcard for serialization (not bincode) | RUSTSEC-2025-0141 advisory on bincode; postcard is compact, no-std, well-maintained | ✓ Good |
| Key on (workspace_id, abs_path) | Session-scoped item_id changes every restart; path-based keying is stable | ✓ Good |
| Mirror struct pattern for serde | Avoids adding Serialize derives to core text types; clean separation | ✓ Good |
| abs_path as BLOB in SQLite | sqlez Path bind uses as_encoded_bytes() (BLOB); TEXT column causes runtime failure | ✓ Good |
| restore_history on original buffer | Fresh buffers lack CRDT fragment state; must restore to buffer with matching state | ✓ Good |
| prune_undo_history as free function | cleanup is static on SerializableItem (no &self); matches blob_path_for pattern | ✓ Good |

---
*Last updated: 2026-03-01 after v1.0 milestone*
