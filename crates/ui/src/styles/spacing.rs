use gpui::*;
use settings::Settings;
use theme::{SpacingStyle, ThemeSettings};

pub enum Spacing {
    None,
    XSmall,
    Small,
    Medium,
    Large,
}

impl Spacing {
    fn rems(self, cx: &WindowContext) -> Rems {
        let spacing_style = ThemeSettings::get_global(cx).spacing;

        match spacing_style {
            SpacingStyle::Loose => match self {
                Self::None => rems(0.0),
                Self::XSmall => rems(1. / 16.),
                Self::Small => rems(2. / 16.),
                Self::Medium => rems(3. / 16.),
                Self::Large => rems(4. / 16.),
            },
            SpacingStyle::Tight => match self {
                Self::None => rems(0.0),
                Self::XSmall => rems(2. / 16.),
                Self::Small => rems(4. / 16.),
                Self::Medium => rems(6. / 16.),
                Self::Large => rems(8. / 16.),
            },
            SpacingStyle::Normal => match self {
                Self::None => rems(0.0),
                Self::XSmall => rems(3. / 16.),
                Self::Small => rems(4. / 16.),
                Self::Medium => rems(8. / 16.),
                Self::Large => rems(10. / 16.),
            },
        }
    }
}

// TODO: Split out a `ui_macros` crate and use a macro like in
// `gpui_macros` to generate these methods

/// Extends [`gpui::Styled`] with spacing-specific styling methods.
pub trait StyledSpacing: Styled + Sized {
    fn space_none(self, cx: &mut WindowContext) -> Rems {
        Spacing::None.rems(cx)
    }

    fn space_gap_none(self, cx: &mut WindowContext) -> Self {
        self.gap(Spacing::None.rems(cx))
    }

    fn space_p_none(self, cx: &mut WindowContext) -> Self {
        self.px(Spacing::None.rems(cx))
    }

    fn space_px_none(self, cx: &mut WindowContext) -> Self {
        self.px(Spacing::None.rems(cx))
    }
}

impl<E: Styled> StyledSpacing for E {}
