# Kotlin

Kotlin language support in Zed is provided by the community-maintained [Kotlin extension](https://github.com/zed-extensions/kotlin).
Report issues to: [https://github.com/zed-extensions/kotlin/issues](https://github.com/zed-extensions/kotlin/issues)

- Tree Sitter: [fwcd/tree-sitter-kotlin](https://github.com/fwcd/tree-sitter-kotlin)
- Language Server: [fwcd/kotlin-language-server](https://github.com/fwcd/kotlin-language-server)

## Configuration

Workspace configuration options can be passed to the language server via lsp
settings in `settings.json`.

The following example changes the JVM target from `default` (which is 1.8) to
`17`:

```json
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

The full list of workspace configuration options can be found
[here](https://github.com/fwcd/kotlin-language-server/blob/main/server/src/main/kotlin/org/javacs/kt/Configuration.kt).
