use fs::Fs;
use gpui::{App, Entity, IntoElement, Window};
use settings::{Settings, update_settings_file};
use theme::{Appearance, ThemeMode, ThemeName, ThemeRegistry, ThemeSelection, ThemeSettings};
use ui::{
    ParentElement as _, StatefulInteractiveElement, SwitchField, ToggleButtonGroup,
    ToggleButtonSimple, ToggleButtonWithIcon, prelude::*, rems_from_px,
};

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
    let theme_mode = theme_selection.mode();

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
        v_flex()
            .id(name.clone())
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
            .flex_1()
            .child(crate::theme_preview::ThemePreviewTile::new(
                theme,
                is_selected,
                theme_seed,
            ))
            .child(
                h_flex()
                    .justify_center()
                    .items_baseline()
                    .child(Label::new(name).color(Color::Muted)),
            )
    });

    return v_flex()
        .child(
            h_flex().justify_between().child(Label::new("Theme")).child(
                h_flex()
                    .gap_2()
                    .child(
                        ToggleButtonGroup::single_row(
                            "theme-selector-onboarding-dark-light",
                            [
                                ToggleButtonSimple::new("Light", {
                                    let appearance_state = appearance_state.clone();
                                    move |_, _, cx| {
                                        write_appearance_change(
                                            &appearance_state,
                                            Appearance::Light,
                                            cx,
                                        );
                                    }
                                }),
                                ToggleButtonSimple::new("Dark", {
                                    let appearance_state = appearance_state.clone();
                                    move |_, _, cx| {
                                        write_appearance_change(
                                            &appearance_state,
                                            Appearance::Dark,
                                            cx,
                                        );
                                    }
                                }),
                            ],
                        )
                        .selected_index(selected_index)
                        .style(ui::ToggleButtonGroupStyle::Outlined)
                        .button_width(rems_from_px(64.)),
                    )
                    .child(
                        ToggleButtonGroup::single_row(
                            "theme-selector-onboarding-system",
                            [ToggleButtonSimple::new("System", {
                                let theme = theme_selection.clone();
                                move |_, _, cx| {
                                    toggle_system_theme_mode(theme.clone(), appearance, cx);
                                }
                            })],
                        )
                        .selected_index((theme_mode != Some(ThemeMode::System)) as usize)
                        .style(ui::ToggleButtonGroupStyle::Outlined)
                        .button_width(rems_from_px(64.)),
                    ),
            ),
        )
        .child(h_flex().justify_between().children(theme_previews));

    fn write_appearance_change(
        appearance_state: &Entity<Appearance>,
        new_appearance: Appearance,
        cx: &mut App,
    ) {
        appearance_state.update(cx, |appearance, _| {
            *appearance = new_appearance;
        });
        let fs = <dyn Fs>::global(cx);

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

fn render_telemetry_section() -> impl IntoElement {
    v_flex()
        .gap_3()
        .child(Label::new("Telemetry").size(LabelSize::Large))
        .child(SwitchField::new(
            "vim_mode",
            "Help Improve Zed",
            "Sending anonymous usage data helps us build the right features and create the best experience.",
            ui::ToggleState::Selected,
            |_, _, _| {},
        ))
        .child(SwitchField::new(
            "vim_mode",
            "Help Fix Zed",
            "Send crash reports so we can fix critical issues fast.",
            ui::ToggleState::Selected,
            |_, _, _| {},
        ))
}

pub(crate) fn render_basics_page(window: &mut Window, cx: &mut App) -> impl IntoElement {
    v_flex()
        .gap_6()
        .child(render_theme_section(window, cx))
        .child(
            v_flex().gap_2().child(Label::new("Base Keymap")).child(
                ToggleButtonGroup::two_rows(
                    "multiple_row_test",
                    [
                        ToggleButtonWithIcon::new("VS Code", IconName::AiZed, |_, _, _| {}),
                        ToggleButtonWithIcon::new("Jetbrains", IconName::AiZed, |_, _, _| {}),
                        ToggleButtonWithIcon::new("Sublime Text", IconName::AiZed, |_, _, _| {}),
                    ],
                    [
                        ToggleButtonWithIcon::new("Atom", IconName::AiZed, |_, _, _| {}),
                        ToggleButtonWithIcon::new("Emacs", IconName::AiZed, |_, _, _| {}),
                        ToggleButtonWithIcon::new("Cursor (Beta)", IconName::AiZed, |_, _, _| {}),
                    ],
                )
                .button_width(rems_from_px(230.))
                .style(ui::ToggleButtonGroupStyle::Outlined)
            ),
        )
        .child(v_flex().justify_center().child(div().h_0().child("hack").invisible()).child(SwitchField::new(
            "vim_mode",
            "Vim Mode",
            "Coming from Neovim? Zed's first-class implementation of Vim Mode has got your back.",
            ui::ToggleState::Selected,
            |_, _, _| {},
        )))
        .child(render_telemetry_section())
}
