use gpui::*;
use settings::Settings;
use theme::{ThemeSettings, UiDensity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Spacing {
    /// No spacing
    None,
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
}

impl Spacing {
    pub fn spacing_ratio(self, cx: &WindowContext) -> f32 {
        match ThemeSettings::get_global(cx).ui_density {
            UiDensity::Compact => match self {
                Spacing::None => 0.0,
                Spacing::XSmall => 1. / 16.,
                Spacing::Small => 2. / 16.,
                Spacing::Medium => 3. / 16.,
                Spacing::Large => 4. / 16.,
            },
            UiDensity::Default => match self {
                Spacing::None => 0.0,
                Spacing::XSmall => 2. / 16.,
                Spacing::Small => 4. / 16.,
                Spacing::Medium => 6. / 16.,
                Spacing::Large => 8. / 16.,
            },
            UiDensity::Comfortable => match self {
                Spacing::None => 0.0,
                Spacing::XSmall => 3. / 16.,
                Spacing::Small => 6. / 16.,
                Spacing::Medium => 8. / 16.,
                Spacing::Large => 10. / 16.,
            },
        }
    }

    pub fn rems(self, cx: &WindowContext) -> Rems {
        rems(self.spacing_ratio(cx))
    }
}

pub fn user_spacing_style(cx: &WindowContext) -> UiDensity {
    ThemeSettings::get_global(cx).ui_density
}
