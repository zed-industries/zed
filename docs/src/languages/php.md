# PHP

PHP support is available through the [PHP extension](https://github.com/zed-extensions/php).

- Tree-sitter: [tree-sitter/tree-sitter-php](https://github.com/tree-sitter/tree-sitter-php)
- Language Server: [phpactor/phpactor](https://github.com/phpactor/phpactor)
- Alternate Language Server: [bmewburn/vscode-intelephense](https://github.com/bmewburn/vscode-intelephense/)

## Install PHP

The PHP extension requires PHP to be installed and available in your `PATH`:

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
## macOS and Linux
which php

## Windows
where php
```

## Choosing a language server

The PHP extension uses [LSP language servers](https://microsoft.github.io/language-server-protocol) with Phpactor as the default. If you want to use other language servers that support Zed (e.g. Intelephense or PHP Tools), make sure to follow the documentation on how to implement it.

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

To use the premium features, you can place your license file inside your home directory at `~/intelephense/licence.txt` for macOS and Linux, or `%USERPROFILE%/intelephense/licence.txt` on Windows.

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

### PHP Tools

[PHP Tools](https://www.devsense.com/) is a proprietary language server that offers free and premium features. You need to [purchase a license](https://www.devsense.com/en/purchase) to activate the premium features.

To use PHP Tools, add the following to your `settings.json`:

```json [settings]
{
  "languages": {
    "PHP": {
      "language_servers": ["phptools", "!intelephense", "!phpactor", "..."]
    }
  }
}
```

To use the premium features, you can add your license in `initialization_options` in your `settings.json`:

```json [settings]
{
  "lsp": {
    "phptools": {
      "initialization_options": {
        "0": "your_license_key"
      }
    }
  }
}
```

or, set environment variable `DEVSENSE_PHP_LS_LICENSE` on `.env` file in your project.

```env
DEVSENSE_PHP_LS_LICENSE="your_license_key"
```

Check out the documentation of [PHP Tools for Zed](https://docs.devsense.com/other/zed/) for more details.

### Phpactor

To use Phpactor instead of Intelephense or any other tools, add the following to your `settings.json`:

```json [settings]
{
  "languages": {
    "PHP": {
      "language_servers": ["phpactor", "!intelephense", "!phptools", "..."]
    }
  }
}
```

## PHPDoc

Zed supports syntax highlighting for PHPDoc comments.

- Tree-sitter: [claytonrcarter/tree-sitter-phpdoc](https://github.com/claytonrcarter/tree-sitter-phpdoc)

## Debugging

The PHP extension provides a debug adapter for PHP via Xdebug. There are several ways to use it:

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

These are common troubleshooting tips, in case you run into issues:

- Ensure that you have Xdebug installed for the version of PHP you’re running.
- Ensure that Xdebug is configured to run in `debug` mode.
- Ensure that Xdebug is actually starting a debugging session.
- Ensure that the host and port matches between Xdebug and Zed.
- Look at the diagnostics log by using the `xdebug_info()` function in the page you’re trying to debug.
