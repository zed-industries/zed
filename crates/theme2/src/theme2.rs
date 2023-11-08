mod colors;
mod default_colors;
mod default_theme;
mod players;
mod registry;
mod scale;
mod settings;
mod syntax;
mod themes;
mod user_theme;

use std::sync::Arc;

use ::settings::Settings;
pub use colors::*;
pub use default_colors::*;
pub use default_theme::*;
pub use players::*;
pub use registry::*;
pub use scale::*;
pub use settings::*;
pub use syntax::*;
pub use themes::*;
pub use user_theme::*;

use gpui::{AppContext, Hsla, SharedString};
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum Appearance {
    Light,
    Dark,
}

pub fn init(cx: &mut AppContext) {
    cx.set_global(ThemeRegistry::default());
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
    /// Returns the [`ThemeColors`] for the theme.
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

    /// Returns the [`GitStatusColors`] for the theme.
    #[inline(always)]
    pub fn git(&self) -> &GitStatusColors {
        &self.styles.git
    }

    /// Returns the color for the syntax node with the given name.
    #[inline(always)]
    pub fn syntax_color(&self, name: &str) -> Hsla {
        self.syntax().color(name)
    }

    /// Returns the [`StatusColors`] for the theme.
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

#[derive(Clone, Debug)]
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
