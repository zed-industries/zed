# Svelte

Svelte support is available through the [Svelte extension](https://github.com/zed-extensions/svelte).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-svelte](https://github.com/tree-sitter-grammars/tree-sitter-svelte)
- Language Server: [sveltejs/language-tools](https://github.com/sveltejs/language-tools)

## Extra theme styling configuration

You can modify how certain styles, such as directives and modifiers, appear in attributes:

```json [settings]
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

When inlay hints is enabled in Zed, to make the language server send them back, Zed sets the following initialization options:

```json [settings]
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

To override these settings, use the following:

```json [settings]
"lsp": {
  "svelte-language-server": {
    "initialization_options": {
      "configuration": {
        "typescript": {
          // ......
        },
        "javascript": {
          // ......
        }
      }
    }
  }
}
```

See [the TypeScript language server `package.json`](https://github.com/microsoft/vscode/blob/main/extensions/typescript-language-features/package.json) for more information.

## Using the Tailwind CSS Language Server with Svelte

To get all the features (autocomplete, linting, etc.) from the [Tailwind CSS language server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in Svelte files, you need to configure the language server so that it knows about where to look for CSS classes by adding the following to your `settings.json`:

```json [settings]
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "includeLanguages": {
          "svelte": "html"
        },
        "experimental": {
          "classRegex": [
            "class=\"([^\"]*)\"",
            "class='([^']*)'",
            "class:\\s*([^\\s{]+)",
            "\\{\\s*class:\\s*\"([^\"]*)\"",
            "\\{\\s*class:\\s*'([^']*)'"
          ]
        }
      }
    }
  }
}
```

With these settings, you will get completions for Tailwind CSS classes in Svelte files. Examples:

```svelte
<!-- Standard class attribute -->
<div class="flex items-center <completion here>">
  <p class="text-lg font-bold <completion here>">Hello World</p>
</div>

<!-- Class directive -->
<button class:active="bg-blue-500 <completion here>">Click me</button>

<!-- Expression -->
<div class={active ? "flex <completion here>" : "hidden <completion here>"}>
  Content
</div>
```
