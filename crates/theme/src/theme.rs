#![deny(missing_docs)]

//! # Theme
//!
//! This crate provides the theme system for Zed.
//!
//! ## Overview
//!
//! A theme is a collection of colors used to build a consistent appearance for UI components across the application.

mod default_colors;
mod fallback_themes;
mod font_family_cache;
mod registry;
mod scale;
mod schema;
mod settings;
mod styles;

use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
pub use default_colors::*;
pub use font_family_cache::*;
pub use registry::*;
pub use scale::*;
pub use schema::*;
pub use settings::*;
pub use styles::*;

use gpui::{
    px, AppContext, AssetSource, HighlightStyle, Hsla, Pixels, Refineable, SharedString,
    WindowAppearance, WindowBackgroundAppearance,
};
use serde::Deserialize;
use uuid::Uuid;

/// Defines window border radius for platforms that use client side decorations.
pub const CLIENT_SIDE_DECORATION_ROUNDING: Pixels = px(10.0);
/// Defines window shadow size for platforms that use client side decorations.
pub const CLIENT_SIDE_DECORATION_SHADOW: Pixels = px(10.0);

/// The appearance of the theme.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum Appearance {
    /// A light appearance.
    Light,
    /// A dark appearance.
    Dark,
}

impl Appearance {
    /// Returns whether the appearance is light.
    pub fn is_light(&self) -> bool {
        match self {
            Self::Light => true,
            Self::Dark => false,
        }
    }
}

impl From<WindowAppearance> for Appearance {
    fn from(value: WindowAppearance) -> Self {
        match value {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
        }
    }
}

/// Which themes should be loaded. This is used primarlily for testing.
pub enum LoadThemes {
    /// Only load the base theme.
    ///
    /// No user themes will be loaded.
    JustBase,

    /// Load all of the built-in themes.
    All(Box<dyn AssetSource>),
}

/// Initialize the theme system.
pub fn init(themes_to_load: LoadThemes, cx: &mut AppContext) {
    let (assets, load_user_themes) = match themes_to_load {
        LoadThemes::JustBase => (Box::new(()) as Box<dyn AssetSource>, false),
        LoadThemes::All(assets) => (assets, true),
    };
    ThemeRegistry::set_global(assets, cx);

    if load_user_themes {
        ThemeRegistry::global(cx).load_bundled_themes();
    }

    ThemeSettings::register(cx);
    FontFamilyCache::init_global(cx);

    let mut prev_buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
    cx.observe_global::<SettingsStore>(move |cx| {
        let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
        if buffer_font_size != prev_buffer_font_size {
            prev_buffer_font_size = buffer_font_size;
            reset_buffer_font_size(cx);
        }
    })
    .detach();
}

/// Implementing this trait allows accessing the active theme.
pub trait ActiveTheme {
    /// Returns the active theme.
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &Arc<Theme> {
        &ThemeSettings::get_global(self).active_theme
    }
}

/// A theme family is a grouping of themes under a single name.
///
/// For example, the "One" theme family contains the "One Light" and "One Dark" themes.
///
/// It can also be used to package themes with many variants.
///
/// For example, the "Atelier" theme family contains "Cave", "Dune", "Estuary", "Forest", "Heath", etc.
pub struct ThemeFamily {
    /// The unique identifier for the theme family.
    pub id: String,
    /// The name of the theme family. This will be displayed in the UI, such as when adding or removing a theme family.
    pub name: SharedString,
    /// The author of the theme family.
    pub author: SharedString,
    /// The [Theme]s in the family.
    pub themes: Vec<Theme>,
    /// The color scales used by the themes in the family.
    /// Note: This will be removed in the future.
    pub scales: ColorScales,
}

impl ThemeFamily {
    // This is on ThemeFamily because we will have variables here we will need
    // in the future to resolve @references.
    /// Refines ThemeContent into a theme, merging it's contents with the base theme.
    pub fn refine_theme(&self, theme: &ThemeContent) -> Theme {
        let appearance = match theme.appearance {
            AppearanceContent::Light => Appearance::Light,
            AppearanceContent::Dark => Appearance::Dark,
        };

        let mut refined_theme_colors = match theme.appearance {
            AppearanceContent::Light => ThemeColors::light(),
            AppearanceContent::Dark => ThemeColors::dark(),
        };
        refined_theme_colors.refine(&theme.style.theme_colors_refinement());

        let mut refined_status_colors = match theme.appearance {
            AppearanceContent::Light => StatusColors::light(),
            AppearanceContent::Dark => StatusColors::dark(),
        };
        refined_status_colors.refine(&theme.style.status_colors_refinement());

        let mut refined_player_colors = match theme.appearance {
            AppearanceContent::Light => PlayerColors::light(),
            AppearanceContent::Dark => PlayerColors::dark(),
        };
        refined_player_colors.merge(&theme.style.players);

        let mut refined_accent_colors = match theme.appearance {
            AppearanceContent::Light => AccentColors::light(),
            AppearanceContent::Dark => AccentColors::dark(),
        };
        refined_accent_colors.merge(&theme.style.accents);

        let syntax_highlights = theme
            .style
            .syntax
            .iter()
            .map(|(syntax_token, highlight)| {
                (
                    syntax_token.clone(),
                    HighlightStyle {
                        color: highlight
                            .color
                            .as_ref()
                            .and_then(|color| try_parse_color(color).ok()),
                        background_color: highlight
                            .background_color
                            .as_ref()
                            .and_then(|color| try_parse_color(color).ok()),
                        font_style: highlight.font_style.map(Into::into),
                        font_weight: highlight.font_weight.map(Into::into),
                        ..Default::default()
                    },
                )
            })
            .collect::<Vec<_>>();
        let syntax_theme = SyntaxTheme::merge(Arc::new(SyntaxTheme::default()), syntax_highlights);

        let window_background_appearance = theme
            .style
            .window_background_appearance
            .map(Into::into)
            .unwrap_or_default();

        Theme {
            id: uuid::Uuid::new_v4().to_string(),
            name: theme.name.clone().into(),
            appearance,
            styles: ThemeStyles {
                system: SystemColors::default(),
                window_background_appearance,
                accents: refined_accent_colors,
                colors: refined_theme_colors,
                status: refined_status_colors,
                player: refined_player_colors,
                syntax: syntax_theme,
            },
        }
    }
}

/// Refines a [ThemeFamilyContent] and it's [ThemeContent]s into a [ThemeFamily].
pub fn refine_theme_family(theme_family_content: ThemeFamilyContent) -> ThemeFamily {
    let id = Uuid::new_v4().to_string();
    let name = theme_family_content.name.clone();
    let author = theme_family_content.author.clone();

    let mut theme_family = ThemeFamily {
        id: id.clone(),
        name: name.clone().into(),
        author: author.clone().into(),
        themes: vec![],
        scales: default_color_scales(),
    };

    let refined_themes = theme_family_content
        .themes
        .iter()
        .map(|theme_content| theme_family.refine_theme(theme_content))
        .collect();

    theme_family.themes = refined_themes;

    theme_family
}

/// A theme is the primary mechanism for defining the appearance of the UI.
#[derive(Clone, PartialEq)]
pub struct Theme {
    /// The unique identifier for the theme.
    pub id: String,
    /// The name of the theme.
    pub name: SharedString,
    /// The appearance of the theme (light or dark).
    pub appearance: Appearance,
    /// The colors and other styles for the theme.
    pub styles: ThemeStyles,
}

impl Theme {
    /// Returns the [`SystemColors`] for the theme.
    #[inline(always)]
    pub fn system(&self) -> &SystemColors {
        &self.styles.system
    }

    /// Returns the [`AccentColors`] for the theme.
    #[inline(always)]
    pub fn accents(&self) -> &AccentColors {
        &self.styles.accents
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

    /// Returns the [`Appearance`] for the theme.
    #[inline(always)]
    pub fn appearance(&self) -> Appearance {
        self.appearance
    }

    /// Returns the [`WindowBackgroundAppearance`] for the theme.
    #[inline(always)]
    pub fn window_background_appearance(&self) -> WindowBackgroundAppearance {
        self.styles.window_background_appearance
    }
}

/// Compounds a color with an alpha value.
/// TODO: Replace this with a method on Hsla.
pub fn color_alpha(color: Hsla, alpha: f32) -> Hsla {
    let mut color = color;
    color.a = alpha;
    color
}
