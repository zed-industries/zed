# Proto

Proto/proto3 (Protocol Buffers definition language) support is available natively in Zed.

- Tree Sitter: [coder3101/tree-sitter-proto](https://github.com/coder3101/tree-sitter-proto)
- Language Server: [protols](https://github.com/coder3101/protols)

## Formatting

ProtoLS supports formatting if you have `clang-format` installed.

```sh
# MacOS:
brew install clang-format
# Ubuntu
sudo apt-get install clang-format
# Fedora
sudo dnf install clang-tools-extra
```

To customize your formatting preferences, create a `.clang-format` file, e.g.:

```clang-format
IndentWidth: 4
ColumnLimit: 120
```

Or you can have zed directly invoke `clang-format` by specifying it as a [formatter](https://zed.dev/docs/configuring-zed#formatter) in your settings:

```json
  "languages": {
    "Proto": {
      "format_on_save": "on",
      "tab_size": 4,
      "formatter": {
        "external": {
          "command": "clang-format",
          "arguments": ["-style={IndentWidth: 4, ColumnLimit: 0}"]
        }
      }
    },
  }
```
