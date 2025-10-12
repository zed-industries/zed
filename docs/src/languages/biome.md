# Biome

[Biome](https://biomejs.dev/) support in Zed is provided by the community-maintained [Biome extension](https://github.com/biomejs/biome-zed).
Report issues to: [https://github.com/biomejs/biome-zed/issues](https://github.com/biomejs/biome-zed/issues)

- Language Server: [biomejs/biome](https://github.com/biomejs/biome)

## Biome Language Support

The Biome extension includes support for the following languages:

- JavaScript
- TypeScript
- JSX
- TSX
- JSON
- JSONC
- Vue.js
- Astro
- Svelte
- CSS

## Configuration

By default, the `biome.json` file is required to be in the root of the workspace.

```json [settings]
{
  "$schema": "https://biomejs.dev/schemas/1.8.3/schema.json"
}
```

For a full list of `biome.json` options see [Biome Configuration](https://biomejs.dev/reference/configuration/) documentation.

See the [Biome Zed Extension README](https://github.com/biomejs/biome-zed) for a complete list of features and configuration options.
