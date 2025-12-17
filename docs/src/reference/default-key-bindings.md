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
| Find in buffer | `Cmd+F` | `Ctrl+F` |
| Replace in buffer | `Cmd+Alt+F` | `Ctrl+H` |
| Project search with replace | `Cmd+Shift+H` | `Ctrl+Shift+H` |
| Comment line | `Cmd+/` | `Ctrl+/` |
| Format document | `Cmd+Shift+I` | `Ctrl+Shift+I` |

### Navigation

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Go to line | `Ctrl+G` | `Ctrl+G` |
| Go to definition | `F12` | `F12` |
| Go to implementation | `Shift+F12` | `Shift+F12` (Linux), `Ctrl+F12` (Windows) |
| Find all references | `Alt+Shift+F12` | `Alt+Shift+F12` (Linux), `Shift+Alt+F12` (Windows) |
| Go to symbol in file | `Cmd+Shift+O` | `Ctrl+Shift+O` |
| Go back | `Ctrl+-` | `Ctrl+Alt+-` (Linux), `Alt+Left` (Windows) |
| Go forward | `Ctrl+_` | `Ctrl+Alt+_` (Linux), `Alt+Right` (Windows) |

### Multi-cursor

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Add cursor above | `Cmd+Alt+Up` | `Shift+Alt+Up` (Linux), `Ctrl+Alt+Up` (Windows) |
| Add cursor below | `Cmd+Alt+Down` | `Shift+Alt+Down` (Linux), `Ctrl+Alt+Down` (Windows) |
| Select next occurrence | `Cmd+D` | `Ctrl+D` |
| Select all occurrences | `Cmd+Shift+L` | `Ctrl+Shift+L` |

### Panels

| Action | macOS | Linux/Windows |
|--------|-------|---------------|
| Project panel | `Cmd+Shift+E` | `Ctrl+Shift+E` |
| Outline panel | `Cmd+Shift+B` | `Ctrl+Shift+B` |
| Git panel | `Ctrl+Shift+G` | `Ctrl+Shift+G` |
| Agent panel | `Cmd+?` | `Ctrl+?` (Linux), `Ctrl+Shift+/` (Windows) |

## Predefined Keymaps

If you prefer another editor's bindings, open the Settings Editor (`Cmd+,` on macOS, `Ctrl+,` on Linux/Windows) and search for `base_keymap`. Select your preferred keymap from the dropdown.

Or add this to your settings.json:

```json
{
  "base_keymap": "VSCode"
}
```

Available options:
- `VSCode` (default)
- `JetBrains`
- `SublimeText`
- `Atom`
- `TextMate`
- `Emacs`
- `Cursor`
- `None`

## See Also

- [Custom Key Bindings](../key-bindings.md) for creating your own bindings
- [All Actions](./all-actions.md) for the complete list of bindable actions
- [Vim Mode](../vim.md) for modal editing
