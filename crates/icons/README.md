# Zed Icons

## Guidelines

Icons are a big part of Zed, and they're how we convey hundreds of actions without relying on labeled buttons.
When introducing a new icon to the set, it's important to ensure it is consistent with the whole set, which follows a few guidelines:

1. The SVG view box should be 16x16.
2. For outlined icons, use a 1.5px stroke width.
3. Not all icons are mathematically aligned; there's quite a bit of optical adjustment. But try to keep the icon within an internal 12x12 bounding box as much as possible while ensuring proper visibility.
4. Use the `filled` and `outlined` terminology when introducing icons that will have the two variants.
5. Icons that are deeply contextual may have the feature context as their name prefix. For example, `ToolWeb`, `ReplPlay`, `DebugStepInto`, etc.
6. Avoid complex layer structure in the icon SVG, like clipping masks and whatnot. When the shape ends up too complex, we recommend running the SVG in [SVGOMG](https://jakearchibald.github.io/svgomg/) to clean it up a bit.

## Sourcing

Most icons are created by sourcing them from [Lucide](https://lucide.dev/).
Then, they're modified, adjusted, cleaned up, and simplified depending on their use and overall fit with Zed.

Sometimes, we may use other sources like [Phosphor](https://phosphoricons.com/), but we also design many of them completely from scratch.

## Contributing

To introduce a new icon, add the `.svg` file in the `assets/icon` directory and then add its corresponding item in the `icons.rs` file within the `crates` directory.

- SVG files in the assets folder follow a snake case name format.
- Icons in the `icons.rs` file follow the pascal case name format.

Ensure you tag a member of Zed's design team so we can adjust and double-check any newly introduced icon.
