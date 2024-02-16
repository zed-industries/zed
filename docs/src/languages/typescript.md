# TypeScript

- Tree Sitter: [tree-sitter-typescript](https://github.com/tree-sitter/tree-sitter-typescript)
- Language Server: [typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)

## Inlay Hints

Zed sets the following initialization options for inlay hints:

```json
"preferences": {
    "includeInlayParameterNameHints": "all",
    "includeInlayParameterNameHintsWhenArgumentMatchesName": true,
    "includeInlayFunctionParameterTypeHints": true,
    "includeInlayVariableTypeHints": true,
    "includeInlayVariableTypeHintsWhenTypeMatchesName": true,
    "includeInlayPropertyDeclarationTypeHints": true,
    "includeInlayFunctionLikeReturnTypeHints": true,
    "includeInlayEnumMemberValueHints": true,
}
```

to make the language server send back inlay hints when Zed has them enabled in the settings.

Use
```json
"lsp": {
    "$LANGUAGE_SERVER_NAME": {
        "initialization_options": {
            "preferences": {
                ....
            }
        }
    }
}
```
to override these settings.

See https://github.com/typescript-language-server/typescript-language-server?tab=readme-ov-file#inlay-hints-textdocumentinlayhint for more information.
