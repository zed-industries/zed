use gpui::{px, rems, Pixels, Rems, WindowContext};
use settings::Settings;
use theme::{ThemeSettings, UiDensity};
use ui_macros::derive_dynamic_spacing;

// Derives [DynamicSpacing]. See [ui_macros::derive_dynamic_spacing].
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
pub fn ui_density(cx: &WindowContext) -> UiDensity {
    ThemeSettings::get_global(cx).ui_density
}
