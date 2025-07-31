use crate::prelude::*;
use gpui::{
    AnyElement, App, IntoElement, ParentElement, Rems, RenderOnce, SharedString, Styled, Window,
    div, rems,
};
use settings::Settings;
use theme::{ActiveTheme, ThemeSettings};

use crate::{Color, rems_from_px};

/// Extends [`gpui::Styled`] with typography-related styling methods.
pub trait StyledTypography: Styled + Sized {
    /// Sets the font family to the buffer font.
    fn font_buffer(self, cx: &App) -> Self {
        let settings = ThemeSettings::get_global(cx);
        let buffer_font_family = settings.buffer_font.family.clone();

        self.font_family(buffer_font_family)
    }

    /// Sets the font family to the UI font.
    fn font_ui(self, cx: &App) -> Self {
        let settings = ThemeSettings::get_global(cx);
        let ui_font_family = settings.ui_font.family.clone();

        self.font_family(ui_font_family)
    }

    /// Sets the text size using a [`TextSize`].
    fn text_ui_size(self, size: TextSize, cx: &App) -> Self {
        self.text_size(size.rems(cx))
    }

    /// The large size for UI text.
    ///
    /// `1rem` or `16px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_lg(self, cx: &App) -> Self {
        self.text_size(TextSize::Large.rems(cx))
    }

    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui_sm` for smaller text.
    fn text_ui(self, cx: &App) -> Self {
        self.text_size(TextSize::default().rems(cx))
    }

    /// The small size for UI text.
    ///
    /// `0.75rem` or `12px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_sm(self, cx: &App) -> Self {
        self.text_size(TextSize::Small.rems(cx))
    }

    /// The extra small size for UI text.
    ///
    /// `0.625rem` or `10px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_xs(self, cx: &App) -> Self {
        self.text_size(TextSize::XSmall.rems(cx))
    }

    /// The font size for buffer text.
    ///
    /// Retrieves the default font size, or the user's custom font size if set.
    ///
    /// This should only be used for text that is displayed in a buffer,
    /// or other places that text needs to match the user's buffer font size.
    fn text_buffer(self, cx: &App) -> Self {
        let settings = ThemeSettings::get_global(cx);
        self.text_size(settings.buffer_font_size(cx))
    }
}

impl<E: Styled> StyledTypography for E {}

/// A utility for getting the size of various semantic text sizes.
#[derive(Debug, Default, Clone)]
pub enum TextSize {
    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    #[default]
    Default,
    /// The large size for UI text.
    ///
    /// `1rem` or `16px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    Large,

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

    /// The `ui_font_size` set by the user.
    Ui,
    /// The `buffer_font_size` set by the user.
    Editor,
    // TODO: The terminal settings will need to be passed to
    // ThemeSettings before we can enable this.
    //// The `terminal.font_size` set by the user.
    // Terminal,
}

impl TextSize {
    /// Returns the text size in rems.
    pub fn rems(self, cx: &App) -> Rems {
        let theme_settings = ThemeSettings::get_global(cx);

        match self {
            Self::Large => rems_from_px(16.),
            Self::Default => rems_from_px(14.),
            Self::Small => rems_from_px(12.),
            Self::XSmall => rems_from_px(10.),
            Self::Ui => rems_from_px(theme_settings.ui_font_size(cx).into()),
            Self::Editor => rems_from_px(theme_settings.buffer_font_size(cx).into()),
        }
    }
}

/// The size of a [`Headline`] element
///
/// Defaults to a Major Second scale.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum HeadlineSize {
    /// An extra small headline - `~14px` @16px/rem
    XSmall,
    /// A small headline - `16px` @16px/rem
    Small,
    #[default]
    /// A medium headline - `~18px` @16px/rem
    Medium,
    /// A large headline - `~20px` @16px/rem
    Large,
    /// An extra large headline - `~22px` @16px/rem
    XLarge,
}

impl HeadlineSize {
    /// Returns the headline size in rems.
    pub fn rems(self) -> Rems {
        match self {
            Self::XSmall => rems(0.88),
            Self::Small => rems(1.0),
            Self::Medium => rems(1.125),
            Self::Large => rems(1.27),
            Self::XLarge => rems(1.43),
        }
    }

    /// Returns the line height for the headline size.
    pub fn line_height(self) -> Rems {
        match self {
            Self::XSmall => rems(1.6),
            Self::Small => rems(1.6),
            Self::Medium => rems(1.6),
            Self::Large => rems(1.6),
            Self::XLarge => rems(1.6),
        }
    }
}

/// A headline element, used to emphasize some text and
/// create a visual hierarchy.
#[derive(IntoElement, RegisterComponent)]
pub struct Headline {
    size: HeadlineSize,
    text: SharedString,
    color: Color,
}

impl RenderOnce for Headline {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();

        div()
            .font(ui_font)
            .line_height(self.size.line_height())
            .text_size(self.size.rems())
            .text_color(cx.theme().colors().text)
            .child(self.text)
    }
}

impl Headline {
    /// Create a new headline element.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            size: HeadlineSize::default(),
            text: text.into(),
            color: Color::default(),
        }
    }

    /// Set the size of the headline.
    pub fn size(mut self, size: HeadlineSize) -> Self {
        self.size = size;
        self
    }

    /// Set the color of the headline.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Component for Headline {
    fn scope() -> ComponentScope {
        ComponentScope::Typography
    }

    fn description() -> Option<&'static str> {
        Some("A headline element used to emphasize text and create visual hierarchy in the UI.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_1()
                .children(vec![
                    single_example(
                        "XLarge",
                        Headline::new("XLarge Headline")
                            .size(HeadlineSize::XLarge)
                            .into_any_element(),
                    ),
                    single_example(
                        "Large",
                        Headline::new("Large Headline")
                            .size(HeadlineSize::Large)
                            .into_any_element(),
                    ),
                    single_example(
                        "Medium (Default)",
                        Headline::new("Medium Headline").into_any_element(),
                    ),
                    single_example(
                        "Small",
                        Headline::new("Small Headline")
                            .size(HeadlineSize::Small)
                            .into_any_element(),
                    ),
                    single_example(
                        "XSmall",
                        Headline::new("XSmall Headline")
                            .size(HeadlineSize::XSmall)
                            .into_any_element(),
                    ),
                ])
                .into_any_element(),
        )
    }
}
