# GPUI Accessibility Design

## Overview

Add platform-level accessibility support to `crates/gpui` by integrating the `accesskit` crate ecosystem. This enables assistive technologies (VoiceOver, NVDA, Orca) and AI automation tools to query and interact with GPUI-built UIs.

GPUI is a primitive layer (analogous to HTML/DOM or React). It provides no built-in Button, Input, or other components — those are built by consumers (e.g., Zed's `crates/ui`). Therefore GPUI cannot auto-infer semantic roles; it exposes an annotation API that component authors call explicitly, exactly as HTML exposes `role` and `aria-*` attributes on `<div>`.

## Architecture

```
Upper layer (Zed ui crate, third-party GPUI users)
  div().role(Role::Button).aria_label("Submit")
                    │ annotations
GPUI Semantic Layer (new)
  Accessibility on Interactivity
  prepaint phase builds AccessibilityFrame
                    │ TreeUpdate
accesskit (third-party crate)
  Unified data model: Node, Role, NodeId, TreeUpdate
       │                 │                │
accesskit_macos   accesskit_windows   accesskit_unix
NSAccessibility   UI Automation       AT-SPI (D-Bus)
       │                 │                │
VoiceOver / AI    NVDA / AI          Orca / AI
```

Data flows in two directions:
1. **Render → platform**: each frame, `AccessibilityFrame` is pushed to the OS via AccessKit
2. **Platform → GPUI**: `ActionRequest` from AT or AI is routed back into GPUI events (click, focus, set value)

## New Cargo Dependencies

```toml
# crates/gpui/Cargo.toml
[dependencies]
accesskit = "0.24"

[target.'cfg(target_os = "macos")'.dependencies]
accesskit_macos = "0.26"

[target.'cfg(target_os = "windows")'.dependencies]
accesskit_windows = "0.32"

[target.'cfg(target_os = "linux")'.dependencies]
accesskit_unix = "0.21"
```

## GPUI Semantic Layer

### `Accessibility` struct

New file: `crates/gpui/src/accessibility.rs`

All fields map 1:1 to WAI-ARIA 1.1 attributes. `Role` and `Live` are re-exported from `accesskit` — no custom enums.

```rust
#[derive(Default, Clone)]
pub struct Accessibility {
    pub role: Option<Role>,                // role
    pub label: Option<SharedString>,       // aria-label
    pub description: Option<SharedString>, // aria-description
    pub checked: Option<bool>,             // aria-checked
    pub disabled: Option<bool>,            // aria-disabled
    pub expanded: Option<bool>,            // aria-expanded
    pub hidden: bool,                      // aria-hidden
    pub pressed: Option<bool>,             // aria-pressed
    pub readonly: Option<bool>,            // aria-readonly
    pub required: Option<bool>,            // aria-required
    pub selected: Option<bool>,            // aria-selected
    pub live: Option<Live>,                // aria-live: off | polite | assertive
}
```

### Changes to `Interactivity`

One new field in `crates/gpui/src/elements/div.rs`:

```rust
pub struct Interactivity {
    // ... all existing fields unchanged ...
    pub(crate) accessibility: Option<Box<Accessibility>>,
}
```

`Box` avoids heap allocation for the ~99% of elements with no accessibility annotation.

### Builder Methods on `InteractiveElement`

12 methods, all named after their WAI-ARIA counterparts using snake_case:

```rust
fn role(self, role: Role) -> Self;
fn aria_label(self, label: impl Into<SharedString>) -> Self;
fn aria_description(self, desc: impl Into<SharedString>) -> Self;
fn aria_checked(self, checked: bool) -> Self;
fn aria_disabled(self, disabled: bool) -> Self;
fn aria_expanded(self, expanded: bool) -> Self;
fn aria_hidden(self) -> Self;           // no argument: sets hidden = true
fn aria_pressed(self, pressed: bool) -> Self;
fn aria_readonly(self, readonly: bool) -> Self;
fn aria_required(self, required: bool) -> Self;
fn aria_selected(self, selected: bool) -> Self;
fn aria_live(self, live: Live) -> Self;
```

Each method lazily initialises `self.interactivity().accessibility` on first call:

```rust
fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
    self.interactivity()
        .accessibility
        .get_or_insert_with(Default::default)
        .label = Some(label.into());
    self
}
```

### Usage by upper-layer components

```rust
// Zed's Button (in crates/ui) — GPUI itself is unchanged
div()
    .role(Role::Button)
    .aria_label(self.label.clone())
    .aria_disabled(self.disabled)
    .track_focus(&focus_handle)
    .on_click(...)

// Progress bar
div()
    .role(Role::ProgressBar)
    .aria_label("Upload progress")

// Live status region
div()
    .role(Role::Status)
    .aria_live(Live::Polite)
    .child(self.status_message.clone())
```

## Paint Pipeline

### `AccessibilityFrame`

Built once per frame and discarded after pushing to the platform adapter.

```rust
pub(crate) struct AccessibilityFrame {
    pub nodes: Vec<(NodeId, Node)>,
    pub root_id: NodeId,
    pub focus: Option<NodeId>,
}
```

### `NodeId` assignment

- Elements with a `FocusHandle`: reuse `FocusId` (u64) cast to `NodeId` (NonZeroU128).
- Elements with accessibility props but no focus handle: allocate from a per-window atomic counter.

```rust
impl From<FocusId> for NodeId {
    fn from(id: FocusId) -> NodeId {
        NodeId(NonZeroU128::new(id.0 as u128).unwrap())
    }
}
```

### Collection during `prepaint`

`Interactivity::prepaint` gains a new block at its end:

```rust
if let Some(props) = &self.accessibility {
    let node_id = self.accessibility_node_id(window);
    let bounds = element_bounds.to_screen_rect(window);
    let node = props.to_accesskit_node(bounds, is_focused, is_disabled);
    window.push_accessibility_node(node_id, node, parent_node_id);
}
```

### Parent-child tracking

A `accessibility_node_stack: Vec<NodeId>` is maintained on `Window` (parallel to the existing paint depth tracking). Elements without accessibility props are transparent — they do not appear on the stack.

```
div (role=List)       → NodeId(1)  push
  div (role=ListItem) → NodeId(2)  push, parent=1
    div (role=Button) → NodeId(3)  push, parent=2
    exit              ← pop 3
  exit                ← pop 2
exit                  ← pop 1
```

### Frame commit

At the end of `Window::draw`, after scene submission:

```rust
let frame = self.take_accessibility_frame();
if let Some(adapter) = &self.accessibility_adapter {
    adapter.update(frame.into_tree_update());
}
```

## Platform Adapters

### Trait

```rust
// crates/gpui/src/platform.rs
pub(crate) trait AccessibilityAdapter {
    fn update(&self, update: TreeUpdate);
}
```

### Per-platform implementation

| Platform | File | Crate |
|----------|------|-------|
| macOS | `platform/mac/accessibility.rs` | `accesskit_macos` |
| Windows | `platform/windows/accessibility.rs` | `accesskit_windows` |
| Linux | `platform/linux/accessibility.rs` | `accesskit_unix` |

macOS is the highest-priority target. GPUI uses a native Cocoa backend (not winit) on macOS, so `accesskit_macos` is used directly:

```rust
// platform/mac/accessibility.rs
use accesskit_macos::Adapter;

pub(crate) struct MacAccessibilityAdapter {
    inner: Adapter,
}

impl MacAccessibilityAdapter {
    pub fn new(ns_view: id, action_handler: impl ActionHandler) -> Self {
        Self {
            inner: Adapter::new(ns_view, || initial_tree(), action_handler),
        }
    }
}

impl AccessibilityAdapter for MacAccessibilityAdapter {
    fn update(&self, update: TreeUpdate) {
        let events = self.inner.update_if_active(|| update);
        if let Some(events) = events {
            events.raise();
        }
    }
}
```

Windows and Linux use `accesskit_windows` and `accesskit_unix` respectively via a similar thin wrapper.

### `ActionRequest` routing (AT/AI → GPUI events)

When an assistive technology or AI agent sends an action, AccessKit fires a callback. The handler maps it back into GPUI's event system:

```rust
fn handle_action_request(request: ActionRequest, window: &mut Window, cx: &mut App) {
    match request.action {
        Action::Click => window.synthesize_click(request.target, cx),
        Action::Focus => {
            if let Some(handle) = window.focus_handle_for_node(request.target) {
                handle.focus(window);
            }
        }
        Action::SetValue => {
            if let Some(ActionData::Value(text)) = request.data {
                window.synthesize_text_input(request.target, text, cx);
            }
        }
        Action::ScrollIntoView => window.scroll_node_into_view(request.target, cx),
        _ => {}
    }
}
```

## File Changes Summary

| File | Change |
|------|--------|
| `crates/gpui/Cargo.toml` | Add `accesskit`, `accesskit_macos`, `accesskit_windows`, `accesskit_unix` |
| `crates/gpui/src/accessibility.rs` | New file: `Accessibility`, `AccessibilityFrame`, `AccessibilityAdapter` trait |
| `crates/gpui/src/elements/div.rs` | Add `accessibility: Option<Box<Accessibility>>` to `Interactivity`; add 12 builder methods to `InteractiveElement` |
| `crates/gpui/src/window.rs` | Add `accessibility_adapter`, `accessibility_node_stack`; update `draw` to push `TreeUpdate` |
| `crates/gpui/src/platform/mac/accessibility.rs` | New file: `MacAccessibilityAdapter` wrapping `accesskit_macos::Adapter` |
| `crates/gpui/src/platform/windows/accessibility.rs` | New file: Windows adapter |
| `crates/gpui/src/platform/linux/accessibility.rs` | New file: Linux adapter |
| `crates/gpui/src/lib.rs` | Re-export `Role`, `Live` from `accesskit` |
| `crates/gpui/examples/accessibility.rs` | New example: labeled counter + checkboxes + live region; live tree text panel showing `accessibility_tree()` output |

## Debug API: Accessibility Tree Dump

Mirrors GPUI's existing inspector pattern — gated on `#[cfg(any(feature = "inspector", debug_assertions))]`, zero overhead in release builds.

### API

```rust
// crates/gpui/src/window.rs

pub struct AccessibilityTree(String);

impl std::fmt::Display for AccessibilityTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(any(feature = "inspector", debug_assertions))]
impl Window {
    /// Returns a snapshot of the current accessibility tree as formatted text.
    pub fn accessibility_tree(&self) -> AccessibilityTree { ... }
}
```

Called from application code or tests at any point after a frame has been drawn:

```rust
println!("{}", window.accessibility_tree());
```

### Output format

Indented text tree, one node per line, modeled on how macOS Accessibility Inspector presents its hierarchy:

```
[accessibility tree]
  Button          "Increment counter"  enabled=true   (160, 60, 120×32)
  CheckBox        "Option A"           checked=true   (40, 110, 80×20)
  CheckBox        "Option B"           checked=false  (140, 110, 80×20)
  StaticText      "Counter value"      value="3"      (40, 60, 100×32)
  Status          live=polite                         (40, 150, 400×20)
    StaticText    "Counter updated to 3"              (40, 150, 400×20)
```

Each line: `role  label/value  state-flags  (x, y, w×h)`

### Implementation

`Window::accessibility_tree` walks the last built `AccessibilityFrame` and formats each node using `accesskit_consumer::Tree` for traversal order:

```rust
#[cfg(any(feature = "inspector", debug_assertions))]
pub fn accessibility_tree(&self) -> String {
    let Some(frame) = &self.last_accessibility_frame else {
        return String::from("[no accessibility frame]");
    };
    let tree = accesskit_consumer::Tree::new(frame.to_tree_update(), false);
    let mut out = String::from("[accessibility tree]\n");
    format_node(&tree, tree.root(), 1, &mut out);
    out
}
```

`Window` stores `last_accessibility_frame: Option<AccessibilityFrame>` only in debug builds (same pattern as `debug_selector` on `Interactivity`).

## Verification Example

New file: `crates/gpui/examples/accessibility.rs`

The example renders a small UI with clearly labeled roles and states, then provides two verification paths:

### What the example renders

The window is split into two panels. The left panel contains interactive UI; the right panel displays the live output of `window.accessibility_tree()`, updated on every state change.

```
┌──────────────────┬─────────────────────────────────────────┐
│                  │ [accessibility tree]                     │
│  Counter: 3      │   Button  "Increment counter"            │
│  [Increment]     │     enabled=true  (160, 60, 120×32)      │
│                  │   CheckBox  "Option A"                   │
│  ☑ Option A      │     checked=true  (40, 110, 80×20)       │
│  ☐ Option B      │   CheckBox  "Option B"                   │
│                  │     checked=false (140, 110, 80×20)      │
│                  │   Status  live=polite                    │
│                  │     "Counter updated to 3"               │
└──────────────────┴─────────────────────────────────────────┘
```

The right panel is simply a `div` rendering `window.accessibility_tree()` as a `SharedString`. No external tool needed — the tree is self-documenting inside the running app.

Elements and their accessibility annotations:

| Element | role | aria_label | other |
|---------|------|------------|-------|
| Counter display | `StaticText` | `"Counter value"` | — |
| Increment button | `Button` | `"Increment counter"` | `aria_disabled` when count ≥ 10 |
| Checkbox A | `CheckBox` | `"Option A"` | `aria_checked` |
| Checkbox B | `CheckBox` | `"Option B"` | `aria_checked` |
| Status div | `Status` | — | `aria_live(Live::Polite)` |

### Verification

Interacting with the left panel (click Increment, toggle checkboxes) causes the right panel tree text to update in real time. This proves:
1. Accessibility annotations are correctly attached to elements
2. State changes (`checked`, `disabled`, label text) propagate into the tree each frame
3. `window.accessibility_tree()` is a usable API for inspecting any GPUI window

For platform-level validation, open **Xcode Accessibility Inspector** and point it at the example window — the AX hierarchy should match the text panel exactly.

## WAI-ARIA Compliance Notes

### Accessible Name Computation (ACCNAME 1.2)

Per [ACCNAME 1.2](https://www.w3.org/TR/accname-1.2/), an element's accessible name is determined in this priority order:

1. `aria-label` (our `label` field) — explicit override, always wins
2. Text content of child elements — used when no explicit label is set

The paint pipeline **must** implement fallback (2): when `label` is `None`, collect the concatenated text content of all descendant `StaticText` nodes and use it as the AccessKit node's name. Without this, a button with only `.child("Submit")` would have no accessible name — which breaks screen readers.

```rust
// In Accessibility::to_accesskit_node()
let name = self.label.clone().or_else(|| collect_child_text(children));
```

### `aria-description` vs `aria-describedby`

The `description` field maps to `aria-description`, which is a WAI-ARIA 1.3 draft attribute. Current AT support is limited. Component authors who need broad AT support today should combine an explicit `aria_label` with a visually hidden companion element. This is a known limitation of the initial implementation; `aria-describedby` (cross-element reference) is out of scope for v1.

### Required Context Roles

WAI-ARIA defines roles that are only meaningful inside a specific parent role:

| Role | Required parent |
|------|----------------|
| `ListItem` | `List` |
| `Option` | `ListBox` |
| `Tab` | `TabList` |
| `Row` | `Grid`, `RowGroup`, `Table`, or `TreeGrid` |
| `TreeItem` | `Tree` or `Group` |

GPUI does not enforce these constraints (nor does AccessKit). Component authors **must** follow them; violations cause AT to ignore or misrepresent the affected nodes. This should be documented in the GPUI accessibility guide.

## Out of Scope

- Value range attributes (`aria-valuemin/max/now/text`) — add when building slider/progress components
- `aria-sort`, `aria-haspopup`, `aria-modal`, `aria-orientation` — add per-component need
- Web/WASM platform — AccessKit has no web backend; browser handles accessibility natively
- Automatic role inference for built-in components — GPUI has no built-in components

## Release Notes

- N/A (infrastructure only; no user-visible change until upper-layer components annotate themselves)
