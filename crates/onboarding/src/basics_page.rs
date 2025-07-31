use fs::Fs;
use gpui::{App, IntoElement, Window};
use settings::{Settings, update_settings_file};
use theme::{ThemeMode, ThemeSettings};
use ui::{SwitchField, ToggleButtonGroup, ToggleButtonSimple, ToggleButtonWithIcon, prelude::*};

fn read_theme_selection(cx: &App) -> ThemeMode {
    let settings = ThemeSettings::get_global(cx);
    settings
        .theme_selection
        .as_ref()
        .and_then(|selection| selection.mode())
        .unwrap_or_default()
}

fn write_theme_selection(theme_mode: ThemeMode, cx: &App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<ThemeSettings>(fs, cx, move |settings, _| {
        settings.set_mode(theme_mode);
    });
}

fn render_theme_section(cx: &mut App) -> impl IntoElement {
    let theme_mode = read_theme_selection(cx);

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

pub(crate) fn render_basics_page(_: &mut Window, cx: &mut App) -> impl IntoElement {
    v_flex()
        .gap_6()
        .child(render_theme_section(cx))
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
                .button_width(rems_from_px(200.))
                .style(ui::ToggleButtonGroupStyle::Filled),
            ),
        )
        .child(v_flex().child(div().child("hack").invisible()).child(SwitchField::new(
            "vim_mode",
            "Vim Mode",
            "Coming from Neovim? Zed's first-class implementation of Vim Mode has got your back.",
            ui::ToggleState::Selected,
            |_, _, _| {},
        )))
        .child(render_telemetry_section())
}
