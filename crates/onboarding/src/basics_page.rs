use std::sync::Arc;

use client::TelemetrySettings;
use fs::Fs;
use gpui::{App, IntoElement};
use settings::{BaseKeymap, Settings, update_settings_file};
use theme::{Appearance, SystemAppearance, ThemeMode, ThemeSettings};
use ui::{
    SwitchField, ThemePreviewTile, ToggleButtonGroup, ToggleButtonSimple, ToggleButtonWithIcon,
    prelude::*,
};
use vim_mode_setting::VimModeSetting;

use crate::Onboarding;

fn read_theme_selection(cx: &App) -> (ThemeMode, SharedString) {
    let settings = ThemeSettings::get_global(cx);
    (
        settings
            .theme_selection
            .as_ref()
            .and_then(|selection| selection.mode())
            .unwrap_or_default(),
        settings.active_theme.name.clone(),
    )
}

fn write_theme_selection(theme_mode: ThemeMode, cx: &App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<ThemeSettings>(fs, cx, move |settings, _| {
        settings.set_mode(theme_mode);
    });
}

fn write_keymap_base(keymap_base: BaseKeymap, cx: &App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<BaseKeymap>(fs, cx, move |setting, _| {
        *setting = Some(keymap_base);
    });
}

fn render_theme_section(theme_mode: ThemeMode) -> impl IntoElement {
    h_flex().justify_between().child(Label::new("Theme")).child(
        ToggleButtonGroup::single_row(
            "theme-selector-onboarding",
            [
                ToggleButtonSimple::new("Light", |_, _, cx| {
                    write_theme_selection(ThemeMode::Light, cx)
                }),
                ToggleButtonSimple::new("Dark", |_, _, cx| {
                    write_theme_selection(ThemeMode::Dark, cx)
                }),
                ToggleButtonSimple::new("System", |_, _, cx| {
                    write_theme_selection(ThemeMode::System, cx)
                }),
            ],
        )
        .selected_index(match theme_mode {
            ThemeMode::Light => 0,
            ThemeMode::Dark => 1,
            ThemeMode::System => 2,
        })
        .style(ui::ToggleButtonGroupStyle::Outlined)
        .button_width(rems_from_px(64.)),
    )
}

fn render_telemetry_section(fs: Arc<dyn Fs>, cx: &App) -> impl IntoElement {
    v_flex()

        .gap_4()
        .child(Label::new("Telemetry").size(LabelSize::Large))
        .child(SwitchField::new(
            "onboarding-telemetry-metrics",
            "Help Improve Zed",
            "Sending anonymous usage data helps us build the right features and create the best experience.",
            if TelemetrySettings::get_global(cx).metrics {
                ui::ToggleState::Selected
            } else {
                ui::ToggleState::Unselected
            },
            {
            let fs = fs.clone();
            move |selection, _, cx| {
                let enabled = match selection {
                    ToggleState::Selected => true,
                    ToggleState::Unselected => false,
                    ToggleState::Indeterminate => { return; },
                };

                update_settings_file::<TelemetrySettings>(
                    fs.clone(),
                    cx,
                    move |setting, _| setting.metrics = Some(enabled),
                );
            }},
        ))
        .child(SwitchField::new(
            "onboarding-telemetry-crash-reports",
            "Help Fix Zed",
            "Send crash reports so we can fix critical issues fast.",
            if TelemetrySettings::get_global(cx).diagnostics {
                ui::ToggleState::Selected
            } else {
                ui::ToggleState::Unselected
            },
            {
                let fs = fs.clone();
                move |selection, _, cx| {
                    let enabled = match selection {
                        ToggleState::Selected => true,
                        ToggleState::Unselected => false,
                        ToggleState::Indeterminate => { return; },
                    };

                    update_settings_file::<TelemetrySettings>(
                        fs.clone(),
                        cx,
                        move |setting, _| setting.diagnostics = Some(enabled),
                    );
                }
            }
        ))
}

pub(crate) fn render_basics_page(onboarding: &Onboarding, cx: &mut App) -> impl IntoElement {
    let (theme_mode, active_theme_name) = read_theme_selection(cx);
    let themes = match theme_mode {
        ThemeMode::Dark => &onboarding.dark_themes,
        ThemeMode::Light => &onboarding.light_themes,
        ThemeMode::System => match SystemAppearance::global(cx).0 {
            Appearance::Light => &onboarding.light_themes,
            Appearance::Dark => &onboarding.dark_themes,
        },
    };

    let base_keymap = match BaseKeymap::get_global(cx) {
        BaseKeymap::VSCode => Some(0),
        BaseKeymap::JetBrains => Some(1),
        BaseKeymap::SublimeText => Some(2),
        BaseKeymap::Atom => Some(3),
        BaseKeymap::Emacs => Some(4),
        BaseKeymap::Cursor => Some(5),
        BaseKeymap::TextMate | BaseKeymap::None => None,
    };

    v_flex()
        .gap_6()
        .child(render_theme_section(theme_mode))
        .child(h_flex().children(
            themes.iter().map(|theme| {
                ThemePreviewTile::new(theme.clone(), active_theme_name == theme.name, 0.48)
                .on_click({
                    let theme_name = theme.name.clone();
                    let fs = onboarding.fs.clone();
                    move |_, _, cx| {
                        let theme_name = theme_name.clone();
                        update_settings_file::<ThemeSettings>(fs.clone(), cx, move |settings, cx| {
                            settings.set_theme(theme_name.to_string(), SystemAppearance::global(cx).0);
                        });
                    }
                })
            })
        ))
        .child(
            v_flex().gap_2().child(Label::new("Base Keymap")).child(
                ToggleButtonGroup::two_rows(
                    "multiple_row_test",
                    [
                        ToggleButtonWithIcon::new("VS Code", IconName::AiZed, |_, _, cx| {
                            write_keymap_base(BaseKeymap::VSCode, cx);
                        }),
                        ToggleButtonWithIcon::new("Jetbrains", IconName::AiZed, |_, _, cx| {
                            write_keymap_base(BaseKeymap::JetBrains, cx);
                        }),
                        ToggleButtonWithIcon::new("Sublime Text", IconName::AiZed, |_, _, cx| {
                            write_keymap_base(BaseKeymap::SublimeText, cx);
                        }),
                    ],
                    [
                        ToggleButtonWithIcon::new("Atom", IconName::AiZed, |_, _, cx| {
                            write_keymap_base(BaseKeymap::Atom, cx);
                        }),
                        ToggleButtonWithIcon::new("Emacs", IconName::AiZed, |_, _, cx| {
                            write_keymap_base(BaseKeymap::Emacs, cx);
                        }),
                        ToggleButtonWithIcon::new("Cursor (Beta)", IconName::AiZed, |_, _, cx| {
                            write_keymap_base(BaseKeymap::Cursor, cx);
                        }),
                    ],
                )
                .when_some(base_keymap, |this, base_keymap| this.selected_index(base_keymap))
                .button_width(rems_from_px(230.))
                .style(ui::ToggleButtonGroupStyle::Outlined)
            ),
        )
        .child(v_flex().justify_center().child(div().h_0().child("hack").invisible()).child(SwitchField::new(
            "onboarding-vim-mode",
            "Vim Mode",
            "Coming from Neovim? Zed's first-class implementation of Vim Mode has got your back.",
            if VimModeSetting::get_global(cx).0 {
                ui::ToggleState::Selected
            } else {
                ui::ToggleState::Unselected
            },
            {
                let fs = onboarding.fs.clone();
                move |selection, _, cx| {
                    let enabled = match selection {
                        ToggleState::Selected => true,
                        ToggleState::Unselected => false,
                        ToggleState::Indeterminate => { return; },
                    };

                    update_settings_file::<VimModeSetting>(
                        fs.clone(),
                        cx,
                        move |setting, _| *setting = Some(enabled),
                    );
                }
            },
        )))
        .child(render_telemetry_section(onboarding.fs.clone(), cx))
}
