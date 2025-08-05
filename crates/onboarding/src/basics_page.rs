use std::sync::Arc;

use client::TelemetrySettings;
use fs::Fs;
use gpui::{App, IntoElement, Window};
use settings::{BaseKeymap, Settings, update_settings_file};
use theme::{
    Appearance, SystemAppearance, ThemeMode, ThemeName, ThemeRegistry, ThemeSelection,
    ThemeSettings,
};
use ui::{
    ParentElement as _, StatefulInteractiveElement, SwitchField, ToggleButtonGroup,
    ToggleButtonSimple, ToggleButtonWithIcon, prelude::*, rems_from_px,
};
use vim_mode_setting::VimModeSetting;

use crate::theme_preview::{ThemePreviewStyle, ThemePreviewTile};

fn render_theme_section(_window: &mut Window, cx: &mut App) -> impl IntoElement {
    let theme_selection = ThemeSettings::get_global(cx).theme_selection.clone();
    let system_appearance = theme::SystemAppearance::global(cx);
    let theme_selection = theme_selection.unwrap_or_else(|| ThemeSelection::Dynamic {
        mode: match *system_appearance {
            Appearance::Light => ThemeMode::Light,
            Appearance::Dark => ThemeMode::Dark,
        },
        light: ThemeName("One Light".into()),
        dark: ThemeName("One Dark".into()),
    });

    let theme_mode = theme_selection
        .mode()
        .unwrap_or_else(|| match *system_appearance {
            Appearance::Light => ThemeMode::Light,
            Appearance::Dark => ThemeMode::Dark,
        });

    return v_flex()
        .gap_2()
        .child(
            h_flex().justify_between().child(Label::new("Theme")).child(
                ToggleButtonGroup::single_row(
                    "theme-selector-onboarding-dark-light",
                    [ThemeMode::Light, ThemeMode::Dark, ThemeMode::System].map(|mode| {
                        const MODE_NAMES: [SharedString; 3] = [
                            SharedString::new_static("Light"),
                            SharedString::new_static("Dark"),
                            SharedString::new_static("System"),
                        ];
                        ToggleButtonSimple::new(
                            MODE_NAMES[mode as usize].clone(),
                            move |_, _, cx| {
                                write_mode_change(mode, cx);
                            },
                        )
                    }),
                )
                .selected_index(theme_mode as usize)
                .style(ui::ToggleButtonGroupStyle::Outlined)
                .button_width(rems_from_px(64.)),
            ),
        )
        .child(
            h_flex()
                .gap_4()
                .justify_between()
                .children(render_theme_previews(&theme_selection, cx)),
        );

    fn render_theme_previews(
        theme_selection: &ThemeSelection,
        cx: &mut App,
    ) -> [impl IntoElement; 3] {
        let system_appearance = SystemAppearance::global(cx);
        let theme_registry = ThemeRegistry::global(cx);

        let theme_seed = 0xBEEF as f32;
        let theme_mode = theme_selection
            .mode()
            .unwrap_or_else(|| match *system_appearance {
                Appearance::Light => ThemeMode::Light,
                Appearance::Dark => ThemeMode::Dark,
            });
        let appearance = match theme_mode {
            ThemeMode::Light => Appearance::Light,
            ThemeMode::Dark => Appearance::Dark,
            ThemeMode::System => *system_appearance,
        };
        let current_theme_name = theme_selection.theme(appearance);

        const LIGHT_THEMES: [&'static str; 3] = ["One Light", "Ayu Light", "Gruvbox Light"];
        const DARK_THEMES: [&'static str; 3] = ["One Dark", "Ayu Dark", "Gruvbox Dark"];
        const FAMILY_NAMES: [SharedString; 3] = [
            SharedString::new_static("One"),
            SharedString::new_static("Ayu"),
            SharedString::new_static("Gruvbox"),
        ];

        let theme_names = match appearance {
            Appearance::Light => LIGHT_THEMES,
            Appearance::Dark => DARK_THEMES,
        };

        let themes = theme_names.map(|theme| theme_registry.get(theme).unwrap());

        let theme_previews = [0, 1, 2].map(|index| {
            let theme = &themes[index];
            let is_selected = theme.name == current_theme_name;
            let name = theme.name.clone();
            let colors = cx.theme().colors();

            v_flex()
                .id(name.clone())
                .w_full()
                .items_center()
                .gap_1()
                .child(
                    h_flex()
                        .relative()
                        .w_full()
                        .border_2()
                        .border_color(colors.border_transparent)
                        .rounded(ThemePreviewTile::ROOT_RADIUS)
                        .map(|this| {
                            if is_selected {
                                this.border_color(colors.border_selected)
                            } else {
                                this.opacity(0.8).hover(|s| s.border_color(colors.border))
                            }
                        })
                        .map(|this| {
                            if theme_mode == ThemeMode::System {
                                let (light, dark) = (
                                    theme_registry.get(LIGHT_THEMES[index]).unwrap(),
                                    theme_registry.get(DARK_THEMES[index]).unwrap(),
                                );
                                this.child(
                                    ThemePreviewTile::new(light, theme_seed)
                                        .style(ThemePreviewStyle::SideBySide(dark)),
                                )
                            } else {
                                this.child(
                                    ThemePreviewTile::new(theme.clone(), theme_seed)
                                        .style(ThemePreviewStyle::Bordered),
                                )
                            }
                        }),
                )
                .child(
                    Label::new(FAMILY_NAMES[index].clone())
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .on_click({
                    let theme_name = theme.name.clone();
                    move |_, _, cx| {
                        write_theme_change(theme_name.clone(), theme_mode, cx);
                    }
                })
        });

        theme_previews
    }

    fn write_mode_change(mode: ThemeMode, cx: &mut App) {
        let fs = <dyn Fs>::global(cx);
        update_settings_file::<ThemeSettings>(fs, cx, move |settings, _cx| {
            settings.set_mode(mode);
        });
    }

    fn write_theme_change(theme: impl Into<Arc<str>>, theme_mode: ThemeMode, cx: &mut App) {
        let fs = <dyn Fs>::global(cx);
        let theme = theme.into();
        update_settings_file::<ThemeSettings>(fs, cx, move |settings, cx| {
            if theme_mode == ThemeMode::System {
                settings.theme = Some(ThemeSelection::Dynamic {
                    mode: ThemeMode::System,
                    light: ThemeName(theme.clone()),
                    dark: ThemeName(theme.clone()),
                });
            } else {
                let appearance = *SystemAppearance::global(cx);
                settings.set_theme(theme.clone(), appearance);
            }
        });
    }
}

fn write_keymap_base(keymap_base: BaseKeymap, cx: &App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<BaseKeymap>(fs, cx, move |setting, _| {
        *setting = Some(keymap_base);
    });
}

fn render_telemetry_section(cx: &App) -> impl IntoElement {
    let fs = <dyn Fs>::global(cx);

    v_flex()
        .gap_4()
        .child(Label::new("Telemetry").size(LabelSize::Large))
        .child(SwitchField::new(
            "onboarding-telemetry-metrics",
            "Help Improve Zed",
            Some("Sending anonymous usage data helps us build the right features and create the best experience.".into()),
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
            Some("Send crash reports so we can fix critical issues fast.".into()),
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

pub(crate) fn render_basics_page(window: &mut Window, cx: &mut App) -> impl IntoElement {
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
         .child(render_theme_section(window, cx))
        .child(
            v_flex().gap_2().child(Label::new("Base Keymap")).child(
                ToggleButtonGroup::two_rows(
                    "multiple_row_test",
                    [
                        ToggleButtonWithIcon::new("VS Code", IconName::EditorVsCode, |_, _, cx| {
                            write_keymap_base(BaseKeymap::VSCode, cx);
                        }),
                        ToggleButtonWithIcon::new("Jetbrains", IconName::EditorJetBrains, |_, _, cx| {
                            write_keymap_base(BaseKeymap::JetBrains, cx);
                        }),
                        ToggleButtonWithIcon::new("Sublime Text", IconName::EditorSublime, |_, _, cx| {
                            write_keymap_base(BaseKeymap::SublimeText, cx);
                        }),
                    ],
                    [
                        ToggleButtonWithIcon::new("Atom", IconName::EditorAtom, |_, _, cx| {
                            write_keymap_base(BaseKeymap::Atom, cx);
                        }),
                        ToggleButtonWithIcon::new("Emacs", IconName::EditorEmacs, |_, _, cx| {
                            write_keymap_base(BaseKeymap::Emacs, cx);
                        }),
                        ToggleButtonWithIcon::new("Cursor (Beta)", IconName::EditorCursor, |_, _, cx| {
                            write_keymap_base(BaseKeymap::Cursor, cx);
                        }),
                    ],
                )
                .when_some(base_keymap, |this, base_keymap| this.selected_index(base_keymap))
                .button_width(rems_from_px(216.))
                .size(ui::ToggleButtonGroupSize::Medium)
                .style(ui::ToggleButtonGroupStyle::Outlined)
            ),
        )
        .child(SwitchField::new(
            "onboarding-vim-mode",
            "Vim Mode",
            Some("Coming from Neovim? Zed's first-class implementation of Vim Mode has got your back.".into()),
            if VimModeSetting::get_global(cx).0 {
                ui::ToggleState::Selected
            } else {
                ui::ToggleState::Unselected
            },
            {
                let fs = <dyn Fs>::global(cx);
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
        ))
        .child(render_telemetry_section(cx))
}
