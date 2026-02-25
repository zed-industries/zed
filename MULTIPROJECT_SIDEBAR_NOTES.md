# Multiproject Sidebar Implementation Notes

## Context

We're implementing a new multiproject sidebar for Zed, based on a design plan from Mikayla. The core idea is replacing the old flat-list sidebar with a grouped view where agent threads are organized by project (root folder).

### Key Concepts from the Plan

- **Workspace** (`struct Workspace`): Layout info for running the app. 1:1 mapping with `Project` structs.
- **Project** (`struct Project`): Data structures powering the app. A folder maps to a `Worktree`.
- **ProjectGroup**: A group of workspaces sharing the same root folder(s). In v0, each group has one workspace. In v0.1 (once Richard's worktree work lands), groups will contain multiple workspaces for different git worktrees.
- **ActiveProjects**: The top-level list of `ProjectGroup`s, rendered by the sidebar. When you open a project, it's automatically added. "Removing" a project is more like hiding — threads stay in the database.

### v0 Scope

`ProjectGroup` → 1 Workspace, no worktrees. We're building the data model to support the full design but only exercising the simple case for now.

### Target UI

The mockup shows a sidebar where each project group contains multiple thread rows. Each row displays:
- Agent icon (Zed Agent, Claude Code, Gemini CLI, OpenCode — each has a distinct icon)
- Thread title (truncated with ellipsis)
- Worktree name (once Richard's worktree feature lands; shows as a branch-like label e.g. "olivetti", "rosewood")
- Diff stats (+N -M lines changed) — future work
- Relative timestamp (e.g. "7:46 PM", "1d", "5d")
- "View More" link at the bottom of groups with many threads

The sidebar is conceptually the **right join** of two datasets:
1. The list of active workspaces (projects currently open in windows)
2. The thread database (all historical threads)

Every workspace shows up (even with no thread yet), and historical threads that aren't currently active also appear.

## Architecture

### Files Involved

- **`crates/sidebar/src/sidebar.rs`** — The main sidebar crate. Contains all the new data structures and the `Picker`-based UI.
- **`crates/agent/src/thread_store.rs`** — `ThreadStore` (global entity) and `DbThreadMetadata`. The sidebar reads thread metadata from here.
- **`crates/agent/src/db.rs`** — `DbThreadMetadata` struct definition and `ThreadsDatabase` queries.
- **`crates/workspace/src/multi_workspace.rs`** — Defines the `Sidebar` trait, `SidebarHandle`, `SidebarEvent`, `MultiWorkspace` struct. These are the primitives the sidebar plugs into. We haven't modified this file.
- **`crates/zed/src/zed.rs`** (L374-382) — Where `Sidebar::new()` is created and `register_sidebar()` is called on every new `MultiWorkspace`.
- **`crates/zed/src/visual_test_runner.rs`** (~L2666) — Visual tests that also create/register the sidebar.

### Data Model (current state)

```
ActiveProjects
  groups: Vec<ProjectGroup>

ProjectGroup
  path_list: PathList               // The root paths that define this group
  entries: Vec<Entity<ProjectEntry>>

ProjectEntry (GPUI Entity)
  workspace: Entity<Workspace>      // Live workspace handle
  session_id: Option<acp::SessionId> // Links to DbThreadMetadata in ThreadStore
```

Thread metadata (title, timestamp, worktree branch) is read **live** from `ThreadStore` via the `session_id` — never cached on `ProjectEntry`. This ensures the sidebar always reflects the latest DB state without stale snapshots.

**How `session_id` is populated**: `workspace_session_id()` reads Workspace → `AgentPanel` → active `AcpThread` → `session_id()`. If the workspace has no active thread, `session_id` is `None` and the entry shows "New Thread".

**How metadata is read**: `ProjectEntry::thread_title(cx)` does `ThreadStore::global(cx).read(cx).thread_from_session_id(&id)` → `DbThreadMetadata.title`. This is a vec scan over in-memory data — no DB hit on render.

**`DbThreadMetadata`** (from `crates/agent/src/db.rs`):
```
DbThreadMetadata
  id: acp::SessionId
  parent_session_id: Option<acp::SessionId>
  title: SharedString
  updated_at: DateTime<Utc>
  worktree_branch: Option<String>    // Denormalized from DbThread for efficient listing
```

**Live-only data** (not in DB, read from thread view at render time):
- `agent_icon` — from `AcpThreadView.agent_icon`, originally from `AgentServer::logo()`. Known mappings: Zed Agent → `ZedAgent`, Claude Code → `AiClaude`, Codex → `AiOpenAi`, Gemini → `AiGemini`, custom → `Terminal`.
- `agent_thread_status` — derived from `AcpThread` state (running turn, errors, confirmations). Enum: `Running`, `Completed`, `WaitingForConfirmation`, `Error`.

**Why `ProjectEntry` is an entity**: We needed a flat `Vec<Entity<ProjectEntry>>` in the delegate for Picker indexing, while `ActiveProjects` maintains the grouped structure. Entity handles are cheap to clone (Arc-like), give stable identity, and avoid self-referential borrow issues. Both the grouped structure and the flat list point to the same underlying data.

**`PathList`** (`crates/workspace/src/path_list.rs`): Zed's canonical type for a set of root paths. Stores paths in lexicographic order so equality comparison is order-independent. Used as the grouping key — two workspaces with the same `PathList` belong in the same `ProjectGroup`.

### Delegate & Picker

The sidebar uses `Picker<ActiveProjectsDelegate>`. The `Picker` provides search input, keyboard nav, scroll, selection highlighting. The delegate provides the domain-specific behavior.

```
ActiveProjectsDelegate
  multi_workspace: Entity<MultiWorkspace>  // For activation/mutation operations
  active_projects: ActiveProjects           // The grouped data model
  flat_entries: Vec<Entity<ProjectEntry>>   // Flat view for Picker indexing
  selected_index: usize
```

- `match_count()` → `flat_entries.len()`
- `render_match(ix)` → reads `flat_entries[ix]`, checks if a group header should be shown by comparing `group_name()` with the previous entry
- `confirm()` → activates the workspace via `multi_workspace.activate(workspace)`
- Group headers are rendered inline with the first entry of each group (no separator entries in the flat list)

### Key Design Decisions

1. **No flat-list separator hack**: The old sidebar (and ~8 other places in the codebase) used `Separator` enum variants in a flat list with `can_select() -> false`. We render headers inside `render_match` by checking if the current entry starts a new group. This avoids index-offset bugs.

2. **DB-backed thread metadata, not snapshots**: Thread titles and timestamps come from `DbThreadMetadata` via `ThreadStore`, read live on each render. This replaces the old approach of snapshotting an `AgentThreadInfo` struct during `update_entries`. The `ThreadStore` holds all metadata in-memory (loaded from SQLite on startup/reload), so reads are just vec scans — no DB I/O on the render path. `preserve_session_ids_from` carries forward old session IDs when a workspace temporarily has no active thread (e.g. mid-switch), ensuring titles don't flicker to "New Thread".

3. **Re-entrancy avoidance**: `Sidebar::new` is called inside an `observe_new` callback on `MultiWorkspace`, which means `MultiWorkspace` is already mutably borrowed. We pass `workspaces: &[Entity<Workspace>]` and `active_workspace: &Entity<Workspace>` as parameters instead of reading through the entity handle. The `update_entries` method uses `cx.defer_in` to safely read `MultiWorkspace` later.

4. **Subscription pattern**: Four subscription sources keep the sidebar reactive:
   - `_subscription` — observes `MultiWorkspace` (workspace added/removed/activated)
   - `_thread_store_subscription` — observes `ThreadStore` (thread metadata changes: title updated, thread deleted)
   - `_project_subscriptions` — `WorktreeAdded`/`Removed`/`OrderChanged` (grouping may change)
   - `_agent_panel_subscriptions` — `AgentPanelEvent` (thread switched)
   - `_thread_subscriptions` — observe active thread entity (status changes for notifications)
   All funnel into `update_entries` which does a full rebuild. The subscription handles are stored in `Vec<Subscription>` fields — reassigning the vec drops old subscriptions (RAII unsubscribe). The `ThreadStore` subscription uses `ThreadStore::try_global(cx)` so it gracefully handles test environments where the store isn't initialized.

### Serialization

Currently minimal in `multi_workspace.rs`: `MultiWorkspaceState` stores `active_workspace_id` and `sidebar_open`, persisted to KVP store keyed by `WindowId`. Will need to be extended for `ProjectGroup` structure (group ordering, collapsed state, hidden groups).

## What's Done

- [x] `ProjectEntry` struct with `group_name()` and `thread_title()` methods
- [x] `ProjectEntry` as a GPUI entity
- [x] `ProjectGroup` with `from_workspace()`, `add_project()`, `contains()`
- [x] `ActiveProjects` with grouping by `PathList`, `iter()`, `preserve_session_ids_from()`
- [x] `ActiveProjectsDelegate` with `flat_entries` for Picker indexing
- [x] Group headers rendered inline (no separator hack)
- [x] `confirm()` activates workspace via `MultiWorkspace::activate()`
- [x] `selected_index` tracking, synced to active workspace
- [x] `update_entries` with full rebuild + subscriptions for reactivity
- [x] All callsites updated (`zed.rs`, `visual_test_runner.rs`, 3 test functions)
- [x] **Migrated from `AgentThreadInfo` to `DbThreadMetadata`**: Thread titles are now sourced from `ThreadStore` (database-backed, in-memory) via `session_id` lookup. Removed `AgentThreadInfo` struct and `workspace_thread_info()` entirely.
- [x] **Added `ThreadStore::try_global(cx)`**: Returns `Option<Entity<Self>>` for graceful access in test environments. Added to `crates/agent/src/thread_store.rs`.
- [x] **`ThreadStore` subscription**: Sidebar observes `ThreadStore` so it refreshes when thread metadata changes (title summarization, deletion, etc.).
- [x] **Live accessors for runtime-only data**: `ProjectEntry::agent_icon(cx)` and `agent_thread_status(cx)` read from the live thread view — ready for use in richer rendering and notification tracking.

## What's Next

### Immediate TODO

- [ ] **Search/filtering** (`update_matches`): Currently a no-op. Need fuzzy matching against `group_name()` and `thread_title()`. The old code used `fuzzy::match_strings` — could reuse that approach but matching against flat_entries.
- [ ] **Richer thread row rendering**: The mockup shows agent icon, diff stats (+21 -12), timestamps, worktree name. Currently just a plain `Label` with the thread title. Should use `ThreadItem` component from `ui` crate. Key fields available now: `thread_title(cx)` for title, `agent_icon(cx)` for icon, `agent_thread_status(cx)` for status, `DbThreadMetadata.updated_at` for timestamp. `ThreadItem` also expects `added`/`removed` diff stats and `worktree` name — those need new data sources.
- [ ] **Remove/hide workspace**: The mockup has an "X" button on each thread row. Need to wire up `multi_workspace.remove_workspace()`. This exists in the old code's `render_match`.

### Soon

- [ ] **Notification tracking**: The old code tracked `notified_workspaces: HashSet<usize>` for background threads that completed. Need to rebuild this using entity identity instead of indices. The `has_notifications` method on `WorkspaceSidebar` trait currently returns `false`. The infrastructure is in place (`agent_thread_status(cx)` provides live status, `test_statuses` HashMap exists for testing), but the actual tracking logic (detecting Running → Completed transitions on background workspaces, clearing on activation) needs to be implemented.
- [ ] **Many threads per project group**: The mockup shows multiple threads per project (not just the active workspace thread). This means the primary data source should be `ThreadStore::entries()` (all threads from DB), grouped by project. Currently we show one entry per workspace. The migration path: iterate `ThreadStore` entries, associate each with a project via worktree path (currently only `worktree_branch` is in `DbThreadMetadata`; may need to denormalize `worktree_path` from `DbThread.git_worktree_info` as well).
- [ ] **Diff stats**: The mockup shows +N -M per thread. Not currently in `DbThreadMetadata` or `DbThread`. Need a new data source — likely aggregated from the thread's tool calls or tracked as a summary stat on save.
- [ ] **Persist agent icon/type in DB**: Currently the agent icon is runtime-only (from `AgentServer::logo()`). For historical threads not currently loaded, we can't show the correct icon. Consider persisting the agent server name or type in `DbThread`/`DbThreadMetadata`.
- [ ] **Serialization of project groups**: Extend `MultiWorkspaceState` to persist group structure, ordering, collapsed/hidden state.
- [ ] **Recent projects integration**: The old sidebar showed recent projects below active workspaces. Need to decide how this fits into the new grouped model.

### v0.1 (after Richard's worktree work)

- [ ] **Multiple workspaces per ProjectGroup**: Currently v0 is 1:1. Need to support multiple git worktrees per project group.
- [ ] **"New Worktree" button**: Shown in the mockup dropdown ("Start Thread in… Current Project / New Worktree").
- [ ] **Worktree-level grouping within a project**: The mockup shows worktree names (e.g. "olivetti", "rosewood") as labels on thread rows. `DbThreadMetadata.worktree_branch` already carries this data for DB threads.

## Codebase Patterns to Know

- **Picker delegate pattern**: Used in ~8+ places across the codebase. `Picker` handles generic list UI, delegate handles domain logic. Common pitfall: flat list with `Separator` enum variants — we're deliberately avoiding this.
- **GPUI entity re-entrancy**: You cannot `entity.read(cx)` while the entity is already being `update`d higher up the call stack. GPUI panics at runtime. Use `cx.defer_in` or pass data as parameters to avoid.
- **`cx.observe_in` / `cx.subscribe_in`**: Return `Subscription` handles. The subscription is active as long as the handle exists. Drop handle = unsubscribe.
- **`PathList`**: Canonical way to identify a workspace by its root paths. Sorted internally for order-independent equality. Has serialization support.
- **`ThreadStore`**: Global GPUI entity (`ThreadStore::global(cx)` / `ThreadStore::try_global(cx)`) backed by SQLite. Holds `Vec<DbThreadMetadata>` in memory, loaded via `reload()`. Notifies observers on changes. Filters out child threads (subagents) — only top-level sessions are surfaced.
- **`AgentServer::logo()`**: Each agent server type has a static icon. Zed Agent → `IconName::ZedAgent`, Claude Code → `AiClaude`, Codex → `AiOpenAi`, Gemini → `AiGemini`, custom → `Terminal`. The icon is set on `AcpThreadView.agent_icon` when a thread view is created and is a runtime-only property.