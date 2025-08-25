use std::sync::Arc;

use gpui::{App, FontFeatures, FontWeight};
use settings::{EditableSettingControl, Settings};
use theme::{
    FontFamilyCache, FontFamilyName, SystemAppearance, ThemeMode, ThemeRegistry, ThemeSettings,
};
use ui::{
    CheckboxWithLabel, ContextMenu, DropdownMenu, NumericStepper, SettingsContainer, SettingsGroup,
    ToggleButton, prelude::*,
};

#[derive(IntoElement)]
pub struct AppearanceSettingsControls {}

impl AppearanceSettingsControls {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for AppearanceSettingsControls {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        SettingsContainer::new()
            .child(
                SettingsGroup::new("Theme").child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .child(ThemeControl)
                        .child(ThemeModeControl),
                ),
            )
            .child(
                SettingsGroup::new("Font")
                    .child(
                        h_flex()
                            .gap_2()
                            .justify_between()
                            .child(UiFontFamilyControl)
                            .child(UiFontWeightControl),
                    )
                    .child(UiFontSizeControl)
                    .child(UiFontLigaturesControl),
            )
    }
}

#[derive(IntoElement)]
struct ThemeControl;

impl EditableSettingControl for ThemeControl {
    type Value = String;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Theme".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        let appearance = SystemAppearance::global(cx);
        settings
            .theme_selection
            .as_ref()
            .map(|selection| selection.theme(appearance.0).to_string())
            .unwrap_or_else(|| ThemeSettings::default_theme(*appearance).to_string())
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        cx: &App,
    ) {
        let appearance = SystemAppearance::global(cx);
        settings.set_theme(value, appearance.0);
    }
}

impl RenderOnce for ThemeControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        DropdownMenu::new(
            "theme",
            value,
            ContextMenu::build(window, cx, |mut menu, _, cx| {
                let theme_registry = ThemeRegistry::global(cx);

                for theme in theme_registry.list_names() {
                    menu = menu.custom_entry(
                        {
                            let theme = theme.clone();
                            move |_window, _cx| Label::new(theme.clone()).into_any_element()
                        },
                        {
                            let theme = theme.clone();
                            move |_window, cx| {
                                Self::write(theme.to_string(), cx);
                            }
                        },
                    )
                }

                menu
            }),
        )
        .full_width(true)
    }
}

#[derive(IntoElement)]
struct ThemeModeControl;

impl EditableSettingControl for ThemeModeControl {
    type Value = ThemeMode;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Theme Mode".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings
            .theme_selection
            .as_ref()
            .and_then(|selection| selection.mode())
            .unwrap_or_default()
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.set_mode(value);
    }
}

impl RenderOnce for ThemeModeControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .child(
                ToggleButton::new("light", "Light")
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Large)
                    .toggle_state(value == ThemeMode::Light)
                    .on_click(|_, _, cx| Self::write(ThemeMode::Light, cx))
                    .first(),
            )
            .child(
                ToggleButton::new("system", "System")
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Large)
                    .toggle_state(value == ThemeMode::System)
                    .on_click(|_, _, cx| Self::write(ThemeMode::System, cx))
                    .middle(),
            )
            .child(
                ToggleButton::new("dark", "Dark")
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Large)
                    .toggle_state(value == ThemeMode::Dark)
                    .on_click(|_, _, cx| Self::write(ThemeMode::Dark, cx))
                    .last(),
            )
    }
}

#[derive(IntoElement)]
struct UiFontFamilyControl;

impl EditableSettingControl for UiFontFamilyControl {
    type Value = SharedString;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "UI Font Family".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.ui_font.family.clone()
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.ui_font_family = Some(FontFamilyName(value.into()));
    }
}

impl RenderOnce for UiFontFamilyControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::Font))
            .child(DropdownMenu::new(
                "ui-font-family",
                value,
                ContextMenu::build(window, cx, |mut menu, _, cx| {
                    let font_family_cache = FontFamilyCache::global(cx);

                    for font_name in font_family_cache.list_font_families(cx) {
                        menu = menu.custom_entry(
                            {
                                let font_name = font_name.clone();
                                move |_window, _cx| Label::new(font_name.clone()).into_any_element()
                            },
                            {
                                let font_name = font_name.clone();
                                move |_window, cx| {
                                    Self::write(font_name.clone(), cx);
                                }
                            },
                        )
                    }

                    menu
                }),
            ))
    }
}

#[derive(IntoElement)]
struct UiFontSizeControl;

impl EditableSettingControl for UiFontSizeControl {
    type Value = Pixels;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "UI Font Size".into()
    }

    fn read(cx: &App) -> Self::Value {
        ThemeSettings::get_global(cx).ui_font_size(cx)
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.ui_font_size = Some(value.into());
    }
}

impl RenderOnce for UiFontSizeControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontSize))
            .child(NumericStepper::new(
                "ui-font-size",
                value.to_string(),
                move |_, _, cx| {
                    Self::write(value - px(1.), cx);
                },
                move |_, _, cx| {
                    Self::write(value + px(1.), cx);
                },
            ))
    }
}

#[derive(IntoElement)]
struct UiFontWeightControl;

impl EditableSettingControl for UiFontWeightControl {
    type Value = FontWeight;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "UI Font Weight".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.ui_font.weight
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.ui_font_weight = Some(value.0);
    }
}

impl RenderOnce for UiFontWeightControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontWeight))
            .child(DropdownMenu::new(
                "ui-font-weight",
                value.0.to_string(),
                ContextMenu::build(window, cx, |mut menu, _window, _cx| {
                    for weight in FontWeight::ALL {
                        menu = menu.custom_entry(
                            move |_window, _cx| Label::new(weight.0.to_string()).into_any_element(),
                            {
                                move |_window, cx| {
                                    Self::write(weight, cx);
                                }
                            },
                        )
                    }

                    menu
                }),
            ))
    }
}

#[derive(IntoElement)]
struct UiFontLigaturesControl;

impl EditableSettingControl for UiFontLigaturesControl {
    type Value = bool;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "UI Font Ligatures".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.ui_font.features.is_calt_enabled().unwrap_or(true)
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        let value = if value { 1 } else { 0 };

        let mut features = settings
            .ui_font_features
            .as_ref()
            .map(|features| features.tag_value_list().to_vec())
            .unwrap_or_default();

        if let Some(calt_index) = features.iter().position(|(tag, _)| tag == "calt") {
            features[calt_index].1 = value;
        } else {
            features.push(("calt".into(), value));
        }

        settings.ui_font_features = Some(FontFeatures(Arc::new(features)));
    }
}

impl RenderOnce for UiFontLigaturesControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        CheckboxWithLabel::new(
            "ui-font-ligatures",
            Label::new(self.name()),
            value.into(),
            |selection, _, cx| {
                Self::write(
                    match selection {
                        ToggleState::Selected => true,
                        ToggleState::Unselected | ToggleState::Indeterminate => false,
                    },
                    cx,
                );
            },
        )
    }
}
