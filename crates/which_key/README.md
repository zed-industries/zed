# Which-Key

Which-key is a UI component that displays available key bindings when you start typing a key combination in Zed. It helps users discover keyboard shortcuts and navigate complex key binding sequences.

## Features

- **Automatic Discovery**: Shows available key bindings when you hold down a key combination
- **Contextual Help**: Displays only the key bindings that are valid in the current context
- **Configurable**: Can be enabled or disabled via settings
- **Smart Layout**: Automatically arranges key bindings in a grid layout that fits the available space
- **Dock-aware**: Positions itself relative to open docks and panels

## Configuration

The which-key display can be configured in your settings:

```json
{
  "which_key": {
    "enabled": true  // Set to false to disable the which-key popup
  }
}
```

## How it Works

1. When you press a key combination that has multiple possible completions, a timer starts
2. After a short delay (600ms), the which-key popup appears
3. The popup shows all possible key combinations you can press next
4. The popup automatically positions itself to avoid covering important UI elements
5. Once you complete the key combination or release the keys, the popup disappears

## Integration

This crate provides the settings and configuration for which-key functionality. The actual UI layer is implemented in the `workspace` crate to avoid circular dependencies.

The which-key system integrates with Zed's key binding system and automatically discovers available shortcuts based on the current context and focus.