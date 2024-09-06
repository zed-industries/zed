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

```json
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

For example, to override the background color of the editor and the font style of comments, add the following to your `settings.json` file:

```json
{
  "experimental.theme_overrides": {
    "editor.background": "#333",
    "syntax": {
      "comment": {
        "font_style": "italic"
      }
    }
  }
}
```

See which attributes are available to override by looking at the JSON format of your theme. For example, [here is the JSON format for the `One` themes](https://github.com/zed-industries/zed/blob/main/assets/themes/one/one.json).

## Local Themes

Store new themes locally by placing them in the `~/.config/zed/themes` directory.

For example, to create a new theme called `my-cool-theme`, create a file called `my-cool-theme.json` in that directory. It will be available in the theme selector the next time Zed loads.

Find more themes at [zed-themes.com](https://zed-themes.com).

## Theme Development

See: [Developing Zed Themes](./extensions/themes.md)
