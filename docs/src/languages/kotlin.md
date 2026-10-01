---
title: Kotlin
description: "Configure Kotlin language support in Zed, including language servers, formatting, and debugging."
---

# Kotlin

Kotlin language support in Zed is provided by the community-maintained [Kotlin extension](https://github.com/zed-extensions/kotlin).
Report issues to: [https://github.com/zed-extensions/kotlin/issues](https://github.com/zed-extensions/kotlin/issues)

- Tree-sitter: [fwcd/tree-sitter-kotlin](https://github.com/fwcd/tree-sitter-kotlin)
- Language Server: [kotlin/kotlin-lsp](https://github.com/kotlin/kotlin-lsp)
- Alternate Language Server: [fwcd/kotlin-language-server](https://github.com/fwcd/kotlin-language-server)

## Kotlin LSP

[Kotlin LSP](https://github.com/kotlin/kotlin-lsp) is the official language server for Kotlin, built by JetBrains. It is used by default.

It is downloaded and updated automatically. If you want to use a manually installed version instead, set the path to the `kotlin-lsp.sh` script from the release assets in your `settings.json`:

```json [settings]
{
  "lsp": {
    "kotlin-lsp": {
      "binary": {
        "path": "path/to/kotlin-lsp.sh",
        "arguments": ["--stdio"]
      }
    }
  }
}
```

Note that the `kotlin-lsp.sh` script expects to be run from within the unzipped release zip file, and should not be moved elsewhere.

## Kotlin Language Server

The community-maintained [Kotlin Language Server](https://github.com/fwcd/kotlin-language-server) can be used instead of Kotlin LSP by explicitly enabling it in your `settings.json`:

```json [settings]
{
  "languages": {
    "Kotlin": {
      "language_servers": ["kotlin-language-server", "!kotlin-lsp", "..."]
    }
  }
}
```

### Configuration

Workspace configuration options can be passed to the language server via lsp
settings in `settings.json`.

The full list of lsp `settings` can be found
[here](https://github.com/fwcd/kotlin-language-server/blob/main/server/src/main/kotlin/org/javacs/kt/Configuration.kt)
under `class Configuration` and initialization_options under `class InitializationOptions`.

#### JVM Target

The following example changes the JVM target from `default` (which is 1.8) to
`17`:

```json [settings]
{
  "lsp": {
    "kotlin-language-server": {
      "settings": {
        "compiler": {
          "jvm": {
            "target": "17"
          }
        }
      }
    }
  }
}
```

#### JAVA_HOME

To use a specific java installation, just specify the `JAVA_HOME` environment variable with:

```json [settings]
{
  "lsp": {
    "kotlin-language-server": {
      "binary": {
        "env": {
          "JAVA_HOME": "/Users/whatever/Applications/Work/Android Studio.app/Contents/jbr/Contents/Home"
        }
      }
    }
  }
}
```
