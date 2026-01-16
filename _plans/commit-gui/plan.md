# Git Commit History Panel - Implementation Plan

## Overview

Add a resizable, scrollable commit history panel to the Zed Git panel that shows recent commits with lazy loading (infinite scroll).

## Requirements

1. **Commit history list** - Show commits (not just HEAD)
2. **Resizable** - Drag to make bigger/smaller vertically
3. **Scrollbar** - Standard scrollbar for the list
4. **Lazy loading** - Load more commits as user scrolls (infinite scroll)
5. **Configurable** - Settings for initial page size
6. **Persistent** - Remember panel height across sessions

---

## Architecture

### Current Structure (crates/git_ui/src/git_panel.rs)

```
v_flex()
├── panel_header
├── entries list (changed files)
└── footer:
    ├── PanelRepoFooter (repo/branch selectors) - 36px
    └── panel_editor_container (commit message editor)
```

### New Structure

```
v_flex()
├── panel_header
├── entries list (changed files)
├── commit_history_panel:  // NEW
│   ├── resize_handle (top edge, 6px)
│   └── commit_history_list (uniform_list with scrollbar)
│       └── [commit entries...]
└── footer:
    ├── PanelRepoFooter
    └── panel_editor_container
```

---

## Files Modified

| File | Changes |
|------|---------|
| `crates/git/src/repository.rs` | Added `branch_history_paginated()` API and `BranchHistory` struct |
| `crates/project/src/git_store.rs` | Added `branch_history_paginated()` wrapper |
| `crates/git_ui/src/git_panel.rs` | Main UI: CommitHistoryState, render methods, resize handling |
| `crates/git_ui/src/git_panel_settings.rs` | Added height + page_size settings |
| `crates/settings/src/settings_content.rs` | Added settings schema |
| `assets/settings/default.json` | Added default values |

---

## Data Structures

### New Types

```rust
// crates/git/src/repository.rs
pub struct BranchHistory {
    pub entries: Vec<FileHistoryEntry>,  // Reuse existing
    pub branch_name: Option<String>,
}

// crates/git_ui/src/git_panel.rs
struct CommitHistoryState {
    entries: Vec<FileHistoryEntry>,
    loading_more: bool,
    has_more: bool,
    selected_entry: Option<usize>,
    scroll_handle: UniformListScrollHandle,
}

// Marker for drag events
#[derive(Clone)]
struct DraggedCommitHistoryResize;
```

### GitPanel Additions

```rust
pub struct GitPanel {
    // ... existing fields ...
    commit_history: Option<CommitHistoryState>,
    commit_history_height: Option<Pixels>,  // User-set height
    commit_history_drag_start_height: Option<Pixels>,  // During drag
}
```

---

## Settings Schema

```json
{
  "git_panel": {
    "commit_history_height": 150,      // Default height in pixels
    "commit_history_page_size": 20     // Commits per page
  }
}
```

---

## API Addition

### GitRepository Trait (crates/git/src/repository.rs)

```rust
fn branch_history_paginated(
    &self,
    branch_name: Option<String>,  // None = HEAD
    skip: usize,
    limit: Option<usize>,
) -> BoxFuture<'_, Result<BranchHistory>>;
```

Implementation uses:
```bash
git --no-optional-locks log [branch] \
  --pretty=format:%H%x00%s%x00%B%x00%at%x00%an%x00%ae<<COMMIT_END>> \
  --skip=N -n M
```

---

## Key Implementation Patterns

### 1. Resize Handle

The resize handle is at the top of the commit history panel:
- 6px tall invisible handle
- `cursor_row_resize` for visual feedback
- `on_drag` starts drag operation
- `on_drag_move` updates height based on delta
- Double-click resets to default height

### 2. Scroll-based Infinite Loading

```rust
const LOADING_THRESHOLD: usize = 5;

// In uniform_list callback:
if range.end >= entry_count.saturating_sub(LOADING_THRESHOLD)
    && has_more
    && !loading_more
{
    this.load_more_commits(window, cx);
}
```

### 3. Load More Logic

- Set `loading_more = true` to prevent duplicate requests
- Calculate `skip` from current entry count
- Fetch next page asynchronously
- Append new entries on completion
- Set `has_more = false` if fewer entries returned than page_size

---

## Verification

1. **Visual**: Panel appears below changed files list
2. **Resize**: Drag top edge to resize, double-click to reset
3. **Scroll**: Scrollbar appears when content exceeds height
4. **Lazy load**: Scroll to bottom → new commits load
5. **Settings**: Change `commit_history_page_size` → affects load count
6. **Persistence**: Close/reopen Zed → height preserved (TODO)
7. **Click**: Click commit → opens CommitView

---

## Notes

- Reuses `FileHistoryEntry` struct (same data shape)
- Follows `file_history_view.rs` patterns for entry rendering
- Follows `dock.rs` patterns for resize handling
- Default height: 150px (shows ~5-6 commits)
- Default page size: 20 commits
