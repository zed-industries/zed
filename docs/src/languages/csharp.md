# C#

C# support is available through the [C# extension](https://github.com/zed-industries/zed/tree/main/extensions/csharp).

## Configuration

The `OmniSharp` binary can be configured in a Zed settings file with:

```jsonc
{
  "lsp": {
    "omnisharp": {
      "binary": {
        "path": "/path/to/OmniSharp",
        "args": ["optional", "additional", "args", "-lsp"],
      },
    },
  },
}
```
