# Python

Python support is available natively in Zed.

- Tree Sitter: [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python)
- Language Server: [pyright](https://github.com/microsoft/pyright)

### Configuration

The [pyright](https://github.com/microsoft/pyright) language server offers flexible configuration options specified in a JSON-formatted text configuration. By default, the file is called `pyrightconfig.json` and is located within the root directory of your project. Pyright settings can also be specified in a `[tool.pyright]` section of a `pyproject.toml` file. A `pyrightconfig.json` file always takes precedent over `pyproject.toml` if both are present.

For more information, see the Pyright [configuration documentation](https://microsoft.github.io/pyright/#/configuration).

### Settings

The [pyright](https://github.com/microsoft/pyright) language server also accepts specific LSP related settings, not necessarily connected to a project. These can be changed in the `lsp` section of your `settings.json`.

For example, in order to:

- use strict type-checking level
- diagnose all files in the workspace instead of the only open files default
- provide the path to a specific python interpreter

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

### Virtual environments

A python [virtual environment](https://docs.python.org/3/tutorial/venv.html) allows you to store all of a project's dependencies, including the Python interpreter and package manager, in a single directory that's isolated from any other Python projects on your computer.

By default, the Pyright language server will look for Python packages in the default global locations. But you can also configure Pyright to use the packages installed in a given virtual environment.

To do this, create a JSON file called `pyrightconfig.json` at the root of your project. This file must include two keys:

- `venvPath`: a relative path from your project directory to any directory that _contains_ one or more virtual environment directories
- `venv`: the name of a virtual environment directory

For example, a common approach is to create a virtual environment directory called `.venv` at the root of your project directory with the following commands:

```bash
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

You can also configure this option directly in your `settings.json` file ([pyrights settings](#settings)), as recommended in [Configuring Your Python Environment](https://microsoft.github.io/pyright/#/import-resolution?id=configuring-your-python-environment).

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

A common tool for formatting Python code is [Ruff](https://black.readthedocs.io/en/stable/). It is another tool written in Rust, an extremely fast Python linter and code formatter.

It is available through the [Ruff extension](https://docs.astral.sh/ruff/). However, the code formatting through the extension is not yet available. You can set up the formatter to run on save by adding the following configuration to your `settings.json`, assuming that [`Ruff`](https://docs.astral.sh/ruff/) is installed in your Python environment.

```json
{
  "languages": {
    ..., // other languages
    "Python": {
      "format_on_save": {
        "external": {
          "command": "python",
          "arguments": [
            "-m",
            "ruff",
            "format",
            "-"
          ]
        }
      }
    }
  }
}
```
