use std::{num::NonZeroU32, path::Path};

use collections::{HashMap, HashSet};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::Error as _};
use settings_macros::{MergeFrom, with_fallible_options};
use std::sync::Arc;

use crate::{DocumentFoldingRanges, DocumentSymbols, ExtendingVec, SemanticTokens, merge_from};

/// The state of the modifier keys at some point in time
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ModifiersContent {
    /// The control key
    #[serde(default)]
    pub control: bool,
    /// The alt key
    /// Sometimes also known as the 'meta' key
    #[serde(default)]
    pub alt: bool,
    /// The shift key
    #[serde(default)]
    pub shift: bool,
    /// The command key, on macos
    /// the windows key, on windows
    /// the super key, on linux
    #[serde(default)]
    pub platform: bool,
    /// The function key
    #[serde(default)]
    pub function: bool,
}

#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AllLanguageSettingsContent {
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

/// The provider that supplies edit predictions.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum EditPredictionProvider {
    None,
    #[default]
    Copilot,
    Zed,
    Codestral,
    Ollama,
    OpenAiCompatibleApi,
    Mercury,
    Experimental(&'static str),
}

const EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME: &str = "zeta2";

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
            Zed,
            Codestral,
            Ollama,
            OpenAiCompatibleApi,
            Mercury,
            Experimental(String),
        }

        Ok(match Content::deserialize(deserializer)? {
            Content::None => EditPredictionProvider::None,
            Content::Copilot => EditPredictionProvider::Copilot,
            Content::Zed => EditPredictionProvider::Zed,
            Content::Codestral => EditPredictionProvider::Codestral,
            Content::Ollama => EditPredictionProvider::Ollama,
            Content::OpenAiCompatibleApi => EditPredictionProvider::OpenAiCompatibleApi,
            Content::Mercury => EditPredictionProvider::Mercury,
            Content::Experimental(name)
                if name == EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME =>
            {
                EditPredictionProvider::Zed
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
            | EditPredictionProvider::Codestral
            | EditPredictionProvider::Ollama
            | EditPredictionProvider::OpenAiCompatibleApi
            | EditPredictionProvider::Mercury
            | EditPredictionProvider::Experimental(_) => false,
        }
    }

    pub fn display_name(&self) -> Option<&'static str> {
        match self {
            EditPredictionProvider::Zed => Some("Zed AI"),
            EditPredictionProvider::Copilot => Some("GitHub Copilot"),
            EditPredictionProvider::Codestral => Some("Codestral"),
            EditPredictionProvider::Mercury => Some("Mercury"),
            EditPredictionProvider::Experimental(_) | EditPredictionProvider::None => None,
            EditPredictionProvider::Ollama => Some("Ollama"),
            EditPredictionProvider::OpenAiCompatibleApi => Some("OpenAI-Compatible API"),
        }
    }
}

/// The contents of the edit prediction settings.
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct EditPredictionSettingsContent {
    /// Determines which edit prediction provider to use.
    pub provider: Option<EditPredictionProvider>,
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
    /// Settings specific to Ollama.
    pub ollama: Option<OllamaEditPredictionSettingsContent>,
    /// Settings specific to using custom OpenAI-compatible servers for edit prediction.
    pub open_ai_compatible_api: Option<CustomEditPredictionProviderSettingsContent>,
    /// The directory where manually captured edit prediction examples are stored.
    pub examples_dir: Option<Arc<Path>>,
    /// Controls whether Zed may collect training data when using Zed's Edit Predictions.
    /// Data is only ever captured for files in projects that are detected as open source.
    ///
    /// - `"default"`: use the preference previously set via the status-bar toggle,
    ///   or false if no preference has been stored.
    /// - `"yes"`: allow data collection for files in open-source projects.
    /// - `"no"`: never allow data collection.
    pub allow_data_collection: Option<EditPredictionDataCollectionChoice>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct CustomEditPredictionProviderSettingsContent {
    /// Api URL to use for completions.
    ///
    /// Default: ""
    pub api_url: Option<String>,
    /// The prompt format to use for completions. Set to `""` to have the format be derived from the model name.
    ///
    /// Default: ""
    pub prompt_format: Option<EditPredictionPromptFormat>,
    /// The name of the model.
    ///
    /// Default: ""
    pub model: Option<String>,
    /// Maximum tokens to generate.
    ///
    /// Default: 256
    pub max_output_tokens: Option<u32>,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
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
pub enum EditPredictionPromptFormat {
    #[default]
    Infer,
    Zeta,
    Zeta2,
    CodeLlama,
    StarCoder,
    DeepseekCoder,
    Qwen,
    CodeGemma,
    Codestral,
    Glm,
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
    /// Whether the Copilot Next Edit Suggestions feature is enabled.
    ///
    /// Default: true
    pub enable_next_edit_suggestions: Option<bool>,
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

/// Ollama model name for edit predictions.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(transparent)]
pub struct OllamaModelName(pub String);

impl AsRef<str> for OllamaModelName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for OllamaModelName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<OllamaModelName> for String {
    fn from(value: OllamaModelName) -> Self {
        value.0
    }
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct OllamaEditPredictionSettingsContent {
    /// Model to use for completions.
    ///
    /// Default: none
    pub model: Option<OllamaModelName>,
    /// Maximum tokens to generate for FIM models.
    ///
    /// Default: 256
    pub max_output_tokens: Option<u32>,
    /// Api URL to use for completions.
    ///
    /// Default: "http://localhost:11434"
    pub api_url: Option<String>,

    /// The prompt format to use for completions. Set to `""` to have the format be derived from the model name.
    ///
    /// Default: ""
    pub prompt_format: Option<EditPredictionPromptFormat>,
}

/// Controls whether Zed collects training data when using Zed's Edit Predictions.
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
pub enum EditPredictionDataCollectionChoice {
    /// Use the preference previously set via the status-bar toggle, or false
    /// if no preference has been stored.
    #[default]
    Default,
    /// Allow Zed to collect training data from open-source projects.
    Yes,
    /// Never allow training data collection.
    No,
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
pub enum AutoIndentMode {
    /// Adjusts indentation based on syntax context when typing.
    /// Uses tree-sitter to analyze code structure and indent accordingly.
    SyntaxAware,
    /// Preserve the indentation of the current line when creating new lines,
    /// but don't adjust based on syntax context.
    PreserveIndent,
    /// No automatic indentation. New lines start at column 0.
    None,
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
    /// Soft wrap line at the preferred line length or the editor width (whichever is smaller).
    #[serde(alias = "preferred_line_length")]
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
    /// How line endings should be handled for new files and during format and
    /// save operations.
    ///
    /// - `detect`: Detect existing line endings and otherwise use the platform
    ///   default (`lf` on Unix, `crlf` on Windows).
    /// - `prefer_lf`: Prefer LF for new files and files with no existing line
    ///   ending.
    /// - `prefer_crlf`: Prefer CRLF for new files and files with no existing
    ///   line ending.
    /// - `enforce_lf`: Enforce LF during format and save.
    /// - `enforce_crlf`: Enforce CRLF during format and save.
    ///
    /// The EditorConfig `end_of_line` property overrides this setting and
    /// behaves like `enforce_lf` or `enforce_crlf`.
    ///
    /// Default: detect
    pub line_ending: Option<LineEndingSetting>,
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
    /// Controls how semantic tokens from language servers are used for syntax highlighting.
    ///
    /// Options:
    /// - "off": Do not request semantic tokens from language servers.
    /// - "combined": Use LSP semantic tokens together with tree-sitter highlighting.
    /// - "full": Use LSP semantic tokens exclusively, replacing tree-sitter highlighting.
    ///
    /// Default: "off"
    pub semantic_tokens: Option<SemanticTokens>,
    /// Controls whether folding ranges from language servers are used instead of
    /// tree-sitter and indent-based folding.
    ///
    /// Options:
    /// - "off": Use tree-sitter and indent-based folding (default).
    /// - "on": Use LSP folding wherever possible, falling back to tree-sitter and indent-based folding when no results were returned by the server.
    ///
    /// Default: "off"
    pub document_folding_ranges: Option<DocumentFoldingRanges>,
    /// Controls the source of document symbols used for outlines and breadcrumbs.
    ///
    /// Options:
    /// - "off": Use tree-sitter queries to compute document symbols (default).
    /// - "on": Use the language server's `textDocument/documentSymbol` LSP response. When enabled, tree-sitter is not used for document symbols.
    ///
    /// Default: "off"
    pub document_symbols: Option<DocumentSymbols>,
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
    /// Controls automatic indentation behavior when typing.
    ///
    /// - "syntax_aware": Adjusts indentation based on syntax context (default)
    /// - "preserve_indent": Preserves current line's indentation on new lines
    /// - "none": No automatic indentation
    ///
    /// Default: syntax_aware
    pub auto_indent: Option<AutoIndentMode>,
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
    pub toggle_on_modifiers_press: Option<ModifiersContent>,
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

/// Controls how line endings are normalized when a buffer is saved.
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
#[serde(rename_all = "snake_case")]
pub enum LineEndingSetting {
    /// Preserve the existing line endings of the file. New files use the
    /// platform default line ending.
    #[strum(serialize = "Detect")]
    Detect,
    /// Use LF for new files and files with no existing line-ending
    /// convention, while preserving existing LF or CRLF files.
    #[strum(serialize = "Prefer LF")]
    PreferLf,
    /// Use CRLF for new files and files with no existing line-ending
    /// convention, while preserving existing LF or CRLF files.
    #[strum(serialize = "Prefer CRLF")]
    PreferCrlf,
    /// Normalize line endings to LF (`\n`) during format and save.
    #[strum(serialize = "Enforce LF")]
    EnforceLf,
    /// Normalize line endings to CRLF (`\r\n`) during format and save.
    #[strum(serialize = "Enforce CRLF")]
    EnforceCrlf,
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
    /// Do not format code.
    None,
    /// Format code using Zed's Prettier integration.
    Prettier,
    /// Format code using an external command.
    External {
        /// The external program to run.
        command: String,
        /// The arguments to pass to the program.
        arguments: Option<Vec<String>>,
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
pub struct LanguageToSettingsMap(pub HashMap<String, LanguageSettingsContent>);

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
        let raw_none = "{\"formatter\": \"none\"}";
        let settings: LanguageSettingsContent = serde_json::from_str(raw_none).unwrap();
        assert_eq!(
            settings.formatter,
            Some(FormatterList::Single(Formatter::None))
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
