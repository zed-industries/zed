use crate::fallback_themes::zed_default_dark;
use crate::{
    Appearance, DEFAULT_ICON_THEME_NAME, IconTheme, IconThemeNotFoundError, SyntaxTheme, Theme,
    ThemeNotFoundError, ThemeRegistry, ThemeStyleContent,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::{
    App, Context, Font, FontFallbacks, FontFeatures, FontStyle, FontWeight, Global, Pixels,
    Subscription, Window, px,
};
use refineable::Refineable;
use schemars::{
    JsonSchema,
    r#gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{Settings, SettingsJsonSchemaParams, SettingsSources, add_references_to_properties};
use std::sync::Arc;
use util::ResultExt as _;

const MIN_FONT_SIZE: Pixels = px(6.0);
const MIN_LINE_HEIGHT: f32 = 1.0;

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
)]

/// Specifies the density of the UI.
/// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
#[serde(rename_all = "snake_case")]
pub enum UiDensity {
    /// A denser UI with tighter spacing and smaller elements.
    #[serde(alias = "compact")]
    Compact,
    #[default]
    #[serde(alias = "default")]
    /// The default UI density.
    Default,
    #[serde(alias = "comfortable")]
    /// A looser UI with more spacing and larger elements.
    Comfortable,
}

impl UiDensity {
    /// The spacing ratio of a given density.
    /// TODO: Standardize usage throughout the app or remove
    pub fn spacing_ratio(self) -> f32 {
        match self {
            UiDensity::Compact => 0.75,
            UiDensity::Default => 1.0,
            UiDensity::Comfortable => 1.25,
        }
    }
}

impl From<String> for UiDensity {
    fn from(s: String) -> Self {
        match s.as_str() {
            "compact" => Self::Compact,
            "default" => Self::Default,
            "comfortable" => Self::Comfortable,
            _ => Self::default(),
        }
    }
}

impl From<UiDensity> for String {
    fn from(val: UiDensity) -> Self {
        match val {
            UiDensity::Compact => "compact".to_string(),
            UiDensity::Default => "default".to_string(),
            UiDensity::Comfortable => "comfortable".to_string(),
        }
    }
}

/// Customizable settings for the UI and theme system.
#[derive(Clone, PartialEq)]
pub struct ThemeSettings {
    /// The UI font size. Determines the size of text in the UI,
    /// as well as the size of a [gpui::Rems] unit.
    ///
    /// Changing this will impact the size of all UI elements.
    ui_font_size: Pixels,
    /// The font used for UI elements.
    pub ui_font: Font,
    /// The font size used for buffers, and the terminal.
    ///
    /// The terminal font size can be overridden using it's own setting.
    buffer_font_size: Pixels,
    /// The font used for buffers, and the terminal.
    ///
    /// The terminal font family can be overridden using it's own setting.
    pub buffer_font: Font,
    /// The line height for buffers, and the terminal.
    ///
    /// Changing this may affect the spacing of some UI elements.
    ///
    /// The terminal font family can be overridden using it's own setting.
    pub buffer_line_height: BufferLineHeight,
    /// The current theme selection.
    pub theme_selection: Option<ThemeSelection>,
    /// The active theme.
    pub active_theme: Arc<Theme>,
    /// Manual overrides for the active theme.
    ///
    /// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
    pub theme_overrides: Option<ThemeStyleContent>,
    /// The current icon theme selection.
    pub icon_theme_selection: Option<IconThemeSelection>,
    /// The active icon theme.
    pub active_icon_theme: Arc<IconTheme>,
    /// The density of the UI.
    /// Note: This setting is still experimental. See [this tracking issue](
    pub ui_density: UiDensity,
    /// The amount of fading applied to unnecessary code.
    pub unnecessary_code_fade: f32,
}

impl ThemeSettings {
    const DEFAULT_LIGHT_THEME: &'static str = "One Light";
    const DEFAULT_DARK_THEME: &'static str = "One Dark";

    /// Returns the name of the default theme for the given [`Appearance`].
    pub fn default_theme(appearance: Appearance) -> &'static str {
        match appearance {
            Appearance::Light => Self::DEFAULT_LIGHT_THEME,
            Appearance::Dark => Self::DEFAULT_DARK_THEME,
        }
    }

    /// Reloads the current theme.
    ///
    /// Reads the [`ThemeSettings`] to know which theme should be loaded,
    /// taking into account the current [`SystemAppearance`].
    pub fn reload_current_theme(cx: &mut App) {
        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        let system_appearance = SystemAppearance::global(cx);

        if let Some(theme_selection) = theme_settings.theme_selection.clone() {
            let mut theme_name = theme_selection.theme(*system_appearance);

            // If the selected theme doesn't exist, fall back to a default theme
            // based on the system appearance.
            let theme_registry = ThemeRegistry::global(cx);
            if let Err(err @ ThemeNotFoundError(_)) = theme_registry.get(theme_name) {
                if theme_registry.extensions_loaded() {
                    log::error!("{err}");
                }

                theme_name = Self::default_theme(*system_appearance);
            };

            if let Some(_theme) = theme_settings.switch_theme(theme_name, cx) {
                ThemeSettings::override_global(theme_settings, cx);
            }
        }
    }

    /// Reloads the current icon theme.
    ///
    /// Reads the [`ThemeSettings`] to know which icon theme should be loaded,
    /// taking into account the current [`SystemAppearance`].
    pub fn reload_current_icon_theme(cx: &mut App) {
        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        let system_appearance = SystemAppearance::global(cx);

        if let Some(icon_theme_selection) = theme_settings.icon_theme_selection.clone() {
            let mut icon_theme_name = icon_theme_selection.icon_theme(*system_appearance);

            // If the selected icon theme doesn't exist, fall back to the default theme.
            let theme_registry = ThemeRegistry::global(cx);
            if let Err(err @ IconThemeNotFoundError(_)) =
                theme_registry.get_icon_theme(icon_theme_name)
            {
                if theme_registry.extensions_loaded() {
                    log::error!("{err}");
                }

                icon_theme_name = DEFAULT_ICON_THEME_NAME;
            };

            if let Some(_theme) = theme_settings.switch_icon_theme(icon_theme_name, cx) {
                ThemeSettings::override_global(theme_settings, cx);
            }
        }
    }
}

/// The appearance of the system.
#[derive(Debug, Clone, Copy, Deref)]
pub struct SystemAppearance(pub Appearance);

impl Default for SystemAppearance {
    fn default() -> Self {
        Self(Appearance::Dark)
    }
}

#[derive(Deref, DerefMut, Default)]
struct GlobalSystemAppearance(SystemAppearance);

impl Global for GlobalSystemAppearance {}

impl SystemAppearance {
    /// Initializes the [`SystemAppearance`] for the application.
    pub fn init(cx: &mut App) {
        *cx.default_global::<GlobalSystemAppearance>() =
            GlobalSystemAppearance(SystemAppearance(cx.window_appearance().into()));
    }

    /// Returns the global [`SystemAppearance`].
    ///
    /// Inserts a default [`SystemAppearance`] if one does not yet exist.
    pub(crate) fn default_global(cx: &mut App) -> Self {
        cx.default_global::<GlobalSystemAppearance>().0
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

#[derive(Default)]
struct BufferFontSize(Pixels);

impl Global for BufferFontSize {}

#[derive(Default)]
pub(crate) struct UiFontSize(Pixels);

impl Global for UiFontSize {}

/// Represents the selection of a theme, which can be either static or dynamic.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum ThemeSelection {
    /// A static theme selection, represented by a single theme name.
    Static(#[schemars(schema_with = "theme_name_ref")] String),
    /// A dynamic theme selection, which can change based the [ThemeMode].
    Dynamic {
        /// The mode used to determine which theme to use.
        #[serde(default)]
        mode: ThemeMode,
        /// The theme to use for light mode.
        #[schemars(schema_with = "theme_name_ref")]
        light: String,
        /// The theme to use for dark mode.
        #[schemars(schema_with = "theme_name_ref")]
        dark: String,
    },
}

fn theme_name_ref(_: &mut SchemaGenerator) -> Schema {
    Schema::new_ref("#/definitions/ThemeName".into())
}

// TODO: Rename ThemeMode -> ThemeAppearanceMode
/// The mode use to select a theme.
///
/// `Light` and `Dark` will select their respective themes.
///
/// `System` will select the theme based on the system's appearance.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    /// Use the specified `light` theme.
    Light,

    /// Use the specified `dark` theme.
    Dark,

    /// Use the theme based on the system's appearance.
    #[default]
    System,
}

impl ThemeSelection {
    /// Returns the theme name for the selected [ThemeMode].
    pub fn theme(&self, system_appearance: Appearance) -> &str {
        match self {
            Self::Static(theme) => theme,
            Self::Dynamic { mode, light, dark } => match mode {
                ThemeMode::Light => light,
                ThemeMode::Dark => dark,
                ThemeMode::System => match system_appearance {
                    Appearance::Light => light,
                    Appearance::Dark => dark,
                },
            },
        }
    }

    /// Returns the [ThemeMode] for the [ThemeSelection].
    pub fn mode(&self) -> Option<ThemeMode> {
        match self {
            ThemeSelection::Static(_) => None,
            ThemeSelection::Dynamic { mode, .. } => Some(*mode),
        }
    }
}

fn icon_theme_name_ref(_: &mut SchemaGenerator) -> Schema {
    Schema::new_ref("#/definitions/IconThemeName".into())
}

/// Represents the selection of an icon theme, which can be either static or dynamic.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum IconThemeSelection {
    /// A static icon theme selection, represented by a single icon theme name.
    Static(#[schemars(schema_with = "icon_theme_name_ref")] String),
    /// A dynamic icon theme selection, which can change based on the [`ThemeMode`].
    Dynamic {
        /// The mode used to determine which theme to use.
        #[serde(default)]
        mode: ThemeMode,
        /// The icon theme to use for light mode.
        #[schemars(schema_with = "icon_theme_name_ref")]
        light: String,
        /// The icon theme to use for dark mode.
        #[schemars(schema_with = "icon_theme_name_ref")]
        dark: String,
    },
}

impl IconThemeSelection {
    /// Returns the icon theme name based on the given [`Appearance`].
    pub fn icon_theme(&self, system_appearance: Appearance) -> &str {
        match self {
            Self::Static(theme) => theme,
            Self::Dynamic { mode, light, dark } => match mode {
                ThemeMode::Light => light,
                ThemeMode::Dark => dark,
                ThemeMode::System => match system_appearance {
                    Appearance::Light => light,
                    Appearance::Dark => dark,
                },
            },
        }
    }

    /// Returns the [`ThemeMode`] for the [`IconThemeSelection`].
    pub fn mode(&self) -> Option<ThemeMode> {
        match self {
            IconThemeSelection::Static(_) => None,
            IconThemeSelection::Dynamic { mode, .. } => Some(*mode),
        }
    }
}

/// Settings for rendering text in UI and text buffers.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ThemeSettingsContent {
    /// The default font size for text in the UI.
    #[serde(default)]
    pub ui_font_size: Option<f32>,
    /// The name of a font to use for rendering in the UI.
    #[serde(default)]
    pub ui_font_family: Option<String>,
    /// The font fallbacks to use for rendering in the UI.
    #[serde(default)]
    #[schemars(default = "default_font_fallbacks")]
    pub ui_font_fallbacks: Option<Vec<String>>,
    /// The OpenType features to enable for text in the UI.
    #[serde(default)]
    #[schemars(default = "default_font_features")]
    pub ui_font_features: Option<FontFeatures>,
    /// The weight of the UI font in CSS units from 100 to 900.
    #[serde(default)]
    pub ui_font_weight: Option<f32>,
    /// The name of a font to use for rendering in text buffers.
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    /// The font fallbacks to use for rendering in text buffers.
    #[serde(default)]
    #[schemars(default = "default_font_fallbacks")]
    pub buffer_font_fallbacks: Option<Vec<String>>,
    /// The default font size for rendering in text buffers.
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    /// The weight of the editor font in CSS units from 100 to 900.
    #[serde(default)]
    pub buffer_font_weight: Option<f32>,
    /// The buffer's line height.
    #[serde(default)]
    pub buffer_line_height: Option<BufferLineHeight>,
    /// The OpenType features to enable for rendering in text buffers.
    #[serde(default)]
    #[schemars(default = "default_font_features")]
    pub buffer_font_features: Option<FontFeatures>,
    /// The name of the Zed theme to use.
    #[serde(default)]
    pub theme: Option<ThemeSelection>,
    /// The name of the icon theme to use.
    #[serde(default)]
    pub icon_theme: Option<IconThemeSelection>,

    /// UNSTABLE: Expect many elements to be broken.
    ///
    // Controls the density of the UI.
    #[serde(rename = "unstable.ui_density", default)]
    pub ui_density: Option<UiDensity>,

    /// How much to fade out unused code.
    #[serde(default)]
    pub unnecessary_code_fade: Option<f32>,

    /// EXPERIMENTAL: Overrides for the current theme.
    ///
    /// These values will override the ones on the current theme specified in `theme`.
    #[serde(rename = "experimental.theme_overrides", default)]
    pub theme_overrides: Option<ThemeStyleContent>,
}

fn default_font_features() -> Option<FontFeatures> {
    Some(FontFeatures::default())
}

fn default_font_fallbacks() -> Option<FontFallbacks> {
    Some(FontFallbacks::default())
}

impl ThemeSettingsContent {
    /// Sets the theme for the given appearance to the theme with the specified name.
    pub fn set_theme(&mut self, theme_name: String, appearance: Appearance) {
        if let Some(selection) = self.theme.as_mut() {
            let theme_to_update = match selection {
                ThemeSelection::Static(theme) => theme,
                ThemeSelection::Dynamic { mode, light, dark } => match mode {
                    ThemeMode::Light => light,
                    ThemeMode::Dark => dark,
                    ThemeMode::System => match appearance {
                        Appearance::Light => light,
                        Appearance::Dark => dark,
                    },
                },
            };

            *theme_to_update = theme_name.to_string();
        } else {
            self.theme = Some(ThemeSelection::Static(theme_name.to_string()));
        }
    }

    /// Sets the icon theme for the given appearance to the icon theme with the specified name.
    pub fn set_icon_theme(&mut self, icon_theme_name: String, appearance: Appearance) {
        if let Some(selection) = self.icon_theme.as_mut() {
            let icon_theme_to_update = match selection {
                IconThemeSelection::Static(theme) => theme,
                IconThemeSelection::Dynamic { mode, light, dark } => match mode {
                    ThemeMode::Light => light,
                    ThemeMode::Dark => dark,
                    ThemeMode::System => match appearance {
                        Appearance::Light => light,
                        Appearance::Dark => dark,
                    },
                },
            };

            *icon_theme_to_update = icon_theme_name.to_string();
        } else {
            self.icon_theme = Some(IconThemeSelection::Static(icon_theme_name.to_string()));
        }
    }

    /// Sets the mode for the theme.
    pub fn set_mode(&mut self, mode: ThemeMode) {
        if let Some(selection) = self.theme.as_mut() {
            match selection {
                ThemeSelection::Static(theme) => {
                    // If the theme was previously set to a single static theme,
                    // we don't know whether it was a light or dark theme, so we
                    // just use it for both.
                    self.theme = Some(ThemeSelection::Dynamic {
                        mode,
                        light: theme.clone(),
                        dark: theme.clone(),
                    });
                }
                ThemeSelection::Dynamic {
                    mode: mode_to_update,
                    ..
                } => *mode_to_update = mode,
            }
        } else {
            self.theme = Some(ThemeSelection::Dynamic {
                mode,
                light: ThemeSettings::DEFAULT_LIGHT_THEME.into(),
                dark: ThemeSettings::DEFAULT_DARK_THEME.into(),
            });
        }

        if let Some(selection) = self.icon_theme.as_mut() {
            match selection {
                IconThemeSelection::Static(icon_theme) => {
                    // If the icon theme was previously set to a single static
                    // theme, we don't know whether it was a light or dark
                    // theme, so we just use it for both.
                    self.icon_theme = Some(IconThemeSelection::Dynamic {
                        mode,
                        light: icon_theme.clone(),
                        dark: icon_theme.clone(),
                    });
                }
                IconThemeSelection::Dynamic {
                    mode: mode_to_update,
                    ..
                } => *mode_to_update = mode,
            }
        } else {
            self.icon_theme = Some(IconThemeSelection::Static(DEFAULT_ICON_THEME_NAME.into()));
        }
    }
}

/// The buffer's line height.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum BufferLineHeight {
    /// A less dense line height.
    #[default]
    Comfortable,
    /// The default line height.
    Standard,
    /// A custom line height.
    ///
    /// A line height of 1.0 is the height of the buffer's font size.
    Custom(f32),
}

impl BufferLineHeight {
    /// Returns the value of the line height.
    pub fn value(&self) -> f32 {
        match self {
            BufferLineHeight::Comfortable => 1.618,
            BufferLineHeight::Standard => 1.3,
            BufferLineHeight::Custom(line_height) => *line_height,
        }
    }
}

impl ThemeSettings {
    /// Returns the buffer font size.
    pub fn buffer_font_size(&self, cx: &App) -> Pixels {
        let font_size = cx
            .try_global::<BufferFontSize>()
            .map(|size| size.0)
            .unwrap_or(self.buffer_font_size);
        clamp_font_size(font_size)
    }

    /// Returns the UI font size.
    pub fn ui_font_size(&self, cx: &App) -> Pixels {
        let font_size = cx
            .try_global::<UiFontSize>()
            .map(|size| size.0)
            .unwrap_or(self.ui_font_size);
        clamp_font_size(font_size)
    }

    /// Returns the buffer font size, read from the settings.
    ///
    /// The real buffer font size is stored in-memory, to support temporary font size changes.
    /// Use [`Self::buffer_font_size`] to get the real font size.
    pub fn buffer_font_size_settings(&self) -> Pixels {
        self.buffer_font_size
    }

    /// Returns the UI font size, read from the settings.
    ///
    /// The real UI font size is stored in-memory, to support temporary font size changes.
    /// Use [`Self::ui_font_size`] to get the real font size.
    pub fn ui_font_size_settings(&self) -> Pixels {
        self.ui_font_size
    }

    // TODO: Rename: `line_height` -> `buffer_line_height`
    /// Returns the buffer's line height.
    pub fn line_height(&self) -> f32 {
        f32::max(self.buffer_line_height.value(), MIN_LINE_HEIGHT)
    }

    /// Switches to the theme with the given name, if it exists.
    ///
    /// Returns a `Some` containing the new theme if it was successful.
    /// Returns `None` otherwise.
    pub fn switch_theme(&mut self, theme: &str, cx: &mut App) -> Option<Arc<Theme>> {
        let themes = ThemeRegistry::default_global(cx);

        let mut new_theme = None;

        match themes.get(theme) {
            Ok(theme) => {
                self.active_theme = theme.clone();
                new_theme = Some(theme);
            }
            Err(err @ ThemeNotFoundError(_)) => {
                log::error!("{err}");
            }
        }

        self.apply_theme_overrides();

        new_theme
    }

    /// Applies the theme overrides, if there are any, to the current theme.
    pub fn apply_theme_overrides(&mut self) {
        if let Some(theme_overrides) = &self.theme_overrides {
            let mut base_theme = (*self.active_theme).clone();

            if let Some(window_background_appearance) = theme_overrides.window_background_appearance
            {
                base_theme.styles.window_background_appearance =
                    window_background_appearance.into();
            }

            base_theme
                .styles
                .colors
                .refine(&theme_overrides.theme_colors_refinement());
            base_theme
                .styles
                .status
                .refine(&theme_overrides.status_colors_refinement());
            base_theme.styles.player.merge(&theme_overrides.players);
            base_theme.styles.accents.merge(&theme_overrides.accents);
            base_theme.styles.syntax =
                SyntaxTheme::merge(base_theme.styles.syntax, theme_overrides.syntax_overrides());

            self.active_theme = Arc::new(base_theme);
        }
    }

    /// Switches to the icon theme with the given name, if it exists.
    ///
    /// Returns a `Some` containing the new icon theme if it was successful.
    /// Returns `None` otherwise.
    pub fn switch_icon_theme(&mut self, icon_theme: &str, cx: &mut App) -> Option<Arc<IconTheme>> {
        let themes = ThemeRegistry::default_global(cx);

        let mut new_icon_theme = None;

        if let Some(icon_theme) = themes.get_icon_theme(icon_theme).log_err() {
            self.active_icon_theme = icon_theme.clone();
            new_icon_theme = Some(icon_theme);
            cx.refresh_windows();
        }

        new_icon_theme
    }
}

/// Observe changes to the adjusted buffer font size.
pub fn observe_buffer_font_size_adjustment<V: 'static>(
    cx: &mut Context<V>,
    f: impl 'static + Fn(&mut V, &mut Context<V>),
) -> Subscription {
    cx.observe_global::<BufferFontSize>(f)
}

/// Gets the font size, adjusted by the difference between the current buffer font size and the one set in the settings.
pub fn adjusted_font_size(size: Pixels, cx: &App) -> Pixels {
    let adjusted_font_size =
        if let Some(BufferFontSize(adjusted_size)) = cx.try_global::<BufferFontSize>() {
            let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
            let delta = *adjusted_size - buffer_font_size;
            size + delta
        } else {
            size
        };
    clamp_font_size(adjusted_font_size)
}

/// Adjusts the buffer font size.
pub fn adjust_buffer_font_size(cx: &mut App, mut f: impl FnMut(&mut Pixels)) {
    let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
    let mut adjusted_size = cx
        .try_global::<BufferFontSize>()
        .map_or(buffer_font_size, |adjusted_size| adjusted_size.0);

    f(&mut adjusted_size);
    cx.set_global(BufferFontSize(clamp_font_size(adjusted_size)));
    cx.refresh_windows();
}

/// Resets the buffer font size to the default value.
pub fn reset_buffer_font_size(cx: &mut App) {
    if cx.has_global::<BufferFontSize>() {
        cx.remove_global::<BufferFontSize>();
        cx.refresh_windows();
    }
}

// TODO: Make private, change usages to use `get_ui_font_size` instead.
#[allow(missing_docs)]
pub fn setup_ui_font(window: &mut Window, cx: &mut App) -> gpui::Font {
    let (ui_font, ui_font_size) = {
        let theme_settings = ThemeSettings::get_global(cx);
        let font = theme_settings.ui_font.clone();
        (font, theme_settings.ui_font_size(cx))
    };

    window.set_rem_size(ui_font_size);
    ui_font
}

/// Sets the adjusted UI font size.
pub fn adjust_ui_font_size(cx: &mut App, mut f: impl FnMut(&mut Pixels)) {
    let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
    let mut adjusted_size = cx
        .try_global::<UiFontSize>()
        .map_or(ui_font_size, |adjusted_size| adjusted_size.0);

    f(&mut adjusted_size);
    cx.set_global(UiFontSize(clamp_font_size(adjusted_size)));
    cx.refresh_windows();
}

/// Resets the UI font size to the default value.
pub fn reset_ui_font_size(cx: &mut App) {
    if cx.has_global::<UiFontSize>() {
        cx.remove_global::<UiFontSize>();
        cx.refresh_windows();
    }
}

/// Ensures font size is within the valid range.
pub fn clamp_font_size(size: Pixels) -> Pixels {
    size.max(MIN_FONT_SIZE)
}

fn clamp_font_weight(weight: f32) -> FontWeight {
    FontWeight(weight.clamp(100., 950.))
}

impl settings::Settings for ThemeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ThemeSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, cx: &mut App) -> Result<Self> {
        let themes = ThemeRegistry::default_global(cx);
        let system_appearance = SystemAppearance::default_global(cx);

        let defaults = sources.default;
        let mut this = Self {
            ui_font_size: defaults.ui_font_size.unwrap().into(),
            ui_font: Font {
                family: defaults.ui_font_family.as_ref().unwrap().clone().into(),
                features: defaults.ui_font_features.clone().unwrap(),
                fallbacks: defaults
                    .ui_font_fallbacks
                    .as_ref()
                    .map(|fallbacks| FontFallbacks::from_fonts(fallbacks.clone())),
                weight: defaults.ui_font_weight.map(FontWeight).unwrap(),
                style: Default::default(),
            },
            buffer_font: Font {
                family: defaults.buffer_font_family.as_ref().unwrap().clone().into(),
                features: defaults.buffer_font_features.clone().unwrap(),
                fallbacks: defaults
                    .buffer_font_fallbacks
                    .as_ref()
                    .map(|fallbacks| FontFallbacks::from_fonts(fallbacks.clone())),
                weight: defaults.buffer_font_weight.map(FontWeight).unwrap(),
                style: FontStyle::default(),
            },
            buffer_font_size: defaults.buffer_font_size.unwrap().into(),
            buffer_line_height: defaults.buffer_line_height.unwrap(),
            theme_selection: defaults.theme.clone(),
            active_theme: themes
                .get(defaults.theme.as_ref().unwrap().theme(*system_appearance))
                .or(themes.get(&zed_default_dark().name))
                .unwrap(),
            theme_overrides: None,
            icon_theme_selection: defaults.icon_theme.clone(),
            active_icon_theme: defaults
                .icon_theme
                .as_ref()
                .and_then(|selection| {
                    themes
                        .get_icon_theme(selection.icon_theme(*system_appearance))
                        .ok()
                })
                .unwrap_or_else(|| themes.get_icon_theme(DEFAULT_ICON_THEME_NAME).unwrap()),
            ui_density: defaults.ui_density.unwrap_or(UiDensity::Default),
            unnecessary_code_fade: defaults.unnecessary_code_fade.unwrap_or(0.0),
        };

        for value in sources
            .user
            .into_iter()
            .chain(sources.release_channel)
            .chain(sources.server)
        {
            if let Some(value) = value.ui_density {
                this.ui_density = value;
            }

            if let Some(value) = value.buffer_font_family.clone() {
                this.buffer_font.family = value.into();
            }
            if let Some(value) = value.buffer_font_features.clone() {
                this.buffer_font.features = value;
            }
            if let Some(value) = value.buffer_font_fallbacks.clone() {
                this.buffer_font.fallbacks = Some(FontFallbacks::from_fonts(value));
            }
            if let Some(value) = value.buffer_font_weight {
                this.buffer_font.weight = clamp_font_weight(value);
            }

            if let Some(value) = value.ui_font_family.clone() {
                this.ui_font.family = value.into();
            }
            if let Some(value) = value.ui_font_features.clone() {
                this.ui_font.features = value;
            }
            if let Some(value) = value.ui_font_fallbacks.clone() {
                this.ui_font.fallbacks = Some(FontFallbacks::from_fonts(value));
            }
            if let Some(value) = value.ui_font_weight {
                this.ui_font.weight = clamp_font_weight(value);
            }

            if let Some(value) = &value.theme {
                this.theme_selection = Some(value.clone());

                let theme_name = value.theme(*system_appearance);

                match themes.get(theme_name) {
                    Ok(theme) => {
                        this.active_theme = theme;
                    }
                    Err(err @ ThemeNotFoundError(_)) => {
                        if themes.extensions_loaded() {
                            log::error!("{err}");
                        }
                    }
                }
            }

            this.theme_overrides.clone_from(&value.theme_overrides);
            this.apply_theme_overrides();

            if let Some(value) = &value.icon_theme {
                this.icon_theme_selection = Some(value.clone());

                let icon_theme_name = value.icon_theme(*system_appearance);

                match themes.get_icon_theme(icon_theme_name) {
                    Ok(icon_theme) => {
                        this.active_icon_theme = icon_theme;
                    }
                    Err(err @ IconThemeNotFoundError(_)) => {
                        if themes.extensions_loaded() {
                            log::error!("{err}");
                        }
                    }
                }
            }

            merge(&mut this.ui_font_size, value.ui_font_size.map(Into::into));
            this.ui_font_size = this.ui_font_size.clamp(px(6.), px(100.));

            merge(
                &mut this.buffer_font_size,
                value.buffer_font_size.map(Into::into),
            );
            this.buffer_font_size = this.buffer_font_size.clamp(px(6.), px(100.));

            merge(&mut this.buffer_line_height, value.buffer_line_height);

            // Clamp the `unnecessary_code_fade` to ensure text can't disappear entirely.
            merge(&mut this.unnecessary_code_fade, value.unnecessary_code_fade);
            this.unnecessary_code_fade = this.unnecessary_code_fade.clamp(0.0, 0.9);
        }

        Ok(this)
    }

    fn json_schema(
        generator: &mut SchemaGenerator,
        params: &SettingsJsonSchemaParams,
        cx: &App,
    ) -> schemars::schema::RootSchema {
        let mut root_schema = generator.root_schema_for::<ThemeSettingsContent>();
        let theme_names = ThemeRegistry::global(cx)
            .list_names()
            .into_iter()
            .map(|theme_name| Value::String(theme_name.to_string()))
            .collect();

        let theme_name_schema = SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(theme_names),
            ..Default::default()
        };

        let icon_theme_names = ThemeRegistry::global(cx)
            .list_icon_themes()
            .into_iter()
            .map(|icon_theme| Value::String(icon_theme.name.to_string()))
            .collect();

        let icon_theme_name_schema = SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(icon_theme_names),
            ..Default::default()
        };

        root_schema.definitions.extend([
            ("ThemeName".into(), theme_name_schema.into()),
            ("IconThemeName".into(), icon_theme_name_schema.into()),
            ("FontFamilies".into(), params.font_family_schema()),
            ("FontFallbacks".into(), params.font_fallback_schema()),
        ]);

        add_references_to_properties(
            &mut root_schema,
            &[
                ("buffer_font_family", "#/definitions/FontFamilies"),
                ("buffer_font_fallbacks", "#/definitions/FontFallbacks"),
                ("ui_font_family", "#/definitions/FontFamilies"),
                ("ui_font_fallbacks", "#/definitions/FontFallbacks"),
            ],
        );

        root_schema
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, old: &mut Self::FileContent) {
        vscode.f32_setting("editor.fontWeight", &mut old.buffer_font_weight);
        vscode.f32_setting("editor.fontSize", &mut old.buffer_font_size);
        vscode.string_setting("editor.font", &mut old.buffer_font_family);
        // TODO: possibly map editor.fontLigatures to buffer_font_features?
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
