# Getting Started

Welcome to Zed! We are excited to have you. Zed is a powerful multiplayer code editor designed to get out of your way and help you build what's next.

## Key Features

- [Smooth Editing](./configuring-zed.md): Zed is built in Rust to feel responsive and intuitive, with a minimalistic aesthetic and pixel-level editing controls.
- [Agentic Editing](./ai/overview.md): Use Zed's hosted models to collaborate with agents directly in an IDE. You can also plug into a third-party agent or bring your own keys into Zed.
- [Debugger](./debugger/overview.md): Debug your code in seconds, not hours, with minimal setup required.
- [Remote Development](./remote-development/overview.md): Offload the heavy lifting to the cloud, so you can focus on writing code.
- [Extensions](./extensions/overview.md): Leverage Zed's extensions to customize how you work with your code.

## Join the Zed Community

Zed is proudly open source, and we get better with every contribution. Join us on GitHub or in Discord to contribute code, report bugs, or suggest features.

- [Join Discord](https://discord.com/invite/zedindustries)
- [GitHub Discussions](https://github.com/zed-industries/zed/discussions)
- [Zed Reddit](https://www.reddit.com/r/ZedEditor)

<! -- Move command palette to configuration section -->

## Command Palette

The Command Palette is the main way to access pretty much any functionality that's available in Zed. Its keybinding is the first one you should make yourself familiar with. To open it, hit: {#kb command_palette::Toggle}.

![The opened Command Palette](https://zed.dev/img/features/command-palette.jpg)

Try it! Open the Command Palette and type in `new file`. You should see the list of commands being filtered down to `workspace: new file`. Hit return and you end up with a new buffer.

Any time you see instructions that include commands of the form `zed: ...` or `editor: ...` and so on that means you need to execute them in the Command Palette.

## Command-line Interface

Zed has a CLI, on Linux this should come with the distribution's Zed package (binary name can vary from distribution to distribution, `zed` will be used later for brevity).
For macOS, the CLI comes in the same package with the editor binary, and could be installed into the system with the `cli: install` Zed command which will create a symlink to the `/usr/local/bin/zed`.
It can also be built from source out of the `cli` crate in this repository.

Use `zed --help` to see the full list of capabilities.
General highlights:

- Opening another empty Zed window: `zed`

- Opening a file or directory in Zed: `zed /path/to/entry` (use `-n` to open in the new window)

- Reading from stdin: `ps axf | zed -`

- Starting Zed with logs in the terminal: `zed --foreground`

- Uninstalling Zed and all its related files: `zed --uninstall`