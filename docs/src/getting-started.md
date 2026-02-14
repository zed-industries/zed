---
title: Getting Started with Zed
description: Get started with Zed, the fast open-source code editor. Essential commands, environment setup, and navigation basics.
---

# Getting Started {#getting-started}

Zed is a code editor written in Rust with GPU-accelerated rendering and built-in multiplayer support. This guide covers the essential commands, configuration basics, and navigation patterns.

## Open a Project {#open-project}

Open a folder from the command line:

```sh
zed ~/projects/my-app
```

Or use {#kb workspace::Open} to open a folder from within Zed.

## Essential Commands {#essential-commands}

| Action          | Keybinding                        |
| --------------- | --------------------------------- |
| Command palette | {#kb command_palette::Toggle}     |
| Go to file      | {#kb file_finder::Toggle}         |
| Go to symbol    | {#kb project_symbols::Toggle}     |
| Find in project | {#kb pane::DeploySearch}          |
| Toggle terminal | {#kb terminal_panel::ToggleFocus} |
| Open settings   | {#kb zed::OpenSettings}           |

The command palette ({#kb command_palette::Toggle}) provides access to every action in Zed. If you forget a shortcut, search for it there.

## Configure Your Editor {#configure}

Open the Settings Editor with {#kb zed::OpenSettings}. Search for any setting and change it directly.

Common first changes:

- **Theme**: {#kb theme_selector::Toggle} opens the theme selector
- **Font**: Search for `buffer_font_family` in Settings
- **Format on save**: Search for `format_on_save` and set to `on`

## Set Up Your Language {#languages}

Zed includes built-in support for many languages. For others, install the extension:

1. Open Extensions with {#kb zed::Extensions}
2. Search for your language
3. Click Install

See [Languages](./languages.md) for language-specific setup instructions.

## AI Features {#ai}

Zed includes built-in AI assistance. Open the Agent Panel with {#kb agent::ToggleFocus} to start a conversation, or use {#kb assistant::InlineAssist} for inline assistance.

See [AI Overview](./ai/overview.md) to configure providers and learn what's possible.

## Coming from Another Editor? {#migration}

Migration guides for common editors:

- [VS Code](./migrate/vs-code.md) — Import settings, map keybindings, find equivalent features
- [IntelliJ IDEA](./migrate/intellij.md) — Adapt to Zed's approach to navigation and refactoring
- [PyCharm](./migrate/pycharm.md) — Set up Python development in Zed
- [WebStorm](./migrate/webstorm.md) — Configure JavaScript/TypeScript workflows
- [RustRover](./migrate/rustrover.md) — Rust development in Zed

Familiar keybinding modes:

- **Vim**: Enable `vim_mode` in settings. See [Vim Mode](./vim.md).
- **Helix**: Enable `helix_mode` in settings. See [Helix Mode](./helix.md).

## Community {#community}

Zed is open source. Contribute code, report bugs, or suggest features:

- [Discord](https://discord.com/invite/zedindustries)
- [GitHub Discussions](https://github.com/zed-industries/zed/discussions)
- [Zed Reddit](https://www.reddit.com/r/ZedEditor)
