---
title: Semantic Tokens and Syntax Highlighting - Zed
description: Enable and configure semantic token highlighting in Zed for richer, language-server-aware syntax coloring.
---

# Semantic Tokens

Semantic tokens provide richer syntax highlighting by using information from language servers. Unlike tree-sitter highlighting, which is based purely on syntax, semantic tokens understand the meaning of your code—distinguishing between local variables and parameters, or between a class definition and a class reference.

## Enabling Semantic Tokens

Semantic tokens are controlled by the `semantic_tokens` setting. By default, semantic tokens are disabled.

```json [settings]
{
  "semantic_tokens": "combined"
}
```

This setting accepts three values:

| Value        | Description                                                                                                                                                 |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"off"`      | Do not request semantic tokens from language servers. Uses tree-sitter highlighting only. (Default)                                                         |
| `"combined"` | Use LSP semantic tokens together with tree-sitter highlighting. Tree-sitter provides base highlighting, and semantic tokens overlay additional information. |
| `"full"`     | Use LSP semantic tokens exclusively. Tree-sitter highlighting is disabled entirely for buffers with semantic token support.                                 |

You can configure this globally or per-language:

```json [settings]
{
  "semantic_tokens": "off",
  "languages": {
    "Rust": {
      "semantic_tokens": "combined"
    },
    "TypeScript": {
      "semantic_tokens": "full"
    }
  }
}
```

> **Note:** Changing the `semantic_tokens` mode may require a language server restart to take effect. Use the `lsp: restart language servers` command from the command palette if highlighting doesn't update immediately.

## Customizing Token Colors

Semantic tokens are styled using rules that map LSP token types and modifiers to theme styles or custom colors. Zed provides sensible defaults, but you can customize these in your settings.json: add rules under `global_lsp_settings.semantic_token_rules` key.

Rules are matched in order, and the first matching rule wins.
User-defined rules take precedence over defaults.

### Rule Structure

Each rule can specify:

| Property           | Description                                                                                                        |
| ------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `token_type`       | The LSP semantic token type to match (e.g., `"variable"`, `"function"`, `"class"`). If omitted, matches all types. |
| `token_modifiers`  | A list of modifiers that must all be present (e.g., `["declaration"]`, `["readonly", "static"]`).                  |
| `style`            | A list of theme style names to try. The first one found in the current theme is used.                              |
| `foreground_color` | Override foreground color in hex format (e.g., `"#ff0000"`).                                                       |
| `background_color` | Override background color in hex format.                                                                           |
| `underline`        | Boolean or hex color. If `true`, underlines with the text color.                                                   |
| `strikethrough`    | Boolean or hex color. If `true`, strikes through with the text color.                                              |
| `font_weight`      | `"normal"` or `"bold"`.                                                                                            |
| `font_style`       | `"normal"` or `"italic"`.                                                                                          |

### Example: Highlighting Unresolved References

To make unresolved references stand out:

```json [settings]
{
  "global_lsp_settings": {
    "semantic_token_rules": [
      {
        "token_type": "unresolvedReference",
        "foreground_color": "#c93f3f",
        "font_weight": "bold"
      }
    ]
  }
}
```

### Example: Highlighting Unsafe Code

To highlight unsafe operations in Rust:

```json [settings]
{
  "global_lsp_settings": {
    "semantic_token_rules": [
      {
        "token_type": "punctuation",
        "token_modifiers": ["unsafe"],
        "foreground_color": "#AA1111",
        "font_weight": "bold"
      }
    ]
  }
}
```

### Example: Using Theme Styles

Instead of hardcoding colors, reference styles from your theme:

```json [settings]
{
  "global_lsp_settings": {
    "semantic_token_rules": [
      {
        "token_type": "variable",
        "token_modifiers": ["mutable"],
        "style": ["variable.mutable", "variable"]
      }
    ]
  }
}
```

The first style found in the current theme is used, providing fallback options.

### Example: Disabling a Token Type

To disable highlighting for a specific token type, add an empty rule that matches it:

```json [settings]
{
  "global_lsp_settings": {
    "semantic_token_rules": [
      {
        "token_type": "comment"
      }
    ]
  }
}
```

Since user rules are prepended to defaults and the first match wins, this empty rule prevents any styling from being applied to comment tokens.

## Default Rules

Zed's default semantic token rules map standard LSP token types to common theme styles. For example:

- `function` → `function` style
- `variable` with `constant` modifier → `constant` style
- `class` → `type.class`, `class`, or `type` style (first found)
- `comment` with `documentation` modifier → `comment.documentation` or `comment.doc` style

The full default configuration can be shown in Zed with the `zed: show default semantic token rules` command.

## Standard Token Types

Language servers report tokens using standardized types. Common types include:

| Type            | Description                        |
| --------------- | ---------------------------------- |
| `namespace`     | Namespace or module names          |
| `type`          | Type names                         |
| `class`         | Class names                        |
| `enum`          | Enum type names                    |
| `interface`     | Interface names                    |
| `struct`        | Struct names                       |
| `typeParameter` | Generic type parameters            |
| `parameter`     | Function/method parameters         |
| `variable`      | Variable names                     |
| `property`      | Object properties or struct fields |
| `enumMember`    | Enum variants                      |
| `function`      | Function names                     |
| `method`        | Method names                       |
| `macro`         | Macro names                        |
| `keyword`       | Language keywords                  |
| `comment`       | Comments                           |
| `string`        | String literals                    |
| `number`        | Numeric literals                   |
| `operator`      | Operators                          |

Common modifiers include: `declaration`, `definition`, `readonly`, `static`, `deprecated`, `async`, `documentation`, `defaultLibrary`, and language-specific modifiers like `unsafe` (Rust) or `abstract` (TypeScript).

For the complete specification, see the [LSP Semantic Tokens documentation](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#semanticTokenTypes).

## Inspecting Semantic Tokens

To see semantic tokens applied to your code in real-time, use the `dev: open highlights tree view` command from the command palette. This opens a panel showing all highlights (including semantic tokens) for the current buffer, making it easier to understand which tokens are being applied and debug your custom rules.

## Troubleshooting

### Semantic highlighting not appearing

1. Ensure `semantic_tokens` is set to `"combined"` or `"full"` for the language
2. Verify the language server supports semantic tokens (not all do)
3. Try restarting the language server with `lsp: restart language servers`
4. Check the LSP logs (`workspace: open lsp log`) for errors

### Colors not updating after changing settings

Changes to `semantic_tokens` mode may require a language server restart. Use `lsp: restart language servers` from the command palette.

### Theme styles not being applied

Ensure the style names in your rules match styles defined in your theme. The `style` array provides fallback options—if the first style isn't found, Zed tries the next one.
