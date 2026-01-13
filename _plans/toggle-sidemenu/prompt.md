# AI Session Prompt: Implement Toggle Tab Bar Position Feature

## Context

You are continuing work on the Zed IDE codebase. The user wants to add a feature to toggle between horizontal tabs at the top of panes and vertical tabs on the right side of panes (similar to VS Code's vertical tab mode).

## Your Task

Implement the "Toggle Tab Bar Position" feature according to the plan in this folder:

- **plans.md** - Detailed implementation plan with code examples
- **tasks.md** - Checkbox list of tasks to complete

## Getting Started

1. **Read the plan files** in this folder (`_plans/toggle-sidemenu/`)
2. **Check off tasks** in `tasks.md` as you complete them
3. **Follow the phase order** in the plan

## Key Implementation Points

### Setting Format
```json
{
  "tab_bar": {
    "position": "top" | "right",
    "default_width": 200
  }
}
```

### Files to Modify
1. `crates/settings/src/settings_content/workspace.rs` - Add TabBarPosition enum
2. `crates/workspace/src/workspace_settings.rs` - Update TabBarSettings struct
3. `crates/workspace/src/pane.rs` - Add side tab bar rendering logic
4. `crates/ui/src/components.rs` - Export new components

### Files to Create
1. `crates/ui/src/components/side_tab_bar.rs` - Vertical tab bar container
2. `crates/ui/src/components/side_tab.rs` - Individual tab entry for side bar

## Architecture Notes

- The side tab bar renders **inside** each Pane (not as a separate dock panel)
- This maintains the existing architecture where tabs are a Pane concern
- Each pane gets its own side tab bar when the setting is enabled
- The `position` setting is global and affects all panes

## Important Patterns to Follow

### GPUI Components
- Use `#[derive(IntoElement)]` for UI components
- Implement `RenderOnce` trait for rendering
- Implement `ParentElement` for components that accept children
- Use builder pattern for component configuration

### Existing Tab Bar Reference
- Look at `crates/ui/src/components/tab_bar.rs` for horizontal tab bar implementation
- Look at `crates/ui/src/components/tab.rs` for individual tab implementation
- The side versions should follow similar patterns but with vertical layout

### Resize Handle Reference
- Look at `crates/workspace/src/dock.rs` for resize handle implementation
- The `create_resize_handle()` function shows the pattern for draggable handles

## Build Commands

```bash
# Build relevant crates
cargo build -p settings -p ui -p workspace

# Or build everything
cargo build

# Run Zed
cargo run
```

## Testing the Feature

1. Build and run Zed
2. Open settings.json (Cmd+,)
3. Add: `"tab_bar": {"position": "right"}`
4. Verify tabs appear vertically on the right side of panes
5. Test all tab functionality (click, close, context menu, etc.)
6. Test resize handle on left edge of side tab bar

## Previous Work

The user has already implemented a terminal tab customization feature that adds:
- Rename terminal tab via right-click menu
- Custom tab background colors (10 preset colors)

This feature added `tab_background_color()` method to the `Item` trait, which should be supported in the side tab bar.

See `_plans/rename-tab/` for that implementation if you need to understand the tab color system.

## Notes

- Follow the Rust coding guidelines in CLAUDE.md (no `unwrap()`, propagate errors with `?`, etc.)
- The codebase uses GPUI framework for UI - refer to CLAUDE.md for GPUI patterns
- Use `cx.notify()` when state changes that affect rendering
- Don't create files with `mod.rs` paths - prefer `src/some_module.rs`
