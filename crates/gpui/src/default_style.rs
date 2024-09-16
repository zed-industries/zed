use crate::{rgb, Rgba, WindowAppearance};

/// The default style for GPUI elements.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DefaultStyle {
    /// The default colors for the style.
    pub color: DefaultColors,
}

impl Default for DefaultStyle {
    fn default() -> Self {
        Self {
            color: DefaultColors::light(),
        }
    }
}

/// Returns the default style for the given appearance.
pub fn default_style(appearance: WindowAppearance) -> DefaultStyle {
    DefaultStyle {
        color: colors(appearance.into()),
    }
}

/// Returns the default colors for the given appearance.
pub fn colors(appearance: WindowAppearance) -> DefaultColors {
    match appearance {
        WindowAppearance::Light | WindowAppearance::VibrantLight => DefaultColors::light(),
        WindowAppearance::Dark | WindowAppearance::VibrantDark => DefaultColors::dark(),
    }
}

/// Returns a list of all the colors in the given appearance.
pub fn colors_iter(appearance: WindowAppearance) -> Vec<Rgba> {
    let colors = colors(appearance);
    vec![
        colors.background,
        colors.background_activated,
        colors.background_selected,
        colors.border,
        colors.border_deemphasized,
        colors.border_focused,
        colors.container,
        colors.foreground,
        colors.foreground_deemphasized,
        colors.foreground_disabled,
        colors.foreground_placeholder,
        colors.foreground_selected,
        colors.separator,
    ]
}

/// The default colors for GPUI components.
///
/// NOTE: Default colors are in active development and will
/// likely change frequently, including breaking changes,
/// until stabalized. Use with caution!
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DefaultColors {
    /// Default color for backgrounds.
    pub background: Rgba,
    /// Default color for activated backgrounds, such as when a button is pressed.
    pub background_activated: Rgba,
    /// Default color for selected backgrounds, such as when a list item is selected,
    /// or a checkbox is checked.
    pub background_selected: Rgba,
    /// Default color for borders.
    pub border: Rgba,
    /// Default color for deemphasized borders.
    pub border_deemphasized: Rgba,
    /// Default color for focused borders.
    pub border_focused: Rgba,
    /// Default color for containers.
    pub container: Rgba,
    /// Default color for text, icons, and other foreground elements.
    pub foreground: Rgba,
    /// Default color for deemphasized text, icons, and other foreground elements.
    pub foreground_deemphasized: Rgba,
    /// Default color for disabled text, icons, and other foreground elements.
    pub foreground_disabled: Rgba,
    /// Default color for placeholder text, icons, and other foreground elements.
    pub foreground_placeholder: Rgba,
    /// Default color for selected text, icons, and other foreground elements.
    pub foreground_selected: Rgba,
    /// Default color for separators.
    pub separator: Rgba,
}

impl Default for DefaultColors {
    fn default() -> Self {
        Self::light()
    }
}

impl DefaultColors {
    /// Returns the default dark colors.
    pub fn dark() -> Self {
        Self {
            background: rgb(0x222222),
            background_activated: rgb(0x4B4B4B),
            background_selected: rgb(0x2457ca),
            border: rgb(0x000000),
            border_deemphasized: rgb(0x313131),
            border_focused: rgb(0x316e99),
            container: rgb(0x262626),
            foreground: rgb(0xffffff),
            foreground_deemphasized: rgb(0xAEAEAE),
            foreground_disabled: rgb(0x565656),
            foreground_placeholder: rgb(0x6e6f6f),
            foreground_selected: rgb(0xffffff),
            separator: rgb(0x3d3d3d),
        }
    }

    /// Returns the default light colors.
    pub fn light() -> Self {
        Self {
            background: rgb(0xffffff),
            background_activated: rgb(0x0f0f0f),
            background_selected: rgb(0x0164e1),
            border: rgb(0xd9d9d9),
            border_deemphasized: rgb(0xe6e6e6),
            border_focused: rgb(0x85acf4),
            container: rgb(0xf4f5f5),
            foreground: rgb(0x252525),
            foreground_deemphasized: rgb(0x7b7b7b),
            foreground_disabled: rgb(0xb0b0b0),
            foreground_placeholder: rgb(0xababab),
            foreground_selected: rgb(0xffffff),
            separator: rgb(0xe6e6e6),
        }
    }
}
