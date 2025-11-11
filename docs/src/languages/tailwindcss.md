# Tailwind CSS

Zed has built-in support for Tailwind CSS autocomplete, linting, and hover previews.

- Language Server: [tailwindlabs/tailwindcss-intellisense](https://github.com/tailwindlabs/tailwindcss-intellisense)

## Configuration

To configure the Tailwind CSS language server, refer [to the extension settings](https://github.com/tailwindlabs/tailwindcss-intellisense?tab=readme-ov-file#extension-settings) and add them to the `lsp` section of your `settings.json`:

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

Languages which can be used with Tailwind CSS in Zed:

- [Astro](./astro.md)
- [CSS](./css.md)
- [ERB](./ruby.md)
- [HEEx](./elixir.md#heex)
- [HTML](./html.md)
- [TypeScript](./typescript.md)
- [JavaScript](./javascript.md)
- [PHP](./php.md)
- [Svelte](./svelte.md)
- [Vue](./vue.md)

### Prettier Plugin

Zed supports Prettier out of the box, which means that if you have the [Tailwind CSS Prettier plugin](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) installed, adding it to your Prettier configuration will make it work automatically:

```json [settings]
// .prettierrc
{
  "plugins": ["prettier-plugin-tailwindcss"]
}
```
