# How to Migrate from IntelliJ IDEA to Zed

This guide is for developers who have spent serious time in IntelliJ IDEA (or other JetBrains IDEs) and want to try Zed without losing your productivity.

If you're here, you might be looking for a faster, lighter editor. Or something that strips away the complexity. Or you're curious about built-in collaboration and AI features. Whatever brought you here, this guide helps you move over your habits, shortcuts, and mental models.

## Install Zed

Zed is available on macOS, Windows, and Linux.

For macOS, you can download it from zed.dev/download, or install via Homebrew:
`brew install zed-editor/zed/zed`

For Windows, download the installer from zed.dev/download, or install via winget:
`winget install Zed.Zed`

For most Linux users, the easiest way to install Zed is through our installation script:
`curl -f https://zed.dev/install.sh | sh`

After installation, you can launch Zed from your Applications folder (macOS), Start menu (Windows), or directly from the terminal using:
`zed .`
This opens the current directory in Zed.

## Set Up the JetBrains Keymap

If you're coming from IntelliJ, the fastest way to feel at home is to use the JetBrains keymap. During onboarding, you can select it as your base keymap. If you missed that step, you can change it anytime:

1. Open Settings with `Cmd+,` (macOS) or `Ctrl+,` (Linux/Windows)
2. Search for `base_keymap`
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

Here's how common IntelliJ settings translate to Zed:

| IntelliJ Setting | Zed Setting | Notes |
| --- | --- | --- |
| Editor → Font → Font | `buffer_font_family` | Zed uses Zed Mono by default |
| Editor → Font → Size | `buffer_font_size` | Set in pixels |
| Editor → Code Style → Tab size | `tab_size` | Can override per language |
| Editor → Code Style → Use tab character | `hard_tabs` | Boolean |
| Editor → General → Auto Import | `use_autoclose` | Zed handles this via LSP |
| Code Style → Reformat on save | `format_on_save` | Works with formatter enabled |
| Editor → General → Soft Wraps | `soft_wrap` | Supports optional wrap column |

Zed also supports per-project settings. Create a `.zed/settings.json` file in your project root to override global settings for that project, similar to how you might use `.idea` folders in IntelliJ.

## Open or Create a Project

After setup, press `Cmd+Shift+O` (with JetBrains keymap) to open a folder. This becomes your workspace in Zed. Unlike IntelliJ, there's no project configuration wizard, no `.iml` files, and no SDK setup required.

To start a new project, create a directory using your terminal or file manager, then open it in Zed. The editor will treat that folder as the root of your project.

You can also launch Zed from the terminal inside any folder with:
`zed .`

Once inside a project:
- Use `Cmd+Shift+O` or `Cmd+E` to jump between files quickly (like IntelliJ's "Recent Files")
- Use `Cmd+Shift+A` or `Shift Shift` to open the command palette (like IntelliJ's "Search Everywhere")
- Use `Cmd+O` to search for symbols (like IntelliJ's "Go to Class")

Open buffers appear as tabs across the top. The sidebar shows your file tree and Git status. Toggle it with `Cmd+1` (just like IntelliJ's Project tool window).

## Differences in Keybindings

If you chose the JetBrains keymap during onboarding, most of your shortcuts should already feel familiar. Here's a quick reference for how Zed compares to IntelliJ.

### Common Shared Keybindings (Zed with JetBrains keymap ↔ IntelliJ)

| Action | Shortcut |
| --- | --- |
| Search Everywhere | `Shift Shift` |
| Find Action / Command Palette | `Cmd + Shift + A` |
| Go to File | `Cmd + Shift + O` |
| Go to Symbol / Class | `Cmd + O` |
| Recent Files | `Cmd + E` |
| Go to Definition | `Cmd + B` |
| Find Usages | `Alt + F7` |
| Rename Symbol | `Shift + F6` |
| Reformat Code | `Cmd + Alt + L` |
| Toggle Project Panel | `Cmd + 1` |
| Toggle Terminal | `Alt + F12` |
| Duplicate Line | `Cmd + D` |
| Delete Line | `Cmd + Backspace` |
| Move Line Up/Down | `Shift + Alt + Up/Down` |
| Expand/Shrink Selection | `Alt + Up/Down` |
| Comment Line | `Cmd + /` |
| Go Back / Forward | `Cmd + [` / `Cmd + ]` |
| Toggle Breakpoint | `Ctrl + F8` |

### Different Keybindings (IntelliJ → Zed)

| Action | IntelliJ | Zed (JetBrains keymap) |
| --- | --- | --- |
| File Structure | `Cmd + F12` | `Cmd + F12` (outline) |
| Navigate to Next Error | `F2` | `F2` |
| Run | `Ctrl + R` | `Ctrl + Alt + R` (tasks) |
| Debug | `Ctrl + D` | `Alt + Shift + F9` |
| Stop | `Cmd + F2` | `Ctrl + F2` |

### Unique to Zed

| Action | Shortcut | Notes |
| --- | --- | --- |
| Toggle Right Dock | `Cmd + R` | Assistant panel, notifications |
| Syntactic Selection | `Alt + Up/Down` | Selects code by structure (matches IntelliJ's Extend Selection) |
| Split Panes | `Cmd + K`, then arrow keys | Create splits in any direction |

### How to Customize Keybindings

To edit your keybindings:
- Open the command palette (`Cmd+Shift+A` or `Shift Shift`)
- Run `Zed: Open Keymap Editor`

This opens a list of all available bindings. You can override individual shortcuts or remove conflicts.

Zed also supports key sequences (multi-key shortcuts).

## Differences in User Experience

### No Indexing

If you've used IntelliJ on large projects, you know the wait: "Indexing..." can take anywhere from 30 seconds to 15 minutes depending on project size. IntelliJ builds a comprehensive index of your entire codebase to power its code intelligence, and it re-indexes when dependencies change or after builds.

Zed doesn't index. You open a folder and start working immediately. File search and navigation work instantly regardless of project size.

The trade-off is real: IntelliJ's index powers features like finding all usages across your entire codebase, understanding class hierarchies, and detecting dead code. Zed delegates this work to language servers, which may not analyze as deeply or as broadly.

**How to adapt:**
- For project-wide symbol search, use `Cmd+O` / Go to Symbol (relies on your language server)
- For finding files by name, use `Cmd+Shift+O` / Go to File
- For text search across files, use `Cmd+Shift+F`—this is fast even on large codebases
- If you need deep static analysis for JVM code, consider running IntelliJ's inspections as a separate step or using standalone tools like Checkstyle, PMD, or SpotBugs

### LSP vs. Native Language Intelligence

IntelliJ has its own language analysis engine built from scratch for each supported language. For Java, Kotlin, and other JVM languages, this engine understands your code deeply: it resolves types, tracks data flow, knows about framework annotations, and offers dozens of specialized refactorings.

Zed uses the Language Server Protocol (LSP). Your code intelligence comes from external language servers—`jdtls` for Java, the Kotlin language server for Kotlin, `rust-analyzer` for Rust, `typescript-language-server` for TypeScript, and so on.

For some languages, the LSP experience is excellent. TypeScript, Rust, and Go have mature language servers that provide fast, accurate completions, diagnostics, and refactorings. For JVM languages, the gap is more noticeable. The Eclipse-based Java language server is capable, but it won't match IntelliJ's depth for things like:

- Spring and Jakarta EE annotation processing
- Complex refactorings (extract interface, pull members up, change signature with all callers)
- Framework-aware inspections
- Automatic import optimization with custom ordering rules

**How to adapt:**
- Accept that some refactorings will require manual work or terminal tools
- Use `Alt+Enter` for available code actions—the list will vary by language server
- For Java, ensure `jdtls` is properly configured with your JDK path in settings
- Consider using Zed for languages where LSP is strong (TypeScript, Rust, Go, Python) and keeping IntelliJ available for deep JVM work

### No Project Model

IntelliJ manages projects through `.idea` folders containing XML configuration files, `.iml` module definitions, SDK assignments, and run configurations. This model enables IntelliJ to understand multi-module projects, manage dependencies automatically, and persist complex run/debug setups.

Zed has no project model. A project is a folder. There's no wizard, no SDK selection screen, no module configuration.

This means:
- No automatic detection of Maven/Gradle projects—you run build commands yourself
- No managed run configurations—you define tasks manually or use the terminal
- No SDK management—your language server uses whatever JDK/SDK is on your PATH or configured in its settings
- No module boundaries—Zed sees your folder structure, nothing more

**How to adapt:**
- Create a `.zed/settings.json` in your project root for project-specific settings
- Define common commands in `tasks.json` (open via command palette: `zed: open tasks`):

```json
[
  {
    "label": "build",
    "command": "./gradlew build"
  },
  {
    "label": "run",
    "command": "./gradlew bootRun"
  },
  {
    "label": "test current file",
    "command": "./gradlew test --tests $ZED_STEM"
  }
]
```

- Use `Ctrl+Alt+R` to run tasks quickly
- Lean on your terminal (`Alt+F12`) for anything tasks don't cover
- For multi-module projects, you can open each module as a separate Zed window, or open the root and navigate via file finder

### No Framework Integration

IntelliJ's value for enterprise Java development comes largely from its framework integration. Spring beans are understood and navigable. JPA entities get special treatment. Endpoints are indexed and searchable. Jakarta EE annotations modify how the IDE analyzes your code.

Zed has none of this. There's no Spring plugin, no JPA support, no awareness of web frameworks. The language server sees Java code as Java code—it doesn't understand that `@Autowired` means something special or that this class is a REST controller.

Similarly for other ecosystems: no Rails integration, no Django awareness, no Angular/React-specific tooling beyond what the TypeScript language server provides.

**How to adapt:**
- Use grep and file search liberally. `Cmd+Shift+F` with a regex can find endpoint definitions, bean names, or annotation usages.
- Rely on your language server's "find references" (`Alt+F7`) for navigation—it works, just without framework context
- For Spring Boot, keep the Actuator endpoints or a separate tool for understanding bean wiring
- Consider using framework-specific CLI tools (Spring CLI, Rails generators) from Zed's terminal
- For database work, use an external client like DataGrip, DBeaver, or TablePlus—Zed has no database tooling

The honest assessment: if your daily work depends heavily on framework-aware navigation and refactoring, you'll feel the gap. Zed works best when you're comfortable navigating code through search rather than specialized tooling, or when your language has strong LSP support that covers most of what you need.

### Tool Windows vs. Docks

IntelliJ organizes auxiliary views into numbered tool windows (Project = 1, Git = 9, Terminal = Alt+F12, etc.). Zed uses a similar concept called "docks":

| IntelliJ Tool Window | Zed Equivalent | Shortcut (JetBrains keymap) |
| --- | --- | --- |
| Project (1) | Project Panel | `Cmd + 1` |
| Git (9 or Cmd+0) | Git Panel | `Cmd + 0` |
| Terminal (Alt+F12) | Terminal Panel | `Alt + F12` |
| Structure (7) | Outline Panel | `Cmd + 7` |
| Problems (6) | Diagnostics | `Cmd + 6` |
| Debug (5) | Debug Panel | `Cmd + 5` |

Zed has three dock positions: left, bottom, and right. Panels can be moved between docks by dragging or through settings.

### Debugging

Both IntelliJ and Zed offer integrated debugging, but the experience differs:

- Zed's debugger uses the Debug Adapter Protocol (DAP), supporting multiple languages
- Set breakpoints with `Ctrl+F8`
- Start debugging with `Alt+Shift+F9`
- Step through code with `F7` (step into), `F8` (step over), `Shift+F8` (step out)
- Continue execution with `F9`

The Debug Panel (`Cmd+5`) shows variables, call stack, and breakpoints—similar to IntelliJ's Debug tool window.

### Extensions vs. Plugins

IntelliJ has a massive plugin ecosystem covering everything from language support to database tools to deployment integrations.

Zed's extension ecosystem is smaller and more focused:
- Language support and syntax highlighting
- Themes
- Slash commands for AI
- Context servers

Several features that require plugins in other editors are built into Zed:
- Real-time collaboration with voice chat
- AI coding assistance
- Built-in terminal
- Task runner
- LSP-based code intelligence

You won't find one-to-one replacements for every IntelliJ plugin, especially for framework-specific tools, database clients, or application server integrations. For those workflows, you may need to use external tools alongside Zed.

## Collaboration in Zed vs. IntelliJ

IntelliJ offers Code With Me as a separate plugin for collaboration. Zed has collaboration built into the core experience.

- Open the Collab Panel in the left dock
- Create a channel and [invite your collaborators](https://zed.dev/docs/collaboration#inviting-a-collaborator) to join
- [Share your screen or your codebase](https://zed.dev/docs/collaboration#share-a-project) directly

Once connected, you'll see each other's cursors, selections, and edits in real time. Voice chat is included. There's no need for separate tools or third-party logins.

## Using AI in Zed

If you're used to AI assistants in IntelliJ (like GitHub Copilot or JetBrains AI), Zed offers similar capabilities with more flexibility.

### Configuring GitHub Copilot

Sign in to GitHub Copilot by clicking on the Zeta icon in the status bar and following the setup instructions. You can also add this to your settings:

```json
{
  "features": {
    "edit_prediction_provider": "copilot"
  }
}
```

To invoke completions, just start typing. Zed will offer suggestions inline for you to accept.

### Additional AI Options

To use other AI models in Zed, you have several options:

- Use Zed's hosted models, with higher rate limits. Requires [authentication](https://zed.dev/docs/accounts.html) and subscription to [Zed Pro](https://zed.dev/docs/ai/subscription.html).
- Bring your own [API keys](https://zed.dev/docs/ai/llm-providers.html), no authentication needed
- Use [external agents like Claude Code](https://zed.dev/docs/ai/external-agents.html)

## Advanced Config and Productivity Tweaks

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

**Configure language servers**: For Java development, you may want to configure the Java language server in your settings:

```json
{
  "lsp": {
    "jdtls": {
      "settings": {
        "java": {
          "home": "/path/to/jdk"
        }
      }
    }
  }
}
```

## What You'll Miss (and What You'll Gain)

### What IntelliJ Does Better

- **Deep JVM language support**: IntelliJ's Java, Kotlin, Scala, and Groovy support is unmatched
- **Framework integration**: Spring, Jakarta EE, Android Studio integration
- **Database tools**: Built-in database client and SQL support
- **Refactoring depth**: More refactoring options, especially for JVM languages
- **Run configurations**: Sophisticated run/debug configuration management
- **Built-in version control UI**: More feature-rich Git integration

### What Zed Does Better

- **Speed**: Zed is significantly faster to launch and navigate
- **Simplicity**: No project configuration, no SDK setup, just open a folder
- **Built-in collaboration**: Real-time multiplayer editing with voice chat
- **AI integration**: Native AI assistance without plugins
- **Resource usage**: Lower memory footprint
- **Modern UI**: Clean, distraction-free interface

Zed may not replace IntelliJ for heavy JVM development, but it excels as a fast, focused editor for polyglot development, quick edits, and collaborative work. Many developers use both: IntelliJ for deep project work and Zed for everything else.
