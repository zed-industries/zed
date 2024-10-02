# Getting Started

Welcome to Zed! We are excited to have you. Here is a jumping-off point to getting started.

## Download Zed

### macOS

You can obtain the stable builds via the [download page](https://zed.dev/download). If you want to download our preview build, you can find it on its [releases page](https://zed.dev/releases/preview) After the first manual installation, Zed will periodically check for and install updates automatically for you.

You can also install Zed stable via Homebrew:

```sh
brew install --cask zed
```

As well as Zed preview:

```sh
brew install --cask zed@preview
```

### Linux

For most people, the easiest way to install Zed is through our installation script:

```sh
curl -f https://zed.dev/install.sh | sh
```

If you'd like to help us test our new features, you can also install our preview build:

```sh
curl -f https://zed.dev/install.sh | ZED_CHANNEL=preview sh
```

This script supports `x86_64` and `AArch64`, as well as common Linux distributions: Ubuntu, Arch, Debian, RedHat, CentOS, Fedora, and more.

If this script is insufficient for your use case or you run into problems running Zed, please see our [Linux-specific documentation](./linux.md).

## Command Palette

The Command Palette is the main way to access functionality in Zed, and its keybinding is the first one you should make yourself familiar with.

To open the Command Palette, use {#kb command_palette::Toggle}.

The Command Palette allows you to access pretty much any functionality that's available in Zed.

![The opened Command Palette](https://zed.dev/img/features/command-palette.jpg)

Try it! Open the Command Palette and type in `new file`. You should see the list of commands being filtered down to `workspace: new file`. Hit return and you end up with a new buffer!

Any time you see instructions that include commands of the form `zed: ...` or `editor: ...` and so on that means you need to execute them in the Command Palette.

## Configure Zed

Use {#kb zed::OpenSettings} to open your custom settings to set things like fonts, formatting settings, per-language settings, and more.

On macOS, you can access the default configuration using the `Zed > Settings > Open Default Settings` menu item. See [Configuring Zed](./configuring-zed.md) for all available settings.

On Linux, you can access the default configuration via the Command Palette. Open it with {#kb zed::OpenDefaultSettings} and type in `zed: open default settings` and then hit return.

## Set up your key bindings

On macOS, you can access the default key binding set using the `Zed > Settings > Open Default Key Bindings` menu item. Use <kbd>cmd-k cmd-s|ctrl-k ctrl-s</kbd> to open your custom keymap to add your key bindings. See Key Bindings for more info.

On Linux, you can access the default key bindings via the Command Palette. Open it with <kbd>ctrl-shift-p</kbd> and type in `zed: open default keymap` and then hit return.
