# Rust

- Tree Sitter: [tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust)
- Language Server: [rust-analyzer](https://github.com/rust-lang/rust-analyzer)

## Inlay Hints

The following configuration can be used to enable inlay hints for rust:

```json
"inlayHints": {
  "maxLength": null,
  "lifetimeElisionHints": {
  "useParameterNames": true,
    "enable": "skip_trivial"
  },
  "closureReturnTypeHints": {
    "enable": "always"
  }
}
```

to make the language server send back inlay hints when Zed has them enabled in the settings.

Use

```json
"lsp": {
  "$LANGUAGE_SERVER_NAME": {
    "initialization_options": {
      ....
    }
  }
}
```

to override these settings.

See https://rust-analyzer.github.io/manual.html#inlay-hints for more information.

## Target directory

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
