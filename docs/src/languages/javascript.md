# JavaScript

- Tree Sitter: [tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript)
- Language Server: [typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)

### Code formatting

Formatting on save is enabled by default for JavaScript, using TypeScript's built-in code formatting. But many JavaScript projects use other command-line code-formatting tools, such as [Prettier](https://prettier.io/). You can use one of these tools by specifying an _external_ code formatter for JavaScript in your settings. See the [configuration](../configuration/configuring-zed.md) documentation for more information.

For example, if you have Prettier installed and on your `PATH`, you can use it to format JavaScript files by adding the following to your `settings.json`:

```json
{
  "language_overrides": {
    "JavaScript": {
      "format_on_save": {
        "external": {
          "command": "prettier",
          "arguments": ["--stdin-filepath", "{buffer_path}"]
        }
      }
    }
  }
}
```

### ESLint

You can configure Zed to format code using `eslint --fix` by running the ESLint
code action when formatting (requires Zed `0.125.0`):

```json
{
  "languages": {
    "JavaScript": {
      "code_actions_on_format": {
        "source.fixAll.eslint": true
      }
    }
  }
}
```

You can also only execute a single ESLint rule when using `fixAll`:

```json
{
  "languages": {
    "JavaScript": {
      "code_actions_on_format": {
        "source.fixAll.eslint": true
      }
    }
  },
  "lsp": {
    "eslint": {
      "settings": {
        "codeActionOnSave": {
          "rules": ["import/order"]
        }
      }
    }
  }
}
```

**Note:** the other formatter you have configured will still run, after ESLint.
So if your language server or prettier configuration don't format according to
ESLint's rules, then they will overwrite what ESLint fixed and you end up with
errors.

#### Configure ESLint's `nodePath`:

You can configure ESLint's `nodePath` setting (requires Zed `0.127.0`):

```json

{
  "lsp": {
    "eslint": {
      "settings": {
        "nodePath": ".yarn/sdks"
      }
    }
  }
}
```
