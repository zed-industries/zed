---
title: Vue
description: "Configure Vue language support in Zed, including language servers, formatting, and debugging."
---

# Vue

Vue support is available through the [Vue extension](https://github.com/zed-extensions/vue).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-vue](https://github.com/tree-sitter-grammars/tree-sitter-vue)
- Language Server: [vuejs/language-tools](https://github.com/vuejs/language-tools)

## Initialization Options

### Specifying location of TypeScript SDK

By default, this extension assumes that you are working in a project with a `node_modules` directory, and searches for
the TypeScript SDK inside that directory.

This may not always be true; for example, when working in a project that uses Yarn PnP, there is no `node_modules`. For
editor support, the [documented](https://yarnpkg.com/getting-started/editor-sdks) approach is to run something like
`yarn dlx @yarnpkg/sdks`. In that case, you can provide the following initialization options in your Zed settings:

```json
{
  "lsp": {
    "vue": {
      "initialization_options": {
        "typescript": {
          "tsdk": ".yarn/sdks/typescript/lib"
        }
      }
    }
  }
}
```

## Settings Options

`lsp.vue.settings` is passed through to the Vue language server (Volar / [`vuejs/language-tools`](https://github.com/vuejs/language-tools)). The following settings are enabled by default:

```json
{
  "lsp": {
    "vue": {
      "settings": {
        // Display inlay hints for the `$event` parameter in inline event handlers.
        "vue.inlayHints.inlineHandlerLeading": true,
        // Display hints when required component props are missing in templates.
        "vue.inlayHints.missingProps": true,
        // Display inlay hints for patterns that wrap component options.
        "vue.inlayHints.optionsWrapper": true,
        // Display inlay hints related to `v-bind` shorthand (`:`).
        "vue.inlayHints.vBindShorthand": true
      }
    }
  }
}
```

You can find the upstream settings configuration schema [`here`](https://github.com/vuejs/language-tools/blob/ee5041d27940cf6f9a5150635d3b13140a9dff54/extensions/vscode/package.json#L252).

> Note: Some settings (e.g. `vue.editor.focusMode`) may not take effect.

## Using the Tailwind CSS Language Server with Vue

To get all the features (autocomplete, linting, etc.) from the [Tailwind CSS language server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in Vue files, you need to configure the language server so that it knows about where to look for CSS classes by adding the following to your `settings.json`:

```json [settings]
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "includeLanguages": {
          "vue": "html"
        },
        "experimental": {
          "classRegex": [
            "class=\"([^\"]*)\"",
            "class='([^']*)'",
            ":class=\"([^\"]*)\"",
            ":class='([^']*)'"
          ]
        }
      }
    }
  }
}
```

With these settings, you will get completions for Tailwind CSS classes in Vue template files. Examples:

```vue
<template>
  <!-- Static class attribute -->
  <div class="flex items-center <completion here>">
    <p class="text-lg font-bold <completion here>">Hello World</p>
  </div>

  <!-- Dynamic class binding -->
  <div
    :class="
      active ? 'bg-blue-500 <completion here>' : 'bg-gray-200 <completion here>'
    "
  >
    Content
  </div>

  <!-- Array syntax -->
  <div :class="['flex', 'items-center', '<completion here>']">Content</div>

  <!-- Object syntax -->
  <div
    :class="{
      'flex <completion here>': isFlex,
      'block <completion here>': isBlock,
    }"
  >
    Content
  </div>
</template>
```
