# Elixir

- Tree Sitter: [tree-sitter-elixir](https://github.com/elixir-lang/tree-sitter-elixir)
- Language Server: [elixir-ls](https://github.com/elixir-lsp/elixir-ls)

## Choosing a language server

The Elixir extension offers language server support for `elixir-ls`, `next-ls`, and `lexical`.

`elixir-ls` is enabled by default.

To switch to `next-ls`, add the following to your `settings.json`:

```json
{
  "languages": {
    "Elixir": {
      "language_servers": ["next-ls", "!elixir-ls", "..."]
    }
  }
}
```

To switch to `lexical`, add the following to your `settings.json`:

```json
{
  "languages": {
    "Elixir": {
      "language_servers": ["lexical", "!elixir-ls", "..."]
    }
  }
}
```

## Setting up `elixir-ls`

1. Install `elixir`:

```bash
brew install elixir
```

2. Install `elixir-ls`:

```bash
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

### Additional workspace configuration options (requires Zed `0.128.0`):

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
