use std::sync::Arc;

use fs::Fs;
use gpui::{AppContext, CursorStyle};
use settings::{update_settings_file, Settings};
use theme::{ThemeSettings, ThemeSettingsContent};
use ui::{prelude::*, ListHeader, NumericStepper};

// pub enum ScalarType {
//     Float32,
// }

pub enum SettingKind {
    Scalar,
}

pub trait EditableSetting: RenderOnce + Send + Sync {
    type Value: Send;
    type Settings: Settings;

    fn name(&self) -> SharedString;

    fn new(settings: &Self::Settings) -> Self;

    fn update(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value);

    // fn update(fs: Arc<dyn Fs>, cx: &AppContext, update: impl  FnOnce(&mut <Self::Settings as Settings>::FileContent, &AppContext) + Send + 'static) {

    // }

    fn write(value: Self::Value, cx: &AppContext) {
        let fs = <dyn Fs>::global(cx);

        update_settings_file::<Self::Settings>(fs, cx, move |settings, _cx| {
            Self::update(settings, value);
        });
    }
}

#[derive(IntoElement)]
pub struct UiFontSizeSetting(Pixels);

impl EditableSetting for UiFontSizeSetting {
    type Value = Pixels;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "UI Font Size".into()
    }

    fn new(settings: &Self::Settings) -> Self {
        Self(settings.ui_font_size)
    }

    fn update(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
        settings.ui_font_size = Some(value.into());
    }
}

impl RenderOnce for UiFontSizeSetting {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = self.0;

        h_flex()
            .gap_2()
            .w_full()
            .justify_between()
            .cursor(CursorStyle::Arrow)
            .child(Label::new(self.name()))
            .child(NumericStepper::new(
                self.0.to_string(),
                move |_, cx| {
                    Self::write(value - px(1.), cx);
                    // self.save
                },
                move |_, cx| {
                    Self::write(value + px(1.), cx);
                }, // cx.listener(|this, _event, cx| {
                   //     if this.0 > px(0.) {
                   //         this.0 -= px(1.);
                   //     }
                   // }),
                   // cx.listener(|this, _event, cx| {
                   //     this.0 += px(1.);
                   // }),
            ))
    }
}

// impl Render for UiFontSizeSetting {
//     fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
//         h_flex()
//             .gap_2()
//             .w_full()
//             .justify_between()
//             .cursor(CursorStyle::Arrow)
//             .child(Label::new(self.name()))
//             .child(NumericStepper::new(
//                 self.0.to_string(),
//                 cx.listener(|this, _event, cx| {
//                     if this.0 > px(0.) {
//                         this.0 -= px(1.);
//                     }
//                 }),
//                 cx.listener(|this, _event, cx| {
//                     this.0 += px(1.);
//                 }),
//             ))
//     }
// }

#[derive(IntoElement)]
pub struct UiFontSettingsControl {}

impl RenderOnce for UiFontSettingsControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme_settings = ThemeSettings::get_global(cx);

        v_flex()
            .p_1()
            .gap_2()
            .child(ListHeader::new("UI Font"))
            .child(
                h_flex()
                    .gap_2()
                    .w_full()
                    .justify_between()
                    .cursor(CursorStyle::Arrow)
                    .child(Label::new("UI Font Size"))
                    .child(NumericStepper::new(
                        theme_settings.ui_font_size.to_string(),
                        |_, cx| cx.dispatch_action(Box::new(zed_actions::DecreaseUiFontSize)),
                        |_, cx| cx.dispatch_action(Box::new(zed_actions::IncreaseUiFontSize)),
                    )),
            )
    }
}

#[derive(IntoElement)]
pub struct BufferFontSettingsControl {}

impl RenderOnce for BufferFontSettingsControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .p_1()
            .gap_2()
            .child(ListHeader::new("Buffer Font"))
            .child(
                h_flex()
                    .gap_2()
                    .w_full()
                    .justify_between()
                    .cursor(CursorStyle::Arrow)
                    .child(Label::new("Buffer Font Size"))
                    .child(NumericStepper::new(
                        theme::get_buffer_font_size(cx).to_string(),
                        |_, cx| cx.dispatch_action(Box::new(zed_actions::DecreaseBufferFontSize)),
                        |_, cx| cx.dispatch_action(Box::new(zed_actions::IncreaseBufferFontSize)),
                    )),
            )
    }
}
