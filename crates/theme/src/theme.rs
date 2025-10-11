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
mod icon_theme;
mod icon_theme_schema;
mod registry;
mod scale;
mod schema;
mod settings;
mod styles;

use std::path::Path;
use std::sync::Arc;

use ::settings::Settings;
use ::settings::SettingsStore;
use anyhow::Result;
use fallback_themes::apply_status_color_defaults;
use fs::Fs;
use gpui::BorrowAppContext;
use gpui::Global;
use gpui::{
    App, AssetSource, HighlightStyle, Hsla, Pixels, Refineable, SharedString, WindowAppearance,
    WindowBackgroundAppearance, px,
};
use serde::Deserialize;
use uuid::Uuid;

pub use crate::default_colors::*;
use crate::fallback_themes::apply_theme_color_defaults;
pub use crate::font_family_cache::*;
pub use crate::icon_theme::*;
pub use crate::icon_theme_schema::*;
pub use crate::registry::*;
pub use crate::scale::*;
pub use crate::schema::*;
pub use crate::settings::*;
pub use crate::styles::*;
pub use ::settings::{
    FontStyleContent, HighlightStyleContent, StatusColorsContent, ThemeColorsContent,
    ThemeStyleContent,
};

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

/// Which themes should be loaded. This is used primarily for testing.
pub enum LoadThemes {
    /// Only load the base theme.
    ///
    /// No user themes will be loaded.
    JustBase,

    /// Load all of the built-in themes.
    All(Box<dyn AssetSource>),
}

/// Initialize the theme system.
pub fn init(themes_to_load: LoadThemes, cx: &mut App) {
    SystemAppearance::init(cx);
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

    let theme = GlobalTheme::configured_theme(cx);
    let icon_theme = GlobalTheme::configured_icon_theme(cx);
    cx.set_global(GlobalTheme { theme, icon_theme });

    let settings = ThemeSettings::get_global(cx);

    let mut prev_buffer_font_size_settings = settings.buffer_font_size_settings();
    let mut prev_ui_font_size_settings = settings.ui_font_size_settings();
    let mut prev_agent_ui_font_size_settings = settings.agent_ui_font_size_settings();
    let mut prev_agent_buffer_font_size_settings = settings.agent_buffer_font_size_settings();
    let mut prev_theme_name = settings.theme.name(SystemAppearance::global(cx).0);
    let mut prev_icon_theme_name = settings.icon_theme.name(SystemAppearance::global(cx).0);
    let mut prev_theme_overrides = (
        settings.experimental_theme_overrides.clone(),
        settings.theme_overrides.clone(),
    );

    cx.observe_global::<SettingsStore>(move |cx| {
        let settings = ThemeSettings::get_global(cx);

        let buffer_font_size_settings = settings.buffer_font_size_settings();
        let ui_font_size_settings = settings.ui_font_size_settings();
        let agent_ui_font_size_settings = settings.agent_ui_font_size_settings();
        let agent_buffer_font_size_settings = settings.agent_buffer_font_size_settings();
        let theme_name = settings.theme.name(SystemAppearance::global(cx).0);
        let icon_theme_name = settings.icon_theme.name(SystemAppearance::global(cx).0);
        let theme_overrides = (
            settings.experimental_theme_overrides.clone(),
            settings.theme_overrides.clone(),
        );

        if buffer_font_size_settings != prev_buffer_font_size_settings {
            prev_buffer_font_size_settings = buffer_font_size_settings;
            reset_buffer_font_size(cx);
        }

        if ui_font_size_settings != prev_ui_font_size_settings {
            prev_ui_font_size_settings = ui_font_size_settings;
            reset_ui_font_size(cx);
        }

        if agent_ui_font_size_settings != prev_agent_ui_font_size_settings {
            prev_agent_ui_font_size_settings = agent_ui_font_size_settings;
            reset_agent_ui_font_size(cx);
        }

        if agent_buffer_font_size_settings != prev_agent_buffer_font_size_settings {
            prev_agent_buffer_font_size_settings = agent_buffer_font_size_settings;
            reset_agent_buffer_font_size(cx);
        }

        if theme_name != prev_theme_name || theme_overrides != prev_theme_overrides {
            prev_theme_name = theme_name;
            prev_theme_overrides = theme_overrides;
            GlobalTheme::reload_theme(cx);
        }

        if icon_theme_name != prev_icon_theme_name {
            prev_icon_theme_name = icon_theme_name;
            GlobalTheme::reload_icon_theme(cx);
        }
    })
    .detach();
}

/// Implementing this trait allows accessing the active theme.
pub trait ActiveTheme {
    /// Returns the active theme.
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        GlobalTheme::theme(self)
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

        let mut refined_status_colors = match theme.appearance {
            AppearanceContent::Light => StatusColors::light(),
            AppearanceContent::Dark => StatusColors::dark(),
        };
        let mut status_colors_refinement = status_colors_refinement(&theme.style.status);
        apply_status_color_defaults(&mut status_colors_refinement);
        refined_status_colors.refine(&status_colors_refinement);

        let mut refined_player_colors = match theme.appearance {
            AppearanceContent::Light => PlayerColors::light(),
            AppearanceContent::Dark => PlayerColors::dark(),
        };
        refined_player_colors.merge(&theme.style.players);

        let mut refined_theme_colors = match theme.appearance {
            AppearanceContent::Light => ThemeColors::light(),
            AppearanceContent::Dark => ThemeColors::dark(),
        };
        let mut theme_colors_refinement =
            theme_colors_refinement(&theme.style.colors, &status_colors_refinement);
        apply_theme_color_defaults(&mut theme_colors_refinement, &refined_player_colors);
        refined_theme_colors.refine(&theme_colors_refinement);

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
        id,
        name: name.into(),
        author: author.into(),
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
#[derive(Clone, Debug, PartialEq)]
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

    /// Darkens the color by reducing its lightness.
    /// The resulting lightness is clamped to ensure it doesn't go below 0.0.
    ///
    /// The first value darkens light appearance mode, the second darkens appearance dark mode.
    ///
    /// Note: This is a tentative solution and may be replaced with a more robust color system.
    pub fn darken(&self, color: Hsla, light_amount: f32, dark_amount: f32) -> Hsla {
        let amount = match self.appearance {
            Appearance::Light => light_amount,
            Appearance::Dark => dark_amount,
        };
        let mut hsla = color;
        hsla.l = (hsla.l - amount).max(0.0);
        hsla
    }
}

/// Asynchronously reads the user theme from the specified path.
pub async fn read_user_theme(theme_path: &Path, fs: Arc<dyn Fs>) -> Result<ThemeFamilyContent> {
    let bytes = fs.load_bytes(theme_path).await?;
    let theme_family: ThemeFamilyContent = serde_json_lenient::from_slice(&bytes)?;

    for theme in &theme_family.themes {
        if theme
            .style
            .colors
            .deprecated_scrollbar_thumb_background
            .is_some()
        {
            log::warn!(
                r#"Theme "{theme_name}" is using a deprecated style property: scrollbar_thumb.background. Use `scrollbar.thumb.background` instead."#,
                theme_name = theme.name
            )
        }
    }

    Ok(theme_family)
}

/// Asynchronously reads the icon theme from the specified path.
pub async fn read_icon_theme(
    icon_theme_path: &Path,
    fs: Arc<dyn Fs>,
) -> Result<IconThemeFamilyContent> {
    let bytes = fs.load_bytes(icon_theme_path).await?;
    let icon_theme_family: IconThemeFamilyContent = serde_json_lenient::from_slice(&bytes)?;

    Ok(icon_theme_family)
}

/// The active theme
pub struct GlobalTheme {
    theme: Arc<Theme>,
    icon_theme: Arc<IconTheme>,
}
impl Global for GlobalTheme {}

impl GlobalTheme {
    fn configured_theme(cx: &mut App) -> Arc<Theme> {
        let themes = ThemeRegistry::default_global(cx);
        let theme_settings = ThemeSettings::get_global(cx);
        let system_appearance = SystemAppearance::global(cx);

        let theme_name = theme_settings.theme.name(*system_appearance);

        let theme = match themes.get(&theme_name.0) {
            Ok(theme) => theme,
            Err(err) => {
                if themes.extensions_loaded() {
                    log::error!("{err}");
                }
                themes
                    .get(default_theme(*system_appearance))
                    // fallback for tests.
                    .unwrap_or_else(|_| themes.get(DEFAULT_DARK_THEME).unwrap())
            }
        };
        theme_settings.apply_theme_overrides(theme)
    }

    /// Reloads the current theme.
    ///
    /// Reads the [`ThemeSettings`] to know which theme should be loaded,
    /// taking into account the current [`SystemAppearance`].
    pub fn reload_theme(cx: &mut App) {
        let theme = Self::configured_theme(cx);
        cx.update_global::<Self, _>(|this, _| this.theme = theme);
        cx.refresh_windows();
    }

    fn configured_icon_theme(cx: &mut App) -> Arc<IconTheme> {
        let themes = ThemeRegistry::default_global(cx);
        let theme_settings = ThemeSettings::get_global(cx);
        let system_appearance = SystemAppearance::global(cx);

        let icon_theme_name = theme_settings.icon_theme.name(*system_appearance);

        match themes.get_icon_theme(&icon_theme_name.0) {
            Ok(theme) => theme,
            Err(err) => {
                if themes.extensions_loaded() {
                    log::error!("{err}");
                }
                themes.get_icon_theme(DEFAULT_ICON_THEME_NAME).unwrap()
            }
        }
    }

    /// Reloads the current icon theme.
    ///
    /// Reads the [`ThemeSettings`] to know which icon theme should be loaded,
    /// taking into account the current [`SystemAppearance`].
    pub fn reload_icon_theme(cx: &mut App) {
        let icon_theme = Self::configured_icon_theme(cx);
        cx.update_global::<Self, _>(|this, _| this.icon_theme = icon_theme);
        cx.refresh_windows();
    }

    /// the active theme
    pub fn theme(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().theme
    }

    /// the active icon theme
    pub fn icon_theme(cx: &App) -> &Arc<IconTheme> {
        &cx.global::<Self>().icon_theme
    }
}
