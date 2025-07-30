use editor::{EditorSettings, ShowMinimap};
use fs::Fs;
use gpui::{App, IntoElement, Pixels, Window};
use language::language_settings::AllLanguageSettings;
use project::project_settings::ProjectSettings;
use settings::{Settings as _, update_settings_file};
use theme::{FontFamilyCache, FontFamilyName, ThemeSettings};
use ui::{
    ContextMenu, DropdownMenu, IconButton, Label, LabelCommon, LabelSize, NumericStepper,
    ParentElement, SharedString, Styled, SwitchColor, SwitchField, ToggleButtonGroup,
    ToggleButtonGroupStyle, ToggleButtonSimple, ToggleState, div, h_flex, px, v_flex,
};

fn read_show_mini_map(cx: &App) -> ShowMinimap {
    editor::EditorSettings::get_global(cx).minimap.show
}

fn write_show_mini_map(show: ShowMinimap, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

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

pub(crate) fn render_editing_page(window: &mut Window, cx: &mut App) -> impl IntoElement {
    let theme_settings = ThemeSettings::get_global(cx);
    let ui_font_size = theme_settings.ui_font_size(cx);
    let font_family = theme_settings.buffer_font.family.clone();
    let buffer_font_size = theme_settings.buffer_font_size(cx);

    v_flex()
        .gap_4()
        .child(Label::new("Import Settings").size(LabelSize::Large))
        .child(
            Label::new("Automatically pull your settings from other editors.")
                .size(LabelSize::Small),
        )
        .child(
            h_flex()
                .child(IconButton::new(
                    "import-vs-code-settings",
                    ui::IconName::Code,
                ))
                .child(IconButton::new(
                    "import-cursor-settings",
                    ui::IconName::CursorIBeam,
                )),
        )
        .child(Label::new("Popular Settings").size(LabelSize::Large))
        .child(
            h_flex()
                .gap_4()
                .justify_between()
                .child(
                    v_flex()
                        .justify_between()
                        .gap_1()
                        .child(Label::new("UI Font"))
                        .child(
                            h_flex()
                                .justify_between()
                                .gap_2()
                                .child(div().min_w(px(120.)).child(DropdownMenu::new(
                                    "ui-font-family",
                                    theme_settings.ui_font.family.clone(),
                                    ContextMenu::build(window, cx, |mut menu, _, cx| {
                                        let font_family_cache = FontFamilyCache::global(cx);

                                        for font_name in font_family_cache.list_font_families(cx) {
                                            menu = menu.custom_entry(
                                                {
                                                    let font_name = font_name.clone();
                                                    move |_window, _cx| {
                                                        Label::new(font_name.clone())
                                                            .into_any_element()
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
                                )))
                                .child(NumericStepper::new(
                                    "ui-font-size",
                                    ui_font_size.to_string(),
                                    move |_, _, cx| {
                                        write_ui_font_size(ui_font_size - px(1.), cx);
                                    },
                                    move |_, _, cx| {
                                        write_ui_font_size(ui_font_size + px(1.), cx);
                                    },
                                )),
                        ),
                )
                .child(
                    v_flex()
                        .justify_between()
                        .gap_1()
                        .child(Label::new("Editor Font"))
                        .child(
                            h_flex()
                                .justify_between()
                                .gap_2()
                                .child(DropdownMenu::new(
                                    "buffer-font-family",
                                    font_family,
                                    ContextMenu::build(window, cx, |mut menu, _, cx| {
                                        let font_family_cache = FontFamilyCache::global(cx);

                                        for font_name in font_family_cache.list_font_families(cx) {
                                            menu = menu.custom_entry(
                                                {
                                                    let font_name = font_name.clone();
                                                    move |_window, _cx| {
                                                        Label::new(font_name.clone())
                                                            .into_any_element()
                                                    }
                                                },
                                                {
                                                    let font_name = font_name.clone();
                                                    move |_window, cx| {
                                                        write_buffer_font_family(
                                                            font_name.clone(),
                                                            cx,
                                                        );
                                                    }
                                                },
                                            )
                                        }

                                        menu
                                    }),
                                ))
                                .child(NumericStepper::new(
                                    "buffer-font-size",
                                    buffer_font_size.to_string(),
                                    move |_, _, cx| {
                                        write_buffer_font_size(buffer_font_size - px(1.), cx);
                                    },
                                    move |_, _, cx| {
                                        write_buffer_font_size(buffer_font_size + px(1.), cx);
                                    },
                                )),
                        ),
                ),
        )
        .child(
            h_flex()
                .justify_between()
                .child(Label::new("Mini Map"))
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
        .child(
            SwitchField::new(
                "onboarding-enable-inlay-hints",
                "Inlay Hints",
                "See parameter names for function and method calls inline.",
                if read_inlay_hints(cx) {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                |toggle_state, _, cx| {
                    write_inlay_hints(toggle_state == &ToggleState::Selected, cx);
                },
            )
            .color(SwitchColor::Accent),
        )
        .child(
            SwitchField::new(
                "onboarding-git-blame-switch",
                "Git Blame",
                "See who committed each line on a given file.",
                if read_git_blame(cx) {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                |toggle_state, _, cx| {
                    set_git_blame(toggle_state == &ToggleState::Selected, cx);
                },
            )
            .color(SwitchColor::Accent),
        )
}
