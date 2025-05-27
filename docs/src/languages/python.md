# Python

Python support is available natively in Zed.

- Tree-sitter: [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python)
- Language Servers:
  - [microsoft/pyright](https://github.com/microsoft/pyright)
  - [python-lsp/python-lsp-server](https://github.com/python-lsp/python-lsp-server) (PyLSP)

## Language Servers

Zed supports multiple Python language servers some of which may require configuration to work properly.

See: [Working with Language Servers](https://zed.dev/docs/configuring-languages#working-with-language-servers) for more information.

## Virtual Environments in the Terminal {#terminal-detect_venv}

Zed will detect Python virtual environments and automatically activate them in terminal if available.
See: [detect_venv documentation](../configuring-zed.md#terminal-detect_venv) for more.

## PyLSP

[python-lsp-server](https://github.com/python-lsp/python-lsp-server/), more commonly known as PyLSP, by default integrates with a number of external tools (autopep8, mccabe, pycodestyle, yapf) while others are optional and must be explicitly enabled and configured (flake8, pylint).

See [Python Language Server Configuration](https://github.com/python-lsp/python-lsp-server/blob/develop/CONFIGURATION.md) for more.

## PyRight

### PyRight Configuration

The [pyright](https://github.com/microsoft/pyright) language server offers flexible configuration options specified in a JSON-formatted text configuration. By default, the file is called `pyrightconfig.json` and is located within the root directory of your project. Pyright settings can also be specified in a `[tool.pyright]` section of a `pyproject.toml` file. A `pyrightconfig.json` file always takes precedence over `pyproject.toml` if both are present.

For more information, see the Pyright [configuration documentation](https://microsoft.github.io/pyright/#/configuration).

### PyRight Settings

The [pyright](https://github.com/microsoft/pyright) language server also accepts specific LSP-related settings, not necessarily connected to a project. These can be changed in the `lsp` section of your `settings.json`.

For example, in order to:

- use strict type-checking level
- diagnose all files in the workspace instead of the only open files default
- provide the path to a specific Python interpreter

```json
{
  "lsp": {
    "pyright": {
      "settings": {
        "python.analysis": {
          "diagnosticMode": "workspace",
          "typeCheckingMode": "strict"
        },
        "python": {
          "pythonPath": ".venv/bin/python"
        }
      }
    }
  }
}
```

For more information, see the Pyright [settings documentation](https://microsoft.github.io/pyright/#/settings).

### Pyright Virtual environments

A Python [virtual environment](https://docs.python.org/3/tutorial/venv.html) allows you to store all of a project's dependencies, including the Python interpreter and package manager, in a single directory that's isolated from any other Python projects on your computer.

By default, the Pyright language server will look for Python packages in the default global locations. But you can also configure Pyright to use the packages installed in a given virtual environment.

To do this, create a JSON file called `pyrightconfig.json` at the root of your project. This file must include two keys:

- `venvPath`: a relative path from your project directory to any directory that _contains_ one or more virtual environment directories
- `venv`: the name of a virtual environment directory

For example, a common approach is to create a virtual environment directory called `.venv` at the root of your project directory with the following commands:

```sh
# create a virtual environment in the .venv directory
python3 -m venv .venv
# set up the current shell to use that virtual environment
source .venv/bin/activate
```

Having done that, you would create a `pyrightconfig.json` with the following content:

```json
{
  "venvPath": ".",
  "venv": ".venv"
}
```

If you prefer to use a `pyproject.toml` file, you can add the following section:

```toml
[tool.pyright]
venvPath = "."
venv = ".venv"
```

You can also configure this option directly in your `settings.json` file ([pyright settings](#pyright-settings)), as recommended in [Configuring Your Python Environment](https://microsoft.github.io/pyright/#/import-resolution?id=configuring-your-python-environment).

```json
{
  "lsp": {
    "pyright": {
      "settings": {
        "python": {
          "pythonPath": ".venv/bin/python"
        }
      }
    }
  }
}
```

### Code formatting & Linting

The Pyright language server does not provide code formatting or linting. If you want to detect lint errors and reformat your Python code upon saving, you'll need to set up.

A common tool for formatting Python code is [Ruff](https://docs.astral.sh/ruff/). It is another tool written in Rust, an extremely fast Python linter and code formatter. It is available through the [Ruff extension](https://github.com/zed-industries/zed/tree/main/extensions/ruff/). To configure the Ruff extension to work within Zed, see the setup documentation [here](https://docs.astral.sh/ruff/editors/setup/#zed).

<!--
TBD: Expand Python Ruff docs.
TBD: Ruff pyproject.toml, ruff.toml docs. `ruff.configuration`.
-->
