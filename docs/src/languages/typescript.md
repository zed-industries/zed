# TypeScript

TypeScript and TSX support are available natively in Zed.

- Tree-sitter: [tree-sitter/tree-sitter-typescript](https://github.com/tree-sitter/tree-sitter-typescript)
- Language Server: [yioneko/vtsls](https://github.com/yioneko/vtsls)
- Alternate Language Server: [typescript-language-server/typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)
- Debug Adapter: [vscode-js-debug](https://github.com/microsoft/vscode-js-debug)

<!--
TBD: Document the difference between Language servers
-->

## Language servers

By default Zed uses [vtsls](https://github.com/yioneko/vtsls) for TypeScript, TSX, and JavaScript files.
You can configure the use of [typescript-language-server](https://github.com/typescript-language-server/typescript-language-server) per language in your settings file:

```json [settings]
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

```json [settings]
{
  "languages": {
    "TypeScript": {
      "prettier": { "allowed": false }
    }
    //...
  }
}
```

## Large projects

`vtsls` may run out of memory on very large projects. We default the limit to 8092 (8 GiB) vs. the default of 3072 but this may not be sufficient for you:

```json [settings]
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

## Inlay Hints

Zed sets the following initialization options to make the language server send back inlay hints (that is, when Zed has inlay hints enabled in the settings).

You can override these settings in your Zed `settings.json` when using `typescript-language-server`:

```json [settings]
{
  "lsp": {
    "typescript-language-server": {
      "initialization_options": {
        "preferences": {
          "includeInlayParameterNameHints": "all",
          "includeInlayParameterNameHintsWhenArgumentMatchesName": true,
          "includeInlayFunctionParameterTypeHints": true,
          "includeInlayVariableTypeHints": true,
          "includeInlayVariableTypeHintsWhenTypeMatchesName": true,
          "includeInlayPropertyDeclarationTypeHints": true,
          "includeInlayFunctionLikeReturnTypeHints": true,
          "includeInlayEnumMemberValueHints": true
        }
      }
    }
  }
}
```

See [typescript-language-server inlayhints documentation](https://github.com/typescript-language-server/typescript-language-server?tab=readme-ov-file#inlay-hints-textdocumentinlayhint) for more information.

When using `vtsls`:

```json [settings]
{
  "lsp": {
    "vtsls": {
      "settings": {
        // For JavaScript:
        "javascript": {
          "inlayHints": {
            "parameterNames": {
              "enabled": "all",
              "suppressWhenArgumentMatchesName": false
            },
            "parameterTypes": {
              "enabled": true
            },
            "variableTypes": {
              "enabled": true,
              "suppressWhenTypeMatchesName": true
            },
            "propertyDeclarationTypes": {
              "enabled": true
            },
            "functionLikeReturnTypes": {
              "enabled": true
            },
            "enumMemberValues": {
              "enabled": true
            }
          }
        },
        // For TypeScript:
        "typescript": {
          "inlayHints": {
            "parameterNames": {
              "enabled": "all",
              "suppressWhenArgumentMatchesName": false
            },
            "parameterTypes": {
              "enabled": true
            },
            "variableTypes": {
              "enabled": true,
              "suppressWhenTypeMatchesName": true
            },
            "propertyDeclarationTypes": {
              "enabled": true
            },
            "functionLikeReturnTypes": {
              "enabled": true
            },
            "enumMemberValues": {
              "enabled": true
            }
          }
        }
      }
    }
  }
}
```

## Debugging

Zed supports debugging TypeScript code out of the box.
The following can be debugged without writing additional configuration:

- Tasks from `package.json`
- Tests written using several popular frameworks (Jest, Mocha, Vitest, Jasmine, Bun, Node)

Run {#action debugger::Start} ({#kb debugger::Start}) to see a contextual list of these predefined debug tasks.

> **Note:** Bun test is automatically detected when `@types/bun` is present in `package.json`.
>
> **Note:** Node test is automatically detected when `@types/node` is present in `package.json` (requires Node.js 20+).

As for all languages, configurations from `.vscode/launch.json` are also available for debugging in Zed.

If your use-case isn't covered by any of these, you can take full control by adding debug configurations to `.zed/debug.json`. See below for example configurations.

### Attach debugger to a server running in web browser (`npx serve`)

Given an externally-ran web server (e.g., with `npx serve` or `npx live-server`) one can attach to it and open it with a browser.

```json [debug]
[
  {
    "label": "Launch Chrome (TypeScript)",
    "adapter": "JavaScript",
    "type": "chrome",
    "request": "launch",
    "url": "http://localhost:5500",
    "program": "$ZED_FILE",
    "webRoot": "${ZED_WORKTREE_ROOT}",
    "build": {
      "command": "npx",
      "args": ["tsc"]
    },
    "skipFiles": ["<node_internals>/**"]
  }
]
```

## See also

- [Zed Yarn documentation](./yarn.md) for a walkthrough of configuring your project to use Yarn.
- [Zed Deno documentation](./deno.md)
