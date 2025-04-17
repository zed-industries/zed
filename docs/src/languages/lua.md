# Lua

Lua support is available through the [Lua extension](https://github.com/zed-extensions/lua).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua)
- Language server: [LuaLS/lua-language-server](https://github.com/LuaLS/lua-language-server)

## luarc.json

To configure LuaLS you can create a `.luarc.json` file in the root of your workspace.

```json
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "format.enable": true,
  "workspace.library": ["../somedir/library"]
}
```

See [LuaLS Settings Documentation](https://luals.github.io/wiki/settings/) for all available configuration options, or when editing this file in Zed available settings options will autocomplete, (e.g `runtime.version` will show `"Lua 5.1"`, `"Lua 5.2"`, `"Lua 5.3"`, `"Lua 5.4"` and `"LuaJIT"` as allowed values). Note when importing settings options from VSCode, remove the `Lua.` prefix. (e.g. `runtime.version` instead of `Lua.runtime.version`).

### LuaCATS Definitions

LuaLS can provide enhanced LSP autocompletion suggestions and type validation with the help of LuaCATS (Lua Comment and Type System) definitions. These definitions are available for many common Lua libraries, and local paths containing them can be specified via `workspace.library` in `luarc.json`. You can do this via relative paths if you checkout your definitions into the same partent directory of your project (`../playdate-luacats`, `../love2d`, etc). Alternatively you can create submodule(s) inside your project for each LuaCATS definition repo.

### LÖVE (Love2D) {#love2d}

To use [LÖVE (Love2D)](https://love2d.org/) in Zed, checkout [LuaCATS/love2d](https://github.com/LuaCATS/love2d) into a folder called `love2d-luacats` into the parent folder of your project:

```sh
cd .. && git clone https://github.com/LuaCATS/love2d love2d-luacats
```

Then in your `.luarc.json`:

```
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "workspace.library": ["../love2d-luacats"],
  "runtime.special": {
    "love.filesystem.load": "loadfile"
  }
}
```

### PlaydateSDK

To use [Playdate Lua SDK](https://play.date/dev/) in Zed, checkout [playdate-luacats](https://github.com/notpeter/playdate-luacats) into the parent folder of your project:

```sh
cd .. && git clone https://github.com/notpeter/playdate-luacats
```

Then in your `.luarc.json`:

```json
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "runtime.nonstandardSymbol": [
    "+=",
    "-=",
    "*=",
    "/=",
    "//=",
    "%=",
    "<<=",
    ">>=",
    "&=",
    "|=",
    "^="
  ],
  "diagnostics.severity": { "duplicate-set-field": "Hint" },
  "diagnostics.globals": ["import"],
  "workspace.library": ["../playdate-luacats"],
  "format.defaultConfig": {
    "indent_style": "space",
    "indent_size": "4"
  },
  "format.enable": true,
  "runtime.builtin": { "io": "disable", "os": "disable", "package": "disable" }
}
```

### Inlay Hints

To enable [Inlay Hints](../configuring-languages#inlay-hints) for LuaLS in Zed

1. Add the following to your Zed settings.json:

```json
  "languages": {
    "Lua": {
      "inlay_hints": {
        "enabled": true,
        "show_type_hints": true,
        "show_parameter_hints": true,
        "show_other_hints": true
      }
    }
  }
```

2. Add `"hint.enable": true` to your `.luarc.json`.

## Formatting

### LuaLS

To enable auto-formatting with your LuaLS (provided by [CppCXY/EmmyLuaCodeStyle](https://github.com/CppCXY/EmmyLuaCodeStyle)) make sure you have `"format.enable": true,` in your .luarc.json add the following to your Zed `settings.json`:

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

You can customize various EmmyLuaCodeStyle style options via `.editorconfig`, see [lua.template.editorconfig](https://github.com/CppCXY/EmmyLuaCodeStyle/blob/master/lua.template.editorconfig) for all available options.

### StyLua

Alternatively to use [StyLua](https://github.com/JohnnyMorganz/StyLua) for auto-formatting:

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
