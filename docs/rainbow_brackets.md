# Rainbow Brackets

Rainbow brackets (also known as bracket pair colorization) is a feature that colorizes matching bracket pairs based on their nesting depth, making it easier to visually identify code structure and matching brackets.

## Overview

When enabled, rainbow brackets will:
- Assign different colors to bracket pairs based on their nesting level
- Use a rotating color palette for deeply nested structures
- Help you quickly identify matching brackets and code blocks
- Reduce cognitive load when reading complex nested code

## Enabling Rainbow Brackets

### Via Settings UI

1. Open Settings (`Cmd+,` on macOS, `Ctrl+,` on Linux/Windows)
2. Navigate to Editor settings
3. Enable "Rainbow Brackets"

### Via settings.json

Add the following to your `settings.json`:

```json
{
  "editor": {
    "rainbow_brackets": true
  }
}
```

## How It Works

### Color Assignment

Rainbow brackets uses a color palette from your current theme to colorize brackets:

- **Level 0** (outermost): First color in the palette
- **Level 1**: Second color
- **Level 2**: Third color
- And so on...

When the nesting depth exceeds the number of colors in the palette, the colors cycle back to the beginning.

### Supported Bracket Types

The feature works with all bracket types defined by your language's syntax:

- Round brackets: `( )`
- Square brackets: `[ ]`
- Curly braces: `{ }`
- Angle brackets: `< >` (where applicable)

### Example

```rust
fn main() {
    let data = vec![
        HashMap::from([
            ("key1", vec![1, 2, 3]),
            ("key2", vec![4, 5, 6]),
        ])
    ];
}
```

In this example:
- `{ }` around `main` body would be color 1
- `[ ]` in `vec![]` would be color 2
- `[ ]` in `HashMap::from([])` would be color 3
- `( )` in tuples would be color 4
- Inner `[ ]` in `vec![1, 2, 3]` would be color 5

## Implementation Details

### Performance

The rainbow brackets implementation is optimized for performance:

- Only visible brackets are colorized
- Bracket depth calculation is cached
- Updates occur asynchronously to avoid blocking the UI

### Compatibility

Rainbow brackets works with:
- All languages supported by Zed
- All themes (uses theme accent colors when available)
- Other editor features like bracket matching and auto-closing

## Theme Customization

Theme authors can provide custom rainbow bracket colors by defining accent colors in their theme:

```json
{
  "accents": [
    "#e06c75",  // Red
    "#98c379",  // Green
    "#61afef",  // Blue
    "#c678dd",  // Purple
    "#56b6c2",  // Cyan
    "#e5c07b"   // Yellow
  ]
}
```

If no custom colors are defined, the feature falls back to a default palette derived from the theme's existing colors.

## Troubleshooting

### Rainbow brackets not appearing

1. **Check if the feature is enabled** in your settings
2. **Verify your theme** has sufficient accent colors defined
3. **Check language support** - ensure the language has bracket definitions
4. **Restart Zed** if you recently enabled the feature

### Colors look too similar

Try a different theme or customize your theme's accent colors to provide more contrast between nesting levels.

### Performance issues

If you experience performance issues with rainbow brackets:

1. Disable the feature temporarily
2. Report an issue with details about your file size and language
3. Consider using the feature only for smaller files

## Related Features

- **Bracket Matching**: Highlights matching brackets when cursor is adjacent to a bracket
- **Auto-closing Brackets**: Automatically inserts closing brackets
- **Indent Guides**: Visual guides showing indentation levels

## Keyboard Shortcuts

While rainbow brackets is a visual feature, these related shortcuts are useful:

- **Go to Matching Bracket**: `Cmd+Shift+\` (macOS) / `Ctrl+Shift+\` (Linux/Windows)
- **Select to Matching Bracket**: Select from cursor to matching bracket
- **Move to Enclosing Bracket**: Jump to the enclosing bracket pair

## Code Quality Benefits

Rainbow brackets improves code quality by:

1. **Reducing Syntax Errors**: Easier to spot mismatched brackets
2. **Improving Readability**: Clear visual hierarchy of code structure
3. **Faster Navigation**: Quickly identify code blocks and nesting levels
4. **Better Collaboration**: Team members can understand code structure faster

## API for Extension Authors

Extension authors can leverage the rainbow brackets system by ensuring their language definitions include proper bracket pair configurations:

```rust
BracketPairConfig {
    pairs: vec![
        BracketPair {
            start: "{".to_string(),
            end: "}".to_string(),
            close: true,
            surround: true,
            newline: true,
        },
        // Add more bracket pairs...
    ],
    ..Default::default()
}
```

## Future Enhancements

Planned improvements for rainbow brackets:

- [ ] Per-language color customization
- [ ] Different color schemes for different bracket types
- [ ] Animation when navigating to matching brackets
- [ ] Dimming of inactive bracket pairs
- [ ] Bracket pair connection lines (optional)

## Feedback

We'd love to hear your feedback on rainbow brackets! Please:

- Report bugs via GitHub issues
- Suggest improvements in the Zed community forum
- Share your color schemes with the community

## References

- [VSCode Bracket Pair Colorization](https://code.visualstudio.com/blogs/2021/09/29/bracket-pair-colorization)
- [Zed Editor Documentation](https://zed.dev/docs)
- [Theme Customization Guide](https://zed.dev/docs/themes)