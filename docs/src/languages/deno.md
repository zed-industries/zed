# Deno

Deno support is available through the [Deno extension](https://github.com/zed-industries/zed/tree/main/extensions/deno).

- Language server: [Deno Language Server](https://docs.deno.com/runtime/manual/advanced/language_server/overview/)

## Deno Configuration

To use the Deno Language Server with TypeScript and TSX files, you will likely wish to to disable the default language servers and enable deno by adding the following to your settings.json:

```json
{
  "languages": {
    "TypeScript": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint"
      ]
    },
    "TSX": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint"
      ]
    }
  }
}
```

See [Configuring supported languages](../configuring-languages.md) in the Zed documentation for more information.

<!--
TBD: Deno Typescript REPL instructions [docs/repl#typescript-deno](../repl.md#typescript-deno)
-->

## See also:

- [TypeScript](./typescript.md)
- [JavaScript](./javascript.md)
