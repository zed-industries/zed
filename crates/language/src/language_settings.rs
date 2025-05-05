//! Provides `language`-related settings.

use crate::{File, Language, LanguageName, LanguageServerName};
use anyhow::Result;
use collections::{FxHashMap, HashMap, HashSet};
use core::slice;
use ec4rs::{
    Properties as EditorconfigProperties,
    property::{FinalNewline, IndentSize, IndentStyle, TabWidth, TrimTrailingWs},
};
use globset::{Glob, GlobMatcher, GlobSet, GlobSetBuilder};
use gpui::{App, Modifiers};
use itertools::{Either, Itertools};
use schemars::{
    JsonSchema,
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec},
};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, IntoDeserializer, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;
use settings::{
    Settings, SettingsLocation, SettingsSources, SettingsStore, add_references_to_properties,
};
use std::{borrow::Cow, num::NonZeroU32, path::Path, sync::Arc};
use util::serde::default_true;

/// Initializes the language settings.
pub fn init(cx: &mut App) {
    AllLanguageSettings::register(cx);
}

/// Returns the settings for the specified language from the provided file.
pub fn language_settings<'a>(
    language: Option<LanguageName>,
    file: Option<&'a Arc<dyn File>>,
    cx: &'a App,
) -> Cow<'a, LanguageSettings> {
    let location = file.map(|f| SettingsLocation {
        worktree_id: f.worktree_id(cx),
        path: f.path().as_ref(),
    });
    AllLanguageSettings::get(location, cx).language(location, language.as_ref(), cx)
}

/// Returns the settings for all languages from the provided file.
pub fn all_language_settings<'a>(
    file: Option<&'a Arc<dyn File>>,
    cx: &'a App,
) -> &'a AllLanguageSettings {
    let location = file.map(|f| SettingsLocation {
        worktree_id: f.worktree_id(cx),
        path: f.path().as_ref(),
    });
    AllLanguageSettings::get(location, cx)
}

/// The settings for all languages.
#[derive(Debug, Clone)]
pub struct AllLanguageSettings {
    /// The edit prediction settings.
    pub edit_predictions: EditPredictionSettings,
    pub defaults: LanguageSettings,
    languages: HashMap<LanguageName, LanguageSettings>,
    pub(crate) file_types: FxHashMap<Arc<str>, GlobSet>,
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
    /// Whether to show wrap guides (vertical rulers) in the editor.
    /// Setting this to true will show a guide at the 'preferred_line_length' value
    /// if softwrap is set to 'preferred_line_length', and will show any
    /// additional guides as specified by the 'wrap_guides' setting.
    pub show_wrap_guides: bool,
    /// Character counts at which to show wrap guides (vertical rulers) in the editor.
    pub wrap_guides: Vec<usize>,
    /// Indent guide related settings.
    pub indent_guides: IndentGuideSettings,
    /// Whether or not to perform a buffer format before saving.
    pub format_on_save: FormatOnSave,
    /// Whether or not to remove any trailing whitespace from lines of a buffer
    /// before saving it.
    pub remove_trailing_whitespace_on_save: bool,
    /// Whether or not to ensure there's a single newline at the end of a buffer
    /// when saving it.
    pub ensure_final_newline_on_save: bool,
    /// How to perform a buffer format.
    pub formatter: SelectedFormatter,
    /// Zed's Prettier integration settings.
    pub prettier: PrettierSettings,
    /// Whether to automatically close JSX tags.
    pub jsx_tag_auto_close: JsxTagAutoCloseSettings,
    /// Whether to use language servers to provide code intelligence.
    pub enable_language_server: bool,
    /// The list of language servers to use (or disable) for this language.
    ///
    /// This array should consist of language server IDs, as well as the following
    /// special tokens:
    /// - `"!<language_server_id>"` - A language server ID prefixed with a `!` will be disabled.
    /// - `"..."` - A placeholder to refer to the **rest** of the registered language servers for this language.
    pub language_servers: Vec<String>,
    /// Controls where the `editor::Rewrap` action is allowed for this language.
    ///
    /// Note: This setting has no effect in Vim mode, as rewrap is already
    /// allowed everywhere.
    pub allow_rewrap: RewrapBehavior,
    /// Controls whether edit predictions are shown immediately (true)
    /// or manually by triggering `editor::ShowEditPrediction` (false).
    pub show_edit_predictions: bool,
    /// Controls whether edit predictions are shown in the given language
    /// scopes.
    pub edit_predictions_disabled_in: Vec<String>,
    /// Whether to show tabs and spaces in the editor.
    pub show_whitespaces: ShowWhitespaceSetting,
    /// Whether to start a new line with a comment when a previous line is a comment as well.
    pub extend_comment_on_newline: bool,
    /// Inlay hint related settings.
    pub inlay_hints: InlayHintSettings,
    /// Whether to automatically close brackets.
    pub use_autoclose: bool,
    /// Whether to automatically surround text with brackets.
    pub use_auto_surround: bool,
    /// Whether to use additional LSP queries to format (and amend) the code after
    /// every "trigger" symbol input, defined by LSP server capabilities.
    pub use_on_type_format: bool,
    /// Whether indentation of pasted content should be adjusted based on the context.
    pub auto_indent_on_paste: bool,
    /// Controls how the editor handles the autoclosed characters.
    pub always_treat_brackets_as_autoclosed: bool,
    /// Which code actions to run on save
    pub code_actions_on_format: HashMap<String, bool>,
    /// Whether to perform linked edits
    pub linked_edits: bool,
    /// Task configuration for this language.
    pub tasks: LanguageTaskConfig,
    /// Whether to pop the completions menu while typing in an editor without
    /// explicitly requesting it.
    pub show_completions_on_input: bool,
    /// Whether to display inline and alongside documentation for items in the
    /// completions menu.
    pub show_completion_documentation: bool,
    /// Completion settings for this language.
    pub completions: CompletionSettings,
    /// Prefered debuggers for this language.
    pub debuggers: Vec<String>,
}

impl LanguageSettings {
    /// A token representing the rest of the available language servers.
    const REST_OF_LANGUAGE_SERVERS: &'static str = "...";

    /// Returns the customized list of language servers from the list of
    /// available language servers.
    pub fn customized_language_servers(
        &self,
        available_language_servers: &[LanguageServerName],
    ) -> Vec<LanguageServerName> {
        Self::resolve_language_servers(&self.language_servers, available_language_servers)
    }

    pub(crate) fn resolve_language_servers(
        configured_language_servers: &[String],
        available_language_servers: &[LanguageServerName],
    ) -> Vec<LanguageServerName> {
        let (disabled_language_servers, enabled_language_servers): (
            Vec<LanguageServerName>,
            Vec<LanguageServerName>,
        ) = configured_language_servers.iter().partition_map(
            |language_server| match language_server.strip_prefix('!') {
                Some(disabled) => Either::Left(LanguageServerName(disabled.to_string().into())),
                None => Either::Right(LanguageServerName(language_server.clone().into())),
            },
        );

        let rest = available_language_servers
            .iter()
            .filter(|&available_language_server| {
                !disabled_language_servers.contains(&available_language_server)
                    && !enabled_language_servers.contains(&available_language_server)
            })
            .cloned()
            .collect::<Vec<_>>();

        enabled_language_servers
            .into_iter()
            .flat_map(|language_server| {
                if language_server.0.as_ref() == Self::REST_OF_LANGUAGE_SERVERS {
                    rest.clone()
                } else {
                    vec![language_server.clone()]
                }
            })
            .collect::<Vec<_>>()
    }
}

/// The provider that supplies edit predictions.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EditPredictionProvider {
    None,
    #[default]
    Copilot,
    Supermaven,
    Zed,
}

impl EditPredictionProvider {
    pub fn is_zed(&self) -> bool {
        match self {
            EditPredictionProvider::Zed => true,
            EditPredictionProvider::None
            | EditPredictionProvider::Copilot
            | EditPredictionProvider::Supermaven => false,
        }
    }
}

/// The settings for edit predictions, such as [GitHub Copilot](https://github.com/features/copilot)
/// or [Supermaven](https://supermaven.com).
#[derive(Clone, Debug, Default)]
pub struct EditPredictionSettings {
    /// The provider that supplies edit predictions.
    pub provider: EditPredictionProvider,
    /// A list of globs representing files that edit predictions should be disabled for.
    /// This list adds to a pre-existing, sensible default set of globs.
    /// Any additional ones you add are combined with them.
    pub disabled_globs: Vec<DisabledGlob>,
    /// Configures how edit predictions are displayed in the buffer.
    pub mode: EditPredictionsMode,
    /// Settings specific to GitHub Copilot.
    pub copilot: CopilotSettings,
    /// Whether edit predictions are enabled in the assistant panel.
    /// This setting has no effect if globally disabled.
    pub enabled_in_assistant: bool,
}

impl EditPredictionSettings {
    /// Returns whether edit predictions are enabled for the given path.
    pub fn enabled_for_file(&self, file: &Arc<dyn File>, cx: &App) -> bool {
        !self.disabled_globs.iter().any(|glob| {
            if glob.is_absolute {
                file.as_local()
                    .map_or(false, |local| glob.matcher.is_match(local.abs_path(cx)))
            } else {
                glob.matcher.is_match(file.path())
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct DisabledGlob {
    matcher: GlobMatcher,
    is_absolute: bool,
}

/// The mode in which edit predictions should be displayed.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EditPredictionsMode {
    /// If provider supports it, display inline when holding modifier key (e.g., alt).
    /// Otherwise, eager preview is used.
    #[serde(alias = "auto")]
    Subtle,
    /// Display inline when there are no language server completions available.
    #[default]
    #[serde(alias = "eager_preview")]
    Eager,
}

#[derive(Clone, Debug, Default)]
pub struct CopilotSettings {
    /// HTTP/HTTPS proxy to use for Copilot.
    pub proxy: Option<String>,
    /// Disable certificate verification for proxy (not recommended).
    pub proxy_no_verify: Option<bool>,
}

/// The settings for all languages.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AllLanguageSettingsContent {
    /// The settings for enabling/disabling features.
    #[serde(default)]
    pub features: Option<FeaturesContent>,
    /// The edit prediction settings.
    #[serde(default)]
    pub edit_predictions: Option<EditPredictionSettingsContent>,
    /// The default language settings.
    #[serde(flatten)]
    pub defaults: LanguageSettingsContent,
    /// The settings for individual languages.
    #[serde(default)]
    pub languages: HashMap<LanguageName, LanguageSettingsContent>,
    /// Settings for associating file extensions and filenames
    /// with languages.
    #[serde(default)]
    pub file_types: HashMap<Arc<str>, Vec<String>>,
}

/// Controls how completions are processed for this language.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CompletionSettings {
    /// Controls how words are completed.
    /// For large documents, not all words may be fetched for completion.
    ///
    /// Default: `fallback`
    #[serde(default = "default_words_completion_mode")]
    pub words: WordsCompletionMode,
    /// Whether to fetch LSP completions or not.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub lsp: bool,
    /// When fetching LSP completions, determines how long to wait for a response of a particular server.
    /// When set to 0, waits indefinitely.
    ///
    /// Default: 0
    #[serde(default = "default_lsp_fetch_timeout_ms")]
    pub lsp_fetch_timeout_ms: u64,
    /// Controls how LSP completions are inserted.
    ///
    /// Default: "replace_suffix"
    #[serde(default = "default_lsp_insert_mode")]
    pub lsp_insert_mode: LspInsertMode,
}

/// Controls how document's words are completed.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WordsCompletionMode {
    /// Always fetch document's words for completions along with LSP completions.
    Enabled,
    /// Only if LSP response errors or times out,
    /// use document's words to show completions.
    Fallback,
    /// Never fetch or complete document's words for completions.
    /// (Word-based completions can still be queried via a separate action)
    Disabled,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LspInsertMode {
    /// Replaces text before the cursor, using the `insert` range described in the LSP specification.
    Insert,
    /// Replaces text before and after the cursor, using the `replace` range described in the LSP specification.
    Replace,
    /// Behaves like `"replace"` if the text that would be replaced is a subsequence of the completion text,
    /// and like `"insert"` otherwise.
    ReplaceSubsequence,
    /// Behaves like `"replace"` if the text after the cursor is a suffix of the completion, and like
    /// `"insert"` otherwise.
    ReplaceSuffix,
}

fn default_words_completion_mode() -> WordsCompletionMode {
    WordsCompletionMode::Fallback
}

fn default_lsp_insert_mode() -> LspInsertMode {
    LspInsertMode::ReplaceSuffix
}

fn default_lsp_fetch_timeout_ms() -> u64 {
    0
}

/// The settings for a particular language.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
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
    /// Indent guide related settings.
    #[serde(default)]
    pub indent_guides: Option<IndentGuideSettings>,
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
    pub formatter: Option<SelectedFormatter>,
    /// Zed's Prettier integration settings.
    /// Allows to enable/disable formatting with Prettier
    /// and configure default Prettier, used when no project-level Prettier installation is found.
    ///
    /// Default: off
    #[serde(default)]
    pub prettier: Option<PrettierSettings>,
    /// Whether to automatically close JSX tags.
    #[serde(default)]
    pub jsx_tag_auto_close: Option<JsxTagAutoCloseSettings>,
    /// Whether to use language servers to provide code intelligence.
    ///
    /// Default: true
    #[serde(default)]
    pub enable_language_server: Option<bool>,
    /// The list of language servers to use (or disable) for this language.
    ///
    /// This array should consist of language server IDs, as well as the following
    /// special tokens:
    /// - `"!<language_server_id>"` - A language server ID prefixed with a `!` will be disabled.
    /// - `"..."` - A placeholder to refer to the **rest** of the registered language servers for this language.
    ///
    /// Default: ["..."]
    #[serde(default)]
    pub language_servers: Option<Vec<String>>,
    /// Controls where the `editor::Rewrap` action is allowed for this language.
    ///
    /// Note: This setting has no effect in Vim mode, as rewrap is already
    /// allowed everywhere.
    ///
    /// Default: "in_comments"
    #[serde(default)]
    pub allow_rewrap: Option<RewrapBehavior>,
    /// Controls whether edit predictions are shown immediately (true)
    /// or manually by triggering `editor::ShowEditPrediction` (false).
    ///
    /// Default: true
    #[serde(default)]
    pub show_edit_predictions: Option<bool>,
    /// Controls whether edit predictions are shown in the given language
    /// scopes.
    ///
    /// Example: ["string", "comment"]
    ///
    /// Default: []
    #[serde(default)]
    pub edit_predictions_disabled_in: Option<Vec<String>>,
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
    /// Whether to automatically surround text with characters for you. For example,
    /// when you select text and type (, Zed will automatically surround text with ().
    ///
    /// Default: true
    pub use_auto_surround: Option<bool>,
    /// Controls how the editor handles the autoclosed characters.
    /// When set to `false`(default), skipping over and auto-removing of the closing characters
    /// happen only for auto-inserted characters.
    /// Otherwise(when `true`), the closing characters are always skipped over and auto-removed
    /// no matter how they were inserted.
    ///
    /// Default: false
    pub always_treat_brackets_as_autoclosed: Option<bool>,
    /// Whether to use additional LSP queries to format (and amend) the code after
    /// every "trigger" symbol input, defined by LSP server capabilities.
    ///
    /// Default: true
    pub use_on_type_format: Option<bool>,
    /// Which code actions to run on save after the formatter.
    /// These are not run if formatting is off.
    ///
    /// Default: {} (or {"source.organizeImports": true} for Go).
    pub code_actions_on_format: Option<HashMap<String, bool>>,
    /// Whether to perform linked edits of associated ranges, if the language server supports it.
    /// For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well.
    ///
    /// Default: true
    pub linked_edits: Option<bool>,
    /// Whether indentation of pasted content should be adjusted based on the context.
    ///
    /// Default: true
    pub auto_indent_on_paste: Option<bool>,
    /// Task configuration for this language.
    ///
    /// Default: {}
    pub tasks: Option<LanguageTaskConfig>,
    /// Whether to pop the completions menu while typing in an editor without
    /// explicitly requesting it.
    ///
    /// Default: true
    pub show_completions_on_input: Option<bool>,
    /// Whether to display inline and alongside documentation for items in the
    /// completions menu.
    ///
    /// Default: true
    pub show_completion_documentation: Option<bool>,
    /// Controls how completions are processed for this language.
    pub completions: Option<CompletionSettings>,
    /// Prefered debuggers for this language.
    ///
    /// Default: []
    pub debuggers: Option<Vec<String>>,
}

/// The behavior of `editor::Rewrap`.
#[derive(Debug, PartialEq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RewrapBehavior {
    /// Only rewrap within comments.
    #[default]
    InComments,
    /// Only rewrap within the current selection(s).
    InSelections,
    /// Allow rewrapping anywhere.
    Anywhere,
}

/// The contents of the edit prediction settings.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct EditPredictionSettingsContent {
    /// A list of globs representing files that edit predictions should be disabled for.
    /// This list adds to a pre-existing, sensible default set of globs.
    /// Any additional ones you add are combined with them.
    #[serde(default)]
    pub disabled_globs: Option<Vec<String>>,
    /// The mode used to display edit predictions in the buffer.
    /// Provider support required.
    #[serde(default)]
    pub mode: EditPredictionsMode,
    /// Settings specific to GitHub Copilot.
    #[serde(default)]
    pub copilot: CopilotSettingsContent,
    /// Whether edit predictions are enabled in the assistant prompt editor.
    /// This has no effect if globally disabled.
    #[serde(default = "default_true")]
    pub enabled_in_assistant: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CopilotSettingsContent {
    /// HTTP/HTTPS proxy to use for Copilot.
    ///
    /// Default: none
    #[serde(default)]
    pub proxy: Option<String>,
    /// Disable certificate verification for the proxy (not recommended).
    ///
    /// Default: false
    #[serde(default)]
    pub proxy_no_verify: Option<bool>,
}

/// The settings for enabling/disabling features.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FeaturesContent {
    /// Determines which edit prediction provider to use.
    pub edit_prediction_provider: Option<EditPredictionProvider>,
}

/// Controls the soft-wrapping behavior in the editor.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    /// Prefer a single line generally, unless an overly long line is encountered.
    None,
    /// Deprecated: use None instead. Left to avoid breaking existing users' configs.
    /// Prefer a single line generally, unless an overly long line is encountered.
    PreferLine,
    /// Soft wrap lines that exceed the editor width.
    EditorWidth,
    /// Soft wrap lines at the preferred line length.
    PreferredLineLength,
    /// Soft wrap line at the preferred line length or the editor width (whichever is smaller).
    Bounded,
}

/// Controls the behavior of formatting files when they are saved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatOnSave {
    /// Files should be formatted on save.
    On,
    /// Files should not be formatted on save.
    Off,
    List(FormatterList),
}

impl JsonSchema for FormatOnSave {
    fn schema_name() -> String {
        "OnSaveFormatter".into()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> Schema {
        let mut schema = SchemaObject::default();
        let formatter_schema = Formatter::json_schema(generator);
        schema.instance_type = Some(
            vec![
                InstanceType::Object,
                InstanceType::String,
                InstanceType::Array,
            ]
            .into(),
        );

        let valid_raw_values = SchemaObject {
            enum_values: Some(vec![
                Value::String("on".into()),
                Value::String("off".into()),
                Value::String("prettier".into()),
                Value::String("language_server".into()),
            ]),
            ..Default::default()
        };
        let mut nested_values = SchemaObject::default();

        nested_values.array().items = Some(formatter_schema.clone().into());

        schema.subschemas().any_of = Some(vec![
            nested_values.into(),
            valid_raw_values.into(),
            formatter_schema,
        ]);
        schema.into()
    }
}

impl Serialize for FormatOnSave {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::On => serializer.serialize_str("on"),
            Self::Off => serializer.serialize_str("off"),
            Self::List(list) => list.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for FormatOnSave {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormatDeserializer;

        impl<'d> Visitor<'d> for FormatDeserializer {
            type Value = FormatOnSave;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid on-save formatter kind")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == "on" {
                    Ok(Self::Value::On)
                } else if v == "off" {
                    Ok(Self::Value::Off)
                } else if v == "language_server" {
                    Ok(Self::Value::List(FormatterList(
                        Formatter::LanguageServer { name: None }.into(),
                    )))
                } else {
                    let ret: Result<FormatterList, _> =
                        Deserialize::deserialize(v.into_deserializer());
                    ret.map(Self::Value::List)
                }
            }
            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'d>,
            {
                let ret: Result<FormatterList, _> =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map));
                ret.map(Self::Value::List)
            }
            fn visit_seq<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'d>,
            {
                let ret: Result<FormatterList, _> =
                    Deserialize::deserialize(de::value::SeqAccessDeserializer::new(map));
                ret.map(Self::Value::List)
            }
        }
        deserializer.deserialize_any(FormatDeserializer)
    }
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
    /// Draw whitespaces at boundaries only.
    ///
    /// For a whitespace to be on a boundary, any of the following conditions need to be met:
    /// - It is a tab
    /// - It is adjacent to an edge (start or end)
    /// - It is adjacent to a whitespace (left or right)
    Boundary,
}

/// Controls which formatter should be used when formatting code.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SelectedFormatter {
    /// Format files using Zed's Prettier integration (if applicable),
    /// or falling back to formatting via language server.
    #[default]
    Auto,
    List(FormatterList),
}

impl JsonSchema for SelectedFormatter {
    fn schema_name() -> String {
        "Formatter".into()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> Schema {
        let mut schema = SchemaObject::default();
        let formatter_schema = Formatter::json_schema(generator);
        schema.instance_type = Some(
            vec![
                InstanceType::Object,
                InstanceType::String,
                InstanceType::Array,
            ]
            .into(),
        );

        let valid_raw_values = SchemaObject {
            enum_values: Some(vec![
                Value::String("auto".into()),
                Value::String("prettier".into()),
                Value::String("language_server".into()),
            ]),
            ..Default::default()
        };

        let mut nested_values = SchemaObject::default();

        nested_values.array().items = Some(formatter_schema.clone().into());

        schema.subschemas().any_of = Some(vec![
            nested_values.into(),
            valid_raw_values.into(),
            formatter_schema,
        ]);
        schema.into()
    }
}

impl Serialize for SelectedFormatter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SelectedFormatter::Auto => serializer.serialize_str("auto"),
            SelectedFormatter::List(list) => list.serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for SelectedFormatter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormatDeserializer;

        impl<'d> Visitor<'d> for FormatDeserializer {
            type Value = SelectedFormatter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid formatter kind")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == "auto" {
                    Ok(Self::Value::Auto)
                } else if v == "language_server" {
                    Ok(Self::Value::List(FormatterList(
                        Formatter::LanguageServer { name: None }.into(),
                    )))
                } else {
                    let ret: Result<FormatterList, _> =
                        Deserialize::deserialize(v.into_deserializer());
                    ret.map(SelectedFormatter::List)
                }
            }
            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'d>,
            {
                let ret: Result<FormatterList, _> =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map));
                ret.map(SelectedFormatter::List)
            }
            fn visit_seq<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'d>,
            {
                let ret: Result<FormatterList, _> =
                    Deserialize::deserialize(de::value::SeqAccessDeserializer::new(map));
                ret.map(SelectedFormatter::List)
            }
        }
        deserializer.deserialize_any(FormatDeserializer)
    }
}
/// Controls which formatter should be used when formatting code.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case", transparent)]
pub struct FormatterList(pub SingleOrVec<Formatter>);

impl AsRef<[Formatter]> for FormatterList {
    fn as_ref(&self) -> &[Formatter] {
        match &self.0 {
            SingleOrVec::Single(single) => slice::from_ref(single),
            SingleOrVec::Vec(v) => v,
        }
    }
}

/// Controls which formatter should be used when formatting code. If there are multiple formatters, they are executed in the order of declaration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Formatter {
    /// Format code using the current language server.
    LanguageServer { name: Option<String> },
    /// Format code using Zed's Prettier integration.
    Prettier,
    /// Format code using an external command.
    External {
        /// The external program to run.
        command: Arc<str>,
        /// The arguments to pass to the program.
        arguments: Option<Arc<[String]>>,
    },
    /// Files should be formatted using code actions executed by language servers.
    CodeActions(HashMap<String, bool>),
}

/// The settings for indent guides.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IndentGuideSettings {
    /// Whether to display indent guides in the editor.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// The width of the indent guides in pixels, between 1 and 10.
    ///
    /// Default: 1
    #[serde(default = "line_width")]
    pub line_width: u32,
    /// The width of the active indent guide in pixels, between 1 and 10.
    ///
    /// Default: 1
    #[serde(default = "active_line_width")]
    pub active_line_width: u32,
    /// Determines how indent guides are colored.
    ///
    /// Default: Fixed
    #[serde(default)]
    pub coloring: IndentGuideColoring,
    /// Determines how indent guide backgrounds are colored.
    ///
    /// Default: Disabled
    #[serde(default)]
    pub background_coloring: IndentGuideBackgroundColoring,
}

fn line_width() -> u32 {
    1
}

fn active_line_width() -> u32 {
    line_width()
}

/// Determines how indent guides are colored.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IndentGuideColoring {
    /// Do not render any lines for indent guides.
    Disabled,
    /// Use the same color for all indentation levels.
    #[default]
    Fixed,
    /// Use a different color for each indentation level.
    IndentAware,
}

/// Determines how indent guide backgrounds are colored.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IndentGuideBackgroundColoring {
    /// Do not render any background for indent guides.
    #[default]
    Disabled,
    /// Use a different color for each indentation level.
    IndentAware,
}

/// The settings for inlay hints.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct InlayHintSettings {
    /// Global switch to toggle hints on and off.
    ///
    /// Default: false
    #[serde(default)]
    pub enabled: bool,
    /// Global switch to toggle inline values on and off.
    ///
    /// Default: false
    #[serde(default)]
    pub show_value_hints: bool,
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
    /// Whether to show a background for inlay hints.
    ///
    /// If set to `true`, the background will use the `hint.background` color
    /// from the current theme.
    ///
    /// Default: false
    #[serde(default)]
    pub show_background: bool,
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
    /// Toggles inlay hints (hides or shows) when the user presses the modifiers specified.
    /// If only a subset of the modifiers specified is pressed, hints are not toggled.
    /// If no modifiers are specified, this is equivalent to `None`.
    ///
    /// Default: None
    #[serde(default)]
    pub toggle_on_modifiers_press: Option<Modifiers>,
}

fn edit_debounce_ms() -> u64 {
    700
}

fn scroll_debounce_ms() -> u64 {
    50
}

/// The task settings for a particular language.
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct LanguageTaskConfig {
    /// Extra task variables to set for a particular language.
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
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
    pub fn language<'a>(
        &'a self,
        location: Option<SettingsLocation<'a>>,
        language_name: Option<&LanguageName>,
        cx: &'a App,
    ) -> Cow<'a, LanguageSettings> {
        let settings = language_name
            .and_then(|name| self.languages.get(name))
            .unwrap_or(&self.defaults);

        let editorconfig_properties = location.and_then(|location| {
            cx.global::<SettingsStore>()
                .editorconfig_properties(location.worktree_id, location.path)
        });
        if let Some(editorconfig_properties) = editorconfig_properties {
            let mut settings = settings.clone();
            merge_with_editorconfig(&mut settings, &editorconfig_properties);
            Cow::Owned(settings)
        } else {
            Cow::Borrowed(settings)
        }
    }

    /// Returns whether edit predictions are enabled for the given path.
    pub fn edit_predictions_enabled_for_file(&self, file: &Arc<dyn File>, cx: &App) -> bool {
        self.edit_predictions.enabled_for_file(file, cx)
    }

    /// Returns whether edit predictions are enabled for the given language and path.
    pub fn show_edit_predictions(&self, language: Option<&Arc<Language>>, cx: &App) -> bool {
        self.language(None, language.map(|l| l.name()).as_ref(), cx)
            .show_edit_predictions
    }

    /// Returns the edit predictions preview mode for the given language and path.
    pub fn edit_predictions_mode(&self) -> EditPredictionsMode {
        self.edit_predictions.mode
    }
}

fn merge_with_editorconfig(settings: &mut LanguageSettings, cfg: &EditorconfigProperties) {
    let tab_size = cfg.get::<IndentSize>().ok().and_then(|v| match v {
        IndentSize::Value(u) => NonZeroU32::new(u as u32),
        IndentSize::UseTabWidth => cfg.get::<TabWidth>().ok().and_then(|w| match w {
            TabWidth::Value(u) => NonZeroU32::new(u as u32),
        }),
    });
    let hard_tabs = cfg
        .get::<IndentStyle>()
        .map(|v| v.eq(&IndentStyle::Tabs))
        .ok();
    let ensure_final_newline_on_save = cfg
        .get::<FinalNewline>()
        .map(|v| match v {
            FinalNewline::Value(b) => b,
        })
        .ok();
    let remove_trailing_whitespace_on_save = cfg
        .get::<TrimTrailingWs>()
        .map(|v| match v {
            TrimTrailingWs::Value(b) => b,
        })
        .ok();
    fn merge<T>(target: &mut T, value: Option<T>) {
        if let Some(value) = value {
            *target = value;
        }
    }
    merge(&mut settings.tab_size, tab_size);
    merge(&mut settings.hard_tabs, hard_tabs);
    merge(
        &mut settings.remove_trailing_whitespace_on_save,
        remove_trailing_whitespace_on_save,
    );
    merge(
        &mut settings.ensure_final_newline_on_save,
        ensure_final_newline_on_save,
    );
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

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
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

        let mut edit_prediction_provider = default_value
            .features
            .as_ref()
            .and_then(|f| f.edit_prediction_provider);
        let mut edit_predictions_mode = default_value
            .edit_predictions
            .as_ref()
            .map(|edit_predictions| edit_predictions.mode)
            .ok_or_else(Self::missing_default)?;

        let mut completion_globs: HashSet<&String> = default_value
            .edit_predictions
            .as_ref()
            .and_then(|c| c.disabled_globs.as_ref())
            .map(|globs| globs.iter().collect())
            .ok_or_else(Self::missing_default)?;

        let mut copilot_settings = default_value
            .edit_predictions
            .as_ref()
            .map(|settings| settings.copilot.clone())
            .map(|copilot| CopilotSettings {
                proxy: copilot.proxy,
                proxy_no_verify: copilot.proxy_no_verify,
            })
            .unwrap_or_default();

        let mut edit_predictions_enabled_in_assistant = default_value
            .edit_predictions
            .as_ref()
            .map(|settings| settings.enabled_in_assistant)
            .unwrap_or(true);

        let mut file_types: FxHashMap<Arc<str>, GlobSet> = FxHashMap::default();

        for (language, patterns) in &default_value.file_types {
            let mut builder = GlobSetBuilder::new();

            for pattern in patterns {
                builder.add(Glob::new(pattern)?);
            }

            file_types.insert(language.clone(), builder.build()?);
        }

        for user_settings in sources.customizations() {
            if let Some(provider) = user_settings
                .features
                .as_ref()
                .and_then(|f| f.edit_prediction_provider)
            {
                edit_prediction_provider = Some(provider);
            }

            if let Some(edit_predictions) = user_settings.edit_predictions.as_ref() {
                edit_predictions_mode = edit_predictions.mode;
                edit_predictions_enabled_in_assistant = edit_predictions.enabled_in_assistant;

                if let Some(disabled_globs) = edit_predictions.disabled_globs.as_ref() {
                    completion_globs.extend(disabled_globs.iter());
                }
            }

            if let Some(proxy) = user_settings
                .edit_predictions
                .as_ref()
                .and_then(|settings| settings.copilot.proxy.clone())
            {
                copilot_settings.proxy = Some(proxy);
            }

            if let Some(proxy_no_verify) = user_settings
                .edit_predictions
                .as_ref()
                .and_then(|settings| settings.copilot.proxy_no_verify)
            {
                copilot_settings.proxy_no_verify = Some(proxy_no_verify);
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

            for (language, patterns) in &user_settings.file_types {
                let mut builder = GlobSetBuilder::new();

                let default_value = default_value.file_types.get(&language.clone());

                // Merge the default value with the user's value.
                if let Some(patterns) = default_value {
                    for pattern in patterns {
                        builder.add(Glob::new(pattern)?);
                    }
                }

                for pattern in patterns {
                    builder.add(Glob::new(pattern)?);
                }

                file_types.insert(language.clone(), builder.build()?);
            }
        }

        Ok(Self {
            edit_predictions: EditPredictionSettings {
                provider: if let Some(provider) = edit_prediction_provider {
                    provider
                } else {
                    EditPredictionProvider::None
                },
                disabled_globs: completion_globs
                    .iter()
                    .filter_map(|g| {
                        Some(DisabledGlob {
                            matcher: globset::Glob::new(g).ok()?.compile_matcher(),
                            is_absolute: Path::new(g).is_absolute(),
                        })
                    })
                    .collect(),
                mode: edit_predictions_mode,
                copilot: copilot_settings,
                enabled_in_assistant: edit_predictions_enabled_in_assistant,
            },
            defaults,
            languages,
            file_types,
        })
    }

    fn json_schema(
        generator: &mut schemars::r#gen::SchemaGenerator,
        params: &settings::SettingsJsonSchemaParams,
        _: &App,
    ) -> schemars::schema::RootSchema {
        let mut root_schema = generator.root_schema_for::<Self::FileContent>();

        // Create a schema for a 'languages overrides' object, associating editor
        // settings with specific languages.
        assert!(
            root_schema
                .definitions
                .contains_key("LanguageSettingsContent")
        );

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

        add_references_to_properties(
            &mut root_schema,
            &[("languages", "#/definitions/Languages")],
        );

        root_schema
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        let d = &mut current.defaults;
        if let Some(size) = vscode
            .read_value("editor.tabSize")
            .and_then(|v| v.as_u64())
            .and_then(|n| NonZeroU32::new(n as u32))
        {
            d.tab_size = Some(size);
        }
        if let Some(v) = vscode.read_bool("editor.insertSpaces") {
            d.hard_tabs = Some(!v);
        }

        vscode.enum_setting("editor.wordWrap", &mut d.soft_wrap, |s| match s {
            "on" => Some(SoftWrap::EditorWidth),
            "wordWrapColumn" => Some(SoftWrap::PreferLine),
            "bounded" => Some(SoftWrap::Bounded),
            "off" => Some(SoftWrap::None),
            _ => None,
        });
        vscode.u32_setting("editor.wordWrapColumn", &mut d.preferred_line_length);

        if let Some(arr) = vscode
            .read_value("editor.rulers")
            .and_then(|v| v.as_array())
            .map(|v| v.iter().map(|n| n.as_u64().map(|n| n as usize)).collect())
        {
            d.wrap_guides = arr;
        }
        if let Some(b) = vscode.read_bool("editor.guides.indentation") {
            if let Some(guide_settings) = d.indent_guides.as_mut() {
                guide_settings.enabled = b;
            } else {
                d.indent_guides = Some(IndentGuideSettings {
                    enabled: b,
                    ..Default::default()
                });
            }
        }

        if let Some(b) = vscode.read_bool("editor.guides.formatOnSave") {
            d.format_on_save = Some(if b {
                FormatOnSave::On
            } else {
                FormatOnSave::Off
            });
        }
        vscode.bool_setting(
            "editor.trimAutoWhitespace",
            &mut d.remove_trailing_whitespace_on_save,
        );
        vscode.bool_setting(
            "files.insertFinalNewline",
            &mut d.ensure_final_newline_on_save,
        );
        vscode.bool_setting("editor.inlineSuggest.enabled", &mut d.show_edit_predictions);
        vscode.enum_setting("editor.renderWhitespace", &mut d.show_whitespaces, |s| {
            Some(match s {
                "boundary" | "trailing" => ShowWhitespaceSetting::Boundary,
                "selection" => ShowWhitespaceSetting::Selection,
                "all" => ShowWhitespaceSetting::All,
                _ => ShowWhitespaceSetting::None,
            })
        });
        vscode.enum_setting(
            "editor.autoSurround",
            &mut d.use_auto_surround,
            |s| match s {
                "languageDefined" | "quotes" | "brackets" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );
        vscode.bool_setting("editor.formatOnType", &mut d.use_on_type_format);
        vscode.bool_setting("editor.linkedEditing", &mut d.linked_edits);
        vscode.bool_setting("editor.formatOnPaste", &mut d.auto_indent_on_paste);
        vscode.bool_setting(
            "editor.suggestOnTriggerCharacters",
            &mut d.show_completions_on_input,
        );
        if let Some(b) = vscode.read_bool("editor.suggest.showWords") {
            let mode = if b {
                WordsCompletionMode::Enabled
            } else {
                WordsCompletionMode::Disabled
            };
            if let Some(completion_settings) = d.completions.as_mut() {
                completion_settings.words = mode;
            } else {
                d.completions = Some(CompletionSettings {
                    words: mode,
                    lsp: true,
                    lsp_fetch_timeout_ms: 0,
                    lsp_insert_mode: LspInsertMode::ReplaceSuffix,
                });
            }
        }
        // TODO: pull ^ out into helper and reuse for per-language settings

        // vscodes file association map is inverted from ours, so we flip the mapping before merging
        let mut associations: HashMap<Arc<str>, Vec<String>> = HashMap::default();
        if let Some(map) = vscode
            .read_value("files.associations")
            .and_then(|v| v.as_object())
        {
            for (k, v) in map {
                let Some(v) = v.as_str() else { continue };
                associations.entry(v.into()).or_default().push(k.clone());
            }
        }
        // TODO: do we want to merge imported globs per filetype? for now we'll just replace
        current.file_types.extend(associations);
    }
}

fn merge_settings(settings: &mut LanguageSettings, src: &LanguageSettingsContent) {
    fn merge<T>(target: &mut T, value: Option<T>) {
        if let Some(value) = value {
            *target = value;
        }
    }

    merge(&mut settings.tab_size, src.tab_size);
    settings.tab_size = settings
        .tab_size
        .clamp(NonZeroU32::new(1).unwrap(), NonZeroU32::new(16).unwrap());

    merge(&mut settings.hard_tabs, src.hard_tabs);
    merge(&mut settings.soft_wrap, src.soft_wrap);
    merge(&mut settings.use_autoclose, src.use_autoclose);
    merge(&mut settings.use_auto_surround, src.use_auto_surround);
    merge(&mut settings.use_on_type_format, src.use_on_type_format);
    merge(&mut settings.auto_indent_on_paste, src.auto_indent_on_paste);
    merge(
        &mut settings.always_treat_brackets_as_autoclosed,
        src.always_treat_brackets_as_autoclosed,
    );
    merge(&mut settings.show_wrap_guides, src.show_wrap_guides);
    merge(&mut settings.wrap_guides, src.wrap_guides.clone());
    merge(&mut settings.indent_guides, src.indent_guides);
    merge(
        &mut settings.code_actions_on_format,
        src.code_actions_on_format.clone(),
    );
    merge(&mut settings.linked_edits, src.linked_edits);
    merge(&mut settings.tasks, src.tasks.clone());

    merge(
        &mut settings.preferred_line_length,
        src.preferred_line_length,
    );
    merge(&mut settings.formatter, src.formatter.clone());
    merge(&mut settings.prettier, src.prettier.clone());
    merge(
        &mut settings.jsx_tag_auto_close,
        src.jsx_tag_auto_close.clone(),
    );
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
    merge(&mut settings.language_servers, src.language_servers.clone());
    merge(&mut settings.allow_rewrap, src.allow_rewrap);
    merge(
        &mut settings.show_edit_predictions,
        src.show_edit_predictions,
    );
    merge(
        &mut settings.edit_predictions_disabled_in,
        src.edit_predictions_disabled_in.clone(),
    );
    merge(&mut settings.show_whitespaces, src.show_whitespaces);
    merge(
        &mut settings.extend_comment_on_newline,
        src.extend_comment_on_newline,
    );
    merge(&mut settings.inlay_hints, src.inlay_hints);
    merge(
        &mut settings.show_completions_on_input,
        src.show_completions_on_input,
    );
    merge(
        &mut settings.show_completion_documentation,
        src.show_completion_documentation,
    );
    merge(&mut settings.completions, src.completions);
}

/// Allows to enable/disable formatting with Prettier
/// and configure default Prettier, used when no project-level Prettier installation is found.
/// Prettier formatting is disabled by default.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrettierSettings {
    /// Enables or disables formatting with Prettier for a given language.
    #[serde(default)]
    pub allowed: bool,

    /// Forces Prettier integration to use a specific parser name when formatting files with the language.
    #[serde(default)]
    pub parser: Option<String>,

    /// Forces Prettier integration to use specific plugins when formatting files with the language.
    /// The default Prettier will be installed with these plugins.
    #[serde(default)]
    pub plugins: HashSet<String>,

    /// Default Prettier options, in the format as in package.json section for Prettier.
    /// If project installs Prettier via its package.json, these options will be ignored.
    #[serde(flatten)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct JsxTagAutoCloseSettings {
    /// Enables or disables auto-closing of JSX tags.
    #[serde(default)]
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;

    use super::*;

    #[test]
    fn test_formatter_deserialization() {
        let raw_auto = "{\"formatter\": \"auto\"}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw_auto).unwrap();
        assert_eq!(settings.formatter, Some(SelectedFormatter::Auto));
        let raw = "{\"formatter\": \"language_server\"}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            settings.formatter,
            Some(SelectedFormatter::List(FormatterList(
                Formatter::LanguageServer { name: None }.into()
            )))
        );
        let raw = "{\"formatter\": [{\"language_server\": {\"name\": null}}]}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            settings.formatter,
            Some(SelectedFormatter::List(FormatterList(
                vec![Formatter::LanguageServer { name: None }].into()
            )))
        );
        let raw = "{\"formatter\": [{\"language_server\": {\"name\": null}}, \"prettier\"]}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            settings.formatter,
            Some(SelectedFormatter::List(FormatterList(
                vec![
                    Formatter::LanguageServer { name: None },
                    Formatter::Prettier
                ]
                .into()
            )))
        );
    }

    #[test]
    fn test_formatter_deserialization_invalid() {
        let raw_auto = "{\"formatter\": {}}";
        let result: Result<LanguageSettingsContent, _> = serde_json::from_str(raw_auto);
        assert!(result.is_err());
    }

    #[gpui::test]
    fn test_edit_predictions_enabled_for_file(cx: &mut TestAppContext) {
        use crate::TestFile;
        use std::path::PathBuf;

        let cx = cx.app.borrow_mut();

        let build_settings = |globs: &[&str]| -> EditPredictionSettings {
            EditPredictionSettings {
                disabled_globs: globs
                    .iter()
                    .map(|glob_str| {
                        #[cfg(windows)]
                        let glob_str = {
                            let mut g = String::new();

                            if glob_str.starts_with('/') {
                                g.push_str("C:");
                            }

                            g.push_str(&glob_str.replace('/', "\\"));
                            g
                        };
                        #[cfg(windows)]
                        let glob_str = glob_str.as_str();

                        DisabledGlob {
                            matcher: globset::Glob::new(glob_str).unwrap().compile_matcher(),
                            is_absolute: Path::new(glob_str).is_absolute(),
                        }
                    })
                    .collect(),
                ..Default::default()
            }
        };

        const WORKTREE_NAME: &str = "project";
        let make_test_file = |segments: &[&str]| -> Arc<dyn File> {
            let mut path_buf = PathBuf::new();
            path_buf.extend(segments);

            Arc::new(TestFile {
                path: path_buf.as_path().into(),
                root_name: WORKTREE_NAME.to_string(),
                local_root: Some(PathBuf::from(if cfg!(windows) {
                    "C:\\absolute\\"
                } else {
                    "/absolute/"
                })),
            })
        };

        let test_file = make_test_file(&["src", "test", "file.rs"]);

        // Test relative globs
        let settings = build_settings(&["*.rs"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["*.txt"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test absolute globs
        let settings = build_settings(&["/absolute/**/*.rs"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["/other/**/*.rs"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test exact path match relative
        let settings = build_settings(&["src/test/file.rs"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["src/test/otherfile.rs"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test exact path match absolute
        let settings = build_settings(&[&format!("/absolute/{}/src/test/file.rs", WORKTREE_NAME)]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["/other/test/otherfile.rs"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test * glob
        let settings = build_settings(&["*"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["*.txt"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test **/* glob
        let settings = build_settings(&["**/*"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["other/**/*"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test directory/** glob
        let settings = build_settings(&["src/**"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));

        let test_file_root: Arc<dyn File> = Arc::new(TestFile {
            path: PathBuf::from("file.rs").as_path().into(),
            root_name: WORKTREE_NAME.to_string(),
            local_root: Some(PathBuf::from("/absolute/")),
        });
        assert!(settings.enabled_for_file(&test_file_root, &cx));

        let settings = build_settings(&["other/**"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test **/directory/* glob
        let settings = build_settings(&["**/test/*"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["**/other/*"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test multiple globs
        let settings = build_settings(&["*.rs", "*.txt", "src/**"]);
        assert!(!settings.enabled_for_file(&test_file, &cx));
        let settings = build_settings(&["*.txt", "*.md", "other/**"]);
        assert!(settings.enabled_for_file(&test_file, &cx));

        // Test dot files
        let dot_file = make_test_file(&[".config", "settings.json"]);
        let settings = build_settings(&[".*/**"]);
        assert!(!settings.enabled_for_file(&dot_file, &cx));

        let dot_env_file = make_test_file(&[".env"]);
        let settings = build_settings(&[".env"]);
        assert!(!settings.enabled_for_file(&dot_env_file, &cx));
    }

    #[test]
    pub fn test_resolve_language_servers() {
        fn language_server_names(names: &[&str]) -> Vec<LanguageServerName> {
            names
                .iter()
                .copied()
                .map(|name| LanguageServerName(name.to_string().into()))
                .collect::<Vec<_>>()
        }

        let available_language_servers = language_server_names(&[
            "typescript-language-server",
            "biome",
            "deno",
            "eslint",
            "tailwind",
        ]);

        // A value of just `["..."]` is the same as taking all of the available language servers.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[LanguageSettings::REST_OF_LANGUAGE_SERVERS.into()],
                &available_language_servers,
            ),
            available_language_servers
        );

        // Referencing one of the available language servers will change its order.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[
                    "biome".into(),
                    LanguageSettings::REST_OF_LANGUAGE_SERVERS.into(),
                    "deno".into()
                ],
                &available_language_servers
            ),
            language_server_names(&[
                "biome",
                "typescript-language-server",
                "eslint",
                "tailwind",
                "deno",
            ])
        );

        // Negating an available language server removes it from the list.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[
                    "deno".into(),
                    "!typescript-language-server".into(),
                    "!biome".into(),
                    LanguageSettings::REST_OF_LANGUAGE_SERVERS.into()
                ],
                &available_language_servers
            ),
            language_server_names(&["deno", "eslint", "tailwind"])
        );

        // Adding a language server not in the list of available language servers adds it to the list.
        assert_eq!(
            LanguageSettings::resolve_language_servers(
                &[
                    "my-cool-language-server".into(),
                    LanguageSettings::REST_OF_LANGUAGE_SERVERS.into()
                ],
                &available_language_servers
            ),
            language_server_names(&[
                "my-cool-language-server",
                "typescript-language-server",
                "biome",
                "deno",
                "eslint",
                "tailwind",
            ])
        );
    }
}
