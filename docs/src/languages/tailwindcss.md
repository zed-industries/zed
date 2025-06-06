# Tailwind CSS

CodeOrbit has built-in support for Tailwind CSS autocomplete, linting, and hover previews.

- Language Server: [tailwindlabs/tailwindcss-intellisense](https://github.com/tailwindlabs/tailwindcss-intellisense)

## Configuration

<!--
TBD: Document Tailwind CSS Configuration
-->

Languages which can be used with Tailwind CSS in CodeOrbit:

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

CodeOrbit supports Prettier out of the box, which means that if you have the [Tailwind CSS Prettier plugin](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) installed, adding it to your Prettier configuration will make it work automatically:

```json
// .prettierrc
{
  "plugins": ["prettier-plugin-tailwindcss"]
}
```
