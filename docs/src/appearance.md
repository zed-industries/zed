# Appearance

Zed's visual appearance is highly configurable. You can change themes, fonts, icon styles, and dozens of UI elements to match your preferences.

This page covers the essentials to get you started quickly, outlines how settings work, and points you to detailed documentation for specific customizations.

## Customize Zed in 5 Minutes

Here's how to make Zed feel like home:

1. **Pick a theme**: Open the command palette with {#kb theme_selector::Toggle} and type "theme" to open the Theme Selector. Arrow through the list to preview themes in real time, and press Enter to apply.

2. **Choose an icon theme**: Run `icon theme selector: toggle` from the command palette to browse icon themes.

3. **Set your font**: Open the Settings Editor with {#kb zed::OpenSettings} and search for `buffer_font_family`. Set it to your preferred coding font.

4. **Adjust font size**: In the same Settings Editor, search for `buffer_font_size` and `ui_font_size` to tweak the editor and interface text sizes.

That's it. You now have a personalized Zed setup.

## How Settings Work

The **Settings Editor** ({#kb zed::OpenSettings}) is the primary way to configure Zed. It provides a searchable interface where you can browse available settings, see their current values, and make changes. As you type in the search box, matching settings appear with descriptions and controls to modify them.

Changes you make in the Settings Editor are saved automatically to your settings file.

### User Settings vs Project Settings

Zed supports two levels of configuration:

- **User settings** apply globally across all projects. These are your defaults.

- **Project settings** override user settings for a specific project. Create a `.zed/settings.json` file in your project root to customize behavior per-codebase (for example, different tab sizes or formatters for different projects).

### Editing Settings as JSON

If you prefer working with JSON directly, open your settings file with {#kb zed::OpenSettingsFile}. This file is located at:

- macOS/Linux: `~/.config/zed/settings.json`
- Windows: `%APPDATA%\Zed\settings.json`

The Settings Editor and JSON file represent the same configuration—changes in one are reflected in the other.

> **Tip:** Some advanced settings aren't yet available in the Settings Editor. For full control, edit the JSON file directly.

### Example: Changing Your Theme

Using the Settings Editor:
1. Press {#kb zed::OpenSettings}
2. Search for "theme"
3. Select your preferred theme from the dropdown

Or add this to your `settings.json`:

```json [settings]
{
  "theme": {
    "mode": "system",
    "light": "One Light",
    "dark": "One Dark"
  }
}
```

## Using Settings Deep Links

Zed supports deep links that open specific settings directly. These are useful for:

- Sharing configuration tips with teammates
- Quick access from documentation
- Automation and scripting

Deep links follow the format `zed://settings/setting_name`. For example, `zed://settings/theme` opens the theme settings.

## Detailed Customization

### Themes

Install themes from the Extensions page ({#action zed::Extensions}), then switch between them with the Theme Selector ({#kb theme_selector::Toggle}).

Zed supports separate themes for light and dark mode, automatic switching based on your system preference, and per-theme overrides for fine-grained control.

→ [Themes documentation](./themes.md)

### Icon Themes

Customize file and folder icons in the project panel and tabs. Browse available icon themes with the Icon Theme Selector.

→ [Icon Themes documentation](./icon-themes.md)

### Fonts & Visual Tweaks

Configure fonts for the editor buffer, UI, and terminal independently. Adjust line height, enable or disable ligatures, and tweak dozens of visual elements like the status bar, tab bar, scrollbar, and panels.

→ [Visual Customization documentation](./visual-customization.md)

## What's Next

- [Key bindings](./key-bindings.md) — Customize keyboard shortcuts
- [Vim Mode](./vim.md) — Enable modal editing
- [Snippets](./snippets.md) — Create custom code snippets
- [All Settings](./configuring-zed.md) — Complete settings reference
