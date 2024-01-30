# Elixir

- Tree Sitter: [tree-sitter-elixir](https://github.com/elixir-lang/tree-sitter-elixir)
- Language Server: [elixir-ls](https://github.com/elixir-lsp/elixir-ls)

### Setting up `elixir-ls`

1. Install `elixir`:

```bash
brew install elixir
```

2. Install `elixir-ls`:

```bash
brew install elixir-ls
```

3. Restart Zed

{% hint style="warning" %}
If `elixir-ls` is not running in an elixir project, check the error log via the command palette action `zed: open log`.  If you find an error message mentioning: `invalid LSP message header "Shall I install Hex? (if running non-interactively, use \"mix local.hex --force\") [Yn]`, you might need to install [`Hex`](https://hex.pm). You run `elixir-ls` from the command line and accept the prompt to install `Hex`.
{% endhint %}

### Formatting with Mix

If you prefer to format your code with [Mix](https://hexdocs.pm/mix/Mix.html), use the following snippet in your `settings.json` file to configure it as an external formatter.  Formatting will occur on file save.

```json
{
  "language_overrides": {
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
