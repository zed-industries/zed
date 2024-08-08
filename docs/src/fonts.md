# Fonts

TBD: WIP: Document fonts in Zed

- Zed Plex Sans & Zed Plex Mono: based on IBM Plex Mono and IBM Plex Sans, etc
  - Document / commit to repo how these were generated customizations made.
- Zed Mono & Zed Sans: Iosevka https://github.com/zed-industries/zed-fonts/

- Step-by-step instructions to use Zed Mono and Zed Sans
  - https://github.com/zed-industries/zed-fonts/releases/download/1.2.0/zed-app-fonts-1.2.0.zip
  - Note: If you install the full set (from zed-mono-1.2.0.zip or zed-sans-1.2.0.zip) you will need to set your font name to "Zed Sans Extended" instead of just "Zed Sans" otherwise Zed will incorrectly use "Zed Sans Narrow" (bug). Also this will show squiggles.
    - https://github.com/zed-industries/zed/pull/13596#issuecomment-2211097532

## Configuration

- Buffer fonts
  - `buffer-font-family`
  - `buffer-font-features`
  - `buffer-font-size`
  - `buffer-line-height`
  - `ui_font_family`
  - `ui_font_fallbacks`
  - `ui_font_features`
  - `ui_font_weight`
  - `ui_font_size`
  - `terminal.font-size`
  - `terminal.font-family`
  - `terminal.font-features`

## See also:

- [configuring zed: active_pane_magnification](./configuring-zed.md#active-pane-magnification)
