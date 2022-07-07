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
use std::{collections::HashMap, num::NonZeroU32, sync::Arc};
use theme::{Theme, ThemeRegistry};
use util::ResultExt as _;

pub use keymap_file::{keymap_file_json_schema, KeymapFileContent};

#[derive(Clone)]
pub struct Settings {
    pub projects_online_by_default: bool,
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub default_buffer_font_size: f32,
    pub hover_popover_enabled: bool,
    pub vim_mode: bool,
    pub autosave: Autosave,
    pub language_settings: LanguageSettings,
    pub language_defaults: HashMap<Arc<str>, LanguageSettings>,
    pub language_overrides: HashMap<Arc<str>, LanguageSettings>,
    pub theme: Arc<Theme>,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct LanguageSettings {
    pub tab_size: Option<NonZeroU32>,
    pub hard_tabs: Option<bool>,
    pub soft_wrap: Option<SoftWrap>,
    pub preferred_line_length: Option<u32>,
    pub format_on_save: Option<FormatOnSave>,
    pub enable_language_server: Option<bool>,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FormatOnSave {
    Off,
    LanguageServer,
    External {
        command: String,
        arguments: Vec<String>,
    },
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Autosave {
    Off,
    AfterDelay { milliseconds: u64 },
    OnFocusChange,
    OnWindowChange,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct SettingsFileContent {
    #[serde(default)]
    pub projects_online_by_default: Option<bool>,
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    #[serde(default)]
    pub hover_popover_enabled: Option<bool>,
    #[serde(default)]
    pub vim_mode: Option<bool>,
    #[serde(default)]
    pub format_on_save: Option<FormatOnSave>,
    #[serde(default)]
    pub autosave: Option<Autosave>,
    #[serde(default)]
    pub enable_language_server: Option<bool>,
    #[serde(flatten)]
    pub editor: LanguageSettings,
    #[serde(default)]
    pub language_overrides: HashMap<Arc<str>, LanguageSettings>,
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
            default_buffer_font_size: 15.,
            hover_popover_enabled: true,
            vim_mode: false,
            autosave: Autosave::Off,
            language_settings: Default::default(),
            language_defaults: Default::default(),
            language_overrides: Default::default(),
            projects_online_by_default: true,
            theme,
        })
    }

    pub fn with_language_defaults(
        mut self,
        language_name: impl Into<Arc<str>>,
        overrides: LanguageSettings,
    ) -> Self {
        self.language_defaults
            .insert(language_name.into(), overrides);
        self
    }

    pub fn tab_size(&self, language: Option<&str>) -> NonZeroU32 {
        self.language_setting(language, |settings| settings.tab_size)
            .unwrap_or(4.try_into().unwrap())
    }

    pub fn hard_tabs(&self, language: Option<&str>) -> bool {
        self.language_setting(language, |settings| settings.hard_tabs)
            .unwrap_or(false)
    }

    pub fn soft_wrap(&self, language: Option<&str>) -> SoftWrap {
        self.language_setting(language, |settings| settings.soft_wrap)
            .unwrap_or(SoftWrap::None)
    }

    pub fn preferred_line_length(&self, language: Option<&str>) -> u32 {
        self.language_setting(language, |settings| settings.preferred_line_length)
            .unwrap_or(80)
    }

    pub fn format_on_save(&self, language: Option<&str>) -> FormatOnSave {
        self.language_setting(language, |settings| settings.format_on_save.clone())
            .unwrap_or(FormatOnSave::LanguageServer)
    }

    pub fn enable_language_server(&self, language: Option<&str>) -> bool {
        self.language_setting(language, |settings| settings.enable_language_server)
            .unwrap_or(true)
    }

    fn language_setting<F, R>(&self, language: Option<&str>, f: F) -> Option<R>
    where
        F: Fn(&LanguageSettings) -> Option<R>,
    {
        let mut language_override = None;
        let mut language_default = None;
        if let Some(language) = language {
            language_override = self.language_overrides.get(language).and_then(&f);
            language_default = self.language_defaults.get(language).and_then(&f);
        }

        language_override
            .or_else(|| f(&self.language_settings))
            .or(language_default)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Settings {
        Settings {
            buffer_font_family: cx.font_cache().load_family(&["Monaco"]).unwrap(),
            buffer_font_size: 14.,
            default_buffer_font_size: 14.,
            hover_popover_enabled: true,
            vim_mode: false,
            autosave: Autosave::Off,
            language_settings: Default::default(),
            language_defaults: Default::default(),
            language_overrides: Default::default(),
            projects_online_by_default: true,
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

        merge(
            &mut self.projects_online_by_default,
            data.projects_online_by_default,
        );
        merge(&mut self.buffer_font_size, data.buffer_font_size);
        merge(&mut self.default_buffer_font_size, data.buffer_font_size);
        merge(&mut self.hover_popover_enabled, data.hover_popover_enabled);
        merge(&mut self.vim_mode, data.vim_mode);
        merge(&mut self.autosave, data.autosave);
        merge_option(
            &mut self.language_settings.format_on_save,
            data.format_on_save.clone(),
        );
        merge_option(
            &mut self.language_settings.enable_language_server,
            data.enable_language_server,
        );
        merge_option(&mut self.language_settings.soft_wrap, data.editor.soft_wrap);
        merge_option(&mut self.language_settings.tab_size, data.editor.tab_size);
        merge_option(
            &mut self.language_settings.preferred_line_length,
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
                &mut target.enable_language_server,
                settings.enable_language_server,
            );
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

    // Construct language settings reference type
    let language_settings_schema_reference = Schema::Object(SchemaObject {
        reference: Some("#/definitions/LanguageSettings".to_owned()),
        ..Default::default()
    });
    let language_settings_properties = language_names
        .into_iter()
        .map(|name| {
            (
                name,
                Schema::Object(SchemaObject {
                    subschemas: Some(Box::new(SubschemaValidation {
                        all_of: Some(vec![language_settings_schema_reference.clone()]),
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
            properties: language_settings_properties,
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

fn merge_option<T>(target: &mut Option<T>, value: Option<T>) {
    if value.is_some() {
        *target = value;
    }
}

pub fn parse_json_with_comments<T: DeserializeOwned>(content: &str) -> Result<T> {
    Ok(serde_json::from_reader(
        json_comments::CommentSettings::c_style().strip_comments(content.as_bytes()),
    )?)
}
