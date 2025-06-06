# Ansible

Support for Ansible in CodeOrbit is provided via a community-maintained [Ansible extension](https://github.com/kartikvashistha/CodeOrbit-ansible).

- Tree-sitter: [CodeOrbit-industries/tree-sitter-yaml](https://github.com/CodeOrbit-industries/tree-sitter-yaml)
- Language Server: [ansible/vscode-ansible](https://github.com/ansible/vscode-ansible/tree/main/packages/ansible-language-server)

## Setup

### File detection

To avoid mishandling non-Ansible YAML files, the Ansible Language is not associated with any file extensions by default. To change this behavior you can add a `"file_types"` section to CodeOrbit settings inside your project (`.CodeOrbit/settings.json`) or your CodeOrbit user settings (`~/.config/CodeOrbit/settings.json`) to match your folder/naming conventions. For example:

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
      "**/host_vars/*.yml",
      "**/host_vars/*.yaml",
      "**/playbooks/*.yml",
      "**/playbooks/*.yaml",
      "**playbook*.yml",
      "**playbook*.yaml"
    ]
  }
```

Feel free to modify this list as per your needs.

#### Inventory

If your inventory file is in the YAML format, you can either:

- Append the `ansible-lint` inventory json schema to it via the following comment at the top of your inventory file:

```yml
# yaml-language-server: $schema=https://raw.githubusercontent.com/ansible/ansible-lint/main/src/ansiblelint/schemas/inventory.json
```

- Or configure the yaml language server settings to set this schema for all your inventory files, that match your inventory pattern, under your CodeOrbit settings ([ref](https://CodeOrbit.dev/docs/languages/yaml)):

```json
"lsp": {
    "yaml-language-server": {
      "settings": {
        "yaml": {
          "schemas": {
            "https://raw.githubusercontent.com/ansible/ansible-lint/main/src/ansiblelint/schemas/inventory.json": [
              "./inventory/*.yaml",
              "hosts.yml",
            ]
          }
        }
      }
    }
},
```

### LSP Configuration

By default, the following default config is passed to the Ansible language server. It conveniently mirrors the defaults set by [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig/blob/03bc581e05e81d33808b42b2d7e76d70adb3b595/lua/lspconfig/configs/ansiblels.lua) for the Ansible language server:

```json
{
  "ansible": {
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
      "lint": {
        "enabled": true,
        "path": "ansible-lint"
      }
    }
  }
}
```

> [!NOTE]
> In order for linting to work, ensure that `ansible-lint` is installed and discoverable on your PATH

When desired, any of the above default settings can be overridden under the `"lsp"` section of your CodeOrbit settings file. For example:

```json
"lsp": {
  // Note, the CodeOrbit Ansible extension prefixes all settings with `ansible`
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
        "enabled": false, // disable validation
        "lint": {
          "enabled": false, // disable ansible-lint
          "path": "ansible-lint"
        }
      }
    }
  }
}
```

A full list of options/settings, that can be passed to the server, can be found at the project's page [here](https://github.com/ansible/vscode-ansible/blob/5a89836d66d470fb9d20e7ea8aa2af96f12f61fb/docs/als/settings.md).
Feel free to modify option values as needed.
