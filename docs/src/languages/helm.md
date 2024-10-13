# Helm

Support for `helm` in Zed is provided by the community-maintained [Helm extension](https://github.com/cabrinha/helm.zed).

- Tree-sitter: [tree-sitter-go-template](https://github.com/ngalaiko/tree-sitter-go-template/tree/master)
- Language Server: [mrjosh/helm-ls](https://github.com/mrjosh/helm-ls)

## Setup

Enable the helm filetypes by editing you `.zed/settings.json` and adding:

```json
  "file_types": {
    "Helm": [
      "**/templates/**/*.tpl",
      "**/templates/**/*.yaml",
      "**/templates/**/*.yml",
      "**/helmfile.d/**/*.yaml"
    ]
  }
```
