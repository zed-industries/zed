# TypeScript

TypeScript and TSX support are available natively in Zed.

- Tree Sitter: [tree-sitter/tree-sitter-typescript](https://github.com/tree-sitter/tree-sitter-typescript)
- Language Server: [yioneko/vtsls](https://github.com/yioneko/vtsls)
- Alternate Language Server: [typescript-language-server/typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)

<!--
TBD: Document the difference between Language servers
-->

## Language servers

By default Zed uses [vtsls](https://github.com/yioneko/vtsls) for TypeScript, TSX and JavaScript files.
You can configure the use of [typescript-language-server](https://github.com/typescript-language-server/typescript-language-server) per language in your settings file:

```json
{
  "languages": {
    "TypeScript": {
      "language_servers": ["typescript-language-server", "!vtsls", "..."]
    },
    "TSX": {
      "language_servers": ["typescript-language-server", "!vtsls", "..."]
    },
    "JavaScript": {
      "language_servers": ["typescript-language-server", "!vtsls", "..."]
    }
  }
}
```

Prettier will also be used for TypeScript files by default. To disable this:

```jsonc
{
  "languages": {
    "TypeScript": {
      "prettier": { "allowed": false },
    },
    //...
  },
}
```

## Large projects

`vtsls` may run out of memory on very large projects. We default the limit to 8092 (8 GiB) vs. the default of 3072 but this may not be sufficient for you:

```json
{
  "lsp": {
    "vtsls": {
      "settings": {
        // For TypeScript:
        "typescript": { "tsserver": { "maxTsServerMemory": 16184 } },
        // For JavaScript:
        "javascript": { "tsserver": { "maxTsServerMemory": 16184 } }
      }
    }
  }
}
```

## See also

- [Zed Yarn documentation](./yarn.md) for a walkthrough of configuring your project to use Yarn.
- [Zed Deno documentation](./deno.md)
