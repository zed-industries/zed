# Ruby

Ruby support is available through the [Ruby extension](https://github.com/zed-extensions/ruby).

- Tree-sitters:
  - [tree-sitter-ruby](https://github.com/tree-sitter/tree-sitter-ruby)
  - [tree-sitter-embedded-template](https://github.com/tree-sitter/tree-sitter-embedded-template)
- Language Servers:
  - [ruby-lsp](https://github.com/Shopify/ruby-lsp)
  - [solargraph](https://github.com/castwide/solargraph)
  - [rubocop](https://github.com/rubocop/rubocop)
- Debug Adapter: [`rdbg`](https://github.com/ruby/debug)

The Ruby extension also provides support for ERB files.

## Language Servers

There are multiple language servers available for Ruby. Zed supports the two following:

- [solargraph](https://github.com/castwide/solargraph)
- [ruby-lsp](https://github.com/Shopify/ruby-lsp)

They both have an overlapping feature set of autocomplete, diagnostics, code actions, etc. and it's up to you to decide which one you want to use. Note that you can't use both at the same time.

In addition to these two language servers, Zed also supports:

- [rubocop](https://github.com/rubocop/rubocop) which is a static code analyzer and linter for Ruby. Under the hood, it's also used by Zed as a language server, but its functionality is complimentary to that of solargraph and ruby-lsp.
- [sorbet](https://sorbet.org/) which is a static type checker for Ruby with a custom gradual type system.
- [steep](https://github.com/soutaro/steep) which is a static type checker for Ruby that leverages Ruby Signature (RBS).

When configuring a language server, it helps to open the LSP Logs window using the 'dev: Open Language Server Logs' command. You can then choose the corresponding language instance to see any logged information.

## Configuring a language server

The [Ruby extension](https://github.com/zed-extensions/ruby) offers both `solargraph` and `ruby-lsp` language server support.

### Language Server Activation

For all supported Ruby language servers (`solargraph`, `ruby-lsp`, `rubocop`, `sorbet`, and `steep`), the Ruby extension follows this activation sequence:

1. If the language server is found in your project's `Gemfile`, it will be used through `bundle exec`.
2. If not found in the `Gemfile`, the Ruby extension will look for the executable in your system `PATH`.
3. If the language server is not found in either location, the Ruby extension will automatically install it as a global gem (note: this will not install to your current Ruby gemset).

You can skip step 1 and force using the system executable by setting `use_bundler` to `false` in your settings:

```json
{
  "lsp": {
    "<SERVER_NAME>": {
      "settings": {
        "use_bundler": false
      }
    }
  }
}
```

### Using `solargraph`

`solargraph` is enabled by default in the Ruby extension.

### Using `ruby-lsp`

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

That disables `solargraph` and `rubocop` and enables `ruby-lsp`.

### Using `rubocop`

The Ruby extension also provides support for `rubocop` language server for offense detection and autocorrection.

To enable it, add the following to your `settings.json`:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "rubocop", "!solargraph", "..."]
    }
  }
}
```

Or, conversely, you can disable `ruby-lsp` and enable `solargraph` and `rubocop` by adding the following to your `settings.json`:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["solargraph", "rubocop", "!ruby-lsp", "..."]
    }
  }
}
```

## Setting up `solargraph`

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

## Setting up `ruby-lsp`

You can pass Ruby LSP configuration to `initialization_options`, e.g.

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "!solargraph", "..."]
    }
  },
  "lsp": {
    "ruby-lsp": {
      "initialization_options": {
        "enabledFeatures": {
          // "someFeature": false
        }
      }
    }
  }
}
```

LSP `settings` and `initialization_options` can also be project-specific. For example to use [standardrb/standard](https://github.com/standardrb/standard) as a formatter and linter for a particular project, add this to a `.zed/settings.json` inside your project repo:

```json
{
  "lsp": {
    "ruby-lsp": {
      "initialization_options": {
        "formatter": "standard",
        "linters": ["standard"]
      }
    }
  }
}
```

## Setting up `rubocop` LSP

Rubocop has unsafe autocorrection disabled by default. We can tell Zed to enable it by adding the following to your `settings.json`:

```json
{
  "languages": {
    "Ruby": {
      // Use ruby-lsp as the primary language server and rubocop as the secondary.
      "language_servers": ["ruby-lsp", "rubocop", "!solargraph", "..."]
    }
  },
  "lsp": {
    "rubocop": {
      "initialization_options": {
        "safeAutocorrect": false
      }
    },
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

## Setting up Sorbet

[Sorbet](https://sorbet.org/) is a popular static type checker for Ruby that includes a language server.

To enable Sorbet, add `\"sorbet\"` to the `language_servers` list for Ruby in your `settings.json`. You may want to disable other language servers if Sorbet is intended to be your primary LSP, or if you plan to use it alongside another LSP for specific features like type checking.

```json
{
  "languages": {
    "Ruby": {
      "language_servers": [
        "ruby-lsp",
        "sorbet",
        "!rubocop",
        "!solargraph",
        "..."
      ]
    }
  }
}
```

For all aspects of installing Sorbet, setting it up in your project, and configuring its behavior, please refer to the [official Sorbet documentation](https://sorbet.org/docs/overview).

## Setting up Steep

[Steep](https://github.com/soutaro/steep) is a static type checker for Ruby that uses RBS files to define types.

To enable Steep, add `\"steep\"` to the `language_servers` list for Ruby in your `settings.json`. You may need to adjust the order or disable other LSPs depending on your desired setup.

```json
{
  "languages": {
    "Ruby": {
      "language_servers": [
        "ruby-lsp",
        "steep",
        "!solargraph",
        "!rubocop",
        "..."
      ]
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
          "html/erb": "html",
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

```rb
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

## Running tests

To run tests in your Ruby project, you can set up custom tasks in your local `.zed/tasks.json` configuration file. These tasks can be defined to work with different test frameworks like Minitest, RSpec, quickdraw, and tldr. Below are some examples of how to set up these tasks to run your tests from within your editor.

### Minitest with Rails

```json
[
  {
    "label": "test $ZED_RELATIVE_FILE -n /$ZED_CUSTOM_RUBY_TEST_NAME/",
    "command": "bin/rails",
    "args": [
      "test",
      "$ZED_RELATIVE_FILE",
      "-n",
      "\"$ZED_CUSTOM_RUBY_TEST_NAME\""
    ],
    "cwd": "$ZED_WORKTREE_ROOT",
    "tags": ["ruby-test"]
  }
]
```

### Minitest

Plain minitest does not support running tests by line number, only by name, so we need to use `$ZED_CUSTOM_RUBY_TEST_NAME` instead:

```json
[
  {
    "label": "-Itest $ZED_RELATIVE_FILE -n /$ZED_CUSTOM_RUBY_TEST_NAME/",
    "command": "bundle",
    "args": [
      "exec",
      "ruby",
      "-Itest",
      "$ZED_RELATIVE_FILE",
      "-n",
      "\"$ZED_CUSTOM_RUBY_TEST_NAME\""
    ],
    "cwd": "$ZED_WORKTREE_ROOT",
    "tags": ["ruby-test"]
  }
]
```

### RSpec

```json
[
  {
    "label": "test $ZED_RELATIVE_FILE:$ZED_ROW",
    "command": "bundle",
    "args": ["exec", "rspec", "\"$ZED_RELATIVE_FILE:$ZED_ROW\""],
    "cwd": "$ZED_WORKTREE_ROOT",
    "tags": ["ruby-test"]
  }
]
```

Similar task syntax can be used for other test frameworks such as `quickdraw` or `tldr`.

## Debugging

The Ruby extension provides a debug adapter for debugging Ruby code. Zed's name for the adapter (in the UI and `debug.json`) is `rdbg`, and under the hood, it uses the [`debug`](https://github.com/ruby/debug) gem. The extension uses the [same activation logic](#language-server-activation) as the language servers.

### Examples

#### Debug a Ruby script

```json
[
  {
    "label": "Debug current file",
    "adapter": "rdbg",
    "request": "launch",
    "script": "$ZED_FILE",
    "cwd": "$ZED_WORKTREE_ROOT"
  }
]
```

#### Debug Rails server

```json
[
  {
    "label": "Debug Rails server",
    "adapter": "rdbg",
    "request": "launch",
    "command": "./bin/rails",
    "args": ["server"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "env": {
      "RUBY_DEBUG_OPEN": "true"
    }
  }
]
```

## Formatters

### `erb-formatter`

To format ERB templates, you can use the `erb-formatter` formatter. This formatter uses the [`erb-formatter`](https://rubygems.org/gems/erb-formatter) gem to format ERB templates.

```jsonc
{
  "HTML/ERB": {
    "formatter": {
      "external": {
        "command": "erb-formatter",
        "arguments": ["--stdin-filename", "{buffer_path}"],
      },
    },
  },
}
```
