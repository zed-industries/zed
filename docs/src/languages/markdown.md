# Markdown

Markdown support is available natively in Zed.

- Tree Sitter: [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- Language Server: N/A

## Syntax Highlighting Code Blocks

Zed supports language-specific syntax highlighting of markdown code blocks by leveraging [tree-sitter language grammars](../extensions/languages.md#grammar). All [Zed supported languages](../languages.md), including those provided by official or community extensions, are available for use in markdown code blocks. All you need to do is provide a language name after the opening <kbd>```</kbd> code fence like so:

````python
```python
import functools as ft

@ft.lru_cache(maxsize=500)
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)
```
````

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
