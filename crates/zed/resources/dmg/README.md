# macOS DMG installer design

These assets style the disk-image window users see when they open a downloaded
`Zed-*.dmg` (the "drag Zed into Applications" screen). They are consumed by
`script/bundle-mac` when it builds the DMG.

## Files

- `background.png` — background image for the installer window (1x).
- `background@2x.png` — optional Retina variant; Finder picks it up automatically.

If `background.png` is absent, `script/bundle-mac` falls back to producing a
plain, unstyled DMG window, so the build never breaks on a missing design.

## Sizing and layout

The window geometry and icon positions live in `script/bundle-mac` (search for
`osascript`). They must match the dimensions of `background.png`:

- Window content size: currently 660x400 pt (bounds `{200, 120, 860, 520}`).
- `Zed.app` icon position: `{180, 200}`.
- `Applications` symlink icon position: `{480, 200}`.

Export `background.png` at the window's point size (e.g. 660x400) and
`background@2x.png` at twice that (1320x800). If you change the window size in
the design, update the coordinates in `script/bundle-mac` to match.
