use gpui::App;
use settings::{LanguageSettingsContent, SettingsContent};
use std::sync::Arc;
use strum::IntoDiscriminant as _;
use ui::{IntoElement, SharedString};

use crate::{
    DynamicItem, LOCAL, SettingField, SettingItem, SettingsFieldMetadata, SettingsPage,
    SettingsPageItem, SubPageLink, USER, all_language_names, sub_page_stack,
};

pub(crate) fn settings_data(cx: &App) -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "General",
            items: vec![
                SettingsPageItem::SectionHeader("General Settings"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Confirm Quit",
                    description: "Confirm before quitting Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.workspace.confirm_quit.as_ref(),
                        write: |settings_content, value| {
                            settings_content.workspace.confirm_quit = value;
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
                            settings_content
                                .workspace
                                .when_closing_with_no_tabs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.when_closing_with_no_tabs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "On Last Window Closed",
                    description: "What to do when the last window is closed",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.on_last_window_closed.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.on_last_window_closed = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Path Prompts",
                    description: "Use native OS dialogs for 'Open' and 'Save As'",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.use_system_path_prompts.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.use_system_path_prompts = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Prompts",
                    description: "Use native OS dialogs for confirmations",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.use_system_prompts.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.use_system_prompts = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Redact Private Values",
                    description: "Hide the values of variables in private files",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.redact_private_values.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.redact_private_values = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Private Files",
                    description: "Globs to match against file paths to determine if a file is private",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content.project.worktree.private_files.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.worktree.private_files = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Workspace Restoration"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Restore Unsaved Buffers",
                    description: "Whether or not to restore unsaved buffers on restart",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .session
                                .as_ref()
                                .and_then(|session| session.restore_unsaved_buffers.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content
                                .session
                                .get_or_insert_default()
                                .restore_unsaved_buffers = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Restore On Startup",
                    description: "What to restore from the previous session when opening Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.restore_on_startup.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.restore_on_startup = value;
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
                            pick: |settings_content| {
                                settings_content.workspace.use_system_prompts.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.workspace.use_system_prompts = value;
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
                            pick: |settings_content| {
                                settings_content.workspace.use_system_prompts.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.workspace.use_system_prompts = value;
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
                            settings_content
                                .telemetry
                                .as_ref()
                                .and_then(|telemetry| telemetry.diagnostics.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content
                                .telemetry
                                .get_or_insert_default()
                                .diagnostics = value;
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
                            settings_content
                                .telemetry
                                .as_ref()
                                .and_then(|telemetry| telemetry.metrics.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content.telemetry.get_or_insert_default().metrics = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Auto Update"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Update",
                    description: "Whether or not to automatically check for updates",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.auto_update.as_ref(),
                        write: |settings_content, value| {
                            settings_content.auto_update = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Appearance",
            items: vec![
                SettingsPageItem::SectionHeader("Theme"),
                SettingsPageItem::DynamicItem(DynamicItem {
                    discriminant: SettingItem {
                        files: USER,
                        title: "Theme Mode",
                        description: "How to select the theme",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                Some(&<<settings::ThemeSelection as strum::IntoDiscriminant>::Discriminant as strum::VariantArray>::VARIANTS[
                                    settings_content
                                        .theme
                                        .theme
                                        .as_ref()?
                                        .discriminant() as usize])
                            },
                            write: |settings_content, value| {
                                let Some(value) = value else {
                                    return;
                                };
                                let settings_value = settings_content.theme.theme.as_mut().expect("Has Default");
                                *settings_value = match value {
                                    settings::ThemeSelectionDiscriminants::Static => {
                                        let name = match settings_value {
                                            settings::ThemeSelection::Static(_) => return,
                                            settings::ThemeSelection::Dynamic { mode, light, dark } => {
                                                match mode {
                                                    theme::ThemeMode::Light => light.clone(),
                                                    theme::ThemeMode::Dark => dark.clone(),
                                                    theme::ThemeMode::System => dark.clone(), // no cx, can't determine correct choice
                                                }
                                            },
                                        };
                                        settings::ThemeSelection::Static(name)
                                    },
                                    settings::ThemeSelectionDiscriminants::Dynamic => {
                                        let static_name = match settings_value {
                                            settings::ThemeSelection::Static(theme_name) => theme_name.clone(),
                                            settings::ThemeSelection::Dynamic {..} => return,
                                        };

                                        settings::ThemeSelection::Dynamic {
                                            mode: settings::ThemeMode::System,
                                            light: static_name.clone(),
                                            dark: static_name,
                                        }
                                    },
                                };
                            },
                        }),
                        metadata: None,
                    },
                    pick_discriminant: |settings_content| {
                        Some(settings_content.theme.theme.as_ref()?.discriminant() as usize)
                    },
                    fields: <<settings::ThemeSelection as strum::IntoDiscriminant>::Discriminant as strum::VariantArray>::VARIANTS.into_iter().map(|variant| {
                        match variant {
                            settings::ThemeSelectionDiscriminants::Static => vec![
                                SettingItem {
                                    files: USER,
                                    title: "Theme Name",
                                    description: "The Name Of The Theme To Use",
                                    field: Box::new(SettingField {
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Static(name)) => Some(name),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            match settings_content
                                                .theme
                                                .theme.as_mut() {
                                                    Some(settings::ThemeSelection::Static(theme_name)) => *theme_name = value,
                                                    _ => return
                                                }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                            settings::ThemeSelectionDiscriminants::Dynamic => vec![
                                SettingItem {
                                    files: USER,
                                    title: "Mode",
                                    description: "How To Determine Whether to Use a Light or Dark Theme",
                                    field: Box::new(SettingField {
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Dynamic { mode, ..}) => Some(mode),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            match settings_content
                                                .theme
                                                .theme.as_mut() {
                                                    Some(settings::ThemeSelection::Dynamic{ mode, ..}) => *mode = value,
                                                    _ => return
                                                }
                                        },
                                    }),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER,
                                    title: "Light Theme",
                                    description: "The Theme To Use When Mode Is Set To Light, Or When Mode Is Set To System And The System Is In Light Mode",
                                    field: Box::new(SettingField {
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Dynamic { light, ..}) => Some(light),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            match settings_content
                                                .theme
                                                .theme.as_mut() {
                                                    Some(settings::ThemeSelection::Dynamic{ light, ..}) => *light = value,
                                                    _ => return
                                                }
                                        },
                                    }),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER,
                                    title: "Dark Theme",
                                    description: "The Theme To Use When Mode Is Set To Dark, Or When Mode Is Set To System And The System Is In Dark Mode",
                                    field: Box::new(SettingField {
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Dynamic { dark, ..}) => Some(dark),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            match settings_content
                                                .theme
                                                .theme.as_mut() {
                                                    Some(settings::ThemeSelection::Dynamic{ dark, ..}) => *dark = value,
                                                    _ => return
                                                }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                        }
                    }).collect(),
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Icon Theme",
                    // todo(settings_ui)
                    // This description is misleading because the icon theme is used in more places than the file explorer)
                    description: "Choose the icon theme for file explorer",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| settings_content.theme.icon_theme.as_ref(),
                            write: |settings_content, value|{  settings_content.theme.icon_theme = value;},
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Buffer Font"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Family",
                    description: "Font family for editor text",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.theme.buffer_font_family.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.buffer_font_family = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Size",
                    description: "Font size for editor text",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.theme.buffer_font_size.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.buffer_font_size = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Weight",
                    description: "Font weight for editor text (100-900)",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.theme.buffer_font_weight.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.buffer_font_weight = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                // todo(settings_ui): This needs custom ui
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Line Height",
                    description: "Line height for editor text",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content.theme.buffer_line_height.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.buffer_line_height = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Font Features",
                    description: "The OpenType features to enable for rendering in text buffers.",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content.theme.buffer_font_features.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.buffer_font_features = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Font Fallbacks",
                    description: "The font fallbacks to use for rendering in text buffers.",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content.theme.buffer_font_fallbacks.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.buffer_font_fallbacks = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("UI Font"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Family",
                    description: "Font family for UI elements",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.theme.ui_font_family.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.ui_font_family = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Size",
                    description: "Font size for UI elements",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.theme.ui_font_size.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.ui_font_size = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Font Weight",
                    description: "Font weight for UI elements (100-900)",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.theme.ui_font_weight.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.ui_font_weight = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Font Features",
                    description: "The OpenType features to enable for rendering in UI elements.",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content.theme.ui_font_features.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.ui_font_features = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Font Fallbacks",
                    description: "The font fallbacks to use for rendering in the UI.",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content.theme.ui_font_fallbacks.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.ui_font_fallbacks = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Agent Panel Font"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "UI Font Size",
                    description: "Font size for agent response text in the agent panel. Falls back to the regular UI font size.",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .theme
                                .agent_ui_font_size
                                .as_ref()
                                .or(settings_content.theme.ui_font_size.as_ref())
                        },
                        write: |settings_content, value|{  settings_content.theme.agent_ui_font_size = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Buffer Font Size",
                    description: "Font size for user messages text in the agent panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .theme
                                .agent_buffer_font_size
                                .as_ref()
                                .or(settings_content.theme.buffer_font_size.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content.theme.agent_buffer_font_size = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Cursor"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Multi Cursor Modifier",
                    description: "Modifier key for adding multiple cursors",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.multi_cursor_modifier.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.multi_cursor_modifier = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Blink",
                    description: "Whether the cursor blinks in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.editor.cursor_blink.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.cursor_blink = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Shape",
                    description: "Cursor shape for the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.editor.cursor_shape.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.cursor_shape = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hide Mouse",
                    description: "When to hide the mouse cursor",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.editor.hide_mouse.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.hide_mouse = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Highlighting"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Unnecessary Code Fade",
                    description: "How much to fade out unused code (0.0 - 0.9)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.theme.unnecessary_code_fade.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.theme.unnecessary_code_fade = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Current Line Highlight",
                    description: "How to highlight the current line",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.current_line_highlight.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.current_line_highlight = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Selection Highlight",
                    description: "Highlight all occurrences of selected text",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.selection_highlight.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.selection_highlight = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Rounded Selection",
                    description: "Whether the text selection should have rounded corners",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.editor.rounded_selection.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.rounded_selection = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Minimum Contrast For Highlights",
                    description: "The minimum APCA perceptual contrast to maintain when rendering text over highlight backgrounds",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .minimum_contrast_for_highlights
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.minimum_contrast_for_highlights = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Guides"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Wrap Guides",
                    description: "Show wrap guides (vertical rulers)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content

                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides = value;
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
                                settings_content
                                    .project
                                    .all_languages
                                    .defaults
                                    .wrap_guides
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.all_languages.defaults.wrap_guides = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | LOCAL,
                }),
            ],
        },
        SettingsPage {
            title: "Keymap",
            items: vec![
                SettingsPageItem::SectionHeader("Base Keymap"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Base Keymap",
                    description: "The name of a base set of key bindings to use",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.base_keymap.as_ref(),
                        write: |settings_content, value| {
                            settings_content.base_keymap = value;
                        },
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        should_do_titlecase: Some(false),
                        ..Default::default()
                    })),
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Modal Editing"),
                // todo(settings_ui): Vim/Helix Mode should be apart of one type because it's undefined
                // behavior to have them both enabled at the same time
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Vim Mode",
                    description: "Enable vim modes and key bindings",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.vim_mode.as_ref(),
                        write: |settings_content, value| {
                            settings_content.vim_mode = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Helix Mode",
                    description: "Enable helix modes and key bindings",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.helix_mode.as_ref(),
                        write: |settings_content, value| {
                            settings_content.helix_mode = value;
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
                    SettingsPageItem::SectionHeader("Auto Save"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Auto Save Mode",
                        description: "When to Auto Save Buffer Changes",
                        field: Box::new(
                            SettingField {
                                pick: |settings_content| {
                                    settings_content.workspace.autosave.as_ref()
                                },
                                write: |settings_content, value| {
                                    settings_content.workspace.autosave = value;
                                },
                            }
                            .unimplemented(),
                        ),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Multibuffer"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Double Click In Multibuffer",
                        description: "What to do when multibuffer is double-clicked in some of its excerpts",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.double_click_in_multibuffer.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.double_click_in_multibuffer = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Expand Excerpt Lines",
                        description: "How many lines to expand the multibuffer excerpts by default",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.expand_excerpt_lines.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.expand_excerpt_lines = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Excerpt Context Lines",
                        description: "How many lines of context to provide in multibuffer excerpts by default",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.excerpt_context_lines.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.excerpt_context_lines = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Expand Outlines With Depth",
                        description: "Default depth to expand outline items in the current file",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .outline_panel
                                    .as_ref()
                                    .and_then(|outline_panel| {
                                        outline_panel.expand_outlines_with_depth.as_ref()
                                    })
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .outline_panel
                                    .get_or_insert_default()
                                    .expand_outlines_with_depth = value;
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
                                settings_content.editor.scroll_beyond_last_line.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.scroll_beyond_last_line = value;
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
                                settings_content.editor.vertical_scroll_margin.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.vertical_scroll_margin = value;
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
                                settings_content.editor.horizontal_scroll_margin.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.horizontal_scroll_margin = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Scroll Sensitivity",
                        description: "Scroll sensitivity multiplier for both horizontal and vertical scrolling",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.scroll_sensitivity.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.scroll_sensitivity = value;
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
                                settings_content.editor.fast_scroll_sensitivity.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.fast_scroll_sensitivity = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Autoscroll On Clicks",
                        description: "Whether to scroll when clicking near the edge of the visible text area",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.autoscroll_on_clicks.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.autoscroll_on_clicks = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Signature Help"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Auto Signature Help",
                        description: "Automatically show a signature help pop-up",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.auto_signature_help.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.auto_signature_help = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Show Signature Help After Edits",
                        description: "Show the signature help pop-up after completions or bracket pairs are inserted",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .show_signature_help_after_edits
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.show_signature_help_after_edits = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Snippet Sort Order",
                        description: "Determines how snippets are sorted relative to other completion items",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.snippet_sort_order.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.snippet_sort_order = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Hover Popover"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Enabled",
                        description: "Show the informational hover box when moving the mouse over symbols in the editor",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.hover_popover_enabled.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.hover_popover_enabled = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    // todo(settings ui): add units to this number input
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Delay",
                        description: "Time to wait in milliseconds before showing the informational hover box",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.hover_popover_delay.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.hover_popover_delay = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Drag And Drop Selection"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Enabled",
                        description: "Enable drag and drop selection",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .as_ref()
                                    .and_then(|drag_and_drop| drag_and_drop.enabled.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .get_or_insert_default()
                                    .enabled = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Delay",
                        description: "Delay in milliseconds before drag and drop selection starts",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .as_ref()
                                    .and_then(|drag_and_drop| drag_and_drop.delay.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .get_or_insert_default()
                                    .delay = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Gutter"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Show Line Numbers",
                        description: "Show line numbers in the gutter",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.line_numbers.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .line_numbers = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Relative Line Numbers",
                        description: "Whether the line numbers in the editor's gutter are relative or not",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.relative_line_numbers.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.relative_line_numbers = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Show Runnables",
                        description: "Show runnable buttons in the gutter",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.runnables.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .runnables = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Show Breakpoints",
                        description: "Show breakpoints in the gutter",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.breakpoints.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .breakpoints = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Show Folds",
                        description: "Show code folding controls in the gutter",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.folds.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content.editor.gutter.get_or_insert_default().folds =
                                    value;
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
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.min_line_number_digits.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .min_line_number_digits = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Inline Code Actions",
                        description: "Show code action button at start of buffer line",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.inline_code_actions.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.inline_code_actions = value;
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
                                settings_content.editor.scrollbar.as_ref()?.show.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .show = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Cursors",
                        description: "Show cursor positions in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.editor.scrollbar.as_ref()?.cursors.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .cursors = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Git Diff",
                        description: "Show git diff indicators in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .git_diff
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .git_diff = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Search Results",
                        description: "Show buffer search result indicators in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .search_results
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .search_results = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Selected Text",
                        description: "Show selected text occurrences in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .selected_text
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .selected_text = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Selected Symbol",
                        description: "Show selected symbol occurrences in the scrollbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .selected_symbol
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .selected_symbol = value;
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
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .diagnostics
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .diagnostics = value;
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
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .axes
                                    .as_ref()?
                                    .horizontal
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .axes
                                    .get_or_insert_default()
                                    .horizontal = value;
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
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .axes
                                    .as_ref()?
                                    .vertical
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .axes
                                    .get_or_insert_default()
                                    .vertical = value;
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
                                settings_content.editor.minimap.as_ref()?.show.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.minimap.get_or_insert_default().show =
                                    value;
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
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()?
                                    .display_in
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .display_in = value;
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
                                settings_content.editor.minimap.as_ref()?.thumb.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .thumb = value;
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
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()?
                                    .thumb_border
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .thumb_border = value;
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
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()
                                    .and_then(|minimap| minimap.current_line_highlight.as_ref())
                                    .or(settings_content.editor.current_line_highlight.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .current_line_highlight = value;
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
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()?
                                    .max_width_columns
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .max_width_columns = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Toolbar"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Breadcrumbs",
                        description: "Show breadcrumbs",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .breadcrumbs
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .breadcrumbs = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Quick Actions",
                        description: "Show quick action buttons (e.g., search, selection, editor controls, etc.)",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .quick_actions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .quick_actions = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Selections Menu",
                        description: "Show the selections menu in the editor toolbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .selections_menu
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .selections_menu = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Agent Review",
                        description: "Show agent review buttons in the editor toolbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .agent_review
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .agent_review = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Code Actions",
                        description: "Show code action buttons in the editor toolbar",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .code_actions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .code_actions = value;
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
            title: "Languages & Tools",
            items: {
                let mut items = vec![];
                items.extend(non_editor_language_settings_data());
                items.extend([
                    SettingsPageItem::SectionHeader("File Types"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "File Type Associations",
                        description: "A Mapping from Languages to files and file extensions that should be treated as that language",
                        field: Box::new(
                            SettingField {
                                pick: |settings_content| {
                                    settings_content.project.all_languages.file_types.as_ref()
                                },
                                write: |settings_content, value| {
                                    settings_content.project.all_languages.file_types = value;

                                },
                            }
                            .unimplemented(),
                        ),
                        metadata: None,
                        files: USER | LOCAL,
                    }),
                ]);

                items.extend([
                    SettingsPageItem::SectionHeader("Diagnostics"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Max Severity",
                        description: "Which level to use to filter out diagnostics displayed in the editor",
                        field: Box::new(SettingField {
                            pick: |settings_content| settings_content.editor.diagnostics_max_severity.as_ref(),
                            write: |settings_content, value| {
                                settings_content.editor.diagnostics_max_severity = value;

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
                                settings_content.diagnostics.as_ref()?.include_warnings.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .include_warnings
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("Inline Diagnostics"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Enabled",
                        description: "Whether to show diagnostics inline or not",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.enabled.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .enabled
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Update Debounce",
                        description: "The delay in milliseconds to show inline diagnostics after the last diagnostic update",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.update_debounce_ms.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .update_debounce_ms
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Padding",
                        description: "The amount of padding between the end of the source line and the start of the inline diagnostic",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.padding.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .padding
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Minimum Column",
                        description: "The minimum column at which to display inline diagnostics",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.min_column.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .min_column
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("LSP Pull Diagnostics"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Enabled",
                        description: "Whether to pull for language server-powered diagnostics or not",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.lsp_pull_diagnostics.as_ref()?.enabled.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .lsp_pull_diagnostics
                                    .get_or_insert_default()
                                    .enabled
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    // todo(settings_ui): Needs unit
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Debounce",
                        description: "Minimum time to wait before pulling diagnostics from the language server(s)",
                        field: Box::new(SettingField {
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.lsp_pull_diagnostics.as_ref()?.debounce_ms.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .lsp_pull_diagnostics
                                    .get_or_insert_default()
                                    .debounce_ms
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("LSP Highlights"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Debounce",
                        description: "The debounce delay before querying highlights from the language",
                        field: Box::new(SettingField {
                            pick: |settings_content| settings_content.editor.lsp_highlight_debounce.as_ref(),
                            write: |settings_content, value| {
                                settings_content.editor.lsp_highlight_debounce = value;

                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                ]);

                // todo(settings_ui): Refresh on extension (un)/installed
                // Note that `crates/json_schema_store` solves the same problem, there is probably a way to unify the two
                items.push(SettingsPageItem::SectionHeader(LANGUAGES_SECTION_HEADER));
                items.extend(all_language_names(cx).into_iter().map(|language_name| {
                    SettingsPageItem::SubPageLink(SubPageLink {
                        title: language_name,
                        files: USER | LOCAL,
                        render: Arc::new(|this, window, cx| {
                            this.render_sub_page_items(
                                language_settings_data()
                                    .iter()
                                    .chain(non_editor_language_settings_data().iter())
                                    .enumerate(),
                                None,
                                window,
                                cx,
                            )
                            .into_any_element()
                        }),
                    })
                }));
                items
            },
        },
        SettingsPage {
            title: "Search & Files",
            items: vec![
                SettingsPageItem::SectionHeader("Search"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Whole Word",
                    description: "Search for whole words by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.search.as_ref()?.whole_word.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .whole_word = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Case Sensitive",
                    description: "Search case-sensitively by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .search
                                .as_ref()?
                                .case_sensitive
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .case_sensitive = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use Smartcase Search",
                    description: "Whether to automatically enable case-sensitive search based on the search query",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.use_smartcase_search.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.use_smartcase_search = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Include Ignored",
                    description: "Include ignored files in search results by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .search
                                .as_ref()?
                                .include_ignored
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .include_ignored = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Regex",
                    description: "Use regex search by default",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.search.as_ref()?.regex.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.search.get_or_insert_default().regex = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Search Wrap",
                    description: "Whether the editor search results will loop",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.editor.search_wrap.as_ref(),
                        write: |settings_content, value| {
                            settings_content.editor.search_wrap = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Seed Search Query From Cursor",
                    description: "When to populate a new search's query based on the text under the cursor",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .seed_search_query_from_cursor
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.seed_search_query_from_cursor = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("File Finder"),
                // todo: null by default
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Include Ignored in Search",
                    description: "Use gitignored files when searching",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .file_finder
                                    .as_ref()?
                                    .include_ignored
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .file_finder
                                    .get_or_insert_default()
                                    .include_ignored = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Icons",
                    description: "Show file icons in the file finder",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.file_finder.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .file_icons = value;
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
                            settings_content
                                .file_finder
                                .as_ref()?
                                .modal_max_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .modal_max_width = value;
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
                            settings_content
                                .file_finder
                                .as_ref()?
                                .skip_focus_for_active_in_search
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .skip_focus_for_active_in_search = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Status",
                    description: "Show the git status in the file finder",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.file_finder.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .git_status = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("File Scan"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Scan Exclusions",
                    description: "Files or globs of files that will be excluded by Zed entirely. They will be skipped during file scans, file searches, and not be displayed in the project file tree. Takes precedence over \"File Scan Inclusions\"",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .project
                                    .worktree
                                    .file_scan_exclusions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.worktree.file_scan_exclusions = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Scan Inclusions",
                    description: "Files or globs of files that will be included by Zed, even when ignored by git. This is useful for files that are not tracked by git, but are still important to your project. Note that globs that are overly broad can slow down Zed's file scanning. \"File Scan Exclusions\" takes precedence over these inclusions",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .project
                                    .worktree
                                    .file_scan_exclusions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.worktree.file_scan_exclusions = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Restore File State",
                    description: "Restore previous file state when reopening",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.restore_on_file_reopen.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.restore_on_file_reopen = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Close on File Delete",
                    description: "Automatically close files that have been deleted",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.close_on_file_delete.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.close_on_file_delete = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Window & Layout",
            items: vec![
                SettingsPageItem::SectionHeader("Status Bar"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Panel Button",
                    description: "Show the project panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Active Language Button",
                    description: "Show the active language button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .active_language_button
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .active_language_button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Position Button",
                    description: "Show the cursor position button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .cursor_position_button
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .cursor_position_button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Terminal Button",
                    description: "Show the terminal button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Diagnostics Button",
                    description: "Show the project diagnostics button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.diagnostics.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.diagnostics.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Search Button",
                    description: "Show the project search button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.editor.search.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Debugger Button",
                    description: "Show the debugger button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.debugger.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.debugger.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Title Bar"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Branch Icon",
                    description: "Show the branch icon beside branch switcher in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_branch_icon
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_branch_icon = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Branch Name",
                    description: "Show the branch name button in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_branch_name
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_branch_name = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Project Items",
                    description: "Show the project host and name in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_project_items
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_project_items = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Onboarding Banner",
                    description: "Show banners announcing new features in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_onboarding_banner
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_onboarding_banner = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show User Picture",
                    description: "Show user picture in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_user_picture
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_user_picture = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Sign In",
                    description: "Show the sign in button in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.title_bar.as_ref()?.show_sign_in.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_sign_in = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Menus",
                    description: "Show the menus in the titlebar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.title_bar.as_ref()?.show_menus.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_menus = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Tab Bar"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Tab Bar",
                    description: "Show the tab bar in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.tab_bar.as_ref()?.show.as_ref(),
                        write: |settings_content, value| {
                            settings_content.tab_bar.get_or_insert_default().show = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Git Status In Tabs",
                    description: "Show the Git file status on a tab item",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.tabs.get_or_insert_default().git_status = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show File Icons In Tabs",
                    description: "Show the file icon for a tab",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.tabs.get_or_insert_default().file_icons = value;
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
                            settings_content.tabs.as_ref()?.close_position.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.tabs.get_or_insert_default().close_position = value;
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
                            pick: |settings_content| settings_content.workspace.max_tabs.as_ref(),
                            write: |settings_content, value| {
                                settings_content.workspace.max_tabs = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Navigation History Buttons",
                    description: "Show the navigation history buttons in the tab bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .tab_bar
                                .as_ref()?
                                .show_nav_history_buttons
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tab_bar
                                .get_or_insert_default()
                                .show_nav_history_buttons = value;
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
                            settings_content.tabs.as_ref()?.activate_on_close.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tabs
                                .get_or_insert_default()
                                .activate_on_close = value;
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
                            settings_content.tabs.as_ref()?.show_diagnostics.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tabs
                                .get_or_insert_default()
                                .show_diagnostics = value;
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
                            settings_content.tabs.as_ref()?.show_close_button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tabs
                                .get_or_insert_default()
                                .show_close_button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Preview Tabs"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Preview Tabs Enabled",
                    description: "Show opened editors as preview tabs",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.preview_tabs.as_ref()?.enabled.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enabled = value;
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
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_preview_from_file_finder
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_from_file_finder = value;
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
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_preview_from_code_navigation
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_from_code_navigation = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Layout"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Bottom Dock Layout",
                    description: "Layout mode for the bottom dock",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.bottom_dock_layout.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.bottom_dock_layout = value;
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
                            settings_content
                                .workspace
                                .centered_layout
                                .as_ref()?
                                .left_padding
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .centered_layout
                                .get_or_insert_default()
                                .left_padding = value;
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
                            settings_content
                                .workspace
                                .centered_layout
                                .as_ref()?
                                .right_padding
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .centered_layout
                                .get_or_insert_default()
                                .right_padding = value;
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Window"),
                // todo(settings_ui): Should we filter by platform.as_ref()?
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use System Window Tabs",
                    description: "(macOS only) Whether to allow windows to tab together",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.workspace.use_system_window_tabs.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.use_system_window_tabs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Pane Modifiers"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Inactive Opacity",
                    description: "Opacity of inactive panels (0.0 - 1.0)",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .as_ref()?
                                .inactive_opacity
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .get_or_insert_default()
                                .inactive_opacity = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Border Size",
                    description: "Size of the border surrounding the active pane",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .as_ref()?
                                .border_size
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .get_or_insert_default()
                                .border_size = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Zoomed Padding",
                    description: "Show padding for zoomed panes",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.workspace.zoomed_padding.as_ref(),
                        write: |settings_content, value| {
                            settings_content.workspace.zoomed_padding = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Pane Split Direction"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Vertical Split Direction",
                    description: "Direction to split vertically",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .pane_split_direction_vertical
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.pane_split_direction_vertical = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Horizontal Split Direction",
                    description: "Direction to split horizontally",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .pane_split_direction_horizontal
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.pane_split_direction_horizontal = value;
                        },
                    }),
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
                            settings_content.project_panel.as_ref()?.dock.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project_panel.get_or_insert_default().dock = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .default_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .default_width = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .hide_gitignore
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_gitignore = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .entry_spacing
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .entry_spacing = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Icons",
                    description: "Show file icons in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .file_icons = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .folder_icons
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .folder_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Status",
                    description: "Show the git status in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .git_status = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .indent_size
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .indent_size = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Reveal Entries",
                    description: "Whether to reveal entries in the project panel automatically when a corresponding project entry becomes active",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .auto_reveal_entries
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .auto_reveal_entries = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .starts_open
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .starts_open = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .auto_fold_dirs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .auto_fold_dirs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Scrollbar",
                    description: "Show the scrollbar in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            show_scrollbar_or_editor(settings_content, |settings_content| {
                                settings_content
                                    .project_panel
                                    .as_ref()?
                                    .scrollbar
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            })
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .show_diagnostics
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .show_diagnostics = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .sticky_scroll
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .sticky_scroll = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "Indent Guides Show",
                    description: "Show indent guides in the project panel",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| {
                                settings_content
                                    .project_panel
                                    .as_ref()?
                                    .indent_guides
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .project_panel
                                    .get_or_insert_default()
                                    .indent_guides
                                    .get_or_insert_default()
                                    .show = value;
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
                            settings_content
                                .project_panel
                                .as_ref()?
                                .drag_and_drop
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .drag_and_drop = value;
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
                            settings_content.project_panel.as_ref()?.hide_root.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_root = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hide Hidden",
                    description: "Whether to hide the hidden entries in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .hide_hidden
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_hidden = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Open File on Paste",
                    description: "Whether to automatically open files when pasting them in the project panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .open_file_on_paste
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .open_file_on_paste = value;
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
                        pick: |settings_content| settings_content.terminal.as_ref()?.dock.as_ref(),
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Outline Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Outline Panel Button",
                    description: "Show the outline panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.outline_panel.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .button = value;
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
                            settings_content.outline_panel.as_ref()?.dock.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.outline_panel.get_or_insert_default().dock = value;
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
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .default_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "File Icons",
                    description: "Show file icons in the outline panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.outline_panel.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .file_icons = value;
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
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .folder_icons
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .folder_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Status",
                    description: "Show the git status in the outline panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.outline_panel.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .git_status = value;
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
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .indent_size
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .indent_size = value;
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
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .auto_reveal_entries
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .auto_reveal_entries = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Fold Directories",
                    description: "Whether to fold directories automatically when a directory contains only one subdirectory",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .auto_fold_dirs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .auto_fold_dirs = value;
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
                                settings_content
                                    .outline_panel
                                    .as_ref()?
                                    .indent_guides
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .outline_panel
                                    .get_or_insert_default()
                                    .indent_guides
                                    .get_or_insert_default()
                                    .show = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Git Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Panel Button",
                    description: "Show the Git panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.git_panel.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Panel Dock",
                    description: "Where to dock the Git panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.git_panel.as_ref()?.dock.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git_panel.get_or_insert_default().dock = value;
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
                            settings_content.git_panel.as_ref()?.default_width.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Git Panel Status Style",
                    description: "How entry statuses are displayed",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.status_style.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .status_style = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Fallback Branch Name",
                    description: "Default branch name will be when init.defaultBranch is not set in git",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git_panel
                                .as_ref()?
                                .fallback_branch_name
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .fallback_branch_name = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Sort By Path",
                    description: "Enable to sort entries in the panel by path, disable to sort by status",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.sort_by_path.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .sort_by_path = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Collapse Untracked Diff",
                    description: "Whether to collapse untracked files in the diff panel",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git_panel
                                .as_ref()?
                                .collapse_untracked_diff
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .collapse_untracked_diff = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Scroll Bar",
                    description: "How and when the scrollbar should be displayed",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            show_scrollbar_or_editor(settings_content, |settings_content| {
                                settings_content
                                    .git_panel
                                    .as_ref()?
                                    .scrollbar
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            })
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show = value;
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
                        pick: |settings_content| settings_content.debugger.as_ref()?.dock.as_ref(),
                        write: |settings_content, value| {
                            settings_content.debugger.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Notification Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Notification Panel Button",
                    description: "Show the notification panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .notification_panel
                                .as_ref()?
                                .button
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .notification_panel
                                .get_or_insert_default()
                                .button = value;
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
                            settings_content.notification_panel.as_ref()?.dock.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .notification_panel
                                .get_or_insert_default()
                                .dock = value;
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
                            settings_content
                                .notification_panel
                                .as_ref()?
                                .default_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .notification_panel
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Collaboration Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Collaboration Panel Button",
                    description: "Show the collaboration panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .collaboration_panel
                                .as_ref()?
                                .button
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .collaboration_panel
                                .get_or_insert_default()
                                .button = value;
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
                            settings_content.collaboration_panel.as_ref()?.dock.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .collaboration_panel
                                .get_or_insert_default()
                                .dock = value;
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
                            settings_content
                                .collaboration_panel
                                .as_ref()?
                                .default_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .collaboration_panel
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Agent Panel"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Agent Panel Button",
                    description: "Whether to show the agent panel button in the status bar",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.agent.as_ref()?.button.as_ref(),
                        write: |settings_content, value| {
                            settings_content.agent.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Agent Panel Dock",
                    description: "Where to dock the agent panel.",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.agent.as_ref()?.dock.as_ref(),
                        write: |settings_content, value| {
                            settings_content.agent.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Agent Panel Default Width",
                    description: "Default width when the agent panel is docked to the left or right",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.agent.as_ref()?.default_width.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.agent.get_or_insert_default().default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Agent Panel Default Height",
                    description: "Default height when the agent panel is docked to the bottom",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.agent.as_ref()?.default_height.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .default_height = value;
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
                            settings_content
                                .debugger
                                .as_ref()?
                                .stepping_granularity
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .stepping_granularity = value;
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
                            settings_content
                                .debugger
                                .as_ref()?
                                .save_breakpoints
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .save_breakpoints = value;
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
                            settings_content.debugger.as_ref()?.timeout.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.debugger.get_or_insert_default().timeout = value;
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
                            settings_content
                                .debugger
                                .as_ref()?
                                .log_dap_communications
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .log_dap_communications = value;
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
                            settings_content
                                .debugger
                                .as_ref()?
                                .format_dap_log_messages
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .format_dap_log_messages = value;
                        },
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
                                settings_content.terminal.as_ref()?.project.shell.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .shell = value;
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
                                settings_content
                                    .terminal
                                    .as_ref()?
                                    .project
                                    .working_directory
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .working_directory = value;
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
                                settings_content.terminal.as_ref()?.project.env.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .env = value;
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
                                settings_content
                                    .terminal
                                    .as_ref()?
                                    .project
                                    .detect_venv
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .detect_venv = value;
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
                            settings_content
                                .terminal
                                .as_ref()
                                .and_then(|terminal| terminal.font_size.as_ref())
                                .or(settings_content.theme.buffer_font_size.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().font_size = value;
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
                            settings_content
                                .terminal
                                .as_ref()
                                .and_then(|terminal| terminal.font_family.as_ref())
                                .or(settings_content.theme.buffer_font_family.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .font_family = value;
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
                                settings_content
                                    .terminal
                                    .as_ref()
                                    .and_then(|terminal| terminal.font_fallbacks.as_ref())
                                    .or(settings_content.theme.buffer_font_fallbacks.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .font_fallbacks = value;
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
                            settings_content.terminal.as_ref()?.font_weight.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .font_weight = value;
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
                                settings_content
                                    .terminal
                                    .as_ref()
                                    .and_then(|terminal| terminal.font_features.as_ref())
                                    .or(settings_content.theme.buffer_font_features.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .font_features = value;
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
                                settings_content.terminal.as_ref()?.line_height.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .line_height = value;
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
                            settings_content.terminal.as_ref()?.cursor_shape.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .cursor_shape = value;
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
                            settings_content.terminal.as_ref()?.blinking.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().blinking = value;
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
                            settings_content
                                .terminal
                                .as_ref()?
                                .alternate_scroll
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .alternate_scroll = value;
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
                            settings_content
                                .terminal
                                .as_ref()?
                                .minimum_contrast
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .minimum_contrast = value;
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
                            settings_content.terminal.as_ref()?.option_as_meta.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .option_as_meta = value;
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
                            settings_content.terminal.as_ref()?.copy_on_select.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .copy_on_select = value;
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
                            settings_content
                                .terminal
                                .as_ref()?
                                .keep_selection_on_copy
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .keep_selection_on_copy = value;
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
                            settings_content.terminal.as_ref()?.default_width.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .default_width = value;
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
                            settings_content.terminal.as_ref()?.default_height.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .default_height = value;
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
                            settings_content
                                .terminal
                                .as_ref()?
                                .max_scroll_history_lines
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .max_scroll_history_lines = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Toolbar"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Breadcrumbs",
                    description: "Display the terminal title in breadcrumbs inside the terminal pane",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()?
                                .toolbar
                                .as_ref()?
                                .breadcrumbs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .toolbar
                                .get_or_insert_default()
                                .breadcrumbs = value;
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
                            show_scrollbar_or_editor(settings_content, |settings_content| {
                                settings_content
                                    .terminal
                                    .as_ref()?
                                    .scrollbar
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            })
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show = value;
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
                SettingsPageItem::SectionHeader("Git Gutter"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Visibilility",
                    description: "Control whether git status is shown in the editor's gutter",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.git.as_ref()?.git_gutter.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git.get_or_insert_default().git_gutter = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                // todo(settings_ui): Figure out the right default for this value in default.json
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Debounce",
                    description: "Debounce threshold in milliseconds after which changes are reflected in the git gutter",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.git.as_ref()?.gutter_debounce.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.git.get_or_insert_default().gutter_debounce = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Inline Git Blame"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Enabled",
                    description: "Whether or not to show git blame data inline in the currently focused line",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .enabled
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .enabled = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Delay",
                    description: "The delay after which the inline blame information is shown",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .delay_ms
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .delay_ms = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Padding",
                    description: "Padding between the end of the source line and the start of the inline blame in columns",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .padding
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .padding = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Minimum Column",
                    description: "The minimum column number at which to show the inline blame information",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .min_column
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .min_column = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Commit Summary",
                    description: "Show commit summary as part of the inline blame",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .show_commit_summary
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .show_commit_summary = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Git Blame View"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Avatar",
                    description: "Show the avatar of the author of the commit",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .blame
                                .as_ref()?
                                .show_avatar
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .blame
                                .get_or_insert_default()
                                .show_avatar = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Branch Picker"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Show Author Name",
                    description: "Show author name as part of the commit information in branch picker",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .branch_picker
                                .as_ref()?
                                .show_author_name
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .branch_picker
                                .get_or_insert_default()
                                .show_author_name = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Git Hunks"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Hunk Style",
                    description: "How git hunks are displayed visually in the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.git.as_ref()?.hunk_style.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git.get_or_insert_default().hunk_style = value;
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
                            settings_content.calls.as_ref()?.mute_on_join.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.calls.get_or_insert_default().mute_on_join = value;
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
                            settings_content.calls.as_ref()?.share_on_join.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.calls.get_or_insert_default().share_on_join = value;
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
                            settings_content.audio.as_ref()?.rodio_audio.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.audio.get_or_insert_default().rodio_audio = value;
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
                            settings_content
                                .audio
                                .as_ref()?
                                .auto_microphone_volume
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .audio
                                .get_or_insert_default()
                                .auto_microphone_volume = value;
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
                            settings_content
                                .audio
                                .as_ref()?
                                .auto_speaker_volume
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .audio
                                .get_or_insert_default()
                                .auto_speaker_volume = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Denoise",
                    description: "Remove background noises (requires Rodio Audio)",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.audio.as_ref()?.denoise.as_ref(),
                        write: |settings_content, value| {
                            settings_content.audio.get_or_insert_default().denoise = value;
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
                            settings_content
                                .audio
                                .as_ref()?
                                .legacy_audio_compatible
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .audio
                                .get_or_insert_default()
                                .legacy_audio_compatible = value;
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
                        pick: |settings_content| settings_content.disable_ai.as_ref(),
                        write: |settings_content, value| {
                            settings_content.disable_ai = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Agent Configuration"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Always Allow Tool Actions",
                    description: "When enabled, the agent can run potentially destructive actions without asking for your confirmation. This setting has no effect on external agents.",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .agent
                                .as_ref()?
                                .always_allow_tool_actions
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .always_allow_tool_actions = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Single File Review",
                    description: "When enabled, agent edits will also be displayed in single-file buffers for review",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.agent.as_ref()?.single_file_review.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .single_file_review = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Enable Feedback",
                    description: "Show voting thumbs up/down icon buttons for feedback on agent edits",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.agent.as_ref()?.enable_feedback.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .enable_feedback = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Notify When Agent Waiting",
                    description: "Where to show notifications when the agent has completed its response or needs confirmation before running a tool action",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .agent
                                .as_ref()?
                                .notify_when_agent_waiting
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .notify_when_agent_waiting = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Play Sound When Agent Done",
                    description: "Whether to play a sound when the agent has either completed its response, or needs user input",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .agent
                                .as_ref()?
                                .play_sound_when_agent_done
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .play_sound_when_agent_done = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Expand Edit Card",
                    description: "Whether to have edit cards in the agent panel expanded, showing a preview of the diff",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.agent.as_ref()?.expand_edit_card.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .expand_edit_card = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Expand Terminal Card",
                    description: "Whether to have terminal cards in the agent panel expanded, showing the whole command output",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .agent
                                .as_ref()?
                                .expand_terminal_card
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .expand_terminal_card = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Use Modifier To Send",
                    description: "Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .agent
                                .as_ref()?
                                .use_modifier_to_send
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .use_modifier_to_send = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Message Editor Min Lines",
                    description: "Minimum number of lines to display in the agent message editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content
                                .agent
                                .as_ref()?
                                .message_editor_min_lines
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .agent
                                .get_or_insert_default()
                                .message_editor_min_lines = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "Network",
            items: vec![
                SettingsPageItem::SectionHeader("Network"),
                // todo(settings_ui): Proxy needs a default
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Proxy",
                    description: "The proxy to use for network requests",
                    field: Box::new(
                        SettingField {
                            pick: |settings_content| settings_content.proxy.as_ref(),
                            write: |settings_content, value| {
                                settings_content.proxy = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        placeholder: Some("socks5h://localhost:10808"),
                        ..Default::default()
                    })),
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Server URL",
                    description: "The URL of the Zed server to connect to",
                    field: Box::new(SettingField {
                        pick: |settings_content| settings_content.server_url.as_ref(),
                        write: |settings_content, value| {
                            settings_content.server_url = value;
                        },
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        placeholder: Some("https://zed.dev"),
                        ..Default::default()
                    })),
                    files: USER,
                }),
            ],
        },
    ]
}

const LANGUAGES_SECTION_HEADER: &'static str = "Languages";

fn current_language() -> Option<SharedString> {
    sub_page_stack().iter().find_map(|page| {
        (page.section_header == LANGUAGES_SECTION_HEADER).then(|| page.link.title.clone())
    })
}

fn language_settings_field<T>(
    settings_content: &SettingsContent,
    get: fn(&LanguageSettingsContent) -> Option<&T>,
) -> Option<&T> {
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
    value: Option<T>,
    write: fn(&mut LanguageSettingsContent, Option<T>),
) {
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
    write(language_content, value);
}

fn language_settings_data() -> Vec<SettingsPageItem> {
    let mut items = vec![
        SettingsPageItem::SectionHeader("Indentation"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Tab Size",
            description: "How many columns a tab should occupy",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.tab_size.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.tab_size = value;
                    })
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
                    language_settings_field(settings_content, |language| {
                        language.hard_tabs.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.hard_tabs = value;
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
                    language_settings_field(settings_content, |language| {
                        language.auto_indent.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.auto_indent = value;
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
                        language.auto_indent_on_paste.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.auto_indent_on_paste = value;
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
                    language_settings_field(settings_content, |language| {
                        language.soft_wrap.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.soft_wrap = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Wrap Guides",
            description: "Show wrap guides in the editor",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.show_wrap_guides.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_wrap_guides = value;
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
                        language.preferred_line_length.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.preferred_line_length = value;
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
                        language_settings_field(settings_content, |language| {
                            language.wrap_guides.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.wrap_guides = value;
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
                    language_settings_field(settings_content, |language| {
                        language.allow_rewrap.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.allow_rewrap = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Indent Guides"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enabled",
            description: "Display indent guides in the editor",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.enabled.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().enabled = value;
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
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.line_width.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().line_width = value;
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
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.active_line_width.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .indent_guides
                            .get_or_insert_default()
                            .active_line_width = value;
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
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.coloring.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().coloring = value;
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
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.background_coloring.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .indent_guides
                            .get_or_insert_default()
                            .background_coloring = value;
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
                            language.format_on_save.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.format_on_save = value;
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
                        language.remove_trailing_whitespace_on_save.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.remove_trailing_whitespace_on_save = value;
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
                        language.ensure_final_newline_on_save.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.ensure_final_newline_on_save = value;
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
                        language_settings_field(settings_content, |language| {
                            language.formatter.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.formatter = value;
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
                        language.use_on_type_format.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.use_on_type_format = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Code Actions On Format",
            description: "Additional Code Actions To Run When Formatting",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.code_actions_on_format.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.code_actions_on_format = value;
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
                    language_settings_field(settings_content, |language| {
                        language.use_autoclose.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.use_autoclose = value;
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
                        language.use_auto_surround.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.use_auto_surround = value;
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
                        language.always_treat_brackets_as_autoclosed.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.always_treat_brackets_as_autoclosed = value;
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
                        language.jsx_tag_auto_close.as_ref()?.enabled.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.jsx_tag_auto_close.get_or_insert_default().enabled = value;
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
                        language.show_edit_predictions.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_edit_predictions = value;
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
                            language.edit_predictions_disabled_in.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.edit_predictions_disabled_in = value;
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
                    language_settings_field(settings_content, |language| {
                        language.show_whitespaces.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_whitespaces = value;
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
                            language.whitespace_map.as_ref()?.space.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.whitespace_map.get_or_insert_default().space = value;
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
                            language.whitespace_map.as_ref()?.tab.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.whitespace_map.get_or_insert_default().tab = value;
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
                        language.show_completions_on_input.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_completions_on_input = value;
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
                        language.show_completion_documentation.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_completion_documentation = value;
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
                        language.completions.as_ref()?.words.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().words = value;
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
                        language.completions.as_ref()?.words_min_length.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .completions
                            .get_or_insert_default()
                            .words_min_length = value;
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
                        language.inlay_hints.as_ref()?.enabled.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().enabled = value;
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
                        language.inlay_hints.as_ref()?.show_value_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .inlay_hints
                            .get_or_insert_default()
                            .show_value_hints = value;
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
                        language.inlay_hints.as_ref()?.show_type_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().show_type_hints = value;
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
                        language.inlay_hints.as_ref()?.show_parameter_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .inlay_hints
                            .get_or_insert_default()
                            .show_parameter_hints = value;
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
                        language.inlay_hints.as_ref()?.show_other_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .inlay_hints
                            .get_or_insert_default()
                            .show_other_hints = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Show Background",
            description: "Show a background for inlay hints",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.show_background.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().show_background = value;
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
                        language.inlay_hints.as_ref()?.edit_debounce_ms.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .inlay_hints
                            .get_or_insert_default()
                            .edit_debounce_ms = value;
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
                        language.inlay_hints.as_ref()?.scroll_debounce_ms.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .inlay_hints
                            .get_or_insert_default()
                            .scroll_debounce_ms = value;
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
                            language
                                .inlay_hints
                                .as_ref()?
                                .toggle_on_modifiers_press
                                .as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language
                                .inlay_hints
                                .get_or_insert_default()
                                .toggle_on_modifiers_press = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
    ];
    if current_language().is_none() {
        items.push(SettingsPageItem::SettingItem(SettingItem {
            title: "LSP Document Colors",
            description: "How to render LSP color previews in the editor",
            field: Box::new(SettingField {
                pick: |settings_content| settings_content.editor.lsp_document_colors.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.lsp_document_colors = value;
                },
            }),
            metadata: None,
            files: USER,
        }))
    }
    items.extend([
        SettingsPageItem::SectionHeader("Tasks"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enabled",
            description: "Whether tasks are enabled for this language",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.tasks.as_ref()?.enabled.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.tasks.get_or_insert_default().enabled = value;

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
                            language.tasks.as_ref()?.variables.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.tasks.get_or_insert_default().variables = value;

                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Prefer LSP",
            description: "Use LSP tasks over Zed language extension tasks",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.tasks.as_ref()?.prefer_lsp.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.tasks.get_or_insert_default().prefer_lsp = value;

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
                        language_settings_field(settings_content, |language| language.debuggers.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.debuggers = value;

                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Middle Click Paste",
            description: "Enable middle-click paste on Linux",
            field: Box::new(SettingField {
                pick: |settings_content| settings_content.editor.middle_click_paste.as_ref(),
                write: |settings_content, value| {settings_content.editor.middle_click_paste = value;},
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Extend Comment On Newline",
            description: "Whether to start a new line with a comment when a previous line is a comment as well",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.extend_comment_on_newline.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.extend_comment_on_newline = value;

                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
    ]);

    if current_language().is_none() {
        items.extend([
            SettingsPageItem::SettingItem(SettingItem {
                title: "Image Viewer",
                description: "The unit for image file sizes",
                field: Box::new(SettingField {
                    pick: |settings_content| {
                        settings_content.image_viewer.as_ref().and_then(|image_viewer| image_viewer.unit.as_ref())
                    },
                    write: |settings_content, value| {
                        settings_content.image_viewer.get_or_insert_default().unit = value;

                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Replace Emoji Shortcode",
                description: "Whether to automatically replace emoji shortcodes with emoji characters",
                field: Box::new(SettingField {
                    pick: |settings_content| {
                        settings_content.message_editor.as_ref().and_then(|message_editor| message_editor.auto_replace_emoji_shortcode.as_ref())
                    },
                    write: |settings_content, value| {
                        settings_content.message_editor.get_or_insert_default().auto_replace_emoji_shortcode = value;

                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Drop Size Target",
                description: "Relative size of the drop target in the editor that will open dropped file as a split pane",
                field: Box::new(SettingField {
                    pick: |settings_content| {
                        settings_content.workspace.drop_target_size.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.workspace.drop_target_size = value;

                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]);
    }
    items
}

/// LanguageSettings items that should be included in the "Languages & Tools" page
/// not the "Editor" page
fn non_editor_language_settings_data() -> Vec<SettingsPageItem> {
    vec![
        SettingsPageItem::SectionHeader("LSP"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enable Language Server",
            description: "Whether to use language servers to provide code intelligence",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.enable_language_server.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.enable_language_server = value;
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
                            language.language_servers.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.language_servers = value;
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
                    language_settings_field(settings_content, |language| {
                        language.linked_edits.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.linked_edits = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Go To Definition Fallback",
            description: "Whether to follow-up empty go to definition responses from the language server",
            field: Box::new(SettingField {
                pick: |settings_content| settings_content.editor.go_to_definition_fallback.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.go_to_definition_fallback = value;
                },
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SectionHeader("LSP Completions"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Enabled",
            description: "Whether to fetch LSP completions or not",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.completions.as_ref()?.lsp.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().lsp = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Fetch Timeout (milliseconds)",
            description: "When fetching LSP completions, determines how long to wait for a response of a particular server (set to 0 to wait indefinitely)",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.completions.as_ref()?.lsp_fetch_timeout_ms.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language
                            .completions
                            .get_or_insert_default()
                            .lsp_fetch_timeout_ms = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Insert Mode",
            description: "Controls how LSP completions are inserted",
            field: Box::new(SettingField {
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.completions.as_ref()?.lsp_insert_mode.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().lsp_insert_mode = value;
                    })
                },
            }),
            metadata: None,
            files: USER | LOCAL,
        }),
        SettingsPageItem::SectionHeader("Debuggers"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Debuggers",
            description: "Preferred debuggers for this language",
            field: Box::new(
                SettingField {
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.debuggers.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.debuggers = value;
                        })
                    },
                }
                .unimplemented(),
            ),
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
                        language.prettier.as_ref()?.allowed.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.prettier.get_or_insert_default().allowed = value;
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
                        language.prettier.as_ref()?.parser.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.prettier.get_or_insert_default().parser = value;
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
                            language.prettier.as_ref()?.plugins.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.prettier.get_or_insert_default().plugins = value;
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
                            language.prettier.as_ref()?.options.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.prettier.get_or_insert_default().options = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | LOCAL,
        }),
    ]
}

fn show_scrollbar_or_editor(
    settings_content: &SettingsContent,
    show: fn(&SettingsContent) -> Option<&settings::ShowScrollbar>,
) -> Option<&settings::ShowScrollbar> {
    show(settings_content).or(settings_content
        .editor
        .scrollbar
        .as_ref()
        .and_then(|scrollbar| scrollbar.show.as_ref()))
}
