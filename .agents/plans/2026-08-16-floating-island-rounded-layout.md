# Floating Island Layout & Rounded Corners Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a fully configurable Floating Island Layout with rounded corners for editor panes, side/bottom docks, pill tabs, and status bar in Zed Code Editor.

**Architecture:** Add `IslandLayoutSettings` to `WorkspaceSettingsContent` and `WorkspaceSettings` for schema-backed user configuration via `settings.json`. Update `Workspace::render`, `Dock::render`, `Pane::render`, and `Tab::render` to apply rounded corners, overflow clipping, outer canvas depth, and inter-panel gaps when island mode is enabled while maintaining pixel-perfect fidelity with vanilla Zed when disabled.

**Tech Stack:** Rust, GPUI, GPUI Styled trait (`gpui_macros`), Serde, JSON Schema.

## Global Constraints

- Rust coding guidelines: Follow `.rules` (no `unwrap()`, prefer `?`, no `mod.rs`, single foreground thread for GPUI entities).
- Zero visual regression when `island_layout.enabled` is `false`.
- Git commit message format: `type(scope): short description` followed by bullet points.

---

### Task 1: Configuration & Settings Schema

**Files:**
- Modify: `crates/settings_content/src/workspace.rs`
- Modify: `crates/workspace/src/workspace_settings.rs`
- Test: `crates/settings/src/settings_store.rs`

**Interfaces:**
- Produces: `IslandLayoutSettings` struct with fields:
  ```rust
  #[derive(Copy, Clone, PartialEq, Debug)]
  pub struct IslandLayoutSettings {
      pub enabled: bool,
      pub corner_radius: f32,
      pub gap: f32,
      pub border: bool,
      pub pill_tabs: bool,
  }
  ```
- Consumes: `SettingsContent`, `WorkspaceSettings::get_global(cx)`.

- [ ] **Step 1: Write unit test for IslandLayoutSettings deserialization**

In `crates/settings_content/src/workspace.rs` (or settings tests):
```rust
#[test]
fn test_island_layout_settings_parsing() {
    let json = serde_json::json!({
        "island_layout": {
            "enabled": true,
            "corner_radius": 20.0,
            "gap": 8.0,
            "border": true,
            "pill_tabs": true
        }
    });
    let parsed: WorkspaceSettingsContent = serde_json::from_value(json).expect("failed to parse");
    let island = parsed.island_layout.expect("island_layout was None");
    assert_eq!(island.enabled, Some(true));
    assert_eq!(island.corner_radius, Some(20.0));
    assert_eq!(island.gap, Some(8.0));
    assert_eq!(island.border, Some(true));
    assert_eq!(island.pill_tabs, Some(true));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p settings_content test_island_layout_settings_parsing`
Expected: FAIL with `unknown field island_layout`

- [ ] **Step 3: Implement IslandLayoutSettings in `settings_content` and `workspace_settings`**

In `crates/settings_content/src/workspace.rs`:
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

Add `pub island_layout: Option<IslandLayoutSettings>` to `WorkspaceSettingsContent`.

In `crates/workspace/src/workspace_settings.rs`:
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
Add `pub island_layout: IslandLayoutSettings` to `WorkspaceSettings` and map it in `impl Settings for WorkspaceSettings`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p settings_content -p workspace`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/settings_content/src/workspace.rs crates/workspace/src/workspace_settings.rs
git commit -m "feat(settings): add island_layout configuration options

- Define IslandLayoutSettings in WorkspaceSettingsContent
- Add runtime IslandLayoutSettings to WorkspaceSettings with default values"
```

---

### Task 2: Floating Pill Tabs Styling

**Files:**
- Modify: `crates/ui/src/components/tab.rs`
- Modify: `crates/ui/src/components/tab_bar.rs`
- Test: `crates/ui/src/components/tab.rs`

**Interfaces:**
- Produces: `Tab::pill_style(mut self, pill: bool) -> Self` and `TabBar::pill_style(mut self, pill: bool) -> Self`.
- Consumes: `TabPosition`, `ThemeColors`.

- [ ] **Step 1: Write test for Tab component pill styling**

In `crates/ui/src/components/tab.rs`:
```rust
#[gpui::test]
async fn test_tab_pill_style(cx: &mut gpui::TestAppContext) {
    let window = cx.add_window(|_| {
        Tab::new("test_tab")
            .pill_style(true)
            .selected(true)
            .child("test.rs")
    });
    assert!(window.is_ok());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p ui test_tab_pill_style`
Expected: FAIL with `no method named pill_style`

- [ ] **Step 3: Implement pill style in Tab and TabBar**

In `crates/ui/src/components/tab.rs`:
Add `pill_style: bool` field to `Tab` with builder method:
```rust
pub fn pill_style(mut self, pill: bool) -> Self {
    self.pill_style = pill;
    self
}
```
In `impl RenderOnce for Tab`:
When `self.pill_style` is true:
- Apply `.rounded_md().mx_0p5().my_1().px_2()`
- If `self.selected`: `.bg(tab_bg).border_1().border_color(cx.theme().colors().border_variant)`
- If not `selected`: `.bg(gpui::transparent_black()).hover(|s| s.bg(tab_hover_bg))` without bottom/side border attachments.

In `crates/ui/src/components/tab_bar.rs`:
Add `pill_style: bool` and pass it down or adjust tab bar container padding (`py_0p5 px_1`).

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p ui test_tab_pill_style`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/ui/src/components/tab.rs crates/ui/src/components/tab_bar.rs
git commit -m "feat(ui): add pill style support to Tab and TabBar components

- Add pill_style option to Tab and TabBar
- Render rounded pill chips with proper active/inactive contrast in pill mode"
```

---

### Task 3: Floating Dock Panels & Resize Handles

**Files:**
- Modify: `crates/workspace/src/dock.rs`
- Test: `crates/workspace/src/dock.rs`

**Interfaces:**
- Consumes: `WorkspaceSettings::get_global(cx).island_layout`.
- Produces: Rounded dock card rendering and adjusted resize handle placement.

- [ ] **Step 1: Write test for Dock island mode rendering**

In `crates/workspace/src/dock.rs`:
```rust
#[gpui::test]
async fn test_dock_island_rendering(cx: &mut gpui::TestAppContext) {
    // Verify dock renders properly with island settings active
}
```

- [ ] **Step 2: Run test to verify initial state**

Run: `cargo test -p workspace test_dock_island_rendering`

- [ ] **Step 3: Update `Dock::render` for Island Mode**

In `crates/workspace/src/dock.rs`:
Read `let island_layout = WorkspaceSettings::get_global(cx).island_layout;`
When `island_layout.enabled`:
```rust
.when(island_layout.enabled, |this| {
    this.rounded(px(island_layout.corner_radius))
        .overflow_hidden()
        .shadow_sm()
        .when(island_layout.border, |this| {
            this.border_1().border_color(cx.theme().colors().border)
        })
})
```
Adjust resize handle margins and padding so handles lie within the inter-card gap.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p workspace`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/workspace/src/dock.rs
git commit -m "feat(workspace): style dock panels as floating cards in island layout

- Apply corner radius, overflow clipping, and borders to dock cards
- Position resize handles within layout gap"
```

---

### Task 4: Floating Editor Panes & Split Views

**Files:**
- Modify: `crates/workspace/src/pane.rs`
- Modify: `crates/workspace/src/pane_group.rs`
- Test: `crates/workspace/src/pane.rs`

**Interfaces:**
- Consumes: `WorkspaceSettings::get_global(cx).island_layout`.
- Produces: Rounded pane containers, clipped buffers/gutters, and rounded drag-and-drop target overlays.

- [ ] **Step 1: Write test for Pane island mode rendering**

In `crates/workspace/src/pane.rs`:
```rust
#[gpui::test]
async fn test_pane_island_rendering(cx: &mut gpui::TestAppContext) {
    // Verify pane renders with rounded container and pill tabs
}
```

- [ ] **Step 2: Run test to verify initial state**

Run: `cargo test -p workspace test_pane_island_rendering`

- [ ] **Step 3: Implement Pane and PaneGroup island card styling**

In `crates/workspace/src/pane_group.rs` (`Member::render`):
When `island_layout.enabled`:
- Add `p(px(island_layout.gap / 2.0))` around pane members in split axes so splits have clean gap spacing.

In `crates/workspace/src/pane.rs` (`Pane::render`):
- Read `let island_layout = WorkspaceSettings::get_global(cx).island_layout;`
- Apply `.when(island_layout.enabled, |this| this.rounded(px(island_layout.corner_radius)).overflow_hidden().when(island_layout.border, |t| t.border_1().border_color(cx.theme().colors().border)))` to pane card container.
- Pass `island_layout.pill_tabs` to `render_tab_bar`.
- In drag target overlay: apply `.rounded(px(island_layout.corner_radius))` so the drop indicator follows the rounded boundary.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p workspace`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/workspace/src/pane.rs crates/workspace/src/pane_group.rs
git commit -m "feat(workspace): add floating card styling and split gaps to editor panes

- Apply corner radius and overflow clipping to editor panes
- Add split pane gap spacing in pane_group
- Render rounded drag and drop indicators"
```

---

### Task 5: Workspace Canvas Backdrop & Floating Status Bar

**Files:**
- Modify: `crates/workspace/src/workspace.rs`
- Test: `crates/workspace/src/workspace.rs`

**Interfaces:**
- Consumes: `WorkspaceSettings::get_global(cx).island_layout`.
- Produces: Complete floating island workspace layout with canvas backdrop, uniform gap grid, and floating status bar pill.

- [ ] **Step 1: Write test for Workspace island layout rendering**

In `crates/workspace/src/workspace.rs`:
```rust
#[gpui::test]
async fn test_workspace_island_layout(cx: &mut gpui::TestAppContext) {
    // Verify workspace layout builds with island layout enabled
}
```

- [ ] **Step 2: Run test to verify initial state**

Run: `cargo test -p workspace test_workspace_island_layout`

- [ ] **Step 3: Implement canvas backdrop, grid gaps, and floating status bar**

In `crates/workspace/src/workspace.rs` (`Workspace::render`):
- Read `let island_layout = WorkspaceSettings::get_global(cx).island_layout;`
- When `island_layout.enabled`:
  - Set `#workspace` background to canvas / window background: `.bg(colors.window_background.unwrap_or(colors.background))` and outer padding `.p(px(island_layout.gap))`.
  - Add gap spacing (`gap(px(island_layout.gap))`) between `left_dock`, center editor group, `right_dock`, and `bottom_dock`.
  - Style status bar as floating pill: `.rounded(px(island_layout.corner_radius * 0.75)).mx_2().mb_1().border_1().border_color(colors.border).shadow_sm()`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p workspace`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/workspace/src/workspace.rs
git commit -m "feat(workspace): implement canvas backdrop, panel gaps, and floating status bar

- Add canvas backdrop and perimeter gaps to workspace grid
- Render status bar as floating pill card
- Complete end-to-end floating island layout"
```

---

### Task 6: End-to-End Verification & Formatting

**Files:**
- All touched crates

- [ ] **Step 1: Run full test suite for workspace, ui, and settings**

Run: `cargo test -p workspace -p ui -p settings_content -p settings`
Expected: All tests PASS.

- [ ] **Step 2: Run clippy and format check**

Run: `./script/clippy`
Expected: Zero errors / warnings on touched files.

- [ ] **Step 3: Final verification commit if needed**

```bash
git commit -m "chore(ui): finalize floating island layout verification"
```
