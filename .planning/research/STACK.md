# Stack Research

**Domain:** Persistent undo/redo history serialization and storage for a Rust code editor (Zed)
**Researched:** 2026-03-01
**Confidence:** HIGH — all technologies verified against Zed lockfile and codebase; serialization format recommendations verified against official advisories and documentation

---

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| `serde` + `#[derive(Serialize, Deserialize)]` | 1.0.221 (workspace) | Derive serialization traits on `HistoryEntry`, `Transaction`, and serialized wrapper types | Already a workspace dependency with `derive` feature enabled. Zero-cost derive macros. Industry standard — every Rust binary format works through serde's data model. No alternative considered. |
| `postcard` | 1.1.3 (already in lockfile) | Binary serialization format for history blobs | **Do not use bincode.** Bincode is marked unmaintained (RUSTSEC-2025-0141) as of 2025 due to maintainer doxxing incident. Postcard is the recommended replacement: stable wire format since v1.0.0, actively maintained, 60+ contributors, 7,000+ repos. Smaller output than bincode via varint encoding. Wire format is documented and stable — breaking changes require a major version bump. Already present in Zed's lockfile as a transitive dependency. |
| `sqlez` (Zed internal) | workspace | SQLite abstraction for history index (file path → metadata mapping) | Zed's existing database layer. Used by `EditorDb`, `WorkspaceDb`, and every other persistence domain. Provides typed statements, migrations, and thread-safe connections. The `Domain` + `MIGRATIONS` pattern is the correct way to add a new persistence domain. Do not introduce rusqlite, SQLx, or SeaORM for this feature — sqlez already covers the use case. |
| `sha2` | 0.10.9 (workspace) | Content hash for external-edit invalidation | Already a workspace dependency (`sha2 = "0.10"`). Used in `crates/project/src/lsp_store.rs` and `agent_server_store.rs`. SHA-256 of the file contents stored alongside the undo history row; on restore, recompute and compare. If hash differs, discard history. mtime is insufficient on its own (can be identical after a git stash/pop on some filesystems); a content hash is definitive. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde_json` | 1.0.144 (workspace) | JSON serialization for settings schema | Only for `persistent_undo` settings structs that feed into `schemars` for JSON schema generation. The history blob itself must use `postcard`, not JSON — JSON is 5-10x larger than binary for numeric-heavy data like Lamport timestamps and byte offsets. |
| `anyhow` | 1.0.86 (workspace) | Error propagation through history read/write path | Already ubiquitous in Zed. Use `anyhow::Result` for all history I/O functions. Do not use `unwrap()` or `expect()` on deserialization; treat corrupt/stale history as a recoverable error (discard and log). |
| `schemars` | 1.0 (workspace) | JSON Schema generation for settings structs | Use `#[derive(JsonSchema)]` on the `PersistentUndoSettings` struct so the settings system validates user configuration. Follows the existing pattern in `crates/settings/`. |
| `log` | 0.4.16 (workspace) | Debug logging for history discard events | When history is discarded (mtime mismatch, hash mismatch, deserialization error), emit `log::debug!()` with the file path and reason. Users never see this; it surfaces during development and bug reports. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `./script/clippy` | Linting | Use instead of `cargo clippy` per CLAUDE.md. The sqlez domain pattern with macro-generated queries bypasses some clippy lints — verify the new domain compiles cleanly with clippy before merging. |
| GPUI test framework | Integration testing | Use `#[gpui::test]` for all tests involving entity state (Buffer, Editor). Use `cx.background_executor().timer(duration).await` instead of `smol::Timer` per CLAUDE.md. Tests for history persistence must cover the full close-restart-reopen cycle, not just in-session save/restore. |
| `tempfile` | Test fixtures | Already in dev-dependencies. Use `tempfile::Builder::new().prefix(...).tempdir()` for isolated SQLite databases in tests, following the pattern in `crates/db/src/db.rs`. |

---

## Installation

```toml
# In the new crate's Cargo.toml (e.g., crates/editor/Cargo.toml additions)
# All of these are already workspace dependencies — just add them to [dependencies]:
serde = { workspace = true }
postcard = { workspace = true }       # Add postcard to workspace Cargo.toml first
sha2 = { workspace = true }
anyhow = { workspace = true }
log = { workspace = true }
schemars = { workspace = true }
db = { workspace = true }             # Pulls in sqlez transitively

# Add postcard to workspace Cargo.toml [workspace.dependencies]:
postcard = "1.1"
```

```toml
# No new crate needed — the domain goes in crates/editor/ alongside EditorDb
# (or a new crates/persistent_undo/ crate if scope warrants it)
```

---

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Binary serialization format | `postcard` 1.1.3 | `bincode` 1.2.1 | **bincode is marked unmaintained** (RUSTSEC-2025-0141, 2025). The advisory explicitly recommends postcard. Zed's current use of bincode is limited to crash report serialization (`crates/crashes`) — that usage is not blocking, but new code should use postcard. |
| Binary serialization format | `postcard` 1.1.3 | `rkyv` | rkyv uses zero-copy deserialization via archive pointers, which is excellent for large immutable data but requires unsafe code and a more complex API surface. Undo history is read rarely (once per file open) and is not a hot path. Postcard's simplicity and serde compatibility are more valuable than rkyv's speed advantage here. rkyv is already in Zed's lockfile as a transitive dependency but is not used in application code. |
| Binary serialization format | `postcard` 1.1.3 | `bitcode` | bitcode is fast and compact but less ecosystem-established than postcard and not already in Zed's dependency graph. No meaningful advantage over postcard for this use case. |
| Binary serialization format | `postcard` 1.1.3 | JSON (`serde_json`) | JSON is human-readable but 5-10x larger than binary for numeric data (Lamport clocks, byte offsets). History blobs for deeply-edited files could reach megabytes in JSON; postcard keeps them in the kilobyte range. |
| Invalidation mechanism | SHA-256 content hash | mtime only | mtime is insufficient for reliable invalidation. On macOS and Linux, `git stash` and `git stash pop` can restore a file to its previous mtime, producing a mtime match on different content. SHA-256 is definitive. sha2 is already a workspace dependency. |
| Invalidation mechanism | SHA-256 content hash | BLAKE3 | BLAKE3 is faster but is not currently a Zed workspace dependency. SHA-256 is slower to compute but only called once per file open — the difference is immeasurable at that frequency. Prefer the already-present dependency. |
| Storage layer | `sqlez` (index) + binary files (blobs) | SQLite BLOBs only | SQLite's own research (sqlite.org/intern-v-extern-blob.html) shows BLOBs smaller than 100KB are faster in-database; larger BLOBs are faster as external files. Undo history for a file with 10,000 entries may exceed 100KB. Recommended approach: store metadata and content hash in sqlez (fast lookup, easy cleanup), store the binary blob as a file in Zed's data directory alongside the existing SQLite files. This keeps the main database lean and allows large histories without SQLite page fragmentation. |
| Storage layer | `sqlez` (index) + binary files (blobs) | `sqlez` BLOBs only | Simpler implementation but risks SQLite page fragmentation on large blobs and makes the main database large. Acceptable for v1 if history is bounded tightly (10,000 entries generates roughly 50-200KB depending on edit sizes — within the 100KB threshold where SQLite is faster). If the team prefers simplicity, inline BLOBs in sqlez are acceptable for v1 with a note to revisit if storage grows. |
| Storage layer | `sqlez` (index) + binary files (blobs) | Separate SQLite database | Zed's pattern (`crates/db/src/db.rs`) already opens multiple SQLite databases by scope. A dedicated `undo_history.sqlite` is consistent with this pattern and keeps undo data isolated from the main workspace database. Trade-off: slightly more file handles and migration complexity. **Recommended approach for production:** dedicated database file, keyed by file path, separate from `db.sqlite`. |

---

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `bincode` (any version) | RUSTSEC-2025-0141: marked unmaintained in 2025 by the maintainer team due to doxxing incident. No patches will be released. The team considers 1.3.3 "finished" but ecosystem has moved on. | `postcard` 1.1.3 |
| `bincode` 2.x alpha | bincode 2.x was in-progress before the project was abandoned. It has a different wire format from 1.x, no stable release, and is incompatible in both directions. | `postcard` 1.1.3 |
| `rusqlite` / `sqlx` / `sea-orm` directly | Zed has a custom SQLite abstraction (`sqlez`) with its own migration system, thread-safe connection model, and macro-based typed queries. Bypassing it introduces an inconsistent persistence pattern and requires duplicating connection lifecycle management. | `sqlez` via `crates/db` |
| JSON for history blobs | 5-10x larger than binary. For 10,000 undo entries on a heavily-edited file, JSON history could reach 5-20MB. Binary stays in the 100KB-2MB range. | `postcard` via `serde` |
| `item_id` as history lookup key | `item_id` is session-scoped (assigned when a workspace item is created). It does not survive a Zed restart. Keying history on `item_id` means history is always lost after a full restart — the primary use case of the feature. | Absolute file path (string key), following the `file_folds` table precedent in `crates/editor/src/persistence.rs` |
| mtime as the sole invalidation signal | `git stash pop` and some filesystem operations restore the original mtime while changing content. mtime-only invalidation silently applies stale history to wrong content. | SHA-256 content hash (sha2 crate, already a workspace dependency) |

---

## Stack Patterns by Scenario

**For the history blob format:**
- Wrap the serialized history in a versioned enum before encoding with postcard:
  ```rust
  #[derive(Serialize, Deserialize)]
  enum HistoryBlob {
      V1(SerializedUndoHistory),
  }
  ```
  This makes future format migrations explicit and detectable without a separate version field.

**For the database schema (sqlez domain):**
- Follow the `file_folds` pattern in `crates/editor/src/persistence.rs`:
  - Key on `(path TEXT, workspace_id INTEGER)` — path is stable across restarts, workspace_id enables cascade delete cleanup
  - Add `last_accessed_at INTEGER` from the start (Unix timestamp) to enable pruning
  - Add `content_hash TEXT` (hex-encoded SHA-256) and `mtime_seconds INTEGER` / `mtime_nanos INTEGER` for invalidation (mtime as a fast pre-filter, hash as the definitive check)
  - Store the binary blob as `history_data BLOB` for v1 (simpler than a file-per-history approach; revisit if blob sizes exceed 100KB regularly)

**For the settings struct:**
- Follow the pattern in `crates/settings/`:
  ```rust
  #[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
  pub struct PersistentUndoSettings {
      pub enabled: Option<bool>,
      pub max_entries: Option<usize>,
      pub exclude: Option<Vec<String>>,
  }
  ```

**For the async write path:**
- Clone the undo/redo stacks on the foreground thread, then move into `cx.background_spawn()`:
  ```rust
  let history_snapshot = editor.read(cx).take_history_snapshot(); // clone on foreground
  cx.background_spawn(async move {
      let blob = postcard::to_allocvec(&HistoryBlob::V1(history_snapshot))?;
      DB.save_undo_history(path, content_hash, mtime, blob).await
  }).detach_and_log_err(cx);
  ```

---

## Version Compatibility

| Package | Version in Lockfile | Workspace Declaration | Compatibility Notes |
|---------|---------------------|-----------------------|---------------------|
| `serde` | 1.0.228 | `1.0.221` | Stable, no breaking changes since 1.0. Derive macros are fully compatible across patch versions. |
| `postcard` | 1.1.3 | Not yet declared (add as `"1.1"`) | Wire format stable since 1.0.0; breaking changes require major bump to 2.0. Safe to add without compatibility concerns. |
| `sha2` | 0.10.9 | `"0.10"` | Part of the RustCrypto family. 0.10.x series is stable. |
| `bincode` | 1.2.1 | `"1.2.1"` | Already in workspace but marked unmaintained (RUSTSEC-2025-0141). **Do not use for new code.** Existing usages in `crates/crashes` and `crates/zed` (GPU specs) are low-risk technical debt. |

---

## Sources

- [RUSTSEC-2025-0141: bincode is unmaintained](https://rustsec.org/advisories/RUSTSEC-2025-0141) — HIGH confidence: official RustSec advisory
- [postcard docs.rs 1.1.3](https://docs.rs/postcard/latest/postcard/) — HIGH confidence: official documentation, verified current version
- [postcard wire format specification](https://postcard.jamesmunns.com/wire-format) — HIGH confidence: stable since v1.0.0, maintained by Mozilla sponsor
- [SQLite: Internal Versus External BLOBs](https://sqlite.org/intern-v-extern-blob.html) — HIGH confidence: official SQLite documentation, 100KB threshold for inline vs. external storage
- Zed codebase: `crates/editor/src/persistence.rs` — HIGH confidence: verified existing sqlez domain pattern, `file_folds` key-by-path precedent
- Zed codebase: `crates/text/src/text.rs` — HIGH confidence: verified `History`, `Transaction`, `HistoryEntry` struct shapes; confirmed no `serde` derives currently on these types
- Zed codebase: `crates/text/src/undo_map.rs` — HIGH confidence: verified `UndoMap` structure (SumTree-based, would need custom serialization)
- Zed codebase: `Cargo.lock` — HIGH confidence: verified postcard 1.1.3 already present as transitive dep; serde 1.0.228; sha2 0.10.9
- Zed codebase: `Cargo.toml` workspace — HIGH confidence: bincode 1.2.1 declared; sha2 0.10; serde with derive feature
- [Is it better to use bincode or postcard?](https://users.rust-lang.org/t/is-it-better-to-use-bincode-or-postcard/88740) — MEDIUM confidence: Rust forum community consensus
- [Rust serialization benchmark](https://github.com/djkoloski/rust_serialization_benchmark) — MEDIUM confidence: performance data, not specific to this use case

---

*Stack research for: persistent undo/redo history in Zed editor (Rust)*
*Researched: 2026-03-01*
