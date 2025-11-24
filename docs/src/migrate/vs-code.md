# How to Migrate from VS Code to Zed

This guide is for developers who’ve spent serious time in VS Code and want to try Zed without starting from scratch.

If you’re here, you might be looking for a faster editor. Or something less cluttered. Or you’re curious about built-in collaboration. Whatever brought you here, this guide helps you move over your habits, shortcuts, and settings.

We’ll cover what to bring, what to change, and what’s different. You can ease in gradually or switch all at once. Either way, you’ll stay productive.

## Install Zed
Zed is available on macOS, Windows, and Linux.

For macOS, you can download it from zed.dev/download, or install via Homebrew:
`brew install zed-editor/zed/zed`

For most Linux users, the easiest way to install Zed is through our installation script:
`curl -f https://zed.dev/install.sh | sh`

After installation, you can launch Zed from your Applications folder (macOS) or directly from the terminal (Linux) using:
`zed .`
This opens the current directory in Zed.

## Import Settings from VS Code

During setup, you have the option to import key settings from VS Code. Zed imports the following settings:

Zed doesn’t import extensions or keybindings, but this is the fastest way to get a familiar feel while trying something new. If you skip that step during setup, you can still import settings manually later via the command palette:

`Cmd+Shift+P → Zed: Import VS Code Settings`

## Set Up Your Editor Preferences

You can also configure settings manually in the Settings Editor.

To edit your settings:
1. `Cmd+,` to open the Settings Editor.
2. Run `zed: open settings` in the Command Palette.

Here’s how common VS Code settings translate:
| VS Code | Zed | Notes |
| --- | --- | --- |
| editor.fontFamily | buffer_font_family | Zed uses Zed Mono by default |
| editor.fontSize | buffer_font_size | Set in pixels |
| editor.tabSize | tab_size | Can override per language |
| editor.insertSpaces | insert_spaces | Boolean |
| editor.formatOnSave | format_on_save | Works with formatter enabled |
| editor.wordWrap | soft_wrap | Supports optional wrap column |


Zed also supports per-project settings. You can find these in the Settings Editor as well.

## Open or Create a Project

After setup, press `Cmd+O` (`Ctrl+O` on Linux) to open a folder. This becomes your workspace in Zed. There's no support for multi-root workspaces or `.code-workspace` files like in VS Code. Zed keeps it simple: one folder, one workspace.

To start a new project, create a directory using your terminal or file manager, then open it in Zed. The editor will treat that folder as the root of your project.

You can also launch Zed from the terminal inside any folder with:
`zed .`

Once inside a project, use `Cmd+P` to jump between files quickly. `Cmd+Shift+P` (`Ctrl+Shift+P` on Linux) opens the command palette for running actions / tasks, toggling settings, or starting a collaboration session.

Open buffers appear as tabs across the top. The sidebar shows your file tree and Git status. Collapse it with `Cmd+B` for a distraction-free view.

## Differences in Keybindings

If you chose the VS Code keymap during onboarding, you're likely good to go, and most of your shortcuts should already feel familiar.
Here’s a quick reference guide for how our keybindings compare to what you’re used to coming from VS Code.

### Common Shared Keybindings (Zed <> VS Code)
| Action | Shortcut |
| --- | --- |
| Find files | `Cmd + P` |
| Run a command | `Cmd + Shift + P` |
| Search text (project-wide) | `Cmd + Shift + F` |
| Find symbols (project-wide) | `Cmd + T` |
| Find symbols (file-wide) | `Cmd + Shift + O` |
| Toggle left dock | `Cmd + B` |
| Toggle bottom dock | `Cmd + J` |
| Open terminal | `Ctrl + ~` |
| Open file tree explorer | `Cmd + Shift + E` |
| Close current buffer | `Cmd + W` |
| Close whole project | `Cmd + Shift + W` |
| Refactor: rename symbol | `F2` |
| Change theme | `Cmd + K, Cmd + T` |
| Wrap text | `Opt + Z` |
| Navigate open tabs | `Cmd + Opt + Arrow` |
| Syntactic fold / unfold | `Cmd + Opt + {` or `}` |


### Different Keybindings (Zed <> VS Code)
| Action | VS Code | Zed |
| --- | --- | --- |
| Open recent project | `Ctrl + R` | `Cmd + Opt + O` |
| Move lines up/down | `Opt + Up/Down` | `Cmd + Ctrl + Up/Down` |
| Split panes | `Cmd + \` | `Cmd + K, Arrow Keys` |
| Expand Selection | `Shift + Alt + Right` | `Opt + Up` |

### Unique to Zed
| Action | Shortcut | Notes |
| --- | --- | --- |
| Toggle right dock | `Cmd + R` or `Cmd + Alt + B` |  |
| Syntactic selection| `Opt + Up/Down` | Selects code by structure (e.g., inside braces). |

### Customize Keybindings

To edit your keybindings:
- Open the command palette (`Cmd+Shift+P`)
- Run `Zed: Open Keymap Editor`

This opens a list of all available bindings. You can override individual shortcuts, remove conflicts, or build a layout that works better for your setup.

Zed also supports chords (multi-key sequences) like `Cmd+K Cmd+C`, like VS Code does.

## Differences in User Interfaces

### No Workspace
VS Code uses a dedicated Workspace concept, with multi-root folders, `.code-workspace` files, and a clear distinction between “a window” and “a workspace.”
Zed simplifies this model.

In Zed:
- There is no workspace file format. Opening a folder is your project context.

- Zed does not support multi-root workspaces. You can only open one folder at a time in a window.

- Most project-level behavior is scoped to the folder you open. Search, Git integration, tasks, and environment detection all treat the opened directory as the project root.

- Per-project settings are optional. You can add a `.zed/settings.json` file inside a project to override global settings, but Zed does not use `.code-workspace` files and won’t import them.

- You can start from a single file or an empty window. Zed doesn’t require you to open a folder to begin editing.

The result is a simpler model:
Open a folder → work inside that folder → no additional workspace layer.


### Navigating in a Project

In VS Code, the standard entry point is opening a folder. From there, the left-hand sidebar is central to your navigation.
Zed takes a different approach:

- You can still open folders, but you don’t need to. Opening a single file or even starting with an empty workspace is valid.
- The Command Palette (`Cmd+Shift+P`) and File Finder (`Cmd+P`) are your primary navigation tools. The File Finder searches across the entire workspace instantly; files, symbols, commands, even teammates if you're collaborating.
- Instead of a persistent sidebar, Zed encourages you to:
  - Fuzzy-find files by name (`Cmd+P`)
  - Jump directly to symbols (`Cmd+Shift+O`)
  - Use split panes and tabs for context, rather than keeping a large file tree open (though you can do this with the Project Panel if you prefer).

The UI is intentionally minimal. Panels slide in only when needed, then get out of your way. The focus is on flowing between code instead of managing panes.

### Extensions vs. Marketplace
Zed does not offer as many extensions as VS Code. The available extensions are focused on language support, themes, syntax highlighting, and other core editing enhancements.

However there are several features that typically require extensions in VS Code which we built directly into Zed:
- Real-time collaboration with voice and cursor sharing (no Live Share required)
- AI coding assistance (no Copilot extension needed)
- Built-in terminal panel
- Project-wide fuzzy search
- Task runner with JSON config
- Inline diagnostics and code actions via LSP

You won’t find one-to-one replacements for every VS Code extension, especially if you rely on tools for DevOps, containers, or test runners. Zed's extension ecosystem is still growing, and the catalog is smaller by design.

### Collaboration in Zed vs. VS Code

Unlike VS Code, Zed doesn’t require an extension to collaborate. It’s built into the core experience.

- Open the Collab Panel in the left dock.
- Create a channel and [invite your collaborators](https://zed.dev/docs/collaboration#inviting-a-collaborator) to join.
- [Share your screen or your codebase](https://zed.dev/docs/collaboration#share-a-project) directly.

Once connected, you’ll see each other's cursors, selections, and edits in real time. Voice chat is included, so you can talk as you work. There’s no need for separate tools or third-party logins. Zed’s collaboration is designed for everything from quick pair programming to longer team sessions.

Learn how [Zed uses Zed](https://zed.dev/blog/zed-is-our-office) to plan work and collaborate.

### Using AI in Zed
If you’re used to GitHub Copilot in VS Code, you can do the same in Zed. You can also explore other agents through Zed Pro, or bring your own keys and connect without authentication. Zed is designed to enable many options for using AI, including disabling it entirely.

#### Configuring GitHub Copilot
You should be able to sign-in to GitHub Copilot by clicking on the Zeta icon in the status bar and following the setup instructions.
You can also add this to your settings:
```json
{
  "features": {
    "edit_prediction_provider": "copilot"
  }
}
```

To invoke completions, just start typing. Zed will offer suggestions inline for you to accept.

#### Additional AI Options
To use other AI models in Zed, you have several options:

- Use Zed’s hosted models, with higher rate limits. Requires [authentication](https://zed.dev/docs/accounts.html) and subscription to [Zed Pro](https://zed.dev/docs/ai/subscription.html).
- Bring your own [API keys](https://zed.dev/docs/ai/llm-providers.html), no authentication needed
- Use [external agents like Claude Code](https://zed.dev/docs/ai/external-agents.html).

### Advanced Config and Productivity Tweaks
Zed exposes advanced settings for power users who want to fine-tune their environment.

Here are a few useful tweaks:

**Format on Save:**
```json
"format_on_save": "on"
```

**Enable direnv support:**
```json
"load_direnv": "shell_hook"
```

**Custom Tasks**: Define build or run commands in your `tasks.json` (accessed via command palette: `zed: open tasks`):

```json
[
  {
    "label": "build",
    "command": "cargo build"
  }
]
```

**Bring over custom snippets**
Copy your VS Code snippet JSON directly into Zed's snippets folder (`zed: configure snippets`).
