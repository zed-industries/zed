# Go

Go support is available natively in Zed.

- Tree Sitter: [tree-sitter/tree-sitter-go](https://github.com/tree-sitter/tree-sitter-go)
- Language Server: [golang/tools/tree/master/gopls](https://github.com/golang/tools/tree/master/gopls)

## Inlay Hints

Zed sets the following initialization options for inlay hints:

```json
"hints": {
    "assignVariableTypes": true,
    "compositeLiteralFields": true,
    "compositeLiteralTypes": true,
    "constantValues": true,
    "functionTypeParameters": true,
    "parameterNames": true,
    "rangeVariableTypes": true
}
```

to make the language server send back inlay hints when Zed has them enabled in the settings.

Use

```json
"lsp": {
    "gopls": {
        "initialization_options": {
            "hints": {
                ....
            }
        }
    }
}
```

to override these settings.

See [gopls inlayHints documentation](https://github.com/golang/tools/blob/master/gopls/doc/inlayHints.md) for more information.

## Go Mod

- Tree Sitter: [camdencheek/tree-sitter-go-mod](https://github.com/camdencheek/tree-sitter-go-mod)
- Language Server: N/A

## Go Sum

- Tree Sitter: [amaanq/tree-sitter-go-sum](https://github.com/amaanq/tree-sitter-go-sum)
- Language Server: N/A

## Go Work

- Tree Sitter:
  [tree-sitter-go-work](https://github.com/d1y/tree-sitter-go-work)
- Language Server: N/A
