# Floating Island Layout & Rounded Corners Design Specification

## Overview

This specification details the design and architecture for implementing a **Floating Island Layout** with configurable rounded corners in Zed Code Editor. The feature transforms docked panels (editor panes, sidebars/project panel, terminal, and status bar) into cohesive, floating cards separated by customizable gaps over a textured background canvas, accompanied by rounded pill-style editor tabs.

The entire system is configurable via `settings.json`, allowing users to toggle floating island mode on/off and adjust corner radii, gaps, and borders dynamically.

---

## 1. Configuration & Settings Schema

### 1.1 Settings Content (`crates/settings_content/src/workspace.rs`)

We introduce `IslandLayoutSettings` in `WorkspaceSettingsContent`:

```rust
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct IslandLayoutSettings {
    /// Whether floating island layout mode is enabled.
    ///
    /// Default: false
    pub enabled: Option<bool>,

    /// Corner radius in pixels for floating panels and cards.
    ///
    /// Default: 18.0
    pub corner_radius: Option<f32>,

    /// Gap / padding around floating islands in pixels.
    ///
    /// Default: 6.0
    pub gap: Option<f32>,

    /// Whether to render a subtle border outline around floating panels.
    ///
    /// Default: true
    pub border: Option<bool>,

    /// Whether to display editor tabs as rounded pills inside the pane header.
    ///
    /// Default: true
    pub pill_tabs: Option<bool>,
}
```

### 1.2 Runtime Workspace Settings (`crates/workspace/src/workspace_settings.rs`)

```rust
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IslandLayoutSettings {
    pub enabled: bool,
    pub corner_radius: f32,
    pub gap: f32,
    pub border: bool,
    pub pill_tabs: bool,
}

impl Default for IslandLayoutSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            corner_radius: 18.0,
            gap: 6.0,
            border: true,
            pill_tabs: true,
        }
    }
}
```

### 1.3 `settings.json` Example

```jsonc
{
  "island_layout": {
    "enabled": true,
    "corner_radius": 18.0,
    "gap": 6.0,
    "border": true,
    "pill_tabs": true
  }
}
```

---

## 2. Workspace Layout & Canvas Architecture

### 2.1 Workspace Canvas Backdrop (`crates/workspace/src/workspace.rs`)

* In `Workspace::render`, the root `#workspace` container acts as the canvas.
* When `island_layout.enabled` is `true`:
  * Root workspace surface background is tinted with `cx.theme().colors().window_background` (or theme canvas background) to provide visual depth behind floating cards.
  * A layout margin (`gap` px) is added around the entire perimeter of the workspace region.
  * Flex children (`left_dock`, `center` editor group, `right_dock`, `bottom_dock`, `status_bar`) receive uniform gap spacing (`gap` px) instead of touching each other.

### 2.2 Dock Transformation (`crates/workspace/src/dock.rs`)

* In `Dock::render`:
  * When island layout is active, each visible dock panel (`#dock-panel`) is rendered as a floating card:
    * `.rounded(px(corner_radius))`
    * `.overflow_hidden()`
    * `.when(border, |this| this.border_1().border_color(cx.theme().colors().border_variant.or(cx.theme().colors().border)))`
    * `.shadow_sm()`
  * Dock resize handles (`#resize-handle`) remain interactive within the gap region between the dock and center panes.

---

## 3. Floating Editor Panes & Split Views

### 3.1 Pane Cards (`crates/workspace/src/pane_group.rs` & `crates/workspace/src/pane.rs`)

* In `Pane::render`:
  * Main pane container applies `.rounded(px(corner_radius))` and `.overflow_hidden()`.
  * Text buffers, line numbers gutter, and minimap clip cleanly inside the card without overflowing the rounded corners.
  * In split pane layouts (horizontal and vertical splits), each split pane rendered in `Member::render` receives `gap` padding and individual card rounding.
  * Active pane borders/highlights follow the rounded corner geometry.

### 3.2 Pill Tabs (`crates/ui/src/components/tab.rs` & `tab_bar.rs`)

* When `pill_tabs` is enabled:
  * Tab bar container has internal padding (`py_1 px_1.5`) and transparent background.
  * Inactive tabs render with `.rounded_md()` (8px), muted typography, and smooth hover backgrounds.
  * Active tab renders with `.rounded_md()` (8px), matching the editor card surface background and active text color with subtle border/shadow.
  * Close icon buttons and tab action slots feature circular hover states.

---

## 4. Status Bar & Interactive Visuals

### 4.1 Floating Status Bar (`crates/workspace/src/workspace.rs` & status bar integration)

* When island layout is active:
  * Status bar renders as a floating pill container at the bottom with `.rounded(px(corner_radius * 0.75))` (or pill), `.mx_2 mb_2`, `.border_1()`, and `.shadow_sm()`.
  * Status entries (branch name, cursor line/col, diagnostics, language indicator) remain fully functional with balanced padding.

### 4.2 Drag-and-Drop Feedback (`crates/workspace/src/pane.rs`)

* Tab and split drag-and-drop target overlays (`DraggedTab`, `DraggedSelection`, `ExternalPaths`) inherit the parent pane's `.rounded(px(corner_radius))`.

---

## 5. Non-Goals & Out of Scope

* Native OS window frame alterations (handled separately by native OS DWM / Mica / titlebar settings).
* Modifying third-party extension renderers directly (extensions render standard GPUI views which automatically inherit theme colors).

---

## 6. Verification & Testing Plan

1. **Compilation & Unit Tests:**
   * Validate `settings_content` serialization and deserialization tests.
   * Run workspace and ui tests (`cargo test -p workspace -p ui -p settings`).
2. **Visual & Interactive Verification:**
   * Launch Zed (`cargo run`) with default settings (confirm vanilla appearance is 100% unaffected).
   * Enable `"island_layout": { "enabled": true }` in `settings.json` and verify live-reload.
   * Test split panes (horizontal & vertical splits) to ensure gaps and corner clipping are uniform.
   * Test left/right/bottom docks opening, collapsing, and resizing.
   * Test tab dragging, dropping, opening, closing, and switching.
   * Test zooming in/out of panes (`cmd+shift+enter` / `ctrl+shift+enter`).
