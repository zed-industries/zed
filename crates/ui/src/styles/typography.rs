use std::ops::Range;

use gpui::{
    div, px, rems, HighlightStyle, InteractiveText, IntoElement, ParentElement, Pixels, Rems,
    RenderOnce, SharedString, Styled, StyledText, TextStyle, UnderlineStyle, WindowContext,
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

    pub fn px(self, cx: &WindowContext) -> Pixels {
        self.rems(cx).to_pixels(cx.rem_size())
    }

    pub fn line_height(self, cx: &WindowContext, ratio: f32) -> Rems {
        self.rems(cx) * ratio
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
}

impl Headline {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            size: HeadlineSize::default(),
            text: text.into(),
        }
    }

    pub fn size(mut self, size: HeadlineSize) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for Headline {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let ui_font = ThemeSettings::get_global(cx).ui_font.family.clone();

        div()
            .font_family(ui_font)
            .line_height(self.size.line_height())
            .text_size(self.size.size())
            .text_color(cx.theme().colors().text)
            .child(self.text)
    }
}

#[derive(Debug, Clone)]
pub enum Font {
    UI,
    Editor,
}

impl Font {
    pub fn font(self, cx: &WindowContext) -> gpui::Font {
        match self {
            Self::UI => ThemeSettings::get_global(cx).ui_font.clone(),
            Self::Editor => ThemeSettings::get_global(cx).buffer_font.clone(),
        }
    }

    pub fn family(self, cx: &WindowContext) -> SharedString {
        match self {
            Self::UI => ThemeSettings::get_global(cx).ui_font.family.clone(),
            Self::Editor => ThemeSettings::get_global(cx).buffer_font.family.clone(),
        }
    }

    pub fn size(self, cx: &WindowContext) -> Rems {
        match self {
            Self::UI => TextSize::UI.rems(cx),
            Self::Editor => TextSize::Editor.rems(cx),
        }
    }
}

#[derive(IntoElement)]
pub struct Link {
    text: String,
    font: Font,
    uri: String,
    tooltip: Option<SharedString>,
    color: Color,
}

impl Link {
    pub fn new(text: String, uri: String) -> Self {
        Self {
            text,
            font: Font::UI,
            uri: uri.into(),
            tooltip: None,
            color: Color::Default,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }
}

impl RenderOnce for Link {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme_settings = ThemeSettings::get_global(cx);
        let font_size = theme_settings.ui_font_size;
        let font = self.font.font(cx);

        let text_style = TextStyle {
            color: self.color.color(cx),
            font_family: font.family,
            font_size: font_size.into(),
            ..Default::default()
        };

        let highlight_style = HighlightStyle {
            color: Some(self.color.color(cx)),
            underline: Some(UnderlineStyle {
                thickness: px(1.0),
                color: Some(self.color.color(cx)),
                wavy: false,
            }),
            ..Default::default()
        };

        let len = self.text.len();

        let link_range = Range { start: 0, end: len };

        let styled_text = StyledText::new(self.text)
            .with_highlights(&text_style, vec![(link_range.clone(), highlight_style)]);

        InteractiveText::new("link", styled_text).on_click(vec![link_range], {
            move |_, cx| {
                let url = &self.uri;
                if url.starts_with("http") {
                    cx.open_url(url);
                }
            }
        })
    }
}
