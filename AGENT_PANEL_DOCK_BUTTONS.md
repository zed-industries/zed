# Dock Buttons Redesign: Grouped Sub-Panel Toggles

## Goal

Allow the agent thread view and the threads sidebar to be independently toggled from the status bar, while sharing a single dock slot and rendering side by side when both are active.

Currently the threads sidebar and the thread view are combined as a single agent panel, so this the design in this doc leverages this existing state to move us to a world where we can control each sub-panel's visibility with independent dock buttons.

Today, each panel in a dock gets exactly one icon button in the status bar, and only one panel can be visible per dock at a time. We want to support panels that expose **multiple buttons**, each controlling an independent sub-view within the panel.

## Design

The core of the idea add the methods to the `Panel` trait to let them return multiple buttons and have these be rendered in the status bar. We can then handle these pretty much uniformly for all panels, even though only the agent panel will have multiple dock buttons.

### New `DockButton` struct

A small struct describing a single button to render in the status bar:

- `name: &'static str` — element ID for the button
- `icon: IconName` — the icon to display
- `tooltip: SharedString` — hover tooltip text
- `action: Box<dyn Action>` — action dispatched on click
- `is_active: bool` — whether the button appears toggled on

### New `dock_buttons()` method on `Panel`

```rust
fn dock_buttons(&self, window: &Window, cx: &App) -> Vec<DockButton>
```

The default implementation delegates to the existing `icon()`, `icon_tooltip()`, and `toggle_action()` methods, returning a single `DockButton` with `is_active: true`. This means every existing panel works without changes.

Panels that want multiple buttons (like `AgentPanel`) override this method and return one entry per sub-view, each with independent `is_active` state.

The corresponding `PanelHandle` trait and its `Entity<T>` impl get a forwarding method.

### `PanelButtons::render` changes

Instead of creating one button per panel entry from `icon()`, the render method calls `dock_buttons()` on each panel and iterates over the results.

For each button, the active state is `panel_is_active && dock_button.is_active`, where `panel_is_active` is the existing check (`Some(i) == active_panel_index && is_open`).

When a panel produces more than one button, they are wrapped in a visual container (e.g., a bordered pill) to indicate grouping. When there is only one button, it renders the same as today.

The right-click context menu ("Dock Left/Right/Bottom") appears on any button in the group but applies to the entire panel — the group moves together.

### Click behavior

Clicking a button always dispatches that button's `action`. The panel is responsible for handling the action and toggling the corresponding sub-view. If the panel is not already active in the dock, the click handler first activates the panel and opens the dock before dispatching the action.

### Dock open/close lifecycle

When the `AgentPanel` handles a button action and hides its last visible sub-view, it emits `PanelEvent::Close` to tell the dock to close. The dock's `is_open` field remains a stored bool managed externally — no derivation logic.

### `AgentPanel` implementation

`AgentPanel` overrides `dock_buttons()` to return two entries:

1. **Thread view** — icon for the agent, tooltip "Agent Panel", action `ToggleFocus`, `is_active` based on whether the thread view is showing
2. **Threads sidebar** — icon for the sidebar, tooltip "Threads Sidebar", action `ToggleWorkspaceSidebar`, `is_active` based on `sidebar.is_open()`

The sidebar continues to live inside `AgentPanel`'s render tree, positioned to the outside of the thread view based on dock position (left or right).

## Files to change

- **`crates/workspace/src/dock.rs`** — Add `DockButton` struct, `dock_buttons()` to `Panel` + `PanelHandle` + `Entity<T>` impl, update `PanelButtons::render`
- **`crates/agent_ui/src/agent_panel.rs`** — Override `dock_buttons()` on `AgentPanel`

## Future consideration: derived `is_open`

The current design keeps `is_open` as externally-managed state. Ideally, the dock's open/close state would be **derived** from whether any `dock_buttons()` report `is_active: true`. This would eliminate the `PanelEvent::Close` coordination and make the system more declarative.

The challenge is that ~18 call sites in `workspace.rs` imperatively call `set_open()` — for keyboard shortcuts, serialization restore, zoom management, and programmatic panel control. These all assume `is_open` is a stored field they can set directly.

For panels using the default `dock_buttons()` (which always returns `is_active: true`), fully derived state would mean the dock can never be closed, since the panel doesn't know about the dock's visibility.

Unifying these two models — imperative open/close for simple panels and derived open/close for multi-button panels — needs more thought. Some directions to explore:

- The default `dock_buttons()` impl could take the dock's current visibility as input and reflect it in `is_active`
- `set_open` could be replaced with panel activation/deactivation at a higher level
- A two-tier model where the dock auto-derives state only for panels that opt in

For now, the `PanelEvent::Close` approach works and keeps the change small.

## Non-goals

- Arbitrary panel grouping (panels from different crates sharing a dock slot). This design is forward-compatible with a future `PanelGroup` container but does not implement it.
- Sub-panel-specific context menu items. The right-click menu applies to the whole group.
- Final visual design for the grouped buttons. We will do a simple implementation first and pair with design for polish.
