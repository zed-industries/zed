use gpui::{
    div, rems, IntoElement, ParentElement, Rems, RenderOnce, SharedString, Styled, WindowContext,
};
use settings::Settings;
use theme::{ActiveTheme, ThemeSettings};

use crate::{rems_from_px, Color};

/// Extends [`gpui::Styled`] with typography-related styling methods.
pub trait StyledTypography: Styled + Sized {
    /// Sets the text size using a [`UiTextSize`].
    fn text_ui_size(self, size: TextSize, cx: &WindowContext) -> Self {
        self.text_size(size.rems(cx))
    }

    /// The large size for UI text.
    ///
    /// `1rem` or `16px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_lg(self, cx: &WindowContext) -> Self {
        self.text_size(TextSize::Large.rems(cx))
    }

    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui_sm` for smaller text.
    fn text_ui(self, cx: &WindowContext) -> Self {
        self.text_size(TextSize::default().rems(cx))
    }

    /// The small size for UI text.
    ///
    /// `0.75rem` or `12px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_sm(self, cx: &WindowContext) -> Self {
        self.text_size(TextSize::Small.rems(cx))
    }

    /// The extra small size for UI text.
    ///
    /// `0.625rem` or `10px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_xs(self, cx: &WindowContext) -> Self {
        self.text_size(TextSize::XSmall.rems(cx))
    }

    /// The font size for buffer text.
    ///
    /// Retrieves the default font size, or the user's custom font size if set.
    ///
    /// This should only be used for text that is displayed in a buffer,
    /// or other places that text needs to match the user's buffer font size.
    fn text_buffer(self, cx: &mut WindowContext) -> Self {
        let settings = ThemeSettings::get_global(cx);
        self.text_size(settings.buffer_font_size(cx))
    }
}

impl<E: Styled> StyledTypography for E {}

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
    UI,
    /// The `buffer_font_size` set by the user.
    Editor,
    // TODO: The terminal settings will need to be passed to
    // ThemeSettings before we can enable this.
    //// The `terminal.font_size` set by the user.
    // Terminal,
}

impl TextSize {
    pub fn rems(self, cx: &WindowContext) -> Rems {
        let theme_settings = ThemeSettings::get_global(cx);

        match self {
            Self::Large => rems_from_px(16.),
            Self::Default => rems_from_px(14.),
            Self::Small => rems_from_px(12.),
            Self::XSmall => rems_from_px(10.),
            Self::UI => rems_from_px(theme_settings.ui_font_size.into()),
            Self::Editor => rems_from_px(theme_settings.buffer_font_size.into()),
        }
    }
}

/// The size of a [`Headline`] element
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum HeadlineSize {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

impl HeadlineSize {
    pub fn size(self) -> Rems {
        match self {
            // Based on the Major Second scale
            Self::XSmall => rems(0.88),
            Self::Small => rems(1.0),
            Self::Medium => rems(1.125),
            Self::Large => rems(1.27),
            Self::XLarge => rems(1.43),
        }
    }

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

#[derive(IntoElement)]
pub struct Headline {
    size: HeadlineSize,
    text: SharedString,
    color: Color,
}

impl RenderOnce for Headline {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();

        div()
            .font(ui_font)
            .line_height(self.size.line_height())
            .text_size(self.size.size())
            .text_color(cx.theme().colors().text)
            .child(self.text)
    }
}

impl Headline {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            size: HeadlineSize::default(),
            text: text.into(),
            color: Color::default(),
        }
    }

    pub fn size(mut self, size: HeadlineSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}
