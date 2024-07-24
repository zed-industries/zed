use gpui::AppContext;
use settings::{EditableSettingControl, Settings};
use theme::ThemeSettings;
use ui::{prelude::*, NumericStepper};

#[derive(IntoElement)]
pub struct UiFontSizeSetting(Pixels);

impl EditableSettingControl for UiFontSizeSetting {
    type Value = Pixels;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "UI Font Size".into()
    }

    fn new(cx: &AppContext) -> Self {
        let settings = ThemeSettings::get_global(cx);

        Self(settings.ui_font_size)
    }

    fn apply(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
        settings.ui_font_size = Some(value.into());
    }
}

impl RenderOnce for UiFontSizeSetting {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let value = self.0;

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontSize))
            .child(NumericStepper::new(
                self.0.to_string(),
                move |_, cx| {
                    Self::write(value - px(1.), cx);
                },
                move |_, cx| {
                    Self::write(value + px(1.), cx);
                },
            ))
    }
}
