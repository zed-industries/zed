use gpui::CursorStyle;
use ui::{prelude::*, ListHeader, NumericStepper};

#[derive(IntoElement)]
pub struct UiFontSettingsControl {}

impl RenderOnce for UiFontSettingsControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
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
                        theme::get_ui_font_size(cx).to_string(),
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
