# Toggle Tab Bar Position - Implementation Tasks

## Phase 1: Settings Infrastructure

### 1.1 Settings Content Schema
**File: `crates/settings/src/settings_content/workspace.rs`**

- [ ] Add `TabBarPosition` enum with `Top` and `Right` variants
- [ ] Add `#[derive(...)]` attributes: Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, strum::VariantArray, strum::VariantNames
- [ ] Add `#[serde(rename_all = "lowercase")]` for JSON serialization
- [ ] Set `#[default]` on `Top` variant
- [ ] Add `position: Option<TabBarPosition>` field to `TabBarSettingsContent`
- [ ] Add `default_width: Option<f32>` field to `TabBarSettingsContent`
- [ ] Add doc comments for new fields

### 1.2 Runtime Settings
**File: `crates/workspace/src/workspace_settings.rs`**

- [ ] Import `TabBarPosition` from settings crate
- [ ] Add `position: TabBarPosition` field to `TabBarSettings` struct
- [ ] Add `default_width: f32` field to `TabBarSettings` struct
- [ ] Update `Settings::from_settings` impl to extract new fields
- [ ] Use `unwrap_or_default()` for position
- [ ] Use `unwrap_or(200.0)` for default_width

---

## Phase 2: UI Components

### 2.1 SideTabBar Component
**File: `crates/ui/src/components/side_tab_bar.rs`** (NEW)

- [ ] Create new file
- [ ] Add imports: gpui, smallvec, prelude
- [ ] Define `SideTabBar` struct with fields:
  - [ ] `id: ElementId`
  - [ ] `children: SmallVec<[AnyElement; 8]>`
  - [ ] `header_children: SmallVec<[AnyElement; 2]>`
  - [ ] `footer_children: SmallVec<[AnyElement; 2]>`
  - [ ] `scroll_handle: Option<ScrollHandle>`
  - [ ] `width: Pixels`
- [ ] Implement `SideTabBar::new(id)` constructor
- [ ] Implement `width(pixels)` builder method
- [ ] Implement `track_scroll(handle)` builder method
- [ ] Implement `header(element)` builder method
- [ ] Implement `footer(element)` builder method
- [ ] Implement `ParentElement` trait
- [ ] Implement `RenderOnce` trait with:
  - [ ] Vertical flex container
  - [ ] Full height
  - [ ] Fixed width
  - [ ] Tab bar background color
  - [ ] Left border
  - [ ] Optional header section with bottom border
  - [ ] Scrollable middle section for tabs
  - [ ] Optional footer section with top border
- [ ] Add `#[derive(IntoElement)]`

### 2.2 SideTab Component
**File: `crates/ui/src/components/side_tab.rs`** (NEW)

- [ ] Create new file
- [ ] Add imports
- [ ] Define `SideTab` struct with fields:
  - [ ] `id: ElementId`
  - [ ] `selected: bool`
  - [ ] `children: SmallVec<[AnyElement; 2]>`
  - [ ] `start_slot: Option<AnyElement>` (for icon)
  - [ ] `end_slot: Option<AnyElement>` (for close button)
  - [ ] `custom_bg: Option<Hsla>`
- [ ] Implement `SideTab::new(id)` constructor
- [ ] Implement `selected(bool)` builder method
- [ ] Implement `start_slot(element)` builder method
- [ ] Implement `end_slot(element)` builder method
- [ ] Implement `bg(color)` builder method
- [ ] Implement `ParentElement` trait
- [ ] Implement `RenderOnce` trait with:
  - [ ] Full width
  - [ ] Horizontal padding
  - [ ] Rounded corners
  - [ ] Background based on selected state or custom_bg
  - [ ] Text color based on selected state
  - [ ] Cursor pointer
  - [ ] Hover effect
  - [ ] Icon slot (flex-none)
  - [ ] Label slot (flex-1, truncated)
  - [ ] Close button slot (flex-none)
- [ ] Add `#[derive(IntoElement)]`

### 2.3 Export Components
**File: `crates/ui/src/components.rs`**

- [ ] Add `mod side_tab;`
- [ ] Add `mod side_tab_bar;`
- [ ] Add `pub use side_tab::*;`
- [ ] Add `pub use side_tab_bar::*;`

---

## Phase 3: Pane Integration

### 3.1 Pane Struct Changes
**File: `crates/workspace/src/pane.rs`**

- [ ] Add `side_tab_bar_width: Option<Pixels>` field to Pane struct
- [ ] Add `side_tab_bar_scroll_handle: ScrollHandle` field to Pane struct
- [ ] Initialize `side_tab_bar_width: None` in `Pane::new()`
- [ ] Initialize `side_tab_bar_scroll_handle: ScrollHandle::new()` in `Pane::new()`

### 3.2 Resize Handle Drag Struct
**File: `crates/workspace/src/pane.rs`**

- [ ] Define `DraggedSideTabBarHandle` struct with `pane: WeakEntity<Pane>`
- [ ] Add `#[derive(Clone)]`
- [ ] Implement `Render` trait (returns Empty)

### 3.3 render_side_tab_bar Method
**File: `crates/workspace/src/pane.rs`**

- [ ] Add `fn render_side_tab_bar(&self, window, cx) -> impl IntoElement` method
- [ ] Get settings and calculate width
- [ ] Build tab entries by iterating `self.items`:
  - [ ] Create `SideTab` for each item
  - [ ] Set selected state based on `active_item_index`
  - [ ] Add tab content from `item.tab_content()`
  - [ ] Add close button via `render_tab_close_button()`
  - [ ] Add custom background color from `item.tab_background_color()`
  - [ ] Add click handler to activate item
  - [ ] Add right-click handler for context menu
- [ ] Build resize handle
- [ ] Return h_flex with resize handle and SideTabBar

### 3.4 render_side_tab_bar_resize_handle Method
**File: `crates/workspace/src/pane.rs`**

- [ ] Add `fn render_side_tab_bar_resize_handle(&self, cx) -> impl IntoElement` method
- [ ] Create 6px wide div
- [ ] Set cursor to col-resize
- [ ] Add hover highlight
- [ ] Add on_drag handler with DraggedSideTabBarHandle
- [ ] Add on_drag_move handler to update width
- [ ] Clamp width between 100px and 400px

### 3.5 Modify Pane::render
**File: `crates/workspace/src/pane.rs`**

- [ ] Get `TabBarSettings::get_global(cx).position`
- [ ] Modify top tab bar condition: only render when `position == TabBarPosition::Top`
- [ ] Modify main content section:
  - [ ] If `position == TabBarPosition::Right` and `display_tab_bar`:
    - [ ] Wrap content in h_flex
    - [ ] Content on left (flex_1)
    - [ ] Side tab bar on right
  - [ ] Else: existing content behavior

---

## Phase 4: Tab Features in Side Mode

### 4.1 Basic Functionality
- [ ] Click to activate tab works
- [ ] Close button works
- [ ] Right-click context menu works
- [ ] All context menu actions work (close, close others, pin, etc.)

### 4.2 Visual Features
- [ ] File type icons display
- [ ] Modified/dirty indicator shows
- [ ] Diagnostic decorations show (error/warning)
- [ ] Preview tabs appear italic
- [ ] Pinned tabs appear at top of list
- [ ] Custom background colors work (terminal tabs)

### 4.3 Drag and Drop
- [ ] Drag tabs to reorder
- [ ] Visual feedback during drag (horizontal insertion line)
- [ ] Drop to reorder works correctly
- [ ] Drag to split pane works (if applicable)

---

## Phase 5: Polish

### 5.1 Resize Handle
- [ ] Visible on left edge of side tab bar
- [ ] Cursor changes to col-resize on hover
- [ ] Highlight appears on hover
- [ ] Drag resizes the panel smoothly
- [ ] Width clamps to min (100px) and max (400px)

### 5.2 Persistence
- [ ] Width saves when workspace is serialized
- [ ] Width restores when workspace is loaded

### 5.3 Toggle Action (Optional)
- [ ] Add `ToggleTabBarPosition` action
- [ ] Add keyboard shortcut binding
- [ ] Handler toggles between Top and Right

---

## Verification

### Build
- [ ] `cargo build -p settings -p ui -p workspace` succeeds

### Manual Testing
- [ ] Default (no setting) shows horizontal tabs at top
- [ ] `"tab_bar": {"position": "top"}` shows horizontal tabs
- [ ] `"tab_bar": {"position": "right"}` shows vertical tabs on right
- [ ] `"tab_bar": {"default_width": 250}` changes initial width
- [ ] All tab features work in side mode (see Phase 4)
- [ ] Resize handle works (see Phase 5)
- [ ] Multiple panes each have their own side tab bar
- [ ] Split pane operations work correctly

---

## Files Modified/Created

### Modified Files
1. `crates/settings/src/settings_content/workspace.rs`
2. `crates/workspace/src/workspace_settings.rs`
3. `crates/workspace/src/pane.rs`
4. `crates/ui/src/components.rs`

### New Files
1. `crates/ui/src/components/side_tab_bar.rs`
2. `crates/ui/src/components/side_tab.rs`
