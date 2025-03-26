# HTML

HTML support is available through the [HTML extension](https://github.com/zed-industries/zed/tree/main/extensions/html).

- Tree-sitter: [tree-sitter/tree-sitter-html](https://github.com/tree-sitter/tree-sitter-html)
- Language Server: [microsoft/vscode-html-languageservice](https://github.com/microsoft/vscode-html-languageservice)

This extension is automatically installed.

If you do not want to use the HTML extension, you can add the following to your settings:

```json
{
  "auto_install_extensions": {
    "html": false
  }
}
```

## Formatting

By default Zed will use Prettier for formatting HTML but if you prefer you can alternately use `vscode-html-language-server` by adding the following to your Zed settings:

```json
  "languages": {
    "HTML": {
      "formatter": "language_server",
      "prettier": {
        "allowed": false
      }
    }
  }
```

## See also:

- [CSS](./css.md)
- [JavaScript](./javascript.md)
- [TypeScript](./typescript.md)
