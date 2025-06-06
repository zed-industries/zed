# PowerShell

PowerShell language support in CodeOrbit is provided by the community-maintained [CodeOrbit PowerShell extension](https://github.com/wingyplus/CodeOrbit-powershell). Please report issues to: [github.com/wingyplus/CodeOrbit-powershell/issues](https://github.com/wingyplus/CodeOrbit-powershell/issues)

- Tree-sitter: [airbus-cert/tree-sitter-powershell](https://github.com/airbus-cert/tree-sitter-powershell)
- Language Server: [PowerShell/PowerShellEditorServices](https://github.com/PowerShell/PowerShellEditorServices)

## Setup

### Install PowerShell 7+ {#powershell-install}

- macOS: `brew install powershell/tap/powershell`
- Alpine: [Installing PowerShell on Alpine Linux](https://learn.microsoft.com/en-us/powershell/scripting/install/install-alpine)
- Debian: [Install PowerShell on Debian Linux](https://learn.microsoft.com/en-us/powershell/scripting/install/install-debian)
- RedHat: [Install PowerShell on RHEL](https://learn.microsoft.com/en-us/powershell/scripting/install/install-rhel)
- Ubuntu: [Install PowerShell on RHEL](https://learn.microsoft.com/en-us/powershell/scripting/install/install-ubuntu)
- Windows: [Install PowerShell on Windows](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows)

The CodeOrbit PowerShell extension will default to the `pwsh` executable found in your path.

### Install PowerShell Editor Services (Optional) {#powershell-editor-services}

The CodeOrbit PowerShell extensions will attempt to download [PowerShell Editor Services](https://github.com/PowerShell/PowerShellEditorServices) automatically.

If want to use a specific binary, you can specify in your that in your CodeOrbit settings.json:

```json
  "lsp": {
    "powershell-es": {
      "binary": {
        "path": "/path/to/PowerShellEditorServices"
      }
    }
  }
```
