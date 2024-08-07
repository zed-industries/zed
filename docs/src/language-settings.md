# Languages in Zed

## Syntax highlighting

Syntax highlighting in Zed is handled by [tree-sitter](./tree-sitter.md).
TBD: Explain how tree-sitter is used in Zed

## Per-Language Settings

TBD: Document how to language associations using [`file_types`](./configuring-zed.md#file-types)
TBD: Certain top-level settings that can be overridden per-language:

```json
{
  "languages": {
    "Makefile": {
      "tab_size": 2,
      "format_on_save": false,
      "remove_trailing_whitespace_on_save": false
    }
  }
}
```

- [`enable_language_server`](./configuring-zed.md#enable-language-server)
- [`ensure_final_newline_on_save`](./configuring-zed.md#ensure-final-newline-on-save)
- [`format_on_save`](./configuring-zed.md#format-on-save)
- [`formatter`](./configuring-zed.md#formatter)
- [`hard_tabs`](./configuring-zed.md#hard-tabs)
- [`preferred_line_length`](./configuring-zed.md#preferred-line-length)
- [`remove_trailing_whitespace_on_save`](./configuring-zed.md#remove-trailing-whitespace-on-save)
- [`show_inline_completions`](./configuring-zed.md#show-inline-completions)
- [`show_whitespaces`](./configuring-zed.md#show-whitespaces)
- [`soft_wrap`](./configuring-zed.md#soft-wrap)
- [`tab_size`](./configuring-zed.md#tab-size)
- [`use_autoclose`](./configuring-zed.md#auto-close)
- [`always_treat_brackets_as_autoclosed`](./configuring-zed.md#always-treat-brackets-as-autoclosed)
- [`indent_guides`](./configuring-zed.md#indent-guides)

TBD: Add additional settings which may be overridden.

## Language Servers

TBD: Explain how Language servers are used in zed

- differentiate between [tree-sitter](./tree-sitter.md)
- explain how we download them
- how they can be found locally (path, etc)
  - https://zed.dev/docs/configuring-zed#direnv-integration

TBD: Explain how to choose between multiple language servers
TBD: Cross link explanation to Python, TypeScript, Ruby, PHP, etc.

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "..."]
    }
  }
}
```

## inlayHints

TBD: Explain what inlay hints are.
Link: https://zed.dev/docs/configuring-zed#inlay-hints

## Other Actions:

TBD: Document the type of actions supported for language servers

- Code Completion
- Hover
- Jump to Def
- Workplace Symbols
- Find References
- Diagnostics

TBD: LSP actions: Do we support all of these? Everywhere? Specific languages?

- Rename
- Code Folding
- Find All Implementations
- Go to type Definition
- Go to Declaration
- Code Actions (optomize imports, etc)
