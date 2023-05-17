use crate::{Theme, ThemeRegistry};
use anyhow::Result;
use gpui::{font_cache::FamilyId, fonts, AppContext};
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::SettingsJsonSchemaParams;
use std::sync::Arc;
use util::ResultExt as _;

const MIN_FONT_SIZE: f32 = 6.0;

#[derive(Clone)]
pub struct ThemeSettings {
    pub buffer_font_family_name: String,
    pub buffer_font_features: fonts::Features,
    pub buffer_font_family: FamilyId,
    buffer_font_size: f32,
    pub theme: Arc<Theme>,
}

pub struct AdjustedBufferFontSize(pub f32);

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ThemeSettingsContent {
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    #[serde(default)]
    pub buffer_font_features: Option<fonts::Features>,
    #[serde(default)]
    pub theme: Option<String>,
}

impl ThemeSettings {
    pub fn buffer_font_size(&self, cx: &AppContext) -> f32 {
        if cx.has_global::<AdjustedBufferFontSize>() {
            cx.global::<AdjustedBufferFontSize>().0
        } else {
            self.buffer_font_size
        }
        .max(MIN_FONT_SIZE)
    }
}

pub fn adjusted_font_size(size: f32, cx: &AppContext) -> f32 {
    if cx.has_global::<AdjustedBufferFontSize>() {
        let buffer_font_size = settings::get::<ThemeSettings>(cx).buffer_font_size;
        let delta = cx.global::<AdjustedBufferFontSize>().0 - buffer_font_size;
        size + delta
    } else {
        size
    }
    .max(MIN_FONT_SIZE)
}

pub fn adjust_font_size(cx: &mut AppContext, f: fn(&mut f32)) {
    if !cx.has_global::<AdjustedBufferFontSize>() {
        let buffer_font_size = settings::get::<ThemeSettings>(cx).buffer_font_size;
        cx.set_global(AdjustedBufferFontSize(buffer_font_size));
    }

    cx.update_global::<AdjustedBufferFontSize, _, _>(|delta, cx| {
        f(&mut delta.0);
        delta.0 = delta
            .0
            .max(MIN_FONT_SIZE - settings::get::<ThemeSettings>(cx).buffer_font_size);
    });
    cx.refresh_windows();
}

pub fn reset_font_size(cx: &mut AppContext) {
    cx.remove_global::<AdjustedBufferFontSize>();
    cx.refresh_windows();
}

impl settings::Setting for ThemeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ThemeSettingsContent;

    fn load(
        defaults: &Self::FileContent,
        user_values: &[&Self::FileContent],
        cx: &AppContext,
    ) -> Result<Self> {
        let buffer_font_features = defaults.buffer_font_features.clone().unwrap();
        let themes = cx.global::<Arc<ThemeRegistry>>();

        let mut this = Self {
            buffer_font_family: cx
                .font_cache()
                .load_family(
                    &[defaults.buffer_font_family.as_ref().unwrap()],
                    &buffer_font_features,
                )
                .unwrap(),
            buffer_font_family_name: defaults.buffer_font_family.clone().unwrap(),
            buffer_font_features,
            buffer_font_size: defaults.buffer_font_size.unwrap(),
            theme: themes.get(defaults.theme.as_ref().unwrap()).unwrap(),
        };

        for value in user_values.into_iter().copied().cloned() {
            let font_cache = cx.font_cache();
            let mut family_changed = false;
            if let Some(value) = value.buffer_font_family {
                this.buffer_font_family_name = value;
                family_changed = true;
            }
            if let Some(value) = value.buffer_font_features {
                this.buffer_font_features = value;
                family_changed = true;
            }
            if family_changed {
                if let Some(id) = font_cache
                    .load_family(&[&this.buffer_font_family_name], &this.buffer_font_features)
                    .log_err()
                {
                    this.buffer_font_family = id;
                }
            }

            if let Some(value) = &value.theme {
                if let Some(theme) = themes.get(value).log_err() {
                    this.theme = theme;
                }
            }

            merge(&mut this.buffer_font_size, value.buffer_font_size);
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
            .list(params.staff_mode)
            .map(|theme| Value::String(theme.name.clone()))
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
