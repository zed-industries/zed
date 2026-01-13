# Codebase Context for Tab Bar Feature

## Relevant Crate Structure

```
crates/
├── settings/
│   └── src/
│       └── settings_content/
│           └── workspace.rs     # JSON schema for settings (TabBarSettingsContent)
│
├── workspace/
│   └── src/
│       ├── workspace_settings.rs  # Runtime settings (TabBarSettings struct)
│       ├── pane.rs               # Pane struct, tab bar rendering (~4200 lines)
│       ├── dock.rs               # Dock/panel system, resize handles
│       └── item.rs               # Item trait (tab content, background color)
│
└── ui/
    └── src/
        └── components/
            ├── tab.rs            # Tab UI component (horizontal)
            ├── tab_bar.rs        # TabBar UI component (horizontal)
            └── components.rs     # Module exports
```

## Key Code Locations

### Settings

**TabBarSettingsContent** (JSON schema)
- File: `crates/settings/src/settings_content/workspace.rs`
- Lines: ~401-416
- Defines: `show`, `show_nav_history_buttons`, `show_tab_bar_buttons`

**TabBarSettings** (Runtime)
- File: `crates/workspace/src/workspace_settings.rs`
- Lines: ~58-126
- Accessed via: `TabBarSettings::get_global(cx)`

### Tab Bar Rendering

**Pane::render_tab_bar**
- File: `crates/workspace/src/pane.rs`
- Lines: ~3156-3437
- Builds the horizontal TabBar with all tabs

**Tab rendering per item**
- File: `crates/workspace/src/pane.rs`
- Lines: ~2597-2800
- Renders individual tab with icon, label, close button, drag/drop

**Pane::render (main render)**
- File: `crates/workspace/src/pane.rs`
- Lines: ~3956-4250
- Layout: v_flex with tab bar on top, content below

### Tab Context Menu

**deploy_tab_context_menu**
- File: `crates/workspace/src/pane.rs`
- Lines: ~2875-3000
- Right-click menu: close, close others, pin, split, etc.

### UI Components

**TabBar**
- File: `crates/ui/src/components/tab_bar.rs`
- Horizontal flex container with scrollable center

**Tab**
- File: `crates/ui/src/components/tab.rs`
- Individual tab with position, selected state, slots for content

### Resize Handle Pattern

**create_resize_handle**
- File: `crates/workspace/src/dock.rs`
- Lines: ~1082-1130
- Shows pattern for draggable resize handles

## Important Types

```rust
// Settings
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
}

// Pane state
pub struct Pane {
    items: Vec<Box<dyn ItemHandle>>,
    active_item_index: usize,
    preview_item_id: Option<EntityId>,
    pinned_tab_count: usize,
    tab_bar_scroll_handle: ScrollHandle,
    // ... more fields
}

// Tab content parameters
pub struct TabContentParams {
    pub detail: usize,
    pub selected: bool,
    pub preview: bool,
    pub pinned: bool,
}
```

## Item Trait (Tab Content)

```rust
// From crates/workspace/src/item.rs
pub trait Item: ... {
    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> AnyElement;
    fn tab_background_color(&self, _cx: &App) -> Option<Hsla> { None }
    // ... more methods
}
```

## GPUI Component Pattern

```rust
#[derive(IntoElement)]
pub struct MyComponent {
    // fields
}

impl RenderOnce for MyComponent {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .child(...)
    }
}
```

## Existing Tab Features to Support

1. **Click to activate** - `on_click` handler calls `pane.activate_item()`
2. **Close button** - `render_tab_close_button()` method
3. **Right-click menu** - `on_secondary_mouse_down` calls `deploy_tab_context_menu()`
4. **Drag and drop** - `Draggable` handlers with `DraggedTab` struct
5. **Icons** - From `item.tab_icon()` with diagnostic decorations
6. **Preview tabs** - Italic when `preview_item_id` matches
7. **Pinned tabs** - First `pinned_tab_count` items, special styling
8. **Custom colors** - From `item.tab_background_color()` (added in rename-tab feature)

## Build and Test

```bash
# Build specific crates
cargo build -p settings -p ui -p workspace

# Run clippy
./script/clippy

# Run Zed
cargo run
```
