# Ruby

Ruby support is available through the [Ruby extension](https://github.com/zed-industries/zed/tree/main/extensions/ruby).

The Ruby extension also provides support for ERB files.

## Choosing a language server

The Ruby extension offers both `solargraph` and `ruby-lsp` language server support.

`solargraph` is enabled by default.

To switch to `ruby-lsp`, add the following to your `settings.json`:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "!solargraph", "!rubocop", "..."]
    }
  }
}
```

The Ruby extension also provides support for `rubocop` language server for offense detection and autocorrection. To enable it, add the following to your
`settings.json`:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "rubocop", "!solargraph", "..."]
    }
  }
}
```

## Setting up `solargraph`

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

To use Solargraph in the context of the bundle, you can use [folder-specific settings](../configuring-zed#settings-files) and specify the absolute path to the [`binstub`](https://bundler.io/v2.5/man/bundle-binstubs.1.html) of Solargraph:

```json
{
  "lsp": {
    "solargraph": {
      "binary": {
        "path": "<path_to_your_project>/bin/solargraph"
      }
    }
  }
}
```

### Configuration

Solargraph reads its configuration from a file called `.solargraph.yml` in the root of your project. For more information about this file, see the [Solargraph configuration documentation](https://solargraph.org/guides/configuration).

## Setting up `ruby-lsp`

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

## Setting up `rubocop` LSP

Zed currently doesn't install `rubocop` automatically. To use `rubocop`, you need to install the gem. Zed just looks for an executable called `rubocop` on your `PATH`.

You can install the gem manually with the following command:

```shell
gem install rubocop
```

Rubocop has unsafe autocorrection disabled by default. We can tell Zed to enable it by adding the following to your `settings.json`:

```json
{
  "lsp": {
    "rubocop": {
      "initialization_options": {
        "safeAutocorrect": false
      }
    }
  }
}
```

To use Rubocop in the context of the bundle, you can use [folder-specific settings](../configuring-zed#settings-files) and specify the absolute path to the [`binstub`](https://bundler.io/v2.5/man/bundle-binstubs.1.html) of Rubocop:

```json
{
  "lsp": {
    "rubocop": {
      "binary": {
        "path": "<path_to_your_project>/bin/rubocop"
      }
    }
  }
}
```

## Using the Tailwind CSS Language Server with Ruby

It's possible to use the [Tailwind CSS Language Server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in Ruby and ERB files.

In order to do that, you need to configure the language server so that it knows about where to look for CSS classes in Ruby/ERB files by adding the following to your `settings.json`:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["tailwindcss-language-server", "..."]
    }
  },
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "includeLanguages": {
          "erb": "html",
          "ruby": "html"
        },
        "experimental": {
          "classRegex": ["\\bclass:\\s*['\"]([^'\"]*)['\"]"]
        }
      }
    }
  }
}
```

With these settings you will get completions for Tailwind CSS classes in HTML attributes inside ERB files and inside Ruby/ERB strings that are coming after a `class:` key. Examples:

```ruby
# Ruby file:
def method
  div(class: "pl-2 <completion here>") do
    p(class: "mt-2 <completion here>") { "Hello World" }
  end
end

# ERB file:
<%= link_to "Hello", "/hello", class: "pl-2 <completion here>" %>
<a href="/hello" class="pl-2 <completion here>">Hello</a>
```
