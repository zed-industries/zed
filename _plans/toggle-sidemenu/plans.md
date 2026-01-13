# Toggle Tab Bar Position (Top vs Right Side)

## Feature Overview

Add a setting to switch between horizontal tabs at the top of panes and vertical tabs on the right side of panes (similar to VS Code's vertical tab feature).

## User Setting

```json
{
  "tab_bar": {
    "position": "top",      // "top" (default) or "right"
    "default_width": 200    // Width of side tab bar in pixels (only applies when position is "right")
  }
}
```

---

## Architecture Overview

### Current Tab Bar System

The tab bar in Zed is:
- Rendered inside each `Pane` via `Pane::render_tab_bar()` method
- Uses `TabBar` component from `crates/ui/src/components/tab_bar.rs`
- Individual tabs use `Tab` component from `crates/ui/src/components/tab.rs`
- Position is fixed at the top of the pane content
- Settings are in `TabBarSettings` struct in `crates/workspace/src/workspace_settings.rs`

### Key Files

| File | Purpose |
|------|---------|
| `crates/settings/src/settings_content/workspace.rs` | Settings content definitions (JSON schema) |
| `crates/workspace/src/workspace_settings.rs` | Runtime settings structs |
| `crates/workspace/src/pane.rs` | Pane rendering, tab management (~4200 lines) |
| `crates/ui/src/components/tab_bar.rs` | Horizontal TabBar UI component |
| `crates/ui/src/components/tab.rs` | Individual Tab UI component |
| `crates/workspace/src/dock.rs` | Dock/panel system (reference for resize handles) |

---

## Detailed Implementation Plan

### Phase 1: Settings Infrastructure

#### 1.1 Add TabBarPosition Enum

**File: `crates/settings/src/settings_content/workspace.rs`**

Add after existing enums (around line 416):

```rust
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum TabBarPosition {
    #[default]
    Top,
    Right,
}
```

#### 1.2 Update TabBarSettingsContent

**File: `crates/settings/src/settings_content/workspace.rs`**

Modify `TabBarSettingsContent` struct (lines 401-416):

```rust
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct TabBarSettingsContent {
    /// Whether or not to show the tab bar in the editor.
    pub show: Option<bool>,
    /// Whether or not to show the navigation history buttons in the tab bar.
    pub show_nav_history_buttons: Option<bool>,
    /// Whether or not to show the tab bar buttons.
    pub show_tab_bar_buttons: Option<bool>,
    /// Position of the tab bar: "top" (horizontal, default) or "right" (vertical side panel).
    pub position: Option<TabBarPosition>,
    /// Default width of the side tab bar in pixels (only applies when position is "right").
    /// Default: 200
    pub default_width: Option<f32>,
}
```

#### 1.3 Update TabBarSettings Runtime Struct

**File: `crates/workspace/src/workspace_settings.rs`**

Update `TabBarSettings` struct (lines 58-63):

```rust
#[derive(Deserialize, RegisterSetting)]
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
    pub position: TabBarPosition,
    pub default_width: f32,
}
```

Update the `Settings` impl for `TabBarSettings` (lines 117-126):

```rust
impl Settings for TabBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let tab_bar = content.tab_bar.clone().unwrap();
        TabBarSettings {
            show: tab_bar.show.unwrap(),
            show_nav_history_buttons: tab_bar.show_nav_history_buttons.unwrap(),
            show_tab_bar_buttons: tab_bar.show_tab_bar_buttons.unwrap(),
            position: tab_bar.position.unwrap_or_default(),
            default_width: tab_bar.default_width.unwrap_or(200.0),
        }
    }
}
```

---

### Phase 2: Side Tab Bar UI Component

#### 2.1 Create SideTabBar Component

**File: `crates/ui/src/components/side_tab_bar.rs`** (NEW FILE)

```rust
use gpui::{AnyElement, ElementId, IntoElement, ScrollHandle, Stateful};
use smallvec::SmallVec;

use crate::prelude::*;

/// A vertical tab bar component for displaying tabs on the side of a pane.
#[derive(IntoElement)]
pub struct SideTabBar {
    id: ElementId,
    children: SmallVec<[AnyElement; 8]>,
    header_children: SmallVec<[AnyElement; 2]>,
    footer_children: SmallVec<[AnyElement; 2]>,
    scroll_handle: Option<ScrollHandle>,
    width: Pixels,
}

impl SideTabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: SmallVec::new(),
            header_children: SmallVec::new(),
            footer_children: SmallVec::new(),
            scroll_handle: None,
            width: px(200.),
        }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(handle.clone());
        self
    }

    pub fn header<E: IntoElement>(mut self, child: E) -> Self {
        self.header_children.push(child.into_any_element());
        self
    }

    pub fn footer<E: IntoElement>(mut self, child: E) -> Self {
        self.footer_children.push(child.into_any_element());
        self
    }
}

impl ParentElement for SideTabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SideTabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .id(self.id)
            .group("side_tab_bar")
            .flex_none()
            .h_full()
            .w(self.width)
            .bg(cx.theme().colors().tab_bar_background)
            .border_l_1()
            .border_color(cx.theme().colors().border)
            // Header section (nav buttons, etc.)
            .when(!self.header_children.is_empty(), |this| {
                this.child(
                    v_flex()
                        .flex_none()
                        .p_1()
                        .gap_1()
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .children(self.header_children)
                )
            })
            // Scrollable tab list
            .child(
                div()
                    .id("side-tab-bar-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .when_some(self.scroll_handle, |this, handle| {
                        this.track_scroll(&handle)
                    })
                    .child(
                        v_flex()
                            .p_1()
                            .gap_px()
                            .children(self.children)
                    )
            )
            // Footer section (action buttons)
            .when(!self.footer_children.is_empty(), |this| {
                this.child(
                    v_flex()
                        .flex_none()
                        .p_1()
                        .gap_1()
                        .border_t_1()
                        .border_color(cx.theme().colors().border)
                        .children(self.footer_children)
                )
            })
    }
}
```

#### 2.2 Create SideTab Component (Tab Entry for Side Bar)

**File: `crates/ui/src/components/side_tab.rs`** (NEW FILE)

```rust
use gpui::{AnyElement, ElementId, IntoElement, Stateful};
use smallvec::SmallVec;

use crate::prelude::*;

/// A single tab entry for the vertical side tab bar.
#[derive(IntoElement)]
pub struct SideTab {
    id: ElementId,
    selected: bool,
    children: SmallVec<[AnyElement; 2]>,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    custom_bg: Option<Hsla>,
}

impl SideTab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            children: SmallVec::new(),
            start_slot: None,
            end_slot: None,
            custom_bg: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.start_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.end_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn bg(mut self, color: Hsla) -> Self {
        self.custom_bg = Some(color);
        self
    }
}

impl ParentElement for SideTab {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SideTab {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (text_color, bg_color) = if self.selected {
            (
                cx.theme().colors().text,
                self.custom_bg.unwrap_or_else(|| cx.theme().colors().tab_active_background),
            )
        } else {
            (
                cx.theme().colors().text_muted,
                self.custom_bg.unwrap_or_else(|| cx.theme().colors().tab_inactive_background),
            )
        };

        div()
            .id(self.id)
            .w_full()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(bg_color)
            .text_color(text_color)
            .cursor_pointer()
            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .w_full()
                    .overflow_hidden()
                    // Icon slot
                    .when_some(self.start_slot, |this, slot| {
                        this.child(div().flex_none().child(slot))
                    })
                    // Label (truncated)
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .children(self.children)
                    )
                    // Close button slot
                    .when_some(self.end_slot, |this, slot| {
                        this.child(div().flex_none().child(slot))
                    })
            )
    }
}
```

#### 2.3 Export New Components

**File: `crates/ui/src/components.rs`**

Add exports:

```rust
mod side_tab;
mod side_tab_bar;

pub use side_tab::*;
pub use side_tab_bar::*;
```

---

### Phase 3: Pane Integration

#### 3.1 Add State to Pane Struct

**File: `crates/workspace/src/pane.rs`**

Add fields to `Pane` struct (around line 425):

```rust
pub struct Pane {
    // ... existing fields ...

    /// Width of the side tab bar when position is "right"
    side_tab_bar_width: Option<Pixels>,
    /// Scroll handle for the side tab bar
    side_tab_bar_scroll_handle: ScrollHandle,
}
```

Initialize in `Pane::new()` (around line 491):

```rust
side_tab_bar_width: None,
side_tab_bar_scroll_handle: ScrollHandle::new(),
```

#### 3.2 Add render_side_tab_bar Method

**File: `crates/workspace/src/pane.rs`**

Add after `render_tab_bar` method (around line 3437):

```rust
fn render_side_tab_bar(&self, window: &mut Window, cx: &mut Context<Pane>) -> impl IntoElement {
    let settings = TabBarSettings::get_global(cx);
    let width = self.side_tab_bar_width.unwrap_or(px(settings.default_width));
    let focus_handle = self.focus_handle.clone();
    let pane = cx.entity().clone();

    // Build tab entries
    let mut tabs = Vec::new();

    for (ix, item) in self.items.iter().enumerate() {
        let is_active = ix == self.active_item_index;
        let item_id = item.item_id();
        let tab_bg = item.tab_background_color(cx);

        let tab_content = item.tab_content(
            TabContentParams {
                detail: self.tab_details(item.as_ref(), cx),
                selected: is_active,
                preview: self.preview_item_id == Some(item_id),
                pinned: ix < self.pinned_tab_count,
            },
            window,
            cx,
        );

        let close_button = self.render_tab_close_button(
            item_id,
            is_active,
            ix < self.pinned_tab_count,
            None, // hovered_tab_ix for side tabs not needed
            window,
            cx,
        );

        let tab = SideTab::new(("side-tab", item_id))
            .selected(is_active)
            .when_some(tab_bg, |tab, color| tab.bg(color))
            .child(tab_content)
            .end_slot(close_button)
            .on_click(cx.listener(move |pane, _, _, cx| {
                pane.activate_item(ix, true, true, window, cx);
            }))
            .on_secondary_mouse_down(cx.listener(move |pane, event, window, cx| {
                pane.deploy_tab_context_menu(event.position, item_id, window, cx);
            }));

        tabs.push(tab.into_any_element());
    }

    // Build resize handle
    let resize_handle = self.render_side_tab_bar_resize_handle(cx);

    h_flex()
        .h_full()
        .child(resize_handle)
        .child(
            SideTabBar::new("side-tab-bar")
                .width(width)
                .track_scroll(&self.side_tab_bar_scroll_handle)
                .children(tabs)
        )
}

fn render_side_tab_bar_resize_handle(&self, cx: &mut Context<Pane>) -> impl IntoElement {
    let pane = cx.entity().downgrade();

    div()
        .id("side-tab-bar-resize-handle")
        .w(px(6.))
        .h_full()
        .cursor_col_resize()
        .bg(gpui::transparent_black())
        .hover(|style| style.bg(cx.theme().colors().border))
        .on_drag(DraggedSideTabBarHandle { pane: pane.clone() }, |_, _, _, cx| {
            cx.stop_propagation();
            cx.new(|_| EmptyView)
        })
        .on_drag_move(cx.listener(|pane, event: &DragMoveEvent<DraggedSideTabBarHandle>, window, cx| {
            let current_width = pane.side_tab_bar_width.unwrap_or(px(TabBarSettings::get_global(cx).default_width));
            let bounds = window.bounds();
            let new_width = bounds.right() - event.event.position.x;
            let clamped_width = new_width.max(px(100.)).min(px(400.));
            pane.side_tab_bar_width = Some(clamped_width);
            cx.notify();
        }))
}
```

Add drag handle struct:

```rust
#[derive(Clone)]
struct DraggedSideTabBarHandle {
    pane: WeakEntity<Pane>,
}

impl Render for DraggedSideTabBarHandle {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}
```

#### 3.3 Modify Pane::render for Conditional Layout

**File: `crates/workspace/src/pane.rs`**

Modify the `Render` impl for `Pane` (around line 3956-4250):

The key change is in the main render body. Current structure:

```rust
v_flex()
    // ... setup ...
    .when(display_tab_bar, |pane| {
        pane.child(render_tab_bar(...))  // Always at top
    })
    .child(content)  // Main content below
```

Change to:

```rust
let tab_position = TabBarSettings::get_global(cx).position;

v_flex()
    // ... setup ...
    // Only show top tab bar when position is Top
    .when(display_tab_bar && tab_position == TabBarPosition::Top, |pane| {
        pane.child(render_tab_bar(...))
    })
    // Main content - wrap in h_flex when using side tabs
    .child({
        let content = /* existing content div */;

        if display_tab_bar && tab_position == TabBarPosition::Right {
            h_flex()
                .size_full()
                .child(content.flex_1())
                .child(self.render_side_tab_bar(window, cx))
                .into_any()
        } else {
            content.into_any()
        }
    })
```

---

### Phase 4: Feature Completeness

#### 4.1 Features to Support in Side Mode

All these features from horizontal tabs must work:

- [x] Click to activate tab
- [x] Right-click context menu (close, close others, pin, etc.)
- [x] Close button on each tab
- [x] Tab icons (file type icons)
- [x] Dirty/modified indicator
- [x] Diagnostic indicators (error/warning decorations)
- [x] Preview tabs (italic styling)
- [x] Pinned tabs (appear at top of list)
- [x] Custom tab background colors (from terminal tab feature)
- [x] Drag and drop reordering
- [x] Keyboard navigation

#### 4.2 Drag and Drop Adjustments

For vertical mode:
- Drop zones should be top/bottom instead of left/right
- Visual feedback shows horizontal insertion line
- Reuse existing `handle_tab_drop` logic with position adjustments

---

### Phase 5: Polish

#### 5.1 Resize Handle

- 6px wide grab handle on left edge of side tab bar
- Changes cursor to `col-resize`
- Drag to resize width
- Highlight on hover
- Min width: 100px, Max width: 400px

#### 5.2 Persistence

- Store `side_tab_bar_width` in workspace serialization
- Restore on workspace load

#### 5.3 Toggle Action (Optional)

Add keyboard shortcut to toggle between top/right:

```rust
actions!(pane, [ToggleTabBarPosition]);

// Handler cycles: Top -> Right -> Top
```

---

## Testing Checklist

1. **Settings**:
   - [ ] Setting `"tab_bar": {"position": "top"}` shows horizontal tabs (default)
   - [ ] Setting `"tab_bar": {"position": "right"}` shows vertical tabs on right
   - [ ] Setting `"tab_bar": {"default_width": 250}` changes initial width

2. **Basic Functionality**:
   - [ ] Clicking a tab activates it
   - [ ] Close button closes the tab
   - [ ] Right-click shows context menu
   - [ ] All context menu actions work

3. **Tab Features**:
   - [ ] File icons display correctly
   - [ ] Modified indicator shows
   - [ ] Diagnostic decorations show
   - [ ] Preview tabs are italic
   - [ ] Pinned tabs appear at top
   - [ ] Custom background colors work (terminal tabs)

4. **Resize**:
   - [ ] Resize handle visible on left edge
   - [ ] Dragging resizes the panel
   - [ ] Width clamps to min/max bounds
   - [ ] Width persists across sessions

5. **Multi-Pane**:
   - [ ] Each pane has its own side tab bar
   - [ ] Splitting panes works correctly
   - [ ] Tab bar position setting is global (affects all panes)

---

## Notes

- The side tab bar is rendered **inside** each Pane, not as a separate dock panel
- This maintains the existing architecture where tabs are a Pane concern
- Each pane gets its own side tab bar when the setting is enabled
- The `render_tab_bar` customization hooks (`set_render_tab_bar`) are preserved for horizontal mode
