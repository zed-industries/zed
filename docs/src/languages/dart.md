# Dart

Dart support is available through the [Dart extension](https://github.com/zed-industries/zed/tree/main/extensions/dart).

- Tree Sitter: [UserNobody14/tree-sitter-dart](https://github.com/UserNobody14/tree-sitter-dart)
- Language Server: [dart language-server](https://github.com/dart-lang/sdk)

## Dart Configuration

The extension will try to find the binary `dart` by default.

If you use other means of installing dart binary, you will need to define this in the json settings.
This is example of using `dart` installed with [FVM](https://fvm.app/):

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
