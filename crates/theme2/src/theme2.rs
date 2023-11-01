mod colors;
mod default_colors;
mod default_theme;
mod registry;
mod scale;
mod settings;
mod syntax;

pub use colors::*;
pub use default_colors::*;
pub use default_theme::*;
pub use registry::*;
pub use scale::*;
pub use settings::*;
pub use syntax::*;

use gpui2::{AppContext, Hsla, SharedString};
use settings2::Settings;

#[derive(Debug, Clone, PartialEq)]
pub enum Appearance {
    Light,
    Dark,
}

pub fn init(cx: &mut AppContext) {
    cx.set_global(ThemeRegistry::default());
    ThemeSettings::register(cx);
}

pub trait ActiveTheme {
    fn theme(&self) -> &ThemeVariant;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &ThemeVariant {
        &ThemeSettings::get_global(self).active_theme
    }
}

pub struct ThemeFamily {
    #[allow(dead_code)]
    pub(crate) id: String,
    pub name: SharedString,
    pub author: SharedString,
    pub themes: Vec<ThemeVariant>,
    pub scales: ColorScales,
}

impl ThemeFamily {}

pub struct ThemeVariant {
    #[allow(dead_code)]
    pub(crate) id: String,
    pub name: SharedString,
    pub appearance: Appearance,
    pub styles: ThemeStyles,
}

impl ThemeVariant {
    /// Returns the [`ThemeColors`] for the theme.
    #[inline(always)]
    pub fn colors(&self) -> &ThemeColors {
        &self.styles.colors
    }

    /// Returns the [`SyntaxTheme`] for the theme.
    #[inline(always)]
    pub fn syntax(&self) -> &SyntaxTheme {
        &self.styles.syntax
    }

    /// Returns the color for the syntax node with the given name.
    #[inline(always)]
    pub fn syntax_color(&self, name: &str) -> Hsla {
        self.syntax().color(name)
    }
}
