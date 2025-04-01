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

By default Zed uses [Prettier](https://prettier.io/) for formatting HTML

You can disable `format_on_save` by adding the following to your Zed settings:

```json
  "languages": {
    "HTML": {
      "format_on_save": "off",
    }
  }
```

You can still trigger formatting manually with {#kb editor::Format} or by opening the command palette ( {#kb commandPalette::Toggle} and selecting `Format Document`.

### LSP Formatting

If you prefer you can use `vscode-html-language-server` instead of Prettier for auto-formatting by adding the following to your Zed settings:

```json
  "languages": {
    "HTML": {
      "formatter": "language_server",
    }
  }
```

You can customize various [formatting options](https://code.visualstudio.com/docs/languages/html#_formatting) for `vscode-html-language-server` via Zed settings.json:

```json
  "lsp": {
    "vscode-html-language-server": {
      "settings": {
        "html": {
          "format": {
            // Indent under <html> and <head> (default: false)
            "indentInnerHtml": true,
            // Disable formatting inside <svg> or <script>
            "contentUnformatted": "svg,script",
            // Add an extra newline before <div> and <p>
            "extraLiners": "div,p"
          }
        }
      }
    }
  }
```

## See also:

- [CSS](./css.md)
- [JavaScript](./javascript.md)
- [TypeScript](./typescript.md)
