# Language settings

- Common settings for language configurations
  - What are these?
- What's done via tree-sitter vs what's done via a language server
- Configuring language-server specific settings


Goal of this page?
Explaining where the


> Move this to a different section maybe?

## What it means to be a "language" in Zed

Language support in Zed is based on the [Tree-sitter](./tree-sitter.md) parsing framework and the [language server protocol](#language-servers).

Tree-sitter is a flexible framework that produces syntax-trees based on grammars. There are grammars available for many languages, and a growing list of Zed extensions powered by those grammars. If there's not an extension, it's [easy to create one] based on an existing grammar, or you can [develop your own grammar]. A growing list of Zed features are built using pattern matching over syntax trees with Tree-sitter queries. These queries are unique to a specific language grammar, and are bundled with language extensions.

To develop a language settings [Developing Extensions](development/languages.md)

> How do I develop a language extension?

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

The Language Server Protocol (LSP) is a standardized protocol that allows communication between language-specific servers and clients like Zed.
Language servers give Zed a way to support advanced features for multiple programming languages without having to implement each feature for each language.

Some of the things language servers provide:

- Code completion. Available completions can be accessed and applied using commands like `editor: show completions` and `editor: confirm completion`.
- Error checking and diagnostics. These appear inline in buffers as squiggly underlines beneath text. The full list of errors and warnings for the project is available in the Project Diagnostics panel.
- Code navigation. These can be accessed using commands like `editor: go to definition`, `editor: go to type definition`, and `editor: find all references`.
- Refactoring. The `editor: rename` command tells the Language Server to rename a symbol both at its definition site as well as everywhere it's used.
- [Inlay hints](./docs/configuring-zed#inlay-hints). These are hints that passively appear inline in the text of a buffer, for things like inferred types (if the language server provides that).
- Hover
- Workplace Symbols
- Code Folding
- Code Actions (optimize imports, etc)

Anyone can create a new language server and use it in Zed, without needing to modify Zed's code base.
This makes it possible to add support for new languages to Zed, as well as to customize how existing
languages integrate with Zed, without the need for Zed itself to change.

### Downloading Language Servers

Zed simplifies language server management for users. When you open a file with a matching extension, the
appropriate language server is automatically downloaded. On macOS, these servers are stored in
`~/Library/Application Support/Zed/languages`,  while on Linux, they're placed in either `$XDG_DATA_HOME/languages`,
`$FLATPAK_XDG_DATA_HOME/languages`, or  `$HOME/.local/share`, depending on which environment variables are set.
Whether you're using bundled languages or those from extensions, Zed keeps your language servers up-to-date
automatically, ensuring you always have the latest features and improvements.

Zed supports multiple language servers per language. Default servers can be overridden using the `language_servers` setting, allowing you to enable or disable specific servers. For example, to enable intelephense and disable phpactor for PHP:

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "..."]
    }
  }
}
```

Some highlights about the above example:

- The `!` symbol before `phpactor` indicates that this server should be disabled
- `intelephense` is explicitly enabled
- `...` ensures that any other default servers for PHP remain active.

Notable languages that expose multiple language servers are: [Python](./languages/python.md), [Ruby](./languages/ruby.md), and [PHP](./languages/php.md).

### Configuring a Language Server

Language servers expose initialization options that can be configured in Zed. These options allow you to customize the behavior of the language server for specific languages.

To configure a language server, use the following JSON structure in your Zed configuration:

```json
"lsp": {
  "server-name": {
    "initialization_options": {
      "option1": value1,
      "option2": value2
    }
  }
}
```

For example, to disable `cargo check` on save for Rust you can set the following in your `settings.json`:

```json
"lsp": {
  "rust-analyzer": {
    "initialization_options": {
      "checkOnSave": false
    }
  }
}
```

Or to enable short open tags for PHP:

```json
"lsp": {
  "intelephense": {
    "initialization_options": {
      "environment": {
        "shortOpenTag": true
      }
    }
  }
}
```

Each language server typically provides documentation on its supported initialization options. For instance, rust-analyzer's configuration options can be found at https://rust-analyzer.github.io/manual.html#configuration.
