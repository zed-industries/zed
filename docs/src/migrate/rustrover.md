# How to Migrate from RustRover to Zed

This guide covers how to set up Zed if you're coming from RustRover, including keybindings, settings, and the differences you should expect as a Rust developer.

## Install Zed

Zed is available on macOS, Windows, and Linux.

For macOS, you can download it from zed.dev/download, or install via Homebrew:

```sh
brew install --cask zed
```

For Windows, download the installer from zed.dev/download, or install via winget:

```sh
winget install Zed.Zed
```

For most Linux users, the easiest way to install Zed is through our installation script:

```sh
curl -f https://zed.dev/install.sh | sh
```

After installation, you can launch Zed from your Applications folder (macOS), Start menu (Windows), or directly from the terminal using:
`zed .`
This opens the current directory in Zed.

## Set Up the JetBrains Keymap

If you're coming from RustRover, the fastest way to feel at home is to use the JetBrains keymap. During onboarding, you can select it as your base keymap. If you missed that step, you can change it anytime:

1. Open Settings with `Cmd+,` (macOS) or `Ctrl+,` (Linux/Windows)
2. Search for `Base Keymap`
3. Select `JetBrains`

Or add this directly to your `settings.json`:

```json
{
  "base_keymap": "JetBrains"
}
```

This maps familiar shortcuts like `Shift Shift` for Search Everywhere, `Cmd+O` for Go to Class, and `Cmd+Shift+A` for Find Action.

## Set Up Editor Preferences

You can configure settings manually in the Settings Editor.

To edit your settings:

1. `Cmd+,` to open the Settings Editor.
2. Run `zed: open settings` in the Command Palette.

Settings RustRover users typically configure first:

| Zed Setting             | What it does                                                                    |
| ----------------------- | ------------------------------------------------------------------------------- |
| `format_on_save`        | Auto-format when saving. Set to `"on"` to enable (uses rustfmt by default).     |
| `soft_wrap`             | Wrap long lines. Options: `"none"`, `"editor_width"`, `"preferred_line_length"` |
| `preferred_line_length` | Column width for wrapping and rulers. Rust convention is 100.                   |
| `inlay_hints`           | Show type hints, parameter names, and chaining hints inline.                    |
| `relative_line_numbers` | Useful if you're coming from IdeaVim.                                           |

Zed also supports per-project settings. Create a `.zed/settings.json` file in your project root to override global settings for that project, similar to how you might use `.idea` folders in RustRover.

> **Tip:** If you're joining an existing project, check `format_on_save` before making your first commit. Otherwise you might accidentally reformat an entire file when you only meant to change one line.

## Open or Create a Project

After setup, press `Cmd+Shift+O` (with JetBrains keymap) to open a folder. This becomes your workspace in Zed. Unlike RustRover, there's no project configuration wizard, no toolchain selection dialog, and no Cargo project setup screen.

To start a new project, use Cargo from the terminal:

```sh
cargo new my_project
cd my_project
zed .
```

Or for a library:

```sh
cargo new --lib my_library
```

You can also launch Zed from the terminal inside any existing Cargo project with:
`zed .`

Once inside a project:

- Use `Cmd+Shift+O` or `Cmd+E` to jump between files quickly (like RustRover's "Recent Files")
- Use `Cmd+Shift+A` or `Shift Shift` to open the Command Palette (like RustRover's "Search Everywhere")
- Use `Cmd+O` to search for symbols (like RustRover's "Go to Symbol")

Open buffers appear as tabs across the top. The sidebar shows your file tree and Git status. Toggle it with `Cmd+1` (just like RustRover's Project tool window).

## Differences in Keybindings

If you chose the JetBrains keymap during onboarding, most of your shortcuts should already feel familiar. Here's a quick reference for how Zed compares to RustRover.

### Common Shared Keybindings

| Action                        | Shortcut                |
| ----------------------------- | ----------------------- |
| Search Everywhere             | `Shift Shift`           |
| Find Action / Command Palette | `Cmd + Shift + A`       |
| Go to File                    | `Cmd + Shift + O`       |
| Go to Symbol                  | `Cmd + O`               |
| Recent Files                  | `Cmd + E`               |
| Go to Definition              | `Cmd + B`               |
| Find Usages                   | `Alt + F7`              |
| Rename Symbol                 | `Shift + F6`            |
| Reformat Code                 | `Cmd + Alt + L`         |
| Toggle Project Panel          | `Cmd + 1`               |
| Toggle Terminal               | `Alt + F12`             |
| Duplicate Line                | `Cmd + D`               |
| Delete Line                   | `Cmd + Backspace`       |
| Move Line Up/Down             | `Shift + Alt + Up/Down` |
| Expand/Shrink Selection       | `Alt + Up/Down`         |
| Comment Line                  | `Cmd + /`               |
| Go Back / Forward             | `Cmd + [` / `Cmd + ]`   |
| Toggle Breakpoint             | `Ctrl + F8`             |

### Different Keybindings (RustRover → Zed)

| Action                 | RustRover   | Zed (JetBrains keymap)   |
| ---------------------- | ----------- | ------------------------ |
| File Structure         | `Cmd + F12` | `Cmd + F12` (outline)    |
| Navigate to Next Error | `F2`        | `F2`                     |
| Run                    | `Ctrl + R`  | `Ctrl + Alt + R` (tasks) |
| Debug                  | `Ctrl + D`  | `Alt + Shift + F9`       |
| Stop                   | `Cmd + F2`  | `Ctrl + F2`              |
| Expand Macro           | `Alt+Enter` | `Cmd + Shift + M`        |

### Unique to Zed

| Action            | Shortcut                   | Notes                          |
| ----------------- | -------------------------- | ------------------------------ |
| Toggle Right Dock | `Cmd + R`                  | Assistant panel, notifications |
| Split Panes       | `Cmd + K`, then arrow keys | Create splits in any direction |

### How to Customize Keybindings

- Open the Command Palette (`Cmd+Shift+A` or `Shift Shift`)
- Run `Zed: Open Keymap Editor`

This opens a list of all available bindings. You can override individual shortcuts or remove conflicts.

Zed also supports key sequences (multi-key shortcuts).

## Differences in User Interfaces

### No Indexing

RustRover indexes your project when you first open it to build a model of your codebase. This process runs whenever you open a project or when dependencies change via Cargo.

Zed skips the indexing step. You open a folder and start working right away. Since both editors rely on rust-analyzer for Rust intelligence, the analysis still happens—but in Zed it runs in the background without blocking the UI or showing modal progress dialogs.

**How to adapt:**

- Use `Cmd+O` to search symbols across your crate (rust-analyzer handles this)
- Jump to files by name with `Cmd+Shift+O`
- `Cmd+Shift+F` gives you fast text search across the entire project
- For linting and deeper checks, run `cargo clippy` in the terminal

### rust-analyzer: Shared Foundation, Different Integration

Here's what makes the RustRover-to-Zed transition unique: **both editors use rust-analyzer** for Rust language intelligence. This means the core code analysis—completions, go-to-definition, find references, type inference—is fundamentally the same.

RustRover integrates rust-analyzer into its JetBrains platform, adding a GUI layer, additional refactorings, and its own indexing on top. Zed uses rust-analyzer more directly through the Language Server Protocol (LSP).

What this means for you:

- **Completions** — Same quality, powered by rust-analyzer
- **Type inference** — Identical, it's the same engine
- **Go to definition / Find usages** — Works the same way
- **Macro expansion** — Available in both (use `Cmd+Shift+M` in Zed)
- **Inlay hints** — Both support type hints, parameter hints, and chaining hints

Where you might notice differences:

- Some refactorings available in RustRover may not have rust-analyzer equivalents
- RustRover's GUI for configuring rust-analyzer is replaced by JSON configuration in Zed
- RustRover-specific inspections (beyond Clippy) won't exist in Zed

**How to adapt:**

- Use `Alt+Enter` for available code actions—rust-analyzer provides many
- Configure rust-analyzer settings in `.zed/settings.json` for project-specific needs
- Run `cargo clippy` for linting (it integrates with rust-analyzer diagnostics)

### No Project Model

RustRover manages projects through `.idea` folders containing XML configuration files, toolchain assignments, and run configurations. The Cargo tool window provides a visual interface for your project structure, targets, and dependencies.

Zed keeps it simpler: a project is a folder with a `Cargo.toml`. No project wizard, no toolchain dialogs, no visual Cargo management layer.

In practice:

- Run configurations don't carry over. Your `.idea/` setup stays behind—define the commands you need in `tasks.json` instead.
- Toolchains are managed externally via `rustup`.
- Dependencies live in `Cargo.toml`. Edit the file directly; rust-analyzer provides completions for crate names and versions.

**How to adapt:**

- Create a `.zed/settings.json` in your project root for project-specific settings
- Define common commands in `tasks.json` (open via Command Palette: `zed: open tasks`):

```json
[
  {
    "label": "cargo run",
    "command": "cargo run"
  },
  {
    "label": "cargo build",
    "command": "cargo build"
  },
  {
    "label": "cargo test",
    "command": "cargo test"
  },
  {
    "label": "cargo clippy",
    "command": "cargo clippy"
  },
  {
    "label": "cargo run --release",
    "command": "cargo run --release"
  }
]
```

- Use `Ctrl+Alt+R` to run tasks quickly
- Lean on your terminal (`Alt+F12`) for anything tasks don't cover

### No Cargo Integration UI

RustRover's Cargo tool window provides visual access to your project's targets, dependencies, and common Cargo commands. You can run builds, tests, and benchmarks with a click.

Zed doesn't have a Cargo GUI. You work with Cargo through:

- **Terminal** — Run any Cargo command directly
- **Tasks** — Define shortcuts for common commands
- **Gutter icons** — Run tests and binaries with clickable icons

**How to adapt:**

- Get comfortable with Cargo CLI commands: `cargo build`, `cargo run`, `cargo test`, `cargo clippy`, `cargo doc`
- Use tasks for commands you run frequently
- For dependency management, edit `Cargo.toml` directly (rust-analyzer provides completions for crate names and versions)

### Tool Windows vs. Docks

RustRover organizes auxiliary views into numbered tool windows (Project = 1, Cargo = Alt+1, Terminal = Alt+F12, etc.). Zed uses a similar concept called "docks":

| RustRover Tool Window | Zed Equivalent | Shortcut (JetBrains keymap) |
| --------------------- | -------------- | --------------------------- |
| Project (1)           | Project Panel  | `Cmd + 1`                   |
| Git (9 or Cmd+0)      | Git Panel      | `Cmd + 0`                   |
| Terminal (Alt+F12)    | Terminal Panel | `Alt + F12`                 |
| Structure (7)         | Outline Panel  | `Cmd + 7`                   |
| Problems (6)          | Diagnostics    | `Cmd + 6`                   |
| Debug (5)             | Debug Panel    | `Cmd + 5`                   |

Zed has three dock positions: left, bottom, and right. Panels can be moved between docks by dragging or through settings.

Note that there's no dedicated Cargo tool window in Zed. Use the terminal or define tasks for your common Cargo commands.

### Debugging

Both RustRover and Zed offer integrated debugging for Rust, but using different backends:

- RustRover uses its own debugger integration
- Zed uses **CodeLLDB** (the same debug adapter popular in VS Code)

To debug Rust code in Zed:

- Set breakpoints with `Ctrl+F8`
- Start debugging with `Alt+Shift+F9` or press `F4` and select a debug target
- Step through code with `F7` (step into), `F8` (step over), `Shift+F8` (step out)
- Continue execution with `F9`

Zed can automatically detect debuggable targets in your Cargo project. Press `F4` to see available options.

For more control, create a `.zed/debug.json` file:

```json
[
  {
    "label": "Debug Binary",
    "adapter": "CodeLLDB",
    "request": "launch",
    "program": "${workspaceFolder}/target/debug/my_project"
  },
  {
    "label": "Debug Tests",
    "adapter": "CodeLLDB",
    "request": "launch",
    "cargo": {
      "args": ["test", "--no-run"],
      "filter": {
        "kind": "test"
      }
    }
  },
  {
    "label": "Debug with Arguments",
    "adapter": "CodeLLDB",
    "request": "launch",
    "program": "${workspaceFolder}/target/debug/my_project",
    "args": ["--config", "dev.toml"]
  }
]
```

> **Note:** Some users have reported that RustRover's debugger can have issues with variable inspection and breakpoints in certain scenarios. CodeLLDB in Zed provides a solid alternative, though debugging Rust can be challenging in any editor due to optimizations and macro-generated code.

### Running Tests

RustRover has a dedicated test runner with a visual interface showing pass/fail status for each test. Zed provides test running through:

- **Gutter icons** — Click the play button next to `#[test]` functions or test modules
- **Tasks** — Define `cargo test` commands in `tasks.json`
- **Terminal** — Run `cargo test` directly

The test output appears in the terminal panel. For more detailed output, use:

- `cargo test -- --nocapture` to see println! output
- `cargo test -- --test-threads=1` for sequential test execution
- `cargo test specific_test_name` to run a single test

### Extensions vs. Plugins

RustRover has a plugin ecosystem, though it's more limited than other JetBrains IDEs since Rust support is built-in.

Zed's extension ecosystem is smaller and more focused:

- Language support and syntax highlighting
- Themes
- Slash commands for AI
- Context servers

Several features that might require plugins in other editors are built into Zed:

- Real-time collaboration with voice chat
- AI coding assistance
- Built-in terminal
- Task runner
- rust-analyzer integration
- rustfmt formatting

### What's Not in Zed

To set expectations clearly, here's what RustRover offers that Zed doesn't have:

- **Cargo.toml GUI editor** — Edit the file directly (rust-analyzer helps with completions)
- **Visual dependency management** — Use `cargo add`, `cargo remove`, or edit `Cargo.toml`
- **Profiler integration** — Use `cargo flamegraph`, `perf`, or external profiling tools
- **Database tools** — Use DataGrip, DBeaver, or TablePlus
- **HTTP Client** — Use tools like `curl`, `httpie`, or Postman
- **Coverage visualization** — Use `cargo tarpaulin` or `cargo llvm-cov` externally

## A Note on Licensing and Telemetry

If you're moving from RustRover partly due to licensing concerns or telemetry policies, you should know:

- **Zed is open source** (MIT licensed for the editor, AGPL for collaboration services)
- **Telemetry is optional** and can be disabled during onboarding or in settings
- **No license tiers**: All features are available to everyone

## Collaboration in Zed vs. RustRover

RustRover offers Code With Me as a separate feature for collaboration. Zed has collaboration built into the core experience.

- Open the Collab Panel in the left dock
- Create a channel and [invite your collaborators](https://zed.dev/docs/collaboration#inviting-a-collaborator) to join
- [Share your screen or your codebase](https://zed.dev/docs/collaboration#share-a-project) directly

Once connected, you'll see each other's cursors, selections, and edits in real time. Voice chat is included. There's no need for separate tools or third-party logins.

## Using AI in Zed

If you're used to AI assistants in RustRover (like JetBrains AI Assistant), Zed offers similar capabilities with more flexibility.

### Configuring GitHub Copilot

1. Open Settings with `Cmd+,` (macOS) or `Ctrl+,` (Linux/Windows)
2. Navigate to **AI → Edit Predictions**
3. Click **Configure** next to "Configure Providers"
4. Under **GitHub Copilot**, click **Sign in to GitHub**

Once signed in, just start typing. Zed will offer suggestions inline for you to accept.

### Additional AI Options

To use other AI models in Zed, you have several options:

- Use Zed's hosted models, with higher rate limits. Requires [authentication](https://zed.dev/docs/accounts.html) and subscription to [Zed Pro](https://zed.dev/docs/ai/subscription.html).
- Bring your own [API keys](https://zed.dev/docs/ai/llm-providers.html), no authentication needed
- Use [external agents like Claude Code](https://zed.dev/docs/ai/external-agents.html)

## Advanced Config and Productivity Tweaks

Zed exposes advanced settings for power users who want to fine-tune their environment.

Here are a few useful tweaks for Rust developers:

**Format on Save (uses rustfmt by default):**

```json
"format_on_save": "on"
```

**Configure inlay hints for Rust:**

```json
{
  "inlay_hints": {
    "enabled": true,
    "show_type_hints": true,
    "show_parameter_hints": true,
    "show_other_hints": true
  }
}
```

**Configure rust-analyzer settings:**

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "checkOnSave": {
          "command": "clippy"
        },
        "cargo": {
          "allFeatures": true
        },
        "procMacro": {
          "enable": true
        }
      }
    }
  }
}
```

**Use a separate target directory for rust-analyzer (faster builds):**

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "rust-analyzer.cargo.targetDir": true
      }
    }
  }
}
```

This tells rust-analyzer to use `target/rust-analyzer` instead of `target`, so IDE analysis doesn't conflict with your manual `cargo build` commands.

**Enable direnv support (useful for Rust projects using direnv):**

```json
"load_direnv": "shell_hook"
```

**Configure linked projects for workspaces:**

If you work with multiple Cargo projects that aren't in a workspace, you can tell rust-analyzer about them:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "linkedProjects": ["./project-a/Cargo.toml", "./project-b/Cargo.toml"]
      }
    }
  }
}
```

## Next Steps

Now that you're set up, here are some resources to help you get the most out of Zed:

- [Configuring Zed](../configuring-zed.md) — Customize settings, themes, and editor behavior
- [Key Bindings](../key-bindings.md) — Learn how to customize and extend your keymap
- [Tasks](../tasks.md) — Set up build and run commands for your projects
- [AI Features](../ai/overview.md) — Explore Zed's AI capabilities beyond code completion
- [Collaboration](../collaboration/overview.md) — Share your projects and code together in real time
- [Rust in Zed](../languages/rust.md) — Rust-specific setup and configuration
