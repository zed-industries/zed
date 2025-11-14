# JSON

JSON support is available natively in Zed.

- Tree-sitter: [tree-sitter/tree-sitter-json](https://github.com/tree-sitter/tree-sitter-json)
- Language Server: [zed-industries/json-language-server](https://github.com/zed-industries/json-language-server)

## JSONC

Zed also supports a super-set of JSON called JSONC, which allows single line comments (`//`) in JSON files.
While editing these files you can use `cmd-/` (macOS) or `ctrl-/` (Linux) to toggle comments on the current line or selection.

## JSONC Prettier Formatting

If you use files with the `*.jsonc` extension when using `Format Document` or have `format_on_save` enabled, Zed invokes Prettier as the formatter. Prettier has an [outstanding issue](https://github.com/prettier/prettier/issues/15956) where it will add trailing commas to files with a `jsonc` extension. JSONC files which have a `.json` extension are unaffected.

To workaround this behavior you can add the following to your `.prettierrc` configuration file:

```json [settings]
{
  "overrides": [
    {
      "files": ["*.jsonc"],
      "options": {
        "parser": "json",
        "trailingComma": "none"
      }
    }
  ]
}
```

## JSON Language Server

Zed automatically out of the box supports JSON Schema validation of `package.json` and `tsconfig.json` files, but `json-language-server` can use JSON Schema definitions in project files, from the [JSON Schema Store](https://www.schemastore.org) or other publicly available URLs for JSON validation.

### Inline Schema Specification

To specify a schema inline with your JSON files, add a `$schema` top level key linking to your json schema file.

For example to for a `.luarc.json` for use with [lua-language-server](https://github.com/LuaLS/lua-language-server/):

```json [settings]
{
  "$schema": "https://raw.githubusercontent.com/sumneko/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4"
}
```

### Schema Specification via Settings

You can alternatively associate JSON Schemas with file paths by via Zed LSP settings.

To

```json [settings]
"lsp": {
  "json-language-server": {
    "settings": {
      "json": {
        "schemas": [
          {
            "fileMatch": ["*/*.luarc.json"],
            "url": "https://raw.githubusercontent.com/sumneko/vscode-lua/master/setting/schema.json"
          }
        ]
      }
    }
  }
}
```

You can also pass any of the [supported settings](https://github.com/Microsoft/vscode/blob/main/extensions/json-language-features/server/README.md#settings) to json-language-server by specifying them in your Zed settings.json:

<!--
TBD: Add formatter (prettier) settings (autoformat, tab_size, etc)
-->
