# Ansible

Support for Ansible in Zed is provided via a community-maintained [Ansible extension](https://github.com/kartikvashistha/zed-ansible).

- Tree-sitter: [zed-industries/tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
- Language Server: [ansible/vscode-ansible](https://github.com/ansible/vscode-ansible/tree/main/packages/ansible-language-server)

## Setup

### File detection

By default, to avoid mishandling non-Ansible YAML files, the Ansible Language is not associated with any file extensions by default. To change this behavior you can add a `"file_types"` section to the Zed settings inside your project (`.zed/settings.json`) or your Zed user settings (`~/.config/zed/settings.json`) to match your folder/naming conventions. For example:

```json
"file_types": {
    "Ansible": [
      "**.ansible.yml",
      "**.ansible.yaml",
      "**/defaults/*.yml",
      "**/defaults/*.yaml",
      "**/meta/*.yml",
      "**/meta/*.yaml",
      "**/tasks/*.yml",
      "**/tasks/*.yaml",
      "**/handlers/*.yml",
      "**/handlers/*.yaml",
      "**/group_vars/*.yml",
      "**/group_vars/*.yaml",
      "**/playbooks/*.yml",
      "**/playbooks/*.yaml",
      "**playbook*.yml",
      "**playbook*.yaml"
    ]
  }
```

Feel free to modify this list as per your needs.

### LSP Configuration

LSP options for this extension can be configured under Zed's settings file. To get the best experience, add the following configuration under the `"lsp"` section in your `~/.zed/settings.json`:

```json
"lsp": {
  // Note, the Zed Ansible extension prefixes all settings with `ansible`
  // so instead of using `ansible.ansible.path` use `ansible.path`.
  "ansible-language-server": {
    "settings": {
      "ansible": {
        "path": "ansible"
      },
      "executionEnvironment": {
        "enabled": false
      },
      "python": {
        "interpreterPath": "python3"
      },
      "validation": {
        "enabled": true,
        // To enable linting, manually install ansible-lint and make sure it is your PATH
        "lint": {
          "enabled": true,
          "path": "ansible-lint"
        }
      }
    }
  }
}
```

This config was conveniently adopted from [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig/blob/ad32182cc4a03c8826a64e9ced68046c575fdb7d/lua/lspconfig/server_configurations/ansiblels.lua#L6-L23).

A full list of options/settings, that can be passed to the server, can be found at the project's page [here](https://github.com/ansible/vscode-ansible/blob/5a89836d66d470fb9d20e7ea8aa2af96f12f61fb/docs/als/settings.md).
Feel free to modify option values as needed.
