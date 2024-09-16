# Dart

Dart support is available through the [Dart extension](https://github.com/zed-industries/zed/tree/main/extensions/dart).

- Tree Sitter: [UserNobody14/tree-sitter-dart](https://github.com/UserNobody14/tree-sitter-dart)
- Language Server: [dart language-server](https://github.com/dart-lang/sdk)

## Dart Configuration

The extension will try to find the binary `dart` by default.
If `dart` binary is found, it will run the LSP with following arguments:

```sh
dart language-server --protocol=lsp
```

If you use other means of installing dart binary, you will need to define this in the json settings.
This is an example of using `dart` installed by [FVM](https://fvm.app/):

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

In the example, `fvm` is installed with [Homebrew](https://brew.sh/).

If you are using default approach of installing `dart` binary but want to supply specific `arguments` to the LSP, you can also use this settings.

<!--
TBD: Document Dart. pubspec.yaml
- https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md
-->
