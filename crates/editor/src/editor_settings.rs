use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Deserialize, Clone)]
pub struct EditorSettings {
    pub cursor_blink: bool,
    pub hover_popover_enabled: bool,
    pub show_completions_on_input: bool,
    pub show_completion_documentation: bool,
    pub completion_documentation_secondary_query_debounce: u64,
    pub use_on_type_format: bool,
    pub toolbar: Toolbar,
    pub scrollbar: Scrollbar,
    pub gutter: Gutter,
    pub vertical_scroll_margin: f32,
    pub relative_line_numbers: bool,
    pub seed_search_query_from_cursor: SeedQuerySetting,
    pub redact_private_values: bool,
    #[serde(default)]
    pub double_click_in_multibuffer: DoubleClickInMultibuffer,
}

/// When to populate a new search's query based on the text under the cursor.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
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
#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DoubleClickInMultibuffer {
    /// Behave as a regular buffer and select the whole word.
    Select,
    #[default]
    /// Open the excerpt clicked as a new buffer in the new tab, if no `alt` modifier was pressed during double click.
    /// Otherwise, behave as a regular buffer and select the whole word.
    Open,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Toolbar {
    pub breadcrumbs: bool,
    pub quick_actions: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Scrollbar {
    pub show: ShowScrollbar,
    pub git_diff: bool,
    pub selections: bool,
    pub symbols_selections: bool,
    pub diagnostics: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Gutter {
    pub line_numbers: bool,
    pub code_actions: bool,
    pub folds: bool,
}

/// When to show the scrollbar in the editor.
///
/// Default: auto
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowScrollbar {
    /// Show the scrollbar if there's important information or
    /// follow the system's configured behavior.
    Auto,
    /// Match the system's configured behavior.
    System,
    /// Always show the scrollbar.
    Always,
    /// Never show the scrollbar.
    Never,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct EditorSettingsContent {
    /// Whether the cursor blinks in the editor.
    ///
    /// Default: true
    pub cursor_blink: Option<bool>,
    /// Whether to show the informational hover box when moving the mouse
    /// over symbols in the editor.
    ///
    /// Default: true
    pub hover_popover_enabled: Option<bool>,
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
    /// The debounce delay before re-querying the language server for completion
    /// documentation when not included in original completion list.
    ///
    /// Default: 300 ms
    pub completion_documentation_secondary_query_debounce: Option<u64>,
    /// Whether to use additional LSP queries to format (and amend) the code after
    /// every "trigger" symbol input, defined by LSP server capabilities.
    ///
    /// Default: true
    pub use_on_type_format: Option<bool>,
    /// Toolbar related settings
    pub toolbar: Option<ToolbarContent>,
    /// Scrollbar related settings
    pub scrollbar: Option<ScrollbarContent>,
    /// Gutter related settings
    pub gutter: Option<GutterContent>,

    /// The number of lines to keep above/below the cursor when auto-scrolling.
    ///
    /// Default: 3.
    pub vertical_scroll_margin: Option<f32>,
    /// Whether the line numbers on editors gutter are relative or not.
    ///
    /// Default: false
    pub relative_line_numbers: Option<bool>,
    /// When to populate a new search's query based on the text under the cursor.
    ///
    /// Default: always
    pub seed_search_query_from_cursor: Option<SeedQuerySetting>,

    /// Hide the values of variables in `private` files, as defined by the
    /// private_files setting. This only changes the visual representation,
    /// the values are still present in the file and can be selected / copied / pasted
    ///
    /// Default: false
    pub redact_private_values: Option<bool>,

    /// What to do when multibuffer is double clicked in some of its excerpts
    /// (parts of singleton buffers).
    ///
    /// Default: open
    pub double_click_in_multibuffer: Option<DoubleClickInMultibuffer>,
}

// Toolbar related settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ToolbarContent {
    /// Whether to display breadcrumbs in the editor toolbar.
    ///
    /// Default: true
    pub breadcrumbs: Option<bool>,
    /// Whether to display quik action buttons in the editor toolbar.
    ///
    /// Default: true
    pub quick_actions: Option<bool>,
}

/// Scrollbar related settings
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarContent {
    /// When to show the scrollbar in the editor.
    ///
    /// Default: auto
    pub show: Option<ShowScrollbar>,
    /// Whether to show git diff indicators in the scrollbar.
    ///
    /// Default: true
    pub git_diff: Option<bool>,
    /// Whether to show buffer search result markers in the scrollbar.
    ///
    /// Default: true
    pub selections: Option<bool>,
    /// Whether to show symbols highlighted markers in the scrollbar.
    ///
    /// Default: true
    pub symbols_selections: Option<bool>,
    /// Whether to show diagnostic indicators in the scrollbar.
    ///
    /// Default: true
    pub diagnostics: Option<bool>,
}

/// Gutter related settings
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct GutterContent {
    /// Whether to show line numbers in the gutter.
    ///
    /// Default: true
    pub line_numbers: Option<bool>,
    /// Whether to show code action buttons in the gutter.
    ///
    /// Default: true
    pub code_actions: Option<bool>,
    /// Whether to show fold buttons in the gutter.
    ///
    /// Default: true
    pub folds: Option<bool>,
}

impl Settings for EditorSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = EditorSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
