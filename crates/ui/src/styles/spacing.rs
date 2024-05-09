use gpui::{px, rems, Pixels, Rems, WindowContext};
use settings::Settings;
use theme::{ThemeSettings, UiDensity};

use crate::{rems_from_px, BASE_REM_SIZE_IN_PX};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Spacing {
    /// No spacing
    None,
    /// Usually a one pixel spacing. Grows to 2px in comfortable density.
    /// @16px/rem: `1px`|`1px`|`2px`
    XXSmall,
    /// Extra small spacing - @16px/rem: `1px`|`2px`|`4px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    XSmall,
    /// Small spacing - @16px/rem: `2px`|`4px`|`6px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Small,
    /// Medium spacing - @16px/rem: `3px`|`6px`|`8px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Medium,
    /// Large spacing - @16px/rem: `4px`|`8px`|`10px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    Large,
    /// Extra Large spacing - @16px/rem: `8px`|`12px`|`16px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    XLarge,
    /// 2X Large spacing - @16px/rem: `12px`|`16px`|`20px`
    ///
    /// Relative to the user's `ui_font_size` and [UiDensity] setting.
    XXLarge,
}

impl Spacing {
    pub fn spacing_ratio(self, cx: &WindowContext) -> f32 {
        match ThemeSettings::get_global(cx).ui_density {
            UiDensity::Compact => match self {
                Spacing::None => 0.,
                Spacing::XXSmall => 1. / BASE_REM_SIZE_IN_PX,
                Spacing::XSmall => 1. / BASE_REM_SIZE_IN_PX,
                Spacing::Small => 2. / BASE_REM_SIZE_IN_PX,
                Spacing::Medium => 3. / BASE_REM_SIZE_IN_PX,
                Spacing::Large => 4. / BASE_REM_SIZE_IN_PX,
                Spacing::XLarge => 8. / BASE_REM_SIZE_IN_PX,
                Spacing::XXLarge => 12. / BASE_REM_SIZE_IN_PX,
            },
            UiDensity::Default => match self {
                Spacing::None => 0.,
                Spacing::XXSmall => 1. / BASE_REM_SIZE_IN_PX,
                Spacing::XSmall => 2. / BASE_REM_SIZE_IN_PX,
                Spacing::Small => 4. / BASE_REM_SIZE_IN_PX,
                Spacing::Medium => 6. / BASE_REM_SIZE_IN_PX,
                Spacing::Large => 8. / BASE_REM_SIZE_IN_PX,
                Spacing::XLarge => 12. / BASE_REM_SIZE_IN_PX,
                Spacing::XXLarge => 16. / BASE_REM_SIZE_IN_PX,
            },
            UiDensity::Comfortable => match self {
                Spacing::None => 0.,
                Spacing::XXSmall => 2. / BASE_REM_SIZE_IN_PX,
                Spacing::XSmall => 3. / BASE_REM_SIZE_IN_PX,
                Spacing::Small => 6. / BASE_REM_SIZE_IN_PX,
                Spacing::Medium => 8. / BASE_REM_SIZE_IN_PX,
                Spacing::Large => 10. / BASE_REM_SIZE_IN_PX,
                Spacing::XLarge => 16. / BASE_REM_SIZE_IN_PX,
                Spacing::XXLarge => 20. / BASE_REM_SIZE_IN_PX,
            },
        }
    }

    pub fn rems(self, cx: &WindowContext) -> Rems {
        rems(self.spacing_ratio(cx))
    }

    pub fn px(self, cx: &WindowContext) -> Pixels {
        let ui_font_size_f32: f32 = ThemeSettings::get_global(cx).ui_font_size.into();

        px(ui_font_size_f32 * self.spacing_ratio(cx))
    }
}

pub fn user_spacing_style(cx: &WindowContext) -> UiDensity {
    ThemeSettings::get_global(cx).ui_density
}

pub fn custom_spacing(cx: &WindowContext, size: f32) -> Rems {
    rems_from_px(size * user_spacing_style(cx).spacing_ratio())
}
