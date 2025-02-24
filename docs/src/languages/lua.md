# Lua

Lua support is available through the [Lua extension](https://github.com/zed-industries/zed/tree/main/extensions/lua).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua)
- Language server: [LuaLS/lua-language-server](https://github.com/LuaLS/lua-language-server)

## luarc.json

To configure LuaLS you can create a `.luarc.json` file in the root of your workspace.

See [LuaLS Settings Documentation](https://luals.github.io/wiki/settings/) for all available configuration options.

```json
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "diagnostics.severity": {
    "duplicate-set-field": "Hint"
  },
  "format.enable": true,
  "format.defaultConfig": {
    "indent_style": "space",
    "indent_size": "4"
  },
  "workspace.library": ["../somedir/library"]
}
```

## Formatting

### LuaLS

To enable auto-formatting with your LuaLS, make sure you have `"format.enable": true,` in your .luarc.json add the following to your Zed `settings.json`:

```json
{
  "languages": {
    "Lua": {
      "format_on_save": "on",
      "formatter": "language_server"
    }
  }
}
```

### StyLua

Alternative you can use [StyLua](https://github.com/JohnnyMorganz/StyLua):

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
          "arguments": [
            "--syntax=Lua54",
            "--respect-ignores",
            "--stdin-filepath",
            "{buffer_path}",
            "-"
          ]
        }
      }
    }
  }
}
```

You can specify various options to StyLua either on the command line above (like `--syntax=Lua54`) or in a `stylua.toml` in your workspace:

```toml
syntax = "Lua54"
column_width = 100
line_endings = "Unix"
indent_type = "Spaces"
indent_width = 4
quote_style = "AutoPreferDouble"
call_parentheses = "Always"
collapse_simple_statement = "All"

[sort_requires]
enabled = true
```

For a complete list of available options, see: [StyLua Options](https://github.com/JohnnyMorganz/StyLua?tab=readme-ov-file#options).
