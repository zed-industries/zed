# Getting Started

Welcome to Zed! We are excited to have you. Here is a jumping-off point to getting started.

## Download Zed

### MacOS

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

## Configure Zed

Use `⌘` + `,` to open your custom settings to set things like fonts, formatting settings, per-language settings, and more. You can access the default configuration using the `Zed > Settings > Open Default Settings` menu item. See [Configuring Zed](./configuring-zed.md) for all available settings.

## Set up your key bindings

You can access the default key binding set using the `Zed > Settings > Open Default Key Bindings` menu item. Use `⌘` + `K`, `⌘` + `S` to open your custom keymap to add your key bindings. See Key Bindings for more info.
