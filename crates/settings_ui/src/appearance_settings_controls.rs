use gpui::{AppContext, FontWeight};
use settings::{EditableSettingControl, Settings};
use theme::{SystemAppearance, ThemeRegistry, ThemeSettings};
use ui::{prelude::*, ContextMenu, DropdownMenu, NumericStepper, SettingsContainer, SettingsGroup};

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
            .child(SettingsGroup::new("Theme").child(ThemeControl))
            .child(
                SettingsGroup::new("Font")
                    .child(UiFontSizeControl)
                    .child(UiFontWeightControl),
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
            .unwrap_or("One Dark".to_string())
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
