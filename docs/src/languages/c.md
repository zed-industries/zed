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
