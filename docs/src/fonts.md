# Fonts

<!--
TBD: WIP. Zed Fonts documentation. This is currently not linked from SUMMARY.md are so unpublished.
-->

Zed ships two fonts: Zed Plex Mono and Zed Plex Sans. These are based on IBM Plex Mono and IBM Plex Sans, respectively.

<!--
TBD: Document how Zed Plex font files were created. Repo links, etc.
-->

## Settings

<!--
TBD: Explain various font settings in Zed.
-->

- Buffer fonts
  - `buffer-font-family`
  - `buffer-font-features`
  - `buffer-font-size`
  - `buffer-line-height`
- UI fonts
  - `ui_font_family`
  - `ui_font_fallbacks`
  - `ui_font_features`
  - `ui_font_weight`
  - `ui_font_size`
- Terminal fonts
  - `terminal.font-size`
  - `terminal.font-family`
  - `terminal.font-features`
- Other settings:
  - `active-pane-magnification`

## Old Zed Fonts

Previously, Zed shipped with `Zed Mono` and `Zed Sans`, customized versions of the [Iosevka](https://typeof.net/Iosevka/) typeface. You can find more about them in the [zed-fonts](https://github.com/zed-industries/zed-fonts/) repository.

Here's how you can use the old Zed fonts instead of `Zed Plex Mono` and `Zed Plex Sans`:

1. Download [zed-app-fonts-1.2.0.zip](https://github.com/zed-industries/zed-fonts/releases/download/1.2.0/zed-app-fonts-1.2.0.zip) from the [zed-fonts releases](https://github.com/zed-industries/zed-fonts/releases) page.
2. Open macOS `Font Book.app`
3. Unzip the file and drag the `ttf` files into the Font Book app.
4. Update your settings `ui_font_family` and `buffer_font_family` to use `Zed Mono` or `Zed Sans` in your `settings.json` file.

```json
{
  "ui_font_family": "Zed Sans Extended",
  "buffer_font_family": "Zed Mono Extend",
  "terminal": {
    "font-family": "Zed Mono Extended"
  }
}
```

5. Note there will be red squiggles under the font name. (this is a bug, but harmless.)
