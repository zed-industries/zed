use gpui::{px, rems, Pixels, Rems, WindowContext};
use settings::Settings;
use theme::{ThemeSettings, UiDensity};
use ui_macros::derive_dynamic_spacing;

use crate::{rems_from_px, BASE_REM_SIZE_IN_PX};

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

/// A dynamic spacing system that adjusts spacing based on
/// [UiDensity].
///
/// The number following "Base" refers to the base pixel size
/// at the default rem size and spacing settings.
///
/// When possible, [Spacing] should be used over manual
/// or built-in spacing values in places dynamic spacing is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Spacing {
    /// No spacing
    None,
    /// Usually a one pixel spacing. Grows to 2px in comfortable density.
    /// @16px/rem: `1px`|`1px`|`2px`
    Base1,
    /// Extra small spacing - @16px/rem: `1px`|`2px`|`4px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base2,
    /// Small spacing - @16px/rem: `2px`|`3px`|`4px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base3,
    /// Small spacing - @16px/rem: `2px`|`4px`|`6px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base4,
    /// Medium spacing - @16px/rem: `3px`|`6px`|`8px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base6,
    /// Large spacing - @16px/rem: `4px`|`8px`|`10px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base8,
    /// Extra Large spacing - @16px/rem: `8px`|`12px`|`16px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base12,
    /// 2X Large spacing - @16px/rem: `12px`|`16px`|`20px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base16,
    /// 3X Large spacing - @16px/rem: `18px`|`20px`|`22px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base20,
    /// 4X Large spacing - @16px/rem: `20px`|`24px`|`28px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base24,
    /// 5X Large spacing - @16px/rem: `28px`|`32px`|`36px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base32,
    /// 6X Large spacing - @16px/rem: `36px`|`40px`|`44px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base40,
    /// 7X Large spacing - @16px/rem: `44px`|`48px`|`52px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Base48,
}

impl Spacing {
    /// Returns the spacing's scaling ratio in pixels.
    pub fn spacing_ratio(self, cx: &WindowContext) -> f32 {
        match ThemeSettings::get_global(cx).ui_density {
            UiDensity::Compact => match self {
                Spacing::None => 0.,
                Spacing::Base1 => 1. / BASE_REM_SIZE_IN_PX,
                Spacing::Base2 => 1. / BASE_REM_SIZE_IN_PX,
                Spacing::Base3 => 2. / BASE_REM_SIZE_IN_PX,
                Spacing::Base4 => 2. / BASE_REM_SIZE_IN_PX,
                Spacing::Base6 => 3. / BASE_REM_SIZE_IN_PX,
                Spacing::Base8 => 4. / BASE_REM_SIZE_IN_PX,
                Spacing::Base12 => 8. / BASE_REM_SIZE_IN_PX,
                Spacing::Base16 => 12. / BASE_REM_SIZE_IN_PX,
                Spacing::Base20 => 18. / BASE_REM_SIZE_IN_PX,
                Spacing::Base24 => 20. / BASE_REM_SIZE_IN_PX,
                Spacing::Base32 => 28. / BASE_REM_SIZE_IN_PX,
                Spacing::Base40 => 36. / BASE_REM_SIZE_IN_PX,
                Spacing::Base48 => 44. / BASE_REM_SIZE_IN_PX,
            },
            UiDensity::Default => match self {
                Spacing::None => 0.,
                Spacing::Base1 => 1. / BASE_REM_SIZE_IN_PX,
                Spacing::Base2 => 2. / BASE_REM_SIZE_IN_PX,
                Spacing::Base3 => 3. / BASE_REM_SIZE_IN_PX,
                Spacing::Base4 => 4. / BASE_REM_SIZE_IN_PX,
                Spacing::Base6 => 6. / BASE_REM_SIZE_IN_PX,
                Spacing::Base8 => 8. / BASE_REM_SIZE_IN_PX,
                Spacing::Base12 => 12. / BASE_REM_SIZE_IN_PX,
                Spacing::Base16 => 16. / BASE_REM_SIZE_IN_PX,
                Spacing::Base20 => 20. / BASE_REM_SIZE_IN_PX,
                Spacing::Base24 => 24. / BASE_REM_SIZE_IN_PX,
                Spacing::Base32 => 32. / BASE_REM_SIZE_IN_PX,
                Spacing::Base40 => 40. / BASE_REM_SIZE_IN_PX,
                Spacing::Base48 => 48. / BASE_REM_SIZE_IN_PX,
            },
            UiDensity::Comfortable => match self {
                Spacing::None => 0.,
                Spacing::Base1 => 2. / BASE_REM_SIZE_IN_PX,
                Spacing::Base2 => 3. / BASE_REM_SIZE_IN_PX,
                Spacing::Base3 => 4. / BASE_REM_SIZE_IN_PX,
                Spacing::Base4 => 6. / BASE_REM_SIZE_IN_PX,
                Spacing::Base6 => 8. / BASE_REM_SIZE_IN_PX,
                Spacing::Base8 => 10. / BASE_REM_SIZE_IN_PX,
                Spacing::Base12 => 16. / BASE_REM_SIZE_IN_PX,
                Spacing::Base16 => 20. / BASE_REM_SIZE_IN_PX,
                Spacing::Base20 => 22. / BASE_REM_SIZE_IN_PX,
                Spacing::Base24 => 28. / BASE_REM_SIZE_IN_PX,
                Spacing::Base32 => 36. / BASE_REM_SIZE_IN_PX,
                Spacing::Base40 => 44. / BASE_REM_SIZE_IN_PX,
                Spacing::Base48 => 52. / BASE_REM_SIZE_IN_PX,
            },
        }
    }

    /// Returns the spacing's value in rems.
    pub fn rems(self, cx: &WindowContext) -> Rems {
        rems(self.spacing_ratio(cx))
    }

    /// Returns the spacing's value in pixels.
    pub fn px(self, cx: &WindowContext) -> Pixels {
        let ui_font_size_f32: f32 = ThemeSettings::get_global(cx).ui_font_size.into();

        px(ui_font_size_f32 * self.spacing_ratio(cx))
    }
}

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
