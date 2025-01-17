use crate::fallback_themes::zed_default_dark;
use crate::{
    Appearance, IconTheme, SyntaxTheme, Theme, ThemeRegistry, ThemeStyleContent,
    DEFAULT_ICON_THEME_NAME,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::{
    px, AppContext, Font, FontFallbacks, FontFeatures, FontStyle, FontWeight, Global, Pixels,
    WindowContext,
};
use refineable::Refineable;
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{add_references_to_properties, Settings, SettingsJsonSchemaParams, SettingsSources};
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
    pub ui_font_size: Pixels,
    /// The font used for UI elements.
    pub ui_font: Font,
    /// The font size used for buffers, and the terminal.
    ///
    /// The terminal font size can be overridden using it's own setting.
    pub buffer_font_size: Pixels,
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
    /// TODO: Document this further
    pub theme_selection: Option<ThemeSelection>,
    /// The active theme.
    pub active_theme: Arc<Theme>,
    /// Manual overrides for the active theme.
    ///
    /// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
    pub theme_overrides: Option<ThemeStyleContent>,
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
    pub fn reload_current_theme(cx: &mut AppContext) {
        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        let system_appearance = SystemAppearance::global(cx);

        if let Some(theme_selection) = theme_settings.theme_selection.clone() {
            let mut theme_name = theme_selection.theme(*system_appearance);

            // If the selected theme doesn't exist, fall back to a default theme
            // based on the system appearance.
            let theme_registry = ThemeRegistry::global(cx);
            if theme_registry.get(theme_name).ok().is_none() {
                theme_name = Self::default_theme(*system_appearance);
            };

            if let Some(_theme) = theme_settings.switch_theme(theme_name, cx) {
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
    pub fn init(cx: &mut AppContext) {
        *cx.default_global::<GlobalSystemAppearance>() =
            GlobalSystemAppearance(SystemAppearance(cx.window_appearance().into()));
    }

    /// Returns the global [`SystemAppearance`].
    ///
    /// Inserts a default [`SystemAppearance`] if one does not yet exist.
    pub(crate) fn default_global(cx: &mut AppContext) -> Self {
        cx.default_global::<GlobalSystemAppearance>().0
    }

    /// Returns the global [`SystemAppearance`].
    pub fn global(cx: &AppContext) -> Self {
        cx.global::<GlobalSystemAppearance>().0
    }

    /// Returns a mutable reference to the global [`SystemAppearance`].
    pub fn global_mut(cx: &mut AppContext) -> &mut Self {
        cx.global_mut::<GlobalSystemAppearance>()
    }
}

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
    ///
    /// Currently not exposed to the user.
    #[serde(skip)]
    #[serde(default)]
    pub icon_theme: Option<String>,

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
    /// Returns the [AdjustedBufferFontSize].
    pub fn buffer_font_size(&self) -> Pixels {
        Self::clamp_font_size(self.buffer_font_size)
    }

    /// Ensures that the font size is within the valid range.
    pub fn clamp_font_size(size: Pixels) -> Pixels {
        size.max(MIN_FONT_SIZE)
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
    pub fn switch_theme(&mut self, theme: &str, cx: &mut AppContext) -> Option<Arc<Theme>> {
        let themes = ThemeRegistry::default_global(cx);

        let mut new_theme = None;

        if let Some(theme) = themes.get(theme).log_err() {
            self.active_theme = theme.clone();
            new_theme = Some(theme);
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
}

// TODO: Make private, change usages to use `get_ui_font_size` instead.
#[allow(missing_docs)]
pub fn setup_ui_font(cx: &mut WindowContext) -> gpui::Font {
    let (ui_font, ui_font_size) = {
        let theme_settings = ThemeSettings::get_global(cx);
        let font = theme_settings.ui_font.clone();
        (font, theme_settings.ui_font_size)
    };

    cx.set_rem_size(ui_font_size);
    ui_font
}

fn clamp_font_weight(weight: f32) -> FontWeight {
    FontWeight(weight.clamp(100., 950.))
}

impl settings::Settings for ThemeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ThemeSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, cx: &mut AppContext) -> Result<Self> {
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
            active_icon_theme: defaults
                .icon_theme
                .as_ref()
                .and_then(|name| themes.get_icon_theme(name).ok())
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

                if let Some(theme) = themes.get(theme_name).log_err() {
                    this.active_theme = theme;
                }
            }

            this.theme_overrides.clone_from(&value.theme_overrides);
            this.apply_theme_overrides();

            if let Some(value) = &value.icon_theme {
                if let Some(icon_theme) = themes.get_icon_theme(value).log_err() {
                    this.active_icon_theme = icon_theme.clone();
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
        cx: &AppContext,
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

        root_schema.definitions.extend([
            ("ThemeName".into(), theme_name_schema.into()),
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
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
