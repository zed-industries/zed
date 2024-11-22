# Dart

Dart support is available through the [Dart extension](https://github.com/zed-extensions/dart).

- Tree Sitter: [UserNobody14/tree-sitter-dart](https://github.com/UserNobody14/tree-sitter-dart)
- Language Server: [dart language-server](https://github.com/dart-lang/sdk)

## Pre-requisites

You will need to install the Dart SDK.

You can install dart from [dart.dev/get-dart](https://dart.dev/get-dart) or via the [Flutter Version Management CLI (fvm)](https://fvm.app/documentation/getting-started/installation)

## Configuration

The dart extension requires no configuration if you have `dart` in your path:

```sh
which dart
dart --version
```

If you would like to use a specific dart binary or use dart via FVM you can specify the `dart` binary in your Zed settings.jsons file:

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

Please see the Dart documentation for more information on [dart language-server capabilities](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md).
