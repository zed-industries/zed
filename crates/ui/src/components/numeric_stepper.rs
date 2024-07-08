use gpui::ClickEvent;

use crate::{prelude::*, IconButtonShape};

pub struct NumericStepperHandlers {
    pub on_decrement: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
    pub on_increment: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
    pub on_reset: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
}

#[derive(IntoElement)]
pub struct NumericStepper {
    value: SharedString,
    on_decrement: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
    on_increment: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
    on_reset: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
}

impl NumericStepper {
    pub fn new(value: impl Into<SharedString>, handlers: NumericStepperHandlers) -> Self {
        Self {
            value: value.into(),
            on_decrement: handlers.on_decrement,
            on_increment: handlers.on_increment,
            on_reset: handlers.on_reset,
        }
    }
}

impl RenderOnce for NumericStepper {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let shape = IconButtonShape::Square;
        let icon_size = IconSize::Small;

        h_flex()
            .gap_2()
            .child(
                IconButton::new("reset", IconName::RotateCcw)
                    .shape(shape)
                    .icon_size(icon_size)
                    .on_click(self.on_reset),
            )
            .child(
                h_flex()
                    .gap_1()
                    .px_1()
                    .rounded_sm()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        IconButton::new("decrement", IconName::Dash)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(self.on_decrement),
                    )
                    .child(Label::new(self.value))
                    .child(
                        IconButton::new("increment", IconName::Plus)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(self.on_increment),
                    ),
            )
    }
}
