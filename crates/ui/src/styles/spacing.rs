use gpui::{px, rems, Pixels, Rems, WindowContext};
use settings::Settings;
use theme::{ThemeSettings, UiDensity};
use ui_macros::derive_dynamic_spacing;

use crate::rems_from_px;

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

/// Returns the current [`UiDensity`] setting.
///
/// Do not use this for custom spacing values.
/// Use existing [`Spacing`] values instead.
pub fn ui_density(cx: &WindowContext) -> UiDensity {
    ThemeSettings::get_global(cx).ui_density
}

/// If you use this, talk to @iamnbutler and let me know what you're doing
/// that needs custom spacing–I'd love to understand so we can extend the
/// system further and remove the need for this.
///
/// Returns a custom spacing value based on the current [`UiDensity`].
pub fn custom_spacing(cx: &WindowContext, size: f32) -> Rems {
    rems_from_px(size * ui_density(cx).spacing_ratio())
}
