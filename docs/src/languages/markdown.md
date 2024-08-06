---
something = "else"
---

# Markdown

Markdown support is available natively in Zed.

- Tree Sitter: [tree-sitter-md](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- Language Server: N/A

## Configuration

If you wish change the default language settings for Markdown files, perhaps to disable auto format on save or if your markdown relies upon trailing whitespace `  ` being converted to `<br />` you can add change these values in your `settings.json`:

```json
  "languages": {
    "Markdown": {
      "remove_trailing_whitespace_on_save": true,
      "format_on_save": "on"
    }
  },
```
