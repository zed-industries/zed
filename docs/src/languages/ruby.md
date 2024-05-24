# Ruby

- Tree Sitter: [tree-sitter-ruby](https://github.com/tree-sitter/tree-sitter-ruby)
- Language Servers: [solargraph](https://github.com/castwide/solargraph), [ruby-lsp](https://github.com/Shopify/ruby-lsp)

### Switching between language servers

To switch between language servers in Zed, you can configure your `settings.json` file.

Here's the default configuration:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["solargraph", "!ruby-lsp", "..."]
    }
  }
}
```

It sets up Solargraph as the primary language server for Ruby, while the `ruby-lsp` is disabled by default (indicated by the `!` before `ruby-lsp`) due to [some limitations in Zed](https://github.com/zed-industries/zed/pull/8613).

### Setting up `solargraph`

Zed currently doesn't install Solargraph automatically. To use Solargraph, you need to install the gem. Zed just looks for an executable called `solargraph` on your `PATH`.

You can install the gem manually with the following command:

```shell
gem install solargraph
```

Alternatively, if your project uses Bundler, you can add the Solargraph gem to your `Gemfile`:

```ruby
gem 'solargraph', group: :development
```

Solargraph has formatting and diagnostics disabled by default. We can tell Zed to enable them by adding the following to your `settings.json`:

```json
{
  "lsp": {
    "solargraph": {
      "initialization_options": {
        "diagnostics": true,
        "formatting": true
      }
    }
  }
}
```

### Configuration

Solargraph reads its configuration from a file called `.solargraph.yml` in the root of your project. For more information about this file, see the [Solargraph configuration documentation](https://solargraph.org/guides/configuration).

### Setting up `ruby-lsp`

Zed currently doesn't install Ruby LSP automatically. To use Ruby LSP, you need to install the gem. Zed just looks for an executable called `ruby-lsp` on your `PATH`.

You can install the gem manually with the following command:

```shell
gem install ruby-lsp
```

Ruby LSP uses pull-based diagnostics which Zed doesn't support yet. We can tell Zed to disable it by adding the following to your `settings.json`:

```json
{
  "lsp": {
    "ruby-lsp": {
      "initialization_options": {
        "enabledFeatures": {
          "diagnostics": false
        }
      }
    }
  }
}
```
