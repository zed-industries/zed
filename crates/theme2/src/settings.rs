use crate::{ThemeRegistry, ThemeVariant};
use anyhow::Result;
use gpui::{px, AppContext, Font, FontFeatures, FontStyle, FontWeight, Pixels};
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
    pub buffer_font: Font,
    pub buffer_font_size: Pixels,
    pub buffer_line_height: BufferLineHeight,
    pub active_theme: Arc<ThemeVariant>,
}

#[derive(Default)]
pub struct AdjustedBufferFontSize(Option<Pixels>);

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ThemeSettingsContent {
    #[serde(default)]
    pub ui_font_size: Option<f32>,
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    #[serde(default)]
    pub buffer_line_height: Option<BufferLineHeight>,
    #[serde(default)]
    pub buffer_font_features: Option<FontFeatures>,
    #[serde(default)]
    pub theme: Option<String>,
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
    pub fn buffer_font_size(&self, cx: &mut AppContext) -> Pixels {
        let font_size = *cx
            .default_global::<AdjustedBufferFontSize>()
            .0
            .get_or_insert(self.buffer_font_size.into());
        font_size.max(MIN_FONT_SIZE)
    }

    pub fn line_height(&self) -> f32 {
        f32::max(self.buffer_line_height.value(), MIN_LINE_HEIGHT)
    }
}

pub fn adjusted_font_size(size: Pixels, cx: &mut AppContext) -> Pixels {
    if let Some(adjusted_size) = cx.default_global::<AdjustedBufferFontSize>().0 {
        let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
        let delta = adjusted_size - buffer_font_size;
        size + delta
    } else {
        size
    }
    .max(MIN_FONT_SIZE)
}

pub fn adjust_font_size(cx: &mut AppContext, f: fn(&mut Pixels)) {
    let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
    let adjusted_size = cx
        .default_global::<AdjustedBufferFontSize>()
        .0
        .get_or_insert(buffer_font_size);
    f(adjusted_size);
    *adjusted_size = (*adjusted_size).max(MIN_FONT_SIZE - buffer_font_size);
    cx.refresh();
}

pub fn reset_font_size(cx: &mut AppContext) {
    if cx.has_global::<AdjustedBufferFontSize>() {
        cx.global_mut::<AdjustedBufferFontSize>().0 = None;
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
        let themes = cx.default_global::<Arc<ThemeRegistry>>();

        let mut this = Self {
            ui_font_size: defaults.ui_font_size.unwrap_or(16.).into(),
            buffer_font: Font {
                family: defaults.buffer_font_family.clone().unwrap().into(),
                features: defaults.buffer_font_features.clone().unwrap(),
                weight: FontWeight::default(),
                style: FontStyle::default(),
            },
            buffer_font_size: defaults.buffer_font_size.unwrap().into(),
            buffer_line_height: defaults.buffer_line_height.unwrap(),
            active_theme: themes
                .get(defaults.theme.as_ref().unwrap())
                .or(themes.get("Zed Pro Moonlight"))
                .unwrap(),
        };

        for value in user_values.into_iter().copied().cloned() {
            if let Some(value) = value.buffer_font_family {
                this.buffer_font.family = value.into();
            }
            if let Some(value) = value.buffer_font_features {
                this.buffer_font.features = value;
            }

            if let Some(value) = &value.theme {
                if let Some(theme) = themes.get(value).log_err() {
                    this.active_theme = theme;
                }
            }

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
        let theme_names = cx
            .global::<Arc<ThemeRegistry>>()
            .list_names(params.staff_mode)
            .map(|theme_name| Value::String(theme_name.to_string()))
            .collect();

        let theme_name_schema = SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(theme_names),
            ..Default::default()
        };

        root_schema
            .definitions
            .extend([("ThemeName".into(), theme_name_schema.into())]);

        root_schema
            .schema
            .object
            .as_mut()
            .unwrap()
            .properties
            .extend([(
                "theme".to_owned(),
                Schema::new_ref("#/definitions/ThemeName".into()),
            )]);

        root_schema
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
