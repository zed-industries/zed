# Which-Key

Which-key is a UI component that displays available key bindings when you start typing a key combination in Zed. It helps users discover keyboard shortcuts and navigate complex key binding sequences.

## Configuration

The which-key display can be configured in your settings:

```json
{
  "which_key": {
    // Whether to show the which-key popup when holding down key combinations.
    "enabled": true,
    // Delay in milliseconds before showing the which-key popup.
    "delay_ms": 600,
    // Whether to group key bindings with the same first keystroke.
    "group": true,
    // Where to show the which-key popup.
    // Options: "buffer" (default), "left_panel"
    "location": "buffer"
  },
}
```

## Grouping

When the `group` setting is enabled (default), which-key will group key bindings that share the same first keystroke. For example, if you have multiple bindings like:
- `g d` → Go to definition
- `g r` → Go to references
- `g i` → Go to implementation

Instead of showing all three separately, which-key will show:
- `g` → +3 keybinds

This helps reduce visual clutter when you have many related key bindings. You can disable grouping by setting `"group": false` in your settings.
