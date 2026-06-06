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
mod styles;
mod theme_settings_provider;
mod ui_density;
mod window_theme;

use std::sync::Arc;

use gpui::BorrowAppContext;
use gpui::Global;
use gpui::{
    App, AssetSource, Hsla, Pixels, SharedString, WindowAppearance, WindowBackgroundAppearance, px,
};
use serde::Deserialize;

pub use crate::default_colors::*;
pub use crate::fallback_themes::{apply_status_color_defaults, apply_theme_color_defaults};
pub use crate::font_family_cache::*;
pub use crate::icon_theme::*;
pub use crate::icon_theme_schema::*;
pub use crate::registry::*;
pub use crate::scale::*;
pub use crate::schema::*;
pub use crate::styles::*;
pub use crate::theme_settings_provider::*;
pub use crate::ui_density::*;
pub use crate::window_theme::*;

/// The name of the default dark theme.
pub const DEFAULT_DARK_THEME: &str = "One Dark";

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

/// Initialize the theme system with default themes.
///
/// This sets up the [`ThemeRegistry`], [`FontFamilyCache`], [`SystemAppearance`],
/// and [`GlobalTheme`] with the default dark theme. It does NOT load bundled
/// themes from JSON or integrate with settings — use `theme_settings::init` for that.
pub fn init(themes_to_load: LoadThemes, cx: &mut App) {
    SystemAppearance::init(cx);
    let assets = match themes_to_load {
        LoadThemes::JustBase => Box::new(()) as Box<dyn AssetSource>,
        LoadThemes::All(assets) => assets,
    };
    ThemeRegistry::set_global(assets, cx);
    FontFamilyCache::init_global(cx);

    let themes = ThemeRegistry::default_global(cx);
    let theme = themes.get(DEFAULT_DARK_THEME).unwrap_or_else(|_| {
        themes
            .list()
            .into_iter()
            .next()
            .map(|m| themes.get(&m.name).unwrap())
            .unwrap()
    });
    let icon_theme = themes.default_icon_theme().unwrap();
    cx.set_global(GlobalTheme::new(theme, icon_theme));

    // Per-window theming: a registry of user-chosen, per-window theme overrides
    // and a draw hook that swaps the active theme into place at the start of each
    // window's render pass. Windows without an override render the configured
    // theme.
    cx.set_global(WindowThemeOverrides::default());
    cx.observe_window_draw(|window, cx| {
        let window_id = window.window_handle().window_id();
        WindowThemeOverrides::apply_for_window(cx, window_id);
    })
    .detach();
    // Drop a closed window's override so the in-memory map doesn't grow for the
    // lifetime of the session. The persisted override (keyed by workspace) is
    // untouched and still restores on reopen.
    cx.on_window_closed(|cx, window_id| {
        WindowThemeOverrides::clear(cx, window_id);
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

/// The appearance of the system.
#[derive(Debug, Clone, Copy)]
pub struct SystemAppearance(pub Appearance);

impl std::ops::Deref for SystemAppearance {
    type Target = Appearance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SystemAppearance {
    fn default() -> Self {
        Self(Appearance::Dark)
    }
}

#[derive(Default)]
struct GlobalSystemAppearance(SystemAppearance);

impl std::ops::DerefMut for GlobalSystemAppearance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for GlobalSystemAppearance {
    type Target = SystemAppearance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Global for GlobalSystemAppearance {}

impl SystemAppearance {
    /// Initializes the [`SystemAppearance`] for the application.
    pub fn init(cx: &mut App) {
        *cx.default_global::<GlobalSystemAppearance>() =
            GlobalSystemAppearance(SystemAppearance(cx.window_appearance().into()));
    }

    /// Returns the global [`SystemAppearance`].
    pub fn global(cx: &App) -> Self {
        cx.global::<GlobalSystemAppearance>().0
    }

    /// Returns a mutable reference to the global [`SystemAppearance`].
    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<GlobalSystemAppearance>()
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

/// Deserializes an icon theme from the given bytes.
pub fn deserialize_icon_theme(bytes: &[u8]) -> anyhow::Result<IconThemeFamilyContent> {
    let icon_theme_family: IconThemeFamilyContent = serde_json_lenient::from_slice(bytes)?;

    Ok(icon_theme_family)
}

/// The active theme.
pub struct GlobalTheme {
    /// The theme used for the render pass currently in progress. The per-window
    /// theming hook swaps this at the start of each window's draw via
    /// [`GlobalTheme::set_active_theme`]; for windows without an override it
    /// equals `configured_theme`.
    theme: Arc<Theme>,
    /// The app-wide theme configured via settings. Used as the fallback for
    /// windows that have no per-window override, and never swapped per frame.
    configured_theme: Arc<Theme>,
    icon_theme: Arc<IconTheme>,
}
impl Global for GlobalTheme {}

impl GlobalTheme {
    /// Creates a new [`GlobalTheme`] with the given theme and icon theme. The
    /// given theme becomes both the active and the configured theme.
    pub fn new(theme: Arc<Theme>, icon_theme: Arc<IconTheme>) -> Self {
        Self {
            configured_theme: theme.clone(),
            theme,
            icon_theme,
        }
    }

    /// Updates the app-wide configured theme (and the active theme) and notifies
    /// observers. Called when the theme settings change.
    pub fn update_theme(cx: &mut App, theme: Arc<Theme>) {
        cx.update_global::<Self, _>(|this, _| {
            this.configured_theme = theme.clone();
            this.theme = theme;
        });
    }

    /// Sets the theme for the render pass currently in progress *without*
    /// notifying observers. Used by the per-window theming draw hook every frame;
    /// notifying here would re-invalidate the window and cause an unbounded
    /// redraw loop. See [`gpui::App::update_global_quietly`].
    pub fn set_active_theme(cx: &mut App, theme: Arc<Theme>) {
        cx.update_global_quietly::<Self, _>(|this, _| this.theme = theme);
    }

    /// Updates the active icon theme.
    pub fn update_icon_theme(cx: &mut App, icon_theme: Arc<IconTheme>) {
        cx.update_global::<Self, _>(|this, _| this.icon_theme = icon_theme);
    }

    /// Returns the active theme (the theme for the window currently drawing).
    pub fn theme(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().theme
    }

    /// Returns the app-wide configured theme — the fallback used for windows
    /// without a per-window override.
    pub fn configured_theme(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().configured_theme
    }

    /// Returns the active icon theme.
    pub fn icon_theme(cx: &App) -> &Arc<IconTheme> {
        &cx.global::<Self>().icon_theme
    }
}
