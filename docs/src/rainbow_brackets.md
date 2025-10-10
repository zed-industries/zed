# Rainbow Brackets

Rainbow brackets colors matching bracket pairs (`{}`, `[]`, `()`) in your code using different colors based on their nesting depth. This helps visually distinguish nested code blocks and makes it easier to identify matching bracket pairs.

## Configuration

Rainbow brackets can be configured in your `settings.json`:

```json
{
  "rainbow_brackets": {
    "enabled": true,
    "start_hue": 0,
    "hue_step": 30,
    "max_brackets": 100000
  }
}
```

### Configuration Options

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable or disable rainbow brackets |
| `start_hue` | number | `0` | Starting hue for colors (0-360 degrees). 0=red, 120=green, 240=blue |
| `hue_step` | number | `30` | Hue change per nesting level in degrees (1-180) |
| `max_brackets` | number | `100000` | Maximum number of brackets to colorize (for performance) |

## How It Works

### Bracket Detection
Rainbow brackets uses tree-sitter to parse the syntax tree and identify bracket pairs. The specific brackets highlighted depend on each language's configuration, but typically include:
- Curly braces: `{}`
- Square brackets: `[]`
- Parentheses: `()`

### Color Assignment
Colors are assigned using the HSL color wheel:
1. Level 0 (outermost) starts at `start_hue`
2. Each nesting level adds `hue_step` degrees
3. Colors wrap around at 360 degrees (e.g., 380° becomes 20°)

Example with `start_hue: 0` and `hue_step: 60`:
- Level 0: Red (0°)
- Level 1: Yellow (60°)
- Level 2: Green (120°)
- Level 3: Cyan (180°)
- Level 4: Blue (240°)
- Level 5: Magenta (300°)
- Level 6: Red again (360° = 0°)

### Technical Limitations

Due to GPUI's highlight system architecture, rainbow brackets can display a **maximum of 12 unique colors** simultaneously. Nesting levels beyond 12 will cycle through the same colors (level 12 uses the same color as level 0, level 13 as level 1, etc.).

This limitation exists because GPUI's `highlight_text()` API requires separate highlight types for overlapping highlights, and we define 12 types (`RainbowBracketHighlight0` through `RainbowBracketHighlight11`).

## Performance

### Optimization Features
- **Smart caching**: Bracket calculations are cached and only recomputed when the buffer is edited
- **Performance limit**: Files with more than `max_brackets` brackets skip highlighting
- **Efficient algorithm**: O(n) single-pass algorithm for nesting level calculation

### Performance Tips
- For very large files (>100K brackets), consider reducing `max_brackets` or disabling rainbow brackets
- The default limit of 100,000 brackets handles most large files without issues

## Language Support

Rainbow brackets work with any language that has bracket pairs configured. Most languages in Zed already have proper bracket configurations, including:
- Rust, C, C++, Go
- JavaScript, TypeScript, TSX
- Python
- JSON, YAML
- HTML, CSS
- And many more

Each language defines its bracket pairs in its `config.toml` file. For example, Python includes `{}`, `[]`, `()` as well as various string delimiters.

## Troubleshooting

### Brackets not appearing
1. Ensure `enabled` is set to `true` in your settings
2. Wait for tree-sitter to finish parsing (may take a moment on large files)
3. Check that the file has fewer brackets than the `max_brackets` setting
4. Verify the file's language has bracket pairs configured

### Colors not as expected
- Remember that only 12 unique colors can be displayed
- Colors cycle: level 12 = level 0, level 13 = level 1, etc.
- Adjust `start_hue` and `hue_step` to customize the color palette

### Performance issues
- Reduce `max_brackets` setting for very large files
- Consider disabling for specific workspaces with many large generated files

## Implementation Details

The rainbow brackets implementation lives in:
- `crates/editor/src/rainbow_brackets.rs` - Core implementation
- `crates/editor/src/editor_settings.rs` - Settings integration
- `crates/settings/src/settings_content/editor.rs` - Configuration schema

The implementation uses an efficient stack-based algorithm to calculate nesting levels in a single pass, caches results to avoid recalculation during scrolling, and integrates cleanly with Zed's highlighting system.