# Pitfalls Research

**Domain:** Persistent undo/redo history for a CRDT-based Rust code editor (Zed)
**Researched:** 2026-03-01
**Confidence:** HIGH (codebase verified, supplemented with cross-editor evidence from Vim, VSCode, Kakoune)

---

## Critical Pitfalls

### Pitfall 1: Serializing the CRDT Operation Log Instead of the Transaction History

**What goes wrong:**
The `text::Buffer` has two distinct history structures: the `History` struct (containing `undo_stack`, `redo_stack`, and raw `operations: TreeMap<Lamport, Operation>`) and the `UndoMap` (which tracks how many times each edit has been undone). Developers new to Zed's internals often conflate these, and may attempt to serialize the `History.operations` map (every raw edit/undo operation the CRDT has ever seen) rather than the linear `undo_stack`/`redo_stack` transaction lists. The raw operations map includes deferred ops, remote peer ops, undo-of-undo operations, and other CRDT bookkeeping that is unnecessary for restoring local undo/redo and would produce unbounded storage growth for collaborative sessions.

**Why it happens:**
The `Buffer::operations()` public method exposes the full operation TreeMap. It is the obvious "get the history" call. The actual undo stack lives in the private `History` struct and is only accessible through transaction-oriented methods (`peek_undo_stack`, `forget_transaction`, etc.). The distinction between "CRDT operation log" and "user undo stack" is not surfaced at the type level.

**How to avoid:**
Serialize only `History.undo_stack` and `History.redo_stack` (as `Vec<HistoryEntry>`, which contains `Transaction` with `edit_ids: Vec<Lamport>`, plus timestamps). To replay on restore, re-insert the referenced edit operations from the operation map (or store a subset of operations: only those referenced by transactions in the undo/redo stacks). Keep the serialization scope to what a user can actually undo/redo, not the full CRDT event log.

**Warning signs:**
- History file sizes grow disproportionately to the number of user edits
- History files are enormous for files opened in collaborative sessions
- Any collaborative-session peer's edits appear in the "user's" undo stack

**Phase to address:** History data model and serialization design (earliest implementation phase)

---

### Pitfall 2: Stale History Applied to a File Modified Externally

**What goes wrong:**
When a file is modified outside Zed (a formatter, a git checkout, a different editor), the saved undo history no longer corresponds to the buffer's actual content. The `Transaction.edit_ids` reference `EditOperation` entries whose `FullOffset` ranges now point at wrong positions in the new file. Applying an undo restores text to a position in the file that no longer means what it used to — resulting in silent data corruption, not a crash.

**Why it happens:**
Zed already detects external changes via file watching and the `mtime` field stored in `SerializedEditor`. The natural response is to clear the buffer and reload from disk. But persistent undo history is stored separately from the buffer content. If that clearing does not also purge the history file/record, the stale history survives. On next open, it loads successfully and appears valid — the corruption only appears when the user presses cmd-z.

**How to avoid:**
When the file-watching system triggers a buffer reload (which today calls `buffer.did_reload()` and clears the undo stack in-memory), also delete the persisted history record for that file path. The mtime already stored at save time should be compared to disk mtime at load time before restoring history. If they differ, discard the history and log a debug message — do not silently load it. Vim solves this with a content hash; Zed's mtime approach is sufficient if applied at both write-time and read-time.

**Warning signs:**
- After running `gofmt` or `prettier` externally, cmd-z produces unexpected results
- Tests pass with synthetic content but break with real files touched by formatters
- The history restore path does not check mtime before applying

**Phase to address:** Invalidation strategy (same phase as initial persistence write/restore)

---

### Pitfall 3: Blocking the UI Thread During History Serialization

**What goes wrong:**
Serializing `10,000` undo entries plus their associated `EditOperation` payloads (which contain `new_text: Vec<Arc<str>>`) and writing them to disk during tab close happens synchronously on the foreground thread. Zed's architecture requires that all entity updates run on the foreground thread, so it is tempting to perform the serialization inline. For large files with deep history, this produces a visible freeze at the moment the user closes a tab or saves a file.

**Why it happens:**
The existing pattern for saving editor state (scroll position, selections, folds) uses `cx.background_spawn()` to write to SQLite asynchronously. It is easy to forget that serialization itself — converting the in-memory history structs to a binary blob — is CPU-bound work that also needs to leave the foreground thread before the write. Developers may spawn the DB write but perform the serialization before the spawn, still blocking during the encode step.

**How to avoid:**
Capture the data that needs to be serialized (clone the undo/redo stacks) on the foreground thread (required, since entity state is foreground-only), then move the clone into a `cx.background_spawn()` closure where both the encoding and the disk write happen. Keep the foreground work to a minimal snapshot clone — do not encode inside the entity update closure.

**Warning signs:**
- Tab close causes a brief freeze, especially on large files
- Profiling shows the foreground thread blocked in `bincode::serialize` or similar during close
- Serialization is performed before the `cx.background_spawn()` call

**Phase to address:** Implementation of save-on-close hook; verify with a large file performance test

---

### Pitfall 4: Using Item ID as the History Lookup Key

**What goes wrong:**
The existing editor persistence schema uses `(item_id, workspace_id)` as the primary key. `item_id` is a monotonically increasing integer assigned when a workspace item is created in a session — it is not stable across sessions. If the history is keyed on `item_id`, a file's undo history is lost every time Zed restarts (because the file gets a new `item_id`), making the feature appear to work within a session but fail across restarts, which is the primary use case.

**Why it happens:**
`item_id` is the natural join key already used in `editors`, `editor_selections`, and `editor_folds` tables. It is easy to replicate this pattern without noticing that `item_id` is session-scoped. The existing persistence in these tables is only expected to survive within a given `workspace_id` lifespan, which is why it works for scroll positions and selections.

**How to avoid:**
Key persistent undo history on the file's absolute path (as a string), not on `item_id`. The existing `file_folds` table already demonstrates this pattern — it uses `(workspace_id, path)` as its primary key, which was introduced specifically because folds also need to survive tab close and workspace cleanup. Follow the `file_folds` precedent for undo history, not the `editor_selections` precedent.

**Warning signs:**
- Undo history is restored within a session but lost after a full Zed restart
- History lookup queries join on `item_id` rather than `path`
- Tests only cover in-session tab close/reopen, not full restart scenarios

**Phase to address:** Database schema design (before any data is written to the schema)

---

### Pitfall 5: No Format Versioning in the Serialized History

**What goes wrong:**
A Zed update changes the internal representation of `Transaction`, `EditOperation`, or `HistoryEntry` (e.g., adds a field, renames a variant, changes `FullOffset` type precision). Older history files stored in `~/.local/share/zed/` are deserialized with the new code and produce either a deserialization error (best case) or silently corrupt data (worst case if the format is a raw binary struct). The user loses all persisted history with no explanation.

**Why it happens:**
Binary serialization formats (bincode, postcard) are not self-describing. Without an explicit version tag, there is no way to detect that a stored blob was written by an older code version. This is easy to overlook in the initial implementation when only one version exists, and very painful to retrofit later once history files are in the wild.

**How to avoid:**
Write a version byte or magic number as the first byte(s) of every history blob. On read, check the version and either migrate (if a migration path exists) or discard and log (if not). Start at version 1. This adds approximately four bytes of overhead and prevents a class of hard-to-debug corruption. Use a versioned wrapper enum: `enum HistoryBlob { V1(HistoryV1) }` — adding `V2` later is straightforward. Note: `bincode` 2.x has breaking changes from 1.x and is itself in flux; consider `postcard` which is actively maintained and stable.

**Warning signs:**
- Serialization code has no version constant or magic number prefix
- Tests only cover round-trip within the same binary, not across simulated version changes
- The serialization format changes during development without migrating existing test fixtures

**Phase to address:** Serialization format design (before writing any production history blobs)

---

### Pitfall 6: Unbounded History File Growth with No Pruning

**What goes wrong:**
Vim's documentation explicitly notes: "Undo files are never deleted by Vim. You need to delete them yourself." Without automatic pruning, every file a user has ever edited accumulates a history file. Over months of use, the combined storage becomes significant — especially for files that are opened briefly, edited heavily (e.g., auto-generated files, log files), and never revisited. The `max_entries` setting per-file limits depth, but does nothing for breadth (number of files with stored history).

**Why it happens:**
It is natural to focus on the per-file entry limit (`max_entries`) during implementation and not think about aggregate storage across all files. Pruning requires knowing which files a user no longer works on, which requires either a last-accessed timestamp or a periodic sweep comparing stored paths to currently existing files.

**How to avoid:**
Store a `last_accessed_at` timestamp in the history index. On Zed startup, or on a periodic background task, delete history entries for files that either (a) no longer exist on disk or (b) have not been accessed in more than N days (configurable, default 90 days). This is complementary to `max_entries` — both limits apply. The auto-pruning of non-existent files is listed as a requirement in `PROJECT.md` and must be implemented, not just declared.

**Warning signs:**
- The cleanup task is stubbed but never triggered
- No `last_accessed_at` field in the history index
- Integration tests do not simulate file deletion and verify cleanup

**Phase to address:** Database schema design (add `last_accessed_at` from the start); pruning logic (can be a follow-up phase, but the column must exist in the initial schema)

---

### Pitfall 7: History Restored Before Buffer Content Is Fully Loaded

**What goes wrong:**
On file open, Zed asynchronously loads the buffer content (from disk via `Fs`, then potentially waiting for language server registration). The history restore path loads the serialized history from the database, then attempts to apply it to the buffer. If the restore runs before the buffer text is populated, the history's offset-based `EditOperation` entries reference positions in an empty or partially-loaded rope, producing incorrect cursor positions or panics in index operations.

**Why it happens:**
The restore is triggered by the editor's `deserialize` path (see `Editor::deserialize` in `items.rs`), which runs in a spawned async task. The task may complete before or after the buffer's async load task, depending on scheduler ordering. The existing fold restore in `Editor::read_metadata_from_db` already demonstrates this ordering sensitivity — it must run after the buffer is populated to resolve fingerprint offsets correctly.

**How to avoid:**
History restore must happen in the same async task chain that loads the buffer content, after `buffer.did_load()` completes. Do not restore history from a separate concurrent task — sequence it explicitly. Study how `restore_from_db` handles fold restoration (with fingerprint-based position recovery) and apply the same sequencing pattern. Add an explicit assertion or guard: restore history only when `buffer.is_loaded()`.

**Warning signs:**
- History restore fires from a separately detached task
- Tests do not cover the race between buffer load and history restore
- Intermittent test failures depending on task scheduling order

**Phase to address:** History restore implementation; write explicit ordering tests

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Store history in the existing `editors` SQLite table | No new schema | Binary blob in the workspace-scoped table gets wiped on workspace cleanup; history tied to item_id, not file path | Never — use a separate file-path-keyed table |
| Use `item_id` as lookup key | Reuses existing join columns | History lost on every restart (item_id is session-scoped) | Never |
| Serialize the full `History.operations` TreeMap | Simple "dump everything" approach | Unbounded growth; CRDT overhead; collaborative-session pollution | Never |
| Skip format versioning in v1 | Simpler initial code | History files silently corrupt or panic after any format change | Never — add versioning from day one |
| Perform encoding on the foreground thread before `background_spawn` | Slightly simpler code structure | UI freeze on close for large files | Never — move encoding into the background closure |
| Skip pruning in v1 | Faster to ship | Disk usage grows without bound; users discover gigabytes of history data after months | Only if `last_accessed_at` column exists and pruning is scheduled for next milestone |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| `sqlez` write queue | Calling `.write()` from the foreground entity closure | Call `.write()` from inside `cx.background_spawn()` — `write_and_log` helper in `crates/db/src/db.rs` shows the pattern |
| File watcher / buffer reload | Only clearing in-memory history on external edit; leaving persisted history intact | When `buffer.did_reload()` fires, also issue a `DELETE` for the history row keyed on that file path |
| `Editor::cleanup()` (item cleanup on workspace close) | Deleting history rows for files that still exist | `cleanup()` should only purge history for items that are no longer alive — use the same `delete_unloaded_items` pattern but key on file path, not item_id |
| `Editor::deserialize()` ordering | Restoring history before `buffer.is_loaded()` | Sequence history restore after buffer content load; use `buffer.wait_for_version()` or explicit load sequencing |
| SQLite WAL mode | Assuming write is durable at the moment `background_spawn` returns | Zed's DB uses `PRAGMA synchronous=NORMAL` — writes are durable after WAL checkpoint, not immediately; acceptable for undo history but means very recent history could be lost on hard crash |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Serializing 10,000 entries synchronously on close | Tab close freezes for 100–500ms | Move serialization into `cx.background_spawn()`; clone only the needed slice of the undo stack | Already visible on files with deep history; threshold ~500 entries on slow disks |
| Deserializing full history on open, even for small buffers | File open latency increased universally | Defer history deserialization until first undo keystroke (lazy load) | Visible when many tabs are restored on startup with full history |
| Writing the entire history blob on every save | Frequent autosave causes continuous disk writes | Write history only on close or on explicit save, not on autosave; or use a dirty flag | On autosave-every-second setups with active files |
| SQLite write lock contention from concurrent history writes | Tab close stalls waiting for DB write | Zed's SQLite is in WAL mode which allows concurrent reads; writes are serialized through the write queue — this is acceptable by design | Only problematic if history writes are very large and frequent |
| Large `new_text` blobs in history for files with many insertions | History blob is 10x the file size | Consider storing only edit metadata (offsets, lengths) and reconstructing text from the current buffer state if needed; or impose a size cap in addition to entry cap | Files with many large auto-generated insertions (code generation, paste-heavy editing) |

---

## "Looks Done But Isn't" Checklist

- [ ] **Tab close persistence:** History saves on close — verify this also fires when Zed quits entirely (app termination may skip item deactivation hooks)
- [ ] **Full restart recovery:** History is restored after `pkill zed && open -a Zed file.rs` — not just tab close/reopen within a session
- [ ] **External edit invalidation:** Running `echo "x" >> file.rs` in a terminal, then reopening in Zed — verify history is discarded, not loaded
- [ ] **File deleted, then recreated:** History for the old file path must not auto-attach to a new file at the same path with different content — verify via mtime check
- [ ] **max_entries enforcement:** With `max_entries: 100`, verify only the 100 most recent transactions are stored, oldest pruned — not accumulated silently
- [ ] **Feature disabled by default:** With `persistent_undo.enabled: false`, verify zero disk writes and zero performance impact on the close path
- [ ] **Excluded file patterns:** With `exclude: ["*.lock"]`, verify `Cargo.lock` edits never write history
- [ ] **Format version roundtrip:** Deserializing a V1 blob with the current binary still works after a field is added to `HistoryEntry` in a simulated future version
- [ ] **Orphan cleanup:** A file that was deleted from disk no longer has a history row after the next Zed startup

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Wrong lookup key (item_id instead of path) | HIGH | Schema migration required; all existing history data unusable (wrong key means it cannot be retrieved for any file); must drop and recreate table |
| No format versioning, format breaks | HIGH | All history blobs on user machines become unreadable; must ship a "discard stale history" fallback and bump minimum compatible version |
| History applied to externally-modified file | MEDIUM | Add mtime check on restore path; ship fix; existing users lose trust but data corruption is recoverable by re-editing |
| History stored in wrong table (tied to workspace cascade delete) | HIGH | Data is silently deleted on workspace cleanup; no recovery without user noticing; requires schema redesign |
| UI thread blocking on close | LOW | Move encoding to background spawn; no data loss, only UX regression |
| No pruning | LOW | Add startup cleanup sweep; existing bloat can be cleaned retroactively with a one-time migration |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| CRDT op log vs transaction history confusion | Data model design (before writing serialization code) | Code review: confirm only `undo_stack`/`redo_stack` are serialized, not raw `operations` map |
| Stale history on external edit | Invalidation logic (same phase as initial read/write) | Test: save history, modify file externally, reopen — assert history is discarded |
| UI thread blocking | Serialization implementation (use `background_spawn` from the start) | Performance test: close a tab on a file with 10,000 history entries; assert no foreground stall >16ms |
| item_id as lookup key | Database schema design (first phase) | Test: save history, restart Zed, reopen file — assert history is present |
| No format versioning | Serialization format design (first phase) | Test: serialize with V1 code, deserialize with a simulated V2 code that adds a field |
| Unbounded growth | Schema design (add `last_accessed_at` from start); pruning logic (can be second phase) | Test: simulate 1000 deleted files with history rows; run startup sweep; assert rows are gone |
| History restored before buffer loaded | History restore implementation | Test: assert history restore only fires after `buffer.is_loaded()` returns true; no intermittent failures over 100 test runs |

---

## Sources

- Zed codebase: `crates/text/src/text.rs` (History, Transaction, EditOperation, UndoOperation types)
- Zed codebase: `crates/text/src/undo_map.rs` (UndoMap structure)
- Zed codebase: `crates/editor/src/persistence.rs` (EditorDb schema, file_folds pattern)
- Zed codebase: `crates/editor/src/items.rs` (deserialize path, cleanup hook, close sequencing)
- Zed codebase: `crates/db/src/db.rs` (ThreadSafeConnection, WAL mode, write_and_log pattern)
- [Vim: Persistent Undo Documentation](https://vimhelp.org/undo.txt.html) — hash validation, silent failure modes, no-auto-delete pitfall
- [Kakoune issue #2021: Persistent Undo](https://github.com/mawww/kakoune/issues/2021) — undo tree validation complexity, format design tradeoffs
- [VSCode issue #7536: Serialize undo stack](https://github.com/microsoft/vscode/issues/7536) — confirms cross-editor relevance
- [VSCode issue #2908: Keep undo history on external change](https://github.com/microsoft/vscode/issues/2908) — external edit invalidation, "drop and reload" approach
- [Cursor IDE SQLite mutex contention issue #3823](https://github.com/cursor/cursor/issues/3823) — real-world example of SQLite blocking the UI thread
- [Zed blog: CRDTs make multiplayer text editing part of Zed's DNA](https://zed.dev/blog/crdts) — Lamport clock and operation semantics in Zed's buffer model

---

*Pitfalls research for: persistent undo/redo in Zed editor*
*Researched: 2026-03-01*
