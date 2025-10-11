use settings::{LanguageSettingsContent, SettingsContent};
use std::sync::Arc;
use ui::{IntoElement, SharedString};

use crate::{
    LOCAL, SettingField, SettingItem, SettingsFieldMetadata, SettingsPage, SettingsPageItem,
    SubPageLink, USER, sub_page_stack,
};

pub(crate) fn settings_data() -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "General",
            items: vec![
                SettingsPageItem::SectionHeader("General Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Confirm Quit",
                    description: "Whether to confirm before quitting Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.confirm_quit,
                        pick_mut: |settings_content| &mut settings_content.workspace.confirm_quit,
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "When Closing With No Tabs",
                    description: "What to do when using the 'close active item' action with no tabs",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.workspace.when_closing_with_no_tabs
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.when_closing_with_no_tabs
                        },
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Path Prompts",
                    description: "Whether to use native OS dialogs for 'Open' and 'Save As'",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            &settings_content.workspace.use_system_path_prompts
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.use_system_path_prompts
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Prompts",
                    description: "Whether to use native OS dialogs for confirmations",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.use_system_prompts,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.use_system_prompts
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Scoped Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    // todo(settings_ui): Implement another setting item type that just shows an edit in settings.json
                    files: USER,
                    title: "Preview Channel",
                    description: "Which settings should be activated only in Preview build of Zed",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| &settings_content.workspace.use_system_prompts,
                            pick_mut: |settings_content| {
                                &mut settings_content.workspace.use_system_prompts
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Settings Profiles",
                    description: "Any number of settings profiles that are temporarily applied on top of your existing user settings",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| &settings_content.workspace.use_system_prompts,
                            pick_mut: |settings_content| {
                                &mut settings_content.workspace.use_system_prompts
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Privacy"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Telemetry Diagnostics",
                    description: "Send debug information like crash reports",
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Telemetry Metrics",
                    description: "Send anonymized usage data like what languages you're using Zed with",
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
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Appearance & Behavior",
            items: vec![
                SettingsPageItem::SectionHeader("Theme"),
                // todo(settings_ui): Figure out how we want to add these
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Theme Mode",
                    description: "How to select the theme",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| &settings_content.theme.theme,
                            pick_mut: |settings_content| &mut settings_content.theme.theme,
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Icon Theme",
                    // todo(settings_ui)
                    // This description is misleading because the icon theme is used in more places than the file explorer)
                    description: "Choose the icon theme for file explorer",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| &settings_content.theme.icon_theme,
                            pick_mut: |settings_content| &mut settings_content.theme.icon_theme,
                        }
                        .unimplemented(),
                    ),
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Buffer Font Size",
                    description: "Font size for editor text",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.buffer_font_size,
                        pick_mut: |settings_content| &mut settings_content.theme.buffer_font_size,
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Buffer Font Weight",
                    description: "Font weight for editor text (100-900)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.buffer_font_weight,
                        pick_mut: |settings_content| &mut settings_content.theme.buffer_font_weight,
                    }),
                    metadata: None,
                    files: USER,
                }),
                // todo(settings_ui): This needs custom ui
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Buffer Line Height",
                    description: "Line height for editor text",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| &settings_content.theme.buffer_line_height,
                            pick_mut: |settings_content| {
                                &mut settings_content.theme.buffer_line_height
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "UI Font Family",
                    description: "Font family for UI elements",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.ui_font_family,
                        pick_mut: |settings_content| &mut settings_content.theme.ui_font_family,
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "UI Font Size",
                    description: "Font size for UI elements",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.ui_font_size,
                        pick_mut: |settings_content| &mut settings_content.theme.ui_font_size,
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "UI Font Weight",
                    description: "Font weight for UI elements (100-900)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.ui_font_weight,
                        pick_mut: |settings_content| &mut settings_content.theme.ui_font_weight,
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Agent Panel UI Font Size",
                    description: "Font size for agent response text in the agent panel. Falls back to the regular UI font size.",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if settings_content.theme.agent_ui_font_size.is_some() {
                                &settings_content.theme.agent_ui_font_size
                            } else {
                                &settings_content.theme.ui_font_size
                            }
                        },
                        pick_mut: |settings_content| &mut settings_content.theme.agent_ui_font_size,
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Agent Panel Buffer Font Size",
                    description: "Font size for user messages text in the agent panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.agent_buffer_font_size,
                        pick_mut: |settings_content| {
                            &mut settings_content.theme.agent_buffer_font_size
                        },
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Helix Mode",
                    description: "Whether to enable helix modes and key bindings",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.helix_mode,
                        pick_mut: |settings_content| &mut settings_content.helix_mode,
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Shape",
                    description: "Cursor shape for the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.cursor_shape,
                        pick_mut: |settings_content| &mut settings_content.editor.cursor_shape,
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hide Mouse",
                    description: "When to hide the mouse cursor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.hide_mouse,
                        pick_mut: |settings_content| &mut settings_content.editor.hide_mouse,
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Rounded Selection",
                    description: "Whether the text selection should have rounded corners",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.rounded_selection,
                        pick_mut: |settings_content| &mut settings_content.editor.rounded_selection,
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER | LOCAL,
                }),
                // todo(settings_ui): This needs a custom component
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Wrap Guides",
                    description: "Character counts at which to show wrap guides",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                &settings_content.project.all_languages.defaults.wrap_guides
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.project.all_languages.defaults.wrap_guides
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | LOCAL,
                }),
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
                    files: USER | LOCAL,
                }),
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Centered Layout Left Padding",
                    description: "Left padding for centered layout",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(centered_layout) =
                                &settings_content.workspace.centered_layout
                            {
                                &centered_layout.left_padding
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .workspace
                                .centered_layout
                                .get_or_insert_default()
                                .left_padding
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Centered Layout Right Padding",
                    description: "Right padding for centered layout",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(centered_layout) =
                                &settings_content.workspace.centered_layout
                            {
                                &centered_layout.right_padding
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .workspace
                                .centered_layout
                                .get_or_insert_default()
                                .right_padding
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
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Window"),
                // todo(settings_ui): Should we filter by platform?
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Window Tabs",
                    description: "(macOS only) Whether to allow windows to tab together",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.use_system_window_tabs,
                        pick_mut: |settings_content| {
                            &mut settings_content.workspace.use_system_window_tabs
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Editor",
            items: {
                let mut items = vec![
                    SettingsPageItem::SectionHeader("Search"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Search Wrap",
                        description: "Whether the editor search results will loop",
                        field: Box::new(SettingField {
                            pick: |settings_content| &settings_content.editor.search_wrap,
                            pick_mut: |settings_content| &mut settings_content.editor.search_wrap,
                        }),
                        metadata: None,
                        files: USER,
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
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Use Smartcase Search",
                        description: "Whether to use smartcase (i.e., case-sensitive) search",
                        field: Box::new(SettingField {
                            pick: |settings_content| &settings_content.editor.use_smartcase_search,
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.use_smartcase_search
                            },
                        }),
                        metadata: None,
                        files: USER,
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
                        files: USER,
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
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Double Click In Multibuffer",
                        description: "What to do when multibuffer is double-clicked in some of its excerpts",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                &settings_content.editor.double_click_in_multibuffer
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.double_click_in_multibuffer
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Go To Definition Fallback",
                        description: "Whether to follow-up empty go to definition responses from the language server",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                &settings_content.editor.go_to_definition_fallback
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.go_to_definition_fallback
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Expand Excerpt Lines",
                        description: "How many lines to expand the multibuffer excerpts by default",
                        field: Box::new(SettingField {
                            pick: |settings_content| &settings_content.editor.expand_excerpt_lines,
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.expand_excerpt_lines
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Excerpt Context Lines",
                        description: "How many lines of context to provide in multibuffer excerpts by default",
                        field: Box::new(SettingField {
                            pick: |settings_content| &settings_content.editor.excerpt_context_lines,
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.excerpt_context_lines
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Minimum Contrast For Highlights",
                        description: "The minimum APCA perceptual contrast to maintain when rendering text over highlight backgrounds",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                &settings_content.editor.minimum_contrast_for_highlights
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.minimum_contrast_for_highlights
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Scrolling"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Scroll Beyond Last Line",
                        description: "Whether the editor will scroll beyond the last line",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                &settings_content.editor.scroll_beyond_last_line
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.scroll_beyond_last_line
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Vertical Scroll Margin",
                        description: "The number of lines to keep above/below the cursor when auto-scrolling",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                &settings_content.editor.vertical_scroll_margin
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.vertical_scroll_margin
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Horizontal Scroll Margin",
                        description: "The number of characters to keep on either side when scrolling with the mouse",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                &settings_content.editor.horizontal_scroll_margin
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.horizontal_scroll_margin
                            },
                        }),
                        metadata: None,
                        files: USER,
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
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Fast Scroll Sensitivity",
                        description: "Fast Scroll sensitivity multiplier for both horizontal and vertical scrolling",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                &settings_content.editor.fast_scroll_sensitivity
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.fast_scroll_sensitivity
                            },
                        }),
                        metadata: None,
                        files: USER,
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
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Signature Help"),
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
                        files: USER,
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
                        files: USER,
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
                        files: USER,
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
                        files: USER,
                    }),
                    // todo(settings ui): add units to this number input
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
                        files: USER,
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
                        files: USER,
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
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Drag And Drop Selection Delay",
                        description: "Delay in milliseconds before drag and drop selection starts",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(drag_and_drop) =
                                    &settings_content.editor.drag_and_drop_selection
                                {
                                    &drag_and_drop.delay
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .get_or_insert_default()
                                    .delay
                            },
                        }),
                        metadata: None,
                        files: USER,
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
                        files: USER,
                    }),
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
                        files: USER,
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
                        files: USER,
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
                        files: USER,
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
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Min Line Number Digits",
                        description: "Minimum number of characters to reserve space for in the gutter",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(gutter) = &settings_content.editor.gutter {
                                    &gutter.min_line_number_digits
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .min_line_number_digits
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Scrollbar"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Show",
                        description: "When to show the scrollbar in the editor",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    &scrollbar.show
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .show
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Cursors",
                        description: "Whether to show cursor positions in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    &scrollbar.cursors
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .cursors
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Git Diff",
                        description: "Whether to show git diff indicators in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    &scrollbar.git_diff
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .git_diff
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Search Results",
                        description: "Whether to show buffer search result indicators in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    &scrollbar.search_results
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .search_results
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Selected Text",
                        description: "Whether to show selected text occurrences in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    &scrollbar.selected_text
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .selected_text
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Selected Symbol",
                        description: "Whether to show selected symbol occurrences in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    &scrollbar.selected_symbol
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .selected_symbol
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Diagnostics",
                        description: "Which diagnostic indicators to show in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    &scrollbar.diagnostics
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .diagnostics
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Horizontal Scrollbar",
                        description: "When false, forcefully disables the horizontal scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    if let Some(axes) = &scrollbar.axes {
                                        &axes.horizontal
                                    } else {
                                        &None
                                    }
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .axes
                                    .get_or_insert_default()
                                    .horizontal
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Vertical Scrollbar",
                        description: "When false, forcefully disables the vertical scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                    if let Some(axes) = &scrollbar.axes {
                                        &axes.vertical
                                    } else {
                                        &None
                                    }
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .axes
                                    .get_or_insert_default()
                                    .vertical
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Minimap"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Show",
                        description: "When to show the minimap in the editor",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(minimap) = &settings_content.editor.minimap {
                                    &minimap.show
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content.editor.minimap.get_or_insert_default().show
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Display In",
                        description: "Where to show the minimap in the editor",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(minimap) = &settings_content.editor.minimap {
                                    &minimap.display_in
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .display_in
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Thumb",
                        description: "When to show the minimap thumb",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(minimap) = &settings_content.editor.minimap {
                                    &minimap.thumb
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .thumb
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Thumb Border",
                        description: "Border style for the minimap's scrollbar thumb",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(minimap) = &settings_content.editor.minimap {
                                    &minimap.thumb_border
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .thumb_border
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Current Line Highlight",
                        description: "How to highlight the current line in the minimap",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(minimap) = &settings_content.editor.minimap
                                    && minimap.current_line_highlight.is_some()
                                {
                                    &minimap.current_line_highlight
                                } else {
                                    &settings_content.editor.current_line_highlight
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .current_line_highlight
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Max Width Columns",
                        description: "Maximum number of columns to display in the minimap",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(minimap) = &settings_content.editor.minimap {
                                    &minimap.max_width_columns
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .max_width_columns
                            },
                        }),
                        metadata: None,
                        files: USER,
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
                        files: USER,
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
                        files: USER,
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
                        files: USER,
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
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        files: USER,
                        title: "Maximum Tabs",
                        description: "Maximum open tabs in a pane. Will not close an unsaved tab",
                        // todo(settings_ui): The default for this value is null and it's use in code
                        // is complex, so I'm going to come back to this later
                        field: Box::new(
                            SettingField {
                                pick: |settings_content| &settings_content.workspace.max_tabs,
                                pick_mut: |settings_content| {
                                    &mut settings_content.workspace.max_tabs
                                },
                            }
                            .unimplemented(),
                        ),
                        metadata: None,
                    }),
                    SettingsPageItem::SectionHeader("Toolbar"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Breadcrumbs",
                        description: "Whether to show breadcrumbs",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(toolbar) = &settings_content.editor.toolbar {
                                    &toolbar.breadcrumbs
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .breadcrumbs
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Quick Actions",
                        description: "Whether to show quick action buttons (e.g., search, selection, editor controls, etc.)",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(toolbar) = &settings_content.editor.toolbar {
                                    &toolbar.quick_actions
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .quick_actions
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Selections Menu",
                        description: "Whether to show the selections menu in the editor toolbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(toolbar) = &settings_content.editor.toolbar {
                                    &toolbar.selections_menu
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .selections_menu
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Agent Review",
                        description: "Whether to show agent review buttons in the editor toolbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(toolbar) = &settings_content.editor.toolbar {
                                    &toolbar.agent_review
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .agent_review
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Code Actions",
                        description: "Whether to show code action buttons in the editor toolbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                if let Some(toolbar) = &settings_content.editor.toolbar {
                                    &toolbar.code_actions
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .code_actions
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                ];
                items.extend(language_settings_data());
                items
            },
        },
        SettingsPage {
            title: "Languages",
            items: vec![
                SettingsPageItem::SectionHeader(LANGUAGES_SECTION_HEADER),
                SettingsPageItem::SubPageLink(SubPageLink {
                    title: "JSON",
                    files: USER | LOCAL,
                    render: Arc::new(|this, window, cx| {
                        this.render_page_items(
                            language_settings_data().iter().enumerate(),
                            None,
                            window,
                            cx,
                        )
                        .into_any_element()
                    }),
                }),
                SettingsPageItem::SubPageLink(SubPageLink {
                    title: "JSONC",
                    files: USER | LOCAL,
                    render: Arc::new(|this, window, cx| {
                        this.render_page_items(
                            language_settings_data().iter().enumerate(),
                            None,
                            window,
                            cx,
                        )
                        .into_any_element()
                    }),
                }),
                SettingsPageItem::SubPageLink(SubPageLink {
                    title: "Rust",
                    files: USER | LOCAL,
                    render: Arc::new(|this, window, cx| {
                        this.render_page_items(
                            language_settings_data().iter().enumerate(),
                            None,
                            window,
                            cx,
                        )
                        .into_any_element()
                    }),
                }),
                SettingsPageItem::SubPageLink(SubPageLink {
                    title: "Python",
                    files: USER | LOCAL,
                    render: Arc::new(|this, window, cx| {
                        this.render_page_items(
                            language_settings_data().iter().enumerate(),
                            None,
                            window,
                            cx,
                        )
                        .into_any_element()
                    }),
                }),
                SettingsPageItem::SubPageLink(SubPageLink {
                    title: "TSX",
                    files: USER | LOCAL,
                    render: Arc::new(|this, window, cx| {
                        this.render_page_items(
                            language_settings_data().iter().enumerate(),
                            None,
                            window,
                            cx,
                        )
                        .into_any_element()
                    }),
                }),
            ],
        },
        SettingsPage {
            title: "Workbench & Window",
            items: vec![
                SettingsPageItem::SectionHeader("Status Bar"),
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Active Language Button",
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
                    files: USER,
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
                    files: USER,
                }),
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
                    files: USER,
                }),
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Search Button",
                    description: "Whether to show the project search button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(search) = &settings_content.editor.search {
                                &search.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .button
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Debugger Button",
                    description: "Whether to show the debugger button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(debugger) = &settings_content.debugger {
                                &debugger.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.debugger.get_or_insert_default().button
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Tab Bar"),
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
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Title Bar"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Branch Icon",
                    description: "Whether to show the branch icon beside branch switcher in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(title_bar) = &settings_content.title_bar {
                                &title_bar.show_branch_icon
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_branch_icon
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Branch Name",
                    description: "Whether to show the branch name button in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(title_bar) = &settings_content.title_bar {
                                &title_bar.show_branch_name
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_branch_name
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Project Items",
                    description: "Whether to show the project host and name in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(title_bar) = &settings_content.title_bar {
                                &title_bar.show_project_items
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_project_items
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Onboarding Banner",
                    description: "Whether to show banners announcing new features in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(title_bar) = &settings_content.title_bar {
                                &title_bar.show_onboarding_banner
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_onboarding_banner
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show User Picture",
                    description: "Whether to show user picture in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(title_bar) = &settings_content.title_bar {
                                &title_bar.show_user_picture
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_user_picture
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Sign In",
                    description: "Whether to show the sign in button in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(title_bar) = &settings_content.title_bar {
                                &title_bar.show_sign_in
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_sign_in
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Menus",
                    description: "Whether to show the menus in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(title_bar) = &settings_content.title_bar {
                                &title_bar.show_menus
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_menus
                        },
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Search Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Whole Word",
                    description: "Whether to search for whole words by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(search) = &settings_content.editor.search {
                                &search.whole_word
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .whole_word
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Case Sensitive",
                    description: "Whether to search case-sensitively by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(search) = &settings_content.editor.search {
                                &search.case_sensitive
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .case_sensitive
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Include Ignored",
                    description: "Whether to include ignored files in search results by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(search) = &settings_content.editor.search {
                                &search.include_ignored
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .include_ignored
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Regex",
                    description: "Whether to use regex search by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(search) = &settings_content.editor.search {
                                &search.regex
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.editor.search.get_or_insert_default().regex
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("File Finder"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Icons",
                    description: "Whether to show file icons in the file finder",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(file_finder) = &settings_content.file_finder {
                                &file_finder.file_icons
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .file_finder
                                .get_or_insert_default()
                                .file_icons
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Modal Max Width",
                    description: "Determines how much space the file finder can take up in relation to the available window width",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(file_finder) = &settings_content.file_finder {
                                &file_finder.modal_max_width
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .file_finder
                                .get_or_insert_default()
                                .modal_max_width
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Skip Focus For Active In Search",
                    description: "Whether the file finder should skip focus for the active file in search results",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(file_finder) = &settings_content.file_finder {
                                &file_finder.skip_focus_for_active_in_search
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .file_finder
                                .get_or_insert_default()
                                .skip_focus_for_active_in_search
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Status",
                    description: "Whether to show the git status in the file finder",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(file_finder) = &settings_content.file_finder {
                                &file_finder.git_status
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .file_finder
                                .get_or_insert_default()
                                .git_status
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                // todo: null by default
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Include Ignored",
                    description: "Whether to use gitignored files when searching",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(file_finder) = &settings_content.file_finder {
                                    &file_finder.include_ignored
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .file_finder
                                    .get_or_insert_default()
                                    .include_ignored
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Panels",
            items: vec![
                SettingsPageItem::SectionHeader("Project Panel"),
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Panel Default Width",
                    description: "Default width of the project panel in pixels",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.default_width
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .default_width
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hide .gitignore",
                    description: "Whether to hide the gitignore entries in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.hide_gitignore
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_gitignore
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Entry Spacing",
                    description: "Spacing between worktree entries in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.entry_spacing
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .entry_spacing
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Icons",
                    description: "Whether to show file icons in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.file_icons
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .file_icons
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Folder Icons",
                    description: "Whether to show folder icons or chevrons for directories in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.folder_icons
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .folder_icons
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Status",
                    description: "Whether to show the git status in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.git_status
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .git_status
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Indent Size",
                    description: "Amount of indentation for nested items",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.indent_size
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .indent_size
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Reveal Entries",
                    description: "Whether to reveal it in the project panel automatically when a corresponding project entry becomes active",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.auto_reveal_entries
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .auto_reveal_entries
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Starts Open",
                    description: "Whether the project panel should open on startup",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.starts_open
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .starts_open
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Fold Directories",
                    description: "Whether to fold directories automatically and show compact folders when a directory has only one subdirectory inside",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.auto_fold_dirs
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .auto_fold_dirs
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Scrollbar Show",
                    description: "When to show the scrollbar in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel
                                && let Some(scrollbar) = &project_panel.scrollbar
                                && scrollbar.show.is_some()
                            {
                                &scrollbar.show
                            } else if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                &scrollbar.show
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Diagnostics",
                    description: "Which files containing diagnostic errors/warnings to mark in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.show_diagnostics
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .show_diagnostics
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Sticky Scroll",
                    description: "Whether to stick parent directories at top of the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.sticky_scroll
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .sticky_scroll
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Indent Guides Show",
                    description: "When to show indent guides in the project panel",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(project_panel) = &settings_content.project_panel {
                                    if let Some(indent_guides) = &project_panel.indent_guides {
                                        &indent_guides.show
                                    } else {
                                        &None
                                    }
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .project_panel
                                    .get_or_insert_default()
                                    .indent_guides
                                    .get_or_insert_default()
                                    .show
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Drag and Drop",
                    description: "Whether to enable drag-and-drop operations in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.drag_and_drop
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .drag_and_drop
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hide Root",
                    description: "Whether to hide the root entry when only one folder is open in the window",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(project_panel) = &settings_content.project_panel {
                                &project_panel.hide_root
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_root
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Terminal Panel"),
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
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Outline Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Outline Panel Button",
                    description: "Whether to show the outline panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .button
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Outline Panel Dock",
                    description: "Where to dock the outline panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.dock
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.outline_panel.get_or_insert_default().dock
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Outline Panel Default Width",
                    description: "Default width of the outline panel in pixels",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.default_width
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .default_width
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Icons",
                    description: "Whether to show file icons in the outline panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.file_icons
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .file_icons
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Folder Icons",
                    description: "Whether to show folder icons or chevrons for directories in the outline panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.folder_icons
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .folder_icons
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Status",
                    description: "Whether to show the git status in the outline panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.git_status
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .git_status
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Indent Size",
                    description: "Amount of indentation for nested items",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.indent_size
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .indent_size
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Reveal Entries",
                    description: "Whether to reveal when a corresponding outline entry becomes active",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.auto_reveal_entries
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .auto_reveal_entries
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Fold Directories",
                    description: "Whether to fold directories automatically when a directory has only one directory inside",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(outline_panel) = &settings_content.outline_panel {
                                &outline_panel.auto_fold_dirs
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .auto_fold_dirs
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Indent Guides Show",
                    description: "When to show indent guides in the outline panel",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(outline_panel) = &settings_content.outline_panel {
                                    if let Some(indent_guides) = &outline_panel.indent_guides {
                                        &indent_guides.show
                                    } else {
                                        &None
                                    }
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .outline_panel
                                    .get_or_insert_default()
                                    .indent_guides
                                    .get_or_insert_default()
                                    .show
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Git Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Panel Button",
                    description: "Whether to show the Git panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git_panel) = &settings_content.git_panel {
                                &git_panel.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.git_panel.get_or_insert_default().button
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Panel Dock",
                    description: "Where to dock the Git panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git_panel) = &settings_content.git_panel {
                                &git_panel.dock
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.git_panel.get_or_insert_default().dock
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Panel Default Width",
                    description: "Default width of the Git panel in pixels",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git_panel) = &settings_content.git_panel {
                                &git_panel.default_width
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .git_panel
                                .get_or_insert_default()
                                .default_width
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Debugger Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Debugger Panel Dock",
                    description: "The dock position of the debug panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(debugger) = &settings_content.debugger {
                                &debugger.dock
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.debugger.get_or_insert_default().dock
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Notification Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Notification Panel Button",
                    description: "Whether to show the notification panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(notification_panel) = &settings_content.notification_panel {
                                &notification_panel.button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .notification_panel
                                .get_or_insert_default()
                                .button
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Notification Panel Dock",
                    description: "Where to dock the notification panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(notification_panel) = &settings_content.notification_panel {
                                &notification_panel.dock
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .notification_panel
                                .get_or_insert_default()
                                .dock
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Notification Panel Default Width",
                    description: "Default width of the notification panel in pixels",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(notification_panel) = &settings_content.notification_panel {
                                &notification_panel.default_width
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .notification_panel
                                .get_or_insert_default()
                                .default_width
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Collaboration Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Collaboration Panel Button",
                    description: "Whether to show the collaboration panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(collaboration_panel) = &settings_content.collaboration_panel
                            {
                                &collaboration_panel.button
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Collaboration Panel Dock",
                    description: "Where to dock the collaboration panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(collaboration_panel) = &settings_content.collaboration_panel
                            {
                                &collaboration_panel.dock
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .collaboration_panel
                                .get_or_insert_default()
                                .dock
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Collaboration Panel Default Width",
                    description: "Default width of the collaboration panel in pixels",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(collaboration_panel) = &settings_content.collaboration_panel
                            {
                                &collaboration_panel.default_width
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .collaboration_panel
                                .get_or_insert_default()
                                .default_width
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Version Control",
            items: vec![
                SettingsPageItem::SectionHeader("Git"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Gutter",
                    description: "Control whether git status is shown in the editor's gutter",
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
                    files: USER,
                }),
                // todo(settings_ui): Figure out the right default for this value in default.json
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gutter Debounce",
                    description: "Debounce threshold in milliseconds after which changes are reflected in the git gutter",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(git) = &settings_content.git {
                                &git.gutter_debounce
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.git.get_or_insert_default().gutter_debounce
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Git Blame",
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Git Blame Delay",
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Git Blame Padding",
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inline Git Blame Min Column",
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "System & Network",
            items: vec![
                SettingsPageItem::SectionHeader("Network"),
                // todo(settings_ui): Proxy needs a default
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Proxy",
                    description: "The proxy to use for network requests",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| &settings_content.proxy,
                            pick_mut: |settings_content| &mut settings_content.proxy,
                        }
                        .unimplemented(),
                    ),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        placeholder: Some("socks5h://localhost:10808"),
                    })),
                    files: USER,
                }),
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
                    files: USER,
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
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Diagnostics & Errors",
            items: vec![
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
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
                    files: USER,
                }),
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
                    files: USER,
                }),
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
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Performance"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "LSP Pull Diagnostics Enabled",
                    description: "Whether to pull for language server-powered diagnostics or not",
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
                    files: USER,
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
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Debugger",
            items: vec![
                SettingsPageItem::SectionHeader("General"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Stepping Granularity",
                    description: "Determines the stepping granularity for debug operations",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(debugger) = &settings_content.debugger {
                                &debugger.stepping_granularity
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .debugger
                                .get_or_insert_default()
                                .stepping_granularity
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Save Breakpoints",
                    description: "Whether breakpoints should be reused across Zed sessions",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(debugger) = &settings_content.debugger {
                                &debugger.save_breakpoints
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .debugger
                                .get_or_insert_default()
                                .save_breakpoints
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Timeout",
                    description: "Time in milliseconds until timeout error when connecting to a TCP debug adapter",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(debugger) = &settings_content.debugger {
                                &debugger.timeout
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.debugger.get_or_insert_default().timeout
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Log DAP Communications",
                    description: "Whether to log messages between active debug adapters and Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(debugger) = &settings_content.debugger {
                                &debugger.log_dap_communications
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .debugger
                                .get_or_insert_default()
                                .log_dap_communications
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Format DAP Log Messages",
                    description: "Whether to format DAP messages when adding them to debug adapter logger",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(debugger) = &settings_content.debugger {
                                &debugger.format_dap_log_messages
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .debugger
                                .get_or_insert_default()
                                .format_dap_log_messages
                        },
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
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
                    files: USER,
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
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Microphone Volume",
                    description: "Automatically adjust microphone volume (requires Rodio Audio)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(audio) = &settings_content.audio {
                                &audio.auto_microphone_volume
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .audio
                                .get_or_insert_default()
                                .auto_microphone_volume
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Speaker Volume",
                    description: "Automatically adjust volume of other call members (requires Rodio Audio)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(audio) = &settings_content.audio {
                                &audio.auto_speaker_volume
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .audio
                                .get_or_insert_default()
                                .auto_speaker_volume
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Denoise",
                    description: "Remove background noises (requires Rodio Audio)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(audio) = &settings_content.audio {
                                &audio.denoise
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.audio.get_or_insert_default().denoise
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Legacy Audio Compatible",
                    description: "Use audio parameters compatible with previous versions (requires Rodio Audio)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(audio) = &settings_content.audio {
                                &audio.legacy_audio_compatible
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .audio
                                .get_or_insert_default()
                                .legacy_audio_compatible
                        },
                    }),
                    metadata: None,
                    files: USER,
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
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Terminal",
            items: vec![
                SettingsPageItem::SectionHeader("Environment"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Shell",
                    description: "What shell to use when opening a terminal",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(terminal) = &settings_content.terminal {
                                    &terminal.project.shell
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .shell
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | LOCAL,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Working Directory",
                    description: "What working directory to use when launching the terminal",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(terminal) = &settings_content.terminal {
                                    &terminal.project.working_directory
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .working_directory
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | LOCAL,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Environment Variables",
                    description: "Key-value pairs to add to the terminal's environment",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(terminal) = &settings_content.terminal {
                                    &terminal.project.env
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .env
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | LOCAL,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Detect Virtual Environment",
                    description: "Activates the python virtual environment, if one is found, in the terminal's working directory",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(terminal) = &settings_content.terminal {
                                    &terminal.project.detect_venv
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .detect_venv
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | LOCAL,
                }),
                SettingsPageItem::SectionHeader("Font"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Size",
                    description: "Font size for terminal text. If not set, defaults to buffer font size",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.font_size
                            } else if settings_content.theme.buffer_font_size.is_some() {
                                &settings_content.theme.buffer_font_size
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.terminal.get_or_insert_default().font_size
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Family",
                    description: "Font family for terminal text. If not set, defaults to buffer font family",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal
                                && terminal.font_family.is_some()
                            {
                                &terminal.font_family
                            } else if settings_content.theme.buffer_font_family.is_some() {
                                &settings_content.theme.buffer_font_family
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .font_family
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Fallbacks",
                    description: "Font fallbacks for terminal text. If not set, defaults to buffer font fallbacks",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(terminal) = &settings_content.terminal {
                                    &terminal.font_fallbacks
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .font_fallbacks
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Weight",
                    description: "Font weight for terminal text in CSS weight units (100-900)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.font_weight
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .font_weight
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Features",
                    description: "Font features for terminal text",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(terminal) = &settings_content.terminal {
                                    &terminal.font_features
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .font_features
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Display Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Line Height",
                    description: "Line height for terminal text",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                if let Some(terminal) = &settings_content.terminal {
                                    &terminal.line_height
                                } else {
                                    &None
                                }
                            },
                            pick_mut: |settings_content| {
                                &mut settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .line_height
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Shape",
                    description: "Default cursor shape for the terminal (bar, block, underline, or hollow)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.cursor_shape
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .cursor_shape
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Blinking",
                    description: "Sets the cursor blinking behavior in the terminal",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.blinking
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content.terminal.get_or_insert_default().blinking
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Alternate Scroll",
                    description: "Whether Alternate Scroll mode is active by default (converts mouse scroll to arrow keys in apps like vim)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.alternate_scroll
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .alternate_scroll
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Minimum Contrast",
                    description: "The minimum APCA perceptual contrast between foreground and background colors (0-106)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.minimum_contrast
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .minimum_contrast
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Behavior Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Option As Meta",
                    description: "Whether the option key behaves as the meta key",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.option_as_meta
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .option_as_meta
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Copy On Select",
                    description: "Whether selecting text in the terminal automatically copies to the system clipboard",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.copy_on_select
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .copy_on_select
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Keep Selection On Copy",
                    description: "Whether to keep the text selection after copying it to the clipboard",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.keep_selection_on_copy
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .keep_selection_on_copy
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Layout Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Default Width",
                    description: "Default width when the terminal is docked to the left or right (in pixels)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.default_width
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .default_width
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Default Height",
                    description: "Default height when the terminal is docked to the bottom (in pixels)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.default_height
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .default_height
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Advanced Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Max Scroll History Lines",
                    description: "Maximum number of lines to keep in scrollback history (max: 100,000; 0 disables scrolling)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                &terminal.max_scroll_history_lines
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .max_scroll_history_lines
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Toolbar"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Breadcrumbs",
                    description: "Whether to display the terminal title in breadcrumbs inside the terminal pane",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal {
                                if let Some(toolbar) = &terminal.toolbar {
                                    &toolbar.breadcrumbs
                                } else {
                                    &None
                                }
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .toolbar
                                .get_or_insert_default()
                                .breadcrumbs
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Scrollbar"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Scrollbar",
                    description: "When to show the scrollbar in the terminal",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            if let Some(terminal) = &settings_content.terminal
                                && let Some(scrollbar) = &terminal.scrollbar
                                && scrollbar.show.is_some()
                            {
                                &scrollbar.show
                            } else if let Some(scrollbar) = &settings_content.editor.scrollbar {
                                &scrollbar.show
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .terminal
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
    ]
}

const LANGUAGES_SECTION_HEADER: &'static str = "Languages";

fn language_settings_data() -> Vec<SettingsPageItem> {
    fn current_language() -> Option<SharedString> {
        sub_page_stack().iter().find_map(|page| {
            (page.section_header == LANGUAGES_SECTION_HEADER)
                .then(|| SharedString::new_static(page.link.title))
        })
    }

    fn language_settings_field<T>(
        settings_content: &SettingsContent,
        get: fn(&LanguageSettingsContent) -> &Option<T>,
    ) -> &Option<T> {
        let all_languages = &settings_content.project.all_languages;
        if let Some(current_language_name) = current_language() {
            if let Some(current_language) = all_languages.languages.0.get(&current_language_name) {
                let value = get(current_language);
                if value.is_some() {
                    return value;
                }
            }
        }
        let default_value = get(&all_languages.defaults);
        return default_value;
    }

    fn language_settings_field_mut<T>(
        settings_content: &mut SettingsContent,
        get: fn(&mut LanguageSettingsContent) -> &mut Option<T>,
    ) -> &mut Option<T> {
        let all_languages = &mut settings_content.project.all_languages;
        let language_content = if let Some(current_language) = current_language() {
            all_languages
                .languages
                .0
                .entry(current_language)
                .or_default()
        } else {
            &mut all_languages.defaults
        };
        return get(language_content);
    }

    vec![
        SettingsPageItem::SectionHeader("Indentation"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Tab Size",
            description: "How many columns a tab should occupy",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.tab_size)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| &mut language.tab_size)
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Hard Tabs",
            description: "Whether to indent lines using tab characters, as opposed to multiple spaces",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.hard_tabs)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.hard_tabs
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Auto Indent",
            description: "Whether indentation should be adjusted based on the context whilst typing",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.auto_indent)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.auto_indent
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Auto Indent On Paste",
            description: "Whether indentation of pasted content should be adjusted based on the context",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.auto_indent_on_paste
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.auto_indent_on_paste
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Wrapping"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Soft Wrap",
            description: "How to soft-wrap long lines of text",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.soft_wrap)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.soft_wrap
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Wrap Guides",
            description: "Whether to show wrap guides in the editor",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.show_wrap_guides)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.show_wrap_guides
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Preferred Line Length",
            description: "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.preferred_line_length
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.preferred_line_length
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Wrap Guides",
            description: "Character counts at which to show wrap guides in the editor",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| &language.wrap_guides)
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.wrap_guides
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Allow Rewrap",
            description: "Controls where the `editor::Rewrap` action is allowed for this language",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.allow_rewrap)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.allow_rewrap
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Indent Guides"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enabled",
            description: "Whether to display indent guides in the editor",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(indent_guides) = &language.indent_guides {
                            &indent_guides.enabled
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.indent_guides.get_or_insert_default().enabled
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Line Width",
            description: "The width of the indent guides in pixels, between 1 and 10",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(indent_guides) = &language.indent_guides {
                            &indent_guides.line_width
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.indent_guides.get_or_insert_default().line_width
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Active Line Width",
            description: "The width of the active indent guide in pixels, between 1 and 10",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(indent_guides) = &language.indent_guides {
                            &indent_guides.active_line_width
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .indent_guides
                            .get_or_insert_default()
                            .active_line_width
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Coloring",
            description: "Determines how indent guides are colored",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(indent_guides) = &language.indent_guides {
                            &indent_guides.coloring
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.indent_guides.get_or_insert_default().coloring
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Background Coloring",
            description: "Determines how indent guide backgrounds are colored",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(indent_guides) = &language.indent_guides {
                            &indent_guides.background_coloring
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .indent_guides
                            .get_or_insert_default()
                            .background_coloring
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Formatting"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Format On Save",
            description: "Whether or not to perform a buffer format before saving",
            field: Box::new(
                // TODO(settings_ui): this setting should just be a bool
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            &language.format_on_save
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.format_on_save
                        })
                    },
                },
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Remove Trailing Whitespace On Save",
            description: "Whether or not to remove any trailing whitespace from lines of a buffer before saving it",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.remove_trailing_whitespace_on_save
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.remove_trailing_whitespace_on_save
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Ensure Final Newline On Save",
            description: "Whether or not to ensure there's a single newline at the end of a buffer when saving it",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.ensure_final_newline_on_save
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.ensure_final_newline_on_save
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Formatter",
            description: "How to perform a buffer format",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| &language.formatter)
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.formatter
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Use On Type Format",
            description: "Whether to use additional LSP queries to format (and amend) the code after every \"trigger\" symbol input, defined by LSP server capabilities",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.use_on_type_format
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.use_on_type_format
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Prettier"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Allowed",
            description: "Enables or disables formatting with Prettier for a given language",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(prettier) = &language.prettier {
                            &prettier.allowed
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.prettier.get_or_insert_default().allowed
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Parser",
            description: "Forces Prettier integration to use a specific parser name when formatting files with the language",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(prettier) = &language.prettier {
                            &prettier.parser
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.prettier.get_or_insert_default().parser
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Plugins",
            description: "Forces Prettier integration to use specific plugins when formatting files with the language",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            if let Some(prettier) = &language.prettier {
                                &prettier.plugins
                            } else {
                                &None
                            }
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.prettier.get_or_insert_default().plugins
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Options",
            description: "Default Prettier options, in the format as in package.json section for Prettier",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            if let Some(prettier) = &language.prettier {
                                &prettier.options
                            } else {
                                &None
                            }
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.prettier.get_or_insert_default().options
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Autoclose"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Use Autoclose",
            description: "Whether to automatically type closing characters for you. For example, when you type (, Zed will automatically add a closing ) at the correct position",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.use_autoclose)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.use_autoclose
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Use Auto Surround",
            description: "Whether to automatically surround text with characters for you. For example, when you select text and type (, Zed will automatically surround text with ()",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.use_auto_surround
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.use_auto_surround
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Always Treat Brackets As Autoclosed",
            description: "Controls whether the closing characters are always skipped over and auto-removed no matter how they were inserted",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.always_treat_brackets_as_autoclosed
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.always_treat_brackets_as_autoclosed
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Jsx Tag Auto Close",
            description: "Whether to automatically close JSX tags",
            field: Box::new(SettingField {
                // TODO(settings_ui): this setting should just be a bool
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        match language.jsx_tag_auto_close.as_ref() {
                            Some(s) => &s.enabled,
                            None => &None,
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.jsx_tag_auto_close.get_or_insert_default().enabled
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("LSP"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enable Language Server",
            description: "Whether to use language servers to provide code intelligence",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.enable_language_server
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.enable_language_server
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Language Servers",
            description: "The list of language servers to use (or disable) for this language",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            &language.language_servers
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.language_servers
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Linked Edits",
            description: "Whether to perform linked edits of associated ranges, if the LS supports it. For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.linked_edits)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.linked_edits
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Edit Predictions"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Edit Predictions",
            description: "Controls whether edit predictions are shown immediately (true) or manually by triggering `editor::ShowEditPrediction` (false)",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.show_edit_predictions
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.show_edit_predictions
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Edit Predictions Disabled In",
            description: "Controls whether edit predictions are shown in the given language scopes",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            &language.edit_predictions_disabled_in
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.edit_predictions_disabled_in
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Whitespace"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Whitespaces",
            description: "Whether to show tabs and spaces in the editor",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| &language.show_whitespaces)
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.show_whitespaces
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Space Whitespace Indicator",
            description: "Visible character used to render space characters when show_whitespaces is enabled (default: \"•\")",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            if let Some(whitespace_map) = &language.whitespace_map {
                                &whitespace_map.space
                            } else {
                                &None
                            }
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.whitespace_map.get_or_insert_default().space
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Tab Whitespace Indicator",
            description: "Visible character used to render tab characters when show_whitespaces is enabled (default: \"→\")",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            if let Some(whitespace_map) = &language.whitespace_map {
                                &whitespace_map.tab
                            } else {
                                &None
                            }
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.whitespace_map.get_or_insert_default().tab
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Completions"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Completions On Input",
            description: "Whether to pop the completions menu while typing in an editor without explicitly requesting it",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.show_completions_on_input
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.show_completions_on_input
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Completion Documentation",
            description: "Whether to display inline and alongside documentation for items in the completions menu",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.show_completion_documentation
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.show_completion_documentation
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Words",
            description: "Controls how words are completed",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(completions) = &language.completions {
                            &completions.words
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.completions.get_or_insert_default().words
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Words Min Length",
            description: "How many characters has to be in the completions query to automatically show the words-based completions",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(completions) = &language.completions {
                            &completions.words_min_length
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .completions
                            .get_or_insert_default()
                            .words_min_length
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Lsp",
            description: "Whether to fetch LSP completions or not",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(completions) = &language.completions {
                            &completions.lsp
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.completions.get_or_insert_default().lsp
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Lsp Fetch Timeout Ms",
            description: "When fetching LSP completions, determines how long to wait for a response of a particular server (set to 0 to wait indefinitely)",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(completions) = &language.completions {
                            &completions.lsp_fetch_timeout_ms
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .completions
                            .get_or_insert_default()
                            .lsp_fetch_timeout_ms
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Lsp Insert Mode",
            description: "Controls how LSP completions are inserted",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(completions) = &language.completions {
                            &completions.lsp_insert_mode
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.completions.get_or_insert_default().lsp_insert_mode
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Inlay Hints"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enabled",
            description: "Global switch to toggle hints on and off",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.enabled
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.inlay_hints.get_or_insert_default().enabled
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Value Hints",
            description: "Global switch to toggle inline values on and off when debugging",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.show_value_hints
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .inlay_hints
                            .get_or_insert_default()
                            .show_value_hints
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Type Hints",
            description: "Whether type hints should be shown",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.show_type_hints
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.inlay_hints.get_or_insert_default().show_type_hints
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Parameter Hints",
            description: "Whether parameter hints should be shown",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.show_parameter_hints
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .inlay_hints
                            .get_or_insert_default()
                            .show_parameter_hints
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Other Hints",
            description: "Whether other hints should be shown",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.show_other_hints
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .inlay_hints
                            .get_or_insert_default()
                            .show_other_hints
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Background",
            description: "Whether to show a background for inlay hints",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.show_background
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.inlay_hints.get_or_insert_default().show_background
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Edit Debounce Ms",
            description: "Whether or not to debounce inlay hints updates after buffer edits (set to 0 to disable debouncing)",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.edit_debounce_ms
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .inlay_hints
                            .get_or_insert_default()
                            .edit_debounce_ms
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Scroll Debounce Ms",
            description: "Whether or not to debounce inlay hints updates after buffer scrolls (set to 0 to disable debouncing)",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(inlay_hints) = &language.inlay_hints {
                            &inlay_hints.scroll_debounce_ms
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language
                            .inlay_hints
                            .get_or_insert_default()
                            .scroll_debounce_ms
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Toggle On Modifiers Press",
            description: "Toggles inlay hints (hides or shows) when the user presses the modifiers specified",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            if let Some(inlay_hints) = &language.inlay_hints {
                                &inlay_hints.toggle_on_modifiers_press
                            } else {
                                &None
                            }
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language
                                .inlay_hints
                                .get_or_insert_default()
                                .toggle_on_modifiers_press
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Tasks"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enabled",
            description: "Whether tasks are enabled for this language",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(tasks) = &language.tasks {
                            &tasks.enabled
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.tasks.get_or_insert_default().enabled
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Variables",
            description: "Extra task variables to set for a particular language",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            if let Some(tasks) = &language.tasks {
                                &tasks.variables
                            } else {
                                &None
                            }
                        })
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.tasks.get_or_insert_default().variables
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Prefer Lsp",
            description: "Use LSP tasks over Zed language extension ones",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        if let Some(tasks) = &language.tasks {
                            &tasks.prefer_lsp
                        } else {
                            &None
                        }
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.tasks.get_or_insert_default().prefer_lsp
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Miscellaneous"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Debuggers",
            description: "Preferred debuggers for this language",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| &language.debuggers)
                    },
                    pick_mut: |settings_content| {
                        language_settings_field_mut(settings_content, |language| {
                            &mut language.debuggers
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Extend Comment On Newline",
            description: "Whether to start a new line with a comment when a previous line is a comment as well",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        &language.extend_comment_on_newline
                    })
                },
                pick_mut: |settings_content| {
                    language_settings_field_mut(settings_content, |language| {
                        &mut language.extend_comment_on_newline
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
    ]
}
