# Lua

Lua support is available through the [Lua extension](https://github.com/zed-industries/zed/tree/main/extensions/lua).

- Tree Sitter: [tree-sitter-grammars/tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua)
- Language server: [LuaLS/lua-language-server](https://github.com/LuaLS/lua-language-server)

## luarc.json

To configure LuaLS you can create a `.luarc.json` file in the root of your workspace.

See [LuaLS Settings Documentation](https://luals.github.io/wiki/settings/) for all available configuration options.

```json
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "diagnostics.severity": {
    //      "duplicate-set-field": "Hint"
  },
  "format.defaultConfig": {
    "indent_style": "space",
    "indent_size": "4"
  },
  // Location(s) of any LuaCATS / EmmyLua annotation stubs
  "workspace.library": [
    //    "path/to/library/directory"
  ]
}
```
