# Python

Python support is available natively in Zed.

- Tree-sitter: [tree-sitter-python](https://github.com/zed-industries/tree-sitter-python)
- Language Servers:
  - [DetachHead/basedpyright](https://github.com/DetachHead/basedpyright)
  - [astral-sh/ruff](https://github.com/astral-sh/ruff)
  - [astral-sh/ty](https://github.com/astral-sh/ty)
  - [microsoft/pyright](https://github.com/microsoft/pyright)
  - [python-lsp/python-lsp-server](https://github.com/python-lsp/python-lsp-server) (PyLSP)
- Debug Adapter: [debugpy](https://github.com/microsoft/debugpy)

## Language Servers

Zed provides several Python language servers by default. By default, [basedpyright](https://github.com/DetachHead/basedpyright) is used as the primary language server, and [Ruff](https://github.com/astral-sh/ruff) is used for formatting. Other language servers are disabled by default, but can be enabled in your settings. For example:

```json
{
  "languages": {
    "Python": {
      "language_servers": {
        // Disable basedpyright and enable pylsp, and otherwise use the default configuration.
        "pylsp", "!basedpyright", ".."
      }
    }
  }
}
```

See: [Working with Language Servers](https://zed.dev/docs/configuring-languages#working-with-language-servers) for more information about how to enable and disable language servers.

### Basedpyright

[basedpyright](https://docs.basedpyright.com/latest/) replaced [Pyright](https://github.com/microsoft/pyright) as the primary Python language server beginning with Zed v0.204.0. It provides support for core language server functionality like navigation (go to definition/find all references) and type checking. Compared to Pyright, it adds support for additional language server features (like inlay hints) and checking rules.

Note that while basedpyright itself defaults to the `recommended` [type-checking mode](https://docs.basedpyright.com/latest/benefits-over-pyright/better-defaults/#typecheckingmode), Zed configures it to use the less-strict `standard` mode by default, which matches the behavior of Pyright. This Zed-specific override is not applied if your project has any basedpyright (or Pyright) configuration (see below), allowing you to configure your preferred type-checking mode in each project.

#### Basedpyright Configuration

basedpyright offers flexible configuration options specified in a JSON-formatted text configuration. By default, the file is called `pyrightconfig.json` and is located within the root directory of your project. basedpyright settings can also be specified in a `[tool.basedpyright]` (or `[tool.pyright]`) section of a `pyproject.toml` file. A `pyrightconfig.json` file always takes precedence over `pyproject.toml` if both are present.

For more information, see the basedpyright [configuration documentation](https://docs.basedpyright.com/latest/configuration/config-files/).

#### Basedpyright Settings

basedpyright also accepts specific LSP-related settings, not necessarily connected to a project. These can be changed in the `lsp` section of your `settings.json`.

For example, in order to:

- use strict type-checking level
- diagnose all files in the workspace instead of the only open files default

```json
{
  "lsp": {
    "basedpyright": {
      "settings": {
        "basedpyright.analysis": {
          "diagnosticMode": "workspace",
          "typeCheckingMode": "strict"
        }
      }
    }
  }
}
```

For more information, see the basedpyright [settings documentation](https://docs.basedpyright.com/latest/configuration/language-server-settings/).

## PyLSP

[python-lsp-server](https://github.com/python-lsp/python-lsp-server/), more commonly known as PyLSP, by default integrates with a number of external tools (autopep8, mccabe, pycodestyle, yapf) while others are optional and must be explicitly enabled and configured (flake8, pylint).

See [Python Language Server Configuration](https://github.com/python-lsp/python-lsp-server/blob/develop/CONFIGURATION.md) for more.

## Virtual environments

Many Python projects use [virtual environments](https://docs.python.org/3/library/venv.html) to manage a project-specific Python toolchain and set of installed dependencies; in larger projects, multiple virtual environments may be used, each covering a different part of the codebase. Zed uses the [Python Environment Tools](https://github.com/microsoft/python-environment-tools) library to discover all relevant virtual environments (and other Python toolchains) when opening a project, and it will automatically start language server instances that use the appropriate toolchain and set of dependencies for each part of the codebase.

## Virtual Environments in the Terminal {#terminal-detect_venv}

Zed will detect Python virtual environments and automatically activate them in terminal if available.
See: [detect_venv documentation](../configuring-zed.md#terminal-detect_venv) for more.

## Code formatting & Linting

Zed provides the [Ruff](https://docs.astral.sh/ruff/) formatter and linter, which is enabled by default. (Specifically, Zed runs Ruff as an LSP server using the `ruff server` subcommand.) Ruff has many configurable options, which can be set in the following places, among others:

- The `ruff.toml` configuration file
- The `[tool.ruff]` section of a `pyproject.toml` manifest
- The language server initialization options, configured in Zed's `settings.json`

For example, to disable all Ruff linting in your project, and configure the formatter to use a custom line width, you can add the following configuration to `ruff.toml` at the root of your project:

```toml
line-length = 100

[lint]
exclude = ["*"]
```

## Debugging

Zed supports zero-configuration debugging of Python module entry points and pytest tests.
Run {#action debugger::Start} ({#kb debugger::Start}) to see a contextual list for the current project.
For greater control, you can add debug configurations to `.zed/debug.json`. See the examples below.

### Debug Active File

```json
[
  {
    "label": "Python Active File",
    "adapter": "Debugpy",
    "program": "$ZED_FILE",
    "request": "launch"
  }
]
```

### Flask App

For a common Flask Application with a file structure similar to the following:

```
.venv/
app/
  init.py
  main.py
  routes.py
templates/
  index.html
static/
  style.css
requirements.txt
```

…the following configuration can be used:

```json
[
  {
    "label": "Python: Flask",
    "adapter": "Debugpy",
    "request": "launch",
    "module": "app",
    "cwd": "$ZED_WORKTREE_ROOT",
    "env": {
      "FLASK_APP": "app",
      "FLASK_DEBUG": "1"
    },
    "args": [
      "run",
      "--reload", // Enables Flask reloader that watches for file changes
      "--debugger" // Enables Flask debugger
    ],
    "autoReload": {
      "enable": true
    },
    "jinja": true,
    "justMyCode": true
  }
]
```
