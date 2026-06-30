---
title: Gleam
description: "Configure Gleam language support in Zed, including language servers, formatting, and debugging."
---

# Gleam

Gleam support is available through the [Gleam extension](https://github.com/gleam-lang/zed-gleam). To learn about Gleam, see the [docs](https://gleam.run/documentation/) or check out the [`stdlib` reference](https://hexdocs.pm/gleam_stdlib/). The Gleam language server has a variety of features, including go-to definition, automatic imports, and [more](https://gleam.run/language-server/).

- Tree-sitter: [gleam-lang/tree-sitter-gleam](https://github.com/gleam-lang/tree-sitter-gleam)
- Language Server: [gleam lsp](https://github.com/gleam-lang/gleam/tree/main/compiler-core/src/language_server)

## Using the Tailwind CSS Language Server with Lustre

[Lustre](https://github.com/lustre-labs/lustre) is a frontend framework for Gleam. Lustre's `attribute.class("...")` calls take a string of class names that are not picked up by Tailwind by default. To get autocomplete and linting from the [Tailwind CSS language server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) inside Gleam files, add an `experimental.classRegex` entry that matches any quoted string and configure the language server in your `settings.json`:

```json [settings]
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "includeLanguages": {
          "gleam": "html"
        },
        "experimental": {
          "classRegex": ["\"([^\"]*)\""]
        }
      }
    }
  }
}
```

The regex above is intentionally broad - it matches any double-quoted string in a Gleam file - so completions will trigger inside non-class strings as well. Tighten the pattern (e.g. to only match `class\\(\"([^\"]*)\"\\)`) if that becomes noisy in your project.

See also:

- [Elixir](./elixir.md)
- [Erlang](./erlang.md)
