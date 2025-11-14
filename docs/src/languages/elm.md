# Elm

Elm support is available through the [Elm extension](https://github.com/zed-extensions/elm).

- Tree-sitter: [elm-tooling/tree-sitter-elm](https://github.com/elm-tooling/tree-sitter-elm)
- Language Server: [elm-tooling/elm-language-server](https://github.com/elm-tooling/elm-language-server)

## Setup

Zed support for Elm requires installation of `elm`, `elm-format`, and `elm-review`.

1. [Install Elm](https://guide.elm-lang.org/install/elm.html) (or run `brew install elm` on macOS).
2. Install `elm-review` to support code linting:
   ```sh
   npm install elm-review --save-dev
   ```
3. Install `elm-format` to support automatic formatting
   ```sh
   npm install -g elm-format
   ```

## Configuring `elm-language-server`

Elm language server can be configured in your `settings.json`, e.g.:

```json [settings]
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

`elm-format`, `elm-review` and `elm` need to be installed and made available in the environment or configured in the settings. See the [full list of server settings here](https://github.com/elm-tooling/elm-language-server?tab=readme-ov-file#server-settings).
