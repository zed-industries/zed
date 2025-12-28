use std::{num::NonZeroU32, path::Path};

use collections::{HashMap, HashSet};
use gpui::{Modifiers, SharedString};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::Error as _};
use settings_macros::{MergeFrom, with_fallible_options};
use std::sync::Arc;

use crate::{ExtendingVec, merge_from};

#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AllLanguageSettingsContent {
    /// The settings for enabling/disabling features.
    pub features: Option<FeaturesContent>,
    /// The edit prediction settings.
    pub edit_predictions: Option<EditPredictionSettingsContent>,
    /// The default language settings.
    #[serde(flatten)]
    pub defaults: LanguageSettingsContent,
    /// The settings for individual languages.
    #[serde(default)]
    pub languages: LanguageToSettingsMap,
    /// Settings for associating file extensions and filenames
    /// with languages.
    pub file_types: Option<HashMap<Arc<str>, ExtendingVec<String>>>,
}

impl merge_from::MergeFrom for AllLanguageSettingsContent {
    fn merge_from(&mut self, other: &Self) {
        self.file_types.merge_from(&other.file_types);
        self.features.merge_from(&other.features);
        self.edit_predictions.merge_from(&other.edit_predictions);

        // A user's global settings override the default global settings and
        // all default language-specific settings.
        //
        self.defaults.merge_from(&other.defaults);
        for language_settings in self.languages.0.values_mut() {
            language_settings.merge_from(&other.defaults);
        }

        // A user's language-specific settings override default language-specific settings.
        for (language_name, user_language_settings) in &other.languages.0 {
            if let Some(existing) = self.languages.0.get_mut(language_name) {
                existing.merge_from(&user_language_settings);
            } else {
                let mut new_settings = self.defaults.clone();
                new_settings.merge_from(&user_language_settings);

                self.languages.0.insert(language_name.clone(), new_settings);
            }
        }
    }
}

/// The settings for enabling/disabling features.
#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct FeaturesContent {
    /// Determines which edit prediction provider to use.
    pub edit_prediction_provider: Option<EditPredictionProvider>,
    /// Enables the experimental edit prediction context retrieval system.
    pub experimental_edit_prediction_context_retrieval: Option<bool>,
}

/// The provider that supplies edit predictions.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum EditPredictionProvider {
    None,
    #[default]
    Copilot,
    Supermaven,
    Zed,
    Codestral,
    Experimental(&'static str),
}

pub const EXPERIMENTAL_SWEEP_EDIT_PREDICTION_PROVIDER_NAME: &str = "sweep";
pub const EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME: &str = "zeta2";
pub const EXPERIMENTAL_MERCURY_EDIT_PREDICTION_PROVIDER_NAME: &str = "mercury";

impl<'de> Deserialize<'de> for EditPredictionProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum Content {
            None,
            Copilot,
            Supermaven,
            Zed,
            Codestral,
            Experimental(String),
        }

        Ok(match Content::deserialize(deserializer)? {
            Content::None => EditPredictionProvider::None,
            Content::Copilot => EditPredictionProvider::Copilot,
            Content::Supermaven => EditPredictionProvider::Supermaven,
            Content::Zed => EditPredictionProvider::Zed,
            Content::Codestral => EditPredictionProvider::Codestral,
            Content::Experimental(name)
                if name == EXPERIMENTAL_SWEEP_EDIT_PREDICTION_PROVIDER_NAME =>
            {
                EditPredictionProvider::Experimental(
                    EXPERIMENTAL_SWEEP_EDIT_PREDICTION_PROVIDER_NAME,
                )
            }
            Content::Experimental(name)
                if name == EXPERIMENTAL_MERCURY_EDIT_PREDICTION_PROVIDER_NAME =>
            {
                EditPredictionProvider::Experimental(
                    EXPERIMENTAL_MERCURY_EDIT_PREDICTION_PROVIDER_NAME,
                )
            }
            Content::Experimental(name)
                if name == EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME =>
            {
                EditPredictionProvider::Experimental(
                    EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME,
                )
            }
            Content::Experimental(name) => {
                return Err(D::Error::custom(format!(
                    "Unknown experimental edit prediction provider: {}",
                    name
                )));
            }
        })
    }
}

impl EditPredictionProvider {
    pub fn is_zed(&self) -> bool {
        match self {
            EditPredictionProvider::Zed => true,
            EditPredictionProvider::None
            | EditPredictionProvider::Copilot
            | EditPredictionProvider::Supermaven
            | EditPredictionProvider::Codestral
            | EditPredictionProvider::Experimental(_) => false,
        }
    }
}

/// The contents of the edit prediction settings.
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct EditPredictionSettingsContent {
    /// A list of globs representing files that edit predictions should be disabled for.
    /// This list adds to a pre-existing, sensible default set of globs.
    /// Any additional ones you add are combined with them.
    pub disabled_globs: Option<Vec<String>>,
    /// The mode used to display edit predictions in the buffer.
    /// Provider support required.
    pub mode: Option<EditPredictionsMode>,
    /// Settings specific to GitHub Copilot.
    pub copilot: Option<CopilotSettingsContent>,
    /// Settings specific to Codestral.
    pub codestral: Option<CodestralSettingsContent>,
    /// Whether edit predictions are enabled in the assistant prompt editor.
    /// This has no effect if globally disabled.
    pub enabled_in_text_threads: Option<bool>,
    /// The directory where manually captured edit prediction examples are stored.
    pub examples_dir: Option<Arc<Path>>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct CopilotSettingsContent {
    /// HTTP/HTTPS proxy to use for Copilot.
    ///
    /// Default: none
    pub proxy: Option<String>,
    /// Disable certificate verification for the proxy (not recommended).
    ///
    /// Default: false
    pub proxy_no_verify: Option<bool>,
    /// Enterprise URI for Copilot.
    ///
    /// Default: none
    pub enterprise_uri: Option<String>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct CodestralSettingsContent {
    /// Model to use for completions.
    ///
    /// Default: "codestral-latest"
    pub model: Option<String>,
    /// Maximum tokens to generate.
    ///
    /// Default: 150
    pub max_tokens: Option<u32>,
    /// Api URL to use for completions.
    ///
    /// Default: "https://codestral.mistral.ai"
    pub api_url: Option<String>,
}

/// The mode in which edit predictions should be displayed.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
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

/// Controls the soft-wrapping behavior in the editor.
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
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

/// The settings for a particular language.
#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LanguageSettingsContent {
    /// How many columns a tab should occupy.
    ///
    /// Default: 4
    #[schemars(range(min = 1, max = 128))]
    pub tab_size: Option<NonZeroU32>,
    /// Whether to indent lines using tab characters, as opposed to multiple
    /// spaces.
    ///
    /// Default: false
    pub hard_tabs: Option<bool>,
    /// How to soft-wrap long lines of text.
    ///
    /// Default: none
    pub soft_wrap: Option<SoftWrap>,
    /// The column at which to soft-wrap lines, for buffers where soft-wrap
    /// is enabled.
    ///
    /// Default: 80
    pub preferred_line_length: Option<u32>,
    /// Whether to show wrap guides in the editor. Setting this to true will
    /// show a guide at the 'preferred_line_length' value if softwrap is set to
    /// 'preferred_line_length', and will show any additional guides as specified
    /// by the 'wrap_guides' setting.
    ///
    /// Default: true
    pub show_wrap_guides: Option<bool>,
    /// Character counts at which to show wrap guides in the editor.
    ///
    /// Default: []
    pub wrap_guides: Option<Vec<usize>>,
    /// Indent guide related settings.
    pub indent_guides: Option<IndentGuideSettingsContent>,
    /// Whether or not to perform a buffer format before saving.
    ///
    /// Default: on
    pub format_on_save: Option<FormatOnSave>,
    /// Whether or not to remove any trailing whitespace from lines of a buffer
    /// before saving it.
    ///
    /// Default: true
    pub remove_trailing_whitespace_on_save: Option<bool>,
    /// Whether or not to ensure there's a single newline at the end of a buffer
    /// when saving it.
    ///
    /// Default: true
    pub ensure_final_newline_on_save: Option<bool>,
    /// How to perform a buffer format.
    ///
    /// Default: auto
    pub formatter: Option<FormatterList>,
    /// Zed's Prettier integration settings.
    /// Allows to enable/disable formatting with Prettier
    /// and configure default Prettier, used when no project-level Prettier installation is found.
    ///
    /// Default: off
    pub prettier: Option<PrettierSettingsContent>,
    /// Whether to automatically close JSX tags.
    pub jsx_tag_auto_close: Option<JsxTagAutoCloseSettingsContent>,
    /// Whether to use language servers to provide code intelligence.
    ///
    /// Default: true
    pub enable_language_server: Option<bool>,
    /// The list of language servers to use (or disable) for this language.
    ///
    /// This array should consist of language server IDs, as well as the following
    /// special tokens:
    /// - `"!<language_server_id>"` - A language server ID prefixed with a `!` will be disabled.
    /// - `"..."` - A placeholder to refer to the **rest** of the registered language servers for this language.
    ///
    /// Default: ["..."]
    pub language_servers: Option<Vec<String>>,
    /// Controls where the `editor::Rewrap` action is allowed for this language.
    ///
    /// Note: This setting has no effect in Vim mode, as rewrap is already
    /// allowed everywhere.
    ///
    /// Default: "in_comments"
    pub allow_rewrap: Option<RewrapBehavior>,
    /// Controls whether edit predictions are shown immediately (true)
    /// or manually by triggering `editor::ShowEditPrediction` (false).
    ///
    /// Default: true
    pub show_edit_predictions: Option<bool>,
    /// Controls whether edit predictions are shown in the given language
    /// scopes.
    ///
    /// Example: ["string", "comment"]
    ///
    /// Default: []
    pub edit_predictions_disabled_in: Option<Vec<String>>,
    /// Whether to show tabs and spaces in the editor.
    pub show_whitespaces: Option<ShowWhitespaceSetting>,
    /// Visible characters used to render whitespace when show_whitespaces is enabled.
    ///
    /// Default: "•" for spaces, "→" for tabs.
    pub whitespace_map: Option<WhitespaceMapContent>,
    /// Whether to start a new line with a comment when a previous line is a comment as well.
    ///
    /// Default: true
    pub extend_comment_on_newline: Option<bool>,
    /// Whether to continue markdown lists when pressing enter.
    ///
    /// Default: true
    pub extend_list_on_newline: Option<bool>,
    /// Whether to indent list items when pressing tab after a list marker.
    ///
    /// Default: true
    pub indent_list_on_tab: Option<bool>,
    /// Inlay hint related settings.
    pub inlay_hints: Option<InlayHintSettingsContent>,
    /// Whether to automatically type closing characters for you. For example,
    /// when you type '(', Zed will automatically add a closing ')' at the correct position.
    ///
    /// Default: true
    pub use_autoclose: Option<bool>,
    /// Whether to automatically surround text with characters for you. For example,
    /// when you select text and type '(', Zed will automatically surround text with ().
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
    /// Which code actions to run on save before the formatter.
    /// These are not run if formatting is off.
    ///
    /// Default: {} (or {"source.organizeImports": true} for Go).
    pub code_actions_on_format: Option<HashMap<String, bool>>,
    /// Whether to perform linked edits of associated ranges, if the language server supports it.
    /// For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well.
    ///
    /// Default: true
    pub linked_edits: Option<bool>,
    /// Whether indentation should be adjusted based on the context whilst typing.
    ///
    /// Default: true
    pub auto_indent: Option<bool>,
    /// Whether indentation of pasted content should be adjusted based on the context.
    ///
    /// Default: true
    pub auto_indent_on_paste: Option<bool>,
    /// Task configuration for this language.
    ///
    /// Default: {}
    pub tasks: Option<LanguageTaskSettingsContent>,
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
    pub completions: Option<CompletionSettingsContent>,
    /// Preferred debuggers for this language.
    ///
    /// Default: []
    pub debuggers: Option<Vec<String>>,
    /// Whether to enable word diff highlighting in the editor.
    ///
    /// When enabled, changed words within modified lines are highlighted
    /// to show exactly what changed.
    ///
    /// Default: true
    pub word_diff_enabled: Option<bool>,
    /// Whether to use tree-sitter bracket queries to detect and colorize the brackets in the editor.
    ///
    /// Default: false
    pub colorize_brackets: Option<bool>,
}

/// Controls how whitespace should be displayedin the editor.
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
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
    /// Draw whitespaces only after non-whitespace characters.
    Trailing,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct WhitespaceMapContent {
    pub space: Option<char>,
    pub tab: Option<char>,
}

/// The behavior of `editor::Rewrap`.
#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
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

#[with_fallible_options]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct JsxTagAutoCloseSettingsContent {
    /// Enables or disables auto-closing of JSX tags.
    pub enabled: Option<bool>,
}

/// The settings for inlay hints.
#[with_fallible_options]
#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct InlayHintSettingsContent {
    /// Global switch to toggle hints on and off.
    ///
    /// Default: false
    pub enabled: Option<bool>,
    /// Global switch to toggle inline values on and off when debugging.
    ///
    /// Default: true
    pub show_value_hints: Option<bool>,
    /// Whether type hints should be shown.
    ///
    /// Default: true
    pub show_type_hints: Option<bool>,
    /// Whether parameter hints should be shown.
    ///
    /// Default: true
    pub show_parameter_hints: Option<bool>,
    /// Whether other hints should be shown.
    ///
    /// Default: true
    pub show_other_hints: Option<bool>,
    /// Whether to show a background for inlay hints.
    ///
    /// If set to `true`, the background will use the `hint.background` color
    /// from the current theme.
    ///
    /// Default: false
    pub show_background: Option<bool>,
    /// Whether or not to debounce inlay hints updates after buffer edits.
    ///
    /// Set to 0 to disable debouncing.
    ///
    /// Default: 700
    pub edit_debounce_ms: Option<u64>,
    /// Whether or not to debounce inlay hints updates after buffer scrolls.
    ///
    /// Set to 0 to disable debouncing.
    ///
    /// Default: 50
    pub scroll_debounce_ms: Option<u64>,
    /// Toggles inlay hints (hides or shows) when the user presses the modifiers specified.
    /// If only a subset of the modifiers specified is pressed, hints are not toggled.
    /// If no modifiers are specified, this is equivalent to `null`.
    ///
    /// Default: null
    pub toggle_on_modifiers_press: Option<Modifiers>,
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
    /// Returns the [`InlayHintKind`]fromthe given name.
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

/// Controls how completions are processed for this language.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom, Default)]
#[serde(rename_all = "snake_case")]
pub struct CompletionSettingsContent {
    /// Controls how words are completed.
    /// For large documents, not all words may be fetched for completion.
    ///
    /// Default: `fallback`
    pub words: Option<WordsCompletionMode>,
    /// How many characters has to be in the completions query to automatically show the words-based completions.
    /// Before that value, it's still possible to trigger the words-based completion manually with the corresponding editor command.
    ///
    /// Default: 3
    pub words_min_length: Option<u32>,
    /// Whether to fetch LSP completions or not.
    ///
    /// Default: true
    pub lsp: Option<bool>,
    /// When fetching LSP completions, determines how long to wait for a response of a particular server.
    /// When set to 0, waits indefinitely.
    ///
    /// Default: 0
    pub lsp_fetch_timeout_ms: Option<u64>,
    /// Controls how LSP completions are inserted.
    ///
    /// Default: "replace_suffix"
    pub lsp_insert_mode: Option<LspInsertMode>,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
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

/// Controls how document's words are completed.
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
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

/// Allows to enable/disable formatting with Prettier
/// and configure default Prettier, used when no project-level Prettier installation is found.
/// Prettier formatting is disabled by default.
#[with_fallible_options]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct PrettierSettingsContent {
    /// Enables or disables formatting with Prettier for a given language.
    pub allowed: Option<bool>,

    /// Forces Prettier integration to use a specific parser name when formatting files with the language.
    pub parser: Option<String>,

    /// Forces Prettier integration to use specific plugins when formatting files with the language.
    /// The default Prettier will be installed with these plugins.
    pub plugins: Option<HashSet<String>>,

    /// Default Prettier options, in the format as in package.json section for Prettier.
    /// If project installs Prettier via its package.json, these options will be ignored.
    #[serde(flatten)]
    pub options: Option<HashMap<String, serde_json::Value>>,
}

/// TODO: this should just be a bool
/// Controls the behavior of formatting files when they are saved.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum FormatOnSave {
    /// Files should be formatted on save.
    On,
    /// Files should not be formatted on save.
    Off,
}

/// Controls which formatters should be used when formatting code.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom)]
#[serde(untagged)]
pub enum FormatterList {
    Single(Formatter),
    Vec(Vec<Formatter>),
}

impl Default for FormatterList {
    fn default() -> Self {
        Self::Single(Formatter::default())
    }
}

impl AsRef<[Formatter]> for FormatterList {
    fn as_ref(&self) -> &[Formatter] {
        match &self {
            Self::Single(single) => std::slice::from_ref(single),
            Self::Vec(v) => v,
        }
    }
}

/// Controls which formatter should be used when formatting code. If there are multiple formatters, they are executed in the order of declaration.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum Formatter {
    /// Format files using Zed's Prettier integration (if applicable),
    /// or falling back to formatting via language server.
    #[default]
    Auto,
    /// Format code using Zed's Prettier integration.
    Prettier,
    /// Format code using an external command.
    External {
        /// The external program to run.
        command: Arc<str>,
        /// The arguments to pass to the program.
        arguments: Option<Arc<[String]>>,
    },
    /// Files should be formatted using a code action executed by language servers.
    CodeAction(String),
    /// Format code using a language server.
    #[serde(untagged)]
    LanguageServer(LanguageServerFormatterSpecifier),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom)]
#[serde(
    rename_all = "snake_case",
    // allow specifying language servers as "language_server" or {"language_server": {"name": ...}}
    from = "LanguageServerVariantContent",
    into = "LanguageServerVariantContent"
)]
pub enum LanguageServerFormatterSpecifier {
    Specific {
        name: String,
    },
    #[default]
    Current,
}

impl From<LanguageServerVariantContent> for LanguageServerFormatterSpecifier {
    fn from(value: LanguageServerVariantContent) -> Self {
        match value {
            LanguageServerVariantContent::Specific {
                language_server: LanguageServerSpecifierContent { name: Some(name) },
            } => Self::Specific { name },
            _ => Self::Current,
        }
    }
}

impl From<LanguageServerFormatterSpecifier> for LanguageServerVariantContent {
    fn from(value: LanguageServerFormatterSpecifier) -> Self {
        match value {
            LanguageServerFormatterSpecifier::Specific { name } => Self::Specific {
                language_server: LanguageServerSpecifierContent { name: Some(name) },
            },
            LanguageServerFormatterSpecifier::Current => {
                Self::Current(CurrentLanguageServerContent::LanguageServer)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case", untagged)]
enum LanguageServerVariantContent {
    /// Format code using a specific language server.
    Specific {
        language_server: LanguageServerSpecifierContent,
    },
    /// Format code using the current language server.
    Current(CurrentLanguageServerContent),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
enum CurrentLanguageServerContent {
    #[default]
    LanguageServer,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
struct LanguageServerSpecifierContent {
    /// The name of the language server to format with
    name: Option<String>,
}

/// The settings for indent guides.
#[with_fallible_options]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct IndentGuideSettingsContent {
    /// Whether to display indent guides in the editor.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// The width of the indent guides in pixels, between 1 and 10.
    ///
    /// Default: 1
    pub line_width: Option<u32>,
    /// The width of the active indent guide in pixels, between 1 and 10.
    ///
    /// Default: 1
    pub active_line_width: Option<u32>,
    /// Determines how indent guides are colored.
    ///
    /// Default: Fixed
    pub coloring: Option<IndentGuideColoring>,
    /// Determines how indent guide backgrounds are colored.
    ///
    /// Default: Disabled
    pub background_coloring: Option<IndentGuideBackgroundColoring>,
}

/// The task settings for a particular language.
#[with_fallible_options]
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Serialize, JsonSchema, MergeFrom)]
pub struct LanguageTaskSettingsContent {
    /// Extra task variables to set for a particular language.
    pub variables: Option<HashMap<String, String>>,
    pub enabled: Option<bool>,
    /// Use LSP tasks over Zed language extension ones.
    /// If no LSP tasks are returned due to error/timeout or regular execution,
    /// Zed language extension tasks will be used instead.
    ///
    /// Other Zed tasks will still be shown:
    /// * Zed task from either of the task config file
    /// * Zed task from history (e.g. one-off task was spawned before)
    pub prefer_lsp: Option<bool>,
}

/// Map from language name to settings.
#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LanguageToSettingsMap(pub HashMap<SharedString, LanguageSettingsContent>);

/// Determines how indent guides are colored.
#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
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
#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum IndentGuideBackgroundColoring {
    /// Do not render any background for indent guides.
    #[default]
    Disabled,
    /// Use a different color for each indentation level.
    IndentAware,
}

#[cfg(test)]
mod test {

    use crate::{ParseStatus, fallible_options};

    use super::*;

    #[test]
    fn test_formatter_deserialization() {
        let raw_auto = "{\"formatter\": \"auto\"}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw_auto).unwrap();
        assert_eq!(
            settings.formatter,
            Some(FormatterList::Single(Formatter::Auto))
        );
        let raw = "{\"formatter\": \"language_server\"}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            settings.formatter,
            Some(FormatterList::Single(Formatter::LanguageServer(
                LanguageServerFormatterSpecifier::Current
            )))
        );

        let raw = "{\"formatter\": [{\"language_server\": {\"name\": null}}]}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            settings.formatter,
            Some(FormatterList::Vec(vec![Formatter::LanguageServer(
                LanguageServerFormatterSpecifier::Current
            )]))
        );
        let raw = "{\"formatter\": [{\"language_server\": {\"name\": null}}, \"language_server\", \"prettier\"]}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            settings.formatter,
            Some(FormatterList::Vec(vec![
                Formatter::LanguageServer(LanguageServerFormatterSpecifier::Current),
                Formatter::LanguageServer(LanguageServerFormatterSpecifier::Current),
                Formatter::Prettier
            ]))
        );

        let raw = "{\"formatter\": [{\"language_server\": {\"name\": \"ruff\"}}, \"prettier\"]}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            settings.formatter,
            Some(FormatterList::Vec(vec![
                Formatter::LanguageServer(LanguageServerFormatterSpecifier::Specific {
                    name: "ruff".to_string()
                }),
                Formatter::Prettier
            ]))
        );

        assert_eq!(
            serde_json::to_string(&LanguageServerFormatterSpecifier::Current).unwrap(),
            "\"language_server\"",
        );
    }

    #[test]
    fn test_formatter_deserialization_invalid() {
        let raw_auto = "{\"formatter\": {}}";
        let (_, result) = fallible_options::parse_json::<LanguageSettingsContent>(raw_auto);
        assert!(matches!(result, ParseStatus::Failed { .. }));
    }

    #[test]
    fn test_prettier_options() {
        let raw_prettier = r#"{"allowed": false, "tabWidth": 4, "semi": false}"#;
        let result = serde_json::from_str::<PrettierSettingsContent>(raw_prettier)
            .expect("Failed to parse prettier options");
        assert!(
            result
                .options
                .as_ref()
                .expect("options were flattened")
                .contains_key("semi")
        );
        assert!(
            result
                .options
                .as_ref()
                .expect("options were flattened")
                .contains_key("tabWidth")
        );
    }
}
