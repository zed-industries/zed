# PHP

PHP support is available through the [PHP extension](https://github.com/zed-extensions/php).

- Tree-sitter: [tree-sitter/tree-sitter-php](https://github.com/tree-sitter/tree-sitter-php)
- Language Server: [phpactor/phpactor](https://github.com/phpactor/phpactor)
- Alternate Language Server: [bmewburn/vscode-intelephense](https://github.com/bmewburn/vscode-intelephense/)

## Install PHP

The PHP extension requires PHP to be installed and available in your path:

```sh
# macOS via Homebrew
brew install php

# Debian/Ubuntu
sudo apt-get install php-cli

# CentOS 8+/RHEL
sudo dnf install php-cli

# Arch Linux
sudo pacman -S php

# check PHP path
which php
```

## Choosing a language server

The PHP extension offers Phpactor and Intelephense as language server with Phpactor as default.

### Phpactor

To use Phpactor instead of Intelephense, add the following to your `settings.json`:

```json [settings]
{
  "languages": {
    "PHP": {
      "language_servers": ["phpactor", "!intelephense", "..."]
    }
  }
}
```

### Intelephense

[Intelephense](https://intelephense.com/) is a [proprietary](https://github.com/bmewburn/vscode-intelephense/blob/master/LICENSE.txt#L29) language server for PHP operating under a freemium model. Certain features require purchase of a [premium license](https://intelephense.com/buy).

To use Intelephense, add the following to your `settings.json`:

```json [settings]
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "!phptools", "..."]
    }
  }
}
```

To use the premium features, you can place your license file inside your home directory at `~/intelephense/licence.txt`.

Alternatively, you can pass the licence key or a path to a file containing the licence key as an initialization option. To do this, add the following to your `settings.json`:

```json [settings]
{
  "lsp": {
    "intelephense": {
      "initialization_options": {
        "licenceKey": "/path/to/licence.txt"
      }
    }
  }
}
```

## PHPDoc

Zed supports syntax highlighting for PHPDoc comments.

- Tree-sitter: [claytonrcarter/tree-sitter-phpdoc](https://github.com/claytonrcarter/tree-sitter-phpdoc)

## Debugging

The PHP extension provides a debug adapter for PHP and Xdebug. Here a couple ways you can use it:

```json
[
  {
    "label": "PHP: Listen to Xdebug",
    "adapter": "Xdebug",
    "request": "launch",
    "port": 9003
  },
  {
    "label": "PHP: Debug this test",
    "adapter": "Xdebug",
    "request": "launch",
    "program": "vendor/bin/phpunit",
    "args": ["--filter", "$ZED_SYMBOL"]
  }
]
```

In case you run into issues, here are some ways to solve them:

- Ensure that you have Xdebug installed for the version of PHP you’re running.
- Ensure that Xdebug is configured to run in `debug` mode.
- Ensure that Xdebug is actually starting a debugging session.
- Ensure that the host and port matches between Xdebug and Zed.
- Look at the diagnostics log by using the `xdebug_info()` function in the page you’re trying to debug.
