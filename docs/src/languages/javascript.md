# JavaScript

JavaScript support is available natively in Zed.

- Tree-sitter: [tree-sitter/tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript)
- Language Server: [typescript-language-server/typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)
- Debug Adapter: [vscode-js-debug](https://github.com/microsoft/vscode-js-debug)

## Code formatting

Formatting on save is enabled by default for JavaScript, using TypeScript's built-in code formatting.
But many JavaScript projects use other command-line code-formatting tools, such as [Prettier](https://prettier.io/).
You can use one of these tools by specifying an _external_ code formatter for JavaScript in your settings.
See [the configuration docs](../configuring-zed.md) for more information.

For example, if you have Prettier installed and on your `PATH`, you can use it to format JavaScript files by adding the following to your `settings.json`:

```json [settings]
{
  "languages": {
    "JavaScript": {
      "formatter": {
        "external": {
          "command": "prettier",
          "arguments": ["--stdin-filepath", "{buffer_path}"]
        }
      }
    }
  }
}
```

## JSX

Zed supports JSX syntax highlighting out of the box.

In JSX strings, the [`tailwindcss-language-server`](./tailwindcss.md) is used to provide autocompletion for Tailwind CSS classes.

## JSDoc

Zed supports JSDoc syntax in JavaScript and TypeScript comments that match the JSDoc syntax.
Zed uses [tree-sitter/tree-sitter-jsdoc](https://github.com/tree-sitter/tree-sitter-jsdoc) for parsing and highlighting JSDoc.

## ESLint

You can configure Zed to format code using `eslint --fix` by running the ESLint code action when formatting:

```json [settings]
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

```json [settings]
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

> **Note:** the other formatter you have configured will still run, after ESLint.
> So if your language server or Prettier configuration don't format according to
> ESLint's rules, then they will overwrite what ESLint fixed and you end up with
> errors.

If you **only** want to run ESLint on save, you can configure code actions as
the formatter:

```json [settings]
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

### Configure ESLint's `nodePath`:

You can configure ESLint's `nodePath` setting:

```json [settings]
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

### Configure ESLint's `problems`:

You can configure ESLint's `problems` setting.

For example, here's how to set `problems.shortenToSingleLine`:

```json [settings]
{
  "lsp": {
    "eslint": {
      "settings": {
        "problems": {
          "shortenToSingleLine": true
        }
      }
    }
  }
}
```

### Configure ESLint's `rulesCustomizations`:

You can configure ESLint's `rulesCustomizations` setting:

```json [settings]
{
  "lsp": {
    "eslint": {
      "settings": {
        "rulesCustomizations": [
          // set all eslint errors/warnings to show as warnings
          { "rule": "*", "severity": "warn" }
        ]
      }
    }
  }
}
```

### Configure ESLint's `workingDirectory`:

You can configure ESLint's `workingDirectory` setting:

```json [settings]
{
  "lsp": {
    "eslint": {
      "settings": {
        "workingDirectory": {
          "mode": "auto"
        }
      }
    }
  }
}
```

## Debugging

Zed supports debugging JavaScript code out of the box with `vscode-js-debug`.
The following can be debugged without writing additional configuration:

- Tasks from `package.json`
- Tests written using several popular frameworks (Jest, Mocha, Vitest, Jasmine, Bun, Node)

Run {#action debugger::Start} ({#kb debugger::Start}) to see a contextual list of these predefined debug tasks.

> **Note:** Bun test is automatically detected when `@types/bun` is present in `package.json`.
>
> **Note:** Node test is automatically detected when `@types/node` is present in `package.json` (requires Node.js 20+).

As for all languages, configurations from `.vscode/launch.json` are also available for debugging in Zed.

If your use-case isn't covered by any of these, you can take full control by adding debug configurations to `.zed/debug.json`. See below for example configurations.

### Configuring JavaScript debug tasks

JavaScript debugging is more complicated than other languages because there are two different environments: Node.js and the browser. `vscode-js-debug` exposes a `type` field, that you can use to specify the environment, either `node` or `chrome`.

- [vscode-js-debug configuration documentation](https://github.com/microsoft/vscode-js-debug/blob/main/OPTIONS.md)

### Debug the current file with Node

```json [debug]
[
  {
    "adapter": "JavaScript",
    "label": "Debug JS file",
    "type": "node",
    "request": "launch",
    "program": "$ZED_FILE",
    "skipFiles": ["<node_internals>/**"]
  }
]
```

### Launch a web app in Chrome

```json [debug]
[
  {
    "adapter": "JavaScript",
    "label": "Debug app in Chrome",
    "type": "chrome",
    "request": "launch",
    "file": "$ZED_WORKTREE_ROOT/index.html",
    "webRoot": "$ZED_WORKTREE_ROOT",
    "console": "integratedTerminal",
    "skipFiles": ["<node_internals>/**"]
  }
]
```

## See also

- [Yarn documentation](./yarn.md) for a walkthrough of configuring your project to use Yarn.
- [TypeScript documentation](./typescript.md)
