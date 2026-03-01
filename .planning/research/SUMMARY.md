# Project Research Summary

**Project:** Persistent Undo/Redo History for Zed Editor
**Domain:** Editor state persistence in a CRDT-based Rust GUI application (GPUI)
**Researched:** 2026-03-01
**Confidence:** HIGH

## Executive Summary

Persistent undo/redo is a well-understood editor feature with established patterns from Vim (`undofile`), VS Code (`files.restoreUndoStack`), and Emacs (`undo-fu-session`). The core design is consistent across all implementations: write a serialized form of the undo/redo stacks to disk on file close or save, validate a content checksum on restore, and discard history when the file has changed externally. Zed's unique complexity is that its undo history lives inside a CRDT-based buffer model (`text::History`, `UndoMap`) whose types are private and lack serde derives — making serialization require either new public accessors or mirror structs. All other persistence infrastructure already exists in the codebase and the approach is straightforward to implement by extending existing patterns.

The recommended approach is to extend `crates/editor/` with a new `undo_history.rs` serialization module and a new `UndoHistoryDb` domain in the existing `persistence.rs`, keyed on `(workspace_id, abs_path)` following the established `file_folds` precedent. The binary blob format uses `postcard` (already in Zed's lockfile; `bincode` is marked unmaintained per RUSTSEC-2025-0141) wrapped in a versioned enum from day one. SHA-256 content hashing (`sha2`, already a workspace dependency) provides definitive invalidation — mtime alone is insufficient because `git stash pop` can restore a previous mtime on changed content.

The primary risks are architectural, not algorithmic. Four patterns must be established correctly in the first phase or they become expensive to change: keying history on file path (not `item_id`), versioning the binary format, moving all serialization work off the foreground thread, and serializing only the user-facing undo/redo transaction stacks (not the full CRDT operation log). Getting these wrong requires schema migrations or user-visible history loss. Getting them right makes the remaining phases straightforward.

## Key Findings

### Recommended Stack

All dependencies are already present in Zed's workspace — no new external crates are strictly required, though `postcard` needs to be promoted from a transitive dependency to a declared workspace dependency. The `sqlez` internal SQLite abstraction handles the persistence layer; adding undo history follows the identical `Domain` + `MIGRATIONS` + `query!` macro pattern used by `EditorDb` and `WorkspaceDb`. Full details in `.planning/research/STACK.md`.

**Core technologies:**
- `postcard` 1.1.3: Binary serialization for history blobs — actively maintained, already in lockfile, wire format stable since v1.0.0
- `sqlez` (Zed internal): SQLite abstraction for history index — use the same `Domain` + migration pattern as `EditorDb`
- `sha2` 0.10.9: SHA-256 content hashing for external-edit invalidation — already a workspace dependency
- `serde` 1.0: Derive macros on mirror structs or newly public types — already a workspace dependency with `derive` feature

**What to avoid:**
- `bincode`: Marked unmaintained (RUSTSEC-2025-0141). Existing Zed usages are technical debt; do not add new ones.
- `item_id` as lookup key: Session-scoped; loses all history on restart.
- JSON for history blobs: 5-10x larger than binary for numeric-heavy data like Lamport timestamps.

### Expected Features

Research across Vim, VS Code, Emacs, and Zed GitHub issues (#4942, #23164, #16485) shows strong user consensus on which features are essential. VS Code's experience of adding `files.restoreUndoStack: false` immediately after shipping v1.44 confirms that a user-controlled toggle is non-negotiable. Full details in `.planning/research/FEATURES.md`.

**Must have (table stakes):**
- Survive tab close and reopen — the core use case confirmed across all Zed issues
- Survive full editor restart — distinguishes from VS Code's in-memory-only approach
- Hash/checksum invalidation on external file modification — mandatory for correctness; not optional
- `persistent_undo.enabled` toggle (default: false, opt-in) — required based on VS Code experience
- `persistent_undo.max_entries` (default: 10,000) — users need an upper bound
- `persistent_undo.exclude` glob patterns — exclude generated files, large CSVs, etc.
- Auto-prune history for files that no longer exist — prevents unbounded disk growth

**Should have (competitive, add in v1.x):**
- File-watcher integration to invalidate history on detected external change (event-driven, not just check-on-restore)
- Non-blocking notification when history is cleared due to external modification
- Time-based pruning (clear entries older than N days)

**Defer (v2+):**
- Undo history survival across remote session reconnects — Zed-specific; requires collab layer coordination
- Undo tree visualization — significant UI investment; not justified until linear persistence is proven valuable

**Anti-features to avoid:**
- Cross-device sync: Not a local persistence feature; Git covers cross-device continuity
- Collaborative undo: Fundamentally different problem domain
- Size-based (MB) limits: Entry count is more predictable and simpler; avoids Notepad++ v7.7-style overflow bugs

### Architecture Approach

All new code belongs in `crates/editor/` — no new crate is warranted. The feature integrates with three existing seams: `SerializableItem::serialize()`/`deserialize()` in `items.rs` (already called on tab close/workspace restore), `cx.subscribe(&buffer, ...)` event handling for save and reload events, and the `sqlez` domain pattern in `persistence.rs`. The only cross-crate change required is adding public accessors to `text::Buffer` (`undo_stack()`, `redo_stack()`, `restore_history()`) since `History` is currently private. Full details with data flow diagrams in `.planning/research/ARCHITECTURE.md`.

**Major components:**
1. `text::Buffer` — source of truth for undo/redo state; needs new public accessors added
2. `undo_history.rs` (new) — serialization/deserialization of `HistoryEntry` + `Operation` types using mirror structs with serde derives
3. `UndoHistoryDb` domain in `persistence.rs` (extend existing) — SQLite table keyed on `(workspace_id, path)` with mtime and content hash columns
4. `PersistentUndoSettings` in `editor_settings.rs` (extend existing) — `enabled`, `max_entries`, `exclude` with `JsonSchema` derive
5. `Editor` in `editor.rs` + `items.rs` (extend existing) — hooks for save/restore triggered by buffer events and item lifecycle

**Build order derived from dependencies:**
Phase 1 (text layer API) → Phase 2 (serialization module) → Phase 3 (database schema) → Phase 4 (settings) → Phase 5 (editor integration)

### Critical Pitfalls

Seven pitfalls were identified; four are critical and must be addressed in Phase 1 schema/format design before any production code is written. Full analysis with warning signs and recovery costs in `.planning/research/PITFALLS.md`.

1. **Serializing the CRDT operation log instead of transaction history** — serialize only `History.undo_stack` and `History.redo_stack` plus the operations referenced by those transactions; never dump the full `operations: TreeMap<Lamport, Operation>` which includes collaborative peer edits and CRDT bookkeeping
2. **Using `item_id` as the history lookup key** — `item_id` is session-scoped; key on `(workspace_id, abs_path)` following the `file_folds` precedent; failure here requires a schema migration to fix
3. **No format versioning in the serialized blob** — wrap all blobs in a versioned enum (`HistoryBlob::V1(...)`) from day one; binary formats are not self-describing and format breaks become silent user-visible data loss
4. **Blocking the UI thread during serialization** — clone the undo/redo stacks on the foreground thread, then move all encoding and disk writes into `cx.background_spawn()`; encoding 10,000 entries synchronously causes a visible tab-close freeze
5. **History restored before buffer content is fully loaded** — sequence history restore after `buffer.is_loaded()` completes; the existing fold restore in `Editor::read_metadata_from_db` demonstrates this pattern
6. **Stale history applied after external file modification** — compare stored mtime and `saved_version` before restoring; subscribe to `BufferEvent::Reloaded` to delete persisted history when a file is reloaded from disk
7. **Unbounded history file growth** — include `last_accessed_at` column in the initial schema (cannot be retrofitted without migration); implement startup pruning sweep for deleted files

## Implications for Roadmap

Research confirms a natural 5-phase dependency chain. Phases 1-3 form the foundation that cannot be reordered. Phases 4-5 are independently sequenceable once Phase 3 exists.

### Phase 1: Foundation — Text Layer API and Data Model

**Rationale:** `text::History` is private and has no serde derives. Nothing else can be built until the boundary is established: which fields are serialized, what public accessors exist, how history is restored without corrupting `UndoMap`. This is also where the four "never wrong" decisions must be locked in: path-keyed lookup, versioned format, transaction-only scope (not CRDT log), and background serialization pattern.
**Delivers:** Public accessors on `text::Buffer` (`undo_stack()`, `redo_stack()`, `restore_history()`), mirror structs with serde derives in `undo_history.rs`, round-trip serialization tests, versioned `HistoryBlob` enum.
**Addresses:** Write history on close (prerequisite); Restore on open (prerequisite)
**Avoids:** CRDT log confusion (Pitfall 1), format versioning omission (Pitfall 5), item_id key mistake (Pitfall 4)

### Phase 2: Persistence Schema and Settings

**Rationale:** The database schema must be defined before any writes occur — adding columns later requires migrations, and `last_accessed_at` cannot be retrofitted without losing historical access data. Settings must exist before editor integration so all hooks can read them.
**Delivers:** `undo_history` SQLite table in `UndoHistoryDb` domain (keyed on `workspace_id, path` with `content_hash`, `mtime_seconds`, `mtime_nanos`, `last_accessed_at`); `PersistentUndoSettings` struct (`enabled`, `max_entries`, `exclude`) registered in settings system.
**Uses:** `sqlez` domain pattern, `serde` + `schemars` derives
**Avoids:** item_id key mistake (Pitfall 4), unbounded growth without `last_accessed_at` (Pitfall 6)

### Phase 3: Core Write and Restore Paths

**Rationale:** This is the central feature delivery — save history on buffer save/close, restore on open with mtime+hash validation. Requires Phase 1 (serialization) and Phase 2 (schema + settings) to be complete. The two flows (write and restore) must be built together because correctness tests require both.
**Delivers:** `Editor::on_buffer_saved()` hook that serializes and writes history to DB; `Editor::read_metadata_from_db()` extension that reads, validates, and restores history; mtime + `saved_version` validation check; background spawn for both encode and write operations; full close-restart-reopen integration test.
**Addresses:** Survive tab close/reopen (P1), Survive full restart (P1), Hash invalidation (P1), Feature disabled by default with zero overhead (P1)
**Avoids:** UI thread blocking (Pitfall 3), history restored before buffer loaded (Pitfall 7), stale history on mismatch (Pitfall 2)

### Phase 4: Pruning and Exclusion Logic

**Rationale:** Pruning is independent of the core write/restore path once the schema exists. `last_accessed_at` is already stored from Phase 2. This phase wires up the actual cleanup logic: exclude globs checked at write time, orphan cleanup on startup, max_entries truncation.
**Delivers:** Glob pattern exclusion evaluated before writes; startup sweep deleting history rows for non-existent files; `max_entries` enforcement truncating oldest transactions before serialization; pruning integration tests (simulate file deletion and verify cleanup).
**Addresses:** Auto-prune for deleted files (P1), `persistent_undo.exclude` globs (P1), `persistent_undo.max_entries` enforcement (P1)
**Avoids:** Unbounded growth (Pitfall 6)

### Phase 5: External Edit Invalidation and v1.x Polish

**Rationale:** File-watcher integration for runtime invalidation is a natural follow-on once core persistence is working. Zed already has file watching; wiring `BufferEvent::Reloaded` to delete persisted history is low complexity. The non-blocking notification is the user-facing payoff for graceful degradation.
**Delivers:** `Editor` subscription to `BufferEvent::Reloaded` that deletes persisted history for the file; non-blocking notification "Undo history cleared — file was modified outside Zed"; time-based pruning (entries older than N days); full external-edit test scenario.
**Addresses:** File-watcher integration (P2), non-blocking notification (P2), time-based pruning (P2)
**Avoids:** Stale history on external edit slipping through (Pitfall 2)

### Phase Ordering Rationale

- Phase 1 before all others: the text layer API is the dependency root; nothing serializes without it
- Phase 2 before Phase 3: schema columns (`last_accessed_at`, `content_hash`) cannot be added retroactively without migrations; settings must exist before the first write
- Phase 3 before Phase 4: pruning and exclusion augment the write path, which must exist first
- Phase 5 last: event-driven invalidation is a safety enhancement layered on top of the check-on-restore approach delivered in Phase 3
- Phases 3 and 4 could potentially be merged if the team prefers fewer phases; they are split here because Phase 4 (pruning logic) can be partially deferred without breaking correctness

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 1 (Text layer API):** The exact scope of `History` field exposure is not trivial — `UndoMap` reconstruction from restored operations requires understanding CRDT invariants. Recommend a targeted reading of `crates/text/src/text.rs` and `undo_map.rs` before implementation, particularly the relationship between `History.operations`, `UndoMap`, and `BufferSnapshot`.
- **Phase 3 (Restore sequencing):** The async ordering between `Editor::deserialize()` and buffer load is subtle. Study how fold restore handles `fingerprint`-based position recovery and the `read_metadata_from_db` async chain before writing the restore path.

Phases with standard patterns (skip research-phase):
- **Phase 2 (Schema + Settings):** The `sqlez` domain pattern and settings registration are thoroughly documented in the codebase. Copy `file_folds` for the schema, copy any existing settings struct for the settings registration.
- **Phase 4 (Pruning):** Glob matching against `exclude` patterns follows the existing `PathMatcher` pattern in Zed. Orphan cleanup is a straightforward startup query.
- **Phase 5 (Event wiring):** `cx.subscribe(&buffer, ...)` with a `BufferEvent::Reloaded` arm is a single-digit-line change following the pattern already in `editor.rs` line 2537.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All technologies verified against Zed lockfile and workspace Cargo.toml; `bincode` advisory confirmed from RustSec; `postcard` wire format stability confirmed from official docs |
| Features | HIGH | Core behaviors verified against VS Code release notes, Vim docs, Emacs package source, and Zed GitHub issues; feature scope confirmed against PROJECT.md |
| Architecture | HIGH | All findings sourced directly from Zed codebase (`crates/text`, `crates/language`, `crates/editor`, `crates/db`); component boundaries verified by reading actual code |
| Pitfalls | HIGH | Four critical pitfalls verified against codebase internals; three supplemented with cross-editor evidence (Vim, VS Code, Kakoune issue trackers) |

**Overall confidence:** HIGH

### Gaps to Address

- **`UndoMap` reconstruction invariants:** Research confirmed that `UndoMap` must be reconstructed from restored `Operation::Undo` entries. The exact invariants for what constitutes a valid `UndoMap` state (e.g., whether operations can be partially restored) are not fully documented in the codebase and will need verification during Phase 1 implementation.
- **App termination vs tab close:** The "Looks Done But Isn't" checklist in PITFALLS.md flags that `SerializableItem::serialize()` may not fire during abrupt app termination. The correct hook for graceful quit (Cmd-Q) vs crash needs to be confirmed during Phase 3 implementation — specifically whether `Editor::save()` on quit is sufficient or if an additional shutdown hook is needed.
- **`postcard` workspace promotion:** `postcard` 1.1.3 is in the lockfile as a transitive dependency but is not declared in `[workspace.dependencies]`. Promoting it requires adding it to the root `Cargo.toml`. This is a trivial change but must happen before Phase 1 code can compile.

## Sources

### Primary (HIGH confidence)

- Zed codebase: `crates/text/src/text.rs` — `Buffer`, `History`, `HistoryEntry`, `Transaction`, `UndoOperation`, `EditOperation`
- Zed codebase: `crates/text/src/undo_map.rs` — `UndoMap` implementation
- Zed codebase: `crates/editor/src/persistence.rs` — `EditorDb`, `file_folds` pattern (key-by-path precedent)
- Zed codebase: `crates/editor/src/items.rs` — `SerializableItem` lifecycle, `deserialize()` async chain
- Zed codebase: `crates/editor/src/editor.rs` — buffer event subscription pattern at line 2537
- Zed codebase: `crates/language/src/buffer.rs` — `BufferEvent::Saved`, `Reloaded`, `did_reload()`
- Zed codebase: `Cargo.lock` — dependency versions verified (`postcard` 1.1.3, `sha2` 0.10.9, `bincode` 1.2.1)
- [RUSTSEC-2025-0141](https://rustsec.org/advisories/RUSTSEC-2025-0141) — bincode marked unmaintained
- [postcard wire format spec](https://postcard.jamesmunns.com/wire-format) — format stability guarantee since v1.0.0
- [VS Code 1.44 release notes](https://code.visualstudio.com/updates/v1_44) — persistent undo/redo implementation reference
- [Vim undo documentation](https://vimhelp.org/undo.txt.html) — hash invalidation mechanism, no-auto-delete caveat
- [undo-fu-session Emacs package](https://codeberg.org/ideasman42/emacs-undo-fu-session) — feature list and exclusion patterns

### Secondary (MEDIUM confidence)

- [Zed Discussion #16485](https://github.com/zed-industries/zed/discussions/16485) — user pain point: history lost on tab close
- [Zed Issue #4942](https://github.com/zed-industries/zed/issues/4942) — confirmed scope (closed in favor of #15097)
- [Zed Issue #31861](https://github.com/zed-industries/zed/issues/31861) — remote reconnect undo (future consideration)
- [SQLite: Internal Versus External BLOBs](https://sqlite.org/intern-v-extern-blob.html) — 100KB threshold for inline vs. external storage
- [Helix Issue #5287](https://github.com/helix-editor/helix/issues/5287) — cross-editor pattern confirmation

### Tertiary (MEDIUM-LOW confidence)

- [Kakoune Issue #2021](https://github.com/mawww/kakoune/issues/2021) — undo tree validation complexity tradeoffs
- [VS Code Issue #2908](https://github.com/microsoft/vscode/issues/2908) — external edit invalidation approach

---
*Research completed: 2026-03-01*
*Ready for roadmap: yes*
