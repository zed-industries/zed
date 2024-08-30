use crate::{rgb, Hsla, Rgba, WindowAppearance};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// The appearance of the base gpui colors, used to style gpui elements
///
/// Varies based on the system's current [WindowAppearance].
pub enum DefaultThemeAppearance {
    #[default]
    /// Use the set of colors for light appearances
    Light,
    /// Use the set of colors for dark appearances
    Dark,
}

impl From<WindowAppearance> for DefaultThemeAppearance {
    fn from(appearance: WindowAppearance) -> Self {
        match appearance {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
        }
    }
}

/// Get the default colors for the given appearance
pub fn colors(appearance: DefaultThemeAppearance) -> DefaultColors {
    match appearance {
        DefaultThemeAppearance::Light => DefaultColors::light(),
        DefaultThemeAppearance::Dark => DefaultColors::dark(),
    }
}

/// A collection of colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DefaultColors {
    text: Rgba,
    selected_text: Rgba,
    background: Rgba,
    disabled: Rgba,
    selected: Rgba,
    border: Rgba,
    separator: Rgba,
    container: Rgba,
}

impl DefaultColors {
    /// Get the default light colors
    pub fn dark() -> Self {
        Self {
            text: rgb(0xFFFFFF),
            selected_text: rgb(0xFFFFFF),
            disabled: rgb(0x565656),
            selected: rgb(0x2457CA),
            background: rgb(0x222222),
            border: rgb(0x000000),
            separator: rgb(0xD9D9D9),
            container: rgb(0x262626),
        }
    }

    /// Get the default dark colors
    pub fn light() -> Self {
        Self {
            text: rgb(0x252525),
            selected_text: rgb(0xFFFFFF),
            background: rgb(0xFFFFFF),
            disabled: rgb(0xB0B0B0),
            selected: rgb(0x2A63D9),
            border: rgb(0xD9D9D9),
            separator: rgb(0xE6E6E6),
            container: rgb(0xF4F5F5),
        }
    }
}

/// A default gpui color
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum DefaultColor {
    /// Text color
    Text,
    /// Selected text color
    SelectedText,
    /// Background color
    Background,
    /// Disabled color
    Disabled,
    /// Selected color
    Selected,
    /// Border color
    Border,
    /// Separator color
    Separator,
    /// Container color
    Container,
}
impl DefaultColor {
    /// Get the Rgb color for the given color type
    pub fn color(&self, colors: &DefaultColors) -> Rgba {
        match self {
            DefaultColor::Text => colors.text,
            DefaultColor::SelectedText => colors.selected_text,
            DefaultColor::Background => colors.background,
            DefaultColor::Disabled => colors.disabled,
            DefaultColor::Selected => colors.selected,
            DefaultColor::Border => colors.border,
            DefaultColor::Separator => colors.separator,
            DefaultColor::Container => colors.container,
        }
    }

    /// Get the Hsla color for the given color type
    pub fn hsla(&self, colors: &DefaultColors) -> Hsla {
        self.color(&colors).into()
    }
}
