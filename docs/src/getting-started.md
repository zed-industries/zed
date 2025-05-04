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

## Configure Zed

To open your custom settings to set things like fonts, formatting settings, per-language settings, and more, use the {#kb zed::OpenSettings} keybinding.

To see all available settings, open the Command Palette with {#kb command_palette::Toggle} and search for "zed: open default settings". You can also check them all out in the [Configuring Zed](./configuring-zed.md) documentation.

## Set up your key bindings

To open your custom keymap to add your key bindings, use the {#kb zed::OpenKeymap} keybinding.

To access the default key binding set, open the Command Palette with {#kb command_palette::Toggle} and search for "zed: open default keymap". See [Key Bindings](./key-bindings.md) for more info.
