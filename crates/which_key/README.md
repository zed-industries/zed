# Which-Key

Which-key is a UI component that displays available key bindings when you start typing a key combination in Zed. It helps users discover keyboard shortcuts and navigate complex key binding sequences.

## Features

- **Automatic Discovery**: Shows available key bindings when you hold down a key combination
- **Contextual Help**: Displays only the key bindings that are valid in the current context
- **Configurable**: Can be enabled or disabled via settings
- **Smart Layout**: Automatically arranges key bindings in a grid layout that fits the available space
- **Dock-aware**: Positions itself relative to open docks and panels
- **Grouping**: Groups key bindings that share the same first keystroke to reduce clutter

## Configuration

The which-key display can be configured in your settings:

```json
{
  "which_key": {
    "enabled": true,  // Set to false to disable the which-key popup
    "delay_ms": 600,  // Delay in milliseconds before showing the popup
    "group": true     // Group key bindings with the same first keystroke
  }
}
```

## How it Works

1. When you press a key combination that has multiple possible completions, a timer starts
2. After a short delay (600ms by default), the which-key popup appears
3. The popup shows all possible key combinations you can press next
4. If grouping is enabled, key bindings with the same first keystroke are grouped together and shown as "group"
5. The popup automatically positions itself to avoid covering important UI elements
6. Once you complete the key combination or release the keys, the popup disappears

## Grouping

When the `group` setting is enabled (default), which-key will group key bindings that share the same first keystroke. For example, if you have multiple bindings like:
- `g d` → Go to definition  
- `g r` → Go to references
- `g i` → Go to implementation

Instead of showing all three separately, which-key will show:
- `g` → +3 keybinds

This helps reduce visual clutter when you have many related key bindings. You can disable grouping by setting `"group": false` in your settings.

## Integration

This crate provides the settings and configuration for which-key functionality. The actual UI layer is implemented in the `workspace` crate to avoid circular dependencies.

The which-key system integrates with Zed's key binding system and automatically discovers available shortcuts based on the current context and focus.