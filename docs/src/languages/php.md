# PHP

PHP support is available through the [PHP extension](https://github.com/zed-extensions/php).

- Tree-sitter: https://github.com/tree-sitter/tree-sitter-php
- Language Servers:
  - [phpactor](https://github.com/phpactor/phpactor)
  - [intelephense](https://github.com/bmewburn/vscode-intelephense/)

## Choosing a language server

The PHP extension offers both `phpactor` and `intelephense` language server support.

`phpactor` is enabled by default.

## Phpactor

The Zed PHP Extension can install `phpactor` automatically but requires `php` to be installed and available in your path:

```sh
# brew install php            # macOS
# sudo apt-get install php    # Debian/Ubuntu
# yum install php             # CentOS/RHEL
# pacman -S php               # Arch Linux
which php
```

## Intelephense

[Intelephense](https://intelephense.com/) is a [proprietary](https://github.com/bmewburn/vscode-intelephense/blob/master/LICENSE.txt#L29) language server for PHP operating under a freemium model. Certain features require purchase of a [premium license](https://intelephense.com/).

To switch to `intelephense`, add the following to your `settings.json`:

```json [settings]
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "..."]
    }
  }
}
```

To use the premium features, you can place your [licence.txt file](https://intelephense.com/faq.html) at `~/intelephense/licence.txt` inside your home directory. Alternatively, you can pass the licence key or a path to a file containing the licence key as an initialization option for the `intelephense` language server. To do this, add the following to your `settings.json`:

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
