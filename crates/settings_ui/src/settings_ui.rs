//! # settings_ui
mod components;
use editor::{Editor, EditorEvent};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use fuzzy::StringMatchCandidate;
use gpui::{
    App, AppContext as _, Context, Div, Entity, Global, IntoElement, ReadGlobal as _, Render,
    ScrollHandle, Stateful, Task, TitlebarOptions, UniformListScrollHandle, Window, WindowHandle,
    WindowOptions, actions, div, point, px, size, uniform_list,
};
use project::WorktreeId;
use settings::{
    BottomDockLayout, CloseWindowWhenNoItems, CursorShape, OnLastWindowClosed,
    RestoreOnStartupBehavior, SaturatingBool, SettingsContent, SettingsStore,
};
use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashMap,
    ops::Range,
    rc::Rc,
    sync::{Arc, atomic::AtomicBool},
};
use ui::{
    ContextMenu, Divider, DropdownMenu, DropdownStyle, Switch, SwitchColor, TreeViewItem,
    prelude::*,
};
use util::{paths::PathStyle, rel_path::RelPath};

use crate::components::SettingsEditor;

#[derive(Clone, Copy)]
struct SettingField<T: 'static> {
    pick: fn(&SettingsContent) -> &Option<T>,
    pick_mut: fn(&mut SettingsContent) -> &mut Option<T>,
}

trait AnySettingField {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
    fn file_set_in(&self, file: SettingsUiFile, cx: &App) -> settings::SettingsFile;
}

impl<T> AnySettingField for SettingField<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn file_set_in(&self, file: SettingsUiFile, cx: &App) -> settings::SettingsFile {
        let (file, _) = cx
            .global::<SettingsStore>()
            .get_value_from_file(file.to_settings(), self.pick);
        return file;
    }
}

#[derive(Default, Clone)]
struct SettingFieldRenderer {
    renderers: Rc<
        RefCell<
            HashMap<
                TypeId,
                Box<
                    dyn Fn(
                        &dyn AnySettingField,
                        SettingsUiFile,
                        Option<&SettingsFieldMetadata>,
                        &mut Window,
                        &mut App,
                    ) -> AnyElement,
                >,
            >,
        >,
    >,
}

impl Global for SettingFieldRenderer {}

impl SettingFieldRenderer {
    fn add_renderer<T: 'static>(
        &mut self,
        renderer: impl Fn(
            &SettingField<T>,
            SettingsUiFile,
            Option<&SettingsFieldMetadata>,
            &mut Window,
            &mut App,
        ) -> AnyElement
        + 'static,
    ) -> &mut Self {
        let key = TypeId::of::<T>();
        let renderer = Box::new(
            move |any_setting_field: &dyn AnySettingField,
                  settings_file: SettingsUiFile,
                  metadata: Option<&SettingsFieldMetadata>,
                  window: &mut Window,
                  cx: &mut App| {
                let field = any_setting_field
                    .as_any()
                    .downcast_ref::<SettingField<T>>()
                    .unwrap();
                renderer(field, settings_file, metadata, window, cx)
            },
        );
        self.renderers.borrow_mut().insert(key, renderer);
        self
    }

    fn render(
        &self,
        any_setting_field: &dyn AnySettingField,
        settings_file: SettingsUiFile,
        metadata: Option<&SettingsFieldMetadata>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let key = any_setting_field.type_id();
        if let Some(renderer) = self.renderers.borrow().get(&key) {
            renderer(any_setting_field, settings_file, metadata, window, cx)
        } else {
            panic!(
                "No renderer found for type: {}",
                any_setting_field.type_name()
            )
        }
    }
}

struct SettingsFieldMetadata {
    placeholder: Option<&'static str>,
}

fn user_settings_data() -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "General Page",
            expanded: false,
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
            expanded: false,
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
                // todo(settings_ui): We need to implement a numeric stepper for these
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Buffer Font Size",
                //     description: "Font size for editor text",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.buffer_font_size,
                //         pick_mut: |settings_content| &mut settings_content.theme.buffer_font_size,
                //     }),
                //     metadata: None,
                // }),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Buffer Font Weight",
                //     description: "Font weight for editor text (100-900)",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.buffer_font_weight,
                //         pick_mut: |settings_content| &mut settings_content.theme.buffer_font_weight,
                //     }),
                //     metadata: None,
                // }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Buffer Line Height",
                    description: "Line height for editor text",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.theme.buffer_line_height,
                        pick_mut: |settings_content| &mut settings_content.theme.buffer_line_height,
                    }),
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
                }),
                // todo(settings_ui): We need to implement a numeric stepper for these
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "UI Font Size",
                //     description: "Font size for UI elements",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.ui_font_size,
                //         pick_mut: |settings_content| &mut settings_content.theme.ui_font_size,
                //     }),
                //     metadata: None,
                // }),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "UI Font Weight",
                //     description: "Font weight for UI elements (100-900)",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.ui_font_weight,
                //         pick_mut: |settings_content| &mut settings_content.theme.ui_font_weight,
                //     }),
                //     metadata: None,
                // }),
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
                // todo(settings_ui): numeric stepper and validator is needed for this
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Unnecessary Code Fade",
                //     description: "How much to fade out unused code (0.0 - 0.9)",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.theme.unnecessary_code_fade,
                //         pick_mut: |settings_content| &mut settings_content.theme.unnecessary_code_fade,
                //     }),
                //     metadata: None,
                // }),
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
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Centered Layout Left Padding",
                //     description: "Left padding for cenetered layout",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.workspace.bottom_dock_layout,
                //         pick_mut: |settings_content| {
                //             &mut settings_content.workspace.bottom_dock_layout
                //         },
                //     }),
                //     metadata: None,
                // }),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Centered Layout Right Padding",
                //     description: "Right padding for cenetered layout",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.workspace.bottom_dock_layout,
                //         pick_mut: |settings_content| {
                //             &mut settings_content.workspace.bottom_dock_layout
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
            expanded: false,
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
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Vertical Scroll Margin",
                //     description: "The number of lines to keep above/below the cursor when auto-scrolling",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.editor.vertical_scroll_margin,
                //         pick_mut: |settings_content| &mut settings_content.editor.vertical_scroll_margin,
                //     }),
                //     metadata: None,
                // }),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Horizontal Scroll Margin",
                //     description: "The number of characters to keep on either side when scrolling with the mouse",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.editor.horizontal_scroll_margin,
                //         pick_mut: |settings_content| &mut settings_content.editor.horizontal_scroll_margin,
                //     }),
                //     metadata: None,
                // }),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Scroll Sensitivity",
                //     description: "Scroll sensitivity multiplier",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.editor.scroll_sensitivity,
                //         pick_mut: |settings_content| &mut settings_content.editor.scroll_sensitivity,
                //     }),
                //     metadata: None,
                // }),
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
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Hover Popover Delay",
                //     description: "Time to wait in milliseconds before showing the informational hover box",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| &settings_content.editor.hover_popover_delay,
                //         pick_mut: |settings_content| &mut settings_content.editor.hover_popover_delay,
                //     }),
                //     metadata: None,
                // }),
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
            title: "Workbench & Window",
            expanded: false,
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
                            if let Some(status_bar) = &settings_content.editor.status_bar {
                                &status_bar.active_language_button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
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
                            if let Some(status_bar) = &settings_content.editor.status_bar {
                                &status_bar.cursor_position_button
                            } else {
                                &None
                            }
                        },
                        pick_mut: |settings_content| {
                            &mut settings_content
                                .editor
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
            expanded: false,
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
            expanded: false,
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
                // todo(settings_ui): Needs numeric stepper
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
            expanded: false,
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
            expanded: false,
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
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Inline Update Debounce",
                //     description: "The delay in milliseconds to show inline diagnostics after the last diagnostic update",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(diagnostics) = &settings_content.diagnostics {
                //                 if let Some(inline) = &diagnostics.inline {
                //                     &inline.update_debounce_ms
                //                 } else {
                //                     &None
                //                 }
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content
                //                 .diagnostics
                //                 .get_or_insert_default()
                //                 .inline
                //                 .get_or_insert_default()
                //                 .update_debounce_ms
                //         },
                //     }),
                //     metadata: None,
                // }),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Inline Padding",
                //     description: "The amount of padding between the end of the source line and the start of the inline diagnostic",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(diagnostics) = &settings_content.diagnostics {
                //                 if let Some(inline) = &diagnostics.inline {
                //                     &inline.padding
                //                 } else {
                //                     &None
                //                 }
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content
                //                 .diagnostics
                //                 .get_or_insert_default()
                //                 .inline
                //                 .get_or_insert_default()
                //                 .padding
                //         },
                //     }),
                //     metadata: None,
                // }),
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Inline Min Column",
                //     description: "The minimum column to display inline diagnostics",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(diagnostics) = &settings_content.diagnostics {
                //                 if let Some(inline) = &diagnostics.inline {
                //                     &inline.min_column
                //                 } else {
                //                     &None
                //                 }
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content
                //                 .diagnostics
                //                 .get_or_insert_default()
                //                 .inline
                //                 .get_or_insert_default()
                //                 .min_column
                //         },
                //     }),
                //     metadata: None,
                // }),
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
                // todo(settings_ui): Needs numeric stepper
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "LSP Pull Debounce",
                //     description: "Minimum time to wait before pulling diagnostics from the language server(s)",
                //     field: Box::new(SettingField {
                //         pick: |settings_content| {
                //             if let Some(diagnostics) = &settings_content.diagnostics {
                //                 if let Some(lsp_pull) = &diagnostics.lsp_pull_diagnostics {
                //                     &lsp_pull.debounce_ms
                //                 } else {
                //                     &None
                //                 }
                //             } else {
                //                 &None
                //             }
                //         },
                //         pick_mut: |settings_content| {
                //             &mut settings_content
                //                 .diagnostics
                //                 .get_or_insert_default()
                //                 .lsp_pull_diagnostics
                //                 .get_or_insert_default()
                //                 .debounce_ms
                //         },
                //     }),
                //     metadata: None,
                // }),
            ],
        },
        SettingsPage {
            title: "Collaboration",
            expanded: false,
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
            expanded: false,
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

// Derive Macro, on the new ProjectSettings struct

fn project_settings_data() -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "Project",
            expanded: false,
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
            expanded: false,
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
            expanded: false,
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

pub struct SettingsUiFeatureFlag;

impl FeatureFlag for SettingsUiFeatureFlag {
    const NAME: &'static str = "settings-ui";
}

actions!(
    zed,
    [
        /// Opens Settings Editor.
        OpenSettingsEditor
    ]
);

pub fn init(cx: &mut App) {
    init_renderers(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action_renderer(|div, _, _, cx| {
            let settings_ui_actions = [std::any::TypeId::of::<OpenSettingsEditor>()];
            let has_flag = cx.has_flag::<SettingsUiFeatureFlag>();
            command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _| {
                if has_flag {
                    filter.show_action_types(&settings_ui_actions);
                } else {
                    filter.hide_action_types(&settings_ui_actions);
                }
            });
            if has_flag {
                div.on_action(cx.listener(|_, _: &OpenSettingsEditor, _, cx| {
                    open_settings_editor(cx).ok();
                }))
            } else {
                div
            }
        });
    })
    .detach();
}

fn init_renderers(cx: &mut App) {
    // fn (field: SettingsField, current_file: SettingsFile, cx) -> (currently_set_in: SettingsFile, overridden_in: Vec<SettingsFile>)
    cx.default_global::<SettingFieldRenderer>()
        .add_renderer::<bool>(|settings_field, file, _, _, cx| {
            render_toggle_button(*settings_field, file, cx).into_any_element()
        })
        .add_renderer::<String>(|settings_field, file, metadata, _, cx| {
            render_text_field(settings_field.clone(), file, metadata, cx)
        })
        .add_renderer::<SaturatingBool>(|settings_field, file, _, _, cx| {
            render_toggle_button(*settings_field, file, cx)
        })
        .add_renderer::<CursorShape>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<RestoreOnStartupBehavior>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<BottomDockLayout>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<OnLastWindowClosed>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<CloseWindowWhenNoItems>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::FontFamilyName>(|settings_field, file, metadata, _, cx| {
            // todo(settings_ui): We need to pass in a validator for this to ensure that users that type in invalid font names
            render_text_field(settings_field.clone(), file, metadata, cx)
        })
        .add_renderer::<settings::BufferLineHeight>(|settings_field, file, _, window, cx| {
            // todo(settings_ui): Do we want to expose the custom variant of buffer line height?
            // right now there's a manual impl of strum::VariantArray
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::BaseKeymapContent>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::MultiCursorModifier>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::HideMouseMode>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::CurrentLineHighlight>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowWhitespaceSetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::SoftWrap>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ScrollBeyondLastLine>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::SnippetSortOrder>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ClosePosition>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DockSide>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::TerminalDockPosition>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::GitGutterSetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::GitHunkStyleSetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DiagnosticSeverityContent>(
            |settings_field, file, _, window, cx| {
                render_dropdown(*settings_field, file, window, cx)
            },
        )
        .add_renderer::<settings::SeedQuerySetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DoubleClickInMultibuffer>(
            |settings_field, file, _, window, cx| {
                render_dropdown(*settings_field, file, window, cx)
            },
        )
        .add_renderer::<settings::GoToDefinitionFallback>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ActivateOnClose>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowDiagnostics>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowCloseButton>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        });

    // todo(settings_ui): Figure out how we want to handle discriminant unions
    // .add_renderer::<ThemeSelection>(|settings_field, file, _, window, cx| {
    //     render_dropdown(*settings_field, file, window, cx)
    // });
}

pub fn open_settings_editor(cx: &mut App) -> anyhow::Result<WindowHandle<SettingsWindow>> {
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Settings Window".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(12.0), px(12.0))),
            }),
            focus: true,
            show: true,
            kind: gpui::WindowKind::Normal,
            window_background: cx.theme().window_background_appearance(),
            window_min_size: Some(size(px(800.), px(600.))), // 4:3 Aspect Ratio
            ..Default::default()
        },
        |window, cx| cx.new(|cx| SettingsWindow::new(window, cx)),
    )
}

pub struct SettingsWindow {
    files: Vec<SettingsUiFile>,
    current_file: SettingsUiFile,
    pages: Vec<SettingsPage>,
    search_bar: Entity<Editor>,
    search_task: Option<Task<()>>,
    navbar_entry: usize, // Index into pages - should probably be (usize, Option<usize>) for section + page
    navbar_entries: Vec<NavBarEntry>,
    list_handle: UniformListScrollHandle,
    search_matches: Vec<Vec<bool>>,
}

#[derive(PartialEq, Debug)]
struct NavBarEntry {
    title: &'static str,
    is_root: bool,
    page_index: usize,
}

struct SettingsPage {
    title: &'static str,
    expanded: bool,
    items: Vec<SettingsPageItem>,
}

#[derive(PartialEq)]
enum SettingsPageItem {
    SectionHeader(&'static str),
    SettingItem(SettingItem),
}

impl std::fmt::Debug for SettingsPageItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsPageItem::SectionHeader(header) => write!(f, "SectionHeader({})", header),
            SettingsPageItem::SettingItem(setting_item) => {
                write!(f, "SettingItem({})", setting_item.title)
            }
        }
    }
}

impl SettingsPageItem {
    fn render(
        &self,
        file: SettingsUiFile,
        is_last: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        match self {
            SettingsPageItem::SectionHeader(header) => v_flex()
                .w_full()
                .gap_1()
                .child(
                    Label::new(SharedString::new_static(header))
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .buffer_font(cx),
                )
                .child(Divider::horizontal().color(ui::DividerColor::BorderVariant))
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => {
                let renderer = cx.default_global::<SettingFieldRenderer>().clone();
                let file_set_in =
                    SettingsUiFile::from_settings(setting_item.field.file_set_in(file.clone(), cx));

                h_flex()
                    .id(setting_item.title)
                    .w_full()
                    .gap_2()
                    .flex_wrap()
                    .justify_between()
                    .when(!is_last, |this| {
                        this.pb_4()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                    })
                    .child(
                        v_flex()
                            .max_w_1_2()
                            .flex_shrink()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        Label::new(SharedString::new_static(setting_item.title))
                                            .size(LabelSize::Default),
                                    )
                                    .when_some(
                                        file_set_in.filter(|file_set_in| file_set_in != &file),
                                        |elem, file_set_in| {
                                            elem.child(
                                                Label::new(format!(
                                                    "set in {}",
                                                    file_set_in.name()
                                                ))
                                                .color(Color::Muted),
                                            )
                                        },
                                    ),
                            )
                            .child(
                                Label::new(SharedString::new_static(setting_item.description))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(renderer.render(
                        setting_item.field.as_ref(),
                        file,
                        setting_item.metadata.as_deref(),
                        window,
                        cx,
                    ))
                    .into_any_element()
            }
        }
    }
}

struct SettingItem {
    title: &'static str,
    description: &'static str,
    field: Box<dyn AnySettingField>,
    metadata: Option<Box<SettingsFieldMetadata>>,
}

impl PartialEq for SettingItem {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && (match (&self.metadata, &other.metadata) {
                (None, None) => true,
                (Some(m1), Some(m2)) => m1.placeholder == m2.placeholder,
                _ => false,
            })
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq)]
enum SettingsUiFile {
    User,                              // Uses all settings.
    Local((WorktreeId, Arc<RelPath>)), // Has a special name, and special set of settings
    Server(&'static str),              // Uses a special name, and the user settings
}

impl SettingsUiFile {
    fn pages(&self) -> Vec<SettingsPage> {
        match self {
            SettingsUiFile::User => user_settings_data(),
            SettingsUiFile::Local(_) => project_settings_data(),
            SettingsUiFile::Server(_) => user_settings_data(),
        }
    }

    fn name(&self) -> SharedString {
        match self {
            SettingsUiFile::User => SharedString::new_static("User"),
            // TODO is PathStyle::local() ever not appropriate?
            SettingsUiFile::Local((_, path)) => {
                format!("Local ({})", path.display(PathStyle::local())).into()
            }
            SettingsUiFile::Server(file) => format!("Server ({})", file).into(),
        }
    }

    fn from_settings(file: settings::SettingsFile) -> Option<Self> {
        Some(match file {
            settings::SettingsFile::User => SettingsUiFile::User,
            settings::SettingsFile::Local(location) => SettingsUiFile::Local(location),
            settings::SettingsFile::Server => SettingsUiFile::Server("todo: server name"),
            settings::SettingsFile::Default => return None,
        })
    }

    fn to_settings(&self) -> settings::SettingsFile {
        match self {
            SettingsUiFile::User => settings::SettingsFile::User,
            SettingsUiFile::Local(location) => settings::SettingsFile::Local(location.clone()),
            SettingsUiFile::Server(_) => settings::SettingsFile::Server,
        }
    }
}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_file = SettingsUiFile::User;
        let search_bar = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search settings…", window, cx);
            editor
        });

        cx.subscribe(&search_bar, |this, _, event: &EditorEvent, cx| {
            let EditorEvent::Edited { transaction_id: _ } = event else {
                return;
            };

            this.update_matches(cx);
        })
        .detach();

        cx.observe_global_in::<SettingsStore>(window, move |this, _, cx| {
            this.fetch_files(cx);
            cx.notify();
        })
        .detach();

        let mut this = Self {
            files: vec![],
            current_file: current_file,
            pages: vec![],
            navbar_entries: vec![],
            navbar_entry: 0,
            list_handle: UniformListScrollHandle::default(),
            search_bar,
            search_task: None,
            search_matches: vec![],
        };

        this.fetch_files(cx);
        this.build_ui(cx);

        this
    }

    fn toggle_navbar_entry(&mut self, ix: usize) {
        // We can only toggle root entries
        if !self.navbar_entries[ix].is_root {
            return;
        }

        let toggle_page_index = self.page_index_from_navbar_index(ix);
        let selected_page_index = self.page_index_from_navbar_index(self.navbar_entry);

        let expanded = &mut self.page_for_navbar_index(ix).expanded;
        *expanded = !*expanded;
        let expanded = *expanded;
        // if currently selected page is a child of the parent page we are folding,
        // set the current page to the parent page
        if selected_page_index == toggle_page_index {
            self.navbar_entry = ix;
        } else if selected_page_index > toggle_page_index {
            let sub_items_count = self.pages[toggle_page_index]
                .items
                .iter()
                .filter(|item| matches!(item, SettingsPageItem::SectionHeader(_)))
                .count();
            if expanded {
                self.navbar_entry += sub_items_count;
            } else {
                self.navbar_entry -= sub_items_count;
            }
        }

        self.build_navbar();
    }

    fn build_navbar(&mut self) {
        let mut navbar_entries = Vec::with_capacity(self.navbar_entries.len());
        for (page_index, page) in self.pages.iter().enumerate() {
            if !self.search_matches[page_index]
                .iter()
                .any(|is_match| *is_match)
                && !self.search_matches[page_index].is_empty()
            {
                continue;
            }
            navbar_entries.push(NavBarEntry {
                title: page.title,
                is_root: true,
                page_index,
            });
            if !page.expanded {
                continue;
            }

            for (item_index, item) in page.items.iter().enumerate() {
                let SettingsPageItem::SectionHeader(title) = item else {
                    continue;
                };
                if !self.search_matches[page_index][item_index] {
                    continue;
                }

                navbar_entries.push(NavBarEntry {
                    title,
                    is_root: false,
                    page_index,
                });
            }
        }
        self.navbar_entries = navbar_entries;
    }

    fn update_matches(&mut self, cx: &mut Context<SettingsWindow>) {
        self.search_task.take();
        let query = self.search_bar.read(cx).text(cx);
        if query.is_empty() {
            for page in &mut self.search_matches {
                page.fill(true);
            }
            self.build_navbar();
            cx.notify();
            return;
        }

        struct ItemKey {
            page_index: usize,
            header_index: usize,
            item_index: usize,
        }
        let mut key_lut: Vec<ItemKey> = vec![];
        let mut candidates = Vec::default();

        for (page_index, page) in self.pages.iter().enumerate() {
            let mut header_index = 0;
            for (item_index, item) in page.items.iter().enumerate() {
                let key_index = key_lut.len();
                match item {
                    SettingsPageItem::SettingItem(item) => {
                        candidates.push(StringMatchCandidate::new(key_index, item.title));
                        candidates.push(StringMatchCandidate::new(key_index, item.description));
                    }
                    SettingsPageItem::SectionHeader(header) => {
                        candidates.push(StringMatchCandidate::new(key_index, header));
                        header_index = item_index;
                    }
                }
                key_lut.push(ItemKey {
                    page_index,
                    header_index,
                    item_index,
                });
            }
        }
        let atomic_bool = AtomicBool::new(false);

        self.search_task = Some(cx.spawn(async move |this, cx| {
            let string_matches = fuzzy::match_strings(
                candidates.as_slice(),
                &query,
                false,
                false,
                candidates.len(),
                &atomic_bool,
                cx.background_executor().clone(),
            );
            let string_matches = string_matches.await;

            this.update(cx, |this, cx| {
                for page in &mut this.search_matches {
                    page.fill(false);
                }

                for string_match in string_matches {
                    let ItemKey {
                        page_index,
                        header_index,
                        item_index,
                    } = key_lut[string_match.candidate_id];
                    let page = &mut this.search_matches[page_index];
                    page[header_index] = true;
                    page[item_index] = true;
                }
                this.build_navbar();
                this.navbar_entry = 0;
                cx.notify();
            })
            .ok();
        }));
    }

    fn build_ui(&mut self, cx: &mut Context<SettingsWindow>) {
        self.pages = self.current_file.pages();
        self.search_matches = self
            .pages
            .iter()
            .map(|page| vec![true; page.items.len()])
            .collect::<Vec<_>>();
        self.build_navbar();

        if !self.search_bar.read(cx).is_empty(cx) {
            self.update_matches(cx);
        }

        cx.notify();
    }

    fn fetch_files(&mut self, cx: &mut Context<SettingsWindow>) {
        let settings_store = cx.global::<SettingsStore>();
        let mut ui_files = vec![];
        let all_files = settings_store.get_all_files();
        for file in all_files {
            let Some(settings_ui_file) = SettingsUiFile::from_settings(file) else {
                continue;
            };
            ui_files.push(settings_ui_file);
        }
        ui_files.reverse();
        self.files = ui_files;
        if !self.files.contains(&self.current_file) {
            self.change_file(0, cx);
        }
    }

    fn change_file(&mut self, ix: usize, cx: &mut Context<SettingsWindow>) {
        if ix >= self.files.len() {
            self.current_file = SettingsUiFile::User;
            return;
        }
        if self.files[ix] == self.current_file {
            return;
        }
        self.current_file = self.files[ix].clone();
        self.navbar_entry = 0;
        self.build_ui(cx);
    }

    fn render_files(&self, _window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        h_flex()
            .gap_1()
            .children(self.files.iter().enumerate().map(|(ix, file)| {
                Button::new(ix, file.name())
                    .on_click(cx.listener(move |this, _, _window, cx| this.change_file(ix, cx)))
            }))
    }

    fn render_search(&self, _window: &mut Window, cx: &mut App) -> Div {
        h_flex()
            .pt_1()
            .px_1p5()
            .gap_1p5()
            .rounded_sm()
            .bg(cx.theme().colors().editor_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
            .child(self.search_bar.clone())
    }

    fn render_nav(&self, window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        v_flex()
            .w_64()
            .p_2p5()
            .pt_10()
            .gap_3()
            .flex_none()
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().panel_background)
            .child(self.render_search(window, cx).pb_1())
            .child(
                uniform_list(
                    "settings-ui-nav-bar",
                    self.navbar_entries.len(),
                    cx.processor(|this, range: Range<usize>, _, cx| {
                        range
                            .into_iter()
                            .map(|ix| {
                                let entry = &this.navbar_entries[ix];

                                TreeViewItem::new(("settings-ui-navbar-entry", ix), entry.title)
                                    .root_item(entry.is_root)
                                    .toggle_state(this.is_navbar_entry_selected(ix))
                                    .when(entry.is_root, |item| {
                                        item.toggle(
                                            this.pages[this.page_index_from_navbar_index(ix)]
                                                .expanded,
                                        )
                                        .on_toggle(
                                            cx.listener(move |this, _, _, cx| {
                                                this.toggle_navbar_entry(ix);
                                                cx.notify();
                                            }),
                                        )
                                    })
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.navbar_entry = ix;
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            })
                            .collect()
                    }),
                )
                .track_scroll(self.list_handle.clone())
                .size_full()
                .flex_grow(),
            )
    }

    fn page_items(&self) -> impl Iterator<Item = &SettingsPageItem> {
        let page_idx = self.current_page_index();

        self.current_page()
            .items
            .iter()
            .enumerate()
            .filter_map(move |(item_index, item)| {
                self.search_matches[page_idx][item_index].then_some(item)
            })
    }

    fn render_page(&self, window: &mut Window, cx: &mut Context<SettingsWindow>) -> Stateful<Div> {
        let items: Vec<_> = self.page_items().collect();
        let items_len = items.len();

        v_flex()
            .id("settings-ui-page")
            .gap_4()
            .children(items.into_iter().enumerate().map(|(index, item)| {
                let is_last = index == items_len - 1;
                item.render(self.current_file.clone(), is_last, window, cx)
            }))
            .overflow_y_scroll()
            .track_scroll(
                window
                    .use_state(cx, |_, _| ScrollHandle::default())
                    .read(cx),
            )
    }

    fn current_page_index(&self) -> usize {
        self.page_index_from_navbar_index(self.navbar_entry)
    }

    fn current_page(&self) -> &SettingsPage {
        &self.pages[self.current_page_index()]
    }

    fn page_index_from_navbar_index(&self, index: usize) -> usize {
        if self.navbar_entries.is_empty() {
            return 0;
        }

        self.navbar_entries[index].page_index
    }

    fn page_for_navbar_index(&mut self, index: usize) -> &mut SettingsPage {
        let index = self.page_index_from_navbar_index(index);
        &mut self.pages[index]
    }

    fn is_navbar_entry_selected(&self, ix: usize) -> bool {
        ix == self.navbar_entry
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);

        div()
            .flex()
            .flex_row()
            .size_full()
            .font(ui_font)
            .bg(cx.theme().colors().background)
            .text_color(cx.theme().colors().text)
            .child(self.render_nav(window, cx))
            .child(
                v_flex()
                    .w_full()
                    .pt_4()
                    .px_6()
                    .gap_4()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.render_files(window, cx))
                    .child(self.render_page(window, cx)),
            )
    }
}

fn render_text_field<T: From<String> + Into<String> + AsRef<str> + Clone>(
    field: SettingField<T>,
    file: SettingsUiFile,
    metadata: Option<&SettingsFieldMetadata>,
    cx: &mut App,
) -> AnyElement {
    let (_, initial_text) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let initial_text = Some(initial_text.clone()).filter(|s| !s.as_ref().is_empty());

    SettingsEditor::new()
        .when_some(initial_text, |editor, text| {
            editor.with_initial_text(text.into())
        })
        .when_some(
            metadata.and_then(|metadata| metadata.placeholder),
            |editor, placeholder| editor.with_placeholder(placeholder),
        )
        .on_confirm(move |new_text, cx: &mut App| {
            cx.update_global(move |store: &mut SettingsStore, cx| {
                store.update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _cx| {
                    *(field.pick_mut)(settings) = new_text.map(Into::into);
                });
            });
        })
        .into_any_element()
}

fn render_toggle_button<B: Into<bool> + From<bool> + Copy>(
    field: SettingField<B>,
    file: SettingsUiFile,
    cx: &mut App,
) -> AnyElement {
    let (_, &value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);

    let toggle_state = if value.into() {
        ToggleState::Selected
    } else {
        ToggleState::Unselected
    };

    Switch::new("toggle_button", toggle_state)
        .color(ui::SwitchColor::Accent)
        .on_click({
            move |state, _window, cx| {
                let state = *state == ui::ToggleState::Selected;
                let field = field;
                cx.update_global(move |store: &mut SettingsStore, cx| {
                    store.update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _cx| {
                        *(field.pick_mut)(settings) = Some(state.into());
                    });
                });
            }
        })
        .color(SwitchColor::Accent)
        .into_any_element()
}

fn render_dropdown<T>(
    field: SettingField<T>,
    file: SettingsUiFile,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + 'static,
{
    let variants = || -> &'static [T] { <T as strum::VariantArray>::VARIANTS };
    let labels = || -> &'static [&'static str] { <T as strum::VariantNames>::VARIANTS };

    let (_, &current_value) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);

    let current_value_label =
        labels()[variants().iter().position(|v| *v == current_value).unwrap()];

    DropdownMenu::new(
        "dropdown",
        current_value_label,
        ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (value, label) in variants()
                .into_iter()
                .copied()
                .zip(labels().into_iter().copied())
            {
                menu = menu.toggleable_entry(
                    label,
                    value == current_value,
                    IconPosition::Start,
                    None,
                    move |_, cx| {
                        if value == current_value {
                            return;
                        }
                        cx.update_global(move |store: &mut SettingsStore, cx| {
                            store.update_settings_file(
                                <dyn fs::Fs>::global(cx),
                                move |settings, _cx| {
                                    *(field.pick_mut)(settings) = Some(value);
                                },
                            );
                        });
                    },
                );
            }
            menu
        }),
    )
    .style(DropdownStyle::Outlined)
    .into_any_element()
}

#[cfg(test)]
mod test {

    use super::*;

    impl SettingsWindow {
        fn navbar(&self) -> &[NavBarEntry] {
            self.navbar_entries.as_slice()
        }

        fn navbar_entry(&self) -> usize {
            self.navbar_entry
        }

        fn new_builder(window: &mut Window, cx: &mut Context<Self>) -> Self {
            let mut this = Self::new(window, cx);
            this.navbar_entries.clear();
            this.pages.clear();
            this
        }

        fn build(mut self) -> Self {
            self.build_navbar();
            self
        }

        fn add_page(
            mut self,
            title: &'static str,
            build_page: impl Fn(SettingsPage) -> SettingsPage,
        ) -> Self {
            let page = SettingsPage {
                title,
                expanded: false,
                items: Vec::default(),
            };

            self.pages.push(build_page(page));
            self
        }

        fn search(&mut self, search_query: &str, window: &mut Window, cx: &mut Context<Self>) {
            self.search_task.take();
            self.search_bar.update(cx, |editor, cx| {
                editor.set_text(search_query, window, cx);
            });
            self.update_matches(cx);
        }

        fn assert_search_results(&self, other: &Self) {
            // page index could be different because of filtered out pages
            assert!(
                self.navbar_entries
                    .iter()
                    .zip(other.navbar_entries.iter())
                    .all(|(entry, other)| {
                        entry.is_root == other.is_root && entry.title == other.title
                    })
            );
            assert_eq!(
                self.current_page().items.iter().collect::<Vec<_>>(),
                other.page_items().collect::<Vec<_>>()
            );
        }
    }

    impl SettingsPage {
        fn item(mut self, item: SettingsPageItem) -> Self {
            self.items.push(item);
            self
        }
    }

    impl SettingsPageItem {
        fn basic_item(title: &'static str, description: &'static str) -> Self {
            SettingsPageItem::SettingItem(SettingItem {
                title,
                description,
                field: Box::new(SettingField {
                    pick: |settings_content| &settings_content.auto_update,
                    pick_mut: |settings_content| &mut settings_content.auto_update,
                }),
                metadata: None,
            })
        }
    }

    fn register_settings(cx: &mut App) {
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        workspace::init_settings(cx);
        project::Project::init_settings(cx);
        language::init(cx);
        editor::init(cx);
        menu::init();
    }

    fn parse(input: &'static str, window: &mut Window, cx: &mut App) -> SettingsWindow {
        let mut pages: Vec<SettingsPage> = Vec::new();
        let mut current_page = None;
        let mut selected_idx = None;
        let mut ix = 0;
        let mut in_closed_subentry = false;

        for mut line in input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
        {
            let mut is_selected = false;
            if line.ends_with("*") {
                assert!(
                    selected_idx.is_none(),
                    "Can only have one selected navbar entry at a time"
                );
                selected_idx = Some(ix);
                line = &line[..line.len() - 1];
                is_selected = true;
            }

            if line.starts_with("v") || line.starts_with(">") {
                if let Some(current_page) = current_page.take() {
                    pages.push(current_page);
                }

                let expanded = line.starts_with("v");
                in_closed_subentry = !expanded;
                ix += 1;

                current_page = Some(SettingsPage {
                    title: line.split_once(" ").unwrap().1,
                    expanded,
                    items: Vec::default(),
                });
            } else if line.starts_with("-") {
                if !in_closed_subentry {
                    ix += 1;
                } else if is_selected && in_closed_subentry {
                    panic!("Can't select sub entry if it's parent is closed");
                }

                let Some(current_page) = current_page.as_mut() else {
                    panic!("Sub entries must be within a page");
                };

                current_page.items.push(SettingsPageItem::SectionHeader(
                    line.split_once(" ").unwrap().1,
                ));
            } else {
                panic!(
                    "Entries must start with one of 'v', '>', or '-'\n line: {}",
                    line
                );
            }
        }

        if let Some(current_page) = current_page.take() {
            pages.push(current_page);
        }

        let search_matches = pages
            .iter()
            .map(|page| vec![true; page.items.len()])
            .collect::<Vec<_>>();

        let mut settings_window = SettingsWindow {
            files: Vec::default(),
            current_file: crate::SettingsUiFile::User,
            pages,
            search_bar: cx.new(|cx| Editor::single_line(window, cx)),
            navbar_entry: selected_idx.expect("Must have a selected navbar entry"),
            navbar_entries: Vec::default(),
            list_handle: UniformListScrollHandle::default(),
            search_matches,
            search_task: None,
        };

        settings_window.build_navbar();
        settings_window
    }

    #[track_caller]
    fn check_navbar_toggle(
        before: &'static str,
        toggle_idx: usize,
        after: &'static str,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut settings_window = parse(before, window, cx);
        settings_window.toggle_navbar_entry(toggle_idx);

        let expected_settings_window = parse(after, window, cx);

        assert_eq!(settings_window.navbar(), expected_settings_window.navbar());
        assert_eq!(
            settings_window.navbar_entry(),
            expected_settings_window.navbar_entry()
        );
    }

    macro_rules! check_navbar_toggle {
        ($name:ident, before: $before:expr, toggle_idx: $toggle_idx:expr, after: $after:expr) => {
            #[gpui::test]
            fn $name(cx: &mut gpui::TestAppContext) {
                let window = cx.add_empty_window();
                window.update(|window, cx| {
                    register_settings(cx);
                    check_navbar_toggle($before, $toggle_idx, $after, window, cx);
                });
            }
        };
    }

    check_navbar_toggle!(
        navbar_basic_open,
        before: r"
        v General
        - General
        - Privacy*
        v Project
        - Project Settings
        ",
        toggle_idx: 0,
        after: r"
        > General*
        v Project
        - Project Settings
        "
    );

    check_navbar_toggle!(
        navbar_basic_close,
        before: r"
        > General*
        - General
        - Privacy
        v Project
        - Project Settings
        ",
        toggle_idx: 0,
        after: r"
        v General*
        - General
        - Privacy
        v Project
        - Project Settings
        "
    );

    check_navbar_toggle!(
        navbar_basic_second_root_entry_close,
        before: r"
        > General
        - General
        - Privacy
        v Project
        - Project Settings*
        ",
        toggle_idx: 1,
        after: r"
        > General
        > Project*
        "
    );

    check_navbar_toggle!(
        navbar_toggle_subroot,
        before: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content*
        v AI
        - General
        > Appearance & Behavior
        ",
        toggle_idx: 3,
        after: r"
        v General Page
        - General
        - Privacy
        > Project*
        v AI
        - General
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_close_propagates_selected_index,
        before: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_idx: 0,
        after: r"
        > General Page
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_expand_propagates_selected_index,
        before: r"
        > General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_idx: 0,
        after: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_sub_entry_does_nothing,
        before: r"
        > General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_idx: 4,
        after: r"
        > General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    #[gpui::test]
    fn test_basic_search(cx: &mut gpui::TestAppContext) {
        let cx = cx.add_empty_window();
        let (actual, expected) = cx.update(|window, cx| {
            register_settings(cx);

            let expected = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item("test title", "General test"))
                    })
                    .build()
            });

            let actual = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item("test title", "General test"))
                    })
                    .add_page("Theme", |page| {
                        page.item(SettingsPageItem::SectionHeader("Theme settings"))
                    })
                    .build()
            });

            actual.update(cx, |settings, cx| settings.search("gen", window, cx));

            (actual, expected)
        });

        cx.cx.run_until_parked();

        cx.update(|_window, cx| {
            let expected = expected.read(cx);
            let actual = actual.read(cx);
            expected.assert_search_results(&actual);
        })
    }

    #[gpui::test]
    fn test_search_render_page_with_filtered_out_navbar_entries(cx: &mut gpui::TestAppContext) {
        let cx = cx.add_empty_window();
        let (actual, expected) = cx.update(|window, cx| {
            register_settings(cx);

            let actual = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item(
                                "Confirm Quit",
                                "Whether to confirm before quitting Zed",
                            ))
                            .item(SettingsPageItem::basic_item(
                                "Auto Update",
                                "Automatically update Zed",
                            ))
                    })
                    .add_page("AI", |page| {
                        page.item(SettingsPageItem::basic_item(
                            "Disable AI",
                            "Whether to disable all AI features in Zed",
                        ))
                    })
                    .add_page("Appearance & Behavior", |page| {
                        page.item(SettingsPageItem::SectionHeader("Cursor")).item(
                            SettingsPageItem::basic_item(
                                "Cursor Shape",
                                "Cursor shape for the editor",
                            ),
                        )
                    })
                    .build()
            });

            let expected = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("Appearance & Behavior", |page| {
                        page.item(SettingsPageItem::SectionHeader("Cursor")).item(
                            SettingsPageItem::basic_item(
                                "Cursor Shape",
                                "Cursor shape for the editor",
                            ),
                        )
                    })
                    .build()
            });

            actual.update(cx, |settings, cx| settings.search("cursor", window, cx));

            (actual, expected)
        });

        cx.cx.run_until_parked();

        cx.update(|_window, cx| {
            let expected = expected.read(cx);
            let actual = actual.read(cx);
            expected.assert_search_results(&actual);
        })
    }
}
