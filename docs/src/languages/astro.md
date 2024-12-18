# Astro

Astro support is available through the [Astro extension](https://github.com/zed-industries/zed/tree/main/extensions/astro).

- Tree Sitter: [virchau13/tree-sitter-astro](https://github.com/virchau13/tree-sitter-astro)
- Language Server: [withastro/language-tools](https://github.com/withastro/language-tools)

## Astro Configuration

To enable importing Astro files in TypeScript and TSX files, you will likely wish to disable the default language servers by adding the following to your settings.json:

```jsonc
{
  "languages": {
    "TypeScript": {
      "language_servers": [
        "astro-typescript",
        "!typescript-language-server",
        "!vtsls",
        "!eslint",
      ],
    },
    "TSX": {
      "language_servers": [
        "astro-typescript",
        "!typescript-language-server",
        "!vtsls",
        "!eslint",
      ],
    },
  },
}
```

See [Configuring supported languages](../configuring-languages.md) in the Zed documentation for more information.
