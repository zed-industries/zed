use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct EditorSettings {
    /// Whether the cursor blinks in the editor.
    pub cursor_blink: bool,
    /// How to highlight the current line in the editor.
    pub current_line_highlight: CurrentLineHighlight,
    /// Whether to show the informational hover box when moving the mouse
    /// over symbols in the editor.
    pub hover_popover_enabled: bool,
    /// Whether to pop the completions menu while typing in an editor without
    /// explicitly requesting it.
    pub show_completions_on_input: bool,
    /// Whether to display inline and alongside documentation for items in the
    /// completions menu.
    pub show_completion_documentation: bool,
    /// The debounce delay before re-querying the language server for completion
    /// documentation when not included in original completion list.
    pub completion_documentation_secondary_query_debounce: u64,
    /// Whether to use additional LSP queries to format (and amend) the code after
    /// every "trigger" symbol input, defined by LSP server capabilities.
    pub use_on_type_format: bool,
    /// Toolbar related settings
    pub toolbar: Toolbar,
    /// Scrollbar related settings
    pub scrollbar: Scrollbar,
    /// Gutter related settings
    pub gutter: Gutter,
    /// Whether the editor will scroll beyond the last line.
    pub scroll_beyond_last_line: ScrollBeyondLastLine,
    /// The number of lines to keep above/below the cursor when auto-scrolling.
    pub vertical_scroll_margin: f32,
    /// Scroll sensitivity multiplier. This multiplier is applied
    /// to both the horizontal and vertical delta values while scrolling.
    pub scroll_sensitivity: f32,
    /// Whether the line numbers on editors gutter are relative or not.
    pub relative_line_numbers: bool,
    /// When to populate a new search's query based on the text under the cursor.
    pub seed_search_query_from_cursor: SeedQuerySetting,
    pub use_smartcase_search: bool,
    /// The key to use for adding multiple cursors
    pub multi_cursor_modifier: MultiCursorModifier,
    /// Hide the values of variables in `private` files, as defined by the
    /// private_files setting. This only changes the visual representation,
    /// the values are still present in the file and can be selected / copied / pasted
    pub redact_private_values: bool,

    /// How many lines to expand the multibuffer excerpts by default
    pub expand_excerpt_lines: u32,
    pub middle_click_paste: bool,
    /// What to do when multibuffer is double clicked in some of its excerpts
    /// (parts of singleton buffers).
    #[serde(default)]
    pub double_click_in_multibuffer: DoubleClickInMultibuffer,
    /// Whether the editor search results will loop
    pub search_wrap: bool,
    #[serde(default)]
    pub search: SearchSettings,
    /// Show method signatures in the editor, when inside parentheses.
    pub auto_signature_help: bool,
    /// Whether to show the signature help after completion or a bracket pair inserted.
    /// If `auto_signature_help` is enabled, this setting will be treated as enabled also.
    pub show_signature_help_after_edits: bool,
    /// Jupyter REPL settings.
    pub jupyter: Jupyter,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            cursor_blink: true,
            current_line_highlight: CurrentLineHighlight::All,
            hover_popover_enabled: true,
            show_completions_on_input: true,
            show_completion_documentation: true,
            completion_documentation_secondary_query_debounce: 300,
            use_on_type_format: true,
            toolbar: Default::default(),
            scrollbar: Default::default(),
            gutter: Default::default(),
            scroll_beyond_last_line: ScrollBeyondLastLine::OnePage,
            vertical_scroll_margin: 3.,
            scroll_sensitivity: 1.0,
            relative_line_numbers: false,
            seed_search_query_from_cursor: SeedQuerySetting::Always,
            multi_cursor_modifier: MultiCursorModifier::Alt,
            redact_private_values: false,
            expand_excerpt_lines: 3,
            double_click_in_multibuffer: DoubleClickInMultibuffer::Select,
            search_wrap: true,
            auto_signature_help: false,
            show_signature_help_after_edits: true,
            jupyter: Default::default(),
            use_smartcase_search: false,
            middle_click_paste: true,
            search: SearchSettings::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
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
    #[default]
    Select,
    /// Open the excerpt clicked as a new buffer in the new tab, if no `alt` modifier was pressed during double click.
    /// Otherwise, behave as a regular buffer and select the whole word.
    Open,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct Jupyter {
    /// Whether the Jupyter feature is enabled.
    pub enabled: bool,
}

impl Default for Jupyter {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(default)]
pub struct Toolbar {
    /// Whether to display breadcrumbs in the editor toolbar.
    pub breadcrumbs: bool,
    /// Whether to display quick action buttons in the editor toolbar.
    pub quick_actions: bool,
    /// Whether to show the selections menu in the editor toolbar
    pub selections_menu: bool,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self {
            breadcrumbs: true,
            quick_actions: true,
            selections_menu: true,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Scrollbar {
    /// When to show the scrollbar in the editor.
    pub show: ShowScrollbar,
    /// Whether to show git diff indicators in the scrollbar.
    pub git_diff: bool,
    /// Whether to show buffer search result indicators in the scrollbar.
    pub selected_symbol: bool,
    /// Whether to show selected symbol occurrences in the scrollbar.
    pub search_results: bool,
    /// Whether to show diagnostic indicators in the scrollbar.
    pub diagnostics: bool,
    /// Whether to show cursor positions in the scrollbar.
    pub cursors: bool,
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self {
            show: ShowScrollbar::Auto,
            git_diff: true,
            selected_symbol: true,
            search_results: true,
            diagnostics: true,
            cursors: true,
        }
    }
}

/// Gutter-related settings.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(default)]
pub struct Gutter {
    /// Whether to show line numbers in the gutter.
    pub line_numbers: bool,
    /// Whether to show code action buttons in the gutter.
    pub code_actions: bool,
    /// Whether to show runnable buttons in the gutter.
    pub runnables: bool,
    /// Whether to show fold buttons in the gutter.
    pub folds: bool,
}

impl Default for Gutter {
    fn default() -> Self {
        Self {
            line_numbers: true,
            code_actions: true,
            runnables: true,
            folds: true,
        }
    }
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

/// The key to use for adding multiple cursors
///
/// Default: alt
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MultiCursorModifier {
    Alt,
    #[serde(alias = "cmd", alias = "ctrl")]
    CmdOrCtrl,
}

/// Whether the editor will scroll beyond the last line.
///
/// Default: one_page
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScrollBeyondLastLine {
    /// The editor will not scroll beyond the last line.
    Off,

    /// The editor will scroll beyond the last line by one page.
    OnePage,

    /// The editor will scroll beyond the last line by the same number of lines as vertical_scroll_margin.
    VerticalScrollMargin,
}

/// Default options for buffer and project search items.
#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct SearchSettings {
    #[serde(default)]
    pub whole_word: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub include_ignored: bool,
    #[serde(default)]
    pub regex: bool,
}

impl EditorSettings {
    pub fn jupyter_enabled(cx: &AppContext) -> bool {
        EditorSettings::get_global(cx).jupyter.enabled
    }
}

impl Settings for EditorSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
