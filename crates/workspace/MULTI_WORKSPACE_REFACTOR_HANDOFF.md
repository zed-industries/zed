# Multi-Workspace Refactor Handoff Document

## Overview

This document captures the current state of the Phase A multi-workspace refactor, which extracts pane-related state from `MultiWorkspace` into an inner `Workspace` struct.

## Goal

Extract UI/pane/panel ownership from `MultiWorkspace` into a new `Workspace` struct as a mechanical refactor with no behavior change. This prepares the codebase for Phase B where multiple `Workspace` instances can share worktrees.

## Current State: PHASE A COMPLETE

### Completed

1. **`single_workspace::Workspace` struct fully defined** (`crates/workspace/src/single_workspace.rs`)
   - Contains pane-related fields: `center`, `panes`, `active_pane`, `panes_by_item`, `last_active_center_pane`, `pane_history_timestamp`
   - Has accessor methods for all fields (including `_mut` variants for mutations)
   - Has `render_center()` helper method
   - Has `center_pane_count()` helper method

2. **`MultiWorkspace` struct updated** (`crates/workspace/src/workspace.rs`)
   - Added `workspace: Entity<Workspace>` field
   - Removed fields that moved to `Workspace`

3. **Constructor updated**
   - Creates `Workspace` entity with initial pane state

4. **All accessor methods updated and working**
   - `panes(&self, cx: &App) -> Vec<Entity<Pane>>` - delegates to workspace
   - `active_pane(&self, cx: &App) -> Entity<Pane>` - delegates to workspace
   - `pane_for(&self, handle, cx: &App) -> Option<Entity<Pane>>` - delegates to workspace
   - `pane_history_timestamp(&self, cx: &App) -> Arc<AtomicUsize>` - delegates to workspace
   - `last_active_center_pane(&self, cx: &App) -> Option<WeakEntity<Pane>>` - delegates to workspace
   - `bounding_box_for_pane(&self, pane, cx: &App)` - delegates to workspace

5. **All internal usages updated**
   - Mutations use `self.workspace.update(cx, |ws, cx| ...)` pattern
   - Reads use `self.workspace.read(cx).method()` pattern

6. **All external crates updated** (~20 crates)
   - `editor` - 8 usages fixed
   - `vim` - all usages fixed
   - `search` - all usages fixed
   - `collab_ui` - all usages fixed
   - `terminal_view` - all usages fixed
   - `git_ui` - all usages fixed
   - `agent_ui` - all usages fixed
   - `onboarding` - all usages fixed
   - `markdown_preview` - all usages fixed
   - `tab_switcher` - all usages fixed
   - `project_symbols` - all usages fixed
   - `extensions_ui` - all usages fixed
   - `keymap_editor` - all usages fixed
   - `outline_panel` - all usages fixed
   - `repl` - all usages fixed
   - `assistant_slash_commands` - all usages fixed
   - `diagnostics` - all usages fixed
   - `svg_preview` - all usages fixed
   - `zed` (main app) - all usages fixed

7. **All tests passing**
   - 109 tests passing in workspace crate
   - Full workspace compilation with 0 errors

## API Changes Summary

The main API change is that `active_pane()`, `panes()`, and `pane_for()` now require a `cx: &App` parameter:

```rust
// Before
workspace.active_pane().clone()
workspace.panes().iter()
workspace.pane_for(handle)

// After
workspace.active_pane(cx)  // Note: no .clone() needed, returns owned Entity
workspace.panes(cx).iter() // Note: returns Vec, use .into_iter() for iterators
workspace.pane_for(handle, cx)
```

**Important notes:**
- `active_pane(cx)` returns `Entity<Pane>` (owned), not a reference
- `panes(cx)` returns `Vec<Entity<Pane>>` (owned), use `.into_iter()` before chaining iterator methods like `.find()`, `.filter_map()`, etc.
- Inside `update` closures, the inner `cx: &mut Context<_>` can be used (it derefs to `App`)
- `items_of_type(cx)` now returns `Vec<Entity<T>>`, use `.into_iter()` before iterator methods

## Files Modified

### Core Workspace Files
- `crates/workspace/src/single_workspace.rs` - Full implementation of inner Workspace
- `crates/workspace/src/workspace.rs` - MultiWorkspace refactored to delegate to Workspace
- `crates/workspace/src/item.rs` - Updated for new accessor patterns
- `crates/workspace/src/pane.rs` - Updated for new accessor patterns

### External Crate Files (partial list)
- `crates/editor/src/editor.rs`
- `crates/editor/src/element.rs`
- `crates/editor/src/items.rs`
- `crates/vim/src/vim.rs`
- `crates/vim/src/state.rs`
- `crates/vim/src/normal/mark.rs`
- `crates/search/src/project_search.rs`
- `crates/search/src/buffer_search/registrar.rs`
- `crates/git_ui/src/commit_view.rs`
- `crates/git_ui/src/file_diff_view.rs`
- `crates/git_ui/src/file_history_view.rs`
- `crates/git_ui/src/project_diff.rs`
- `crates/git_ui/src/text_diff_view.rs`
- `crates/agent_ui/src/agent_diff.rs`
- `crates/agent_ui/src/text_thread_editor.rs`
- (and many more)

## Commands

```bash
# Check workspace crate (should be 0 errors)
cargo check -p workspace 2>&1 | grep -E "^error\[E" | wc -l

# Check full workspace error count (should be 0)
cargo check --workspace 2>&1 | grep -E "^error\[E" | wc -l

# Run workspace tests (should pass)
cargo test -p workspace
```

## Architecture Summary

```
MultiWorkspace (crates/workspace/src/workspace.rs)
├── workspace: Entity<Workspace>  ← holds pane state
├── project: Entity<Project>
├── left_dock, right_dock, bottom_dock
├── follower_states, leader states
├── notifications, status_bar
└── ... other UI state

Workspace (crates/workspace/src/single_workspace.rs)
├── center: PaneGroup
├── panes: Vec<Entity<Pane>>
├── active_pane: Entity<Pane>
├── panes_by_item: HashMap<EntityId, WeakEntity<Pane>>
├── last_active_center_pane: Option<WeakEntity<Pane>>
├── pane_history_timestamp: Arc<AtomicUsize>
└── worktree_ids: HashSet<WorktreeId>  (for future Phase B)
```

## Queued Changes (Phase A.1)

### 1. Move Rendering to Workspace (ANALYSIS COMPLETE)

#### Current State Analysis

The `MultiWorkspace::render()` method (~600 lines, lines 7103-7677 in workspace.rs) currently handles:
1. Key context setup (debugger status, dock states)
2. Titlebar rendering
3. Center pane rendering (delegates to `PaneGroup::render()`)
4. Dock rendering (left, right, bottom)
5. Utility pane rendering
6. Zoomed view overlay
7. Notifications
8. Status bar
9. Modal layer
10. Toast layer
11. Client-side window decorations wrapper

**Dependencies for center pane rendering:**
The `PaneRenderContext` (pane_group.rs:318) requires:
- `project: &Entity<Project>` - on MultiWorkspace
- `follower_states: &HashMap<CollaboratorId, FollowerState>` - on MultiWorkspace
- `active_call: Option<&Entity<ActiveCall>>` - global state
- `active_pane: &Entity<Pane>` - now accessible via Workspace
- `app_state: &Arc<AppState>` - on MultiWorkspace
- `workspace: &WeakEntity<MultiWorkspace>` - self-reference

#### Implementation Options

**Option A: Minimal (Current State)**
- Workspace already has `render_center()` that delegates to `PaneGroup::render()`
- No additional changes needed; works correctly
- Center rendering is conceptually "owned" by Workspace

**Option B: Move docks to Workspace (Significant change)**
Would require moving:
- `left_dock`, `right_dock`, `bottom_dock` from MultiWorkspace to Workspace
- All dock resize/toggle methods (~15 methods)
- Dock-related test code
- **Impact:** ~50+ method changes across workspace.rs

**What should stay in MultiWorkspace:**
- Modal layers, toast layers (window-level)
- Window decorations (`client_side_decorations`)
- Titlebar
- Notifications
- Status bar
- Key context setup
- Zoomed view state

**Current Workspace rendering capability:**
```rust
// In single_workspace.rs - already exists:
pub fn render_center(
    &self,
    zoomed: Option<&AnyWeakView>,
    render_cx: &dyn PaneLeaderDecorator,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    self.center.render(zoomed, render_cx, window, cx)
}
```

**Decision needed from reviewer:** Should docks move to Workspace? This significantly affects Phase B architecture (would each Workspace have its own docks, or are docks shared?)

### 2. Fix Test Compilation Errors (COMPLETE)
Fixed test code across ~15 crates:
- Added `cx` arguments to `active_pane()`, `panes()`, `pane_for()` calls
- Changed `.count()` on `Vec` to `.len()`
- Removed `.collect()` on methods that now return `Vec`
- Fixed closure params from `_` to `cx` where needed
- Fixed `Entity<Pane>` comparisons (owned vs reference)

### 3. Re-introduce Entity<Workspace> to APIs (PENDING REVIEW)
**Problem:** Current pattern where code calls `MultiWorkspace::active_pane(cx)` is vulnerable to race conditions. For example:
```rust
// Dangerous pattern:
let pane = workspace.read(cx).active_pane(cx);
// ... async work ...
// User switches panes during async work
pane.update(cx, |pane, cx| pane.add_item(...)); // Wrong pane!
```

**Solution:** APIs that need to operate on a specific workspace's pane state should take `Entity<Workspace>` or capture it at the start of async operations, not re-query through MultiWorkspace each time.

This change is queued for after the render move and test fixes are reviewed.

---

## Phase B Preview (Future Work)

Once Phase A.1 is complete, Phase B will:
- Allow multiple `Workspace` instances within a `MultiWorkspace`
- Each `Workspace` can have its own subset of worktrees
- Enable split-brain editing across different project contexts
