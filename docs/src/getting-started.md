# Getting Started

Welcome to Zed! We are excited to have you. Here is a jumping-off point to getting started.

## Download Zed

### macOS

Get the latest stable builds via [the download page](https://zed.dev/download). If you want to download our preview build, you can find it on its [releases page](https://zed.dev/releases/preview). After the first manual installation, Zed will periodically check for install updates.

You can also install Zed stable via Homebrew:

```sh
brew install --cask zed
```

As well as Zed preview:

```sh
brew install --cask zed@preview
```

### Linux

For most Linux users, the easiest way to install Zed is through our installation script:

```sh
curl -f https://zed.dev/install.sh | sh
```

If you'd like to help us test our new features, you can also install our preview build:

```sh
curl -f https://zed.dev/install.sh | ZED_CHANNEL=preview sh
```

This script supports `x86_64` and `AArch64`, as well as common Linux distributions: Ubuntu, Arch, Debian, RedHat, CentOS, Fedora, and more.

If Zed is installed using this installation script, it can be uninstalled at any time by running the shell command `zed --uninstall`. The shell will then prompt you whether you'd like to keep your preferences or delete them. After making a choice, you should see a message that Zed was successfully uninstalled.

If this script is insufficient for your use case, you run into problems running Zed, or there are errors in uninstalling Zed, please see our [Linux-specific documentation](./linux.md).

## Command Palette

The Command Palette is the main way to access pretty much any functionality that's available in Zed. Its keybinding is the first one you should make yourself familiar with. To open it, hit: {#kb command_palette::Toggle}.

![The opened Command Palette](https://zed.dev/img/features/command-palette.jpg)

Try it! Open the Command Palette and type in `new file`. You should see the list of commands being filtered down to `workspace: new file`. Hit return and you end up with a new buffer.

Any time you see instructions that include commands of the form `zed: ...` or `editor: ...` and so on that means you need to execute them in the Command Palette.

## CLI

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

## Configure Zed

To open your custom settings to set things like fonts, formatting settings, per-language settings, and more, use the {#kb zed::OpenSettings} keybinding.

To see all available settings, open the Command Palette with {#kb command_palette::Toggle} and search for `zed: open default settings`.
You can also check them all out in the [Configuring Zed](./configuring-zed.md) documentation.

## Configure AI in Zed

Zed smoothly integrates LLMs in multiple ways across the editor.
Visit [the AI overview page](./ai/overview.md) to learn how to quickly get started with LLMs on Zed.

## Set up your key bindings

To edit your custom keymap and add or remap bindings, you can either use {#kb zed::OpenKeymap} to spawn the Zed Keymap Editor ({#action zed::OpenKeymap}) or you can directly open your Zed Keymap json (`~/.config/zed/keymap.json`) with {#action zed::OpenKeymap}.

To access the default key binding set, open the Command Palette with {#kb command_palette::Toggle} and search for "zed: open default keymap". See [Key Bindings](./key-bindings.md) for more info.
