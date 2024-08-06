# Fonts

TBD: Document fonts in Zed
Consider renaming this `configuring-zed-fonts` and moving all font settings here.

- Zed Plex Sans & Zed Plex Mono: based on IBM Plex Mono and IBM Plex Sans, etc
  - Document / commit to repo how these were generated customizations made.
- Zed Mono & Zed Sans: Iosevka https://github.com/zed-industries/zed-fonts/

- Step-by-step instructions to use Zed Mono and Zed Sans
  - https://github.com/zed-industries/zed-fonts/releases/download/1.2.0/zed-app-fonts-1.2.0.zip
  - Note: If you install the full set (from zed-mono-1.2.0.zip or zed-sans-1.2.0.zip) you will need to set your font name to "Zed Sans Extended" instead of just "Zed Sans" otherwise Zed will incorrectly use "Zed Sans Narrow" (bug). Also this will show squiggles.
    - https://github.com/zed-industries/zed/pull/13596#issuecomment-2211097532

## Configuration

- Buffer fonts
  - https://zed.dev/docs/configuring-zed#buffer-font-family
  - https://zed.dev/docs/configuring-zed#buffer-font-features
  - https://zed.dev/docs/configuring-zed#buffer-font-size
  - https://zed.dev/docs/configuring-zed#buffer-line-height
- Terminal fonts: (note these anchors should be changed to be `terminal-` prefixed)
  - https://zed.dev/docs/configuring-zed#font-size
  - https://zed.dev/docs/configuring-zed#font-family
  - https://zed.dev/docs/configuring-zed#font-features

TBD: Missing font settings documentation for `ui_font_*` and `buffer_font_fallbacks`

## See also:

- [configuring zed: active_pane_magnification](./configuring-zed.md#active-pane-magnification)
