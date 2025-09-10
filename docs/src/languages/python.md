# How to Set Up Python in Zed

Python support is available natively in Zed.

- Tree-sitter: [tree-sitter-python](https://github.com/zed-industries/tree-sitter-python)
- Language Servers:
  - [DetachHead/basedpyright](https://github.com/DetachHead/basedpyright)
  - [astral-sh/ruff](https://github.com/astral-sh/ruff)
  - [astral-sh/ty](https://github.com/astral-sh/ty)
  - [microsoft/pyright](https://github.com/microsoft/pyright)
  - [python-lsp/python-lsp-server](https://github.com/python-lsp/python-lsp-server) (PyLSP)
- Debug Adapter: [debugpy](https://github.com/microsoft/debugpy)

## Install Zed and Python on Your Machine
Zed supports Python development natively. You'll need both Zed and Python installed before you can begin.

### Step 1: Install Zed
- Go to [zed.dev/download](https://zed.dev/download) and download the latest release.
- Open the `.dmg` file and drag Zed into your Applications folder.
- Launch Zed. If on Mac, You may need to right-click > Open to bypass macOS Gatekeeper on first launch.

### Step 2: Install Python
Zed does not bundle a Python runtime, so you’ll need to install one yourself.
Choose one of the following options:
- Astral (recommended):
```json
curl -LsSf https://astral.sh/uv/install.sh | sh
```
To learn more, visit Astral’s installation guide
- Homebrew:
```json
brew install python
```
- Python.org installer: Download the latest version from [python.org/downloads](https://python.org/downloads).

### Step 3: Verify Python Installation
Confirm Python is installed and available in your shell:
```json
uv --version
```
You should see an output like `Python 3.x.x`.

## Open Your First Python Project in Zed
Once Zed and Python are installed, open a folder containing Python code to start working.

### Step 1: Launch Zed with a Python Project
Open Zed.
From the menu bar, choose File > Open Folder, or launch from the terminal:
`zed path/to/your/project`

Zed will recognize `.py` files automatically using its native tree-sitter-python parser, with no plugins or manual setup required.

### Step 2: Use the Integrated Terminal (Optional)
Zed includes a terminal, accessible from the bottom panel (). You can also use your external terminal of choice.

From the terminal, verify you’re in the correct environment:
`python3 script.py`

If you’re using a virtual environment, Zed will attempt to auto-activate it in the terminal if it detects one (see `detect_venv` behavior in docs).


## Configure Python Language Servers in Zed

Zed provides several Python language servers by default. By default, [basedpyright](https://github.com/DetachHead/basedpyright) is the primary language server, and [Ruff](https://github.com/astral-sh/ruff) is used for formatting. Other language servers are disabled by default, but can be enabled in your settings. For example:

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

#### Configure Basedpyright

basedpyright offers flexible configuration options specified in a JSON-formatted text configuration. 

By default, the file is called `pyrightconfig.json` and is located within the root directory of your project. basedpyright settings can also be specified in a `[tool.basedpyright]` (or `[tool.pyright]`) section of a `pyproject.toml` file. A `pyrightconfig.json` file always takes precedence over `pyproject.toml` if both are present.

For more information, see the basedpyright [configuration documentation](https://docs.basedpyright.com/latest/configuration/config-files/).

#### Basedpyright Settings

basedpyright also accepts specific LSP-related settings, not necessarily connected to a project. 

For example, in order to:

- Use strict type-checking level
- Diagnose all files in the workspace instead of the only open files default

These can be changed in the `lsp` section of your `settings.json`.
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

## Configure PyLSP

[python-lsp-server](https://github.com/python-lsp/python-lsp-server/), more commonly known as PyLSP, by default integrates with a number of external tools (autopep8, mccabe, pycodestyle, yapf) while others are optional and must be explicitly enabled and configured (flake8, pylint).

See [Python Language Server Configuration](https://github.com/python-lsp/python-lsp-server/blob/develop/CONFIGURATION.md) for more.

## Set Up and Activate Python Virtual Environments

If you don’t already have a virtual environment set up on your computer, follow these steps to create one. Use a [virtual environments](https://docs.python.org/3/library/venv.html) to isolate your project’s dependencies and interpreter. Zed detects and activates virtual environments automatically in its terminal.

### Create and Activate a Virtual Environment
In your project root:
`python3 -m venv .venv
source .venv/bin/activate`

Zed will recognize `.venv` and activate it in the terminal without extra configuration.

### Link Virtual Environment to Language Server
For Pyright, create a `pyrightconfig.json` at the root of your project:
``` json {
  "venvPath": ".",
  "venv": ".venv"
}
```

Or, if you're using pyproject.toml, add:
``` json 
[tool.pyright]
venvPath = "."
venv = ".venv"`
```

You can also set the path directly in `settings.json` as shown above. This ensures Pyright uses the correct interpreter and dependencies when analyzing your code.

## Virtual Environments in the Terminal {#terminal-detect_venv}

Zed will detect Python virtual environments and automatically activate them in terminal if available.
See: [detect_venv documentation](../configuring-zed.md#terminal-detect_venv) for more.

## Fine-Tune Pyright for Python IDE Features
Pyright can be configured to match your preferred level of type safety and analysis depth. It supports project-level and workspace-level settings.

### Control Diagnostics and Type Checking
In your `pyrightconfig.json` or `pyproject.toml`, specify how aggressively Pyright should analyze your code:
``` json
{
  "typeCheckingMode": "strict",
  "diagnosticMode": "workspace"
}
```
- typeCheckingMode: Accepts "off", "basic", or "strict".
- diagnosticMode: Use "openFilesOnly" or "workspace" to control scope.

These options can also be set in settings.json under the python.analysis key.

### Set the Python Interpreter Path
Point Pyright to a specific interpreter—typically within a virtual environment—so it can resolve imports and packages correctly.

In `settings.json`:
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

This is useful when working across projects with different environments.

## Code Formatting & Linting

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

Zed supports Python debugging through the `debugpy` adapter. You can start with no configuration or define custom launch profiles in `.zed/debug.json`.

### Start Debugging with No Setup
Zed can automatically detect debuggable Python entry points. Press F4 (or run debugger: start from the Command Palette) to see available options for your current project.
This works for:
- Python scripts
- Modules
-  pytest tests

Zed uses `debugpy` under the hood, but no manual adapter configuration is required.

### Define Custom Debug Configurations
For reusable setups, create a `.zed/debug.json` file in your project root. This gives you more control over how Zed runs and debugs your code.

#### Debug Active File

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
This runs the file currently open in the editor.

#### Debug a Flask App

For projects using Flask, you can define a full launch configuration:

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
These can be combined to tailor the experience for web servers, test runners, or custom scripts.

## Troubleshoot and Maintain a Productive Python Setup
Zed is designed to minimize configuration overhead, but occasional issues can still arise—especially around environments, language servers, or tooling. Here's how to keep your Python setup working smoothly.

### Resolve Language Server Startup Issues
If a language server isn't responding or features like diagnostics or autocomplete aren't available:
- Confirm the language server is installed and available in your environment (e.g., `pyright` or `pylsp`).
- Verify your `settings.json` or `pyrightconfig.json` is syntactically correct.
- Restart Zed to reinitialize language server connections.
- If using a virtual environment, check that the language server is installed inside that environment.

### Diagnose Environment Detection Failures
Zed will attempt to detect and activate a virtual environment in its terminal using `detect_venv`. If this fails:
- Confirm your environment is located at `.venv/` in the project root.
- Ensure the environment was created using `python3 -m venv .venv`.
- Manually activate the environment in the terminal to verify it's working.
- If using Pyright, ensure venvPath and venv are correctly set in `pyrightconfig.json`.

### Keep Zed and Language Tools Up to Date
Outdated tools can cause silent failures or missing features. Periodically:
- Update Zed from [zed.dev](https://zed.dev) or using the in-app updater.
- Run `pip install --upgrade` for tools like `pyright`, `pylsp`, `flake8`, etc.
- Recreate your virtual environment if dependencies become inconsistent or corrupted.

### Review Logs and Terminal Output
If something breaks and it's unclear why:
- Check the Zed terminal for environment activation logs or Python errors.
- Use verbose flags (e.g., --verbose for some CLI tools) to get more detailed output.
- Validate that paths and environment variables are what you expect.
- Taking a few minutes to inspect the output can often reveal the root cause faster than reconfiguring settings blindly.
