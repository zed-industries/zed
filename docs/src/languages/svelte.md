# Svelte

Svelte support is available through the [Svelte extension](https://github.com/zed-industries/zed/tree/main/extensions/svelte).

- Tree Sitter: [Himujjal/tree-sitter-svelte](https://github.com/Himujjal/tree-sitter-svelte)
- Language Server: [sveltejs/language-tools](https://github.com/sveltejs/language-tools)

<!--
TBD: Rewrite Svelte docs so it doesn't begin with a json block assuming you know what inlayHints are.
-->

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
  "svelte-language-server": {
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
