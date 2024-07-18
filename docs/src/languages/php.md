# PHP

PHP support is available through the [PHP extension](https://github.com/zed-industries/zed/tree/main/extensions/php).

## Choosing a language server

The PHP extension offers both `phpactor` and `intelephense` language server support.

`phpactor` is enabled by default.

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
