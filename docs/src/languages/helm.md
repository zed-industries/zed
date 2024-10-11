# Helm

Support for `helm` in Zed is provided by community-maintained extensions.

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

- Language Server: [tree-sitter-go-template](https://github.com/ngalaiko/tree-sitter-go-template/tree/master)
