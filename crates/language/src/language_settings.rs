use crate::{File, Language};
use anyhow::Result;
use collections::HashMap;
use globset::GlobMatcher;
use gpui::AppContext;
use schemars::{
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use std::{num::NonZeroU32, path::Path, sync::Arc};

pub fn init(cx: &mut AppContext) {
    settings::register::<AllLanguageSettings>(cx);
}

pub fn language_settings<'a>(
    language: Option<&Arc<Language>>,
    file: Option<&Arc<dyn File>>,
    cx: &'a AppContext,
) -> &'a LanguageSettings {
    let language_name = language.map(|l| l.name());
    all_language_settings(file, cx).language(language_name.as_deref())
}

pub fn all_language_settings<'a>(
    file: Option<&Arc<dyn File>>,
    cx: &'a AppContext,
) -> &'a AllLanguageSettings {
    let location = file.map(|f| (f.worktree_id(), f.path().as_ref()));
    settings::get_local(location, cx)
}

#[derive(Debug, Clone)]
pub struct AllLanguageSettings {
    pub copilot: CopilotSettings,
    defaults: LanguageSettings,
    languages: HashMap<Arc<str>, LanguageSettings>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageSettings {
    pub tab_size: NonZeroU32,
    pub hard_tabs: bool,
    pub soft_wrap: SoftWrap,
    pub preferred_line_length: u32,
    pub format_on_save: FormatOnSave,
    pub remove_trailing_whitespace_on_save: bool,
    pub ensure_final_newline_on_save: bool,
    pub formatter: Formatter,
    pub enable_language_server: bool,
    pub show_copilot_suggestions: bool,
    pub show_whitespaces: ShowWhitespaceSetting,
    pub extend_comment_on_newline: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CopilotSettings {
    pub feature_enabled: bool,
    pub disabled_globs: Vec<GlobMatcher>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct AllLanguageSettingsContent {
    #[serde(default)]
    pub features: Option<FeaturesContent>,
    #[serde(default)]
    pub copilot: Option<CopilotSettingsContent>,
    #[serde(flatten)]
    pub defaults: LanguageSettingsContent,
    #[serde(default, alias = "language_overrides")]
    pub languages: HashMap<Arc<str>, LanguageSettingsContent>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct LanguageSettingsContent {
    #[serde(default)]
    pub tab_size: Option<NonZeroU32>,
    #[serde(default)]
    pub hard_tabs: Option<bool>,
    #[serde(default)]
    pub soft_wrap: Option<SoftWrap>,
    #[serde(default)]
    pub preferred_line_length: Option<u32>,
    #[serde(default)]
    pub format_on_save: Option<FormatOnSave>,
    #[serde(default)]
    pub remove_trailing_whitespace_on_save: Option<bool>,
    #[serde(default)]
    pub ensure_final_newline_on_save: Option<bool>,
    #[serde(default)]
    pub formatter: Option<Formatter>,
    #[serde(default)]
    pub enable_language_server: Option<bool>,
    #[serde(default)]
    pub show_copilot_suggestions: Option<bool>,
    #[serde(default)]
    pub show_whitespaces: Option<ShowWhitespaceSetting>,
    #[serde(default)]
    pub extend_comment_on_newline: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct CopilotSettingsContent {
    #[serde(default)]
    pub disabled_globs: Option<Vec<String>>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FeaturesContent {
    pub copilot: Option<bool>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FormatOnSave {
    On,
    Off,
    LanguageServer,
    External {
        command: Arc<str>,
        arguments: Arc<[String]>,
    },
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ShowWhitespaceSetting {
    Selection,
    None,
    All,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Formatter {
    LanguageServer,
    External {
        command: Arc<str>,
        arguments: Arc<[String]>,
    },
}

impl AllLanguageSettings {
    pub fn language<'a>(&'a self, language_name: Option<&str>) -> &'a LanguageSettings {
        if let Some(name) = language_name {
            if let Some(overrides) = self.languages.get(name) {
                return overrides;
            }
        }
        &self.defaults
    }

    pub fn copilot_enabled_for_path(&self, path: &Path) -> bool {
        !self
            .copilot
            .disabled_globs
            .iter()
            .any(|glob| glob.is_match(path))
    }

    pub fn copilot_enabled(&self, language: Option<&Arc<Language>>, path: Option<&Path>) -> bool {
        if !self.copilot.feature_enabled {
            return false;
        }

        if let Some(path) = path {
            if !self.copilot_enabled_for_path(path) {
                return false;
            }
        }

        self.language(language.map(|l| l.name()).as_deref())
            .show_copilot_suggestions
    }
}

impl settings::Setting for AllLanguageSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = AllLanguageSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_settings: &[&Self::FileContent],
        _: &AppContext,
    ) -> Result<Self> {
        // A default is provided for all settings.
        let mut defaults: LanguageSettings =
            serde_json::from_value(serde_json::to_value(&default_value.defaults)?)?;

        let mut languages = HashMap::default();
        for (language_name, settings) in &default_value.languages {
            let mut language_settings = defaults.clone();
            merge_settings(&mut language_settings, &settings);
            languages.insert(language_name.clone(), language_settings);
        }

        let mut copilot_enabled = default_value
            .features
            .as_ref()
            .and_then(|f| f.copilot)
            .ok_or_else(Self::missing_default)?;
        let mut copilot_globs = default_value
            .copilot
            .as_ref()
            .and_then(|c| c.disabled_globs.as_ref())
            .ok_or_else(Self::missing_default)?;

        for user_settings in user_settings {
            if let Some(copilot) = user_settings.features.as_ref().and_then(|f| f.copilot) {
                copilot_enabled = copilot;
            }
            if let Some(globs) = user_settings
                .copilot
                .as_ref()
                .and_then(|f| f.disabled_globs.as_ref())
            {
                copilot_globs = globs;
            }

            // A user's global settings override the default global settings and
            // all default language-specific settings.
            merge_settings(&mut defaults, &user_settings.defaults);
            for language_settings in languages.values_mut() {
                merge_settings(language_settings, &user_settings.defaults);
            }

            // A user's language-specific settings override default language-specific settings.
            for (language_name, user_language_settings) in &user_settings.languages {
                merge_settings(
                    languages
                        .entry(language_name.clone())
                        .or_insert_with(|| defaults.clone()),
                    &user_language_settings,
                );
            }
        }

        Ok(Self {
            copilot: CopilotSettings {
                feature_enabled: copilot_enabled,
                disabled_globs: copilot_globs
                    .iter()
                    .filter_map(|g| Some(globset::Glob::new(g).ok()?.compile_matcher()))
                    .collect(),
            },
            defaults,
            languages,
        })
    }

    fn json_schema(
        generator: &mut schemars::gen::SchemaGenerator,
        params: &settings::SettingsJsonSchemaParams,
        _: &AppContext,
    ) -> schemars::schema::RootSchema {
        let mut root_schema = generator.root_schema_for::<Self::FileContent>();

        // Create a schema for a 'languages overrides' object, associating editor
        // settings with specific languages.
        assert!(root_schema
            .definitions
            .contains_key("LanguageSettingsContent"));

        let languages_object_schema = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties: params
                    .language_names
                    .iter()
                    .map(|name| {
                        (
                            name.clone(),
                            Schema::new_ref("#/definitions/LanguageSettingsContent".into()),
                        )
                    })
                    .collect(),
                ..Default::default()
            })),
            ..Default::default()
        };

        root_schema
            .definitions
            .extend([("Languages".into(), languages_object_schema.into())]);

        root_schema
            .schema
            .object
            .as_mut()
            .unwrap()
            .properties
            .extend([
                (
                    "languages".to_owned(),
                    Schema::new_ref("#/definitions/Languages".into()),
                ),
                // For backward compatibility
                (
                    "language_overrides".to_owned(),
                    Schema::new_ref("#/definitions/Languages".into()),
                ),
            ]);

        root_schema
    }
}

fn merge_settings(settings: &mut LanguageSettings, src: &LanguageSettingsContent) {
    merge(&mut settings.tab_size, src.tab_size);
    merge(&mut settings.hard_tabs, src.hard_tabs);
    merge(&mut settings.soft_wrap, src.soft_wrap);
    merge(
        &mut settings.preferred_line_length,
        src.preferred_line_length,
    );
    merge(&mut settings.formatter, src.formatter.clone());
    merge(&mut settings.format_on_save, src.format_on_save.clone());
    merge(
        &mut settings.remove_trailing_whitespace_on_save,
        src.remove_trailing_whitespace_on_save,
    );
    merge(
        &mut settings.ensure_final_newline_on_save,
        src.ensure_final_newline_on_save,
    );
    merge(
        &mut settings.enable_language_server,
        src.enable_language_server,
    );
    merge(
        &mut settings.show_copilot_suggestions,
        src.show_copilot_suggestions,
    );
    merge(&mut settings.show_whitespaces, src.show_whitespaces);
    merge(
        &mut settings.extend_comment_on_newline,
        src.extend_comment_on_newline,
    );
    fn merge<T>(target: &mut T, value: Option<T>) {
        if let Some(value) = value {
            *target = value;
        }
    }
}
