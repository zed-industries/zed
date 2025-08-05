use std::sync::Arc;

use editor::{EditorSettings, ShowMinimap};
use fs::Fs;
use gpui::{Action, App, FontFeatures, IntoElement, Pixels, Window};
use language::language_settings::{AllLanguageSettings, FormatOnSave};
use project::project_settings::ProjectSettings;
use settings::{Settings as _, update_settings_file};
use theme::{FontFamilyCache, FontFamilyName, ThemeSettings};
use ui::{
    ButtonLike, ContextMenu, DropdownMenu, NumericStepper, SwitchField, ToggleButtonGroup,
    ToggleButtonGroupStyle, ToggleButtonSimple, ToggleState, Tooltip, prelude::*,
};

use crate::{ImportCursorSettings, ImportVsCodeSettings, SettingsImportState};

fn read_show_mini_map(cx: &App) -> ShowMinimap {
    editor::EditorSettings::get_global(cx).minimap.show
}

fn write_show_mini_map(show: ShowMinimap, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    // This is used to speed up the UI
    // the UI reads the current values to get what toggle state to show on buttons
    // there's a slight delay if we just call update_settings_file so we manually set
    // the value here then call update_settings file to get around the delay
    let mut curr_settings = EditorSettings::get_global(cx).clone();
    curr_settings.minimap.show = show;
    EditorSettings::override_global(curr_settings, cx);

    update_settings_file::<EditorSettings>(fs, cx, move |editor_settings, _| {
        editor_settings.minimap.get_or_insert_default().show = Some(show);
    });
}

fn read_inlay_hints(cx: &App) -> bool {
    AllLanguageSettings::get_global(cx)
        .defaults
        .inlay_hints
        .enabled
}

fn write_inlay_hints(enabled: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    let mut curr_settings = AllLanguageSettings::get_global(cx).clone();
    curr_settings.defaults.inlay_hints.enabled = enabled;
    AllLanguageSettings::override_global(curr_settings, cx);

    update_settings_file::<AllLanguageSettings>(fs, cx, move |all_language_settings, cx| {
        all_language_settings
            .defaults
            .inlay_hints
            .get_or_insert_with(|| {
                AllLanguageSettings::get_global(cx)
                    .clone()
                    .defaults
                    .inlay_hints
            })
            .enabled = enabled;
    });
}

fn read_git_blame(cx: &App) -> bool {
    ProjectSettings::get_global(cx).git.inline_blame_enabled()
}

fn set_git_blame(enabled: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    let mut curr_settings = ProjectSettings::get_global(cx).clone();
    curr_settings
        .git
        .inline_blame
        .get_or_insert_default()
        .enabled = enabled;
    ProjectSettings::override_global(curr_settings, cx);

    update_settings_file::<ProjectSettings>(fs, cx, move |project_settings, _| {
        project_settings
            .git
            .inline_blame
            .get_or_insert_default()
            .enabled = enabled;
    });
}

fn write_ui_font_family(font: SharedString, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<ThemeSettings>(fs, cx, move |theme_settings, _| {
        theme_settings.ui_font_family = Some(FontFamilyName(font.into()));
    });
}

fn write_ui_font_size(size: Pixels, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<ThemeSettings>(fs, cx, move |theme_settings, _| {
        theme_settings.ui_font_size = Some(size.into());
    });
}

fn write_buffer_font_size(size: Pixels, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<ThemeSettings>(fs, cx, move |theme_settings, _| {
        theme_settings.buffer_font_size = Some(size.into());
    });
}

fn write_buffer_font_family(font_family: SharedString, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<ThemeSettings>(fs, cx, move |theme_settings, _| {
        theme_settings.buffer_font_family = Some(FontFamilyName(font_family.into()));
    });
}

fn read_font_ligatures(cx: &App) -> bool {
    ThemeSettings::get_global(cx)
        .buffer_font
        .features
        .is_calt_enabled()
        .unwrap_or(true)
}

fn write_font_ligatures(enabled: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);
    let bit = if enabled { 1 } else { 0 };

    update_settings_file::<ThemeSettings>(fs, cx, move |theme_settings, _| {
        let mut features = theme_settings
            .buffer_font_features
            .as_mut()
            .map(|features| features.tag_value_list().to_vec())
            .unwrap_or_default();

        if let Some(calt_index) = features.iter().position(|(tag, _)| tag == "calt") {
            features[calt_index].1 = bit;
        } else {
            features.push(("calt".into(), bit));
        }

        theme_settings.buffer_font_features = Some(FontFeatures(Arc::new(features)));
    });
}

fn read_format_on_save(cx: &App) -> bool {
    match AllLanguageSettings::get_global(cx).defaults.format_on_save {
        FormatOnSave::On | FormatOnSave::List(_) => true,
        FormatOnSave::Off => false,
    }
}

fn write_format_on_save(format_on_save: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<AllLanguageSettings>(fs, cx, move |language_settings, _| {
        language_settings.defaults.format_on_save = Some(match format_on_save {
            true => FormatOnSave::On,
            false => FormatOnSave::Off,
        });
    });
}

fn render_setting_import_button(
    label: SharedString,
    icon_name: IconName,
    action: &dyn Action,
    imported: bool,
) -> impl IntoElement {
    let action = action.boxed_clone();
    h_flex().w_full().child(
        ButtonLike::new(label.clone())
            .full_width()
            .style(ButtonStyle::Outlined)
            .size(ButtonSize::Large)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .px_1()
                            .child(
                                Icon::new(icon_name)
                                    .color(Color::Muted)
                                    .size(IconSize::XSmall),
                            )
                            .child(Label::new(label)),
                    )
                    .when(imported, |this| {
                        this.child(
                            h_flex()
                                .gap_1p5()
                                .child(
                                    Icon::new(IconName::Check)
                                        .color(Color::Success)
                                        .size(IconSize::XSmall),
                                )
                                .child(Label::new("Imported").size(LabelSize::Small)),
                        )
                    }),
            )
            .on_click(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx)),
    )
}

fn render_import_settings_section(cx: &App) -> impl IntoElement {
    let import_state = SettingsImportState::global(cx);
    let imports: [(SharedString, IconName, &dyn Action, bool); 2] = [
        (
            "VS Code".into(),
            IconName::EditorVsCode,
            &ImportVsCodeSettings { skip_prompt: false },
            import_state.vscode,
        ),
        (
            "Cursor".into(),
            IconName::EditorCursor,
            &ImportCursorSettings { skip_prompt: false },
            import_state.cursor,
        ),
    ];

    let [vscode, cursor] = imports.map(|(label, icon_name, action, imported)| {
        render_setting_import_button(label, icon_name, action, imported)
    });

    v_flex()
        .gap_4()
        .child(
            v_flex()
                .child(Label::new("Import Settings").size(LabelSize::Large))
                .child(
                    Label::new("Automatically pull your settings from other editors.")
                        .color(Color::Muted),
                ),
        )
        .child(h_flex().w_full().gap_4().child(vscode).child(cursor))
}

fn render_font_customization_section(window: &mut Window, cx: &mut App) -> impl IntoElement {
    let theme_settings = ThemeSettings::get_global(cx);
    let ui_font_size = theme_settings.ui_font_size(cx);
    let font_family = theme_settings.buffer_font.family.clone();
    let buffer_font_size = theme_settings.buffer_font_size(cx);

    h_flex()
        .w_full()
        .gap_4()
        .child(
            v_flex()
                .w_full()
                .gap_1()
                .child(Label::new("UI Font"))
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_2()
                        .child(
                            DropdownMenu::new(
                                "ui-font-family",
                                theme_settings.ui_font.family.clone(),
                                ContextMenu::build(window, cx, |mut menu, _, cx| {
                                    let font_family_cache = FontFamilyCache::global(cx);

                                    for font_name in font_family_cache.list_font_families(cx) {
                                        menu = menu.custom_entry(
                                            {
                                                let font_name = font_name.clone();
                                                move |_window, _cx| {
                                                    Label::new(font_name.clone()).into_any_element()
                                                }
                                            },
                                            {
                                                let font_name = font_name.clone();
                                                move |_window, cx| {
                                                    write_ui_font_family(font_name.clone(), cx);
                                                }
                                            },
                                        )
                                    }

                                    menu
                                }),
                            )
                            .style(ui::DropdownStyle::Outlined)
                            .full_width(true),
                        )
                        .child(
                            NumericStepper::new(
                                "ui-font-size",
                                ui_font_size.to_string(),
                                move |_, _, cx| {
                                    write_ui_font_size(ui_font_size - px(1.), cx);
                                },
                                move |_, _, cx| {
                                    write_ui_font_size(ui_font_size + px(1.), cx);
                                },
                            )
                            .style(ui::NumericStepperStyle::Outlined),
                        ),
                ),
        )
        .child(
            v_flex()
                .w_full()
                .gap_1()
                .child(Label::new("Editor Font"))
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_2()
                        .child(
                            DropdownMenu::new(
                                "buffer-font-family",
                                font_family,
                                ContextMenu::build(window, cx, |mut menu, _, cx| {
                                    let font_family_cache = FontFamilyCache::global(cx);

                                    for font_name in font_family_cache.list_font_families(cx) {
                                        menu = menu.custom_entry(
                                            {
                                                let font_name = font_name.clone();
                                                move |_window, _cx| {
                                                    Label::new(font_name.clone()).into_any_element()
                                                }
                                            },
                                            {
                                                let font_name = font_name.clone();
                                                move |_window, cx| {
                                                    write_buffer_font_family(font_name.clone(), cx);
                                                }
                                            },
                                        )
                                    }

                                    menu
                                }),
                            )
                            .style(ui::DropdownStyle::Outlined)
                            .full_width(true),
                        )
                        .child(
                            NumericStepper::new(
                                "buffer-font-size",
                                buffer_font_size.to_string(),
                                move |_, _, cx| {
                                    write_buffer_font_size(buffer_font_size - px(1.), cx);
                                },
                                move |_, _, cx| {
                                    write_buffer_font_size(buffer_font_size + px(1.), cx);
                                },
                            )
                            .style(ui::NumericStepperStyle::Outlined),
                        ),
                ),
        )
}

fn render_popular_settings_section(window: &mut Window, cx: &mut App) -> impl IntoElement {
    const LIGATURE_TOOLTIP: &'static str = "Ligatures are when a font creates a special character out of combining two characters into one. For example, with ligatures turned on, =/= would become ≠.";

    v_flex()
        .gap_5()
        .child(Label::new("Popular Settings").size(LabelSize::Large).mt_8())
        .child(render_font_customization_section(window, cx))
        .child(
            SwitchField::new(
                "onboarding-font-ligatures",
                "Font Ligatures",
                Some("Combine text characters into their associated symbols.".into()),
                if read_font_ligatures(cx) {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                |toggle_state, _, cx| {
                    write_font_ligatures(toggle_state == &ToggleState::Selected, cx);
                },
            )
            .tooltip(Tooltip::text(LIGATURE_TOOLTIP)),
        )
        .child(SwitchField::new(
            "onboarding-format-on-save",
            "Format on Save",
            Some("Format code automatically when saving.".into()),
            if read_format_on_save(cx) {
                ui::ToggleState::Selected
            } else {
                ui::ToggleState::Unselected
            },
            |toggle_state, _, cx| {
                write_format_on_save(toggle_state == &ToggleState::Selected, cx);
            },
        ))
        .child(SwitchField::new(
            "onboarding-enable-inlay-hints",
            "Inlay Hints",
            Some("See parameter names for function and method calls inline.".into()),
            if read_inlay_hints(cx) {
                ui::ToggleState::Selected
            } else {
                ui::ToggleState::Unselected
            },
            |toggle_state, _, cx| {
                write_inlay_hints(toggle_state == &ToggleState::Selected, cx);
            },
        ))
        .child(SwitchField::new(
            "onboarding-git-blame-switch",
            "Git Blame",
            Some("See who committed each line on a given file.".into()),
            if read_git_blame(cx) {
                ui::ToggleState::Selected
            } else {
                ui::ToggleState::Unselected
            },
            |toggle_state, _, cx| {
                set_git_blame(toggle_state == &ToggleState::Selected, cx);
            },
        ))
        .child(
            h_flex()
                .items_start()
                .justify_between()
                .child(
                    v_flex().child(Label::new("Mini Map")).child(
                        Label::new("See a high-level overview of your source code.")
                            .color(Color::Muted),
                    ),
                )
                .child(
                    ToggleButtonGroup::single_row(
                        "onboarding-show-mini-map",
                        [
                            ToggleButtonSimple::new("Auto", |_, _, cx| {
                                write_show_mini_map(ShowMinimap::Auto, cx);
                            }),
                            ToggleButtonSimple::new("Always", |_, _, cx| {
                                write_show_mini_map(ShowMinimap::Always, cx);
                            }),
                            ToggleButtonSimple::new("Never", |_, _, cx| {
                                write_show_mini_map(ShowMinimap::Never, cx);
                            }),
                        ],
                    )
                    .selected_index(match read_show_mini_map(cx) {
                        ShowMinimap::Auto => 0,
                        ShowMinimap::Always => 1,
                        ShowMinimap::Never => 2,
                    })
                    .style(ToggleButtonGroupStyle::Outlined)
                    .button_width(ui::rems_from_px(64.)),
                ),
        )
}

pub(crate) fn render_editing_page(window: &mut Window, cx: &mut App) -> impl IntoElement {
    v_flex()
        .gap_4()
        .child(render_import_settings_section(cx))
        .child(render_popular_settings_section(window, cx))
}
