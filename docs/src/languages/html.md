# HTML

HTML support is available through the [HTML extension](https://github.com/zed-industries/zed/tree/main/extensions/html).

- Tree-sitter: [tree-sitter/tree-sitter-html](https://github.com/tree-sitter/tree-sitter-html)
- Language Server: [microsoft/vscode-html-languageservice](https://github.com/microsoft/vscode-html-languageservice)

This extension is automatically installed, but if you do not want to use it, you can add the following to your settings:

```json [settings]
{
  "auto_install_extensions": {
    "html": false
  }
}
```

## Formatting

By default Zed uses [Prettier](https://prettier.io/) for formatting HTML.

You can disable `format_on_save` by adding the following to your Zed `settings.json`:

```json [settings]
  "languages": {
    "HTML": {
      "format_on_save": "off",
    }
  }
```

You can still trigger formatting manually with {#kb editor::Format} or by opening [the Command Palette](..//getting-started.md#command-palette) ({#kb command_palette::Toggle}) and selecting "Format Document".

### LSP Formatting

To use the `vscode-html-language-server` language server auto-formatting instead of Prettier, add the following to your Zed settings:

```json [settings]
  "languages": {
    "HTML": {
      "formatter": "language_server",
    }
  }
```

You can customize various [formatting options](https://code.visualstudio.com/docs/languages/html#_formatting) for `vscode-html-language-server` via your Zed `settings.json`:

```json [settings]
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

## Using the Tailwind CSS Language Server with HTML

To get all the features (autocomplete, linting, etc.) from the [Tailwind CSS language server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in HTML files, you need to configure the language server so that it knows about where to look for CSS classes by adding the following to your `settings.json`:

```json [settings]
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "experimental": {
          "classRegex": ["class=\"([^\"]*)\""]
        }
      }
    }
  }
}
```

With these settings, you will get completions for Tailwind CSS classes in HTML `class` attributes. Examples:

```html
<div class="flex items-center <completion here>">
  <p class="text-lg font-bold <completion here>">Hello World</p>
</div>
```

## See also

- [CSS](./css.md)
- [JavaScript](./javascript.md)
- [TypeScript](./typescript.md)
