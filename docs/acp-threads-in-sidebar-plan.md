# Plan: Show ACP Threads in the Sidebar (Revised)

## Problem

The sidebar currently only shows **Zed-native agent threads** (from `ThreadStore`/`ThreadsDatabase`). ACP threads (Claude Code, Codex, Gemini, etc.) are invisible in the sidebar once they're no longer live.

## Root Cause

`ThreadStore` and `ThreadsDatabase` only persist metadata for native threads. When `rebuild_contents` populates the sidebar, it reads from `ThreadStore` for historical threads and overlays live info from the `AgentPanel` — but non-native threads never get written to `ThreadStore`, so once they stop being live, they disappear.

## Solution Overview (Revised)

**Key change from the original plan:** We completely remove the sidebar's dependency on `ThreadStore`. Instead, the `Sidebar` itself owns a **single, unified persistence layer** — a new `SidebarDb` domain stored in the workspace DB — that tracks metadata for _all_ thread types (native and ACP). The sidebar becomes the single source of truth for what threads appear in the list.

### Why Remove the ThreadStore Dependency?

1. **Single responsibility** — The sidebar is the only consumer of "which threads to show in the list." Having it depend on `ThreadStore` (which exists primarily for native agent save/load) creates an indirect coupling that makes ACP integration awkward.
2. **No merge logic** — The original plan required merging native `ThreadStore` data with a separate `AcpThreadMetadataDb` in `ThreadStore::reload`. By moving all sidebar metadata into one place, there's nothing to merge.
3. **Simpler data flow** — Writers (native agent, ACP connections) push metadata to the sidebar DB. The sidebar reads from one table. No cross-crate coordination needed.
4. **ThreadStore stays focused** — `ThreadStore` continues to manage native thread blob storage (save/load message data) without being polluted with sidebar display concerns.

### Architecture

```
  ┌─────────────────────┐      ┌─────────────────────────┐
  │    NativeAgent      │      │   ACP Connections       │
  │  (on save_thread)   │      │ (on create/update/list) │
  └──────────┬──────────┘      └──────────┬──────────────┘
             │                            │
             │   save_sidebar_thread()    │
             └──────────┬─────────────────┘
                        ▼
              ┌───────────────────┐
              │   SidebarDb       │
              │  (workspace DB)   │
              │  sidebar_threads  │
              └────────┬──────────┘
                       │
                       ▼
              ┌───────────────────┐
              │     Sidebar       │
              │ rebuild_contents  │
              └───────────────────┘
```

---

## Step 1: Create `SidebarDb` Domain in `sidebar.rs`

**File:** `crates/agent_ui/src/sidebar.rs`

Add a `SidebarDb` domain using `db::static_connection!`, co-located in the sidebar module (or a small `persistence` submodule within `sidebar.rs` if it helps organization, but keeping it in the same file is fine for now).

### Schema

```rust
use db::{
    sqlez::{
        bindable::Column, domain::Domain, statement::Statement,
        thread_safe_connection::ThreadSafeConnection,
    },
    sqlez_macros::sql,
};

/// Lightweight metadata for any thread (native or ACP), enough to populate
/// the sidebar list and route to the correct load path when clicked.
#[derive(Debug, Clone)]
pub struct SidebarThreadRow {
    pub session_id: acp::SessionId,
    /// `None` for native Zed threads, `Some("claude-code")` etc. for ACP agents.
    pub agent_name: Option<String>,
    pub title: SharedString,
    pub updated_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub folder_paths: PathList,
}

pub struct SidebarDb(ThreadSafeConnection);

impl Domain for SidebarDb {
    const NAME: &str = stringify!(SidebarDb);

    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS sidebar_threads(
            session_id TEXT PRIMARY KEY,
            agent_name TEXT,
            title TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            created_at TEXT,
            folder_paths TEXT,
            folder_paths_order TEXT
        ) STRICT;
    )];
}

db::static_connection!(SIDEBAR_DB, SidebarDb, []);
```

### CRUD Methods

```rust
impl SidebarDb {
    /// Upsert metadata for a thread (native or ACP).
    pub async fn save(&self, row: &SidebarThreadRow) -> Result<()> {
        let id = row.session_id.0.clone();
        let agent_name = row.agent_name.clone();
        let title = row.title.to_string();
        let updated_at = row.updated_at.to_rfc3339();
        let created_at = row.created_at.map(|dt| dt.to_rfc3339());
        let serialized = row.folder_paths.serialize();
        let (fp, fpo) = if row.folder_paths.is_empty() {
            (None, None)
        } else {
            (Some(serialized.paths), Some(serialized.order))
        };

        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "INSERT INTO sidebar_threads(session_id, agent_name, title, updated_at, created_at, folder_paths, folder_paths_order)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                 ON CONFLICT(session_id) DO UPDATE SET
                     agent_name = excluded.agent_name,
                     title = excluded.title,
                     updated_at = excluded.updated_at,
                     folder_paths = excluded.folder_paths,
                     folder_paths_order = excluded.folder_paths_order",
            )?;
            let mut i = stmt.bind(&id, 1)?;
            i = stmt.bind(&agent_name, i)?;
            i = stmt.bind(&title, i)?;
            i = stmt.bind(&updated_at, i)?;
            i = stmt.bind(&created_at, i)?;
            i = stmt.bind(&fp, i)?;
            stmt.bind(&fpo, i)?;
            stmt.exec()
        })
        .await
    }

    /// List all sidebar thread metadata, ordered by updated_at descending.
    pub fn list(&self) -> Result<Vec<SidebarThreadRow>> {
        self.select::<SidebarThreadRow>(
            "SELECT session_id, agent_name, title, updated_at, created_at, folder_paths, folder_paths_order
             FROM sidebar_threads
             ORDER BY updated_at DESC"
        )?(())
    }

    /// List threads for a specific folder path set.
    pub fn list_for_paths(&self, paths: &PathList) -> Result<Vec<SidebarThreadRow>> {
        let serialized = paths.serialize();
        self.select_bound::<String, SidebarThreadRow>(sql!(
            SELECT session_id, agent_name, title, updated_at, created_at, folder_paths, folder_paths_order
            FROM sidebar_threads
            WHERE folder_paths = ?
            ORDER BY updated_at DESC
        ))?(serialized.paths)
    }

    /// Look up a single thread by session ID.
    pub fn get(&self, session_id: &acp::SessionId) -> Result<Option<SidebarThreadRow>> {
        let id = session_id.0.clone();
        self.select_row_bound::<Arc<str>, SidebarThreadRow>(sql!(
            SELECT session_id, agent_name, title, updated_at, created_at, folder_paths, folder_paths_order
            FROM sidebar_threads
            WHERE session_id = ?
        ))?(id)
    }

    /// Return the total number of rows in the table.
    pub fn count(&self) -> Result<usize> {
        let count: (i32, i32) = self.select_row(sql!(
            SELECT COUNT(*) FROM sidebar_threads
        ))?(())?.unwrap_or_default();
        Ok(count.0 as usize)
    }

    /// Delete metadata for a single thread.
    pub async fn delete(&self, session_id: acp::SessionId) -> Result<()> {
        let id = session_id.0;
        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "DELETE FROM sidebar_threads WHERE session_id = ?",
            )?;
            stmt.bind(&id, 1)?;
            stmt.exec()
        })
        .await
    }

    /// Delete all thread metadata.
    pub async fn delete_all(&self) -> Result<()> {
        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "DELETE FROM sidebar_threads",
            )?;
            stmt.exec()
        })
        .await
    }
}
```

### `Column` Implementation

```rust
impl Column for SidebarThreadRow {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (id, next): (Arc<str>, i32) = Column::column(statement, start_index)?;
        let (agent_name, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (title, next): (String, i32) = Column::column(statement, next)?;
        let (updated_at_str, next): (String, i32) = Column::column(statement, next)?;
        let (created_at_str, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_str, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_order_str, next): (Option<String>, i32) = Column::column(statement, next)?;

        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);
        let created_at = created_at_str
            .as_deref()
            .map(DateTime::parse_from_rfc3339)
            .transpose()?
            .map(|dt| dt.with_timezone(&Utc));

        let folder_paths = folder_paths_str
            .map(|paths| {
                PathList::deserialize(&util::path_list::SerializedPathList {
                    paths,
                    order: folder_paths_order_str.unwrap_or_default(),
                })
            })
            .unwrap_or_default();

        Ok((
            SidebarThreadRow {
                session_id: acp::SessionId::new(id),
                agent_name,
                title: title.into(),
                updated_at,
                created_at,
                folder_paths,
            },
            next,
        ))
    }
}
```

**Key points:**

- `SIDEBAR_DB` is a `LazyLock` static — initialized on first use, no manual connection management.
- The `agent_name` column is `NULL` for native Zed threads and a string like `"claude-code"` for ACP agents. This replaces the `agent_type` field from the original plan.
- The DB file lives alongside other `static_connection!` databases.
- `ThreadsDatabase` and `ThreadStore` are **completely unchanged** by this step.

---

## Step 2: Replace `ThreadStore` Reads in `rebuild_contents` with `SidebarDb` Reads

**File:** `crates/agent_ui/src/sidebar.rs`

### Remove `ThreadStore` Dependency

1. **Remove** `ThreadStore::global(cx)` and `ThreadStore::try_global(cx)` from `Sidebar::new` and `rebuild_contents`.
2. **Remove** the `cx.observe_in(&thread_store, ...)` subscription that triggers `update_entries` when `ThreadStore` changes.
3. **Replace** `thread_store.read(cx).threads_for_paths(&path_list)` calls with `SIDEBAR_DB.list_for_paths(&path_list)` (or read all rows once at the top of `rebuild_contents` and index them in memory, which is simpler and avoids repeated DB calls).

### New Data Flow in `rebuild_contents`

```rust
fn rebuild_contents(&mut self, cx: &App) {
    // ... existing workspace iteration setup ...

    // Read ALL sidebar thread metadata once, index by folder_paths.
    let all_sidebar_threads = SIDEBAR_DB.list().unwrap_or_default();
    let mut threads_by_paths: HashMap<PathList, Vec<SidebarThreadRow>> = HashMap::new();
    for row in all_sidebar_threads {
        threads_by_paths
            .entry(row.folder_paths.clone())
            .or_default()
            .push(row);
    }

    for (ws_index, workspace) in workspaces.iter().enumerate() {
        // ... existing absorbed-workspace logic ...

        let path_list = workspace_path_list(workspace, cx);

        if should_load_threads {
            let mut seen_session_ids: HashSet<acp::SessionId> = HashSet::new();

            // Read from SidebarDb instead of ThreadStore
            if let Some(rows) = threads_by_paths.get(&path_list) {
                for row in rows {
                    seen_session_ids.insert(row.session_id.clone());
                    let (agent, icon) = match &row.agent_name {
                        None => (Agent::NativeAgent, IconName::ZedAgent),
                        Some(name) => (
                            Agent::Custom { name: name.clone().into() },
                            IconName::ZedAgent, // placeholder, resolved in Step 5
                        ),
                    };
                    threads.push(ThreadEntry {
                        agent,
                        session_info: AgentSessionInfo {
                            session_id: row.session_id.clone(),
                            cwd: None,
                            title: Some(row.title.clone()),
                            updated_at: Some(row.updated_at),
                            created_at: row.created_at,
                            meta: None,
                        },
                        icon,
                        icon_from_external_svg: None,
                        status: AgentThreadStatus::default(),
                        workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                        is_live: false,
                        is_background: false,
                        highlight_positions: Vec::new(),
                        worktree_name: None,
                        worktree_highlight_positions: Vec::new(),
                        diff_stats: DiffStats::default(),
                    });
                }
            }

            // ... existing linked git worktree logic, also reading from threads_by_paths ...
            // ... existing live thread overlay logic (unchanged) ...
        }
    }
}
```

### What Changes

- `rebuild_contents` reads from `SIDEBAR_DB` instead of `ThreadStore`.
- The `ThreadEntry.agent` field now carries `Agent::Custom { name }` for ACP threads, enabling correct routing in `activate_thread`.
- The live thread overlay logic (from `all_thread_infos_for_workspace`) is **unchanged** — it still reads from `AgentPanel` to get real-time status of running threads.

### What Stays the Same

- The entire workspace/absorbed-workspace/git-worktree structure.
- The live thread overlay pass.
- The notification tracking logic.
- The search/filter logic.

---

## Step 3: Write Native Thread Metadata to `SidebarDb`

**File:** `crates/agent_ui/src/sidebar.rs` and/or `crates/agent_ui/src/agent_panel.rs`

When a native thread is saved (after conversation, on title update, etc.), we also write its metadata to `SidebarDb`. There are two approaches:

### Option A: Subscribe to `ThreadStore` Changes (Recommended)

Keep a one-directional sync: when `ThreadStore` finishes a `save_thread` or `reload`, the sidebar syncs the metadata to `SidebarDb`. This can be done in the sidebar's workspace subscription or by observing `ThreadStore` changes purely for the purpose of syncing (not for reading).

```rust
// In Sidebar::subscribe_to_workspace or a dedicated sync method:
fn sync_native_threads_to_sidebar_db(&self, cx: &App) {
    if let Some(thread_store) = ThreadStore::try_global(cx) {
        let entries: Vec<_> = thread_store.read(cx).entries().collect();
        cx.background_spawn(async move {
            for meta in entries {
                SIDEBAR_DB.save(&SidebarThreadRow {
                    session_id: meta.id,
                    agent_name: None, // native
                    title: meta.title,
                    updated_at: meta.updated_at,
                    created_at: meta.created_at,
                    folder_paths: meta.folder_paths,
                }).await.log_err();
            }
        }).detach();
    }
}
```

### Option B: Write at the Point of Save

In `AgentPanel` or wherever `thread_store.save_thread()` is called, also call `SIDEBAR_DB.save(...)`. This is more direct but requires touching more call sites.

**Recommendation:** Option A is simpler for the initial implementation. We observe `ThreadStore` changes, diff against `SidebarDb`, and sync. Later, if we want to remove `ThreadStore` entirely from the write path for native threads, we can switch to Option B.

---

## Step 4: Write ACP Thread Metadata to `SidebarDb`

**File:** `crates/agent_ui/src/connection_view.rs` (or `agent_panel.rs`)

When ACP sessions are created, updated, or listed, write metadata directly to `SidebarDb`:

- **On new session creation:** After `connection.new_session()` returns the `AcpThread`, call `SIDEBAR_DB.save(...)`.
- **On title update:** ACP threads receive title updates via `SessionInfoUpdate`. When these come in, call `SIDEBAR_DB.save(...)` with the new title and updated timestamp.
- **On session list refresh:** When `AgentSessionList::list_sessions` returns for an ACP agent, bulk-sync the metadata into `SidebarDb`.

After any write, call `cx.notify()` on the `Sidebar` entity (or use a channel/event) to trigger a `rebuild_contents`.

### Triggering Sidebar Refresh

Since the sidebar no longer observes `ThreadStore`, we need a mechanism to trigger `rebuild_contents` after DB writes. Options:

1. **Emit an event from `AgentPanel`** — The sidebar already subscribes to `AgentPanelEvent`. Add a new variant like `AgentPanelEvent::ThreadMetadataChanged` and emit it after saving to `SidebarDb`.
2. **Use `cx.notify()` directly** — If the save happens within a `Sidebar` method, just call `self.update_entries(cx)`.
3. **Observe a lightweight signal entity** — A simple `Entity<()>` that gets notified after DB writes.

**Recommendation:** Option 1 (emit from `AgentPanel`) is cleanest since the sidebar already subscribes to panel events.

---

## Step 5: Handle Agent Icon Resolution for ACP Threads

**File:** `crates/agent_ui/src/sidebar.rs`

For ACP threads in the sidebar, we need the correct agent icon. The `agent_name` string stored in `SidebarDb` maps to an agent in the `AgentServerStore`, which has icon info.

In `rebuild_contents`, after building the initial thread list from `SidebarDb`, resolve icons for ACP threads:

```rust
// For ACP threads, look up the icon from the agent server store
if let Some(name) = &row.agent_name {
    if let Some(agent_server_store) = /* get from workspace */ {
        // resolve icon from agent_server_store using name
    }
}
```

---

## Step 6: Handle Delete Operations Correctly

**File:** `crates/agent_ui/src/sidebar.rs`

When the user deletes a thread from the sidebar:

- **All threads** → Delete from `SidebarDb` via `SIDEBAR_DB.delete(session_id)`.
- **Native threads** → _Also_ delete from `ThreadStore`/`ThreadsDatabase` (to clean up the blob data).
- **ACP threads** → Optionally notify the ACP server via `AgentSessionList::delete_session`.

The `agent_name` field on `SidebarThreadRow` (or the `Agent` enum on `ThreadEntry`) tells us which path to take.

When the user clears all history:

```rust
// Delete all sidebar metadata
SIDEBAR_DB.delete_all().await?;
// Also clear native thread blobs
thread_store.delete_threads(cx);
// Optionally notify ACP servers
```

---

## Step 7: Handle `activate_thread` Routing

**File:** `crates/agent_ui/src/sidebar.rs`, `crates/agent_ui/src/agent_panel.rs`

In `activate_thread`, branch on the `Agent` variant:

- `Agent::NativeAgent` → Call `panel.load_agent_thread(Agent::NativeAgent, session_id, ...)` (current behavior).
- `Agent::Custom { name }` → Call `panel.load_agent_thread(Agent::Custom { name }, session_id, ...)` so it routes to the correct `AgentConnection::load_session`.

This is already partially set up — `activate_thread` takes an `Agent` parameter. The key change is that `ThreadEntry` now carries the correct `Agent` variant based on `SidebarThreadRow.agent_name`.

---

## Step 8: Handle `activate_archived_thread` Without ThreadStore

**File:** `crates/agent_ui/src/sidebar.rs`

Currently, `activate_archived_thread` looks up `saved_path_list` from `ThreadStore`:

```rust
let saved_path_list = ThreadStore::try_global(cx).and_then(|thread_store| {
    thread_store
        .read(cx)
        .thread_from_session_id(&session_info.session_id)
        .map(|thread| thread.folder_paths.clone())
});
```

Replace this with a targeted `SidebarDb::get` lookup (single-row SELECT, no full table scan):

```rust
let saved_path_list = SIDEBAR_DB
    .get(&session_info.session_id)
    .ok()
    .flatten()
    .map(|row| row.folder_paths);
```

---

## Step 9: Error Handling for Offline Agents

When an ACP thread is clicked but the agent server is not running:

- Show a toast/notification explaining the agent is offline.
- Keep the metadata in the sidebar (don't remove it).
- Optionally offer to start the agent server.

---

## Step 10: Migration — Backfill Existing Native Threads

On first launch after this change, the `SidebarDb` will be empty while `ThreadsDatabase` has existing native threads. We need a one-time backfill:

```rust
// In Sidebar::new or a dedicated init method:
fn backfill_native_threads_if_needed(cx: &App) {
    if SIDEBAR_DB.count()  > 0 {
        return; // Already populated
    }

    if let Some(thread_store) = ThreadStore::try_global(cx) {
        let entries: Vec<_> = thread_store.read(cx).entries().collect();
        cx.background_spawn(async move {
            for meta in entries {
                SIDEBAR_DB.save(&SidebarThreadRow {
                    session_id: meta.id,
                    agent_name: None,
                    title: meta.title,
                    updated_at: meta.updated_at,
                    created_at: meta.created_at,
                    folder_paths: meta.folder_paths,
                }).await.log_err();
            }
        }).detach();
    }
}
```

---

## Summary of Files to Change

| File                                     | Changes                                                                                                                                                                                                                                                        |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/agent_ui/Cargo.toml`             | Add `db.workspace = true`, `sqlez.workspace = true`, `sqlez_macros.workspace = true`, `chrono.workspace = true` dependencies                                                                                                                                   |
| `crates/agent_ui/src/sidebar.rs`         | **Main changes.** Add `SidebarDb` domain + `SIDEBAR_DB` static + `SidebarThreadRow`. Replace all `ThreadStore` reads in `rebuild_contents` with `SidebarDb` reads. Update `activate_archived_thread`. Add native thread sync logic. Add backfill on first run. |
| `crates/agent_ui/src/agent_panel.rs`     | Emit `AgentPanelEvent::ThreadMetadataChanged` after thread saves. Potentially write ACP metadata to `SidebarDb` here.                                                                                                                                          |
| `crates/agent_ui/src/connection_view.rs` | Write ACP metadata to `SidebarDb` on session creation, title updates, and session list refreshes.                                                                                                                                                              |

## What Is NOT Changed

| File / Area                                | Why                                                                                                                          |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `threads` table schema                     | No migration needed — native blob persistence is completely untouched                                                        |
| `ThreadsDatabase` methods                  | `save_thread_sync`, `load_thread`, `list_threads`, `delete_thread`, `delete_threads` — all unchanged                         |
| `ThreadStore` struct/methods               | Stays exactly as-is. It's still used for native thread blob save/load. The sidebar just no longer reads from it for display. |
| `NativeAgent::load_thread` / `open_thread` | These deserialize `DbThread` blobs — completely unaffected                                                                   |
| `crates/acp_thread/`                       | No new persistence module needed there (unlike the original plan)                                                            |
| `crates/agent/src/db.rs`                   | `DbThreadMetadata` is unchanged — no `agent_type` field added                                                                |

## Execution Order

1. **SidebarDb domain** (Step 1) — Create `SidebarDb`, `SidebarThreadRow`, `SIDEBAR_DB` static, CRUD methods in `sidebar.rs`.
2. **Replace reads** (Step 2) — Swap `ThreadStore` reads in `rebuild_contents` for `SidebarDb` reads.
3. **Native write path** (Step 3) — Sync native thread metadata from `ThreadStore` into `SidebarDb`.
4. **ACP write path** (Step 4) — Write ACP thread metadata to `SidebarDb` from connection views.
5. **Icon resolution** (Step 5) — Resolve ACP agent icons in the sidebar.
6. **Delete path** (Step 6) — Route deletes to `SidebarDb` + native blob cleanup + ACP server notification.
7. **Activate routing** (Step 7) — Ensure `activate_thread` routes correctly based on `Agent` variant.
8. **Archive fix** (Step 8) — Update `activate_archived_thread` to use `SidebarDb`.
9. **Migration** (Step 10) — Backfill existing native threads on first run.
10. **Polish** (Step 9) — Error handling for offline agents.

## Key Differences from Original Plan

| Aspect                               | Original Plan                                                                              | Revised Plan                                                                    |
| ------------------------------------ | ------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------- |
| **Where ACP metadata lives**         | New `AcpThreadMetadataDb` in `crates/acp_thread/`                                          | `SidebarDb` in `crates/agent_ui/src/sidebar.rs`                                 |
| **Where sidebar reads from**         | `ThreadStore` (which merges native + ACP)                                                  | `SidebarDb` directly (single source)                                            |
| **ThreadStore changes**              | Added `agent_type` to `DbThreadMetadata`, merge logic in `reload`, new save/delete methods | **None** — ThreadStore is untouched                                             |
| **`crates/agent/src/db.rs` changes** | Added `agent_type: Option<String>` to `DbThreadMetadata`                                   | **None**                                                                        |
| **Merge complexity**                 | Two data sources merged in `ThreadStore::reload`                                           | No merge — one table, one read                                                  |
| **Crate dependencies**               | `acp_thread` gains `db` dependency                                                         | `agent_ui` gains `db` dependency (more natural — it's a UI persistence concern) |
