# Sidebar v2: Multi-Workspace Project Sidebar

## Mental Model

The sidebar is **not** a history view. It is the **window's workspace manager** — analogous to editor tabs, but for entire workspaces. Each OS window has its own sidebar with its own set of open projects/workspaces. This is ephemeral session state: started fresh per window, manually managed by the user. No cross-window synchronization.

This means:
- Opening a window gives you one workspace (your initial project).
- You can add more workspaces to the window (open another project, create a new empty workspace, start a new thread in a new worktree).
- The sidebar shows all open workspaces grouped by project, with their threads listed underneath.
- Recent projects / history are **not** shown in the sidebar. That's a separate concern (file menu, command palette, etc.).

## What the Screenshots Show

### Left panel — the sidebar itself

1. **Title bar**: "Threads" label, close button (left), new-thread button (right, `+` with gear ⚙️).

2. **Project groups**: Threads are grouped by their worktree paths (the "project"). Each group has:
   - A **header** showing the project folder names (e.g. `ex`, `ex, zed`, `zed`) with a colored sidebar indicator showing workspace associations.
   - **Thread entries** underneath, each showing:
     - Agent icon (Zed Agent, Claude, Codex CLI, etc.)
     - Thread title (truncated with `...`)
     - Optional: author name (e.g. `olivetti`, `rosewood`)
     - Optional: diff stats (`+21 -12`)
     - Timestamp (e.g. `5:45 PM`, `1d`, `3d`)
   - A **"+ View More"** link at the bottom of groups with many threads.

3. **Project group actions** (visible on hover / right-click):
   - "Remove Project" (with keybinding)
   - "Collapse Project" (with keybinding)

4. **New Thread dropdown** (from `+` button):
   - "New Thread in..."
     - "Current Project"
     - "New Worktree"

5. **Agent picker** (dropdown from agent selector in toolbar):
   - "Zed Agent" (default)
   - "External Agents" section: Claude Code, Codex CLI, Gemini CLI, OpenCode
   - "+ Add More Agents"

6. **Search bar** at top for filtering threads.

---

## Current State (what exists today)

The current `Sidebar` in `crates/sidebar/src/sidebar.rs` is a flat list using a `Picker` with `WorkspacePickerDelegate`. It has:

- **`WorkspaceThreadEntry`**: One entry per workspace, showing worktree label + the active thread's title/status.
- **`SidebarEntry`**: Either a separator, a workspace-thread entry, or a recent project.
- **Recent projects**: Fetched from disk, shown below active workspaces, time-bucketed (Today, Yesterday, etc.).
- **Notifications**: Tracks when background workspaces finish generating.
- **Search**: Fuzzy matching across workspace names and recent project names.

### Key gaps vs. the target design

| Feature | Current | Target |
|---------|---------|--------|
| Data model | Flat list of workspaces | Flat `ListEntry` enum with project headers + threads |
| Threads shown | Only the *active* thread per workspace | *All* threads for each project group |
| Recent projects | Shown in sidebar | **Removed** from sidebar |
| Grouping | None (flat list with separators) | By worktree paths (project headers in flat list) |
| Thread source | `AgentPanel.active_thread_view()` | `ThreadStore.threads_for_paths()` for saved threads + active thread from `AgentPanel` |
| Collapsible groups | No | Yes |
| "View More" pagination | No | Yes (show N most recent, expand on click) |
| Project actions | Remove workspace only | Remove project, collapse project |
| New thread flow | Creates empty workspace | "New Thread in Current Project" / "New Worktree" |
| Workspace color indicators | None | Colored vertical bars per workspace |
| Rendering | `Picker` with `PickerDelegate` | `ListState` + `render_list_entry` (collab_panel pattern) |

---

## Implementation Plan

### Phase 1: New Data Model & List Infrastructure

**Goal**: Replace the current `WorkspaceThreadEntry` / `SidebarEntry` / `Picker` model with a flat `ListEntry` enum and `ListState`-based rendering, following the pattern established by `collab_panel.rs`. Remove recent projects from the sidebar entirely.

The key insight from `collab_panel` is: **don't model the hierarchy in your data structures — model it in your `update_entries()` function**. You have a flat `Vec<ListEntry>` and a single `update_entries()` method that walks the data sources (workspaces, thread store) and pushes entries in the right order. The grouping is implicit in the push order.

#### 1.1 Remove recent projects

- Remove `RecentProjectEntry` from the entry enum.
- Remove `recent_projects`, `recent_project_thread_titles`, `_fetch_recent_projects` from the sidebar.
- Remove `get_recent_projects` dependency and time-bucketing logic (`TimeBucket`, etc.).
- Remove `recent_projects` dependency from `Cargo.toml`.

#### 1.2 Define a flat `ListEntry` enum

Replace `SidebarEntry`, `WorkspaceThreadEntry`, `SidebarMatch`, and the `WorkspacePickerDelegate` with a single flat enum. Each variant carries all the data it needs to render, just like `collab_panel::ListEntry`:

```rust
enum ListEntry {
    /// A project group header (e.g. "ex", "ex, zed", "zed").
    /// Not selectable. Clicking toggles collapse.
    ProjectHeader {
        path_list: PathList,
        label: SharedString,
    },
    /// A thread belonging to the project group above it.
    Thread {
        session_id: acp::SessionId,
        title: SharedString,
        icon: IconName,
        status: AgentThreadStatus,
        updated_at: DateTime<Utc>,
        diff_stats: Option<(usize, usize)>,
        /// If this thread is actively running in a workspace, which one.
        workspace_index: Option<usize>,
    },
    /// "+ View More" link at the end of a project group.
    ViewMore {
        path_list: PathList,
        remaining_count: usize,
    },
}
```

Auxiliary state lives on `Sidebar` itself, not in the entries:
- `collapsed_groups: HashSet<PathList>` — which project groups are collapsed.
- `expanded_groups: HashSet<PathList>` — which groups have "View More" expanded (default is collapsed to N items).
- `selection: Option<usize>` — index into `entries`.
- `entries: Vec<ListEntry>` — the flat list, rebuilt on every change.

#### 1.3 Replace Picker with ListState-based rendering

Drop the `Picker<WorkspacePickerDelegate>` entirely. Instead, follow the collab_panel pattern:

- `Sidebar` owns a `ListState`, a `Vec<ListEntry>`, an optional `selection: Option<usize>`, and a search `Editor`.
- Render with `list(self.list_state.clone(), cx.processor(Self::render_list_entry)).size_full()`.
- `render_list_entry(&mut self, ix: usize, window, cx) -> AnyElement` matches on `self.entries[ix]` and dispatches to `render_project_header()`, `render_thread()`, `render_view_more()`.
- Keyboard nav (`select_next`, `select_previous`, `confirm`) is implemented directly on `Sidebar` via action handlers, same as collab_panel.

This gives us full-width rendering for every item (no picker chrome), collapsible headers, and direct control over the list.

#### 1.4 Build the flat list in `update_entries()`

A single `update_entries(&mut self, cx)` method (called whenever workspaces or threads change) rebuilds `self.entries` from scratch:

1. Gather open workspaces from `MultiWorkspace.workspaces()`. For each, compute its `PathList` from worktree paths.
2. For each workspace's `PathList`, query `ThreadStore::global(cx).threads_for_paths(&path_list)` to get saved threads for that project.
3. List workspace groups in workspace creation order (i.e. their order in `MultiWorkspace.workspaces()`).
4. For each workspace group:
   - Push `ListEntry::ProjectHeader { path_list, label }` (always visible, even when collapsed).
   - If not in `collapsed_groups`, push `ListEntry::Thread { ... }` for each thread (active thread from `AgentPanel` merged with saved threads from `threads_for_paths()`, deduped by session ID, sorted by `updated_at` descending).
   - If there are more than N threads and the group isn't in `expanded_groups`, push only the first N threads then `ListEntry::ViewMore { remaining_count }`.
5. Update `self.list_state` item count.

This is the same imperative "walk and push" pattern as `collab_panel::update_entries`.

#### 1.5 Subscribe to data sources

Add subscriptions so `update_entries()` is called when data changes:
- **`MultiWorkspace`** (already exists): workspace added/removed/activated.
- **`ThreadStore::global(cx)`** (new): threads saved/deleted/reloaded.
- **Per-workspace `AgentPanel`** (already exists): active thread changes, thread status changes.
- **Per-workspace `Project`** (already exists): worktree added/removed (changes which group a workspace belongs to).

#### 1.6 Tests for `update_entries()`

Use visual snapshot tests following the `project_panel_tests.rs` pattern. Write a `visible_entries_as_strings` helper that reads `self.entries` and formats each `ListEntry` into a human-readable string, then assert against expected output.

**Helper function**:

The helper should show collapse state on project headers (`>` collapsed, `v` expanded), and selection state (`<== selected`) on any entry — mirroring the project panel pattern:

```rust
fn visible_entries_as_strings(
    sidebar: &Entity<Sidebar>,
    cx: &mut VisualTestContext,
) -> Vec<String> {
    sidebar.read_with(cx, |sidebar, _cx| {
        sidebar.entries.iter().enumerate().map(|(ix, entry)| {
            let selected = if sidebar.selection == Some(ix) {
                "  <== selected"
            } else {
                ""
            };
            match entry {
                ListEntry::ProjectHeader { label, path_list, .. } => {
                    let icon = if sidebar.collapsed_groups.contains(path_list) {
                        ">"
                    } else {
                        "v"
                    };
                    format!("{} [{}]{}", icon, label, selected)
                }
                ListEntry::Thread { title, status, workspace_index, .. } => {
                    let active = if workspace_index.is_some() { " *" } else { "" };
                    let status_str = match status {
                        AgentThreadStatus::Running => " (running)",
                        AgentThreadStatus::Error => " (error)",
                        _ => "",
                    };
                    format!("  {}{}{}{}", title, active, status_str, selected)
                }
                ListEntry::ViewMore { remaining_count, .. } => {
                    format!("  + View More ({}){}", remaining_count, selected)
                }
            }
        }).collect()
    })
}
```

**Test cases**:

1. **Single workspace, no threads**:
   ```
   v [my-project]
   ```

2. **Single workspace with threads from ThreadStore**:
   ```
   v [my-project]
     Fix crash in project panel
     Add inline diff view
     Build a task runner panel
   ```

3. **Multiple workspaces, each with their own threads**:
   ```
   v [project-a]
     Thread A1 * (running)
     Thread A2
   v [project-b]
     Thread B1
   ```

4. **View More when threads exceed N**:
   ```
   v [my-project]
     Thread 1
     Thread 2
     Thread 3
     Thread 4
     Thread 5
     + View More (7)
   ```

5. **Active thread from AgentPanel merged with saved threads**: the active thread appears in the list with `*` marker and is deduped against the ThreadStore copy.

6. **Adding a workspace updates entries**: create a second workspace, assert it appears as a new project header with its threads.

7. **Removing a workspace updates entries**: remove a workspace, assert its project header and threads are gone.

8. **Worktree change updates group label**: add a folder to a workspace, assert the project header label updates (e.g. `v [ex]` → `v [ex, zed]`).

### Phase 2: Rendering List Entries

**Goal**: Implement the `render_list_entry` dispatcher and per-variant render methods.

Each method returns a full-width `AnyElement`. No picker chrome, no indentation magic — each entry is a top-level item in the list, same as collab_panel.

#### 2.1 Render project group headers

Each `ProjectHeader` renders as:
- The group label (derived from folder names, e.g. "ex, zed")
- A collapse/expand chevron
- On hover: action buttons (remove, collapse keybindings)

Headers are not selectable (skipped by keyboard nav).

#### 2.2 Render thread items

Each thread renders using the existing `ThreadItem` component, which already supports:
- Icon, title, timestamp
- Diff stats (`.added()`, `.removed()`)
- Running/completed/error status
- Selected/hovered state
- Action slot (for context menu or remove button)

New additions needed for `ThreadItem`:
- Author/branch name display (visible in screenshots as "olivetti", "rosewood")

#### 2.3 Render "View More" items

When a project group has more than N threads (e.g. 5), show only the N most recent and add a "+ View More" item. Clicking it adds the group's `PathList` to `self.expanded_groups` and calls `update_entries()`.

#### 2.4 Implement collapse/expand

Clicking a project group header or using the keybinding toggles membership in `self.collapsed_groups` and calls `update_entries()`. When collapsed, the group's threads and "View More" are not pushed into `entries` at all — the header is still visible.

#### 2.5 Keyboard navigation and selection

Implement directly on `Sidebar` (no Picker):
- `select_next` / `select_previous` actions: move `self.selection`, skipping `ProjectHeader` entries.
- `confirm` action: if selection is a `Thread`, activate its workspace or open it. If `ViewMore`, expand the group.
- Track `selection: Option<usize>` and pass `is_selected` to render methods for highlighting.

#### 2.6 Tests for collapse/expand, View More expansion, and selection

Reuse `visible_entries_as_strings` from 1.6 — the `>` / `v` and `<== selected` markers make these behaviors directly assertable.

1. **Collapsed group hides its threads**:
   ```
   > [project-a]
   v [project-b]
     Thread B1
   ```

2. **Expanding a collapsed group shows threads again**:
   ```
   v [project-a]
     Thread A1
     Thread A2
   v [project-b]
     Thread B1
   ```

3. **Expanding View More shows all threads**: start with `+ View More (7)`, click it, assert all 12 threads appear and "View More" is gone.

4. **Selection skips headers**:
   ```
   v [project-a]
     Thread A1  <== selected
     Thread A2
   v [project-b]
     Thread B1
   ```
   After `select_next`:
   ```
   v [project-a]
     Thread A1
     Thread A2  <== selected
   v [project-b]
     Thread B1
   ```
   After `select_next` again (jumps over header):
   ```
   v [project-a]
     Thread A1
     Thread A2
   v [project-b]
     Thread B1  <== selected
   ```

5. **Confirm on selection activates workspace**: select a thread with `workspace_index: Some(1)`, confirm, assert `MultiWorkspace.active_workspace_index()` changed.

### Phase 3: Project Group Actions

**Goal**: Implement the context menu actions visible in the screenshots.

#### 3.1 "Remove Project"

- Removes all workspaces associated with this project group from the `MultiWorkspace`.
- If there are no open workspaces for the group (it's only showing historical threads), this is a no-op or hides the group.
- Keybinding: `Shift-Cmd-Backspace` (from screenshot)

#### 3.2 "Collapse Project"

- Toggles the collapsed state of the project group.
- Keybinding: `Ctrl-Cmd-[` (from screenshot)

#### 3.3 "New Thread" dropdown

The `+` button in the header should show a popover/context menu:
- **"Current Project"**: Creates a new thread in the currently active workspace's project. This means creating a new agent thread in the existing workspace's `AgentPanel`.
- **"New Worktree"**: Creates a new empty workspace (existing `create_workspace` behavior) — prompts for a folder to open and starts a thread there.

### Phase 5: Search

**Goal**: The search behavior changes because we now search across thread titles (not just workspace names).

#### 5.1 Update search candidates

The fuzzy search should match against:
- Thread titles
- Project group labels (folder names)

When filtering, show matching threads under their group headers. Hide groups with no matching threads.

### Phase 6: Thread Lifecycle Integration

**Goal**: Ensure the sidebar correctly reflects thread state changes in real time.

#### 6.1 Live thread status

For threads that are actively running in a workspace:
- Subscribe to the workspace's `AgentPanel` events and the active `AcpThread` entity.
- Update status (Running → Completed → Error) in real time.
- The notification system (badge on sidebar toggle button) should continue working.

#### 6.2 Thread saving

When a thread is saved (via `ThreadStore.save_thread`), the `ThreadStore` reloads and notifies observers. The sidebar's `ThreadStore` subscription picks this up and rebuilds entries.

#### 6.3 Thread switching within a workspace

When the user switches threads within a workspace's `AgentPanel`, the sidebar should update to reflect which thread is active/selected.

---

## Execution Order & Dependencies

```
Phase 1 (Data Model & List Infrastructure)
  ├── 1.1 Remove recent projects (pure deletion)
  ├── 1.2 Define ListEntry enum
  ├── 1.3 Replace Picker with ListState (depends on 1.2)
  ├── 1.4 Build flat list in update_entries() (depends on 1.2)
  ├── 1.5 Subscribe to data sources (depends on 1.4)
  └── 1.6 Tests for update_entries() (depends on 1.4, 1.5)

Phase 2 (Rendering) — depends on Phase 1
  ├── 2.1 Render project headers
  ├── 2.2 Render thread items
  ├── 2.3 Render "View More"
  ├── 2.4 Collapse/expand
  └── 2.5 Keyboard navigation

Phase 3 (Actions) — depends on Phase 2
  ├── 3.1 Remove Project
  ├── 3.2 Collapse Project
  └── 3.3 New Thread dropdown

Phase 4 (Color Indicators) — depends on Phase 2
Phase 5 (Search) — depends on Phase 2
Phase 6 (Thread Lifecycle) — depends on Phase 1
```

Phases 1 and 6 are the most critical — they determine the data flow. Phases 2-3 are the UI work. Phases 4-5 are polish.

## What's Explicitly Deferred

- **Git worktree integration**: The "New Worktree" option in the new-thread dropdown hints at this, but the full git-worktree-based workflow (create branch → create worktree → open workspace) is a follow-up.
- **Cross-window sidebar sync**: Explicitly not doing this. Each window manages its own sidebar state.
- **Recent projects in sidebar**: Removed. Recent projects are accessible via command palette / file menu.
- **Thread history browsing**: The sidebar shows threads for *open* project groups. Full thread history browsing is a separate feature.
- **Collaborative features**: The "people" icon in the bottom toolbar is deferred.
- **Bottom toolbar icons**: Exact functionality of the bottom icon row needs clarification — implement container only.

## Files to Modify

| File | Changes |
|------|---------|
| `crates/sidebar/src/sidebar.rs` | Major rewrite: `ListEntry` enum, `ListState` rendering, `update_entries()`, remove Picker |
| `crates/sidebar/Cargo.toml` | Remove `recent_projects` dep, add `agent` dep (for `ThreadStore`, `DbThreadMetadata`) |
| `crates/workspace/src/multi_workspace.rs` | Possibly add helper methods for project-group-level operations |
| `crates/ui/src/components/ai/thread_item.rs` | May need author/branch name field |
| `crates/agent/src/thread_store.rs` | May need additional query methods (e.g. threads grouped by path_list) |
