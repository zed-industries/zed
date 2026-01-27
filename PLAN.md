# Synchronized Scrolling in Side-by-Side Diff Views

## Overview

This document describes the implementation plan for synchronized scrolling between the LHS (left-hand side) and RHS (right-hand side) editors in a split diff view. The core approach is to make the `ScrollManager` a shared `Entity<ScrollManager>` so both editors can share the same scroll state, with appropriate translation between the different multibuffers.

## Key Concepts

- **LHS (Left-Hand Side)**: The editor showing the "before" state (deleted/old content)
- **RHS (Right-Hand Side)**: The editor showing the "after" state (current content)
- **ScrollAnchor**: Contains an `Anchor` and offset that define the scroll position
- **Anchor**: A position in a specific multibuffer, tied to that buffer's structure

Since the LHS and RHS have different multibuffers with different structures, an anchor from one side cannot be directly used on the other side without translation.

## Implementation Steps

### Step 1: Convert `scroll_manager` to `Entity<ScrollManager>`

**File:** `crates/editor/src/editor.rs`

This is a mechanical refactor that can be done first to keep a clean repo state.

Change the field type:

```rust
pub struct Editor {
    // ... other fields ...
    pub scroll_manager: Entity<ScrollManager>,
    // ...
}
```

Update `Editor::new_internal()` to create the entity:

```rust
let scroll_manager = cx.new(|cx| ScrollManager::new(cx));
```

### Step 2: Update All `scroll_manager` Usages

**Files:** Multiple files across the editor crate

All direct field accesses need to be converted to entity reads/updates:

- `self.scroll_manager.anchor()` → `self.scroll_manager.read(cx).anchor()`
- `self.scroll_manager.set_...()` → `self.scroll_manager.update(cx, |sm, cx| sm.set_...(cx))`

Key places to update (non-exhaustive):
- `crates/editor/src/editor.rs`: Many methods access scroll_manager
- `crates/editor/src/element.rs`: Layout and scrollbar handling
- `crates/editor/src/scroll.rs`: Editor impl block methods
- `crates/agent_ui/src/inline_assistant.rs`
- `crates/agent_ui/src/acp/entry_view_state.rs`
- `crates/agent_ui/src/text_thread_editor.rs`
- `crates/editor/src/editor_tests.rs`

### Step 3: Add `SplitSide` to `ScrollAnchor`

**File:** `crates/editor/src/scroll.rs`

We will reuse the existing `SplitSide` enum already defined in `crates/editor/src/element.rs`:

```rust
pub enum SplitSide {
    Left,
    Right,
}
```

Update `ScrollAnchor` to track which side it originated from:

```rust
use crate::element::SplitSide;

pub struct ScrollAnchor {
    pub offset: gpui::Point<ScrollOffset>,
    pub anchor: Anchor,
    /// Which side of a split diff this anchor belongs to.
    /// `None` for non-split editors.
    pub split_side: Option<SplitSide>,
}
```

Update `ScrollAnchor::new()` to initialize `split_side: None`.

### Step 4: Add `Option<SplitSide>` to `EditorSnapshot`

**File:** `crates/editor/src/editor.rs`

Add a field to `EditorSnapshot`:

```rust
pub struct EditorSnapshot {
    // ... existing fields ...
    /// Which side of a split diff this snapshot belongs to.
    /// `None` for non-split editors.
    pub split_side: Option<SplitSide>,
}
```

Update `Editor::snapshot()` to populate this field. The editor will need to know its own split side, which can be stored as a field on `Editor` or passed in when creating the snapshot.

### Step 5: Add `companion_display_snapshot` to `DisplaySnapshot`

**File:** `crates/editor/src/display_map.rs`

Add a field to `DisplaySnapshot`:

```rust
pub struct DisplaySnapshot {
    pub crease_snapshot: CreaseSnapshot,
    block_snapshot: BlockSnapshot,
    /// Display snapshot for the companion editor in a split diff view.
    /// This allows translating scroll positions between LHS and RHS.
    /// Boxed to avoid infinite size due to recursive type.
    pub(crate) companion_display_snapshot: Option<Box<DisplaySnapshot>>,
    // ... existing fields ...
}
```

The `DisplayMap` will need a way to obtain and store the companion's display snapshot. This will be set up when the split is created, where both display maps get references to each other. The `Box` is necessary because `DisplaySnapshot` cannot contain itself directly (would be infinite size).

### Step 6: Update `scroll_position` Translation Logic

**File:** `crates/editor/src/editor.rs`

Update `EditorSnapshot::scroll_position()` to handle translation. The key insight is that "translation" simply means resolving the anchor against the companion's display snapshot instead of our own:

```rust
pub fn scroll_position(&self) -> gpui::Point<ScrollOffset> {
    let anchor_side = self.scroll_anchor.split_side;
    let snapshot_side = self.split_side;
    
    // If sides match or no translation needed, use direct calculation
    if anchor_side == snapshot_side || anchor_side.is_none() || snapshot_side.is_none() {
        self.scroll_anchor.scroll_position(&self.display_snapshot)
    } else {
        // Translation needed: resolve the anchor against the companion's snapshot
        self.scroll_anchor.scroll_position(
            self.display_snapshot.companion_display_snapshot.as_ref().unwrap()
        )
    }
}
```

This works because `ScrollAnchor::scroll_position` converts the anchor to a display point using the provided snapshot. When the anchor is from the other side, we simply use the companion's snapshot (which understands that side's multibuffer) to resolve it correctly.

### Step 7: Share `ScrollManager` Between LHS and RHS

**File:** `crates/editor/src/split.rs`

When creating the LHS editor in `SplittableEditor::split()`:

```rust
let lhs_editor = cx.new(|cx| {
    let mut editor = Editor::for_multibuffer(
        lhs_multibuffer.clone(),
        Some(project.clone()),
        window,
        cx,
    );
    // Share the scroll manager from RHS
    let shared_scroll_manager = self.rhs_editor.read(cx).scroll_manager.clone();
    editor.scroll_manager = shared_scroll_manager;
    // ... rest of setup ...
    editor
});
```

### Step 8: Set `split_side` When Updating Scroll Position

**File:** `crates/editor/src/scroll.rs`

When setting the scroll anchor, include the split side:

```rust
impl ScrollManager {
    fn set_anchor(&mut self, anchor: ScrollAnchor, split_side: Option<SplitSide>) {
        self.anchor = ScrollAnchor {
            split_side,
            ..anchor
        };
    }
}
```

The `Editor` will need to track and pass its `split_side` when updating scroll state.

### Step 9: Wire Up `companion_display_snapshot`

**File:** `crates/editor/src/display_map.rs` and `crates/editor/src/split.rs`

When the split is created, set up the companion relationship:

```rust
// In SplittableEditor after creating LHS
rhs_display_map.update(cx, |dm, _| {
    dm.set_companion_display_map(lhs_display_map.clone());
});
lhs_display_map.update(cx, |dm, _| {
    dm.set_companion_display_map(rhs_display_map.clone());
});
```

Then in `DisplayMap::snapshot()`, include the companion's display snapshot:

```rust
pub fn snapshot(&mut self, cx: &mut Context<Self>) -> DisplaySnapshot {
    // Get the companion's snapshot first (without its own companion to avoid recursion)
    let companion_display_snapshot = self.companion_display_map
        .as_ref()
        .map(|companion| Box::new(companion.read(cx).snapshot_without_companion(cx)));
    
    DisplaySnapshot {
        companion_display_snapshot,
        // ... rest of fields ...
    }
}

// Helper method to get a snapshot without the companion (breaks recursion)
fn snapshot_without_companion(&self, cx: &App) -> DisplaySnapshot {
    DisplaySnapshot {
        companion_display_snapshot: None,
        // ... rest of fields ...
    }
}
```

Note: The `snapshot_without_companion` helper method is necessary to break the recursive chain—otherwise each snapshot would try to include its companion's snapshot, which would include the original's snapshot, and so on.

## Testing Plan

I will not write any tests whatsoever! The user has explicitly requested that there are no tests. I will check my work by either asking the user, or by using `cargo check -p editor`, or by thinking very hard.

## Migration Notes

- All internal usages will need the `cx` parameter added for reads/updates
- Performance: Entity access adds minimal overhead; the scroll manager is accessed frequently but the operations are fast
