---
title: TOML
description: "Configure TOML language support in Zed, including language servers, formatting, and debugging."
---

# TOML

TOML support is available through the [TOML extension](https://zed.dev/extensions/toml).

- Tree-sitter: [tree-sitter/tree-sitter-toml](https://github.com/tree-sitter/tree-sitter-toml)

## Language server

A TOML language server is available in the [Tombi extension](https://zed.dev/extensions/tombi).

<div class="warning">

Tombi replies to language server requests for definitions by opening a scratch JSON buffer with the schema definition for some known formats (`Cargo.toml`, `pyproject.toml`, ...). Since [Edit Predictions](/docs/ai/edit-prediction) rely on looking up definitions, this leads to tabs opening when you edit the TOML file.

You can fix it by [disabling definition requests](https://tombi-toml.github.io/tombi/docs/configuration/#lsp-goto-definition-enabled) in your Tombi configuration:

```toml
[lsp]
goto-type-definition.enabled = false
```

Alternatively, you can disable edit predictions in TOML buffers:

```json
{
  "languages": {
    "TOML": {
      "show_edit_predictions": false // https://github.com/tombi-toml/tombi/issues/1556
    }
  }
}
```

</div>
