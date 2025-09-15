use std::borrow::Cow;
use std::env;
use std::num::NonZeroU32;
use std::sync::Arc;

use anyhow::Result;
use collections::{HashMap, HashSet};
use gpui::{App, FontFallbacks, FontFeatures, HighlightStyle, Hsla, Modifiers, SharedString};
use release_channel::ReleaseChannel;
use schemars::{JsonSchema, json_schema};
use serde::de::{self, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use util::schemars::replace_subschema;
use util::serde::default_true;

use crate::{ActiveSettingsProfileName, ParameterizedJsonSchema, Settings};

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SettingsContent {
    #[serde(flatten)]
    pub project: ProjectSettingsContent,

    pub base_keymap: Option<BaseKeymapContent>,

    pub auto_update: Option<bool>,

    pub title_bar: Option<TitleBarSettingsContent>,
}

impl SettingsContent {
    pub fn languages_mut(&mut self) -> &mut HashMap<SharedString, LanguageSettingsContent> {
        &mut self.project.all_languages.languages.0
    }
}

// todo!() what should this be?
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ServerSettingsContent {
    #[serde(flatten)]
    pub project: ProjectSettingsContent,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct UserSettingsContent {
    #[serde(flatten)]
    pub(crate) content: SettingsContent,

    pub(crate) dev: Option<SettingsContent>,
    pub(crate) nightly: Option<SettingsContent>,
    pub(crate) preview: Option<SettingsContent>,
    pub(crate) stable: Option<SettingsContent>,

    pub(crate) macos: Option<SettingsContent>,
    pub(crate) windows: Option<SettingsContent>,
    pub(crate) linux: Option<SettingsContent>,

    #[serde(default)]
    pub(crate) profiles: HashMap<String, SettingsContent>,
}

pub struct ExtensionsSettingsContent {
    pub(crate) all_languages: AllLanguageSettingsContent,
}

impl UserSettingsContent {
    pub(crate) fn for_release_channel(&self) -> Option<&SettingsContent> {
        match *release_channel::RELEASE_CHANNEL {
            ReleaseChannel::Dev => self.dev.as_ref(),
            ReleaseChannel::Nightly => self.nightly.as_ref(),
            ReleaseChannel::Preview => self.preview.as_ref(),
            ReleaseChannel::Stable => self.stable.as_ref(),
        }
    }

    pub(crate) fn for_os(&self) -> Option<&SettingsContent> {
        match env::consts::OS {
            "macos" => self.macos.as_ref(),
            "linux" => self.linux.as_ref(),
            "windows" => self.windows.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn for_profile(&self, cx: &App) -> Option<&SettingsContent> {
        let Some(active_profile) = cx.try_global::<ActiveSettingsProfileName>() else {
            return None;
        };
        self.profiles.get(&active_profile.0)
    }
}

/// Base key bindings scheme. Base keymaps can be overridden with user keymaps.
///
/// Default: VSCode
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub enum BaseKeymapContent {
    #[default]
    VSCode,
    JetBrains,
    SublimeText,
    Atom,
    TextMate,
    Emacs,
    Cursor,
    None,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProjectSettingsContent {
    #[serde(flatten)]
    pub(crate) all_languages: AllLanguageSettingsContent,
}

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
    pub languages: LanguageToSettingsMap,
    /// Settings for associating file extensions and filenames
    /// with languages.
    #[serde(default)]
    pub file_types: HashMap<SharedString, Vec<String>>,
}

/// The settings for enabling/disabling features.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FeaturesContent {
    /// Determines which edit prediction provider to use.
    pub edit_prediction_provider: Option<EditPredictionProviderContent>,
}

/// The provider that supplies edit predictions.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EditPredictionProviderContent {
    None,
    #[default]
    Copilot,
    Supermaven,
    Zed,
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
    pub mode: EditPredictionsModeContent,
    /// Settings specific to GitHub Copilot.
    #[serde(default)]
    pub copilot: CopilotSettingsContent,
    /// Whether edit predictions are enabled in the assistant prompt editor.
    /// This has no effect if globally disabled.
    #[serde(default = "default_true")]
    pub enabled_in_text_threads: bool,
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
    /// Enterprise URI for Copilot.
    ///
    /// Default: none
    #[serde(default)]
    pub enterprise_uri: Option<String>,
}

/// The mode in which edit predictions should be displayed.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EditPredictionsModeContent {
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
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrapContent {
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
    pub soft_wrap: Option<SoftWrapContent>,
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
    pub indent_guides: Option<IndentGuideSettingsContent>,
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
    /// Visible characters used to render whitespace when show_whitespaces is enabled.
    ///
    /// Default: "•" for spaces, "→" for tabs.
    #[serde(default)]
    pub whitespace_map: Option<WhitespaceMap>,
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
    /// Preferred debuggers for this language.
    ///
    /// Default: []
    pub debuggers: Option<Vec<String>>,
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
    /// Draw whitespaces only after non-whitespace characters.
    Trailing,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct WhitespaceMap {
    #[serde(default)]
    pub space: Option<String>,
    #[serde(default)]
    pub tab: Option<String>,
}

impl WhitespaceMap {
    pub fn space(&self) -> SharedString {
        self.space
            .as_ref()
            .map_or_else(|| SharedString::from("•"), |s| SharedString::from(s))
    }

    pub fn tab(&self) -> SharedString {
        self.tab
            .as_ref()
            .map_or_else(|| SharedString::from("→"), |s| SharedString::from(s))
    }
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct JsxTagAutoCloseSettings {
    /// Enables or disables auto-closing of JSX tags.
    #[serde(default)]
    pub enabled: bool,
}

/// The settings for inlay hints.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct InlayHintSettings {
    /// Global switch to toggle hints on and off.
    ///
    /// Default: false
    #[serde(default)]
    pub enabled: bool,
    /// Global switch to toggle inline values on and off when debugging.
    ///
    /// Default: true
    #[serde(default = "default_true")]
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
    /// How many characters has to be in the completions query to automatically show the words-based completions.
    /// Before that value, it's still possible to trigger the words-based completion manually with the corresponding editor command.
    ///
    /// Default: 3
    #[serde(default = "default_3")]
    pub words_min_length: usize,
    /// Whether to fetch LSP completions or not.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub lsp: bool,
    /// When fetching LSP completions, determines how long to wait for a response of a particular server.
    /// When set to 0, waits indefinitely.
    ///
    /// Default: 0
    #[serde(default)]
    pub lsp_fetch_timeout_ms: u64,
    /// Controls how LSP completions are inserted.
    ///
    /// Default: "replace_suffix"
    #[serde(default = "default_lsp_insert_mode")]
    pub lsp_insert_mode: LspInsertMode,
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

fn default_words_completion_mode() -> WordsCompletionMode {
    WordsCompletionMode::Fallback
}

fn default_lsp_insert_mode() -> LspInsertMode {
    LspInsertMode::ReplaceSuffix
}

fn default_3() -> usize {
    3
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
    fn schema_name() -> Cow<'static, str> {
        "OnSaveFormatter".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let formatter_schema = Formatter::json_schema(generator);

        json_schema!({
            "oneOf": [
                {
                    "type": "array",
                    "items": formatter_schema
                },
                {
                    "type": "string",
                    "enum": ["on", "off", "language_server"]
                },
                formatter_schema
            ]
        })
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
                    Ok(Self::Value::List(FormatterList::Single(
                        Formatter::LanguageServer { name: None },
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
    fn schema_name() -> Cow<'static, str> {
        "Formatter".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let formatter_schema = Formatter::json_schema(generator);

        json_schema!({
            "oneOf": [
                {
                    "type": "array",
                    "items": formatter_schema
                },
                {
                    "type": "string",
                    "enum": ["auto", "language_server"]
                },
                formatter_schema
            ]
        })
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
                    Ok(Self::Value::List(FormatterList::Single(
                        Formatter::LanguageServer { name: None },
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

/// Controls which formatters should be used when formatting code.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
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

/// Controls which formatter should be used when formatting code. If there are multiple formatters, they are executed in the order of declaration.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Formatter {
    /// Format code using the current language server.
    LanguageServer { name: Option<String> },
    /// Format code using Zed's Prettier integration.
    #[default]
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
pub struct IndentGuideSettingsContent {
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

/// The task settings for a particular language.
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct LanguageTaskConfig {
    /// Extra task variables to set for a particular language.
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Use LSP tasks over Zed language extension ones.
    /// If no LSP tasks are returned due to error/timeout or regular execution,
    /// Zed language extension tasks will be used instead.
    ///
    /// Other Zed tasks will still be shown:
    /// * Zed task from either of the task config file
    /// * Zed task from history (e.g. one-off task was spawned before)
    #[serde(default = "default_true")]
    pub prefer_lsp: bool,
}

/// Map from language name to settings. Its `ParameterizedJsonSchema` allows only known language
/// names in the keys.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LanguageToSettingsMap(pub HashMap<SharedString, LanguageSettingsContent>);

inventory::submit! {
    ParameterizedJsonSchema {
        add_and_get_ref: |generator, params, _cx| {
            let language_settings_content_ref = generator
                .subschema_for::<LanguageSettingsContent>()
                .to_value();
            replace_subschema::<LanguageToSettingsMap>(generator, || json_schema!({
                "type": "object",
                "properties": params
                    .language_names
                    .iter()
                    .map(|name| {
                        (
                            name.clone(),
                            language_settings_content_ref.clone(),
                        )
                    })
                    .collect::<serde_json::Map<_, _>>()
            }))
        }
    }
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

#[derive(Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct TitleBarSettingsContent {
    /// Controls when the title bar is visible: "always" | "never" | "hide_in_full_screen".
    ///
    /// Default: "always"
    pub show: Option<TitleBarVisibilityContent>,
    /// Whether to show the branch icon beside branch switcher in the title bar.
    ///
    /// Default: false
    pub show_branch_icon: Option<bool>,
    /// Whether to show onboarding banners in the title bar.
    ///
    /// Default: true
    pub show_onboarding_banner: Option<bool>,
    /// Whether to show user avatar in the title bar.
    ///
    /// Default: true
    pub show_user_picture: Option<bool>,
    /// Whether to show the branch name button in the titlebar.
    ///
    /// Default: true
    pub show_branch_name: Option<bool>,
    /// Whether to show the project host and name in the titlebar.
    ///
    /// Default: true
    pub show_project_items: Option<bool>,
    /// Whether to show the sign in button in the title bar.
    ///
    /// Default: true
    pub show_sign_in: Option<bool>,
    /// Whether to show the menus in the title bar.
    ///
    /// Default: false
    pub show_menus: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TitleBarVisibilityContent {
    Always,
    Never,
    HideInFullScreen,
}

/// Settings for rendering text in UI and text buffers.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ThemeSettingsContent {
    /// The default font size for text in the UI.
    #[serde(default)]
    pub ui_font_size: Option<f32>,
    /// The name of a font to use for rendering in the UI.
    #[serde(default)]
    pub ui_font_family: Option<FontFamilyName>,
    /// The font fallbacks to use for rendering in the UI.
    #[serde(default)]
    #[schemars(default = "default_font_fallbacks")]
    #[schemars(extend("uniqueItems" = true))]
    pub ui_font_fallbacks: Option<Vec<FontFamilyName>>,
    /// The OpenType features to enable for text in the UI.
    #[serde(default)]
    #[schemars(default = "default_font_features")]
    pub ui_font_features: Option<FontFeatures>,
    /// The weight of the UI font in CSS units from 100 to 900.
    #[serde(default)]
    pub ui_font_weight: Option<f32>,
    /// The name of a font to use for rendering in text buffers.
    #[serde(default)]
    pub buffer_font_family: Option<FontFamilyName>,
    /// The font fallbacks to use for rendering in text buffers.
    #[serde(default)]
    #[schemars(extend("uniqueItems" = true))]
    pub buffer_font_fallbacks: Option<Vec<FontFamilyName>>,
    /// The default font size for rendering in text buffers.
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    /// The weight of the editor font in CSS units from 100 to 900.
    #[serde(default)]
    pub buffer_font_weight: Option<f32>,
    /// The buffer's line height.
    #[serde(default)]
    pub buffer_line_height: Option<BufferLineHeight>,
    /// The OpenType features to enable for rendering in text buffers.
    #[serde(default)]
    #[schemars(default = "default_font_features")]
    pub buffer_font_features: Option<FontFeatures>,
    /// The font size for the agent panel. Falls back to the UI font size if unset.
    #[serde(default)]
    pub agent_font_size: Option<Option<f32>>,
    /// The name of the Zed theme to use.
    #[serde(default)]
    pub theme: Option<ThemeSelection>,
    /// The name of the icon theme to use.
    #[serde(default)]
    pub icon_theme: Option<IconThemeSelection>,

    /// UNSTABLE: Expect many elements to be broken.
    ///
    // Controls the density of the UI.
    #[serde(rename = "unstable.ui_density", default)]
    pub ui_density: Option<UiDensity>,

    /// How much to fade out unused code.
    #[serde(default)]
    pub unnecessary_code_fade: Option<f32>,

    /// EXPERIMENTAL: Overrides for the current theme.
    ///
    /// These values will override the ones on the current theme specified in `theme`.
    #[serde(rename = "experimental.theme_overrides", default)]
    pub experimental_theme_overrides: Option<ThemeStyleContent>,

    /// Overrides per theme
    ///
    /// These values will override the ones on the specified theme
    #[serde(default)]
    pub theme_overrides: HashMap<String, ThemeStyleContent>,
}

fn default_font_features() -> Option<FontFeatures> {
    Some(FontFeatures::default())
}

fn default_font_fallbacks() -> Option<FontFallbacks> {
    Some(FontFallbacks::default())
}

/// Represents the selection of a theme, which can be either static or dynamic.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum ThemeSelection {
    /// A static theme selection, represented by a single theme name.
    Static(ThemeName),
    /// A dynamic theme selection, which can change based the [ThemeMode].
    Dynamic {
        /// The mode used to determine which theme to use.
        #[serde(default)]
        mode: ThemeMode,
        /// The theme to use for light mode.
        light: ThemeName,
        /// The theme to use for dark mode.
        dark: ThemeName,
    },
}

/// Represents the selection of an icon theme, which can be either static or dynamic.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum IconThemeSelection {
    /// A static icon theme selection, represented by a single icon theme name.
    Static(IconThemeName),
    /// A dynamic icon theme selection, which can change based on the [`ThemeMode`].
    Dynamic {
        /// The mode used to determine which theme to use.
        #[serde(default)]
        mode: ThemeMode,
        /// The icon theme to use for light mode.
        light: IconThemeName,
        /// The icon theme to use for dark mode.
        dark: IconThemeName,
    },
}

// TODO: Rename ThemeMode -> ThemeAppearanceMode
/// The mode use to select a theme.
///
/// `Light` and `Dark` will select their respective themes.
///
/// `System` will select the theme based on the system's appearance.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
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

/// Specifies the density of the UI.
/// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum UiDensity {
    /// A denser UI with tighter spacing and smaller elements.
    #[serde(alias = "compact")]
    Compact,
    #[default]
    #[serde(alias = "default")]
    /// The default UI density.
    Default,
    #[serde(alias = "comfortable")]
    /// A looser UI with more spacing and larger elements.
    Comfortable,
}

impl UiDensity {
    /// The spacing ratio of a given density.
    /// TODO: Standardize usage throughout the app or remove
    pub fn spacing_ratio(self) -> f32 {
        match self {
            UiDensity::Compact => 0.75,
            UiDensity::Default => 1.0,
            UiDensity::Comfortable => 1.25,
        }
    }
}

/// Newtype for font family name. Its `ParameterizedJsonSchema` lists the font families known at
/// runtime.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(transparent)]
pub struct FontFamilyName(pub Arc<str>);

inventory::submit! {
    ParameterizedJsonSchema {
        add_and_get_ref: |generator, params, _cx| {
            replace_subschema::<FontFamilyName>(generator, || {
                json_schema!({
                    "type": "string",
                    "enum": params.font_names,
                })
            })
        }
    }
}

/// The buffer's line height.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum BufferLineHeight {
    /// A less dense line height.
    #[default]
    Comfortable,
    /// The default line height.
    Standard,
    /// A custom line height, where 1.0 is the font's height. Must be at least 1.0.
    Custom(#[serde(deserialize_with = "deserialize_line_height")] f32),
}

fn deserialize_line_height<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = f32::deserialize(deserializer)?;
    if value < 1.0 {
        return Err(serde::de::Error::custom(
            "buffer_line_height.custom must be at least 1.0",
        ));
    }

    Ok(value)
}

/// The content of a serialized theme.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(default)]
pub struct ThemeStyleContent {
    #[serde(default, rename = "background.appearance")]
    pub window_background_appearance: Option<WindowBackgroundContent>,

    #[serde(default)]
    pub accents: Vec<AccentContent>,

    #[serde(flatten, default)]
    pub colors: ThemeColorsContent,

    #[serde(flatten, default)]
    pub status: StatusColorsContent,

    #[serde(default)]
    pub players: Vec<PlayerColorContent>,

    /// The styles for syntax nodes.
    #[serde(default)]
    pub syntax: IndexMap<String, HighlightStyleContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AccentContent(pub Option<String>);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct PlayerColorContent {
    pub cursor: Option<String>,
    pub background: Option<String>,
    pub selection: Option<String>,
}

pub(crate) fn try_parse_color(color: &str) -> Result<Hsla> {
    let rgba = gpui::Rgba::try_from(color)?;
    let rgba = palette::rgb::Srgba::from_components((rgba.r, rgba.g, rgba.b, rgba.a));
    let hsla = palette::Hsla::from_color(rgba);

    let hsla = gpui::hsla(
        hsla.hue.into_positive_degrees() / 360.,
        hsla.saturation,
        hsla.lightness,
        hsla.alpha,
    );

    Ok(hsla)
}

/// Newtype for a theme name. Its `ParameterizedJsonSchema` lists the theme names known at runtime.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(transparent)]
pub struct ThemeName(pub Arc<str>);

inventory::submit! {
    ParameterizedJsonSchema {
        add_and_get_ref: |generator, _params, cx| {
            todo!()
            // replace_subschema::<ThemeName>(generator, || json_schema!({
            //     "type": "string",
            //     "enum": ThemeRegistry::global(cx).list_names(),
            // }))
        }
    }
}

/// Newtype for a icon theme name. Its `ParameterizedJsonSchema` lists the icon theme names known at
/// runtime.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(transparent)]
pub struct IconThemeName(pub Arc<str>);

inventory::submit! {
    ParameterizedJsonSchema {
        add_and_get_ref: |generator, _params, cx| {
            todo!()
            // replace_subschema::<IconThemeName>(generator, || json_schema!({
            //     "type": "string",
            //     "enum": ThemeRegistry::global(cx)
            //         .list_icon_themes()
            //         .into_iter()
            //         .map(|icon_theme| icon_theme.name)
            //         .collect::<Vec<SharedString>>(),
            // }))
        }
    }
}
