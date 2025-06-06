# Icon Themes

CodeOrbit comes with a built-in icon theme, with more icon themes available as extensions.

## Selecting an Icon Theme

See what icon themes are installed and preview them via the Icon Theme Selector, which you can open from the command palette with "icon theme selector: toggle".

Navigating through the icon theme list by moving up and down will change the icon theme in real time and hitting enter will save it to your settings file.

## Installing more Icon Themes

More icon themes are available from the Extensions page, which you can access via the command palette with "CodeOrbit: Extensions" or the [CodeOrbit website](https://CodeOrbit.dev/extensions).

## Configuring Icon Themes

Your selected icon theme is stored in your settings file. You can open your settings file from the command palette with "CodeOrbit: open settings" (bound to `cmd-,` on macOS and `ctrl-,` on Linux).

Just like with themes, CodeOrbit allows for configuring different icon themes for light and dark mode. You can set the mode to `"light"` or `"dark"` to ignore the current system mode.

```json
{
  "icon_theme": {
    "mode": "system",
    "light": "Light Icon Theme",
    "dark": "Dark Icon Theme"
  }
}
```

## Icon Theme Development

See: [Developing CodeOrbit Icon Themes](./extensions/icon-themes.md)
