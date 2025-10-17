use std::fmt::Display;
use std::num;

use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use settings_macros::MergeFrom;

use crate::{DiagnosticSeverityContent, ShowScrollbar};

#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct EditorSettingsContent {
    /// Whether the cursor blinks in the editor.
    ///
    /// Default: true
    pub cursor_blink: Option<bool>,
    /// Cursor shape for the default editor.
    /// Can be "bar", "block", "underline", or "hollow".
    ///
    /// Default: bar
    pub cursor_shape: Option<CursorShape>,
    /// Determines when the mouse cursor should be hidden in an editor or input box.
    ///
    /// Default: on_typing_and_movement
    pub hide_mouse: Option<HideMouseMode>,
    /// Determines how snippets are sorted relative to other completion items.
    ///
    /// Default: inline
    pub snippet_sort_order: Option<SnippetSortOrder>,
    /// How to highlight the current line in the editor.
    ///
    /// Default: all
    pub current_line_highlight: Option<CurrentLineHighlight>,
    /// Whether to highlight all occurrences of the selected text in an editor.
    ///
    /// Default: true
    pub selection_highlight: Option<bool>,
    /// Whether the text selection should have rounded corners.
    ///
    /// Default: true
    pub rounded_selection: Option<bool>,
    /// The debounce delay before querying highlights from the language
    /// server based on the current cursor location.
    ///
    /// Default: 75
    pub lsp_highlight_debounce: Option<u64>,
    /// Whether to show the informational hover box when moving the mouse
    /// over symbols in the editor.
    ///
    /// Default: true
    pub hover_popover_enabled: Option<bool>,
    /// Time to wait in milliseconds before showing the informational hover box.
    ///
    /// Default: 300
    pub hover_popover_delay: Option<u64>,
    /// Toolbar related settings
    pub toolbar: Option<ToolbarContent>,
    /// Scrollbar related settings
    pub scrollbar: Option<ScrollbarContent>,
    /// Minimap related settings
    pub minimap: Option<MinimapContent>,
    /// Gutter related settings
    pub gutter: Option<GutterContent>,
    /// Whether the editor will scroll beyond the last line.
    ///
    /// Default: one_page
    pub scroll_beyond_last_line: Option<ScrollBeyondLastLine>,
    /// The number of lines to keep above/below the cursor when auto-scrolling.
    ///
    /// Default: 3.
    pub vertical_scroll_margin: Option<f32>,
    /// Whether to scroll when clicking near the edge of the visible text area.
    ///
    /// Default: false
    pub autoscroll_on_clicks: Option<bool>,
    /// The number of characters to keep on either side when scrolling with the mouse.
    ///
    /// Default: 5.
    pub horizontal_scroll_margin: Option<f32>,
    /// Scroll sensitivity multiplier. This multiplier is applied
    /// to both the horizontal and vertical delta values while scrolling.
    ///
    /// Default: 1.0
    pub scroll_sensitivity: Option<f32>,
    /// Scroll sensitivity multiplier for fast scrolling. This multiplier is applied
    /// to both the horizontal and vertical delta values while scrolling. Fast scrolling
    /// happens when a user holds the alt or option key while scrolling.
    ///
    /// Default: 4.0
    pub fast_scroll_sensitivity: Option<f32>,
    /// Whether the line numbers on editors gutter are relative or not.
    ///
    /// Default: false
    pub relative_line_numbers: Option<bool>,
    /// When to populate a new search's query based on the text under the cursor.
    ///
    /// Default: always
    pub seed_search_query_from_cursor: Option<SeedQuerySetting>,
    pub use_smartcase_search: Option<bool>,
    /// Determines the modifier to be used to add multiple cursors with the mouse. The open hover link mouse gestures will adapt such that it do not conflict with the multicursor modifier.
    ///
    /// Default: alt
    pub multi_cursor_modifier: Option<MultiCursorModifier>,
    /// Hide the values of variables in `private` files, as defined by the
    /// private_files setting. This only changes the visual representation,
    /// the values are still present in the file and can be selected / copied / pasted
    ///
    /// Default: false
    pub redact_private_values: Option<bool>,

    /// How many lines to expand the multibuffer excerpts by default
    ///
    /// Default: 3
    pub expand_excerpt_lines: Option<u32>,

    /// How many lines of context to provide in multibuffer excerpts by default
    ///
    /// Default: 2
    pub excerpt_context_lines: Option<u32>,

    /// Whether to enable middle-click paste on Linux
    ///
    /// Default: true
    pub middle_click_paste: Option<bool>,

    /// What to do when multibuffer is double clicked in some of its excerpts
    /// (parts of singleton buffers).
    ///
    /// Default: select
    pub double_click_in_multibuffer: Option<DoubleClickInMultibuffer>,
    /// Whether the editor search results will loop
    ///
    /// Default: true
    pub search_wrap: Option<bool>,

    /// Defaults to use when opening a new buffer and project search items.
    ///
    /// Default: nothing is enabled
    pub search: Option<SearchSettingsContent>,

    /// Whether to automatically show a signature help pop-up or not.
    ///
    /// Default: false
    pub auto_signature_help: Option<bool>,

    /// Whether to show the signature help pop-up after completions or bracket pairs inserted.
    ///
    /// Default: false
    pub show_signature_help_after_edits: Option<bool>,
    /// The minimum APCA perceptual contrast to maintain when
    /// rendering text over highlight backgrounds in the editor.
    ///
    /// Values range from 0 to 106. Set to 0 to disable adjustments.
    /// Default: 45
    #[schemars(range(min = 0, max = 106))]
    pub minimum_contrast_for_highlights: Option<MinimumContrast>,

    /// Whether to follow-up empty go to definition responses from the language server or not.
    /// `FindAllReferences` allows to look up references of the same symbol instead.
    /// `None` disables the fallback.
    ///
    /// Default: FindAllReferences
    pub go_to_definition_fallback: Option<GoToDefinitionFallback>,

    /// Jupyter REPL settings.
    pub jupyter: Option<JupyterContent>,

    /// Which level to use to filter out diagnostics displayed in the editor.
    ///
    /// Affects the editor rendering only, and does not interrupt
    /// the functionality of diagnostics fetching and project diagnostics editor.
    /// Which files containing diagnostic errors/warnings to mark in the tabs.
    /// Diagnostics are only shown when file icons are also active.
    ///
    /// Shows all diagnostics if not specified.
    ///
    /// Default: warning
    pub diagnostics_max_severity: Option<DiagnosticSeverityContent>,

    /// Whether to show code action button at start of buffer line.
    ///
    /// Default: true
    pub inline_code_actions: Option<bool>,

    /// Drag and drop related settings
    pub drag_and_drop_selection: Option<DragAndDropSelectionContent>,

    /// How to render LSP `textDocument/documentColor` colors in the editor.
    ///
    /// Default: [`DocumentColorsRenderMode::Inlay`]
    pub lsp_document_colors: Option<DocumentColorsRenderMode>,
}

// Toolbar related settings
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct ToolbarContent {
    /// Whether to display breadcrumbs in the editor toolbar.
    ///
    /// Default: true
    pub breadcrumbs: Option<bool>,
    /// Whether to display quick action buttons in the editor toolbar.
    ///
    /// Default: true
    pub quick_actions: Option<bool>,
    /// Whether to show the selections menu in the editor toolbar.
    ///
    /// Default: true
    pub selections_menu: Option<bool>,
    /// Whether to display Agent review buttons in the editor toolbar.
    /// Only applicable while reviewing a file edited by the Agent.
    ///
    /// Default: true
    pub agent_review: Option<bool>,
    /// Whether to display code action buttons in the editor toolbar.
    ///
    /// Default: false
    pub code_actions: Option<bool>,
}

/// Scrollbar related settings
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Default)]
pub struct ScrollbarContent {
    /// When to show the scrollbar in the editor.
    ///
    /// Default: auto
    pub show: Option<ShowScrollbar>,
    /// Whether to show git diff indicators in the scrollbar.
    ///
    /// Default: true
    pub git_diff: Option<bool>,
    /// Whether to show buffer search result indicators in the scrollbar.
    ///
    /// Default: true
    pub search_results: Option<bool>,
    /// Whether to show selected text occurrences in the scrollbar.
    ///
    /// Default: true
    pub selected_text: Option<bool>,
    /// Whether to show selected symbol occurrences in the scrollbar.
    ///
    /// Default: true
    pub selected_symbol: Option<bool>,
    /// Which diagnostic indicators to show in the scrollbar:
    ///
    /// Default: all
    pub diagnostics: Option<ScrollbarDiagnostics>,
    /// Whether to show cursor positions in the scrollbar.
    ///
    /// Default: true
    pub cursors: Option<bool>,
    /// Forcefully enable or disable the scrollbar for each axis
    pub axes: Option<ScrollbarAxesContent>,
}

/// Minimap related settings
#[skip_serializing_none]
#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct MinimapContent {
    /// When to show the minimap in the editor.
    ///
    /// Default: never
    pub show: Option<ShowMinimap>,

    /// Where to show the minimap in the editor.
    ///
    /// Default: [`DisplayIn::ActiveEditor`]
    pub display_in: Option<DisplayIn>,

    /// When to show the minimap thumb.
    ///
    /// Default: always
    pub thumb: Option<MinimapThumb>,

    /// Defines the border style for the minimap's scrollbar thumb.
    ///
    /// Default: left_open
    pub thumb_border: Option<MinimapThumbBorder>,

    /// How to highlight the current line in the minimap.
    ///
    /// Default: inherits editor line highlights setting
    pub current_line_highlight: Option<CurrentLineHighlight>,

    /// Maximum number of columns to display in the minimap.
    ///
    /// Default: 80
    pub max_width_columns: Option<num::NonZeroU32>,
}

/// Forcefully enable or disable the scrollbar for each axis
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Default)]
pub struct ScrollbarAxesContent {
    /// When false, forcefully disables the horizontal scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    pub horizontal: Option<bool>,

    /// When false, forcefully disables the vertical scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    pub vertical: Option<bool>,
}

/// Gutter related settings
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct GutterContent {
    /// Whether to show line numbers in the gutter.
    ///
    /// Default: true
    pub line_numbers: Option<bool>,
    /// Minimum number of characters to reserve space for in the gutter.
    ///
    /// Default: 4
    pub min_line_number_digits: Option<usize>,
    /// Whether to show runnable buttons in the gutter.
    ///
    /// Default: true
    pub runnables: Option<bool>,
    /// Whether to show breakpoints in the gutter.
    ///
    /// Default: true
    pub breakpoints: Option<bool>,
    /// Whether to show fold buttons in the gutter.
    ///
    /// Default: true
    pub folds: Option<bool>,
}

/// How to render LSP `textDocument/documentColor` colors in the editor.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
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
pub enum DocumentColorsRenderMode {
    /// Do not query and render document colors.
    None,
    /// Render document colors as inlay hints near the color text.
    #[default]
    Inlay,
    /// Draw a border around the color text.
    Border,
    /// Draw a background behind the color text.
    Background,
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
pub enum CurrentLineHighlight {
    // Don't highlight the current line.
    None,
    // Highlight the gutter area.
    Gutter,
    // Highlight the editor area.
    Line,
    // Highlight the full line.
    All,
}

/// When to populate a new search's query based on the text under the cursor.
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
pub enum SeedQuerySetting {
    /// Always populate the search query with the word under the cursor.
    Always,
    /// Only populate the search query when there is text selected.
    Selection,
    /// Never populate the search query
    Never,
}

/// What to do when multibuffer is double clicked in some of its excerpts (parts of singleton buffers).
#[derive(
    Default,
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
pub enum DoubleClickInMultibuffer {
    /// Behave as a regular buffer and select the whole word.
    #[default]
    Select,
    /// Open the excerpt clicked as a new buffer in the new tab, if no `alt` modifier was pressed during double click.
    /// Otherwise, behave as a regular buffer and select the whole word.
    Open,
}

/// When to show the minimap thumb.
///
/// Default: always
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum MinimapThumb {
    /// Show the minimap thumb only when the mouse is hovering over the minimap.
    Hover,
    /// Always show the minimap thumb.
    #[default]
    Always,
}

/// Defines the border style for the minimap's scrollbar thumb.
///
/// Default: left_open
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum MinimapThumbBorder {
    /// Displays a border on all sides of the thumb.
    Full,
    /// Displays a border on all sides except the left side of the thumb.
    #[default]
    LeftOpen,
    /// Displays a border on all sides except the right side of the thumb.
    RightOpen,
    /// Displays a border only on the left side of the thumb.
    LeftOnly,
    /// Displays the thumb without any border.
    None,
}

/// Which diagnostic indicators to show in the scrollbar.
///
/// Default: all
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum ScrollbarDiagnostics {
    /// Show all diagnostic levels: hint, information, warnings, error.
    All,
    /// Show only the following diagnostic levels: information, warning, error.
    Information,
    /// Show only the following diagnostic levels: warning, error.
    Warning,
    /// Show only the following diagnostic level: error.
    Error,
    /// Do not show diagnostics.
    None,
}

/// The key to use for adding multiple cursors
///
/// Default: alt
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum MultiCursorModifier {
    Alt,
    #[serde(alias = "cmd", alias = "ctrl")]
    CmdOrCtrl,
}

/// Whether the editor will scroll beyond the last line.
///
/// Default: one_page
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ScrollBeyondLastLine {
    /// The editor will not scroll beyond the last line.
    Off,

    /// The editor will scroll beyond the last line by one page.
    OnePage,

    /// The editor will scroll beyond the last line by the same number of lines as vertical_scroll_margin.
    VerticalScrollMargin,
}

/// The shape of a selection cursor.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
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
pub enum CursorShape {
    /// A vertical bar
    #[default]
    Bar,
    /// A block that surrounds the following character
    Block,
    /// An underline that runs along the following character
    Underline,
    /// A box drawn around the following character
    Hollow,
}

/// What to do when go to definition yields no results.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
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
pub enum GoToDefinitionFallback {
    /// Disables the fallback.
    None,
    /// Looks up references of the same symbol instead.
    #[default]
    FindAllReferences,
}

/// Determines when the mouse cursor should be hidden in an editor or input box.
///
/// Default: on_typing_and_movement
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
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
pub enum HideMouseMode {
    /// Never hide the mouse cursor
    Never,
    /// Hide only when typing
    OnTyping,
    /// Hide on both typing and cursor movement
    #[default]
    OnTypingAndMovement,
}

/// Determines how snippets are sorted relative to other completion items.
///
/// Default: inline
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
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
pub enum SnippetSortOrder {
    /// Place snippets at the top of the completion list
    Top,
    /// Sort snippets normally using the default comparison logic
    #[default]
    Inline,
    /// Place snippets at the bottom of the completion list
    Bottom,
    /// Do not show snippets in the completion list
    None,
}

/// Default options for buffer and project search items.
#[skip_serializing_none]
#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct SearchSettingsContent {
    /// Whether to show the project search button in the status bar.
    pub button: Option<bool>,
    pub whole_word: Option<bool>,
    pub case_sensitive: Option<bool>,
    pub include_ignored: Option<bool>,
    pub regex: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct JupyterContent {
    /// Whether the Jupyter feature is enabled.
    ///
    /// Default: true
    pub enabled: Option<bool>,

    /// Default kernels to select for each language.
    ///
    /// Default: `{}`
    pub kernel_selections: Option<HashMap<String, String>>,
}

/// Whether to allow drag and drop text selection in buffer.
#[skip_serializing_none]
#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct DragAndDropSelectionContent {
    /// When true, enables drag and drop text selection in buffer.
    ///
    /// Default: true
    pub enabled: Option<bool>,

    /// The delay in milliseconds that must elapse before drag and drop is allowed. Otherwise, a new text selection is created.
    ///
    /// Default: 300
    pub delay: Option<u64>,
}

/// When to show the minimap in the editor.
///
/// Default: never
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ShowMinimap {
    /// Follow the visibility of the scrollbar.
    Auto,
    /// Always show the minimap.
    Always,
    /// Never show the minimap.
    #[default]
    Never,
}

/// Where to show the minimap in the editor.
///
/// Default: all_editors
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum DisplayIn {
    /// Show on all open editors.
    AllEditors,
    /// Show the minimap on the active editor only.
    #[default]
    ActiveEditor,
}

/// Minimum APCA perceptual contrast for text over highlight backgrounds.
///
/// Valid range: 0.0 to 106.0
/// Default: 45.0
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    PartialOrd,
    derive_more::FromStr,
)]
#[serde(transparent)]
pub struct MinimumContrast(pub f32);

impl Display for MinimumContrast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
