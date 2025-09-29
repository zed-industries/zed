# Deno

Deno support is available through the [Deno extension](https://github.com/zed-extensions/deno).

- Language server: [Deno Language Server](https://docs.deno.com/runtime/manual/advanced/language_server/overview/)

## Deno Configuration

To use the Deno Language Server with TypeScript and TSX files, you will likely wish to disable the default language servers and enable deno by adding the following to your `settings.json`:

```json
{
  "lsp": {
    "deno": {
      "settings": {
        "deno": {
          "enable": true
        }
      }
    }
  },
  "languages": {
    "JavaScript": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint"
      ],
      "formatter": "language_server"
    },
    "TypeScript": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint"
      ],
      "formatter": "language_server"
    },
    "TSX": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint"
      ],
      "formatter": "language_server"
    }
  }
}
```

See [Configuring supported languages](../configuring-languages.md) in the Zed documentation for more information.

<!--
TBD: Deno TypeScript REPL instructions [docs/repl#typescript-deno](../repl.md#typescript-deno)
-->

## Configuration completion

To get completions for `deno.json` or `package.json` you can add the following to your `settings.json`: (More info here https://zed.dev/docs/languages/json)

```json
"lsp": {
    "json-language-server": {
      "settings": {
        "json": {
          "schemas": [
            {
              "fileMatch": [
                "deno.json"
              ],
              "url": "https://raw.githubusercontent.com/denoland/deno/refs/heads/main/cli/schemas/config-file.v1.json"
            },
            {
              "fileMatch": [
                "package.json"
              ],
              "url": "http://json.schemastore.org/package"
            }
          ]
        }
      }
    }
  }
```

## DAP support

To debug deno programs, add this to `.zed/debug.json`

```json
[
  {
    "adapter": "JavaScript",
    "label": "Deno",
    "request": "launch",
    "type": "pwa-node",
    "cwd": "$ZED_WORKTREE_ROOT",
    "program": "$ZED_FILE",
    "runtimeExecutable": "deno",
    "runtimeArgs": ["run", "--allow-all", "--inspect-wait"],
    "attachSimplePort": 9229
  }
]
```

## Runnable support

To run deno tasks like tests from the ui, add this to `.zed/tasks.json`

```json
[
  {
    "label": "deno test",
    "command": "deno test -A --filter '/^$ZED_CUSTOM_DENO_TEST_NAME$/' $ZED_FILE",
    "tags": ["js-test"]
  }
]
```

## See also:

- [TypeScript](./typescript.md)
- [JavaScript](./javascript.md)
