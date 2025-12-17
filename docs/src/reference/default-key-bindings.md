# Default Key Bindings

Zed ships with default key bindings optimized for each platform. You can customize these in your [keymap file](../key-bindings.md).

## View Default Keymaps

The complete default keymaps are maintained in the Zed repository:

| Platform | Keymap File |
|----------|-------------|
| macOS | [default-macos.json](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json) |
| Windows | [default-windows.json](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-windows.json) |
| Linux | [default-linux.json](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-linux.json) |

## Essential Bindings

These are the most commonly used default bindings. Platform-specific keys are shown as `Cmd` (macOS) / `Ctrl` (Linux/Windows).

### General

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Command palette | `Cmd+Shift+P` | `Ctrl+Shift+P` |
| Settings | `Cmd+,` | `Ctrl+,` |
| File finder | `Cmd+P` | `Ctrl+P` |
| Project search | `Cmd+Shift+F` | `Ctrl+Shift+F` |
| Toggle terminal | `` Ctrl+` `` | `` Ctrl+` `` |
| Toggle left dock | `Cmd+B` | `Ctrl+B` |

### Editing

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Cut | `Cmd+X` | `Ctrl+X` |
| Copy | `Cmd+C` | `Ctrl+C` |
| Paste | `Cmd+V` | `Ctrl+V` |
| Undo | `Cmd+Z` | `Ctrl+Z` |
| Redo | `Cmd+Shift+Z` | `Ctrl+Shift+Z` |
| Save | `Cmd+S` | `Ctrl+S` |
| Find | `Cmd+F` | `Ctrl+F` |
| Find and replace | `Cmd+H` | `Ctrl+H` |
| Comment line | `Cmd+/` | `Ctrl+/` |
| Format document | `Cmd+Shift+I` | `Ctrl+Shift+I` |

### Navigation

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Go to line | `Ctrl+G` | `Ctrl+G` |
| Go to definition | `F12` or `Cmd+Click` | `F12` or `Ctrl+Click` |
| Go to references | `Shift+F12` | `Shift+F12` |
| Go to symbol | `Cmd+Shift+O` | `Ctrl+Shift+O` |
| Go back | `Ctrl+-` | `Alt+Left` |
| Go forward | `Ctrl+Shift+-` | `Alt+Right` |

### Multi-cursor

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Add cursor above | `Cmd+Alt+Up` | `Ctrl+Alt+Up` |
| Add cursor below | `Cmd+Alt+Down` | `Ctrl+Alt+Down` |
| Select next occurrence | `Cmd+D` | `Ctrl+D` |
| Select all occurrences | `Cmd+Shift+L` | `Ctrl+Shift+L` |

### Panels

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Project panel | `Cmd+Shift+E` | `Ctrl+Shift+E` |
| Outline panel | `Cmd+Shift+O` | `Ctrl+Shift+O` |
| Git panel | `Cmd+Shift+G` | `Ctrl+Shift+G` |
| Agent panel | `Cmd+Shift+A` | `Ctrl+Shift+A` |

## Predefined Keymaps

If you prefer another editor's bindings, change the `base_keymap` setting:

```json
{
  "base_keymap": "VSCode"
}
```

Available options:
- `VSCode` (default)
- `Atom`
- `Emacs`
- `JetBrains`
- `SublimeText`
- `TextMate`
- `Cursor`
- `None`

## See Also

- [Custom Key Bindings](../key-bindings.md) for creating your own bindings
- [All Actions](./all-actions.md) for the complete list of bindable actions
- [Vim Mode](../vim.md) for modal editing
