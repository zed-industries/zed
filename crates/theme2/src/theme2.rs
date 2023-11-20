mod default_colors;
mod default_theme;
mod one_themes;
mod registry;
mod scale;
mod settings;
mod styles;
#[cfg(not(feature = "importing-themes"))]
mod themes;
mod user_theme;

use std::sync::Arc;

use ::settings::Settings;
pub use default_colors::*;
pub use default_theme::*;
pub use registry::*;
pub use scale::*;
pub use settings::*;
pub use styles::*;
#[cfg(not(feature = "importing-themes"))]
pub use themes::*;
pub use user_theme::*;

use gpui::{AppContext, Hsla, SharedString};
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum Appearance {
    Light,
    Dark,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoadThemes {
    /// Only load the base theme.
    ///
    /// No user themes will be loaded.
    JustBase,

    /// Load all of the built-in themes.
    All,
}

pub fn init(themes_to_load: LoadThemes, cx: &mut AppContext) {
    cx.set_global(ThemeRegistry::default());

    match themes_to_load {
        LoadThemes::JustBase => (),
        LoadThemes::All => cx.global_mut::<ThemeRegistry>().load_user_themes(),
    }

    ThemeSettings::register(cx);
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &Arc<Theme> {
        &ThemeSettings::get_global(self).active_theme
    }
}

pub struct ThemeFamily {
    pub id: String,
    pub name: SharedString,
    pub author: SharedString,
    pub themes: Vec<Theme>,
    pub scales: ColorScales,
}

impl ThemeFamily {}

pub struct Theme {
    pub id: String,
    pub name: SharedString,
    pub appearance: Appearance,
    pub styles: ThemeStyles,
}

impl Theme {
    /// Returns the [`SystemColors`] for the theme.
    #[inline(always)]
    pub fn system(&self) -> &SystemColors {
        &self.styles.system
    }

    /// Returns the [`PlayerColors`] for the theme.
    #[inline(always)]
    pub fn players(&self) -> &PlayerColors {
        &self.styles.player
    }

    /// Returns the [`ThemeColors`] for the theme.
    #[inline(always)]
    pub fn colors(&self) -> &ThemeColors {
        &self.styles.colors
    }

    /// Returns the [`SyntaxTheme`] for the theme.
    #[inline(always)]
    pub fn syntax(&self) -> &Arc<SyntaxTheme> {
        &self.styles.syntax
    }

    /// Returns the [`StatusColors`] for the theme.
    #[inline(always)]
    pub fn status(&self) -> &StatusColors {
        &self.styles.status
    }

    /// Returns the color for the syntax node with the given name.
    #[inline(always)]
    pub fn syntax_color(&self, name: &str) -> Hsla {
        self.syntax().color(name)
    }

    /// Returns the [`DiagnosticStyle`] for the theme.
    #[inline(always)]
    pub fn diagnostic_style(&self) -> DiagnosticStyle {
        DiagnosticStyle {
            error: self.status().error,
            warning: self.status().warning,
            info: self.status().info,
            hint: self.status().info,
            ignored: self.status().ignored,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DiagnosticStyle {
    pub error: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
    pub hint: Hsla,
    pub ignored: Hsla,
}

#[cfg(feature = "stories")]
mod story;
#[cfg(feature = "stories")]
pub use story::*;
