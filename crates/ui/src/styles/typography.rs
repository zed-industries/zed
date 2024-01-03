use gpui::{rems, Rems};

#[derive(Debug, Default, Clone)]
pub enum UiTextSize {
    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    #[default]
    Default,
    /// The small size for UI text.
    ///
    /// `0.75rem` or `12px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    Small,

    /// The extra small size for UI text.
    ///
    /// `0.625rem` or `10px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    XSmall,
}

impl UiTextSize {
    pub fn rems(self) -> Rems {
        match self {
            Self::Default => rems(14. / 16.),
            Self::Small => rems(12. / 16.),
            Self::XSmall => rems(10. / 16.),
        }
    }
}
