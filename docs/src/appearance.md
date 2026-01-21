# Appearance

Customize Zed's visual appearance to match your preferences. This guide covers themes, fonts, icons, and other visual settings.

For information on how the settings system works, see [All Settings](./reference/all-settings.md).

## Customize Zed in 5 Minutes

Here's how to make Zed feel like home:

1. **Pick a theme**: Press {#kb theme_selector::Toggle} to open the Theme Selector. Arrow through the list to preview themes in real time, and press Enter to apply.

2. **Choose an icon theme**: Run `icon theme selector: toggle` from the command palette to browse icon themes.

3. **Set your font**: Open the Settings Editor with {#kb zed::OpenSettings} and search for `buffer_font_family`. Set it to your preferred coding font.

4. **Adjust font size**: In the same Settings Editor, search for `buffer_font_size` and `ui_font_size` to tweak the editor and interface text sizes.

That's it. You now have a personalized Zed setup.

## Themes

Install themes from the Extensions page ({#action zed::Extensions}), then switch between them with the Theme Selector ({#kb theme_selector::Toggle}).

Zed supports separate themes for light and dark mode with automatic switching based on your system preference:

```json [settings]
{
  "theme": {
    "mode": "system",
    "light": "One Light",
    "dark": "One Dark"
  }
}
```

You can also override specific theme attributes for fine-grained control.

→ [Themes documentation](./themes.md)

## Icon Themes

Customize file and folder icons in the Project Panel and tabs. Browse available icon themes with the Icon Theme Selector (`icon theme selector: toggle` in the command palette).

Like color themes, icon themes support separate light and dark variants:

```json [settings]
{
  "icon_theme": {
    "mode": "system",
    "light": "Zed (Default)",
    "dark": "Zed (Default)"
  }
}
```

→ [Icon Themes documentation](./icon-themes.md)

## Fonts

Zed uses three font settings for different contexts:

| Setting                | Used for                  |
| ---------------------- | ------------------------- |
| `buffer_font_family`   | Editor text               |
| `ui_font_family`       | Interface elements        |
| `terminal.font_family` | [Terminal](./terminal.md) |

Example configuration:

```json [settings]
{
  "buffer_font_family": "JetBrains Mono",
  "buffer_font_size": 14,
  "ui_font_family": "Inter",
  "ui_font_size": 16,
  "terminal": {
    "font_family": "JetBrains Mono",
    "font_size": 14
  }
}
```

### Font Ligatures

To disable font ligatures:

```json [settings]
{
  "buffer_font_features": {
    "calt": false
  }
}
```

### Line Height

Adjust line spacing with `buffer_line_height`:

- `"comfortable"` — 1.618 ratio (default)
- `"standard"` — 1.3 ratio
- `{ "custom": 1.5 }` — Custom ratio

## UI Elements

Zed provides extensive control over UI elements including:

- **Tab bar** — Show/hide, navigation buttons, file icons, git status
- **Status bar** — Language selector, cursor position, line endings
- **Scrollbar** — Visibility, git diff indicators, search results
- **Minimap** — Code overview display
- **Gutter** — Line numbers, fold indicators, breakpoints
- **Panels** — Project Panel, Terminal, Agent Panel sizing and docking

→ [Visual Customization documentation](./visual-customization.md) for all UI element settings

## What's Next

- [All Settings](./reference/all-settings.md) — Complete settings reference
- [Key bindings](./key-bindings.md) — Customize keyboard shortcuts
- [Vim Mode](./vim.md) — Enable modal editing
