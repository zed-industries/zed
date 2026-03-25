---
title: Elixir
description: "Configure Elixir language support in Zed, including language servers, formatting, and debugging."
---

# Elixir

Elixir support is available through the [Elixir extension](https://github.com/zed-extensions/elixir).

- Tree-sitter grammars:
  - [elixir-lang/tree-sitter-elixir](https://github.com/elixir-lang/tree-sitter-elixir)
  - [phoenixframework/tree-sitter-heex](https://github.com/phoenixframework/tree-sitter-heex)
- Language Servers:
  - [elixir-lang/expert](https://github.com/elixir-lang/expert)
  - [elixir-lsp/elixir-ls](https://github.com/elixir-lsp/elixir-ls)
  - [elixir-tools/next-ls](https://github.com/elixir-tools/next-ls)
  - [lexical-lsp/lexical](https://github.com/lexical-lsp/lexical)

The Elixir extension also supports [EEx](https://hexdocs.pm/eex/EEx.html) (Embedded Elixir) templates and [HEEx](https://hexdocs.pm/phoenix/components.html#heex) templates, a mix of HTML and EEx used by Phoenix LiveView applications.

## Language Servers

The Elixir extension offers language server support for `elixir-ls`, `expert`, `next-ls`, and `lexical`.

### ElixirLS

`elixir-ls` is enabled by default.

You can pass additional workspace configuration options to it via `lsp` > `settings` in your settings file ([how to edit](../configuring-zed.md#settings-files)).

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

See [ElixirLS configuration settings](https://github.com/elixir-lsp/elixir-ls#elixirls-configuration-settings) for more options.

### Expert

Configure language servers in Settings ({#kb zed::OpenSettings}) under Languages > Elixir, Languages > EEx, and Languages > HEEx, or add to your settings file ([how to edit](../configuring-zed.md#settings-files)):

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

You can pass additional workspace configuration options to Expert via `lsp` > `settings` in your settings file.

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

See [Expert configuration](https://expert-lsp.org/docs/configuration/) for more options.

### Next LS

Configure language servers in Settings ({#kb zed::OpenSettings}) under Languages > Elixir, Languages > EEx, and Languages > HEEx, or add to your settings file ([how to edit](../configuring-zed.md#settings-files)):

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

You can pass additional initialization options to Next LS via `lsp` > `initialization_options` in your settings file.

Next LS has completions enabled by default on Zed. This is an experimental feature, so it can be disabled by adding the following to your settings file:

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

Next LS also has [Credo](https://hexdocs.pm/credo/overview.html) detection support enabled by default. This can be disabled by adding the following to your settings file:

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

### Lexical

Configure language servers in Settings ({#kb zed::OpenSettings}) under Languages > Elixir, Languages > EEx, and Languages > HEEx, or add to your settings file ([how to edit](../configuring-zed.md#settings-files)):

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

If you prefer to work without a language server but would still like code formatting from [Mix](https://hexdocs.pm/mix/Mix.html), you can configure it as an external formatter.

Configure formatting in Settings ({#kb zed::OpenSettings}) under Languages > Elixir, Languages > EEx, and Languages > HEEx, or add to your settings file ([how to edit](../configuring-zed.md#settings-files)):

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

To get all features (autocomplete, linting, and hover docs) from the [Tailwind CSS language server](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme) in HEEx templates, add the following to your settings file ([how to edit](../configuring-zed.md#settings-files)):

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
