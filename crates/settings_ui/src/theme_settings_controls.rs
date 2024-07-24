use gpui::{AppContext, FontWeight};
use settings::{EditableSettingControl, Settings};
use theme::ThemeSettings;
use ui::{prelude::*, ContextMenu, DropdownMenu, NumericStepper, SettingsContainer, SettingsGroup};

#[derive(IntoElement)]
pub struct ThemeSettingsControls {}

impl ThemeSettingsControls {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for ThemeSettingsControls {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        SettingsContainer::new().child(
            SettingsGroup::new("Font")
                .child(UiFontSizeControl)
                .child(UiFontWeightControl),
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

    fn apply(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
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

    fn apply(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
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
