# Configuring Zed

Zed is designed to be configured: we want to fit your workflow and preferences exactly. We provide default settings that are designed to be a comfortable starting point for as many people as possible, but we hope you will enjoy tweaking it to make it feel incredible.

In addition to the settings described here, you may also want to change your [theme](./themes.md), configure your [key bindings](./key-bindings.md), set up [tasks](./tasks.md) or install [extensions](https://github.com/zed-industries/extensions).

## Settings Editor

You can browse through many of the supported settings via the Settings Editor, which can be opened with the {#kb zed::OpenSettings} keybinding, or through the `zed: open settings` action in the command palette. Through it, you can customize your local, user settings as well as project settings.

> Note that not all settings that Zed supports are available through the Settings Editor yet.
> Some more intricate ones, such as language formatters, can only be changed through the JSON settings file {#kb zed::OpenSettingsFile}.

## User Settings File

<!--
TBD: Settings files. Rewrite with "remote settings" in mind (e.g. `local settings` on the remote host).
Consider renaming `zed: Open Local Settings` to `zed: Open Project Settings`.

TBD: Add settings documentation about how settings are merged as overlays. E.g. project>local>default. Note how settings that are maps are merged, but settings that are arrays are replaced and must include the defaults.
-->

Your settings JSON file can be opened with {#kb zed::OpenSettingsFile}.
By default it is located at `~/.config/zed/settings.json`, though if you have `XDG_CONFIG_HOME` in your environment on Linux it will be at `$XDG_CONFIG_HOME/zed/settings.json` instead.

Whatever you have added to your user settings file gets merged with any local configuration inside your projects.

### Default Settings

In the Settings Editor, the values you see set are the default ones.
You can also verify them in JSON by running {#action zed::OpenDefaultSettings} from the command palette.

Extensions that provide language servers may also provide default settings for those language servers.

## Project Settings File

Similarly to user files, you can open your project settings file by running {#action zed::OpenProjectSettings} from the command palette.
This will create a `.zed` directory containing`.zed/settings.json`.

Although most projects will only need one settings file at the root, you can add more local settings files for subdirectories as needed.
Not all settings can be set in local files, just those that impact the behavior of the editor and language tooling.
For example you can set `tab_size`, `formatter` etc. but not `theme`, `vim_mode` and similar.

The syntax for configuration files is a super-set of JSON that allows `//` comments.

## Per-release Channel Overrides

Zed reads the same `settings.json` across all release channels (Stable, Preview or Nightly).
However, you can scope overrides to a specific channel by adding top-level `stable`, `preview`, `nightly` or `dev` objects.
They are merged into the base configuration with settings from these keys taking precedence upon launching the specified build. For example:

```json [settings]
{
  "theme": "sunset",
  "vim_mode": false,
  "nightly": {
    "theme": "cave-light",
    "vim_mode": true
  },
  "preview": {
    "theme": "zed-dark"
  }
}
```

With this configuration, Stable keeps all base preferences, Preview switches to `zed-dark`, and Nightly enables Vim mode with a different theme.

Changing settings in the Settings Editor will always apply the change across all channels.

## All Settings Reference

For a complete list of all available settings, see the [All Settings](./reference/all-settings.md) reference.
