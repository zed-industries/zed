# Jsonnet

Jsonnet language support in Zed is provided by the community-maintained [Jsonnet extension](https://github.com/narqo/zed-jsonnet).

- Tree-sitter: [sourcegraph/tree-sitter-jsonnet](https://github.com/sourcegraph/tree-sitter-jsonnet)
- Language Server: [grafana/jsonnet-language-server](https://github.com/grafana/jsonnet-language-server)

## Configuration

Workspace configuration options can be passed to the language server via the `lsp` settings of the `settings.json`.

The following example enables support for resolving [tanka](https://tanka.dev) import paths in `jsonnet-language-server`:

```json [settings]
{
  "lsp": {
    "jsonnet-language-server": {
      "settings": {
        "resolve_paths_with_tanka": true
      }
    }
  }
}
```
