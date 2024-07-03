# Themes

Zed comes with a number of built-in themes, with more themes available as extensions.

## Selecting a Theme

You can see what these are installed and preview them from the Theme Selector. You can open the Theme Selector from the command palette with "theme selector: Toggle" (bound to `cmd-k cmd-t` on macOS and `ctrl-k ctrl-t` on Linux). Selecting a theme by moving up and down will change the theme in real time and hitting enter will save it to your settings file.

## Installing more Themes

More themes are available from the Extensions page. You can open the Extensions page from the command palette with "zed: Extensions". Many popular themes have been ported to Zed, and if you're struggling to choose one, there's a third-party gallery hosted by https://zed-themes.com with visible previews for many of them.

## Configuring a Theme

Your selected theme is stored in your settings file. You can open your settings file from the command palette with "zed: Open Settings" (bound to `cmd,` on macOS and `ctrl,` on Linux).

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
