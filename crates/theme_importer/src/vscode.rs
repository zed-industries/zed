use anyhow::Result;
use gpui::{Hsla, Rgba};
use serde::Deserialize;
use theme::{ThemeColorsRefinement, UserTheme, UserThemeStylesRefinement};

use crate::util::Traverse;
use crate::ThemeMetadata;

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
}

#[derive(Debug, Deserialize)]
pub struct VsCodeColors {
    #[serde(rename = "terminal.background")]
    pub terminal_background: Option<String>,
    #[serde(rename = "terminal.foreground")]
    pub terminal_foreground: Option<String>,
    #[serde(rename = "terminal.ansiBrightBlack")]
    pub terminal_ansi_bright_black: Option<String>,
    #[serde(rename = "terminal.ansiBrightRed")]
    pub terminal_ansi_bright_red: Option<String>,
    #[serde(rename = "terminal.ansiBrightGreen")]
    pub terminal_ansi_bright_green: Option<String>,
    #[serde(rename = "terminal.ansiBrightYellow")]
    pub terminal_ansi_bright_yellow: Option<String>,
    #[serde(rename = "terminal.ansiBrightBlue")]
    pub terminal_ansi_bright_blue: Option<String>,
    #[serde(rename = "terminal.ansiBrightMagenta")]
    pub terminal_ansi_bright_magenta: Option<String>,
    #[serde(rename = "terminal.ansiBrightCyan")]
    pub terminal_ansi_bright_cyan: Option<String>,
    #[serde(rename = "terminal.ansiBrightWhite")]
    pub terminal_ansi_bright_white: Option<String>,
    #[serde(rename = "terminal.ansiBlack")]
    pub terminal_ansi_black: Option<String>,
    #[serde(rename = "terminal.ansiRed")]
    pub terminal_ansi_red: Option<String>,
    #[serde(rename = "terminal.ansiGreen")]
    pub terminal_ansi_green: Option<String>,
    #[serde(rename = "terminal.ansiYellow")]
    pub terminal_ansi_yellow: Option<String>,
    #[serde(rename = "terminal.ansiBlue")]
    pub terminal_ansi_blue: Option<String>,
    #[serde(rename = "terminal.ansiMagenta")]
    pub terminal_ansi_magenta: Option<String>,
    #[serde(rename = "terminal.ansiCyan")]
    pub terminal_ansi_cyan: Option<String>,
    #[serde(rename = "terminal.ansiWhite")]
    pub terminal_ansi_white: Option<String>,
    #[serde(rename = "focusBorder")]
    pub focus_border: Option<String>,
    pub foreground: Option<String>,
    #[serde(rename = "selection.background")]
    pub selection_background: Option<String>,
    #[serde(rename = "errorForeground")]
    pub error_foreground: Option<String>,
    #[serde(rename = "button.background")]
    pub button_background: Option<String>,
    #[serde(rename = "button.foreground")]
    pub button_foreground: Option<String>,
    #[serde(rename = "button.secondaryBackground")]
    pub button_secondary_background: Option<String>,
    #[serde(rename = "button.secondaryForeground")]
    pub button_secondary_foreground: Option<String>,
    #[serde(rename = "button.secondaryHoverBackground")]
    pub button_secondary_hover_background: Option<String>,
    #[serde(rename = "dropdown.background")]
    pub dropdown_background: Option<String>,
    #[serde(rename = "dropdown.border")]
    pub dropdown_border: Option<String>,
    #[serde(rename = "dropdown.foreground")]
    pub dropdown_foreground: Option<String>,
    #[serde(rename = "input.background")]
    pub input_background: Option<String>,
    #[serde(rename = "input.foreground")]
    pub input_foreground: Option<String>,
    #[serde(rename = "input.border")]
    pub input_border: Option<String>,
    #[serde(rename = "input.placeholderForeground")]
    pub input_placeholder_foreground: Option<String>,
    #[serde(rename = "inputOption.activeBorder")]
    pub input_option_active_border: Option<String>,
    #[serde(rename = "inputValidation.infoBorder")]
    pub input_validation_info_border: Option<String>,
    #[serde(rename = "inputValidation.warningBorder")]
    pub input_validation_warning_border: Option<String>,
    #[serde(rename = "inputValidation.errorBorder")]
    pub input_validation_error_border: Option<String>,
    #[serde(rename = "badge.foreground")]
    pub badge_foreground: Option<String>,
    #[serde(rename = "badge.background")]
    pub badge_background: Option<String>,
    #[serde(rename = "progressBar.background")]
    pub progress_bar_background: Option<String>,
    #[serde(rename = "list.activeSelectionBackground")]
    pub list_active_selection_background: Option<String>,
    #[serde(rename = "list.activeSelectionForeground")]
    pub list_active_selection_foreground: Option<String>,
    #[serde(rename = "list.dropBackground")]
    pub list_drop_background: Option<String>,
    #[serde(rename = "list.focusBackground")]
    pub list_focus_background: Option<String>,
    #[serde(rename = "list.highlightForeground")]
    pub list_highlight_foreground: Option<String>,
    #[serde(rename = "list.hoverBackground")]
    pub list_hover_background: Option<String>,
    #[serde(rename = "list.inactiveSelectionBackground")]
    pub list_inactive_selection_background: Option<String>,
    #[serde(rename = "list.warningForeground")]
    pub list_warning_foreground: Option<String>,
    #[serde(rename = "list.errorForeground")]
    pub list_error_foreground: Option<String>,
    #[serde(rename = "activityBar.background")]
    pub activity_bar_background: Option<String>,
    #[serde(rename = "activityBar.inactiveForeground")]
    pub activity_bar_inactive_foreground: Option<String>,
    #[serde(rename = "activityBar.foreground")]
    pub activity_bar_foreground: Option<String>,
    #[serde(rename = "activityBar.activeBorder")]
    pub activity_bar_active_border: Option<String>,
    #[serde(rename = "activityBar.activeBackground")]
    pub activity_bar_active_background: Option<String>,
    #[serde(rename = "activityBarBadge.background")]
    pub activity_bar_badge_background: Option<String>,
    #[serde(rename = "activityBarBadge.foreground")]
    pub activity_bar_badge_foreground: Option<String>,
    #[serde(rename = "sideBar.background")]
    pub side_bar_background: Option<String>,
    #[serde(rename = "sideBarTitle.foreground")]
    pub side_bar_title_foreground: Option<String>,
    #[serde(rename = "sideBarSectionHeader.background")]
    pub side_bar_section_header_background: Option<String>,
    #[serde(rename = "sideBarSectionHeader.border")]
    pub side_bar_section_header_border: Option<String>,
    #[serde(rename = "editorGroup.border")]
    pub editor_group_border: Option<String>,
    #[serde(rename = "editorGroup.dropBackground")]
    pub editor_group_drop_background: Option<String>,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    pub editor_group_header_tabs_background: Option<String>,
    #[serde(rename = "tab.activeBackground")]
    pub tab_active_background: Option<String>,
    #[serde(rename = "tab.activeForeground")]
    pub tab_active_foreground: Option<String>,
    #[serde(rename = "tab.border")]
    pub tab_border: Option<String>,
    #[serde(rename = "tab.activeBorderTop")]
    pub tab_active_border_top: Option<String>,
    #[serde(rename = "tab.inactiveBackground")]
    pub tab_inactive_background: Option<String>,
    #[serde(rename = "tab.inactiveForeground")]
    pub tab_inactive_foreground: Option<String>,
    #[serde(rename = "editor.foreground")]
    pub editor_foreground: Option<String>,
    #[serde(rename = "editor.background")]
    pub editor_background: Option<String>,
    #[serde(rename = "editorLineNumber.foreground")]
    pub editor_line_number_foreground: Option<String>,
    #[serde(rename = "editor.selectionBackground")]
    pub editor_selection_background: Option<String>,
    #[serde(rename = "editor.selectionHighlightBackground")]
    pub editor_selection_highlight_background: Option<String>,
    #[serde(rename = "editor.foldBackground")]
    pub editor_fold_background: Option<String>,
    #[serde(rename = "editor.wordHighlightBackground")]
    pub editor_word_highlight_background: Option<String>,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    pub editor_word_highlight_strong_background: Option<String>,
    #[serde(rename = "editor.findMatchBackground")]
    pub editor_find_match_background: Option<String>,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    pub editor_find_match_highlight_background: Option<String>,
    #[serde(rename = "editor.findRangeHighlightBackground")]
    pub editor_find_range_highlight_background: Option<String>,
    #[serde(rename = "editor.hoverHighlightBackground")]
    pub editor_hover_highlight_background: Option<String>,
    #[serde(rename = "editor.lineHighlightBorder")]
    pub editor_line_highlight_border: Option<String>,
    #[serde(rename = "editorLink.activeForeground")]
    pub editor_link_active_foreground: Option<String>,
    #[serde(rename = "editor.rangeHighlightBackground")]
    pub editor_range_highlight_background: Option<String>,
    #[serde(rename = "editor.snippetTabstopHighlightBackground")]
    pub editor_snippet_tabstop_highlight_background: Option<String>,
    #[serde(rename = "editor.snippetTabstopHighlightBorder")]
    pub editor_snippet_tabstop_highlight_border: Option<String>,
    #[serde(rename = "editor.snippetFinalTabstopHighlightBackground")]
    pub editor_snippet_final_tabstop_highlight_background: Option<String>,
    #[serde(rename = "editor.snippetFinalTabstopHighlightBorder")]
    pub editor_snippet_final_tabstop_highlight_border: Option<String>,
    #[serde(rename = "editorWhitespace.foreground")]
    pub editor_whitespace_foreground: Option<String>,
    #[serde(rename = "editorIndentGuide.background")]
    pub editor_indent_guide_background: Option<String>,
    #[serde(rename = "editorIndentGuide.activeBackground")]
    pub editor_indent_guide_active_background: Option<String>,
    #[serde(rename = "editorRuler.foreground")]
    pub editor_ruler_foreground: Option<String>,
    #[serde(rename = "editorCodeLens.foreground")]
    pub editor_code_lens_foreground: Option<String>,
    #[serde(rename = "editorBracketHighlight.foreground1")]
    pub editor_bracket_highlight_foreground1: Option<String>,
    #[serde(rename = "editorBracketHighlight.foreground2")]
    pub editor_bracket_highlight_foreground2: Option<String>,
    #[serde(rename = "editorBracketHighlight.foreground3")]
    pub editor_bracket_highlight_foreground3: Option<String>,
    #[serde(rename = "editorBracketHighlight.foreground4")]
    pub editor_bracket_highlight_foreground4: Option<String>,
    #[serde(rename = "editorBracketHighlight.foreground5")]
    pub editor_bracket_highlight_foreground5: Option<String>,
    #[serde(rename = "editorBracketHighlight.foreground6")]
    pub editor_bracket_highlight_foreground6: Option<String>,
    #[serde(rename = "editorBracketHighlight.unexpectedBracket.foreground")]
    pub editor_bracket_highlight_unexpected_bracket_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.border")]
    pub editor_overview_ruler_border: Option<String>,
    #[serde(rename = "editorOverviewRuler.selectionHighlightForeground")]
    pub editor_overview_ruler_selection_highlight_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.wordHighlightForeground")]
    pub editor_overview_ruler_word_highlight_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.wordHighlightStrongForeground")]
    pub editor_overview_ruler_word_highlight_strong_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.modifiedForeground")]
    pub editor_overview_ruler_modified_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.addedForeground")]
    pub editor_overview_ruler_added_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.deletedForeground")]
    pub editor_overview_ruler_deleted_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.errorForeground")]
    pub editor_overview_ruler_error_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.warningForeground")]
    pub editor_overview_ruler_warning_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.infoForeground")]
    pub editor_overview_ruler_info_foreground: Option<String>,
    #[serde(rename = "editorError.foreground")]
    pub editor_error_foreground: Option<String>,
    #[serde(rename = "editorWarning.foreground")]
    pub editor_warning_foreground: Option<String>,
    #[serde(rename = "editorGutter.modifiedBackground")]
    pub editor_gutter_modified_background: Option<String>,
    #[serde(rename = "editorGutter.addedBackground")]
    pub editor_gutter_added_background: Option<String>,
    #[serde(rename = "editorGutter.deletedBackground")]
    pub editor_gutter_deleted_background: Option<String>,
    #[serde(rename = "gitDecoration.modifiedResourceForeground")]
    pub git_decoration_modified_resource_foreground: Option<String>,
    #[serde(rename = "gitDecoration.deletedResourceForeground")]
    pub git_decoration_deleted_resource_foreground: Option<String>,
    #[serde(rename = "gitDecoration.untrackedResourceForeground")]
    pub git_decoration_untracked_resource_foreground: Option<String>,
    #[serde(rename = "gitDecoration.ignoredResourceForeground")]
    pub git_decoration_ignored_resource_foreground: Option<String>,
    #[serde(rename = "gitDecoration.conflictingResourceForeground")]
    pub git_decoration_conflicting_resource_foreground: Option<String>,
    #[serde(rename = "diffEditor.insertedTextBackground")]
    pub diff_editor_inserted_text_background: Option<String>,
    #[serde(rename = "diffEditor.removedTextBackground")]
    pub diff_editor_removed_text_background: Option<String>,
    #[serde(rename = "inlineChat.regionHighlight")]
    pub inline_chat_region_highlight: Option<String>,
    #[serde(rename = "editorWidget.background")]
    pub editor_widget_background: Option<String>,
    #[serde(rename = "editorSuggestWidget.background")]
    pub editor_suggest_widget_background: Option<String>,
    #[serde(rename = "editorSuggestWidget.foreground")]
    pub editor_suggest_widget_foreground: Option<String>,
    #[serde(rename = "editorSuggestWidget.selectedBackground")]
    pub editor_suggest_widget_selected_background: Option<String>,
    #[serde(rename = "editorHoverWidget.background")]
    pub editor_hover_widget_background: Option<String>,
    #[serde(rename = "editorHoverWidget.border")]
    pub editor_hover_widget_border: Option<String>,
    #[serde(rename = "editorMarkerNavigation.background")]
    pub editor_marker_navigation_background: Option<String>,
    #[serde(rename = "peekView.border")]
    pub peek_view_border: Option<String>,
    #[serde(rename = "peekViewEditor.background")]
    pub peek_view_editor_background: Option<String>,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    pub peek_view_editor_match_highlight_background: Option<String>,
    #[serde(rename = "peekViewResult.background")]
    pub peek_view_result_background: Option<String>,
    #[serde(rename = "peekViewResult.fileForeground")]
    pub peek_view_result_file_foreground: Option<String>,
    #[serde(rename = "peekViewResult.lineForeground")]
    pub peek_view_result_line_foreground: Option<String>,
    #[serde(rename = "peekViewResult.matchHighlightBackground")]
    pub peek_view_result_match_highlight_background: Option<String>,
    #[serde(rename = "peekViewResult.selectionBackground")]
    pub peek_view_result_selection_background: Option<String>,
    #[serde(rename = "peekViewResult.selectionForeground")]
    pub peek_view_result_selection_foreground: Option<String>,
    #[serde(rename = "peekViewTitle.background")]
    pub peek_view_title_background: Option<String>,
    #[serde(rename = "peekViewTitleDescription.foreground")]
    pub peek_view_title_description_foreground: Option<String>,
    #[serde(rename = "peekViewTitleLabel.foreground")]
    pub peek_view_title_label_foreground: Option<String>,
    #[serde(rename = "merge.currentHeaderBackground")]
    pub merge_current_header_background: Option<String>,
    #[serde(rename = "merge.incomingHeaderBackground")]
    pub merge_incoming_header_background: Option<String>,
    #[serde(rename = "editorOverviewRuler.currentContentForeground")]
    pub editor_overview_ruler_current_content_foreground: Option<String>,
    #[serde(rename = "editorOverviewRuler.incomingContentForeground")]
    pub editor_overview_ruler_incoming_content_foreground: Option<String>,
    #[serde(rename = "panel.background")]
    pub panel_background: Option<String>,
    #[serde(rename = "panel.border")]
    pub panel_border: Option<String>,
    #[serde(rename = "panelTitle.activeBorder")]
    pub panel_title_active_border: Option<String>,
    #[serde(rename = "panelTitle.activeForeground")]
    pub panel_title_active_foreground: Option<String>,
    #[serde(rename = "panelTitle.inactiveForeground")]
    pub panel_title_inactive_foreground: Option<String>,
    #[serde(rename = "statusBar.background")]
    pub status_bar_background: Option<String>,
    #[serde(rename = "statusBar.foreground")]
    pub status_bar_foreground: Option<String>,
    #[serde(rename = "statusBar.debuggingBackground")]
    pub status_bar_debugging_background: Option<String>,
    #[serde(rename = "statusBar.debuggingForeground")]
    pub status_bar_debugging_foreground: Option<String>,
    #[serde(rename = "statusBar.noFolderBackground")]
    pub status_bar_no_folder_background: Option<String>,
    #[serde(rename = "statusBar.noFolderForeground")]
    pub status_bar_no_folder_foreground: Option<String>,
    #[serde(rename = "statusBarItem.prominentBackground")]
    pub status_bar_item_prominent_background: Option<String>,
    #[serde(rename = "statusBarItem.prominentHoverBackground")]
    pub status_bar_item_prominent_hover_background: Option<String>,
    #[serde(rename = "statusBarItem.remoteForeground")]
    pub status_bar_item_remote_foreground: Option<String>,
    #[serde(rename = "statusBarItem.remoteBackground")]
    pub status_bar_item_remote_background: Option<String>,
    #[serde(rename = "titleBar.activeBackground")]
    pub title_bar_active_background: Option<String>,
    #[serde(rename = "titleBar.activeForeground")]
    pub title_bar_active_foreground: Option<String>,
    #[serde(rename = "titleBar.inactiveBackground")]
    pub title_bar_inactive_background: Option<String>,
    #[serde(rename = "titleBar.inactiveForeground")]
    pub title_bar_inactive_foreground: Option<String>,
    #[serde(rename = "extensionButton.prominentForeground")]
    pub extension_button_prominent_foreground: Option<String>,
    #[serde(rename = "extensionButton.prominentBackground")]
    pub extension_button_prominent_background: Option<String>,
    #[serde(rename = "extensionButton.prominentHoverBackground")]
    pub extension_button_prominent_hover_background: Option<String>,
    #[serde(rename = "pickerGroup.border")]
    pub picker_group_border: Option<String>,
    #[serde(rename = "pickerGroup.foreground")]
    pub picker_group_foreground: Option<String>,
    #[serde(rename = "debugToolBar.background")]
    pub debug_tool_bar_background: Option<String>,
    #[serde(rename = "walkThrough.embeddedEditorBackground")]
    pub walk_through_embedded_editor_background: Option<String>,
    #[serde(rename = "settings.headerForeground")]
    pub settings_header_foreground: Option<String>,
    #[serde(rename = "settings.modifiedItemIndicator")]
    pub settings_modified_item_indicator: Option<String>,
    #[serde(rename = "settings.dropdownBackground")]
    pub settings_dropdown_background: Option<String>,
    #[serde(rename = "settings.dropdownForeground")]
    pub settings_dropdown_foreground: Option<String>,
    #[serde(rename = "settings.dropdownBorder")]
    pub settings_dropdown_border: Option<String>,
    #[serde(rename = "settings.checkboxBackground")]
    pub settings_checkbox_background: Option<String>,
    #[serde(rename = "settings.checkboxForeground")]
    pub settings_checkbox_foreground: Option<String>,
    #[serde(rename = "settings.checkboxBorder")]
    pub settings_checkbox_border: Option<String>,
    #[serde(rename = "settings.textInputBackground")]
    pub settings_text_input_background: Option<String>,
    #[serde(rename = "settings.textInputForeground")]
    pub settings_text_input_foreground: Option<String>,
    #[serde(rename = "settings.textInputBorder")]
    pub settings_text_input_border: Option<String>,
    #[serde(rename = "settings.numberInputBackground")]
    pub settings_number_input_background: Option<String>,
    #[serde(rename = "settings.numberInputForeground")]
    pub settings_number_input_foreground: Option<String>,
    #[serde(rename = "settings.numberInputBorder")]
    pub settings_number_input_border: Option<String>,
    #[serde(rename = "breadcrumb.foreground")]
    pub breadcrumb_foreground: Option<String>,
    #[serde(rename = "breadcrumb.background")]
    pub breadcrumb_background: Option<String>,
    #[serde(rename = "breadcrumb.focusForeground")]
    pub breadcrumb_focus_foreground: Option<String>,
    #[serde(rename = "breadcrumb.activeSelectionForeground")]
    pub breadcrumb_active_selection_foreground: Option<String>,
    #[serde(rename = "breadcrumbPicker.background")]
    pub breadcrumb_picker_background: Option<String>,
    #[serde(rename = "listFilterWidget.background")]
    pub list_filter_widget_background: Option<String>,
    #[serde(rename = "listFilterWidget.outline")]
    pub list_filter_widget_outline: Option<String>,
    #[serde(rename = "listFilterWidget.noMatchesOutline")]
    pub list_filter_widget_no_matches_outline: Option<String>,
}

fn try_parse_color(color: &str) -> Result<Hsla> {
    Ok(Rgba::try_from(color)?.into())
}

pub struct VsCodeThemeConverter {
    theme: VsCodeTheme,
    theme_metadata: ThemeMetadata,
}

impl VsCodeThemeConverter {
    pub fn new(theme: VsCodeTheme, theme_metadata: ThemeMetadata) -> Self {
        Self {
            theme,
            theme_metadata,
        }
    }

    pub fn convert(self) -> Result<UserTheme> {
        let appearance = self.theme_metadata.appearance.into();

        let vscode_colors = &self.theme.colors;

        let theme_colors_refinements = ThemeColorsRefinement {
            border: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_variant: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_focused: vscode_colors
                .focus_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_disabled: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_selected: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_transparent: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            elevated_surface_background: vscode_colors
                .panel_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            surface_background: vscode_colors
                .panel_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_background: vscode_colors
                .button_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_hover: vscode_colors
                .list_hover_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_selected: vscode_colors
                .list_active_selection_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ghost_element_hover: vscode_colors
                .list_hover_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            drop_target_background: vscode_colors
                .list_drop_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            text: vscode_colors
                .foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            tab_active_background: vscode_colors
                .tab_active_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            tab_inactive_background: vscode_colors
                .tab_inactive_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_gutter_background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_line_number: vscode_colors
                .editor_line_number_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_active_line_number: vscode_colors
                .editor_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_background: vscode_colors
                .terminal_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_black: vscode_colors
                .terminal_ansi_bright_black
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_red: vscode_colors
                .terminal_ansi_bright_red
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_green: vscode_colors
                .terminal_ansi_bright_green
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_yellow: vscode_colors
                .terminal_ansi_bright_yellow
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_blue: vscode_colors
                .terminal_ansi_bright_blue
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_magenta: vscode_colors
                .terminal_ansi_bright_magenta
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_cyan: vscode_colors
                .terminal_ansi_bright_cyan
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_white: vscode_colors
                .terminal_ansi_bright_white
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_black: vscode_colors
                .terminal_ansi_black
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_red: vscode_colors
                .terminal_ansi_red
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_green: vscode_colors
                .terminal_ansi_green
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_yellow: vscode_colors
                .terminal_ansi_yellow
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_blue: vscode_colors
                .terminal_ansi_blue
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_magenta: vscode_colors
                .terminal_ansi_magenta
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_cyan: vscode_colors
                .terminal_ansi_cyan
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_white: vscode_colors
                .terminal_ansi_white
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ..Default::default()
        };

        Ok(UserTheme {
            name: self.theme_metadata.name.into(),
            appearance,
            styles: UserThemeStylesRefinement {
                colors: theme_colors_refinements,
            },
        })
    }
}
