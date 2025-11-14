# Erlang

Erlang support is available through the [Erlang extension](https://github.com/zed-extensions/erlang).

- Tree-sitter: [WhatsApp/tree-sitter-erlang](https://github.com/WhatsApp/tree-sitter-erlang)
- Language Servers:
  - [erlang-ls/erlang_ls](https://github.com/erlang-ls/erlang_ls)
  - [WhatsApp/erlang-language-platform](https://github.com/WhatsApp/erlang-language-platform)

## Choosing a language server

The Erlang extension offers language server support for `erlang_ls` and `erlang-language-platform`.

`erlang_ls` is enabled by default.

To switch to `erlang-language-platform`, add the following to your `settings.json`:

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
