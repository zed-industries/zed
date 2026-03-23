# Sidebar / Workspace Refactor — Implementation Plan

**Reference spec:** `PARALLEL_AGENTS_SPEC.md` (adjacent)

**What problem does this refactor solve?**

## Current Architecture Summary

Constraints:
- Users should be able to open different worktrees in different windows
- Users should always see worktrees under their associated project (!!!!)
- Opening a new project should not stop a currently running thread
- WRONG: Clicking on a thread, should always open it in your current window if possible (!!!!!)
- Each workspace should only ever show in 1 window

- 👀 = active workspace/thread
- 🏃‍♀️ = Agentic thread is running

STARTING STATE:

Window 1 active project: `zed`
Window 1, threads sidebar:
`zed`
  Thread 1 🏃‍♀️
  Thread 2  {olivetti}

USER: Clicks "open in new window" on the Worktree Thread 2

Window 1 active project: `zed`
Window 1, threads sidebar:
`zed`
  Thread 1 🏃‍♀️
  Thread 2  {olivetti}

Window 2 active project: `zed {olivetti}`
Window 2, threads sidebar:
`zed`
  Thread 1 🏃‍♀️
  Thread 2  {olivetti}

USER: Then changes both windows to focus on `cloud` and `zed.dev` respectively

Window 1 active project: `cloud`
Window 1, threads sidebar:
`cloud`
  Thread 3
`zed`
  Thread 1 🏃‍♀️
  Thread 2  {olivetti}

Window 2 active project: `zed.dev`
Window 2, threads sidebar:
`zed.dev`
  Thread 4
`zed`
  Thread 1 🏃‍♀️
  Thread 2  {olivetti}

So in this state, what happens in each of these cases:
- *What retains thread 1 in the application?*
- *What happens when Window 1 clicks "Thread 1"?*
  - A: Trivial, it should show the workspace associated with thread 1
- *What happens when the user in Window 2 clicks "Thread 1"?*
  - Options: Open window 1 and show that workspace, or show that workspace in window 2?
    - Showing this workspace in Window 2, is really hard, right now.
    - We just declare "nope, it shows up in Window 1"
- *What happens when "Thread 1" finishes and produces a notification?*
  - Options: Show up in your currently focused window? Show up in the window 1?
    - Showing up in your currently focused window feels surprising / destructive
    - So, it should also show up in Window 1
MULTIWORKSPACE WINS AGAIN

These questions are what the architecture below attempts to answer.

*currently*

App
-> Window  1
  -> MultiWorkspace
    -> Workspace (`zed`) 🏃‍♀️
    -> Workspace (`cloud`)  👀
-> Window 2
  -> MultiWorkspace
    -> Workspace (`zed.dev`) 👀
    -> ????

----->

App
-> WorkspaceStore
  -> Workspace (`zed`) 🏃‍♀️
-> Window 1
  -> MultiWorkspace
    -> Workspace (`cloud`)
-> Window 2
  -> MultiWorkspace
    -> Workspace (`zed.dev`)


### How it works today

1. **`MultiWorkspace`** (`crates/workspace/src/multi_workspace.rs`) is the root entity for a window. It owns:
   - `workspaces: Vec<Entity<Workspace>>` — the list of open projects in this window
   - `active_workspace_index: usize` — which one is currently active
   - `sidebar: Option<Box<dyn SidebarHandle>>` — the sidebar UI
   - Serialization via `_serialize_task` that writes `MultiWorkspaceState` (active workspace ID + sidebar open) to KVP keyed by `WindowId`

2. **`WorkspaceStore`** (`crates/workspace/src/workspace.rs:1074`) is a **global** entity on `AppState`. It holds:
   - `workspaces: HashSet<(AnyWindowHandle, WeakEntity<Workspace>)>` — **all** workspaces across **all** windows
   - Currently only used for collab follow/unfollow protocol — **not** used for project management

3. **`Sidebar`** (`crates/sidebar/src/sidebar.rs`) holds:
   - `multi_workspace: WeakEntity<MultiWorkspace>` — it reaches into MultiWorkspace to get the list of workspaces
   - On click/activate, it calls `multi_workspace.activate(workspace)` and `multi_workspace.focus_active_workspace()`
   - `rebuild_contents()` iterates `multi_workspace.workspaces()` to build the thread list per project

4. **Persistence** (`crates/workspace/src/persistence.rs`):
   - `session_workspaces(session_id)` returns all workspaces from last session, including their `window_id`
   - `read_serialized_multi_workspaces()` groups `SessionWorkspace`s by `window_id` into `SerializedMultiWorkspace` groups
   - `restore_multiworkspace()` in `workspace.rs` opens the first workspace's window, then adds each remaining workspace into that same window's MultiWorkspace
   - **Bug**: `MultiWorkspaceState` is keyed by `WindowId` which changes between sessions, so active workspace restoration only works by coincidence

### Key problems with current approach

1. **Persistence assumes multi-workspace world**: `MultiWorkspaceState` is keyed by `WindowId` (ephemeral). On restart, window IDs change, so `read_multi_workspace_state` returns defaults. The "active workspace" restore only works when the serialized `window_id` happens to match.

2. **Eager restoration**: All workspaces from last session are restored on startup, grouped by window. If you had 5 projects open, all 5 get workspaces created immediately. The new model should be: show the project list in the sidebar, but only create/load workspaces on click.

3. **Workspace ownership split**: Workspace `Entity<Workspace>` handles live in `MultiWorkspace.workspaces` (per-window) AND in `WorkspaceStore.workspaces` (global). There's no single source of truth for "which workspaces exist, which are open, which window are they in."

4. **Cross-window coordination is missing**: If project X is open in Window A and you click it in Window B's sidebar, there's no mechanism to detect the conflict and focus Window A instead.

Window 1 active project: `cloud`
Window 1 Thread Sidebar:
`cloud`
  Thread 3
`zed`
  Thread 1 🏃‍♀️
  Thread 2 {olivetti}

Window 2 active project: `zed`
Window 2 Thread Sidebar:
`zed`
  Thread 1 🏃‍♀️
  Thread 2 {olivetti} (<- clicking here focuses Window 3)

Window 3 active project: `zed {olivetti}`
Window 3 Thread Sidebar:
`zed`
  Thread 1 🏃‍♀️         (<- clicking here focuses Window 1)
  Thread 2 {olivetti}

^ What do we need to model this case?
We need a mapping from Items in the sidebar, to their associated windows

---

## Target Architecture

### Core Principle: WorkspaceStore becomes the source of truth

`WorkspaceStore` gets promoted from "collab follow helper" to the **central registry** of all workspaces across all windows. The sidebar and MultiWorkspace query WorkspaceStore instead of maintaining their own lists.

### New mental model

```
WorkspaceStore (global, on AppState)
  Workspace A  ->  Window 1
  Workspace B  ->  Window 1
  Workspace C  ->  Window 2
  Workspace D  ->  (not loaded / sidebar-only entry)
```

The sidebar shows **project entries** (a `PathList` + metadata), not necessarily live workspaces. Clicking a project entry asks WorkspaceStore: "get or create a workspace for this `PathList`." WorkspaceStore either:
- Returns an existing one (possibly in another window -> focus that window)
- Creates a new one in the current window

---

## Work Streams

### Stream 1: Clarify Roles of WorkspaceStore, MultiWorkspace, and Sidebar

**Goal**: Establish clean separation of concerns. WorkspaceStore is the global registry. MultiWorkspace holds the active workspace pointer for its window. Sidebar is the orchestrator that bridges them.

#### Roles

- **WorkspaceStore** (global): Registry of all live workspaces across all windows. Knows which window each workspace is in. Provides query/lookup methods. Does NOT own the "active workspace" concept.
- **MultiWorkspace** (per-window): Holds a pointer to the active workspace for this window. Does NOT need to track all workspaces in the window — just the one that's active. Keeps `workspace: Entity<Workspace>` (singular, not a Vec).
- **Sidebar** (per-window): Owns the project list (the list of `PathList`s shown in this window). When the user clicks a project, the sidebar asks WorkspaceStore to find/create a workspace, then tells MultiWorkspace to activate it.

#### Data flow

```
User clicks project in sidebar
  -> Sidebar asks WorkspaceStore: "find or create workspace for PathList X"
  -> WorkspaceStore returns Entity<Workspace> (existing from another window, or newly created)
  -> Sidebar tells MultiWorkspace: "activate this workspace"
  -> MultiWorkspace sets its active pointer, emits ActiveWorkspaceChanged
```

#### WorkspaceStore Dual-Collection Design

WorkspaceStore holds two separate collections:

1. **Windowed workspaces** (existing): `HashSet<(AnyWindowHandle, WeakEntity<Workspace>)>` — weak refs for workspaces that are actively shown in a window. These stay alive because some window's MultiWorkspace holds the strong ref.
2. **Headless workspaces** (new): `Vec<Entity<Workspace>>` (or similar) — strong refs for workspaces that have been loaded but aren't currently active in any window. This is a "parking lot" for workspaces between activations.

Lifecycle:
1. User has project A active in Window 1. MultiWorkspace holds the strong ref, WorkspaceStore has the weak ref + window handle.
2. User clicks project B in sidebar. Sidebar tells WorkspaceStore: "stash A as headless." WorkspaceStore takes a strong ref of A into the headless list.
3. Sidebar tells MultiWorkspace: "activate B." MultiWorkspace drops A's strong ref, but A stays alive in WorkspaceStore's headless list.
4. Later, user clicks project A again. `find_workspace_for_paths()` finds it in the headless list, hands it back. Sidebar tells MultiWorkspace to activate it. WorkspaceStore removes it from headless (the windowed weak ref re-appears naturally since MultiWorkspace now holds it).

Lookup via `find_workspace_for_paths()` searches both collections:
- Found in windowed set -> it's active in a specific window, focus that window
- Found in headless set -> it's loaded but idle, activate it in the requesting window
- Found in neither -> create a new one

#### Tasks

1. **Expand `WorkspaceStore`**:
   - Add `headless_workspaces: Vec<Entity<Workspace>>` field for loaded-but-not-shown workspaces
   - Add `find_workspace_for_paths(path_list) -> Option<FindResult>` that searches both windowed (weak) and headless (strong) collections. `FindResult` distinguishes: `Windowed(AnyWindowHandle, WeakEntity<Workspace>)` vs `Headless(Entity<Workspace>)`
   - Add `stash_as_headless(workspace: Entity<Workspace>)` to park a workspace when it's deactivated
   - Add `take_headless(path_list) -> Option<Entity<Workspace>>` to reclaim a headless workspace for activation
   - WorkspaceStore already registers windowed workspaces via `Workspace::new()` (line 1564) and deregisters on release (line 1671). No changes needed there.

2. **Simplify `MultiWorkspace`**:
   - Replace `workspaces: Vec<Entity<Workspace>>` + `active_workspace_index: usize` with just `workspace: Entity<Workspace>` (the active one)
   - `MultiWorkspace::activate()` just swaps the pointer
   - Remove `add_workspace()`, `remove_workspace()`, `activate_index()`, `activate_next/previous_workspace()` — these become sidebar concerns
   - Keep: sidebar management, serialization, window lifecycle

3. **Move project list to Sidebar**:
   - Sidebar owns a `Vec<PathList>` (or similar) representing the projects shown in this window
   - This list is populated from persistence on startup and updated as projects are added/removed
   - Everything that currently queries `multi_workspace.workspaces()` to see all projects should query the sidebar's list instead



#### Migrating `multi_workspace.workspaces()` callers

Most callers of `multi_workspace.workspaces()` are "do X to all workspaces" queries. These should move to WorkspaceStore (which already tracks all workspaces globally). MultiWorkspace only needs `.workspace()` (the active one).

Callers to migrate to WorkspaceStore:
- `vim/state.rs` (line 746): register keybindings on all workspaces
- `notifications.rs` (line 1054, 1080): show/dismiss notification on all workspaces
- `workspace.rs` `workspace_windows_for_location` (line 8675): find workspaces matching a location
- `workspace.rs` `find_existing_workspace` (line 8715): find best matching workspace for paths
- `workspace.rs` `join_in_room_project` (line 9349): find workspace with matching remote project
- `settings_ui` `all_projects` (line 3744): already chains WorkspaceStore AND MultiWorkspace (simplify to just WorkspaceStore)

Callers that stay on MultiWorkspace:
- `multi_workspace.workspace()` — used everywhere for "give me the active workspace", stays as-is

#### Current code to change
- `WorkspaceStore` struct (workspace.rs line 1074-1078): Add `headless_workspaces` field, new query/stash/take methods
- `MultiWorkspace` struct (multi_workspace.rs line 100-110): `workspaces: Vec` -> `workspace: Entity`
- `MultiWorkspace::add_workspace()` (line 320-335): Remove, sidebar handles project list
- `MultiWorkspace::remove_workspace()` (line 604-635): Remove, sidebar handles project list
- `MultiWorkspace::activate_index/next/previous()` (line 337-368): Remove
- Sidebar: `rebuild_contents()` (sidebar.rs line 464): Stop iterating `mw.workspaces()`, use own project list
- Sidebar: `activate_workspace()` (sidebar.rs line 1228): Query WorkspaceStore, stash old workspace as headless, then set MultiWorkspace pointer
- Migrate all `multi_workspace.workspaces()` callers listed above to WorkspaceStore

### Stream 2: Lazy Workspace Loading (Click-to-Load)

**Goal**: The sidebar shows project entries derived from persistence/recent projects, but workspaces are only created when you click on a project header.

#### Tasks

1. **Separate "project entry" from "live workspace"**:
   - `ListEntry::ProjectHeader` currently holds `workspace: Entity<Workspace>`. Change to hold `PathList` + optional `WeakEntity<Workspace>`
   - The pattern `ThreadEntryWorkspace::Open(Entity<Workspace>)` vs `ThreadEntryWorkspace::Closed(PathList)` already exists — extend to project headers

2. **Sidebar initialization from persistence**:
   - On window open, read the `PathList`s for this window from persistence
   - Don't create `Entity<Workspace>` for each — just show them as entries
   - Only the "active" workspace gets a real `Entity<Workspace>`

3. **Click handler changes**:
   - Clicking a project header calls `WorkspaceStore.get_or_create_workspace(path_list, window)`
   - Creates workspace if needed, activates it, sidebar updates
   - If already open in another window, focus that window

4. **Thread click for non-loaded projects**:
   - `open_workspace_and_activate_thread()` already handles the "open then activate" flow
   - Wire this through WorkspaceStore

#### Current code flow to change
- `Sidebar::rebuild_contents()` (line 464-895) iterates `mw.workspaces()` — needs to also handle unloaded PathList entries
- `Sidebar::activate_workspace()` (line 1228-1247) calls `multi_workspace.activate()` — needs to handle lazy creation
- `ListEntry::ProjectHeader` (line 108-114) holds `workspace: Entity<Workspace>` — needs to become optional

### Stream 3: Persistence Overhaul

**Goal**: Persistence stores per-window project lists as `PathList`s rather than relying on workspace DB rows grouped by `WindowId`.

#### Tasks

1. **New persistence model — "WindowProjectList"**:
   - Store a list of `PathList`s per window in KVP (or new table)
   - Replaces grouping `session_workspaces` by `window_id`
   - Include: which project is active, sidebar open state, ordering

2. **Remove `MultiWorkspaceState` from KVP**:
   - Currently `write_multi_workspace_state(window_id, state)` persists `active_workspace_id` + `sidebar_open` keyed by `WindowId`
   - Replace with WindowProjectList

3. **Restore flow**:
   - `restore_or_create_workspace` reads new WindowProjectList
   - For each window: create window, populate sidebar's project list from stored PathLists
   - Only create/load `Workspace` for the one marked "active"
   - Others appear as unloaded sidebar entries

4. **Save flow**:
   - On project add/remove/reorder/activate: serialize current window's project list
   - Debounce as currently done

#### Current code flow to change
- `read_serialized_multi_workspaces()` (persistence.rs line 315-349) groups by window_id
- `restore_multiworkspace()` (workspace.rs line 8210-8310) creates all workspaces eagerly
- `MultiWorkspace::serialize()` (multi_workspace.rs line 370-379) writes MultiWorkspaceState
- `restorable_workspaces()` (main.rs line 1480) orchestrates restore

### Stream 4: Cross-Window Coordination

**Goal**: Clicking a project already open in another window focuses that window instead of duplicating.

#### Tasks

1. **WorkspaceStore lookup**: `find_workspace_for_paths(path_list)` checks all registered workspaces
2. **Sidebar focus-or-load**: On project click, check WorkspaceStore. If in current window -> activate. If other window -> focus it. If nowhere -> create.
3. **CLI behavior**: `zed my-project/` checks WorkspaceStore before opening new window.

#### Current code providing this hook
- `WorkspaceStore.workspaces` already has `(AnyWindowHandle, WeakEntity<Workspace>)` tuples
- `WorkspaceStore::workspaces_with_windows()` (workspace.rs line 8130) returns these

### Stream 5: Folder Add/Remove Behavior ("Splitting")

**Goal**: When folders change in the active project, the sidebar creates a new project entry preserving old threads.

#### Tasks

1. **Detect folder changes**: Already subscribed to `ProjectEvent::WorktreeAdded/Removed` in `subscribe_to_workspace()`
2. **Snapshot and split**: On folder change, snapshot old PathList, compute new. If different, add new entry to sidebar.
3. **Thread preservation**: Threads keyed by PathList in ThreadMetadataStore. Old entry keeps its threads, new entry starts fresh.

---

## Suggested Ordering

| Phase | Stream | What | Why first |
|-------|--------|------|-----------|
| 1     | S1     | Promote WorkspaceStore | Foundation for everything else |
| 2     | S3     | Persistence overhaul | Needed before lazy loading works |
| 3     | S2     | Lazy workspace loading | Core UX improvement |
| 4     | S4     | Cross-window coordination | Correctness / conflict resolution |
| 5     | S5     | Folder splitting behavior | Polish / spec compliance |

---

## Key Files to Modify

| File | Changes |
|------|---------|
| `crates/workspace/src/workspace.rs` (WorkspaceStore) | Add fields, methods, events for workspace registry |
| `crates/workspace/src/multi_workspace.rs` | Thin out to delegate to WorkspaceStore; remove `workspaces: Vec` |
| `crates/sidebar/src/sidebar.rs` | Query WorkspaceStore; support unloaded project entries |
| `crates/workspace/src/persistence.rs` | New WindowProjectList model; remove MultiWorkspaceState KVP |
| `crates/workspace/src/persistence/model.rs` | New serialization types |
| `crates/zed/src/main.rs` | Update `restore_or_create_workspace` |
| `crates/zed/src/zed.rs` | Update `open_paths` / CLI handling |

---

## Open Questions

1. **Tear-down timing**: When should a workspace be destroyed? Options: never (until window closes), when removed from sidebar, or LRU eviction. The spec says removing a project "will stop running threads," implying the workspace should be torn down. Deferred for now.

2. **Thread metadata indexing**: Threads are associated by `PathList`. The project-list-per-window model doesn't change this, but we should verify ThreadMetadataStore doesn't need changes.

3. **Workspace ID allocation**: Currently `create_workspace()` eagerly allocates a DB row via `DB.next_id()`. With lazy loading, when does the DB row get created? Probably on first workspace creation (click), not on sidebar entry creation.

## Resolved Questions

- **Empty window state**: An empty window is just a workspace with no worktrees. No special "no active workspace" state needed — `MultiWorkspace` already starts with one workspace, it just has nothing in it.

- **Migration path**: We're mostly *removing* complexity (fewer stores, fewer workspaces), not adding new data. The existing persistence data is ours to change. We keep most of the same data but need less of it, so the migration is straightforward — essentially just stop writing the redundant multi-workspace grouping data.
