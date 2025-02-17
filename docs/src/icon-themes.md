# Icon Themes

Zed comes by default with a light and dark icon theme, with more icon themes available as extensions.

## Selecting an Icon Theme

See what icon themes are installed and preview them via the Icon Theme Selector, which you can open from the command palette with "icon theme selector: Toggle".

Navigating through the icon theme list by moving up and down will change the icon theme in real time and hitting enter will save it to your settings file.

## Installing more Icon Themes

More icon themes are available from the Extensions page, which you can access via the command palette with "zed: Extensions" or the [Zed website](https://zed.dev/extensions).

## Configuring Icon Themes

Your selected icon theme is stored in your settings file. You can open your settings file from the command palette with "zed: Open Settings" (bound to `cmd-,` on macOS and `ctrl-,` on Linux).

By default, Zed maintains two icon themes: one for light mode and one for dark mode. You can set the mode to `"light"` or `"dark"` to ignore the current system mode.

```json
{
  "icon_theme": {
    "mode": "system",
    "light": "Zed (Default)",
    "dark": "Zed (Default)"
  }
}
```

## Icon Theme Development

See: [Developing Zed Icon Themes](./extensions/icon-themes.md)
