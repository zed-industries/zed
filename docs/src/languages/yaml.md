# YAML

YAML support is available natively in Zed.

- Tree Sitter: [zed-industries/tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
- Language Server: [redhat-developer/yaml-language-server](https://github.com/redhat-developer/yaml-language-server)

## Configuration

You can configure various [yaml-language-server settings](https://github.com/redhat-developer/yaml-language-server?tab=readme-ov-file#language-server-settings) by adding them to your Zed settings.json in a `yaml-language-server` block under the `lsp` key. For example:

```json
  "lsp": {
    "yaml-language-server": {
      "settings": {
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

By default yaml-language-server will attempt to determine the correct schema for a given yaml file and retrieve the appropriate JSON Schema from [Json Schema Store](https://schemastore.org/).

You can override any auto-detected schema via the `schemas` settings key (demonstrated above) or by providing an [inlined schema](https://github.com/redhat-developer/yaml-language-server#using-inlined-schema) reference via a modeline comment at the top of your yaml file:

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/github-action.json
name: Issue Assignment
on:
  issues:
    types: [oppened]
```

You can disable the automatic detection and retrieval of schemas from the JSON Schema if desired:

```json
  "lsp": {
    "yaml-language-server": {
      "settings": {
        "yaml": {
          "schemaStore": {
            "enable": false
          }
        }
      }
    }
  }
```

## Custom Tags

Yaml-language-server supports [custom tags](https://github.com/redhat-developer/yaml-language-server#adding-custom-tags) which can be used to inject custom application functionality at runtime into your yaml files.

For example Amazon CloudFormation YAML uses a number of custom tags, to support these you can add the following to your settings.json:

```json
  "lsp": {
    "yaml-language-server": {
      "settings": {
        "yaml": {
          "customTags": [
            "!And scalar",
            "!And mapping",
            "!And sequence",
            "!If scalar",
            "!If mapping",
            "!If sequence",
            "!Not scalar",
            "!Not mapping",
            "!Not sequence",
            "!Equals scalar",
            "!Equals mapping",
            "!Equals sequence",
            "!Or scalar",
            "!Or mapping",
            "!Or sequence",
            "!FindInMap scalar",
            "!FindInMap mapping",
            "!FindInMap sequence",
            "!Base64 scalar",
            "!Base64 mapping",
            "!Base64 sequence",
            "!Cidr scalar",
            "!Cidr mapping",
            "!Cidr sequence",
            "!Ref scalar",
            "!Ref mapping",
            "!Ref sequence",
            "!Sub scalar",
            "!Sub mapping",
            "!Sub sequence",
            "!GetAtt scalar",
            "!GetAtt mapping",
            "!GetAtt sequence",
            "!GetAZs scalar",
            "!GetAZs mapping",
            "!GetAZs sequence",
            "!ImportValue scalar",
            "!ImportValue mapping",
            "!ImportValue sequence",
            "!Select scalar",
            "!Select mapping",
            "!Select sequence",
            "!Split scalar",
            "!Split mapping",
            "!Split sequence",
            "!Join scalar",
            "!Join mapping",
            "!Join sequence",
            "!Condition scalar",
            "!Condition mapping",
            "!Condition sequence"
          ]
        }
      }
    }
  }
```
