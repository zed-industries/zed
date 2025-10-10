# Rainbow Brackets Cleanup Plan

## Overview
This document outlines the cleanup needed for the rainbow brackets feature to remove non-functional code and configuration options while preserving all working functionality.

## Current Issues
- **Dead code**: ~50% of the codebase consists of non-functional animation and visualization code
- **Misleading configuration**: 8 out of 11 configuration options don't actually do anything
- **False complexity**: The code pretends to support features that GPUI's highlighting system cannot deliver
- **Maintenance burden**: Extensive code for viewport tracking, animation states, and mode switching that serves no purpose

## Core Functionality to Preserve
The rainbow brackets feature currently provides:
1. **Bracket coloring** based on nesting depth (12 colors max due to GPUI limitations)
2. **Color customization** via start hue and step configuration
3. **Performance limiting** via max_brackets setting
4. **Enable/disable** toggle

**No working functionality will be removed.**

## Cleanup Scope

### 1. Configuration Simplification

**Current (11 options, 8 non-functional):**
```yaml
rainbow_brackets:
  enabled: true/false                # ✅ Works
  mode: "gradient"/"classic"          # ❌ Both do the same thing
  gradient_start_hue: 0-360           # ✅ Works (rename to start_hue)
  gradient_step: 1-180                # ✅ Works (rename to hue_step)
  show_in_minimap: true/false         # ❌ Does nothing
  pulse_active_scope: true/false      # ❌ Does nothing
  pulse_duration_ms: 100-1000         # ❌ Does nothing
  dim_inactive_scopes: true/false     # ❌ Does nothing
  animate_fade: true/false            # ❌ Doesn't work
  animate_glow: true/false            # ❌ Doesn't work
  animation_duration_ms: 50-1000      # ❌ Doesn't work
  max_brackets: number                # ✅ Works
```

**After cleanup (4 options, all functional):**
```yaml
rainbow_brackets:
  enabled: true/false      # Enable/disable rainbow brackets
  start_hue: 0-360        # Starting color (0=red, 120=green, 240=blue)
  hue_step: 1-180         # Color change per nesting level
  max_brackets: 100000    # Performance limit
```

### 2. Code Removal

**Files to modify:**

#### `crates/settings/src/settings_content/editor.rs`
- Remove `RainbowModeContent` enum
- Simplify `RainbowBracketsContent` to 4 fields only

#### `crates/editor/src/editor_settings.rs`
- Remove `RainbowMode` enum
- Simplify `RainbowBracketSettings` struct
- Remove mode parsing logic

#### `crates/editor/src/rainbow_brackets.rs`
**Remove entirely:**
- `RainbowMode` enum
- `GradientConfig` struct
- `AnimationState` struct
- `classic_colors` field and related methods
- `active_scope` tracking (unless used for cursor position)
- `animation_state` field
- `last_viewport` tracking
- `cached_edit_count` (keep if actually useful for performance)

**Remove methods:**
- `set_mode()`
- `set_gradient_config()`
- `default_classic_colors()`
- `should_trigger_fade()`
- `update_active_scope()` (unless needed)
- `active_scope()`
- `update_animation_settings()`
- `start_fade_animation()`
- `calculate_fade_progress()`
- `calculate_glow_intensity()`
- `apply_animation()`
- `needs_animation_frame()`
- `complete_fade_if_done()`

**Simplify methods:**
- `get_color_for_level()` - Direct HSL calculation, no mode checking
- `update_brackets()` - Remove viewport tracking and animation triggers
- `refresh_rainbow_brackets()` - Remove animation code, mode switching, active scope updates

### 3. Test Updates

**Remove tests for:**
- Animation features (8 tests)
- Classic vs gradient modes (4 tests)
- Viewport tracking (2 tests)
- Mode switching (1 test)

**Keep/improve tests for:**
- Color calculation with start_hue and hue_step
- Nesting level assignment
- Enable/disable functionality
- Max brackets limit
- Color wrapping behavior

### 4. Code Style Guidelines

The cleaned code should follow these principles:
- **Compact**: No unnecessary abstractions or indirection
- **Efficient**: Direct calculations, minimal allocations
- **Legible**: Clear variable names, logical flow
- **Honest**: Comments explain actual limitations (12 colors max)
- **Minimal**: Only essential comments, no obvious docstrings

Example of desired style:
```rust
pub struct RainbowBracketTracker {
    enabled: bool,
    start_hue: f32,
    hue_step: f32,
    max_brackets: u32,
    nesting_levels: IndexMap<Range<Anchor>, u32>,
}

impl RainbowBracketTracker {
    pub fn new(enabled: bool, start_hue: f32, hue_step: f32, max_brackets: u32) -> Self {
        Self {
            enabled,
            start_hue,
            hue_step,
            max_brackets,
            nesting_levels: IndexMap::new(),
        }
    }

    pub fn get_color_for_level(&self, level: u32) -> Hsla {
        let hue = (self.start_hue + (level as f32 * self.hue_step)) % 360.0;
        hsla(hue / 360.0, 0.75, 0.6, 1.0)
    }

    // ... other essential methods only
}
```

## Expected Outcome

- **Lines of code**: ~1000 → ~400 (60% reduction)
- **Configuration options**: 11 → 4 (only working options)
- **Test coverage**: Maintained or improved for actual functionality
- **Performance**: Identical or slightly better (less overhead)
- **User experience**: Identical (minus non-working config options)

## Implementation Notes

1. **No functionality loss**: Users will see the exact same rainbow brackets as before
2. **GPUI limitation**: Add a clear comment explaining why only 12 colors are possible
3. **Future-proof**: Structure should make it easy to add real animations if GPUI improves
4. **Single PR**: This should be done as one atomic change for easy review

## Success Criteria

- [ ] All working functionality preserved
- [ ] No non-functional code remains
- [ ] Configuration matches actual capabilities
- [ ] Tests cover all remaining code paths
- [ ] Code is significantly more maintainable
- [ ] Zed team can easily review and understand the feature
