use gpui::{App, Pixels, Rems, px, rems};
use settings::Settings;
use theme::{ThemeSettings, UiDensity};
use ui_macros::derive_dynamic_spacing;

// Derives [DynamicSpacing]. See [ui_macros::derive_dynamic_spacing].
//
// There are 3 UI density settings: Compact, Default, and Comfortable.
//
// When a tuple of three values is provided, the values are used directly.
//
// Example: (1, 2, 4) => Compact: 1px, Default: 2px, Comfortable: 4px
//
// When a single value is provided, the standard spacing formula is
// used to derive the of spacing values. This formula can be found in
// the macro.
//
// Example:
//
// Assuming the standard formula is (n-4, n, n+4)
//
// 24 => Compact: 20px, Default: 24px, Comfortable: 28px
//
// The [DynamicSpacing] enum variants use a BaseXX format,
// where XX = the pixel value @ default rem size and the default UI density.
//
// Example:
//
// DynamicSpacing::Base16 would return 16px at the default UI scale & density.
derive_dynamic_spacing![
    (0, 0, 0),
    (1, 1, 2),
    (1, 2, 4),
    (2, 3, 4),
    (2, 4, 6),
    (3, 6, 8),
    (4, 8, 10),
    (10, 12, 14),
    (14, 16, 18),
    (18, 20, 22),
    24,
    32,
    40,
    48
];

/// Returns the current [`UiDensity`] setting. Use this to
/// modify or show something in the UI other than spacing.
///
/// Do not use this to calculate spacing values.
///
/// Always use [DynamicSpacing] for spacing values.
pub fn ui_density(cx: &mut App) -> UiDensity {
    ThemeSettings::get_global(cx).ui_density
}
