use client::TelemetrySettings;
use fs::Fs;
use gpui::{App, Entity, IntoElement, Window};
use settings::{BaseKeymap, Settings, update_settings_file};
use theme::{Appearance, ThemeMode, ThemeName, ThemeRegistry, ThemeSelection, ThemeSettings};
use ui::{
    ParentElement as _, StatefulInteractiveElement, SwitchField, ToggleButtonGroup,
    ToggleButtonSimple, ToggleButtonWithIcon, prelude::*, rems_from_px,
};
use vim_mode_setting::VimModeSetting;

use crate::theme_preview::ThemePreviewTile;

/// separates theme "mode" ("dark" | "light" | "system") into two separate states
/// - appearance = "dark" | "light"
/// - "system" true/false
/// when system selected:
///  - toggling between light and dark does not change theme.mode, just which variant will be changed
/// when system not selected:
///  - toggling between light and dark does change theme.mode
/// selecting a theme preview will always change theme.["light" | "dark"] to the selected theme,
///
/// this allows for selecting a dark and light theme option regardless of whether the mode is set to system or not
/// it does not support setting theme to a static value
fn render_theme_section(window: &mut Window, cx: &mut App) -> impl IntoElement {
    let theme_selection = ThemeSettings::get_global(cx).theme_selection.clone();
    let system_appearance = theme::SystemAppearance::global(cx);
    let appearance_state = window.use_state(cx, |_, _cx| {
        theme_selection
            .as_ref()
            .and_then(|selection| selection.mode())
            .and_then(|mode| match mode {
                ThemeMode::System => None,
                ThemeMode::Light => Some(Appearance::Light),
                ThemeMode::Dark => Some(Appearance::Dark),
            })
            .unwrap_or(*system_appearance)
    });
    let appearance = *appearance_state.read(cx);
    let theme_selection = theme_selection.unwrap_or_else(|| ThemeSelection::Dynamic {
        mode: match *system_appearance {
            Appearance::Light => ThemeMode::Light,
            Appearance::Dark => ThemeMode::Dark,
        },
        light: ThemeName("One Light".into()),
        dark: ThemeName("One Dark".into()),
    });
    let theme_registry = ThemeRegistry::global(cx);

    let current_theme_name = theme_selection.theme(appearance);
    let theme_mode = theme_selection.mode().unwrap_or_default();

    // let theme_mode = theme_selection.mode();
    // TODO: Clean this up once the "System" button inside the
    // toggle button group is done

    let selected_index = match appearance {
        Appearance::Light => 0,
        Appearance::Dark => 1,
    };

    let theme_seed = 0xBEEF as f32;

    const LIGHT_THEMES: [&'static str; 3] = ["One Light", "Ayu Light", "Gruvbox Light"];
    const DARK_THEMES: [&'static str; 3] = ["One Dark", "Ayu Dark", "Gruvbox Dark"];

    let theme_names = match appearance {
        Appearance::Light => LIGHT_THEMES,
        Appearance::Dark => DARK_THEMES,
    };
    let themes = theme_names
        .map(|theme_name| theme_registry.get(theme_name))
        .map(Result::unwrap);

    let theme_previews = themes.map(|theme| {
        let is_selected = theme.name == current_theme_name;
        let name = theme.name.clone();
        let colors = cx.theme().colors();

        v_flex()
            .id(name.clone())
            .w_full()
            .items_center()
            .gap_1()
            .child(
                div()
                    .w_full()
                    .border_2()
                    .border_color(colors.border_transparent)
                    .rounded(ThemePreviewTile::CORNER_RADIUS)
                    .map(|this| {
                        if is_selected {
                            this.border_color(colors.border_selected)
                        } else {
                            this.opacity(0.8).hover(|s| s.border_color(colors.border))
                        }
                    })
                    .child(ThemePreviewTile::new(theme.clone(), theme_seed)),
            )
            .child(Label::new(name).color(Color::Muted).size(LabelSize::Small))
            .on_click({
                let theme_name = theme.name.clone();
                move |_, _, cx| {
                    let fs = <dyn Fs>::global(cx);
                    let theme_name = theme_name.clone();
                    update_settings_file::<ThemeSettings>(fs, cx, move |settings, _| {
                        settings.set_theme(theme_name, appearance);
                    });
                }
            })
    });

    return v_flex()
        .gap_2()
        .child(
            h_flex().justify_between().child(Label::new("Theme")).child(
                ToggleButtonGroup::single_row(
                    "theme-selector-onboarding-dark-light",
                    [
                        ToggleButtonSimple::new("Light", {
                            let appearance_state = appearance_state.clone();
                            move |_, _, cx| {
                                write_appearance_change(&appearance_state, Appearance::Light, cx);
                            }
                        }),
                        ToggleButtonSimple::new("Dark", {
                            let appearance_state = appearance_state.clone();
                            move |_, _, cx| {
                                write_appearance_change(&appearance_state, Appearance::Dark, cx);
                            }
                        }),
                        // TODO: Properly put the System back as a button within this group
                        // Currently, given "System" is not an option in the Appearance enum,
                        // this button doesn't get selected
                        ToggleButtonSimple::new("System", {
                            let theme = theme_selection.clone();
                            move |_, _, cx| {
                                toggle_system_theme_mode(theme.clone(), appearance, cx);
                            }
                        })
                        .selected(theme_mode == ThemeMode::System),
                    ],
                )
                .selected_index(selected_index)
                .style(ui::ToggleButtonGroupStyle::Outlined)
                .button_width(rems_from_px(64.)),
            ),
        )
        .child(h_flex().gap_4().justify_between().children(theme_previews));

    fn write_appearance_change(
        appearance_state: &Entity<Appearance>,
        new_appearance: Appearance,
        cx: &mut App,
    ) {
        let fs = <dyn Fs>::global(cx);
        appearance_state.write(cx, new_appearance);

        update_settings_file::<ThemeSettings>(fs, cx, move |settings, _| {
            if settings.theme.as_ref().and_then(ThemeSelection::mode) == Some(ThemeMode::System) {
                return;
            }
            let new_mode = match new_appearance {
                Appearance::Light => ThemeMode::Light,
                Appearance::Dark => ThemeMode::Dark,
            };
            settings.set_mode(new_mode);
        });
    }

    fn toggle_system_theme_mode(
        theme_selection: ThemeSelection,
        appearance: Appearance,
        cx: &mut App,
    ) {
        let fs = <dyn Fs>::global(cx);

        update_settings_file::<ThemeSettings>(fs, cx, move |settings, _| {
            settings.theme = Some(match theme_selection {
                ThemeSelection::Static(theme_name) => ThemeSelection::Dynamic {
                    mode: ThemeMode::System,
                    light: theme_name.clone(),
                    dark: theme_name.clone(),
                },
                ThemeSelection::Dynamic {
                    mode: ThemeMode::System,
                    light,
                    dark,
                } => {
                    let mode = match appearance {
                        Appearance::Light => ThemeMode::Light,
                        Appearance::Dark => ThemeMode::Dark,
                    };
                    ThemeSelection::Dynamic { mode, light, dark }
                }
                ThemeSelection::Dynamic {
                    mode: _,
                    light,
                    dark,
                } => ThemeSelection::Dynamic {
                    mode: ThemeMode::System,
                    light,
                    dark,
                },
            });
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
