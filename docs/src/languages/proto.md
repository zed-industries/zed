---
title: Proto
description: "Configure Proto language support in Zed, including language servers, formatting, and debugging."
---

# Proto

Proto/proto3 (Protocol Buffers definition language) support is available through the [Proto extension](https://github.com/zed-industries/zed/tree/main/extensions/proto).

- Tree-sitter: [coder3101/tree-sitter-proto](https://github.com/coder3101/tree-sitter-proto)
- Language Servers:
  - [bufbuild/buf](https://github.com/bufbuild/buf) (`buf`)
  - [lasorda/protobuf-language-server](https://github.com/lasorda/protobuf-language-server) (`protobuf-language-server`)
  - [coder3101/protols](https://github.com/coder3101/protols) (`protols`)

## Language Servers

The Proto extension supports three language servers: Buf, Protobuf Language Server, and Protols.
Buf is enabled by default and is downloaded automatically.
You can change the enabled language servers in your settings ({#kb zed::OpenSettings}).

### Using Buf

Buf is downloaded automatically from [GitHub Releases](https://github.com/bufbuild/buf/releases).

To use a custom Buf binary, add the following to your settings:

```json [settings]
{
  "lsp": {
    "buf": {
      "binary": {
        "path": "/path/to/buf"
      }
    }
  }
}
```

### Using Protols

Protols is downloaded automatically from [GitHub Releases](https://github.com/coder3101/protols/releases).

Enable Protols by adding the following to your settings:

```json [settings]
{
  "languages": {
    "Proto": {
      "language_servers": ["protols", "!buf", "!protobuf-language-server", "..."]
    }
  }
}
```

To use a custom Protols binary:

```json [settings]
{
  "lsp": {
    "protols": {
      "binary": {
        "path": "/path/to/protols"
      }
    }
  }
}
```

### Using Protobuf Language Server

Protobuf Language Server must be installed manually and available in your PATH:

```sh
go install github.com/lasorda/protobuf-language-server@latest
```

Enable it by adding the following to your settings:

```json [settings]
{
  "languages": {
    "Proto": {
      "language_servers": ["protobuf-language-server", "!buf", "!protols", "..."]
    }
  }
}
```

## Formatting

Protols supports formatting via `clang-format`. Install it first:

```sh
# macOS
brew install clang-format
# Ubuntu
sudo apt-get install clang-format
# Fedora
sudo dnf install clang-tools-extra
```

To customize formatting, create a `.clang-format` file, for example:

```yaml
IndentWidth: 4
ColumnLimit: 120
```

Alternatively, invoke `clang-format` directly as an [external formatter](../reference/all-settings.md#formatter):

```json [settings]
{
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
    }
  }
}
```
