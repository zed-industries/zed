# Zed UI

## Goals

- Intensity: Describe visual relationship between elements
- Use Components: Encourage reusability
- Make everything you need to build a theme contained in `/theme`
- Make Rust the source of truth for element types
- Reduce UI layers
- Ship a base ui theme

## Terminology

**Intensity**:

Intensity is a number between 1 and 100 representing the relative lightness or darkness of a value in a theme.

In dark themes, higher intensity values are lighter, and in light themes, higher intensity values are darker.

**Scale Factor**:

The scale factor is the ratio between a given theme's intensity and the maximum (1-100).

For example, if a theme has an intensity difference of 65 between its minimum and maximum, the scale factor would be 1.523 (99/65). This can be used to move between "relative" and "absolute" intensities.

**Relative Intensity**:

Relative intensity is the intensity of a value in a theme relative to the minimum and maximum of that theme.

For example, if a theme has an intensity range of 5-70, a value with an intensity of 40 simply has a relative intensity of 40.

**Absolute Intensity**:

Absolute intensity is the intensity of a value in a theme relative to the minimum and maximum possible intensities (1-100).

For example, if we use the same theme intensities as above (5-70), a value with an intensity of 40 would have an absolute intensity of 60.92 (40 `(value)` \* 1.523 `(scale factor)`), which would be rounded up to 61.

Absolute intensities are useful for programmatically shifting intensities, as absolute intensities are the same across all themes. If an element becomes **5% more intense**, it will always be 5% lighter or darker than its original value, even if the theme uses low contrasts.
