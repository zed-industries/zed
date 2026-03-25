---
title: Elixir
description: "Configure Elixir language support in Zed, including language servers, formatting, and debugging."
---

# Elixir

Elixir support is available through the [Elixir extension](https://github.com/zed-extensions/elixir).

- Tree-sitter Grammars:
  - [elixir-lang/tree-sitter-elixir](https://github.com/elixir-lang/tree-sitter-elixir)
  - [phoenixframework/tree-sitter-heex](https://github.com/phoenixframework/tree-sitter-heex)
- Language Servers:
  - [elixir-lang/expert](https://github.com/elixir-lang/expert)
  - [elixir-lsp/elixir-ls](https://github.com/elixir-lsp/elixir-ls)
  - [elixir-tools/next-ls](https://github.com/elixir-tools/next-ls)
  - [lexical-lsp/lexical](https://github.com/lexical-lsp/lexical)

The Elixir extension also supports [EEx](https://hexdocs.pm/eex/EEx.html) (Embedded Elixir) templates and [HEEx](https://hexdocs.pm/phoenix/components.html#heex) templates, a mix of HTML and EEx used by Phoenix LiveView applications.

## Language Servers

The Elixir extension offers language server support for ElixirLS, Expert, Next LS, and Lexical. By default, ElixirLS is enabled, but this can be changed or disabled in Settings ({#kb zed::OpenSettings}) under Languages > Elixir/EEx/HEEx, or directly in your settings file.

Some of the language servers can also accept initialization or workspace configuration options; what each one accepts is outlined in the sections below. These can be passed in your settings file via `lsp` > `initialization_options` and `lsp` > `settings` respectively.

See the [Configuring Zed](../configuring-zed.md#settings-files) guide for more information on how to edit your settings file.

### Using ElixirLS

ElixirLS can accept workspace configuration options.

The following example disables [Dialyzer](https://github.com/elixir-lsp/elixir-ls#dialyzer-integration):

```json [settings]
  "lsp": {
    "elixir-ls": {
      "settings": {
        "dialyzerEnabled": false
      }
    }
  }
```

See the official list of [ElixirLS configuration settings](https://github.com/elixir-lsp/elixir-ls#elixirls-configuration-settings) for all available options.

### Using Expert

Enable Expert by adding the following to your settings file:

```json [settings]
  "languages": {
    "Elixir": {
      "language_servers": ["expert", "!elixir-ls", "!next-ls", "!lexical", "..."]
    },
    "EEx": {
      "language_servers": ["expert", "!elixir-ls", "!next-ls", "!lexical", "..."]
    },
    "HEEx": {
      "language_servers": ["expert", "!elixir-ls", "!next-ls", "!lexical", "..."]
    }
  }
```

Expert can accept workspace configuration options.

The following example sets the minimum number of characters required for a project symbols search to return results:

```json [settings]
  "lsp": {
    "expert": {
      "settings": {
        "workspaceSymbols": {
          "minQueryLength": 0
        }
      }
    }
  }
```

See the [Expert configuration](https://expert-lsp.org/docs/configuration/) page for all available options.

### Using Next LS

Enable Next LS by adding the following to your settings file:

```json [settings]
  "languages": {
    "Elixir": {
      "language_servers": ["next-ls", "!expert", "!elixir-ls", "!lexical", "..."]
    },
    "EEx": {
      "language_servers": ["next-ls", "!expert", "!elixir-ls", "!lexical", "..."]
    },
    "HEEx": {
      "language_servers": ["next-ls", "!expert", "!elixir-ls", "!lexical", "..."]
    }
  }
```

Next LS can accept initialization options.

Completions are an experimental feature in Next LS, but they are enabled by default on Zed. They can be disabled again by adding the following to your settings file:

```json [settings]
  "lsp": {
    "next-ls": {
      "initialization_options": {
        "experimental": {
          "completions": {
            "enable": false
          }
        }
      }
    }
  }
```

Next LS also has an extension for [Credo](https://hexdocs.pm/credo/overview.html) detection; this is enabled by default, but can be disabled by adding the following to your settings file:

```json [settings]
  "lsp": {
    "next-ls": {
      "initialization_options": {
        "extensions": {
          "credo": {
            "enable": false
          }
        }
      }
    }
  }
```

It is also possible to pass CLI options to Credo. The following example passes `--min-priority high` to it:

```json [settings]
  "lsp": {
    "next-ls": {
      "initialization_options": {
        "extensions": {
          "credo": {
            "cli_options": ["--min-priority high"]
          }
        }
      }
    }
  }
```

See the [Credo Command Line Switches](https://hexdocs.pm/credo/suggest_command.html#command-line-switches) page for more CLI options.

### Using Lexical

Enable Lexical by adding the following to your settings file:

```json [settings]
  "languages": {
    "Elixir": {
      "language_servers": ["lexical", "!expert", "!elixir-ls", "!next-ls", "..."]
    },
    "EEx": {
      "language_servers": ["lexical", "!expert", "!elixir-ls", "!next-ls", "..."]
    },
    "HEEx": {
      "language_servers": ["lexical", "!expert", "!elixir-ls", "!next-ls", "..."]
    }
  }
```

## Formatting without a language server

If you prefer to work without a language server but would still like code formatting from [Mix](https://hexdocs.pm/mix/Mix.html), you can configure it as an external formatter by adding the following to your settings file:

```json [settings]
  "languages": {
    "Elixir": {
      "enable_language_server": false,
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "mix",
          "arguments": ["format", "--stdin-filename", "{buffer_path}", "-"]
        }
      }
    },
    "EEx": {
      "enable_language_server": false,
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "mix",
          "arguments": ["format", "--stdin-filename", "{buffer_path}", "-"]
        }
      }
    },
    "HEEx": {
      "enable_language_server": false,
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "mix",
          "arguments": ["format", "--stdin-filename", "{buffer_path}", "-"]
        }
      }
    }
  }
```

## Using the Tailwind CSS Language Server with HEEx templates

To get all features (autocomplete, linting, and hover docs) from the [Tailwind CSS language server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in HEEx templates, add the following to your settings file:

```json [settings]
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "includeLanguages": {
          "elixir": "html",
          "heex": "html"
        },
        "experimental": {
          "classRegex": ["class=\"([^\"]*)\"", "class='([^']*)'"]
        }
      }
    }
  }
```

With these settings, you will get completions for Tailwind CSS classes in HEEx templates. Examples:

```heex
<%!-- Standard class attribute --%>
<div class="flex items-center <completion here>">
  <p class="text-lg font-bold <completion here>">Hello World</p>
</div>

<%!-- With Elixir expression --%>
<div class={"flex #{@custom_class} <completion here>"}>
  Content
</div>

<%!-- With Phoenix function --%>
<div class={class_list(["flex", "items-center", "<completion here>"])}>
  Content
</div>
```

## See also

- [Erlang](./erlang.md)
- [Gleam](./gleam.md)
