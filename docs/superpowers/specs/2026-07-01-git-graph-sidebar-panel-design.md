# Git Graph Sidebar Panel — Design & Phased Plan

Date: 2026-07-01
Status: Approved design, ready for phased implementation

## Goal

Show Zed's commit graph in a dockable **sidebar panel** (like the VSCode "Git
Graph" view), instead of only as a wide editor **tab**. Reuse the existing
`GitGraph` entity and all its data loading, DAG layout, search, and context
menus. The panel renders responsively: a compact single-column layout when
docked narrow, progressively revealing the date / author / commit columns as
the dock widens.

## Decisions (locked)

- **Reuse `GitGraph`** rather than writing a new compact graph view.
- **Responsive layout**: compact (graph + subject + ref badges) when narrow,
  reveal extra columns as width grows.
- **Coexist with the tab**: the panel is the default entry point; an *expand*
  icon in the panel header opens the existing full tab view via
  `open_or_reuse_graph`. The tab is no longer opened directly by the user.
- **Commit click in the panel** opens the commit's details in a `CommitView`
  **tab** (reusing `open_commit_view`); the inline details split is suppressed
  in panel mode.

## Current state (codebase facts)

- `crates/git_ui/src/git_graph.rs` (~7000 lines):
  - `pub struct GitGraph` — `impl Item` (opens as a tab) and `impl Render`.
  - `GitGraph::new(repo_id, git_store, workspace, log_source, window, cx)`.
  - `open_or_reuse_graph(...)` → `workspace.add_item_to_active_pane(...)`.
  - `set_repo_id(repo_id, cx)`, `open_commit_view(index, window, cx)`,
    `open_selected_commit_view(...)`.
  - Responsive column plumbing already exists: `column_widths`
    (`RedistributableColumnsState`), `preview_column_fractions`,
    `table_column_width_config`, `graph_viewport_width`.
  - Inline commit details split: `commit_details_split_state`,
    `selected_commit_diff`, `selected_commit_diff_stats`.
  - Persistence of log source/order/open state: `GitGraphsDb`.
- `crates/git_ui/src/git_panel.rs` (~10000 lines) — reference `Panel` impl:
  - `impl Panel for GitPanel` with `persistent_name`/`panel_key`/`position`/
    `set_position`/`default_size`/`icon`/`toggle_action`/`starts_open`/
    `activation_priority`.
  - Active repo tracking via `cx.subscribe` on `GitStoreEvent`
    (`ActiveRepositoryChanged`, `RepositoryAdded/Removed`, ...).
  - `serialization_key(workspace)` + workspace DB for persistence.
  - `GitPanel::load(workspace_handle, cx)` async constructor.
  - `register(workspace)` registers actions (`ToggleFocus`, etc.).
- `crates/git_ui/src/git_ui.rs` `init`: calls `git_graph::init(cx)` and, inside
  `observe_new(workspace)`, calls `git_panel::register(workspace)`.
- `crates/zed/src/zed.rs` ~750: panels are created with `::load` and attached
  with `add_panel_when_ready(...)`.
- `crates/git_ui/src/git_panel_settings.rs`: `GitPanelSettings` via
  `RegisterSetting` (`dock`, `default_width`, `button`, `starts_open`, ...).

## Target architecture (Approach 1)

**New file `crates/git_ui/src/git_graph_panel.rs`** holding:

- `pub struct GitGraphPanel`:
  - `graph: Entity<GitGraph>`
  - `workspace: WeakEntity<Workspace>`
  - `git_store: Entity<GitStore>`
  - `focus_handle: FocusHandle`
  - `width: Option<Pixels>` (persisted dock width)
  - `_subscriptions: Vec<Subscription>` (active-repo sync, graph notify)
- `impl Panel for GitGraphPanel` — boilerplate mirroring `GitPanel`, with its
  own `GitGraphPanelSettings` (dock, default_width, button, starts_open).
- `impl Render for GitGraphPanel` — header (title + search + expand-to-tab
  button + log controls) and the responsive graph body, delegating row
  rendering to a shared `GitGraph` method.
- `GitGraphPanel::load(workspace, cx)` async constructor + `register(workspace)`
  for toggle actions.

**Refactor inside `git_graph.rs`:**

- `struct GraphColumns { description: bool, date: bool, author: bool, commit: bool }`
  (the graph lane canvas is always present). Computed responsively from
  available width via width breakpoints.
- Extract the current `Render`-time table/row construction into
  `GitGraph::render_table(&mut self, columns: GraphColumns, window, cx) -> impl IntoElement`.
  The existing `impl Render for GitGraph` (tab) calls it with all columns
  `true`; `GitGraphPanel` calls it with a responsive subset.
- `enum GitGraphHost { Item, Panel }` field on `GitGraph`, defaulting to `Item`.
  Used **only** where behavior must differ:
  - Panel mode suppresses the inline commit-details split.
  - Panel mode always routes commit activation to `open_commit_view`
    (CommitView tab).
- Expose whatever small accessors the panel needs (search state hooks, log
  source/order controls) as `pub(crate)` methods rather than duplicating logic.

**Wiring:**

- `git_ui.rs` `init`: call `git_graph_panel::register(workspace)` inside the
  existing `observe_new(workspace)` block.
- `zed.rs`: add `GitGraphPanel::load(...)` to the panel set and
  `add_panel_when_ready(...)`.
- Repurpose the existing "open graph" action to **focus/open the panel**;
  keep `open_or_reuse_graph` reachable from the panel's expand button.

## Data flow

1. `GitGraphPanel::load` resolves the active repository, constructs
   `GitGraph::new(repo_id, git_store, workspace, None /* default log source */,
   window, cx)` with host = `Panel`.
2. Panel subscribes to `GitStoreEvent::ActiveRepositoryChanged` and calls
   `graph.set_repo_id(new_repo_id, cx)`; re-renders on graph `notify`.
3. On render, the panel measures available width (its `size()` / dock width),
   computes `GraphColumns`, and calls `graph.render_table(columns, ...)`.
4. Commit click → `graph.open_commit_view(index, window, cx)` → CommitView tab.
5. Expand button → `open_or_reuse_graph(workspace, repo_id, log_source, ...)` →
   full tab; reuses the existing graph state where possible.

## Error handling

- Follow repo conventions: no `unwrap()` / silent `let _ =`. Use `?`,
  `.log_err()`, or explicit handling. Async repo/data failures surface through
  the existing `GitGraph` paths (unchanged).
- No active repository → panel renders an empty/placeholder state (mirror how
  `GitPanel` handles `active_repository == None`).

## Persistence & settings

- New `GitGraphPanelSettings` (dock position, default width, button visibility,
  starts_open) registered alongside `GitPanelSettings`.
- Panel state (open/closed, dock, width) via workspace panel serialization
  (`serialization_key` + workspace DB), mirroring `GitPanel`.
- Graph's own log source/order continues to persist via `GitGraphsDb`.

## Testing

- GPUI tests in `git_graph_panel.rs` (and additions to `git_graph.rs` tests):
  - Panel toggles open/closed and docks left/right.
  - `GraphColumns` computed correctly at representative widths (narrow → only
    subject; wide → all columns).
  - Active-repo change updates the panel's graph (`set_repo_id`).
  - Commit click in panel mode opens a `CommitView` and does **not** open the
    inline split.
  - Expand button opens the full `GitGraph` tab.
- Use GPUI executor timers (per CLAUDE.md), not `smol::Timer`.
- Build with `./script/clippy`.

## Out of scope (YAGNI)

- New graph rendering styles or DAG algorithm changes.
- Changes to `CommitView` itself.
- Multi-repo simultaneous graphs in one panel.

---

## Phased implementation plan

Each phase is independently buildable and verifiable. Run `./script/clippy`
after each phase.

### Phase 1 — Panel scaffold, docking, active-repo wiring
**Goal:** A dockable `GitGraphPanel` that opens/toggles/docks and renders the
existing `GitGraph` (full table, unchanged) inside it.

- Create `crates/git_ui/src/git_graph_panel.rs` with `GitGraphPanel`,
  `impl Panel`, `impl Render` (delegates to `graph`'s existing render for now),
  `load`, and `register`.
- Add `GitGraphPanelSettings` (clone of the relevant `GitPanelSettings`
  fields). Register it.
- Wire `register` into `git_ui.rs` `init`; add `load` + `add_panel_when_ready`
  in `zed.rs`. Add a toggle action (`git_graph_panel::ToggleFocus` or reuse a
  `zed_actions` graph action).
- Construct `GitGraph` with host = `Panel` (add the `GitGraphHost` enum now,
  default `Item`; panel passes `Panel`). No behavioral branching yet beyond
  construction.
- Subscribe to `GitStoreEvent::ActiveRepositoryChanged` → `set_repo_id`.

**Verify:** Panel toggles via action + status-bar icon, docks left/right,
shows the graph, follows the active repository. Builds clean.

### Phase 2 — Panel host behavior (click routing + suppress inline split)
**Goal:** Panel mode behaves like a sidebar, not a full editor.

- Branch on `GitGraphHost::Panel` to:
  - suppress the inline commit-details split (`commit_details_split_state`),
  - route commit activation through `open_commit_view` → CommitView tab.
- Ensure single-click select + activate works from the panel.

**Verify:** Clicking a commit in the panel opens a CommitView tab; no inline
diff split appears in the panel. Tab view behavior unchanged.

### Phase 3 — Responsive columns
**Goal:** Compact-to-wide responsive layout.

- Add `GraphColumns` descriptor and extract `GitGraph::render_table(columns,
  ...)`; update the tab `Render` to call it with all columns.
- Compute `GraphColumns` in the panel from available dock width using
  breakpoints (e.g. only subject below ~X; add commit/date/author at wider
  thresholds), reusing `column_widths` / `preview_column_fractions`.

**Verify:** Resizing the dock collapses/reveals date/author/commit columns;
narrow dock shows graph lanes + subject + ref badges only. Tab unchanged.

### Phase 4 — Expand-to-tab, persistence, polish
**Goal:** Header controls + persisted panel state.

- Add expand icon in the panel header → `open_or_reuse_graph` (full tab).
- Add search + log source/order controls to the panel header (reuse existing
  `GitGraph` controls/methods).
- Implement `serialization_key` + serialize/restore panel open/dock/width.
- Empty/placeholder state when no active repository.

**Verify:** Expand opens the full tab; panel open/dock/width persist across
reload; search and log controls work in the panel.

### Phase 5 — Tests & docs
**Goal:** Coverage + documentation.

- Add GPUI tests listed in the Testing section.
- Document the new setting(s) in user-facing settings docs.
- `Release Notes:` entry: `- Added a Git Graph sidebar panel.`

**Verify:** `./script/clippy` clean; new tests pass.
