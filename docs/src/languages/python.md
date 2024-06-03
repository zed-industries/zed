# Python

- Tree Sitter: [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python)
- Language Server: [pyright](https://github.com/microsoft/pyright)

### Configuration

The [pyright](https://github.com/microsoft/pyright) language server offers flexible configuration options specified in a JSON-formatted text configuration. By default, the file is called `pyrightconfig.json` and is located within the root directory of your project. Pyright settings can also be specified in a `[tool.pyright]` section of a `pyproject.toml` file. A `pyrightconfig.json` file always takes precedent over `pyproject.toml` if both are present.

For more information, see the Pyright [configuration documentation](https://microsoft.github.io/pyright/#/configuration).

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

### Code formatting

The Pyright language server does not provide code formatting. If you want to automatically reformat your Python code when saving, you'll need to specify an \_external_code formatter in your settings. See the [configuration](../configuring-zed.md) documentation for more information.

A common tool for formatting python code is [Black](https://black.readthedocs.io/en/stable/). If you have Black installed globally, you can use it to format Python files by adding the following to your `settings.json`:

```json
{
  "languages": {
    "Python": {
      "format_on_save": {
        "external": {
          "command": "black",
          "arguments": ["-"]
        }
      }
    }
  }
}
```
