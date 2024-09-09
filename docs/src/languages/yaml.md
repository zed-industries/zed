# YAML

YAML support is available natively in Zed.

- Tree Sitter: [zed-industries/tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
- Language Server: [redhat-developer/yaml-language-server](https://github.com/redhat-developer/yaml-language-server)

## Configuration

You can configure various [yaml-language-server settings](https://github.com/redhat-developer/yaml-language-server?tab=readme-ov-file#language-server-settings) by adding them to your Zed settings.json in a `yaml-language-server` block under the `lsp` key. For example:

```json
  "lsp": {
    "yaml-language-server": {
      "initialization_options": {
        "yaml": {
          "keyOrdering": true,
          "format": {
            "singleQuote": true
          },
          "schemas": {
              "http://json.schemastore.org/composer": ["/*"],
              "../relative/path/schema.json": ["/config*.yaml"]
          }
        }
      }
    }
  }
```

Note, settings keys must be nested, so `yaml.keyOrdering` becomes `{"yaml": { "keyOrdering": true }}`.

## Schemas

By default yaml-language-server will attempt to determine the correct schema for a given yaml file and retrieve the appropriate JSON Schema from [Json Schema Store].

You can override this by [using an inlined schema] reference via a modeline comment at the top of your yaml file:

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/github-action.json
name: Issue Assignment
on:
  issues:
    types: [oppened]
```

You can disable this functionality entirely if desired:

```json
  "lsp": {
    "yaml-language-server": {
      "initialization_options": {
        "yaml": {
          "schemaStore": {
            "enable": false
          }
        }
      }
    }
  }
```
