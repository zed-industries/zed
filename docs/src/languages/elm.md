# Elm

Elm support is available through the [Elm extension](https://github.com/zed-industries/zed/tree/main/extensions/elm).

- Tree Sitter: [elm-tooling/tree-sitter-elm](https://github.com/elm-tooling/tree-sitter-elm)
- Language Server: [elm-tooling/elm-language-server](https://github.com/elm-tooling/elm-language-server)

## Setting up `elm-language-server`

Elm language server can be configured in your `settings.json`, e.g.:

```json
{
  "lsp": {
    "elm-language-server": {
      "initialization_options": {
        "disableElmLSDiagnostics": true,
        "onlyUpdateDiagnosticsOnSave": false,
        "elmReviewDiagnostics": "warning"
      }
    }
  }
}
```

<!--
TBD: Add example of how to install `elm-format` and `elm-review`.
-->

`elm-format`, `elm-review` and `elm` need to be installed and made available in the environment or configured in the settings. See the [full list of server settings here](https://github.com/elm-tooling/elm-language-server?tab=readme-ov-file#server-settings).
