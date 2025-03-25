# Haskell

Haskell support is available through the [Haskell extension](https://github.com/zed-extensions/haskell).

- Tree-sitter: [tree-sitter-haskell](https://github.com/tree-sitter/tree-sitter-haskell)
- Language Server: [haskell-language-server](https://github.com/haskell/haskell-language-server)

## Installing HLS

Recommended method to [install haskell-language-server](https://haskell-language-server.readthedocs.io/en/latest/installation.html) (HLS) is via [ghcup](https://www.haskell.org/ghcup/install/) (`curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh
`):

```sh
ghcup install hls
which haskell-language-server-wrapper
```

## Configuring HLS

If you need to configure haskell-language-server (hls) you can add configuration options to your Zed settings.json:

```json
{
  "lsp": {
    "hls": {
      "initialization_options": {
        "haskell": {
          "formattingProvider": "fourmolu"
        }
      }
    }
  }
}
```

See: official [configuring haskell-language-server](https://haskell-language-server.readthedocs.io/en/latest/configuration.html) docs for more.
