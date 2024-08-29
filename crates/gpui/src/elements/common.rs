use crate::{rgb, Hsla, Rgba, WindowAppearance};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// The appearance of the base gpui colors, used to style gpui elements
///
/// Varries based on the system's current [WindowAppearance].
pub enum DefaultThemeApperance {
    #[default]
    /// Use the set of colors for light appearances
    Light,
    /// Use the set of colors for dark appearances
    Dark,
}

impl From<WindowAppearance> for DefaultThemeApperance {
    fn from(appearance: WindowAppearance) -> Self {
        match appearance {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
        }
    }
}

/// Get the default colors for the given appearance
pub fn colors(appearance: DefaultThemeApperance) -> DefaultColors {
    match appearance {
        DefaultThemeApperance::Light => DefaultColors::light(),
        DefaultThemeApperance::Dark => DefaultColors::dark(),
    }
}

/// A collection of colors
pub struct DefaultColors {
    text: Rgba,
    background: Rgba,
    disabled: Rgba,
    selected: Rgba,
}

impl DefaultColors {
    /// Get the default light colors
    pub fn light() -> Self {
        Self {
            text: rgb(0xEAEAEA),
            disabled: rgb(0x565656),
            selected: rgb(0x2457CA),
            background: rgb(0x222222),
        }
    }

    /// Get the default dark colors
    pub fn dark() -> Self {
        Self {
            text: rgb(0x484848),
            background: rgb(0xFFFFFF),
            disabled: rgb(0xB0B0B0),
            selected: rgb(0x2A63D9),
        }
    }
}

/// A default gpui color
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum DefaultColor {
    /// Text color
    Text,
    /// Background color
    Background,
    /// Disabled color
    Disabled,
    /// Selected color
    Selected,
}

impl DefaultColors {
    /// Get the Rgb color for the given color type
    pub fn color(&self, color: DefaultColor) -> Rgba {
        match color {
            DefaultColor::Text => self.text,
            DefaultColor::Background => self.background,
            DefaultColor::Disabled => self.disabled,
            DefaultColor::Selected => self.selected,
        }
    }

    /// Get the Hsla color for the given color type
    pub fn hsla(&self, color: DefaultColor) -> Hsla {
        self.color(color).into()
    }
}
