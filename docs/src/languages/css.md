---
title: CSS
description: "Configure CSS language support in Zed, including language servers, formatting, and debugging."
---

# CSS

Zed has built-in support for CSS.

- Tree-sitter: [tree-sitter/tree-sitter-css](https://github.com/tree-sitter/tree-sitter-css)
- Language Servers:
  - [microsoft/vscode-css-languageservice](https://github.com/microsoft/vscode-css-languageservice)
  - [tailwindcss-language-server](https://github.com/tailwindlabs/tailwindcss-intellisense)

## Tailwind CSS

Zed also supports [Tailwind CSS](./tailwindcss.md) out-of-the-box. To use Tailwind CSS IntelliSense in CSS files, configure the Tailwind CSS language server for the `CSS` language and disable the default CSS language server. See the [docs page](./tailwindcss.md#L45) for Tailwind CSS for more detail.

This enables autocomplete, diagnostics, and hover previews for Tailwind-specific CSS such as `@apply`, `@layer`, and `@theme`. The `tailwindcss-intellisense-css` language server is provided by Zed's built-in Tailwind CSS capabilities, and is an alternative to the default CSS language server; do not enable both at the same time.

For Tailwind CSS classes in other languages and frameworks, see the [language-specific configuration examples](./tailwindcss.md).

## Recommended Reading

- [HTML](./html.md)
- [TypeScript](./typescript.md)
- [JavaScript](./javascript.md)
