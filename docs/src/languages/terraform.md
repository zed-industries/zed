# Terraform

- Tree Sitter: [tree-sitter-hcl](https://github.com/MichaHoffmann/tree-sitter-hcl)
- Language Server: [terraform-ls](https://github.com/hashicorp/terraform-ls)

### Configuration

The Terraform language server can be configured in your `settings.json`, e.g.:

```json
{
  "lsp": {
    "terraform-ls": {
      "initialization_options": {
        "experimentalFeatures": {
          "prefillRequiredFields": true
        }
      }
    }
  }
}
```

See the [full list of server settings here](https://github.com/hashicorp/terraform-ls/blob/main/docs/SETTINGS.md).
