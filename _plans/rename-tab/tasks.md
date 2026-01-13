# Terminal Tab Customization - Tasks

## Phase 1: Core Terminal Changes
**File: `crates/terminal/src/terminal.rs`**

- [x] Add `tab_color: Option<Hsla>` field to `Terminal` struct
- [x] Initialize `tab_color` to `None` in `Terminal::new()` constructor
- [x] Add `set_title_override(&mut self, title: Option<String>)` method
- [x] Add `set_tab_color(&mut self, color: Option<Hsla>)` method
- [x] Add `tab_color(&self) -> Option<Hsla>` getter method

---

## Phase 2: Terminal View Updates
**File: `crates/terminal_view/src/terminal_view.rs`**

- [x] Add `RenameTerminal` action
- [x] Add `SetTerminalTabColor` action with color parameter
- [x] Implement `tab_background_color()` method for background color
- [x] Implement `rename_terminal()` handler - shows modal dialog
- [x] Implement `set_tab_color()` handler

---

## Phase 3: Context Menu Integration
**Note: Added to terminal's right-click context menu instead of pane.rs to avoid circular dependencies**

- [x] Add "Rename Terminal" menu entry to terminal context menu
- [x] Add "Tab Color" submenu with 10 preset colors
- [x] Add "Clear Color" option to reset to default

---

## Phase 4: Rename Modal Dialog
**File: `crates/terminal_view/src/terminal_view.rs`**

- [x] Create `RenameTerminalModal` component
- [x] Add text input field pre-filled with current title
- [x] Handle Enter key to confirm rename (via `Confirm` action)
- [x] Handle Escape key to cancel (via `Cancel` action)
- [x] Call `terminal.set_title_override()` on confirm

---

## Phase 5: Testing & Verification

- [x] Build with `cargo build -p terminal -p terminal_view` - no errors
- [ ] Run with `cargo run`
- [ ] Test: Right-click inside terminal shows context menu with new options
- [ ] Test: "Rename Terminal" opens modal dialog
- [ ] Test: Typing new name and pressing Enter renames tab
- [ ] Test: Pressing Escape cancels rename
- [ ] Test: "Tab Color" submenu shows 10 colors
- [ ] Test: Clicking color changes tab background color
- [ ] Test: "Clear Color" resets tab to default color
- [ ] Test: Tab width adjusts to fit longer/shorter names
- [ ] Test: Changes persist during session

---

## Implementation Notes

- Added `menu` crate dependency to terminal_view/Cargo.toml
- Implemented custom `Action` trait for `SetTerminalTabColor` since it carries data (Hsla)
- Used `Hsla` struct literals instead of `hsla()` function for const colors
- Context menu is added to terminal's own right-click menu (not tab context menu) to avoid pane.rs <-> terminal_view circular dependency
- Modal uses workspace's `toggle_modal()` pattern from git_ui crate

---

## Files Modified

1. `crates/terminal/src/terminal.rs`
   - Added `tab_color` field to `Terminal` struct
   - Added `set_title_override()`, `set_tab_color()`, `tab_color()` methods

2. `crates/terminal_view/src/terminal_view.rs`
   - Added actions: `RenameTerminal`, `SetTerminalTabColor`, `ClearTerminalTabColor`
   - Added `TERMINAL_TAB_COLORS` constant with 10 preset colors
   - Added `RenameTerminalModal` struct with Render, ModalView, Focusable impls
   - Updated `deploy_context_menu()` to include rename and color options
   - Added `tab_background_color()` implementation
   - Added action handlers for rename and color changes

3. `crates/terminal_view/Cargo.toml`
   - Added `menu.workspace = true` dependency

4. `crates/workspace/src/item.rs`
   - Added `tab_background_color()` method to `Item` trait
   - Added `tab_background_color()` method to `ItemHandle` trait
   - Added implementation for `Entity<T: Item>`

5. `crates/workspace/src/pane.rs`
   - Updated tab rendering to apply custom background color via `.bg()`

6. `crates/ui/src/components/tab.rs`
   - Added `custom_bg: Option<Hsla>` field to `Tab` struct
   - Added `pub fn bg(color: Hsla)` method to set custom background
   - Updated `render()` to use custom background when set
