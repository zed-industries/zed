# Active Projects Architecture

## Core Narrative

The sidebar is evolving from a window-level workspace switcher into a **global command center** for active projects. It is the primary interface for understanding "what am I working on right now?" — similar in spirit to the Codex app or Cursor's agents mode, but designed to work seamlessly across single and multi-window setups.

There are two intertwined narratives driving this:

1. **Active Projects** — The sidebar shows everything you're currently thinking about and worrying about. It's not a file browser or a project history. It's a live, curated view of your in-flight work. Projects appear here because you're actively working on them, and they stay until you explicitly dismiss them.

2. **Window Management** — The sidebar also serves as the way you navigate between projects across windows. It shows which projects are currently displayed in which windows, and lets you open, switch, and close them fluidly. But this is secondary to the "active projects" narrative — window management is a consequence of the active projects model, not the other way around.

## The Key Insight: Active Projects ⊇ Open Windows

The set of active projects is a **superset** of currently open windows. If you have three windows open but five projects with threads, all five show in the sidebar. Three of them happen to have windows; two don't. When you click one of the un-windowed projects, it opens (or takes over a window). When you close a window, the project doesn't disappear — it stays in the active projects list, just without a window.

This is the fundamental shift from the current model where the sidebar shows "workspaces in this window" to showing "everything I'm working on, globally."

## Two Data Sources

The sidebar merges two datasets:

### 1. Live Workspace Data
Real-time, reactive state from `Entity<Workspace>`. Folder renames show up instantly. Worktree additions and removals are reflected immediately. This is the source of truth for "what folders define this project" and "is this project currently loaded."

### 2. Thread History
Persistent data from the threads database. This enriches each project with its thread history — titles, diff stats, timestamps, agent identifiers, run status. A project with zero threads is completely valid; it just shows the project header without any thread children.

The active projects list is the **live data structure**. The threads database **enhances** it. The sidebar renders the merge of both.

## Data Model

### The Stored State Is Flat

The underlying data is two independent flat lists. No grouping, no hierarchy — just workspaces and threads.

#### Active Workspaces

A flat list of live workspaces the user is currently working on. Every entry is a live `Entity<Workspace>` — no lazy loading, no deferred state. At startup, all persisted entries are eagerly rehydrated into live workspaces.

```rust
/// The active list is just a Vec of live workspaces. That's it.
/// Every entry is fully loaded with worktrees, language servers, etc.
active_workspaces: Vec<Entity<Workspace>>
```

Each workspace is the live source of truth for its own folder paths. You can always ask it "what are your current worktrees?" and get the real answer — no stale caches, no consistency problems.

#### Threads

A flat list of thread metadata from the thread database. Each thread knows which folder paths it was created against. This is the existing `DbThreadMetadata` infrastructure, potentially extended with diff stats and agent identity.

```rust
/// Already exists in crates/agent/src/db.rs — may need extension.
struct DbThreadMetadata {
    id: acp::SessionId,
    parent_session_id: Option<acp::SessionId>,
    title: SharedString,
    updated_at: DateTime<Utc>,
    worktree_branch: Option<String>,
    // Potential extensions for the new sidebar:
    // agent_name: Option<SharedString>,
    // lines_added: Option<i32>,
    // lines_removed: Option<i32>,
}
```

### Project Groups Are Derived Data

**Project groups are not stored.** They are computed from the two flat lists every time the sidebar needs to render. This is a pure function:

```rust
fn derive_project_groups(
    workspaces: &[Entity<Workspace>],
    threads: &[DbThreadMetadata],
    cx: &App,
) -> Vec<ProjectGroup>;
```

The derivation works as follows:

1. For each workspace, compute a `PathList` from its current visible worktree paths.
2. For each thread, look at which folder paths it was created against.
3. Group workspaces and threads that share the same `PathList`.
4. Produce `ProjectGroup` values for the sidebar to render.

```rust
/// Computed at render time. Never stored.
struct ProjectGroup {
    /// The PathList that defines this group. Computed from the workspaces
    /// within it — never stored as an identity.
    path_list: PathList,
    /// Display name derived from folder names (e.g., "zed" or "ex, zed").
    display_name: SharedString,
    /// The entries in this group, ready for rendering.
    entries: Vec<ThreadEntry>,
}

/// A single entry in a project group, ready for rendering.
struct ThreadEntry {
    workspace: Entity<Workspace>,
    thread_info: Option<SidebarThreadInfo>,
    /// The window currently displaying this workspace, if any.
    /// Only 0 or 1 windows can display a given workspace.
    window: Option<WindowId>,
    /// Git worktree annotation (e.g., "in olivetti") if applicable.
    worktree_annotation: Option<SharedString>,
}
```

#### Why Derived?

Because the underlying data mutates out from under us. A user renames a folder, adds a worktree, changes a project — and the grouping should just reflect reality. If project groups were stored, we'd have to detect every mutation and re-file things. With derivation, we just re-derive and the UI updates.

**Concrete example:** You have a git worktree "olivetti" of the "zed" repo, showing as its own workspace under the "zed" group. You click "Add Folder" and add "cloud" to that workspace. The workspace's `PathList` changes from `{zed}` to `{cloud, zed}`. Next derivation: the workspace moves to the "cloud, zed" group, with an annotation "zed is in olivetti." No bookkeeping, no re-filing — the derivation function just sees the new state.

### PathList As Grouping Key

`PathList` (from `crates/workspace/src/path_list.rs`) is the **derived grouping key**. It's a sorted set of folder paths computed by asking a workspace for its current worktree paths. `PathList` already handles lexicographic sorting for comparison, preserves original order, and has serialization/deserialization support.

```rust
/// Already exists in crates/workspace/src/path_list.rs.
/// Stores paths in lexicographic order for equality comparison,
/// with original insertion order preserved separately.
#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct PathList {
    paths: Arc<[PathBuf]>,
    order: Arc<[usize]>,
}
```

To compute a `PathList` from a workspace:

```rust
fn path_list_from_workspace(workspace: &Workspace, cx: &App) -> PathList {
    let paths: Vec<PathBuf> = workspace
        .worktrees(cx)
        .filter(|wt| wt.read(cx).is_visible())
        .map(|wt| wt.read(cx).abs_path().to_path_buf())
        .collect();
    PathList::new(&paths)
}
```

**Progressive enhancement with git:** Without git, the `PathList` is literally the set of folder paths. With git, worktrees at different physical locations can be recognized as the same logical project and grouped together. This is a refinement on top of the base concept, not a requirement.

**Open question:** The exact canonicalization logic for git worktrees. A worktree at `/tmp/zed-worktree-fix-123` and the main repo at `/home/user/zed` need to resolve to the same `PathList`. This likely means "the path to the main git directory" is the canonical identity, with worktree paths as variants. There's existing infrastructure for this — `AgentGitWorktreeInfo` already tracks the worktree path and branch, and the thread DB stores `worktree_branch` for listing convenience.

### SidebarThreadInfo (Display Data)

The metadata the sidebar displays for each thread. Assembled during derivation from thread database records and live thread state.

```rust
/// Assembled during project group derivation for display purposes.
struct SidebarThreadInfo {
    thread_id: acp::SessionId,
    title: SharedString,
    icon: IconName,
    agent_name: Option<SharedString>,
    status: AgentThreadStatus,
    lines_added: Option<i32>,
    lines_removed: Option<i32>,
    updated_at: DateTime<Utc>,
}
```

This is significantly richer than the current `AgentThreadInfo` which only has title, status, and icon. The additional fields (`agent_name`, `lines_added`, `lines_removed`, `updated_at`) come from the thread database and the live `Thread` entity.

## Three Data Sources, One Sidebar

The sidebar composes three independent data sources into a grouped UI. They have separate responsibilities and separate persistence.

### 1. Active Projects List (Persisted)

An app-global, persisted list of projects the user is actively working on. This is its own data — not derived from recent projects, not derived from thread history. It has its own spot in the database.

```rust
/// App-global persisted list. Its own database table/file.
/// This is an append-mostly list — entries are added automatically
/// when a thread is created, and removed only by explicit user action.
/// At startup, every entry is eagerly rehydrated into a live Entity<Workspace>.
struct ActiveProjectsList {
    /// Every entry is a live workspace. Rehydrated eagerly at startup.
    entries: Vec<Entity<Workspace>>,
}
```

**Entry condition:** A project joins this list when the user creates a thread in it. Having an open workspace is not enough — the thread is the "save" trigger. This is the moment the project becomes durable.

**Persistence:** The list is serialized as folder paths + workspace database IDs. On startup, each entry is deserialized back into a live `Entity<Workspace>` — worktrees are opened, language servers start, file watchers attach. The serialized form is an implementation detail of persistence, not part of the runtime data model.

**The list is append-mostly:** Once a project is in the list, it stays. No automatic removal based on time, inactivity, or window closing.

**Manual removal only:** The user explicitly clicks "Remove Project" to take something off the list. That is the only removal mechanism.

### 2. Live Window State (Ephemeral)

The set of currently open windows and which workspace each one is displaying. This is runtime state — not persisted to the active projects list.

```rust
/// Runtime-only. Tracks which windows exist and what they're showing.
struct WindowState {
    /// Which workspace each window is currently displaying.
    assignments: HashMap<WindowId, Entity<Workspace>>,
}
```

A workspace that is open in a window but has no threads appears in the sidebar while the window is open — it's live, ephemeral data. When the window closes, it disappears from the sidebar entirely because it was never persisted to the active projects list.

### 3. Thread Database (Thread History)

The existing `ThreadStore` / agent DB infrastructure. The source of truth for thread metadata — titles, timestamps, diff stats, which folder paths a thread was running against.

The sidebar queries this during derivation to enrich project entries with their thread lists. The thread database knows nothing about windows, active/inactive state, or project groups.

Key query the sidebar needs:

```rust
/// Query threads whose folder paths match. Returns thread metadata sorted by recency.
fn threads_for_paths(paths: &[PathBuf]) -> Vec<DbThreadMetadata>;
```

This builds on the existing `DbThreadMetadata` which already has `id`, `title`, `updated_at`, and `worktree_branch`. It may need to be extended with diff stats and agent identity.

### 4. How The Sidebar Composes All Three

The sidebar's visible list is the **union** of two sets:

1. **The persisted active projects list** — durable entries, always shown regardless of window state.
2. **Currently open windows not yet in the active list** — ephemeral entries that disappear on window close.

The sidebar observes all three data sources and derives the grouped view:

1. Read the persisted active projects list.
2. Read the live window state to find any workspaces not yet in the active list.
3. Merge both sets into a single flat workspace list.
4. Query the thread database for threads matching each workspace's folder paths.
5. For loaded workspaces, also check live `Entity<Thread>` for real-time status.
6. Call `derive_project_groups()` to group everything by folder set.
7. Render the derived groups with window indicators showing which entries currently have a window looking at them.

**The thread-creation trigger:** When the user creates a thread in an ephemeral workspace (one that's only in the sidebar because its window is open), the sidebar persists that workspace to the active projects list. From that point on, it survives window close.

This keeps all three systems decoupled. The active projects list doesn't know about threads or windows. The thread database doesn't know about windows or active/inactive state. The window state doesn't know about persistence. The sidebar is the composition and derivation layer.

### WorkspaceStore (Coordination Layer)

The existing `WorkspaceStore` (in `crates/workspace/src/workspace.rs`) is already an app-global entity that tracks all workspaces with their window handles. It's currently used for collaboration/follow features. We extend it to also own the persisted active projects list and provide the unified API the sidebar needs.

```rust
/// Already exists in workspace crate. Extended for active projects.
pub struct WorkspaceStore {
    /// Existing: all workspaces with their window handles (used for collab/follow).
    workspaces: HashSet<(AnyWindowHandle, WeakEntity<Workspace>)>,
    /// NEW: the persisted active projects list. Workspaces that have had
    /// at least one thread created in them. Eagerly rehydrated at startup.
    active_projects: Vec<Entity<Workspace>>,
    client: Arc<Client>,
    _subscriptions: Vec<client::Subscription>,
}
```

#### Key Queries (new)

```rust
impl WorkspaceStore {
    /// Existing: all workspaces with their window handles.
    fn workspaces_with_windows(&self) -> impl Iterator<Item = (AnyWindowHandle, &WeakEntity<Workspace>)>;

    /// NEW: the persisted active projects (workspaces with threads).
    fn active_projects(&self) -> &[Entity<Workspace>];

    /// NEW: which windows are currently showing a workspace with this PathList.
    fn windows_for_path_list(&self, path_list: &PathList, cx: &App) -> Vec<WindowId>;
}
```

#### Key Mutations (new)

```rust
impl WorkspaceStore {
    /// NEW: a thread was created in a workspace. Persist it to the active list.
    fn persist_project(&mut self, workspace: &Entity<Workspace>, cx: &mut App);

    /// NEW: user clicked "Remove Project." Remove from the active list.
    fn remove_project(&mut self, index: usize, cx: &mut App);

    /// NEW: a window closed. The project stays in the active list if persisted;
    /// disappears from the sidebar if it was only ephemeral.
    fn detach_window(&mut self, window_id: WindowId, cx: &mut App);
}
```

Note: window assignment tracking already exists via `workspaces: HashSet<(AnyWindowHandle, WeakEntity<Workspace>)>`. We don't need a separate `window_assignments` map — the existing set already associates windows with workspaces. The new `active_projects` field is the only addition to stored state.

## What Happens To MultiWorkspace

MultiWorkspace doesn't get deleted. It gets **hollowed out**.

### What It Keeps

- **Window layout** — rendering the sidebar alongside the workspace, the resize handle, sidebar open/close state
- **Sidebar host** — the `SidebarHandle` trait, sidebar registration, focus management
- **Action routing** — keyboard shortcuts for toggling sidebar, focus switching
- **Window shell** — client-side decorations, the top-level element composition

### What It Loses

- `Vec<Entity<Workspace>>` — moves to `WorkspaceStore.active_projects`
- `active_workspace_index` — `WorkspaceStore` tracks which project is in which window
- `create_workspace` / `remove_workspace` — `WorkspaceStore` handles lifecycle
- `activate_index` / `activate_next` / `activate_previous` — navigation goes through `WorkspaceStore`

### What It Becomes

Essentially a **WindowRoot** — a thin component that holds one workspace, overlays the sidebar, and handles window-level layout. It might even be renamed to reflect this.

```
WindowRoot (née MultiWorkspace)
  window_id: WindowId
  workspace: Entity<Workspace>         — the single currently-displayed workspace
  sidebar: Option<Box<dyn SidebarHandle>>
  sidebar_open: bool
```

The key conceptual shift: MultiWorkspace today *owns* workspaces. WindowRoot *borrows* whichever workspace `WorkspaceStore` tells it to display.

## Sidebar Rendering (unchanged from original)

### Visual Structure

```
┌─────────────────────────────┐
│ [Close]  Threads  [+ New]   │  ← Title bar with close button and new thread button
│ [Search...]                 │  ← Search/filter
├─────────────────────────────┤
│ ex                          │  ← ProjectGroup header (folder set display name)
│   ⚙ Block decoration hei…  │  ← Thread entry (icon, title)
│     ♦ olivetti · 7:46 PM   │  ←   (agent name, timestamp)
│   ⚙ Hit testing returns…   │
│     ♦ rosewood · +21 -12   │
│   ⚙ Add a new tool block…  │
│     ♦ rosewood · +8 -3     │
│   + View More               │  ← Collapsed overflow
├─────────────────────────────┤
│ ex, zed                     │  ← Another project group
│   ⚙ Add soft wrap suppor…  │
│   ⚙ Thread view needs to…  │
├─────────────────────────────┤
│ zed                    🖥   │  ← Window icon = this project has a window
│   ⚙ Implement subagents…   │
│   ⚙ Project search over…   │
│   + View More               │
└─────────────────────────────┘
```

### Context Menu (on Project Group header)

- **Remove Project** — removes from active projects list
- **Collapse Project** — toggles collapsed state for the group
- **New Thread** — starts a new thread in this project
- **New Thread in...** submenu:
  - **Current Project** — new thread reusing an existing workspace
  - **New Worktree** — new thread in a new git worktree

### Thread Entry Interactions

- **Click** — activates this thread's workspace in the current window
- **Hover** — shows full path tooltip
- **Running indicator** — animated dot for actively generating threads
- **Notification dot** — thread completed while you weren't looking

## Lifecycle

### What the sidebar shows

The sidebar shows the **union** of:
- The persisted active projects list (projects that have had threads)
- Any currently windowed workspaces not yet in that list (ephemeral)

Each entry has a **window indicator** showing whether a window is currently looking at it.

### When does a project enter the active list?

A project is persisted to the active list when the user **creates a thread** in it. This is the durable "save" moment. Before that, the project only appears in the sidebar because it has an open window — it's ephemeral.

### When does a project leave the active list?

**Manual removal only.** The user clicks "Remove Project" (the X). That's it. No automatic removal based on time, inactivity, or window closing. The list is append-mostly.

### What happens when you open a window?

The workspace appears in the sidebar immediately (as an ephemeral entry if it has no threads, or it matches an existing active entry if it does). The window indicator lights up.

### What happens when you close a window?

- If the project is in the persisted active list → it stays in the sidebar, just without the window indicator. Still visible, still accessible.
- If the project was only ephemeral (open window, no threads) → it disappears from the sidebar. Nothing was ever persisted.

### What happens when you click an inactive project in the sidebar?

A workspace is created for it (using the stored folder paths), associated with the current window, and displayed. The window indicator lights up.

### What happens with multiple windows?

Each window has its own WindowRoot. The sidebar in each window shows the **same** global active projects list. A project can be displayed in multiple windows simultaneously. The window indicator shows all windows currently viewing that project.

## What The Six Weeks of MultiWorkspace Work Actually Bought

The critical achievement was **breaking the window-workspace coupling**. Before that work, `Window == Workspace` was a hard assumption baked throughout the codebase. That decoupling is the foundation this new architecture stands on. Without it, none of this would be possible.

What changes is *where the multiplexing lives* — it moves from inside the window to outside it, at the global level. The decoupling itself was the right work. The specific topology of "N workspaces inside one window" was an exploration that informed the better topology of "N projects globally, windows as viewports."

The sidebar infrastructure (SidebarHandle, resize, focus management), the serialization plumbing (workspace IDs, session bindings), and the deep understanding of workspace lifecycle — all of that carries forward directly.

## Addendum: Future Optimization — Lazy Loading

The V1 data model is intentionally simple: every active project is a live `Entity<Workspace>`, eagerly rehydrated at startup. This means N workspaces = N sets of language servers, file watchers, worktrees, etc.

This will work fine when the active list is small (single digits to low tens). As the list grows, startup cost and memory usage will become concerns. The optimization path is:

### WorkspaceEntry Enum

Replace `Vec<Entity<Workspace>>` with `Vec<WorkspaceEntry>`:

```rust
enum WorkspaceEntry {
    /// Fully loaded — worktrees, language servers, file watchers all running.
    Loaded(Entity<Workspace>),
    /// Just metadata — folder paths and database ID. No live state.
    Unloaded {
        folder_paths: Vec<PathBuf>,
        workspace_id: Option<WorkspaceId>,
    },
}
```

### Loading Strategy

- Rehydrate only the N most recent projects eagerly (e.g., the last 5 used).
- Keep the rest as `Unloaded` entries with just paths.
- When the user clicks an unloaded entry, create the workspace on demand.
- Optionally pre-warm workspaces in the background after startup settles.

### Display Implications

Unloaded entries can't show live worktree data (folder renames won't be detected). The sidebar would show the last-known folder names from persistence. This is acceptable — the names only go stale if the user renames a folder while Zed isn't watching it, which is rare.

### When To Do This

Not now. The architecture supports it cleanly because the derivation function and the sidebar don't care whether a workspace is live or not — they just need folder paths and thread data. The `WorkspaceEntry` enum can be introduced later without changing the sidebar, the thread database, or the derivation logic. It's a contained optimization at the storage layer.

## Addendum: Path Staleness and Thread DB Reconciliation

With the eager loading model, live workspaces detect folder renames through normal file watching — if you rename a folder, the worktree picks it up and the sidebar derivation reflects the new name. No staleness problem for loaded workspaces.

But there's a second-order issue: the thread database stores folder paths at thread-creation time. If a folder is renamed, existing threads in the DB still reference the old path. This means:

- The thread was created against `{zed}`, the folder gets renamed to `{zed2}`.
- The workspace sees `{zed2}` (live data, file watcher caught it) — its `PathList` changes.
- The thread DB still says `{zed}`.
- The derivation sees the workspace under "zed2" and the thread under "zed" — their `PathList`s don't match, so they don't group together.

This is not optional — it's an essential part of the active projects work. We don't currently store folder-path associations in the thread database in a way that needs rewriting, but we will once the sidebar is grouping threads by folder set. At that point, path reconciliation becomes necessary for correctness, not just polish.

The fix is straightforward: when a workspace detects a path change, it updates the thread DB to reconcile old paths with new ones. The live workspace knows both the old and new paths, so it can issue the update. This keeps the thread DB consistent with reality.

Folder renames are an edge case, so this doesn't need to be in the very first implementation phase. But it should be part of the active projects work — not deferred indefinitely. It can be its own phase or cut out as a follow-up, but it needs to land as part of this overall effort.

## Lifecycle Scenarios

A comprehensive walkthrough of every interaction and what happens to the three data sources and the sidebar.

Legend:
- **Active List** = the persisted active projects list
- **Window State** = the live window assignments
- **Sidebar** = what the user sees

---

### Window Operations

#### Opening a new empty window

> User launches Zed, or hits Cmd+N for a new window.

- A new `Workspace` is created with no worktrees.
- **Window State:** new entry `WindowId → Entity<Workspace>`.
- **Active List:** unchanged. No thread exists, nothing to persist.
- **Sidebar:** shows an ephemeral entry (e.g., "Empty Workspace") with a window indicator. If the user closes this window without ever creating a thread, the entry vanishes.

#### Opening a project from the file system (File > Open, CLI, drag-and-drop)

> User opens `/home/user/zed` in a new or existing window.

- A `Workspace` is created (or reused) with the folder as a worktree.
- **Window State:** `WindowId → Entity<Workspace>` with the zed folder.
- **Active List:** unchanged — no thread yet.
- **Sidebar:** shows "zed" as an ephemeral entry with a window indicator. If "zed" already exists in the active list (from a previous thread), the existing entry lights up with a window indicator instead.

#### Opening a recent project from the sidebar

> User clicks a persisted project "zed" in the sidebar that has no window.

- A new `Workspace` is created from the stored folder paths, attached to the current window.
- **Window State:** current window's assignment changes to the new workspace.
- **Active List:** unchanged — the entry already exists.
- **Sidebar:** the "zed" entry now shows a window indicator. If the window was previously showing another project, that project's window indicator goes away (but it stays in the sidebar if it's persisted).

#### Closing a window (persisted project)

> User closes a window that's showing "zed," which has threads and is in the active list.

- The workspace may be kept alive (if threads are still running) or dropped.
- **Window State:** the `WindowId` entry is removed.
- **Active List:** unchanged. "zed" stays in the list.
- **Sidebar:** "zed" remains visible but loses its window indicator. The user can click it later to reopen it.

#### Closing a window (ephemeral project, no threads)

> User closes a window that's showing "my-experiment," which has no threads.

- The workspace is dropped.
- **Window State:** the `WindowId` entry is removed.
- **Active List:** unchanged — "my-experiment" was never in it.
- **Sidebar:** "my-experiment" disappears entirely. It was only there because the window was open.

#### Closing the last window (app quit)

> User closes the final window or quits the app.

- **Window State:** cleared entirely.
- **Active List:** unchanged. All persisted entries survive.
- **Sidebar:** on next launch, shows all persisted entries without window indicators. As windows reopen (from session restore), window indicators light up.

---

### Sidebar Interactions

#### Clicking on a persisted project that has no window

> "ex" is in the active list, has threads, but no window is showing it.

- A workspace is created from the stored folder paths, displayed in the current window.
- **Window State:** current window now points to the new workspace.
- **Active List:** unchanged.
- **Sidebar:** "ex" gains a window indicator. Whatever was previously in the current window loses its window indicator (but remains in the sidebar if persisted).

#### Clicking on a persisted project that already has a window

> "ex" is in the active list and Window 2 is already showing it. The user clicks it from the sidebar in Window 1.

- Focus Window 2 (bring it to front). Window 1 stays on its current project.
- **Active List:** unchanged.
- **Sidebar:** "ex" continues to show Window 2's indicator. Window 1's sidebar doesn't change.

#### Clicking X (Remove Project) on a project with an open window

> User clicks the X on "zed" in the sidebar. A window is currently showing "zed."

- **Active List:** "zed" is removed.
- **Window State:** the window needs to show something else. Options:
  - Switch the window to another active project.
  - Show an empty workspace.
  - Close the window.
- **Sidebar:** "zed" disappears from the persisted list. If the window stays open showing something else, that something else appears. If the window closes, it's gone too.
- Any running threads are killed immediately. The removal is a hard stop — if you're removing a project, you're done with it.

#### Clicking X (Remove Project) on a project with no window

> User clicks the X on "old-experiment" in the sidebar. No window is showing it.

- **Active List:** "old-experiment" is removed.
- **Window State:** unchanged.
- **Sidebar:** "old-experiment" disappears. Clean and simple.

#### Clicking X on an ephemeral entry

> User clicks the X on "my-scratch" which is only in the sidebar because its window is open (no threads, not persisted).

- This is equivalent to closing the window.
- **Window State:** the window either closes or switches to another project.
- **Active List:** unchanged — it was never there.
- **Sidebar:** "my-scratch" disappears.

---

### Thread Lifecycle

#### Creating a thread in an ephemeral workspace

> "my-app" is open in a window but has no threads. The user starts a new agent thread.

- The thread is created in the workspace's project.
- **Active List:** "my-app" is now persisted. This is the durable save moment.
- **Window State:** unchanged.
- **Sidebar:** "my-app" was already visible (ephemeral), but now it's backed by the active list. The thread appears under the "my-app" group. If the user closes the window later, "my-app" will remain in the sidebar.

#### Creating a thread in an already-persisted project

> "zed" is in the active list. The user starts another thread.

- The thread is created and shows up in the thread database.
- **Active List:** unchanged — "zed" is already there.
- **Window State:** unchanged.
- **Sidebar:** a new thread entry appears under the "zed" group.

#### A thread finishes generating

> A thread under "zed" transitions from Running to Completed.

- **Active List:** unchanged.
- **Window State:** unchanged.
- **Thread DB:** the thread's status is updated.
- **Sidebar:** the running indicator stops. If the user is looking at a different project in their current window, a notification dot appears on the thread.

#### A thread is running when the window closes

> User closes a window while a thread under "zed" is still generating.

- The workspace entity stays alive because the thread holds a reference to it.
- **Window State:** the `WindowId` entry is removed.
- **Active List:** unchanged — "zed" is persisted.
- **Sidebar:** "zed" loses its window indicator but stays visible. The thread still shows as running. The user can click "zed" to reopen it and see the thread working.
- The workspace entity stays alive because threads need a live `Entity<Workspace>` for tool execution (file reads, writes, terminal commands, etc.). The workspace just isn't displayed in any window.

#### A thread is running when the user clicks Remove Project

> User clicks X on "zed" while a thread is actively generating.

- Running threads are killed immediately. The removal is a hard stop.
- **Active List:** "zed" is removed.
- **Window State:** if a window was showing "zed," it switches to another project or shows an empty workspace.
- **Sidebar:** "zed" and all its threads disappear.

---

### Project Mutations

#### Adding a folder to a workspace

> "zed" is open. User clicks "Add Folder" and adds "cloud."

- The workspace now has worktrees `{cloud, zed}`.
- **Active List:** if "zed" was persisted, its stored folder paths should update to `{cloud, zed}`.
- **Window State:** unchanged — same workspace, same window.
- **Sidebar:** on next derivation, the workspace's folder set changes from `{zed}` to `{cloud, zed}`. The entry moves from the "zed" group to the "cloud, zed" group. Any threads that were created against just `{zed}` stay under the "zed" group. The new workspace shows under "cloud, zed."

#### Removing a folder from a workspace

> "cloud, zed" is open. User removes the "cloud" folder.

- The workspace now has worktrees `{zed}`.
- **Active List:** folder paths update to `{zed}`.
- **Sidebar:** the workspace moves from the "cloud, zed" group to the "zed" group. If "cloud, zed" has no other workspaces or threads left, that group disappears.

#### Renaming a folder on disk

> The user renames `/home/user/zed` to `/home/user/zed2` outside of Zed.

- The workspace detects the rename through file watching. Its worktree path updates.
- **Active List:** the stored folder paths should update.
- **Sidebar:** the group display name changes from "zed" to "zed2." Threads created against the old path may no longer match — they'd appear as orphaned under a "zed" group (if any remain) or need path reconciliation.
- **Open question:** How aggressively do we reconcile old thread paths with renamed folders? This might be a "good enough" situation where old threads keep their original paths and gradually age out of relevance.

#### Creating a git worktree

> User creates a git worktree "olivetti" from the "zed" repo for a new branch.

- A new workspace is created with the worktree path (e.g., `/tmp/zed-olivetti`).
- **Window State:** the new workspace is assigned to a window.
- **Sidebar:** with git canonicalization, `path_list_from_workspace` resolves `/tmp/zed-olivetti` to the same canonical `PathList` as `/home/user/zed`. Both workspaces appear under the "zed" group. The worktree entry gets an annotation like "in olivetti."
- Without git canonicalization (fallback), the worktree would appear as its own group "zed-olivetti." This is the progressive enhancement — git support makes it smarter.

---

### Multi-Window Scenarios

#### Two windows, same project

> Window 1 and Window 2 are both showing "zed."

- **Window State:** both `WindowId`s point to workspaces with the "zed" folder set. (These may be the same `Entity<Workspace>` or different ones.)
- **Active List:** one entry for "zed."
- **Sidebar:** "zed" shows with indicators for both windows. Visible from both windows' sidebars.

#### Closing one of two windows showing the same project

> Window 1 closes. Window 2 stays open on "zed."

- **Window State:** Window 1's entry is removed.
- **Active List:** unchanged.
- **Sidebar:** "zed" now shows only Window 2's indicator. Still fully visible and active.

#### Each window showing a different project

> Window 1 shows "zed," Window 2 shows "ex."

- **Sidebar:** (visible from both windows, since it's global) shows both "zed" and "ex." "zed" has Window 1's indicator. "ex" has Window 2's indicator. The user can click either to switch the current window.

#### Switching a window from one project to another

> Window 1 is showing "zed." User clicks "ex" in the sidebar.

- **Window State:** Window 1's assignment changes from "zed" workspace to "ex" workspace.
- **Active List:** unchanged.
- **Sidebar:** "zed" loses Window 1's indicator (but stays in sidebar if persisted). "ex" gains Window 1's indicator.

---

### Edge Cases

#### App restart / session restore

> User quits and relaunches Zed.

- **Active List:** loaded from database. All persisted entries are present.
- **Window State:** empty initially, then populated as windows restore.
- All persisted entries are eagerly rehydrated into live `Entity<Workspace>` instances — worktrees open, language servers start, file watchers attach.
- **Sidebar:** immediately shows all active projects (live, with real-time folder data) without window indicators. As session restore opens windows, indicators appear.

#### Empty sidebar (fresh install, no history)

> User opens Zed for the first time.

- **Active List:** empty.
- **Window State:** one window with an empty workspace.
- **Sidebar:** shows one ephemeral entry for the empty workspace. Possibly shows recent projects below (from the existing recent projects infrastructure, separate from the active list).

#### Persisted project whose folders no longer exist on disk

> "old-project" is in the active list, but `/home/user/old-project` was deleted.

- During rehydration at startup, we detect that the folder paths no longer exist.
- **Active List:** the entry is auto-removed. If the folder is gone, the project is no longer active.
- **Sidebar:** "old-project" never appears. Clean slate.
- Threads associated with "old-project" are still in the thread database on disk. They're not lost — they're just not surfaced in the active projects sidebar. A future "historical threads" view or search could find them.

#### A thread's folder paths don't match any active project

> A thread was created against `{zed}` but the user removed "zed" from the active list. The thread still exists in the thread database.

- **Sidebar:** the thread doesn't appear. It's not associated with any active project or open window.
- The thread is still in the thread database and would appear if "zed" is re-added to the active list (e.g., by opening it again and creating a new thread, or opening it from recent projects).
- These orphaned threads are not lost — they're just not visible in the active projects sidebar. They could be found through a separate "all threads" search or history view.

#### Multiple workspaces with overlapping but not identical folder sets

> Workspace A has `{cloud, zed}`. Workspace B has `{zed}`. Workspace C has `{cloud}`.

- These are three different `PathList`s and three different project groups.
- **Sidebar:** shows three groups — "cloud," "cloud, zed," and "zed." Each with their own entries and threads.
- A thread created in Workspace A appears under "cloud, zed" — not under "cloud" or "zed" individually.

#### Creating a thread in a workspace, then adding a folder

> "zed" is open, user creates a thread (persisted as `{zed}`), then adds "cloud" to the workspace.

- The workspace's `PathList` is now `{cloud, zed}`.
- The thread was created against `{zed}`.
- **Active List:** the entry's folder paths update to `{cloud, zed}`.
- **Sidebar:** the workspace entry moves to the "cloud, zed" group. The old thread (created against `{zed}`) stays under the "zed" group (because the thread DB recorded `{zed}` as its `PathList`). Future threads in this workspace will be recorded against `{cloud, zed}`.
- This is a natural consequence of derived grouping — the thread doesn't retroactively change its `PathList`. It was created in a `{zed}` context and stays there.