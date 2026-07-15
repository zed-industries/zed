---
title: Gleam
description: "Configure Gleam language support in Zed, including language servers, formatting, and debugging."
---

# Gleam

Gleam support is available through the [Gleam extension](https://github.com/gleam-lang/zed-gleam). To learn about Gleam, see the [docs](https://gleam.run/documentation/) or check out the [`stdlib` reference](https://hexdocs.pm/gleam_stdlib/). The Gleam language server has a variety of features, including go-to definition, automatic imports, and [more](https://gleam.run/language-server/).

- Tree-sitter: [gleam-lang/tree-sitter-gleam](https://github.com/gleam-lang/tree-sitter-gleam)
- Language Server: [gleam lsp](https://github.com/gleam-lang/gleam/tree/main/compiler-core/src/language_server)

## Using the Tailwind CSS Language Server with Gleam

To get all the features (autocomplete, linting, etc.) from the [Tailwind CSS language server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in Gleam files, you need to enable the language server for Gleam and configure where it should look for CSS classes by adding the following to your `settings.json`:

```json [settings]
{
  "languages": {
    "Gleam": {
      "language_servers": ["tailwindcss-language-server", "..."]
    }
  },
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "experimental": {
          "classRegex": ["\"([^\"]*)\""]
        }
      }
    }
  }
}
```

This works with plain string literals and with [Lustre](https://github.com/lustre-labs/lustre) view templates where class names are passed as string arguments.

See also:

- [Elixir](./elixir.md)
- [Erlang](./erlang.md)
