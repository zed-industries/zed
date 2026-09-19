---
title: Erlang
description: "Configure Erlang language support in Zed, including language servers, formatting, and debugging."
---

# Erlang

Erlang support is available through the [Erlang extension](https://github.com/zed-extensions/erlang).

- Tree-sitter: [WhatsApp/tree-sitter-erlang](https://github.com/WhatsApp/tree-sitter-erlang)
- Language Servers:
  - [WhatsApp/erlang-language-platform](https://github.com/WhatsApp/erlang-language-platform)
  - [erlang-ls/erlang_ls](https://github.com/erlang-ls/erlang_ls)

## Choosing a language server

The Erlang extension offers language server support for `erlang-language-platform` and `erlang_ls`.

`elp` is enabled by default.

Configure language servers in Settings ({#kb zed::OpenSettings}) under Languages > Erlang, or add to your settings file:

```json [settings]
{
  "languages": {
    "Erlang": {
      "language_servers": ["elp", "!erlang-ls", "..."]
    }
  }
}
```

## See also:

- [Elixir](./elixir.md)
- [Gleam](./gleam.md)
