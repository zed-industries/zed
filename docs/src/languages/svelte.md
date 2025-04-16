# Svelte

Svelte support is available through the [Svelte extension](https://github.com/zed-extensions/svelte).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-svelte](https://github.com/tree-sitter-grammars/tree-sitter-svelte)
- Language Server: [sveltejs/language-tools](https://github.com/sveltejs/language-tools)

## Extra theme styling configuration

You can modify how certain styles such as directives and modifiers appear in attributes:

```json
"syntax": {
  // Styling for directives (e.g., `class:foo` or `on:click`) (the `on` or `class` part of the attribute).
  "attribute.function": {
    "color": "#ff0000"
  },
  // Styling for modifiers at the end of attributes, e.g. `on:<click|preventDefault|stopPropagation>`
  "attribute.special": {
    "color": "#00ff00"
  }
}
```

## Inlay Hints

Zed sets the following initialization options for inlay hints:

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
