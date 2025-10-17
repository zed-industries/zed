# Themes

Zed comes with a number of built-in themes, with more themes available as extensions.

## Selecting a Theme

See what themes are installed and preview them via the Theme Selector, which you can open from the command palette with "theme selector: Toggle" (bound to `cmd-k cmd-t` on macOS and `ctrl-k ctrl-t` on Linux).

Navigating through the theme list by moving up and down will change the theme in real time and hitting enter will save it to your settings file.

## Installing more Themes

More themes are available from the Extensions page, which you can access via the command palette with "zed: Extensions" or the [Zed website](https://zed.dev/extensions).

Many popular themes have been ported to Zed, and if you're struggling to choose one, visit [zed-themes.com](https://zed-themes.com), a third-party gallery with visible previews for many of them.

## Configuring a Theme

Your selected theme is stored in your settings file. You can open your settings file from the command palette with "zed: Open Settings" (bound to `cmd-,` on macOS and `ctrl-,` on Linux).

By default, Zed maintains two themes: one for light mode and one for dark mode. You can set the mode to `"dark"` or `"light"` to ignore the current system mode.

```json [settings]
{
  "theme": {
    "mode": "system",
    "light": "One Light",
    "dark": "One Dark"
  }
}
```

## Theme Overrides

To override specific attributes of a theme, use the `experimental.theme_overrides` setting.

For example, add the following to your `settings.json` if you wish to override the background color of the editor and display comments and doc comments as italics:

```json [settings]
{
  "experimental.theme_overrides": {
    "editor.background": "#333",
    "syntax": {
      "comment": {
        "font_style": "italic"
      },
      "comment.doc": {
        "font_style": "italic"
      }
    }
  }
}
```

To see a comprehensive list of list of captures (like `comment` and `comment.doc`) see: [Language Extensions: Syntax highlighting](./extensions/languages.md#syntax-highlighting).

To see a list of available theme attributes look at the JSON file for your theme. For example, [assets/themes/one/one.json](https://github.com/zed-industries/zed/blob/main/assets/themes/one/one.json) for the default One Dark and One Light themes.

## Local Themes

Store new themes locally by placing them in the `~/.config/zed/themes` directory.

For example, to create a new theme called `my-cool-theme`, create a file called `my-cool-theme.json` in that directory. It will be available in the theme selector the next time Zed loads.

Find more themes at [zed-themes.com](https://zed-themes.com).

## Theme Development

See: [Developing Zed Themes](./extensions/themes.md)
