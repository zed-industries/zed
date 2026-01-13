# Terminal Tab Customization Feature

## Overview
Add right-click context menu to terminal tabs with:
1. Rename terminal tab
2. Change tab color (10 preset colors)
3. Tab auto-resizes to fit name

## Files to Modify

### Core Terminal Changes
- `crates/terminal/src/terminal.rs` - Add `set_title_override()` method and `tab_color` field
- `crates/terminal_view/src/terminal_view.rs` - Add context menu, update `tab_content()` for colors
- `crates/workspace/src/pane.rs` - Add terminal-specific context menu entries

---

## Implementation Plan

### Step 1: Add Terminal State Fields
**File: `crates/terminal/src/terminal.rs`**

```rust
// Add to Terminal struct
pub struct Terminal {
    // ... existing fields ...
    title_override: Option<String>,  // Already exists
    tab_color: Option<Hsla>,         // NEW: custom tab color
}

// Add setter methods
impl Terminal {
    pub fn set_title_override(&mut self, title: Option<String>) {
        self.title_override = title;
    }

    pub fn set_tab_color(&mut self, color: Option<Hsla>) {
        self.tab_color = color;
    }

    pub fn tab_color(&self) -> Option<Hsla> {
        self.tab_color
    }
}
```

---

### Step 2: Define Actions
**File: `crates/terminal_view/src/terminal_view.rs`**

```rust
actions!(terminal, [RenameTerminal, SetTerminalColor]);
```

---

### Step 3: Implement Context Menu on Tab
**File: `crates/workspace/src/pane.rs`**

The tab context menu is built in `pane.rs` (line ~2887). We need to:
1. Check if the item can be downcast to `TerminalView`
2. If it's a terminal, add "Rename" and "Change Color" submenu entries

```rust
// In the context menu builder, after existing entries:
if let Some(terminal_view) = item.act_as::<TerminalView>(cx) {
    menu = menu
        .separator()
        .entry("Rename Terminal", None, /* handler */)
        .submenu("Change Color", |menu, _, _| {
            // Add color options
        });
}
```

---

### Step 4: Color Picker Submenu
Create submenu with 10 preset colors:

```rust
const TERMINAL_TAB_COLORS: &[(&str, Option<Hsla>)] = &[
    ("Red", Some(hsla(0.0, 0.7, 0.5, 1.0))),
    ("Orange", Some(hsla(30.0/360.0, 0.7, 0.5, 1.0))),
    ("Yellow", Some(hsla(60.0/360.0, 0.7, 0.5, 1.0))),
    ("Green", Some(hsla(120.0/360.0, 0.6, 0.4, 1.0))),
    ("Cyan", Some(hsla(180.0/360.0, 0.6, 0.5, 1.0))),
    ("Blue", Some(hsla(210.0/360.0, 0.7, 0.5, 1.0))),
    ("Purple", Some(hsla(270.0/360.0, 0.6, 0.5, 1.0))),
    ("Pink", Some(hsla(330.0/360.0, 0.6, 0.6, 1.0))),
    ("Gray", Some(hsla(0.0, 0.0, 0.5, 1.0))),
    ("Clear", None),  // Reset to default
];
```

---

### Step 5: Update Tab Rendering with Color
**File: `crates/terminal_view/src/terminal_view.rs`**

Modify `tab_content()` to apply custom color:

```rust
fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
    let terminal = self.terminal().read(cx);
    let title = terminal.title(true);
    let tab_color = terminal.tab_color();

    let label = Label::new(title)
        .color(if let Some(custom_color) = tab_color {
            Color::Custom(custom_color)
        } else {
            params.text_color()
        });

    h_flex()
        .gap_1()
        .child(label)
        .into_any()
}
```

---

### Step 6: Rename Input Modal
**Approach: Popup Modal**

Use a modal dialog with text input for renaming. Pattern from existing Zed modals.

---

## Verification Checklist
1. Build with `cargo build`
2. Run Zed with `cargo run`
3. Open terminal panel
4. Right-click on terminal tab
5. Test "Rename" - should show input, accept new name
6. Test "Change Color" submenu - should show 10 colors
7. Verify tab text changes color
8. Verify tab resizes with longer/shorter names
9. Test "Clear Color" resets to default
