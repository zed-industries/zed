---
title: Shell Script
description: "Configure Shell Script language support in Zed, including language servers, formatting, and debugging."
---

# Shell Scripts

Shell Scripts (bash, zsh, dash, sh) are supported natively by Zed.

- Tree-sitter: [tree-sitter/tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash)

## Settings

Configure settings in Settings ({#kb zed::OpenSettings}) under Languages > Shell Script, or add to your settings file:

```json [settings]
  "languages": {
    "Shell Script": {
      "tab_size": 2,
      "hard_tabs": false
    }
  }
```

### Formatting

Zed supports auto-formatting Shell Scripts using external tools like [`shfmt`](https://github.com/mvdan/sh).

1. Install `shfmt`:

```sh
brew install shfmt            # macos (homebrew)
sudo apt-get install shfmt    # debian/ubuntu
dnf install shfmt             # fedora
yum install shfmt             # redhat
pacman -Sy shfmt              # archlinux
choco install shfmt           # windows (chocolatey)
```

2. Ensure `shfmt` is available in your path and check the version:

```sh
which shfmt
shfmt --version
```

3. Configure formatting in Settings ({#kb zed::OpenSettings}) under Languages > Shell Script, or add to your settings file:

```json [settings]
  "languages": {
    "Shell Script": {
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "shfmt",
          // Change `--indent 2` to match your preferred tab_size
          "arguments": ["--filename", "{buffer_path}", "--indent", "2"]
        }
      }
    }
  }
```

## See also:

- [Zed Docs: Language Support: Bash](./bash.md)
- [Zed Docs: Language Support: Fish](./fish.md)
