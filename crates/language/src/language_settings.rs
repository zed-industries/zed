//! Provides `language`-related settings.

use crate::{File, Language};
use anyhow::Result;
use collections::{HashMap, HashSet};
use globset::GlobMatcher;
use gpui::AppContext;
use schemars::{
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsLocation, SettingsSources};
use std::{num::NonZeroU32, path::Path, sync::Arc};

impl<'a> Into<SettingsLocation<'a>> for &'a dyn File {
    fn into(self) -> SettingsLocation<'a> {
        SettingsLocation {
            worktree_id: self.worktree_id(),
            path: self.path().as_ref(),
        }
    }
}

/// Initializes the language settings.
pub fn init(cx: &mut AppContext) {
    AllLanguageSettings::register(cx);
}

/// Returns the settings for the specified language from the provided file.
pub fn language_settings<'a>(
    language: Option<&Arc<Language>>,
    file: Option<&Arc<dyn File>>,
    cx: &'a AppContext,
) -> &'a LanguageSettings {
    let language_name = language.map(|l| l.name());
    all_language_settings(file, cx).language(language_name.as_deref())
}

/// Returns the settings for all languages from the provided file.
pub fn all_language_settings<'a>(
    file: Option<&Arc<dyn File>>,
    cx: &'a AppContext,
) -> &'a AllLanguageSettings {
    let location = file.map(|f| f.as_ref().into());
    AllLanguageSettings::get(location, cx)
}

/// The settings for all languages.
#[derive(Debug, Clone)]
pub struct AllLanguageSettings {
    /// The settings for GitHub Copilot.
    pub copilot: CopilotSettings,
    defaults: LanguageSettings,
    languages: HashMap<Arc<str>, LanguageSettings>,
    pub(crate) file_types: HashMap<Arc<str>, Vec<String>>,
}

/// The settings for a particular language.
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageSettings {
    /// How many columns a tab should occupy.
    pub tab_size: NonZeroU32,
    /// Whether to indent lines using tab characters, as opposed to multiple
    /// spaces.
    pub hard_tabs: bool,
    /// How to soft-wrap long lines of text.
    pub soft_wrap: SoftWrap,
    /// The column at which to soft-wrap lines, for buffers where soft-wrap
    /// is enabled.
    pub preferred_line_length: u32,
    /// Whether to show wrap guides in the editor. Setting this to true will
    /// show a guide at the 'preferred_line_length' value if softwrap is set to
    /// 'preferred_line_length', and will show any additional guides as specified
    /// by the 'wrap_guides' setting.
    pub show_wrap_guides: bool,
    /// Character counts at which to show wrap guides in the editor.
    pub wrap_guides: Vec<usize>,
    /// Whether or not to perform a buffer format before saving.
    pub format_on_save: FormatOnSave,
    /// Whether or not to remove any trailing whitespace from lines of a buffer
    /// before saving it.
    pub remove_trailing_whitespace_on_save: bool,
    /// Whether or not to ensure there's a single newline at the end of a buffer
    /// when saving it.
    pub ensure_final_newline_on_save: bool,
    /// How to perform a buffer format.
    pub formatter: Formatter,
    /// Zed's Prettier integration settings.
    /// If Prettier is enabled, Zed will use this for its Prettier instance for any applicable file, if
    /// the project has no other Prettier installed.
    pub prettier: HashMap<String, serde_json::Value>,
    /// Whether to use language servers to provide code intelligence.
    pub enable_language_server: bool,
    /// Controls whether Copilot provides suggestion immediately (true)
    /// or waits for a `copilot::Toggle` (false).
    pub show_copilot_suggestions: bool,
    /// Whether to show tabs and spaces in the editor.
    pub show_whitespaces: ShowWhitespaceSetting,
    /// Whether to start a new line with a comment when a previous line is a comment as well.
    pub extend_comment_on_newline: bool,
    /// Inlay hint related settings.
    pub inlay_hints: InlayHintSettings,
    /// Whether to automatically close brackets.
    pub use_autoclose: bool,
    // Controls how the editor handles the autoclosed characters.
    pub always_treat_brackets_as_autoclosed: bool,
    /// Which code actions to run on save
    pub code_actions_on_format: HashMap<String, bool>,
}

/// The settings for [GitHub Copilot](https://github.com/features/copilot).
#[derive(Clone, Debug, Default)]
pub struct CopilotSettings {
    /// Whether Copilot is enabled.
    pub feature_enabled: bool,
    /// A list of globs representing files that Copilot should be disabled for.
    pub disabled_globs: Vec<GlobMatcher>,
}

/// The settings for all languages.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AllLanguageSettingsContent {
    /// The settings for enabling/disabling features.
    #[serde(default)]
    pub features: Option<FeaturesContent>,
    /// The settings for GitHub Copilot.
    #[serde(default)]
    pub copilot: Option<CopilotSettingsContent>,
    /// The default language settings.
    #[serde(flatten)]
    pub defaults: LanguageSettingsContent,
    /// The settings for individual languages.
    #[serde(default, alias = "language_overrides")]
    pub languages: HashMap<Arc<str>, LanguageSettingsContent>,
    /// Settings for associating file extensions and filenames
    /// with languages.
    #[serde(default)]
    pub file_types: HashMap<Arc<str>, Vec<String>>,
}

/// The settings for a particular language.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LanguageSettingsContent {
    /// How many columns a tab should occupy.
    ///
    /// Default: 4
    #[serde(default)]
    pub tab_size: Option<NonZeroU32>,
    /// Whether to indent lines using tab characters, as opposed to multiple
    /// spaces.
    ///
    /// Default: false
    #[serde(default)]
    pub hard_tabs: Option<bool>,
    /// How to soft-wrap long lines of text.
    ///
    /// Default: none
    #[serde(default)]
    pub soft_wrap: Option<SoftWrap>,
    /// The column at which to soft-wrap lines, for buffers where soft-wrap
    /// is enabled.
    ///
    /// Default: 80
    #[serde(default)]
    pub preferred_line_length: Option<u32>,
    /// Whether to show wrap guides in the editor. Setting this to true will
    /// show a guide at the 'preferred_line_length' value if softwrap is set to
    /// 'preferred_line_length', and will show any additional guides as specified
    /// by the 'wrap_guides' setting.
    ///
    /// Default: true
    #[serde(default)]
    pub show_wrap_guides: Option<bool>,
    /// Character counts at which to show wrap guides in the editor.
    ///
    /// Default: []
    #[serde(default)]
    pub wrap_guides: Option<Vec<usize>>,
    /// Whether or not to perform a buffer format before saving.
    ///
    /// Default: on
    #[serde(default)]
    pub format_on_save: Option<FormatOnSave>,
    /// Whether or not to remove any trailing whitespace from lines of a buffer
    /// before saving it.
    ///
    /// Default: true
    #[serde(default)]
    pub remove_trailing_whitespace_on_save: Option<bool>,
    /// Whether or not to ensure there's a single newline at the end of a buffer
    /// when saving it.
    ///
    /// Default: true
    #[serde(default)]
    pub ensure_final_newline_on_save: Option<bool>,
    /// How to perform a buffer format.
    ///
    /// Default: auto
    #[serde(default)]
    pub formatter: Option<Formatter>,
    /// Zed's Prettier integration settings.
    /// If Prettier is enabled, Zed will use this for its Prettier instance for any applicable file, if
    /// the project has no other Prettier installed.
    ///
    /// Default: {}
    #[serde(default)]
    pub prettier: Option<HashMap<String, serde_json::Value>>,
    /// Whether to use language servers to provide code intelligence.
    ///
    /// Default: true
    #[serde(default)]
    pub enable_language_server: Option<bool>,
    /// Controls whether Copilot provides suggestion immediately (true)
    /// or waits for a `copilot::Toggle` (false).
    ///
    /// Default: true
    #[serde(default)]
    pub show_copilot_suggestions: Option<bool>,
    /// Whether to show tabs and spaces in the editor.
    #[serde(default)]
    pub show_whitespaces: Option<ShowWhitespaceSetting>,
    /// Whether to start a new line with a comment when a previous line is a comment as well.
    ///
    /// Default: true
    #[serde(default)]
    pub extend_comment_on_newline: Option<bool>,
    /// Inlay hint related settings.
    #[serde(default)]
    pub inlay_hints: Option<InlayHintSettings>,
    /// Whether to automatically type closing characters for you. For example,
    /// when you type (, Zed will automatically add a closing ) at the correct position.
    ///
    /// Default: true
    pub use_autoclose: Option<bool>,
    // Controls how the editor handles the autoclosed characters.
    // When set to `false`(default), skipping over and auto-removing of the closing characters
    // happen only for auto-inserted characters.
    // Otherwise(when `true`), the closing characters are always skipped over and auto-removed
    // no matter how they were inserted.
    ///
    /// Default: false
    pub always_treat_brackets_as_autoclosed: Option<bool>,
    /// Which code actions to run on save after the formatter.
    /// These are not run if formatting is off.
    ///
    /// Default: {} (or {"source.organizeImports": true} for Go).
    pub code_actions_on_format: Option<HashMap<String, bool>>,
}

/// The contents of the GitHub Copilot settings.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct CopilotSettingsContent {
    /// A list of globs representing files that Copilot should be disabled for.
    #[serde(default)]
    pub disabled_globs: Option<Vec<String>>,
}

/// The settings for enabling/disabling features.
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FeaturesContent {
    /// Whether the GitHub Copilot feature is enabled.
    pub copilot: Option<bool>,
}

/// Controls the soft-wrapping behavior in the editor.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    /// Do not soft wrap.
    None,
    /// Soft wrap lines that overflow the editor
    EditorWidth,
    /// Soft wrap lines at the preferred line length
    PreferredLineLength,
}

/// Controls the behavior of formatting files when they are saved.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FormatOnSave {
    /// Files should be formatted on save.
    On,
    /// Files should not be formatted on save.
    Off,
    /// Files should be formatted using the current language server.
    LanguageServer,
    /// The external program to use to format the files on save.
    External {
        /// The external program to run.
        command: Arc<str>,
        /// The arguments to pass to the program.
        arguments: Arc<[String]>,
    },
    /// Files should be formatted using code actions executed by language servers.
    CodeActions(HashMap<String, bool>),
}

/// Controls how whitespace should be displayedin the editor.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ShowWhitespaceSetting {
    /// Draw whitespace only for the selected text.
    Selection,
    /// Do not draw any tabs or spaces.
    None,
    /// Draw all invisible symbols.
    All,
}

/// Controls which formatter should be used when formatting code.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Formatter {
    /// Format files using Zed's Prettier integration (if applicable),
    /// or falling back to formatting via language server.
    #[default]
    Auto,
    /// Format code using the current language server.
    LanguageServer,
    /// Format code using Zed's Prettier integration.
    Prettier,
    /// Format code using an external command.
    External {
        /// The external program to run.
        command: Arc<str>,
        /// The arguments to pass to the program.
        arguments: Arc<[String]>,
    },
    /// Files should be formatted using code actions executed by language servers.
    CodeActions(HashMap<String, bool>),
}

/// The settings for inlay hints.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct InlayHintSettings {
    /// Global switch to toggle hints on and off.
    ///
    /// Default: false
    #[serde(default)]
    pub enabled: bool,
    /// Whether type hints should be shown.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub show_type_hints: bool,
    /// Whether parameter hints should be shown.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub show_parameter_hints: bool,
    /// Whether other hints should be shown.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub show_other_hints: bool,
    /// Whether or not to debounce inlay hints updates after buffer edits.
    ///
    /// Set to 0 to disable debouncing.
    ///
    /// Default: 700
    #[serde(default = "edit_debounce_ms")]
    pub edit_debounce_ms: u64,
    /// Whether or not to debounce inlay hints updates after buffer scrolls.
    ///
    /// Set to 0 to disable debouncing.
    ///
    /// Default: 50
    #[serde(default = "scroll_debounce_ms")]
    pub scroll_debounce_ms: u64,
}

fn default_true() -> bool {
    true
}

fn edit_debounce_ms() -> u64 {
    700
}

fn scroll_debounce_ms() -> u64 {
    50
}

impl InlayHintSettings {
    /// Returns the kinds of inlay hints that are enabled based on the settings.
    pub fn enabled_inlay_hint_kinds(&self) -> HashSet<Option<InlayHintKind>> {
        let mut kinds = HashSet::default();
        if self.show_type_hints {
            kinds.insert(Some(InlayHintKind::Type));
        }
        if self.show_parameter_hints {
            kinds.insert(Some(InlayHintKind::Parameter));
        }
        if self.show_other_hints {
            kinds.insert(None);
        }
        kinds
    }
}

impl AllLanguageSettings {
    /// Returns the [`LanguageSettings`] for the language with the specified name.
    pub fn language<'a>(&'a self, language_name: Option<&str>) -> &'a LanguageSettings {
        if let Some(name) = language_name {
            if let Some(overrides) = self.languages.get(name) {
                return overrides;
            }
        }
        &self.defaults
    }

    /// Returns whether GitHub Copilot is enabled for the given path.
    pub fn copilot_enabled_for_path(&self, path: &Path) -> bool {
        !self
            .copilot
            .disabled_globs
            .iter()
            .any(|glob| glob.is_match(path))
    }

    /// Returns whether GitHub Copilot is enabled for the given language and path.
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

/// The kind of an inlay hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InlayHintKind {
    /// An inlay hint for a type.
    Type,
    /// An inlay hint for a parameter.
    Parameter,
}

impl InlayHintKind {
    /// Returns the [`InlayHintKind`] from the given name.
    ///
    /// Returns `None` if `name` does not match any of the expected
    /// string representations.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "type" => Some(InlayHintKind::Type),
            "parameter" => Some(InlayHintKind::Parameter),
            _ => None,
        }
    }

    /// Returns the name of this [`InlayHintKind`].
    pub fn name(&self) -> &'static str {
        match self {
            InlayHintKind::Type => "type",
            InlayHintKind::Parameter => "parameter",
        }
    }
}

impl settings::Settings for AllLanguageSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = AllLanguageSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        let default_value = sources.default;

        // A default is provided for all settings.
        let mut defaults: LanguageSettings =
            serde_json::from_value(serde_json::to_value(&default_value.defaults)?)?;

        let mut languages = HashMap::default();
        for (language_name, settings) in &default_value.languages {
            let mut language_settings = defaults.clone();
            merge_settings(&mut language_settings, settings);
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

        let mut file_types: HashMap<Arc<str>, Vec<String>> = HashMap::default();
        for user_settings in sources.customizations() {
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
                    user_language_settings,
                );
            }

            for (language, suffixes) in &user_settings.file_types {
                file_types
                    .entry(language.clone())
                    .or_default()
                    .extend_from_slice(suffixes);
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
            file_types,
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
    merge(&mut settings.use_autoclose, src.use_autoclose);
    merge(
        &mut settings.always_treat_brackets_as_autoclosed,
        src.always_treat_brackets_as_autoclosed,
    );
    merge(&mut settings.show_wrap_guides, src.show_wrap_guides);
    merge(&mut settings.wrap_guides, src.wrap_guides.clone());
    merge(
        &mut settings.code_actions_on_format,
        src.code_actions_on_format.clone(),
    );

    merge(
        &mut settings.preferred_line_length,
        src.preferred_line_length,
    );
    merge(&mut settings.formatter, src.formatter.clone());
    merge(&mut settings.prettier, src.prettier.clone());
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
    merge(&mut settings.inlay_hints, src.inlay_hints);
    fn merge<T>(target: &mut T, value: Option<T>) {
        if let Some(value) = value {
            *target = value;
        }
    }
}
