use std::sync::Arc;

use gpui::{AppContext, FontFeatures, FontWeight};
use settings::{EditableSettingControl, Settings};
use theme::{FontFamilyCache, SystemAppearance, ThemeMode, ThemeRegistry, ThemeSettings};
use ui::{
    prelude::*, CheckboxWithLabel, ContextMenu, DropdownMenu, NumericStepper, SettingsContainer,
    SettingsGroup, ToggleButton,
};

#[derive(IntoElement)]
pub struct AppearanceSettingsControls {}

impl AppearanceSettingsControls {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for AppearanceSettingsControls {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
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

    fn read(cx: &AppContext) -> Self::Value {
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
        cx: &AppContext,
    ) {
        let appearance = SystemAppearance::global(cx);
        settings.set_theme(value, appearance.0);
    }
}

impl RenderOnce for ThemeControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        DropdownMenu::new(
            "theme",
            value.clone(),
            ContextMenu::build(cx, |mut menu, cx| {
                let theme_registry = ThemeRegistry::global(cx);

                for theme in theme_registry.list_names(false) {
                    menu = menu.custom_entry(
                        {
                            let theme = theme.clone();
                            move |_cx| Label::new(theme.clone()).into_any_element()
                        },
                        {
                            let theme = theme.clone();
                            move |cx| {
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

    fn read(cx: &AppContext) -> Self::Value {
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
        _cx: &AppContext,
    ) {
        settings.set_mode(value);
    }
}

impl RenderOnce for ThemeModeControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .child(
                ToggleButton::new("light", "Light")
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Large)
                    .selected(value == ThemeMode::Light)
                    .on_click(|_, cx| Self::write(ThemeMode::Light, cx))
                    .first(),
            )
            .child(
                ToggleButton::new("system", "System")
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Large)
                    .selected(value == ThemeMode::System)
                    .on_click(|_, cx| Self::write(ThemeMode::System, cx))
                    .middle(),
            )
            .child(
                ToggleButton::new("dark", "Dark")
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Large)
                    .selected(value == ThemeMode::Dark)
                    .on_click(|_, cx| Self::write(ThemeMode::Dark, cx))
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

    fn read(cx: &AppContext) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.ui_font.family.clone()
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &AppContext,
    ) {
        settings.ui_font_family = Some(value.to_string());
    }
}

impl RenderOnce for UiFontFamilyControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::Font))
            .child(DropdownMenu::new(
                "ui-font-family",
                value.clone(),
                ContextMenu::build(cx, |mut menu, cx| {
                    let font_family_cache = FontFamilyCache::global(cx);

                    for font_name in font_family_cache.list_font_families(cx) {
                        menu = menu.custom_entry(
                            {
                                let font_name = font_name.clone();
                                move |_cx| Label::new(font_name.clone()).into_any_element()
                            },
                            {
                                let font_name = font_name.clone();
                                move |cx| {
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

    fn read(cx: &AppContext) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.ui_font_size
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &AppContext,
    ) {
        settings.ui_font_size = Some(value.into());
    }
}

impl RenderOnce for UiFontSizeControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontSize))
            .child(NumericStepper::new(
                "ui-font-size",
                value.to_string(),
                move |_, cx| {
                    Self::write(value - px(1.), cx);
                },
                move |_, cx| {
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

    fn read(cx: &AppContext) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.ui_font.weight
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &AppContext,
    ) {
        settings.ui_font_weight = Some(value.0);
    }
}

impl RenderOnce for UiFontWeightControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontWeight))
            .child(DropdownMenu::new(
                "ui-font-weight",
                value.0.to_string(),
                ContextMenu::build(cx, |mut menu, _cx| {
                    for weight in FontWeight::ALL {
                        menu = menu.custom_entry(
                            move |_cx| Label::new(weight.0.to_string()).into_any_element(),
                            {
                                move |cx| {
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

    fn read(cx: &AppContext) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.ui_font.features.is_calt_enabled().unwrap_or(true)
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &AppContext,
    ) {
        let value = if value { 1 } else { 0 };

        let mut features = settings
            .ui_font_features
            .as_ref()
            .map(|features| {
                features
                    .tag_value_list()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>()
            })
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
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        CheckboxWithLabel::new(
            "ui-font-ligatures",
            Label::new(self.name()),
            value.into(),
            |selection, cx| {
                Self::write(
                    match selection {
                        Selection::Selected => true,
                        Selection::Unselected | Selection::Indeterminate => false,
                    },
                    cx,
                );
            },
        )
    }
}
