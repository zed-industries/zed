# Rust

- Tree Sitter: [tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust)
- Language Server: [rust-analyzer](https://github.com/rust-lang/rust-analyzer)

### Target directory

The `rust-analyzer` target directory can be set in `initialization_options`:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "rust": {
          "analyzerTargetDir": true
        }
      }
    }
  }
}
```

A `true` setting will set the target directory to `target/rust-analyzer`. You can set a custom directory with a string like `"target/analyzer"` instead of `true`.
