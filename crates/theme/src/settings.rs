use crate::one_themes::one_dark;
use crate::{Appearance, SyntaxTheme, Theme, ThemeRegistry, ThemeStyleContent};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::{
    px, AppContext, Font, FontFeatures, FontStyle, FontWeight, Global, Pixels, Subscription,
    ViewContext,
};
use refineable::Refineable;
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{Settings, SettingsJsonSchemaParams};
use std::sync::Arc;
use util::ResultExt as _;

const MIN_FONT_SIZE: Pixels = px(6.0);
const MIN_LINE_HEIGHT: f32 = 1.0;

#[derive(Clone)]
pub struct ThemeSettings {
    pub ui_font_size: Pixels,
    pub ui_font: Font,
    pub buffer_font: Font,
    pub buffer_font_size: Pixels,
    pub buffer_line_height: BufferLineHeight,
    pub theme_selection: Option<ThemeSelection>,
    pub active_theme: Arc<Theme>,
    pub theme_overrides: Option<ThemeStyleContent>,
}

impl ThemeSettings {
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
                theme_name = match *system_appearance {
                    Appearance::Light => "One Light",
                    Appearance::Dark => "One Dark",
                };
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

#[derive(Default)]
pub(crate) struct AdjustedBufferFontSize(Pixels);

impl Global for AdjustedBufferFontSize {}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ThemeSelection {
    Static(#[schemars(schema_with = "theme_name_ref")] String),
    Dynamic {
        #[serde(default)]
        mode: ThemeMode,
        #[schemars(schema_with = "theme_name_ref")]
        light: String,
        #[schemars(schema_with = "theme_name_ref")]
        dark: String,
    },
}

fn theme_name_ref(_: &mut SchemaGenerator) -> Schema {
    Schema::new_ref("#/definitions/ThemeName".into())
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
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
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ThemeSettingsContent {
    #[serde(default)]
    pub ui_font_size: Option<f32>,
    #[serde(default)]
    pub ui_font_family: Option<String>,
    #[serde(default)]
    pub ui_font_features: Option<FontFeatures>,
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    #[serde(default)]
    pub buffer_line_height: Option<BufferLineHeight>,
    #[serde(default)]
    pub buffer_font_features: Option<FontFeatures>,
    #[serde(default)]
    pub theme: Option<ThemeSelection>,

    /// EXPERIMENTAL: Overrides for the current theme.
    ///
    /// These values will override the ones on the current theme specified in `theme`.
    #[serde(rename = "experimental.theme_overrides", default)]
    pub theme_overrides: Option<ThemeStyleContent>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum BufferLineHeight {
    #[default]
    Comfortable,
    Standard,
    Custom(f32),
}

impl BufferLineHeight {
    pub fn value(&self) -> f32 {
        match self {
            BufferLineHeight::Comfortable => 1.618,
            BufferLineHeight::Standard => 1.3,
            BufferLineHeight::Custom(line_height) => *line_height,
        }
    }
}

impl ThemeSettings {
    pub fn buffer_font_size(&self, cx: &AppContext) -> Pixels {
        cx.try_global::<AdjustedBufferFontSize>()
            .map_or(self.buffer_font_size, |size| size.0)
            .max(MIN_FONT_SIZE)
    }

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

            base_theme
                .styles
                .colors
                .refine(&theme_overrides.theme_colors_refinement());
            base_theme
                .styles
                .status
                .refine(&theme_overrides.status_colors_refinement());
            base_theme.styles.player.merge(&theme_overrides.players);
            base_theme.styles.syntax = Arc::new(SyntaxTheme {
                highlights: {
                    let mut highlights = base_theme.styles.syntax.highlights.clone();
                    // Overrides come second in the highlight list so that they take precedence
                    // over the ones in the base theme.
                    highlights.extend(theme_overrides.syntax_overrides());
                    highlights
                },
            });

            self.active_theme = Arc::new(base_theme);
        }
    }
}

pub fn observe_buffer_font_size_adjustment<V: 'static>(
    cx: &mut ViewContext<V>,
    f: impl 'static + Fn(&mut V, &mut ViewContext<V>),
) -> Subscription {
    cx.observe_global::<AdjustedBufferFontSize>(f)
}

pub fn adjusted_font_size(size: Pixels, cx: &mut AppContext) -> Pixels {
    if let Some(AdjustedBufferFontSize(adjusted_size)) = cx.try_global::<AdjustedBufferFontSize>() {
        let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
        let delta = *adjusted_size - buffer_font_size;
        size + delta
    } else {
        size
    }
    .max(MIN_FONT_SIZE)
}

pub fn adjust_font_size(cx: &mut AppContext, f: fn(&mut Pixels)) {
    let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
    let mut adjusted_size = cx
        .try_global::<AdjustedBufferFontSize>()
        .map_or(buffer_font_size, |adjusted_size| adjusted_size.0);

    f(&mut adjusted_size);
    adjusted_size = adjusted_size.max(MIN_FONT_SIZE);
    cx.set_global(AdjustedBufferFontSize(adjusted_size));
    cx.refresh();
}

pub fn reset_font_size(cx: &mut AppContext) {
    if cx.has_global::<AdjustedBufferFontSize>() {
        cx.remove_global::<AdjustedBufferFontSize>();
        cx.refresh();
    }
}

impl settings::Settings for ThemeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ThemeSettingsContent;

    fn load(
        defaults: &Self::FileContent,
        user_values: &[&Self::FileContent],
        cx: &mut AppContext,
    ) -> Result<Self> {
        let themes = ThemeRegistry::default_global(cx);
        let system_appearance = SystemAppearance::default_global(cx);

        let mut this = Self {
            ui_font_size: defaults.ui_font_size.unwrap().into(),
            ui_font: Font {
                family: defaults.ui_font_family.clone().unwrap().into(),
                features: defaults.ui_font_features.unwrap(),
                weight: Default::default(),
                style: Default::default(),
            },
            buffer_font: Font {
                family: defaults.buffer_font_family.clone().unwrap().into(),
                features: defaults.buffer_font_features.unwrap(),
                weight: FontWeight::default(),
                style: FontStyle::default(),
            },
            buffer_font_size: defaults.buffer_font_size.unwrap().into(),
            buffer_line_height: defaults.buffer_line_height.unwrap(),
            theme_selection: defaults.theme.clone(),
            active_theme: themes
                .get(defaults.theme.as_ref().unwrap().theme(*system_appearance))
                .or(themes.get(&one_dark().name))
                .unwrap(),
            theme_overrides: None,
        };

        for value in user_values.iter().copied().cloned() {
            if let Some(value) = value.buffer_font_family {
                this.buffer_font.family = value.into();
            }
            if let Some(value) = value.buffer_font_features {
                this.buffer_font.features = value;
            }

            if let Some(value) = value.ui_font_family {
                this.ui_font.family = value.into();
            }
            if let Some(value) = value.ui_font_features {
                this.ui_font.features = value;
            }

            if let Some(value) = &value.theme {
                this.theme_selection = Some(value.clone());

                let theme_name = value.theme(*system_appearance);

                if let Some(theme) = themes.get(theme_name).log_err() {
                    this.active_theme = theme;
                }
            }

            this.theme_overrides = value.theme_overrides;
            this.apply_theme_overrides();

            merge(&mut this.ui_font_size, value.ui_font_size.map(Into::into));
            merge(
                &mut this.buffer_font_size,
                value.buffer_font_size.map(Into::into),
            );
            merge(&mut this.buffer_line_height, value.buffer_line_height);
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
            .list_names(params.staff_mode)
            .into_iter()
            .map(|theme_name| Value::String(theme_name.to_string()))
            .collect();

        let theme_name_schema = SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(theme_names),
            ..Default::default()
        };

        let available_fonts = params
            .font_names
            .iter()
            .cloned()
            .map(Value::String)
            .collect();
        let fonts_schema = SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(available_fonts),
            ..Default::default()
        };
        root_schema.definitions.extend([
            ("ThemeName".into(), theme_name_schema.into()),
            ("FontFamilies".into(), fonts_schema.into()),
        ]);

        root_schema
            .schema
            .object
            .as_mut()
            .unwrap()
            .properties
            .extend([
                (
                    "buffer_font_family".to_owned(),
                    Schema::new_ref("#/definitions/FontFamilies".into()),
                ),
                (
                    "ui_font_family".to_owned(),
                    Schema::new_ref("#/definitions/FontFamilies".into()),
                ),
            ]);

        root_schema
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
