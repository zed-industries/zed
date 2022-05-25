mod keymap_file;

use anyhow::Result;
use gpui::font_cache::{FamilyId, FontCache};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{
        InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec, SubschemaValidation,
    },
    JsonSchema,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use theme::{Theme, ThemeRegistry};
use util::ResultExt as _;

pub use keymap_file::{keymap_file_json_schema, KeymapFileContent};

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub vim_mode: bool,
    pub tab_size: u32,
    pub soft_wrap: SoftWrap,
    pub preferred_line_length: u32,
    pub format_on_save: bool,
    pub language_overrides: HashMap<Arc<str>, LanguageOverride>,
    pub theme: Arc<Theme>,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct LanguageOverride {
    pub tab_size: Option<u32>,
    pub soft_wrap: Option<SoftWrap>,
    pub preferred_line_length: Option<u32>,
    pub format_on_save: Option<bool>,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct SettingsFileContent {
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    #[serde(default)]
    pub vim_mode: Option<bool>,
    #[serde(default)]
    pub format_on_save: Option<bool>,
    #[serde(flatten)]
    pub editor: LanguageOverride,
    #[serde(default)]
    pub language_overrides: HashMap<Arc<str>, LanguageOverride>,
    #[serde(default)]
    pub theme: Option<String>,
}

impl Settings {
    pub fn new(
        buffer_font_family: &str,
        font_cache: &FontCache,
        theme: Arc<Theme>,
    ) -> Result<Self> {
        Ok(Self {
            buffer_font_family: font_cache.load_family(&[buffer_font_family])?,
            buffer_font_size: 15.,
            vim_mode: false,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            language_overrides: Default::default(),
            format_on_save: true,
            theme,
        })
    }

    pub fn with_overrides(
        mut self,
        language_name: impl Into<Arc<str>>,
        overrides: LanguageOverride,
    ) -> Self {
        self.language_overrides
            .insert(language_name.into(), overrides);
        self
    }

    pub fn tab_size(&self, language: Option<&str>) -> u32 {
        language
            .and_then(|language| self.language_overrides.get(language))
            .and_then(|settings| settings.tab_size)
            .unwrap_or(self.tab_size)
    }

    pub fn soft_wrap(&self, language: Option<&str>) -> SoftWrap {
        language
            .and_then(|language| self.language_overrides.get(language))
            .and_then(|settings| settings.soft_wrap)
            .unwrap_or(self.soft_wrap)
    }

    pub fn preferred_line_length(&self, language: Option<&str>) -> u32 {
        language
            .and_then(|language| self.language_overrides.get(language))
            .and_then(|settings| settings.preferred_line_length)
            .unwrap_or(self.preferred_line_length)
    }

    pub fn format_on_save(&self, language: Option<&str>) -> bool {
        language
            .and_then(|language| self.language_overrides.get(language))
            .and_then(|settings| settings.format_on_save)
            .unwrap_or(self.format_on_save)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Settings {
        Settings {
            buffer_font_family: cx.font_cache().load_family(&["Monaco"]).unwrap(),
            buffer_font_size: 14.,
            vim_mode: false,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            format_on_save: true,
            language_overrides: Default::default(),
            theme: gpui::fonts::with_font_cache(cx.font_cache().clone(), || Default::default()),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_async(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings = Self::test(cx);
            cx.set_global(settings.clone());
        });
    }

    pub fn merge(
        &mut self,
        data: &SettingsFileContent,
        theme_registry: &ThemeRegistry,
        font_cache: &FontCache,
    ) {
        if let Some(value) = &data.buffer_font_family {
            if let Some(id) = font_cache.load_family(&[value]).log_err() {
                self.buffer_font_family = id;
            }
        }
        if let Some(value) = &data.theme {
            if let Some(theme) = theme_registry.get(&value.to_string()).log_err() {
                self.theme = theme;
            }
        }

        merge(&mut self.buffer_font_size, data.buffer_font_size);
        merge(&mut self.vim_mode, data.vim_mode);
        merge(&mut self.format_on_save, data.format_on_save);
        merge(&mut self.soft_wrap, data.editor.soft_wrap);
        merge(&mut self.tab_size, data.editor.tab_size);
        merge(
            &mut self.preferred_line_length,
            data.editor.preferred_line_length,
        );

        for (language_name, settings) in data.language_overrides.clone().into_iter() {
            let target = self
                .language_overrides
                .entry(language_name.into())
                .or_default();

            merge_option(&mut target.tab_size, settings.tab_size);
            merge_option(&mut target.soft_wrap, settings.soft_wrap);
            merge_option(&mut target.format_on_save, settings.format_on_save);
            merge_option(
                &mut target.preferred_line_length,
                settings.preferred_line_length,
            );
        }
    }
}

pub fn settings_file_json_schema(
    theme_names: Vec<String>,
    language_names: Vec<String>,
) -> serde_json::Value {
    let settings = SchemaSettings::draft07().with(|settings| {
        settings.option_add_null_type = false;
    });
    let generator = SchemaGenerator::new(settings);
    let mut root_schema = generator.into_root_schema_for::<SettingsFileContent>();

    // Construct theme names reference type
    let theme_names = theme_names
        .into_iter()
        .map(|name| Value::String(name))
        .collect();
    let theme_names_schema = Schema::Object(SchemaObject {
        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
        enum_values: Some(theme_names),
        ..Default::default()
    });
    root_schema
        .definitions
        .insert("ThemeName".to_owned(), theme_names_schema);

    // Construct language overrides reference type
    let language_override_schema_reference = Schema::Object(SchemaObject {
        reference: Some("#/definitions/LanguageOverride".to_owned()),
        ..Default::default()
    });
    let language_overrides_properties = language_names
        .into_iter()
        .map(|name| {
            (
                name,
                Schema::Object(SchemaObject {
                    subschemas: Some(Box::new(SubschemaValidation {
                        all_of: Some(vec![language_override_schema_reference.clone()]),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
            )
        })
        .collect();
    let language_overrides_schema = Schema::Object(SchemaObject {
        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
        object: Some(Box::new(ObjectValidation {
            properties: language_overrides_properties,
            ..Default::default()
        })),
        ..Default::default()
    });
    root_schema
        .definitions
        .insert("LanguageOverrides".to_owned(), language_overrides_schema);

    // Modify theme property to use new theme reference type
    let settings_file_schema = root_schema.schema.object.as_mut().unwrap();
    let language_overrides_schema_reference = Schema::Object(SchemaObject {
        reference: Some("#/definitions/ThemeName".to_owned()),
        ..Default::default()
    });
    settings_file_schema.properties.insert(
        "theme".to_owned(),
        Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                all_of: Some(vec![language_overrides_schema_reference]),
                ..Default::default()
            })),
            ..Default::default()
        }),
    );

    // Modify language_overrides property to use LanguageOverrides reference
    settings_file_schema.properties.insert(
        "language_overrides".to_owned(),
        Schema::Object(SchemaObject {
            reference: Some("#/definitions/LanguageOverrides".to_owned()),
            ..Default::default()
        }),
    );
    serde_json::to_value(root_schema).unwrap()
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

fn merge_option<T: Copy>(target: &mut Option<T>, value: Option<T>) {
    if value.is_some() {
        *target = value;
    }
}

pub fn parse_json_with_comments<T: DeserializeOwned>(content: &str) -> Result<T> {
    Ok(serde_json::from_reader(
        json_comments::CommentSettings::c_style().strip_comments(content.as_bytes()),
    )?)
}
