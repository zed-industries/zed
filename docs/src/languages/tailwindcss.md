# Tailwind CSS

Zed has built-in support for Tailwind CSS autocomplete, linting, and hover previews.

- Language Server: [tailwindlabs/tailwindcss-intellisense](https://github.com/tailwindlabs/tailwindcss-intellisense)

Languages which can be used with Tailwind CSS in Zed:

- [Astro](./astro.md)
- [CSS](./css.md)
- [ERB](./ruby.md)
- [Gleam](./gleam.md)
- [HEEx](./elixir.md#heex)
- [HTML](./html.md)
- [TypeScript](./typescript.md)
- [JavaScript](./javascript.md)
- [PHP](./php.md)
- [Svelte](./svelte.md)
- [Vue](./vue.md)

## Configuration

If by default the language server isn't enough to make Tailwind work for a given language, you can configure the language server settings and add them to the `lsp` section of your `settings.json`:

```json [settings]
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "classFunctions": ["cva", "cx"],
        "experimental": {
          "classRegex": ["[cls|className]\\s\\:\\=\\s\"([^\"]*)"]
        }
      }
    }
  }
}
```

Refer to [the Tailwind CSS language server settings docs](https://github.com/tailwindlabs/tailwindcss-intellisense?tab=readme-ov-file#extension-settings) for more information.

### Using Tailwind CSS Mode in CSS Files

Zed includes support for the Tailwind CSS language mode, which provides full CSS IntelliSense support even when using Tailwind-specific at-rules like `@apply`, `@layer`, and `@theme`.
To use the Tailwind CSS language mode for CSS files, add the following to `languages` section of your `settings.json`:

```json [settings]
{
  "languages": {
    "CSS": {
      "language_servers": [
        "tailwindcss-intellisense-css",
        "!vscode-css-language-server",
        "..."
      ]
    }
  }
}
```

The `tailwindcss-intellisense-css` language server serves as an alternative to the default CSS language server, maintaining all standard CSS IntelliSense features while adding support for Tailwind-specific syntax.

### Prettier Plugin

Zed supports Prettier out of the box, which means that if you have the [Tailwind CSS Prettier plugin](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) installed, adding it to your Prettier configuration will make it work automatically:

```json [settings]
// .prettierrc
{
  "plugins": ["prettier-plugin-tailwindcss"]
}
```
