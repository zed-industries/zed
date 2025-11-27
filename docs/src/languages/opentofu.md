# OpenTofu

OpenTofu support is available through the [OpenTofu extension](https://github.com/ashpool37/zed-extension-opentofu).

- Tree-sitter: [MichaHoffmann/tree-sitter-hcl](https://github.com/MichaHoffmann/tree-sitter-hcl)
- Language Server: [opentofu/tofu-ls](https://github.com/opentofu/tofu-ls)

## Configuration

In order to automatically use the OpenTofu extension and language server when editing .tf and .tfvars files,
either uninstall the Terraform extension or add this to your settings.json:

```json
"file_types": {
  "OpenTofu": ["tf"],
  "OpenTofu Vars": ["tfvars"]
},
```

See the [full list of server settings here](https://github.com/opentofu/tofu-ls/blob/main/docs/SETTINGS.md).
