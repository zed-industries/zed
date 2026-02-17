# Vue

Vue support is available through the [Vue extension](https://github.com/zed-extensions/vue).

- Tree-sitter: [tree-sitter-grammars/tree-sitter-vue](https://github.com/tree-sitter-grammars/tree-sitter-vue)
- Language Server: [vuejs/language-tools/](https://github.com/vuejs/language-tools/)

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
