use core::num;
use std::num::NonZeroU32;

use gpui::App;
use language::CursorShape;
use project::project_settings::DiagnosticSeverity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsUi, VsCodeSettings};
use util::serde::default_true;

/// Imports from the VSCode settings at
/// https://code.visualstudio.com/docs/reference/default-settings
#[derive(Deserialize, Clone)]
pub struct EditorSettings {
    pub cursor_blink: bool,
    pub cursor_shape: Option<CursorShape>,
    pub current_line_highlight: CurrentLineHighlight,
    pub selection_highlight: bool,
    pub rounded_selection: bool,
    pub lsp_highlight_debounce: u64,
    pub hover_popover_enabled: bool,
    pub hover_popover_delay: u64,
    pub status_bar: StatusBar,
    pub toolbar: Toolbar,
    pub scrollbar: Scrollbar,
    pub minimap: Minimap,
    pub gutter: Gutter,
    pub scroll_beyond_last_line: ScrollBeyondLastLine,
    pub vertical_scroll_margin: f32,
    pub autoscroll_on_clicks: bool,
    pub horizontal_scroll_margin: f32,
    pub scroll_sensitivity: f32,
    pub fast_scroll_sensitivity: f32,
    pub relative_line_numbers: bool,
    pub seed_search_query_from_cursor: SeedQuerySetting,
    pub use_smartcase_search: bool,
    pub multi_cursor_modifier: MultiCursorModifier,
    pub redact_private_values: bool,
    pub expand_excerpt_lines: u32,
    pub excerpt_context_lines: u32,
    pub middle_click_paste: bool,
    #[serde(default)]
    pub double_click_in_multibuffer: DoubleClickInMultibuffer,
    pub search_wrap: bool,
    #[serde(default)]
    pub search: SearchSettings,
    pub auto_signature_help: bool,
    pub show_signature_help_after_edits: bool,
    #[serde(default)]
    pub go_to_definition_fallback: GoToDefinitionFallback,
    pub jupyter: Jupyter,
    pub hide_mouse: Option<HideMouseMode>,
    pub snippet_sort_order: SnippetSortOrder,
    #[serde(default)]
    pub diagnostics_max_severity: Option<DiagnosticSeverity>,
    pub inline_code_actions: bool,
    pub drag_and_drop_selection: DragAndDropSelection,
    pub lsp_document_colors: DocumentColorsRenderMode,
    pub minimum_contrast_for_highlights: f32,
}

/// How to render LSP `textDocument/documentColor` colors in the editor.
#[derive(
    Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi,
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi)]
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
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi)]
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
    Default, Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi,
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

#[derive(Debug, Clone, Deserialize)]
pub struct Jupyter {
    /// Whether the Jupyter feature is enabled.
    ///
    /// Default: true
    pub enabled: bool,
}

#[derive(
    Default, Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi,
)]
#[serde(rename_all = "snake_case")]
pub struct JupyterContent {
    /// Whether the Jupyter feature is enabled.
    ///
    /// Default: true
    pub enabled: Option<bool>,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct StatusBar {
    /// Whether to display the active language button in the status bar.
    ///
    /// Default: true
    pub active_language_button: bool,
    /// Whether to show the cursor position button in the status bar.
    ///
    /// Default: true
    pub cursor_position_button: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Toolbar {
    pub breadcrumbs: bool,
    pub quick_actions: bool,
    pub selections_menu: bool,
    pub agent_review: bool,
    pub code_actions: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Scrollbar {
    pub show: ShowScrollbar,
    pub git_diff: bool,
    pub selected_text: bool,
    pub selected_symbol: bool,
    pub search_results: bool,
    pub diagnostics: ScrollbarDiagnostics,
    pub cursors: bool,
    pub axes: ScrollbarAxes,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Minimap {
    pub show: ShowMinimap,
    pub display_in: DisplayIn,
    pub thumb: MinimapThumb,
    pub thumb_border: MinimapThumbBorder,
    pub current_line_highlight: Option<CurrentLineHighlight>,
    pub max_width_columns: num::NonZeroU32,
}

impl Minimap {
    pub fn minimap_enabled(&self) -> bool {
        self.show != ShowMinimap::Never
    }

    #[inline]
    pub fn on_active_editor(&self) -> bool {
        self.display_in == DisplayIn::ActiveEditor
    }

    pub fn with_show_override(self) -> Self {
        Self {
            show: ShowMinimap::Always,
            ..self
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Gutter {
    pub min_line_number_digits: usize,
    pub line_numbers: bool,
    pub runnables: bool,
    pub breakpoints: bool,
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

/// When to show the minimap in the editor.
///
/// Default: never
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DisplayIn {
    /// Show on all open editors.
    AllEditors,
    /// Show the minimap on the active editor only.
    #[default]
    ActiveEditor,
}

/// When to show the minimap thumb.
///
/// Default: always
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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

/// Forcefully enable or disable the scrollbar for each axis
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct ScrollbarAxes {
    /// When false, forcefully disables the horizontal scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    pub horizontal: bool,

    /// When false, forcefully disables the vertical scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    pub vertical: bool,
}

/// Whether to allow drag and drop text selection in buffer.
#[derive(
    Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi,
)]
pub struct DragAndDropSelection {
    /// When true, enables drag and drop text selection in buffer.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// The delay in milliseconds that must elapse before drag and drop is allowed. Otherwise, a new text selection is created.
    ///
    /// Default: 300
    #[serde(default = "default_drag_and_drop_selection_delay_ms")]
    pub delay: u64,
}

fn default_drag_and_drop_selection_delay_ms() -> u64 {
    300
}

/// Which diagnostic indicators to show in the scrollbar.
///
/// Default: all
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi)]
#[serde(rename_all = "snake_case")]
pub enum MultiCursorModifier {
    Alt,
    #[serde(alias = "cmd", alias = "ctrl")]
    CmdOrCtrl,
}

/// Whether the editor will scroll beyond the last line.
///
/// Default: one_page
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi)]
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
#[derive(
    Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi,
)]
pub struct SearchSettings {
    /// Whether to show the project search button in the status bar.
    #[serde(default = "default_true")]
    pub button: bool,
    #[serde(default)]
    pub whole_word: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub include_ignored: bool,
    #[serde(default)]
    pub regex: bool,
}

/// What to do when go to definition yields no results.
#[derive(
    Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi,
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
    Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi,
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
    Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi,
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

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi)]
#[settings_ui(group = "Editor")]
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
    /// Status bar related settings
    pub status_bar: Option<StatusBarContent>,
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
    pub search: Option<SearchSettings>,

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
    pub minimum_contrast_for_highlights: Option<f32>,

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
    #[serde(default)]
    pub diagnostics_max_severity: Option<DiagnosticSeverity>,

    /// Whether to show code action button at start of buffer line.
    ///
    /// Default: true
    pub inline_code_actions: Option<bool>,

    /// Drag and drop related settings
    pub drag_and_drop_selection: Option<DragAndDropSelection>,

    /// How to render LSP `textDocument/documentColor` colors in the editor.
    ///
    /// Default: [`DocumentColorsRenderMode::Inlay`]
    pub lsp_document_colors: Option<DocumentColorsRenderMode>,
}

// Status bar related settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi)]
pub struct StatusBarContent {
    /// Whether to display the active language button in the status bar.
    ///
    /// Default: true
    pub active_language_button: Option<bool>,
    /// Whether to show the cursor position button in the status bar.
    ///
    /// Default: true
    pub cursor_position_button: Option<bool>,
}

// Toolbar related settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi)]
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
#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Default, SettingsUi,
)]
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
#[derive(
    Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, SettingsUi,
)]
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
    pub current_line_highlight: Option<Option<CurrentLineHighlight>>,

    /// Maximum number of columns to display in the minimap.
    ///
    /// Default: 80
    pub max_width_columns: Option<num::NonZeroU32>,
}

/// Forcefully enable or disable the scrollbar for each axis
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
pub struct ScrollbarAxesContent {
    /// When false, forcefully disables the horizontal scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    horizontal: Option<bool>,

    /// When false, forcefully disables the vertical scrollbar. Otherwise, obey other settings.
    ///
    /// Default: true
    vertical: Option<bool>,
}

/// Gutter related settings
#[derive(
    Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, SettingsUi,
)]
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

impl EditorSettings {
    pub fn jupyter_enabled(cx: &App) -> bool {
        EditorSettings::get_global(cx).jupyter.enabled
    }
}

impl Settings for EditorSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = EditorSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &VsCodeSettings, current: &mut Self::FileContent) {
        vscode.enum_setting(
            "editor.cursorBlinking",
            &mut current.cursor_blink,
            |s| match s {
                "blink" | "phase" | "expand" | "smooth" => Some(true),
                "solid" => Some(false),
                _ => None,
            },
        );
        vscode.enum_setting(
            "editor.cursorStyle",
            &mut current.cursor_shape,
            |s| match s {
                "block" => Some(CursorShape::Block),
                "block-outline" => Some(CursorShape::Hollow),
                "line" | "line-thin" => Some(CursorShape::Bar),
                "underline" | "underline-thin" => Some(CursorShape::Underline),
                _ => None,
            },
        );

        vscode.enum_setting(
            "editor.renderLineHighlight",
            &mut current.current_line_highlight,
            |s| match s {
                "gutter" => Some(CurrentLineHighlight::Gutter),
                "line" => Some(CurrentLineHighlight::Line),
                "all" => Some(CurrentLineHighlight::All),
                _ => None,
            },
        );

        vscode.bool_setting(
            "editor.selectionHighlight",
            &mut current.selection_highlight,
        );
        vscode.bool_setting("editor.roundedSelection", &mut current.rounded_selection);
        vscode.bool_setting("editor.hover.enabled", &mut current.hover_popover_enabled);
        vscode.u64_setting("editor.hover.delay", &mut current.hover_popover_delay);

        let mut gutter = GutterContent::default();
        vscode.enum_setting(
            "editor.showFoldingControls",
            &mut gutter.folds,
            |s| match s {
                "always" | "mouseover" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );
        vscode.enum_setting(
            "editor.lineNumbers",
            &mut gutter.line_numbers,
            |s| match s {
                "on" | "relative" => Some(true),
                "off" => Some(false),
                _ => None,
            },
        );
        if let Some(old_gutter) = current.gutter.as_mut() {
            if gutter.folds.is_some() {
                old_gutter.folds = gutter.folds
            }
            if gutter.line_numbers.is_some() {
                old_gutter.line_numbers = gutter.line_numbers
            }
        } else if gutter != GutterContent::default() {
            current.gutter = Some(gutter)
        }
        if let Some(b) = vscode.read_bool("editor.scrollBeyondLastLine") {
            current.scroll_beyond_last_line = Some(if b {
                ScrollBeyondLastLine::OnePage
            } else {
                ScrollBeyondLastLine::Off
            })
        }

        let mut scrollbar_axes = ScrollbarAxesContent::default();
        vscode.enum_setting(
            "editor.scrollbar.horizontal",
            &mut scrollbar_axes.horizontal,
            |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            },
        );
        vscode.enum_setting(
            "editor.scrollbar.vertical",
            &mut scrollbar_axes.horizontal,
            |s| match s {
                "auto" | "visible" => Some(true),
                "hidden" => Some(false),
                _ => None,
            },
        );

        if scrollbar_axes != ScrollbarAxesContent::default() {
            let scrollbar_settings = current.scrollbar.get_or_insert_default();
            let axes_settings = scrollbar_settings.axes.get_or_insert_default();

            if let Some(vertical) = scrollbar_axes.vertical {
                axes_settings.vertical = Some(vertical);
            }
            if let Some(horizontal) = scrollbar_axes.horizontal {
                axes_settings.horizontal = Some(horizontal);
            }
        }

        // TODO: check if this does the int->float conversion?
        vscode.f32_setting(
            "editor.cursorSurroundingLines",
            &mut current.vertical_scroll_margin,
        );
        vscode.f32_setting(
            "editor.mouseWheelScrollSensitivity",
            &mut current.scroll_sensitivity,
        );
        vscode.f32_setting(
            "editor.fastScrollSensitivity",
            &mut current.fast_scroll_sensitivity,
        );
        if Some("relative") == vscode.read_string("editor.lineNumbers") {
            current.relative_line_numbers = Some(true);
        }

        vscode.enum_setting(
            "editor.find.seedSearchStringFromSelection",
            &mut current.seed_search_query_from_cursor,
            |s| match s {
                "always" => Some(SeedQuerySetting::Always),
                "selection" => Some(SeedQuerySetting::Selection),
                "never" => Some(SeedQuerySetting::Never),
                _ => None,
            },
        );
        vscode.bool_setting("search.smartCase", &mut current.use_smartcase_search);
        vscode.enum_setting(
            "editor.multiCursorModifier",
            &mut current.multi_cursor_modifier,
            |s| match s {
                "ctrlCmd" => Some(MultiCursorModifier::CmdOrCtrl),
                "alt" => Some(MultiCursorModifier::Alt),
                _ => None,
            },
        );

        vscode.bool_setting(
            "editor.parameterHints.enabled",
            &mut current.auto_signature_help,
        );
        vscode.bool_setting(
            "editor.parameterHints.enabled",
            &mut current.show_signature_help_after_edits,
        );

        if let Some(use_ignored) = vscode.read_bool("search.useIgnoreFiles") {
            let search = current.search.get_or_insert_default();
            search.include_ignored = use_ignored;
        }

        let mut minimap = MinimapContent::default();
        let minimap_enabled = vscode.read_bool("editor.minimap.enabled").unwrap_or(true);
        let autohide = vscode.read_bool("editor.minimap.autohide");
        let mut max_width_columns: Option<u32> = None;
        vscode.u32_setting("editor.minimap.maxColumn", &mut max_width_columns);
        if minimap_enabled {
            if let Some(false) = autohide {
                minimap.show = Some(ShowMinimap::Always);
            } else {
                minimap.show = Some(ShowMinimap::Auto);
            }
        } else {
            minimap.show = Some(ShowMinimap::Never);
        }
        if let Some(max_width_columns) = max_width_columns {
            minimap.max_width_columns = NonZeroU32::new(max_width_columns);
        }

        vscode.enum_setting(
            "editor.minimap.showSlider",
            &mut minimap.thumb,
            |s| match s {
                "always" => Some(MinimapThumb::Always),
                "mouseover" => Some(MinimapThumb::Hover),
                _ => None,
            },
        );

        if minimap != MinimapContent::default() {
            current.minimap = Some(minimap)
        }
    }
}
