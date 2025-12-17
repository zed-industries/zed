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

It's possible to use the [Tailwind CSS Language Server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in HTML files.

You can follow the following setups:
 * [PostCSS Tailwind Guide](https://tailwindcss.com/docs/installation/using-postcss)
 * [Tailwind CLI Guide](https://tailwindcss.com/docs/installation/tailwind-cli)
 * [Tailwind Play CDN](https://tailwindcss.com/docs/installation/play-cdn)

We will follow the CDN approach for this example for brevity.

Note: "The Play CDN is designed for development purposes only, and is not intended for production."

Firstly you need to update your `settings.json`, to include the
Tailwind CSS language server for HTML files and configure the `classRegex`
settings to recognize Tailwind CSS classes in HTML:
```json [settings]
{
  "languages": {
    "HTML": {
      "language_servers": ["tailwindcss-language-server", "..."]
    }
  },
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "experimental": {
          "classRegex": [
            "class\\s*=\\s*['\"]([^'\"]*)['\"]"
          ]
        }
      }
    }
  }
}
```

Then you need to include the Tailwind Play CDN script in your HTML file,
then you can use Tailwind CSS classes as usual:

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
        <title>Tailwind CSS with Zed</title>
        <style type="text/tailwindcss">
            @theme {
                --color-primary: #1DA1F2;
            }
        </style>
    </head>
    <body>
       <h1 class="text-3xl font-bold underline text-primary <completion here>">
           Hello, Tailwind CSS!
       </h1>
    </body>
</html>
```


## See also

- [CSS](./css.md)
- [JavaScript](./javascript.md)
- [TypeScript](./typescript.md)
