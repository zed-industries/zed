use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Deserialize)]
pub struct EditorSettings {
    pub cursor_blink: bool,
    pub hover_popover_enabled: bool,
    pub show_completions_on_input: bool,
    pub show_completion_documentation: bool,
    pub use_on_type_format: bool,
    pub scrollbar: Scrollbar,
    pub relative_line_numbers: bool,
    pub seed_search_query_from_cursor: SeedQuerySetting,
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Scrollbar {
    pub show: ShowScrollbar,
    pub git_diff: bool,
    pub selections: bool,
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
    /// Whether to use additional LSP queries to format (and amend) the code after
    /// every "trigger" symbol input, defined by LSP server capabilities.
    ///
    /// Default: true
    pub use_on_type_format: Option<bool>,
    /// Scrollbar related settings
    pub scrollbar: Option<ScrollbarContent>,
    /// Whether the line numbers on editors gutter are relative or not.
    ///
    /// Default: false
    pub relative_line_numbers: Option<bool>,
    /// When to populate a new search's query based on the text under the cursor.
    ///
    /// Default: always
    pub seed_search_query_from_cursor: Option<SeedQuerySetting>,
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
