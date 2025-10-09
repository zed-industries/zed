# Rainbow Brackets

Rainbow brackets is a feature that colors matching bracket pairs (`{}`, `[]`, `()`) in your code using different colors based on their nesting depth. This helps visually distinguish nested code blocks and makes it easier to identify matching bracket pairs.

## Current Status

The rainbow brackets feature is currently in early development. Basic functionality is working:
- Brackets are colored based on nesting depth
- Colors remain stable while scrolling
- Performance is optimized using caching

## Configuration

Rainbow brackets can be configured in your `settings.json` file under the `editor.rainbow_brackets` key:

```json
{
  "editor": {
    "rainbow_brackets": {
      "enabled": true,
      "mode": "gradient",
      "gradient_start_hue": 0,
      "gradient_step": 60,
      "show_in_minimap": true,
      "pulse_active_scope": true,
      "pulse_duration_ms": 300,
      "dim_inactive_scopes": false,
      "animate_fade": true,
      "animate_glow": true,
      "animation_duration_ms": 200,
      "max_brackets": 100000
    }
  }
}
```

### Configuration Options

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable or disable rainbow brackets |
| `mode` | string | `"gradient"` | Coloring mode: `"gradient"` (infinite HSL gradient) or `"classic"` (6-color cycling) |
| `gradient_start_hue` | number | `0` | Starting hue for gradient mode (0-360 degrees) |
| `gradient_step` | number | `60` | Hue step per nesting level in degrees |
| `show_in_minimap` | boolean | `true` | Whether to show bracket colors in the minimap |
| `pulse_active_scope` | boolean | `true` | Whether to pulse/animate the active bracket pair |
| `pulse_duration_ms` | number | `300` | Duration of pulse animation in milliseconds |
| `dim_inactive_scopes` | boolean | `false` | Whether to dim inactive bracket scopes |
| `animate_fade` | boolean | `true` | Whether to show fade-in animation when opening files |
| `animate_glow` | boolean | `true` | Whether to show glow animation on active bracket pair |
| `animation_duration_ms` | number | `200` | Duration of fade-in animation in milliseconds |
| `max_brackets` | number | `100000` | Maximum number of brackets to colorize for performance |

## How It Works

### Bracket Detection
Rainbow brackets uses tree-sitter to parse the syntax tree and identify bracket pairs in the code. The system processes brackets in the following types:
- Curly braces: `{}`
- Square brackets: `[]`
- Parentheses: `()`

### Nesting Level Calculation
The feature uses a stack-based O(n) algorithm to calculate nesting levels:
1. Brackets are sorted by their position in the document
2. A stack tracks open brackets
3. When an open bracket is found, it's pushed to the stack
4. When a close bracket is found, it's matched with the corresponding open bracket
5. The nesting level is determined by the stack depth

### Color Assignment
Colors are assigned based on the nesting level:
- **Gradient Mode**: Uses an HSL color wheel with configurable start hue and step
- **Classic Mode**: Cycles through a fixed set of 6 colors (not fully implemented)

To avoid highlight conflicts, the system uses 12 separate highlight types (RainbowBracketHighlight0-11) with colors assigned using modulo arithmetic.

### Caching
To improve performance, bracket calculations are cached:
- Cache is invalidated when the buffer is edited (tracked by edit_count)
- Empty results (when tree-sitter isn't ready) are not cached
- Cache uses IndexMap to maintain deterministic iteration order

## Current Limitations

### Non-Functional Features
Several configuration options are defined but not yet implemented:
- **Classic mode**: The 6-color cycling mode is not implemented
- **Animations**: All animation settings (pulse, fade, glow) are not functional
- **Minimap**: Bracket colors are not shown in the minimap
- **Dim inactive scopes**: Does not dim brackets outside the current scope
- **Active scope tracking**: Cursor position tracking for active brackets is not working

### Known Issues
1. **Tree-sitter dependency**: Brackets won't appear until tree-sitter finishes parsing
2. **Animation limitations**: GPUI's `highlight_text()` API doesn't support animations
3. **Large file performance**: Files with >100,000 brackets are skipped by default

### Performance Considerations
- The `max_brackets` setting limits processing to prevent performance issues
- Default limit is 100,000 brackets, suitable for most large files
- Processing is limited to the visible viewport when possible

## Implementation Details

### Key Files
- `crates/editor/src/rainbow_brackets.rs` - Main implementation
- `crates/editor/src/editor_settings.rs` - Settings integration
- `crates/settings/src/settings_content/editor.rs` - Settings schema

### Data Structures
```rust
pub struct RainbowBracketTracker {
    nesting_levels: IndexMap<Range<Anchor>, u32>,  // Bracket ranges to nesting levels
    cached_edit_count: Option<usize>,               // For cache invalidation
    enabled: bool,                                  // Feature toggle
    mode: RainbowMode,                              // Gradient or Classic
    gradient_config: GradientConfig,                // Color configuration
    // ... animation fields (not functional)
}
```

### Highlight System
The implementation uses Zed's highlight system with 12 separate highlight types to avoid conflicts when multiple brackets need different colors at the same position.

## Future Improvements

Planned enhancements include:
- Implementing classic 6-color mode
- Adding animation support when GPUI allows
- Minimap integration
- Active scope highlighting with cursor tracking
- Language-specific bracket customization
- Performance optimizations for very large files
- Configurable bracket types per language

## Troubleshooting

### Brackets not appearing
- Check that `enabled` is set to `true` in settings
- Wait for tree-sitter to finish parsing (especially on large files)
- Verify the file has fewer brackets than `max_brackets` setting

### Colors changing unexpectedly
- This issue has been fixed by using IndexMap for deterministic ordering
- If still occurring, check for settings changes or file edits

### Performance issues
- Reduce `max_brackets` setting for very large files
- Consider disabling for specific file types if needed