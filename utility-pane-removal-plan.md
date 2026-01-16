# Utility Pane Removal Plan

## Overview

The utility pane is a secondary pane system that sits alongside the main editor panes. It allows panels (specifically `AgentThreadPane`) to "pop out" a detail view next to the center editor area. It supports two slots:

- **Left slot**: Adjacent to the left dock
- **Right slot**: Adjacent to the right dock

According to `threads-sidebar-plan.md`, this removal is part of **Phase 3** of the MultiWorkspace Agent V2 Refactor. The utility pane concept is being replaced by an "Agents Sidebar" that will live outside the workspace/docks structure.

---

## Files to Modify/Delete

### 1. Core Infrastructure (workspace crate)

- `crates/workspace/src/utility_pane.rs` — **DELETE**
  - Contains all utility pane state, types, and impl methods on Workspace

- `crates/workspace/src/dock.rs` — **EDIT**
  - Remove `UtilityPane`, `UtilityPaneHandle`, `UtilityPanePosition`, `MinimizePane`, `ClosePane`

- `crates/workspace/src/workspace.rs` — **EDIT**
  - Remove: `utility_panes` field, imports, `clamp_utility_pane_widths()`, `max_utility_pane_width()`, `UtilityPaneFrame` renders in `fn render()`

- `crates/workspace/src/pane.rs` — **EDIT**
  - Remove utility pane toggle buttons from tab bar rendering

### 2. Agent UI V2 (consumer of utility panes)

- `crates/agent_ui_v2/src/agent_thread_pane.rs` — **DELETE**
  - The only implementer of `UtilityPane` trait

- `crates/agent_ui_v2/src/agents_panel.rs` — **EDIT**
  - Remove all utility pane management (`register_utility_pane`, `restore_utility_pane`, subscriptions, etc.)

### 3. Settings

- `crates/settings/src/settings_content/agent.rs` — **EDIT**
  - Remove "utility pane" dock setting comment/option

---

## Types and Traits to Remove

### From `crates/workspace/src/dock.rs`

- `MinimizePane` (struct)
- `ClosePane` (struct)
- `UtilityPane` (trait)
- `UtilityPaneHandle` (trait)
- `UtilityPanePosition` (enum)

### From `crates/workspace/src/utility_pane.rs` (entire file)

- `UtilityPaneSlot` (enum)
- `UtilityPaneSlotState` (struct)
- `UtilityPaneState` (struct)
- `DraggedUtilityPane` (struct)
- `UtilityPaneFrame` (struct)
- `utility_slot_for_dock_position()` (function)
- `UTILITY_PANE_RESIZE_HANDLE_SIZE` (const)
- `UTILITY_PANE_MIN_WIDTH` (const)
- All `impl Workspace` methods for utility panes

---

## Methods on `Workspace` to Remove

- `utility_pane()` — Get a utility pane by slot
- `toggle_utility_pane()` — Toggle expanded state
- `register_utility_pane()` — Register a new utility pane
- `clear_utility_pane()` — Clear a utility pane slot
- `clear_utility_pane_if_provider()` — Conditionally clear if owned by panel
- `resize_utility_pane()` — Resize a utility pane
- `reset_utility_pane_width()` — Reset to default width
- `clamp_utility_pane_widths()` — Clamp widths when docks resize
- `max_utility_pane_width()` — Calculate max allowed width

---

## Render Changes in `workspace.rs`

The `render()` method has **8 places** where `UtilityPaneFrame::new()` is conditionally rendered based on dock positions. All of these need to be removed:

- Line ~7244: `UtilityPaneFrame::new(UtilityPaneSlot::Left, ...)`
- Line ~7294: `UtilityPaneFrame::new(UtilityPaneSlot::Right, ...)`
- Line ~7333: `UtilityPaneFrame::new(UtilityPaneSlot::Left, ...)`
- Line ~7368: `UtilityPaneFrame::new(UtilityPaneSlot::Right, ...)`
- Line ~7400: `UtilityPaneFrame::new(UtilityPaneSlot::Left, ...)`
- Line ~7447: `UtilityPaneFrame::new(UtilityPaneSlot::Right, ...)`
- Line ~7474: `UtilityPaneFrame::new(UtilityPaneSlot::Left, ...)`
- Line ~7519: `UtilityPaneFrame::new(UtilityPaneSlot::Right, ...)`

Also remove:
- The `DragMoveEvent<DraggedUtilityPane>` handler (~line 7195)
- The `DraggedUtilityPane` import

---

## Implementation Order

### Phase 1: Remove Consumer (agent_ui_v2)

1. Delete `crates/agent_ui_v2/src/agent_thread_pane.rs`
2. Edit `crates/agent_ui_v2/src/agents_panel.rs`:
   - Remove imports from `workspace::dock` and `workspace::utility_pane`
   - Remove `agent_thread_pane` field and related fields
   - Remove `restore_utility_pane()` method
   - Remove `handle_utility_pane_event()` method
   - Remove `handle_close_pane_event()` method
   - Remove utility pane registration in `open_thread()`
   - Remove `utility_slot()` method
   - Update `serialize()` to not include pane state

### Phase 2: Remove Infrastructure (workspace)

1. Delete `crates/workspace/src/utility_pane.rs`
2. Edit `crates/workspace/src/dock.rs`:
   - Remove `MinimizePane`, `ClosePane` structs
   - Remove `UtilityPane` trait
   - Remove `UtilityPaneHandle` trait and impl
   - Remove `UtilityPanePosition` enum
3. Edit `crates/workspace/src/workspace.rs`:
   - Remove `pub mod utility_pane;` declaration
   - Remove utility_pane imports
   - Remove `utility_panes: UtilityPaneState` field from `Workspace` struct
   - Remove initialization in `Workspace::new()`
   - Remove `clamp_utility_pane_widths()` calls in dock resize methods
   - Remove `max_utility_pane_width()` method
   - Remove `clamp_utility_pane_widths()` method
   - Remove all `UtilityPaneFrame` renders in `fn render()`
   - Remove `DragMoveEvent<DraggedUtilityPane>` handler
   - Remove `clear_utility_pane_if_provider()` call in `remove_panel()`
4. Edit `crates/workspace/src/pane.rs`:
   - Remove `utility_pane::UtilityPaneSlot` import
   - Remove `open_aside_left` rendering block in `render_tab_bar()`
   - Remove `open_aside_right` rendering block in `render_tab_bar()`

### Phase 3: Clean Up Settings

1. Edit `crates/settings/src/settings_content/agent.rs`:
   - Remove or update the "utility pane" dock setting documentation

---

## Testing

After removal:
1. Run `cargo check -p workspace -p agent_ui_v2`
2. Run `./script/clippy`
3. Verify the workspace renders correctly without utility panes
4. Verify the agents panel still functions (without the thread pane popup)