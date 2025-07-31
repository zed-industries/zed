# TOML

TOML support is available through the [TOML extension](https://github.com/zed-industries/zed/tree/main/extensions/toml).

- Tree-sitter: [tree-sitter/tree-sitter-toml](https://github.com/tree-sitter/tree-sitter-toml)
- Language Server: [tamasfe/taplo](https://github.com/tamasfe/taplo)

## Configuration

You can control the behavior of the Taplo TOML language server by adding a `.taplo.toml` file to the root of your project. See the [Taplo Configuration File](https://taplo.tamasfe.dev/configuration/file.html#configuration-file) and [Taplo Formatter Options](https://taplo.tamasfe.dev/configuration/formatter-options.html) documentation for more.

```toml
# .taplo.toml
[formatting]
align_comments = false
reorder_keys = true

include = ["Cargo.toml", "some_directory/**/*.toml"]
# exclude = ["vendor/**/*.toml"]
```

Note: The taplo language server will not automatically pickup changes to `.taplo.toml`. You must manually trigger {#action editor::RestartLanguageServer} or reload Zed for it to pickup changes.
