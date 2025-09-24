# Elixir

Elixir support is available through the [Elixir extension](https://github.com/zed-extensions/elixir).

- Tree-sitter:
  - [elixir-lang/tree-sitter-elixir](https://github.com/elixir-lang/tree-sitter-elixir)
  - [phoenixframework/tree-sitter-heex](https://github.com/phoenixframework/tree-sitter-heex)
- Language servers:
  - [elixir-lang/expert](https://github.com/elixir-lang/expert)
  - [elixir-lsp/elixir-ls](https://github.com/elixir-lsp/elixir-ls)
  - [elixir-tools/next-ls](https://github.com/elixir-tools/next-ls)
  - [lexical-lsp/lexical](https://github.com/lexical-lsp/lexical)

## Choosing a language server

The Elixir extension offers language server support for `expert`, `elixir-ls`, `next-ls`, and `lexical`.

`elixir-ls` is enabled by default.

### Expert

To switch to `expert`, add the following to your `settings.json`:

```json
  "languages": {
    "Elixir": {
      "language_servers": ["expert", "!elixir-ls", "!next-ls", "!lexical", "..."]
    },
    "HEEX": {
      "language_servers": ["expert", "!elixir-ls", "!next-ls", "!lexical", "..."]
    }
  }
```

### Next LS

To switch to `next-ls`, add the following to your `settings.json`:

```json
  "languages": {
    "Elixir": {
      "language_servers": ["next-ls", "!expert", "!elixir-ls", "!lexical", "..."]
    },
    "HEEX": {
      "language_servers": ["next-ls", "!expert", "!elixir-ls", "!lexical", "..."]
    }
  }
```

### Lexical

To switch to `lexical`, add the following to your `settings.json`:

```json
  "languages": {
    "Elixir": {
      "language_servers": ["lexical", "!expert", "!elixir-ls", "!next-ls", "..."]
    },
    "HEEX": {
      "language_servers": ["lexical", "!expert", "!elixir-ls", "!next-ls", "..."]
    }
  }
```

## Setting up `elixir-ls`

1. Install `elixir`:

```sh
brew install elixir
```

2. Install `elixir-ls`:

```sh
brew install elixir-ls
```

3. Restart Zed

> If `elixir-ls` is not running in an elixir project, check the error log via the command palette action `zed: open log`. If you find an error message mentioning: `invalid LSP message header "Shall I install Hex? (if running non-interactively, use \"mix local.hex --force\") [Yn]`, you might need to install [`Hex`](https://hex.pm). You run `elixir-ls` from the command line and accept the prompt to install `Hex`.

### Formatting with Mix

If you prefer to format your code with [Mix](https://hexdocs.pm/mix/Mix.html), use the following snippet in your `settings.json` file to configure it as an external formatter. Formatting will occur on file save.

```json
{
  "languages": {
    "Elixir": {
      "format_on_save": {
        "external": {
          "command": "mix",
          "arguments": ["format", "--stdin-filename", "{buffer_path}", "-"]
        }
      }
    }
  }
}
```

### Additional workspace configuration options

You can pass additional elixir-ls workspace configuration options via lsp settings in `settings.json`.

The following example disables dialyzer:

```json
  "lsp": {
    "elixir-ls": {
      "settings": {
        "dialyzerEnabled": false
      }
    }
  }
```

See [ElixirLS configuration settings](https://github.com/elixir-lsp/elixir-ls#elixirls-configuration-settings) for more options.

### HEEx

Zed also supports HEEx templates. HEEx is a mix of [EEx](https://hexdocs.pm/eex/1.12.3/EEx.html) (Embedded Elixir) and HTML, and is used in Phoenix LiveView applications.

- Tree-sitter: [phoenixframework/tree-sitter-heex](https://github.com/phoenixframework/tree-sitter-heex)
