# Svelte

Svelte support is available through the [Svelte extension](https://github.com/zed-industries/zed/tree/main/extensions/svelte).

## Inlay Hints

Zed sets the following initialization options for inlay Hints:

```json
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
    "suppressWhenTypeMatchesName": false
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
```

to make the language server send back inlay hints when Zed has them enabled in the settings.

Use

```json
"lsp": {
  "$LANGUAGE_SERVER_NAME": {
    "initialization_options": {
      "configuration": {
        "typescript": {
          ......
        },
        "javascript": {
          ......
        }
      }
    }
  }
}
```

to override these settings.

See https://github.com/microsoft/vscode/blob/main/extensions/typescript-language-features/package.json for more information.
