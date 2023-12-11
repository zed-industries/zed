use serde::{Deserialize, Deserializer};

use crate::vscode::VsCodeTokenColor;

fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.filter(|value| !value.is_empty()))
}

#[derive(Deserialize, Debug)]
pub struct VsCodeTheme {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub maintainers: Option<Vec<String>>,
    #[serde(rename = "semanticClass")]
    pub semantic_class: Option<String>,
    #[serde(rename = "semanticHighlighting")]
    pub semantic_highlighting: Option<bool>,
    pub colors: VsCodeColors,
    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<VsCodeTokenColor>,
}

#[derive(Debug, Deserialize)]
pub struct VsCodeColors {
    #[serde(
        default,
        rename = "terminal.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_background: Option<String>,

    #[serde(
        default,
        rename = "terminal.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_foreground: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightBlack",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_black: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightRed",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_red: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightGreen",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_green: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightYellow",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_yellow: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightBlue",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_blue: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightMagenta",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_magenta: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightCyan",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_cyan: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBrightWhite",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_bright_white: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBlack",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_black: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiRed",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_red: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiGreen",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_green: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiYellow",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_yellow: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiBlue",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_blue: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiMagenta",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_magenta: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiCyan",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_cyan: Option<String>,

    #[serde(
        default,
        rename = "terminal.ansiWhite",
        deserialize_with = "empty_string_as_none"
    )]
    pub terminal_ansi_white: Option<String>,

    #[serde(
        default,
        rename = "focusBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_border: Option<String>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub foreground: Option<String>,

    #[serde(
        default,
        rename = "selection.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,

    #[serde(
        default,
        rename = "errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,

    #[serde(
        default,
        rename = "button.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub button_background: Option<String>,

    #[serde(
        default,
        rename = "button.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub button_foreground: Option<String>,

    #[serde(
        default,
        rename = "button.secondaryBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub button_secondary_background: Option<String>,

    #[serde(
        default,
        rename = "button.secondaryForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub button_secondary_foreground: Option<String>,

    #[serde(
        default,
        rename = "button.secondaryHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub button_secondary_hover_background: Option<String>,

    #[serde(
        default,
        rename = "dropdown.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub dropdown_background: Option<String>,

    #[serde(
        default,
        rename = "dropdown.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub dropdown_border: Option<String>,

    #[serde(
        default,
        rename = "dropdown.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub dropdown_foreground: Option<String>,

    #[serde(
        default,
        rename = "input.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_background: Option<String>,

    #[serde(
        default,
        rename = "input.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_foreground: Option<String>,

    #[serde(
        default,
        rename = "input.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_border: Option<String>,

    #[serde(
        default,
        rename = "input.placeholderForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_placeholder_foreground: Option<String>,

    #[serde(
        default,
        rename = "inputOption.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_option_active_border: Option<String>,

    #[serde(
        default,
        rename = "inputValidation.infoBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_validation_info_border: Option<String>,

    #[serde(
        default,
        rename = "inputValidation.warningBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_validation_warning_border: Option<String>,

    #[serde(
        default,
        rename = "inputValidation.errorBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub input_validation_error_border: Option<String>,

    #[serde(
        default,
        rename = "badge.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub badge_foreground: Option<String>,

    #[serde(
        default,
        rename = "badge.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub badge_background: Option<String>,

    #[serde(
        default,
        rename = "progressBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub progress_bar_background: Option<String>,

    #[serde(
        default,
        rename = "list.activeSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_active_selection_background: Option<String>,

    #[serde(
        default,
        rename = "list.activeSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_active_selection_foreground: Option<String>,

    #[serde(
        default,
        rename = "list.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_drop_background: Option<String>,

    #[serde(
        default,
        rename = "list.focusBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_focus_background: Option<String>,

    #[serde(
        default,
        rename = "list.highlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_highlight_foreground: Option<String>,

    #[serde(
        default,
        rename = "list.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_hover_background: Option<String>,

    #[serde(
        default,
        rename = "list.inactiveSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_inactive_selection_background: Option<String>,

    #[serde(
        default,
        rename = "list.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_warning_foreground: Option<String>,

    #[serde(
        default,
        rename = "list.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_error_foreground: Option<String>,

    #[serde(
        default,
        rename = "activityBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub activity_bar_background: Option<String>,

    #[serde(
        default,
        rename = "activityBar.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub activity_bar_inactive_foreground: Option<String>,

    #[serde(
        default,
        rename = "activityBar.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub activity_bar_foreground: Option<String>,

    #[serde(
        default,
        rename = "activityBar.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub activity_bar_active_border: Option<String>,

    #[serde(
        default,
        rename = "activityBar.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub activity_bar_active_background: Option<String>,

    #[serde(
        default,
        rename = "activityBarBadge.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub activity_bar_badge_background: Option<String>,

    #[serde(
        default,
        rename = "activityBarBadge.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub activity_bar_badge_foreground: Option<String>,

    #[serde(
        default,
        rename = "sideBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub side_bar_background: Option<String>,

    #[serde(
        default,
        rename = "sideBarTitle.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub side_bar_title_foreground: Option<String>,

    #[serde(
        default,
        rename = "sideBarSectionHeader.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub side_bar_section_header_background: Option<String>,

    #[serde(
        default,
        rename = "sideBarSectionHeader.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub side_bar_section_header_border: Option<String>,

    #[serde(
        default,
        rename = "editorGroup.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_group_border: Option<String>,

    #[serde(
        default,
        rename = "editorGroup.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_group_drop_background: Option<String>,

    #[serde(
        default,
        rename = "editorGroupHeader.tabsBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_group_header_tabs_background: Option<String>,

    #[serde(
        default,
        rename = "tab.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub tab_active_background: Option<String>,

    #[serde(
        default,
        rename = "tab.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub tab_active_foreground: Option<String>,

    #[serde(
        default,
        rename = "tab.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub tab_border: Option<String>,

    #[serde(
        default,
        rename = "tab.activeBorderTop",
        deserialize_with = "empty_string_as_none"
    )]
    pub tab_active_border_top: Option<String>,

    #[serde(
        default,
        rename = "tab.inactiveBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub tab_inactive_background: Option<String>,

    #[serde(
        default,
        rename = "tab.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub tab_inactive_foreground: Option<String>,

    #[serde(
        default,
        rename = "editor.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_foreground: Option<String>,

    #[serde(
        default,
        rename = "editor.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_background: Option<String>,

    #[serde(
        default,
        rename = "editorInlayHint.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_inlay_hint_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorInlayHint.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_inlay_hint_background: Option<String>,

    #[serde(
        default,
        rename = "editorInlayHint.parameterForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_inlay_hint_parameter_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorInlayHint.parameterBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_inlay_hint_parameter_background: Option<String>,

    #[serde(
        default,
        rename = "editorInlayHint.typForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_inlay_hint_typ_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorInlayHint.typBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_inlay_hint_typ_background: Option<String>,

    #[serde(
        default,
        rename = "editorLineNumber.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_line_number_foreground: Option<String>,

    #[serde(
        default,
        rename = "editor.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_selection_background: Option<String>,

    #[serde(
        default,
        rename = "editor.selectionHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_selection_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.foldBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_fold_background: Option<String>,

    #[serde(
        default,
        rename = "editor.wordHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_word_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.wordHighlightStrongBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_word_highlight_strong_background: Option<String>,

    #[serde(
        default,
        rename = "editor.findMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_find_match_background: Option<String>,

    #[serde(
        default,
        rename = "editor.findMatchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_find_match_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.findRangeHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_find_range_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.hoverHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_hover_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.lineHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_line_highlight_border: Option<String>,

    #[serde(
        default,
        rename = "editorLink.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_link_active_foreground: Option<String>,

    #[serde(
        default,
        rename = "editor.rangeHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_range_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.snippetTabstopHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_snippet_tabstop_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.snippetTabstopHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_snippet_tabstop_highlight_border: Option<String>,

    #[serde(
        default,
        rename = "editor.snippetFinalTabstopHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_snippet_final_tabstop_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "editor.snippetFinalTabstopHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_snippet_final_tabstop_highlight_border: Option<String>,

    #[serde(
        default,
        rename = "editorWhitespace.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_whitespace_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorIndentGuide.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_indent_guide_background: Option<String>,

    #[serde(
        default,
        rename = "editorIndentGuide.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_indent_guide_active_background: Option<String>,

    #[serde(
        default,
        rename = "editorRuler.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_ruler_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorCodeLens.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_code_lens_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorBracketHighlight.foreground1",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_bracket_highlight_foreground1: Option<String>,

    #[serde(
        default,
        rename = "editorBracketHighlight.foreground2",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_bracket_highlight_foreground2: Option<String>,

    #[serde(
        default,
        rename = "editorBracketHighlight.foreground3",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_bracket_highlight_foreground3: Option<String>,

    #[serde(
        default,
        rename = "editorBracketHighlight.foreground4",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_bracket_highlight_foreground4: Option<String>,

    #[serde(
        default,
        rename = "editorBracketHighlight.foreground5",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_bracket_highlight_foreground5: Option<String>,

    #[serde(
        default,
        rename = "editorBracketHighlight.foreground6",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_bracket_highlight_foreground6: Option<String>,

    #[serde(
        default,
        rename = "editorBracketHighlight.unexpectedBracket.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_bracket_highlight_unexpected_bracket_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_border: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.selectionHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_selection_highlight_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.wordHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_word_highlight_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.wordHighlightStrongForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_word_highlight_strong_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.modifiedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_modified_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.addedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_added_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.deletedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_deleted_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_error_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_warning_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.infoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_info_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorError.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_error_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorWarning.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_warning_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorGutter.modifiedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_gutter_modified_background: Option<String>,

    #[serde(
        default,
        rename = "editorGutter.addedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_gutter_added_background: Option<String>,

    #[serde(
        default,
        rename = "editorGutter.deletedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_gutter_deleted_background: Option<String>,

    #[serde(
        default,
        rename = "gitDecoration.modifiedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub git_decoration_modified_resource_foreground: Option<String>,

    #[serde(
        default,
        rename = "gitDecoration.deletedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub git_decoration_deleted_resource_foreground: Option<String>,

    #[serde(
        default,
        rename = "gitDecoration.untrackedResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub git_decoration_untracked_resource_foreground: Option<String>,

    #[serde(
        default,
        rename = "gitDecoration.ignoredResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub git_decoration_ignored_resource_foreground: Option<String>,

    #[serde(
        default,
        rename = "gitDecoration.conflictingResourceForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub git_decoration_conflicting_resource_foreground: Option<String>,

    #[serde(
        default,
        rename = "diffEditor.insertedTextBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub diff_editor_inserted_text_background: Option<String>,

    #[serde(
        default,
        rename = "diffEditor.removedTextBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub diff_editor_removed_text_background: Option<String>,

    #[serde(
        default,
        rename = "inlineChat.regionHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub inline_chat_region_highlight: Option<String>,

    #[serde(
        default,
        rename = "editorWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_widget_background: Option<String>,

    #[serde(
        default,
        rename = "editorSuggestWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_suggest_widget_background: Option<String>,

    #[serde(
        default,
        rename = "editorSuggestWidget.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_suggest_widget_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorSuggestWidget.selectedBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_suggest_widget_selected_background: Option<String>,

    #[serde(
        default,
        rename = "editorHoverWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_hover_widget_background: Option<String>,

    #[serde(
        default,
        rename = "editorHoverWidget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_hover_widget_border: Option<String>,

    #[serde(
        default,
        rename = "editorMarkerNavigation.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_marker_navigation_background: Option<String>,

    #[serde(
        default,
        rename = "peekView.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_border: Option<String>,

    #[serde(
        default,
        rename = "peekViewEditor.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_editor_background: Option<String>,

    #[serde(
        default,
        rename = "peekViewEditor.matchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_editor_match_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "peekViewResult.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_result_background: Option<String>,

    #[serde(
        default,
        rename = "peekViewResult.fileForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_result_file_foreground: Option<String>,

    #[serde(
        default,
        rename = "peekViewResult.lineForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_result_line_foreground: Option<String>,

    #[serde(
        default,
        rename = "peekViewResult.matchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_result_match_highlight_background: Option<String>,

    #[serde(
        default,
        rename = "peekViewResult.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_result_selection_background: Option<String>,

    #[serde(
        default,
        rename = "peekViewResult.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_result_selection_foreground: Option<String>,

    #[serde(
        default,
        rename = "peekViewTitle.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_title_background: Option<String>,

    #[serde(
        default,
        rename = "peekViewTitleDescription.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_title_description_foreground: Option<String>,

    #[serde(
        default,
        rename = "peekViewTitleLabel.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub peek_view_title_label_foreground: Option<String>,

    #[serde(
        default,
        rename = "merge.currentHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub merge_current_header_background: Option<String>,

    #[serde(
        default,
        rename = "merge.incomingHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub merge_incoming_header_background: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.currentContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_current_content_foreground: Option<String>,

    #[serde(
        default,
        rename = "editorOverviewRuler.incomingContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub editor_overview_ruler_incoming_content_foreground: Option<String>,

    #[serde(
        default,
        rename = "panel.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub panel_background: Option<String>,

    #[serde(
        default,
        rename = "panel.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub panel_border: Option<String>,

    #[serde(
        default,
        rename = "panelTitle.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub panel_title_active_border: Option<String>,

    #[serde(
        default,
        rename = "panelTitle.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub panel_title_active_foreground: Option<String>,

    #[serde(
        default,
        rename = "panelTitle.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub panel_title_inactive_foreground: Option<String>,

    #[serde(
        default,
        rename = "scrollbar.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub scrollbar_shadow: Option<String>,

    #[serde(
        default,
        rename = "scrollbarSlider.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub scrollbar_slider_background: Option<String>,

    #[serde(
        default,
        rename = "scrollbarSlider.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub scrollbar_slider_active_background: Option<String>,

    #[serde(
        default,
        rename = "scrollbarSlider.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub scrollbar_slider_hover_background: Option<String>,

    #[serde(
        default,
        rename = "statusBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_background: Option<String>,

    #[serde(
        default,
        rename = "statusBar.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_foreground: Option<String>,

    #[serde(
        default,
        rename = "statusBar.debuggingBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_debugging_background: Option<String>,

    #[serde(
        default,
        rename = "statusBar.debuggingForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_debugging_foreground: Option<String>,

    #[serde(
        default,
        rename = "statusBar.noFolderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_no_folder_background: Option<String>,

    #[serde(
        default,
        rename = "statusBar.noFolderForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_no_folder_foreground: Option<String>,

    #[serde(
        default,
        rename = "statusBarItem.prominentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_item_prominent_background: Option<String>,

    #[serde(
        default,
        rename = "statusBarItem.prominentHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_item_prominent_hover_background: Option<String>,

    #[serde(
        default,
        rename = "statusBarItem.remoteForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_item_remote_foreground: Option<String>,

    #[serde(
        default,
        rename = "statusBarItem.remoteBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_item_remote_background: Option<String>,

    #[serde(
        default,
        rename = "titleBar.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub title_bar_active_background: Option<String>,

    #[serde(
        default,
        rename = "titleBar.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub title_bar_active_foreground: Option<String>,

    #[serde(
        default,
        rename = "titleBar.inactiveBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub title_bar_inactive_background: Option<String>,

    #[serde(
        default,
        rename = "titleBar.inactiveForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub title_bar_inactive_foreground: Option<String>,

    #[serde(
        default,
        rename = "extensionButton.prominentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub extension_button_prominent_foreground: Option<String>,

    #[serde(
        default,
        rename = "extensionButton.prominentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub extension_button_prominent_background: Option<String>,

    #[serde(
        default,
        rename = "extensionButton.prominentHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub extension_button_prominent_hover_background: Option<String>,

    #[serde(
        default,
        rename = "pickerGroup.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub picker_group_border: Option<String>,

    #[serde(
        default,
        rename = "pickerGroup.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub picker_group_foreground: Option<String>,

    #[serde(
        default,
        rename = "debugToolBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub debug_tool_bar_background: Option<String>,

    #[serde(
        default,
        rename = "walkThrough.embeddedEditorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub walk_through_embedded_editor_background: Option<String>,

    #[serde(
        default,
        rename = "settings.headerForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_header_foreground: Option<String>,

    #[serde(
        default,
        rename = "settings.modifiedItemIndicator",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_modified_item_indicator: Option<String>,

    #[serde(
        default,
        rename = "settings.dropdownBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_dropdown_background: Option<String>,

    #[serde(
        default,
        rename = "settings.dropdownForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_dropdown_foreground: Option<String>,

    #[serde(
        default,
        rename = "settings.dropdownBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_dropdown_border: Option<String>,

    #[serde(
        default,
        rename = "settings.checkboxBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_checkbox_background: Option<String>,

    #[serde(
        default,
        rename = "settings.checkboxForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_checkbox_foreground: Option<String>,

    #[serde(
        default,
        rename = "settings.checkboxBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_checkbox_border: Option<String>,

    #[serde(
        default,
        rename = "settings.textInputBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_text_input_background: Option<String>,

    #[serde(
        default,
        rename = "settings.textInputForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_text_input_foreground: Option<String>,

    #[serde(
        default,
        rename = "settings.textInputBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_text_input_border: Option<String>,

    #[serde(
        default,
        rename = "settings.numberInputBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_number_input_background: Option<String>,

    #[serde(
        default,
        rename = "settings.numberInputForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_number_input_foreground: Option<String>,

    #[serde(
        default,
        rename = "settings.numberInputBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub settings_number_input_border: Option<String>,

    #[serde(
        default,
        rename = "breadcrumb.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breadcrumb_foreground: Option<String>,

    #[serde(
        default,
        rename = "breadcrumb.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub breadcrumb_background: Option<String>,

    #[serde(
        default,
        rename = "breadcrumb.focusForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breadcrumb_focus_foreground: Option<String>,

    #[serde(
        default,
        rename = "breadcrumb.activeSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub breadcrumb_active_selection_foreground: Option<String>,

    #[serde(
        default,
        rename = "breadcrumbPicker.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub breadcrumb_picker_background: Option<String>,

    #[serde(
        default,
        rename = "listFilterWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_filter_widget_background: Option<String>,

    #[serde(
        default,
        rename = "listFilterWidget.outline",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_filter_widget_outline: Option<String>,

    #[serde(
        default,
        rename = "listFilterWidget.noMatchesOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_filter_widget_no_matches_outline: Option<String>,
}
