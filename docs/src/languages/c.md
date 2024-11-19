# C

C support is available natively in Zed.

- Tree Sitter: [tree-sitter/tree-sitter-c](https://github.com/tree-sitter/tree-sitter-c)
- Language Server: [clangd/clangd](https://github.com/clangd/clangd)

## Clangd: Force detect as C

Clangd out of the box assumes mixed C++/C projects. If you have a C-only project you may wish to instruct clangd to all files as C using the `-xc` flag. To do this, create a `.clangd` file in the root of your project with the following:

```yaml
CompileFlags:
  Add: [-xc]
```

## Formatting

By default Zed will use the `clangd` language server for formatting C code. The Clangd is the same as the `clang-format` CLI tool. To configure this you can add a `.clang-format` file. For example:

```yaml
---
BasedOnStyle: GNU
IndentWidth: 2
---
```

See [Clang-Format Style Options](https://clang.llvm.org/docs/ClangFormatStyleOptions.html) for a complete list of options.

You can trigger formatting via {#kb editor::Format} or the `editor: format` action from the command palette or by adding `format_on_save` to your Zed settings:

```json
  "languages": {
    "C" {
      "format_on_save": "on",
      "tab_size": 2
    }
  }
```

See [Clang-Format Style Options](https://clang.llvm.org/docs/ClangFormatStyleOptions.html) for a complete list of options.
