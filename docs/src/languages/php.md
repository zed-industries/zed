# PHP

PHP support is available through the [PHP extension](https://github.com/zed-industries/zed/tree/main/extensions/php).

- Tree Sitter: https://github.com/tree-sitter/tree-sitter-php
- Language Servers:
  - [phpactor](https://github.com/phpactor/phpactor)
  - [intelephense](https://github.com/bmewburn/vscode-intelephense/)

## Choosing a language server

The PHP extension offers both `phpactor` and `intelephense` language server support.

`phpactor` is enabled by default.

## Intelephense

[Intelephense](https://intelephense.com/) is a [proprietary](https://github.com/bmewburn/vscode-intelephense/blob/master/LICENSE.txt#L29) language server for PHP operating under a freemium model. Certain features require purchase of a [premium license](https://intelephense.com/). To use these features you must place your [license.txt file](https://intelephense.com/faq.html) at `~/intelephense/licence.txt` inside your home directory.

To switch to `intelephense`, add the following to your `settings.json`:

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "..."]
    }
  }
}
```

## PHPDoc

Zed supports syntax highlighting for PHPDoc comments.

- Tree Sitter: [claytonrcarter/tree-sitter-phpdoc](https://github.com/claytonrcarter/tree-sitter-phpdoc)
