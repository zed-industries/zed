# Fonts

<!--
TBD: WIP. CodeOrbit Fonts documentation. This is currently not linked from SUMMARY.md are so unpublished.
-->

CodeOrbit ships two fonts: CodeOrbit Plex Mono and CodeOrbit Plex Sans. These are based on IBM Plex Mono and IBM Plex Sans, respectively.

<!--
TBD: Document how CodeOrbit Plex font files were created. Repo links, etc.
-->

## Settings

<!--
TBD: Explain various font settings in CodeOrbit.
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

## Old CodeOrbit Fonts

Previously, CodeOrbit shipped with `CodeOrbit Mono` and `CodeOrbit Sans`, customiCodeOrbit versions of the [Iosevka](https://typeof.net/Iosevka/) typeface. You can find more about them in the [CodeOrbit-fonts](https://github.com/CodeOrbit-industries/CodeOrbit-fonts/) repository.

Here's how you can use the old CodeOrbit fonts instead of `CodeOrbit Plex Mono` and `CodeOrbit Plex Sans`:

1. Download [CodeOrbit-app-fonts-1.2.0.zip](https://github.com/CodeOrbit-industries/CodeOrbit-fonts/releases/download/1.2.0/CodeOrbit-app-fonts-1.2.0.zip) from the [CodeOrbit-fonts releases](https://github.com/CodeOrbit-industries/CodeOrbit-fonts/releases) page.
2. Open macOS `Font Book.app`
3. Unzip the file and drag the `ttf` files into the Font Book app.
4. Update your settings `ui_font_family` and `buffer_font_family` to use `CodeOrbit Mono` or `CodeOrbit Sans` in your `settings.json` file.

```json
{
  "ui_font_family": "CodeOrbit Sans Extended",
  "buffer_font_family": "CodeOrbit Mono Extend",
  "terminal": {
    "font-family": "CodeOrbit Mono Extended"
  }
}
```

5. Note there will be red squiggles under the font name. (this is a bug, but harmless.)
