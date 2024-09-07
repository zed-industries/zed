# C++

C++ support is available natively in Zed.

- Tree Sitter: [tree-sitter/tree-sitter-cpp](https://github.com/tree-sitter/tree-sitter-cpp)
- Language Server: [clangd/clangd](https://github.com/clangd/clangd)

## Binary

You can configure which `clangd` binary Zed should use.

To use a binary in a custom location, add the following to your `settings.json`:

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "path": "/path/to/clangd",
        "args": []
      }
    }
  }
}
```
