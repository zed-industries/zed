# Dart

Dart support is available through the [Dart extension](https://github.com/zed-industries/zed/tree/main/extensions/dart).

- Tree Sitter: [UserNobody14/tree-sitter-dart](https://github.com/UserNobody14/tree-sitter-dart)
- Language Server: [dart language-server](https://github.com/dart-lang/sdk)

## Configuration

The `dart` binary can be configured in a Zed settings file with:

```json
{
  "lsp": {
    "dart": {
      "binary": {
        "path": "/opt/homebrew/bin/fvm",
        "arguments": ["dart", "language-server", "--protocol=lsp"]
      }
    }
  }
}
```

<!--
TBD: Document Dart. pubspec.yaml
- https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md
-->
