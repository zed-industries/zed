# Configuring Zed

This guide explains how Zed's settings system works, including the Settings Editor, JSON configuration files, and project-specific settings.

For visual customization (themes, fonts, icons), see [Appearance](./appearance.md).

## Settings Editor

The **Settings Editor** ({#kb zed::OpenSettings}) is the primary way to configure Zed. It provides a searchable interface where you can browse available settings, see their current values, and make changes.

To open it:

- Press {#kb zed::OpenSettings}
- Or run `zed: open settings` from the command palette

As you type in the search box, matching settings appear with descriptions and controls to modify them. Changes save automatically to your settings file.

> **Note:** Not all settings are available in the Settings Editor yet. Some advanced options, like language formatters, require editing the JSON file directly.

## Settings Files

### User Settings

Your user settings apply globally across all projects. Open the file with {#kb zed::OpenSettingsFile} or run `zed: open settings file` from the command palette.

The file is located at:

- macOS: `~/.config/zed/settings.json`
- Linux: `~/.config/zed/settings.json` (or `$XDG_CONFIG_HOME/zed/settings.json`)
- Windows: `%APPDATA%\Zed\settings.json`

The syntax is JSON with support for `//` comments.

### Default Settings

To see all available settings with their default values, run {#action zed::OpenDefaultSettings} from the command palette. This opens a read-only reference you can use when editing your own settings.

### Project Settings

Override user settings for a specific project by creating a `.zed/settings.json` file in your project root. Run {#action zed::OpenProjectSettings} to create this file.

Project settings take precedence over user settings for that project only.

```json [settings]
// .zed/settings.json
{
  "tab_size": 2,
  "formatter": "prettier",
  "format_on_save": "on"
}
```

You can also add settings files in subdirectories for more granular control.

**Limitation:** Not all settings can be set at the project level. Settings that affect the editor globally (like `theme` or `vim_mode`) only work in user settings. Project settings are limited to editor behavior and language tooling options like `tab_size`, `formatter`, and `format_on_save`.

## How Settings Merge

Settings are applied in layers:

1. **Default settings** — Zed's built-in defaults
2. **User settings** — Your global preferences
3. **Project settings** — Project-specific overrides

Later layers override earlier ones. For object settings (like `terminal`), properties merge rather than replace entirely.

## Per-Release Channel Overrides

Use different settings for Stable, Preview, or Nightly builds by adding top-level channel keys:

```json [settings]
{
  "theme": "One Dark",
  "vim_mode": false,
  "nightly": {
    "theme": "Rosé Pine",
    "vim_mode": true
  },
  "preview": {
    "theme": "Catppuccin Mocha"
  }
}
```

With this configuration:

- **Stable** uses One Dark with vim mode off
- **Preview** uses Catppuccin Mocha with vim mode off
- **Nightly** uses Rosé Pine with vim mode on

Changes made in the Settings Editor apply across all channels.

## Settings Deep Links

Zed supports deep links that open specific settings directly:

```
zed://settings/theme
zed://settings/vim_mode
zed://settings/buffer_font_size
```

These are useful for sharing configuration tips or linking from documentation.

## Example Configuration

```json [settings]
{
  "theme": {
    "mode": "system",
    "light": "One Light",
    "dark": "One Dark"
  },
  "buffer_font_family": "JetBrains Mono",
  "buffer_font_size": 14,
  "tab_size": 2,
  "format_on_save": "on",
  "autosave": "on_focus_change",
  "vim_mode": false,
  "terminal": {
    "font_family": "JetBrains Mono",
    "font_size": 14
  },
  "languages": {
    "Python": {
      "tab_size": 4
    }
  }
}
```

## What's Next

- [Appearance](./appearance.md) — Themes, fonts, and visual customization
- [Key bindings](./key-bindings.md) — Customize keyboard shortcuts
- [All Settings](./reference/all-settings.md) — Complete settings reference
