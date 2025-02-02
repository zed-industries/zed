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

## Formatting

Zed can enable auto-formatting of code with formatters like [StyLua](https://github.com/JohnnyMorganz/StyLua).

1. Install [StyLua](https://github.com/JohnnyMorganz/StyLua): `brew install stylua` or `cargo install stylua --features lua52,lua53,lua54,luau,luajit` (feel free to remove any Lua versions you don't need).
2. Add the following to your `settings.json`:

```json
{
  "languages": {
    "Lua": {
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "stylua",
          "arguments": ["--syntax=Lua54", "-"]
        }
      }
    }
  }
}
```
