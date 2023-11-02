use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Theme {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub name: String,
    pub author: String,
    pub maintainers: Vec<String>,
    pub semantic_class: String,
    pub semantic_highlighting: bool,
    pub colors: VSCodeColors,
}

#[derive(Debug, Deserialize)]
pub struct VSCodeColors {
    #[serde(rename = "vsc_terminal_background")]
    terminal_background: String,
    #[serde(rename = "vsc_terminal_foreground")]
    terminal_foreground: String,
    #[serde(rename = "vsc_terminal_ansi_bright_black")]
    terminal_ansi_bright_black: String,
    #[serde(rename = "vsc_terminal_ansi_bright_red")]
    terminal_ansi_bright_red: String,
    #[serde(rename = "vsc_terminal_ansi_bright_green")]
    terminal_ansi_bright_green: String,
    #[serde(rename = "vsc_terminal_ansi_bright_yellow")]
    terminal_ansi_bright_yellow: String,
    #[serde(rename = "vsc_terminal_ansi_bright_blue")]
    terminal_ansi_bright_blue: String,
    #[serde(rename = "vsc_terminal_ansi_bright_magenta")]
    terminal_ansi_bright_magenta: String,
    #[serde(rename = "vsc_terminal_ansi_bright_cyan")]
    terminal_ansi_bright_cyan: String,
    #[serde(rename = "vsc_terminal_ansi_bright_white")]
    terminal_ansi_bright_white: String,
    #[serde(rename = "vsc_terminal_ansi_black")]
    terminal_ansi_black: String,
    #[serde(rename = "vsc_terminal_ansi_red")]
    terminal_ansi_red: String,
    #[serde(rename = "vsc_terminal_ansi_green")]
    terminal_ansi_green: String,
    #[serde(rename = "vsc_terminal_ansi_yellow")]
    terminal_ansi_yellow: String,
    #[serde(rename = "vsc_terminal_ansi_blue")]
    terminal_ansi_blue: String,
    #[serde(rename = "vsc_terminal_ansi_magenta")]
    terminal_ansi_magenta: String,
    #[serde(rename = "vsc_terminal_ansi_cyan")]
    terminal_ansi_cyan: String,
    #[serde(rename = "vsc_terminal_ansi_white")]
    terminal_ansi_white: String,
    #[serde(rename = "vsc_focus_border")]
    focus_border: String,
    #[serde(rename = "vsc_foreground")]
    foreground: String,
    #[serde(rename = "vsc_selection_background")]
    selection_background: String,
    #[serde(rename = "vsc_error_foreground")]
    error_foreground: String,
    #[serde(rename = "vsc_button_background")]
    button_background: String,
    #[serde(rename = "vsc_button_foreground")]
    button_foreground: String,
    #[serde(rename = "vsc_button_secondary_background")]
    button_secondary_background: String,
    #[serde(rename = "vsc_button_secondary_foreground")]
    button_secondary_foreground: String,
    #[serde(rename = "vsc_button_secondary_hover_background")]
    button_secondary_hover_background: String,
    #[serde(rename = "vsc_dropdown_background")]
    dropdown_background: String,
    #[serde(rename = "vsc_dropdown_border")]
    dropdown_border: String,
    #[serde(rename = "vsc_dropdown_foreground")]
    dropdown_foreground: String,
    #[serde(rename = "vsc_input_background")]
    input_background: String,
    #[serde(rename = "vsc_input_foreground")]
    input_foreground: String,
    #[serde(rename = "vsc_input_border")]
    input_border: String,
    #[serde(rename = "vsc_input_placeholder_foreground")]
    input_placeholder_foreground: String,
    #[serde(rename = "vsc_input_option_active_border")]
    input_option_active_border: String,
    #[serde(rename = "vsc_input_validation_info_border")]
    input_validation_info_border: String,
    #[serde(rename = "vsc_input_validation_warning_border")]
    input_validation_warning_border: String,
    #[serde(rename = "vsc_input_validation_error_border")]
    input_validation_error_border: String,
    #[serde(rename = "vsc_badge_foreground")]
    badge_foreground: String,
    #[serde(rename = "vsc_badge_background")]
    badge_background: String,
    #[serde(rename = "vsc_progress_bar_background")]
    progress_bar_background: String,
    #[serde(rename = "vsc_list_active_selection_background")]
    list_active_selection_background: String,
    #[serde(rename = "vsc_list_active_selection_foreground")]
    list_active_selection_foreground: String,
    #[serde(rename = "vsc_list_drop_background")]
    list_drop_background: String,
    #[serde(rename = "vsc_list_focus_background")]
    list_focus_background: String,
    #[serde(rename = "vsc_list_highlight_foreground")]
    list_highlight_foreground: String,
    #[serde(rename = "vsc_list_hover_background")]
    list_hover_background: String,
    #[serde(rename = "vsc_list_inactive_selection_background")]
    list_inactive_selection_background: String,
    #[serde(rename = "vsc_list_warning_foreground")]
    list_warning_foreground: String,
    #[serde(rename = "vsc_list_error_foreground")]
    list_error_foreground: String,
    #[serde(rename = "vsc_activity_bar_background")]
    activity_bar_background: String,
    #[serde(rename = "vsc_activity_bar_inactive_foreground")]
    activity_bar_inactive_foreground: String,
    #[serde(rename = "vsc_activity_bar_foreground")]
    activity_bar_foreground: String,
    #[serde(rename = "vsc_activity_bar_active_border")]
    activity_bar_active_border: String,
    #[serde(rename = "vsc_activity_bar_active_background")]
    activity_bar_active_background: String,
    #[serde(rename = "vsc_activity_bar_badge_background")]
    activity_bar_badge_background: String,
    #[serde(rename = "vsc_activity_bar_badge_foreground")]
    activity_bar_badge_foreground: String,
    #[serde(rename = "vsc_side_bar_background")]
    side_bar_background: String,
    #[serde(rename = "vsc_side_bar_title_foreground")]
    side_bar_title_foreground: String,
    #[serde(rename = "vsc_side_bar_section_header_background")]
    side_bar_section_header_background: String,
    #[serde(rename = "vsc_side_bar_section_header_border")]
    side_bar_section_header_border: String,
    #[serde(rename = "vsc_editor_group_border")]
    editor_group_border: String,
    #[serde(rename = "vsc_editor_group_drop_background")]
    editor_group_drop_background: String,
    #[serde(rename = "vsc_editor_group_header_tabs_background")]
    editor_group_header_tabs_background: String,
    #[serde(rename = "vsc_tab_active_background")]
    tab_active_background: String,
    #[serde(rename = "vsc_tab_active_foreground")]
    tab_active_foreground: String,
    #[serde(rename = "vsc_tab_border")]
    tab_border: String,
    #[serde(rename = "vsc_tab_active_border_top")]
    tab_active_border_top: String,
    #[serde(rename = "vsc_tab_inactive_background")]
    tab_inactive_background: String,
    #[serde(rename = "vsc_tab_inactive_foreground")]
    tab_inactive_foreground: String,
    #[serde(rename = "vsc_editor_foreground")]
    editor_foreground: String,
    #[serde(rename = "vsc_editor_background")]
    editor_background: String,
    #[serde(rename = "vsc_editor_line_number_foreground")]
    editor_line_number_foreground: String,
    #[serde(rename = "vsc_editor_selection_background")]
    editor_selection_background: String,
    #[serde(rename = "vsc_editor_selection_highlight_background")]
    editor_selection_highlight_background: String,
    #[serde(rename = "vsc_editor_fold_background")]
    editor_fold_background: String,
    #[serde(rename = "vsc_editor_word_highlight_background")]
    editor_word_highlight_background: String,
    #[serde(rename = "vsc_editor_word_highlight_strong_background")]
    editor_word_highlight_strong_background: String,
    #[serde(rename = "vsc_editor_find_match_background")]
    editor_find_match_background: String,
    #[serde(rename = "vsc_editor_find_match_highlight_background")]
    editor_find_match_highlight_background: String,
    #[serde(rename = "vsc_editor_find_range_highlight_background")]
    editor_find_range_highlight_background: String,
    #[serde(rename = "vsc_editor_hover_highlight_background")]
    editor_hover_highlight_background: String,
    #[serde(rename = "vsc_editor_line_highlight_border")]
    editor_line_highlight_border: String,
    #[serde(rename = "vsc_editor_link_active_foreground")]
    editor_link_active_foreground: String,
    #[serde(rename = "vsc_editor_range_highlight_background")]
    editor_range_highlight_background: String,
    #[serde(rename = "vsc_editor_snippet_tabstop_highlight_background")]
    editor_snippet_tabstop_highlight_background: String,
    #[serde(rename = "vsc_editor_snippet_tabstop_highlight_border")]
    editor_snippet_tabstop_highlight_border: String,
    #[serde(rename = "vsc_editor_snippet_final_tabstop_highlight_background")]
    editor_snippet_final_tabstop_highlight_background: String,
    #[serde(rename = "vsc_editor_snippet_final_tabstop_highlight_border")]
    editor_snippet_final_tabstop_highlight_border: String,
    #[serde(rename = "vsc_editor_whitespace_foreground")]
    editor_whitespace_foreground: String,
    #[serde(rename = "vsc_editor_indent_guide_background")]
    editor_indent_guide_background: String,
    #[serde(rename = "vsc_editor_indent_guide_active_background")]
    editor_indent_guide_active_background: String,
    #[serde(rename = "vsc_editor_ruler_foreground")]
    editor_ruler_foreground: String,
    #[serde(rename = "vsc_editor_code_lens_foreground")]
    editor_code_lens_foreground: String,
    #[serde(rename = "vsc_editor_bracket_highlight_foreground1")]
    editor_bracket_highlight_foreground1: String,
    #[serde(rename = "vsc_editor_bracket_highlight_foreground2")]
    editor_bracket_highlight_foreground2: String,
    #[serde(rename = "vsc_editor_bracket_highlight_foreground3")]
    editor_bracket_highlight_foreground3: String,
    #[serde(rename = "vsc_editor_bracket_highlight_foreground4")]
    editor_bracket_highlight_foreground4: String,
    #[serde(rename = "vsc_editor_bracket_highlight_foreground5")]
    editor_bracket_highlight_foreground5: String,
    #[serde(rename = "vsc_editor_bracket_highlight_foreground6")]
    editor_bracket_highlight_foreground6: String,
    #[serde(rename = "vsc_editor_bracket_highlight_unexpected_bracket_foreground")]
    editor_bracket_highlight_unexpected_bracket_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_border")]
    editor_overview_ruler_border: String,
    #[serde(rename = "vsc_editor_overview_ruler_selection_highlight_foreground")]
    editor_overview_ruler_selection_highlight_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_word_highlight_foreground")]
    editor_overview_ruler_word_highlight_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_word_highlight_strong_foreground")]
    editor_overview_ruler_word_highlight_strong_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_modified_foreground")]
    editor_overview_ruler_modified_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_added_foreground")]
    editor_overview_ruler_added_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_deleted_foreground")]
    editor_overview_ruler_deleted_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_error_foreground")]
    editor_overview_ruler_error_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_warning_foreground")]
    editor_overview_ruler_warning_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_info_foreground")]
    editor_overview_ruler_info_foreground: String,
    #[serde(rename = "vsc_editor_error_foreground")]
    editor_error_foreground: String,
    #[serde(rename = "vsc_editor_warning_foreground")]
    editor_warning_foreground: String,
    #[serde(rename = "vsc_editor_gutter_modified_background")]
    editor_gutter_modified_background: String,
    #[serde(rename = "vsc_editor_gutter_added_background")]
    editor_gutter_added_background: String,
    #[serde(rename = "vsc_editor_gutter_deleted_background")]
    editor_gutter_deleted_background: String,
    #[serde(rename = "vsc_git_decoration_modified_resource_foreground")]
    git_decoration_modified_resource_foreground: String,
    #[serde(rename = "vsc_git_decoration_deleted_resource_foreground")]
    git_decoration_deleted_resource_foreground: String,
    #[serde(rename = "vsc_git_decoration_untracked_resource_foreground")]
    git_decoration_untracked_resource_foreground: String,
    #[serde(rename = "vsc_git_decoration_ignored_resource_foreground")]
    git_decoration_ignored_resource_foreground: String,
    #[serde(rename = "vsc_git_decoration_conflicting_resource_foreground")]
    git_decoration_conflicting_resource_foreground: String,
    #[serde(rename = "vsc_diff_editor_inserted_text_background")]
    diff_editor_inserted_text_background: String,
    #[serde(rename = "vsc_diff_editor_removed_text_background")]
    diff_editor_removed_text_background: String,
    #[serde(rename = "vsc_inline_chat_region_highlight")]
    inline_chat_region_highlight: String,
    #[serde(rename = "vsc_editor_widget_background")]
    editor_widget_background: String,
    #[serde(rename = "vsc_editor_suggest_widget_background")]
    editor_suggest_widget_background: String,
    #[serde(rename = "vsc_editor_suggest_widget_foreground")]
    editor_suggest_widget_foreground: String,
    #[serde(rename = "vsc_editor_suggest_widget_selected_background")]
    editor_suggest_widget_selected_background: String,
    #[serde(rename = "vsc_editor_hover_widget_background")]
    editor_hover_widget_background: String,
    #[serde(rename = "vsc_editor_hover_widget_border")]
    editor_hover_widget_border: String,
    #[serde(rename = "vsc_editor_marker_navigation_background")]
    editor_marker_navigation_background: String,
    #[serde(rename = "vsc_peek_view_border")]
    peek_view_border: String,
    #[serde(rename = "vsc_peek_view_editor_background")]
    peek_view_editor_background: String,
    #[serde(rename = "vsc_peek_view_editor_match_highlight_background")]
    peek_view_editor_match_highlight_background: String,
    #[serde(rename = "vsc_peek_view_result_background")]
    peek_view_result_background: String,
    #[serde(rename = "vsc_peek_view_result_file_foreground")]
    peek_view_result_file_foreground: String,
    #[serde(rename = "vsc_peek_view_result_line_foreground")]
    peek_view_result_line_foreground: String,
    #[serde(rename = "vsc_peek_view_result_match_highlight_background")]
    peek_view_result_match_highlight_background: String,
    #[serde(rename = "vsc_peek_view_result_selection_background")]
    peek_view_result_selection_background: String,
    #[serde(rename = "vsc_peek_view_result_selection_foreground")]
    peek_view_result_selection_foreground: String,
    #[serde(rename = "vsc_peek_view_title_background")]
    peek_view_title_background: String,
    #[serde(rename = "vsc_peek_view_title_description_foreground")]
    peek_view_title_description_foreground: String,
    #[serde(rename = "vsc_peek_view_title_label_foreground")]
    peek_view_title_label_foreground: String,
    #[serde(rename = "vsc_merge_current_header_background")]
    merge_current_header_background: String,
    #[serde(rename = "vsc_merge_incoming_header_background")]
    merge_incoming_header_background: String,
    #[serde(rename = "vsc_editor_overview_ruler_current_content_foreground")]
    editor_overview_ruler_current_content_foreground: String,
    #[serde(rename = "vsc_editor_overview_ruler_incoming_content_foreground")]
    editor_overview_ruler_incoming_content_foreground: String,
    #[serde(rename = "vsc_panel_background")]
    panel_background: String,
    #[serde(rename = "vsc_panel_border")]
    panel_border: String,
    #[serde(rename = "vsc_panel_title_active_border")]
    panel_title_active_border: String,
    #[serde(rename = "vsc_panel_title_active_foreground")]
    panel_title_active_foreground: String,
    #[serde(rename = "vsc_panel_title_inactive_foreground")]
    panel_title_inactive_foreground: String,
    #[serde(rename = "vsc_status_bar_background")]
    status_bar_background: String,
    #[serde(rename = "vsc_status_bar_foreground")]
    status_bar_foreground: String,
    #[serde(rename = "vsc_status_bar_debugging_background")]
    status_bar_debugging_background: String,
    #[serde(rename = "vsc_status_bar_debugging_foreground")]
    status_bar_debugging_foreground: String,
    #[serde(rename = "vsc_status_bar_no_folder_background")]
    status_bar_no_folder_background: String,
    #[serde(rename = "vsc_status_bar_no_folder_foreground")]
    status_bar_no_folder_foreground: String,
    #[serde(rename = "vsc_status_bar_item_prominent_background")]
    status_bar_item_prominent_background: String,
    #[serde(rename = "vsc_status_bar_item_prominent_hover_background")]
    status_bar_item_prominent_hover_background: String,
    #[serde(rename = "vsc_status_bar_item_remote_foreground")]
    status_bar_item_remote_foreground: String,
    #[serde(rename = "vsc_status_bar_item_remote_background")]
    status_bar_item_remote_background: String,
    #[serde(rename = "vsc_title_bar_active_background")]
    title_bar_active_background: String,
    #[serde(rename = "vsc_title_bar_active_foreground")]
    title_bar_active_foreground: String,
    #[serde(rename = "vsc_title_bar_inactive_background")]
    title_bar_inactive_background: String,
    #[serde(rename = "vsc_title_bar_inactive_foreground")]
    title_bar_inactive_foreground: String,
    #[serde(rename = "vsc_extension_button_prominent_foreground")]
    extension_button_prominent_foreground: String,
    #[serde(rename = "vsc_extension_button_prominent_background")]
    extension_button_prominent_background: String,
    #[serde(rename = "vsc_extension_button_prominent_hover_background")]
    extension_button_prominent_hover_background: String,
    #[serde(rename = "vsc_picker_group_border")]
    picker_group_border: String,
    #[serde(rename = "vsc_picker_group_foreground")]
    picker_group_foreground: String,
    #[serde(rename = "vsc_debug_tool_bar_background")]
    debug_tool_bar_background: String,
    #[serde(rename = "vsc_walk_through_embedded_editor_background")]
    walk_through_embedded_editor_background: String,
    #[serde(rename = "vsc_settings_header_foreground")]
    settings_header_foreground: String,
    #[serde(rename = "vsc_settings_modified_item_indicator")]
    settings_modified_item_indicator: String,
    #[serde(rename = "vsc_settings_dropdown_background")]
    settings_dropdown_background: String,
    #[serde(rename = "vsc_settings_dropdown_foreground")]
    settings_dropdown_foreground: String,
    #[serde(rename = "vsc_settings_dropdown_border")]
    settings_dropdown_border: String,
    #[serde(rename = "vsc_settings_checkbox_background")]
    settings_checkbox_background: String,
    #[serde(rename = "vsc_settings_checkbox_foreground")]
    settings_checkbox_foreground: String,
    #[serde(rename = "vsc_settings_checkbox_border")]
    settings_checkbox_border: String,
    #[serde(rename = "vsc_settings_text_input_background")]
    settings_text_input_background: String,
    #[serde(rename = "vsc_settings_text_input_foreground")]
    settings_text_input_foreground: String,
    #[serde(rename = "vsc_settings_text_input_border")]
    settings_text_input_border: String,
    #[serde(rename = "vsc_settings_number_input_background")]
    settings_number_input_background: String,
    #[serde(rename = "vsc_settings_number_input_foreground")]
    settings_number_input_foreground: String,
    #[serde(rename = "vsc_settings_number_input_border")]
    settings_number_input_border: String,
    #[serde(rename = "vsc_breadcrumb_foreground")]
    breadcrumb_foreground: String,
    #[serde(rename = "vsc_breadcrumb_background")]
    breadcrumb_background: String,
    #[serde(rename = "vsc_breadcrumb_focus_foreground")]
    breadcrumb_focus_foreground: String,
    #[serde(rename = "vsc_breadcrumb_active_selection_foreground")]
    breadcrumb_active_selection_foreground: String,
    #[serde(rename = "vsc_breadcrumb_picker_background")]
    breadcrumb_picker_background: String,
    #[serde(rename = "vsc_list_filter_widget_background")]
    list_filter_widget_background: String,
    #[serde(rename = "vsc_list_filter_widget_outline")]
    list_filter_widget_outline: String,
    #[serde(rename = "vsc_list_filter_widget_no_matches_outline")]
    list_filter_widget_no_matches_outline: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_deserialize_dracula() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root_dir = manifest_dir.parent().unwrap().parent().unwrap();

        let mut d = root_dir.to_path_buf();
        d.push("assets/themes/src/vsc/dracula/dracula.json");

        let data = std::fs::read_to_string(d).expect("Unable to read file");

        let result: Theme = serde_json::from_str(&data).unwrap();
        println!("{:#?}", result);

        // Uncomment the following lines to print specific fields
        println!("Name: {:?}", result.name);
        println!("Author: {:?}", result.author);
    }
}

// use crate::ThemeColorsRefinement;

// struct ImportedThemeFamily {
//     pub id: String,
//     pub name: String,
//     pub author: String,
//     pub url: String,
//     pub license: String,
//     pub themes: Vec<ImportedThemeVariant>,
// }

// struct ImportedThemeVariant {
//     pub id: String,
//     pub name: String,
//     pub colors: ThemeColorsRefinement,
// }

// pub fn try_vscode_colors_to_theme_colors(colors: VSCodeColors) -> ThemeColorsRefinement {
//     let mut theme_colors = ThemeColorsRefinement::default();

//     theme_colors
// }

// pub fn vscode_colors_to_theme_colors(color: String) -> ThemeColorsRefinement {
//     ThemeColorsRefinement {
//         text: Some(color),
//         ..Default::default()
//     }
// }
