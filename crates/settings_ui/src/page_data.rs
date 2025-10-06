use gpui::IntoElement;
use std::rc::Rc;

use crate::{
    SettingField, SettingItem, SettingsFieldMetadata, SettingsPage, SettingsPageItem, SubPageLink,
};

pub(crate) fn user_settings_data() -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "General Page",
            items: vec![
                SettingsPageItem::SectionHeader("General"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Confirm Quit",
                    description: "Whether to confirm before quitting Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.confirm_quit,
                        pick_mut: |settings_content| &mut settings_content.workspace.confirm_quit,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Restore On Startup",
                    description: "Whether to restore previous session when opening Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.restore_on_startup,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.restore_on_startup
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Restore File State",
                    description: "Whether to restore previous file state when reopening",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.restore_on_file_reopen,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.restore_on_file_reopen
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Close on File Delete",
                    description: "Whether to automatically close files that have been deleted",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.close_on_file_delete,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.close_on_file_delete
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "When Closing With No Tabs",
                    description: "What to do when using 'close active item' with no tabs",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.workspace.when_closing_with_no_tabs
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.when_closing_with_no_tabs
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "On Last Window Closed",
                    description: "What to do when the last window is closed",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.on_last_window_closed,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.on_last_window_closed
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Path Prompts",
                    description: "Whether to use system dialogs for Open and Save As",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.workspace.use_system_path_prompts
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.use_system_path_prompts
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Prompts",
                    description: "Whether to use system prompts for confirmations",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.use_system_prompts,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.use_system_prompts
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Scoped Settings"),
                // todo(settings_ui): Implement another setting item type that just shows an edit in settings.json
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Preview Channel",
                //     description: "Which settings should be activated only in Preview build of Zed",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.workspace.use_system_prompts,
                //         pick_mut: |settings_content| {
                //             &mut settings_content.workspace.use_system_prompts
                //         },
                //     }),
                //     metadata: None,
                // }),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Settings Profiles",
                //     description: "Any number of settings profiles that are temporarily applied on top of your existing user settings.",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.workspace.use_system_prompts,
                //         pick_mut: |settings_content| {
                //             &mut settings_content.workspace.use_system_prompts
                //         },
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SectionHeader("Privacy"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Telemetry Diagnostics",
                    description: "Send debug info like crash reports.",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(telemetry) = &settings_content.telemetry {
                                &telemetry.diagnostics
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .telemetry
                                .get_or_insert_default()
                                .diagnostics
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Telemetry Metrics",
                    description: "Send anonymized usage data like what languages you're using Zed with.",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(telemetry) = &settings_content.telemetry {
                                &telemetry.metrics
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.telemetry.get_or_insert_default().metrics
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Appearance & Behavior",
            items: vec![
                SettingsPageItem::SectionHeader("Theme"),
                // todo(settings_ui): Figure out how we want to add these
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Theme Mode",
                //     description: "How to select the theme",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.theme,
                //         pick_mut: |settings_content| &mut settings_content.theme.theme,
                //     }),
                //     metadata: None,
                // }),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Icon Theme",
                //     // todo(settings_ui)
                //     // This description is misleading because the icon theme is used in more places than the file explorer)
                //     description: "Choose the icon theme for file explorer",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.icon_theme,
                //         pick_mut: |settings_content| &mut settings_content.theme.icon_theme,
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SectionHeader("Layout"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Bottom Dock Layout",
                    description: "Layout mode for the bottom dock",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.bottom_dock_layout,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.bottom_dock_layout
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Zoomed Padding",
                    description: "Whether to show padding for zoomed panels",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.zoomed_padding,
                        pick_mut: |settings_content| &mut settings_content.workspace.zoomed_padding,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Window Tabs",
                    description: "Whether to allow windows to tab together based on the user's tabbing preference (macOS only)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.use_system_window_tabs,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.use_system_window_tabs
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Fonts"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Buffer Font Family",
                    description: "Font family for editor text",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.buffer_font_family,
                        pick_mut: |settings_content| &mut settings_content.theme.buffer_font_family,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Buffer Font Size",
                    description: "Font size for editor text",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.buffer_font_size,
                        pick_mut: |settings_content| &mut settings_content.theme.buffer_font_size,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Buffer Font Weight",
                    description: "Font weight for editor text (100-900)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.buffer_font_weight,
                        pick_mut: |settings_content| &mut settings_content.theme.buffer_font_weight,
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): This needs custom ui
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Buffer Line Height",
                //     description: "Line height for editor text",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.buffer_line_height,
                //         pick_mut: |settings_content| &mut settings_content.theme.buffer_line_height,
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "UI Font Family",
                    description: "Font family for UI elements",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.ui_font_family,
                        pick_mut: |settings_content| &mut settings_content.theme.ui_font_family,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "UI Font Size",
                    description: "Font size for UI elements",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.ui_font_size,
                        pick_mut: |settings_content| &mut settings_content.theme.ui_font_size,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "UI Font Weight",
                    description: "Font weight for UI elements (100-900)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.ui_font_weight,
                        pick_mut: |settings_content| &mut settings_content.theme.ui_font_weight,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Keymap"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Base Keymap",
                    description: "The name of a base set of key bindings to use",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.base_keymap,
                        pick_mut: |settings_content| &mut settings_content.base_keymap,
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Vim/Helix Mode should be apart of one type because it's undefined
                // behavior to have them both enabled at the same time
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Vim Mode",
                    description: "Whether to enable vim modes and key bindings",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.vim_mode,
                        pick_mut: |settings_content| &mut settings_content.vim_mode,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Helix Mode",
                    description: "Whether to enable helix modes and key bindings",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.helix_mode,
                        pick_mut: |settings_content| &mut settings_content.helix_mode,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Multi Cursor Modifier",
                    description: "Modifier key for adding multiple cursors",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.multi_cursor_modifier,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.multi_cursor_modifier
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Cursor"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Blink",
                    description: "Whether the cursor blinks in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.cursor_blink,
                        pick_mut: |settings_content| &mut settings_content.editor.cursor_blink,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Shape",
                    description: "Cursor shape for the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.cursor_shape,
                        pick_mut: |settings_content| &mut settings_content.editor.cursor_shape,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hide Mouse",
                    description: "When to hide the mouse cursor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.hide_mouse,
                        pick_mut: |settings_content| &mut settings_content.editor.hide_mouse,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Highlighting"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Unnecessary Code Fade",
                    description: "How much to fade out unused code (0.0 - 0.9)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.unnecessary_code_fade,
                        pick_mut: |settings_content| {
                            &mut settings_content.theme.unnecessary_code_fade
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Current Line Highlight",
                    description: "How to highlight the current line",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.current_line_highlight,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.current_line_highlight
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Selection Highlight",
                    description: "Whether to highlight all occurrences of selected text",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.selection_highlight,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.selection_highlight
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Rounded Selection",
                    description: "Whether the text selection should have rounded corners",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.rounded_selection,
                        pick_mut: |settings_content| &mut settings_content.editor.rounded_selection,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Guides"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Wrap Guides",
                    description: "Whether to show wrap guides (vertical rulers)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): This needs a custom component
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Wrap Guides",
                //     description: "Character counts at which to show wrap guides",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             &settings_content
                //                 .project
                //                 .all_languages
                //                 .defaults
                //                 .wrap_guides
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content
                //                 .project
                //                 .all_languages
                //                 .defaults
                //                 .wrap_guides
                //         },
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SectionHeader("Whitespace"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Whitespace",
                    description: "Whether to show tabs and spaces",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_whitespaces
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_whitespaces
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Window"),
                // todo(settings_ui): Should we filter by platform?
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Window Tabs",
                    description: "Whether to allow windows to tab together (macOS only)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.use_system_window_tabs,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.use_system_window_tabs
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Layout"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Zoomed Padding",
                    description: "Whether to show padding for zoomed panels",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.zoomed_padding,
                        pick_mut: |settings_content| &mut settings_content.workspace.zoomed_padding,
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs numeric stepper + option within an option
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Centered Layout Left Padding",
                //     description: "Left padding for centered layout",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             &settings_content.workspace.centered_layout.left_padding
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content.workspace.centered_layout.left_padding
                //         },
                //     }),
                //     metadata: None,
                // }),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Centered Layout Right Padding",
                //     description: "Right padding for centered layout",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(centered_layout) =
                //                 &settings_content.workspace.centered_layout
                //             {
                //                 &centered_layout.right_padding
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             if let Some(mut centered_layout) =
                //                 settings_content.workspace.centered_layout
                //             {
                //                 &mut centered_layout.right_padding
                //             } else {
                //                 &mut None
                //             }
                //         },
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Bottom Dock Layout",
                    description: "Layout mode of the bottom dock",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.bottom_dock_layout,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.bottom_dock_layout
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Editor",
            items: vec![
                SettingsPageItem::SectionHeader("Indentation"),
                // todo(settings_ui): Needs numeric stepper
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Tab Size",
                    description: "How many columns a tab should occupy",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.project.all_languages.defaults.tab_size
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project.all_languages.defaults.tab_size
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hard Tabs",
                    description: "Whether to indent lines using tab characters, as opposed to multiple spaces",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.project.all_languages.defaults.hard_tabs
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project.all_languages.defaults.hard_tabs
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Indent",
                    description: "Whether indentation should be adjusted based on the context whilst typing",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.project.all_languages.defaults.auto_indent
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project.all_languages.defaults.auto_indent
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Indent On Paste",
                    description: "Whether indentation of pasted content should be adjusted based on the context",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .auto_indent_on_paste
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .auto_indent_on_paste
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Wrapping"),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Preferred Line Length",
                //     description: "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.project.all_languages.defaults.preferred_line_length,
                //         pick_mut: |settings_content| &mut settings_content.project.all_languages.defaults.preferred_line_length,
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Soft Wrap",
                    description: "How to soft-wrap long lines of text",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.project.all_languages.defaults.soft_wrap
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project.all_languages.defaults.soft_wrap
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Search"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Search Wrap",
                    description: "Whether the editor search results will loop",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.search_wrap,
                        pick_mut: |settings_content| &mut settings_content.editor.search_wrap,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Seed Search Query From Cursor",
                    description: "When to populate a new search's query based on the text under the cursor",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.editor.seed_search_query_from_cursor
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.seed_search_query_from_cursor
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use Smartcase Search",
                    description: "Whether to use smartcase search",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.use_smartcase_search,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.use_smartcase_search
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Editor Behavior"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Redact Private Values",
                    description: "Hide the values of variables in private files",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.redact_private_values,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.redact_private_values
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Middle Click Paste",
                    description: "Whether to enable middle-click paste on Linux",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.middle_click_paste,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.middle_click_paste
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Double Click In Multibuffer",
                    description: "What to do when multibuffer is double clicked in some of its excerpts",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.editor.double_click_in_multibuffer
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.double_click_in_multibuffer
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Go To Definition Fallback",
                    description: "Whether to follow-up empty go to definition responses from the language server",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.go_to_definition_fallback,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.go_to_definition_fallback
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Scrolling"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Scroll Beyond Last Line",
                    description: "Whether the editor will scroll beyond the last line",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.scroll_beyond_last_line,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.scroll_beyond_last_line
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Vertical Scroll Margin",
                    description: "The number of lines to keep above/below the cursor when auto-scrolling",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.vertical_scroll_margin,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.vertical_scroll_margin
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Horizontal Scroll Margin",
                    description: "The number of characters to keep on either side when scrolling with the mouse",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.horizontal_scroll_margin,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.horizontal_scroll_margin
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Scroll Sensitivity",
                    description: "Scroll sensitivity multiplier for both horizontal and vertical scrolling",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.scroll_sensitivity,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.scroll_sensitivity
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Fast Scroll Sensitivity",
                    description: "Fast Scroll sensitivity multiplier for both horizontal and vertical scrolling",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.fast_scroll_sensitivity,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.fast_scroll_sensitivity
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Autoscroll On Clicks",
                    description: "Whether to scroll when clicking near the edge of the visible text area",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.autoscroll_on_clicks,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.autoscroll_on_clicks
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Auto Actions"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use Autoclose",
                    description: "Whether to automatically type closing characters for you",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_autoclose
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_autoclose
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use Auto Surround",
                    description: "Whether to automatically surround text with characters for you",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_auto_surround
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_auto_surround
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use On Type Format",
                    description: "Whether to use additional LSP queries to format the code after every trigger symbol input",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_on_type_format
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_on_type_format
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Always Treat Brackets As Autoclosed",
                    description: "Controls how the editor handles the autoclosed characters",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .always_treat_brackets_as_autoclosed
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .always_treat_brackets_as_autoclosed
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Formatting"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Remove Trailing Whitespace On Save",
                    description: "Whether or not to remove any trailing whitespace from lines of a buffer before saving it",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .remove_trailing_whitespace_on_save
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .remove_trailing_whitespace_on_save
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Ensure Final Newline On Save",
                    description: "Whether or not to ensure there's a single newline at the end of a buffer when saving it",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .ensure_final_newline_on_save
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .ensure_final_newline_on_save
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Extend Comment On Newline",
                    description: "Whether to start a new line with a comment when a previous line is a comment as well",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .extend_comment_on_newline
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .extend_comment_on_newline
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Completions"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Completions On Input",
                    description: "Whether to pop the completions menu while typing in an editor without explicitly requesting it",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completions_on_input
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completions_on_input
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Completion Documentation",
                    description: "Whether to display inline and alongside documentation for items in the completions menu",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completion_documentation
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completion_documentation
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Signature Help",
                    description: "Whether to automatically show a signature help pop-up or not",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.auto_signature_help,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.auto_signature_help
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Signature Help After Edits",
                    description: "Whether to show the signature help pop-up after completions or bracket pairs inserted",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.editor.show_signature_help_after_edits
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.show_signature_help_after_edits
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Snippet Sort Order",
                    description: "Determines how snippets are sorted relative to other completion items",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.snippet_sort_order,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.snippet_sort_order
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Hover"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hover Popover Enabled",
                    description: "Whether to show the informational hover box when moving the mouse over symbols in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.hover_popover_enabled,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.hover_popover_enabled
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings ui): add units to this numeric stepper
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hover Popover Delay",
                    description: "Time to wait in milliseconds before showing the informational hover box",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.hover_popover_delay,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.hover_popover_delay
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Code Actions"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Code Actions",
                    description: "Whether to show code action button at start of buffer line",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.inline_code_actions,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.inline_code_actions
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Selection"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Drag And Drop Selection",
                    description: "Whether to enable drag and drop selection",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(drag_and_drop) =
                                &settings_content.editor.drag_and_drop_selection
                            {
                                &drag_and_drop.enabled
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .drag_and_drop_selection
                                .get_or_insert_default()
                                .enabled
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Drag And Drop Selection Delay",
                //     description: "Delay in milliseconds before drag and drop selection starts",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(drag_and_drop) = &settings_content.editor.drag_and_drop_selection {
                //                 &drag_and_drop.delay
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content.editor.drag_and_drop_selection.get_or_insert_default().delay
                //         },
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SectionHeader("Line Numbers"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Relative Line Numbers",
                    description: "Whether the line numbers on editors gutter are relative or not",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.relative_line_numbers,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.relative_line_numbers
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Gutter"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Line Numbers",
                    description: "Whether to show line numbers in the gutter",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(gutter) = &settings_content.editor.gutter {
                                &gutter.line_numbers
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .gutter
                                .get_or_insert_default()
                                .line_numbers
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Runnables",
                    description: "Whether to show runnable buttons in the gutter",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(gutter) = &settings_content.editor.gutter {
                                &gutter.runnables
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .gutter
                                .get_or_insert_default()
                                .runnables
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Breakpoints",
                    description: "Whether to show breakpoints in the gutter",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(gutter) = &settings_content.editor.gutter {
                                &gutter.breakpoints
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .gutter
                                .get_or_insert_default()
                                .breakpoints
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Folds",
                    description: "Whether to show code folding controls in the gutter",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(gutter) = &settings_content.editor.gutter {
                                &gutter.folds
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.gutter.get_or_insert_default().folds
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Tabs"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Tab Bar",
                    description: "Whether or not to show the tab bar in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tab_bar) = &settings_content.tab_bar {
                                &tab_bar.show
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.tab_bar.get_or_insert_default().show
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Git Status In Tabs",
                    description: "Whether to show the Git file status on a tab item",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tabs) = &settings_content.tabs {
                                &tabs.git_status
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.tabs.get_or_insert_default().git_status
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show File Icons In Tabs",
                    description: "Whether to show the file icon for a tab",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tabs) = &settings_content.tabs {
                                &tabs.file_icons
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.tabs.get_or_insert_default().file_icons
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Tab Close Position",
                    description: "Position of the close button in a tab",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tabs) = &settings_content.tabs {
                                &tabs.close_position
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.tabs.get_or_insert_default().close_position
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Maximum Tabs",
                //     description: "Maximum open tabs in a pane. Will not close an unsaved tab",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.workspace.max_tabs,
                //         pick_mut: |settings_content| &mut settings_content.workspace.max_tabs,
                //     }),
                //     metadata: None,
                // }),
            ],
        },
        SettingsPage {
            title: "Languages & Frameworks",
            items: vec![
                SettingsPageItem::SectionHeader("General"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Enable Language Server",
                    description: "Whether to use language servers to provide code intelligence",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .enable_language_server
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .enable_language_server
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Languages"),
                SettingsPageItem::SubPageLink(SubPageLink {
                    title: "JSON",
                    render: Rc::new(|_, _, _| "A settings page!".into_any_element()),
                }),
            ],
        },
        SettingsPage {
            title: "Workbench & Window",
            items: vec![
                SettingsPageItem::SectionHeader("Workbench"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Editor Tabs",
                    description: "Whether or not to show the tab bar in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tab_bar) = &settings_content.tab_bar {
                                &tab_bar.show
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.tab_bar.get_or_insert_default().show
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Active language Button",
                    description: "Whether to show the active language button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(status_bar) = &settings_content.status_bar {
                                &status_bar.active_language_button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .status_bar
                                .get_or_insert_default()
                                .active_language_button
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Position Button",
                    description: "Whether to show the cursor position button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(status_bar) = &settings_content.status_bar {
                                &status_bar.cursor_position_button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .status_bar
                                .get_or_insert_default()
                                .cursor_position_button
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Terminal"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Terminal Button",
                    description: "Whether to show the terminal button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.terminal.get_or_insert_default().button
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Navigation History Buttons",
                    description: "Whether or not to show the navigation history buttons in the tab bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tab_bar) = &settings_content.tab_bar {
                                &tab_bar.show_nav_history_buttons
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .tab_bar
                                .get_or_insert_default()
                                .show_nav_history_buttons
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Panels & Tools",
            items: vec![
                SettingsPageItem::SectionHeader("Project Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Panel Button",
                    description: "Whether to show the project panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .button
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Panel Dock",
                    description: "Where to dock the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.dock
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project_panel.get_or_insert_default().dock
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Project Panel Default Width",
                //     description: "Default width of the project panel in pixels",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(project_panel) = &settings_content.project_panel {
                //                 &project_panel.default_width
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content
                //                 .project_panel
                //                 .get_or_insert_default()
                //                 .default_width
                //         },
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SectionHeader("Terminal"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Terminal Dock",
                    description: "Where to dock the terminal panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.dock
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.terminal.get_or_insert_default().dock
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Tab Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Activate On Close",
                    description: "What to do after closing the current tab",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tabs) = &settings_content.tabs {
                                &tabs.activate_on_close
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .tabs
                                .get_or_insert_default()
                                .activate_on_close
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Tab Show Diagnostics",
                    description: "Which files containing diagnostic errors/warnings to mark in the tabs",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tabs) = &settings_content.tabs {
                                &tabs.show_diagnostics
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .tabs
                                .get_or_insert_default()
                                .show_diagnostics
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Close Button",
                    description: "Controls the appearance behavior of the tab's close button",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(tabs) = &settings_content.tabs {
                                &tabs.show_close_button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .tabs
                                .get_or_insert_default()
                                .show_close_button
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Preview Tabs"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Preview Tabs Enabled",
                    description: "Whether to show opened editors as preview tabs",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(preview_tabs) = &settings_content.preview_tabs {
                                &preview_tabs.enabled
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enabled
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Enable Preview From File Finder",
                    description: "Whether to open tabs in preview mode when selected from the file finder",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(preview_tabs) = &settings_content.preview_tabs {
                                &preview_tabs.enable_preview_from_file_finder
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_from_file_finder
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Enable Preview From Code Navigation",
                    description: "Whether a preview tab gets replaced when code navigation is used to navigate away from the tab",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(preview_tabs) = &settings_content.preview_tabs {
                                &preview_tabs.enable_preview_from_code_navigation
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_from_code_navigation
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Version Control",
            items: vec![
                SettingsPageItem::SectionHeader("Git"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Gutter",
                    description: "Control whether the git gutter is shown",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                &git.git_gutter
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.git.get_or_insert_default().git_gutter
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Figure out the right default for this value in default.json
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Gutter Debounce",
                //     description: "Debounce threshold in milliseconds after which changes are reflected in the git gutter",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(git) = &settings_content.git {
                //                 &git.gutter_debounce
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content.git.get_or_insert_default().gutter_debounce
                //         },
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Blame Enabled",
                    description: "Whether or not to show git blame data inline in the currently focused line",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                if let Some(inline_blame) = &git.inline_blame {
                                    &inline_blame.enabled
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .enabled
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Blame Delay",
                    description: "The delay after which the inline blame information is shown",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                if let Some(inline_blame) = &git.inline_blame {
                                    &inline_blame.delay_ms
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .delay_ms
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Blame Padding",
                    description: "Padding between the end of the source line and the start of the inline blame in columns",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                if let Some(inline_blame) = &git.inline_blame {
                                    &inline_blame.padding
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .padding
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Blame Min Column",
                    description: "The minimum column number to show the inline blame information at",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                if let Some(inline_blame) = &git.inline_blame {
                                    &inline_blame.min_column
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .min_column
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Commit Summary",
                    description: "Whether to show commit summary as part of the inline blame",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                if let Some(inline_blame) = &git.inline_blame {
                                    &inline_blame.show_commit_summary
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .show_commit_summary
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Avatar",
                    description: "Whether to show the avatar of the author of the commit",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                if let Some(blame) = &git.blame {
                                    &blame.show_avatar
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git
                                .get_or_insert_default()
                                .blame
                                .get_or_insert_default()
                                .show_avatar
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Author Name In Branch Picker",
                    description: "Whether to show author name as part of the commit information in branch picker",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                if let Some(branch_picker) = &git.branch_picker {
                                    &branch_picker.show_author_name
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git
                                .get_or_insert_default()
                                .branch_picker
                                .get_or_insert_default()
                                .show_author_name
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hunk Style",
                    description: "How git hunks are displayed visually in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                &git.hunk_style
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.git.get_or_insert_default().hunk_style
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "System & Network",
            items: vec![
                SettingsPageItem::SectionHeader("Network"),
                // todo(settings_ui): Proxy needs a default
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Proxy",
                //     description: "The proxy to use for network requests",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.proxy,
                //         pick_mut: |settings_content| &mut settings_content.proxy,
                //     }),
                //     metadata: Some(Box::new(SettingsFieldMetadata {
                //         placeholder: Some("socks5h://localhost:10808"),
                //     })),
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Server URL",
                    description: "The URL of the Zed server to connect to",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.server_url,
                        pick_mut: |settings_content| &mut settings_content.server_url,
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        placeholder: Some("https://zed.dev"),
                    })),
                }),
                SettingsPageItem::SectionHeader("System"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Update",
                    description: "Whether or not to automatically check for updates",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.auto_update,
                        pick_mut: |settings_content| &mut settings_content.auto_update,
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Diagnostics & Errors",
            items: vec![
                SettingsPageItem::SectionHeader("Display"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Diagnostics Button",
                    description: "Whether to show the project diagnostics button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                &diagnostics.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.diagnostics.get_or_insert_default().button
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Filtering"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Max Severity",
                    description: "Which level to use to filter out diagnostics displayed in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.diagnostics_max_severity,
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.diagnostics_max_severity
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Include Warnings",
                    description: "Whether to show warnings or not by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                &diagnostics.include_warnings
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .diagnostics
                                .get_or_insert_default()
                                .include_warnings
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Inline"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Diagnostics Enabled",
                    description: "Whether to show diagnostics inline or not",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                if let Some(inline) = &diagnostics.inline {
                                    &inline.enabled
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .diagnostics
                                .get_or_insert_default()
                                .inline
                                .get_or_insert_default()
                                .enabled
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs numeric stepper
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Update Debounce",
                    description: "The delay in milliseconds to show inline diagnostics after the last diagnostic update",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                if let Some(inline) = &diagnostics.inline {
                                    &inline.update_debounce_ms
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .diagnostics
                                .get_or_insert_default()
                                .inline
                                .get_or_insert_default()
                                .update_debounce_ms
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs numeric stepper
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Padding",
                    description: "The amount of padding between the end of the source line and the start of the inline diagnostic",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                if let Some(inline) = &diagnostics.inline {
                                    &inline.padding
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .diagnostics
                                .get_or_insert_default()
                                .inline
                                .get_or_insert_default()
                                .padding
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs numeric stepper
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Min Column",
                    description: "The minimum column to display inline diagnostics",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                if let Some(inline) = &diagnostics.inline {
                                    &inline.min_column
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .diagnostics
                                .get_or_insert_default()
                                .inline
                                .get_or_insert_default()
                                .min_column
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Performance"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "LSP Pull Diagnostics Enabled",
                    description: "Whether to pull for diagnostics or not",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                if let Some(lsp_pull) = &diagnostics.lsp_pull_diagnostics {
                                    &lsp_pull.enabled
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .diagnostics
                                .get_or_insert_default()
                                .lsp_pull_diagnostics
                                .get_or_insert_default()
                                .enabled
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): Needs unit
                SettingsPageItem::SettingItem(SettingItem {
                    title: "LSP Pull Debounce",
                    description: "Minimum time to wait before pulling diagnostics from the language server(s)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(diagnostics) = &settings_content.diagnostics {
                                if let Some(lsp_pull) = &diagnostics.lsp_pull_diagnostics {
                                    &lsp_pull.debounce_ms
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .diagnostics
                                .get_or_insert_default()
                                .lsp_pull_diagnostics
                                .get_or_insert_default()
                                .debounce_ms
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Collaboration",
            items: vec![
                SettingsPageItem::SectionHeader("Calls"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Mute On Join",
                    description: "Whether the microphone should be muted when joining a channel or a call",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(calls) = &settings_content.calls {
                                &calls.mute_on_join
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.calls.get_or_insert_default().mute_on_join
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Share On Join",
                    description: "Whether your current project should be shared when joining an empty channel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(calls) = &settings_content.calls {
                                &calls.share_on_join
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.calls.get_or_insert_default().share_on_join
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Collaboration Panel Button",
                    description: "Whether to show the collaboration panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(collab) = &settings_content.collaboration_panel {
                                &collab.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .collaboration_panel
                                .get_or_insert_default()
                                .button
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Experimental"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Rodio Audio",
                    description: "Opt into the new audio system",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(audio) = &settings_content.audio {
                                &audio.rodio_audio
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.audio.get_or_insert_default().rodio_audio
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "AI",
            items: vec![
                SettingsPageItem::SectionHeader("General"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Disable AI",
                    description: "Whether to disable all AI features in Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.disable_ai,
                        pick_mut: |settings_content| &mut settings_content.disable_ai,
                    }),
                    metadata: None,
                }),
            ],
        },
    ]
}

pub(crate) fn project_settings_data() -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "Project",
            items: vec![
                SettingsPageItem::SectionHeader("Worktree Settings Content"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Name",
                    description: "The displayed name of this project. If not set, the root directory name",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.project.worktree.project_name,
                        pick_mut: |settings_content| {
                            &mut settings_content.project.worktree.project_name
                        },
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        placeholder: Some("A new name"),
                    })),
                }),
            ],
        },
        SettingsPage {
            title: "Appearance & Behavior",
            items: vec![
                SettingsPageItem::SectionHeader("Guides"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Wrap Guides",
                    description: "Whether to show wrap guides (vertical rulers)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides
                        },
                    }),
                    metadata: None,
                }),
                // todo(settings_ui): This needs a custom component
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Wrap Guides",
                //     description: "Character counts at which to show wrap guides",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             &settings_content
                //                 .project
                //                 .all_languages
                //                 .defaults
                //                 .wrap_guides
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content
                //                 .project
                //                 .all_languages
                //                 .defaults
                //                 .wrap_guides
                //         },
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SectionHeader("Whitespace"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Whitespace",
                    description: "Whether to show tabs and spaces",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_whitespaces
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_whitespaces
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Editing",
            items: vec![
                SettingsPageItem::SectionHeader("Indentation"),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Tab Size",
                //     description: "How many columns a tab should occupy",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.project.all_languages.defaults.tab_size,
                //         pick_mut: |settings_content| &mut settings_content.project.all_languages.defaults.tab_size,
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hard Tabs",
                    description: "Whether to indent lines using tab characters, as opposed to multiple spaces",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.project.all_languages.defaults.hard_tabs
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project.all_languages.defaults.hard_tabs
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Indent",
                    description: "Whether indentation should be adjusted based on the context whilst typing",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.project.all_languages.defaults.auto_indent
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project.all_languages.defaults.auto_indent
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Indent On Paste",
                    description: "Whether indentation of pasted content should be adjusted based on the context",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .auto_indent_on_paste
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .auto_indent_on_paste
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Wrapping"),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Preferred Line Length",
                //     description: "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.project.all_languages.defaults.preferred_line_length,
                //         pick_mut: |settings_content| &mut settings_content.project.all_languages.defaults.preferred_line_length,
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Soft Wrap",
                    description: "How to soft-wrap long lines of text",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.project.all_languages.defaults.soft_wrap
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.project.all_languages.defaults.soft_wrap
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Auto Actions"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use Autoclose",
                    description: "Whether to automatically type closing characters for you",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_autoclose
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_autoclose
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use Auto Surround",
                    description: "Whether to automatically surround text with characters for you",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_auto_surround
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_auto_surround
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use On Type Format",
                    description: "Whether to use additional LSP queries to format the code after every trigger symbol input",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_on_type_format
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .use_on_type_format
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Always Treat Brackets As Autoclosed",
                    description: "Controls how the editor handles the autoclosed characters",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .always_treat_brackets_as_autoclosed
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .always_treat_brackets_as_autoclosed
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Formatting"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Remove Trailing Whitespace On Save",
                    description: "Whether or not to remove any trailing whitespace from lines of a buffer before saving it",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .remove_trailing_whitespace_on_save
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .remove_trailing_whitespace_on_save
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Ensure Final Newline On Save",
                    description: "Whether or not to ensure there's a single newline at the end of a buffer when saving it",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .ensure_final_newline_on_save
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .ensure_final_newline_on_save
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Extend Comment On Newline",
                    description: "Whether to start a new line with a comment when a previous line is a comment as well",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .extend_comment_on_newline
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .extend_comment_on_newline
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Completions"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Completions On Input",
                    description: "Whether to pop the completions menu while typing in an editor without explicitly requesting it",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completions_on_input
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completions_on_input
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Completion Documentation",
                    description: "Whether to display inline and alongside documentation for items in the completions menu",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completion_documentation
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_completion_documentation
                        },
                    }),
                    metadata: None,
                }),
            ],
        },
    ]
}
