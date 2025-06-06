# Diff

Diff support is available natively in CodeOrbit.

- Tree-sitter: [CodeOrbit-industries/the-mikedavis/tree-sitter-diff](https://github.com/the-mikedavis/tree-sitter-diff)

## Configuration

CodeOrbit will not attempt to format diff files and has [`remove_trailing_whitespace_on_save`](https://CodeOrbit.dev/docs/configuring-CodeOrbit#remove-trailing-whitespace-on-save) and [`ensure-final-newline-on-save`](https://CodeOrbit.dev/docs/configuring-CodeOrbit#ensure-final-newline-on-save) set to false.

CodeOrbit will automatically recognize files with `patch` and `diff` extensions as Diff files. To recognize other extensions, add them to `file_types` in your CodeOrbit settings.json:

```json
  "file_types": {
    "Diff": ["dif"]
  },
```
