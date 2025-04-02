use crate::{Hsla, Rgba, WindowAppearance, rgb};

/// The appearance of the base GPUI colors, used to style GPUI elements
///
/// Varies based on the system's current [`WindowAppearance`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DefaultThemeAppearance {
    /// Use the set of colors for light appearances.
    #[default]
    Light,
    /// Use the set of colors for dark appearances.
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

/// Returns the default colors for the given appearance.
pub fn colors(appearance: DefaultThemeAppearance) -> DefaultColors {
    match appearance {
        DefaultThemeAppearance::Light => DefaultColors::light(),
        DefaultThemeAppearance::Dark => DefaultColors::dark(),
    }
}

/// A collection of colors.
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
    /// Returns the default dark colors.
    pub fn dark() -> Self {
        Self {
            text: rgb(0xffffff),
            selected_text: rgb(0xffffff),
            disabled: rgb(0x565656),
            selected: rgb(0x2457ca),
            background: rgb(0x222222),
            border: rgb(0x000000),
            separator: rgb(0xd9d9d9),
            container: rgb(0x262626),
        }
    }

    /// Returns the default light colors.
    pub fn light() -> Self {
        Self {
            text: rgb(0x252525),
            selected_text: rgb(0xffffff),
            background: rgb(0xffffff),
            disabled: rgb(0xb0b0b0),
            selected: rgb(0x2a63d9),
            border: rgb(0xd9d9d9),
            separator: rgb(0xe6e6e6),
            container: rgb(0xf4f5f5),
        }
    }
}

/// A default GPUI color.
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
    /// Returns the RGBA color for the given color type.
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

    /// Returns the HSLA color for the given color type.
    pub fn hsla(&self, colors: &DefaultColors) -> Hsla {
        self.color(colors).into()
    }
}
