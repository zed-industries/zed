# C#

Note language name is "CSharp" for settings not "C#'

C# support is available through the [C# extension](https://github.com/zed-industries/zed/tree/main/extensions/csharp).

- Tree Sitter: [tree-sitter/tree-sitter-c-sharp](https://github.com/tree-sitter/tree-sitter-c-sharp)
- Language Server: [OmniSharp/omnisharp-roslyn](https://github.com/OmniSharp/omnisharp-roslyn)

## Configuration

The `OmniSharp` binary can be configured in a Zed settings file with:

```json
{
  "lsp": {
    "omnisharp": {
      "binary": {
        "path": "/path/to/OmniSharp",
        "arguments": ["optional", "additional", "args", "-lsp"]
      }
    }
  }
}
```

If you want to disable Zed looking for a `omnisharp` binary, you can set `ignore_system_version` to `true`:

```json
{
  "lsp": {
    "omnisharp": {
      "binary": {
        "ignore_system_version": true
      }
    }
  }
}
```
