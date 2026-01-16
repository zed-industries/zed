# Project Panel Search Bar Implementation

## Overview
Added a search/filter bar above the file explorer (project panel) in Zed. When typing, it filters the file tree to show only matching files with case-insensitive substring matching. Directories remain visible to maintain tree structure.

## Feature Behavior
- Search input bar positioned at the top of the project panel (above the file tree)
- Real-time filtering as user types
- Case-insensitive substring matching on file names
- Directories always remain visible (to maintain tree structure)
- Clear button (X) appears when there's text in the filter
- Placeholder text: "Filter files…"

## Files Modified

### Primary File: `crates/project_panel/src/project_panel.rs`

---

## Detailed Changes

### 1. Added Imports (lines 59-64)

**Before:**
```rust
use ui::{
    Color, ContextMenu, DecoratedIcon, Divider, Icon, IconDecoration, IconDecorationKind,
    IndentGuideColors, IndentGuideLayout, KeyBinding, Label, LabelSize, ListItem, ListItemSpacing,
    ScrollAxes, ScrollableHandle, Scrollbars, StickyCandidate, Tooltip, WithScrollbar, prelude::*,
    v_flex,
};
```

**After:**
```rust
use ui::{
    Color, ContextMenu, DecoratedIcon, Divider, Icon, IconButton, IconButtonShape, IconDecoration,
    IconDecorationKind, IndentGuideColors, IndentGuideLayout, KeyBinding, Label, LabelSize,
    ListItem, ListItemSpacing, ScrollAxes, ScrollableHandle, Scrollbars, StickyCandidate, Tab,
    Tooltip, WithScrollbar, prelude::*, v_flex,
};
```

**Added imports:**
- `IconButton` - for the clear filter button
- `IconButtonShape` - for square button shape
- `Tab` - for `Tab::container_height(cx)` to get consistent header height

---

### 2. Added Fields to ProjectPanel Struct (around line 127-129)

**Location:** Inside `pub struct ProjectPanel { ... }`

**Added after `filename_editor: Entity<Editor>,`:**
```rust
    filename_editor: Entity<Editor>,
    filter_editor: Entity<Editor>,
    _filter_subscription: Subscription,
    clipboard: Option<ClipboardEntry>,
```

**New fields:**
- `filter_editor: Entity<Editor>` - The text input for the search/filter bar
- `_filter_subscription: Subscription` - Keeps the editor change subscription alive (underscore prefix indicates it's stored but not directly accessed)

---

### 3. Initialize Filter Editor in `new()` Method (around lines 775-788)

**Location:** Inside `fn new()`, after the `cx.observe_global::<FileIcons>` block and before `let mut project_panel_settings`

**Added code:**
```rust
            let filter_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("Filter files…", window, cx);
                editor
            });
            let filter_subscription = cx.subscribe_in(
                &filter_editor,
                window,
                |panel: &mut Self, _, event, window, cx| {
                    if let EditorEvent::BufferEdited = event {
                        panel.update_visible_entries(None, false, false, window, cx);
                    }
                },
            );
```

**What this does:**
- Creates a single-line editor with placeholder text "Filter files…"
- Subscribes to `EditorEvent::BufferEdited` events
- When the filter text changes, triggers `update_visible_entries()` to re-filter the file tree

---

### 4. Added Fields to Self Initialization (around lines 829-830)

**Location:** Inside the `Self { ... }` struct initialization in `new()`

**Added after `filename_editor,`:**
```rust
                filename_editor,
                filter_editor,
                _filter_subscription: filter_subscription,
                clipboard: None,
```

---

### 5. Added `render_filter_header()` Method (around lines 4598-4630)

**Location:** Added as a new method in `impl ProjectPanel`, right before `fn render_entry()`

**Full method:**
```rust
    fn render_filter_header(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let query = self.filter_editor.read(cx).text(cx);
        let has_query = !query.is_empty();

        h_flex()
            .p_2()
            .h(Tab::container_height(cx))
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .w_full()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.filter_editor.clone()),
            )
            .when(has_query, |this| {
                this.child(
                    IconButton::new("clear_filter", IconName::Close)
                        .shape(IconButtonShape::Square)
                        .tooltip(Tooltip::text("Clear Filter"))
                        .on_click(cx.listener(|panel, _, window, cx| {
                            panel.filter_editor.update(cx, |editor, cx| {
                                editor.set_text("", window, cx);
                            });
                        })),
                )
            })
    }
```

**Key details:**
- Return type uses `+ use<>` to fix Rust 2024 lifetime capture issues
- Uses `Tab::container_height(cx)` for consistent height with other panel headers
- Horizontal flex layout with padding, border at bottom
- Left side: magnifying glass icon + filter editor
- Right side: clear button (only shown when `has_query` is true)
- Clear button calls `editor.set_text("", window, cx)` to clear the filter

---

### 6. Modified `render()` to Include Filter Header (around lines 5705 and 5869)

**Location:** Inside `impl Render for ProjectPanel`, in the `fn render()` method

**Change 1 - Render filter header early (line 5705):**

**Before:**
```rust
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_worktree = !self.state.visible_entries.is_empty();
        let project = self.project.read(cx);
```

**After:**
```rust
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_worktree = !self.state.visible_entries.is_empty();
        let filter_header = self.render_filter_header(cx);
        let project = self.project.read(cx);
```

**Why:** The filter header must be rendered BEFORE borrowing `project` from `cx` to avoid borrow conflicts.

**Change 2 - Add filter header as child (line 5869):**

**Before:**
```rust
                .child(
                    v_flex()
                        .child(
                            uniform_list("entries", item_count, {
```

**After:**
```rust
                .child(
                    v_flex()
                        .child(filter_header)
                        .child(
                            uniform_list("entries", item_count, {
```

---

### 7. Modified `update_visible_entries()` to Apply Filter (around lines 3519-3623)

**Location:** Inside `fn update_visible_entries()`

**Change 1 - Capture filter query before background spawn (lines 3519-3520):**

**Added after `let hide_hidden = settings.hide_hidden;`:**
```rust
        let filter_query = self.filter_editor.read(cx).text(cx).to_lowercase();
        let has_filter = !filter_query.is_empty();
```

**Why:** These values are captured before the `cx.spawn_in()` call so they can be used in the background task.

**Change 2 - Add filter check in entry iteration (lines 3613-3623):**

**Before:**
```rust
                            auto_folded_ancestors.clear();
                            if (!hide_gitignore || !entry.is_ignored)
                                && (!hide_hidden || !entry.is_hidden)
                            {
                                visible_worktree_entries.push(entry.to_owned());
                            }
```

**After:**
```rust
                            auto_folded_ancestors.clear();
                            let matches_filter = !has_filter
                                || !entry.is_file()
                                || entry
                                    .path
                                    .file_name()
                                    .map(|n| n.to_lowercase().contains(&filter_query))
                                    .unwrap_or(false);
                            if (!hide_gitignore || !entry.is_ignored)
                                && (!hide_hidden || !entry.is_hidden)
                                && matches_filter
                            {
                                visible_worktree_entries.push(entry.to_owned());
                            }
```

**Filter logic explained:**
- `!has_filter` - If no filter is active, show the entry
- `!entry.is_file()` - If entry is a directory, always show it (maintains tree structure)
- `entry.path.file_name().map(|n| n.to_lowercase().contains(&filter_query))` - For files, check if filename contains the filter query (case-insensitive)

---

## Build Commands Used

```bash
# Quick type check (fast, ~3 seconds)
cargo check -p project_panel

# Full release build (needed to test visually, ~7-20 minutes)
cargo build --release -p zed

# Launch the application
./target/release/zed .
```

---

## Technical Notes

### Rust 2024 Lifetime Capture Fix
The `render_filter_header()` method returns `impl IntoElement + use<>`. The `+ use<>` syntax is required because Rust 2024 changed how `impl Trait` captures lifetimes. Without it, the returned element would capture the lifetime of `cx`, causing borrow conflicts in the `render()` method where `cx` is used again after calling `render_filter_header()`.

### Why Directories Always Show
The filter keeps all directories visible to maintain the tree structure. If we filtered out directories, the tree would collapse and users couldn't see the hierarchy. Only files are filtered by name match.

### Subscription Pattern
The `_filter_subscription` field uses an underscore prefix because it's not directly accessed after initialization - it just needs to stay alive to keep the subscription active. When the `ProjectPanel` is dropped, the subscription is automatically cleaned up.

---

## Future Improvements (Not Yet Implemented)

1. **File type filtering** - Filter by extension (e.g., `*.rs`, `*.ts`)
2. **Fuzzy matching** - Use the existing `fuzzy` crate for smarter matching
3. **Keyboard shortcut** - Add `Cmd+F` or similar to focus the filter input
4. **Hide empty directories** - Don't show directories that have no matching files inside them
5. **Highlight matches** - Highlight the matching portion of filenames

---

## Git Branch
All changes are on branch: `terminal-tab-customization`

## Related Previous Work
This builds on previous work that added terminal tab color customization (renaming tabs, changing background/text colors).
