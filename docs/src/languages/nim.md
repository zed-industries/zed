# Nim

Nim language support in Zed is provided by the community-maintained [Nim extension](https://github.com/foxoman/zed-nim).
Report issues to: [https://github.com/foxoman/zed-nim/issues](https://github.com/foxoman/zed-nim/issues)

- Tree-sitter: [alaviss/tree-sitter-nim](https://github.com/alaviss/tree-sitter-nim)
- Language Server: [nim-lang/langserver](https://github.com/nim-lang/langserver)

## Formatting

To use [arnetheduck/nph](https://github.com/arnetheduck/nph) as a formatter, follow the [nph installation instructions](https://github.com/arnetheduck/nph?tab=readme-ov-file#installation) and add this to your Zed `settings.json`:

```json [settings]
  "languages": {
    "Nim": {
      "formatter": {
        "external": {
          "command": "nph",
          "arguments": ["-"]
        }
      }
    }
  }
```
