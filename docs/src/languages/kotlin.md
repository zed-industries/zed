# Kotlin

Kotlin language support in Zed is provided by the community-maintained [Kotlin extension](https://github.com/zed-extensions/kotlin).
Report issues to: [https://github.com/zed-extensions/kotlin/issues](https://github.com/zed-extensions/kotlin/issues)

- Tree-sitter: [fwcd/tree-sitter-kotlin](https://github.com/fwcd/tree-sitter-kotlin)
- Language Server: [fwcd/kotlin-language-server](https://github.com/fwcd/kotlin-language-server)

## Configuration

Workspace configuration options can be passed to the language server via lsp
settings in `settings.json`.

The full list of lsp `settings` can be found
[here](https://github.com/fwcd/kotlin-language-server/blob/main/server/src/main/kotlin/org/javacs/kt/Configuration.kt)
under `class Configuration` and initialization_options under `class InitializationOptions`.

### JVM Target

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

### JAVA_HOME

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
