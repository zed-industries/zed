# Architecture Research

**Domain:** Persistent undo/redo in Zed editor (Rust, GPUI, sqlez)
**Researched:** 2026-03-01
**Confidence:** HIGH — all findings sourced directly from the Zed codebase

## Standard Architecture

### System Overview

```
┌────────────────────────────────────────────────────────────────────┐
│                         Editor Layer (crates/editor)               │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Editor (Entity<Editor>)                                     │   │
│  │  - Holds Entity<MultiBuffer>                                 │   │
│  │  - Subscribes to buffer events (Saved, Reloaded, Edited)     │   │
│  │  - Drives persist/restore via UndoHistoryDb                  │   │
│  └─────────────┬───────────────────────────────────────────────┘   │
│                │ cx.subscribe(&buffer, ...)                         │
│  ┌─────────────▼───────────────────────────────────────────────┐   │
│  │  items.rs: impl SerializableItem for Editor                  │   │
│  │  - serialize() → DB.save_serialized_editor(...)              │   │
│  │  - deserialize() → restore Editor from DB + file             │   │
│  │  - should_serialize() on Saved/DirtyChanged/BufferEdited     │   │
│  └─────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────┘
                    │
                    │ Entity<Buffer> (language::Buffer)
                    ▼
┌────────────────────────────────────────────────────────────────────┐
│                      Language Layer (crates/language)              │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  language::Buffer (Entity<Buffer>)                           │   │
│  │  - text: TextBuffer (crates/text::Buffer)                    │   │
│  │  - file: Option<Arc<dyn File>>                               │   │
│  │  - saved_version: clock::Global                              │   │
│  │  - saved_mtime: Option<MTime>                                │   │
│  │  - Emits: BufferEvent::Saved, Reloaded, ReloadNeeded,        │   │
│  │           FileHandleChanged                                  │   │
│  └─────────────┬───────────────────────────────────────────────┘   │
└────────────────┼───────────────────────────────────────────────────┘
                 │ .text field (TextBuffer)
                 ▼
┌────────────────────────────────────────────────────────────────────┐
│                       Text Layer (crates/text)                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  text::Buffer                                                │   │
│  │  - history: History { undo_stack, redo_stack, operations }   │   │
│  │  - snapshot: BufferSnapshot { undo_map: UndoMap, ... }       │   │
│  │  - lamport_clock: clock::Lamport                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  History (private)                                           │   │
│  │  - base_text: Rope                                           │   │
│  │  - operations: TreeMap<Lamport, Operation>                   │   │
│  │  - undo_stack: Vec<HistoryEntry>                             │   │
│  │  - redo_stack: Vec<HistoryEntry>                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  UndoMap (SumTree of UndoMapEntry)                           │   │
│  │  - Maps (edit_id, undo_id) → undo_count                      │   │
│  │  - is_undone(edit_id) → bool                                 │   │
│  └─────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────┐
│                    Persistence Layer (NEW)                         │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  UndoHistoryDb (new Domain in crates/editor/src/persistence) │   │
│  │  - Table: undo_history (workspace_id, path, data BLOB,       │   │
│  │            saved_version, mtime_seconds, mtime_nanos)        │   │
│  │  - save_undo_history(workspace_id, path, data) → Result<()>  │   │
│  │  - get_undo_history(workspace_id, path) → Result<Option<..>> │   │
│  │  - delete_undo_history(workspace_id, path) → Result<()>      │   │
│  └─────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────┐
│                    Settings Layer (crates/settings)                │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  PersistentUndoSettings (new, impl Settings)                 │   │
│  │  - enabled: bool (default: false)                            │   │
│  │  - max_entries: usize (default: 10_000)                      │   │
│  │  - exclude: Vec<String> (glob patterns)                      │   │
│  └─────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Crate / File |
|-----------|----------------|--------------|
| `text::Buffer` | Owns the authoritative undo/redo state: `History` (undo_stack, redo_stack, operations) and `UndoMap` | `crates/text/src/text.rs` |
| `UndoMap` | Compact SumTree mapping (edit_id, undo_id) → undo_count; queried to determine if any edit is currently undone | `crates/text/src/undo_map.rs` |
| `History` | Two `Vec<HistoryEntry>` (undo_stack, redo_stack) + `TreeMap<Lamport, Operation>`; fully private to the text crate | `crates/text/src/text.rs` |
| `language::Buffer` | Wraps `TextBuffer`; owns file metadata (`saved_version`, `saved_mtime`); emits `BufferEvent` (Saved, Reloaded, ReloadNeeded) | `crates/language/src/buffer.rs` |
| `Editor` | GPUI entity that holds `Entity<MultiBuffer>`; subscribes to buffer events; drives persist/restore via hooks in `items.rs` | `crates/editor/src/editor.rs`, `items.rs` |
| `impl SerializableItem for Editor` | Existing hook called by Workspace on tab close/open; `serialize()` fires on Saved/DirtyChanged/BufferEdited; `deserialize()` fires on workspace restore | `crates/editor/src/items.rs` |
| `EditorDb` (existing) | SQLite domain for editor state: path, scroll, selections, folds; model for how to add `UndoHistoryDb` | `crates/editor/src/persistence.rs` |
| `UndoHistoryDb` (new) | New `Domain` in editor persistence; stores serialized undo/redo history per (workspace_id, file_path) with mtime for invalidation | `crates/editor/src/persistence.rs` (extend) |
| `PersistentUndoSettings` (new) | New `Settings` impl; provides `enabled`, `max_entries`, `exclude` globs; read via `PersistentUndoSettings::get_global(cx)` | `crates/editor/src/` (new file or extend existing) |
| `WorkspaceDb` | Existing workspace SQLite database; `UndoHistoryDb` depends on it via `FK(workspace_id) REFERENCES workspaces` | `crates/workspace/src/persistence.rs` |

## Recommended Project Structure

All new code lives inside `crates/editor/` — no new crate needed.

```
crates/editor/src/
├── persistence.rs          # EXTEND: add UndoHistoryDb domain + queries
│                           # (alongside existing EditorDb)
├── undo_history.rs         # NEW: serialization/deserialization of History
│                           # struct; HistoryEntry ↔ bytes (bincode)
├── items.rs                # EXTEND: hook serialize()/deserialize() to call
│                           # undo history save/restore
└── editor.rs               # EXTEND: add on_buffer_saved_or_reloaded handler
                            #  that invalidates history on external edit
```

Settings live in `crates/editor/src/` since they apply to editor behavior:

```
crates/editor/src/
└── editor_settings.rs      # EXTEND: add PersistentUndoSettings struct
                            # (existing file already has EditorSettings)
```

### Structure Rationale

- **Extend `persistence.rs` not a new file:** The `file_folds` feature (shipped recently) extended `EditorDb` in the same file. History persistence follows the exact same pattern: new migration SQL, new `Domain` impl, new query methods. Keeping it together avoids fragmenting the persistence boundary.
- **New `undo_history.rs` for serialization:** The `History` struct from `crates/text` is private. Serialization logic (turning `Vec<HistoryEntry>` + `TreeMap<Lamport, Operation>` into bytes and back) is non-trivial and belongs in its own focused file rather than growing `persistence.rs` or `items.rs`.
- **No new crate:** The feature is scoped to the editor's item lifecycle. A new crate would add build graph complexity with no benefit.

## Architectural Patterns

### Pattern 1: File-Level Persistence (the `file_folds` model)

**What:** Store data by `(workspace_id, absolute_file_path)` rather than `(item_id, workspace_id)`. The key difference: item_id is ephemeral (a new item_id is assigned each session), but the file path persists.

**When to use:** Any editor state that should survive tab close + reopen across sessions. Folds already use this. Undo history must use this.

**Trade-offs:** Path-based lookup is slightly more expensive and must handle path changes (file rename/move), but editor_id–based lookup loses data the moment a tab is closed and a new item_id is assigned.

**Example (existing file_folds pattern to replicate):**

```rust
// In persistence.rs: migration SQL
sql! (
    CREATE TABLE undo_history (
        workspace_id INTEGER NOT NULL,
        path TEXT NOT NULL,
        data BLOB NOT NULL,
        saved_version TEXT NOT NULL,  -- serialized clock::Global
        mtime_seconds INTEGER,
        mtime_nanos INTEGER,
        FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE ON UPDATE CASCADE,
        PRIMARY KEY(workspace_id, path)
    );
)

// Query methods on UndoHistoryDb
query! {
    pub async fn save_undo_history(
        workspace_id: WorkspaceId,
        path: &Path,
        data: Vec<u8>,
        saved_version: String,
        mtime_seconds: Option<i64>,
        mtime_nanos: Option<i32>,
    ) -> Result<()> {
        INSERT OR REPLACE INTO undo_history
            (workspace_id, path, data, saved_version, mtime_seconds, mtime_nanos)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
    }
}
```

### Pattern 2: Background Spawn for DB Writes

**What:** All database writes are dispatched via `cx.background_spawn(async move { ... })` and the returned `Task` stored in a field (e.g., `serialize_folds: Option<Task<()>>`). The field acts as a drop guard: if a new write is triggered before the old one completes, the old task is cancelled by being overwritten.

**When to use:** Every sqlez write in Zed uses this. Undo history saves must follow suit — never block the UI thread on a DB write.

**Trade-offs:** The last-write-wins cancellation means very rapid undo+close could theoretically miss one cycle. A SERIALIZATION_THROTTLE_TIME delay (like folds use) mitigates this by batching.

**Example:**

```rust
// In Editor struct
serialize_undo_history: Option<Task<()>>,

// In the save method
self.serialize_undo_history = Some(cx.background_spawn(async move {
    // optional: cx.background_executor().timer(THROTTLE).await;
    DB.save_undo_history(workspace_id, path, data, ...)
        .await
        .log_err();
}));
```

### Pattern 3: Invalidation via `BufferEvent::Reloaded`

**What:** Subscribe to `BufferEvent::Reloaded` (emitted from `language::Buffer::did_reload()` when the file changes externally and is reloaded from disk). On receiving this event, delete the persisted undo history for that file — the existing history no longer maps to the current document state.

**When to use:** Any time the file is modified outside Zed. The `ReloadNeeded` event is a pre-signal; `Reloaded` is the post-signal after the buffer has been updated. Delete on `Reloaded`.

**Trade-offs:** Simple and correct. The alternative — trying to rebase history onto the new file state — is extremely complex (requires CRDT-level merge) and out of scope for v1.

**Example (wiring in editor.rs):**

```rust
cx.subscribe(&buffer, |this, _buffer, event, cx| {
    match event {
        BufferEvent::Saved => this.on_buffer_saved(cx),
        BufferEvent::Reloaded => this.invalidate_persisted_undo_history(cx),
        _ => {}
    }
})
```

### Pattern 4: Bincode for History Serialization

**What:** Use `bincode` (already in the dep tree at `bincode = "1.2.1"`) to serialize `Vec<HistoryEntry>` plus the `UndoMap` state as a compact binary blob. Store the blob in the `data BLOB` column.

**When to use:** For the actual history bytes stored in SQLite. `serde_json` would work but produces 5-10x larger output for the Lamport clock maps; `bincode` is standard in Zed for protocol-level binary encoding.

**What to serialize:** The minimal set needed to fully reconstruct the undo/redo stacks:
1. `undo_stack: Vec<SerializableHistoryEntry>` — transaction id, edit_ids, timestamps
2. `redo_stack: Vec<SerializableHistoryEntry>`
3. `operations: Vec<(Lamport, Operation)>` — the full operation log (needed to reconstruct `UndoMap` state and apply undo/redo correctly)

**Constraint:** `HistoryEntry`, `History`, and `Operation` do not currently derive `Serialize`/`Deserialize`. The new `undo_history.rs` file must define mirror structs (`SerializableHistoryEntry`, `SerializableOperation`, etc.) that derive serde traits, and conversion functions between the text crate's types and these mirror types. This is the primary implementation complexity.

## Data Flow

### Save Flow (Buffer Close / Save)

```
BufferEvent::Saved (or SerializableItem::serialize called on tab close)
    │
    ▼
Editor::on_buffer_saved() [in editor.rs]
    │  reads: PersistentUndoSettings::get_global(cx)
    │  checks: enabled? file excluded? settings
    │
    ▼
collect history snapshot:
    buffer.read(cx).text.history.undo_stack  (via pub accessor)
    buffer.read(cx).text.history.redo_stack
    buffer.read(cx).text.history.operations
    buffer.read(cx).saved_version
    buffer.read(cx).saved_mtime
    │
    ▼
prune to max_entries (from settings)
    │
    ▼
undo_history::serialize(undo_stack, redo_stack, operations)
    → Vec<u8> via bincode
    │
    ▼
cx.background_spawn(async move {
    DB.save_undo_history(workspace_id, path, data, saved_version, mtime)
        .await.log_err()
})  → stored in self.serialize_undo_history: Option<Task<()>>
```

### Restore Flow (Buffer Open / Workspace Restore)

```
Editor::read_metadata_from_db() [called from deserialize() in items.rs]
    │
    ▼
PersistentUndoSettings::get_global(cx)
    checks: enabled? file excluded?
    │
    ▼
DB.get_undo_history(workspace_id, abs_path)
    → Option<(data: Vec<u8>, saved_version: String, mtime: Option<MTime>)>
    │
    ├── None → no history, proceed normally
    │
    └── Some(row)
            │
            ▼
        validate: does saved_version match buffer.saved_version?
                  does mtime match buffer.saved_mtime?
            │
            ├── mismatch → delete stale row, proceed normally
            │              DB.delete_undo_history(workspace_id, path)
            │
            └── match
                    │
                    ▼
                undo_history::deserialize(data: Vec<u8>)
                    → (undo_stack, redo_stack, operations)
                    │
                    ▼
                buffer.update(cx, |buffer, cx| {
                    buffer.text.history.restore(undo_stack, redo_stack, operations)
                })
                    │
                    ▼
                Editor reflects restored undo/redo state immediately
```

### Invalidation Flow (External Edit)

```
File changes on disk (detected by Fs watcher in project crate)
    │
    ▼
project::Buffer::file_updated() → emits BufferEvent::ReloadNeeded
    │
    ▼
(user confirms, or auto-reload fires)
    │
    ▼
language::Buffer::did_reload() → emits BufferEvent::Reloaded
    │
    ▼
Editor subscription fires → Editor::invalidate_persisted_undo_history()
    │
    ▼
cx.background_spawn(async move {
    DB.delete_undo_history(workspace_id, path).await.log_err()
})
```

### Settings Read Flow

```
Any code that needs to check settings:
    PersistentUndoSettings::get_global(cx).enabled
    PersistentUndoSettings::get(Some(location), cx).max_entries
    PersistentUndoSettings::get_global(cx).exclude  (Vec<String> globs)
```

## Integration Points

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `text::Buffer` → new code | `text::Buffer` must expose `history` accessors: `pub fn undo_stack(&self) -> &[HistoryEntry]`, `pub fn redo_stack(&self) -> &[HistoryEntry]`, `pub fn operations(&self) -> &TreeMap<Lamport, Operation>` (already exists), and a restore method | Currently `history` is a private field; partial exposure exists via `operations()` and `peek_undo_stack()`. Need to add `undo_stack()` and `redo_stack()` accessors, plus a `restore_history()` method. |
| `language::Buffer` → Editor | Via `BufferEvent` subscription. Already subscribed in editor via `cx.subscribe_in(&multi_buffer, window, Self::on_buffer_event)`. New event arms added for save/reload. | HIGH confidence — pattern exists at line 2537 of editor.rs |
| `Editor` → `UndoHistoryDb` | Direct call via `DB.save_undo_history(...)` on background thread, same as `DB.save_file_folds(...)` | Mirrors exact existing pattern |
| `EditorDb` → `WorkspaceDb` | Foreign key dependency. `UndoHistoryDb` must declare `[WorkspaceDb]` as a dependency in `static_connection!` macro, same as `EditorDb` does today | See `db::static_connection!(DB, EditorDb, [WorkspaceDb])` in persistence.rs |
| `items.rs` → `undo_history.rs` | Function calls to `serialize()` / `deserialize()` in the new module | Internal to `crates/editor` |

### Key API Additions Required in `crates/text`

The `History` struct is private. To avoid copying all of Zed's text internals, we need:

```rust
// crates/text/src/text.rs — new public accessors on Buffer
impl Buffer {
    pub fn undo_stack(&self) -> &[HistoryEntry] {
        &self.history.undo_stack
    }

    pub fn redo_stack(&self) -> &[HistoryEntry] {
        &self.history.redo_stack
    }

    pub fn restore_history(
        &mut self,
        undo_stack: Vec<HistoryEntry>,
        redo_stack: Vec<HistoryEntry>,
        operations: Vec<(clock::Lamport, Operation)>,
    ) {
        self.history.undo_stack = undo_stack;
        self.history.redo_stack = redo_stack;
        for (timestamp, op) in operations {
            self.history.operations.insert(timestamp, op);
        }
        // Also restore UndoMap from operations
        self.snapshot.undo_map = UndoMap::default();
        for op in self.history.operations.values() {
            if let Operation::Undo(undo) = op {
                self.snapshot.undo_map.insert(undo);
            }
        }
    }
}
```

`Transaction` and `HistoryEntry` also need `#[derive(Clone)]` — they already have it. They need serde derives added or mirror structs defined.

## Build Order

Dependencies between components determine implementation order:

```
Phase 1: Text layer API surface
    text::Buffer gains undo_stack(), redo_stack(), restore_history() accessors
    HistoryEntry, Transaction, EditOperation, UndoOperation get serde derives
    (or mirror structs are defined in editor/src/undo_history.rs)
    ↓

Phase 2: Serialization module
    crates/editor/src/undo_history.rs:
        serialize(undo_stack, redo_stack, operations) → Vec<u8>
        deserialize(data: Vec<u8>) → (undo_stack, redo_stack, operations)
    ↓

Phase 3: Database schema
    UndoHistoryDb Domain added to crates/editor/src/persistence.rs
    Migration SQL: undo_history table
    Query methods: save, get, delete, prune
    ↓

Phase 4: Settings
    PersistentUndoSettings in crates/editor/src/editor_settings.rs
    Registered in zed crate initialization
    ↓

Phase 5: Editor integration
    Editor::on_buffer_saved() → triggers save
    Editor::invalidate_persisted_undo_history() → triggered by Reloaded event
    Editor::read_metadata_from_db() extended → restore on open
    Pruning logic (max_entries, exclude globs) applied during save
```

## Anti-Patterns

### Anti-Pattern 1: Using `item_id` as the History Key

**What people do:** Key undo history by `(item_id, workspace_id)` like the old `editor_folds` schema did.

**Why it's wrong:** `item_id` is generated fresh for each editor instantiation. Closing a tab and reopening the same file produces a different `item_id`. This is exactly why `file_folds` was migrated from `editor_folds` to a path-based table. Undo history keyed by `item_id` would never survive tab close.

**Do this instead:** Key by `(workspace_id, abs_path)` exactly as `file_folds` does.

### Anti-Pattern 2: Storing History in the `editors` Table

**What people do:** Add new columns to the existing `editors` table for the history blob.

**Why it's wrong:** The `editors` table is keyed by `(item_id, workspace_id)` — it has the same item_id problem. It also gets cleaned up aggressively by `delete_unloaded_items()` during workspace restore. History data would be lost whenever Zed restores.

**Do this instead:** Separate `undo_history` table keyed by `(workspace_id, path)`.

### Anti-Pattern 3: Blocking the Foreground Thread on DB Writes

**What people do:** Call `DB.save_undo_history(...).await` directly inside an event handler or `update` closure.

**Why it's wrong:** GPUI's entity update closures are synchronous on the foreground thread. Awaiting a DB write here would deadlock or panic. All sqlez writes must go through `cx.background_spawn(...)`.

**Do this instead:** Use `cx.background_spawn(async move { DB.save_undo_history(...).await.log_err() })` and store the task handle to prevent cancellation.

### Anti-Pattern 4: Restoring History Without Validating mtime

**What people do:** Restore serialized history whenever a file is opened, regardless of whether the file has changed since the history was saved.

**Why it's wrong:** If the file was modified externally (or by another editor, git checkout, etc.) between sessions, the old history's operations reference text positions that no longer exist. Applying undo would corrupt the buffer to nonsense.

**Do this instead:** Store `saved_version` (the buffer's `clock::Global` at save time) and `mtime` alongside the history blob. On restore, compare both against the current buffer state. If either mismatches, discard the history and delete the row.

### Anti-Pattern 5: Serializing the Entire `text::Buffer`

**What people do:** Serialize the entire buffer state (including all fragment trees, visible text, etc.) to avoid having to add targeted accessors to `text::Buffer`.

**Why it's wrong:** This is massively over-engineered — the buffer text is already on disk (the file). Only the undo/redo stacks need to survive. A full buffer serialization would be hundreds of kilobytes per file and completely redundant.

**Do this instead:** Serialize only `undo_stack`, `redo_stack`, and `operations` (the operation log). The buffer text is read from disk as normal; history is layered on top.

## Sources

- `crates/text/src/text.rs` — `Buffer`, `History`, `HistoryEntry`, `Transaction`, `UndoOperation`, `EditOperation` (direct code read)
- `crates/text/src/undo_map.rs` — `UndoMap` implementation (direct code read)
- `crates/language/src/buffer.rs` — `BufferEvent`, `language::Buffer` struct, `did_reload()`, `file_updated()` (direct code read)
- `crates/editor/src/persistence.rs` — `EditorDb`, `SerializedEditor`, `file_folds` schema and migration pattern (direct code read)
- `crates/editor/src/items.rs` — `impl SerializableItem for Editor`, `serialize()`, `deserialize()`, `read_metadata_from_db()` (direct code read)
- `crates/editor/src/editor.rs` — `on_buffer_event` subscription, `save_file_folds` pattern, `serialize_folds: Option<Task<()>>` field pattern (direct code read)
- `crates/db/src/db.rs` — `open_db`, `static_connection!` macro, `write_and_log` utility (direct code read)
- `crates/db/src/kvp.rs` — `KeyValueStore` as reference for Domain impl pattern (direct code read)
- `crates/sqlez/src/domain.rs` — `Domain` trait, `Migrator` (direct code read)
- `crates/workspace/src/persistence.rs` — `WorkspaceDb`, `SerializableItem` trait, item lifecycle hooks (direct code read)
- `crates/workspace/src/item.rs` — `SerializableItem` trait definition, `serialize()` / `deserialize()` / `should_serialize()` (direct code read)

---
*Architecture research for: Persistent undo/redo in Zed editor*
*Researched: 2026-03-01*
