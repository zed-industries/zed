# Git Commit History Panel - Context for Next Session

## Current Problem

There are TWO commit displays showing:
1. **TOP: "Recent Commits" panel** - This should NOT exist (it's the old render_commit_history_panel)
2. **BOTTOM: Single commit line** - This IS where we want multiple commits to show

The user wants ONLY the bottom area to show multiple commits with the same tooltip styling.

## What Went Wrong

The `render_commit_history_panel` was supposedly removed from the render chain but it's STILL showing up. We need to find where it's being called and completely remove it.

## Files to Check

1. **crates/git_ui/src/git_panel.rs** - Main file
   - Search for `render_commit_history_panel` - should NOT be called anywhere
   - Search for `"Recent Commits"` label - this needs to be removed
   - `render_previous_commit` at ~line 4766 - this is where commits should show (BOTTOM)

2. **crates/git_ui/src/git_panel_settings.rs** - Settings
   - `show_commit_history: bool` - toggle for the feature
   - `commit_history_height: Pixels` - height of scrollable area
   - `commit_history_page_size: usize` - commits per page

3. **assets/settings/default.json** - Default values
   - `"show_commit_history": true`
   - `"commit_history_height": 150`
   - `"commit_history_page_size": 20`

## What User Wants

1. **Location**: Commits show at the BOTTOM of the git panel (where the single HEAD commit currently shows)
2. **Styling**: Same tooltip style as the current commit hover (shows full commit message, author, timestamp)
3. **Scrollable**: When there are many commits, the area should scroll
4. **No separate panel**: NO "Recent Commits" header or separate section at top

## Current Code State

### render_previous_commit (~line 4766)
This function was modified to:
- Show the HEAD commit first (with undo button)
- Then show additional commits from `self.commit_history.entries` (skipping the first one since HEAD is already shown)
- Each commit has `hoverable_tooltip` that shows `GitPanelMessageTooltip`

### render_commit_history_panel (~line 4437)
This function SHOULD NOT be called but apparently still is. It creates the "Recent Commits" section at the top.

### The render() method (~line 5820)
Check if `.children(self.render_commit_history_panel(window, cx))` is still in there - it should be REMOVED.

## Key Structs

```rust
struct CommitHistoryState {
    entries: Vec<FileHistoryEntry>,
    loading_more: bool,
    has_more: bool,
    selected_entry: Option<usize>,
    scroll_handle: UniformListScrollHandle,
}
```

```rust
// In GitPanel
commit_history: Option<CommitHistoryState>,
commit_history_height: Option<Pixels>,
commit_history_drag_start_height: Option<Pixels>,
commit_history_drag_start_y: Option<Pixels>,
```

## Backend API

`branch_history_paginated` in:
- `crates/git/src/repository.rs` - GitRepository trait
- `crates/project/src/git_store.rs` - Repository wrapper
- `crates/fs/src/fake_git_repo.rs` - Test stub

## Next Steps

1. Find and remove ANY call to `render_commit_history_panel` in the render chain
2. Remove the "Recent Commits" label/section entirely
3. Verify `render_previous_commit` is the ONLY place showing commits
4. Test that hovering shows the tooltip with full commit message
5. Test scrolling works when many commits

## Build Commands

```bash
# Quick check
cargo check -p git_ui

# Build for testing
cargo build -p git_ui

# Run debug build
/Volumes/Code/GitHub/zed/target/debug/zed
```
