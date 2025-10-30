# Snippets

Use the {#action snippets::ConfigureSnippets} action to create a new snippets file or edit a existing snippets file for a specified [scope](#scopes).

The snippets are located in `~/.config/zed/snippets` directory to which you can navigate to with the {#action snippets::OpenFolder} action.

## Example configuration

```json [settings]
{
  // Each snippet must have a name and body, but the prefix and description are optional.
  // The prefix is used to trigger the snippet, but when omitted then the name is used.
  // Use placeholders like $1, $2 or ${1:defaultValue} to define tab stops.
  // The $0 determines the final cursor position.
  // Placeholders with the same value are linked.
  "Log to console": {
    "prefix": "log",
    "body": ["console.info(\"Hello, ${1:World}!\")", "$0"],
    "description": "Logs to console"
  }
}
```

## Scopes

The scope is determined by the language name in lowercase e.g. `python.json` for Python, `shell script.json` for Shell Script, but there are some exceptions to this rule:

| Scope      | Filename        |
| ---------- | --------------- |
| Global     | snippets.json   |
| JSX        | javascript.json |
| Plain Text | plaintext.json  |

To create JSX snippets you have to use `javascript.json` snippets file, instead of `jsx.json`, but this does not apply to TSX and TypeScript which follow the above rule.

## Known Limitations

- Only the first prefix is used when an list of prefixes is passed in.
- Currently only the `json` snippet file format is supported, even though the `simple-completion-language-server` supports both `json` and `toml` file formats.

## See also

The `feature_paths` option in `simple-completion-language-server` is disabled by default.

If you want to enable it you can add the following to your `settings.json`:

```json [settings]
{
  "lsp": {
    "snippet-completion-server": {
      "settings": {
        "feature_paths": true
      }
    }
  }
}
```

For more configuration information, see the [`simple-completion-language-server` instructions](https://github.com/zed-industries/simple-completion-language-server/tree/main).
