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

### Step 3: Add `display_map_id` to `ScrollAnchor`

**File:** `crates/editor/src/scroll.rs`

Update `ScrollAnchor` to track which display map it originated from:

```rust
pub struct ScrollAnchor {
    pub offset: gpui::Point<ScrollOffset>,
    pub anchor: Anchor,
    /// The EntityId of the DisplayMap this anchor was created from.
    /// Used to determine if translation is needed when resolving the anchor
    /// in a split diff view where LHS and RHS have different display maps.
    pub display_map_id: EntityId,
}
```

Update `ScrollAnchor::new()` to take a `display_map_id: EntityId` parameter.

### Step 4: Track `display_map_id` in `EditorSnapshot`

**File:** `crates/editor/src/editor.rs`

The `EditorSnapshot` already contains a `DisplaySnapshot`, and `DisplaySnapshot` has access to its `DisplayMap`'s `EntityId`. We can use this to compare against the `ScrollAnchor`'s `display_map_id` to determine if translation is needed.

Ensure `DisplaySnapshot` exposes its `EntityId` (if not already available):

```rust
impl DisplaySnapshot {
    pub fn display_map_id(&self) -> EntityId {
        self.display_map_id
    }
}
```

This may require storing the `EntityId` in `DisplaySnapshot` when it's created from `DisplayMap::snapshot()`.

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
    let anchor_display_map_id = self.scroll_anchor.display_map_id;
    let our_display_map_id = self.display_snapshot.display_map_id();

    // If the anchor's display_map_id matches ours, use direct calculation
    if anchor_display_map_id == our_display_map_id {
        self.scroll_anchor.scroll_position(&self.display_snapshot)
    } else {
        // Translation needed: resolve the anchor against the companion's snapshot
        self.scroll_anchor.scroll_position(
            self.display_snapshot.companion_display_snapshot.as_ref().unwrap()
        )
    }
}
```

This works because `ScrollAnchor::scroll_position` converts the anchor to a display point using the provided snapshot. When the anchor is from a different display map, we use the companion's snapshot (which understands that side's multibuffer) to resolve it correctly.

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

### Step 8: Set `display_map_id` When Updating Scroll Position

**File:** `crates/editor/src/scroll.rs`

When setting the scroll anchor, include the display map's EntityId:

```rust
impl ScrollManager {
    fn set_anchor(&mut self, anchor: ScrollAnchor, display_map_id: EntityId) {
        self.anchor = ScrollAnchor {
            display_map_id,
            ..anchor
        };
    }
}
```

The `Editor` will pass its `display_map.entity_id()` when updating scroll state. This naturally identifies which side of a split the anchor belongs to without needing a separate enum.

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
